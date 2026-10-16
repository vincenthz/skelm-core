//! Hugging Face Hub support for skelm: searching GGUF models, listing the
//! quantizations of a repository, and downloading them into the standard
//! Hugging Face cache (`~/.cache/huggingface/hub`), shared with the other Hub
//! tools.
//!
//! Models are named as with Ollama, `hf.co/{owner}/{name}[:{quantization}]`
//! (see [`HfModelRef`]): [`HfClient::pull`] downloads one, and
//! [`HfCache::resolve`] finds its file, offline, to load it.
//!
//! ```no_run
//! # async fn demo() -> Result<(), skelm_hf::HfError> {
//! use skelm_hf::{HfClient, HfModelRef};
//! use skelm_download::NoProgress;
//!
//! let client = HfClient::from_env()?;
//! let model: HfModelRef = "hf.co/unsloth/Qwen3.5-2B-GGUF:Q4_K_M".parse()?;
//! let pulled = client.pull::<NoProgress>(&model).await?;
//! assert_eq!(client.cache().resolve(&model)?, pulled.path);
//! # Ok(()) }
//! ```

mod cache;
mod client;
mod config;
pub mod gguf;
mod name;

use reqwest::StatusCode;
use thiserror::Error;

pub use cache::{CachedModel, DEFAULT_REVISION, HfCache};
pub use client::{
    DownloadStatus, HfClient, ModelSummary, Pulled, RepoInfo, SearchQuery, SearchSort,
};
pub use config::HfConfig;
pub use gguf::{GgufVariant, RepoFile};
pub use name::{HfModelRef, RepoId};

#[derive(Debug, Error)]
pub enum HfError {
    #[error("{0}")]
    InvalidName(String),
    #[error("{0} is not downloaded, pull it first with `skelm pull {0}`")]
    NotCached(HfModelRef),
    #[error("the repository has no GGUF model file")]
    NoGguf,
    #[error("quantization {quant} not found, available: {}", .available.join(", "))]
    NoSuchQuant {
        quant: String,
        available: Vec<String>,
    },
    #[error(
        "the repository has several quantizations, pick one with `:<quantization>`: {}",
        .0.join(", ")
    )]
    QuantRequired(Vec<String>),
    #[error("the Hugging Face token is not a valid header value")]
    InvalidToken,
    #[error("request to {url} failed: {detail}")]
    Request { url: String, detail: String },
    #[error("{url} returned http-status={status}{}", status_hint(*.status, *.authenticated))]
    Status {
        url: String,
        status: StatusCode,
        authenticated: bool,
    },
    #[error("could not parse the response of {url}: {detail}")]
    Parse { url: String, detail: String },
    #[error("download of {url} failed: {source}")]
    Download {
        url: String,
        #[source]
        source: skelm_download::http::HttpError,
    },
    #[error("{path} is corrupt: expected {expected}, got {got}")]
    Corrupt {
        path: String,
        expected: String,
        got: String,
    },
    #[error("cache I/O error: {0}")]
    Io(#[from] std::io::Error),
}

fn status_hint(status: StatusCode, authenticated: bool) -> &'static str {
    match status.as_u16() {
        // the Hub also answers 401 for repositories that don't exist
        401 | 403 if !authenticated => {
            "\n  hint: the repository doesn't exist, or it is gated / private: check the name; \
             for a gated repository, accept its license on the Hub and set HF_TOKEN \
             (or log in with `huggingface-cli login`)"
        }
        401 | 403 => {
            "\n  hint: the token was rejected, or the repository license was not accepted on the Hub"
        }
        404 => "\n  hint: repository or revision not found, check the name",
        429 => "\n  hint: rate limited by the Hub, retry later (or set HF_TOKEN)",
        500..=599 => "\n  hint: Hub server error, retry later",
        _ => "",
    }
}
