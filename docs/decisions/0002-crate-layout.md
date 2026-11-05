# ADR 0002: Crate layout differs from PRD §7.2

Status: accepted (2026-11-04)

## Context
PRD §7.2 lists `pixelox-raster` (all pixel kernels and their commands), `pixelox-js`, and
`pixelox-3d`. The first implementation was written by several engineers in parallel, one
crate each, and compile isolation mattered more than the PRD's grouping.

## Decision
* Shared kernels (blur, resample, morphology, convolution, SSIM, blend modes) and the
  compositor live in `pixelox-core`.
* The raster command groups are split by domain: `pixelox-adjust` (adjust, analyze),
  `pixelox-filter` (filter, retouch, fill), `pixelox-select` (select, mask, transform),
  `pixelox-fx` (layer styles), `pixelox-comp` (layer, comp).
* `pixelox-render` holds both the JS sandbox and native 3D-lite (`render *`).
* Contract-level commands (schema, describe, docs, replay, verify, history, pipeline,
  batch, jobs, config, doctor) live in `pixelox-cli`; `analyze exif` lives in `pixelox-io`.
* Renderers for text, shapes, adjustments, layer styles, and embedded-asset decoding plug
  into the core compositor through hooks installed at startup (`pixelox_cli::install`).

## Consequences
Every crate builds and tests on its own; core stays free of font, vector, and codec
dependencies. The PRD's crate names are not used for raster, js, and 3d.
