---
id: p012
title: Smart speaker on a blue 3D pedestal
genre: [tech-ad, product-ad, social-post]
mood: [cool, dark, futuristic, bold]
style: [photo-composite, geometric, flat]
format: landscape
archetype: A-hero-object
palette: ['#183463', '#11162c', '#01050d', '#0354be', '#3a3d3f', '#ffffff']
techniques: [T-spotlight, T-gradient-type, T-grunge, T-fit-width]
needs: [product-cutout, subject-cutout, logo]
---

# p012: Smart speaker on a blue 3D pedestal

## 1. Concept

A small round speaker stands on a pedestal drawn from three flat shapes (a lit blue face,
a black face, a pale top) while a hand reaches in from the top right corner and touches
it. The ground is nothing but two navy tones cut into big angular planes. It works
because the pedestal does two jobs at once: it stages the product like a museum piece,
and its two front faces are the two text panels (name on the blue face, tagline on the
black face), so product and copy are one object.

## 2. Format and grid

* Landscape 4:3. No outer margin: the pedestal runs off the bottom edge and the hand off
  the top right corner.
* The pedestal is the grid. Its vertical corner edge at 53.5% W splits the lower third
  into a left panel (17.5 to 53.5% W) and a right panel (53.5 to 89% W). Text in each
  panel is left-aligned with a 2.3% W (left panel) or 13.5% W (right panel) inset.
* Zones: hand 0 to 30% H (right half only); speaker 26 to 66% H, centered at 53% W, right
  above the pedestal's corner edge; pedestal top face 59 to 68% H; pedestal faces 63 to
  100% H.
* The left 17% and the right 11% of the canvas hold only background planes.

## 3. Layer stack

1. Ground: dark navy `#11162c`, full canvas.
2. Blue plane: one large five-sided shape in `#183463` with corners at about 0,32%;
   52%,9.5%; 100%,53%; 100%,100%; 0,100%. It reads as a tilted wall or a wide beam of
   light behind the product.
3. Top wedge: a thin sliver of brighter blue `#00357a` along the top edge, from the left
   corner to a tip at 49% W, 6.5% H.
4. Scuffs: sparse fine white scratches and specks over the planes at about 12% opacity
   (chalkboard or brushed-metal feel).
5. Pedestal, right face: four-sided shape `53.5%,68%` to `89%,63%` down to the bottom
   edge, near-black `#01050d` drifting to `#000b19` at its right side.
6. Pedestal, left face: `17.5%,63%` to `53.5%,68%` down to the bottom edge, gradient from
   bright blue `#0354be` (left) to deep blue `#01306c` (right, at the corner edge).
7. Pedestal, top face: a flat parallelogram with corners 17.5%,63%; 39%,59%; 89%,63%;
   53.5%,68%; teal-blue `#3a9fbe` at the left fading to `#14599b` at the right.
8. Contact shadow: a soft dark ellipse on the top face under the speaker, 26% W wide.
9. Speaker: a dark gray mesh sphere `#3a3d3f` with a flattened, softly glowing top
   (pink-to-teal), `37%,26%,32% x 40%`, resting on the top face.
10. Hand: a right hand entering from the top right corner, index finger extended down and
    left, its tip on the speaker's top at 52% W, 29% H; natural skin color; covers
    `51%,0%,49% x 30%`.
11. Logo: small white mark, 4% W wide, at 31% W, 76.5% H (centered over the first third of
    the product name).
12. Product name: one word in title case with a second capital mid-word, white, bold,
    `19.8%,82.3%,28% x 6%`.
13. Model word: 4 caps, cap height 5.6% H, light blue gradient `#5fd0f5` to `#1a7fe0`
    left to right, left edge 19.5% W, top 90.3% H.
14. Tagline: three short white lines in regular caps on the black face, left edge 67% W,
    cap tops at 87%, 91.7%, 96.5% H.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#183463` | blue background plane | 35% |
| `#11162c` | navy ground | 25% |
| `#01050d` | black pedestal face | 12% |
| `#0354be` | lit pedestal face (to `#01306c`) | 10% |
| `#3a3d3f` | speaker body | 10% |
| `#3a9fbe` | pedestal top, model word | 4% |
| `#ffffff` | type and logo | 2% |
| `#edb49b` | skin of the hand (photo only) | 2% |

Monochrome blue, from near-black to one electric blue, with white type. The hand is the
only warm thing in the picture, which is why the touch point gets the first look.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Product name | `[PRODUCT NAME]`, one compound word, 7 to 9 letters | title, mid-word capital | geometric sans (Poppins, Montserrat / Avenir Next) | 700 | 5.3% | 0 | - | left | `#ffffff` | 19.8%, top 82.3% |
| Model word | 4-letter size or tier word | caps | same | 600 | 5.6% | 40 | - | left | gradient `#5fd0f5` to `#1a7fe0` | 19.5%, top 90.3% |
| Tagline | 3 lines, 2 words each, a benefit promise | caps | neo-grotesque (Inter, Work Sans / Helvetica Neue) | 400 | 2.4% | 20 | 145% | left | `#ffffff` | 67%, top 87% |

One geometric family for the name block, a plain grotesque for the tagline. The name and
the model word are the same size; only weight and color separate them.

## 6. Imagery

Two cutouts. The product: a fist-sized spherical speaker in dark gray mesh, photographed
straight on at its own height, lit from above so the top glows. The hand: a real hand
with the index finger pressing down, cut out and placed so that the fingertip lands
exactly on the product and the wrist leaves through the corner. The touch is the story
(this thing responds to you), and it also gives scale. Both keep natural color; the only
treatment is a soft contact shadow under the speaker (**T-spotlight**, shadow part). For a
build, any small device works (earbuds case, smartwatch, camera, lamp) with any pointing
or holding hand. Presented as a cropped social banner: the lowest text line almost
touches the bottom edge.

## 7. Effects and textures

* Angular planes: plain `shape path` polygons; no blur, no gradient.
* Pedestal: three `shape path` layers. The left face carries `fx overlay --angle 0` with
  `#0358c6` to `#01306c`; the top face `#3a9fbe` to `#14599b`; the right face stays flat.
  Light comes from the left: left face bright, top palest, right face black.
* **T-grunge** (dirt layer only), white, `scratches=70`, `specks=1800`, at 12% opacity,
  placed above the planes and below the pedestal.
* **T-spotlight** contact shadow: a `shape ellipse` 26% W x 3.5% H in `#04122b` with
  `filter gaussian --sigma 6` on it. The blur is live and is not cut off at the shape's
  box.
* **T-gradient-type** on the model word, left to right, light blue to mid blue.
* **T-fit-width** to set the product name to 28% W.

## 8. Hierarchy and eye path

1. The fingertip touching the speaker (the only warm color, on the center axis).
2. Down the sphere to the pedestal's pale top, then the bright blue face.
3. The white product name and the light-blue model word on that face.
4. Across the corner edge to the tagline on the black face.

Cap heights: name 5.3 : model word 5.6 : tagline 2.4, about 2.2 : 2.3 : 1. The product
(40% H) is 7.5 times the height of the name: the object leads, the type labels it.

## 9. Build recipe

`product.png` (the device), `subject.png` (your image here: a cutout of a hand or arm
reaching toward the product), and `logo.png` (a white mark) are your images. The stand-in
subject is a bust, so the recipe turns it upside down to act as the reaching hand; with a
real hand, drop the `transform rotate` line and adjust `--at` (`0-228` starts the layer
228 px above the canvas, so the wrist leaves through the top edge). For a round product use
`--size 460x` instead of `--size x430`.

```bash
pixelox render js scripts/grunge.js -o scuffs.png --size 1440x1080 --seed 9 --param color='#ffffff' --param specks=1800 --param blotches=12 --param scratches=70 --param clump=0.5
pixelox new -o promo.pxo --size 1440x1080 --fill '#11162c'
pixelox shape path promo.pxo --in-place --name Plane --d 'M 0 346 L 749 103 L 1440 572 L 1440 1080 L 0 1080 Z' --fill '#183463'
pixelox shape path promo.pxo --in-place --name Wedge --d 'M 0 0 L 634 0 L 706 70 L 0 27 Z' --fill '#00357a'
pixelox comp place promo.pxo scuffs.png --in-place --as Scuffs --fit cover
pixelox layer opacity promo.pxo --in-place --layer Scuffs --value 12
pixelox shape path promo.pxo --in-place --name FaceRight --d 'M 770 734 L 1282 680 L 1282 1080 L 770 1080 Z' --fill '#01050d'
pixelox fx overlay promo.pxo --in-place --layer FaceRight --stops '#01050d@0,#000b19@1' --angle 0
pixelox shape path promo.pxo --in-place --name FaceLeft --d 'M 252 680 L 770 734 L 770 1080 L 252 1080 Z' --fill '#0354be'
pixelox fx overlay promo.pxo --in-place --layer FaceLeft --stops '#0358c6@0,#01306c@1' --angle 0
pixelox shape path promo.pxo --in-place --name FaceTop --d 'M 252 680 L 562 637 L 1282 680 L 770 734 Z' --fill '#3a9fbe'
pixelox fx overlay promo.pxo --in-place --layer FaceTop --stops '#3a9fbe@0,#14599b@1' --angle 0
pixelox shape ellipse promo.pxo --in-place --name Shadow --rect 40%,62.5%,26%x3.5% --fill '#04122b'
pixelox filter gaussian promo.pxo --in-place --layer Shadow --sigma 6
pixelox comp place promo.pxo product.png --in-place --as Product --size x430 --at 53%,66% --anchor b
pixelox comp place promo.pxo subject.png --in-place --as Hand --size x600 --at 66%,0-228 --anchor t
pixelox transform rotate promo.pxo --in-place --layer Hand --degrees 205
pixelox comp place promo.pxo logo.png --in-place --as Logo --size 58x --at 31%,76.5% --anchor c
pixelox text add promo.pxo --in-place --name Name --content "OrbTone" --font @geometric-sans --weight 700 --fit-width 403 --color '#ffffff' --at 19.8%,82.3% --anchor-to ink
pixelox text add promo.pxo --in-place --name Model --content "LITE" --font @geometric-sans --weight 600 --size 7.8% --tracking 40 --color '#5fd0f5' --at 19.6%,90.3% --anchor tl --anchor-to ink
pixelox fx overlay promo.pxo --in-place --layer Model --stops '#5fd0f5@0,#1a7fe0@1' --angle 0
pixelox text add promo.pxo --in-place --name Tagline --content $'ROOM FILLING\nSOUND FOR EVERY\nMORNING' --font @neo-grotesque --weight 400 --size 3.3% --tracking 20 --leading 145% --color '#ffffff' --at 67%,87% --anchor tl --anchor-to ink
pixelox export promo.pxo -o promo.png
```

Change first: the product and the hand (make the fingertip meet the product), then the
name. To move the pedestal, shift all three `shape path` layers by the same amount; the
shared corner `770 734` must stay under the product's center.

## 10. Variations and pairings

* Other hues, same structure: black ground `#0c0c0e`, plane `#2a2a2e`, lit face `#ff5a1f`
  for audio or gaming gear; cream ground `#efe9df`, plane `#e2d8c8`, lit face `#c8553d`,
  dark type for home goods or cosmetics.
* Portrait 4:5: make the pedestal taller (faces from 58% H down), stack the tagline under
  the name on the lit face, and let the hand come straight down from the top edge.
* No hand: replace it with **T-sunburst** light shafts from the top (`uneven=0.8`,
  `cy=-0.1`) or with T-lens-flare on the product's top.
* Two products: widen the top face and stand them side by side; keep one text panel per
  face.
* Pairs with T-neon-glow on the model word, T-pill-button on the black face in place of
  the tagline, and T-paper-grain at amount 10 to soften the flat planes.

## 11. Pitfalls

* A pedestal whose three faces have similar brightness looks like a flat hexagon. Keep
  the order: top palest, one face bright, the other near-black.
* The product floating above the top face. Its base must overlap the top face by 3 to 5%
  H, with the blurred contact shadow under it.
* The hand as decoration. If the fingertip does not land on the product, remove the hand;
  a near miss looks like a mistake.
* Text crossing the corner edge. Each face holds its own block; nothing spans both.
* Adding a third hue. The warm skin tone only works as an accent because everything else
  is blue; an orange button or a red badge would compete with the touch point.
