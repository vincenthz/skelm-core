//! KV Cache Round-Trip and Position-Shift Coherence Test
//!
//! Exercises three things against a real model:
//!
//! - **Round-trip (control):** serialize a sequence's KV cache out of one
//!   context and load it into another with **no position shift**, generate, and
//!   compare against a baseline that prefilled the equivalent prompt directly.
//!   This MUST produce byte-identical output. It is the only operation in stock
//!   llama.cpp that can.
//!
//! - **Single-slice shift:** prefill `PREFIX` in a target context, serialize
//!   `MIDDLE` from a separate context, inject `MIDDLE`'s KV into seq 1, shift
//!   it by `len(PREFIX)`, copy into seq 0, generate. Compare against a baseline
//!   that prefilled `PREFIX + MIDDLE` directly.
//!
//! - **Multi-slice shift:** same idea with three serialized segments.
//!
//! Important: `llama_memory_seq_add` only updates position metadata. RoPE has
//! already been baked into the K vectors at decode time and is **not** re-applied
//! by the shift. The shift tests therefore cannot produce byte-identical output
//! to a contiguous-prefill baseline. They are reported as measurements (word
//! overlap %) rather than as PASS/FAIL.
//!
//! The overall test PASSES iff the round-trip (control) test passes byte-identical.
//! That is the real, supported capability of this code path. Output of the shift
//! tests is informational: it lets a downstream caller decide for their use case
//! whether the divergence is acceptable.
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
/// compute logits at the boundary. Removing the last position via `seq_rm`,
/// rewinding the token counter, and re-decoding the same token leaves the
/// earlier KV entries intact and produces fresh logits.
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

#[derive(Debug)]
enum ShiftOutcome {
    Skipped(&'static str),
    /// Identical bytes to baseline. Not expected to occur with stock llama.cpp seq_add.
    Identical,
    /// Divergent from baseline. Carries word-overlap percentage as a coarse coherence signal.
    Divergent { word_overlap_pct: f64 },
}

fn word_overlap_pct(a: &str, b: &str) -> f64 {
    let aw: Vec<&str> = a.split_whitespace().collect();
    let bw: Vec<&str> = b.split_whitespace().collect();
    let matching = aw.iter().zip(bw.iter()).filter(|(x, y)| x == y).count();
    let total = aw.len().max(bw.len()).max(1);
    matching as f64 / total as f64 * 100.0
}

fn report_shift_outcome(label: &str, baseline: &str, composed: &str, out: &ShiftOutcome) {
    print_section(&format!("{} RESULT", label));
    match out {
        ShiftOutcome::Skipped(reason) => {
            println!("SKIPPED — {}", reason);
        }
        ShiftOutcome::Identical => {
            println!("IDENTICAL — composed output matches baseline byte-for-byte.");
            println!("(Unexpected on stock llama.cpp; may indicate the model is unusually");
            println!(" robust to RoPE phase mismatch, or that seq_add path was a no-op.)");
        }
        ShiftOutcome::Divergent { word_overlap_pct } => {
            println!("DIVERGENT — measurement only, not a PASS/FAIL.");
            println!("  Baseline: {}", baseline);
            println!("  Composed: {}", composed);
            println!("  Word overlap: {:.0}%", word_overlap_pct);
            println!();
            println!("This is expected: llama_memory_seq_add updates position metadata only;");
            println!("RoPE is baked into K vectors at decode time and is not re-applied here.");
            println!("Whether the divergence is acceptable depends on the downstream use case.");
        }
    }
}

fn classify_shift(baseline: &str, composed: &str) -> ShiftOutcome {
    if baseline == composed {
        ShiftOutcome::Identical
    } else {
        ShiftOutcome::Divergent {
            word_overlap_pct: word_overlap_pct(baseline, composed),
        }
    }
}

/// TEST 3 — round-trip without position shift.
///
/// This is the meaningful PASS/FAIL gate. Returns true iff composed output is
/// byte-identical to baseline. Prints its own section banner.
fn test_round_trip(model: &Model, vocab: &Vocab, ctx_params: &ContextParams, gen_tokens: usize) -> bool {
    print_section("TEST 3 — Round-trip (control, no position shift)");

    let prompt = "The capital of France is Paris. The Eiffel Tower is located in";
    let tokens = tokenize(vocab, prompt);
    println!("Prompt tokens: {}", tokens.len());

    let mut ctx_base = model.new_context(ctx_params).expect("ctx failed");
    ctx_base.append_tokens(&tokens).expect("prefill failed");
    let baseline = generate(&mut ctx_base, vocab, gen_tokens);
    println!("Baseline: {}|{}", prompt, baseline);
    drop(ctx_base);

    let mut ctx_src = model.new_context(ctx_params).expect("ctx failed");
    ctx_src.append_tokens(&tokens).expect("prefill failed");
    let kv = ctx_src.state_seq_get(0);
    drop(ctx_src);

    let mut ctx_dst = model.new_context(ctx_params).expect("ctx failed");
    let bytes = ctx_dst.state_seq_set(&kv, 0);
    assert!(bytes > 0, "state_seq_set returned 0 bytes loaded — round-trip is broken");

    ctx_dst.set_tokens(tokens.len());
    prime_logits(&mut ctx_dst, *tokens.last().unwrap());
    let composed = generate(&mut ctx_dst, vocab, gen_tokens);
    println!("Composed: {}|{}", prompt, composed);
    drop(ctx_dst);

    let pass = baseline == composed;
    print_section("TEST 3 VERDICT");
    if pass {
        println!("PASS — round-trip produces identical output.");
    } else {
        println!("FAIL — round-trip diverges. The state_seq_get / state_seq_set");
        println!("       pair is not preserving KV bytes for this model.");
        println!("  Baseline: {}", baseline);
        println!("  Composed: {}", composed);
    }
    pass
}

/// TEST 1 — single-slice position-shifted KV injection. Returns measurement only.
fn test_single_slice_shift(
    model: &Model,
    vocab: &Vocab,
    ctx_params: &ContextParams,
    gen_tokens: usize,
    supports_shift: bool,
    supports_partial_seq_cp: bool,
) -> (String, String, ShiftOutcome) {
    print_section("TEST 1 — Single-slice position-shifted KV injection");

    let prefix = "The capital of France is";
    let middle = " Paris. The Eiffel Tower is located in";
    let full_prompt = format!("{}{}", prefix, middle);

    let prefix_tokens = tokenize(vocab, prefix);
    let middle_tokens = tokenize(vocab, middle);
    let full_tokens = tokenize(vocab, &full_prompt);

    println!("PREFIX tokens: {}; MIDDLE tokens: {}; FULL tokens: {}",
        prefix_tokens.len(), middle_tokens.len(), full_tokens.len());

    // Baseline — direct contiguous prefill.
    let mut ctx_baseline = model.new_context(ctx_params).expect("ctx failed");
    ctx_baseline.append_tokens(&full_tokens).expect("prefill failed");
    let baseline_output = generate(&mut ctx_baseline, vocab, gen_tokens);
    println!("Baseline: {}|{}", full_prompt, baseline_output);
    drop(ctx_baseline);

    if !supports_shift {
        return (baseline_output, String::new(),
            ShiftOutcome::Skipped("memory backend reports can_shift = false (Mamba / hybrid models)"));
    }
    if !supports_partial_seq_cp {
        return (baseline_output, String::new(),
            ShiftOutcome::Skipped("model uses interleaved sliding-window attention; partial cross-stream seq_cp is unsupported in stock llama.cpp (asserts is_full)"));
    }

    // Composition path.
    let mut ctx_tmp = model.new_context(ctx_params).expect("ctx failed");
    ctx_tmp.append_tokens(&middle_tokens).expect("prefill MIDDLE failed");
    let middle_kv = ctx_tmp.state_seq_get(0);
    drop(ctx_tmp);

    let mut ctx_composed = model.new_context(ctx_params).expect("ctx failed");
    ctx_composed.append_tokens(&prefix_tokens).expect("prefill PREFIX failed");
    let prefix_len = prefix_tokens.len() as i32;

    let bytes = ctx_composed.state_seq_set(&middle_kv, 1);
    assert!(bytes > 0,
        "state_seq_set into seq 1 failed (returned 0). \
         Most likely cause: ContextParams.n_seq_max < 2.");

    {
        let mut mem = ctx_composed.memory_mut();
        mem.seq_add(1, 0, -1, prefix_len);
        let shifted_start = prefix_len;
        let shifted_end = prefix_len + middle_tokens.len() as i32;
        mem.seq_cp(1, 0, shifted_start, shifted_end);
        mem.seq_rm(1, -1, -1);
    }

    ctx_composed.set_tokens(full_tokens.len());
    prime_logits(&mut ctx_composed, *full_tokens.last().unwrap());

    let composed_output = generate(&mut ctx_composed, vocab, gen_tokens);
    println!("Composed: {}|{}", full_prompt, composed_output);
    drop(ctx_composed);

    let outcome = classify_shift(&baseline_output, &composed_output);
    (baseline_output, composed_output, outcome)
}

/// TEST 2 — three-slice position-shifted KV injection. Returns measurement only.
fn test_multi_slice_shift(
    model: &Model,
    vocab: &Vocab,
    ctx_params: &ContextParams,
    gen_tokens: usize,
    supports_shift: bool,
    supports_partial_seq_cp: bool,
) -> (String, String, ShiftOutcome) {
    print_section("TEST 2 — Multi-slice position-shifted KV injection (3 segments)");

    let prompt_a = "Rust is a systems programming language.";
    let prompt_b = " It emphasizes safety and performance.";
    let prompt_c = " The borrow checker ensures memory safety. Therefore,";
    let full_multi = format!("{}{}{}", prompt_a, prompt_b, prompt_c);

    let tokens_a = tokenize(vocab, prompt_a);
    let tokens_b = tokenize(vocab, prompt_b);
    let tokens_c = tokenize(vocab, prompt_c);
    let tokens_full = tokenize(vocab, &full_multi);

    println!("A tokens: {}; B tokens: {}; C tokens: {}; FULL tokens: {}",
        tokens_a.len(), tokens_b.len(), tokens_c.len(), tokens_full.len());

    let mut ctx_baseline = model.new_context(ctx_params).expect("ctx failed");
    ctx_baseline.append_tokens(&tokens_full).expect("prefill failed");
    let baseline_multi = generate(&mut ctx_baseline, vocab, gen_tokens);
    println!("Baseline: {}|{}", full_multi, baseline_multi);
    drop(ctx_baseline);

    if !supports_shift {
        return (baseline_multi, String::new(),
            ShiftOutcome::Skipped("memory backend reports can_shift = false"));
    }
    if !supports_partial_seq_cp {
        return (baseline_multi, String::new(),
            ShiftOutcome::Skipped("model uses interleaved sliding-window attention; partial cross-stream seq_cp is unsupported in stock llama.cpp"));
    }

    // Serialize each segment.
    let serialize = |toks: &[Token]| {
        let mut c = model.new_context(ctx_params).expect("ctx failed");
        c.append_tokens(toks).expect("prefill failed");
        c.state_seq_get(0)
    };
    let kv_a = serialize(&tokens_a);
    let kv_b = serialize(&tokens_b);
    let kv_c = serialize(&tokens_c);

    let mut ctx_target = model.new_context(ctx_params).expect("ctx failed");
    let n_a = tokens_a.len() as i32;
    let n_b = tokens_b.len() as i32;
    let n_c = tokens_c.len() as i32;

    // Slice A goes in at position 0, no shift.
    let bytes = ctx_target.state_seq_set(&kv_a, 0);
    assert!(bytes > 0, "state_seq_set(A) failed");

    // Slice B: load, shift, copy, drop.
    let bytes = ctx_target.state_seq_set(&kv_b, 1);
    assert!(bytes > 0, "state_seq_set(B) failed");
    {
        let mut mem = ctx_target.memory_mut();
        mem.seq_add(1, 0, -1, n_a);
        mem.seq_cp(1, 0, n_a, n_a + n_b);
        mem.seq_rm(1, -1, -1);
    }

    // Slice C: load, shift, copy, drop.
    let bytes = ctx_target.state_seq_set(&kv_c, 1);
    assert!(bytes > 0, "state_seq_set(C) failed");
    {
        let mut mem = ctx_target.memory_mut();
        mem.seq_add(1, 0, -1, n_a + n_b);
        mem.seq_cp(1, 0, n_a + n_b, n_a + n_b + n_c);
        mem.seq_rm(1, -1, -1);
    }
    let _ = n_c;

    ctx_target.set_tokens(tokens_full.len());
    prime_logits(&mut ctx_target, *tokens_full.last().unwrap());
    let composed_multi = generate(&mut ctx_target, vocab, gen_tokens);
    println!("Composed: {}|{}", full_multi, composed_multi);
    drop(ctx_target);

    let outcome = classify_shift(&baseline_multi, &composed_multi);
    (baseline_multi, composed_multi, outcome)
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

    // n_seq_max must be >= 2: the shift tests stage serialized KV slices into
    // seq 1 as scratch before copying them into seq 0. Default is 1; without
    // bumping it state_seq_set silently fails for seq_id = 1.
    let ctx_params = ContextParams {
        n_ctx: 2048,
        n_seq_max: 2,
        ..Default::default()
    };

    let gen_tokens = 30;

    // Probe whether the model's memory backend supports position shifts at all.
    // Mamba / SSM / hybrid backends return false; calling seq_add on them aborts
    // with GGML_ASSERT(n_pos_per_embd() == 1).
    let supports_shift = {
        let probe = model.new_context(&ctx_params).expect("probe context failed");
        let s = probe.memory().can_shift();
        drop(probe);
        s
    };

    // Detect interleaved sliding-window attention (iswa) models. Their KV cache
    // backend (`llama_kv_cache_iswa`) routes cross-stream seq_cp through
    // `llama_kv_cache::seq_cp`, which aborts with `GGML_ASSERT(is_full)` for
    // partial-range copies. Our composition pattern uses a partial copy from a
    // scratch sequence into the target sequence, which is exactly the path that
    // is forbidden, so we must skip TESTS 1 and 2 on these models.
    //
    // Detection: presence of an `<arch>.attention.sliding_window` metadata
    // entry. The architecture name comes from `general.architecture`.
    let arch = model.architecture().unwrap_or_default();
    let supports_partial_seq_cp = if arch.is_empty() {
        true
    } else {
        let key = format!("{}.attention.sliding_window", arch);
        model.meta_str(&key).is_none()
    };

    println!("model architecture: {}", if arch.is_empty() { "<unknown>" } else { &arch });
    println!("memory backend can_shift = {}", supports_shift);
    println!("supports partial cross-stream seq_cp = {}", supports_partial_seq_cp);
    if !supports_shift || !supports_partial_seq_cp {
        println!("  Note: TEST 1 and TEST 2 will be SKIPPED for this model.");
    }

    // TEST 3 first — it's the only test that gates overall PASS.
    let t3_pass = test_round_trip(&model, &vocab, &ctx_params, gen_tokens);

    // TEST 1 and TEST 2 — measurement only, never block PASS.
    let (t1_baseline, t1_composed, t1_outcome) = test_single_slice_shift(
        &model, &vocab, &ctx_params, gen_tokens,
        supports_shift, supports_partial_seq_cp,
    );
    report_shift_outcome("TEST 1", &t1_baseline, &t1_composed, &t1_outcome);

    let (t2_baseline, t2_composed, t2_outcome) = test_multi_slice_shift(
        &model, &vocab, &ctx_params, gen_tokens,
        supports_shift, supports_partial_seq_cp,
    );
    report_shift_outcome("TEST 2", &t2_baseline, &t2_composed, &t2_outcome);

    // ===================================================================
    // SUMMARY
    // ===================================================================
    print_section("SUMMARY");
    println!(
        "TEST 3 (round-trip, no shift): {}",
        if t3_pass { "PASS" } else { "FAIL" }
    );
    println!("TEST 1 (single-slice shift):    {:?}", t1_outcome);
    println!("TEST 2 (multi-slice shift):     {:?}", t2_outcome);
    println!();

    if t3_pass {
        println!("OVERALL: PASS");
        println!("The round-trip path (state_seq_get + state_seq_set without shift) is");
        println!("verified working. The shift tests are reported as measurements; their");
        println!("divergence reflects llama.cpp's documented seq_add semantics, not a bug.");
    } else {
        println!("OVERALL: FAIL");
        println!("The round-trip path produced divergent output. This is a real bug:");
        println!("either in skelm-llama-cpp, or in the underlying llama.cpp build, or");
        println!("in the model file. Investigate before relying on KV serialization.");
        std::process::exit(1);
    }
}
