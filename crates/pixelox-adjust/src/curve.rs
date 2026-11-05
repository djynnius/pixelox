//! Tone curves: a monotone cubic (Fritsch–Carlson) through control points on the 0–255
//! scale, and the `--curve file.json` reader.

use crate::util::num;
use pixelox_core::prelude::*;

/// A monotone cubic Hermite spline: segments that rise (or fall) in the control points
/// rise (or fall) in the curve too, so it never overshoots like a natural spline.
#[derive(Clone, Debug)]
pub struct Curve {
    xs: Vec<f32>,
    ys: Vec<f32>,
    ms: Vec<f32>,
}

impl Curve {
    /// Build from `(input, output)` points on the 0–255 scale. Inputs must strictly
    /// increase (PX3310); values outside 0..255 fail with PX1013.
    pub fn new(points: &[(f64, f64)]) -> Result<Curve> {
        if points.len() < 2 {
            return Err(Error::new("PX3310", format!("a curve needs at least two points, got {}", points.len()))
                .with_hint("pixelox adjust curves photo.jpg -o out.jpg --points '0,0 128,150 255,255'"));
        }
        for &(x, y) in points {
            if !(0.0..=255.0).contains(&x) || !(0.0..=255.0).contains(&y) {
                return Err(Error::new("PX1013", format!("curve point {},{} is outside 0..255", num(x, 3), num(y, 3)))
                    .with_context("point", [x, y]));
            }
        }
        for w in points.windows(2) {
            if w[1].0 <= w[0].0 {
                return Err(Error::new(
                    "PX3310",
                    format!(
                        "curve inputs must strictly increase: {} is followed by {}",
                        num(w[0].0, 3),
                        num(w[1].0, 3)
                    ),
                )
                .with_context("points", points.iter().map(|p| [p.0, p.1]).collect::<Vec<_>>())
                .with_hint("sort the points by input value and drop duplicates, e.g. --points '0,0 64,80 255,255'"));
            }
        }
        let xs: Vec<f32> = points.iter().map(|p| p.0 as f32).collect();
        let ys: Vec<f32> = points.iter().map(|p| p.1 as f32).collect();
        let n = xs.len();
        let d: Vec<f32> = (0..n - 1).map(|k| (ys[k + 1] - ys[k]) / (xs[k + 1] - xs[k])).collect();
        let mut ms = vec![0f32; n];
        ms[0] = d[0];
        ms[n - 1] = d[n - 2];
        for k in 1..n - 1 {
            ms[k] = if d[k - 1] * d[k] <= 0.0 { 0.0 } else { (d[k - 1] + d[k]) / 2.0 };
        }
        for k in 0..n - 1 {
            if d[k] == 0.0 {
                ms[k] = 0.0;
                ms[k + 1] = 0.0;
                continue;
            }
            let a = ms[k] / d[k];
            let b = ms[k + 1] / d[k];
            let s = a * a + b * b;
            if s > 9.0 {
                let t = 3.0 / s.sqrt();
                ms[k] = t * a * d[k];
                ms[k + 1] = t * b * d[k];
            }
        }
        Ok(Curve { xs, ys, ms })
    }

    /// Evaluate at `x` (0–255 scale); constant beyond the end points, like Photoshop.
    pub fn eval(&self, x: f32) -> f32 {
        let n = self.xs.len();
        if x <= self.xs[0] {
            return self.ys[0];
        }
        if x >= self.xs[n - 1] {
            return self.ys[n - 1];
        }
        let k = self.xs.windows(2).position(|w| x < w[1]).unwrap_or(n - 2);
        let h = self.xs[k + 1] - self.xs[k];
        let t = (x - self.xs[k]) / h;
        let (t2, t3) = (t * t, t * t * t);
        (2.0 * t3 - 3.0 * t2 + 1.0) * self.ys[k]
            + (t3 - 2.0 * t2 + t) * h * self.ms[k]
            + (-2.0 * t3 + 3.0 * t2) * self.ys[k + 1]
            + (t3 - t2) * h * self.ms[k + 1]
    }

    /// Map an encoded 0..1 value through the curve.
    #[inline]
    pub fn map01(&self, v: f32) -> f32 {
        self.eval(v * 255.0) / 255.0
    }
}

/// Points as the `--points` syntax (`0,0 64,80 255,255`).
pub fn format_points(points: &[(f64, f64)]) -> String {
    points.iter().map(|(x, y)| format!("{},{}", num(*x, 3), num(*y, 3))).collect::<Vec<_>>().join(" ")
}

fn bad_file(msg: impl Into<String>) -> Error {
    Error::new("PX1010", msg.into())
        .with_hint("a curve file is JSON: [[0,0],[64,80],[255,255]], {\"points\": [[0,0],[255,255]]}, or {\"points\": \"0,0 255,255\"}")
}

fn point_of(v: &Value) -> Option<(f64, f64)> {
    match v {
        Value::Array(a) if a.len() == 2 => Some((a[0].as_f64()?, a[1].as_f64()?)),
        Value::Object(o) => Some((o.get("x")?.as_f64()?, o.get("y")?.as_f64()?)),
        _ => None,
    }
}

/// Read the points of a curve file: `[[x,y],...]`, `[{"x":..,"y":..},...]`, or an object
/// with `points` holding either form or a `--points` string.
pub fn parse_curve_json(bytes: &[u8]) -> Result<Vec<(f64, f64)>> {
    let v: Value =
        serde_json::from_slice(bytes).map_err(|e| bad_file(format!("the curve file is not valid JSON: {e}")))?;
    let pts = match &v {
        Value::Object(o) => o.get("points").ok_or_else(|| bad_file("the curve file has no \"points\" key"))?,
        other => other,
    };
    match pts {
        Value::String(s) => units::parse_points(s),
        Value::Array(items) => items
            .iter()
            .map(|it| point_of(it).ok_or_else(|| bad_file(format!("'{it}' is not a curve point"))))
            .collect(),
        other => Err(bad_file(format!("\"points\" must be a list of [x, y] pairs, got {other}"))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes_through_points_and_stays_monotone() {
        let c = Curve::new(&[(0.0, 0.0), (64.0, 80.0), (192.0, 200.0), (255.0, 255.0)]).unwrap();
        assert!((c.eval(64.0) - 80.0).abs() < 1e-4);
        assert!((c.eval(192.0) - 200.0).abs() < 1e-4);
        let mut prev = -1.0;
        for i in 0..=255 {
            let y = c.eval(i as f32);
            assert!(y >= prev - 1e-4, "not monotone at {i}");
            prev = y;
        }
        // flat segments stay flat (no overshoot)
        let f = Curve::new(&[(0.0, 0.0), (100.0, 100.0), (150.0, 100.0), (255.0, 255.0)]).unwrap();
        for i in 100..=150 {
            assert!((f.eval(i as f32) - 100.0).abs() < 1e-3);
        }
        // two points: a straight line, constant outside
        let l = Curve::new(&[(50.0, 0.0), (150.0, 255.0)]).unwrap();
        assert!((l.eval(100.0) - 127.5).abs() < 1e-3);
        assert_eq!(l.eval(10.0), 0.0);
        assert_eq!(l.eval(200.0), 255.0);
    }

    #[test]
    fn rejects_bad_points() {
        assert_eq!(Curve::new(&[(0.0, 0.0), (0.0, 10.0)]).unwrap_err().code, "PX3310");
        assert_eq!(Curve::new(&[(10.0, 0.0), (5.0, 10.0)]).unwrap_err().code, "PX3310");
        assert_eq!(Curve::new(&[(0.0, 0.0)]).unwrap_err().code, "PX3310");
        assert_eq!(Curve::new(&[(0.0, 0.0), (300.0, 10.0)]).unwrap_err().code, "PX1013");
    }

    #[test]
    fn reads_every_file_shape() {
        let want = vec![(0.0, 0.0), (64.0, 80.0), (255.0, 255.0)];
        assert_eq!(parse_curve_json(b"[[0,0],[64,80],[255,255]]").unwrap(), want);
        assert_eq!(parse_curve_json(br#"{"points": [[0,0],[64,80],[255,255]]}"#).unwrap(), want);
        assert_eq!(parse_curve_json(br#"{"points": "0,0 64,80 255,255"}"#).unwrap(), want);
        assert_eq!(parse_curve_json(br#"[{"x":0,"y":0},{"x":64,"y":80},{"x":255,"y":255}]"#).unwrap(), want);
        assert_eq!(parse_curve_json(b"{\"nope\": 1}").unwrap_err().code, "PX1010");
        assert_eq!(parse_curve_json(b"not json").unwrap_err().code, "PX1010");
        assert_eq!(format_points(&want), "0,0 64,80 255,255");
    }
}
