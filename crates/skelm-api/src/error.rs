//! API error type rendered as an OpenAI-style error envelope.

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

/// An error surfaced to an API caller. Each variant maps to an HTTP status and
/// an OpenAI `error.type`.
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    /// The requested model isn't installed / has no manifest.
    #[error("model not found: {0}")]
    ModelNotFound(String),
    /// The request was malformed in a way we could describe.
    #[error("{0}")]
    BadRequest(String),
    /// Anything that went wrong while serving the request.
    #[error("{0}")]
    Internal(String),
}

impl ApiError {
    fn parts(&self) -> (StatusCode, &'static str) {
        match self {
            ApiError::ModelNotFound(_) => (StatusCode::NOT_FOUND, "invalid_request_error"),
            ApiError::BadRequest(_) => (StatusCode::BAD_REQUEST, "invalid_request_error"),
            ApiError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal_error"),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, kind) = self.parts();
        if status == StatusCode::INTERNAL_SERVER_ERROR {
            tracing::error!("request failed: {self}");
        }
        let body = json!({
            "error": {
                "message": self.to_string(),
                "type": kind,
                "param": serde_json::Value::Null,
                "code": serde_json::Value::Null,
            }
        });
        (status, Json(body)).into_response()
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(e: anyhow::Error) -> Self {
        ApiError::Internal(e.to_string())
    }
}

impl From<tokio::task::JoinError> for ApiError {
    fn from(e: tokio::task::JoinError) -> Self {
        ApiError::Internal(format!("worker task failed: {e}"))
    }
}
