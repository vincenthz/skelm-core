/// Prefix Cache Coherence Tests (Tests 4, 5, 6)
///
/// Tests the actual munu-code workflow: serialize a prefix KV cache,
/// deserialize into a fresh context, then incrementally prefill dynamic
/// context on top and generate.
///
/// These tests BLOCK Phase 25a of the munu-code roadmap.
///
/// Usage: cargo run --example prefix_cache_test -- <model_path>

use skelm_llama_cpp::{
    Context, ContextParams, Model, ModelParams, SamplerGreedy, Token, Vocab,
};

fn tokenize(vocab: &Vocab, text: &str) -> Vec<Token> {
    vocab.tokenize(text.as_bytes(), false)
}

/// Re-decode the last token to prime logits after KV cache deserialization.
fn prime_logits(ctx: &mut Context, last_token: Token) {
    let last_pos = (ctx.tokens - 1) as i32;
    ctx.memory_seq_rm(0, last_pos, last_pos + 1);
    ctx.set_tokens(last_pos as usize);
    ctx.append_tokens(&[last_token]).expect("prime decode failed");
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

// Simulated environment text: Munu syntax rules, extern signatures, viewport conventions.
// ~200 tokens of realistic system prompt content.
const ENVIRONMENT: &str = r#"You are a Munu programming assistant. Munu is a functional language with pattern matching and typed holes.

Syntax rules:
- Functions are declared with `mu name(args) -> Type = body`
- Pattern matching uses `| pattern => expression` arms
- Let bindings use `let x = expr` chains
- Spawn creates concurrent tasks with `spawn expr`
- Guards use `| pattern if condition => expression`

Available operations (workshop):
extern mu store_get(path: String) -> String;
extern mu store_put(path: String, content: String) -> ();
extern mu compile_viewport(prefix: String, binding: String, suffix: String) -> bool;
extern mu symbol_search(query: String) -> String;
extern mu symbol_deps(name: String) -> String;
extern mu llm_generate(session: i64, prompt: String) -> String;
extern mu session_create(model: String) -> i64;
extern mu session_rollback(session: i64, n: i64) -> ();

Viewport conventions:
- The viewport has three regions: prefix (context), hole (to be filled), suffix (continuation)
- The LLM generates a binding for the hole
- The compiler validates the binding against the type signature
- WRONG patterns show what failed and why
- RIGHT patterns show the expected shape"#;

const TASK_A: &str = r#"
Task: Generate a binding for the following viewport hole.

File: src/parser.uv
Function: parse_expr

mu parse_expr(tokens: TokenStream) -> Expr =
  let tok = tokens.peek()
  | Tok::Number(n) => Expr::Lit(n)
  | Tok::Ident(name) => Expr::Var(name)
  | Tok::LParen => ???

The hole ??? needs a binding that parses a parenthesized expression.
Available: parse_expr (recursive), tokens.advance(), tokens.expect(Tok::RParen)

Generate the binding:"#;

const TASK_B: &str = r#"
Task: Explain the type error in the following viewport result.

File: src/typechecker.uv
Function: unify

Viewport result: WRONG
  Expected: Type::Fn(Type::Int, Type::Bool)
  Got:      Type::Fn(Type::Int, Type::Int)

The binding was:
  | (Type::Fn(a1, r1), Type::Fn(a2, r2)) =>
    let _ = unify(a1, a2)
    r1

The return type r1 is Type::Int but the expected return is Type::Bool.
The binding returns r1 directly instead of unifying r1 with r2.

Explain why this is wrong and what the correct binding should be:"#;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <model_path>", args[0]);
        std::process::exit(1);
    }
    let model_path = &args[1];

    println!("Loading model: {}", model_path);
    let model = Model::load(model_path, &ModelParams::default()).expect("failed to load model");
    let vocab = model.vocab();

    let ctx_params = ContextParams {
        n_ctx: 4096,
        ..Default::default()
    };

    let gen_tokens = 40;

    let env_tokens = tokenize(&vocab, ENVIRONMENT);
    let task_a_tokens = tokenize(&vocab, TASK_A);
    let task_b_tokens = tokenize(&vocab, TASK_B);

    println!("ENVIRONMENT tokens: {}", env_tokens.len());
    println!("TASK_A tokens: {}", task_a_tokens.len());
    println!("TASK_B tokens: {}", task_b_tokens.len());

    // ===================================================================
    // TEST 4: Prefix cache with incremental prefill
    // ===================================================================
    print_section("TEST 4: prefix_cache_with_incremental_prefill");

    let mut full_tokens_a = env_tokens.clone();
    full_tokens_a.extend_from_slice(&task_a_tokens);

    // Step 1: Baseline — prefill everything from scratch
    println!("\n--- Step 1: Baseline (full prefill from scratch) ---");
    let mut ctx_base = model.new_context(&ctx_params).expect("ctx failed");
    ctx_base.append_tokens(&full_tokens_a).expect("prefill failed");
    let baseline_a = generate(&mut ctx_base, &vocab, gen_tokens);
    println!("Baseline output ({} tokens prefilled):", full_tokens_a.len());
    println!("  {}", baseline_a);
    drop(ctx_base);

    // Step 2: Cache the environment prefix, load it, then incrementally prefill task
    println!("\n--- Step 2: Cached prefix + incremental prefill ---");

    // 2a: Prefill environment, serialize
    let mut ctx_env = model.new_context(&ctx_params).expect("ctx failed");
    ctx_env.append_tokens(&env_tokens).expect("env prefill failed");
    let env_kv = ctx_env.state_seq_get(0);
    println!("Serialized environment KV: {} bytes ({} tokens, positions [0, {}])",
        env_kv.len(), env_tokens.len(), ctx_env.memory_seq_pos_max(0));
    drop(ctx_env);

    // 2b: Load cached env into fresh context, then prefill task on top
    let mut ctx_cached = model.new_context(&ctx_params).expect("ctx failed");
    ctx_cached.state_seq_set(&env_kv, 0);
    ctx_cached.set_tokens(env_tokens.len());
    println!("Loaded env cache: seq 0 range [{}, {}]",
        ctx_cached.memory_seq_pos_min(0), ctx_cached.memory_seq_pos_max(0));

    // Incrementally prefill TASK_A on top of the cached environment
    ctx_cached.append_tokens(&task_a_tokens).expect("task prefill failed");
    println!("After incremental prefill: seq 0 range [{}, {}], tokens = {}",
        ctx_cached.memory_seq_pos_min(0), ctx_cached.memory_seq_pos_max(0),
        ctx_cached.tokens);

    let cached_a = generate(&mut ctx_cached, &vocab, gen_tokens);
    println!("Cached output:");
    println!("  {}", cached_a);
    drop(ctx_cached);

    // Step 3: Compare
    print_section("TEST 4 RESULT");
    if baseline_a == cached_a {
        println!("PASS — cached prefix + incremental prefill produces identical output.");
        println!("Prefix-level caching with incremental prefill is viable for munu-code.");
    } else {
        println!("FAIL — output differs.");
        println!("  Baseline: {}", baseline_a);
        println!("  Cached:   {}", cached_a);
    }
    let t4_pass = baseline_a == cached_a;

    // ===================================================================
    // TEST 5: Prefix cache reuse across different tasks
    // ===================================================================
    print_section("TEST 5: prefix_cache_reuse_across_different_tasks");

    // Step 1: Serialize environment (reuse env_kv from above)
    println!("Reusing serialized environment KV: {} bytes", env_kv.len());

    // Step 2a: Load env, prefill TASK_A
    println!("\n--- Step 2a: env cache + TASK_A ---");
    let mut ctx_d = model.new_context(&ctx_params).expect("ctx failed");
    ctx_d.state_seq_set(&env_kv, 0);
    ctx_d.set_tokens(env_tokens.len());
    ctx_d.append_tokens(&task_a_tokens).expect("task A prefill failed");
    let output_a = generate(&mut ctx_d, &vocab, gen_tokens);
    println!("Output A: {}", output_a);
    drop(ctx_d);

    // Step 2b: Load env, prefill TASK_B
    println!("\n--- Step 2b: env cache + TASK_B ---");
    let mut ctx_e = model.new_context(&ctx_params).expect("ctx failed");
    ctx_e.state_seq_set(&env_kv, 0);
    ctx_e.set_tokens(env_tokens.len());
    ctx_e.append_tokens(&task_b_tokens).expect("task B prefill failed");
    let output_b = generate(&mut ctx_e, &vocab, gen_tokens);
    println!("Output B: {}", output_b);
    drop(ctx_e);

    // Step 3: Baselines from scratch
    println!("\n--- Step 3: Baselines from scratch ---");
    let mut full_tokens_b = env_tokens.clone();
    full_tokens_b.extend_from_slice(&task_b_tokens);

    let mut ctx_base_a = model.new_context(&ctx_params).expect("ctx failed");
    ctx_base_a.append_tokens(&full_tokens_a).expect("prefill failed");
    let baseline_a2 = generate(&mut ctx_base_a, &vocab, gen_tokens);
    println!("Baseline A: {}", baseline_a2);
    drop(ctx_base_a);

    let mut ctx_base_b = model.new_context(&ctx_params).expect("ctx failed");
    ctx_base_b.append_tokens(&full_tokens_b).expect("prefill failed");
    let baseline_b = generate(&mut ctx_base_b, &vocab, gen_tokens);
    println!("Baseline B: {}", baseline_b);
    drop(ctx_base_b);

    // Compare
    print_section("TEST 5 RESULT");
    let t5a_pass = output_a == baseline_a2;
    let t5b_pass = output_b == baseline_b;
    let t5_diff = output_a != output_b;

    println!("OUTPUT_A == BASELINE_A: {} {}", if t5a_pass { "PASS" } else { "FAIL" },
        if !t5a_pass { format!("\n  Output:   {}\n  Baseline: {}", output_a, baseline_a2) } else { String::new() });
    println!("OUTPUT_B == BASELINE_B: {} {}", if t5b_pass { "PASS" } else { "FAIL" },
        if !t5b_pass { format!("\n  Output:   {}\n  Baseline: {}", output_b, baseline_b) } else { String::new() });
    println!("OUTPUT_A != OUTPUT_B:   {} (expected — different tasks)", if t5_diff { "YES" } else { "NO (unexpected!)" });

    if t5a_pass && t5b_pass && t5_diff {
        println!("\nPASS — environment .uvkv can be reused across different tasks.");
        println!("Same cached prefix produces correct output for both tasks,");
        println!("and different tasks produce different output as expected.");
    } else if t5a_pass && t5b_pass {
        println!("\nPASS (tasks match) — but both tasks produced identical output (unexpected).");
    } else {
        println!("\nFAIL — prefix reuse produces divergent output for at least one task.");
    }
    let t5_pass = t5a_pass && t5b_pass;

    // ===================================================================
    // TEST 6: KV cache quantization storage test
    // ===================================================================
    print_section("TEST 6: quantized_kv_serialization_size");

    // Test with default (f16) KV cache — quantized KV requires specific
    // context params. Check if the model/context supports it.
    let long_text = format!("{}{}{}", ENVIRONMENT, TASK_A, TASK_B);
    let long_tokens = tokenize(&vocab, &long_text);
    let token_count = long_tokens.len();
    println!("Test text: {} tokens", token_count);

    // Full precision (f16, the default)
    println!("\n--- Full precision (f16) KV cache ---");
    let mut ctx_full = model.new_context(&ctx_params).expect("ctx failed");
    ctx_full.append_tokens(&long_tokens).expect("prefill failed");
    let kv_full = ctx_full.state_seq_get(0);
    let full_size = kv_full.len();
    println!("Serialized size: {} bytes ({:.2} MB)", full_size, full_size as f64 / 1_048_576.0);
    println!("Per token: {:.0} bytes", full_size as f64 / token_count as f64);

    // Generate baseline for quality comparison
    let full_baseline = generate(&mut ctx_full, &vocab, gen_tokens);
    println!("Full precision output: {}", full_baseline);
    drop(ctx_full);

    // Verify the full-precision blob round-trips correctly
    let mut ctx_rt = model.new_context(&ctx_params).expect("ctx failed");
    ctx_rt.state_seq_set(&kv_full, 0);
    ctx_rt.set_tokens(token_count);
    prime_logits(&mut ctx_rt, *long_tokens.last().unwrap());
    let rt_output = generate(&mut ctx_rt, &vocab, gen_tokens);
    drop(ctx_rt);

    let rt_match = full_baseline == rt_output;
    println!("Round-trip match: {}", if rt_match { "YES" } else { "NO" });
    if !rt_match {
        println!("  Full:       {}", full_baseline);
        println!("  Round-trip: {}", rt_output);
    }

    // Try quantized KV cache (q8_0)
    println!("\n--- Quantized (q8_0) KV cache ---");
    let mut ctx_params_q8 = unsafe { skelm_llama_cpp_sys::llama::llama_context_default_params() };
    ctx_params_q8.n_ctx = 4096;
    ctx_params_q8.type_k = skelm_llama_cpp_sys::llama::ggml_type::GGML_TYPE_Q8_0;
    ctx_params_q8.type_v = skelm_llama_cpp_sys::llama::ggml_type::GGML_TYPE_Q8_0;

    let q8_params = ContextParams {
        n_ctx: 4096,
        ..Default::default()
    };

    // We need to create the context with the quantized params directly
    // since ContextParams doesn't expose type_k/type_v yet.
    // For now, use the raw API.
    let ctx_q8_ptr = unsafe {
        skelm_llama_cpp_sys::llama::llama_new_context_with_model(model.as_raw(), ctx_params_q8)
    };

    if ctx_q8_ptr.is_null() {
        println!("q8_0 KV cache not supported by this model/backend. Skipping.");
    } else {
        // We can't easily wrap this in Context without exposing internals,
        // so we'll use the safe API by adding type_k/type_v support later.
        // For now, work with the raw pointer.
        unsafe {
            // Prefill using a batch
            let mut ctx_q8 = Context::from_raw(model.clone(), ctx_q8_ptr, &q8_params);
            ctx_q8.append_tokens(&long_tokens).expect("q8 prefill failed");

            let kv_q8 = ctx_q8.state_seq_get(0);
            let q8_size = kv_q8.len();
            println!("Serialized size: {} bytes ({:.2} MB)", q8_size, q8_size as f64 / 1_048_576.0);
            println!("Per token: {:.0} bytes", q8_size as f64 / token_count as f64);
            println!("Ratio vs f16: {:.1}%", q8_size as f64 / full_size as f64 * 100.0);
            println!("Savings: {:.1}%", (1.0 - q8_size as f64 / full_size as f64) * 100.0);

            let q8_output = generate(&mut ctx_q8, &vocab, gen_tokens);
            println!("q8_0 output: {}", q8_output);
            let q8_match = full_baseline == q8_output;
            println!("Matches f16 baseline: {}", if q8_match { "YES" } else { "NO (expected — quantization introduces minor differences)" });
            if !q8_match {
                println!("  f16: {}", full_baseline);
                println!("  q8:  {}", q8_output);
            }
        }
    }

    // Try q4_0 too for maximum compression
    println!("\n--- Quantized (q4_0) KV cache ---");
    let mut ctx_params_q4 = unsafe { skelm_llama_cpp_sys::llama::llama_context_default_params() };
    ctx_params_q4.n_ctx = 4096;
    ctx_params_q4.type_k = skelm_llama_cpp_sys::llama::ggml_type::GGML_TYPE_Q4_0;
    ctx_params_q4.type_v = skelm_llama_cpp_sys::llama::ggml_type::GGML_TYPE_Q4_0;

    let ctx_q4_ptr = unsafe {
        skelm_llama_cpp_sys::llama::llama_new_context_with_model(model.as_raw(), ctx_params_q4)
    };

    if ctx_q4_ptr.is_null() {
        println!("q4_0 KV cache not supported by this model/backend. Skipping.");
    } else {
        unsafe {
            let mut ctx_q4 = Context::from_raw(model.clone(), ctx_q4_ptr, &q8_params);
            ctx_q4.append_tokens(&long_tokens).expect("q4 prefill failed");

            let kv_q4 = ctx_q4.state_seq_get(0);
            let q4_size = kv_q4.len();
            println!("Serialized size: {} bytes ({:.2} MB)", q4_size, q4_size as f64 / 1_048_576.0);
            println!("Per token: {:.0} bytes", q4_size as f64 / token_count as f64);
            println!("Ratio vs f16: {:.1}%", q4_size as f64 / full_size as f64 * 100.0);
            println!("Savings: {:.1}%", (1.0 - q4_size as f64 / full_size as f64) * 100.0);

            let q4_output = generate(&mut ctx_q4, &vocab, gen_tokens);
            println!("q4_0 output: {}", q4_output);
            let q4_match = full_baseline == q4_output;
            println!("Matches f16 baseline: {}", if q4_match { "YES" } else { "NO" });
            if !q4_match {
                println!("  f16: {}", full_baseline);
                println!("  q4:  {}", q4_output);
            }
        }
    }

    // ===================================================================
    // SUMMARY
    // ===================================================================
    print_section("SUMMARY");
    println!("Test 4 (prefix cache + incremental prefill): {}",
        if t4_pass { "PASS" } else { "FAIL" });
    println!("Test 5 (prefix reuse across tasks):          {}",
        if t5_pass { "PASS" } else { "FAIL" });
    println!("Test 6 (KV quantization storage):            see sizes above");
    println!();
    if t4_pass && t5_pass {
        println!("CONCLUSION: Prefix-level caching with incremental prefill is VIABLE.");
        println!("Phase 25a can proceed with the prefix caching architecture.");
        println!("The environment .uvkv is serialized once and reused across tasks.");
        println!("Dynamic task context is incrementally prefilled on top.");
    } else {
        println!("CONCLUSION: Prefix-level caching has ISSUES — see failing tests above.");
        if !t4_pass {
            println!("  Test 4 FAIL: incremental prefill after deserialization diverges.");
        }
        if !t5_pass {
            println!("  Test 5 FAIL: prefix reuse produces different output than from-scratch.");
        }
    }
}
