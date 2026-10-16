// C ABI wrappers over the staging API of llama.cpp's `src/llama-ext.h`.
//
// That header declares its functions outside of `extern "C"` (they get C++
// mangled names) and uses C++ types, so it cannot be bound from Rust directly.
// Only the functions needed for MTP speculative decoding are re-exported here;
// their Rust declarations live in `src/lib.rs` (`ext` module).

#include "llama-ext.h"

extern "C" {

void skelm_llama_set_embeddings_nextn(llama_context * ctx, bool value, bool masked) {
    llama_set_embeddings_nextn(ctx, value, masked);
}

void skelm_llama_set_nextn_layer_offset(llama_context * ctx, int32_t offset) {
    llama_set_nextn_layer_offset(ctx, offset);
}

float * skelm_llama_get_embeddings_nextn(llama_context * ctx) {
    return llama_get_embeddings_nextn(ctx);
}

float * skelm_llama_get_embeddings_nextn_ith(llama_context * ctx, int32_t i) {
    return llama_get_embeddings_nextn_ith(ctx, i);
}

llama_context * skelm_llama_get_ctx_other(llama_context * ctx) {
    return llama_get_ctx_other(ctx);
}

}
