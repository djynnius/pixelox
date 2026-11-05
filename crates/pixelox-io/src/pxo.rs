//! The `.pxo` document container (PRD §7.12): a ZIP with a stored `mimetype` first entry,
//! `manifest.json`, `doc.json`, `recipe.json`, layer pixels, masks, content-addressed
//! assets, source metadata, and a preview. Writing is deterministic: fixed timestamps,
//! sorted entries, fixed compression settings.

use crate::codec::{self, EmbedMeta};
use pixelox_core::ctx::{EncodeOptions, OpenOptions};
use pixelox_core::doc::{Asset, Document, Layer, LayerKind, RecipeStep, PXO_FORMAT_VERSION};
use pixelox_core::envelope::{sha256_bytes, PIXELOX_VERSION};
use pixelox_core::{compose, kernels, Error, Image, Mask, Result};
use serde_json::{json, Map, Value};
use std::collections::BTreeMap;
use std::io::{Cursor, Read, Write};
use std::sync::Arc;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, DateTime, ZipArchive, ZipWriter};

pub const MIMETYPE: &str = "application/x-pixelox";

fn png_bytes(img: &Image, depth16: bool) -> Result<Vec<u8>> {
    let opts =
        EncodeOptions { format: "png".into(), bit_depth: Some(if depth16 { 16 } else { 8 }), ..Default::default() };
    Ok(codec::encode(img, &opts, &EmbedMeta::default())?.0)
}

fn mask_png(m: &Mask, depth16: bool) -> Result<Vec<u8>> {
    let img = Image { width: m.width, height: m.height, data: m.data.iter().flat_map(|&v| [v, v, v, 1.0]).collect() };
    png_bytes(&img, depth16)
}

fn png_to_mask(bytes: &[u8]) -> Result<Mask> {
    let (img, _) = codec::decode_raster(bytes, "png", &OpenOptions { limit_pixels: u64::MAX, ..Default::default() })?;
    Ok(Mask { width: img.width, height: img.height, data: img.data.chunks(4).map(|p| p[0]).collect() })
}

/// File-safe, reversible encoding of a mask name.
fn mask_file(name: &str) -> String {
    let safe: String =
        name.chars().map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '_' { c } else { '~' }).collect();
    format!("masks/{}-{}.png", safe, &sha256_bytes(name.as_bytes())[..8])
}

fn walk_layers<'a>(layers: &'a [Layer], out: &mut Vec<&'a Layer>) {
    for l in layers {
        out.push(l);
        if let Some(ch) = l.children() {
            walk_layers(ch, out);
        }
    }
}

fn walk_layers_mut(layers: &mut [Layer], f: &mut dyn FnMut(&mut Layer) -> Result<()>) -> Result<()> {
    for l in layers.iter_mut() {
        f(l)?;
        if let Some(ch) = l.children_mut() {
            walk_layers_mut(ch, f)?;
        }
    }
    Ok(())
}

/// Serialize a document to `.pxo` bytes.
pub fn write(doc: &Document) -> Result<Vec<u8>> {
    let depth16 = doc.depth == 16;
    let mut entries: BTreeMap<String, (Vec<u8>, bool)> = BTreeMap::new();
    let mut all = Vec::new();
    walk_layers(&doc.layers, &mut all);
    for l in &all {
        if let LayerKind::Raster { image } = &l.content {
            if !image.is_empty() {
                entries.insert(format!("layers/{}.png", l.id), (png_bytes(image, depth16)?, false));
            }
        }
        if let Some(lm) = &l.mask {
            entries.insert(format!("masks/layer-{}.png", l.id), (mask_png(&lm.mask, depth16)?, false));
        }
    }
    let mut mask_index = Map::new();
    for (name, m) in &doc.masks {
        let f = mask_file(name);
        entries.insert(f.clone(), (mask_png(m, depth16)?, false));
        mask_index.insert(name.clone(), json!(f));
    }
    if let Some(sel) = &doc.selection {
        entries.insert("masks/selection.png".into(), (mask_png(sel, depth16)?, false));
    }
    let mut asset_index = Map::new();
    for (key, a) in &doc.assets {
        let f = format!("assets/sha256-{}.{}", key, if a.ext.is_empty() { "bin" } else { &a.ext });
        entries.insert(f.clone(), (a.bytes.as_ref().clone(), false));
        asset_index.insert(key.clone(), json!({"file": f, "ext": a.ext}));
    }
    if let Some(ex) = &doc.meta.exif {
        entries.insert("meta/exif.bin".into(), (ex.clone(), true));
    }
    if let Some(icc) = &doc.meta.icc {
        entries.insert("meta/profile.icc".into(), (icc.clone(), true));
    }
    let doc_json = serde_json::to_vec_pretty(doc)?;
    entries.insert("doc.json".into(), (doc_json, true));
    entries.insert("recipe.json".into(), (serde_json::to_vec_pretty(&doc.recipe)?, true));
    entries.insert("masks/index.json".into(), (serde_json::to_vec_pretty(&Value::Object(mask_index))?, true));
    entries.insert("assets/index.json".into(), (serde_json::to_vec_pretty(&Value::Object(asset_index))?, true));
    // preview (best effort: needs every renderer the document uses)
    if let Ok(img) = compose::composite(doc) {
        let scale = 512.0 / img.width.max(img.height).max(1) as f64;
        let prev = if scale < 1.0 {
            let (w, h) = (
                ((img.width as f64 * scale).round() as u32).max(1),
                ((img.height as f64 * scale).round() as u32).max(1),
            );
            kernels::resize(&img, w, h, kernels::Filter::Lanczos3)
        } else {
            img
        };
        entries.insert("thumbs/preview.png".into(), (png_bytes(&prev, false)?, false));
    }
    let integrity: Map<String, Value> = entries.iter().map(|(k, (b, _))| (k.clone(), json!(sha256_bytes(b)))).collect();
    let manifest = json!({
        "format": MIMETYPE,
        "format_version": PXO_FORMAT_VERSION,
        "tool": "pixelox",
        "tool_version": PIXELOX_VERSION,
        "document": {"size": [doc.width, doc.height], "depth": doc.depth, "color": doc.color_space, "layers": doc.layer_count()},
        "integrity": integrity,
    });
    let mut cur = Cursor::new(Vec::new());
    {
        let mut z = ZipWriter::new(&mut cur);
        let stored = SimpleFileOptions::default()
            .compression_method(CompressionMethod::Stored)
            .last_modified_time(DateTime::default());
        let deflated = SimpleFileOptions::default()
            .compression_method(CompressionMethod::Deflated)
            .compression_level(Some(6))
            .last_modified_time(DateTime::default());
        let zerr = |e: zip::result::ZipError| Error::new("PX1107", format!("cannot write .pxo: {e}"));
        z.start_file("mimetype", stored).map_err(zerr)?;
        z.write_all(MIMETYPE.as_bytes())?;
        z.start_file("manifest.json", deflated).map_err(zerr)?;
        z.write_all(&serde_json::to_vec_pretty(&manifest)?)?;
        for (name, (bytes, compress)) in &entries {
            z.start_file(name.as_str(), if *compress { deflated } else { stored }).map_err(zerr)?;
            z.write_all(bytes)?;
        }
        z.finish().map_err(zerr)?;
    }
    Ok(cur.into_inner())
}

fn corrupt(msg: impl Into<String>) -> Error {
    Error::new("PX1210", msg)
}

/// Read a `.pxo` document. Placed layers get their sources decoded from assets.
pub fn read(bytes: &[u8]) -> Result<Document> {
    let mut z = ZipArchive::new(Cursor::new(bytes)).map_err(|e| corrupt(format!("not a .pxo container: {e}")))?;
    let mut get = |name: &str| -> Result<Option<Vec<u8>>> {
        match z.by_name(name) {
            Ok(mut f) => {
                let mut v = Vec::new();
                f.read_to_end(&mut v)?;
                Ok(Some(v))
            }
            Err(zip::result::ZipError::FileNotFound) => Ok(None),
            Err(e) => Err(corrupt(format!("cannot read {name}: {e}"))),
        }
    };
    let mime = get("mimetype")?.ok_or_else(|| corrupt("missing mimetype entry"))?;
    if mime != MIMETYPE.as_bytes() {
        return Err(corrupt("wrong mimetype: not a pixelox document"));
    }
    let manifest: Value =
        serde_json::from_slice(&get("manifest.json")?.ok_or_else(|| corrupt("missing manifest.json"))?)
            .map_err(|e| corrupt(format!("manifest.json: {e}")))?;
    let ver = manifest["format_version"].as_u64().unwrap_or(0) as u32;
    if ver > PXO_FORMAT_VERSION {
        return Err(Error::new(
            "PX1211",
            format!("this document uses .pxo format {ver}; this pixelox reads up to {PXO_FORMAT_VERSION}"),
        )
        .with_context("tool_version", manifest["tool_version"].clone()));
    }
    let mut doc: Document = serde_json::from_slice(&get("doc.json")?.ok_or_else(|| corrupt("missing doc.json"))?)
        .map_err(|e| corrupt(format!("doc.json: {e}")))?;
    if let Some(r) = get("recipe.json")? {
        let recipe: Vec<RecipeStep> = serde_json::from_slice(&r).map_err(|e| corrupt(format!("recipe.json: {e}")))?;
        doc.recipe = recipe;
    }
    // assets
    if let Some(idx) = get("assets/index.json")? {
        let idx: Map<String, Value> =
            serde_json::from_slice(&idx).map_err(|e| corrupt(format!("assets/index.json: {e}")))?;
        for (key, v) in idx {
            let file = v["file"].as_str().unwrap_or_default().to_string();
            let bytes = get(&file)?.ok_or_else(|| corrupt(format!("missing asset {file}")))?;
            if sha256_bytes(&bytes) != key {
                return Err(corrupt(format!("asset {file} does not match its hash")));
            }
            doc.assets.insert(
                key.clone(),
                Asset { sha256: key, ext: v["ext"].as_str().unwrap_or("").to_string(), bytes: Arc::new(bytes) },
            );
        }
    }
    // pixels and layer masks
    let mut pending: Vec<(String, bool, bool)> = Vec::new(); // (id, raster?, has mask)
    {
        let mut all = Vec::new();
        walk_layers(&doc.layers, &mut all);
        for l in all {
            pending.push((l.id.clone(), matches!(l.content, LayerKind::Raster { .. }), l.mask.is_some()));
        }
    }
    let mut pixels: BTreeMap<String, Image> = BTreeMap::new();
    let mut lmasks: BTreeMap<String, Mask> = BTreeMap::new();
    let open = OpenOptions { limit_pixels: u64::MAX, auto_orient: false, ..Default::default() };
    for (id, raster, has_mask) in &pending {
        if *raster {
            if let Some(b) = get(&format!("layers/{id}.png"))? {
                pixels.insert(id.clone(), codec::decode_raster(&b, "png", &open)?.0);
            }
        }
        if *has_mask {
            let b =
                get(&format!("masks/layer-{id}.png"))?.ok_or_else(|| corrupt(format!("missing mask of layer {id}")))?;
            lmasks.insert(id.clone(), png_to_mask(&b)?);
        }
    }
    let assets = doc.assets.clone();
    walk_layers_mut(&mut doc.layers, &mut |l: &mut Layer| {
        let id = l.id.clone();
        if let LayerKind::Raster { image } = &mut l.content {
            if let Some(px) = pixels.remove(&id) {
                *image = px;
            }
        }
        if let LayerKind::Placed { asset, source, .. } = &mut l.content {
            if let Some(a) = assets.get(asset) {
                let img = crate::decode_bytes(&a.bytes, Some(&a.ext), &open)?.0;
                *source = Some(Arc::new(img));
            }
        }
        if let (Some(lm), Some(m)) = (l.mask.as_mut(), lmasks.remove(&id)) {
            lm.mask = m;
        }
        Ok(())
    })?;
    if let Some(idx) = get("masks/index.json")? {
        let idx: Map<String, Value> =
            serde_json::from_slice(&idx).map_err(|e| corrupt(format!("masks/index.json: {e}")))?;
        for (name, f) in idx {
            let file = f.as_str().unwrap_or_default().to_string();
            let b = get(&file)?.ok_or_else(|| corrupt(format!("missing mask {file}")))?;
            doc.masks.insert(name, png_to_mask(&b)?);
        }
    }
    if let Some(b) = get("masks/selection.png")? {
        doc.selection = Some(png_to_mask(&b)?);
    }
    doc.meta.exif = get("meta/exif.bin")?;
    doc.meta.icc = get("meta/profile.icc")?;
    Ok(doc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pixelox_core::doc::LayerMask;
    use pixelox_core::Rgba;

    fn sample_doc() -> Document {
        let mut d = Document::from_image(pixelox_core::testing::gradient(32, 16));
        let mut l = d.new_layer("Top", LayerKind::Raster { image: Image::filled(8, 8, Rgba::new(1.0, 0.0, 0.0, 0.5)) });
        l.x = 4;
        l.y = 2;
        l.mask = Some(LayerMask::new(pixelox_core::testing::half_mask(32, 16)));
        d.insert_above(None, l);
        let g = d.new_layer("G", LayerKind::Group { children: vec![] });
        d.insert_above(None, g);
        d.masks.insert("subject".into(), pixelox_core::testing::half_mask(32, 16));
        d.selection = Some(Mask::filled(32, 16, 0.5));
        d.recipe.push(RecipeStep { op: "open".into(), params: Map::new(), inputs: vec![], model: None, fonts: vec![] });
        d
    }

    #[test]
    fn round_trip_and_determinism() {
        let d = sample_doc();
        let a = write(&d).unwrap();
        let b = write(&d).unwrap();
        assert_eq!(a, b, "identical documents produce identical bytes");
        assert_eq!(&a[30..38], b"mimetype");
        let back = read(&a).unwrap();
        assert_eq!(back.layer_count(), 3);
        assert_eq!(back.recipe.len(), 1);
        assert!(back.masks.contains_key("subject"));
        assert!((back.selection.as_ref().unwrap().get(0, 0) - 0.5).abs() < 0.01);
        let top = back.layer(&back.resolve("Top").unwrap());
        assert_eq!((top.x, top.y), (4, 2));
        assert!((top.raster().unwrap().get(0, 0)[3] - 0.5).abs() < 0.01);
        assert!(top.mask.is_some());
        let c1 = compose::composite(&d).unwrap();
        let c2 = compose::composite(&back).unwrap();
        assert!(pixelox_core::testing::max_diff(&c1, &c2) < 2.0 / 255.0);
    }

    #[test]
    fn rejects_garbage_and_future_versions() {
        assert_eq!(read(b"not a zip").unwrap_err().code, "PX1210");
        let d = sample_doc();
        let bytes = write(&d).unwrap();
        // rewrite the manifest with a future version
        let mut z = ZipArchive::new(Cursor::new(&bytes)).unwrap();
        let mut out = Cursor::new(Vec::new());
        {
            let mut w = ZipWriter::new(&mut out);
            for i in 0..z.len() {
                let mut f = z.by_index(i).unwrap();
                let name = f.name().to_string();
                let mut v = Vec::new();
                f.read_to_end(&mut v).unwrap();
                if name == "manifest.json" {
                    let mut m: Value = serde_json::from_slice(&v).unwrap();
                    m["format_version"] = json!(99);
                    v = serde_json::to_vec(&m).unwrap();
                }
                w.start_file(name, SimpleFileOptions::default()).unwrap();
                w.write_all(&v).unwrap();
            }
            w.finish().unwrap();
        }
        assert_eq!(read(&out.into_inner()).unwrap_err().code, "PX1211");
    }
}
