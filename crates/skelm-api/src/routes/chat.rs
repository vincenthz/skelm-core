//! `POST /v1/chat/completions`.

use axum::Json;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use tokio::sync::mpsc::UnboundedReceiver;

use skelm_exec::{ModelParameters, parse_tool_calls};

use crate::error::ApiError;
use crate::generate::{GenConfig, GenEvent, collect};
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

    let prompt = model.model_template_render(&params);
    let events = state.generate(&descr, &model, &prompt, cfg).await?;
    if req.stream {
        return Ok(stream_chat(events, model_name));
    }
    let out = collect(events).await?;

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
        usage: Usage::new(out.prompt_tokens, out.cached_tokens, out.completion_tokens),
    };

    Ok(Json(response).into_response())
}

/// Streaming variant: emit an initial role delta, then content deltas as tokens
/// arrive, then a final chunk carrying `finish_reason` (and tool calls, if any).
///
/// Tool-call markup is recovered from the buffered text at the end; for reliable
/// tool-call extraction prefer a non-streaming request.
fn stream_chat(mut events: UnboundedReceiver<GenEvent>, model_name: String) -> Response {
    let id = new_id("chatcmpl");
    let created = unix_now();

    sse_stream(move |tx| async move {
        let chunk = |delta: ChunkDelta, finish_reason: Option<&'static str>| ChatCompletionChunk {
            id: id.clone(),
            object: "chat.completion.chunk",
            created,
            model: model_name.clone(),
            choices: vec![ChunkChoice {
                index: 0,
                delta,
                finish_reason,
            }],
        };

        let first = ChunkDelta {
            role: Some("assistant"),
            ..Default::default()
        };
        if !tx.send_json(&chunk(first, None)) {
            return;
        }

        // returning drops `events`, which cancels the generation
        let out = loop {
            match events.recv().await {
                Some(GenEvent::Text(text)) => {
                    let delta = ChunkDelta {
                        content: Some(text),
                        ..Default::default()
                    };
                    if !tx.send_json(&chunk(delta, None)) {
                        return;
                    }
                }
                Some(GenEvent::Done(out)) => break out,
                Some(GenEvent::Error(e)) => {
                    tracing::error!("chat stream generation failed: {e}");
                    tx.done();
                    return;
                }
                None => {
                    tracing::error!("chat stream generation aborted");
                    tx.done();
                    return;
                }
            }
        };

        let parsed = parse_tool_calls(&out.text);
        let (tool_calls, finish) = if parsed.tool_calls.is_empty() {
            (None, out.finish_reason.as_str())
        } else {
            (Some(to_chunk_tool_calls(parsed.tool_calls)), "tool_calls")
        };
        let last = ChunkDelta {
            tool_calls,
            ..Default::default()
        };
        tx.send_json(&chunk(last, Some(finish)));
        tx.done();
    })
}
