# Technique library

Named, reusable effect recipes. Poster descriptions cite them by ID (`T-smoke`, `T-duotone`).
Every block below was run end to end with pixelox 0.5 and its output was looked at.
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
* Change `--seed` to get a different random pattern from any generator or native effect
  (`fill noise`, `render splash`, `render ice`, `filter grain`). The same recipe always
  gives the same picture, but a native effect's pattern also depends on how many commands
  ran before it: after you add or remove earlier lines, look again and pick a new `--seed`
  if a wisp, a cube, or a splash now covers something.
* New layers land on top of the stack. To put one elsewhere, add `--below NAME` or
  `--above NAME` to the command that creates it (or move it later with `layer reorder`).
* `--font @condensed-heavy` names a font **class**; it resolves to the best installed face on
  any machine (`pixelox fonts classes`, [typography.md](typography.md)).
* Filters on text, shapes, and placed images are live: the layer stays editable, nothing needs
  `layer rasterize` first.
* Add `--json` to each call when you need to read the result (layer bounds, warnings).

## Index

| Group | IDs |
| --- | --- |
| Backgrounds (24) | T-gradient-mesh, T-smoke, T-fire-glow, T-spotlight, T-grunge, T-paper-grain, T-bokeh, T-light-leak, T-sunburst, T-waves, T-geo-shapes, T-pattern, T-torn-collage, T-brush-swash, T-splatter, T-light-streaks, T-fluid-ribbons, T-tile-mosaic, T-ring-arcs, T-spoke-burst, T-newsprint, T-swoosh-band, T-haze-band, T-difference-clouds |
| Photo and subject (20) | T-cutout-drop-shadow, T-text-behind-subject, T-duotone, T-mono-on-color, T-halftone-portrait, T-stencil-poster, T-double-exposure, T-silhouette, T-photo-fade, T-glitch, T-distort, T-motion-streak, T-color-grade, T-shape-crop, T-floor-reflection, T-water-splash, T-ice-cubes, T-condensation, T-torn-photo, T-device-mockup |
| Type (24) | T-fit-width, T-outline-type, T-gradient-type, T-image-in-type, T-distress-type, T-block-shadow, T-long-shadow, T-extrude-type, T-neon-glow, T-fire-type, T-vertical-type, T-arc-type, T-script-overlap, T-label-box, T-stretch-type, T-sliced-type, T-ghost-type, T-floor-type, T-counterchange, T-warp-type, T-stencil-type, T-texture-fill, T-crack, T-object-as-letter |
| Graphic furniture (10) | T-pill-button, T-price-burst, T-info-bar, T-icon-row, T-qr-code, T-barcode, T-frame-border, T-rules, T-lens-flare, T-credit-block |
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
pixelox filter grain d.pxo --in-place --as-layer --name Grain --amount 20
pixelox export d.pxo -o T-gradient-mesh.png
```

**Parameters.** `edge` 0.85 keeps the middle dark for type; 0 scatters color everywhere
(use with a light `bg` for pastel meshes). `size` 0.4 to 0.8: larger blobs, calmer picture.
`--sigma` 40 to 90: below 40 the blobs read as separate discs. `colors`: 2 to 4 hues that sit
next to each other on the wheel (orange, amber, pink) plus at most one contrast hue. Place
exact glows with `--param at='0.1,0.05;0.95,0.9'` (canvas fractions, one per blob). For a
flowing, lava-lamp mesh without the script, fill one layer with smooth noise instead:
`pixelox fill noise d.pxo --in-place --as-layer --name Mesh --scale 90% --octaves 2 --stops '#140d0b@0.35,#7a3cff@0.5,#e83e8c@0.62,#ff5a1f@0.75,#ffb000@0.9'`.

**Fails when.** Banding (visible steps in the gradient): keep the Grain layer; it is what
hides them. Muddy gray-brown middle: the hues are complements; drop one or raise `edge`.

## T-smoke

**Purpose.** Drifting smoke or fog on a dark ground: moody concert, film, and product
backdrops.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#07080c'
pixelox fill noise d.pxo --in-place --as-layer --name Smoke --kind smoke --seed 3 --stops 'transparent,#c9d2e0' --blend screen --opacity 85
pixelox mask add d.pxo --in-place --layer Smoke --gradient linear --from 0,30% --to 0,95% --ease smooth
pixelox export d.pxo -o T-smoke.png
```

**Parameters.** One command draws the smoke on its own layer, already in `screen` mode.
The second color of `--stops` tints it: `#c9d2e0` cold, `#e8c9a0` warm, or a brand color
for colored stage smoke. `--bias` 15 thickens it, `--bias=-15` thins it to a few wisps;
`--angle 0` makes it drift sideways; `--scale` 40 to 100% sets the size of the curls;
change `--seed` for another pattern. The gradient mask clears the lower part for text
(revealed at `--from`, hidden at `--to`); swap the two points to clear the top instead, or
leave the mask out for smoke everywhere. For thick fog add a second layer under it:
`pixelox fill noise d.pxo --in-place --as-layer --name Fog --below Smoke --scale 60% --stops 'transparent,#8fa0c0' --blend screen --opacity 40`.
For smoke that rises from one object, use the live style on that layer instead:
`pixelox fx smoke d.pxo --in-place --layer Subject --height 80% --density 40`.

**Fails when.** It grays the whole picture: lower `--opacity` to 50 to 70 or use a negative
`--bias`. It is invisible: the ground is too light; smoke in `screen` mode needs a ground
darker than 20% lightness (on a light ground use dark stops and `--blend multiply`).

## T-fire-glow

**Purpose.** A hot, swirling red-orange field with flying embers: revival, concert, and
sports flyers. No photo of fire needed.

```bash
pixelox render js scripts/particles.js -o embers.png --size 1080x1350 --seed 5 --param shape=glow --param colors='#ffd27a,#ff8a1f,#fff1c9' --param count=140 --param min=0.15 --param max=0.7 --param grow=0.8
pixelox new -o d.pxo --size 1080x1350 --fill '#3a0604'
pixelox fill noise d.pxo --in-place --as-layer --name Heat --kind turbulence --seed 11 --scale 50% --octaves 4 --warp 60 --stops '#3a0604,#c2260f,#ff9a3d'
pixelox filter radial-blur d.pxo --in-place --layer Heat --kind spin --amount 14 --center 50%,45%
pixelox layer add d.pxo --in-place --name Shade
pixelox fill gradient d.pxo --in-place --layer Shade --kind radial --from 50%,42% --to 50%,115% --stops '#1a030200@0.35,#1a0302d9@1'
pixelox comp place d.pxo embers.png --in-place --as Embers --fit cover
pixelox layer blend d.pxo --in-place --layer Embers --mode screen
pixelox export d.pxo -o T-fire-glow.png
```

**Parameters.** The three colors set the temperature: `#3a0604,#c2260f,#ff9a3d` is fire;
`#02122e,#0b4fd1,#5fd0ff` is the same effect as ice or water; `#12051f,#6a1bb3,#ff5ad1` is
neon. `--amount` 8 to 25 is the swirl of the spin blur; `--warp` 30 to 120 the swirl inside
the noise; `--scale` 35 to 70% the size of the billows. For embers that streak, use
`shape=streak` with `angle=-75`.

**Fails when.** White type is hard to read on the bright patches: put a `fx drop-shadow`
(offset 0,6, blur 18, opacity 45) on the headline, or strengthen the Shade layer.

## T-spotlight

**Purpose.** One flat color with a pool of light behind the hero and a contact shadow under
it: the classic product-ad stage.

```bash
pixelox transform trim product.png -o tight.png
pixelox new -o d.pxo --size 1080x1350 --fill '#9c0f14'
pixelox shape ellipse d.pxo --in-place --name Glow --rect 15%,14%,70%x64% --fill '#ff6a55'
pixelox filter gaussian d.pxo --in-place --layer Glow --sigma 120
pixelox layer add d.pxo --in-place --name Corners
pixelox fill gradient d.pxo --in-place --layer Corners --kind radial --from 50%,46% --to 50%,125% --stops '#00000000@0.45,#000000a6@1'
pixelox shape ellipse d.pxo --in-place --name Shadow --rect 36%,83.5%,28%x4% --fill '#3a0204'
pixelox filter gaussian d.pxo --in-place --layer Shadow --sigma 16
pixelox comp place d.pxo tight.png --in-place --as Product --size x760 --at 50%,86% --anchor b
pixelox export d.pxo -o T-spotlight.png
```

**Parameters.** Glow and shadow are plain shapes with a blur on them. The blur is live:
the shape can still be moved, resized (`shape set`), and recolored, and it is not cut off
at the shape's edge. Glow color: the background hue, lighter and a little warmer (never
white); its ellipse covers 60 to 75% of the width and `--sigma` is 10 to 12% of the width.
Shadow: 60 to 80% of the product's width, 3 to 5% of the canvas height, centered on the
product's base, color = the background darkened, `--sigma` 12 to 20. For a floor line, put
a darker `shape rect` over the bottom 15 to 25% before the shadow.

**Fails when.** The product floats: the shadow is missing, too far below it, or too sharp.
Stand-in and loosely cropped cutouts have empty space under the object: `transform trim`
the file first (as here) so the shadow meets it. The glow looks like a hard disc: `--sigma`
is too small for the ellipse; keep it above a seventh of the ellipse's width.

## T-grunge

**Purpose.** A worn, dirty, printed-and-handled surface: street, underground, vintage, and
protest posters. Works under bold black type.

```bash
pixelox render js scripts/grunge.js -o scratches.png --size 1080x1350 --seed 9 --param color='#2a2118' --param specks=0 --param blotches=0 --param scratches=45
pixelox new -o d.pxo --size 1080x1350 --fill '#ddd3bf'
pixelox fill noise d.pxo --in-place --as-layer --name Mottle --seed 21 --scale 25% --warp 16 --stops '#8a7a62,#f6eedc' --blend multiply --opacity 60
pixelox fill noise d.pxo --in-place --as-layer --name Stains --seed 4 --kind turbulence --scale 16% --stops 'transparent@0.8,#2a2118@0.92' --opacity 40
pixelox fill noise d.pxo --in-place --as-layer --name Specks --seed 7 --scale 0.7% --octaves 2 --stops 'transparent@0.84,#2a2118@0.9' --opacity 70
pixelox comp place d.pxo scratches.png --in-place --as Scratches --fit cover
pixelox layer opacity d.pxo --in-place --layer Scratches --value 55
pixelox filter grain d.pxo --in-place --as-layer --name Grain --amount 30
pixelox export d.pxo -o T-grunge.png
```

**Parameters.** Four kinds of wear, each its own layer, so you can drop or dim any of them:
**Mottle** (uneven paper tone, multiplied), **Stains** (a few soft blotches), **Specks**
(fine dirt), **Scratches** (`grunge.js` with only its scratches). The two positions in a
`--stops` pair are the threshold: raise both (`@0.86`, `@0.95`) for fewer, smaller marks,
lower them for more. Paper color `#ddd3bf` (aged), `#e9e6e1` (newsprint), `#1a1a1a` (dark
wall: then use white marks in `screen` mode and mottle stops `'#000000,#555555'`). Put
Stains, Specks, Scratches, and Grain **above** photos and flat shapes too, so everything
shares one surface; keep small text above them.

**Fails when.** It looks like a rash rather than wear: marks are too many and too even;
raise the thresholds and keep Stains under 45% opacity.

## T-paper-grain

**Purpose.** Fine film or paper grain over the finished design. Removes the "too clean,
too digital" look and hides gradient banding. Use it on almost everything, last.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#e8452c'
pixelox shape ellipse d.pxo --in-place --name Disc --rect 240,375,600x600 --fill '#14110f'
pixelox filter grain d.pxo --in-place --as-layer --name Grain --amount 25
pixelox export d.pxo -o T-paper-grain.png
```

**Parameters.** One live adjustment layer grains everything below it and nothing above it.
`--amount` 10 to 15 is invisible-but-felt (clean corporate work), 20 to 30 is visible print
grain (retro, editorial), 40 to 60 is heavy. The grain keeps the same visible strength at
any canvas size; `--size 3` makes it coarser, `--color 30` adds a little color noise (old
film). Dial it back later with `pixelox layer opacity d.pxo --in-place --layer Grain --value 60`,
or remove it with `layer rm`. For grain on one layer only, `filter grain --layer Photo`
(pixels) or `fx grain --layer Title` (live, on type).

**Fails when.** Small text turns gritty: add the grain before the small text, or move it
down: `pixelox layer reorder d.pxo --in-place --layer Grain --below Body`.

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
(fish-scale arcs for cloth and wave borders). A real barcode is `shape barcode`
(T-barcode). `cell` 20 to 40 for dots (2 to 4% of the width), `weight` 0.2 to 0.45.
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

**Fails when.** A blurred shape arc dims where it leaves the canvas: `filter gaussian` on a
shape is fine for glows that sit inside the canvas (T-spotlight), but the part of a shape
that lies outside the canvas is not blurred back in. Light that bleeds off an edge is
painted, as here, or drawn with `fill gradient`.

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
pixelox new -o d.pxo --size 1080x1350 --fill '#1a0f0a'
pixelox comp place d.pxo subject.png --in-place --as Left --size x760 --at 30%,96% --anchor b
pixelox comp place d.pxo subject2.png --in-place --as Right --size x760 --at 70%,96% --anchor b
pixelox layer add d.pxo --in-place --name Haze
pixelox fill gradient d.pxo --in-place --layer Haze --from 0,66% --to 0,100% --stops '#e8913a00@0,#e8913acc@0.55,#ffd9a0ff@1'
pixelox fill noise d.pxo --in-place --as-layer --name Fog --seed 5 --scale 35% --warp 40 --stops 'transparent,#ffd9a0' --blend screen
pixelox mask add d.pxo --in-place --layer Fog --gradient linear --from 0,100% --to 0,60% --ease smooth
pixelox export d.pxo -o T-haze-band.png
```

**Parameters.** The gradient runs from the glow color at alpha 0 (where the bodies are
still clean, about two thirds down) to solid at the bottom edge; the fog on top breaks up
its straight edge, and its gradient mask keeps it in the lower 40%. Use the scene's light
color (amber, white, the ground color for a fade to nothing). The headline or info bar
then sits on the band.

**Fails when.** A hard line shows where the gradient starts: lengthen the ramp to at least
30% of the height.

## T-difference-clouds

**Purpose.** Electric veins, lightning, marble, and plasma: the "Difference Clouds" trick.
Two cloud fills blended in `difference` mode leave thin dark lines where they are equal;
inverted and pushed with levels, those lines glow.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#05060a'
pixelox fill noise d.pxo --in-place --as-layer --name Veins --seed 3 --scale 60%
pixelox fill noise d.pxo --in-place --layer Veins --seed 8 --scale 60% --blend difference
pixelox adjust invert d.pxo --in-place --layer Veins
pixelox adjust levels d.pxo --in-place --layer Veins --in-black 170 --gamma 0.6
pixelox adjust gradient-map d.pxo --in-place --layer Veins --stops '#05060a@0,#2a4bd8@0.6,#bfe0ff@0.9,#ffffff@1'
pixelox export d.pxo -o T-difference-clouds.png
```

**Parameters.** `--scale` 40 to 80% sets how far apart the veins are; a third `difference`
pass with another seed doubles their number. `--in-black` 140 (thick, glowing) to 200
(hairlines). The gradient map colors the result from background (first stop) to core (last
stop): blue-white is electricity, `#12051f,#b3261e,#ffd27a` is lava cracks, a white-to-gray
map without the `invert` step is marble. Use it full-bleed behind type, or set the layer to
`screen` over a dark photo for lightning. One-step relatives: `fill noise --kind ridged`
(veins without the difference pass) and `--kind cell-edges` (cracked mud, crackle glaze).

**Fails when.** Everything glows and nothing is dark: `--in-black` is too low; raise it
until two thirds of the canvas is the background color.

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
pixelox text add d.pxo --in-place --name Word --content "SUNDAY" --font @condensed-heavy --fit-width 980 --at 50%,9% --anchor t --anchor-to ink --color '#161616'
pixelox comp place d.pxo subject.png --in-place --as Subject --size x1190 --at 50%,100% --anchor b
pixelox fx drop-shadow d.pxo --in-place --layer Subject --offset 0,10 --blur 40 --opacity 45
pixelox export d.pxo -o T-text-behind-subject.png
```

**Parameters.** The word spans 85 to 100% of the width (`--fit-width`) and its cap height
is 25 to 45% of the canvas height. The subject must cover 15 to 40% of the word and leave
the first and last letters clear, so the word still reads. Order: ground, word, subject,
then small text on top. A word added after the subject goes behind it with
`--below Subject`. A second line of the word can pass in front of the body (leave it on
top) for an interlocked look.

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
pixelox mask add d.pxo --in-place --layer Photo --gradient linear --from 0,22% --to 0,58%
pixelox export d.pxo -o T-photo-fade.png
```

**Parameters.** A gradient mask on the photo itself: the photo is whole at `--from`, gone
at `--to`, and whatever is below it (the ground color) shows through. The fade starts
where the photo's subject ends and is complete before the first line of text (a ramp 25
to 40% of the height). Fade toward the top (`--from 0,60% --to 0,20%`) or sideways
(`--from 45%,0 --to 100%,0`) the same way; `--ease smooth` hides the start and end of the
ramp; `--gradient radial --from 50%,40% --to 50%,95%` keeps an oval and fades all around
it. It works on a cutout too (a person cut off at the waist fades into the ground); on a
layer that already has a mask, add `--combine intersect` so both apply. To darken a photo
under text without hiding it, use a gradient **layer** instead: `layer add --name Shade`,
then `fill gradient --layer Shade --from 0,50% --to 0,100% --stops '#00000000@0,#000000cc@1'`.

**Fails when.** A hard photo edge still shows: the photo ends before the fade is complete;
make the photo taller or move `--to` up.

## T-glitch

**Purpose.** Digital damage: color channels pulled apart, displaced slices, scanlines.
Tech, gaming, electronic music, cyber themes.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#000000'
pixelox comp place d.pxo photo.png --in-place --as Photo --fit cover
pixelox filter glitch d.pxo --in-place --layer Photo --split 2% --slices 5 --slice-shift 6% --seed 4
pixelox text add d.pxo --in-place --name Word --content "SIGNAL" --font @heavy-wide --fit-width 860 --at 50%,72% --anchor c --anchor-to ink --color '#ffffff'
pixelox filter glitch d.pxo --in-place --layer Word --split 12 --slices 4 --slice-shift 5% --scanlines 0 --seed 9
pixelox export d.pxo -o T-glitch.png
```

**Parameters.** One filter does the red/cyan split, the shifted bands, and the scanlines.
`--split` 1 to 3% of the width (more reads as 3D glasses); `--slices` 2 to 8 bands,
`--slice-shift` 3 to 8%; `--scanlines` 20 to 40 (0 = none); `--seed` moves the bands. On a
photo, a placed image, or type the filter is live: the text can still be edited, and
`layer smart-filter` turns it off. Glitch the photo and the headline separately, with
different seeds, and leave the small text clean.

**Fails when.** The picture just looks blurry: the split is too small or the image has no
hard edges; glitch needs contrast. A band cuts through a face or the key word: change
`--seed`.

## T-distort

**Purpose.** Bend pixels and type: a twirl in a sky, a waving stripe or flag, heat haze and
melted lettering. For the 15 warp shapes (arc, flag, rise, bulge) see T-warp-type.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#101216'
pixelox comp place d.pxo photo.png --in-place --as Photo --fit cover
pixelox filter distort d.pxo --in-place --layer Photo --kind twirl --angle 220 --center 50%,36% --radius 45%
pixelox shape rect d.pxo --in-place --name Band --rect 0,58%,100%x10% --fill '#ffd23c'
pixelox filter distort d.pxo --in-place --layer Band --kind wave --amplitude 26 --wavelength 270
pixelox text add d.pxo --in-place --name Word --content "HEAT" --font @heavy-wide --fit-width 760 --at 50%,84% --anchor c --anchor-to ink --color '#ff5a1f'
pixelox filter displace d.pxo --in-place --layer Word --amount 2% --scale 6% --seed 3
pixelox export d.pxo -o T-distort.png
```

**Parameters.** `filter distort --kind` is `wave` (`--amplitude`, `--wavelength`,
`--shape sine|triangle|square`), `ripple` (rings from `--center`), `twirl` (`--angle` at
the center, fading out at `--radius`), `pinch`, `spherize` (`--amount`), or `polar` (wraps
a strip into a ring). `filter displace` without a map pushes pixels along generated noise:
`--amount` 1 to 3% is heat haze or wet paint, `--scale` 4 to 12% the size of the wobbles.
On photos, shapes, and type these are live filters: the text stays editable and
`layer smart-filter` removes the effect.

**Fails when.** Type becomes unreadable: keep `--amount` under 3% on text and distort one
word, not a sentence.

## T-motion-streak

**Purpose.** Speed lines pulled out of the subject or the type: sport, dance, fashion,
launch energy.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#f6d7d9'
pixelox comp place d.pxo subject.png --in-place --as Streak --size x1050 --at 50%-150,100% --anchor b
pixelox filter motion-blur d.pxo --in-place --layer Streak --angle 0 --distance 420
pixelox layer opacity d.pxo --in-place --layer Streak --value 85
pixelox comp place d.pxo subject.png --in-place --as Subject --size x1050 --at 50%,100% --anchor b
pixelox export d.pxo -o T-motion-streak.png
```

**Parameters.** `--distance` 25 to 50% of the width; `--angle` 0 (sideways), 90 (falling or
rising), 20 to 30 (dynamic diagonal). Place the streak copy a third of the distance behind
the subject, against the direction of travel (`50%-150` is 150 px left of center), so it
trails. Tint it with
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
pixelox comp place d.pxo tight.png --in-place --as Product --size x620 --at 50%,70% --anchor b
pixelox fx reflection d.pxo --in-place --layer Product --length 30% --opacity 40 --blur 8
pixelox export d.pxo -o T-floor-reflection.png
```

**Parameters.** The reflection is a live style on the product: it follows the layer when
you move, scale, or relink it. `--length` 25 to 45% of the product's height, `--opacity`
25 to 45, `--blur` 0 (polished glass) to 12 (satin; the blur grows away from the floor
line), `--gap 6` lifts the object a little. Trim the cutout first (`transform trim` removes
the transparent border), or the reflection will not touch the product. A slightly lighter
floor band starting at the floor line sells the surface. It works on type and shapes too.

**Fails when.** There is a gap between product and reflection: the cutout was not trimmed.
The reflection is cut off: the product stands too close to the bottom edge; leave at least
the reflection's length below it.

## T-water-splash

**Purpose.** Liquid thrown up around a product: drinks, freshness, sport. A crown of clear
liquid lit like glass, in front of and behind the product.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#041c4a'
pixelox shape ellipse d.pxo --in-place --name Glow --rect 14%,30%,72%x58% --fill '#1f6fe0'
pixelox filter gaussian d.pxo --in-place --layer Glow --sigma 110
pixelox render splash d.pxo --in-place --name SplashBack --part back --at 50%,86% --size 95% --seed 3
pixelox comp place d.pxo product.png --in-place --as Product --size x900 --at 50%,92% --anchor b
pixelox render splash d.pxo --in-place --name SplashFront --part front --at 50%,86% --size 95% --seed 3
pixelox export d.pxo -o T-water-splash.png
```

**Parameters.** The product stands **in** the crown: draw the back half, place the
product, draw the front half, with the same `--at`, `--size`, and `--seed` on both; the
product hides where the two halves meet (if the pool's outline shows a step at its sides,
change `--seed`). `--at` is the center of the pool, a little above the product's base;
`--size` 70 to 110% of the shorter side. `--liquid` is `water`, `milk`, `juice`, `cola`, `coffee`, `wine`, or `oil`;
`--color '#ff5a1f'` recolors any of them (orange soda). `--kind sheet` throws streams from
a point instead of raising a ring: `--angle 90` is straight up, `--spread` 60 to 180 the
fan. A sheet behind the product plus the front half of a small crown at its foot is the
classic juice ad:
`pixelox render splash d.pxo --in-place --kind sheet --liquid juice --at 50%,88% --size 90% --spread 150 --below Product`.
Clear water needs a ground it can show against: mid to dark, saturated, with a glow
behind it. For a photographic splash, the user must supply a splash photo: place it in
`screen` mode.

**Fails when.** The splash covers the label: lower `--at` or shrink `--size`, or change
`--seed`. Water is invisible on a pale ground: use `--liquid milk` or `juice`, or darken
the ground.

## T-ice-cubes

**Purpose.** Clear ice cubes tumbling around a cold drink. The cubes are ray-traced: they
bend the design behind them.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#06245c'
pixelox shape ellipse d.pxo --in-place --name Glow --rect 14%,25%,72%x60% --fill '#1f6fe0'
pixelox filter gaussian d.pxo --in-place --layer Glow --sigma 110
pixelox render ice d.pxo --in-place --name IceBack --at 50%,40% --count 5 --size 11% --spread 42% --seed 4
pixelox filter gaussian d.pxo --in-place --layer IceBack --sigma 3
pixelox comp place d.pxo product.png --in-place --as Product --size x900 --at 50%,92% --anchor b
pixelox render ice d.pxo --in-place --name IceFront --at 50%,88% --count 4 --size 17% --spread 30% --seed 11
pixelox export d.pxo -o T-ice-cubes.png
```

**Parameters.** Two groups sell the depth: small, slightly blurred cubes behind the
product and a few large sharp ones in front of its base. `--count` cubes scatter within
`--spread` of `--at`; `--size` is the edge length (10 to 18% of the shorter side). Few and
large beats many and small. `--frost` 0 is clear glass, 55 the default cloudy core, 100
white; `--tint` colors the ice. Change `--seed` until no cube covers the label. **Order
matters:** a cube shows what is behind it at the moment the command runs, so add the ice
after the background and the product, and run it again if they change.

**Fails when.** The cubes look like gray stickers: the ground is too light or too flat; ice
needs a dark, saturated ground with a glow or a product to refract.

## T-condensation

**Purpose.** Beads of water on a cold can, bottle, or glass.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#7a0c10'
pixelox comp place d.pxo product.png --in-place --as Product --size x1100 --at 50%,96% --anchor b
pixelox render droplets d.pxo --in-place --layer Product --density 60 --trickles 5
pixelox export d.pxo -o T-condensation.png
```

**Parameters.** One command adds a `Droplets` layer directly above the product, clipped to
it, so the beads appear only on the product. Each bead is a small lens that shows the
product behind it. `--density` 30 (a few beads) to 80 (soaked); `--trickles` is the number
of drops running down with a trail; `--mist` 0 to 100 adds the fine frosting between
beads; `--size` sets the largest bead (2 to 4% of the product width). **Order matters:**
the beads capture the product when the command runs, so run it right after placing the
product and **before** any `fx outer-glow` or `fx drop-shadow` on it (beads drawn after a
glow also land on the glow). After `comp relink`, a move, or a recolor, delete the layer
(`layer rm --layer Droplets`) and run it again.

**Fails when.** Beads cover the logo: erase a zone with
`pixelox retouch erase d.pxo --in-place --layer Droplets --mask @sel` after selecting the label,
or lower `--density`.

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

Fonts in these recipes are classes (`--font @condensed-heavy`, `@heavy-wide`,
`@geometric-sans`, `@script-formal`): each draws the best installed face of that kind on any
machine, and `--weight` picks the weight inside it. A named family that is not installed is
replaced by the closest face of its class with warning PXW4010; see
[typography.md](typography.md). Text stays live under everything here: `fx ...`, `filter ...`
(a live smart filter), `transform ...`, `text warp`, `layer clip`, and masks all keep the
words editable with `text edit`. Only `fill ...` and `paint stroke` need pixels
(`layer rasterize`), and for a new color or gradient use `text style --color` or `fx overlay`
instead.

## T-fit-width

**Purpose.** A stacked headline where every line is fitted to exactly the same width, so the
block forms a solid rectangle: the strongest type-only poster layout.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#f1ede4'
pixelox text add d.pxo --in-place --name L1 --content "MAKE" --font @condensed-heavy --fit-width 920 --at 80,90 --anchor-to ink --color '#111111'
pixelox text add d.pxo --in-place --name L2 --content "NOISE" --font @condensed-heavy --fit-width 920 --at 80,457 --anchor-to ink --color '#e4002b'
pixelox text add d.pxo --in-place --name L3 --content "TONIGHT" --font @condensed-heavy --fit-width 920 --at 80,817 --anchor-to ink --color '#111111'
pixelox export d.pxo -o T-fit-width.png
```

**Parameters.** `--fit-width 920` sizes the type so its visible letters span exactly 920 px
(leave `--size` out; `85%` of the canvas width works too). `--anchor-to ink` makes `--at`
place the visible letters instead of the text box, so `--at 80,90` puts the top-left of the
letters exactly there; with `--anchor t` or `c` it centers them on `--at`. Read each line's
height from the `bounds` in the JSON result (`[x, y, w, h]`) and place the next line at
`y + h + gap`; the gap is 2 to 3% of the canvas height (here 24 px). Width = canvas minus
two margins. Short words become tall, long words short: order the lines so the most
important word is the shortest. The fit is live: `text edit --content` refits the new word.

**Fails when.** A long line becomes tiny: split it, or let it be a smaller second-level
line on purpose. Mixed fonts break the block; use one heavy condensed face.

## T-outline-type

**Purpose.** Letters drawn as outlines only, often repeated or paired with a filled copy:
fashion, music, sport, modern editorial.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#1437e6'
pixelox text add d.pxo --in-place --name Ghost1 --content "LOUDER" --font @heavy-wide --size 16% --color transparent --stroke '#ffffff' --stroke-width 3 --at 50%,24% --anchor c
pixelox text add d.pxo --in-place --name Ghost2 --content "LOUDER" --font @heavy-wide --size 16% --color transparent --stroke '#ffffff' --stroke-width 3 --at 50%,41% --anchor c
pixelox text add d.pxo --in-place --name Solid --content "LOUDER" --font @heavy-wide --size 16% --color '#ffffff' --at 50%,58% --anchor c
pixelox text add d.pxo --in-place --name Ghost3 --content "LOUDER" --font @heavy-wide --size 16% --color transparent --stroke '#ffffff' --stroke-width 3 --at 50%,75% --anchor c
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
pixelox text add d.pxo --in-place --name Chrome --content "Code" --font @neo-grotesque --weight 700 --size 24% --color '#ffffff' --tracking -30 --at 50%,30% --anchor c
pixelox fx overlay d.pxo --in-place --layer Chrome --stops '#ffffff@0,#c9ccd2@0.45,#5b5f68@1' --angle 0
pixelox text add d.pxo --in-place --name Gold --content "AWARDS" --font @didone --weight 700 --size 13% --color '#ffffff' --tracking 60 --at 50%,62% --anchor c
pixelox fx overlay d.pxo --in-place --layer Gold --stops '#8a5a00@0,#f6d365@0.35,#fff6c8@0.5,#d99a1c@0.65,#7a4b00@1' --angle 90
pixelox text add d.pxo --in-place --name Fade --content "LIGHT" --font @condensed --weight 600 --size 16% --color '#ffffff' --tracking 120 --at 50%,84% --anchor c
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
pixelox text add d.pxo --in-place --name Word --content "MOON" --font @condensed-heavy --fit-width 940 --at 50%,22% --anchor t --anchor-to ink --color '#111111'
pixelox comp place d.pxo photo.png --in-place --as Fill --fit cover
pixelox layer clip d.pxo --in-place --layer Fill
pixelox export d.pxo -o T-image-in-type.png
```

**Parameters.** `layer clip` shows the layer only where the layer directly below it has
pixels: the image goes immediately above the text, and the text stays editable. Use the
heaviest face available and tight tracking so there is enough letter area; cap height at
least 20% of the canvas. Textures (moon surface, flowers, crowd, marble, a T-fire-glow
fill) work better than photos with one small subject. The other way round, when the photo
must stay where it is in the stack: `pixelox mask add d.pxo --in-place --layer Fill --from-layer Word`
masks the photo with the word (live), then `pixelox layer hide d.pxo --in-place --layer Word`.

**Fails when.** The image is unrecognizable and the word hard to read: choose an image with
strong contrast against the page, or add a thin `fx stroke` to the word.

## T-distress-type

**Purpose.** Worn, cracked, badly printed ink on big letters: grunge, vintage, street.

```bash
pixelox render js scripts/grunge.js -o wear.png --size 1080x1350 --seed 14 --param color='#e4d9c3' --param specks=9000 --param blotches=160 --param scratches=70 --param clump=0.4
pixelox new -o d.pxo --size 1080x1350 --fill '#e4d9c3'
pixelox text add d.pxo --in-place --name Word --content "ABLE" --font @condensed-heavy --fit-width 940 --at 50%,50% --anchor c --anchor-to ink --color '#151515'
pixelox comp place d.pxo wear.png --in-place --as Wear --fit cover
pixelox layer clip d.pxo --in-place --layer Wear
pixelox export d.pxo -o T-distress-type.png
```

**Parameters.** The wear is a grunge layer in the **background color**, clipped to the
word, so it reads as missing ink. `specks` 6000 to 12000, `blotches` 100 to 250 for heavy
wear; `clump` 0.3 to 0.6. On a photo or texture ground, use the ground's average color.
Combine with T-grunge on the whole design so type and paper wear together. For a rough,
eaten **edge** on the letters as well, add
`pixelox filter displace d.pxo --in-place --layer Word --amount 0.6% --scale 1.5%`.

**Fails when.** It looks like the word has measles: marks are too even; raise `clump`, and
add scratches.

## T-block-shadow

**Purpose.** Letters with a hard, flat shadow or solid sides and an outline: retro signage,
comics, bold sale graphics. Live styles: the text stays editable.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#ffd23c'
pixelox text add d.pxo --in-place --name Title --content "MEGA" --font @heavy-wide --size 22% --color '#ffffff' --at 50%,34% --anchor c
pixelox fx stroke d.pxo --in-place --layer Title --width 10 --color '#1b1b1b' --position outside
pixelox fx drop-shadow d.pxo --in-place --layer Title --offset 28,28 --blur 0 --color '#e4002b' --blend normal --opacity 100
pixelox text add d.pxo --in-place --name Deal --content "DEAL" --font @heavy-wide --size 22% --color '#ffffff' --at 50%,70% --anchor c
pixelox fx extrude d.pxo --in-place --layer Deal --depth 30 --color '#e4002b' --outline 8 --outline-color '#1b1b1b'
pixelox export d.pxo -o T-block-shadow.png
```

**Parameters.** Two looks. **Offset copy** (MEGA): `fx drop-shadow --blur 0 --blend normal
--opacity 100` is a flat copy shifted 10 to 18% of the cap height; it must be at least
twice the stroke width away, because the shadow is cast by the letters, not their stroke.
**Solid block** (DEAL): `fx extrude` joins letters and shadow into one block; `--depth` 10
to 18% of the cap height, `--angle 315` is down-right (225 down-left), `--outline` strokes
the whole block. Side color: black, or a strong second palette color. For deeper, shaded,
or gradient blocks see T-extrude-type.

**Fails when.** Letters fill in: the stroke is too thick for the font size; keep it under
5% of the cap height.

## T-long-shadow

**Purpose.** A flat shadow dragged diagonally from letters or an icon until it leaves the
canvas: the flat-design long shadow.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#19b5a5'
pixelox text add d.pxo --in-place --name Title --content "RETRO" --font @heavy-wide --size 16% --color '#fff4d6' --at 50%,26% --anchor c
pixelox fx long-shadow d.pxo --in-place --layer Title --color '#0d6b62' --blend normal --opacity 100
pixelox shape ellipse d.pxo --in-place --name Badge --rect 110,790,300x300 --fill '#ff6b4a'
pixelox fx long-shadow d.pxo --in-place --layer Badge --length 35% --fade 100 --opacity 45
pixelox shape icon d.pxo music --in-place --name Note --size 144 --at 260,940 --anchor c --color '#fff4d6'
pixelox fx long-shadow d.pxo --in-place --layer Note --length 80 --color '#c2442a' --blend normal --opacity 100
pixelox export d.pxo -o T-long-shadow.png
```

**Parameters.** One live style; it follows the layer when it moves or its text changes.
**Flat** (Title): a solid color, the ground darkened 30 to 40%, `--blend normal --opacity
100`, default length (to the canvas edge). **Fading** (Badge): black at `--opacity` 35 to
50 with `--fade 100` and `--length` 25 to 40% of the diagonal. **Short** (Note): `--length`
in px, inside a badge. `--angle 315` falls down-right (the default), 225 down-left. Use one
direction for every long shadow in a design.

**Fails when.** Shadows of neighboring elements cross and tangle: shorten them
(`--length`) or keep long shadows to one element.

## T-extrude-type

**Purpose.** Solid 3D block lettering: letters with real sides, shaded or in a second
color, optionally casting a long shadow. Sale headlines, retro sport, game titles.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#1b2a6b'
pixelox text add d.pxo --in-place --name Top --content "SUPER" --font @heavy-wide --fit-width 820 --at 50%,16% --anchor t --anchor-to ink --color '#ffd23c'
pixelox fx extrude d.pxo --in-place --layer Top --depth 44 --color '#b3261e'
pixelox fx stroke d.pxo --in-place --layer Top --width 4 --color '#3b0d0a'
pixelox text add d.pxo --in-place --name Mid --content "SALE" --font @heavy-wide --fit-width 820 --at 50%,42% --anchor t --anchor-to ink --color '#ffffff'
pixelox fx overlay d.pxo --in-place --layer Mid --stops '#ffcf56@0,#e4572e@1' --angle 90
pixelox fx extrude d.pxo --in-place --layer Mid --depth 12% --angle 270 --shade 50 --outline 5 --outline-color '#0b1230'
pixelox fx long-shadow d.pxo --in-place --layer Mid --length 30% --fade 100 --opacity 45
pixelox export d.pxo -o T-extrude-type.png
```

**Parameters.** `fx extrude` is a live style: `--depth` in px or % of the layer's height
(8 to 20% of the cap height reads as a block, more as a tower); `--angle` is where the
sides go (315 down-right, 270 straight down, 225 down-left). `--color` gives the sides one
flat color; left out, the sides take the face's own colors darkened by `--darken` (so a
gradient face gets gradient sides), and `--shade` darkens them toward the back.
`--outline` strokes the whole block; `fx stroke` outlines the face only. A `fx long-shadow`
on the same layer is cast by the block. It works on shapes and cutouts too (a badge, a
price burst).

**Fails when.** Thin strokes disappear into the sides: extrude heavy faces only, and keep
the depth under the letters' stroke width times 1.5.

## T-neon-glow

**Purpose.** Type or line art that glows like a neon tube: nightlife, gaming, synthwave,
"open late" energy.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#0b0614'
pixelox text add d.pxo --in-place --name Halo --content "OPEN" --font @geometric-sans --weight 600 --size 17% --color '#ff2bd6' --tracking 80 --at 50%,42% --anchor c
pixelox filter gaussian d.pxo --in-place --layer Halo --sigma 38
pixelox layer blend d.pxo --in-place --layer Halo --mode screen
pixelox text add d.pxo --in-place --name Tube --content "OPEN" --font @geometric-sans --weight 600 --size 17% --color '#fff2fc' --tracking 80 --at 50%,42% --anchor c
pixelox fx outer-glow d.pxo --in-place --layer Tube --blur 26 --spread 12 --color '#ff2bd6' --opacity 95
pixelox shape rect d.pxo --in-place --name Frame --rect 14%,28%,72%x28% --radius 40 --fill none --stroke '#38e8ff' --stroke-width 6
pixelox fx outer-glow d.pxo --in-place --layer Frame --blur 30 --spread 10 --color '#00c8ff' --opacity 90
pixelox export d.pxo -o T-neon-glow.png
```

**Parameters.** Three parts: a near-white tube (the text color is the glow color mixed 85%
toward white), a tight glow (`--blur` 2 to 3% of the width), and a wide blurred halo copy
underneath in `screen` mode (`--sigma` 3 to 5% of the width). The blur on the halo is a
live filter: both layers are still text, so change the word on both with `text edit`.
Ground must be very dark (lightness under 10%) and slightly tinted toward the glow. Medium
weight, rounded or geometric faces look like bent tubes; heavy faces look like light boxes.

**Fails when.** It looks flat: the letters are fully saturated; the tube itself must be
almost white, only the glow carries the color.

## T-fire-type

**Purpose.** Flames rising from a word (or any layer's outline).

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#070504'
pixelox text add d.pxo --in-place --name Title --content "FIRE" --font @condensed-heavy --size 26% --color '#1c0b02' --at 50%,60% --anchor c
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
pixelox text add d.pxo --in-place --name Big --content "MODERN" --font @neo-grotesque --weight 700 --size 15% --color '#ffffff' --at 50%,50% --anchor c
pixelox transform rotate d.pxo --in-place --layer Big --degrees -90
pixelox transform move d.pxo --in-place --layer Big --to 7%,93% --anchor bl
pixelox text add d.pxo --in-place --name Small --content "10 . 10 . 2030   19:00" --font @neo-grotesque --weight 400 --size 3.2% --tracking 300 --color '#ff5a1f' --at 50%,50% --anchor c
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
pixelox text add d.pxo --in-place --name Arc --content "LIMITED EDITION  *  SMALL BATCH  *  SINCE 1998  *" --font @geometric-sans --weight 700 --size 40px --tracking 110 --color '#f3ead7'
pixelox text on-path d.pxo --in-place --layer Arc --d 'M 357 675 A 183 183 0 1 1 723 675 A 183 183 0 1 1 357 675' --side outside --fit shrink
pixelox text add d.pxo --in-place --name Center --content "No.7" --font @didone --size 11% --color '#f3ead7' --at 540,675 --anchor c
pixelox export d.pxo -o T-arc-type.png
```

**Parameters.** The path is a circle written as two arcs starting at the left point and
going clockwise over the top, so the text starts at 9 o'clock; `--offset 25%` starts it at
12 o'clock. Path radius = where the baseline sits. `--fit shrink` lowers the size until the
text closes the circle without overlap, so set `--size` a little too large and let it
shrink; without it, letters that do not fit are reported (warning PXW4012). `--side inside`
puts the text inside the circle, reading the other way. A gentle arch over a headline
needs no path: use `text warp --preset arc` (T-warp-type).

**Fails when.** The shrunk text is too small to read: the text is too long for the
circle; shorten it or enlarge the circle.

## T-script-overlap

**Purpose.** A flowing script word laid over (or tucked under) a block word: the
"Gospel CONCERT / Night Worship / Coffee & Jazz" pairing. Adds warmth and a second voice.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#101a3a'
pixelox text add d.pxo --in-place --name Block --content "CONCERT" --font @condensed --weight 700 --size 15% --color '#ffffff' --tracking 40 --at 50%,52% --anchor c
pixelox text add d.pxo --in-place --name Script --content "Gospel" --font @script-formal --weight 900 --size 19% --color '#f5a623' --at 44%,41% --anchor c
pixelox transform rotate d.pxo --in-place --layer Script --degrees -6
pixelox fx drop-shadow d.pxo --in-place --layer Script --offset 0,6 --blur 14 --opacity 45
pixelox export d.pxo -o T-script-overlap.png
```

**Parameters.** The script is 1.1 to 1.5x the block word's font size, in the accent color,
rotated -4 to -8 degrees, and overlaps the block word's top or bottom edge by 15 to 30% of
its own height. A small shadow separates the two. One script word only, 4 to 10 letters,
never all caps. Script classes: `@script-formal` (copperplate hand), `@brush` (sign
painter), `@script-signature` (hairline autograph), `@marker`.

**Fails when.** The script is unreadable: it sits on a busy photo or is smaller than 8% of
the canvas height.

## T-label-box

**Purpose.** A solid box or slanted tag behind a short line of text: name plates, "NEW",
dates, a highlighted word in a sentence.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#d7f000'
pixelox shape rect d.pxo --in-place --name Box --rect 120,480,470x120 --fill '#111111'
pixelox text add d.pxo --in-place --name BoxText --content "BE YOURSELF" --font @neo-grotesque --weight 700 --fit-width 390 --at 355,540 --anchor c --anchor-to ink --color '#d7f000'
pixelox shape path d.pxo --in-place --name Tag --d 'M 150 700 L 720 700 L 690 800 L 120 800 Z' --fill '#ffffff'
pixelox text add d.pxo --in-place --name TagText --content "THIS FRIDAY" --font @neo-grotesque --weight 700 --style italic --fit-width 440 --at 420,750 --anchor c --anchor-to ink --color '#111111'
pixelox shape rect d.pxo --in-place --name Pill --rect 120,900,380x84 --radius 42 --fill '#ffffff' --stroke '#111111' --stroke-width 4
pixelox fx overlay d.pxo --in-place --layer Pill --stops '#ffffff@0,#ff9d2e@1' --angle 0
pixelox text add d.pxo --in-place --name PillText --content "free entry" --case upper --font @neo-grotesque --weight 700 --size 3% --tracking 120 --color '#111111' --at 310,942 --anchor c --anchor-to ink
pixelox export d.pxo -o T-label-box.png
```

**Parameters.** Box first, then the text fitted into it: `--fit-width` = box width minus
two paddings (padding 0.35 to 0.5 x the cap height), `--at` = the box center, and
`--anchor c --anchor-to ink` centers the visible letters on it exactly, whatever the font.
When the text decides the size instead, add the text first, read `bounds` from the JSON,
and draw the box around it with `--below BoxText`. Box and text swap the two main colors.
Slanted tag: shift the top edge 25 to 40 px to the right of the bottom edge. `fx overlay`
on a stroked shape recolors the fill and keeps the outline (the gradient pill). `--case
upper` sets the words in capitals without retyping them.

**Fails when.** The text touches the box edges: the padding is under a third of the cap
height; lower `--fit-width`.

## T-stretch-type

**Purpose.** Change a font's proportions when the right face is not installed: taller and
narrower (compressed), wider (extended), heavier (fattened), or slanted. A stand-in for
ultra-condensed, extended, and extra-black display faces. Everything stays live text.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#f1ede4'
pixelox text add d.pxo --in-place --name Tall --content "COMPRESSED" --font @condensed-heavy --scale-x 55% --fit-width 900 --at 50%,7% --anchor t --anchor-to ink --color '#111111'
pixelox text add d.pxo --in-place --name Wide --content "WIDE" --font @heavy-wide --scale-x 150% --style italic --fit-width 900 --at 50%,37% --anchor t --anchor-to ink --color '#e4002b'
pixelox text add d.pxo --in-place --name Fat --content "Heavier" --font @geometric-sans --weight 900 --faux-bold 60 --size 16% --color '#111111' --at 50%,66% --anchor c
pixelox export d.pxo -o T-stretch-type.png
```

**Parameters.** `--scale-x 55%` narrows the letters and their spacing (with `--fit-width`
the line then grows taller to keep its width); `--scale-x 150%` widens them. Keep the
scale within 50% to 160%; beyond that the strokes look wrong. `--style italic` slants (a
face without an italic is slanted for you, warning PXW4002). `--faux-bold` 20 to 80
thickens every stem (20 is about one weight step) with sharp corners; for rounded, chunky
fattening use an outside `fx stroke` in the text color instead. First try a real face of
the right width: `--stretch condensed` or `--stretch expanded` picks one when the family
has it (`"Helvetica Neue" --weight 900 --stretch condensed`), and the classes
`@compressed` and `@extended` name real ones (`pixelox fonts classes`).

**Fails when.** Round letters turn into ovals with thin sides: the scale is too strong;
choose a narrower or wider font first (typography.md) and scale only the rest of the way.

## T-sliced-type

**Purpose.** A word cut along a diagonal with the halves slipped apart: edgy tech, music,
editorial, "breaking" and "glitch" ideas without the noise.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#f1ede4'
pixelox shape path d.pxo --in-place --name Field --d 'M 0 640 L 1080 500 V 1350 H 0 Z' --fill '#e4002b'
pixelox text add d.pxo --in-place --name Top --content "SLICE" --font @condensed-heavy --fit-width 900 --at 50%,42% --anchor c --anchor-to ink --color '#111111'
pixelox layer dup d.pxo --in-place --layer Top --name Bottom
pixelox mask add d.pxo --in-place --layer Top --from-path 'M 0 0 H 1080 V 500 L 0 640 Z'
pixelox mask add d.pxo --in-place --layer Bottom --from-layer Field
pixelox transform move d.pxo --in-place --layer Bottom --by 46,16
pixelox export d.pxo -o T-sliced-type.png
```

**Parameters.** Two copies of the word carry opposite halves of one cut line as masks: the
upper copy a vector mask (`--from-path`), the lower copy the colored field's own outline
(`--from-layer Field`). The lower copy is then moved 3 to 6% of the width along the cut.
The cut should cross the letters at 5 to 15 degrees and pass through their middle third.
The Field shape is optional: it colors the ground under the lower half for a two-field
look (without it, give Bottom a `--from-path` mask too). For several cuts, repeat with
three copies and two cut lines.

**Fails when.** The word becomes unreadable: the offset is larger than a letter's stroke
width times 3, or the cut runs along the baseline.

## T-ghost-type

**Purpose.** A giant, faint word as atmosphere: behind the subject, bleeding off the
edges, or outlined across the subject. It sets the theme without competing with the
headline.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#0b2a8f'
pixelox text add d.pxo --in-place --name Ghost --content "FRESH" --font @heavy-wide --fit-width 1240 --at 50%,22% --anchor c --anchor-to ink --color '#ffffff'
pixelox layer opacity d.pxo --in-place --layer Ghost --value 12
pixelox text add d.pxo --in-place --name Soft --content "FRESH" --font @heavy-wide --fit-width 1240 --at 50%,50% --anchor c --anchor-to ink --color '#5fa8ff'
pixelox filter gaussian d.pxo --in-place --layer Soft --sigma 12
pixelox layer opacity d.pxo --in-place --layer Soft --value 55
pixelox comp place d.pxo product.png --in-place --as Product --size x1000 --at 50%,96% --anchor b
pixelox text add d.pxo --in-place --name Outline --content "FRESH" --font @heavy-wide --fit-width 1240 --at 50%,80% --anchor c --anchor-to ink --color transparent --stroke '#ffffff' --stroke-width 2
pixelox export d.pxo -o T-ghost-type.png
```

**Parameters.** Three variants, use one: **faint** (the ground's own hue, 8 to 15%
opacity), **defocused** (a lighter tint, blurred 1 to 1.5% of the width, as if out of
focus behind the subject; the blur is live, the word stays text), **outlined over the
subject** (hollow letters, 1 to 2 px, on top). The word is wider than the canvas
(`--fit-width` 110 to 130% of it) so its ends are cut off. It never carries information
the reader needs. A word that should start past the left edge: `--at 0-120,40%`.

**Fails when.** It competes with the real headline: lower its opacity until you notice it
second.

## T-floor-type

**Purpose.** Type lying flat on the ground, receding into the picture: film titles, sport,
street and stage scenes.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#b3121a'
pixelox layer add d.pxo --in-place --name Far
pixelox fill gradient d.pxo --in-place --layer Far --from 0,40% --to 0,100% --stops '#5a070bff@0,#5a070b00@1'
pixelox text add d.pxo --in-place --name Title --content "ON THE LINE" --font @condensed-heavy --size 12% --color '#111111' --at 50%,50% --anchor c
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
pixelox text add d.pxo --in-place --name Light --content "CONTRAST" --font @neo-grotesque --weight 700 --size 13% --color '#f2c230' --at 50%,45% --anchor c
pixelox text add d.pxo --in-place --name Dark --content "CONTRAST" --font @neo-grotesque --weight 700 --size 13% --color '#111111' --at 50%,45% --anchor c
pixelox mask add d.pxo --in-place --layer Dark --from-layer Field
pixelox export d.pxo -o T-counterchange.png
```

**Parameters.** Two identical text layers in the two field colors; the upper one is masked
by the field it sits on (`--from-layer Field`: a live mask that follows the field's
outline, so a circle, a diagonal, or a moved field needs no extra work). The field edge
should cut through letters, not between them.

**Fails when.** The two copies drift apart: both need the same content, font, size, and
`--at`; change them together.

## T-warp-type

**Purpose.** A headline that grows, rises, arches, or waves: hand-lettered show posters,
retro sport, anything that should feel shouted.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#f6e7c8'
pixelox text add d.pxo --in-place --name Grow --content "LOUDER" --font @condensed-heavy --fit-width 860 --at 50%,5% --anchor t --anchor-to ink --color '#c62a1a' --warp rise --bend 40 --h-distort 70
pixelox text add d.pxo --in-place --name Arc --content "EVERY NIGHT" --font @condensed-heavy --size 9% --color '#1d1a16' --at 50%,48% --anchor c
pixelox text warp d.pxo --in-place --layer Arc --preset arc --bend 35
pixelox text add d.pxo --in-place --name Rise --content "ALL SUMMER" --font @condensed-heavy --size 9% --color '#1d1a16' --at 50%,66% --anchor c --warp rise --bend 60
pixelox text add d.pxo --in-place --name Flag --content "FREE ENTRY" --font @condensed-heavy --size 9% --color '#c62a1a' --at 50%,86% --anchor c --warp flag --bend 40
pixelox export d.pxo -o T-warp-type.png
```

**Parameters.** `--warp PRESET --bend N` on `text add`, or `text warp --preset` on an
existing layer; the text stays live and editable. The presets are Photoshop's 15: `arc`,
`arc-lower`, `arc-upper`, `arch`, `bulge`, `shell-lower`, `shell-upper`, `flag`, `wave`,
`fish`, `rise`, `fisheye`, `inflate`, `squeeze`, `twist`. `--bend` -100 to 100 (negative
bends the other way: `rise --bend=-60` falls to the right). `--h-distort 70` adds a
perspective that grows the right side (Grow), `--v-distort` the bottom. The warp fills the
box of the unwarped letters, so a strong warp wants a larger size. Pair a rising line with
a falling one for a zigzag stack. The same presets bend a photo or shape:
`pixelox transform warp d.pxo --in-place --layer Banner --preset flag --bend 30`. Text on a full
circle is T-arc-type.

**Fails when.** The small end is unreadable: keep its cap height above 4% of the canvas.

## T-stencil-type

**Purpose.** Stencil lettering: crates, military, street, industrial.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#d8cfb4'
pixelox text add d.pxo --in-place --name Word --content "CARGO" --font @stencil --fit-width 920 --at 50%,45% --anchor c --anchor-to ink --color '#23301f'
pixelox export d.pxo -o T-stencil-type.png
```

**Parameters.** `--font @stencil` picks an installed stencil face. Most machines have
none: the result then carries warning PXW4010 and the word is drawn in a plain heavy face
**without gaps**. Ask the user whether a free stencil family may be downloaded, then run
`pixelox fonts get @stencil -y` (Allerta Stencil, light and wide) or
`pixelox fonts get "Saira Stencil One" -y` (heavy, the crate look) and name it in `--font`.
With no download, cut the gaps yourself, as below: vertical stripes in the ground color,
clipped to the word, become the stencil bridges.

```bash
pixelox render js scripts/pattern.js -o cuts.png --size 1080x1350 --param kind=stripes --param cell=118 --param weight=0.09 --param color='#d8cfb4'
pixelox new -o d.pxo --size 1080x1350 --fill '#d8cfb4'
pixelox text add d.pxo --in-place --name Word --content "CARGO" --font @condensed-heavy --fit-width 920 --at 50%,45% --anchor c --anchor-to ink --color '#23301f'
pixelox comp place d.pxo cuts.png --in-place --as Cuts --at 0-20,0
pixelox layer clip d.pxo --in-place --layer Cuts
pixelox export d.pxo -o T-stencil-type-cut.png
```

`cell` is the gap spacing (55 to 70% of a letter's width, so every letter gets one or two
cuts) and `weight` 0.07 to 0.12 the gap width. Slide the stripes with the first number of
`--at` (`0-20` is 20 px to the left) until no cut lands between letters. Add
T-distress-type or T-grunge for sprayed paint.

**Fails when.** Cuts slice letters in half visually: gaps too wide; keep them under 10% of
the letter width.

## T-texture-fill

**Purpose.** Type or shapes filled with a generated material (thick paint, marble, crumpled
paper) instead of a flat color.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#16302b'
pixelox text add d.pxo --in-place --name Word --content "ART" --font @heavy-wide --size 30% --color '#e8a23a' --at 50%,40% --anchor c
pixelox fill noise d.pxo --in-place --as-layer --name Paint --seed 12 --scale 20% --octaves 5 --warp 60 --contrast 25 --stops '#b3541e,#e8a23a,#f6e3b4'
pixelox filter oil-paint d.pxo --in-place --layer Paint --impasto 80
pixelox layer clip d.pxo --in-place --layer Paint
pixelox shape ellipse d.pxo --in-place --name Disc --rect 340,860,400x400 --fill '#e8a23a'
pixelox fill noise d.pxo --in-place --as-layer --name Marble --seed 3 --kind ridged --scale 60% --warp 120 --stops '#2b2b30,#f4f1ea'
pixelox layer clip d.pxo --in-place --layer Marble
pixelox export d.pxo -o T-texture-fill.png
```

**Parameters.** A noise layer directly above the word or shape, clipped to it. **Paint**:
three colors, `--contrast` 20 to 40 for hard-edged marbling, then `filter oil-paint
--impasto 60` to `90` for brushed strokes. **Marble**: `--kind ridged` with `--warp` 100
to 150 draws veins. Creased paper: `--kind ridged --scale 30% --octaves 4` with two
near-whites (`--stops '#d9d4c8,#fbf8f0'`). Stone or scales: `--kind cells`; cracked glaze:
`--kind cell-edges`. Each clipped layer attaches to the layer right below it.

**Fails when.** The word loses its edge: the texture's darkest color matches the ground;
keep all the texture colors clearly lighter or darker than the ground.

## T-crack

**Purpose.** A jagged split through a word, a face, or a silhouette: thrillers, breakups,
"broken" concepts, lightning.

```bash
pixelox render js scripts/crack.js -o crack.png --size 1080x1350 --seed 5 --param color='#f1ede4' --param from=0.56,0.12 --param to=0.47,0.9 --param width=0.03 --param steps=16
pixelox new -o d.pxo --size 1080x1350 --fill '#f1ede4'
pixelox text add d.pxo --in-place --name Word --content "SPLIT" --font @condensed-heavy --fit-width 940 --at 50%,50% --anchor c --anchor-to ink --color '#161616'
pixelox comp place d.pxo crack.png --in-place --as Crack --fit cover
pixelox export d.pxo -o T-crack.png
```

**Parameters.** The crack is drawn in the ground color on top of the object, so it reads
as a gap. `from` / `to` are canvas fractions; `width` 0.015 to 0.04; `jag` 0.3 to 0.8;
`branches` 0 to 5. Render it at canvas size. For lightning, use a pale color on a dark sky
and add `fx outer-glow` to the layer (or T-difference-clouds for a whole sky of it).

**Fails when.** It looks like a scratch: too thin for the object; the widest point should
be 3 to 6% of the object's width.

## T-object-as-letter

**Purpose.** The product or an object stands in for one letter of the headline word. The
word and the picture become one thing.

```bash
pixelox transform trim product.png -o tight.png
pixelox new -o d.pxo --size 1080x1350 --fill '#bfe3f2'
pixelox text add d.pxo --in-place --name Left --content "CH" --font @geometric-sans --weight 500 --size 22% --color '#ffffff' --at 43%,45% --anchor r
pixelox text add d.pxo --in-place --name Right --content "LL" --font @geometric-sans --weight 500 --size 22% --color '#ffffff' --at 57%,45% --anchor l
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
pixelox text add d.pxo --in-place --name OutlineText --content "JOIN THE EXPO" --font @geometric-sans --weight 500 --size 2.4% --tracking 80 --color '#ffffff' --at 515,597 --anchor c
pixelox shape arrow d.pxo --in-place --name Arrow --from 668,597 --to 712,597 --stroke '#ffffff' --stroke-width 2 --head-size 12
pixelox shape rect d.pxo --in-place --name Solid --rect 370,700,340x84 --radius 42 --fill '#ffd400'
pixelox text add d.pxo --in-place --name SolidText --content "Shop Now" --font @geometric-sans --weight 700 --size 3.4% --color '#0a0a1f' --at 540,742 --anchor c
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
pixelox fx extrude d.pxo --in-place --layer Burst --depth 22 --angle 300
pixelox fx drop-shadow d.pxo --in-place --layer Burst --offset 0,14 --blur 28 --opacity 30
pixelox text add d.pxo --in-place --name Price --content "\$3.99" --font @heavy-wide --size 7% --color '#ffffff' --at 62%,55% --anchor c
pixelox transform rotate d.pxo --in-place --layer Price --degrees -6
pixelox fx long-shadow d.pxo --in-place --layer Price --length 30 --color '#7a1d05' --blend normal --opacity 100
pixelox export d.pxo -o T-price-burst.png
```

**Parameters.** `--points` 16 to 28; `--inner` 78 to 85% of `--outer` for short teeth
(sticker), 55% for a spiky comic burst. Diameter 30 to 40% of the canvas width. `fx
extrude` gives the sticker a thick edge (its sides are the fill color, darkened); the short
`fx long-shadow` presses the price into it. Leave both out for a flat sticker. It should
overlap the product's edge by a quarter of its width and be tilted 5 to 12 degrees. The
price is the largest text after the headline. Write a dollar sign as `\$` inside double
quotes.

**Fails when.** It floats alone in a corner: let it overlap the hero.

## T-info-bar

**Purpose.** A horizontal strip that carries date, time, and place in two or three columns,
each led by an icon: the bottom band of most event flyers.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#5a0c0a'
pixelox shape rect d.pxo --in-place --name Bar --rect 6%,81%,88%x11% --fill '#1c0505' --stroke '#ffffff55' --stroke-width 1
pixelox fx overlay d.pxo --in-place --layer Bar --stops '#000000@0,#7a0f0c@1' --angle 0
pixelox shape icon d.pxo calendar --in-place --name IconDate --size 48 --at 9%,86.5% --anchor l --color '#ffd23c'
pixelox text add d.pxo --in-place --name Date --content $'3RD - 23RD\nFEB. 2027' --font @neo-grotesque --weight 700 --size 3% --leading 100% --color '#ffffff' --at 15%,86.5% --anchor l
pixelox shape line d.pxo --in-place --name Sep1 --from 37.5%,83% --to 37.5%,90% --stroke '#ffffff' --stroke-width 2 --opacity 50
pixelox shape icon d.pxo clock --in-place --name IconTime --size 48 --at 40%,86.5% --anchor l --color '#ffd23c'
pixelox text add d.pxo --in-place --name Time --content "6PM" --font @neo-grotesque --weight 700 --size 4.4% --color '#ffffff' --at 46%,86.5% --anchor l
pixelox shape line d.pxo --in-place --name Sep2 --from 59%,83% --to 59%,90% --stroke '#ffffff' --stroke-width 2 --opacity 50
pixelox shape icon d.pxo map-pin --in-place --name IconPlace --size 48 --at 61.5%,86.5% --anchor l --color '#ffd23c'
pixelox text add d.pxo --in-place --name Place --content $'123 ANYWHERE ST.\nANY CITY 12345' --font @neo-grotesque --weight 700 --size 2% --leading 115% --color '#ffffff' --at 67.5%,86.5% --anchor l
pixelox export d.pxo -o T-info-bar.png
```

**Parameters.** Bar: 82 to 92% of the width, 9 to 13% of the height, sitting 6 to 10% above
the bottom edge; darker than the ground, optionally with a left-to-right gradient (`fx
overlay` keeps the bar's thin stroke). Three columns in the ratio 1.2 : 0.8 : 1.5 (date,
time, place), split by thin vertical rules at 50% opacity. Each column starts with an
icon: `shape icon NAME --size 48` (sizes that are multiples of 24 are pixel-sharp), in the
accent color, with the text 1.2 icon widths to its right, both on one center line
(`--anchor l` at the same Y). Usual names: `calendar`, `clock`, `map-pin`, `ticket`,
`phone`, `globe`, `mail`, `music`, `mic`, `users`; find others with
`pixelox shape icons --search WORD`. Text sizes 2 to 4.5% of the canvas height: the time
may be the largest. `$'line one\nline two'` makes a two-line layer (a plain "\n" inside
double quotes does not). A subject cutout may overlap the bar's top edge.

**Fails when.** Lines touch the bar's edge or the next column's rule: keep padding of at
least 25% of the bar height on the left and right, and shrink the longest text (usually
the place) until it clears.

## T-icon-row

**Purpose.** A row of icons with short labels (features, amenities, steps), and small icons
in front of contact lines. Icons are bundled outline drawings; no image files needed.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#0f3d3e'
pixelox shape ellipse d.pxo --in-place --name Dot1 --rect 150,560,180x180 --fill '#e2f1af'
pixelox shape icon d.pxo wifi --in-place --name I1 --size 96 --at 240,650 --anchor c --color '#0f3d3e'
pixelox text add d.pxo --in-place --name L1 --content "FREE WI-FI" --font @geometric-sans --weight 700 --size 1.8% --tracking 120 --color '#e2f1af' --at 240,790 --anchor c
pixelox shape ellipse d.pxo --in-place --name Dot2 --rect 450,560,180x180 --fill '#e2f1af'
pixelox shape icon d.pxo coffee --in-place --name I2 --size 96 --at 540,650 --anchor c --color '#0f3d3e'
pixelox text add d.pxo --in-place --name L2 --content "OPEN LATE" --font @geometric-sans --weight 700 --size 1.8% --tracking 120 --color '#e2f1af' --at 540,790 --anchor c
pixelox shape ellipse d.pxo --in-place --name Dot3 --rect 750,560,180x180 --fill '#e2f1af'
pixelox shape icon d.pxo music --in-place --name I3 --size 96 --at 840,650 --anchor c --color '#0f3d3e'
pixelox text add d.pxo --in-place --name L3 --content "LIVE MUSIC" --font @geometric-sans --weight 700 --size 1.8% --tracking 120 --color '#e2f1af' --at 840,790 --anchor c
pixelox shape icon d.pxo phone --in-place --name C1 --size 48 --at 8%,93% --anchor l --color '#e2f1af'
pixelox text add d.pxo --in-place --name C1T --content "555 0100" --font @geometric-sans --weight 500 --size 2% --color '#e2f1af' --at 13.5%,93% --anchor l
pixelox shape icon d.pxo globe --in-place --name C2 --size 48 --at 40%,93% --anchor l --color '#e2f1af'
pixelox text add d.pxo --in-place --name C2T --content "www.example.com" --font @geometric-sans --weight 500 --size 2% --color '#e2f1af' --at 45.5%,93% --anchor l
pixelox export d.pxo -o T-icon-row.png
```

**Parameters.** Find an icon by any plain word: `pixelox shape icons --search coffee`
(about 2,100 icons, all in one outline style, no brand logos). Row icons: 72 to 120 px on a
disc 1.8 to 2x their size, in the two main colors swapped; 3 to 5 in a row, equal gaps, one
label of 1 to 2 words under each. Contact icons: the cap height of the text x 1.5 to 2
(48 px here). `--stroke-width` sets the line weight (default size / 12); thicken small
icons to match bold text (`--stroke-width 5` at 48 px). `--fill` fills closed outlines for
solid hearts, stars, and pins. Icons are shapes: recolor with `shape set --stroke`, add
`fx` styles, rotate, scale.

**Fails when.** Mixed icon styles or sizes in one row: use the bundled set only, one size,
one color. Social-network and brand logos are not in the set and must come from the user.

## T-qr-code

**Purpose.** A scannable code to tickets, a menu, or a sign-up page, with a short call to
action beside it.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#12233f'
pixelox text add d.pxo --in-place --name CTA --content $'SCAN FOR\nTICKETS' --font @condensed-heavy --size 7% --leading 95% --color '#ffffff' --at 8%,84% --anchor l
pixelox shape icon d.pxo arrow-right --in-place --name Arrow --size 96 --at 52%,84% --anchor c --color '#ffd23c'
pixelox shape qr d.pxo --in-place --name Tickets --data 'www.example.com/tickets' --size 260 --background-color white --radius 16 --at 92%,84% --anchor r
pixelox export d.pxo -o T-qr-code.png
```

**Parameters.** `--data` is the exact link or text the user gives, scheme included (ask
for it; never invent a live address; the placeholder here is not one). `--size` includes the quiet zone: 20 to 28% of the width on a social post, at
least 25 mm in print. Dark modules on a light plate always: on a dark or busy ground add
`--background-color white` (with `--radius` for rounded corners), which makes the code a
group that moves as one. `--style rounded` or `dots` restyles the modules; `--color` takes
a dark brand color. The code is decoded back before it is added, and warning PXW6010 means
the contrast is too low to scan. `--logo-gap 22` clears the middle for a logo (place the
logo at the `logo_box` in the result). Always put a reason to scan next to it.

**Fails when.** It will not scan: light modules on dark, a plate with no quiet zone, or a
code under 2 cm in print. Keep the module color darker than 40% lightness.

## T-barcode

**Purpose.** A real barcode on a coupon, ticket, tag, or receipt-style design, or as an
honest "product label" detail.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#f1ede4'
pixelox shape barcode d.pxo --in-place --name Serial --type code128 --data 'TKT-2027-0042' --width 46% --height 70 --at 50%,40% --anchor c
pixelox shape rect d.pxo --in-place --name Coupon --rect 10%,58%,80%x30% --radius 24 --fill '#ffffff' --stroke '#111111' --stroke-width 3 --dash 14,10
pixelox text add d.pxo --in-place --name Offer --content "20% OFF" --font @condensed-heavy --size 9.5% --color '#e4002b' --at 15%,68% --anchor l
pixelox text add d.pxo --in-place --name Terms --content "ONE PER VISIT. ENDS 30 JUNE." --font @neo-grotesque --weight 500 --size 1.6% --tracking 80 --color '#111111' --at 15%,80% --anchor l
pixelox shape barcode d.pxo --in-place --name Code --type ean13 --data 400638133393 --width 250 --at 86%,73% --anchor r
pixelox export d.pxo -o T-barcode.png
```

**Parameters.** `--type code128` encodes any short text (ticket numbers, order codes);
`ean13`, `ean8`, and `upca` take digits without the check digit (it is added, and a wrong
one is refused); `code39` is capitals and digits. `--width` is the whole symbol with its
quiet zones; `--height` the bars; `--text off` hides the number line. Black on white or a
pale ground only: add `--background-color white` anywhere else. Use real data from the
user when the code has to work; as decoration, encode something harmless such as the
event date.

**Fails when.** Bars blur together: the symbol is too narrow; keep an EAN at 220 px or
more on a 1080 px canvas.

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
`--color` tints the whole flare (glow, streak, halo, and ghosts; the core stays white). The flare is its own `Lens Flare` layer in screen mode: lower it with
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
pixelox text add d.pxo --in-place --name Quote --content "A BOLD, FUNNY, FEARLESS DEBUT" --font @condensed --weight 600 --size 1.5% --tracking 120 --color '#e9e6df' --at 50%,10.4% --anchor c
pixelox text add d.pxo --in-place --name Title --content "THE LONG WAY" --font @condensed-heavy --size 11% --color '#ffffff' --at 50%,72% --anchor c
pixelox text add d.pxo --in-place --name Tagline --content "SOME ROADS CHOOSE YOU" --font @condensed --weight 500 --size 1.9% --tracking 260 --color '#e9e6df' --at 50%,79.5% --anchor c
pixelox text add d.pxo --in-place --name Credits --content $'NORTH LIGHT PICTURES presents a QUIET HARBOR production  a film by LEE VARGA\ncasting by SAM OKORO  music by THE NIGHT OWLS  costume design JO MARSH  editor PIA LUND\nproduced by ANA REYES and TOM HALE  written and directed by LEE VARGA' --font @condensed --weight 500 --size 1.25% --tracking 60 --leading 135% --align center --color '#8f8c86' --at 50%,87.5% --anchor c
pixelox text add d.pxo --in-place --name Date --content "IN CINEMAS THIS AUTUMN" --font @condensed --weight 700 --size 1.6% --tracking 300 --color '#f2c230' --at 50%,94% --anchor c
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
pixelox text add d.pxo --in-place --name Title --content "HORIZON" --font @condensed --weight 700 --size 12% --tracking 200 --color '#ffffff' --at 50%,80% --anchor c
pixelox layer stamp d.pxo --in-place --name Grade
pixelox adjust clarity d.pxo --in-place --layer Grade --amount 30
pixelox adjust curves d.pxo --in-place --layer Grade --points '0,6 60,50 190,205 255,250'
pixelox filter grain d.pxo --in-place --as-layer --name Grain --amount 20
pixelox export d.pxo -o T-final-grade.png
```

**Parameters.** `layer stamp` merges everything visible into a new top layer and leaves the
layers below intact (delete the stamp to keep editing). Clarity 15 to 40 adds midtone punch
(negative values give a soft glow). The curve here is a gentle S with lifted blacks.
Lower the stamp's opacity to 50 to 80 to dial the grade back. The grain layer goes on top
(T-paper-grain). Do this last, then export.

**Fails when.** Small text gets crunchy: use clarity under 25, or stamp before adding the
small text.

