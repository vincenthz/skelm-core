//! Text generation primitive.
//!
//! The interactive tool loop and the [`orchestrator`](crate::orchestrator) both
//! need to turn a rendered prompt into generated text; this module provides that
//! step so callers don't each reimplement the token loop.

use std::sync::atomic::{AtomicBool, Ordering};

use skelm_llama_cpp::{
    Sampler, SamplerChain, SamplerDistance, SamplerMinP, SamplerTemperature,
};

use crate::Context;

/// The default sampler chain: min-p 0.05, temperature 0.8, then distribution
/// sampling. A fresh chain must be built per generation — samplers carry RNG and
/// acceptance state.
pub fn default_sampler() -> SamplerChain {
    let mut sampler = SamplerChain::new();
    sampler.add(Box::new(SamplerMinP::new(0.05, 1)));
    sampler.add(Box::new(SamplerTemperature::new(0.8)));
    sampler.add(Box::new(SamplerDistance::new(0xFFFF_FFFF)));
    sampler
}

impl Context {
    /// Append `prompt` to the context and generate tokens until end-of-generation
    /// (or `cancel` is set), returning the full decoded text.
    ///
    /// `on_token` receives each decoded token's bytes as they are produced, for
    /// streaming; control tokens are skipped (they carry no user-visible text).
    /// Uses [`default_sampler`]; see [`Context::generate_with`] to supply your own.
    pub fn generate(
        &mut self,
        prompt: &str,
        cancel: &AtomicBool,
        on_token: impl FnMut(&[u8]),
    ) -> anyhow::Result<String> {
        let mut sampler = default_sampler();
        self.generate_with(prompt, &mut sampler, cancel, on_token)
    }

    /// Like [`Context::generate`] but driven by a caller-provided `sampler`.
    pub fn generate_with<S: Sampler>(
        &mut self,
        prompt: &str,
        sampler: &mut S,
        cancel: &AtomicBool,
        mut on_token: impl FnMut(&[u8]),
    ) -> anyhow::Result<String> {
        self.append_bytes(prompt.as_bytes());

        let vocab = self.0.vocab.clone();
        let context = &mut self.1;

        let mut generated: Vec<u8> = Vec::new();
        while !cancel.load(Ordering::Relaxed) {
            match context.next_token(sampler, &vocab) {
                None => break,
                Some(t) => {
                    context.append_tokens(&[t])?;
                    if vocab.token_attr(t).is_control() {
                        continue;
                    }
                    let bytes = vocab.as_bytes(t);
                    on_token(&bytes);
                    generated.extend_from_slice(&bytes);
                }
            }
        }

        Ok(String::from_utf8_lossy(&generated).into_owned())
    }
}
