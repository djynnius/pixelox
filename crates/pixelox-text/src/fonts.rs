//! Font discovery and matching: fontdb over the system fonts, the folders registered with
//! `fonts add` (one per line in `paths::fonts_file()`), the Google Fonts cache (`gfonts`),
//! and the embedded Tuffy fallback; case-insensitive family matching with the CSS
//! font-matching order (width first, then style, then weight); font classes (`@didone`);
//! substitution of a missing family by the best installed face of its class (ADR 0015);
//! a small cache of font files with their SHA-256; and the `fonts *` commands.

use crate::classes::{self, Cand, FontClass, Genre, CLASSES};
use crate::{gfonts, metrics};
use pixelox_core::ctx::absolutize;
use pixelox_core::envelope::sha256_bytes;
use pixelox_core::paths;
use pixelox_core::prelude::*;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};

/// The embedded fallback face (Tuffy Regular, public domain).
pub(crate) const TUFFY: &[u8] = include_bytes!("../assets/Tuffy.ttf");
/// Family name of the embedded fallback.
pub const FALLBACK_FAMILY: &str = "Tuffy";
const EMBEDDED_KEY: &str = "(embedded) Tuffy.ttf";

/// System families tried, in order, for the generic names.
const GENERIC_SANS: &[&str] =
    &["Helvetica", "Arial", "Noto Sans", "DejaVu Sans", "Liberation Sans", "Segoe UI", "Roboto", "Open Sans"];
const GENERIC_SERIF: &[&str] =
    &["Times New Roman", "Times", "Georgia", "Noto Serif", "DejaVu Serif", "Liberation Serif"];
const GENERIC_MONO: &[&str] =
    &["Menlo", "Consolas", "Courier New", "DejaVu Sans Mono", "Noto Sans Mono", "Liberation Mono", "Courier"];

/// Families searched (after the embedded fallback) for characters the chosen face lacks.
const FALLBACK_CHAIN: &[&str] = &[
    "Noto Sans",
    "DejaVu Sans",
    "Arial Unicode MS",
    "Arial",
    "Helvetica",
    "Segoe UI",
    "Segoe UI Symbol",
    "Apple Symbols",
    "Noto Sans Symbols",
    "Noto Sans Symbols 2",
    "PingFang SC",
    "Hiragino Sans",
    "Noto Sans CJK SC",
    "Microsoft YaHei",
    "Apple SD Gothic Neo",
    "Malgun Gothic",
    "Geeza Pro",
    "Noto Sans Arabic",
    "Arial Hebrew",
    "Noto Sans Hebrew",
    "Kohinoor Devanagari",
    "Noto Sans Devanagari",
    "Nirmala UI",
    "Thonburi",
    "Noto Sans Thai",
];

/// Font style request / face style.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

impl FontStyle {
    pub fn parse(s: &str) -> Option<FontStyle> {
        Some(match s.trim().to_ascii_lowercase().as_str() {
            "normal" | "regular" | "upright" | "roman" => FontStyle::Normal,
            "italic" => FontStyle::Italic,
            "oblique" | "slanted" => FontStyle::Oblique,
            _ => return None,
        })
    }

    pub fn name(self) -> &'static str {
        match self {
            FontStyle::Normal => "normal",
            FontStyle::Italic => "italic",
            FontStyle::Oblique => "oblique",
        }
    }

    fn from_db(s: fontdb::Style) -> FontStyle {
        match s {
            fontdb::Style::Normal => FontStyle::Normal,
            fontdb::Style::Italic => FontStyle::Italic,
            fontdb::Style::Oblique => FontStyle::Oblique,
        }
    }
}

// ---------------------------------------------------------------- width (font-stretch)

/// CSS font-stretch keywords, 1 (ultra-condensed) .. 9 (ultra-expanded).
pub const STRETCH_NAMES: [&str; 9] = [
    "ultra-condensed",
    "extra-condensed",
    "condensed",
    "semi-condensed",
    "normal",
    "semi-expanded",
    "expanded",
    "extra-expanded",
    "ultra-expanded",
];
const STRETCH_PCT: [f32; 9] = [50.0, 62.5, 75.0, 87.5, 100.0, 112.5, 125.0, 150.0, 200.0];

/// Percent width of keyword `kw` (1..9).
pub fn stretch_percent(kw: u8) -> f32 {
    STRETCH_PCT[(kw.clamp(1, 9) - 1) as usize]
}

/// The keyword (1..9) nearest a percent width.
pub fn stretch_keyword(pct: f32) -> u8 {
    let mut best = 5u8;
    let mut d = f32::MAX;
    for (i, p) in STRETCH_PCT.iter().enumerate() {
        let e = (p - pct).abs();
        if e < d - 1e-6 {
            d = e;
            best = i as u8 + 1;
        }
    }
    best
}

pub fn stretch_name(pct: f32) -> String {
    let kw = stretch_keyword(pct);
    if (stretch_percent(kw) - pct).abs() < 1e-3 {
        STRETCH_NAMES[kw as usize - 1].to_string()
    } else {
        format!("{pct}%")
    }
}

/// `condensed`, `extended`, `75%`, or `75` → percent of the normal width (50..200).
pub fn parse_stretch(s: &str) -> Result<f32> {
    let t = s.trim().to_ascii_lowercase().replace(['_', ' '], "-");
    let named = match t.as_str() {
        "ultra-condensed" | "ultracondensed" => Some(1),
        "extra-condensed" | "extracondensed" | "compressed" => Some(2),
        "condensed" | "narrow" => Some(3),
        "semi-condensed" | "semicondensed" => Some(4),
        "normal" | "regular" | "medium" => Some(5),
        "semi-expanded" | "semiexpanded" | "semi-extended" => Some(6),
        "expanded" | "extended" | "wide" => Some(7),
        "extra-expanded" | "extraexpanded" | "extra-extended" => Some(8),
        "ultra-expanded" | "ultraexpanded" | "ultra-extended" => Some(9),
        _ => None,
    };
    if let Some(k) = named {
        return Ok(stretch_percent(k));
    }
    match t.trim_end_matches('%').parse::<f32>() {
        Ok(p) if (50.0..=200.0).contains(&p) => Ok(p),
        Ok(p) => Err(Error::new("PX1013", format!("--stretch {p}% is outside 50%..200%")).with_context("stretch", s)),
        Err(_) => Err(Error::usage(format!("--stretch: '{s}' is not a width")).with_hint(
            "use condensed, semi-condensed, normal, expanded (extended), ultra-condensed, ..., or a percent like 75%",
        )),
    }
}

// ---------------------------------------------------------------- the database

/// The loaded font database.
pub struct FontDb {
    pub db: fontdb::Database,
    /// Folders registered with `fonts add`.
    pub dirs: Vec<PathBuf>,
    /// System fonts were loaded (`PIXELOX_SYSTEM_FONTS=0` turns them off).
    pub system: bool,
    /// Normalized family name → faces.
    index: BTreeMap<String, Vec<fontdb::ID>>,
}

static DB: RwLock<Option<Arc<FontDb>>> = RwLock::new(None);

/// The shared font database, loaded on first use.
pub fn db() -> Arc<FontDb> {
    if let Some(d) = DB.read().unwrap_or_else(|e| e.into_inner()).as_ref() {
        return d.clone();
    }
    let mut w = DB.write().unwrap_or_else(|e| e.into_inner());
    if let Some(d) = w.as_ref() {
        return d.clone();
    }
    let d = Arc::new(load_db());
    *w = Some(d.clone());
    d
}

/// Forget the loaded database (after `fonts add` or `fonts get`), so the next lookup rescans.
pub fn reload() {
    *DB.write().unwrap_or_else(|e| e.into_inner()) = None;
}

fn system_fonts_enabled() -> bool {
    !matches!(
        std::env::var("PIXELOX_SYSTEM_FONTS").map(|v| v.trim().to_ascii_lowercase()).as_deref(),
        Ok("0" | "off" | "false" | "no")
    )
}

/// `PIXELOX_STRICT_FONTS=1`: a missing family is an error (PX4102 / PX1540), as before 0.5.
pub fn strict_env() -> bool {
    std::env::var("PIXELOX_STRICT_FONTS").map(|v| !v.is_empty() && v != "0" && v != "false").unwrap_or(false)
}

fn load_db() -> FontDb {
    let mut db = fontdb::Database::new();
    let system = system_fonts_enabled();
    if system {
        db.load_system_fonts();
    }
    let dirs = registered_dirs();
    for d in &dirs {
        if d.is_dir() {
            db.load_fonts_dir(d);
        }
    }
    let cache = gfonts::cache_root();
    if cache.is_dir() {
        db.load_fonts_dir(&cache);
    }
    db.load_font_source(fontdb::Source::Binary(Arc::new(TUFFY)));
    let mut index: BTreeMap<String, Vec<fontdb::ID>> = BTreeMap::new();
    for f in db.faces() {
        for (name, _) in &f.families {
            let ids = index.entry(norm(name)).or_default();
            if !ids.contains(&f.id) {
                ids.push(f.id);
            }
        }
    }
    FontDb { db, dirs, system, index }
}

/// Folders listed in the fonts file (one per line; `#` comments and blanks ignored).
pub fn registered_dirs() -> Vec<PathBuf> {
    let text = std::fs::read_to_string(paths::fonts_file()).unwrap_or_default();
    let mut out: Vec<PathBuf> = Vec::new();
    for l in text.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')) {
        let p = PathBuf::from(l);
        if !out.contains(&p) {
            out.push(p);
        }
    }
    out
}

fn is_embedded(f: &fontdb::FaceInfo) -> bool {
    matches!(f.source, fontdb::Source::Binary(_))
}

fn face_path(src: &fontdb::Source) -> Option<PathBuf> {
    match src {
        fontdb::Source::File(p) => Some(p.clone()),
        fontdb::Source::SharedFile(p, _) => Some(p.clone()),
        fontdb::Source::Binary(_) => None,
    }
}

fn source_key(src: &fontdb::Source) -> String {
    face_path(src).map(|p| p.display().to_string()).unwrap_or_else(|| EMBEDDED_KEY.to_string())
}

/// A font file's bytes and SHA-256.
pub struct FontFile {
    pub key: String,
    pub path: Option<PathBuf>,
    pub data: Vec<u8>,
    pub sha256: String,
}

static FILES: Mutex<Vec<Arc<FontFile>>> = Mutex::new(Vec::new());
static SHAS: Mutex<BTreeMap<String, String>> = Mutex::new(BTreeMap::new());
/// Font bytes kept in memory between renders (the oldest files are dropped first).
const FILE_CACHE_BYTES: usize = 192 << 20;

/// Read (or reuse) the file holding face `id`.
pub fn load_file(db: &FontDb, id: fontdb::ID) -> Result<Arc<FontFile>> {
    let info = db.db.face(id).ok_or_else(|| Error::internal("font face vanished from the database"))?;
    let key = source_key(&info.source);
    if let Some(f) = FILES.lock().unwrap_or_else(|e| e.into_inner()).iter().find(|f| f.key == key) {
        return Ok(f.clone());
    }
    let data = db
        .db
        .with_face_data(id, |d, _| d.to_vec())
        .ok_or_else(|| Error::new("PX4101", format!("cannot read font file {key}")).with_context("file", &key))?;
    let sha256 = sha256_bytes(&data);
    SHAS.lock().unwrap_or_else(|e| e.into_inner()).insert(key.clone(), sha256.clone());
    let f = Arc::new(FontFile { key, path: face_path(&info.source), data, sha256 });
    let mut files = FILES.lock().unwrap_or_else(|e| e.into_inner());
    files.push(f.clone());
    while files.len() > 1 && files.iter().map(|f| f.data.len()).sum::<usize>() > FILE_CACHE_BYTES {
        files.remove(0);
    }
    Ok(f)
}

fn file_sha(db: &FontDb, f: &fontdb::FaceInfo) -> Option<String> {
    let key = source_key(&f.source);
    if let Some(s) = SHAS.lock().unwrap_or_else(|e| e.into_inner()).get(&key) {
        return Some(s.clone());
    }
    // hash through the memory map: candidates are not kept in the file cache
    let sha = db.db.with_face_data(f.id, |d, _| sha256_bytes(d))?;
    SHAS.lock().unwrap_or_else(|e| e.into_inner()).insert(key, sha.clone());
    Some(sha)
}

/// Whether any installed face lives in a file with SHA-256 `sha` (and which family).
pub fn find_sha(sha: &str) -> Option<String> {
    let db = db();
    let faces: Vec<&fontdb::FaceInfo> = db.db.faces().collect();
    faces.into_iter().find(|f| file_sha(&db, f).as_deref() == Some(sha)).map(primary_family)
}

/// A resolved face: which file, which face in it, and what it is.
#[derive(Clone)]
pub struct FaceChoice {
    pub file: Arc<FontFile>,
    pub index: u32,
    pub family: String,
    pub postscript: String,
    pub weight: u16,
    /// Width keyword 1..9 (5 = normal).
    pub stretch: u8,
    pub style: FontStyle,
    pub embedded: bool,
    /// Variation axes of the face: (tag, min, max).
    pub axes: Vec<(String, f32, f32)>,
}

impl FaceChoice {
    /// The sidecar record (`ctx.font_used`).
    pub fn record(&self) -> Value {
        let mut v = json!({"family": self.family, "style": self.style.name(), "weight": self.weight, "sha256": self.file.sha256});
        if let Some((_, src)) = gfonts::source_of_sha(&self.file.sha256) {
            v["source"] = json!(src);
        }
        v
    }

    /// Record plus identification of the face inside its file (and the file itself).
    pub fn describe(&self, with_file: bool) -> Value {
        let mut v = self.record();
        v["postscript"] = json!(self.postscript);
        if self.stretch != 5 {
            v["stretch"] = json!(STRETCH_NAMES[self.stretch as usize - 1]);
        }
        if self.index > 0 {
            v["index"] = json!(self.index);
        }
        if with_file {
            v["file"] = match &self.file.path {
                Some(p) => json!(p.display().to_string()),
                None => json!(EMBEDDED_KEY),
            };
            v["embedded"] = json!(self.embedded);
        }
        v
    }

    pub fn axis(&self, tag: &str) -> Option<(f32, f32)> {
        self.axes.iter().find(|(t, _, _)| t == tag).map(|(_, a, b)| (*a, *b))
    }
}

fn face_axes(db: &FontDb, id: fontdb::ID) -> Vec<(String, f32, f32)> {
    db.db
        .with_face_data(id, |data, index| {
            ttf_parser::Face::parse(data, index)
                .map(|f| {
                    f.variation_axes()
                        .into_iter()
                        .map(|a| (String::from_utf8_lossy(&a.tag.to_bytes()).to_string(), a.min_value, a.max_value))
                        .collect()
                })
                .unwrap_or_default()
        })
        .unwrap_or_default()
}

fn choice(db: &FontDb, f: &fontdb::FaceInfo) -> Result<FaceChoice> {
    Ok(FaceChoice {
        file: load_file(db, f.id)?,
        index: f.index,
        family: primary_family(f),
        postscript: f.post_script_name.clone(),
        weight: f.weight.0,
        stretch: f.stretch.to_number() as u8,
        style: FontStyle::from_db(f.style),
        embedded: is_embedded(f),
        axes: face_axes(db, f.id),
    })
}

fn primary_family(f: &fontdb::FaceInfo) -> String {
    f.families.first().map(|(n, _)| n.clone()).unwrap_or_else(|| f.post_script_name.clone())
}

/// Lowercase alphanumerics only: `Noto-Sans`, `noto sans`, and `NotoSans` compare equal.
pub fn norm(s: &str) -> String {
    s.chars().filter(|c| c.is_alphanumeric()).flat_map(char::to_lowercase).collect()
}

fn generic_list(family: &str) -> Option<&'static [&'static str]> {
    match norm(family).as_str() {
        "sansserif" | "sans" | "systemui" | "uisansserif" | "cursive" | "fantasy" => Some(GENERIC_SANS),
        "serif" | "uiserif" => Some(GENERIC_SERIF),
        "monospace" | "mono" | "uimonospace" => Some(GENERIC_MONO),
        _ => None,
    }
}

fn family_faces<'d>(db: &'d FontDb, family: &str) -> Vec<&'d fontdb::FaceInfo> {
    db.index.get(&norm(family)).map(|ids| ids.iter().filter_map(|id| db.db.face(*id)).collect()).unwrap_or_default()
}

/// Whether `family` is installed (any face).
pub fn has_family(family: &str) -> bool {
    !family_faces(&db(), family).is_empty()
}

// ---------------------------------------------------------------- CSS font matching

/// CSS font-weight matching order as a sortable key.
fn weight_key(desired: u16, w: u16) -> (u8, u16) {
    if w == desired {
        return (0, 0);
    }
    if (400..=500).contains(&desired) {
        if w > desired && w <= 500 {
            (1, w - desired)
        } else if w < desired {
            (2, desired - w)
        } else {
            (3, w - desired)
        }
    } else if desired < 400 {
        if w < desired {
            (1, desired - w)
        } else {
            (2, w - desired)
        }
    } else if w > desired {
        (1, w - desired)
    } else {
        (2, desired - w)
    }
}

/// CSS font-stretch matching order: at or below normal, narrower widths are tried first
/// (closest first), then wider; above normal, wider first.
fn stretch_key(desired: u8, s: u8) -> (u8, u8) {
    if s == desired {
        return (0, 0);
    }
    if desired <= 5 {
        if s < desired {
            (1, desired - s)
        } else {
            (2, s - desired)
        }
    } else if s > desired {
        (1, s - desired)
    } else {
        (2, desired - s)
    }
}

fn style_rank(desired: FontStyle, s: FontStyle) -> u8 {
    use FontStyle::*;
    match (desired, s) {
        (a, b) if a == b => 0,
        (Normal, Oblique) | (Italic, Oblique) | (Oblique, Italic) => 1,
        _ => 2,
    }
}

type AxisRange = Option<(f32, f32)>;

/// Variable `wght`/`wdth` ranges of a face (cached per face id).
fn var_ranges(db: &FontDb, f: &fontdb::FaceInfo) -> (AxisRange, AxisRange) {
    static CACHE: Mutex<BTreeMap<(String, u32), (AxisRange, AxisRange)>> = Mutex::new(BTreeMap::new());
    let key = (source_key(&f.source), f.index);
    if let Some(v) = CACHE.lock().unwrap_or_else(|e| e.into_inner()).get(&key) {
        return *v;
    }
    let axes = face_axes(db, f.id);
    let get = |t: &str| axes.iter().find(|(a, _, _)| a == t).map(|(_, lo, hi)| (*lo, *hi));
    let v = (get("wght"), get("wdth"));
    CACHE.lock().unwrap_or_else(|e| e.into_inner()).insert(key, v);
    v
}

/// The CSS font-matching choice among `cands`: width first, then style, then weight
/// (a variable face matches any value its axes cover).
fn pick<'d>(
    db: &FontDb,
    cands: &[&'d fontdb::FaceInfo],
    weight: u16,
    style: FontStyle,
    stretch: u8,
) -> Option<&'d fontdb::FaceInfo> {
    cands
        .iter()
        .min_by_key(|f| {
            let (wr, dr) = var_ranges(db, f);
            let pct = stretch_percent(stretch);
            let sk = match dr {
                Some((lo, hi)) if pct >= lo - 1e-3 && pct <= hi + 1e-3 => (0, 0),
                _ => stretch_key(stretch, f.stretch.to_number() as u8),
            };
            let wk = match wr {
                Some((lo, hi)) if weight as f32 >= lo - 1e-3 && weight as f32 <= hi + 1e-3 => (0, 0),
                _ => weight_key(weight, f.weight.0),
            };
            (sk, style_rank(style, FontStyle::from_db(f.style)), wk, !is_embedded(f), source_key(&f.source), f.index)
        })
        .copied()
}

// ---------------------------------------------------------------- requests and resolution

/// What to look for.
#[derive(Clone, Debug)]
pub struct Request<'a> {
    /// A family, a generic name (`sans-serif`), or a class (`@didone`).
    pub family: &'a str,
    /// `None`: 400, or the weight a class candidate is set in.
    pub weight: Option<u16>,
    pub style: FontStyle,
    /// Width in percent of normal; `None`: normal, or the class candidate's width.
    pub stretch: Option<f32>,
    /// Prefer the face in the file with this SHA-256 (the one recorded when the layer was set).
    pub sha256: Option<&'a str>,
    /// Prefer the face with this PostScript name (recorded with the layer).
    pub postscript: Option<&'a str>,
}

impl<'a> Request<'a> {
    pub fn family(family: &'a str) -> Request<'a> {
        Request { family, weight: None, style: FontStyle::Normal, stretch: None, sha256: None, postscript: None }
    }
}

/// How a request was satisfied.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum How {
    /// The named family.
    Family,
    /// A generic name (`sans-serif`) mapped to a system default or Tuffy.
    Generic,
    /// A class (`@didone`) resolved to an installed member, or to a stand-in.
    Class { member: bool },
    /// The family (or every member of the class) is missing; `by` says how the substitute
    /// was found: `class`, `related`, `metrics`, `generic`, or `fallback`.
    Substitute { by: &'static str },
}

impl How {
    pub fn name(self) -> &'static str {
        match self {
            How::Family => "family",
            How::Generic => "generic",
            How::Class { member: true } => "class",
            How::Class { member: false } => "class-stand-in",
            How::Substitute { .. } => "substitute",
        }
    }
}

/// A resolved request.
#[derive(Clone)]
pub struct Resolution {
    pub choice: FaceChoice,
    /// The weight, width (percent), and style the face is drawn at (variable axes, synthetic bold).
    pub weight: u16,
    pub stretch: f32,
    pub style: FontStyle,
    pub how: How,
    /// The class used (named with `@`, or the class of a missing family).
    pub class: Option<&'static FontClass>,
}

impl Resolution {
    pub fn found(&self) -> bool {
        !matches!(self.how, How::Substitute { .. })
    }

    /// What the result JSON reports about the resolution (`font.resolved`).
    pub fn describe(&self, requested: &str) -> Value {
        let mut v = json!({"requested": requested, "how": self.how.name()});
        if let Some(c) = self.class {
            v["class"] = json!(c.alias());
        }
        if let How::Substitute { by } = self.how {
            v["by"] = json!(by);
        }
        v
    }
}

const STYLE_WORDS: &[(&str, Option<u16>, Option<FontStyle>, Option<u8>)] = &[
    ("thin", Some(100), None, None),
    ("hairline", Some(100), None, None),
    ("extralight", Some(200), None, None),
    ("ultralight", Some(200), None, None),
    ("light", Some(300), None, None),
    ("regular", Some(400), None, None),
    ("normal", Some(400), None, None),
    ("book", Some(400), None, None),
    ("medium", Some(500), None, None),
    ("semibold", Some(600), None, None),
    ("demibold", Some(600), None, None),
    ("bold", Some(700), None, None),
    ("extrabold", Some(800), None, None),
    ("ultrabold", Some(800), None, None),
    ("black", Some(900), None, None),
    ("heavy", Some(900), None, None),
    ("italic", None, Some(FontStyle::Italic), None),
    ("oblique", None, Some(FontStyle::Oblique), None),
    ("ultracondensed", None, None, Some(1)),
    ("extracondensed", None, None, Some(2)),
    ("compressed", None, None, Some(2)),
    ("condensed", None, None, Some(3)),
    ("narrow", None, None, Some(3)),
    ("semicondensed", None, None, Some(4)),
    ("semiexpanded", None, None, Some(6)),
    ("expanded", None, None, Some(7)),
    ("extended", None, None, Some(7)),
];

type Suffix = (String, Option<u16>, Option<FontStyle>, Option<u8>);

/// `Inter Bold Italic` → (`Inter`, 700, italic); `Helvetica Neue Condensed Black` →
/// (`Helvetica Neue`, 900, -, condensed).
fn split_style_suffix(family: &str) -> Option<Suffix> {
    let mut words: Vec<&str> = family.split_whitespace().collect();
    let (mut w, mut s, mut st) = (None, None, None);
    let mut changed = false;
    while words.len() > 1 {
        let last = norm(words[words.len() - 1]);
        match STYLE_WORDS.iter().find(|(k, _, _, _)| *k == last) {
            Some((_, kw, ks, kst)) => {
                w = w.or(*kw);
                s = s.or(*ks);
                st = st.or(*kst);
                words.pop();
                changed = true;
            }
            None => break,
        }
    }
    changed.then(|| (words.join(" "), w, s, st))
}

fn make_resolution(
    db: &FontDb,
    f: &fontdb::FaceInfo,
    weight: u16,
    stretch: f32,
    style: FontStyle,
    how: How,
    class: Option<&'static FontClass>,
) -> Result<Resolution> {
    Ok(Resolution { choice: choice(db, f)?, weight, stretch, style, how, class })
}

/// The best face of family `family` for weight/style/stretch, honoring a recorded
/// PostScript name and file hash (so saved documents keep the face they were set with).
fn choose_in<'d>(
    db: &'d FontDb,
    faces: &[&'d fontdb::FaceInfo],
    req: &Request,
    weight: u16,
    style: FontStyle,
    stretch: f32,
) -> Option<&'d fontdb::FaceInfo> {
    if let Some(ps) = req.postscript.filter(|p| !p.is_empty()) {
        let same: Vec<&fontdb::FaceInfo> = faces.iter().filter(|f| f.post_script_name == ps).copied().collect();
        if !same.is_empty() {
            if let Some(sha) = req.sha256 {
                if let Some(f) = same.iter().find(|f| file_sha(db, f).as_deref() == Some(sha)) {
                    return Some(f);
                }
            }
            return Some(same[0]);
        }
    }
    let kw = stretch_keyword(stretch);
    if let Some(sha) = req.sha256 {
        let same: Vec<&fontdb::FaceInfo> =
            faces.iter().filter(|f| file_sha(db, f).as_deref() == Some(sha)).copied().collect();
        if let Some(f) = pick(db, &same, weight, style, kw) {
            return Some(f);
        }
    }
    pick(db, faces, weight, style, kw)
}

/// A candidate's face when its family is installed and the face really has the candidate's
/// width (a variable width axis counts).
fn cand_face<'d>(db: &'d FontDb, c: &Cand, req: &Request) -> Option<(&'d fontdb::FaceInfo, u16, f32, FontStyle)> {
    let faces = family_faces(db, c.family);
    if faces.is_empty() {
        return None;
    }
    let weight = req.weight.unwrap_or(c.weight);
    let stretch = req.stretch.unwrap_or_else(|| stretch_percent(c.stretch));
    let style = if c.italic && req.style == FontStyle::Normal { FontStyle::Italic } else { req.style };
    let f = pick(db, &faces, weight, style, stretch_keyword(stretch))?;
    let (_, dr) = var_ranges(db, f);
    let width_ok = dr.is_some_and(|(lo, hi)| stretch >= lo - 1e-3 && stretch <= hi + 1e-3)
        || (f.stretch.to_number() as i32 - stretch_keyword(stretch) as i32).abs() <= 1;
    width_ok.then_some((f, weight, stretch, style))
}

/// The first installed member (or, with `standins`, stand-in) of a class.
fn resolve_in_class(db: &FontDb, cls: &'static FontClass, req: &Request, standins: bool) -> Result<Option<Resolution>> {
    for c in cls.candidates().filter(|c| c.member != standins) {
        if let Some((f, w, s, st)) = cand_face(db, c, req) {
            return make_resolution(db, f, w, s, st, How::Class { member: !standins }, Some(cls)).map(Some);
        }
    }
    Ok(None)
}

/// Measured metrics of a face at a weight/width (cached).
fn face_metrics(db: &FontDb, f: &fontdb::FaceInfo, weight: u16, stretch: f32) -> Option<metrics::Metrics> {
    type Key = (String, u32, u16, i32);
    static CACHE: Mutex<BTreeMap<Key, Option<metrics::Metrics>>> = Mutex::new(BTreeMap::new());
    let key = (source_key(&f.source), f.index, weight, (stretch * 10.0) as i32);
    if let Some(v) = CACHE.lock().unwrap_or_else(|e| e.into_inner()).get(&key) {
        return *v;
    }
    let vars = metrics::request_vars(weight, stretch);
    let m = db.db.with_face_data(f.id, |data, index| metrics::measure(data, index, &vars)).flatten();
    CACHE.lock().unwrap_or_else(|e| e.into_inner()).insert(key, m);
    m
}

/// Substitute for a class none of whose candidates is installed: related classes, then the
/// installed class candidate (of the same genre first) with the closest measured metrics,
/// then the genre's generic family, then Tuffy.
fn substitute_for_class(db: &FontDb, cls: &'static FontClass, req: &Request) -> Result<Resolution> {
    for r in cls.related.iter().filter_map(|r| classes::find(r)) {
        for standins in [false, true] {
            if let Some(mut res) = resolve_in_class(db, r, req, standins)? {
                res.how = How::Substitute { by: "related" };
                res.class = Some(cls);
                return Ok(res);
            }
        }
    }
    for same_genre in [true, false] {
        let mut best: Option<(f32, usize, &fontdb::FaceInfo, u16, f32, FontStyle)> = None;
        let mut order = 0usize;
        let mut seen = BTreeSet::new();
        for other in CLASSES.iter().filter(|o| (o.genre == cls.genre) == same_genre) {
            for c in other.candidates() {
                order += 1;
                if !seen.insert(norm(c.family)) {
                    continue;
                }
                let Some((f, w, s, st)) = cand_face(db, c, &Request { stretch: None, ..req.clone() }) else {
                    continue;
                };
                let Some(m) = face_metrics(db, f, w, s) else { continue };
                let d = m.distance(cls.target);
                if best.as_ref().is_none_or(|b| d < b.0 - 1e-6) {
                    best = Some((d, order, f, w, s, st));
                }
            }
        }
        if let Some((_, _, f, w, s, st)) = best {
            return make_resolution(db, f, w, s, st, How::Substitute { by: "metrics" }, Some(cls));
        }
    }
    let weight = req.weight.unwrap_or(400);
    let stretch = req.stretch.unwrap_or(100.0);
    if let Some(list) = generic_list(cls.genre.generic()) {
        for fam in list {
            if let Some(f) = pick(db, &family_faces(db, fam), weight, req.style, stretch_keyword(stretch)) {
                return make_resolution(
                    db,
                    f,
                    weight,
                    stretch,
                    req.style,
                    How::Substitute { by: "generic" },
                    Some(cls),
                );
            }
        }
    }
    let f = pick(db, &family_faces(db, FALLBACK_FAMILY), weight, req.style, 5)
        .ok_or_else(|| Error::internal("the embedded fallback font is missing"))?;
    make_resolution(db, f, weight, stretch, req.style, How::Substitute { by: "fallback" }, Some(cls))
}

/// The class a missing family is replaced from: the class listing it with the closest
/// weight and width, or one guessed from its name.
fn class_for_missing(family: &str, weight: Option<u16>, stretch: Option<f32>) -> &'static FontClass {
    let n = norm(family);
    let (w, s) = (weight.unwrap_or(400) as f32, stretch_keyword(stretch.unwrap_or(100.0)) as f32);
    let mut best: Option<(f32, &'static FontClass)> = None;
    for (rank, cls) in CLASSES.iter().enumerate() {
        for c in cls.candidates().filter(|c| norm(c.family) == n) {
            let d = (c.weight as f32 - w).abs() / 100.0
                + 3.0 * (c.stretch as f32 - s).abs()
                + if c.member { 0.0 } else { 0.5 }
                + rank as f32 * 1e-4;
            if best.is_none_or(|b| d < b.0) {
                best = Some((d, cls));
            }
        }
    }
    best.map(|b| b.1).unwrap_or_else(|| classes::guess_class(family))
}

/// Errors of strict mode.
fn strict_missing(req: &Request) -> Error {
    if let Some(f) = gfonts::curated(req.family) {
        return gfonts::consent_error(&f.family, Some(f));
    }
    let db = db();
    missing_family(req.family, close_families(&db, req.family))
}

pub fn unknown_class(name: &str) -> Error {
    Error::new("PX4110", format!("'{name}' is not a font class"))
        .with_hint(
            "list the classes with `pixelox fonts classes`; e.g. --font @condensed-heavy, @didone, @script-formal",
        )
        .with_context("class", name)
        .with_context("classes", CLASSES.iter().map(|c| c.alias()).collect::<Vec<_>>())
}

/// Resolve a request. A class resolves to its best installed member, then stand-in; a
/// missing family (or class with nothing installed) is replaced by the closest installed
/// face of its class, unless `strict` (then PX4102, or PX1540 when it can be downloaded).
pub fn resolve(req: &Request, strict: bool) -> Result<Resolution> {
    let db = db();
    let weight = req.weight.unwrap_or(400);
    let stretch = req.stretch.unwrap_or(100.0);
    if classes::is_class_ref(req.family) {
        let cls = classes::find(req.family).ok_or_else(|| unknown_class(req.family.trim()))?;
        if let Some(r) = resolve_in_class(&db, cls, req, false)? {
            return Ok(r);
        }
        if strict {
            let g = cls.google.iter().find(|g| g.member).map(|g| g.family).unwrap_or(cls.google[0].family);
            return Err(gfonts::consent_error(&cls.alias(), gfonts::curated(g))
                .with_context("class", cls.alias())
                .with_context("downloads", g));
        }
        if let Some(r) = resolve_in_class(&db, cls, req, true)? {
            return Ok(r);
        }
        return substitute_for_class(&db, cls, req);
    }
    if let Some(list) = generic_list(req.family) {
        for fam in list {
            if let Some(f) = pick(&db, &family_faces(&db, fam), weight, req.style, stretch_keyword(stretch)) {
                return make_resolution(&db, f, weight, stretch, req.style, How::Generic, None);
            }
        }
        let f = pick(&db, &family_faces(&db, FALLBACK_FAMILY), weight, req.style, 5)
            .ok_or_else(|| Error::internal("the embedded fallback font is missing"))?;
        return make_resolution(&db, f, weight, stretch, req.style, How::Generic, None);
    }
    let mut faces = family_faces(&db, req.family);
    let (mut w, mut st, mut s) = (weight, req.style, stretch);
    if faces.is_empty() {
        let n = norm(req.family);
        let by_ps = db
            .db
            .faces()
            .filter(|f| !n.is_empty() && norm(&f.post_script_name) == n)
            .min_by_key(|f| (source_key(&f.source), f.index));
        if let Some(f) = by_ps {
            return make_resolution(
                &db,
                f,
                f.weight.0,
                stretch_percent(f.stretch.to_number() as u8),
                FontStyle::from_db(f.style),
                How::Family,
                None,
            );
        }
        if let Some((fam, sw, ss, sst)) = split_style_suffix(req.family) {
            faces = family_faces(&db, &fam);
            w = req.weight.or(sw).unwrap_or(400);
            st = ss.unwrap_or(st);
            s = req.stretch.or(sst.map(stretch_percent)).unwrap_or(100.0);
        }
    }
    if !faces.is_empty() {
        let f = choose_in(&db, &faces, req, w, st, s).ok_or_else(|| Error::internal("no candidate face"))?;
        return make_resolution(&db, f, w, s, st, How::Family, None);
    }
    if strict {
        return Err(strict_missing(req));
    }
    // the substitute is drawn at its class's own weight unless one was asked for
    let cls = class_for_missing(req.family, req.weight, req.stretch);
    for standins in [false, true] {
        if let Some(mut r) = resolve_in_class(&db, cls, req, standins)? {
            r.how = How::Substitute { by: "class" };
            return Ok(r);
        }
    }
    let mut r = substitute_for_class(&db, cls, req)?;
    if matches!(r.how, How::Substitute { by: "related" }) {
        r.how = How::Substitute { by: "class" };
    }
    Ok(r)
}

/// Like [`resolve`] in strict mode: the family itself or an error (PX4102, PX1540, PX4110).
pub fn resolve_strict(req: &Request) -> Result<FaceChoice> {
    resolve(req, true).map(|r| r.choice)
}

pub fn missing_family(family: &str, cands: Vec<String>) -> Error {
    let msg = if cands.is_empty() {
        format!("font family '{family}' is not installed")
    } else {
        format!(
            "font family '{family}' is not installed; close matches: {}",
            cands.iter().take(5).cloned().collect::<Vec<_>>().join(", ")
        )
    };
    Error::new("PX4102", msg)
        .with_hint(match cands.first() {
            Some(c) => format!("use --font '{c}', a class such as --font @condensed-heavy (`pixelox fonts classes`), or register a folder with `pixelox fonts add <dir>`; without PIXELOX_STRICT_FONTS / --strict-font the closest installed face is used"),
            None => format!("use a class such as --font @neo-grotesque (`pixelox fonts classes`), try {}, or register a folder with `pixelox fonts add <dir>`", gfonts::get_command(family)),
        })
        .with_context("family", family)
        .with_context("candidates", cands)
        .with_context("fallback", FALLBACK_FAMILY)
}

/// The embedded fallback (or an installed Tuffy face matching better).
pub fn fallback_face(db: &FontDb, weight: u16, style: FontStyle) -> Result<FaceChoice> {
    let f = pick(db, &family_faces(db, FALLBACK_FAMILY), weight, style, 5)
        .ok_or_else(|| Error::internal("the embedded fallback font is missing"))?;
    choice(db, f)
}

/// Families of the glyph-fallback chain present here: Tuffy first, then system families.
pub fn fallback_chain(db: &FontDb) -> Vec<String> {
    let mut out = vec![FALLBACK_FAMILY.to_string()];
    for fam in FALLBACK_CHAIN {
        if !family_faces(db, fam).is_empty() {
            out.push(fam.to_string());
        }
    }
    out
}

/// The best face of each glyph-fallback family present, in chain order.
pub fn chain_ids(db: &FontDb, weight: u16, style: FontStyle) -> Vec<fontdb::ID> {
    std::iter::once(FALLBACK_FAMILY)
        .chain(FALLBACK_CHAIN.iter().copied())
        .filter_map(|fam| pick(db, &family_faces(db, fam), weight, style, 5).map(|f| f.id))
        .collect()
}

/// Whether face `id` maps `c` to a glyph (read through the memory map; nothing is copied).
pub fn face_covers(db: &FontDb, id: fontdb::ID, c: char) -> bool {
    db.db
        .with_face_data(id, |data, index| {
            ttf_parser::Face::parse(data, index).map(|f| f.glyph_index(c).is_some()).unwrap_or(false)
        })
        .unwrap_or(false)
}

pub fn choice_by_id(db: &FontDb, id: fontdb::ID) -> Result<FaceChoice> {
    let f = db.db.face(id).ok_or_else(|| Error::internal("font face vanished from the database"))?;
    choice(db, f)
}

/// The weights (and widths) a family offers, for warnings: `["275", "400", "900 condensed"]`.
pub fn family_weights(family: &str) -> Vec<String> {
    let db = db();
    let mut v: Vec<(u8, u16, String)> = family_faces(&db, family)
        .iter()
        .filter(|f| f.style == fontdb::Style::Normal)
        .map(|f| {
            let s = f.stretch.to_number() as u8;
            let (wr, _) = var_ranges(&db, f);
            let label = match wr {
                Some((lo, hi)) => format!("{lo}-{hi}"),
                None => f.weight.0.to_string(),
            };
            let label = if s == 5 { label } else { format!("{label} {}", STRETCH_NAMES[s as usize - 1]) };
            (s, f.weight.0, label)
        })
        .collect();
    v.sort();
    v.dedup_by(|a, b| a.2 == b.2);
    v.into_iter().map(|x| x.2).collect()
}

fn levenshtein(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    for (i, ca) in a.iter().enumerate() {
        let mut cur = vec![i + 1; b.len() + 1];
        for (j, cb) in b.iter().enumerate() {
            cur[j + 1] = (prev[j] + usize::from(ca != cb)).min(prev[j + 1] + 1).min(cur[j] + 1);
        }
        prev = cur;
    }
    prev[b.len()]
}

fn all_families(db: &FontDb) -> BTreeSet<String> {
    db.db.faces().map(primary_family).collect()
}

/// Installed families close to `family` (substring or small edit distance), best first.
pub fn close_families(db: &FontDb, family: &str) -> Vec<String> {
    let n = norm(family);
    let mut scored: Vec<(usize, String, String)> = all_families(db)
        .into_iter()
        .filter_map(|name| {
            let m = norm(&name);
            if m.is_empty() || n.is_empty() {
                return None;
            }
            let contains = (m.len() >= 3 && n.contains(&m)) || (n.len() >= 3 && m.contains(&n));
            let d = levenshtein(&n, &m);
            if contains {
                Some((0, m, name))
            } else if d <= (n.len().max(m.len()) / 3).max(1) {
                Some((d, m, name))
            } else {
                None
            }
        })
        .collect();
    scored.sort();
    scored.into_iter().map(|(_, _, name)| name).take(8).collect()
}

/// What each generic family resolves to here.
fn generic_report(db: &FontDb) -> Value {
    let mut m = Map::new();
    for (name, list) in [("sans-serif", GENERIC_SANS), ("serif", GENERIC_SERIF), ("monospace", GENERIC_MONO)] {
        let found = list.iter().find(|f| !family_faces(db, f).is_empty()).copied().unwrap_or(FALLBACK_FAMILY);
        m.insert(name.to_string(), json!(found));
    }
    Value::Object(m)
}

/// The class (`@id`) a family belongs to at its regular weight and width, if it is listed.
pub fn family_class(family: &str) -> Option<String> {
    classes::class_of_family(family)?;
    Some(class_for_missing(family, None, None).alias())
}

/// Every class that lists a family.
pub fn family_classes(family: &str) -> Vec<String> {
    let n = norm(family);
    CLASSES.iter().filter(|c| c.candidates().any(|k| norm(k.family) == n)).map(|c| c.alias()).collect()
}

// ---------------------------------------------------------------- commands

fn pad(s: &str, n: usize) -> String {
    let len = s.chars().count();
    if len >= n {
        format!("{s} ")
    } else {
        format!("{s}{}", " ".repeat(n - len))
    }
}

pub(crate) fn fonts_list(a: &Args, _ctx: &mut OpCtx) -> Result<Value> {
    let db = db();
    let filter = a.str("family").map(|f| norm(&f)).filter(|f| !f.is_empty());
    let class_filter = match a.str("class") {
        Some(c) => Some(classes::find(&c).ok_or_else(|| unknown_class(&c))?),
        None => None,
    };
    let google_root = gfonts::cache_root();
    let mut fams: BTreeMap<String, (String, Vec<&fontdb::FaceInfo>)> = BTreeMap::new();
    for f in db.db.faces() {
        let fam = primary_family(f);
        if let Some(q) = &filter {
            if !f.families.iter().any(|(n, _)| norm(n).contains(q.as_str())) {
                continue;
            }
        }
        if let Some(c) = class_filter {
            if !c.candidates().any(|k| norm(k.family) == norm(&fam)) {
                continue;
            }
        }
        fams.entry(fam.to_lowercase()).or_insert_with(|| (fam.clone(), Vec::new())).1.push(f);
    }
    let mut faces_total = 0;
    let mut text = String::new();
    let families: Vec<Value> = fams
        .into_values()
        .map(|(name, mut faces)| {
            faces.sort_by_key(|f| (FontStyle::from_db(f.style), f.weight.0, f.post_script_name.clone(), f.index));
            faces_total += faces.len();
            let styles: BTreeSet<&str> = faces.iter().map(|f| FontStyle::from_db(f.style).name()).collect();
            let class = match class_filter {
                Some(c) => Some(c.alias()),
                None => family_class(&name),
            };
            let google = faces.iter().any(|f| face_path(&f.source).is_some_and(|p| p.starts_with(&google_root)));
            let weights: BTreeSet<u16> = faces.iter().map(|f| f.weight.0).collect();
            text.push_str(&format!(
                "{}{}{}{}\n",
                pad(&name, 34),
                pad(class.as_deref().unwrap_or("-"), 22),
                pad(&weights.iter().map(|w| w.to_string()).collect::<Vec<_>>().join(","), 28),
                if google { "google-fonts" } else { "" }
            ));
            let mut v = json!({
                "family": name,
                "class": class,
                "classes": family_classes(&name),
                "styles": styles,
                "faces": faces.iter().map(|f| {
                    let mut v = json!({
                        "style": FontStyle::from_db(f.style).name(),
                        "weight": f.weight.0,
                        "stretch": f.stretch.to_number(),
                        "postscript": f.post_script_name,
                        "file": face_path(&f.source).map(|p| p.display().to_string()).unwrap_or_else(|| EMBEDDED_KEY.to_string()),
                        "embedded": is_embedded(f),
                    });
                    if f.index > 0 {
                        v["index"] = json!(f.index);
                    }
                    if f.monospaced {
                        v["monospaced"] = json!(true);
                    }
                    v
                }).collect::<Vec<_>>(),
            });
            if google {
                v["source"] = json!("google-fonts");
            }
            v
        })
        .collect();
    let tuffy = fallback_face(&db, 400, FontStyle::Normal)?;
    let header = format!("{}{}{}source\n", pad("family", 34), pad("class", 22), pad("weights", 28));
    let mut out = json!({
        "count": families.len(),
        "faces": faces_total,
        "families": families,
        "registered_dirs": db.dirs.iter().map(|d| json!({"dir": d.display().to_string(), "exists": d.is_dir()})).collect::<Vec<_>>(),
        "fonts_file": paths::fonts_file().display().to_string(),
        "system_fonts": db.system,
        "fallback": {"family": FALLBACK_FAMILY, "embedded": true, "license": "public domain", "sha256": tuffy.file.sha256},
        "fallback_chain": fallback_chain(&db),
        "generic": generic_report(&db),
        "google_fonts": {
            "cache": google_root.display().to_string(),
            "installed": gfonts::installed_all().iter().map(|r| r.family.clone()).collect::<Vec<_>>(),
            "curated": gfonts::registry().families.len(),
            "commit": gfonts::registry().commit,
        },
        "text": format!("{header}{text}{} families, {} faces", out_count(&text), faces_total),
    });
    if let (Some(q), 0) = (a.str("family"), out["count"].as_u64().unwrap_or(0)) {
        out["candidates"] = json!(close_families(&db, &q));
        if let Some(f) = gfonts::curated(&q) {
            out["download"] = json!(gfonts::get_command(&f.family));
        }
    }
    Ok(out)
}

fn out_count(text: &str) -> usize {
    text.lines().count()
}

pub(crate) fn fonts_add(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let raw = a.path("dir").ok_or_else(|| {
        Error::new("PX1003", "'fonts add' needs a folder: pixelox fonts add <dir>").with_context("param", "dir")
    })?;
    let dir = absolutize(&expand_tilde(&raw));
    ctx.check_allowed(&dir)?;
    if !dir.exists() {
        return Err(Error::input_missing(&dir));
    }
    if !dir.is_dir() {
        return Err(Error::usage(format!("{} is not a folder", dir.display()))
            .with_hint("pass the folder that holds the font files, e.g. pixelox fonts add ~/fonts")
            .with_context("dir", dir.display().to_string()));
    }
    let mut scan = fontdb::Database::new();
    scan.load_fonts_dir(&dir);
    let families: BTreeSet<String> = scan.faces().map(primary_family).collect();
    let faces = scan.len();
    let file = paths::fonts_file();
    let mut dirs = registered_dirs();
    let already = dirs.contains(&dir);
    let mut added = false;
    if !already {
        dirs.push(dir.clone());
        if !ctx.dry_run() {
            write_dirs(&file, &dirs)?;
            reload();
            added = true;
        } else {
            ctx.plan_add("writes", json!(file.display().to_string()));
        }
    }
    Ok(json!({
        "dir": dir.display().to_string(),
        "faces": faces,
        "families": families,
        "added": added,
        "already_registered": already,
        "fonts_file": file.display().to_string(),
        "registered_dirs": dirs.iter().map(|d| d.display().to_string()).collect::<Vec<_>>(),
    }))
}

/// What a class resolves to here (for `fonts classes`).
fn class_report(cls: &'static FontClass) -> Result<Value> {
    let req = Request::family("");
    let db = db();
    let res = match resolve_in_class(&db, cls, &req, false)? {
        Some(r) => r,
        None => match resolve_in_class(&db, cls, &req, true)? {
            Some(r) => r,
            None => substitute_for_class(&db, cls, &req)?,
        },
    };
    let installed: Vec<&str> = cls
        .candidates()
        .filter(|c| !family_faces(&db, c.family).is_empty())
        .map(|c| c.family)
        .fold(Vec::new(), |mut v, f| {
            if !v.contains(&f) {
                v.push(f);
            }
            v
        });
    let list = |cs: &[Cand]| -> Vec<Value> {
        cs.iter()
            .map(|c| {
                let mut v = json!({"family": c.family, "weight": c.weight});
                if c.stretch != 5 {
                    v["stretch"] = json!(STRETCH_NAMES[c.stretch as usize - 1]);
                }
                if c.italic {
                    v["style"] = json!("italic");
                }
                if !c.member {
                    v["stand_in"] = json!(true);
                }
                v
            })
            .collect()
    };
    let status = match res.how {
        How::Class { member: true } => "member",
        How::Class { member: false } => "stand-in",
        _ => "substitute",
    };
    let first_google = cls.google.iter().find(|g| g.member).map(|g| g.family).unwrap_or(cls.google[0].family);
    let mut v = json!({
        "class": cls.alias(),
        "title": cls.title,
        "uses": cls.uses,
        "genre": cls.genre.name(),
        "resolves_to": {
            "family": res.choice.family,
            "postscript": res.choice.postscript,
            "weight": res.weight,
            "stretch": stretch_name(res.stretch),
            "status": status,
        },
        "installed": installed,
        "candidates": {
            "macos": list(cls.macos),
            "windows": list(cls.windows),
            "linux": list(cls.linux),
            "google": list(cls.google),
        },
        "related": cls.related.iter().map(|r| format!("@{r}")).collect::<Vec<_>>(),
    });
    if status != "member" {
        v["get"] = json!(gfonts::get_command(&cls.alias()));
        v["get_family"] = json!(first_google);
    }
    Ok(v)
}

pub(crate) fn fonts_classes(a: &Args, _ctx: &mut OpCtx) -> Result<Value> {
    let only = match a.str("class") {
        Some(c) => Some(classes::find(&c).ok_or_else(|| unknown_class(&c))?),
        None => None,
    };
    let mut rows = Vec::new();
    let mut text = format!("{}{}{}\n", pad("class", 22), pad("resolves to", 40), "status");
    for cls in CLASSES.iter().filter(|c| only.is_none_or(|o| o.id == c.id)) {
        let r = class_report(cls)?;
        let target = format!(
            "{} {}",
            r["resolves_to"]["family"].as_str().unwrap_or(""),
            r["resolves_to"]["weight"].as_u64().unwrap_or(0)
        );
        let status = r["resolves_to"]["status"].as_str().unwrap_or("").to_string();
        let tail = match r.get("get").and_then(|g| g.as_str()) {
            Some(g) => format!("{status}  (better: {g})"),
            None => status,
        };
        text.push_str(&format!("{}{}{}\n", pad(&cls.alias(), 22), pad(&target, 40), tail));
        rows.push(r);
    }
    let members = rows.iter().filter(|r| r["resolves_to"]["status"] == "member").count();
    Ok(json!({
        "count": rows.len(),
        "with_member": members,
        "classes": rows,
        "text": text,
    }))
}

pub(crate) fn fonts_get(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let consent = ctx.globals.yes;
    let mut names: Vec<String> = Vec::new();
    if a.flag("all") {
        names.extend(gfonts::registry().families.iter().map(|f| f.family.clone()));
    }
    for n in a.all("family") {
        let n = n.trim().to_string();
        if n.is_empty() {
            continue;
        }
        if classes::is_class_ref(&n) {
            let cls = classes::find(&n).ok_or_else(|| unknown_class(&n))?;
            if a.flag("class-all") {
                names.extend(cls.google.iter().map(|g| g.family.to_string()));
            } else {
                let g = cls.google.iter().find(|g| g.member).unwrap_or(&cls.google[0]);
                names.push(g.family.to_string());
            }
        } else {
            names.push(n);
        }
    }
    if names.is_empty() {
        return Err(Error::new("PX1003", "'fonts get' needs a family, a class (@didone), or --all")
            .with_hint("pixelox fonts get \"Bebas Neue\" -y   (or: pixelox fonts get @stencil -y)"));
    }
    let mut seen = BTreeSet::new();
    names.retain(|n| seen.insert(norm(n)));
    if !consent && !ctx.dry_run() {
        let need: Vec<&String> = names
            .iter()
            .filter(|n| gfonts::curated(n).is_none_or(|f| gfonts::installed(&f.family).is_none()))
            .collect();
        if let Some(first) = need.first() {
            let what = if need.len() == 1 { first.to_string() } else { format!("{} families", need.len()) };
            let mut e = gfonts::consent_error(first, gfonts::curated(first));
            e.hint =
                Some(format!("{}   (ask the user first: it downloads from Google Fonts)", gfonts::get_command(first)));
            if need.len() > 1 {
                e.message = format!("{what} are not installed; they can be downloaded from Google Fonts");
                let bytes: u64 = need.iter().filter_map(|n| gfonts::curated(n)).map(|f| f.bytes()).sum();
                e = e
                    .with_context("families", need.iter().map(|s| s.to_string()).collect::<Vec<_>>())
                    .with_context("bytes", bytes);
                let args = if a.flag("all") { "--all".to_string() } else { a.all("family").join(" ") };
                e.hint = Some(format!("pixelox fonts get {args} -y"));
            }
            return Err(e);
        }
    }
    let mut out = Vec::new();
    for n in &names {
        let r = gfonts::get(ctx, n, consent)?;
        out.push(r.to_json());
    }
    let downloaded = out.iter().filter(|r| r["status"] == "downloaded").count();
    let bytes: u64 = out.iter().map(|r| r["fetched_bytes"].as_u64().unwrap_or(0)).sum();
    Ok(json!({
        "count": out.len(),
        "downloaded": downloaded,
        "fetched_bytes": bytes,
        "families": out,
        "cache": gfonts::cache_root().display().to_string(),
    }))
}

fn expand_tilde(p: &Path) -> PathBuf {
    if let Ok(rest) = p.strip_prefix("~") {
        if let Some(home) = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")) {
            return PathBuf::from(home).join(rest);
        }
    }
    p.to_path_buf()
}

fn write_dirs(file: &Path, dirs: &[PathBuf]) -> Result<()> {
    if let Some(parent) = file.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut text = String::from("# Font folders registered with `pixelox fonts add` (one per line).\n");
    for d in dirs {
        text.push_str(&d.display().to_string());
        text.push('\n');
    }
    let tmp = pixelox_core::ctx::tmp_path(file);
    std::fs::write(&tmp, text)?;
    std::fs::rename(&tmp, file)?;
    Ok(())
}

/// Fonts a recipe recorded (`steps[].fonts`) that this machine does not have: the family is
/// missing (`missing`), or installed from a different file (`different-file`, the output may
/// differ slightly). Each entry names the command that gets the font.
pub fn missing_recorded(steps: &[pixelox_core::doc::RecipeStep]) -> Vec<Value> {
    let db = db();
    let mut out: Vec<Value> = Vec::new();
    let mut seen = BTreeSet::new();
    for (i, s) in steps.iter().enumerate() {
        for f in &s.fonts {
            let family = f.get("family").and_then(|v| v.as_str()).unwrap_or("");
            let sha = f.get("sha256").and_then(|v| v.as_str()).unwrap_or("");
            if family.is_empty() || !seen.insert((family.to_string(), sha.to_string())) {
                continue;
            }
            let faces = family_faces(&db, family);
            let status = if faces.is_empty() {
                "missing"
            } else if sha.is_empty() || faces.iter().any(|x| file_sha(&db, x).as_deref() == Some(sha)) {
                continue;
            } else {
                "different-file"
            };
            let google = gfonts::source_of_sha(sha)
                .map(|(f, _)| f)
                .or_else(|| gfonts::curated(family).map(|f| f.family.clone()));
            let get = match &google {
                Some(g) => gfonts::get_command(g),
                None => "pixelox fonts add <folder with the font files>".to_string(),
            };
            out.push(json!({
                "step": i + 1,
                "op": s.op,
                "family": family,
                "weight": f.get("weight"),
                "style": f.get("style"),
                "sha256": sha,
                "status": status,
                "source": f.get("source"),
                "get": get,
            }));
        }
    }
    out
}

/// Genre of a class id (for docs and tests).
pub fn genre_of(id: &str) -> Option<Genre> {
    classes::find(id).map(|c| c.genre)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weight_matching_follows_css() {
        // 400 prefers 500 before lighter, then heavier
        let mut ws = [300u16, 500, 700];
        ws.sort_by_key(|w| weight_key(400, *w));
        assert_eq!(ws, [500, 300, 700]);
        let mut ws = [300u16, 500, 900];
        ws.sort_by_key(|w| weight_key(700, *w));
        assert_eq!(ws, [900, 500, 300]);
        let mut ws = [200u16, 400, 500];
        ws.sort_by_key(|w| weight_key(300, *w));
        assert_eq!(ws, [200, 400, 500]);
    }

    #[test]
    fn stretch_matching_follows_css() {
        // normal: narrower first (closest), then wider
        let mut s = [3u8, 4, 6, 7];
        s.sort_by_key(|k| stretch_key(5, *k));
        assert_eq!(s, [4, 3, 6, 7]);
        // expanded: wider first
        let mut s = [3u8, 5, 8, 9];
        s.sort_by_key(|k| stretch_key(7, *k));
        assert_eq!(s, [8, 9, 5, 3]);
        assert_eq!(stretch_keyword(75.0), 3);
        assert_eq!(stretch_keyword(80.0), 3);
        assert_eq!(stretch_percent(7), 125.0);
        assert_eq!(parse_stretch("condensed").unwrap(), 75.0);
        assert_eq!(parse_stretch("Extended").unwrap(), 125.0);
        assert_eq!(parse_stretch("80%").unwrap(), 80.0);
        assert!(parse_stretch("squashed").is_err());
        assert_eq!(parse_stretch("300%").unwrap_err().code, "PX1013");
        assert_eq!(stretch_name(75.0), "condensed");
        assert_eq!(stretch_name(80.0), "80%");
    }

    #[test]
    fn names_normalize_and_split() {
        assert_eq!(norm("Noto-Sans CJK"), "notosanscjk");
        let (f, w, s, st) = split_style_suffix("Inter Bold Italic").unwrap();
        assert_eq!((f.as_str(), w, s, st), ("Inter", Some(700), Some(FontStyle::Italic), None));
        let (f, w, _, st) = split_style_suffix("Helvetica Neue Condensed Black").unwrap();
        assert_eq!((f.as_str(), w, st), ("Helvetica Neue", Some(900), Some(3)));
        assert!(split_style_suffix("Inter").is_none());
        assert_eq!(levenshtein("tuffy", "tufy"), 1);
    }

    #[test]
    fn embedded_fallback_is_always_there() {
        let db = db();
        let t = fallback_face(&db, 400, FontStyle::Normal).unwrap();
        assert_eq!(t.family, FALLBACK_FAMILY);
        assert_eq!(t.file.sha256, sha256_bytes(TUFFY));
        let c = resolve_strict(&Request::family("tuffy")).unwrap();
        assert_eq!(c.family, FALLBACK_FAMILY);
        assert_eq!(fallback_chain(&db)[0], FALLBACK_FAMILY);
    }

    #[test]
    fn missing_families_are_substituted_unless_strict() {
        let req = Request::family("No Such Family Condensed Black");
        let e = resolve(&req, true).err().expect("strict fails");
        assert_eq!(e.code, "PX4102");
        let r = resolve(&req, false).unwrap();
        assert!(!r.found());
        assert_eq!(r.class.unwrap().id, "condensed-heavy");
        // a curated Google family that is not installed: strict mode asks for consent
        let req = Request::family("Zzz Not Real");
        assert!(resolve(&req, false).is_ok());
        assert_eq!(resolve(&Request::family("@nope"), false).err().unwrap().code, "PX4110");
    }

    #[test]
    fn class_for_missing_family_follows_weight() {
        assert_eq!(class_for_missing("Oswald", None, None).id, "condensed");
        assert_eq!(class_for_missing("Oswald", Some(700), None).id, "condensed-heavy");
        assert_eq!(class_for_missing("Impact", None, None).id, "condensed-heavy");
        assert_eq!(class_for_missing("Helvetica Neue", Some(900), None).id, "neo-grotesque");
        assert_eq!(class_for_missing("Helvetica Neue", Some(900), Some(75.0)).id, "condensed-heavy");
        assert_eq!(class_for_missing("Great Vibes", None, None).id, "script-formal");
        assert_eq!(class_for_missing("Unknown Mono", None, None).id, "mono");
    }
}
