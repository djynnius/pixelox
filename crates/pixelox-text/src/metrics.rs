//! Measured face metrics for substitution: how wide (mean advance of the capitals of
//! `HAMBURGEFONTSIV` over the cap height) and how dark (ink area over advance x cap height)
//! a face sets. They rank installed faces when no member of a class is installed.

use rustybuzz::ttf_parser::{self, GlyphId, OutlineBuilder, Tag};
use tiny_skia::{FillRule, Mask, PathBuilder, Transform};

/// The capitals measured.
pub const SAMPLE: &str = "HAMBURGEFONTSIV";
/// Cap height of the darkness raster, px.
const CAP_PX: f32 = 48.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Metrics {
    /// Mean advance of the sample capitals / cap height (Impact about 0.55, Arial 0.75).
    pub width: f32,
    /// Ink area / (advance x cap height): 0.2 for hairlines, 0.6 for black faces.
    pub darkness: f32,
}

impl Metrics {
    /// Distance to a target: relative width difference plus darkness difference, weighted so
    /// that a 15% width error costs as much as 0.1 of darkness.
    pub fn distance(&self, t: (f32, f32)) -> f32 {
        let dw = (self.width / t.0.max(1e-3)).ln().abs() / 0.15;
        let dd = (self.darkness - t.1).abs() / 0.1;
        dw + dd
    }
}

struct Sink {
    b: PathBuilder,
    sx: f32,
    sy: f32,
    dx: f32,
    dy: f32,
}

impl OutlineBuilder for Sink {
    fn move_to(&mut self, x: f32, y: f32) {
        self.b.move_to(x * self.sx + self.dx, self.dy - y * self.sy);
    }
    fn line_to(&mut self, x: f32, y: f32) {
        self.b.line_to(x * self.sx + self.dx, self.dy - y * self.sy);
    }
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.b.quad_to(x1 * self.sx + self.dx, self.dy - y1 * self.sy, x * self.sx + self.dx, self.dy - y * self.sy);
    }
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.b.cubic_to(
            x1 * self.sx + self.dx,
            self.dy - y1 * self.sy,
            x2 * self.sx + self.dx,
            self.dy - y2 * self.sy,
            x * self.sx + self.dx,
            self.dy - y * self.sy,
        );
    }
    fn close(&mut self) {
        self.b.close();
    }
}

struct Bbox;
impl OutlineBuilder for Bbox {
    fn move_to(&mut self, _: f32, _: f32) {}
    fn line_to(&mut self, _: f32, _: f32) {}
    fn quad_to(&mut self, _: f32, _: f32, _: f32, _: f32) {}
    fn curve_to(&mut self, _: f32, _: f32, _: f32, _: f32, _: f32, _: f32) {}
    fn close(&mut self) {}
}

/// Measure face `index` of `data` at the given variation axis values (`wght`, `wdth`, ...).
pub fn measure(data: &[u8], index: u32, vars: &[(Tag, f32)]) -> Option<Metrics> {
    let mut face = ttf_parser::Face::parse(data, index).ok()?;
    for (t, v) in vars {
        if face.variation_axes().into_iter().any(|a| a.tag == *t) {
            face.set_variation(*t, *v);
        }
    }
    let h = face.glyph_index('H')?;
    let hb = face.outline_glyph(h, &mut Bbox)?;
    let cap = (hb.y_max as f32 - hb.y_min.min(0) as f32).max(1.0);
    let k = CAP_PX / cap;
    let glyphs: Vec<GlyphId> = SAMPLE.chars().filter_map(|c| face.glyph_index(c)).collect();
    if glyphs.len() < SAMPLE.len() / 2 {
        return None;
    }
    let adv_units: f32 = glyphs.iter().map(|g| face.glyph_hor_advance(*g).unwrap_or(0) as f32).sum();
    if adv_units <= 0.0 {
        return None;
    }
    let width = adv_units / glyphs.len() as f32 / cap;
    let w = (adv_units * k).ceil() as u32 + 8;
    let hpx = (CAP_PX * 2.0) as u32;
    let mut sink = Sink { b: PathBuilder::new(), sx: k, sy: k, dx: 4.0, dy: CAP_PX * 1.5 };
    for g in &glyphs {
        face.outline_glyph(*g, &mut sink);
        sink.dx += face.glyph_hor_advance(*g).unwrap_or(0) as f32 * k;
    }
    let path = sink.b.finish()?;
    let mut mask = Mask::new(w, hpx)?;
    mask.fill_path(&path, FillRule::Winding, true, Transform::identity());
    let ink: f32 = mask.data().iter().map(|v| *v as f32 / 255.0).sum();
    let darkness = ink / (adv_units * k * CAP_PX);
    Some(Metrics { width, darkness })
}

/// Axis values a request sets: `wght` = weight, `wdth` = stretch percent.
pub fn request_vars(weight: u16, stretch_pct: f32) -> Vec<(Tag, f32)> {
    vec![(Tag::from_bytes(b"wght"), weight as f32), (Tag::from_bytes(b"wdth"), stretch_pct)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuffy_measures_sensibly() {
        let m = measure(crate::fonts::TUFFY, 0, &[]).unwrap();
        assert!(m.width > 0.5 && m.width < 1.0, "{m:?}");
        assert!(m.darkness > 0.1 && m.darkness < 0.6, "{m:?}");
        assert!(m.distance((m.width, m.darkness)) < 1e-6);
    }

    /// Prints the target metrics of every class from a folder of downloaded Google Fonts
    /// (`PIXELOX_GF_DIR=<dir> cargo test -p pixelox-text print_class_targets -- --ignored --nocapture`).
    #[test]
    #[ignore]
    fn print_class_targets() {
        let dir = std::path::PathBuf::from(std::env::var("PIXELOX_GF_DIR").expect("PIXELOX_GF_DIR"));
        for c in crate::classes::CLASSES {
            let g = c.google.iter().find(|g| g.member).unwrap();
            let fam = crate::gfonts::curated(g.family).unwrap();
            let file = fam
                .files
                .iter()
                .filter(|f| (f.style == "italic") == g.italic)
                .min_by_key(|f| (f.weight as i32 - g.weight as i32).abs())
                .unwrap_or(&fam.files[0]);
            let sub = fam.dir.split('/').nth(1).unwrap();
            let data = std::fs::read(dir.join(sub).join(&file.name)).unwrap();
            let pct = crate::fonts::stretch_percent(g.stretch);
            let m = measure(&data, 0, &request_vars(g.weight, pct)).unwrap();
            println!("{:<20} {:<22} target: ({:.3}, {:.3}),", c.id, g.family, m.width, m.darkness);
        }
    }
}
