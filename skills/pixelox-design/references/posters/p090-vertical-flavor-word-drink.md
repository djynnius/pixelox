---
id: p090
title: Drink ad with stacked vertical flavor words
genre: [drink-ad, food-ad, social-post]
mood: [fresh, playful, friendly, calm]
style: [photo-composite, type-led]
format: portrait-4x5
archetype: A-vertical-type
palette: ['#e6e6e6', '#a07a93', '#1e1f40', '#ffffff', '#5e687e']
techniques: [T-vertical-type, T-outline-type, T-fit-width, T-pill-button]
needs: product-cutout
---

# p090: Drink ad with stacked vertical flavor words

## 1. Concept

A social-feed ad for a flavored shake: the flavor name is written three times, each time
turned 90 degrees to read upward (once in white, once in navy, once as a huge pale
outline), and the glass stands between the copies with loose fruit floating around it.
It works because one word does all the talking: repeating it in three treatments makes a
pattern, and turning it sideways lets a 9-letter word be 50% of the canvas tall without
ever crossing the product.

## 2. Format and grid

* Portrait 4:5. No center axis: three vertical bands. Left band 10 to 52% W (a rounded
  color panel with two word copies), middle band 44 to 79% W (the glass, overlapping the
  panel's right edge), right band 64 to 86% W (the outline word, behind the glass).
* All type reads bottom to top (rotated -90 degrees) except the button.
* The two solid words are staggered like steps: the white one low (40 to 90% H) at the
  panel's left edge, the navy one high (9 to 60% H) one column to the right. Each breaks
  out of the panel by 5 to 6% H, the white at the bottom, the navy at the top.
* Button centered at 50% W, 89% H, directly under the panel's right edge.
* Margins: nothing but fruit comes closer than 10% W to the left edge and 14% W to the
  right edge.

## 3. Layer stack

1. Ground: vertical gradient, near-white `#fcfcfc` at the top to light gray `#d0d0d0` at
   the bottom.
2. Outline word: the flavor word, white 3 px outline with no fill, rotated -90, 21% W
   tall letters, running from 86% H up to 8% H at x 64 to 86% W. On the pale ground it is
   a ghost: visible only where the gray is darkest (the lower half).
3. Panel: rounded rectangle `10%,14.8%,42% x 70.4%`, corner radius 5.5% W, flat mauve
   `#a07a93`.
4. White word: the same word, solid white, rotated -90, letters 11% W tall, 51% H long,
   left edge at 12.8% W, running from 90.4% H up to 39.7% H. Its first letter hangs below
   the panel and almost disappears into the light ground.
5. Navy word: the same word, solid navy `#1e1f40`, same size, left edge at 25.3% W,
   running from 59.5% H up to 8.8% H. Its last letter rises above the panel.
6. Tagline: 3 words in small tracked caps, rotated -90, dark plum `#4a3f52`, at
   38 to 40.5% W, running from 72% H up to 26% H: a thin thread of type that separates the
   navy word from the glass.
7. Leaves and loose fruit behind the glass: two mint leaves at its upper left (33 to 60%
   W, 7 to 25% H); berries at 10%,7.5% and 68.5%,7% (each 7.5% W across).
8. Product: a tall glass of purple shake with cream, syrup, berries, and a straw, cutout,
   `44%,8%,35% x 69%`, no shadow.
9. Fruit in front: one large berry with leaves at 53% W, 77% H (12% W across), covering
   the foot of the glass; small berries at 88.5%,45.5%, 12%,70%, and 75.5%,74%.
10. Button: white rounded rectangle `38.3%,86.4%,22.9% x 5%`, radius 1.3% W, soft shadow;
    "ORDER NOW" in navy condensed caps, cap height 2.4% H.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#e6e6e6` | ground (gradient `#fcfcfc` to `#d0d0d0`) | 50% |
| `#a07a93` | mauve panel; the shake repeats it | 30% |
| `#1e1f40` | navy word, button text | 8% |
| `#ffffff` | white word, outline word, button | 7% |
| `#5e687e` | dusty blue berries | 5% |

A muted analogous scheme taken from the product: the panel is the color of the shake, the
navy is the color of the fruit skin, and the rest is neutral gray. Nothing is saturated.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| White word | one flavor word, 8 to 10 letters | caps | condensed bold grotesque (Bebas Neue, Oswald 700, Barlow Condensed 800 / Helvetica Neue 900) | 900 | 11% W (8.8% H), 51% H long | 0 | - | rotated -90 | `#ffffff` | left 12.8% W, bottom 90.4% H |
| Navy word | the same word | caps | same | 900 | same | 0 | - | rotated -90 | `#1e1f40` | left 25.3% W, bottom 59.5% H |
| Outline word | the same word | caps | same | 900, outline 3 px | 21% W (17% H), 78% H long | 0 | - | rotated -90 | `#ffffff` stroke | left 64.2% W, bottom 86.4% H |
| Tagline | 3 words, about 17 letters | caps | geometric sans (Montserrat 500 / Avenir Next 500) | 500 | 1.6% H | 450 | - | rotated -90 | `#4a3f52` | left 38% W, bottom 72.3% H |
| Button | "ORDER NOW" | caps | condensed bold grotesque | 900 | 2.4% | 0 | - | center | `#1e1f40` | 49.8%, 88.9% |

Because the words are rotated, their "cap height" is measured across the canvas width.
The outline word is about twice the size of the solid ones (21% W against 11% W).

## 6. Imagery

One product cutout: a tall ribbed glass of berry shake topped with whipped cream, dark
syrup running down the sides, whole berries, and a straw, photographed straight on with
soft front light. It is 69% of the canvas height and stands with its left third over the
panel, so panel and product are locked together. Around it float six loose berries and a
few mint leaves at different sizes (5 to 12% W), some behind and some in front of the
glass; the far ones are slightly soft. These are separate cutouts of the ingredient: they
say what the flavor is without a word of copy. In a build, use real ingredient cutouts
(`comp place` each one); the recipe draws plain gradient discs as placeholders so the
positions can be judged.

## 7. Effects and textures

* **T-vertical-type**: every word is added flat at the canvas center with its length set
  by `--fit-width`, rotated -90 (it stays live text), then moved by its bottom-left
  corner.
* **T-fit-width** before rotating: `--fit-width 640` for the solid words (47% H on a
  1350 px canvas), `--fit-width 1058` for the outline word (78% H). The face is the
  condensed black cut (`--stretch condensed`), so the letters are tall for their length.
* **T-outline-type** for the ghost word: `--color transparent`, white stroke 3 px, placed
  below the panel and the product in the stack.
* **T-pill-button**, filled variant with a small radius (a rounded rectangle, not a full
  pill), plus `fx drop-shadow` offset 0,6, blur 14, opacity 20.
* Ground gradient: `fill gradient` from `#fcfcfc` to `#d0d0d0`, top to bottom.
* No grain and no vignette: the look is clean and bright.

## 8. Hierarchy and eye path

1. The glass: the only photograph, the darkest and most saturated object.
2. The navy word beside it (highest contrast of the three copies), read upward.
3. The white word on the panel, as an echo; then the tagline beside the glass.
4. The button at the bottom.
5. The outline word is never read: it is texture that fills the right band.

Size ratio across the width: outline 21 : solid words 11 : button 2.4% H : tagline 1.6% H.

## 9. Build recipe

`product.png` is your image here: a tall drink or product cutout. The six `Berry` discs
are placeholders: replace each with a real ingredient cutout of the same size.

```bash
pixelox new -o drink.pxo --size 1080x1350 --fill '#fcfcfc'
pixelox layer add drink.pxo --in-place --name Ground
pixelox fill gradient drink.pxo --in-place --layer Ground --from 0,0 --to 0,100% --stops '#fcfcfc@0,#d0d0d0@1'
pixelox text add drink.pxo --in-place --name Ghost --content "LAVENDER" --font "Helvetica Neue" --weight 900 --stretch condensed --fit-width 1058 --color transparent --stroke '#ffffff' --stroke-width 3 --at 50%,50% --anchor c
pixelox transform rotate drink.pxo --in-place --layer Ghost --degrees -90
pixelox transform move drink.pxo --in-place --layer Ghost --to 64.2%,86.4% --anchor bl
pixelox shape rect drink.pxo --in-place --name Panel --rect 108,200,454x950 --radius 60 --fill '#a07a93'
pixelox text add drink.pxo --in-place --name WordWhite --content "LAVENDER" --font "Helvetica Neue" --weight 900 --stretch condensed --fit-width 640 --color '#ffffff' --at 50%,50% --anchor c
pixelox transform rotate drink.pxo --in-place --layer WordWhite --degrees -90
pixelox transform move drink.pxo --in-place --layer WordWhite --to 12.8%,90.4% --anchor bl
pixelox text add drink.pxo --in-place --name WordNavy --content "LAVENDER" --font "Helvetica Neue" --weight 900 --stretch condensed --fit-width 640 --color '#1e1f40' --at 50%,50% --anchor c
pixelox transform rotate drink.pxo --in-place --layer WordNavy --degrees -90
pixelox transform move drink.pxo --in-place --layer WordNavy --to 25.3%,56.2% --anchor bl
pixelox text add drink.pxo --in-place --name Tagline --content "SIP THE CALM SIDE" --font @geometric-sans --weight 500 --fit-width 560 --tracking 450 --color '#4a3f52' --at 50%,50% --anchor c
pixelox transform rotate drink.pxo --in-place --layer Tagline --degrees -90
pixelox transform move drink.pxo --in-place --layer Tagline --to 38.4%,70% --anchor bl
pixelox shape ellipse drink.pxo --in-place --name Berry1 --rect 68,61,81x81 --fill '#5e687e'
pixelox shape ellipse drink.pxo --in-place --name Berry2 --rect 699,54,81x81 --fill '#5e687e'
pixelox comp place drink.pxo product.png --in-place --as Product --size x930 --at 60.5%,77% --anchor b
pixelox shape ellipse drink.pxo --in-place --name Berry3 --rect 915,574,81x81 --fill '#5e687e'
pixelox shape ellipse drink.pxo --in-place --name Berry4 --rect 92,907,76x76 --fill '#5e687e'
pixelox shape ellipse drink.pxo --in-place --name Berry5 --rect 788,972,54x54 --fill '#5e687e'
pixelox shape ellipse drink.pxo --in-place --name Berry6 --rect 507,975,130x130 --fill '#5e687e'
pixelox fx overlay drink.pxo --in-place --layer Berry1 --stops '#2b3350@0,#8a96b4@1' --angle 60
pixelox fx copy drink.pxo --in-place --from Berry1 --to Berry2,Berry3,Berry4,Berry5,Berry6
pixelox shape rect drink.pxo --in-place --name Button --rect 414,1166,247x68 --radius 14 --fill '#ffffff'
pixelox fx drop-shadow drink.pxo --in-place --layer Button --offset 0,6 --blur 14 --opacity 20
pixelox text add drink.pxo --in-place --name ButtonText --content "ORDER NOW" --font "Helvetica Neue" --weight 900 --stretch condensed --size 3.2% --color '#1e1f40' --at 49.8%,88.9% --anchor c
pixelox export drink.pxo -o drink.png
```

Change first: the flavor word in all three text layers, then the panel color (sample it
from the drink) and the navy (sample it from the darkest part of the fruit). A longer
word needs no other change: `--fit-width` keeps each copy at the same length, so the
letters only get a little smaller.

## 10. Variations and pairings

* Flavor swaps: mango (`#f2b33d` panel, `#7a3b00` dark word, `#fff7e0` ground), mint
  (`#9fd8b8` panel, `#0f3d2e` dark word), strawberry (`#e88aa0` panel, `#5a0f24` dark
  word), coffee (`#b08968` panel, `#2b1a10` dark word). Keep the ground neutral.
* Dark version: ground `#17141a` to `#0b0a0d`, the outline word at 40% opacity, the white
  word stays, the dark word becomes the panel color lightened.
* Story 9:16: lengthen the panel to 12 to 88% H and let the words run longer; the button
  moves to 92% H.
* Suits any single product with a one-word name or flavor: perfume, sneakers, ice cream,
  a candle; also a speaker's first name beside a portrait cutout.
* Pairs with T-cutout-drop-shadow (soft studio shadow under the glass) when the ground is
  darker, T-water-splash behind the glass (`render splash --liquid milk --color` in the
  drink's color), and T-paper-grain at amount 8.

## 11. Pitfalls

* Rotating +90 on the left. Words on the left side of a product read bottom to top
  (-90); turned the other way the three copies look like a spine label and are harder to
  read.
* A wide or light font. The word must be condensed and heavy, or 9 letters will not fit
  in 51% H at a readable size; set the length with `--fit-width` before rotating.
* The product not touching the panel. Let it overlap the panel's right edge by a third of
  its width; side by side, the layout falls into two halves.
* An outline word that competes. It is white on near-white on purpose; a dark or thick
  outline (over 4 px on 1080) turns the texture into a second headline.
* Fruit scattered evenly. Use 5 to 7 pieces in clearly different sizes, half of them
  cropped near the corners and one large piece in front of the product's foot.
