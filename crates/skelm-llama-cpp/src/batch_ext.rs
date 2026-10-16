//! The extended batch API (`llama_batch_ext`), needed to attach a hidden-state
//! row to each token, which MTP contexts take as input.

use skelm_llama_cpp_sys::llama;

use crate::Context;
use crate::context::DecodeError;
use crate::token::Token;

pub(crate) struct BatchExt {
    ptr: *mut llama::llama_batch_ext,
}

impl Drop for BatchExt {
    fn drop(&mut self) {
        unsafe { llama::llama_batch_ext_free(self.ptr) }
    }
}

impl BatchExt {
    /// A batch for `context` (the batch is sized from the context's parameters)
    pub(crate) fn new(context: &Context) -> Self {
        let ptr = unsafe { llama::llama_batch_ext_init(context.ptr.0) };
        assert!(!ptr.is_null(), "llama_batch_ext_init failed");
        BatchExt { ptr }
    }

    pub(crate) fn clear(&mut self) {
        unsafe { llama::llama_batch_ext_clear(self.ptr) }
    }

    /// Add `token` at `pos` in sequence 0, returning its index in the batch
    pub(crate) fn add(&mut self, token: Token, pos: usize, output: bool) -> i32 {
        let idx = unsafe { llama::llama_batch_ext_add_token(self.ptr, 0, token.0) };
        assert!(idx >= 0, "llama_batch_ext_add_token failed: {idx}");
        let pos = [i32::try_from(pos).expect("valid position")];
        unsafe {
            assert!(llama::llama_batch_ext_set_pos(self.ptr, idx, pos.as_ptr()));
            assert!(llama::llama_batch_ext_set_output_logits(
                self.ptr, idx, output
            ));
        }
        idx
    }

    /// Attach a hidden-state row to the token at `idx` (the data is copied)
    pub(crate) fn set_state(&mut self, idx: i32, row: &[f32]) {
        let embd = llama::llama_embd {
            data: row.as_ptr(),
            n_rows: 1,
            n_embd: row.len(),
        };
        let ok = unsafe { llama::llama_batch_ext_set_embd_state(self.ptr, idx, embd) };
        assert!(ok, "llama_batch_ext_set_embd_state failed");
    }

    /// Decode the batch on `context`
    pub(crate) fn decode(&mut self, context: &Context) -> Result<(), DecodeError> {
        let ret = unsafe {
            llama::llama_process(
                context.ptr.0,
                llama::llama_process_type::LLAMA_PROCESS_TYPE_DECODE,
                self.ptr,
            )
        };
        DecodeError::check(ret)
    }
}
