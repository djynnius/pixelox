//! `comp align`, `comp distribute`, `comp guide *`, `comp grid set`, and `comp presets`
//! (PRD §8.14). Layers are measured with `compose::layer_bounds` (their visible content)
//! and moved with `geom::translate_layer`, so every layer kind moves consistently.

use crate::util::*;
use pixelox_core::doc::{GridSpec, Guide, LayerPath};
use pixelox_core::prelude::*;
use pixelox_core::presets::CANVAS_PRESETS;
use pixelox_core::units::Length;

pub fn ops() -> Vec<OpSpec> {
    vec![
        OpSpec::doc("comp align", Category::Comp, "Align layers to the canvas, selection, or a layer", align)
            .about(
                "Moves each listed layer so its visible bounds line up with the target box: the canvas \
                 (default), the active selection's bounds (`selection`), or another layer's bounds \
                 (`layer:REF`). Give `--x`, `--y`, or both. Every layer kind moves (text anchors, shape paths, \
                 placed rects, group children, masks); locked layers fail with PX2304, empty layers are \
                 skipped. Moves are whole pixels.",
            )
            .param(param("layers", ParamKind::List).required().help("Layers to align (comma-separated references)"))
            .param(param("x", ParamKind::Choice).choices(&["left", "center", "right"]).help("Horizontal alignment: left, center, or right"))
            .param(param("y", ParamKind::Choice).choices(&["top", "middle", "bottom"]).help("Vertical alignment: top, middle, or bottom"))
            .param(param("to", ParamKind::Str).default("canvas").help("Align to: canvas, selection (the active selection's bounds), or layer:REF"))
            .example("pixelox comp align doc.pxo --in-place --layers Logo,Headline --x center", "Center two layers horizontally on the canvas")
            .example("pixelox comp align doc.pxo --in-place --layers Badge --x right --y bottom --to layer:Card", "Put a badge in a card's corner")
            .example("pixelox comp align doc.pxo --in-place --layers Product --x center --y middle --to selection", "Center a layer in the selection"),
        OpSpec::doc("comp distribute", Category::Comp, "Distribute layers with even or fixed spacing", distribute)
            .about(
                "Spaces layers along an axis by their visible bounds. `even` (default) keeps the first and last \
                 layers in place and makes the gaps between neighbours equal; a number keeps the first layer \
                 and sets every gap to that many px (`%` of the canvas). Layers are taken in order of position \
                 along the axis. Moves are whole pixels.",
            )
            .param(param("layers", ParamKind::List).required().help("Layers to distribute (comma-separated, at least two)"))
            .param(param("axis", ParamKind::Choice).choices(&["x", "y"]).default("x").help("Axis to distribute along: x (horizontal) or y (vertical)"))
            .param(param("spacing", ParamKind::Str).default("even").help("Gap between neighbours: even, or a length such as 24 or 2%"))
            .example("pixelox comp distribute doc.pxo --in-place --layers Icon1,Icon2,Icon3 --axis x", "Equal gaps between three icons")
            .example("pixelox comp distribute doc.pxo --in-place --layers Row1,Row2,Row3 --axis y --spacing 24", "Stack rows 24 px apart")
            .example("pixelox comp distribute doc.pxo -o spaced.pxo --layers A,B,C,D --spacing even --json", "Even spacing into a new document"),
        OpSpec::doc("comp guide add", Category::Comp, "Add a vertical or horizontal guide", guide_add)
            .about(
                "Adds a guide: `--v POS` a vertical line at x = POS, `--h POS` a horizontal line at y = POS. \
                 POS is a length (px default, `%` of the canvas width or height, `pt/mm/in`); negative values \
                 count from the right/bottom. Adding an existing guide does nothing.",
            )
            .param(param("v", ParamKind::Length).help("Vertical guide position (x)"))
            .param(param("h", ParamKind::Length).help("Horizontal guide position (y)"))
            .example("pixelox comp guide add doc.pxo --in-place --v 50%", "A vertical guide at the center")
            .example("pixelox comp guide add doc.pxo --in-place --h 120", "A horizontal guide 120 px from the top")
            .example("pixelox comp guide add poster.pxo --in-place --v 10mm", "A margin guide 10 mm from the left"),
        OpSpec::doc("comp guide list", Category::Comp, "List guides and the grid", guide_list)
            .about("Lists the document's guides (with the 1-based index `guide rm --index` takes) and its grid. Read-only.")
            .read_only()
            .example("pixelox comp guide list doc.pxo", "List guides")
            .example("pixelox comp guide list doc.pxo --json", "Guides as JSON")
            .example("pixelox comp guide list template.pxo --json --quiet", "Guides of a template, JSON only"),
        OpSpec::doc("comp guide rm", Category::Comp, "Remove a guide by position or index", guide_rm)
            .about(
                "Removes the guide at a position (`--v POS` / `--h POS`, matched within half a pixel) or by its \
                 1-based index from `comp guide list`. PX1010 when no guide matches.",
            )
            .param(param("v", ParamKind::Length).help("Remove the vertical guide at this position"))
            .param(param("h", ParamKind::Length).help("Remove the horizontal guide at this position"))
            .param(param("index", ParamKind::Int).range("1..1000000").help("Remove the guide with this index (1-based, as listed)"))
            .example("pixelox comp guide rm doc.pxo --in-place --v 50%", "Remove the center guide")
            .example("pixelox comp guide rm doc.pxo --in-place --h 120", "Remove a horizontal guide")
            .example("pixelox comp guide rm doc.pxo --in-place --index 2", "Remove the second guide"),
        OpSpec::doc("comp guide clear", Category::Comp, "Remove all guides", guide_clear)
            .about("Removes every guide (the grid is kept).")
            .example("pixelox comp guide clear doc.pxo --in-place", "Remove all guides")
            .example("pixelox comp guide clear doc.pxo -o clean.pxo", "Remove guides into a new document")
            .example("pixelox comp guide clear doc.pxo -o clean.pxo --dry-run --json", "Plan without writing"),
        OpSpec::doc("comp grid set", Category::Comp, "Set or hide the document grid", grid_set)
            .about(
                "Sets the document grid: `--size N` square cells, or `--size N,N2` for width and height \
                 (lengths; `%` of the canvas). `--off` hides the grid (keeping its size).",
            )
            .param(param("size", ParamKind::Str).help("Cell size: N or N,N2 (lengths; px default)"))
            .param(param("off", ParamKind::Flag).help("Hide the grid"))
            .example("pixelox comp grid set doc.pxo --in-place --size 40", "A 40 px grid")
            .example("pixelox comp grid set doc.pxo --in-place --size 40,20", "40 px wide, 20 px tall cells")
            .example("pixelox comp grid set doc.pxo --in-place --off", "Hide the grid"),
        OpSpec::custom("comp presets", Category::Comp, "List canvas presets", presets)
            .about(
                "Lists the canvas presets `new --preset` accepts: name, pixel size, DPI, and what it is for \
                 (print sizes at 300 DPI, social formats at 72).",
            )
            .inputs(Inputs::NONE)
            .example("pixelox comp presets", "List canvas presets")
            .example("pixelox comp presets --json", "Presets as JSON")
            .example("pixelox comp presets --json --quiet", "Presets as JSON, nothing on stderr"),
    ]
}

// ---------------------------------------------------------------------------------------
// align and distribute

fn target_box(doc: &Document, to: &str) -> Result<RectI> {
    match to.trim() {
        "canvas" => Ok(doc.canvas()),
        "selection" => {
            let m = doc.named_mask("@sel")?;
            m.bbox(0.0).ok_or_else(|| Error::new("PX3320", "the selection covers no pixels"))
        }
        t if t.starts_with("layer:") => {
            let r = &t["layer:".len()..];
            let p = doc.resolve(r)?;
            compose::layer_bounds(doc, doc.layer(&p))?.ok_or_else(|| {
                Error::new("PX3320", format!("layer '{r}' has no visible content to align to")).with_context("layer", r)
            })
        }
        other => Err(Error::new("PX1010", format!("--to: '{other}' is not canvas, selection, or layer:REF"))
            .with_hint("e.g. --to canvas, --to selection, --to layer:Card")),
    }
}

/// Paths of `ids` after checking that none of them (or anything inside them) is locked.
fn movable(doc: &Document, ids: &[String]) -> Result<Vec<LayerPath>> {
    let paths: Vec<LayerPath> = ids.iter().map(|id| path_of(doc, id)).collect::<Result<_>>()?;
    for p in &paths {
        check_tree_unlocked(doc.layer(p))?;
    }
    Ok(paths)
}

fn align(a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    let ids = layers_arg(a, doc)?;
    let (hx, vy) = (a.str("x"), a.str("y"));
    if hx.is_none() && vy.is_none() {
        return Err(Error::new("PX1003", "say how to align: --x left|center|right, --y top|middle|bottom, or both"));
    }
    let t = target_box(doc, &a.str_or("to", "canvas"))?;
    movable(doc, &ids)?;
    let center = |x: i32, w: u32| x as f64 + w as f64 / 2.0;
    let mut moved = Vec::new();
    let mut skipped = Vec::new();
    for id in &ids {
        let p = path_of(doc, id)?;
        let name = doc.layer(&p).name.clone();
        let Some(b) = compose::layer_bounds(doc, doc.layer(&p))? else {
            skipped.push(name);
            continue;
        };
        let dx = match hx.as_deref() {
            Some("left") => (t.x - b.x) as f64,
            Some("center") => (center(t.x, t.w) - center(b.x, b.w)).round(),
            Some("right") => (t.right() - b.right()) as f64,
            _ => 0.0,
        };
        let dy = match vy.as_deref() {
            Some("top") => (t.y - b.y) as f64,
            Some("middle") => (center(t.y, t.h) - center(b.y, b.h)).round(),
            Some("bottom") => (t.bottom() - b.bottom()) as f64,
            _ => 0.0,
        };
        if dx != 0.0 || dy != 0.0 {
            geom::translate_layer(doc, &p, dx, dy)?;
        }
        moved.push(json!({"layer": name, "dx": dx, "dy": dy, "bounds": [b.x + dx as i32, b.y + dy as i32, b.w, b.h]}));
    }
    Ok(json!({"to": t.to_json(), "layers": moved, "skipped": skipped}))
}

fn distribute(a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    let ids = layers_arg(a, doc)?;
    if ids.len() < 2 {
        return Err(Error::new("PX1010", "--layers needs at least two layers to distribute"));
    }
    let horizontal = a.str_or("axis", "x") == "x";
    let spacing = a.str_or("spacing", "even");
    let gap_fixed = if spacing.trim() == "even" {
        None
    } else {
        let l = Length::parse(&spacing).map_err(|_| {
            Error::new("PX1010", format!("--spacing: '{spacing}' is not 'even' or a length"))
                .with_context("param", "spacing")
        })?;
        let extent = if horizontal { doc.width } else { doc.height } as f64;
        Some(l.to_px(extent, doc.dpi()))
    };
    movable(doc, &ids)?;
    // (start, size, id, name) along the axis, in order of position
    let mut items: Vec<(i32, u32, String, String)> = Vec::new();
    let mut skipped = Vec::new();
    for id in &ids {
        let l = doc.layer(&path_of(doc, id)?);
        match compose::layer_bounds(doc, l)? {
            Some(b) => items.push(if horizontal {
                (b.x, b.w, id.clone(), l.name.clone())
            } else {
                (b.y, b.h, id.clone(), l.name.clone())
            }),
            None => skipped.push(l.name.clone()),
        }
    }
    items.sort_by_key(|it| it.0);
    if items.len() < 2 {
        return Ok(json!({"layers": [], "skipped": skipped, "gap": Value::Null}));
    }
    let gap = match gap_fixed {
        Some(g) => g,
        None => {
            let first = items[0].0 as f64;
            let last = items[items.len() - 1].0 as f64 + items[items.len() - 1].1 as f64;
            let total: f64 = items.iter().map(|it| it.1 as f64).sum();
            (last - first - total) / (items.len() - 1) as f64
        }
    };
    let mut pos = items[0].0 as f64;
    let mut out = Vec::new();
    for (start, size, id, name) in &items {
        let d = (pos.round() - *start as f64).round();
        if d != 0.0 {
            let p = path_of(doc, id)?;
            let (dx, dy) = if horizontal { (d, 0.0) } else { (0.0, d) };
            geom::translate_layer(doc, &p, dx, dy)?;
        }
        out.push(json!({"layer": name, "start": *start as f64 + d, "size": size, "moved": d}));
        pos += *size as f64 + gap;
    }
    Ok(json!({"axis": if horizontal { "x" } else { "y" }, "gap": round3(gap), "layers": out, "skipped": skipped}))
}

// ---------------------------------------------------------------------------------------
// guides and grid

fn guides_json(doc: &Document) -> Value {
    Value::Array(
        doc.guides
            .iter()
            .enumerate()
            .map(|(i, g)| json!({"index": i + 1, "orientation": g.orientation, "pos": g.pos}))
            .collect(),
    )
}

/// The one `--v` / `--h` given, resolved to `(orientation, px)`; `index_ok` admits `--index`.
fn guide_pos(a: &Args, doc: &Document, index_ok: bool) -> Result<Option<(&'static str, f64)>> {
    let mut given: Vec<&str> = ["v", "h"].into_iter().filter(|k| a.has(k)).collect();
    if index_ok && a.has("index") {
        given.push("index");
    }
    let flags = if index_ok { "--v POS, --h POS, or --index N" } else { "--v POS or --h POS" };
    if given.is_empty() {
        return Err(Error::new("PX1003", format!("say which guide: {flags}")));
    }
    if given.len() > 1 {
        return Err(Error::new("PX1005", format!("pass only one of {flags}")));
    }
    for (k, extent) in [("v", doc.width), ("h", doc.height)] {
        if let Some(l) = a.length(k)? {
            return Ok(Some((if k == "v" { "v" } else { "h" }, round3(l.resolve_pos(extent as f64, doc.dpi())))));
        }
    }
    Ok(None)
}

fn guide_add(a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    let (orientation, pos) =
        guide_pos(a, doc, false)?.ok_or_else(|| Error::new("PX1003", "say which guide: --v POS or --h POS"))?;
    let exists = doc.guides.iter().any(|g| g.orientation == orientation && (g.pos - pos).abs() < 1e-6);
    if !exists {
        doc.guides.push(Guide { orientation: orientation.into(), pos });
    }
    Ok(json!({"added": !exists, "guide": {"orientation": orientation, "pos": pos}, "guides": guides_json(doc)}))
}

fn guide_list(_a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    Ok(json!({"guides": guides_json(doc), "count": doc.guides.len(), "grid": doc.grid}))
}

fn guide_rm(a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    let removed: Vec<Guide> = match guide_pos(a, doc, true)? {
        Some((o, pos)) => {
            let before = doc.guides.clone();
            doc.guides.retain(|g| !(g.orientation == o && (g.pos - pos).abs() <= 0.5));
            if doc.guides.len() == before.len() {
                return Err(Error::new(
                    "PX1010",
                    format!("no {} guide at {pos}", if o == "v" { "vertical" } else { "horizontal" }),
                )
                .with_hint("list guides with `pixelox comp guide list <doc>`")
                .with_context("guides", guides_json(doc)));
            }
            before.into_iter().filter(|g| !doc.guides.contains(g)).collect()
        }
        None => {
            let n = a.usize("index")?;
            if n == 0 || n > doc.guides.len() {
                return Err(Error::new("PX1013", format!("--index {n} is outside 1..{}", doc.guides.len()))
                    .with_context("guides", guides_json(doc)));
            }
            vec![doc.guides.remove(n - 1)]
        }
    };
    let removed: Vec<Value> = removed.iter().map(|g| json!({"orientation": g.orientation, "pos": g.pos})).collect();
    Ok(json!({"removed": removed, "guides": guides_json(doc)}))
}

fn guide_clear(_a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    let n = doc.guides.len();
    doc.guides.clear();
    Ok(json!({"removed": n}))
}

fn grid_set(a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    let off = a.flag("off");
    match a.str("size") {
        Some(s) => {
            let parts: Vec<&str> = s.split(',').map(str::trim).collect();
            if parts.is_empty() || parts.len() > 2 || parts.iter().any(|p| p.is_empty()) {
                return Err(
                    Error::new("PX1010", format!("--size: '{s}' is not N or N,N2")).with_context("param", "size")
                );
            }
            let w = Length::parse(parts[0])?.to_px(doc.width as f64, doc.dpi());
            let h = match parts.get(1) {
                Some(p) => Length::parse(p)?.to_px(doc.height as f64, doc.dpi()),
                None if parts[0].ends_with('%') => Length::parse(parts[0])?.to_px(doc.height as f64, doc.dpi()),
                None => w,
            };
            if !(w > 0.0 && h > 0.0) {
                return Err(Error::new("PX1013", format!("--size: grid cells must be positive, got {w}x{h}"))
                    .with_context("param", "size"));
            }
            doc.grid = Some(GridSpec { size: [round3(w), round3(h)], visible: !off });
        }
        None if off => {
            if let Some(g) = &mut doc.grid {
                g.visible = false;
            }
        }
        None => return Err(Error::new("PX1003", "give --size N[,N2], or --off to hide the grid")),
    }
    Ok(json!({"grid": doc.grid}))
}

fn presets(_a: &Args, _ctx: &mut OpCtx) -> Result<Value> {
    let list: Vec<Value> = CANVAS_PRESETS
        .iter()
        .map(|(name, w, h, dpi, about)| json!({"name": name, "size": [w, h], "dpi": dpi, "description": about}))
        .collect();
    Ok(json!({"presets": list, "count": CANVAS_PRESETS.len()}))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testio::run_custom;
    use pixelox_core::ctx::Globals;
    use pixelox_core::testing::{self, find, run};

    fn op(name: &str) -> OpSpec {
        find(&ops(), name)
    }

    fn block(w: u32, h: u32) -> Image {
        Image::filled(w, h, Rgba::BLACK)
    }

    /// 100×50 canvas; A: 10×10 at (5,5), B: 20×10 at (40,20), C: 10×20 at (70,10).
    fn doc() -> Document {
        let mut d =
            testing::doc_layers(100, 50, vec![("A", block(10, 10)), ("B", block(20, 10)), ("C", block(10, 20))]);
        for (n, x, y) in [("A", 5, 5), ("B", 40, 20), ("C", 70, 10)] {
            let p = d.resolve(n).unwrap();
            d.layer_mut(&p).x = x;
            d.layer_mut(&p).y = y;
        }
        d
    }

    fn bounds(d: &Document, n: &str) -> RectI {
        compose::layer_bounds(d, d.layer(&d.resolve(n).unwrap())).unwrap().unwrap()
    }

    #[test]
    fn align_to_the_canvas() {
        let mut d = doc();
        run(&op("comp align"), &[("layers", "A,B,C"), ("x", "left")], &mut d).unwrap();
        assert_eq!([bounds(&d, "A").x, bounds(&d, "B").x, bounds(&d, "C").x], [0, 0, 0]);
        assert_eq!(bounds(&d, "B").y, 20, "y unchanged without --y");
        run(&op("comp align"), &[("layers", "A,B"), ("x", "right"), ("y", "bottom")], &mut d).unwrap();
        assert_eq!(bounds(&d, "A"), RectI::new(90, 40, 10, 10));
        assert_eq!(bounds(&d, "B"), RectI::new(80, 40, 20, 10));
        run(&op("comp align"), &[("layers", "B,C"), ("x", "center"), ("y", "middle")], &mut d).unwrap();
        assert_eq!(bounds(&d, "B"), RectI::new(40, 20, 20, 10));
        assert_eq!(bounds(&d, "C"), RectI::new(45, 15, 10, 20));
        run(&op("comp align"), &[("layers", "A"), ("y", "top")], &mut d).unwrap();
        assert_eq!(bounds(&d, "A").y, 0);
    }

    #[test]
    fn align_to_a_layer_and_the_selection() {
        let mut d = doc();
        run(&op("comp align"), &[("layers", "A,C"), ("x", "right"), ("y", "top"), ("to", "layer:B")], &mut d).unwrap();
        assert_eq!(bounds(&d, "A"), RectI::new(50, 20, 10, 10));
        assert_eq!(bounds(&d, "C"), RectI::new(50, 20, 10, 20));
        assert_eq!(bounds(&d, "B"), RectI::new(40, 20, 20, 10));
        d.selection =
            Some(Mask::from_fn(100, 50, |x, y| if (60..80).contains(&x) && (30..50).contains(&y) { 1.0 } else { 0.0 }));
        run(&op("comp align"), &[("layers", "A"), ("x", "center"), ("y", "middle"), ("to", "selection")], &mut d)
            .unwrap();
        assert_eq!(bounds(&d, "A"), RectI::new(65, 35, 10, 10));
    }

    #[test]
    fn align_errors_locks_and_live_layers() {
        let mut d = doc();
        assert_eq!(run(&op("comp align"), &[("layers", "A")], &mut d).unwrap_err().code, "PX1003");
        assert_eq!(
            run(&op("comp align"), &[("layers", "A"), ("x", "left"), ("to", "page")], &mut d).unwrap_err().code,
            "PX1010"
        );
        assert_eq!(
            run(&op("comp align"), &[("layers", "A"), ("x", "left"), ("to", "selection")], &mut d).unwrap_err().code,
            "PX2312"
        );
        assert_eq!(run(&op("comp align"), &[("layers", "A,Z"), ("x", "left")], &mut d).unwrap_err().code, "PX2301");
        let p = d.resolve("B").unwrap();
        d.layer_mut(&p).locked = true;
        assert_eq!(run(&op("comp align"), &[("layers", "A,B"), ("x", "left")], &mut d).unwrap_err().code, "PX2304");
        assert_eq!(bounds(&d, "A").x, 5, "nothing moved");
        // placed layers move by their rect; empty layers are skipped
        let l = d.new_layer(
            "P",
            LayerKind::Placed {
                asset: "k".into(),
                path: "p.png".into(),
                linked: false,
                rect: [10.0, 10.0, 8.0, 8.0],
                angle: 0.0,
                source: Some(std::sync::Arc::new(block(2, 2))),
            },
        );
        d.insert_above(None, l);
        let e = d.new_layer("Empty", LayerKind::Raster { image: Image::new(4, 4) });
        d.insert_above(None, e);
        let v = run(&op("comp align"), &[("layers", "P,Empty"), ("x", "right")], &mut d).unwrap();
        assert_eq!(v["skipped"], json!(["Empty"]));
        let LayerKind::Placed { rect, .. } = &d.layer(&d.resolve("P").unwrap()).content else { panic!() };
        assert_eq!(rect[0], 92.0);
    }

    #[test]
    fn distribute_even_and_fixed() {
        let mut d = doc();
        // x: A [5,15), B [40,60), C [70,80): span 75, sizes 40 → gap 17.5
        let v = run(&op("comp distribute"), &[("layers", "C,A,B")], &mut d).unwrap();
        assert_eq!(v["gap"], 17.5);
        assert_eq!(bounds(&d, "A").x, 5);
        assert_eq!(bounds(&d, "B").x, 33);
        assert_eq!(bounds(&d, "C").x, 70);
        // y: A [5,15), C [10,30), B [20,30) → fixed 4 px gaps from the first
        run(&op("comp distribute"), &[("layers", "A,B,C"), ("axis", "y"), ("spacing", "4")], &mut d).unwrap();
        assert_eq!([bounds(&d, "A").y, bounds(&d, "C").y, bounds(&d, "B").y], [5, 19, 43]);
        assert_eq!(bounds(&d, "A").x, 5, "x untouched");
        assert_eq!(run(&op("comp distribute"), &[("layers", "A")], &mut d).unwrap_err().code, "PX1010");
        assert_eq!(
            run(&op("comp distribute"), &[("layers", "A,B"), ("spacing", "wide")], &mut d).unwrap_err().code,
            "PX1010"
        );
    }

    #[test]
    fn guides_add_list_rm_clear() {
        let mut d = doc();
        run(&op("comp guide add"), &[("v", "50%")], &mut d).unwrap();
        run(&op("comp guide add"), &[("h", "10")], &mut d).unwrap();
        let v = run(&op("comp guide add"), &[("v", "-10")], &mut d).unwrap();
        assert_eq!(v["guide"]["pos"], 90.0);
        let v = run(&op("comp guide add"), &[("v", "50")], &mut d).unwrap();
        assert_eq!(v["added"], false);
        let v = run(&op("comp guide list"), &[], &mut d).unwrap();
        assert_eq!(v["count"], 3);
        assert_eq!(v["guides"][1], json!({"index": 2, "orientation": "h", "pos": 10.0}));
        assert_eq!(run(&op("comp guide add"), &[("v", "1"), ("h", "1")], &mut d).unwrap_err().code, "PX1005");
        assert_eq!(run(&op("comp guide add"), &[], &mut d).unwrap_err().code, "PX1003");
        run(&op("comp guide rm"), &[("v", "50.3")], &mut d).unwrap();
        assert_eq!(d.guides.len(), 2);
        assert_eq!(run(&op("comp guide rm"), &[("h", "50")], &mut d).unwrap_err().code, "PX1010");
        let v = run(&op("comp guide rm"), &[("index", "2")], &mut d).unwrap();
        assert_eq!(v["removed"][0]["pos"], 90.0);
        assert_eq!(run(&op("comp guide rm"), &[("index", "5")], &mut d).unwrap_err().code, "PX1013");
        let v = run(&op("comp guide clear"), &[], &mut d).unwrap();
        assert_eq!(v["removed"], 1);
        assert!(d.guides.is_empty());
    }

    #[test]
    fn grid_set_and_off() {
        let mut d = doc();
        run(&op("comp grid set"), &[("size", "40")], &mut d).unwrap();
        assert_eq!(d.grid, Some(GridSpec { size: [40.0, 40.0], visible: true }));
        run(&op("comp grid set"), &[("size", "10%,20")], &mut d).unwrap();
        assert_eq!(d.grid.as_ref().unwrap().size, [10.0, 20.0]);
        run(&op("comp grid set"), &[("off", "true")], &mut d).unwrap();
        assert!(!d.grid.as_ref().unwrap().visible);
        assert_eq!(run(&op("comp grid set"), &[], &mut d).unwrap_err().code, "PX1003");
        assert_eq!(run(&op("comp grid set"), &[("size", "0")], &mut d).unwrap_err().code, "PX1013");
        assert_eq!(run(&op("comp grid set"), &[("size", "1,2,3")], &mut d).unwrap_err().code, "PX1010");
    }

    #[test]
    fn presets_list_every_canvas_preset() {
        let spec = op("comp presets");
        assert_eq!(spec.inputs, Inputs::NONE);
        let (v, _) = run_custom(&spec, &[], vec![], Globals::default(), &pixelox_core::ctx::NoIo).unwrap();
        assert_eq!(v["count"], CANVAS_PRESETS.len());
        assert!(v["presets"]
            .as_array()
            .unwrap()
            .iter()
            .any(|p| p["name"] == "ig-story" && p["size"] == json!([1080, 1920])));
    }
}
