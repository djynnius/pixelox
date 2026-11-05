//! The shape layer spec (AUTHORING.md "Shape layer spec") and the path helpers every
//! vector command shares: SVG path data ⇄ kurbo, kurbo → tiny-skia, flattening to
//! polygons, number formatting, and the geometry builders behind `shape *`.

use kurbo::{BezPath, PathEl, Point};
use pixelox_core::prelude::*;

/// Default anchor precision (decimals) for path data this crate writes.
pub const PREC: usize = 3;

/// A parsed shape layer spec. Absent keys take SVG defaults; `fill`/`stroke` absent or
/// `null` mean none.
#[derive(Clone, Debug)]
pub struct ShapeSpec {
    pub d: String,
    pub fill: Option<Rgba>,
    pub stroke: Option<Rgba>,
    pub stroke_width: f64,
    pub dash: Vec<f64>,
    pub dash_offset: f64,
    pub cap: String,
    pub join: String,
    pub miter_limit: f64,
    pub fill_rule: String,
    pub geometry: Value,
}

fn num(v: Option<&Value>, d: f64) -> f64 {
    v.and_then(|v| v.as_f64().or_else(|| v.as_str().and_then(|s| s.trim().parse().ok())))
        .filter(|x| x.is_finite())
        .unwrap_or(d)
}

fn color_of(v: Option<&Value>) -> Result<Option<Rgba>> {
    match v {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(s)) => parse_paint(s),
        Some(other) => Err(Error::new("PX1012", format!("shape color {other} is not a CSS color string"))),
    }
}

/// `none` (or a fully transparent color) → None; otherwise the parsed color.
pub fn parse_paint(s: &str) -> Result<Option<Rgba>> {
    let t = s.trim();
    if t.eq_ignore_ascii_case("none") || t.is_empty() {
        return Ok(None);
    }
    let c = color::parse_color(t)?;
    Ok(if c.a <= 0.0 { None } else { Some(c) })
}

impl ShapeSpec {
    pub fn new(d: String, geometry: Value) -> ShapeSpec {
        ShapeSpec {
            d,
            fill: None,
            stroke: None,
            stroke_width: 1.0,
            dash: Vec::new(),
            dash_offset: 0.0,
            cap: "butt".into(),
            join: "miter".into(),
            miter_limit: 4.0,
            fill_rule: "nonzero".into(),
            geometry,
        }
    }

    pub fn from_value(v: &Value) -> Result<ShapeSpec> {
        let s = |k: &str, d: &str| v.get(k).and_then(|x| x.as_str()).unwrap_or(d).to_string();
        let dash = match v.get("dash") {
            Some(Value::Array(a)) => a.iter().filter_map(|x| x.as_f64()).collect(),
            Some(Value::String(t)) => parse_dash(t)?,
            _ => Vec::new(),
        };
        Ok(ShapeSpec {
            d: s("d", ""),
            fill: color_of(v.get("fill"))?,
            stroke: color_of(v.get("stroke"))?,
            stroke_width: num(v.get("stroke_width"), 1.0).max(0.0),
            dash,
            dash_offset: num(v.get("dash_offset"), 0.0),
            cap: s("cap", "butt"),
            join: s("join", "miter"),
            miter_limit: num(v.get("miter_limit"), 4.0).max(1.0),
            fill_rule: s("fill_rule", "nonzero"),
            geometry: v.get("geometry").cloned().unwrap_or(Value::Null),
        })
    }

    /// Write this spec into `target` (an existing spec object keeps keys it does not own).
    pub fn write_into(&self, target: &mut Value) {
        if !target.is_object() {
            *target = json!({});
        }
        let paint = |c: Option<Rgba>| c.map(|c| Value::String(c.to_hex())).unwrap_or(Value::Null);
        let m = target.as_object_mut().expect("object");
        m.insert("d".into(), json!(self.d));
        m.insert("fill".into(), paint(self.fill));
        m.insert("stroke".into(), paint(self.stroke));
        m.insert("stroke_width".into(), json!(round(self.stroke_width, 4)));
        m.insert("dash".into(), json!(self.dash.iter().map(|v| round(*v, 4)).collect::<Vec<_>>()));
        m.insert("dash_offset".into(), json!(round(self.dash_offset, 4)));
        m.insert("cap".into(), json!(self.cap));
        m.insert("join".into(), json!(self.join));
        m.insert("miter_limit".into(), json!(round(self.miter_limit, 4)));
        m.insert("fill_rule".into(), json!(self.fill_rule));
        m.insert("geometry".into(), self.geometry.clone());
    }

    pub fn to_value(&self) -> Value {
        let mut v = json!({});
        self.write_into(&mut v);
        v
    }

    pub fn bez(&self) -> Result<BezPath> {
        parse_d(&self.d)
    }

    pub fn has_stroke(&self) -> bool {
        self.stroke.is_some() && self.stroke_width > 0.0
    }

    /// A tiny-skia stroke without dashes (dashing is applied to the path first).
    pub fn skia_stroke(&self) -> tiny_skia::Stroke {
        tiny_skia::Stroke {
            width: self.stroke_width as f32,
            miter_limit: self.miter_limit as f32,
            line_cap: match self.cap.as_str() {
                "round" => tiny_skia::LineCap::Round,
                "square" => tiny_skia::LineCap::Square,
                _ => tiny_skia::LineCap::Butt,
            },
            line_join: match self.join.as_str() {
                "round" => tiny_skia::LineJoin::Round,
                "bevel" => tiny_skia::LineJoin::Bevel,
                _ => tiny_skia::LineJoin::Miter,
            },
            dash: None,
        }
    }

    /// The SVG dash array with an even count (an odd list repeats, as in SVG).
    pub fn dash_even(&self) -> Option<Vec<f64>> {
        if self.dash.is_empty() || self.dash.iter().sum::<f64>() <= 0.0 || self.dash.iter().any(|v| *v < 0.0) {
            return None;
        }
        let mut d = self.dash.clone();
        if d.len() % 2 == 1 {
            d.extend(self.dash.clone());
        }
        Some(d)
    }

    pub fn kurbo_stroke(&self) -> kurbo::Stroke {
        let cap = match self.cap.as_str() {
            "round" => kurbo::Cap::Round,
            "square" => kurbo::Cap::Square,
            _ => kurbo::Cap::Butt,
        };
        let join = match self.join.as_str() {
            "round" => kurbo::Join::Round,
            "bevel" => kurbo::Join::Bevel,
            _ => kurbo::Join::Miter,
        };
        let mut s =
            kurbo::Stroke::new(self.stroke_width).with_caps(cap).with_join(join).with_miter_limit(self.miter_limit);
        if let Some(d) = self.dash_even() {
            s = s.with_dashes(self.dash_offset, d);
        }
        s
    }
}

/// Parse a dash list (`4,2`, `4 2`, or `none`); every value ≥ 0, sum > 0.
pub fn parse_dash(s: &str) -> Result<Vec<f64>> {
    let t = s.trim();
    if t.is_empty() || t.eq_ignore_ascii_case("none") {
        return Ok(Vec::new());
    }
    let vals: Vec<f64> = t
        .split(|c: char| c == ',' || c.is_whitespace())
        .filter(|p| !p.is_empty())
        .map(|p| p.trim_end_matches("px").parse::<f64>())
        .collect::<std::result::Result<_, _>>()
        .map_err(|_| {
            Error::new("PX1010", format!("--dash: '{s}' is not a list of numbers")).with_hint("use e.g. --dash '4,2'")
        })?;
    if vals.iter().any(|v| !v.is_finite() || *v < 0.0) || vals.iter().sum::<f64>() <= 0.0 {
        return Err(Error::new("PX1010", format!("--dash: '{s}' needs non-negative lengths with a positive sum")));
    }
    Ok(vals)
}

pub fn round(v: f64, prec: i32) -> f64 {
    let k = 10f64.powi(prec);
    let r = (v * k).round() / k;
    if r == 0.0 {
        0.0
    } else {
        r
    }
}

/// Format a number with at most `prec` decimals and no trailing zeros (`-0` → `0`).
pub fn fmt_num(v: f64, prec: usize) -> String {
    if !v.is_finite() {
        return "0".into();
    }
    let mut s = format!("{v:.prec$}");
    if s.contains('.') {
        while s.ends_with('0') {
            s.pop();
        }
        if s.ends_with('.') {
            s.pop();
        }
    }
    if s == "-0" {
        s = "0".into();
    }
    s
}

/// Parse SVG path data (PX1010 when invalid).
pub fn parse_d(d: &str) -> Result<BezPath> {
    if d.trim().is_empty() {
        return Ok(BezPath::new());
    }
    BezPath::from_svg(d).map_err(|e| {
        Error::new("PX1010", format!("invalid SVG path data: {e}"))
            .with_hint("path data looks like 'M 10 10 L 90 10 C 90 50 50 90 10 90 Z' (canvas px)")
            .with_context("d", d)
    })
}

fn pt(out: &mut String, p: Point, prec: usize) {
    out.push_str(&fmt_num(p.x, prec));
    out.push(' ');
    out.push_str(&fmt_num(p.y, prec));
}

/// Absolute SVG path data for a kurbo path (`M x y L x y C … Z`).
pub fn bez_to_d(p: &BezPath, prec: usize) -> String {
    let mut out = String::new();
    for el in p.elements() {
        if !out.is_empty() {
            out.push(' ');
        }
        match *el {
            PathEl::MoveTo(a) => {
                out.push_str("M ");
                pt(&mut out, a, prec);
            }
            PathEl::LineTo(a) => {
                out.push_str("L ");
                pt(&mut out, a, prec);
            }
            PathEl::QuadTo(a, b) => {
                out.push_str("Q ");
                pt(&mut out, a, prec);
                out.push(' ');
                pt(&mut out, b, prec);
            }
            PathEl::CurveTo(a, b, c) => {
                out.push_str("C ");
                pt(&mut out, a, prec);
                out.push(' ');
                pt(&mut out, b, prec);
                out.push(' ');
                pt(&mut out, c, prec);
            }
            PathEl::ClosePath => out.push('Z'),
        }
    }
    out
}

/// Path data for polygons (`M … L … Z` per contour).
pub fn polygons_to_d(contours: &[Vec<[f64; 2]>], prec: usize) -> String {
    let mut p = BezPath::new();
    for c in contours.iter().filter(|c| c.len() >= 2) {
        p.move_to((c[0][0], c[0][1]));
        for q in &c[1..] {
            p.line_to((q[0], q[1]));
        }
        p.close_path();
    }
    bez_to_d(&p, prec)
}

/// Convert to a tiny-skia path (None when empty).
pub fn to_skia(p: &BezPath) -> Option<tiny_skia::Path> {
    let mut b = tiny_skia::PathBuilder::new();
    let f = |v: f64| v as f32;
    for el in p.elements() {
        match *el {
            PathEl::MoveTo(a) => b.move_to(f(a.x), f(a.y)),
            PathEl::LineTo(a) => b.line_to(f(a.x), f(a.y)),
            PathEl::QuadTo(a, c) => b.quad_to(f(a.x), f(a.y), f(c.x), f(c.y)),
            PathEl::CurveTo(a, c, d) => b.cubic_to(f(a.x), f(a.y), f(c.x), f(c.y), f(d.x), f(d.y)),
            PathEl::ClosePath => b.close(),
        }
    }
    b.finish()
}

/// Flatten every subpath to a closed polygon (fill semantics: open subpaths close
/// implicitly). Degenerate contours (< 3 distinct points) are dropped.
pub fn flatten_contours(p: &BezPath, tolerance: f64) -> Vec<Vec<[f64; 2]>> {
    let mut out: Vec<Vec<[f64; 2]>> = Vec::new();
    let mut cur: Vec<[f64; 2]> = Vec::new();
    let mut start = [0.0, 0.0];
    let finish = |cur: &mut Vec<[f64; 2]>, out: &mut Vec<Vec<[f64; 2]>>| {
        let mut c = std::mem::take(cur);
        c.dedup_by(|a, b| (a[0] - b[0]).abs() < 1e-9 && (a[1] - b[1]).abs() < 1e-9);
        while c.len() > 1 && {
            let (f, l) = (c[0], c[c.len() - 1]);
            (f[0] - l[0]).abs() < 1e-9 && (f[1] - l[1]).abs() < 1e-9
        } {
            c.pop();
        }
        if c.len() >= 3 {
            out.push(c);
        }
    };
    kurbo::flatten(p.elements().iter().copied(), tolerance, |el| match el {
        PathEl::MoveTo(a) => {
            finish(&mut cur, &mut out);
            start = [a.x, a.y];
            cur.push(start);
        }
        PathEl::LineTo(a) => {
            if cur.is_empty() {
                cur.push(start);
            }
            cur.push([a.x, a.y]);
        }
        PathEl::ClosePath => finish(&mut cur, &mut out),
        _ => {}
    });
    finish(&mut cur, &mut out);
    out
}

/// Number of on-curve points (anchors) of a path.
pub fn anchor_count(p: &BezPath) -> usize {
    p.elements().iter().filter(|e| !matches!(e, PathEl::ClosePath)).count()
}

/// Tight bounding box `[x, y, w, h]` of a path, grown by half the stroke width.
pub fn spec_bbox(s: &ShapeSpec) -> Option<[f64; 4]> {
    use kurbo::Shape;
    let p = s.bez().ok()?;
    if p.elements().is_empty() {
        return None;
    }
    let r = p.bounding_box();
    let pad = if s.has_stroke() { s.stroke_width / 2.0 } else { 0.0 };
    Some([
        round(r.x0 - pad, 2),
        round(r.y0 - pad, 2),
        round(r.width() + 2.0 * pad, 2),
        round(r.height() + 2.0 * pad, 2),
    ])
}

// ---------------------------------------------------------------------------------------
// geometry builders (canvas px)

/// Rectangle with per-corner radii `[tl, tr, br, bl]` (clamped to half the short side).
pub fn rect_path(x: f64, y: f64, w: f64, h: f64, radii: [f64; 4]) -> BezPath {
    let (x0, x1) = (x.min(x + w), x.max(x + w));
    let (y0, y1) = (y.min(y + h), y.max(y + h));
    let lim = ((x1 - x0).min(y1 - y0) / 2.0).max(0.0);
    let [tl, tr, br, bl] = radii.map(|r| r.clamp(0.0, lim));
    let k = 0.552_284_749_830_793_6;
    let mut p = BezPath::new();
    p.move_to((x0 + tl, y0));
    p.line_to((x1 - tr, y0));
    if tr > 0.0 {
        p.curve_to((x1 - tr + tr * k, y0), (x1, y0 + tr - tr * k), (x1, y0 + tr));
    }
    p.line_to((x1, y1 - br));
    if br > 0.0 {
        p.curve_to((x1, y1 - br + br * k), (x1 - br + br * k, y1), (x1 - br, y1));
    }
    p.line_to((x0 + bl, y1));
    if bl > 0.0 {
        p.curve_to((x0 + bl - bl * k, y1), (x0, y1 - bl + bl * k), (x0, y1 - bl));
    }
    if tl > 0.0 {
        p.line_to((x0, y0 + tl));
        p.curve_to((x0, y0 + tl - tl * k), (x0 + tl - tl * k, y0), (x0 + tl, y0));
    }
    p.close_path();
    dedupe_lines(p)
}

/// Drop zero-length line segments (from square corners).
fn dedupe_lines(p: BezPath) -> BezPath {
    let mut out = BezPath::new();
    let mut last: Option<Point> = None;
    for el in p.elements() {
        match *el {
            PathEl::LineTo(a) if last.map(|l| (l - a).hypot() < 1e-9).unwrap_or(false) => {}
            PathEl::MoveTo(a) | PathEl::LineTo(a) | PathEl::QuadTo(_, a) | PathEl::CurveTo(_, _, a) => {
                out.push(*el);
                last = Some(a);
            }
            PathEl::ClosePath => out.push(*el),
        }
    }
    out
}

/// Ellipse inscribed in a rectangle (four cubic arcs).
pub fn ellipse_path(x: f64, y: f64, w: f64, h: f64) -> BezPath {
    let (cx, cy, rx, ry) = (x + w / 2.0, y + h / 2.0, (w / 2.0).abs(), (h / 2.0).abs());
    let k = 0.552_284_749_830_793_6;
    let (ox, oy) = (rx * k, ry * k);
    let mut p = BezPath::new();
    p.move_to((cx, cy - ry));
    p.curve_to((cx + ox, cy - ry), (cx + rx, cy - oy), (cx + rx, cy));
    p.curve_to((cx + rx, cy + oy), (cx + ox, cy + ry), (cx, cy + ry));
    p.curve_to((cx - ox, cy + ry), (cx - rx, cy + oy), (cx - rx, cy));
    p.curve_to((cx - rx, cy - oy), (cx - ox, cy - ry), (cx, cy - ry));
    p.close_path();
    p
}

fn polar(cx: f64, cy: f64, r: f64, deg: f64) -> (f64, f64) {
    let a = (deg - 90.0).to_radians();
    (cx + r * a.cos(), cy + r * a.sin())
}

/// Regular polygon; the first vertex points up (rotated by `rotation` degrees clockwise).
pub fn polygon_path(cx: f64, cy: f64, r: f64, sides: usize, rotation: f64) -> BezPath {
    let mut p = BezPath::new();
    for i in 0..sides {
        let v = polar(cx, cy, r, rotation + i as f64 * 360.0 / sides as f64);
        if i == 0 {
            p.move_to(v);
        } else {
            p.line_to(v);
        }
    }
    p.close_path();
    p
}

/// Star with `points` tips on radius `outer` and valleys on `inner`; first tip points up.
pub fn star_path(cx: f64, cy: f64, points: usize, outer: f64, inner: f64, rotation: f64) -> BezPath {
    let mut p = BezPath::new();
    let n = points * 2;
    for i in 0..n {
        let r = if i % 2 == 0 { outer } else { inner };
        let v = polar(cx, cy, r, rotation + i as f64 * 360.0 / n as f64);
        if i == 0 {
            p.move_to(v);
        } else {
            p.line_to(v);
        }
    }
    p.close_path();
    p
}

pub fn line_path(a: (f64, f64), b: (f64, f64)) -> BezPath {
    let mut p = BezPath::new();
    p.move_to(a);
    p.line_to(b);
    p
}

/// Arrow: an open shaft plus closed triangular head(s) of length `head` and width `head`.
pub fn arrow_path(a: (f64, f64), b: (f64, f64), head: f64, tail_head: bool) -> BezPath {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let len = (dx * dx + dy * dy).sqrt();
    let mut p = BezPath::new();
    if len < 1e-9 {
        return p;
    }
    let (ux, uy) = (dx / len, dy / len);
    let (nx, ny) = (-uy, ux);
    let heads = if tail_head { 2.0 } else { 1.0 };
    let head = head.min(len / heads).max(0.0);
    let half = head / 2.0;
    let s0 = if tail_head { (a.0 + ux * head, a.1 + uy * head) } else { a };
    let s1 = (b.0 - ux * head, b.1 - uy * head);
    p.move_to(s0);
    p.line_to(s1);
    let mut tri = |tip: (f64, f64), dir: f64| {
        let base = (tip.0 - dir * ux * head, tip.1 - dir * uy * head);
        p.move_to(tip);
        p.line_to((base.0 + nx * half, base.1 + ny * half));
        p.line_to((base.0 - nx * half, base.1 - ny * half));
        p.close_path();
    };
    tri(b, 1.0);
    if tail_head {
        tri(a, -1.0);
    }
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_format_compactly() {
        assert_eq!(fmt_num(10.0, 3), "10");
        assert_eq!(fmt_num(0.12345, 3), "0.123");
        assert_eq!(fmt_num(-0.0001, 3), "0");
        assert_eq!(fmt_num(2.5, 0), "2");
    }

    #[test]
    fn spec_round_trips_and_keeps_foreign_keys() {
        let mut s = ShapeSpec::new("M 0 0 L 10 0 L 10 10 Z".into(), json!({"type": "path"}));
        s.fill = Some(Rgba::from_u8(255, 212, 0, 255));
        s.dash = vec![4.0, 2.0];
        let mut v = json!({"owner_key": 1});
        s.write_into(&mut v);
        assert_eq!(v["owner_key"], 1);
        assert_eq!(v["fill"], "#ffd400");
        assert_eq!(v["stroke"], Value::Null);
        let back = ShapeSpec::from_value(&v).unwrap();
        assert_eq!(back.dash, vec![4.0, 2.0]);
        assert_eq!(back.fill.unwrap().to_hex(), "#ffd400");
    }

    #[test]
    fn dash_parsing() {
        assert_eq!(parse_dash("4,2").unwrap(), vec![4.0, 2.0]);
        assert_eq!(parse_dash("none").unwrap(), Vec::<f64>::new());
        assert!(parse_dash("4,-1").is_err());
        assert!(parse_dash("0,0").is_err());
    }

    #[test]
    fn flatten_drops_degenerate_and_closes() {
        let p = parse_d("M 0 0 L 10 0 L 10 10 Z M 5 5 L 6 6").unwrap();
        let c = flatten_contours(&p, 0.1);
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].len(), 3);
    }
}
