// The hand-written `template-jinja` engine is disabled for now (kept in the
// tree for later use). Chat-template rendering currently goes through minijinja.

use minijinja::{Value, context};
use serde::{Deserialize, Serialize};

/// A tool/function definition exposed to a chat template via the `tools`
/// variable.
///
/// Serializes to the OpenAI-style shape that the builtin (Jinja) chat templates
/// shipped inside GGUF models expect:
/// `{"type": "function", "function": {"name", "description", "parameters"}}`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tool {
    #[serde(rename = "type")]
    pub kind: String,
    pub function: ToolFunction,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolFunction {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// JSON-schema object describing the function parameters.
    pub parameters: serde_json::Value,
}

impl Tool {
    /// Build a `function`-kind tool from its name, optional description and a
    /// JSON-schema parameters object.
    pub fn function(
        name: impl Into<String>,
        description: Option<String>,
        parameters: serde_json::Value,
    ) -> Self {
        Tool {
            kind: "function".to_string(),
            function: ToolFunction {
                name: name.into(),
                description,
                parameters,
            },
        }
    }
}

/// A tool call requested by the assistant, carried on an assistant [`Message`].
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolCall {
    #[serde(rename = "type")]
    pub kind: String,
    pub function: ToolCallFunction,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolCallFunction {
    pub name: String,
    pub arguments: serde_json::Value,
}

/// A chat message.
///
/// `tool_calls` is populated on assistant messages that request tools; `name`
/// identifies the answering tool on `role = "tool"` response messages. Both are
/// omitted from the rendered context when absent so templates that don't expect
/// them are unaffected.
#[derive(Clone, Debug, Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Message {
    pub fn new(role: impl Into<String>, content: impl Into<String>) -> Self {
        Message {
            role: role.into(),
            content: content.into(),
            tool_calls: None,
            name: None,
        }
    }

    pub fn system(content: impl Into<String>) -> Self {
        Message::new("system", content)
    }

    pub fn user(content: impl Into<String>) -> Self {
        Message::new("user", content)
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Message::new("assistant", content)
    }

    /// An assistant message that requests one or more tool calls.
    pub fn assistant_tool_calls(content: impl Into<String>, tool_calls: Vec<ToolCall>) -> Self {
        Message {
            role: "assistant".to_string(),
            content: content.into(),
            tool_calls: Some(tool_calls),
            name: None,
        }
    }

    /// A tool-result message answering the tool named `name`.
    pub fn tool(name: impl Into<String>, content: impl Into<String>) -> Self {
        Message {
            role: "tool".to_string(),
            content: content.into(),
            tool_calls: None,
            name: Some(name.into()),
        }
    }
}

fn raise_exception(err_text: String) -> Result<String, minijinja::Error> {
    Err(minijinja::Error::new(
        minijinja::ErrorKind::SyntaxError,
        err_text,
    ))
}

fn strftime_now(format: String) -> Result<String, minijinja::Error> {
    Ok(chrono::Utc::now().format(&format).to_string())
}

/// Render a builtin (Jinja) chat template with the given messages and tools.
///
/// `tools` is exposed to the template as the `tools` variable; an empty slice
/// serializes to an empty sequence, which is falsy, so templates gated on
/// `{% if tools %}` correctly render no tool section. `add_generation_prompt`
/// maps to the same-named variable most templates use to decide whether to emit
/// the trailing assistant header.
pub fn chat_template(
    template: &str,
    messages: &[Message],
    tools: &[Tool],
    add_generation_prompt: bool,
) -> Result<String, String> {
    let mut env = minijinja::Environment::new();
    minijinja_contrib::add_to_environment(&mut env);

    env.set_unknown_method_callback(minijinja_contrib::pycompat::unknown_method_callback);
    env.add_function("raise_exception", raise_exception);
    env.add_function("strftime_now", strftime_now);

    const MAIN: &str = "main";

    env.add_template(MAIN, template)
        .map_err(|e| format!("chat template parse error: {}", e))?;

    let tmpl = env
        .get_template(MAIN)
        .map_err(|e| format!("chat template load error: {}", e))?;

    let ctx = context! {
        messages => Value::from_serialize(messages),
        tools => Value::from_serialize(tools),
        add_generation_prompt => add_generation_prompt,
    };

    tmpl.render(ctx)
        .map_err(|e| format!("chat template render error: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    // A representative tool-aware chat template in the same idiom as the builtin
    // Jinja templates shipped in GGUF models (Hermes/Qwen style).
    const TOOL_TEMPLATE: &str = r#"
{%- if tools %}
<tools>
{%- for tool in tools %}
{{ tool.function.name }}: {{ tool.function.description }} | {{ tool | tojson }}
{%- endfor %}
</tools>
{%- endif %}
{%- for message in messages %}
<|{{ message.role }}|>
{{ message.content }}
{%- endfor %}
{%- if add_generation_prompt %}
<|assistant|>
{%- endif %}"#;

    fn weather_tool() -> Tool {
        Tool::function(
            "get_weather",
            Some("Get the weather for a city".to_string()),
            serde_json::json!({
                "type": "object",
                "properties": { "city": { "type": "string" } },
                "required": ["city"]
            }),
        )
    }

    #[test]
    fn renders_tools_section() {
        let messages = vec![Message::system("be helpful"), Message::user("weather in Paris?")];
        let out = chat_template(TOOL_TEMPLATE, &messages, &[weather_tool()], true).unwrap();

        // tool definition surfaced to the template
        assert!(out.contains("<tools>"), "missing tools section:\n{out}");
        assert!(out.contains("get_weather: Get the weather for a city"));
        // `tojson` produced the OpenAI-style shape
        assert!(out.contains(r#""type":"function""#), "tojson shape wrong:\n{out}");
        assert!(out.contains(r#""name":"get_weather""#));
        // messages and generation prompt both render
        assert!(out.contains("<|system|>"));
        assert!(out.contains("weather in Paris?"));
        assert!(out.trim_end().ends_with("<|assistant|>"));
    }

    #[test]
    fn empty_tools_is_falsy() {
        let messages = vec![Message::user("hi")];
        let out = chat_template(TOOL_TEMPLATE, &messages, &[], true).unwrap();
        assert!(!out.contains("<tools>"), "empty tools should render no section:\n{out}");
    }

    #[test]
    fn no_generation_prompt() {
        let messages = vec![Message::user("hi")];
        let out = chat_template(TOOL_TEMPLATE, &messages, &[], false).unwrap();
        assert!(!out.contains("<|assistant|>"));
    }

    #[test]
    fn strftime_now_honors_format() {
        // Llama-3.x tool templates call strftime_now("%d %b %Y"); make sure the
        // format string is actually applied rather than ignored.
        let out = chat_template(
            r#"{{ strftime_now("%Y") }}"#,
            &[Message::user("x")],
            &[],
            false,
        )
        .unwrap();
        let year: i32 = out.trim().parse().expect("year should be numeric");
        assert!(year >= 2024, "unexpected year: {out}");
    }
}
