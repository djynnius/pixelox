//! Helpers shared by the `layer` and `comp` commands: layer addressing, name rules,
//! insertion, lock checks, and fitting a source image into a box.

use pixelox_core::doc::LayerPath;
use pixelox_core::prelude::*;
use std::path::Path;

/// Resolve `--layer` (the only layer of a single-layer document when omitted).
pub fn layer_path(a: &Args, doc: &Document) -> Result<LayerPath> {
    doc.resolve_or_only(a.raw("layer"))
}

/// The current path of a layer id.
pub fn path_of(doc: &Document, id: &str) -> Result<LayerPath> {
    doc.find_by_id(id).ok_or_else(|| Error::internal(format!("layer {id} is no longer in the document")))
}

/// Resolve layer references to ids (deduplicated, in the given order).
pub fn resolve_ids(doc: &Document, refs: &[String]) -> Result<Vec<String>> {
    let mut ids: Vec<String> = Vec::new();
    for r in refs {
        let p = doc.resolve(r)?;
        let id = doc.layer(&p).id.clone();
        if !ids.contains(&id) {
            ids.push(id);
        }
    }
    Ok(ids)
}

/// `--layers` resolved to ids; PX1003 when empty.
pub fn layers_arg(a: &Args, doc: &Document) -> Result<Vec<String>> {
    let refs = a.list("layers");
    if refs.is_empty() {
        return Err(Error::new("PX1003", "--layers needs at least one layer reference").with_context("param", "layers"));
    }
    resolve_ids(doc, &refs)
}

/// Check a name given for a layer: not empty, not addressing syntax, no commas.
pub fn validate_name(name: &str) -> Result<String> {
    let n = name.trim();
    let looks_like_index = n.len() > 1 && n.starts_with('#') && n[1..].chars().all(|c| c.is_ascii_digit());
    if n.is_empty() || n.starts_with('@') || looks_like_index || n.contains(',') {
        return Err(Error::new(
            "PX1010",
            format!("'{n}' is not a usable layer name: names cannot be empty, start with @, look like #N, or contain commas"),
        )
        .with_context("name", n));
    }
    Ok(n.to_string())
}

/// An explicit name for a layer: valid and not used by any layer other than `except` (ids).
pub fn fresh_name(doc: &Document, name: &str, except: &[String]) -> Result<String> {
    let n = validate_name(name)?;
    if let Some(p) = doc.find_by_name(&n) {
        if !except.contains(&doc.layer(&p).id) {
            return Err(Error::new("PX2305", format!("a layer named '{n}' already exists"))
                .with_hint("choose another name; `pixelox layer list <doc>` shows the names in use")
                .with_context("name", &n));
        }
    }
    Ok(n)
}

/// `name` if neither the document nor `taken` uses it, else `name 2`, `name 3`, ...
pub fn unique_avoiding(doc: &Document, name: &str, taken: &[String]) -> String {
    let base = if name.trim().is_empty() { "Layer" } else { name.trim() };
    let free = |c: &str| doc.find_by_name(c).is_none() && !taken.iter().any(|t| t == c);
    if free(base) {
        return base.to_string();
    }
    let mut n = 2;
    loop {
        let c = format!("{base} {n}");
        if free(&c) {
            return c;
        }
        n += 1;
    }
}

/// Insert a new layer above the active layer (or at the top) and make it active.
pub fn insert_new(doc: &mut Document, layer: Layer) -> LayerPath {
    let active = doc.active.clone().and_then(|id| doc.find_by_id(&id));
    doc.insert_above(active.as_deref(), layer)
}

/// The child vector of the group at `parent` (the top-level stack when `parent` is empty).
pub fn children_of<'a>(doc: &'a mut Document, parent: &[usize]) -> &'a mut Vec<Layer> {
    if parent.is_empty() {
        &mut doc.layers
    } else {
        doc.layer_mut(parent).children_mut().expect("parent is a group")
    }
}

/// Name of the group containing the layer at `path` (null at the top level).
pub fn parent_name(doc: &Document, path: &[usize]) -> Value {
    if path.len() <= 1 {
        Value::Null
    } else {
        json!(doc.layer(&path[..path.len() - 1]).name)
    }
}

/// PX2304 when the layer or anything inside it is locked.
pub fn check_tree_unlocked(l: &Layer) -> Result<()> {
    l.check_unlocked()?;
    if let Some(ch) = l.children() {
        for c in ch {
            check_tree_unlocked(c)?;
        }
    }
    Ok(())
}

pub fn round3(v: f64) -> f64 {
    (v * 1000.0).round() / 1000.0
}

/// PX1104 when a new layer would exceed the pixel limit.
pub fn check_pixels(ctx: &OpCtx, w: u32, h: u32) -> Result<()> {
    let n = w as u64 * h as u64;
    let limit = ctx.globals.open.limit_pixels;
    if n > limit {
        return Err(Error::new("PX1104", format!("{w}x{h} is {n} pixels, over the limit of {limit}"))
            .with_context("size", [w, h])
            .with_context("limit_pixels", limit));
    }
    Ok(())
}

/// Fit a `sw`×`sh` source into the box (`bx`, `by`, `bw`, `bh`) per `fit` (`cover`,
/// `contain`, or `fill`), centered, in whole pixels: `[x, y, w, h]`.
pub fn fit_rect(sw: f64, sh: f64, bx: f64, by: f64, bw: f64, bh: f64, fit: &str) -> [f64; 4] {
    let (sw, sh) = (sw.max(1.0), sh.max(1.0));
    let (w, h) = match fit {
        "fill" => (bw, bh),
        "contain" => {
            let s = (bw / sw).min(bh / sh);
            (sw * s, sh * s)
        }
        _ => {
            let s = (bw / sw).max(bh / sh);
            (sw * s, sh * s)
        }
    };
    let (w, h) = (w.round().max(1.0), h.round().max(1.0));
    [(bx + (bw - w) / 2.0).round(), (by + (bh - h) / 2.0).round(), w, h]
}

/// Resample `src` to exactly `bw`×`bh` pixels per `fit`: `cover` crops the overflow,
/// `contain` pads with transparency, `fill` stretches.
pub fn fit_image(src: &Image, bw: u32, bh: u32, fit: &str) -> Image {
    let r = fit_rect(src.width as f64, src.height as f64, 0.0, 0.0, bw as f64, bh as f64, fit);
    let (w, h) = (r[2] as u32, r[3] as u32);
    let filter = kernels::Filter::auto(src.width.max(src.height), w.max(h));
    let scaled = if (w, h) == (src.width, src.height) { src.clone() } else { kernels::resize(src, w, h, filter) };
    if (w, h) == (bw, bh) {
        return scaled;
    }
    Rendered { image: scaled, x: r[0] as i32, y: r[1] as i32 }.to_canvas(bw, bh)
}

/// Read and decode an image file through the IO layer, recording it as an input of this
/// op's recipe step. Returns the pixels, the bytes, and the lower-case extension.
pub fn read_image(ctx: &mut OpCtx, file: &str, role: &str, param: Option<&str>) -> Result<(Image, Vec<u8>, String)> {
    let path = Path::new(file);
    let bytes = ctx.read_aux(path, role, param)?;
    let ext = path.extension().map(|e| e.to_string_lossy().to_ascii_lowercase()).unwrap_or_else(|| "bin".into());
    let (img, _) = ctx.io.decode(&bytes, Some(&ext), &ctx.globals.open)?;
    if img.is_empty() {
        return Err(Error::new("PX1103", format!("{file} decoded to an empty image")).with_context("path", file));
    }
    Ok((img, bytes, ext))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pixelox_core::testing;

    #[test]
    fn names_and_fits() {
        assert!(validate_name("@top").is_err());
        assert!(validate_name("#3").is_err());
        assert!(validate_name("a,b").is_err());
        assert_eq!(validate_name(" Sky ").unwrap(), "Sky");
        assert_eq!(validate_name("#hashtag").unwrap(), "#hashtag");
        let doc = testing::doc_layers(4, 4, vec![("A", Image::new(4, 4))]);
        assert_eq!(fresh_name(&doc, "A", &[]).unwrap_err().code, "PX2305");
        assert_eq!(unique_avoiding(&doc, "A", &["A 2".into()]), "A 3");
        assert_eq!(fit_rect(200.0, 100.0, 0.0, 0.0, 100.0, 100.0, "contain"), [0.0, 25.0, 100.0, 50.0]);
        assert_eq!(fit_rect(200.0, 100.0, 0.0, 0.0, 100.0, 100.0, "cover"), [-50.0, 0.0, 200.0, 100.0]);
        assert_eq!(fit_rect(200.0, 100.0, 10.0, 10.0, 100.0, 100.0, "fill"), [10.0, 10.0, 100.0, 100.0]);
        let img = fit_image(&Image::filled(20, 10, Rgba::WHITE), 10, 10, "contain");
        assert_eq!((img.width, img.height), (10, 10));
        assert_eq!(img.get(5, 0)[3], 0.0);
        assert!(img.get(5, 5)[3] > 0.99);
        let img = fit_image(&Image::filled(20, 10, Rgba::WHITE), 10, 10, "cover");
        assert!(img.get(0, 0)[3] > 0.99);
    }
}
