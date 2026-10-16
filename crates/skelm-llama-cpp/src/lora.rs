//! LoRA adapters.
//!
//! An adapter is loaded against a [`Model`] and then applied, with a scale, to
//! any number of contexts of that model with [`Context::set_lora_adapters`].

use std::ffi::CString;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use skelm_llama_cpp_sys::llama;
use thiserror::Error;

use crate::model::read_meta_string;
use crate::{Context, Model};

#[derive(Debug, Error)]
pub enum LoraError {
    #[error("invalid LoRA adapter path {0:?}")]
    InvalidPath(PathBuf),
    #[error("failed to load LoRA adapter {0:?} (consult logging for reasons)")]
    LoadFailed(PathBuf),
    #[error("LoRA adapter {0:?} was loaded for a different model")]
    ModelMismatch(PathBuf),
    #[error("failed to apply LoRA adapters to the context")]
    ApplyFailed,
}

pub(crate) struct LoraPtr(*mut llama::llama_adapter_lora);

unsafe impl Send for LoraPtr {}
unsafe impl Sync for LoraPtr {}

/// A LoRA adapter loaded for a model. Cheap to clone.
///
/// The adapter keeps its model alive: llama.cpp frees the adapters of a model
/// together with it, so the adapter must not outlive it.
#[derive(Clone)]
pub struct LoraAdapter {
    // declared before `model` so the adapter is freed before the model
    ptr: Arc<LoraPtr>,
    model: Model,
    path: PathBuf,
}

impl Drop for LoraPtr {
    fn drop(&mut self) {
        unsafe { llama::llama_adapter_lora_free(self.0) }
    }
}

impl LoraAdapter {
    /// Load a GGUF LoRA adapter for `model`
    pub fn load(model: &Model, path: impl AsRef<Path>) -> Result<Self, LoraError> {
        let path = path.as_ref();
        let cpath =
            crate::model::path_to_cpath(path).ok_or_else(|| LoraError::InvalidPath(path.into()))?;
        let ptr = unsafe { llama::llama_adapter_lora_init(model.ptr.0, cpath.as_ptr()) };
        if ptr.is_null() {
            return Err(LoraError::LoadFailed(path.into()));
        }
        Ok(LoraAdapter {
            ptr: Arc::new(LoraPtr(ptr)),
            model: model.clone(),
            path: path.into(),
        })
    }

    /// The file the adapter was loaded from
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// The model the adapter was loaded for
    pub fn model(&self) -> &Model {
        &self.model
    }

    /// A GGUF metadata value of the adapter, as a string (e.g. `adapter.lora.alpha`)
    pub fn meta_val_str(&self, key: &str) -> Option<String> {
        let key = CString::new(key).ok()?;
        read_meta_string(|buf, len| unsafe {
            llama::llama_adapter_meta_val_str(self.ptr.0, key.as_ptr(), buf, len)
        })
    }

    /// All the scalar GGUF metadata of the adapter, as `(key, value)` strings
    pub fn meta(&self) -> Vec<(String, String)> {
        let n = unsafe { llama::llama_adapter_meta_count(self.ptr.0) };
        (0..n)
            .filter_map(|i| {
                let key = read_meta_string(|buf, len| unsafe {
                    llama::llama_adapter_meta_key_by_index(self.ptr.0, i, buf, len)
                })?;
                let value = read_meta_string(|buf, len| unsafe {
                    llama::llama_adapter_meta_val_str_by_index(self.ptr.0, i, buf, len)
                })?;
                Some((key, value))
            })
            .collect()
    }
}

impl Context {
    /// Set the LoRA adapters of this context, each with its scale (1.0 = the
    /// adapter as trained, 0.0 = disabled). Replaces the previous set; an empty
    /// slice removes all adapters.
    pub fn set_lora_adapters(&mut self, adapters: &[(LoraAdapter, f32)]) -> Result<(), LoraError> {
        for (adapter, _) in adapters {
            if !adapter.model.same_as(&self.model) {
                return Err(LoraError::ModelMismatch(adapter.path.clone()));
            }
        }
        let mut ptrs: Vec<*mut llama::llama_adapter_lora> =
            adapters.iter().map(|(a, _)| a.ptr.0).collect();
        let mut scales: Vec<f32> = adapters.iter().map(|(_, s)| *s).collect();
        let ret = unsafe {
            llama::llama_set_adapters_lora(
                self.ptr.0,
                ptrs.as_mut_ptr(),
                ptrs.len(),
                scales.as_mut_ptr(),
            )
        };
        if ret != 0 {
            return Err(LoraError::ApplyFailed);
        }
        // keep the adapters alive for as long as the context uses them
        self.lora_adapters = adapters.iter().map(|(a, _)| a.clone()).collect();
        Ok(())
    }

    /// The LoRA adapters currently set on this context
    pub fn lora_adapters(&self) -> &[LoraAdapter] {
        &self.lora_adapters
    }
}
