//! `POST /v1/embeddings`.

use axum::Json;
use axum::extract::State;

use crate::error::ApiError;
use crate::protocol::*;
use crate::state::{AppState, parse_model};

pub async fn embeddings(
    State(state): State<AppState>,
    Json(req): Json<EmbeddingRequest>,
) -> Result<Json<EmbeddingResponse>, ApiError> {
    let descr = parse_model(&req.model)?;
    let model = state.load_model(&descr).await?;

    let inputs = req.input.into_vec();
    if inputs.is_empty() {
        return Err(ApiError::BadRequest(
            "`input` must not be empty".to_string(),
        ));
    }

    let permit = state.gen_permit().await;
    let model_name = req.model;

    let (data, total_tokens) =
        tokio::task::spawn_blocking(move || -> anyhow::Result<(Vec<EmbeddingData>, usize)> {
            let _permit = permit;
            let mut data = Vec::with_capacity(inputs.len());
            let mut total = 0usize;
            for (index, input) in inputs.iter().enumerate() {
                let tokens = model.vocab.tokenize(input.as_bytes(), true);
                total += tokens.len();
                // A fresh context per input keeps the pooled embedding isolated.
                let mut context = model.new_context_embeddings();
                let embedding = context.1.embeddings(&tokens)?;
                data.push(EmbeddingData {
                    object: "embedding",
                    embedding,
                    index,
                });
            }
            Ok((data, total))
        })
        .await??;

    Ok(Json(EmbeddingResponse {
        object: "list",
        data,
        model: model_name,
        usage: EmbeddingUsage {
            prompt_tokens: total_tokens,
            total_tokens,
        },
    }))
}
