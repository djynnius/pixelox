//! `update` (alias `self-update`): check for a newer release and update pixelox the way it was
//! installed (ported from ferraus).
//!
//! The release is read from the `dist-manifest.json` that cargo-dist attaches to every GitHub
//! release (no API token, no rate limit). A copy placed by the shell or PowerShell installer is
//! updated by running that release's own installer, pinned to the version just checked, into
//! the folder its install receipt names; the installer verifies each archive against the
//! SHA-256 it carries. `cargo install` builds, containers, and copies pixelox cannot place are
//! left alone: `update` reports the command that updates them and changes nothing.

use pixelox_core::prelude::*;
use std::cmp::Ordering;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::Duration;

const RELEASES: &str = "https://github.com/djynnius/pixelox/releases";
const MANIFEST_LIMIT: u64 = 4 << 20;

pub fn ops() -> Vec<OpSpec> {
    vec![OpSpec::custom("update", Category::System, "Check for a newer release and update pixelox", update)
        .alias("self-update")
        .about(
            "Reads the latest release (or --version) from GitHub and compares it with this binary. \
             A copy installed by the release installer (curl ... | sh, irm ... | iex) is updated by \
             that release's installer, pinned to the version just checked and written to the \
             folder the install receipt names. cargo install builds, containers, and copies \
             pixelox cannot place are left alone: the result names the command (`command`) and \
             nothing changes. Never runs on its own; --check only reports. PIXELOX_OFFLINE=1 \
             forbids it.",
        )
        .inputs(Inputs::NONE)
        .param(
            param("check", ParamKind::Flag)
                .help("Only report the current and newest version and how to update; change nothing"),
        )
        .param(
            param("version", ParamKind::Str)
                .help("Install this release instead of the newest, e.g. 0.2.0 (an older one goes back)"),
        )
        .example("pixelox update --check", "Is a newer release out? Shows both versions, its notes, and the command")
        .example("pixelox update", "Update an installer copy to the newest release")
        .example("pixelox update --version 0.2.0", "Go back to (or pin) a specific release")
        .example("pixelox update --dry-run --json", "Show the installer that would run and where, without running it")]
}

/// How this copy of pixelox was installed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Method {
    Installer,
    Cargo,
    Container,
    Unknown,
}

impl Method {
    fn id(self) -> &'static str {
        match self {
            Method::Installer => "installer",
            Method::Cargo => "cargo",
            Method::Container => "container",
            Method::Unknown => "unknown",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Method::Installer => "the release installer",
            Method::Cargo => "cargo install",
            Method::Container => "a container image",
            Method::Unknown => "a method pixelox cannot tell (a source build?)",
        }
    }
}

/// What detection looks at; filled from the real system by [`gather`] and by hand in tests.
pub(crate) struct Facts<'a> {
    pub exe: &'a Path,
    pub install_prefix: Option<&'a Path>,
    pub cargo_bin: Option<&'a Path>,
    pub container: bool,
}

pub(crate) fn detect(f: &Facts) -> Method {
    let dir = f.exe.parent();
    if let (Some(d), Some(p)) = (dir, f.install_prefix) {
        if same_path(d, p) {
            return Method::Installer;
        }
    }
    if f.container {
        return Method::Container;
    }
    if let (Some(d), Some(c)) = (dir, f.cargo_bin) {
        if same_path(d, c) {
            return Method::Cargo;
        }
    }
    Method::Unknown
}

fn same_path(a: &Path, b: &Path) -> bool {
    if a == b {
        return true;
    }
    match (a.canonicalize(), b.canonicalize()) {
        (Ok(x), Ok(y)) => x == y,
        _ => false,
    }
}

/// The command that updates a non-installer copy to `target`.
pub(crate) fn manual_command(m: Method, target: &str, base: &str, windows: bool) -> String {
    match m {
        Method::Installer => "pixelox update".into(),
        Method::Cargo => {
            format!("cargo install --git https://github.com/djynnius/pixelox --tag v{target} --locked pixelox")
        }
        Method::Container => format!("docker build -t pixelox:v{target} https://github.com/djynnius/pixelox.git#v{target} -f packaging/docker/Dockerfile"),
        Method::Unknown => installer_command(&installer_url(base, target, windows), windows),
    }
}

/// A release as `dist-manifest.json` describes it.
#[derive(Debug, PartialEq)]
pub(crate) struct Release {
    pub version: String,
    pub tag: String,
    pub title: String,
    pub notes: String,
}

fn unreadable(m: String) -> Error {
    Error::new("PX1532", m).with_hint(format!("retry later, or open {RELEASES}"))
}

pub(crate) fn parse_release(bytes: &[u8]) -> Result<Release> {
    let v: Value =
        serde_json::from_slice(bytes).map_err(|e| unreadable(format!("the release manifest is not JSON: {e}")))?;
    let version = v["releases"]
        .as_array()
        .and_then(|rs| rs.iter().find(|r| r["app_name"] == "pixelox"))
        .and_then(|r| r["app_version"].as_str())
        .map(str::to_string)
        .ok_or_else(|| unreadable("the release manifest lists no pixelox release".into()))?;
    if parse_version(&version).is_none() {
        return Err(unreadable(format!("the release manifest names an unreadable version '{version}'")));
    }
    Ok(Release {
        tag: v["announcement_tag"].as_str().map(str::to_string).unwrap_or_else(|| format!("v{version}")),
        title: v["announcement_title"].as_str().unwrap_or("").to_string(),
        notes: v["announcement_changelog"].as_str().unwrap_or("").to_string(),
        version,
    })
}

/// `1.2.3` or `1.2.3-rc.1` (a leading `v` is accepted) as numbers plus a pre-release flag.
pub(crate) fn parse_version(s: &str) -> Option<(u64, u64, u64, bool)> {
    let s = s.trim().trim_start_matches('v');
    let (core, pre) = match s.split_once(['-', '+']) {
        Some((c, rest)) => (c, s[c.len()..].starts_with('-') && !rest.is_empty()),
        None => (s, false),
    };
    let mut it = core.split('.');
    let v = (it.next()?.parse().ok()?, it.next()?.parse().ok()?, it.next()?.parse().ok()?);
    if it.next().is_some() {
        return None;
    }
    Some((v.0, v.1, v.2, pre))
}

/// Order two versions; a pre-release sorts before the release with the same numbers.
pub(crate) fn compare_versions(a: &str, b: &str) -> Option<Ordering> {
    let (x, y) = (parse_version(a)?, parse_version(b)?);
    Some((x.0, x.1, x.2).cmp(&(y.0, y.1, y.2)).then_with(|| y.3.cmp(&x.3)))
}

/// An install receipt written by a cargo-dist installer.
#[derive(Debug)]
pub(crate) struct Receipt {
    prefix: PathBuf,
    version: String,
    modify_path: bool,
}

/// Where the installers keep `pixelox-receipt.json`: `$XDG_CONFIG_HOME` or `~/.config` on
/// macOS and Linux, `$XDG_CONFIG_HOME` or `%LOCALAPPDATA%` on Windows, in a `pixelox` folder.
fn receipt_path() -> Option<PathBuf> {
    let env = |k: &str| std::env::var_os(k).filter(|v| !v.is_empty()).map(PathBuf::from);
    let root = env("XDG_CONFIG_HOME").or_else(|| {
        if cfg!(windows) {
            env("LOCALAPPDATA")
        } else {
            env("HOME").map(|h| h.join(".config"))
        }
    })?;
    Some(root.join("pixelox").join("pixelox-receipt.json"))
}

fn read_receipt() -> Option<Receipt> {
    let text = std::fs::read_to_string(receipt_path()?).ok()?;
    let v: Value = serde_json::from_str(&text).ok()?;
    Some(Receipt {
        prefix: PathBuf::from(v["install_prefix"].as_str()?),
        version: v["version"].as_str().unwrap_or("").to_string(),
        modify_path: v["modify_path"].as_bool().unwrap_or(true),
    })
}

fn releases_base() -> String {
    std::env::var("PIXELOX_UPDATE_BASE")
        .ok()
        .filter(|s| !s.is_empty())
        .map(|s| s.trim_end_matches('/').to_string())
        .unwrap_or_else(|| RELEASES.to_string())
}

fn exe_name() -> String {
    format!("pixelox{}", std::env::consts::EXE_SUFFIX)
}

fn installer_url(base: &str, version: &str, windows: bool) -> String {
    let ext = if windows { "ps1" } else { "sh" };
    format!("{base}/download/v{version}/pixelox-installer.{ext}")
}

fn installer_command(url: &str, windows: bool) -> String {
    if windows {
        format!("powershell -ExecutionPolicy ByPass -c \"irm {url} | iex\"")
    } else {
        format!("curl --proto '=https' --tlsv1.2 -LsSf {url} | sh")
    }
}

/// The installer run: its URL, folder, and environment.
pub(crate) fn plan_entry(base: &str, version: &str, r: &Receipt) -> Value {
    let mut env = Map::new();
    env.insert("PIXELOX_INSTALL_DIR".into(), json!(r.prefix.display().to_string()));
    if !r.modify_path {
        env.insert("PIXELOX_NO_MODIFY_PATH".into(), json!("1"));
    }
    json!({
        "from": r.version,
        "to": version,
        "installer": installer_url(base, version, cfg!(windows)),
        "install_dir": r.prefix.display().to_string(),
        "env": env,
    })
}

fn gather(exe: &Path, receipt: Option<&Receipt>) -> Method {
    let cargo_bin = std::env::var_os("CARGO_HOME")
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var_os("HOME")
                .or_else(|| std::env::var_os("USERPROFILE"))
                .map(|h| PathBuf::from(h).join(".cargo"))
        })
        .map(|c| c.join("bin"));
    detect(&Facts {
        exe,
        install_prefix: receipt.map(|r| r.prefix.as_path()),
        cargo_bin: cargo_bin.as_deref(),
        container: Path::new("/.dockerenv").exists() || Path::new("/run/.containerenv").exists(),
    })
}

/// Fetch a small document (the release manifest, an installer script) into memory.
fn get_bytes(url: &str) -> Result<Vec<u8>> {
    let failed =
        |m: String| Error::new("PX1504", format!("{url}: {m}")).with_hint("check the network connection and retry");
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .http_status_as_error(false)
        .timeout_connect(Some(Duration::from_secs(20)))
        .timeout_recv_response(Some(Duration::from_secs(60)))
        .user_agent(format!("pixelox/{}", pixelox_core::PIXELOX_VERSION))
        .build()
        .into();
    let resp = agent.get(url).call().map_err(|e| failed(e.to_string()))?;
    let status = resp.status().as_u16();
    if status != 200 {
        return Err(failed(format!("HTTP {status}")).with_context("status", status));
    }
    let mut out = Vec::new();
    resp.into_body().into_reader().take(MANIFEST_LIMIT).read_to_end(&mut out).map_err(|e| failed(e.to_string()))?;
    Ok(out)
}

fn fetch_release(base: &str, pinned: Option<&str>) -> Result<Release> {
    let url = match pinned {
        Some(v) => format!("{base}/download/v{v}/dist-manifest.json"),
        None => format!("{base}/latest/download/dist-manifest.json"),
    };
    let bytes = get_bytes(&url).map_err(|e| match pinned {
        Some(v) if e.message.contains("HTTP 404") => Error::new("PX1010", format!("there is no pixelox release {v}"))
            .with_hint(format!("open {RELEASES} for the list of releases")),
        _ => e,
    })?;
    parse_release(&bytes)
}

fn offline() -> bool {
    std::env::var("PIXELOX_OFFLINE").map(|v| !v.is_empty() && v != "0").unwrap_or(false)
}

fn update(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    if offline() {
        return Err(Error::new("PX1504", "update needs the network, and PIXELOX_OFFLINE is set")
            .with_hint("unset PIXELOX_OFFLINE and run pixelox update again"));
    }
    let base = releases_base();
    let current = pixelox_core::PIXELOX_VERSION;
    let pinned = a.str("version").map(|v| v.trim().trim_start_matches('v').to_string()).filter(|v| !v.is_empty());
    if let Some(v) = &pinned {
        if parse_version(v).is_none() {
            return Err(Error::new("PX1010", format!("--version {v} is not a version like 0.2.0")));
        }
    }
    let release = fetch_release(&base, pinned.as_deref())?;
    let target = release.version.clone();
    let order = compare_versions(current, &target).unwrap_or(Ordering::Less);
    let newer = order == Ordering::Less;

    let exe = std::env::current_exe().map(|p| p.canonicalize().unwrap_or(p)).unwrap_or_default();
    let receipt = read_receipt();
    let method = gather(&exe, receipt.as_ref());
    let command = manual_command(method, &target, &base, cfg!(windows));
    let release_url = format!("{base}/tag/{}", release.tag);

    let mut out = json!({
        "current": current,
        "target": target,
        "update_available": newer,
        "pinned": pinned.is_some(),
        "release": {"tag": release.tag, "title": release.title, "url": release_url, "notes": release.notes},
        "install": {"method": method.id(), "path": exe.display().to_string()},
        "command": command,
    });

    let wanted = match &pinned {
        Some(_) => order != Ordering::Equal,
        None => newer,
    };
    if a.flag("check") || !wanted {
        out["action"] = json!("none");
        let text = if !wanted {
            format!("pixelox {current} is up to date (newest release: {target}).")
        } else if newer {
            format!(
                "pixelox {target} is available; this is {current}, installed with {}.\nUpdate: {command}\nRelease notes: {release_url}",
                method.label()
            )
        } else {
            format!(
                "pixelox {target} is older than this {current}; `pixelox update --version {target}` goes back to it."
            )
        };
        out["text"] = json!(text);
        return Ok(out);
    }

    let receipt = match (method, receipt) {
        (Method::Installer, Some(r)) => r,
        _ => {
            return Err(Error::new(
                "PX1530",
                format!(
                    "pixelox {current} at {} was installed with {}; update it that way ({target} is the {} release)",
                    exe.display(),
                    method.label(),
                    if pinned.is_some() { "requested" } else { "newest" }
                ),
            )
            .with_hint(command));
        }
    };

    let entry = plan_entry(&base, &target, &receipt);
    out["plan"] = json!([entry.clone()]);
    if ctx.dry_run() {
        out["action"] = json!("would_update");
        out["text"] = json!(format!(
            "Would update pixelox {current} to {target} with {} into {}.",
            installer_url(&base, &target, cfg!(windows)),
            receipt.prefix.display()
        ));
        return Ok(out);
    }

    remove_stale_old(&receipt.prefix);
    run_installer(&entry, &receipt, ctx)?;
    let installed = installed_version(&receipt.prefix)?;
    if compare_versions(&installed, &target) != Some(Ordering::Equal) {
        return Err(Error::new(
            "PX1531",
            format!(
                "the installer finished but {} reports {installed}, not {target}",
                receipt.prefix.join(exe_name()).display()
            ),
        )
        .with_hint(installer_command(&installer_url(&base, &target, cfg!(windows)), cfg!(windows))));
    }
    out["action"] = json!("updated");
    out["installed"] = json!(installed);
    out["text"] = json!(format!(
        "Updated pixelox {current} -> {installed} in {}.\nRelease notes: {release_url}",
        receipt.prefix.display()
    ));
    Ok(out)
}

/// Windows cannot overwrite a running executable but can rename it: the old `pixelox.exe`
/// moves aside before the installer runs and is removed on the next update.
fn remove_stale_old(prefix: &Path) {
    if cfg!(windows) {
        let _ = std::fs::remove_file(prefix.join(format!("{}.old", exe_name())));
    }
}

fn run_installer(entry: &Value, r: &Receipt, ctx: &mut OpCtx) -> Result<()> {
    let url = entry["installer"].as_str().unwrap_or_default();
    let failed = |m: String| Error::new("PX1504", m).with_hint(installer_command(url, cfg!(windows)));
    let script = get_bytes(url)?;
    let dir = std::env::temp_dir().join(format!("pixelox-update-{}", std::process::id()));
    std::fs::create_dir_all(&dir)?;
    let path = dir.join(if cfg!(windows) { "installer.ps1" } else { "installer.sh" });
    std::fs::write(&path, &script)?;

    let mut cmd = if cfg!(windows) {
        let mut c = std::process::Command::new("powershell");
        c.args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-File"]).arg(&path);
        c
    } else {
        let mut c = std::process::Command::new("sh");
        c.arg(&path);
        c
    };
    if let Some(env) = entry["env"].as_object() {
        for (k, v) in env {
            cmd.env(k, v.as_str().unwrap_or_default());
        }
    }

    // Move the running pixelox.exe aside on Windows so the installer can write the new one.
    let running = r.prefix.join(exe_name());
    let aside = r.prefix.join(format!("{}.old", exe_name()));
    let moved = cfg!(windows) && running.exists() && std::fs::rename(&running, &aside).is_ok();

    ctx.progress.log(&format!("running {url}"));
    let output = cmd.output();
    let _ = std::fs::remove_dir_all(&dir);
    let output = match output {
        Ok(o) => o,
        Err(e) => {
            if moved {
                let _ = std::fs::rename(&aside, &running);
            }
            return Err(failed(format!("could not run the pixelox installer: {e}")));
        }
    };
    // The installer's messages are logs: stderr, so stdout keeps only the result.
    {
        use std::io::Write;
        let _ = std::io::stderr().write_all(&output.stdout);
        let _ = std::io::stderr().write_all(&output.stderr);
    }
    if !output.status.success() {
        if moved && !running.exists() {
            let _ = std::fs::rename(&aside, &running);
        }
        let tail: Vec<String> =
            String::from_utf8_lossy(&output.stderr).lines().rev().take(5).map(str::to_string).collect();
        let tail: Vec<String> = tail.into_iter().rev().collect();
        return Err(failed(format!("the pixelox installer exited with {}: {}", output.status, tail.join("\n"))));
    }
    Ok(())
}

/// Ask the freshly installed binary for its version.
fn installed_version(prefix: &Path) -> Result<String> {
    let bin = prefix.join(exe_name());
    let out = std::process::Command::new(&bin)
        .arg("--version")
        .output()
        .map_err(|e| Error::new("PX1531", format!("could not run {}: {e}", bin.display())))?;
    let text = String::from_utf8_lossy(&out.stdout);
    text.split_whitespace()
        .nth(1)
        .map(str::to_string)
        .ok_or_else(|| Error::new("PX1531", format!("{} printed no version", bin.display())))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn facts(exe: &str) -> Facts<'_> {
        Facts { exe: Path::new(exe), install_prefix: None, cargo_bin: None, container: false }
    }

    #[test]
    fn versions_order_numerically_with_prereleases_first() {
        assert_eq!(compare_versions("0.2.0", "0.3.0"), Some(Ordering::Less));
        assert_eq!(compare_versions("0.1.10", "0.1.9"), Some(Ordering::Greater));
        assert_eq!(compare_versions("v1.0.0", "1.0.0"), Some(Ordering::Equal));
        assert_eq!(compare_versions("1.0.0-rc.1", "1.0.0"), Some(Ordering::Less));
        assert_eq!(compare_versions("1.0.0+build.5", "1.0.0"), Some(Ordering::Equal));
        assert_eq!(compare_versions("1.0", "1.0.0"), None);
        assert_eq!(compare_versions("1.0.0.0", "1.0.0"), None);
    }

    #[test]
    fn detects_each_install_method() {
        let prefix = Path::new("/home/a/.pixelox/bin");
        let f = Facts { install_prefix: Some(prefix), ..facts("/home/a/.pixelox/bin/pixelox") };
        assert_eq!(detect(&f), Method::Installer);
        // a receipt for another folder does not make this copy an installer copy
        let f = Facts { install_prefix: Some(prefix), ..facts("/home/a/src/pixelox/target/release/pixelox") };
        assert_eq!(detect(&f), Method::Unknown);
        let cargo = Path::new("/home/a/.cargo/bin");
        let f = Facts { cargo_bin: Some(cargo), ..facts("/home/a/.cargo/bin/pixelox") };
        assert_eq!(detect(&f), Method::Cargo);
        let f = Facts { container: true, ..facts("/usr/local/bin/pixelox") };
        assert_eq!(detect(&f), Method::Container);
    }

    #[test]
    fn commands_name_the_right_tool() {
        let b = RELEASES;
        assert!(manual_command(Method::Cargo, "0.3.0", b, false).contains("--tag v0.3.0"));
        assert!(
            manual_command(Method::Unknown, "0.3.0", b, false).ends_with("/download/v0.3.0/pixelox-installer.sh | sh")
        );
        assert!(
            manual_command(Method::Unknown, "0.3.0", b, true).contains("/download/v0.3.0/pixelox-installer.ps1 | iex")
        );
        assert!(manual_command(Method::Container, "0.3.0", b, false).contains("#v0.3.0"));
    }

    #[test]
    fn reads_the_dist_manifest() {
        let m = br####"{"announcement_tag":"v0.3.0","announcement_title":"0.3.0 - 2026-11-06",
            "announcement_changelog":"### Added\n- update","releases":[{"app_name":"pixelox","app_version":"0.3.0"}]}"####;
        let r = parse_release(m).unwrap();
        assert_eq!((r.version.as_str(), r.tag.as_str(), r.notes.as_str()), ("0.3.0", "v0.3.0", "### Added\n- update"));
        assert_eq!(parse_release(br#"{"releases":[]}"#).unwrap_err().code, "PX1532");
        assert_eq!(parse_release(b"<html>").unwrap_err().code, "PX1532");
    }

    #[test]
    fn installer_env_keeps_the_receipt_choices() {
        let r = Receipt { prefix: PathBuf::from("/opt/pixelox/bin"), version: "0.2.0".into(), modify_path: false };
        let e = plan_entry("https://example.test/releases", "0.3.0", &r);
        assert_eq!(e["env"]["PIXELOX_INSTALL_DIR"], "/opt/pixelox/bin");
        assert_eq!(e["env"]["PIXELOX_NO_MODIFY_PATH"], "1");
        assert!(e["installer"]
            .as_str()
            .unwrap()
            .starts_with("https://example.test/releases/download/v0.3.0/pixelox-installer."));
        let r = Receipt { modify_path: true, ..r };
        assert!(plan_entry("b", "0.3.0", &r)["env"].get("PIXELOX_NO_MODIFY_PATH").is_none());
    }
}
