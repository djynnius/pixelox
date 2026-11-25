---
id: i040
title: Rocket over four fanned exhaust columns
kind: [process, list]
topic: [business, generic]
mood: [clean, friendly, vibrant]
style: [flat, gradient]
format: square
structure: S-object-callouts
data: [process, text-only]
charts: [C-rocket, C-world-map, C-callout, C-icon-badge]
palette: ['#ffffff', '#46c4dc', '#7b9aac', '#f56328', '#ffc653', '#f42a3a', '#f2f2f2']
needs: nothing
---

# i040: Rocket over four fanned exhaust columns

## 1. Concept

A square four-step graphic on white. A small flat rocket climbs on the center axis; its exhaust
is four thin colored stripes that drop straight down, then fan out like a stage floor seen in
perspective and land on a row of four colored cells, one icon in each. Four numbered callouts
(01 to 04) sit beside the fan, two on each side, tied to their color by a thin leader. The
message is "these four things, together, lift it off". It works because the eye can follow
each color without a break from the cell at the bottom, up its fan, into its stripe, and into
the rocket: four inputs visibly become one launch. A pale world map behind adds "everywhere"
without adding a single thing to read.

## 2. Data story

No numbers. Text only, an ordered set of four:

* 4 steps, each a number (01 to 04), a title of 2 to 4 words (two short lines in the
  original, one line in the recipe), and a note of about 20 words on 4 lines (the recipe: 7 to
  9 words on 2 lines, because it sets the notes more than twice as large).
* 4 pictograms, one per step, in the bottom cells.
* No title, no source line: the piece is a module, meant to sit under a headline.

Placeholder content in the recipe (invented, about bringing a product to market): 01 Write the
brief, 02 Build the core, 03 Sign partners, 04 Go to market, each with one sentence.

## 3. Information architecture

1. **The rocket**, top center: the goal. Nothing is written on it.
2. **The exhaust**: four stripes in the step colors, left to right cyan, slate, orange,
   yellow. They say "four parts feed this".
3. **The fan**: each stripe widens into a trapezoid, darker toward the bottom, and meets its
   cell. This is the link between the list and the object.
4. **Four cells** in a row at the bottom: flat color, one white line icon each. Read left to
   right, they give the order 01 to 04.
5. **Four callouts** in an arch: 01 low on the left, 02 high on the left, 03 high on the
   right, 04 low on the right. The outer steps (01, 04) are low because the fan is wide
   there; the inner steps (02, 03) are high, where the fan is narrow. A leader with a dot
   runs from each title to its own trapezoid.
6. **World map and dot grid** behind everything: context only.

The numbers carry the order. Without them the arch would be read 02, 03, 01, 04.

## 4. Format and grid

* Square. Recipe canvas 1080 x 1080. One vertical axis at 50% W; the left half mirrors the
  right.
* Rocket: nose tip at 6.8% H, nozzle bottom at 41.7% H. Body 12.5% W wide (12.6% W from the
  script with `aspect=1.3`); side boosters reach 36.7% and 63.3% W (the script's fins: 39.5%
  and 60.5% W).
* Exhaust stem: `46.7%,41.7%,6.7% x 14.4%`, four stripes of 1.67% W each.
* Fan: from the stem's foot (56.1% H) to the top of the cells (75.9% H); it spreads from 6.7%
  W to 64% W.
* Cells: `17.9%,75.9%,64.1% x 16.3%`, four equal cells of 16% W, no gaps.
* Floor shadow: an ellipse 52% W by 2.2% H centered on 50% W, 93.5% H.
* Callouts: text margins 6% W (the original: 9.4% W). Number cap tops at 46.1% H (02 and 03)
  and 59.3% H (01 and 04). Left edges 6% W (01) and 15.9% W (02); right edges 84.1% W (03)
  and 94% W (04). Left blocks are flush left, right blocks flush right.
* World map: `5%,10%,90% x 42%`. Dot grid over the whole ground, pitch 3.3% W.

## 5. Layer stack

1. Ground: white, with a very soft vignette to `#f1f1f1` in the corners.
2. Dot grid: pale gray dots `#e9e9e9`, 4 px, every 36 px.
3. World map: a flat silhouette in `#f2f2f2`, about 5% darker than the ground, no borders, no
   labels. The rocket overlaps it.
4. Fan: four trapezoids. Each starts at the stripe's color at the top and darkens by about a
   third toward the bottom edge: cyan `#46c4dc` to `#2a9fd0`, slate `#7b9aac` to `#5a7684`,
   orange `#e2591f` to `#9c3f1c`, yellow-ochre `#e9b24a` to `#8e6f2f`.
5. Stem: four flat stripes `#47c2db`, `#7b9cac`, `#f16f2b`, `#fbcd69`.
6. Floor shadow: a blurred dark ellipse under the cells, 30% opacity.
7. Cells: four flat rectangles `#46c4dc`, `#7b9aac`, `#f56328`, `#ffc653`.
8. Rocket: red nose `#f42a3a`, cream body `#f6c574` (left half a little lighter, right half a
   little darker, split on the axis), an orange belt `#e9502b` two thirds of the way down, a
   dark red nozzle `#921d27`. In the original two orange pods on short wings stand beside the
   belt; the script draws swept fins in the same orange `#e85428`.
9. Porthole on the upper body: a blue-gray disc `#5292ad`, radius 2.8% W, with a red core
   `#f0222a`, radius 1.5% W.
10. Four white line icons, 60 px, centered in the cells.
11. Four leaders, 2 px, each in its step color, one elbow, a 12 px dot at the text end.
12. Numbers and titles in the step colors; notes in gray `#8a8f94`.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#ffffff` | ground (corners `#f1f1f1`) | 62% | nothing |
| `#f2f2f2` | world map, dot grid (`#e9e9e9`) | 12% | nothing: context |
| `#46c4dc` | cell, fan, stripe, number, title, and leader of step 01 | 6% | step 1 |
| `#7b9aac` | the same set for step 02 (text `#6e92a5`) | 6% | step 2 |
| `#f56328` | the same set for step 03 (text `#f26a2a`) | 6% | step 3 |
| `#ffc653` | the same set for step 04 (text `#f5b335`) | 6% | step 4 |
| `#f42a3a` | rocket nose and porthole core; with cream `#f6c574` and orange `#e9502b` for the rocket | 2% | nothing: the rocket stays outside the four step colors |

Categorical: two cool steps on the left (cyan, slate), two warm steps on the right (orange,
yellow), so the left and right halves also read as a pair of pairs. Each step color appears
six times (cell, fan, stripe, number, title, leader) and nowhere else. The rocket's red is the
single accent and belongs to no step.

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Step number (x4) | 01 to 04 | - | geometric sans (Montserrat, Poppins / Avenir Next) | 600 | 46 (the original: 34) | 0 | - | left for 01 and 02, right for 03 and 04 | the step color | 01: 6%, cap top 59.3%; 02: 15.9%, 46.1%; 03: right edge 84.1%, 46.1%; 04: right edge 94%, 59.3% |
| Step title (x4) | 2 to 4 words, up to 15 characters on one line (the original: 2 lines) | sentence | geometric sans | 700 | 22 (the original: 11) | 0 | - | as its number | the step color | 5% H under the number's cap top |
| Step note (x4) | 7 to 9 words on 2 lines of up to 25 and 21 characters (the original: about 20 words on 4 lines) | sentence | geometric sans | 400 | 18 (the original: 8) | 0 | 135% | as its number | `#8a8f94` | 3% H under the title's cap top |

One family, three levels: number 46, title 22, note 18 in the recipe. The original is a
template preview with far smaller words (34 : 11 : 8, about 4 : 1.4 : 1); the recipe keeps the
order of the levels and lifts the two small ones to sizes that can be read.

## 8. Charts and diagrams

* **C-rocket**: `scripts/object.js`, `kind=rocket`, `aspect=1.3` (the stout body: its width
  is a third of its height, as in the original), `bands=4`, `badges=none`, `flame=0`, box
  `36,6.8,28,42.2`, `colors=` red, cream, orange, cream, `fincolor='#e85428'`,
  `dark='#921d27'`. The bands here are paint, not steps: they give the rocket its red nose and
  orange belt. The script's `band2` anchor (540,246 on the recipe canvas) is the porthole's
  center; the porthole is two native discs, because the script's own (`window=1`) has fixed
  colors and no red core. At this aspect the nozzle is exactly as wide as the stem (72 px).
  Simplified: the original's nose cone is short (a fifth of the body), and it has two booster
  pods on wings; the script draws a longer nose and swept fins.
* **The fan (perspective spec)**: not a chart; four `shape path` trapezoids. It reads as the
  top face of a low box seen from the front in one-point perspective: front face = the row of
  cells, flat, facing the viewer (rotation 0 degrees); top face = the fan, tilted away at
  about 35 degrees of elevation, all four edges meeting at a vanishing point on the stem's
  foot (50% W, 56.1% H). Light comes from the far end (from the rocket): the far end of each
  trapezoid has the stripe's color, the near edge is about 30% darker, and the front face is
  the flat, lightest color. No side faces are visible. Depth encodes nothing: the four cells
  are equal. Build: trapezoid N runs from stripe N at the stem's foot to cell N's top edge;
  `fx overlay` with a two-stop gradient at `--angle 90` (dark at the bottom) shades it.
* **C-world-map** (backdrop): `scripts/geo.js`, run with `render d3`, box `5,10,90,42`,
  `projection=natural`, `stroke=none`, `land='#ededed'`, `extent=-180,-56,180,84`. **Map
  spec**: region set: all land as one silhouette, no country borders, Antarctica left out,
  cut at about 56 degrees south; projection: a flat world with straight parallels and gently
  curved sides (Natural Earth; the original looks like Robinson or Miller); encoding: none,
  every region one tint (`#f2f2f2` in the original, `#ededed` in the recipe) on white;
  borders: none; labels: none; legend: none; no insets; box `5%,10%,90% x 42%`, behind the
  rocket and the two upper callouts. It must stay within 6% lightness of the ground.
  `extent=` is what leaves Antarctica out: with `stroke=none` the script draws all land, and
  an extent clips the map to the box, here just south of Cape Horn.
  Fallback for builds before 0.6.0: C-dot-map (`scripts/worldmap.js`, `render js`, the same
  box, `shape=square`, `fill=1`, `dot='#ededed'`), a blocky pale world.
* **C-callout** x4: a 12 px dot beside the title, 42 to 48 px of horizontal line, then a
  45-degree stroke down to the step's trapezoid. The two upper leaders cross the neighboring
  outer stripe to reach the inner trapezoids.
* **C-icon-badge**, square variant without a frame: the cell itself is the badge.

## 9. Icons and illustration

* Cells, left to right: `list-checks`, `brain`, `handshake`, `rocket`; 60 px, white, 3 px
  line, centered on the cells at 25.9%, 42%, 58%, 74% W and 84% H. The original's icons are a
  ruled checklist page, a brain drawn as a circuit, two shaking hands, and a small rocket, all
  in the same thin white line.
* The rocket is drawn by `scripts/object.js` (see section 8); the porthole is two
  `shape ellipse` layers on its `band2` anchor.
* No photos, no logo.

## 10. Effects and finish

* Fan shading: `fx overlay --kind gradient --angle 90` on each trapezoid, from the dark tone
  at the bottom to the stripe color at 75 to 90% of the height. This is the only gradient
  that carries the 3D illusion: keep the cells and the stripes flat.
* Floor shadow: `shape ellipse` in black, `filter gaussian --sigma 12`, `layer opacity` 30.
  It lifts the row of cells off the page.
* Vignette: a radial `fill gradient` from white (to 55% of the radius) to `#f1f1f1`.
* Dot grid: `scripts/pattern.js`, `kind=dots`, `cell=36`, `weight=0.1`, `color='#e9e9e9'`.
* Rocket: the script's light strip on the left of the body (`shade=1`). No outlines anywhere.

## 11. Hierarchy and eye path

1. The rocket: the only red, at the top of the axis.
2. Down the four stripes into the fan: the largest colored area, a strong arrowhead shape
   pointing at the rocket.
3. The four cells and their icons, left to right.
4. The numbers 01 to 04 (46 px, in color), then titles, then notes.
5. The map is noticed last, if at all.

Number 46, title 22, note 18: about 2.5 : 1.2 : 1.

## 12. Build recipe

Nothing to supply. Canvas 1080 x 1080. The copy is example copy on an invented subject. The
world map is drawn by `render d3` (pixelox 0.6.0 or later).

```bash
pixelox new -o launch.pxo --size 1080x1080 --fill '#ffffff'
pixelox layer add launch.pxo --in-place --name Vignette
pixelox fill gradient launch.pxo --in-place --layer Vignette --kind radial --from 50%,45% --to 100%,100% --stops '#ffffff@0,#ffffff@0.55,#f1f1f1@1'
pixelox render js scripts/pattern.js -o dots.png --size 1080x1080 --param kind=dots --param color='#e9e9e9' --param cell=36 --param weight=0.1
pixelox comp place launch.pxo dots.png --in-place --as Dots --at 0,0
pixelox render d3 scripts/geo.js -o world.png --size 1080x1080 --param box=5,10,90,42 --param projection=natural --param extent=-180,-56,180,84 --param stroke=none --param land='#ededed'
pixelox comp place launch.pxo world.png --in-place --as World --at 0,0
pixelox shape path launch.pxo --in-place --name Fan1 --d 'M 504 606 L 522 606 L 367 820 L 194 820 Z' --fill '#46c4dc'
pixelox fx overlay launch.pxo --in-place --layer Fan1 --kind gradient --stops '#2a9fd0@0,#46c4dc@0.75' --angle 90
pixelox shape path launch.pxo --in-place --name Fan2 --d 'M 522 606 L 540 606 L 540 820 L 367 820 Z' --fill '#7b9aac'
pixelox fx overlay launch.pxo --in-place --layer Fan2 --kind gradient --stops '#5a7684@0,#7b9aac@0.9' --angle 90
pixelox shape path launch.pxo --in-place --name Fan3 --d 'M 540 606 L 558 606 L 713 820 L 540 820 Z' --fill '#f56328'
pixelox fx overlay launch.pxo --in-place --layer Fan3 --kind gradient --stops '#9c3f1c@0,#e2591f@0.9' --angle 90
pixelox shape path launch.pxo --in-place --name Fan4 --d 'M 558 606 L 576 606 L 886 820 L 713 820 Z' --fill '#ffc653'
pixelox fx overlay launch.pxo --in-place --layer Fan4 --kind gradient --stops '#8e6f2f@0,#e9b24a@0.9' --angle 90
pixelox shape rect launch.pxo --in-place --name Stem1 --rect 504,440,18x166 --fill '#47c2db'
pixelox shape rect launch.pxo --in-place --name Stem2 --rect 522,440,18x166 --fill '#7b9cac'
pixelox shape rect launch.pxo --in-place --name Stem3 --rect 540,440,18x166 --fill '#f16f2b'
pixelox shape rect launch.pxo --in-place --name Stem4 --rect 558,440,18x166 --fill '#fbcd69'
pixelox shape ellipse launch.pxo --in-place --name Shadow --rect 260,998,560x24 --fill '#000000'
pixelox filter gaussian launch.pxo --in-place --layer Shadow --sigma 12
pixelox layer opacity launch.pxo --in-place --layer Shadow --value 30
pixelox shape rect launch.pxo --in-place --name Cell1 --rect 194,820,173x176 --fill '#46c4dc'
pixelox shape rect launch.pxo --in-place --name Cell2 --rect 367,820,173x176 --fill '#7b9aac'
pixelox shape rect launch.pxo --in-place --name Cell3 --rect 540,820,173x176 --fill '#f56328'
pixelox shape rect launch.pxo --in-place --name Cell4 --rect 713,820,173x176 --fill '#ffc653'
pixelox render js scripts/object.js -o ship.png --size 1080x1080 --param box=36,6.8,28,42.2 --param kind=rocket --param aspect=1.3 --param bands=4 --param badges=none --param flame=0 --param colors='#f42a3a,#f6c574,#e9502b,#f6c574' --param fincolor='#e85428' --param dark='#921d27'
pixelox comp place launch.pxo ship.png --in-place --as Rocket --at 0,0
pixelox shape ellipse launch.pxo --in-place --name PortRing --rect 510,216,60x60 --fill '#5292ad'
pixelox shape ellipse launch.pxo --in-place --name PortCore --rect 524,230,32x32 --fill '#f0222a'
pixelox shape icon launch.pxo list-checks --in-place --name Ic1 --size 60 --at 280,908 --anchor c --color '#ffffff' --stroke-width 3
pixelox shape icon launch.pxo brain --in-place --name Ic2 --size 60 --at 453,908 --anchor c --color '#ffffff' --stroke-width 3
pixelox shape icon launch.pxo handshake --in-place --name Ic3 --size 60 --at 627,908 --anchor c --color '#ffffff' --stroke-width 3
pixelox shape icon launch.pxo rocket --in-place --name Ic4 --size 60 --at 800,908 --anchor c --color '#ffffff' --stroke-width 3
pixelox shape path launch.pxo --in-place --name Lead1 --d 'M 270 702 L 318 702 L 340 724' --fill none --stroke '#47c2db' --stroke-width 2
pixelox shape ellipse launch.pxo --in-place --name Dot1 --rect 264,696,12x12 --fill '#47c2db'
pixelox text add launch.pxo --in-place --name N1 --content "01" --font @geometric-sans --weight 600 --size 46 --color '#3bbbd6' --at 65,640 --anchor tl --anchor-to ink
pixelox text add launch.pxo --in-place --name T1 --content "Write the brief" --font @geometric-sans --weight 700 --size 22 --color '#3bbbd6' --at 65,694 --anchor tl --anchor-to ink
pixelox text add launch.pxo --in-place --name B1 --content $'One page: who it is for\nand what it replaces.' --font @geometric-sans --size 18 --leading 135% --color '#8a8f94' --at 65,724 --anchor tl
pixelox shape path launch.pxo --in-place --name Lead2 --d 'M 388 560 L 430 560 L 504 634' --fill none --stroke '#6e92a5' --stroke-width 2
pixelox shape ellipse launch.pxo --in-place --name Dot2 --rect 382,554,12x12 --fill '#6e92a5'
pixelox text add launch.pxo --in-place --name N2 --content "02" --font @geometric-sans --weight 600 --size 46 --color '#6e92a5' --at 172,498 --anchor tl --anchor-to ink
pixelox text add launch.pxo --in-place --name T2 --content "Build the core" --font @geometric-sans --weight 700 --size 22 --color '#6e92a5' --at 172,552 --anchor tl --anchor-to ink
pixelox text add launch.pxo --in-place --name B2 --content $'Ship the smallest version\nthat solves the problem.' --font @geometric-sans --size 18 --leading 135% --color '#8a8f94' --at 172,582 --anchor tl
pixelox shape path launch.pxo --in-place --name Lead3 --d 'M 692 560 L 650 560 L 576 634' --fill none --stroke '#f16f2b' --stroke-width 2
pixelox shape ellipse launch.pxo --in-place --name Dot3 --rect 686,554,12x12 --fill '#f16f2b'
pixelox text add launch.pxo --in-place --name N3 --content "03" --font @geometric-sans --weight 600 --size 46 --color '#f26a2a' --at 908,498 --anchor tr --anchor-to ink
pixelox text add launch.pxo --in-place --name T3 --content "Sign partners" --font @geometric-sans --weight 700 --size 22 --color '#f26a2a' --at 908,552 --anchor tr --anchor-to ink
pixelox text add launch.pxo --in-place --name B3 --content $'Two resellers and one\nsupplier, in writing.' --font @geometric-sans --size 18 --leading 135% --align right --color '#8a8f94' --at 908,582 --anchor tr
pixelox shape path launch.pxo --in-place --name Lead4 --d 'M 810 702 L 762 702 L 740 724' --fill none --stroke '#f5b335' --stroke-width 2
pixelox shape ellipse launch.pxo --in-place --name Dot4 --rect 804,696,12x12 --fill '#f5b335'
pixelox text add launch.pxo --in-place --name N4 --content "04" --font @geometric-sans --weight 600 --size 46 --color '#f5b335' --at 1015,640 --anchor tr --anchor-to ink
pixelox text add launch.pxo --in-place --name T4 --content "Go to market" --font @geometric-sans --weight 700 --size 22 --color '#f5b335' --at 1015,694 --anchor tr --anchor-to ink
pixelox text add launch.pxo --in-place --name B4 --content $'Open orders, then tell\neveryone at once.' --font @geometric-sans --size 18 --leading 135% --align right --color '#8a8f94' --at 1015,724 --anchor tr
pixelox export launch.pxo -o launch.png
```

Reuse:

* The fan is plain geometry on five x positions at the stem's foot (504, 522, 540, 558, 576:
  the stripe edges) and five at the cells' top (194, 367, 540, 713, 886: the cell edges).
  Trapezoid N joins stripe N to cell N. For 3 or 5 steps divide both rows again (stripes 72 /
  N px wide, cells 692 / N px wide) and write one path, one stem rectangle, and one cell per
  step.
* The rocket's anchors came from `scripts/object.js` run with `--json`: `band2` is the
  porthole center; the nozzle's lower edge is 10 px under the stem's top, so the stripes
  start inside the nozzle.
* Each leader ends on its own trapezoid. The fan's outer edge runs from (504,606) to
  (194,820): text on the left must stay left of that line (at y = 744 the edge is at x = 304,
  at y = 768 at x = 269), which is why the second line of a lower note is the short one.
* The map is a backdrop, not data: one pale `land=` tint and `stroke=none`. To show where
  something is, add `points=` or `highlight=` to the same `render d3` line and darken the
  land to `#e2e5e8`; then it needs a caption.
* Change first: the four titles and notes, the four icon names, the four step colors (each
  color is written in a cell, a fan, a stripe, a leader, a dot, a number, and a title).

## 13. Variations and pairings

* **Closest relative: i046.** The same idea (rocket, four fanned stripes, numbers 01 to 04)
  on a dark space ground: there the fan folds into four upright paper columns that carry the
  numbers and labels themselves, the four callouts hang beside the rocket with icons instead
  of beside the fan, and a globe and a planet replace the world map. Use i046 for a dark,
  showy slide; use this one for a light report page.
* **Other rockets:** i038 cuts the rocket itself into four numbered bands with callouts on
  both sides (the object is the list); i020 does the same with three bands and all text on
  the right; i039 runs five numbered bars from the rocket's side to rows of text; i041
  stacks six numbered steps into the exhaust column under the rocket. Here the rocket is
  whole and the list lives in its exhaust.
* **With a title:** the original has none. Put a two-line title top left at 6% W, 7% H (54
  px, 700) over the map, or center it under the cells in a 4:5 canvas (1080 x 1350) and add
  C-source-line.
* **Other heads on the same fan:** swap the rocket for `kind=bulb` (four inputs to one
  idea) or a C-icon-badge with `trophy` or `target`; the fan then reads as a spotlight or a
  road. Turn the whole fan upside down (cells on top) for "one source, four outputs".
* **With numbers:** set a value in each cell instead of the icon (C-stat-row inside the
  cells, 44 px white) and keep the callouts as the labels. Dark version: ground `#16233b`,
  map `#1e2e4a`, notes `#c4cdd0`, the same four hues.

## 14. Pitfalls

* **Unequal cells or fans that look like amounts.** Four equal cells say "four steps". If
  one cell is wider, readers take it for a bigger share. When the steps do have sizes
  (budget, weeks), keep the cells equal and print the figure in each cell, or use
  C-percent-bar under the fan; never widen a trapezoid by eye. This is the data-integrity
  rule for this piece.
* **Callout order read as 02, 03, 01, 04.** The arch puts the inner steps on top. Keep the
  numbers large (46 px) and in color, and keep the cells in order left to right, so the
  numbering wins over the position.
* **Leaders that end on the wrong color.** A slate leader that stops on the cyan trapezoid
  ties the note to the wrong step. End each leader inside its own trapezoid and give it the
  step's color.
* **Yellow text on white.** The fourth step's number and title in `#ffc653` nearly vanish;
  the recipe darkens the text to `#f5b335` and keeps the lighter yellow for the cell. With a
  paler step color, set its text in the gray of the notes and keep only the dot in color.
* **A map that looks like data.** One flat tint, no borders, within 6% lightness of the
  ground. A darker or two-tone map makes readers look for a legend and fights the callouts
  that lie on it.
* **Shading everything.** Only the fan is shaded. A gradient in the cells or the stripes
  flattens the perspective, because the front face must be the lightest, flattest surface.
