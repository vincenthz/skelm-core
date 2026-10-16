//! End-to-end tests against real GGUF models, ignored by default. Run them with
//! the model paths in the environment, e.g.:
//!
//! ```sh
//! SKELM_TEST_TARGET=qwen2.5-7b.gguf SKELM_TEST_DRAFT=qwen2.5-0.5b.gguf \
//! SKELM_TEST_MTP=Qwen3.5-2B-MTP.gguf SKELM_TEST_LORA=lora-for-target.gguf \
//!     cargo test -p skelm-llama-cpp --release --test models -- --ignored --nocapture
//! ```
//!
//! With a greedy sampler, speculative decoding must produce the same tokens as
//! plain decoding: the drafts only change how many tokens one target decode
//! yields, never which tokens are produced.

use std::time::Instant;

use skelm_llama_cpp::{
    Context, ContextParams, DraftParams, Drafter, LoraAdapter, Model, ModelParams, SamplerGreedy,
    SpeculativeConfig, Token, Vocab,
};

const PROMPT: &str = "Here is a short Python function that returns the n-th Fibonacci number:\n\n```python\ndef fib(n):";
const N_TOKENS: usize = 128;

fn env_path(var: &str) -> String {
    std::env::var(var).unwrap_or_else(|_| panic!("set {var} to a GGUF path to run this test"))
}

fn load(path: &str, load_mtp: bool) -> Model {
    let params = ModelParams {
        load_mtp,
        ..ModelParams::default()
    };
    Model::load(path, &params).expect("model loads")
}

fn params() -> ContextParams {
    ContextParams {
        n_ctx: 4096,
        ..ContextParams::default()
    }
}

/// `SKELM_TEST_DRAFT_MAX` / `SKELM_TEST_DRAFT_P_MIN` override the defaults
fn draft_params() -> DraftParams {
    let var = |v: &str| std::env::var(v).ok();
    let d = DraftParams::default();
    DraftParams {
        n_max: var("SKELM_TEST_DRAFT_MAX").map_or(d.n_max, |v| v.parse().unwrap()),
        p_min: var("SKELM_TEST_DRAFT_P_MIN").map_or(d.p_min, |v| v.parse().unwrap()),
        ..d
    }
}

/// Greedily generate up to `N_TOKENS` tokens after `PROMPT`
fn generate(ctx: &mut Context, vocab: &Vocab) -> (Vec<Token>, f64) {
    ctx.append_tokens(&vocab.tokenize(PROMPT.as_bytes(), true))
        .expect("prompt decodes");
    let mut sampler = SamplerGreedy;
    let mut out = Vec::new();
    let start = Instant::now();
    while out.len() < N_TOKENS {
        let step = ctx.step(&mut sampler, vocab).expect("step decodes");
        out.extend(step.tokens);
        if step.eog {
            break;
        }
    }
    let tps = out.len() as f64 / start.elapsed().as_secs_f64();
    out.truncate(N_TOKENS);
    (out, tps)
}

fn text(vocab: &Vocab, tokens: &[Token]) -> String {
    let bytes: Vec<u8> = tokens.iter().flat_map(|t| vocab.as_bytes(*t)).collect();
    String::from_utf8_lossy(&bytes).into_owned()
}

/// Compare a speculative run with a plain one
fn check_speculative(target: &Model, config: SpeculativeConfig) {
    let vocab = target.vocab();

    // the first run of each warms up the backend (Metal compiles its kernels
    // on first use of each batch shape), the second is timed
    let mut runs = (0..2).map(|_| {
        let mut plain = target.new_context(&params()).expect("context");
        generate(&mut plain, &vocab)
    });
    let (expected, plain_tps) = runs.nth(1).unwrap();

    let mut runs = (0..2).map(|_| {
        let mut spec = Context::new_speculative(target.clone(), &params(), &config)
            .expect("speculative context");
        let res = generate(&mut spec, &vocab);
        (res, spec.speculative_stats().unwrap())
    });
    let ((got, spec_tps), stats) = runs.nth(1).unwrap();

    println!(
        "--- plain ({plain_tps:.1} t/s):\n{}",
        text(&vocab, &expected)
    );
    println!(
        "--- speculative ({spec_tps:.1} t/s): {stats:?}, acceptance {:.1}%",
        stats.acceptance_rate() * 100.0
    );
    assert!(stats.rounds > 0, "no speculative round ran");
    assert_eq!(
        text(&vocab, &got),
        text(&vocab, &expected),
        "speculative output differs from plain greedy decoding"
    );
}

#[test]
#[ignore = "needs SKELM_TEST_TARGET and SKELM_TEST_DRAFT"]
fn draft_model_matches_plain() {
    let target = load(&env_path("SKELM_TEST_TARGET"), false);
    let draft = load(&env_path("SKELM_TEST_DRAFT"), false);
    check_speculative(
        &target,
        SpeculativeConfig {
            drafter: Drafter::Model {
                model: draft,
                params: params(),
            },
            draft: draft_params(),
        },
    );
}

#[test]
#[ignore = "needs SKELM_TEST_MTP"]
fn mtp_matches_plain() {
    let target = load(&env_path("SKELM_TEST_MTP"), true);
    assert!(target.has_mtp(), "the test model has no MTP layers");
    check_speculative(
        &target,
        SpeculativeConfig {
            drafter: Drafter::Mtp,
            draft: draft_params(),
        },
    );
}

#[test]
#[ignore = "needs SKELM_TEST_MTP"]
fn mtp_needs_load_mtp() {
    let target = load(&env_path("SKELM_TEST_MTP"), false);
    assert!(target.n_layer_nextn() > 0);
    assert!(!target.has_mtp());
    let config = SpeculativeConfig {
        drafter: Drafter::Mtp,
        draft: DraftParams::default(),
    };
    assert!(Context::new_speculative(target, &params(), &config).is_err());
}

#[test]
#[ignore = "needs SKELM_TEST_TARGET and SKELM_TEST_LORA"]
fn lora_changes_output() {
    let target = load(&env_path("SKELM_TEST_TARGET"), false);
    let vocab = target.vocab();
    let adapter = LoraAdapter::load(&target, env_path("SKELM_TEST_LORA")).expect("adapter loads");
    println!("adapter metadata: {:?}", adapter.meta());
    assert_eq!(
        adapter.meta_val_str("adapter.type").as_deref(),
        Some("lora")
    );

    let mut plain = target.new_context(&params()).unwrap();
    let (base, _) = generate(&mut plain, &vocab);
    drop(plain);

    let mut ctx = target.new_context(&params()).unwrap();
    ctx.set_lora_adapters(&[(adapter.clone(), 1.0)]).unwrap();
    let (with_lora, _) = generate(&mut ctx, &vocab);
    drop(ctx);

    let mut ctx = target.new_context(&params()).unwrap();
    ctx.set_lora_adapters(&[(adapter.clone(), 0.0)]).unwrap();
    let (scale_zero, _) = generate(&mut ctx, &vocab);

    println!("--- base:\n{}", text(&vocab, &base));
    println!("--- with LoRA:\n{}", text(&vocab, &with_lora));
    assert_ne!(base, with_lora, "the adapter did not change the output");
    assert_eq!(
        text(&vocab, &base),
        text(&vocab, &scale_zero),
        "a 0.0 scale must not change the output"
    );
}

#[test]
#[ignore = "needs SKELM_TEST_TARGET and SKELM_TEST_MTP"]
fn lora_rejects_other_model() {
    let target = load(&env_path("SKELM_TEST_TARGET"), false);
    let other = load(&env_path("SKELM_TEST_MTP"), false);
    let adapter = LoraAdapter::load(&target, env_path("SKELM_TEST_LORA")).expect("adapter loads");
    let mut ctx = other.new_context(&params()).unwrap();
    assert!(ctx.set_lora_adapters(&[(adapter, 1.0)]).is_err());
}
