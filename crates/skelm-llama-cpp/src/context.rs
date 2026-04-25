use std::sync::Arc;

use skelm_llama_cpp_sys::llama;
use thiserror::Error;

use crate::{Model, Sampler, Vocab, batch::Batch, token::Token};

#[allow(dead_code)]
pub struct Context {
    pub(crate) model: Model,
    pub(crate) tokens: usize,
    pub(crate) context_params: llama::llama_context_params,
    pub(crate) ptr: Arc<ContextPtr>,
}

unsafe impl Send for Context {}

pub struct ContextPtr(pub(crate) *mut llama::llama_context);

unsafe impl Send for ContextPtr {}
unsafe impl Sync for ContextPtr {}

impl Drop for ContextPtr {
    fn drop(&mut self) {
        unsafe {
            llama::llama_free(self.0);
        }
    }
}

pub struct ContextParams {
    pub n_ctx: u32,
    pub embeddings: bool,
}

impl Default for ContextParams {
    fn default() -> Self {
        let mut context = unsafe { llama::llama_context_default_params() };
        context.n_ctx = 16384;
        Self {
            n_ctx: context.n_ctx,
            embeddings: context.embeddings,
        }
    }
}

impl ContextParams {
    fn as_c(&self) -> llama::llama_context_params {
        let mut context = unsafe { llama::llama_context_default_params() };
        context.n_ctx = self.n_ctx;
        context.embeddings = self.embeddings;
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
        let c_params = params.as_c();
        let c_params_clone = params.as_c();
        let ctx = unsafe { llama::llama_init_from_model(model.ptr.0, c_params) };
        if ctx.is_null() {
            return Err(ContextCreateError);
        }

        Ok(Self {
            model,
            tokens: 0,
            context_params: c_params_clone,
            ptr: Arc::new(ContextPtr(ctx)),
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

    /// Serializes the KV state of a single sequence into a buffer.
    pub fn state_seq_get(&self, seq_id: i32) -> Vec<u8> {
        let size = unsafe { llama::llama_state_seq_get_size(self.ptr.0, seq_id) };
        let mut buf = vec![0u8; size];
        unsafe {
            llama::llama_state_seq_get_data(self.ptr.0, buf.as_mut_ptr(), size, seq_id);
        }
        buf
    }

    /// Loads previously-serialized sequence KV state into `dest_seq_id`.
    /// Returns the number of bytes consumed; zero indicates failure.
    pub fn state_seq_set(&mut self, data: &[u8], dest_seq_id: i32) -> usize {
        unsafe {
            llama::llama_state_seq_set_data(self.ptr.0, data.as_ptr(), data.len(), dest_seq_id)
        }
    }

    /// Returns a read-only handle to this context's memory (KV cache and related state).
    ///
    /// The handle holds a refcounted clone of the context's underlying llama_context
    /// (mirroring how `Context` itself holds `Model` by value). The KV memory therefore
    /// stays alive as long as either the originating `Context` or any outstanding
    /// `Memory` handle exists, and is freed exactly when the last reference drops.
    /// For mutating operations use [`memory_mut`](Self::memory_mut).
    pub fn memory(&self) -> Memory {
        Memory {
            ptr: self.ptr.clone(),
        }
    }

    /// Returns a mutating handle to this context's memory (KV cache and related state).
    ///
    /// Takes `&mut self`; the wrapper holds the exclusive borrow for its entire
    /// lifetime, so the borrow checker enforces single-writer access for the
    /// duration of any KV mutations.
    pub fn memory_mut(&mut self) -> MemoryMut<'_> {
        MemoryMut { ctx: self }
    }

    /// Number of tokens the context has decoded into its KV cache.
    pub fn tokens(&self) -> usize {
        self.tokens
    }

    /// Override the internal token-position counter.
    ///
    /// Only meaningful after a corresponding [`MemoryMut::seq_rm`] (or similar)
    /// has rolled back the actual KV state. Misuse desynchronises the counter
    /// from the cache and produces wrong-position decodes.
    pub fn set_tokens(&mut self, n: usize) {
        self.tokens = n;
    }

    fn decode(&self, batch: &Batch) -> Result<(), DecodeError> {
        let b = batch.dup_batch();
        let ret = unsafe { llama::llama_decode(self.ptr.0, b) };
        match ret {
            0 => Ok(()),
            1 => Err(DecodeError::CannotFindKVSlot),
            2 => Err(DecodeError::Aborted),
            -1 => Err(DecodeError::InvalidBatch),
            _ if ret > 0 => Err(DecodeError::UnspecifiedWarning(ret)),
            _ => Err(DecodeError::FatalError(ret)),
        }
    }

    pub fn append_tokens(&mut self, tokens: &[Token]) -> Result<(), DecodeError> {
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

    pub fn next_token<S: Sampler>(&mut self, sampler: &mut S, vocab: &Vocab) -> Option<Token> {
        let new_token = sampler.sample(self, -1);
        (!vocab.is_eog(new_token)).then_some(new_token)
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

/// Read-only handle to a context's memory (KV cache and related state).
///
/// Returned from [`Context::memory`]. Owns a refcounted clone of the underlying
/// `llama_context` (mirroring the `Context`-holds-`Model` ownership pattern), so
/// the cache stays alive while any `Memory` exists and is freed exactly when the
/// last handle drops.
pub struct Memory {
    ptr: Arc<ContextPtr>,
}

impl Memory {
    /// Smallest position currently held in the cache for the given sequence id.
    pub fn seq_pos_min(&self, seq_id: i32) -> i32 {
        unsafe {
            let raw = llama::llama_get_memory(self.ptr.0);
            llama::llama_memory_seq_pos_min(raw, seq_id)
        }
    }

    /// Largest position currently held in the cache for the given sequence id.
    pub fn seq_pos_max(&self, seq_id: i32) -> i32 {
        unsafe {
            let raw = llama::llama_get_memory(self.ptr.0);
            llama::llama_memory_seq_pos_max(raw, seq_id)
        }
    }

    /// Whether this memory implementation supports position shifts via [`MemoryMut::seq_add`].
    pub fn can_shift(&self) -> bool {
        unsafe {
            let raw = llama::llama_get_memory(self.ptr.0);
            llama::llama_memory_can_shift(raw)
        }
    }
}

/// Mutating handle to a context's memory.
///
/// Returned from [`Context::memory_mut`]. Holds an actual `&mut Context` borrow
/// so exclusive access is enforced structurally for the wrapper's lifetime.
pub struct MemoryMut<'a> {
    ctx: &'a mut Context,
}

impl<'a> MemoryMut<'a> {
    /// Clears the entire memory. If `clear_data` is true, underlying buffers are zeroed.
    pub fn clear(&mut self, clear_data: bool) {
        unsafe {
            let raw = llama::llama_get_memory(self.ctx.ptr.0);
            llama::llama_memory_clear(raw, clear_data)
        }
    }

    /// Removes positions `[p0, p1)` of `seq_id` from the cache. `p0 < 0` means -infinity, `p1 < 0` means +infinity.
    /// Returns false if the cache cannot represent the request (e.g. partial removal not supported by the memory backend).
    pub fn seq_rm(&mut self, seq_id: i32, p0: i32, p1: i32) -> bool {
        unsafe {
            let raw = llama::llama_get_memory(self.ctx.ptr.0);
            llama::llama_memory_seq_rm(raw, seq_id, p0, p1)
        }
    }

    /// Copies positions `[p0, p1)` of `src` to `dst`.
    pub fn seq_cp(&mut self, src: i32, dst: i32, p0: i32, p1: i32) {
        unsafe {
            let raw = llama::llama_get_memory(self.ctx.ptr.0);
            llama::llama_memory_seq_cp(raw, src, dst, p0, p1)
        }
    }

    /// Removes everything except `seq_id` from the cache.
    pub fn seq_keep(&mut self, seq_id: i32) {
        unsafe {
            let raw = llama::llama_get_memory(self.ctx.ptr.0);
            llama::llama_memory_seq_keep(raw, seq_id)
        }
    }

    /// Shifts positions `[p0, p1)` of `seq_id` by `delta`. The memory implementation
    /// must support position shifts (see [`Memory::can_shift`]).
    ///
    /// Note: this updates the cache's position metadata only. RoPE is baked into
    /// the K vectors at decode time and is not re-applied here, so generation
    /// after a position-shifted KV injection may diverge from a baseline that
    /// prefilled the equivalent prompt directly. See the rope_coherence_test
    /// example for a measurement harness.
    pub fn seq_add(&mut self, seq_id: i32, p0: i32, p1: i32, delta: i32) {
        unsafe {
            let raw = llama::llama_get_memory(self.ctx.ptr.0);
            llama::llama_memory_seq_add(raw, seq_id, p0, p1, delta)
        }
    }

    /// Divides positions `[p0, p1)` of `seq_id` by `d`.
    pub fn seq_div(&mut self, seq_id: i32, p0: i32, p1: i32, d: i32) {
        unsafe {
            let raw = llama::llama_get_memory(self.ctx.ptr.0);
            llama::llama_memory_seq_div(raw, seq_id, p0, p1, d)
        }
    }
}
