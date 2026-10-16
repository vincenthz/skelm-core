//! The Hub API: searching models, listing repository files, and downloading
//! into the cache.

use std::path::PathBuf;

use cryptoxide::hashing::sha2::Context256;
use serde::Deserialize;
use skelm_download::http::describe_reqwest_error;
use skelm_download::{DataUpdatable, ProgressDisplay};
use url::Url;

use crate::cache::{DEFAULT_REVISION, HfCache};
use crate::gguf::{GgufVariant, RepoFile, gguf_variants, select_variant};
use crate::{HfConfig, HfError, HfModelRef, RepoId};

/// How search results are ordered (most first)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SearchSort {
    #[default]
    Downloads,
    Likes,
    Trending,
    LastModified,
}

impl SearchSort {
    fn as_param(self) -> &'static str {
        match self {
            SearchSort::Downloads => "downloads",
            SearchSort::Likes => "likes",
            SearchSort::Trending => "trendingScore",
            SearchSort::LastModified => "lastModified",
        }
    }
}

/// A model search
#[derive(Clone, Debug)]
pub struct SearchQuery {
    /// text to look for in the repository names
    pub search: Option<String>,
    /// only the repositories of this user / organization
    pub author: Option<String>,
    /// only repositories with GGUF files
    pub gguf_only: bool,
    pub sort: SearchSort,
    pub limit: usize,
}

impl Default for SearchQuery {
    fn default() -> Self {
        SearchQuery {
            search: None,
            author: None,
            gguf_only: true,
            sort: SearchSort::Downloads,
            limit: 20,
        }
    }
}

/// A search result
#[derive(Clone, Debug, Deserialize)]
pub struct ModelSummary {
    pub id: String,
    #[serde(default)]
    pub downloads: u64,
    #[serde(default)]
    pub likes: u64,
    #[serde(default, rename = "lastModified")]
    pub last_modified: Option<String>,
    #[serde(default, rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// The files of a repository at a commit
#[derive(Clone, Debug)]
pub struct RepoInfo {
    pub repo: RepoId,
    /// the commit the revision resolved to
    pub commit: String,
    /// gated repositories need an accepted license and a token
    pub gated: bool,
    pub files: Vec<RepoFile>,
}

impl RepoInfo {
    /// The GGUF quantizations of the repository, smallest first
    pub fn gguf_variants(&self) -> Vec<GgufVariant> {
        gguf_variants(&self.files)
    }
}

#[derive(Deserialize)]
struct ApiRepo {
    sha: String,
    #[serde(default)]
    gated: serde_json::Value,
    #[serde(default)]
    siblings: Vec<ApiSibling>,
}

#[derive(Deserialize)]
struct ApiSibling {
    rfilename: String,
    #[serde(default)]
    size: u64,
    #[serde(default, rename = "blobId")]
    blob_id: Option<String>,
    #[serde(default)]
    lfs: Option<ApiLfs>,
}

#[derive(Deserialize)]
struct ApiLfs {
    sha256: String,
}

/// What a download did
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DownloadStatus {
    /// the file was already in the cache
    Cached,
    Downloaded,
}

/// The result of [`HfClient::pull`]
#[derive(Clone, Debug)]
pub struct Pulled {
    /// the model, with the quantization that was picked
    pub model: HfModelRef,
    pub variant: GgufVariant,
    /// the file to load (the first shard of a split model)
    pub path: PathBuf,
    /// each file of the variant, with what was done
    pub files: Vec<(PathBuf, DownloadStatus)>,
}

/// sha256 of a download, fed incrementally (and replayed on resume)
struct Sha256(Context256);

impl DataUpdatable for Sha256 {
    fn ctx_new() -> Self {
        Sha256(Context256::new())
    }

    fn ctx_update(&mut self, data: &[u8]) {
        self.0.update_mut(data)
    }
}

/// A client of the Hub
#[derive(Clone)]
pub struct HfClient {
    config: HfConfig,
    cache: HfCache,
    http: reqwest::Client,
}

impl HfClient {
    pub fn new(config: HfConfig) -> Result<Self, HfError> {
        let mut headers = reqwest::header::HeaderMap::new();
        if let Some(token) = &config.token {
            // reqwest drops it on redirects to other hosts (the file CDN)
            let mut value = reqwest::header::HeaderValue::from_str(&format!("Bearer {token}"))
                .map_err(|_| HfError::InvalidToken)?;
            value.set_sensitive(true);
            headers.insert(reqwest::header::AUTHORIZATION, value);
        }
        let http = reqwest::Client::builder()
            .user_agent(concat!("skelm/", env!("CARGO_PKG_VERSION")))
            .default_headers(headers)
            .build()
            .map_err(|e| HfError::Request {
                url: config.endpoint.to_string(),
                detail: describe_reqwest_error(&e),
            })?;
        Ok(HfClient {
            cache: HfCache::from_config(&config),
            config,
            http,
        })
    }

    /// A client configured from the environment, see [`HfConfig::from_env`]
    pub fn from_env() -> Result<Self, HfError> {
        Self::new(HfConfig::from_env())
    }

    pub fn cache(&self) -> &HfCache {
        &self.cache
    }

    fn url(&self, path: &str) -> Url {
        self.config.endpoint.join(path).expect("valid hub url")
    }

    async fn get_json<T: serde::de::DeserializeOwned>(&self, url: Url) -> Result<T, HfError> {
        let response = self
            .http
            .get(url.clone())
            .send()
            .await
            .map_err(|e| HfError::Request {
                url: url.to_string(),
                detail: describe_reqwest_error(&e),
            })?;
        let status = response.status();
        if !status.is_success() {
            return Err(HfError::Status {
                url: url.to_string(),
                status,
                authenticated: self.config.token.is_some(),
            });
        }
        let bytes = response.bytes().await.map_err(|e| HfError::Request {
            url: url.to_string(),
            detail: describe_reqwest_error(&e),
        })?;
        serde_json::from_slice(&bytes).map_err(|e| HfError::Parse {
            url: url.to_string(),
            detail: e.to_string(),
        })
    }

    /// Search model repositories
    pub async fn search_models(&self, query: &SearchQuery) -> Result<Vec<ModelSummary>, HfError> {
        let mut url = self.url("api/models");
        {
            let mut params = url.query_pairs_mut();
            if let Some(search) = &query.search {
                params.append_pair("search", search);
            }
            if let Some(author) = &query.author {
                params.append_pair("author", author);
            }
            if query.gguf_only {
                params.append_pair("filter", "gguf");
            }
            params.append_pair("sort", query.sort.as_param());
            params.append_pair("direction", "-1");
            params.append_pair("limit", &query.limit.to_string());
            for field in ["downloads", "likes", "lastModified", "createdAt", "tags"] {
                params.append_pair("expand[]", field);
            }
        }
        self.get_json(url).await
    }

    /// The files of a repository at a revision (`main`, a branch or a commit)
    pub async fn repo_info(&self, repo: &RepoId, revision: &str) -> Result<RepoInfo, HfError> {
        let mut url = self.url(&format!("api/models/{repo}/revision/{revision}"));
        url.query_pairs_mut().append_pair("blobs", "true");
        let api: ApiRepo = self.get_json(url).await?;
        let files = api
            .siblings
            .into_iter()
            .map(|s| RepoFile {
                path: s.rfilename,
                size: s.size,
                sha256: s.lfs.map(|l| l.sha256),
                blob_id: s.blob_id,
            })
            .collect();
        Ok(RepoInfo {
            repo: repo.clone(),
            commit: api.sha,
            // `false`, or the gating mode ("auto" / "manual")
            gated: !matches!(
                api.gated,
                serde_json::Value::Bool(false) | serde_json::Value::Null
            ),
            files,
        })
    }

    /// Download a file of a repository into the cache (resuming a previous
    /// partial download, and checking the sha256 of LFS files), returning its
    /// snapshot path
    pub async fn download_file<PB: ProgressDisplay>(
        &self,
        info: &RepoInfo,
        file: &RepoFile,
    ) -> Result<(PathBuf, DownloadStatus), HfError> {
        let repo = &info.repo;
        let blob = file
            .sha256
            .clone()
            .or_else(|| file.blob_id.clone())
            .ok_or_else(|| HfError::Parse {
                url: format!("{repo}/{}", file.path),
                detail: "the file has no blob id".into(),
            })?;
        let blob_path = self.cache.blob_path(repo, &blob);

        let status = if std::fs::metadata(&blob_path).is_ok_and(|m| m.len() == file.size) {
            DownloadStatus::Cached
        } else {
            std::fs::create_dir_all(blob_path.parent().expect("blob directory"))?;
            let tmp = blob_path.with_file_name(format!("{blob}.incomplete"));
            let encoded: Vec<String> = file
                .path
                .split('/')
                .map(|segment| url::form_urlencoded::byte_serialize(segment.as_bytes()).collect())
                .collect();
            let url = self.url(&format!(
                "{repo}/resolve/{}/{}",
                info.commit,
                encoded.join("/")
            ));

            let mut hash = Sha256::ctx_new();
            skelm_download::http::download::<_, PB>(&self.http, &url, &tmp, &mut hash)
                .await
                .map_err(|source| HfError::Download {
                    url: url.to_string(),
                    source,
                })?;

            let size = std::fs::metadata(&tmp)?.len();
            let got = hex::encode(hash.0.finalize());
            let expected_hash = file.sha256.as_deref().unwrap_or(got.as_str());
            if size != file.size || !got.eq_ignore_ascii_case(expected_hash) {
                // a corrupt partial download would never resume correctly
                let _ = std::fs::remove_file(&tmp);
                return Err(HfError::Corrupt {
                    path: file.path.clone(),
                    expected: format!("{} bytes, sha256 {}", file.size, expected_hash),
                    got: format!("{size} bytes, sha256 {got}"),
                });
            }
            std::fs::rename(&tmp, &blob_path)?;
            DownloadStatus::Downloaded
        };

        let path = self
            .cache
            .link_snapshot(repo, &info.commit, &file.path, &blob)?;
        Ok((path, status))
    }

    /// Download a GGUF model (every shard of the selected quantization) into
    /// the cache, from the `main` revision
    pub async fn pull<PB: ProgressDisplay>(&self, model: &HfModelRef) -> Result<Pulled, HfError> {
        let info = self.repo_info(&model.repo, DEFAULT_REVISION).await?;
        let variants = info.gguf_variants();
        let variant = select_variant(&variants, model.quant.as_deref())?.clone();

        let mut files = Vec::new();
        for file in &variant.files {
            files.push(self.download_file::<PB>(&info, file).await?);
        }
        self.cache
            .write_ref(&model.repo, DEFAULT_REVISION, &info.commit)?;

        Ok(Pulled {
            model: HfModelRef {
                repo: model.repo.clone(),
                quant: Some(variant.quant.clone()),
            },
            path: files[0].0.clone(),
            variant,
            files,
        })
    }
}
