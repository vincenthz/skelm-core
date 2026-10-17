//! `--lora`, `--draft-model` and `--mtp`: loading a model with its LoRA
//! adapters and speculative decoding set up.

use std::path::PathBuf;

use anyhow::Context as _;
use skelm_exec::{
    ContextOptions, DraftParams, Drafter, Model, ModelDescr, ModelParams, SpeculativeConfig,
};

use crate::args::ModelOptions;

/// Parse `PATH[:SCALE]`. The scale is only split off when it parses as a
/// number, so a path containing `:` (e.g. `C:\...` on Windows) is kept whole.
pub fn parse_lora(arg: &str) -> anyhow::Result<(PathBuf, f32)> {
    let (path, scale) = match arg.rsplit_once(':') {
        Some((path, scale)) if !path.is_empty() => match scale.parse::<f32>() {
            Ok(scale) => (path, scale),
            Err(_) => (arg, 1.0),
        },
        _ => (arg, 1.0),
    };
    let path = PathBuf::from(path);
    if !path.is_file() {
        anyhow::bail!("LoRA adapter {} not found", path.display());
    }
    Ok((path, scale))
}

/// A draft model given as a GGUF path, or as a model name
pub fn draft_descr(name: &str) -> anyhow::Result<ModelDescr> {
    crate::parse_model_descr(name).with_context(|| format!("draft model {name}"))
}

fn draft_params(options: &ModelOptions) -> DraftParams {
    DraftParams {
        n_max: options.draft_max,
        n_min: options.draft_min,
        p_min: options.draft_p_min,
    }
}

/// Load a model, with the LoRA adapters and speculative decoding of `options`
pub fn load_model(descr: &ModelDescr, options: &ModelOptions) -> anyhow::Result<Model> {
    let params = ModelParams {
        load_mtp: options.mtp,
        ..ModelParams::default()
    };
    let model = Model::load_with_params(descr, &params)?;

    let mut lora = Vec::new();
    for arg in &options.lora {
        let (path, scale) = parse_lora(arg)?;
        let adapter = model
            .load_lora(&path)
            .with_context(|| format!("loading LoRA adapter {}", path.display()))?;
        eprintln!("LoRA adapter {} (scale {scale})", path.display());
        lora.push((adapter, scale));
    }

    let draft = draft_params(options);
    let speculative = if options.mtp {
        eprintln!(
            "speculative decoding with the model's MTP layers (draft max {})",
            draft.n_max
        );
        Some(SpeculativeConfig {
            drafter: Drafter::Mtp,
            draft,
        })
    } else if let Some(name) = &options.draft_model {
        // an MTP assistant (e.g. Gemma 4's) needs its MTP layers
        let params = ModelParams {
            load_mtp: true,
            ..ModelParams::default()
        };
        let draft_model = Model::load_with_params(&draft_descr(name)?, &params)
            .with_context(|| format!("loading draft model {name}"))?;
        eprintln!(
            "speculative decoding with draft model {name} (draft max {})",
            draft.n_max
        );
        Some(SpeculativeConfig {
            drafter: Drafter::Model {
                model: draft_model.model,
                params: Default::default(),
            },
            draft,
        })
    } else {
        None
    };

    Ok(model.with_context_options(ContextOptions {
        lora,
        speculative,
        ..ContextOptions::default()
    })?)
}

/// The server equivalent of [`load_model`]: applied to every model it loads
pub fn serve_options(options: &ModelOptions) -> anyhow::Result<skelm_api::ServeOptions> {
    let lora = options
        .lora
        .iter()
        .map(|arg| parse_lora(arg))
        .collect::<anyhow::Result<Vec<_>>>()?;
    let draft = draft_params(options);
    let speculative = if options.mtp {
        Some((skelm_api::DraftSource::Mtp, draft))
    } else if let Some(name) = &options.draft_model {
        Some((skelm_api::DraftSource::Model(draft_descr(name)?), draft))
    } else {
        None
    };
    Ok(skelm_api::ServeOptions {
        lora,
        speculative,
        ..skelm_api::ServeOptions::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lora_scale_is_split_off_the_path() {
        let dir = std::env::temp_dir().join(format!("skelm-lora-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("adapter.gguf");
        std::fs::write(&file, b"").unwrap();
        let path = file.to_str().unwrap();

        assert_eq!(parse_lora(path).unwrap(), (file.clone(), 1.0));
        assert_eq!(
            parse_lora(&format!("{path}:0.5")).unwrap(),
            (file.clone(), 0.5)
        );
        assert!(parse_lora(&format!("{path}.missing")).is_err());

        std::fs::remove_dir_all(&dir).unwrap();
    }
}
