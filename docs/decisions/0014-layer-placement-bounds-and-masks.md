# ADR 0014: New layers go on top; filters grow layers; live filters on text and shapes

Status: accepted (2026-11-22)

## Context
Agents built 126 posters with pixelox and hit four structural problems:

* **Insertion order.** A new layer was inserted above the *active* layer, and `layer reorder`
  and `layer dup` change which layer is active. After a reorder or a duplicate, the next
  `text add` or `comp place` landed mid-stack; on one poster this hid all of the type. The
  design skill had to tell agents to restack every new layer after any reorder.
* **Blurs cut off at a layer's bounds.** A raster layer is only as large as its content
  (a rasterized shape or text, a placed cutout). `filter gaussian` and the other spatial
  filters kept the layer's size, so a soft glow or shadow ended in a hard rectangle. The
  workaround was a canvas-sized layer with a feathered selection fill.
* **Filters on text and shapes** failed with PX2303 until the layer was rasterized, which
  froze the wording.
* **Positions past the left or top edge** could not be written: a negative coordinate counts
  from the far edge, so `--at -80,0` is 80 px from the *right*.

## Decision
* **Placement.** A newly created layer goes on top of the stack. Every command that creates
  a layer takes `--above <layer>`, `--below <layer>`, and `--into <group>` (one at most;
  PX1005 for two, PX2330 when `--into` names a layer that is not a group). The helper is
  `pixelox_core::stack` (`common::placement()` for the flags, `stack::insert_new` /
  `insert_at` to insert); `pixelox_comp::util::insert_new` now inserts on top as well.
  Commands with the flags: `layer add`, `layer dup`, `layer stamp`, `comp place`,
  `text add`, every `shape` creator, `render lens-flare`, `render 3d --as-layer`, and every
  `adjust ... --as-layer`.
  * Derived layers keep their place next to their source, because the composite depends on
    it: `layer dup` puts the copy directly above the original, `adjust --as-layer --layer X`
    directly above X, `render lens-flare --layer X` directly above X (flags override these
    defaults). `path unite|subtract|intersect|exclude --into` (a name there) goes above the
    topmost operand, `path outline-stroke` above its source, `retouch frequency-split` above
    its source, `layer group` and `layer merge` where the topmost member was, and
    `transform canvas --fill` at the bottom; these take no placement flags.
  * **"Active"** now means only "the layer the last structural command created or moved"
    (what `@active` refers to). Creating, duplicating, reordering, grouping, merging, and
    stamping still set it, as before, so `--layer @active` keeps working; it no longer
    decides where anything is inserted.
* **Spatial filters grow the layer.** `region::apply_to_raster_grow(args, doc, ctx, reach,
  f)` pads the layer's raster with transparency by the filter's reach before filtering and
  trims the growth back to its alpha bounding box afterwards (the original extent is
  always kept, so an unchanged layer keeps its size and offset). Reach: 3σ + 1 for
  `filter gaussian` and `retouch blur`, radius × passes for `filter box`, half the distance
  split by the angle for `filter motion-blur`, the farthest corner's arc or zoom shift for
  `filter radial-blur`, the radius for `filter lens-blur`, one cell for `filter pixelate`.
  Growth stops at the canvas: a layer that covers the canvas does not grow at all (its
  output is bit-identical to before), and a side that already reaches the canvas edge keeps
  its old edge behavior (an opaque layer is clamped there, through replicated edge pixels
  that are dropped afterwards). The other filters were audited by blurring a disc on a
  canvas-sized layer and on a tight layer: none of them moves alpha past the content
  (`lens-correct` and `vignette` are centered on the layer by design). Layer styles already
  grow their bounds, and layer masks are canvas-sized, so `mask feather` and `fx
  drop-shadow|outer-glow` were never clipped.
* **Smart filters on text and shape layers.** ADR 0011's smart filters extend from placed
  layers to text and shape layers: `filter *` and `adjust *` record themselves on the layer
  and re-run whenever it renders, on the content spread onto the canvas, so a blurred title
  keeps its wording editable (`text edit`) and its blur is never cut off. `layer
  smart-filter`, `layer rasterize` (bakes them), `text to-path` (keeps them on the shape),
  and transforms that rasterize (bake them) all handle them. `svg from-doc` notes that it
  cannot export them. Fills on live layers fail with PX2303 and a hint naming the live
  alternative (`text style --color`, `shape set --fill`, `fx overlay`).
* **Masks.** `mask add` gains `--gradient linear|radial` (revealed at `--from`, hidden at
  `--to`; the fade the posters built 21 times from a gradient-filled layer), `--combine
  intersect|add|subtract` (merge with the existing mask), and `--from-layer L [--channel
  alpha|luma]`: a **live** mask stored as a link (`LayerMask.link`: source layer id,
  channel, invert) that the compositor re-reads from the source's content (without its
  styles) whenever it draws, so a mask from a text layer follows `text edit`, and a mask
  from a shape layer is a resolution-independent vector mask. The pixels in
  `LayerMask.mask` are kept as a snapshot, refreshed after every document step (from
  `Document::quantize_to_storage`), so every reader of mask pixels (`@mask:`, `mask list`,
  PSD export, `mask remove --apply`) sees the current shape and replays stay bit-exact.
  Editing the pixels (paint, expand, refine, ...) turns the link into an ordinary mask;
  `mask invert` flips the link's `invert` and keeps it live. A layer cannot use itself or
  an enclosing group as its source (PX2331), and links are rendered at most four deep, so
  a loop through groups cannot recurse. `@luma:<layer>` joins `@alpha:` and `@mask:` as a
  mask reference. Clipping several layers onto one base already worked and is documented
  in the new `pixelox docs masking` topic.
* **True negative coordinates.** A coordinate may be the sum or difference of two lengths,
  `A+B` or `A-B`: `0-80` (80 px past the left or top edge), `50%-120`, `100%+2in`. A sum
  always counts from the near edge; a single negative value (`-80`) keeps counting from the
  far edge, so every existing recipe means what it meant. The syntax needs no quoting in a
  shell, does not start with a minus (no `--at=` needed), and works everywhere geometry is
  parsed (`units::Length`, so points, rects, sizes, and point lists).

## Consequences
* Scripts that relied on "above the active layer" now get the layer on top. The poster
  regression suite and the guides were checked; one guide step gained `--below`.
* A blur on a small layer now produces a larger layer (the result's `grown` field reports
  the growth per side). An opaque layer smaller than the canvas now softens into
  transparency at its edges when blurred, as in Photoshop, instead of clamping.
* Text and shape layers with smart filters render canvas-sized, like placed layers with
  smart filters.
