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
//!   * Accelerate in the CPU backend on Apple (mirrors `GGML_ACCELERATE`)
//!   * the BLAS backend (mirrors `ggml/src/ggml-blas`): Accelerate on Apple,
//!     where it is on by default like upstream, and OpenBLAS elsewhere with the
//!     `blas` feature
//!
//! What is detected but not (yet) built: the accelerator backends that require
//! a vendor toolchain we cannot drive from `cc` alone — CUDA (`nvcc`), HIP,
//! MUSA, SYCL, Vulkan (shader codegen) and OpenCL. When one of these is
//! requested via its cargo feature we auto-locate the toolkit, emit a warning
//! that building it is not implemented, and fall back to the CPU backend. When
//! a toolkit is found on the system but not requested we print a hint.
//!
//! Overrides (environment variables):
//!   * `SKELM_LLAMA_CPU_FLAGS` — space separated list that fully replaces the
//!     auto-selected CPU architecture flags (e.g. `-march=x86-64-v3`). Useful
//!     for portable/distributable builds where `-march=native` is undesirable.
//!   * `SKELM_LLAMA_BLAS_VENDOR` — `Apple` (default on Apple), `OpenBLAS`
//!     (default elsewhere) or `none` to disable the BLAS backend.
//!   * `OPENBLAS_PATH` — OpenBLAS install prefix (with `include/` and `lib/`),
//!     tried before `pkg-config` and the usual system locations.

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
const GGML_VERSION_MINOR: u64 = 27;
const GGML_VERSION_PATCH: u64 = 0;
const GGML_COMMIT: &str = "e707790c";
#[allow(unused)]
const GGML_COMMIT_LONG: &str = "e707790c73590c2ff5303045b714afb0c5e22035";

// llama.cpp version metadata (from the top-level CMakeLists.txt; LLAMA_BUILD_IS_DEV
// defaults to ON upstream, hence the `-dev` suffix)
const LLAMA_VERSION_MAJOR: u64 = 0;
const LLAMA_VERSION_MINOR: u64 = 7;
const LLAMA_VERSION_PATCH: u64 = 0;

fn ggml_version() -> String {
    format!(
        "{}.{}.{}",
        GGML_VERSION_MAJOR, GGML_VERSION_MINOR, GGML_VERSION_PATCH
    )
}

fn llama_version() -> String {
    format!(
        "{}.{}.{}-dev",
        LLAMA_VERSION_MAJOR, LLAMA_VERSION_MINOR, LLAMA_VERSION_PATCH
    )
}

/// Generate `ggml-version.h` and `llama-version.h` into `out_path`, mirroring
/// the `configure_file(*-version.h.in ...)` calls of the upstream CMake.
fn version_headers(lib_path: &Path, out_path: &Path) {
    let headers = [
        (
            lib_path.join("ggml/src/ggml-version.h.in"),
            "ggml-version.h",
            [
                ("@GGML_VERSION@", ggml_version()),
                ("@GGML_BUILD_COMMIT@", GGML_COMMIT.to_string()),
            ],
        ),
        (
            lib_path.join("src/llama-version.h.in"),
            "llama-version.h",
            [
                ("@LLAMA_VERSION@", llama_version()),
                ("@LLAMA_BUILD_COMMIT@", GGML_COMMIT.to_string()),
            ],
        ),
    ];
    for (template, name, substs) in headers {
        let mut data = std::fs::read_to_string(&template)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", template.display()));
        for (from, to) in substs {
            data = data.replace(from, &to);
        }
        std::fs::write(out_path.join(name), data).expect("cannot write version header");
    }
}

/// Defines shared by the ggml and llama compilation units.
fn base_defines(b: &mut Build, target: &Target) {
    b.define("GGML_SCHED_MAX_COPIES", "4");
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

/// MSVC options that upstream sets on every target (top-level and
/// `ggml/CMakeLists.txt`): sources are UTF-8, `llama-model.cpp` and friends
/// exceed the default COFF section limit, and the noisy conversion /
/// redefinition warnings are silenced.
fn msvc_options(b: &mut Build, target: &Target) {
    if !target.is_msvc() {
        return;
    }
    for flag in [
        "/utf-8", "/bigobj", "/wd4005", "/wd4244", "/wd4267", "/wd4305", "/wd4566", "/wd4996",
        "/wd4702",
    ] {
        b.flag(flag);
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
        "SKELM_LLAMA_BLAS_VENDOR",
    ] {
        println!("cargo:rerun-if-env-changed={var}");
    }
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=shim");

    // Report which accelerator backends were requested / detected and decide
    // whether we can build one. Anything other than CPU/Metal/BLAS currently
    // falls back to CPU.
    report_backends(&target);

    bindings(&lib_path, &out_path);
    version_headers(&lib_path, &out_path);

    // Metal is the one GPU backend we actually build, and only on Apple.
    let use_metal = target.is_macos() && metal_wanted(&target);
    let blas = blas_config(&target);

    let ggml_objects = lib_ggml(&lib_path, &out_path, &target, use_metal, blas.as_ref());
    lib_llama(&lib_path, &out_path, &target, ggml_objects);
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
}

impl Gpu {
    const ALL: [Gpu; 6] = [
        Gpu::Cuda,
        Gpu::Hip,
        Gpu::Musa,
        Gpu::Sycl,
        Gpu::Vulkan,
        Gpu::OpenCL,
    ];

    fn feature(self) -> &'static str {
        match self {
            Gpu::Cuda => "cuda",
            Gpu::Hip => "hip",
            Gpu::Musa => "musa",
            Gpu::Sycl => "sycl",
            Gpu::Vulkan => "vulkan",
            Gpu::OpenCL => "opencl",
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
        }
    }

    /// Try to locate the toolkit/SDK for this backend, returning a short human
    /// readable description of where it was found. Mirrors the corresponding
    /// CMake `find_package(...)` at a best-effort level.
    fn detect(self) -> Option<String> {
        match self {
            Gpu::Cuda => detect_cuda(),
            Gpu::Hip => detect_hip(),
            Gpu::Musa => detect_musa(),
            Gpu::Sycl => detect_sycl(),
            Gpu::Vulkan => detect_vulkan(),
            Gpu::OpenCL => detect_opencl(),
        }
    }
}

fn report_backends(target: &Target) {
    let mut hints = Vec::new();

    // The `metal` feature is only meaningful on Apple targets.
    if feature_enabled("metal") && !target.is_macos() {
        warn(
            "the `metal` feature was enabled but the target is not Apple; Metal will not be built.",
        );
    }

    for gpu in Gpu::ALL {
        let requested = feature_enabled(gpu.feature());
        let found = gpu.detect();
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
        exts.split(';').map(|e| format!("{}{}", prog, e)).collect()
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

// ---------------------------------------------------------------------------
// BLAS backend configuration (mirrors ggml/src/ggml-blas/CMakeLists.txt)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BlasVendor {
    /// the Accelerate framework (GGML_BLAS_VENDOR "Apple")
    Apple,
    OpenBlas,
}

struct BlasConfig {
    vendor: BlasVendor,
    /// include directories for `ggml-blas.cpp` (to find `cblas.h`)
    include_dirs: Vec<PathBuf>,
    /// extra preprocessor defines (from `pkg-config --cflags`)
    defines: Vec<(String, Option<String>)>,
    /// native library search directories
    link_dirs: Vec<PathBuf>,
    /// libraries to link
    libs: Vec<String>,
}

impl BlasConfig {
    fn new(vendor: BlasVendor) -> Self {
        BlasConfig {
            vendor,
            include_dirs: vec![],
            defines: vec![],
            link_dirs: vec![],
            libs: vec![],
        }
    }
}

/// Decide whether to build the BLAS backend and with which vendor.
///
/// Like upstream (`GGML_BLAS_DEFAULT`), it is on by default on Apple with
/// Accelerate; elsewhere it requires the `blas` feature and an OpenBLAS
/// install. `SKELM_LLAMA_BLAS_VENDOR` overrides the vendor or disables it.
fn blas_config(target: &Target) -> Option<BlasConfig> {
    let vendor = match env::var("SKELM_LLAMA_BLAS_VENDOR").ok().as_deref() {
        Some(v) if v.eq_ignore_ascii_case("none") => return None,
        Some(v) if v.eq_ignore_ascii_case("apple") => BlasVendor::Apple,
        Some(v) if v.eq_ignore_ascii_case("openblas") => BlasVendor::OpenBlas,
        Some(v) => panic!(
            "unsupported SKELM_LLAMA_BLAS_VENDOR `{v}` (expected `Apple`, `OpenBLAS` or `none`)"
        ),
        None if target.is_apple() => BlasVendor::Apple,
        None => BlasVendor::OpenBlas,
    };

    if !target.is_apple() && !feature_enabled("blas") {
        if let Some((_, loc)) = find_openblas(target) {
            warn(format!(
                "detected OpenBLAS ({loc}) — enable the BLAS backend with `--features blas`"
            ));
        }
        return None;
    }

    match vendor {
        BlasVendor::Apple => {
            if !target.is_apple() {
                panic!("the Accelerate BLAS vendor is only available on Apple targets");
            }
            Some(BlasConfig::new(BlasVendor::Apple))
        }
        BlasVendor::OpenBlas => match find_openblas(target) {
            Some((cfg, loc)) => {
                warn(format!(
                    "building the BLAS backend with OpenBLAS from {loc}"
                ));
                Some(cfg)
            }
            None => panic!(
                "BLAS backend requested but OpenBLAS was not found: install it (e.g. \
                 `libopenblas-dev`), set OPENBLAS_PATH to its install prefix, or set \
                 SKELM_LLAMA_BLAS_VENDOR=none"
            ),
        },
    }
}

/// Locate OpenBLAS: `$OPENBLAS_PATH`, then `pkg-config` (`openblas64` first, as
/// upstream), then the `cblas.h` hint paths of the upstream CMake. Returns the
/// configuration and a description of where it was found.
fn find_openblas(target: &Target) -> Option<(BlasConfig, String)> {
    if let Some(prefix) = env::var_os("OPENBLAS_PATH").map(PathBuf::from) {
        let include = [
            prefix.join("include").join("openblas"),
            prefix.join("include"),
        ]
        .into_iter()
        .find(|d| d.join("cblas.h").is_file());
        match include {
            Some(include) => {
                let lib_dir = prefix.join("lib");
                // Windows distributions ship either `openblas.lib` (vcpkg) or
                // `libopenblas.lib` (upstream release archives).
                let lib = if target.os == "windows" && lib_dir.join("libopenblas.lib").is_file() {
                    "libopenblas"
                } else {
                    "openblas"
                };
                let mut cfg = BlasConfig::new(BlasVendor::OpenBlas);
                cfg.include_dirs.push(include);
                cfg.link_dirs.push(lib_dir);
                cfg.libs.push(lib.to_string());
                return Some((cfg, format!("{} ($OPENBLAS_PATH)", prefix.display())));
            }
            None => warn(format!(
                "OPENBLAS_PATH is set to {} but no cblas.h was found under its include/ directory",
                prefix.display()
            )),
        }
    }

    for name in ["openblas64", "openblas"] {
        if let Some(cfg) = pkg_config_blas(name) {
            return Some((cfg, format!("pkg-config `{name}`")));
        }
    }

    let include = [
        "/usr/include/openblas",
        "/usr/include/x86_64-linux-gnu/openblas/include",
        "/opt/homebrew/opt/openblas/include",
        "/usr/local/opt/openblas/include",
        "/usr/include",
        "/usr/local/include",
    ]
    .into_iter()
    .map(PathBuf::from)
    .find(|d| d.join("cblas.h").is_file())?;
    let lib_dir = [
        "/opt/homebrew/opt/openblas/lib",
        "/usr/local/opt/openblas/lib",
        "/usr/lib/x86_64-linux-gnu",
        "/usr/lib/aarch64-linux-gnu",
        "/usr/lib64",
        "/usr/lib",
        "/usr/local/lib",
    ]
    .into_iter()
    .map(PathBuf::from)
    .find(|d| {
        ["libopenblas.so", "libopenblas.a", "libopenblas.dylib"]
            .iter()
            .any(|f| d.join(f).is_file())
    })?;

    let mut cfg = BlasConfig::new(BlasVendor::OpenBlas);
    cfg.include_dirs.push(include.clone());
    cfg.link_dirs.push(lib_dir.clone());
    cfg.libs.push("openblas".to_string());
    let loc = format!("{} / {}", include.display(), lib_dir.display());
    Some((cfg, loc))
}

/// Query `pkg-config --cflags --libs <name>` and turn it into a BLAS config.
fn pkg_config_blas(name: &str) -> Option<BlasConfig> {
    let out = std::process::Command::new("pkg-config")
        .args(["--cflags", "--libs", name])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let mut cfg = BlasConfig::new(BlasVendor::OpenBlas);
    for tok in String::from_utf8_lossy(&out.stdout).split_whitespace() {
        if let Some(dir) = tok.strip_prefix("-I") {
            cfg.include_dirs.push(PathBuf::from(dir));
        } else if let Some(def) = tok.strip_prefix("-D") {
            let (key, value) = match def.split_once('=') {
                Some((k, v)) => (k.to_string(), Some(v.to_string())),
                None => (def.to_string(), None),
            };
            cfg.defines.push((key, value));
        } else if let Some(dir) = tok.strip_prefix("-L") {
            cfg.link_dirs.push(PathBuf::from(dir));
        } else if let Some(lib) = tok.strip_prefix("-l") {
            cfg.libs.push(lib.to_string());
        }
    }
    if cfg.libs.is_empty() {
        return None;
    }
    Some(cfg)
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
        .allowlist_var("LLAMA_.*")
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
            cfg.flags = flag_override
                .unwrap_or_else(|| vec!["-mcpu=native".to_string(), "-mtune=native".to_string()]);
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
            cfg.cpp_files.push("arch/s390/repack.cpp");
            cfg.flags = flag_override
                .unwrap_or_else(|| vec!["-march=native".to_string(), "-mtune=native".to_string()]);
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

fn lib_ggml(
    lib_path: &Path,
    out_path: &Path,
    target: &Target,
    use_metal: bool,
    blas: Option<&BlasConfig>,
) -> Vec<PathBuf> {
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
    // generated ggml-version.h
    common.include(out_path);
    common.opt_level(3);
    common.flags(&warn.common);

    base_defines(&mut common, target);
    os_defines(&mut common, target);
    msvc_options(&mut common, target);

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
            "tiled/tiled.cpp",
            "tiled/tiled-kernel.cpp",
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

        // GGML_ACCELERATE (ON by default upstream): vDSP / vForce for the
        // vector ops of the CPU backend.
        if target.is_apple() {
            for def in [
                "GGML_USE_ACCELERATE",
                "ACCELERATE_NEW_LAPACK",
                "ACCELERATE_LAPACK_ILP64",
            ] {
                c.define(def, None);
                cpp.define(def, None);
            }
            println!("cargo:rustc-link-lib=framework=Accelerate");
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
            "ggml-metal-fusion.cpp",
            "ggml-metal-ops.cpp",
            "ggml-metal-tuning.cpp",
        ];

        cpp.define("GGML_METAL_EMBED_LIBRARY", None);

        let embed_file_path = metal_embed_library(&src_path, &backend_dir, out_path);

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

    // ---- BLAS backend -------------------------------------------------
    if let Some(blas) = blas {
        // makes ggml-backend-reg.cpp register the backend
        cpp.define("GGML_USE_BLAS", None);

        // the vendor defines and include paths are private to ggml-blas
        let mut b = common.clone();
        b.cpp(true).std("c++17");
        b.flags(&warn.cpp_only);
        for dir in &blas.include_dirs {
            b.include(dir);
        }
        let vendor_defines: &[&str] = match blas.vendor {
            BlasVendor::Apple => &[
                "ACCELERATE_NEW_LAPACK",
                "ACCELERATE_LAPACK_ILP64",
                "GGML_BLAS_USE_ACCELERATE",
            ],
            BlasVendor::OpenBlas => &["GGML_BLAS_USE_OPENBLAS"],
        };
        for def in vendor_defines {
            b.define(def, None);
        }
        for (key, value) in &blas.defines {
            b.define(key, value.as_deref());
        }
        b.file(src_path.join("ggml-blas/ggml-blas.cpp"));
        backend_intermediates.extend(b.compile_intermediates());

        match blas.vendor {
            BlasVendor::Apple => println!("cargo:rustc-link-lib=framework=Accelerate"),
            BlasVendor::OpenBlas => {
                for dir in &blas.link_dirs {
                    println!("cargo:rustc-link-search=native={}", dir.display());
                }
                for lib in &blas.libs {
                    println!("cargo:rustc-link-lib={lib}");
                }
            }
        }
        println!("cargo:rustc-cfg=feature_blas");
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

/// The per-op Metal libraries, in the order of `METALLIB_KERNEL_SOURCES` in
/// `ggml/src/ggml-metal/CMakeLists.txt` (and `GGML_METAL_LIBS` in
/// `ggml-metal-device.m`, which references one start/end symbol pair per kind).
const METAL_KERNELS: [&str; 36] = [
    "fa_aux",
    "fa_f16",
    "fa_f32",
    "fa_q4_0",
    "fa_q4_1",
    "fa_q5_0",
    "fa_q5_1",
    "fa_q8_0",
    "fa_tensor",
    "fa_vec_f16",
    "fa_vec_f32",
    "fa_vec_q4_0",
    "fa_vec_q4_1",
    "fa_vec_q5_0",
    "fa_vec_q5_1",
    "fa_vec_q8_0",
    "mul_mv",
    "mul_mv_mma",
    "mul_mm",
    "quantize",
    "softmax",
    "norm",
    "unary",
    "binbcast",
    "reduce",
    "tri",
    "ssm",
    "wkv",
    "gated_delta_net",
    "solve_tri",
    "rope",
    "conv",
    "upscale",
    "argsort",
    "pool",
    "misc",
];

/// Generate the self-contained Metal sources and the assembly file embedding
/// them, mirroring the `GGML_METAL_EMBED_LIBRARY` branch of the upstream CMake:
/// for each kernel, the shared headers it includes are prepended, the internal
/// `#include`s / `#pragma once` dropped, and `ggml-common.h` /
/// `ggml-metal-impl.h` inlined. Returns the path of the assembly file.
fn metal_embed_library(src_path: &Path, backend_dir: &Path, out_path: &Path) -> PathBuf {
    let read = |p: PathBuf| {
        std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("cannot read {}: {e}", p.display()))
    };
    let kernels_dir = backend_dir.join("kernels");

    let ggml_common_h = read(src_path.join("ggml-common.h"));
    let ggml_metal_impl_h = read(backend_dir.join("ggml-metal-impl.h"));

    let common_h = read(kernels_dir.join("common.h"));
    let dequantize_h = read(kernels_dir.join("dequantize.h"));
    let quantize_h = read(kernels_dir.join("quantize.h"));
    let fa_common = read(kernels_dir.join("fa_common.metal"));
    let fa_vec_common = read(kernels_dir.join("fa_vec_common.metal"));

    let stripped = [
        "#include \"common.h\"",
        "#include \"dequantize.h\"",
        "#include \"quantize.h\"",
        "#include \"fa_common.metal\"",
        "#include \"fa_vec_common.metal\"",
        "#pragma once",
    ];

    // replace every line containing `marker` by `content` (sed `/marker/r file` + `/marker/d`)
    let inline = |data: &str, marker: &str, content: &str| -> String {
        let mut res = String::with_capacity(data.len() + content.len());
        for line in data.lines() {
            if line.contains(marker) {
                res.push_str(content);
            } else {
                res.push_str(line);
                res.push('\n');
            }
        }
        res
    };

    let embed_dir = out_path.join("ggml-metal-embed");
    std::fs::create_dir_all(&embed_dir).unwrap();

    let mut asm = String::new();
    asm.push_str(".section __DATA,__ggml_metallib\n");

    for kind in METAL_KERNELS {
        let src = read(kernels_dir.join(format!("{kind}.metal")));

        // only prepend headers that this source actually includes
        let mut data = common_h.clone();
        for (inc, hdr) in [
            ("dequantize.h", &dequantize_h),
            ("quantize.h", &quantize_h),
            ("fa_common.metal", &fa_common),
            ("fa_vec_common.metal", &fa_vec_common),
        ] {
            if src.contains(&format!("#include \"{inc}\"")) {
                data.push_str(hdr);
            }
        }
        data.push_str(&src);

        let data: String = data
            .lines()
            .filter(|l| !stripped.iter().any(|s| l.contains(s)))
            .flat_map(|l| [l, "\n"])
            .collect();
        let data = inline(&data, "__embed_ggml-common.h__", &ggml_common_h);
        let data = inline(&data, "#include \"ggml-metal-impl.h\"", &ggml_metal_impl_h);

        let embed_file = embed_dir.join(format!("ggml-metal-embed-{kind}.metal"));
        std::fs::write(&embed_file, data).unwrap();

        // the macOS section name is limited to 16 chars, so it is shared across
        // kinds and only the global symbols vary
        let sym = kind.replace('-', "_");
        asm.push_str(&format!(
            ".globl _ggml_metallib_{sym}_start\n\
             _ggml_metallib_{sym}_start:\n\
             .incbin \"{}\"\n\
             .globl _ggml_metallib_{sym}_end\n\
             _ggml_metallib_{sym}_end:\n",
            embed_file.display()
        ));
    }

    let asm_path = out_path.join("ggml-metal-embed.s");
    let mut f = std::fs::File::create(&asm_path).unwrap();
    f.write_all(asm.as_bytes()).unwrap();
    asm_path
}

// ---------------------------------------------------------------------------
// llama library
// ---------------------------------------------------------------------------

fn lib_llama(lib_path: &Path, out_path: &Path, target: &Target, ggml_objects: Vec<PathBuf>) {
    let ggml_path = lib_path.join("ggml");
    let ggml_include_path = ggml_path.join("include");
    let src_path = lib_path.join("src");
    let include_path = lib_path.join("include");

    // LLAMA_CORE_SOURCES in src/CMakeLists.txt
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
        "llama-kv-cache-dsa.cpp",
        "llama-kv-cache-dsa-iswa.cpp",
        "llama-kv-cache-msa.cpp",
        "llama-kv-cache-dsv4.cpp",
        "llama-memory.cpp",
        "llama-moe-cache.cpp",
        "llama-memory-hybrid.cpp",
        "llama-memory-hybrid-iswa.cpp",
        "llama-memory-hybrid-idx.cpp",
        "llama-memory-recurrent.cpp",
        "llama-mmap.cpp",
        "llama-model-loader.cpp",
        "llama-model-saver.cpp",
        "llama-model.cpp",
        "llama-quant.cpp",
        "llama-sampler.cpp",
        "llama-vocab.cpp",
        "unicode-data.cpp",
        "unicode.cpp",
    ];

    // `file(GLOB LLAMA_MODELS_SOURCES "models/*.cpp")` upstream
    let models_path = src_path.join("models");
    let mut model_files: Vec<PathBuf> = std::fs::read_dir(&models_path)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", models_path.display()))
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|ext| ext == "cpp"))
        .collect();
    model_files.sort();

    // create a common build
    let mut common = Build::new();
    common.include(include_path);
    common.include(ggml_include_path);
    common.include(&src_path);
    // generated llama-version.h
    common.include(out_path);
    common.opt_level(3);
    let warn = warning_flags(target);
    common.flags(&warn.common);
    base_defines(&mut common, target);
    os_defines(&mut common, target);
    msvc_options(&mut common, target);

    // specialize for CPP (llama is C++ only)
    let mut cpp = common;

    cpp.cpp(true).std("c++17");
    cpp.flags(&warn.cpp_only);
    cpp.objects(ggml_objects);
    cpp.files(cpp_files.into_iter().map(|f| src_path.join(f)));
    cpp.files(model_files);
    // C ABI wrappers over the staging `llama-ext.h` API (see shim/)
    cpp.file("shim/skelm-llama-ext.cpp");

    cpp.compile("llama");
}
