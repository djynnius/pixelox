---
id: p105
title: Teal can ad with splash and button
genre: [drink-ad, product-ad, social-post]
mood: [cool, fresh, calm, bold]
style: [photo-composite, minimal]
format: square
archetype: A-hero-object
palette: ['#042c49', '#07476e', '#146d9f', '#4ba3d6', '#cfe9f4', '#ffffff']
techniques: [T-spotlight, T-pill-button, T-cutout-drop-shadow, T-fit-width, T-vignette, T-water-splash, T-ice-cubes]
needs: product-cutout
---

# p105: Teal can ad with splash and button

## 1. Concept

The textbook square product ad: the brand's lowercase wordmark across the top, one frosted
can standing in the center of a deep teal-blue stage with water and ice bursting out to
both sides, an outlined pill button under it, and a three-word line at the very bottom.
Four elements on one center axis, one hue, white type. It works because nothing is
clever: every element is where the eye expects it, with generous space between.

## 2. Format and grid

* Square, symmetric on the 50% W axis. Top and bottom margins 4% H.
* Stack: wordmark 4 to 15.5% H; can 21 to 81% H; button 86 to 91.5% H; tagline at 95.3% H.
* Widths step down and up like an hourglass: wordmark 38% W, can 27% W, button 27% W,
  tagline 43% W.
* The splash is the only wide element: it spans 10 to 87% W behind the can, lower on the
  left (50 to 72% H) and higher on the right (18 to 45% H), a diagonal that keeps the
  symmetric layout from feeling stiff.
* Gaps: 5.5% H between wordmark and can; 5% H between can and button; 3% H between button
  and tagline.

## 3. Layer stack

1. Ground: deep teal-navy `#042c49`, darkest in the corners `#04243d`.
2. Stage light: a radial pool, `#146d9f` at the center behind the can fading through
   `#07476e`; about 75% W across.
3. Splash, left: translucent pale-blue water `#4ba3d6` to `#cfe9f4` thrown out to the left
   and down, 10 to 37% W, 50 to 72% H, with spray.
4. Splash, right: a second arc up and to the right, 63 to 87% W, 18 to 45% H.
5. Ice: four or five clear cubes, 7 to 10% W each, at 25 to 35% W, 28 to 40% H and 67 to
   73% W, 45 to 50% H; more hidden behind the can.
6. Floor light: a bright cyan streak `#2393c1` under the can at 30 to 70% W, 80 to 84% H,
   and a short dark contact shadow.
7. Can: royal-blue can, `37%,21%,27% x 60%`, upright, covered in fine condensation, with a
   round red-and-blue brand emblem in its upper half and a white lowercase wordmark running
   vertically up its lower half; silver rims top and bottom.
8. Wordmark: the brand in white lowercase, centered, 38% W wide.
9. Button: outlined pill, 0.3% W white stroke, no fill, `36.5%,86%,27% x 5.5%`; label in
   white bold caps.
10. Tagline: three adjectives separated by commas, white light caps, centered.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#042c49` | ground | 45% |
| `#07476e` | mid stage | 25% |
| `#146d9f` | stage light, can | 15% |
| `#4ba3d6` | water, floor light | 7% |
| `#cfe9f4` | ice, spray highlights | 5% |
| `#ffffff` | type, button outline | 3% |

One hue from navy to ice. The red in the can's emblem (about 1% of the area) is the single
accent and sits at the exact center.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Wordmark | `[BRAND]`, 5 lowercase letters | lower | geometric sans with round bowls (Poppins 400, Jost 400 / Avenir Next) | 500 | x-height 6%, ascender to 8.5% | -10 | - | center | `#ffffff` | 51%, top 4% |
| Button label | 3 short words and "!" (a call to action) | caps | humanist sans (Open Sans 600 / Avenir Next) | 600 | 1.9% | 20 | - | center | `#ffffff` | 50%, 88.8% |
| Tagline | 3 adjectives with commas, about 25 characters | caps | same, light | 400 | 2.2% | 20 | - | center | `#ffffff` | 50%, 95.3% |

One family, three weights. The button label is slightly smaller than the tagline but
bolder, and the outline does the emphasizing.

## 6. Imagery

A clean pack shot: can frontal, upright, evenly lit with a soft highlight down the left
edge and dense condensation. Water and ice are stock splash elements tinted to the ground
hue and placed entirely behind the can, so its outline stays intact. For a build: a can
cutout on a T-spotlight ground, two splashes in pale blue behind it, translucent shards
as ice, a floor streak under it.

## 7. Effects and textures

* **T-spotlight**: radial `#1a78b8` to `#0a4f7e` to the ground; floor light as a blurred
  cyan ellipse (sigma 14) under the can, plus a small dark contact shadow.
* **T-water-splash** (`scripts/splash.js`) twice, pale blue, `dir=160` (left and slightly down) and `dir=-35`
  (up-right), `screen` mode at 85%. Each splash's center sits behind the
  can, so only tongues and spray show at the sides.
* Ice: `particles.js` rect shards, 10 pieces, placed behind the can.
* **T-cutout-drop-shadow**, glow variant: pale blue outer glow, blur 40, 35%.
* **T-fit-width** for the wordmark (410 px on a 1080 px canvas).
* **T-pill-button**, outlined: stroke 3 px, radius half the height, label 35% of the pill
  height.
* **T-vignette** in `#021624`.
* **T-water-splash** in [techniques.md](../techniques.md) is the tested library version of the liquid splash; start from it when reusing this idea.
* **T-ice-cubes** in [techniques.md](../techniques.md) is the tested library version of the ice cubes (`particles.js` with `shape=cube` draws fuller cubes than the shards here); start from it when reusing this idea.

## 8. Hierarchy and eye path

1. The can (center, sharp, the only saturated object).
2. Up to the wordmark.
3. Down to the button, then the tagline.

Sizes: wordmark letters 8.5% H; tagline cap height 2.2%; button label 1.9%. About 4 : 1 :
0.9 for the type, and the can is 60% H.

## 9. Build recipe

`product.png` is your image here: a can or bottle cutout, front view.

```bash
pixelox render js scripts/splash.js -o splash1.png --size 760x760 --seed 6 --param color='#a9d8f5' --param cx=0.5 --param cy=0.5 --param radius=0.4 --param tongues=20 --param drops=280 --param dir=160 --param arc=200
pixelox render js scripts/splash.js -o splash2.png --size 680x680 --seed 11 --param color='#a9d8f5' --param cx=0.5 --param cy=0.5 --param radius=0.4 --param tongues=18 --param drops=250 --param dir=-35 --param arc=200
pixelox render js scripts/particles.js -o ice.png --size 1080x1080 --seed 4 --param shape=rect --param colors='#ffffff,#cfe9f4,#8fc4e6' --param count=10 --param min=4 --param max=8 --param jitter=180 --param amin=0.45 --param amax=0.85 --param focus='0.5,0.42,0.26'
pixelox new -o post.pxo --size 1080x1080 --fill '#042c49'
pixelox layer add post.pxo --in-place --name StageLight
pixelox fill gradient post.pxo --in-place --layer StageLight --kind radial --from 50%,52% --to 50%,100% --stops '#1a78b8ff@0,#0a4f7ecc@0.5,#042c4900@1'
pixelox comp place post.pxo splash1.png --in-place --as SplashLeft --at 41%,58% --anchor c
pixelox layer blend post.pxo --in-place --layer SplashLeft --mode screen
pixelox layer opacity post.pxo --in-place --layer SplashLeft --value 85
pixelox comp place post.pxo splash2.png --in-place --as SplashRight --at 60%,38% --anchor c
pixelox layer blend post.pxo --in-place --layer SplashRight --mode screen
pixelox layer opacity post.pxo --in-place --layer SplashRight --value 85
pixelox comp place post.pxo ice.png --in-place --as Ice --fit cover
pixelox shape ellipse post.pxo --in-place --name FloorLight --rect 30%,79.5%,40%x4.5% --fill '#2aa6d8'
pixelox layer rasterize post.pxo --in-place --layer FloorLight
pixelox filter gaussian post.pxo --in-place --layer FloorLight --sigma 14
pixelox shape ellipse post.pxo --in-place --name Contact --rect 39%,80%,23%x2.4% --fill '#021a2c'
pixelox layer rasterize post.pxo --in-place --layer Contact
pixelox filter gaussian post.pxo --in-place --layer Contact --sigma 8
pixelox comp place post.pxo product.png --in-place --as Can --size x710 --at 50.5%,84% --anchor b
pixelox fx outer-glow post.pxo --in-place --layer Can --blur 40 --color '#8fd0ff' --opacity 35
pixelox text add post.pxo --in-place --name Wordmark --content "brand" --font "Avenir Next" --weight 500 --size 14% --tracking -10 --color '#ffffff'
pixelox transform scale post.pxo --in-place --layer Wordmark --size 410x
pixelox transform move post.pxo --in-place --layer Wordmark --to 51%,4% --anchor t
pixelox shape rect post.pxo --in-place --name Button --rect 36.5%,86%,27%x5.5% --radius 30 --fill none --stroke '#ffffff' --stroke-width 3
pixelox text add post.pxo --in-place --name ButtonLabel --content "TRY IT NOW!" --font "Avenir Next" --weight 600 --size 2.6% --tracking 20 --color '#ffffff' --at 50%,88.8% --anchor c
pixelox text add post.pxo --in-place --name Tagline --content "CRISP, COLD, BRILLIANT" --font "Avenir Next" --weight 400 --size 3% --tracking 20 --color '#ffffff' --at 50%,95.3% --anchor c
pixelox layer add post.pxo --in-place --name Vignette
pixelox fill gradient post.pxo --in-place --layer Vignette --kind radial --from 50%,50% --to 50%,125% --stops '#02162400@0.5,#021624b3@1'
pixelox export post.pxo -o post.png
```

Change first: the wordmark and the two text lines; then the hue (ground, stage light, and
splash color move together); then the can's height, keeping at least 5% H clear above and
below it.

## 10. Variations and pairings

* Any flavor or brand color: forest `#06301f` with a lime light, plum `#2a0b3d` with a
  magenta light, charcoal with an amber light. Keep type white.
* Filled button (T-pill-button, solid) in the accent color for a harder sell; price in the
  tagline's place.
* Portrait 4:5: add 12% H of air; move the wordmark to 5% H and scale the can to 55% H.
* Replace the splash with the product's own props (fruit ring as in p080 and p081, beans,
  leaves) and keep the same four-part stack.
* Pairs with T-bokeh in the ground hue and T-lens-flare (`35mm`) on the top rim.

## 11. Pitfalls

* Splash in front of the can or across the wordmark. Water stays behind the product and
  below the wordmark's baseline.
* Wordmark as wide as the canvas: the stack loses its hourglass and the can looks small.
  Keep the wordmark near 1.4 times the can's width.
* Button too close to the can (under 4% H) reads as part of the pack. Leave 5% H.
* A filled white button on this ground outshouts the product; the outline is deliberate.
  If you fill it, shrink it.
* Stage light centered on the canvas while the can sits lower: the glow halo shows above
  the can like a second object. Center the light on the can's middle (52% H).
