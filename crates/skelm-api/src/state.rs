//! Shared server state: a loaded-model cache and a generation gate.

use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use tokio::sync::{OwnedSemaphorePermit, Semaphore};

use skelm_exec::{
    ContextOptions, DraftParams, Drafter, Model, ModelDescr, ModelLoadError, ModelParams, Models,
    SpeculativeConfig,
};
use skelm_ollama as ollama;

use crate::error::ApiError;

/// Where the drafts of speculative decoding come from
#[derive(Clone, Debug)]
pub enum DraftSource {
    /// a draft model, shared by every model the server loads
    Model(ModelDescr),
    /// the MTP layers of each model (models without MTP layers decode normally)
    Mtp,
}

/// Settings applied to every model the server loads
#[derive(Clone, Debug, Default)]
pub struct ServeOptions {
    /// LoRA adapters `(path, scale)`, loaded for every model
    pub lora: Vec<(PathBuf, f32)>,
    /// speculative decoding
    pub speculative: Option<(DraftSource, DraftParams)>,
}

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
    options: Arc<ServeOptions>,
    /// the draft model, loaded on first use
    draft: Arc<Mutex<Option<Model>>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        Self::with_options(ServeOptions::default())
    }

    pub fn with_options(options: ServeOptions) -> Self {
        AppState {
            models: Models::new(),
            gen_gate: Arc::new(Semaphore::new(1)),
            options: Arc::new(options),
            draft: Arc::default(),
        }
    }

    /// Load a model, returning a cached handle when one already exists. The blocking
    /// `llama.cpp` load runs on the blocking thread pool.
    pub async fn load_model(&self, descr: &ModelDescr) -> Result<Model, ApiError> {
        if let Some(model) = self.models.get(descr.clone()) {
            return Ok(model);
        }

        let descr_for_task = descr.clone();
        let state = self.clone();
        let model =
            tokio::task::spawn_blocking(move || state.load_blocking(&descr_for_task)).await??;

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

impl AppState {
    /// Load a model and set it up from the [`ServeOptions`]
    fn load_blocking(&self, descr: &ModelDescr) -> Result<Model, ApiError> {
        let mtp = matches!(self.options.speculative, Some((DraftSource::Mtp, _)));
        let params = ModelParams {
            load_mtp: mtp,
            ..ModelParams::default()
        };
        let model = Model::load_with_params(descr, &params).map_err(load_error)?;

        let lora = self
            .options
            .lora
            .iter()
            .map(|(path, scale)| model.load_lora(path).map(|adapter| (adapter, *scale)))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| ApiError::Internal(e.to_string()))?;

        let speculative = match &self.options.speculative {
            None => None,
            Some((DraftSource::Mtp, draft)) if model.model.has_mtp() => Some(SpeculativeConfig {
                drafter: Drafter::Mtp,
                draft: *draft,
            }),
            Some((DraftSource::Mtp, _)) => {
                tracing::warn!(
                    "{descr} has no MTP layers, speculative decoding is disabled for it"
                );
                None
            }
            Some((DraftSource::Model(draft_descr), draft)) => Some(SpeculativeConfig {
                drafter: Drafter::Model {
                    model: self.draft_model(draft_descr)?.model,
                    params: Default::default(),
                },
                draft: *draft,
            }),
        };

        let with_options = |speculative| {
            model.clone().with_context_options(ContextOptions {
                lora: lora.clone(),
                speculative,
                ..ContextOptions::default()
            })
        };
        match with_options(speculative) {
            Ok(model) => Ok(model),
            Err(skelm_exec::ContextError::Speculative(e)) => {
                tracing::warn!("speculative decoding is disabled for {descr}: {e}");
                with_options(None).map_err(|e| ApiError::Internal(e.to_string()))
            }
            Err(e) => Err(ApiError::Internal(e.to_string())),
        }
    }

    /// The draft model, loaded once and shared by every model
    fn draft_model(&self, descr: &ModelDescr) -> Result<Model, ApiError> {
        let mut draft = self.draft.lock().expect("draft model lock");
        if let Some(model) = draft.as_ref() {
            return Ok(model.clone());
        }
        // an MTP assistant needs its MTP layers
        let params = ModelParams {
            load_mtp: true,
            ..ModelParams::default()
        };
        let model = Model::load_with_params(descr, &params).map_err(load_error)?;
        *draft = Some(model.clone());
        Ok(model)
    }
}

/// Map a model load failure to the right API error: a missing manifest / config
/// is a 404, an actual load failure is a 500.
fn load_error(e: ModelLoadError) -> ApiError {
    match e {
        ModelLoadError::OllamaConfigGetError(inner) => ApiError::ModelNotFound(inner.to_string()),
        ModelLoadError::HfError(inner @ skelm_hf::HfError::NotCached(_)) => {
            ApiError::ModelNotFound(inner.to_string())
        }
        ModelLoadError::HfError(inner) => ApiError::BadRequest(inner.to_string()),
        ModelLoadError::LlamaModelFailedLoading(inner) => {
            ApiError::Internal(format!("failed to load model: {inner}"))
        }
    }
}

/// Parse an OpenAI `model` string (`<model>:<variant>` or
/// `<registry>/<model>:<variant>`) into a skelm descriptor.
pub fn parse_model(name: &str) -> Result<ModelDescr, ApiError> {
    // file paths are deliberately not accepted from API clients
    if skelm_hf::HfModelRef::is_hf_name(name) {
        return name
            .parse()
            .map(ModelDescr::Hf)
            .map_err(|e: skelm_hf::HfError| ApiError::BadRequest(e.to_string()));
    }
    ollama::ModelDescr::from_str(name)
        .map(ModelDescr::Ollama)
        .map_err(|_| {
            ApiError::BadRequest(format!(
                "invalid model name '{name}': expected '<model>:<variant>', \
                 '<registry>/<model>:<variant>' or 'hf.co/<owner>/<name>[:<quantization>]'"
            ))
        })
}
