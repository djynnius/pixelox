# ADR 0017: Icons, QR codes, barcodes, and cast layer styles

Status: accepted (2026-11-22)

## Context
Agents building 126 posters with the design skill faked several common elements. Info bars
had no icons ("skip them rather than fake them"); there was no way to put a scannable code on a
flyer; long shadows and block lettering were brush-tip drags of an exported word
(`T-long-shadow`), and floor reflections were a flipped, faded copy plus a gradient layer
(`T-floor-reflection`). Two layer-style bugs got in the way: `fx inner-glow` glowed along the
canvas edge where the canvas cut a shape, and `fx overlay` painted over a shape's own stroke.

## Decision
* **Icons are Lucide (ISC), bundled as data.** Lucide is complete (about 2,100 icons, every
  event and info-bar glyph needed), drawn on one 24-unit grid with one stroke weight, and has
  no brand logos since 1.0. Tabler (MIT) is as large but less uniform at small sizes;
  Material Symbols are Apache-2.0 but fill-based and need a font or a large SVG set. The
  generator (`crates/pixelox-vector/scripts/gen-lucide.py`) turns a lucide-static release into
  one text line per icon (name, aliases, tags, SVG path data), records the release and its
  SHA-256 in the file header, copies the license, leaves out trademark symbols (Bluetooth,
  Creative Commons), and adds plain-word aliases (cart, location, email, web). The text file
  (~480 KB) is diffable and `include_str!`ed. `shape icon` draws an icon as a live shape layer
  (round caps and joins, stroke width size/12, even-odd fill for `--fill`); its box is snapped
  to whole pixels, so multiples of 24 px are pixel-sharp. `shape icons --search` ranks names,
  aliases, then tags.
* **QR codes come from `qrcode` (MIT/Apache) and are read back with `rqrr` (MIT/Apache + ISC).**
  The square style traces the dark modules into clean outlines (outer contours clockwise,
  holes counter-clockwise), rounded rounds every corner of those outlines by half a module,
  dots draws one circle per module. Finder patterns stay square in every style: rqrr (a quirc
  port, like several deployed scanners) locates codes by finder corners and missed 40 to 60%
  of codes with rounded eyes in a 288-case probe; with square eyes it read all of them.
  Modules of 3 px or more are snapped to whole pixels. A logo gap is allowed only when the
  codewords it touches (found by drawing index bits through the encoder's own placement) stay
  within two thirds of `max_allowed_errors` and the code still decodes with the gap blank,
  filled, and noisy; otherwise PX6034 names the largest gap that would pass. A background
  plate makes the result a group (`<name> background`, `<name> code`) so it moves as one.
* **Barcodes use a small table-driven encoder** (EAN-13, EAN-8, UPC-A, Code 128 with automatic
  code sets, Code 39 at 3:1). The tables were checked against python-barcode's as data, and
  tests decode rendered pixel rows back to modules. Check digits are validated (PX6032 with
  the right digit in the hint). The human-readable line is vector outlines from a printable
  ASCII subset of Tuffy (public domain, 10 KB) in the same path, so the code is one layer.
* **Long shadow, extrude, and reflection are layer styles** (`FxSpec` kinds `long-shadow`,
  `extrude`, `reflection`), in their own module (`pixelox-fx/src/cast.rs`). Shadows and
  extrusions are directional sweeps of the alpha: one line per quarter pixel, a monotonic
  deque for each line's sliding maximum (O(pixels) whatever the length), each pixel the mean
  of the two lines a quarter pixel either side of it (anti-aliased sides at any angle) and
  never less than its own alpha (no fringe around the shape). Auto-colored extrusions carry
  the nearest face pixel's color after overlays, so a gradient face gets gradient sides; the
  renderer therefore draws the clipped effects before the ones beneath the content. Long and
  drop shadows are cast by the extruded block. Long shadows stop at the canvas edge (their
  margins are clamped there); their fade is one gradient along the direction, so letter gaps
  do not streak it. A reflection mirrors the block with its strokes (not shadows or glows)
  about the bottom of its content, with a blur that grows with distance (`--distance`).
* **Content cut by the canvas continues past it.** Every style except fire and smoke sees the
  content's alpha extended beyond each side of the content image that lies on or past the
  canvas edge (edge pixels repeated). Inner glows, inner shadows, satin, bevels, and inside or
  center strokes no longer draw a false edge along the canvas; offset shadows reach the edge.
* **Overlays recolor the fill.** On a shape or text layer with both a fill and a stroke, the
  compositor renders the fill alone with the layer's overlays, then draws the stroke on top,
  then applies the other effects; the gradient spans the fill. `fx overlay --cover-stroke`
  keeps Photoshop's behavior (the stroke is part of the content). A stroke-only shape, or a
  stroke in the fill's own color (a common way to fatten letters, as in poster p062), is still
  recolored whole: the regression run showed that a fattening stroke left on top reads as an
  unwanted outline. This lives in `compose::content` because the fx hook only sees
  pixels.

## Consequences
The binary grows by about 0.6 MB (icons, font subset, decoder). A long shadow on a very large
canvas costs a sweep over the canvas-sized buffer. Documents whose shapes or text combine a
stroke with an overlay, or whose styled content touches the canvas edge, render differently
than in 0.4.1 (recorded under Changed/Fixed). Rounded QR eyes are not offered; adding them
needs a decoder that does not depend on finder corners to verify them.
