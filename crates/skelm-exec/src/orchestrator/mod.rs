//! A cooperative multi-agent orchestrator.
//!
//! One **root** agent is seeded programmatically; the model fans work out by
//! calling the built-in [`delegate`](tools::DELEGATE_TOOL) tool, which spawns a
//! fresh sub-agent. Agents run under hard limits ([`Limits`]) with a
//! generation-**step** budget.
//!
//! # Model
//!
//! An *agent* is a chat transcript ([`ModelParameters`]) advanced through a tool
//! loop: render → generate → parse tool calls → obtain results → repeat. An agent
//! is **running** when its next reply is being generated, and **waiting** when it
//! needs results that aren't ready. Three things produce those results, through
//! one mechanism (a per-call *slot*): a sub-agent it delegated to, a synchronous
//! tool run on a worker thread, or the host (external service / human) via
//! [`Orchestrator::fulfill`].
//!
//! # Threads
//!
//! Generation is serialized on a single worker thread (llama.cpp decodes one
//! context at a time); synchronous tools run on a small worker pool so their I/O
//! overlaps with generation of *other* agents. A dedicated scheduler thread owns
//! all agent state (so it needs no locks) and reacts to events. The host drives
//! it through the [`Orchestrator`] handle.
//!
//! ```no_run
//! use skelm_exec::{Limits, LlamaGenBackend, Model, ModelDescr, ModelParameters, Orchestrator};
//! # fn demo(descr: ModelDescr) -> anyhow::Result<()> {
//! let model = Model::load(&descr)?;
//! let orch = Orchestrator::spawn(LlamaGenBackend::new(model), vec![], Limits::default());
//! let root = ModelParameters::single_turn("You coordinate specialists.", "Plan a trip to Kyoto.");
//! let (answer, truncated) = orch.run(root, |_tool, _args| String::new());
//! println!("{answer} (truncated: {truncated})");
//! # Ok(()) }
//! ```

mod tools;

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use serde_json::Value;

use crate::{Message, Model, ModelParameters, Tool, ToolCall, parse_tool_calls};

pub use tools::{ToolKind, ToolSpec};
use tools::{DELEGATE_TOOL, delegate_tool_def, parse_delegate_args};

type SyncHandler = Arc<dyn Fn(&ToolCall) -> anyhow::Result<String> + Send + Sync>;

/// Identifies an agent. The root is always `AgentId(0)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AgentId(pub usize);

/// Identifies an outstanding result slot (a delegated sub-agent, a running tool,
/// or a deferred host request). Returned to the host in
/// [`HostEvent::NeedInput`] and passed back to [`Orchestrator::fulfill`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PendingId(pub usize);

/// Hard limits on a run.
#[derive(Debug, Clone)]
pub struct Limits {
    /// Total number of generation steps (one render→generate round each) across
    /// all agents. The primary budget; when it runs out the run stops.
    pub budget_steps: usize,
    /// Maximum number of agents created over the whole run, including the root.
    pub max_agents: usize,
    /// Maximum agent depth; the root is depth 0. `delegate` is only offered to,
    /// and only succeeds for, agents whose children stay within this depth.
    pub max_depth: usize,
    /// Maximum generation rounds a single agent may take before being finalized.
    pub max_iters_per_agent: usize,
    /// Number of threads running synchronous tools concurrently.
    pub tool_workers: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Limits {
            budget_steps: 32,
            max_agents: 16,
            max_depth: 3,
            max_iters_per_agent: 8,
            tool_workers: 4,
        }
    }
}

/// An event the host receives from [`Orchestrator::recv`].
#[derive(Debug, Clone)]
pub enum HostEvent {
    /// A [`ToolKind::Deferred`] tool was called; the host must produce its result
    /// and pass it to [`Orchestrator::fulfill`] with this `pending` id.
    NeedInput {
        pending: PendingId,
        tool: String,
        args: Value,
    },
    /// The root agent completed. `truncated` is true when the run was stopped by a
    /// limit (budget/iterations/deadlock) rather than the model finishing.
    Finished { result: String, truncated: bool },
}

/// Turns a chat transcript into generated text. Behind a trait so the scheduler
/// is testable without a model and so all model/`Send` concerns stay on the
/// generation thread.
pub trait GenBackend: Send {
    fn generate(&mut self, params: &ModelParameters) -> anyhow::Result<String>;
}

/// The real backend: renders the transcript with the model's chat template and
/// generates a reply on a fresh context.
pub struct LlamaGenBackend {
    model: Model,
    cancel: Arc<AtomicBool>,
}

impl LlamaGenBackend {
    pub fn new(model: Model) -> Self {
        LlamaGenBackend {
            model,
            cancel: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl GenBackend for LlamaGenBackend {
    fn generate(&mut self, params: &ModelParameters) -> anyhow::Result<String> {
        let prompt = self.model.model_template_render(params);
        let mut ctx = self.model.new_context();
        ctx.generate(&prompt, &self.cancel, |_| {})
    }
}

// --- internal messages -----------------------------------------------------

struct GenJob {
    agent: AgentId,
    params: ModelParameters,
}

struct ToolJob {
    pending: PendingId,
    handler: SyncHandler,
    call: ToolCall,
}

enum Event {
    GenDone { agent: AgentId, text: String },
    GenFailed { agent: AgentId, error: String },
    ToolDone {
        pending: PendingId,
        result: Result<String, String>,
    },
    Cmd(Cmd),
}

enum Cmd {
    SubmitRoot(Box<ModelParameters>),
    Fulfill { pending: PendingId, result: String },
    Shutdown,
}

// --- host handle -----------------------------------------------------------

/// Handle to a running orchestrator. Cheap to hold; drives the scheduler through
/// channels.
pub struct Orchestrator {
    events_tx: Sender<Event>,
    host_rx: Receiver<HostEvent>,
    scheduler: Option<thread::JoinHandle<()>>,
}

impl Orchestrator {
    /// Start the orchestrator: spawns the scheduler, the generation worker (owning
    /// `backend`), and a pool of tool workers. `tools` are the host tools agents
    /// may call (besides the built-in `delegate`).
    pub fn spawn<B: GenBackend + 'static>(
        backend: B,
        tools: Vec<ToolSpec>,
        limits: Limits,
    ) -> Orchestrator {
        let (events_tx, events_rx) = mpsc::channel::<Event>();
        let (gen_tx, gen_rx) = mpsc::channel::<GenJob>();
        let (tool_tx, tool_rx) = mpsc::channel::<ToolJob>();
        let (host_tx, host_rx) = mpsc::channel::<HostEvent>();

        // Generation worker: owns the backend; only strings cross the boundary.
        let gen_events = events_tx.clone();
        thread::spawn(move || gen_worker(backend, gen_rx, gen_events));

        // Tool worker pool: shares one receiver behind a mutex so dequeue is
        // serialized but the tool bodies run concurrently.
        let tool_rx = Arc::new(Mutex::new(tool_rx));
        for _ in 0..limits.tool_workers.max(1) {
            let rx = tool_rx.clone();
            let ev = events_tx.clone();
            thread::spawn(move || tool_worker(rx, ev));
        }

        let scheduler = Scheduler::new(Registry::new(tools, limits.max_depth), limits, tool_tx, host_tx, gen_tx);
        let handle = thread::spawn(move || scheduler.run(events_rx));

        Orchestrator {
            events_tx,
            host_rx,
            scheduler: Some(handle),
        }
    }

    /// Seed the root agent. Its tool set is taken from the orchestrator's registry
    /// (any `tools` on `params` are replaced).
    pub fn submit_root(&self, params: ModelParameters) {
        let _ = self.events_tx.send(Event::Cmd(Cmd::SubmitRoot(Box::new(params))));
    }

    /// Provide the result for a [`HostEvent::NeedInput`]. Unknown or already-filled
    /// pendings are ignored.
    pub fn fulfill(&self, pending: PendingId, result: impl Into<String>) {
        let _ = self.events_tx.send(Event::Cmd(Cmd::Fulfill {
            pending,
            result: result.into(),
        }));
    }

    /// Block until the next host event, or `None` once the run has ended.
    pub fn recv(&self) -> Option<HostEvent> {
        self.host_rx.recv().ok()
    }

    /// Ask the scheduler to stop as soon as possible.
    pub fn shutdown(&self) {
        let _ = self.events_tx.send(Event::Cmd(Cmd::Shutdown));
    }

    /// Wait for the scheduler thread to finish.
    pub fn join(mut self) {
        if let Some(h) = self.scheduler.take() {
            let _ = h.join();
        }
    }

    /// Convenience driver: submit `root`, servicing any deferred-tool requests
    /// inline with `on_need_input(tool_name, args) -> result`, and return the
    /// root's final answer plus whether the run was truncated by a limit.
    pub fn run(
        &self,
        root: ModelParameters,
        mut on_need_input: impl FnMut(&str, &Value) -> String,
    ) -> (String, bool) {
        self.submit_root(root);
        loop {
            match self.recv() {
                Some(HostEvent::NeedInput { pending, tool, args }) => {
                    let result = on_need_input(&tool, &args);
                    self.fulfill(pending, result);
                }
                Some(HostEvent::Finished { result, truncated }) => return (result, truncated),
                None => return (String::new(), true),
            }
        }
    }
}

// --- tool registry ---------------------------------------------------------

struct Registry {
    host_defs: Vec<Tool>,
    sync: HashMap<String, SyncHandler>,
    deferred: HashSet<String>,
    delegate_def: Tool,
    max_depth: usize,
}

impl Registry {
    fn new(specs: Vec<ToolSpec>, max_depth: usize) -> Self {
        let mut host_defs = Vec::new();
        let mut sync = HashMap::new();
        let mut deferred = HashSet::new();
        for spec in specs {
            let name = spec.def.function.name.clone();
            host_defs.push(spec.def);
            match spec.kind {
                ToolKind::Sync(f) => {
                    sync.insert(name, Arc::from(f));
                }
                ToolKind::Deferred => {
                    deferred.insert(name);
                }
            }
        }
        Registry {
            host_defs,
            sync,
            deferred,
            delegate_def: delegate_tool_def(),
            max_depth,
        }
    }

    /// Tool definitions exposed to an agent at `depth`. `delegate` is only offered
    /// when the agent may legally spawn a child (`depth < max_depth`).
    fn tool_defs_for(&self, depth: usize) -> Vec<Tool> {
        let mut defs = self.host_defs.clone();
        if depth < self.max_depth {
            defs.push(self.delegate_def.clone());
        }
        defs
    }
}

// --- scheduler -------------------------------------------------------------

struct Slot {
    tool_name: String,
    result: Option<String>,
}

/// A pending tool round for an agent: the ordered slots and how many are unfilled.
struct Round {
    slots: Vec<Slot>,
    remaining: usize,
}

struct Agent {
    params: ModelParameters,
    depth: usize,
    iters: usize,
    /// The pending slot in this agent's *parent* that its final answer resolves;
    /// `None` for the root.
    parent_slot: Option<PendingId>,
    /// Present while the agent is waiting on tool/sub-agent results.
    round: Option<Round>,
}

struct Scheduler {
    registry: Registry,
    limits: Limits,
    tool_tx: Sender<ToolJob>,
    host_tx: Sender<HostEvent>,
    gen_tx: Sender<GenJob>,

    agents: HashMap<AgentId, Agent>,
    ready: VecDeque<AgentId>,
    /// pending slot -> (owning agent, slot index within its round)
    pending: HashMap<PendingId, (AgentId, usize)>,

    next_agent: usize,
    agents_created: usize,
    next_pending: usize,

    budget: usize,
    gen_busy: bool,
    root: Option<AgentId>,
    root_last_text: Option<String>,
    done: bool,
    /// When set (via the `SKELM_ORCH_TRACE` env var), narrate scheduling decisions
    /// to stderr — invaluable for seeing how work fanned out.
    trace: bool,
}

/// Shorten text for single-line trace output.
fn snippet(s: &str) -> String {
    let s = s.split_whitespace().collect::<Vec<_>>().join(" ");
    if s.chars().count() > 60 {
        format!("{}…", s.chars().take(60).collect::<String>())
    } else {
        s
    }
}

impl Scheduler {
    fn new(
        registry: Registry,
        limits: Limits,
        tool_tx: Sender<ToolJob>,
        host_tx: Sender<HostEvent>,
        gen_tx: Sender<GenJob>,
    ) -> Self {
        Scheduler {
            budget: limits.budget_steps,
            registry,
            limits,
            tool_tx,
            host_tx,
            gen_tx,
            agents: HashMap::new(),
            ready: VecDeque::new(),
            pending: HashMap::new(),
            next_agent: 0,
            agents_created: 0,
            next_pending: 0,
            gen_busy: false,
            root: None,
            root_last_text: None,
            done: false,
            trace: std::env::var_os("SKELM_ORCH_TRACE").is_some(),
        }
    }

    fn run(mut self, events_rx: Receiver<Event>) {
        while let Ok(event) = events_rx.recv() {
            match event {
                Event::Cmd(Cmd::SubmitRoot(params)) => self.submit_root(*params),
                Event::Cmd(Cmd::Fulfill { pending, result }) => self.resolve(pending, result),
                Event::Cmd(Cmd::Shutdown) => break,
                Event::GenDone { agent, text } => self.on_gen_done(agent, text),
                Event::GenFailed { agent, error } => {
                    self.gen_busy = false;
                    self.finish_agent(agent, format!("[generation failed: {error}]"), true);
                }
                Event::ToolDone { pending, result } => {
                    let value = result.unwrap_or_else(|e| format!("[tool error: {e}]"));
                    self.resolve(pending, value);
                }
            }

            self.dispatch();

            if self.should_terminate() {
                self.terminate();
                break;
            }
        }
        // Returning drops gen_tx / tool_tx / host_tx, ending the workers and
        // closing the host channel.
    }

    fn submit_root(&mut self, mut params: ModelParameters) {
        if self.root.is_some() {
            return;
        }
        params.tools = self.registry.tool_defs_for(0);
        let id = self.new_agent_id();
        self.root = Some(id);
        self.agents.insert(
            id,
            Agent {
                params,
                depth: 0,
                iters: 0,
                parent_slot: None,
                round: None,
            },
        );
        self.ready.push_back(id);
    }

    /// Dispatch one generation if the worker is free, an agent is ready, and there
    /// is budget. Only one generation runs at a time (serialized decode).
    fn dispatch(&mut self) {
        while !self.gen_busy && self.budget > 0 {
            let Some(id) = self.ready.pop_front() else {
                return;
            };
            // Agents that hit the per-agent iteration cap are finalized instead.
            let over_iters = self
                .agents
                .get(&id)
                .is_some_and(|a| a.iters >= self.limits.max_iters_per_agent);
            if over_iters {
                self.finish_agent(id, "[reached max iterations]".to_string(), true);
                continue;
            }
            let Some(agent) = self.agents.get_mut(&id) else {
                continue;
            };
            agent.iters += 1;
            let depth = agent.depth;
            let params = agent.params.clone();
            self.budget -= 1;
            self.gen_busy = true;
            if self.trace {
                eprintln!(
                    "[orch] generate agent#{} (depth {depth}) — {} step(s) left",
                    id.0, self.budget
                );
            }
            let _ = self.gen_tx.send(GenJob { agent: id, params });
        }
    }

    fn on_gen_done(&mut self, id: AgentId, text: String) {
        self.gen_busy = false;
        let parsed = parse_tool_calls(&text);
        // Track the root's most recent *visible* text (markup stripped) so a
        // budget/limit truncation can return a sensible partial answer.
        if self.root == Some(id) {
            self.root_last_text = Some(parsed.content.clone());
        }
        if !self.agents.contains_key(&id) {
            return; // agent was finalized while generating
        }

        if parsed.tool_calls.is_empty() {
            self.finish_agent(id, parsed.content, false);
            return;
        }

        if self.trace {
            let names: Vec<&str> = parsed
                .tool_calls
                .iter()
                .map(|c| c.function.name.as_str())
                .collect();
            eprintln!("[orch] agent#{} requested tool(s): {}", id.0, names.join(", "));
        }

        // Record the assistant's tool-call turn, then open a slot per call.
        let depth = self.agents[&id].depth;
        {
            let agent = self.agents.get_mut(&id).unwrap();
            agent.params.messages.push(Message::assistant_tool_calls(
                parsed.content,
                parsed.tool_calls.clone(),
            ));
            let slots = parsed
                .tool_calls
                .iter()
                .map(|c| Slot {
                    tool_name: c.function.name.clone(),
                    result: None,
                })
                .collect::<Vec<_>>();
            agent.round = Some(Round {
                remaining: slots.len(),
                slots,
            });
        }

        // Dispatch each call. Some resolve immediately (unknown tool / over a cap);
        // collect those and apply them once all slots exist so the round finalizes
        // cleanly.
        let mut immediate: Vec<(PendingId, String)> = Vec::new();
        for (idx, call) in parsed.tool_calls.iter().enumerate() {
            let pid = self.new_pending_id();
            self.pending.insert(pid, (id, idx));
            if let Some(result) = self.dispatch_call(id, pid, call, depth) {
                immediate.push((pid, result));
            }
        }
        for (pid, result) in immediate {
            self.resolve(pid, result);
        }
    }

    /// Route one tool call. Returns `Some(result)` if it resolves synchronously,
    /// `None` if a worker / sub-agent / host will resolve it later.
    fn dispatch_call(
        &mut self,
        agent_id: AgentId,
        pid: PendingId,
        call: &ToolCall,
        depth: usize,
    ) -> Option<String> {
        let name = call.function.name.as_str();

        if name == DELEGATE_TOOL {
            if depth + 1 > self.registry.max_depth {
                if self.trace {
                    eprintln!(
                        "[orch] agent#{} delegate refused: would exceed max depth {}",
                        agent_id.0, self.registry.max_depth
                    );
                }
                return Some("[delegate error: maximum delegation depth reached]".to_string());
            }
            if self.agents_created >= self.limits.max_agents {
                if self.trace {
                    eprintln!(
                        "[orch] agent#{} delegate refused: agent budget ({}) exhausted",
                        agent_id.0, self.limits.max_agents
                    );
                }
                return Some("[delegate error: agent budget exhausted]".to_string());
            }
            let (role, task) = parse_delegate_args(call);
            let child_id = self.new_agent_id();
            if self.trace {
                eprintln!(
                    "[orch] agent#{} delegates -> agent#{}: \"{}\"",
                    agent_id.0,
                    child_id.0,
                    snippet(&task)
                );
            }
            let mut params = ModelParameters::single_turn(role, task);
            params.tools = self.registry.tool_defs_for(depth + 1);
            self.agents.insert(
                child_id,
                Agent {
                    params,
                    depth: depth + 1,
                    iters: 0,
                    parent_slot: Some(pid),
                    round: None,
                },
            );
            self.ready.push_back(child_id);
            return None;
        }

        if let Some(handler) = self.registry.sync.get(name) {
            if self.trace {
                eprintln!("[orch] agent#{} calls sync tool `{name}`", agent_id.0);
            }
            let _ = self.tool_tx.send(ToolJob {
                pending: pid,
                handler: handler.clone(),
                call: call.clone(),
            });
            return None;
        }

        if self.registry.deferred.contains(name) {
            if self.trace {
                eprintln!(
                    "[orch] agent#{} calls deferred tool `{name}` (awaiting host)",
                    agent_id.0
                );
            }
            let _ = self.host_tx.send(HostEvent::NeedInput {
                pending: pid,
                tool: name.to_string(),
                args: call.function.arguments.clone(),
            });
            return None;
        }

        Some(format!("[unknown tool: {name}]"))
    }

    /// Fill a pending slot. When it is the agent's last slot, append the results as
    /// `tool` messages in call order and re-queue the agent for another round.
    fn resolve(&mut self, pid: PendingId, result: String) {
        let Some((agent_id, slot_idx)) = self.pending.remove(&pid) else {
            return;
        };
        let Some(agent) = self.agents.get_mut(&agent_id) else {
            return;
        };
        let Some(round) = agent.round.as_mut() else {
            return;
        };
        if round.slots[slot_idx].result.is_none() {
            round.slots[slot_idx].result = Some(result);
            round.remaining -= 1;
        }
        if round.remaining == 0 {
            let round = agent.round.take().unwrap();
            for slot in round.slots {
                let value = slot.result.unwrap_or_default();
                agent
                    .params
                    .messages
                    .push(Message::tool(slot.tool_name, value));
            }
            self.ready.push_back(agent_id);
        }
    }

    /// Complete an agent with `content`: resolve its parent's slot, or, for the
    /// root, emit [`HostEvent::Finished`].
    fn finish_agent(&mut self, id: AgentId, content: String, truncated: bool) {
        let Some(agent) = self.agents.remove(&id) else {
            return;
        };
        if self.trace {
            let who = if self.root == Some(id) {
                "root".to_string()
            } else {
                format!("agent#{}", id.0)
            };
            let trunc = if truncated { " (truncated)" } else { "" };
            eprintln!("[orch] {who} finished{trunc}: \"{}\"", snippet(&content));
        }
        match agent.parent_slot {
            Some(pid) => self.resolve(pid, content),
            None => self.finish_root(content, truncated),
        }
    }

    fn finish_root(&mut self, result: String, truncated: bool) {
        if self.done {
            return;
        }
        self.done = true;
        let _ = self.host_tx.send(HostEvent::Finished { result, truncated });
    }

    fn should_terminate(&self) -> bool {
        if self.done {
            return true;
        }
        if !self.gen_busy && self.budget == 0 {
            return true; // out of budget; no further generation possible
        }
        // Nothing running, nothing queued, nothing outstanding: genuinely stuck.
        !self.gen_busy && self.ready.is_empty() && self.pending.is_empty()
    }

    /// Force the root to finish with the best answer we have.
    fn terminate(&mut self) {
        if self.done {
            return;
        }
        if self.trace {
            eprintln!(
                "[orch] terminating — {} step(s) left, {} agent(s) created, {} pending",
                self.budget,
                self.agents_created,
                self.pending.len()
            );
        }
        let text = self.root_last_text.clone().unwrap_or_default();
        let result = if text.trim().is_empty() {
            "[orchestrator stopped before the root produced an answer]".to_string()
        } else {
            text
        };
        self.finish_root(result, true);
    }

    fn new_agent_id(&mut self) -> AgentId {
        let id = AgentId(self.next_agent);
        self.next_agent += 1;
        self.agents_created += 1;
        id
    }

    fn new_pending_id(&mut self) -> PendingId {
        let id = PendingId(self.next_pending);
        self.next_pending += 1;
        id
    }
}

fn gen_worker<B: GenBackend>(mut backend: B, rx: Receiver<GenJob>, events: Sender<Event>) {
    while let Ok(job) = rx.recv() {
        let event = match backend.generate(&job.params) {
            Ok(text) => Event::GenDone {
                agent: job.agent,
                text,
            },
            Err(e) => Event::GenFailed {
                agent: job.agent,
                error: e.to_string(),
            },
        };
        if events.send(event).is_err() {
            break;
        }
    }
}

fn tool_worker(rx: Arc<Mutex<Receiver<ToolJob>>>, events: Sender<Event>) {
    loop {
        // Hold the lock only across the (instant) dequeue; run the tool unlocked so
        // other workers can pick up further jobs concurrently.
        let job = {
            let guard = rx.lock().unwrap();
            guard.recv()
        };
        let Ok(job) = job else {
            break;
        };
        let result = (job.handler)(&job.call).map_err(|e| e.to_string());
        if events
            .send(Event::ToolDone {
                pending: job.pending,
                result,
            })
            .is_err()
        {
            break;
        }
    }
}

#[cfg(test)]
mod tests;
