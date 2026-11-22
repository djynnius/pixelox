# Technique library

Named, reusable effect recipes. Poster descriptions cite them by ID (`T-smoke`, `T-duotone`).
Every block below was run end to end with pixelox 0.4 and its output was looked at.
Find a technique by searching this file for its heading (`## T-smoke`); do not read the
whole file.

## How to use a recipe

* Each block is self-contained: it starts from a `new` canvas and ends with an export, so you
  can paste it as is to see the effect, then lift the middle lines into your own document
  (replace `d.pxo` with your file; keep the layer order).
* `scripts/` is this skill's `scripts/` folder (generators run by `render js`). Copy it next to
  your work, or write its full path.
* Sizes are for a **1080 x 1350** canvas. Lengths written as `%` scale by themselves. Plain
  pixel values (`--sigma`, `--blur`, `--offset`, `--stroke-width`, `--cell`, `--distance`)
  do not: multiply them by `canvas width / 1080`.
* `subject.png` is a cutout with transparency, `photo.png` an opaque photo, `product.png` a
  product cutout: **your image here**. With none at hand, make a stand-in to test the layout
  (`pixelox render js scripts/standin.js -o subject.png --size 900x1200 --param kind=person`;
  kinds: person, bottle, can, box, skyline, logo, scene), and never deliver a stand-in.
  Recipe sizes assume the stand-ins, which have empty space around the object: a tightly
  cropped real cutout needs a smaller `--size` (compare by eye, or `transform trim` both).
* Change `--seed` to get a different random pattern from any generator; the same seed always
  gives the same picture.
* New layers land on top. A technique that must sit behind the type is run before the type is
  added, or moved with `layer reorder`.
* Add `--json` to each call when you need to read the result (layer bounds, warnings).

## Index

| Group | IDs |
| --- | --- |
| Backgrounds (23) | T-gradient-mesh, T-smoke, T-fire-glow, T-spotlight, T-grunge, T-paper-grain, T-bokeh, T-light-leak, T-sunburst, T-waves, T-geo-shapes, T-pattern, T-torn-collage, T-brush-swash, T-splatter, T-light-streaks, T-fluid-ribbons, T-tile-mosaic, T-ring-arcs, T-spoke-burst, T-newsprint, T-swoosh-band, T-haze-band |
| Photo and subject (19) | T-cutout-drop-shadow, T-text-behind-subject, T-duotone, T-mono-on-color, T-halftone-portrait, T-stencil-poster, T-double-exposure, T-silhouette, T-photo-fade, T-glitch, T-motion-streak, T-color-grade, T-shape-crop, T-floor-reflection, T-water-splash, T-ice-cubes, T-condensation, T-torn-photo, T-device-mockup |
| Type (23) | T-fit-width, T-outline-type, T-gradient-type, T-image-in-type, T-distress-type, T-block-shadow, T-long-shadow, T-neon-glow, T-fire-type, T-vertical-type, T-arc-type, T-script-overlap, T-label-box, T-stretch-type, T-sliced-type, T-ghost-type, T-floor-type, T-counterchange, T-warp-type, T-stencil-type, T-texture-fill, T-crack, T-object-as-letter |
| Graphic furniture (7) | T-pill-button, T-price-burst, T-info-bar, T-frame-border, T-rules, T-lens-flare, T-credit-block |
| Finishing (2) | T-vignette, T-final-grade |

---

# Backgrounds

## T-gradient-mesh

**Purpose.** A soft multi-color aura: dark center, glowing colored corners. The modern event
and tech look. Type sits on the dark middle.

```bash
pixelox render js scripts/blobs.js -o mesh.png --size 1080x1350 --seed 4 --param bg='#140d0b' --param colors='#ff5a1f,#ffb000,#e83e8c,#7a3cff' --param count=7 --param size=0.6 --param edge=0.85
pixelox new -o d.pxo --size 1080x1350 --fill '#140d0b'
pixelox comp place d.pxo mesh.png --in-place --as Mesh --fit cover
pixelox filter gaussian d.pxo --in-place --layer Mesh --sigma 60
pixelox layer add d.pxo --in-place --name Grain --fill '#808080'
pixelox filter add-noise d.pxo --in-place --layer Grain --amount 14 --monochrome
pixelox layer blend d.pxo --in-place --layer Grain --mode overlay
pixelox layer opacity d.pxo --in-place --layer Grain --value 30
pixelox export d.pxo -o T-gradient-mesh.png
```

**Parameters.** `edge` 0.85 keeps the middle dark for type; 0 scatters color everywhere
(use with a light `bg` for pastel meshes). `size` 0.4 to 0.8: larger blobs, calmer picture.
`--sigma` 40 to 90: below 40 the blobs read as separate discs. `colors`: 2 to 4 hues that sit
next to each other on the wheel (orange, amber, pink) plus at most one contrast hue. Place
exact glows with `--param at='0.1,0.05;0.95,0.9'` (canvas fractions, one per blob).

**Fails when.** Banding (visible steps in the gradient): keep the Grain layer; it is what
hides them. Muddy gray-brown middle: the hues are complements; drop one or raise `edge`.

## T-smoke

**Purpose.** Drifting smoke or fog on a dark ground: moody concert, film, and product
backdrops.

```bash
pixelox render js scripts/clouds.js -o smoke.png --size 1080x1350 --seed 3 --param lo=transparent --param hi='#c9d2e0' --param scale=2.5 --param warp=1.2 --param stretch=1.6 --param bias=-0.12
pixelox render js scripts/clouds.js -o wisps.png --size 1080x1350 --seed 8 --param lo=transparent --param hi='#ffffff' --param scale=5 --param warp=1.6 --param stretch=2.2 --param bias=-0.2 --param res=2
pixelox new -o d.pxo --size 1080x1350 --fill '#07080c'
pixelox comp place d.pxo smoke.png --in-place --as Smoke --fit cover
pixelox filter gaussian d.pxo --in-place --layer Smoke --sigma 12
pixelox layer blend d.pxo --in-place --layer Smoke --mode screen
pixelox layer opacity d.pxo --in-place --layer Smoke --value 65
pixelox comp place d.pxo wisps.png --in-place --as Wisps --fit cover
pixelox filter gaussian d.pxo --in-place --layer Wisps --sigma 4
pixelox layer blend d.pxo --in-place --layer Wisps --mode screen
pixelox layer opacity d.pxo --in-place --layer Wisps --value 30
pixelox layer add d.pxo --in-place --name Shade
pixelox fill gradient d.pxo --in-place --layer Shade --from 0,35% --to 0,100% --stops '#07080c00@0,#07080cf2@1'
pixelox export d.pxo -o T-smoke.png
```

**Parameters.** `warp` 0.8 to 1.8 is the swirl; `stretch` above 1 makes the smoke rise;
`bias` -0.25 (thin wisps) to 0 (thick fog). `hi` tints it: `#c9d2e0` cold, `#e8c9a0` warm,
or a brand color for colored stage smoke. `--sigma` 8 to 20 softens. The Shade gradient
clears the lower part for text; flip its `--from`/`--to` to clear the top instead. For smoke
that rises from one object, use the live style on that layer instead:
`pixelox fx smoke d.pxo --in-place --layer Subject --height 80% --density 40`.

**Fails when.** It looks like marble or camouflage: blur more and lower the opacity. It
grays the whole picture: `bias` is too high, or the layer is not in `screen` mode.

## T-fire-glow

**Purpose.** A hot, swirling red-orange field with flying embers: revival, concert, and
sports flyers. No photo of fire needed.

```bash
pixelox render js scripts/clouds.js -o heat.png --size 1080x1350 --seed 11 --param lo='#3a0604' --param mid='#c2260f' --param hi='#ff9a3d' --param scale=2 --param octaves=4 --param warp=1.5 --param contrast=1.3
pixelox render js scripts/particles.js -o embers.png --size 1080x1350 --seed 5 --param shape=glow --param colors='#ffd27a,#ff8a1f,#fff1c9' --param count=140 --param min=0.15 --param max=0.7 --param grow=0.8
pixelox new -o d.pxo --size 1080x1350 --fill '#3a0604'
pixelox comp place d.pxo heat.png --in-place --as Heat --fit cover
pixelox filter gaussian d.pxo --in-place --layer Heat --sigma 16
pixelox filter radial-blur d.pxo --in-place --layer Heat --kind spin --amount 14 --center 50%,45%
pixelox layer add d.pxo --in-place --name Shade
pixelox fill gradient d.pxo --in-place --layer Shade --kind radial --from 50%,42% --to 50%,115% --stops '#1a030200@0.35,#1a0302d9@1'
pixelox comp place d.pxo embers.png --in-place --as Embers --fit cover
pixelox layer blend d.pxo --in-place --layer Embers --mode screen
pixelox export d.pxo -o T-fire-glow.png
```

**Parameters.** The three colors set the temperature: `#3a0604 / #c2260f / #ff9a3d` is fire;
`#02122e / #0b4fd1 / #5fd0ff` is the same effect as ice or water; `#12051f / #6a1bb3 /
#ff5ad1` is neon. `--amount` 8 to 25 is the swirl. `--sigma` 10 to 30: more blur reads as
glow, less as flame. For embers that streak, use `shape=streak` with `angle=-75`.

**Fails when.** White type is hard to read on the bright patches: put a `fx drop-shadow`
(offset 0,6, blur 18, opacity 45) on the headline, or strengthen the Shade layer.

## T-spotlight

**Purpose.** One flat color with a pool of light behind the hero and a contact shadow under
it: the classic product-ad stage.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#9c0f14'
pixelox layer add d.pxo --in-place --name Glow
pixelox fill gradient d.pxo --in-place --layer Glow --kind radial --from 50%,46% --to 50%,96% --stops '#ff6a55ff@0,#ff6a5500@1'
pixelox layer add d.pxo --in-place --name Corners
pixelox fill gradient d.pxo --in-place --layer Corners --kind radial --from 50%,46% --to 50%,125% --stops '#00000000@0.45,#000000a6@1'
pixelox shape ellipse d.pxo --in-place --name Shadow --rect 31%,80.5%,38%x5% --fill '#3a0204'
pixelox layer rasterize d.pxo --in-place --layer Shadow
pixelox filter gaussian d.pxo --in-place --layer Shadow --sigma 16
pixelox comp place d.pxo product.png --in-place --as Product --size x850 --at 50%,86% --anchor b
pixelox export d.pxo -o T-spotlight.png
```

**Parameters.** Glow color: the background hue, lighter and a little warmer (never white).
Glow radius (distance from `--from` to `--to`): 40 to 60% of the height. Shadow: 60 to 80%
of the product's width, 4 to 6% of the canvas height, color = the background darkened.
For a floor line, put a darker `shape rect` over the bottom 15 to 25% before the shadow.

**Fails when.** The product floats: the shadow is missing, too far below it, or too sharp.
A blurred shape is cut off at its own bounds: for a large, very soft shadow use a
canvas-sized layer instead (`layer add`, then `select ellipse --feather 40`, then
`fill solid --mask @sel`). Stand-in and loosely cropped cutouts have empty space under
the object: `transform trim` the file first so the shadow meets it.
The glow looks like a sticker: its edge color must have alpha 0 of the *same* hue
(`#ff6a5500`), not transparent black.

## T-grunge

**Purpose.** A worn, dirty, printed-and-handled surface: street, underground, vintage, and
protest posters. Works under bold black type.

```bash
pixelox render js scripts/clouds.js -o mottle.png --size 1080x1350 --seed 21 --param lo='#8a7a62' --param hi='#f6eedc' --param scale=4 --param octaves=6 --param warp=0.4
pixelox render js scripts/grunge.js -o dirt.png --size 1080x1350 --seed 9 --param color='#2a2118' --param specks=7000 --param blotches=90 --param scratches=45
pixelox new -o d.pxo --size 1080x1350 --fill '#ddd3bf'
pixelox comp place d.pxo mottle.png --in-place --as Mottle --fit cover
pixelox layer blend d.pxo --in-place --layer Mottle --mode multiply
pixelox layer opacity d.pxo --in-place --layer Mottle --value 60
pixelox comp place d.pxo dirt.png --in-place --as Dirt --fit cover
pixelox layer opacity d.pxo --in-place --layer Dirt --value 55
pixelox layer add d.pxo --in-place --name Grain --fill '#808080'
pixelox filter add-noise d.pxo --in-place --layer Grain --amount 22 --monochrome
pixelox layer blend d.pxo --in-place --layer Grain --mode overlay
pixelox layer opacity d.pxo --in-place --layer Grain --value 45
pixelox export d.pxo -o T-grunge.png
```

**Parameters.** Paper color `#ddd3bf` (aged), `#e9e6e1` (newsprint), `#1a1a1a` (dark wall:
then use `color='#ffffff'` dirt in `screen` mode and mottle `lo='#000000' hi='#555555'`).
`clump` 0.6 leaves clean patches; 0 dirties evenly. `size=2` gives coarser marks for big
prints. Put Dirt and Grain **above** photos and flat shapes too, so everything shares one
surface; keep small text above them.

**Fails when.** It looks like digital noise rather than wear: too many specks at one size;
lower `specks`, raise `blotches`, keep `clump` at 0.5 or more.

## T-paper-grain

**Purpose.** Fine film or paper grain over the finished design. Removes the "too clean,
too digital" look and hides gradient banding. Use it on almost everything, last.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#e8452c'
pixelox layer add d.pxo --in-place --name Grain --fill '#808080'
pixelox filter add-noise d.pxo --in-place --layer Grain --amount 18 --monochrome
pixelox layer blend d.pxo --in-place --layer Grain --mode overlay
pixelox layer opacity d.pxo --in-place --layer Grain --value 40
pixelox export d.pxo -o T-paper-grain.png
```

**Parameters.** `--amount` 8 to 14 with opacity 25 to 35 is invisible-but-felt (clean
corporate work); 18 to 30 with opacity 40 to 60 is visible print grain (retro, editorial).
`soft-light` instead of `overlay` is gentler on dark designs. For coarser grain on a large
canvas, add `pixelox filter gaussian d.pxo --in-place --layer Grain --sigma 0.7` after the
noise.

**Fails when.** Small text turns gritty: move the text layers above Grain
(`pixelox layer reorder d.pxo --in-place --layer Grain --below Body`).

## T-bokeh

**Purpose.** Out-of-focus points of light: night, celebration, festive, worship backdrops.

```bash
pixelox render js scripts/blobs.js -o far.png --size 1080x1350 --seed 4 --param bg=transparent --param colors='#ffd27a,#ff8a1f,#ffffff' --param count=40 --param size=0.11 --param soft=0.3 --param alpha=0.3 --param mode=screen
pixelox render js scripts/blobs.js -o near.png --size 1080x1350 --seed 12 --param bg=transparent --param colors='#ffd27a,#ffb04a' --param count=60 --param size=0.05 --param soft=0.25 --param alpha=0.45 --param mode=screen
pixelox new -o d.pxo --size 1080x1350 --fill '#140a05'
pixelox comp place d.pxo far.png --in-place --as Far --fit cover
pixelox filter gaussian d.pxo --in-place --layer Far --sigma 9
pixelox layer blend d.pxo --in-place --layer Far --mode screen
pixelox comp place d.pxo near.png --in-place --as Near --fit cover
pixelox filter gaussian d.pxo --in-place --layer Near --sigma 2
pixelox layer blend d.pxo --in-place --layer Near --mode screen
pixelox export d.pxo -o T-bokeh.png
```

**Parameters.** Two depths sell it: big, faint, blurred discs (Far) and small, brighter ones
(Near). `soft` 0.2 to 0.35 keeps a disc with a soft rim; 1 turns them into glows. Keep the
colors within one family plus white. To keep the middle clear for a subject, add
`--param edge=0.7`.

**Fails when.** It reads as polka dots: the discs are too opaque or too evenly sized; lower
`alpha`, keep `vary` at its default.

## T-light-leak

**Purpose.** A warm wash of light entering from an edge or corner, as on film. Adds warmth
and depth over a photo or a dark layout.

```bash
pixelox render js scripts/blobs.js -o leak.png --size 1080x1350 --seed 2 --param bg=transparent --param colors='#ff7a1a,#ffd36b,#ff3d6e' --param at='1.0,0.05;0.85,0.0;1.05,0.35' --param size=0.55 --param alpha=0.9
pixelox new -o d.pxo --size 1080x1350 --fill '#101216'
pixelox comp place d.pxo photo.png --in-place --as Photo --fit cover
pixelox comp place d.pxo leak.png --in-place --as Leak --fit cover
pixelox filter gaussian d.pxo --in-place --layer Leak --sigma 60
pixelox layer blend d.pxo --in-place --layer Leak --mode screen
pixelox export d.pxo -o T-light-leak.png
```

**Parameters.** `at` lists the blob centers as canvas fractions; values just outside 0..1
put the source off-canvas, which is what makes it a leak. Orange + yellow + a touch of
magenta is the film look; cyan + blue for a cold leak. Tame it with `layer opacity` 50 to 90.

**Fails when.** It covers the face or the headline: move the centers to the opposite corner.

## T-sunburst

**Purpose.** Rays fanning out from a point: retro sale posters, sunrise and "glory" church
flyers, comic bursts behind a product.

```bash
pixelox render js scripts/rays.js -o rays.png --size 1080x1350 --param color='#fff3c4' --param cx=0.5 --param cy=0.3 --param count=22 --param alpha=0.5 --param fade=0.9
pixelox new -o d.pxo --size 1080x1350 --fill '#e8a200'
pixelox layer add d.pxo --in-place --name Core
pixelox fill gradient d.pxo --in-place --layer Core --kind radial --from 50%,30% --to 50%,100% --stops '#fff6c9ff@0,#ffd23cff@0.25,#e8a20000@1'
pixelox comp place d.pxo rays.png --in-place --as Rays --fit cover
pixelox layer blend d.pxo --in-place --layer Rays --mode screen
pixelox layer add d.pxo --in-place --name Floor
pixelox fill gradient d.pxo --in-place --layer Floor --from 0,62% --to 0,100% --stops '#a8480000@0,#a84800e6@1'
pixelox export d.pxo -o T-sunburst.png
```

**Parameters.** `fade=0` with `alpha=1` and two flat colors gives the hard retro burst
(for example `#f2c14e` rays on `#e9a23b`). `uneven=0.8` with `count=40`, `spread=70`,
`cy=-0.1` gives natural light shafts falling from above (blur them with `--sigma 6`).
`count` 12 to 30. Put the center behind the hero (the title or the product). For firework
bursts of separate round-capped lines, see T-spoke-burst.

**Fails when.** Rays compete with the type: lower `alpha` to 0.2 to 0.35 or blur the layer.

## T-waves

**Purpose.** Layered flowing bands (water, hills, ribbons) or thin contour lines: fresh,
friendly, brand-ad and summer looks.

```bash
pixelox render js scripts/waves.js -o bands.png --size 1080x1350 --seed 8 --param colors='#ffffff,#e11b22,#ffffff,#e11b22,#ffffff' --param count=5 --param top=0.62 --param bottom=0.95 --param amp=0.035 --param freq=1.0 --param chaos=0.7
pixelox render js scripts/waves.js -o lines.png --size 1080x1350 --seed 3 --param mode=line --param colors='#ffffff' --param count=16 --param top=0.05 --param bottom=0.5 --param amp=0.02 --param chaos=0.1 --param width=2 --param alpha=0.35
pixelox new -o d.pxo --size 1080x1350 --fill '#e11b22'
pixelox comp place d.pxo lines.png --in-place --as Lines --fit cover
pixelox comp place d.pxo bands.png --in-place --as Bands --fit cover
pixelox export d.pxo -o T-waves.png
```

**Parameters.** `count` 3 to 7 bands; alternate two colors for stripes of water, or step
one hue from dark to light for depth (`#0b2a6f,#1557c0,#3aa0ff`). `amp` 0.02 (calm) to 0.1
(stormy); `freq` 0.6 to 2.5 waves across the width; `chaos` 0 makes parallel bands, 1 makes
each band its own shape. `mode=line` with `count` 12 to 40 gives topographic lines for a
quiet texture.

**Fails when.** The bands look mechanical: raise `chaos`, lower `freq`.

## T-geo-shapes

**Purpose.** Flat geometric shapes (half discs, quarter rings, bars, dots, triangles) in a
tight cluster: Bauhaus, museum, art-fair, and playful corporate posters.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#0d0d0d'
pixelox shape path d.pxo --in-place --name HalfRing --d 'M 250 250 A 280 280 0 0 0 810 250 L 680 250 A 150 150 0 0 1 380 250 Z' --fill '#ffc20e'
pixelox shape rect d.pxo --in-place --name Bar --rect 250,500,80x480 --fill '#ff5c8a'
pixelox shape path d.pxo --in-place --name Quarter --d 'M 500 580 L 740 580 A 240 240 0 0 1 500 820 Z' --fill '#12a150'
pixelox shape ellipse d.pxo --in-place --name Dot --rect 360,760,190x190 --fill '#ff6a13'
pixelox shape path d.pxo --in-place --name Arch --d 'M 250 1180 A 280 280 0 0 1 810 1180 L 680 1180 A 150 150 0 0 0 380 1180 Z' --fill '#3ec5f4'
pixelox shape polygon d.pxo --in-place --name Tri --center 830,960 --radius 90 --sides 3 --rotation 90 --fill '#ffffff'
pixelox shape ellipse d.pxo --in-place --name Ring --rect 770,640,100x100 --fill none --stroke '#ff2b2b' --stroke-width 22
pixelox export d.pxo -o T-geo-shapes.png
```

**Parameters.** Build on a square unit (here 280 px = about 26% of the width) so every
shape is 1, 1/2, or 1/3 of it and edges line up. 4 to 6 colors at full saturation on
black, white, or one strong ground. Arcs: `A rx ry 0 0 1 x y` draws clockwise, `0 0 0`
counter-clockwise; a half ring is an outer arc, a line in, an inner arc back, `Z`.

**Fails when.** It looks like scattered clip-art: shapes must touch or share edges, and the
cluster needs one clear outer silhouette (a column or a square).

## T-pattern

**Purpose.** Flat repeat patterns as a quiet texture or accent: a halftone dot field that
fades out, diagonal stripes, target rings, a grid.

```bash
pixelox render js scripts/pattern.js -o dots.png --size 1080x1350 --param kind=dots --param color='#ffffff' --param cell=30 --param weight=0.36 --param fade=down --param alpha=0.5
pixelox render js scripts/pattern.js -o stripes.png --size 420x420 --param kind=stripes --param color='#ffd400' --param cell=36 --param weight=0.5 --param angle=45
pixelox render js scripts/pattern.js -o rings.png --size 1080x1350 --param kind=rings --param color='#ffffff' --param cell=46 --param weight=0.12 --param cx=0.5 --param cy=0.8 --param fade=out --param alpha=0.6
pixelox new -o d.pxo --size 1080x1350 --fill '#1b2bd6'
pixelox comp place d.pxo dots.png --in-place --as Dots --fit cover
pixelox comp place d.pxo rings.png --in-place --as Rings --fit cover
pixelox comp place d.pxo stripes.png --in-place --as Stripes --at 100%,0 --anchor tr
pixelox export d.pxo -o T-pattern.png
```

**Parameters.** Kinds: `dots`, `stripes`, `grid`, `rings`, `checker`, `zigzag`, `scallop`
(fish-scale arcs for cloth and wave borders), `barcode` (render it small, as a label
detail). `cell` 20 to 40 for dots (2 to 4% of the width), `weight` 0.2 to 0.45.
`fade` = `down`, `up`, `left`, `right`, `out`. Use one pattern per design, two at most, at
low contrast against the ground (alpha 0.15 to 0.5), in a corner or a band, not full-bleed
behind text.

**Fails when.** Text shimmers on top of it: clear the pattern behind text with a solid
panel, or fade it out before it reaches the text block.

## T-torn-collage

**Purpose.** Scraps of torn paper layered at small angles with paint: zine, punk,
protest, and cut-and-paste editorial looks.

```bash
pixelox render js scripts/torn.js -o scrap1.png --size 1200x520 --seed 3 --param color='#e9e3d5' --param edges=tb
pixelox render js scripts/torn.js -o scrap2.png --size 900x380 --seed 7 --param color='#cfc7b5' --param edges=tblr --param rough=0.8
pixelox render js scripts/brush.js -o paint.png --size 1300x460 --seed 2 --param color='#f28c13' --param bristles=90 --param dry=0.5
pixelox render js scripts/grunge.js -o dirt.png --size 1080x1350 --seed 9 --param color='#1c1712' --param specks=5000
pixelox new -o d.pxo --size 1080x1350 --fill '#bdb6a6'
pixelox comp place d.pxo scrap1.png --in-place --as Scrap1 --at 50%,22% --anchor c
pixelox transform rotate d.pxo --in-place --layer Scrap1 --degrees -5
pixelox fx drop-shadow d.pxo --in-place --layer Scrap1 --offset 4,8 --blur 14 --opacity 35
pixelox comp place d.pxo paint.png --in-place --as Paint --at 50%,52% --anchor c
pixelox transform rotate d.pxo --in-place --layer Paint --degrees 18
pixelox layer blend d.pxo --in-place --layer Paint --mode multiply
pixelox comp place d.pxo scrap2.png --in-place --as Scrap2 --at 58%,80% --anchor c
pixelox transform rotate d.pxo --in-place --layer Scrap2 --degrees 4
pixelox fx drop-shadow d.pxo --in-place --layer Scrap2 --offset 4,8 --blur 14 --opacity 35
pixelox comp place d.pxo dirt.png --in-place --as Dirt --fit cover
pixelox layer opacity d.pxo --in-place --layer Dirt --value 45
pixelox export d.pxo -o T-torn-collage.png
```

**Parameters.** Rotate scraps between -8 and 8 degrees, never 0, and let them run off the
canvas. A soft, short drop shadow (blur 10 to 16, opacity 25 to 40) lifts each scrap.
Paint in `multiply` soaks into the paper. To tear a **photo**: render a scrap the size of
the canvas, `mask import` it, and `mask add --from` that mask on the photo layer.
Real newsprint or handwriting inside the scraps has to come from your own scans or photos
(place them and clip them to the scrap with `layer clip`).

**Fails when.** Everything is parallel and the same size: vary size by at least 1.5x and
angle by at least 6 degrees between neighbors.

## T-brush-swash

**Purpose.** One loaded brush stroke as a color field behind a product, a word, or a price:
food ads, handmade and energetic looks.

```bash
pixelox render js scripts/brush.js -o swash.png --size 1000x520 --seed 2 --param color='#f5a623' --param bristles=120 --param passes=3
pixelox new -o d.pxo --size 1080x1350 --fill '#e8c9a8'
pixelox comp place d.pxo swash.png --in-place --as Swash --at 50%,58% --anchor c
pixelox transform rotate d.pxo --in-place --layer Swash --degrees -4
pixelox comp place d.pxo product.png --in-place --as Product --size x720 --at 50%,52% --anchor c
pixelox fx drop-shadow d.pxo --in-place --layer Product --offset 0,18 --blur 30 --opacity 35
pixelox export d.pxo -o T-brush-swash.png
```

**Parameters.** Render the swash at its final size: canvas width 80 to 95% of the design,
height 30 to 45% of that. `bristles` 60 (dry, gappy) to 200 (solid); `passes` 2 to 3;
`dry` 0.2 to 0.6. Tilt it -3 to -8 degrees. For a small highlight behind one word, render
it about 1.3x the word's width and 1.6x its height.

**Fails when.** It looks like a striped rectangle: raise `rough` to 0.7 and use 3 passes.

## T-splatter

**Purpose.** Thrown ink or paint: thriller and horror covers, sports energy, grunge accents;
also a rough mask for a portrait.

```bash
pixelox render js scripts/splatter.js -o splat.png --size 1080x1080 --seed 6 --param color='#8f0f16' --param radius=0.2 --param drops=320 --param arms=10
pixelox new -o d.pxo --size 1080x1350 --fill '#f1efe9'
pixelox comp place d.pxo splat.png --in-place --as Splat --at 50%,46% --anchor c
pixelox export d.pxo -o T-splatter.png
```

**Parameters.** `radius` 0.12 to 0.3 (core size), `drops` 100 to 500, `reach` 1.5 to 3,
`arms` 0 to 14. `dir=90` throws it downward (drips), `dir=0` to the right. Two or three
splats with different seeds and sizes look more natural than one. To show a photo only
inside the splat, place the photo above it and clip it:
`pixelox layer clip d.pxo --in-place --layer Photo`.

**Fails when.** It reads as a cartoon blob: add a second, smaller splat off-center and
overlay T-grunge specks in the same color.

## T-light-streaks

**Purpose.** Defocused streaks of light crossing a dark ground: tech expos, launches,
night-drive and stage moods. Type sits on the dark gaps between streaks.

```bash
pixelox render js scripts/blobs.js -o lights.png --size 1080x1350 --param bg=transparent --param colors='#9a80e0,#6a3fd0,#4a14c8,#a898dc,#5a2ad0' --param at='0.2,0.34;0.38,0.56;0.56,0.76;0.82,0.24;0.96,0.5' --param size=0.16 --param soft=0.6 --param alpha=1
pixelox render js scripts/particles.js -o lines.png --size 1080x1350 --seed 4 --param shape=streak --param colors='#d9ccff,#ffffff' --param count=26 --param min=0.3 --param max=0.9 --param angle=52 --param jitter=3 --param amin=0.15 --param amax=0.5
pixelox new -o d.pxo --size 1080x1350 --fill '#02020f'
pixelox comp place d.pxo lights.png --in-place --as Streaks --fit cover
pixelox filter motion-blur d.pxo --in-place --layer Streaks --angle=-52 --distance 420
pixelox filter gaussian d.pxo --in-place --layer Streaks --sigma 16
pixelox layer blend d.pxo --in-place --layer Streaks --mode screen
pixelox comp place d.pxo lines.png --in-place --as Lines --fit cover
pixelox filter motion-blur d.pxo --in-place --layer Lines --angle=-52 --distance 160
pixelox layer blend d.pxo --in-place --layer Lines --mode screen
pixelox export d.pxo -o T-light-streaks.png
```

**Parameters.** Each blob in `at` becomes one streak; place 4 to 6 along two diagonals and
leave the center empty for the title. `--distance` 30 to 45% of the width sets the streak
length, `--sigma` 10 to 25 the softness. One hue family (violet here; amber
`#ffb347,#ff7a1a` for warm stage light). Note the sign: `motion-blur --angle=-52` and
particles `angle=52` point the same way. The thin Lines layer adds a few sharp trails; drop
it for a calmer field.

**Fails when.** It reads as blurry dots: the motion blur is too short. Text is unreadable:
a streak runs behind it; move that blob.

## T-fluid-ribbons

**Purpose.** Silky, twisting gradient ribbons: abstract corporate and tech backgrounds,
music and fashion energy without a photo.

```bash
pixelox render js scripts/ribbons.js -o ribbons.png --size 1080x1350 --seed 6 --param colors='#ff7a00,#ff2e88,#2b6bff,#18d6ff' --param count=3 --param width=0.24
pixelox new -o d.pxo --size 1080x1350 --fill '#0a1030'
pixelox comp place d.pxo ribbons.png --in-place --as Ribbons --at 62%,50% --anchor c
pixelox layer dup d.pxo --in-place --layer Ribbons --name Glow
pixelox filter gaussian d.pxo --in-place --layer Glow --sigma 28
pixelox layer blend d.pxo --in-place --layer Glow --mode screen
pixelox layer opacity d.pxo --in-place --layer Glow --value 55
pixelox export d.pxo -o T-fluid-ribbons.png
```

**Parameters.** `twist` 0 gives flat bands, 1 to 2 gives ribbons that turn over; `sway`
0.1 to 0.3 is the swing; `dir=h` runs them left to right. Each ribbon blends from one
color to the next in `colors`. Push the layer off-center (here 62%) so one side stays clear
for a left-hung headline. The blurred Glow copy makes it look lit.

**Fails when.** Ribbons look like scribbles: `strands` too low for the width; keep 60 or
more, or lower `width`.

## T-tile-mosaic

**Purpose.** A panel of square tiles with quarter discs, half discs, dots, and bars in
flat palette colors: corporate events, festivals, annual reports.

```bash
pixelox render js scripts/tiles.js -o tiles.png --size 540x1350 --seed 2 --param cols=3 --param colors='#e63946,#f1c40f,#1d3557,#2ec4b6,#ff7f11'
pixelox new -o d.pxo --size 1080x1350 --fill '#f4f1de'
pixelox comp place d.pxo tiles.png --in-place --as Tiles --at 100%,0 --anchor tr
pixelox export d.pxo -o T-tile-mosaic.png
```

**Parameters.** Render the mosaic at the exact size of the panel it fills (here the right
half). `cols` 2 to 5; tiles are squares, so rows follow. `motifs` picks the shapes
(`qhd` for only curves, `tb` for hard geometry). 4 to 6 colors of equal strength; include
the page color so some tiles look empty. Text lives on the plain half.

**Fails when.** It fights the text: never put small text over the mosaic; give it its own
column or a solid panel.

## T-ring-arcs

**Purpose.** Parts of big circles as graphic structure: thick flat arcs that frame a
subject, or one glowing arc sweeping across a dark ground.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#12100f'
pixelox layer add d.pxo --in-place --name GlowWide
pixelox paint stroke d.pxo --in-place --layer GlowWide --d 'M -80 420 A 760 760 0 0 1 760 1420' --size 260 --hardness 0 --flow 22 --color '#ff6a00'
pixelox layer blend d.pxo --in-place --layer GlowWide --mode screen
pixelox layer add d.pxo --in-place --name GlowCore
pixelox paint stroke d.pxo --in-place --layer GlowCore --d 'M -80 420 A 760 760 0 0 1 760 1420' --size 46 --hardness 20 --flow 60 --color '#ffd9a0'
pixelox layer blend d.pxo --in-place --layer GlowCore --mode screen
pixelox shape path d.pxo --in-place --name Arc1 --d 'M 640 250 A 300 300 0 0 1 940 550' --fill none --stroke '#2ec4b6' --stroke-width 70
pixelox shape path d.pxo --in-place --name Arc2 --d 'M 760 250 A 180 180 0 0 1 940 430' --fill none --stroke '#f4f1de' --stroke-width 26
pixelox export d.pxo -o T-ring-arcs.png
```

**Parameters.** A glow arc is painted, not blurred: a wide, soft, low-flow stroke
(`--hardness 0`, size 20 to 30% of the width) under a narrow bright one, both in `screen`
mode on canvas-sized layers, so nothing is clipped. Flat arcs are `shape path` arcs with
`--fill none` and a thick stroke; concentric arcs share a center and differ in radius by
the stroke width plus a gap. Arc syntax: `A rx ry 0 0 1 x y` (clockwise). For a circle
with center `cx,cy` and radius `r`: a full ring is `M cx-r cy A r r 0 1 1 cx+r cy A r r 0 1 1
cx-r cy` (work the sums out: center 540,600 and r 300 gives `M 240 600 A 300 300 0 1 1 840
600 A 300 300 0 1 1 240 600`); the upper half alone is its first arc, the lower half its
second. Concentric rings with a gap at the bottom or top (a record, sound waves, a target)
are three such arcs at radii in the ratio 1 : 1.8 : 2.5.

**Fails when.** A blurred shape arc is cut off in a box: `filter gaussian` on a rasterized
shape stops at the shape's bounds; paint the glow instead, as here.

## T-spoke-burst

**Purpose.** Fireworks, sparkles, and sunbursts made of round-capped spokes: festivals,
celebrations, playful accents around a title.

```bash
pixelox render js scripts/rays.js -o burst1.png --size 520x520 --param style=spokes --param count=18 --param color='#ff5fa2' --param inner=0.07 --param outer=0.47 --param width=0.035
pixelox render js scripts/rays.js -o burst2.png --size 300x300 --param style=spokes --param count=14 --param color='#ffb21a' --param inner=0.08 --param outer=0.46 --param width=0.03
pixelox render js scripts/rays.js -o burst3.png --size 240x240 --param style=spokes --param count=12 --param color='#8a5cff' --param inner=0.1 --param outer=0.46 --param width=0.04
pixelox new -o d.pxo --size 1080x1350 --fill '#1b1413'
pixelox comp place d.pxo burst1.png --in-place --as Burst1 --at 76%,33% --anchor c
pixelox comp place d.pxo burst2.png --in-place --as Burst2 --at 40%,24% --anchor c
pixelox comp place d.pxo burst3.png --in-place --as Burst3 --at 6%,60% --anchor c
pixelox export d.pxo -o T-spoke-burst.png
```

**Parameters.** Render each burst on its own small square canvas; `inner` and `outer` are
fractions of that square, `width` the spoke thickness. `count` 10 to 24. Three bursts in
three sizes (ratio about 1 : 0.6 : 0.45) and three palette colors, one of them cut by the
canvas edge. `uneven=0.6` varies the spoke lengths for a hand-drawn feel.

**Fails when.** Bursts sit in a row at one size: vary size and let them overlap the type.

## T-newsprint

**Purpose.** A sheet of newspaper as texture: collage scraps, grunge grounds, "headline"
moments. Built from real type, so it survives a close look.

```bash
pixelox new -o news.pxo --size 1200x900 --fill '#e6e0d2'
pixelox text add news.pxo --in-place --name Masthead --content "THE DAILY EXAMPLE" --font "Times New Roman" --weight 700 --size 9% --color '#1a1714' --at 50%,3% --anchor t
pixelox shape line news.pxo --in-place --name Rule --from 3%,15% --to 97%,15% --stroke '#1a1714' --stroke-width 4
pixelox text add news.pxo --in-place --name Head --content "NIGHT MARKET TO STAY OPEN ALL SUMMER" --font "Times New Roman" --weight 700 --size 5% --color '#1a1714' --at 3%,17.5%
pixelox text add news.pxo --in-place --name Col1 --content "Traders packed the square on Friday as the council voted to keep the stalls open until the end of the season. Musicians played from the steps of the old hall while the crowd spilled into the side streets. Organisers said more than two hundred stalls had applied for a place, and that the lanterns would stay lit every weekend. Neighbours asked for quieter nights after eleven, and the committee agreed to move the stage." --font "Times New Roman" --size 19px --leading 122% --box 360x560 --align justify --color '#1a1714' --at 3%,27%
pixelox text add news.pxo --in-place --name Col2 --content "A second vote next month will decide whether the market may grow along the river walk. Shop owners welcomed the plan, saying evening trade had doubled since spring. The oldest stallholder, who has sold bread here for forty years, said she had never seen the square so full. Rain is forecast for the opening night, but the organisers have promised canopies, hot drinks, and a brass band." --font "Times New Roman" --size 19px --leading 122% --box 360x560 --align justify --color '#1a1714' --at 35%,27%
pixelox text add news.pxo --in-place --name Col3 --content "Elsewhere in the city, the tram line reopened after three weeks of repairs, and the library announced longer hours. A lost dog was returned to its owner after a search that reached four districts. The weather office expects warm days and clear skies through the weekend, with a light wind from the west and a full moon on Sunday." --font "Times New Roman" --size 19px --leading 122% --box 360x560 --align justify --color '#1a1714' --at 67%,27%
pixelox export news.pxo -o news.png
pixelox new -o d.pxo --size 1080x1350 --fill '#c9c1b2'
pixelox comp place d.pxo news.png --in-place --as News1 --size 1500x --at 50%,30% --anchor c
pixelox transform rotate d.pxo --in-place --layer News1 --degrees -7
pixelox comp place d.pxo news.png --in-place --as News2 --size 1300x --at 55%,82% --anchor c
pixelox transform rotate d.pxo --in-place --layer News2 --degrees 94
pixelox layer blend d.pxo --in-place --layer News2 --mode multiply
pixelox export d.pxo -o T-newsprint.png
```

**Parameters.** Body text at 1.6 to 2.2% of the sheet height, justified, three or four
columns, with one bold headline and a rule; write your own filler sentences. Place the
sheet 1.2 to 1.5x wider than the poster, rotated, more than once, upside down and sideways
too, so it reads as texture and not as an article. Then tear it (clip to a T-torn-collage
scrap), age it (T-grunge), or brush paint over it (T-brush-swash in `multiply`).

**Fails when.** People start reading it: it is too straight and too large; rotate and
overlap the sheets, and cover the middle with the real headline.

## T-swoosh-band

**Purpose.** A curved color band that sweeps across the layout and splits it into two
fields: template-style product and corporate promos.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#2b2a30'
pixelox shape path d.pxo --in-place --name BandBack --d 'M 0 930 C 300 760 700 1120 1080 800 L 1080 1350 L 0 1350 Z' --fill '#ffd166'
pixelox shape path d.pxo --in-place --name Band --d 'M 0 990 C 320 830 720 1190 1080 880 L 1080 1350 L 0 1350 Z' --fill '#f5a800'
pixelox comp place d.pxo product.png --in-place --as Product --size x640 --at 50%,84% --anchor b
pixelox fx drop-shadow d.pxo --in-place --layer Product --offset 0,20 --blur 40 --opacity 40
pixelox export d.pxo -o T-swoosh-band.png
```

**Parameters.** One cubic curve: `M` start on the left edge, `C` two control points (the
first above, the second below the line for an S shape), end on the right edge, then down
and back along the bottom. A second band 5% higher in a lighter tint gives a rim. The
product stands on the band's edge so it overlaps both fields.

**Fails when.** The curve has a kink: keep the control points roughly a third and two
thirds of the way across.

## T-haze-band

**Purpose.** A soft band of glow or fog along the bottom that hides where cutouts are cut
off and where a photo ends. The quiet fix behind most multi-person flyers.

```bash
pixelox render js scripts/clouds.js -o fog.png --size 1080x1350 --seed 5 --param lo=transparent --param hi='#ffd9a0' --param scale=3 --param warp=1 --param bias=-0.1
pixelox new -o d.pxo --size 1080x1350 --fill '#1a0f0a'
pixelox comp place d.pxo subject.png --in-place --as Left --size x760 --at 30%,96% --anchor b
pixelox comp place d.pxo subject2.png --in-place --as Right --size x760 --at 70%,96% --anchor b
pixelox layer add d.pxo --in-place --name Haze
pixelox fill gradient d.pxo --in-place --layer Haze --from 0,66% --to 0,100% --stops '#e8913a00@0,#e8913acc@0.55,#ffd9a0ff@1'
pixelox comp place d.pxo fog.png --in-place --as Fog --fit cover
pixelox filter gaussian d.pxo --in-place --layer Fog --sigma 14
pixelox mask add d.pxo --in-place --layer Fog --from-path 'M 0 880 H 1080 V 1350 H 0 Z'
pixelox mask feather d.pxo --in-place --layer Fog --radius 60
pixelox layer blend d.pxo --in-place --layer Fog --mode screen
pixelox export d.pxo -o T-haze-band.png
```

**Parameters.** The gradient runs from the glow color at alpha 0 (where the bodies are
still clean, about two thirds down) to solid at the bottom edge; the fog on top breaks up
its straight edge. Use the scene's light color (amber, white, the ground color for a
fade to nothing). The headline or info bar then sits on the band.

**Fails when.** A hard line shows where the gradient starts: lengthen the ramp to at least
30% of the height.

---

# Photo and subject

**Getting a cutout.** Most subject techniques need the subject on a transparent background.
With a photo on a plain backdrop: `pixelox select color shot.pxo --in-place --at 4,4 --contiguous --tolerance 40`,
then `pixelox select invert shot.pxo --in-place`, then `pixelox export shot.pxo -o subject.png --apply-mask @sel`.
On any backdrop, use the local model (ask the user before the first download):
`pixelox detect subject shot.pxo --in-place --save-mask subject -y`, then
`pixelox mask refine shot.pxo --in-place --mask subject --decontaminate`, then
`pixelox export shot.pxo -o subject.png --apply-mask subject`.

## T-cutout-drop-shadow

**Purpose.** Lift a cutout off a flat ground. Three standard looks: a soft studio shadow, a
hard offset "sticker" shadow, and a white die-cut outline.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#f3c613'
pixelox comp place d.pxo subject.png --in-place --as Subject --size x1050 --at 50%,100% --anchor b
pixelox fx stroke d.pxo --in-place --layer Subject --width 12 --color '#ffffff' --position outside
pixelox fx drop-shadow d.pxo --in-place --layer Subject --offset 22,22 --blur 0 --color '#161616' --opacity 100
pixelox export d.pxo -o T-cutout-drop-shadow.png
```

**Parameters.** Soft studio shadow: `--offset 0,24 --blur 60 --opacity 40`, no stroke.
Hard graphic shadow: `--blur 0`, offset 1.5 to 3% of the width, in black or a darker shade
of the ground. Die-cut outline: stroke 8 to 16 px white. Glow on a dark ground:
`pixelox fx outer-glow d.pxo --in-place --layer Subject --blur 60 --color '#ffb347' --opacity 60`.

**Fails when.** A fringe of the old background shows inside the outline: refine the mask
(`mask refine --decontaminate`) or `mask contract` it by 1 to 2 px before exporting.

## T-text-behind-subject

**Purpose.** A giant word behind the subject so the head or shoulders overlap the letters:
instant depth, the signature magazine-cover and church-flyer move.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#f2460d'
pixelox text add d.pxo --in-place --name Word --content "SUNDAY" --font Impact --size 20% --color '#161616' --at 50%,9% --anchor t
pixelox transform scale d.pxo --in-place --layer Word --size 980x --anchor t
pixelox comp place d.pxo subject.png --in-place --as Subject --size x1190 --at 50%,100% --anchor b
pixelox fx drop-shadow d.pxo --in-place --layer Subject --offset 0,10 --blur 40 --opacity 45
pixelox export d.pxo -o T-text-behind-subject.png
```

**Parameters.** The word spans 85 to 100% of the width and its cap height is 25 to 45% of
the canvas height. The subject must cover 15 to 40% of the word and leave the first and
last letters clear, so the word still reads. Order: ground, word, subject, then small
text on top. A second line of the word can pass in front of the body (put it above the
subject) for an interlocked look.

**Fails when.** The word becomes unreadable: the subject hides too many letter shapes;
move the word up, widen it, or use a shorter word. Thin fonts fail here; use a heavy
condensed face.

## T-duotone

**Purpose.** Recolor a photo with two or three brand colors mapped from shadows to
highlights. Unifies mixed photos and makes stock imagery look designed.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#1b1464'
pixelox comp place d.pxo photo.png --in-place --as Photo --fit cover
pixelox adjust gradient-map d.pxo --in-place --layer Photo --stops '#1b1464@0,#ff3d7f@0.55,#ffe66d@1'
pixelox export d.pxo -o T-duotone.png
```

**Parameters.** First stop = shadows (dark, saturated), last = highlights (light). Two stops
is a true duotone; a middle stop at 0.4 to 0.6 adds punch. Proven pairs: navy `#0b1f4b` to
cream `#f4e9d8`; purple `#2d0b59` to coral `#ff6b5e`; black to red `#e10600` to white;
forest `#0b3d2e` to lime `#d7f171`. Raise contrast first with `adjust levels` when the photo
is flat. Keep it editable with `--as-layer` (then it tints everything below it).

**Fails when.** Faces go muddy: the two colors are too close in lightness; make the shadow
stop darker than 25% and the highlight stop lighter than 80% lightness.

## T-mono-on-color

**Purpose.** A black-and-white, high-contrast subject on one flat saturated color: book
covers, fashion and music editorial. The color does the shouting, the photo stays calm.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#ef3340'
pixelox comp place d.pxo subject.png --in-place --as Subject --size x1150 --at 50%,100% --anchor b
pixelox adjust bw d.pxo --in-place --layer Subject --preset high-contrast-red
pixelox adjust levels d.pxo --in-place --layer Subject --in-black 18 --in-white 228 --gamma 1.1
pixelox export d.pxo -o T-mono-on-color.png
```

**Parameters.** `--in-black` 10 to 40 and `--in-white` 200 to 235 set the punch. For skin,
`--preset red-filter` lightens and smooths, `high-contrast-red` is harsher, `default` is
neutral. Set the subject layer to `multiply` to let the color flood the light areas
(a one-color print look). Add T-paper-grain last.

**Fails when.** The subject looks pasted on: cover part of it with a flat shape or a text
block in the ground color (a bar over the eyes, type across the chest) to lock it in.

## T-halftone-portrait

**Purpose.** A photo printed as coarse dots, as in old newsprint and pop art: modern-art,
music, and zine posters.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#0e0e0e'
pixelox layer add d.pxo --in-place --name Aura
pixelox fill gradient d.pxo --in-place --layer Aura --kind radial --from 50%,48% --to 50%,92% --stops '#ffb000ff@0,#ff5a1fcc@0.45,#ff5a1f00@1'
pixelox comp place d.pxo subject.png --in-place --as Subject --size x900 --at 50%,50% --anchor c
pixelox adjust bw d.pxo --in-place --layer Subject
pixelox adjust levels d.pxo --in-place --layer Subject --in-black 20 --in-white 215
pixelox filter halftone d.pxo --in-place --layer Subject --cell 11 --angle 45 --ink '#101010' --paper '#f4f1ea'
pixelox export d.pxo -o T-halftone-portrait.png
```

**Parameters.** `--cell` is the dot pitch: 1% of the width reads as a photo, 1.5 to 2.5% is
clearly graphic. `--shape line` gives an engraved, banknote-style line screen (use `--cell`
6 to 8 and `--angle 0` for a portrait in horizontal lines); `--mode cmyk` gives colored
rosettes. Push contrast before the screen (the `levels` step) or mid-grays turn into an even
dot carpet. Recolor with `--ink` / `--paper`, or set the layer to `multiply` so the paper
color becomes the ground color.

**Fails when.** The face disappears: the cell is too large for the face size; a face needs
at least 40 cells across.

## T-stencil-poster

**Purpose.** A portrait reduced to 3 to 5 flat tones in a limited palette, split background:
the campaign-poster and street-stencil look.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#c1121f'
pixelox shape rect d.pxo --in-place --name RightHalf --rect 50%,0,50%x100% --fill '#5b8fa8'
pixelox comp place d.pxo subject.png --in-place --as Subject --size x1180 --at 50%,100% --anchor b
pixelox adjust bw d.pxo --in-place --layer Subject
pixelox filter median d.pxo --in-place --layer Subject --radius 5
pixelox adjust gradient-map d.pxo --in-place --layer Subject --stops '#0b2545@0,#0b2545@0.28,#c1121f@0.28,#c1121f@0.5,#5b8fa8@0.5,#5b8fa8@0.72,#f3e3b9@0.72,#f3e3b9@1'
pixelox export d.pxo -o T-stencil-poster.png
```

**Parameters.** Each color appears twice in `--stops`, at the start and end of its band:
that is what makes hard edges. Bands from dark to light: navy 0 to 0.28, red to 0.5,
mid blue to 0.72, cream to 1. Move the band edges to push more or less of the face into
each tone; aim for navy in hair and deep shadow, cream only on the lit planes. `median`
radius 3 to 8 rounds the shapes (higher = simpler). Strong side light in the photo matters
more than any setting. Finish with T-paper-grain and a little T-grunge.

**Fails when.** The face is a flat blob of one tone: the photo is too evenly lit; raise
contrast first (`adjust levels --in-black 30 --in-white 210`) or pick another photo.

## T-double-exposure

**Purpose.** A second image (landscape, city, forest) shows inside the subject's silhouette:
novel covers, thoughtful film and music posters.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#e9edf0'
pixelox comp place d.pxo subject.png --in-place --as Subject --size x1150 --at 50%,100% --anchor b
pixelox adjust bw d.pxo --in-place --layer Subject
pixelox adjust levels d.pxo --in-place --layer Subject --in-white 200 --gamma 0.8
pixelox comp place d.pxo photo.png --in-place --as Scene --fit cover
pixelox layer clip d.pxo --in-place --layer Scene
pixelox layer blend d.pxo --in-place --layer Scene --mode screen
pixelox export d.pxo -o T-double-exposure.png
```

**Parameters.** `screen` shows the scene in the subject's dark areas (hair, clothes, shadow
side) and keeps the lit face; `lighten` is harder-edged; `normal` at opacity 70 fills the
whole silhouette. Darken the subject first (`--gamma` 0.6 to 0.9) to give the scene room.
Works best on a pale, empty ground with a profile or three-quarter view.

**Fails when.** The face is lost: the scene's bright area sits on the face; move the scene
layer so its brightest part falls on hair or shoulders.

## T-silhouette

**Purpose.** The subject as one flat shape (solid or gradient-filled): minimalist film
posters, teaser art, and privacy-safe portraits.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#e2912f'
pixelox layer add d.pxo --in-place --name Sun
pixelox fill gradient d.pxo --in-place --layer Sun --kind radial --from 50%,40% --to 50%,80% --stops '#ffe9a8ff@0,#ffcf6aff@0.35,#e2912f00@1'
pixelox comp place d.pxo subject.png --in-place --as Subject --size x1100 --at 50%,100% --anchor b
pixelox fx overlay d.pxo --in-place --layer Subject --stops '#141018@0,#3a2230@1' --angle 90
pixelox export d.pxo -o T-silhouette.png
```

**Parameters.** `fx overlay --color '#111111'` gives a flat fill; `--stops` with `--angle 90`
(first stop at the bottom) gives a graded one. The outline must be recognizable on its own:
profiles, hats, raised arms, tools. Put the brightest part of the ground right behind the
head.

**Fails when.** It reads as a blob: frontal poses with arms at the sides; choose a profile
or add a prop.

## T-photo-fade

**Purpose.** A photo that melts into the flat ground color, leaving a clean area for text:
the standard way to combine a full-bleed image with a text block.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#0c0c0e'
pixelox comp place d.pxo photo.png --in-place --as Photo --size 1080x --at 0,0
pixelox adjust bw d.pxo --in-place --layer Photo
pixelox layer add d.pxo --in-place --name Fade
pixelox fill gradient d.pxo --in-place --layer Fade --from 0,22% --to 0,58% --stops '#0c0c0e00@0,#0c0c0eff@1'
pixelox export d.pxo -o T-photo-fade.png
```

**Parameters.** The fade starts where the photo's subject ends and reaches full color
before the first line of text (a ramp 25 to 40% of the height). The two stops are the
ground color with alpha `00` and `ff`. Fade toward the top (`--from 0,60% --to 0,20%`) or
sideways (`--from 45%,0 --to 100%,0`) the same way.

**Fails when.** A hard photo edge still shows: the photo ends before the fade reaches full
color; make the photo taller or end the ramp earlier.

## T-glitch

**Purpose.** Digital damage: color channels pulled apart, displaced slices, scanlines.
Tech, gaming, electronic music, cyber themes.

```bash
pixelox render js scripts/pattern.js -o scan.png --size 1080x1350 --param kind=stripes --param angle=90 --param cell=6 --param weight=0.35 --param color='#000000' --param alpha=0.35
pixelox new -o d.pxo --size 1080x1350 --fill '#000000'
pixelox comp place d.pxo photo.png --in-place --as Cyan --fit cover
pixelox layer dup d.pxo --in-place --layer Cyan --name Red
pixelox adjust channel-mixer d.pxo --in-place --layer Cyan --out r --r 0 --g 0 --b 0
pixelox adjust channel-mixer d.pxo --in-place --layer Red --out g --r 0 --g 0 --b 0
pixelox adjust channel-mixer d.pxo --in-place --layer Red --out b --r 0 --g 0 --b 0
pixelox layer blend d.pxo --in-place --layer Red --mode screen
pixelox transform move d.pxo --in-place --layer Red --by -22,0
pixelox layer stamp d.pxo --in-place --name Slice
pixelox mask add d.pxo --in-place --layer Slice --from-path 'M 0 430 H 1080 V 500 H 0 Z M 0 860 H 1080 V 885 H 0 Z M 0 1010 H 1080 V 1100 H 0 Z'
pixelox transform move d.pxo --in-place --layer Slice --by 60,0
pixelox comp place d.pxo scan.png --in-place --as Scanlines --fit cover
pixelox export d.pxo -o T-glitch.png
```

**Parameters.** Channel offset 1 to 3% of the width (`--by -22,0`); more reads as 3D
glasses. Slices: 2 to 5 bands of different heights (2 to 8% of the height), moved 3 to 8%
sideways. Scanlines: `cell` 4 to 8, alpha 0.2 to 0.4. The same split works on white type on
black: duplicate the text layer in `#ff0040` and `#00e5ff`, offset each 6 px either way
under the white copy.

**Fails when.** The picture just looks blurry: the offset is too small or the image has
no hard edges; glitch needs contrast.

## T-motion-streak

**Purpose.** Speed lines pulled out of the subject or the type: sport, dance, fashion,
launch energy.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#f6d7d9'
pixelox comp place d.pxo subject.png --in-place --as Streak --size x1050 --at 50%,100% --anchor b
pixelox filter motion-blur d.pxo --in-place --layer Streak --angle 0 --distance 420
pixelox transform move d.pxo --in-place --layer Streak --by -150,0
pixelox layer opacity d.pxo --in-place --layer Streak --value 85
pixelox comp place d.pxo subject.png --in-place --as Subject --size x1050 --at 50%,100% --anchor b
pixelox export d.pxo -o T-motion-streak.png
```

**Parameters.** `--distance` 25 to 50% of the width; `--angle` 0 (sideways), 90 (falling or
rising), 20 to 30 (dynamic diagonal). Move the streak by a third of the distance against
the direction of travel so it trails behind. Tint it with
`pixelox fx overlay d.pxo --in-place --layer Streak --color '#e6007e' --blend color` for a colored
trail. `filter radial-blur --kind zoom --amount 30` gives a burst toward the viewer instead.

**Fails when.** The sharp subject looks ghosted: the streak is in front; keep the sharp
copy on top.

## T-color-grade

**Purpose.** One color mood across the whole picture (cinematic teal and orange, warm
vintage, cold night) so photos, shapes, and type feel like one image.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#101216'
pixelox comp place d.pxo photo.png --in-place --as Photo --fit cover
pixelox adjust curves d.pxo --in-place --points '0,10 64,52 128,128 192,206 255,248' --as-layer --name Contrast
pixelox adjust color-balance d.pxo --in-place --shadows=-12,0,18 --highlights 14,2,-14 --as-layer --name TealOrange
pixelox adjust vibrance d.pxo --in-place --vibrance 20 --as-layer --name Vibrance
pixelox export d.pxo -o T-color-grade.png
```

**Parameters.** Curves: lift the black point (`0,10`) for a matte film look, or pin it
(`0,0`) for hard contrast. Color balance triples are cyan-red, magenta-green, yellow-blue:
shadows `-12,0,18` (teal) with highlights `14,2,-14` (orange) is the blockbuster grade;
`8,0,-14` in both is warm vintage; `-10,0,20` in both is cold night. A value that starts
with a minus sign must be written with `=` (`--shadows=-12,0,18`). Adjustment layers act on
everything below them: put them above the photo and below the type.

**Fails when.** Skin turns orange or gray: halve the highlight shift.

## T-shape-crop

**Purpose.** A photo shown through a shape instead of a rectangle: an arch, a circle, a
slanted panel, a row of tiles, or a hand-drawn letter. Makes any stock photo look placed
on purpose.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#efe7dc'
pixelox shape path d.pxo --in-place --name Arch --d 'M 240 1180 L 240 560 A 300 300 0 0 1 840 560 L 840 1180 Z' --fill '#c9b9a6'
pixelox comp place d.pxo photo.png --in-place --as Photo --fit cover
pixelox layer clip d.pxo --in-place --layer Photo
pixelox shape path d.pxo --in-place --name Keyline --d 'M 210 1210 L 210 560 A 330 330 0 0 1 870 560 L 870 1210 Z' --fill none --stroke '#2b2520' --stroke-width 3
pixelox shape path d.pxo --in-place --name Tiles --d 'M 60 60 H 300 V 300 H 60 Z M 330 60 H 570 V 300 H 330 Z M 600 60 H 840 V 300 H 600 Z' --fill '#2b2520'
pixelox comp place d.pxo photo2.png --in-place --as TilePhoto --size 900x --at 0,0
pixelox layer clip d.pxo --in-place --layer TilePhoto
pixelox export d.pxo -o T-shape-crop.png
```

**Parameters.** Draw the window as a filled `shape path`, place the photo directly above
it, and `layer clip` the photo: it shows only inside the shape and can still be moved or
scaled (`transform move`, `transform scale`) to choose the crop. One path with several
`M ... Z` parts makes several windows from one photo (the tile row here; stripes of a
letter work the same way). An outline 30 px outside the window, or a subject that breaks
out past its edge (a second, unclipped cutout on top), adds depth.

**Fails when.** The photo does not fill the window: `--fit cover` fills the canvas, not the
shape; move or scale the photo until the window is covered.

## T-floor-reflection

**Purpose.** A faded mirror image under a product, as if it stood on glass: premium
product and tech ads.

```bash
pixelox transform trim product.png -o tight.png
pixelox new -o d.pxo --size 1080x1350 --fill '#0d0f14'
pixelox layer add d.pxo --in-place --name Floor
pixelox fill gradient d.pxo --in-place --layer Floor --from 0,70% --to 0,100% --stops '#1c2130ff@0,#0d0f14ff@1'
pixelox comp place d.pxo tight.png --in-place --as Reflection --size x620 --at 50%,70% --anchor b
pixelox transform flip d.pxo --in-place --layer Reflection --v
pixelox transform move d.pxo --in-place --layer Reflection --to 50%,70% --anchor t
pixelox layer opacity d.pxo --in-place --layer Reflection --value 40
pixelox layer add d.pxo --in-place --name Fade
pixelox fill gradient d.pxo --in-place --layer Fade --from 0,71% --to 0,90% --stops '#0d0f1400@0,#0d0f14ff@1'
pixelox comp place d.pxo tight.png --in-place --as Product --size x620 --at 50%,70% --anchor b
pixelox export d.pxo -o T-floor-reflection.png
```

**Parameters.** Trim the cutout first (`transform trim` removes the transparent border),
or the reflection will not touch the product. The mirror copy is flipped, hung from the
floor line, set to 25 to 45% opacity, and faded to the ground color within 15 to 25% of the
height. A slightly lighter floor band starting at the floor line sells the surface.
`transform flip` rasterizes a placed layer (warning PXW2003): harmless here.

**Fails when.** There is a gap between product and reflection: the cutout was not trimmed,
or the two anchors differ.

## T-water-splash

**Purpose.** Liquid thrown up around a product: drinks, freshness, sport. A drawn splash,
not a photo: stylized, but it reads at once.

```bash
pixelox render js scripts/splash.js -o back.png --size 1080x1350 --seed 3 --param color='#9fd4ff' --param cx=0.5 --param cy=0.74 --param radius=0.8 --param tongues=24 --param drops=340 --param arc=250
pixelox render js scripts/splash.js -o front.png --size 1080x1350 --seed 9 --param color='#cfeaff' --param cx=0.5 --param cy=0.88 --param radius=0.42 --param tongues=10 --param drops=120 --param arc=170
pixelox new -o d.pxo --size 1080x1350 --fill '#041c4a'
pixelox layer add d.pxo --in-place --name Glow
pixelox fill gradient d.pxo --in-place --layer Glow --kind radial --from 50%,55% --to 50%,100% --stops '#1f6fe0ff@0,#1f6fe000@1'
pixelox comp place d.pxo back.png --in-place --as SplashBack --fit cover
pixelox layer dup d.pxo --in-place --layer SplashBack --name SplashGlow
pixelox filter gaussian d.pxo --in-place --layer SplashGlow --sigma 14
pixelox layer blend d.pxo --in-place --layer SplashGlow --mode screen
pixelox comp place d.pxo product.png --in-place --as Product --size x900 --at 50%,92% --anchor b
pixelox comp place d.pxo front.png --in-place --as SplashFront --fit cover
pixelox filter gaussian d.pxo --in-place --layer SplashFront --sigma 1.2
pixelox export d.pxo -o T-water-splash.png
```

**Parameters.** Two splashes: a big one behind the product (`radius` 0.6 to 0.9, center
at the product's base) and a small one in front that covers its foot. `arc` 150 to 360 is
the fan; `dir` tilts it (`dir=-60` throws up and right). `color`: water `#9fd4ff`, cola
`#5a2a12` with `shine='#ffd9a8'`, milk `#ffffff`, juice `#ff9a1f`. The blurred duplicate in
`screen` mode makes it glow on dark grounds; on light grounds skip it and lower `alpha`.
For a photographic splash, the user must supply a splash photo: place it in `screen` mode.

**Fails when.** It looks like a flower: too regular; change `--seed`, lower `tongues`, and
let the product hide the center.

## T-ice-cubes

**Purpose.** Translucent ice blocks tumbling around a cold drink.

```bash
pixelox render js scripts/particles.js -o far.png --size 1080x1350 --seed 4 --param shape=cube --param colors='#cfeaff,#ffffff' --param count=9 --param min=5 --param max=9 --param jitter=180
pixelox render js scripts/particles.js -o near.png --size 1080x1350 --seed 11 --param shape=cube --param colors='#e6f5ff' --param count=5 --param min=9 --param max=13 --param jitter=180
pixelox new -o d.pxo --size 1080x1350 --fill '#06245c'
pixelox comp place d.pxo far.png --in-place --as IceFar --fit cover
pixelox filter gaussian d.pxo --in-place --layer IceFar --sigma 5
pixelox comp place d.pxo product.png --in-place --as Product --size x900 --at 50%,92% --anchor b
pixelox comp place d.pxo near.png --in-place --as IceNear --fit cover
pixelox fx drop-shadow d.pxo --in-place --layer IceNear --offset 0,14 --blur 30 --color '#020c26' --opacity 50
pixelox export d.pxo -o T-ice-cubes.png
```

**Parameters.** `min` / `max` are the half-size in % of the short side (9 = a cube 18% of
the width). Few and large beats many and small: 4 to 6 near cubes, 6 to 10 far ones,
blurred. Change `--seed` until no cube covers the label. Tint with the drink's ground
color (`#cfeaff` on blue, `#ffe9d6` on warm).

**Fails when.** They look like gray stickers: the ground is too light; ice needs a dark,
saturated ground to read as translucent.

## T-condensation

**Purpose.** Beads of water on a cold can, bottle, or glass.

```bash
pixelox render js scripts/particles.js -o beads.png --size 1080x1350 --seed 4 --param shape=drop --param colors='#ffffff' --param count=420 --param min=0.3 --param max=1.5 --param amin=0.6 --param amax=1
pixelox new -o d.pxo --size 1080x1350 --fill '#7a0c10'
pixelox comp place d.pxo product.png --in-place --as Product --size x1100 --at 50%,96% --anchor b
pixelox comp place d.pxo beads.png --in-place --as Beads --fit cover
pixelox layer clip d.pxo --in-place --layer Beads
pixelox export d.pxo -o T-condensation.png
```

**Parameters.** The beads layer sits directly above the product and is clipped to it, so
drops appear only on the product. `count` 250 to 600; `max` 1 to 2 (% of the short side).
Two layers (many tiny, few large with `max=3`) look more real. Set the layer to `overlay`
for subtler beads on light products.

**Fails when.** Beads cover the logo evenly: erase a zone with
`pixelox retouch erase d.pxo --in-place --layer Beads --mask @sel` after selecting the label.

## T-torn-photo

**Purpose.** A photo with ragged torn edges and a white paper rim: collage portraits,
zines, scrapbook layouts.

```bash
pixelox render js scripts/torn.js -o paper.png --size 820x1000 --seed 3 --param color='#ffffff' --param rim='#ffffff' --param edges=tblr --param rough=0.7
pixelox render js scripts/torn.js -o window.png --size 820x1000 --seed 3 --param edges=tblr --param rough=0.7 --param part=inner
pixelox new -o d.pxo --size 1080x1350 --fill '#d9d2c4'
pixelox comp place d.pxo paper.png --in-place --as Paper --at 50%,48% --anchor c
pixelox fx drop-shadow d.pxo --in-place --layer Paper --offset 6,10 --blur 18 --opacity 35
pixelox comp place d.pxo window.png --in-place --as Window --at 50%,48% --anchor c
pixelox comp place d.pxo photo.png --in-place --as Photo --fit cover
pixelox layer clip d.pxo --in-place --layer Photo
pixelox adjust bw d.pxo --in-place --layer Photo
pixelox transform rotate d.pxo --in-place --layer Paper --degrees -4
pixelox transform rotate d.pxo --in-place --layer Window --degrees -4
pixelox export d.pxo -o T-torn-photo.png
```

**Parameters.** Render the scrap twice with the **same seed and size**: once whole (the
white paper with its rim) and once with `part=inner` (the area inside the rim). Place both
at the same spot, put the photo above the inner one, and clip it. Rotate Paper and Window
by the same angle. `edges` picks the torn sides (`b` alone tears only the bottom of a
full-width photo).

**Fails when.** The photo spills over the rim: the two renders differ in seed, size, or
position.

## T-device-mockup

**Purpose.** A phone showing a screen: app promos, "swipe up" stories, digital products.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#4a6cf7'
pixelox shape rect d.pxo --in-place --name Body --rect 330,250,420x860 --radius 62 --fill '#14151a'
pixelox shape rect d.pxo --in-place --name Screen --rect 346,266,388x828 --radius 48 --fill '#f4f5ff'
pixelox comp place d.pxo photo.png --in-place --as ScreenArt --size 388x --at 346,266
pixelox layer clip d.pxo --in-place --layer ScreenArt
pixelox shape rect d.pxo --in-place --name Notch --rect 470,282,140x30 --radius 15 --fill '#14151a'
pixelox layer merge d.pxo --in-place --layers Body,Screen,ScreenArt,Notch --name Phone
pixelox transform rotate d.pxo --in-place --layer Phone --degrees 8
pixelox fx drop-shadow d.pxo --in-place --layer Phone --offset 18,26 --blur 46 --color '#1b2a80' --opacity 45
pixelox export d.pxo -o T-device-mockup.png
```

**Parameters.** Body: width 36 to 42% of the canvas, height 2.05x the width, radius 15% of
the width. Screen: inset 16 px, radius a little smaller. The screen art (a screenshot, a
photo, or text and shapes you build) is clipped to the screen. Merge the parts, then tilt
5 to 12 degrees and add a long soft shadow tinted with the ground color. Build the screen
content before merging; after the merge it is one raster layer.

**Fails when.** The screen content is unreadable: it is a full poster shrunk down; give the
screen one headline, one button, one image.

---

# Type

Fonts in these recipes are common macOS families. A family that is not installed is an
error (PX4102), not a silent fallback: check `fonts list` and pick the look-alike from
[typography.md](typography.md) first. Text and shape layers are vectors: commands that need
pixels (`filter ...`, `fill solid`) want `layer rasterize` first; `fx ...`, `transform ...`,
`layer clip`, and `layer blend` work on live text.

## T-fit-width

**Purpose.** A stacked headline where every line is scaled to exactly the same width, so the
block forms a solid rectangle: the strongest type-only poster layout.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#f1ede4'
pixelox text add d.pxo --in-place --name L1 --content "MAKE" --font Impact --size 10% --color '#111111'
pixelox transform scale d.pxo --in-place --layer L1 --size 920x --anchor tl
pixelox transform move d.pxo --in-place --layer L1 --to 80,90 --anchor tl
pixelox text add d.pxo --in-place --name L2 --content "NOISE" --font Impact --size 10% --color '#e4002b'
pixelox transform scale d.pxo --in-place --layer L2 --size 920x --anchor tl
pixelox transform move d.pxo --in-place --layer L2 --to 80,457 --anchor tl
pixelox text add d.pxo --in-place --name L3 --content "TONIGHT" --font Impact --size 10% --color '#111111'
pixelox transform scale d.pxo --in-place --layer L3 --size 920x --anchor tl
pixelox transform move d.pxo --in-place --layer L3 --to 80,817 --anchor tl
pixelox export d.pxo -o T-fit-width.png
```

**Parameters.** `transform scale --size 920x` sets the visible width of a text layer to
920 px and keeps it live text; `transform move --to X,Y --anchor tl` then puts the top-left
of the visible letters exactly at X,Y. Read each line's new height from the `bounds` in the
JSON result (`[x, y, w, h]`) and place the next line at `y + h + gap`; the gap is 2 to 3% of
the canvas height (here 24 px). Width = canvas minus two margins. Short words become tall,
long words short: order the lines so the most important word is the shortest.

**Fails when.** A long line becomes tiny: split it, or let it be a smaller second-level
line on purpose. Mixed fonts break the block; use one heavy condensed face.

## T-outline-type

**Purpose.** Letters drawn as outlines only, often repeated or paired with a filled copy:
fashion, music, sport, modern editorial.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#1437e6'
pixelox text add d.pxo --in-place --name Ghost1 --content "LOUDER" --font "Arial Black" --size 16% --color transparent --stroke '#ffffff' --stroke-width 3 --at 50%,24% --anchor c
pixelox text add d.pxo --in-place --name Ghost2 --content "LOUDER" --font "Arial Black" --size 16% --color transparent --stroke '#ffffff' --stroke-width 3 --at 50%,41% --anchor c
pixelox text add d.pxo --in-place --name Solid --content "LOUDER" --font "Arial Black" --size 16% --color '#ffffff' --at 50%,58% --anchor c
pixelox text add d.pxo --in-place --name Ghost3 --content "LOUDER" --font "Arial Black" --size 16% --color transparent --stroke '#ffffff' --stroke-width 3 --at 50%,75% --anchor c
pixelox export d.pxo -o T-outline-type.png
```

**Parameters.** `--color transparent` with `--stroke` makes hollow letters. Stroke width
1.5 to 3% of the font size (here 3 px on 216 px type); thinner looks elegant, thicker
sporty. Repeat the line 3 to 6 times with one solid copy as the focus, or put one huge
outlined word behind a photo. Heavy, wide faces only: light fonts turn into double lines.
An outlined copy shifted 6 to 10 px down-right of a solid copy gives the offset-print look.

**Fails when.** The outline is hard to read on a photo: outlines need a flat, calm ground.

## T-gradient-type

**Purpose.** A gradient inside the letters: chrome, gold, sunset, or a fade into the
background. Gives one word the feel of an object.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#0d0d10'
pixelox text add d.pxo --in-place --name Chrome --content "Code" --font "Helvetica Neue" --weight 700 --size 24% --color '#ffffff' --tracking -30 --at 50%,30% --anchor c
pixelox fx overlay d.pxo --in-place --layer Chrome --stops '#ffffff@0,#c9ccd2@0.45,#5b5f68@1' --angle 0
pixelox text add d.pxo --in-place --name Gold --content "AWARDS" --font Didot --weight 700 --size 13% --color '#ffffff' --tracking 60 --at 50%,62% --anchor c
pixelox fx overlay d.pxo --in-place --layer Gold --stops '#8a5a00@0,#f6d365@0.35,#fff6c8@0.5,#d99a1c@0.65,#7a4b00@1' --angle 90
pixelox text add d.pxo --in-place --name Fade --content "LIGHT" --font "Avenir Next Condensed" --weight 600 --size 16% --color '#ffffff' --tracking 120 --at 50%,84% --anchor c
pixelox fx overlay d.pxo --in-place --layer Fade --stops '#3a2a1a@0,#f0c48a@0.6,#fff3df@1' --angle 90
pixelox export d.pxo -o T-gradient-type.png
```

**Parameters.** `--angle 90` runs bottom to top (first stop at the bottom), `270` top to
bottom, `0` left to right. Chrome: white to mid gray, 3 stops. Gold: dark, light, highlight,
mid, dark (5 stops, highlight just above the middle). Fade-out: the first stop is close to
the background color. Add `fx drop-shadow` (offset 0,6, blur 20, opacity 50) under metallic
type.

**Fails when.** It looks like 2005 WordArt: more than one gradient word per design, or
rainbow hues. Keep the stops within one hue family.

## T-image-in-type

**Purpose.** A photo or texture shows only inside the letters. The word becomes the window.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#f4f1ea'
pixelox text add d.pxo --in-place --name Word --content "MOON" --font "Helvetica Neue" --weight 900 --size 24% --color '#111111' --at 50%,26% --anchor t
pixelox transform scale d.pxo --in-place --layer Word --size 940x --anchor t
pixelox comp place d.pxo photo.png --in-place --as Fill --fit cover
pixelox layer clip d.pxo --in-place --layer Fill
pixelox export d.pxo -o T-image-in-type.png
```

**Parameters.** `layer clip` shows the layer only where the layer directly below it has
pixels: the image goes immediately above the text. Use the heaviest face available and
tight tracking so there is enough letter area; cap height at least 20% of the canvas.
Textures (moon surface, flowers, crowd, marble, a T-fire-glow render) work better than
photos with one small subject.

**Fails when.** The image is unrecognizable and the word hard to read: choose an image with
strong contrast against the page, or add a thin `fx stroke` to the word.

## T-distress-type

**Purpose.** Worn, cracked, badly printed ink on big letters: grunge, vintage, street.

```bash
pixelox render js scripts/grunge.js -o wear.png --size 1080x1350 --seed 14 --param color='#e4d9c3' --param specks=9000 --param blotches=160 --param scratches=70 --param clump=0.4
pixelox new -o d.pxo --size 1080x1350 --fill '#e4d9c3'
pixelox text add d.pxo --in-place --name Word --content "ABLE" --font Impact --size 30% --color '#151515' --at 50%,50% --anchor c
pixelox transform scale d.pxo --in-place --layer Word --size 940x --anchor c
pixelox comp place d.pxo wear.png --in-place --as Wear --fit cover
pixelox layer clip d.pxo --in-place --layer Wear
pixelox export d.pxo -o T-distress-type.png
```

**Parameters.** The wear is a grunge layer in the **background color**, clipped to the
word, so it reads as missing ink. `specks` 6000 to 12000, `blotches` 100 to 250 for heavy
wear; `clump` 0.3 to 0.6. On a photo or texture ground, use the ground's average color.
Combine with T-grunge on the whole design so type and paper wear together.

**Fails when.** It looks like the word has measles: marks are too even; raise `clump`, and
add scratches.

## T-block-shadow

**Purpose.** A hard, flat offset shadow with an outline: retro signage, comics, bold sale
graphics. One command pair, stays live.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#ffd23c'
pixelox text add d.pxo --in-place --name Title --content "MEGA" --font "Arial Black" --size 22% --color '#ffffff' --at 50%,40% --anchor c
pixelox fx stroke d.pxo --in-place --layer Title --width 10 --color '#1b1b1b' --position outside
pixelox fx drop-shadow d.pxo --in-place --layer Title --offset 28,28 --blur 0 --color '#e4002b' --blend normal --opacity 100
pixelox export d.pxo -o T-block-shadow.png
```

**Parameters.** `--blur 0 --blend normal --opacity 100` makes the shadow a flat copy.
Offset 10 to 18% of the cap height, down-right (or down-left for a lit-from-right feel);
it must be at least twice the stroke width, because the shadow is cast by the letters,
not by their stroke. Shadow color: black, or a strong second palette color.

**Fails when.** Letters fill in: the stroke is too thick for the font size; keep it under
5% of the cap height.

## T-long-shadow

**Purpose.** A flat shadow dragged diagonally from the letters until it leaves the canvas
(flat-design long shadow), or at short length a solid 3D extrusion (block lettering).

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#19b5a5'
pixelox text add d.pxo --in-place --name Title --content "RETRO" --font "Arial Black" --size 16% --color '#fff4d6' --at 50%,30% --anchor c
pixelox export d.pxo -o tip.png --layer Title
pixelox layer add d.pxo --in-place --name Shadow
pixelox paint stroke d.pxo --in-place --layer Shadow --tip tip.png --size 1350 --spacing 1 --color '#0d6b62' --points '540,675 1640,1775; 541.6,676.6 1640,1775; 543.2,678.2 1640,1775; 544.8,679.8 1640,1775; 546.4,681.4 1640,1775; 548,683 1640,1775'
pixelox layer reorder d.pxo --in-place --layer Shadow --below Title
pixelox text add d.pxo --in-place --name Block --content "BLOCK" --font "Arial Black" --size 16% --color '#ffd23c' --at 50%,72% --anchor c
pixelox export d.pxo -o tip2.png --layer Block
pixelox layer add d.pxo --in-place --name Side
pixelox paint stroke d.pxo --in-place --layer Side --tip tip2.png --size 1350 --spacing 1 --color '#b3261e' --points '540,675 568,703; 541.6,676.6 568,703; 543.2,678.2 568,703; 544.8,679.8 568,703; 546.4,681.4 568,703; 548,683 568,703'
pixelox layer reorder d.pxo --in-place --layer Side --below Block
pixelox fx stroke d.pxo --in-place --layer Block --width 4 --color '#3b0d0a'
pixelox export d.pxo -o T-long-shadow.png
```

**Parameters.** How it works: the word itself becomes the brush tip. `export --layer Title`
writes the word alone on a transparent canvas-sized image; `paint stroke --tip` then drags
that image from the canvas center (`540,675` = the tip in its original place) along the
shadow direction, stamping copies. `--size` is the tip's longer side: the canvas height
for a portrait canvas, the width for a landscape one. The six sub-strokes, each started
1.6 px further along, close the gaps between stamps so the edge is smooth. End point =
center plus the shadow length on each axis: `+1100,+1100` runs off the canvas (long
shadow); `+28,+28` is a block extrusion (6 to 12% of the cap height). For down-left use
negative x steps. Shadow color: the ground darkened 30 to 40%; extrusion color: a darker
shade of the letters, plus `fx stroke` on the letters.

**Fails when.** The shadow starts beside the letters instead of under them: the first
point is not the exact canvas center, or `--size` is not the canvas's longer side. Stair
steps on the diagonal edge: a sub-stroke is missing.

## T-neon-glow

**Purpose.** Type or line art that glows like a neon tube: nightlife, gaming, synthwave,
"open late" energy.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#0b0614'
pixelox text add d.pxo --in-place --name Halo --content "OPEN" --font "Avenir Next" --weight 600 --size 17% --color '#ff2bd6' --tracking 80 --at 50%,42% --anchor c
pixelox layer rasterize d.pxo --in-place --layer Halo
pixelox filter gaussian d.pxo --in-place --layer Halo --sigma 38
pixelox layer blend d.pxo --in-place --layer Halo --mode screen
pixelox text add d.pxo --in-place --name Tube --content "OPEN" --font "Avenir Next" --weight 600 --size 17% --color '#fff2fc' --tracking 80 --at 50%,42% --anchor c
pixelox fx outer-glow d.pxo --in-place --layer Tube --blur 26 --spread 12 --color '#ff2bd6' --opacity 95
pixelox shape rect d.pxo --in-place --name Frame --rect 14%,28%,72%x28% --radius 40 --fill none --stroke '#38e8ff' --stroke-width 6
pixelox fx outer-glow d.pxo --in-place --layer Frame --blur 30 --spread 10 --color '#00c8ff' --opacity 90
pixelox export d.pxo -o T-neon-glow.png
```

**Parameters.** Three parts: a near-white tube (the text color is the glow color mixed 85%
toward white), a tight glow (`--blur` 2 to 3% of the width), and a wide blurred halo copy
underneath in `screen` mode (`--sigma` 3 to 5% of the width). Ground must be very dark
(lightness under 10%) and slightly tinted toward the glow. Medium weight, rounded or
geometric faces look like bent tubes; heavy faces look like light boxes.

**Fails when.** It looks flat: the letters are fully saturated; the tube itself must be
almost white, only the glow carries the color.

## T-fire-type

**Purpose.** Flames rising from a word (or any layer's outline).

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#070504'
pixelox text add d.pxo --in-place --name Title --content "FIRE" --font Impact --size 26% --color '#1c0b02' --at 50%,60% --anchor c
pixelox fx fire d.pxo --in-place --layer Title --height 75% --intensity 120 --turbulence 60 --variant 4
pixelox fx inner-glow d.pxo --in-place --layer Title --blur 30 --color '#ff7a1a' --opacity 70
pixelox export d.pxo -o T-fire-type.png
```

**Parameters.** `--height` 40 to 120% of the layer height; `--intensity` 80 to 160;
`--variant N` picks the flame pattern; `--front` 15 to 40 lets flames lick over the letters.
Letters should be dark (charred) with a warm inner glow, on a near-black ground. The same
style on a cutout (`--layer Subject`) sets a subject alight; `fx smoke` is its sibling.
`--colors` recolors the flames (blue: `'#000000@0,#001a66@0.3,#0a5bff@0.6,#7fd4ff@0.85,#ffffff@1'`).

**Fails when.** Flames are invisible: the ground is too light; fire needs a dark ground.

## T-vertical-type

**Purpose.** A word or line turned 90 degrees and run along an edge: Swiss and editorial
layouts, spine-like labels, a second reading direction.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#0e0e0e'
pixelox text add d.pxo --in-place --name Big --content "MODERN" --font "Helvetica Neue" --weight 700 --size 15% --color '#ffffff' --at 50%,50% --anchor c
pixelox transform rotate d.pxo --in-place --layer Big --degrees -90
pixelox transform move d.pxo --in-place --layer Big --to 7%,93% --anchor bl
pixelox text add d.pxo --in-place --name Small --content "10 . 10 . 2030   19:00" --font "Helvetica Neue" --weight 400 --size 3.2% --tracking 300 --color '#ff5a1f' --at 50%,50% --anchor c
pixelox transform rotate d.pxo --in-place --layer Small --degrees -90
pixelox transform move d.pxo --in-place --layer Small --to 27%,93% --anchor bl
pixelox export d.pxo -o T-vertical-type.png
```

**Parameters.** `--degrees -90` reads bottom to top (the usual choice on a left edge);
`90` reads top to bottom (right edge). Rotate first, then `transform move --to X,Y --anchor bl`
positions the rotated block by its visible corner. Keep vertical lines short (1 to 3 words)
and big; pair one giant vertical word with small tracked vertical details.

**Fails when.** Body copy is vertical: never rotate anything the reader must read in full
sentences.

## T-arc-type

**Purpose.** Text on a circle or arc: badges, seals, stamps, "limited edition" roundels.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#f3ead7'
pixelox shape ellipse d.pxo --in-place --name Disc --rect 290,425,500x500 --fill '#c8102e'
pixelox shape ellipse d.pxo --in-place --name RingLine --rect 318,453,444x444 --fill none --stroke '#f3ead7' --stroke-width 3
pixelox text add d.pxo --in-place --name Arc --content "LIMITED EDITION  *  SMALL BATCH  *  SINCE 1998  *" --font "Avenir Next" --weight 700 --size 34px --tracking 110 --color '#f3ead7'
pixelox text on-path d.pxo --in-place --layer Arc --d 'M 357 675 A 183 183 0 1 1 723 675 A 183 183 0 1 1 357 675' --side outside
pixelox text add d.pxo --in-place --name Center --content "No.7" --font Didot --weight 700 --size 11% --color '#f3ead7' --at 540,675 --anchor c
pixelox export d.pxo -o T-arc-type.png
```

**Parameters.** The path is a circle written as two arcs starting at the left point and
going clockwise over the top, so the text starts at 9 o'clock; `--offset 25%` starts it at
12 o'clock. Path radius = where the baseline sits. Length of text that fits = 2 x 3.14 x
radius: adjust `--tracking` (60 to 200) or the size until the text closes the circle
without overlap (read `bounds` and look at the render). `--side inside` puts the text
inside the circle, reading the other way.

**Fails when.** Letters collide at the bottom: the text is longer than the circumference;
shorten it or lower the tracking.

## T-script-overlap

**Purpose.** A flowing script word laid over (or tucked under) a block word: the
"Gospel CONCERT / Night Worship / Coffee & Jazz" pairing. Adds warmth and a second voice.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#101a3a'
pixelox text add d.pxo --in-place --name Block --content "CONCERT" --font "Avenir Next Condensed" --weight 700 --size 15% --color '#ffffff' --tracking 40 --at 50%,52% --anchor c
pixelox text add d.pxo --in-place --name Script --content "Gospel" --font "Snell Roundhand" --weight 900 --size 19% --color '#f5a623' --at 44%,41% --anchor c
pixelox transform rotate d.pxo --in-place --layer Script --degrees -6
pixelox fx drop-shadow d.pxo --in-place --layer Script --offset 0,6 --blur 14 --opacity 45
pixelox export d.pxo -o T-script-overlap.png
```

**Parameters.** The script is 1.1 to 1.5x the block word's font size, in the accent color,
rotated -4 to -8 degrees, and overlaps the block word's top or bottom edge by 15 to 30% of
its own height. A small shadow separates the two. One script word only, 4 to 10 letters,
never all caps. Script faces: `Snell Roundhand` (formal), `Brush Script MT` and
`SignPainter` (brush), or a registered Google script (Great Vibes, Pacifico, Yellowtail).

**Fails when.** The script is unreadable: it sits on a busy photo or is smaller than 8% of
the canvas height.

## T-label-box

**Purpose.** A solid box or slanted tag behind a short line of text: name plates, "NEW",
dates, a highlighted word in a sentence.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#d7f000'
pixelox shape rect d.pxo --in-place --name Box --rect 120,480,470x120 --fill '#111111'
pixelox text add d.pxo --in-place --name BoxText --content "BE YOURSELF" --font "Helvetica Neue" --weight 900 --size 5.6% --color '#d7f000' --at 355,540 --anchor c
pixelox shape path d.pxo --in-place --name Tag --d 'M 150 700 L 720 700 L 690 800 L 120 800 Z' --fill '#ffffff'
pixelox text add d.pxo --in-place --name TagText --content "THIS FRIDAY" --font "Helvetica Neue" --weight 900 --style italic --size 5.2% --color '#111111' --at 420,750 --anchor c
pixelox shape rect d.pxo --in-place --name Pill --rect 120,900,380x84 --radius 42 --fill none --stroke '#111111' --stroke-width 4
pixelox text add d.pxo --in-place --name PillText --content "FREE ENTRY" --font "Helvetica Neue" --weight 700 --size 3% --tracking 120 --color '#111111' --at 310,942 --anchor c
pixelox export d.pxo -o T-label-box.png
```

**Parameters.** Add the text first when you do not know its width: read `bounds` from the
JSON, then draw the box as `bounds` grown by a padding of 0.35 to 0.5 x the cap height on
every side, and move the box below the text (`layer reorder --below`). Box and text swap
the two main colors. Slanted tag: shift the top edge 25 to 40 px to the right of the bottom
edge. Text is centered with `--anchor c` on the box center.

**Fails when.** Padding is uneven: center on the box center, not the canvas, and check the
render; fonts with tall ascenders need the text nudged down 2 to 4% of the font size.

## T-stretch-type

**Purpose.** Change a font's proportions when the right face is not installed: taller and
narrower (compressed), wider (extended), heavier (fattened), or slanted. A stand-in for
ultra-condensed, extended, and extra-black display faces.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#f1ede4'
pixelox text add d.pxo --in-place --name Tall --content "COMPRESSED" --font Impact --size 10% --color '#111111'
pixelox transform scale d.pxo --in-place --layer Tall --size 900x330
pixelox transform move d.pxo --in-place --layer Tall --to 50%,7% --anchor t
pixelox text add d.pxo --in-place --name Wide --content "WIDE" --font "Arial Black" --size 10% --color '#e4002b'
pixelox transform scale d.pxo --in-place --layer Wide --size 900x170
pixelox transform skew d.pxo --in-place --layer Wide --x -12
pixelox transform move d.pxo --in-place --layer Wide --to 50%,37% --anchor t
pixelox text add d.pxo --in-place --name Fat --content "Heavier" --font "Avenir Next" --weight 900 --size 16% --color '#111111' --at 50%,66% --anchor c
pixelox fx stroke d.pxo --in-place --layer Fat --width 9 --color '#111111' --position outside
pixelox export d.pxo -o T-stretch-type.png
```

**Parameters.** `transform scale --size WxH` with both numbers stretches the text and
turns it into pixels (warning PXW2003): do it last, at final size. Keep the stretch
within 0.7x to 1.5x of the natural shape; beyond that the strokes look wrong. Slant with
`transform skew --x -8` to `-14` (negative leans right). Fatten with an outside `fx stroke`
in the text color, 3 to 6% of the cap height: it also rounds the corners, which suits
chunky retro and food type.

**Fails when.** Round letters turn into ovals with thin sides: the stretch is too strong;
choose a narrower or wider font first (typography.md) and stretch only the rest of the way.

## T-sliced-type

**Purpose.** A word cut along a diagonal with the halves slipped apart: edgy tech, music,
editorial, "breaking" and "glitch" ideas without the noise.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#f1ede4'
pixelox text add d.pxo --in-place --name Top --content "SLICE" --font Impact --size 24% --color '#111111' --at 50%,42% --anchor c
pixelox transform scale d.pxo --in-place --layer Top --size 900x --anchor c
pixelox layer dup d.pxo --in-place --layer Top --name Bottom
pixelox mask add d.pxo --in-place --layer Top --from-path 'M 0 0 H 1080 V 500 L 0 640 Z'
pixelox mask add d.pxo --in-place --layer Bottom --from-path 'M 0 640 L 1080 500 V 1350 H 0 Z'
pixelox transform move d.pxo --in-place --layer Bottom --by 46,16
pixelox fill solid d.pxo --in-place --layer Background --color '#e4002b' --mask @mask:Bottom
pixelox fill solid d.pxo --in-place --layer Background --color '#f1ede4' --mask @mask:Top
pixelox export d.pxo -o T-sliced-type.png
```

**Parameters.** Two copies of the word carry opposite halves of one cut line as vector
masks (`mask add --from-path`); the lower copy is then moved 3 to 6% of the width along the
cut. The cut should cross the letters at 5 to 15 degrees and pass through their middle
third. The two `fill solid` lines are optional: they color the ground under the lower half
for a two-field look. For several cuts, repeat with three copies and two cut lines.

**Fails when.** The word becomes unreadable: the offset is larger than a letter's stroke
width times 3, or the cut runs along the baseline.

## T-ghost-type

**Purpose.** A giant, faint word as atmosphere: behind the subject, bleeding off the
edges, or outlined across the subject. It sets the theme without competing with the
headline.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#0b2a8f'
pixelox text add d.pxo --in-place --name Ghost --content "FRESH" --font "Arial Black" --size 20% --color '#ffffff' --at 50%,22% --anchor c
pixelox transform scale d.pxo --in-place --layer Ghost --size 1240x --anchor c
pixelox layer opacity d.pxo --in-place --layer Ghost --value 12
pixelox text add d.pxo --in-place --name Soft --content "FRESH" --font "Arial Black" --size 20% --color '#5fa8ff' --at 50%,50% --anchor c
pixelox transform scale d.pxo --in-place --layer Soft --size 1240x --anchor c
pixelox layer rasterize d.pxo --in-place --layer Soft
pixelox filter gaussian d.pxo --in-place --layer Soft --sigma 12
pixelox layer opacity d.pxo --in-place --layer Soft --value 55
pixelox comp place d.pxo product.png --in-place --as Product --size x1000 --at 50%,96% --anchor b
pixelox text add d.pxo --in-place --name Outline --content "FRESH" --font "Arial Black" --size 20% --color transparent --stroke '#ffffff' --stroke-width 2 --at 50%,80% --anchor c
pixelox transform scale d.pxo --in-place --layer Outline --size 1240x --anchor c
pixelox export d.pxo -o T-ghost-type.png
```

**Parameters.** Three variants, use one: **faint** (the ground's own hue, 8 to 15%
opacity), **defocused** (a lighter tint, blurred 1 to 1.5% of the width, as if out of
focus behind the subject), **outlined over the subject** (hollow letters, 1 to 2 px, on
top). The word is wider than the canvas (110 to 130%) so its ends are cut off. It never
carries information the reader needs.

**Fails when.** It competes with the real headline: lower its opacity until you notice it
second.

## T-floor-type

**Purpose.** Type lying flat on the ground, receding into the picture: film titles, sport,
street and stage scenes.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#b3121a'
pixelox layer add d.pxo --in-place --name Far
pixelox fill gradient d.pxo --in-place --layer Far --from 0,40% --to 0,100% --stops '#5a070bff@0,#5a070b00@1'
pixelox text add d.pxo --in-place --name Title --content "ON THE LINE" --font Impact --size 12% --color '#111111' --at 50%,50% --anchor c
pixelox transform perspective d.pxo --in-place --layer Title --quad '310,960 770,960 1040,1250 40,1250'
pixelox comp place d.pxo subject.png --in-place --as Subject --size x520 --at 50%,72.5% --anchor b
pixelox fx overlay d.pxo --in-place --layer Subject --color '#111111'
pixelox export d.pxo -o T-floor-type.png
```

**Parameters.** `--quad` lists where the text's four corners go: top-left, top-right,
bottom-right, bottom-left. For a floor, the top edge is short and far (25 to 40% of the
bottom edge's width) and the bottom edge near and wide; both edges centered on the
vanishing line. The text becomes pixels (PXW2003), so set its words first. A darker ground
toward the horizon and a figure standing at the far end complete the space.

**Fails when.** It looks like a trapezoid sticker: the top edge is too wide; narrow it, and
keep the block's height under a third of its bottom width, or the letters stretch into bars.

## T-counterchange

**Purpose.** A word that changes color where it crosses from one field into another:
light on dark, dark on light. A classic of Swiss and book-cover design.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#111111'
pixelox shape rect d.pxo --in-place --name Field --rect 50%,0,50%x100% --fill '#f2c230'
pixelox text add d.pxo --in-place --name Light --content "CONTRAST" --font "Helvetica Neue" --weight 700 --size 13% --color '#f2c230' --at 50%,45% --anchor c
pixelox text add d.pxo --in-place --name Dark --content "CONTRAST" --font "Helvetica Neue" --weight 700 --size 13% --color '#111111' --at 50%,45% --anchor c
pixelox mask add d.pxo --in-place --layer Dark --from-path 'M 540 0 H 1080 V 1350 H 540 Z'
pixelox export d.pxo -o T-counterchange.png
```

**Parameters.** Two identical text layers in the two field colors; the upper one is masked
with the same outline as the field it sits on (a rectangle here; use the circle's or
diagonal's path for other fields). The field edge should cut through letters, not between
them.

**Fails when.** A hairline of the wrong color shows at the boundary: the mask path and the
field shape differ by a pixel; use the same numbers.

## T-warp-type

**Purpose.** A headline that grows, rises, or fans: hand-lettered show posters, retro
sport, anything that should feel shouted.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#f6e7c8'
pixelox text add d.pxo --in-place --name Grow --content "LOUDER" --font Impact --size 14% --color '#c62a1a' --at 50%,26% --anchor c
pixelox transform perspective d.pxo --in-place --layer Grow --quad '120,330 960,170 960,520 120,430'
pixelox text add d.pxo --in-place --name Rise --content "EVERY NIGHT" --font Impact --size 9% --color '#1d1a16' --at 50%,58% --anchor c
pixelox transform skew d.pxo --in-place --layer Rise --y -8
pixelox text add d.pxo --in-place --name Fall --content "ALL SUMMER" --font Impact --size 9% --color '#1d1a16' --at 50%,76% --anchor c
pixelox transform skew d.pxo --in-place --layer Fall --y 8
pixelox export d.pxo -o T-warp-type.png
```

**Parameters.** Grow: `transform perspective` with a short left edge and a tall right edge
(ratio 1 : 2 to 1 : 3.5). Rise or fall: `transform skew --y -8` (negative rises to the
right) keeps the letters upright while the baseline climbs. Pair a rising line with a
falling one for a zigzag stack. All of these rasterize the text. A true arc needs
`text on-path` (T-arc-type).

**Fails when.** The small end is unreadable: keep its cap height above 4% of the canvas.

## T-stencil-type

**Purpose.** Stencil lettering (crates, military, street) when no stencil font is
installed: thin gaps cut through heavy letters.

```bash
pixelox render js scripts/pattern.js -o cuts.png --size 1080x1350 --param kind=stripes --param cell=118 --param weight=0.09 --param color='#d8cfb4'
pixelox new -o d.pxo --size 1080x1350 --fill '#d8cfb4'
pixelox text add d.pxo --in-place --name Word --content "CARGO" --font Impact --size 26% --color '#23301f' --at 50%,45% --anchor c
pixelox transform scale d.pxo --in-place --layer Word --size 920x --anchor c
pixelox comp place d.pxo cuts.png --in-place --as Cuts --fit cover
pixelox layer clip d.pxo --in-place --layer Cuts
pixelox export d.pxo -o T-stencil-type.png
```

**Parameters.** Vertical stripes in the ground color, clipped to the word, become the
stencil bridges. `cell` = the gap spacing: 55 to 70% of a letter's width so every letter
gets one or two cuts; `weight` 0.07 to 0.12 is the gap width. Shift the pattern sideways
with `transform move --layer Cuts --by 20,0` until no cut lands between letters. Add
T-distress-type or T-grunge for sprayed paint.

**Fails when.** Cuts slice letters in half visually: gaps too wide; keep them under 10% of
the letter width.

## T-texture-fill

**Purpose.** Type or shapes filled with a generated material (thick paint, marble, crumpled
paper) instead of a flat color.

```bash
pixelox render js scripts/clouds.js -o paint.png --size 1080x1350 --seed 12 --param lo='#b3541e' --param mid='#e8a23a' --param hi='#f6e3b4' --param scale=5 --param octaves=5 --param warp=1.4 --param contrast=2.2
pixelox new -o d.pxo --size 1080x1350 --fill '#16302b'
pixelox text add d.pxo --in-place --name Word --content "ART" --font "Arial Black" --size 30% --color '#e8a23a' --at 50%,40% --anchor c
pixelox comp place d.pxo paint.png --in-place --as Paint --fit cover
pixelox filter oil-paint d.pxo --in-place --layer Paint --impasto 80
pixelox layer clip d.pxo --in-place --layer Paint
pixelox shape ellipse d.pxo --in-place --name Disc --rect 340,860,400x400 --fill '#e8a23a'
pixelox comp place d.pxo paint.png --in-place --as Paint2 --fit cover
pixelox filter oil-paint d.pxo --in-place --layer Paint2 --impasto 80
pixelox layer clip d.pxo --in-place --layer Paint2
pixelox export d.pxo -o T-texture-fill.png
```

**Parameters.** `clouds.js` with three colors and `contrast` 2 to 3 gives hard-edged
marbling; `filter oil-paint --impasto 60` to `90` turns it into brushed paint. For
crumpled paper use two near-whites with `scale=7`, `octaves=6`, `warp=0.3`, and
`filter emboss` at low amount. Clip the texture to the word or shape (each clipped copy
attaches to the layer right below it).

**Fails when.** The word loses its edge: the texture's darkest color matches the ground;
keep all three texture colors clearly lighter or darker than the ground.

## T-crack

**Purpose.** A jagged split through a word, a face, or a silhouette: thrillers, breakups,
"broken" concepts, lightning.

```bash
pixelox render js scripts/crack.js -o crack.png --size 1080x1350 --seed 5 --param color='#f1ede4' --param from=0.56,0.12 --param to=0.47,0.9 --param width=0.03 --param steps=16
pixelox new -o d.pxo --size 1080x1350 --fill '#f1ede4'
pixelox text add d.pxo --in-place --name Word --content "SPLIT" --font Impact --size 30% --color '#161616' --at 50%,50% --anchor c
pixelox transform scale d.pxo --in-place --layer Word --size 940x --anchor c
pixelox comp place d.pxo crack.png --in-place --as Crack --fit cover
pixelox export d.pxo -o T-crack.png
```

**Parameters.** The crack is drawn in the ground color on top of the object, so it reads
as a gap. `from` / `to` are canvas fractions; `width` 0.015 to 0.04; `jag` 0.3 to 0.8;
`branches` 0 to 5. Render it at canvas size. For lightning, use a pale color on a dark sky
and add `fx outer-glow` to the layer.

**Fails when.** It looks like a scratch: too thin for the object; the widest point should
be 3 to 6% of the object's width.

## T-object-as-letter

**Purpose.** The product or an object stands in for one letter of the headline word. The
word and the picture become one thing.

```bash
pixelox transform trim product.png -o tight.png
pixelox new -o d.pxo --size 1080x1350 --fill '#bfe3f2'
pixelox text add d.pxo --in-place --name Left --content "CH" --font "Avenir Next" --weight 500 --size 22% --color '#ffffff' --at 43%,45% --anchor r
pixelox text add d.pxo --in-place --name Right --content "LL" --font "Avenir Next" --weight 500 --size 22% --color '#ffffff' --at 57%,45% --anchor l
pixelox comp place d.pxo tight.png --in-place --as Letter --size x290 --at 50%,45% --anchor c
pixelox fx drop-shadow d.pxo --in-place --layer Letter --offset 0,16 --blur 30 --opacity 30
pixelox export d.pxo -o T-object-as-letter.png
```

**Parameters.** Choose a letter the object resembles (tall objects for I, L, T, or 1; round
ones for O, Q, or 0) in the middle of a short word. Split the word into the parts before
and after that letter, anchor them to the left and right of a gap, and scale the object to
1.2 to 1.35x the cap height (it should poke out above and below a little). The gap is the
object's width plus normal letter spacing.

**Fails when.** The word no longer reads: the object replaces the first letter, or its
shape is too far from the letter's.

---

# Graphic furniture

## T-pill-button

**Purpose.** A call-to-action that looks tappable: outlined or filled pill with a short
command and an arrow.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#0a0a1f'
pixelox shape rect d.pxo --in-place --name Outline --rect 330,560,420x74 --radius 37 --fill none --stroke '#ffffff' --stroke-width 2
pixelox text add d.pxo --in-place --name OutlineText --content "JOIN THE EXPO" --font "Avenir Next" --weight 500 --size 2.4% --tracking 80 --color '#ffffff' --at 515,597 --anchor c
pixelox shape arrow d.pxo --in-place --name Arrow --from 668,597 --to 712,597 --stroke '#ffffff' --stroke-width 2 --head-size 12
pixelox shape rect d.pxo --in-place --name Solid --rect 370,700,340x84 --radius 42 --fill '#ffd400'
pixelox text add d.pxo --in-place --name SolidText --content "Shop Now" --font "Avenir Next" --weight 700 --size 3.4% --color '#0a0a1f' --at 540,742 --anchor c
pixelox export d.pxo -o T-pill-button.png
```

**Parameters.** Height 5 to 6.5% of the canvas height; `--radius` = half the height; width
2.5 to 5x the height; text 40 to 45% of the pill height. One button per design, placed
after the headline and sub-line, with clear space around it of at least its own height.
Filled pills use the accent color; outlined pills use the text color.

**Fails when.** It looks like a label, not a button: text too big for the pill, or there
are several of them.

## T-price-burst

**Purpose.** A jagged starburst sticker carrying a price or a short offer.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#e8c9a8'
pixelox shape star d.pxo --in-place --name Burst --center 62%,55% --points 22 --outer 190 --inner 150 --fill '#d83a0a'
pixelox transform rotate d.pxo --in-place --layer Burst --degrees 8
pixelox fx drop-shadow d.pxo --in-place --layer Burst --offset 0,10 --blur 24 --opacity 30
pixelox text add d.pxo --in-place --name Price --content "\$3.99" --font "Arial Black" --size 7% --color '#ffffff' --at 62%,55% --anchor c
pixelox transform rotate d.pxo --in-place --layer Price --degrees -6
pixelox export d.pxo -o T-price-burst.png
```

**Parameters.** `--points` 16 to 28; `--inner` 78 to 85% of `--outer` for short teeth
(sticker), 55% for a spiky comic burst. Diameter 30 to 40% of the canvas width. It should
overlap the product's edge by a quarter of its width and be tilted 5 to 12 degrees. The
price is the largest text after the headline. Write a dollar sign as `\$` inside double
quotes.

**Fails when.** It floats alone in a corner: let it overlap the hero.

## T-info-bar

**Purpose.** A horizontal strip that carries date, time, and place in two or three columns:
the bottom band of most event flyers.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#5a0c0a'
pixelox shape rect d.pxo --in-place --name Bar --rect 8%,81%,84%x11% --fill '#1c0505'
pixelox fx overlay d.pxo --in-place --layer Bar --stops '#000000@0,#7a0f0c@1' --angle 0
pixelox shape rect d.pxo --in-place --name BarEdge --rect 8%,81%,84%x11% --fill none --stroke '#ffffff' --stroke-width 1 --dash 2,4 --opacity 60
pixelox text add d.pxo --in-place --name Date --content $'3RD - 23RD\nFEB. 2027' --font "Helvetica Neue" --weight 900 --size 3.4% --leading 96% --color '#ffffff' --at 14%,86.5% --anchor l
pixelox shape line d.pxo --in-place --name Sep1 --from 41%,83% --to 41%,90% --stroke '#ffffff' --stroke-width 2 --opacity 50
pixelox text add d.pxo --in-place --name Time --content "6PM" --font "Helvetica Neue" --weight 900 --size 4.4% --color '#ffffff' --at 50%,85.5% --anchor c
pixelox text add d.pxo --in-place --name TimeNote --content "EACH NIGHT" --font "Helvetica Neue" --weight 700 --size 1.4% --tracking 60 --color '#ffffff' --at 50%,89% --anchor c
pixelox shape line d.pxo --in-place --name Sep2 --from 59%,83% --to 59%,90% --stroke '#ffffff' --stroke-width 2 --opacity 50
pixelox text add d.pxo --in-place --name Place --content $'123 ANYWHERE ST.,\nANY CITY 12345' --font "Helvetica Neue" --weight 900 --size 2.5% --leading 100% --color '#ffffff' --at 63%,86.5% --anchor l
pixelox export d.pxo -o T-info-bar.png
```

**Parameters.** Bar: 82 to 92% of the width, 9 to 13% of the height, sitting 6 to 10% above
the bottom edge; darker than the ground, optionally with a left-to-right gradient. Three
columns in the ratio 1.2 : 0.8 : 1.5 (date, time, place), split by thin vertical rules at
50% opacity. Text sizes 2.5 to 4.5% of the canvas height: the time may be the largest.
`$'line one\nline two'` makes a two-line layer (a plain "\n" inside double quotes does
not). A subject cutout may overlap the bar's top edge. Icons: place small SVG or PNG icons
(`comp place`) or build them from 2 to 3 shapes; skip them rather than fake them with odd
glyphs.

**Fails when.** Lines touch the bar's edge: keep padding of at least 25% of the bar height
on the left and right.

## T-frame-border

**Purpose.** A border set in from the edge (magazine covers, certificates, framed prints),
or a thin keyline that holds a loose layout together.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#e4002b'
pixelox shape rect d.pxo --in-place --name Inner --rect 2.6%,2.1%,94.8%x95.8% --fill '#e9e4dc'
pixelox comp place d.pxo photo.png --in-place --as Photo --size 1024x1294 --at 28,28
pixelox shape rect d.pxo --in-place --name Keyline --rect 6%,5%,88%x90% --fill none --stroke '#ffffff' --stroke-width 2 --opacity 80
pixelox export d.pxo -o T-frame-border.png
```

**Parameters.** Thick color border: 2.5 to 4% of the canvas width on every side, in the
accent color; the photo is cropped inside it. Thin keyline: 1 to 3 px, set in 5 to 7% from
the edge, at 60 to 90% opacity. Headline type may break out over the frame; a subject's
head may overlap the masthead inside it.

**Fails when.** The border is thicker at the top than the sides: compute the rect from the
same pixel inset on all four sides (here 28 px).

## T-rules

**Purpose.** Thin lines, dotted lines, short accent bars, and corner ticks that organize
small text: the quiet scaffolding of Swiss and corporate layouts.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#101010'
pixelox shape line d.pxo --in-place --name Accent --from 7%,56% --to 12%,56% --stroke '#ffffff' --stroke-width 3
pixelox shape line d.pxo --in-place --name Hairline --from 7%,70% --to 93%,70% --stroke '#ffffff' --stroke-width 1 --opacity 45
pixelox shape line d.pxo --in-place --name Dotted --from 7%,78% --to 93%,78% --stroke '#ffffff' --stroke-width 5 --dash 0,22 --cap round --opacity 80
pixelox shape rect d.pxo --in-place --name Marker --rect 7%,84%,3.4%x2.7% --fill '#5a5a5a'
pixelox shape path d.pxo --in-place --name Corner --d 'M 76 130 L 76 76 L 130 76' --fill none --stroke '#ffffff' --stroke-width 3
pixelox shape path d.pxo --in-place --name Corner2 --d 'M 1004 1220 L 1004 1274 L 950 1274' --fill none --stroke '#ffffff' --stroke-width 3
pixelox export d.pxo -o T-rules.png
```

**Parameters.** Accent bar: 4 to 6% of the width, 2 to 4 px thick, placed between the
date block and the body. Hairlines: 1 px at 30 to 50% opacity, the width of the text
column. Dotted line: `--dash 0,22 --cap round` with the stroke width as the dot size.
Section markers: a small gray square behind the first letters of a sub-heading. Corner
ticks: 5% of the width, at the margin corners.

**Fails when.** Lines are decoration without a job: every rule must separate two things or
align with a text edge.

## T-lens-flare

**Purpose.** A camera flare or light hit: sunrise and "glory" moments, product sparkle, the
light source behind a silhouette.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#0b0d14'
pixelox comp place d.pxo photo.png --in-place --as Photo --fit cover
pixelox adjust levels d.pxo --in-place --layer Photo --gamma 0.7
pixelox render lens-flare d.pxo --in-place --at 69%,39% --lens 105mm --brightness 130 --color '#ffd9a0'
pixelox export d.pxo -o T-lens-flare.png
```

**Parameters.** `--lens 105mm` is a big soft glow, `35mm` a small sharp starburst, `zoom`
adds a chain of ghosts, `movie` a blue anamorphic streak. Put the center exactly on a
plausible light source (sun, lamp, the edge of a silhouette). `--brightness` 80 to 160;
tint with `--color`. The flare is its own `Lens Flare` layer in screen mode: lower it with
`layer opacity`.

**Fails when.** There is no light source where it sits: a flare in empty sky or on a face
looks like a mistake.

## T-credit-block

**Purpose.** The billing block at the foot of a film poster (tiny condensed credits), with
an optional row of rating stars and a pull quote above the title.

```bash
pixelox new -o d.pxo --size 1080x1620 --fill '#0e0e10'
pixelox shape star d.pxo --in-place --name Star1 --center 430,120 --points 5 --outer 20 --inner 9 --fill '#f2c230'
pixelox shape star d.pxo --in-place --name Star2 --center 485,120 --points 5 --outer 20 --inner 9 --fill '#f2c230'
pixelox shape star d.pxo --in-place --name Star3 --center 540,120 --points 5 --outer 20 --inner 9 --fill '#f2c230'
pixelox shape star d.pxo --in-place --name Star4 --center 595,120 --points 5 --outer 20 --inner 9 --fill '#f2c230'
pixelox shape star d.pxo --in-place --name Star5 --center 650,120 --points 5 --outer 20 --inner 9 --fill '#f2c230'
pixelox text add d.pxo --in-place --name Quote --content "A BOLD, FUNNY, FEARLESS DEBUT" --font "Avenir Next Condensed" --weight 600 --size 1.5% --tracking 120 --color '#e9e6df' --at 50%,10.4% --anchor c
pixelox text add d.pxo --in-place --name Title --content "THE LONG WAY" --font Impact --size 11% --color '#ffffff' --at 50%,72% --anchor c
pixelox text add d.pxo --in-place --name Tagline --content "SOME ROADS CHOOSE YOU" --font "Avenir Next Condensed" --weight 500 --size 1.9% --tracking 260 --color '#e9e6df' --at 50%,79.5% --anchor c
pixelox text add d.pxo --in-place --name Credits --content $'NORTH LIGHT PICTURES presents a QUIET HARBOR production  a film by LEE VARGA\ncasting by SAM OKORO  music by THE NIGHT OWLS  costume design JO MARSH  editor PIA LUND\nproduced by ANA REYES and TOM HALE  written and directed by LEE VARGA' --font "Avenir Next Condensed" --weight 500 --size 1.25% --tracking 60 --leading 135% --align center --color '#8f8c86' --at 50%,87.5% --anchor c
pixelox text add d.pxo --in-place --name Date --content "IN CINEMAS THIS AUTUMN" --font "Avenir Next Condensed" --weight 700 --size 1.6% --tracking 300 --color '#f2c230' --at 50%,94% --anchor c
pixelox export d.pxo -o T-credit-block.png
```

**Parameters.** Credits: a condensed face at cap height 0.8 to 1% of the height, mid-gray
on dark (never white), centered, three or four lines, 60 to 75% of the width, with names in
capitals and roles in lowercase. Stars: five small `shape star` layers 2.5% of the width
apart. Tagline: tracked 200 to 300, just under the title. All invented names. This block is
texture as much as text: its even gray band anchors the poster's foot.

**Fails when.** The credits are readable from across the room: too large or too bright;
they should be found only when looked for.

---

# Finishing

## T-vignette

**Purpose.** Darken the edges so the eye stays in the middle. Non-destructive: one gradient
layer over everything except the type.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#b8452c'
pixelox layer add d.pxo --in-place --name Vignette
pixelox fill gradient d.pxo --in-place --layer Vignette --kind radial --from 50%,45% --to 50%,125% --stops '#00000000@0.4,#000000b3@1'
pixelox export d.pxo -o T-vignette.png
```

**Parameters.** The first stop's position (0.3 to 0.55) is where darkening begins; the last
stop's alpha (`66` light, `b3` strong, `e6` heavy) is the corner darkness. Move `--from` to
the focal point. Use a dark shade of the ground color instead of black to keep colors rich
(`#3a0d0500` to `#3a0d05cc`). For one photo layer only, `filter vignette --amount 40` works
directly on that layer.

**Fails when.** Corners turn muddy gray on a light design: vignette dark designs only; on
light ones use a lighter center glow instead (T-spotlight).

## T-final-grade

**Purpose.** The last pass that makes separate layers feel like one printed image: merge a
copy, add punch, unify color, add grain.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#101216'
pixelox comp place d.pxo photo.png --in-place --as Photo --fit cover
pixelox text add d.pxo --in-place --name Title --content "HORIZON" --font "Avenir Next Condensed" --weight 700 --size 12% --tracking 200 --color '#ffffff' --at 50%,80% --anchor c
pixelox layer stamp d.pxo --in-place --name Grade
pixelox adjust clarity d.pxo --in-place --layer Grade --amount 30
pixelox adjust curves d.pxo --in-place --layer Grade --points '0,6 60,50 190,205 255,250'
pixelox layer add d.pxo --in-place --name Grain --fill '#808080'
pixelox filter add-noise d.pxo --in-place --layer Grain --amount 14 --monochrome
pixelox layer blend d.pxo --in-place --layer Grain --mode soft-light
pixelox layer opacity d.pxo --in-place --layer Grain --value 50
pixelox export d.pxo -o T-final-grade.png
```

**Parameters.** `layer stamp` merges everything visible into a new top layer and leaves the
layers below intact (delete the stamp to keep editing). Clarity 15 to 40 adds midtone punch
(negative values give a soft glow). The curve here is a gentle S with lifted blacks.
Lower the stamp's opacity to 50 to 80 to dial the grade back. Do this last, then export.

**Fails when.** Small text gets crunchy: use clarity under 25, or stamp before adding the
small text.
