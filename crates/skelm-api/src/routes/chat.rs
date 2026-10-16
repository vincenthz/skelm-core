//! `POST /v1/chat/completions`.

use std::sync::atomic::{AtomicBool, Ordering};

use axum::Json;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use tokio::sync::OwnedSemaphorePermit;

use skelm_exec::{Model, ModelParameters, parse_tool_calls};

use crate::error::ApiError;
use crate::generate::{GenConfig, generate};
use crate::protocol::*;
use crate::sse::sse_stream;
use crate::state::{AppState, parse_model};

pub async fn chat_completions(
    State(state): State<AppState>,
    Json(req): Json<ChatCompletionRequest>,
) -> Result<Response, ApiError> {
    let descr = parse_model(&req.model)?;
    let model = state.load_model(&descr).await?;

    let mut params = ModelParameters::new(to_exec_messages(req.messages));
    params.tools = req.tools.unwrap_or_default();
    params.add_generation_prompt = true;

    let cfg = GenConfig {
        temperature: req.temperature,
        seed: req.seed,
        max_tokens: req.max_tokens,
        stop: stop_vec(req.stop),
    };
    let model_name = req.model;

    if req.stream {
        let permit = state.gen_permit().await;
        return Ok(stream_chat(model, params, cfg, model_name, permit));
    }

    let permit = state.gen_permit().await;
    let (out, prompt_tokens) = tokio::task::spawn_blocking(
        move || -> anyhow::Result<(crate::generate::GenResult, usize)> {
            let _permit = permit;
            let prompt = model.model_template_render(&params);
            let prompt_tokens = model.vocab.tokenize(prompt.as_bytes(), true).len();
            let mut context = model.new_context();
            let cancel = AtomicBool::new(false);
            let out = generate(&mut context, &prompt, &cfg, &cancel, |_| {})?;
            Ok((out, prompt_tokens))
        },
    )
    .await??;

    // Recover any tool calls the model emitted; otherwise the whole text is the answer.
    let parsed = parse_tool_calls(&out.text);
    let (content, tool_calls, finish): (
        Option<String>,
        Option<Vec<ResponseToolCall>>,
        &'static str,
    ) = if parsed.tool_calls.is_empty() {
        (Some(out.text), None, out.finish_reason.as_str())
    } else {
        let content = (!parsed.content.is_empty()).then_some(parsed.content);
        (
            content,
            Some(to_response_tool_calls(parsed.tool_calls)),
            "tool_calls",
        )
    };

    let response = ChatCompletionResponse {
        id: new_id("chatcmpl"),
        object: "chat.completion",
        created: unix_now(),
        model: model_name,
        choices: vec![ChatChoice {
            index: 0,
            message: ResponseMessage {
                role: "assistant",
                content,
                tool_calls,
            },
            finish_reason: finish,
        }],
        usage: Usage {
            prompt_tokens,
            completion_tokens: out.completion_tokens,
            total_tokens: prompt_tokens + out.completion_tokens,
        },
    };

    Ok(Json(response).into_response())
}

/// Streaming variant: emit an initial role delta, then content deltas as tokens
/// arrive, then a final chunk carrying `finish_reason` (and tool calls, if any).
///
/// Tool-call markup is recovered from the buffered text at the end; for reliable
/// tool-call extraction prefer a non-streaming request.
fn stream_chat(
    model: Model,
    params: ModelParameters,
    cfg: GenConfig,
    model_name: String,
    permit: OwnedSemaphorePermit,
) -> Response {
    let id = new_id("chatcmpl");
    let created = unix_now();

    sse_stream(move |tx| {
        let _permit = permit;

        let first = ChatCompletionChunk {
            id: id.clone(),
            object: "chat.completion.chunk",
            created,
            model: model_name.clone(),
            choices: vec![ChunkChoice {
                index: 0,
                delta: ChunkDelta {
                    role: Some("assistant"),
                    ..Default::default()
                },
                finish_reason: None,
            }],
        };
        if !tx.send_json(&first) {
            return;
        }

        let prompt = model.model_template_render(&params);
        let mut context = model.new_context();
        let cancel = AtomicBool::new(false);

        let result = generate(&mut context, &prompt, &cfg, &cancel, |chunk| {
            let ev = ChatCompletionChunk {
                id: id.clone(),
                object: "chat.completion.chunk",
                created,
                model: model_name.clone(),
                choices: vec![ChunkChoice {
                    index: 0,
                    delta: ChunkDelta {
                        content: Some(chunk.to_string()),
                        ..Default::default()
                    },
                    finish_reason: None,
                }],
            };
            if !tx.send_json(&ev) {
                cancel.store(true, Ordering::Relaxed);
            }
        });

        let out = match result {
            Ok(out) => out,
            Err(e) => {
                tracing::error!("chat stream generation failed: {e}");
                tx.done();
                return;
            }
        };

        let parsed = parse_tool_calls(&out.text);
        let (tool_calls, finish) = if parsed.tool_calls.is_empty() {
            (None, out.finish_reason.as_str())
        } else {
            (Some(to_chunk_tool_calls(parsed.tool_calls)), "tool_calls")
        };

        let last = ChatCompletionChunk {
            id,
            object: "chat.completion.chunk",
            created,
            model: model_name,
            choices: vec![ChunkChoice {
                index: 0,
                delta: ChunkDelta {
                    tool_calls,
                    ..Default::default()
                },
                finish_reason: Some(finish),
            }],
        };
        tx.send_json(&last);
        tx.done();
    })
}
