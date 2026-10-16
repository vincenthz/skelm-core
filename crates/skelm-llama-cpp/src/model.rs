use skelm_llama_cpp_sys::llama;
use std::ffi::{CStr, CString};
use std::path::{Path, PathBuf};
use std::ptr::null_mut;
use std::sync::Arc;
use thiserror::Error;

use crate::context::ContextParams;
use crate::vocab::{Vocab, VocabPtr};

use super::context::{Context, ContextCreateError};

#[derive(Clone)]
pub struct Model {
    pub(crate) ptr: Arc<ModelPtr>,
    /// whether the MTP (nextn) layers were loaded, see `ModelParams::load_mtp`
    pub(crate) mtp_loaded: bool,
}

pub struct ModelPtr(pub(crate) *mut llama::llama_model);

unsafe impl Send for ModelPtr {}
unsafe impl Sync for ModelPtr {}

impl Drop for ModelPtr {
    fn drop(&mut self) {
        unsafe {
            llama::llama_model_free(self.0);
        }
    }
}

/// How to split the model across multiple GPUs
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SplitMode {
    /// single GPU (`ModelParams::main_gpu`)
    None,
    /// split layers and KV across GPUs
    Layer,
    /// split layers and KV across GPUs, use tensor parallelism if supported
    Row,
    Tensor,
}

impl SplitMode {
    fn to_c(self) -> llama::llama_split_mode {
        match self {
            SplitMode::None => llama::llama_split_mode::LLAMA_SPLIT_MODE_NONE,
            SplitMode::Layer => llama::llama_split_mode::LLAMA_SPLIT_MODE_LAYER,
            SplitMode::Row => llama::llama_split_mode::LLAMA_SPLIT_MODE_ROW,
            SplitMode::Tensor => llama::llama_split_mode::LLAMA_SPLIT_MODE_TENSOR,
        }
    }

    fn from_c(value: llama::llama_split_mode) -> Self {
        match value {
            llama::llama_split_mode::LLAMA_SPLIT_MODE_NONE => SplitMode::None,
            llama::llama_split_mode::LLAMA_SPLIT_MODE_ROW => SplitMode::Row,
            llama::llama_split_mode::LLAMA_SPLIT_MODE_TENSOR => SplitMode::Tensor,
            _ => SplitMode::Layer,
        }
    }
}

/// How to load the model weights
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoadMode {
    /// auto-detect based on device capabilities
    Auto,
    /// no special loading mode (read the file)
    None,
    /// memory map the model
    Mmap,
    /// force the system to keep the model in RAM rather than swapping or compressing
    Mlock,
    /// mmap + mlock
    MmapMlock,
    /// use direct I/O if available
    DirectIo,
}

impl LoadMode {
    fn to_c(self) -> llama::llama_load_mode {
        match self {
            LoadMode::Auto => llama::llama_load_mode::LLAMA_LOAD_MODE_AUTO,
            LoadMode::None => llama::llama_load_mode::LLAMA_LOAD_MODE_NONE,
            LoadMode::Mmap => llama::llama_load_mode::LLAMA_LOAD_MODE_MMAP,
            LoadMode::Mlock => llama::llama_load_mode::LLAMA_LOAD_MODE_MLOCK,
            LoadMode::MmapMlock => llama::llama_load_mode::LLAMA_LOAD_MODE_MMAP_MLOCK,
            LoadMode::DirectIo => llama::llama_load_mode::LLAMA_LOAD_MODE_DIRECT_IO,
        }
    }

    fn from_c(value: llama::llama_load_mode) -> Self {
        match value {
            llama::llama_load_mode::LLAMA_LOAD_MODE_NONE => LoadMode::None,
            llama::llama_load_mode::LLAMA_LOAD_MODE_MMAP => LoadMode::Mmap,
            llama::llama_load_mode::LLAMA_LOAD_MODE_MLOCK => LoadMode::Mlock,
            llama::llama_load_mode::LLAMA_LOAD_MODE_MMAP_MLOCK => LoadMode::MmapMlock,
            llama::llama_load_mode::LLAMA_LOAD_MODE_DIRECT_IO => LoadMode::DirectIo,
            _ => LoadMode::Auto,
        }
    }
}

/// Model loading parameters; `Default` uses the llama.cpp defaults
#[derive(Clone, Debug)]
pub struct ModelParams {
    /// only load the vocabulary, no weights
    pub vocab_only: bool,
    /// number of layers to store in VRAM, a negative value means all layers
    pub n_gpu_layers: i32,
    /// how to split the model across multiple GPUs
    pub split_mode: SplitMode,
    /// the GPU used for the entire model when `split_mode` is `SplitMode::None`
    pub main_gpu: i32,
    /// how to load the model weights
    pub load_mode: LoadMode,
    /// validate model tensor data
    pub check_tensors: bool,
    /// load the MTP (multi-token prediction, a.k.a. nextn) layers, needed for
    /// MTP speculative decoding; they are skipped otherwise
    pub load_mtp: bool,
}

impl Default for ModelParams {
    fn default() -> Self {
        let params = unsafe { llama::llama_model_default_params() };
        Self {
            vocab_only: params.vocab_only,
            n_gpu_layers: params.n_gpu_layers,
            split_mode: SplitMode::from_c(params.split_mode),
            main_gpu: params.main_gpu,
            load_mode: LoadMode::from_c(params.load_mode),
            check_tensors: params.check_tensors,
            load_mtp: params.load_mtp,
        }
    }
}

impl ModelParams {
    fn as_c(&self) -> llama::llama_model_params {
        let mut params = unsafe { llama::llama_model_default_params() };

        params.vocab_only = self.vocab_only;
        params.n_gpu_layers = self.n_gpu_layers;
        params.split_mode = self.split_mode.to_c();
        params.main_gpu = self.main_gpu;
        params.load_mode = self.load_mode.to_c();
        params.check_tensors = self.check_tensors;
        params.load_mtp = self.load_mtp;
        params
    }
}

#[derive(Debug, Error)]
pub enum ModelLoadError {
    #[error(
        "invalid model path {0:?}: it must not contain NUL bytes, and must be UTF-8 on Windows"
    )]
    InvalidPath(PathBuf),
    #[error("Model Load Error (consult logging for reasons)")]
    LoadFailed,
}

impl Model {
    pub fn load(path: impl AsRef<Path>, params: &ModelParams) -> Result<Self, ModelLoadError> {
        let path = path.as_ref();
        let cpath = path_to_cpath(path).ok_or_else(|| ModelLoadError::InvalidPath(path.into()))?;
        let c_params = params.as_c();
        let ret = unsafe { llama::llama_model_load_from_file(cpath.as_ptr(), c_params) };
        if ret.is_null() {
            return Err(ModelLoadError::LoadFailed);
        }

        Ok(Model {
            ptr: Arc::new(ModelPtr(ret)),
            mtp_loaded: params.load_mtp && !params.vocab_only,
        })
    }

    pub fn vocab(&self) -> Vocab {
        unsafe {
            Vocab {
                model: self.clone(),
                ptr: Arc::new(VocabPtr(llama::llama_model_get_vocab(self.ptr.0))),
            }
        }
    }

    pub fn n_ctx_train(&self) -> i32 {
        unsafe { llama::llama_model_n_ctx_train(self.ptr.0) }
    }

    /// Get the model type
    pub fn description(&self) -> String {
        let sz = unsafe { llama::llama_model_desc(self.ptr.0, null_mut(), 0) as usize };
        let mut buf = vec![0; sz];
        unsafe {
            llama::llama_model_desc(
                self.ptr.0,
                buf.as_mut_ptr() as *mut ::std::os::raw::c_char,
                sz,
            )
        };
        String::from_utf8(buf).unwrap()
    }

    pub fn has_encoder(&self) -> bool {
        unsafe { llama::llama_model_has_encoder(self.ptr.0) }
    }

    pub fn has_decoder(&self) -> bool {
        unsafe { llama::llama_model_has_decoder(self.ptr.0) }
    }

    pub fn n_embd(&self) -> usize {
        unsafe { llama::llama_model_n_embd(self.ptr.0) as usize }
    }

    /// Whether `other` is a handle to the same loaded model
    pub fn same_as(&self, other: &Model) -> bool {
        Arc::ptr_eq(&self.ptr, &other.ptr)
    }

    /// Width of the hidden-state rows a model outputs (the MTP input width)
    pub fn n_embd_out(&self) -> usize {
        unsafe { llama::llama_model_n_embd_out(self.ptr.0) as usize }
    }

    /// Number of MTP (nextn) layers in the model, 0 if it has none
    pub fn n_layer_nextn(&self) -> usize {
        unsafe { llama::llama_model_n_layer_nextn(self.ptr.0).max(0) as usize }
    }

    /// Whether the model can drive MTP speculative decoding: it has MTP layers,
    /// and they were loaded (`ModelParams::load_mtp`)
    pub fn has_mtp(&self) -> bool {
        self.mtp_loaded && self.n_layer_nextn() > 0
    }

    /// The architecture name from the GGUF metadata (e.g. `qwen2`)
    pub fn architecture(&self) -> Option<String> {
        self.meta_val_str("general.architecture")
    }

    /// A GGUF metadata value of the model, as a string
    pub fn meta_val_str(&self, key: &str) -> Option<String> {
        let key = CString::new(key).ok()?;
        read_meta_string(|buf, len| unsafe {
            llama::llama_model_meta_val_str(self.ptr.0, key.as_ptr(), buf, len)
        })
    }

    /// Create a new context for this model
    pub fn new_context(&self, params: &ContextParams) -> Result<Context, ContextCreateError> {
        Context::new(self.clone(), params)
    }

    pub fn chat_template(&self) -> Option<String> {
        unsafe {
            let ptr = llama::llama_model_chat_template(self.ptr.0, core::ptr::null());
            if ptr.is_null() {
                return None;
            }
            let cstr = CStr::from_ptr(ptr);
            Some(cstr.to_string_lossy().to_string())
        }
    }
}

/// Call a llama.cpp "write a string into `buf`" function, growing the buffer
/// until the value fits. These return the full length, or -1 on failure.
pub(crate) fn read_meta_string(
    mut f: impl FnMut(*mut ::std::os::raw::c_char, usize) -> i32,
) -> Option<String> {
    let mut buf = vec![0u8; 256];
    loop {
        let n = f(buf.as_mut_ptr() as *mut ::std::os::raw::c_char, buf.len());
        if n < 0 {
            return None;
        }
        if (n as usize) < buf.len() {
            buf.truncate(n as usize);
            return Some(String::from_utf8_lossy(&buf).into_owned());
        }
        buf.resize(n as usize + 1, 0);
    }
}

/// Convert a path to the NUL-terminated string llama.cpp expects: the raw bytes
/// on unix, UTF-8 elsewhere (on Windows `ggml_fopen` converts UTF-8 to UTF-16).
#[cfg(unix)]
pub(crate) fn path_to_cpath(path: &Path) -> Option<CString> {
    use std::os::unix::ffi::OsStrExt;
    CString::new(path.as_os_str().as_bytes()).ok()
}

#[cfg(not(unix))]
pub(crate) fn path_to_cpath(path: &Path) -> Option<CString> {
    CString::new(path.to_str()?).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpath_is_nul_terminated() {
        let cpath = path_to_cpath(Path::new("/tmp/model.gguf")).unwrap();
        assert_eq!(cpath.as_bytes_with_nul(), b"/tmp/model.gguf\0");
    }

    #[test]
    fn cpath_rejects_interior_nul() {
        assert!(path_to_cpath(Path::new("/tmp/mo\0del.gguf")).is_none());
    }

    #[test]
    fn default_params_round_trip() {
        let c = unsafe { llama::llama_model_default_params() };
        let back = ModelParams::default().as_c();
        assert_eq!(back.vocab_only, c.vocab_only);
        assert_eq!(back.n_gpu_layers, c.n_gpu_layers);
        assert_eq!(back.split_mode, c.split_mode);
        assert_eq!(back.main_gpu, c.main_gpu);
        assert_eq!(back.load_mode, c.load_mode);
        assert_eq!(back.check_tensors, c.check_tensors);
        assert_eq!(back.load_mtp, c.load_mtp);
    }
}
