//! `filter grain`: film grain (`pixelox_core::grain`) on a layer's pixels, as a smart
//! filter on placed layers, or as a grain adjustment layer that grains everything below.

use pixelox_core::grain::{self, Grain};
use pixelox_core::prelude::*;
use rand_core::RngCore;

pub fn ops() -> Vec<OpSpec> {
    vec![OpSpec::doc("filter grain", Category::Filter, "Film grain (size-aware, tone-weighted)", grain_op)
        .about(
            "Adds film grain: soft clumps about --size px across (auto: 1.5 px on a 1080 px canvas, growing with \
             the resolution), --amount strong (the same visible strength at every size), strongest in the \
             midtones and fading toward pure black and white (--tone 0 grains every tone evenly). --roughness mixes \
             in a finer, crisper layer; --color 0 is monochrome grain, 100 independent color grain. The grain is \
             anchored to the canvas and seeded (--seed). On a placed layer it is recorded as a smart filter. \
             --as-layer instead adds a 'Grain' adjustment layer above --layer (or the active layer, or on top) that \
             grains everything below it, live: the usual last step of a poster, keeping small text above it clean.",
        )
        .param(
            param("amount", ParamKind::Float)
                .default("30")
                .range("0..400")
                .help("Strength: 15 subtle, 30 visible print grain, 60 heavy (0 = no change)"),
        )
        .param(
            param("size", ParamKind::Float)
                .auto()
                .default("auto")
                .range("0.3..100")
                .help("Grain size in px; auto scales with the canvas (1.5 px at 1080 px)"),
        )
        .param(
            param("roughness", ParamKind::Float)
                .default("50")
                .range("0..100")
                .help("0 = soft round clumps, 100 = crisp grain with fine detail"),
        )
        .param(
            param("color", ParamKind::Float)
                .default("0")
                .range("0..100")
                .help("Color grain: 0 = monochrome, 100 = independent per channel"),
        )
        .param(
            param("tone", ParamKind::Float)
                .default("60")
                .range("0..100")
                .help("How much the grain follows the tone: 0 = even, 100 = none in pure black or white"),
        )
        .param(
            common::as_layer()
                .help("Add a live grain adjustment layer that grains everything below it instead of changing pixels"),
        )
        .param(common::name())
        .params(common::region())
        .example("pixelox filter grain photo.jpg -o grainy.jpg", "Default film grain")
        .example(
            "pixelox filter grain doc.pxo --in-place --as-layer --amount 25",
            "Grain the whole poster with one live layer",
        )
        .example(
            "pixelox filter grain doc.pxo --in-place --layer Photo --amount 45 --size 3 --color 30",
            "Coarse, slightly colored grain on one layer",
        )
        .example(
            "pixelox filter grain doc.pxo --in-place --as-layer --layer Background --amount 15 --tone 90",
            "Subtle grain under the type, sparing blacks and whites",
        )]
}

fn grain_op(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let short = doc.width.min(doc.height).max(1) as f64;
    let size = match a.opt_f64("size")? {
        Some(s) => s,
        None => {
            let s = (grain::auto_size(short) * 1000.0).round() / 1000.0;
            ctx.resolve_param("size", s);
            s
        }
    };
    let g = Grain {
        amount: a.f64("amount")?,
        size,
        roughness: a.f64("roughness")?,
        color: a.f64("color")?,
        tone: a.f64("tone")?,
        seed: ctx.rng().next_u32() as u64,
    };
    let report = json!({"size": size, "variant": g.seed});
    if a.flag("as-layer") {
        return as_layer(a, doc, ctx, g, report);
    }
    if a.has("name") {
        return Err(Error::new("PX1005", "--name names the layer made by --as-layer")
            .with_hint("add --as-layer to make a grain adjustment layer, or drop --name"));
    }
    let path = target_layer(a, doc)?;
    let origin = {
        let l = doc.layer(&path);
        (l.x as i64, l.y as i64)
    };
    let mut v = apply_to_raster(a, doc, ctx, |img, _| {
        let mut out = img.clone();
        g.apply(&mut out, origin);
        Ok(out)
    })?;
    if let (Value::Object(m), Value::Object(r)) = (&mut v, report) {
        m.extend(r);
    }
    Ok(v)
}

fn as_layer(a: &Args, doc: &mut Document, ctx: &mut OpCtx, g: Grain, report: Value) -> Result<Value> {
    let above = match a.raw("layer") {
        Some(r) => Some(doc.resolve(r)?),
        None => doc.active.clone().and_then(|id| doc.find_by_id(&id)),
    };
    let mask = ctx.mask_arg(a, doc)?;
    let name = match a.str("name").map(|n| n.trim().to_string()).filter(|n| !n.is_empty()) {
        Some(n) => {
            doc.check_name_free(&n, None)?;
            n
        }
        None => "Grain".to_string(),
    };
    let params = g.to_params();
    let mut layer = doc.new_layer(&name, LayerKind::Adjustment { op: "grain".into(), params: params.clone() });
    if let Some(m) = mask {
        layer.mask = Some(LayerMask::new(m));
    }
    let (id, lname) = (layer.id.clone(), layer.name.clone());
    doc.insert_above(above.as_deref(), layer);
    let mut v =
        json!({"layer": lname, "layer_id": id, "as_layer": true, "adjustment": {"op": "grain", "params": params}});
    if let Some(p) = &above {
        v["above"] = json!(doc.layer(p).name);
    }
    if let (Value::Object(m), Value::Object(r)) = (&mut v, report) {
        m.extend(r);
    }
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pixelox_core::testing::{self, find, raster, run};

    fn gray(w: u32, h: u32) -> Image {
        Image::filled(w, h, Rgba::new(0.5, 0.5, 0.5, 1.0))
    }

    #[test]
    fn grain_changes_pixels_and_is_seeded() {
        let spec = find(&ops(), "filter grain");
        let mut a = testing::doc_from(gray(64, 48));
        let v = run(&spec, &[], &mut a).unwrap();
        assert!(v["size"].as_f64().unwrap() >= 1.0);
        let mut b = testing::doc_from(gray(64, 48));
        run(&spec, &[], &mut b).unwrap();
        assert_eq!(raster(&a, "Background"), raster(&b, "Background"));
        assert!(testing::max_diff(raster(&a, "Background"), &gray(64, 48)) > 0.02);
        // amount 0 is the identity
        let mut c = testing::doc_from(gray(16, 16));
        run(&spec, &[("amount", "0")], &mut c).unwrap();
        assert_eq!(raster(&c, "Background"), &gray(16, 16));
    }

    #[test]
    fn as_layer_adds_a_grain_adjustment() {
        let spec = find(&ops(), "filter grain");
        let mut d = testing::doc_from(gray(32, 32));
        let v = run(&spec, &[("as-layer", "true"), ("amount", "40")], &mut d).unwrap();
        assert_eq!(v["layer"], "Grain");
        let l = d.layer(&d.resolve("Grain").unwrap());
        match &l.content {
            LayerKind::Adjustment { op, params } => {
                assert_eq!(op, "grain");
                assert_eq!(params["amount"], "40");
            }
            other => panic!("not an adjustment layer: {other:?}"),
        }
        assert_eq!(run(&spec, &[("name", "G")], &mut d).unwrap_err().code, "PX1005");
    }
}
