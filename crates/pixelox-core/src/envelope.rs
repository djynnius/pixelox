//! The `--json` result envelope (PRD §6.1) and the provenance sidecar (§6.4).

use crate::doc::{InputRef, RecipeStep};
use crate::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use std::io::Read;
use std::path::{Path, PathBuf};

/// Envelope major version. Fields are additive-only within a version.
pub const ENVELOPE_VERSION: u32 = 1;
/// Sidecar format version.
pub const SIDECAR_VERSION: u32 = 1;
pub const PIXELOX_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const BUILD_TARGET: &str = env!("PIXELOX_TARGET");
pub const BUILD_PROFILE: &str = env!("PIXELOX_PROFILE");
pub const BUILD_GIT: &str = env!("PIXELOX_GIT");

/// A file read or written, with its hash.
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct FileRecord {
    pub path: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub sha256: String,
    #[serde(default)]
    pub bytes: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sidecar: Option<String>,
    /// `[width, height]` for images.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<[u32; 2]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Dry runs: what would happen (`write`, `overwrite`, `mutate-in-place`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}

/// A structured warning: `{code, message, context}`. Warnings never change the exit code.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Warning {
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub context: Map<String, Value>,
}

/// Every command's single JSON result object on stdout.
#[derive(Clone, Debug, Serialize)]
pub struct Envelope {
    pub tool: &'static str,
    pub version: &'static str,
    pub envelope: u32,
    /// Canonical dotted command id.
    pub command: String,
    pub ok: bool,
    pub dry_run: bool,
    pub seed: u64,
    #[serde(skip_serializing_if = "Value::is_null")]
    pub result: Value,
    pub inputs: Vec<FileRecord>,
    pub outputs: Vec<FileRecord>,
    pub warnings: Vec<Warning>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
    pub timing_ms: u64,
}

impl Envelope {
    pub fn new(command: &str, seed: u64, dry_run: bool) -> Envelope {
        Envelope {
            tool: "pixelox",
            version: PIXELOX_VERSION,
            envelope: ENVELOPE_VERSION,
            command: command.to_string(),
            ok: true,
            dry_run,
            seed,
            result: Value::Null,
            inputs: vec![],
            outputs: vec![],
            warnings: vec![],
            error: None,
            timing_ms: 0,
        }
    }

    pub fn failure(command: &str, e: Error, seed: u64, dry_run: bool, timing_ms: u64) -> Envelope {
        let mut env = Envelope::new(command, seed, dry_run);
        env.ok = false;
        env.error = Some(e);
        env.timing_ms = timing_ms;
        env
    }

    pub fn exit_code(&self) -> i32 {
        self.error.as_ref().map(|e| e.exit_code).unwrap_or(0)
    }
}

/// `tool` block of a sidecar.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ToolInfo {
    pub name: String,
    pub version: String,
    pub build: String,
    pub target: String,
    pub profile: String,
}

impl ToolInfo {
    pub fn current() -> ToolInfo {
        ToolInfo {
            name: "pixelox".into(),
            version: PIXELOX_VERSION.into(),
            build: format!("git:{BUILD_GIT}"),
            target: BUILD_TARGET.into(),
            profile: BUILD_PROFILE.into(),
        }
    }
}

/// `command` block of a sidecar.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CommandInfo {
    pub id: String,
    pub argv: Vec<String>,
    pub params: Map<String, Value>,
    pub seed: u64,
}

/// `output` block of a sidecar.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct OutputInfo {
    pub path: String,
    pub sha256: String,
    pub bytes: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<[u32; 2]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color_profile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bit_depth: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

/// `<output>.pixelox.json`: everything needed to verify or reproduce one output.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Sidecar {
    pub pixelox_sidecar: u32,
    pub tool: ToolInfo,
    pub created_utc: String,
    /// Directory relative paths in this file are resolved against.
    pub base_dir: String,
    pub command: CommandInfo,
    /// The full op chain back to the source inputs.
    pub recipe: Vec<RecipeStep>,
    pub inputs: Vec<InputRef>,
    #[serde(default)]
    pub models: Vec<Value>,
    #[serde(default)]
    pub fonts: Vec<Value>,
    pub output: OutputInfo,
    pub env: Value,
    pub determinism: Value,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<Warning>,
}

impl Sidecar {
    pub fn path_for(output: &Path) -> PathBuf {
        PathBuf::from(format!("{}.pixelox.json", output.display()))
    }

    pub fn determinism_block() -> Value {
        serde_json::json!({
            "bit_exact_scope": "same-version-same-target",
            "perceptual": {"ssim_min": 0.999, "de00_p99_max": 1.0}
        })
    }

    pub fn read(path: &Path) -> crate::Result<Sidecar> {
        let bytes = std::fs::read(path).map_err(|e| match e.kind() {
            std::io::ErrorKind::NotFound => Error::input_missing(path),
            _ => Error::from(e),
        })?;
        serde_json::from_slice(&bytes).map_err(|e| {
            Error::new("PX1603", format!("{} is not a pixelox sidecar: {e}", path.display()))
                .with_context("path", path.display().to_string())
        })
    }
}

/// SHA-256 of a file, streamed.
pub fn sha256_file(path: &Path) -> std::io::Result<String> {
    let mut f = std::fs::File::open(path)?;
    let mut h = Sha256::new();
    let mut buf = vec![0u8; 1 << 16];
    loop {
        let n = f.read(&mut buf)?;
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
    }
    Ok(hex::encode(h.finalize()))
}

pub fn sha256_bytes(b: &[u8]) -> String {
    hex::encode(Sha256::digest(b))
}

/// Hash a file into a record (path kept as given).
pub fn file_record(path: &Path, role: Option<&str>) -> crate::Result<FileRecord> {
    let meta = std::fs::metadata(path)?;
    if meta.is_dir() {
        return Ok(FileRecord {
            path: path.display().to_string(),
            sha256: dir_digest(path)?,
            bytes: 0,
            role: role.map(String::from),
            ..Default::default()
        });
    }
    Ok(FileRecord {
        path: path.display().to_string(),
        sha256: sha256_file(path)?,
        bytes: meta.len(),
        role: role.map(String::from),
        ..Default::default()
    })
}

/// Merkle-style digest of a directory: sha256 over sorted `relpath\0sha256\n` lines.
pub fn dir_digest(dir: &Path) -> crate::Result<String> {
    fn walk(base: &Path, d: &Path, out: &mut Vec<String>) -> std::io::Result<()> {
        for e in std::fs::read_dir(d)? {
            let p = e?.path();
            if p.is_dir() {
                walk(base, &p, out)?;
            } else {
                let rel = p.strip_prefix(base).unwrap_or(&p).to_string_lossy().replace('\\', "/");
                out.push(format!("{rel}\0{}\n", sha256_file(&p)?));
            }
        }
        Ok(())
    }
    let mut lines = Vec::new();
    walk(dir, dir, &mut lines)?;
    lines.sort();
    Ok(sha256_bytes(lines.concat().as_bytes()))
}

/// Current UTC time as `YYYY-MM-DDTHH:MM:SSZ` (metadata only; never read inside ops).
pub fn now_utc() -> String {
    let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    let days = (secs / 86_400) as i64;
    let rem = secs % 86_400;
    // civil-from-days (Howard Hinnant)
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!("{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}Z", rem / 3600, (rem % 3600) / 60, rem % 60)
}

/// Replace a path's directories by a short hash (`--redact-paths`).
pub fn redact_path(p: &str) -> String {
    let path = Path::new(p);
    match (path.parent(), path.file_name()) {
        (Some(dir), Some(name)) if !dir.as_os_str().is_empty() => {
            let h = sha256_bytes(dir.to_string_lossy().as_bytes());
            format!("<{}>/{}", &h[..12], name.to_string_lossy())
        }
        _ => p.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utc_format() {
        let t = now_utc();
        assert_eq!(t.len(), 20);
        assert!(t.ends_with('Z') && t.as_bytes()[10] == b'T');
    }

    #[test]
    fn error_envelope_shape() {
        let e = Envelope::failure("export", Error::new("PX1401", "exists"), 0, false, 3);
        let v = serde_json::to_value(&e).unwrap();
        assert_eq!(v["tool"], "pixelox");
        assert_eq!(v["envelope"], 1);
        assert_eq!(v["ok"], false);
        assert_eq!(v["error"]["code"], "PX1401");
        assert_eq!(v["error"]["docs"], "errors/PX1401");
        assert_eq!(v["error"]["exit_code"], 4);
        assert!(v.get("result").is_none());
        assert_eq!(e.exit_code(), 4);
    }

    #[test]
    fn redaction_keeps_file_name() {
        let r = redact_path("/home/me/work/out.png");
        assert!(r.ends_with("/out.png") && !r.contains("home"));
        assert_eq!(redact_path("out.png"), "out.png");
    }
}
