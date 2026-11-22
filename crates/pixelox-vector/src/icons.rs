//! `shape icon` and `shape icons`: the Lucide icon set (ISC) as live vector shape layers.
//!
//! The icons live in `data/lucide-icons.txt`, generated from a lucide-static release by
//! `scripts/gen-lucide.py` (source, version, and hash in the file's header; license in
//! `data/LICENSE-lucide.txt`). Each line is `name<TAB>aliases<TAB>tags<TAB>path data` on
//! Lucide's 24 × 24 grid; icons are drawn the Lucide way: a round-capped stroke 2 units
//! wide, no fill. Brand logos are not in Lucide, and the few trademark symbols that are
//! (Bluetooth, Creative Commons) are left out by the generator.

use crate::prims::{self, Built};
use crate::spec::{self, ShapeSpec, PREC};
use kurbo::{Affine, BezPath};
use pixelox_core::prelude::*;
use std::sync::OnceLock;

const DATA: &str = include_str!("../data/lucide-icons.txt");

/// The Lucide release bundled (from the data file's header).
pub fn version() -> &'static str {
    DATA.lines()
        .next()
        .and_then(|l| l.strip_prefix("# Lucide "))
        .and_then(|l| l.split_whitespace().next())
        .unwrap_or("?")
}

/// One icon of the bundled set.
#[derive(Debug)]
pub struct Icon {
    pub name: &'static str,
    pub aliases: Vec<&'static str>,
    pub tags: Vec<&'static str>,
    /// SVG path data on the 24 × 24 grid.
    pub d: &'static str,
}

/// Every bundled icon, in name order.
pub fn all() -> &'static [Icon] {
    static ICONS: OnceLock<Vec<Icon>> = OnceLock::new();
    ICONS.get_or_init(|| {
        let list = |s: &'static str| s.split(',').filter(|t| !t.is_empty()).collect::<Vec<_>>();
        DATA.lines()
            .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
            .filter_map(|l| {
                let mut f = l.split('\t');
                let (name, aliases, tags, d) = (f.next()?, f.next()?, f.next()?, f.next()?);
                Some(Icon { name, aliases: list(aliases), tags: list(tags), d })
            })
            .collect()
    })
}

/// Normalize a name as typed: lower case, spaces and underscores as hyphens.
fn norm(s: &str) -> String {
    s.trim().to_ascii_lowercase().replace([' ', '_'], "-")
}

/// The icon with this name or alias.
pub fn find(name: &str) -> Option<&'static Icon> {
    let n = norm(name);
    let icons = all();
    icons
        .binary_search_by(|i| i.name.cmp(n.as_str()))
        .ok()
        .map(|i| &icons[i])
        .or_else(|| icons.iter().find(|i| i.aliases.iter().any(|a| *a == n)))
}

/// Icons matching every word of `query`, best first: name words weigh most, then aliases,
/// then Lucide's tags. Ties keep name order.
pub fn search(query: &str) -> Vec<&'static Icon> {
    let words: Vec<String> =
        query.split(|c: char| c.is_whitespace() || c == ',').map(norm).filter(|w| !w.is_empty()).collect();
    if words.is_empty() {
        return all().iter().collect();
    }
    let mut hits: Vec<(u32, &Icon)> = all()
        .iter()
        .filter_map(|icon| {
            let mut total = 0;
            for w in &words {
                let parts: Vec<&str> = icon.name.split('-').collect();
                let s = if icon.name == w {
                    1000
                } else if icon.aliases.iter().any(|a| a == w) {
                    900
                } else if parts.first() == Some(&w.as_str()) {
                    300
                } else if parts.iter().any(|p| p == w) {
                    200
                } else if icon.name.contains(w.as_str()) {
                    90
                } else if icon.aliases.iter().any(|a| a.contains(w.as_str())) {
                    70
                } else if icon.tags.iter().any(|t| t == w) {
                    60
                } else if icon.tags.iter().any(|t| t.split(' ').any(|p| p.starts_with(w.as_str()))) {
                    20
                } else {
                    return None;
                };
                total += s;
            }
            // shorter names are the more general icon (calendar before calendar-check)
            Some((total * 100 - icon.name.len().min(99) as u32, icon))
        })
        .collect();
    hits.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.name.cmp(b.1.name)));
    hits.into_iter().map(|(_, i)| i).collect()
}

/// Levenshtein distance (names are short).
fn distance(a: &str, b: &str) -> usize {
    let b: Vec<char> = b.chars().collect();
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    for (i, ca) in a.chars().enumerate() {
        let mut cur = vec![i + 1];
        for (j, cb) in b.iter().enumerate() {
            cur.push((prev[j] + usize::from(ca != *cb)).min(prev[j + 1] + 1).min(cur[j] + 1));
        }
        prev = cur;
    }
    prev[b.len()]
}

/// Close names for a misspelling: search hits, else the nearest names by edit distance.
fn suggestions(name: &str) -> Vec<&'static str> {
    let hits: Vec<&str> = search(name).iter().take(6).map(|i| i.name).collect();
    if !hits.is_empty() {
        return hits;
    }
    let n = norm(name);
    let mut near: Vec<(usize, &str)> = all()
        .iter()
        .flat_map(|i| std::iter::once(i.name).chain(i.aliases.iter().copied()).map(move |w| (w, i.name)))
        .map(|(w, canon)| (distance(&n, w), canon))
        .filter(|(d, _)| *d <= 3)
        .collect();
    near.sort();
    near.dedup_by(|a, b| a.1 == b.1);
    near.into_iter().take(6).map(|(_, c)| c).collect()
}

fn unknown(name: &str) -> Error {
    let near = suggestions(name);
    let hint = if near.is_empty() {
        "list icons with `pixelox shape icons`, or search: `pixelox shape icons --search <word>`".to_string()
    } else {
        format!("did you mean: {}? (search with `pixelox shape icons --search {}`)", near.join(", "), norm(name))
    };
    Error::new("PX6030", format!("no icon named '{name}' in the bundled Lucide {} set", version()))
        .with_hint(hint)
        .with_context("icon", name)
        .with_context("suggestions", near)
}

pub fn ops() -> Vec<OpSpec> {
    let mut icon = OpSpec::doc("shape icon", Category::Vector, "Add a Lucide icon as a shape layer", icon)
        .about(
            "Adds one of the ~2,100 bundled Lucide icons (ISC license; no brand logos) as a live vector shape \
             layer: a round-capped outline drawn on a 24-unit grid and scaled to --size. Find names with \
             `pixelox shape icons --search WORD`; common words work too (cart, location, email, web, person). \
             --stroke-width defaults to Lucide's weight (size / 12); --fill also fills the icon's closed outlines \
             (heart, star, map-pin) with a second color. Placement follows `comp place`: --at with --anchor, \
             --anchor alone for a canvas point, or centered. The box is snapped to whole pixels, so sizes that are \
             multiples of 24 render pixel-sharp. Restyle later with `shape set`.",
        )
        .positional("icon")
        .param(param("icon", ParamKind::Str).required().help("Icon name or alias, e.g. calendar, map-pin, ticket"))
        .param(
            param("size", ParamKind::Length)
                .default("48")
                .help("Icon size (width = height) in px or % of the shorter canvas side"),
        )
        .params(prims::place_params("icon"))
        .param(
            param("rotation", ParamKind::Angle)
                .default("0")
                .help("Clockwise rotation in degrees about the icon's center"),
        )
        .param(param("color", ParamKind::Color).default("#000000").help("Line color"))
        .param(
            param("stroke-width", ParamKind::Float)
                .auto()
                .default("auto")
                .range("0..1000")
                .help("Line width in px; auto = size / 12 (Lucide's 2 units)"),
        )
        .param(
            param("fill", ParamKind::Str)
                .default("none")
                .help("Fill for the icon's closed outlines (CSS color), or none"),
        )
        .param(param("opacity", ParamKind::Float).range("0..100").default("100").help("Layer opacity 0-100"))
        .param(param("name", ParamKind::Str).help("Layer name (default: the icon name, made unique)"));
    for (cmd, explain) in [
        (
            "pixelox shape icon flyer.pxo calendar --in-place --size 48 --at 120,1180 --color white",
            "A white calendar icon in an info bar",
        ),
        (
            "pixelox shape icon flyer.pxo map-pin --in-place --size 4% --at 50%,90% --anchor c --color '#ffd23c'",
            "A pin sized relative to the canvas",
        ),
        (
            "pixelox shape icon card.pxo heart --in-place --size 96 --anchor tr --color '#e11d48' --fill '#e11d48'",
            "A solid red heart in the top-right corner",
        ),
        (
            "pixelox shape icon doc.pxo ticket --in-place --size 72 --rotation -12 --stroke-width 4 --name Ticket",
            "A tilted, bolder ticket",
        ),
    ] {
        icon = icon.example(cmd, explain);
    }
    vec![
        icon,
        OpSpec::custom("shape icons", Category::Vector, "List or search the bundled icons", icons)
            .about(
                "Lists the bundled Lucide icons that `shape icon` draws. With --search, ranks icons whose name, \
                 alias, or tags match every word (calendar finds calendar, calendar-check, calendar-days...; event \
                 finds them through their tags). Without it, prints every name.",
            )
            .inputs(Inputs::NONE)
            .param(param("search", ParamKind::Str).help("Words to match against names, aliases, and tags"))
            .param(
                param("limit", ParamKind::Int)
                    .default("30")
                    .range("0..10000")
                    .help("Most matches to return with --search (0 = all)"),
            )
            .example("pixelox shape icons --search calendar", "Calendar icons")
            .example("pixelox shape icons --search 'music note' --json", "Icons matching both words, as JSON")
            .example("pixelox shape icons --json", "Every icon name"),
    ]
}

fn icons(a: &Args, _ctx: &mut OpCtx) -> Result<Value> {
    let entry = |i: &Icon| json!({"name": i.name, "aliases": i.aliases, "tags": i.tags});
    match a.str("search").filter(|s| !s.trim().is_empty()) {
        Some(q) => {
            let hits = search(&q);
            let limit = a.usize("limit")?;
            let shown: Vec<Value> =
                hits.iter().take(if limit == 0 { usize::MAX } else { limit }).map(|i| entry(i)).collect();
            Ok(json!({"set": "lucide", "version": version(), "query": q, "count": hits.len(), "icons": shown}))
        }
        None => {
            let names: Vec<&str> = all().iter().map(|i| i.name).collect();
            Ok(json!({"set": "lucide", "version": version(), "count": names.len(), "icons": names}))
        }
    }
}

/// The icon's path on the canvas: the 24-unit grid scaled to `size`, rotated about its
/// center, with its top-left at (`x`, `y`).
pub fn icon_path(icon: &Icon, x: f64, y: f64, size: f64, rotation: f64) -> Result<BezPath> {
    let mut p = spec::parse_d(icon.d)?;
    let k = size / 24.0;
    let tf = Affine::translate((x, y))
        * Affine::scale(k)
        * Affine::translate((12.0, 12.0))
        * Affine::rotate(rotation.to_radians())
        * Affine::translate((-12.0, -12.0));
    p.apply_affine(tf);
    Ok(p)
}

fn icon(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let want = a.str("icon").unwrap_or_default();
    let ic = find(&want).ok_or_else(|| unknown(&want))?;
    let size = a.px("size", doc.width.min(doc.height) as f64, doc.dpi())?;
    if !size.is_finite() || size <= 0.0 {
        return Err(Error::new("PX1013", format!("--size must be positive, got {size}")).with_context("param", "size"));
    }
    let (x, y) = prims::place_box(a, doc, size, size)?;
    let rot = a.angle("rotation")?;
    let sw = match a.opt_f64("stroke-width")? {
        Some(w) => w,
        None => {
            let w = size / 12.0;
            ctx.resolve_param("stroke-width", w);
            w
        }
    };
    let fill = spec::parse_paint(&a.str_or("fill", "none")).map_err(|mut e| {
        e.message = format!("--fill: {}", e.message);
        e
    })?;
    let bez = icon_path(ic, x, y, size, rot)?;
    let geometry = json!({
        "type": "icon", "set": "lucide", "version": version(), "icon": ic.name,
        "box": prims::rect_str(x, y, size, size), "rotation": rot,
    });
    let mut s = ShapeSpec::new(spec::bez_to_d(&bez, PREC), geometry);
    s.stroke = Some(a.color("color")?).filter(|c| c.a > 0.0);
    s.stroke_width = sw;
    s.fill = fill;
    s.cap = "round".into();
    s.join = "round".into();
    s.fill_rule = "evenodd".into();
    let name = prims::layer_name(a, doc, ic.name)?;
    let mut r = prims::add(a, doc, Built { name, spec: s, plate: None, opacity: a.f64("opacity")? as f32 }, "icon")?;
    r["icon"] = json!(ic.name);
    r["box"] = json!([x, y, size, size]);
    r["stroke_width"] = json!(sw);
    Ok(r)
}
