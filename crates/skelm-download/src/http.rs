use std::{io::SeekFrom, path::Path};

use futures_util::StreamExt;
use reqwest::Client;
use thiserror::Error;
use tokio::io::{AsyncSeekExt, AsyncWriteExt};
use url::Url;

use super::utils::{DataUpdatable, ProgressDisplay};

#[derive(Debug, Error)]
pub enum HttpError {
    #[error("I/O Error {0}")]
    IO(#[from] std::io::Error),
    #[error("{detail}")]
    HTTP {
        /// Precomputed, human-readable description (URL, classification and the
        /// underlying cause chain). See [`describe_reqwest_error`].
        detail: String,
        #[source]
        source: reqwest::Error,
    },
}

impl HttpError {
    pub(crate) fn from_reqwest(source: reqwest::Error) -> Self {
        HttpError::HTTP {
            detail: describe_reqwest_error(&source),
            source,
        }
    }
}

/// Build a detailed, human-readable description of a reqwest error.
///
/// reqwest's own `Display` is terse (often just `error sending request for url
/// (...)`); the actionable cause — DNS failure, TLS error, connection refused,
/// an OS errno, a timeout — lives in the `source()` chain. This surfaces the URL,
/// a classification, any HTTP status, and that whole chain.
pub(crate) fn describe_reqwest_error(err: &reqwest::Error) -> String {
    use std::error::Error;

    let mut parts = Vec::new();
    if let Some(url) = err.url() {
        parts.push(format!("url={url}"));
    }
    if let Some(status) = err.status() {
        parts.push(format!("http-status={status}"));
    }
    parts.push(
        if err.is_timeout() {
            "timed out"
        } else if err.is_connect() {
            "connection failed"
        } else if err.is_redirect() {
            "too many redirects"
        } else if err.is_body() {
            "error reading body"
        } else if err.is_decode() {
            "error decoding response"
        } else if err.is_request() {
            "request error"
        } else if err.is_builder() {
            "malformed request"
        } else {
            "error"
        }
        .to_string(),
    );

    // Walk the cause chain for the real underlying failure.
    let mut causes = Vec::new();
    let mut source = err.source();
    while let Some(cause) = source {
        causes.push(cause.to_string());
        source = cause.source();
    }
    if causes.is_empty() {
        causes.push(err.to_string());
    }

    format!("{} ({})", parts.join(", "), causes.join(": "))
}

/// Interface to download from resumable HTTP with an incremental hash and a potential progress report
pub async fn download<H: DataUpdatable, PB: ProgressDisplay>(
    client: &Client,
    url: &Url,
    destination: &Path,
    hash_ctx: &mut H,
) -> Result<(), HttpError> {
    let mut downloaded: u64 = 0;
    let mut file = if destination.exists() {
        let mut f = tokio::fs::OpenOptions::new()
            .read(true)
            .append(true)
            .open(destination)
            .await?;
        downloaded = f.metadata().await?.len();

        hash_ctx.ctx_update_read_file(&mut f).await?;
        // in case the update_read_file is not reading anything, seek to the end
        f.seek(SeekFrom::Start(downloaded)).await?;
        f
    } else {
        tokio::fs::File::create(destination).await?
    };

    let mut request = client.get(url.clone());
    if downloaded > 0 {
        request = request.header(reqwest::header::RANGE, format!("bytes={}-", downloaded));
    }

    let response = request.send().await.map_err(HttpError::from_reqwest)?;

    let total_size = response.content_length().map(|len| len + downloaded);

    let pb = PB::progress_start(total_size);
    pb.progress_update(downloaded);

    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(HttpError::from_reqwest)?;
        file.write_all(&chunk).await?;
        hash_ctx.ctx_update(&chunk);
        downloaded += chunk.len() as u64;
        pb.progress_update(downloaded);
    }

    pb.progress_finalize();

    Ok(())
}
