//! Vector shapes (shape *, path *), SVG authoring (svg *), and tracing (trace) for pixelox.
//!
//! * `shape *` creates live shape layers (`LayerKind::Shape`, the spec format in
//!   AUTHORING.md); [`install`] registers the tiny-skia renderer that draws them.
//! * `path *` combines and edits shape geometry (i_overlay booleans, kurbo stroking).
//! * `svg *` edits standalone SVG files through an owned XML DOM with deterministic
//!   serialization ([`xml`]), and renders them with usvg + resvg.
//! * `svg from-doc` exports a document's layers as structured SVG; `trace` vectorizes a
//!   bitmap with the visioncortex engine (the core of vtracer).

pub mod fromdoc;
pub mod pathops;
pub mod render;
pub mod shape;
pub mod spec;
pub mod svg;
pub mod trace;
pub mod xml;

#[cfg(test)]
mod tests;

use pixelox_core::prelude::*;

/// Every command this crate defines.
pub fn ops() -> Vec<OpSpec> {
    let mut v = shape::ops();
    v.extend(pathops::ops());
    v.extend(svg::ops());
    v.push(fromdoc::spec());
    v.push(trace::spec());
    v
}

/// Install the shape layer renderer into the compositor.
pub fn install() {
    compose::set_shape_renderer(render::render);
}
