use std::{
    path::PathBuf,
    process::exit,
    str::FromStr,
    sync::Arc,
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, SystemTime},
};

use anyhow::Context;
use clap::Parser;
use skelm_exec::{Message, ModelDescr, ModelParameters, Tool};
use skelm_ollama as ollama;
use skelm_ollama::{OllamaConfig, OllamaStore};

use reqwest::ClientBuilder;

mod args;
mod human;
mod progressbar;
mod run;

use args::Cli;
use progressbar::ProgressBar;

use crate::human::bench_duration_units;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        args::Commands::List { filter } => cmd_list(filter).await,
        args::Commands::Pull { name } => cmd_pull(name).await,
        args::Commands::Remove { name } => cmd_remove(name).await,
        args::Commands::Set { name, key, value } => cmd_set(name, key, value).await,
        args::Commands::Verify { blobs } => cmd_verify(blobs).await,
        args::Commands::Run {
            name,
            debug,
            model_path,
            no_prompt,
            system,
            input,
            tools,
            tool_exec,
            max_tool_iters,
            output,
        } => {
            cmd_run(
                name,
                debug,
                model_path,
                no_prompt,
                system,
                input,
                tools,
                tool_exec,
                max_tool_iters,
                output,
            )
            .await
        }
        args::Commands::Info { name } => cmd_info(name).await,
        args::Commands::Bench { name, max_tokens } => cmd_bench(name, max_tokens).await,
        args::Commands::Embed { name } => cmd_embed(name).await,
    }
}

async fn cmd_set(name: String, key: String, value: String) -> anyhow::Result<()> {
    let model_descr = parse_ollama_descr(&name)?;
    let store = OllamaStore::default();
    let mut manifest = store.get_manifest(&model_descr)?;

    match key.as_str() {
        "model" => {
            let file_path = PathBuf::from(&value);
            if file_path.exists() {
                let src_file = std::fs::File::open(file_path)?;
                let blob = store.add_blob_from_file(src_file)?;
                println!("added blob {}", blob);
                if let Some(layer) = manifest.find_media_type_mut(ollama::MEDIA_TYPE_IMAGE_MODEL) {
                    layer.digest = blob;
                } else {
                    anyhow::bail!("manifest doesn't have a model")
                }

                store.add_manifest(
                    &model_descr.registry,
                    &model_descr.model,
                    &model_descr.variant,
                    &manifest,
                )?;
            } else {
                anyhow::bail!("value should be a model file")
            }
        }
        _ => {
            anyhow::bail!("unknown key to set {}", key)
        }
    }

    Ok(())
}

async fn cmd_info(name: String) -> anyhow::Result<()> {
    let model_descr = parse_model_descr(&name)?;
    let model = skelm_exec::Model::load(&model_descr)?;

    if let Some(chat_template) = model.model.chat_template() {
        println!("chat-template:\n{}", chat_template);
        // The template-jinja block/parse debug dump is disabled while that crate
        // is out of the build; restore it here when template-jinja is re-enabled.
    }

    Ok(())
}

async fn cmd_embed(name: String) -> anyhow::Result<()> {
    let model_descr = parse_model_descr(&name)?;
    let model = skelm_exec::Model::load(&model_descr)?;

    run::llama_init_logging(false);

    let mut context = model.new_context_embeddings().1;

    let pooling_type = context.pooling_type();

    println!("pooling type: {:?}", pooling_type);

    let tokens = model.vocab.tokenize(b"test", true);

    let e = context.embeddings(&tokens)?;

    println!("embedding {:?}", e);
    Ok(())
}

async fn cmd_bench(name: String, max_tokens: Option<u64>) -> anyhow::Result<()> {
    let model_descr = parse_model_descr(&name)?;

    let max_tokens = max_tokens.unwrap_or(u64::MAX);

    run::llama_init_logging(false);

    let model = skelm_exec::Model::load(&model_descr)?;

    let mut context = model.new_context().1;
    let vocab = model.vocab;

    const BENCHMARK_CONTEXT: &str = "this is a context for doing tokens benchmarks";
    let tokens = vocab.tokenize(BENCHMARK_CONTEXT.as_bytes(), true);
    context.append_tokens(&tokens)?;

    let mut sampler = run::llama_sampler();

    let mut token_generated = 0u64;
    let start = SystemTime::now();
    let bar = indicatif::ProgressBar::new_spinner();

    bar.set_style(
        indicatif::ProgressStyle::with_template(
            "{pos:>7} tokens generated in {elapsed_precise} ({per_sec})",
        )
        .unwrap()
        .progress_chars("##-"),
    );

    loop {
        match context.next_token(&mut sampler, &vocab) {
            None => break,
            Some(t) => {
                context.append_tokens(&[t])?;
                token_generated += 1;
                bar.set_position(token_generated);
                if token_generated >= max_tokens {
                    break;
                }
            }
        }
    }

    let end = SystemTime::now();
    bar.finish();

    let dur = end.duration_since(start).unwrap_or(Duration::ZERO);

    let dur_per_token = dur
        .checked_div(token_generated as u32)
        .unwrap_or(Duration::ZERO);

    let tps = token_generated as f64 / dur.as_secs_f64();
    let time_token = bench_duration_units(dur_per_token);

    println!("model              : {}", name);
    println!("tokens generated   : {}", token_generated);
    println!("elapsed            : {}", bench_duration_units(dur));
    println!("tokens per seconds : {:.4}", tps);
    println!("time per token     : {}", time_token);

    Ok(())
}

async fn cmd_run(
    name: String,
    debug: bool,
    model_path: bool,
    no_prompt: bool,
    system: Option<String>,
    input: Option<String>,
    tools: Option<String>,
    tool_exec: Option<String>,
    max_tool_iters: usize,
    output: Option<String>,
) -> anyhow::Result<()> {
    const DEFAULT_SYSTEM_PROMPT: &str = "you are a chatbot answering question";

    let model_descr = if model_path {
        ModelDescr::Path(PathBuf::from(name))
    } else {
        ModelDescr::Ollama(ollama::ModelDescr::from_str(&name).unwrap())
    };

    let input_data = if let Some(input_file) = input {
        std::fs::read_to_string(&input_file)
            .with_context(|| format!("reading input file {}", input_file))?
    } else {
        String::new()
    };

    let prompt = if no_prompt {
        input_data
    } else {
        let mut rl = rustyline::DefaultEditor::new()?;
        match rl.readline(">> ") {
            Err(e) => {
                anyhow::bail!("error {:?}", e);
            }
            Ok(line) => format!("{}\n{}", input_data, line),
        }
    };

    let tools = if let Some(tools_file) = tools {
        let data = std::fs::read_to_string(&tools_file)
            .with_context(|| format!("reading tools file {}", tools_file))?;
        serde_json::from_str::<Vec<Tool>>(&data).with_context(|| {
            format!(
                "parsing tools file {} (expected a JSON array of OpenAI function tool definitions)",
                tools_file
            )
        })?
    } else {
        Vec::new()
    };

    let system = system.unwrap_or_else(|| DEFAULT_SYSTEM_PROMPT.to_string());

    if let Some(output) = &output {
        if std::fs::exists(output).unwrap_or(false) {
            eprintln!("output file \"{}\" already exists, bailing", output);
            exit(0)
        }
    }

    run::llama_init_logging(debug);
    tracing_subscriber::fmt::init();

    let model = skelm_exec::Model::load(&model_descr)?;

    let mut parameters = ModelParameters::single_turn(system, prompt);
    parameters.tools = tools;

    // Ctrl-C aborts the current generation; set the handler once for the whole
    // loop (ctrlc::set_handler errors if called more than once).
    let quit = Arc::new(AtomicBool::new(false));
    {
        let quit = quit.clone();
        ctrlc::set_handler(move || quit.store(true, Ordering::Relaxed))
            .expect("Error setting Ctrl-C handler");
    }

    // Tool loop: render the transcript, generate a reply, and if the model
    // emitted tool calls, execute them, append the results, and generate again.
    for round in 0..max_tool_iters.max(1) {
        let template = model.model_template_render(&parameters);
        let mut context = model.new_context();
        let reply = run::llama_generate(&mut context, &template, &output, &quit)?;

        if quit.load(Ordering::Relaxed) {
            break;
        }

        let parsed = skelm_exec::parse_tool_calls(&reply);
        if parsed.tool_calls.is_empty() {
            // Final answer: already streamed by llama_generate.
            return Ok(());
        }

        eprintln!("\n--- tool calls ({}) ---", parsed.tool_calls.len());
        for call in &parsed.tool_calls {
            let args = serde_json::to_string(&call.function.arguments)
                .unwrap_or_else(|_| "<unserializable>".to_string());
            eprintln!("{}({})", call.function.name, args);
        }

        let Some(tool_exec) = &tool_exec else {
            eprintln!("(no --tool-exec set; not executing tool calls)");
            return Ok(());
        };

        // Record the assistant's tool-call turn, then run each tool and append
        // its result as a `tool` message for the next round.
        parameters.messages.push(Message::assistant_tool_calls(
            parsed.content,
            parsed.tool_calls.clone(),
        ));
        for call in &parsed.tool_calls {
            let result = run::run_tool(tool_exec, call)
                .with_context(|| format!("executing tool {}", call.function.name))?;
            eprintln!("-> {}: {}", call.function.name, result.trim());
            parameters
                .messages
                .push(Message::tool(&call.function.name, result));
        }

        if round + 1 == max_tool_iters.max(1) {
            eprintln!("reached max tool iterations ({})", max_tool_iters);
        }
    }

    Ok(())
}

async fn cmd_list(filter: Option<String>) -> anyhow::Result<()> {
    let store = OllamaStore::default();
    let regs = store.list_registries()?;

    let mut model_lines = Vec::new();

    let now = SystemTime::now();
    for reg in regs {
        println!("{:?}", reg);
        let models = store.list_models(&reg)?;
        for model in models {
            let variants = store.list_model_variants(&reg, &model)?;
            for variant in variants {
                let manifest_path =
                    store.manifest_registry_model_variant_path(&reg, &model, &variant);
                let fs = tokio::fs::File::open(manifest_path).await?;
                let metadata = fs.metadata().await?;
                let modified = metadata.modified()?;

                let descr = ollama::ModelDescr {
                    registry: reg.clone(),
                    model: model.clone(),
                    variant: variant.clone(),
                };
                let manifest = store.get_manifest(&descr)?;
                let size = manifest.size();
                let name = format!("{}:{}", model.as_str(), variant.as_str());
                let acceptable = if let Some(filter) = &filter {
                    name.starts_with(filter)
                } else {
                    true
                };
                if acceptable {
                    let dur = now.duration_since(modified).unwrap_or(Duration::ZERO);
                    model_lines.push((name, size, dur))
                }
            }
        }
    }

    model_lines.sort_by(|(_, _, m1), (_, _, m2)| m1.cmp(m2));

    println!("{:40} {:15} {:15}", "NAME", "SIZE", "MODIFIED");
    for (model_name, size, modified) in model_lines {
        println!(
            "{:40} {:15} {:15}",
            model_name,
            human::size_units(size),
            format!("{} ago", human::duration_units(modified)),
        )
    }
    Ok(())
}

async fn cmd_pull(name: String) -> anyhow::Result<()> {
    let model_descr = parse_ollama_descr(&name)?;

    let store = OllamaStore::default();
    let config = OllamaConfig::default();
    let client = ClientBuilder::new()
        .user_agent("llmup/0.1")
        //.redirect(Policy::none())
        .build()
        .unwrap();

    let download_results = skelm_download::ollama::download_model::<ProgressBar>(
        &client,
        &config,
        &store,
        &model_descr.registry,
        &model_descr.model,
        &model_descr.variant,
    )
    .await?;

    for (download_name, download_result) in download_results {
        let r = match download_result {
            skelm_download::ollama::DownloadResult::Skipped(blob) => {
                format!("{} already downloaded", blob)
            }
            skelm_download::ollama::DownloadResult::Success(blob) => format!("{} downloaded", blob),
        };
        println!("{}: {}", download_name, r)
    }

    Ok(())
}

async fn cmd_remove(name: String) -> anyhow::Result<()> {
    let skelm_exec::ModelDescr::Ollama(model_descr) = parse_model_descr(&name)? else {
        anyhow::bail!("ollama invalid name")
    };
    let store = OllamaStore::default();
    store.remove_manifest(
        &model_descr.registry,
        &model_descr.model,
        &model_descr.variant,
    )?;

    Ok(())
}

async fn cmd_verify(blobs: bool) -> anyhow::Result<()> {
    let store = OllamaStore::default();
    let regs = store.list_registries()?;

    for reg in regs {
        let models = store.list_models(&reg)?;
        for model in models {
            let variants = store.list_model_variants(&reg, &model)?;
            for variant in variants {
                let model_descr = ollama::ModelDescr {
                    registry: reg.clone(),
                    model: model.clone(),
                    variant: variant.clone(),
                };
                let manifest = store.get_manifest(&model_descr)?;
                let digests = manifest.all_digests();

                let mut failed = Vec::new();

                for blob in digests.iter() {
                    if !store.blob_exists(blob) {
                        failed.push(format!("missing {}", blob));
                        continue;
                    }
                    if blobs {
                        let verified = store.blob_self_verify(blob)?;
                        if !verified {
                            failed.push(format!("invalid blob {}", blob))
                        }
                    }
                }

                if failed.is_empty() {
                    println!("{}:{}: OK", model.as_str(), variant.as_str())
                } else {
                    println!("{}:{}: FAILED", model.as_str(), variant.as_str());
                    for f in failed {
                        println!(" * {}", f)
                    }
                }
            }
        }
    }
    Ok(())
}

fn parse_ollama_descr(name: &str) -> anyhow::Result<ollama::ModelDescr> {
    ollama::ModelDescr::from_str(name).map_err(|_| {
        anyhow::anyhow!("Invalid Ollama model description: expecting <registry>/<model>:<variant> or <model>:<variant>")
    })
}

fn parse_model_descr(name: &str) -> anyhow::Result<skelm_exec::ModelDescr> {
    ollama::ModelDescr::from_str(name).map_err(|_| {
        anyhow::anyhow!("Invalid Ollama model description: expecting <registry>/<model>:<variant> or <model>:<variant>")
    }).map(skelm_exec::ModelDescr::Ollama)
}
