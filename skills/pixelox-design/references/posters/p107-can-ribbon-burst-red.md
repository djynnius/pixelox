---
id: p107
title: Red can ad with ribbon burst
genre: [drink-ad, product-ad]
mood: [festive, energetic, warm, bold]
style: [photo-composite, illustration]
format: portrait-2x3
archetype: A-hero-object
palette: ['#b82124', '#90191b', '#e0655a', '#ffffff', '#f6a35c']
techniques: [T-spotlight, T-vignette, T-cutout-drop-shadow]
needs: product-cutout
---

# p107: Red can ad with ribbon burst

## 1. Concept

A soft-drink ad with no copy at all: an open cola can stands in a pool of light on a red
ground, and a plume of white, cream, and orange ribbons with sparks shoots out of its
mouth to the top edge like fireworks; the `[BRAND]` script wordmark sits small at the
bottom. It works because the whole poster is one color (the brand red) and one gesture:
"open it and a celebration comes out". The can occupies the calm lower half, the burst
fills the upper half, and nothing else competes.

## 2. Format and grid

* Portrait 2:3. One center axis at 50% W: can, floor light, and wordmark are centered on
  it. The burst leans off the axis to the upper right, which gives the still layout its
  movement.
* Zones: burst 0 to 40% H (it fans from 13 to 90% W at the top edge down to a point at
  the can's mouth, 52% W, 38% H); can 36.5 to 70.5% H; floor light 61 to 78% H; empty red
  78 to 82% H; wordmark 82 to 87% H; empty red below.
* The can is small for a product ad: 30% W wide, 34% H tall. The burst is three times its
  area. Side margins of plain red: 22% W left and right of the floor light.

## 3. Layer stack

1. Ground: brand red `#b82124`, full canvas.
2. Back glow: a soft radial light behind the can, center 50% W, 47% H, radius about 45% H,
   salmon `#e0655a` at the center fading into the ground.
3. Corner shade: darker red `#90191b` toward all four corners (radial, starting at 50% of
   the radius).
4. Floor light: a pale peach ellipse (`#e2a08a` at its center), `22%,61%,56% x 17%`,
   with a very soft edge that takes a third of its radius: the "stage" the can stands
   on.
5. Contact shadow: a thin dark-red ellipse `#5a0c0e`, `32%,69.6%,38% x 1.6%`, soft edge,
   directly under the can.
6. Can: a red aluminum can with a white script wordmark running up its side, seen
   slightly from above so the open silver top shows; `34.5%,36.5%,30% x 34%`, tilted
   about 4 degrees.
7. Ribbons: 12 to 15 tapered streamers that start as hairlines at the can's mouth and end
   in round heads 3 to 4% W thick. They rise and curve, most toward the upper right, two
   long thin arcs toward the upper left corner. Colors: white, cream `#fde6c4`, orange
   `#f6a35c`, and dark red `#8e1216` (the dark ones give depth). A soft white glow
   surrounds them.
8. Mouth glow: a white radial flare, center 52% W, 38.5% H, radius 11% W, hiding where
   the ribbons leave the can.
9. Sparks: about 150 tiny white and cream dots clustered around the burst (center 58% W,
   18% H), and 10 to 15 small four-point sparkles and fine firework bursts near the top
   (27%,3%; 52%,17%; 72%,10% W,H).
10. Wordmark: white script, 22.8% W wide, 4.9% H tall, centered at 50% W, 84.5% H.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#b82124` | brand red ground | 55% |
| `#90191b` | dark red corners, dark ribbons | 15% |
| `#e0655a` | salmon back glow | 15% |
| `#e2a08a` | peach floor light | 5% |
| `#ffffff` | ribbons, sparks, wordmark, mouth glow | 7% |
| `#f6a35c` | orange and cream ribbons | 3% |

Monochrome red in four lightness steps, with white as the only contrast and a small
amount of orange to warm the burst. No black, no gray, no second hue.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Wordmark | `[BRAND]` script wordmark, 2 short words | title | flowing formal script with heavy strokes (Lobster, Pacifico, Great Vibes / Snell Roundhand 900) | 900 | 2.6% (whole mark 4.9% H tall) | 0 | - | center | `#ffffff` | 50%, 84.5% |

There is no headline, no tagline, no price. The wordmark is the only type and it is
small: 23% of the width. In a build with a real brand, place the logo file instead of
setting text.

## 6. Imagery

One product photograph, cut out: a standard drinks can, opened, seen from slightly above
so the silver lid and the dark opening are visible, lit from the upper left with a broad
soft highlight down its left side. It leans a few degrees, as if caught mid-pop. The can
is the same red as the ground, so its outline is carried by the highlight, the silver
rim, and the contact shadow (**T-spotlight**: a glow behind, a pool of light and a
shadow below). The burst is drawn, not photographed: flat vector streamers with round
heads and needle tails, plus glows and sparkles. Any product that opens works the same
way: a bottle, a jar, a gift box, a perfume, a paint tin.

## 7. Effects and textures

* **T-spotlight**: back glow radius 45% H in `#e0655a`; the floor light and the contact
  shadow are two `shape ellipse` layers blurred with `filter gaussian` (sigma 40 and 5).
  The blur is live and spreads past the shape's bounds.
* **T-vignette** in dark red (`#5a0a0c` at 60% in the corners), never black.
* Ribbons: `paint stroke` with `--d` (one SVG subpath per streamer), `--hardness 100`,
  and `--taper 55%`, so each stroke grows from a hairline at the mouth to a full round
  head over the first half of its length. One command per color, brush sizes 30 to 40 px
  (dark red thinnest, white thickest), dark colors painted first. Each path is an S: its
  first control point sits above the mouth, its second far out to the side.
  `fx outer-glow` (blur 22, white, 40%) on the ribbon layer.
* Firework bursts: three `shape star` layers with 26 to 30 points and an inner radius of
  5 px (needle-thin spikes), pale pink at 55% opacity, behind the ribbons.
* Mouth glow: a radial `fill gradient` from opaque white to transparent peach.
* Sparks: `scripts/particles.js`, `shape=glow` (190 dots) clustered with `focus`, and
  `shape=star` (22 small sparkles), both in screen mode.
* **T-cutout-drop-shadow** is not used as a drop shadow: the can only has the contact
  shadow on the floor.

## 8. Hierarchy and eye path

1. The white mouth glow and the ribbons leaving it (the brightest area, upper middle).
2. Up and right along the ribbons to the sparks at the top edge.
3. Back down to the can (the darkest, sharpest object) and the wordmark printed on it.
4. The small wordmark at the bottom, as a signature.

There is one type size. The visual hierarchy is by area: burst about 35% of the canvas,
can 10%, wordmark 1%.

## 9. Build recipe

`product.png` is your image here: a can, bottle, or box cutout, 34% of the canvas height.
The `Wordmark` text is a placeholder for a logo: with a logo file, use `comp place` with
`--size 246x --at 50%,84.5% --anchor c` instead of the three wordmark lines.

```bash
pixelox render js scripts/particles.js -o dots.png --size 1080x1620 --seed 7 --param shape=glow --param colors='#ffffff,#ffe2c4' --param count=190 --param min=0.15 --param max=0.7 --param focus=0.56,0.18,0.3
pixelox render js scripts/particles.js -o stars.png --size 1080x1620 --seed 3 --param shape=star --param colors='#ffffff,#ffd9b0' --param count=22 --param min=0.6 --param max=2.2 --param focus=0.58,0.14,0.34
pixelox new -o can.pxo --size 1080x1620 --fill '#b82124'
pixelox layer add can.pxo --in-place --name Glow
pixelox fill gradient can.pxo --in-place --layer Glow --kind radial --from 50%,47% --to 50%,92% --stops '#e97a6cff@0,#d9534a99@0.4,#b8212400@1'
pixelox layer add can.pxo --in-place --name Corners
pixelox fill gradient can.pxo --in-place --layer Corners --kind radial --from 50%,47% --to 50%,125% --stops '#5a0a0c00@0.5,#5a0a0c99@1'
pixelox shape ellipse can.pxo --in-place --name Floor --rect 22%,62%,56%x15% --fill '#eaa692'
pixelox filter gaussian can.pxo --in-place --layer Floor --sigma 40
pixelox shape ellipse can.pxo --in-place --name Contact --rect 33%,69.5%,34%x1.8% --fill '#5a0c0e'
pixelox filter gaussian can.pxo --in-place --layer Contact --sigma 5
pixelox shape star can.pxo --in-place --name Firework1 --center 22%,4% --points 30 --outer 80 --inner 5 --fill '#ffd9d0' --opacity 55
pixelox shape star can.pxo --in-place --name Firework2 --center 68%,11% --points 30 --outer 95 --inner 5 --fill '#ffd9d0' --opacity 55
pixelox shape star can.pxo --in-place --name Firework3 --center 47%,17% --points 26 --outer 60 --inner 4 --fill '#ffd9d0' --opacity 55
pixelox comp place can.pxo product.png --in-place --as Product --size x560 --at 50%,70.5% --anchor b
pixelox transform rotate can.pxo --in-place --layer Product --degrees -4
pixelox layer add can.pxo --in-place --name Ribbons
pixelox paint stroke can.pxo --in-place --layer Ribbons --color '#8e1216' --size 30 --hardness 100 --spacing 4 --taper 55% --d 'M 534 604 C 520 380 330 80 151 0 M 536 602 C 500 510 580 400 567 308 M 540 604 C 560 540 780 500 788 356 M 534 602 C 540 460 420 300 421 162'
pixelox paint stroke can.pxo --in-place --layer Ribbons --color '#f6a35c' --size 36 --hardness 100 --spacing 4 --taper 55% --d 'M 540 602 C 520 480 760 430 734 227 M 534 604 C 540 470 380 330 351 194 M 536 602 C 560 430 420 200 367 -16 M 542 604 C 600 470 900 330 864 49'
pixelox paint stroke can.pxo --in-place --layer Ribbons --color '#fde6c4' --size 34 --hardness 100 --spacing 4 --taper 55% --d 'M 536 602 C 490 500 560 330 508 235 M 540 604 C 560 560 630 510 616 437'
pixelox paint stroke can.pxo --in-place --layer Ribbons --color '#ffffff' --size 40 --hardness 100 --spacing 4 --taper 55% --d 'M 534 604 C 560 520 440 400 454 275 M 538 602 C 480 440 600 200 508 -16 M 538 604 C 500 470 660 330 648 113 M 542 604 C 560 440 830 330 810 16 M 534 602 C 520 400 400 100 259 49'
pixelox fx outer-glow can.pxo --in-place --layer Ribbons --blur 22 --color '#ffffff' --opacity 40
pixelox layer add can.pxo --in-place --name MouthGlow
pixelox fill gradient can.pxo --in-place --layer MouthGlow --kind radial --from 49.6%,37.3% --to 49.6%,46% --stops '#ffffffff@0,#ffe9dcb3@0.4,#ffd0c000@1'
pixelox comp place can.pxo dots.png --in-place --as Dots --fit cover
pixelox layer blend can.pxo --in-place --layer Dots --mode screen
pixelox comp place can.pxo stars.png --in-place --as Stars --fit cover
pixelox layer blend can.pxo --in-place --layer Stars --mode screen
pixelox text add can.pxo --in-place --name Wordmark --content "Your Brand" --font @script-formal --weight 900 --fit-width 250 --color '#ffffff' --at 50%,84.5% --anchor c --anchor-to ink
pixelox fx stroke can.pxo --in-place --layer Wordmark --width 2 --color '#ffffff' --position outside
pixelox export can.pxo -o can.png
```

Change first: the ground red (then derive the glow as a lighter, warmer tint and the
corners as a darker shade of the same hue), then the ribbon colors. Every ribbon path
starts within 15 px of the product's opening (here about 540,598): after moving or
resizing the product, shift all `M` points and the `MouthGlow` center to the new opening.

## 10. Variations and pairings

* Other brand colors: green `#0f7a3a` with lime and white ribbons; blue `#1246a8` with
  cyan and silver; black `#0d0d0f` with gold `#f2c14e` ribbons for a premium edition.
  Keep one hue for ground, product, and glow.
* What comes out can change the message: music notes and sound waves (T-waves in line
  mode), fruit slices, snow and stars for a seasonal edition (particles with
  `shape=star`), or flames (`fx fire` on the product).
* Landscape banner: can at 30% W, burst sweeping to the right across the width,
  wordmark at the lower right.
* Add one line of copy between the floor light and the wordmark (cap height 2% H, white,
  tracked caps) when a campaign line is needed; never above the burst.
* Pairs with T-bokeh (far, faint discs behind the burst), T-lens-flare at the mouth, and
  T-paper-grain at amount 8 to hide banding in the red gradients.

## 11. Pitfalls

* A black vignette or a gray shadow on red: it looks dirty. Use darker reds
  (`#5a0a0c`, `#5a0c0e`) for every shade.
* Ribbons of equal width from end to end look like spaghetti, and hairline ribbons look
  like sparkler wire. They must taper to a point at the opening (`--taper 55%`), be 3 to
  4% W thick at the head, and end round.
* Ribbons spread evenly in all directions read as a hairbrush. Lean the whole plume to
  one side and vary lengths by at least 2x; let three or four leave the top edge.
* A product as big as in a normal product ad. Keep it near one third of the height: the
  burst is the hero, and it needs the upper half.
* The product melting into the ground when both are the same color. Keep the back glow,
  the pale floor ellipse, and the dark contact shadow; they draw its outline.
* Rebuilding a known brand's trade dress. A white script wordmark and ribbon on this exact red
  is one company's signature. Take the idea (what the object, the mark, and the ground do
  to each other) and build it with your own brand's colors, lettering, and shapes.
