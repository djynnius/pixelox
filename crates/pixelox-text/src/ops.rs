//! The `text *` commands.

use crate::layout::{compose, parse_path, Composed, Polyline};
use crate::render::svg_path_data;
use crate::spec::{apply_place, apply_style, place_params, style_params, Fit, Side, TextSpec, WARP_CHOICES};
use crate::{classes, fonts, gfonts, warp};
use pixelox_core::doc::LayerPath;
use pixelox_core::prelude::*;
use pixelox_core::units::Length;

fn round2(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}

fn layer_help() -> &'static str {
    "Text layer: name, #N (1 = top), @top, @bottom, or @active; defaults to the only text layer"
}

fn not_text(l: &Layer) -> Error {
    Error::new("PX2303", format!("layer '{}' is a {} layer; this command needs a text layer", l.name, l.kind_name()))
        .with_hint("list text layers with `pixelox text list <doc>`, or add one with `pixelox text add`")
        .with_context("layer", &l.name)
        .with_context("kind", l.kind_name())
}

/// Resolve `--layer` to a text layer (default: the only text layer).
fn target_text(doc: &Document, r: Option<&str>) -> Result<LayerPath> {
    let path = match r {
        Some(r) => doc.resolve(r)?,
        None => {
            let texts: Vec<LayerPath> = doc
                .walk()
                .into_iter()
                .filter(|(_, l)| matches!(l.content, LayerKind::Text { .. }))
                .map(|(p, _)| p)
                .collect();
            match texts.len() {
                1 => texts[0].clone(),
                0 => {
                    return Err(Error::new("PX2302", "the document has no text layer")
                        .with_hint("add one with `pixelox text add <doc> --content \"...\" --in-place`"))
                }
                n => {
                    return Err(Error::new(
                        "PX2302",
                        format!("the document has {n} text layers; say which one with --layer"),
                    )
                    .with_context("text_layers", texts.iter().map(|p| doc.layer(p).name.clone()).collect::<Vec<_>>()))
                }
            }
        }
    };
    let l = doc.layer(&path);
    if !matches!(l.content, LayerKind::Text { .. }) {
        return Err(not_text(l));
    }
    Ok(path)
}

fn spec_of(l: &Layer) -> Value {
    match &l.content {
        LayerKind::Text { spec } => spec.clone(),
        _ => Value::Null,
    }
}

/// Strict font mode: `--strict-font` or `PIXELOX_STRICT_FONTS=1`.
fn strict(a: &Args) -> bool {
    a.flag("strict-font") || fonts::strict_env()
}

/// With download consent (`-y`), fetch a missing family (or the first Google member of a
/// class with no installed member) before the layer is set.
fn prepare_font(spec: &TextSpec, ctx: &mut OpCtx) -> Result<Option<Value>> {
    if !ctx.globals.yes || ctx.dry_run() {
        return Ok(None);
    }
    let req = crate::layout::request_of(spec);
    let Err(e) = fonts::resolve(&req, true) else { return Ok(None) };
    if !matches!(e.code.as_str(), "PX1540" | "PX4102") {
        return Ok(None);
    }
    let name = if classes::is_class_ref(&spec.font) {
        match classes::find(&spec.font) {
            Some(c) => c.google.iter().find(|g| g.member).unwrap_or(&c.google[0]).family.to_string(),
            None => return Ok(None),
        }
    } else {
        spec.font.clone()
    };
    match gfonts::get(ctx, &name, true) {
        Ok(r) => Ok(Some(r.to_json())),
        // not a Google Fonts family: fall through to substitution
        Err(e) if e.code == "PX4111" => Ok(None),
        Err(e) => Err(e),
    }
}

/// Size the type for `spec.fit` (ink width or box), iterating because the stroke and
/// optical-size axes are not proportional to the size.
fn fit_size(spec: &mut TextSpec, doc: &Document, strict: bool) -> Result<Composed> {
    let mut c = compose(spec, doc, strict)?;
    let Some(fit) = spec.fit else { return Ok(c) };
    for _ in 0..12 {
        let Some((w, h)) = c.ink_size().filter(|(w, h)| *w > 1e-6 && *h > 1e-6) else {
            return Err(Error::new("PX4113", "the text has no visible letters to fit")
                .with_hint("give the layer visible content first, or drop --fit-width / --fit-box"));
        };
        let k = match fit {
            Fit::Width(tw) => tw / w,
            Fit::Box(tw, th) => (tw / w).min(th / h),
        };
        if (k - 1.0).abs() < 1e-7 {
            break;
        }
        let s = (c.size_px * k).clamp(0.01, 20_000.0);
        spec.size = format!("{s}px");
        c = compose(spec, doc, strict)?;
    }
    Ok(c)
}

/// `text on-path --fit shrink`: lower the size until the set line fits on the path.
fn shrink_to_path(spec: &mut TextSpec, doc: &Document, c: Composed, strict: bool) -> Result<Composed> {
    let mut c = c;
    for _ in 0..12 {
        let (Some(len), Some(w)) = (c.path_length, c.path_text_width) else { break };
        if w <= len + 1e-6 || w <= 0.0 {
            break;
        }
        // just under the path length so rounding never leaves the last glyph overhanging
        let s = (c.size_px * (len / w) * 0.999_999).clamp(0.01, 20_000.0);
        spec.size = format!("{s}px");
        c = compose(spec, doc, strict)?;
    }
    Ok(c)
}

/// Lay out `spec`, record the face in the spec and the sidecar, and raise the warnings.
fn finalize(spec: &mut TextSpec, doc: &Document, ctx: &mut OpCtx, strict: bool) -> Result<Composed> {
    let downloaded = prepare_font(spec, ctx)?;
    let mut c = fit_size(spec, doc, strict)?;
    c.downloaded = downloaded;
    if spec.path.is_some() && spec.path_shrink {
        c = shrink_to_path(spec, doc, c, strict)?;
    }
    font_warnings(spec, &c, ctx);
    spec.font_sha256 = Some(c.primary.file.sha256.clone());
    spec.font_face = Some(c.primary.describe(false));
    ctx.font_used(c.primary.record());
    for f in &c.fallbacks {
        ctx.font_used(f.record());
    }
    if !c.synthetic.is_empty() {
        ctx.warn_ctx(
            "PXW4002",
            format!(
                "'{}' has no {} face; synthetic {} was applied",
                c.primary.family,
                c.synthetic.join("/"),
                c.synthetic.join(" and ")
            ),
            json!({"family": c.primary.family, "synthetic": c.synthetic}),
        );
    }
    if !c.fallbacks.is_empty() || !c.missing.is_empty() {
        let fams: Vec<String> = c.fallbacks.iter().map(|f| f.family.clone()).collect();
        let msg = if c.missing.is_empty() {
            format!("'{}' lacks some characters; they were drawn with {}", c.primary.family, fams.join(", "))
        } else {
            format!(
                "no installed font has {}; drawn as missing-glyph boxes",
                c.missing.iter().map(|ch| format!("U+{:04X}", *ch as u32)).collect::<Vec<_>>().join(", ")
            )
        };
        ctx.warn_ctx(
            "PXW4001",
            msg,
            json!({
                "fallback": fams,
                "missing": c.missing.iter().map(|ch| format!("U+{:04X}", *ch as u32)).collect::<Vec<_>>(),
            }),
        );
    }
    if c.warp_ignored {
        ctx.warn("PXW4015", "text on a path is not warped; the warp is kept and applies when the text leaves the path");
    }
    if c.overflow_glyphs > 0 {
        let (len, w) = (c.path_length.unwrap_or(0.0), c.path_text_width.unwrap_or(0.0));
        ctx.warn_ctx(
            "PXW4012",
            format!(
                "{} of the text do not fit on the path ({:.0} px of text on a {:.0} px path); use --fit shrink, a smaller --size, or less --tracking",
                plural(c.overflow_glyphs, "glyph"),
                w,
                len
            ),
            json!({"overflow_glyphs": c.overflow_glyphs, "text_length": round2(w), "path_length": round2(len),
                   "fit_size_px": round2(c.size_px * len / w.max(1e-9))}),
        );
    }
    Ok(c)
}

fn plural(n: usize, w: &str) -> String {
    if n == 1 {
        format!("1 {w}")
    } else {
        format!("{n} {w}s")
    }
}

/// PXW4010 (substitute), PXW4014 (class stand-in), PXW4013 (weight or width not available).
fn font_warnings(spec: &TextSpec, c: &Composed, ctx: &mut OpCtx) {
    let r = &c.resolution;
    let face = &c.primary;
    match r.how {
        fonts::How::Substitute { by } => {
            let class = r.class.map(|c| c.alias()).unwrap_or_default();
            let what = if classes::is_class_ref(&spec.font) {
                format!("no font of class {} is installed", spec.font.trim())
            } else {
                format!("font family '{}' is not installed", spec.font)
            };
            let get = if classes::is_class_ref(&spec.font) {
                Some(gfonts::get_command(spec.font.trim()))
            } else {
                gfonts::curated(&spec.font).map(|f| gfonts::get_command(&f.family))
            };
            let fix = match &get {
                Some(g) => format!("get it with `{g}`"),
                None => format!(
                    "register its files with `pixelox fonts add <dir>`, or try `{}` if it is a Google Fonts family",
                    gfonts::get_command(&spec.font)
                ),
            };
            ctx.warn_ctx(
                "PXW4010",
                format!("{what}; drew it with {} {} ({class}, chosen by {by}); {fix}", face.family, face.weight),
                json!({"family": spec.font, "substitute": face.family, "postscript": face.postscript,
                       "class": class, "by": by, "get": get}),
            );
        }
        fonts::How::Class { member: false } => {
            let cls = r.class.expect("class");
            let g = cls.google.iter().find(|g| g.member).unwrap_or(&cls.google[0]);
            ctx.warn_ctx(
                "PXW4014",
                format!(
                    "no member of {} is installed; drew it with the stand-in {} {}; `{}` downloads {}",
                    cls.alias(),
                    face.family,
                    face.weight,
                    gfonts::get_command(&cls.alias()),
                    g.family
                ),
                json!({"class": cls.alias(), "stand_in": face.family, "get": gfonts::get_command(&cls.alias()), "downloads": g.family}),
            );
        }
        fonts::How::Family => {
            let wvar = face.axis("wght").is_some_and(|(lo, hi)| (r.weight as f32) >= lo && (r.weight as f32) <= hi);
            let dvar = face.axis("wdth").is_some_and(|(lo, hi)| r.stretch >= lo && r.stretch <= hi);
            let weight_off = spec.weight.is_some() && !wvar && face.weight.abs_diff(r.weight) >= 100;
            let width_off = !dvar && fonts::stretch_keyword(r.stretch) != face.stretch && spec.stretch.is_some();
            if weight_off || width_off {
                let mut want = format!("weight {}", r.weight);
                if spec.stretch.is_some() {
                    want.push_str(&format!(", {}", fonts::stretch_name(r.stretch)));
                }
                ctx.warn_ctx(
                    "PXW4013",
                    format!(
                        "'{}' has no {want} face; using {} ({}{}), the nearest by CSS font matching; faces: {}",
                        face.family,
                        face.postscript,
                        face.weight,
                        if face.stretch != 5 {
                            format!(", {}", fonts::STRETCH_NAMES[face.stretch as usize - 1])
                        } else {
                            String::new()
                        },
                        fonts::family_weights(&face.family).join(", ")
                    ),
                    json!({"family": face.family, "requested_weight": r.weight, "weight": face.weight,
                           "postscript": face.postscript, "faces": fonts::family_weights(&face.family)}),
                );
            }
        }
        _ => {}
    }
}

fn result_json(l: &Layer, spec: &TextSpec, c: &Composed) -> Value {
    let mut font = c.primary.describe(true);
    font["resolved"] = c.resolution.describe(&spec.font);
    let mut v = json!({
        "layer": l.name,
        "id": l.id,
        "content": spec.content,
        "font": font,
        "size_px": round2(c.size_px),
        "lines": c.lines,
        "glyphs": c.glyphs,
        "bounds": c.ink_rect().map(|r| r.to_json()),
        "overflow": c.overflow,
    });
    if spec.path.is_none() {
        v["box"] = json!(c.layout_box.map(round2));
    } else {
        v["path_length"] = json!(c.path_length.map(round2));
        v["text_length"] = json!(c.path_text_width.map(round2));
        v["overflow_glyphs"] = json!(c.overflow_glyphs);
    }
    if !c.synthetic.is_empty() {
        v["synthetic"] = json!(c.synthetic);
    }
    if !c.fallbacks.is_empty() {
        v["fallback_fonts"] = json!(c.fallbacks.iter().map(|f| f.family.clone()).collect::<Vec<_>>());
    }
    if !c.missing.is_empty() {
        v["missing_glyphs"] = json!(c.missing.iter().map(|ch| format!("U+{:04X}", *ch as u32)).collect::<Vec<_>>());
    }
    extras_json(spec, c, &mut v);
    if let Some(d) = &c.downloaded {
        v["downloaded"] = d.clone();
    }
    v
}

/// The 0.5 type settings, reported when set.
fn extras_json(spec: &TextSpec, c: &Composed, v: &mut Value) {
    if let Some(cs) = spec.case {
        v["case"] = json!(cs.name());
    }
    if spec.scale_x != 1.0 || spec.scale_y != 1.0 {
        v["scale"] = json!([spec.scale_x, spec.scale_y]);
    }
    if spec.faux_bold != 0.0 {
        v["faux_bold"] = json!(spec.faux_bold);
    }
    if let Some(s) = spec.stretch {
        v["stretch"] = json!(fonts::stretch_name(s as f32));
    }
    if let Some(f) = spec.fit {
        let ink = c.ink_size().map(|(w, h)| json!([round2(w), round2(h)]));
        v["fit"] = match f {
            Fit::Width(w) => json!({"width": round2(w), "ink": ink}),
            Fit::Box(w, h) => json!({"box": [round2(w), round2(h)], "ink": ink}),
        };
    }
    if spec.anchor_ink {
        v["anchor_to"] = json!("ink");
    }
    if let Some(w) = &spec.warp {
        v["warp"] = w.to_json();
    }
}

fn default_name(content: &str) -> String {
    let first = content.lines().map(str::trim).find(|l| !l.is_empty()).unwrap_or("");
    let mut name: String = first.chars().take(32).collect();
    if first.chars().count() > 32 {
        name = name.trim_end().to_string();
        name.push('…');
    }
    if name.is_empty() {
        "Text".to_string()
    } else {
        name
    }
}

pub(crate) fn text_add(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let mut spec = TextSpec { content: a.str_or("content", ""), ..TextSpec::default() };
    let ch = apply_style(a, doc, &mut spec)?;
    apply_place(a, doc, &mut spec)?;
    let _ = ch;
    let c = finalize(&mut spec, doc, ctx, strict(a))?;
    let name = match a.str("name").map(|n| n.trim().to_string()).filter(|n| !n.is_empty()) {
        Some(n) => {
            doc.check_name_free(&n, None)?;
            n
        }
        None => default_name(&spec.content),
    };
    let at = pixelox_core::stack::from_args(a, doc)?.unwrap_or(pixelox_core::stack::Placement::Top);
    let layer = doc.new_layer(&name, LayerKind::Text { spec: spec.to_value(None) });
    let path = pixelox_core::stack::insert_at(doc, layer, &at);
    Ok(result_json(doc.layer(&path), &spec, &c))
}

fn edit_common(a: &Args, doc: &mut Document, ctx: &mut OpCtx, content: bool) -> Result<Value> {
    let path = target_text(doc, a.raw("layer"))?;
    doc.layer(&path).check_unlocked()?;
    let old = spec_of(doc.layer(&path));
    let mut spec = TextSpec::from_value(&old);
    let mut changed = false;
    if content {
        if let Some(c) = a.str("content") {
            spec.content = c;
            changed = true;
        }
        changed |= apply_place(a, doc, &mut spec)?;
    }
    let ch = apply_style(a, doc, &mut spec)?;
    changed |= ch.any;
    if !changed {
        let flags = if content { "--content, a style flag, --at, --anchor, or --box" } else { "a style flag" };
        return Err(Error::new("PX1003", format!("nothing to change: give {flags}"))
            .with_hint(format!("pixelox docs text.{}", if content { "edit" } else { "style" })));
    }
    if ch.face {
        spec.font_sha256 = None;
        spec.font_face = None;
    }
    let c = finalize(&mut spec, doc, ctx, ch.font_given && strict(a))?;
    let l = doc.layer_mut(&path);
    l.content = LayerKind::Text { spec: spec.to_value(Some(&old)) };
    let l = doc.layer(&path);
    Ok(result_json(l, &spec, &c))
}

pub(crate) fn text_edit(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    edit_common(a, doc, ctx, true)
}

pub(crate) fn text_style(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    edit_common(a, doc, ctx, false)
}

pub(crate) fn text_on_path(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let path = target_text(doc, a.raw("layer"))?;
    doc.layer(&path).check_unlocked()?;
    let old = spec_of(doc.layer(&path));
    let mut spec = TextSpec::from_value(&old);
    let d = match (a.str("d"), a.str("follow")) {
        (Some(_), Some(_)) => {
            return Err(Error::new("PX1005", "give either --d or --follow, not both")
                .with_hint("use --d 'M…' for path data, or --follow <shape layer> to reuse a shape's outline"))
        }
        (Some(d), None) if d.trim() == "none" => String::new(),
        (Some(d), None) => {
            parse_path(&d)?;
            d
        }
        (None, Some(f)) => {
            let fp = doc.resolve(&f)?;
            let fl = doc.layer(&fp);
            match &fl.content {
                LayerKind::Shape { spec } => {
                    spec.get("d").and_then(|d| d.as_str()).map(String::from).ok_or_else(|| {
                        Error::new("PX2303", format!("shape layer '{}' has no path data", fl.name))
                            .with_context("layer", &fl.name)
                    })?
                }
                _ => {
                    return Err(Error::new(
                        "PX2303",
                        format!("--follow needs a shape layer; '{}' is a {} layer", fl.name, fl.kind_name()),
                    )
                    .with_hint("draw one with `pixelox shape ellipse` or pass path data with --d")
                    .with_context("layer", &fl.name)
                    .with_context("kind", fl.kind_name()))
                }
            }
        }
        (None, None) => spec.path.clone().ok_or_else(|| {
            Error::new("PX1003", "give the path with --d 'M…' or --follow <shape layer>").with_hint(
                "pixelox text on-path doc.pxo --in-place --layer Title --d 'M 100,300 C 200,100 400,100 500,300'",
            )
        })?,
    };
    if d.is_empty() {
        // `--d none`: back to normal text at its anchor point
        spec.path = None;
        let c = finalize(&mut spec, doc, ctx, false)?;
        doc.layer_mut(&path).content = LayerKind::Text { spec: spec.to_value(Some(&old)) };
        return Ok(result_json(doc.layer(&path), &spec, &c));
    }
    Polyline::from_svg(&d)?;
    if let Some(o) = a.raw("offset") {
        Length::parse(o)?;
        spec.path_offset = o.trim().to_string();
    }
    if let Some(s) = a.raw("side") {
        spec.path_side =
            Side::parse(s).ok_or_else(|| Error::usage(format!("--side: '{s}' is not inside or outside")))?;
    }
    if let Some(f) = a.raw("fit") {
        spec.path_shrink = f.trim() == "shrink";
    }
    spec.path = Some(d);
    let c = finalize(&mut spec, doc, ctx, false)?;
    doc.layer_mut(&path).content = LayerKind::Text { spec: spec.to_value(Some(&old)) };
    let mut v = result_json(doc.layer(&path), &spec, &c);
    v["side"] = json!(spec.path_side.name());
    v["offset"] = json!(spec.path_offset);
    Ok(v)
}

pub(crate) fn text_warp(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let path = target_text(doc, a.raw("layer"))?;
    doc.layer(&path).check_unlocked()?;
    let old = spec_of(doc.layer(&path));
    let mut spec = TextSpec::from_value(&old);
    let preset = warp::parse_preset(&a.str_or("preset", "none"))?;
    spec.warp = match preset {
        None => None,
        Some(p) => Some(warp::Warp {
            preset: p,
            bend: a.f64("bend")?,
            h_distort: a.f64("h-distort")?,
            v_distort: a.f64("v-distort")?,
            vertical: a.str_or("orient", "horizontal").trim() == "vertical",
        }),
    };
    let c = finalize(&mut spec, doc, ctx, false)?;
    doc.layer_mut(&path).content = LayerKind::Text { spec: spec.to_value(Some(&old)) };
    let mut v = result_json(doc.layer(&path), &spec, &c);
    if spec.warp.is_none() {
        v["warp"] = Value::Null;
    }
    Ok(v)
}

pub(crate) fn text_to_path(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let path = target_text(doc, a.raw("layer"))?;
    doc.layer(&path).check_unlocked()?;
    let mut spec = TextSpec::from_value(&spec_of(doc.layer(&path)));
    let c = finalize(&mut spec, doc, ctx, false)?;
    let d = svg_path_data(&c);
    if d.is_empty() {
        return Err(Error::new(
            "PX3320",
            format!("layer '{}' has no glyph outlines to convert", doc.layer(&path).name),
        )
        .with_hint(
            "give the layer visible content first: pixelox text edit <doc> --layer <name> --content \"...\" --in-place",
        ));
    }
    let shape = json!({
        "d": d,
        "fill": spec.color.to_hex(),
        "stroke": c.stroke.map(|(s, _)| s.to_hex()),
        "stroke_width": c.stroke.map(|(_, w)| w).unwrap_or(0.0),
        "join": "round",
        "fill_rule": "nonzero",
    });
    let l = doc.layer_mut(&path);
    l.content = LayerKind::Shape { spec: shape };
    let l = doc.layer(&path);
    Ok(json!({
        "layer": l.name,
        "id": l.id,
        "kind": "shape",
        "glyphs": c.glyphs,
        "d_bytes": d.len(),
        "bounds": c.ink_rect().map(|r| r.to_json()),
        "font": c.primary.describe(false),
    }))
}

pub(crate) fn text_list(a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    let with_fonts = a.flag("fonts");
    let mut layers = Vec::new();
    let mut fonts_used: Vec<Value> = Vec::new();
    for (_, l) in doc.walk() {
        let LayerKind::Text { spec: raw } = &l.content else { continue };
        let spec = TextSpec::from_value(raw);
        let c = compose(&spec, doc, false)?;
        let mut v = json!({
            "id": l.id,
            "name": l.name,
            "content": spec.content,
            "font": spec.font,
            "face": c.primary.describe(with_fonts),
            "font_found": c.found,
            "font_resolved": c.resolution.describe(&spec.font),
            "size": spec.size,
            "size_px": round2(c.size_px),
            "weight": spec.weight.unwrap_or(c.resolution.weight),
            "style": spec.style.name(),
            "color": spec.color.to_hex(),
            "align": spec.align.name(),
            "lines": c.lines,
            "on_path": spec.path.is_some(),
            "visible": l.visible,
            "bounds": c.ink_rect().map(|r| r.to_json()),
        });
        if let Some(s) = spec.stroke {
            v["stroke"] = json!({"color": s.to_hex(), "width": spec.stroke_width});
        }
        extras_json(&spec, &c, &mut v);
        if !c.synthetic.is_empty() {
            v["synthetic"] = json!(c.synthetic);
        }
        if with_fonts {
            if !c.fallbacks.is_empty() {
                v["fallback_fonts"] = json!(c.fallbacks.iter().map(|f| f.describe(true)).collect::<Vec<_>>());
            }
            for f in std::iter::once(&c.primary).chain(&c.fallbacks) {
                let d = f.describe(true);
                if !fonts_used.contains(&d) {
                    fonts_used.push(d);
                }
            }
        }
        layers.push(v);
    }
    let mut out = json!({"count": layers.len(), "layers": layers});
    if with_fonts {
        out["fonts"] = json!(fonts_used);
    }
    Ok(out)
}

/// The `text *` op specs.
pub(crate) fn text_ops() -> Vec<OpSpec> {
    vec![
        OpSpec::doc("text add", Category::Text, "Add a live text layer", text_add)
            .about(
                "Creates a live (re-editable) text layer on top of the stack (or where --above, --below, or --into \
                 say). Text is shaped with rustybuzz \
                 (kerning, ligatures, bidi/RTL, complex scripts), wraps inside --box, and is drawn from glyph \
                 outlines without hinting, so it renders identically everywhere. --at places the anchor point of the \
                 text box (--anchor) on the canvas (--anchor-to ink places the visible letters instead). Fonts are \
                 matched by family (case-insensitive) in CSS order: width (--stretch), then style, then weight, so a \
                 weight never swaps in a condensed face. --font @class picks the best installed face of a font class \
                 (`pixelox fonts classes`). A family that is not installed is replaced by the closest installed face of \
                 its class with warning PXW4010 naming the substitute and the `pixelox fonts get` command (-y downloads \
                 a Google Fonts family first; --strict-font or PIXELOX_STRICT_FONTS=1 fails instead). Generic names map \
                 to a system default and then to the embedded Tuffy fallback. Missing glyphs fall back through a font \
                 chain (PXW4001); a missing bold or italic face is synthesized (PXW4002). Live type settings: --case, \
                 --scale-x/--scale-y, --faux-bold, --fit-width/--fit-box (sized on the visible letters, refit on every \
                 edit), and --warp presets on the glyph outlines. The resolved face and its SHA-256 are recorded for \
                 replay.",
            )
            .param(param("content", ParamKind::Str).required().help("The text; \\n in the value starts a new paragraph"))
            .params(style_params(true))
            .params(place_params(true))
            .param(common::name().help("Layer name (default: the first words of the content)"))
            .params(common::placement())
            .example(
                "pixelox text add poster.pxo --in-place --content \"Hello\" --font Inter --size 64pt --color '#ffffff' --at 50%,50% --anchor c",
                "Centered white headline",
            )
            .example(
                "pixelox text add brand.pxo --in-place --name Headline --content \"FALL DROP 09\" --font \"Archivo Black\" --size 64pt --color '#101014' --at 50%,120 --anchor t --align center",
                "Re-set a PSD headline as live text",
            )
            .example(
                "pixelox text add card.pxo -o card.png --content \"Every line of this paragraph wraps inside the box.\" --box 600x400 --at 40,40 --align justify --leading 140%",
                "Justified paragraph in a box",
            )
            .example(
                "pixelox text add badge.pxo --in-place --name Arc --content \"LIMITED EDITION\" --font Inter --weight 800 --size 54pt --tracking 60",
                "Bold, letter-spaced text (then flow it with text on-path)",
            )
            .example(
                "pixelox text add poster.pxo --in-place --name L1 --content \"MAKE\" --font @condensed-heavy --fit-width 920 --at 80,90 --anchor-to ink --color '#111111'",
                "A headline whose letters span exactly 920 px from the margin",
            )
            .example(
                "pixelox text add poster.pxo --in-place --name Script --content \"gospel night\" --font @script-formal --case title --size 12% --at 50%,40% --anchor c",
                "A formal script word from whatever script face is installed",
            )
            .example(
                "pixelox text add poster.pxo --in-place --name Arc --content \"SUMMER JAM\" --font @heavy-wide --size 9% --warp arc --bend 35 --at 50%,30% --anchor c",
                "Arched headline that stays live text",
            )
            .example(
                "pixelox text add poster.pxo --in-place --content \"FESTIVAL\" --font \"Bebas Neue\" --size 15% --at 50%,50% --anchor c -y",
                "Download the Google Fonts family first, with consent",
            )
            .example(
                "pixelox text add poster.pxo --in-place --name Ghost --content \"SUMMER\" --size 30% --color '#ffffff22' --at 0-120,40% --below Subject",
                "Huge ghost type behind a subject, running off the left edge",
            ),
        OpSpec::doc("text edit", Category::Text, "Change a text layer's content or style", text_edit)
            .about(
                "Re-sets the content, position, and/or any style flag of a text layer; flags not given keep their \
                 values, and the layer stays live. Changing the font, weight, style, or variation re-resolves the face.",
            )
            .param(param("layer", ParamKind::Layer).help(layer_help()))
            .param(param("content", ParamKind::Str).help("New text; \\n in the value starts a new paragraph"))
            .params(style_params(false))
            .params(place_params(false))
            .example(
                "pixelox text edit poster.pxo --in-place --layer Headline --content \"FALL DROP 10\"",
                "Replace the words, keep the styling",
            )
            .example(
                "pixelox text edit poster.pxo --in-place --layer Headline --size 72pt --color '#e11d48'",
                "Bigger and red",
            )
            .example(
                "pixelox text edit poster.pxo -o poster.png --layer @top --content \"Sold out\" --at 50%,90% --anchor b",
                "New words at a new position, exported",
            ),
        OpSpec::doc("text style", Category::Text, "Restyle a text layer, keeping its content", text_style)
            .about(
                "The styling-only form of `text edit`: font, size, weight, width (--stretch), style, variation, color, \
                 stroke, alignment, tracking, leading, kerning, case, scale, faux bold, fit, and warp. It never changes \
                 the words (`--content` is rejected) or the position.",
            )
            .param(param("layer", ParamKind::Layer).help(layer_help()))
            .params(style_params(false))
            .example("pixelox text style poster.pxo --in-place --layer Headline --font Inter --weight bold", "Switch to Inter Bold")
            .example(
                "pixelox text style poster.pxo --in-place --layer Headline --tracking 80 --align center",
                "Letter-space and center",
            )
            .example(
                "pixelox text style poster.pxo --in-place --layer Caption --stroke '#000000' --stroke-width 3",
                "Outline the glyphs",
            )
            .example(
                "pixelox text style poster.pxo --in-place --layer Headline --scale-x 85% --faux-bold 15",
                "Condense the letters and add a little weight, still live",
            )
            .example(
                "pixelox text style poster.pxo --in-place --layer Headline --font \"Helvetica Neue\" --weight 900 --stretch condensed",
                "Ask for the condensed width explicitly (a weight alone never changes the width)",
            )
            .example("pixelox text style poster.pxo --in-place --layer Headline --warp flag --bend 30", "Wave the headline"),
        OpSpec::doc("text on-path", Category::Text, "Flow a text layer along a path or shape", text_on_path)
            .about(
                "Sets the text along SVG path data (--d) or the outline of a shape layer (--follow): glyph advances are \
                 measured along the path by arc length and each glyph is rotated to the path's tangent. --offset moves \
                 the start (aligned by --align); --side outside keeps glyphs outside a closed path (left of an open \
                 path's direction), inside flips them to the other side. The path is stored with the layer, so later \
                 edits keep the text on it.",
            )
            .param(param("layer", ParamKind::Layer).help(layer_help()))
            .param(param("d", ParamKind::Str).help(
                "SVG path data in canvas px, e.g. 'M 500,180 A 320,320 0 1 1 499.9,180'; none takes the text off its path",
            ))
            .param(param("follow", ParamKind::Layer).help("A shape layer whose path the text follows (instead of --d)"))
            .param(
                param("offset", ParamKind::Length)
                    .default("0")
                    .help("Where the text starts along the path: % of its length or px"),
            )
            .param(
                param("side", ParamKind::Choice)
                    .choices(&["outside", "inside"])
                    .default("outside")
                    .help("Glyphs outside or inside a closed path (above or below an open one)"),
            )
            .param(param("fit", ParamKind::Choice).choices(&["none", "shrink"]).help(
                "shrink: lower the font size until the text fits on the path (kept for later edits); none: leave the size (glyphs that do not fit raise PXW4012)",
            ))
            .example(
                "pixelox text on-path badge.pxo --in-place --layer Arc --d 'M 500,180 A 320,320 0 1 1 499.9,180'",
                "Text around a circle",
            )
            .example("pixelox text on-path badge.pxo --in-place --layer Arc --follow Ring --offset 25%", "Follow a shape layer")
            .example(
                "pixelox text on-path badge.pxo --in-place --layer Bottom --follow Ring --side inside --offset 25% --dry-run --json",
                "Plan text along the inside of a ring",
            )
            .example(
                "pixelox text on-path badge.pxo --in-place --layer Arc --follow Ring --fit shrink",
                "Shrink the type until it closes the circle without overlap",
            ),
        OpSpec::doc("text warp", Category::Text, "Warp a text layer with a preset, keeping it live", text_warp)
            .about(
                "Bends the glyph outlines with one of Photoshop's warp presets (arc, arc-lower, arc-upper, arch, bulge, \
                 shell-lower, shell-upper, flag, wave, fish, rise, fisheye, inflate, squeeze, twist): the text stays vector, \
                 crisp at any size, and editable (`text edit` re-warps the new words). --bend is the strength (-100..100, \
                 negative bends the other way), --h-distort / --v-distort add a perspective, and --orient vertical bends \
                 along the vertical axis. The warp fills the bounds of the unwarped letters. --preset none removes it. \
                 `text style --warp` sets the same thing. `pixelox docs text-warp` shows every preset.",
            )
            .param(param("layer", ParamKind::Layer).help(layer_help()))
            .param(
                param("preset", ParamKind::Choice)
                    .choices(&WARP_CHOICES)
                    .required()
                    .help("arc, arc-lower, arc-upper, arch, bulge, shell-lower, shell-upper, flag, wave, fish, rise, fisheye, inflate, squeeze, twist, or none"),
            )
            .param(param("bend", ParamKind::Float).default("50").range("-100..100").help("Strength in % (negative bends the other way)"))
            .param(
                param("h-distort", ParamKind::Float)
                    .default("0")
                    .range("-100..100")
                    .help("Horizontal distortion in %: a perspective that grows the right side (negative: the left)"),
            )
            .param(
                param("v-distort", ParamKind::Float)
                    .default("0")
                    .range("-100..100")
                    .help("Vertical distortion in %: a perspective that grows the bottom (negative: the top)"),
            )
            .param(
                param("orient", ParamKind::Choice)
                    .choices(&["horizontal", "vertical"])
                    .default("horizontal")
                    .help("Bend along the horizontal or the vertical axis"),
            )
            .example("pixelox text warp poster.pxo --in-place --layer Title --preset arc --bend 40", "A rainbow arc")
            .example("pixelox text warp poster.pxo --in-place --layer Title --preset rise --bend 30 --h-distort 20", "Rising, growing to the right")
            .example("pixelox text warp poster.pxo --in-place --layer Title --preset flag --bend -25 --json", "A waving banner")
            .example("pixelox text warp poster.pxo --in-place --layer Title --preset none", "Remove the warp"),
        OpSpec::doc("text to-path", Category::Text, "Convert a text layer to a vector shape layer", text_to_path)
            .about(
                "Replaces the text layer with a shape layer (same name, id, and stack position, styles kept) whose path \
                 data is the glyph outlines in canvas px, filled with the text color and stroked like the text. The \
                 words are no longer editable; path booleans and outline effects now apply.",
            )
            .param(param("layer", ParamKind::Layer).help(layer_help()))
            .example("pixelox text to-path logo.pxo --in-place --layer Wordmark", "Outline a wordmark")
            .example("pixelox text to-path logo.pxo -o logo-outlined.pxo --layer @top", "Outline into a new document")
            .example("pixelox text to-path logo.pxo --in-place --layer Wordmark --dry-run --json", "Preview the conversion"),
        OpSpec::doc("text list", Category::Text, "List text layers and the fonts they use", text_list)
            .read_only()
            .about(
                "Lists every text layer (top first) with its content, requested and resolved font, size, color, \
                 bounds, and whether it follows a path. --fonts adds the font files, their SHA-256, and fallback faces.",
            )
            .param(param("fonts", ParamKind::Flag).help("Also list the font files used (paths, SHA-256, fallbacks)"))
            .example("pixelox text list poster.pxo", "Text layers of a document")
            .example("pixelox text list poster.pxo --fonts --json", "With the font files, for packaging")
            .example("pixelox text list badge.pxo --json", "Machine-readable listing"),
    ]
}
