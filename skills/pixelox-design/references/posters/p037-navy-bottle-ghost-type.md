---
id: p037
title: Navy bottle ad with ghost type
genre: [drink-ad, product-ad]
mood: [cool, moody, fresh, dark]
style: [photo-composite]
format: portrait-3x4
archetype: A-hero-object
palette: ['#032765', '#15356f', '#0c0e15', '#b8c4ce', '#6e2c29', '#ffffff']
techniques: [T-spotlight, T-vertical-type, T-cutout-drop-shadow, T-paper-grain, T-water-splash, T-condensation, T-ghost-type]
needs: [product-cutout, logo]
---

# p037: Navy bottle ad with ghost type

## 1. Concept

A dark cola bottle and a glass of the drink over ice stand on a deep navy field with water
splashing around them; a year, and two words stacked letter by letter, are set in a blue
only a step lighter than the ground, so they are felt before they are read. One spaced-out
handwritten line under the products carries the tagline. The trick is tone-on-tone "ghost"
type: it fills the empty navy with structure without competing with the product.

## 2. Format and grid

* Portrait, close to 3:4. Margins 8% W. The product pair is centered as a group: glass at
  31 to 50% W, bottle at 50 to 68% W, their bases on one line at 73% H.
* Ghost type frames the pair: the year across the top (8 to 64% W, 4 to 15.5% H); one
  stacked word down the right of the bottle (72 to 77% W, 20 to 73% H); a shorter stacked
  word down the left of the glass (26 to 31% W, 48 to 73% H).
* Logo in the top-right corner (85 to 97% W, 3.5 to 10% H).
* Tagline on one line at 77% H, 22 to 83% W, just under the products' feet.
* The bottom 22% H is empty, slightly lighter navy: breathing room.

## 3. Layer stack

1. Ground: navy `#032765`, full canvas.
2. Lower haze: lighter slate blue `#15356f` rising from the bottom edge to 78% H.
3. Glow: a soft lighter-blue pool `#0d4a9a` behind the bottle, 35% W radius, centered at
   59% W, 45% H.
4. Ghost year: 4 digits, widely spaced, hand-drawn marker style, color `#0b3476` (a hair
   lighter than the ground), cap height 11% H. Four tiny tracked letters sit inside the
   second digit as a signature.
5. Ghost word right: 7 lowercase letters, one per line, upright, color `#17407f`,
   letter height 4.5% H, running from 20 to 73% H at 74.5% W.
6. Ghost word left: 4 glyphs, one per line, same color, at 28.7% W, 48 to 73% H.
7. Splashes: light cyan water `#5fb8e8` to white, thrown up and to the left from the glass
   (23 to 48% W, 17 to 49% H), with a second, smaller spray right of the bottle (69 to 87%
   W, 25 to 62% H).
8. Glass: a tumbler of cola and ice cubes, `31%,44%,19% x 28.5%`, with water crashing at
   its foot.
9. Bottle: a tall glass bottle of dark cola with a blue cap, a round emblem and a white
   wordmark on its shoulder, `49.6%,19%,18.4% x 55%`; rim-lit in blue on both sides, red
   reflections in the liquid at its left edge.
10. Water line: a thin dark ripple band across 5 to 97% W at 76 to 79% H, where the
    products meet the surface.
11. Tagline: white handwritten lowercase with very wide spacing; the second word is replaced
    by the brand wordmark in bold italic blue.
12. Logo: round emblem 5% W wide with a bold italic wordmark under it, top right.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#032765` | navy ground | 54% |
| `#15356f` | lower haze, ghost type | 30% |
| `#0c0e15` | bottle body | 6% |
| `#b8c4ce` | ice, splash highlights, glass | 5% |
| `#6e2c29` | cola in the glass, red reflections | 3% |
| `#ffffff` | tagline, wordmark | 2% |

Blue monochrome with a small warm accent from the drink itself. The ghost type differs
from the ground by only 6 to 10% lightness.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Ghost year | 4 digits | - | hand-drawn marker (Permanent Marker, Gochi Hand / Marker Felt) | 400 | 11% | 250 | - | left | `#0b3476` | 8%, top 4% |
| Ghost word right | 1 word, 7 letters, stacked one per line | lower | same | 400 | 4.5% (letter height) | 0 | 140% | center | `#17407f` | 74.5%, 20 to 73% |
| Ghost word left | 4 glyphs, stacked | caps | same | 400 | 4% | 0 | 140% | center | `#17407f` | 28.7%, 48 to 73% |
| Tagline | 5 words; word 2 is the `[BRAND]` wordmark | lower | handwritten mono-line (Architects Daughter, Patrick Hand / Bradley Hand) | 700 | 1.3% (x-height 1%) | 300 | - | center | `#ffffff` | 22 to 83% W, 77% |
| Inline wordmark | `[BRAND]`, 5 letters | caps | wide heavy italic sans (Montserrat 900 Italic / Avenir Next) | 900 italic | 1.6% | 0 | - | center | `#3f7fd6` | 39.5%, 77% |
| Logo wordmark | `[BRAND]` | caps | same | 900 italic | 1.5% | 0 | - | center | `#3f7fd6` | 91%, 9.2% |

Three voices: a marker face for the ghosts, a thin handwritten face for the tagline, and
the brand's own heavy italic. Only the tagline is at full contrast.

## 6. Imagery

Two cutouts photographed in the same cold light: a full bottle with heavy condensation and
a glass of the drink with ice. The bottle is 1.9 times as tall as the glass and stands a
little behind it. Water is the connecting element: a crown splash at the glass's foot, an
arc of thrown water above it, a few drops off the bottle's right shoulder. Both products
have a blue rim light, which is what separates the black bottle from the navy ground.

## 7. Effects and textures

* **T-spotlight** in blue: glow `#0d4a9a` at 60% behind the bottle; no floor line, a haze
  gradient instead.
* Ghost type: plain text layers in a color 6 to 10% lighter than the ground. No outline, no
  blur.
* **T-vertical-type**, the stacked variant: one letter per line (`$'f\nr\ne...'`), letters
  upright, instead of a rotated word.
* **T-water-splash**: `render splash` tinted light cyan (`--color '#8fd0f5'`), twice. The
  arc above the glass is `--kind sheet --angle 115 --spread 60` (thrown up and to the left).
  The glass stands in a small crown: `--part back` before the glass and `--part front`
  after it, with the same `--at`, `--size`, and `--seed`. `particles.js` rings add loose
  bubbles.
* **T-condensation**: `render droplets --layer Bottle`, run after the bottle is placed.
* **T-cutout-drop-shadow**, glow variant: `fx outer-glow` blur 70, `#3f8ff0`, 45% on the
  bottle as the rim light.
* **T-paper-grain**: `filter grain --as-layer --amount 8`, last.
* **T-ghost-type** in [techniques.md](../techniques.md) is the tested library version of the faint, defocused, or outlined giant word; start from it when reusing this idea.

## 8. Hierarchy and eye path

1. The bottle (tallest, darkest, with the only saturated emblem).
2. The glass and the splash to its left.
3. The white tagline under them.
4. The logo top right; the ghost year and stacked words are noticed last.

Cap heights: ghost year 11 : stacked letters 4.5 : inline wordmark 1.6 : tagline 1.3. The
biggest type has the least contrast and the smallest has the most: size and contrast are
traded against each other.

## 9. Build recipe

`product.png` is your image here (used twice: place your glass or second pack as `Glass`),
and `logo.png` is your emblem.

```bash
pixelox render js scripts/particles.js -o drops.png --size 1080x1440 --seed 4 --param shape=ring --param colors='#bfe6ff,#ffffff' --param count=40 --param min=0.5 --param max=1.8 --param amin=0.3 --param amax=0.8 --param focus='0.55,0.42,0.33'
pixelox new -o ad.pxo --size 1080x1440 --fill '#032765'
pixelox layer add ad.pxo --in-place --name Haze
pixelox fill gradient ad.pxo --in-place --layer Haze --from 0,74% --to 0,100% --stops '#15356f00@0,#15356fff@0.5,#1a3a74ff@1'
pixelox layer add ad.pxo --in-place --name Glow
pixelox fill gradient ad.pxo --in-place --layer Glow --kind radial --from 59%,45% --to 59%,80% --stops '#0d4a9a99@0,#0d4a9a00@1'
pixelox text add ad.pxo --in-place --name GhostYear --content "2031" --font @marker --size 15% --tracking 250 --color '#0b3476' --at 8%,4% --anchor tl --anchor-to ink
pixelox text add ad.pxo --in-place --name GhostRight --content $'f\nr\ne\ns\nh\ne\nr' --font @marker --size 5.4% --leading 140% --align center --color '#17407f' --at 74.5%,20% --anchor t --anchor-to ink
pixelox text add ad.pxo --in-place --name GhostLeft --content $'C\nO\nL\nD' --font @marker --size 4.6% --leading 140% --align center --color '#17407f' --at 28.7%,48% --anchor t --anchor-to ink
pixelox render splash ad.pxo --in-place --name SplashArc --kind sheet --color '#8fd0f5' --at 40%,50% --angle 115 --spread 60 --size 38% --seed 6
pixelox comp place ad.pxo drops.png --in-place --as Drops --fit cover
pixelox render splash ad.pxo --in-place --name SplashBack --part back --color '#8fd0f5' --at 40.5%,71.3% --size 27% --seed 3
pixelox comp place ad.pxo product.png --in-place --as Glass --size x450 --at 40.5%,74% --anchor b
pixelox render splash ad.pxo --in-place --name SplashFront --part front --color '#8fd0f5' --at 40.5%,71.3% --size 27% --seed 3
pixelox comp place ad.pxo product.png --in-place --as Bottle --size x860 --at 58.8%,76% --anchor b
pixelox fx outer-glow ad.pxo --in-place --layer Bottle --blur 70 --color '#3f8ff0' --opacity 45
pixelox render droplets ad.pxo --in-place --layer Bottle --density 50 --trickles 4
pixelox shape ellipse ad.pxo --in-place --name WaterLine --rect 5%,76.6%,92%x2% --fill '#021a47'
pixelox filter gaussian ad.pxo --in-place --layer WaterLine --sigma 7
pixelox text add ad.pxo --in-place --name TagLeft --content "each" --font "Bradley Hand" --weight 700 --size 2.3% --tracking 300 --color '#ffffff' --at 31%,77.2% --anchor r
pixelox text add ad.pxo --in-place --name TagBrand --content "BRAND" --font @geometric-sans --weight 900 --style italic --size 2.2% --color '#3f7fd6' --at 39.8%,77.2% --anchor c
pixelox text add ad.pxo --in-place --name TagRight --content "brightens the day" --font "Bradley Hand" --weight 700 --size 2.3% --tracking 300 --color '#ffffff' --at 48.5%,77.2% --anchor l
pixelox comp place ad.pxo logo.png --in-place --as Logo --size 58x --at 91%,5.7% --anchor c
pixelox text add ad.pxo --in-place --name LogoWord --content "BRAND" --font @geometric-sans --weight 900 --style italic --size 2.1% --color '#3f7fd6' --at 91%,9.3% --anchor c
pixelox filter grain ad.pxo --in-place --as-layer --name Grain --amount 8
pixelox export ad.pxo -o ad.png
```

Change first: the three ghost strings and the tagline; then the ground hue, with the ghost
color recomputed as the ground lightened by 6 to 10%. With a different glass, move the
crown (`--at` on both `SplashBack` and `SplashFront`) so its pool sits just above the
glass's base and stays clear of the tagline. The droplets show the bottle as it was when
their command ran: after swapping the bottle, run the recipe again from the top.

## 10. Variations and pairings

* Ghost type on any dark monochrome: bottle green `#06301f` with ghosts in `#0d4a31`;
  oxblood `#3d0509` with `#5a0d12`; black with `#1c1c1c`.
* New-year or seasonal greeting from a brand (the ghost year is the occasion), limited
  editions, a countdown series with one ghost numeral per poster.
* Replace the stacked ghost words with one rotated word (T-vertical-type) up the right
  edge for a cleaner, more editorial version.
* 9:16 story: year at 6% H, products 25 to 70% H, tagline at 75% H, logo bottom center.
* Pairs with T-bokeh in blue behind the products and T-lens-flare (`35mm`) on the bottle's
  shoulder.

## 11. Pitfalls

* Ghost type too light: it becomes real type and the layout turns busy. Keep it within 10%
  lightness of the ground; check that the tagline is still the first thing read.
* Ghost type too dark or too small: invisible on phones. Cap height at least 4% H.
* A black bottle on navy with no rim light disappears. Add the blue outer glow or a lighter
  pool behind it.
* The tagline tracked wide in a bold or condensed face looks broken. Wide tracking needs a
  thin, open face and lowercase.
* Splash in gray or pure white reads as smoke or paint. Tint it cyan (`--color`) and keep
  the navy ground dark enough for clear water to show.
* A crown wider than the glass's own width times 1.5 swallows the tagline under it. Keep
  `--size` near 27% here and the pool above the water line.
