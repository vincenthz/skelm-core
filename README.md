# skelm

**A framework to manage and run local LLM models — in Rust.**

## skelm-cli

`skelm` pulls models from an [Ollama](https://ollama.com)-compatible registry, stores
them in the familiar `~/.ollama` layout, and runs them locally on top of
[llama.cpp](https://github.com/ggml-org/llama.cpp) — with first-class support for the
Jinja chat templates shipped inside GGUF models, including **tool calling**.

![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)
![rust](https://img.shields.io/badge/rust-1.85%2B-orange)
![edition](https://img.shields.io/badge/edition-2024-informational)

---

## Features

- 📦 **Model management** — pull, list, verify, inspect and remove models by name
  (`registry/model:variant`), reusing an existing Ollama store.
- 🚀 **Local inference** — text generation, benchmarking and embeddings via llama.cpp,
  with a Metal backend on macOS.
- 🧩 **Builtin Jinja chat templates** — the chat template embedded in a GGUF model is
  rendered directly (via minijinja), so prompts match what the model was trained on.
- 🛠️ **Tool calling** — expose tools to the template, parse tool calls back out of the
  model's output, and run an automated tool loop end-to-end (see [Tool calling](#tool-calling)).

## Workspace layout

| Crate | Purpose |
|-------|---------|
| `skelm-cli` | The `skelm` command-line tool |
| `skelm-exec` | High-level model loading, chat-template rendering and tool-call parsing |
| `skelm-llama-cpp` | Safe bindings over llama.cpp (model, context, vocab, sampler) |
| `skelm-llama-cpp-sys` | Builds and links llama.cpp (git submodule) |
| `skelm-ollama` | Ollama-compatible model store and manifests |
| `skelm-download` | Registry download client |
| `skelm-hf` | Hugging Face support *(early stub)* |

## Installation

llama.cpp is vendored as a git submodule and built from source (no `cmake` required —
`crates/skelm-llama-cpp-sys/build.rs` replicates the relevant CMake detection and
compilation), so you need a **C/C++ toolchain**. On macOS install the Xcode
command-line tools (`xcode-select --install`) — the Metal backend is compiled in
automatically. On Linux/Windows the CPU backend is built with the correct
per-architecture sources and flags (x86, ARM, etc.).

```sh
git clone --recursive https://github.com/vincenthz/skelm-core
cd skelm-core
# if you already cloned without --recursive:
git submodule update --init --recursive

cargo install --path skelm-cli   # installs the `skelm` binary
# ...or run without installing:
cargo run --release -p skelm-cli -- <command>
```

### GPU / accelerator backends

`skelm-llama-cpp-sys` exposes a cargo feature per accelerator backend
(`cuda`, `hip`, `musa`, `sycl`, `vulkan`, `opencl`, `blas`). Enabling one makes the
build script **auto-locate** the corresponding toolkit (via env vars like `CUDA_PATH`
/ `VULKAN_SDK` / `ROCM_PATH`, tools on `PATH`, or standard install locations) and
report what it found. Building these backends is **not yet implemented** — for now the
build falls back to the CPU backend and prints a warning. If a toolkit is detected but
its feature isn't enabled, the build prints a hint instead.

```sh
cargo build --features cuda        # detects CUDA, warns, falls back to CPU (for now)
```

The auto-selected CPU architecture flags default to `-march=native`. To pin a portable
baseline for distributable builds, set `SKELM_LLAMA_CPU_FLAGS`:

```sh
SKELM_LLAMA_CPU_FLAGS="-march=x86-64-v3" cargo build --release
```

## Quickstart

```sh
# pull a model from the registry into ~/.ollama
skelm pull qwen2.5:7b

# list installed models (optionally filtered)
skelm list
skelm list --filter qwen

# chat with a model (prompts interactively; --system sets the system prompt)
skelm run qwen2.5:7b --system "You are a helpful assistant."

# non-interactive: read the prompt from a file
skelm run qwen2.5:7b --no-prompt --input prompt.txt

# run a raw GGUF file directly, bypassing the store
skelm run ./model.gguf --model-path

# benchmark generation throughput
skelm bench qwen2.5:7b --max-tokens 256

# generate an embedding
skelm embed nomic-embed-text:latest

# inspect a model's chat template, or verify blob integrity
skelm info qwen2.5:7b
skelm verify --blobs
```

Models are stored under `~/.ollama` and pulled from `registry.ollama.ai` by default.

## Tool calling

`skelm` can render tool definitions into the prompt, recover the tool calls a model
emits, execute them, and feed the results back — an automated agentic loop.

### 1. Declare the tools

Write the tool definitions in the OpenAI function format. These are exposed to the chat
template's `tools` variable:

```json
[
  {
    "type": "function",
    "function": {
      "name": "get_weather",
      "description": "Get the current weather for a city",
      "parameters": {
        "type": "object",
        "properties": { "city": { "type": "string" } },
        "required": ["city"]
      }
    }
  }
]
```

### 2. Provide an executor

`--tool-exec` names a program that runs a tool call. For each call it receives
`{"name": ..., "arguments": {...}}` as JSON on **stdin** and returns the result on
**stdout**. One program handles every tool, switching on the name:

```sh
#!/bin/sh
# weather-tool.sh
call=$(cat)
name=$(printf '%s' "$call" | jq -r .name)
args=$(printf '%s' "$call" | jq -c .arguments)

case "$name" in
  get_weather)
    city=$(printf '%s' "$args" | jq -r .city)
    printf '{"city": "%s", "temp_c": 24, "sky": "sunny"}' "$city"
    ;;
  *)
    printf '{"error": "unknown tool: %s"}' "$name"
    ;;
esac
```

### 3. Run the loop

```sh
skelm run qwen2.5:7b \
  --no-prompt --input question.txt \
  --tools tools.json \
  --tool-exec ./weather-tool.sh \
  --max-tool-iters 4
```

Each round: render the transcript → generate → parse tool calls. If the model asked for
tools, each call is executed, its result appended as a `tool` message, and the model runs
again — until it produces a final answer or `--max-tool-iters` is reached. Without
`--tool-exec`, detected tool calls are printed and the run stops.

### Supported output formats

Tool calls are recovered from whichever convention the model uses:

| Family | Surface syntax |
|--------|----------------|
| Hermes / Qwen / Nous | `<tool_call>{…}</tool_call>` (one or many) |
| Mistral | `[TOOL_CALLS][ {…}, … ]` |
| Llama 3.x | optional `<\|python_tag\|>` then `{"name": …, "parameters": …}` |

### Library API

The same capability is available from `skelm-exec` for building your own loop:

```rust
use skelm_exec::{Message, ModelParameters, Tool, parse_tool_calls};

let mut params = ModelParameters::new(vec![
    Message::system("You can call tools."),
    Message::user("What's the weather in Paris?"),
]);
params.tools = vec![/* Tool::function(...) */];

let prompt = model.model_template_render(&params);   // render with tools
// ... generate `reply` ...
let parsed = parse_tool_calls(&reply);               // recover tool calls

// feed results back and re-render for the next round
params.messages.push(Message::assistant_tool_calls(parsed.content, parsed.tool_calls.clone()));
for call in &parsed.tool_calls {
    let result = /* run the tool */;
    params.messages.push(Message::tool(&call.function.name, result));
}
```

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
