//! Color adjustments: hue/saturation (with per-range targeting and colorize), color
//! balance, vibrance, white balance, selective color, channel mixer, gradient map, and
//! black & white. Same conventions as `tone.rs`: f32 on 0..1, alpha untouched.

use crate::adjust::{pixel_job, Job};
use crate::util::{clamp01, per_row, round_to, sum_rows, triple};
use pixelox_core::color::{hsl_to_rgb, rgb_to_hsl, srgb_to_linear, Interp};
use pixelox_core::prelude::*;

/// Hue centers of Photoshop's color ranges (degrees).
fn range_center(range: &str) -> Option<f32> {
    Some(match range {
        "reds" => 0.0,
        "yellows" => 60.0,
        "greens" => 120.0,
        "cyans" => 180.0,
        "blues" => 240.0,
        "magentas" => 300.0,
        _ => return None,
    })
}

/// Membership of hue `h` in the range centered at `c`: full within ±15°, fading out by
/// ±45° (Photoshop's default range and falloff).
fn range_weight(h: f32, c: f32) -> f32 {
    let d = (h - c).rem_euclid(360.0);
    let d = d.min(360.0 - d);
    if d <= 15.0 {
        1.0
    } else if d < 45.0 {
        (45.0 - d) / 30.0
    } else {
        0.0
    }
}

fn lighten(v: f32, l: f32) -> f32 {
    if l >= 0.0 {
        v + (1.0 - v) * l
    } else {
        v * (1.0 + l)
    }
}

pub fn hsl(a: &Args) -> Result<Job> {
    let range = a.choice("range");
    let hue = a.f32("hue")?;
    let sat = a.f32("saturation")? / 100.0;
    let light = a.f32("lightness")? / 100.0;
    if a.flag("colorize") {
        if range != "master" {
            return Err(Error::new("PX1005", "--colorize works on the master range only").with_hint(
                "drop --range, e.g. pixelox adjust hsl photo.jpg -o out.jpg --colorize --hue 30 --saturation 40",
            ));
        }
        let th = hue.rem_euclid(360.0);
        let ts = if sat == 0.0 { 0.25 } else { sat.max(0.0) };
        return Ok(pixel_job(move |p| {
            let l = lighten(color::luma(p[0], p[1], p[2]), light);
            let (r, g, b) = hsl_to_rgb(th, ts, clamp01(l));
            [r, g, b]
        }));
    }
    let center = range_center(&range);
    Ok(pixel_job(move |p| {
        let (h, s, l) = rgb_to_hsl(p[0], p[1], p[2]);
        let w = match center {
            None => 1.0,
            Some(c) => {
                let chroma = p[0].max(p[1]).max(p[2]) - p[0].min(p[1]).min(p[2]);
                range_weight(h, c) * (chroma * 5.0).min(1.0)
            }
        };
        if w <= 0.0 {
            return p;
        }
        let (r, g, b) = hsl_to_rgb(h + hue * w, clamp01(s * (1.0 + sat * w)), l);
        [r, g, b].map(|v| lighten(clamp01(v), light * w))
    }))
}

/// GIMP's color-balance transfer masks (shadows, midtones, highlights) at lightness `l`;
/// they sum to 0.7 everywhere, so equal corrections in all three are uniform.
fn balance_weights(l: f32) -> [f32; 3] {
    const A: f32 = 0.25;
    const B: f32 = 0.333;
    const SCALE: f32 = 0.7;
    let s = ((l - B) / -A + 0.5).clamp(0.0, 1.0) * SCALE;
    let m = ((l - B) / A + 0.5).clamp(0.0, 1.0) * ((l + B - 1.0) / -A + 0.5).clamp(0.0, 1.0) * SCALE;
    let h = ((l + B - 1.0) / A + 0.5).clamp(0.0, 1.0) * SCALE;
    [s, m, h]
}

pub fn color_balance(a: &Args) -> Result<Job> {
    let ranges = [
        triple(a, "shadows", -100.0, 100.0)?,
        triple(a, "midtones", -100.0, 100.0)?,
        triple(a, "highlights", -100.0, 100.0)?,
    ]
    .map(|t| t.map(|v| v / 100.0));
    let preserve = a.flag("preserve-luma");
    Ok(pixel_job(move |p| {
        let (_, _, l) = rgb_to_hsl(p[0], p[1], p[2]);
        let w = balance_weights(l);
        let out = [0, 1, 2].map(|c| clamp01(p[c] + ranges[0][c] * w[0] + ranges[1][c] * w[1] + ranges[2][c] * w[2]));
        if preserve {
            let (h, s, _) = rgb_to_hsl(out[0], out[1], out[2]);
            let (r, g, b) = hsl_to_rgb(h, s, l);
            [r, g, b]
        } else {
            out
        }
    }))
}

/// Scale chroma around luma by `f`, limited so no channel leaves 0..1 (keeps hue).
fn scale_chroma(p: [f32; 3], f: f32) -> [f32; 3] {
    let l = color::luma(p[0], p[1], p[2]);
    let mut f = f.max(0.0);
    if f > 1.0 {
        for v in p {
            let d = v - l;
            if d > 1e-6 {
                f = f.min((1.0 - l) / d);
            } else if d < -1e-6 {
                f = f.min(l / -d);
            }
        }
        f = f.max(1.0);
    }
    p.map(|v| l + (v - l) * f)
}

pub fn vibrance(a: &Args) -> Result<Job> {
    let vib = a.f32("vibrance")? / 100.0;
    let sat = a.f32("saturation")? / 100.0;
    Ok(pixel_job(move |p| {
        let mx = p[0].max(p[1]).max(p[2]);
        let mn = p[0].min(p[1]).min(p[2]);
        let s = if mx > 1e-6 { (mx - mn) / mx } else { 0.0 };
        let mut f = 1.0;
        if vib > 0.0 {
            // protect skin tones (orange hues) the way Photoshop's vibrance does
            let (h, _, _) = rgb_to_hsl(p[0], p[1], p[2]);
            let skin = if (20.0..=50.0).contains(&h) {
                1.0
            } else if (10.0..20.0).contains(&h) {
                (h - 10.0) / 10.0
            } else if h > 50.0 && h < 60.0 {
                (60.0 - h) / 10.0
            } else {
                0.0
            };
            f += vib * (1.0 - s) * (1.0 - s) * (1.0 - 0.5 * skin);
        } else if vib < 0.0 {
            f += vib * (1.0 - s);
        }
        f *= 1.0 + sat;
        if f == 1.0 {
            p
        } else {
            scale_chroma(p, f)
        }
    }))
}

/// Log-gain strength of `--temp` (red against blue) and `--tint` (green against magenta).
const TEMP_K: f32 = 0.5;
const TINT_K: f32 = 0.4;

/// Linear-light channel gains for temperature/tint (−100..100 each), normalized so that
/// a neutral gray keeps its luminance.
pub fn wb_gains(temp: f32, tint: f32) -> [f32; 3] {
    let (t, n) = (temp / 100.0, tint / 100.0);
    let l = [TEMP_K * t + TINT_K * n / 2.0, -TINT_K * n, -TEMP_K * t + TINT_K * n / 2.0];
    let g = l.map(f32::exp);
    let k = 0.2126 * g[0] + 0.7152 * g[1] + 0.0722 * g[2];
    g.map(|v| v / k)
}

/// The temperature/tint that turns a linear-light color neutral (unclamped).
pub fn wb_neutralize(lin: [f64; 3]) -> (f64, f64) {
    let l = lin.map(|v| v.max(1e-6).ln());
    let t = (l[2] - l[0]) / (2.0 * TEMP_K as f64);
    let n = (l[1] - (l[0] + l[2]) / 2.0) / (1.5 * TINT_K as f64);
    (t * 100.0, n * 100.0)
}

pub fn white_balance(a: &Args) -> Result<Job> {
    let g = wb_gains(a.f32("temp")?, a.f32("tint")?);
    Ok(pixel_job(move |p| [0, 1, 2].map(|c| color::linear_to_srgb((srgb_to_linear(p[c]) * g[c]).min(1.0)))))
}

/// Gray-world estimate of the illuminant: mean linear color of well-exposed, not
/// strongly saturated pixels (fixed-order sums).
pub fn gray_world(img: &Image) -> Option<[f64; 3]> {
    let pass = |strict: bool| {
        let rows = per_row(img, |_, row| {
            let mut acc = [0.0f64; 4];
            for p in row.chunks_exact(4) {
                if p[3] < 0.5 {
                    continue;
                }
                if strict {
                    let l = color::luma(p[0], p[1], p[2]);
                    let mx = p[0].max(p[1]).max(p[2]);
                    if !(0.05..=0.95).contains(&l) || mx >= 0.98 {
                        continue;
                    }
                }
                for c in 0..3 {
                    acc[c] += srgb_to_linear(p[c]) as f64;
                }
                acc[3] += 1.0;
            }
            acc
        });
        let s = sum_rows(&rows);
        (s[3] > 0.0).then(|| [s[0] / s[3], s[1] / s[3], s[2] / s[3]])
    };
    pass(true).or_else(|| pass(false))
}

/// Mean color of a 3×3 neighborhood (opaque-enough pixels) around image pixel (x, y).
pub fn sample_point(img: &Image, x: i64, y: i64) -> Option<[f32; 3]> {
    if x < 0 || y < 0 || x >= img.width as i64 || y >= img.height as i64 {
        return None;
    }
    let mut acc = [0f32; 3];
    let mut n = 0.0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            let (sx, sy) = (x + dx, y + dy);
            if sx < 0 || sy < 0 || sx >= img.width as i64 || sy >= img.height as i64 {
                continue;
            }
            let p = img.get(sx as u32, sy as u32);
            if p[3] <= 0.0 {
                continue;
            }
            for c in 0..3 {
                acc[c] += p[c];
            }
            n += 1.0;
        }
    }
    (n > 0.0).then(|| acc.map(|v| v / n))
}

/// Temperature and tint (rounded to 0.01, clamped to ±100) that neutralize `lin`, and
/// whether clamping was needed.
pub fn wb_params_for(lin: [f64; 3]) -> (f64, f64, bool) {
    let (t, n) = wb_neutralize(lin);
    let clamped = t.abs() > 100.0 || n.abs() > 100.0;
    (round_to(t.clamp(-100.0, 100.0), 2), round_to(n.clamp(-100.0, 100.0), 2), clamped)
}

/// Selective-color membership of a pixel in `target`.
fn selective_weight(target: &str, p: [f32; 3]) -> f32 {
    let [r, g, b] = p;
    let mx = r.max(g).max(b);
    let mn = r.min(g).min(b);
    let mid = r + g + b - mx - mn;
    match target {
        "reds" if r >= g && r >= b => mx - mid,
        "greens" if g >= r && g >= b => mx - mid,
        "blues" if b >= r && b >= g => mx - mid,
        "yellows" if b <= r && b <= g => mid - mn,
        "cyans" if r <= g && r <= b => mid - mn,
        "magentas" if g <= r && g <= b => mid - mn,
        "whites" if mn > 0.5 => (mn - 0.5) * 2.0,
        "blacks" if mx < 0.5 => (0.5 - mx) * 2.0,
        "neutrals" => (1.0 - ((mx - 0.5).abs() + (mn - 0.5).abs())).clamp(0.0, 1.0),
        _ => 0.0,
    }
}

pub fn selective_color(a: &Args) -> Result<Job> {
    let target = a.choice("target");
    let cmy = [a.f32("c")? / 100.0, a.f32("m")? / 100.0, a.f32("y")? / 100.0];
    let k = a.f32("k")? / 100.0;
    let relative = a.flag("relative");
    Ok(pixel_job(move |p| {
        let w = selective_weight(&target, p);
        if w <= 0.0 {
            return p;
        }
        // cyan acts on red, magenta on green, yellow on blue; black on all three
        [0, 1, 2].map(|c| {
            let ink = (-1.0 - cmy[c]) * k - cmy[c];
            let d = if relative { ink * (1.0 - p[c]) } else { ink };
            p[c] + d * w
        })
    }))
}

pub fn channel_mixer(a: &Args) -> Result<Job> {
    let mono = a.flag("monochrome");
    let out = match a.choice("out").as_str() {
        "g" => 1,
        "b" => 2,
        _ => 0,
    };
    let default = |c: usize| -> f32 {
        if mono {
            [0.4, 0.4, 0.2][c]
        } else if c == out {
            1.0
        } else {
            0.0
        }
    };
    let mut w = [0f32; 3];
    for (c, name) in ["r", "g", "b"].iter().enumerate() {
        w[c] = match a.opt_f64(name)? {
            Some(v) => v as f32 / 100.0,
            None => default(c),
        };
    }
    let k = a.f32("constant")? / 100.0;
    Ok(pixel_job(move |p| {
        let v = w[0] * p[0] + w[1] * p[1] + w[2] * p[2] + k;
        if mono {
            [v; 3]
        } else {
            let mut o = p;
            o[out] = v;
            o
        }
    }))
}

pub fn gradient_map(a: &Args) -> Result<Job> {
    let stops = a.stops("stops")?;
    let interp = Interp::parse(&a.choice("interp"))?;
    Ok(pixel_job(move |p| {
        let c = color::sample_stops(&stops, clamp01(color::luma(p[0], p[1], p[2])), interp);
        let t = c.a;
        [p[0] + (c.r - p[0]) * t, p[1] + (c.g - p[1]) * t, p[2] + (c.b - p[2]) * t]
    }))
}

/// Black & white presets: weights for reds, yellows, greens, cyans, blues, magentas (%).
pub const BW_PRESETS: &[(&str, [f32; 6])] = &[
    ("default", [40.0, 60.0, 40.0, 60.0, 20.0, 80.0]),
    ("red-filter", [120.0, 110.0, -10.0, -50.0, -50.0, 120.0]),
    ("yellow-filter", [120.0, 110.0, 40.0, -30.0, -50.0, 90.0]),
    ("green-filter", [-20.0, 70.0, 120.0, 60.0, -20.0, -10.0]),
    ("blue-filter", [-30.0, -30.0, 30.0, 100.0, 120.0, 60.0]),
    ("high-contrast-red", [120.0, 120.0, -10.0, -50.0, -50.0, 120.0]),
    ("high-contrast-blue", [-50.0, -50.0, 0.0, 100.0, 150.0, 100.0]),
    ("infrared", [-40.0, 235.0, 144.0, -68.0, -3.0, -107.0]),
    ("darker", [20.0, 40.0, 20.0, 40.0, 0.0, 60.0]),
    ("lighter", [60.0, 80.0, 60.0, 80.0, 40.0, 100.0]),
    ("maximum-black", [0.0; 6]),
    ("maximum-white", [100.0; 6]),
];

pub const BW_PRESET_NAMES: &[&str] = &[
    "default",
    "red-filter",
    "yellow-filter",
    "green-filter",
    "blue-filter",
    "high-contrast-red",
    "high-contrast-blue",
    "infrared",
    "darker",
    "lighter",
    "maximum-black",
    "maximum-white",
];

/// Gray value of a pixel under black & white weights `[r, y, g, c, b, m]` (fractions):
/// `min + (max − mid)·primary + (mid − min)·secondary`, Photoshop's model, which keeps
/// neutrals neutral.
pub fn bw_gray(p: [f32; 3], w: &[f32; 6]) -> f32 {
    let [r, g, b] = p;
    // order channels: (value, index)
    let mut ch = [(r, 0usize), (g, 1), (b, 2)];
    ch.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal).then(a.1.cmp(&b.1)));
    let (mx, imx) = ch[0];
    let (mid, imid) = ch[1];
    let mn = ch[2].0;
    // primaries: r → reds(0), g → greens(2), b → blues(4)
    let primary = [0, 2, 4][imx];
    // secondaries: r+g → yellows(1), g+b → cyans(3), r+b → magentas(5)
    let secondary = match (imx.min(imid), imx.max(imid)) {
        (0, 1) => 1,
        (1, 2) => 3,
        _ => 5,
    };
    mn + (mx - mid) * w[primary] + (mid - mn) * w[secondary]
}

pub fn bw(a: &Args) -> Result<Job> {
    let preset = a.choice("preset");
    let mut w = BW_PRESETS.iter().find(|(n, _)| *n == preset).map(|(_, w)| *w).unwrap_or(BW_PRESETS[0].1);
    for (i, name) in ["r", "y", "g", "c", "b", "m"].iter().enumerate() {
        if let Some(v) = a.opt_f64(name)? {
            w[i] = v as f32;
        }
    }
    let w = w.map(|v| v / 100.0);
    Ok(pixel_job(move |p| [bw_gray(p, &w); 3]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranges_fade() {
        assert_eq!(range_weight(10.0, 0.0), 1.0);
        assert_eq!(range_weight(350.0, 0.0), 1.0);
        assert!((range_weight(30.0, 0.0) - 0.5).abs() < 1e-6);
        assert_eq!(range_weight(120.0, 0.0), 0.0);
    }

    #[test]
    fn balance_masks_sum_to_scale() {
        for l in [0.0f32, 0.2, 0.5, 0.8, 1.0] {
            let w = balance_weights(l);
            assert!((w.iter().sum::<f32>() - 0.7).abs() < 1e-4, "{l}: {w:?}");
        }
        assert!(balance_weights(0.05)[0] > 0.6);
        assert!(balance_weights(0.95)[2] > 0.6);
    }

    #[test]
    fn white_balance_round_trip() {
        let g = wb_gains(0.0, 0.0);
        assert!(g.iter().all(|v| (v - 1.0).abs() < 1e-6));
        // a warm cast is neutralized by the detected temperature/tint
        let cast = [0.5f64, 0.4, 0.25];
        let (t, n, clamped) = wb_params_for(cast);
        assert!(!clamped);
        assert!(t < 0.0, "a warm cast needs a cooler temperature, got {t}");
        let g = wb_gains(t as f32, n as f32);
        let out = [cast[0] as f32 * g[0], cast[1] as f32 * g[1], cast[2] as f32 * g[2]];
        assert!((out[0] - out[1]).abs() < 2e-3 && (out[1] - out[2]).abs() < 2e-3, "{out:?}");
    }

    #[test]
    fn bw_model() {
        let w = BW_PRESETS[0].1.map(|v| v / 100.0);
        assert!((bw_gray([1.0, 0.0, 0.0], &w) - 0.4).abs() < 1e-6);
        assert!((bw_gray([1.0, 1.0, 0.0], &w) - 0.6).abs() < 1e-6);
        assert!((bw_gray([0.0, 0.0, 1.0], &w) - 0.2).abs() < 1e-6);
        assert!((bw_gray([0.3, 0.3, 0.3], &w) - 0.3).abs() < 1e-6);
        assert_eq!(BW_PRESETS.len(), BW_PRESET_NAMES.len());
        for (a, b) in BW_PRESETS.iter().zip(BW_PRESET_NAMES) {
            assert_eq!(a.0, *b);
        }
    }

    #[test]
    fn selective_weights() {
        assert!((selective_weight("reds", [1.0, 0.2, 0.0]) - 0.8).abs() < 1e-6);
        assert_eq!(selective_weight("greens", [1.0, 0.2, 0.0]), 0.0);
        assert!((selective_weight("yellows", [1.0, 0.8, 0.0]) - 0.8).abs() < 1e-6);
        assert_eq!(selective_weight("neutrals", [0.5, 0.5, 0.5]), 1.0);
        assert_eq!(selective_weight("neutrals", [1.0, 0.0, 0.0]), 0.0);
        assert!((selective_weight("whites", [0.9, 0.9, 0.9]) - 0.8).abs() < 1e-6);
        assert!((selective_weight("blacks", [0.1, 0.1, 0.1]) - 0.8).abs() < 1e-6);
    }
}
