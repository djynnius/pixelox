//! Tonal and color adjustments (adjust *), adjustment-layer rendering, and analysis (analyze *) for pixelox.
//!
//! * `adjust *` — twenty Photoshop-style adjustments. Each changes a raster layer's pixels
//!   (limited by `--mask`), or with `--as-layer` adds an adjustment layer whose params are
//!   self-contained: curve points, LUT tables, and detected `auto` values are embedded.
//! * [`install`] registers the compositor's adjuster hook, which renders those layers
//!   through the same code as the destructive ops.
//! * `analyze histogram|stats|colors|diff` — measurement for agents verifying their work.

mod adjust;
mod analyze;
mod auto;
mod curve;
mod hue;
mod lut;
mod tone;
mod util;

#[cfg(test)]
mod tests;

use pixelox_core::prelude::*;

pub use adjust::OPS as ADJUSTMENTS;

/// Every command this crate defines.
pub fn ops() -> Vec<OpSpec> {
    let mut v = adjust::specs();
    v.extend(analyze::specs());
    v
}

/// Install the adjustment-layer renderer into the compositor.
pub fn install() {
    compose::set_adjuster(adjust::adjuster);
}
