//! `POST /v1/completions` — legacy text completion (no chat template applied).

use axum::Json;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use tokio::sync::mpsc::UnboundedReceiver;

use crate::error::ApiError;
use crate::generate::{GenConfig, GenEvent, collect};
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

    let events = state.generate(&descr, &model, &prompt, cfg).await?;
    if req.stream {
        return Ok(stream_completion(events, model_name));
    }
    let out = collect(events).await?;

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
        usage: Usage::new(out.prompt_tokens, out.cached_tokens, out.completion_tokens),
    };

    Ok(Json(response).into_response())
}

fn stream_completion(mut events: UnboundedReceiver<GenEvent>, model_name: String) -> Response {
    let id = new_id("cmpl");
    let created = unix_now();

    sse_stream(move |tx| async move {
        let chunk = |text: String, finish_reason: Option<&'static str>| CompletionChunk {
            id: id.clone(),
            object: "text_completion",
            created,
            model: model_name.clone(),
            choices: vec![CompletionChoice {
                index: 0,
                text,
                finish_reason,
                logprobs: None,
            }],
        };

        // returning drops `events`, which cancels the generation
        let finish = loop {
            match events.recv().await {
                Some(GenEvent::Text(text)) => {
                    if !tx.send_json(&chunk(text, None)) {
                        return;
                    }
                }
                Some(GenEvent::Done(out)) => break out.finish_reason.as_str(),
                Some(GenEvent::Error(e)) => {
                    tracing::error!("completion stream generation failed: {e}");
                    break "stop";
                }
                None => {
                    tracing::error!("completion stream generation aborted");
                    break "stop";
                }
            }
        };

        tx.send_json(&chunk(String::new(), Some(finish)));
        tx.done();
    })
}
