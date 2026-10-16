//! The Hugging Face cache (`~/.cache/huggingface/hub`), in the layout shared
//! with `huggingface_hub` and the other Hub tools:
//!
//! ```text
//! models--{owner}--{name}/
//!     blobs/{sha256 or git blob id}         the file contents
//!     refs/main                             the commit `main` points to
//!     snapshots/{commit}/{path}             a relative symlink to the blob
//! ```

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::gguf::{GgufVariant, RepoFile, gguf_variants, select_variant};
use crate::{HfConfig, HfError, HfModelRef, RepoId};

/// The revision models are pulled from and resolved at
pub const DEFAULT_REVISION: &str = "main";

#[derive(Clone, Debug)]
pub struct HfCache {
    root: PathBuf,
}

/// A GGUF quantization present in the cache
#[derive(Clone, Debug)]
pub struct CachedModel {
    /// the model, with its quantization
    pub model: HfModelRef,
    /// the file to load (the first shard of a split model)
    pub path: PathBuf,
    /// total size of its files
    pub size: u64,
    /// when it was downloaded
    pub modified: Option<SystemTime>,
}

impl HfCache {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        HfCache { root: root.into() }
    }

    pub fn from_config(config: &HfConfig) -> Self {
        Self::new(&config.cache_dir)
    }

    /// The cache of the environment, see [`HfConfig::from_env`]
    pub fn from_env() -> Self {
        Self::from_config(&HfConfig::from_env())
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn repo_dir(&self, repo: &RepoId) -> PathBuf {
        self.root.join(repo.cache_folder())
    }

    pub fn blob_path(&self, repo: &RepoId, blob: &str) -> PathBuf {
        self.repo_dir(repo).join("blobs").join(blob)
    }

    pub fn snapshot_dir(&self, repo: &RepoId, commit: &str) -> PathBuf {
        self.repo_dir(repo).join("snapshots").join(commit)
    }

    /// The commit a revision (e.g. `main`) pointed to at the last download
    pub fn read_ref(&self, repo: &RepoId, revision: &str) -> Option<String> {
        let path = self.repo_dir(repo).join("refs").join(revision);
        let commit = std::fs::read_to_string(path).ok()?;
        Some(commit.trim().to_string()).filter(|c| !c.is_empty())
    }

    pub(crate) fn write_ref(
        &self,
        repo: &RepoId,
        revision: &str,
        commit: &str,
    ) -> std::io::Result<()> {
        let refs = self.repo_dir(repo).join("refs");
        std::fs::create_dir_all(&refs)?;
        std::fs::write(refs.join(revision), commit)
    }

    /// Point `snapshots/{commit}/{path}` at the blob `blob`
    pub(crate) fn link_snapshot(
        &self,
        repo: &RepoId,
        commit: &str,
        path: &str,
        blob: &str,
    ) -> std::io::Result<PathBuf> {
        let link = self.snapshot_dir(repo, commit).join(path);
        if let Some(parent) = link.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let _ = std::fs::remove_file(&link);
        // relative, so the cache can be moved: up from the snapshot path, then
        // up from `snapshots/{commit}` into `blobs/`
        let depth = path.split('/').count() - 1;
        let target = PathBuf::from("../".repeat(depth + 2))
            .join("blobs")
            .join(blob);
        if symlink(&target, &link).is_err() {
            // without symlink support (e.g. Windows without developer mode),
            // the snapshot holds a copy, as huggingface_hub does
            std::fs::copy(self.blob_path(repo, blob), &link)?;
        }
        Ok(link)
    }

    /// The commit whose snapshot to use: the one of `main`, else the most
    /// recently modified snapshot
    fn current_commit(&self, repo: &RepoId) -> Option<String> {
        if let Some(commit) = self.read_ref(repo, DEFAULT_REVISION) {
            if self.snapshot_dir(repo, &commit).is_dir() {
                return Some(commit);
            }
        }
        let snapshots = std::fs::read_dir(self.repo_dir(repo).join("snapshots")).ok()?;
        snapshots
            .filter_map(|e| e.ok())
            .filter_map(|e| Some((e.metadata().ok()?.modified().ok()?, e.file_name())))
            .max()
            .map(|(_, name)| name.to_string_lossy().into_owned())
    }

    /// The files of a snapshot, with their sizes (following the symlinks)
    fn snapshot_files(dir: &Path) -> Vec<RepoFile> {
        fn walk(base: &Path, dir: &Path, out: &mut Vec<RepoFile>) {
            let Ok(entries) = std::fs::read_dir(dir) else {
                return;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    walk(base, &path, out);
                } else if let Ok(meta) = std::fs::metadata(&path) {
                    let rel = path.strip_prefix(base).unwrap_or(&path);
                    let rel = rel
                        .components()
                        .map(|c| c.as_os_str().to_string_lossy())
                        .collect::<Vec<_>>()
                        .join("/");
                    out.push(RepoFile {
                        path: rel,
                        size: meta.len(),
                        sha256: None,
                        blob_id: None,
                    });
                }
            }
        }
        let mut out = Vec::new();
        walk(dir, dir, &mut out);
        out
    }

    /// The GGUF quantizations cached for `repo`, and their snapshot directory
    pub fn cached_variants(&self, repo: &RepoId) -> Option<(PathBuf, Vec<GgufVariant>)> {
        let commit = self.current_commit(repo)?;
        let dir = self.snapshot_dir(repo, &commit);
        let variants = gguf_variants(&Self::snapshot_files(&dir));
        Some((dir, variants))
    }

    /// The local file of a model, without any network access (download it
    /// first with [`crate::HfClient::pull`])
    pub fn resolve(&self, model: &HfModelRef) -> Result<PathBuf, HfError> {
        let not_cached = || HfError::NotCached(model.clone());
        let (dir, variants) = self.cached_variants(&model.repo).ok_or_else(not_cached)?;
        let variant = match select_variant(&variants, model.quant.as_deref()) {
            Ok(v) => v,
            Err(HfError::NoGguf | HfError::NoSuchQuant { .. }) => return Err(not_cached()),
            Err(e) => return Err(e),
        };
        // a split model is only usable with all its shards
        let expected = crate::gguf::shard_count(&variant.primary().path);
        if expected.is_some_and(|n| variant.files.len() != n as usize) {
            return Err(not_cached());
        }
        Ok(dir.join(&variant.primary().path))
    }

    /// Every GGUF quantization in the cache
    pub fn list_models(&self) -> Vec<CachedModel> {
        let Ok(entries) = std::fs::read_dir(&self.root) else {
            return Vec::new();
        };
        let mut models = Vec::new();
        for entry in entries.flatten() {
            let Some(repo) = RepoId::from_cache_folder(&entry.file_name().to_string_lossy()) else {
                continue;
            };
            let Some((dir, variants)) = self.cached_variants(&repo) else {
                continue;
            };
            for variant in variants {
                let path = dir.join(&variant.primary().path);
                let modified = std::fs::metadata(&path).and_then(|m| m.modified()).ok();
                models.push(CachedModel {
                    model: HfModelRef {
                        repo: repo.clone(),
                        quant: Some(variant.quant.clone()),
                    },
                    size: variant.size(),
                    path,
                    modified,
                });
            }
        }
        models.sort_by_key(|m| m.model.to_string());
        models
    }

    /// Remove a cached quantization: its snapshot files, and the blobs no other
    /// snapshot uses. Returns the removed snapshot paths.
    pub fn remove(&self, model: &HfModelRef) -> Result<Vec<PathBuf>, HfError> {
        let path = self.resolve(model)?;
        let (dir, variants) = self
            .cached_variants(&model.repo)
            .ok_or_else(|| HfError::NotCached(model.clone()))?;
        let variant = select_variant(&variants, model.quant.as_deref())?;
        debug_assert_eq!(path, dir.join(&variant.primary().path));

        let mut removed = Vec::new();
        let mut blobs = HashSet::new();
        for file in &variant.files {
            let link = dir.join(&file.path);
            if let Ok(target) = std::fs::read_link(&link) {
                if let Some(blob) = target.file_name() {
                    blobs.insert(blob.to_owned());
                }
            }
            std::fs::remove_file(&link)?;
            removed.push(link);
        }

        // blobs still linked from any snapshot are kept
        let mut in_use = HashSet::new();
        let snapshots = self.repo_dir(&model.repo).join("snapshots");
        fn collect_links(dir: &Path, out: &mut HashSet<std::ffi::OsString>) {
            for entry in std::fs::read_dir(dir).into_iter().flatten().flatten() {
                let path = entry.path();
                if let Ok(target) = std::fs::read_link(&path) {
                    if let Some(blob) = target.file_name() {
                        out.insert(blob.to_owned());
                    }
                } else if path.is_dir() {
                    collect_links(&path, out);
                }
            }
        }
        collect_links(&snapshots, &mut in_use);
        for blob in blobs.difference(&in_use) {
            let _ = std::fs::remove_file(self.repo_dir(&model.repo).join("blobs").join(blob));
        }
        Ok(removed)
    }
}

#[cfg(unix)]
fn symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(windows)]
fn symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_file(target, link)
}

#[cfg(not(any(unix, windows)))]
fn symlink(_target: &Path, _link: &Path) -> std::io::Result<()> {
    Err(std::io::Error::other("symlinks are not supported"))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A fake cached repository: two quantizations, one split in two shards
    fn fake_cache(root: &Path) -> (HfCache, RepoId) {
        let cache = HfCache::new(root);
        let repo: RepoId = "owner/Model-GGUF".parse().unwrap();
        let commit = "0123456789abcdef";
        let files = [
            ("Model-Q8_0.gguf", "aaaa", 8),
            ("Q4_K_M/Model-Q4_K_M-00001-of-00002.gguf", "bbbb", 3),
            ("Q4_K_M/Model-Q4_K_M-00002-of-00002.gguf", "cccc", 2),
        ];
        std::fs::create_dir_all(cache.repo_dir(&repo).join("blobs")).unwrap();
        for (path, blob, size) in files {
            std::fs::write(cache.blob_path(&repo, blob), vec![0u8; size]).unwrap();
            cache.link_snapshot(&repo, commit, path, blob).unwrap();
        }
        cache.write_ref(&repo, DEFAULT_REVISION, commit).unwrap();
        (cache, repo)
    }

    #[test]
    fn resolve_list_and_remove() {
        let root = std::env::temp_dir().join(format!("skelm-hf-cache-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let (cache, repo) = fake_cache(&root);

        let default = HfModelRef {
            repo: repo.clone(),
            quant: None,
        };
        let path = cache.resolve(&default).unwrap();
        assert!(path.ends_with("Q4_K_M/Model-Q4_K_M-00001-of-00002.gguf"));
        assert_eq!(std::fs::metadata(&path).unwrap().len(), 3);

        let models = cache.list_models();
        assert_eq!(models.len(), 2);
        assert_eq!(models[0].model.to_string(), "hf.co/owner/Model-GGUF:Q4_K_M");
        assert_eq!(models[0].size, 5);

        let q8 = HfModelRef {
            repo: repo.clone(),
            quant: Some("q8_0".into()),
        };
        assert_eq!(cache.remove(&q8).unwrap().len(), 1);
        assert!(cache.resolve(&q8).is_err());
        assert!(!cache.blob_path(&repo, "aaaa").exists());
        assert!(cache.blob_path(&repo, "bbbb").exists());
        assert_eq!(cache.list_models().len(), 1);

        std::fs::remove_dir_all(&root).unwrap();
    }
}
