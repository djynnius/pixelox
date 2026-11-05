//! `layer *` (PRD §8.3): list, add, duplicate, remove, rename, restack, group, merge,
//! flatten, and the per-layer properties (opacity, blend, visibility, lock, clipping), plus
//! rasterizing live layers.

use crate::util::*;
use pixelox_core::blend::BLEND_MODES;
use pixelox_core::doc::LayerPath;
use pixelox_core::prelude::*;

fn layer_p() -> pixelox_core::ParamSpec {
    common::layer()
}

pub fn ops() -> Vec<OpSpec> {
    vec![
        OpSpec::doc("layer list", Category::Layer, "List layers with kind, blend, opacity, mask, fx, bounds", list)
            .about(
                "Lists every layer in panel order (top first) with its id, name, kind, visibility, lock, opacity, \
                 blend mode, clipping, mask, styles, and bounds on the canvas. The default is a flat list where \
                 `depth` and `parent` show nesting and `index` is the `#N` reference; `--tree` nests group \
                 children instead. Read-only.",
            )
            .param(param("tree", ParamKind::Flag).help("Nest group children under their group instead of a flat list"))
            .read_only()
            .example("pixelox layer list doc.pxo", "List the layers of a document")
            .example("pixelox layer list doc.pxo --tree --json", "The layer tree as JSON, groups nested")
            .example("pixelox layer list brand.psd --json", "Inspect the layers imported from a PSD"),
        OpSpec::doc("layer add", Category::Layer, "Add an empty or solid-filled raster layer", add)
            .about(
                "Adds a raster layer above the active layer (or at the top when none is active) and makes it \
                 active. By default it is canvas-sized and transparent; `--size` and `--at` place a smaller \
                 layer (`%` of the canvas; negative `--at` values count from the right/bottom) and `--fill` \
                 paints it a solid color. An explicit `--name` must be unused (PX2305).",
            )
            .param(param("name", ParamKind::Str).help("Name for the new layer (default: 'Layer', made unique)"))
            .param(
                param("size", ParamKind::Size)
                    .help("Layer size WxH in px or % of the canvas (default: the canvas size)"),
            )
            .param(
                param("at", ParamKind::Point).help(
                    "Top-left corner X,Y on the canvas (default 0,0); negative values count from the right/bottom",
                ),
            )
            .param(param("fill", ParamKind::Color).help("Fill color (default: transparent)"))
            .example("pixelox layer add doc.pxo --in-place --name Paint", "Add a transparent canvas-sized layer")
            .example(
                "pixelox layer add doc.pxo --in-place --name Band --size 100%x200 --at 0,-200 --fill '#101014'",
                "A dark band across the bottom 200 px",
            )
            .example(
                "pixelox layer add photo.jpg -o layered.pxo --name Dim --fill '#0000004d'",
                "Darken a photo with a translucent layer",
            ),
        OpSpec::doc("layer dup", Category::Layer, "Duplicate a layer with its mask and styles", dup)
            .about(
                "Copies a layer (with its mask, styles, and for groups every child) directly above the original \
                 and makes the copy active. Copies get fresh ids and unique names ('Logo copy'); an explicit \
                 `--name` must be unused (PX2305).",
            )
            .param(layer_p())
            .param(param("name", ParamKind::Str).help("Name for the copy (default: '<name> copy', made unique)"))
            .example("pixelox layer dup doc.pxo --in-place --layer Logo", "Duplicate a layer")
            .example(
                "pixelox layer dup doc.pxo --in-place --layer Logo --name 'Logo shadow'",
                "Duplicate under a chosen name",
            )
            .example(
                "pixelox layer dup doc.pxo -o variant.pxo --layer @top --json",
                "Duplicate the top layer into a new document",
            ),
        OpSpec::doc("layer rm", Category::Layer, "Remove a layer", rm)
            .about(
                "Removes a layer (a group with everything inside it) from the document. Locked layers are kept \
                 (PX2304). Only the document changes; no file is ever deleted.",
            )
            .param(layer_p())
            .example("pixelox layer rm brand.pxo --in-place --layer Headline", "Remove a layer by name")
            .example("pixelox layer rm doc.pxo --in-place --layer '#1'", "Remove the top layer of the panel")
            .example(
                "pixelox layer rm doc.pxo -o trimmed.pxo --layer @top --dry-run --json",
                "Plan a removal without writing",
            ),
        OpSpec::doc("layer rename", Category::Layer, "Rename a layer (names stay unique)", rename)
            .about(
                "Renames a layer. Names are unique within a document (PX2305 when taken) and cannot look like \
                 references (`@top`, `#2`) or contain commas. Template slots that target the layer by name are \
                 updated to follow it.",
            )
            .param(layer_p())
            .param(param("to", ParamKind::Str).required().help("The new name"))
            .example("pixelox layer rename doc.pxo --in-place --layer 'Layer 2' --to Sky", "Rename a layer")
            .example("pixelox layer rename doc.pxo --in-place --layer @top --to Headline", "Rename the top layer")
            .example("pixelox layer rename doc.pxo --in-place --layer L7 --to Logo --json", "Rename by layer id"),
        OpSpec::doc("layer reorder", Category::Layer, "Move a layer above/below another or to an index", reorder)
            .about(
                "Restacks a layer: directly above or below another layer (which may move it into or out of a \
                 group), or to a position among its siblings with `--index` (1 = top). Give exactly one of \
                 `--above`, `--below`, `--index`.",
            )
            .param(layer_p())
            .param(param("above", ParamKind::Layer).help("Place the layer directly above this layer"))
            .param(param("below", ParamKind::Layer).help("Place the layer directly below this layer"))
            .param(param("index", ParamKind::Int).range("1..1000000").help("Position among its siblings, 1 = top"))
            .example(
                "pixelox layer reorder doc.pxo --in-place --layer Logo --above Headline",
                "Move a layer above another",
            )
            .example(
                "pixelox layer reorder doc.pxo --in-place --layer Shadow --below Logo",
                "Move a layer below another",
            )
            .example(
                "pixelox layer reorder doc.pxo --in-place --layer Sky --index 2",
                "Second from the top of its stack",
            ),
        OpSpec::doc("layer group", Category::Layer, "Group sibling layers into a pass-through group", group)
            .about(
                "Moves sibling layers (same parent) into a new group placed where the topmost of them was, \
                 keeping their order. Groups blend `pass-through` by default, so the composite is unchanged.",
            )
            .param(param("layers", ParamKind::List).required().help("Layers to group (comma-separated references)"))
            .param(param("name", ParamKind::Str).help("Name for the group (default: 'Group', made unique)"))
            .example("pixelox layer group doc.pxo --in-place --layers Logo,Tagline --name Brand", "Group two layers")
            .example("pixelox layer group doc.pxo --in-place --layers '#1,#2,#3'", "Group the top three layers")
            .example(
                "pixelox layer group doc.pxo -o grouped.pxo --layers Sky,Clouds --name Backdrop --json",
                "Group into a new document",
            ),
        OpSpec::doc("layer ungroup", Category::Layer, "Dissolve a group, keeping its children in place", ungroup)
            .about(
                "Replaces a group with its children, in order, at the group's position. The group's own opacity, \
                 blend mode, mask, and styles are dropped; the result lists what was discarded.",
            )
            .param(layer_p())
            .example("pixelox layer ungroup doc.pxo --in-place --layer Brand", "Ungroup a group")
            .example("pixelox layer ungroup doc.pxo --in-place --layer @top", "Ungroup the top group")
            .example(
                "pixelox layer ungroup doc.pxo -o loose.pxo --layer Backdrop --json",
                "Ungroup into a new document",
            ),
        OpSpec::doc("layer merge", Category::Layer, "Merge layers into one raster layer (bakes styles)", merge)
            .about(
                "Bakes layers into one raster layer: their content, styles, masks, opacity, and blend modes are \
                 composited together (as if only they were visible) and the result replaces them at the \
                 position of the topmost one. `--down --layer REF` merges a layer into the one below it, \
                 keeping the lower layer's name. A single group can be merged on its own. Locked layers are \
                 kept (PX2304). Content outside the canvas is dropped.",
            )
            .param(param("layers", ParamKind::List).help("Layers to merge (comma-separated references)"))
            .param(param("down", ParamKind::Flag).help("Merge --layer into the layer directly below it"))
            .param(layer_p())
            .param(
                param("name", ParamKind::Str)
                    .help("Name for the merged layer (default: the top layer's name; with --down, the lower one's)"),
            )
            .example("pixelox layer merge doc.pxo --in-place --layers Logo,Tagline --name Lockup", "Merge two layers")
            .example("pixelox layer merge doc.pxo --in-place --down --layer Shadow", "Merge a layer into the one below")
            .example("pixelox layer merge doc.pxo --in-place --layers Brand", "Bake a group into one raster layer"),
        OpSpec::doc("layer flatten", Category::Layer, "Flatten the document to one Background layer", flatten)
            .about(
                "Composites every visible layer into a single canvas-sized raster layer named `Background`. The \
                 named-mask library, the selection, guides, and grid are kept; template slots are removed \
                 because the layers they target are gone.",
            )
            .example("pixelox layer flatten doc.pxo --in-place", "Flatten a document")
            .example("pixelox layer flatten doc.pxo -o flat.pxo", "Flatten into a new document")
            .example("pixelox layer flatten doc.pxo -o flat.pxo --dry-run --json", "Plan a flatten without writing"),
        OpSpec::doc("layer opacity", Category::Layer, "Set a layer's opacity (0-100)", opacity)
            .about("Sets a layer's opacity from 0 (invisible) to 100 (opaque). Locked layers are kept (PX2304).")
            .param(layer_p())
            .param(param("value", ParamKind::Float).required().range("0..100").help("Opacity in percent, 0..100"))
            .example("pixelox layer opacity doc.pxo --in-place --layer Watermark --value 85", "Fade a watermark")
            .example("pixelox layer opacity doc.pxo --in-place --layer @top --value 50", "Half-transparent top layer")
            .example("pixelox layer opacity photo.jpg -o faded.pxo --value 40", "Fade the only layer of a photo"),
        OpSpec::doc("layer blend", Category::Layer, "Set a layer's blend mode", blend)
            .about(
                "Sets a layer's blend mode: any of Photoshop's 27 modes, or `pass-through` (groups only, \
                 PX1010 otherwise). Locked layers are kept (PX2304).",
            )
            .param(layer_p())
            .param(param("mode", ParamKind::Choice).required().choices(BLEND_MODES).help("Blend mode name"))
            .example("pixelox layer blend doc.pxo --in-place --layer Watermark --mode multiply", "Multiply a watermark")
            .example("pixelox layer blend doc.pxo --in-place --layer Glow --mode screen", "Screen a glow layer")
            .example(
                "pixelox layer blend doc.pxo --in-place --layer Brand --mode pass-through",
                "Make a group pass-through",
            ),
        OpSpec::doc("layer show", Category::Layer, "Make a layer visible", show)
            .about("Makes a layer visible. Allowed on locked layers.")
            .param(layer_p())
            .example("pixelox layer show doc.pxo --in-place --layer Watermark", "Show a layer")
            .example("pixelox layer show doc.pxo --in-place --layer '#2'", "Show the second layer from the top")
            .example("pixelox layer show doc.pxo -o shown.pxo --layer Grid --json", "Show a layer into a new document"),
        OpSpec::doc("layer hide", Category::Layer, "Hide a layer", hide)
            .about(
                "Hides a layer: it stays in the document but is left out of the composite. Allowed on locked layers.",
            )
            .param(layer_p())
            .example("pixelox layer hide doc.pxo --in-place --layer Watermark", "Hide a layer")
            .example("pixelox layer hide doc.pxo --in-place --layer @top", "Hide the top layer")
            .example(
                "pixelox layer hide doc.pxo -o draft.pxo --layer Notes --json",
                "Hide a layer into a new document",
            ),
        OpSpec::doc("layer lock", Category::Layer, "Lock a layer against edits", lock)
            .about(
                "Locks a layer: commands that change its pixels, geometry, opacity, blend mode, or clipping fail \
                 with PX2304, and it cannot be removed or merged. Renaming, visibility, and restacking still work.",
            )
            .param(layer_p())
            .example("pixelox layer lock doc.pxo --in-place --layer Background", "Lock the background")
            .example("pixelox layer lock doc.pxo --in-place --layer Logo", "Protect a logo from edits")
            .example("pixelox layer lock template.pxo --in-place --layer '#1' --json", "Lock the top layer"),
        OpSpec::doc("layer unlock", Category::Layer, "Unlock a layer", unlock)
            .about("Unlocks a layer so commands may change it again.")
            .param(layer_p())
            .example("pixelox layer unlock doc.pxo --in-place --layer Background", "Unlock the background")
            .example("pixelox layer unlock doc.pxo --in-place --layer Logo", "Unlock a layer")
            .example("pixelox layer unlock doc.pxo -o editable.pxo --layer @top --json", "Unlock into a new document"),
        OpSpec::doc("layer clip", Category::Layer, "Clip a layer to the layer below (or release it)", clip)
            .about(
                "Turns the layer into a clipping mask member: it only shows where the nearest non-clipped layer \
                 below it has coverage (Photoshop semantics). `--off` releases it. The bottom layer of a stack \
                 has nothing to clip to (PX1010).",
            )
            .param(layer_p())
            .param(param("off", ParamKind::Flag).help("Release the clipping mask instead of creating it"))
            .example("pixelox layer clip doc.pxo --in-place --layer Texture", "Clip a texture to the layer below")
            .example("pixelox layer clip doc.pxo --in-place --layer Texture --off", "Release the clipping mask")
            .example("pixelox layer clip doc.pxo -o clipped.pxo --layer @top --json", "Clip the top layer"),
        OpSpec::doc("layer rasterize", Category::Layer, "Convert a text, shape, or placed layer to pixels", rasterize)
            .about(
                "Renders a text, shape, or placed layer to a raster layer at the same place. Layer styles stay \
                 live (they are not baked; use `layer merge` to bake them); the mask, opacity, and blend mode \
                 are kept, and the original spec is kept in the layer's properties. Raster layers are left \
                 unchanged; groups and adjustment layers have no single content to rasterize (PX2303).",
            )
            .param(layer_p())
            .example("pixelox layer rasterize doc.pxo --in-place --layer Headline", "Rasterize a text layer")
            .example("pixelox layer rasterize doc.pxo --in-place --layer Logo", "Rasterize a placed logo")
            .example(
                "pixelox layer rasterize doc.pxo -o raster.pxo --layer Badge --json",
                "Rasterize a shape into a new document",
            ),
        OpSpec::doc("layer blend-if", Category::Layer, "Show a layer only where tones fall in ranges", blend_if)
            .about(
                "Photoshop's Blend If (Blending Options): the layer shows only where its own values (--this) \
                 and the composite beneath it (--underlying) fall inside the ranges. A range is `B,W` or the \
                 split form `B1,B2,W1,W2` on 0..255; a split pair ramps softly instead of cutting hard (like \
                 Alt-dragging the slider halves). --channel picks gray (luma), red, green, or blue. Works on \
                 every layer kind; a pass-through group with Blend If composites isolated. `--off` removes it.",
            )
            .param(layer_p())
            .param(
                param("this", ParamKind::Str)
                    .default("0,255")
                    .help("This layer's visible range: B,W or B1,B2,W1,W2 on 0..255"),
            )
            .param(
                param("underlying", ParamKind::Str)
                    .default("0,255")
                    .help("The underlying composite's range: B,W or B1,B2,W1,W2 on 0..255"),
            )
            .param(
                param("channel", ParamKind::Choice)
                    .choices(&["gray", "red", "green", "blue"])
                    .default("gray")
                    .help("Channel the ranges read: gray (luma), red, green, or blue"),
            )
            .param(param("off", ParamKind::Flag).help("Remove Blend If from the layer"))
            .example(
                "pixelox layer blend-if doc.pxo --in-place --layer Clouds --this 30,90,255,255",
                "Drop a texture's dark parts with a soft ramp",
            )
            .example(
                "pixelox layer blend-if doc.pxo --in-place --layer Shadows --underlying 0,0,110,190",
                "Paint shadows only over darker areas below",
            )
            .example("pixelox layer blend-if doc.pxo --in-place --layer Clouds --off", "Remove Blend If"),
        OpSpec::doc("layer stamp", Category::Layer, "Merge all visible layers into a new top layer", stamp)
            .about(
                "Photoshop's Stamp Visible (Ctrl+Alt+Shift+E): composites every visible layer into one new raster \
                 layer and inserts it on top (or above --above), leaving the layers underneath untouched, so \
                 finishing filters and grades can work on the whole image non-destructively.",
            )
            .param(param("name", ParamKind::Str).default("Stamp").help("Name of the new layer (made unique)"))
            .param(param("above", ParamKind::Layer).help("Insert above this layer instead of on top"))
            .example("pixelox layer stamp doc.pxo --in-place", "Stamp everything visible onto a new top layer")
            .example("pixelox layer stamp doc.pxo --in-place --name Final --above Grade", "Stamp above a named layer")
            .example("pixelox layer stamp doc.pxo -o stamped.pxo --json", "Stamp into a new document"),
        OpSpec::doc("layer smart-filter", Category::Layer, "Disable, enable, or remove a smart filter", smart_filter)
            .about(
                "Manages the smart filters of a placed layer (`filter` and `adjust` commands run on a placed \
                 layer are recorded as smart filters and re-run whenever it renders). --index counts from 1 in \
                 the order they were added (`layer list` shows them); the default is the last one.",
            )
            .param(layer_p())
            .param(
                param("action", ParamKind::Choice)
                    .choices(&["disable", "enable", "remove"])
                    .required()
                    .help("What to do with the filter: disable, enable, or remove"),
            )
            .param(param("index", ParamKind::Int).range("1..").help("Which filter, from 1 (default: the last)"))
            .example(
                "pixelox layer smart-filter doc.pxo --in-place --layer Photo --action disable",
                "Turn off the last smart filter",
            )
            .example(
                "pixelox layer smart-filter doc.pxo --in-place --layer Photo --action remove --index 1",
                "Remove the first smart filter",
            )
            .example(
                "pixelox layer smart-filter doc.pxo --in-place --layer Photo --action enable --index 2",
                "Turn the second one back on",
            ),
    ]
}

fn smart_filter(a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    let p = layer_path(a, doc)?;
    let l = doc.layer(&p);
    l.check_unlocked()?;
    let n = l.smart_filters.len();
    if n == 0 {
        return Err(Error::new("PX1010", format!("layer '{}' has no smart filters", l.name))
            .with_hint("run a filter or adjust command on a placed layer to add one; `pixelox layer list` shows them")
            .with_context("layer", l.name.clone()));
    }
    let i = a.opt_int("index")?.map(|v| v.max(0) as usize).unwrap_or(n);
    if i == 0 || i > n {
        return Err(Error::new(
            "PX1013",
            format!("--index {i} is out of range; layer '{}' has {n} smart filters", l.name),
        )
        .with_context("count", n));
    }
    let action = a.choice("action");
    let l = doc.layer_mut(&p);
    let op = l.smart_filters[i - 1].op.clone();
    match action.as_str() {
        "remove" => {
            l.smart_filters.remove(i - 1);
        }
        "enable" => l.smart_filters[i - 1].enabled = true,
        _ => l.smart_filters[i - 1].enabled = false,
    }
    let list: Vec<Value> = l.smart_filters.iter().map(|f| json!({"op": f.op, "enabled": f.enabled})).collect();
    Ok(json!({"layer": l.name, "action": action, "index": i, "op": op, "smart_filters": list}))
}

/// A Blend If range: `B,W` or `B1,B2,W1,W2` on 0..255, non-decreasing.
fn parse_blend_range(name: &str, s: &str) -> Result<[f32; 4]> {
    let bad = |why: &str| {
        Error::new("PX1010", format!("--{name} '{s}': {why}"))
            .with_hint(format!("use B,W or B1,B2,W1,W2 on 0..255, e.g. --{name} 0,0,120,200"))
            .with_context(name, s)
    };
    let v: Vec<f32> = s
        .split(',')
        .map(|t| t.trim().parse::<f32>())
        .collect::<std::result::Result<_, _>>()
        .map_err(|_| bad("not a list of numbers"))?;
    let r = match v.as_slice() {
        [b, w] => [*b, *b, *w, *w],
        [a, b, c, d] => [*a, *b, *c, *d],
        _ => return Err(bad("expected 2 or 4 numbers")),
    };
    if r.iter().any(|x| !(0.0..=255.0).contains(x)) {
        return Err(Error::new("PX1013", format!("--{name} '{s}': values must be within 0..255")).with_context(name, s));
    }
    if r.windows(2).any(|w| w[0] > w[1]) {
        return Err(bad("values must not decrease (black range first, then white)"));
    }
    Ok(r)
}

fn blend_if(a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    let p = layer_path(a, doc)?;
    doc.layer(&p).check_unlocked()?;
    if a.flag("off") {
        for k in ["this", "underlying"] {
            if a.is_explicit(k) {
                return Err(Error::new("PX1005", format!("--off removes Blend If; drop --{k}")));
            }
        }
        let l = doc.layer_mut(&p);
        let had = l.blend_if.take().is_some();
        return Ok(json!({"layer": l.name, "blend_if": null, "removed": had}));
    }
    let bi = BlendIf {
        channel: a.choice("channel"),
        this: parse_blend_range("this", &a.str("this").unwrap_or_default())?,
        underlying: parse_blend_range("underlying", &a.str("underlying").unwrap_or_default())?,
    };
    let l = doc.layer_mut(&p);
    let v = json!({"layer": l.name, "blend_if": bi, "identity": bi.is_identity()});
    l.blend_if = if bi.is_identity() { None } else { Some(bi) };
    Ok(v)
}

fn stamp(a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    let above = match a.raw("above") {
        Some(r) => Some(doc.resolve(r)?),
        None => None,
    };
    let full = compose::composite(doc)?;
    let (image, x, y) = match full.alpha_bbox(0.0) {
        Some(r) if (r.w, r.h) != (full.width, full.height) => (full.crop(r), r.x, r.y),
        Some(_) => (full, 0, 0),
        None => (Image::new(doc.width, doc.height), 0, 0),
    };
    let name = a.str("name").unwrap_or_else(|| "Stamp".into());
    let mut l = doc.new_layer(&name, LayerKind::Raster { image });
    l.x = x;
    l.y = y;
    let (id, nm) = (l.id.clone(), l.name.clone());
    doc.insert_above(above.as_deref(), l);
    let bounds = doc.find_by_id(&id).and_then(|p| compose::layer_bounds(doc, doc.layer(&p)).ok().flatten());
    Ok(json!({"layer": nm, "id": id, "bounds": bounds.map(|b| b.to_json())}))
}

// ---------------------------------------------------------------------------------------
// inspection

fn list(a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    let doc: &Document = doc;
    let active = doc.active.as_deref().and_then(|id| doc.find_by_id(id)).map(|p| doc.layer(&p).name.clone());
    let layers = if a.flag("tree") {
        doc.tree_json()
    } else {
        let walk = doc.walk();
        Value::Array(
            walk.iter()
                .enumerate()
                .map(|(i, (p, l))| {
                    let mut v = doc.layer_json(l, p.len() - 1);
                    if let Some(o) = v.as_object_mut() {
                        if let Some(Value::Array(ch)) = o.remove("children") {
                            o.insert("child_count".into(), json!(ch.len()));
                        }
                        o.insert("index".into(), json!(i + 1));
                        o.insert("parent".into(), parent_name(doc, p));
                    }
                    v
                })
                .collect(),
        )
    };
    Ok(json!({"layers": layers, "count": doc.layer_count(), "active": active}))
}

// ---------------------------------------------------------------------------------------
// creating and removing

fn add(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let (cw, ch, dpi) = (doc.width as f64, doc.height as f64, doc.dpi());
    let (w, h) = match a.size("size")? {
        Some(s) => s.resolve(cw, ch, dpi),
        None => (doc.width, doc.height),
    };
    check_pixels(ctx, w, h)?;
    let (x, y) = match a.point("at")? {
        Some(p) => {
            let (x, y) = p.resolve(cw, ch, dpi);
            (x.round() as i32, y.round() as i32)
        }
        None => (0, 0),
    };
    let fill = a.opt_color("fill")?.unwrap_or(Rgba::TRANSPARENT);
    let name = match a.str("name") {
        Some(n) => fresh_name(doc, &n, &[])?,
        None => doc.unique_name("Layer"),
    };
    let mut l = doc.new_layer(&name, LayerKind::Raster { image: Image::filled(w, h, fill) });
    l.x = x;
    l.y = y;
    let id = l.id.clone();
    let p = insert_new(doc, l);
    Ok(
        json!({"layer": name, "id": id, "size": [w, h], "offset": [x, y], "fill": fill.to_hex(), "parent": parent_name(doc, &p)}),
    )
}

/// Give a copied layer (and everything inside it) fresh ids and unique names.
fn fresh_copy(doc: &mut Document, l: &mut Layer, taken: &mut Vec<String>, name: Option<String>) {
    l.id = doc.next_layer_id();
    l.name = match name {
        Some(n) => n,
        None => unique_avoiding(doc, &format!("{} copy", l.name), taken),
    };
    taken.push(l.name.clone());
    if let Some(ch) = l.children_mut() {
        for c in ch.iter_mut() {
            fresh_copy(doc, c, taken, None);
        }
    }
}

fn dup(a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    let p = layer_path(a, doc)?;
    let explicit = match a.str("name") {
        Some(n) => Some(fresh_name(doc, &n, &[])?),
        None => None,
    };
    let mut copy = doc.layer(&p).clone();
    let source = copy.name.clone();
    let mut taken: Vec<String> = explicit.iter().cloned().collect();
    fresh_copy(doc, &mut copy, &mut taken, explicit);
    let (name, id) = (copy.name.clone(), copy.id.clone());
    let np = doc.insert_above(Some(&p), copy);
    Ok(json!({"layer": name, "id": id, "source": source, "parent": parent_name(doc, &np), "layers": doc.layer_count()}))
}

fn rm(a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    let p = layer_path(a, doc)?;
    check_tree_unlocked(doc.layer(&p))?;
    let removed = doc.remove(&p);
    Ok(json!({"removed": removed.name, "id": removed.id, "kind": removed.kind_name(), "layers": doc.layer_count()}))
}

fn rename(a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    let p = layer_path(a, doc)?;
    let (id, old) = {
        let l = doc.layer(&p);
        (l.id.clone(), l.name.clone())
    };
    let new = fresh_name(doc, &a.str_or("to", ""), std::slice::from_ref(&id))?;
    doc.layer_mut(&p).name = new.clone();
    let mut slots = Vec::new();
    for s in doc.slots.iter_mut() {
        let follows = match &s.layer {
            Some(r) => *r == old,
            None => s.name == old,
        };
        if follows && old != new {
            s.layer = Some(new.clone());
            slots.push(s.name.clone());
        }
    }
    Ok(json!({"layer": new, "from": old, "id": id, "slots_updated": slots}))
}

// ---------------------------------------------------------------------------------------
// structure

fn reorder(a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    let p = layer_path(a, doc)?;
    let id = doc.layer(&p).id.clone();
    let given: Vec<&str> = ["above", "below", "index"].into_iter().filter(|k| a.has(k)).collect();
    if given.is_empty() {
        return Err(Error::new("PX1003", "say where to move the layer: --above REF, --below REF, or --index N"));
    }
    if given.len() > 1 {
        return Err(Error::new(
            "PX1005",
            format!("pass only one of --above, --below, --index (got --{})", given.join(", --")),
        ));
    }
    if let Some(n) = a.opt_int("index")? {
        let count = doc.siblings(&p).len();
        if n < 1 || n as usize > count {
            return Err(Error::new(
                "PX1013",
                format!("--index {n} is outside 1..{count} (the layer's stack has {count} layers)"),
            )
            .with_context("range", format!("1..{count}")));
        }
        let layer = doc.remove(&p);
        let idx = count - n as usize;
        children_of(doc, &p[..p.len() - 1]).insert(idx, layer);
    } else {
        let above = a.has("above");
        let r = a.str(if above { "above" } else { "below" }).unwrap_or_default();
        let rp = doc.resolve(&r)?;
        if rp == p {
            return Err(Error::new("PX1010", format!("cannot move layer '{}' relative to itself", doc.layer(&p).name)));
        }
        if rp.len() > p.len() && rp.starts_with(&p) {
            return Err(Error::new(
                "PX1010",
                format!("'{r}' is inside '{}'; a group cannot move into itself", doc.layer(&p).name),
            ));
        }
        let ref_id = doc.layer(&rp).id.clone();
        let layer = doc.remove(&p);
        let rp = path_of(doc, &ref_id)?;
        let idx = rp.last().copied().unwrap_or(0) + usize::from(above);
        children_of(doc, &rp[..rp.len() - 1]).insert(idx, layer);
    }
    doc.active = Some(id.clone());
    let np = path_of(doc, &id)?;
    let count = doc.siblings(&np).len();
    let pos = count - np.last().copied().unwrap_or(0);
    Ok(json!({"layer": doc.layer(&np).name, "index": pos, "of": count, "parent": parent_name(doc, &np)}))
}

fn group(a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    let ids = layers_arg(a, doc)?;
    let paths: Vec<LayerPath> = ids.iter().map(|id| path_of(doc, id)).collect::<Result<_>>()?;
    let parent = paths[0][..paths[0].len() - 1].to_vec();
    if paths.iter().any(|p| p[..p.len() - 1] != parent[..]) {
        return Err(Error::new("PX1010", "layers to group must share a parent (be siblings in the same stack)")
            .with_hint("move them next to each other first with `pixelox layer reorder`")
            .with_context("layers", a.list("layers")));
    }
    let name = match a.str("name") {
        Some(n) => fresh_name(doc, &n, &[])?,
        None => doc.unique_name("Group"),
    };
    let mut idxs: Vec<usize> = paths.iter().map(|p| *p.last().unwrap_or(&0)).collect();
    idxs.sort_unstable();
    let top = *idxs.last().unwrap_or(&0);
    let members = {
        let sibs = children_of(doc, &parent);
        let mut m: Vec<Layer> = idxs.iter().rev().map(|&i| sibs.remove(i)).collect();
        m.reverse();
        m
    };
    let names: Vec<String> = members.iter().rev().map(|l| l.name.clone()).collect();
    let mut g = doc.new_layer(&name, LayerKind::Group { children: members });
    g.name = name.clone();
    let id = g.id.clone();
    let at = top + 1 - idxs.len();
    children_of(doc, &parent).insert(at, g);
    doc.active = Some(id.clone());
    Ok(json!({"group": name, "id": id, "layers": names, "blend": "pass-through"}))
}

fn ungroup(a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    let p = layer_path(a, doc)?;
    let l = doc.layer(&p);
    if !l.is_group() {
        return Err(Error::new("PX2303", format!("layer '{}' is a {} layer, not a group", l.name, l.kind_name()))
            .with_context("layer", &l.name)
            .with_context("kind", l.kind_name()));
    }
    let g = doc.remove(&p);
    let mut discarded = Vec::new();
    if g.opacity < 100.0 {
        discarded.push("opacity");
    }
    if g.blend != BlendMode::PassThrough {
        discarded.push("blend");
    }
    if g.mask.is_some() {
        discarded.push("mask");
    }
    if !g.fx.is_empty() {
        discarded.push("fx");
    }
    if !g.visible {
        discarded.push("visible");
    }
    let children = match g.content {
        LayerKind::Group { children } => children,
        _ => Vec::new(),
    };
    let names: Vec<String> = children.iter().rev().map(|c| c.name.clone()).collect();
    if let Some(top) = children.last() {
        doc.active = Some(top.id.clone());
    }
    let idx = *p.last().unwrap_or(&0);
    children_of(doc, &p[..p.len() - 1]).splice(idx..idx, children);
    Ok(json!({"ungrouped": g.name, "layers": names, "discarded": discarded}))
}

fn merge(a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    let down = a.flag("down");
    let (mut ids, default_name) = match (down, a.has("layers")) {
        (true, true) => return Err(Error::new("PX1005", "pass either --layers or --down --layer, not both")),
        (false, false) => return Err(Error::new("PX1003", "say what to merge: --layers a,b or --down --layer REF")),
        (true, false) => {
            let p = layer_path(a, doc)?;
            let idx = *p.last().unwrap_or(&0);
            if idx == 0 {
                return Err(Error::new(
                    "PX1010",
                    format!("layer '{}' is the bottom of its stack; nothing is below it", doc.layer(&p).name),
                ));
            }
            let mut below = p.clone();
            *below.last_mut().unwrap_or(&mut 0) = idx - 1;
            let lower = doc.layer(&below);
            (vec![lower.id.clone(), doc.layer(&p).id.clone()], Some(lower.name.clone()))
        }
        (false, true) => {
            if a.has("layer") {
                return Err(Error::new("PX1005", "--layer is only used with --down; list the layers with --layers"));
            }
            (layers_arg(a, doc)?, None)
        }
    };
    // a group already brings its children
    let paths: Vec<LayerPath> = ids.iter().map(|id| path_of(doc, id)).collect::<Result<_>>()?;
    ids = ids
        .iter()
        .zip(&paths)
        .filter(|(_, p)| !paths.iter().any(|q| q.len() < p.len() && p.starts_with(q)))
        .map(|(id, _)| id.clone())
        .collect();
    for id in &ids {
        check_tree_unlocked(doc.layer(&path_of(doc, id)?))?;
    }
    let walk: Vec<(String, String)> = doc.walk().iter().map(|(_, l)| (l.id.clone(), l.name.clone())).collect();
    let (top_id, top_name) = walk.iter().find(|(id, _)| ids.contains(id)).cloned().unwrap_or_default();
    let merged_names: Vec<String> = walk.iter().filter(|(id, _)| ids.contains(id)).map(|(_, n)| n.clone()).collect();
    // every layer the merge consumes (groups bring their children) may lend its name
    let mut consumed: Vec<String> = Vec::new();
    for id in &ids {
        let p = path_of(doc, id)?;
        consumed.extend(doc.walk().iter().filter(|(q, _)| q.starts_with(&p)).map(|(_, l)| l.id.clone()));
    }
    let name = match a.str("name") {
        Some(n) => fresh_name(doc, &n, &consumed)?,
        None => default_name.unwrap_or(top_name),
    };
    let canvas = compose::composite_subset(doc, &ids)?;
    let (image, x, y) = match canvas.alpha_bbox(0.0) {
        Some(r) => (canvas.crop(r), r.x, r.y),
        None => (canvas, 0, 0),
    };
    let size = [image.width, image.height];
    let mut l = doc.new_layer(&name, LayerKind::Raster { image });
    l.name = name.clone();
    l.x = x;
    l.y = y;
    let new_id = l.id.clone();
    let top = path_of(doc, &top_id)?;
    doc.insert_above(Some(&top), l);
    for id in &ids {
        if let Some(p) = doc.find_by_id(id) {
            doc.remove(&p);
        }
    }
    doc.active = Some(new_id.clone());
    Ok(
        json!({"layer": name, "id": new_id, "merged": merged_names, "offset": [x, y], "size": size, "layers": doc.layer_count()}),
    )
}

fn flatten(_a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    let image = compose::composite(doc)?;
    let removed = doc.layer_count();
    let slots: Vec<String> = doc.slots.iter().map(|s| s.name.clone()).collect();
    doc.layers.clear();
    doc.slots.clear();
    let l = doc.new_layer("Background", LayerKind::Raster { image });
    let id = l.id.clone();
    doc.active = Some(id.clone());
    doc.layers.push(l);
    Ok(json!({"layer": "Background", "id": id, "flattened": removed, "slots_removed": slots}))
}

// ---------------------------------------------------------------------------------------
// properties

fn opacity(a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    let p = layer_path(a, doc)?;
    doc.layer(&p).check_unlocked()?;
    let v = a.f64("value")?;
    let l = doc.layer_mut(&p);
    let previous = l.opacity;
    l.opacity = v as f32;
    Ok(json!({"layer": l.name, "opacity": v, "previous": previous}))
}

fn blend(a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    let p = layer_path(a, doc)?;
    let l = doc.layer(&p);
    l.check_unlocked()?;
    let mode = BlendMode::parse(&a.choice("mode"))?;
    if mode == BlendMode::PassThrough && !l.is_group() {
        return Err(Error::new(
            "PX1010",
            format!("pass-through applies to groups only; '{}' is a {} layer", l.name, l.kind_name()),
        )
        .with_hint(format!(
            "use another mode, e.g. pixelox layer blend <doc> --layer '{}' --mode normal --in-place",
            l.name
        )));
    }
    let l = doc.layer_mut(&p);
    let previous = l.blend.name();
    l.blend = mode;
    Ok(json!({"layer": l.name, "blend": mode.name(), "previous": previous}))
}

fn set_bool(a: &Args, doc: &mut Document, key: &str, value: bool) -> Result<Value> {
    let p = layer_path(a, doc)?;
    let l = doc.layer_mut(&p);
    let slot = if key == "visible" { &mut l.visible } else { &mut l.locked };
    let changed = *slot != value;
    *slot = value;
    let mut out = json!({"layer": l.name, "changed": changed});
    out[key] = json!(value);
    Ok(out)
}

fn show(a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    set_bool(a, doc, "visible", true)
}

fn hide(a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    set_bool(a, doc, "visible", false)
}

fn lock(a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    set_bool(a, doc, "locked", true)
}

fn unlock(a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    set_bool(a, doc, "locked", false)
}

fn clip(a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    let p = layer_path(a, doc)?;
    let l = doc.layer(&p);
    l.check_unlocked()?;
    let on = !a.flag("off");
    let idx = *p.last().unwrap_or(&0);
    if on && idx == 0 {
        return Err(Error::new(
            "PX1010",
            format!("layer '{}' is the bottom of its stack; there is no layer below to clip to", l.name),
        ));
    }
    let base = if on {
        let sibs = doc.siblings(&p);
        (0..idx).rev().map(|i| &sibs[i]).find(|s| !s.clip).map(|s| s.name.clone())
    } else {
        None
    };
    let l = doc.layer_mut(&p);
    let changed = l.clip != on;
    l.clip = on;
    Ok(json!({"layer": l.name, "clip": on, "base": base, "changed": changed}))
}

fn rasterize(a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    let p = layer_path(a, doc)?;
    let l = doc.layer(&p);
    l.check_unlocked()?;
    let kind = l.kind_name();
    match &l.content {
        LayerKind::Raster { .. } => return Ok(json!({"layer": l.name, "from": kind, "changed": false})),
        LayerKind::Group { .. } => {
            return Err(Error::new("PX2303", format!("layer '{}' is a group; merge it to bake it into pixels", l.name))
                .with_hint(format!("pixelox layer merge <doc> --layers '{}' --in-place", l.name))
                .with_context("kind", kind))
        }
        LayerKind::Adjustment { .. } => {
            return Err(Error::new(
                "PX2303",
                format!("layer '{}' is an adjustment layer; it has no pixels of its own", l.name),
            )
            .with_hint(format!(
                "bake it into the layer below: pixelox layer merge <doc> --down --layer '{}' --in-place",
                l.name
            ))
            .with_context("kind", kind))
        }
        _ => {}
    }
    let origin = match &l.content {
        LayerKind::Text { spec } | LayerKind::Shape { spec } => json!({"kind": kind, "spec": spec}),
        LayerKind::Placed { path, asset, rect, angle, linked, .. } => {
            json!({"kind": kind, "path": path, "asset": asset, "rect": rect, "angle": angle, "linked": linked})
        }
        _ => Value::Null,
    };
    let mut bare = l.clone();
    bare.fx.clear();
    let baked_filters = bare.smart_filters.len();
    let rendered = compose::render_layer(doc, &bare)?;
    let (image, x, y) = match rendered {
        // smart filters render canvas-sized: keep only the visible part
        Some(r) if baked_filters > 0 => match r.image.alpha_bbox(0.0) {
            Some(b) => (r.image.crop(b), r.x + b.x, r.y + b.y),
            None => (Image::new(doc.width, doc.height), 0, 0),
        },
        Some(r) => (r.image, r.x, r.y),
        None => (Image::new(doc.width, doc.height), 0, 0),
    };
    let size = [image.width, image.height];
    let l = doc.layer_mut(&p);
    l.content = LayerKind::Raster { image };
    l.x = x;
    l.y = y;
    l.smart_filters.clear();
    l.props.insert("rasterized_from".into(), origin);
    Ok(
        json!({"layer": l.name, "from": kind, "changed": true, "offset": [x, y], "size": size, "fx_live": l.fx.len(), "smart_filters_baked": baked_filters}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use pixelox_core::doc::{FxSpec, Slot};
    use pixelox_core::testing::{self, find, max_diff, raster, run};
    use std::sync::Arc;

    fn op(name: &str) -> OpSpec {
        find(&ops(), name)
    }

    fn red() -> Rgba {
        Rgba::new(1.0, 0.0, 0.0, 1.0)
    }

    /// Background (white), A (red, left half), B (blue, top half): bottom → top.
    fn abc() -> Document {
        let mut d = testing::doc_layers(
            8,
            8,
            vec![
                ("Background", Image::filled(8, 8, Rgba::WHITE)),
                ("A", Image::from_fn(8, 8, |x, _| if x < 4 { [1.0, 0.0, 0.0, 1.0] } else { [0.0; 4] })),
                ("B", Image::from_fn(8, 8, |_, y| if y < 4 { [0.0, 0.0, 1.0, 1.0] } else { [0.0; 4] })),
            ],
        );
        d.active = None;
        d
    }

    fn names(d: &Document) -> Vec<String> {
        d.walk().iter().map(|(_, l)| l.name.clone()).collect()
    }

    fn placed(d: &mut Document, name: &str, rect: [f64; 4]) {
        let l = d.new_layer(
            name,
            LayerKind::Placed {
                asset: "k".into(),
                path: "p.png".into(),
                linked: false,
                rect,
                angle: 0.0,
                source: Some(Arc::new(Image::filled(4, 4, Rgba::new(0.0, 1.0, 0.0, 1.0)))),
            },
        );
        d.insert_above(None, l);
    }

    #[test]
    fn blend_if_sets_validates_and_removes() {
        let mut d = abc();
        let v = run(&op("layer blend-if"), &[("layer", "A"), ("underlying", "0,0,100,150")], &mut d).unwrap();
        assert_eq!(v["identity"], false);
        let a = d.resolve("A").unwrap();
        assert_eq!(d.layer(&a).blend_if.as_ref().unwrap().underlying, [0.0, 0.0, 100.0, 150.0]);
        // the white backdrop (255) is outside 0..150: A disappears where nothing dark is below
        let c = compose::composite(&d).unwrap();
        assert_eq!(c.get(0, 5), [1.0, 1.0, 1.0, 1.0]);
        let v = run(&op("layer list"), &[], &mut d).unwrap();
        assert!(v["layers"].as_array().unwrap().iter().any(|l| l["name"] == "A" && l["blend_if"]["channel"] == "gray"));
        for bad in ["10", "0,300", "200,100", "a,b"] {
            let e = run(&op("layer blend-if"), &[("layer", "A"), ("this", bad)], &mut d).unwrap_err();
            assert!(e.code == "PX1010" || e.code == "PX1013", "{bad}: {}", e.code);
        }
        let v = run(&op("layer blend-if"), &[("layer", "A"), ("off", "true")], &mut d).unwrap();
        assert_eq!(v["removed"], true);
        assert!(d.layer(&a).blend_if.is_none());
        assert_eq!(
            run(&op("layer blend-if"), &[("layer", "A"), ("off", "true"), ("this", "0,200")], &mut d).unwrap_err().code,
            "PX1005"
        );
        // full ranges are the identity: nothing stored
        run(&op("layer blend-if"), &[("layer", "A")], &mut d).unwrap();
        assert!(d.layer(&a).blend_if.is_none());
    }

    #[test]
    fn stamp_composites_visible_into_new_top_layer() {
        let mut d = abc();
        let before = compose::composite(&d).unwrap();
        let v = run(&op("layer stamp"), &[], &mut d).unwrap();
        assert_eq!(v["layer"], "Stamp");
        assert_eq!(d.layers.last().unwrap().name, "Stamp");
        assert_eq!(d.layer_count(), 4);
        // hiding everything else leaves the same picture
        for l in d.layers.iter_mut().filter(|l| l.name != "Stamp") {
            l.visible = false;
        }
        assert!(max_diff(&compose::composite(&d).unwrap(), &before) < 1e-6);
        // --above and unique names
        let v = run(&op("layer stamp"), &[("above", "Background")], &mut d).unwrap();
        assert_eq!(v["layer"], "Stamp 2");
        assert_eq!(d.layers[1].name, "Stamp 2");
    }

    #[test]
    fn smart_filter_toggle_and_remove() {
        let mut d = abc();
        placed(&mut d, "P", [2.0, 2.0, 4.0, 4.0]);
        let sm = op("layer smart-filter");
        assert_eq!(run(&sm, &[("layer", "P"), ("action", "disable")], &mut d).unwrap_err().code, "PX1010");
        let p = d.resolve("P").unwrap();
        for name in ["a.one", "b.two"] {
            d.layer_mut(&p).smart_filters.push(pixelox_core::SmartFilter {
                op: name.into(),
                enabled: true,
                params: Map::new(),
                seed: 0,
                op_index: 0,
            });
        }
        let v = run(&sm, &[("layer", "P"), ("action", "disable")], &mut d).unwrap();
        assert_eq!(v["op"], "b.two");
        assert!(!d.layer(&p).smart_filters[1].enabled);
        run(&sm, &[("layer", "P"), ("action", "enable"), ("index", "2")], &mut d).unwrap();
        assert!(d.layer(&p).smart_filters[1].enabled);
        run(&sm, &[("layer", "P"), ("action", "remove"), ("index", "1")], &mut d).unwrap();
        assert_eq!(d.layer(&p).smart_filters.len(), 1);
        assert_eq!(d.layer(&p).smart_filters[0].op, "b.two");
        assert_eq!(
            run(&sm, &[("layer", "P"), ("action", "remove"), ("index", "3")], &mut d).unwrap_err().code,
            "PX1013"
        );
    }

    #[test]
    fn rasterize_bakes_smart_filters() {
        let mut d = abc();
        placed(&mut d, "P", [2.0, 2.0, 4.0, 4.0]);
        let p = d.resolve("P").unwrap();
        d.layer_mut(&p).smart_filters.push(pixelox_core::SmartFilter {
            op: "nothing.registered".into(),
            enabled: false,
            params: Map::new(),
            seed: 0,
            op_index: 0,
        });
        let v = run(&op("layer rasterize"), &[("layer", "P")], &mut d).unwrap();
        assert_eq!(v["smart_filters_baked"], 1);
        let l = d.layer(&p);
        assert!(l.smart_filters.is_empty());
        assert_eq!((l.x, l.y), (2, 2));
        assert_eq!(l.raster().unwrap().width, 4);
    }

    #[test]
    fn list_flat_and_tree() {
        let mut d = abc();
        run(&op("layer group"), &[("layers", "A,B"), ("name", "G")], &mut d).unwrap();
        let v = run(&op("layer list"), &[], &mut d).unwrap();
        let rows = v["layers"].as_array().unwrap();
        assert_eq!(rows.len(), 4);
        assert_eq!(rows[0]["name"], "G");
        assert_eq!(rows[0]["child_count"], 2);
        assert_eq!(rows[1]["name"], "B");
        assert_eq!(rows[1]["depth"], 1);
        assert_eq!(rows[1]["parent"], "G");
        assert_eq!(rows[1]["index"], 2);
        assert_eq!(rows[1]["bounds"], json!([0, 0, 8, 4]));
        let v = run(&op("layer list"), &[("tree", "true")], &mut d).unwrap();
        assert_eq!(v["layers"].as_array().unwrap().len(), 2);
        assert_eq!(v["layers"][0]["children"][0]["name"], "B");
        assert!(op("layer list").is_read_only());
    }

    #[test]
    fn add_defaults_and_geometry() {
        let mut d = abc();
        let v = run(&op("layer add"), &[], &mut d).unwrap();
        assert_eq!(v["layer"], "Layer");
        assert_eq!(v["size"], json!([8, 8]));
        assert_eq!(raster(&d, "Layer").get(3, 3)[3], 0.0);
        assert_eq!(names(&d)[0], "Layer", "on top when nothing is active");
        // above the active layer (the new one is active now; activate A)
        d.active = Some(d.layer(&d.resolve("A").unwrap()).id.clone());
        let v =
            run(&op("layer add"), &[("name", "Band"), ("size", "100%x2"), ("at", "0,-2"), ("fill", "#ff0000")], &mut d)
                .unwrap();
        assert_eq!(v["offset"], json!([0, 6]));
        assert_eq!(names(&d), ["Layer", "B", "Band", "A", "Background"]);
        let l = d.layer(&d.resolve("Band").unwrap());
        assert_eq!((l.x, l.y, l.raster().unwrap().width, l.raster().unwrap().height), (0, 6, 8, 2));
        assert_eq!(l.raster().unwrap().get(0, 0), [1.0, 0.0, 0.0, 1.0]);
        assert_eq!(run(&op("layer add"), &[("name", "A")], &mut d).unwrap_err().code, "PX2305");
        assert_eq!(run(&op("layer add"), &[("name", "@top")], &mut d).unwrap_err().code, "PX1010");
    }

    #[test]
    fn dup_copies_mask_fx_and_children_with_fresh_ids() {
        let mut d = abc();
        let pa = d.resolve("A").unwrap();
        d.layer_mut(&pa).mask = Some(LayerMask::new(Mask::filled(8, 8, 1.0)));
        d.layer_mut(&pa).fx.push(FxSpec { kind: "stroke".into(), enabled: true, params: Map::new() });
        let v = run(&op("layer dup"), &[("layer", "A")], &mut d).unwrap();
        assert_eq!(v["layer"], "A copy");
        assert_eq!(names(&d), ["B", "A copy", "A", "Background"]);
        let c = d.layer(&d.resolve("A copy").unwrap());
        assert!(c.mask.is_some() && c.fx.len() == 1);
        assert_ne!(c.id, d.layer(&pa).id);
        let v = run(&op("layer dup"), &[("layer", "A")], &mut d).unwrap();
        assert_eq!(v["layer"], "A copy 2");
        // groups: children get fresh ids and unique names
        run(&op("layer group"), &[("layers", "B"), ("name", "G")], &mut d).unwrap();
        run(&op("layer dup"), &[("layer", "G"), ("name", "G2")], &mut d).unwrap();
        let all = d.walk();
        let mut ids: Vec<&str> = all.iter().map(|(_, l)| l.id.as_str()).collect();
        let n = ids.len();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), n, "ids are unique");
        assert!(names(&d).contains(&"B copy".to_string()));
        assert_eq!(run(&op("layer dup"), &[("layer", "A"), ("name", "B")], &mut d).unwrap_err().code, "PX2305");
    }

    #[test]
    fn rm_addressing_and_locks() {
        let mut d = abc();
        let v = run(&op("layer rm"), &[("layer", "#1")], &mut d).unwrap();
        assert_eq!(v["removed"], "B");
        assert_eq!(names(&d), ["A", "Background"]);
        let p = d.resolve("@bottom").unwrap();
        d.layer_mut(&p).locked = true;
        assert_eq!(run(&op("layer rm"), &[("layer", "@bottom")], &mut d).unwrap_err().code, "PX2304");
        assert_eq!(run(&op("layer rm"), &[], &mut d).unwrap_err().code, "PX2302");
        assert_eq!(run(&op("layer rm"), &[("layer", "nope")], &mut d).unwrap_err().code, "PX2301");
    }

    #[test]
    fn rename_keeps_names_unique_and_follows_slots() {
        let mut d = abc();
        d.slots.push(Slot {
            name: "A".into(),
            kind: "image".into(),
            required: false,
            layer: None,
            fit: None,
            constraints: Map::new(),
        });
        let v = run(&op("layer rename"), &[("layer", "A"), ("to", "Hero")], &mut d).unwrap();
        assert_eq!(v["from"], "A");
        assert_eq!(d.slots[0].layer.as_deref(), Some("Hero"));
        assert_eq!(run(&op("layer rename"), &[("layer", "Hero"), ("to", "B")], &mut d).unwrap_err().code, "PX2305");
        // renaming to its own name is fine
        run(&op("layer rename"), &[("layer", "Hero"), ("to", "Hero")], &mut d).unwrap();
        // locked layers can be renamed
        let p = d.resolve("B").unwrap();
        d.layer_mut(&p).locked = true;
        run(&op("layer rename"), &[("layer", "B"), ("to", "Blue")], &mut d).unwrap();
    }

    #[test]
    fn reorder_above_below_index() {
        let mut d = abc();
        run(&op("layer reorder"), &[("layer", "Background"), ("above", "B")], &mut d).unwrap();
        assert_eq!(names(&d), ["Background", "B", "A"]);
        run(&op("layer reorder"), &[("layer", "Background"), ("below", "A")], &mut d).unwrap();
        assert_eq!(names(&d), ["B", "A", "Background"]);
        let v = run(&op("layer reorder"), &[("layer", "Background"), ("index", "2")], &mut d).unwrap();
        assert_eq!(v["index"], 2);
        assert_eq!(names(&d), ["B", "Background", "A"]);
        assert_eq!(run(&op("layer reorder"), &[("layer", "A"), ("index", "4")], &mut d).unwrap_err().code, "PX1013");
        assert_eq!(
            run(&op("layer reorder"), &[("layer", "A"), ("index", "1"), ("above", "B")], &mut d).unwrap_err().code,
            "PX1005"
        );
        assert_eq!(run(&op("layer reorder"), &[("layer", "A")], &mut d).unwrap_err().code, "PX1003");
        // into a group, and never into itself
        run(&op("layer group"), &[("layers", "B"), ("name", "G")], &mut d).unwrap();
        run(&op("layer reorder"), &[("layer", "A"), ("below", "B")], &mut d).unwrap();
        assert_eq!(d.resolve("A").unwrap().len(), 2);
        assert_eq!(run(&op("layer reorder"), &[("layer", "G"), ("above", "B")], &mut d).unwrap_err().code, "PX1010");
    }

    #[test]
    fn group_and_ungroup_keep_the_composite() {
        let mut d = abc();
        let before = compose::composite(&d).unwrap();
        let v = run(&op("layer group"), &[("layers", "B,A"), ("name", "G")], &mut d).unwrap();
        assert_eq!(v["layers"], json!(["B", "A"]));
        assert_eq!(names(&d), ["G", "B", "A", "Background"]);
        let g = d.layer(&d.resolve("G").unwrap());
        assert_eq!(g.blend, BlendMode::PassThrough);
        assert_eq!(max_diff(&before, &compose::composite(&d).unwrap()), 0.0);
        assert_eq!(run(&op("layer group"), &[("layers", "B,Background")], &mut d).unwrap_err().code, "PX1010");
        assert_eq!(
            run(&op("layer group"), &[("layers", "Background"), ("name", "A")], &mut d).unwrap_err().code,
            "PX2305"
        );
        // group in the middle of a stack lands at the topmost member's position
        let mut d2 = abc();
        run(&op("layer group"), &[("layers", "Background,A")], &mut d2).unwrap();
        assert_eq!(names(&d2), ["B", "Group", "A", "Background"]);
        let v = run(&op("layer ungroup"), &[("layer", "G")], &mut d).unwrap();
        assert_eq!(v["layers"], json!(["B", "A"]));
        assert_eq!(names(&d), ["B", "A", "Background"]);
        assert_eq!(max_diff(&before, &compose::composite(&d).unwrap()), 0.0);
        assert_eq!(run(&op("layer ungroup"), &[("layer", "A")], &mut d).unwrap_err().code, "PX2303");
    }

    #[test]
    fn merge_equals_the_composite_of_the_merged_layers() {
        let mut d = abc();
        let pb = d.resolve("B").unwrap();
        d.layer_mut(&pb).blend = BlendMode::Multiply;
        d.layer_mut(&pb).opacity = 60.0;
        let ids: Vec<String> = ["A", "B"].iter().map(|n| d.layer(&d.resolve(n).unwrap()).id.clone()).collect();
        let expected = compose::composite_subset(&d, &ids).unwrap();
        let full_before = compose::composite(&d).unwrap();
        let v = run(&op("layer merge"), &[("layers", "A,B"), ("name", "AB")], &mut d).unwrap();
        assert_eq!(v["merged"], json!(["B", "A"]));
        assert_eq!(names(&d), ["AB", "Background"]);
        let l = d.layer(&d.resolve("AB").unwrap());
        let got = Rendered { image: l.raster().unwrap().clone(), x: l.x, y: l.y }.to_canvas(8, 8);
        assert!(max_diff(&got, &expected) < 1e-6);
        // multiply over white equals normal, so here the whole composite is unchanged too
        assert!(max_diff(&compose::composite(&d).unwrap(), &full_before) < 1e-5);
        assert_eq!(l.blend, BlendMode::Normal);
        assert_eq!(l.opacity, 100.0);
    }

    #[test]
    fn merge_down_names_and_locks() {
        let mut d = abc();
        let v = run(&op("layer merge"), &[("down", "true"), ("layer", "B")], &mut d).unwrap();
        assert_eq!(v["layer"], "A");
        assert_eq!(names(&d), ["A", "Background"]);
        let a = raster(&d, "A");
        assert_eq!(d.layer(&d.resolve("A").unwrap()).x, 0);
        assert_eq!(a.get(0, 0), [0.0, 0.0, 1.0, 1.0]);
        assert_eq!(a.get(1, 6), [1.0, 0.0, 0.0, 1.0]);
        assert_eq!(
            run(&op("layer merge"), &[("down", "true"), ("layer", "Background")], &mut d).unwrap_err().code,
            "PX1010"
        );
        let p = d.resolve("Background").unwrap();
        d.layer_mut(&p).locked = true;
        assert_eq!(run(&op("layer merge"), &[("down", "true"), ("layer", "A")], &mut d).unwrap_err().code, "PX2304");
        assert_eq!(run(&op("layer merge"), &[], &mut d).unwrap_err().code, "PX1003");
        assert_eq!(run(&op("layer merge"), &[("down", "true"), ("layers", "A")], &mut d).unwrap_err().code, "PX1005");
    }

    #[test]
    fn merge_bakes_a_group_and_its_name() {
        let mut d = abc();
        run(&op("layer group"), &[("layers", "A,B"), ("name", "G")], &mut d).unwrap();
        let before = compose::composite(&d).unwrap();
        // naming the result like a merged layer is allowed
        run(&op("layer merge"), &[("layers", "G,A"), ("name", "A")], &mut d).unwrap();
        assert_eq!(names(&d), ["A", "Background"]);
        assert!(max_diff(&before, &compose::composite(&d).unwrap()) < 1e-6);
    }

    #[test]
    fn flatten_keeps_masks_and_selection() {
        let mut d = abc();
        d.put_mask("m", testing::half_mask(8, 8), false).unwrap();
        d.selection = Some(testing::half_mask(8, 8));
        let before = compose::composite(&d).unwrap();
        let v = run(&op("layer flatten"), &[], &mut d).unwrap();
        assert_eq!(v["flattened"], 3);
        assert_eq!(names(&d), ["Background"]);
        assert_eq!(max_diff(&before, raster(&d, "Background")), 0.0);
        assert!(d.masks.contains_key("m") && d.selection.is_some());
    }

    #[test]
    fn opacity_blend_and_locks() {
        let mut d = abc();
        let v = run(&op("layer opacity"), &[("layer", "A"), ("value", "85")], &mut d).unwrap();
        assert_eq!(v["previous"], 100.0);
        assert_eq!(d.layer(&d.resolve("A").unwrap()).opacity, 85.0);
        assert_eq!(run(&op("layer opacity"), &[("layer", "A"), ("value", "101")], &mut d).unwrap_err().code, "PX1013");
        for m in BLEND_MODES.iter().filter(|m| **m != "pass-through") {
            run(&op("layer blend"), &[("layer", "A"), ("mode", m)], &mut d).unwrap();
            assert_eq!(d.layer(&d.resolve("A").unwrap()).blend.name(), *m);
        }
        assert_eq!(BLEND_MODES.len(), 28);
        assert_eq!(
            run(&op("layer blend"), &[("layer", "A"), ("mode", "pass-through")], &mut d).unwrap_err().code,
            "PX1010"
        );
        assert_eq!(run(&op("layer blend"), &[("layer", "A"), ("mode", "glow")], &mut d).unwrap_err().code, "PX1010");
        run(&op("layer group"), &[("layers", "A"), ("name", "G")], &mut d).unwrap();
        run(&op("layer blend"), &[("layer", "G"), ("mode", "normal")], &mut d).unwrap();
        run(&op("layer blend"), &[("layer", "G"), ("mode", "pass-through")], &mut d).unwrap();
        run(&op("layer lock"), &[("layer", "B")], &mut d).unwrap();
        assert_eq!(run(&op("layer opacity"), &[("layer", "B"), ("value", "5")], &mut d).unwrap_err().code, "PX2304");
        assert_eq!(run(&op("layer blend"), &[("layer", "B"), ("mode", "screen")], &mut d).unwrap_err().code, "PX2304");
    }

    #[test]
    fn show_hide_lock_unlock() {
        let mut d = abc();
        let v = run(&op("layer hide"), &[("layer", "B")], &mut d).unwrap();
        assert_eq!((v["visible"].clone(), v["changed"].clone()), (json!(false), json!(true)));
        assert_eq!(compose::composite(&d).unwrap().get(6, 0), [1.0, 1.0, 1.0, 1.0]);
        let v = run(&op("layer hide"), &[("layer", "B")], &mut d).unwrap();
        assert_eq!(v["changed"], false);
        run(&op("layer show"), &[("layer", "B")], &mut d).unwrap();
        assert!(d.layer(&d.resolve("B").unwrap()).visible);
        run(&op("layer lock"), &[("layer", "B")], &mut d).unwrap();
        assert!(d.layer(&d.resolve("B").unwrap()).locked);
        // visibility still toggles on a locked layer
        run(&op("layer hide"), &[("layer", "B")], &mut d).unwrap();
        run(&op("layer unlock"), &[("layer", "B")], &mut d).unwrap();
        assert!(!d.layer(&d.resolve("B").unwrap()).locked);
    }

    #[test]
    fn clip_onto_the_layer_below() {
        let mut d = abc();
        let v = run(&op("layer clip"), &[("layer", "B")], &mut d).unwrap();
        assert_eq!(v["base"], "A");
        // B only shows where A has coverage
        let c = compose::composite(&d).unwrap();
        assert_eq!(c.get(1, 1), [0.0, 0.0, 1.0, 1.0]);
        assert_eq!(c.get(6, 1), [1.0, 1.0, 1.0, 1.0]);
        run(&op("layer clip"), &[("layer", "B"), ("off", "true")], &mut d).unwrap();
        assert!(!d.layer(&d.resolve("B").unwrap()).clip);
        assert_eq!(run(&op("layer clip"), &[("layer", "Background")], &mut d).unwrap_err().code, "PX1010");
    }

    #[test]
    fn rasterize_placed_keeps_styles_live() {
        let mut d = abc();
        placed(&mut d, "P", [2.0, 2.0, 4.0, 4.0]);
        let p = d.resolve("P").unwrap();
        // no fx renderer is installed: rasterize must not need it
        d.layer_mut(&p).fx.push(FxSpec { kind: "drop-shadow".into(), enabled: true, params: Map::new() });
        d.layer_mut(&p).opacity = 50.0;
        let v = run(&op("layer rasterize"), &[("layer", "P")], &mut d).unwrap();
        assert_eq!(v["offset"], json!([2, 2]));
        let l = d.layer(&p);
        assert_eq!(l.kind_name(), "raster");
        assert_eq!(l.fx.len(), 1);
        assert_eq!(l.opacity, 50.0);
        assert_eq!(l.raster().unwrap().get(1, 1), [0.0, 1.0, 0.0, 1.0]);
        assert_eq!(l.props["rasterized_from"]["kind"], "placed");
        let v = run(&op("layer rasterize"), &[("layer", "A")], &mut d).unwrap();
        assert_eq!(v["changed"], false);
        run(&op("layer group"), &[("layers", "A"), ("name", "G")], &mut d).unwrap();
        assert_eq!(run(&op("layer rasterize"), &[("layer", "G")], &mut d).unwrap_err().code, "PX2303");
        run(&op("layer lock"), &[("layer", "B")], &mut d).unwrap();
        assert_eq!(run(&op("layer rasterize"), &[("layer", "B")], &mut d).unwrap_err().code, "PX2304");
    }

    #[test]
    fn single_layer_documents_need_no_layer_flag() {
        let mut d = testing::doc_from(testing::solid(2, 2, red()));
        run(&op("layer opacity"), &[("value", "40")], &mut d).unwrap();
        assert_eq!(d.layers[0].opacity, 40.0);
    }
}
