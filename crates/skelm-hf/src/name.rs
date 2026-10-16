//! Repository ids and model names.

use std::fmt;
use std::str::FromStr;

use crate::HfError;

/// A model repository on the Hub, `{owner}/{name}`
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RepoId(String);

impl RepoId {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The directory of the repository in the cache, `models--{owner}--{name}`
    pub fn cache_folder(&self) -> String {
        format!("models--{}", self.0.replace('/', "--"))
    }

    /// The repository of a cache directory name, see [`RepoId::cache_folder`]
    pub fn from_cache_folder(folder: &str) -> Option<RepoId> {
        let rest = folder.strip_prefix("models--")?;
        let (owner, name) = rest.split_once("--")?;
        format!("{owner}/{name}").parse().ok()
    }
}

impl FromStr for RepoId {
    type Err = HfError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let valid_part = |p: &str| {
            !p.is_empty()
                && !p.starts_with('.')
                && p.chars()
                    .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
        };
        match s.split_once('/') {
            Some((owner, name)) if valid_part(owner) && valid_part(name) => {
                Ok(RepoId(s.to_string()))
            }
            _ => Err(HfError::InvalidName(format!(
                "invalid repository '{s}': expected '<owner>/<name>'"
            ))),
        }
    }
}

impl fmt::Display for RepoId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Name prefixes of Hub models, as used by Ollama (`hf.co/...`)
const PREFIXES: [&str; 2] = ["hf.co/", "huggingface.co/"];

/// A GGUF model on the Hub: `hf.co/{owner}/{name}[:{quantization}]`
///
/// Without a quantization, `Q4_K_M` is picked if the repository has it, or
/// the only quantization it has.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct HfModelRef {
    pub repo: RepoId,
    /// the quantization, e.g. `Q4_K_M` (case insensitive)
    pub quant: Option<String>,
}

impl HfModelRef {
    /// Whether `name` designates a Hub model (it starts with `hf.co/`)
    pub fn is_hf_name(name: &str) -> bool {
        let lower = name.to_ascii_lowercase();
        PREFIXES.iter().any(|p| lower.starts_with(p))
    }

    /// Parse a name with or without the `hf.co/` prefix
    pub fn parse_unprefixed(name: &str) -> Result<Self, HfError> {
        let (repo, quant) = match name.rsplit_once(':') {
            Some((repo, quant)) if !quant.is_empty() => (repo, Some(quant.to_string())),
            Some(_) => {
                return Err(HfError::InvalidName(format!(
                    "invalid model name '{name}': empty quantization"
                )));
            }
            None => (name, None),
        };
        Ok(HfModelRef {
            repo: repo.parse()?,
            quant,
        })
    }
}

impl FromStr for HfModelRef {
    type Err = HfError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_ascii_lowercase();
        let rest = PREFIXES
            .iter()
            .find(|p| lower.starts_with(*p))
            .map(|p| &s[p.len()..])
            .ok_or_else(|| {
                HfError::InvalidName(format!(
                    "invalid model name '{s}': expected 'hf.co/<owner>/<name>[:<quantization>]'"
                ))
            })?;
        Self::parse_unprefixed(rest)
    }
}

impl fmt::Display for HfModelRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hf.co/{}", self.repo)?;
        if let Some(quant) = &self.quant {
            write!(f, ":{quant}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_model_names() {
        let r: HfModelRef = "hf.co/unsloth/Qwen3.5-2B-GGUF:Q4_K_M".parse().unwrap();
        assert_eq!(r.repo.as_str(), "unsloth/Qwen3.5-2B-GGUF");
        assert_eq!(r.quant.as_deref(), Some("Q4_K_M"));
        assert_eq!(r.to_string(), "hf.co/unsloth/Qwen3.5-2B-GGUF:Q4_K_M");

        let r: HfModelRef = "huggingface.co/bartowski/Llama-3.2-1B-GGUF"
            .parse()
            .unwrap();
        assert_eq!(r.repo.as_str(), "bartowski/Llama-3.2-1B-GGUF");
        assert_eq!(r.quant, None);

        let r = HfModelRef::parse_unprefixed("ggml-org/gemma-3-1b-it-GGUF:q8_0").unwrap();
        assert_eq!(r.quant.as_deref(), Some("q8_0"));

        assert!(HfModelRef::is_hf_name("HF.co/a/b"));
        assert!(!HfModelRef::is_hf_name("qwen2.5:7b"));
        assert!("qwen2.5:7b".parse::<HfModelRef>().is_err());
        assert!("hf.co/just-a-name".parse::<HfModelRef>().is_err());
        assert!("hf.co/a/b/c".parse::<HfModelRef>().is_err());
        assert!("hf.co/a/b:".parse::<HfModelRef>().is_err());
        assert!("hf.co/../b".parse::<HfModelRef>().is_err());
    }

    #[test]
    fn cache_folder_round_trip() {
        let repo: RepoId = "unsloth/Qwen3.5-2B-GGUF".parse().unwrap();
        assert_eq!(repo.cache_folder(), "models--unsloth--Qwen3.5-2B-GGUF");
        assert_eq!(RepoId::from_cache_folder(&repo.cache_folder()), Some(repo));
        assert_eq!(RepoId::from_cache_folder("datasets--a--b"), None);
    }
}
