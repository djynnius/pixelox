//! The shared value parser: geometry (`X,Y,WxH`, `X,Y`, `WxH`), lengths with units
//! (px default, `%`, `pt`, `mm`, `in`), anchors, point lists, ranges, angles, durations,
//! and byte sizes. CLI flags, pipeline YAML, and MCP JSON all go through these, so the same
//! text means the same thing everywhere (PRD §8.1).

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};

/// Unit of a [`Length`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Unit {
    Px,
    Pct,
    Pt,
    Mm,
    In,
}

/// A length or coordinate with a unit. Resolve with [`Length::resolve`].
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Length {
    pub value: f64,
    pub unit: Unit,
}

fn geom_err(s: &str, what: &str) -> Error {
    Error::new("PX1011", format!("'{s}' is not a valid {what}")).with_context("value", s)
}

impl Length {
    pub fn px(v: f64) -> Length {
        Length { value: v, unit: Unit::Px }
    }

    pub fn parse(s: &str) -> Result<Length> {
        let t = s.trim();
        let (num, unit) = if let Some(n) = t.strip_suffix('%') {
            (n, Unit::Pct)
        } else if let Some(n) = t.strip_suffix("px") {
            (n, Unit::Px)
        } else if let Some(n) = t.strip_suffix("pt") {
            (n, Unit::Pt)
        } else if let Some(n) = t.strip_suffix("mm") {
            (n, Unit::Mm)
        } else if let Some(n) = t.strip_suffix("in") {
            (n, Unit::In)
        } else {
            (t, Unit::Px)
        };
        let value: f64 = num.trim().parse().map_err(|_| geom_err(s, "length"))?;
        if !value.is_finite() {
            return Err(geom_err(s, "length"));
        }
        Ok(Length { value, unit })
    }

    /// Size in pixels: `%` of `reference`, physical units through `dpi`.
    pub fn to_px(self, reference: f64, dpi: f64) -> f64 {
        match self.unit {
            Unit::Px => self.value,
            Unit::Pct => self.value / 100.0 * reference,
            Unit::Pt => self.value * dpi / 72.0,
            Unit::Mm => self.value * dpi / 25.4,
            Unit::In => self.value * dpi,
        }
    }

    /// A position: like [`Length::to_px`], but negative non-percent values count from the
    /// far edge (`-48` on a 1000 px axis is 952).
    pub fn resolve_pos(self, extent: f64, dpi: f64) -> f64 {
        let v = self.to_px(extent, dpi);
        if self.value < 0.0 {
            extent + v
        } else {
            v
        }
    }
}

/// `X,Y` in document space.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PointSpec {
    pub x: Length,
    pub y: Length,
}

impl PointSpec {
    pub fn parse(s: &str) -> Result<PointSpec> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err(geom_err(s, "point (X,Y)"));
        }
        Ok(PointSpec { x: Length::parse(parts[0])?, y: Length::parse(parts[1])? })
    }

    pub fn resolve(&self, w: f64, h: f64, dpi: f64) -> (f64, f64) {
        (self.x.resolve_pos(w, dpi), self.y.resolve_pos(h, dpi))
    }
}

/// `WxH`; either side may be blank (`2048x`, `x1080`) to keep the aspect ratio.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SizeSpec {
    pub w: Option<Length>,
    pub h: Option<Length>,
}

impl SizeSpec {
    pub fn parse(s: &str) -> Result<SizeSpec> {
        let t = s.trim();
        // a bare percentage scales both sides
        if !t.contains('x') && !t.contains('X') {
            let l = Length::parse(t).map_err(|_| geom_err(s, "size (WxH)"))?;
            return Ok(SizeSpec { w: Some(l), h: Some(l) });
        }
        let (a, b) = t.split_once(['x', 'X']).ok_or_else(|| geom_err(s, "size (WxH)"))?;
        let w = if a.trim().is_empty() { None } else { Some(Length::parse(a)?) };
        let h = if b.trim().is_empty() { None } else { Some(Length::parse(b)?) };
        if w.is_none() && h.is_none() {
            return Err(geom_err(s, "size (WxH)"));
        }
        Ok(SizeSpec { w, h })
    }

    /// Resolve against a reference size (`%` and aspect); returns whole pixels ≥ 1.
    pub fn resolve(&self, ref_w: f64, ref_h: f64, dpi: f64) -> (u32, u32) {
        let (w, h) = self.resolve_f(ref_w, ref_h, dpi);
        (w.round().max(1.0) as u32, h.round().max(1.0) as u32)
    }

    pub fn resolve_f(&self, ref_w: f64, ref_h: f64, dpi: f64) -> (f64, f64) {
        let aspect = if ref_h > 0.0 { ref_w / ref_h } else { 1.0 };
        match (self.w, self.h) {
            (Some(w), Some(h)) => (w.to_px(ref_w, dpi), h.to_px(ref_h, dpi)),
            (Some(w), None) => {
                let w = w.to_px(ref_w, dpi);
                (w, w / aspect)
            }
            (None, Some(h)) => {
                let h = h.to_px(ref_h, dpi);
                (h * aspect, h)
            }
            (None, None) => (ref_w, ref_h),
        }
    }
}

/// `X,Y,WxH`.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct RectSpec {
    pub x: Length,
    pub y: Length,
    pub w: Length,
    pub h: Length,
}

/// A resolved rectangle in pixels (may be fractional).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RectF {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

/// A resolved integer rectangle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RectI {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

impl RectI {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> RectI {
        RectI { x, y, w, h }
    }
    pub fn right(&self) -> i32 {
        self.x + self.w as i32
    }
    pub fn bottom(&self) -> i32 {
        self.y + self.h as i32
    }
    pub fn is_empty(&self) -> bool {
        self.w == 0 || self.h == 0
    }
    /// Intersection (empty rect when disjoint).
    pub fn intersect(&self, o: &RectI) -> RectI {
        let x0 = self.x.max(o.x);
        let y0 = self.y.max(o.y);
        let x1 = self.right().min(o.right());
        let y1 = self.bottom().min(o.bottom());
        if x1 <= x0 || y1 <= y0 {
            RectI::new(x0, y0, 0, 0)
        } else {
            RectI::new(x0, y0, (x1 - x0) as u32, (y1 - y0) as u32)
        }
    }
    pub fn union(&self, o: &RectI) -> RectI {
        if self.is_empty() {
            return *o;
        }
        if o.is_empty() {
            return *self;
        }
        let x0 = self.x.min(o.x);
        let y0 = self.y.min(o.y);
        let x1 = self.right().max(o.right());
        let y1 = self.bottom().max(o.bottom());
        RectI::new(x0, y0, (x1 - x0) as u32, (y1 - y0) as u32)
    }
    /// `[x, y, w, h]` for JSON results.
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!([self.x, self.y, self.w, self.h])
    }
}

impl RectF {
    pub fn round(&self) -> RectI {
        let x0 = self.x.round();
        let y0 = self.y.round();
        let x1 = (self.x + self.w).round();
        let y1 = (self.y + self.h).round();
        RectI::new(x0 as i32, y0 as i32, (x1 - x0).max(0.0) as u32, (y1 - y0).max(0.0) as u32)
    }
}

impl RectSpec {
    pub fn parse(s: &str) -> Result<RectSpec> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 3 {
            return Err(geom_err(s, "rectangle (X,Y,WxH)"));
        }
        let (w, h) = parts[2].split_once(['x', 'X']).ok_or_else(|| geom_err(s, "rectangle (X,Y,WxH)"))?;
        Ok(RectSpec {
            x: Length::parse(parts[0])?,
            y: Length::parse(parts[1])?,
            w: Length::parse(w)?,
            h: Length::parse(h)?,
        })
    }

    pub fn resolve(&self, canvas_w: f64, canvas_h: f64, dpi: f64) -> RectF {
        RectF {
            x: self.x.resolve_pos(canvas_w, dpi),
            y: self.y.resolve_pos(canvas_h, dpi),
            w: self.w.to_px(canvas_w, dpi).abs(),
            h: self.h.to_px(canvas_h, dpi).abs(),
        }
    }
}

/// Nine-point anchor (`tl t tr l c r bl b br`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Anchor {
    Tl,
    T,
    Tr,
    L,
    C,
    R,
    Bl,
    B,
    Br,
}

pub const ANCHORS: &[&str] = &["tl", "t", "tr", "l", "c", "r", "bl", "b", "br"];

impl Anchor {
    pub fn parse(s: &str) -> Result<Anchor> {
        Ok(match s.trim().to_ascii_lowercase().as_str() {
            "tl" | "top-left" | "northwest" | "nw" => Anchor::Tl,
            "t" | "top" | "north" | "n" => Anchor::T,
            "tr" | "top-right" | "northeast" | "ne" => Anchor::Tr,
            "l" | "left" | "west" | "w" => Anchor::L,
            "c" | "center" | "centre" | "middle" => Anchor::C,
            "r" | "right" | "east" | "e" => Anchor::R,
            "bl" | "bottom-left" | "southwest" | "sw" => Anchor::Bl,
            "b" | "bottom" | "south" | "s" => Anchor::B,
            "br" | "bottom-right" | "southeast" | "se" => Anchor::Br,
            _ => return Err(Error::usage(format!("'{s}' is not an anchor (tl t tr l c r bl b br)"))),
        })
    }

    /// Fractions (0, 0.5, 1) of the box measured from its top-left.
    pub fn fractions(self) -> (f64, f64) {
        match self {
            Anchor::Tl => (0.0, 0.0),
            Anchor::T => (0.5, 0.0),
            Anchor::Tr => (1.0, 0.0),
            Anchor::L => (0.0, 0.5),
            Anchor::C => (0.5, 0.5),
            Anchor::R => (1.0, 0.5),
            Anchor::Bl => (0.0, 1.0),
            Anchor::B => (0.5, 1.0),
            Anchor::Br => (1.0, 1.0),
        }
    }

    /// Top-left corner of a `w`×`h` box whose anchor point sits at (`px`, `py`).
    pub fn place(self, px: f64, py: f64, w: f64, h: f64) -> (f64, f64) {
        let (fx, fy) = self.fractions();
        (px - fx * w, py - fy * h)
    }
}

/// `x1,y1 x2,y2 ...` (whitespace between points, a comma inside a point).
pub fn parse_points(s: &str) -> Result<Vec<(f64, f64)>> {
    let mut out = Vec::new();
    for tok in s.split_whitespace() {
        let (a, b) = tok.split_once(',').ok_or_else(|| geom_err(s, "point list ('x1,y1 x2,y2 ...')"))?;
        let x: f64 = a.trim().parse().map_err(|_| geom_err(s, "point list"))?;
        let y: f64 = b.trim().parse().map_err(|_| geom_err(s, "point list"))?;
        out.push((x, y));
    }
    if out.is_empty() {
        return Err(geom_err(s, "point list"));
    }
    Ok(out)
}

/// `a..b` numeric range (either side optional: `..0.5`, `0.2..`).
pub fn parse_range(s: &str) -> Result<(f64, f64)> {
    let (a, b) = s.split_once("..").ok_or_else(|| Error::usage(format!("'{s}' is not a range (a..b)")))?;
    let lo = if a.trim().is_empty() { f64::NEG_INFINITY } else { parse_num(a)? };
    let hi = if b.trim().is_empty() { f64::INFINITY } else { parse_num(b)? };
    Ok((lo, hi))
}

fn parse_num(s: &str) -> Result<f64> {
    let t = s.trim();
    if let Some(p) = t.strip_suffix('%') {
        return p.trim().parse::<f64>().map(|v| v / 100.0).map_err(|_| Error::usage(format!("'{s}' is not a number")));
    }
    t.parse::<f64>().map_err(|_| Error::usage(format!("'{s}' is not a number")))
}

/// Degrees: `45`, `45deg`, `0.5rad`, `0.25turn`.
pub fn parse_angle(s: &str) -> Result<f64> {
    let t = s.trim();
    let err = || Error::usage(format!("'{s}' is not an angle"));
    if let Some(r) = t.strip_suffix("rad") {
        return r.trim().parse::<f64>().map(|v| v.to_degrees()).map_err(|_| err());
    }
    if let Some(r) = t.strip_suffix("turn") {
        return r.trim().parse::<f64>().map(|v| v * 360.0).map_err(|_| err());
    }
    t.trim_end_matches("deg").trim_end_matches('°').trim().parse::<f64>().map_err(|_| err())
}

/// Seconds: `1.5`, `1.5s`, `500ms`, `2m`.
pub fn parse_duration_s(s: &str) -> Result<f64> {
    let t = s.trim();
    let err = || Error::usage(format!("'{s}' is not a duration"));
    if let Some(v) = t.strip_suffix("ms") {
        return v.trim().parse::<f64>().map(|x| x / 1000.0).map_err(|_| err());
    }
    if let Some(v) = t.strip_suffix('s') {
        return v.trim().parse::<f64>().map_err(|_| err());
    }
    if let Some(v) = t.strip_suffix('m') {
        return v.trim().parse::<f64>().map(|x| x * 60.0).map_err(|_| err());
    }
    t.parse::<f64>().map_err(|_| err())
}

/// Megabytes for `--mem-budget` (`4096`, `4GB`, `512MB`).
pub fn parse_megabytes(s: &str) -> Result<u64> {
    let t = s.trim().to_ascii_uppercase();
    let err = || Error::usage(format!("'{s}' is not a size in MB"));
    if let Some(v) = t.strip_suffix("GB").or_else(|| t.strip_suffix('G')) {
        return v.trim().parse::<f64>().map(|x| (x * 1024.0) as u64).map_err(|_| err());
    }
    if let Some(v) = t.strip_suffix("MB").or_else(|| t.strip_suffix('M')) {
        return v.trim().parse::<f64>().map(|x| x as u64).map_err(|_| err());
    }
    t.parse::<u64>().map_err(|_| err())
}

/// Split a comma list, trimming empties.
pub fn split_list(s: &str) -> Vec<String> {
    s.split(',').map(|p| p.trim().to_string()).filter(|p| !p.is_empty()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lengths_and_positions() {
        assert_eq!(Length::parse("50%").unwrap().to_px(200.0, 72.0), 100.0);
        assert_eq!(Length::parse("1in").unwrap().to_px(0.0, 300.0), 300.0);
        assert_eq!(Length::parse("72pt").unwrap().to_px(0.0, 144.0), 144.0);
        assert_eq!(Length::parse("-48").unwrap().resolve_pos(1000.0, 72.0), 952.0);
        assert!(Length::parse("abc").is_err());
    }

    #[test]
    fn rects_points_sizes() {
        let r = RectSpec::parse("10,20,300x200").unwrap().resolve(1000.0, 1000.0, 72.0);
        assert_eq!((r.x, r.y, r.w, r.h), (10.0, 20.0, 300.0, 200.0));
        let r = RectSpec::parse("0,840,3508x1842").unwrap().resolve(3508.0, 4961.0, 300.0);
        assert_eq!(r.w, 3508.0);
        let p = PointSpec::parse("50%,120").unwrap().resolve(1080.0, 1920.0, 72.0);
        assert_eq!(p, (540.0, 120.0));
        assert_eq!(SizeSpec::parse("2048x").unwrap().resolve(4000.0, 2000.0, 72.0), (2048, 1024));
        assert_eq!(SizeSpec::parse("x500").unwrap().resolve(4000.0, 2000.0, 72.0), (1000, 500));
        assert_eq!(SizeSpec::parse("50%").unwrap().resolve(400.0, 200.0, 72.0), (200, 100));
        assert!(RectSpec::parse("1,2,3").is_err());
    }

    #[test]
    fn anchors_place_boxes() {
        assert_eq!(Anchor::parse("br").unwrap().place(1000.0, 800.0, 100.0, 50.0), (900.0, 750.0));
        assert_eq!(Anchor::parse("c").unwrap().place(50.0, 50.0, 10.0, 10.0), (45.0, 45.0));
    }

    #[test]
    fn misc_parsers() {
        assert_eq!(parse_points("0,0 64,80 255,255").unwrap().len(), 3);
        assert_eq!(parse_range("0.2..0.8").unwrap(), (0.2, 0.8));
        assert_eq!(parse_angle("0.5turn").unwrap(), 180.0);
        assert_eq!(parse_duration_s("500ms").unwrap(), 0.5);
        assert_eq!(parse_megabytes("4GB").unwrap(), 4096);
    }
}
