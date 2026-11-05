# Writing pixelox ops (contributors and building agents)

Every command is one `OpSpec` (`crates/pixelox-core/src/op.rs`). The CLI flags, JSON
Schemas, pipeline steps, MCP tools, `describe --agent`, and the docs site are generated
from it, so names and help text are written exactly once (ADR 0001). A crate contributes
commands through `pub fn ops() -> Vec<OpSpec>`; the CLI calls every crate's `ops()` and
`install()` (`crates/pixelox-cli/src/lib.rs`). Never edit the CLI to add a command.

## The pixel model

* `Image` (`pixelox_core::image`): RGBA `f32`, 0..1, **sRGB-encoded, straight (not
  premultiplied) alpha**, row-major, 4 floats per pixel. `get/set/from_fn/map_pixels`,
  `premultiply/unpremultiply`, `crop`, `alpha_bbox`, `sample_bilinear`.
* `Mask`: one `f32` per pixel, 0..1 coverage. Selections, named masks, and layer masks are
  all **canvas-sized, in canvas coordinates**.
* A raster layer's pixels sit at `(layer.x, layer.y)` on the canvas and may be any size.
* `Rendered { image, x, y }`: an image placed on the canvas; `to_canvas(w, h)`.
* Shared kernels (`pixelox_core::kernels`): `gaussian_blur(img, sigma, Edge)`,
  `gaussian_blur_xy`, `box_blur`, `blur_mask`, `auto_edge(img)` (Clamp for opaque, Zero for
  transparent layers), `resize(img, w, h, Filter)`, `resize_mask`, `dilate/erode(mask, r)`,
  `median_mask`, `convolve(img, kernel, kw, kh, keep_alpha)`, `ssim(a, b)`.
* Color (`pixelox_core::color`): `Rgba`, `parse_color`, `parse_stops`, `sample_stops`,
  `rgb_to_hsl/hsl_to_rgb`, `rgb_to_hsv/hsv_to_rgb`, `srgb_to_linear/linear_to_srgb`,
  `luma`, `lum`, `rgb_to_lab`, `delta_e00`, `rgb_to_oklab/oklab_to_rgb`, `mix`, `Interp`.
* Blend modes (`pixelox_core::blend`): `BlendMode`, `blend_color`, `composite_pixel`.
* Geometry (`pixelox_core::units`): `Length`, `PointSpec`, `SizeSpec`, `RectSpec`, `RectI`,
  `RectF`, `Anchor`, `parse_points`, `parse_range`, `parse_angle`.

## The document model

`Document` (`crates/pixelox-core/src/doc.rs`): `width`, `height`, `dpi`, `color_space`,
`depth` (8|16), `layers` (**bottom → top**), `selection: Option<Mask>`, `masks` (named mask
library), `guides`, `grid`, `slots` (template slots), `recipe`, `assets`, `meta`.

* Layers: `Layer { id, name, content: LayerKind, visible, locked, opacity (0..100), blend,
  x, y, mask: Option<LayerMask>, clip, fx: Vec<FxSpec>, props }`.
* `LayerKind`: `Raster { image }`, `Text { spec }`, `Shape { spec }`,
  `Adjustment { op, params }`, `Group { children }`, `Placed { asset, path, linked, rect,
  angle, source }`.
* Addressing: `doc.resolve(ref)` (name, id, `#N` from the top, `@top`, `@bottom`,
  `@active`), `doc.resolve_or_only(Option<&str>)` (default to the only layer, PX2302
  otherwise), `doc.layer(&path)`, `doc.layer_mut(&path)`, `doc.walk()` (panel order, top
  first), `doc.new_layer(name, kind)` (fresh id + unique name), `doc.insert_above(Some(&path)
  | None, layer)`, `doc.remove(&path)`, `doc.siblings_mut(&path)`, `layer.raster()`,
  `layer.raster_mut()` (PX2303 when not raster), `layer.check_unlocked()` (PX2304).
* Masks: `doc.named_mask(name)` (named or `@sel`), `doc.put_mask(name, mask, replace)`,
  `ctx.resolve_mask(doc, ref)` (also `@alpha:<layer>`, `@mask:<layer>`, and image paths),
  `ctx.mask_arg(args, doc)` (reads `--mask` and `--invert-mask`).
* Rendering: `compose::composite(doc)`, `compose::composite_subset(doc, &ids)`,
  `compose::render_layer(doc, layer)` (one layer's content with fx, no mask/opacity),
  `compose::layer_bounds(doc, layer)`, `compose::apply_adjustment(op, params, img)`.

### Spec formats shared between crates

These JSON objects live in layers and are read by more than one crate. Keys not listed
here are owned by the crate that writes the layer.

**Text layer spec** (`LayerKind::Text { spec }`, owned by `pixelox-text`). Other crates
read/write only: `content` (string), `color` (CSS color string), `font` (family string),
`size` (CSS length string such as `48pt` or `64px`). `comp template apply` replaces
`content` and `color`, and may lower `size` to auto-shrink into the slot's box.

**Shape layer spec** (`LayerKind::Shape { spec }`, owned by `pixelox-vector`):

```json
{"d": "M 10 10 L 90 10 L 90 90 Z", "fill": "#ffd400", "stroke": "#111111", "stroke_width": 10,
 "dash": [4, 2], "dash_offset": 0, "cap": "butt", "join": "miter", "miter_limit": 4,
 "fill_rule": "nonzero", "geometry": {"type": "rect", "rect": "10,10,80x80"}}
```

`d` is SVG path data in canvas pixels; `fill`/`stroke` are CSS colors or `null`/absent for
none. `text to-path` writes this format. `comp template apply` sets `fill` for color slots.

**Moving and transforming layers.** `geom::translate_layer(doc, &path, dx, dy)` moves any
layer consistently (raster offset, text anchor, shape path, placed rect, group children,
and the layer mask with it); use it for every move (`transform move`, `comp align`,
`comp distribute`, document crops). For scale/rotate/skew, these keys are the shared
contract, so transforms keep live layers live:

* text spec: `x`, `y` (numbers, the anchor point in canvas px), `anchor` (`tl`..`br`),
  `size` (length string), `rotation` (degrees about the anchor, default 0), `path`
  (SVG path data for text on a path);
* shape spec: `d` (transform it with `geom::transform_path_data(d, [a,b,c,d,e,f])`) and
  `stroke_width` (scale it by the uniform scale factor);
* placed layers: `rect` `[x, y, w, h]` and `angle` (degrees about the rect center);
* groups: recurse into children; adjustment layers have no geometry.

Skew, perspective, and warp of a text, shape, or placed layer rasterize it first
(`compose::render_layer`), with a warning. `geom::path_bbox(d)` and `geom::shift_mask` are
there too. Canvas presets (`a4`, `ig-story`, ...) live in `pixelox_core::presets`.
`compose::decode_asset(doc, key)` decodes bytes embedded with `doc.add_asset(bytes, ext)`
(used for pattern overlays), through the decoder `pixelox-io` installs.

**Adjustment layers**: `Adjustment { op, params }` where `op` is the adjust command name
without the group (`levels`, `curves`, `hsl`, ...) and `params` is the command's
`args.to_json()` minus `layer`, `mask`, `invert-mask`, `as-layer`, `name`. The adjuster
hook (`compose::set_adjuster`, installed by `pixelox-adjust`) applies it to an image.

**Layer styles**: `FxSpec { kind, enabled, params }`, `kind` = the fx command name
(`drop-shadow`, `stroke`, ...), `params` = `args.to_json()` minus `layer`. The fx hook
(`compose::set_fx_renderer`, installed by `pixelox-fx`) receives a layer's rendered
content and returns it with the effects composited (bounds may grow).

**Renderer hooks**: text and shape layers are drawn by hooks installed with
`compose::set_text_renderer` / `compose::set_shape_renderer`
(`fn(&Layer, &Document) -> Result<Option<Rendered>>`). A crate installs its hooks in
`pub fn install()`; tests that need a hook call `install()` themselves.

## Defining an op

```rust
use pixelox_core::prelude::*;

pub fn ops() -> Vec<OpSpec> {
    vec![
        OpSpec::doc("filter gaussian", Category::Filter, "Gaussian blur", gaussian)
            .about("Separable Gaussian blur; switches to three box passes above sigma 10.")
            .param(param("sigma", ParamKind::Float).default("2").range("0..1000").help("Blur radius (standard deviation) in px"))
            .params(common::region())
            .example("pixelox filter gaussian walk.jpg -o soft.jpg --sigma 8", "Blur a photo")
            .example("pixelox filter gaussian walk.pxo --in-place --sigma 8 --mask dog --invert-mask", "Blur everything except a mask")
            .example("pixelox filter gaussian walk.pxo --sigma 4 --dry-run --json", "Plan without writing"),
    ]
}

fn gaussian(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let sigma = a.f64("sigma")?;
    apply_to_raster(a, doc, ctx, |img, _| Ok(kernels::gaussian_blur(img, sigma, kernels::auto_edge(img))))
}
```

Three runners:

* `OpSpec::doc(name, cat, summary, fn(&Args, &mut Document, &mut OpCtx) -> Result<Value>)`
  — the executor opens INPUT (image, PSD, SVG, or `.pxo`) as a document, runs the op, then
  writes it (`-o x.pxo` keeps layers, `-o x.png` flattens, `--in-place` rewrites a `.pxo`),
  appends the recipe step, and writes the sidecar. Mark inspection commands
  `.read_only()` (no output needed; `info`, `layer list`, `analyze stats`). Commands that
  write their own deliverables from a document use `.export_mode()`.
* `OpSpec::source(...)` → `Document` — creates a document (`new`).
* `OpSpec::custom(name, cat, summary, fn(&Args, &mut OpCtx) -> Result<Value>)` — full
  control: read inputs with `ctx.input(i)`, `ctx.load_document(path)`,
  `ctx.read_aux(path, role, param)`, `ctx.decode_aux(path, role, param)`; write with
  `ctx.write_image(path, &img, doc, &opts, role, recipe)`, `ctx.write_doc(path, &doc,
  role)`, `ctx.write_file(path, bytes, role, recipe)`. Output path: `ctx.globals.output`
  (PX1004 when required and missing). These helpers enforce `--overwrite` (PX1401), the MCP
  sandbox (PX1310), atomic temp+rename writes, and record outputs for sidecars. Honor
  `ctx.dry_run()`: the write helpers already skip writing in dry runs.

Rules:

1. **Names and flags come from the PRD** (§8). The command path is space-separated
   (`"comp template apply"`); its id is dotted (`comp.template.apply`). Flags are kebab-case
   (`--in-black`); JSON/pipeline keys use underscores automatically.
2. **Never declare a param named like a global flag**: `output`, `format`, `json`,
   `dry-run`, `overwrite`, `in-place`, `seed`, `yes`, `quiet`, `verbose`, `threads`,
   `mem-budget`, `progress`, `no-sidecar`, `background`, `config`, `redact-paths`,
   `limit-pixels`. (`-o` is the output.)
3. **Summaries ≤ 60 characters**, no trailing period: they make up `describe --agent`,
   which has a hard 4,000-token budget for ~180 commands. Put detail in `.about(...)`.
4. **Every param has help text**; give defaults that do the right thing with zero flags;
   use `.range("0..100")` for numeric ranges (PX1013 enforced), `.choices(&[...])` with
   `ParamKind::Choice`, `.auto()` when the word `auto` is accepted, `.required()` only when
   no default makes sense. Positional words after the input go to params named with
   `.positional("name")` (e.g. `pixelox select load doc.pxo wires`).
5. **At least three examples per command**, each a complete, valid command line starting
   with `pixelox ` using the command's real flags (they are checked against the CLI in a
   contract test). Use plausible file names (`photo.jpg`, `doc.pxo`, `out.png`).
6. **Region ops** use `common::region()` (`--layer`, `--mask`, `--invert-mask`) and
   `apply_to_raster(args, doc, ctx, |img, ctx| ...)`, which resolves the layer, checks the
   lock, and limits the result to the mask. Ops on non-raster layers fail with PX2303.
7. **Errors** use catalog codes (`crates/pixelox-core/src/error.rs`):
   `Error::new("PX1010", msg)` (bad value), `PX1013` (out of range), `PX1005` (conflicting
   flags), `PX2303` (wrong layer kind), `PX3310` (curve points), `PX3311` (bad LUT),
   `PX3320` (empty region), `PX3001` / `Error::runtime` (operation failed). Add
   `.with_hint(...)` with a runnable fix and `.with_context(key, value)` with the offending
   values. Do not add codes to the catalog from an op crate; ask in your report instead.
8. **Warnings**: `ctx.warn("PXW....", msg)` with codes from `error::WARNINGS`.
9. **Determinism** (PRD §6.6): no wall clock, no `HashMap` iteration order in output, all
   randomness from `ctx.rng()` / `ctx.rng_lane(n)` (ChaCha12 keyed by seed, op index, op
   id). Thread count must never change output bytes: parallelize independent per-pixel or
   per-row work (`par_chunks_mut`); for float reductions collect per-row partials in order
   and sum sequentially (rayon's parallel `sum` of floats depends on the split).
10. **Values resolved at run time** (`--angle auto` → 2.3°) are recorded with
    `ctx.resolve_param("angle", 2.3)` so `replay` reproduces them without re-detecting.
11. **Results**: return a JSON object with what an agent needs to decide the next step
    (counts, bounds, coverage, chosen values). Doc ops get `op`, `params`, and `document`
    added by the executor.
12. **Complexity**: O(pixels) per op; separable or sliding-window kernels for large radii;
    nothing accidentally quadratic in image dimension.

## Tests

Use `pixelox_core::testing`: `gradient`, `checker`, `solid`, `doc_from`, `doc_layers`,
`half_mask`, `find(&ops(), "filter gaussian")`, `run(&spec, &[("sigma", "3")], &mut doc)`,
`run_with_warnings`, `run_source`, `max_diff`, `mean`, `raster(&doc, "Background")`,
`TempDir`. Fixtures are generated in code; never commit binary images. Each command needs
at least one behavior test of what it does; add identity tests where one exists
(`--amount 0`, `--radius 0`), a mask test for region ops, and an edge case (transparent
layer, 1×1 image, locked layer). Keep each test under a second in debug builds.

## Building

The workspace shares one `target/` directory (disk is tight); do not set a private
`CARGO_TARGET_DIR` and never run `cargo clean`. Work only inside your crate:

```bash
cargo test -p pixelox-filter
cargo clippy -p pixelox-filter --all-targets -- -D warnings
cargo fmt -p pixelox-filter
```

New dependencies go in your crate's `Cargo.toml`; prefer the workspace's
(`tiny-skia`, `kurbo`, `usvg`, `resvg`, `rustybuzz`, `ttf-parser`, `fontdb`,
`unicode-bidi`, `unicode-linebreak`, `rayon`, `serde_json`); any new crate must be MIT,
Apache-2.0, BSD, Zlib, ISC, or similarly permissive (no GPL/LGPL/AGPL).
