//! Tests of the batching engine against a real GGUF model, ignored by default.
//! Run them with the model path in the environment, e.g.:
//!
//! ```sh
//! SKELM_TEST_MODEL=qwen2.5-0.5b.gguf \
//!     cargo test -p skelm-exec --release --test engine -- --ignored --nocapture
//! ```
//!
//! Both an attention-only model and a recurrent / hybrid one (e.g. Qwen3.5,
//! whose prompt cache uses checkpoints) are worth running. Generation is
//! greedy, so a job must produce the same tokens however it is batched and
//! whatever part of its prompt came from the cache.

use std::path::PathBuf;
use std::sync::mpsc;

use skelm_exec::{Engine, EngineConfig, Flow, Job, JobEnd, JobEvent, JobSink, Model, ModelDescr};
use skelm_llama_cpp::{SamplerGreedy, Token};

const N_TOKENS: usize = 24;

const SYSTEM: &str = "<|im_start|>system\nYou are a helpful assistant. Answer in a short and \
precise way, with no preamble, and always explain your reasoning in one sentence at the \
end.<|im_end|>\n";

fn model() -> Model {
    let path = std::env::var("SKELM_TEST_MODEL")
        .unwrap_or_else(|_| panic!("set SKELM_TEST_MODEL to a GGUF path to run this test"));
    Model::load(&ModelDescr::Path(PathBuf::from(path))).expect("model loads")
}

fn chat(model: &Model, question: &str) -> Vec<Token> {
    let prompt = format!("{SYSTEM}<|im_start|>user\n{question}<|im_end|>\n<|im_start|>assistant\n");
    model.vocab.tokenize(prompt.as_bytes(), true)
}

/// Greedy generation on a plain context of its own
fn reference(model: &Model, prompt: &[Token]) -> Vec<Token> {
    let mut context = model.new_context();
    context.1.append_tokens(prompt).unwrap();
    let mut out = Vec::new();
    while out.len() < N_TOKENS {
        let step = context.1.step(&mut SamplerGreedy, &model.vocab).unwrap();
        out.extend(step.tokens);
        if step.eog {
            break;
        }
    }
    out.truncate(N_TOKENS);
    out
}

#[derive(Debug, Default)]
struct Outcome {
    cached: usize,
    tokens: Vec<Token>,
    end: Option<JobEnd>,
}

struct Sink(mpsc::Sender<JobEvent>);

impl JobSink for Sink {
    fn event(&mut self, event: JobEvent) -> Flow {
        let _ = self.0.send(event);
        Flow::Continue
    }
}

fn submit(engine: &Engine, prompt: &[Token]) -> mpsc::Receiver<JobEvent> {
    let (tx, rx) = mpsc::channel();
    let job = Job {
        prompt: prompt.to_vec(),
        sampler: Box::new(|| Box::new(SamplerGreedy)),
        max_tokens: Some(N_TOKENS),
        sink: Box::new(Sink(tx)),
    };
    assert!(engine.submit(job).is_ok());
    rx
}

fn wait(rx: mpsc::Receiver<JobEvent>) -> Outcome {
    let mut outcome = Outcome::default();
    for event in rx {
        match event {
            JobEvent::Started { cached_tokens, .. } => outcome.cached = cached_tokens,
            JobEvent::Token { token, .. } => outcome.tokens.push(token),
            JobEvent::Finished(end) => outcome.end = Some(end),
        }
    }
    outcome
}

fn run(engine: &Engine, prompt: &[Token]) -> Outcome {
    wait(submit(engine, prompt))
}

fn engine(model: &Model, slots: usize) -> Engine {
    let config = EngineConfig { slots, n_ctx: 4096 };
    Engine::new(model, &config).expect("engine starts")
}

#[test]
#[ignore]
fn batched_jobs_match_sequential_generation() {
    let model = model();
    let questions = [
        "What is the capital of France?",
        "Write a haiku about the sea.",
        "List three prime numbers.",
        "How many legs does a spider have?",
        "What is 12 times 12?",
        "Name a famous painter.",
    ];
    let prompts: Vec<Vec<Token>> = questions.iter().map(|q| chat(&model, q)).collect();
    let expected: Vec<Vec<Token>> = prompts.iter().map(|p| reference(&model, p)).collect();

    // more jobs than slots: some wait for a slot
    let engine = engine(&model, 4);
    let pending: Vec<_> = prompts.iter().map(|p| submit(&engine, p)).collect();
    for (i, rx) in pending.into_iter().enumerate() {
        let outcome = wait(rx);
        assert!(matches!(outcome.end, Some(JobEnd::MaxTokens | JobEnd::Eog)));
        assert_eq!(outcome.tokens, expected[i], "job {i}: {}", questions[i]);
    }
}

#[test]
#[ignore]
fn prompt_cache_is_reused() {
    let model = model();
    let engine = engine(&model, 2);
    let prompt = chat(&model, "What is the capital of France?");
    let expected = reference(&model, &prompt);

    let first = run(&engine, &prompt);
    assert_eq!(first.cached, 0);
    assert_eq!(first.tokens, expected);

    // the same prompt again: all but its last token come from the cache
    let again = run(&engine, &prompt);
    assert_eq!(again.cached, prompt.len() - 1);
    assert_eq!(again.tokens, expected);

    // the conversation goes on: the prompt and the reply are cached
    let mut longer = prompt.clone();
    longer.extend(&again.tokens);
    let next_turn = model.vocab.tokenize(
        b"<|im_end|>\n<|im_start|>user\nAnd of Italy?<|im_end|>\n",
        false,
    );
    longer.extend(next_turn);
    let continued = run(&engine, &longer);
    // when stopped by max_tokens, the reply's last token was sampled but
    // never decoded
    let undecoded = usize::from(again.end == Some(JobEnd::MaxTokens));
    assert_eq!(
        continued.cached,
        prompt.len() + again.tokens.len() - undecoded
    );
    assert_eq!(continued.tokens, reference(&model, &longer));

    // a different question shares the system prompt: a model that can remove
    // the end of a sequence keeps it, a recurrent one restarts from a
    // checkpoint or from scratch
    let other = chat(&model, "What is the capital of Spain?");
    let outcome = run(&engine, &other);
    eprintln!(
        "different question: {} of {} prompt tokens cached",
        outcome.cached,
        other.len()
    );
    assert_eq!(outcome.tokens, reference(&model, &other));

    // a third question: the prefix shared by the previous two is cached
    // (recurrent models checkpointed where the previous one diverged)
    let third = chat(&model, "What is the capital of Germany?");
    let outcome = run(&engine, &third);
    eprintln!(
        "third question: {} of {} prompt tokens cached",
        outcome.cached,
        third.len()
    );
    assert!(outcome.cached >= SYSTEM.len() / 8);
    assert_eq!(outcome.tokens, reference(&model, &third));
}

#[test]
#[ignore]
fn shared_prefix_is_copied_between_slots() {
    let model = model();
    let engine = engine(&model, 2);
    let a = chat(&model, "What is the capital of France?");
    let b = chat(&model, "What is the capital of Spain?");
    let c = chat(&model, "What is the capital of Japan?");

    run(&engine, &a);
    // b takes a's slot, c the other one, copying the shared prefix from a's
    // (when the model's memory allows it)
    let (rb, rc) = (submit(&engine, &b), submit(&engine, &c));
    let (ob, oc) = (wait(rb), wait(rc));
    eprintln!(
        "cached: b {} of {}, c {} of {}",
        ob.cached,
        b.len(),
        oc.cached,
        c.len()
    );
    assert_eq!(ob.tokens, reference(&model, &b));
    assert_eq!(oc.tokens, reference(&model, &c));
}

#[test]
#[ignore]
fn too_long_prompt_is_rejected() {
    let model = model();
    let engine = engine(&model, 1);
    let prompt = model.vocab.tokenize("hello ".repeat(5000).as_bytes(), true);
    assert!(prompt.len() >= engine.n_ctx());
    let outcome = run(&engine, &prompt);
    assert_eq!(outcome.end, Some(JobEnd::ContextFull));
    assert!(outcome.tokens.is_empty());
}
