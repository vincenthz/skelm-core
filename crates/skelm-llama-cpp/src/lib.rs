//!

// NOT MEMORY SAFE YET / it will explode if you look at it side ways.

mod batch;
mod batch_ext;
mod context;
mod log;
mod lora;
mod model;
mod sampler;
mod speculative;
mod token;
mod tokendata;
mod vocab;

pub use context::{CacheType, Context, ContextCreateError, ContextParams, DecodeError, FlashAttn};
pub use log::{LogKey, LogLevel, llama_logging};
pub use lora::{LoraAdapter, LoraError};
pub use model::{LoadMode, Model, ModelLoadError, ModelParams, SplitMode};
pub use sampler::{
    Sampler, SamplerChain, SamplerDistance, SamplerGreedy, SamplerMinP, SamplerMirostatV1,
    SamplerMirostatV2, SamplerRandom, SamplerTemperature,
};
pub use speculative::{
    DraftParams, Drafter, SpeculativeConfig, SpeculativeError, SpeculativeStats, Step,
};
pub use token::Token;
pub use tokendata::{TokenData, TokenDataArray};
pub use vocab::{TokenAttr, Vocab, VocabType};

// TODO un-hardcode
pub const PACKAGE_VERSION: &str = "0.1.0";
