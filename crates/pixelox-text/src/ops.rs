//! The `text *` commands.

use crate::layout::{compose, parse_path, Composed, Polyline};
use crate::render::svg_path_data;
use crate::spec::{apply_place, apply_style, place_params, style_params, Side, TextSpec};
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

/// Lay out `spec`, record the face in the spec and the sidecar, and raise the warnings.
fn finalize(spec: &mut TextSpec, doc: &Document, ctx: &mut OpCtx, strict: bool) -> Result<Composed> {
    let c = compose(spec, doc, strict)?;
    if !c.found {
        ctx.warn_ctx(
            "PXW4001",
            format!("font family '{}' is not installed; the text is drawn with {}", spec.font, c.primary.family),
            json!({"family": spec.font, "fallback": c.primary.family}),
        );
    }
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
    Ok(c)
}

fn result_json(l: &Layer, spec: &TextSpec, c: &Composed) -> Value {
    let mut v = json!({
        "layer": l.name,
        "id": l.id,
        "content": spec.content,
        "font": c.primary.describe(true),
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
    v
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
    let c = finalize(&mut spec, doc, ctx, ch.font_given || a.has("font"))?;
    let name = match a.str("name").map(|n| n.trim().to_string()).filter(|n| !n.is_empty()) {
        Some(n) => {
            doc.check_name_free(&n, None)?;
            n
        }
        None => default_name(&spec.content),
    };
    let layer = doc.new_layer(&name, LayerKind::Text { spec: spec.to_value(None) });
    let above = doc.active.as_deref().and_then(|id| doc.find_by_id(id));
    let path = doc.insert_above(above.as_deref(), layer);
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
    }
    let c = finalize(&mut spec, doc, ctx, ch.font_given)?;
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
    spec.path = Some(d);
    let c = finalize(&mut spec, doc, ctx, false)?;
    doc.layer_mut(&path).content = LayerKind::Text { spec: spec.to_value(Some(&old)) };
    let mut v = result_json(doc.layer(&path), &spec, &c);
    v["side"] = json!(spec.path_side.name());
    v["offset"] = json!(spec.path_offset);
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
            "size": spec.size,
            "size_px": round2(c.size_px),
            "weight": spec.weight,
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
                "Creates a live (re-editable) text layer above the active layer. Text is shaped with rustybuzz \
                 (kerning, ligatures, bidi/RTL, complex scripts), wraps inside --box, and is drawn from glyph \
                 outlines without hinting, so it renders identically everywhere. --at places the anchor point of the \
                 text box (--anchor) on the canvas. Fonts are matched by family (case-insensitive) with weight and \
                 style; generic names map to a system default and then to the embedded Tuffy fallback. Missing \
                 glyphs fall back through a font chain (PXW4001); a missing bold or italic face is synthesized \
                 (PXW4002). The resolved face and its SHA-256 are recorded for replay.",
            )
            .param(param("content", ParamKind::Str).required().help("The text; \\n in the value starts a new paragraph"))
            .params(style_params(true))
            .params(place_params(true))
            .param(common::name().help("Layer name (default: the first words of the content)"))
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
                "The styling-only form of `text edit`: font, size, weight, style, variation, color, stroke, alignment, \
                 tracking, leading, and kerning. It never changes the words (`--content` is rejected) or the position.",
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
            ),
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
            .example(
                "pixelox text on-path badge.pxo --in-place --layer Arc --d 'M 500,180 A 320,320 0 1 1 499.9,180'",
                "Text around a circle",
            )
            .example("pixelox text on-path badge.pxo --in-place --layer Arc --follow Ring --offset 25%", "Follow a shape layer")
            .example(
                "pixelox text on-path badge.pxo --in-place --layer Bottom --follow Ring --side inside --offset 25% --dry-run --json",
                "Plan text along the inside of a ring",
            ),
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
