---
id: p083
title: Can in hand over blurred script
genre: [drink-ad, product-ad, social-post]
mood: [elegant, warm, bold, moody]
style: [photo-composite, minimal]
format: portrait-3x4
archetype: A-text-behind-subject
palette: ['#ba121f', '#810c11', '#390514', '#190210', '#ebccc6', '#ffffff']
techniques: [T-text-behind-subject, T-fit-width, T-vignette, T-rules, T-paper-grain, T-ghost-type]
needs: product-cutout
---

# p083: Can in hand over blurred script

## 1. Concept

A red-on-red fashion-style product shot: a hand with long nude nails, in a deep red
sleeve, lifts a slim red can into the center of a red field, and the brand's script
wordmark stretches across the whole width behind it, white and thrown far out of focus.
Three tiny pieces of text sit in the corners. The blur is the idea: the logo is present
as a glow, the can in front is razor sharp, and the depth between them makes a flat color
feel like a room.

## 2. Format and grid

* Portrait, close to 3:4. Margins 10% W left and right, 9% H top, 7% H bottom.
* Center axis at 51% W for the can and the blurred word. The corner texts hang from the
  margins: logo top-left, descriptor top-right, slogan bottom-left. The bottom-right corner
  is empty.
* Zones: corner texts 9 to 15% H; can 16 to 73% H; blurred wordmark 35 to 68% H, bleeding
  off both sides; arm 55 to 100% H; slogan line at 92.7% H.
* The can is 29% W wide and 57% H tall: slim, so 35% W of red remains on each side for the
  blurred letters to show.

## 3. Layer stack

1. Ground: red `#c80118` at the top, slightly deeper `#b60b18` in the middle, falling to
   near-black plum `#190210` at the bottom edge (from 75% H down).
2. Blurred wordmark: the brand's flowing script in white, 105% W wide (its first and last
   letters are cut by the edges), centered at 51% H, blurred with a radius of about 1.5% W
   so the letter shapes are just readable; about 85% opacity. It tints the red to pink
   `#e0a3a9` where it is densest.
3. Arm and hand: a hand rising from the bottom center, fingers wrapped around the can's
   lower half from the left, thumb on the right; glossy almond nails in beige; a jeweled
   ring; the sleeve is dark red velvet `#810c11`, merging with the dark bottom.
4. Can: slim aluminum can, `37%,16%,29% x 57%`, glossy red with white script across its
   upper half, a two-word caps descriptor under the script, and small text at the rim and
   base. Sharp, lit from the front with a soft highlight down the left side.
5. Logo, top-left: the script wordmark in white, 19% W wide, at 10% W, 12% H.
6. Descriptor, top-right: two words in tiny bold white caps, right-aligned at 87% W,
   13% H.
7. Slogan, bottom-left: one line of tiny white caps with very wide letterspacing, from
   10% W to 65% W at 92.7% H.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#ba121f` | red ground and can | 44% |
| `#810c11` | sleeve, shaded red | 18% |
| `#390514` | lower shadows | 14% |
| `#190210` | bottom edge | 9% |
| `#ebccc6` | blurred script glow, skin highlights, nails | 9% |
| `#ffffff` | sharp type and can lettering | 6% |

Monochrome red from pink-white to plum-black. Skin is the only other hue and it is warm,
so nothing leaves the family.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Blurred wordmark | `[BRAND]` script, 2 short words | title | heavy brush script (Yellowtail, Pacifico / SignPainter) | 400 | 26% (letter height 33%) | 0 | - | center, bleeding | `#ffffff` at 85%, blurred | 51%, 51% |
| Logo | `[BRAND]` script | title | same | 400 | 3.3% | 0 | - | left | `#ffffff` | 10%, 12% |
| Descriptor | 2 words (the variant name) | caps | neo-grotesque (Inter 700 / Helvetica Neue) | 700 | 1% | 40 | - | right | `#ffffff` | 87%, 13% |
| Slogan | 4 short words and "!" | caps | neo-grotesque (Inter 600 / Helvetica Neue) | 700 | 0.9% | 700 | - | left | `#ffffff` | 10%, 92.7% |

The sharp type is tiny (cap heights about 1% H) and the giant type is blurred: nothing
typeset competes with the can.

## 6. Imagery

One photograph does most of the work: a hand holding the product, shot against a seamless
red backdrop with the model's sleeve in the same red. For a build with only a product
cutout, place it upright in the center, and suggest the arm with a dark, blurred shape
rising from the bottom edge, or use a real hand-with-product cutout. Keep the product
slim and tall; a wide can hides too much of the blurred word.

## 7. Effects and textures

* **T-text-behind-subject** with defocus: the word is blurred (`filter gaussian --sigma 16`
  at 1080 px wide; the blur is live, so the word can still be edited), then the sharp
  product is placed over its middle. First and last letters stay visible at the sides.
* **T-fit-width**: `--fit-width 1130` makes the word wider than the canvas, so it bleeds.
* Ground: vertical `fill gradient`; **T-vignette** weighted to the bottom in plum.
* **T-rules** spirit in the corner texts: three small anchors at the margins that frame
  the empty field.
* **T-paper-grain** (`filter grain --as-layer --amount 8`) for the soft editorial finish.
* **T-ghost-type** in [techniques.md](../techniques.md) is the tested library version of the faint, defocused, or outlined giant word; start from it when reusing this idea.

## 8. Hierarchy and eye path

1. The sharp can (the only crisp, detailed object; center).
2. The hand and nails holding it.
3. The blurred white script, recognized as the brand.
4. Corner texts: logo, descriptor, slogan.

Letter heights: blurred word 33% H against 1% H for the sharp captions: a 30:1 ratio, with
focus (not size) deciding what is read first.

## 9. Build recipe

`product.png` is your image here: a slim can or bottle, or a hand holding one.

```bash
pixelox new -o ad.pxo --size 1080x1440 --fill '#c80118'
pixelox layer add ad.pxo --in-place --name Ground
pixelox fill gradient ad.pxo --in-place --layer Ground --from 0,0 --to 0,100% --stops '#c80118@0,#b60b18@0.55,#5f020e@0.78,#190210@1'
pixelox text add ad.pxo --in-place --name BlurWord --content "Fizz Club" --font @brush --fit-width 1130 --color '#ffffff' --at 51%,51% --anchor c --anchor-to ink
pixelox filter gaussian ad.pxo --in-place --layer BlurWord --sigma 16
pixelox layer opacity ad.pxo --in-place --layer BlurWord --value 85
pixelox shape path ad.pxo --in-place --name Arm --d 'M 250 1440 C 300 1240 420 1060 500 930 L 720 930 C 780 1080 820 1260 850 1440 Z' --fill '#6a0812'
pixelox filter gaussian ad.pxo --in-place --layer Arm --sigma 14
pixelox comp place ad.pxo product.png --in-place --as Can --size x900 --at 51.5%,76% --anchor b
pixelox fx drop-shadow ad.pxo --in-place --layer Can --offset 0,14 --blur 50 --color '#3a0308' --opacity 45
pixelox layer add ad.pxo --in-place --name BottomShade
pixelox fill gradient ad.pxo --in-place --layer BottomShade --from 0,72% --to 0,100% --stops '#19021000@0,#190210d9@1'
pixelox text add ad.pxo --in-place --name Logo --content "Fizz Club" --font @brush --size 5.6% --color '#ffffff' --at 10%,12% --anchor l
pixelox text add ad.pxo --in-place --name Descriptor --content "CLASSIC RECIPE" --font @neo-grotesque --weight 700 --size 1.4% --tracking 40 --color '#ffffff' --at 87%,13% --anchor r
pixelox text add ad.pxo --in-place --name Slogan --content "GOOD TIMES POURED DAILY!" --font @neo-grotesque --weight 700 --size 1.25% --tracking 700 --color '#ffffff' --at 10%,92.7% --anchor l
pixelox filter grain ad.pxo --in-place --as-layer --name Grain --amount 8
pixelox export ad.pxo -o ad.png
```

Change first: the blur word and its `--sigma` (12 to 20: lower when the word is long; the
`--fit-width` refits a new word by itself), then the ground hue (sample your product),
then the three corner texts.

## 10. Variations and pairings

* Any single-color product on its own color: a green bottle on green, a yellow tube on
  yellow, a black phone on charcoal with the word in gray.
* Sharp word, blurred product: swap the focus for a teaser where the product is not yet
  revealed.
* Perfume, cosmetics, sneakers, headphones: anything held or worn that looks good at
  hand-scale.
* Story 9:16: can 20 to 66% H, blurred word 36 to 62% H, slogan centered at 93% H.
* Pairs with T-light-leak (pink, from the top-right) and T-lens-flare on the can's rim; a
  T-pill-button can replace the bottom slogan in a shoppable post.

## 11. Pitfalls

* Blur too weak: the word looks like a low-resolution mistake. Blur until the letter
  joins fill in and only the overall script shape remains (sigma about 1.5% of the width).
* Blur too strong, or a thin script: the word becomes a pink cloud. Use a heavy script and
  check that a stranger can still tell it is lettering.
* The product covers the whole word. The product should hide no more than the middle 30%
  of the word; widen the word past the canvas edges if needed.
* Corner texts set at normal caption size (2% H and up) start to compete. Keep them near
  1% H and bold so they hold at small size.
* A product whose red differs from the ground's (orange-red on blue-red) looks dirty.
  Match the ground to the product, not the other way round.
* Rebuilding a known brand's trade dress. A white script wordmark and ribbon on this exact red
  is one company's signature. Take the idea (what the object, the mark, and the ground do
  to each other) and build it with your own brand's colors, lettering, and shapes.
