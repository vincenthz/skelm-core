//! Shared server state: the loaded models, and the engines generating with them.

use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use tokio::sync::mpsc::{self, UnboundedReceiver};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

use skelm_exec::{
    ContextOptions, DraftParams, Drafter, Engine, EngineConfig, Job, JobEnd, JobEvent, JobSink,
    Model, ModelDescr, ModelLoadError, ModelParams, Models, SpeculativeConfig,
};
use skelm_ollama as ollama;

use crate::error::ApiError;
use crate::generate::{ApiSink, GenConfig, GenEvent, build_sampler, generate_serial};

/// Where the drafts of speculative decoding come from
#[derive(Clone, Debug)]
pub enum DraftSource {
    /// a draft model, shared by every model the server loads
    Model(ModelDescr),
    /// the MTP layers of each model (models without MTP layers decode normally)
    Mtp,
}

/// Settings applied to every model the server loads
#[derive(Clone, Debug)]
pub struct ServeOptions {
    /// LoRA adapters `(path, scale)`, loaded for every model
    pub lora: Vec<(PathBuf, f32)>,
    /// speculative decoding (requests to models using it are generated one
    /// at a time, without prompt caching)
    pub speculative: Option<(DraftSource, DraftParams)>,
    /// requests generated at the same time, per model
    pub parallel: usize,
    /// context size in tokens of each model, shared by its parallel requests
    /// (`None`: the default, 16384)
    pub ctx_size: Option<u32>,
}

impl Default for ServeOptions {
    fn default() -> Self {
        ServeOptions {
            lora: Vec::new(),
            speculative: None,
            parallel: EngineConfig::default().slots,
            ctx_size: None,
        }
    }
}

/// Application state handed to every route. Cheap to clone (everything inside is
/// reference-counted).
#[derive(Clone)]
pub struct AppState {
    /// Cache of loaded models, keyed by descriptor. Loading a GGUF is expensive,
    /// so we keep each resident after first use.
    models: Models<ModelDescr>,
    /// The engine of each model, generating its requests together, created on
    /// first use
    engines: Arc<tokio::sync::Mutex<HashMap<ModelDescr, Arc<Engine>>>>,
    /// Serializes the embeddings, and the generations of models with
    /// speculative decoding (which the engines don't do): each runs on a
    /// context of its own
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
            engines: Arc::default(),
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
    pub(crate) async fn gen_permit(&self) -> OwnedSemaphorePermit {
        // The semaphore is never closed, so acquisition cannot fail.
        self.gen_gate
            .clone()
            .acquire_owned()
            .await
            .expect("generation semaphore closed")
    }

    /// The engine of a model, created on first use
    async fn engine(&self, descr: &ModelDescr, model: &Model) -> Result<Arc<Engine>, ApiError> {
        let mut engines = self.engines.lock().await;
        if let Some(engine) = engines.get(descr) {
            return Ok(engine.clone());
        }
        let config = EngineConfig {
            slots: self.options.parallel,
            n_ctx: model.options.params.n_ctx,
        };
        let model = model.clone();
        let engine = tokio::task::spawn_blocking(move || Engine::new(&model, &config))
            .await?
            .map_err(|e| ApiError::Internal(e.to_string()))?;
        tracing::info!(
            "{descr}: {} parallel requests, {} tokens of context",
            engine.slots(),
            engine.n_ctx()
        );
        let engine = Arc::new(engine);
        engines.insert(descr.clone(), engine.clone());
        Ok(engine)
    }

    /// Generate a reply to `prompt` with `model`: the returned channel gets
    /// the text as it is generated, then the result. Dropping it cancels the
    /// generation.
    pub async fn generate(
        &self,
        descr: &ModelDescr,
        model: &Model,
        prompt: &str,
        cfg: GenConfig,
    ) -> Result<UnboundedReceiver<GenEvent>, ApiError> {
        let (tx, rx) = mpsc::unbounded_channel();
        let tokens = model.vocab.tokenize(prompt.as_bytes(), true);

        if model.options.speculative.is_some() {
            let permit = self.gen_permit().await;
            let model = model.clone();
            tokio::task::spawn_blocking(move || {
                let _permit = permit;
                let n_ctx = model.options.params.n_ctx as usize;
                let mut sink = ApiSink::new(tx, &cfg, n_ctx);
                match model.try_new_context() {
                    Ok(mut context) => generate_serial(&mut context, &tokens, &cfg, &mut sink),
                    Err(e) => {
                        sink.event(JobEvent::Finished(JobEnd::Error(e.to_string())));
                    }
                }
            });
            return Ok(rx);
        }

        let engine = self.engine(descr, model).await?;
        let sink = ApiSink::new(tx, &cfg, engine.n_ctx());
        let job = Job {
            prompt: tokens,
            max_tokens: cfg.max_tokens,
            sampler: Box::new(move || Box::new(build_sampler(&cfg))),
            sink: Box::new(sink),
        };
        if engine.submit(job).is_err() {
            // its thread panicked: the next request starts a new one
            self.engines.lock().await.remove(descr);
            return Err(ApiError::Internal("the generation engine stopped".into()));
        }
        Ok(rx)
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

        let mut params = skelm_exec::ContextParams::default();
        if let Some(n_ctx) = self.options.ctx_size {
            params.n_ctx = n_ctx;
        }
        let with_options = |speculative| {
            model.clone().with_context_options(ContextOptions {
                params: params.clone(),
                lora: lora.clone(),
                speculative,
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
