//! Grouping the GGUF files of a repository into quantizations.

use crate::HfError;

/// A file of a repository (or of a cached snapshot)
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RepoFile {
    /// path in the repository, e.g. `Q4_K_M/model-Q4_K_M-00001-of-00002.gguf`
    pub path: String,
    pub size: u64,
    /// the sha256 of an LFS file, which is also its cache blob name
    pub sha256: Option<String>,
    /// the git blob id, the cache blob name of a non-LFS file
    pub blob_id: Option<String>,
}

impl RepoFile {
    fn file_name(&self) -> &str {
        self.path.rsplit('/').next().unwrap_or(&self.path)
    }

    pub fn is_gguf(&self) -> bool {
        self.path.to_ascii_lowercase().ends_with(".gguf")
    }

    /// A multimodal projector (`mmproj-*.gguf`), not a model by itself
    pub fn is_projector(&self) -> bool {
        self.file_name().to_ascii_lowercase().starts_with("mmproj")
    }
}

/// One quantization of a model: a single GGUF file, or the shards of a split one
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GgufVariant {
    /// the quantization, upper case, e.g. `Q4_K_M` or `UD-Q4_K_XL`
    pub quant: String,
    /// the files, shards in order
    pub files: Vec<RepoFile>,
}

impl GgufVariant {
    pub fn size(&self) -> u64 {
        self.files.iter().map(|f| f.size).sum()
    }

    /// The file to load: the only file, or the first shard (llama.cpp loads
    /// the following shards from it)
    pub fn primary(&self) -> &RepoFile {
        &self.files[0]
    }
}

/// `-00001-of-00003` at the end of a file stem: (stem without it, shard index)
fn split_shard(stem: &str) -> (&str, Option<u32>) {
    let bytes = stem.as_bytes();
    // "-NNNNN-of-NNNNN" is 15 characters
    if bytes.len() > 15 {
        let tail = &stem[stem.len() - 15..];
        let digits = |s: &str| s.len() == 5 && s.bytes().all(|b| b.is_ascii_digit());
        if tail.starts_with('-')
            && &tail[6..10] == "-of-"
            && digits(&tail[1..6])
            && digits(&tail[10..])
        {
            return (&stem[..stem.len() - 15], tail[1..6].parse().ok());
        }
    }
    (stem, None)
}

/// The number of shards of a split GGUF file (`...-00001-of-00003.gguf`)
pub fn shard_count(path: &str) -> Option<u32> {
    let stem = path.strip_suffix(".gguf")?;
    split_shard(stem).1?;
    stem[stem.len() - 5..].parse().ok()
}

fn is_quant(part: &str) -> bool {
    let up = part.to_ascii_uppercase();
    let numbered = |prefix: &str| {
        up.strip_prefix(prefix)
            .and_then(|r| r.chars().next())
            .is_some_and(|c| c.is_ascii_digit())
    };
    numbered("Q")
        || numbered("IQ")
        || numbered("TQ")
        || up.starts_with("MXFP")
        || up.starts_with("NVFP")
        || matches!(up.as_str(), "F16" | "BF16" | "F32" | "FP16" | "FP32")
}

/// The quantization of a GGUF file from its name, e.g. `Qwen3.5-2B-Q4_K_M.gguf`
/// gives `Q4_K_M`, `Qwen3.5-2B-UD-Q4_K_XL.gguf` gives `UD-Q4_K_XL`
pub fn quant_of(path: &str) -> Option<String> {
    let name = path.rsplit('/').next().unwrap_or(path);
    let lower = name.to_ascii_lowercase();
    let stem = &name[..lower.strip_suffix(".gguf")?.len()];
    let (stem, _) = split_shard(stem);
    let parts: Vec<&str> = stem.split(['-', '.']).collect();
    let i = parts.iter().rposition(|p| is_quant(p))?;
    let quant = parts[i].to_ascii_uppercase();
    if i > 0 && parts[i - 1].eq_ignore_ascii_case("UD") {
        Some(format!("UD-{quant}"))
    } else {
        Some(quant)
    }
}

/// The quantizations available in `files`, smallest first
pub fn gguf_variants(files: &[RepoFile]) -> Vec<GgufVariant> {
    let mut variants: Vec<GgufVariant> = Vec::new();
    let mut shards: Vec<(String, u32, RepoFile)> = Vec::new();
    for file in files.iter().filter(|f| f.is_gguf() && !f.is_projector()) {
        let Some(quant) = quant_of(&file.path) else {
            continue;
        };
        let stem = &file.path[..file.path.len() - ".gguf".len()];
        let shard = split_shard(stem).1.unwrap_or(0);
        shards.push((quant, shard, file.clone()));
    }
    shards.sort_by(|a, b| (&a.0, a.1).cmp(&(&b.0, b.1)));
    for (quant, _, file) in shards {
        match variants.last_mut() {
            Some(v) if v.quant == quant => v.files.push(file),
            _ => variants.push(GgufVariant {
                quant,
                files: vec![file],
            }),
        }
    }
    variants.sort_by_key(|v| v.size());
    variants
}

/// The multimodal projectors of `files`
pub fn projectors(files: &[RepoFile]) -> Vec<&RepoFile> {
    files
        .iter()
        .filter(|f| f.is_gguf() && f.is_projector())
        .collect()
}

/// The default quantization when a name doesn't give one (as Ollama)
pub const DEFAULT_QUANT: &str = "Q4_K_M";

/// Pick the variant designated by `quant` (case insensitive); without one,
/// [`DEFAULT_QUANT`], or the only variant
pub fn select_variant<'a>(
    variants: &'a [GgufVariant],
    quant: Option<&str>,
) -> Result<&'a GgufVariant, HfError> {
    let available = || variants.iter().map(|v| v.quant.clone()).collect::<Vec<_>>();
    match quant {
        Some(q) => variants
            .iter()
            .find(|v| v.quant.eq_ignore_ascii_case(q))
            .ok_or_else(|| HfError::NoSuchQuant {
                quant: q.to_string(),
                available: available(),
            }),
        None => match variants {
            [] => Err(HfError::NoGguf),
            [only] => Ok(only),
            _ => variants
                .iter()
                .find(|v| v.quant.eq_ignore_ascii_case(DEFAULT_QUANT))
                .ok_or_else(|| HfError::QuantRequired(available())),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn file(path: &str, size: u64) -> RepoFile {
        RepoFile {
            path: path.into(),
            size,
            sha256: None,
            blob_id: None,
        }
    }

    #[test]
    fn quant_from_file_names() {
        assert_eq!(
            quant_of("Qwen3.5-2B-Q4_K_M.gguf").as_deref(),
            Some("Q4_K_M")
        );
        assert_eq!(
            quant_of("Qwen3.5-2B-UD-Q4_K_XL.gguf").as_deref(),
            Some("UD-Q4_K_XL")
        );
        assert_eq!(quant_of("Qwen3.5-2B-BF16.gguf").as_deref(), Some("BF16"));
        assert_eq!(
            quant_of("Qwen3.5-2B-IQ4_NL.gguf").as_deref(),
            Some("IQ4_NL")
        );
        assert_eq!(
            quant_of("qwen2.5-0.5b-instruct-q4_k_m.gguf").as_deref(),
            Some("Q4_K_M")
        );
        assert_eq!(quant_of("gemma-3-1b-it.Q8_0.gguf").as_deref(), Some("Q8_0"));
        assert_eq!(
            quant_of("Q4_K_M/Big-Model-Q4_K_M-00002-of-00003.gguf").as_deref(),
            Some("Q4_K_M")
        );
        assert_eq!(quant_of("model.gguf"), None);
        assert_eq!(quant_of("README.md"), None);
    }

    #[test]
    fn variants_group_shards() {
        let files = [
            file("README.md", 1),
            file("m-Q8_0.gguf", 800),
            file("Q4_K_M/m-Q4_K_M-00002-of-00002.gguf", 200),
            file("Q4_K_M/m-Q4_K_M-00001-of-00002.gguf", 300),
            file("mmproj-F16.gguf", 50),
        ];
        let variants = gguf_variants(&files);
        assert_eq!(variants.len(), 2);
        assert_eq!(variants[0].quant, "Q4_K_M");
        assert_eq!(variants[0].size(), 500);
        assert_eq!(
            variants[0].primary().path,
            "Q4_K_M/m-Q4_K_M-00001-of-00002.gguf"
        );
        assert_eq!(variants[1].quant, "Q8_0");
        assert_eq!(projectors(&files).len(), 1);

        assert_eq!(select_variant(&variants, None).unwrap().quant, "Q4_K_M");
        assert_eq!(
            select_variant(&variants, Some("q8_0")).unwrap().quant,
            "Q8_0"
        );
        assert!(select_variant(&variants, Some("Q2_K")).is_err());
        assert!(select_variant(&variants[1..], None).is_ok());
        assert!(select_variant(&[], None).is_err());
    }
}
