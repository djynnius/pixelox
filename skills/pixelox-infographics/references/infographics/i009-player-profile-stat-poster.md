---
id: i009
title: Player profile poster with a cutout and stat modules
kind: [statistical, dashboard]
topic: [sports]
mood: [bold, energetic, vibrant]
style: [photo-led, flat]
format: portrait
structure: S-stat-poster
data: [single-stat, comparison, part-to-whole]
charts: [C-grouped-columns, C-bars, C-progress-ring, C-stat-block, C-ribbon-label, C-table, C-title-block]
palette: ['#e6e5ea', '#b1073b', '#77a5ce', '#892a34', '#ef306c', '#484755', '#2774cc']
needs: subject-cutout
---

# i009: Player profile poster with a cutout and stat modules

## 1. Concept

A 4:5 fan poster that turns one athlete's season into a wall of small scoreboards. A
full-height cutout of the player stands in the left third of a concrete-gray page, and eight
unrelated stat modules in crimson and steel blue fill the rest: counts in boxes, dates in
boxes, counts in brush rings, a rating chart, a bar list, four progress rings, a small table,
a two-triangle comparison. The reader takes away "this player delivers in every column". It
works because only two colors carry all the data, every module has its own slanted crimson
name tag, and the photo is the only thing on the page that is not a number.

## 2. Data story

Eleven small data sets about one person, none longer than six values:

* **3 counts from one match** (single digits to low tens), each with a one-word label.
* **2 dates** (a month and a day each): two career firsts.
* **3 season totals** by competition (10 to 15 each, close together).
* **10 skill ratings on a 1 to 5 scale**, drawn as 5 pairs: the first of each pair high (4.5
  to 5 and above), the second low (about 2). Each of the ten bars is a different skill.
* **6 totals** for three competitions in two seasons (10 to 15), in season order, not sorted.
* **3 percentages** in a three-column mini table (roughly two thirds to four fifths).
* **1 two-way split** of a count (home against away, the first clearly larger).
* **4 independent percentages** (a fifth to three fifths) as progress rings.

Relationships: single stats, small comparisons, shares of 100.

Placeholder data in the recipe (example data, invented, a basketball guard): last game 24
points, 7 blocks, 3 steals; debuts April 9 and August 21; season points 31, 36, 24 by
competition; skills 4.6, 3.9, 5, 4.1, 4.7 and 1.8, 1.5, 2.1, 1.6, 1.9; career totals 18, 22,
25, 20, 16, 23; mini table 78%, 61%, 72%; home and away wins 19 and 11; rings 24%, 55%,
62%, 41%.

## 3. Information architecture

1. **Title** between two full-width hairlines: the name of the series or the season.
2. **Photo**: the player, full height, left third. The eye starts here.
3. **Last game** (three boxed counts in a crimson frame) and **two dates** (label, month
   strip, day box), across the top right. Read left to right from the player's head.
4. **Season totals**: three numbers in brush rings under a tag, beside the photo.
5. **Skill ratings**: a paired column chart, right.
6. **Career totals**: a list of six bars with names inside and values in small rings.
7. **Four rings**, 2 x 2, bottom right: shares.
8. **Under the photo**: the player's name tag, a three-column mini table, and the home and
   away triangles.

There is no single path after the photo and the title: each module is a self-contained
card without a card, named by its tag. Modules of one family (boxes, rings, bars) share a
shape, which is what keeps eight of them from looking like eight designs.

## 4. Format and grid

* Portrait 4:5. Recipe canvas 1080 x 1350. Margins 2.2% W left, 2% W right.
* Title zone 0 to 12% H: hairlines at 3.2% and 11.4% H from 4.3% to 95.6% W; title caps
  between them, 93.5% W wide.
* Three loose columns below: photo 1.3% to 31.7% W; middle 32% to 62% W; right 61% to 98% W.
* Photo cutout box `1.3%,17.5%,30.4% x 58.2%` (feet at 75.7% H).
* Last game: tab `35.7%,17.3%,24% x 2.4%`; frame `19.9%,19.6%,40.6% x 9.6%` holding three
  boxes 10.6% W x 8% H at x = 22%, 35.1%, 48.1% W. The player's head overlaps the frame's
  left end.
* Dates: two columns centered on 71.9% and 90.4% W: label at 17.9% H, month strip
  `x,19.4%,14.8% x 3.2%`, day box `x,22.7%,12.6% x 5.3%`.
* Season tag `34.3%,32.6%,26.8% x 2.7%`; ring centers (40.4%, 39.9%), (53.7%, 39.9%),
  (46.8%, 54.4%), diameter 8.5% W; labels 5% H under each center.
* Skill chart `60.5%,35.2%,37.5% x 22.6%`, a second label row down to 61% H.
* Career tag `31.7%,63.3%,30.1% x 3.7%`; bar list `32.6%,69.2%,36% x 26.5%`.
* Rings: centers at 75.1% and 92.3% W, 72.1% and 88% H; outer radius 6% W; captions 5.8% H
  under each center.
* Name tag `3.4%,75.9%,23.2% x 2.3%`; mini table `2.2%,79.3%,26.9% x 4.5%`; home and away
  label between rules at 86.1% and 88.4% H; triangles `4.4%,89%,20% x 9.6%`.

## 5. Layer stack

1. Ground `#e6e5ea` with a faint concrete texture (specks and scratches, 3 to 4% darker).
2. Two black hairlines; the title in dark red `#892a34` with a pale gray shadow 3 px down
   and right (reads as embossed).
3. Last-game frame, crimson `#b1073b`; three steel-blue boxes `#7aa7c8` inside it; a
   lighter blue tab `#80adc4` with a slanted left end on top of the frame.
4. The cutout photo, over the frame's left end, under everything that follows.
5. Date modules: black label, crimson strip `#b30832` with the month in black, blue box
   `#76a3cc` with the day in white.
6. Season tag: a crimson parallelogram `#ba1736` (both ends lean 20 degrees), black wide
   type. Three dry-brush rings in red `#b3252f`, a condensed number in each, a two-line slab
   label under each.
7. Skill chart: a 6 px blue axis `#2774cc` in an L, navy numbers 1 to 5 up the left; five
   tall red bars `#df1849`, each with a short charcoal bar `#484755` overlapping its lower
   right; a row of red labels and a row of charcoal labels under the axis.
8. Career tag: a larger crimson parallelogram, black condensed type. Six bars alternating
   steel blue `#77a5ce` and hot pink-red `#ef306c`, the row's name inside each bar in black,
   the value just past the bar end in a small navy brush ring `#24519f`.
9. Four rings: track `#6f9cd1`, value arc `#4c7cbc` clockwise from the top, a white disc in
   the middle with the percentage in light blue `#6b99d4`; a two-line black caption under each.
10. Name tag (crimson parallelogram, black wide type); mini table of three labels and three
    percentages between hairlines; a centered label between two more hairlines.
11. Two triangles: crimson `#a80843` (larger, behind) and steel blue `#6c9ac5` (smaller,
    overlapping its right foot), each with a darker right facet and a white number.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#e6e5ea` | ground | 65% | nothing |
| `#b1073b` | frame, strips, tags, large triangle (`#a80843`) | 9% | structure: "a module starts here"; home in the split |
| `#77a5ce` | boxes, tab, every second career bar, rings (`#6f9cd1`, `#4c7cbc`), small triangle | 11% | the holder of a number; season A in the bar list; away in the split |
| `#892a34` | title; brush rings and their labels (`#b3252f`) | 5% | nothing: display type |
| `#ef306c` | every second career bar; tall skill bars (`#df1849`) | 5% | season B in the bar list; primary skills |
| `#484755` | short skill bars and their labels | 2% | secondary skills |
| `#2774cc` | chart axis; tick numbers (`#071e6f`); small rings (`#24519f`) | 1% | nothing: chart furniture |

Two-color scheme (crimson and steel blue, the colors of the player's kit) on a cool gray,
with black type on light and on crimson, white type on blue. The hot pink-red of the bars
is a brighter step of the crimson, used only for data marks.

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Title | 3 words, 15 letters | caps | fat slab (Alfa Slab One, Chunk Five / Superclarendon) | 900 | about 94, fitted to 93.5% W | 0 | - | center | `#892a34` | 50%, cap top 4.9% |
| Tab label | 2 words | title | humanist sans (Ubuntu, Open Sans / Helvetica Neue) | 500 | 29 | 0 | - | center | `#ffffff` | 48%, 18.4% |
| Box number (x3) | 1 or 2 digits | - | humanist sans | 400 | 62 | 0 | - | center | `#ffffff` | box centers, 23.2% |
| Box label (x3) | 1 word | title | humanist sans | 400 | 30 | 0 | - | center | `#ffffff` | box centers, 27.1% (on the box's lower edge) |
| Date label (x2) | 2 words | title | humanist sans | 400 | 29 | 0 | - | center | `#1a1718` | 71.9% and 90.4%, 17.9% |
| Month (x2) | month name | title | humanist sans | 400 | 36 | 0 | - | center | `#1a1718` | on the strip, 21% |
| Day (x2) | 1 or 2 digits | - | humanist sans | 400 | 53 | 0 | - | center | `#ffffff` | in the box, 25.4% |
| Tag text (x2 small) | 2 to 4 words | sentence | wide sans (Michroma, Krona One / Helvetica Neue stretched 135%) | 500 | 23 and 19 | 0 | - | center | `#1a1718` | tag centers |
| Ring number (x3) | 2 digits | - | condensed heavy (Anton, Bebas Neue / Impact) | 400 | 60 | 0 | - | center | `#b3252f` | ring centers |
| Ring label (x3) | 2 or 3 words on 1 or 2 lines | caps | fat slab | 900 | 27 | 0 | 36 px | center | `#b3252f` | 5 to 6.7% H under the ring center |
| Axis numbers | 1 to 5 | - | condensed heavy | 400 | 22 | 0 | - | right | `#071e6f` | left of the axis |
| Skill labels, row 1 (x5) | 1 or 2 short words, 2 lines | caps | fat slab | 900 | 17 to 19 | 0 | 22 px | center | `#df1849` | under each pair, 55.5% |
| Skill labels, row 2 (x5) | 1 or 2 short words, 2 lines | caps | fat slab | 900 | 17 to 19 | 0 | 22 px | center | `#484755` | under row 1, 60.1% |
| Career tag | 4 words | sentence | condensed (Ubuntu Condensed, Barlow Condensed / Avenir Next Condensed) | 500 | 39 | 0 | - | center | `#1a1718` | 46.8%, 65.1% |
| Bar name (x6) | 2 or 3 words and a season | title | condensed | 400 | 25 | 0 | bar pitch 60 px | left | `#1a1718` | 10 px inside each bar's left end |
| Bar value (x6) | 2 digits | - | condensed heavy | 400 | 26 | 0 | - | left | `#1a1718` | 23 px past each bar end |
| Ring percentage (x4) | NN% | - | humanist sans | 400 | 35 | 0 | - | center | `#6b99d4` | ring centers |
| Ring caption (x4) | 3 or 4 words on 2 lines | sentence | humanist sans | 400 | 24 | 0 | 125% | left | `#1a1718` | from 4 px left of the ring's inner edge, 5.8% H under the center |
| Name | 2 words | title | wide sans | 500 | 19 | 0 | - | center | `#1a1718` | 15.1%, 77% |
| Mini table labels (x3) | 1 or 2 words | title | condensed | 400 | 21 | 0 | - | center | `#1a1718` | 7.8%, 18.2%, 26.2% W; 80.1% H |
| Mini table values (x3) | NN% | - | condensed | 400 | 32 | 0 | - | center | `#1a1718` | same columns, 82.4% H |
| Split label | 5 words | sentence | condensed | 400 | 21 | 0 | - | center | `#1a1718` | 15.6%, 87.3% |
| Triangle numbers | 2 digits | - | humanist sans | 400 | 50 and 39 | 0 | - | center | `#ffffff` | low in each triangle |

Four families, which is two more than the skill recommends: a fat slab for the title and
the caps labels, a humanist sans for numbers in boxes, a condensed sans for lists, and a
condensed heavy for numbers in rings. It holds together only because each family has one
job. Sizes 94 : 60 : 36 : 25 : 19.

## 8. Charts and diagrams

* **C-grouped-columns** (skill ratings): `scripts/bars.js`, box `62.4,35.2,35.6,22.6` (the
  plot; the tick numbers stand left of it), two series of five values,
  `colors='#df1849,#484755'`, `max=5`, `axis=0`, `vlabels=0`, `gridlines=0`, `legend=none`,
  `radius=0`, `gap=0.3`. The script's category labels (drawn in `muted`, set to the red) are
  the first label row; the second row is five `text add` layers in charcoal centered under
  each pair. A 6 px blue `shape path` in an L is drawn over the plot's left and bottom edges
  (the logged `plot` rectangle), and the navy tick numbers 5 to 0 are one right-aligned
  `text add` layer whose leading is the plot height divided by 5 (253 / 5 = 50.6 px).
  * **The original is pseudo-3D.** Each bar is a flat front face whose top edge is sheared:
    the left corner sits 0.35 to 0.45 scale units higher than the right, as if a square
    prism were seen from the lower right at about 30 degrees elevation, rotated 20 degrees.
    There is no side face, no top face, no light, no shadow: one flat color per bar
    (`#df1849` tall, `#484755` short). The short bar stands in front, shifted right by 70%
    of a bar width, hiding the tall bar's lower right. Depth encodes nothing. The sloped top
    makes every value ambiguous by a third of a unit, two bars rise above the top tick, and
    the axis starts at 1, not 0. The recipe draws flat tops from zero.
  * A three.js build would use 10 box meshes (width 1, depth 1, heights = values) in 5
    pairs, camera elevation 30 degrees and azimuth 20 degrees, flat unlit materials.
* **C-bars** (career totals): `scripts/bars.js`, `dir=h`, box `32.6,69.2,36,26.5`, six values
  in season order, colors alternating blue and pink-red (one per bar), `max=25`, `radius=0`,
  `labelwidth=0` (no names outside), values at the bar ends in the condensed heavy face. The
  six names are one left-aligned `text add` layer with `--leading 60` (the bar pitch) placed
  10 px inside the bars' left ends. Bars start at zero and their lengths match the values.
  The script's `labelpos=inside` (C-label-bars) is not used: it measures a name as a wide
  grotesque, so two of these condensed names would be moved behind their bars, and it
  writes names in white, where the original's are black.
* **C-progress-ring** x4: `scripts/donut.js`, `kind=ring`, `cx/cy`, `r=6`, `inner=0.65`,
  `round=0`, value arc `#4c7cbc` on track `#6f9cd1`, number at 35 px, regular weight. A
  white disc (radius 42 px) lies under each ring's hole.
* **C-stat-block** variants: boxed counts, date boxes, brush-ring counts.
* **C-ribbon-label**: the three slanted tags are parallelograms (`shape path`), not folded
  ribbons. **C-table**: the three-column mini table. **C-title-block**: centered, between rules.
* The triangles are two `shape path` layers. Their areas are in the ratio of the two values
  (1.7 to 1); see Pitfalls.

## 9. Icons and illustration

* **Photo:** one athlete cut out of an action shot, whole body, mid-stride with the ball at
  the feet, turned three-quarters toward the page center, in a striped kit whose two colors
  are the poster's two colors. Height 58% H, width about 30% W, no shadow, no outline. The
  head overlaps the first module's frame, which ties photo and data together. Supply
  `subject.png` ("your image here"): a clean cutout with transparency.
* **Brush rings:** rough hand-painted circles with broken, dry edges, 8.5% W across, line
  about 0.8% W. The recipe paints them with `paint stroke` along circle paths with size
  jitter and a taper. The six small value rings in the bar list are the same stroke in navy,
  taller than wide (40 x 50 px).
* No icons.

## 10. Effects and finish

* Concrete texture on the ground: `filter grain --as-layer --amount 14`. The original has
  larger soft specks; a grunge photo placed over the ground at 10% opacity with
  `layer blend --mode multiply` gets closer.
* Title emboss: `fx drop-shadow --offset 3,4 --blur 2 --color '#aaa5ae' --opacity 90`.
* Everything else is flat: no gradients, no rounded corners, no shadows under the cutout.

## 11. Hierarchy and eye path

1. The player (the only photograph, the tallest shape).
2. The title (the widest and heaviest type).
3. The crimson frame with its three white numbers, then the two date boxes to its right.
4. The red ring numbers and the red bars of the chart (the most saturated marks).
5. The bar list and the four rings, bottom right.
6. The small print under the photo and the triangles.

Title 94 px, ring and box numbers 53 to 62, tag and month type 36 to 39, list type 24 to 26,
chart labels 17 to 19.

## 12. Build recipe

Supply `subject.png`: your cutout of the athlete (your image here). The stand-in is a bust,
so the recipe places it 500 px wide with its base at 75.9% H; place a full-figure cutout
with `--size x786 --at 178,1022 --anchor b` instead. Canvas 1080 x 1350. All figures are
example data, invented.

```bash
pixelox new -o profile.pxo --size 1080x1350 --fill '#e6e5ea'
pixelox shape path profile.pxo --in-place --name Rules --d 'M 46 43 L 1032 43 M 46 154 L 1032 154' --fill none --stroke '#1a1718' --stroke-width 3
pixelox text add profile.pxo --in-place --name Title --content "SEASON IN NUMBERS" --font @fat-display --weight 900 --faux-bold 40 --fit-width 1010 --color '#892a34' --at 50%,66 --anchor t --anchor-to ink
pixelox fx drop-shadow profile.pxo --in-place --layer Title --offset 3,4 --blur 2 --color '#aaa5ae' --opacity 90
pixelox shape path profile.pxo --in-place --name Tab --d 'M 400 233 L 645 233 L 645 265 L 386 265 Z' --fill '#80adc4'
pixelox shape rect profile.pxo --in-place --name Frame --rect 215,265,438x130 --fill '#b1073b'
pixelox shape path profile.pxo --in-place --name Boxes --d 'M 238 276 h 115 v 108 h -115 Z M 379 276 h 115 v 108 h -115 Z M 520 276 h 115 v 108 h -115 Z' --fill '#7aa7c8'
pixelox text add profile.pxo --in-place --name TabT --content "Last Game" --font @neo-grotesque --weight 500 --size 29 --color '#ffffff' --at 518,249 --anchor c
pixelox text add profile.pxo --in-place --name B1n --content "24" --font @neo-grotesque --size 62 --color '#ffffff' --at 295,313 --anchor c --anchor-to ink
pixelox text add profile.pxo --in-place --name B2n --content "7" --font @neo-grotesque --size 62 --color '#ffffff' --at 436,313 --anchor c --anchor-to ink
pixelox text add profile.pxo --in-place --name B3n --content "3" --font @neo-grotesque --size 62 --color '#ffffff' --at 577,313 --anchor c --anchor-to ink
pixelox text add profile.pxo --in-place --name B1l --content "Points" --font @neo-grotesque --size 30 --color '#ffffff' --at 295,366 --anchor c
pixelox text add profile.pxo --in-place --name B2l --content "Blocks" --font @neo-grotesque --size 30 --color '#ffffff' --at 436,366 --anchor c
pixelox text add profile.pxo --in-place --name B3l --content "Steals" --font @neo-grotesque --size 30 --color '#ffffff' --at 577,366 --anchor c
pixelox comp place profile.pxo subject.png --in-place --as Player --size 500x --at 150,1024 --anchor b
pixelox text add profile.pxo --in-place --name D1h --content "League Debut" --font @neo-grotesque --size 29 --color '#1a1718' --at 777,242 --anchor c
pixelox text add profile.pxo --in-place --name D2h --content "Cup Debut" --font @neo-grotesque --size 29 --color '#1a1718' --at 976,242 --anchor c
pixelox shape path profile.pxo --in-place --name Strips --d 'M 697 262 h 160 v 43 h -160 Z M 896 262 h 160 v 43 h -160 Z' --fill '#b30832'
pixelox shape path profile.pxo --in-place --name DateBoxes --d 'M 709 307 h 136 v 71 h -136 Z M 908 307 h 136 v 71 h -136 Z' --fill '#76a3cc'
pixelox text add profile.pxo --in-place --name D1m --content "April" --font @neo-grotesque --size 36 --color '#1a1718' --at 777,283 --anchor c
pixelox text add profile.pxo --in-place --name D2m --content "August" --font @neo-grotesque --size 36 --color '#1a1718' --at 976,283 --anchor c
pixelox text add profile.pxo --in-place --name D1d --content "9" --font @neo-grotesque --size 53 --color '#ffffff' --at 777,343 --anchor c --anchor-to ink
pixelox text add profile.pxo --in-place --name D2d --content "21" --font @neo-grotesque --size 53 --color '#ffffff' --at 976,343 --anchor c --anchor-to ink
pixelox shape path profile.pxo --in-place --name Banner1 --d 'M 383 440 L 659 440 L 645 476 L 370 476 Z' --fill '#ba1736'
pixelox text add profile.pxo --in-place --name Banner1T --content "Points this year" --font @neo-grotesque --weight 500 --scale-x 135% --size 23 --color '#1a1718' --at 514,458 --anchor c
pixelox layer add profile.pxo --in-place --name Brush
pixelox paint stroke profile.pxo --in-place --layer Brush --d 'M 390 539 A 46 46 0 1 1 482 539 A 46 46 0 1 1 390 539 M 534 539 A 46 46 0 1 1 626 539 A 46 46 0 1 1 534 539 M 459 734 A 46 46 0 1 1 551 734 A 46 46 0 1 1 459 734' --size 9 --hardness 90 --spacing 6 --size-jitter 70 --scatter 25 --taper 20%,30% --color '#b3252f'
pixelox text add profile.pxo --in-place --name R1n --content "31" --font @condensed-heavy --size 60 --color '#b3252f' --at 436,539 --anchor c --anchor-to ink
pixelox text add profile.pxo --in-place --name R2n --content "36" --font @condensed-heavy --size 60 --color '#b3252f' --at 580,539 --anchor c --anchor-to ink
pixelox text add profile.pxo --in-place --name R3n --content "24" --font @condensed-heavy --size 60 --color '#b3252f' --at 505,734 --anchor c --anchor-to ink
pixelox text add profile.pxo --in-place --name R1l --content $'LEAGUE\nGAMES' --font @fat-display --weight 900 --size 27 --leading 36 --align center --color '#b3252f' --at 436,630 --anchor c
pixelox text add profile.pxo --in-place --name R2l --content $'CUP\nGAMES' --font @fat-display --weight 900 --size 27 --leading 36 --align center --color '#b3252f' --at 580,630 --anchor c
pixelox text add profile.pxo --in-place --name R3l --content "THE PLAYOFFS" --font @fat-display --weight 900 --size 27 --color '#b3252f' --at 505,809 --anchor c
pixelox render js scripts/bars.js -o skills.png --size 1080x1350 --param box=62.4,35.2,35.6,22.6 --param values='4.6,3.9,5,4.1,4.7;1.8,1.5,2.1,1.6,1.9' --param labels='PACE|FIRST STEP|JUMP SHOT|COURT SENSE|BALL CARE' --param colors='#df1849,#484755' --param max=5 --param axis=0 --param vlabels=0 --param gridlines=0 --param legend=none --param gap=0.3 --param radius=0 --param fs=17 --param ink='#df1849' --param muted='#df1849' --param font='Superclarendon, Alfa Slab One, Rockwell, Georgia, serif' --param weight=900
pixelox comp place profile.pxo skills.png --in-place --as Skills --at 0,0
pixelox shape path profile.pxo --in-place --name Axis --d 'M 674 483 L 674 736 L 1058 736' --fill none --stroke '#2774cc' --stroke-width 6
pixelox text add profile.pxo --in-place --name Ticks --content $'5\n4\n3\n2\n1\n0' --font @condensed-heavy --size 22 --leading 50.6 --align right --color '#071e6f' --at 662,610 --anchor r
pixelox text add profile.pxo --in-place --name G1 --content $'BLOCK\nRATE' --font @fat-display --weight 900 --size 17 --leading 22 --align center --color '#484755' --at 712,812 --anchor c
pixelox text add profile.pxo --in-place --name G2 --content $'STEAL\nRATE' --font @fat-display --weight 900 --size 17 --leading 22 --align center --color '#484755' --at 790,812 --anchor c
pixelox text add profile.pxo --in-place --name G3 --content $'POST\nPLAY' --font @fat-display --weight 900 --size 17 --leading 22 --align center --color '#484755' --at 866,812 --anchor c
pixelox text add profile.pxo --in-place --name G4 --content $'FREE\nSHOT' --font @fat-display --weight 900 --size 17 --leading 22 --align center --color '#484755' --at 944,812 --anchor c
pixelox text add profile.pxo --in-place --name G5 --content $'BOX\nOUT' --font @fat-display --weight 900 --size 17 --leading 22 --align center --color '#484755' --at 1020,812 --anchor c
pixelox shape path profile.pxo --in-place --name Banner2 --d 'M 360 854 L 667 854 L 651 904 L 342 904 Z' --fill '#b8092a'
pixelox text add profile.pxo --in-place --name Banner2T --content "Points in her career" --font @condensed --weight 500 --size 39 --color '#1a1718' --at 505,879 --anchor c
pixelox render js scripts/bars.js -o career.png --size 1080x1350 --param box=32.6,69.2,36,26.5 --param dir=h --param values=18,22,25,20,16,23 --param colors='#77a5ce,#ef306c,#77a5ce,#ef306c,#77a5ce,#ef306c' --param max=25 --param gap=0.3 --param radius=0 --param labelwidth=0 --param fs=26 --param ink='#1a1718' --param numfont='Impact, Anton, Arial Narrow, sans-serif' --param numweight=400
pixelox comp place profile.pxo career.png --in-place --as Career --at 0,0
pixelox text add profile.pxo --in-place --name CareerNames --content $'League 2023-2024\nCup 2023-2024\nLeague 2024-2025\nCup 2024-2025\nPlayoffs 2023-2024\nPlayoffs 2024-2025' --font @condensed --size 25 --leading 60 --color '#1a1718' --at 362,1113 --anchor l
pixelox paint stroke profile.pxo --in-place --layer Brush --d 'M 601 964 A 20 25 0 1 1 641 964 A 20 25 0 1 1 601 964 M 656 1024 A 20 25 0 1 1 696 1024 A 20 25 0 1 1 656 1024 M 697 1083 A 20 25 0 1 1 737 1083 A 20 25 0 1 1 697 1083 M 629 1143 A 20 25 0 1 1 669 1143 A 20 25 0 1 1 629 1143 M 574 1203 A 20 25 0 1 1 614 1203 A 20 25 0 1 1 574 1203 M 670 1262 A 20 25 0 1 1 710 1262 A 20 25 0 1 1 670 1262' --size 4 --hardness 90 --spacing 8 --size-jitter 60 --scatter 20 --taper 15%,25% --color '#24519f'
pixelox shape path profile.pxo --in-place --name Discs --d 'M 769 973 a 42 42 0 1 0 84 0 a 42 42 0 1 0 -84 0 Z M 955 973 a 42 42 0 1 0 84 0 a 42 42 0 1 0 -84 0 Z M 769 1188 a 42 42 0 1 0 84 0 a 42 42 0 1 0 -84 0 Z M 955 1188 a 42 42 0 1 0 84 0 a 42 42 0 1 0 -84 0 Z' --fill '#ffffff'
pixelox render js scripts/donut.js -o ring1.png --size 1080x1350 --param kind=ring --param cx=75.1 --param cy=72.1 --param r=6 --param inner=0.65 --param round=0 --param value=24 --param colors='#4c7cbc' --param trackcolor='#6f9cd1' --param ink='#6b99d4' --param numweight=400 --param centersize=35
pixelox render js scripts/donut.js -o ring2.png --size 1080x1350 --param kind=ring --param cx=92.3 --param cy=72.1 --param r=6 --param inner=0.65 --param round=0 --param value=55 --param colors='#4c7cbc' --param trackcolor='#6f9cd1' --param ink='#6b99d4' --param numweight=400 --param centersize=35
pixelox render js scripts/donut.js -o ring3.png --size 1080x1350 --param kind=ring --param cx=75.1 --param cy=88 --param r=6 --param inner=0.65 --param round=0 --param value=62 --param colors='#4c7cbc' --param trackcolor='#6f9cd1' --param ink='#6b99d4' --param numweight=400 --param centersize=35
pixelox render js scripts/donut.js -o ring4.png --size 1080x1350 --param kind=ring --param cx=92.3 --param cy=88 --param r=6 --param inner=0.65 --param round=0 --param value=41 --param colors='#4c7cbc' --param trackcolor='#6f9cd1' --param ink='#6b99d4' --param numweight=400 --param centersize=35
pixelox comp place profile.pxo ring1.png --in-place --as Ring1 --at 0,0
pixelox comp place profile.pxo ring2.png --in-place --as Ring2 --at 0,0
pixelox comp place profile.pxo ring3.png --in-place --as Ring3 --at 0,0
pixelox comp place profile.pxo ring4.png --in-place --as Ring4 --at 0,0
pixelox text add profile.pxo --in-place --name RL1 --content $'Steals in\ngames' --font @neo-grotesque --size 24 --leading 125% --color '#1a1718' --at 766,1052 --anchor tl
pixelox text add profile.pxo --in-place --name RL2 --content $'Points in\nthe paint' --font @neo-grotesque --size 24 --leading 125% --color '#1a1718' --at 952,1052 --anchor tl
pixelox text add profile.pxo --in-place --name RL3 --content $'Shots made\nin games' --font @neo-grotesque --size 24 --leading 125% --color '#1a1718' --at 766,1268 --anchor tl
pixelox text add profile.pxo --in-place --name RL4 --content $'Points from\nfree throws' --font @neo-grotesque --size 24 --leading 125% --color '#1a1718' --at 952,1268 --anchor tl
pixelox shape path profile.pxo --in-place --name NameTag --d 'M 50 1024 L 288 1024 L 276 1055 L 37 1055 Z' --fill '#b00927'
pixelox text add profile.pxo --in-place --name NameT --content "Guard Number 12" --font @neo-grotesque --weight 500 --scale-x 135% --size 19 --color '#1a1718' --at 163,1040 --anchor c
pixelox text add profile.pxo --in-place --name S1l --content "Usage Rate" --font @condensed --size 21 --color '#1a1718' --at 84,1081 --anchor c
pixelox text add profile.pxo --in-place --name S2l --content "Shot %" --font @condensed --size 21 --color '#1a1718' --at 197,1081 --anchor c
pixelox text add profile.pxo --in-place --name S3l --content "Clutch %" --font @condensed --size 21 --color '#1a1718' --at 283,1081 --anchor c
pixelox text add profile.pxo --in-place --name S1v --content "78%" --font @condensed --size 32 --color '#1a1718' --at 84,1112 --anchor c
pixelox text add profile.pxo --in-place --name S2v --content "61%" --font @condensed --size 32 --color '#1a1718' --at 197,1112 --anchor c
pixelox text add profile.pxo --in-place --name S3v --content "72%" --font @condensed --size 32 --color '#1a1718' --at 283,1112 --anchor c
pixelox shape path profile.pxo --in-place --name StatRules --d 'M 24 1094 L 314 1094 M 24 1131 L 314 1131 M 24 1163 L 314 1163 M 24 1193 L 314 1193' --fill none --stroke '#1a1718' --stroke-width 1.5
pixelox text add profile.pxo --in-place --name HomeAway --content "Wins at home and away" --font @condensed --size 21 --color '#1a1718' --at 169,1178 --anchor c
pixelox shape path profile.pxo --in-place --name TriA --d 'M 48 1331 L 123 1202 L 197 1331 Z' --fill '#a80843'
pixelox shape path profile.pxo --in-place --name TriAshade --d 'M 123 1202 L 197 1331 L 160 1331 Z' --fill '#8b0537'
pixelox shape path profile.pxo --in-place --name TriB --d 'M 151 1331 L 207 1230 L 263 1331 Z' --fill '#6c9ac5'
pixelox shape path profile.pxo --in-place --name TriBshade --d 'M 207 1230 L 263 1331 L 236 1331 Z' --fill '#4d7fb6'
pixelox text add profile.pxo --in-place --name TriAn --content "19" --font @neo-grotesque --size 50 --color '#ffffff' --at 112,1294 --anchor c --anchor-to ink
pixelox text add profile.pxo --in-place --name TriBn --content "11" --font @neo-grotesque --size 39 --color '#ffffff' --at 205,1308 --anchor c --anchor-to ink
pixelox filter grain profile.pxo --in-place --as-layer --name Concrete --amount 14
pixelox export profile.pxo -o profile.png
```

Reuse: change the title, then one module at a time. The skill chart's second label row sits
on the pair centers (the mean x of `bar1.1` and `bar1.2`, and so on; 712, 790, 866, 944, 1020
here); the axis path and the tick numbers follow the logged `plot` rectangle (674,483 to
1058,736). The small
rings in the bar list are centered 23 px to the right of the `bar1` to `bar6` anchors: run
the career chart with `--json` again after changing `values=` and move the six ellipses in
the `paint stroke` path. Keep every label in the skill chart to two lines of six letters or
fewer, or lower `fs`. Add a source line (C-source-line) at 98.5% H when the numbers are real.

## 13. Variations and pairings

* **Team sheet:** replace the cutout with a team photo cutout or a crest (`logo.png`) and
  the mini table with the league position; the same eight modules hold team totals.
* **Other palettes:** take the two colors from the kit in the photo
  (`analyze colors subject.png --k 5`); keep the ground a cool or warm gray and the type black.
* **Fewer modules:** for a square post keep the photo, the title, the three boxed counts,
  and the four rings; drop the charts.
* **One honest comparison instead of the triangles:** a C-percent-bar with two shares, or
  two C-stat-block numbers side by side.
* **Product or employee profile:** the structure works for anything with a cutout and many
  small numbers: a product (specs), a city (key figures), a candidate for an award.
* Pairs with C-ranked-list (a top five in the middle column) and C-timeline (career
  milestones along the bottom).

## 14. Pitfalls

* **Pseudo-3D bars.** A sloped or beveled top has two heights, so no bar has a readable
  value; the original's bars also pass the top of their own scale. Draw flat tops and keep
  values inside the axis range.
* **An axis that starts at 1.** A 1 to 5 rating drawn from a baseline labeled 1 makes a 2
  look a quarter the size of a 5 instead of 40%. `scripts/bars.js` starts at zero; label the
  scale "rating out of 5".
* **Triangles as amounts.** Readers compare heights (1.3 to 1 here) while the values differ
  by 1.7 to 1; only the areas match, and the overlap hides part of one. If you keep the
  motif, print both numbers large, do not overlap the shapes, and scale by area (side
  length proportional to the square root of the value).
* **Unsorted bars with alternating colors.** The six career bars alternate color by season
  and are in calendar order, so they are neither a ranking nor a clear pair comparison. Say
  what the colors mean with a two-key legend, or group them (C-grouped-columns: three
  competitions, two seasons).
* **Labels inside short bars.** A name set inside its bar overflows when the value is
  small. Keep the scale so the shortest bar is longer than the longest name, or move the
  names to the left of the bars (`labelwidth=0.45`).
* **Too many module styles.** Boxes, rings, bars, triangles, and a table already make five
  kinds. Reuse them; do not add a sixth.
* **Numbers about a real person from memory.** Every figure here belongs to one named
  athlete. Use only numbers the user supplies and name the source and the date.
