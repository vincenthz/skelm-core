//! Blocking token generation shared by the chat and completion routes.
//!
//! This wraps skelm-exec's context primitives with the extra behavior the API
//! surface needs — `max_tokens` capping, `stop` sequences, and incremental
//! UTF-8-safe decoding for streaming — without pulling that policy into the
//! lower-level crates.

use std::sync::atomic::{AtomicBool, Ordering};

use skelm_exec::Context;
use skelm_llama_cpp::{
    SamplerChain, SamplerDistance, SamplerGreedy, SamplerMinP, SamplerTemperature,
};

/// Sampling / stopping knobs distilled from a request.
#[derive(Debug, Clone, Default)]
pub struct GenConfig {
    pub temperature: Option<f32>,
    pub seed: Option<u32>,
    pub max_tokens: Option<usize>,
    pub stop: Vec<String>,
}

/// Why generation ended.
#[derive(Debug, Clone, Copy)]
pub enum FinishReason {
    /// End-of-generation token, a stop sequence, or client cancellation.
    Stop,
    /// The `max_tokens` cap was reached.
    Length,
}

impl FinishReason {
    pub fn as_str(self) -> &'static str {
        match self {
            FinishReason::Stop => "stop",
            FinishReason::Length => "length",
        }
    }
}

pub struct GenResult {
    /// The full generated text, with any trailing stop sequence removed.
    pub text: String,
    /// Number of tokens sampled (including control tokens).
    pub completion_tokens: usize,
    pub finish_reason: FinishReason,
}

/// Build the sampler chain for a request.
///
/// `temperature <= 0` selects greedy decoding; otherwise we use the same
/// min-p + temperature + distribution chain as the CLI default, seeded by the
/// request (or llama.cpp's default seed).
fn build_sampler(cfg: &GenConfig) -> SamplerChain {
    let mut chain = SamplerChain::new();
    let temperature = cfg.temperature.unwrap_or(0.8);
    let seed = cfg.seed.unwrap_or(0xFFFF_FFFF);

    if temperature <= 0.0 {
        chain.add(Box::new(SamplerGreedy));
    } else {
        chain.add(Box::new(SamplerMinP::new(0.05, 1)));
        chain.add(Box::new(SamplerTemperature::new(temperature)));
        chain.add(Box::new(SamplerDistance::new(seed)));
    }
    chain
}

/// Byte offset of the earliest stop-sequence match in `s`, if any.
fn earliest_stop(s: &str, stops: &[String]) -> Option<usize> {
    stops
        .iter()
        .filter(|stop| !stop.is_empty())
        .filter_map(|stop| s.find(stop.as_str()))
        .min()
}

/// Length (in bytes) of the longest suffix of `s` that is a *strict* prefix of
/// some stop sequence. Those trailing bytes are held back from streaming until
/// we know whether they complete a stop sequence, so partial markers never leak.
fn holdback_len(s: &str, stops: &[String]) -> usize {
    let mut max = 0usize;
    for stop in stops {
        if stop.is_empty() {
            continue;
        }
        // A full match is handled by `earliest_stop`; only prefixes shorter than
        // the whole stop matter here.
        let upper = (stop.len() - 1).min(s.len());
        let mut k = upper;
        while k >= 1 {
            let s_start = s.len() - k;
            if stop.is_char_boundary(k)
                && s.is_char_boundary(s_start)
                && s.as_bytes()[s_start..] == stop.as_bytes()[..k]
            {
                max = max.max(k);
                break;
            }
            k -= 1;
        }
    }
    max
}

/// The valid-UTF-8 prefix of `bytes` as a `&str` (tokens can split a codepoint,
/// so the trailing incomplete bytes are excluded until completed).
fn valid_prefix(bytes: &[u8]) -> &str {
    let valid = match std::str::from_utf8(bytes) {
        Ok(s) => return s,
        Err(e) => e.valid_up_to(),
    };
    // `valid` is a char boundary by definition, so this cannot fail.
    std::str::from_utf8(&bytes[..valid]).expect("valid utf8 prefix")
}

/// Append `prompt` to `context` and generate a reply.
///
/// `on_text` receives each newly finalized slice of decoded text (never partial
/// UTF-8, never a partial stop sequence) — feed it to a streaming sink, or pass
/// a no-op closure and read [`GenResult::text`] for the buffered result.
pub fn generate(
    context: &mut Context,
    prompt: &str,
    cfg: &GenConfig,
    cancel: &AtomicBool,
    mut on_text: impl FnMut(&str),
) -> anyhow::Result<GenResult> {
    let mut sampler = build_sampler(cfg);
    context.append_bytes(prompt.as_bytes());

    let vocab = context.0.vocab.clone();
    let llama_ctx = &mut context.1;

    let mut bytes: Vec<u8> = Vec::new();
    let mut emitted: usize = 0; // byte offset into decoded text already sent to `on_text`
    let mut completion_tokens = 0usize;
    let mut finish = FinishReason::Stop;

    loop {
        if cancel.load(Ordering::Relaxed) {
            break;
        }
        let Some(token) = llama_ctx.next_token(&mut sampler, &vocab) else {
            break; // end-of-generation
        };
        llama_ctx.append_tokens(&[token])?;
        completion_tokens += 1;

        if !vocab.token_attr(token).is_control() {
            bytes.extend_from_slice(&vocab.as_bytes(token));
            let text = valid_prefix(&bytes);

            if let Some(cut) = earliest_stop(text, &cfg.stop) {
                if cut > emitted {
                    on_text(&text[emitted..cut]);
                }
                return Ok(GenResult {
                    text: text[..cut].to_string(),
                    completion_tokens,
                    finish_reason: FinishReason::Stop,
                });
            }

            let end = text.len() - holdback_len(text, &cfg.stop);
            if end > emitted {
                on_text(&text[emitted..end]);
                emitted = end;
            }
        }

        if let Some(max) = cfg.max_tokens {
            if completion_tokens >= max {
                finish = FinishReason::Length;
                break;
            }
        }
    }

    // Flush any bytes withheld for a stop sequence that never completed.
    let text = valid_prefix(&bytes);
    if text.len() > emitted {
        on_text(&text[emitted..]);
    }

    Ok(GenResult {
        text: String::from_utf8_lossy(&bytes).into_owned(),
        completion_tokens,
        finish_reason: finish,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn earliest_stop_finds_first() {
        let stops = vec!["END".to_string(), "STOP".to_string()];
        assert_eq!(earliest_stop("hi STOP there END", &stops), Some(3));
        assert_eq!(earliest_stop("nothing here", &stops), None);
    }

    #[test]
    fn holdback_matches_partial_suffix() {
        let stops = vec!["STOP".to_string()];
        // "...STO" is a strict prefix of "STOP" -> hold back 3 bytes.
        assert_eq!(holdback_len("abcSTO", &stops), 3);
        // A completed match is not a held-back prefix.
        assert_eq!(holdback_len("abcSTOP", &stops), 0);
        // No shared suffix.
        assert_eq!(holdback_len("abcdef", &stops), 0);
    }

    #[test]
    fn valid_prefix_drops_incomplete_codepoint() {
        // "é" is 0xC3 0xA9; a lone leading byte must be withheld.
        assert_eq!(valid_prefix(&[b'a', 0xC3]), "a");
        assert_eq!(valid_prefix(&[b'a', 0xC3, 0xA9]), "aé");
    }
}
