# Changelog

All notable changes are listed here. The format follows [Keep a Changelog](https://keepachangelog.com/),
and the project uses [Semantic Versioning](https://semver.org/) for its CLI contract: a breaking
change to any `--json` envelope, schema, sidecar, or exit code is a major version.

## [Unreleased]

### Added
- Companion skill `skills/pixelox-design`: an art-direction playbook for posters, flyers, ads,
  covers, and social posts from a brief, including vague briefs with no reference image.
  It ships text descriptions of 126 studied poster designs (grid in percent, layer stack,
  palette, type table, a tested build recipe, variations, pitfalls), an index that says which
  to combine for which brief, 27 layout archetypes, 74 tested effect recipes with stable IDs
  (`T-smoke`, `T-grunge`, `T-duotone`, `T-fit-width`, `T-text-behind-subject`,
  `T-water-splash`, ...), font classes with free look-alikes and per-OS stand-ins, named
  palettes, and 15 `render js` generators (clouds, blobs, grunge, rays, particles, brush,
  splatter, splash, torn, crack, waves, ribbons, pattern, tiles, stand-ins). No images are
  included. Every documented command is parsed by the contract test.
- Placement flags on every command that creates a layer (ADR 0014): `--above <layer>`,
  `--below <layer>`, and `--into <group>` on `layer add`, `layer dup`, `layer stamp`,
  `comp place`, `text add`, every `shape` creator, `render lens-flare`, `render 3d
  --as-layer`, and `adjust ... --as-layer` (PX2330 when `--into` is not a group).
- Filters and adjustments on live text and shape layers: `filter *` and `adjust *` on a
  text or shape layer are recorded as smart filters, as on placed layers, so
  `filter gaussian --layer Title` keeps the title editable (`text edit`) and its blur whole.
  `layer smart-filter`, `layer rasterize`, and `text to-path` handle them.
- True negative coordinates: a coordinate may be a sum or difference of two lengths,
  `0-80` (80 px past the left or top edge), `50%-120`, `100%+2in`. A single negative value
  still counts from the far edge. Works in every point, rectangle, size, and point list;
  `pixelox docs addressing` documents it.
- Masking: `mask add --gradient linear|radial --from X,Y --to X,Y [--ease smooth]` fades a
  layer out in one command; `mask add --from-layer L [--channel luma]` makes a live mask that
  follows another layer's outline or brightness (text stays editable; `mask invert` keeps it
  live); `mask add --combine intersect|add|subtract` merges a new mask with the existing
  one; the `@luma:<layer>` mask reference (a layer's brightness) works wherever a mask is
  taken (PX2331 when a layer would mask itself). New topic: `pixelox docs masking` explains
  every masking feature with examples.
- Font classes (ADR 0015): `--font @condensed-heavy`, `@didone`, `@script-formal`, `@brush`,
  `@slab`, `@stencil`, `@blackletter`, `@geometric-sans`, `@mono`, and 19 more resolve to the
  best installed face (macOS, Windows, and Linux families first, then Google Fonts), with the
  class's own weight. `pixelox fonts classes` shows what each resolves to; `fonts list` has a
  class column and `--class`.
- `pixelox fonts get <family|@class>`: downloads open-licensed Google Fonts families with
  consent (`-y`; PX1540, exit 6, without it; MCP `consent: ["font-download"]` with
  `mcp serve --allow-font-downloads`; `PIXELOX_OFFLINE=1` forbids it). 64 curated families
  covering every class are pinned to one google/fonts commit with the SHA-256 of every file;
  other families can be fetched by name. Files land in the data directory with their license
  and are registered at once. `text add/edit/style -y` downloads a missing family first.
- `--stretch` on `text add/edit/style` (condensed, extended, 50%..200%; sets variable `wdth`).
- Live type settings on `text add/edit/style`: `--case upper|lower|title`,
  `--scale-x`/`--scale-y` (character scaling), `--faux-bold` (extra stem weight in 1/1000 em),
  `--fit-width`/`--fit-box` (size the visible letters to a width or box; refit on every edit),
  and `--anchor-to ink` (place the visible letters, not the text box).
- Text warp presets on the glyph outlines, live and vector: `pixelox text warp --preset arc`
  (and `text style --warp`), with Photoshop's 15 styles (arc, arc-lower, arc-upper, arch,
  bulge, shell-lower, shell-upper, flag, wave, fish, rise, fisheye, inflate, squeeze, twist),
  `--bend`, `--h-distort`, `--v-distort`, and `--orient`.
- `text on-path --fit shrink` lowers the size until the text fits on the path.
- `pixelox docs fonts` and `pixelox docs text-warp` topic pages.

### Changed
- New layers go on top of the stack (ADR 0014). They used to land above the *active* layer,
  and `layer reorder` and `layer dup` change which layer is active, so a layer created after
  a reorder could land mid-stack and hide under others. `layer dup` still puts the copy
  directly above the original, and `adjust --as-layer --layer X` and `render lens-flare
  --layer X` directly above X. The active layer (`@active`) no longer decides placement.
- Spatial filters grow a layer that is smaller than the canvas before filtering and trim the
  growth to what is visible afterwards: `filter gaussian`, `box`, `motion-blur`,
  `radial-blur`, `lens-blur`, `pixelate`, and `retouch blur`. A blurred rasterized shape or
  text is no longer cut off in a rectangle at the layer's edge; the result's `grown` field
  reports the growth. Layers that cover the canvas render exactly as before. An opaque
  layer smaller than the canvas now softens into transparency at its edges when blurred.
- A font family that is not installed no longer fails (PX4102, exit 2): the closest installed
  face of its class is used, with warning PXW4010 naming the substitute and the
  `pixelox fonts get` command. `--strict-font` or `PIXELOX_STRICT_FONTS=1` keeps the error
  (PX4102, or PX1540 when the family can be downloaded). Documents whose font is missing are
  drawn with that substitute instead of Tuffy.
- Font matching follows the CSS order (width, then style, then weight): a weight request no
  longer swaps in a face of another width (`Helvetica Neue` 900 and `Futura` 800 drew condensed
  faces; they now draw the normal-width Bold and warn with PXW4013). Saved documents keep the
  face recorded with each layer.
- `transform scale` with unequal factors keeps an unrotated (or 90°-turned) text layer live by
  setting its size and `scale_x`, instead of rasterizing it (PXW2003).
- `text add` records no `weight` unless `--weight` is given (default still 400; a class uses
  its own weight).
- `replay` and `verify --full` name the recorded fonts that are not installed and how to get
  them (PXW4011; `replay --strict` fails with PX4112, exit 9).

### Fixed
- Fills (`fill solid|gradient|pattern`) on a text, shape, or placed layer now fail with a
  PX2303 hint naming the live alternative (`text style --color`, `shape set --fill`,
  `fx overlay`) instead of only suggesting rasterizing.
- Transforms that rasterize a layer with smart filters bake the filters and drop them from
  the layer (they were left behind on the raster); `svg from-doc` no longer exports a
  placed layer's source without its smart filters.
- The contract test tokenizes quoted strings: commands with `|`, `#`, or `...` inside a quoted
  value (`--content "A | B"`) are now checked instead of skipped.
- `filter motion-blur --angle` was verified to follow the same counter-clockwise convention
  as `fx overlay`, `fx bevel`, and `filter emboss` (45 rises to the right); its help and
  `pixelox docs addressing` now say so, and that `render js` scripts measure angles the
  other way. No behavior changed.
- `text on-path` let letters run past the end of an open path, or overlap the start of a closed
  one, silently; it now warns (PXW4012) with the count and the size that would fit.

## [0.4.1] — 2026-11-09

### Fixed
- The `pixelox-retouch` skill's frontmatter now parses as strict YAML: its description, which
  contains `: `, is quoted, so agents that read skill frontmatter with a YAML parser load the
  skill instead of skipping it.

## [0.4.0] — 2026-11-06

### Added
- `mask refine` works (ADR 0013): Photoshop's Extract rebuilt from published methods. Any
  rough mask (a `detect subject` mask, a selection, a path) becomes a hair-level matte by
  closed-form matting in a band around the edge (`--band`, or a painted `--unknown`
  highlighter), and `--decontaminate` removes background color from the edge pixels;
  `--force-foreground` for single-color hair. On a 12 MP portrait it removed a blue fringe
  (62% → 0.2% of edge pixels) and recovered lost strands in about a second. No model needed.
- `render lens-flare` (zoom, 35 mm, 105 mm, and movie lenses) as a movable screen layer.
- `fx fire` and `fx smoke`: flames and smoke rising from a layer's outline, as live layer
  styles (`--variant` picks the pattern).
- Art filters: `filter pencil` (and colored pencil with `--color`), `filter charcoal`,
  `filter ink`, `filter oil-paint`, `filter watercolor`, `filter cartoon`, `filter cutout`,
  `filter halftone` (mono or CMYK).
- Cookbook: extract a subject; lens flares, fire, and art filters.
- `pixelox update` (alias `self-update`, which was a stub): reads the newest release from
  GitHub, and for a copy the curl or PowerShell installer placed runs that release's
  installer into the same folder, then checks the new binary's version; `--check` only
  reports, `--version` pins or goes back. Ported from ferraus. Errors PX1530-PX1532.
- README: one prompt that sets up an agent (CLI, skills, MCP, contract), and the skills tables.

## [0.3.0] — 2026-11-06

### Added
- `select skin`: a skin mask sampled at `--at` points (so it works for every skin tone),
  tolerant of light and shadow, excluding eyes, brows, lips, teeth, hair, and backdrops, with
  advice when a sample or the mask looks off.
- `retouch blemishes`: finds pimples and small spots at pixel accuracy inside a mask
  (scale-space blob detection against the skin's own texture), lists them with contrast and
  redness, and heals them in one pass (`--heal`); `--min-redness` keeps moles and freckles.
  Warning PXW2005 when it searched without a mask.
- `retouch frequency-split`: Photoshop's frequency separation done exactly (Low + High in
  linear light, reconstruction error 0 at 8 and 16 bit).
- Companion skill `skills/pixelox-retouch`: a portrait retouching playbook (keep the person,
  keep the texture, skin only, prove it) with two evals.

## [0.2.0] — 2026-11-05

Closes the gaps found by replaying a long-form Photoshop compositing tutorial against the
command surface (see `docs/guides/cookbook/photo-composite.md`).

### Added
- `paint stroke`: brush strokes along points, a points file, or SVG path data, onto
  pixels, a layer mask, the selection, or a named mask. Size, hardness, flow, opacity
  (Photoshop's per-stroke cap), spacing, angle and roundness, image tips, seeded size/angle
  jitter, scatter and count, tapers, pressure, blend modes, erase. Warning PXW2004 when a
  stroke paints nothing.
- `select sky`: a classical (no-AI) Select › Sky with clouds, `--holes`, edge-aware
  refinement, a horizon estimate, and a confidence score with advice.
- `adjust clarity`, `adjust texture`, `adjust dehaze` (dark channel prior; negative adds
  haze), destructive or as adjustment layers.
- Smart filters: `filter` and `adjust` commands run on a placed layer are recorded on it
  and re-run at render time; `layer smart-filter` disables, enables, or removes one;
  `layer rasterize` bakes them (ADR 0011).
- `layer blend-if` (Blend If on this layer and the underlying composite, with split
  ramps), `layer stamp` (Stamp Visible), and the `@mask:<layer>` mask reference.
- Cookbook: blend a photo composite.
- **Local AI models** (ADR 0012): `detect subject` (BiRefNet-lite, MIT, 224 MB; `--fast` uses
  U²-Netp) and `detect salient` (U²-Netp, Apache-2.0, 5 MB) run on the CPU through ONNX
  Runtime, reproducibly (fixed threads, deterministic kernels). Results go to the selection,
  `--save-mask`, `--layer-mask` (one-step cutouts with `-o cut.png`), and side files
  (`--cutout`, `--mask-out`) whose sidecars replay bit for bit. Edges are refined by guided
  joint upsampling (`--refine`).
- `models pull|rm|verify` work: downloads need `-y` (PX1502, exit 6, otherwise), resume
  after interruptions, are SHA-256 verified before use, and respect `PIXELOX_OFFLINE=1`; a
  model file copied into the models directory is adopted when its hash matches.
- **Installers**: `curl -fsSL …/install.sh | sh` (macOS, Linux) and
  `irm …/install.ps1 | iex` (Windows) run the installers that the new release workflow
  (cargo-dist 0.32, 8 targets, attested) attaches to each GitHub release.

### Changed
- `detect subject` and `detect salient` are document commands now (they need `-o` or
  `--in-place` unless they only write `--cutout`/`--mask-out`); `--annotate` was dropped
  from them. Builds for Intel macOS and musl have no ML runtime (exit 12, PX1510).

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
