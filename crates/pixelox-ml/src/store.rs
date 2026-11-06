//! Installed models: `<models dir>/<name>.onnx` plus `installed.json` (the SHA-256, size,
//! and source of each). Downloads happen only with consent (`-y`), never offline
//! (`PIXELOX_OFFLINE=1`), and every file is verified before it is used for the first time.

use crate::download;
use crate::registry::ModelEntry;
use pixelox_core::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Installed {
    pub sha256: String,
    pub bytes: u64,
    pub url: String,
}

pub fn dir() -> PathBuf {
    pixelox_core::paths::models_dir()
}

pub fn model_path(name: &str) -> PathBuf {
    dir().join(format!("{name}.onnx"))
}

fn lock_path() -> PathBuf {
    dir().join("installed.json")
}

fn load() -> BTreeMap<String, Installed> {
    std::fs::read(lock_path()).ok().and_then(|b| serde_json::from_slice(&b).ok()).unwrap_or_default()
}

fn save(lock: &BTreeMap<String, Installed>) -> Result<()> {
    std::fs::create_dir_all(dir())?;
    let tmp = dir().join("installed.json.tmp");
    std::fs::write(&tmp, serde_json::to_vec_pretty(lock)?)?;
    std::fs::rename(&tmp, lock_path())?;
    Ok(())
}

/// The installed record of `name` when its file is present with the recorded size.
pub fn installed(name: &str) -> Option<Installed> {
    let rec = load().remove(name)?;
    let len = std::fs::metadata(model_path(name)).ok()?.len();
    (len == rec.bytes).then_some(rec)
}

pub fn forget(name: &str) -> Result<Option<Installed>> {
    let mut lock = load();
    let rec = lock.remove(name);
    save(&lock)?;
    let path = model_path(name);
    crate::runtime::evict(&path);
    if path.exists() {
        std::fs::remove_file(&path)?;
    }
    let _ = std::fs::remove_file(download::part_path(&path));
    Ok(rec)
}

pub fn offline() -> bool {
    std::env::var("PIXELOX_OFFLINE").map(|v| !v.is_empty() && v != "0").unwrap_or(false)
}

/// The model's file, downloading it first when `consent` is given (PX1502 otherwise).
pub fn ensure(ctx: &mut OpCtx, e: &ModelEntry, consent: bool) -> Result<(PathBuf, Installed)> {
    let src = e.source.ok_or_else(|| {
        Error::new("PX1520", format!("the {} model is not available in this release", e.name))
            .with_context("model", e.name)
    })?;
    if let Some(rec) = installed(e.name) {
        if rec.sha256 == src.sha256 {
            return Ok((model_path(e.name), rec));
        }
    }
    // a file copied into the models directory by hand (offline installs): adopt it when its
    // hash matches the registry
    let path = model_path(e.name);
    if std::fs::metadata(&path).map(|m| m.len() == src.bytes).unwrap_or(false)
        && download::sha256_file(&path)? == src.sha256
    {
        let rec = Installed { sha256: src.sha256.to_string(), bytes: src.bytes, url: "local file".into() };
        let mut lock = load();
        lock.insert(e.name.to_string(), rec.clone());
        save(&lock)?;
        return Ok((path, rec));
    }
    if !consent {
        return Err(Error::new(
            "PX1502",
            format!("the {} model ({} MB, {}) is not downloaded yet", e.name, src.bytes / 1_000_000, e.license),
        )
        .with_hint(format!("pixelox models pull {} -y   (or add -y to this command to download it first)", e.name))
        .with_context("model", e.name)
        .with_context("bytes", src.bytes)
        .with_context("license", e.license)
        .with_context("url", src.url));
    }
    if offline() {
        return Err(Error::new(
            "PX1504",
            format!("the {} model must be downloaded, and PIXELOX_OFFLINE is set", e.name),
        )
        .with_hint("unset PIXELOX_OFFLINE, or copy the file into the models directory (pixelox models list)"));
    }
    let progress = ctx.progress;
    let cancel = ctx.cancel.clone();
    let stage = format!("download {}", e.name);
    let report = download::download(download::Request {
        url: src.url,
        sha256: src.sha256,
        bytes: src.bytes,
        dest: &path,
        log: &|m| progress.log(m),
        progress: &|f| progress.progress(&stage, f),
        check_cancel: &|| cancel.check(),
    })?;
    let rec = Installed { sha256: report.sha256, bytes: report.size, url: src.url.to_string() };
    let mut lock = load();
    lock.insert(e.name.to_string(), rec.clone());
    save(&lock)?;
    Ok((path, rec))
}
