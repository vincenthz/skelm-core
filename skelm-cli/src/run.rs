use std::io::Write;
use std::sync::Arc;
use std::{path::Path, sync::atomic::AtomicBool};

use skelm_llama_cpp as llama;

pub struct Output {
    handle: Option<std::fs::File>,
    utf8_errors: usize,
}

impl Output {
    pub fn new() -> Self {
        Self {
            utf8_errors: 0,
            handle: None,
        }
    }

    pub fn new_file<P: AsRef<Path>>(p: P) -> std::io::Result<Self> {
        let file = std::fs::File::create(p)?;
        Ok(Self {
            utf8_errors: 0,
            handle: Some(file),
        })
    }

    pub fn append(&mut self, bytes: &[u8]) {
        if let Some(file) = &mut self.handle {
            file.write_all(bytes).unwrap();
        } else {
            match std::str::from_utf8(bytes) {
                Ok(valid) => {
                    print!("{}", valid);
                    std::io::stdout().flush().unwrap();
                }
                Err(_) => {
                    self.utf8_errors += 1;
                }
            }
        }
    }
}

pub fn llama_init_logging(debug: bool) {
    llama::llama_logging(Box::new(move |level, key, t| {
        if level != llama::LogLevel::Error
            && (!debug && ![llama::LogKey::ModelLoader].contains(&key))
        {
            return;
        }
        eprintln!(
            "{:<5} | {:<22} | {}",
            format!("{}", level),
            format!("{:?}", key),
            t
        )
    }));
}

pub fn llama_sampler() -> impl llama::Sampler {
    skelm_exec::default_sampler()
}

/// Feed `line` into `context`, generate until EOS (or Ctrl-C via `quit`),
/// streaming the decoded output, and return the full generated text.
pub fn llama_generate(
    context: &mut skelm_exec::Context,
    line: &str,
    output: &Option<String>,
    quit: &Arc<AtomicBool>,
) -> anyhow::Result<String> {
    let mut out = output
        .as_ref()
        .map(|o| Output::new_file(o))
        .unwrap_or(Ok(Output::new()))?;
    context.generate(line, quit, |bytes| out.append(bytes))
}

/// Execute a single tool call by invoking an external `program`: the call is
/// written to its stdin as `{"name": ..., "arguments": {...}}` JSON, and its
/// stdout is returned as the tool result.
pub fn run_tool(program: &str, call: &skelm_exec::ToolCall) -> anyhow::Result<String> {
    use anyhow::Context;
    use std::process::{Command, Stdio};

    let input = serde_json::json!({
        "name": call.function.name,
        "arguments": call.function.arguments,
    })
    .to_string();

    let mut child = Command::new(program)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .with_context(|| format!("spawning tool executor '{}'", program))?;

    {
        let mut stdin = child
            .stdin
            .take()
            .context("tool executor stdin unavailable")?;
        stdin.write_all(input.as_bytes())?;
    } // drop stdin to signal EOF

    let out = child.wait_with_output()?;
    if !out.status.success() {
        anyhow::bail!("tool executor '{}' exited with {}", program, out.status);
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use skelm_exec::{ToolCall, ToolCallFunction};

    fn call() -> ToolCall {
        ToolCall {
            kind: "function".to_string(),
            function: ToolCallFunction {
                name: "get_weather".to_string(),
                arguments: serde_json::json!({ "city": "Paris" }),
            },
        }
    }

    #[test]
    fn run_tool_pipes_call_json_and_returns_stdout() {
        // `cat` echoes the call JSON we write to stdin straight back to stdout.
        let result = run_tool("cat", &call()).unwrap();
        let value: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(value["name"], "get_weather");
        assert_eq!(value["arguments"]["city"], "Paris");
    }

    #[test]
    fn run_tool_reports_nonzero_exit() {
        // `false` consumes stdin and exits 1.
        assert!(run_tool("false", &call()).is_err());
    }

    #[test]
    fn run_tool_missing_program_errors() {
        assert!(run_tool("skelm-no-such-program", &call()).is_err());
    }
}
