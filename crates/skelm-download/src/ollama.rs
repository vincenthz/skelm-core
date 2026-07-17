use crate::{
    ProgressDisplay,
    http::{HttpError, describe_reqwest_error},
};

use super::http;
use reqwest::StatusCode;
use skelm_ollama as ollama;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DownloadError {
    #[error("failed downloading {origin} from {url}: {source}")]
    BlobDownloadFailed {
        /// Which part of the manifest this blob belongs to (see [`blob_origin`]).
        origin: String,
        url: String,
        #[source]
        source: HttpError,
    },
    #[error("could not reach the registry to fetch manifest {url}: {detail}")]
    ManifestRequestFailed {
        url: String,
        detail: String,
        #[source]
        source: reqwest::Error,
    },
    #[error("registry returned http-status={status} for manifest {url}{detail}")]
    ManifestError {
        url: String,
        status: StatusCode,
        /// Precomputed hint + (truncated) response body, or empty.
        detail: String,
    },
    #[error("could not read the manifest response body from {url}: {detail}")]
    ManifestBodyFailed {
        url: String,
        detail: String,
        #[source]
        source: reqwest::Error,
    },
    #[error("could not parse the manifest JSON from {url}: {detail}")]
    ManifestParseFailed { url: String, detail: String },
    #[error("Fail to add manifest {0:?}")]
    ManifestAddingFailed(std::io::Error),
    #[error("failed to commit {origin} (blob {blob}): {source}")]
    BlobCommitFailed {
        origin: String,
        blob: ollama::Blob,
        #[source]
        source: std::io::Error,
    },
    #[error("{origin} is corrupt: expected {expected} but downloaded {got}")]
    InvalidBlobDownloaded {
        origin: String,
        expected: ollama::Blob,
        got: ollama::Blob,
    },
}

/// A short, actionable hint for the common manifest HTTP failures.
fn status_hint(status: StatusCode) -> Option<&'static str> {
    match status.as_u16() {
        401 | 403 => Some("the registry rejected the request (authentication/authorization)"),
        404 => Some("model or variant not found — check the name, e.g. `qwen2.5:7b`"),
        429 => Some("rate limited by the registry; retry later"),
        500..=599 => Some("registry server error; retry later or check the registry status"),
        _ => None,
    }
}

/// Build the trailing detail (hint + truncated response body) for a failed
/// manifest fetch, so the error carries enough to diagnose it.
fn manifest_error_detail(status: StatusCode, body: &str) -> String {
    let mut detail = String::new();
    if let Some(hint) = status_hint(status) {
        detail.push_str("\n  hint: ");
        detail.push_str(hint);
    }
    let body = body.trim();
    if !body.is_empty() {
        const MAX: usize = 2000;
        let shown: String = body.chars().take(MAX).collect();
        detail.push_str("\n  response body: ");
        detail.push_str(&shown);
        if body.chars().count() > MAX {
            detail.push_str(" …(truncated)");
        }
    }
    detail
}

pub async fn download_model<PB: ProgressDisplay>(
    client: &reqwest::Client,
    config: &ollama::OllamaConfig,
    store: &ollama::OllamaStore,
    registry: &ollama::Registry,
    model: &ollama::Model,
    variant: &ollama::Variant,
) -> Result<Vec<(String, DownloadResult)>, DownloadError> {
    let manifest_url = config.manifest_url(model, variant);
    let url = manifest_url.to_string();

    let request = client.get(manifest_url).header(
        reqwest::header::CONTENT_TYPE,
        "application/vnd.docker.distribution.manifest.v2+json",
    );

    let response = request
        .send()
        .await
        .map_err(|source| DownloadError::ManifestRequestFailed {
            url: url.clone(),
            detail: describe_reqwest_error(&source),
            source,
        })?;

    let status = response.status();
    if status != StatusCode::OK {
        // Best-effort read of the error body; registries usually return a JSON
        // error document describing exactly what went wrong.
        let body = response.text().await.unwrap_or_default();
        return Err(DownloadError::ManifestError {
            url,
            status,
            detail: manifest_error_detail(status, &body),
        });
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|source| DownloadError::ManifestBodyFailed {
            url: url.clone(),
            detail: describe_reqwest_error(&source),
            source,
        })?;

    let manifest = ollama::Manifest::from_json_bytes(&bytes).map_err(|source| {
        DownloadError::ManifestParseFailed {
            url,
            detail: source.to_string(),
        }
    })?;

    download_model_with_manifest::<PB>(client, config, store, &manifest, registry, model, variant)
        .await
}

pub enum DownloadResult {
    Skipped(ollama::Blob),
    Success(ollama::Blob),
}

/// A human-readable label for a blob's place in the manifest, so a blob failure
/// names which part of `model:variant` broke (e.g. `layer [application/
/// vnd.ollama.image.model] of qwen2.5:7b`).
fn blob_origin(model: &ollama::Model, variant: &ollama::Variant, part: &str) -> String {
    format!("{part} of {}:{}", model.as_str(), variant.as_str())
}

async fn download_model_with_manifest<PB: ProgressDisplay>(
    client: &reqwest::Client,
    config: &ollama::OllamaConfig,
    store: &ollama::OllamaStore,
    manifest: &ollama::Manifest,
    registry: &ollama::Registry,
    model: &ollama::Model,
    variant: &ollama::Variant,
) -> Result<Vec<(String, DownloadResult)>, DownloadError> {
    let mut results = Vec::new();

    let config_origin = blob_origin(
        model,
        variant,
        &format!("config layer [{}]", manifest.config.media_type),
    );
    let manifest_result = download_model_blob::<PB>(
        client,
        config,
        store,
        &manifest.config.digest,
        &config_origin,
    )
    .await?;
    results.push(("manifest".to_string(), manifest_result));

    for layer in &manifest.layers {
        let origin = blob_origin(model, variant, &format!("layer [{}]", layer.media_type));
        let r = download_model_blob::<PB>(client, config, store, &layer.digest, &origin).await?;
        results.push((layer.media_type.clone(), r))
    }

    store
        .add_manifest(registry, model, variant, manifest)
        .map_err(|e| DownloadError::ManifestAddingFailed(e))?;
    Ok(results)
}

async fn download_model_blob<PB: ProgressDisplay>(
    client: &reqwest::Client,
    config: &ollama::OllamaConfig,
    store: &ollama::OllamaStore,
    blob: &ollama::Blob,
    origin: &str,
) -> Result<DownloadResult, DownloadError> {
    if store.blob_exists(blob) {
        return Ok(DownloadResult::Skipped(blob.clone()));
    }

    let blob_url = config.blob_url(blob);
    let blob_tmp_path = store.blob_path_tmp(blob);

    if let Some(parent) = blob_tmp_path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }

    let mut blob_context = ollama::BlobContext::new_from_blob_type(blob);

    http::download::<_, PB>(client, &blob_url, &blob_tmp_path, &mut blob_context)
        .await
        .map_err(|source| DownloadError::BlobDownloadFailed {
            origin: origin.to_string(),
            url: blob_url.to_string(),
            source,
        })?;

    let got_blob = blob_context.finalize();
    if &got_blob != blob {
        let _ = std::fs::remove_file(&blob_tmp_path);
        return Err(DownloadError::InvalidBlobDownloaded {
            origin: origin.to_string(),
            expected: blob.clone(),
            got: got_blob,
        });
    }

    std::fs::rename(blob_tmp_path, store.blob_path(blob)).map_err(|source| {
        DownloadError::BlobCommitFailed {
            origin: origin.to_string(),
            blob: blob.clone(),
            source,
        }
    })?;
    Ok(DownloadResult::Success(blob.clone()))
}
