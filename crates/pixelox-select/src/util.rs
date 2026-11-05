//! Helpers shared by the select, mask, and transform ops: parameter builders, mask
//! statistics (summed in a fixed order so results never depend on the thread count), mask
//! references, and a small affine type.

use pixelox_core::ctx::image_to_mask;
use pixelox_core::image::SelectMode;
use pixelox_core::kernels::{Filter, FILTERS};
use pixelox_core::op::ParamSpec;
use pixelox_core::prelude::*;
use rayon::prelude::*;
use std::path::Path;

/// Coverage below this counts as empty for bounds (it would quantize to 0 at 8 bits).
pub const BBOX_THRESHOLD: f32 = 0.5 / 255.0;

pub fn r1(v: f64) -> f64 {
    (v * 10.0).round() / 10.0
}

pub fn r3(v: f64) -> f64 {
    (v * 1000.0).round() / 1000.0
}

pub fn r4(v: f64) -> f64 {
    (v * 10000.0).round() / 10000.0
}

// ---------------------------------------------------------------------------------------
// parameters

pub fn mode_param() -> ParamSpec {
    param("mode", ParamKind::Choice)
        .choices(pixelox_core::image::SELECT_MODES)
        .default("replace")
        .help("How the result combines with the active selection: replace, add, subtract, or intersect")
}

pub fn feather_param() -> ParamSpec {
    param("feather", ParamKind::Length).default("0").help(
        "Soften the edge by this radius in px (Gaussian; % of the shorter canvas side); 0 = crisp anti-aliased edge",
    )
}

pub fn interp_param() -> ParamSpec {
    param("interp", ParamKind::Choice).choices(FILTERS).auto().default("auto").help(
        "Resampling filter: nearest, bilinear, bicubic, lanczos3; auto = Lanczos-3 when shrinking, bicubic otherwise",
    )
}

pub fn anchor_param(default: &'static str, help: &'static str) -> ParamSpec {
    param("anchor", ParamKind::Choice).choices(units::ANCHORS).default(default).help(help)
}

pub fn select_mode(a: &Args) -> Result<SelectMode> {
    SelectMode::parse(&a.choice("mode"))
}

/// `None` for `auto`.
pub fn interp(a: &Args) -> Result<Option<Filter>> {
    if a.is_auto("interp") || a.raw("interp").is_none() {
        return Ok(None);
    }
    Filter::parse(&a.choice("interp")).map(Some)
}

pub fn anchor(a: &Args) -> Result<Anchor> {
    Anchor::parse(&a.choice("anchor"))
}

/// A non-negative length param in px (`%` of the shorter canvas side).
pub fn len_px(a: &Args, name: &str, doc: &Document) -> Result<f64> {
    let reference = doc.width.min(doc.height) as f64;
    let v = a.px(name, reference, doc.dpi())?;
    if v < 0.0 || !v.is_finite() {
        return Err(Error::new("PX1013", format!("--{name} must not be negative")).with_context("param", name));
    }
    Ok(v)
}

/// Which of the value params (no default) and flags were given, as `--name`.
pub fn given(a: &Args, values: &[&str], flags: &[&str]) -> Vec<String> {
    let mut out: Vec<String> = values.iter().filter(|n| a.raw(n).is_some()).map(|n| format!("--{n}")).collect();
    out.extend(flags.iter().filter(|n| a.flag(n)).map(|n| format!("--{n}")));
    out
}

/// PX1005 when more than one of them is given; returns the given names.
pub fn at_most_one(a: &Args, values: &[&str], flags: &[&str]) -> Result<Vec<String>> {
    let g = given(a, values, flags);
    if g.len() > 1 {
        return Err(Error::new("PX1005", format!("pass only one of {}", g.join(", "))).with_context("given", &g));
    }
    Ok(g)
}

// ---------------------------------------------------------------------------------------
// masks

pub struct MaskStats {
    pub area: f64,
    pub coverage_pct: f64,
    pub bounds: Option<RectI>,
}

/// Area (sum of coverage), coverage %, and bounds, summed row by row in order.
pub fn mask_stats(m: &Mask) -> MaskStats {
    let w = m.width as usize;
    if w == 0 || m.height == 0 {
        return MaskStats { area: 0.0, coverage_pct: 0.0, bounds: None };
    }
    let rows: Vec<f64> = m.data.par_chunks(w).map(|r| r.iter().map(|&v| v as f64).sum::<f64>()).collect();
    let area: f64 = rows.iter().sum();
    let bounds = if area <= 0.0 { None } else { m.bbox(BBOX_THRESHOLD) };
    MaskStats { area, coverage_pct: area / m.data.len() as f64 * 100.0, bounds }
}

/// `{"empty", "bounds", "area", "coverage"}` for results (`coverage` in percent).
pub fn mask_json(m: &Mask) -> Value {
    let s = mask_stats(m);
    json!({
        "empty": s.bounds.is_none(),
        "bounds": s.bounds.map(|b| b.to_json()),
        "area": r1(s.area),
        "coverage": r3(s.coverage_pct),
    })
}

/// Gaussian feather with the same radius → sigma convention as live layer-mask feather.
pub fn feather(m: &Mask, radius: f64) -> Mask {
    if radius <= 0.0 {
        m.clone()
    } else {
        kernels::blur_mask(m, radius / 2.0)
    }
}

/// Make a stored mask match the canvas (masks written by older steps may be stale).
pub fn fit_mask(m: Mask, w: u32, h: u32) -> Mask {
    if m.width == w && m.height == h {
        m
    } else {
        kernels::resize_mask(&m, w, h, Filter::Bilinear)
    }
}

/// A grayscale, opaque image of a mask.
pub fn mask_image(m: &Mask) -> Image {
    let mut img = Image::new(m.width, m.height);
    img.data.par_chunks_mut(4).zip(m.data.par_iter()).for_each(|(p, &v)| {
        let v = v.clamp(0.0, 1.0);
        p.copy_from_slice(&[v, v, v, 1.0]);
    });
    img
}

/// Resolve a mask reference (`@sel`, a library name, `@alpha:<layer>`, or a grayscale
/// image path) recording a file under this op's `param` for replay.
pub fn resolve_mask_ref(ctx: &mut OpCtx, doc: &Document, r: &str, param: &str) -> Result<Mask> {
    let r = r.trim();
    if r.starts_with("@alpha:") || r == "@sel" || doc.masks.contains_key(r) {
        return ctx.resolve_mask(doc, r).map(|m| fit_mask(m, doc.width, doc.height));
    }
    let path = Path::new(r);
    if r.starts_with('@') || (!path.exists() && path.extension().is_none()) {
        return doc.named_mask(r);
    }
    decode_mask_file(ctx, doc, path, param)
}

/// Decode a grayscale image as a canvas-sized mask (PX2313 on a size mismatch).
pub fn decode_mask_file(ctx: &mut OpCtx, doc: &Document, path: &Path, param: &str) -> Result<Mask> {
    let (img, _) = ctx.decode_aux(path, "mask", Some(param))?;
    if img.width != doc.width || img.height != doc.height {
        let p = path.display().to_string();
        return Err(Error::new(
            "PX2313",
            format!("mask image {p} is {}x{}, the canvas is {}x{}", img.width, img.height, doc.width, doc.height),
        )
        .with_hint(format!(
            "resize it first: pixelox transform resize {p} --size {}x{} -o mask-resized.png",
            doc.width, doc.height
        ))
        .with_context("mask_size", [img.width, img.height])
        .with_context("canvas", [doc.width, doc.height]));
    }
    Ok(image_to_mask(&img))
}

/// Visible bounds of a layer's own content (fx excluded), or `None` when empty.
pub fn content_bounds(doc: &Document, layer: &Layer) -> Result<Option<RectI>> {
    if layer.fx.is_empty() {
        return compose::layer_bounds(doc, layer);
    }
    let mut c = layer.clone();
    c.fx.clear();
    compose::layer_bounds(doc, &c)
}

// ---------------------------------------------------------------------------------------
// affine transforms

/// `x' = a·x + c·y + e`, `y' = b·x + d·y + f` (the SVG / `geom::transform_path_data` order).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Affine2 {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f64,
}

/// Relative tolerance for treating a transform as uniform (resize rounding to whole px).
pub const SIMILAR_TOL: f64 = 0.01;

impl Affine2 {
    pub const IDENTITY: Affine2 = Affine2 { a: 1.0, b: 0.0, c: 0.0, d: 1.0, e: 0.0, f: 0.0 };

    pub fn new(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Affine2 {
        Affine2 { a, b, c, d, e, f }
    }

    pub fn translate(dx: f64, dy: f64) -> Affine2 {
        Affine2::new(1.0, 0.0, 0.0, 1.0, dx, dy)
    }

    pub fn scale(sx: f64, sy: f64) -> Affine2 {
        Affine2::new(sx, 0.0, 0.0, sy, 0.0, 0.0)
    }

    /// Clockwise (on a y-down canvas) rotation by `deg` about the origin; exact for
    /// multiples of 90°.
    pub fn rotate(deg: f64) -> Affine2 {
        let q = deg / 90.0;
        let (s, c) = if (q - q.round()).abs() < 1e-9 {
            match (q.round() as i64).rem_euclid(4) {
                0 => (0.0, 1.0),
                1 => (1.0, 0.0),
                2 => (0.0, -1.0),
                _ => (-1.0, 0.0),
            }
        } else {
            deg.to_radians().sin_cos()
        };
        Affine2::new(c, s, -s, c, 0.0, 0.0)
    }

    /// `self` applied about the point (`px`, `py`).
    pub fn about(self, px: f64, py: f64) -> Affine2 {
        Affine2::translate(-px, -py).then(self).then(Affine2::translate(px, py))
    }

    /// `self` first, then `next`.
    pub fn then(self, n: Affine2) -> Affine2 {
        Affine2 {
            a: n.a * self.a + n.c * self.b,
            b: n.b * self.a + n.d * self.b,
            c: n.a * self.c + n.c * self.d,
            d: n.b * self.c + n.d * self.d,
            e: n.a * self.e + n.c * self.f + n.e,
            f: n.b * self.e + n.d * self.f + n.f,
        }
    }

    #[inline]
    pub fn apply(&self, x: f64, y: f64) -> (f64, f64) {
        (self.a * x + self.c * y + self.e, self.b * x + self.d * y + self.f)
    }

    pub fn det(&self) -> f64 {
        self.a * self.d - self.b * self.c
    }

    pub fn inverse(&self) -> Option<Affine2> {
        let det = self.det();
        if det.abs() < 1e-12 || !det.is_finite() {
            return None;
        }
        Some(Affine2 {
            a: self.d / det,
            b: -self.b / det,
            c: -self.c / det,
            d: self.a / det,
            e: (self.c * self.f - self.d * self.e) / det,
            f: (self.b * self.e - self.a * self.f) / det,
        })
    }

    pub fn coeffs(&self) -> [f64; 6] {
        [self.a, self.b, self.c, self.d, self.e, self.f]
    }

    pub fn is_identity(&self) -> bool {
        *self == Affine2::IDENTITY
    }

    pub fn is_translation(&self) -> bool {
        self.a == 1.0 && self.b == 0.0 && self.c == 0.0 && self.d == 1.0
    }

    /// Maps the pixel lattice onto itself (multiples of 90°, flips, whole-pixel shifts):
    /// such transforms are exact pixel permutations.
    pub fn is_lattice(&self) -> bool {
        let unit = |v: f64| v == 0.0 || v == 1.0 || v == -1.0;
        let int = |v: f64| (v - v.round()).abs() < 1e-9;
        unit(self.a)
            && unit(self.b)
            && unit(self.c)
            && unit(self.d)
            && self.det().abs() == 1.0
            && int(self.e)
            && int(self.f)
    }

    /// Uniform scale factor (√|det|).
    pub fn uniform_scale(&self) -> f64 {
        self.det().abs().sqrt()
    }

    /// `(scale, clockwise degrees)` when this is a rotation + uniform scale (+ translation)
    /// without reflection, within `tol` (relative).
    pub fn similarity(&self, tol: f64) -> Option<(f64, f64)> {
        let s = self.uniform_scale();
        if self.det() <= 0.0 || s < 1e-12 {
            return None;
        }
        if (self.a - self.d).abs() <= tol * s && (self.b + self.c).abs() <= tol * s {
            Some((s, self.b.atan2(self.a).to_degrees()))
        } else {
            None
        }
    }

    /// `(sx, sy)` when this is an axis-aligned positive scale (+ translation).
    pub fn axis_scale(&self) -> Option<(f64, f64)> {
        if self.b == 0.0 && self.c == 0.0 && self.a > 0.0 && self.d > 0.0 {
            Some((self.a, self.d))
        } else {
            None
        }
    }

    /// Singular values `(min, max)` of the linear part.
    pub fn singular_values(&self) -> (f64, f64) {
        let (a, b, c, d) = (self.a, self.b, self.c, self.d);
        let s1 = a * a + b * b + c * c + d * d;
        let det = (a * d - b * c).abs();
        let disc = (s1 * s1 - 4.0 * det * det).max(0.0).sqrt();
        let smax = ((s1 + disc) / 2.0).sqrt();
        let smin = ((s1 - disc) / 2.0).max(0.0).sqrt();
        (smin, smax)
    }

    /// Bounding box `(x0, y0, x1, y1)` of a rectangle's image.
    pub fn map_rect(&self, x: f64, y: f64, w: f64, h: f64) -> (f64, f64, f64, f64) {
        let pts = [self.apply(x, y), self.apply(x + w, y), self.apply(x, y + h), self.apply(x + w, y + h)];
        let mut b = (f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
        for (px, py) in pts {
            b.0 = b.0.min(px);
            b.1 = b.1.min(py);
            b.2 = b.2.max(px);
            b.3 = b.3.max(py);
        }
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn affine_algebra() {
        let m = Affine2::rotate(30.0).about(10.0, 5.0).then(Affine2::translate(3.0, -2.0));
        let inv = m.inverse().unwrap();
        let (x, y) = m.apply(7.0, 9.0);
        let (bx, by) = inv.apply(x, y);
        assert!((bx - 7.0).abs() < 1e-9 && (by - 9.0).abs() < 1e-9);
        let (s, deg) = m.similarity(1e-9).unwrap();
        assert!((s - 1.0).abs() < 1e-12 && (deg - 30.0).abs() < 1e-9);
        assert!(Affine2::rotate(90.0).then(Affine2::translate(4.0, 0.0)).is_lattice());
        assert!(!Affine2::rotate(45.0).is_lattice());
        let (lo, hi) = Affine2::scale(2.0, 0.5).singular_values();
        assert!((lo - 0.5).abs() < 1e-12 && (hi - 2.0).abs() < 1e-12);
        // clockwise on a y-down canvas: +x goes to +y
        let (x, y) = Affine2::rotate(90.0).apply(1.0, 0.0);
        assert_eq!((x, y), (0.0, 1.0));
    }

    #[test]
    fn stats_are_sane() {
        let m = Mask::from_fn(10, 10, |x, y| if (2..5).contains(&x) && (3..7).contains(&y) { 1.0 } else { 0.0 });
        let s = mask_stats(&m);
        assert_eq!(s.area, 12.0);
        assert_eq!(s.bounds, Some(RectI::new(2, 3, 3, 4)));
        assert!((s.coverage_pct - 12.0).abs() < 1e-9);
        assert!(mask_stats(&Mask::new(4, 4)).bounds.is_none());
    }
}
