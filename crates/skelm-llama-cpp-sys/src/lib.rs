#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn)]

extern crate link_cplusplus;

pub mod ggml {
    include!(concat!(env!("OUT_DIR"), "/bindings_ggml.rs"));
}
pub mod llama {
    include!(concat!(env!("OUT_DIR"), "/bindings_llama.rs"));
}

/// C ABI wrappers over the staging API of llama.cpp's `src/llama-ext.h`, used
/// for MTP speculative decoding (see `shim/skelm-llama-ext.cpp`). That API is
/// work in progress upstream and may change with any llama.cpp update.
pub mod ext {
    use super::llama::llama_context;

    unsafe extern "C" {
        /// Set whether the context outputs nextn (MTP input) embeddings. With
        /// `masked`, only for the tokens whose logits are output.
        pub fn skelm_llama_set_embeddings_nextn(ctx: *mut llama_context, value: bool, masked: bool);
        /// Select which appended nextn block an MTP context runs.
        pub fn skelm_llama_set_nextn_layer_offset(ctx: *mut llama_context, offset: i32);
        /// The nextn embeddings of the last decoded batch, one row per token.
        pub fn skelm_llama_get_embeddings_nextn(ctx: *mut llama_context) -> *mut f32;
        /// The nextn embeddings row of the `i`th token of the last decoded batch.
        pub fn skelm_llama_get_embeddings_nextn_ith(ctx: *mut llama_context, i: i32) -> *mut f32;
        /// The `ctx_other` a context was created with, if the model uses it.
        pub fn skelm_llama_get_ctx_other(ctx: *mut llama_context) -> *mut llama_context;
    }
}
