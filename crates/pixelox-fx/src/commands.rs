//! The `fx *` commands: one doc op per style (add or replace that style on a layer) plus
//! `list`, `rm`, `copy`, and `scale` for managing a layer's style stack.

use crate::render::{fmt_len, fmt_num, parse_num, value_str, DEFAULT_STOPS};
use pixelox_core::doc::{FxSpec, LayerPath};
use pixelox_core::op::ParamSpec;
use pixelox_core::prelude::*;
use pixelox_core::units::PointSpec;
use std::path::Path;

/// Blend modes an effect accepts (every layer mode except `pass-through`).
const FX_BLENDS: &[&str] = &[
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
];

/// `fx rm --kind` values: the style kinds, plus one overlay type at a time.
const RM_KINDS: &[&str] = &[
    "drop-shadow",
    "inner-shadow",
    "outer-glow",
    "inner-glow",
    "bevel",
    "satin",
    "overlay",
    "stroke",
    "color-overlay",
    "gradient-overlay",
    "pattern-overlay",
];

// ---------------------------------------------------------------------------------------
// shared params

/// `--blend`, `--opacity`, `--enabled`: common to every style.
fn look(blend: &'static str, opacity: &'static str, blend_help: &'static str) -> Vec<ParamSpec> {
    vec![
        param("blend", ParamKind::Choice).choices(FX_BLENDS).default(blend).help(blend_help),
        param("opacity", ParamKind::Float).range("0..100").default(opacity).help("Opacity of the effect in percent"),
        param("enabled", ParamKind::Switch)
            .default("on")
            .help("on|off; given alone, turns the layer's existing effect on or off and keeps its settings"),
    ]
}

fn offset(default: &'static str, help: &'static str) -> ParamSpec {
    param("offset", ParamKind::Point).default(default).help(help)
}

fn blur(default: &'static str, help: &'static str) -> ParamSpec {
    param("blur", ParamKind::Float).range("0..1000").default(default).help(help)
}

fn color(default: &'static str, help: &'static str) -> ParamSpec {
    param("color", ParamKind::Color).default(default).help(help)
}

fn stops(help: &'static str) -> ParamSpec {
    param("stops", ParamKind::Stops).help(help)
}

fn from_tile(help: &'static str) -> ParamSpec {
    param("from", ParamKind::Path).help(help)
}

fn tile_scale() -> ParamSpec {
    param("scale", ParamKind::Float).range("1..1000").default("100").help("Pattern tile scale in percent")
}

fn grad_angle(help: &'static str) -> ParamSpec {
    param("angle", ParamKind::Angle).default("90").help(help)
}

/// Every command this crate defines.
pub fn ops() -> Vec<OpSpec> {
    vec![
        OpSpec::doc("fx drop-shadow", Category::Fx, "Add or replace a layer's drop shadow", drop_shadow)
            .about(
                "A soft shadow of the layer's shape drawn beneath it, offset by --offset. --blur is the blur radius \
                 (Gaussian sigma = radius / 2, as in CSS); --spread turns that percentage of the radius into a hard \
                 grow before blurring. The style is live: it re-renders when the layer changes, grows the layer's \
                 bounds, and replaces any drop shadow the layer already has.",
            )
            .param(common::layer())
            .param(offset("4,4", "Shadow offset DX,DY in px (positive = right and down; % is of the canvas)"))
            .param(blur("8", "Blur radius in px (Gaussian sigma = radius / 2)"))
            .param(param("spread", ParamKind::Float).range("0..100").default("0").help("Percent of the blur radius grown hard before blurring"))
            .param(color("#000000", "Shadow color; its alpha multiplies --opacity"))
            .params(look("multiply", "75", "Blend mode of the shadow"))
            .example("pixelox fx drop-shadow badge.pxo --in-place --layer Burst --offset 0,10 --blur 24 --color '#00000066'", "Soft shadow below a layer")
            .example("pixelox fx drop-shadow logo.png -o logo-shadow.png --offset 6,6 --blur 4 --spread 30 --opacity 60", "Tight shadow on a single-layer image")
            .example("pixelox fx drop-shadow doc.pxo --in-place --layer Title --enabled off", "Turn the existing shadow off, keeping its settings"),
        OpSpec::doc("fx inner-shadow", Category::Fx, "Add or replace a layer's inner shadow", inner_shadow)
            .about(
                "A shadow cast inside the layer's edges, as if the shape were cut out of the surface. The area outside \
                 the shape is offset by --offset, grown by --choke percent of the blur radius, blurred, and drawn \
                 clipped to the layer's pixels. Replaces any inner shadow the layer already has.",
            )
            .param(common::layer())
            .param(offset("4,4", "Light offset DX,DY in px: the shadow falls along the edges facing away from it"))
            .param(blur("8", "Blur radius in px (Gaussian sigma = radius / 2)"))
            .param(param("choke", ParamKind::Float).range("0..100").default("0").help("Percent of the blur radius grown hard into the shape before blurring"))
            .param(color("#000000", "Shadow color; its alpha multiplies --opacity"))
            .params(look("multiply", "75", "Blend mode of the shadow"))
            .example("pixelox fx inner-shadow doc.pxo --in-place --layer Button --offset 0,3 --blur 6", "Pressed-in button")
            .example("pixelox fx inner-shadow doc.pxo --in-place --layer Title --offset 2,2 --blur 4 --choke 25 --color '#3b0764'", "Tight colored inner shadow")
            .example("pixelox fx inner-shadow doc.pxo --in-place --layer Card --offset 0,-4 --blur 10 --blend normal --opacity 40", "Shadow along the bottom edge"),
        OpSpec::doc("fx outer-glow", Category::Fx, "Add or replace a layer's outer glow", outer_glow)
            .about(
                "A glow around the layer's shape, drawn beneath it: the shape grown by --spread percent of the radius \
                 and blurred, at full strength at the edge. Color it with --color or a gradient with --stops (the \
                 first stop at the edge, the last at the outer fade). Replaces any outer glow the layer has.",
            )
            .param(common::layer())
            .param(blur("10", "Glow radius in px (Gaussian sigma = radius / 2)"))
            .param(param("spread", ParamKind::Float).range("0..100").default("0").help("Percent of the radius grown hard before blurring"))
            .param(color("#ffffbe", "Glow color (ignored with --stops)"))
            .param(stops("Gradient glow: stops from the layer's edge (0) outward (1), instead of --color"))
            .params(look("screen", "75", "Blend mode of the glow"))
            .example("pixelox fx outer-glow doc.pxo --in-place --layer Title --blur 18 --color '#38bdf8'", "Blue glow around a title")
            .example("pixelox fx outer-glow doc.pxo --in-place --layer Neon --blur 30 --spread 15 --stops '#fde047@0,#f97316@1'", "Gradient neon glow")
            .example("pixelox fx outer-glow logo.png -o glow.png --blur 12 --blend normal --opacity 90", "Glow on a single-layer image"),
        OpSpec::doc("fx inner-glow", Category::Fx, "Add or replace a layer's inner glow", inner_glow)
            .about(
                "A glow inside the layer's shape, clipped to its pixels: from the edges inward (--source edge) or from \
                 the middle outward (--source center). --choke grows it hard by that percent of the radius before \
                 blurring. Color it with --color or a gradient with --stops (the first stop at the source).",
            )
            .param(common::layer())
            .param(blur("10", "Glow radius in px (Gaussian sigma = radius / 2)"))
            .param(param("choke", ParamKind::Float).range("0..100").default("0").help("Percent of the radius grown hard before blurring"))
            .param(param("source", ParamKind::Choice).choices(&["edge", "center"]).default("edge").help("Where the glow starts: the shape's edges or its center"))
            .param(color("#ffffbe", "Glow color (ignored with --stops)"))
            .param(stops("Gradient glow: stops from the source (0) to the fade (1), instead of --color"))
            .params(look("screen", "75", "Blend mode of the glow"))
            .example("pixelox fx inner-glow doc.pxo --in-place --layer Title --blur 8", "Soft light rim inside a title")
            .example("pixelox fx inner-glow doc.pxo --in-place --layer Orb --blur 20 --source center --color '#ffffff'", "Glowing center")
            .example("pixelox fx inner-glow doc.pxo --in-place --layer Badge --blur 6 --choke 40 --blend normal --color '#fbbf24'", "Hard golden inner rim"),
        OpSpec::doc("fx bevel", Category::Fx, "Add or replace a layer's bevel & emboss", bevel)
            .about(
                "Shades the layer's edges as if raised: a height map from the blurred shape lit by a distant light at \
                 --angle (0 = from the right, 90 = from above) and --altitude, with highlights in --highlight \
                 (--blend, default screen) and shadows in --shadow (--shadow-blend, default multiply). Styles: inner \
                 (inside the edge), outer (outside it), emboss (across it), pillow (raised inside, pressed in outside).",
            )
            .param(common::layer())
            .param(param("style", ParamKind::Choice).choices(&["inner", "outer", "emboss", "pillow"]).default("inner").help("Where the bevel sits relative to the edge"))
            .param(param("depth", ParamKind::Float).range("1..1000").default("100").help("Steepness of the bevel in percent"))
            .param(param("size", ParamKind::Float).range("0..250").default("5").help("Width of the bevel in px"))
            .param(param("soften", ParamKind::Float).range("0..16").default("0").help("Blur of the shading in px"))
            .param(param("angle", ParamKind::Angle).default("120").help("Light direction in degrees (0 = from the right, 90 = from above)"))
            .param(param("altitude", ParamKind::Float).range("0..90").default("30").help("Light height above the surface in degrees"))
            .param(param("direction", ParamKind::Choice).choices(&["up", "down"]).default("up").help("Raised (up) or sunken (down)"))
            .param(param("highlight", ParamKind::Color).default("#ffffff").help("Highlight color"))
            .param(param("shadow", ParamKind::Color).default("#000000").help("Shadow color"))
            .param(param("shadow-blend", ParamKind::Choice).choices(FX_BLENDS).default("multiply").help("Blend mode of the shadow side"))
            .params(look("screen", "75", "Blend mode of the highlight side"))
            .example("pixelox fx bevel doc.pxo --in-place --layer Button --style inner --depth 120 --size 6", "Raised button")
            .example("pixelox fx bevel doc.pxo --in-place --layer Logo --style emboss --depth 200 --size 10 --soften 2 --angle 135 --altitude 40", "Embossed logo")
            .example("pixelox fx bevel doc.pxo --in-place --layer Stamp --style pillow --depth 80 --size 8 --direction down", "Stamped-in look"),
        OpSpec::doc("fx satin", Category::Fx, "Add or replace a layer's satin sheen", satin)
            .about(
                "Silky interior shading: the difference of two copies of the shape offset by +/- --offset and blurred, \
                 colored and clipped to the layer's pixels. --invert swaps the light and dark bands.",
            )
            .param(common::layer())
            .param(offset("8,8", "Offset DX,DY in px of the two shape copies"))
            .param(blur("14", "Blur radius in px (Gaussian sigma = radius / 2)"))
            .param(color("#000000", "Satin color; its alpha multiplies --opacity"))
            .param(param("invert", ParamKind::Flag).help("Invert the satin bands"))
            .params(look("multiply", "50", "Blend mode of the satin"))
            .example("pixelox fx satin doc.pxo --in-place --layer Title --offset 8,6 --blur 12", "Dark satin sheen")
            .example("pixelox fx satin doc.pxo --in-place --layer Ribbon --offset 10,4 --blur 16 --color '#7c3aed' --invert", "Inverted purple satin")
            .example("pixelox fx satin doc.pxo --in-place --layer Medal --offset 6,6 --blur 10 --blend overlay --opacity 35", "Subtle overlay satin"),
        OpSpec::doc("fx overlay", Category::Fx, "Add a color, gradient, or pattern overlay", overlay)
            .about(
                "Paints over the layer's pixels (clipped to them) with a color, a gradient (--stops at --angle across \
                 the layer's bounds; 0 = left to right, 90 = bottom to top), or a pattern tile (--from, embedded in \
                 the document, tiled from the canvas origin at --scale). --kind defaults to the fill you give. A layer \
                 holds one overlay of each kind; this replaces the overlay of the same kind.",
            )
            .param(common::layer())
            .param(param("kind", ParamKind::Choice).choices(&["color", "gradient", "pattern"]).auto().default("auto").help("Overlay kind; auto picks it from --color, --stops, or --from"))
            .param(color("#000000", "Overlay color (kind color)"))
            .param(stops("Gradient stops (kind gradient; default black to white)"))
            .param(grad_angle("Gradient angle in degrees (0 = left to right, 90 = bottom to top)"))
            .param(param("style", ParamKind::Choice).choices(&["linear", "radial", "reflected"]).default("linear").help("Gradient shape"))
            .param(param("reverse", ParamKind::Flag).help("Reverse the gradient"))
            .param(from_tile("Pattern tile image (kind pattern), embedded in the document"))
            .param(tile_scale())
            .params(look("normal", "100", "Blend mode of the overlay"))
            .example("pixelox fx overlay doc.pxo --in-place --layer Icon --kind color --color '#ffffff'", "Recolor an icon white")
            .example("pixelox fx overlay doc.pxo --in-place --layer Title --kind gradient --stops '#f97316@0,#db2777@1' --angle 90", "Gradient title")
            .example("pixelox fx overlay doc.pxo --in-place --layer Panel --kind pattern --from tile.png --scale 50 --blend multiply --opacity 30", "Textured panel"),
        OpSpec::doc("fx stroke", Category::Fx, "Add or replace a stroke along a layer's edges", stroke)
            .about(
                "Outlines the layer's shape --width px wide, outside (beneath the content), inside (clipped to it), or \
                 centered on the edge. Fill it with --color, a gradient (--stops at --angle across the stroke's \
                 bounds), or a pattern tile (--from, embedded in the document). An outside or center stroke grows the \
                 layer's bounds. Replaces any stroke the layer already has.",
            )
            .param(common::layer())
            .param(param("width", ParamKind::Float).range("0..1000").default("3").help("Stroke width in px"))
            .param(param("position", ParamKind::Choice).choices(&["inside", "center", "outside"]).default("outside").help("Where the stroke sits relative to the edge"))
            .param(color("#000000", "Stroke color"))
            .param(stops("Gradient fill stops, instead of --color"))
            .param(grad_angle("Gradient angle in degrees (0 = left to right, 90 = bottom to top)"))
            .param(from_tile("Pattern tile image to fill the stroke with, embedded in the document"))
            .param(tile_scale())
            .params(look("normal", "100", "Blend mode of the stroke"))
            .example("pixelox fx stroke badge.pxo --in-place --layer Arc --width 6 --position outside --color '#ffffff'", "White outline")
            .example("pixelox fx stroke doc.pxo --in-place --layer Title --width 3 --position center --stops '#22d3ee@0,#6366f1@1' --angle 0", "Gradient outline")
            .example("pixelox fx stroke doc.pxo --in-place --layer Photo --width 12 --position inside --from tile.png", "Patterned inner frame"),
        OpSpec::doc("fx list", Category::Fx, "List a layer's style effects", list)
            .about("Shows the layer's style stack in order with each effect's settings (the params its command stored) and the layer's bounds with the styles applied.")
            .param(common::layer())
            .read_only()
            .example("pixelox fx list doc.pxo --layer Title", "Show the styles on a layer")
            .example("pixelox fx list doc.pxo --layer '#1' --json", "Styles on the top layer as JSON")
            .example("pixelox fx list badge.pxo --layer Burst --json", "Check a shadow's settings"),
        OpSpec::doc("fx rm", Category::Fx, "Remove style effects from a layer", rm)
            .about("Removes the effects of one kind (--kind; color-overlay, gradient-overlay, or pattern-overlay remove one overlay) or every effect (--all) from a layer.")
            .param(common::layer())
            .param(param("kind", ParamKind::Choice).choices(RM_KINDS).help("Effect kind to remove"))
            .param(param("all", ParamKind::Flag).help("Remove every effect"))
            .example("pixelox fx rm doc.pxo --in-place --layer Title --kind drop-shadow", "Remove a drop shadow")
            .example("pixelox fx rm doc.pxo --in-place --layer Title --all", "Clear all styles")
            .example("pixelox fx rm doc.pxo --in-place --layer Panel --kind gradient-overlay", "Remove only the gradient overlay"),
        OpSpec::doc("fx copy", Category::Fx, "Copy a layer's style stack to other layers", copy)
            .about("Replaces the style stack of every --to layer with a copy of the --from layer's stack (like Photoshop's Copy/Paste Layer Style). Pattern tiles stay shared inside the document.")
            .param(param("from", ParamKind::Layer).required().help("Layer to copy the styles from"))
            .param(param("to", ParamKind::List).required().help("Comma-separated layers to paste the styles onto"))
            .example("pixelox fx copy doc.pxo --in-place --from Title --to Subtitle", "Match a subtitle's style to the title")
            .example("pixelox fx copy doc.pxo --in-place --from 'Button 1' --to 'Button 2,Button 3'", "Style a row of buttons alike")
            .example("pixelox fx copy doc.pxo -o styled.pxo --from @top --to '#2,#3'", "Copy the top layer's styles down"),
        OpSpec::doc("fx scale", Category::Fx, "Scale all effect geometry on a layer", scale)
            .about("Multiplies every effect's pixel sizes on the layer by --by percent: offsets, blur radii, bevel size and soften, stroke width, and pattern scale. Percent settings (spread, choke, depth) are relative and unchanged. Values beyond a setting's range are clamped (PXW3001).")
            .param(common::layer())
            .param(param("by", ParamKind::Float).range("1..1000").required().help("Scale factor in percent (200 doubles every effect size)"))
            .example("pixelox fx scale doc.pxo --in-place --layer Title --by 200", "Double the styles after enlarging a layer")
            .example("pixelox fx scale doc.pxo --in-place --layer Badge --by 50", "Halve the styles")
            .example("pixelox fx scale doc.pxo -o big.pxo --layer Logo --by 150 --dry-run --json", "Preview the scaled settings"),
    ]
}

// ---------------------------------------------------------------------------------------
// helpers

fn styleable(layer: &Layer) -> Result<()> {
    if matches!(layer.content, LayerKind::Adjustment { .. }) {
        return Err(Error::new(
            "PX2303",
            format!("layer '{}' is an adjustment layer; layer styles need pixels", layer.name),
        )
        .with_hint("apply the style to a pixel, text, shape, placed, or group layer")
        .with_context("layer", &layer.name)
        .with_context("kind", "adjustment"));
    }
    Ok(())
}

/// The overlay type of a spec (`color`, `gradient`, `pattern`), inferred like the renderer.
fn overlay_type(f: &FxSpec) -> String {
    let get = |k: &str| f.params.get(k).and_then(value_str);
    match get("kind").map(|k| k.trim().to_ascii_lowercase()).filter(|k| !k.is_empty() && k != "auto") {
        Some(k) => k,
        None if get("asset").is_some() => "pattern".into(),
        None if get("stops").is_some() => "gradient".into(),
        None => "color".into(),
    }
}

/// A display label: the kind, or `<type>-overlay` for overlays.
fn label(f: &FxSpec) -> String {
    if f.kind == "overlay" {
        format!("{}-overlay", overlay_type(f))
    } else {
        f.kind.clone()
    }
}

/// Does `f` match `kind` (and, for overlays, the overlay type `sub`; `None` = any)?
fn matches(f: &FxSpec, kind: &str, sub: Option<&str>) -> bool {
    f.kind == kind && sub.is_none_or(|s| overlay_type(f) == s)
}

fn stack_json(l: &Layer) -> Value {
    Value::Array(l.fx.iter().map(|f| json!({"fx": label(f), "enabled": f.enabled})).collect())
}

fn bounds_json(doc: &Document, path: &LayerPath) -> Value {
    compose::layer_bounds(doc, doc.layer(path)).ok().flatten().map(|r| r.to_json()).unwrap_or(Value::Null)
}

/// Resolve the overlay type from `--kind` and the fill flags given.
fn overlay_kind(a: &Args) -> Result<&'static str> {
    let given = match a.raw("kind").map(str::trim) {
        None | Some("auto") | Some("") => None,
        Some("color") => Some("color"),
        Some("gradient") => Some("gradient"),
        Some("pattern") => Some("pattern"),
        Some(other) => {
            return Err(Error::new("PX1010", format!("--kind: '{other}' is not one of: color, gradient, pattern")))
        }
    };
    let inferred = fill_given(a);
    match (given, inferred) {
        (Some(k), None) => Ok(k),
        (Some(k), Some(i)) if k == i => Ok(k),
        (Some(k), Some(i)) => Err(Error::new("PX1005", format!("--kind {k} conflicts with the {i} fill given"))
            .with_hint(format!("drop --kind, or use --kind {i}"))),
        (None, i) => Ok(i.unwrap_or("color")),
    }
}

/// The fill type implied by explicit `--from` / `--stops` / `--color`.
fn fill_given(a: &Args) -> Option<&'static str> {
    if a.is_explicit("from") {
        Some("pattern")
    } else if a.is_explicit("stops") {
        Some("gradient")
    } else if a.is_explicit("color") {
        Some("color")
    } else {
        None
    }
}

/// Fail with PX1005 when more than one of the fill flags is given.
fn one_fill(a: &Args) -> Result<()> {
    let given: Vec<String> =
        ["color", "stops", "from"].iter().filter(|k| a.is_explicit(k)).map(|k| format!("--{k}")).collect();
    if given.len() > 1 {
        return Err(Error::new("PX1005", format!("give only one of {}", given.join(", "))).with_context("flags", given));
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------
// style commands

fn drop_shadow(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    set_style("drop-shadow", a, doc, ctx)
}
fn inner_shadow(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    set_style("inner-shadow", a, doc, ctx)
}
fn outer_glow(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    set_style("outer-glow", a, doc, ctx)
}
fn inner_glow(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    set_style("inner-glow", a, doc, ctx)
}
fn bevel(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    set_style("bevel", a, doc, ctx)
}
fn satin(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    set_style("satin", a, doc, ctx)
}
fn overlay(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    set_style("overlay", a, doc, ctx)
}
fn stroke(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    set_style("stroke", a, doc, ctx)
}

/// Add or replace one effect of `kind` on `--layer`; `--enabled` alone toggles it.
fn set_style(kind: &'static str, a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let path = target_layer(a, doc)?;
    let layer = doc.layer(&path);
    styleable(layer)?;
    if !layer.visible {
        ctx.warn("PXW2001", format!("layer '{}' is hidden", layer.name));
    }
    let enabled = a.switch("enabled", true);
    let mut params = a.to_json();
    params.remove("layer");
    params.remove("enabled");
    let explicit: Vec<String> = params.keys().map(|k| k.replace('_', "-")).filter(|k| a.is_explicit(k)).collect();

    // `--enabled on|off` alone (an overlay may also name its --kind): toggle in place
    if a.is_explicit("enabled") && explicit.iter().all(|k| kind == "overlay" && k == "kind") {
        let sub = if kind == "overlay" && a.is_explicit("kind") { Some(overlay_kind(a)?) } else { None };
        let layer = doc.layer_mut(&path);
        let mut n = 0;
        for f in layer.fx.iter_mut().filter(|f| matches(f, kind, sub)) {
            f.enabled = enabled;
            n += 1;
        }
        if n > 0 {
            return Ok(result(doc, &path, kind, "toggled", enabled));
        }
        // nothing to toggle: add one with the default settings
    }

    one_fill(a)?;
    let mut sub = None;
    match kind {
        "overlay" => {
            let k = overlay_kind(a)?;
            params.insert("kind".into(), json!(k));
            ctx.resolve_param("kind", k);
            if k == "gradient" && !params.contains_key("stops") {
                params.insert("stops".into(), json!(DEFAULT_STOPS));
            }
            if k == "pattern" && !a.is_explicit("from") {
                return Err(Error::new("PX1003", "a pattern overlay needs --from tile.png")
                    .with_hint("pixelox fx overlay <doc> --kind pattern --from tile.png --in-place"));
            }
            sub = Some(k);
        }
        "stroke" => {
            params.insert("fill".into(), json!(fill_given(a).unwrap_or("color")));
        }
        _ => {}
    }
    if let Some(from) = a.str("from") {
        let p = Path::new(&from);
        let bytes = ctx.read_aux(p, "pattern", Some("from"))?;
        let ext = p.extension().map(|e| e.to_string_lossy().to_ascii_lowercase()).unwrap_or_else(|| "png".into());
        let key = doc.add_asset(bytes, &ext);
        // fail now, not at render time, when the tile cannot be decoded
        compose::decode_asset(doc, &key)?;
        params.insert("asset".into(), json!(key));
    }

    let spec = FxSpec { kind: kind.to_string(), enabled, params };
    let layer = doc.layer_mut(&path);
    let action = match layer.fx.iter().position(|f| matches(f, kind, sub)) {
        Some(i) => {
            layer.fx[i] = spec;
            "replaced"
        }
        None => {
            layer.fx.push(spec);
            "added"
        }
    };
    let mut r = result(doc, &path, kind, action, enabled);
    if let Some(s) = sub {
        r["kind"] = json!(s);
    }
    Ok(r)
}

fn result(doc: &Document, path: &LayerPath, kind: &str, action: &str, enabled: bool) -> Value {
    let l = doc.layer(path);
    json!({
        "layer": l.name,
        "fx": kind,
        "action": action,
        "enabled": enabled,
        "stack": stack_json(l),
        "bounds": bounds_json(doc, path),
    })
}

// ---------------------------------------------------------------------------------------
// list, rm, copy, scale

fn list(a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    let path = doc.resolve_or_only(a.raw("layer"))?;
    let l = doc.layer(&path);
    let fx: Vec<Value> = l
        .fx
        .iter()
        .enumerate()
        .map(|(i, f)| json!({"index": i + 1, "fx": label(f), "kind": f.kind, "enabled": f.enabled, "params": f.params}))
        .collect();
    Ok(json!({"layer": l.name, "count": fx.len(), "fx": fx, "bounds": bounds_json(doc, &path)}))
}

fn rm(a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    let kind = a.str("kind");
    let all = a.flag("all");
    match (&kind, all) {
        (Some(_), true) => return Err(Error::new("PX1005", "give --kind or --all, not both")),
        (None, false) => {
            return Err(Error::new("PX1003", "say which effects to remove with --kind K or --all")
                .with_hint("pixelox fx list <doc> --layer <name> shows the layer's effects"))
        }
        _ => {}
    }
    let path = target_layer(a, doc)?;
    let layer = doc.layer_mut(&path);
    let before = layer.fx.len();
    let mut removed = Vec::new();
    layer.fx.retain(|f| {
        let hit = match kind.as_deref() {
            None => true,
            Some(k) => match k.strip_suffix("-overlay") {
                Some(t) => matches(f, "overlay", Some(t)),
                None => f.kind == k,
            },
        };
        if hit {
            removed.push(label(f));
        }
        !hit
    });
    let after = layer.fx.len();
    let mut r = result(doc, &path, kind.as_deref().unwrap_or("all"), "removed", true);
    if let Value::Object(m) = &mut r {
        m.remove("enabled");
        m.remove("fx");
        m.insert("removed".into(), json!(removed));
        m.insert("count".into(), json!(before - after));
    }
    Ok(r)
}

fn copy(a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    let from = a.str("from").ok_or_else(|| Error::new("PX1003", "fx copy needs --from <layer>"))?;
    let src = doc.resolve(&from)?;
    let stack = doc.layer(&src).fx.clone();
    let src_name = doc.layer(&src).name.clone();
    let targets = a.list("to");
    if targets.is_empty() {
        return Err(Error::new("PX1003", "fx copy needs --to <layer>[,<layer>...]"));
    }
    // resolve and check every target before changing any
    let mut paths = Vec::new();
    for t in &targets {
        let p = doc.resolve(t)?;
        let l = doc.layer(&p);
        l.check_unlocked()?;
        styleable(l)?;
        if !paths.contains(&p) {
            paths.push(p);
        }
    }
    let mut names = Vec::new();
    for p in &paths {
        let l = doc.layer_mut(p);
        l.fx = stack.clone();
        names.push(l.name.clone());
    }
    Ok(json!({
        "from": src_name,
        "to": names,
        "stack": stack.iter().map(label).collect::<Vec<_>>(),
        "count": stack.len(),
    }))
}

/// Upper bounds of the scaled settings (their command ranges).
const SCALED: &[(&str, f64)] = &[("blur", 1000.0), ("size", 250.0), ("soften", 16.0), ("width", 1000.0)];

fn scale(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let path = target_layer(a, doc)?;
    let by = a.f64("by")?;
    let k = by / 100.0;
    let mut clamped = Vec::new();
    let layer = doc.layer_mut(&path);
    let mut report = Vec::new();
    for f in layer.fx.iter_mut() {
        let mut changed = Map::new();
        let mut set = |f: &mut FxSpec, key: &str, old: String, new: String| {
            if old != new {
                changed.insert(key.to_string(), json!([old, new]));
            }
            f.params.insert(key.to_string(), json!(new));
        };
        for &(key, max) in SCALED {
            let Some(old) = f.params.get(key).and_then(value_str) else { continue };
            let Some(v) = parse_num(&old) else { continue };
            let mut nv = v * k;
            if nv > max {
                nv = max;
                clamped.push(format!("{} {key}", label(f)));
            }
            set(f, key, old, fmt_num(nv));
        }
        if let Some(old) = f.params.get("offset").and_then(value_str) {
            let mut p = PointSpec::parse(&old)?;
            p.x.value *= k;
            p.y.value *= k;
            set(f, "offset", old, format!("{},{}", fmt_len(p.x), fmt_len(p.y)));
        }
        if f.params.contains_key("asset") {
            if let Some(old) = f.params.get("scale").and_then(value_str) {
                if let Some(v) = parse_num(&old) {
                    let nv = (v * k).clamp(1.0, 1000.0);
                    if nv != v * k {
                        clamped.push(format!("{} scale", label(f)));
                    }
                    set(f, "scale", old, fmt_num(nv));
                }
            }
        }
        report.push(json!({"fx": label(f), "changed": changed}));
    }
    if !clamped.is_empty() {
        ctx.warn_ctx(
            "PXW3001",
            format!("scaled settings were clamped to their range: {}", clamped.join(", ")),
            json!({"clamped": clamped}),
        );
    }
    let mut r = result(doc, &path, "all", "scaled", true);
    if let Value::Object(m) = &mut r {
        m.remove("enabled");
        m.remove("fx");
        m.insert("by".into(), json!(by));
        m.insert("scaled".into(), json!(report));
    }
    Ok(r)
}
