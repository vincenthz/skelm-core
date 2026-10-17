use skelm_llama_cpp_sys::llama;
use thiserror::Error;

use crate::speculative::{Speculative, Step};
use crate::{LoraAdapter, Model, Sampler, Vocab, batch::Batch, token::Token};

// Field order matters: fields are dropped in declaration order, and the draft
// context of `speculative` (an MTP context references this one), then this
// context, must be freed before the adapters and the model they use.
#[allow(dead_code)]
pub struct Context {
    pub(crate) speculative: Option<Box<Speculative>>,
    pub(crate) ptr: ContextPtr,
    pub(crate) lora_adapters: Vec<LoraAdapter>,
    pub(crate) model: Model,
    pub(crate) tokens: usize,
    pub(crate) context_params: llama::llama_context_params,
}

unsafe impl Send for Context {}

pub struct ContextPtr(pub(crate) *mut llama::llama_context);

impl Drop for ContextPtr {
    fn drop(&mut self) {
        unsafe {
            llama::llama_free(self.0);
        }
    }
}

/// When to enable Flash Attention
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlashAttn {
    /// let llama.cpp decide based on the model and devices
    Auto,
    Disabled,
    Enabled,
}

impl FlashAttn {
    fn to_c(self) -> llama::llama_flash_attn_type {
        match self {
            FlashAttn::Auto => llama::llama_flash_attn_type::LLAMA_FLASH_ATTN_TYPE_AUTO,
            FlashAttn::Disabled => llama::llama_flash_attn_type::LLAMA_FLASH_ATTN_TYPE_DISABLED,
            FlashAttn::Enabled => llama::llama_flash_attn_type::LLAMA_FLASH_ATTN_TYPE_ENABLED,
        }
    }

    fn from_c(value: llama::llama_flash_attn_type) -> Self {
        match value {
            llama::llama_flash_attn_type::LLAMA_FLASH_ATTN_TYPE_DISABLED => FlashAttn::Disabled,
            llama::llama_flash_attn_type::LLAMA_FLASH_ATTN_TYPE_ENABLED => FlashAttn::Enabled,
            _ => FlashAttn::Auto,
        }
    }
}

/// Data type of the K / V cache (the types llama.cpp's own tools accept)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CacheType {
    F32,
    F16,
    BF16,
    Q8_0,
    Q4_0,
    Q4_1,
    #[allow(non_camel_case_types)]
    IQ4_NL,
    Q5_0,
    Q5_1,
}

impl CacheType {
    fn to_c(self) -> llama::ggml_type {
        match self {
            CacheType::F32 => llama::ggml_type::GGML_TYPE_F32,
            CacheType::F16 => llama::ggml_type::GGML_TYPE_F16,
            CacheType::BF16 => llama::ggml_type::GGML_TYPE_BF16,
            CacheType::Q8_0 => llama::ggml_type::GGML_TYPE_Q8_0,
            CacheType::Q4_0 => llama::ggml_type::GGML_TYPE_Q4_0,
            CacheType::Q4_1 => llama::ggml_type::GGML_TYPE_Q4_1,
            CacheType::IQ4_NL => llama::ggml_type::GGML_TYPE_IQ4_NL,
            CacheType::Q5_0 => llama::ggml_type::GGML_TYPE_Q5_0,
            CacheType::Q5_1 => llama::ggml_type::GGML_TYPE_Q5_1,
        }
    }

    fn from_c(value: llama::ggml_type) -> Self {
        match value {
            llama::ggml_type::GGML_TYPE_F32 => CacheType::F32,
            llama::ggml_type::GGML_TYPE_BF16 => CacheType::BF16,
            llama::ggml_type::GGML_TYPE_Q8_0 => CacheType::Q8_0,
            llama::ggml_type::GGML_TYPE_Q4_0 => CacheType::Q4_0,
            llama::ggml_type::GGML_TYPE_Q4_1 => CacheType::Q4_1,
            llama::ggml_type::GGML_TYPE_IQ4_NL => CacheType::IQ4_NL,
            llama::ggml_type::GGML_TYPE_Q5_0 => CacheType::Q5_0,
            llama::ggml_type::GGML_TYPE_Q5_1 => CacheType::Q5_1,
            _ => CacheType::F16,
        }
    }
}

/// Context creation parameters; `Default` uses the llama.cpp defaults, except
/// for a 16384 tokens context
#[derive(Clone, Debug)]
pub struct ContextParams {
    /// text context size, 0 = from model
    pub n_ctx: u32,
    /// logical maximum batch size that can be submitted to decode
    pub n_batch: u32,
    /// physical maximum batch size
    pub n_ubatch: u32,
    /// max number of sequences (i.e. distinct states for recurrent models)
    pub n_seq_max: u32,
    /// number of threads to use for generation
    pub n_threads: i32,
    /// number of threads to use for batch (prompt) processing
    pub n_threads_batch: i32,
    /// extract embeddings (together with logits)
    pub embeddings: bool,
    /// when to enable Flash Attention
    pub flash_attn: FlashAttn,
    /// data type for the K cache
    pub type_k: CacheType,
    /// data type for the V cache
    pub type_v: CacheType,
    /// offload the KQV ops (including the KV cache) to GPU
    pub offload_kqv: bool,
    /// offload host tensor operations to device
    pub op_offload: bool,
    /// device cache in bytes for the MoE experts kept in host memory, 0 = disabled
    pub moe_cache_size: usize,
    /// with several sequences, share one KV buffer between them (each
    /// sequence can then use the whole `n_ctx`), instead of splitting it into
    /// `n_seq_max` equal parts
    pub kv_unified: bool,
}

impl Default for ContextParams {
    fn default() -> Self {
        let context = unsafe { llama::llama_context_default_params() };
        Self {
            n_ctx: 16384,
            n_batch: context.n_batch,
            n_ubatch: context.n_ubatch,
            n_seq_max: context.n_seq_max,
            n_threads: context.n_threads,
            n_threads_batch: context.n_threads_batch,
            embeddings: context.embeddings,
            flash_attn: FlashAttn::from_c(context.flash_attn_type),
            type_k: CacheType::from_c(context.type_k),
            type_v: CacheType::from_c(context.type_v),
            offload_kqv: context.offload_kqv,
            op_offload: context.op_offload,
            moe_cache_size: context.moe_cache_size,
            kv_unified: context.kv_unified,
        }
    }
}

impl ContextParams {
    pub(crate) fn as_c(&self) -> llama::llama_context_params {
        let mut context = unsafe { llama::llama_context_default_params() };
        context.n_ctx = self.n_ctx;
        context.n_batch = self.n_batch;
        context.n_ubatch = self.n_ubatch;
        context.n_seq_max = self.n_seq_max;
        context.n_threads = self.n_threads;
        context.n_threads_batch = self.n_threads_batch;
        context.embeddings = self.embeddings;
        context.flash_attn_type = self.flash_attn.to_c();
        context.type_k = self.type_k.to_c();
        context.type_v = self.type_v.to_c();
        context.offload_kqv = self.offload_kqv;
        context.op_offload = self.op_offload;
        context.moe_cache_size = self.moe_cache_size;
        context.kv_unified = self.kv_unified;
        context
    }
}

#[derive(Clone, Copy, Debug, Error)]
pub enum DecodeError {
    #[error("cannot find KV Slot")]
    CannotFindKVSlot,
    #[error("Aborted")]
    Aborted,
    #[error("Invalid Batch")]
    InvalidBatch,
    #[error("Unspecified Decode Warning {0}")]
    UnspecifiedWarning(#[allow(dead_code)] i32),
    #[error("Fatal Decode Error {0}")]
    FatalError(#[allow(dead_code)] i32),
}

impl DecodeError {
    /// Map a `llama_decode` / `llama_process` return value
    pub(crate) fn check(ret: i32) -> Result<(), DecodeError> {
        match ret {
            0 => Ok(()),
            1 => Err(DecodeError::CannotFindKVSlot),
            2 => Err(DecodeError::Aborted),
            -1 => Err(DecodeError::InvalidBatch),
            _ if ret > 0 => Err(DecodeError::UnspecifiedWarning(ret)),
            _ => Err(DecodeError::FatalError(ret)),
        }
    }
}

#[derive(Debug)]
pub struct ContextCreateError;

impl std::fmt::Display for ContextCreateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Context Create Error (consult logging for reasons)")
    }
}

impl std::error::Error for ContextCreateError {}

#[derive(Debug, Clone)]
pub struct EmbeddingSeqError(i32);

impl std::fmt::Display for EmbeddingSeqError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Embedding Sequence Error {}", self.0)
    }
}

impl std::error::Error for EmbeddingSeqError {}

#[derive(Debug, Error)]
pub enum ContextEmbeddingError {
    #[error("embedding decoding failed: {0}")]
    DecodeError(#[from] DecodeError),
    #[error("embedding sequence getting failed: {0}")]
    EmbeddingSeqError(#[from] EmbeddingSeqError),
    #[error("pooling type not yet supported: {0:?}")]
    UnsupportedPoolingType(PoolingType),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PoolingType {
    Unspecified,
    None,
    Mean,
    Cls,
    Last,
    Rank,
}

impl From<llama::llama_pooling_type> for PoolingType {
    fn from(value: llama::llama_pooling_type) -> Self {
        match value {
            llama::llama_pooling_type::LLAMA_POOLING_TYPE_UNSPECIFIED => Self::Unspecified,
            llama::llama_pooling_type::LLAMA_POOLING_TYPE_NONE => Self::None,
            llama::llama_pooling_type::LLAMA_POOLING_TYPE_MEAN => Self::Mean,
            llama::llama_pooling_type::LLAMA_POOLING_TYPE_CLS => Self::Cls,
            llama::llama_pooling_type::LLAMA_POOLING_TYPE_LAST => Self::Last,
            llama::llama_pooling_type::LLAMA_POOLING_TYPE_RANK => Self::Rank,
            _ => Self::Unspecified,
        }
    }
}

impl Context {
    pub fn new(model: Model, params: &ContextParams) -> Result<Self, ContextCreateError> {
        Self::new_from_c(model, params.as_c())
    }

    pub(crate) fn new_from_c(
        model: Model,
        c_params: llama::llama_context_params,
    ) -> Result<Self, ContextCreateError> {
        // a plain C struct, bindgen just doesn't derive Copy for it
        let context_params = unsafe { std::ptr::read(&c_params) };
        let ctx = unsafe { llama::llama_init_from_model(model.ptr.0, c_params) };
        if ctx.is_null() {
            return Err(ContextCreateError);
        }

        Ok(Self {
            speculative: None,
            ptr: ContextPtr(ctx),
            lora_adapters: Vec::new(),
            model,
            tokens: 0,
            context_params,
        })
    }

    pub fn model(&self) -> &Model {
        &self.model
    }

    pub fn embeddings(&mut self, tokens: &[Token]) -> Result<Vec<f32>, ContextEmbeddingError> {
        let pooling_type = self.pooling_type();
        if pooling_type != PoolingType::Mean {
            return Err(ContextEmbeddingError::UnsupportedPoolingType(pooling_type));
        }

        self.append_tokens(&tokens)?;

        let e = self.embeddings_seq_ith(0)?;
        let s = e.iter().map(|f| f * f).sum::<f32>().sqrt();
        let norm = if s > 0.0 { 1.0 / s } else { 0.0 };
        let mut e = e.to_vec();

        for f in e.iter_mut() {
            *f *= norm;
        }
        Ok(e)
    }

    pub fn n_ctx(&self) -> u32 {
        unsafe { llama::llama_n_ctx(self.ptr.0) }
    }

    pub fn n_batch(&self) -> u32 {
        unsafe { llama::llama_n_batch(self.ptr.0) }
    }

    pub fn n_ubatch(&self) -> u32 {
        unsafe { llama::llama_n_ubatch(self.ptr.0) }
    }

    pub fn state_get(&self) -> Vec<u8> {
        let state_size = unsafe { llama::llama_state_get_size(self.ptr.0) };

        let mut state = vec![0; state_size];
        unsafe {
            llama::llama_state_get_data(self.ptr.0, state.as_mut_ptr(), state_size);
        }
        state
    }

    pub fn state_set(&mut self, data: &[u8]) {
        let read = unsafe { llama::llama_state_set_data(self.ptr.0, data.as_ptr(), data.len()) };
        assert_eq!(read, data.len());
    }

    pub fn memory_clear(&self, clear_data: bool) {
        unsafe {
            let memory = llama::llama_get_memory(self.ptr.0);
            llama::llama_memory_clear(memory, clear_data)
        }
    }

    /// Remove the tokens at positions `p0..p1` (`p1 = None`: to the end) of the
    /// sequence from the memory. Returns false if a partial removal is not
    /// supported by the memory (e.g. a recurrent model).
    pub fn memory_seq_rm(&self, p0: usize, p1: Option<usize>) -> bool {
        self.seq_rm(0, p0, p1)
    }

    /// Number of tokens in the context (the next position)
    pub fn n_past(&self) -> usize {
        self.tokens
    }

    /// Number of recurrent-state snapshots kept for rollback (0 if not supported)
    pub(crate) fn n_rs_seq(&self) -> usize {
        unsafe { llama::llama_n_rs_seq(self.ptr.0) as usize }
    }

    /// Save the partial state (recurrent / SWA caches) of the sequence
    pub(crate) fn state_seq_get_partial(&self) -> Vec<u8> {
        self.seq_state_get_partial(0)
    }

    /// Restore a partial state saved with [`Context::state_seq_get_partial`]
    pub(crate) fn state_seq_set_partial(&self, data: &[u8]) -> bool {
        self.seq_state_set_partial(0, data)
    }

    pub(crate) fn decode(&self, batch: &Batch) -> Result<(), DecodeError> {
        let b = batch.dup_batch();
        let ret = unsafe { llama::llama_decode(self.ptr.0, b) };
        DecodeError::check(ret)
    }

    pub fn append_tokens(&mut self, tokens: &[Token]) -> Result<(), DecodeError> {
        if self.speculative.is_some() {
            return self.speculative_append(tokens);
        }
        if tokens.is_empty() {
            return Ok(());
        }
        const MAX_CHUNKS: usize = 512;
        let sz = std::cmp::min(tokens.len(), MAX_CHUNKS);

        let mut batch = Batch::new(sz, 0, 1);
        for chunks in tokens.chunks(MAX_CHUNKS) {
            for (i, token) in chunks.iter().enumerate() {
                let last = i == chunks.len() - 1;
                batch.append(*token, self.tokens + i, &[0], last);
            }
            self.decode(&batch)?;
            batch.clear();
            self.tokens += chunks.len();
        }
        Ok(())
    }

    /// Sample the next token from the logits of the last decoded token,
    /// `None` at end-of-generation. The caller appends it with
    /// [`Context::append_tokens`]; [`Context::step`] does both, and speculates
    /// when the context was created with [`Context::new_speculative`].
    pub fn next_token<S: Sampler>(&mut self, sampler: &mut S, vocab: &Vocab) -> Option<Token> {
        if self.speculative.is_some() {
            // a speculative context keeps its last sampled token undecoded
            self.speculative_flush()
                .expect("decoding the pending speculative token");
        }
        let new_token = sampler.sample(self, -1);
        (!vocab.is_eog(new_token)).then_some(new_token)
    }

    /// Generate the next token(s): sample from the target, append to the
    /// context and return what was produced. A plain context produces one token
    /// per step; a speculative one produces every token it accepted from the
    /// drafter in this round. `eog` is set at end-of-generation, the
    /// end-of-generation token itself is not returned nor appended.
    pub fn step<S: Sampler>(
        &mut self,
        sampler: &mut S,
        vocab: &Vocab,
    ) -> Result<Step, DecodeError> {
        if self.speculative.is_some() {
            return self.speculative_step(sampler, vocab);
        }
        let token = sampler.sample(self, -1);
        if vocab.is_eog(token) {
            return Ok(Step {
                tokens: Vec::new(),
                eog: true,
            });
        }
        self.append_tokens(&[token])?;
        Ok(Step {
            tokens: vec![token],
            eog: false,
        })
    }

    pub fn pooling_type(&self) -> PoolingType {
        let pooling_type = unsafe { llama::llama_pooling_type(self.ptr.0) };
        pooling_type.into()
    }

    pub fn get_logits(&self, i: i32) -> &[f32] {
        unsafe {
            let logits = llama::llama_get_logits_ith(self.ptr.0, i);
            core::slice::from_raw_parts(logits as *const _, self.model.vocab().n_tokens() as usize)
        }
    }

    pub fn embeddings_seq_ith(&self, i: i32) -> Result<&[f32], EmbeddingSeqError> {
        let n_embd = self.model.n_embd() as usize;

        unsafe {
            let embedding = llama::llama_get_embeddings_seq(self.ptr.0, i);
            if embedding == core::ptr::null_mut() {
                return Err(EmbeddingSeqError(i));
            }

            Ok(core::slice::from_raw_parts(embedding, n_embd))
        }
    }
}

/// Several sequences in one context: a context created with `n_seq_max > 1`
/// holds up to that many independent token sequences (ids `0..n_seq_max`),
/// decoded together in batches built by the caller. These methods leave the
/// single-sequence bookkeeping ([`Context::n_past`]) untouched: the caller
/// tracks the positions of each sequence.
impl Context {
    /// Maximum number of sequences
    pub fn n_seq_max(&self) -> u32 {
        unsafe { llama::llama_n_seq_max(self.ptr.0) }
    }

    /// Decode a batch of tokens of any sequences. The logits of the tokens
    /// appended with `logits` set are then available by their index in the
    /// batch, see [`Context::get_logits`] and [`crate::Sampler::sample`].
    pub fn decode_batch(&mut self, batch: &Batch) -> Result<(), DecodeError> {
        self.decode(batch)
    }

    /// Remove the positions `p0..p1` (`p1 = None`: to the end) of sequence
    /// `seq`. Returns false when the memory cannot remove part of a sequence
    /// (recurrent models), removing a whole sequence always works.
    pub fn seq_rm(&self, seq: i32, p0: usize, p1: Option<usize>) -> bool {
        let p0 = i32::try_from(p0).expect("valid position");
        let p1 = p1.map_or(-1, |p| i32::try_from(p).expect("valid position"));
        unsafe {
            let memory = llama::llama_get_memory(self.ptr.0);
            llama::llama_memory_seq_rm(memory, seq, p0, p1)
        }
    }

    /// Make the positions `p0..p1` (`p1 = None`: to the end) of sequence `src`
    /// part of sequence `dst` too. With a unified KV cache this shares the
    /// cells instead of copying them.
    pub fn seq_cp(&self, src: i32, dst: i32, p0: usize, p1: Option<usize>) {
        let p0 = i32::try_from(p0).expect("valid position");
        let p1 = p1.map_or(-1, |p| i32::try_from(p).expect("valid position"));
        unsafe {
            let memory = llama::llama_get_memory(self.ptr.0);
            llama::llama_memory_seq_cp(memory, src, dst, p0, p1)
        }
    }

    /// The largest position of sequence `seq`, `None` when it is empty
    pub fn seq_pos_max(&self, seq: i32) -> Option<usize> {
        let pos = unsafe {
            let memory = llama::llama_get_memory(self.ptr.0);
            llama::llama_memory_seq_pos_max(memory, seq)
        };
        usize::try_from(pos).ok()
    }

    /// Save the partial state (recurrent / SWA caches) of sequence `seq`, to
    /// restore it with [`Context::seq_state_set_partial`] after more tokens
    /// were decoded: a recurrent state cannot be rolled back otherwise.
    pub fn seq_state_get_partial(&self, seq: i32) -> Vec<u8> {
        let flags = llama::LLAMA_STATE_SEQ_FLAGS_PARTIAL_ONLY;
        unsafe {
            let size = llama::llama_state_seq_get_size_ext(self.ptr.0, seq, flags);
            let mut data = vec![0; size];
            let written = llama::llama_state_seq_get_data_ext(
                self.ptr.0,
                data.as_mut_ptr(),
                size,
                seq,
                flags,
            );
            data.truncate(written);
            data
        }
    }

    /// Restore a partial state saved with [`Context::seq_state_get_partial`]
    /// into sequence `seq`. The positions after the state's must then be
    /// removed with [`Context::seq_rm`].
    pub fn seq_state_set_partial(&self, seq: i32, data: &[u8]) -> bool {
        let flags = llama::LLAMA_STATE_SEQ_FLAGS_PARTIAL_ONLY;
        let read = unsafe {
            llama::llama_state_seq_set_data_ext(self.ptr.0, data.as_ptr(), data.len(), seq, flags)
        };
        read == data.len()
    }

    /// Whether the memory supports removing the end of a sequence, found by
    /// decoding two tokens and removing the last one. The whole memory is
    /// cleared afterwards, call this on a fresh context.
    pub fn probe_seq_rm(&mut self) -> Result<bool, DecodeError> {
        let mut batch = Batch::new(2, 0, 1);
        batch.append(Token(0), 0, &[0], false);
        batch.append(Token(0), 1, &[0], true);
        self.decode(&batch)?;
        let partial = self.seq_rm(0, 1, None);
        self.memory_clear(true);
        self.tokens = 0;
        Ok(partial)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_params_round_trip() {
        let c = unsafe { llama::llama_context_default_params() };
        let params = ContextParams::default();
        let back = params.as_c();
        assert_eq!(back.n_ctx, 16384);
        assert_eq!(back.n_batch, c.n_batch);
        assert_eq!(back.n_ubatch, c.n_ubatch);
        assert_eq!(back.n_threads, c.n_threads);
        assert_eq!(back.flash_attn_type, c.flash_attn_type);
        assert_eq!(back.type_k, c.type_k);
        assert_eq!(back.type_v, c.type_v);
        assert_eq!(back.offload_kqv, c.offload_kqv);
        assert_eq!(back.moe_cache_size, c.moe_cache_size);
        assert_eq!(back.kv_unified, c.kv_unified);
    }

    #[test]
    fn cache_type_round_trip() {
        for ty in [
            CacheType::F32,
            CacheType::F16,
            CacheType::BF16,
            CacheType::Q8_0,
            CacheType::Q4_0,
            CacheType::Q4_1,
            CacheType::IQ4_NL,
            CacheType::Q5_0,
            CacheType::Q5_1,
        ] {
            assert_eq!(CacheType::from_c(ty.to_c()), ty);
        }
    }
}
