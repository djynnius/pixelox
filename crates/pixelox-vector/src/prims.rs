//! Helpers shared by the generated vector primitives (`shape icon`, `shape qr`,
//! `shape barcode`): placing a box with `--at`/`--anchor` (the `comp place` rules), adding
//! the layer (or a group with a background plate), and checking that a code stands out
//! from what lies under it.

use crate::spec::{self, fmt_num, ShapeSpec, PREC};
use pixelox_core::doc::LayerPath;
use pixelox_core::op::ParamSpec;
use pixelox_core::prelude::*;

/// `--at` and `--anchor`, worded like `comp place`, and the stack placement flags.
pub fn place_params(what: &'static str) -> Vec<ParamSpec> {
    let mut v = vec![
        param("at", ParamKind::Point).help(match what {
            "icon" => "Position X,Y of the icon's anchor point (px or %); negative counts from the right/bottom",
            _ => "Position X,Y of the code's anchor point (px or %); negative counts from the right/bottom",
        }),
        param("anchor", ParamKind::Choice).choices(units::ANCHORS).help(
            "Which point of the box sits at --at (default tl; alone: align to that canvas point; neither: centered)",
        ),
    ];
    v.extend(common::placement());
    v
}

/// Top-left of a `w`×`h` box from `--at`/`--anchor` (centered when neither is given),
/// snapped to whole pixels so grid-aligned artwork stays crisp.
pub fn place_box(a: &Args, doc: &Document, w: f64, h: f64) -> Result<(f64, f64)> {
    let (cw, ch, dpi) = (doc.width as f64, doc.height as f64, doc.dpi());
    let anchor = match a.str("anchor") {
        Some(s) => Some(Anchor::parse(&s)?),
        None => None,
    };
    let (x, y) = match (a.point("at")?, anchor) {
        (Some(p), anc) => {
            let (px, py) = p.resolve(cw, ch, dpi);
            anc.unwrap_or(Anchor::Tl).place(px, py, w, h)
        }
        (None, Some(anc)) => {
            let (fx, fy) = anc.fractions();
            anc.place(fx * cw, fy * ch, w, h)
        }
        (None, None) => ((cw - w) / 2.0, (ch - h) / 2.0),
    };
    Ok((x.round(), y.round()))
}

pub fn rect_str(x: f64, y: f64, w: f64, h: f64) -> String {
    format!("{},{},{}x{}", fmt_num(x, PREC), fmt_num(y, PREC), fmt_num(w, PREC), fmt_num(h, PREC))
}

/// The layer name from `--name` (must be free) or a default made unique.
pub fn layer_name(a: &Args, doc: &Document, default: &str) -> Result<String> {
    match a.str("name").map(|n| n.trim().to_string()).filter(|n| !n.is_empty()) {
        Some(n) => {
            doc.check_name_free(&n, None)?;
            Ok(n)
        }
        None => Ok(doc.unique_name(default)),
    }
}

/// Insert a new layer on top of the stack, or where `--above`/`--below`/`--into` say
/// (ADR 0014), like every layer-creating command.
pub fn insert(a: &Args, doc: &mut Document, layer: Layer) -> Result<LayerPath> {
    pixelox_core::stack::insert_new(a, doc, layer)
}

/// A finished primitive: the shape spec of its artwork, and optionally a background plate.
pub struct Built {
    pub name: String,
    pub spec: ShapeSpec,
    /// Background color and rounded-rect path data (canvas px).
    pub plate: Option<(Rgba, String)>,
    pub opacity: f32,
}

/// Add the primitive as one shape layer, or, with a plate, as a group named `name` holding
/// `<name> background` and `<name> code`. Returns the result fields for the envelope.
pub fn add(a: &Args, doc: &mut Document, b: Built, code_suffix: &str) -> Result<Value> {
    let bbox = spec::spec_bbox(&b.spec);
    match b.plate {
        None => {
            let mut l = doc.new_layer(&b.name, LayerKind::Shape { spec: b.spec.to_value() });
            l.opacity = b.opacity;
            let (id, name) = (l.id.clone(), l.name.clone());
            insert(a, doc, l)?;
            Ok(json!({"layer": name, "id": id, "kind": "shape", "bbox": bbox}))
        }
        Some((color, d)) => {
            let mut plate = ShapeSpec::new(d, json!({"type": "plate"}));
            plate.fill = Some(color);
            let plate_bbox = spec::spec_bbox(&plate);
            let bg = doc.new_layer(&format!("{} background", b.name), LayerKind::Shape { spec: plate.to_value() });
            let code =
                doc.new_layer(&format!("{} {code_suffix}", b.name), LayerKind::Shape { spec: b.spec.to_value() });
            let children = vec![bg.name.clone(), code.name.clone()];
            let mut g = doc.new_layer(&b.name, LayerKind::Group { children: vec![bg, code] });
            g.opacity = b.opacity;
            let (id, name) = (g.id.clone(), g.name.clone());
            insert(a, doc, g)?;
            Ok(json!({
                "layer": name, "id": id, "kind": "group", "children": children, "bbox": plate_bbox, "code_bbox": bbox,
            }))
        }
    }
}

/// `--background-color` as an optional color (`none` = no plate).
pub fn background(a: &Args) -> Result<Option<Rgba>> {
    match a.raw("background-color") {
        None => Ok(None),
        Some(v) => spec::parse_paint(v).map_err(|mut e| {
            e.message = format!("--background-color: {}", e.message);
            e
        }),
    }
}

/// Relative luminance (WCAG) of an sRGB color.
fn rel_lum(c: [f32; 3]) -> f64 {
    let f = |v: f32| {
        let v = v as f64;
        if v <= 0.04045 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * f(c[0]) + 0.7152 * f(c[1]) + 0.0722 * f(c[2])
}

/// WCAG contrast ratio of two colors (1..21).
pub fn contrast(a: [f32; 3], b: [f32; 3]) -> f64 {
    let (la, lb) = (rel_lum(a), rel_lum(b));
    (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
}

/// Warn (PXW6010) when a code drawn in `ink` over the backdrop that `plate` (if any) or
/// the document below provides in `rect` would be hard for a scanner to read: low contrast,
/// or light modules on a dark ground (inverted codes are not read by every scanner).
pub fn check_contrast(doc: &Document, ctx: &mut OpCtx, ink: Rgba, plate: Option<Rgba>, rect: [f64; 4], what: &str) {
    let ground: [f32; 3] = match plate {
        Some(p) => [p.r, p.g, p.b],
        None => {
            // the mean color under the code's box (transparent canvas counts as white paper)
            let Ok(img) = compose::composite(doc) else { return };
            let x0 = rect[0].max(0.0) as u32;
            let y0 = rect[1].max(0.0) as u32;
            let x1 = ((rect[0] + rect[2]).ceil().max(0.0) as u32).min(doc.width);
            let y1 = ((rect[1] + rect[3]).ceil().max(0.0) as u32).min(doc.height);
            if x1 <= x0 || y1 <= y0 {
                return;
            }
            let mut acc = [0f64; 3];
            let mut n = 0f64;
            for y in y0..y1 {
                for x in x0..x1 {
                    let p = img.get(x, y);
                    for (c, v) in acc.iter_mut().enumerate() {
                        *v += (p[c] * p[3] + (1.0 - p[3])) as f64;
                    }
                    n += 1.0;
                }
            }
            [(acc[0] / n) as f32, (acc[1] / n) as f32, (acc[2] / n) as f32]
        }
    };
    let ink3 = [ink.r, ink.g, ink.b];
    let ratio = contrast(ink3, ground);
    let inverted = rel_lum(ink3) > rel_lum(ground);
    if ratio < 4.0 || inverted {
        let why = if inverted { "the modules are lighter than the ground (inverted)" } else { "the contrast is low" };
        ctx.warn_ctx(
            "PXW6010",
            format!(
                "the {what} may not scan: {why} (contrast {}:1, at least 4:1 with dark modules is safe); add --background-color white",
                (ratio * 10.0).round() / 10.0
            ),
            json!({"contrast": (ratio * 100.0).round() / 100.0, "inverted": inverted, "ground": Rgba::new(ground[0], ground[1], ground[2], 1.0).to_hex()}),
        );
    }
}
