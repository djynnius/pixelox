# ADR 0011: Smart filters, Blend If, and layer-mask references

Status: accepted (2026-11-05)

## Context
A review of a long-form Photoshop compositing tutorial against the command surface found
four gaps an agent could not work around cleanly: filters on placed layers (Photoshop's
smart filters) failed with PX2303; there was no Blend If; there was no Stamp Visible; and a
layer's mask could not be read back as a selection, so masks could not be edited in steps.

## Decision
* **Smart filters.** A `filter *` or `adjust *` command that targets a placed layer is
  recorded on the layer as `{op, params, seed, op_index}` instead of failing, and re-run
  whenever the layer renders. The CLI installs the hook (`compose::set_smart_filter_hook`)
  because only it holds the full registry; it runs the command on a temporary one-layer
  document, so every pixel command works without per-command code. The placed content is
  spread onto the canvas before filtering, so a blur can grow past the placed rectangle
  (clipped to the canvas, as in Photoshop). Commands given a file argument (a LUT, a curve
  file) or a second mask are not eligible (they would re-read files at render time), and
  `--mask` is refused with a rasterize hint: Photoshop's filter masks are left for later.
  `layer rasterize` bakes the filters. The seed and recipe index are stored, so noise and
  other seeded filters render identically on every replay.
* **Blend If** is a layer property (`blend_if: {channel, this, underlying}`), ranges
  `[b_lo, b_hi, w_lo, w_hi]` on 0..255, applied as an extra coverage factor in the
  compositor for every layer kind, including adjustment layers. A pass-through group with
  Blend If composites isolated. PSD export does not write it (warning PXW1205).
* **`layer stamp`** composites the visible document into a new raster layer, cropped to
  its opaque bounds.
* **`@mask:<layer>`** joins the mask references next to `@alpha:<layer>`, so
  `select load doc.pxo @mask:Sky` loads a layer mask as the selection.

## Consequences
Old `.pxo` files load unchanged (the new layer fields default to empty). A placed layer
with smart filters renders canvas-sized, which costs memory on very large canvases; it is
the price of filters that can spread. `layer smart-filter` disables, enables, or removes
one filter; reordering them is not supported yet.
