---
id: p019
title: White sculpture bottle on red studio
genre: [drink-ad, product-ad]
mood: [minimal, elegant, bold, calm]
style: [3d-render, minimal]
format: portrait-a
archetype: A-hero-object
palette: ['#af1012', '#8e0404', '#e9e6e2', '#b5b4af', '#ffffff']
techniques: [T-spotlight, T-vignette, T-cutout-drop-shadow, T-paper-grain, T-condensation, T-floor-reflection]
needs: product-cutout
---

# p019: White sculpture bottle on red studio

## 1. Concept

A limited-edition announcement: the product is shown as a white, faceted sculpture of
itself, beaded with clear droplets, standing alone on a seamless deep-red studio floor with
a soft white halo and a faint reflection. Two small centered lines above it say what it is;
the brand mark and legal lines sit quietly at the bottom. The idea is "museum piece":
strip the product of its color, light it like an object in a gallery, and let the brand
color be the room.

## 2. Format and grid

* Portrait A-series. Strictly symmetric on the 50% W axis; side margins 13% W for the fine
  print, far more for everything else.
* Zones: header 11 to 15.5% H; bottle 21 to 77% H; reflection on the floor 77 to 95% H;
  brand mark 85.5 to 89% H (inside the reflection); fine print 91 to 96% H.
* The bottle is 28% W wide and 56% H tall, so 36% W of empty red remains on each side.

## 3. Layer stack

1. Ground: deep red `#af1012`, full canvas.
2. Corner shade: darker red `#8e0404` toward all four corners (the top-left is darkest).
3. Floor light: a soft lighter red-pink pool `#af5c58` at 35 to 65% W, 77 to 96% H, under
   the bottle; there is no visible horizon line, the wall and floor are one sweep.
4. Reflection: the bottle mirrored below its base, blurred, fading out by 95% H.
5. Halo: soft white glow around the bottle, strongest at the shoulders and cap, about
   6% W wide, 45% opacity.
6. Bottle: `36%,21%,28% x 56%`, near-white `#e9e6e2` with light gray facets `#b5b4af`:
   a low-polygon version of a curvy glass bottle, covered in small clear droplets.
7. Mark on the bottle: script wordmark in the ground red, 20% W wide, across the bottle's
   label zone at 45 to 53% H.
8. Header line 1: white bold caps, centered, 12% H, 39% W wide.
9. Header line 2: white light caps, centered, 14.5% H, 21% W wide.
10. Brand mark: white script wordmark, 14% W wide, centered at 87.3% H.
11. Fine print: two centered lines of tiny tracked caps at 91.5% and 93% H, 74% W wide,
    white at 70% opacity; a web address at 95.5% H, lowercase.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#af1012` | studio red | 64% |
| `#8e0404` | corner shade | 20% |
| `#e9e6e2` | bottle highlights, halo | 8% |
| `#b5b4af` | bottle facets in shadow | 5% |
| `#ffffff` | all type | 3% |

Red plus neutral white and gray: the product gives up its own color so that the room can
carry the brand color.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Header 1 | number + "YEARS LIMITED EDITION", 4 words | caps | wide geometric sans (Montserrat 700 / Avenir Next) | 700 | 1.3% | 30 | - | center | `#ffffff` | 50%, 12% |
| Header 2 | material + product, 2 words | caps | same | 300 | 1.1% | 30 | - | center | `#ffffff` | 50%, 14.5% |
| Mark on bottle | `[BRAND]` script wordmark | title | formal script (Great Vibes / Snell Roundhand) | 900 | 3.5% | 0 | - | center | `#b00f10` | 50%, 49% |
| Brand mark | `[BRAND]` script wordmark | title | same script | 900 | 2.2% | 0 | - | center | `#ffffff` | 50%, 87.3% |
| Fine print | 2 lines of legal text, about 90 and 50 characters | caps | geometric sans (Montserrat 400 / Avenir Next) | 400 | 0.6% | 40 | 180% | center | `#ffffff` at 70% | 50%, 91.5% and 93% |
| Web address | one URL | lower | same | 400 | 0.6% | 40 | - | center | `#ffffff` at 70% | 50%, 95.5% |

One sans family in two weights plus the brand script. The header is tiny for a poster
(1.3% H): it is a museum label, not a headline.

## 6. Imagery

One 3D-rendered object: a bottle rebuilt from flat triangular facets in matte white, with
glossy water droplets scattered over it and a red script mark wrapped around the middle.
Soft frontal studio light, so the facets differ only slightly in tone. For a build from a
photo, a product cutout is turned into "plaster" with a gradient map from mid gray to
white, lit with a white halo (T-cutout-drop-shadow, glow variant), beaded with clear
droplets (`render droplets`), and mirrored on the floor (`fx reflection`). True faceting
needs a 3D model or the `filter cutout` art filter on the cutout.

## 7. Effects and textures

* **T-spotlight**, inverted in emphasis: no bright pool behind the product; instead a
  floor light under it and a white halo on it.
* **T-vignette** in a dark shade of the ground (`#5a0202`), starting at 45% of the radius.
* Plaster treatment: `adjust bw`, then `adjust gradient-map` with stops from `#8f8d8a` to
  `#ffffff`.
* Halo: `fx outer-glow`, blur 70, white, 50%.
* **T-floor-reflection**: `fx reflection --length 32% --opacity 30 --blur 10` on the bottle.
  It starts at the base and fades out by 95% H on its own, so there is no flipped copy and
  no fade layer. The cutout is trimmed first (`transform trim`) so the reflection touches
  the base.
* **T-condensation**: `render droplets --layer Bottle --density 40 --mist 0 --trickles 0
  --refract off`. Clear beads without a lens effect suit a matte white object; they are
  clipped to the bottle.
* **T-paper-grain**: `filter grain --as-layer --amount 8`, last, to hide banding in the red.

## 8. Hierarchy and eye path

1. The white bottle (the only light object, 56% H tall, dead center).
2. The red mark on it.
3. Up to the two header lines.
4. Down the reflection to the brand mark; fine print last.

Cap heights: bottle mark 3.5 : brand mark 2.2 : header 1.3 : fine print 0.6. The bottle is
43 times taller than the header's capitals.

## 9. Build recipe

`product.png` is your image here: a tall product cutout, front view. The `transform trim`
line removes its transparent border and writes `tight.png`, which is the file placed.

```bash
pixelox new -o ad.pxo --size 1080x1528 --fill '#af1012'
pixelox layer add ad.pxo --in-place --name Corners
pixelox fill gradient ad.pxo --in-place --layer Corners --kind radial --from 52%,46% --to 52%,118% --stops '#5a020200@0.4,#5a0202b3@1'
pixelox layer add ad.pxo --in-place --name FloorLight
pixelox fill gradient ad.pxo --in-place --layer FloorLight --kind radial --from 50%,84% --to 50%,100% --stops '#c9706c99@0,#c9706c00@1'
pixelox transform trim product.png -o tight.png
pixelox comp place ad.pxo tight.png --in-place --as Bottle --size x846 --at 50%,77% --anchor b
pixelox adjust bw ad.pxo --in-place --layer Bottle
pixelox adjust gradient-map ad.pxo --in-place --layer Bottle --stops '#8f8d8a@0,#d6d3cf@0.5,#ffffff@1'
pixelox fx outer-glow ad.pxo --in-place --layer Bottle --blur 70 --color '#ffffff' --opacity 50
pixelox fx reflection ad.pxo --in-place --layer Bottle --length 32% --opacity 30 --blur 10
pixelox render droplets ad.pxo --in-place --layer Bottle --density 40 --mist 0 --trickles 0 --refract off
pixelox text add ad.pxo --in-place --name Mark --content "Brand" --font @script-formal --weight 900 --fit-width 104 --color '#b00f10' --at 50.7%,48.7% --anchor c --anchor-to ink
pixelox text add ad.pxo --in-place --name Header1 --content "50 YEARS SPECIAL EDITION" --font @geometric-sans --weight 700 --size 1.8% --tracking 30 --color '#ffffff' --at 50%,12% --anchor c
pixelox text add ad.pxo --in-place --name Header2 --content "CERAMIC BOTTLES" --font @thin-sans --size 1.5% --tracking 30 --color '#ffffff' --at 50%,14.5% --anchor c
pixelox text add ad.pxo --in-place --name BrandMark --content "Brand" --font @script-formal --weight 900 --size 3.4% --color '#ffffff' --at 50%,87.3% --anchor c
pixelox text add ad.pxo --in-place --name Legal --content $'BRAND AND THE BOTTLE SHAPE ARE REGISTERED TRADEMARKS OF YOUR COMPANY NAME\nSPECIAL EDITION DESIGNED BY YOUR STUDIO NAME' --font @geometric-sans --weight 400 --size 0.85% --tracking 40 --leading 180% --align center --color '#ffffffb3' --at 50%,92.2% --anchor c
pixelox text add ad.pxo --in-place --name Site --content "www.example.com" --font @geometric-sans --weight 400 --size 0.85% --tracking 40 --color '#ffffffb3' --at 50%,95.5% --anchor c
pixelox filter grain ad.pxo --in-place --as-layer --name Grain --amount 8
pixelox export ad.pxo -o ad.png
```

Change first: the ground red to your brand color (and the two shades derived from it:
corner `#5a0202`, floor light `#c9706c`), then the two header lines, then the width of
`Mark` (104 px suits the narrow stand-in; use 75% of your product's width). The droplets
are made for the bottle that is there when the command runs: after swapping the product,
run the recipe again from the top.

## 10. Variations and pairings

* Black studio with a gold-mapped product (`#3a2a0a` to `#f6d365` to `#fff6c8`) for a
  premium or anniversary edition; white studio `#efefef` with a product mapped to one
  saturated color for a design-award look.
* Art-filter the cutout before mapping: `filter cutout` for facets, `filter pencil` for a
  drawn edition, T-halftone-portrait for a print edition.
* Suits anniversaries, collector's editions, packaging redesign reveals, awards entries,
  portfolio pieces.
* Landscape: bottle at 30% W, the header and brand mark left-aligned at 55% W.
* Pairs with T-lens-flare (`35mm`, small) on the cap, and T-arc-type for an edition seal
  beside the base.

## 11. Pitfalls

* The gradient map goes from black to white: the product looks like a gray photo, not a
  white object. Start the map at a mid gray (`#8f8d8a`) so the darkest facet is still light.
* A halo in pure white at high opacity turns into a sticker outline. Keep it wide (blur 60
  to 80) and at or below 50%.
* The reflection as strong as the product. It must be under 35% opacity, blurred, and gone
  before the fine print (`--opacity 30`, `--length` about a third of the bottle).
* A gap between the base and the reflection: the cutout still has a transparent border.
  Keep the `transform trim` line.
* Header set large. Past 2% H cap height it reads as a sale poster; the calm comes from
  the 40:1 ratio between object and type.
* A ground red that is too bright (`#e61a2b`) kills the white: the studio red must be deep
  (lightness about 37%) with darker corners.
* Rebuilding a known brand's trade dress. A white script wordmark and ribbon on this exact red
  is one company's signature. Take the idea (what the object, the mark, and the ground do
  to each other) and build it with your own brand's colors, lettering, and shapes.
