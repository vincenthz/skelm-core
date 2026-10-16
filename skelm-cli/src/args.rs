use clap::{Args, Parser, Subcommand};

/// skelm — manage and run local LLMs.
#[derive(Parser, Debug)]
#[command(name = "skelm")]
#[command(about = "Manage and run local LLM models", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List available model
    List {
        /// Optional filter for the list
        #[arg(short, long)]
        filter: Option<String>,
    },
    /// Pull a model by name: `<model>:<variant>` from the Ollama registry, or
    /// `hf.co/<owner>/<name>[:<quantization>]` from Hugging Face
    Pull {
        /// The name of the model to pull
        name: String,
    },
    Set {
        /// The name of the model
        name: String,
        /// The key to replace
        key: String,
        /// The value to use (can be a filepath)
        value: String,
    },
    /// Remove a model by name
    Remove {
        /// The name of the model to remove
        name: String,
    },
    /// Verify install
    Verify {
        /// Flag to verify blobs (might take a long time)
        #[arg(short, long, default_value_t = false)]
        blobs: bool,
    },
    /// Information about a model
    Info {
        /// The name of the model to get info
        name: String,
    },
    /// Run a model
    Run {
        /// The name of the model to run
        name: String,
        /// Debug information
        #[arg(long, default_value_t = false)]
        debug: bool,
        /// Use model path directly (no ollama)
        #[arg(long, default_value_t = false)]
        model_path: bool,
        /// Use model path directly (no ollama)
        #[arg(long)]
        system: Option<String>,
        /// User input file
        #[arg(long)]
        input: Option<String>,
        /// JSON file with tool definitions (OpenAI function format) exposed to
        /// the chat template's `tools` variable
        #[arg(long)]
        tools: Option<String>,
        /// Program that executes tool calls: receives `{"name","arguments"}`
        /// JSON on stdin, returns the result on stdout. Enables the tool loop.
        #[arg(long)]
        tool_exec: Option<String>,
        /// Maximum number of tool-call rounds before giving up
        #[arg(long, default_value_t = 8)]
        max_tool_iters: usize,
        /// Don't ask for a prompt
        #[arg(long, default_value_t = false)]
        no_prompt: bool,
        #[arg(long)]
        output: Option<String>,
        #[command(flatten)]
        options: ModelOptions,
    },
    /// Run a model as a multi-agent orchestrator: the model can fan work out to
    /// sub-agents via the built-in `delegate` tool, under a step budget.
    Orchestrate {
        /// The name of the model to run
        name: String,
        /// Debug information
        #[arg(long, default_value_t = false)]
        debug: bool,
        /// Use model path directly (no ollama)
        #[arg(long, default_value_t = false)]
        model_path: bool,
        /// System prompt for the root agent
        #[arg(long)]
        system: Option<String>,
        /// File holding the root task/prompt
        #[arg(long)]
        input: Option<String>,
        /// JSON file with tool definitions (OpenAI function format) offered to
        /// every agent alongside `delegate`
        #[arg(long)]
        tools: Option<String>,
        /// Program that executes the `--tools` (same protocol as `run`): receives
        /// `{"name","arguments"}` on stdin, returns the result on stdout
        #[arg(long)]
        tool_exec: Option<String>,
        /// Total generation-step budget across all agents
        #[arg(long, default_value_t = 16)]
        max_steps: usize,
        /// Maximum number of agents (including the root)
        #[arg(long, default_value_t = 8)]
        max_agents: usize,
        /// Maximum delegation depth (root is depth 0)
        #[arg(long, default_value_t = 3)]
        max_depth: usize,
        #[command(flatten)]
        options: ModelOptions,
    },
    /// Serve an OpenAI-compatible HTTP API (chat/completions, completions,
    /// embeddings and models) backed by locally installed models.
    Serve {
        /// Address to bind on
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        /// Port to listen on
        #[arg(short, long, default_value_t = 11434)]
        port: u16,
        /// Applied to every model the server loads (models without MTP layers,
        /// or incompatible with the draft model, decode normally)
        #[command(flatten)]
        options: ModelOptions,
    },
    /// Bench model generation
    Bench {
        /// The name of the model to run
        name: String,
        #[arg(short, long)]
        max_tokens: Option<u64>,
        #[command(flatten)]
        options: ModelOptions,
    },
    /// Embedding generation
    Embed {
        /// The name of the model to run
        name: String,
    },
    /// Hugging Face Hub: search GGUF models, list a repository's
    /// quantizations, and pull them (into the Hugging Face cache)
    Hf {
        #[command(subcommand)]
        command: HfCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum HfCommands {
    /// Search model repositories (with GGUF files, unless --all)
    Search {
        /// Text to look for in repository names
        query: Option<String>,
        /// Only the repositories of this user or organization
        #[arg(long)]
        author: Option<String>,
        /// Maximum number of results
        #[arg(short = 'n', long, default_value_t = 20)]
        limit: usize,
        /// Order of the results
        #[arg(long, value_enum, default_value_t = HfSort::Downloads)]
        sort: HfSort,
        /// Include repositories without GGUF files
        #[arg(long)]
        all: bool,
    },
    /// List the GGUF quantizations of a repository
    Files {
        /// The repository, `<owner>/<name>` (an `hf.co/` prefix is accepted)
        repo: String,
    },
    /// Pull a GGUF model, like `skelm pull hf.co/<owner>/<name>[:<quantization>]`
    Pull {
        /// `<owner>/<name>[:<quantization>]` (an `hf.co/` prefix is accepted)
        name: String,
    },
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum HfSort {
    Downloads,
    Likes,
    Trending,
    Updated,
}

/// LoRA adapters and speculative decoding
#[derive(Args, Debug, Clone, Default)]
pub struct ModelOptions {
    /// LoRA adapter (GGUF file) to apply, as `PATH` or `PATH:SCALE` (default
    /// scale 1.0); repeat for several adapters
    #[arg(long = "lora", value_name = "PATH[:SCALE]")]
    pub lora: Vec<String>,
    /// Speculative decoding with a draft model: a smaller model sharing the
    /// model's vocabulary, as a model name or a GGUF path
    #[arg(long, value_name = "MODEL")]
    pub draft_model: Option<String>,
    /// Speculative decoding with the model's own MTP (multi-token prediction)
    /// layers, for models that have them (Qwen3.5, Qwen3-Next, GLM-4.5, ...)
    #[arg(long, conflicts_with = "draft_model")]
    pub mtp: bool,
    /// Maximum number of tokens drafted per speculative round
    #[arg(long, default_value_t = 3)]
    pub draft_max: usize,
    /// Minimum number of drafted tokens for a draft to be verified
    #[arg(long, default_value_t = 0)]
    pub draft_min: usize,
    /// Stop drafting when the drafter's confidence drops below this probability
    #[arg(long, default_value_t = 0.0)]
    pub draft_p_min: f32,
}
