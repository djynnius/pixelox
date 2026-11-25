---
id: i054
title: Arrow-loop explainer around a three-class map
kind: [explainer, geographic]
topic: [politics, education]
mood: [friendly, clean, editorial, serious]
style: [flat, editorial, illustrated]
format: portrait
structure: S-map-callouts
data: [geography, process, counted-units]
charts: [C-us-choropleth, C-icon-array, C-pie, C-stat-block, C-title-block, C-source-line]
palette: ['#ffffff', '#e4e1e0', '#cf1c21', '#206fb6', '#b5b3b5', '#3e3b3b']
needs: nothing
---

# i054: Arrow-loop explainer around a three-class map

## 1. Concept

A classroom page that explains an indirect election (voters choose electors, electors choose
the office-holder) in six short stops. A conveyor of fat pale-gray arrows carries the story
around the page: two arrows across the top, one long arrow down the right side, two arrows
back along the bottom, and inside this "U" sits a map of the country where every state shows
its number of electors and one of three colors. Tiny person pictograms do the arithmetic at
each stop. It works because the arrows are both the layout and the reading order, so a page
with a dozen separate little texts never needs a numbered list, and because all counting is
done with countable people, not with charts a child would have to learn to read.

## 2. Data story

* **Process, 6 stops:** a rule (how a state's number is made), the vote, what the vote
  really selects, winner-takes-all, the count against a majority line, the final formal
  vote. Each stop is 15 to 40 words.
* **Counted units, small sums:** a fixed part plus a part by population, both single
  digits, with their sum (drawn as dark and gray figures); a column of 8 to 10 figures per
  side for the electors of one state; two blocks of about 10 figures (all one color
  against split colors) for "not split"; two rows of about 35 tiny figures meeting at a
  majority marker.
* **Part-to-whole, one near-even split:** two shares within 2 points of each other as a
  pie; the larger side is labelled as the winner.
* **Geography, 51 regions:** one whole number per region (from a low single digit to
  several dozen; the total, in the hundreds, is shown as a hero number above the map) and
  one of 3 classes: leans to one side (blue), leans to the other (red), neither (gray);
  each class holds between a quarter and under half of the regions.

Placeholder data used in the recipe (example data, invented: delegates per region in a
441-delegate vote, 1 fixed per region plus seats by population, from 2 to 47 per region and
following no real apportionment; a made-up west, middle, and east split): 18 regions lean
to Party A (130 delegates), 17 to Party B (156), 16 are open (155); 221 make a majority;
the example region has 1 + 6 = 7 delegates; the sample vote splits 52% to 48%.

## 3. Information architecture

1. **Title** (top left, three stacked lines in black, red, and blue) with a seven-line
   intro beside it and the publisher's mark at the right.
2. **Star-and-stripe rule** across the page.
3. **"How it works"** (a small slab heading), then the arrow conveyor:
   arrow 1 (the rule, with a small sum drawn in figures), arrow 2 (the vote, with a campaign
   button on its top edge), the down arrow (what the voter really chooses: a hand marking a
   ballot, then two columns of figures), the corner (winner takes all: the pie), arrow 4
   back to the left (seats are not split: two blocks of figures), arrow 5 (the count: two
   rows of figures and the majority marker).
4. **"Finally"**: a bold lead-in and a paragraph at the bottom left, where the last arrow
   points.
5. **The count per region** (a second slab heading of 3 words) inside the U: legend of two
   color keys with curved pointers, "Total" and the hero number, the map with a number and
   a name on every state.
6. **Sources**, one tiny line at the bottom right.

The arrows give the order; the map is a pause in the middle that can be read at any time.

## 4. Format and grid

* Portrait 3:4. Recipe canvas 1080 x 1440. Margins 3.5 to 4.5% W.
* Title block `4.5%,1%,32% x 13%`; intro `41.5%,2.5%,31% x 10.5%`; mark `78%,1.5%,16% x 12%`.
* Rule: 16% to 17.2% H, from 3.8% to 93.5% W.
* Heading 1 at 3.9% W, 19.7% H.
* Arrow band, top: 21.5% to 30.5% H. Arrow 1 from 3.4% to a point at 43.5% W; arrow 2 from
  a notch at 45% to a point at 79% W.
* Down arrow: 78% to 94.5% W, from 21.5% H to a point at 80.5% H.
* Arrow band, bottom: 84.5% to 95.5% H. Arrow 4 from 94.5% W leftward to a point at 51.5%
  W; arrow 5 from a notch at 49.6% to a point at 19% W.
* "Finally" block `4.4%,82%,14% x 13%`.
* Map module: heading at 3.9% W, 36.8% H; legend keys at 4.7% and 26% W, 42% H; hero
  number centered at 50% W, 44% H; map `4%,46%,64% x 34%` with the outside names running
  on to 80% W. Recipe box `4%,46.5%,72% x 31%`, the right 10% W of it for the column that
  lists the small north-eastern regions.
* Sources right-aligned at 96.3% W, 98% H.

## 5. Layer stack

1. Ground: white.
2. Five arrows in pale warm gray `#e4e1e0`: flat bands with pointed heads and notched
   tails, white gaps of 1% W between them. In the original the top right and bottom right
   corners are rounded elbows, so arrows 2, 3, and 4 are one continuous bend.
3. Rule: a navy strip with small white stars between two red hairlines.
4. Title: a small black word, then one word in red `#cf1c21` and one in blue `#206fb6`,
   both in a bold slab at 84 px.
5. Intro text and all running text in dark gray `#3e3b3b`.
6. Map: the real outlines of the regions in blue, red, or gray `#b5b3b5` (white borders);
   each with its number (white on color, dark on gray) and its name (the two-letter code in
   the recipe); small north-eastern regions are named at the right on leader lines.
7. Hero number in black slab with a small "Total" above it; two legend keys (12 px
   squares) with two-line labels and thin curved pointer arrows to sample regions.
8. Person pictograms: simple standing figures (round head, block body), 11 to 16 px wide,
   in dark gray, mid gray `#9a9899`, red, and blue.
9. Small illustrations: a round red-and-blue campaign button on arrow 2's top edge; a
   cut-out photo of a hand with a red pen entering from the right edge beside the down
   arrow; two empty check boxes with head-and-shoulder icons under them.
10. Pie: red and blue halves with white percentages; "Winner" beside the larger half.
11. Majority marker: a small black triangle and the number under the gap between the two
    rows of figures; thin red and blue arrows point at it from both sides.
12. Sources line.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#ffffff` | ground, region borders, numbers on color | 62% | nothing |
| `#e4e1e0` | the arrow conveyor | 17% | nothing: reading order |
| `#cf1c21` | title word, regions, figures, pie half | 7% | Party B (leans B; votes for B) |
| `#206fb6` | title word, regions, figures, pie half | 5% | Party A (leans A; votes for A) |
| `#b5b3b5` | open regions (`#c1bfc1` in the original); "by population" figures `#9a9899` | 6% | neither side; the variable part of a sum |
| `#3e3b3b` | text, hero number, "fixed" figures | 3% | nothing |

Categorical, two hues plus gray (two-party red and blue convention, US). The same red and
blue mean the same two sides in the map, the figures, and the pie. Gray is doing two jobs:
"neither" on the map and "the part that varies" in the sum; the pale arrow gray is lighter
than both so the conveyor never reads as data.

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Title, small word | 1 word ("The") | title | slab (Bitter, Roboto Slab / Rockwell) | 700 | 30 | 0 | - | left | `#000000` | 13%, cap top 1% |
| Title, two words | 1 word per line | title | slab | 700 | 84 | -10 | line pitch 74 px | left | `#cf1c21`, then `#206fb6` | 4.7%, cap tops 3.1% and 8.3% |
| Intro | 40 to 50 words on 7 lines of about 38 characters | sentence | neo-grotesque (Inter, Roboto / Helvetica Neue) | 400 | 18 | 0 | 21 px | left | `#3e3b3b` | 41.5%, 2.5% |
| Module heading (x2) | 3 words | sentence | slab | 700 | 22 | 0 | - | left | `#3e3b3b` | 3.9%, 19.7% and 36.8% |
| Arrow text (x6) | 15 to 40 words, lines of 17 to 36 characters | sentence | neo-grotesque | 400, key words 700 | 15 to 17 | 0 | 19 to 21 px | left; centered in the down arrow | `#3e3b3b` | inside each arrow |
| Sum labels | a bold number and 2 to 5 words | sentence | neo-grotesque | 700 / 400 | 15 | 0 | 18 px | left | `#3e3b3b` | right of each group of figures |
| Sum result | 1 to 2 digits; 4 words on 3 lines (the digit alone in the recipe) | - | neo-grotesque | 700 / 400 | 34 / 15 | 0 | 18 px | left | `#3e3b3b` | in arrow 1's head |
| Hero number | 3 digits | - | slab | 700 | 64 | 0 | - | center | `#000000` | 50%, 44% |
| Hero label | "Total" | title | neo-grotesque | 400 | 16 | 0 | - | left | `#3e3b3b` | 45%, 41% |
| Legend label (x2; x3 in the recipe, which also keys the gray) | 2 to 3 words on 2 lines | title | neo-grotesque | 400 | 15 | 0 | 18 px | left | `#3e3b3b` | right of each key |
| Region number (x51) | 1 to 2 digits | - | neo-grotesque | 700 | 17 (13 in the recipe) | 0 | - | center | `#ffffff` on color, `#3e3b3b` on gray | in each region |
| Region name (x51) | full name (codes in the recipe) | title | neo-grotesque | 400 (700 for the codes) | 15 (14 in the recipe) | 0 | - | center | as the number | above or under the number; outside on leaders for small regions |
| Small heads (the name of the rule at the corner, "Finally") | 1 to 3 words | sentence | neo-grotesque | 700 | 17 | 0 | - | center or left | `#3e3b3b` | at their stops |
| Pie labels | NN% x2; a 2-word caption in italics above | - | neo-grotesque | 700 | 17 | 0 | - | center | `#ffffff` | in the halves |
| Sources | "Sources:" and a list, one line | sentence | neo-grotesque | 400 | 12 | 0 | - | right | `#3e3b3b` | 96.3%, 98% |

Two families: a bold slab for the title, the two headings, and the hero number, and a plain
grotesque for everything else. Sizes 84 : 64 : 22 : 17 : 12. Apart from the title and the
total, nothing is larger than 22 px: the page relies on pictures and position, not on type
contrast.

## 8. Charts and diagrams

* **Arrow conveyor:** five `shape path` bands (a rectangle with a pointed end; arrows 2 and
  5 also have a notch cut into the tail). Point depth 40 to 60 px. No script: these are
  layout furniture, not data.
* **C-us-choropleth** in its class form with numbers: one run of `scripts/usmap.js` (run
  with `render d3`) with `groups=` (every region's class: 1 leans A, 2 leans B, 3 open),
  `colors='#206fb6,#cf1c21,#b5b3b5'`, `values=` (every region's number), `labels=both`
  (code and number on each state; white on blue and red, dark on gray), `legend=none`,
  `stroke='#ffffff' strokewidth=1.5`, `fs=19 weight=700`, box `4,46.5,72,31`. The nine
  small north-eastern regions are listed with a color key, code, and number in a column
  at the right, joined to their places by thin lines. A state too small for its two lines
  gets no label from the script: here that is one island state, labelled with one
  `text add` under its logged anchor.
* **C-icon-array** for every group of figures: `scripts/waffle.js` with `shape=person`.
  The sum: `value=1 total=1` in dark gray and `value=0 total=6 cols=3` in mid gray. "Your
  delegates": `values=8 cols=2` in red and in blue. "Not split": `value=10 total=10 cols=4`
  in blue beside `values=5,5 cols=4` in red and blue. The count: `values=12 cols=12` in red
  and in blue on either side of the marker.
* **C-pie:** `scripts/donut.js` with `inner=0`, two values starting at 12 o'clock (the
  larger first, in blue, so it takes the right half), `labelpos=in show=pct`, radius 4.45%
  W. A pie is acceptable here: two slices, and the point is "barely more than half".
* **C-stat-block** for the total above the map.

* Simplified: two-letter codes stand for the full names; the south-eastern regions keep
  their labels inside instead of on leaders; the legend has no curved pointers and gains a
  key for the gray class.

**Map spec:**

* Region set: 50 US states plus the capital district; the two non-contiguous states as
  insets at the lower left.
* Projection: Albers USA; map box `4%,44%,67% x 36%` in the original, `4%,46.5%,72% x 31%`
  in the recipe.
* Encoding: fill by class, 3 classes: `#206fb6` (leans A), `#cf1c21` (leans B), `#c1bfc1`
  in the original and `#b5b3b5` in the recipe (neither). Borders white, 1.5 px. The number
  is written, not encoded: no ramp, no bubbles.
* Labels: every region carries its number in bold (17 px; white on blue and red, dark on
  gray) and its full name in regular 15 px (inside large regions; just outside small ones).
  Nine small north-eastern regions and five south-eastern ones are named outside the map
  with thin dark leader lines (0.75 px, gently curved), the number first, then the name, in
  a column at the right edge of the map. The recipe writes code over number at 14 and 13
  px and lists only the nine north-eastern regions outside.
* Legend: two keys above the map's left part, each a 12 px square and a two-line label,
  with a thin curved arrow from the label to one region of that class. No key for gray in
  the original; the recipe adds one.
* Callout: the national total as a 64 px slab number above the map's center with "Total"
  over it.
* Fallback for builds before 0.6.0: C-tile-map, `scripts/tilemap.js` (run with `render js`)
  with the same `groups=`, `colors=`, and `values=`, and `showvalues=1`.

## 9. Icons and illustration

* Campaign button: an 84 px red disc with a white ring and a white `vote` icon (the
  original shows a glossy two-color button with party mascots; do not copy emblems).
* Hand marking a ballot: a cut-out photo of a right hand holding a red pencil, entering
  from the right edge at 36% to 47% H, about 20% W wide; the recipe uses the `pencil` icon,
  84 px, red. Place `subject.png` there for a real hand.
* `check` and `x`, 24 px, dark, above the two "not split" blocks.
* Majority marker: a 9 px `shape polygon` triangle with the number under it.
* Rule stars: five-point `shape star` marks, outer radius 6 px, white on the navy strip.
* Publisher's mark: the original is a blue hand print with a red label; the recipe draws a
  red label with "YOUR MARK".

## 10. Effects and finish

None: flat color, no shadows or gradients (the photo of the hand and the glossy button are
the only non-flat things in the original). White gaps between arrows and white borders
between regions keep everything crisp.

## 11. Hierarchy and eye path

1. The two-color title.
2. The first arrow, then along the conveyor: right, right, down, left, left.
3. The red and blue map inside the loop, and the hero number above it.
4. The pictogram groups at each stop (color pulls the eye to the red and blue ones).
5. "Finally" at the end of the last arrow; the sources line.

Title 84 px, total 64, headings 22, text 15 to 18.

## 12. Build recipe

Nothing to supply. Canvas 1080 x 1440. All numbers are example data.

```bash
pixelox new -o loop.pxo --size 1080x1440 --fill '#ffffff'
pixelox shape path loop.pxo --in-place --name Arrow1 --d 'M 37 309 L 430 309 L 470 374 L 430 439 L 37 439 Z' --fill '#e4e1e0'
pixelox shape path loop.pxo --in-place --name Arrow2 --d 'M 446 309 L 812 309 L 852 374 L 812 439 L 446 439 L 486 374 Z' --fill '#e4e1e0'
pixelox shape path loop.pxo --in-place --name Arrow3 --d 'M 842 309 L 1020 309 L 1020 1100 L 931 1160 L 842 1100 Z' --fill '#e4e1e0'
pixelox shape path loop.pxo --in-place --name Arrow4 --d 'M 1020 1217 L 600 1217 L 556 1296 L 600 1375 L 1020 1375 Z' --fill '#e4e1e0'
pixelox shape path loop.pxo --in-place --name Arrow5 --d 'M 580 1217 L 250 1217 L 206 1296 L 250 1375 L 580 1375 L 536 1296 Z' --fill '#e4e1e0'
pixelox text add loop.pxo --in-place --name TitleThe --content "The" --font @slab --weight 700 --size 30 --color '#000000' --at 142,14 --anchor tl --anchor-to ink
pixelox text add loop.pxo --in-place --name Title1 --content "Delegate" --font @slab --weight 700 --size 84 --tracking=-10 --color '#cf1c21' --at 51,45 --anchor tl --anchor-to ink
pixelox text add loop.pxo --in-place --name Title2 --content "Vote" --font @slab --weight 700 --size 84 --tracking=-10 --color '#206fb6' --at 51,119 --anchor tl --anchor-to ink
pixelox text add loop.pxo --in-place --name Intro --content "Some offices are filled in two steps. Voters in every region pick delegates, and the delegates pick the office-holder. Because each region has a set number of delegates, the map matters more than the national tally of votes." --font @neo-grotesque --size 18 --leading 21 --color '#3e3b3b' --box 350x160 --at 448,35 --anchor tl
pixelox shape rect loop.pxo --in-place --name MarkBox --rect 858,62,160x56 --fill '#cf1c21'
pixelox text add loop.pxo --in-place --name Mark --content "YOUR MARK" --font @heavy-wide --size 20 --color '#ffffff' --at 938,90 --anchor c --anchor-to ink
pixelox shape rect loop.pxo --in-place --name RuleBand --rect 41,233,969x14 --fill '#27408b'
pixelox shape path loop.pxo --in-place --name RuleLines --d 'M 41 229 L 1010 229 M 41 251 L 1010 251' --fill none --stroke '#cf1c21' --stroke-width 1.5
pixelox shape star loop.pxo --in-place --name St1 --center 120,240 --points 5 --outer 6 --fill '#ffffff'
pixelox shape star loop.pxo --in-place --name St2 --center 322,240 --points 5 --outer 6 --fill '#ffffff'
pixelox shape star loop.pxo --in-place --name St3 --center 525,240 --points 5 --outer 6 --fill '#ffffff'
pixelox shape star loop.pxo --in-place --name St4 --center 728,240 --points 5 --outer 6 --fill '#ffffff'
pixelox shape star loop.pxo --in-place --name St5 --center 930,240 --points 5 --outer 6 --fill '#ffffff'
pixelox text add loop.pxo --in-place --name Head1 --content "How it works" --font @slab --weight 700 --size 22 --color '#3e3b3b' --at 42,276 --anchor l
pixelox text add loop.pxo --in-place --name A1Text --content $'Each region has\none fixed delegate\nplus one for each\nof its seats.' --font @neo-grotesque --size 16 --leading 20 --color '#3e3b3b' --at 56,374 --anchor l
pixelox text add loop.pxo --in-place --name A1Ex --content "Example" --font @neo-grotesque --style italic --size 14 --color '#3e3b3b' --at 214,326 --anchor l
pixelox render js scripts/waffle.js -o fixed.png --size 1080x1440 --param box=19.8,23.5,1.3,1.9 --param value=1 --param total=1 --param cols=1 --param shape=person --param colors='#3e3b3b' --param legend=none
pixelox comp place loop.pxo fixed.png --in-place --as SumFixed --at 0,0
pixelox render js scripts/waffle.js -o bypop.png --size 1080x1440 --param box=19.8,26.8,4,3.9 --param value=0 --param total=6 --param cols=3 --param shape=person --param empty='#9a9899' --param legend=none
pixelox comp place loop.pxo bypop.png --in-place --as SumByPop --at 0,0
pixelox text add loop.pxo --in-place --name SumL1 --content "1 fixed (every region)" --font @neo-grotesque --size 14 --color '#3e3b3b' --at 252,352 --anchor l
pixelox text add loop.pxo --in-place --name SumPlus --content "+" --font @neo-grotesque --weight 700 --size 20 --color '#3e3b3b' --at 228,376 --anchor c
pixelox text add loop.pxo --in-place --name SumL2 --content $'6 seats by\npopulation' --font @neo-grotesque --size 14 --leading 17 --color '#3e3b3b' --at 284,414 --anchor l
pixelox text add loop.pxo --in-place --name SumEq --content "= 7" --font @neo-grotesque --weight 700 --size 30 --color '#3e3b3b' --at 368,374 --anchor l
pixelox text add loop.pxo --in-place --name A2Text --content $'Every five years each region holds\na vote. On the ballot people choose\na pair: a candidate and a deputy.' --font @neo-grotesque --size 16 --leading 20 --color '#3e3b3b' --at 506,384 --anchor l
pixelox shape ellipse loop.pxo --in-place --name Button --rect 688,268,84x84 --fill '#cf1c21' --stroke '#ffffff' --stroke-width 5
pixelox shape icon loop.pxo vote --in-place --name ButtonIcon --size 46 --at 730,310 --anchor c --color '#ffffff' --stroke-width 4
pixelox text add loop.pxo --in-place --name A3Text --content $'In fact the vote\npicks the region’s\ndelegates, who\nlater cast their\nvotes for the\nwinning pair.' --font @neo-grotesque --size 16 --leading 20 --color '#3e3b3b' --at 864,470 --anchor tl
pixelox shape icon loop.pxo pencil --in-place --name Pen --size 84 --at 958,664 --anchor c --color '#cf1c21' --stroke-width 6
pixelox text add loop.pxo --in-place --name BallotA --content "Party B" --font @neo-grotesque --size 14 --color '#3e3b3b' --at 895,752 --anchor c
pixelox text add loop.pxo --in-place --name BallotB --content "Party A" --font @neo-grotesque --size 14 --color '#3e3b3b' --at 968,752 --anchor c
pixelox render js scripts/waffle.js -o delB.png --size 1080x1440 --param box=81.2,53.5,3.3,10 --param values=8 --param cols=2 --param shape=person --param colors='#cf1c21' --param legend=none
pixelox comp place loop.pxo delB.png --in-place --as DelegatesB --at 0,0
pixelox render js scripts/waffle.js -o delA.png --size 1080x1440 --param box=88,53.5,3.3,10 --param values=8 --param cols=2 --param shape=person --param colors='#206fb6' --param legend=none
pixelox comp place loop.pxo delA.png --in-place --as DelegatesA --at 0,0
pixelox text add loop.pxo --in-place --name A3Head --content $'What the ballot\ndecides' --font @neo-grotesque --weight 700 --size 17 --leading 21 --align center --color '#3e3b3b' --at 931,972 --anchor c
pixelox text add loop.pxo --in-place --name A3Note --content $'the delegates of\nyour region' --font @neo-grotesque --size 15 --leading 19 --align center --color '#3e3b3b' --at 931,1038 --anchor c
pixelox text add loop.pxo --in-place --name WinHead --content "All or nothing" --font @neo-grotesque --weight 700 --size 17 --color '#3e3b3b' --at 931,1190 --anchor c
pixelox text add loop.pxo --in-place --name WinSub --content "Regional result" --font @neo-grotesque --style italic --size 15 --color '#3e3b3b' --at 935,1240 --anchor c
pixelox render js scripts/donut.js -o pie.png --size 1080x1440 --param cx=86.6 --param cy=91.2 --param r=4.45 --param inner=0 --param values=52,48 --param colors='#206fb6,#cf1c21' --param labelpos=in --param show=pct --param sep='#ffffff' --param fs=17
pixelox comp place loop.pxo pie.png --in-place --as Pie --at 0,0
pixelox text add loop.pxo --in-place --name WinLabel --content "Winner" --font @neo-grotesque --size 14 --color '#3e3b3b' --at 990,1290 --anchor l
pixelox text add loop.pxo --in-place --name A4Head --content "Delegates are never shared." --font @neo-grotesque --size 16 --color '#3e3b3b' --at 668,1196 --anchor c
pixelox shape icon loop.pxo check --in-place --name Yes --size 24 --at 638,1240 --anchor c --color '#3e3b3b' --stroke-width 4
pixelox shape icon loop.pxo x --in-place --name No --size 24 --at 704,1240 --anchor c --color '#3e3b3b' --stroke-width 4
pixelox render js scripts/waffle.js -o whole.png --size 1080x1440 --param box=56.7,87.4,4.8,7.4 --param value=10 --param total=10 --param cols=4 --param shape=person --param colors='#206fb6' --param legend=none
pixelox comp place loop.pxo whole.png --in-place --as NotSplitYes --at 0,0
pixelox render js scripts/waffle.js -o split.png --size 1080x1440 --param box=62.8,87.4,4.8,7.4 --param values=5,5 --param cols=4 --param shape=person --param colors='#cf1c21,#206fb6' --param legend=none
pixelox comp place loop.pxo split.png --in-place --as NotSplitNo --at 0,0
pixelox text add loop.pxo --in-place --name A4Text --content $'Just over half of\nthe votes wins all\nof a region’s\ndelegates.' --font @neo-grotesque --size 15 --leading 19 --color '#3e3b3b' --at 748,1300 --anchor l
pixelox text add loop.pxo --in-place --name A5Text --content $'Then every delegate is counted. More\nthan half of the 441 wins: 221.' --font @neo-grotesque --size 15 --leading 19 --color '#3e3b3b' --at 268,1262 --anchor l
pixelox render js scripts/waffle.js -o rowB.png --size 1080x1440 --param box=24.8,90.4,11.4,2.2 --param values=12 --param cols=12 --param shape=person --param colors='#cf1c21' --param legend=none
pixelox comp place loop.pxo rowB.png --in-place --as CountB --at 0,0
pixelox render js scripts/waffle.js -o rowA.png --size 1080x1440 --param box=39.6,90.4,11.4,2.2 --param values=12 --param cols=12 --param shape=person --param colors='#206fb6' --param legend=none
pixelox comp place loop.pxo rowA.png --in-place --as CountA --at 0,0
pixelox shape polygon loop.pxo --in-place --name Marker --center 409,1342 --radius 9 --sides 3 --fill '#000000'
pixelox text add loop.pxo --in-place --name MarkerN --content "221" --font @neo-grotesque --size 15 --color '#3e3b3b' --at 409,1362 --anchor c
pixelox text add loop.pxo --in-place --name FinalHead --content "Finally" --font @neo-grotesque --weight 700 --size 17 --color '#3e3b3b' --at 47,1188 --anchor l
pixelox text add loop.pxo --in-place --name FinalText --content $'Some weeks later\nthe delegates gather\nin each region and\nhand in their votes.\nThat count is the\nofficial result.' --font @neo-grotesque --size 15 --leading 19 --color '#3e3b3b' --at 47,1206 --anchor tl
pixelox text add loop.pxo --in-place --name Head2 --content "How many delegates" --font @slab --weight 700 --size 22 --color '#3e3b3b' --at 42,530 --anchor l
pixelox shape rect loop.pxo --in-place --name KeyA --rect 51,600,12x12 --fill '#206fb6'
pixelox text add loop.pxo --in-place --name KeyAt --content $'Leans to\nParty A' --font @neo-grotesque --size 15 --leading 18 --color '#3e3b3b' --at 72,598 --anchor tl
pixelox shape rect loop.pxo --in-place --name KeyB --rect 201,600,12x12 --fill '#cf1c21'
pixelox text add loop.pxo --in-place --name KeyBt --content $'Leans to\nParty B' --font @neo-grotesque --size 15 --leading 18 --color '#3e3b3b' --at 222,598 --anchor tl
pixelox shape rect loop.pxo --in-place --name KeyO --rect 351,600,12x12 --fill '#b5b3b5'
pixelox text add loop.pxo --in-place --name KeyOt --content $'Open\nregions' --font @neo-grotesque --size 15 --leading 18 --color '#3e3b3b' --at 372,598 --anchor tl
pixelox text add loop.pxo --in-place --name TotalL --content "Total" --font @neo-grotesque --size 16 --color '#3e3b3b' --at 476,590 --anchor l
pixelox text add loop.pxo --in-place --name Total --content "441" --font @slab --weight 700 --size 64 --color '#000000' --at 540,634 --anchor c --anchor-to ink
pixelox render d3 scripts/usmap.js -o map.png --size 1080x1440 --param box=4,46.5,72,31 --param groups='AK:1,WA:1,OR:1,CA:1,HI:1,ID:1,NV:1,UT:1,AZ:1,MT:1,WY:1,CO:1,NM:1,ND:1,SD:1,NE:1,KS:1,OK:1,PA:2,VA:2,NC:2,SC:2,GA:2,NY:2,NJ:2,MD:2,DC:2,FL:2,RI:2,CT:2,DE:2,VT:2,NH:2,MA:2,ME:2,MN:3,IA:3,MO:3,AR:3,LA:3,IL:3,IN:3,KY:3,TN:3,MS:3,WI:3,OH:3,WV:3,AL:3,TX:3,MI:3' --param values='CA:47,TX:36,FL:27,NY:24,PA:16,IL:16,OH:14,GA:13,NC:13,MI:12,VA:12,AZ:12,NJ:11,WA:11,TN:10,WI:9,CO:9,MA:8,IN:8,AL:8,MO:7,MD:7,MN:7,KY:7,OR:7,SC:6,OK:6,LA:5,UT:5,IA:5,NV:5,MS:5,CT:4,NM:4,NE:4,AR:3,KS:3,ID:3,WV:3,HI:3,NH:3,ME:3,MT:3,RI:3,DE:2,SD:2,ND:2,AK:2,DC:2,VT:2,WY:2' --param colors='#206fb6,#cf1c21,#b5b3b5' --param labels=both --param legend=none --param stroke='#ffffff' --param strokewidth=1.5 --param fs=19 --param weight=700 --param ink='#3e3b3b' --param muted='#3e3b3b'
pixelox comp place loop.pxo map.png --in-place --as Map --at 0,0
pixelox text add loop.pxo --in-place --name LabelHI --content "HI  3" --font @neo-grotesque --weight 700 --size 14 --color '#3e3b3b' --at 252,1110 --anchor c
pixelox text add loop.pxo --in-place --name Sources --content "Sources: Example Elections Office. Example data, invented for this layout." --font @neo-grotesque --size 12 --color '#3e3b3b' --at 1040,1412 --anchor r
pixelox export loop.pxo -o loop.png
```

Reuse: change `groups=` and `values=` (one entry per region in each), the hero total (the
sum of all values), and the majority marker (half the total, rounded up). After a change
look at the map for regions without a label and add them by hand at their logged anchors
(`lab/anchors.py scripts/usmap.js` with the same `--size` and params; the island state is
at 252,1079 here). The map needs pixelox 0.6.0 or later. An arrow is a rectangle with a
point: for a band from x0 to x1 between y0 and y1 pointing right, `M x0,y0 L x1,y0 L
x1+40,yc L x1,y1 L x0,y1 Z`; add `L x0+40,yc` before `Z` for a notched tail. Put at most
one group of figures and 40 words in an arrow.

## 13. Variations and pairings

* **Any "how it works" with one dataset in the middle:** how a budget passes (a C-donut of
  spending inside the loop), how a product ships (a C-tile-map of warehouses), how a
  tournament is seeded (a table).
* **Fewer stops:** four arrows around a square canvas (right, down, left, up) make a
  process loop; use C-cycle when the steps repeat.
* **Straight version:** all arrows in one row is C-chevrons (`scripts/steps.js`,
  `style=chevron`) with the text inside the band.
* **Other palettes:** teal `#0f766e` and amber `#b45309` for two neutral sides, on a
  `#ece9e4` conveyor; keep the conveyor paler than any data color.
* Pairs with C-hemicycle in place of the count row (delegates as seats) and with
  C-percent-bar for the majority line.

## 14. Pitfalls

* **Figures that do not match the sum.** The 1 dark and 6 gray figures must be exactly 1
  and 6, the "not split" blocks exactly 10 and 10, and the map's values must add up to the
  hero total (130 + 156 + 155 = 441 here). Check each script's logged `total`. Pictograms
  that are "about right" teach the wrong number: this is the data-integrity rule for every
  counted-unit explainer.
* **Numbers the map drops or hides.** The script skips the label of a state that is too
  small for it, and white numbers that run past a blue or red state's edge vanish on the
  white ground. Count the labels after every change, add the missing ones by hand at the
  logged anchors, and set any number that hangs over its state's edge again in dark type
  beside the state. Never change a region's class to make its label readable.
* **Arrows darker than the data.** A mid-gray conveyor swallows the gray regions and the
  gray figures; the conveyor must be the palest tint on the page.
* **A pie for more than two slices** stops showing "just over half". For three or more
  sides use C-percent-bar with the majority line marked.
* **Class colors that look like results.** A red and blue map reads as an outcome; say
  "leans" or "open" in the legend, date it, and never color a region for which you have no
  basis.
* **Too many words per arrow.** Past 40 words the type drops under 15 px and the page turns
  into an article with arrows behind it; cut the text or add a stop.
