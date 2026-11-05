//! `path unite|subtract|intersect|exclude|simplify|outline-stroke` (PRD §8.11, §7.9).
//!
//! Booleans flatten every operand at 0.1 px, normalize each with its own fill rule, and
//! combine them with the i_overlay polygon engine; the result is a polygonal path.

use crate::shape::shape_layer;
use crate::spec::{self, anchor_count, bez_to_d, flatten_contours, polygons_to_d, ShapeSpec, PREC};
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::float::simplify::SimplifyShape;
use i_overlay::float::single::SingleFloatOverlay;
use kurbo::{BezPath, ParamCurve, PathEl, PathSeg, Point};
use pixelox_core::doc::LayerPath;
use pixelox_core::prelude::*;

/// Flattening tolerance for booleans, in canvas px (PRD §7.9).
pub const FLATTEN_TOL: f64 = 0.1;

pub type Shapes = Vec<Vec<Vec<[f64; 2]>>>;

fn boolean_op(
    name: &'static str,
    summary: &'static str,
    about: &'static str,
    f: pixelox_core::op::DocFn,
    verb: &'static [&'static str; 3],
) -> OpSpec {
    OpSpec::doc(name, Category::Vector, summary, f)
        .about(about)
        .param(param("layers", ParamKind::List).required().help("Shape layers to combine, in order: A,B[,C...] (names, #N, @top, ...)"))
        .param(param("into", ParamKind::Str).help("Write the result to a new layer with this name and keep the operands (default: replace A, remove the others)"))
        .example(verb[0], "Replace A with the result and remove B")
        .example(verb[1], "Keep the operands; write a new layer")
        .example(verb[2], "Plan without writing")
}

pub fn ops() -> Vec<OpSpec> {
    vec![
        boolean_op(
            "path unite",
            "Union of shape layers",
            "Combines the filled areas of the shape layers into one path (A ∪ B). Curves are flattened at 0.1 px; the result is polygonal. Paint, opacity, and blend come from A.",
            unite,
            &[
                "pixelox path unite logo.pxo --in-place --layers Circle,Square",
                "pixelox path unite logo.pxo --in-place --layers 'Left Wing,Right Wing,Body' --into Bird",
                "pixelox path unite logo.pxo --layers A,B --dry-run --json",
            ],
        ),
        boolean_op(
            "path subtract",
            "Subtract later shape layers from the first",
            "Cuts B (and any further layers) out of A (A − B). Curves are flattened at 0.1 px; the result is polygonal. Paint comes from A.",
            subtract,
            &[
                "pixelox path subtract badge.pxo --in-place --layers Disc,Hole",
                "pixelox path subtract badge.pxo --in-place --layers Disc,Hole --into Ring",
                "pixelox path subtract badge.pxo --layers Disc,Hole --dry-run --json",
            ],
        ),
        boolean_op(
            "path intersect",
            "Intersection of shape layers",
            "Keeps only the area covered by every operand (A ∩ B). Curves are flattened at 0.1 px; the result is polygonal. Paint comes from A.",
            intersect,
            &[
                "pixelox path intersect doc.pxo --in-place --layers Circle,Square",
                "pixelox path intersect doc.pxo --in-place --layers Circle,Square --into Lens",
                "pixelox path intersect doc.pxo --layers Circle,Square --dry-run --json",
            ],
        ),
        boolean_op(
            "path exclude",
            "Exclusive-or of shape layers",
            "Keeps the area covered by an odd number of operands (A ⊕ B): overlaps become holes. Curves are flattened at 0.1 px; the result is polygonal. Paint comes from A.",
            exclude,
            &[
                "pixelox path exclude doc.pxo --in-place --layers Circle,Square",
                "pixelox path exclude doc.pxo --in-place --layers Circle,Square --into Pattern",
                "pixelox path exclude doc.pxo --layers Circle,Square --dry-run --json",
            ],
        ),
        OpSpec::doc("path simplify", Category::Vector, "Reduce the anchor count of a shape path", simplify)
            .about("Removes anchors while staying within `--tolerance` px of the original: straight runs are reduced with Douglas–Peucker, smooth curve runs are refitted with fewer cubic Béziers, and corners are kept.")
            .param(common::layer())
            .param(param("tolerance", ParamKind::Float).default("0.5").range("0..1000").help("Maximum deviation from the original outline in px"))
            .example("pixelox path simplify logo.pxo --in-place --layer Trace --tolerance 1", "Simplify a traced outline")
            .example("pixelox path simplify logo.pxo --in-place --layer @top", "Simplify the top shape at 0.5 px")
            .example("pixelox path simplify logo.pxo --layer Blob --tolerance 2 --dry-run --json", "See how many anchors would remain"),
        OpSpec::doc("path outline-stroke", Category::Vector, "Convert a shape's stroke to a filled path", outline_stroke)
            .about("Expands the stroke (width, dashes, caps, joins) into a filled outline painted with the stroke color. A shape without a fill is replaced; a shape with a fill keeps its fill and gets a new '<name> Stroke' layer above it.")
            .param(common::layer())
            .example("pixelox path outline-stroke badge.pxo --in-place --layer Burst", "Outline the burst's stroke")
            .example("pixelox path outline-stroke doc.pxo --in-place --layer Line", "Turn a line into a filled shape")
            .example("pixelox path outline-stroke doc.pxo --layer Frame --dry-run --json", "Plan without writing"),
    ]
}

/// Flatten and normalize one shape with its own fill rule (non-overlapping contours).
pub fn normalize(s: &ShapeSpec) -> Result<Shapes> {
    let contours = flatten_contours(&s.bez()?, FLATTEN_TOL);
    if contours.is_empty() {
        return Ok(Vec::new());
    }
    let rule = if s.fill_rule == "evenodd" { FillRule::EvenOdd } else { FillRule::NonZero };
    Ok(contours.simplify_shape(rule))
}

/// Combine normalized shapes left to right with `rule`.
pub fn combine(shapes: &[Shapes], rule: OverlayRule) -> Shapes {
    let mut acc = shapes[0].clone();
    for s in &shapes[1..] {
        acc = acc.overlay(s, rule, FillRule::NonZero);
    }
    acc
}

fn ring_area(c: &[[f64; 2]]) -> f64 {
    let n = c.len();
    let mut a = 0.0;
    for i in 0..n {
        let (p, q) = (c[i], c[(i + 1) % n]);
        a += p[0] * q[1] - q[0] * p[1];
    }
    (a / 2.0).abs()
}

/// Filled area of normalized shapes (outer contour minus holes).
pub fn shapes_area(s: &Shapes) -> f64 {
    s.iter()
        .map(|shape| {
            shape.iter().enumerate().map(|(i, c)| if i == 0 { ring_area(c) } else { -ring_area(c) }).sum::<f64>()
        })
        .sum()
}

fn unite(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    boolean(a, doc, ctx, "unite", OverlayRule::Union)
}
fn subtract(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    boolean(a, doc, ctx, "subtract", OverlayRule::Difference)
}
fn intersect(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    boolean(a, doc, ctx, "intersect", OverlayRule::Intersect)
}
fn exclude(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    boolean(a, doc, ctx, "exclude", OverlayRule::Xor)
}

fn boolean(a: &Args, doc: &mut Document, _ctx: &mut OpCtx, op: &str, rule: OverlayRule) -> Result<Value> {
    let refs = a.list("layers");
    if refs.len() < 2 {
        return Err(Error::new("PX1003", format!("'path {op}' needs at least two shape layers in --layers"))
            .with_hint(format!("e.g. --layers A,B (A {op} B)")));
    }
    let mut ids: Vec<String> = Vec::new();
    let mut names: Vec<String> = Vec::new();
    let mut specs: Vec<ShapeSpec> = Vec::new();
    for r in &refs {
        let (p, s) = shape_layer(doc, Some(r))?;
        let l = doc.layer(&p);
        if ids.contains(&l.id) {
            return Err(Error::new("PX1010", format!("layer '{}' is listed twice in --layers", l.name)));
        }
        ids.push(l.id.clone());
        names.push(l.name.clone());
        specs.push(s);
    }
    let into = a.str("into").map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
    if into.is_none() {
        for id in &ids {
            doc.layer(&doc.find_by_id(id).expect("resolved")).check_unlocked()?;
        }
    }
    let shapes: Vec<Shapes> = specs.iter().map(normalize).collect::<Result<_>>()?;
    let result = combine(&shapes, rule);
    let contours: Vec<Vec<[f64; 2]>> = result.iter().flatten().cloned().collect();
    let d = polygons_to_d(&contours, PREC);
    let mut out = specs[0].clone();
    out.d = d;
    out.fill_rule = "nonzero".into();
    out.geometry = json!({"type": "boolean", "op": op, "layers": names});
    let area = spec::round(shapes_area(&result), 3);
    let (layer_name, layer_id, removed) = match into {
        Some(name) => {
            doc.check_name_free(&name, None)?;
            let src = doc.layer(&doc.find_by_id(&ids[0]).expect("resolved")).clone();
            let mut l = doc.new_layer(&name, LayerKind::Shape { spec: out.to_value() });
            l.opacity = src.opacity;
            l.blend = src.blend;
            // above the topmost operand in panel order
            let top: LayerPath =
                doc.walk().into_iter().find(|(_, l)| ids.contains(&l.id)).map(|(p, _)| p).expect("operand");
            let (n, i) = (l.name.clone(), l.id.clone());
            doc.insert_above(Some(&top), l);
            (n, i, Vec::<String>::new())
        }
        None => {
            let p = doc.find_by_id(&ids[0]).expect("resolved");
            let l = doc.layer_mut(&p);
            if let LayerKind::Shape { spec } = &mut l.content {
                out.write_into(spec);
            }
            let (n, i) = (l.name.clone(), l.id.clone());
            for id in &ids[1..] {
                let p = doc.find_by_id(id).expect("resolved");
                doc.remove(&p);
            }
            doc.active = Some(i.clone());
            (n, i, names[1..].to_vec())
        }
    };
    Ok(json!({
        "layer": layer_name,
        "id": layer_id,
        "op": op,
        "operands": names,
        "removed": removed,
        "area": area,
        "contours": contours.len(),
        "empty": contours.is_empty(),
        "bbox": spec::spec_bbox(&out),
    }))
}

// ---------------------------------------------------------------------------------------
// simplify

struct Sub {
    start: Point,
    segs: Vec<PathSeg>,
    closed: bool,
}

fn subpaths(p: &BezPath) -> Vec<Sub> {
    let mut subs = Vec::new();
    let mut cur: Option<Sub> = None;
    let mut last = Point::ZERO;
    for el in p.elements() {
        let seg = |s: PathSeg, cur: &mut Option<Sub>| {
            cur.get_or_insert_with(|| Sub { start: last, segs: Vec::new(), closed: false }).segs.push(s)
        };
        match *el {
            PathEl::MoveTo(a) => {
                if let Some(s) = cur.take() {
                    subs.push(s);
                }
                cur = Some(Sub { start: a, segs: Vec::new(), closed: false });
                last = a;
            }
            PathEl::LineTo(a) => {
                seg(PathSeg::Line(kurbo::Line::new(last, a)), &mut cur);
                last = a;
            }
            PathEl::QuadTo(b, a) => {
                seg(PathSeg::Quad(kurbo::QuadBez::new(last, b, a)), &mut cur);
                last = a;
            }
            PathEl::CurveTo(b, c, a) => {
                seg(PathSeg::Cubic(kurbo::CubicBez::new(last, b, c, a)), &mut cur);
                last = a;
            }
            PathEl::ClosePath => {
                if let Some(mut s) = cur.take() {
                    s.closed = true;
                    last = s.start;
                    subs.push(s);
                }
            }
        }
    }
    if let Some(s) = cur {
        subs.push(s);
    }
    subs
}

fn seg_dist(p: Point, a: Point, b: Point) -> f64 {
    let ab = b - a;
    let l2 = ab.hypot2();
    if l2 < 1e-18 {
        return (p - a).hypot();
    }
    let t = ((p - a).dot(ab) / l2).clamp(0.0, 1.0);
    (p - (a + ab * t)).hypot()
}

/// Ramer–Douglas–Peucker (iterative; endpoints kept).
pub fn rdp(pts: &[Point], tol: f64) -> Vec<Point> {
    let n = pts.len();
    if n < 3 {
        return pts.to_vec();
    }
    let mut keep = vec![false; n];
    keep[0] = true;
    keep[n - 1] = true;
    let mut stack = vec![(0usize, n - 1)];
    while let Some((s, e)) = stack.pop() {
        let mut best = (0.0, 0usize);
        for (i, p) in pts.iter().enumerate().take(e).skip(s + 1) {
            let d = seg_dist(*p, pts[s], pts[e]);
            if d > best.0 {
                best = (d, i);
            }
        }
        if best.0 > tol {
            keep[best.1] = true;
            stack.push((s, best.1));
            stack.push((best.1, e));
        }
    }
    pts.iter().zip(keep).filter(|(_, k)| *k).map(|(p, _)| *p).collect()
}

/// Simplify a path within `tol` px: RDP on straight runs, kurbo curve refitting on curves.
pub fn simplify_path(p: &BezPath, tol: f64) -> BezPath {
    let mut out = BezPath::new();
    for sub in subpaths(p) {
        out.move_to(sub.start);
        let mut i = 0;
        while i < sub.segs.len() {
            let is_line = matches!(sub.segs[i], PathSeg::Line(_));
            let mut j = i;
            while j < sub.segs.len() && matches!(sub.segs[j], PathSeg::Line(_)) == is_line {
                j += 1;
            }
            let run = &sub.segs[i..j];
            if is_line {
                let mut pts = vec![run[0].start()];
                pts.extend(run.iter().map(|s| s.end()));
                for q in rdp(&pts, tol).into_iter().skip(1) {
                    out.line_to(q);
                }
            } else {
                let mut rp = BezPath::new();
                rp.move_to(run[0].start());
                for s in run {
                    match *s {
                        PathSeg::Quad(q) => rp.quad_to(q.p1, q.p2),
                        PathSeg::Cubic(c) => rp.curve_to(c.p1, c.p2, c.p3),
                        PathSeg::Line(l) => rp.line_to(l.p1),
                    }
                }
                let simp = kurbo::simplify::simplify_bezpath(
                    rp.elements().iter().copied(),
                    tol.max(1e-3),
                    &kurbo::simplify::SimplifyOptions::default(),
                );
                for el in simp.elements().iter().skip(1) {
                    if !matches!(el, PathEl::MoveTo(_) | PathEl::ClosePath) {
                        out.push(*el);
                    }
                }
            }
            i = j;
        }
        if sub.closed {
            out.close_path();
        }
    }
    out
}

fn simplify(a: &Args, doc: &mut Document, _: &mut OpCtx) -> Result<Value> {
    let (p, mut s) = shape_layer(doc, a.raw("layer"))?;
    doc.layer(&p).check_unlocked()?;
    let tol = a.f64("tolerance")?;
    let bez = s.bez()?;
    let before = anchor_count(&bez);
    let simp = if tol <= 0.0 { bez } else { simplify_path(&bez, tol) };
    let after = anchor_count(&simp);
    s.d = bez_to_d(&simp, PREC);
    let l = doc.layer_mut(&p);
    if let LayerKind::Shape { spec } = &mut l.content {
        s.write_into(spec);
    }
    Ok(json!({"layer": l.name, "id": l.id, "anchors_before": before, "anchors_after": after, "tolerance": tol}))
}

// ---------------------------------------------------------------------------------------
// outline-stroke

/// The stroke of a spec expanded to a filled outline (kurbo stroke expansion).
pub fn stroke_outline(s: &ShapeSpec) -> Result<BezPath> {
    let bez = s.bez()?;
    Ok(kurbo::stroke(bez.elements().iter().copied(), &s.kurbo_stroke(), &kurbo::StrokeOpts::default(), FLATTEN_TOL))
}

fn outline_stroke(a: &Args, doc: &mut Document, _: &mut OpCtx) -> Result<Value> {
    let (p, s) = shape_layer(doc, a.raw("layer"))?;
    let layer = doc.layer(&p).clone();
    layer.check_unlocked()?;
    if !s.has_stroke() {
        return Err(Error::new("PX1010", format!("shape layer '{}' has no stroke to outline", layer.name)).with_hint(
            format!(
                "add one first: pixelox shape set <doc> --layer '{}' --stroke black --stroke-width 4 --in-place",
                layer.name
            ),
        ));
    }
    let outline = stroke_outline(&s)?;
    let mut o = ShapeSpec::new(bez_to_d(&outline, PREC), json!({"type": "outline-stroke", "source": s.geometry}));
    o.fill = s.stroke;
    let anchors = anchor_count(&outline);
    if s.fill.is_none() {
        let l = doc.layer_mut(&p);
        if let LayerKind::Shape { spec } = &mut l.content {
            o.write_into(spec);
        }
        return Ok(
            json!({"layer": l.name, "id": l.id, "replaced": true, "anchors": anchors, "bbox": spec::spec_bbox(&o)}),
        );
    }
    // keep the fill on the original; the stroke becomes its own layer above it
    let mut kept = s.clone();
    kept.stroke = None;
    if let LayerKind::Shape { spec } = &mut doc.layer_mut(&p).content {
        kept.write_into(spec);
    }
    let mut l = doc.new_layer(&format!("{} Stroke", layer.name), LayerKind::Shape { spec: o.to_value() });
    l.opacity = layer.opacity;
    l.blend = layer.blend;
    let (n, i) = (l.name.clone(), l.id.clone());
    doc.insert_above(Some(&p), l);
    Ok(
        json!({"layer": n, "id": i, "replaced": false, "fill_layer": layer.name, "anchors": anchors, "bbox": spec::spec_bbox(&o)}),
    )
}
