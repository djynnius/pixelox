---
id: p125
title: Blue cell mosaic dissolving into white
genre: [corporate, social-post]
mood: [cool, fresh, minimal, calm]
style: [geometric, flat, minimal]
format: square
archetype: A-geometric-cluster
palette: ['#ffffff', '#075cb9', '#1d78d3', '#082e62', '#4aaaf0', '#111111']
techniques: [T-stencil-poster, T-paper-grain]
needs: nothing
---

# p125: Blue cell mosaic dissolving into white

## 1. Concept

A square background template: a mass of irregular, outlined cells in five blues fills the
right edge and breaks up into loose fragments as it moves left, leaving the left half of
the canvas white for a two-line title. It works because of one clear gradient of density
(solid on the right, crumbs in the middle, nothing on the left) and because the color is
held to a single hue, so the busy texture still reads as one calm shape beside the text.

## 2. Format and grid

* Square (1 : 1). The canvas splits by density, not by a line: 0 to 45% W is empty white;
  45 to 75% W is the break-up zone (islands of 3 to 12% W with white between them);
  75 to 100% W is solid mosaic that bleeds off the top, right, and bottom edges.
* The mass's left contour wanders between 62% and 80% W: it bulges left at 10 to 20% H
  and 45 to 60% H, and pulls back at 30 to 40% H and 65 to 75% H. Never a straight edge.
* Text block: left edge at 13% W, optically centered at 50.5% H, 18% W wide. The gap
  from the text's right edge (31% W) to the first fragment (45% W) is 14% W: about the
  width of the text block itself.
* Margins for anything added later: 13% W on the left, the same top and bottom.

## 3. Layer stack

1. Ground: white `#ffffff`.
2. Cell field: the whole canvas covered with organic, rounded patches, 5 to 20% W
   across, in flat blues. Mid blues `#075cb9` and `#1d78d3` take about 80% of the
   patches; light azure `#2a8ede` and pale `#4aaaf0` appear as small accents (1 patch in
   8); deep navy `#082e62` appears only at the top right and far right edge.
3. Tone variation: a second, differently shaped patch field in grays, blended in
   soft-light, so cells overlap into more shapes and shades than the base map has.
4. Cover: white, hiding the cell field wherever the mosaic should be absent. Its edge is
   hard and irregular; it is solid left of 45% W, absent right of 80% W, and broken into
   holes and islands between.
5. Outlines: a thin dark line (1 to 2 px at 1080, navy to black) around every cell and
   every fragment, including the edge against white.
6. Title line 1: one word, near-black `#111111`, bold condensed sans, at 13% W, cap top
   47.2% H, baseline 51% H.
7. Title line 2: one word, lowercase, dark gray `#333333`, light geometric sans, at
   13% W, x-height 52.3 to 54.2% H. Both lines are 17.5 to 18% W wide.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#ffffff` | ground | 63% |
| `#075cb9` | main cell blue | 15% |
| `#1d78d3` | second cell blue | 13% |
| `#082e62` | deep cells and outlines | 4% |
| `#4aaaf0` | pale accent cells (with `#2a8ede`) | 3% |
| `#111111` | title | 2% |

Monochromatic blue on white. The blues stay within a narrow lightness band (35 to 55%),
with a few lighter and darker exceptions for sparkle; the outline does the separating,
not the contrast between neighbors.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Title | 1 word, 6 to 8 letters | title | condensed bold sans (Roboto Condensed Bold, PT Sans Narrow Bold / Avenir Next Condensed) | 700 | 3.6% | 0 | - | left | `#111111` | 13%, center 49.2% |
| Sub-title | 1 word, 8 to 10 letters | lower | light geometric sans (Raleway Light, Jost Light / Avenir Next) | 300 | 1.9% (x-height) | 0 | - | left | `#333333` | 13%, center 53.3% |

Weight contrast in place of size contrast: both lines are the same width, so the bold
condensed word needs larger letters than the light wide word to span it (font sizes
5.6% and 3.4% H). This "heavy narrow over light wide, same width" lockup is the only
typographic idea, and it is enough.

## 6. Imagery

No photo or illustration. The picture is generated: a field of cells that look like
flakes of paint, shards of sea glass, or a map of shallows. Cell edges are smooth curves
with occasional corners; cells never have straight sides or equal sizes. The fragments
in the break-up zone are whole cells or groups of 2 to 4 cells that have drifted off,
each still outlined, which makes the mass look as if it is coming apart, or assembling.

## 7. Effects and textures

* Cells (no library ID): two renders of `scripts/clouds.js` with low `octaves`
  (2 to 3: smooth, blobby noise), each turned into flat patches with a hard-banded
  `adjust gradient-map`. The banding rule is the one from **T-stencil-poster**: every
  color appears twice in `--stops`, at the start and the end of its band. Band colors
  alternate dark and light so neighbors differ.
* Dissolve (no library ID): a third noise render plus a left-to-right white-to-black
  gradient in `linear-light` mode (which adds the gradient to the noise), merged and cut
  with `adjust threshold` into a pure black-and-white layer; set to `screen`, its black
  part vanishes and its white part hides the cells. The gradient's start and end
  (`--from 18%,0 --to 96%,0`) set where the break-up zone lies.
* Outlines: `layer stamp`, `filter find-edges`, `adjust bw`, then `adjust levels` to
  darken the lines, in `multiply`.
* The original is perfectly flat. **T-paper-grain** (amount 8, opacity 25) is optional.

## 8. Hierarchy and eye path

1. The dense blue mass at the right edge (largest, most saturated).
2. The eye follows the fragments leftward as they thin out.
3. It lands on the bold title word at the far left, then the light word under it.

The picture points at the text: density falls toward the title, so the emptiest place
on the canvas is where the words are. Mass 35% of the area; title cap height 3.6% H.

## 9. Build recipe

Needs no images. Change `--seed` on any of the three renders for a different mosaic.

```bash
pixelox render js scripts/clouds.js -o cells.png --size 1080x1080 --seed 12 --param scale=6 --param octaves=3 --param warp=0.5 --param contrast=1.1 --param res=2
pixelox render js scripts/clouds.js -o patch.png --size 1080x1080 --seed 31 --param scale=8 --param octaves=2 --param warp=0.3 --param contrast=1.1 --param res=2
pixelox render js scripts/clouds.js -o break.png --size 1080x1080 --seed 9 --param scale=14 --param octaves=2 --param warp=0.7 --param contrast=1.5 --param res=2
pixelox new -o mosaic.pxo --size 1080x1080 --fill '#ffffff'
pixelox comp place mosaic.pxo cells.png --in-place --as Cells --fit cover
pixelox adjust gradient-map mosaic.pxo --in-place --layer Cells --stops '#0b5abe@0,#0b5abe@0.38,#1d78d3@0.38,#1d78d3@0.5,#0a4fa8@0.5,#0a4fa8@0.6,#2a8ede@0.6,#2a8ede@1'
pixelox comp place mosaic.pxo patch.png --in-place --as Patch --fit cover
pixelox adjust gradient-map mosaic.pxo --in-place --layer Patch --stops '#303030@0,#303030@0.36,#8a8a8a@0.36,#8a8a8a@0.5,#5a5a5a@0.5,#5a5a5a@0.62,#c8c8c8@0.62,#c8c8c8@1'
pixelox layer blend mosaic.pxo --in-place --layer Patch --mode soft-light
pixelox comp place mosaic.pxo break.png --in-place --as Break --fit cover
pixelox layer add mosaic.pxo --in-place --name Ramp
pixelox fill gradient mosaic.pxo --in-place --layer Ramp --from 18%,0 --to 96%,0 --stops '#ffffff@0,#000000@1'
pixelox layer blend mosaic.pxo --in-place --layer Ramp --mode linear-light
pixelox layer merge mosaic.pxo --in-place --layers Break,Ramp --name Cover
pixelox adjust threshold mosaic.pxo --in-place --layer Cover --level 128
pixelox filter gaussian mosaic.pxo --in-place --layer Cover --sigma 0.8
pixelox layer blend mosaic.pxo --in-place --layer Cover --mode screen
pixelox layer stamp mosaic.pxo --in-place --name Lines
pixelox filter find-edges mosaic.pxo --in-place --layer Lines
pixelox adjust bw mosaic.pxo --in-place --layer Lines
pixelox adjust levels mosaic.pxo --in-place --layer Lines --in-black 190 --in-white 250
pixelox layer blend mosaic.pxo --in-place --layer Lines --mode multiply
pixelox text add mosaic.pxo --in-place --name Title --content "Mosaic" --font "Avenir Next Condensed" --weight 700 --size 5.6% --color '#111111' --at 13%,49.2% --anchor l
pixelox text add mosaic.pxo --in-place --name Sub --content "background" --font "Avenir Next" --weight 300 --size 3.4% --color '#333333' --at 13%,53.3% --anchor l
pixelox export mosaic.pxo -o mosaic.png
```

Change first: the four band colors in the Cells gradient map (keep two mid tones, one
darker, one lighter). To move the mass, move the Ramp's `--from` and `--to` (swap them
to put the mass on the left; use `0,18%` and `0,96%` for a mass at the bottom). Cell
size follows `scale` on cells.png (4 = big slabs, 10 = pebbles); fragment size follows
`scale` on break.png. Keep `contrast` on break.png at 1.5 or less: higher values give
the islands straight vertical cuts. Add the type after the Lines layer, or the edge
filter will outline the letters too.

Limit: noise gives nested, contour-like patches; the original's cells are hand-drawn
shards with sharper corners. For that exact look, draw 30 to 50 `shape path` polygons.

## 10. Variations and pairings

* Other hues, same structure: greens `#0b6b3a`, `#1e9e57`, `#0a4f2c`, `#63d08c`;
  corals `#d9482b`, `#f26b3a`, `#a8321c`, `#ff9a6b`; grays for a neutral report cover.
* Dark ground: run `adjust invert` on the Cover and set it to `multiply` in place of
  `screen`; the empty half turns black. Set the title in white.
* Mass in a corner: use a radial Ramp (`--kind radial --from 100%,100% --to 10%,10%`,
  stops reversed: black first) so the mosaic grows out of the bottom-right corner.
* Formats: for a portrait cover put the mass at the bottom 45% and the title at 13% W,
  25% H; for a wide banner keep the mass on the right third.
* Uses: presentation covers, report dividers, social quote cards (replace the two
  title lines with a 3-line quote at the same left edge), event backdrops.
* Pairs with T-rules (a short accent bar above the title in the main blue), T-label-box
  for a date tag, and T-image-in-type or a clipped photo in place of the flat cell
  colors (place a photo above Cells and below Cover).

## 11. Pitfalls

* Cells in many hues: it becomes confetti. One hue, four to five tones.
* A soft or blurred break-up edge: it reads as a smudge, not as fragments. The Cover
  must be thresholded to pure black and white; blur it by less than 1 px only to smooth
  the pixel steps.
* No outlines: neighboring blues of similar lightness melt together and the mosaic
  turns into a flat blue stain. Keep the Lines layer, and keep it thin.
* Text placed over the break-up zone: fragments cut through the letters. The text
  stays in the empty half with a clear gap of its own width to the nearest fragment.
* Centered title: the composition is a diagonal of density from right to left; the
  title must sit at the far left, vertically near the middle, left-aligned.
* A straight boundary between mass and white (a Ramp that is too short): spread
  `--from` and `--to` at least 60% W apart so the zone of islands is wide.
