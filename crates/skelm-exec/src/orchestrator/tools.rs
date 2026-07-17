//! Tools an orchestrated agent can call.
//!
//! Besides the built-in [`delegate`](DELEGATE_TOOL) tool (handled by the
//! scheduler itself), the host registers [`ToolSpec`]s. Each spec pairs the tool
//! definition rendered into the chat template with how its result is obtained:
//! synchronously on a worker thread ([`ToolKind::Sync`]) or out-of-band by the
//! host ([`ToolKind::Deferred`] — external services, humans).

use serde_json::{Value, json};

use crate::template::{Tool, ToolCall};

/// Name of the built-in delegation tool. Calls to it are intercepted by the
/// scheduler and turned into a fresh sub-agent rather than dispatched like a
/// host tool.
pub const DELEGATE_TOOL: &str = "delegate";

/// How the orchestrator obtains a tool call's result.
pub enum ToolKind {
    /// Runs synchronously on a worker thread; the closure returns the result
    /// string (or an error, which is reported back to the model as the result).
    Sync(Box<dyn Fn(&ToolCall) -> anyhow::Result<String> + Send + Sync>),
    /// The result is provided out-of-band by the host: the call surfaces as a
    /// [`HostEvent::NeedInput`](crate::orchestrator::HostEvent::NeedInput) and is
    /// resolved later via [`Orchestrator::fulfill`](crate::orchestrator::Orchestrator::fulfill).
    /// Use for external async services and human-in-the-loop.
    Deferred,
}

/// A tool exposed to agents: its definition (rendered into the chat template's
/// `tools` variable) plus how its result is obtained.
pub struct ToolSpec {
    pub def: Tool,
    pub kind: ToolKind,
}

impl ToolSpec {
    /// A tool whose result is produced synchronously by `f` on a worker thread.
    pub fn sync(
        def: Tool,
        f: impl Fn(&ToolCall) -> anyhow::Result<String> + Send + Sync + 'static,
    ) -> Self {
        ToolSpec {
            def,
            kind: ToolKind::Sync(Box::new(f)),
        }
    }

    /// A tool whose result the host provides later via
    /// [`Orchestrator::fulfill`](crate::orchestrator::Orchestrator::fulfill).
    pub fn deferred(def: Tool) -> Self {
        ToolSpec {
            def,
            kind: ToolKind::Deferred,
        }
    }
}

/// The definition of the built-in [`delegate`](DELEGATE_TOOL) tool. Injected into
/// every agent's tool set (subject to the depth cap) so the model can fan work
/// out to sub-agents.
pub fn delegate_tool_def() -> Tool {
    Tool::function(
        DELEGATE_TOOL,
        Some(
            "Delegate a subtask to a fresh specialist sub-agent and wait for its \
             answer. Use this to split a problem into independent pieces or to \
             consult a focused expert; the sub-agent starts with no memory of this \
             conversation, so give it everything it needs in `task`."
                .to_string(),
        ),
        json!({
            "type": "object",
            "properties": {
                "role": {
                    "type": "string",
                    "description": "Optional system prompt describing the sub-agent's role or expertise."
                },
                "task": {
                    "type": "string",
                    "description": "The self-contained task or question for the sub-agent to work on."
                }
            },
            "required": ["task"]
        }),
    )
}

/// Extract `(role, task)` from a `delegate` call's arguments. `role` falls back
/// to a generic assistant prompt; `task` falls back to the raw arguments so a
/// malformed call still gives the sub-agent something to act on.
pub(crate) fn parse_delegate_args(call: &ToolCall) -> (String, String) {
    let args = &call.function.arguments;
    let role = str_field(args, "role")
        .unwrap_or_else(|| "You are a focused, helpful assistant.".to_string());
    let task = str_field(args, "task").unwrap_or_else(|| args.to_string());
    (role, task)
}

fn str_field(args: &Value, key: &str) -> Option<String> {
    args.get(key)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}
