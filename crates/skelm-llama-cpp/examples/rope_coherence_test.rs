//! RoPE Coherence Test for Position-Shifted KV Cache Composition
//!
//! Validates whether per-sequence KV cache slices, serialized out of one context
//! and re-injected into another with a position shift, produce coherent output
//! during downstream generation.
//!
//! The concern: rotary position embedding bakes position information into the K
//! vectors during the forward pass. If `MemoryMut::seq_add` only updates the
//! cache's position metadata without re-applying RoPE to the underlying vectors,
//! shifted K vectors carry the wrong rotary phase and attention degrades.
//!
//! The test is structured as three checks:
//!
//! - TEST 1: Single-slice composition. Prefill `PREFIX` in a target context,
//!   serialize `MIDDLE` from a separate context, inject it with a shift equal
//!   to `PREFIX`'s length, and compare generation against a baseline that
//!   prefilled the concatenation directly.
//!
//! - TEST 2: Three-slice composition. Same idea with three serialized segments,
//!   each shifted by the cumulative offset of its predecessors.
//!
//! - TEST 3: Control. Serialize and reload a sequence with no shift to
//!   distinguish RoPE-shift failures from serialization-machinery failures.
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

/// Re-decode the last token to prime logits after a fresh `state_seq_set`.
///
/// `state_seq_set` populates the KV cache for all positions but does not
/// compute logits for the last position, so generation cannot start. Removing
/// the last position via `seq_rm`, rewinding the token counter, and
/// re-decoding the same token leaves all earlier KV entries intact and
/// produces fresh logits at the boundary.
fn prime_logits(ctx: &mut Context, last_token: Token) {
    let last_pos = (ctx.tokens() - 1) as i32;
    {
        let mut mem = ctx.memory_mut();
        mem.seq_rm(0, last_pos, last_pos + 1);
    }
    ctx.set_tokens(last_pos as usize);
    ctx.append_tokens(&[last_token])
        .expect("prime decode failed");
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

fn report_divergence(label: &str, baseline: &str, composed: &str) {
    println!("DIVERGENCE DETECTED ({})", label);
    println!("  Baseline: {}", baseline);
    println!("  Composed: {}", composed);

    let b_words: Vec<&str> = baseline.split_whitespace().collect();
    let c_words: Vec<&str> = composed.split_whitespace().collect();
    let matching = b_words
        .iter()
        .zip(c_words.iter())
        .filter(|(a, b)| a == b)
        .count();
    let total = b_words.len().max(c_words.len()).max(1);
    let match_pct = matching as f64 / total as f64 * 100.0;
    println!("  Word overlap: {:.0}% ({}/{})", match_pct, matching, total);
    if match_pct > 50.0 {
        println!("  PARTIAL — divergent but may be acceptable");
    } else {
        println!("  FAIL — output has low overlap, likely incoherent");
    }
}

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
        n_ctx: 2048,
        ..Default::default()
    };

    let gen_tokens = 30;

    // ===================================================================
    // TEST 1: Single-slice position-shifted KV injection
    // ===================================================================
    print_section("TEST 1: Single-slice position-shifted KV injection");

    let prefix = "The capital of France is";
    let middle = " Paris. The Eiffel Tower is located in";
    let full_prompt = format!("{}{}", prefix, middle);

    let prefix_tokens = tokenize(&vocab, prefix);
    let middle_tokens = tokenize(&vocab, middle);
    let full_tokens = tokenize(&vocab, &full_prompt);

    println!("PREFIX tokens: {} -> {}", prefix, prefix_tokens.len());
    println!("MIDDLE tokens: {} -> {}", middle, middle_tokens.len());
    println!(
        "FULL   tokens: {} (should == prefix + middle = {})",
        full_tokens.len(),
        prefix_tokens.len() + middle_tokens.len()
    );

    println!("\n--- Step 1: Baseline (normal prefill) ---");
    let mut ctx_baseline = model
        .new_context(&ctx_params)
        .expect("context creation failed");
    ctx_baseline
        .append_tokens(&full_tokens)
        .expect("prefill failed");
    let baseline_output = generate(&mut ctx_baseline, &vocab, gen_tokens);
    println!("Baseline output: {}|{}", full_prompt, baseline_output);
    drop(ctx_baseline);

    println!("\n--- Step 2: Composition (serialize + shift + inject) ---");

    // 2a: serialize MIDDLE from a temporary context
    let mut ctx_tmp = model
        .new_context(&ctx_params)
        .expect("context creation failed");
    ctx_tmp
        .append_tokens(&middle_tokens)
        .expect("prefill MIDDLE failed");
    let middle_kv = ctx_tmp.state_seq_get(0);
    {
        let mem = ctx_tmp.memory();
        println!(
            "Serialized MIDDLE KV state: {} bytes; pos range [{}, {}]",
            middle_kv.len(),
            mem.seq_pos_min(0),
            mem.seq_pos_max(0)
        );
    }
    drop(ctx_tmp);

    // 2b: target context with PREFIX prefilled
    let mut ctx_composed = model
        .new_context(&ctx_params)
        .expect("context creation failed");
    ctx_composed
        .append_tokens(&prefix_tokens)
        .expect("prefill PREFIX failed");
    let prefix_len = prefix_tokens.len() as i32;
    {
        let mem = ctx_composed.memory();
        println!(
            "PREFIX prefilled, pos range [{}, {}]; can_shift = {}",
            mem.seq_pos_min(0),
            mem.seq_pos_max(0),
            mem.can_shift()
        );
    }

    // 2c: inject MIDDLE into seq 1, shift, copy into seq 0, drop seq 1
    let _ = ctx_composed.state_seq_set(&middle_kv, 1);
    {
        let mem = ctx_composed.memory();
        println!(
            "After seq_set into seq 1: seq 0 [{}, {}]; seq 1 [{}, {}]",
            mem.seq_pos_min(0),
            mem.seq_pos_max(0),
            mem.seq_pos_min(1),
            mem.seq_pos_max(1)
        );
    }

    {
        let mut mem = ctx_composed.memory_mut();
        mem.seq_add(1, 0, -1, prefix_len);
    }
    {
        let mem = ctx_composed.memory();
        println!(
            "After seq_add(seq=1, delta={}): seq 1 [{}, {}]",
            prefix_len,
            mem.seq_pos_min(1),
            mem.seq_pos_max(1)
        );
    }

    let shifted_start = prefix_len;
    let shifted_end = prefix_len + middle_tokens.len() as i32;
    {
        let mut mem = ctx_composed.memory_mut();
        mem.seq_cp(1, 0, shifted_start, shifted_end);
        mem.seq_rm(1, -1, -1);
    }
    {
        let mem = ctx_composed.memory();
        println!(
            "After seq_cp + seq_rm cleanup: seq 0 [{}, {}]",
            mem.seq_pos_min(0),
            mem.seq_pos_max(0)
        );
    }

    ctx_composed.set_tokens(full_tokens.len());
    prime_logits(&mut ctx_composed, *full_tokens.last().unwrap());

    let composed_output = generate(&mut ctx_composed, &vocab, gen_tokens);
    println!("Composed output: {}|{}", full_prompt, composed_output);
    drop(ctx_composed);

    print_section("TEST 1 RESULT");
    if baseline_output == composed_output {
        println!("PASS — identical output");
    } else {
        report_divergence("TEST 1", &baseline_output, &composed_output);
        println!();
        println!("Manual review: if the composed output is coherent English, this is a");
        println!("PARTIAL PASS (RoPE shift produced different but legible generation).");
        println!("If garbled or repetitive, this is a FAIL.");
    }

    // ===================================================================
    // TEST 2: Multi-slice composition (three segments)
    // ===================================================================
    print_section("TEST 2: Multi-slice composition (three segments)");

    let prompt_a = "Rust is a systems programming language.";
    let prompt_b = " It emphasizes safety and performance.";
    let prompt_c = " The borrow checker ensures memory safety. Therefore,";
    let full_multi = format!("{}{}{}", prompt_a, prompt_b, prompt_c);

    let tokens_a = tokenize(&vocab, prompt_a);
    let tokens_b = tokenize(&vocab, prompt_b);
    let tokens_c = tokenize(&vocab, prompt_c);
    let tokens_full = tokenize(&vocab, &full_multi);

    println!("PROMPT_A tokens: {}", tokens_a.len());
    println!("PROMPT_B tokens: {}", tokens_b.len());
    println!("PROMPT_C tokens: {}", tokens_c.len());
    println!(
        "FULL     tokens: {} (sum = {})",
        tokens_full.len(),
        tokens_a.len() + tokens_b.len() + tokens_c.len()
    );

    println!("\n--- Step 1: Baseline ---");
    let mut ctx_baseline = model.new_context(&ctx_params).expect("ctx failed");
    ctx_baseline
        .append_tokens(&tokens_full)
        .expect("prefill failed");
    let baseline_multi = generate(&mut ctx_baseline, &vocab, gen_tokens);
    println!("Baseline: {}|{}", full_multi, baseline_multi);
    drop(ctx_baseline);

    println!("\n--- Step 2: Compose three slices ---");

    let mut ctx_a = model.new_context(&ctx_params).expect("ctx failed");
    ctx_a.append_tokens(&tokens_a).expect("prefill A failed");
    let kv_a = ctx_a.state_seq_get(0);
    println!("Serialized A: {} bytes, {} tokens", kv_a.len(), tokens_a.len());
    drop(ctx_a);

    let mut ctx_b = model.new_context(&ctx_params).expect("ctx failed");
    ctx_b.append_tokens(&tokens_b).expect("prefill B failed");
    let kv_b = ctx_b.state_seq_get(0);
    println!("Serialized B: {} bytes, {} tokens", kv_b.len(), tokens_b.len());
    drop(ctx_b);

    let mut ctx_c = model.new_context(&ctx_params).expect("ctx failed");
    ctx_c.append_tokens(&tokens_c).expect("prefill C failed");
    let kv_c = ctx_c.state_seq_get(0);
    println!("Serialized C: {} bytes, {} tokens", kv_c.len(), tokens_c.len());
    drop(ctx_c);

    let mut ctx_target = model.new_context(&ctx_params).expect("ctx failed");
    let n_a = tokens_a.len() as i32;
    let n_b = tokens_b.len() as i32;
    let n_c = tokens_c.len() as i32;

    // Slice A goes in at position 0 with no shift.
    let _ = ctx_target.state_seq_set(&kv_a, 0);
    {
        let mem = ctx_target.memory();
        println!(
            "Loaded A: seq 0 [{}, {}]",
            mem.seq_pos_min(0),
            mem.seq_pos_max(0)
        );
    }

    // Slice B: load into seq 1, shift by n_a, copy into seq 0, drop seq 1.
    let _ = ctx_target.state_seq_set(&kv_b, 1);
    {
        let mut mem = ctx_target.memory_mut();
        mem.seq_add(1, 0, -1, n_a);
        mem.seq_cp(1, 0, n_a, n_a + n_b);
        mem.seq_rm(1, -1, -1);
    }
    {
        let mem = ctx_target.memory();
        println!(
            "Loaded B: seq 0 [{}, {}]",
            mem.seq_pos_min(0),
            mem.seq_pos_max(0)
        );
    }

    // Slice C: load into seq 1, shift by n_a + n_b, copy into seq 0, drop seq 1.
    let _ = ctx_target.state_seq_set(&kv_c, 1);
    {
        let mut mem = ctx_target.memory_mut();
        mem.seq_add(1, 0, -1, n_a + n_b);
        mem.seq_cp(1, 0, n_a + n_b, n_a + n_b + n_c);
        mem.seq_rm(1, -1, -1);
    }
    {
        let mem = ctx_target.memory();
        println!(
            "Loaded C: seq 0 [{}, {}]",
            mem.seq_pos_min(0),
            mem.seq_pos_max(0)
        );
    }
    let _ = n_c; // currently unused beyond construction; kept for symmetry

    ctx_target.set_tokens(tokens_full.len());
    prime_logits(&mut ctx_target, *tokens_full.last().unwrap());

    let composed_multi = generate(&mut ctx_target, &vocab, gen_tokens);
    println!("Composed: {}|{}", full_multi, composed_multi);
    drop(ctx_target);

    print_section("TEST 2 RESULT");
    if baseline_multi == composed_multi {
        println!("PASS — identical output");
    } else {
        report_divergence("TEST 2", &baseline_multi, &composed_multi);
    }

    // ===================================================================
    // TEST 3: Control — serialize/deserialize with no shift
    // ===================================================================
    print_section("TEST 3: Control (serialize/deserialize, no shift)");

    let control_prompt = "The capital of France is Paris. The Eiffel Tower is located in";
    let control_tokens = tokenize(&vocab, control_prompt);
    println!("Control prompt tokens: {}", control_tokens.len());

    let mut ctx_ctrl_base = model.new_context(&ctx_params).expect("ctx failed");
    ctx_ctrl_base
        .append_tokens(&control_tokens)
        .expect("prefill failed");
    let ctrl_baseline = generate(&mut ctx_ctrl_base, &vocab, gen_tokens);
    println!("Control baseline: {}|{}", control_prompt, ctrl_baseline);
    drop(ctx_ctrl_base);

    let mut ctx_src = model.new_context(&ctx_params).expect("ctx failed");
    ctx_src
        .append_tokens(&control_tokens)
        .expect("prefill failed");
    let kv_ctrl = ctx_src.state_seq_get(0);
    drop(ctx_src);

    let mut ctx_dst = model.new_context(&ctx_params).expect("ctx failed");
    let _ = ctx_dst.state_seq_set(&kv_ctrl, 0);
    ctx_dst.set_tokens(control_tokens.len());
    prime_logits(&mut ctx_dst, *control_tokens.last().unwrap());
    let ctrl_composed = generate(&mut ctx_dst, &vocab, gen_tokens);
    println!("Control composed: {}|{}", control_prompt, ctrl_composed);
    drop(ctx_dst);

    print_section("TEST 3 RESULT");
    if ctrl_baseline == ctrl_composed {
        println!("PASS — serialize/deserialize without shifting produces identical output.");
        println!("Any divergence in TEST 1 or TEST 2 is therefore caused by position");
        println!("shifting itself, not by the serialization machinery.");
    } else {
        println!("UNEXPECTED: even without shifting, output differs.");
        println!("  Baseline: {}", ctrl_baseline);
        println!("  Composed: {}", ctrl_composed);
    }

    // ===================================================================
    // SUMMARY
    // ===================================================================
    print_section("SUMMARY");
    let t1_pass = baseline_output == composed_output;
    let t2_pass = baseline_multi == composed_multi;
    let t3_pass = ctrl_baseline == ctrl_composed;
    println!(
        "TEST 1 (single-slice): {}",
        if t1_pass { "PASS" } else { "DIVERGENT" }
    );
    println!(
        "TEST 2 (multi-slice):  {}",
        if t2_pass { "PASS" } else { "DIVERGENT" }
    );
    println!(
        "TEST 3 (control):      {}",
        if t3_pass { "PASS" } else { "FAIL" }
    );
    println!();
    if t1_pass && t2_pass && t3_pass {
        println!("Position-shifted KV cache composition is coherent on this model.");
    } else if !t3_pass {
        println!("Control failed: serialization round-trip itself is broken on this model.");
    } else {
        println!("Position-shifted composition diverges; review the outputs above to");
        println!("decide whether the divergence is acceptable for your use case.");
    }
}
