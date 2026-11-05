//! Font discovery and matching: fontdb over the system fonts, the folders registered with
//! `fonts add` (one per line in `paths::fonts_file()`), and the embedded Tuffy fallback;
//! case-insensitive family matching with CSS-style weight/style selection; a small cache of
//! font files with their SHA-256; and the `fonts list` / `fonts add` commands.

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

/// Forget the loaded database (after `fonts add`), so the next lookup rescans.
pub fn reload() {
    *DB.write().unwrap_or_else(|e| e.into_inner()) = None;
}

fn system_fonts_enabled() -> bool {
    !matches!(
        std::env::var("PIXELOX_SYSTEM_FONTS").map(|v| v.trim().to_ascii_lowercase()).as_deref(),
        Ok("0" | "off" | "false" | "no")
    )
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

/// A resolved face: which file, which face in it, and what it is.
#[derive(Clone)]
pub struct FaceChoice {
    pub file: Arc<FontFile>,
    pub index: u32,
    pub family: String,
    pub postscript: String,
    pub weight: u16,
    pub style: FontStyle,
    pub embedded: bool,
}

impl FaceChoice {
    /// The sidecar record (`ctx.font_used`).
    pub fn record(&self) -> Value {
        json!({"family": self.family, "style": self.style.name(), "weight": self.weight, "sha256": self.file.sha256})
    }

    /// Record plus identification of the face inside its file (and the file itself).
    pub fn describe(&self, with_file: bool) -> Value {
        let mut v = self.record();
        v["postscript"] = json!(self.postscript);
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
}

fn choice(db: &FontDb, f: &fontdb::FaceInfo) -> Result<FaceChoice> {
    Ok(FaceChoice {
        file: load_file(db, f.id)?,
        index: f.index,
        family: primary_family(f),
        postscript: f.post_script_name.clone(),
        weight: f.weight.0,
        style: FontStyle::from_db(f.style),
        embedded: is_embedded(f),
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

fn style_rank(desired: FontStyle, s: FontStyle) -> u8 {
    use FontStyle::*;
    match (desired, s) {
        (a, b) if a == b => 0,
        (Normal, Oblique) | (Italic, Oblique) | (Oblique, Italic) => 1,
        _ => 2,
    }
}

fn pick<'d>(cands: &[&'d fontdb::FaceInfo], weight: u16, style: FontStyle) -> Option<&'d fontdb::FaceInfo> {
    cands
        .iter()
        .min_by_key(|f| {
            (
                style_rank(style, FontStyle::from_db(f.style)),
                weight_key(weight, f.weight.0),
                (f.stretch.to_number() as i32 - 5).unsigned_abs(),
                !is_embedded(f),
                source_key(&f.source),
                f.index,
            )
        })
        .copied()
}

/// What to look for.
#[derive(Clone, Debug)]
pub struct Request<'a> {
    pub family: &'a str,
    pub weight: u16,
    pub style: FontStyle,
    /// Prefer the face in the file with this SHA-256 (the one recorded when the layer was set).
    pub sha256: Option<&'a str>,
}

pub enum Lookup {
    Found(FaceChoice),
    Missing(Vec<String>),
}

const STYLE_WORDS: &[(&str, Option<u16>, Option<FontStyle>)] = &[
    ("thin", Some(100), None),
    ("hairline", Some(100), None),
    ("extralight", Some(200), None),
    ("ultralight", Some(200), None),
    ("light", Some(300), None),
    ("regular", Some(400), None),
    ("normal", Some(400), None),
    ("book", Some(400), None),
    ("medium", Some(500), None),
    ("semibold", Some(600), None),
    ("demibold", Some(600), None),
    ("bold", Some(700), None),
    ("extrabold", Some(800), None),
    ("ultrabold", Some(800), None),
    ("black", Some(900), None),
    ("heavy", Some(900), None),
    ("italic", None, Some(FontStyle::Italic)),
    ("oblique", None, Some(FontStyle::Oblique)),
];

/// `Inter Bold Italic` → (`Inter`, 700, italic).
fn split_style_suffix(family: &str) -> Option<(String, Option<u16>, Option<FontStyle>)> {
    let mut words: Vec<&str> = family.split_whitespace().collect();
    let (mut w, mut s) = (None, None);
    let mut changed = false;
    while words.len() > 1 {
        let last = norm(words[words.len() - 1]);
        match STYLE_WORDS.iter().find(|(k, _, _)| *k == last) {
            Some((_, kw, ks)) => {
                w = w.or(*kw);
                s = s.or(*ks);
                words.pop();
                changed = true;
            }
            None => break,
        }
    }
    changed.then(|| (words.join(" "), w, s))
}

/// Find the face for a request (generic names map to a system default, then Tuffy).
pub fn lookup(req: &Request) -> Result<Lookup> {
    let db = db();
    if let Some(list) = generic_list(req.family) {
        for fam in list {
            if let Some(f) = pick(&family_faces(&db, fam), req.weight, req.style) {
                return Ok(Lookup::Found(choice(&db, f)?));
            }
        }
        return Ok(Lookup::Found(fallback_face(&db, req.weight, req.style)?));
    }
    let mut cands = family_faces(&db, req.family);
    let (mut weight, mut style) = (req.weight, req.style);
    if cands.is_empty() {
        let n = norm(req.family);
        let by_ps = db
            .db
            .faces()
            .filter(|f| !n.is_empty() && norm(&f.post_script_name) == n)
            .min_by_key(|f| (source_key(&f.source), f.index));
        if let Some(f) = by_ps {
            return Ok(Lookup::Found(choice(&db, f)?));
        }
        if let Some((fam, w, s)) = split_style_suffix(req.family) {
            cands = family_faces(&db, &fam);
            weight = w.unwrap_or(weight);
            style = s.unwrap_or(style);
        }
    }
    if cands.is_empty() {
        return Ok(Lookup::Missing(close_families(&db, req.family)));
    }
    if let Some(sha) = req.sha256 {
        let same: Vec<&fontdb::FaceInfo> =
            cands.iter().filter(|f| file_sha(&db, f).as_deref() == Some(sha)).copied().collect();
        if let Some(f) = pick(&same, weight, style) {
            return Ok(Lookup::Found(choice(&db, f)?));
        }
    }
    let f = pick(&cands, weight, style).ok_or_else(|| Error::internal("no candidate face"))?;
    Ok(Lookup::Found(choice(&db, f)?))
}

/// Like [`lookup`], failing with PX4102 (with close candidates) when the family is missing.
pub fn resolve_strict(req: &Request) -> Result<FaceChoice> {
    match lookup(req)? {
        Lookup::Found(c) => Ok(c),
        Lookup::Missing(cands) => Err(missing_family(req.family, cands)),
    }
}

/// Like [`lookup`], falling back to Tuffy; the flag says whether the family was found.
pub fn resolve_lenient(req: &Request) -> Result<(FaceChoice, bool)> {
    match lookup(req)? {
        Lookup::Found(c) => Ok((c, true)),
        Lookup::Missing(_) => Ok((fallback_face(&db(), req.weight, req.style)?, false)),
    }
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
            Some(c) => format!("use --font '{c}', list fonts with `pixelox fonts list`, or register a folder with `pixelox fonts add <dir>`"),
            None => "list fonts with `pixelox fonts list`, use a generic family (sans-serif, serif, monospace), or register a folder with `pixelox fonts add <dir>`".to_string(),
        })
        .with_context("family", family)
        .with_context("candidates", cands)
        .with_context("fallback", FALLBACK_FAMILY)
}

/// The embedded fallback (or an installed Tuffy face matching better).
pub fn fallback_face(db: &FontDb, weight: u16, style: FontStyle) -> Result<FaceChoice> {
    let f = pick(&family_faces(db, FALLBACK_FAMILY), weight, style)
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
        .filter_map(|fam| pick(&family_faces(db, fam), weight, style).map(|f| f.id))
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

// ---------------------------------------------------------------- commands

pub(crate) fn fonts_list(a: &Args, _ctx: &mut OpCtx) -> Result<Value> {
    let db = db();
    let filter = a.str("family").map(|f| norm(&f)).filter(|f| !f.is_empty());
    let mut fams: BTreeMap<String, (String, Vec<&fontdb::FaceInfo>)> = BTreeMap::new();
    for f in db.db.faces() {
        let fam = primary_family(f);
        if let Some(q) = &filter {
            if !f.families.iter().any(|(n, _)| norm(n).contains(q.as_str())) {
                continue;
            }
        }
        fams.entry(fam.to_lowercase()).or_insert_with(|| (fam.clone(), Vec::new())).1.push(f);
    }
    let mut faces_total = 0;
    let families: Vec<Value> = fams
        .into_values()
        .map(|(name, mut faces)| {
            faces.sort_by_key(|f| (FontStyle::from_db(f.style), f.weight.0, f.post_script_name.clone(), f.index));
            faces_total += faces.len();
            let styles: BTreeSet<&str> = faces.iter().map(|f| FontStyle::from_db(f.style).name()).collect();
            json!({
                "family": name,
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
            })
        })
        .collect();
    let tuffy = fallback_face(&db, 400, FontStyle::Normal)?;
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
    });
    if let (Some(q), 0) = (a.str("family"), out["count"].as_u64().unwrap_or(0)) {
        out["candidates"] = json!(close_families(&db, &q));
    }
    Ok(out)
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
    fn names_normalize_and_split() {
        assert_eq!(norm("Noto-Sans CJK"), "notosanscjk");
        let (f, w, s) = split_style_suffix("Inter Bold Italic").unwrap();
        assert_eq!((f.as_str(), w, s), ("Inter", Some(700), Some(FontStyle::Italic)));
        assert!(split_style_suffix("Inter").is_none());
        assert_eq!(levenshtein("tuffy", "tufy"), 1);
    }

    #[test]
    fn embedded_fallback_is_always_there() {
        let db = db();
        let t = fallback_face(&db, 400, FontStyle::Normal).unwrap();
        assert_eq!(t.family, FALLBACK_FAMILY);
        assert_eq!(t.file.sha256, sha256_bytes(TUFFY));
        let c =
            resolve_strict(&Request { family: "tuffy", weight: 400, style: FontStyle::Normal, sha256: None }).unwrap();
        assert_eq!(c.family, FALLBACK_FAMILY);
        assert_eq!(fallback_chain(&db)[0], FALLBACK_FAMILY);
    }
}
