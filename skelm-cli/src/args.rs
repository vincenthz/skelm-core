use clap::{Parser, Subcommand};

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
    /// Pull an model by name
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
    },
    /// Bench model generation
    Bench {
        /// The name of the model to run
        name: String,
        #[arg(short, long)]
        max_tokens: Option<u64>,
    },
    /// Embedding generation
    Embed {
        /// The name of the model to run
        name: String,
    },
}
