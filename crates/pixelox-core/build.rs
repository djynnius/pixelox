//! Build metadata recorded in sidecars: target triple, profile, and git revision.

fn main() {
    let target = std::env::var("TARGET").unwrap_or_else(|_| "unknown".into());
    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "unknown".into());
    println!("cargo:rustc-env=PIXELOX_TARGET={target}");
    println!("cargo:rustc-env=PIXELOX_PROFILE={profile}");
    let git = std::env::var("PIXELOX_GIT_REV").ok().or_else(|| {
        std::process::Command::new("git")
            .args(["rev-parse", "--short", "HEAD"])
            .output()
            .ok()
            .filter(|o| o.status.success())
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
    });
    println!("cargo:rustc-env=PIXELOX_GIT={}", git.unwrap_or_else(|| "unknown".into()));
    println!("cargo:rerun-if-env-changed=PIXELOX_GIT_REV");
    for p in ["../../.git/HEAD", "../../.git/refs/heads"] {
        if std::path::Path::new(p).exists() {
            println!("cargo:rerun-if-changed={p}");
        }
    }
}
