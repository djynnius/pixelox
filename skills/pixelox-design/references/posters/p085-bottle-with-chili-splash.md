---
id: p085
title: Dark red bottle with chili splash
genre: [drink-ad, product-ad, social-post]
mood: [dramatic, dark, intense, warm]
style: [photo-composite, 3d-render]
format: square
archetype: A-hero-object
palette: ['#250101', '#3d0403', '#640f0a', '#942219', '#cb4639', '#e4c5b9']
techniques: [T-spotlight, T-cutout-drop-shadow, T-vignette, T-photo-fade, T-water-splash]
needs: product-cutout
---

# p085: Dark red bottle with chili splash

## 1. Concept

A "spicy edition" flavor shot with no words: a cold glass bottle stands upright in the
middle of a nearly black red stage, a red glow burns behind it, and whole chili peppers
shoot out of a splash at its foot like flames. Drops hang in the air all around. The
ingredient is arranged to look like fire, so the picture says "hot" before anyone reads a
label.

## 2. Format and grid

* Square, strictly symmetric on the 50% W axis.
* Bottle: `40%,7%,22% x 81%`, cap at 7% H, base at 88% H. It is tall: only 7% H of air
  above it.
* The peppers fan out from the base in a V: two on the left reaching 17% W, 34% H and
  3% W, 59% H; two on the right reaching 83% W, 55% H and 98% W, 62% H. Their tips point
  away from the bottle; their thick ends meet the splash at 75 to 82% H.
* Splash crown: 15 to 85% W, 68 to 92% H. Wet floor and reflection: 88 to 100% H.
* The glow is a disc about 60% W across, centered behind the bottle's label at 50% W,
  42% H.

## 3. Layer stack

1. Ground: near-black red `#250101`, darkest in the top corners `#1c0001`.
2. Glow: saturated red `#cb180f` at the center fading through `#640f0a` to the ground by
   35% W from the axis.
3. Far drops: small round droplets, out of focus, scattered over the whole upper two
   thirds; 30 to 40 of them, 0.5 to 2% W.
4. Peppers: four long red chilies `#c8140c`, glossy, each 35 to 45% W long and 4 to 6% W
   thick at the shoulder, tapering to a point; the right-hand one shows a green stem. A
   fifth small one enters from the right edge at 55% H.
5. Splash: dark red translucent liquid `#942219` thrown up in a crown around the bottle's
   base, with thin tendrils that follow the peppers.
6. Bottle: a glass bottle of dark cola with a green crown cap, a band of foam under the
   neck, a red wrap label with white script at 40 to 55% H, and dense condensation. Rim
   light on both edges; the lower body is almost black.
7. Near drops: a few larger, sharp droplets with highlights (2 to 3% W) at the sides.
8. Floor: glossy black-red surface from 88% H with a short mirror image of the bottle and
   splash.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#250101` | ground | 33% |
| `#3d0403` | outer glow, floor | 28% |
| `#640f0a` | mid glow, liquid | 17% |
| `#942219` | splash, pepper shadows | 11% |
| `#cb4639` | peppers, label, glow core | 8% |
| `#e4c5b9` | highlights, script, foam | 3% |

A single hue, red, from black to pink-white, with one small green accent (cap and stem)
that makes the reds look hotter.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Script on label | `[BRAND]` script wordmark (printed on the product) | title | heavy brush script (Yellowtail, Pacifico / SignPainter) | 400 | 6% (letter height) | 0 | - | center | `#ffffff` | 50%, 48% |

Nothing is typeset. If the post needs words, they go in the caption, not the image.

## 6. Imagery

A rendered still life in low-key light: the bottle is frontal and upright, the props are
dynamic. The peppers are the "flames" and are lit to be the brightest red in the frame;
the bottle is mostly dark so its label band glows. For a build: a bottle cutout over a
T-spotlight glow, peppers drawn as tapered curved paths with a gradient and a bevel, a
splash crown at the base, and generated drops.

## 7. Effects and textures

* **T-spotlight**: radial glow `#d21c10` to `#8a0f0a` to transparent, radius 40% H; no
  floor line, a reflective floor instead.
* Peppers: closed `shape path` curves (two cubic edges meeting at a point), `fx overlay`
  from dark red to bright red along their length, `fx bevel` (size 10) for the glossy
  ridge; the styles are copied to all four with `fx copy`.
* **T-water-splash** (`scripts/splash.js`), dark red, `dir=-90` (upward), placed at the base behind the bottle.
* **T-cutout-drop-shadow**, glow variant: `fx outer-glow` blur 40, `#ff3a2a`, 35% as rim
  light.
* Floor: a dark gradient from 86% H down (**T-photo-fade** logic) over the bottle's foot.
* Drops: `particles.js` rings, two sizes; **T-vignette** in `#0d0000`.
* **T-water-splash** in [techniques.md](../techniques.md) is the tested library version of the liquid splash; start from it when reusing this idea.

## 8. Hierarchy and eye path

1. The red label band on the dark bottle, in the middle of the glow.
2. Out along the peppers to the edges.
3. Down to the splash and the reflection; then the drifting drops.

The bottle is 81% H tall; the peppers reach 40% W to each side. Symmetry and one glow do
all the ranking.

## 9. Build recipe

`product.png` is your image here: a tall bottle cutout, front view.

```bash
pixelox render js scripts/splash.js -o crown.png --size 1000x1000 --seed 5 --param color='#8a140e' --param cx=0.5 --param cy=0.5 --param radius=0.4 --param tongues=18 --param drops=210 --param dir=-90 --param arc=200
pixelox render js scripts/particles.js -o drops.png --size 1080x1080 --seed 8 --param shape=ring --param colors='#ff7a6a,#c8302a,#ffb8a8' --param count=70 --param min=0.4 --param max=2.2 --param amin=0.3 --param amax=0.85
pixelox new -o post.pxo --size 1080x1080 --fill '#250101'
pixelox layer add post.pxo --in-place --name Glow
pixelox fill gradient post.pxo --in-place --layer Glow --kind radial --from 50%,42% --to 50%,84% --stops '#d21c10ff@0,#8a0f0acc@0.45,#25010100@1'
pixelox comp place post.pxo drops.png --in-place --as Drops --fit cover
pixelox shape path post.pxo --in-place --name ChiliLeft1 --d 'M 400 860 C 330 700 250 520 180 365 C 280 480 380 660 440 835 Z' --fill '#c8140c'
pixelox fx overlay post.pxo --in-place --layer ChiliLeft1 --stops '#7a0a06@0,#d21c10@0.6,#ff4a32@1' --angle 90
pixelox fx bevel post.pxo --in-place --layer ChiliLeft1 --size 10 --depth 140 --soften 4 --highlight '#ffb0a0' --shadow '#3a0402'
pixelox shape path post.pxo --in-place --name ChiliLeft2 --d 'M 380 870 C 270 800 140 720 30 640 C 170 660 310 740 410 835 Z' --fill '#c8140c'
pixelox shape path post.pxo --in-place --name ChiliRight1 --d 'M 680 860 C 760 770 830 690 895 600 C 870 710 800 800 715 870 Z' --fill '#c8140c'
pixelox shape path post.pxo --in-place --name ChiliRight2 --d 'M 700 880 C 830 800 950 730 1070 660 C 980 760 850 850 730 895 Z' --fill '#c8140c'
pixelox fx copy post.pxo --in-place --from ChiliLeft1 --to ChiliLeft2,ChiliRight1,ChiliRight2
pixelox shape path post.pxo --in-place --name Stem --d 'M 893 604 C 905 580 925 566 950 560' --fill none --stroke '#3f7a1c' --stroke-width 12 --cap round
pixelox comp place post.pxo crown.png --in-place --as Crown --at 50%,86% --anchor c
pixelox comp place post.pxo product.png --in-place --as Bottle --size x960 --at 51%,92% --anchor b
pixelox fx outer-glow post.pxo --in-place --layer Bottle --blur 40 --color '#ff3a2a' --opacity 35
pixelox layer add post.pxo --in-place --name Floor
pixelox fill gradient post.pxo --in-place --layer Floor --from 0,86% --to 0,100% --stops '#1c000100@0,#1c0001e6@1'
pixelox layer add post.pxo --in-place --name Vignette
pixelox fill gradient post.pxo --in-place --layer Vignette --kind radial --from 50%,48% --to 50%,122% --stops '#0d000000@0.5,#0d0000cc@1'
pixelox layer add post.pxo --in-place --name Grain --fill '#808080'
pixelox filter add-noise post.pxo --in-place --layer Grain --amount 10 --monochrome
pixelox layer blend post.pxo --in-place --layer Grain --mode soft-light
pixelox layer opacity post.pxo --in-place --layer Grain --value 30
pixelox export post.pxo -o post.png
```

Change first: the props (redraw the four paths as your ingredient, or place cutouts), the
glow color, then the bottle height so the cap keeps 5 to 8% H of air.

## 10. Variations and pairings

* Cool edition: ground `#020a1c`, glow `#1a57c8`, mint leaves and ice instead of peppers.
  Citrus: ground `#1c1200`, glow `#ffa400`, lemon wedges.
* Real fire: put **T-fire-type** logic on the product (`fx fire` on the bottle layer, height
  40%) and drop the peppers, for an "extra hot" variant.
* Add one word under the bottle (the flavor), tracked caps, cap height 2% H at 95% H.
* Portrait 4:5: same build with 10% H more air above and room for a logo at 6% H.
* Pairs with T-lens-flare (`35mm`) at the label and T-bokeh in red for the far drops.

## 11. Pitfalls

* Props pointing inward or lying flat look like a salad. They must radiate from the base
  and point outward and up, like flames.
* A glow wider than 70% W lights the whole stage and the drama is gone. Keep the corners
  near black.
* Bright bottle on a bright glow: no silhouette. The bottle body should be darker than the
  glow, with only the label and rim light bright.
* Green everywhere (leaves, lime) cancels the heat. One or two small green notes only.
* A hard cut where the bottle meets the bottom. Use the splash and the floor gradient to
  hide the base.
