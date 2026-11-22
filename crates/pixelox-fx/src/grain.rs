//! `fx grain`: film grain as a live layer style. It works on any layer kind (text,
//! shapes, placed images, groups) without rasterizing: the renderer adds the grain to
//! everything the layer draws, after its other effects, anchored to the layer's content
//! so the pattern moves with the layer. The grain itself is `pixelox_core::grain`, the
//! same as `filter grain`.

use crate::commands::set_style;
use pixelox_core::doc::FxSpec;
use pixelox_core::grain::Grain;
use pixelox_core::prelude::*;

pub(crate) fn ops() -> Vec<OpSpec> {
    vec![OpSpec::doc("fx grain", Category::Fx, "Add or replace a layer's film grain", grain)
        .about(
            "Live film grain on a layer of any kind (text and shapes stay editable): soft clumps about --size px \
             across (auto: 1.5 px on a 1080 px canvas, growing with the resolution), strongest in the midtones \
             (--tone), monochrome or colored (--color), over everything the layer draws including its other \
             effects. The grain is anchored to the layer, so moving the layer moves its grain with it. --variant \
             fixes the pattern (stored with the style). Remove it with fx rm --kind grain; for grain over a whole \
             design use `filter grain --as-layer`.",
        )
        .param(common::layer())
        .param(
            param("amount", ParamKind::Float)
                .default("30")
                .range("0..400")
                .help("Strength: 15 subtle, 30 visible print grain, 60 heavy"),
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
        .param(param("variant", ParamKind::Int).range("0..4294967295").auto().default("auto").help(
            "Grain pattern number: auto keeps the layer's current pattern or picks one from the op's seed (--seed)",
        ))
        .param(
            param("enabled", ParamKind::Switch)
                .default("on")
                .help("on|off; given alone, turns the layer's existing grain on or off and keeps its settings"),
        )
        .example("pixelox fx grain doc.pxo --in-place --layer Title", "Printed grain on a live text layer")
        .example(
            "pixelox fx grain doc.pxo --in-place --layer Badge --amount 45 --size 2.5 --color 40",
            "Coarse color grain on a shape",
        )
        .example(
            "pixelox fx grain doc.pxo --in-place --layer Title --enabled off",
            "Turn the grain off, keeping its settings",
        )]
}

fn grain(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    set_style("grain", a, doc, ctx)
}

/// Add the grain of every enabled `grain` spec to a layer's styled rendering; `anchor` is
/// where the layer's own content sits on the canvas.
pub(crate) fn apply(specs: &[FxSpec], mut out: Rendered, anchor: (i32, i32), doc: &Document) -> Result<Rendered> {
    let short = doc.width.min(doc.height).max(1) as f64;
    for s in specs.iter().filter(|s| s.enabled && s.kind == "grain") {
        let g = Grain::from_params(&s.params, short)?;
        let origin = ((out.x - anchor.0) as i64, (out.y - anchor.1) as i64);
        g.apply(&mut out.image, origin);
    }
    Ok(out)
}
