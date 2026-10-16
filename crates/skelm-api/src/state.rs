//! Shared server state: a loaded-model cache and a generation gate.

use std::str::FromStr;
use std::sync::Arc;

use tokio::sync::{OwnedSemaphorePermit, Semaphore};

use skelm_exec::{Model, ModelDescr, ModelLoadError, Models};
use skelm_ollama as ollama;

use crate::error::ApiError;

/// Application state handed to every route. Cheap to clone (everything inside is
/// reference-counted).
#[derive(Clone)]
pub struct AppState {
    /// Cache of loaded models, keyed by descriptor. Loading a GGUF is expensive,
    /// so we keep each resident after first use.
    models: Models<ModelDescr>,
    /// Serializes token generation. llama.cpp inference already saturates the
    /// available cores, so running requests one at a time keeps the backend
    /// simple and avoids sharing a model across contexts concurrently.
    gen_gate: Arc<Semaphore>,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            models: Models::new(),
            gen_gate: Arc::new(Semaphore::new(1)),
        }
    }

    /// Load a model, returning a cached handle when one already exists. The blocking
    /// `llama.cpp` load runs on the blocking thread pool.
    pub async fn load_model(&self, descr: &ModelDescr) -> Result<Model, ApiError> {
        if let Some(model) = self.models.get(descr.clone()) {
            return Ok(model);
        }

        let descr_for_task = descr.clone();
        let model = tokio::task::spawn_blocking(move || Model::load(&descr_for_task))
            .await?
            .map_err(load_error)?;

        self.models.set(descr.clone(), model.clone());
        Ok(model)
    }

    /// Acquire the generation gate. The returned permit is released on drop, so
    /// hold it for the whole generation (move it into the blocking task).
    pub async fn gen_permit(&self) -> OwnedSemaphorePermit {
        // The semaphore is never closed, so acquisition cannot fail.
        self.gen_gate
            .clone()
            .acquire_owned()
            .await
            .expect("generation semaphore closed")
    }
}

/// Map a model load failure to the right API error: a missing manifest / config
/// is a 404, an actual load failure is a 500.
fn load_error(e: ModelLoadError) -> ApiError {
    match e {
        ModelLoadError::OllamaConfigGetError(inner) => ApiError::ModelNotFound(inner.to_string()),
        ModelLoadError::LlamaModelFailedLoading(inner) => {
            ApiError::Internal(format!("failed to load model: {inner}"))
        }
    }
}

/// Parse an OpenAI `model` string (`<model>:<variant>` or
/// `<registry>/<model>:<variant>`) into a skelm descriptor.
pub fn parse_model(name: &str) -> Result<ModelDescr, ApiError> {
    ollama::ModelDescr::from_str(name)
        .map(ModelDescr::Ollama)
        .map_err(|_| {
            ApiError::BadRequest(format!(
                "invalid model name '{name}': expected '<model>:<variant>' or '<registry>/<model>:<variant>'"
            ))
        })
}
