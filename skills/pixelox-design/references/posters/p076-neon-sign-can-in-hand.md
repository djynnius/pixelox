---
id: p076
title: Can in hand before neon sign
genre: [drink-ad, product-ad]
mood: [retro, energetic, bold, cool]
style: [retro, photo-composite, neon]
format: portrait-a
archetype: A-hero-object
palette: ['#231f26', '#325379', '#7ea7c6', '#dad6d6', '#c25948', '#ffffff']
techniques: [T-neon-glow, T-fit-width, T-bokeh, T-cutout-drop-shadow, T-vignette, T-paper-grain, T-ghost-type]
needs: product-cutout
---

# p076: Can in hand before neon sign

## 1. Concept

A late-1980s point-of-sale card: a hand with red nails holds a dewy can up to the camera in
a dark club, and behind it the brand name burns as a huge blue neon sign, so big that only
three letters fit. A loud one-line claim in white condensed capitals runs across the top
and a quiet slogan in thin capitals across the bottom. The product is lit warm and sharp;
everything behind it is cold, blue, and out of focus, and that temperature split is the
whole design.

## 2. Format and grid

* Portrait, close to the A-series ratio. Text margins 10% W; the two text lines span the
  same width (10 to 91% W) and bracket the picture top and bottom.
* Zones: headline 5.7 to 11% H; neon sign 14 to 55% H, full bleed and cropped by both side
  edges; hand and can 19 to 100% H; slogan at 89.5% H.
* The can is the diagonal: it leans 14 degrees counter-clockwise, centered at 42% W, 50% H,
  and is 55% W wide. The hand enters from the bottom-right corner.
* Depth in three planes: blurred club lights (far), neon letters (middle, slightly soft),
  hand and can (near, sharp).

## 3. Layer stack

1. Ground: near-black with a violet cast `#231f26`.
2. Club blur: soft warm patches (red `#723130`, skin tones) at the left edge 55 to 100% H
   and bottom corners: a crowd far out of focus.
3. Neon sign: capital letters drawn as double blue tubes (outline lettering), cap height
   41% H, stroke about 1.6% W, color `#7ea7c6` in the tube core with a wide `#325379` glow
   that floods the upper two thirds of the picture. Three letters are visible; the first
   and last are cut by the edges.
4. Hand and can: a hand holding a white can with a round red-and-blue brand emblem and a navy
   italic wordmark, covered in condensation; the can spans 14 to 76% W, 19 to 84% H. Warm
   key light from the front left, red rim reflections from the nails.
5. Headline: white heavy condensed capitals, one line, 10 to 91% W, 5.7 to 11% H, ending in
   an exclamation mark. It sits on the dark strip above the neon.
6. Slogan: white thin geometric capitals, one line, 11 to 90% W, centered at 89.5% H, over
   the hand's shadow side.
7. Print grain and a slight vignette.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#231f26` | club darkness | 30% |
| `#325379` | neon glow | 25% |
| `#7ea7c6` | neon tube core | 12% |
| `#dad6d6` | can white, type | 16% |
| `#c25948` | reds: emblem, nails, base of the can | 12% |
| `#6e362e` | skin in shadow | 5% |

Cold blue ground against a warm red-and-skin foreground: complementary temperature, with
white shared by the product and the type.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Headline | 4 words, a popularity claim naming a place, ending in "!"; about 21 characters | caps | heavy condensed grotesque (Oswald 700, Anton / Impact) | 400 (the face is heavy) | 5.3% | -10 | - | center | `#ffffff` | 10 to 91% W, top 5.7% |
| Slogan | 6 words, about 31 characters | caps | thin geometric sans, tight (Jost 300, Poppins 300 / Avenir Next) | 400 | 2.3% | -30 | - | center | `#ffffff` | 11 to 90% W, 89.5% |
| Neon sign | `[BRAND]`, 5 letters, 3 visible | caps | rounded heavy sans drawn as tubes (Fredoka, Baloo 2 / Arial Rounded MT Bold) | outline | 41% | 0 | - | center, cropped | `#7ea7c6` | 14 to 55% H |

Loud condensed on top, whisper-thin at the bottom, both fitted to the same width so they
form a frame.

## 6. Imagery

One flash photograph: a hand holding the product toward the lens at chest height in a dark
venue. The background is a real neon sign of the brand, shot out of focus. For a build:
a product cutout (or a hand-with-product cutout) over a T-neon-glow sign made from live
text, with T-bokeh warm blobs for the crowd. Tilt the product and let it overlap at least
one neon letter completely.

## 7. Effects and textures

* **T-neon-glow**: an outlined word (`--color transparent --stroke`) for the tubes, a tight
  `fx outer-glow`, and a blurred filled copy in `screen` mode for the flood of blue.
  Blur the tube layer itself slightly (sigma 2) so it sits behind the product in depth.
* **T-bokeh** / blobs: two or three large red-brown glows at the left and bottom edges,
  sigma 60.
* **T-cutout-drop-shadow**, glow variant: a faint warm rim so the product separates from
  the blue.
* **T-fit-width** on both text lines (875 px and 850 px on a 1080 px canvas).
* **T-vignette** at 55% and **T-paper-grain** (amount 16, overlay, 35%) for the printed-card
  feel.
* **T-ghost-type** in [techniques.md](../techniques.md) is the tested library version of the faint, defocused, or outlined giant word; start from it when reusing this idea.

## 8. Hierarchy and eye path

1. The can and its emblem (sharp, warm, central).
2. The headline above (large white type on dark).
3. The neon letters, recognized as the brand a moment later.
4. The slogan at the bottom.

Cap heights: neon 41 : headline 5.3 : slogan 2.3. The neon is the biggest lettering by far
but reads last because it is cropped, soft, and behind the subject.

## 9. Build recipe

`product.png` is your image here: the product alone, or a hand holding it.

```bash
pixelox render js scripts/blobs.js -o club.png --size 1080x1528 --seed 3 --param bg=transparent --param colors='#8a2f2a,#b0563a,#5a2030' --param at='0.02,0.72;0.1,0.98;0.95,1.02' --param size=0.4 --param alpha=0.8
pixelox new -o card.pxo --size 1080x1528 --fill '#1b1820'
pixelox comp place card.pxo club.png --in-place --as Club --fit cover
pixelox filter gaussian card.pxo --in-place --layer Club --sigma 60
pixelox text add card.pxo --in-place --name Flood --content "SODA" --font "Arial Rounded MT Bold" --size 56% --color '#2f6fc0' --at 50%,35% --anchor c
pixelox layer rasterize card.pxo --in-place --layer Flood
pixelox filter gaussian card.pxo --in-place --layer Flood --sigma 70
pixelox layer blend card.pxo --in-place --layer Flood --mode screen
pixelox text add card.pxo --in-place --name Tubes --content "SODA" --font "Arial Rounded MT Bold" --size 56% --color transparent --stroke '#9cc8ea' --stroke-width 16 --at 50%,35% --anchor c
pixelox fx outer-glow card.pxo --in-place --layer Tubes --blur 34 --spread 10 --color '#2f7fe0' --opacity 90
pixelox comp place card.pxo product.png --in-place --as Can --size x1150 --at 43%,55% --anchor c
pixelox transform rotate card.pxo --in-place --layer Can --degrees -14
pixelox fx outer-glow card.pxo --in-place --layer Can --blur 40 --color '#ffb090' --opacity 25
pixelox layer add card.pxo --in-place --name Vignette
pixelox fill gradient card.pxo --in-place --layer Vignette --kind radial --from 45%,48% --to 45%,120% --stops '#0a080e00@0.45,#0a080e8c@1'
pixelox text add card.pxo --in-place --name Headline --content "LOVED IN EVERY TOWN!" --font Impact --size 7% --color '#ffffff'
pixelox transform scale card.pxo --in-place --layer Headline --size 875x
pixelox transform move card.pxo --in-place --layer Headline --to 50.5%,5.7% --anchor t
pixelox text add card.pxo --in-place --name Slogan --content "MADE FOR THE LONGEST SUMMER NIGHTS" --font "Avenir Next" --weight 400 --size 3.2% --tracking -30 --color '#ffffff'
pixelox transform scale card.pxo --in-place --layer Slogan --size 850x
pixelox transform move card.pxo --in-place --layer Slogan --to 50.5%,89.5% --anchor c
pixelox layer add card.pxo --in-place --name Grain --fill '#808080'
pixelox filter add-noise card.pxo --in-place --layer Grain --amount 16 --monochrome
pixelox layer blend card.pxo --in-place --layer Grain --mode overlay
pixelox layer opacity card.pxo --in-place --layer Grain --value 35
pixelox export card.pxo -o card.png
```

Change first: the neon word (4 or 5 letters; keep the size so that only 2 to 3 letters
fit), the headline, then the neon color (`#ff2bd6` pink or `#ff6a1a` orange with a matching
flood color).

## 10. Variations and pairings

* Bar, club night, or concert flyer: replace the can with a performer cutout and keep the
  neon word and the two framing lines.
* Warm and cold swapped: orange neon `#ff8a2a` behind a blue-lit product.
* Sharp, graphic version: no blur on the tubes, flat black ground, product in black and
  white (T-mono-on-color).
* Square: neon 10 to 60% H, product 20 to 100% H, drop the slogan and keep the headline.
* Pairs with T-lens-flare (`movie`, the blue streak) on a tube corner and T-motion-streak
  on the product for speed.

## 11. Pitfalls

* The whole neon word visible and centered: it becomes the headline and fights the real
  one. Crop it hard; two to three letters are enough when the product repeats the name.
* Neon tubes fully saturated blue look flat. The tube core is pale (`#9cc8ea`); the glow
  and the flood carry the color.
* Product lit cold on a cold ground disappears. Warm it (`adjust color-balance --midtones
  10,0,-10`) or give it the faint warm rim.
* Headline over the neon letters: unreadable. Keep a dark strip (the top 12% H) free of
  tubes by placing the word's cap tops at 14% H.
* Thin slogan on a busy area. Put it over the darkest part of the foreground or add a
  shadow (offset 0,2, blur 10, 50%).
