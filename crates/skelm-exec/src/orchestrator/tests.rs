//! Scheduler tests driven by a scripted mock backend — no model required.

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use serde_json::{Value, json};

use super::{GenBackend, Limits, Orchestrator};
use crate::{ModelParameters, Tool, ToolSpec};

/// A backend that computes each reply from the transcript via a closure, so tests
/// can express agent behavior as a function of what the agent has seen so far.
struct MockGenBackend<F>(F);

impl<F: FnMut(&ModelParameters) -> String + Send> GenBackend for MockGenBackend<F> {
    fn generate(&mut self, params: &ModelParameters) -> anyhow::Result<String> {
        Ok((self.0)(params))
    }
}

fn mock<F: FnMut(&ModelParameters) -> String + Send + 'static>(f: F) -> MockGenBackend<F> {
    MockGenBackend(f)
}

/// A Hermes-style tool call, the surface syntax `parse_tool_calls` recognizes.
fn tc(name: &str, args: Value) -> String {
    format!(
        "<tool_call>{}</tool_call>",
        json!({ "name": name, "arguments": args })
    )
}

fn user_of(p: &ModelParameters) -> String {
    p.messages
        .iter()
        .find(|m| m.role == "user")
        .map(|m| m.content.clone())
        .unwrap_or_default()
}

fn has_tool(p: &ModelParameters) -> bool {
    p.messages.iter().any(|m| m.role == "tool")
}

fn last_tool(p: &ModelParameters) -> String {
    p.messages
        .iter()
        .rev()
        .find(|m| m.role == "tool")
        .map(|m| m.content.clone())
        .unwrap_or_default()
}

fn empty_params() -> Value {
    json!({ "type": "object", "properties": {} })
}

#[test]
fn single_agent_no_tools_returns_content() {
    let orch = Orchestrator::spawn(mock(|_p| "hello world".to_string()), vec![], Limits::default());
    let (answer, truncated) = orch.run(
        ModelParameters::single_turn("You are helpful.", "hi"),
        |_, _| String::new(),
    );
    assert_eq!(answer, "hello world");
    assert!(!truncated);
}

#[test]
fn delegate_result_flows_back_to_parent() {
    let backend = mock(|p| {
        if user_of(p) == "compute 2+2" {
            return "4".to_string(); // the sub-agent
        }
        // the root
        if has_tool(p) {
            format!("the answer is {}", last_tool(p))
        } else {
            tc("delegate", json!({ "role": "math", "task": "compute 2+2" }))
        }
    });

    let orch = Orchestrator::spawn(backend, vec![], Limits::default());
    let (answer, truncated) =
        orch.run(ModelParameters::single_turn("root", "solve"), |_, _| String::new());
    assert_eq!(answer, "the answer is 4");
    assert!(!truncated);
}

#[test]
fn multiple_tool_calls_resolve_in_call_order() {
    let tools = vec![
        ToolSpec::sync(Tool::function("first", None, empty_params()), |_| {
            Ok("A".to_string())
        }),
        ToolSpec::sync(Tool::function("second", None, empty_params()), |_| {
            Ok("B".to_string())
        }),
    ];

    let backend = mock(|p| {
        if has_tool(p) {
            let results: Vec<String> = p
                .messages
                .iter()
                .filter(|m| m.role == "tool")
                .map(|m| m.content.clone())
                .collect();
            results.join("|")
        } else {
            format!("{}{}", tc("first", json!({})), tc("second", json!({})))
        }
    });

    let orch = Orchestrator::spawn(backend, tools, Limits::default());
    let (answer, truncated) =
        orch.run(ModelParameters::single_turn("root", "go"), |_, _| String::new());
    assert_eq!(answer, "A|B");
    assert!(!truncated);
}

#[test]
fn deferred_tool_surfaces_and_fulfill_unblocks() {
    let tools = vec![ToolSpec::deferred(Tool::function(
        "ask_human",
        Some("ask the operator".to_string()),
        empty_params(),
    ))];

    let backend = mock(|p| {
        if has_tool(p) {
            format!("done: {}", last_tool(p))
        } else {
            tc("ask_human", json!({ "question": "approve?" }))
        }
    });

    let orch = Orchestrator::spawn(backend, tools, Limits::default());
    let seen = Arc::new(AtomicUsize::new(0));
    let seen2 = seen.clone();
    let (answer, truncated) = orch.run(ModelParameters::single_turn("root", "go"), move |tool, args| {
        assert_eq!(tool, "ask_human");
        assert_eq!(args.get("question").and_then(|v| v.as_str()), Some("approve?"));
        seen2.fetch_add(1, Ordering::SeqCst);
        "approved".to_string()
    });

    assert_eq!(answer, "done: approved");
    assert!(!truncated);
    assert_eq!(seen.load(Ordering::SeqCst), 1);
}

#[test]
fn budget_exhaustion_stops_and_truncates() {
    let calls = Arc::new(AtomicUsize::new(0));
    let calls2 = calls.clone();
    // Root never finishes: it delegates every turn.
    let backend = mock(move |_p| {
        calls2.fetch_add(1, Ordering::SeqCst);
        tc("delegate", json!({ "task": "keep going" }))
    });

    let limits = Limits {
        budget_steps: 1,
        ..Limits::default()
    };
    let orch = Orchestrator::spawn(backend, vec![], limits);
    let (answer, truncated) =
        orch.run(ModelParameters::single_turn("root", "go"), |_, _| String::new());

    assert!(truncated);
    // Exactly one generation happened: the root's. The delegated child never ran
    // because the budget was already spent.
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert!(answer.contains("stopped"));
}

#[test]
fn max_depth_blocks_deeper_delegation() {
    let backend = mock(|p| match user_of(p).as_str() {
        "root-task" => {
            if has_tool(p) {
                format!("root got: {}", last_tool(p))
            } else {
                tc("delegate", json!({ "task": "child-task" }))
            }
        }
        "child-task" => {
            if has_tool(p) {
                format!("child saw: {}", last_tool(p))
            } else {
                // Depth-1 child tries to delegate again; should be refused.
                tc("delegate", json!({ "task": "grandchild-task" }))
            }
        }
        other => format!("unexpected agent: {other}"),
    });

    let limits = Limits {
        max_depth: 1,
        ..Limits::default()
    };
    let orch = Orchestrator::spawn(backend, vec![], limits);
    let (answer, truncated) =
        orch.run(ModelParameters::single_turn("root", "root-task"), |_, _| String::new());

    assert!(!truncated);
    assert!(
        answer.contains("maximum delegation depth"),
        "expected depth-cap error to propagate, got: {answer}"
    );
    // The grandchild must never have run.
    assert!(!answer.contains("unexpected agent"));
}

#[test]
fn max_agents_caps_total_spawns() {
    let backend = mock(|p| match user_of(p).as_str() {
        "root-task" => {
            if has_tool(p) {
                let results: Vec<String> = p
                    .messages
                    .iter()
                    .filter(|m| m.role == "tool")
                    .map(|m| m.content.clone())
                    .collect();
                format!("root: {}", results.join(" ; "))
            } else {
                // Two delegations in one turn; only the first fits the budget.
                format!(
                    "{}{}",
                    tc("delegate", json!({ "task": "a" })),
                    tc("delegate", json!({ "task": "b" }))
                )
            }
        }
        "a" => "done-a".to_string(),
        "b" => "done-b".to_string(),
        other => format!("?? {other}"),
    });

    let limits = Limits {
        max_agents: 2, // root + one child
        max_depth: 5,
        ..Limits::default()
    };
    let orch = Orchestrator::spawn(backend, vec![], limits);
    let (answer, truncated) =
        orch.run(ModelParameters::single_turn("root", "root-task"), |_, _| String::new());

    assert!(!truncated);
    assert!(answer.contains("done-a"), "first delegate should run: {answer}");
    assert!(
        answer.contains("agent budget exhausted"),
        "second delegate should be refused: {answer}"
    );
    assert!(!answer.contains("done-b"), "second child must not run: {answer}");
}
