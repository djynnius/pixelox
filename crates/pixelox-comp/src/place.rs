//! `comp place`, `comp relink`, `comp update-links` (PRD §8.14): placed (smart-object-like)
//! layers. The source bytes are always embedded in the document (so it renders anywhere);
//! `--link` additionally keeps the path so `update-links` can refresh it when the file
//! changes.

use crate::util::*;
use pixelox_core::ctx::is_pxo;
use pixelox_core::envelope::sha256_file;
use pixelox_core::exec;
use pixelox_core::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::Arc;

const FITS: &[&str] = &["cover", "contain", "fill"];

pub fn ops() -> Vec<OpSpec> {
    vec![
        OpSpec::doc("comp place", Category::Comp, "Place an image file as a new placed layer", place)
            .about(
                "Imports an image (any readable format, SVG included) as a placed layer above the active layer. \
                 The file is embedded in the document; `--link` also records it as linked so `comp \
                 update-links` re-reads it when it changes. By default the image keeps its natural size \
                 (scaled down to fit the canvas when larger) and is centered. `--size WxH` sets the size (`%` \
                 of the canvas; a blank side keeps the aspect: `220x`); `--fit` fits it to the canvas instead. \
                 `--at X,Y` puts the `--anchor` point of the image there (default anchor `tl`; negative values \
                 count from the right/bottom); `--anchor` alone snaps to that canvas corner or edge.",
            )
            .param(param("file", ParamKind::Path).required().help("Image file to place"))
            .param(param("as", ParamKind::Str).help("Name for the placed layer (default: the file name without extension)"))
            .param(param("at", ParamKind::Point).help("Position X,Y of the anchor point; negative values count from the right/bottom"))
            .param(param("anchor", ParamKind::Choice).choices(units::ANCHORS).help("Which point of the image sits at --at (default tl; alone: align to that canvas point)"))
            .param(param("size", ParamKind::Size).help("Placed size WxH in px or % of the canvas; a blank side keeps the aspect"))
            .param(param("fit", ParamKind::Choice).choices(FITS).help("Fit the image to the canvas instead of --size: cover, contain, or fill"))
            .param(param("link", ParamKind::Flag).help("Keep a link to the file so `comp update-links` can refresh it"))
            .param(param("embed", ParamKind::Flag).help("Embed only, without a link (the default)"))
            .positional("file")
            .example(
                "pixelox comp place photo.jpg brand/logo.png -o marked.pxo --as Watermark --anchor br --at -48,-48 --size 220x",
                "Watermark: a 220 px wide logo 48 px from the bottom-right corner",
            )
            .example("pixelox comp place doc.pxo hero.jpg --in-place --fit cover", "Fill the canvas with a photo")
            .example("pixelox comp place doc.pxo logo.svg --in-place --as Logo --at 50%,120 --anchor t --link", "Place a linked logo centered near the top"),
        OpSpec::doc("comp relink", Category::Comp, "Point a placed layer at a different file", relink)
            .about(
                "Replaces the source of a placed layer with another file (embedded; the link flag is kept). \
                 When the new image has the same aspect ratio it fills the same box; otherwise it is fitted \
                 inside the old box, centered. Locked layers are kept (PX2304).",
            )
            .param(common::layer())
            .param(param("to", ParamKind::Path).required().help("The new source file"))
            .example("pixelox comp relink doc.pxo --in-place --layer Logo --to brand/logo-2027.png", "Swap in a new logo")
            .example("pixelox comp relink doc.pxo -o v2.pxo --layer Product --to shots/hero-v2.jpg", "Replace a product shot")
            .example("pixelox comp relink doc.pxo --in-place --layer '#2' --to art/badge.svg --json", "Relink the second layer"),
        OpSpec::custom("comp update-links", Category::Comp, "Refresh linked placed layers whose files changed", update_links)
            .about(
                "Hashes the file behind every linked placed layer and compares it with the embedded copy. \
                 Changed files are re-read and re-embedded (write with `-o` or `--in-place`); `--check` only \
                 reports drift and writes nothing. Each link is reported as `ok`, `changed`, `updated`, or \
                 `missing`.",
            )
            .param(param("check", ParamKind::Flag).help("Only report drift (sha256 of each linked file); write nothing"))
            .example("pixelox comp update-links doc.pxo --check --json", "Report which linked files changed")
            .example("pixelox comp update-links doc.pxo --in-place", "Re-embed every changed linked file")
            .example("pixelox comp update-links doc.pxo -o refreshed.pxo --json", "Refresh links into a new document"),
    ]
}

fn file_stem(file: &str) -> String {
    Path::new(file)
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| "Placed".into())
}

fn place(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let file = a
        .str("file")
        .ok_or_else(|| Error::new("PX1003", "'comp place' needs a FILE to place").with_context("param", "file"))?;
    let link = a.flag("link");
    if link && a.flag("embed") {
        return Err(Error::new("PX1005", "pass either --link or --embed, not both"));
    }
    let size = a.size("size")?;
    let fit = a.str("fit");
    if size.is_some() && fit.is_some() {
        return Err(Error::new("PX1005", "pass either --size or --fit, not both"));
    }
    let name = match a.str("as") {
        Some(n) => fresh_name(doc, &n, &[])?,
        None => {
            let stem = file_stem(&file);
            match validate_name(&stem) {
                Ok(s) => doc.unique_name(&s),
                Err(_) => doc.unique_name("Placed"),
            }
        }
    };
    let (img, bytes, ext) = read_image(ctx, &file, "file", Some("file"))?;
    let (iw, ih) = (img.width as f64, img.height as f64);
    let (cw, ch, dpi) = (doc.width as f64, doc.height as f64, doc.dpi());
    let (w, h) = match (size, fit.as_deref()) {
        (Some(s), _) => match (s.w, s.h) {
            (Some(w), Some(h)) => (w.to_px(cw, dpi), h.to_px(ch, dpi)),
            (Some(w), None) => {
                let w = w.to_px(cw, dpi);
                (w, w * ih / iw)
            }
            (None, Some(h)) => {
                let h = h.to_px(ch, dpi);
                (h * iw / ih, h)
            }
            (None, None) => (iw, ih),
        },
        (None, Some(f)) => {
            let r = fit_rect(iw, ih, 0.0, 0.0, cw, ch, f);
            (r[2], r[3])
        }
        (None, None) => {
            let s = (cw / iw).min(ch / ih).min(1.0);
            (iw * s, ih * s)
        }
    };
    if !(w > 0.0 && h > 0.0) {
        return Err(Error::new("PX1013", format!("--size resolves to {w}x{h}; both sides must be positive"))
            .with_context("param", "size"));
    }
    let (w, h) = (w.round().max(1.0), h.round().max(1.0));
    let anchor = match a.str("anchor") {
        Some(s) => Some(Anchor::parse(&s)?),
        None => None,
    };
    let (x, y) = match (a.point("at")?, anchor) {
        (Some(p), anc) => {
            let (px, py) = p.resolve(cw, ch, dpi);
            anc.unwrap_or(Anchor::Tl).place(px, py, w, h)
        }
        (None, Some(anc)) => {
            let (fx, fy) = anc.fractions();
            anc.place(fx * cw, fy * ch, w, h)
        }
        (None, None) => ((cw - w) / 2.0, (ch - h) / 2.0),
    };
    let rect = [x.round(), y.round(), w, h];
    let asset = doc.add_asset(bytes, &ext);
    let l = doc.new_layer(
        &name,
        LayerKind::Placed {
            asset: asset.clone(),
            path: file.clone(),
            linked: link,
            rect,
            angle: 0.0,
            source: Some(Arc::new(img)),
        },
    );
    let id = l.id.clone();
    let p = insert_new(doc, l);
    Ok(json!({
        "layer": name,
        "id": id,
        "rect": rect,
        "natural_size": [iw, ih],
        "scale": round3(w / iw),
        "linked": link,
        "asset": asset,
        "parent": parent_name(doc, &p),
    }))
}

fn relink(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let p = layer_path(a, doc)?;
    let l = doc.layer(&p);
    l.check_unlocked()?;
    let LayerKind::Placed { rect: old, .. } = &l.content else {
        return Err(Error::new(
            "PX2303",
            format!("layer '{}' is a {} layer; relink needs a placed layer", l.name, l.kind_name()),
        )
        .with_hint("place the file as a new layer: pixelox comp place <doc> FILE --in-place")
        .with_context("layer", &l.name)
        .with_context("kind", l.kind_name()));
    };
    let old = *old;
    let to = a.str_or("to", "");
    let (img, bytes, ext) = read_image(ctx, &to, "file", Some("to"))?;
    let (iw, ih) = (img.width as f64, img.height as f64);
    let same_aspect = old[3] > 0.0 && ((iw / ih) / (old[2] / old[3]) - 1.0).abs() < 0.01;
    let rect = if same_aspect { old } else { fit_rect(iw, ih, old[0], old[1], old[2], old[3], "contain") };
    let key = doc.add_asset(bytes, &ext);
    let l = doc.layer_mut(&p);
    if let LayerKind::Placed { asset, path, rect: r, source, linked, .. } = &mut l.content {
        *asset = key.clone();
        *path = to.clone();
        *r = rect;
        *source = Some(Arc::new(img));
        Ok(json!({"layer": l.name, "path": to, "asset": key, "rect": rect, "previous_rect": old, "linked": *linked}))
    } else {
        Err(Error::internal("placed layer changed kind"))
    }
}

/// The work of `comp update-links` on a loaded document.
pub(crate) fn update_links_doc(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let check = a.flag("check");
    let links: Vec<(String, String, String, String)> = doc
        .walk()
        .iter()
        .filter_map(|(_, l)| match &l.content {
            LayerKind::Placed { asset, path, linked: true, .. } => {
                Some((l.id.clone(), l.name.clone(), path.clone(), asset.clone()))
            }
            _ => None,
        })
        .collect();
    let mut report = Vec::new();
    let (mut changed, mut missing, mut updated) = (0, 0, 0);
    for (id, name, file, recorded) in links {
        let path = Path::new(&file);
        ctx.check_allowed(path)?;
        if !path.is_file() {
            missing += 1;
            report.push(json!({"layer": name, "path": file, "status": "missing", "recorded": recorded}));
            continue;
        }
        let current = sha256_file(path)?;
        if current == recorded {
            report.push(json!({"layer": name, "path": file, "status": "ok", "sha256": current}));
            continue;
        }
        changed += 1;
        if check {
            report.push(
                json!({"layer": name, "path": file, "status": "changed", "recorded": recorded, "current": current}),
            );
            continue;
        }
        let lp = path_of(doc, &id)?;
        doc.layer(&lp).check_unlocked()?;
        let (img, bytes, ext) = read_image(ctx, &file, "link", None)?;
        let key = doc.add_asset(bytes, &ext);
        if let LayerKind::Placed { asset, source, .. } = &mut doc.layer_mut(&lp).content {
            *asset = key.clone();
            *source = Some(Arc::new(img));
        }
        updated += 1;
        report.push(json!({"layer": name, "path": file, "status": "updated", "recorded": recorded, "current": key}));
    }
    Ok(json!({
        "links": report,
        "linked": report.len(),
        "changed": changed,
        "missing": missing,
        "updated": updated,
        "drift": changed + missing > 0,
        "check": check,
    }))
}

fn update_links_spec() -> OpSpec {
    ops().into_iter().find(|o| o.name == "comp update-links").expect("update-links spec")
}

/// Where a refreshing (non-`--check`) run writes: `--in-place`, `-o`, or nowhere in a dry run.
fn update_target(ctx: &OpCtx, input: &Path, check: bool) -> Result<Option<PathBuf>> {
    let g = &ctx.globals;
    if check {
        if g.in_place || g.output.is_some() {
            return Err(Error::new(
                "PX1005",
                "--check only reports and writes nothing; drop -o / --in-place, or drop --check to refresh",
            ));
        }
        return Ok(None);
    }
    if g.in_place {
        if g.output.is_some() {
            return Err(Error::new("PX1005", "pass either -o or --in-place, not both"));
        }
        if !is_pxo(input) {
            return Err(Error::new(
                "PX1108",
                format!("{} is not a .pxo document; write a new file with -o", input.display()),
            ));
        }
        ctx.prepare_output(input)?;
        return Ok(Some(input.to_path_buf()));
    }
    match &g.output {
        Some(o) => {
            let p = PathBuf::from(o);
            ctx.prepare_output(&p)?;
            Ok(Some(p))
        }
        None if g.dry_run => Ok(None),
        None => Err(Error::new("PX1004", "'comp update-links' changes the document; say where to write it")
            .with_hint("pass --in-place (or -o <path>), or --check to only report drift")),
    }
}

/// `comp update-links` reads its own document so `--check` needs no output.
fn update_links(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let input = ctx.input(0)?.to_path_buf();
    let check = a.flag("check");
    let target = update_target(ctx, &input, check)?;
    let mut doc = ctx.load_document(&input)?;
    ctx.op_index = doc.recipe.len();
    let mut v = update_links_doc(a, &mut doc, ctx)?;
    if !check {
        let step = exec::make_step(&update_links_spec(), a, ctx);
        doc.recipe.push(step);
        if let Some(t) = &target {
            exec::write_document(ctx, &doc, t)?;
        }
    }
    let mut summary = doc.summary();
    summary["path"] = json!(target.as_deref().unwrap_or(&input).display().to_string());
    v["document"] = summary;
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testio::{encode, run_custom, run_io, TestIo};
    use pixelox_core::ctx::Globals;
    use pixelox_core::testing::{self, find, TempDir};

    fn op(name: &str) -> OpSpec {
        find(&ops(), name)
    }

    fn canvas(w: u32, h: u32) -> Document {
        testing::doc_from(Image::filled(w, h, Rgba::WHITE))
    }

    fn file(dir: &TempDir, name: &str, w: u32, h: u32, c: Rgba) -> String {
        let p = dir.path(name);
        std::fs::write(&p, encode(&Image::filled(w, h, c))).unwrap();
        p.display().to_string()
    }

    fn rect(doc: &Document, name: &str) -> [f64; 4] {
        match &doc.layer(&doc.resolve(name).unwrap()).content {
            LayerKind::Placed { rect, .. } => *rect,
            _ => panic!("not placed"),
        }
    }

    #[test]
    fn place_natural_size_centered_and_embedded() {
        let dir = TempDir::new("place");
        let f = file(&dir, "logo.pxt", 40, 20, Rgba::BLACK);
        let mut d = canvas(100, 80);
        let (v, _) = run_io(&op("comp place"), &[("file", &f)], &mut d, &TestIo::default()).unwrap();
        assert_eq!(v["layer"], "logo");
        assert_eq!(rect(&d, "logo"), [30.0, 30.0, 40.0, 20.0]);
        assert_eq!(d.layers.len(), 2);
        assert_eq!(d.walk()[0].1.name, "logo");
        let l = d.layer(&d.resolve("logo").unwrap());
        let LayerKind::Placed { asset, linked, .. } = &l.content else { panic!() };
        assert!(!linked);
        assert!(d.assets.contains_key(asset));
        // it renders
        assert_eq!(compose::composite(&d).unwrap().get(50, 40), [0.0, 0.0, 0.0, 1.0]);
        // a second placement of the same file gets a unique name
        let (v, _) = run_io(&op("comp place"), &[("file", &f), ("link", "true")], &mut d, &TestIo::default()).unwrap();
        assert_eq!(v["layer"], "logo 2");
        assert_eq!(v["linked"], true);
    }

    #[test]
    fn place_scales_large_images_down_and_fits() {
        let dir = TempDir::new("place-fit");
        let f = file(&dir, "big.pxt", 400, 100, Rgba::BLACK);
        let io = TestIo::default();
        let mut d = canvas(100, 80);
        run_io(&op("comp place"), &[("file", &f), ("as", "Big")], &mut d, &io).unwrap();
        assert_eq!(rect(&d, "Big"), [0.0, 28.0, 100.0, 25.0]);
        run_io(&op("comp place"), &[("file", &f), ("as", "Cover"), ("fit", "cover")], &mut d, &io).unwrap();
        assert_eq!(rect(&d, "Cover"), [-110.0, 0.0, 320.0, 80.0]);
        run_io(&op("comp place"), &[("file", &f), ("as", "Fill"), ("fit", "fill")], &mut d, &io).unwrap();
        assert_eq!(rect(&d, "Fill"), [0.0, 0.0, 100.0, 80.0]);
        run_io(&op("comp place"), &[("file", &f), ("as", "Contain"), ("fit", "contain")], &mut d, &io).unwrap();
        assert_eq!(rect(&d, "Contain"), [0.0, 28.0, 100.0, 25.0]);
    }

    #[test]
    fn place_e7_watermark_anchor_and_negative_at() {
        let dir = TempDir::new("place-e7");
        let f = file(&dir, "logo.pxt", 440, 220, Rgba::BLACK);
        let mut d = canvas(1000, 800);
        let pairs = [("file", f.as_str()), ("as", "Watermark"), ("anchor", "br"), ("at", "-48,-48"), ("size", "220x")];
        run_io(&op("comp place"), &pairs, &mut d, &TestIo::default()).unwrap();
        assert_eq!(rect(&d, "Watermark"), [732.0, 642.0, 220.0, 110.0]);
        // anchor alone snaps to that canvas point; --at alone uses the top-left corner
        run_io(
            &op("comp place"),
            &[("file", &f), ("as", "TR"), ("anchor", "tr"), ("size", "x50")],
            &mut d,
            &TestIo::default(),
        )
        .unwrap();
        assert_eq!(rect(&d, "TR"), [900.0, 0.0, 100.0, 50.0]);
        run_io(
            &op("comp place"),
            &[("file", &f), ("as", "At"), ("at", "10%,20"), ("size", "10x10")],
            &mut d,
            &TestIo::default(),
        )
        .unwrap();
        assert_eq!(rect(&d, "At"), [100.0, 20.0, 10.0, 10.0]);
    }

    #[test]
    fn place_errors() {
        let dir = TempDir::new("place-err");
        let f = file(&dir, "a.pxt", 4, 4, Rgba::BLACK);
        let io = TestIo::default();
        let mut d = canvas(10, 10);
        let e =
            run_io(&op("comp place"), &[("file", &f), ("link", "true"), ("embed", "true")], &mut d, &io).unwrap_err();
        assert_eq!(e.code, "PX1005");
        let e = run_io(&op("comp place"), &[("file", &f), ("size", "4x4"), ("fit", "cover")], &mut d, &io).unwrap_err();
        assert_eq!(e.code, "PX1005");
        let e = run_io(&op("comp place"), &[("file", &f), ("as", "Background")], &mut d, &io).unwrap_err();
        assert_eq!(e.code, "PX2305");
        let missing = dir.path("missing.pxt").display().to_string();
        let e = run_io(&op("comp place"), &[("file", &missing)], &mut d, &io).unwrap_err();
        assert_eq!(e.code, "PX1101");
        assert_eq!(d.layers.len(), 1);
    }

    #[test]
    fn relink_swaps_the_source() {
        let dir = TempDir::new("relink");
        let a = file(&dir, "a.pxt", 40, 20, Rgba::BLACK);
        let b = file(&dir, "b.pxt", 80, 40, Rgba::new(1.0, 0.0, 0.0, 1.0));
        let c = file(&dir, "c.pxt", 20, 20, Rgba::WHITE);
        let io = TestIo::default();
        let mut d = canvas(100, 80);
        run_io(&op("comp place"), &[("file", &a), ("as", "Logo")], &mut d, &io).unwrap();
        let (v, _) = run_io(&op("comp relink"), &[("layer", "Logo"), ("to", &b)], &mut d, &io).unwrap();
        assert_eq!(v["rect"], json!([30.0, 30.0, 40.0, 20.0]));
        assert_eq!(compose::composite(&d).unwrap().get(50, 40), [1.0, 0.0, 0.0, 1.0]);
        let (v, _) = run_io(&op("comp relink"), &[("layer", "Logo"), ("to", &c)], &mut d, &io).unwrap();
        assert_eq!(v["rect"], json!([40.0, 30.0, 20.0, 20.0]));
        let LayerKind::Placed { path, .. } = &d.layer(&d.resolve("Logo").unwrap()).content else { panic!() };
        assert_eq!(path, &c);
        let e = run_io(&op("comp relink"), &[("layer", "Background"), ("to", &c)], &mut d, &io).unwrap_err();
        assert_eq!(e.code, "PX2303");
        let p = d.resolve("Logo").unwrap();
        d.layer_mut(&p).locked = true;
        assert_eq!(
            run_io(&op("comp relink"), &[("layer", "Logo"), ("to", &a)], &mut d, &io).unwrap_err().code,
            "PX2304"
        );
    }

    fn linked_doc(dir: &TempDir) -> (Document, String) {
        let f = file(dir, "link.pxt", 10, 10, Rgba::BLACK);
        let mut d = canvas(20, 20);
        run_io(&op("comp place"), &[("file", &f), ("as", "L"), ("link", "true")], &mut d, &TestIo::default()).unwrap();
        run_io(&op("comp place"), &[("file", &f), ("as", "E")], &mut d, &TestIo::default()).unwrap();
        (d, f)
    }

    #[test]
    fn update_links_check_refresh_and_missing() {
        let dir = TempDir::new("links");
        let (mut d, f) = linked_doc(&dir);
        let spec = op("comp update-links");
        let io = TestIo::default();
        let run = |pairs: &[(&str, &str)], d: &mut Document| {
            let a = testing::args(&spec, pairs).unwrap();
            let mut ctx = OpCtx::new(spec.name, vec![], Globals::default(), &io, &pixelox_core::ctx::NoProgress);
            update_links_doc(&a, d, &mut ctx)
        };
        let v = run(&[("check", "true")], &mut d).unwrap();
        assert_eq!((v["linked"].clone(), v["drift"].clone()), (json!(1), json!(false)));
        assert_eq!(v["links"][0]["status"], "ok");
        std::fs::write(&f, encode(&Image::filled(10, 10, Rgba::WHITE))).unwrap();
        let before = d.layer(&d.resolve("L").unwrap()).clone();
        let v = run(&[("check", "true")], &mut d).unwrap();
        assert_eq!(v["links"][0]["status"], "changed");
        assert_eq!(v["drift"], true);
        let LayerKind::Placed { asset: a0, .. } = &before.content else { panic!() };
        let LayerKind::Placed { asset: a1, .. } = &d.layer(&d.resolve("L").unwrap()).content else { panic!() };
        assert_eq!(a0, a1, "--check changes nothing");
        let v = run(&[], &mut d).unwrap();
        assert_eq!(v["updated"], 1);
        let c = compose::composite(&d).unwrap();
        assert_eq!(c.get(10, 10), [0.0, 0.0, 0.0, 1.0], "the embedded copy E is on top and unchanged");
        let p = d.resolve("E").unwrap();
        d.layer_mut(&p).visible = false;
        assert_eq!(compose::composite(&d).unwrap().get(10, 10), [1.0, 1.0, 1.0, 1.0], "L now shows the new file");
        std::fs::remove_file(&f).unwrap();
        let v = run(&[], &mut d).unwrap();
        assert_eq!(v["links"][0]["status"], "missing");
        assert_eq!(v["missing"], 1);
    }

    #[test]
    fn update_links_command_check_needs_no_output() {
        let dir = TempDir::new("links-cmd");
        let (d, _f) = linked_doc(&dir);
        let doc_path = dir.path("doc.pxo");
        let io = TestIo::default();
        io.docs.lock().unwrap().insert(doc_path.clone(), d);
        let input = vec![doc_path.clone()];
        let (v, _) =
            run_custom(&op("comp update-links"), &[("check", "true")], input.clone(), Globals::default(), &io).unwrap();
        assert_eq!(v["check"], true);
        assert!(io.saved.lock().unwrap().is_empty());
        let e = run_custom(&op("comp update-links"), &[], input.clone(), Globals::default(), &io).unwrap_err();
        assert_eq!(e.code, "PX1004");
        let g = Globals { output: Some(dir.path("out.pxo").display().to_string()), ..Globals::default() };
        run_custom(&op("comp update-links"), &[], input.clone(), g.clone(), &io).unwrap();
        let saved = io.saved.lock().unwrap();
        assert_eq!(saved.len(), 1);
        assert_eq!(saved[0].recipe.last().unwrap().op, "comp.update-links");
        drop(saved);
        let e = run_custom(&op("comp update-links"), &[("check", "true")], input, g, &io).unwrap_err();
        assert_eq!(e.code, "PX1005");
    }
}
