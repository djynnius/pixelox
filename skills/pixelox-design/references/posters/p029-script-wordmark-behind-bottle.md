---
id: p029
title: Giant script wordmark behind bottle
genre: [drink-ad, product-ad, social-post]
mood: [bold, energetic, retro, warm]
style: [photo-composite, type-led]
format: landscape
archetype: A-text-behind-subject
palette: ['#ea0001', '#ffffff', '#871412', '#311612']
techniques: [T-text-behind-subject, T-outline-type, T-waves, T-fit-width]
needs: product-cutout
---

# p029: Giant script wordmark behind bottle

## 1. Concept

A landscape brand banner: the script wordmark is blown up until it nearly touches both
edges, set once in solid white and once more below as a white outline, and a photographed
glass bottle stands in front of both, splitting each wordmark in two. Thin dark contour
lines ripple across the lower half. It works because the logo is used as scenery: solid,
then echo, with the real product as the only photographic thing in a flat world.

## 2. Format and grid

* Landscape, about 5:4 (1.28 : 1). Side margins 5% W for the wordmarks; the bottle sits on
  an axis at 47.5% W, a little left of center.
* Two stacked rows of type: solid wordmark 9 to 47% H; outline wordmark 52 to 93% H. Both
  span 5 to 92% W, so the two rows are the same width and the same size.
* The bottle is one column, 33 to 62% W, from 11% H down past the bottom edge (cropped at
  about its lower third). It hides the middle 30% of each wordmark: the last letter of the
  first word and the first letter of the second.
* Contour lines fill the band from 57 to 100% H, full width, behind the outline wordmark.

## 3. Layer stack

1. Ground: flat saturated red `#ea0001`, full canvas.
2. Contour lines: about 12 thin wavy lines in dark red-black `#871412` to `#311612`,
   0.4% W wide, flowing left to right across 57 to 100% H, bunching and spreading like a
   topographic map. They carry on behind the bottle.
3. Solid wordmark: a two-word script logotype joined by a hyphen, white, 87% W wide,
   capitals reaching from 9 to 47% H, with a long swash from the first capital.
4. Outline wordmark: the same wordmark, same size, drawn as a 2 px white outline with no
   fill, top at 52% H, so the contour lines show through the letters.
5. Bottle: a glass bottle of dark cola with a metal cap, a white wrap label with a script
   mark, and beads of condensation; 29% W wide at the body, cap top at 11% H, cropped by
   the bottom edge. Lit from the upper left, its liquid glows orange-red `#d15a37` in the
   middle, so it melts into the red ground at its edges.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#ea0001` | ground | 64% |
| `#ffffff` | solid wordmark, outline, bottle label | 15% |
| `#871412` | bottle body mid-tones, contour lines | 12% |
| `#311612` | bottle shadows, darkest lines | 9% |

Red and white only; the dark tones all come from the photo and the hairlines. The bottle's
own red-brown belongs to the ground's hue family, which is why it does not look pasted on.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Solid wordmark | `[BRAND]`, two 4-letter words joined by a hyphen | title | heavy flowing script (Yellowtail, Pacifico / SignPainter) | 400 (the face is heavy) | 30% (capitals with swashes span 38%) | 0 | - | center | `#ffffff` | 5 to 92% W, top 9% |
| Outline wordmark | the same wordmark | title | same | outline 2 to 3 px | same | 0 | - | center | stroke `#ffffff`, no fill | 5 to 92% W, top 52% |

No other text: no tagline, no web address. The two wordmarks are the same width (T-fit-width
used on a script).

## 6. Imagery

One studio photo of a cold glass bottle, cut out, front view, upright, placed in front of
all the type (T-text-behind-subject). It is big: its body is 29% W wide and it is taller
than the canvas, so the bottom is cropped. Condensation and a bright highlight down the
left side give it the only depth in the design. Its printed label repeats the wordmark a
third time at small size, at 62 to 78% H.

## 7. Effects and textures

* **T-text-behind-subject**: ground, lines, wordmarks, then the product on top. The
  product covers 30% of each word and leaves the first and last letters clear.
* **T-outline-type**: the second row is `--color transparent` with a white `--stroke` of
  2 to 3 px (about 0.8% of the font size: thin, so it reads as an echo).
* **T-waves** in line mode for the contour lines: `count=14`, `amp=0.035`, `freq=1.6`,
  `chaos=0.25`, width 5 px on a 1350 px canvas, dark red, top 0.58.
* No shadow under the bottle, no grain. The ground is perfectly flat.

## 8. Hierarchy and eye path

1. The bottle (the only photograph, highest detail, on the center axis).
2. The solid white wordmark it interrupts, read left and right of it.
3. The outline echo below, then the texture of the lines.

Size: wordmark capitals 30% H against a bottle 90% H tall. There is one level of type; the
outline copy is the same size at about a third of the visual weight.

## 9. Build recipe

`product.png` is your image here: a tall bottle or can cutout, front view.

```bash
pixelox render js scripts/waves.js -o lines.png --size 1350x1080 --seed 3 --param mode=line --param colors='#4a0808' --param count=14 --param top=0.58 --param bottom=1.03 --param amp=0.035 --param freq=1.6 --param chaos=0.25 --param width=5 --param alpha=0.9
pixelox new -o banner.pxo --size 1350x1080 --fill '#ea0001'
pixelox comp place banner.pxo lines.png --in-place --as Lines --fit cover
pixelox text add banner.pxo --in-place --name Solid --content "Fizz-Club" --font @brush --fit-width 1175 --color '#ffffff' --at 50%,9% --anchor t --anchor-to ink
pixelox text add banner.pxo --in-place --name Outline --content "Fizz-Club" --font @brush --fit-width 1175 --color transparent --stroke '#ffffff' --stroke-width 3 --at 50%,54% --anchor t --anchor-to ink
pixelox comp place banner.pxo product.png --in-place --as Bottle --size x1380 --at 47.5%,5% --anchor t
pixelox fx drop-shadow banner.pxo --in-place --layer Bottle --offset 0,0 --blur 50 --color '#7a0000' --opacity 35
pixelox export banner.pxo -o banner.png
```

Change first: the wordmark text (two short words; `--fit-width 1175` refits any length to
1175 px), then the bottle size so it covers the join between the two words and nothing
else. A formal copperplate script (Snell Roundhand) is too thin here: at this width it is
only 17% H tall, where a heavy brush script fills 35% H.

## 10. Variations and pairings

* Swap the script for a heavy condensed sans in caps (Impact) for a sport or energy-drink
  feel; keep solid above, outline below.
* Three rows instead of two: outline, solid, outline, with the product crossing all three
  (portrait 4:5).
* Palette swaps: cobalt `#0a3fd6` with white; black with acid yellow `#d7f000` type; cream
  `#f3ead7` with red type for a retro diner look.
* Works for sauces, beer, perfume, sneakers, any product with a strong upright shape and a
  recognizable logotype. Use it for web banners and channel headers.
* Pairs with T-paper-grain; replace the contour lines with T-pattern (dots, `fade=up`) or
  T-sunburst behind the bottle.

## 11. Pitfalls

* The product hides the wrong letters. It must sit on the gap between two words or over
  the middle of one long word; the first and last letters stay visible on both rows.
* Solid and outline rows at different widths look like two unrelated logos. Fit both to the
  same width and stack them with a gap of 5 to 8% H.
* An outline thicker than 1.5% of the font size competes with the solid row; a script with
  very thin hairlines makes both rows look frail and the outline row vanish: use a heavy
  brush script.
* Contour lines in white or black: they must be a dark shade of the ground (60% as light)
  so they read as texture, not as text or wires.
* A product photo with a neutral gray or blue cast looks cut out. Warm it toward the ground
  hue (`adjust color-balance --midtones 12,0,-8`) or pick a product whose color is related.
* Rebuilding a known brand's trade dress. A white script wordmark and ribbon on this exact red
  is one company's signature. Take the idea (what the object, the mark, and the ground do
  to each other) and build it with your own brand's colors, lettering, and shapes.
