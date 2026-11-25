---
id: i007
title: Serpentine ribbon through five nodes with side banners
kind: [process, list]
topic: [business, generic]
mood: [corporate, clean, cool, serious]
style: [paper-cut, line-icon, gradient]
format: square
structure: S-spine-steps
data: [process, text-only]
charts: [C-process-spine, C-icon-badge, C-ribbon-label]
palette: ['#f3f3f3', '#1b7fa0', '#234a63', '#6583b5', '#3f4c6b', '#2c333d', '#fbfbfb']
needs: nothing
---

# i007: Serpentine ribbon through five nodes with side banners

## 1. Concept

A square five-step sheet on a pale gray ground. Five white icon discs zig-zag down the middle
(left, right, left, right, left), a thick ribbon snakes around them in one continuous serpentine curve
that changes color at every disc, and each disc owns a flat banner that runs to the nearest
page edge and carries the step's title and a short paragraph. The message is "five linked
steps, in this order". It works because the ribbon does the numbering: the eye cannot leave
the curve, so it visits the discs in order, and each ribbon segment has the color of its own
banner, which ties text to step without a single leader line.

## 2. Data story

No numbers. Text only, as an ordered list:

* 5 steps. Each: a title of one caps word followed by a two-digit number (01 to 05), and a
  paragraph of 20 to 25 words set on 6 short justified lines.
* 1 title of 2 words on 2 lines, with a row of 5 small color keys under it (one per step).
* The five colors run from a clear teal to a near-black slate: order is also shown as "gets
  darker toward the end", with one lighter periwinkle in the middle so neighbors differ.

Placeholder content in the recipe (invented, a project kickoff): DEFINE 01, IDEATE 02, DIVIDE
03, ASSIGN 04, REVIEW 05, each with a one-sentence note of 21 to 23 words.

## 3. Information architecture

1. **Title**, top right, in the only empty corner: two lines of gray condensed caps and five
   hexagon keys that preview the five step colors.
2. **Step 1**, top left: banner from the left edge, disc at 42% W, ribbon wrapping the disc's
   left side.
3. **Step 2**: disc at 58% W, one step lower, ribbon wrapping its right side, banner from the
   right edge.
4. **Steps 3 to 5** repeat the pair: odd steps left, even steps right.

The reader is led by the ribbon: it enters at the upper right of disc 1, swings around the
left, crosses the center axis, swings around the right of disc 2, and so on. Banners sit level
with their discs, so the text is always found by looking sideways from the icon.

## 4. Format and grid

* Square. Recipe canvas 1080 x 1080. Text margin 3.6% W from the outer edges.
* Two node columns: odd steps centered on 42% W, even steps on 58% W. Node centers at 18%,
  34%, 50%, 66%, 82% H: one step every 16% H. The horizontal offset between the columns
  (16% W) equals the vertical step, so every neighbor sits on a 45 degree diagonal.
* Node disc diameter 16% W (172 px). Ribbon: a ring arc around each node, centerline radius
  11.3% W (122 px), 2.6% W (28 px) thick. Consecutive arcs are tangent where they meet: on
  the center axis (50% W) at 10%, 26%, 42%, 58%, 74%, 90% H.
* Banners: 24% H tall (260 px), centered on their node's height. Left banners
  `0,y,31.2% x 24%`, right banners `68.7%,y,31.3% x 24%`, with y = 5.9%, 21.9%, 38%, 54%, 70%.
  The inner end of each banner is a concave arc on the node's own center, radius 16.1% W
  (174 px), so a 4% W strip of ground separates banner and ribbon.
* Title block centered on 84% W: lines at 7.5% and 11.5% H, keys at 15.7% H.
* Banner text: left edge 3.6% W (left banners) or 78% W (right banners); column 18.9% W wide.

## 5. Layer stack

1. Ground: pale gray `#f3f3f3`, flat.
2. Five banners, back to front in step order: teal `#1b7fa0`, deep blue `#234a63`, periwinkle
   `#6583b5`, slate indigo `#3f4c6b`, charcoal navy `#2c333d`. Each has a soft drop shadow
   falling 22 px down and a faint left-to-right gradient (about 18% darker at the left end,
   10% lighter at the right).
3. Five ribbon arcs, each a 270 degree ring arc in its banner's color, drawn from step 5 up
   to step 1 so the earlier ribbon overlaps the later one at each joint.
4. Five node discs: near-white `#fbfbfb` with a 12 px pale rim `#e6e8e7` inside the edge and
   a soft shadow to the lower right (the original: a raised plate with a radial white-to-gray
   center).
5. Five line icons in charcoal `#383838`, 86 px, centered on the discs.
6. Banner titles and paragraphs in white.
7. Title in mid gray `#6b6b6b`; five hexagons, 22 px across, in the five step colors.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#f3f3f3` | ground | 52% | nothing |
| `#1b7fa0` | banner, ribbon, key 1 | 8% | step 1 |
| `#234a63` | banner, ribbon, key 2 | 8% | step 2 |
| `#6583b5` | banner, ribbon, key 3 | 8% | step 3 |
| `#3f4c6b` | banner, ribbon, key 4 | 8% | step 4 |
| `#2c333d` | banner, ribbon, key 5 | 8% | step 5 |
| `#fbfbfb` | node discs; all banner text (`#ffffff`) | 8% | nothing |

Categorical inside one cool family: five blues and slates, no warm color. Steps are told
apart by lightness (mid, dark, light, dark, darkest), not by hue, so the sheet stays corporate
and prints well in one ink. Gray `#6b6b6b` (title) and charcoal `#383838` (icons) are the only
neutrals.

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Title | 2 lines, 1 word and 1 to 2 words (6 and 12 letters) | caps | condensed gothic (Oswald, Bebas Neue / Avenir Next Condensed) | 700 | 38 | 0 | 42 px pitch | center | `#6b6b6b` | 84% W, block center 10.5% H |
| Step title (x5) | one word and a two-digit number, up to 14 characters | caps | condensed gothic (Bebas Neue, Oswald Light / Avenir Next Condensed) | 400 (the original: light) | 32 | 0 | - | left | `#ffffff` | 3.6% W or 78% W; cap top 5.2% H under the banner's top edge |
| Step note (x5) | 20 to 25 words on 6 lines | sentence | neo-grotesque (Inter, Roboto / Helvetica Neue) | 400 | 18 | 0 | 20 px (110%) | justify | `#ffffff` | same left edge; first line 3.3% H under the title's cap top; box 204 px wide |

Two families: a condensed gothic for everything that names, a neutral sans for the notes.
Sizes 38 : 32 : 18, about 2.1 : 1.8 : 1. The page has no large type at all: the ribbon is the
display element.

## 8. Charts and diagrams

* **C-process-spine**, used twice with `connector=none` to get the zig-zag: `scripts/steps.js`
  `dir=v badges=none node=172`, blank labels. Column A (steps 1, 3, 5): `box=34,2,16,96`,
  three nodes at (453,194), (453,540), (453,886). Column B (steps 2, 4): `box=50,18,16,64`,
  two nodes at (626,367), (626,713). Rule: a column's box is 32% H per node; the second
  column starts 16% lower and 16% further right; `node=` equals the box width in px. The
  script logs every center (`lab/anchors.py`): icons, ribbons, and banners hang on them.
* **The wrapped ribbon is drawn natively.** C-snake-steps (`scripts/steps.js style=snake`)
  draws a serpentine ribbon, but as one S-curve in one color that runs through the node
  centers; here the ribbon wraps around the outside of every disc, leaves a strip of ground
  between itself and the disc, and changes color at each step. The recipe therefore draws
  one `shape path` arc per node, hung on the anchors: every arc starts
  and ends on the center axis, half a step above and half a step below its node (node at
  y = 194: from 540,108 to 540,281), radius 122.3 px (0.707 x the 172.8 px step), large-arc
  flag 1, sweep flag 0 for a node left of the axis and 1 for a node right of it. No axis, no
  values, no legend: the five hexagons under the title are a decorative key.
* **C-ribbon-label** variant for the banners: a rectangle whose inner end is cut by an arc
  (`shape path` with one `A 174 174`), concentric with the node.
* Simplified: in the original each ribbon tapers into a pointed tail that tucks under the
  next disc, so the joints look like a figure of eight; the ring is slightly smaller (10.5% W
  centerline radius) than tangency requires and the tails make up the difference. The recipe
  uses plain tangent arcs of constant width.

## 9. Icons and illustration

* Five line icons, 86 px (half the disc), 5 px line, charcoal `#383838`, centered on the
  node anchors: `target` (01), `lightbulb` (02), `chart-pie` (03), `user` (04), `settings`
  (05). The original draws the same five subjects as thin outline pictograms (a target with
  an arrow, a bulb, a pie chart with one hatched slice, a bust, a cog).
* Five hexagon keys: `shape polygon --sides 6 --radius 11`, 39 px apart, centered under the
  title.
* No photos, no illustration.

## 10. Effects and finish

* Paper-cut depth from two shadows: banners `fx drop-shadow --offset 0,22 --blur 30
  --opacity 24` (the shadow falls on the ground below each banner and under the next one);
  discs `--offset 8,12 --blur 22 --opacity 30`.
* Banner gradient: `fx overlay --kind gradient --stops '#00000030@0,#ffffff1c@1' --angle 0`,
  copied to all five with `fx copy`.
* Disc rim: `fx stroke --width 12 --position inside --color '#e6e8e7'` on the placed node
  layers.
* The ribbons are flat. No texture, no outlines.

## 11. Hierarchy and eye path

1. The ribbon and the five discs: the only curved, shadowed shapes, on the center axis.
2. The first banner (top left, the most saturated color), its title, then its note.
3. Down the curve: disc, banner, disc, banner, alternating sides.
4. The title, top right: quiet gray, read last or as a label for the whole.

Type steps 38 : 32 : 18. Emphasis comes from color area and shadow, not from type size.

## 12. Build recipe

Nothing to supply. Canvas 1080 x 1080. All copy is invented.

```bash
pixelox new -o snake.pxo --size 1080x1080 --fill '#f3f3f3'
pixelox shape path snake.pxo --in-place --name Banner1 --d 'M 0 64 L 337 64 A 174 174 0 0 0 337 324 L 0 324 Z' --fill '#1b7fa0'
pixelox shape path snake.pxo --in-place --name Banner2 --d 'M 1080 237 L 742 237 A 174 174 0 0 1 742 497 L 1080 497 Z' --fill '#234a63'
pixelox shape path snake.pxo --in-place --name Banner3 --d 'M 0 410 L 337 410 A 174 174 0 0 0 337 670 L 0 670 Z' --fill '#6583b5'
pixelox shape path snake.pxo --in-place --name Banner4 --d 'M 1080 583 L 742 583 A 174 174 0 0 1 742 843 L 1080 843 Z' --fill '#3f4c6b'
pixelox shape path snake.pxo --in-place --name Banner5 --d 'M 0 756 L 337 756 A 174 174 0 0 0 337 1016 L 0 1016 Z' --fill '#2c333d'
pixelox fx drop-shadow snake.pxo --in-place --layer Banner1 --offset 0,22 --blur 30 --color '#000000' --opacity 24
pixelox fx overlay snake.pxo --in-place --layer Banner1 --kind gradient --stops '#00000030@0,#ffffff1c@1' --angle 0
pixelox fx copy snake.pxo --in-place --from Banner1 --to Banner2,Banner3,Banner4,Banner5
pixelox shape path snake.pxo --in-place --name Ribbon5 --d 'M 540 799 A 122.3 122.3 0 1 0 540 972' --fill none --stroke '#2c333d' --stroke-width 28
pixelox shape path snake.pxo --in-place --name Ribbon4 --d 'M 540 626 A 122.3 122.3 0 1 1 540 799' --fill none --stroke '#3f4c6b' --stroke-width 28
pixelox shape path snake.pxo --in-place --name Ribbon3 --d 'M 540 454 A 122.3 122.3 0 1 0 540 626' --fill none --stroke '#6583b5' --stroke-width 28
pixelox shape path snake.pxo --in-place --name Ribbon2 --d 'M 540 281 A 122.3 122.3 0 1 1 540 454' --fill none --stroke '#234a63' --stroke-width 28
pixelox shape path snake.pxo --in-place --name Ribbon1 --d 'M 540 108 A 122.3 122.3 0 1 0 540 281' --fill none --stroke '#1b7fa0' --stroke-width 28
pixelox render js scripts/steps.js -o nodes-a.png --size 1080x1080 --param box=34,2,16,96 --param dir=v --param connector=none --param badges=none --param node=172 --param labels=' | | ' --param colors='#fbfbfb'
pixelox render js scripts/steps.js -o nodes-b.png --size 1080x1080 --param box=50,18,16,64 --param dir=v --param connector=none --param badges=none --param node=172 --param labels=' | ' --param colors='#fbfbfb'
pixelox comp place snake.pxo nodes-a.png --in-place --as NodesA --at 0,0
pixelox comp place snake.pxo nodes-b.png --in-place --as NodesB --at 0,0
pixelox fx stroke snake.pxo --in-place --layer NodesA --width 12 --position inside --color '#e6e8e7'
pixelox fx drop-shadow snake.pxo --in-place --layer NodesA --offset 8,12 --blur 22 --color '#000000' --opacity 30
pixelox fx copy snake.pxo --in-place --from NodesA --to NodesB
pixelox shape icon snake.pxo target --in-place --name Ic1 --size 86 --at 453,194 --anchor c --color '#383838' --stroke-width 5
pixelox shape icon snake.pxo lightbulb --in-place --name Ic2 --size 86 --at 626,367 --anchor c --color '#383838' --stroke-width 5
pixelox shape icon snake.pxo chart-pie --in-place --name Ic3 --size 86 --at 453,540 --anchor c --color '#383838' --stroke-width 5
pixelox shape icon snake.pxo user --in-place --name Ic4 --size 86 --at 626,713 --anchor c --color '#383838' --stroke-width 5
pixelox shape icon snake.pxo settings --in-place --name Ic5 --size 86 --at 453,886 --anchor c --color '#383838' --stroke-width 5
pixelox text add snake.pxo --in-place --name H1 --content "DEFINE 01" --font @condensed --weight 400 --size 32 --color '#ffffff' --at 39,120 --anchor tl --anchor-to ink
pixelox text add snake.pxo --in-place --name B1 --content "Write one sentence that says what will be different in ninety days, who will notice it first, and how you will measure it." --font @neo-grotesque --size 18 --leading 20 --align justify --box 204x130 --color '#ffffff' --at 39,156 --anchor tl
pixelox text add snake.pxo --in-place --name H2 --content "IDEATE 02" --font @condensed --weight 400 --size 32 --color '#ffffff' --at 842,293 --anchor tl --anchor-to ink
pixelox text add snake.pxo --in-place --name B2 --content "Collect twenty rough ideas in a single hour, then keep only the three that cost the least money and time to test." --font @neo-grotesque --size 18 --leading 20 --align justify --box 204x130 --color '#ffffff' --at 842,329 --anchor tl
pixelox text add snake.pxo --in-place --name H3 --content "DIVIDE 03" --font @condensed --weight 400 --size 32 --color '#ffffff' --at 39,466 --anchor tl --anchor-to ink
pixelox text add snake.pxo --in-place --name B3 --content "Cut the plan into parts that one person can finish in a week, and put a rough share of the budget on each." --font @neo-grotesque --size 18 --leading 20 --align justify --box 204x130 --color '#ffffff' --at 39,502 --anchor tl
pixelox text add snake.pxo --in-place --name H4 --content "ASSIGN 04" --font @condensed --weight 400 --size 32 --color '#ffffff' --at 842,639 --anchor tl --anchor-to ink
pixelox text add snake.pxo --in-place --name B4 --content "Every part gets one name beside it. Shared tasks slip between people, so a whole team is never written in the owner column." --font @neo-grotesque --size 18 --leading 20 --align justify --box 204x130 --color '#ffffff' --at 842,675 --anchor tl
pixelox text add snake.pxo --in-place --name H5 --content "REVIEW 05" --font @condensed --weight 400 --size 32 --color '#ffffff' --at 39,812 --anchor tl --anchor-to ink
pixelox text add snake.pxo --in-place --name B5 --content "Meet for fifteen minutes each Monday, move the cards across the board, and change the plan when the numbers say so." --font @neo-grotesque --size 18 --leading 20 --align justify --box 204x130 --color '#ffffff' --at 39,848 --anchor tl
pixelox text add snake.pxo --in-place --name Title --content $'PROJECT\nKICKOFF PLAN' --font @condensed --weight 700 --size 38 --leading 42 --align center --color '#6b6b6b' --at 908,114 --anchor c
pixelox shape polygon snake.pxo --in-place --name Hex1 --center 830,170 --radius 11 --sides 6 --fill '#1b7fa0'
pixelox shape polygon snake.pxo --in-place --name Hex2 --center 869,170 --radius 11 --sides 6 --fill '#234a63'
pixelox shape polygon snake.pxo --in-place --name Hex3 --center 908,170 --radius 11 --sides 6 --fill '#6583b5'
pixelox shape polygon snake.pxo --in-place --name Hex4 --center 947,170 --radius 11 --sides 6 --fill '#3f4c6b'
pixelox shape polygon snake.pxo --in-place --name Hex5 --center 986,170 --radius 11 --sides 6 --fill '#2c333d'
pixelox export snake.pxo -o snake.png
```

Reuse: change the five titles and notes first. The geometry hangs on the node anchors
(`lab/anchors.py scripts/steps.js --size 1080x1080 --param box=34,2,16,96 --param dir=v
--param node=172 --param labels=' | | '`): for a node at (cx, cy) the banner runs from
cy - 130 to cy + 130, its arc ends are 116 px from cx (337 = 453 - 116; 742 = 626 + 116), the
step title's cap top is at cy - 74 and the note starts at cy - 38. For 4 steps drop Banner5,
Ribbon5, the third label of column A, and its texts; for 6 steps use a 1080 x 1250 canvas and
add a third node to column B.

## 13. Variations and pairings

* **Straight spine or plain snake:** when there are 6 to 8 steps, drop the wrapped ribbon
  and the banners: one C-process-spine column with `side=alt`, or C-snake-steps
  (`style=snake`: one call, a one-color S-curved ribbon through the nodes, text beside each
  node); keep the banner colors as node colors.
* **Numbers instead of icons:** drop `badges=none` and give the nodes a dark `ink`: the discs
  then carry 01 to 05 and the banner titles lose their numbers.
* **Warm or brand palette:** any five steps of one hue family work (for example `#f59e0b`,
  `#ea580c`, `#dc2626`, `#9f1239`, `#4a044e`); keep the lightest in the middle or at the top.
* **Landscape:** turn the grid 90 degrees on a 1600 x 900 canvas: nodes alternate above and
  below a horizontal axis, banners become columns that reach the top and bottom edges.
* Pairs with C-stat-row under the last banner (three numbers that prove the process works)
  and with C-timeline when each step has a date.

## 14. Pitfalls

* **Arcs that do not meet.** If the two columns are not offset by exactly the vertical step,
  the 270 degree arcs overlap or leave gaps. Keep the column offset equal to the step (16% W
  and 16% H on a square) and the radius at 0.707 x the step.
* **Titles that run into the cut.** The banner's free width at mid height is only 22% W. A
  title over 14 characters or a note over 6 lines hits the concave edge: shorten the copy,
  never shrink the type under 18 px.
* **Justified rivers.** Six 22-character lines set justified open wide word gaps when a line
  holds only three words. Write notes with short words, or set them flush left.
* **Order implied but not stated.** The ribbon implies a sequence. If the five items are not
  ordered (options, departments), this layout tells a false story: use C-hub-spokes or
  C-icon-grid instead. This is the data-integrity rule for every path diagram.
* **Darkness read as amount.** The colors darken toward step 5; readers may read "more" or
  "worse". If one step matters most, give it the single saturated color and set the others in
  slates.
* **Shadows on everything.** Only the banners and the discs cast shadows; a shadow on the
  ribbon or the text turns the paper-cut look muddy.
