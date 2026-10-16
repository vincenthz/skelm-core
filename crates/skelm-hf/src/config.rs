//! Hub endpoint, cache location and token, from the standard `HF_*`
//! environment variables (as `huggingface_hub` reads them).

use std::path::PathBuf;

use url::Url;

const DEFAULT_ENDPOINT: &str = "https://huggingface.co";

#[derive(Clone, Debug)]
pub struct HfConfig {
    /// The Hub, `HF_ENDPOINT` (default `https://huggingface.co`)
    pub endpoint: Url,
    /// The access token for gated / private repositories: `HF_TOKEN`, else
    /// the token saved by `huggingface-cli login` (`$HF_HOME/token`)
    pub token: Option<String>,
    /// The cache directory: `HF_HUB_CACHE`, else `$HF_HOME/hub`, with
    /// `HF_HOME` defaulting to `~/.cache/huggingface`
    pub cache_dir: PathBuf,
}

impl Default for HfConfig {
    fn default() -> Self {
        Self::from_env()
    }
}

impl HfConfig {
    pub fn from_env() -> Self {
        let var = |name: &str| std::env::var(name).ok().filter(|v| !v.is_empty());

        let endpoint = var("HF_ENDPOINT")
            .and_then(|e| Url::parse(&e).ok())
            .unwrap_or_else(|| Url::parse(DEFAULT_ENDPOINT).unwrap());

        let hf_home = var("HF_HOME").map(PathBuf::from).unwrap_or_else(|| {
            let cache = var("XDG_CACHE_HOME").map(PathBuf::from).unwrap_or_else(|| {
                std::env::home_dir()
                    .expect("cannot find the home directory")
                    .join(".cache")
            });
            cache.join("huggingface")
        });

        let cache_dir = var("HF_HUB_CACHE")
            .or_else(|| var("HUGGINGFACE_HUB_CACHE"))
            .map(PathBuf::from)
            .unwrap_or_else(|| hf_home.join("hub"));

        let token = var("HF_TOKEN")
            .or_else(|| var("HUGGING_FACE_HUB_TOKEN"))
            .or_else(|| {
                std::fs::read_to_string(hf_home.join("token"))
                    .ok()
                    .map(|t| t.trim().to_string())
                    .filter(|t| !t.is_empty())
            });

        HfConfig {
            endpoint,
            token,
            cache_dir,
        }
    }
}
