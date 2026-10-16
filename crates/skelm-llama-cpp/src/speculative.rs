//! Speculative decoding.
//!
//! A cheap *drafter* proposes the next few tokens, and the target model checks
//! them all in a single batch: at each drafted position the target's own
//! sampler picks a token, and drafts are kept for as long as they match
//! ("sample and match"). Every emitted token is therefore a sample of the target
//! model, the output distribution is unchanged; when drafts are accepted,
//! several tokens are produced for the cost of one target decode.
//!
//! Two drafters are supported (ported from llama.cpp's `common/speculative.cpp`):
//!   * a **draft model**: a smaller model with a compatible vocabulary,
//!     drafting greedily (`draft-simple` upstream);
//!   * **MTP** (multi-token prediction, a.k.a. nextn): the MTP layers trained
//!     with the target model (Qwen3.5, Qwen3-Next, GLM-4.5, DeepSeek, ...) run
//!     in a second context fed with the target's hidden states, or a separate
//!     MTP "assistant" model such as Gemma 4's (`draft-mtp` upstream).
//!
//! Rejected drafts are removed from the target memory. For recurrent / hybrid
//! models that cannot remove a partial sequence, either llama.cpp's bounded
//! recurrent-state snapshots are used (`n_rs_seq`, for the architectures that
//! support them), or the partial state is checkpointed before verification and
//! restored on rejection.

use skelm_llama_cpp_sys::{ext, llama};
use thiserror::Error;

use crate::batch_ext::BatchExt;
use crate::context::{ContextCreateError, DecodeError};
use crate::{Context, ContextParams, Model, Sampler, Token, Vocab, batch::Batch};

/// Number of tokens decoded per batch when appending a prompt
const PROMPT_CHUNK: usize = 512;

/// The drafter samples greedily from its top-k candidates (as upstream)
const DRAFT_TOP_K: usize = 10;

/// Drafting parameters
#[derive(Clone, Copy, Debug)]
pub struct DraftParams {
    /// maximum number of tokens drafted per round
    pub n_max: usize,
    /// minimum number of drafted tokens for a draft to be verified (shorter
    /// drafts are dropped)
    pub n_min: usize,
    /// stop drafting when the drafter's confidence in its best candidate drops
    /// below this probability
    pub p_min: f32,
}

impl Default for DraftParams {
    /// The llama.cpp defaults
    fn default() -> Self {
        DraftParams {
            n_max: 3,
            n_min: 0,
            p_min: 0.0,
        }
    }
}

/// Where the drafts come from
#[derive(Clone)]
pub enum Drafter {
    /// A separate draft model with a vocabulary compatible with the target's,
    /// and the parameters of its context (its context size follows the
    /// target's). A model that carries MTP layers (e.g. a Gemma 4 assistant) is
    /// driven as an MTP drafter; load it with `ModelParams::load_mtp`.
    Model { model: Model, params: ContextParams },
    /// The MTP layers of the target model itself; the target must be loaded
    /// with `ModelParams::load_mtp`
    Mtp,
}

#[derive(Clone)]
pub struct SpeculativeConfig {
    pub drafter: Drafter,
    pub draft: DraftParams,
}

impl SpeculativeConfig {
    /// Check, without creating any context, that this configuration can drive
    /// `target`: the MTP layers are available, or the draft vocabulary is
    /// compatible
    pub fn check(&self, target: &Model) -> Result<(), SpeculativeError> {
        match &self.drafter {
            Drafter::Mtp if !target.has_mtp() => Err(SpeculativeError::NoMtp),
            Drafter::Mtp => Ok(()),
            Drafter::Model { model, .. } => {
                check_vocab(&target.vocab(), &model.vocab())?;
                if model.n_layer_nextn() > 0 && !model.has_mtp() {
                    // an MTP assistant whose MTP layers were skipped
                    return Err(SpeculativeError::NoMtp);
                }
                Ok(())
            }
        }
    }
}

/// Counters of a speculative context
#[derive(Clone, Copy, Debug, Default)]
pub struct SpeculativeStats {
    /// verification rounds
    pub rounds: u64,
    /// tokens drafted (and verified)
    pub drafted: u64,
    /// drafted tokens accepted by the target
    pub accepted: u64,
}

impl SpeculativeStats {
    /// Fraction of drafted tokens that were accepted
    pub fn acceptance_rate(&self) -> f64 {
        if self.drafted == 0 {
            0.0
        } else {
            self.accepted as f64 / self.drafted as f64
        }
    }
}

#[derive(Debug, Error)]
pub enum SpeculativeError {
    #[error(
        "the model has no MTP layers, or they were not loaded (load the model with `load_mtp`)"
    )]
    NoMtp,
    #[error("the draft model vocabulary is not compatible with the target model: {0}")]
    VocabMismatch(String),
    #[error("the MTP drafter needs hidden states of width {0}, the target produces {1}")]
    WidthMismatch(usize, usize),
    #[error("failed to create the speculative context: {0}")]
    ContextCreate(#[from] ContextCreateError),
    #[error("the draft context cannot remove rejected drafts (partial sequence removal)")]
    DraftRollback,
    #[error("decoding failed: {0}")]
    Decode(#[from] DecodeError),
}

/// The tokens produced by [`Context::step`]
#[derive(Clone, Debug, Default)]
pub struct Step {
    /// generated tokens, in order (never the end-of-generation token)
    pub tokens: Vec<Token>,
    /// end-of-generation was reached
    pub eog: bool,
}

/// How rejected drafts are removed from a context's memory
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Rollback {
    /// the memory supports partial sequence removal
    Partial,
    /// recurrent-state snapshots allow removing up to this many tokens
    Bounded(usize),
    /// the partial state must be checkpointed before verification
    Checkpoint,
}

/// Find how `ctx` can roll back, by decoding two tokens and removing the last
/// one (like upstream `common_context_can_seq_rm`). `state_width` is the
/// hidden-state width for an MTP context, which takes a state row per token.
fn probe_rollback(ctx: &Context, state_width: Option<usize>) -> Result<Rollback, DecodeError> {
    let n_rs = ctx.n_rs_seq();
    if n_rs > 0 {
        return Ok(Rollback::Bounded(n_rs));
    }
    match state_width {
        None => {
            let mut batch = Batch::new(2, 0, 1);
            batch.append(Token(0), 0, &[0], false);
            batch.append(Token(0), 1, &[0], true);
            ctx.decode(&batch)?;
        }
        Some(width) => {
            let zeros = vec![0.0; width];
            let mut batch = BatchExt::new(ctx);
            for pos in 0..2 {
                let idx = batch.add(Token(0), pos, pos == 1);
                batch.set_state(idx, &zeros);
            }
            batch.decode(ctx)?;
        }
    }
    let partial = ctx.memory_seq_rm(1, None);
    ctx.memory_clear(true);
    Ok(if partial {
        Rollback::Partial
    } else {
        Rollback::Checkpoint
    })
}

/// The drafter's best next token from `logits`, with its probability among
/// the top-k candidates
fn draft_pick(logits: &[f32]) -> (Token, f32) {
    let mut top: Vec<(usize, f32)> = Vec::with_capacity(DRAFT_TOP_K + 1);
    for (i, &l) in logits.iter().enumerate() {
        if top.len() < DRAFT_TOP_K || l > top[top.len() - 1].1 {
            let at = top.partition_point(|&(_, x)| x >= l);
            top.insert(at, (i, l));
            top.truncate(DRAFT_TOP_K);
        }
    }
    let max = top[0].1;
    let sum: f32 = top.iter().map(|&(_, l)| (l - max).exp()).sum();
    (Token(top[0].0 as i32), 1.0 / sum)
}

/// Check that drafts of `draft` can be read as tokens of `target` (upstream
/// `common_speculative_are_compatible`)
fn check_vocab(target: &Vocab, draft: &Vocab) -> Result<(), SpeculativeError> {
    const MAX_SIZE_DIFFERENCE: u32 = 128;
    const CHECK_START: u32 = 5;

    if target.vocab_type() != draft.vocab_type() {
        return Err(SpeculativeError::VocabMismatch(
            "the vocabulary types differ".into(),
        ));
    }
    if target.bos() != draft.bos() || target.eos() != draft.eos() {
        return Err(SpeculativeError::VocabMismatch(
            "the special tokens differ".into(),
        ));
    }
    let (n_tgt, n_dft) = (target.n_tokens(), draft.n_tokens());
    if n_tgt.abs_diff(n_dft) > MAX_SIZE_DIFFERENCE {
        return Err(SpeculativeError::VocabMismatch(format!(
            "the vocabulary sizes differ too much ({n_tgt} vs {n_dft})"
        )));
    }
    for i in CHECK_START..n_tgt.min(n_dft) {
        if target.text(Token(i as i32)) != draft.text(Token(i as i32)) {
            return Err(SpeculativeError::VocabMismatch(format!(
                "token {i} differs"
            )));
        }
    }
    Ok(())
}

/// A separate draft model, drafting greedily
struct ModelDrafter {
    ctx: Context,
}

impl ModelDrafter {
    /// Mirror a batch decoded on the target
    fn process(&mut self, tokens: &[(Token, usize)]) -> Result<(), DecodeError> {
        let mut batch = Batch::new(tokens.len(), 0, 1);
        for (k, &(token, pos)) in tokens.iter().enumerate() {
            batch.append(token, pos, &[0], k == tokens.len() - 1);
        }
        self.ctx.decode(&batch)
    }

    fn draft(
        &mut self,
        id_last: Token,
        pos0: usize,
        params: &DraftParams,
        n_max: usize,
    ) -> Result<Vec<Token>, DecodeError> {
        let mut result = Vec::new();
        let mut batch = Batch::new(1, 0, 1);
        batch.append(id_last, pos0, &[0], true);
        self.ctx.decode(&batch)?;
        loop {
            let (token, p) = draft_pick(self.ctx.get_logits(-1));
            if p < params.p_min {
                break;
            }
            result.push(token);
            if result.len() >= n_max {
                break;
            }
            batch.clear();
            batch.append(token, pos0 + result.len(), &[0], true);
            self.ctx.decode(&batch)?;
        }
        Ok(result)
    }
}

/// An MTP drafter: a context of type MTP running the nextn layers, fed with
/// the hidden states of the target
struct MtpDrafter {
    ctx: Context,
    batch: BatchExt,
    n_embd: usize,
    /// number of MTP heads
    n_heads: usize,
    /// the drafter reads the target memory (Gemma 4 assistants), no catch-up
    mem_shared: bool,
    /// several trained heads, one per draft step (Step 3.5)
    chain_heads: bool,
    /// the hidden state pairing with the next token to process / draft from
    pending_h: Vec<f32>,
    /// the hidden-state rows of the last target batch
    verify_h: Vec<f32>,
    verify_rows: usize,
    /// hidden-state rows of the current chained draft
    chain_h: Vec<f32>,
}

impl MtpDrafter {
    fn new(
        target: &Context,
        model: Model,
        c_params: llama::llama_context_params,
    ) -> Result<Self, SpeculativeError> {
        if !model.has_mtp() {
            return Err(SpeculativeError::NoMtp);
        }
        let n_embd = model.n_embd_out();
        let n_embd_target = target.model.n_embd_out();
        if n_embd != n_embd_target {
            return Err(SpeculativeError::WidthMismatch(n_embd, n_embd_target));
        }
        let n_heads = model.n_layer_nextn().max(1);

        let mut c_params = c_params;
        c_params.ctx_type = llama::llama_context_type::LLAMA_CONTEXT_TYPE_MTP;
        c_params.ctx_other = target.ptr.0;
        c_params.n_ctx = target.n_ctx();
        c_params.n_rs_seq = 0;
        c_params.n_outputs_max = 0;
        c_params.n_outputs_max_per_seq = 1;
        c_params.embeddings = false;
        let ctx = Context::new_from_c(model, c_params)?;

        unsafe {
            ext::skelm_llama_set_embeddings_nextn(target.ptr.0, true, false);
            ext::skelm_llama_set_embeddings_nextn(ctx.ptr.0, true, true);
        }
        let mem_shared = unsafe { ext::skelm_llama_get_ctx_other(ctx.ptr.0) } == target.ptr.0;
        let batch = BatchExt::new(&ctx);

        Ok(MtpDrafter {
            ctx,
            batch,
            n_embd,
            n_heads,
            mem_shared,
            chain_heads: n_heads > 1 && !mem_shared,
            pending_h: vec![0.0; n_embd],
            verify_h: Vec::new(),
            verify_rows: 0,
            chain_h: Vec::new(),
        })
    }

    fn set_head(&self, head: usize) {
        unsafe { ext::skelm_llama_set_nextn_layer_offset(self.ctx.ptr.0, head as i32) }
    }

    /// The hidden-state row of the `i`th token of the last batch of `ctx`
    fn row(ctx: &Context, i: i32, n_embd: usize) -> &[f32] {
        unsafe {
            let p = ext::skelm_llama_get_embeddings_nextn_ith(ctx.ptr.0, i);
            assert!(!p.is_null(), "no nextn embeddings for row {i}");
            core::slice::from_raw_parts(p, n_embd)
        }
    }

    /// Catch up with a batch decoded on the target: pair each token with the
    /// target hidden state of the previous position
    fn process(&mut self, target: &Context, tokens: &[(Token, usize)]) -> Result<(), DecodeError> {
        let n_embd = self.n_embd;
        if !self.mem_shared {
            let h_tgt = unsafe { ext::skelm_llama_get_embeddings_nextn(target.ptr.0) };
            assert!(!h_tgt.is_null(), "the target has no nextn embeddings");
            self.batch.clear();
            for (k, &(token, pos)) in tokens.iter().enumerate() {
                let idx = self.batch.add(token, pos, false);
                let row = if k == 0 {
                    &self.pending_h[..]
                } else {
                    unsafe { core::slice::from_raw_parts(h_tgt.add((k - 1) * n_embd), n_embd) }
                };
                self.batch.set_state(idx, row);
            }
            let heads = if self.chain_heads { self.n_heads } else { 1 };
            for head in 0..heads {
                if self.chain_heads {
                    self.ctx.memory_seq_rm(tokens[0].1, None);
                    self.set_head(head);
                }
                let res = self.batch.decode(&self.ctx);
                if res.is_err() {
                    self.set_head(0);
                }
                res?;
            }
            if self.chain_heads {
                self.set_head(0);
            }
        }

        self.verify_rows = tokens.len();
        self.verify_h.clear();
        for k in 0..tokens.len() {
            let row = Self::row(target, k as i32, n_embd);
            self.verify_h.extend_from_slice(row);
        }
        let last = (tokens.len() - 1) * n_embd;
        self.pending_h
            .copy_from_slice(&self.verify_h[last..last + n_embd]);
        Ok(())
    }

    fn draft(
        &mut self,
        id_last: Token,
        pos0: usize,
        params: &DraftParams,
        n_max: usize,
    ) -> Result<Vec<Token>, DecodeError> {
        let n_max = if self.chain_heads {
            n_max.min(self.n_heads)
        } else {
            n_max
        };
        let n_embd = self.n_embd;
        let mut result = Vec::new();

        self.batch.clear();
        let mut i_last = self.batch.add(id_last, pos0, true);
        self.batch.set_state(i_last, &self.pending_h);
        if self.chain_heads {
            self.chain_h.clear();
            self.chain_h.extend_from_slice(&self.pending_h);
        }

        let mut i = 0;
        let res = loop {
            if self.chain_heads {
                self.ctx.memory_seq_rm(pos0, None);
                self.set_head(i);
            }
            if let Err(e) = self.batch.decode(&self.ctx) {
                break Err(e);
            }

            let (token, p) = draft_pick(self.ctx.get_logits(i_last));
            if p < params.p_min {
                break Ok(());
            }
            result.push(token);
            if result.len() >= n_max {
                break Ok(());
            }
            let h_row = Self::row(&self.ctx, i_last, n_embd).to_vec();

            self.batch.clear();
            if self.chain_heads {
                // each head reads the whole chain, rebuilt at the next head
                self.chain_h.extend_from_slice(&h_row);
                let n_rows = result.len() + 1;
                for t in 0..n_rows {
                    let tok = if t == 0 { id_last } else { result[t - 1] };
                    i_last = self.batch.add(tok, pos0 + t, t == n_rows - 1);
                    let row = self.chain_h[t * n_embd..(t + 1) * n_embd].to_vec();
                    self.batch.set_state(i_last, &row);
                }
            } else if self.mem_shared {
                // with shared memory, all the drafts use the same position
                i_last = self.batch.add(token, pos0, true);
                self.batch.set_state(i_last, &h_row);
            } else {
                i_last = self.batch.add(token, pos0 + i + 1, true);
                self.batch.set_state(i_last, &h_row);
            }
            i += 1;
        };
        if self.chain_heads {
            self.set_head(0);
        }
        res.map(|()| result)
    }

    /// `n_accepted` drafts were accepted: continue from the hidden state of the
    /// last accepted token
    fn accept(&mut self, n_accepted: usize) {
        if self.verify_rows == 0 {
            return;
        }
        let i = n_accepted.min(self.verify_rows - 1) * self.n_embd;
        self.pending_h
            .copy_from_slice(&self.verify_h[i..i + self.n_embd]);
    }
}

enum DraftState {
    Model(ModelDrafter),
    Mtp(MtpDrafter),
}

impl DraftState {
    fn ctx(&self) -> &Context {
        match self {
            DraftState::Model(d) => &d.ctx,
            DraftState::Mtp(d) => &d.ctx,
        }
    }

    fn process(&mut self, target: &Context, tokens: &[(Token, usize)]) -> Result<(), DecodeError> {
        match self {
            DraftState::Model(d) => d.process(tokens),
            DraftState::Mtp(d) => d.process(target, tokens),
        }
    }

    fn draft(
        &mut self,
        id_last: Token,
        pos0: usize,
        params: &DraftParams,
        n_max: usize,
    ) -> Result<Vec<Token>, DecodeError> {
        let mut result = match self {
            DraftState::Model(d) => d.draft(id_last, pos0, params, n_max)?,
            DraftState::Mtp(d) => d.draft(id_last, pos0, params, n_max)?,
        };
        if result.len() < params.n_min {
            result.clear();
        }
        Ok(result)
    }

    fn accept(&mut self, n_accepted: usize) {
        if let DraftState::Mtp(d) = self {
            d.accept(n_accepted)
        }
    }

    /// Remove everything from position `pos` on from the draft memory
    fn rollback_to(&self, pos: usize) {
        let ok = self.ctx().memory_seq_rm(pos, None);
        assert!(ok, "the draft context failed to remove a partial sequence");
    }
}

pub(crate) struct Speculative {
    drafter: DraftState,
    params: DraftParams,
    rollback: Rollback,
    /// the last sampled token, not yet decoded on the target
    pending: Option<Token>,
    stats: SpeculativeStats,
}

impl Context {
    /// Create a context with speculative decoding (see the module
    /// documentation). Generate with [`Context::step`].
    pub fn new_speculative(
        model: Model,
        params: &ContextParams,
        config: &SpeculativeConfig,
    ) -> Result<Self, SpeculativeError> {
        Self::new_speculative_impl(model, params, config, true)
    }

    /// `snapshots: false` disables the recurrent-state snapshots of the target,
    /// forcing hybrid models onto checkpoints (for testing that path)
    pub(crate) fn new_speculative_impl(
        model: Model,
        params: &ContextParams,
        config: &SpeculativeConfig,
        snapshots: bool,
    ) -> Result<Self, SpeculativeError> {
        let n_max = config.draft.n_max.max(1);

        let mut c_params = params.as_c();
        // the target outputs logits for every token of a verification batch,
        // and keeps enough recurrent-state snapshots to reject a whole draft
        c_params.n_outputs_max_per_seq = c_params.n_outputs_max_per_seq.max(1 + n_max as u32);
        c_params.n_rs_seq = if snapshots { n_max as u32 } else { 0 };
        let mut target = Context::new_from_c(model.clone(), c_params)?;

        let drafter = match &config.drafter {
            Drafter::Mtp => DraftState::Mtp(MtpDrafter::new(&target, model, params.as_c())?),
            Drafter::Model {
                model: draft_model,
                params: draft_params,
            } => {
                check_vocab(&target.model.vocab(), &draft_model.vocab())?;
                let mut c_params = draft_params.as_c();
                c_params.n_ctx = target.n_ctx();
                if draft_model.has_mtp() {
                    DraftState::Mtp(MtpDrafter::new(&target, draft_model.clone(), c_params)?)
                } else {
                    c_params.n_rs_seq = n_max as u32;
                    let ctx = Context::new_from_c(draft_model.clone(), c_params)?;
                    DraftState::Model(ModelDrafter { ctx })
                }
            }
        };

        let rollback = probe_rollback(&target, None)?;
        let draft_width = match &drafter {
            DraftState::Mtp(d) if !d.mem_shared => Some(d.n_embd),
            DraftState::Mtp(_) => None,
            DraftState::Model(_) => None,
        };
        let draft_rollback = match &drafter {
            // an assistant sharing the target memory has nothing of its own to probe
            DraftState::Mtp(d) if d.mem_shared => Rollback::Partial,
            d => probe_rollback(d.ctx(), draft_width)?,
        };
        let draft_n_max = match draft_rollback {
            Rollback::Partial => n_max,
            Rollback::Bounded(n) => n_max.min(n),
            Rollback::Checkpoint => return Err(SpeculativeError::DraftRollback),
        };
        let n_max = match rollback {
            Rollback::Bounded(n) => draft_n_max.min(n),
            _ => draft_n_max,
        };

        target.speculative = Some(Box::new(Speculative {
            drafter,
            params: DraftParams {
                n_max,
                ..config.draft
            },
            rollback,
            pending: None,
            stats: SpeculativeStats::default(),
        }));
        Ok(target)
    }

    /// The counters of a speculative context, `None` for a plain one
    pub fn speculative_stats(&self) -> Option<SpeculativeStats> {
        self.speculative.as_ref().map(|s| s.stats)
    }

    /// Run `f` with the speculative state taken out of `self`
    fn with_speculative<R>(&mut self, f: impl FnOnce(&mut Context, &mut Speculative) -> R) -> R {
        let mut spec = self
            .speculative
            .take()
            .expect("speculative context expected");
        let res = f(self, &mut spec);
        self.speculative = Some(spec);
        res
    }

    /// Decode tokens at the end of the target and mirror them on the drafter
    fn speculative_decode(
        &mut self,
        spec: &mut Speculative,
        tokens: &[Token],
        logits_last: bool,
    ) -> Result<(), DecodeError> {
        for chunk in tokens.chunks(PROMPT_CHUNK) {
            let at = self.tokens;
            let positioned: Vec<(Token, usize)> = chunk
                .iter()
                .enumerate()
                .map(|(i, &t)| (t, at + i))
                .collect();
            let mut batch = Batch::new(chunk.len(), 0, 1);
            for (i, &(token, pos)) in positioned.iter().enumerate() {
                batch.append(token, pos, &[0], logits_last && i == chunk.len() - 1);
            }
            self.decode(&batch)?;
            self.tokens += chunk.len();
            spec.drafter.process(self, &positioned)?;
        }
        Ok(())
    }

    pub(crate) fn speculative_append(&mut self, tokens: &[Token]) -> Result<(), DecodeError> {
        self.with_speculative(|ctx, spec| {
            let mut all = Vec::with_capacity(tokens.len() + 1);
            all.extend(spec.pending.take());
            all.extend_from_slice(tokens);
            ctx.speculative_decode(spec, &all, true)
        })
    }

    /// Decode the pending token, if any
    pub(crate) fn speculative_flush(&mut self) -> Result<(), DecodeError> {
        self.speculative_append(&[])
    }

    pub(crate) fn speculative_step<S: Sampler>(
        &mut self,
        sampler: &mut S,
        vocab: &Vocab,
    ) -> Result<Step, DecodeError> {
        self.with_speculative(|ctx, spec| ctx.speculative_round(spec, sampler, vocab))
    }

    fn speculative_round<S: Sampler>(
        &mut self,
        spec: &mut Speculative,
        sampler: &mut S,
        vocab: &Vocab,
    ) -> Result<Step, DecodeError> {
        let Some(id_last) = spec.pending.take() else {
            // right after a prompt: sample from its logits, the drafts start
            // from this token at the next step
            let token = sampler.sample(self, -1);
            if vocab.is_eog(token) {
                return Ok(Step {
                    tokens: Vec::new(),
                    eog: true,
                });
            }
            spec.pending = Some(token);
            return Ok(Step {
                tokens: vec![token],
                eog: false,
            });
        };

        let n_past = self.tokens;
        // keep room in the context for the verification batch
        let n_max = (self.n_ctx() as usize)
            .saturating_sub(n_past + 2)
            .min(spec.params.n_max);
        let mut draft = if n_max > 0 {
            spec.drafter.draft(id_last, n_past, &spec.params, n_max)?
        } else {
            Vec::new()
        };
        // a draft vocabulary can be slightly larger than the target's
        let n_vocab = vocab.n_tokens() as usize;
        if let Some(cut) = draft.iter().position(|t| t.as_index() >= n_vocab) {
            draft.truncate(cut);
        }
        // the verification below re-processes the drafts on the drafter
        spec.drafter.rollback_to(n_past);

        let checkpoint = (spec.rollback == Rollback::Checkpoint && !draft.is_empty())
            .then(|| self.state_seq_get_partial());

        // verify [id_last, draft...] on the target, with logits for every position
        let positioned: Vec<(Token, usize)> = std::iter::once(id_last)
            .chain(draft.iter().copied())
            .enumerate()
            .map(|(i, t)| (t, n_past + i))
            .collect();
        let mut batch = Batch::new(positioned.len(), 0, 1);
        for &(token, pos) in &positioned {
            batch.append(token, pos, &[0], true);
        }
        self.decode(&batch)?;
        self.tokens += positioned.len();
        spec.drafter.process(self, &positioned)?;

        // sample and match: keep drafts while the target samples the same token
        let mut ids = Vec::with_capacity(draft.len() + 1);
        for i in 0..=draft.len() {
            let token = sampler.sample(self, i as i32);
            ids.push(token);
            if vocab.is_eog(token) || i == draft.len() || token != draft[i] {
                break;
            }
        }
        let n_accepted = ids.len() - 1;

        spec.stats.rounds += 1;
        spec.stats.drafted += draft.len() as u64;
        spec.stats.accepted += n_accepted as u64;

        // keep id_last and the accepted drafts, drop the rest
        let keep = n_past + 1 + n_accepted;
        if keep < self.tokens {
            match (spec.rollback, checkpoint) {
                (Rollback::Checkpoint, Some(state)) => {
                    // restore the state before verification, and replay what
                    // was accepted
                    assert!(
                        self.state_seq_set_partial(&state),
                        "failed to restore the speculative checkpoint"
                    );
                    self.memory_seq_rm(n_past, None);
                    let mut replay = Batch::new(1 + n_accepted, 0, 1);
                    for &(token, pos) in &positioned[..1 + n_accepted] {
                        replay.append(token, pos, &[0], pos + 1 == keep);
                    }
                    self.decode(&replay)?;
                }
                _ => {
                    let ok = self.memory_seq_rm(keep, None);
                    assert!(ok, "the target failed to remove the rejected drafts");
                }
            }
            self.tokens = keep;
        }
        spec.drafter.accept(n_accepted);
        spec.drafter.rollback_to(keep);

        let last = *ids.last().expect("at least one sampled token");
        if vocab.is_eog(last) {
            ids.pop();
            return Ok(Step {
                tokens: ids,
                eog: true,
            });
        }
        spec.pending = Some(last);
        Ok(Step {
            tokens: ids,
            eog: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ModelParams, SamplerGreedy};

    /// Greedily generate `n` tokens after `prompt`
    fn generate(ctx: &mut Context, prompt: &str, n: usize) -> Vec<Token> {
        let vocab = ctx.model().vocab();
        ctx.append_tokens(&vocab.tokenize(prompt.as_bytes(), true))
            .unwrap();
        let mut out = Vec::new();
        while out.len() < n {
            let step = ctx.step(&mut SamplerGreedy, &vocab).unwrap();
            out.extend(step.tokens);
            if step.eog {
                break;
            }
        }
        out.truncate(n);
        out
    }

    /// The checkpoint rollback, on a hybrid MTP model (e.g. Qwen3.5) with its
    /// recurrent-state snapshots disabled
    #[test]
    #[ignore = "needs SKELM_TEST_MTP"]
    fn checkpoint_rollback_matches_plain() {
        let path = std::env::var("SKELM_TEST_MTP").expect("SKELM_TEST_MTP");
        let params = ModelParams {
            load_mtp: true,
            ..ModelParams::default()
        };
        let model = Model::load(path, &params).unwrap();
        let ctx_params = ContextParams {
            n_ctx: 2048,
            ..ContextParams::default()
        };
        let prompt = "The three laws of robotics are:\n1.";

        let mut plain = model.new_context(&ctx_params).unwrap();
        let expected = generate(&mut plain, prompt, 96);
        drop(plain);

        let config = SpeculativeConfig {
            drafter: Drafter::Mtp,
            draft: DraftParams::default(),
        };
        let mut spec =
            Context::new_speculative_impl(model.clone(), &ctx_params, &config, false).unwrap();
        assert_eq!(
            spec.speculative.as_ref().unwrap().rollback,
            Rollback::Checkpoint,
            "the model should need checkpoints without snapshots"
        );
        let got = generate(&mut spec, prompt, 96);
        let stats = spec.speculative_stats().unwrap();
        println!("{stats:?}");
        assert!(
            stats.accepted < stats.drafted,
            "no rejection exercised the rollback"
        );
        assert_eq!(got, expected);
    }

    #[test]
    fn draft_pick_takes_the_best_of_top_k() {
        let mut logits = vec![0.0f32; 100];
        logits[42] = 10.0;
        logits[7] = 9.0;
        let (token, p) = draft_pick(&logits);
        assert_eq!(token, Token(42));
        // top-10: 10.0, 9.0 and eight 0.0 logits
        let expected = 1.0 / (1.0 + (-1.0f32).exp() + 8.0 * (-10.0f32).exp());
        assert!((p - expected).abs() < 1e-6, "{p} vs {expected}");
    }

    #[test]
    fn draft_pick_is_certain_on_a_single_candidate() {
        let mut logits = vec![f32::NEG_INFINITY; 50];
        logits[3] = 1.0;
        let (token, p) = draft_pick(&logits);
        assert_eq!(token, Token(3));
        assert_eq!(p, 1.0);
    }
}
