//! Build script for llama.cpp / ggml.
//!
//! This replicates (a useful subset of) the upstream CMake build without ever
//! invoking `cmake`. Everything CMake would detect or compile is reproduced
//! here in Rust so the crate can be built as part of a normal `cargo build`.
//!
//! What is fully implemented:
//!   * `ggml-base` + the `llama` library
//!   * the CPU backend, with per-architecture source selection and flags
//!     (mirrors `ggml/src/ggml-cpu/CMakeLists.txt`)
//!   * the Metal backend on macOS (mirrors `ggml/src/ggml-metal`)
//!
//! What is detected but not (yet) built: the accelerator backends that require
//! a vendor toolchain we cannot drive from `cc` alone — CUDA (`nvcc`), HIP,
//! MUSA, SYCL, Vulkan (shader codegen), OpenCL and BLAS. When one of these is
//! requested via its cargo feature we auto-locate the toolkit, emit a warning
//! that building it is not implemented, and fall back to the CPU backend. When
//! a toolkit is found on the system but not requested we print a hint.
//!
//! Overrides (environment variables):
//!   * `SKELM_LLAMA_CPU_FLAGS` — space separated list that fully replaces the
//!     auto-selected CPU architecture flags (e.g. `-march=x86-64-v3`). Useful
//!     for portable/distributable builds where `-march=native` is undesirable.

use std::{
    env,
    io::Write,
    path::{Path, PathBuf},
};

use bindgen::Builder;
use cc::Build;

// ---------------------------------------------------------------------------
// ggml version metadata (kept in sync with the vendored submodule)
// ---------------------------------------------------------------------------

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

/// Defines shared by the ggml and llama compilation units.
fn base_defines(b: &mut Build, target: &Target) {
    b.define("GGML_SCHED_MAX_COPIES", "4");
    b.define("GGML_VERSION", Some(format!("\"{}\"", ggml_version()).as_str()));
    b.define("GGML_COMMIT", GGML_COMMIT);
    b.define("GGML_USE_CPU", None);

    // GGML_SHARED/GGML_BUILD select the symbol-visibility macro in the public
    // headers. On unix-likes they expand to `__attribute__((visibility("default")))`,
    // which is harmless when we link the archive statically. On Windows they
    // expand to `__declspec(dllexport)`, which is wrong for a static library and
    // would mismatch the (plain `extern`) declarations bindgen generates — so we
    // leave them undefined there, yielding plain `extern`.
    if target.os != "windows" {
        b.define("GGML_SHARED", None);
        b.define("GGML_BUILD", None);
        b.define("ggml_base_EXPORTS", None);
    }
}

/// POSIX / platform feature-test macros, mirroring the conformance section of
/// `ggml/src/CMakeLists.txt`.
fn os_defines(b: &mut Build, target: &Target) {
    if target.is_linux() {
        b.define("_XOPEN_SOURCE", "600");
        b.define("_GNU_SOURCE", None);
    }
    if target.is_apple() {
        b.define("_DARWIN_C_SOURCE", None);
    }
    if target.os == "windows" {
        b.define("_CRT_SECURE_NO_WARNINGS", None);
    }
}

// ---------------------------------------------------------------------------
// Target description (driven by CARGO_CFG_* so cross-compilation works)
// ---------------------------------------------------------------------------

/// The ggml "system arch" families, matching `ggml_get_system_arch()` in
/// `ggml/cmake/common.cmake`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Arch {
    X86,
    Arm,
    PowerPC,
    Riscv64,
    S390x,
    LoongArch64,
    Wasm,
    Unknown,
}

struct Target {
    /// e.g. "macos", "linux", "windows"
    os: String,
    /// e.g. "msvc", "gnu", "musl", ""
    env: String,
    arch: Arch,
}

impl Target {
    fn from_env() -> Self {
        let os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
        let env_ = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
        let raw = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
        // Mirror ggml_get_system_arch(): map the rust target arch onto ggml's
        // coarse families.
        let arch = match raw.as_str() {
            "x86_64" | "x86" => Arch::X86,
            "aarch64" | "arm" => Arch::Arm,
            "powerpc" | "powerpc64" => Arch::PowerPC,
            "riscv64" => Arch::Riscv64,
            "s390x" => Arch::S390x,
            "loongarch64" => Arch::LoongArch64,
            "wasm32" | "wasm64" => Arch::Wasm,
            _ => Arch::Unknown,
        };
        Target {
            os,
            env: env_,
            arch,
        }
    }

    fn is_macos(&self) -> bool {
        self.os == "macos" || self.os == "ios"
    }

    fn is_apple(&self) -> bool {
        matches!(self.os.as_str(), "macos" | "ios" | "tvos" | "watchos")
    }

    fn is_msvc(&self) -> bool {
        self.env == "msvc"
    }

    fn is_linux(&self) -> bool {
        self.os == "linux" || self.os == "android"
    }
}

// ---------------------------------------------------------------------------
// Compiler warning flags (skip the clang/gcc style flags on MSVC)
// ---------------------------------------------------------------------------

/// Compiler warning flags, split by language.
///
/// Some flags are only meaningful for one language: e.g. `-Wstrict-prototypes`
/// is "valid for C/ObjC but not for C++", so passing it to the C++ compiler
/// makes clang/gcc emit a warning on every translation unit. The upstream CMake
/// keeps separate `WARNING_FLAGS` / `C_FLAGS` / `CXX_FLAGS` lists for this; we
/// mirror that split here and apply each set to the matching `cc::Build`.
struct WarnFlags {
    /// applied to both C and C++ (and ObjC) sources
    common: Vec<&'static str>,
    /// applied to C / ObjC sources only
    c_only: Vec<&'static str>,
    /// applied to C++ sources only
    cpp_only: Vec<&'static str>,
}

fn warning_flags(target: &Target) -> WarnFlags {
    if target.is_msvc() {
        // MSVC does not understand the -W... flags below.
        return WarnFlags {
            common: vec![],
            c_only: vec![],
            cpp_only: vec![],
        };
    }

    let mut common = vec![
        //"-Wshadow",
        "-Wall",
        "-Wextra",
        "-Wpedantic",
        "-Wcast-qual",
        "-Wno-unused-function",
    ];

    if target.is_macos() {
        // Valid for both C and C++ under clang (see ggml_get_flags()).
        common.push("-Wunreachable-code-break");
        common.push("-Wunreachable-code-return");
    }

    // These are valid for C/ObjC but NOT for C++.
    let c_only = vec![
        "-Wstrict-prototypes",
        "-Wpointer-arith",
        "-Wmissing-prototypes",
        "-Werror=implicit-int",
        "-Werror=implicit-function-declaration",
    ];

    // C++-only warnings (upstream adds -Wmissing-declarations/-Wmissing-noreturn
    // here); left empty to preserve the current warning surface.
    let cpp_only = vec![];

    WarnFlags {
        common,
        c_only,
        cpp_only,
    }
}

fn main() {
    let lib_path = PathBuf::from("libs");

    if !lib_path.exists() {
        panic!("cannot compile without cloning the llama.cpp git submodule")
    }

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target = Target::from_env();

    // Rerun when a toolkit-location override changes.
    for var in [
        "SKELM_LLAMA_CPU_FLAGS",
        "CUDA_PATH",
        "CUDA_HOME",
        "ROCM_PATH",
        "HIP_PATH",
        "MUSA_PATH",
        "ONEAPI_ROOT",
        "VULKAN_SDK",
        "OPENBLAS_PATH",
        "MKLROOT",
    ] {
        println!("cargo:rerun-if-env-changed={var}");
    }

    // Report which accelerator backends were requested / detected and decide
    // whether we can build one. Anything other than CPU/Metal currently falls
    // back to CPU.
    report_backends(&target);

    bindings(&lib_path, &out_path);

    // Metal is the one GPU backend we actually build, and only on Apple.
    let use_metal = target.is_macos() && metal_wanted(&target);

    let ggml_objects = lib_ggml(&lib_path, &out_path, &target, use_metal);
    lib_llama(&lib_path, &target, ggml_objects);
}

// ---------------------------------------------------------------------------
// Backend detection / reporting
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
enum Gpu {
    Cuda,
    Hip,
    Musa,
    Sycl,
    Vulkan,
    OpenCL,
    Blas,
}

impl Gpu {
    const ALL: [Gpu; 7] = [
        Gpu::Cuda,
        Gpu::Hip,
        Gpu::Musa,
        Gpu::Sycl,
        Gpu::Vulkan,
        Gpu::OpenCL,
        Gpu::Blas,
    ];

    fn feature(self) -> &'static str {
        match self {
            Gpu::Cuda => "cuda",
            Gpu::Hip => "hip",
            Gpu::Musa => "musa",
            Gpu::Sycl => "sycl",
            Gpu::Vulkan => "vulkan",
            Gpu::OpenCL => "opencl",
            Gpu::Blas => "blas",
        }
    }

    fn display(self) -> &'static str {
        match self {
            Gpu::Cuda => "CUDA",
            Gpu::Hip => "HIP/ROCm",
            Gpu::Musa => "MUSA",
            Gpu::Sycl => "SYCL/oneAPI",
            Gpu::Vulkan => "Vulkan",
            Gpu::OpenCL => "OpenCL",
            Gpu::Blas => "BLAS",
        }
    }

    /// Try to locate the toolkit/SDK for this backend, returning a short human
    /// readable description of where it was found. Mirrors the corresponding
    /// CMake `find_package(...)` at a best-effort level.
    fn detect(self, target: &Target) -> Option<String> {
        match self {
            Gpu::Cuda => detect_cuda(),
            Gpu::Hip => detect_hip(),
            Gpu::Musa => detect_musa(),
            Gpu::Sycl => detect_sycl(),
            Gpu::Vulkan => detect_vulkan(),
            Gpu::OpenCL => detect_opencl(),
            Gpu::Blas => detect_blas(target),
        }
    }
}

fn report_backends(target: &Target) {
    let mut hints = Vec::new();

    // The `metal` feature is only meaningful on Apple targets.
    if feature_enabled("metal") && !target.is_macos() {
        warn("the `metal` feature was enabled but the target is not Apple; Metal will not be built.");
    }

    for gpu in Gpu::ALL {
        let requested = feature_enabled(gpu.feature());
        let found = gpu.detect(target);
        match (requested, &found) {
            (true, Some(loc)) => warn(format!(
                "{name} backend requested (feature `{feat}`); toolkit detected at {loc}. \
                 Building {name} from build.rs is not yet implemented — falling back to the CPU backend.",
                name = gpu.display(),
                feat = gpu.feature(),
            )),
            (true, None) => warn(format!(
                "{name} backend requested (feature `{feat}`) but no toolkit was detected, \
                 and building it is not yet implemented — falling back to the CPU backend.",
                name = gpu.display(),
                feat = gpu.feature(),
            )),
            (false, Some(loc)) => hints.push(format!(
                "{name} ({loc}) — enable with `--features {feat}` (detection only, build not yet implemented)",
                name = gpu.display(),
                feat = gpu.feature(),
            )),
            (false, None) => {}
        }
    }

    if !hints.is_empty() {
        warn(format!(
            "detected accelerator toolkits on this system: {}",
            hints.join("; ")
        ));
    }
}

fn feature_enabled(name: &str) -> bool {
    // Cargo exports CARGO_FEATURE_<NAME> (uppercased, `-` -> `_`) for each
    // enabled feature of this crate.
    let var = format!("CARGO_FEATURE_{}", name.to_uppercase().replace('-', "_"));
    env::var_os(var).is_some()
}

fn metal_wanted(target: &Target) -> bool {
    // Matches GGML_METAL_DEFAULT (ON on Apple). The `metal` feature can be used
    // to be explicit but is not required on macOS.
    target.is_macos()
}

fn warn(msg: impl AsRef<str>) {
    println!("cargo:warning={}", msg.as_ref());
}

// --- individual toolkit probes -------------------------------------------

/// Look up an executable on the host `PATH` (build machine).
fn which(prog: &str) -> Option<PathBuf> {
    let paths = env::var_os("PATH")?;
    let candidates: Vec<String> = if cfg!(target_os = "windows") {
        let exts = env::var("PATHEXT").unwrap_or_else(|_| ".EXE;.CMD;.BAT".into());
        exts.split(';')
            .map(|e| format!("{}{}", prog, e))
            .collect()
    } else {
        vec![prog.to_string()]
    };
    for dir in env::split_paths(&paths) {
        for c in &candidates {
            let full = dir.join(c);
            if full.is_file() {
                return Some(full);
            }
        }
    }
    None
}

fn env_dir(var: &str) -> Option<String> {
    let v = env::var_os(var)?;
    let p = PathBuf::from(&v);
    if p.exists() {
        Some(format!("{} (${})", p.display(), var))
    } else {
        None
    }
}

fn first_existing(paths: &[&str]) -> Option<String> {
    paths
        .iter()
        .find(|p| Path::new(p).exists())
        .map(|p| p.to_string())
}

fn detect_cuda() -> Option<String> {
    ["CUDA_PATH", "CUDA_HOME", "CUDAToolkit_ROOT"]
        .iter()
        .find_map(|v| env_dir(v))
        .or_else(|| which("nvcc").map(|p| format!("{} (nvcc on PATH)", p.display())))
        .or_else(|| first_existing(&["/usr/local/cuda", "/opt/cuda"]))
}

fn detect_hip() -> Option<String> {
    ["ROCM_PATH", "HIP_PATH"]
        .iter()
        .find_map(|v| env_dir(v))
        .or_else(|| which("hipcc").map(|p| format!("{} (hipcc on PATH)", p.display())))
        .or_else(|| first_existing(&["/opt/rocm"]))
}

fn detect_musa() -> Option<String> {
    env_dir("MUSA_PATH")
        .or_else(|| which("mcc").map(|p| format!("{} (mcc on PATH)", p.display())))
        .or_else(|| first_existing(&["/usr/local/musa"]))
}

fn detect_sycl() -> Option<String> {
    env_dir("ONEAPI_ROOT")
        .or_else(|| which("icpx").map(|p| format!("{} (icpx on PATH)", p.display())))
        .or_else(|| first_existing(&["/opt/intel/oneapi"]))
}

fn detect_vulkan() -> Option<String> {
    env_dir("VULKAN_SDK")
        .or_else(|| which("glslc").map(|p| format!("{} (glslc on PATH)", p.display())))
        .or_else(|| {
            first_existing(&[
                "/usr/lib/x86_64-linux-gnu/libvulkan.so.1",
                "/usr/lib/libvulkan.so.1",
                "/usr/local/lib/libvulkan.so.1",
            ])
        })
}

fn detect_opencl() -> Option<String> {
    env_dir("OpenCL_ROOT").or_else(|| {
        first_existing(&[
            "/usr/include/CL/cl.h",
            "/usr/local/include/CL/cl.h",
            "/usr/lib/x86_64-linux-gnu/libOpenCL.so",
        ])
    })
}

fn detect_blas(target: &Target) -> Option<String> {
    if target.is_apple() {
        // The Accelerate framework ships with macOS (GGML_BLAS_VENDOR "Apple").
        return Some("Accelerate framework".to_string());
    }
    env_dir("OPENBLAS_PATH")
        .or_else(|| env_dir("MKLROOT"))
        .or_else(|| {
            first_existing(&[
                "/usr/lib/x86_64-linux-gnu/libopenblas.so",
                "/usr/lib/libopenblas.so",
                "/usr/local/lib/libopenblas.so",
                "/usr/lib64/libopenblas.so",
            ])
        })
}

// ---------------------------------------------------------------------------
// bindgen
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// CPU backend per-architecture configuration
// (mirrors ggml/src/ggml-cpu/CMakeLists.txt)
// ---------------------------------------------------------------------------

#[derive(Default)]
struct CpuArchConfig {
    /// arch-specific C sources, relative to `ggml-cpu/`
    c_files: Vec<&'static str>,
    /// arch-specific C++ sources, relative to `ggml-cpu/`
    cpp_files: Vec<&'static str>,
    /// compiler flags applied to the CPU sources (e.g. `-march=native`)
    flags: Vec<String>,
    /// preprocessor defines applied to the CPU sources
    defines: Vec<&'static str>,
}

fn cpu_arch_config(target: &Target) -> CpuArchConfig {
    let mut cfg = CpuArchConfig::default();

    // Allow a full manual override of the architecture flags.
    let flag_override: Option<Vec<String>> = env::var("SKELM_LLAMA_CPU_FLAGS")
        .ok()
        .map(|s| s.split_whitespace().map(str::to_string).collect());

    match target.arch {
        Arch::X86 => {
            cfg.c_files.push("arch/x86/quants.c");
            cfg.cpp_files.push("arch/x86/repack.cpp");
            if let Some(f) = flag_override {
                cfg.flags = f;
            } else if target.is_msvc() {
                // MSVC has no `-march=native`; pick a widely-available baseline.
                // /arch:AVX2 implies FMA + F16C, but MSVC doesn't define the
                // corresponding macros, so we set the ggml ones by hand
                // (matching the MSVC branch of the CMake file).
                cfg.flags.push("/arch:AVX2".to_string());
                cfg.defines.push("GGML_AVX2");
                cfg.defines.push("GGML_FMA");
                cfg.defines.push("GGML_F16C");
            } else {
                // GGML_NATIVE defaults to ON upstream.
                cfg.flags.push("-march=native".to_string());
            }
        }
        Arch::Arm => {
            cfg.c_files.push("arch/arm/quants.c");
            cfg.cpp_files.push("arch/arm/repack.cpp");
            if let Some(f) = flag_override {
                cfg.flags = f;
            } else if target.is_apple() {
                // Apple clang enables the right NEON baseline for arm64 without
                // extra flags; this is the historically working configuration.
            } else if !target.is_msvc() {
                cfg.flags.push("-mcpu=native".to_string());
            }
        }
        Arch::PowerPC => {
            cfg.c_files.push("arch/powerpc/quants.c");
            cfg.flags = flag_override.unwrap_or_else(|| {
                vec!["-mcpu=native".to_string(), "-mtune=native".to_string()]
            });
        }
        Arch::Riscv64 => {
            cfg.c_files.push("arch/riscv/quants.c");
            cfg.cpp_files.push("arch/riscv/repack.cpp");
            // Conservative baseline (no vector ext) to avoid SIGILL on cores
            // without RVV; override with SKELM_LLAMA_CPU_FLAGS for RVV targets.
            cfg.flags = flag_override
                .unwrap_or_else(|| vec!["-march=rv64gc".to_string(), "-mabi=lp64d".to_string()]);
        }
        Arch::S390x => {
            cfg.c_files.push("arch/s390/quants.c");
            cfg.flags = flag_override.unwrap_or_else(|| {
                vec!["-march=native".to_string(), "-mtune=native".to_string()]
            });
        }
        Arch::LoongArch64 => {
            cfg.c_files.push("arch/loongarch/quants.c");
            cfg.flags = flag_override.unwrap_or_else(|| {
                vec![
                    "-march=loongarch64".to_string(),
                    "-mlasx".to_string(),
                    "-mlsx".to_string(),
                ]
            });
        }
        Arch::Wasm => {
            cfg.c_files.push("arch/wasm/quants.c");
            cfg.flags = flag_override.unwrap_or_else(|| vec!["-msimd128".to_string()]);
        }
        Arch::Unknown => {
            warn("unknown CPU architecture — falling back to generic ggml implementations");
            cfg.defines.push("GGML_CPU_GENERIC");
            if let Some(f) = flag_override {
                cfg.flags = f;
            }
        }
    }

    cfg
}

// ---------------------------------------------------------------------------
// ggml library (base + CPU + optional Metal)
// ---------------------------------------------------------------------------

fn lib_ggml(lib_path: &Path, out_path: &Path, target: &Target, use_metal: bool) -> Vec<PathBuf> {
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

    let warn = warning_flags(target);

    // create a common build
    let mut common = Build::new();
    common.include(&include_path);
    common.include(&src_path);
    common.opt_level(3);
    common.flags(&warn.common);

    base_defines(&mut common, target);
    os_defines(&mut common, target);

    // specialize for CPP
    let mut cpp = common.clone();
    cpp.cpp(true).std("c++17");
    cpp.flags(&warn.cpp_only);
    cpp.files(cpp_files.into_iter().map(|f| src_path.join(f)));

    // specialize for C (also compiles the ObjC .m Metal sources added later)
    let mut c = common.clone();
    c.cpp(false).std("c11");
    c.flags(&warn.c_only);
    c.files(c_files.into_iter().map(|f| src_path.join(f)));

    let mut backend_intermediates = Vec::new();

    // ---- CPU backend --------------------------------------------------
    {
        let backend_dir = src_path.join("ggml-cpu");
        c.include(&backend_dir);
        cpp.include(&backend_dir);

        // Sources always built regardless of arch.
        let base_c = ["ggml-cpu.c", "quants.c"];
        let base_cpp = [
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
        ];

        c.files(base_c.into_iter().map(|f| backend_dir.join(f)));
        cpp.files(base_cpp.into_iter().map(|f| backend_dir.join(f)));

        // Activate the code paths that are compiled in `base_cpp`/`ggml-cpu.c`
        // above. Both are ON by default in the upstream llama.cpp CMake build.
        for def in ["GGML_USE_CPU_REPACK", "GGML_USE_LLAMAFILE"] {
            c.define(def, None);
            cpp.define(def, None);
        }

        // Arch-specific sources / flags / defines.
        let arch_cfg = cpu_arch_config(target);
        c.files(arch_cfg.c_files.iter().map(|f| backend_dir.join(f)));
        cpp.files(arch_cfg.cpp_files.iter().map(|f| backend_dir.join(f)));
        for flag in &arch_cfg.flags {
            c.flag(flag);
            cpp.flag(flag);
        }
        for def in &arch_cfg.defines {
            c.define(def, None);
            cpp.define(def, None);
        }
    }

    // ---- Metal backend (Apple only) -----------------------------------
    if use_metal {
        let metaldefs = [
            "GGML_USE_METAL",
            "GGML_METAL_EMBED_LIBRARY",
            "ggml_metal_EXPORTS",
        ];
        for metaldef in metaldefs {
            c.define(metaldef, None);
            cpp.define(metaldef, None);
        }

        let backend_dir = src_path.join("ggml-metal");
        c.include(&backend_dir);
        cpp.include(&backend_dir);

        let metal_c_files = ["ggml-metal-context.m", "ggml-metal-device.m"];
        let metal_cpp_files = [
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

        c.files(metal_c_files.into_iter().map(|f| backend_dir.join(f)));
        cpp.files(metal_cpp_files.into_iter().map(|f| backend_dir.join(f)));

        println!("cargo:rustc-link-lib=framework=Metal");
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=MetalPerformanceShaders");
        println!("cargo:rustc-link-lib=framework=MetalKit");

        println!("cargo:rustc-cfg=feature_metal");
    }

    cpp.file(src_path.join("ggml-backend-reg.cpp"));

    let res_cpp = cpp.compile_intermediates();
    let res_c = c.compile_intermediates();

    let mut all = vec![];
    all.extend(res_cpp);
    all.extend(res_c);
    all.extend(backend_intermediates);
    all
}

// ---------------------------------------------------------------------------
// llama library
// ---------------------------------------------------------------------------

fn lib_llama(lib_path: &Path, target: &Target, ggml_objects: Vec<PathBuf>) {
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
    let warn = warning_flags(target);
    common.flags(&warn.common);
    base_defines(&mut common, target);
    os_defines(&mut common, target);

    // specialize for CPP (llama is C++ only)
    let mut cpp = common;

    cpp.cpp(true).std("c++17");
    cpp.flags(&warn.cpp_only);
    cpp.objects(ggml_objects);
    cpp.files(cpp_files.into_iter().map(|f| src_path.join(f)));

    cpp.compile("llama");
}
