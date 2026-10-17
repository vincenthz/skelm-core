//! An OpenAI-compatible HTTP API for skelm, served with [axum].
//!
//! The [`router`] builds a [`axum::Router`] exposing the OpenAI surface backed
//! by locally installed models, and [`serve`] binds it to an address. The
//! implemented endpoints are:
//!
//! * `POST /v1/chat/completions` — chat, with `stream`, `tools`, `temperature`,
//!   `max_tokens`, `seed` and `stop` support.
//! * `POST /v1/completions` — legacy text completion.
//! * `POST /v1/embeddings` — embeddings (for embedding models).
//! * `GET /v1/models` and `GET /v1/models/{id}` — installed models.
//! * `GET /health` — liveness probe.
//!
//! Models are named exactly as the CLI names them (`qwen2.5:7b`,
//! `registry/model:variant`) and are pulled from the shared Ollama store. The
//! request/response types live in [`protocol`] for building clients.
//!
//! Each model generates its requests together, batched on one context (see
//! [`skelm_exec::engine`]): up to [`ServeOptions::parallel`] at a time, the
//! others waiting for a slot. Prompts are cached: a request whose prompt
//! starts like a previous one (the same conversation with one more turn, a
//! shared system prompt) only processes the rest. Models with speculative
//! decoding generate one request at a time instead, without prompt caching.

mod error;
mod generate;
pub mod protocol;
mod routes;
mod sse;
mod state;

use std::net::SocketAddr;

use axum::Router;
use axum::routing::{get, post};

pub use error::ApiError;
pub use state::{AppState, DraftSource, ServeOptions};

/// Build the API router with the given shared state.
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/v1/models", get(routes::models::list_models))
        // Wildcard so ids containing `/` (a non-default registry) still match.
        .route("/v1/models/{*id}", get(routes::models::get_model))
        .route("/v1/chat/completions", post(routes::chat::chat_completions))
        .route("/v1/completions", post(routes::completions::completions))
        .route("/v1/embeddings", post(routes::embeddings::embeddings))
        .with_state(state)
}

async fn health() -> &'static str {
    "ok"
}

/// Bind the API to `addr` and serve until the process is stopped.
pub async fn serve(addr: SocketAddr) -> anyhow::Result<()> {
    serve_with(addr, ServeOptions::default()).await
}

/// Like [`serve`], with LoRA adapters / speculative decoding applied to every model
pub async fn serve_with(addr: SocketAddr, options: ServeOptions) -> anyhow::Result<()> {
    let app = router(AppState::with_options(options));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("skelm OpenAI-compatible API listening on http://{addr}");
    axum::serve(listener, app).await?;
    Ok(())
}
