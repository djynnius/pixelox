//! `svg from-doc`: export a document's layer tree as structured SVG (PRD §8.15, §7.9).
//! Shape layers become `<path>`, text layers `<text>` (shared spec keys), groups `<g>`,
//! raster and placed layers `<image>` with an embedded PNG (or the placed file's own
//! bytes) unless `--vector-only`. Opacity, visibility, and blend modes carry over.

use crate::spec::{fmt_num, ShapeSpec};
use crate::xml::{WriteOpts, XDoc, XEl, XNode, SVG_NS};
use base64::Engine;
use pixelox_core::exec;
use pixelox_core::prelude::*;
use std::collections::BTreeSet;
use std::path::PathBuf;

pub fn spec() -> OpSpec {
    OpSpec::doc("svg from-doc", Category::Svg, "Export a document's layers as structured SVG", from_doc)
        .export_mode()
        .about("Writes the layer tree as SVG: shape layers as <path>, text layers as <text>, groups as <g>, raster and placed layers as embedded PNG <image>s (dropped with --vector-only). Layer names become ids; opacity, visibility, and CSS-expressible blend modes (mix-blend-mode) carry over. Adjustment layers, layer styles on vector layers, and masks on vector layers are reported in `skipped`/`notes`.")
        .param(param("vector-only", ParamKind::Flag).help("Drop raster and placed layers instead of embedding them as images"))
        .example("pixelox svg from-doc badge.pxo -o badge.svg", "Export every layer")
        .example("pixelox svg from-doc poster.pxo -o poster-vectors.svg --vector-only", "Only shapes and text")
        .example("pixelox svg from-doc badge.pxo -o badge.svg --overwrite --json", "Re-export and report what was skipped")
}

/// Blend modes with a CSS `mix-blend-mode` equivalent.
const CSS_BLEND: &[&str] = &[
    "multiply",
    "screen",
    "overlay",
    "darken",
    "lighten",
    "color-dodge",
    "color-burn",
    "hard-light",
    "soft-light",
    "difference",
    "exclusion",
    "hue",
    "saturation",
    "color",
    "luminosity",
];

struct Exporter<'a> {
    doc: &'a Document,
    vector_only: bool,
    ids: BTreeSet<String>,
    defs: Vec<XEl>,
    counts: [usize; 4],
    skipped: Vec<Value>,
    notes: Vec<Value>,
}

fn sanitize(name: &str) -> String {
    let mut s = String::new();
    for c in name.chars() {
        let ok = c.is_alphanumeric() || matches!(c, '-' | '_' | '.');
        if ok {
            s.push(c);
        } else if !s.ends_with('-') {
            s.push('-');
        }
    }
    let s = s.trim_matches('-').to_string();
    if s.chars().next().map(|c| c.is_alphabetic() || c == '_').unwrap_or(false) {
        s
    } else {
        format!("layer-{s}").trim_end_matches('-').to_string()
    }
}

fn paint_attrs(el: &mut XEl, prefix: &str, c: Option<Rgba>) {
    match c {
        None => el.set(prefix, "none"),
        Some(c) => {
            el.set(prefix, c.with_alpha(1.0).to_hex());
            if c.a < 1.0 {
                el.set(&format!("{prefix}-opacity"), fmt_num(c.a as f64, 4));
            }
        }
    }
}

/// A shape spec as a `<path>`.
pub fn shape_el(spec: &Value) -> Result<XEl> {
    let s = ShapeSpec::from_value(spec)?;
    let mut el = XEl::new("path").attr("d", s.d.trim());
    paint_attrs(&mut el, "fill", s.fill);
    if s.fill_rule == "evenodd" {
        el.set("fill-rule", "evenodd");
    }
    if s.has_stroke() {
        paint_attrs(&mut el, "stroke", s.stroke);
        el.set("stroke-width", fmt_num(s.stroke_width, 4));
        if s.cap != "butt" {
            el.set("stroke-linecap", &s.cap);
        }
        if s.join != "miter" {
            el.set("stroke-linejoin", &s.join);
        }
        if (s.miter_limit - 4.0).abs() > 1e-9 {
            el.set("stroke-miterlimit", fmt_num(s.miter_limit, 4));
        }
        if s.dash_even().is_some() {
            el.set("stroke-dasharray", s.dash.iter().map(|v| fmt_num(*v, 4)).collect::<Vec<_>>().join(","));
            if s.dash_offset != 0.0 {
                el.set("stroke-dashoffset", fmt_num(s.dash_offset, 4));
            }
        }
    }
    Ok(el)
}

/// Straight-alpha image → PNG bytes through tiny-skia's encoder.
pub fn encode_png(img: &Image) -> Result<Vec<u8>> {
    let mut pm = tiny_skia::Pixmap::new(img.width.max(1), img.height.max(1))
        .ok_or_else(|| Error::new("PX3001", "cannot allocate an image for PNG encoding"))?;
    for (dst, p) in pm.pixels_mut().iter_mut().zip(img.data.chunks(4)) {
        *dst = tiny_skia::ColorU8::from_rgba(
            color::to_u8(p[0]),
            color::to_u8(p[1]),
            color::to_u8(p[2]),
            color::to_u8(p[3]),
        )
        .premultiply();
    }
    pm.encode_png().map_err(|e| Error::new("PX1107", format!("PNG encoding failed: {e}")))
}

fn data_uri(mime: &str, bytes: &[u8]) -> String {
    format!("data:{mime};base64,{}", base64::engine::general_purpose::STANDARD.encode(bytes))
}

fn mime_of(ext: &str) -> Option<&'static str> {
    Some(match ext.to_ascii_lowercase().as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        _ => return None,
    })
}

impl Exporter<'_> {
    fn unique_id(&mut self, name: &str) -> String {
        let base = sanitize(name);
        let mut id = base.clone();
        let mut n = 2;
        while self.ids.contains(&id) {
            id = format!("{base}-{n}");
            n += 1;
        }
        self.ids.insert(id.clone());
        id
    }

    fn skip(&mut self, l: &Layer, reason: &str) {
        self.skipped.push(json!({"layer": l.name, "kind": l.kind_name(), "reason": reason}));
    }

    fn note(&mut self, l: &Layer, note: &str) {
        self.notes.push(json!({"layer": l.name, "note": note}));
    }

    fn text_el(&mut self, l: &Layer, spec: &Value, id: &str) -> XEl {
        let num = |k: &str| {
            spec.get(k).and_then(|v| v.as_f64().or_else(|| v.as_str().and_then(|s| s.parse().ok()))).unwrap_or(0.0)
        };
        let content = spec.get("content").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let (x, y) = (num("x"), num("y"));
        let size = match spec.get("size") {
            Some(Value::Number(n)) => n.as_f64().unwrap_or(16.0),
            Some(Value::String(s)) => {
                units::Length::parse(s).map(|l| l.to_px(self.doc.height as f64, self.doc.dpi())).unwrap_or(16.0)
            }
            _ => 16.0,
        };
        let anchor = spec.get("anchor").and_then(|v| v.as_str()).unwrap_or("tl");
        let mut t = XEl::new("text").attr("x", fmt_num(x, 3)).attr("y", fmt_num(y, 3));
        if let Some(f) = spec.get("font").and_then(|v| v.as_str()) {
            t.set("font-family", f);
        }
        t.set("font-size", fmt_num(size, 3));
        if let Some(w) =
            spec.get("weight").and_then(|v| v.as_f64().map(|n| fmt_num(n, 0)).or_else(|| v.as_str().map(String::from)))
        {
            t.set("font-weight", w);
        }
        let c =
            spec.get("color").and_then(|v| v.as_str()).and_then(|s| color::parse_color(s).ok()).unwrap_or(Rgba::BLACK);
        paint_attrs(&mut t, "fill", Some(c));
        let h = match anchor {
            "t" | "c" | "b" => "middle",
            "tr" | "r" | "br" => "end",
            _ => "start",
        };
        if h != "start" {
            t.set("text-anchor", h);
        }
        let row = match anchor {
            "tl" | "t" | "tr" => 0,
            "l" | "c" | "r" => 1,
            _ => 2,
        };
        t.set("dominant-baseline", ["hanging", "central", "text-after-edge"][row]);
        let rot = num("rotation");
        if rot != 0.0 {
            t.set("transform", format!("rotate({} {} {})", fmt_num(rot, 4), fmt_num(x, 3), fmt_num(y, 3)));
        }
        if content.contains("  ") || content.starts_with(' ') || content.ends_with(' ') {
            t.set("xml:space", "preserve");
        }
        if let Some(d) = spec.get("path").and_then(|v| v.as_str()) {
            let pid = format!("{id}-path");
            self.ids.insert(pid.clone());
            self.defs.push(XEl::new("path").attr("id", &pid).attr("d", d));
            let mut tp = XEl::new("textPath").attr("href", format!("#{pid}"));
            tp.children.push(XNode::Text(content.replace('\n', " ")));
            t.push(tp);
            return t;
        }
        let lines: Vec<&str> = content.split('\n').collect();
        if lines.len() <= 1 {
            t.children.push(XNode::Text(content));
        } else {
            let lead = 1.2;
            let first = -(lines.len() as f64 - 1.0) * lead * row as f64 / 2.0;
            for (i, line) in lines.iter().enumerate() {
                let dy = if i == 0 { first } else { lead };
                let mut s = XEl::new("tspan").attr("x", fmt_num(x, 3)).attr("dy", format!("{}em", fmt_num(dy, 3)));
                s.children.push(XNode::Text(line.to_string()));
                t.push(s);
            }
        }
        if !l.fx.is_empty() {
            self.note(l, "layer styles (fx) are not exported for text layers");
        }
        t
    }

    fn image_el(&mut self, l: &Layer) -> Result<Option<XEl>> {
        if let LayerKind::Placed { asset, rect, angle, .. } = &l.content {
            if let (Some(a), true) = (self.doc.assets.get(asset), l.fx.is_empty() && l.mask.is_none()) {
                if let Some(mime) = mime_of(&a.ext) {
                    let [x, y, w, h] = *rect;
                    let mut el = XEl::new("image")
                        .attr("x", fmt_num(x, 3))
                        .attr("y", fmt_num(y, 3))
                        .attr("width", fmt_num(w, 3))
                        .attr("height", fmt_num(h, 3))
                        .attr("preserveAspectRatio", "none")
                        .attr("href", data_uri(mime, &a.bytes));
                    if angle.abs() > 1e-9 {
                        el.set(
                            "transform",
                            format!(
                                "rotate({} {} {})",
                                fmt_num(*angle, 4),
                                fmt_num(x + w / 2.0, 3),
                                fmt_num(y + h / 2.0, 3)
                            ),
                        );
                    }
                    return Ok(Some(el));
                }
            }
        }
        let Some(mut r) = compose::render_layer(self.doc, l)? else {
            return Ok(None);
        };
        if let Some(lm) = l.mask.as_ref().filter(|m| m.enabled) {
            let w = r.image.width;
            for (i, p) in r.image.data.chunks_mut(4).enumerate() {
                let (px, py) = ((i as u32 % w) as i64 + r.x as i64, (i as u32 / w) as i64 + r.y as i64);
                p[3] *= lm.value(lm.mask.at(px, py));
            }
        }
        let png = encode_png(&r.image)?;
        Ok(Some(
            XEl::new("image")
                .attr("x", r.x.to_string())
                .attr("y", r.y.to_string())
                .attr("width", r.image.width.to_string())
                .attr("height", r.image.height.to_string())
                .attr("href", data_uri("image/png", &png)),
        ))
    }

    fn layer(&mut self, l: &Layer) -> Result<Option<XEl>> {
        let id = self.unique_id(&l.name);
        let vector = matches!(l.content, LayerKind::Shape { .. } | LayerKind::Text { .. });
        let mut el = match &l.content {
            LayerKind::Shape { spec } => {
                self.counts[0] += 1;
                if !l.fx.is_empty() {
                    self.note(l, "layer styles (fx) are not exported for shape layers");
                }
                shape_el(spec)?
            }
            LayerKind::Text { spec } => {
                self.counts[1] += 1;
                self.text_el(l, spec, &id)
            }
            LayerKind::Raster { .. } | LayerKind::Placed { .. } => {
                if self.vector_only {
                    self.skip(l, "raster content (--vector-only)");
                    return Ok(None);
                }
                match self.image_el(l)? {
                    Some(e) => {
                        self.counts[2] += 1;
                        e
                    }
                    None => {
                        self.skip(l, "empty layer");
                        return Ok(None);
                    }
                }
            }
            LayerKind::Group { children } => {
                self.counts[3] += 1;
                let mut g = XEl::new("g");
                for c in children {
                    if let Some(e) = self.layer(c)? {
                        g.push(e);
                    }
                }
                if l.blend == BlendMode::Normal {
                    g.set("style", "isolation:isolate");
                }
                g
            }
            LayerKind::Adjustment { op, .. } => {
                self.skip(l, &format!("adjustment layer ({op}) has no SVG equivalent"));
                return Ok(None);
            }
        };
        el.attrs.insert(0, ("id".into(), id.clone()));
        if id != l.name {
            el.attrs.insert(1, ("data-name".into(), l.name.clone()));
        }
        if !l.visible {
            el.set("display", "none");
        }
        if l.opacity < 100.0 {
            el.set("opacity", fmt_num(l.opacity as f64 / 100.0, 4));
        }
        let mode = l.blend.name();
        if CSS_BLEND.contains(&mode) {
            let style = el.get("style").map(|s| format!("{s};")).unwrap_or_default();
            el.set("style", format!("{style}mix-blend-mode:{mode}"));
        } else if !matches!(l.blend, BlendMode::Normal | BlendMode::PassThrough) {
            el.set("data-blend", mode);
            self.note(l, &format!("blend mode '{mode}' has no CSS equivalent; exported as normal"));
        }
        if vector && l.mask.is_some() {
            self.note(l, "layer masks are not exported for vector layers");
        }
        if l.clip {
            self.note(l, "clipping masks are not exported");
        }
        Ok(Some(el))
    }
}

fn from_doc(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let to = match &ctx.globals.output {
        Some(o) => Some(PathBuf::from(o)),
        None if ctx.dry_run() => None,
        None => {
            return Err(Error::new("PX1004", "'svg from-doc' writes an SVG; say where").with_hint("pass -o out.svg"))
        }
    };
    let (w, h) = (doc.width as f64, doc.height as f64);
    let mut ex = Exporter {
        doc,
        vector_only: a.flag("vector-only"),
        ids: BTreeSet::new(),
        defs: Vec::new(),
        counts: [0; 4],
        skipped: Vec::new(),
        notes: Vec::new(),
    };
    let mut root = XEl::new("svg")
        .attr("xmlns", SVG_NS)
        .attr("width", fmt_num(w, 3))
        .attr("height", fmt_num(h, 3))
        .attr("viewBox", format!("0 0 {} {}", fmt_num(w, 3), fmt_num(h, 3)));
    for l in &doc.layers {
        if let Some(e) = ex.layer(l)? {
            root.push(e);
        }
    }
    if !ex.defs.is_empty() {
        let mut defs = XEl::new("defs");
        for d in std::mem::take(&mut ex.defs) {
            defs.push(d);
        }
        root.children.insert(0, XNode::El(defs));
    }
    let text = XDoc::new(root).serialize(WriteOpts::PRETTY);
    if let Some(p) = &to {
        let recipe = exec::recipe_with(doc, exec::make_step(&spec(), a, ctx));
        ctx.write_file(p, text.as_bytes(), None, Some(recipe))?;
    }
    let [shapes, texts, images, groups] = ex.counts;
    Ok(json!({
        "path": to.map(|p| p.display().to_string()),
        "shapes": shapes,
        "texts": texts,
        "images": images,
        "groups": groups,
        "skipped": ex.skipped,
        "notes": ex.notes,
        "bytes": text.len(),
    }))
}
