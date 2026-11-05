//! Photoshop's blend-mode set (PRD §14.3) and W3C/PDF source-over compositing with a
//! blend function, on straight-alpha sRGB-encoded `f32` pixels.

use crate::color::lum;
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Hash, Default)]
#[serde(rename_all = "kebab-case")]
pub enum BlendMode {
    PassThrough,
    #[default]
    Normal,
    Dissolve,
    Darken,
    Multiply,
    ColorBurn,
    LinearBurn,
    DarkerColor,
    Lighten,
    Screen,
    ColorDodge,
    LinearDodge,
    LighterColor,
    Overlay,
    SoftLight,
    HardLight,
    VividLight,
    LinearLight,
    PinLight,
    HardMix,
    Difference,
    Exclusion,
    Subtract,
    Divide,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

/// Every mode name as accepted on the command line, in Photoshop menu order.
pub const BLEND_MODES: &[&str] = &[
    "normal",
    "dissolve",
    "darken",
    "multiply",
    "color-burn",
    "linear-burn",
    "darker-color",
    "lighten",
    "screen",
    "color-dodge",
    "linear-dodge",
    "lighter-color",
    "overlay",
    "soft-light",
    "hard-light",
    "vivid-light",
    "linear-light",
    "pin-light",
    "hard-mix",
    "difference",
    "exclusion",
    "subtract",
    "divide",
    "hue",
    "saturation",
    "color",
    "luminosity",
    "pass-through",
];

impl BlendMode {
    pub fn parse(s: &str) -> Result<BlendMode> {
        let m = match s.trim().to_ascii_lowercase().replace(['_', ' '], "-").as_str() {
            "pass-through" | "passthrough" => BlendMode::PassThrough,
            "normal" => BlendMode::Normal,
            "dissolve" => BlendMode::Dissolve,
            "darken" => BlendMode::Darken,
            "multiply" => BlendMode::Multiply,
            "color-burn" => BlendMode::ColorBurn,
            "linear-burn" => BlendMode::LinearBurn,
            "darker-color" => BlendMode::DarkerColor,
            "lighten" => BlendMode::Lighten,
            "screen" => BlendMode::Screen,
            "color-dodge" => BlendMode::ColorDodge,
            "linear-dodge" | "add" => BlendMode::LinearDodge,
            "lighter-color" => BlendMode::LighterColor,
            "overlay" => BlendMode::Overlay,
            "soft-light" => BlendMode::SoftLight,
            "hard-light" => BlendMode::HardLight,
            "vivid-light" => BlendMode::VividLight,
            "linear-light" => BlendMode::LinearLight,
            "pin-light" => BlendMode::PinLight,
            "hard-mix" => BlendMode::HardMix,
            "difference" => BlendMode::Difference,
            "exclusion" => BlendMode::Exclusion,
            "subtract" => BlendMode::Subtract,
            "divide" => BlendMode::Divide,
            "hue" => BlendMode::Hue,
            "saturation" => BlendMode::Saturation,
            "color" => BlendMode::Color,
            "luminosity" => BlendMode::Luminosity,
            other => {
                return Err(Error::new("PX3101", format!("unknown blend mode '{other}'"))
                    .with_hint(format!("one of: {}", BLEND_MODES.join(" "))))
            }
        };
        Ok(m)
    }

    pub fn name(self) -> &'static str {
        match self {
            BlendMode::PassThrough => "pass-through",
            BlendMode::Normal => "normal",
            BlendMode::Dissolve => "dissolve",
            BlendMode::Darken => "darken",
            BlendMode::Multiply => "multiply",
            BlendMode::ColorBurn => "color-burn",
            BlendMode::LinearBurn => "linear-burn",
            BlendMode::DarkerColor => "darker-color",
            BlendMode::Lighten => "lighten",
            BlendMode::Screen => "screen",
            BlendMode::ColorDodge => "color-dodge",
            BlendMode::LinearDodge => "linear-dodge",
            BlendMode::LighterColor => "lighter-color",
            BlendMode::Overlay => "overlay",
            BlendMode::SoftLight => "soft-light",
            BlendMode::HardLight => "hard-light",
            BlendMode::VividLight => "vivid-light",
            BlendMode::LinearLight => "linear-light",
            BlendMode::PinLight => "pin-light",
            BlendMode::HardMix => "hard-mix",
            BlendMode::Difference => "difference",
            BlendMode::Exclusion => "exclusion",
            BlendMode::Subtract => "subtract",
            BlendMode::Divide => "divide",
            BlendMode::Hue => "hue",
            BlendMode::Saturation => "saturation",
            BlendMode::Color => "color",
            BlendMode::Luminosity => "luminosity",
        }
    }

    /// The 4-byte PSD key (§14.3).
    pub fn psd_key(self) -> &'static str {
        match self {
            BlendMode::PassThrough => "pass",
            BlendMode::Normal => "norm",
            BlendMode::Dissolve => "diss",
            BlendMode::Darken => "dark",
            BlendMode::Multiply => "mul ",
            BlendMode::ColorBurn => "idiv",
            BlendMode::LinearBurn => "lbrn",
            BlendMode::DarkerColor => "dkCl",
            BlendMode::Lighten => "lite",
            BlendMode::Screen => "scrn",
            BlendMode::ColorDodge => "div ",
            BlendMode::LinearDodge => "lddg",
            BlendMode::LighterColor => "lgCl",
            BlendMode::Overlay => "over",
            BlendMode::SoftLight => "sLit",
            BlendMode::HardLight => "hLit",
            BlendMode::VividLight => "vLit",
            BlendMode::LinearLight => "lLit",
            BlendMode::PinLight => "pLit",
            BlendMode::HardMix => "hMix",
            BlendMode::Difference => "diff",
            BlendMode::Exclusion => "smud",
            BlendMode::Subtract => "fsub",
            BlendMode::Divide => "fdiv",
            BlendMode::Hue => "hue ",
            BlendMode::Saturation => "sat ",
            BlendMode::Color => "colr",
            BlendMode::Luminosity => "lum ",
        }
    }

    pub fn from_psd_key(k: &str) -> Option<BlendMode> {
        BLEND_MODES.iter().filter_map(|n| BlendMode::parse(n).ok()).find(|m| m.psd_key() == k)
    }

    /// Separable modes act per channel.
    pub fn is_separable(self) -> bool {
        !matches!(
            self,
            BlendMode::Hue
                | BlendMode::Saturation
                | BlendMode::Color
                | BlendMode::Luminosity
                | BlendMode::DarkerColor
                | BlendMode::LighterColor
        )
    }
}

#[inline]
fn sep(mode: BlendMode, cb: f32, cs: f32) -> f32 {
    match mode {
        BlendMode::Normal | BlendMode::Dissolve | BlendMode::PassThrough => cs,
        BlendMode::Multiply => cb * cs,
        BlendMode::Screen => cb + cs - cb * cs,
        BlendMode::Overlay => hard_light(cs, cb),
        BlendMode::Darken => cb.min(cs),
        BlendMode::Lighten => cb.max(cs),
        BlendMode::ColorDodge => {
            if cb <= 0.0 {
                0.0
            } else if cs >= 1.0 {
                1.0
            } else {
                (cb / (1.0 - cs)).min(1.0)
            }
        }
        BlendMode::ColorBurn => {
            if cb >= 1.0 {
                1.0
            } else if cs <= 0.0 {
                0.0
            } else {
                1.0 - ((1.0 - cb) / cs).min(1.0)
            }
        }
        BlendMode::HardLight => hard_light(cb, cs),
        BlendMode::SoftLight => {
            // Photoshop's soft light (PRD §14.3: PS parity chosen)
            if cs <= 0.5 {
                2.0 * cb * cs + cb * cb * (1.0 - 2.0 * cs)
            } else {
                2.0 * cb * (1.0 - cs) + cb.max(0.0).sqrt() * (2.0 * cs - 1.0)
            }
        }
        BlendMode::Difference => (cb - cs).abs(),
        BlendMode::Exclusion => cb + cs - 2.0 * cb * cs,
        BlendMode::LinearBurn => (cb + cs - 1.0).max(0.0),
        BlendMode::LinearDodge => (cb + cs).min(1.0),
        BlendMode::VividLight => {
            if cs <= 0.5 {
                // color burn with 2*cs
                let s = 2.0 * cs;
                if cb >= 1.0 {
                    1.0
                } else if s <= 0.0 {
                    0.0
                } else {
                    1.0 - ((1.0 - cb) / s).min(1.0)
                }
            } else {
                // color dodge with 2*(cs-0.5)
                let s = 2.0 * (cs - 0.5);
                if cb <= 0.0 {
                    0.0
                } else if s >= 1.0 {
                    1.0
                } else {
                    (cb / (1.0 - s)).min(1.0)
                }
            }
        }
        BlendMode::LinearLight => (cb + 2.0 * cs - 1.0).clamp(0.0, 1.0),
        BlendMode::PinLight => {
            if cs <= 0.5 {
                cb.min(2.0 * cs)
            } else {
                cb.max(2.0 * cs - 1.0)
            }
        }
        BlendMode::HardMix => {
            if cb + cs >= 1.0 {
                1.0
            } else {
                0.0
            }
        }
        BlendMode::Subtract => (cb - cs).max(0.0),
        BlendMode::Divide => {
            if cs <= 0.0 {
                if cb <= 0.0 {
                    0.0
                } else {
                    1.0
                }
            } else {
                (cb / cs).min(1.0)
            }
        }
        _ => cs,
    }
}

#[inline]
fn hard_light(cb: f32, cs: f32) -> f32 {
    if cs <= 0.5 {
        cb * 2.0 * cs
    } else {
        let s = 2.0 * cs - 1.0;
        cb + s - cb * s
    }
}

#[inline]
fn clip_color(c: [f32; 3]) -> [f32; 3] {
    let l = lum(c[0], c[1], c[2]);
    let n = c[0].min(c[1]).min(c[2]);
    let x = c[0].max(c[1]).max(c[2]);
    let mut out = c;
    if n < 0.0 {
        for v in out.iter_mut() {
            *v = l + (*v - l) * l / (l - n).max(1e-7);
        }
    }
    if x > 1.0 {
        for v in out.iter_mut() {
            *v = l + (*v - l) * (1.0 - l) / (x - l).max(1e-7);
        }
    }
    out
}

#[inline]
fn set_lum(c: [f32; 3], l: f32) -> [f32; 3] {
    let d = l - lum(c[0], c[1], c[2]);
    clip_color([c[0] + d, c[1] + d, c[2] + d])
}

#[inline]
fn sat(c: [f32; 3]) -> f32 {
    c[0].max(c[1]).max(c[2]) - c[0].min(c[1]).min(c[2])
}

#[inline]
fn set_sat(c: [f32; 3], s: f32) -> [f32; 3] {
    let mx = c[0].max(c[1]).max(c[2]);
    let mn = c[0].min(c[1]).min(c[2]);
    if mx - mn <= 1e-7 {
        return [0.0; 3];
    }
    let mut out = [0.0; 3];
    for i in 0..3 {
        out[i] = (c[i] - mn) * s / (mx - mn);
    }
    out
}

/// The blend function B(Cb, Cs) for one pixel's color.
#[inline]
pub fn blend_color(mode: BlendMode, cb: [f32; 3], cs: [f32; 3]) -> [f32; 3] {
    if mode.is_separable() {
        return [sep(mode, cb[0], cs[0]), sep(mode, cb[1], cs[1]), sep(mode, cb[2], cs[2])];
    }
    match mode {
        BlendMode::Hue => set_lum(set_sat(cs, sat(cb)), lum(cb[0], cb[1], cb[2])),
        BlendMode::Saturation => set_lum(set_sat(cb, sat(cs)), lum(cb[0], cb[1], cb[2])),
        BlendMode::Color => set_lum(cs, lum(cb[0], cb[1], cb[2])),
        BlendMode::Luminosity => set_lum(cb, lum(cs[0], cs[1], cs[2])),
        BlendMode::DarkerColor => {
            if lum(cs[0], cs[1], cs[2]) < lum(cb[0], cb[1], cb[2]) {
                cs
            } else {
                cb
            }
        }
        BlendMode::LighterColor => {
            if lum(cs[0], cs[1], cs[2]) > lum(cb[0], cb[1], cb[2]) {
                cs
            } else {
                cb
            }
        }
        _ => cs,
    }
}

/// Composite `src` (with its alpha already scaled by opacity and masks) over `dst`
/// using `mode` (W3C Compositing Level 1 source-over with blending). Both straight alpha.
#[inline]
pub fn composite_pixel(mode: BlendMode, dst: &mut [f32], src: [f32; 4]) {
    let a_s = src[3];
    if a_s <= 0.0 {
        return;
    }
    let a_b = dst[3];
    let cs = [src[0], src[1], src[2]];
    let cb = [dst[0], dst[1], dst[2]];
    let mixed = if matches!(mode, BlendMode::Normal | BlendMode::Dissolve | BlendMode::PassThrough) || a_b <= 0.0 {
        cs
    } else {
        let b = blend_color(mode, cb, cs);
        [(1.0 - a_b) * cs[0] + a_b * b[0], (1.0 - a_b) * cs[1] + a_b * b[1], (1.0 - a_b) * cs[2] + a_b * b[2]]
    };
    let a_o = a_s + a_b * (1.0 - a_s);
    if a_o <= 1e-7 {
        dst.copy_from_slice(&[0.0, 0.0, 0.0, 0.0]);
        return;
    }
    for i in 0..3 {
        dst[i] = (a_s * mixed[i] + a_b * (1.0 - a_s) * cb[i]) / a_o;
    }
    dst[3] = a_o;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_parses_and_round_trips() {
        assert_eq!(BLEND_MODES.len(), 28);
        for n in BLEND_MODES {
            let m = BlendMode::parse(n).unwrap();
            assert_eq!(m.name(), *n);
            assert_eq!(BlendMode::from_psd_key(m.psd_key()), Some(m));
        }
        assert!(BlendMode::parse("sparkle").is_err());
    }

    #[test]
    fn known_values() {
        let b = [0.5, 0.25, 1.0];
        let s = [0.5, 0.5, 0.5];
        assert_eq!(blend_color(BlendMode::Multiply, b, s), [0.25, 0.125, 0.5]);
        assert_eq!(blend_color(BlendMode::Screen, b, s), [0.75, 0.625, 1.0]);
        assert_eq!(blend_color(BlendMode::Difference, b, s), [0.0, 0.25, 0.5]);
        // multiply by white is identity, screen with black is identity
        for v in [0.0f32, 0.3, 1.0] {
            assert_eq!(sep(BlendMode::Multiply, v, 1.0), v);
            assert_eq!(sep(BlendMode::Screen, v, 0.0), v);
        }
    }

    #[test]
    fn source_over_alpha() {
        let mut d = [0.0, 0.0, 1.0, 1.0];
        composite_pixel(BlendMode::Normal, &mut d, [1.0, 0.0, 0.0, 0.5]);
        assert!((d[0] - 0.5).abs() < 1e-6 && (d[2] - 0.5).abs() < 1e-6 && d[3] == 1.0);
        let mut t = [0.0, 0.0, 0.0, 0.0];
        composite_pixel(BlendMode::Multiply, &mut t, [0.2, 0.4, 0.6, 1.0]);
        assert_eq!(t, [0.2, 0.4, 0.6, 1.0], "blend modes act like normal over transparency");
    }

    #[test]
    fn multiply_commutes_on_opaque() {
        let a = [0.3, 0.7, 0.1];
        let b = [0.9, 0.2, 0.5];
        for m in [BlendMode::Multiply, BlendMode::Screen, BlendMode::Difference, BlendMode::Exclusion] {
            let x = blend_color(m, a, b);
            let y = blend_color(m, b, a);
            for i in 0..3 {
                assert!((x[i] - y[i]).abs() < 1e-6, "{m:?}");
            }
        }
    }
}
