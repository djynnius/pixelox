//! Test helpers for op crates: synthetic images and documents, and runners that execute
//! an op spec on an in-memory document without touching the filesystem.
//!
//! ```ignore
//! let spec = find(&ops(), "filter gaussian");
//! let mut doc = testing::doc_from(testing::gradient(64, 64));
//! testing::run(&spec, &[("sigma", "3")], &mut doc).unwrap();
//! ```

use crate::color::Rgba;
use crate::ctx::{Globals, NoIo, NoProgress, OpCtx};
use crate::doc::{Document, LayerKind};
use crate::error::Result;
use crate::image::{Image, Mask};
use crate::op::{Args, OpSpec, Runner};
use serde_json::Value;
use std::path::PathBuf;

/// Horizontal red ramp, vertical green ramp, constant blue 0.5, opaque.
pub fn gradient(w: u32, h: u32) -> Image {
    Image::from_fn(w, h, |x, y| [x as f32 / (w.max(2) - 1) as f32, y as f32 / (h.max(2) - 1) as f32, 0.5, 1.0])
}

/// 8 px black/white checkerboard, opaque.
pub fn checker(w: u32, h: u32) -> Image {
    Image::from_fn(w, h, |x, y| if (x / 8 + y / 8) % 2 == 0 { [1.0, 1.0, 1.0, 1.0] } else { [0.0, 0.0, 0.0, 1.0] })
}

pub fn solid(w: u32, h: u32, c: Rgba) -> Image {
    Image::filled(w, h, c)
}

/// A one-layer document (`Background`).
pub fn doc_from(img: Image) -> Document {
    Document::from_image(img)
}

/// A document with the given raster layers, bottom first.
pub fn doc_layers(w: u32, h: u32, layers: Vec<(&str, Image)>) -> Document {
    let mut d = Document::new(w, h);
    for (name, img) in layers {
        let l = d.new_layer(name, LayerKind::Raster { image: img });
        d.insert_above(None, l);
    }
    d
}

/// Left half selected.
pub fn half_mask(w: u32, h: u32) -> Mask {
    Mask::from_fn(w, h, |x, _| if x < w / 2 { 1.0 } else { 0.0 })
}

pub fn find(ops: &[OpSpec], name: &str) -> OpSpec {
    ops.iter().find(|o| o.name == name).unwrap_or_else(|| panic!("no op '{name}'")).clone()
}

pub fn args(spec: &OpSpec, pairs: &[(&str, &str)]) -> Result<Args> {
    Args::build(spec, pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect())
}

/// Run a doc op on `doc` in memory; returns its result value.
pub fn run(spec: &OpSpec, pairs: &[(&str, &str)], doc: &mut Document) -> Result<Value> {
    let (v, _) = run_with_warnings(spec, pairs, doc)?;
    Ok(v)
}

/// Like [`run`], also returning the warning codes raised.
pub fn run_with_warnings(spec: &OpSpec, pairs: &[(&str, &str)], doc: &mut Document) -> Result<(Value, Vec<String>)> {
    let a = args(spec, pairs)?;
    let mut ctx = OpCtx::new(spec.name, vec![], Globals::default(), &NoIo, &NoProgress);
    let v = match spec.runner {
        Runner::Doc(f) => f(&a, doc, &mut ctx)?,
        _ => panic!("{} is not a doc op", spec.name),
    };
    Ok((v, ctx.warnings.iter().map(|w| w.code.clone()).collect()))
}

/// Run a source op (`new`) and return its document.
pub fn run_source(spec: &OpSpec, pairs: &[(&str, &str)]) -> Result<Document> {
    let a = args(spec, pairs)?;
    let mut ctx = OpCtx::new(spec.name, vec![], Globals::default(), &NoIo, &NoProgress);
    match spec.runner {
        Runner::Source(f) => f(&a, &mut ctx),
        _ => panic!("{} is not a source op", spec.name),
    }
}

/// Run a custom op with the given input files (no IO layer: ops that decode files need
/// a real one) and return its result value.
pub fn run_custom(spec: &OpSpec, pairs: &[(&str, &str)], inputs: Vec<PathBuf>) -> Result<Value> {
    let a = args(spec, pairs)?;
    let mut ctx = OpCtx::new(spec.name, inputs, Globals::default(), &NoIo, &NoProgress);
    match spec.runner {
        Runner::Custom(f) => f(&a, &mut ctx),
        _ => panic!("{} is not a custom op", spec.name),
    }
}

/// Max absolute per-component difference between two same-sized images.
pub fn max_diff(a: &Image, b: &Image) -> f32 {
    assert_eq!((a.width, a.height), (b.width, b.height), "image sizes differ");
    a.data.iter().zip(&b.data).map(|(x, y)| (x - y).abs()).fold(0.0, f32::max)
}

/// Mean of one channel (0 = r, 1 = g, 2 = b, 3 = a).
pub fn mean(img: &Image, ch: usize) -> f64 {
    img.data.chunks(4).map(|p| p[ch] as f64).sum::<f64>() / img.pixels().max(1) as f64
}

/// The raster pixels of a named layer.
pub fn raster<'a>(doc: &'a Document, name: &str) -> &'a Image {
    doc.layer(&doc.resolve(name).expect("layer")).raster().expect("raster layer")
}

/// A fresh temporary directory, removed on drop.
pub struct TempDir(pub PathBuf);

impl TempDir {
    pub fn new(tag: &str) -> TempDir {
        let n = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0);
        let p = std::env::temp_dir().join(format!("pixelox-test-{tag}-{}-{n}", std::process::id()));
        std::fs::create_dir_all(&p).expect("temp dir");
        TempDir(p)
    }
    pub fn path(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
