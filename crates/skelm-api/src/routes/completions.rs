//! `POST /v1/completions` — legacy text completion (no chat template applied).

use std::sync::atomic::{AtomicBool, Ordering};

use axum::Json;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use tokio::sync::OwnedSemaphorePermit;

use skelm_exec::Model;

use crate::error::ApiError;
use crate::generate::{GenConfig, generate};
use crate::protocol::*;
use crate::sse::sse_stream;
use crate::state::{AppState, parse_model};

pub async fn completions(
    State(state): State<AppState>,
    Json(req): Json<CompletionRequest>,
) -> Result<Response, ApiError> {
    let descr = parse_model(&req.model)?;
    let model = state.load_model(&descr).await?;

    let prompt = req.prompt.into_text();
    let cfg = GenConfig {
        temperature: req.temperature,
        seed: req.seed,
        max_tokens: req.max_tokens,
        stop: stop_vec(req.stop),
    };
    let model_name = req.model;

    if req.stream {
        let permit = state.gen_permit().await;
        return Ok(stream_completion(model, prompt, cfg, model_name, permit));
    }

    let permit = state.gen_permit().await;
    let (out, prompt_tokens) = tokio::task::spawn_blocking(
        move || -> anyhow::Result<(crate::generate::GenResult, usize)> {
            let _permit = permit;
            let prompt_tokens = model.vocab.tokenize(prompt.as_bytes(), true).len();
            let mut context = model.new_context();
            let cancel = AtomicBool::new(false);
            let out = generate(&mut context, &prompt, &cfg, &cancel, |_| {})?;
            Ok((out, prompt_tokens))
        },
    )
    .await??;

    let response = CompletionResponse {
        id: new_id("cmpl"),
        object: "text_completion",
        created: unix_now(),
        model: model_name,
        choices: vec![CompletionChoice {
            index: 0,
            text: out.text,
            finish_reason: Some(out.finish_reason.as_str()),
            logprobs: None,
        }],
        usage: Usage {
            prompt_tokens,
            completion_tokens: out.completion_tokens,
            total_tokens: prompt_tokens + out.completion_tokens,
        },
    };

    Ok(Json(response).into_response())
}

fn stream_completion(
    model: Model,
    prompt: String,
    cfg: GenConfig,
    model_name: String,
    permit: OwnedSemaphorePermit,
) -> Response {
    let id = new_id("cmpl");
    let created = unix_now();

    sse_stream(move |tx| {
        let _permit = permit;
        let mut context = model.new_context();
        let cancel = AtomicBool::new(false);

        let result = generate(&mut context, &prompt, &cfg, &cancel, |chunk| {
            let ev = CompletionChunk {
                id: id.clone(),
                object: "text_completion",
                created,
                model: model_name.clone(),
                choices: vec![CompletionChoice {
                    index: 0,
                    text: chunk.to_string(),
                    finish_reason: None,
                    logprobs: None,
                }],
            };
            if !tx.send_json(&ev) {
                cancel.store(true, Ordering::Relaxed);
            }
        });

        let finish = match &result {
            Ok(out) => out.finish_reason.as_str(),
            Err(e) => {
                tracing::error!("completion stream generation failed: {e}");
                "stop"
            }
        };

        let last = CompletionChunk {
            id,
            object: "text_completion",
            created,
            model: model_name,
            choices: vec![CompletionChoice {
                index: 0,
                text: String::new(),
                finish_reason: Some(finish),
                logprobs: None,
            }],
        };
        tx.send_json(&last);
        tx.done();
    })
}
