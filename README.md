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
- 🌐 **OpenAI-compatible API** — `skelm serve` speaks the OpenAI HTTP API (chat, completions,
  embeddings, models) so existing clients work unmodified, batching concurrent requests and
  caching prompts (see [OpenAI-compatible API](#openai-compatible-api)).
- ⚡ **Speculative decoding & LoRA** — faster generation with a draft model or the model's own MTP
  layers, and LoRA adapters applied at load time (see [Speculative decoding & LoRA](#speculative-decoding--lora)).

## Workspace layout

| Crate | Purpose |
|-------|---------|
| `skelm-cli` | The `skelm` command-line tool |
| `skelm-api` | OpenAI-compatible HTTP API server (axum) |
| `skelm-exec` | High-level model loading, chat-template rendering, tool-call parsing, and the engine batching generations |
| `skelm-llama-cpp` | Safe bindings over llama.cpp (model, context, vocab, sampler) |
| `skelm-llama-cpp-sys` | Builds and links llama.cpp (git submodule) |
| `skelm-ollama` | Ollama-compatible model store and manifests |
| `skelm-download` | Registry download client |
| `skelm-hf` | Hugging Face Hub: search, list quantizations, download GGUF models into the HF cache |

## Installation

llama.cpp is vendored as a git submodule and built from source (no `cmake` required —
`crates/skelm-llama-cpp-sys/build.rs` replicates the relevant CMake detection and
compilation), so you need a **C/C++ toolchain**. On macOS install the Xcode
command-line tools (`xcode-select --install`) — the Metal backend and the
Accelerate-based BLAS backend are compiled in automatically. On Linux/Windows the
CPU backend is built with the correct per-architecture sources and flags (x86, ARM,
etc.); add `--features blas` to also build the BLAS backend against OpenBLAS (found
via `OPENBLAS_PATH`, `pkg-config` or the usual system paths, e.g. `libopenblas-dev`).

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

# ...or a GGUF from Hugging Face (into ~/.cache/huggingface/hub), see below
skelm pull hf.co/unsloth/Qwen3.5-2B-GGUF:Q4_K_M

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

## OpenAI-compatible API

`skelm serve` exposes your locally installed models over an OpenAI-compatible HTTP API
(built with [axum]), so existing OpenAI clients work unmodified — point them at the
server and use your model names (exactly as `skelm list` shows them).

```sh
skelm serve                              # binds 127.0.0.1:11434 by default
skelm serve --host 0.0.0.0 --port 8080
skelm serve --parallel 8 --ctx-size 32768
```

| Method & path | Purpose |
|---------------|---------|
| `POST /v1/chat/completions` | Chat — supports `stream`, `tools`, `temperature`, `max_tokens`, `seed`, `stop` |
| `POST /v1/completions` | Legacy text completion (no chat template) |
| `POST /v1/embeddings` | Embeddings (for embedding models) |
| `GET /v1/models`, `GET /v1/models/{id}` | List / retrieve installed models |
| `GET /health` | Liveness probe |

```sh
curl http://127.0.0.1:11434/v1/chat/completions \
  -H 'content-type: application/json' \
  -d '{"model": "qwen2.5:7b", "messages": [{"role": "user", "content": "Say hello"}]}'
```

…or with the OpenAI Python SDK:

```python
from openai import OpenAI

client = OpenAI(base_url="http://127.0.0.1:11434/v1", api_key="not-needed")
resp = client.chat.completions.create(
    model="qwen2.5:7b",
    messages=[{"role": "user", "content": "Say hello"}],
)
print(resp.choices[0].message.content)
```

Notes:

- Requests are **batched**: each model generates up to `--parallel` requests at a time
  (default 4) on one context, decoding the next token of every request in the same batch;
  further requests wait for a slot. Long prompts are processed in chunks alongside the
  ongoing generations.
- Prompts are **cached**: a finished request's tokens stay in its slot, and a new request
  goes to the slot sharing the longest prefix with its prompt, so only the rest is
  processed — a conversation sent again with one more turn, a retry, or requests sharing a
  system prompt. The response's `usage.prompt_tokens_details.cached_tokens` says how many
  prompt tokens were reused. Recurrent / hybrid models (e.g. Qwen3.5) resume from state
  checkpoints, saved before the last prompt token and where prompts diverge.
- `--ctx-size` is the context of each model in tokens (default 16384), shared by its
  parallel requests and their cached prompts: a single request can use all of it, idle
  cached prompts are evicted (least recently used first) when room is needed.
- With `--draft-model` / `--mtp`, requests are generated one at a time on a context of
  their own (speculative decoding is not batched), without prompt caching.
- Greedy (`temperature: 0`) output is not bit-for-bit identical across batch compositions:
  llama.cpp's kernels round differently with different batch sizes.
- Tool calls are returned in the response `tool_calls`; for reliable tool-call recovery
  prefer non-streaming requests (streaming emits raw token text as it is produced).
- Unsupported OpenAI knobs (`top_p`, `n`, …) are accepted and ignored rather than rejected.
- The `skelm-api` crate is also a library: `skelm_api::serve`/`router` and the
  request/response types under `skelm_api::protocol` let you embed the API in another service.

[axum]: https://github.com/tokio-rs/axum

## Hugging Face models

GGUF models on the Hugging Face Hub are named as with Ollama,
`hf.co/<owner>/<repo>[:<quantization>]`, and can be used wherever a model name is expected
(`run`, `bench`, `serve` requests, `--draft-model`, ...) once pulled:

```sh
# find GGUF repositories, and list the quantizations of one
skelm hf search qwen3.5 --sort trending
skelm hf files unsloth/Qwen3.5-2B-GGUF

# download a quantization (Q4_K_M when none is given), then run it
skelm pull hf.co/unsloth/Qwen3.5-2B-GGUF:Q8_0
skelm run hf.co/unsloth/Qwen3.5-2B-GGUF:Q8_0

# pulled models appear in `skelm list`, and are removed with `skelm remove`
skelm remove hf.co/unsloth/Qwen3.5-2B-GGUF:Q8_0
```

Files are stored in the standard Hugging Face cache (`~/.cache/huggingface/hub`, shared with
`huggingface-cli`, transformers, etc.), downloads resume after an interruption and are checked
against their sha256. Split GGUFs (`-00001-of-0000N`) are downloaded whole. The usual variables
apply: `HF_TOKEN` (or the token saved by `huggingface-cli login`) for gated / private
repositories, `HF_HOME` / `HF_HUB_CACHE` for the cache location, and `HF_ENDPOINT` for a mirror.

## Speculative decoding & LoRA

`run`, `orchestrate`, `bench` and `serve` share these options:

```sh
# speculative decoding with a small draft model sharing the model's vocabulary
skelm run qwen2.5:7b --draft-model qwen2.5:0.5b --draft-max 8 --draft-p-min 0.75

# speculative decoding with the model's own MTP (multi-token prediction) layers,
# for models trained with them (Qwen3.5, Qwen3-Next, GLM-4.5, DeepSeek, ...)
skelm bench ./Qwen3.5-2B-MTP-Q8_0.gguf --mtp

# apply LoRA adapters (GGUF), optionally scaled; repeat --lora for several
skelm run qwen2.5:7b --lora ./my-adapter.gguf:0.8
```

The drafter proposes up to `--draft-max` tokens (default 3), and the model checks them all in a
single batch, keeping the drafts for as long as its own sampler agrees with them. The output is
the same as without speculative decoding: only the speed changes, depending on how often the
drafts are accepted (`bench` reports the acceptance rate). `--draft-p-min` stops a draft when the
drafter gets unsure, which usually pays off with draft models.

Drafts are rolled back by removing them from the model's memory, or, for hybrid / recurrent models
that cannot do that, with recurrent-state snapshots or checkpoints of the state.

For `serve`, the options apply to every model the server loads; models without MTP layers, or
whose vocabulary doesn't match the draft model, decode normally.

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
