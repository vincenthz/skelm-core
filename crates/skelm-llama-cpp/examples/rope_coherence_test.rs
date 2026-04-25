/// RoPE Coherence Test
///
/// Tests whether position-shifted KV cache injection produces coherent output.
/// This determines if munu-code's function-level .uvkv composition is viable.
///
/// The concern: RoPE bakes position information into K vectors during the forward
/// pass. If memory_seq_add only changes metadata without re-applying RoPE, then
/// shifted K vectors would have wrong rotary embeddings, producing incoherent
/// attention in downstream generation.
///
/// Usage: cargo run --example rope_coherence_test -- <model_path>

use skelm_llama_cpp::{
    Context, ContextParams, Model, ModelParams, SamplerGreedy, Token, Vocab,
};

fn tokenize(vocab: &Vocab, text: &str) -> Vec<Token> {
    vocab.tokenize(text.as_bytes(), false)
}

/// Re-decode the last token to prime the logits after KV cache composition.
/// The KV cache has entries for all positions but no logits are computed.
/// We remove the last position's KV entry, then re-decode that token.
/// This produces logits while keeping all prior KV entries intact.
fn prime_logits(ctx: &mut Context, last_token: Token) {
    let last_pos = (ctx.tokens - 1) as i32;
    // Remove only the last position from seq 0
    ctx.memory_seq_rm(0, last_pos, last_pos + 1);
    ctx.set_tokens(last_pos as usize);
    // Re-decode: this uses the existing KV cache for positions 0..last_pos-1
    // and computes fresh KV + logits for last_pos
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

    println!("PREFIX tokens: {} -> {:?}", prefix, prefix_tokens.len());
    println!("MIDDLE tokens: {} -> {:?}", middle, middle_tokens.len());
    println!("FULL   tokens: {} (should == prefix + middle = {})",
        full_tokens.len(), prefix_tokens.len() + middle_tokens.len());

    // Step 1: Baseline — normal prefill of full prompt
    println!("\n--- Step 1: Baseline (normal prefill) ---");
    let mut ctx_baseline = model.new_context(&ctx_params).expect("context creation failed");
    ctx_baseline.append_tokens(&full_tokens).expect("prefill failed");
    let baseline_output = generate(&mut ctx_baseline, &vocab, gen_tokens);
    println!("Baseline output: {}{}{}", full_prompt, "|", baseline_output);
    drop(ctx_baseline);

    // Step 2: Composition — serialize MIDDLE from separate context, inject with shift
    println!("\n--- Step 2: Composition (serialize + shift + inject) ---");

    // 2a: prefill MIDDLE in a temporary context
    let mut ctx_tmp = model.new_context(&ctx_params).expect("context creation failed");
    ctx_tmp.append_tokens(&middle_tokens).expect("prefill MIDDLE failed");
    let middle_kv = ctx_tmp.state_seq_get(0);
    println!("Serialized MIDDLE KV state: {} bytes", middle_kv.len());
    println!("MIDDLE seq pos range: [{}, {}]",
        ctx_tmp.memory_seq_pos_min(0), ctx_tmp.memory_seq_pos_max(0));
    drop(ctx_tmp);

    // 2b: create target context with PREFIX prefilled
    let mut ctx_composed = model.new_context(&ctx_params).expect("context creation failed");
    ctx_composed.append_tokens(&prefix_tokens).expect("prefill PREFIX failed");
    let prefix_len = prefix_tokens.len() as i32;
    println!("PREFIX prefilled, pos range: [{}, {}]",
        ctx_composed.memory_seq_pos_min(0), ctx_composed.memory_seq_pos_max(0));
    println!("Can shift: {}", ctx_composed.memory_can_shift());

    // 2c: inject MIDDLE KV state with position shift
    //   1. Load into temp seq 1
    ctx_composed.state_seq_set(&middle_kv, 1);
    println!("After seq_set into seq 1:");
    println!("  seq 0 pos range: [{}, {}]",
        ctx_composed.memory_seq_pos_min(0), ctx_composed.memory_seq_pos_max(0));
    println!("  seq 1 pos range: [{}, {}]",
        ctx_composed.memory_seq_pos_min(1), ctx_composed.memory_seq_pos_max(1));

    //   2. Shift seq 1 positions by prefix_len
    ctx_composed.memory_seq_add(1, 0, -1, prefix_len);
    println!("After seq_add(seq=1, delta={}):", prefix_len);
    println!("  seq 1 pos range: [{}, {}]",
        ctx_composed.memory_seq_pos_min(1), ctx_composed.memory_seq_pos_max(1));

    //   3. Copy seq 1 into seq 0
    let shifted_start = prefix_len;
    let shifted_end = prefix_len + middle_tokens.len() as i32;
    ctx_composed.memory_seq_cp(1, 0, shifted_start, shifted_end);
    println!("After seq_cp(src=1, dst=0, p0={}, p1={}):", shifted_start, shifted_end);
    println!("  seq 0 pos range: [{}, {}]",
        ctx_composed.memory_seq_pos_min(0), ctx_composed.memory_seq_pos_max(0));

    //   4. Remove temp seq 1
    ctx_composed.memory_seq_rm(1, -1, -1);
    println!("After seq_rm(seq=1): cleaned up temp sequence");
    println!("  seq 0 pos range: [{}, {}]",
        ctx_composed.memory_seq_pos_min(0), ctx_composed.memory_seq_pos_max(0));

    // Update the Rust-side token counter to match the composed KV state
    ctx_composed.set_tokens(full_tokens.len());

    // Prime logits by re-decoding the last token (KV cache has no logits after state_seq_set)
    prime_logits(&mut ctx_composed, *full_tokens.last().unwrap());

    // Generate from composed state
    let composed_output = generate(&mut ctx_composed, &vocab, gen_tokens);
    println!("Composed output: {}{}{}", full_prompt, "|", composed_output);
    drop(ctx_composed);

    // Step 3: Compare
    print_section("TEST 1 RESULT");
    if baseline_output == composed_output {
        println!("PASS — identical output");
        println!("Position-shifted KV injection produces exact same generation.");
    } else {
        // Check if it's at least coherent English
        println!("DIVERGENCE DETECTED");
        println!("  Baseline: {}", baseline_output);
        println!("  Composed: {}", composed_output);
        println!();
        // Simple heuristic: if more than half of tokens match, it's partial
        let b_words: Vec<&str> = baseline_output.split_whitespace().collect();
        let c_words: Vec<&str> = composed_output.split_whitespace().collect();
        let matching = b_words.iter().zip(c_words.iter()).filter(|(a, b)| a == b).count();
        let total = b_words.len().max(c_words.len()).max(1);
        let match_pct = matching as f64 / total as f64 * 100.0;
        println!("  Word overlap: {:.0}% ({}/{})", match_pct, matching, total);
        if match_pct > 50.0 {
            println!("  PARTIAL PASS — divergent but may be acceptable");
        } else {
            println!("  FAIL — output has low overlap, likely incoherent");
        }
        println!();
        println!("  Manual review required: read the composed output above.");
        println!("  If it is coherent English, this is a PARTIAL PASS.");
        println!("  If it is garbled/repetitive/nonsensical, this is a FAIL.");
    }

    // ===================================================================
    // TEST 2: Multi-slice composition (3 segments)
    // ===================================================================
    print_section("TEST 2: Multi-slice composition (3 segments)");

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
    println!("FULL     tokens: {} (sum = {})",
        tokens_full.len(), tokens_a.len() + tokens_b.len() + tokens_c.len());

    // Step 1: Baseline
    println!("\n--- Step 1: Baseline ---");
    let mut ctx_baseline = model.new_context(&ctx_params).expect("context failed");
    ctx_baseline.append_tokens(&tokens_full).expect("prefill failed");
    let baseline_multi = generate(&mut ctx_baseline, &vocab, gen_tokens);
    println!("Baseline: {}{}{}", full_multi, "|", baseline_multi);
    drop(ctx_baseline);

    // Step 2: Compose three slices
    println!("\n--- Step 2: Compose 3 slices ---");

    // Serialize each segment from its own context
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

    // Compose into target context
    let mut ctx_target = model.new_context(&ctx_params).expect("ctx failed");
    let n_a = tokens_a.len() as i32;
    let n_b = tokens_b.len() as i32;
    let n_c = tokens_c.len() as i32;

    // Load slice A at position 0 (no shift needed)
    ctx_target.state_seq_set(&kv_a, 0);
    println!("Loaded A: seq 0 range [{}, {}]",
        ctx_target.memory_seq_pos_min(0), ctx_target.memory_seq_pos_max(0));

    // Load slice B into seq 1, shift by n_a, copy to seq 0
    ctx_target.state_seq_set(&kv_b, 1);
    ctx_target.memory_seq_add(1, 0, -1, n_a);
    ctx_target.memory_seq_cp(1, 0, n_a, n_a + n_b);
    ctx_target.memory_seq_rm(1, -1, -1);
    println!("Loaded B: seq 0 range [{}, {}]",
        ctx_target.memory_seq_pos_min(0), ctx_target.memory_seq_pos_max(0));

    // Load slice C into seq 1, shift by n_a + n_b, copy to seq 0
    ctx_target.state_seq_set(&kv_c, 1);
    ctx_target.memory_seq_add(1, 0, -1, n_a + n_b);
    ctx_target.memory_seq_cp(1, 0, n_a + n_b, n_a + n_b + n_c);
    ctx_target.memory_seq_rm(1, -1, -1);
    println!("Loaded C: seq 0 range [{}, {}]",
        ctx_target.memory_seq_pos_min(0), ctx_target.memory_seq_pos_max(0));

    ctx_target.set_tokens(tokens_full.len());

    // Prime logits by re-decoding the last token
    prime_logits(&mut ctx_target, *tokens_full.last().unwrap());

    let composed_multi = generate(&mut ctx_target, &vocab, gen_tokens);
    println!("Composed: {}{}{}", full_multi, "|", composed_multi);
    drop(ctx_target);

    // Step 3: Compare
    print_section("TEST 2 RESULT");
    if baseline_multi == composed_multi {
        println!("PASS — identical output");
        println!("Multi-slice KV composition produces exact same generation.");
    } else {
        println!("DIVERGENCE DETECTED");
        println!("  Baseline: {}", baseline_multi);
        println!("  Composed: {}", composed_multi);
        let b_words: Vec<&str> = baseline_multi.split_whitespace().collect();
        let c_words: Vec<&str> = composed_multi.split_whitespace().collect();
        let matching = b_words.iter().zip(c_words.iter()).filter(|(a, b)| a == b).count();
        let total = b_words.len().max(c_words.len()).max(1);
        let match_pct = matching as f64 / total as f64 * 100.0;
        println!("  Word overlap: {:.0}% ({}/{})", match_pct, matching, total);
        if match_pct > 50.0 {
            println!("  PARTIAL PASS — divergent but may be acceptable");
        } else {
            println!("  FAIL — output has low overlap, likely incoherent");
        }
        println!();
        println!("  Manual review required.");
    }

    // ===================================================================
    // TEST 3: Control — serialize/deserialize without shifting
    // ===================================================================
    print_section("TEST 3: Control — serialize/deserialize without shifting");

    let control_prompt = "The capital of France is Paris. The Eiffel Tower is located in";
    let control_tokens = tokenize(&vocab, control_prompt);
    println!("Control prompt tokens: {}", control_tokens.len());

    // Baseline
    let mut ctx_ctrl_base = model.new_context(&ctx_params).expect("ctx failed");
    ctx_ctrl_base.append_tokens(&control_tokens).expect("prefill failed");
    let ctrl_baseline = generate(&mut ctx_ctrl_base, &vocab, gen_tokens);
    println!("Control baseline: {}|{}", control_prompt, ctrl_baseline);
    drop(ctx_ctrl_base);

    // Serialize from one context, load into another (no shift)
    let mut ctx_src = model.new_context(&ctx_params).expect("ctx failed");
    ctx_src.append_tokens(&control_tokens).expect("prefill failed");
    let kv_ctrl = ctx_src.state_seq_get(0);
    drop(ctx_src);

    let mut ctx_dst = model.new_context(&ctx_params).expect("ctx failed");
    ctx_dst.state_seq_set(&kv_ctrl, 0);
    ctx_dst.set_tokens(control_tokens.len());
    prime_logits(&mut ctx_dst, *control_tokens.last().unwrap());
    let ctrl_composed = generate(&mut ctx_dst, &vocab, gen_tokens);
    println!("Control composed: {}|{}", control_prompt, ctrl_composed);
    drop(ctx_dst);

    print_section("TEST 3 RESULT");
    if ctrl_baseline == ctrl_composed {
        println!("PASS — serialize/deserialize without shifting produces identical output.");
        println!("This confirms the divergence in tests 1-2 is caused by position shifting,");
        println!("not by the serialization/deserialization machinery itself.");
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
    println!("Test 1 (single-slice): {}", if t1_pass { "PASS" } else { "DIVERGENT — see above" });
    println!("Test 2 (multi-slice):  {}", if t2_pass { "PASS" } else { "DIVERGENT — see above" });
    println!();
    if t1_pass && t2_pass {
        println!("CONCLUSION: Function-level .uvkv composition is VIABLE.");
        println!("RoPE is handled correctly by llama.cpp memory_seq_add.");
        println!("munu-code can proceed with per-function KV slice injection.");
    } else if !t1_pass && !t2_pass {
        println!("CONCLUSION: Composition produces DIVERGENT output.");
        println!("If outputs above are incoherent -> FAIL: fall back to prefix-level caching.");
        println!("If outputs are coherent but different -> PARTIAL: may still work for code gen.");
    } else {
        println!("CONCLUSION: Mixed results — review individual tests above.");
    }
}
