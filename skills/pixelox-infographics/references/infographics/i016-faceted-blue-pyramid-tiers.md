---
id: i016
title: Faceted blue pyramid in three labelled tiers
kind: [hierarchy]
topic: [technology, business]
mood: [clean, corporate, cool, technical]
style: [flat, 3d]
format: landscape
structure: S-pyramid-notes
data: [hierarchy, text-only]
charts: [C-pyramid]
palette: ['#fcfcfb', '#009ad9', '#0090c9', '#32aee1', '#0a6ebd', '#57585b']
needs: nothing
---

# i016: Faceted blue pyramid in three labelled tiers

## 1. Concept

A wide header figure: one large blue pyramid, seen straight at a corner so two faces show,
cut into three slabs that float a little apart, with white pictograms on the faces. Each
slab has its name in blue capitals on the left and a short paragraph in gray on the right,
and both text blocks lean along the pyramid's slopes. The message is "three layers, each
resting on the one below, and the base is the widest". It works because the pyramid is the
only colored thing on a near-white page, the text obeys its outline, and the number of
pictograms grows with the width of the slab (2, 2, 6), so the base reads as "many small
things" and the top as "few big ones" before a word is read.

## 2. Data story

No numbers. A hierarchy of 3 levels, text only:

* 3 level names, each 3 to 4 words on 2 lines with an ampersand joining two ideas.
* 3 notes, one sentence each: 25 words on 5 lines (top), 16 words on 4 lines (middle), 14
  words on 6 short lines (base). The notes get narrower as the pyramid gets wider.
* 10 pictograms: 2 on the top slab, 2 on the middle slab, 6 on the base (3 per face).
* No title, no source: the piece sits under an article headline.

Placeholder content in the recipe (invented, about how a small chain of shops uses its
numbers): WEEKLY REPORTS & TEAM DECISIONS; ANALYSIS TOOLS & MODELS; RAW RECORDS & SOURCES,
with one sentence each.

## 3. Information architecture

1. **The pyramid**, on a vertical axis slightly left of center: read top to bottom, apex
   first. The bottom edge of the page crops the base, which says "this level goes on".
2. **Level names**, left: one per slab, level with the slab's upper half.
3. **Notes**, right: one per slab, at the same height as its name, so each slab reads as a
   row: name, pictograms, explanation.
4. **Pictograms** on the faces: what lives on that level.

There are no leaders, numbers, or arrows: closeness to the slope ties each text to its slab.

## 4. Format and grid

* Landscape, 1.905 : 1 (the social preview ratio). Recipe canvas 1600 x 840.
* Pyramid axis at 48.4% W. Apex at 48.4% W, 3.8% H. The outline falls 0.644 px sideways for
  every px down (about 57 degrees from the horizontal); at the bottom edge it spans 15.9% to
  80.9% W. The pyramid covers about 30% of the canvas.
* Slab heights at the outline: top slab 3.8% to 39.5% H; middle 45.6% to 64.7% H; base from
  72.6% H to past the bottom edge. The openings between slabs are 6% and 8% H.
* Left column (names): right-aligned on a line parallel to the left slope, 3.4% W outside it:
  right edges at 39.5% W (19.9% H), 29.1% W (51% H), 18.5% W (82.5% H). Two lines each, 29 px
  apart; the second line's right edge is 19 px further left.
* Right column (notes): left-aligned on a line parallel to the right slope, 3.1% W outside
  it: first lines start at 54.6% W (12.5% H), 66.5% W (48.2% H), 76.3% W (76.5% H). Line
  pitch 30 px; every line starts 19 px further right than the line above.
* Outer margins: the widest name starts at 7% W; the notes end by 93% W.

## 5. Layer stack

1. Ground: near-white `#fcfcfb`.
2. Wash: a blurred interior photograph faded to almost nothing, pale blue-gray patches
   (`#e9eef1`) at the top center and right. It adds depth without a single readable shape.
3. Pyramid, back to front (see the 3D spec in section 8): per slab a left face `#009ad9`
   and a right face `#0090c9` meeting on the axis; on slabs 2 and 3 a lighter top face
   `#32aee1` seen as a flat rhombus; a deep blue rhombus `#0a6ebd` where the slab above
   overlaps that top face.
4. Ten white pictograms, solid, about 6% W each, slightly sheared so they lie on the faces.
5. Three level names in blue caps `#0a9bdb`.
6. Three notes in gray `#57585b`.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#fcfcfb` | ground (with a `#e9eef1` wash) | 68% | nothing |
| `#009ad9` | left faces; level names (`#0a9bdb`) | 14% | nothing: the lit side |
| `#0090c9` | right faces | 12% | nothing: the shaded side |
| `#32aee1` | top faces of slabs 2 and 3 | 3% | nothing: the surface a slab rests on |
| `#0a6ebd` | overlap under slabs 1 and 2 | 1% | nothing: contact shadow |
| `#57585b` | notes | 2% | nothing |
| `#ffffff` | pictograms | 1% | nothing |

Monochrome: four steps of one blue (light 45% to 64%) and a neutral gray. No color encodes
data; the steps only model light. The level names take the pyramid's blue so they read as
part of it.

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Level name (x3) | 3 to 4 words on 2 lines, joined by "&" | caps | humanist sans (Source Sans 3, Open Sans / Helvetica Neue through `@neo-grotesque`) | 700 | 25 | 20 | 29 px pitch | right, stepped along the slope | `#0a9bdb` | right edges 39.5%, 19.9%; 29.1%, 51%; 18.5%, 82.5% |
| Note, top | 25 words on 5 lines of 24 to 38 characters | sentence | humanist sans, same family | 400 | 24 | 0 | 30 px pitch | left, stepped along the slope | `#57585b` | first line 54.6%, 12.5% |
| Note, middle | 16 words on 4 lines of 22 to 24 characters | sentence | same | 400 | 24 | 0 | 30 px pitch | left, stepped | `#57585b` | first line 66.5%, 48.2% |
| Note, base | 14 words on 6 lines of 13 to 20 characters | sentence | same | 400 | 24 | 0 | 30 px pitch | left, stepped | `#57585b` | first line 76.3%, 76.5% |

One family, two weights, one size (24 to 25 px on a 1600 px canvas): hierarchy comes from
caps, weight, and color, not from size. The picture is the headline.

## 8. Charts and diagrams

* **C-pyramid**: `scripts/pyramid.js`, box `14.6,3.8,67.6,100` (the box is as tall as the
  canvas and starts at 3.8% H, so the base tier is cropped by the bottom edge), 3 tiers,
  `labels=' | | '` (three empty names) with `labelpos=in` so the script writes nothing,
  `gap=40`, one color. Anchors `tier1` to `tier3` give the axis (x = 774 px) and each tier's
  middle. No axis, no legend, no values: tiers are equal steps.
* The recipe renders the same box a second time as one solid triangle in the top-face blue
  (`labels=' | '`, `gap=0`) and places it under the tiers: the openings between tiers then
  show `#32aee1`, standing in for the top faces. A hard-stop gradient overlay
  (`fx overlay`, stops at 48.4%) splits the tiers into a lit left half and a shaded right
  half.
* **Simplified:** the original's slabs have unequal heights and real top faces; the script
  draws equal tiers with flat gaps. Pictograms are upright, not sheared onto the faces.

**3D spec (for a later three.js build).** Object: a square pyramid cut by two horizontal
planes into three slabs, pulled apart vertically by about 15% of a slab's height. View:
orthographic, rotated 45 degrees about the vertical axis (one vertical edge faces the
viewer, both visible faces equally wide), camera elevation about 13 degrees above the
horizon (a top face appears as a rhombus 0.22 times as high as it is wide). Light from the
upper left. Face colors: left `#009ad9`, right `#0090c9`, top `#32aee1`; contact shadow on
a top face where the slab above covers it `#0a6ebd`. No gloss, no cast shadow on the
ground, no outlines. Depth, slab height, and face area encode nothing. Outline points in %
of W and H, for a build by hand:

| Part | Points |
| --- | --- |
| Slab 1 | apex 48.4, 3.8; side corners 36.3 and 60.5 at 39.5; front corner 48.4, 45.9 |
| Slab 2 top face | side corners 34.0 and 62.9 at 45.6; back 48.4, 40.5; front 48.4, 52.4 |
| Slab 2 body | from the top face's front edges down to side corners 27.4 and 69.5 at 64.7; front corner 48.4, 76.6 |
| Slab 3 top face | side corners 25.3 and 71.7 at 72.6; back 48.4, 64.3; front 48.4, 84.5 |
| Slab 3 body | from the top face's front edges to the bottom edge, 15.9 to 80.9 |
| Shadow 1 | 48.4, 40.5; 42.2 and 54.7 at 42.9; 48.4, 45.9 |
| Shadow 2 | 48.4, 64.3; 35.0 and 61.9 at 68.7; 48.4, 76.6 |

## 9. Icons and illustration

* Original: ten solid white pictograms of devices and services (a database with a plus, a
  cloud; mixer sliders, a monitor with gears; a satellite, two signal fans, a dial gauge, a
  thermometer, a fan), each about 5 to 8% W, sheared to follow its face. The two on the top
  slab sit side by side just above its lower edge; the base has three per face in a shallow
  V, the middle ones cropped by the bottom edge.
* Recipe: Lucide line icons in white, upright. Top tier: `lightbulb`, `target`, 78 px, 6 px
  line, at 45.1% and 51.9% W, 27.6% H. Middle tier: `calculator`, `chart-column`, 104 px, 7
  px line, at 39.5% and 57.5% W, 56.4% H. Base tier: `receipt`, `package`, `users` on the
  left face and `clipboard-list`, `store`, `clock` on the right, 96 px, 7 px line, in a V
  from 89.4% H at the outside to 96% H next to the axis.
* Icon size grows a little with the tier (78, 104, 96) because the faces get wider; line
  weight stays at about 1/14 of the size so all ten look equally heavy.
* No photo is needed. For the wash use `scripts/blobs.js` (below) or your own `photo.png`
  placed with `--fit cover` at 6 to 8% opacity.

## 10. Effects and finish

* Two-tone faces: `fx overlay --kind gradient` on the tiers layer with four stops, two on
  each side of the axis (`'#009ad9@0,#009ad9@0.484,#0090c9@0.484,#0090c9@1'`, `--angle 0`):
  a hard vertical edge at 48.4% W.
* Top faces: the under-layer triangle in `#32aee1` showing through 40 px gaps.
* Wash: `scripts/blobs.js`, 7 soft blobs in `#dfe9ee`, `#e9eef1`, `#d8e4ea`, `alpha=0.55`,
  `edge=1` (pushed to the border so the middle stays clean).
* No shadows on the ground, no outlines, no gradients inside a face.

## 11. Hierarchy and eye path

1. The pyramid: the only saturated shape, 30% of the page, apex at the top center.
2. Down the slabs through the white pictograms.
3. The blue level names on the left (same hue as the pyramid, bold caps).
4. The gray notes on the right, one per slab.

All type is one size; the order is set by color and weight: blue bold caps before gray
regular sentences. The pictograms are 3 to 4 times the type height.

## 12. Build recipe

Nothing to supply. Canvas 1600 x 840. Placeholder copy, no data.

```bash
pixelox new -o pyr.pxo --size 1600x840 --fill '#fcfcfb'
pixelox render js scripts/blobs.js -o wash.png --size 1600x840 --seed 7 --param colors='#dfe9ee,#e9eef1,#d8e4ea' --param bg=transparent --param count=7 --param size=0.5 --param alpha=0.55 --param edge=1
pixelox comp place pyr.pxo wash.png --in-place --as Wash --at 0,0
pixelox render js scripts/pyramid.js -o tops.png --size 1600x840 --param box=14.6,3.8,67.6,100 --param labels=' | ' --param labelpos=in --param gap=0 --param colors='#32aee1'
pixelox comp place pyr.pxo tops.png --in-place --as TopFaces --at 0,0
pixelox render js scripts/pyramid.js -o tiers.png --size 1600x840 --param box=14.6,3.8,67.6,100 --param labels=' | | ' --param labelpos=in --param gap=40 --param colors='#009ad9'
pixelox comp place pyr.pxo tiers.png --in-place --as Pyramid --at 0,0
pixelox fx overlay pyr.pxo --in-place --layer Pyramid --kind gradient --stops '#009ad9@0,#009ad9@0.484,#0090c9@0.484,#0090c9@1' --angle 0
pixelox shape icon pyr.pxo lightbulb --in-place --name I1a --size 78 --at 722,232 --anchor c --color '#ffffff' --stroke-width 6
pixelox shape icon pyr.pxo target --in-place --name I1b --size 78 --at 830,236 --anchor c --color '#ffffff' --stroke-width 6
pixelox shape icon pyr.pxo calculator --in-place --name I2a --size 104 --at 632,474 --anchor c --color '#ffffff' --stroke-width 7
pixelox shape icon pyr.pxo chart-column --in-place --name I2b --size 104 --at 920,474 --anchor c --color '#ffffff' --stroke-width 7
pixelox shape icon pyr.pxo receipt --in-place --name I3a --size 96 --at 406,751 --anchor c --color '#ffffff' --stroke-width 7
pixelox shape icon pyr.pxo package --in-place --name I3b --size 96 --at 536,786 --anchor c --color '#ffffff' --stroke-width 7
pixelox shape icon pyr.pxo users --in-place --name I3c --size 96 --at 664,806 --anchor c --color '#ffffff' --stroke-width 7
pixelox shape icon pyr.pxo clipboard-list --in-place --name I3d --size 96 --at 884,806 --anchor c --color '#ffffff' --stroke-width 7
pixelox shape icon pyr.pxo store --in-place --name I3e --size 96 --at 1020,786 --anchor c --color '#ffffff' --stroke-width 7
pixelox shape icon pyr.pxo clock --in-place --name I3f --size 96 --at 1150,751 --anchor c --color '#ffffff' --stroke-width 7
pixelox text add pyr.pxo --in-place --name L1a --content "WEEKLY REPORTS &" --font @neo-grotesque --weight 700 --size 25 --tracking 20 --color '#0a9bdb' --at 632,167 --anchor r
pixelox text add pyr.pxo --in-place --name L1b --content "TEAM DECISIONS" --font @neo-grotesque --weight 700 --size 25 --tracking 20 --color '#0a9bdb' --at 613,196 --anchor r
pixelox text add pyr.pxo --in-place --name L2a --content "ANALYSIS TOOLS" --font @neo-grotesque --weight 700 --size 25 --tracking 20 --color '#0a9bdb' --at 465,428 --anchor r
pixelox text add pyr.pxo --in-place --name L2b --content "& MODELS" --font @neo-grotesque --weight 700 --size 25 --tracking 20 --color '#0a9bdb' --at 446,457 --anchor r
pixelox text add pyr.pxo --in-place --name L3a --content "RAW RECORDS" --font @neo-grotesque --weight 700 --size 25 --tracking 20 --color '#0a9bdb' --at 296,693 --anchor r
pixelox text add pyr.pxo --in-place --name L3b --content "& SOURCES" --font @neo-grotesque --weight 700 --size 25 --tracking 20 --color '#0a9bdb' --at 277,722 --anchor r
pixelox text add pyr.pxo --in-place --name N1a --content "Reports turn the numbers into choices:" --font @neo-grotesque --size 24 --color '#57585b' --at 873,105 --anchor l
pixelox text add pyr.pxo --in-place --name N1b --content "what to stock, when to hire, and" --font @neo-grotesque --size 24 --color '#57585b' --at 892,135 --anchor l
pixelox text add pyr.pxo --in-place --name N1c --content "where to open the next shop. Every" --font @neo-grotesque --size 24 --color '#57585b' --at 911,165 --anchor l
pixelox text add pyr.pxo --in-place --name N1d --content "decision at the top rests on" --font @neo-grotesque --size 24 --color '#57585b' --at 931,195 --anchor l
pixelox text add pyr.pxo --in-place --name N1e --content "the two levels below it." --font @neo-grotesque --size 24 --color '#57585b' --at 950,225 --anchor l
pixelox text add pyr.pxo --in-place --name N2a --content "Spreadsheets and simple" --font @neo-grotesque --size 24 --color '#57585b' --at 1064,405 --anchor l
pixelox text add pyr.pxo --in-place --name N2b --content "models sort the records," --font @neo-grotesque --size 24 --color '#57585b' --at 1083,435 --anchor l
pixelox text add pyr.pxo --in-place --name N2c --content "compare the weeks, and" --font @neo-grotesque --size 24 --color '#57585b' --at 1102,465 --anchor l
pixelox text add pyr.pxo --in-place --name N2d --content "flag what looks unusual." --font @neo-grotesque --size 24 --color '#57585b' --at 1122,495 --anchor l
pixelox text add pyr.pxo --in-place --name N3a --content "Till receipts, stock" --font @neo-grotesque --size 24 --color '#57585b' --at 1220,643 --anchor l
pixelox text add pyr.pxo --in-place --name N3b --content "counts, and visitor" --font @neo-grotesque --size 24 --color '#57585b' --at 1239,673 --anchor l
pixelox text add pyr.pxo --in-place --name N3c --content "numbers are noted" --font @neo-grotesque --size 24 --color '#57585b' --at 1258,703 --anchor l
pixelox text add pyr.pxo --in-place --name N3d --content "in every shop," --font @neo-grotesque --size 24 --color '#57585b' --at 1278,733 --anchor l
pixelox text add pyr.pxo --in-place --name N3e --content "every day, by" --font @neo-grotesque --size 24 --color '#57585b' --at 1297,763 --anchor l
pixelox text add pyr.pxo --in-place --name N3f --content "the people there." --font @neo-grotesque --size 24 --color '#57585b' --at 1316,793 --anchor l
pixelox export pyr.pxo -o pyr.png
```

Reuse:

* Every line of text is its own layer so the blocks can lean on the slopes. For a text line
  whose middle is at y px, the left outline is at x = 774 - 0.644 x (y - 32) and the right
  outline at x = 774 + 0.644 x (y - 32). Names end 55 px left of the left outline; notes
  start 50 px right of the right outline. With a 30 px line pitch each line shifts 19 px.
* Change the tier count with the number of `|` in `labels=` (four empty names: `' | | | '`)
  and keep `gap=40`; run `lab/anchors.py` on the tiers line to get each tier's middle, put
  the icons 20 to 60 px below it, the name and the note's first line 20 to 60 px above it.
* If you move or resize the box, move the overlay's two middle stops to the new axis
  (axis x / canvas width).
* Break notes by hand: the top note may run 38 characters a line, the base note only 20.

## 13. Variations and pairings

* **More levels:** 4 or 5 tiers work with one pictogram on each face of the upper tiers and
  two or three on the lower ones; drop the notes to 2 lines.
* **Not cropped:** set the box height to 88 so the whole pyramid shows and put a C-source-line
  under it; use this for a square or 4:5 post (box about `18,14,64,74`).
* **Flat editorial:** one face color, no overlay, names written inside the tiers
  (`labelpos=in` with real labels) and notes on leaders (`labelpos=right`): the plain
  C-pyramid.
* **Other hues:** the four steps of any hue: green `#1e9e62`, `#188f58`, `#52bd8a`,
  `#0d6f43`; or slate with one tier in an accent color to say "we are here".
* **With amounts:** when the levels have counts, switch to C-funnel and print the numbers.
* Relatives: i045 (a ranked list on colored ribbons) for the same levels as a list; pairs
  with C-title-block above it in a taller format.

## 14. Pitfalls

* **Reading sizes into the tiers.** The base slab looks many times larger than the top one,
  but the levels have no amounts. If your levels do have values, do not let the areas speak
  for them: print the numbers, or use C-funnel, whose widths follow the values. If they do
  not, never add percentages to a pyramid like this one.
* **Text that ignores the slope.** A plain left-aligned paragraph beside a diagonal leaves a
  wedge of dead space and loosens the link to its tier. Step each line by pitch x 0.644, or
  center short notes on a leader instead.
* **Too many pictograms on the top tier.** The top face is narrow: two icons at most, or
  they touch the edges. Put the crowd at the base.
* **Icons heavier than the faces.** White icons with thin lines vanish on mid blue: keep the
  line at 6 to 7 px for 80 to 100 px icons, and never use a second icon color.
* **A light pyramid.** With face colors lighter than about 60% lightness the white icons and
  the split between the two faces disappear. Keep the faces between 40% and 50% lightness
  and within 4 points of each other.
* **A title added in the pyramid's blue above the apex** competes with the apex. Put a title
  to the left in the gray of the notes, or leave it to the page around the figure.
