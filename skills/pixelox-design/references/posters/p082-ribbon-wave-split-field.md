---
id: p082
title: Label ribbon becomes a wave
genre: [drink-ad, product-ad]
mood: [bold, minimal, playful, warm]
style: [photo-composite, minimal, duotone]
format: portrait-4x5
archetype: A-split-field
palette: ['#f30101', '#1a0302', '#470f0d', '#a11612', '#f6f6f6']
techniques: [T-duotone, T-image-in-type, T-silhouette, T-paper-grain, T-counterchange]
needs: product-cutout
---

# p082: Label ribbon becomes a wave

## 1. Concept

A visual pun in two halves: the left half is an extreme close-up of a dark glass bottle,
the right half is flat, pure red. The white ribbon printed on the bottle's label does not
stop at the glass: it sweeps out across the red field and thins to a point at the right
edge, and a tiny red swimmer dives through it. A piece of the logo becomes a wave. No
words are needed; the cropped script on the bottle is enough to name the brand.

## 2. Format and grid

* Portrait 4:5, full bleed, no margins.
* Vertical split at 48% W: bottle photo on the left, flat color on the right. The split is
  not a straight line: it is the bottle's own right edge, slightly curved.
* One horizontal band carries everything: the ribbon, 42 to 56% H on the bottle, rising to
  37 to 39% H at the right edge. Above and below it the right half is empty red.
* The swimmer is at 56% W, 48% H, just right of the split: 11% W long, the smallest thing
  on the page and the focus.
* The script wordmark on the bottle is cropped by the left edge: only its last 5 or 6
  letters show, between 33 and 58% H.

## 3. Layer stack

1. Field: flat red `#f30101`, full canvas.
2. Bottle: macro photo of a glass bottle of cola, filling 0 to 48% W from top to bottom,
   toned to a red monochrome: near-black `#1a0302` in the glass, dark red `#470f0d` to
   `#a11612` in the highlights, covered with condensation drops. Its ridged shoulder
   crosses at 12% H and the base ridge at 73% H.
3. Script wordmark on the bottle: large flowing white script, letters about 20% H tall,
   cropped at the left edge.
4. Ribbon on the bottle: a pale gray-white band `#d7d8d1`, 13% H thick at the split,
   narrowing to the left; where the script crosses it the letters turn dark red
   (counterchange).
5. Ribbon on the field: pure white `#ffffff`, continuing from the same band at the split
   (43 to 55% H) and tapering in a shallow upward curve to a point at 100% W, 38% H. The
   upper edge is nearly straight; the lower edge curves up.
6. Swimmer: a tiny red figure in a diving pose (arms forward, legs back), in the field red,
   inside the white ribbon near its thick end.
7. Fine grain on the bottle half only (it comes from the photo).

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#f30101` | flat field | 53% |
| `#1a0302` | bottle glass | 24% |
| `#470f0d` | bottle mid-tones | 9% |
| `#a11612` | bottle highlights | 7% |
| `#f6f6f6` | ribbon, script | 7% |

One red, its darkened versions, and white. The photo is toned so that it has no hue but
the field's.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Script on bottle | `[BRAND]` script wordmark (printed on the product), cropped to its last 5 or 6 letters | title | heavy brush script (Yellowtail, Pacifico / SignPainter) | 400 | 20% (letter height) | 0 | - | left, bleeding off | `#f6f6f6`, dark red `#a11612` where it crosses the ribbon | 0 to 47% W, 33 to 58% H |

No typeset text at all: no tagline, no logo block, no address. The cropped script is part
of the picture.

## 6. Imagery

* A macro product photo with heavy condensation, cropped so tightly that the bottle is a
  dark textured wall with a curved right edge. It is toned with **T-duotone** (black to the
  field red) so it reads as a shade of the background.
* The ribbon is a flat vector shape that starts exactly where the printed one ends.
* The swimmer is a one-color pictogram, about 1/9 of the canvas width: small enough to be
  discovered. Any tiny figure that fits the pun works (a surfer, a skater on a stripe, a
  climber on a letter).

## 7. Effects and textures

* **T-duotone** on the product: `adjust gradient-map` from `#120101` through `#5a0a0a` and
  `#c81410` to `#ff4a3a`.
* A left-edge shade clipped to the bottle (`layer clip`) so the glass falls into black
  away from the split.
  The white script is clipped to the bottle as well, so no letter spills onto the field.
* Counterchange (**T-image-in-type** logic): the script is set twice in the same place,
  white under the ribbon and dark red clipped to the ribbon, so letters change color where
  they cross it.
* **T-silhouette** thinking for the swimmer: a figure reduced to one flat color.
* **T-paper-grain** at low strength over the bottle half.
* **T-counterchange** in [techniques.md](../techniques.md) is the tested library version of the lettering that changes color where it crosses a shape; start from it when reusing this idea.

## 8. Hierarchy and eye path

1. The white ribbon (the only white on the red half) pulls the eye right from the bottle.
2. The tiny swimmer inside it: the surprise.
3. Back left to the script on the bottle, which names the brand.

Scale contrast is the hierarchy: bottle 100% H, ribbon 13% H, swimmer 2.5% H.

## 9. Build recipe

`product.png` is your image here: a bottle or can cutout, scaled until only its right-hand
part is on the canvas.

```bash
pixelox new -o ad.pxo --size 1080x1350 --fill '#f30101'
pixelox comp place ad.pxo product.png --in-place --as Bottle --size x3000 --at 16%,50% --anchor c
pixelox adjust gradient-map ad.pxo --in-place --layer Bottle --stops '#120101@0,#5a0a0a@0.45,#c81410@0.8,#ff4a3a@1'
pixelox layer add ad.pxo --in-place --name BottleShade
pixelox fill gradient ad.pxo --in-place --layer BottleShade --from 0,0 --to 46%,0 --stops '#100001e6@0,#10000100@1'
pixelox layer clip ad.pxo --in-place --layer BottleShade
pixelox text add ad.pxo --in-place --name Script --content "Brand" --font SignPainter --size 36% --color '#f6f6f6'
pixelox transform move ad.pxo --in-place --layer Script --to 0,31% --anchor tl
pixelox transform move ad.pxo --in-place --layer Script --by -230,0
pixelox layer clip ad.pxo --in-place --layer Script
pixelox shape path ad.pxo --in-place --name RibbonIn --d 'M 0 640 C 150 590 350 600 522 583 L 522 745 C 350 770 150 720 0 770 Z' --fill '#d7d8d1'
pixelox text add ad.pxo --in-place --name ScriptRed --content "Brand" --font SignPainter --size 36% --color '#a11612'
pixelox transform move ad.pxo --in-place --layer ScriptRed --to 0,31% --anchor tl
pixelox transform move ad.pxo --in-place --layer ScriptRed --by -230,0
pixelox layer clip ad.pxo --in-place --layer ScriptRed
pixelox shape path ad.pxo --in-place --name RibbonOut --d 'M 520 583 C 700 578 900 556 1080 503 L 1080 520 C 900 610 700 725 520 745 Z' --fill '#ffffff'
pixelox shape path ad.pxo --in-place --name Swimmer --d 'M 545 652 L 598 648 L 640 640 L 668 644 M 598 648 L 572 664 M 640 640 L 662 628' --fill none --stroke '#f30101' --stroke-width 8 --cap round --join round
pixelox shape ellipse ad.pxo --in-place --name SwimmerHead --rect 600,634,14x14 --fill '#f30101'
pixelox layer add ad.pxo --in-place --name Grain --fill '#808080'
pixelox filter add-noise ad.pxo --in-place --layer Grain --amount 12 --monochrome
pixelox layer blend ad.pxo --in-place --layer Grain --mode soft-light
pixelox layer opacity ad.pxo --in-place --layer Grain --value 30
pixelox export ad.pxo -o ad.png
```

Change first: the product's scale and position until its right edge sits near 48% W (then
move the two ribbon paths so they meet at that edge: x 520 here), the script word, and the
pictogram that rides the ribbon.

## 10. Variations and pairings

* Any brand device can leave the pack: a stripe becomes a road with a tiny car, a swoosh a
  ski slope, a dotted line a trail of footprints, a check mark a ramp.
* Mirror it (product on the right) or turn it 90 degrees for a landscape billboard:
  product strip along the bottom, the device rising into a flat sky.
* Other one-color systems: cobalt `#0a3fd6` with a navy-toned product; yellow `#ffd400`
  with a black-toned product.
* Add a line of copy only if needed: 3 to 5 words, cap height 1.4% H, white, bottom right
  at 94% H, right-aligned at 94% W.
* Pairs with T-paper-grain only. Shadows, glows, and gradients on the flat half would
  break the cut-paper look.

## 11. Pitfalls

* The ribbon on the field does not line up with the one on the product. Both edges must
  meet at the split without a step: read the product's bounds, then set the first points
  of `RibbonOut` to the last points of `RibbonIn`.
* The product in natural color (brown cola, green glass) makes two pictures instead of
  one. Tone it to the field's hue.
* A pictogram that is too big or too detailed stops being a discovery. Keep it under 12% W,
  one flat color, no face.
* A field color that is not exactly one flat value (a gradient, a vignette) weakens the
  split. The right half is a single hex.
* Adding the full logo "to be safe". The cropped script on the pack already identifies the
  brand; a second logo kills the wit.
* Rebuilding a known brand's trade dress. A white script wordmark and ribbon on this exact red
  is one company's signature. Take the idea (what the object, the mark, and the ground do
  to each other) and build it with your own brand's colors, lettering, and shapes.
