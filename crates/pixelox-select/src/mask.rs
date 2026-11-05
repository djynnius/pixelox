//! `mask *`: the named-mask library, layer masks (raster and vector), and mask refinement.
//!
//! Refinement commands (`density`, `feather`, `expand`, `contract`, `smooth`, `invert`)
//! edit one stored mask: `--mask NAME`, `--mask @sel` (the active selection), or
//! `--layer REF` (that layer's mask). With neither flag they edit the active selection.

use crate::raster::{fill_mask, parse_path};
use crate::util::{self, at_most_one, decode_mask_file, fit_mask, mask_json, r3, resolve_mask_ref};
use pixelox_core::doc::LayerPath;
use pixelox_core::exec;
use pixelox_core::op::ParamSpec;
use pixelox_core::prelude::*;
use rayon::prelude::*;
use std::path::PathBuf;

const CAT: Category = Category::Mask;

fn target_mask_param() -> ParamSpec {
    param("mask", ParamKind::Mask).help("Mask to edit: a library name or @sel (default: the active selection)")
}

fn target_layer_param() -> ParamSpec {
    param("layer", ParamKind::Layer).help("Edit this layer's mask instead of --mask (name, #N, @top, @bottom, @active)")
}

fn export_spec() -> OpSpec {
    OpSpec::doc("mask export", CAT, "Write a mask as a grayscale image", export)
        .about("Render any mask reference (library name, @sel, @alpha:<layer>) as an opaque grayscale image (white = covered) at canvas size. The document itself is not changed; the image's sidecar records the document's recipe plus this step.")
        .export_mode()
        .param(param("mask", ParamKind::Mask).required().help("Mask to write: a library name, @sel, or @alpha:<layer>"))
        .example("pixelox mask export doc.pxo --mask subject -o subject-mask.png", "Export a named mask")
        .example("pixelox mask export doc.pxo --mask @sel -o selection.png", "Export the active selection")
        .example("pixelox mask export doc.pxo --mask @alpha:Logo -o logo-alpha.png --overwrite", "Export a layer's alpha")
}

pub fn ops() -> Vec<OpSpec> {
    vec![
        OpSpec::doc("mask list", CAT, "List named masks and layer masks with coverage", list)
            .about("Read-only: every library mask and every layer mask (with enabled, density, live feather, and whether a vector mask is attached), each with its bounds, area, and coverage percent; plus the active selection.")
            .read_only()
            .example("pixelox mask list doc.pxo --json", "Inspect masks")
            .example("pixelox mask list doc.pxo", "Print a summary")
            .example("pixelox mask list poster.pxo --json --quiet", "Machine-readable listing"),
        OpSpec::doc("mask import", CAT, "Import a grayscale image as a named mask", import)
            .about("Decode FILE (luma, or its alpha when it has transparency) into the named-mask library. The image must match the canvas size (PX2313).")
            .param(param("file", ParamKind::Path).required().help("Grayscale image the size of the canvas"))
            .positional("file")
            .param(param("as", ParamKind::Str).required().help("Name for the mask (cannot start with @)"))
            .param(param("replace", ParamKind::Flag).help("Overwrite a mask with the same name"))
            .example("pixelox mask import doc.pxo matte.png --as subject --in-place", "Import a matte")
            .example("pixelox mask import doc.pxo sky.png --as sky --replace --in-place", "Replace a mask")
            .example("pixelox mask import photo.jpg hair.png --as hair -o doc.pxo", "Open a photo with a mask"),
        export_spec(),
        OpSpec::doc("mask rm", CAT, "Remove a named mask from the library", rm)
            .about("Delete a library mask (PX2311 if absent). `--mask @sel` clears the active selection.")
            .param(param("mask", ParamKind::Mask).required().help("Library mask name (or @sel to deselect)"))
            .example("pixelox mask rm doc.pxo --mask old-sky --in-place", "Delete a mask")
            .example("pixelox mask rm doc.pxo --mask @sel --in-place", "Clear the selection")
            .example("pixelox mask rm doc.pxo --mask tmp -o clean.pxo", "Write a copy without the mask"),
        OpSpec::doc("mask add", CAT, "Attach a layer mask", add)
            .about("Attach a non-destructive layer mask (Photoshop semantics: white reveals, black hides) from the selection or a mask reference (--from), an image (--from-file), all revealed or hidden, or SVG path data (--from-path: a vector mask, rasterized anti-aliased into the layer mask with the path kept on the layer). With no source flag: the active selection if there is one, else reveal-all. Replaces an existing layer mask.")
            .param(param("layer", ParamKind::Layer).help("Layer to mask (default: the only layer)"))
            .param(param("from", ParamKind::Mask).help("Mask reference: @sel, a library name, or @alpha:<layer>"))
            .param(param("from-file", ParamKind::Path).help("Grayscale image the size of the canvas"))
            .param(param("reveal-all", ParamKind::Flag).help("A white mask (everything visible)"))
            .param(param("hide-all", ParamKind::Flag).help("A black mask (everything hidden)"))
            .param(param("from-path", ParamKind::Str).help("SVG path data in canvas pixels: a vector mask"))
            .example("pixelox mask add doc.pxo --layer Photo --from @sel --in-place", "Mask a layer by the selection")
            .example("pixelox mask add doc.pxo --layer Photo --from-file matte.png --in-place", "Mask a layer by an image")
            .example("pixelox mask add doc.pxo --layer Badge --from-path 'M 0 0 H 300 V 300 Z' --in-place", "Vector mask"),
        OpSpec::doc("mask remove", CAT, "Delete a layer mask, or bake it into alpha", remove)
            .about("Remove a layer's mask (and vector mask). With --apply, first multiply the mask (with its density and live feather) into the raster layer's alpha (PX2303 for non-raster layers).")
            .param(param("layer", ParamKind::Layer).help("Layer whose mask to remove (default: the only layer)"))
            .param(param("apply", ParamKind::Flag).help("Bake the mask into the layer's alpha first"))
            .example("pixelox mask remove doc.pxo --layer Photo --in-place", "Drop a layer mask")
            .example("pixelox mask remove doc.pxo --layer Photo --apply --in-place", "Apply the mask to the pixels")
            .example("pixelox mask remove cutout.pxo --apply -o baked.png", "Bake and export a cutout"),
        OpSpec::doc("mask enable", CAT, "Turn a layer mask on", enable)
            .about("Re-enable a disabled layer mask (it is kept, just not applied, while disabled).")
            .param(param("layer", ParamKind::Layer).help("Layer whose mask to enable (default: the only layer)"))
            .example("pixelox mask enable doc.pxo --layer Photo --in-place", "Enable the mask")
            .example("pixelox mask enable doc.pxo --layer '#1' --in-place", "Enable the top layer's mask")
            .example("pixelox mask enable doc.pxo --layer Photo -o on.png", "Render with the mask on"),
        OpSpec::doc("mask disable", CAT, "Turn a layer mask off without deleting it", disable)
            .about("Disable a layer mask: the layer renders as if unmasked until `mask enable`.")
            .param(param("layer", ParamKind::Layer).help("Layer whose mask to disable (default: the only layer)"))
            .example("pixelox mask disable doc.pxo --layer Photo --in-place", "Disable the mask")
            .example("pixelox mask disable doc.pxo --layer Photo -o unmasked.png", "Preview without the mask")
            .example("pixelox mask disable doc.pxo --layer @top --in-place", "Disable the top layer's mask"),
        OpSpec::doc("mask density", CAT, "Set a mask's density (maximum strength)", density)
            .about("Density 0..100 limits how much a mask hides: coverage becomes 1 - density·(1 - m). On a layer mask (--layer REF) it is a live property; on --mask NAME or --mask @sel (the default when neither flag is given) the pixels are rewritten.")
            .param(target_layer_param())
            .param(target_mask_param())
            .param(param("value", ParamKind::Float).required().range("0..100").help("Density in percent, 0..100"))
            .example("pixelox mask density doc.pxo --layer Photo --value 60 --in-place", "Soften a layer mask")
            .example("pixelox mask density doc.pxo --mask sky --value 80 --in-place", "Rewrite a library mask")
            .example("pixelox mask density doc.pxo --mask @sel --value 50 --in-place", "Half-strength selection"),
        OpSpec::doc("mask feather", CAT, "Feather a mask (Gaussian)", feather)
            .about("Gaussian feather by --radius px (sigma = radius / 2). --mask NAME or --mask @sel (the default when neither flag is given) is blurred in place; with --layer REF the radius becomes that layer mask's live feather property (non-destructive, 0 removes it).")
            .param(target_mask_param())
            .param(target_layer_param())
            .param(param("radius", ParamKind::Length).required().help("Feather radius in px (or % of the shorter canvas side)"))
            .example("pixelox mask feather doc.pxo --mask dog --radius 12 --in-place", "Feather a named mask")
            .example("pixelox mask feather doc.pxo --mask @sel --radius 4px --in-place", "Feather the selection")
            .example("pixelox mask feather doc.pxo --layer Photo --radius 8 --in-place", "Live-feather a layer mask"),
        OpSpec::doc("mask expand", CAT, "Grow a mask morphologically", expand)
            .about("Dilate the mask by a disc of radius --by px (soft edges are preserved). Edits --mask NAME, --mask @sel, or --layer REF (that layer's mask); with neither, the active selection.")
            .param(target_mask_param())
            .param(target_layer_param())
            .param(param("by", ParamKind::Length).required().help("Growth radius in px"))
            .example("pixelox mask expand doc.pxo --mask dog --by 6 --in-place", "Grow a mask 6 px")
            .example("pixelox mask expand doc.pxo --mask @sel --by 2px --in-place", "Grow the selection")
            .example("pixelox mask expand doc.pxo --layer Photo --by 3 --in-place", "Grow a layer mask"),
        OpSpec::doc("mask contract", CAT, "Shrink a mask morphologically", contract)
            .about("Erode the mask by a disc of radius --by px (the canvas edge does not erode inward). Edits --mask NAME, --mask @sel, or --layer REF (that layer's mask); with neither, the active selection.")
            .param(target_mask_param())
            .param(target_layer_param())
            .param(param("by", ParamKind::Length).required().help("Shrink radius in px"))
            .example("pixelox mask contract doc.pxo --mask dog --by 2 --in-place", "Pull the edge in 2 px")
            .example("pixelox mask contract doc.pxo --mask @sel --by 5px --in-place", "Shrink the selection")
            .example("pixelox mask contract doc.pxo --layer Photo --by 1 --in-place", "Shrink a layer mask"),
        OpSpec::doc("mask smooth", CAT, "Median-smooth jagged mask edges", smooth)
            .about("Median filter over a square window of --radius px: removes specks and jaggies while keeping edges in place. Edits --mask NAME, --mask @sel, or --layer REF (that layer's mask); with neither, the active selection.")
            .param(target_mask_param())
            .param(target_layer_param())
            .param(param("radius", ParamKind::Length).required().help("Window radius in px"))
            .example("pixelox mask smooth doc.pxo --mask dog --radius 3 --in-place", "Clean up a mask")
            .example("pixelox mask smooth doc.pxo --mask @sel --radius 2 --in-place", "Smooth the selection")
            .example("pixelox mask smooth doc.pxo --layer Photo --radius 4 --in-place", "Smooth a layer mask"),
        OpSpec::doc("mask invert", CAT, "Invert a mask", invert)
            .about("Complement the mask (1 - coverage). Edits --mask NAME, --mask @sel, or --layer REF (that layer's mask); with neither, the active selection.")
            .param(target_mask_param())
            .param(target_layer_param())
            .example("pixelox mask invert doc.pxo --mask sky --in-place", "Invert a library mask")
            .example("pixelox mask invert doc.pxo --mask @sel --in-place", "Invert the selection")
            .example("pixelox mask invert doc.pxo --layer Photo --in-place", "Invert a layer mask"),
    ]
}

fn no_layer_mask(name: &str) -> Error {
    Error::new("PX2311", format!("layer '{name}' has no layer mask"))
        .with_hint(format!("add one first: pixelox mask add <doc> --layer '{name}' --reveal-all --in-place"))
        .with_context("layer", name)
}

/// What a refinement command edits.
enum Target {
    Sel,
    Named(String),
    Layer(LayerPath),
}

fn target(a: &Args, doc: &Document) -> Result<Target> {
    match (a.str("mask"), a.str("layer")) {
        (Some(_), Some(_)) => Err(Error::new("PX1005", "pass either --mask or --layer, not both")),
        (None, Some(l)) => {
            let p = doc.resolve(&l)?;
            let layer = doc.layer(&p);
            layer.check_unlocked()?;
            if layer.mask.is_none() {
                return Err(no_layer_mask(&layer.name));
            }
            Ok(Target::Layer(p))
        }
        (Some(m), None) => {
            let m = m.trim().to_string();
            if m == "@sel" {
                doc.named_mask("@sel")?;
                Ok(Target::Sel)
            } else if doc.masks.contains_key(&m) {
                Ok(Target::Named(m))
            } else if m.starts_with("@alpha:") || std::path::Path::new(&m).extension().is_some() {
                Err(Error::new(
                    "PX1010",
                    format!("'{m}' cannot be edited in place; refinement edits library masks, @sel, or a layer mask"),
                )
                .with_hint(format!("store it first: pixelox mask import <doc> {m} --as NAME --in-place")))
            } else {
                Err(doc.named_mask(&m).err().unwrap_or_else(|| Error::new("PX2311", format!("no mask named '{m}'"))))
            }
        }
        (None, None) => {
            if doc.selection.is_some() {
                Ok(Target::Sel)
            } else {
                Err(Error::new("PX1003", "say which mask to edit: --mask NAME, --mask @sel, or --layer REF")
                    .with_context("masks", doc.masks.keys().cloned().collect::<Vec<_>>()))
            }
        }
    }
}

fn label(doc: &Document, t: &Target) -> Value {
    match t {
        Target::Sel => json!({"mask": "@sel"}),
        Target::Named(n) => json!({"mask": n}),
        Target::Layer(p) => json!({"layer": doc.layer(p).name}),
    }
}

fn get(doc: &Document, t: &Target) -> Mask {
    let (w, h) = (doc.width, doc.height);
    match t {
        Target::Sel => fit_mask(doc.selection.clone().unwrap_or_else(|| Mask::new(w, h)), w, h),
        Target::Named(n) => fit_mask(doc.masks[n].clone(), w, h),
        Target::Layer(p) => fit_mask(
            doc.layer(p).mask.as_ref().map(|m| m.mask.clone()).unwrap_or_else(|| Mask::filled(w, h, 1.0)),
            w,
            h,
        ),
    }
}

fn set(doc: &mut Document, t: &Target, m: Mask) {
    match t {
        Target::Sel => doc.selection = Some(m),
        Target::Named(n) => {
            doc.masks.insert(n.clone(), m);
        }
        Target::Layer(p) => {
            let l = doc.layer_mut(p);
            if let Some(lm) = &mut l.mask {
                lm.mask = m;
            }
            // the edited pixels no longer match a vector mask's path
            l.vector_mask = None;
        }
    }
}

/// Rewrite the target's pixels with `f` and report it.
fn refine(a: &Args, doc: &mut Document, f: impl FnOnce(&Mask) -> Mask) -> Result<Value> {
    let t = target(a, doc)?;
    let m = f(&get(doc, &t));
    let mut v = label(doc, &t);
    v["stats"] = mask_json(&m);
    set(doc, &t, m);
    Ok(v)
}

fn list(_: &Args, doc: &mut Document, _: &mut OpCtx) -> Result<Value> {
    let masks: Vec<Value> = doc
        .masks
        .iter()
        .map(|(n, m)| {
            let mut j = mask_json(m);
            j["name"] = json!(n);
            j["size"] = json!([m.width, m.height]);
            j
        })
        .collect();
    let layer_masks: Vec<Value> = doc
        .walk()
        .into_iter()
        .filter_map(|(_, l)| {
            let lm = l.mask.as_ref()?;
            let mut j = mask_json(&lm.mask);
            j["layer"] = json!(l.name);
            j["id"] = json!(l.id);
            j["enabled"] = json!(lm.enabled);
            j["density"] = json!(lm.density);
            j["feather"] = json!(lm.feather);
            j["vector"] = json!(l.vector_mask.is_some());
            Some(j)
        })
        .collect();
    Ok(json!({
        "count": masks.len() + layer_masks.len(),
        "masks": masks,
        "layer_masks": layer_masks,
        "selection": doc.selection.as_ref().map(mask_json),
    }))
}

fn import(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let file = a.path("file").ok_or_else(|| Error::new("PX1003", "'mask import' needs FILE"))?;
    let name = a.str("as").unwrap_or_default().trim().to_string();
    let replaced = doc.masks.contains_key(&name);
    let m = decode_mask_file(ctx, doc, &file, "file")?;
    let j = mask_json(&m);
    doc.put_mask(&name, m, a.flag("replace"))?;
    Ok(json!({"mask": name, "replaced": replaced, "stats": j}))
}

fn export(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let r = a.str("mask").unwrap_or_default();
    let m = resolve_mask_ref(ctx, doc, &r, "mask")?;
    let out = ctx.globals.output.clone().ok_or_else(|| {
        Error::new("PX1004", "'mask export' writes an image; say where with -o").with_hint("pass -o mask.png")
    })?;
    let path = PathBuf::from(&out);
    let mut opts = EncodeOptions::for_path(&path);
    if let Some(f) = &ctx.globals.format {
        opts.format = f.clone();
    }
    let step = exec::make_step(&export_spec(), a, ctx);
    let img = util::mask_image(&m);
    ctx.write_image(&path, &img, Some(doc), &opts, Some("mask"), Some(exec::recipe_with(doc, step)))?;
    Ok(json!({"mask": r, "path": out, "size": [m.width, m.height], "stats": mask_json(&m)}))
}

fn rm(a: &Args, doc: &mut Document, _: &mut OpCtx) -> Result<Value> {
    let name = a.str("mask").unwrap_or_default().trim().to_string();
    if name == "@sel" {
        let had = doc.selection.take().is_some();
        return Ok(json!({"mask": "@sel", "removed": had}));
    }
    if doc.masks.remove(&name).is_none() {
        return Err(doc
            .named_mask(&name)
            .err()
            .unwrap_or_else(|| Error::new("PX2311", format!("no mask named '{name}'"))));
    }
    Ok(json!({"mask": name, "removed": true, "remaining": doc.masks.keys().collect::<Vec<_>>()}))
}

fn add(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let p = target_layer(a, doc)?;
    at_most_one(a, &["from", "from-file", "from-path"], &["reveal-all", "hide-all"])?;
    let (w, h) = (doc.width, doc.height);
    let (mask, source, vector) = if let Some(r) = a.str("from") {
        (resolve_mask_ref(ctx, doc, &r, "from")?, r, None)
    } else if let Some(f) = a.path("from-file") {
        (decode_mask_file(ctx, doc, &f, "from-file")?, f.display().to_string(), None)
    } else if let Some(d) = a.str("from-path") {
        let path = parse_path(&d)?;
        (fill_mask(w, h, &path, false), "path".to_string(), Some(d))
    } else if a.flag("hide-all") {
        (Mask::new(w, h), "hide-all".to_string(), None)
    } else if a.flag("reveal-all") || doc.selection.is_none() {
        (Mask::filled(w, h, 1.0), "reveal-all".to_string(), None)
    } else {
        (fit_mask(doc.named_mask("@sel")?, w, h), "@sel".to_string(), None)
    };
    let stats = mask_json(&mask);
    let l = doc.layer_mut(&p);
    let replaced = l.mask.is_some();
    l.mask = Some(LayerMask::new(mask));
    l.vector_mask = vector.clone();
    Ok(json!({"layer": l.name, "source": source, "vector": vector.is_some(), "replaced": replaced, "stats": stats}))
}

/// The mask's effect as rendered: live feather, then density.
fn effective(lm: &LayerMask) -> Mask {
    let mut m = util::feather(&lm.mask, lm.feather as f64);
    let d = lm.density / 100.0;
    m.data.par_iter_mut().for_each(|v| *v = 1.0 - d * (1.0 - *v));
    m
}

fn remove(a: &Args, doc: &mut Document, _: &mut OpCtx) -> Result<Value> {
    let p = target_layer(a, doc)?;
    let apply = a.flag("apply");
    let (w, h) = (doc.width, doc.height);
    let l = doc.layer_mut(&p);
    let name = l.name.clone();
    if l.mask.is_none() && l.vector_mask.is_none() {
        return Err(no_layer_mask(&name));
    }
    if apply && l.mask.is_some() {
        l.raster()?;
    }
    let lm = l.mask.take();
    l.vector_mask = None;
    let mut applied = false;
    if let (true, Some(mut lm)) = (apply, lm) {
        lm.mask = fit_mask(lm.mask, w, h);
        let m = effective(&lm);
        let (ox, oy) = (l.x as i64, l.y as i64);
        let img = l.raster_mut()?;
        let iw = img.width as usize;
        img.data.par_chunks_mut(iw * 4).enumerate().for_each(|(y, row)| {
            let cy = y as i64 + oy;
            if cy < 0 || cy >= h as i64 {
                return;
            }
            for x in 0..iw {
                let cx = x as i64 + ox;
                if cx < 0 || cx >= w as i64 {
                    continue;
                }
                row[x * 4 + 3] *= m.get(cx as u32, cy as u32);
            }
        });
        applied = true;
    }
    Ok(json!({"layer": name, "removed": true, "applied": applied}))
}

fn set_enabled(a: &Args, doc: &mut Document, on: bool) -> Result<Value> {
    let p = target_layer(a, doc)?;
    let l = doc.layer_mut(&p);
    let name = l.name.clone();
    let lm = l.mask.as_mut().ok_or_else(|| no_layer_mask(&name))?;
    let changed = lm.enabled != on;
    lm.enabled = on;
    Ok(json!({"layer": name, "enabled": on, "changed": changed}))
}

fn enable(a: &Args, doc: &mut Document, _: &mut OpCtx) -> Result<Value> {
    set_enabled(a, doc, true)
}

fn disable(a: &Args, doc: &mut Document, _: &mut OpCtx) -> Result<Value> {
    set_enabled(a, doc, false)
}

fn density(a: &Args, doc: &mut Document, _: &mut OpCtx) -> Result<Value> {
    let value = a.f64("value")? as f32;
    let t = target(a, doc)?;
    if let Target::Layer(p) = &t {
        let l = doc.layer_mut(p);
        if let Some(lm) = &mut l.mask {
            lm.density = value;
        }
        return Ok(json!({"layer": l.name, "density": value, "live": true}));
    }
    let d = value / 100.0;
    let mut m = get(doc, &t);
    m.data.par_iter_mut().for_each(|v| *v = 1.0 - d * (1.0 - *v));
    let mut v = label(doc, &t);
    v["density"] = json!(value);
    v["live"] = json!(false);
    v["stats"] = mask_json(&m);
    set(doc, &t, m);
    Ok(v)
}

fn feather(a: &Args, doc: &mut Document, _: &mut OpCtx) -> Result<Value> {
    let r = util::len_px(a, "radius", doc)?;
    let t = target(a, doc)?;
    if let Target::Layer(p) = &t {
        let l = doc.layer_mut(p);
        if let Some(lm) = &mut l.mask {
            lm.feather = r as f32;
        }
        return Ok(json!({"layer": l.name, "radius": r3(r), "live": true}));
    }
    let mut v = refine(a, doc, |m| util::feather(m, r))?;
    v["radius"] = json!(r3(r));
    v["live"] = json!(false);
    Ok(v)
}

fn expand(a: &Args, doc: &mut Document, _: &mut OpCtx) -> Result<Value> {
    let by = util::len_px(a, "by", doc)?;
    let mut v = refine(a, doc, |m| kernels::dilate(m, by))?;
    v["by"] = json!(r3(by));
    Ok(v)
}

fn contract(a: &Args, doc: &mut Document, _: &mut OpCtx) -> Result<Value> {
    let by = util::len_px(a, "by", doc)?;
    let mut v = refine(a, doc, |m| kernels::erode(m, by))?;
    v["by"] = json!(r3(by));
    Ok(v)
}

fn smooth(a: &Args, doc: &mut Document, _: &mut OpCtx) -> Result<Value> {
    let r = util::len_px(a, "radius", doc)?.round() as usize;
    let mut v = refine(a, doc, |m| kernels::median_mask(m, r))?;
    v["radius"] = json!(r);
    Ok(v)
}

fn invert(a: &Args, doc: &mut Document, _: &mut OpCtx) -> Result<Value> {
    refine(a, doc, |m| m.inverted())
}
