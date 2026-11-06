//! The layer-style renderer installed into the compositor (`compose::set_fx_renderer`).
//!
//! It receives a layer's rendered content and returns it with every enabled effect
//! composited, on a canvas grown by the largest effect extent and trimmed back to the
//! visible pixels. Order (bottom → top), after Photoshop's style stack:
//!
//! 1. beneath the content: drop shadow, smoke, fire, outer glow, outside stroke;
//! 2. the content, with these clipped to its alpha (colors change, coverage does not):
//!    pattern, gradient, and color overlays, satin, inner glow, inner shadow, the inner
//!    part of bevel & emboss (highlight and shadow), and an inside stroke;
//! 3. on top: the outer part of bevel & emboss, a center stroke, and the part of the
//!    fire that licks over the content's edges (`--front`).
//!
//! Fire and smoke are computed by [`crate::flame`] from the content's alpha, anchored to
//! the content (not the canvas), so they move with the layer unchanged.
//!
//! Every effect uses its own blend mode and opacity. Within the layer, an effect over
//! transparent pixels composites like `normal` (the W3C rule); the layer as a whole is
//! then blended onto the backdrop with the layer's own mode.

use crate::flame::{self, Plume, Rise};
use pixelox_core::blend::composite_pixel;
use pixelox_core::color::{Interp, Stop};
use pixelox_core::doc::FxSpec;
use pixelox_core::kernels::{Edge, Filter};
use pixelox_core::op::parse_bool;
use pixelox_core::prelude::*;
use pixelox_core::units::{Length, PointSpec, Unit};
use rayon::prelude::*;
use std::sync::Arc;

/// Largest margin (px) an effect may add around the content.
const MAX_PAD: f64 = 4096.0;

/// The default gradient when an overlay or stroke asks for one without `--stops`.
pub(crate) const DEFAULT_STOPS: &str = "#000000@0,#ffffff@1";

/// The default fire ramp: black, deep red, orange, yellow, white.
pub(crate) const FIRE_STOPS: &str = "#000000@0,#6e0a00@0.22,#d2320a@0.45,#ff8c1a@0.66,#ffd84a@0.84,#fffbe6@1";

/// The default smoke color.
pub(crate) const SMOKE_COLOR: &str = "#8c8c8c";

// ---------------------------------------------------------------------------------------
// params

/// Lenient reader for an `FxSpec`'s params (strings from `args.to_json()`, but numbers
/// and booleans are accepted too, so hand-written or imported specs work).
pub(crate) struct Params<'a> {
    kind: &'a str,
    m: &'a Map<String, Value>,
}

impl<'a> Params<'a> {
    pub(crate) fn new(kind: &'a str, m: &'a Map<String, Value>) -> Params<'a> {
        Params { kind, m }
    }

    pub(crate) fn raw(&self, key: &str) -> Option<String> {
        value_str(self.m.get(key)?)
    }

    fn bad(&self, key: &str, v: &str, what: &str) -> Error {
        Error::new("PX1010", format!("fx {}: {key} '{v}' is not {what}", self.kind))
            .with_context("fx", self.kind)
            .with_context("param", key)
    }

    fn num(&self, key: &str, d: f64) -> Result<f64> {
        match self.raw(key) {
            None => Ok(d),
            Some(v) => parse_num(&v).ok_or_else(|| self.bad(key, &v, "a number")),
        }
    }

    fn color(&self, key: &str, d: Rgba) -> Result<[f32; 4]> {
        match self.raw(key) {
            None => Ok(d.to_array()),
            Some(v) => color::parse_color(&v).map(|c| c.to_array()),
        }
    }

    fn stops(&self, key: &str) -> Result<Option<Vec<Stop>>> {
        self.raw(key).map(|v| color::parse_stops(&v)).transpose()
    }

    fn blend(&self, key: &str, d: BlendMode) -> Result<BlendMode> {
        match self.raw(key) {
            None => Ok(d),
            Some(v) => Ok(match BlendMode::parse(&v)? {
                BlendMode::PassThrough => BlendMode::Normal,
                m => m,
            }),
        }
    }

    fn look(&self, key: &str, mode: BlendMode, opacity: f64) -> Result<Look> {
        Ok(Look {
            mode: self.blend(key, mode)?,
            opacity: (self.num("opacity", opacity)?.clamp(0.0, 100.0) / 100.0) as f32,
        })
    }

    fn flag(&self, key: &str) -> bool {
        self.raw(key).map(|v| parse_bool(&v).unwrap_or(false)).unwrap_or(false)
    }

    /// A lower-case word; `None` when absent, empty, or `auto`.
    fn word(&self, key: &str) -> Option<String> {
        self.raw(key).map(|v| v.trim().to_ascii_lowercase()).filter(|v| !v.is_empty() && v != "auto")
    }

    /// The stored noise pattern (`variant`, or `seed` in hand-written specs); 0 if absent.
    fn variant(&self) -> Result<u64> {
        match self.raw("variant").or_else(|| self.raw("seed")) {
            None => Ok(0),
            Some(v) => match parse_num(&v) {
                Some(n) if n >= 0.0 => Ok(n as u64),
                _ => Err(self.bad("variant", &v, "a whole number")),
            },
        }
    }

    fn angle(&self, key: &str, d: f64) -> Result<f64> {
        match self.raw(key) {
            None => Ok(d),
            Some(v) => units::parse_angle(&v),
        }
    }

    /// `DX,DY` in px (`%` of the canvas, physical units through the document dpi).
    fn offset(&self, key: &str, d: (f64, f64), doc: &Document) -> Result<(f64, f64)> {
        match self.raw(key) {
            None => Ok(d),
            Some(v) => {
                let p = PointSpec::parse(&v)?;
                Ok((p.x.to_px(doc.width as f64, doc.dpi()), p.y.to_px(doc.height as f64, doc.dpi())))
            }
        }
    }
}

pub(crate) fn value_str(v: &Value) -> Option<String> {
    match v {
        Value::Null => None,
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        other => Some(other.to_string()),
    }
}

pub(crate) fn parse_num(s: &str) -> Option<f64> {
    s.trim().trim_end_matches('%').trim().parse::<f64>().ok().filter(|v| v.is_finite())
}

/// Compact number text: integers without a fraction, otherwise up to 3 decimals.
pub(crate) fn fmt_num(v: f64) -> String {
    let r = (v * 1000.0).round() / 1000.0;
    if r.fract() == 0.0 {
        format!("{}", r as i64)
    } else {
        format!("{r}")
    }
}

/// A length back to text, keeping its unit.
pub(crate) fn fmt_len(l: Length) -> String {
    let unit = match l.unit {
        Unit::Px => "",
        Unit::Pct => "%",
        Unit::Pt => "pt",
        Unit::Mm => "mm",
        Unit::In => "in",
    };
    format!("{}{unit}", fmt_num(l.value))
}

// ---------------------------------------------------------------------------------------
// effects

#[derive(Clone, Copy, Debug)]
struct Look {
    mode: BlendMode,
    /// 0..1.
    opacity: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum GradStyle {
    Linear,
    Radial,
    Reflected,
}

/// What an overlay or a stroke is filled with.
enum Fill {
    Color([f32; 4]),
    Gradient {
        stops: Vec<Stop>,
        angle: f64,
        style: GradStyle,
        reverse: bool,
    },
    /// The tile (pre-shrunk when `--scale` < 100) and the enlargement still to apply when
    /// sampling it; anchored at the canvas origin.
    Pattern {
        tile: Arc<Image>,
        zoom: f64,
    },
}

/// A glow's color: solid, or a gradient from the glow's source outward.
enum Glow {
    Color([f32; 4]),
    Stops(Vec<Stop>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum BevelStyle {
    Inner,
    Outer,
    Emboss,
    Pillow,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum StrokePos {
    Inside,
    Center,
    Outside,
}

struct Bevel {
    style: BevelStyle,
    depth: f64,
    size: f64,
    soften: f64,
    angle: f64,
    altitude: f64,
    up: bool,
    highlight: [f32; 4],
    shadow: [f32; 4],
    hi_look: Look,
    sh_look: Look,
}

enum Effect {
    DropShadow {
        look: Look,
        color: [f32; 4],
        off: (f64, f64),
        blur: f64,
        spread: f64,
    },
    InnerShadow {
        look: Look,
        color: [f32; 4],
        off: (f64, f64),
        blur: f64,
        choke: f64,
    },
    OuterGlow {
        look: Look,
        glow: Glow,
        blur: f64,
        spread: f64,
    },
    InnerGlow {
        look: Look,
        glow: Glow,
        blur: f64,
        choke: f64,
        center: bool,
    },
    Bevel(Bevel),
    Satin {
        look: Look,
        color: [f32; 4],
        off: (f64, f64),
        blur: f64,
        invert: bool,
    },
    /// `rank`: 0 pattern, 1 gradient, 2 color (Photoshop stacks them in that order).
    Overlay {
        look: Look,
        fill: Fill,
        rank: u8,
    },
    Stroke {
        look: Look,
        fill: Fill,
        width: f64,
        pos: StrokePos,
    },
    Plume(PlumeFx),
}

/// Fire or smoke.
struct PlumeFx {
    look: Look,
    rise: Rise,
    /// Fire: the color ramp sampled at 256 heats (straight RGBA).
    ramp: Vec<[f32; 4]>,
    /// Smoke: the base color.
    color: [f32; 4],
    /// Fire: the share of the flames drawn over the content's edges (0..1).
    front: f32,
}

/// Split a blur radius into a hard part (dilate/erode radius, `pct` of `r`) and the
/// Gaussian σ of the rest (CSS semantics: σ = radius / 2).
fn blur_parts(r: f64, pct: f64) -> (f64, f64) {
    let r = r.max(0.0);
    let p = (pct / 100.0).clamp(0.0, 1.0);
    (r * p, r * (1.0 - p) / 2.0)
}

impl Effect {
    /// How far (px) the effect can reach beyond the content, or how much context an
    /// inner effect needs around it.
    fn extent(&self) -> f64 {
        let reach = |off: &(f64, f64)| off.0.abs().max(off.1.abs());
        match self {
            Effect::DropShadow { off, blur, spread, .. } => {
                let (m, s) = blur_parts(*blur, *spread);
                m + 3.0 * s + reach(off)
            }
            Effect::InnerShadow { off, blur, choke, .. } => {
                let (m, s) = blur_parts(*blur, *choke);
                m + 3.0 * s + reach(off)
            }
            Effect::OuterGlow { blur, spread, .. } => {
                let (m, s) = blur_parts(*blur, *spread);
                m + 3.0 * s
            }
            Effect::InnerGlow { blur, choke, .. } => {
                let (m, s) = blur_parts(*blur, *choke);
                m + 3.0 * s
            }
            Effect::Bevel(b) => 2.0 * b.size + 1.5 * b.soften,
            Effect::Satin { off, blur, .. } => reach(off) + 1.5 * blur,
            Effect::Overlay { .. } => 0.0,
            Effect::Stroke { width, .. } => *width,
            Effect::Plume(p) => p.rise.margins().into_iter().fold(0.0, f64::max),
        }
    }

    /// How far the effect reaches beyond the content on each side: left, top, right,
    /// bottom (px). Plumes reach far only toward their rise direction.
    fn margins(&self) -> [f64; 4] {
        match self {
            Effect::Plume(p) => p.rise.margins(),
            other => [other.extent(); 4],
        }
    }

    /// Stacking position (lower = drawn first).
    fn phase(&self) -> u8 {
        match self {
            Effect::DropShadow { .. } => 0,
            Effect::Plume(p) if p.rise.plume == Plume::Smoke => 1,
            Effect::Plume(_) => 2,
            Effect::OuterGlow { .. } => 3,
            Effect::Stroke { pos: StrokePos::Outside, .. } => 4,
            Effect::Overlay { rank, .. } => 10 + rank,
            Effect::Satin { .. } => 13,
            Effect::InnerGlow { .. } => 14,
            Effect::InnerShadow { .. } => 15,
            Effect::Bevel(_) => 16,
            Effect::Stroke { pos: StrokePos::Inside, .. } => 17,
            Effect::Stroke { pos: StrokePos::Center, .. } => 21,
        }
    }
}

fn parse_fill(p: &Params, doc: &Document, kind_key: &str) -> Result<(Fill, u8)> {
    let sub = p.word(kind_key).unwrap_or_else(|| {
        if p.raw("asset").is_some() {
            "pattern".into()
        } else if p.raw("stops").is_some() {
            "gradient".into()
        } else {
            "color".into()
        }
    });
    Ok(match sub.as_str() {
        "color" => (Fill::Color(p.color("color", Rgba::BLACK)?), 2),
        "gradient" => {
            let stops = match p.stops("stops")? {
                Some(s) => s,
                None => color::parse_stops(DEFAULT_STOPS)?,
            };
            let style = match p.word("style").as_deref() {
                None | Some("linear") => GradStyle::Linear,
                Some("radial") => GradStyle::Radial,
                Some("reflected") => GradStyle::Reflected,
                Some(other) => return Err(p.bad("style", other, "linear, radial, or reflected")),
            };
            (Fill::Gradient { stops, angle: p.angle("angle", 90.0)?, style, reverse: p.flag("reverse") }, 1)
        }
        "pattern" => {
            let key = p.raw("asset").ok_or_else(|| {
                Error::new("PX1010", format!("fx {}: a pattern fill needs an embedded tile", p.kind))
                    .with_hint(format!("re-run `pixelox fx {} <doc> --from tile.png --in-place`", p.kind))
            })?;
            let tile = compose::decode_asset(doc, &key)?;
            let scale = (p.num("scale", 100.0)? / 100.0).clamp(0.01, 100.0);
            let (tile, zoom) = scale_tile(tile, scale);
            (Fill::Pattern { tile, zoom }, 0)
        }
        other => return Err(p.bad(kind_key, other, "color, gradient, or pattern")),
    })
}

/// Shrink a tile ahead of time (filtered, so it does not alias); enlargements are left to
/// wrap-aware bilinear sampling so the tile's seams stay continuous.
fn scale_tile(tile: Arc<Image>, scale: f64) -> (Arc<Image>, f64) {
    if scale >= 1.0 - 1e-6 || tile.is_empty() {
        return (tile, scale.max(1.0));
    }
    let nw = (tile.width as f64 * scale).round().max(1.0) as u32;
    let nh = (tile.height as f64 * scale).round().max(1.0) as u32;
    (Arc::new(kernels::resize(&tile, nw, nh, Filter::Lanczos3)), 1.0)
}

fn parse_glow(p: &Params) -> Result<Glow> {
    Ok(match p.stops("stops")? {
        Some(s) => Glow::Stops(s),
        None => Glow::Color(p.color("color", Rgba::from_u8(0xff, 0xff, 0xbe, 0xff))?),
    })
}

/// Interpret one spec; `None` for kinds this renderer does not know (kept for
/// forward compatibility, e.g. styles imported from PSD). `size` is the content's
/// visible width and height (percent lengths refer to it).
fn parse(spec: &FxSpec, doc: &Document, size: (u32, u32)) -> Result<Option<Effect>> {
    let p = Params::new(&spec.kind, &spec.params);
    Ok(Some(match spec.kind.as_str() {
        "drop-shadow" => Effect::DropShadow {
            look: p.look("blend", BlendMode::Multiply, 75.0)?,
            color: p.color("color", Rgba::BLACK)?,
            off: p.offset("offset", (4.0, 4.0), doc)?,
            blur: p.num("blur", 8.0)?.max(0.0),
            spread: p.num("spread", 0.0)?,
        },
        "inner-shadow" => Effect::InnerShadow {
            look: p.look("blend", BlendMode::Multiply, 75.0)?,
            color: p.color("color", Rgba::BLACK)?,
            off: p.offset("offset", (4.0, 4.0), doc)?,
            blur: p.num("blur", 8.0)?.max(0.0),
            choke: p.num("choke", 0.0)?,
        },
        "outer-glow" => Effect::OuterGlow {
            look: p.look("blend", BlendMode::Screen, 75.0)?,
            glow: parse_glow(&p)?,
            blur: p.num("blur", 10.0)?.max(0.0),
            spread: p.num("spread", 0.0)?,
        },
        "inner-glow" => Effect::InnerGlow {
            look: p.look("blend", BlendMode::Screen, 75.0)?,
            glow: parse_glow(&p)?,
            blur: p.num("blur", 10.0)?.max(0.0),
            choke: p.num("choke", 0.0)?,
            center: match p.word("source").as_deref() {
                None | Some("edge") => false,
                Some("center") => true,
                Some(other) => return Err(p.bad("source", other, "edge or center")),
            },
        },
        "bevel" => {
            let style = match p.word("style").as_deref() {
                None | Some("inner") => BevelStyle::Inner,
                Some("outer") => BevelStyle::Outer,
                Some("emboss") => BevelStyle::Emboss,
                Some("pillow") => BevelStyle::Pillow,
                Some(other) => return Err(p.bad("style", other, "inner, outer, emboss, or pillow")),
            };
            let up = match p.word("direction").as_deref() {
                None | Some("up") => true,
                Some("down") => false,
                Some(other) => return Err(p.bad("direction", other, "up or down")),
            };
            Effect::Bevel(Bevel {
                style,
                depth: p.num("depth", 100.0)?.max(0.0),
                size: p.num("size", 5.0)?.max(0.0),
                soften: p.num("soften", 0.0)?.max(0.0),
                angle: p.angle("angle", 120.0)?,
                altitude: p.num("altitude", 30.0)?.clamp(0.0, 90.0),
                up,
                highlight: p.color("highlight", Rgba::WHITE)?,
                shadow: p.color("shadow", Rgba::BLACK)?,
                hi_look: p.look("blend", BlendMode::Screen, 75.0)?,
                sh_look: p.look("shadow_blend", BlendMode::Multiply, 75.0)?,
            })
        }
        "satin" => Effect::Satin {
            look: p.look("blend", BlendMode::Multiply, 50.0)?,
            color: p.color("color", Rgba::BLACK)?,
            off: p.offset("offset", (8.0, 8.0), doc)?,
            blur: p.num("blur", 14.0)?.max(0.0),
            invert: p.flag("invert"),
        },
        "overlay" => {
            let (fill, rank) = parse_fill(&p, doc, "kind")?;
            Effect::Overlay { look: p.look("blend", BlendMode::Normal, 100.0)?, fill, rank }
        }
        "stroke" => {
            let (fill, _) = parse_fill(&p, doc, "fill")?;
            let pos = match p.word("position").as_deref() {
                None | Some("outside") => StrokePos::Outside,
                Some("inside") => StrokePos::Inside,
                Some("center") => StrokePos::Center,
                Some(other) => return Err(p.bad("position", other, "inside, center, or outside")),
            };
            Effect::Stroke {
                look: p.look("blend", BlendMode::Normal, 100.0)?,
                fill,
                width: p.num("width", 3.0)?.max(0.0),
                pos,
            }
        }
        "fire" | "smoke" => Effect::Plume(parse_plume(&p, spec.kind == "fire", doc, size)?),
        _ => return Ok(None),
    }))
}

/// Fire (`--height --intensity --turbulence --colors --angle --front`) or smoke
/// (`--height --density --turbulence --color --drift`); both keep their noise pattern
/// in `variant`.
fn parse_plume(p: &Params, fire: bool, doc: &Document, size: (u32, u32)) -> Result<PlumeFx> {
    let angle = p.angle(if fire { "angle" } else { "drift" }, 90.0)?;
    // percent heights are of the content's extent along the rise direction
    let th = angle.to_radians();
    let reference = th.cos().abs() * size.0 as f64 + th.sin().abs() * size.1 as f64;
    let height = match p.raw("height") {
        None => reference * if fire { 0.5 } else { 0.8 },
        Some(v) => Length::parse(&v).map_err(|_| p.bad("height", &v, "a length"))?.to_px(reference, doc.dpi()),
    };
    let strength =
        if fire { p.num("intensity", 100.0)?.clamp(0.0, 200.0) } else { p.num("density", 50.0)?.clamp(0.0, 100.0) };
    let rise = Rise {
        plume: if fire { Plume::Fire } else { Plume::Smoke },
        height: height.clamp(0.0, MAX_PAD / 2.0),
        strength: strength / 100.0,
        turbulence: p.num("turbulence", 50.0)?.clamp(0.0, 100.0) / 100.0,
        angle,
        seed: p.variant()?,
    };
    let ramp = if fire {
        let stops = match p.stops("colors")? {
            Some(s) => s,
            None => color::parse_stops(FIRE_STOPS)?,
        };
        (0..256).map(|i| color::sample_stops(&stops, i as f32 / 255.0, Interp::Perceptual).to_array()).collect()
    } else {
        Vec::new()
    };
    let color = if fire { [0.0; 4] } else { p.color("color", color::parse_color(SMOKE_COLOR)?)? };
    Ok(PlumeFx {
        look: p.look("blend", BlendMode::Normal, 100.0)?,
        rise,
        ramp,
        color,
        front: if fire { (p.num("front", 15.0)?.clamp(0.0, 100.0) / 100.0) as f32 } else { 0.0 },
    })
}

// ---------------------------------------------------------------------------------------
// mask helpers

fn gauss(m: &Mask, sigma: f64, edge: Edge) -> Mask {
    let mut out = m.clone();
    if sigma > 0.0 && !m.data.is_empty() {
        kernels::gaussian_planar(&mut out.data, m.width as usize, m.height as usize, 1, sigma, sigma, edge);
    }
    out
}

/// `out(x, y) = m(x - dx, y - dy)`, bilinear for fractional offsets; `fill` outside.
fn shift(m: &Mask, dx: f64, dy: f64, fill: f32) -> Mask {
    if dx == 0.0 && dy == 0.0 {
        return m.clone();
    }
    let (ix, iy) = (dx.floor(), dy.floor());
    let (fx, fy) = ((dx - ix) as f32, (dy - iy) as f32);
    let (ix, iy) = (ix as i64, iy as i64);
    let (w, h) = (m.width as i64, m.height as i64);
    let get = |x: i64, y: i64| if x < 0 || y < 0 || x >= w || y >= h { fill } else { m.data[(y * w + x) as usize] };
    Mask::from_fn(m.width, m.height, |x, y| {
        let (sx, sy) = (x as i64 - ix, y as i64 - iy);
        let top = get(sx, sy) * (1.0 - fx) + get(sx - 1, sy) * fx;
        let bot = get(sx, sy - 1) * (1.0 - fx) + get(sx - 1, sy - 1) * fx;
        top * (1.0 - fy) + bot * fy
    })
}

fn zip(a: &Mask, b: &Mask, f: impl Fn(f32, f32) -> f32 + Sync + Send) -> Mask {
    let mut out = a.clone();
    out.data.par_iter_mut().zip(&b.data).for_each(|(x, &y)| *x = f(*x, y));
    out
}

fn map(mut m: Mask, f: impl Fn(f32) -> f32 + Sync + Send) -> Mask {
    m.data.par_iter_mut().for_each(|v| *v = f(*v));
    m
}

/// Photoshop's default glow contour ("range 50%"): full strength at the source edge.
fn boost(v: f32) -> f32 {
    (v * 2.0).clamp(0.0, 1.0)
}

// ---------------------------------------------------------------------------------------
// painting

enum Paint<'a> {
    Solid([f32; 4]),
    /// Color from intensity: the first stop at the glow's source (`t = 1 - e`).
    Ramp(&'a [Stop]),
    Grad {
        stops: &'a [Stop],
        style: GradStyle,
        reverse: bool,
        dir: (f64, f64),
        lo: f64,
        span: f64,
        center: (f64, f64),
        radius: f64,
    },
    /// Buffer pixel (x, y) shows the tile at ((x + ox) / zoom, (y + oy) / zoom), wrapped.
    Tile {
        tile: &'a Image,
        ox: i64,
        oy: i64,
        zoom: f64,
    },
    /// The colors of a fire or smoke field over the buffer (coverage comes separately).
    Plume(&'a flame::Field, &'a PlumeFx),
}

impl Paint<'_> {
    #[inline]
    fn at(&self, x: usize, y: usize, e: f32) -> [f32; 4] {
        match self {
            Paint::Solid(c) => *c,
            Paint::Ramp(stops) => color::sample_stops(stops, 1.0 - e, Interp::Perceptual).to_array(),
            Paint::Grad { stops, style, reverse, dir, lo, span, center, radius } => {
                let (px, py) = (x as f64 + 0.5, y as f64 + 0.5);
                let lin = (px * dir.0 + py * dir.1 - lo) / span;
                let mut t = match style {
                    GradStyle::Linear => lin,
                    GradStyle::Reflected => (lin * 2.0 - 1.0).abs(),
                    GradStyle::Radial => (px - center.0).hypot(py - center.1) / radius,
                }
                .clamp(0.0, 1.0);
                if *reverse {
                    t = 1.0 - t;
                }
                color::sample_stops(stops, t as f32, Interp::Perceptual).to_array()
            }
            Paint::Tile { tile, ox, oy, zoom } => {
                let (cx, cy) = (x as i64 + ox, y as i64 + oy);
                if *zoom <= 1.0 + 1e-9 {
                    let tx = cx.rem_euclid(tile.width as i64) as u32;
                    let ty = cy.rem_euclid(tile.height as i64) as u32;
                    return tile.get(tx, ty);
                }
                sample_wrapped(tile, (cx as f64 + 0.5) / zoom - 0.5, (cy as f64 + 0.5) / zoom - 0.5)
            }
            Paint::Plume(f, pf) => {
                let i = y * f.value.width as usize + x;
                plume_color(f.value.data[i], f.aux.data[i], pf)
            }
        }
    }
}

/// The paint of a fill over a box `[x, y, w, h]` (buffer coordinates; gradients span it).
fn fill_paint(fill: &Fill, bx: [f64; 4], origin: (i32, i32)) -> Paint<'_> {
    match fill {
        Fill::Color(c) => Paint::Solid(*c),
        Fill::Gradient { stops, angle, style, reverse } => {
            // Photoshop's convention: 0° runs left → right, 90° bottom → top
            let th = angle.to_radians();
            let dir = (th.cos(), -th.sin());
            let [x, y, w, h] = bx;
            let proj = [(x, y), (x + w, y), (x, y + h), (x + w, y + h)].map(|(px, py)| px * dir.0 + py * dir.1);
            let lo = proj.iter().copied().fold(f64::INFINITY, f64::min);
            let hi = proj.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            Paint::Grad {
                stops,
                style: *style,
                reverse: *reverse,
                dir,
                lo,
                span: (hi - lo).max(1e-6),
                center: (x + w / 2.0, y + h / 2.0),
                radius: (w.max(h) / 2.0).max(1e-6),
            }
        }
        Fill::Pattern { tile, .. } if tile.is_empty() => Paint::Solid([0.0; 4]),
        Fill::Pattern { tile, zoom } => Paint::Tile { tile, ox: origin.0 as i64, oy: origin.1 as i64, zoom: *zoom },
    }
}

/// Bilinear sample of a repeating tile at continuous pixel coordinates (centers on
/// integers), premultiplied so transparent texels do not bleed color.
fn sample_wrapped(tile: &Image, u: f64, v: f64) -> [f32; 4] {
    let (x0, y0) = (u.floor(), v.floor());
    let (tx, ty) = ((u - x0) as f32, (v - y0) as f32);
    let (w, h) = (tile.width as i64, tile.height as i64);
    let mut acc = [0f32; 4];
    for (dx, dy, k) in
        [(0, 0, (1.0 - tx) * (1.0 - ty)), (1, 0, tx * (1.0 - ty)), (0, 1, (1.0 - tx) * ty), (1, 1, tx * ty)]
    {
        let p = tile.get((x0 as i64 + dx).rem_euclid(w) as u32, (y0 as i64 + dy).rem_euclid(h) as u32);
        acc[0] += p[0] * p[3] * k;
        acc[1] += p[1] * p[3] * k;
        acc[2] += p[2] * p[3] * k;
        acc[3] += p[3] * k;
    }
    if acc[3] > 1e-6 {
        [acc[0] / acc[3], acc[1] / acc[3], acc[2] / acc[3], acc[3].min(1.0)]
    } else {
        [0.0; 4]
    }
}

fn glow_paint(g: &Glow) -> Paint<'_> {
    match g {
        Glow::Color(c) => Paint::Solid(*c),
        Glow::Stops(s) => Paint::Ramp(s),
    }
}

#[inline]
fn hash01(seed: u64, x: u32, y: u32) -> f32 {
    let mut z = seed ^ ((x as u64) << 32 | y as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^= z >> 31;
    (z >> 40) as f32 / (1u64 << 24) as f32
}

/// Composite `paint` with per-pixel coverage `cov` onto `dst` using the effect's blend
/// mode and opacity. `clip`: keep `dst`'s alpha (the effect only recolors existing
/// coverage; `cov` is then relative to it), else source-over.
fn apply(dst: &mut Image, cov: &Mask, paint: &Paint, look: Look, clip: bool, seed: u64, origin: (i32, i32)) {
    if look.opacity <= 0.0 {
        return;
    }
    let w = dst.width as usize;
    dst.data.par_chunks_mut(w * 4).enumerate().for_each(|(y, row)| {
        for x in 0..w {
            let e = cov.data[y * w + x].min(1.0);
            if e <= 0.0 {
                continue;
            }
            let px = &mut row[x * 4..x * 4 + 4];
            if clip && px[3] <= 0.0 {
                continue;
            }
            let c = paint.at(x, y, e);
            let mut k = (e * c[3] * look.opacity).min(1.0);
            if k <= 0.0 {
                continue;
            }
            if look.mode == BlendMode::Dissolve {
                let (cx, cy) = ((x as i64 + origin.0 as i64) as u32, (y as i64 + origin.1 as i64) as u32);
                if hash01(seed, cx, cy) >= k {
                    continue;
                }
                k = 1.0;
            }
            let src = [c[0], c[1], c[2], k];
            if clip {
                let a = px[3];
                px[3] = 1.0;
                composite_pixel(look.mode, px, src);
                px[3] = a;
            } else {
                composite_pixel(look.mode, px, src);
            }
        }
    });
}

/// Source-over `src` onto `dst` (same size).
fn over(dst: &mut Image, src: &Image) {
    let w = dst.width as usize * 4;
    dst.data.par_chunks_mut(w).zip(src.data.par_chunks(w)).for_each(|(d, s)| {
        for (dp, sp) in d.chunks_mut(4).zip(s.chunks(4)) {
            if sp[3] > 0.0 {
                composite_pixel(BlendMode::Normal, dp, [sp[0], sp[1], sp[2], sp[3]]);
            }
        }
    });
}

// ---------------------------------------------------------------------------------------
// bevel & emboss

struct BevelMaps {
    /// Highlight and shadow strength inside the content (relative to its coverage).
    inner: Option<(Mask, Mask)>,
    /// Highlight and shadow coverage outside the content (already × (1 - alpha)).
    outer: Option<(Mask, Mask)>,
}

/// Shade a height map built from the blurred alpha with a distant light at
/// `angle` (0° = from the right, 90° = from above) and `altitude` (90° = overhead).
fn bevel_maps(a: &Mask, b: &Bevel) -> BevelMaps {
    let r = b.size.max(0.5);
    // the height ramp: inside the edge (inner), outside it (outer), or across it
    let (hmap, ramp) = match b.style {
        BevelStyle::Inner => (gauss(&kernels::erode(a, r / 2.0), r / 4.0, Edge::Zero), r),
        BevelStyle::Outer => (gauss(&kernels::dilate(a, r / 2.0), r / 4.0, Edge::Zero), r),
        BevelStyle::Emboss | BevelStyle::Pillow => (gauss(a, r / 2.0, Edge::Zero), 2.0 * r),
    };
    let hs = b.depth / 100.0 * ramp * if b.up { 1.0 } else { -1.0 };
    let (th, ph) = (b.angle.to_radians(), b.altitude.to_radians());
    let light = [ph.cos() * th.cos(), -ph.cos() * th.sin(), ph.sin()];
    let flat = ph.sin();
    let (w, h) = (a.width as i64, a.height as i64);
    let at = |x: i64, y: i64| hmap.data[(y.clamp(0, h - 1) * w + x.clamp(0, w - 1)) as usize] as f64;
    // how much brighter (+) or darker (-) than a flat surface each pixel is lit
    let delta = Mask::from_fn(a.width, a.height, |x, y| {
        let (x, y) = (x as i64, y as i64);
        let gx = (at(x + 1, y) - at(x - 1, y)) * 0.5 * hs;
        let gy = (at(x, y + 1) - at(x, y - 1)) * 0.5 * hs;
        let n = (gx * gx + gy * gy + 1.0).sqrt();
        ((-gx * light[0] - gy * light[1] + light[2]) / n - flat) as f32
    });
    let hi_div = (1.0 - flat).max(1e-3) as f32;
    let sh_div = (1.0 + flat) as f32 / 1.5;
    let soften = |m: Mask| if b.soften > 0.0 { gauss(&m, b.soften / 2.0, Edge::Clamp) } else { m };
    let shade = |sign: f32| -> (Mask, Mask) {
        let hl = soften(map(delta.clone(), move |d| (sign * d / hi_div).clamp(0.0, 1.0)));
        let sh = soften(map(delta.clone(), move |d| (-sign * d / sh_div).clamp(0.0, 1.0)));
        (hl, sh)
    };
    let outside = |(hl, sh): (Mask, Mask)| (zip(&hl, a, |v, al| v * (1.0 - al)), zip(&sh, a, |v, al| v * (1.0 - al)));
    match b.style {
        BevelStyle::Inner => BevelMaps { inner: Some(shade(1.0)), outer: None },
        BevelStyle::Outer => BevelMaps { inner: None, outer: Some(outside(shade(1.0))) },
        BevelStyle::Emboss => {
            let s = shade(1.0);
            BevelMaps { outer: Some(outside((s.0.clone(), s.1.clone()))), inner: Some(s) }
        }
        // the outside slopes the other way: the shape looks pressed into the surface
        BevelStyle::Pillow => BevelMaps { inner: Some(shade(1.0)), outer: Some(outside(shade(-1.0))) },
    }
}

// ---------------------------------------------------------------------------------------
// fire and smoke colors

/// The color of a plume pixel: fire from its ramp (hotter near the outline, cooler
/// toward the tips), smoke as lighter and darker shades of its color.
#[inline]
fn plume_color(v: f32, aux: f32, pf: &PlumeFx) -> [f32; 4] {
    if pf.rise.plume == Plume::Fire {
        let heat = (v * (1.05 - 0.4 * aux)).clamp(0.0, 1.0);
        let c = pf.ramp[(heat * 255.0).round() as usize];
        [c[0], c[1], c[2], 1.0]
    } else {
        let shade = 0.72 + 0.56 * aux;
        let c = pf.color;
        [(c[0] * shade).min(1.0), (c[1] * shade).min(1.0), (c[2] * shade).min(1.0), 1.0]
    }
}

/// The coverage of a plume: fire fades in over low heats (the ramp's cold end is
/// transparent), smoke is as opaque as it is dense.
fn plume_cov(f: &flame::Field, pf: &PlumeFx) -> Mask {
    let fire = pf.rise.plume == Plume::Fire;
    let ramp = &pf.ramp;
    let alpha = pf.color[3];
    let mut cov = f.value.clone();
    cov.data.par_iter_mut().zip(&f.aux.data).for_each(|(v, &aux)| {
        if *v <= 0.0 {
            *v = 0.0;
        } else if fire {
            let heat = (*v * (1.05 - 0.4 * aux)).clamp(0.0, 1.0);
            let t = ((*v - 0.07) / 0.33).clamp(0.0, 1.0);
            *v = t * t * (3.0 - 2.0 * t) * ramp[(heat * 255.0).round() as usize][3];
        } else {
            *v = v.min(1.0) * alpha;
        }
    });
    cov
}

// ---------------------------------------------------------------------------------------
// the renderer

/// Apply a layer's style stack to its rendered content (the `compose` fx hook).
pub fn render(specs: &[FxSpec], content: Rendered, doc: &Document) -> Result<Rendered> {
    let visible = content.image.alpha_bbox(0.0);
    let size = visible.map(|b| (b.w, b.h)).unwrap_or((0, 0));
    let mut effects = Vec::new();
    for s in specs.iter().filter(|s| s.enabled) {
        if let Some(e) = parse(s, doc, size)? {
            effects.push(e);
        }
    }
    if effects.is_empty() || content.image.is_empty() {
        return Ok(content);
    }
    let Some(bb) = visible else {
        return Ok(content);
    };
    let reach = effects.iter().map(Effect::margins).fold([0.0f64; 4], |m, e| [0, 1, 2, 3].map(|k| m[k].max(e[k])));
    let [pl, pt, pr, pb] = reach.map(|v| v.clamp(0.0, MAX_PAD).ceil() as u32 + 2);
    let (w, h) = (bb.w + pl + pr, bb.h + pt + pb);
    let origin = (content.x + bb.x - pl as i32, content.y + bb.y - pt as i32);
    let mut fill = content.image.crop(RectI::new(bb.x - pl as i32, bb.y - pt as i32, w, h));
    let a = fill.alpha_mask();
    let cbox = [pl as f64, pt as f64, bb.w as f64, bb.h as f64];
    let grow = |d: f64| [cbox[0] - d, cbox[1] - d, cbox[2] + 2.0 * d, cbox[3] + 2.0 * d];
    let seed = |i: usize| 0xF0F0_5EED_u64 ^ (i as u64 + 1).wrapping_mul(0x9E37_79B9_7F4A_7C15);
    // the buffer's corner relative to the content: plumes are anchored there
    let anchor = ((bb.x - pl as i32) as f64, (bb.y - pt as i32) as f64);
    // fire drawn over the content's edges, composited last: (coverage, field, effect)
    let mut fronts: Vec<(Mask, flame::Field, usize)> = Vec::new();

    let mut order: Vec<usize> = (0..effects.len()).collect();
    order.sort_by_key(|&i| effects[i].phase());

    // 1. beneath the content
    let mut below = Image::new(w, h);
    for &i in &order {
        match &effects[i] {
            Effect::DropShadow { look, color, off, blur, spread } => {
                let (m, s) = blur_parts(*blur, *spread);
                let cov = shift(&gauss(&kernels::dilate(&a, m), s, Edge::Zero), off.0, off.1, 0.0);
                apply(&mut below, &cov, &Paint::Solid(*color), *look, false, seed(i), origin);
            }
            Effect::OuterGlow { look, glow, blur, spread } => {
                let (m, s) = blur_parts(*blur, *spread);
                let cov = map(gauss(&kernels::dilate(&a, m), s, Edge::Zero), boost);
                apply(&mut below, &cov, &glow_paint(glow), *look, false, seed(i), origin);
            }
            Effect::Plume(pf) => {
                let f = flame::field(&a, &pf.rise, anchor);
                let cov = plume_cov(&f, pf);
                apply(&mut below, &cov, &Paint::Plume(&f, pf), pf.look, false, seed(i), origin);
                if pf.front > 0.0 {
                    // tongues of the flames over the content's own edges
                    let k = pf.front;
                    let licks = flame::licks(&a, &pf.rise, anchor);
                    fronts.push((zip(&cov, &licks, move |c, l| c * l * k), f, i));
                }
            }
            Effect::Stroke { look, fill: f, width, pos: StrokePos::Outside } => {
                // the coverage that, under the content, makes the union reach the dilation
                let d = kernels::dilate(&a, *width);
                let cov = zip(&d, &a, |dv, av| if av >= 1.0 { 0.0 } else { ((dv - av) / (1.0 - av)).clamp(0.0, 1.0) });
                apply(&mut below, &cov, &fill_paint(f, grow(*width), origin), *look, false, seed(i), origin);
            }
            _ => {}
        }
    }

    // 2. the content and the effects clipped to it
    let ones = Mask::filled(w, h, 1.0);
    let mut top: Vec<(Mask, [f32; 4], Look, u64)> = Vec::new();
    for &i in &order {
        match &effects[i] {
            Effect::Overlay { look, fill: f, .. } => {
                apply(&mut fill, &ones, &fill_paint(f, cbox, origin), *look, true, seed(i), origin);
            }
            Effect::Satin { look, color, off, blur, invert } => {
                let s = blur / 2.0;
                let p1 = gauss(&shift(&a, off.0, off.1, 0.0), s, Edge::Zero);
                let p2 = gauss(&shift(&a, -off.0, -off.1, 0.0), s, Edge::Zero);
                let inv = *invert;
                let cov = zip(&p1, &p2, move |x, y| {
                    let d = (x - y).abs().min(1.0);
                    if inv {
                        1.0 - d
                    } else {
                        d
                    }
                });
                apply(&mut fill, &cov, &Paint::Solid(*color), *look, true, seed(i), origin);
            }
            Effect::InnerGlow { look, glow, blur, choke, center } => {
                let (m, s) = blur_parts(*blur, *choke);
                let edge = map(gauss(&kernels::dilate(&a.inverted(), m), s, Edge::Clamp), boost);
                let cov = if *center { map(edge, |v| 1.0 - v) } else { edge };
                apply(&mut fill, &cov, &glow_paint(glow), *look, true, seed(i), origin);
            }
            Effect::InnerShadow { look, color, off, blur, choke } => {
                let (m, s) = blur_parts(*blur, *choke);
                let outside = shift(&a.inverted(), off.0, off.1, 1.0);
                let cov = gauss(&kernels::dilate(&outside, m), s, Edge::Clamp);
                apply(&mut fill, &cov, &Paint::Solid(*color), *look, true, seed(i), origin);
            }
            Effect::Bevel(b) => {
                let maps = bevel_maps(&a, b);
                if let Some((hl, sh)) = maps.inner {
                    apply(&mut fill, &hl, &Paint::Solid(b.highlight), b.hi_look, true, seed(i), origin);
                    apply(&mut fill, &sh, &Paint::Solid(b.shadow), b.sh_look, true, seed(i) ^ 1, origin);
                }
                if let Some((hl, sh)) = maps.outer {
                    top.push((hl, b.highlight, b.hi_look, seed(i)));
                    top.push((sh, b.shadow, b.sh_look, seed(i) ^ 1));
                }
            }
            Effect::Stroke { look, fill: f, width, pos: StrokePos::Inside } => {
                let e = kernels::erode(&a, *width);
                let cov = zip(&a, &e, |av, ev| if av <= 0.0 { 0.0 } else { (1.0 - ev / av).clamp(0.0, 1.0) });
                apply(&mut fill, &cov, &fill_paint(f, cbox, origin), *look, true, seed(i), origin);
            }
            _ => {}
        }
    }
    let mut out = below;
    over(&mut out, &fill);

    // 3. on top
    for (cov, color, look, s) in &top {
        apply(&mut out, cov, &Paint::Solid(*color), *look, false, *s, origin);
    }
    for &i in &order {
        if let Effect::Stroke { look, fill: f, width, pos: StrokePos::Center } = &effects[i] {
            let half = width / 2.0;
            let cov = zip(&kernels::dilate(&a, half), &kernels::erode(&a, half), |d, e| (d - e).clamp(0.0, 1.0));
            apply(&mut out, &cov, &fill_paint(f, grow(half), origin), *look, false, seed(i), origin);
        }
    }
    for (cov, f, i) in &fronts {
        if let Effect::Plume(pf) = &effects[*i] {
            apply(&mut out, cov, &Paint::Plume(f, pf), pf.look, false, seed(*i) ^ 2, origin);
        }
    }

    let ob = out.alpha_bbox(0.0).unwrap_or(RectI::new(0, 0, w, h));
    let image = if ob.w == w && ob.h == h { out } else { out.crop(ob) };
    Ok(Rendered { image, x: origin.0 + ob.x, y: origin.1 + ob.y })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shift_moves_and_fills() {
        let m = Mask::from_fn(4, 1, |x, _| if x == 1 { 1.0 } else { 0.0 });
        let s = shift(&m, 1.0, 0.0, 0.0);
        assert_eq!(s.data, vec![0.0, 0.0, 1.0, 0.0]);
        let s = shift(&m, 0.5, 0.0, 0.0);
        assert!((s.data[1] - 0.5).abs() < 1e-6 && (s.data[2] - 0.5).abs() < 1e-6);
        let s = shift(&Mask::new(3, 1), -1.0, 0.0, 1.0);
        assert_eq!(s.data, vec![0.0, 0.0, 1.0]);
    }

    #[test]
    fn numbers_format_compactly() {
        assert_eq!(fmt_num(16.0), "16");
        assert_eq!(fmt_num(4.5), "4.5");
        assert_eq!(fmt_num(1.0 / 3.0), "0.333");
        assert_eq!(fmt_len(Length::parse("-12.5%").unwrap()), "-12.5%");
        assert_eq!(parse_num("75%"), Some(75.0));
    }

    #[test]
    fn blur_parts_split_spread() {
        assert_eq!(blur_parts(10.0, 0.0), (0.0, 5.0));
        assert_eq!(blur_parts(10.0, 100.0), (10.0, 0.0));
        assert_eq!(blur_parts(10.0, 50.0), (5.0, 2.5));
    }

    #[test]
    fn gradient_paint_follows_photoshop_angles() {
        let stops = color::parse_stops("#000@0,#fff@1").unwrap();
        let f = Fill::Gradient { stops, angle: 0.0, style: GradStyle::Linear, reverse: false };
        let p = fill_paint(&f, [0.0, 0.0, 10.0, 10.0], (0, 0));
        assert!(p.at(0, 5, 1.0)[0] < 0.1 && p.at(9, 5, 1.0)[0] > 0.9, "0° runs left to right");
        let f = Fill::Gradient {
            stops: color::parse_stops("#000@0,#fff@1").unwrap(),
            angle: 90.0,
            style: GradStyle::Linear,
            reverse: false,
        };
        let p = fill_paint(&f, [0.0, 0.0, 10.0, 10.0], (0, 0));
        assert!(p.at(5, 9, 1.0)[0] < 0.1 && p.at(5, 0, 1.0)[0] > 0.9, "90° runs bottom to top");
    }
}
