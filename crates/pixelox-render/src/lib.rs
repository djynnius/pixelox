//! Generative rendering for pixelox (PRD §7.10, §7.11, §8.16):
//!
//! * `render js` / `render d3` / `render three`: scripts in a QuickJS sandbox with a frozen
//!   `px` API (a Canvas2D context drawn in Rust over tiny-skia, a DOM-lite SVG document);
//! * `render 3d`: a native CPU 3D-lite renderer (primitives, SVG-profile extrusions, OBJ
//!   meshes; wireframe, hidden-line, and flat-shaded styles; SVG or raster output).

mod js;
mod three_d;

use pixelox_core::ctx::format_of;
use pixelox_core::prelude::*;
use std::path::Path;

/// Every command this crate defines.
pub fn ops() -> Vec<OpSpec> {
    let mut v = js::ops();
    v.extend(three_d::ops());
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

/// usvg options for untrusted SVG: no external files, optional system fonts for `<text>`.
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
        opt.fontdb_mut().load_system_fonts();
    }
    opt
}

/// Rasterize SVG text at its own size, or scaled to fit `size`.
pub(crate) fn rasterize_svg(svg: &str, size: Option<(u32, u32)>) -> Result<tiny_skia::Pixmap> {
    let opt = svg_options(svg.contains("<text"));
    let tree = usvg::Tree::from_str(svg, &opt)
        .map_err(|e| Error::new("PX3001", format!("the script produced invalid SVG: {e}")))?;
    let s = tree.size();
    let (w, h) = match size {
        Some(wh) => wh,
        None => (s.width().ceil().max(1.0) as u32, s.height().ceil().max(1.0) as u32),
    };
    let mut pm = tiny_skia::Pixmap::new(w, h)
        .ok_or_else(|| Error::new("PX1104", format!("cannot allocate a {w}x{h} raster")))?;
    let ts = tiny_skia::Transform::from_scale(w as f32 / s.width(), h as f32 / s.height());
    resvg::render(&tree, ts, &mut pm.as_mut());
    Ok(pm)
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
