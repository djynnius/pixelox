# Changelog

All notable changes are listed here. The format follows [Keep a Changelog](https://keepachangelog.com/),
and the project uses [Semantic Versioning](https://semver.org/) for its CLI contract: a breaking
change to any `--json` envelope, schema, sidecar, or exit code is a major version.

## [0.1.0] — 2026-11-05

First implementation of the PRD: the v0.1 "Document engine" milestone, most of v0.2–v0.5, and
the agent contract complete. Decisions taken along the way are in `docs/decisions/`.

### Added
- **Agent contract**: the versioned JSON envelope, `PX####` error codes with hints and exit
  codes 0–12, `--dry-run` plans, `--overwrite` / `--in-place` / `-y` consent gates, atomic
  writes, provenance sidecars with the full recipe, `replay`, `verify --fast|--full`,
  `schema` (draft 2020-12, snapshot-tested), `describe --agent` (under 4,000 tokens),
  `docs` (offline pages, topics, error catalog), seeded determinism invariant to thread
  count.
- **Documents**: `new` (sizes and canvas presets), `open`, `save`, `info`, `export` (8/16-bit
  PNG and TIFF, JPEG, lossless WebP, GIF, BMP, flattened and basic-layered PSD, regions,
  masks, size/format ladders); the `.pxo` container (deterministic ZIP); layered PSD import;
  SVG import; EXIF orientation; ICC profiles kept and re-embedded; `frames extract|assemble`.
- **Layers** (`layer *`): raster, text, shape, adjustment, group, and placed layers; all 28
  blend modes including pass-through groups; masks with density and feather; clipping
  masks; merge, flatten, rasterize, reorder, lock.
- **Selections and masks** (`select *`, `mask *`): rect, ellipse, polygon, path, color range
  and magic wand, luminosity; boolean modes; feather, expand, contract, smooth, invert;
  named masks, layer masks, vector masks.
- **Adjustments** (`adjust *`, destructive or `--as-layer`): 20 adjustments from levels and
  curves to 3D LUTs and auto tone. **Analysis** (`analyze *`): histogram, stats (eyedropper),
  dominant colors, diff (SSIM, ΔE00), EXIF.
- **Filters, retouching, fills** (`filter *`, `retouch *`, `fill *`): 19 filters, 8 retouch
  tools, solid, gradient (four geometries, OKLab interpolation), and pattern fills.
- **Transforms** (`transform *`): move, scale, rotate, flip, skew, perspective, mesh warp,
  crop, straighten (auto angle), canvas, resize, trim — live layers stay live.
- **Type** (`text *`, `fonts *`): rustybuzz shaping with bidi, ligatures, kerning, variable
  fonts, paragraph boxes, text on a path, text to outlines, an embedded fallback font.
- **Vectors** (`shape *`, `path *`): rectangles, ellipses, polygons, stars, lines, arrows,
  Bézier paths; boolean path ops, simplify, outline stroke.
- **Layer styles** (`fx *`): drop and inner shadow, outer and inner glow, bevel & emboss,
  satin, color/gradient/pattern overlays, stroke; copy and scale.
- **Composition** (`comp *`): place (embedded or linked), relink, update links, align,
  distribute, guides, grid, canvas presets, templates with typed slots and auto-shrink.
- **SVG and tracing** (`svg *`, `trace`): element-level SVG authoring with selectors, defs,
  symbols, query, optimize, rasterize; document-to-SVG export; bitmap tracing.
- **Generative rendering** (`render *`): a QuickJS sandbox with Canvas2D and a DOM for
  d3-style graphics (d3/three loaded from a local `--lib`), and native 3D-lite wireframe,
  hidden-line, and shaded rendering of primitives, extrusions, and OBJ meshes.
- **Automation**: pipeline files (vars, inputs with hash pins, `needs` DAG, `foreach` over
  globs and CSV, in-memory documents), `batch run`, background jobs, `history list|undo|
  checkpoint`, `config`.
- **MCP server** (`mcp serve`): eight tools, resources, allow-listed paths, read-only mode,
  two-key download consent, audit log.
- Claude Code skill and plugin manifest, `AGENTS.md`, guides, runnable examples.

### Not in this build
- AI-assisted commands (`detect *`, `mask refine`, `fill content-aware`, `models pull`) are
  present with full schemas and dry-run plans but exit 12: no ONNX runtime or weights.
- Color-profile conversion, lossy WebP, progressive JPEG, HEIC and RAW input, vendored
  d3/three, self-update, tile streaming under `--mem-budget`.
