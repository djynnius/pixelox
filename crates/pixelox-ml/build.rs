//! Sets `cfg(pixelox_ml)` when the `ml` feature is on and the target has a prebuilt ONNX
//! Runtime (ort's download covers glibc Linux, Apple-silicon macOS, and Windows; not musl or
//! Intel macOS, which keep the gated build).

fn main() {
    println!("cargo::rustc-check-cfg=cfg(pixelox_ml)");
    println!("cargo::rerun-if-changed=build.rs");
    let ml = std::env::var_os("CARGO_FEATURE_ML").is_some();
    let env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let supported = env != "musl" && !(os == "macos" && arch == "x86_64");
    if ml && supported {
        println!("cargo::rustc-cfg=pixelox_ml");
        if os == "macos" {
            link_compiler_rt();
        }
    }
}

/// On macOS the prebuilt ONNX Runtime (with CoreML) references `___isPlatformVersionAtLeast`
/// from clang's compiler-rt. Debug builds pick it up implicitly, but release builds with LTO
/// and one codegen unit fail to link without it, so link `libclang_rt.osx.a` explicitly when
/// the toolchain provides it (as ferraus does).
fn link_compiler_rt() {
    let Ok(out) = std::process::Command::new("clang").arg("-print-runtime-dir").output() else { return };
    let dir = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if !dir.is_empty() && std::path::Path::new(&dir).join("libclang_rt.osx.a").exists() {
        println!("cargo::rustc-link-search=native={dir}");
        println!("cargo::rustc-link-lib=static=clang_rt.osx");
    }
}
