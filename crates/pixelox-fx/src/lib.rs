//! Layer styles (fx *): shadows, glows, bevel, satin, overlays, stroke, fire, smoke;
//! and `render lens-flare`.
//!
//! Each style command stores one [`FxSpec`](pixelox_core::doc::FxSpec) on a layer:
//! `kind` is the command name (`drop-shadow`, `stroke`, ...) and `params` is the command's
//! `args.to_json()` minus `layer` and `enabled` (keys use underscores, values are the
//! strings given or defaulted). A few keys are added: overlays record the resolved `kind`
//! (`color|gradient|pattern`), strokes the resolved `fill`, and pattern fills the `asset`
//! key of the tile embedded in the document, and fire and smoke the `variant` (noise
//! pattern) they were drawn with. [`install`] registers [`render`], which the compositor
//! calls with a layer's content to draw the stack live.

mod commands;
mod flame;
mod flare;
mod noise;
mod render;

use pixelox_core::prelude::*;

pub use render::render;

/// Every command this crate defines.
pub fn ops() -> Vec<OpSpec> {
    let mut v = commands::ops();
    v.extend(flare::ops());
    v
}

/// Install the layer-style renderer into the compositor.
pub fn install() {
    compose::set_fx_renderer(render::render);
}
