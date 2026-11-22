---
id: p080
title: Berry splash can on blue and purple
genre: [drink-ad, product-ad, social-post]
mood: [fresh, energetic, bold, cool]
style: [photo-composite, 3d-render]
format: portrait-3x4
archetype: A-hero-object
palette: ['#2d246f', '#100b33', '#2850b4', '#813997', '#8d88ca', '#cfd0ee']
techniques: [T-cutout-drop-shadow, T-vignette, T-fit-width, T-paper-grain, T-water-splash, T-ice-cubes, T-ghost-type]
needs: product-cutout
---

# p080: Berry splash can on blue and purple

## 1. Concept

A flavor shot: one frosted can stands dead center while blueberries, purple juice, and ice
burst outward around it, on a ground that shifts from the brand's blue on the left to the
flavor's purple on the right. The brand name hangs behind everything as a huge, faint
lowercase word. There is no copy at all: the ground color names the flavor, the ghost word
names the brand, and the can does the rest.

## 2. Format and grid

* Portrait 3:4. Symmetric on the 50% W axis, like a product on a pedestal.
* Can: `33.6%,20%,36.4% x 63%`, upright, its base on a wet surface at 83% H.
* Ghost word: 8 to 95% W, 6 to 22% H, behind the top of the can (the can's rim covers the
  middle letter's lower half).
* Fruit and splash form a rough ring around the can from 15 to 95% H and leave the can's
  face completely clear. Big, blurred fruit sits in the bottom corners (the near plane).
* Color split: blue at 0 to 45% W, purple-magenta at 55 to 100% W, blending behind the can.
  Corners fall to near-black.

## 3. Layer stack

1. Ground: left-to-right gradient, royal blue `#0341bf` to violet `#2a2aa0` to purple
   `#6a1594`.
2. Corner shade: radial darkening to `#05041a`, strongest at the top corners and the
   bottom edge.
3. Ghost word: the brand name in lowercase geometric letters, 87% W wide, white at about
   22% opacity (it reads as light blue on the left and pink on the right).
4. Juice splash: translucent purple `#9a3fd0` to magenta `#c869ea` liquid thrown out to
   both sides from behind the can, larger on the right (55 to 98% W, 22 to 72% H), with
   fine droplets.
5. Far fruit: 9 or 10 blueberries, 8 to 13% W each, dusty navy `#1f2a6b` with a pale bloom,
   flying on both sides between 15 and 65% H; two cut halves show purple flesh.
6. Ice: clear cubes along the bottom, 0 to 30% W and 80 to 100% W, 68 to 100% H; two small
   green leaves among them.
7. Can: a royal-blue aluminum can beaded with condensation; a round red-and-blue brand emblem
   in its upper half and a white lowercase wordmark running vertically up its lower half.
   Lit from both sides, so two bright vertical highlights run down its edges.
8. Near fruit: three or four large berries (15 to 20% W), out of focus, cropped by the
   bottom edge.
9. Wet floor: dark reflective surface under the can from 83% H down, with purple
   reflections.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#2d246f` | indigo mid-ground, berries | 29% |
| `#100b33` | dark corners and floor | 22% |
| `#2850b4` | blue left field, can | 18% |
| `#813997` | purple right field, juice | 13% |
| `#8d88ca` | lilac ghost word, bloom on the fruit | 12% |
| `#cfd0ee` | ice, highlights | 6% |

Analogous cool scheme running from blue through indigo to purple. The red in the can's
emblem is the only warm color and marks the center.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Ghost word | `[BRAND]`, 5 lowercase letters with round bowls | lower | geometric sans, light to regular (Poppins 400, Jost 400 / Avenir Next) | 500 | x-height 11%, ascenders to 16% | -20 | - | center | `#ffffff` at 22% | 8 to 95% W, top 6% |
| Wordmark on can | `[BRAND]` (printed on the product) | lower | same | 500 | 5% W | 0 | - | rotated -90 | `#ffffff` | 44% W, 55 to 75% H |

No typeset copy. The ghost word is scenery: one word, fitted to the canvas width, at low
opacity, partly hidden by the product.

## 6. Imagery

A rendered or composited "flavor explosion": product shot upright and frontal; fruit of
the flavor at three depths (far and sharp, mid and sharp, near and blurred); a liquid
splash in the flavor's color; ice for cold. All elements share the two-sided rim lighting
of the can. For a build: your can cutout in the middle, generator splashes behind it, and
fruit as cutouts (the recipe draws stand-in berries from beveled dots).

## 7. Effects and textures

* Split ground: one `fill gradient` left to right with three stops; **T-vignette** in a
  dark shade of the ground on top of it.
* Ghost word: **T-fit-width** to 940 px, then `layer opacity` 22.
* **T-water-splash** (`scripts/splash.js`) twice in purple (`dir=-25` to the right, `dir=200` to the left; `arc=200`,
  centers hidden behind the can so mostly tongues and drops show), opacity
  85, under the can; fine purple dots from `particles.js` as spray.
* Fruit: `particles.js` dots in navy with `fx bevel` (size 26, soften 10, pale lilac
  highlight) and a drop shadow; a second, larger set blurred sigma 7 for the near plane.
* Ice: translucent `rect` shards rendered on a short strip and placed along the bottom.
* **T-cutout-drop-shadow**, glow variant: `fx outer-glow` blur 60, `#6f8cff`, 50% on the can.
* **T-paper-grain**, amount 10, soft-light, 30%.
* **T-water-splash** in [techniques.md](../techniques.md) is the tested library version of the liquid splash; start from it when reusing this idea.
* **T-ice-cubes** in [techniques.md](../techniques.md) is the tested library version of the ice cubes (`particles.js` with `shape=cube` draws fuller cubes than the shards here); start from it when reusing this idea.
* **T-ghost-type** in [techniques.md](../techniques.md) is the tested library version of the faint, defocused, or outlined giant word; start from it when reusing this idea.

## 8. Hierarchy and eye path

1. The can's emblem (the only red, center).
2. Down the vertical wordmark, out along the splash to the berries.
3. Up to the ghost word, last.

The can is 63% H tall; the ghost word's letters are 16% H. There is no small text.

## 9. Build recipe

`product.png` is your image here: a can or bottle cutout, front view. Swap the generated
berries for real fruit cutouts when you have them.

```bash
pixelox render js scripts/splash.js -o juice1.png --size 900x900 --seed 4 --param color='#a548dc' --param cx=0.5 --param cy=0.5 --param radius=0.4 --param tongues=18 --param drops=260 --param dir=-25 --param arc=200
pixelox render js scripts/splash.js -o juice2.png --size 760x760 --seed 8 --param color='#8a45d0' --param cx=0.5 --param cy=0.5 --param radius=0.4 --param tongues=16 --param drops=230 --param dir=200 --param arc=200
pixelox render js scripts/particles.js -o spray.png --size 1080x1440 --seed 6 --param shape=dot --param colors='#c869ea,#e0a0ff,#7a3fc0' --param count=160 --param min=0.2 --param max=0.9 --param amin=0.5 --param amax=1 --param focus='0.52,0.5,0.5'
pixelox render js scripts/particles.js -o berries.png --size 1080x1440 --seed 21 --param shape=dot --param colors='#1f2a6b,#2b3a8c,#16204f,#34409a' --param count=13 --param min=5 --param max=9 --param amin=1 --param amax=1
pixelox render js scripts/particles.js -o near.png --size 1080x420 --seed 3 --param shape=dot --param colors='#1a2460,#2b3a8c,#141a48' --param count=5 --param min=15 --param max=22 --param amin=1 --param amax=1
pixelox render js scripts/particles.js -o ice.png --size 1080x300 --seed 3 --param shape=rect --param colors='#ffffff,#cfd0ee,#9fa8e0' --param count=36 --param min=14 --param max=26 --param jitter=180 --param amin=0.35 --param amax=0.8
pixelox new -o ad.pxo --size 1080x1440 --fill '#0b0a2e'
pixelox layer add ad.pxo --in-place --name Split
pixelox fill gradient ad.pxo --in-place --layer Split --from 0,0 --to 100%,0 --stops '#0341bf@0,#2a2aa0@0.5,#6a1594@1'
pixelox layer add ad.pxo --in-place --name Shade
pixelox fill gradient ad.pxo --in-place --layer Shade --kind radial --from 50%,46% --to 50%,112% --stops '#05041a00@0.4,#05041ae6@1'
pixelox text add ad.pxo --in-place --name Ghost --content "brand" --font "Avenir Next" --weight 500 --size 20% --tracking -20 --color '#ffffff'
pixelox transform scale ad.pxo --in-place --layer Ghost --size 940x
pixelox transform move ad.pxo --in-place --layer Ghost --to 51.5%,6% --anchor t
pixelox layer opacity ad.pxo --in-place --layer Ghost --value 22
pixelox comp place ad.pxo juice1.png --in-place --as JuiceRight --at 70%,48% --anchor c
pixelox layer opacity ad.pxo --in-place --layer JuiceRight --value 85
pixelox comp place ad.pxo juice2.png --in-place --as JuiceLeft --at 30%,56% --anchor c
pixelox layer opacity ad.pxo --in-place --layer JuiceLeft --value 85
pixelox comp place ad.pxo spray.png --in-place --as Spray --fit cover
pixelox comp place ad.pxo berries.png --in-place --as Berries --fit cover
pixelox fx bevel ad.pxo --in-place --layer Berries --size 26 --depth 130 --soften 10 --highlight '#aebcf5' --shadow '#070a2a'
pixelox fx drop-shadow ad.pxo --in-place --layer Berries --offset 0,10 --blur 20 --color '#05041a' --opacity 55
pixelox comp place ad.pxo ice.png --in-place --as Ice --at 50%,100% --anchor b
pixelox comp place ad.pxo product.png --in-place --as Can --size x1000 --at 51.8%,85.5% --anchor b
pixelox fx outer-glow ad.pxo --in-place --layer Can --blur 60 --color '#6f8cff' --opacity 50
pixelox comp place ad.pxo near.png --in-place --as NearBerries --at 50%,100% --anchor b
pixelox fx bevel ad.pxo --in-place --layer NearBerries --size 40 --depth 130 --soften 12 --highlight '#aebcf5' --shadow '#070a2a'
pixelox filter gaussian ad.pxo --in-place --layer NearBerries --sigma 7
pixelox layer add ad.pxo --in-place --name Grain --fill '#808080'
pixelox filter add-noise ad.pxo --in-place --layer Grain --amount 10 --monochrome
pixelox layer blend ad.pxo --in-place --layer Grain --mode soft-light
pixelox layer opacity ad.pxo --in-place --layer Grain --value 30
pixelox export ad.pxo -o ad.png
```

Change first: the right-hand ground color and the splash color (both are the flavor),
then the fruit colors, then the ghost word.

## 10. Variations and pairings

* Twin: p081 is the same template with a red-to-blue ground, apples, and a clear water
  splash. Build a whole flavor range this way: left color fixed (the brand), right color
  and props per flavor (lime green with mint, orange with citrus slices, pink with
  strawberries).
* Add a flavor name: one word in tracked caps, cap height 1.6% H, centered at 93% H in
  white, the only typeset line.
* Square post: can 14 to 86% H, ghost word 4 to 20% H, fruit pushed toward the corners.
* Dark premium version: black ground, a single color glow behind the can (T-spotlight), no
  ghost word.
* Pairs with T-lens-flare (`35mm`) on the rim and T-bokeh in the flavor color.

## 11. Pitfalls

* Fruit or splash across the can's face hides the brand. Keep a clear corridor 40% W wide
  down the middle; props overlap the can only at its base and its edges.
* All props at one size and one sharpness look like wallpaper. Three planes: small sharp
  (far), medium sharp (beside the can), large blurred (bottom corners).
* A ghost word at more than 30% opacity competes with the emblem; under 12% it is lost on
  a phone. 18 to 25% is the range.
* A hard vertical seam between the two ground colors. Blend them over at least 30% W, and
  let the can cover the middle of the blend.
* Warm props on this cool ground (strawberries on blue-purple) fight the scheme; change
  the right-hand color to match the flavor first.
