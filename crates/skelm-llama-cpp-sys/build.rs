use std::{
    env,
    io::Write,
    path::{Path, PathBuf},
};

use bindgen::Builder;
use cc::Build;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(unused)]
enum Backend {
    Metal,
}

#[cfg(target_os = "macos")]
const WARNING_FLAGS: &[&str] = &[
    //"-Wshadow",
    "-Wstrict-prototypes",
    "-Wpointer-arith",
    "-Wmissing-prototypes",
    "-Werror=implicit-int",
    "-Werror=implicit-function-declaration",
    "-Wall",
    "-Wextra",
    "-Wpedantic",
    "-Wcast-qual",
    "-Wno-unused-function",
    "-Wunreachable-code-break",
    "-Wunreachable-code-return",
    //"-Wdouble-promotion",
];

#[cfg(not(target_os = "macos"))]
const WARNING_FLAGS: &[&str] = &[
    //"-Wshadow",
    "-Wstrict-prototypes",
    "-Wpointer-arith",
    "-Wmissing-prototypes",
    "-Werror=implicit-int",
    "-Werror=implicit-function-declaration",
    "-Wall",
    "-Wextra",
    "-Wpedantic",
    "-Wcast-qual",
    "-Wno-unused-function",
    //"-Wdouble-promotion",
];

const GGML_VERSION_MAJOR: u64 = 0;
const GGML_VERSION_MINOR: u64 = 13;
const GGML_VERSION_PATCH: u64 = 0;
const GGML_COMMIT: &str = "\"35c9b1f3\"";
#[allow(unused)]
const GGML_COMMIT_LONG: &str = "35c9b1f39ebe5a7bb83986d64415a079218be78d";

fn ggml_version() -> String {
    format!(
        "{}.{}.{}",
        GGML_VERSION_MAJOR, GGML_VERSION_MINOR, GGML_VERSION_PATCH
    )
}

fn main() {
    let lib_path = std::path::PathBuf::from("libs");

    if !lib_path.exists() {
        panic!("cannot compile without cloning the llama.cpp git submodule")
    }

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings(&lib_path, &out_path);
    let ggml_objects = lib_ggml(&lib_path, &out_path);
    lib_llama(&lib_path, ggml_objects);
}

fn bindings(lib_path: &Path, out_path: &Path) {
    let ggml_path = lib_path.join("ggml");
    let ggml_include_path = ggml_path.join("include");
    let llama_include_path = lib_path.join("include");

    let ggml_bindings = Builder::default()
        .header(ggml_include_path.join("ggml.h").to_string_lossy())
        .allowlist_function("ggml_.*")
        .allowlist_type("ggml_.*")
        .derive_debug(true)
        .derive_copy(false)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true,
        })
        .generate()
        .expect("Unable to generate bindings");

    let llama_bindings = Builder::default()
        .header(llama_include_path.join("llama.h").to_string_lossy())
        .allowlist_function("llama_.*")
        .allowlist_type("llama_.*")
        .clang_arg(format!("-I{}", ggml_include_path.display()))
        .derive_debug(true)
        .derive_copy(false)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true,
        })
        .generate()
        .expect("Unable to generate bindings");

    ggml_bindings
        .write_to_file(out_path.join("bindings_ggml.rs"))
        .expect("cannot write ggml bindings");
    llama_bindings
        .write_to_file(out_path.join("bindings_llama.rs"))
        .expect("cannot write llama bindings")
}

fn lib_ggml(lib_path: &Path, out_path: &Path) -> Vec<PathBuf> {
    let ggml_path = lib_path.join("ggml");
    let src_path = ggml_path.join("src");
    let include_path = ggml_path.join("include");

    let c_files = ["ggml.c", "ggml-alloc.c", "ggml-quants.c"];

    let cpp_files = [
        "ggml.cpp",
        "ggml-backend.cpp",
        "ggml-backend-dl.cpp",
        "ggml-backend-meta.cpp",
        "ggml-opt.cpp",
        "ggml-threading.cpp",
        "gguf.cpp",
    ];

    // create a common build
    let mut common = Build::new();
    common.include(include_path);
    common.include(&src_path);
    common.opt_level(3);
    common.flags(WARNING_FLAGS);

    common.define("GGML_SCHED_MAX_COPIES", "4");
    common.define("GGML_SHARED", None);
    common.define("GGML_BUILD", None);
    common.define(
        "GGML_VERSION",
        Some(format!("\"{}\"", ggml_version()).as_str()),
    );
    common.define("GGML_COMMIT", GGML_COMMIT);
    common.define("ggml_base_EXPORTS", None);

    common.define("GGML_USE_CPU", None);

    #[cfg(target_os = "linux")]
    {
        common.define("_XOPEN_SOURCE", "\"600\"");
        common.define("_GNU_SOURCE", None);
    }

    let metaldefs = [
        "GGML_USE_METAL",
        "GGML_METAL_EMBED_LIBRARY",
        "ggml_metal_EXPORTS",
    ];

    // specialize for CPP
    let mut cpp = common.clone();

    cpp.cpp(true).std("c++17");
    cpp.files(cpp_files.into_iter().map(|f| src_path.join(f)));

    // specialize for C
    let mut c = common.clone();
    c.cpp(false).std("c11");
    c.files(c_files.into_iter().map(|f| src_path.join(f)));

    let mut backend_intermediates = Vec::new();

    // support cpu
    {
        let backend_dir = src_path.join("ggml-cpu");
        c.include(&backend_dir);
        cpp.include(&backend_dir);
        let c_files = ["ggml-cpu.c", "quants.c", "arch/arm/quants.c"];
        let cpp_files = [
            "ggml-cpu.cpp",
            "repack.cpp",
            "hbm.cpp",
            "traits.cpp",
            "amx/amx.cpp",
            "amx/mmq.cpp",
            "binary-ops.cpp",
            "unary-ops.cpp",
            "vec.cpp",
            "ops.cpp",
            "llamafile/sgemm.cpp",
            "arch/arm/repack.cpp",
        ];

        c.files(c_files.into_iter().map(|f| backend_dir.join(f)));
        cpp.files(cpp_files.into_iter().map(|f| backend_dir.join(f)));
    }

    #[cfg(target_os = "macos")]
    let backend = Some(Backend::Metal);
    #[cfg(not(target_os = "macos"))]
    let backend = None;

    match backend {
        Some(Backend::Metal) => {
            for metaldef in metaldefs {
                c.define(metaldef, None);
                cpp.define(metaldef, None);
            }

            let backend_dir = src_path.join("ggml-metal");
            c.include(&backend_dir);
            cpp.include(&backend_dir);

            let c_files = ["ggml-metal-context.m", "ggml-metal-device.m"];
            let cpp_files = [
                "ggml-metal.cpp",
                "ggml-metal-device.cpp",
                "ggml-metal-common.cpp",
                "ggml-metal-ops.cpp",
            ];

            cpp.define("GGML_METAL_EMBED_LIBRARY", None);

            let ggml_common_h_path = src_path.join("ggml-common.h");
            let ggml_common_h_data = std::fs::read_to_string(ggml_common_h_path).unwrap();

            let ggml_metal_impl_h_path = backend_dir.join("ggml-metal-impl.h");
            let ggml_metal_impl_h_data = std::fs::read_to_string(ggml_metal_impl_h_path).unwrap();

            let embed_file_name = "ggml-metal-embed.s";

            let metal_file = "ggml-metal.metal";
            let metal_file_path = backend_dir.join(metal_file);
            let metal_file_out = out_path.join(metal_file);

            let metal_file_data = std::fs::read_to_string(metal_file_path).unwrap();

            let replace_embed_ggml = "__embed_ggml-common.h__\n";
            let replace_ggml_metal = "#include \"ggml-metal-impl.h\"\n";

            let metal_file_data = metal_file_data
                .replace(replace_embed_ggml, &ggml_common_h_data)
                .replace(replace_ggml_metal, &ggml_metal_impl_h_data);

            let mut out_metal_file = std::fs::File::create(&metal_file_out).unwrap();
            out_metal_file
                .write_all(metal_file_data.as_bytes())
                .unwrap();

            let prefix = r#".section __DATA,__ggml_metallib
.globl _ggml_metallib_start
_ggml_metallib_start:
"#;
            let suffix = r#".globl _ggml_metallib_end
_ggml_metallib_end:
"#;

            let embed_file_path = out_path.join(embed_file_name);
            let mut out_embed_file = std::fs::File::create(&embed_file_path).unwrap();

            let embed_path = format!(".incbin \"{}\"\n", metal_file_out.display());
            out_embed_file.write_all(prefix.as_bytes()).unwrap();
            out_embed_file.write_all(embed_path.as_bytes()).unwrap();
            out_embed_file.write_all(suffix.as_bytes()).unwrap();

            let mut s = common.clone();
            s.cpp(false).std("c11");

            s.file(embed_file_path);
            let res_s = s.compile_intermediates();

            backend_intermediates.extend(res_s);

            c.files(c_files.into_iter().map(|f| backend_dir.join(f)));
            cpp.files(cpp_files.into_iter().map(|f| backend_dir.join(f)));

            println!("cargo:rustc-link-lib=framework=Metal");
            println!("cargo:rustc-link-lib=framework=Foundation");
            println!("cargo:rustc-link-lib=framework=MetalPerformanceShaders");
            println!("cargo:rustc-link-lib=framework=MetalKit");

            //
            println!("cargo:rustc-cfg=feature_metal");
        }
        None => {}
    }

    cpp.file(src_path.join("ggml-backend-reg.cpp"));

    let res_cpp = cpp.compile_intermediates();
    let res_c = c.compile_intermediates();

    let mut all = vec![];
    all.extend(res_cpp.into_iter());
    all.extend(res_c.into_iter());
    all.extend(backend_intermediates.into_iter());
    all
}

fn lib_llama(lib_path: &Path, ggml_objects: Vec<PathBuf>) {
    let ggml_path = lib_path.join("ggml");
    let ggml_include_path = ggml_path.join("include");
    let src_path = lib_path.join("src");
    let include_path = lib_path.join("include");

    let cpp_files = [
        "llama.cpp",
        "llama-adapter.cpp",
        "llama-arch.cpp",
        "llama-batch.cpp",
        "llama-chat.cpp",
        "llama-context.cpp",
        "llama-cparams.cpp",
        "llama-grammar.cpp",
        "llama-graph.cpp",
        "llama-hparams.cpp",
        "llama-impl.cpp",
        "llama-io.cpp",
        "llama-kv-cache.cpp",
        "llama-kv-cache-iswa.cpp",
        "llama-memory.cpp",
        "llama-memory-hybrid.cpp",
        "llama-memory-hybrid-iswa.cpp",
        "llama-memory-recurrent.cpp",
        "llama-mmap.cpp",
        "llama-model-loader.cpp",
        "llama-model-saver.cpp",
        "llama-model.cpp",
        "llama-quant.cpp",
        "llama-vocab.cpp",
        "llama-sampler.cpp",
        "unicode-data.cpp",
        "unicode.cpp",
        "models/afmoe.cpp",
        "models/apertus.cpp",
        "models/arcee.cpp",
        "models/arctic.cpp",
        "models/arwkv7.cpp",
        "models/baichuan.cpp",
        "models/bailingmoe.cpp",
        "models/bailingmoe2.cpp",
        "models/bert.cpp",
        "models/bitnet.cpp",
        "models/bloom.cpp",
        "models/chameleon.cpp",
        "models/chatglm.cpp",
        "models/codeshell.cpp",
        "models/cogvlm.cpp",
        "models/cohere2.cpp",
        "models/command-r.cpp",
        "models/dbrx.cpp",
        "models/deci.cpp",
        "models/deepseek.cpp",
        "models/deepseek2.cpp",
        "models/deepseek2ocr.cpp",
        "models/delta-net-base.cpp",
        "models/dots1.cpp",
        "models/dream.cpp",
        "models/ernie4-5.cpp",
        "models/ernie4-5-moe.cpp",
        "models/eurobert.cpp",
        "models/exaone.cpp",
        "models/exaone-moe.cpp",
        "models/exaone4.cpp",
        "models/falcon.cpp",
        "models/falcon-h1.cpp",
        "models/gemma.cpp",
        "models/gemma-embedding.cpp",
        "models/gemma2.cpp",
        "models/gemma3.cpp",
        "models/gemma3n.cpp",
        "models/gemma4.cpp",
        "models/glm-dsa.cpp",
        "models/glm4.cpp",
        "models/glm4-moe.cpp",
        "models/gpt2.cpp",
        "models/gptneox.cpp",
        "models/granite.cpp",
        "models/granite-hybrid.cpp",
        "models/granite-moe.cpp",
        "models/grok.cpp",
        "models/grovemoe.cpp",
        "models/hunyuan-dense.cpp",
        "models/hunyuan-moe.cpp",
        "models/hunyuan-vl.cpp",
        "models/internlm2.cpp",
        "models/jais.cpp",
        "models/jais2.cpp",
        "models/jamba.cpp",
        "models/jina-bert-v2.cpp",
        "models/jina-bert-v3.cpp",
        "models/kimi-linear.cpp",
        "models/lfm2.cpp",
        "models/lfm2moe.cpp",
        "models/llada.cpp",
        "models/llada-moe.cpp",
        "models/llama.cpp",
        "models/llama-embed.cpp",
        "models/llama4.cpp",
        "models/maincoder.cpp",
        "models/maincoder.cpp",
        "models/mamba.cpp",
        "models/mamba-base.cpp",
        "models/mamba2.cpp",
        "models/mimo2.cpp",
        "models/minicpm.cpp",
        "models/minicpm3.cpp",
        "models/minimax-m2.cpp",
        "models/mistral3.cpp",
        "models/mistral4.cpp",
        "models/modern-bert.cpp",
        "models/mpt.cpp",
        "models/nemotron.cpp",
        "models/nemotron-h.cpp",
        "models/nemotron-h-moe.cpp",
        "models/neo-bert.cpp",
        "models/nomic-bert.cpp",
        "models/nomic-bert-moe.cpp",
        "models/olmo.cpp",
        "models/olmo2.cpp",
        "models/olmoe.cpp",
        "models/openai-moe.cpp",
        "models/openelm.cpp",
        "models/orion.cpp",
        "models/paddleocr.cpp",
        "models/pangu-embed.cpp",
        "models/phi2.cpp",
        "models/phi3.cpp",
        "models/phimoe.cpp",
        "models/plamo.cpp",
        "models/plamo2.cpp",
        "models/plamo3.cpp",
        "models/plm.cpp",
        "models/qwen.cpp",
        "models/qwen2.cpp",
        "models/qwen2moe.cpp",
        "models/qwen2vl.cpp",
        "models/qwen3.cpp",
        "models/qwen3moe.cpp",
        "models/qwen3next.cpp",
        "models/qwen3vl.cpp",
        "models/qwen3vlmoe.cpp",
        "models/qwen35.cpp",
        "models/qwen35moe.cpp",
        "models/refact.cpp",
        "models/rnd1.cpp",
        "models/rwkv6.cpp",
        "models/rwkv6-base.cpp",
        "models/rwkv6qwen2.cpp",
        "models/rwkv7.cpp",
        "models/rwkv7-base.cpp",
        "models/seed-oss.cpp",
        "models/smallthinker.cpp",
        "models/smollm3.cpp",
        "models/stablelm.cpp",
        "models/starcoder.cpp",
        "models/starcoder2.cpp",
        "models/step35.cpp",
        "models/t5.cpp",
        "models/t5encoder.cpp",
        "models/wavtokenizer-dec.cpp",
        "models/xverse.cpp",
    ];

    // create a common build
    let mut common = Build::new();
    common.include(include_path);
    common.include(ggml_include_path);
    common.include(&src_path);
    common.opt_level(3);
    common.flags(WARNING_FLAGS);
    common.define("GGML_SCHED_MAX_COPIES", "4");
    common.define("GGML_SHARED", None);
    common.define("GGML_BUILD", None);
    common.define(
        "GGML_VERSION",
        Some(format!("\"{}\"", ggml_version()).as_str()),
    );
    common.define("GGML_COMMIT", GGML_COMMIT);
    common.define("ggml_base_EXPORTS", None);

    common.define("GGML_USE_CPU", None);

    // specialize for CPP
    let mut cpp = common;

    cpp.cpp(true).std("c++17");
    cpp.objects(ggml_objects);
    cpp.files(cpp_files.into_iter().map(|f| src_path.join(f)));

    cpp.compile("llama");
}
