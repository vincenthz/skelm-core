//! `skelm hf ...`, and the Hugging Face side of `pull`, `list` and `remove`.

use std::time::{Duration, SystemTime};

use skelm_hf::{DownloadStatus, HfCache, HfClient, HfModelRef, RepoId, SearchQuery, SearchSort};

use crate::args::{HfCommands, HfSort};
use crate::human;
use crate::progressbar::ProgressBar;

pub async fn cmd_hf(command: HfCommands) -> anyhow::Result<()> {
    match command {
        HfCommands::Search {
            query,
            author,
            limit,
            sort,
            all,
        } => search(query, author, limit, sort, all).await,
        HfCommands::Files { repo } => files(&repo).await,
        HfCommands::Pull { name } => {
            let unprefixed = if HfModelRef::is_hf_name(&name) {
                name.split_once('/').map_or(name.as_str(), |(_, rest)| rest)
            } else {
                name.as_str()
            };
            pull(&HfModelRef::parse_unprefixed(unprefixed)?).await
        }
    }
}

async fn search(
    query: Option<String>,
    author: Option<String>,
    limit: usize,
    sort: HfSort,
    all: bool,
) -> anyhow::Result<()> {
    let client = HfClient::from_env()?;
    let results = client
        .search_models(&SearchQuery {
            search: query,
            author,
            gguf_only: !all,
            sort: match sort {
                HfSort::Downloads => SearchSort::Downloads,
                HfSort::Likes => SearchSort::Likes,
                HfSort::Trending => SearchSort::Trending,
                HfSort::Updated => SearchSort::LastModified,
            },
            limit,
        })
        .await?;

    let width = results.iter().map(|m| m.id.len()).max().unwrap_or(0).max(4);
    println!(
        "{:width$}  {:>10}  {:>6}  {:10}",
        "REPO", "DOWNLOADS", "LIKES", "UPDATED"
    );
    for model in results {
        let date = model
            .last_modified
            .or(model.created_at)
            .map(|d| d.chars().take(10).collect::<String>())
            .unwrap_or_default();
        println!(
            "{:width$}  {:>10}  {:>6}  {:10}",
            model.id, model.downloads, model.likes, date
        );
    }
    Ok(())
}

fn parse_repo(repo: &str) -> anyhow::Result<RepoId> {
    let repo = if HfModelRef::is_hf_name(repo) {
        repo.split_once('/').map_or(repo, |(_, rest)| rest)
    } else {
        repo
    };
    Ok(repo.parse()?)
}

async fn files(repo: &str) -> anyhow::Result<()> {
    let repo = parse_repo(repo)?;
    let client = HfClient::from_env()?;
    let info = client.repo_info(&repo, skelm_hf::DEFAULT_REVISION).await?;
    let variants = info.gguf_variants();
    if variants.is_empty() {
        println!("{repo} has no GGUF model file");
        return Ok(());
    }

    let (_, cached) = client.cache().cached_variants(&repo).unwrap_or_default();
    let names: Vec<String> = variants
        .iter()
        .map(|v| format!("hf.co/{repo}:{}", v.quant))
        .collect();
    let width = names.iter().map(|n| n.len()).max().unwrap_or(0);
    println!("{:width$}  {:>12}  {:6}  CACHED", "NAME", "SIZE", "FILES");
    for (variant, name) in variants.iter().zip(&names) {
        let is_cached = cached.iter().any(|c| c.quant == variant.quant);
        println!(
            "{:width$}  {:>12}  {:6}  {}",
            name,
            human::size_units(variant.size()),
            variant.files.len(),
            if is_cached { "yes" } else { "" }
        );
    }
    for projector in skelm_hf::gguf::projectors(&info.files) {
        println!(
            "(multimodal projector {}, {})",
            projector.path,
            human::size_units(projector.size)
        );
    }
    if info.gated {
        println!(
            "note: {repo} is gated: accept its license on the Hub and set HF_TOKEN to pull it"
        );
    }
    Ok(())
}

pub async fn pull(model: &HfModelRef) -> anyhow::Result<()> {
    let client = HfClient::from_env()?;
    let pulled = client.pull::<ProgressBar>(model).await?;
    for (path, status) in &pulled.files {
        let status = match status {
            DownloadStatus::Cached => "already downloaded",
            DownloadStatus::Downloaded => "downloaded",
        };
        println!("{}: {status}", path.display());
    }
    println!(
        "{} ({}) ready, run it with `skelm run {}`",
        pulled.model,
        human::size_units(pulled.variant.size()),
        pulled.model
    );
    Ok(())
}

pub fn remove(model: &HfModelRef) -> anyhow::Result<()> {
    let removed = match HfCache::from_env().remove(model) {
        Err(skelm_hf::HfError::NotCached(_)) => {
            anyhow::bail!("{model} is not in the Hugging Face cache")
        }
        res => res?,
    };
    for path in removed {
        println!("removed {}", path.display());
    }
    Ok(())
}

/// The cached Hugging Face models, as `skelm list` lines
pub fn list_lines(now: SystemTime, filter: Option<&str>) -> Vec<(String, u64, Duration)> {
    HfCache::from_env()
        .list_models()
        .into_iter()
        .map(|m| {
            let age = m
                .modified
                .and_then(|t| now.duration_since(t).ok())
                .unwrap_or(Duration::ZERO);
            (m.model.to_string(), m.size, age)
        })
        .filter(|(name, _, _)| filter.is_none_or(|f| name.starts_with(f)))
        .collect()
}
