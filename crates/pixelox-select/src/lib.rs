//! Selections (`select *`), masks (`mask *`), and geometric transforms (`transform *`) for
//! pixelox.
//!
//! * `select`: anti-aliased shape selections (tiny-skia), color range / magic wand,
//!   luminosity ranges, a classical sky selection, boolean modes, and the named-mask
//!   library round trip.
//! * `mask`: library masks, layer masks (raster and vector), and refinement (density,
//!   feather, expand, contract, smooth, invert) on any stored mask, `@sel`, or a layer mask.
//! * `transform`: layer and document geometry that keeps text, shape, placed, and group
//!   layers live where their specs can express the map, and moves every mask and guide
//!   with the pixels.

mod engine;
mod mask;
mod raster;
mod sample;
mod select;
mod sky;
mod straighten;
mod transform;
mod util;
mod warp;

use pixelox_core::prelude::*;

/// Every command this crate defines.
pub fn ops() -> Vec<OpSpec> {
    let mut v = select::ops();
    v.extend(sky::ops());
    v.extend(mask::ops());
    v.extend(transform::ops());
    v
}

/// Install renderer hooks (none: this crate renders nothing on its own).
pub fn install() {}

#[cfg(test)]
mod tests;
