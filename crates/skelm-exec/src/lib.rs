mod generate;
pub mod orchestrator;
mod output;
pub mod template;
mod toolparse;

use std::hash::Hash;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Mutex;
use std::{collections::HashMap, sync::Arc};

use thiserror::Error;

use skelm_llama_cpp::{self as llama, Vocab};
use skelm_ollama as ollama;

pub use generate::default_sampler;
pub use llama::{
    CacheType, ContextParams, DraftParams, Drafter, FlashAttn, LoadMode, LoraAdapter, LoraError,
    ModelParams, SpeculativeConfig, SpeculativeError, SpeculativeStats, SplitMode,
};
pub use orchestrator::{
    AgentId, GenBackend, HostEvent, Limits, LlamaGenBackend, Orchestrator, PendingId, ToolKind,
    ToolSpec,
};
pub use template::{Message, Tool, ToolCall, ToolCallFunction, ToolFunction, chat_template};
pub use toolparse::{ParsedOutput, parse_tool_calls};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum ModelDescr {
    Ollama(ollama::ModelDescr),
    Path(PathBuf),
    /// a GGUF model downloaded from the Hugging Face Hub, `hf.co/{owner}/{name}[:{quant}]`
    Hf(skelm_hf::HfModelRef),
}

impl ModelDescr {
    /// Parse a model name: `hf.co/{owner}/{name}[:{quant}]` for a Hugging Face
    /// model, the path of an existing GGUF file, or an Ollama name
    /// (`<model>:<variant>` / `<registry>/<model>:<variant>`)
    pub fn parse(name: &str) -> Result<Self, String> {
        if skelm_hf::HfModelRef::is_hf_name(name) {
            return name
                .parse()
                .map(ModelDescr::Hf)
                .map_err(|e: skelm_hf::HfError| e.to_string());
        }
        if std::path::Path::new(name).is_file() {
            return Ok(ModelDescr::Path(PathBuf::from(name)));
        }
        ollama::ModelDescr::from_str(name)
            .map(ModelDescr::Ollama)
            .map_err(|_| {
                format!(
                    "invalid model name '{name}': expected '<model>:<variant>', \
                     '<registry>/<model>:<variant>', 'hf.co/<owner>/<name>[:<quantization>]' \
                     or the path of a GGUF file"
                )
            })
    }
}

impl std::fmt::Display for ModelDescr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModelDescr::Ollama(descr) => descr.fmt(f),
            ModelDescr::Path(path) => path.display().fmt(f),
            ModelDescr::Hf(model) => model.fmt(f),
        }
    }
}

#[derive(Clone)]
pub struct Model {
    pub model: llama::Model,
    pub vocab: llama::Vocab,
    pub config: Arc<ModelConfig>,
    pub options: Arc<ContextOptions>,
}

/// How the contexts of a [`Model`] are set up, see [`Model::with_context_options`]
#[derive(Clone, Default)]
pub struct ContextOptions {
    /// the context parameters
    pub params: llama::ContextParams,
    /// LoRA adapters, with their scale, applied to every context
    pub lora: Vec<(llama::LoraAdapter, f32)>,
    /// speculative decoding, for the generation contexts (not embeddings)
    pub speculative: Option<llama::SpeculativeConfig>,
}

#[derive(Debug, Error)]
pub enum ContextError {
    #[error("failed to create the context: {0}")]
    Create(#[from] llama::ContextCreateError),
    #[error("failed to set up speculative decoding: {0}")]
    Speculative(#[from] llama::SpeculativeError),
    #[error("failed to apply the LoRA adapters: {0}")]
    Lora(#[from] llama::LoraError),
}

#[derive(Clone)]
pub enum ModelConfig {
    Ollama(ollama::ModelConfig),
    Implicit,
}

#[derive(Debug, Error)]
pub enum ModelLoadError {
    #[error("Llama model failed to load model")]
    LlamaModelFailedLoading(#[from] llama::ModelLoadError),
    #[error("Ollama config get error {0}")]
    OllamaConfigGetError(#[from] ollama::ModelConfigGetError),
    #[error("{0}")]
    HfError(#[from] skelm_hf::HfError),
}

impl Model {
    pub fn load(descr: &ModelDescr) -> Result<Self, ModelLoadError> {
        Self::load_with_params(descr, &llama::ModelParams::default())
    }

    pub fn load_with_params(
        descr: &ModelDescr,
        params: &llama::ModelParams,
    ) -> Result<Self, ModelLoadError> {
        let (config, model_path) = match descr {
            ModelDescr::Ollama(model_descr) => {
                let config = ollama::model_config_get(model_descr)?;
                let model_path = config.model_path.clone();
                (ModelConfig::Ollama(config), model_path)
            }
            ModelDescr::Path(path_buf) => (ModelConfig::Implicit, path_buf.clone()),
            ModelDescr::Hf(model) => (
                ModelConfig::Implicit,
                skelm_hf::HfCache::from_env().resolve(model)?,
            ),
        };
        llama::Model::load(model_path, params)
            .map_err(ModelLoadError::LlamaModelFailedLoading)
            .map(|m| Model {
                vocab: m.vocab(),
                model: m,
                config: Arc::new(config),
                options: Arc::default(),
            })
    }

    pub fn load_vocab(descr: &ModelDescr) -> Result<Vocab, ModelLoadError> {
        let (config, model_path) = match descr {
            ModelDescr::Ollama(model_descr) => {
                let config = ollama::model_config_get(model_descr)?;
                let model_path = config.model_path.clone();
                (ModelConfig::Ollama(config), model_path)
            }
            ModelDescr::Path(path_buf) => (ModelConfig::Implicit, path_buf.clone()),
            ModelDescr::Hf(model) => (
                ModelConfig::Implicit,
                skelm_hf::HfCache::from_env().resolve(model)?,
            ),
        };
        let params = llama::ModelParams {
            vocab_only: true,
            ..llama::ModelParams::default()
        };
        let model = llama::Model::load(model_path, &params)
            .map_err(ModelLoadError::LlamaModelFailedLoading)
            .map(|m| Model {
                vocab: m.vocab(),
                model: m,
                config: Arc::new(config),
                options: Arc::default(),
            })?;
        Ok(model.vocab)
    }

    /// Set how the contexts of this model are created: every later
    /// [`Model::new_context`] uses these parameters, LoRA adapters and
    /// speculative decoding. The speculative configuration is checked here.
    pub fn with_context_options(mut self, options: ContextOptions) -> Result<Self, ContextError> {
        if let Some(speculative) = &options.speculative {
            speculative.check(&self.model)?;
        }
        for (adapter, _) in &options.lora {
            if !adapter.model().same_as(&self.model) {
                return Err(llama::LoraError::ModelMismatch(adapter.path().into()).into());
            }
        }
        self.options = Arc::new(options);
        Ok(self)
    }

    /// Load a LoRA adapter for this model (apply it with [`Model::with_context_options`])
    pub fn load_lora(
        &self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<llama::LoraAdapter, llama::LoraError> {
        llama::LoraAdapter::load(&self.model, path)
    }

    /// A generation context, set up from the model's [`ContextOptions`]
    pub fn try_new_context(&self) -> Result<Context, ContextError> {
        self.try_new_context_with_params(&self.options.params)
    }

    /// Like [`Model::try_new_context`] with other context parameters
    pub fn try_new_context_with_params(
        &self,
        params: &llama::ContextParams,
    ) -> Result<Context, ContextError> {
        let mut ctx = match &self.options.speculative {
            Some(config) => llama::Context::new_speculative(self.model.clone(), params, config)?,
            None => self.model.new_context(params)?,
        };
        if !self.options.lora.is_empty() {
            ctx.set_lora_adapters(&self.options.lora)?;
        }
        Ok(Context(self.clone(), ctx))
    }

    /// Like [`Model::try_new_context`], panicking on failure
    pub fn new_context(&self) -> Context {
        self.try_new_context().unwrap()
    }

    /// Like [`Model::try_new_context_with_params`], panicking on failure
    pub fn new_context_with_params(&self, params: &llama::ContextParams) -> Context {
        self.try_new_context_with_params(params).unwrap()
    }

    pub fn new_context_embeddings(&self) -> Context {
        self.new_context_embeddings_with_params(&self.options.params)
    }

    /// Create an embeddings context; `params.embeddings` is forced on
    pub fn new_context_embeddings_with_params(&self, params: &llama::ContextParams) -> Context {
        if self.model.has_encoder() && self.model.has_decoder() {
            panic!("cannot generate embeddings in models with encoder-decoder")
        }
        let params = llama::ContextParams {
            embeddings: true,
            ..params.clone()
        };
        let mut ctx = self.model.new_context(&params).unwrap();
        if !self.options.lora.is_empty() {
            ctx.set_lora_adapters(&self.options.lora).unwrap();
        }
        Context(self.clone(), ctx)
    }

    pub fn model_template_render(&self, parameters: &ModelParameters) -> String {
        match self.config.as_ref() {
            ModelConfig::Ollama(_model_config) => implicit_model_template(self, parameters),
            ModelConfig::Implicit => implicit_model_template(self, parameters),
        }
    }
}

fn implicit_model_template(model: &Model, parameters: &ModelParameters) -> String {
    if let Some(template) = model.model.chat_template() {
        //println!("template:\n{}", template);
        match chat_template(
            &template,
            &parameters.messages,
            &parameters.tools,
            parameters.add_generation_prompt,
        ) {
            Err(e) => {
                eprintln!("rendering chat template failed: {}", e);
                eprintln!("chat template:");
                for (i, l) in template.lines().enumerate() {
                    eprintln!("{:03} {}", i + 1, l)
                }
                messages_fallback(&parameters.messages)
            }
            Ok(render) => {
                //println!("rendered:\n{}", render);
                render
            }
        }
    } else {
        messages_fallback(&parameters.messages)
    }
}

/// Fallback prompt for models without a chat template (e.g. base models): the
/// message contents concatenated in order.
fn messages_fallback(messages: &[Message]) -> String {
    messages
        .iter()
        .map(|m| m.content.as_str())
        .collect::<Vec<_>>()
        .join("\n")
}

#[derive(Clone)]
pub struct ModelParameters {
    /// The chat transcript rendered by the model's chat template, oldest first.
    /// Drives multi-turn conversations and tool round-trips (append the
    /// assistant's tool-call turn and the `tool` result turn, then re-render).
    pub messages: Vec<Message>,
    /// Tool definitions exposed to the chat template's `tools` variable. Empty
    /// when the model is run without tools.
    pub tools: Vec<Tool>,
    /// Whether to emit the trailing assistant header (the template's
    /// `add_generation_prompt` variable). True when generating a reply.
    pub add_generation_prompt: bool,
}

impl ModelParameters {
    /// Build parameters from an explicit chat transcript. `tools` defaults to
    /// empty and `add_generation_prompt` to true; set the fields to adjust.
    pub fn new(messages: Vec<Message>) -> Self {
        ModelParameters {
            messages,
            tools: Vec::new(),
            add_generation_prompt: true,
        }
    }

    /// Convenience for the common single-turn system + user request.
    pub fn single_turn(system: impl Into<String>, prompt: impl Into<String>) -> Self {
        ModelParameters::new(vec![Message::system(system), Message::user(prompt)])
    }
}

pub struct Context(pub Model, pub llama::Context);

impl Context {
    pub fn model(&self) -> &Model {
        &self.0
    }

    pub fn append_bytes(&mut self, bytes: &[u8]) {
        let mut tokens = self.0.vocab.tokenize(bytes, true);
        self.1.append_tokens(&mut tokens).unwrap();
    }
}

pub struct Models<K> {
    models: Arc<Mutex<HashMap<K, Model>>>,
}

impl<K> Clone for Models<K> {
    fn clone(&self) -> Self {
        Self {
            models: self.models.clone(),
        }
    }
}

impl<K: Hash + Eq> Models<K> {
    pub fn new() -> Self {
        Self {
            models: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn set(&self, key: K, model: Model) {
        let mut models = self.models.lock().unwrap();
        models.insert(key, model.clone());
    }

    pub fn get(&self, key: K) -> Option<Model> {
        let models = self.models.lock().unwrap();
        models.get(&key).map(|m| m.clone())
    }
}
