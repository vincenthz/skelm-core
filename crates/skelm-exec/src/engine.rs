//! Several generations at once on one model: continuous batching with prompt
//! caching.
//!
//! An [`Engine`] owns one llama.cpp context holding up to `slots` token
//! sequences, driven by its own thread. Each submitted [`Job`] gets a slot,
//! and all the slots are decoded together: every step decodes one batch with
//! the next token of each generating job, plus chunks of the prompts being
//! processed (so a long prompt never stalls the other generations).
//!
//! When a job ends, its tokens stay in its slot. A new job goes to the free slot
//! whose tokens share the longest prefix with its prompt, and only the rest of
//! the prompt is decoded: a conversation sent again with one more turn, or a
//! retried request, skips most of its prompt processing. With models whose
//! memory can share a prefix between sequences, a job can also start from the
//! tokens of another slot (e.g. a long system prompt shared by several agents).
//!
//! The KV cache is unified: every sequence can use the whole context, the
//! slots compete for its cells. A job is admitted when its prompt fits next
//! to the other slots' tokens (idle slots are evicted, least recently used
//! first); when the cache runs out during generation, idle slots are evicted
//! first, then the most recently started job is ended with
//! [`JobEnd::ContextFull`].
//!
//! Recurrent and hybrid models (Mamba, Qwen3.5, ...) cannot remove the end of
//! a sequence: for them, the recurrent state is checkpointed while processing
//! a prompt, just before its last token and where it diverges from the
//! tokens of the other slots (e.g. after a shared system prompt), and a later
//! prompt sharing at least that prefix resumes from the checkpoint.
//!
//! Speculative decoding is not used by the engine.

use std::collections::VecDeque;
use std::sync::mpsc;
use std::thread::JoinHandle;

use skelm_llama_cpp::{self as llama, Batch, DecodeError, Sampler, Token, Vocab};

use crate::{ContextError, Model};

/// Tokens kept free when admitting a job, for its first generated tokens
const ADMIT_RESERVE: usize = 64;

/// Minimum shared prefix for a job to start from another slot's tokens
const MIN_SHARED_PREFIX: usize = 32;

/// Checkpoints kept per slot (recurrent / hybrid models)
const MAX_CHECKPOINTS: usize = 3;

/// How an [`Engine`] is set up
#[derive(Clone, Debug)]
pub struct EngineConfig {
    /// maximum number of jobs generating at the same time
    pub slots: usize,
    /// context size in tokens, shared by all the slots (a single job can use
    /// all of it)
    pub n_ctx: u32,
}

impl Default for EngineConfig {
    fn default() -> Self {
        EngineConfig {
            slots: 4,
            n_ctx: llama::ContextParams::default().n_ctx,
        }
    }
}

/// What happens to a [`Job`], reported to its [`JobSink`]
#[derive(Debug)]
pub enum JobEvent {
    /// The job got a slot, `cached_tokens` of its prompt were already in it
    Started {
        prompt_tokens: usize,
        cached_tokens: usize,
    },
    /// A generated token, with its text (empty for a control token). The
    /// end-of-generation token is not reported.
    Token {
        token: Token,
        piece: Vec<u8>,
        control: bool,
    },
    /// The job is over, nothing is reported after this
    Finished(JobEnd),
}

/// Why a job ended
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JobEnd {
    /// the model generated an end-of-generation token
    Eog,
    /// `max_tokens` tokens were generated
    MaxTokens,
    /// the context has no room left for the job (or its prompt is too long)
    ContextFull,
    /// the sink returned [`Flow::Stop`], or was cancelled
    Stopped,
    /// decoding failed
    Error(String),
}

/// Whether a job goes on, returned by [`JobSink::event`]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flow {
    Continue,
    Stop,
}

/// Receives the [`JobEvent`]s of a job, on the engine thread (so keep it quick)
pub trait JobSink: Send {
    fn event(&mut self, event: JobEvent) -> Flow;

    /// Polled at every step: a cancelled job is dropped, even while it waits
    /// for a slot or processes its prompt (e.g. its client disconnected)
    fn cancelled(&self) -> bool {
        false
    }
}

/// Creates the sampler of a job, on the engine thread (samplers are not `Send`)
pub type SamplerFactory = Box<dyn FnOnce() -> Box<dyn Sampler> + Send>;

/// A generation: a prompt to continue, and where the tokens go
pub struct Job {
    /// the prompt, tokenized (with its special tokens)
    pub prompt: Vec<Token>,
    pub sampler: SamplerFactory,
    /// stop after this many generated tokens
    pub max_tokens: Option<usize>,
    pub sink: Box<dyn JobSink>,
}

/// A model serving several jobs at once, see the [module](self) documentation.
/// Dropping the engine finishes the submitted jobs, then stops its thread.
pub struct Engine {
    jobs: Option<mpsc::Sender<Job>>,
    thread: Option<JoinHandle<()>>,
    slots: usize,
    n_ctx: usize,
}

impl Engine {
    /// Create the context of the engine for `model` (with the model's context
    /// parameters and LoRA adapters, but the context size and number of
    /// sequences of `config`), and start its thread
    pub fn new(model: &Model, config: &EngineConfig) -> Result<Self, ContextError> {
        let base = &model.options.params;
        // llama.cpp handles up to 256 sequences, and each needs a batch entry
        let slots = config.slots.clamp(1, 256).min(base.n_batch.max(1) as usize);
        let params = llama::ContextParams {
            n_ctx: config.n_ctx,
            n_seq_max: u32::try_from(slots).expect("valid number of slots"),
            kv_unified: true,
            embeddings: false,
            ..base.clone()
        };
        let mut ctx = model.model.new_context(&params)?;
        if !model.options.lora.is_empty() {
            ctx.set_lora_adapters(&model.options.lora)?;
        }
        let partial_rm = ctx.probe_seq_rm()?;
        let n_ctx = ctx.n_ctx() as usize;
        let n_batch = ctx.n_batch() as usize;

        let (jobs, rx) = mpsc::channel();
        let vocab = model.vocab.clone();
        // the worker holds the samplers, which are not `Send`: it is built on
        // its thread
        let thread = std::thread::Builder::new()
            .name("skelm-engine".into())
            .spawn(move || {
                Worker {
                    batch: Batch::new(n_batch, 0, 1),
                    ctx,
                    vocab,
                    jobs: rx,
                    open: true,
                    queue: VecDeque::new(),
                    slots: (0..slots)
                        .map(|i| Slot {
                            seq: i as i32,
                            cache: Vec::new(),
                            shared: 0,
                            checkpoints: Vec::new(),
                            last_used: 0,
                            task: None,
                        })
                        .collect(),
                    partial_rm,
                    n_ctx,
                    n_batch,
                    tick: 0,
                }
                .run()
            })
            .expect("spawning the engine thread");
        Ok(Engine {
            jobs: Some(jobs),
            thread: Some(thread),
            slots,
            n_ctx,
        })
    }

    /// Queue a job. The job is handed back if the engine thread is gone (it
    /// panicked).
    pub fn submit(&self, job: Job) -> Result<(), Job> {
        let jobs = self.jobs.as_ref().expect("engine running");
        jobs.send(job).map_err(|e| e.0)
    }

    /// Maximum number of jobs generating at the same time
    pub fn slots(&self) -> usize {
        self.slots
    }

    /// Context size in tokens, shared by the slots
    pub fn n_ctx(&self) -> usize {
        self.n_ctx
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        // closing the channel lets the thread finish its jobs and return
        self.jobs.take();
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

/// A partial state of a slot's sequence, after its first `pos` tokens
struct Checkpoint {
    pos: usize,
    state: Vec<u8>,
}

struct Slot {
    /// the sequence id of the slot in the context
    seq: i32,
    /// the tokens in the context for this sequence, at positions `0..len`
    cache: Vec<Token>,
    /// leading tokens of `cache` shared with another sequence (copied with
    /// `seq_cp`), not counted as using cells of their own
    shared: usize,
    checkpoints: Vec<Checkpoint>,
    /// when the slot was last used, to evict the least recently used first
    last_used: u64,
    task: Option<Task>,
}

impl Slot {
    /// Estimated number of KV cells used by the slot
    fn cells(&self) -> usize {
        self.cache.len() - self.shared.min(self.cache.len())
    }

    /// Forget the tokens after the first `len` (the context is updated by the caller)
    fn truncate(&mut self, len: usize) {
        self.cache.truncate(len);
        self.shared = self.shared.min(len);
        self.checkpoints.retain(|c| c.pos <= len);
    }
}

struct Task {
    sink: Box<dyn JobSink>,
    sampler: Box<dyn Sampler>,
    prompt: Vec<Token>,
    max_tokens: Option<usize>,
    generated: usize,
    /// sampled, to be decoded at the next step
    pending: Option<Token>,
    /// index of the task's logits in the batch being decoded
    logits_at: Option<i32>,
    /// prompt tokens put in the batch being decoded
    batched: usize,
    /// positions to checkpoint while processing the prompt (recurrent models)
    checkpoint_at: Vec<usize>,
    /// admission order
    started: u64,
}

impl Task {
    /// Whether the prompt is still being processed (the slot holds `cached`
    /// tokens of it)
    fn in_prompt(&self, cached: usize) -> bool {
        cached < self.prompt.len()
    }
}

struct Worker {
    ctx: llama::Context,
    vocab: Vocab,
    jobs: mpsc::Receiver<Job>,
    /// the job channel is still open
    open: bool,
    queue: VecDeque<Job>,
    slots: Vec<Slot>,
    /// the memory can remove the end of a sequence (no checkpoints needed)
    partial_rm: bool,
    n_ctx: usize,
    n_batch: usize,
    batch: Batch,
    tick: u64,
}

fn common_prefix(a: &[Token], b: &[Token]) -> usize {
    a.iter().zip(b).take_while(|(x, y)| x == y).count()
}

impl Worker {
    fn run(mut self) {
        loop {
            if !self.receive() {
                return;
            }
            self.drop_cancelled();
            self.admit();
            if self.slots.iter().any(|s| s.task.is_some()) {
                self.step();
            }
        }
    }

    fn active(&self) -> usize {
        self.slots.iter().filter(|s| s.task.is_some()).count()
    }

    /// Collect the submitted jobs, waiting for one when there is nothing to
    /// do. False when the engine is dropped and every job is done.
    fn receive(&mut self) -> bool {
        if self.open && self.queue.is_empty() && self.active() == 0 {
            match self.jobs.recv() {
                Ok(job) => self.queue.push_back(job),
                Err(_) => self.open = false,
            }
        }
        while self.open {
            match self.jobs.try_recv() {
                Ok(job) => self.queue.push_back(job),
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => self.open = false,
            }
        }
        self.open || !self.queue.is_empty() || self.active() > 0
    }

    fn drop_cancelled(&mut self) {
        let mut i = 0;
        while i < self.queue.len() {
            if self.queue[i].sink.cancelled() {
                let mut job = self.queue.remove(i).expect("queued job");
                job.sink.event(JobEvent::Finished(JobEnd::Stopped));
            } else {
                i += 1;
            }
        }
        for i in 0..self.slots.len() {
            if self.slots[i]
                .task
                .as_ref()
                .is_some_and(|t| t.sink.cancelled())
            {
                self.finish(i, JobEnd::Stopped);
            }
        }
    }

    /// End the task of slot `i`, keeping its tokens for later jobs
    fn finish(&mut self, i: usize, end: JobEnd) {
        self.tick += 1;
        let slot = &mut self.slots[i];
        slot.last_used = self.tick;
        if let Some(mut task) = slot.task.take() {
            if matches!(end, JobEnd::Error(_)) {
                // whatever the failed decode left in the sequence is unknown
                self.ctx.seq_rm(slot.seq, 0, None);
                slot.truncate(0);
            }
            task.sink.event(JobEvent::Finished(end));
        }
    }

    /// Remove every token of slot `i` from the context
    fn evict(&mut self, i: usize) {
        let slot = &mut self.slots[i];
        self.ctx.seq_rm(slot.seq, 0, None);
        slot.truncate(0);
        slot.checkpoints.clear();
    }

    /// Start the queued jobs that fit, in order
    fn admit(&mut self) {
        while let Some(mut job) = self.queue.pop_front() {
            let reject = if job.prompt.is_empty() {
                Some(JobEnd::Error("empty prompt".into()))
            } else if job.prompt.len() >= self.n_ctx {
                Some(JobEnd::ContextFull)
            } else {
                None
            };
            if let Some(end) = reject {
                job.sink.event(JobEvent::Finished(end));
                continue;
            }
            match self.place(&job.prompt) {
                Some((slot, source)) => self.start(slot, source, job),
                None => {
                    self.queue.push_front(job);
                    return;
                }
            }
        }
    }

    /// The free slot for a prompt, and the slot to copy a shared prefix from
    /// (with its length), making room in the context. `None` when the job
    /// has to wait.
    fn place(&mut self, prompt: &[Token]) -> Option<(usize, Option<(usize, usize)>)> {
        // the free slot sharing the longest prefix, else an empty one, else
        // the least recently used
        let (slot, lcp) = self
            .slots
            .iter()
            .enumerate()
            .filter(|(_, s)| s.task.is_none())
            .map(|(i, s)| (i, common_prefix(&s.cache, prompt), s))
            .max_by_key(|(_, lcp, s)| (*lcp, s.cache.is_empty(), std::cmp::Reverse(s.last_used)))
            .map(|(i, lcp, _)| (i, lcp))?;

        // a longer prefix in another slot can be shared, when the memory
        // supports sequences diverging after a common prefix
        let mut source = None;
        if self.partial_rm {
            source = self
                .slots
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != slot)
                .map(|(i, s)| (i, common_prefix(&s.cache, prompt).min(prompt.len() - 1)))
                .filter(|(_, len)| *len >= MIN_SHARED_PREFIX && *len > lcp)
                .max_by_key(|(_, len)| *len);
        }

        loop {
            let own = match source {
                Some((_, len)) => prompt.len() - len,
                None => prompt.len(),
            };
            let others: usize = self
                .slots
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != slot)
                .map(|(_, s)| s.cells())
                .sum();
            if others + own + ADMIT_RESERVE <= self.n_ctx {
                return Some((slot, source));
            }
            // evict the least recently used idle slot (keeping the source)
            let victim = self
                .slots
                .iter()
                .enumerate()
                .filter(|(i, s)| {
                    *i != slot
                        && s.task.is_none()
                        && !s.cache.is_empty()
                        && source.is_none_or(|(src, _)| src != *i)
                })
                .min_by_key(|(_, s)| s.last_used)
                .map(|(i, _)| i);
            match victim {
                Some(i) => self.evict(i),
                // the source itself is in the way
                None if source.is_some_and(|(src, _)| self.slots[src].task.is_none()) => {
                    source = None
                }
                // nothing else running: the job gets the whole context
                None if self.active() == 0 => return Some((slot, source)),
                None => return None,
            }
        }
    }

    /// Set slot `i` up for `job`: reuse what can be of its tokens (or of the
    /// source's), and report the start
    fn start(&mut self, i: usize, source: Option<(usize, usize)>, job: Job) {
        let prompt = job.prompt;
        // where the prompt diverges from the tokens of the slots
        let branch = self
            .slots
            .iter()
            .map(|s| common_prefix(&s.cache, &prompt))
            .max()
            .unwrap_or(0);
        match source {
            Some((src, len)) => {
                let (seq, src_seq) = (self.slots[i].seq, self.slots[src].seq);
                self.ctx.seq_rm(seq, 0, None);
                self.ctx.seq_cp(src_seq, seq, 0, Some(len));
                let slot = &mut self.slots[i];
                slot.cache = prompt[..len].to_vec();
                slot.shared = len;
                slot.checkpoints.clear();
            }
            None => {
                // at least the last prompt token is decoded, for its logits
                let keep = common_prefix(&self.slots[i].cache, &prompt).min(prompt.len() - 1);
                if keep < self.slots[i].cache.len() {
                    self.rollback(i, keep);
                }
            }
        }

        self.tick += 1;
        let cached = self.slots[i].cache.len();
        let mut checkpoint_at = Vec::new();
        if !self.partial_rm {
            let last = prompt.len() - 1;
            checkpoint_at.extend(
                [branch, last]
                    .into_iter()
                    .filter(|&p| p > cached && p <= last),
            );
            checkpoint_at.dedup();
        }
        let mut task = Task {
            sink: job.sink,
            sampler: (job.sampler)(),
            max_tokens: job.max_tokens,
            generated: 0,
            pending: None,
            logits_at: None,
            batched: 0,
            checkpoint_at,
            started: self.tick,
            prompt,
        };
        let flow = task.sink.event(JobEvent::Started {
            prompt_tokens: task.prompt.len(),
            cached_tokens: cached,
        });
        self.slots[i].task = Some(task);
        if flow == Flow::Stop {
            self.finish(i, JobEnd::Stopped);
        }
    }

    /// Keep at most the first `keep` tokens of slot `i`: all of them when the
    /// memory can remove the rest, else up to the closest checkpoint, else none
    fn rollback(&mut self, i: usize, keep: usize) {
        let seq = self.slots[i].seq;
        if self.partial_rm && self.ctx.seq_rm(seq, keep, None) {
            self.slots[i].truncate(keep);
            return;
        }
        let slot = &self.slots[i];
        if let Some(cp) = slot
            .checkpoints
            .iter()
            .filter(|c| c.pos <= keep && c.pos <= slot.cache.len())
            .max_by_key(|c| c.pos)
        {
            let pos = cp.pos;
            if self.ctx.seq_state_set_partial(seq, &cp.state) && self.ctx.seq_rm(seq, pos, None) {
                self.slots[i].truncate(pos);
                return;
            }
        }
        self.evict(i);
    }

    /// Decode one batch for every active task, and sample their next tokens
    fn step(&mut self) {
        self.batch.clear();

        // the next token of each generating task
        for i in 0..self.slots.len() {
            let cached = self.slots[i].cache.len();
            let Some(task) = &mut self.slots[i].task else {
                continue;
            };
            task.logits_at = None;
            task.batched = 0;
            let Some(token) = task.pending else {
                continue;
            };
            if cached >= self.n_ctx {
                self.finish(i, JobEnd::ContextFull);
                continue;
            }
            task.logits_at = Some(self.batch.len() as i32);
            self.batch.append(token, cached, &[self.slots[i].seq], true);
        }

        // then prompt chunks, the oldest jobs first
        let mut prompts: Vec<usize> = (0..self.slots.len())
            .filter(|&i| {
                let slot = &self.slots[i];
                slot.task
                    .as_ref()
                    .is_some_and(|t| t.in_prompt(slot.cache.len()))
            })
            .collect();
        prompts.sort_by_key(|&i| self.slots[i].task.as_ref().map(|t| t.started));
        for i in prompts {
            let room = self.n_batch - self.batch.len();
            if room == 0 {
                break;
            }
            let slot = &mut self.slots[i];
            let task = slot.task.as_mut().expect("active task");
            let start = slot.cache.len();
            let last = task.prompt.len() - 1;
            let mut end = task.prompt.len().min(start + room);
            if let Some(&pos) = task.checkpoint_at.iter().find(|&&pos| pos > start) {
                // stop there, to checkpoint
                end = end.min(pos);
            }
            for pos in start..end {
                self.batch
                    .append(task.prompt[pos], pos, &[slot.seq], pos == last);
            }
            task.batched = end - start;
            if end == task.prompt.len() {
                task.logits_at = Some(self.batch.len() as i32 - 1);
            }
        }

        if self.batch.is_empty() {
            return;
        }
        match self.ctx.decode_batch(&self.batch) {
            Ok(()) => {}
            Err(DecodeError::CannotFindKVSlot) => {
                self.make_room();
                return;
            }
            Err(e) => {
                let in_batch: Vec<usize> = (0..self.slots.len())
                    .filter(|&i| {
                        self.slots[i]
                            .task
                            .as_ref()
                            .is_some_and(|t| t.logits_at.is_some() || t.batched > 0)
                    })
                    .collect();
                for i in in_batch {
                    self.finish(i, JobEnd::Error(e.to_string()));
                }
                return;
            }
        }

        for i in 0..self.slots.len() {
            self.after_decode(i);
        }
    }

    /// The batch did not fit in the context: evict the least recently used
    /// idle slot, or else end the most recently started task
    fn make_room(&mut self) {
        let idle = self
            .slots
            .iter()
            .enumerate()
            .filter(|(_, s)| s.task.is_none() && !s.cache.is_empty())
            .min_by_key(|(_, s)| s.last_used)
            .map(|(i, _)| i);
        if let Some(i) = idle {
            self.evict(i);
            return;
        }
        let newest = self
            .slots
            .iter()
            .enumerate()
            .filter_map(|(i, s)| Some((i, s.task.as_ref()?.started)))
            .max_by_key(|(_, started)| *started)
            .map(|(i, _)| i);
        if let Some(i) = newest {
            self.finish(i, JobEnd::ContextFull);
            self.evict(i);
        }
    }

    /// Record what slot `i` decoded in the batch, and sample its next token
    fn after_decode(&mut self, i: usize) {
        let slot = &mut self.slots[i];
        let Some(task) = &mut slot.task else {
            return;
        };
        if task.batched > 0 {
            let start = slot.cache.len();
            slot.cache
                .extend_from_slice(&task.prompt[start..start + task.batched]);
            let pos = slot.cache.len();
            if task.checkpoint_at.contains(&pos) && !slot.checkpoints.iter().any(|c| c.pos == pos) {
                if slot.checkpoints.len() == MAX_CHECKPOINTS {
                    slot.checkpoints.remove(0);
                }
                slot.checkpoints.push(Checkpoint {
                    pos,
                    state: self.ctx.seq_state_get_partial(slot.seq),
                });
            }
        } else if let Some(token) = task.pending.take() {
            slot.cache.push(token);
        }

        let Some(idx) = task.logits_at else {
            return;
        };
        let token = task.sampler.sample(&self.ctx, idx);
        if self.vocab.is_eog(token) {
            self.finish(i, JobEnd::Eog);
            return;
        }
        let control = self.vocab.token_attr(token).is_control();
        let piece = if control {
            Vec::new()
        } else {
            self.vocab.as_bytes(token)
        };
        task.generated += 1;
        let flow = task.sink.event(JobEvent::Token {
            token,
            piece,
            control,
        });
        if flow == Flow::Stop {
            self.finish(i, JobEnd::Stopped);
        } else if task.max_tokens.is_some_and(|max| task.generated >= max) {
            self.finish(i, JobEnd::MaxTokens);
        } else {
            task.pending = Some(token);
        }
    }
}
