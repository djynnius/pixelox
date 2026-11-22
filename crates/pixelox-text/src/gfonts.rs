//! Google Fonts, downloaded with consent (ADR 0015, following ADR 0012's pattern).
//!
//! * A curated set of open-licensed families (`assets/google-fonts.json`, built by
//!   `tools/gen_gfonts.py`) is pinned to one commit of github.com/google/fonts: every font
//!   file and license file is listed with its size and SHA-256, and a download is verified
//!   before it is kept (PX1541 on a mismatch).
//! * Any other family of that commit can be fetched by name too (`fonts get "Space Grotesk"
//!   -y`): its `METADATA.pb` lists the files, and the hashes seen are recorded (trust on first
//!   use) in the install record and in every sidecar that uses the font.
//! * Files land in `<data dir>/fonts/google/<family>/` with the family's license file, and
//!   `<data dir>/fonts` is always on the font search path, so a downloaded family works with
//!   `--font` at once. A folder copied there by hand is adopted when its hashes match.
//! * Nothing touches the network without consent: `-y` on the CLI (PX1540, exit 6,
//!   otherwise), `consent: ["font-download"]` over MCP; `PIXELOX_OFFLINE=1` forbids it.

use crate::fonts::{self, norm};
use pixelox_core::envelope::sha256_bytes;
use pixelox_core::paths;
use pixelox_core::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::Duration;

const REGISTRY_JSON: &str = include_str!("../assets/google-fonts.json");
/// Raw-file host for the pinned commit (`PIXELOX_FONTS_URL` overrides it, for mirrors and tests).
const RAW_BASE: &str = "https://raw.githubusercontent.com/google/fonts";

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FileRec {
    pub name: String,
    #[serde(default)]
    pub style: String,
    #[serde(default)]
    pub weight: u16,
    pub bytes: u64,
    pub sha256: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Axis {
    pub tag: String,
    pub min: f64,
    pub max: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Family {
    pub family: String,
    /// `ofl/anton`: the folder in the google/fonts repository.
    pub dir: String,
    pub license: String,
    pub license_file: FileRec,
    #[serde(default)]
    pub axes: Vec<Axis>,
    pub files: Vec<FileRec>,
}

impl Family {
    pub fn bytes(&self) -> u64 {
        self.files.iter().map(|f| f.bytes).sum::<u64>() + self.license_file.bytes
    }

    /// The cache folder name (`anton`).
    pub fn slug(&self) -> &str {
        self.dir.rsplit('/').next().unwrap_or(&self.dir)
    }
}

#[derive(Debug, Deserialize)]
pub struct Registry {
    pub source: String,
    pub commit: String,
    pub families: Vec<Family>,
}

pub fn registry() -> &'static Registry {
    static R: OnceLock<Registry> = OnceLock::new();
    R.get_or_init(|| serde_json::from_str(REGISTRY_JSON).expect("assets/google-fonts.json is valid"))
}

/// The curated family named `name` (case, spaces, and punctuation ignored).
pub fn curated(name: &str) -> Option<&'static Family> {
    let n = norm(name);
    registry().families.iter().find(|f| norm(&f.family) == n)
}

/// Root of the font cache; always on the font search path.
pub fn cache_root() -> PathBuf {
    paths::data_dir().join("fonts")
}

fn google_dir() -> PathBuf {
    cache_root().join("google")
}

fn record_path() -> PathBuf {
    google_dir().join("installed.json")
}

/// What `installed.json` says about one downloaded family.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Installed {
    pub family: String,
    pub dir: String,
    pub commit: String,
    pub license: String,
    pub curated: bool,
    pub files: Vec<FileRec>,
    pub license_file: FileRec,
}

fn load_records() -> BTreeMap<String, Installed> {
    std::fs::read(record_path()).ok().and_then(|b| serde_json::from_slice(&b).ok()).unwrap_or_default()
}

fn save_records(r: &BTreeMap<String, Installed>) -> Result<()> {
    std::fs::create_dir_all(google_dir())?;
    let p = record_path();
    let tmp = pixelox_core::ctx::tmp_path(&p);
    std::fs::write(&tmp, serde_json::to_vec_pretty(r)?)?;
    std::fs::rename(&tmp, &p)?;
    Ok(())
}

fn family_dir(slug: &str) -> PathBuf {
    google_dir().join(slug)
}

/// The install record of `name` when every file is present with its recorded size.
pub fn installed(name: &str) -> Option<Installed> {
    let rec = load_records().remove(&norm(name))?;
    let dir = family_dir(rec.dir.rsplit('/').next().unwrap_or(&rec.dir));
    rec.files
        .iter()
        .all(|f| std::fs::metadata(dir.join(&f.name)).map(|m| m.len() == f.bytes).unwrap_or(false))
        .then_some(rec)
}

/// Every downloaded family (name, record), sorted by name.
pub fn installed_all() -> Vec<Installed> {
    load_records().into_values().collect()
}

/// The download source recorded for a font file hash, when it came from Google Fonts.
pub fn source_of_sha(sha: &str) -> Option<(String, String)> {
    for r in load_records().into_values() {
        if r.files.iter().any(|f| f.sha256 == sha) {
            return Some((r.family.clone(), format!("google-fonts:{}@{}", r.dir, short(&r.commit))));
        }
    }
    for f in &registry().families {
        if f.files.iter().any(|x| x.sha256 == sha) {
            return Some((f.family.clone(), format!("google-fonts:{}@{}", f.dir, short(&registry().commit))));
        }
    }
    None
}

fn short(c: &str) -> &str {
    &c[..c.len().min(12)]
}

pub fn offline() -> bool {
    std::env::var("PIXELOX_OFFLINE").map(|v| !v.is_empty() && v != "0").unwrap_or(false)
}

fn base_url() -> String {
    std::env::var("PIXELOX_FONTS_URL").ok().filter(|s| !s.trim().is_empty()).unwrap_or_else(|| RAW_BASE.to_string())
}

fn encode(path: &str) -> String {
    let mut out = String::new();
    for b in path.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' | b'/' => out.push(b as char),
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

fn file_url(commit: &str, dir: &str, name: &str) -> String {
    format!("{}/{}/{}", base_url().trim_end_matches('/'), commit, encode(&format!("{dir}/{name}")))
}

fn agent() -> ureq::Agent {
    ureq::Agent::config_builder()
        .http_status_as_error(false)
        .timeout_connect(Some(Duration::from_secs(20)))
        .timeout_recv_response(Some(Duration::from_secs(60)))
        .timeout_recv_body(Some(Duration::from_secs(120)))
        .user_agent(format!("pixelox/{}", pixelox_core::PIXELOX_VERSION))
        .build()
        .into()
}

fn failed(url: &str, msg: &str) -> Error {
    Error::new("PX1504", format!("could not download {url}: {msg}"))
        .with_hint("check the network connection and retry the same command")
        .with_context("url", url)
}

/// GET `url` into memory (fonts are small); `Ok(None)` on 404.
fn fetch(agent: &ureq::Agent, url: &str) -> Result<Option<Vec<u8>>> {
    use std::io::Read;
    let resp = agent.get(url).call().map_err(|e| failed(url, &e.to_string()))?;
    match resp.status().as_u16() {
        200 => {}
        404 => return Ok(None),
        s => return Err(failed(url, &format!("HTTP {s}"))),
    }
    let mut buf = Vec::new();
    resp.into_body()
        .into_reader()
        .take(64 << 20)
        .read_to_end(&mut buf)
        .map_err(|e| failed(url, &format!("connection interrupted: {e}")))?;
    Ok(Some(buf))
}

/// Find a family that is not in the curated set at the pinned commit, through its
/// `METADATA.pb` (files are listed there; hashes are taken from the download).
fn lookup_uncurated(agent: &ureq::Agent, name: &str) -> Result<Option<Family>> {
    let slug: String = name.chars().filter(|c| c.is_ascii_alphanumeric()).flat_map(char::to_lowercase).collect();
    if slug.is_empty() {
        return Ok(None);
    }
    let commit = &registry().commit;
    for lic in ["ofl", "apache", "ufl"] {
        let dir = format!("{lic}/{slug}");
        let Some(meta) = fetch(agent, &file_url(commit, &dir, "METADATA.pb"))? else { continue };
        let text = String::from_utf8_lossy(&meta).to_string();
        let mut fam = parse_metadata(&text, &dir);
        fam.license_file.name = match lic {
            "ofl" => "OFL.txt",
            "apache" => "LICENSE.txt",
            _ => "UFL.txt",
        }
        .to_string();
        if fam.files.is_empty() {
            return Ok(None);
        }
        return Ok(Some(fam));
    }
    Ok(None)
}

/// The fields of a google/fonts `METADATA.pb` (protobuf text format) that matter here.
pub fn parse_metadata(text: &str, dir: &str) -> Family {
    let mut family = String::new();
    let mut license = String::from("OFL");
    let mut files: Vec<FileRec> = Vec::new();
    let mut cur: Option<FileRec> = None;
    let unq = |v: &str| v.trim().trim_matches('"').to_string();
    for line in text.lines() {
        let l = line.trim();
        if l.starts_with("fonts {") {
            cur = Some(FileRec {
                name: String::new(),
                style: "normal".into(),
                weight: 400,
                bytes: 0,
                sha256: String::new(),
            });
            continue;
        }
        if l == "}" {
            if let Some(f) = cur.take().filter(|f| !f.name.is_empty()) {
                if !files.iter().any(|x| x.name == f.name) {
                    files.push(f);
                }
            }
            continue;
        }
        let Some((k, v)) = l.split_once(':') else { continue };
        match (cur.as_mut(), k.trim()) {
            (Some(f), "filename") => f.name = unq(v),
            (Some(f), "style") => f.style = unq(v),
            (Some(f), "weight") => f.weight = v.trim().parse().unwrap_or(400),
            (None, "name") if family.is_empty() => family = unq(v),
            (None, "license") => license = unq(v),
            _ => {}
        }
    }
    Family {
        family,
        dir: dir.to_string(),
        license,
        license_file: FileRec { name: String::new(), style: String::new(), weight: 0, bytes: 0, sha256: String::new() },
        axes: Vec::new(),
        files,
    }
}

/// What a `get` did.
#[derive(Debug)]
pub struct GetReport {
    pub family: Family,
    pub status: &'static str,
    pub curated: bool,
    pub fetched_bytes: u64,
    pub dir: PathBuf,
}

impl GetReport {
    pub fn to_json(&self) -> Value {
        json!({
            "family": self.family.family,
            "status": self.status,
            "curated": self.curated,
            "source": format!("{}/tree/{}/{}", registry().source, registry().commit, self.family.dir),
            "commit": registry().commit,
            "license": self.family.license,
            "dir": self.dir.display().to_string(),
            "license_file": self.dir.join(&self.family.license_file.name).display().to_string(),
            "files": self.family.files.iter().map(|f| json!({
                "file": self.dir.join(&f.name).display().to_string(),
                "style": f.style, "weight": f.weight, "bytes": f.bytes, "sha256": f.sha256,
            })).collect::<Vec<_>>(),
            "bytes": self.family.bytes(),
            "fetched_bytes": self.fetched_bytes,
        })
    }
}

/// The consent error for downloading `what` (a family or `@class`).
pub fn consent_error(what: &str, fam: Option<&Family>) -> Error {
    let (size, license) = match fam {
        Some(f) => (format!("{} KB", f.bytes().div_ceil(1024)), f.license.clone()),
        None => ("size unknown".to_string(), "open license".to_string()),
    };
    let label = fam.map(|f| f.family.clone()).unwrap_or_else(|| what.to_string());
    Error::new(
        "PX1540",
        format!(
            "the font family '{label}' is not installed; it can be downloaded from Google Fonts ({size}, {license})"
        ),
    )
    .with_hint(format!("{}   (or add -y to this command to download it first)", get_command(what)))
    .with_context("family", label)
    .with_context("license", license)
    .with_context("source", format!("{}@{}", registry().source, short(&registry().commit)))
}

/// `pixelox fonts get "Family" -y`.
pub fn get_command(what: &str) -> String {
    if what.starts_with('@') || !what.contains(' ') {
        format!("pixelox fonts get {what} -y")
    } else {
        format!("pixelox fonts get \"{what}\" -y")
    }
}

fn verify(data: &[u8], want: &FileRec, url: &str) -> Result<String> {
    let sha = sha256_bytes(data);
    if !want.sha256.is_empty() && !sha.eq_ignore_ascii_case(&want.sha256) {
        return Err(Error::new(
            "PX1541",
            format!("{url} failed SHA-256 verification (got {sha}, expected {})", want.sha256),
        )
        .with_hint("retry once; a repeated mismatch means the source changed or a proxy altered the file")
        .with_context("url", url));
    }
    Ok(sha)
}

fn write_atomic(path: &Path, data: &[u8]) -> Result<()> {
    if let Some(p) = path.parent() {
        std::fs::create_dir_all(p)?;
    }
    let tmp = pixelox_core::ctx::tmp_path(path);
    std::fs::write(&tmp, data)?;
    std::fs::rename(&tmp, path)?;
    Ok(())
}

fn adopt(fam: &Family, dir: &Path) -> bool {
    fam.files.iter().chain(std::iter::once(&fam.license_file)).all(|f| {
        std::fs::read(dir.join(&f.name))
            .map(|d| d.len() as u64 == f.bytes && sha256_bytes(&d) == f.sha256)
            .unwrap_or(false)
    })
}

fn record(fam: &Family, curated: bool) -> Result<()> {
    let mut recs = load_records();
    recs.insert(
        norm(&fam.family),
        Installed {
            family: fam.family.clone(),
            dir: fam.dir.clone(),
            commit: registry().commit.clone(),
            license: fam.license.clone(),
            curated,
            files: fam.files.clone(),
            license_file: fam.license_file.clone(),
        },
    );
    save_records(&recs)
}

/// Make `name` (a family) available: already installed, adopted from files copied by hand,
/// or downloaded with `consent`. `dry_run` reports the plan without touching anything.
pub fn get(ctx: &mut OpCtx, name: &str, consent: bool) -> Result<GetReport> {
    let cur = curated(name);
    if let Some(fam) = cur {
        let dir = family_dir(fam.slug());
        if let Some(rec) = installed(&fam.family) {
            if rec.files == fam.files {
                return Ok(GetReport {
                    family: fam.clone(),
                    status: "installed",
                    curated: true,
                    fetched_bytes: 0,
                    dir,
                });
            }
        }
        if adopt(fam, &dir) {
            if !ctx.dry_run() {
                record(fam, true)?;
                fonts::reload();
            }
            return Ok(GetReport { family: fam.clone(), status: "adopted", curated: true, fetched_bytes: 0, dir });
        }
    } else if let Some(rec) = installed(name) {
        let dir = family_dir(rec.dir.rsplit('/').next().unwrap_or(&rec.dir));
        let fam = Family {
            family: rec.family.clone(),
            dir: rec.dir.clone(),
            license: rec.license.clone(),
            license_file: rec.license_file.clone(),
            axes: Vec::new(),
            files: rec.files.clone(),
        };
        return Ok(GetReport { family: fam, status: "installed", curated: false, fetched_bytes: 0, dir });
    }
    if ctx.dry_run() {
        let fam = cur.cloned().unwrap_or_else(|| Family {
            family: name.to_string(),
            dir: String::new(),
            license: String::new(),
            license_file: FileRec {
                name: String::new(),
                style: String::new(),
                weight: 0,
                bytes: 0,
                sha256: String::new(),
            },
            axes: Vec::new(),
            files: Vec::new(),
        });
        let dir = family_dir(if fam.dir.is_empty() { "?" } else { fam.slug() });
        ctx.plan_add(
            "downloads",
            json!({"family": fam.family, "bytes": fam.bytes(), "curated": cur.is_some(), "consent": consent}),
        );
        return Ok(GetReport { family: fam, status: "planned", curated: cur.is_some(), fetched_bytes: 0, dir });
    }
    if !consent {
        return Err(consent_error(name, cur));
    }
    if offline() {
        return Err(Error::new("PX1504", format!("'{name}' must be downloaded, and PIXELOX_OFFLINE is set"))
            .with_hint(format!(
                "unset PIXELOX_OFFLINE, or copy the family's files into {} (they are adopted when their hashes match)",
                google_dir().display()
            )));
    }
    let agent = agent();
    let (mut fam, is_curated) = match cur {
        Some(f) => (f.clone(), true),
        None => match lookup_uncurated(&agent, name)? {
            Some(f) => (f, false),
            None => {
                return Err(Error::new(
                    "PX4111",
                    format!("'{name}' is not a Google Fonts family (looked in google/fonts at {})", short(&registry().commit)),
                )
                .with_hint("check the spelling at fonts.google.com, list the curated families with `pixelox fonts classes`, or register your own files with `pixelox fonts add <dir>`")
                .with_context("family", name))
            }
        },
    };
    let commit = registry().commit.clone();
    let dir = family_dir(fam.slug());
    let mut fetched = 0u64;
    let files: Vec<FileRec> = fam.files.clone();
    let mut done = Vec::new();
    for (i, f) in files.iter().chain(std::iter::once(&fam.license_file)).enumerate() {
        ctx.check_cancel()?;
        let url = file_url(&commit, &fam.dir, &f.name);
        ctx.progress.log(&format!("downloading {url}"));
        let data = fetch(&agent, &url)?.ok_or_else(|| failed(&url, "HTTP 404"))?;
        let sha = verify(&data, f, &url)?;
        fetched += data.len() as u64;
        write_atomic(&dir.join(&f.name), &data)?;
        let mut rec = f.clone();
        rec.sha256 = sha;
        rec.bytes = data.len() as u64;
        done.push(rec);
        ctx.progress.progress(&format!("fonts get {}", fam.family), (i + 1) as f64 / (files.len() + 1) as f64);
    }
    let lic = done.pop().expect("license file");
    fam.files = done;
    fam.license_file = lic;
    record(&fam, is_curated)?;
    fonts::reload();
    Ok(GetReport { family: fam, status: "downloaded", curated: is_curated, fetched_bytes: fetched, dir })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_is_pinned_and_complete() {
        let r = registry();
        assert_eq!(r.commit.len(), 40);
        assert!(r.families.len() >= 40 && r.families.len() <= 70, "{}", r.families.len());
        for f in &r.families {
            assert!(!f.files.is_empty(), "{}", f.family);
            assert!(f.license == "OFL" || f.license.starts_with("APACHE") || f.license == "UFL", "{}", f.family);
            for x in f.files.iter().chain(std::iter::once(&f.license_file)) {
                assert_eq!(x.sha256.len(), 64, "{} {}", f.family, x.name);
                assert!(x.bytes > 0);
            }
        }
        assert_eq!(curated("bebas neue").unwrap().family, "Bebas Neue");
        assert_eq!(curated("JetBrainsMono").unwrap().dir, "ofl/jetbrainsmono");
        assert!(curated("Comic Sans").is_none());
    }

    #[test]
    fn urls_are_encoded() {
        assert_eq!(encode("ofl/inter/Inter[opsz,wght].ttf"), "ofl/inter/Inter%5Bopsz%2Cwght%5D.ttf");
        assert_eq!(get_command("Anton"), "pixelox fonts get Anton -y");
        assert_eq!(get_command("Bebas Neue"), "pixelox fonts get \"Bebas Neue\" -y");
        assert_eq!(get_command("@stencil"), "pixelox fonts get @stencil -y");
    }

    #[test]
    fn metadata_parses() {
        let text = "name: \"Space Grotesk\"\ndesigner: \"x\"\nlicense: \"OFL\"\nfonts {\n  name: \"Space Grotesk\"\n  style: \"normal\"\n  weight: 300\n  filename: \"SpaceGrotesk[wght].ttf\"\n}\nfonts {\n  name: \"Space Grotesk\"\n  style: \"italic\"\n  weight: 400\n  filename: \"SpaceGrotesk-Italic.ttf\"\n}\n";
        let f = parse_metadata(text, "ofl/spacegrotesk");
        assert_eq!(f.family, "Space Grotesk");
        assert_eq!(f.license, "OFL");
        assert_eq!(f.files.len(), 2);
        assert_eq!(f.files[0].name, "SpaceGrotesk[wght].ttf");
        assert_eq!(f.files[0].weight, 300);
        assert_eq!(f.files[1].style, "italic");
    }
}
