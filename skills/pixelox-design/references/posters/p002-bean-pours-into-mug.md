---
id: p002
title: Giant bean pours into red mug
genre: [drink-ad, product-ad]
mood: [warm, minimal, playful, bold]
style: [photo-composite, minimal]
format: portrait-4x5
archetype: A-hero-object
palette: ['#b61e1f', '#c64f4e', '#a81515', '#40190d', '#ffffff']
techniques: [T-spotlight, T-vertical-type, T-cutout-drop-shadow, T-paper-grain]
needs: product-cutout
---

# p002: Giant bean pours into red mug

## 1. Concept

A coffee ad with no headline: a red mug stands on a red studio floor, and an oversized
roasted coffee bean floats above it, tipped like a jug, pouring a thin stream of coffee
straight into the cup. One visual metaphor ("this cup comes directly from the bean") does
the whole job, and the red-on-red staging makes the brown bean and the white brand mark the
only two things that are not red.

## 2. Format and grid

* Portrait 4:5. No text margins are needed; the picture is the layout.
* One vertical axis at 47% W: the stream, the middle of the coffee surface, and the mug
  body all sit on it. The bean hangs to the right of the axis, the mug handle too, so the
  composition leans right and the left 28% W stays empty.
* Zones: empty red air 0 to 12% H; bean 12 to 29% H; stream 25 to 53% H; mug 48 to 89% H;
  floor 89 to 100% H.
* Ratio of the two objects: bean 31% W wide, mug body 38% W wide (47% W with the handle).
  The bean is deliberately almost as wide as the mug.

## 3. Layer stack

1. Ground: vertical gradient, darker red `#a81515` at the top to lighter, slightly pink red
   `#c64f4e` at the bottom (the floor catches the light). No horizon line.
2. Floor glow: soft lighter patch `#cf6a68` at 60 to 100% W, 70 to 95% H, lower right.
3. Cast shadow: soft dark red `#8a2a2a` blur on the floor to the lower left of the mug,
   15 to 45% W, 84 to 90% H.
4. Mug: glossy red ceramic mug `29%,48%,47% x 41%`, handle on the right, seen slightly
   from above so the dark coffee surface shows as an ellipse at 49 to 54% H. Lighter red
   on its right side `#e4665a`, deep red `#9e030c` on the left.
5. Brand mark: white wordmark printed on the mug, rotated 90 degrees (reading bottom to
   top), about 25% H long, at 56% W.
6. Stream: a thin brown-black ribbon of coffee, 1.5% W wide, falling from the bean's left
   tip at 49% W, 26% H in a slight S-curve to the cup at 47% W, 53% H. Small ripple rings
   where it lands.
7. Bean: one roasted coffee bean, `47%,12%,33% x 16%`, long axis tilted 12 degrees up to
   the right, crease facing the viewer, glossy highlight along the top. Dark brown
   `#40190d` to mid brown `#6b3b25`.
8. Fine grain over everything (barely visible): one live grain layer on top.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#b61e1f` | mid red of the ground and the mug | 55% |
| `#c64f4e` | lighter floor red, lower third | 25% |
| `#a81515` | dark red at the top edge and corners | 10% |
| `#40190d` | bean, stream, coffee surface | 8% |
| `#ffffff` | brand mark, rim highlight | 2% |

Monochrome red with one natural brown and a touch of white. The product and the ground
share a hue, so shape and light, not color, separate them.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Brand mark on the product | `[BRAND]`, one word of 6 to 8 letters | caps | heavy humanist sans (Archivo Black, Montserrat 800 / Avenir Next) | 800 | 3.2% | 20 | - | rotated -90, reads upward | `#ffffff` | centered on 56% W, 70% H |

There is no headline, no tagline, and no footer. The only type is the mark on the mug, and
turning it on its side keeps it from competing with the picture.

## 6. Imagery

Two photographed objects on one seamless red backdrop, lit softly from the upper right:

* A plain red mug, three-quarter view from slightly above, full of black coffee.
* A single coffee bean enlarged to the width of the mug, sharp, with a wet-looking
  highlight, hanging in the air above and to the right of the cup.

The stream is the join between them: it leaves the lowest tip of the bean and enters the
exact center of the coffee surface. For a build, the mug is a product cutout placed on a
T-spotlight ground with a soft floor shadow (T-cutout-drop-shadow, soft variant); the bean
can be a second cutout, or drawn as in the recipe (an ellipse with a brown gradient and a
dark crease).

## 7. Effects and textures

* **T-spotlight**, reduced to a top-to-bottom gradient plus one off-center floor glow; the
  glow color is the ground red lightened, never white.
* Soft cast shadow: a blurred dark red ellipse, sigma 22, offset to the lower left
  (the light comes from the upper right).
* **T-vertical-type** for the brand mark on the product.
* Gradient and highlight on the bean (`fx overlay` at 60 degrees, plus a blurred light
  ellipse clipped to it) so it reads as a glossy solid.
* **T-paper-grain** (`filter grain --as-layer --amount 8`): only to stop banding in the red
  gradient.

## 8. Hierarchy and eye path

1. The bean: the darkest, most detailed shape, alone in the upper right.
2. Down the stream (a literal line for the eye to follow).
3. The mug and its white mark, where the eye stops.

There is no type hierarchy. Size ratio of the objects: bean width 0.8 of the mug body
width; the stream is 1/25 of the mug width.

## 9. Build recipe

`product.png` is your image here: a mug or cup cutout, about as wide as it is tall. If your
mug already carries a printed logo, delete the `Brand` lines.

```bash
pixelox new -o ad.pxo --size 1080x1350 --fill '#b61e1f'
pixelox layer add ad.pxo --in-place --name Ground
pixelox fill gradient ad.pxo --in-place --layer Ground --from 0,0 --to 0,100% --stops '#a81515@0,#b61e1f@0.45,#c64f4e@1'
pixelox layer add ad.pxo --in-place --name FloorGlow
pixelox fill gradient ad.pxo --in-place --layer FloorGlow --kind radial --from 78%,84% --to 78%,112% --stops '#d47270cc@0,#d4727000@1'
pixelox shape ellipse ad.pxo --in-place --name Shadow --rect 14%,83.5%,40%x6% --fill '#7a1a1c'
pixelox filter gaussian ad.pxo --in-place --layer Shadow --sigma 22
pixelox layer opacity ad.pxo --in-place --layer Shadow --value 75
pixelox comp place ad.pxo product.png --in-place --as Mug --size x560 --at 47.5%,89.5% --anchor b
pixelox text add ad.pxo --in-place --name Brand --content "BRAND" --font @geometric-sans --weight 800 --size 4.4% --tracking 20 --color '#ffffff' --at 50%,50% --anchor c
pixelox transform rotate ad.pxo --in-place --layer Brand --degrees -90
pixelox transform move ad.pxo --in-place --layer Brand --to 51%,70% --anchor c
pixelox shape path ad.pxo --in-place --name Stream --d 'M 522 345 C 500 400 512 470 506 560 C 502 620 506 680 506 716 L 520 716 C 520 680 518 620 522 560 C 528 470 520 400 546 352 Z' --fill '#2c120b'
pixelox shape ellipse ad.pxo --in-place --name Ripple --rect 478,706,70x18 --fill none --stroke '#6b3b25' --stroke-width 3
pixelox shape ellipse ad.pxo --in-place --name Bean --rect 508,162,356x216 --fill '#40190d'
pixelox fx overlay ad.pxo --in-place --layer Bean --stops '#2c120b@0,#40190d@0.4,#8a5433@1' --angle 60
pixelox shape ellipse ad.pxo --in-place --name Gloss --rect 560,176,240x60 --fill '#c99367'
pixelox filter gaussian ad.pxo --in-place --layer Gloss --sigma 16
pixelox layer clip ad.pxo --in-place --layer Gloss
pixelox layer opacity ad.pxo --in-place --layer Gloss --value 60
pixelox shape path ad.pxo --in-place --name Crease --d 'M 524 288 C 600 250 700 300 852 262' --fill none --stroke '#1a0905' --stroke-width 12 --cap round
pixelox transform rotate ad.pxo --in-place --layer Bean --degrees -12 --around 686,270
pixelox transform rotate ad.pxo --in-place --layer Gloss --degrees -12 --around 686,270
pixelox transform rotate ad.pxo --in-place --layer Crease --degrees -12 --around 686,270
pixelox fx drop-shadow ad.pxo --in-place --layer Bean --offset -10,18 --blur 40 --color '#5a0a0c' --opacity 45
pixelox filter grain ad.pxo --in-place --as-layer --name Grain --amount 8
pixelox export ad.pxo -o ad.png
```

Change first: the ground hue (keep the top-dark, bottom-light ramp), then the `Stream`
path so it ends exactly on your cup's opening (its last points are at x 506 to 520, y 716),
then swap the drawn bean for a real cutout of the ingredient.

## 10. Variations and pairings

* Any "ingredient becomes the drink" pairing: an orange squeezing into a glass, a tea leaf
  dripping into a cup, a cocoa pod into a mug, a honeycomb into a jar. Keep the ingredient
  nearly as wide as the vessel.
* Change the monochrome: mustard `#e0a526` ground with a yellow cup, forest `#1f5a3c` with
  a green cup. The ground must be the product's own color.
* Add one line of copy: 3 to 5 lowercase words in a light geometric sans, cap height 1.6% H,
  centered at 6% H, white. Nothing more.
* Square format: crop to 12 to 96% H and move the bean to 55% W.
* Pairs with T-vignette (very light) and T-lens-flare on the mug rim; avoid T-grunge and any
  pattern, which break the studio feel.

## 11. Pitfalls

* The stream misses the cup or ends in the air. Its end must sit on the center of the
  opening, with a ripple ring; check the render after you change the product size.
* A ground in a different hue from the product (red mug on blue) turns the idea into an
  ordinary packshot. Sample the product's main color and build the ground from it.
* A flat, single-color ground makes the mug look pasted on. Keep the gradient, the floor
  glow, and the cast shadow; the shadow falls away from the light, to the lower left.
* A small bean reads as a prop. Scale it to 70 to 85% of the vessel's width so the
  exaggeration is obvious.
* Adding a headline, a price, and a web address. This design holds one idea; any copy
  beyond one short line goes on a separate version.
