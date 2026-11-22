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
   area `4.5%,3%,90.5% x 97%`; soft focus.
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
marbled ground; the paper margin is drawn last so it trims the oversized cutout.

## 7. Effects and textures

* **T-frame-border**: the margin is three cream bars drawn on top (left, right, top), not a
  rectangle underneath, so they crop the photo and the oversized product.
* Ice: `clouds.js` in blue-gray to white (`scale=5`, `warp=1.4`, `contrast=1.8`) blurred a
  little; translucent `rect` shards from `particles.js` over it for cube facets.
* Condensation: `particles.js` rings and dots in white over the product, clipped to it.
* **T-cutout-drop-shadow**, soft, under the can onto the ice.
* **T-color-grade** for age: lifted blacks and a warm cast (`adjust curves` with the black
  point at 14; `adjust color-balance` 8,0,-14 in the highlights).
* **T-paper-grain**, amount 18, overlay, 35%.
* A soft dark gradient in the bottom-right corner keeps the white slogan readable.
* **T-ice-cubes** in [techniques.md](../techniques.md) is the tested library version of the ice cubes (`particles.js` with `shape=cube` draws fuller cubes than the shards here); start from it when reusing this idea.
* **T-condensation** in [techniques.md](../techniques.md) is the tested library version of the beads of condensation on the product (`particles.js` with `shape=drop`); start from it when reusing this idea.

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
pixelox render js scripts/clouds.js -o ice.png --size 1080x1350 --seed 5 --param lo='#6f8296' --param hi='#f6f8fa' --param scale=5 --param octaves=5 --param warp=1.4 --param contrast=1.8
pixelox render js scripts/particles.js -o cubes.png --size 1080x1350 --seed 3 --param shape=rect --param colors='#ffffff,#dbe7f0,#b6c6d6' --param count=60 --param min=5 --param max=11 --param jitter=180 --param amin=0.25 --param amax=0.6
pixelox render js scripts/particles.js -o beads.png --size 1080x1350 --seed 9 --param shape=ring --param colors='#ffffff,#eef3f7' --param count=260 --param min=0.5 --param max=2.6 --param amin=0.5 --param amax=0.95
pixelox new -o ad.pxo --size 1080x1350 --fill '#e8e0d6'
pixelox comp place ad.pxo ice.png --in-place --as Ice --fit cover
pixelox filter gaussian ad.pxo --in-place --layer Ice --sigma 3
pixelox comp place ad.pxo cubes.png --in-place --as Cubes --fit cover
pixelox comp place ad.pxo product.png --in-place --as Can --size x3300 --at 50%,52% --anchor c
pixelox transform rotate ad.pxo --in-place --layer Can --degrees -18
pixelox fx drop-shadow ad.pxo --in-place --layer Can --offset 30,40 --blur 80 --color '#1c2233' --opacity 50
pixelox comp place ad.pxo beads.png --in-place --as Beads --fit cover
pixelox layer clip ad.pxo --in-place --layer Beads
pixelox layer opacity ad.pxo --in-place --layer Beads --value 70
pixelox layer add ad.pxo --in-place --name CornerShade
pixelox fill gradient ad.pxo --in-place --layer CornerShade --kind radial --from 72%,96% --to 72%,60% --stops '#2a0f1acc@0,#2a0f1a00@1'
pixelox adjust curves ad.pxo --in-place --points '0,14 64,62 128,130 192,200 255,246' --as-layer --name Faded
pixelox adjust color-balance ad.pxo --in-place --highlights 8,0,-14 --as-layer --name Warm
pixelox shape rect ad.pxo --in-place --name MarginLeft --rect 0,0,4.5%x100% --fill '#e8e0d6'
pixelox shape rect ad.pxo --in-place --name MarginRight --rect 95.5%,0,4.5%x100% --fill '#e8e0d6'
pixelox shape rect ad.pxo --in-place --name MarginTop --rect 0,0,100%x3% --fill '#e8e0d6'
pixelox text add ad.pxo --in-place --name Slogan --content $'BRAND.\nMADE FOR THE\nLONGEST SUMMERS.' --font "Avenir Next" --weight 400 --size 3.3% --tracking -30 --leading 112% --align center --color '#ffffff'
pixelox transform scale ad.pxo --in-place --layer Slogan --size 540x
pixelox transform move ad.pxo --in-place --layer Slogan --to 64%,92.4% --anchor c
pixelox fx drop-shadow ad.pxo --in-place --layer Slogan --offset 0,2 --blur 10 --color '#200a10' --opacity 45
pixelox layer add ad.pxo --in-place --name Grain --fill '#808080'
pixelox filter add-noise ad.pxo --in-place --layer Grain --amount 18 --monochrome
pixelox layer blend ad.pxo --in-place --layer Grain --mode overlay
pixelox layer opacity ad.pxo --in-place --layer Grain --value 35
pixelox export ad.pxo -o ad.png
```

Change first: the product scale and angle (the logo on the pack should land near the
center of the canvas), then the slogan lines, then how old it looks (the `Faded` curve and
the grain amount).

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
* Beads drawn at one size look like a dot pattern. Use a wide size range (0.5 to 2.6% of
  the short side) and clip them to the product.
