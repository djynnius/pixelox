//! Generative rendering for pixelox (PRD §7.10, §7.11, §8.16):
//!
//! * `render js` / `render d3` / `render three`: scripts in a QuickJS sandbox with a frozen
//!   `px` API (a Canvas2D context drawn in Rust over tiny-skia, a DOM-lite SVG document);
//! * `render 3d`: a native CPU 3D-lite renderer (primitives, SVG-profile extrusions, OBJ
//!   meshes; wireframe, hidden-line, and flat-shaded styles; SVG or raster output);
//! * `render splash` / `render droplets` / `render ice`: native liquid and ice generators
//!   that add a layer.

mod js;
mod liquid;
mod three_d;

/// Bundled JavaScript libraries and map data (ADR 0018).
pub use js::bundled;

use pixelox_core::ctx::format_of;
use pixelox_core::prelude::*;
use std::path::Path;

/// Every command this crate defines.
pub fn ops() -> Vec<OpSpec> {
    let mut v = js::ops();
    v.extend(three_d::ops());
    v.extend(liquid::ops());
    v
}

/// Install renderer hooks (none: this crate only adds commands).
pub fn install() {}

/// The output format: `--format` when given, else the `-o` extension (`png` by default).
pub(crate) fn output_format(ctx: &OpCtx, out: &Path) -> String {
    ctx.globals
        .format
        .as_ref()
        .map(|f| f.trim().to_ascii_lowercase())
        .or_else(|| format_of(out))
        .unwrap_or_else(|| "png".into())
}

/// usvg options for untrusted SVG: no external files; with `with_fonts`, `<text>` draws with
/// pixelox's fonts (see [`text_fontdb`]).
pub(crate) fn svg_options(with_fonts: bool) -> usvg::Options<'static> {
    let mut opt = usvg::Options {
        resources_dir: None,
        image_href_resolver: usvg::ImageHrefResolver {
            resolve_data: usvg::ImageHrefResolver::default_data_resolver(),
            resolve_string: Box::new(|_, _| None),
        },
        ..Default::default()
    };
    if with_fonts {
        opt.fontdb = std::sync::Arc::new(text_fontdb());
    }
    opt
}

/// The fonts SVG `<text>` uses: the database `text add` uses (system fonts unless
/// `PIXELOX_SYSTEM_FONTS=0`, `fonts add` folders, `fonts get` downloads, the embedded Tuffy),
/// with the CSS generic families (`sans-serif`, `serif`, `monospace`, ...) mapped to installed
/// faces, so d3's `font-family="sans-serif"` axis labels draw on any machine. Where the usual
/// faces (Arial, Times New Roman, Courier New) are installed the choice is unchanged.
fn text_fontdb() -> usvg::fontdb::Database {
    const SANS: &[&str] = &["Arial", "Helvetica", "Liberation Sans", "DejaVu Sans", "Noto Sans", "Roboto", "Inter"];
    const SERIF: &[&str] = &["Times New Roman", "Times", "Liberation Serif", "DejaVu Serif", "Noto Serif", "Georgia"];
    const MONO: &[&str] =
        &["Courier New", "Courier", "Menlo", "Liberation Mono", "DejaVu Sans Mono", "Noto Sans Mono", "Consolas"];
    let mut db = pixelox_text::fonts::db().db.clone();
    let installed = |prefs: &[&str]| -> Option<String> {
        prefs
            .iter()
            .find(|p| db.faces().any(|f| f.families.iter().any(|(n, _)| n.eq_ignore_ascii_case(p))))
            .map(|s| s.to_string())
    };
    let fallback = pixelox_text::fonts::FALLBACK_FAMILY.to_string();
    let sans = installed(SANS).unwrap_or_else(|| fallback.clone());
    let serif = installed(SERIF).unwrap_or_else(|| sans.clone());
    let mono = installed(MONO).unwrap_or_else(|| sans.clone());
    let cursive = installed(&["Comic Sans MS", "Apple Chancery"]).unwrap_or_else(|| sans.clone());
    let fantasy = installed(&["Impact", "Papyrus"]).unwrap_or_else(|| sans.clone());
    db.set_sans_serif_family(sans);
    db.set_serif_family(serif);
    db.set_monospace_family(mono);
    db.set_cursive_family(cursive);
    db.set_fantasy_family(fantasy);
    db
}

/// The faces an SVG's `<text>` drew with, as sidecar font records (so replay names a
/// missing one, ADR 0015).
pub(crate) fn svg_fonts(tree: &usvg::Tree) -> Vec<serde_json::Value> {
    fn walk(g: &usvg::Group, ids: &mut Vec<usvg::fontdb::ID>) {
        for n in g.children() {
            match n {
                usvg::Node::Group(g) => walk(g, ids),
                usvg::Node::Text(t) => {
                    for s in t.layouted() {
                        for gl in &s.positioned_glyphs {
                            if !ids.contains(&gl.font) {
                                ids.push(gl.font);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
    let mut ids = Vec::new();
    walk(tree.root(), &mut ids);
    if ids.is_empty() {
        return Vec::new();
    }
    let db = pixelox_text::fonts::db();
    let mut out: Vec<serde_json::Value> = Vec::new();
    for id in ids {
        if let Ok(c) = pixelox_text::fonts::choice_by_id(&db, id) {
            let r = c.record();
            if !out.contains(&r) {
                out.push(r);
            }
        }
    }
    out
}

/// Rasterize SVG text at its own size, or scaled to fit `size`.
#[cfg(test)]
pub(crate) fn rasterize_svg(svg: &str, size: Option<(u32, u32)>) -> Result<tiny_skia::Pixmap> {
    rasterize_svg_fonts(svg, size).map(|(pm, _)| pm)
}

/// [`rasterize_svg`], also returning the font records of its `<text>`.
pub(crate) fn rasterize_svg_fonts(
    svg: &str,
    size: Option<(u32, u32)>,
) -> Result<(tiny_skia::Pixmap, Vec<serde_json::Value>)> {
    let opt = svg_options(svg.contains("<text"));
    let tree = usvg::Tree::from_str(svg, &opt)
        .map_err(|e| Error::new("PX3001", format!("the script produced invalid SVG: {e}")))?;
    let fonts = svg_fonts(&tree);
    let s = tree.size();
    let (w, h) = match size {
        Some(wh) => wh,
        None => (s.width().ceil().max(1.0) as u32, s.height().ceil().max(1.0) as u32),
    };
    let mut pm = tiny_skia::Pixmap::new(w, h)
        .ok_or_else(|| Error::new("PX1104", format!("cannot allocate a {w}x{h} raster")))?;
    let ts = tiny_skia::Transform::from_scale(w as f32 / s.width(), h as f32 / s.height());
    resvg::render(&tree, ts, &mut pm.as_mut());
    Ok((pm, fonts))
}

/// Standard base64 (for PNG data URIs).
pub(crate) fn base64(bytes: &[u8]) -> String {
    const T: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut s = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for c in bytes.chunks(3) {
        let n = (c[0] as u32) << 16 | (*c.get(1).unwrap_or(&0) as u32) << 8 | *c.get(2).unwrap_or(&0) as u32;
        s.push(T[(n >> 18) as usize & 63] as char);
        s.push(T[(n >> 12) as usize & 63] as char);
        s.push(if c.len() > 1 { T[(n >> 6) as usize & 63] as char } else { '=' });
        s.push(if c.len() > 2 { T[n as usize & 63] as char } else { '=' });
    }
    s
}

/// Number of elements in serialized markup (`<name` openings).
pub(crate) fn count_elements(svg: &str) -> usize {
    svg.as_bytes().windows(2).filter(|w| w[0] == b'<' && w[1].is_ascii_alphabetic()).count()
}

#[cfg(test)]
mod tests;
