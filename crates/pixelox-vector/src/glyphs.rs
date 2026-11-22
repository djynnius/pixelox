//! Vector text for barcode captions: printable ASCII outlines from a subset of Tuffy
//! (public domain, `assets/TuffyAscii.ttf`; see `assets/Tuffy-LICENSE.txt`), so the digits
//! under a barcode are part of its shape path and scale and move with it.

use kurbo::BezPath;
use std::sync::OnceLock;
use ttf_parser::{Face, OutlineBuilder};

const FONT: &[u8] = include_bytes!("../assets/TuffyAscii.ttf");

fn face() -> &'static Face<'static> {
    static FACE: OnceLock<Face<'static>> = OnceLock::new();
    FACE.get_or_init(|| Face::parse(FONT, 0).expect("bundled TuffyAscii.ttf parses"))
}

struct Pen {
    p: BezPath,
    k: f64,
    ox: f64,
    oy: f64,
}

impl Pen {
    fn pt(&self, x: f32, y: f32) -> (f64, f64) {
        (self.ox + x as f64 * self.k, self.oy - y as f64 * self.k)
    }
}

impl OutlineBuilder for Pen {
    fn move_to(&mut self, x: f32, y: f32) {
        let p = self.pt(x, y);
        self.p.move_to(p);
    }
    fn line_to(&mut self, x: f32, y: f32) {
        let p = self.pt(x, y);
        self.p.line_to(p);
    }
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let (a, b) = (self.pt(x1, y1), self.pt(x, y));
        self.p.quad_to(a, b);
    }
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let (a, b, c) = (self.pt(x1, y1), self.pt(x2, y2), self.pt(x, y));
        self.p.curve_to(a, b, c);
    }
    fn close(&mut self) {
        self.p.close_path();
    }
}

/// Characters the caption font can draw (printable ASCII).
pub fn drawable(c: char) -> bool {
    (' '..='~').contains(&c)
}

fn scale(size: f64) -> f64 {
    size / face().units_per_em() as f64
}

/// Advance width of `text` at `size` px per em.
pub fn width(text: &str, size: f64) -> f64 {
    let f = face();
    text.chars()
        .filter_map(|c| f.glyph_index(c))
        .map(|g| f.glyph_hor_advance(g).unwrap_or(0) as f64 * scale(size))
        .sum()
}

/// Height of the digits (the cap height) at `size` px per em.
pub fn digit_height(size: f64) -> f64 {
    let f = face();
    let h = f
        .glyph_index('0')
        .and_then(|g| f.glyph_bounding_box(g))
        .map(|b| b.y_max as f64)
        .unwrap_or(f.units_per_em() as f64 * 0.7);
    h * scale(size)
}

/// Outlines of `text` with its left end on the baseline at (`x`, `y`) (canvas px, y down).
pub fn text_path(text: &str, x: f64, y: f64, size: f64) -> BezPath {
    let f = face();
    let k = scale(size);
    let mut pen = Pen { p: BezPath::new(), k, ox: x, oy: y };
    for c in text.chars() {
        let Some(g) = f.glyph_index(c) else { continue };
        f.outline_glyph(g, &mut pen);
        pen.ox += f.glyph_hor_advance(g).unwrap_or(0) as f64 * k;
    }
    pen.p
}

/// One character centered horizontally on `cx`, baseline at `y`.
pub fn char_path_centered(c: char, cx: f64, y: f64, size: f64) -> BezPath {
    let s = c.to_string();
    text_path(&s, cx - width(&s, size) / 2.0, y, size)
}

#[cfg(test)]
mod tests {
    use super::*;
    use kurbo::Shape;

    #[test]
    fn digits_have_outlines_and_widths() {
        for c in '0'..='9' {
            let p = char_path_centered(c, 50.0, 100.0, 20.0);
            let b = p.bounding_box();
            assert!(b.width() > 3.0 && b.height() > 10.0, "{c}: {b:?}");
            assert!(b.y1 <= 100.5 && b.y0 > 80.0, "{c} sits on the baseline: {b:?}");
        }
        assert!(width("0123", 20.0) > 30.0);
        assert!((digit_height(100.0) - 70.0).abs() < 15.0);
        assert!(drawable('A') && !drawable('é'));
    }
}
