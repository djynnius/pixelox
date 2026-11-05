//! `select *`: the active selection (`doc.selection`, a canvas-sized soft mask).
//!
//! Shape selections are rasterized with anti-aliasing (tiny-skia), optionally feathered,
//! then combined with the active selection by `--mode replace|add|subtract|intersect`.

use crate::raster::{ellipse_path, fill_mask, parse_path, polygon, rect_path};
use crate::util::{self, at_most_one, feather_param, len_px, mask_json, mode_param, resolve_mask_ref, select_mode};
use kurbo::BezPath;
use pixelox_core::image::SelectMode;
use pixelox_core::prelude::*;
use rayon::prelude::*;

const CAT: Category = Category::Select;

pub fn ops() -> Vec<OpSpec> {
    vec![
        OpSpec::doc("select rect", CAT, "Select a rectangle", rect)
            .about("Replace or combine the active selection with an anti-aliased rectangle. Coordinates are canvas pixels (or %, pt, mm, in); negative X/Y count from the right/bottom edge. `--feather` softens the edge before the boolean combine.")
            .param(param("rect", ParamKind::Rect).required().help("Rectangle X,Y,WxH in canvas coordinates"))
            .param(feather_param())
            .param(mode_param())
            .example("pixelox select rect doc.pxo --rect 100,80,640x480 --in-place", "Select a 640x480 box")
            .example("pixelox select rect doc.pxo --rect 0,0,50%x100% --feather 12 --in-place", "Select the left half with a soft edge")
            .example("pixelox select rect doc.pxo --rect 200,200,100x100 --mode subtract --in-place", "Cut a square out of the selection"),
        OpSpec::doc("select ellipse", CAT, "Select an ellipse inscribed in a rectangle", ellipse)
            .about("Replace or combine the active selection with the anti-aliased ellipse inscribed in --rect (a circle when the rect is square).")
            .param(param("rect", ParamKind::Rect).required().help("Bounding rectangle X,Y,WxH of the ellipse"))
            .param(feather_param())
            .param(mode_param())
            .example("pixelox select ellipse doc.pxo --rect 300,200,400x400 --in-place", "Select a circle")
            .example("pixelox select ellipse doc.pxo --rect 10%,10%,80%x80% --feather 30 --in-place", "Soft vignette-style oval")
            .example("pixelox select ellipse doc.pxo --rect 0,0,200x100 --mode add --in-place", "Add an oval to the selection"),
        OpSpec::doc("select poly", CAT, "Select a polygon from vertices", poly)
            .about("Select the anti-aliased polygon through --points (closed automatically, nonzero winding). Points are canvas pixels.")
            .param(param("points", ParamKind::Points).required().help("Vertices 'x1,y1 x2,y2 ...' in canvas pixels (at least 3)"))
            .param(feather_param())
            .param(mode_param())
            .example("pixelox select poly doc.pxo --points '10,10 400,40 220,300' --in-place", "Select a triangle")
            .example("pixelox select poly doc.pxo --points '0,0 100,0 100,100 0,100' --feather 4 --in-place", "Soft square")
            .example("pixelox select poly doc.pxo --points '50,50 90,60 70,90' --mode intersect --in-place", "Intersect with a triangle"),
        OpSpec::doc("select path", CAT, "Select a region from SVG path data or points", path)
            .about("Select the region enclosed by SVG path data (--d, canvas pixels; curves and arcs allowed) or by a points file (--points-file). The points file is JSON: an array of [x, y] pairs or {\"x\", \"y\"} objects, an object {\"points\": [...]}, or {\"d\": \"M...Z\"}; an array of arrays of points makes several polygons.")
            .param(param("d", ParamKind::Str).help("SVG path data, e.g. 'M 10 10 C 80 0 120 90 200 40 L 150 200 Z'"))
            .param(param("points-file", ParamKind::Path).help("JSON file of points (see the command description)"))
            .param(param("fill-rule", ParamKind::Choice).choices(&["nonzero", "evenodd"]).default("nonzero").advanced().help("Which overlaps count as inside: nonzero (SVG default) or evenodd (holes)"))
            .param(feather_param())
            .param(mode_param())
            .example("pixelox select path doc.pxo --d 'M 10 10 C 120 0 200 80 180 200 L 20 180 Z' --in-place", "Bezier region")
            .example("pixelox select path doc.pxo --points-file lasso.json --feather 2 --in-place", "Freehand lasso from a points file")
            .example("pixelox select path doc.pxo --d 'M0 0H100V100H0Z M25 25H75V75H25Z' --fill-rule evenodd --in-place", "A frame with a hole"),
        OpSpec::doc("select color", CAT, "Select by color range or magic wand", color_range)
            .about("Select pixels within --tolerance (0..255, max channel difference, premultiplied) of --ref. Samples the composite unless --layer names a layer. With --contiguous, flood-fills from --at (magic wand); --ref defaults to the color under --at.")
            .param(param("ref", ParamKind::Color).help("Reference color (default: the color under --at)"))
            .param(param("at", ParamKind::Point).help("Seed point X,Y (needed for --contiguous)"))
            .param(param("tolerance", ParamKind::Int).default("32").range("0..255").help("Largest per-channel difference that is selected, 0..255"))
            .param(param("contiguous", ParamKind::Flag).help("Only pixels connected to --at (magic wand)"))
            .param(param("layer", ParamKind::Layer).help("Sample this layer instead of the composite"))
            .param(feather_param())
            .param(mode_param())
            .example("pixelox select color photo.pxo --ref '#2a6fdb' --tolerance 40 --in-place", "Select every blue-ish pixel")
            .example("pixelox select color photo.pxo --at 12,12 --contiguous --tolerance 20 --in-place", "Magic wand from the corner")
            .example("pixelox select color photo.pxo --ref white --layer Logo --mode subtract --in-place", "Remove the logo's white from the selection"),
        OpSpec::doc("select luma", CAT, "Select by luminosity or channel range", luma)
            .about("Select pixels whose luma (Rec. 709) or channel value lies in --range (0..1 or %), with a linear soft edge of --softness outside it. Samples the composite unless --layer is given; color channels are premultiplied, so transparent areas read as dark.")
            .param(param("range", ParamKind::Range).default("0.5..1").help("Selected value range a..b in 0..1 (or %); default: the highlights"))
            .param(param("channel", ParamKind::Choice).choices(&["l", "r", "g", "b", "a"]).default("l").help("Value to test: l (luma), r, g, b, or a (alpha)"))
            .param(param("softness", ParamKind::Float).default("0.05").range("0..1").help("Width of the soft edge outside the range (0 = hard)"))
            .param(param("layer", ParamKind::Layer).help("Sample this layer instead of the composite"))
            .param(feather_param())
            .param(mode_param())
            .example("pixelox select luma photo.pxo --range 0.7..1 --in-place", "Select the highlights")
            .example("pixelox select luma photo.pxo --range 0..0.25 --softness 0.1 --in-place", "Select the shadows softly")
            .example("pixelox select luma photo.pxo --channel a --range 0.5..1 --layer Cutout --in-place", "Select a layer's opaque pixels"),
        OpSpec::doc("select all", CAT, "Select the whole canvas", all)
            .about("Select every pixel (with --mode subtract this clears the selection; with intersect it keeps it).")
            .param(mode_param())
            .example("pixelox select all doc.pxo --in-place", "Select everything")
            .example("pixelox select all photo.jpg -o selected.pxo", "Open a photo with everything selected")
            .example("pixelox select all doc.pxo --mode add --in-place", "Grow the selection to the canvas"),
        OpSpec::doc("select none", CAT, "Clear the active selection", none)
            .about("Deselect: the document has no active selection afterwards (named masks are untouched).")
            .example("pixelox select none doc.pxo --in-place", "Deselect")
            .example("pixelox select none doc.pxo -o clean.pxo", "Write a copy without a selection")
            .example("pixelox select none doc.pxo --dry-run --json", "Plan it"),
        OpSpec::doc("select invert", CAT, "Invert the active selection", invert)
            .about("Complement the selection (coverage becomes 1 - coverage). With no selection, the result selects everything.")
            .example("pixelox select invert doc.pxo --in-place", "Select everything outside the selection")
            .example("pixelox select invert doc.pxo -o inverted.pxo", "Write the inverted selection to a copy")
            .example("pixelox select invert doc.pxo --dry-run --json", "Plan it"),
        OpSpec::doc("select save", CAT, "Store the selection as a named mask", save)
            .about("Copy the active selection into the document's named-mask library (PX2310 if the name is taken, unless --replace).")
            .param(param("as", ParamKind::Str).required().help("Name for the mask (cannot start with @)"))
            .param(param("replace", ParamKind::Flag).help("Overwrite a mask with the same name"))
            .example("pixelox select save doc.pxo --as sky --in-place", "Save the selection as 'sky'")
            .example("pixelox select save doc.pxo --as sky --replace --in-place", "Overwrite 'sky'")
            .example("pixelox select save doc.pxo --as subject -o masked.pxo", "Save into a copy"),
        OpSpec::doc("select load", CAT, "Load a named mask as the selection", load)
            .about("Make a mask the active selection, combined by --mode. NAME may be a library mask, @alpha:<layer>, @mask:<layer> (a layer mask), or a grayscale image path the size of the canvas.")
            .param(param("name", ParamKind::Str).required().help("Mask to load: a library name, @alpha:<layer>, @mask:<layer>, or an image path"))
            .positional("name")
            .param(mode_param())
            .example("pixelox select load doc.pxo sky --in-place", "Select the 'sky' mask")
            .example("pixelox select load doc.pxo @alpha:Logo --mode add --in-place", "Add a layer's opaque pixels")
            .example("pixelox select load doc.pxo wires --mode subtract --in-place", "Remove 'wires' from the selection"),
        OpSpec::doc("select bounds", CAT, "Report the selection's bbox, area, coverage", bounds)
            .about("Read-only: the selection's bounding box [x, y, w, h] (coverage above 0.5/255), area in px (sum of coverage), and coverage in percent of the canvas; `empty: true` when nothing is selected.")
            .read_only()
            .example("pixelox select bounds doc.pxo --json", "Measure the selection")
            .example("pixelox select bounds doc.pxo", "Print the bounds")
            .example("pixelox select bounds photo.pxo --json --quiet", "Quiet JSON"),
    ]
}

/// Combine `shape` (feathered by `feather_px`) into the active selection.
pub fn apply(doc: &mut Document, ctx: &mut OpCtx, shape: Mask, mode: SelectMode, feather_px: f64) -> Value {
    let (w, h) = (doc.width, doc.height);
    let shape = util::feather(&shape, feather_px);
    let new = match (mode, doc.selection.take()) {
        (SelectMode::Replace, _) => shape,
        (_, prev) => {
            let mut base = prev.map(|p| util::fit_mask(p, w, h)).unwrap_or_else(|| Mask::new(w, h));
            base.combine(&shape, mode);
            base
        }
    };
    let j = mask_json(&new);
    if j["empty"] == json!(true) {
        ctx.warn("PXW2002", "the selection is empty");
    }
    doc.selection = Some(new);
    json!({"mode": format!("{mode:?}").to_ascii_lowercase(), "selection": j})
}

fn shape_op(a: &Args, doc: &mut Document, ctx: &mut OpCtx, path: &BezPath, even_odd: bool) -> Result<Value> {
    let mode = select_mode(a)?;
    let f = len_px(a, "feather", doc)?;
    let m = fill_mask(doc.width, doc.height, path, even_odd);
    Ok(apply(doc, ctx, m, mode, f))
}

fn rect_arg(a: &Args, doc: &Document) -> Result<(f64, f64, f64, f64)> {
    let r = a.rect("rect")?.ok_or_else(|| Error::new("PX1003", "missing --rect"))?.resolve(
        doc.width as f64,
        doc.height as f64,
        doc.dpi(),
    );
    if r.w <= 0.0 || r.h <= 0.0 {
        return Err(Error::new("PX1011", "--rect has zero width or height").with_context("rect", [r.x, r.y, r.w, r.h]));
    }
    Ok((r.x, r.y, r.w, r.h))
}

fn rect(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let (x, y, w, h) = rect_arg(a, doc)?;
    shape_op(a, doc, ctx, &rect_path(x, y, w, h), false)
}

fn ellipse(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let (x, y, w, h) = rect_arg(a, doc)?;
    shape_op(a, doc, ctx, &ellipse_path(x, y, w, h), false)
}

fn poly(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let pts = a.points("points")?;
    if pts.len() < 3 {
        return Err(Error::new("PX1010", format!("--points needs at least 3 vertices, got {}", pts.len()))
            .with_hint("--points '10,10 200,10 100,150'"));
    }
    shape_op(a, doc, ctx, &polygon(&pts), false)
}

/// Polygons from a points file (see `select path`).
fn points_file_path(v: &Value) -> Result<BezPath> {
    fn point(p: &Value) -> Option<(f64, f64)> {
        match p {
            Value::Array(xy) if xy.len() == 2 => Some((xy[0].as_f64()?, xy[1].as_f64()?)),
            Value::Object(o) => Some((o.get("x")?.as_f64()?, o.get("y")?.as_f64()?)),
            _ => None,
        }
    }
    let bad = || {
        Error::new("PX1010", "the points file is not a list of points")
            .with_hint("write [[x, y], ...], [{\"x\": .., \"y\": ..}, ...], {\"points\": [...]}, or {\"d\": \"M...Z\"}")
    };
    if let Some(d) = v.get("d").and_then(|d| d.as_str()) {
        return parse_path(d);
    }
    let list = v.get("points").unwrap_or(v).as_array().ok_or_else(bad)?;
    let rings: Vec<&Vec<Value>> =
        if list.first().is_some_and(|f| f.as_array().is_some_and(|a| a.first().is_some_and(|p| !p.is_number()))) {
            list.iter().filter_map(|r| r.as_array()).collect()
        } else {
            vec![list]
        };
    let mut path = BezPath::new();
    for ring in rings {
        let pts: Vec<(f64, f64)> = ring.iter().map(point).collect::<Option<_>>().ok_or_else(bad)?;
        if pts.len() < 3 {
            return Err(bad().with_context("points", pts.len()));
        }
        path.extend(polygon(&pts));
    }
    Ok(path)
}

fn path(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    at_most_one(a, &["d", "points-file"], &[])?;
    let p = match (a.str("d"), a.path("points-file")) {
        (Some(d), _) => parse_path(&d)?,
        (None, Some(f)) => {
            let bytes = ctx.read_aux(&f, "points", Some("points-file"))?;
            let v: Value = serde_json::from_slice(&bytes)?;
            points_file_path(&v)?
        }
        (None, None) => {
            return Err(Error::new("PX1003", "'select path' needs --d or --points-file")
                .with_hint("--d 'M 10 10 L 200 10 L 100 150 Z'"))
        }
    };
    let even_odd = a.choice("fill-rule") == "evenodd";
    shape_op(a, doc, ctx, &p, even_odd)
}

/// The canvas-sized pixels to sample: one layer (`--layer`) or the composite.
pub fn sample_source(doc: &Document, layer: Option<&str>) -> Result<Image> {
    match layer {
        None => compose::composite(doc),
        Some(r) => {
            let p = doc.resolve(r)?;
            let l = doc.layer(&p);
            Ok(match compose::render_layer(doc, l)? {
                Some(rd) => rd.to_canvas(doc.width, doc.height),
                None => Image::new(doc.width, doc.height),
            })
        }
    }
}

fn seed_point(a: &Args, doc: &Document) -> Result<Option<(u32, u32)>> {
    let Some(p) = a.point("at")? else { return Ok(None) };
    let (x, y) = p.resolve(doc.width as f64, doc.height as f64, doc.dpi());
    let (ix, iy) = (x.floor(), y.floor());
    if ix < 0.0 || iy < 0.0 || ix >= doc.width as f64 || iy >= doc.height as f64 {
        return Err(Error::new("PX1011", format!("--at {x},{y} is outside the {}x{} canvas", doc.width, doc.height))
            .with_context("at", [x, y]));
    }
    Ok(Some((ix as u32, iy as u32)))
}

/// Scanline flood fill over pixels with coverage > 0, 4-connected.
pub fn flood(cov: &Mask, sx: u32, sy: u32) -> Mask {
    let (w, h) = (cov.width as usize, cov.height as usize);
    let mut out = Mask::new(cov.width, cov.height);
    let mut seen = vec![false; w * h];
    let ok = |i: usize, seen: &[bool]| !seen[i] && cov.data[i] > 0.0;
    let mut stack = vec![(sx as usize, sy as usize)];
    while let Some((x, y)) = stack.pop() {
        if !ok(y * w + x, &seen) {
            continue;
        }
        let (mut l, mut r) = (x, x);
        while l > 0 && ok(y * w + l - 1, &seen) {
            l -= 1;
        }
        while r + 1 < w && ok(y * w + r + 1, &seen) {
            r += 1;
        }
        for i in l..=r {
            seen[y * w + i] = true;
            out.data[y * w + i] = cov.data[y * w + i];
        }
        for ny in [y.wrapping_sub(1), y + 1] {
            if ny >= h {
                continue;
            }
            let mut in_run = false;
            for i in l..=r {
                if ok(ny * w + i, &seen) {
                    if !in_run {
                        stack.push((i, ny));
                        in_run = true;
                    }
                } else {
                    in_run = false;
                }
            }
        }
    }
    out
}

fn color_range(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let mode = select_mode(a)?;
    let f = len_px(a, "feather", doc)?;
    let src = sample_source(doc, a.raw("layer"))?;
    let at = seed_point(a, doc)?;
    let contiguous = a.flag("contiguous");
    if contiguous && at.is_none() {
        return Err(
            Error::new("PX1003", "--contiguous needs a seed point --at X,Y").with_hint("--at 10,10 --contiguous")
        );
    }
    let reference: [f32; 4] = match (a.opt_color("ref")?, at) {
        (Some(c), _) => c.to_array(),
        (None, Some((x, y))) => src.get(x, y),
        (None, None) => {
            return Err(Error::new("PX1003", "'select color' needs --ref COLOR or --at X,Y")
                .with_hint("--ref '#ff0000' or --at 10,10"))
        }
    };
    let tol = a.int("tolerance")? as f32;
    let pm = |p: [f32; 4]| [p[0] * p[3], p[1] * p[3], p[2] * p[3], p[3]];
    let rp = pm(reference);
    let mut cov = Mask::new(doc.width, doc.height);
    cov.data.par_iter_mut().zip(src.data.par_chunks(4)).for_each(|(o, p)| {
        let q = pm([p[0], p[1], p[2], p[3]]);
        let d = (0..4).map(|c| (q[c] - rp[c]).abs()).fold(0.0f32, f32::max) * 255.0;
        *o = (tol + 1.0 - d).clamp(0.0, 1.0);
    });
    let m = match (contiguous, at) {
        (true, Some((x, y))) => flood(&cov, x, y),
        _ => cov,
    };
    let mut v = apply(doc, ctx, m, mode, f);
    v["ref"] = json!(Rgba::from_array(reference).to_hex());
    v["contiguous"] = json!(contiguous);
    Ok(v)
}

fn luma(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let mode = select_mode(a)?;
    let f = len_px(a, "feather", doc)?;
    let (lo, hi) = a.range("range")?.unwrap_or((0.5, 1.0));
    if lo > hi {
        return Err(Error::new("PX1010", format!("--range {lo}..{hi} is empty (low end above high end)")));
    }
    let soft = a.f64("softness")? as f32;
    let ch = a.choice("channel");
    let src = sample_source(doc, a.raw("layer"))?;
    let (lo, hi) = (lo as f32, hi as f32);
    let mut m = Mask::new(doc.width, doc.height);
    m.data.par_iter_mut().zip(src.data.par_chunks(4)).for_each(|(o, p)| {
        let v = match ch.as_str() {
            "r" => p[0] * p[3],
            "g" => p[1] * p[3],
            "b" => p[2] * p[3],
            "a" => p[3],
            _ => color::luma(p[0], p[1], p[2]) * p[3],
        };
        let out = if v < lo {
            lo - v
        } else if v > hi {
            v - hi
        } else {
            0.0
        };
        *o = if out <= 0.0 {
            1.0
        } else if soft <= 0.0 {
            0.0
        } else {
            (1.0 - out / soft).clamp(0.0, 1.0)
        };
    });
    Ok(apply(doc, ctx, m, mode, f))
}

fn all(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let mode = select_mode(a)?;
    let m = Mask::filled(doc.width, doc.height, 1.0);
    Ok(apply(doc, ctx, m, mode, 0.0))
}

fn none(_: &Args, doc: &mut Document, _: &mut OpCtx) -> Result<Value> {
    let had = doc.selection.take().is_some();
    Ok(json!({"cleared": had, "selection": Value::Null}))
}

fn invert(_: &Args, doc: &mut Document, _: &mut OpCtx) -> Result<Value> {
    let (w, h) = (doc.width, doc.height);
    let m = match doc.selection.take() {
        Some(s) => util::fit_mask(s, w, h).inverted(),
        None => Mask::filled(w, h, 1.0),
    };
    let j = mask_json(&m);
    doc.selection = Some(m);
    Ok(json!({"selection": j}))
}

fn save(a: &Args, doc: &mut Document, _: &mut OpCtx) -> Result<Value> {
    let name = a.str("as").unwrap_or_default();
    let sel = doc.named_mask("@sel")?;
    let replaced = doc.masks.contains_key(name.trim());
    doc.put_mask(name.trim(), util::fit_mask(sel, doc.width, doc.height), a.flag("replace"))?;
    let j = mask_json(&doc.masks[name.trim()]);
    Ok(json!({"mask": name.trim(), "replaced": replaced, "stats": j}))
}

fn load(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let mode = select_mode(a)?;
    let name = a.str("name").unwrap_or_default();
    let m = resolve_mask_ref(ctx, doc, &name, "name")?;
    let mut v = apply(doc, ctx, m, mode, 0.0);
    v["mask"] = json!(name);
    Ok(v)
}

fn bounds(_: &Args, doc: &mut Document, _: &mut OpCtx) -> Result<Value> {
    Ok(match &doc.selection {
        None => json!({"empty": true, "active": false, "bounds": Value::Null, "area": 0, "coverage": 0}),
        Some(s) => {
            let mut j = mask_json(s);
            j["active"] = json!(true);
            j
        }
    })
}
