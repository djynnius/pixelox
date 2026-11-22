//! Native liquid generators: `render splash` (a crown or a thrown sheet of liquid with
//! flying drops) and `render droplets` (condensation beads clipped onto a product). Both
//! add a new raster layer. Shapes are metaball fields of tapered strands and drops
//! (`field.rs`) lit as glossy liquid; every random choice comes from the op's seed.

mod droplets;
mod field;
mod splash;

use pixelox_core::doc::LayerPath;
use pixelox_core::prelude::*;
use rand_chacha::ChaCha12Rng;
use rand_core::RngCore;

pub use droplets::ops as droplet_ops;
pub use splash::ops as splash_ops;

/// Every liquid command.
pub fn ops() -> Vec<OpSpec> {
    let mut v = splash_ops();
    v.extend(droplet_ops());
    v
}

/// The liquids `--liquid` names.
pub(crate) const LIQUIDS: &[&str] = &["water", "milk", "juice", "cola", "coffee", "wine", "oil"];

/// The look of a named liquid, optionally recolored.
pub(crate) fn liquid_look(name: &str, color: Option<Rgba>, shine: Rgba, opacity: f32) -> field::Look {
    let hex = |s: &str| color::parse_color(s).map(|c| [c.r, c.g, c.b]).unwrap_or([1.0; 3]);
    // (color, deep, alpha at the center, alpha at the rim, opaque)
    let (c, d, ac, ar, op) = match name {
        "milk" => ("#f8f6f1", "#cfc8bf", 1.0, 1.0, 1.0),
        "juice" => ("#ffa22e", "#c2410c", 0.72, 0.96, 0.45),
        "cola" => ("#6e3616", "#1f0a03", 0.78, 0.98, 0.35),
        "coffee" => ("#7a4a26", "#2b1508", 0.9, 0.99, 0.75),
        "wine" => ("#8e1232", "#3a0412", 0.68, 0.96, 0.3),
        "oil" => ("#f2c445", "#9a6a00", 0.6, 0.93, 0.3),
        _ => ("#cfe9ff", "#2c557c", 0.16, 0.82, 0.0),
    };
    let (mut col, mut deep) = (hex(c), hex(d));
    if let Some(k) = color {
        col = [k.r, k.g, k.b];
        // a darker, more saturated version of the color for the rim
        deep = col.map(|v| (v * v * 0.55).min(1.0));
    }
    field::Look {
        color: col,
        deep,
        shine: [shine.r, shine.g, shine.b],
        alpha_center: ac,
        alpha_rim: ar,
        opaque: op,
        opacity,
    }
}

/// Uniform draws from the op's RNG.
pub(crate) struct Draw(pub ChaCha12Rng);

impl Draw {
    /// Uniform in [0, 1).
    pub fn u(&mut self) -> f64 {
        (self.0.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
    /// Uniform in [lo, hi).
    pub fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.u()
    }
}

/// The placement of a new liquid layer: directly above `--layer` when given (the layer it
/// belongs to), else where `--above/--below/--into` say (on top by default). PX1005 when
/// both `--layer` and a placement flag are given.
pub(crate) fn placement(a: &Args, doc: &Document) -> Result<pixelox_core::stack::Placement> {
    use pixelox_core::stack::{self, Placement};
    Ok(match (stack::from_args(a, doc)?, a.raw("layer")) {
        (Some(_), Some(_)) => {
            return Err(Error::new(
                "PX1005",
                "--layer and --above/--below/--into both place the new layer; give only one",
            )
            .with_hint("use --above <layer> (the same as --layer) or --below/--into"))
        }
        (Some(p), None) => p,
        (None, Some(r)) => Placement::Above(doc.resolve(r)?),
        (None, None) => Placement::Top,
    })
}

/// Insert `image` at (`x`, `y`) as a new normal raster layer at [`placement`]; returns its
/// path.
pub(crate) fn add_layer(a: &Args, doc: &mut Document, r: Rendered, default: &str) -> Result<LayerPath> {
    let at = placement(a, doc)?;
    let name = match a.str("name").map(|n| n.trim().to_string()).filter(|n| !n.is_empty()) {
        Some(n) => {
            doc.check_name_free(&n, None)?;
            n
        }
        None => default.to_string(),
    };
    let (image, x, y) = match r.image.alpha_bbox(0.0) {
        Some(b) if (b.w, b.h) != (r.image.width, r.image.height) => (r.image.crop(b), r.x + b.x, r.y + b.y),
        Some(_) => (r.image, r.x, r.y),
        None => (Image::new(1, 1), r.x, r.y),
    };
    let mut layer = doc.new_layer(&name, LayerKind::Raster { image });
    layer.x = x;
    layer.y = y;
    Ok(pixelox_core::stack::insert_at(doc, layer, &at))
}

/// Round to one decimal for results.
pub(crate) fn r1(v: f64) -> f64 {
    (v * 10.0).round() / 10.0
}
