# ADR 0003: Raster layers store pixels; the recipe is the op-graph

Status: accepted (2026-11-04)

## Context
PRD §7.3 describes a lazily evaluated op-graph with per-tile caches and tile streaming
under `--mem-budget`.

## Decision
For v0.1:
* Raster layers store their pixels (f32 RGBA in memory; 8- or 16-bit PNG in `.pxo`).
  Text, shape, adjustment, placed, and group layers stay live and render on demand.
* Every mutating command appends a recipe step; the recipe is the history. Sidecars embed
  it, `replay` re-executes it from the recorded sources, and `history undo` rebuilds a
  document from a shortened recipe. Any historical state is recoverable as long as the
  sources are.
* Pixels are straight (not premultiplied) alpha, sRGB-encoded; kernels premultiply
  internally where it matters (blur, resample) so transparent pixels never bleed color.
* Compositing is whole-canvas, row-parallel, with fixed-order reductions; there is no tile
  cache, and `--mem-budget` is recorded but not enforced.

## Consequences
Bit-exact replay and undo work today without a tile scheduler. Very large documents
(100 MP, 10 layers) need several GB of RAM; tile streaming is future work and does not
change the file format or the contract.

## Addendum: storage-depth quantization after every step

A recipe must produce the same bytes whether its intermediate documents were saved
(separate CLI calls, each writing a `.pxo` at 8 or 16 bits) or kept in memory (pipelines,
`replay`, `history undo`). After every document step the executor rounds raster pixels and
masks to the document's storage depth (`Document::quantize_to_storage`), exactly as a save
and reload would. Replays are therefore bit-identical to the original run on the same
build; an integration test checks it across adjustment layers and seeded noise.
