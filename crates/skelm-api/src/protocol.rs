//! OpenAI-compatible request and response types.
//!
//! These mirror the shapes documented at <https://platform.openai.com/docs/api-reference>
//! closely enough that off-the-shelf OpenAI clients (the `openai` Python SDK,
//! LangChain, `curl` recipes, …) talk to skelm unmodified. Only the fields skelm
//! can act on are honored; unsupported knobs are accepted and ignored so requests
//! from feature-rich clients don't fail (see the per-field notes).

use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use ulid::Ulid;

use skelm_exec::{Message, Tool, ToolCall, ToolCallFunction};

/// Seconds since the Unix epoch, for the `created` field.
pub fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// A response id of the form `<prefix>-<ulid>` (e.g. `chatcmpl-01J…`).
pub fn new_id(prefix: &str) -> String {
    format!("{prefix}-{}", Ulid::new())
}

// ---------------------------------------------------------------------------
// Shared request fragments
// ---------------------------------------------------------------------------

/// `stop` may be a single string or an array of strings.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum StopField {
    One(String),
    Many(Vec<String>),
}

/// Normalize an optional `stop` field into a plain list.
pub fn stop_vec(stop: Option<StopField>) -> Vec<String> {
    match stop {
        None => Vec::new(),
        Some(StopField::One(s)) => vec![s],
        Some(StopField::Many(v)) => v,
    }
}

/// Message `content`: either a plain string or an array of typed parts
/// (the multimodal shape). Only text parts are understood; anything else is
/// dropped when flattening to text.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum MessageContent {
    Text(String),
    Parts(Vec<ContentPart>),
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContentPart {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub text: Option<String>,
}

impl MessageContent {
    pub fn into_text(self) -> String {
        match self {
            MessageContent::Text(t) => t,
            MessageContent::Parts(parts) => parts
                .into_iter()
                .filter_map(|p| p.text)
                .collect::<Vec<_>>()
                .join(""),
        }
    }
}

// ---------------------------------------------------------------------------
// Chat completions
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<RequestMessage>,
    #[serde(default)]
    pub temperature: Option<f32>,
    /// Accepted for compatibility; skelm has no top-p sampler, so it is ignored.
    #[serde(default)]
    pub top_p: Option<f32>,
    #[serde(default, alias = "max_completion_tokens")]
    pub max_tokens: Option<usize>,
    #[serde(default)]
    pub seed: Option<u32>,
    #[serde(default)]
    pub stop: Option<StopField>,
    #[serde(default)]
    pub stream: bool,
    /// Tool definitions exposed to the model's chat template. Uses the same
    /// OpenAI `{"type":"function","function":{…}}` shape skelm renders natively.
    #[serde(default)]
    pub tools: Option<Vec<Tool>>,
    /// Accepted but ignored: skelm always returns a single choice.
    #[serde(default)]
    pub n: Option<u32>,
    #[serde(default)]
    pub user: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RequestMessage {
    pub role: String,
    #[serde(default)]
    pub content: Option<MessageContent>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub tool_calls: Option<Vec<RequestToolCall>>,
    #[serde(default)]
    pub tool_call_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RequestToolCall {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(rename = "type", default)]
    pub kind: Option<String>,
    pub function: RequestToolCallFunction,
}

#[derive(Debug, Deserialize)]
pub struct RequestToolCallFunction {
    pub name: String,
    /// OpenAI encodes arguments as a JSON *string*; we decode it back to a value.
    #[serde(default)]
    pub arguments: String,
}

/// Decode an OpenAI arguments string into structured JSON, tolerating an empty
/// string (→ `{}`) and non-JSON text (→ a JSON string).
fn decode_arguments(arguments: String) -> Value {
    if arguments.trim().is_empty() {
        return Value::Object(Default::default());
    }
    match serde_json::from_str::<Value>(&arguments) {
        Ok(v) => v,
        Err(_) => Value::String(arguments),
    }
}

impl RequestToolCall {
    fn into_exec(self) -> ToolCall {
        ToolCall {
            kind: self.kind.unwrap_or_else(|| "function".to_string()),
            function: ToolCallFunction {
                name: self.function.name,
                arguments: decode_arguments(self.function.arguments),
            },
        }
    }
}

/// Convert incoming OpenAI chat messages into the transcript skelm renders.
///
/// Assistant messages carrying `tool_calls` and `tool` result messages are
/// reconstructed so multi-turn tool conversations round-trip through the chat
/// template.
pub fn to_exec_messages(messages: Vec<RequestMessage>) -> Vec<Message> {
    messages
        .into_iter()
        .map(|m| {
            let content = m.content.map(MessageContent::into_text).unwrap_or_default();
            match m.role.as_str() {
                "assistant" if m.tool_calls.is_some() => {
                    let calls = m
                        .tool_calls
                        .unwrap()
                        .into_iter()
                        .map(RequestToolCall::into_exec)
                        .collect();
                    Message::assistant_tool_calls(content, calls)
                }
                "tool" => {
                    let name = m.name.or(m.tool_call_id).unwrap_or_default();
                    Message::tool(name, content)
                }
                other => Message::new(other, content),
            }
        })
        .collect()
}

#[derive(Debug, Serialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: &'static str,
    pub created: u64,
    pub model: String,
    pub choices: Vec<ChatChoice>,
    pub usage: Usage,
}

#[derive(Debug, Serialize)]
pub struct ChatChoice {
    pub index: u32,
    pub message: ResponseMessage,
    pub finish_reason: &'static str,
}

#[derive(Debug, Serialize)]
pub struct ResponseMessage {
    pub role: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ResponseToolCall>>,
}

#[derive(Debug, Serialize)]
pub struct ResponseToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: &'static str,
    pub function: ResponseToolCallFunction,
}

#[derive(Debug, Serialize)]
pub struct ResponseToolCallFunction {
    pub name: String,
    /// Serialized back to the OpenAI JSON-string form.
    pub arguments: String,
}

fn arguments_to_string(arguments: &Value) -> String {
    serde_json::to_string(arguments).unwrap_or_else(|_| "{}".to_string())
}

/// Turn parsed tool calls into the OpenAI response shape (arguments re-encoded
/// as a JSON string, each call given a synthetic id).
pub fn to_response_tool_calls(calls: Vec<ToolCall>) -> Vec<ResponseToolCall> {
    calls
        .into_iter()
        .map(|tc| ResponseToolCall {
            id: new_id("call"),
            kind: "function",
            function: ResponseToolCallFunction {
                name: tc.function.name,
                arguments: arguments_to_string(&tc.function.arguments),
            },
        })
        .collect()
}

#[derive(Debug, Serialize)]
pub struct Usage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_tokens_details: Option<PromptTokensDetails>,
}

impl Usage {
    pub fn new(prompt_tokens: usize, cached_tokens: usize, completion_tokens: usize) -> Self {
        Usage {
            prompt_tokens,
            completion_tokens,
            total_tokens: prompt_tokens + completion_tokens,
            prompt_tokens_details: Some(PromptTokensDetails { cached_tokens }),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PromptTokensDetails {
    /// prompt tokens reused from the server's cache, not processed again
    pub cached_tokens: usize,
}

// --- streaming chunks ---

#[derive(Debug, Serialize)]
pub struct ChatCompletionChunk {
    pub id: String,
    pub object: &'static str,
    pub created: u64,
    pub model: String,
    pub choices: Vec<ChunkChoice>,
}

#[derive(Debug, Serialize)]
pub struct ChunkChoice {
    pub index: u32,
    pub delta: ChunkDelta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<&'static str>,
}

#[derive(Debug, Default, Serialize)]
pub struct ChunkDelta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ChunkToolCall>>,
}

#[derive(Debug, Serialize)]
pub struct ChunkToolCall {
    pub index: u32,
    pub id: String,
    #[serde(rename = "type")]
    pub kind: &'static str,
    pub function: ResponseToolCallFunction,
}

pub fn to_chunk_tool_calls(calls: Vec<ToolCall>) -> Vec<ChunkToolCall> {
    calls
        .into_iter()
        .enumerate()
        .map(|(i, tc)| ChunkToolCall {
            index: i as u32,
            id: new_id("call"),
            kind: "function",
            function: ResponseToolCallFunction {
                name: tc.function.name,
                arguments: arguments_to_string(&tc.function.arguments),
            },
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Legacy text completions
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum PromptField {
    One(String),
    Many(Vec<String>),
}

impl PromptField {
    /// The prompt text. Arrays (batch prompts) are not supported; the first
    /// element is used.
    pub fn into_text(self) -> String {
        match self {
            PromptField::One(s) => s,
            PromptField::Many(v) => v.into_iter().next().unwrap_or_default(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CompletionRequest {
    pub model: String,
    pub prompt: PromptField,
    #[serde(default)]
    pub temperature: Option<f32>,
    #[serde(default)]
    pub top_p: Option<f32>,
    #[serde(default)]
    pub max_tokens: Option<usize>,
    #[serde(default)]
    pub seed: Option<u32>,
    #[serde(default)]
    pub stop: Option<StopField>,
    #[serde(default)]
    pub stream: bool,
    #[serde(default)]
    pub n: Option<u32>,
    #[serde(default)]
    pub user: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CompletionResponse {
    pub id: String,
    pub object: &'static str,
    pub created: u64,
    pub model: String,
    pub choices: Vec<CompletionChoice>,
    pub usage: Usage,
}

#[derive(Debug, Serialize)]
pub struct CompletionChoice {
    pub index: u32,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<&'static str>,
    /// Always null: skelm does not expose logprobs.
    pub logprobs: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct CompletionChunk {
    pub id: String,
    pub object: &'static str,
    pub created: u64,
    pub model: String,
    pub choices: Vec<CompletionChoice>,
}

// ---------------------------------------------------------------------------
// Embeddings
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum EmbeddingInput {
    Single(String),
    Multiple(Vec<String>),
}

impl EmbeddingInput {
    pub fn into_vec(self) -> Vec<String> {
        match self {
            EmbeddingInput::Single(s) => vec![s],
            EmbeddingInput::Multiple(v) => v,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct EmbeddingRequest {
    pub model: String,
    pub input: EmbeddingInput,
    /// Accepted for compatibility; skelm returns `float` embeddings only.
    #[serde(default)]
    pub encoding_format: Option<String>,
    #[serde(default)]
    pub user: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EmbeddingResponse {
    pub object: &'static str,
    pub data: Vec<EmbeddingData>,
    pub model: String,
    pub usage: EmbeddingUsage,
}

#[derive(Debug, Serialize)]
pub struct EmbeddingData {
    pub object: &'static str,
    pub embedding: Vec<f32>,
    pub index: usize,
}

#[derive(Debug, Serialize)]
pub struct EmbeddingUsage {
    pub prompt_tokens: usize,
    pub total_tokens: usize,
}

// ---------------------------------------------------------------------------
// Models
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize)]
pub struct ModelObject {
    pub id: String,
    pub object: &'static str,
    pub created: u64,
    pub owned_by: &'static str,
}

impl ModelObject {
    pub fn new(id: String) -> Self {
        ModelObject {
            id,
            object: "model",
            created: 0,
            owned_by: "skelm",
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ModelList {
    pub object: &'static str,
    pub data: Vec<ModelObject>,
}
