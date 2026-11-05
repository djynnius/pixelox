//! `transform *`: layer geometry (move, scale, rotate, flip, skew, perspective, warp) and
//! document geometry (rotate/flip without --layer, crop, straighten, canvas, resize, trim).
//!
//! Document-scoped commands transform every layer consistently (locked layers too, so the
//! stack stays registered), plus the selection, library masks, layer masks, vector masks,
//! and guides. Layer-scoped commands refuse locked layers (PX2304).

use crate::engine::{self, Geo};
use crate::straighten::{detect_tilt, max_inscribed};
use crate::util::{anchor, anchor_param, at_most_one, content_bounds, interp, interp_param, r3, r4, Affine2};
use crate::warp;
use pixelox_core::doc::LayerPath;
use pixelox_core::kernels::Filter;
use pixelox_core::op::ParamSpec;
use pixelox_core::prelude::*;
use pixelox_core::units::Length;
use rayon::prelude::*;

const CAT: Category = Category::Transform;

fn layer_param(help: &'static str) -> ParamSpec {
    param("layer", ParamKind::Layer).help(help)
}

const SCOPED: &str = "Layer to transform: name, #N (1 = top), @top, @bottom, @active (default: the only layer)";

pub fn ops() -> Vec<OpSpec> {
    vec![
        OpSpec::doc("transform move", CAT, "Move a layer", mv)
            .about("Move a layer and its mask: --by shifts it; --to puts the --anchor point of its visible bounds at X,Y (negative values count from the right/bottom). Every layer kind stays live (text anchor, shape path, placed rect, group children).")
            .param(layer_param(SCOPED))
            .param(param("to", ParamKind::Point).help("Destination X,Y for the anchor point of the layer's bounds"))
            .param(param("by", ParamKind::Point).help("Offset DX,DY in px (or % of the canvas)"))
            .param(anchor_param("tl", "Which point of the bounds lands on --to: tl t tr l c r bl b br"))
            .example("pixelox transform move doc.pxo --layer Logo --by 20,-10 --in-place", "Nudge a layer")
            .example("pixelox transform move doc.pxo --layer Logo --to 50%,50% --anchor c --in-place", "Center a layer")
            .example("pixelox transform move doc.pxo --layer Badge --to 100%,100% --anchor br --in-place", "Pin to the bottom-right corner"),
        OpSpec::doc("transform scale", CAT, "Scale a layer", scale)
            .about("Scale a layer about the --anchor point of its visible bounds: --to by a factor (50% or 0.5), or --size to an exact bounds size (one side blank keeps the aspect; % is of the current bounds). Text stays live under uniform scales (size scales); shapes scale their path and stroke; non-uniform text is rasterized with a warning.")
            .param(layer_param(SCOPED))
            .param(param("to", ParamKind::Str).help("Uniform factor: 50% or 0.5"))
            .param(param("size", ParamKind::Size).help("New bounds size WxH (2048x keeps aspect)"))
            .param(anchor_param("c", "Fixed point of the scale on the layer's bounds: tl t tr l c r bl b br"))
            .param(interp_param())
            .example("pixelox transform scale doc.pxo --layer Logo --to 50% --in-place", "Halve a layer")
            .example("pixelox transform scale doc.pxo --layer Photo --size 800x --anchor tl --in-place", "Scale to 800 px wide")
            .example("pixelox transform scale doc.pxo --layer Icon --to 3 --interp nearest --in-place", "Pixel-art upscale"),
        OpSpec::doc("transform rotate", CAT, "Rotate a layer or the whole canvas", rotate)
            .about("Rotate clockwise by --degrees. With --layer: that layer about its bounds center (or --around). Without: the whole document; multiples of 90° are lossless and swap the canvas for 90/270; other angles keep the canvas size unless --expand grows it to fit. Text and placed layers stay live (rotation accumulates).")
            .param(layer_param("Layer to rotate (omit to rotate the whole document)"))
            .param(param("degrees", ParamKind::Angle).required().help("Clockwise angle in degrees (negative = counter-clockwise)"))
            .param(param("expand", ParamKind::Flag).help("Grow the canvas to fit the rotated document (document rotation only)"))
            .param(param("around", ParamKind::Point).help("Center of rotation X,Y (default: the layer's bounds center or the canvas center)"))
            .param(interp_param())
            .example("pixelox transform rotate photo.jpg -o rotated.jpg --degrees 90", "Lossless quarter turn")
            .example("pixelox transform rotate doc.pxo --degrees 12 --expand --in-place", "Tilt the canvas, growing it")
            .example("pixelox transform rotate doc.pxo --layer Badge --degrees 15 --in-place", "Rotate one layer"),
        OpSpec::doc("transform flip", CAT, "Flip a layer or the whole canvas", flip)
            .about("Mirror horizontally (--h) and/or vertically (--v), losslessly: a layer about its bounds center, or the whole document. Shapes stay live; text and placed layers cannot be mirrored live and are rasterized with a warning.")
            .param(layer_param("Layer to flip (omit to flip the whole document)"))
            .param(param("h", ParamKind::Flag).help("Mirror left-right"))
            .param(param("v", ParamKind::Flag).help("Mirror top-bottom"))
            .example("pixelox transform flip photo.jpg -o mirrored.jpg --h", "Mirror a photo")
            .example("pixelox transform flip doc.pxo --layer Arrow --h --in-place", "Point an arrow the other way")
            .example("pixelox transform flip doc.pxo --v --in-place", "Flip the document upside down"),
        OpSpec::doc("transform skew", CAT, "Skew (shear) a layer", skew)
            .about("Shear a layer about its bounds center: --x DEG slants vertical edges (positive moves the bottom right), --y DEG slants horizontal edges (positive moves the right side down). Text, shape, and placed layers are rasterized first with a warning.")
            .param(layer_param(SCOPED))
            .param(param("x", ParamKind::Angle).default("0").help("Horizontal shear angle in degrees (|x| < 89)"))
            .param(param("y", ParamKind::Angle).default("0").help("Vertical shear angle in degrees (|y| < 89)"))
            .param(interp_param())
            .example("pixelox transform skew doc.pxo --layer Banner --x 15 --in-place", "Italic-style slant")
            .example("pixelox transform skew doc.pxo --layer Card --x 0 --y 8 --in-place", "Slant the horizontal edges")
            .example("pixelox transform skew art.png -o skewed.png --x 10 --y 5", "Skew a flat image"),
        OpSpec::doc("transform perspective", CAT, "Map a layer's corners to a quad", perspective)
            .about("Projective transform: the corners of the layer's visible bounds (top-left, top-right, bottom-right, bottom-left) go to the four --quad points in canvas pixels. Text, shape, and placed layers are rasterized first with a warning; a layer mask is warped along, a vector mask is dropped (the raster mask keeps its shape).")
            .param(layer_param(SCOPED))
            .param(param("quad", ParamKind::Points).required().help("Four corners 'x1,y1 x2,y2 x3,y3 x4,y4': top-left, top-right, bottom-right, bottom-left"))
            .param(interp_param())
            .example("pixelox transform perspective doc.pxo --layer Screen --quad '120,80 620,40 640,420 100,380' --in-place", "Put a screenshot on a tilted screen")
            .example("pixelox transform perspective poster.png -o keystone.png --quad '50,0 750,0 800,600 0,600'", "Keystone a flat image")
            .example("pixelox transform perspective doc.pxo --layer Label --quad '0,0 200,20 200,120 0,100' --interp bilinear --in-place", "Faster sampling"),
        OpSpec::doc("transform warp", CAT, "Grid-mesh warp of a layer", warp_op)
            .about("Bend a layer with a grid of control points spread evenly over its visible bounds; the points file gives where each node goes. JSON schema: {\"grid\": \"3x3\" (optional, else --grid), \"relative\": false, \"points\": [[x, y], ...]} or a bare array of points; cols x rows points in row-major order (top row first, left to right), canvas pixels, or offsets from each node's rest position when \"relative\" is true ({\"x\", \"y\"} objects also work). Displacements are interpolated smoothly (Catmull-Rom). Text, shape, and placed layers are rasterized first with a warning.")
            .param(layer_param(SCOPED))
            .param(param("grid", ParamKind::Str).default("3x3").help("Control grid COLSxROWS, each 2..16"))
            .param(param("points", ParamKind::Path).required().help("JSON file of destination points (schema in the description)"))
            .param(interp_param())
            .example("pixelox transform warp doc.pxo --layer Flag --grid 3x3 --points warp.json --in-place", "Bend a layer")
            .example("pixelox transform warp doc.pxo --layer Label --grid 4x2 --points bulge.json --in-place", "Wrap a label")
            .example("pixelox transform warp sticker.png -o bent.png --points warp.json", "Warp a flat image"),
        OpSpec::doc("transform crop", CAT, "Crop the document", crop)
            .about("Crop the canvas to --rect, to the trimmed content (--auto trim), or to the largest --ratio W:H rectangle placed by --gravity. Every layer, mask, and guide shifts with it; raster pixels outside the new canvas are deleted; live layers stay live. --content-aware (seam carving) is a stretch item and not in this build (PX1520).")
            .param(param("rect", ParamKind::Rect).help("Keep this rectangle X,Y,WxH (clipped to the canvas)"))
            .param(param("auto", ParamKind::Choice).choices(&["trim"]).help("trim: remove a uniform or transparent border (like `transform trim`)"))
            .param(param("ratio", ParamKind::Str).help("Aspect ratio W:H (4:5, 16:9, or 1.5): the largest such rectangle"))
            .param(param("gravity", ParamKind::Choice).choices(units::ANCHORS).default("c").help("Where a --ratio crop sits: tl t tr l c r bl b br"))
            .param(param("content-aware", ParamKind::Flag).help("Seam-carving retarget (stretch feature; not in this build)"))
            .example("pixelox transform crop photo.jpg -o crop.jpg --rect 100,50,800x600", "Crop to a box")
            .example("pixelox transform crop photo.jpg -o portrait.jpg --ratio 4:5 --gravity t", "Instagram portrait from the top")
            .example("pixelox transform crop scan.png -o tight.png --auto trim", "Trim the border"),
        OpSpec::doc("transform straighten", CAT, "Level a tilted horizon or edges", straighten)
            .about("Rotate the document to level it. --angle auto detects the dominant near-horizontal/vertical edge tilt (Sobel gradients + a projection-profile search within ±15°) and records the correction it applied, so replay is exact; a number is the clockwise rotation to apply. --crop auto then crops to the largest axis-aligned rectangle inside the rotated content.")
            .param(param("angle", ParamKind::Angle).auto().default("auto").help("auto (detect) or the clockwise rotation in degrees to apply"))
            .param(param("crop", ParamKind::Choice).choices(&["auto", "none"]).default("auto").help("auto: crop away the empty corners; none: keep the canvas size"))
            .param(interp_param())
            .example("pixelox transform straighten horizon.jpg -o level.jpg", "Auto-level a photo")
            .example("pixelox transform straighten scan.pxo --angle 1.5 --crop none --in-place", "Apply a known correction")
            .example("pixelox transform straighten doc.pxo --angle auto --dry-run --json", "Detect the tilt only"),
        OpSpec::doc("transform canvas", CAT, "Resize the canvas without resampling", canvas)
            .about("Change the canvas size: absolute WxH (px or % of the current size) or relative '+200x+0' / '-100x' (a blank side keeps it). The old canvas is placed by --anchor; layers are not resampled or clipped. A --fill color fills the new area on the bottom raster layer (or a new 'Canvas Fill' layer).")
            .param(param("size", ParamKind::Str).required().help("New size WxH, or relative +DWx+DH (blank side = unchanged)"))
            .param(anchor_param("c", "Where the old canvas sits in the new one: tl t tr l c r bl b br"))
            .param(param("fill", ParamKind::Color).default("transparent").help("Color for the added area, or transparent"))
            .example("pixelox transform canvas photo.jpg -o padded.jpg --size '+200x+200' --fill white", "Add a 100 px white border")
            .example("pixelox transform canvas doc.pxo --size 1080x1350 --anchor t --in-place", "Extend to a portrait canvas")
            .example("pixelox transform canvas doc.pxo --size '+0x+120' --anchor t --fill '#111111' --in-place", "Add a caption strip"),
        OpSpec::doc("transform resize", CAT, "Resample the document to a new size", resize)
            .about("Resample every layer, mask, and guide to --size (a blank side keeps the aspect; 50% scales both). Lanczos-3 when shrinking and bicubic when enlarging unless --interp says otherwise. Text stays live (its size scales) when the aspect is kept. --sharpen auto adds a light unsharp mask scaled to the reduction.")
            .param(param("size", ParamKind::Size).required().help("New canvas size WxH (2048x keeps aspect, 50% halves)"))
            .param(interp_param())
            .param(param("sharpen", ParamKind::Float).auto().default("0").range("0..2").help("Unsharp amount after resampling: auto (scaled to the reduction) or 0..2; 0 = none"))
            .example("pixelox transform resize photo.jpg -o small.jpg --size 1024x", "Fit to 1024 px wide")
            .example("pixelox transform resize photo.jpg -o thumb.jpg --size 25% --sharpen auto", "Quarter-size thumbnail")
            .example("pixelox transform resize sprite.png -o big.png --size 400% --interp nearest", "Crisp pixel-art upscale"),
        OpSpec::doc("transform trim", CAT, "Crop away a uniform or transparent border", trim)
            .about("Crop the document to its content: if a corner of the composite is transparent, transparent pixels (alpha <= threshold) are trimmed; otherwise pixels within --threshold (0..255, max channel difference) of the top-left color.")
            .param(param("threshold", ParamKind::Int).default("8").range("0..255").help("Tolerance 0..255 for 'same as the border'"))
            .example("pixelox transform trim logo.png -o tight.png", "Trim transparent margins")
            .example("pixelox transform trim scan.jpg -o trimmed.jpg --threshold 24", "Trim a noisy white border")
            .example("pixelox transform trim doc.pxo --in-place --threshold 0", "Exact-match trim"),
    ]
}

fn canvas_size(doc: &Document) -> (f64, f64) {
    (doc.width as f64, doc.height as f64)
}

/// Visible bounds of the layer (fx excluded), PX3320 when it has none.
fn bounds_of(doc: &Document, p: &[usize]) -> Result<RectI> {
    let l = doc.layer(p);
    content_bounds(doc, l)?.ok_or_else(|| {
        Error::new("PX3320", format!("layer '{}' has no visible content to transform", l.name))
            .with_context("layer", &l.name)
    })
}

fn center_of(doc: &Document, p: &[usize]) -> Result<(f64, f64)> {
    Ok(match content_bounds(doc, doc.layer(p))? {
        Some(b) => (b.x as f64 + b.w as f64 / 2.0, b.y as f64 + b.h as f64 / 2.0),
        None => (doc.width as f64 / 2.0, doc.height as f64 / 2.0),
    })
}

fn layer_result(doc: &Document, p: &[usize], rasterized: Vec<String>) -> Result<Value> {
    let l = doc.layer(p);
    Ok(json!({
        "layer": l.name,
        "bounds": content_bounds(doc, l).ok().flatten().map(|b| b.to_json()),
        "rasterized": rasterized,
    }))
}

fn doc_result(doc: &Document, rasterized: Vec<String>) -> Value {
    json!({"size": [doc.width, doc.height], "rasterized": rasterized})
}

fn resolve_layer_arg(a: &Args, doc: &Document) -> Result<LayerPath> {
    let p = doc.resolve_or_only(a.raw("layer"))?;
    doc.layer(&p).check_unlocked()?;
    Ok(p)
}

fn mv(a: &Args, doc: &mut Document, _: &mut OpCtx) -> Result<Value> {
    let p = resolve_layer_arg(a, doc)?;
    at_most_one(a, &["to", "by"], &[])?;
    let (cw, ch) = canvas_size(doc);
    let dpi = doc.dpi();
    let (dx, dy) = if let Some(to) = a.point("to")? {
        let (tx, ty) = to.resolve(cw, ch, dpi);
        let b = bounds_of(doc, &p)?;
        let (fx, fy) = anchor(a)?.fractions();
        (tx - (b.x as f64 + fx * b.w as f64), ty - (b.y as f64 + fy * b.h as f64))
    } else if let Some(by) = a.point("by")? {
        (by.x.to_px(cw, dpi), by.y.to_px(ch, dpi))
    } else {
        return Err(Error::new("PX1003", "'transform move' needs --to X,Y or --by DX,DY").with_hint("--by 10,0"));
    };
    let raster = matches!(doc.layer(&p).content, LayerKind::Raster { .. });
    let (dx, dy) = if raster { (dx.round(), dy.round()) } else { (r4(dx), r4(dy)) };
    geom::translate_layer(doc, &p, dx, dy)?;
    let mut v = layer_result(doc, &p, vec![])?;
    v["by"] = json!([dx, dy]);
    Ok(v)
}

fn parse_factor(s: &str) -> Result<f64> {
    let t = s.trim();
    let v =
        if let Some(p) = t.strip_suffix('%') { p.trim().parse::<f64>().map(|v| v / 100.0) } else { t.parse::<f64>() };
    match v {
        Ok(v) if v > 0.0 && v.is_finite() => Ok(v),
        Ok(v) => Err(Error::new("PX1013", format!("--to must be positive, got {v}"))),
        Err(_) => Err(Error::new("PX1010", format!("--to '{s}' is not a factor (use 50% or 0.5)"))),
    }
}

fn scale(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let p = resolve_layer_arg(a, doc)?;
    at_most_one(a, &["to", "size"], &[])?;
    let b = bounds_of(doc, &p)?;
    let (bw, bh) = (b.w as f64, b.h as f64);
    let (sx, sy) = if let Some(t) = a.str("to") {
        let k = parse_factor(&t)?;
        (k, k)
    } else if let Some(s) = a.size("size")? {
        let (w, h) = s.resolve_f(bw, bh, doc.dpi());
        (w / bw, h / bh)
    } else {
        return Err(Error::new("PX1003", "'transform scale' needs --to 50% or --size WxH"));
    };
    if !(sx > 0.0 && sy > 0.0 && sx.is_finite() && sy.is_finite()) {
        return Err(Error::new("PX1013", "the scale must be positive").with_context("scale", [sx, sy]));
    }
    let (fx, fy) = anchor(a)?.fractions();
    let m = Affine2::scale(sx, sy).about(b.x as f64 + fx * bw, b.y as f64 + fy * bh);
    let r = engine::layer_geo(doc, ctx, &p, &Geo::Affine(m), interp(a)?, false)?;
    let mut v = layer_result(doc, &p, r)?;
    v["scale"] = json!([r4(sx), r4(sy)]);
    Ok(v)
}

/// A clockwise quarter-turn map of a `w`×`h` canvas and the new size.
fn quarter_turn(k: i64, w: f64, h: f64) -> (Affine2, f64, f64) {
    match k.rem_euclid(4) {
        0 => (Affine2::IDENTITY, w, h),
        1 => (Affine2::new(0.0, 1.0, -1.0, 0.0, h, 0.0), h, w),
        2 => (Affine2::new(-1.0, 0.0, 0.0, -1.0, w, h), w, h),
        _ => (Affine2::new(0.0, -1.0, 1.0, 0.0, 0.0, w), h, w),
    }
}

/// Quarter turns about a half-pixel center would land between pixels; shift by at most
/// half a pixel so they stay exact permutations.
fn snap_quarter(mut m: Affine2) -> Affine2 {
    let unit = |v: f64| v == 0.0 || v.abs() == 1.0;
    if unit(m.a) && unit(m.b) && unit(m.c) && unit(m.d) {
        m.e = m.e.round();
        m.f = m.f.round();
    }
    m
}

/// Rotate the whole document clockwise; `expand` grows the canvas to fit.
fn rotate_doc(
    doc: &mut Document,
    ctx: &mut OpCtx,
    deg: f64,
    around: Option<(f64, f64)>,
    expand: bool,
    filter: Option<Filter>,
) -> Result<Vec<String>> {
    let (w, h) = canvas_size(doc);
    let q = deg / 90.0;
    let (m, nw, nh) = if (q - q.round()).abs() < 1e-9 && around.is_none() {
        quarter_turn(q.round() as i64, w, h)
    } else {
        let (cx, cy) = around.unwrap_or((w / 2.0, h / 2.0));
        let m0 = snap_quarter(Affine2::rotate(deg).about(cx, cy));
        if expand {
            let (x0, y0, x1, y1) = m0.map_rect(0.0, 0.0, w, h);
            let (bw, bh) = (x1 - x0, y1 - y0);
            let (nw, nh) = ((bw - 1e-6).ceil().max(1.0), (bh - 1e-6).ceil().max(1.0));
            (m0.then(Affine2::translate(-x0 + (nw - bw) / 2.0, -y0 + (nh - bh) / 2.0)), nw, nh)
        } else {
            (m0, w, h)
        }
    };
    let (nw, nh) = crate::sample::check_size(nw, nh)?;
    engine::doc_affine(doc, ctx, &m, nw, nh, filter)
}

fn rotate(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let deg = a.angle("degrees")?;
    let filter = interp(a)?;
    let (cw, ch) = canvas_size(doc);
    let around = a.point("around")?.map(|p| p.resolve(cw, ch, doc.dpi()));
    if a.raw("layer").is_some() {
        let p = resolve_layer_arg(a, doc)?;
        let (cx, cy) = match around {
            Some(c) => c,
            None => center_of(doc, &p)?,
        };
        let m = snap_quarter(Affine2::rotate(deg).about(cx, cy));
        let r = engine::layer_geo(doc, ctx, &p, &Geo::Affine(m), filter, false)?;
        let mut v = layer_result(doc, &p, r)?;
        v["degrees"] = json!(deg);
        return Ok(v);
    }
    let r = rotate_doc(doc, ctx, deg, around, a.flag("expand"), filter)?;
    let mut v = doc_result(doc, r);
    v["degrees"] = json!(deg);
    Ok(v)
}

fn flip(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let (h, v) = (a.flag("h"), a.flag("v"));
    if !h && !v {
        return Err(
            Error::new("PX1003", "'transform flip' needs --h, --v, or both").with_hint("--h mirrors left-right")
        );
    }
    let (sx, sy) = (if h { -1.0 } else { 1.0 }, if v { -1.0 } else { 1.0 });
    let mut out = if a.raw("layer").is_some() {
        let p = resolve_layer_arg(a, doc)?;
        let (cx, cy) = center_of(doc, &p)?;
        let m = Affine2::scale(sx, sy).about(cx, cy);
        let r = engine::layer_geo(doc, ctx, &p, &Geo::Affine(m), Some(Filter::Nearest), false)?;
        layer_result(doc, &p, r)?
    } else {
        let (w, hh) = canvas_size(doc);
        let m = Affine2::scale(sx, sy).about(w / 2.0, hh / 2.0);
        let r = engine::doc_affine(doc, ctx, &m, doc.width, doc.height, Some(Filter::Nearest))?;
        doc_result(doc, r)
    };
    out["h"] = json!(h);
    out["v"] = json!(v);
    Ok(out)
}

fn skew(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let (ax, ay) = (a.angle("x")?, a.angle("y")?);
    for (n, v) in [("x", ax), ("y", ay)] {
        if v.abs() >= 89.0 {
            return Err(Error::new("PX1013", format!("--{n} {v} is too steep (|angle| < 89)")).with_context("param", n));
        }
    }
    let p = resolve_layer_arg(a, doc)?;
    let (cx, cy) = center_of(doc, &p)?;
    let m = Affine2::new(1.0, ay.to_radians().tan(), ax.to_radians().tan(), 1.0, 0.0, 0.0).about(cx, cy);
    let r = engine::layer_geo(doc, ctx, &p, &Geo::Affine(m), interp(a)?, true)?;
    let mut v = layer_result(doc, &p, r)?;
    v["skew"] = json!([ax, ay]);
    Ok(v)
}

fn perspective(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let pts = a.points("quad")?;
    if pts.len() != 4 {
        return Err(Error::new("PX1010", format!("--quad needs exactly 4 points, got {}", pts.len()))
            .with_hint("--quad 'x1,y1 x2,y2 x3,y3 x4,y4' (top-left, top-right, bottom-right, bottom-left)"));
    }
    let p = resolve_layer_arg(a, doc)?;
    let b = bounds_of(doc, &p)?;
    let quad = [pts[0], pts[1], pts[2], pts[3]];
    let field = warp::perspective_field((b.x as f64, b.y as f64, b.w as f64, b.h as f64), &quad)?;
    let r = engine::layer_geo(doc, ctx, &p, &Geo::Field(&field), interp(a)?, true)?;
    layer_result(doc, &p, r)
}

fn parse_grid(s: &str) -> Result<(usize, usize)> {
    let bad = || Error::new("PX1010", format!("--grid '{s}' is not COLSxROWS with 2..16 each (e.g. 3x3)"));
    let (c, r) = s.trim().split_once(['x', 'X']).ok_or_else(bad)?;
    let (c, r): (usize, usize) = (c.trim().parse().map_err(|_| bad())?, r.trim().parse().map_err(|_| bad())?);
    if !(2..=16).contains(&c) || !(2..=16).contains(&r) {
        return Err(bad());
    }
    Ok((c, r))
}

/// Points, whether they are relative, and the grid named in the file (if any).
fn parse_warp_file(v: &Value) -> Result<(Vec<(f64, f64)>, bool, Option<String>)> {
    let bad = || {
        Error::new("PX1010", "the warp points file does not match the schema")
            .with_hint("{\"grid\": \"3x3\", \"points\": [[x, y], ...]} with cols*rows points, row-major (see `pixelox docs transform.warp`)")
    };
    let (list, relative, grid) = match v {
        Value::Array(a) => (a, false, None),
        Value::Object(o) => (
            o.get("points").and_then(|p| p.as_array()).ok_or_else(bad)?,
            o.get("relative").and_then(|r| r.as_bool()).unwrap_or(false),
            o.get("grid").map(|g| match g {
                Value::String(s) => s.clone(),
                Value::Array(a) if a.len() == 2 => format!("{}x{}", a[0], a[1]),
                other => other.to_string(),
            }),
        ),
        _ => return Err(bad()),
    };
    let pts = list
        .iter()
        .map(|p| match p {
            Value::Array(xy) if xy.len() == 2 => Some((xy[0].as_f64()?, xy[1].as_f64()?)),
            Value::Object(o) => Some((o.get("x")?.as_f64()?, o.get("y")?.as_f64()?)),
            _ => None,
        })
        .collect::<Option<Vec<_>>>()
        .ok_or_else(bad)?;
    Ok((pts, relative, grid))
}

fn warp_op(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let p = resolve_layer_arg(a, doc)?;
    if a.is_explicit("grid") {
        parse_grid(&a.str_or("grid", "3x3"))?;
    }
    let file = a.path("points").ok_or_else(|| Error::new("PX1003", "'transform warp' needs --points FILE"))?;
    let bytes = ctx.read_aux(&file, "points", Some("points"))?;
    let v: Value = serde_json::from_slice(&bytes)?;
    let (pts, relative, file_grid) = parse_warp_file(&v)?;
    let grid = match (file_grid, a.is_explicit("grid")) {
        (Some(g), false) => g,
        (Some(g), true) if parse_grid(&g)? != parse_grid(&a.str_or("grid", "3x3"))? => {
            return Err(Error::new(
                "PX1005",
                format!("--grid {} disagrees with the points file's grid {g}", a.str_or("grid", "3x3")),
            ))
        }
        _ => a.str_or("grid", "3x3"),
    };
    let (cols, rows) = parse_grid(&grid)?;
    if pts.len() != cols * rows {
        return Err(Error::new(
            "PX1010",
            format!("a {cols}x{rows} grid needs {} points, the file has {}", cols * rows, pts.len()),
        )
        .with_context("expected", cols * rows)
        .with_context("got", pts.len()));
    }
    let b = bounds_of(doc, &p)?;
    let (bx, by, bw, bh) = (b.x as f64, b.y as f64, b.w as f64, b.h as f64);
    let dst: Vec<(f64, f64)> = if relative {
        pts.iter()
            .enumerate()
            .map(|(i, &(dx, dy))| {
                let (c, r) = (i % cols, i / cols);
                (bx + bw * c as f64 / (cols - 1) as f64 + dx, by + bh * r as f64 / (rows - 1) as f64 + dy)
            })
            .collect()
    } else {
        pts
    };
    let field = warp::mesh_field((bx, by, bw, bh), cols, rows, &dst)?;
    let r = engine::layer_geo(doc, ctx, &p, &Geo::Field(&field), interp(a)?, true)?;
    let mut v = layer_result(doc, &p, r)?;
    v["grid"] = json!(format!("{cols}x{rows}"));
    Ok(v)
}

/// The content box of `img`: transparent border when a corner is transparent, else a
/// border within `thr` of the top-left color. `None` when nothing differs.
pub fn trim_rect(img: &Image, thr: i64) -> Option<RectI> {
    if img.is_empty() {
        return None;
    }
    let t = thr as f32 / 255.0;
    let (w, h) = (img.width, img.height);
    let corners = [img.get(0, 0), img.get(w - 1, 0), img.get(0, h - 1), img.get(w - 1, h - 1)];
    let transparent = corners.iter().any(|p| p[3] <= t.max(0.5 / 255.0));
    let r = img.get(0, 0);
    let pm = |p: &[f32]| [p[0] * p[3], p[1] * p[3], p[2] * p[3], p[3]];
    let rp = pm(&r);
    let mut keep = Mask::new(w, h);
    keep.data.par_iter_mut().zip(img.data.par_chunks(4)).for_each(|(k, p)| {
        let differs = if transparent {
            p[3] > t.max(0.5 / 255.0)
        } else {
            let q = pm(p);
            (0..4).any(|c| (q[c] - rp[c]).abs() > t + 1e-6)
        };
        *k = if differs { 1.0 } else { 0.0 };
    });
    keep.bbox(0.5)
}

fn parse_ratio(s: &str) -> Result<f64> {
    let t = s.trim();
    let r = match t.split_once([':', '/']) {
        Some((a, b)) => match (a.trim().parse::<f64>(), b.trim().parse::<f64>()) {
            (Ok(a), Ok(b)) if b != 0.0 => Some(a / b),
            _ => None,
        },
        None => t.parse::<f64>().ok(),
    };
    r.filter(|v| *v > 0.0 && v.is_finite())
        .ok_or_else(|| Error::new("PX1010", format!("--ratio '{s}' is not W:H (e.g. 4:5) or a positive number")))
}

fn crop(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    if a.flag("content-aware") {
        return Err(Error::new(
            "PX1520",
            "content-aware crop (seam-carving retarget) is a stretch feature and is not in this build",
        )
        .with_hint("crop with --rect, --ratio W:H, or --auto trim; or pad with `pixelox transform canvas`"));
    }
    let g = at_most_one(a, &["rect", "auto", "ratio"], &[])?;
    let canvas = doc.canvas();
    let (cw, ch) = canvas_size(doc);
    let r = if let Some(rs) = a.rect("rect")? {
        rs.resolve(cw, ch, doc.dpi()).round().intersect(&canvas)
    } else if a.raw("auto").is_some() {
        let img = compose::composite(doc)?;
        match trim_rect(&img, 8) {
            Some(r) => r,
            None => {
                return Ok(
                    json!({"cropped": false, "reason": "the composite is uniform", "size": [doc.width, doc.height]}),
                )
            }
        }
    } else if let Some(rs) = a.str("ratio") {
        let k = parse_ratio(&rs)?;
        let (w, h) = if cw / ch > k { ((ch * k).round().max(1.0), ch) } else { (cw, (cw / k).round().max(1.0)) };
        let (fx, fy) = Anchor::parse(&a.choice("gravity"))?.fractions();
        RectI::new(((cw - w) * fx).round() as i32, ((ch - h) * fy).round() as i32, w as u32, h as u32)
    } else {
        return Err(Error::new("PX1003", "'transform crop' needs --rect, --ratio, or --auto trim")
            .with_hint("--rect 0,0,800x600"));
    };
    if r.is_empty() {
        return Err(
            Error::new("PX3320", "the crop rectangle does not overlap the canvas").with_context("rect", r.to_json())
        );
    }
    let rasterized = engine::crop_doc(doc, ctx, r)?;
    let mut v = doc_result(doc, rasterized);
    v["cropped"] = json!(true);
    v["rect"] = r.to_json();
    v["method"] = json!(g.first().map(|s| s.trim_start_matches('-')).unwrap_or("rect"));
    Ok(v)
}

fn straighten(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let filter = interp(a)?;
    let mut detected = Value::Null;
    let angle = match a.opt_angle("angle")? {
        Some(d) => d,
        None => {
            let img = compose::composite(doc)?;
            let deg = match detect_tilt(&img) {
                Some(t) => {
                    detected = json!({"tilt": r3(t.degrees), "confidence": r3(t.confidence)});
                    -t.degrees
                }
                None => {
                    detected = json!({"tilt": Value::Null, "confidence": 0.0});
                    0.0
                }
            };
            let deg = (deg * 100.0).round() / 100.0;
            ctx.resolve_param("angle", deg);
            deg
        }
    };
    if angle.abs() > 45.0 {
        return Err(Error::new(
            "PX1013",
            format!("--angle {angle} is outside -45..45; use `transform rotate` for larger turns"),
        ));
    }
    let (w, h) = canvas_size(doc);
    let mut rasterized = Vec::new();
    let mut crop_rect = Value::Null;
    if angle.abs() > 1e-9 {
        rasterized = engine::doc_affine(
            doc,
            ctx,
            &Affine2::rotate(angle).about(w / 2.0, h / 2.0),
            doc.width,
            doc.height,
            filter,
        )?;
        if a.choice("crop") == "auto" {
            let (iw, ih) = max_inscribed(w, h, angle);
            let x0 = ((w - iw) / 2.0 - 1e-6).ceil().max(0.0);
            let y0 = ((h - ih) / 2.0 - 1e-6).ceil().max(0.0);
            let x1 = ((w + iw) / 2.0 + 1e-6).floor().min(w);
            let y1 = ((h + ih) / 2.0 + 1e-6).floor().min(h);
            let r = RectI::new(x0 as i32, y0 as i32, (x1 - x0).max(1.0) as u32, (y1 - y0).max(1.0) as u32);
            if r != doc.canvas() {
                rasterized.extend(engine::crop_doc(doc, ctx, r)?);
                crop_rect = r.to_json();
            }
        }
    }
    let mut v = doc_result(doc, rasterized);
    v["angle"] = json!(angle);
    v["detected"] = detected;
    v["crop"] = crop_rect;
    Ok(v)
}

/// `WxH` (absolute; % of the current size) or `+DWx+DH` / `-DW` (relative); blank = same.
fn parse_canvas_size(s: &str, w: f64, h: f64, dpi: f64) -> Result<(u32, u32)> {
    let bad = || {
        Error::new("PX1011", format!("--size '{s}' is not WxH or +DWx+DH"))
            .with_hint("--size 1080x1350, --size '+200x+0', or --size 'x1200'")
    };
    let (a, b) = s.trim().split_once(['x', 'X']).ok_or_else(bad)?;
    let side = |t: &str, cur: f64| -> Result<f64> {
        let t = t.trim();
        if t.is_empty() {
            return Ok(cur);
        }
        if let Some(rest) = t.strip_prefix('+') {
            return Ok(cur + Length::parse(rest).map_err(|_| bad())?.to_px(cur, dpi));
        }
        if let Some(rest) = t.strip_prefix('-') {
            return Ok(cur - Length::parse(rest).map_err(|_| bad())?.to_px(cur, dpi));
        }
        Ok(Length::parse(t).map_err(|_| bad())?.to_px(cur, dpi))
    };
    let (nw, nh) = (side(a, w)?.round(), side(b, h)?.round());
    if nw < 1.0 || nh < 1.0 {
        return Err(Error::new("PX1013", format!("--size '{s}' gives an empty canvas ({nw}x{nh})")));
    }
    let (nw, nh) = crate::sample::check_size(nw, nh)?;
    Ok((nw, nh))
}

fn over(src: [f32; 4], dst: [f32; 4]) -> [f32; 4] {
    let a = src[3] + dst[3] * (1.0 - src[3]);
    if a <= 1e-7 {
        return [0.0; 4];
    }
    let mut o = [0.0, 0.0, 0.0, a];
    for c in 0..3 {
        o[c] = (src[c] * src[3] + dst[c] * dst[3] * (1.0 - src[3])) / a;
    }
    o
}

/// Fill the area outside `old` (the previous canvas, in new coordinates) on the bottom
/// raster layer, or on a new bottom layer when the bottom is not raster.
fn fill_new_area(doc: &mut Document, fill: Rgba, old: RectI) {
    let canvas = doc.canvas();
    let in_new = |x: i32, y: i32| {
        canvas.intersect(&RectI::new(x, y, 1, 1)).w == 1 && old.intersect(&RectI::new(x, y, 1, 1)).w == 0
    };
    let col = fill.to_array();
    let bottom_raster = matches!(doc.layers.first().map(|l| &l.content), Some(LayerKind::Raster { .. }));
    if bottom_raster {
        let l = &mut doc.layers[0];
        let (lx, ly) = (l.x, l.y);
        if let LayerKind::Raster { image } = &mut l.content {
            let lr = RectI::new(lx, ly, image.width, image.height);
            let u = lr.union(&canvas);
            let src = &*image;
            let out = Image::from_fn(u.w, u.h, |x, y| {
                let (cx, cy) = (u.x + x as i32, u.y + y as i32);
                let (sx, sy) = (cx - lx, cy - ly);
                let p = if sx >= 0 && sy >= 0 && (sx as u32) < src.width && (sy as u32) < src.height {
                    src.get(sx as u32, sy as u32)
                } else {
                    [0.0; 4]
                };
                if in_new(cx, cy) {
                    over(p, col)
                } else {
                    p
                }
            });
            *image = out;
            l.x = u.x;
            l.y = u.y;
        }
    } else {
        let img = Image::from_fn(canvas.w, canvas.h, |x, y| if in_new(x as i32, y as i32) { col } else { [0.0; 4] });
        let mut layer = doc.new_layer("Canvas Fill", LayerKind::Raster { image: img });
        layer.locked = false;
        doc.layers.insert(0, layer);
    }
}

fn canvas(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let (w, h) = canvas_size(doc);
    let (nw, nh) = parse_canvas_size(&a.str_or("size", ""), w, h, doc.dpi())?;
    let (fx, fy) = anchor(a)?.fractions();
    let (dx, dy) = (((nw as f64 - w) * fx).round(), ((nh as f64 - h) * fy).round());
    let fill = a.color("fill")?;
    let rasterized = engine::doc_affine(doc, ctx, &Affine2::translate(dx, dy), nw, nh, None)?;
    let filled = fill.a > 0.0 && (nw as f64 > w || nh as f64 > h || dx > 0.0 || dy > 0.0);
    if filled {
        fill_new_area(doc, fill, RectI::new(dx as i32, dy as i32, w as u32, h as u32));
    }
    let mut v = doc_result(doc, rasterized);
    v["offset"] = json!([dx, dy]);
    v["fill"] = json!(if fill.a > 0.0 { fill.to_hex() } else { "transparent".into() });
    Ok(v)
}

fn unsharp(img: &mut Image, amount: f32) {
    let blurred = kernels::gaussian_blur(img, 0.7, kernels::auto_edge(img));
    img.data.par_chunks_mut(4).zip(blurred.data.par_chunks(4)).for_each(|(p, b)| {
        for c in 0..3 {
            p[c] = (p[c] + amount * (p[c] - b[c])).clamp(0.0, 1.0);
        }
    });
}

fn sharpen_tree(l: &mut Layer, amount: f32) {
    if let LayerKind::Raster { image } = &mut l.content {
        unsharp(image, amount);
    }
    if let Some(ch) = l.children_mut() {
        for c in ch.iter_mut() {
            sharpen_tree(c, amount);
        }
    }
}

fn filter_name(f: Filter) -> &'static str {
    match f {
        Filter::Nearest => "nearest",
        Filter::Bilinear => "bilinear",
        Filter::Bicubic => "bicubic",
        Filter::Lanczos3 => "lanczos3",
    }
}

fn resize(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let (w, h) = canvas_size(doc);
    let s = a.size("size")?.ok_or_else(|| Error::new("PX1003", "'transform resize' needs --size WxH"))?;
    let (nw, nh) = s.resolve(w, h, doc.dpi());
    let (nw, nh) = crate::sample::check_size(nw as f64, nh as f64)?;
    let filter = interp(a)?;
    let used = filter.unwrap_or_else(|| Filter::auto(doc.width.max(doc.height), nw.max(nh)));
    let (sx, sy) = (nw as f64 / w, nh as f64 / h);
    let rasterized = engine::doc_affine(doc, ctx, &Affine2::scale(sx, sy), nw, nh, filter)?;
    let amount = if a.is_auto("sharpen") {
        let k = (sx * sy).sqrt();
        let amt = if k < 1.0 { (0.6 * (1.0 - k)).min(0.5) } else { 0.0 };
        let amt = r3(amt);
        ctx.resolve_param("sharpen", amt);
        amt
    } else {
        a.f64("sharpen")?
    };
    if amount > 0.0 {
        for l in doc.layers.iter_mut() {
            sharpen_tree(l, amount as f32);
        }
    }
    let mut v = doc_result(doc, rasterized);
    v["scale"] = json!([r4(sx), r4(sy)]);
    v["interp"] = json!(filter_name(used));
    v["sharpen"] = json!(amount);
    Ok(v)
}

fn trim(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let thr = a.int("threshold")?;
    let img = compose::composite(doc)?;
    let Some(r) = trim_rect(&img, thr) else {
        return Ok(json!({"trimmed": false, "reason": "the composite is uniform", "size": [doc.width, doc.height]}));
    };
    if r == doc.canvas() {
        return Ok(json!({"trimmed": false, "reason": "no border to trim", "size": [doc.width, doc.height]}));
    }
    let rasterized = engine::crop_doc(doc, ctx, r)?;
    let mut v = doc_result(doc, rasterized);
    v["trimmed"] = json!(true);
    v["rect"] = r.to_json();
    Ok(v)
}
