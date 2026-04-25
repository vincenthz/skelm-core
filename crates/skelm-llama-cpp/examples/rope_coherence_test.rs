//! KV Cache Round-Trip Coherence Test (universal across llama.cpp backends).
//!
//! Verifies that `state_seq_get` followed by `state_seq_set` preserves a
//! sequence's KV state byte-identically, on every memory backend llama.cpp
//! supports (unified KV cache, interleaved sliding-window, recurrent /
//! Mamba, hybrid attn+recurrent, hybrid+iswa) and on every RoPE variant
//! (standard, M-RoPE).
//!
//! The algorithm uses only forward-decode and full-sequence round-trip:
//!
//! ```text
//! tokens = tokenize(prompt)            // length N, require N >= 2
//! head, tail = tokens[..N-1], tokens[N-1]
//!
//! baseline = generate after append_tokens(tokens)
//!
//! ctx_src.append_tokens(head)
//! kv = ctx_src.state_seq_get(0)        // serializes positions 0..N-2
//!
//! ctx_dst.state_seq_set(kv, 0)         // restores positions 0..N-2
//! ctx_dst.set_tokens(N - 1)
//! ctx_dst.append_tokens(tail)          // decodes position N-1, fresh logits
//! composed = generate
//!
//! assert baseline == composed
//! ```
//!
//! Why this is universal:
//!
//! - Never calls `seq_rm`, `seq_add`, or partial `seq_cp`. Every backend
//!   that aborts on those primitives is unaffected (recurrent rejects
//!   partial-tail `seq_rm`; iswa asserts on partial cross-stream `seq_cp`;
//!   recurrent / STEP35 reject `seq_add`).
//! - Satisfies M-RoPE's strict `Y > X` position invariant: `head` loads
//!   max position `N-2`; `tail` appends at `N-1`.
//! - `state_seq_get` / `state_seq_set` are the only primitive implemented
//!   identically across all five backends.
//! - Logits at the boundary are produced by the forward decode of `tail`,
//!   so no rewind-and-redecode trick is needed.
//!
//! Position-shifted KV composition (RoPE-baked-into-K vectors) is not
//! universally viable; that probe lives in `kv_composition_probe.rs` as a
//! documented negative result, not a regression test.
//!
//! Usage:
//!
//! ```text
//! cargo run --release --example rope_coherence_test -- path/to/model.gguf
//! ```

use skelm_llama_cpp::{
    Context, ContextParams, Model, ModelParams, SamplerGreedy, Token, Vocab,
};

fn tokenize(vocab: &Vocab, text: &str) -> Vec<Token> {
    vocab.tokenize(text.as_bytes(), false)
}

fn generate(ctx: &mut Context, vocab: &Vocab, n_tokens: usize) -> String {
    let mut sampler = SamplerGreedy;
    let mut output = String::new();
    for _ in 0..n_tokens {
        let Some(token) = ctx.next_token(&mut sampler, vocab) else {
            break;
        };
        output.push_str(&vocab.as_string_lossy(token));
        ctx.append_tokens(&[token]).expect("decode failed");
    }
    output
}

fn print_section(title: &str) {
    println!("\n{}", "=".repeat(70));
    println!("  {}", title);
    println!("{}", "=".repeat(70));
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <model_path>", args[0]);
        std::process::exit(2);
    }
    let model_path = &args[1];

    println!("Loading model: {}", model_path);
    let model = Model::load(model_path, &ModelParams::default()).expect("failed to load model");
    let vocab = model.vocab();

    let ctx_params = ContextParams {
        n_ctx: 2048,
        ..Default::default()
    };

    let prompt = "The capital of France is Paris. The Eiffel Tower is located in";
    let tokens = tokenize(&vocab, prompt);
    assert!(tokens.len() >= 2, "prompt must tokenize to at least 2 tokens");
    let n = tokens.len();
    let head = &tokens[..n - 1];
    let tail = tokens[n - 1];
    let gen_tokens = 30;

    print_section("Round-trip KV coherence test");
    println!("Prompt tokens: {} (head: {}, tail: 1)", n, head.len());

    let mut ctx_base = model.new_context(&ctx_params).expect("ctx failed");
    ctx_base.append_tokens(&tokens).expect("baseline prefill failed");
    let baseline = generate(&mut ctx_base, &vocab, gen_tokens);
    println!("Baseline: {}|{}", prompt, baseline);
    drop(ctx_base);

    let mut ctx_src = model.new_context(&ctx_params).expect("ctx failed");
    ctx_src.append_tokens(head).expect("head prefill failed");
    let kv = ctx_src.state_seq_get(0);
    println!("Serialized seq 0: {} bytes ({} positions)", kv.len(), head.len());
    drop(ctx_src);

    let mut ctx_dst = model.new_context(&ctx_params).expect("ctx failed");
    let bytes_loaded = ctx_dst.state_seq_set(&kv, 0);
    assert!(
        bytes_loaded > 0,
        "state_seq_set returned 0 bytes — round-trip is broken at the load step"
    );
    ctx_dst.set_tokens(n - 1);
    ctx_dst
        .append_tokens(&[tail])
        .expect("tail decode failed after state_seq_set");
    let composed = generate(&mut ctx_dst, &vocab, gen_tokens);
    println!("Composed: {}|{}", prompt, composed);
    drop(ctx_dst);

    print_section("VERDICT");
    if baseline == composed {
        println!("PASS — round-trip produces byte-identical output.");
        println!();
        println!("state_seq_get / state_seq_set preserve KV state for this model.");
    } else {
        println!("FAIL — round-trip diverges. state_seq_get / state_seq_set is not");
        println!("       byte-preserving on this backend, or append_tokens after");
        println!("       a fresh state load does not produce equivalent logits.");
        println!("  Baseline: {}", baseline);
        println!("  Composed: {}", composed);
        std::process::exit(1);
    }
}
