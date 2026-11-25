---
id: i021
title: Seven rainbow step columns with ring caps
kind: [process, list]
topic: [business, generic]
mood: [vibrant, energetic, bold, friendly]
style: [gradient, paper-cut, line-icon]
format: landscape
structure: S-row-steps
data: [process, text-only]
charts: [C-process-row, C-icon-badge]
palette: ['#ffffff', '#fdb94a', '#ff6a1d', '#f93a75', '#0cc6ef', '#9590e4', '#00cfa4']
needs: nothing
---

# i021: Seven rainbow step columns with ring caps

## 1. Concept

A wide seven-step band for a slide or a web header. Seven tall columns stand shoulder to
shoulder and run off the bottom edge, one hue each, from amber through orange, pink, cyan,
and violet to two greens. Every column has a round top that frames a white disc with the step
number, a small icon badge floating above it, and one big white word halfway down. The
message is "seven named stages, left to right". It works because each step is one vertical
strip read top to bottom (icon, number, word, note), the strips touch so the row reads as one
object, and the hue walk around the color wheel says "sequence" before any number is read.

## 2. Data story

No measured data. An ordered list of 7 steps; each step has five short text items:

* a two-digit number (01 to 07) with the label STEP over it and a 2-word sub-label under it,
  inside the white disc;
* a small line "STEP 0N", one big word of 4 to 6 letters, a 2 to 4 word sub-label, and a
  note of 7 to 9 words on 2 lines, in white on the column;
* 1 title of 2 to 4 words in small spaced caps and a deck of 1 to 4 words under it.

Placeholder content in the recipe (invented, a product from first drawing to first orders):
SKETCH, MODEL, TEST, REFINE, PRICE, PACK, SHIP, with week ranges in the discs.

## 3. Information architecture

1. **Title**, top center: tiny, wide-spaced; it labels the band and stays out of the way.
2. **Icon row**: seven small colored discs with white line icons, one above each column.
3. **Number row**: seven white discs in colored rings: STEP, the number, a sub-label.
4. **Word row**, on the columns: STEP 0N, the big word, a sub-label, a two-line note.

Reading is left to right along any row, or top to bottom inside one column. The big words
alone tell the story; everything else is detail.

## 4. Format and grid

* Landscape 2 : 1. Recipe canvas 1600 x 800.
* Seven columns of 11% W (176 px) with no gutter, from 11.5% to 88.5% W. Column centers at
  17%, 28%, 39%, 50%, 61%, 72%, 83% W. Side margins 11.5% W.
* Columns: a round cap (radius 5.5% W) centered at 38% H, then a straight body down to and
  past the bottom edge (they bleed off the canvas).
* White disc inside the cap: radius 4.5% W (72 px), same center.
* Icon badges: 78 px discs centered at 19.4% H, 61 px clear of the caps.
* Word block on each column: STEP line at 60.8% H, big word at 65.5% H, sub-label at 70.5%
  H, note from 73.8% to 78.5% H. The bottom 20% of every column is empty color.
* Title at 4.3% H, deck at 7.5% H, both centered on 50% W.

## 5. Layer stack

1. Ground: a vertical gradient from white at the top through `#efefef` at mid height to
   `#cccccc` at the bottom, so the columns seem to stand on a gray floor.
2. Seven column bodies, drawn right to left so each casts its shadow on the neighbor to its
   right. Top-to-bottom gradients: amber `#fdb94a` to `#ff951c`; orange `#ff6a1d` to
   `#ff4a20`; pink `#f93a75` to `#e60f4f`; cyan `#0cc6ef` to `#00adee`; violet `#9590e4` to
   `#857cc4`; teal green `#00cfa4` to `#00b6a3`; light green `#93ee93` to `#12f0a8`.
3. Seven ring caps: a ring 16 px thick in the column's top color around a near-white disc
   `#fafafa`, with the step number in the column's hue. Soft shadow down and right.
4. Seven icon badges in the column colors with a soft shadow; white 40 px line icons.
5. In each disc: STEP above the number, a gray sub-label under it.
6. On each column, in white: STEP 0N, the big word, a sub-label, a note.
7. Title in dark gray `#555555`, deck in light gray `#a8a8a8`.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#ffffff` | ground (to `#cccccc` at the bottom), discs, type on color | 42% | nothing |
| `#fdb94a` | column, ring, badge 1 | 8% | step 1 |
| `#ff6a1d` | column, ring, badge 2 | 8% | step 2 |
| `#f93a75` | column, ring, badge 3 | 8% | step 3 |
| `#0cc6ef` | column, ring, badge 4 | 8% | step 4 |
| `#9590e4` | column, ring, badge 5 | 8% | step 5 |
| `#00cfa4` | column, ring, badge 6 (step 7: `#93ee93`) | 16% | steps 6 and 7 |

Categorical, in spectrum order: warm on the left, cool on the right. Every hue is used three
times in its own strip (badge, ring, column) and nowhere else. The disc numbers use a
slightly deeper tone of each hue for contrast on white (`#f7a63a`, `#ff7f1a`, `#ff3e87`,
`#0a8fd0`, `#8378c4`, `#00b894`, `#6fc050`).

## 7. Text elements

Sizes are px on the 1600 px wide canvas.

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Title | 2 to 4 words | caps | neo-grotesque (Inter, Montserrat / Helvetica Neue) | 500 | 20 | 300 | - | center | `#555555` | 50% W, 4.3% H |
| Deck | 1 to 4 words | caps | neo-grotesque | 300 | 15 | 250 | - | center | `#a8a8a8` | 50% W, 7.5% H |
| Disc label (x7) | STEP | caps | condensed gothic (Oswald, Barlow Condensed / Avenir Next Condensed) | 500 | 22 | 0 | - | center | the column's deep tone | column center, 32.3% H |
| Disc number (x7) | 01 to 07 | - | condensed gothic | 700 | 67 (drawn by the script: 0.38 x the node) | 0 | - | center | the column's hue | disc center, 38% H |
| Disc sub-label (x7) | 2 words | caps | condensed gothic | 500 | 15 | 20 | - | center | `#7a7a7a` | column center, 43.3% H |
| Step line (x7) | STEP and the number | caps | condensed gothic | 400 (the original: light) | 22 | 0 | - | center | `#ffffff` | column center, 60.8% H |
| Big word (x7) | 1 word, 4 to 6 letters | caps | condensed gothic | 700 | 46 (the original: 53 in a narrower face) | 0 | - | center | `#ffffff` | column center, 65.5% H |
| Column sub-label (x7) | 2 to 4 words | caps | condensed gothic | 500 | 16 | 20 | - | center | `#ffffff` | column center, 70.5% H |
| Note (x7) | 7 to 9 words on 2 lines | sentence | neo-grotesque | 400 | 15 (the original: 12) | 0 | 19 px | center | `#ffffff` | column center, from 73.8% H |

Two families: a condensed gothic for all step type, a neutral sans for the title and notes.
Sizes 67 : 46 : 22 : 15, about 4.5 : 3 : 1.5 : 1. Condensed type is what lets a 6-letter word
fit a column 176 px wide.

## 8. Charts and diagrams

* **C-process-row**: `scripts/steps.js` with `style=ring connector=none node=176 fs=20`,
  seven blank labels, `box=11.5,27,77,24`, `colors=` the seven top colors,
  `nodefill='#fafafa'`, numbers in a condensed stack (`numfont=`, `numweight=700`). The box
  is 77% W wide so each band is 176 px, and `node=176` makes neighboring rings touch. The
  script draws the seven rings with 01 to 07 and logs their centers: x = 272, 448, 624, 800,
  976, 1152, 1328; y = 304.
* The column bodies are native `shape rect` layers, 176 px wide, from the ring centers'
  height (y = 304) to the bottom edge, in the same colors: ring and body merge into one
  round-topped column.
* No axis, no values, no legend. All columns are the same height: the row shows order, not
  amounts.
* Simplified: the original rings are thinner (10 px) and glossy; its columns carry a soft
  highlight down the left side and a dark shadow strip on the right neighbor.

## 9. Icons and illustration

* Seven line icons, 40 px, 3 px line, white, centered in the 78 px badges (the original's
  subjects: a bulb, a person in a tie, a rocket, a sprouting coin, a pie chart, rising
  columns with an arrow, a money bag). The recipe uses `pencil`, `hammer`, `users`, `wrench`,
  `tag`, `package`, `truck` for its own seven words.
* Badge centers: the node anchors' x, y = 155.
* No photos.

## 10. Effects and finish

* Column gradient: `fx overlay --kind gradient --stops 'BOTTOM@0,TOP@1' --angle 90` on each
  body; the top stop equals the ring color so the join is invisible.
* Column shadow: `fx drop-shadow --offset 16,0 --blur 26 --opacity 34` on the first column,
  copied to the rest with `fx copy` before the gradients are added.
* Ring shadow: `--offset 10,8 --blur 22 --opacity 28` on the placed node layer; badge
  shadow `--offset 0,10 --blur 18 --opacity 22`.
* Ground gradient: `layer add`, then `fill gradient` with four stops.

## 11. Hierarchy and eye path

1. The band of seven colors as a whole.
2. The white discs and their numbers, 01 to 07, left to right.
3. The big white words.
4. Icons above; sub-labels and notes below.
5. The title, last: it is the smallest dark thing on the page.

Type steps 67 : 46 : 22 : 15. The title is deliberately tiny (20 px on 1600): the band is
meant to sit under a slide's own headline.

## 12. Build recipe

Nothing to supply. Canvas 1600 x 800. All copy is invented.

```bash
pixelox new -o seven.pxo --size 1600x800 --fill '#ffffff'
pixelox layer add seven.pxo --in-place --name Floor
pixelox fill gradient seven.pxo --in-place --layer Floor --kind linear --from 0,0 --to 0,100% --stops '#ffffff@0,#efefef@0.49,#dbdbdb@0.73,#cccccc@1'
pixelox shape rect seven.pxo --in-place --name Col7 --rect 1240,304,176x496 --fill '#93ee93'
pixelox shape rect seven.pxo --in-place --name Col6 --rect 1064,304,176x496 --fill '#00cfa4'
pixelox shape rect seven.pxo --in-place --name Col5 --rect 888,304,176x496 --fill '#9590e4'
pixelox shape rect seven.pxo --in-place --name Col4 --rect 712,304,176x496 --fill '#0cc6ef'
pixelox shape rect seven.pxo --in-place --name Col3 --rect 536,304,176x496 --fill '#f93a75'
pixelox shape rect seven.pxo --in-place --name Col2 --rect 360,304,176x496 --fill '#ff6a1d'
pixelox shape rect seven.pxo --in-place --name Col1 --rect 184,304,176x496 --fill '#fdb94a'
pixelox fx drop-shadow seven.pxo --in-place --layer Col1 --offset 16,0 --blur 26 --color '#000000' --opacity 34
pixelox fx copy seven.pxo --in-place --from Col1 --to Col2,Col3,Col4,Col5,Col6,Col7
pixelox fx overlay seven.pxo --in-place --layer Col1 --kind gradient --stops '#ff951c@0,#fdb94a@1' --angle 90
pixelox fx overlay seven.pxo --in-place --layer Col2 --kind gradient --stops '#ff4a20@0,#ff6a1d@1' --angle 90
pixelox fx overlay seven.pxo --in-place --layer Col3 --kind gradient --stops '#e60f4f@0,#f93a75@1' --angle 90
pixelox fx overlay seven.pxo --in-place --layer Col4 --kind gradient --stops '#00adee@0,#0cc6ef@1' --angle 90
pixelox fx overlay seven.pxo --in-place --layer Col5 --kind gradient --stops '#857cc4@0,#9590e4@1' --angle 90
pixelox fx overlay seven.pxo --in-place --layer Col6 --kind gradient --stops '#00b6a3@0,#00cfa4@1' --angle 90
pixelox fx overlay seven.pxo --in-place --layer Col7 --kind gradient --stops '#12f0a8@0,#93ee93@1' --angle 90
pixelox render js scripts/steps.js -o nodes.png --size 1600x800 --param box=11.5,27,77,24 --param style=ring --param connector=none --param node=176 --param fs=20 --param labels=' | | | | | | ' --param colors='#fdb94a,#ff6a1d,#f93a75,#0cc6ef,#9590e4,#00cfa4,#93ee93' --param nodefill='#fafafa' --param numfont='Avenir Next Condensed, Oswald, Barlow Condensed, Arial Narrow, sans-serif' --param numweight=700
pixelox comp place seven.pxo nodes.png --in-place --as Nodes --at 0,0
pixelox fx drop-shadow seven.pxo --in-place --layer Nodes --offset 10,8 --blur 22 --color '#000000' --opacity 28
pixelox shape ellipse seven.pxo --in-place --name Badge1 --rect 233,116,78x78 --fill '#fdb94a'
pixelox shape ellipse seven.pxo --in-place --name Badge2 --rect 409,116,78x78 --fill '#ff6a1d'
pixelox shape ellipse seven.pxo --in-place --name Badge3 --rect 585,116,78x78 --fill '#f93a75'
pixelox shape ellipse seven.pxo --in-place --name Badge4 --rect 761,116,78x78 --fill '#0cc6ef'
pixelox shape ellipse seven.pxo --in-place --name Badge5 --rect 937,116,78x78 --fill '#9590e4'
pixelox shape ellipse seven.pxo --in-place --name Badge6 --rect 1113,116,78x78 --fill '#00cfa4'
pixelox shape ellipse seven.pxo --in-place --name Badge7 --rect 1289,116,78x78 --fill '#93ee93'
pixelox fx drop-shadow seven.pxo --in-place --layer Badge1 --offset 0,10 --blur 18 --color '#000000' --opacity 22
pixelox fx copy seven.pxo --in-place --from Badge1 --to Badge2,Badge3,Badge4,Badge5,Badge6,Badge7
pixelox shape icon seven.pxo pencil --in-place --name Icon1 --size 40 --at 272,155 --anchor c --color '#ffffff' --stroke-width 3
pixelox shape icon seven.pxo hammer --in-place --name Icon2 --size 40 --at 448,155 --anchor c --color '#ffffff' --stroke-width 3
pixelox shape icon seven.pxo users --in-place --name Icon3 --size 40 --at 624,155 --anchor c --color '#ffffff' --stroke-width 3
pixelox shape icon seven.pxo wrench --in-place --name Icon4 --size 40 --at 800,155 --anchor c --color '#ffffff' --stroke-width 3
pixelox shape icon seven.pxo tag --in-place --name Icon5 --size 40 --at 976,155 --anchor c --color '#ffffff' --stroke-width 3
pixelox shape icon seven.pxo package --in-place --name Icon6 --size 40 --at 1152,155 --anchor c --color '#ffffff' --stroke-width 3
pixelox shape icon seven.pxo truck --in-place --name Icon7 --size 40 --at 1328,155 --anchor c --color '#ffffff' --stroke-width 3
pixelox text add seven.pxo --in-place --name Cap1 --content "STEP" --font @condensed --weight 500 --size 22 --color '#f7a63a' --at 272,258 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name When1 --content "WEEKS 1-2" --font @condensed --weight 500 --size 15 --tracking 20 --color '#7a7a7a' --at 272,346 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name Cap2 --content "STEP" --font @condensed --weight 500 --size 22 --color '#ff7f1a' --at 448,258 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name When2 --content "WEEKS 3-4" --font @condensed --weight 500 --size 15 --tracking 20 --color '#7a7a7a' --at 448,346 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name Cap3 --content "STEP" --font @condensed --weight 500 --size 22 --color '#ff3e87' --at 624,258 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name When3 --content "WEEK 5" --font @condensed --weight 500 --size 15 --tracking 20 --color '#7a7a7a' --at 624,346 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name Cap4 --content "STEP" --font @condensed --weight 500 --size 22 --color '#0a8fd0' --at 800,258 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name When4 --content "WEEKS 6-7" --font @condensed --weight 500 --size 15 --tracking 20 --color '#7a7a7a' --at 800,346 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name Cap5 --content "STEP" --font @condensed --weight 500 --size 22 --color '#8378c4' --at 976,258 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name When5 --content "WEEK 8" --font @condensed --weight 500 --size 15 --tracking 20 --color '#7a7a7a' --at 976,346 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name Cap6 --content "STEP" --font @condensed --weight 500 --size 22 --color '#00b894' --at 1152,258 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name When6 --content "WEEK 9" --font @condensed --weight 500 --size 15 --tracking 20 --color '#7a7a7a' --at 1152,346 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name Cap7 --content "STEP" --font @condensed --weight 500 --size 22 --color '#6fc050' --at 1328,258 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name When7 --content "WEEK 10" --font @condensed --weight 500 --size 15 --tracking 20 --color '#7a7a7a' --at 1328,346 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name No1 --content "STEP 01" --font @condensed --weight 400 --size 22 --color '#ffffff' --at 272,486 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name Word1 --content "SKETCH" --font @condensed --weight 700 --size 46 --color '#ffffff' --at 272,524 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name Sub1 --content "ON PAPER FIRST" --font @condensed --weight 500 --size 16 --tracking 20 --color '#ffffff' --at 272,564 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name Note1 --content $'Draw twenty versions\nbefore you pick one.' --font @neo-grotesque --size 15 --leading 19 --align center --color '#ffffff' --at 272,590 --anchor t
pixelox text add seven.pxo --in-place --name No2 --content "STEP 02" --font @condensed --weight 400 --size 22 --color '#ffffff' --at 448,486 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name Word2 --content "MODEL" --font @condensed --weight 700 --size 46 --color '#ffffff' --at 448,524 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name Sub2 --content "ONE ROUGH BUILD" --font @condensed --weight 500 --size 16 --tracking 20 --color '#ffffff' --at 448,564 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name Note2 --content $'Cardboard and tape\nbeat a perfect render.' --font @neo-grotesque --size 15 --leading 19 --align center --color '#ffffff' --at 448,590 --anchor t
pixelox text add seven.pxo --in-place --name No3 --content "STEP 03" --font @condensed --weight 400 --size 22 --color '#ffffff' --at 624,486 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name Word3 --content "TEST" --font @condensed --weight 700 --size 46 --color '#ffffff' --at 624,524 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name Sub3 --content "WITH TEN USERS" --font @condensed --weight 500 --size 16 --tracking 20 --color '#ffffff' --at 624,564 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name Note3 --content $'Watch people use it\nand say nothing.' --font @neo-grotesque --size 15 --leading 19 --align center --color '#ffffff' --at 624,590 --anchor t
pixelox text add seven.pxo --in-place --name No4 --content "STEP 04" --font @condensed --weight 400 --size 22 --color '#ffffff' --at 800,486 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name Word4 --content "REFINE" --font @condensed --weight 700 --size 46 --color '#ffffff' --at 800,524 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name Sub4 --content "FIX THE TOP THREE" --font @condensed --weight 500 --size 16 --tracking 20 --color '#ffffff' --at 800,564 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name Note4 --content $'Change only what\ntesters tripped on.' --font @neo-grotesque --size 15 --leading 19 --align center --color '#ffffff' --at 800,590 --anchor t
pixelox text add seven.pxo --in-place --name No5 --content "STEP 05" --font @condensed --weight 400 --size 22 --color '#ffffff' --at 976,486 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name Word5 --content "PRICE" --font @condensed --weight 700 --size 46 --color '#ffffff' --at 976,524 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name Sub5 --content "ASK, THEN SET IT" --font @condensed --weight 500 --size 16 --tracking 20 --color '#ffffff' --at 976,564 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name Note5 --content $'Charge what five\nbuyers said yes to.' --font @neo-grotesque --size 15 --leading 19 --align center --color '#ffffff' --at 976,590 --anchor t
pixelox text add seven.pxo --in-place --name No6 --content "STEP 06" --font @condensed --weight 400 --size 22 --color '#ffffff' --at 1152,486 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name Word6 --content "PACK" --font @condensed --weight 700 --size 46 --color '#ffffff' --at 1152,524 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name Sub6 --content "BOX AND LABEL" --font @condensed --weight 500 --size 16 --tracking 20 --color '#ffffff' --at 1152,564 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name Note6 --content $'Design the box last,\naround the product.' --font @neo-grotesque --size 15 --leading 19 --align center --color '#ffffff' --at 1152,590 --anchor t
pixelox text add seven.pxo --in-place --name No7 --content "STEP 07" --font @condensed --weight 400 --size 22 --color '#ffffff' --at 1328,486 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name Word7 --content "SHIP" --font @condensed --weight 700 --size 46 --color '#ffffff' --at 1328,524 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name Sub7 --content "FIRST 100 ORDERS" --font @condensed --weight 500 --size 16 --tracking 20 --color '#ffffff' --at 1328,564 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name Note7 --content $'Send a small batch\nand read every reply.' --font @neo-grotesque --size 15 --leading 19 --align center --color '#ffffff' --at 1328,590 --anchor t
pixelox text add seven.pxo --in-place --name Title --content "FROM SKETCH TO SHELF" --font @neo-grotesque --weight 500 --size 20 --tracking 300 --color '#555555' --at 800,34 --anchor c --anchor-to ink
pixelox text add seven.pxo --in-place --name Deck --content "SEVEN STEPS, TEN WEEKS" --font @neo-grotesque --weight 300 --size 15 --tracking 250 --color '#a8a8a8' --at 800,60 --anchor c --anchor-to ink
pixelox export seven.pxo -o seven.png
```

Reuse: change the seven big words first (6 letters at most at 46 px), then the sub-labels
and notes. The column centers come from `lab/anchors.py scripts/steps.js --size 1600x800
--param box=11.5,27,77,24 --param style=ring --param node=176 --param fs=20 --param
labels=' | | | | | | '`. For N columns keep the box 77% W wide, set `node=` to 1232 / N, give
N blank labels, and make each body rect that wide: 5 columns are 246 px wide and take words
of up to 9 letters at 60 px; 4 columns suit a 1600 x 900 canvas.

## 13. Variations and pairings

* **One hue:** seven steps from `#dbeafe` to `#1e3a8a` (light to dark) for a calm corporate
  version; type on the three lightest columns then turns dark `#1e3a8a`.
* **Values instead of words:** put a figure in the big-word place ("12%", "4.2M") and the
  measure in the sub-label: a seven-KPI strip. If the figures share one unit, do not use
  equal columns: use C-columns so heights carry the values.
* **Portrait:** turn the row into C-process-spine with the same seven colors, ring nodes,
  and notes to one side (`side=one`).
* **Dark ground:** ground `#14161f` to `#0b0c12`, discs `#ffffff`, title `#c8cbd6`; the hues
  stay.
* Pairs with C-timeline along the bottom fifth of the columns (dates under each step) and
  with C-title-block above when the piece stands alone.

## 14. Pitfalls

* **Words too long for the column.** A 176 px column holds 6 condensed caps at 46 px. Longer
  names need fewer columns or a smaller size for all seven: never shrink one word alone.
* **Equal columns read as equal amounts.** These columns are all the same height on
  purpose. If the steps carry numbers, a reader will compare column heights: either keep
  them equal and print no numbers, or switch to a real chart. This is the data-integrity
  rule for this design.
* **Hues out of order.** The spectrum order is the second numbering. Shuffled colors make
  the row look like seven categories, not seven steps.
* **Text over the gradient's light end.** White 15 px notes fail on the light green column
  (lightness about 85%). Darken the last column's bottom stop or set that column's type in
  dark green.
* **Gutters between columns.** A 10 px gap turns one band into seven cards and kills the
  cast shadows. Keep the bodies touching.
* **Tiny notes.** The original's notes are 12 px on 1600: unreadable. Use 15 px or cut them.
