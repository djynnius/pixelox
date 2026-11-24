//! The bundled JavaScript libraries (d3, three.js, topojson-client) and map data (world-atlas,
//! us-atlas), vendored under `vendor/` by `scripts/vendor-js.py` (ADR 0018).
//!
//! `build.rs` embeds every file of `vendor/manifest.tsv` deflate-compressed. A file is inflated
//! the first time a run needs it, checked against the manifest's SHA-256 (PX1553 when it does
//! not match), and kept for the rest of the process. Scripts reach the libraries through
//! `--with` (globals `d3`, `THREE`, `topojson`) and the atlases through `px.atlas(name)`, which
//! returns TopoJSON whose features carry codes for joining data (`iso_a2`/`iso_a3` on
//! countries, `postal`/`fips` on U.S. states).

use pixelox_core::envelope::sha256_bytes;
use pixelox_core::error::{Error, Result};
use serde_json::{json, Map, Value};
use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as _;
use std::sync::{Arc, Mutex, OnceLock};

include!(concat!(env!("OUT_DIR"), "/vendored.rs"));

const MANIFEST: &str = include_str!("../../vendor/manifest.tsv");

/// One vendored file (a row of `vendor/manifest.tsv`).
#[derive(Debug)]
pub struct FileInfo {
    pub path: &'static str,
    pub package: &'static str,
    pub version: &'static str,
    pub license: &'static str,
    pub bytes: usize,
    pub sha256: &'static str,
}

/// Every vendored file, in manifest order.
pub fn files() -> &'static [FileInfo] {
    static ROWS: OnceLock<Vec<FileInfo>> = OnceLock::new();
    ROWS.get_or_init(|| {
        MANIFEST
            .lines()
            .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
            .map(|l| {
                let c: Vec<&'static str> = l.split('\t').collect();
                FileInfo {
                    path: c[0],
                    package: c[1],
                    version: c[2],
                    license: c[3],
                    bytes: c[4].parse().unwrap_or(0),
                    sha256: c[5],
                }
            })
            .collect()
    })
}

pub fn info(path: &str) -> Option<&'static FileInfo> {
    files().iter().find(|f| f.path == path)
}

/// The text of a vendored file, inflated and checked once per process.
pub fn text(path: &str) -> Result<Arc<str>> {
    static CACHE: OnceLock<Mutex<HashMap<String, Arc<str>>>> = OnceLock::new();
    let cache = CACHE.get_or_init(Default::default);
    if let Some(t) = cache.lock().unwrap_or_else(|e| e.into_inner()).get(path) {
        return Ok(t.clone());
    }
    let damaged = |why: String| {
        Error::new("PX1553", format!("the bundled file {path} is damaged: {why}"))
            .with_hint("reinstall pixelox (`pixelox update`, or the installer); the binary does not match its manifest")
            .with_context("path", path)
    };
    let meta = info(path).ok_or_else(|| damaged("it is not in the manifest".into()))?;
    let packed =
        FILES.iter().find(|(p, _)| *p == path).map(|(_, b)| *b).ok_or_else(|| damaged("not embedded".into()))?;
    let raw = miniz_oxide::inflate::decompress_to_vec_with_limit(packed, meta.bytes + 1)
        .map_err(|e| damaged(format!("cannot inflate it ({e:?})")))?;
    let sha = sha256_bytes(&raw);
    if sha != meta.sha256 || raw.len() != meta.bytes {
        return Err(damaged(format!("SHA-256 {sha}, expected {}", meta.sha256)));
    }
    let t: Arc<str> = String::from_utf8(raw).map_err(|_| damaged("not UTF-8".into()))?.into();
    cache.lock().unwrap_or_else(|e| e.into_inner()).insert(path.to_string(), t.clone());
    Ok(t)
}

/// The provenance record of a vendored file (sidecar `bundled`, result `libraries`/`atlases`).
pub fn record(kind: &str, name: &str, f: &FileInfo) -> Value {
    json!({
        "kind": kind,
        "name": name,
        "package": f.package,
        "version": f.version,
        "file": f.path,
        "sha256": f.sha256,
        "bytes": f.bytes,
        "license": f.license,
    })
}

// ---------------------------------------------------------------------------------------
// libraries

/// A library `--with` can load.
#[derive(Debug)]
pub struct Library {
    /// `--with` name.
    pub name: &'static str,
    /// The global the library defines.
    pub global: &'static str,
    /// Its files in load order: (vendored path, ES module name or `None` for a classic script).
    pub files: &'static [(&'static str, Option<&'static str>)],
    pub about: &'static str,
}

pub const LIBRARIES: &[Library] = &[
    Library {
        name: "d3",
        global: "d3",
        files: &[("d3/d3.min.js", None)],
        about: "the whole d3 (scales, axes, shapes, layouts, hierarchy, geo projections, formats)",
    },
    Library {
        name: "three",
        global: "THREE",
        files: &[
            ("three/three.core.js", Some("three")),
            ("three/addons/renderers/Projector.js", Some("three/addons/renderers/Projector.js")),
            ("three/addons/renderers/SVGRenderer.js", Some("three/addons/renderers/SVGRenderer.js")),
            ("three/addons/geometries/RoundedBoxGeometry.js", Some("three/addons/geometries/RoundedBoxGeometry.js")),
        ],
        about: "three.js core plus THREE.SVGRenderer, THREE.Projector, and THREE.RoundedBoxGeometry",
    },
    Library {
        name: "topojson",
        global: "topojson",
        files: &[("topojson-client/topojson-client.min.js", None)],
        about: "topojson-client: feature, mesh, merge, neighbors for px.atlas() data",
    },
];

impl Library {
    pub fn find(name: &str) -> Option<&'static Library> {
        LIBRARIES.iter().find(|l| l.name == name)
    }

    /// The main file's manifest row (package, version, hash).
    pub fn main(&self) -> &'static FileInfo {
        info(self.files[0].0).expect("every library file is in the manifest")
    }

    pub fn version(&self) -> &'static str {
        self.main().version
    }

    /// `d3@7.9.0`
    pub fn pinned(&self) -> String {
        format!("{}@{}", self.name, self.version())
    }
}

/// `--with` names: `d3`, `three`, `topojson` (optionally `@version`), or `none`.
pub fn parse_with(raw: &str) -> Result<Vec<&'static Library>> {
    let mut out: Vec<&'static Library> = Vec::new();
    for tok in raw.split(',').map(str::trim).filter(|t| !t.is_empty()) {
        if tok == "none" {
            continue;
        }
        let (name, version) = match tok.split_once('@') {
            Some((n, v)) => (n.trim(), Some(v.trim())),
            None => (tok, None),
        };
        let lib = Library::find(name).ok_or_else(|| {
            Error::new("PX1550", format!("--with: '{name}' is not a bundled library"))
                .with_hint(
                    "use --with d3, three, topojson (comma-separated), or none; other libraries go in --lib file.js",
                )
                .with_context("available", LIBRARIES.iter().map(|l| l.name).collect::<Vec<_>>())
        })?;
        if let Some(v) = version {
            if v != lib.version() {
                return Err(Error::new(
                    "PX1552",
                    format!("--with {tok}: this build bundles {} {}, not {v}", lib.name, lib.version()),
                )
                .with_hint(format!(
                    "use --with {} (the bundled {}), replay with the pixelox release that bundled {} {v}, or pass that version's file with --lib",
                    lib.name,
                    lib.version(),
                    lib.name
                ))
                .with_context("requested", v)
                .with_context("bundled", lib.version()));
            }
        }
        if !out.iter().any(|l| l.name == lib.name) {
            out.push(lib);
        }
    }
    // load order: three before anything that may extend it, topojson last
    out.sort_by_key(|l| LIBRARIES.iter().position(|x| x.name == l.name));
    Ok(out)
}

// ---------------------------------------------------------------------------------------
// atlases

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AtlasKind {
    World,
    Land,
    UsStates,
    UsNation,
}

/// A map `px.atlas(name)` returns.
#[derive(Debug)]
pub struct Atlas {
    pub name: &'static str,
    pub file: &'static str,
    pub objects: &'static [&'static str],
    pub kind: AtlasKind,
    pub about: &'static str,
}

pub const ATLASES: &[Atlas] = &[
    Atlas {
        name: "world-110m",
        file: "world-atlas/countries-110m.json",
        objects: &["countries", "land"],
        kind: AtlasKind::World,
        about: "177 countries and the land, Natural Earth 1:110m; longitude/latitude (any d3 projection)",
    },
    Atlas {
        name: "world-50m",
        file: "world-atlas/countries-50m.json",
        objects: &["countries", "land"],
        kind: AtlasKind::World,
        about: "241 countries and territories and the land, Natural Earth 1:50m (finer coasts, small islands)",
    },
    Atlas {
        name: "land-110m",
        file: "world-atlas/land-110m.json",
        objects: &["land"],
        kind: AtlasKind::Land,
        about: "the land masses only, Natural Earth 1:110m (a light background or a globe)",
    },
    Atlas {
        name: "us-states",
        file: "us-atlas/states-10m.json",
        objects: &["states", "nation"],
        kind: AtlasKind::UsStates,
        about: "50 states, DC, and 5 territories, Census 2017; longitude/latitude: draw with d3.geoAlbersUsa() (Alaska and Hawaii as insets)",
    },
    Atlas {
        name: "us-states-albers",
        file: "us-atlas/states-albers-10m.json",
        objects: &["states", "nation"],
        kind: AtlasKind::UsStates,
        about: "50 states and DC already projected (Albers USA, insets) into a 975x610 frame: draw with d3.geoPath() and no projection",
    },
    Atlas {
        name: "us-nation",
        file: "us-atlas/nation-10m.json",
        objects: &["nation"],
        kind: AtlasKind::UsNation,
        about: "the U.S. outline, Census 2017; longitude/latitude",
    },
];

impl Atlas {
    pub fn find(name: &str) -> Option<&'static Atlas> {
        ATLASES.iter().find(|a| a.name == name)
    }

    pub fn file_info(&self) -> &'static FileInfo {
        info(self.file).expect("every atlas is in the manifest")
    }

    /// The code table whose values are added to the features (it shapes the data too).
    pub fn codes(&self) -> Option<&'static FileInfo> {
        match self.kind {
            AtlasKind::World => info("i18n-iso-countries/codes.json"),
            AtlasKind::UsStates => info("us-states.tsv"),
            AtlasKind::Land | AtlasKind::UsNation => None,
        }
    }
}

pub fn unknown_atlas(name: &str) -> Error {
    Error::new("PX1551", format!("px.atlas('{name}'): no such atlas"))
        .with_hint(format!("use one of {} (`pixelox docs atlas` lists their features)", atlas_names().join(", ")))
        .with_context("available", atlas_names())
}

pub fn atlas_names() -> Vec<&'static str> {
    ATLASES.iter().map(|a| a.name).collect()
}

/// ISO 3166-1 numeric code → (alpha-2, alpha-3), from i18n-iso-countries' codes.json.
fn iso_codes() -> Result<&'static BTreeMap<String, (String, String)>> {
    static CODES: OnceLock<BTreeMap<String, (String, String)>> = OnceLock::new();
    if let Some(c) = CODES.get() {
        return Ok(c);
    }
    let text = text("i18n-iso-countries/codes.json")?;
    let rows: Vec<Vec<String>> = serde_json::from_str(&text)
        .map_err(|e| Error::new("PX1553", format!("the bundled ISO 3166 table is unreadable: {e}")))?;
    let map = rows.into_iter().filter(|r| r.len() >= 3).map(|r| (r[2].clone(), (r[0].clone(), r[1].clone()))).collect();
    Ok(CODES.get_or_init(|| map))
}

/// FIPS state code → (USPS code, name), from vendor/us-states.tsv.
fn us_states() -> Result<&'static BTreeMap<String, (String, String)>> {
    static STATES: OnceLock<BTreeMap<String, (String, String)>> = OnceLock::new();
    if let Some(s) = STATES.get() {
        return Ok(s);
    }
    let text = text("us-states.tsv")?;
    let map = text
        .lines()
        .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
        .filter_map(|l| {
            let c: Vec<&str> = l.split('\t').collect();
            (c.len() == 3).then(|| (c[0].to_string(), (c[1].to_string(), c[2].to_string())))
        })
        .collect();
    Ok(STATES.get_or_init(|| map))
}

/// Kosovo has no ISO 3166-1 code; XK / XKX is the user-assigned code in wide use (EU, World Bank).
const KOSOVO: (&str, &str) = ("XK", "XKX");

/// The atlas's TopoJSON with join codes added to feature properties. Countries keep `id`
/// (ISO 3166-1 numeric, a string such as "840") and `name`, and gain `iso_a2` / `iso_a3`;
/// states keep `id` (FIPS, "06") and `name` and gain `postal` ("CA") and `fips`.
pub fn atlas_json(atlas: &Atlas) -> Result<Arc<str>> {
    static CACHE: OnceLock<Mutex<HashMap<&'static str, Arc<str>>>> = OnceLock::new();
    let cache = CACHE.get_or_init(Default::default);
    if let Some(t) = cache.lock().unwrap_or_else(|e| e.into_inner()).get(atlas.name) {
        return Ok(t.clone());
    }
    let raw = text(atlas.file)?;
    let mut topo: Value = serde_json::from_str(&raw)
        .map_err(|e| Error::new("PX1553", format!("the bundled atlas {} is unreadable: {e}", atlas.file)))?;
    enrich(&mut topo, atlas.kind)?;
    let t: Arc<str> = topo.to_string().into();
    cache.lock().unwrap_or_else(|e| e.into_inner()).insert(atlas.name, t.clone());
    Ok(t)
}

fn geometries<'a>(topo: &'a mut Value, object: &str) -> impl Iterator<Item = &'a mut Map<String, Value>> {
    topo.pointer_mut(&format!("/objects/{object}/geometries"))
        .and_then(Value::as_array_mut)
        .into_iter()
        .flatten()
        .filter_map(Value::as_object_mut)
}

fn props(g: &mut Map<String, Value>) -> &mut Map<String, Value> {
    let p = g.entry("properties").or_insert_with(|| Value::Object(Map::new()));
    if !p.is_object() {
        *p = Value::Object(Map::new());
    }
    p.as_object_mut().expect("just made an object")
}

fn enrich(topo: &mut Value, kind: AtlasKind) -> Result<()> {
    match kind {
        AtlasKind::World => {
            let codes = iso_codes()?;
            for g in geometries(topo, "countries") {
                let id = g.get("id").and_then(Value::as_str).map(String::from);
                let p = props(g);
                let found = match id.as_deref().and_then(|i| codes.get(i)) {
                    Some((a2, a3)) => Some((a2.as_str(), a3.as_str())),
                    None if p.get("name").and_then(Value::as_str) == Some("Kosovo") => Some(KOSOVO),
                    None => None,
                };
                if let Some((a2, a3)) = found {
                    p.insert("iso_a2".into(), json!(a2));
                    p.insert("iso_a3".into(), json!(a3));
                }
            }
        }
        AtlasKind::UsStates => {
            let states = us_states()?;
            for g in geometries(topo, "states") {
                let id = g.get("id").and_then(Value::as_str).map(String::from);
                if let Some((postal, _)) = id.as_deref().and_then(|i| states.get(i)) {
                    let p = props(g);
                    p.insert("postal".into(), json!(postal));
                    p.insert("fips".into(), json!(id));
                }
            }
        }
        AtlasKind::Land | AtlasKind::UsNation => {}
    }
    Ok(())
}

/// `px.atlases()`: name, objects, and description of every atlas.
pub fn atlas_list() -> Value {
    Value::Array(
        ATLASES
            .iter()
            .map(|a| {
                let f = a.file_info();
                json!({"name": a.name, "objects": a.objects, "about": a.about, "package": f.package, "version": f.version})
            })
            .collect(),
    )
}

/// The `pixelox docs atlas` page: libraries, atlases, and every feature with its codes.
pub fn atlas_topic() -> Result<String> {
    let mut s = String::new();
    let _ = writeln!(s, "# Bundled map data and JavaScript libraries\n");
    let _ = writeln!(
        s,
        "`render d3` and `render three` load bundled libraries, so a script needs no `--lib`. \
Every script (`render js` too) can read the bundled maps with `px.atlas(name)`, which returns \
TopoJSON; `topojson.feature()` turns an object of it into GeoJSON for `d3.geoPath()`. This \
page is generated from the bundled files (ADR 0018); the cookbook \
`docs/guides/cookbook/charts-and-maps-with-d3.md` has complete charts and maps.\n"
    );
    let _ = writeln!(s, "## Libraries (`--with`)\n");
    let _ = writeln!(s, "| `--with` | Global | Package | Version | License | What it gives |");
    let _ = writeln!(s, "| --- | --- | --- | --- | --- | --- |");
    for l in LIBRARIES {
        let f = l.main();
        let _ = writeln!(
            s,
            "| `{}` | `{}` | {} | {} | {} | {} |",
            l.name, l.global, f.package, f.version, f.license, l.about
        );
    }
    let _ = writeln!(
        s,
        "\nDefaults: `render d3` loads `d3,topojson`, `render three` loads `three,topojson`, `render js` \
loads nothing (`--with d3` opts in). A `--lib` file replaces the bundled `d3`/`three` unless \
`--with` names it; with `--with d3 --lib d3-sankey.js` a plugin extends the bundled d3. The \
versions that ran are recorded in the sidecar (`bundled`) and pinned in the recipe \
(`with: d3@{}`).\n",
        LIBRARIES[0].version()
    );
    let _ = writeln!(s, "## Atlases (`px.atlas(name)`)\n");
    let _ = writeln!(s, "| Name | Objects | Source | What it is |");
    let _ = writeln!(s, "| --- | --- | --- | --- |");
    for a in ATLASES {
        let f = a.file_info();
        let _ = writeln!(
            s,
            "| `{}` | {} | {} {} `{}` | {} |",
            a.name,
            a.objects.iter().map(|o| format!("`{o}`")).collect::<Vec<_>>().join(", "),
            f.package,
            f.version,
            f.path.rsplit('/').next().unwrap_or(f.path),
            a.about
        );
    }
    let _ = writeln!(
        s,
        "\nGeometry: Natural Earth (public domain) through world-atlas (ISC); U.S. Census Bureau \
cartographic boundaries (public domain) through us-atlas (ISC). Boundaries are Natural Earth's \
de facto lines.\n"
    );
    let _ = writeln!(s, "```js\nconst us = px.atlas('us-states');                       // TopoJSON");
    let _ = writeln!(s, "const states = topojson.feature(us, us.objects.states);   // GeoJSON FeatureCollection");
    let _ = writeln!(s, "const borders = topojson.mesh(us, us.objects.states, (a, b) => a !== b);");
    let _ = writeln!(s, "const path = d3.geoPath(d3.geoAlbersUsa().fitSize([960, 600], states));");
    let _ = writeln!(s, "// each feature: id '06', properties {{name: 'California', postal: 'CA', fips: '06'}}");
    let _ = writeln!(s, "const world = px.atlas('world-110m');");
    let _ = writeln!(s, "const countries = topojson.feature(world, world.objects.countries);");
    let _ = writeln!(
        s,
        "// each country: id '840', properties {{name: 'United States of America', iso_a2: 'US', iso_a3: 'USA'}}"
    );
    let _ = writeln!(s, "```\n");
    let _ = writeln!(s, "`px.atlases()` returns this list inside a script.\n");

    // U.S. states
    let states_topo: Value = serde_json::from_str(&atlas_json(Atlas::find("us-states").unwrap())?)
        .map_err(|e| Error::internal(e.to_string()))?;
    let albers: Value = serde_json::from_str(&atlas_json(Atlas::find("us-states-albers").unwrap())?)
        .map_err(|e| Error::internal(e.to_string()))?;
    let in_albers: Vec<String> = albers["objects"]["states"]["geometries"]
        .as_array()
        .map(|a| a.iter().filter_map(|g| g["id"].as_str().map(String::from)).collect())
        .unwrap_or_default();
    let _ = writeln!(s, "## U.S. states (`us-states`, `us-states-albers`)\n");
    let _ = writeln!(
        s,
        "`id` is the FIPS code (a string). Join data on `properties.postal` (`CA`), \
`properties.name`, or `id`. The five territories (AS, GU, MP, PR, VI) are only in `us-states`, \
and `d3.geoAlbersUsa()` draws none of them (it projects the 50 states and DC only).\n"
    );
    let _ = writeln!(s, "| id (FIPS) | postal | name | in us-states-albers |");
    let _ = writeln!(s, "| --- | --- | --- | --- |");
    if let Some(gs) = states_topo["objects"]["states"]["geometries"].as_array() {
        let mut rows: Vec<(&str, &str, &str)> = gs
            .iter()
            .map(|g| {
                (
                    g["id"].as_str().unwrap_or(""),
                    g["properties"]["postal"].as_str().unwrap_or(""),
                    g["properties"]["name"].as_str().unwrap_or(""),
                )
            })
            .collect();
        rows.sort();
        for (id, postal, name) in rows {
            let yes = if in_albers.iter().any(|x| x == id) { "yes" } else { "no" };
            let _ = writeln!(s, "| {id} | {postal} | {name} | {yes} |");
        }
    }

    // countries
    let w50: Value = serde_json::from_str(&atlas_json(Atlas::find("world-50m").unwrap())?)
        .map_err(|e| Error::internal(e.to_string()))?;
    let w110: Value = serde_json::from_str(&atlas_json(Atlas::find("world-110m").unwrap())?)
        .map_err(|e| Error::internal(e.to_string()))?;
    let names_110: Vec<String> = w110["objects"]["countries"]["geometries"]
        .as_array()
        .map(|a| a.iter().filter_map(|g| g["properties"]["name"].as_str().map(String::from)).collect())
        .unwrap_or_default();
    let _ = writeln!(s, "\n## Countries (`world-110m`, `world-50m`)\n");
    let _ = writeln!(
        s,
        "`id` is the ISO 3166-1 numeric code (a string, `\"036\"`); `iso_a2` and `iso_a3` are added \
by pixelox (Kosovo gets the user-assigned XK / XKX). Names are Natural Earth's short names \
(`Dem. Rep. Congo`, `United States of America`): join data on `iso_a3` where you can. A few \
features have no code (N. Cyprus, Somaliland, Siachen Glacier, Indian Ocean Ter.). In \
`world-50m`, Ashmore and Cartier Is. share Australia's id.\n"
    );
    let _ = writeln!(s, "| id | iso_a2 | iso_a3 | name | in world-110m |");
    let _ = writeln!(s, "| --- | --- | --- | --- | --- |");
    if let Some(gs) = w50["objects"]["countries"]["geometries"].as_array() {
        let mut rows: Vec<(&str, &str, &str, &str)> = gs
            .iter()
            .map(|g| {
                (
                    g["properties"]["name"].as_str().unwrap_or(""),
                    g["id"].as_str().unwrap_or("-"),
                    g["properties"]["iso_a2"].as_str().unwrap_or("-"),
                    g["properties"]["iso_a3"].as_str().unwrap_or("-"),
                )
            })
            .collect();
        rows.sort();
        for (name, id, a2, a3) in rows {
            let yes = if names_110.iter().any(|n| n == name) { "yes" } else { "no" };
            let _ = writeln!(s, "| {id} | {a2} | {a3} | {name} | {yes} |");
        }
    }
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_vendored_file_inflates_and_matches_its_hash() {
        assert_eq!(FILES.len(), files().len());
        for f in files() {
            let t = text(f.path).unwrap();
            assert_eq!(t.len(), f.bytes, "{}", f.path);
        }
        assert!(text("nope.js").is_err());
    }

    #[test]
    fn with_names_parse_pin_and_order() {
        let libs = parse_with("topojson, d3").unwrap();
        assert_eq!(libs.iter().map(|l| l.name).collect::<Vec<_>>(), ["d3", "topojson"]);
        assert!(parse_with("none").unwrap().is_empty());
        let pinned = format!("three@{}", Library::find("three").unwrap().version());
        assert_eq!(parse_with(&pinned).unwrap()[0].name, "three");
        assert_eq!(parse_with("d3@1.0.0").unwrap_err().code, "PX1552");
        assert_eq!(parse_with("lodash").unwrap_err().code, "PX1550");
        assert_eq!(Library::find("d3").unwrap().pinned(), "d3@7.9.0");
    }

    #[test]
    fn atlases_carry_join_codes() {
        let us: Value = serde_json::from_str(&atlas_json(Atlas::find("us-states").unwrap()).unwrap()).unwrap();
        let states = us["objects"]["states"]["geometries"].as_array().unwrap();
        assert_eq!(states.len(), 56);
        let ca = states.iter().find(|g| g["id"] == "06").unwrap();
        assert_eq!(ca["properties"], json!({"name": "California", "postal": "CA", "fips": "06"}));
        assert!(states.iter().all(|g| g["properties"]["postal"].is_string()));
        let albers: Value =
            serde_json::from_str(&atlas_json(Atlas::find("us-states-albers").unwrap()).unwrap()).unwrap();
        assert_eq!(albers["objects"]["states"]["geometries"].as_array().unwrap().len(), 51);
        let w: Value = serde_json::from_str(&atlas_json(Atlas::find("world-110m").unwrap()).unwrap()).unwrap();
        let cs = w["objects"]["countries"]["geometries"].as_array().unwrap();
        assert_eq!(cs.len(), 177);
        let usa = cs.iter().find(|g| g["id"] == "840").unwrap();
        assert_eq!(usa["properties"]["iso_a3"], "USA");
        assert_eq!(usa["properties"]["iso_a2"], "US");
        let kosovo = cs.iter().find(|g| g["properties"]["name"] == "Kosovo").unwrap();
        assert_eq!(kosovo["properties"]["iso_a3"], "XKX");
        let coded = cs.iter().filter(|g| g["properties"]["iso_a3"].is_string()).count();
        assert_eq!(coded, 175, "all but N. Cyprus and Somaliland");
        assert_eq!(unknown_atlas("mars").code, "PX1551");
    }
}
