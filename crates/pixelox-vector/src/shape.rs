//! `shape rect|ellipse|polygon|star|line|arrow|path|set`: live vector shape layers (PRD §8.11).

use crate::spec::{self, fmt_num, parse_dash, parse_paint, ShapeSpec, PREC};
use kurbo::BezPath;
use pixelox_core::op::ParamSpec;
use pixelox_core::prelude::*;

/// How a shape is painted when neither `--fill` nor `--stroke` is given.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Paint {
    /// Closed shapes: fill black.
    Closed,
    /// Lines: stroke black.
    Line,
    /// Arrows: shaft stroked, head filled, one color.
    Arrow,
}

/// The common paint flags. `set` gets them without defaults (only given flags change).
fn paint_params(set: bool) -> Vec<ParamSpec> {
    let d = |p: ParamSpec, v: &'static str| if set { p } else { p.default(v) };
    vec![
        param("fill", ParamKind::Str).help(if set {
            "New fill color (CSS color), or none to remove the fill"
        } else {
            "Fill color (CSS color) or none; black when neither --fill nor --stroke is given"
        }),
        param("stroke", ParamKind::Str).help(if set {
            "New stroke color (CSS color), or none to remove the stroke"
        } else {
            "Stroke color (CSS color) or none (default none)"
        }),
        param("stroke-width", ParamKind::Float).range("0..10000").help(if set {
            "New stroke width in px"
        } else {
            "Stroke width in px (default 1; 2 for line and arrow)"
        }),
        param("dash", ParamKind::List).help("Dash pattern in px, e.g. '4,2' (none = solid)"),
        d(param("dash-offset", ParamKind::Float), "0").advanced().help("Offset into the dash pattern in px"),
        d(param("cap", ParamKind::Choice).choices(&["butt", "round", "square"]), "butt")
            .help("Line cap at open path ends"),
        d(param("join", ParamKind::Choice).choices(&["miter", "round", "bevel"]), "miter").help("Line join at corners"),
        d(param("miter-limit", ParamKind::Float).range("1..1000"), "4")
            .advanced()
            .help("Miter limit (ratio of miter length to stroke width)"),
        d(param("fill-rule", ParamKind::Choice).choices(&["nonzero", "evenodd"]), "nonzero")
            .advanced()
            .help("Fill rule for self-overlapping paths and holes"),
        d(param("opacity", ParamKind::Float).range("0..100"), "100").help("Layer opacity 0-100"),
        param("name", ParamKind::Str).help(if set {
            "Rename the layer"
        } else {
            "Name for the new layer (default: generated, unique)"
        }),
    ]
}

pub fn ops() -> Vec<OpSpec> {
    vec![
        OpSpec::doc("shape rect", Category::Vector, "Add a rectangle shape layer (rounded via radius)", rect)
            .about("Adds a live vector rectangle layer above the active layer. `--radius` rounds every corner, or give four radii 'tl,tr,br,bl'. The layer stays editable: restyle it with `shape set`, combine it with `path unite`, export it with `svg from-doc`.")
            .param(param("rect", ParamKind::Rect).required().help("Rectangle X,Y,WxH in px or % of the canvas"))
            .param(param("radius", ParamKind::List).default("0").help("Corner radius in px, or 'tl,tr,br,bl' per corner"))
            .params(paint_params(false))
            .example("pixelox shape rect doc.pxo --in-place --rect 40,40,320x200 --fill '#0ea5e9'", "A filled rectangle")
            .example("pixelox shape rect card.pxo --in-place --rect 5%,5%,90%x90% --radius 24 --fill '#111827' --name Card", "A rounded card background")
            .example("pixelox shape rect photo.jpg -o framed.png --rect 10,10,300x200 --fill none --stroke '#ffffff' --stroke-width 8", "An outline frame on a photo")
            .example("pixelox shape rect doc.pxo --in-place --rect 0,0,200x100 --radius '24,24,0,0' --fill coral", "Round only the top corners"),
        OpSpec::doc("shape ellipse", Category::Vector, "Add an ellipse or circle shape layer", ellipse)
            .about("Adds a live vector ellipse inscribed in `--rect` (a square rect gives a circle).")
            .param(param("rect", ParamKind::Rect).required().help("Bounding box X,Y,WxH of the ellipse in px or %"))
            .params(paint_params(false))
            .example("pixelox shape ellipse doc.pxo --in-place --rect 100,100,200x200 --fill '#ef4444'", "A red circle")
            .example("pixelox shape ellipse photo.jpg -o ringed.png --rect 50,50,300x150 --stroke yellow --stroke-width 6 --dash '12,6'", "A dashed ellipse outline")
            .example("pixelox shape ellipse badge.pxo --in-place --rect 25%,25%,50%x50% --fill white --opacity 80 --name Halo", "A translucent centered disc"),
        OpSpec::doc("shape polygon", Category::Vector, "Add a regular polygon shape layer", polygon)
            .about("Adds a regular polygon with `--sides` vertices on a circle of `--radius` around `--center`. The first vertex points up; `--rotation` turns it clockwise.")
            .param(param("center", ParamKind::Point).default("50%,50%").help("Center X,Y in px or % of the canvas"))
            .param(param("radius", ParamKind::Length).required().help("Circumradius in px or % of the shorter canvas side"))
            .param(param("sides", ParamKind::Int).default("6").range("3..1000").help("Number of sides"))
            .param(param("rotation", ParamKind::Angle).default("0").help("Clockwise rotation in degrees"))
            .params(paint_params(false))
            .example("pixelox shape polygon doc.pxo --in-place --center 500,500 --radius 200 --sides 6 --fill '#22c55e'", "A green hexagon")
            .example("pixelox shape polygon doc.pxo --in-place --radius 120 --sides 3 --rotation 180 --fill none --stroke black --stroke-width 4", "A downward triangle outline at the center")
            .example("pixelox shape polygon logo.png -o octagon.png --center 256,256 --radius 100 --sides 8 --fill '#0000ff80'", "A translucent octagon"),
        OpSpec::doc("shape star", Category::Vector, "Add a star or burst shape layer", star)
            .about("Adds a star with `--points` tips on radius `--outer` and valleys on `--inner` (default half of outer). Many points with a close inner radius make a sticker burst.")
            .param(param("center", ParamKind::Point).default("50%,50%").help("Center X,Y in px or % of the canvas"))
            .param(param("points", ParamKind::Int).default("5").range("2..1000").help("Number of tips"))
            .param(param("outer", ParamKind::Length).required().help("Tip radius in px or % of the shorter canvas side"))
            .param(param("inner", ParamKind::Length).auto().default("auto").help("Valley radius in px or %; auto = half of --outer"))
            .param(param("rotation", ParamKind::Angle).default("0").help("Clockwise rotation in degrees"))
            .params(paint_params(false))
            .example(
                "pixelox shape star badge.pxo --in-place --center 500,500 --points 12 --outer 460 --inner 380 --fill '#ffd400' --stroke '#111111' --stroke-width 10 --name Burst",
                "A sticker burst",
            )
            .example("pixelox shape star doc.pxo --in-place --center 200,200 --points 5 --outer 100 --inner 40 --fill gold", "A gold five-point star")
            .example("pixelox shape star photo.jpg -o starred.png --center 80%,20% --points 8 --outer 60 --inner 30 --rotation 22.5 --fill white", "A small white star on a photo"),
        OpSpec::doc("shape line", Category::Vector, "Add a straight line shape layer", line)
            .about("Adds a stroked line from `--from` to `--to`. Without --stroke the line is black; --cap shapes its ends.")
            .param(param("from", ParamKind::Point).required().help("Start point X,Y in px or %"))
            .param(param("to", ParamKind::Point).required().help("End point X,Y in px or %"))
            .params(paint_params(false))
            .example("pixelox shape line doc.pxo --in-place --from 10,10 --to 300,10 --stroke black --stroke-width 4", "A black rule")
            .example("pixelox shape line doc.pxo --in-place --from 0,50% --to 100%,50% --stroke '#ff0000' --stroke-width 2 --dash '8,4' --cap round", "A dashed red midline")
            .example("pixelox shape line chart.png -o marked.png --from 100,400 --to 500,100 --stroke-width 3", "A 3 px trend line"),
        OpSpec::doc("shape arrow", Category::Vector, "Add an arrow shape layer", arrow)
            .about("Adds an arrow from `--from` to `--to`: a stroked shaft and a filled head of `--head-size` px (auto: 4x the stroke width, at least 8). `--tail-head` adds a second head at the start. One color paints both (--stroke or --fill, default black).")
            .param(param("from", ParamKind::Point).required().help("Tail point X,Y in px or %"))
            .param(param("to", ParamKind::Point).required().help("Tip point X,Y in px or %"))
            .param(param("head-size", ParamKind::Float).auto().default("auto").range("0..10000").help("Head length and width in px; auto = 4x stroke width (min 8)"))
            .param(param("tail-head", ParamKind::Flag).help("Also draw a head at the tail (double-headed arrow)"))
            .params(paint_params(false))
            .example("pixelox shape arrow doc.pxo --in-place --from 100,100 --to 400,300 --stroke-width 6 --head-size 30", "A thick black arrow")
            .example("pixelox shape arrow screenshot.png -o annotated.png --from 600,80 --to 420,240 --stroke '#e11d48' --stroke-width 8", "Point at something in a screenshot")
            .example("pixelox shape arrow diagram.pxo --in-place --from 50,200 --to 350,200 --tail-head --stroke black", "A double-headed dimension arrow"),
        OpSpec::doc("shape path", Category::Vector, "Add a shape layer from SVG path data", path)
            .about("Adds an arbitrary Bézier path layer. `--d` is SVG path data in canvas pixels (M, L, H, V, C, S, Q, T, A, Z; relative forms too).")
            .param(param("d", ParamKind::Str).required().help("SVG path data in canvas px, e.g. 'M 10 10 C 40 0 60 0 90 10'"))
            .params(paint_params(false))
            .example("pixelox shape path doc.pxo --in-place --d 'M 10 10 L 90 10 L 50 80 Z' --fill '#7c3aed'", "A purple triangle")
            .example("pixelox shape path doc.pxo --in-place --d 'M 20 200 C 100 20 300 20 380 200' --fill none --stroke black --stroke-width 5 --cap round", "A stroked curve")
            .example("pixelox shape path logo.png -o out.png --d 'M 0 0 H 100 V 100 H 0 Z M 25 25 H 75 V 75 H 25 Z' --fill-rule evenodd --fill teal", "A square ring (even-odd hole)"),
        OpSpec::doc("shape set", Category::Vector, "Restyle an existing shape layer", set)
            .about("Changes only the paint flags you pass (fill, stroke, width, dash, caps, joins, opacity, name); the geometry is kept. `--fill none` / `--stroke none` remove paint.")
            .param(common::layer())
            .params(paint_params(true))
            .example("pixelox shape set badge.pxo --in-place --layer Burst --fill '#ff5a00'", "Recolor a shape")
            .example("pixelox shape set doc.pxo --in-place --layer Rectangle --stroke white --stroke-width 4 --dash '6,3'", "Add a dashed white stroke")
            .example("pixelox shape set doc.pxo --in-place --layer @top --fill none --opacity 50 --name Outline", "Turn the top shape into a half-opacity outline"),
    ]
}

fn px_point(a: &Args, name: &str, doc: &Document) -> Result<(f64, f64)> {
    let p = a.point(name)?.ok_or_else(|| Error::new("PX1003", format!("missing --{name}")))?;
    Ok(p.resolve(doc.width as f64, doc.height as f64, doc.dpi()))
}

fn short_side(doc: &Document) -> f64 {
    doc.width.min(doc.height) as f64
}

/// Apply paint flags to a spec. `kind` is None for `shape set` (only given flags change).
fn apply_paint(s: &mut ShapeSpec, a: &Args, kind: Option<Paint>) -> Result<Vec<&'static str>> {
    let mut changed = Vec::new();
    let paint = |name: &str| -> Result<Option<Option<Rgba>>> {
        match a.raw(name) {
            None => Ok(None),
            Some(v) => parse_paint(v).map(Some).map_err(|mut e| {
                e.message = format!("--{name}: {}", e.message);
                e
            }),
        }
    };
    let fill = paint("fill")?;
    let stroke = paint("stroke")?;
    match kind {
        None => {
            if let Some(f) = fill {
                s.fill = f;
                changed.push("fill");
            }
            if let Some(st) = stroke {
                s.stroke = st;
                changed.push("stroke");
            }
        }
        Some(k) => {
            let black = Some(Rgba::BLACK);
            let (f, st) = match (k, fill, stroke) {
                (Paint::Arrow, None, None) => (black, black),
                (Paint::Arrow, Some(f), None) => (f, f),
                (Paint::Arrow, None, Some(st)) => (st, st),
                (Paint::Line, None, None) => (None, black),
                (Paint::Line, Some(f), None) => (None, f),
                (_, None, None) => (black, None),
                (_, f, st) => (f.flatten(), st.flatten()),
            };
            s.fill = f;
            s.stroke = st;
            s.stroke_width = if matches!(k, Paint::Line | Paint::Arrow) { 2.0 } else { 1.0 };
        }
    }
    if a.has("stroke-width") {
        s.stroke_width = a.f64("stroke-width")?;
        changed.push("stroke_width");
    }
    if let Some(d) = a.raw("dash") {
        s.dash = parse_dash(d)?;
        changed.push("dash");
    }
    if a.has("dash-offset") {
        s.dash_offset = a.f64("dash-offset")?;
        changed.push("dash_offset");
    }
    if a.has("cap") {
        s.cap = a.choice("cap");
        changed.push("cap");
    }
    if a.has("join") {
        s.join = a.choice("join");
        changed.push("join");
    }
    if a.has("miter-limit") {
        s.miter_limit = a.f64("miter-limit")?;
        changed.push("miter_limit");
    }
    if a.has("fill-rule") {
        s.fill_rule = a.choice("fill-rule");
        changed.push("fill_rule");
    }
    Ok(changed)
}

/// Create a shape layer above the active layer (or at the top) and describe it.
fn add_shape(
    a: &Args,
    doc: &mut Document,
    default_name: &str,
    bez: BezPath,
    geometry: Value,
    kind: Paint,
) -> Result<Value> {
    let d = spec::bez_to_d(&bez, PREC);
    add_shape_d(a, doc, default_name, d, geometry, kind)
}

fn add_shape_d(
    a: &Args,
    doc: &mut Document,
    default_name: &str,
    d: String,
    geometry: Value,
    kind: Paint,
) -> Result<Value> {
    let mut s = ShapeSpec::new(d, geometry);
    apply_paint(&mut s, a, Some(kind))?;
    let name = match a.str("name") {
        Some(n) => {
            doc.check_name_free(n.trim(), None)?;
            n.trim().to_string()
        }
        None => default_name.to_string(),
    };
    let mut layer = doc.new_layer(&name, LayerKind::Shape { spec: s.to_value() });
    layer.opacity = a.f64("opacity")? as f32;
    let (id, lname) = (layer.id.clone(), layer.name.clone());
    let above = doc.active.as_deref().and_then(|id| doc.find_by_id(id));
    doc.insert_above(above.as_deref(), layer);
    Ok(json!({
        "layer": lname,
        "id": id,
        "kind": "shape",
        "bbox": spec::spec_bbox(&s),
        "fill": s.fill.map(|c| c.to_hex()),
        "stroke": s.stroke.map(|c| c.to_hex()),
        "geometry": s.geometry,
    }))
}

fn rect_str(x: f64, y: f64, w: f64, h: f64) -> String {
    format!("{},{},{}x{}", fmt_num(x, PREC), fmt_num(y, PREC), fmt_num(w, PREC), fmt_num(h, PREC))
}

fn pt_str(p: (f64, f64)) -> String {
    format!("{},{}", fmt_num(p.0, PREC), fmt_num(p.1, PREC))
}

fn rect_arg(a: &Args, doc: &Document) -> Result<(f64, f64, f64, f64)> {
    let r = a.rect("rect")?.ok_or_else(|| Error::new("PX1003", "missing --rect"))?;
    let r = r.resolve(doc.width as f64, doc.height as f64, doc.dpi());
    if r.w <= 0.0 || r.h <= 0.0 {
        return Err(Error::new("PX1011", format!("--rect needs a positive width and height, got {}x{}", r.w, r.h)));
    }
    Ok((r.x, r.y, r.w, r.h))
}

fn radii(a: &Args) -> Result<[f64; 4]> {
    let vals = a.list("radius");
    let nums: Vec<f64> = vals
        .iter()
        .map(|v| v.trim().trim_end_matches("px").parse::<f64>().ok().filter(|x| x.is_finite() && *x >= 0.0))
        .collect::<Option<_>>()
        .ok_or_else(|| {
            Error::new("PX1010", format!("--radius: '{}' is not R or 'tl,tr,br,bl' (px, >= 0)", vals.join(",")))
        })?;
    Ok(match nums.as_slice() {
        [] => [0.0; 4],
        [r] => [*r; 4],
        [a, b] => [*a, *b, *a, *b],
        [tl, tr, br, bl] => [*tl, *tr, *br, *bl],
        _ => {
            return Err(Error::new("PX1010", "--radius takes 1, 2, or 4 values")
                .with_hint("e.g. --radius 12 or --radius '12,12,0,0'"))
        }
    })
}

fn rect(a: &Args, doc: &mut Document, _: &mut OpCtx) -> Result<Value> {
    let (x, y, w, h) = rect_arg(a, doc)?;
    let r = radii(a)?;
    let geom = json!({"type": "rect", "rect": rect_str(x, y, w, h), "radius": r});
    add_shape(a, doc, "Rectangle", spec::rect_path(x, y, w, h, r), geom, Paint::Closed)
}

fn ellipse(a: &Args, doc: &mut Document, _: &mut OpCtx) -> Result<Value> {
    let (x, y, w, h) = rect_arg(a, doc)?;
    let geom = json!({"type": "ellipse", "rect": rect_str(x, y, w, h)});
    add_shape(a, doc, "Ellipse", spec::ellipse_path(x, y, w, h), geom, Paint::Closed)
}

fn polygon(a: &Args, doc: &mut Document, _: &mut OpCtx) -> Result<Value> {
    let c = px_point(a, "center", doc)?;
    let r = a.px("radius", short_side(doc), doc.dpi())?;
    if r <= 0.0 {
        return Err(Error::new("PX1013", "--radius must be positive"));
    }
    let sides = a.usize("sides")?;
    let rot = a.angle("rotation")?;
    let geom =
        json!({"type": "polygon", "center": pt_str(c), "radius": spec::round(r, 3), "sides": sides, "rotation": rot});
    add_shape(a, doc, "Polygon", spec::polygon_path(c.0, c.1, r, sides, rot), geom, Paint::Closed)
}

fn star(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let c = px_point(a, "center", doc)?;
    let outer = a.px("outer", short_side(doc), doc.dpi())?;
    if outer <= 0.0 {
        return Err(Error::new("PX1013", "--outer must be positive"));
    }
    let inner = if a.is_auto("inner") {
        ctx.resolve_param("inner", spec::round(outer / 2.0, 3));
        outer / 2.0
    } else {
        a.px("inner", short_side(doc), doc.dpi())?
    };
    if inner < 0.0 {
        return Err(Error::new("PX1013", "--inner must not be negative"));
    }
    let points = a.usize("points")?;
    let rot = a.angle("rotation")?;
    let geom = json!({"type": "star", "center": pt_str(c), "points": points, "outer": spec::round(outer, 3), "inner": spec::round(inner, 3), "rotation": rot});
    add_shape(a, doc, "Star", spec::star_path(c.0, c.1, points, outer, inner, rot), geom, Paint::Closed)
}

fn line(a: &Args, doc: &mut Document, _: &mut OpCtx) -> Result<Value> {
    let p0 = px_point(a, "from", doc)?;
    let p1 = px_point(a, "to", doc)?;
    let geom = json!({"type": "line", "from": pt_str(p0), "to": pt_str(p1)});
    add_shape(a, doc, "Line", spec::line_path(p0, p1), geom, Paint::Line)
}

fn arrow(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let p0 = px_point(a, "from", doc)?;
    let p1 = px_point(a, "to", doc)?;
    if (p0.0 - p1.0).hypot(p0.1 - p1.1) < 1e-9 {
        return Err(Error::new("PX1010", "--from and --to are the same point"));
    }
    let width = if a.has("stroke-width") { a.f64("stroke-width")? } else { 2.0 };
    let head = match a.opt_f64("head-size")? {
        Some(h) => h,
        None => {
            let h = (4.0 * width).max(8.0);
            ctx.resolve_param("head-size", spec::round(h, 3));
            h
        }
    };
    let tail = a.flag("tail-head");
    let geom = json!({"type": "arrow", "from": pt_str(p0), "to": pt_str(p1), "head_size": spec::round(head, 3), "tail_head": tail});
    add_shape(a, doc, "Arrow", spec::arrow_path(p0, p1, head, tail), geom, Paint::Arrow)
}

fn path(a: &Args, doc: &mut Document, _: &mut OpCtx) -> Result<Value> {
    let d = a.str("d").unwrap_or_default().trim().to_string();
    let bez = spec::parse_d(&d)?;
    if bez.elements().is_empty() {
        return Err(
            Error::new("PX1010", "--d is empty").with_hint("pass SVG path data, e.g. --d 'M 10 10 L 90 10 L 50 80 Z'")
        );
    }
    add_shape_d(a, doc, "Path", d, json!({"type": "path"}), Paint::Closed)
}

/// Resolve `--layer` to an unlocked shape layer; PX2303 for other kinds.
pub fn shape_layer(doc: &Document, r: Option<&str>) -> Result<(pixelox_core::doc::LayerPath, ShapeSpec)> {
    let p = doc.resolve_or_only(r)?;
    let l = doc.layer(&p);
    let LayerKind::Shape { spec } = &l.content else {
        return Err(Error::new(
            "PX2303",
            format!("layer '{}' is a {} layer; this operation needs a shape layer", l.name, l.kind_name()),
        )
        .with_hint(
            "list layers with `pixelox layer list <doc>`; create shapes with `pixelox shape rect|ellipse|path ...`",
        )
        .with_context("layer", &l.name)
        .with_context("kind", l.kind_name()));
    };
    Ok((p.clone(), ShapeSpec::from_value(spec)?))
}

fn set(a: &Args, doc: &mut Document, _: &mut OpCtx) -> Result<Value> {
    let (p, mut s) = shape_layer(doc, a.raw("layer"))?;
    doc.layer(&p).check_unlocked()?;
    let mut changed = apply_paint(&mut s, a, None)?;
    if a.has("opacity") {
        changed.push("opacity");
    }
    let new_name = a.str("name").map(|n| n.trim().to_string());
    if let Some(n) = &new_name {
        doc.check_name_free(n, Some(&doc.layer(&p).id))?;
        changed.push("name");
    }
    if changed.is_empty() {
        return Err(Error::new("PX1003", "'shape set' needs at least one paint flag to change")
            .with_hint("e.g. --fill '#ff5a00', --stroke none, --stroke-width 4, --opacity 50"));
    }
    let opacity = if a.has("opacity") { Some(a.f64("opacity")? as f32) } else { None };
    let l = doc.layer_mut(&p);
    if let LayerKind::Shape { spec } = &mut l.content {
        s.write_into(spec);
    }
    if let Some(o) = opacity {
        l.opacity = o;
    }
    if let Some(n) = new_name {
        l.name = n;
    }
    Ok(json!({
        "layer": l.name,
        "id": l.id,
        "changed": changed,
        "fill": s.fill.map(|c| c.to_hex()),
        "stroke": s.stroke.map(|c| c.to_hex()),
        "stroke_width": s.stroke_width,
        "bbox": spec::spec_bbox(&s),
    }))
}
