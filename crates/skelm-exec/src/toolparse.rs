//! Parsing tool calls back out of a model's generated text.
//!
//! Different model families emit tool calls in different surface syntaxes; the
//! builtin (Jinja) chat templates that render tools (see [`crate::template`])
//! pair with one of these output conventions:
//!
//! * **Hermes / Qwen / Nous** — one or more `<tool_call>{json}</tool_call>`
//!   blocks, where `json` is `{"name": ..., "arguments": {...}}`.
//! * **Mistral** — a `[TOOL_CALLS]` marker followed by a JSON array of call
//!   objects.
//! * **Llama 3.x** — an optional `<|python_tag|>` marker followed by a JSON
//!   object using `{"name": ..., "parameters": {...}}`.
//!
//! [`parse_tool_calls`] tries each convention and returns the recovered calls
//! alongside the residual assistant text (the tool-call markup removed).

use serde_json::Value;

use crate::template::{ToolCall, ToolCallFunction};

/// The result of scanning model output for tool calls.
#[derive(Clone, Debug)]
pub struct ParsedOutput {
    /// Assistant text with any recognized tool-call markup removed.
    pub content: String,
    /// Tool calls recovered from the output, in order.
    pub tool_calls: Vec<ToolCall>,
}

impl ParsedOutput {
    fn plain(output: &str) -> Self {
        ParsedOutput {
            content: output.to_string(),
            tool_calls: Vec::new(),
        }
    }
}

/// Parse tool calls out of a model's generated `output`.
///
/// Recognizes the Hermes/Qwen, Mistral and Llama conventions (see the module
/// docs). When no tool call is found the output is returned verbatim as
/// [`ParsedOutput::content`] with an empty [`ParsedOutput::tool_calls`].
pub fn parse_tool_calls(output: &str) -> ParsedOutput {
    let parsers: &[fn(&str) -> Option<(Vec<ToolCall>, String)>] =
        &[parse_hermes, parse_mistral, parse_llama];

    for parser in parsers {
        if let Some((tool_calls, content)) = parser(output) {
            if !tool_calls.is_empty() {
                return ParsedOutput {
                    content,
                    tool_calls,
                };
            }
        }
    }
    ParsedOutput::plain(output)
}

/// Hermes / Qwen / Nous: `<tool_call>{json}</tool_call>` blocks, possibly many
/// and possibly interleaved with plain text.
fn parse_hermes(output: &str) -> Option<(Vec<ToolCall>, String)> {
    const OPEN: &str = "<tool_call>";
    const CLOSE: &str = "</tool_call>";

    if !output.contains(OPEN) {
        return None;
    }

    let mut calls = Vec::new();
    let mut content = String::new();
    let mut rest = output;

    while let Some(start) = rest.find(OPEN) {
        content.push_str(&rest[..start]);
        let after = &rest[start + OPEN.len()..];

        match after.find(CLOSE) {
            Some(end) => {
                let inner = &after[..end];
                match tool_call_from_str(inner) {
                    Some(tc) => calls.push(tc),
                    // Not valid JSON: keep the original block as text.
                    None => content.push_str(&rest[start..start + OPEN.len() + end + CLOSE.len()]),
                }
                rest = &after[end + CLOSE.len()..];
            }
            None => {
                // Unterminated block (e.g. generation hit the token limit).
                match tool_call_from_str(after) {
                    Some(tc) => calls.push(tc),
                    None => content.push_str(&rest[start..]),
                }
                rest = "";
            }
        }
    }
    content.push_str(rest);

    if calls.is_empty() {
        None
    } else {
        Some((calls, content.trim().to_string()))
    }
}

/// Mistral: `[TOOL_CALLS]` followed by a JSON array of call objects.
fn parse_mistral(output: &str) -> Option<(Vec<ToolCall>, String)> {
    const MARK: &str = "[TOOL_CALLS]";

    let idx = output.find(MARK)?;
    let before = &output[..idx];
    let after = output[idx + MARK.len()..].trim_start();

    // Read the first JSON value, tolerating any trailing tokens after it.
    let value = serde_json::Deserializer::from_str(after)
        .into_iter::<Value>()
        .next()?
        .ok()?;

    let array = value.as_array()?;
    let calls: Vec<ToolCall> = array.iter().filter_map(tool_call_from_json).collect();

    if calls.is_empty() {
        None
    } else {
        Some((calls, before.trim().to_string()))
    }
}

/// Llama 3.x: optional `<|python_tag|>` marker then a JSON object using the
/// `parameters` key. Without the marker we only accept a bare object that looks
/// unambiguously like a tool call (has both `name` and `parameters`/`arguments`)
/// so ordinary JSON answers aren't misread as calls.
fn parse_llama(output: &str) -> Option<(Vec<ToolCall>, String)> {
    const TAG: &str = "<|python_tag|>";

    let (before, body, had_tag) = match output.find(TAG) {
        Some(i) => (&output[..i], &output[i + TAG.len()..], true),
        None => ("", output, false),
    };

    let trimmed = body.trim();
    if !trimmed.starts_with('{') {
        return None;
    }

    let mut calls = Vec::new();
    for value in serde_json::Deserializer::from_str(trimmed).into_iter::<Value>() {
        let Ok(value) = value else { break };
        let Some(obj) = value.as_object() else {
            return None;
        };
        let looks_like_call = obj.contains_key("name")
            && (had_tag || obj.contains_key("parameters") || obj.contains_key("arguments"));
        if !looks_like_call {
            return None;
        }
        if let Some(tc) = tool_call_from_json(&value) {
            calls.push(tc);
        }
    }

    if calls.is_empty() {
        None
    } else {
        Some((calls, before.trim().to_string()))
    }
}

fn tool_call_from_str(s: &str) -> Option<ToolCall> {
    let s = s.trim();
    // Some chat templates (notably several Qwen2.5 GGUFs) print the tool-call
    // example with doubled braces — `{{"name": ...}}` — so the model dutifully
    // emits doubled braces, which is invalid JSON. Fall back to stripping one
    // outer brace layer, the same leniency llama.cpp applies.
    let value: Value = serde_json::from_str(s)
        .or_else(|_| serde_json::from_str(&undouble_outer_braces(s)))
        .ok()?;
    tool_call_from_json(&value)
}

/// If `s` is wrapped in a redundant extra `{{ … }}` brace layer, strip one level.
/// A well-formed JSON object never starts with `{{`, so this only ever rescues
/// the malformed doubled-brace form.
fn undouble_outer_braces(s: &str) -> String {
    let t = s.trim();
    if t.len() >= 4 && t.starts_with("{{") && t.ends_with("}}") {
        // The braces are ASCII, so these byte indices are char boundaries.
        t[1..t.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

/// Build a [`ToolCall`] from a `{"name": ..., "arguments"|"parameters": ...}`
/// object. Accepts either key for the arguments and normalizes a stringified
/// JSON arguments payload into structured JSON.
fn tool_call_from_json(value: &Value) -> Option<ToolCall> {
    let obj = value.as_object()?;
    let name = obj.get("name")?.as_str()?.to_string();
    let arguments = obj
        .get("arguments")
        .or_else(|| obj.get("parameters"))
        .cloned()
        .map(normalize_arguments)
        .unwrap_or_else(|| Value::Object(Default::default()));

    Some(ToolCall {
        kind: "function".to_string(),
        function: ToolCallFunction { name, arguments },
    })
}

/// Some models emit `arguments` as a JSON-encoded string rather than an object;
/// decode it when possible so callers always see structured arguments.
fn normalize_arguments(value: Value) -> Value {
    match value {
        Value::String(s) => serde_json::from_str(&s).unwrap_or(Value::String(s)),
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(tc: &ToolCall) -> &Value {
        &tc.function.arguments
    }

    #[test]
    fn hermes_single() {
        let out = "Let me check.\n<tool_call>\n{\"name\": \"get_weather\", \"arguments\": {\"city\": \"Paris\"}}\n</tool_call>";
        let parsed = parse_tool_calls(out);
        assert_eq!(parsed.tool_calls.len(), 1);
        assert_eq!(parsed.tool_calls[0].function.name, "get_weather");
        assert_eq!(args(&parsed.tool_calls[0])["city"], "Paris");
        assert_eq!(parsed.content, "Let me check.");
    }

    #[test]
    fn hermes_multiple() {
        let out = "<tool_call>{\"name\":\"a\",\"arguments\":{}}</tool_call><tool_call>{\"name\":\"b\",\"arguments\":{\"x\":1}}</tool_call>";
        let parsed = parse_tool_calls(out);
        assert_eq!(parsed.tool_calls.len(), 2);
        assert_eq!(parsed.tool_calls[0].function.name, "a");
        assert_eq!(parsed.tool_calls[1].function.name, "b");
        assert_eq!(args(&parsed.tool_calls[1])["x"], 1);
    }

    #[test]
    fn hermes_doubled_outer_braces() {
        // Several Qwen2.5 GGUF templates render the example with doubled braces,
        // so the model emits `{{ ... }}`; we must still recover the call.
        let out = "<tool_call>\n{{\"name\": \"delegate\", \"arguments\": {\"task\": \"do it\"}}}\n</tool_call>";
        let parsed = parse_tool_calls(out);
        assert_eq!(parsed.tool_calls.len(), 1);
        assert_eq!(parsed.tool_calls[0].function.name, "delegate");
        assert_eq!(args(&parsed.tool_calls[0])["task"], "do it");
    }

    #[test]
    fn hermes_unterminated() {
        let out = "<tool_call>{\"name\":\"a\",\"arguments\":{}}";
        let parsed = parse_tool_calls(out);
        assert_eq!(parsed.tool_calls.len(), 1);
        assert_eq!(parsed.tool_calls[0].function.name, "a");
    }

    #[test]
    fn mistral_array() {
        let out = "[TOOL_CALLS][{\"name\": \"get_weather\", \"arguments\": {\"city\": \"Paris\"}}]";
        let parsed = parse_tool_calls(out);
        assert_eq!(parsed.tool_calls.len(), 1);
        assert_eq!(parsed.tool_calls[0].function.name, "get_weather");
        assert_eq!(args(&parsed.tool_calls[0])["city"], "Paris");
    }

    #[test]
    fn llama_python_tag() {
        let out = "<|python_tag|>{\"name\": \"get_weather\", \"parameters\": {\"city\": \"Paris\"}}";
        let parsed = parse_tool_calls(out);
        assert_eq!(parsed.tool_calls.len(), 1);
        assert_eq!(parsed.tool_calls[0].function.name, "get_weather");
        assert_eq!(args(&parsed.tool_calls[0])["city"], "Paris");
    }

    #[test]
    fn llama_bare_object() {
        let out = "{\"name\": \"get_weather\", \"parameters\": {\"city\": \"Paris\"}}";
        let parsed = parse_tool_calls(out);
        assert_eq!(parsed.tool_calls.len(), 1);
        assert_eq!(parsed.tool_calls[0].function.name, "get_weather");
    }

    #[test]
    fn stringified_arguments_are_decoded() {
        let out = "<tool_call>{\"name\":\"a\",\"arguments\":\"{\\\"city\\\":\\\"Paris\\\"}\"}</tool_call>";
        let parsed = parse_tool_calls(out);
        assert_eq!(parsed.tool_calls.len(), 1);
        assert_eq!(args(&parsed.tool_calls[0])["city"], "Paris");
    }

    #[test]
    fn plain_json_answer_is_not_a_call() {
        // A bare object without a `name` (or without params/arguments) must not
        // be misread as a tool call.
        let out = "{\"answer\": 42}";
        let parsed = parse_tool_calls(out);
        assert!(parsed.tool_calls.is_empty());
        assert_eq!(parsed.content, out);
    }

    #[test]
    fn plain_text_is_passthrough() {
        let out = "The weather in Paris is sunny.";
        let parsed = parse_tool_calls(out);
        assert!(parsed.tool_calls.is_empty());
        assert_eq!(parsed.content, out);
    }
}
