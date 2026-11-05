//! SVG input: parse with usvg and rasterize with resvg (PRD §7.9). External file
//! references are disabled; embedded `data:` images still work.

use pixelox_core::{Error, Image, Result};
use std::sync::{Arc, OnceLock};

fn fontdb() -> Arc<usvg::fontdb::Database> {
    static DB: OnceLock<Arc<usvg::fontdb::Database>> = OnceLock::new();
    DB.get_or_init(|| {
        let mut db = usvg::fontdb::Database::new();
        db.load_system_fonts();
        Arc::new(db)
    })
    .clone()
}

/// Parse SVG bytes into a usvg tree with external references disabled.
pub fn parse(bytes: &[u8]) -> Result<usvg::Tree> {
    let mut opt = usvg::Options { fontdb: fontdb(), ..Default::default() };
    opt.image_href_resolver = usvg::ImageHrefResolver {
        resolve_data: usvg::ImageHrefResolver::default_data_resolver(),
        resolve_string: Box::new(|_, _| None),
    };
    usvg::Tree::from_data(bytes, &opt).map_err(|e| Error::new("PX1103", format!("cannot parse SVG: {e}")))
}

/// Intrinsic size in px (CSS px at 96 dpi), rounded up.
pub fn intrinsic_size(tree: &usvg::Tree) -> (u32, u32) {
    let s = tree.size();
    (s.width().ceil().max(1.0) as u32, s.height().ceil().max(1.0) as u32)
}

/// Rasterize to `w`×`h` (scaling the SVG's viewport to fit exactly).
pub fn rasterize(tree: &usvg::Tree, w: u32, h: u32) -> Result<Image> {
    let mut pm = resvg::tiny_skia::Pixmap::new(w.max(1), h.max(1))
        .ok_or_else(|| Error::new("PX1104", format!("cannot allocate a {w}x{h} raster")))?;
    let s = tree.size();
    let t = resvg::tiny_skia::Transform::from_scale(w as f32 / s.width(), h as f32 / s.height());
    resvg::render(tree, t, &mut pm.as_mut());
    let mut img = Image::from_rgba8(w.max(1), h.max(1), pm.data());
    img.unpremultiply();
    Ok(img)
}

/// Decode SVG bytes at intrinsic size.
pub fn decode(bytes: &[u8], limit_pixels: u64) -> Result<Image> {
    let tree = parse(bytes)?;
    let (w, h) = intrinsic_size(&tree);
    if w as u64 * h as u64 > limit_pixels {
        return Err(Error::new("PX1104", format!("SVG raster would be {w}x{h}, above the pixel limit")));
    }
    rasterize(&tree, w, h)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rasterizes_and_blocks_external_files() {
        let svg = br##"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="10"><rect x="0" y="0" width="10" height="10" fill="#ff0000"/><image href="/etc/passwd" width="5" height="5"/></svg>"##;
        let img = decode(svg, 1 << 30).unwrap();
        assert_eq!((img.width, img.height), (20, 10));
        assert_eq!(img.get(2, 2), [1.0, 0.0, 0.0, 1.0]);
        assert_eq!(img.get(15, 5)[3], 0.0);
        assert!(decode(b"<svg", 1 << 30).is_err());
    }
}
