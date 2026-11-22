---
id: p075
title: Macro can on ice, retro print ad
genre: [drink-ad, product-ad, editorial]
mood: [retro, fresh, bold, cool]
style: [retro, photo-composite]
format: portrait-4x5
archetype: A-macro-crop
palette: ['#e3dfda', '#bab5ba', '#2a2d4f', '#d03233', '#7b8592', '#ffffff']
techniques: [T-frame-border, T-color-grade, T-paper-grain, T-cutout-drop-shadow, T-ice-cubes, T-condensation]
needs: product-cutout
---

# p075: Macro can on ice, retro print ad

## 1. Concept

A 1980s magazine page: one extreme close-up of a cold drink can lying diagonally in ice,
so large that it does not fit in the frame, with a three-line slogan in thin white capitals
tucked into the bottom-right corner. The can's own graphics (a big round emblem and a
slanted wordmark) become the headline, so the ad needs almost no type. A narrow cream paper
margin frames it on three sides.

## 2. Format and grid

* Portrait 4:5. Paper margin 4.5% W left and right and 3% H at the top; the photo bleeds
  off the bottom edge.
* The can is the diagonal: its axis leans 18 degrees counter-clockwise from vertical, its
  top rim is cut by the top-left of the frame and its body runs out of the bottom-right.
  It fills about 80% of the photo; ice fills the corners (top right, left edge, bottom
  left).
* The emblem on the can is centered at 47% W, 44% H and is 62% W across: it is the real
  focal point.
* Slogan block: three centered lines on an axis at 64% W, 87 to 98% H, over the darkest,
  reddest part of the can.

## 3. Layer stack

1. Paper: warm cream `#e8e0d6`, full canvas (shows only as the margin).
2. Ice: crushed ice and clear cubes in blue-gray `#7b8592` to white, filling the photo
   area `4.5%,3%,90.5% x 97%`; large cubes at the top right, the left edge, and the
   bottom left.
3. Can: white `#e3dfda` aluminum can, lower third printed red `#d03233`, lying at -18
   degrees, about 85% W wide, cropped by the frame at the top-left and bottom-right. Lit
   from the upper left; the lower right falls into deep red-brown shadow `#45222f`.
4. Emblem on the can: the brand's large two-color roundel (red above, navy `#2a2d4f`
   below) with the wordmark in navy heavy italic capitals across its white middle,
   following the can's tilt.
5. Condensation: hundreds of water beads in three sizes over the whole can, each with a
   white highlight and a dark rim; the biggest are 3% W across.
6. Slogan: three lines of thin white capitals, bottom right.
7. Print character: slight yellowing, soft contrast, fine grain.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#e3dfda` | can white, paper | 36% |
| `#bab5ba` | shaded white, ice mid-tones | 18% |
| `#2a2d4f` | navy of the emblem and wordmark | 16% |
| `#d03233` | red of the emblem and can base | 22% |
| `#7b8592` | blue-gray ice shadows | 8% |

Red, white, and navy: a three-color flag scheme, softened by the cream of old paper so the
white is never pure.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Slogan | 3 lines: `[BRAND]` plus a period; 3 words; 3 words plus a period (6, 13, and 17 characters) | caps | thin, wide geometric sans, tightly set (Jost 300, Poppins 300 / Avenir Next) | 400 | 2.4% (2.9% in a narrower face, to keep the block 50 to 54% W wide) | -30 | 112% | center | `#ffffff` | axis 64% W; lines at 88.7%, 92.6%, 96.4% |
| Wordmark on can | `[BRAND]`, 5 letters (part of the product) | caps | heavy wide italic sans (Montserrat 900 Italic / Avenir Next) | 900 italic | 9% | -20 | - | along the can, -18 degrees | `#2a2d4f` | centered 47%, 44% |

One typeset block, small. Geometric capitals with round O and C, set tight enough that
the letters almost touch: the period look.

## 6. Imagery

A single macro product photograph: studio flash from the upper left, the can wet and
frosted, ice around it. The crop is the idea: the product is bigger than the page, tilted,
and cut on two sides, so it feels close enough to grab. For a build, a product cutout is
scaled until its body is 80 to 90% W wide, rotated -18 degrees, and laid over a cold,
marbled ground with rendered ice cubes and beads of condensation; the paper margin is
drawn last so it trims the oversized cutout. The ice and the beads are rendered, not
photographed: clean cubes and round drops, not the crushed ice of the original.

## 7. Effects and textures

* **T-frame-border**: the margin is three cream bars drawn on top (left, right, top), not a
  rectangle underneath, so they crop the photo and the oversized product.
* Crushed ice ground: `fill noise --as-layer` in blue-gray to white (`--scale 20%`,
  `--warp 56`, `--contrast 14`).
* **T-ice-cubes**: `render ice`, three groups of large cubes (20% of the short side,
  `--frost 40`): two before the can, at the top right and the left edge, and one after it
  at the bottom left, so the can lies in the ice, not on it.
* **T-condensation**: `render droplets --layer Can --density 70 --trickles 0 --mist 45`,
  run after the can is placed and rotated. No trickles: the can is lying down.
* **T-cutout-drop-shadow**, soft, under the can onto the ice.
* **T-color-grade** for age: lifted blacks and a warm cast (`adjust curves` with the black
  point at 14; `adjust color-balance` 8,0,-14 in the highlights).
* **T-paper-grain**: `filter grain --as-layer --amount 15`, last.
* A soft dark gradient in the bottom-right corner keeps the white slogan readable.

## 8. Hierarchy and eye path

1. The emblem and wordmark on the can (62% W across, highest color contrast).
2. Along the can's diagonal down to the red base.
3. The white slogan sitting on that red, bottom right.

The printed wordmark's cap height is 9% H and the slogan's 2.4% H: the product is the
headline at almost four times the size of the type.

## 9. Build recipe

`product.png` is your image here. Scale it so the body is 80 to 90% of the canvas width
(`--size x3300` suits the narrow stand-in bottle; a can cutout cropped tight needs about
`x1900`).

```bash
pixelox new -o ad.pxo --size 1080x1350 --fill '#e8e0d6'
pixelox fill noise ad.pxo --in-place --as-layer --name Ice --seed 5 --scale 20% --octaves 5 --warp 56 --contrast 14 --stops '#6f8296,#f6f8fa'
pixelox render ice ad.pxo --in-place --name IceRight --at 90%,28% --count 6 --size 20% --spread 26% --frost 40 --seed 11
pixelox render ice ad.pxo --in-place --name IceLeft --at 6%,62% --count 5 --size 20% --spread 24% --frost 40 --seed 14
pixelox comp place ad.pxo product.png --in-place --as Can --size x3300 --at 50%,52% --anchor c
pixelox transform rotate ad.pxo --in-place --layer Can --degrees -18
pixelox fx drop-shadow ad.pxo --in-place --layer Can --offset 30,40 --blur 80 --color '#1c2233' --opacity 50
pixelox render droplets ad.pxo --in-place --layer Can --density 70 --trickles 0 --mist 45 --seed 9
pixelox render ice ad.pxo --in-place --name IceFront --at 10%,90% --count 3 --size 21% --spread 14% --frost 40 --seed 14
pixelox layer add ad.pxo --in-place --name CornerShade
pixelox fill gradient ad.pxo --in-place --layer CornerShade --kind radial --from 72%,96% --to 72%,60% --stops '#2a0f1acc@0,#2a0f1a00@1'
pixelox adjust curves ad.pxo --in-place --points '0,14 64,62 128,130 192,200 255,246' --as-layer --name Faded
pixelox adjust color-balance ad.pxo --in-place --highlights 8,0,-14 --as-layer --name Warm
pixelox shape rect ad.pxo --in-place --name MarginLeft --rect 0,0,4.5%x100% --fill '#e8e0d6'
pixelox shape rect ad.pxo --in-place --name MarginRight --rect 95.5%,0,4.5%x100% --fill '#e8e0d6'
pixelox shape rect ad.pxo --in-place --name MarginTop --rect 0,0,100%x3% --fill '#e8e0d6'
pixelox text add ad.pxo --in-place --name Slogan --content $'BRAND.\nMADE FOR THE\nLONGEST SUMMERS.' --font @geometric-sans --weight 400 --fit-width 540 --tracking -30 --leading 112% --align center --color '#ffffff' --at 64%,92.4% --anchor c --anchor-to ink
pixelox fx drop-shadow ad.pxo --in-place --layer Slogan --offset 0,2 --blur 10 --color '#200a10' --opacity 45
pixelox filter grain ad.pxo --in-place --as-layer --name Grain --amount 15
pixelox export ad.pxo -o ad.png
```

Change first: the product scale and angle (the logo on the pack should land near the
center of the canvas), then the slogan lines, then how old it looks (the `Faded` curve and
the grain amount). Ice and droplets show what is behind them when their command runs:
after changing the product, its scale, or its angle, run the recipe again from the top.
If a cube covers the logo, change that group's `--seed`.

## 10. Variations and pairings

* Clean modern version: white margin on all four sides, no fade, no grain, slogan in a
  heavy grotesque.
* Other "bigger than the page" products: a chocolate bar snapped diagonally, a lipstick, a
  watch face, a sneaker sole, a burger cross-section.
* Swap ice for the product's own world: sand for sunscreen, coffee beans for an espresso
  can, citrus slices for a soda.
* Landscape billboard: can at -60 degrees running corner to corner, slogan bottom left.
* Pairs with T-halftone-portrait at a fine cell (`--cell 5`) on the whole photo for a
  newsprint look, and T-light-leak in orange from the top-left corner.

## 11. Pitfalls

* Showing the whole product. The effect needs at least two sides cropped by the frame;
  scale up until it hurts, then check that the logo is still fully visible.
* A straight, upright pack looks like a catalog shot. Tilt it 12 to 25 degrees.
* The slogan placed on the white part of the can or on ice: white type vanishes. Put it on
  the darkest area and add the corner shade.
* Pure white paper and pure white highlights make it look new. Use cream `#e8e0d6` for the
  margin and lift the blacks.
* Beads added before the can is rotated, or kept after it is swapped: they no longer
  match the pack. `render droplets` goes after `transform rotate`; run it again when the
  product changes.
* Ice cubes on a flat pale ground look like gray stickers. The marbled blue-gray noise
  under them gives them something to refract.
