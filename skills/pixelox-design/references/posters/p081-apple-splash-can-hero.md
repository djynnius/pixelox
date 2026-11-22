---
id: p081
title: Apple splash can on red and blue
genre: [drink-ad, product-ad, social-post]
mood: [fresh, energetic, bold, playful]
style: [photo-composite, 3d-render]
format: portrait-3x4
archetype: A-hero-object
palette: ['#d04441', '#1c4b92', '#e7caaf', '#1d1a30', '#782e2c', '#638dbe']
techniques: [T-cutout-drop-shadow, T-vignette, T-fit-width, T-paper-grain, T-water-splash, T-ghost-type]
needs: product-cutout
---

# p081: Apple splash can on red and blue

## 1. Concept

A flavor variant ad: a tall blue can stands in the center of a ground that is apple-red on
the left and brand-blue on the right, while whole apples, cut wedges, and a crown of clear
water fly around it. The brand name sits behind as a giant pale lowercase word. No
headline: the fruit says the flavor, the split ground says "the brand meets the fruit",
and the only typeset words are two tiny lines on the can itself.

## 2. Format and grid

* Portrait 3:4. Symmetric on the 50% W axis.
* Can: `32%,21%,36% x 67%`, upright (a slim tall can), base at 88% H in a ring of water.
* Ghost word: 6 to 97% W, 4 to 21% H, behind the can's top; the rim hides the lower part of
  the middle letters.
* Fruit in four corners around the can: a wedge at the upper left (11 to 31% W, 18 to
  33% H) and upper right (75 to 98% W, 26 to 48% H); whole apples at the middle left (5 to
  30% W, 50 to 78% H) and lower right (70 to 100% W, 60 to 85% H); two more wedges in the
  bottom corners, cut faces toward the viewer.
* Color split: red 0 to 40% W, blue 60 to 100% W, with a soft blend behind the can; the
  bottom 8% H is a dark wet floor.

## 3. Layer stack

1. Ground: left-to-right gradient, red `#ea1e2f` to violet-gray `#6a3a70` in the middle to
   blue `#0541aa`.
2. Shade: darkening toward the bottom edge and the lower right corner (`#03040c`).
3. Ghost word: the brand name in lowercase geometric letters, 91% W wide, white at about
   25% opacity (pink on the red side, light blue on the blue side).
4. Mist: a little pale vapor around the top of the can.
5. Water splash: a clear crown rising around the lower third of the can (20 to 85% W, 55 to
   92% H), with long arcs of droplets thrown up both sides to 20% H; the water picks up
   red on the left and blue on the right.
6. Whole apples: two glossy red apples `#d7322b` with droplets, each about 25% W across;
   one green leaf at 17 to 27% W, 43 to 53% H.
7. Wedges: four apple slices with cream flesh `#f6e8d4`, a thin red skin edge, and dark
   seeds; 20 to 25% W long, each at a different angle.
8. Can: royal blue, beaded with condensation; a round red-and-blue brand emblem in the upper
   third, a white lowercase wordmark running vertically, and a two-line flavor label
   (a bold red word over a small white word) at 77 to 80% H.
9. Wet floor: dark blue reflective surface, 92 to 100% H.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#d04441` | red field, apples | 31% |
| `#1c4b92` | blue field, can | 28% |
| `#e7caaf` | apple flesh, splash highlights | 16% |
| `#1d1a30` | floor and deep shadow | 11% |
| `#782e2c` | dark red in the apples' shadows | 8% |
| `#638dbe` | light blue highlights, ghost word on the blue side | 6% |

Two strong hues set against each other, red and blue, with cream as the light neutral.
The blue can stands mostly on the boundary, so it has a red rim on one side and a blue
rim on the other.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Ghost word | `[BRAND]`, 5 lowercase letters | lower | geometric sans (Poppins 400, Jost 400 / Avenir Next) | 500 | x-height 11%, ascenders to 17% | -20 | - | center | `#ffffff` at 25% | 6 to 97% W, top 4% |
| Flavor word | 1 word, 5 letters (the fruit) | caps | heavy grotesque (Archivo Black / Arial Black) | 900 | 1.3% | 20 | - | center | `#e8202a` | 50%, 77.5% |
| Flavor note | 1 word, 9 letters | caps | neo-grotesque (Inter 600 / Helvetica Neue) | 700 | 0.7% | 60 | - | center | `#ffffff` | 50%, 79.6% |
| Wordmark on can | `[BRAND]` (printed on the product) | lower | geometric sans | 500 | 5% W | 0 | - | rotated -90 | `#ffffff` | 43% W, 48 to 74% H |

The flavor label is the only readable copy and it is tiny (cap height 1.3% H): the fruit
has already said it.

## 6. Imagery

The same "flavor explosion" kit as its twin: frontal can, fruit at several angles, liquid
splash, all with crisp studio light and heavy condensation. Here the liquid is clear
water, so the fruit stays the color story, and the fruit is large (each piece 60 to 70% of
the can's width). The cut faces of the wedges are turned to the camera because the cream
color is what separates them from the red ground.

## 7. Effects and textures

* Split ground: `fill gradient` left to right, three stops; **T-vignette** weighted to the
  bottom.
* Ghost word: **T-fit-width** to 980 px, `layer opacity` 25.
* **T-water-splash** (`scripts/splash.js`) in pale blue-white, `dir=-90` (straight up) from the can's base, in
  `screen` mode, plus `particles.js` rings as flying drops.
* Whole apples: red ellipses with a radial `fx overlay` and `fx bevel` for gloss. Wedges:
  half-disc `shape path` in cream with a red stroke, each rotated differently.
* **T-cutout-drop-shadow**, glow variant: a white-blue rim (`fx outer-glow`, blur 50, 40%).
* **T-paper-grain**, amount 10, soft-light, 30%.
* **T-water-splash** in [techniques.md](../techniques.md) is the tested library version of the liquid splash; start from it when reusing this idea.
* **T-ghost-type** in [techniques.md](../techniques.md) is the tested library version of the faint, defocused, or outlined giant word; start from it when reusing this idea.

## 8. Hierarchy and eye path

1. The can's emblem (center, where red and blue meet again in small).
2. The cream wedge faces, clockwise around the can.
3. The red apples, then the ghost word.
4. The tiny flavor label on the can.

The can is 67% H tall; ghost letters 17% H; the flavor label 1.3% H: three levels with
more than a 10x step between each.

## 9. Build recipe

`product.png` is your image here: a can or bottle cutout, front view. Replace the drawn
apples and wedges with real fruit cutouts when you have them.

```bash
pixelox render js scripts/splash.js -o crown.png --size 1300x1300 --seed 12 --param color='#dff0ff' --param cx=0.5 --param cy=0.5 --param radius=0.4 --param tongues=22 --param drops=310 --param dir=-90 --param arc=200
pixelox render js scripts/particles.js -o drops.png --size 1080x1440 --seed 4 --param shape=ring --param colors='#ffffff,#ffe9d0,#cfe6ff' --param count=90 --param min=0.3 --param max=1.6 --param amin=0.4 --param amax=0.9
pixelox new -o ad.pxo --size 1080x1440 --fill '#1d1a30'
pixelox layer add ad.pxo --in-place --name Split
pixelox fill gradient ad.pxo --in-place --layer Split --from 0,0 --to 100%,0 --stops '#ea1e2f@0,#c8283c@0.3,#6a3a70@0.52,#0b47a8@0.75,#0541aa@1'
pixelox layer add ad.pxo --in-place --name Shade
pixelox fill gradient ad.pxo --in-place --layer Shade --from 0,70% --to 0,100% --stops '#03040c00@0,#03040ce6@1'
pixelox text add ad.pxo --in-place --name Ghost --content "brand" --font "Avenir Next" --weight 500 --size 20% --tracking -20 --color '#ffffff'
pixelox transform scale ad.pxo --in-place --layer Ghost --size 980x
pixelox transform move ad.pxo --in-place --layer Ghost --to 51.5%,4% --anchor t
pixelox layer opacity ad.pxo --in-place --layer Ghost --value 25
pixelox comp place ad.pxo crown.png --in-place --as Crown --at 50%,74% --anchor c
pixelox layer blend ad.pxo --in-place --layer Crown --mode screen
pixelox layer opacity ad.pxo --in-place --layer Crown --value 80
pixelox shape ellipse ad.pxo --in-place --name AppleLeft --rect 55,734,285x270 --fill '#d7322b'
pixelox fx overlay ad.pxo --in-place --layer AppleLeft --stops '#ff6a5a@0,#d7322b@0.5,#7a1512@1' --style radial
pixelox shape ellipse ad.pxo --in-place --name AppleRight --rect 765,866,295x275 --fill '#d7322b'
pixelox fx copy ad.pxo --in-place --from AppleLeft --to AppleRight
pixelox shape path ad.pxo --in-place --name WedgeTopLeft --d 'M 120 300 A 110 110 0 0 0 340 300 Z' --fill '#f6e8d4' --stroke '#c8201c' --stroke-width 10
pixelox transform rotate ad.pxo --in-place --layer WedgeTopLeft --degrees 35
pixelox shape path ad.pxo --in-place --name WedgeRight --d 'M 800 520 A 125 125 0 0 0 1050 520 Z' --fill '#f6e8d4' --stroke '#c8201c' --stroke-width 10
pixelox transform rotate ad.pxo --in-place --layer WedgeRight --degrees -70
pixelox shape path ad.pxo --in-place --name WedgeBottomLeft --d 'M 80 1150 A 120 120 0 0 0 320 1150 Z' --fill '#f6e8d4' --stroke '#c8201c' --stroke-width 10
pixelox transform rotate ad.pxo --in-place --layer WedgeBottomLeft --degrees -150
pixelox shape path ad.pxo --in-place --name WedgeBottomRight --d 'M 710 1190 A 140 140 0 0 0 990 1190 Z' --fill '#f6e8d4' --stroke '#c8201c' --stroke-width 10
pixelox transform rotate ad.pxo --in-place --layer WedgeBottomRight --degrees 160
pixelox shape path ad.pxo --in-place --name Leaf --d 'M 190 760 C 200 680 260 640 290 620 C 300 690 270 750 190 760 Z' --fill '#5aa02c'
pixelox comp place ad.pxo product.png --in-place --as Can --size x1060 --at 50%,90.5% --anchor b
pixelox fx outer-glow ad.pxo --in-place --layer Can --blur 50 --color '#cfe6ff' --opacity 40
pixelox comp place ad.pxo drops.png --in-place --as Drops --fit cover
pixelox text add ad.pxo --in-place --name Flavor --content "APPLE" --font "Arial Black" --size 1.8% --tracking 20 --color '#e8202a' --at 50%,77.5% --anchor c
pixelox text add ad.pxo --in-place --name FlavorNote --content "FLAVOURED" --font "Helvetica Neue" --weight 700 --size 1% --tracking 60 --color '#ffffff' --at 50%,79.6% --anchor c
pixelox layer add ad.pxo --in-place --name Grain --fill '#808080'
pixelox filter add-noise ad.pxo --in-place --layer Grain --amount 10 --monochrome
pixelox layer blend ad.pxo --in-place --layer Grain --mode soft-light
pixelox layer opacity ad.pxo --in-place --layer Grain --value 30
pixelox export ad.pxo -o ad.png
```

Change first: the left ground color and the fruit (the flavor), keeping the right side in
the brand color; then the ghost word; then the two-line flavor label.

## 10. Variations and pairings

* Twin: p080 is the same template in blue-to-purple with berries and a colored juice
  splash. Use the pair as a system: brand color on one side, flavor color on the other.
* Flip the split to top and bottom (flavor color above, brand color below) for a 9:16
  story.
* Make the flavor label the headline: the fruit word at cap height 6% H under the can,
  white, centered, with the note line beneath.
* A two-product comparison: two cans side by side, each on its own half of the split
  (A-split-field).
* Pairs with T-sunburst (hard rays, low alpha) behind the can and T-lens-flare on the rim.

## 11. Pitfalls

* Red fruit on the red half disappears. Turn cut faces toward the camera on the red side,
  or put the whole red fruit where the ground is darker, and give every piece a rim light.
* A blue can on the blue half loses its right edge: keep the can on the seam and add the
  pale outer glow.
* The middle of a red-to-blue blend turns gray-violet mud when it is wide. Keep the blend
  to the 25% W that the can covers.
* Fruit all at one angle looks stamped. Rotate every wedge differently and vary sizes by
  20 to 30%.
* Water splash in opaque white reads as milk. Use `screen` mode at 70 to 85% so the ground
  colors tint it.
