//! The document model (PRD §7.3): a tree of layers, a named-mask library, the active
//! selection, guides, a grid, template slots, and the recipe (the op history that a
//! sidecar embeds and `replay` re-executes).
//!
//! Layers are stored bottom → top in `layers` (Photoshop's panel lists them top first).
//! Raster layers hold pixels; text, shape, adjustment, and placed layers hold their
//! parameters and are rendered live by the compositor (`compose.rs`).

use crate::blend::BlendMode;
use crate::error::{Error, Result};
use crate::image::{Image, Mask};
use crate::units::RectI;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::collections::BTreeMap;
use std::sync::Arc;

/// Version of the `.pxo` container and `doc.json` layout.
pub const PXO_FORMAT_VERSION: u32 = 1;

impl Default for Image {
    fn default() -> Self {
        Image::new(0, 0)
    }
}

impl Default for Mask {
    fn default() -> Self {
        Mask::new(0, 0)
    }
}

/// A non-destructive per-layer mask (Photoshop semantics), in canvas coordinates.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct LayerMask {
    #[serde(skip)]
    pub mask: Mask,
    pub enabled: bool,
    /// 0..100: the maximum strength of the mask (100 = full).
    pub density: f32,
    /// Feather radius in px applied when rendering (0 = none).
    #[serde(default)]
    pub feather: f32,
}

impl LayerMask {
    pub fn new(mask: Mask) -> LayerMask {
        LayerMask { mask, enabled: true, density: 100.0, feather: 0.0 }
    }

    /// Effective coverage at a canvas pixel: `1 - density·(1 - m)`.
    #[inline]
    pub fn value(&self, m: f32) -> f32 {
        1.0 - (self.density / 100.0) * (1.0 - m)
    }
}

/// One entry of a layer's style stack (drop shadow, stroke, ...). Interpreted by the fx
/// renderer installed by `pixelox-fx`.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FxSpec {
    pub kind: String,
    #[serde(default = "yes")]
    pub enabled: bool,
    #[serde(default)]
    pub params: Map<String, Value>,
}

fn yes() -> bool {
    true
}

/// Layer contents by kind.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum LayerKind {
    /// Pixels, placed at the layer's (`x`, `y`) offset.
    Raster {
        #[serde(skip)]
        image: Image,
    },
    /// Live type: `spec` is owned by `pixelox-text`.
    Text { spec: Value },
    /// Live vector shape: `spec` is owned by `pixelox-vector`.
    Shape { spec: Value },
    /// An adjustment applied to the composite below (`adjust ... --as-layer`).
    Adjustment {
        op: String,
        #[serde(default)]
        params: Map<String, Value>,
    },
    /// A group of layers; `pass-through` blending by default.
    Group { children: Vec<Layer> },
    /// A placed file (smart-object analog): an embedded or linked source drawn into `rect`.
    Placed {
        /// Asset key (`sha256` of the source bytes).
        asset: String,
        /// The path it was placed from (as given).
        path: String,
        linked: bool,
        /// Destination `[x, y, w, h]` in canvas pixels.
        rect: [f64; 4],
        /// Rotation in degrees about the rect center.
        #[serde(default)]
        angle: f64,
        #[serde(skip)]
        source: Option<Arc<Image>>,
    },
}

impl LayerKind {
    pub fn name(&self) -> &'static str {
        match self {
            LayerKind::Raster { .. } => "raster",
            LayerKind::Text { .. } => "text",
            LayerKind::Shape { .. } => "shape",
            LayerKind::Adjustment { .. } => "adjustment",
            LayerKind::Group { .. } => "group",
            LayerKind::Placed { .. } => "placed",
        }
    }
}

/// A node of the layer tree.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Layer {
    pub id: String,
    pub name: String,
    #[serde(flatten)]
    pub content: LayerKind,
    pub visible: bool,
    pub locked: bool,
    /// 0..100.
    pub opacity: f32,
    pub blend: BlendMode,
    /// Offset of raster content (other kinds keep 0).
    #[serde(default)]
    pub x: i32,
    #[serde(default)]
    pub y: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mask: Option<LayerMask>,
    /// SVG path data of a vector mask (v0.3).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vector_mask: Option<String>,
    /// Clipping mask onto the nearest non-clipped layer below.
    #[serde(default)]
    pub clip: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fx: Vec<FxSpec>,
    /// Free-form properties (imported PSD text, notes).
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub props: Map<String, Value>,
}

impl Layer {
    fn base(id: String, name: String, content: LayerKind) -> Layer {
        let blend = if matches!(content, LayerKind::Group { .. }) { BlendMode::PassThrough } else { BlendMode::Normal };
        Layer {
            id,
            name,
            content,
            visible: true,
            locked: false,
            opacity: 100.0,
            blend,
            x: 0,
            y: 0,
            mask: None,
            vector_mask: None,
            clip: false,
            fx: Vec::new(),
            props: Map::new(),
        }
    }

    pub fn kind_name(&self) -> &'static str {
        self.content.name()
    }

    pub fn is_group(&self) -> bool {
        matches!(self.content, LayerKind::Group { .. })
    }

    pub fn children(&self) -> Option<&Vec<Layer>> {
        match &self.content {
            LayerKind::Group { children } => Some(children),
            _ => None,
        }
    }

    pub fn children_mut(&mut self) -> Option<&mut Vec<Layer>> {
        match &mut self.content {
            LayerKind::Group { children } => Some(children),
            _ => None,
        }
    }

    /// The raster pixels, or PX2303 naming the remedy.
    pub fn raster(&self) -> Result<&Image> {
        match &self.content {
            LayerKind::Raster { image } => Ok(image),
            other => Err(not_raster(&self.name, other.name())),
        }
    }

    pub fn raster_mut(&mut self) -> Result<&mut Image> {
        let name = self.name.clone();
        match &mut self.content {
            LayerKind::Raster { image } => Ok(image),
            other => Err(not_raster(&name, other.name())),
        }
    }

    /// Fail with PX2304 when locked.
    pub fn check_unlocked(&self) -> Result<()> {
        if self.locked {
            return Err(Error::new("PX2304", format!("layer '{}' is locked", self.name))
                .with_hint(format!("pixelox layer unlock <doc> --layer '{}' --in-place", self.name))
                .with_context("layer", &self.name));
        }
        Ok(())
    }
}

fn not_raster(name: &str, kind: &str) -> Error {
    Error::new("PX2303", format!("layer '{name}' is a {kind} layer; this operation needs pixels"))
        .with_hint(format!("pixelox layer rasterize <doc> --layer '{name}' --in-place"))
        .with_context("layer", name)
        .with_context("kind", kind)
}

/// A guide line.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Guide {
    /// `v` (vertical, at x = pos) or `h` (horizontal, at y = pos).
    pub orientation: String,
    pub pos: f64,
}

/// The document grid.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GridSpec {
    pub size: [f64; 2],
    pub visible: bool,
}

/// A template slot (`comp template init`).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Slot {
    pub name: String,
    /// `text`, `image`, or `color`.
    pub kind: String,
    pub required: bool,
    /// Layer the slot fills (defaults to the layer with the slot's name).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layer: Option<String>,
    /// Image fit: `cover`, `contain`, or `fill`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fit: Option<String>,
    /// Constraints: `max_lines`, `min_size` (auto-shrink), ...
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub constraints: Map<String, Value>,
}

/// Source bytes embedded in the document (placed files, imported fonts).
#[derive(Clone, Debug, Default)]
pub struct Asset {
    pub sha256: String,
    pub ext: String,
    pub bytes: Arc<Vec<u8>>,
}

/// A recorded input of a recipe step.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct InputRef {
    /// `source` for the document's source image; otherwise the param or purpose
    /// (`file`, `mask`, `cube`, ...).
    pub role: String,
    pub path: String,
    pub sha256: String,
    pub bytes: u64,
    /// The parameter holding this path, so `replay --input role=path` can substitute it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
}

/// One op of the recipe.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RecipeStep {
    /// Canonical dotted command id.
    pub op: String,
    #[serde(default)]
    pub params: Map<String, Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inputs: Vec<InputRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fonts: Vec<Value>,
}

/// Metadata carried from the source file.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct DocMeta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_format: Option<String>,
    /// Name of the source ICC profile, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icc_name: Option<String>,
    #[serde(skip)]
    pub icc: Option<Vec<u8>>,
    #[serde(skip)]
    pub exif: Option<Vec<u8>>,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
}

/// A pixelox document.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Document {
    pub format_version: u32,
    pub width: u32,
    pub height: u32,
    pub dpi: f32,
    /// Working color space: `srgb`, `display-p3`, or `adobe-rgb`.
    pub color_space: String,
    /// Storage depth: 8 or 16.
    pub depth: u8,
    /// Bottom → top.
    pub layers: Vec<Layer>,
    #[serde(skip)]
    pub selection: Option<Mask>,
    #[serde(skip)]
    pub masks: BTreeMap<String, Mask>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guides: Vec<Guide>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grid: Option<GridSpec>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub slots: Vec<Slot>,
    #[serde(skip)]
    pub recipe: Vec<RecipeStep>,
    #[serde(skip)]
    pub assets: BTreeMap<String, Asset>,
    #[serde(default)]
    pub meta: DocMeta,
    pub next_id: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<String>,
    /// Named history checkpoints: name → recipe length.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub checkpoints: BTreeMap<String, usize>,
}

/// Index path to a layer: indices into nested `layers` / `children` vectors.
pub type LayerPath = Vec<usize>;

impl Document {
    pub fn new(width: u32, height: u32) -> Document {
        Document {
            format_version: PXO_FORMAT_VERSION,
            width,
            height,
            dpi: 72.0,
            color_space: "srgb".into(),
            depth: 8,
            layers: Vec::new(),
            selection: None,
            masks: BTreeMap::new(),
            guides: Vec::new(),
            grid: None,
            slots: Vec::new(),
            recipe: Vec::new(),
            assets: BTreeMap::new(),
            meta: DocMeta::default(),
            next_id: 1,
            active: None,
            checkpoints: BTreeMap::new(),
        }
    }

    /// A one-layer document named `Background` holding `image` (how plain images open).
    pub fn from_image(image: Image) -> Document {
        let mut d = Document::new(image.width, image.height);
        let l = d.new_layer("Background", LayerKind::Raster { image });
        d.active = Some(l.id.clone());
        d.layers.push(l);
        d
    }

    pub fn canvas(&self) -> RectI {
        RectI::new(0, 0, self.width, self.height)
    }

    pub fn dpi(&self) -> f64 {
        self.dpi as f64
    }

    /// Next deterministic layer id (`L1`, `L2`, ...).
    pub fn next_layer_id(&mut self) -> String {
        let id = format!("L{}", self.next_id);
        self.next_id += 1;
        id
    }

    /// Build a layer with a fresh id and a unique name derived from `name`.
    pub fn new_layer(&mut self, name: &str, content: LayerKind) -> Layer {
        let id = self.next_layer_id();
        let name = self.unique_name(name);
        Layer::base(id, name, content)
    }

    /// `name` if unused, else `name 2`, `name 3`, ...
    pub fn unique_name(&self, name: &str) -> String {
        let base = if name.trim().is_empty() { "Layer" } else { name.trim() };
        if self.find_by_name(base).is_none() {
            return base.to_string();
        }
        let mut n = 2;
        loop {
            let c = format!("{base} {n}");
            if self.find_by_name(&c).is_none() {
                return c;
            }
            n += 1;
        }
    }

    /// Every layer with its path, in panel order (top first, depth-first).
    pub fn walk(&self) -> Vec<(LayerPath, &Layer)> {
        fn rec<'a>(layers: &'a [Layer], prefix: &[usize], out: &mut Vec<(LayerPath, &'a Layer)>) {
            for i in (0..layers.len()).rev() {
                let mut p = prefix.to_vec();
                p.push(i);
                out.push((p.clone(), &layers[i]));
                if let Some(ch) = layers[i].children() {
                    rec(ch, &p, out);
                }
            }
        }
        let mut out = Vec::new();
        rec(&self.layers, &[], &mut out);
        out
    }

    pub fn layer_count(&self) -> usize {
        self.walk().len()
    }

    pub fn find_by_name(&self, name: &str) -> Option<LayerPath> {
        self.walk().into_iter().find(|(_, l)| l.name == name).map(|(p, _)| p)
    }

    pub fn find_by_id(&self, id: &str) -> Option<LayerPath> {
        self.walk().into_iter().find(|(_, l)| l.id == id).map(|(p, _)| p)
    }

    /// Resolve a layer reference: unique name, id, `#N` (1-based from the top of the
    /// panel), `@top`, `@bottom`, `@active`.
    pub fn resolve(&self, r: &str) -> Result<LayerPath> {
        let r = r.trim();
        if self.layers.is_empty() {
            return Err(Error::new("PX2320", "the document has no layers"));
        }
        let found = match r {
            "@top" => Some(vec![self.layers.len() - 1]),
            "@bottom" => Some(vec![0]),
            "@active" => self.active.as_deref().and_then(|id| self.find_by_id(id)),
            _ if r.starts_with('#') && r[1..].parse::<usize>().is_ok() => {
                let n: usize = r[1..].parse().unwrap_or(0);
                if n == 0 {
                    None
                } else {
                    self.walk().into_iter().nth(n - 1).map(|(p, _)| p)
                }
            }
            _ => self.find_by_name(r).or_else(|| self.find_by_id(r)),
        };
        found.ok_or_else(|| {
            let names: Vec<String> = self.walk().iter().map(|(_, l)| l.name.clone()).collect();
            Error::no_layer(r).with_context("layers", names)
        })
    }

    /// Resolve `--layer`, defaulting to the only layer of a single-layer document
    /// (PX2302 when the reference is missing and the document has several layers).
    pub fn resolve_or_only(&self, r: Option<&str>) -> Result<LayerPath> {
        match r {
            Some(r) => self.resolve(r),
            None => {
                let all = self.walk();
                let leaves: Vec<&(LayerPath, &Layer)> = all.iter().filter(|(_, l)| !l.is_group()).collect();
                match (leaves.len(), all.len()) {
                    (0, 0) => Err(Error::new("PX2320", "the document has no layers")),
                    (1, _) => Ok(leaves[0].0.clone()),
                    _ => Err(Error::new(
                        "PX2302",
                        format!("the document has {} layers; say which one with --layer", all.len()),
                    )
                    .with_context("layers", all.iter().map(|(_, l)| l.name.clone()).collect::<Vec<_>>())),
                }
            }
        }
    }

    pub fn layer(&self, path: &[usize]) -> &Layer {
        let mut cur = &self.layers[path[0]];
        for &i in &path[1..] {
            cur = &cur.children().expect("path through a non-group")[i];
        }
        cur
    }

    pub fn layer_mut(&mut self, path: &[usize]) -> &mut Layer {
        let mut cur = &mut self.layers[path[0]];
        for &i in &path[1..] {
            cur = &mut cur.children_mut().expect("path through a non-group")[i];
        }
        cur
    }

    /// The sibling vector that contains the layer at `path`.
    pub fn siblings_mut(&mut self, path: &[usize]) -> &mut Vec<Layer> {
        if path.len() == 1 {
            return &mut self.layers;
        }
        self.layer_mut(&path[..path.len() - 1]).children_mut().expect("parent is a group")
    }

    pub fn siblings(&self, path: &[usize]) -> &Vec<Layer> {
        if path.len() == 1 {
            return &self.layers;
        }
        self.layer(&path[..path.len() - 1]).children().expect("parent is a group")
    }

    /// Remove and return the layer at `path`.
    pub fn remove(&mut self, path: &[usize]) -> Layer {
        let idx = *path.last().unwrap();
        let l = self.siblings_mut(path).remove(idx);
        if self.active.as_deref() == Some(l.id.as_str()) {
            self.active = self.layers.last().map(|t| t.id.clone());
        }
        l
    }

    /// Insert `layer` directly above the layer at `path` (or at the top when `None`), and
    /// make it active. Returns the new layer's path.
    pub fn insert_above(&mut self, path: Option<&[usize]>, layer: Layer) -> LayerPath {
        self.active = Some(layer.id.clone());
        match path {
            None => {
                self.layers.push(layer);
                vec![self.layers.len() - 1]
            }
            Some(p) => {
                let idx = *p.last().unwrap() + 1;
                self.siblings_mut(p).insert(idx, layer);
                let mut np = p.to_vec();
                *np.last_mut().unwrap() = idx;
                np
            }
        }
    }

    /// Fail with PX2305 when a different layer already has `name`.
    pub fn check_name_free(&self, name: &str, except: Option<&str>) -> Result<()> {
        if let Some(p) = self.find_by_name(name) {
            if Some(self.layer(&p).id.as_str()) != except {
                return Err(
                    Error::new("PX2305", format!("a layer named '{name}' already exists")).with_context("name", name)
                );
            }
        }
        Ok(())
    }

    /// A named mask, `@sel`, or PX2311/PX2312. (`@alpha:<layer>` and file paths are
    /// resolved by `OpCtx::resolve_mask`, which can render layers and read files.)
    pub fn named_mask(&self, name: &str) -> Result<Mask> {
        if name == "@sel" {
            return self.selection.clone().ok_or_else(|| {
                Error::new("PX2312", "the document has no active selection")
                    .with_hint("make one first, e.g. pixelox select rect <doc> --rect X,Y,WxH --in-place")
            });
        }
        self.masks.get(name).cloned().ok_or_else(|| {
            Error::new("PX2311", format!("no mask named '{name}'"))
                .with_context("masks", self.masks.keys().cloned().collect::<Vec<_>>())
        })
    }

    /// Store a mask in the library (PX2310 when the name is taken and `replace` is false).
    pub fn put_mask(&mut self, name: &str, mask: Mask, replace: bool) -> Result<()> {
        if name.is_empty() || name.starts_with('@') {
            return Err(Error::usage(format!("'{name}' is not a valid mask name (names cannot start with @)")));
        }
        if !replace && self.masks.contains_key(name) {
            return Err(Error::new("PX2310", format!("a mask named '{name}' already exists")).with_hint(format!(
                "choose another name or remove it: pixelox mask rm <doc> --mask {name} --in-place"
            )));
        }
        self.masks.insert(name.to_string(), mask);
        Ok(())
    }

    /// Embed asset bytes; returns the key.
    pub fn add_asset(&mut self, bytes: Vec<u8>, ext: &str) -> String {
        let sha = crate::envelope::sha256_bytes(&bytes);
        self.assets.entry(sha.clone()).or_insert_with(|| Asset {
            sha256: sha.clone(),
            ext: ext.to_string(),
            bytes: Arc::new(bytes),
        });
        sha
    }

    /// Short summary for envelopes.
    pub fn summary(&self) -> Value {
        json!({
            "size": [self.width, self.height],
            "color": self.color_space,
            "depth": self.depth,
            "dpi": self.dpi,
            "layers": self.layer_count(),
            "masks": self.masks.keys().collect::<Vec<_>>(),
            "selection": self.selection.is_some(),
            "recipe_steps": self.recipe.len(),
        })
    }

    /// The layer tree as JSON (panel order, top first); `bounds` needs rendering, so it is
    /// filled by the compositor when available.
    pub fn layer_json(&self, l: &Layer, depth: usize) -> Value {
        let mut v = json!({
            "id": l.id,
            "name": l.name,
            "kind": l.kind_name(),
            "visible": l.visible,
            "locked": l.locked,
            "opacity": l.opacity,
            "blend": l.blend.name(),
            "clip": l.clip,
            "depth": depth,
            "mask": l.mask.as_ref().map(|m| json!({"enabled": m.enabled, "density": m.density, "feather": m.feather, "coverage": round3(m.mask.coverage())})),
            "fx": l.fx.iter().map(|f| f.kind.clone()).collect::<Vec<_>>(),
            "bounds": crate::compose::layer_bounds(self, l).ok().flatten().map(|r| r.to_json()),
        });
        match &l.content {
            LayerKind::Raster { image } => {
                v["size"] = json!([image.width, image.height]);
                v["offset"] = json!([l.x, l.y]);
            }
            LayerKind::Text { spec } => {
                v["text"] = spec.get("content").cloned().unwrap_or(Value::Null);
            }
            LayerKind::Adjustment { op, params } => {
                v["adjustment"] = json!({"op": op, "params": params});
            }
            LayerKind::Placed { path, linked, rect, .. } => {
                v["placed"] = json!({"path": path, "linked": linked, "rect": rect});
            }
            LayerKind::Group { children } => {
                v["children"] = Value::Array(children.iter().rev().map(|c| self.layer_json(c, depth + 1)).collect());
            }
            LayerKind::Shape { .. } => {}
        }
        if !l.props.is_empty() {
            v["props"] = Value::Object(l.props.clone());
        }
        v
    }

    /// Round raster pixels and masks to the storage depth (8 or 16 bits), exactly as a
    /// `.pxo` save and reload would. The executor calls this after every document step, so
    /// a recipe produces the same bytes whether its intermediate documents were saved
    /// (CLI steps) or kept in memory (pipelines, replay).
    pub fn quantize_to_storage(&mut self) {
        let levels = if self.depth == 16 { 65535.0f32 } else { 255.0f32 };
        let q = |v: &mut f32| *v = (v.clamp(0.0, 1.0) * levels).round() / levels;
        fn walk(layers: &mut [Layer], q: &(dyn Fn(&mut f32) + Sync)) {
            use rayon::prelude::*;
            for l in layers.iter_mut() {
                if let LayerKind::Raster { image } = &mut l.content {
                    image.data.par_iter_mut().for_each(q);
                }
                if let Some(lm) = &mut l.mask {
                    lm.mask.data.par_iter_mut().for_each(q);
                }
                if let Some(ch) = l.children_mut() {
                    walk(ch, q);
                }
            }
        }
        walk(&mut self.layers, &q);
        use rayon::prelude::*;
        if let Some(sel) = &mut self.selection {
            sel.data.par_iter_mut().for_each(q);
        }
        for m in self.masks.values_mut() {
            m.data.par_iter_mut().for_each(q);
        }
    }

    pub fn tree_json(&self) -> Value {
        Value::Array(self.layers.iter().rev().map(|l| self.layer_json(l, 0)).collect())
    }
}

pub fn round3(v: f64) -> f64 {
    (v * 1000.0).round() / 1000.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Rgba;

    fn doc3() -> Document {
        let mut d = Document::from_image(Image::filled(10, 10, Rgba::WHITE));
        let a = d.new_layer("A", LayerKind::Raster { image: Image::new(10, 10) });
        d.insert_above(None, a);
        let mut g = d.new_layer("Group", LayerKind::Group { children: vec![] });
        let inner = d.new_layer("Inner", LayerKind::Raster { image: Image::new(10, 10) });
        g.children_mut().unwrap().push(inner);
        d.insert_above(None, g);
        d
    }

    #[test]
    fn addressing() {
        let d = doc3();
        assert_eq!(d.layer(&d.resolve("@top").unwrap()).name, "Group");
        assert_eq!(d.layer(&d.resolve("@bottom").unwrap()).name, "Background");
        assert_eq!(d.layer(&d.resolve("#1").unwrap()).name, "Group");
        assert_eq!(d.layer(&d.resolve("#2").unwrap()).name, "Inner");
        assert_eq!(d.layer(&d.resolve("#3").unwrap()).name, "A");
        assert_eq!(d.layer(&d.resolve("Inner").unwrap()).name, "Inner");
        assert_eq!(d.resolve("nope").unwrap_err().code, "PX2301");
        assert_eq!(d.resolve_or_only(None).unwrap_err().code, "PX2302");
        let single = Document::from_image(Image::new(2, 2));
        assert_eq!(single.resolve_or_only(None).unwrap(), vec![0]);
    }

    #[test]
    fn unique_names_and_serde() {
        let mut d = doc3();
        assert_eq!(d.unique_name("A"), "A 2");
        assert_eq!(d.unique_name("Fresh"), "Fresh");
        let text = serde_json::to_string(&d).unwrap();
        let back: Document = serde_json::from_str(&text).unwrap();
        assert_eq!(back.layer_count(), 4);
        assert_eq!(back.layers[2].blend, BlendMode::PassThrough);
        let removed = d.remove(&d.resolve("A").unwrap());
        assert_eq!(removed.name, "A");
        assert_eq!(d.layer_count(), 3);
    }

    #[test]
    fn masks_library() {
        let mut d = doc3();
        d.put_mask("m", Mask::filled(10, 10, 1.0), false).unwrap();
        assert_eq!(d.put_mask("m", Mask::new(10, 10), false).unwrap_err().code, "PX2310");
        assert!(d.named_mask("m").is_ok());
        assert_eq!(d.named_mask("@sel").unwrap_err().code, "PX2312");
        assert_eq!(d.named_mask("x").unwrap_err().code, "PX2311");
    }
}
