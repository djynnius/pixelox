---
id: i005
title: Three-view resource spread with a radial hub
kind: [statistical, comparison]
topic: [environment, science, business]
mood: [editorial, vibrant, technical, bold]
style: [flat, editorial, 3d]
format: landscape
structure: S-hub-spokes
data: [comparison, ranking, part-to-whole]
charts: [C-radial-bars, C-3d-bars, C-progress-ring, C-icon-array, C-table, C-stat-block]
palette: ['#f4f4f0', '#e2e5df', '#3391bd', '#189180', '#ec9379', '#e6c04f', '#111111']
needs: nothing
---

# i005: Three-view resource spread with a radial hub

## 1. Concept

A magazine double-page spread that shows one list of about twenty materials three times,
with three different questions. Left: how much of each is used (tall columns on a log
scale). Center: how many years each will last (bars shooting out of a white hub like the
spokes of a wheel, each with a round pictogram of what the material is used for). Right:
how much is recycled (a cloud of small donuts whose size grows with the share). A block of
tiny people at the bottom right gives the scale of "everyone" against "one country". It
works because every material keeps one color in all three views, so a reader can follow a
single material across the page, and because the central wheel turns a dull ranked list
into the picture that sells the story.

## 2. Data story

* **One category list, 19 items** (materials), shown in alphabetical order everywhere; one
  item has no data in some views ("n/a").
* **View 1, amounts across categories:** one value per item spanning **seven orders of
  magnitude** (from a fraction of a gram to many kilograms per person and year), hence a
  logarithmic axis of seven decades with a label on every column.
* **View 2, two durations per item:** years left under two scenarios (today's rate, and a
  heavier one), from single digits to more than a thousand; the heavier scenario is always
  the shorter bar. Most items lie between ten and about 150 years; three are far longer.
* **View 3, one share per item:** from 0% to about 70% of use met by recycling; several
  items at 0%, a cluster between 15% and 50%, one near 70%. Each share is its own whole.
* **Two totals:** a world count (billions) and one country's count (hundreds of millions),
  a ratio of about 20 to 1, shown as counted figures.
* **A small table:** 8 items with a range of years ("N to M years") under a third scenario.

Placeholder data used in the recipe (example data, invented: ten spare parts held by a
bike-share depot, in alphabetical order from bells to tyres):

| View | Values |
| --- | --- |
| Parts used per 100 bikes a year | 33, 96, 41, 17, 64, 12, 28, 8, 52, 81 |
| Weeks of stock left at today's rate | 54, 118, 66, 74, 110, 102, 84, 92, 49, 79 |
| Weeks left at last summer's peak rate | 32, 70, 33, 44, 62, 55, 47, 41, 30, 39 |
| Share refurbished, seven of the parts | 28%, 38%, 22%, 14%, 47%, 64%, 18% |
| Riders | 48,000 in the network; 2,400 of them (5 in 100) use this depot |

## 3. Information architecture

1. **Title** (top left): the question, in heavy condensed caps.
2. **Left third:** the column chart; the axis title runs up its left side.
3. **Center:** the wheel. The hub holds the two definitions (what each of the two bar
   shades means, written as fractions) and a caveat in italics. Around it the bars, each
   with name, uses, pictogram, and two numbers.
4. **Right fifth:** a right-aligned caps heading, then the donut cloud, each donut labelled
   underneath with name and percentage.
5. **Bottom right:** the crowd block with two number plates.
6. **Bottom left of the wheel:** a boxed note with a heading, two lines, and the small
   table; a one-line source under it.

The three views are read left to right, but the wheel wins the first look because it is the
largest and sits on a white disc that cuts across the striped ground.

## 4. Format and grid

* Landscape, 1.63 : 1 (a magazine spread). Recipe canvas 1600 x 980.
* Title: 1% W, 1% H, cap height 2.9% H.
* Column chart: `3.2%,5%,23.5% x 91%` (axis numbers at 0.5% to 3% W; baseline at 96% H).
* White disc: center 57.3% W, 50.8% H, radius 29% W; fine concentric rings every 2.2% W.
* Hub: same center, radius 12.3% W. Bars: bands 3.5 to 4.5% W wide starting at the hub's
  edge, one every 18.9 degrees, lengths from 5% W to more than 40% W (the longest run off
  the page).
* Donut cloud: 79% to 98% W, 5% to 78% H; heading right-aligned at 99% W, 3% to 9% H.
* Crowd: `78.5%,84%,20% x 14%` as a rhombus; plates at its upper left and lower right.
* Note box: `30%,85%,14% x 12.5%` (26.5% W in the recipe, clear of the columns).
* Recipe wheel (10 items): center 55% W, 50.8% H; disc radius 26.9% W; hub radius 9.4% W
  (a 9.75% W white disc covers the bar roots); a full-length bar ends at 23.5% W; one bar
  every 36 degrees, clockwise from 12 o'clock. Recipe blocks: `1.5%,13%,25% x 83%`.

## 5. Layer stack

1. Ground: paper `#f4f4f0` with fine horizontal stripes `#e2e5df` (4 px pitch in the
   original); the stripes turn pale blue `#cfe3f0` on the right fifth.
2. Disc: off-white `#fbfbf8` with hairline concentric rings `#dfe3dc`.
3. Columns: 19 square prisms, each in its material's color, with a lighter top facet and a
   darker side; values written vertically above them; names at 45 degrees under the axis.
4. Radial bars in the same colors: a darker inner part (the shorter scenario) and a lighter
   outer part (the longer one), a small number at the end of each part; name in bold caps
   and uses in brackets running along one edge of the bar.
5. Pictogram badges: black discs 6% W across with a 3 px ring in the bar's color and a white
   line drawing (a car, a tin can, a bucket, a screen, a camera and phone, rings, pills).
6. Hub: white disc with a soft shadow; definitions in black, two small swatch squares.
7. Donuts: white discs with soft shadows; a colored ring segment from 12 o'clock clockwise
   for the share, the rest of the ring white with a gray outline; a green recycling mark in
   the hole; name and percentage in bold caps underneath.
8. Crowd: a white rhombus with a black outline carrying a grid of about 600 small red
   figures and, in the near corner, about 28 blue ones; two plates (a colored label strip
   over a white box with a heavy black number).
9. Note box: white, no outline, heavy caps heading, small text, two-column table.
10. Title in black.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#f4f4f0` | ground; stripes `#e2e5df`; disc `#fbfbf8` | 69% | nothing |
| `#3391bd` | item 1 in all three views (blue); also the country's figures | 4% | a material; "home" in the crowd |
| `#189180` | item 2 (teal) | 3% | a material |
| `#ec9379` | item 3 (salmon) | 3% | a material |
| `#e6c04f` | item 4 (yellow); further items in `#f0a030`, `#c8232c`, `#66649d`, `#529371`, `#a9cfe6`, `#efabb0` | 12% | one material each |
| `#111111` | title, badges, numbers | 5% | nothing |
| `#c8232c` | world figures in the crowd, one plate | 4% | "everyone" |

Categorical with 19 hues: far more than a reader can hold, which is acceptable only because
every mark is also labelled by name. The colors' job is continuity between the three views.
In each radial bar a darker and a lighter step of the same hue encode the two scenarios.

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Title | a question of 5 to 6 words | caps | condensed heavy (Anton, League Gothic / Impact) | 400 (the face is heavy) | 46 | 0 | - | left | `#111111` | 1%, 1% |
| Module heading (right) | 7 to 8 words on 2 lines, unit in brackets | caps | condensed heavy | 400 | 28 | 0 | 110% | right | `#111111` | 99%, 3% |
| Axis title | measure, year, unit | caps and lower | condensed heavy | 400 | 18 | 0 | - | turned 90 degrees in the original; horizontal in the recipe | `#111111` | left of the columns |
| Column value (x19) | number and unit letter | - | condensed, bold (Oswald / Avenir Next Condensed) | 700 | 13 in the original (turned 90 degrees); 18 in the recipe | 0 | - | center | `#111111` | above each column |
| Column name (x19) | 1 word | caps | condensed | 500 | 12 at 45 degrees; 16 in the recipe, upright, stepping down the floor's front edge | 0 | - | center | `#111111` | under each column |
| Bar name and uses (x19) | name; 2 to 4 uses in brackets | caps, then lower | condensed | 700 / 400 italic | 14 along the bar; in the recipe the name only, 18, upright at the bar's end | 0 | - | along the bar | `#111111` | beside each bar |
| Bar numbers (x38) | 1 to 4 digits | - | condensed | 400 (700 in the recipe) | 12; 16 to 18 in the recipe | 0 | - | center | `#ffffff` or `#111111` | at the end of each part |
| Hub heads (x2) | 4 words; a 6 to 8 word condition under it (the recipe: one head at 28 and two keys of 3 to 5 words at 15) | caps | condensed heavy | 400 | 24 / 11 | 0 | - | left, right | `#111111` | hub |
| Hub formula | a fraction of two short phrases | sentence | neo-grotesque (Inter / Helvetica Neue) | 400 | 11 | 0 | - | center | `#111111` | under each head |
| Donut label (x19) | name and NN% | caps | condensed | 700 | 14 | 0 | - | center | `#111111` | under each donut |
| Plate label (x2) | 3 to 4 words with a date | caps | condensed | 700 | 12 | 0 | - | left | `#ffffff` | on the colored strips |
| Plate number (x2) | a count with thousands separators | - | condensed heavy | 400 | 30 | 0 | - | left | `#111111` | in the white boxes |
| Note heading, text, table | 3 words; 20 words; 8 rows of name and range | caps; sentence | condensed heavy; neo-grotesque; condensed | - | 20 / 12 / 12 | 0 | 16 px | left | `#111111` | note box |

Two families: a heavy condensed face for every heading and plate number and a condensed
grotesque for labels, with a plain grotesque only for the fine print. Sizes 46 : 30 : 28 :
14 : 12 on a 1600 px canvas: the original is a print spread and its labels are too small
for a screen; the recipe uses 18 px labels and fewer items.

## 8. Charts and diagrams

* **C-radial-bars** x2 for the wheel (the hero): `scripts/radial.js`, placed by center
  (`cx=55 cy=50.8 r=23.5 inner=0.4 barwidth=0.46 max=120`), ten bars, one every 36 degrees
  clockwise from 12 o'clock, in alphabetical order. Two runs on the same center with the
  same `max=` make the two-part bars: first the longer scenario in pale tints (each item's
  color mixed half-way to white) with its name and value at the bar's end, then the
  shorter scenario in the full colors on top, its value written just past its end, on the
  pale part. Lengths are linear from the hub's edge (226 px for 120 weeks). No axis and no
  guide values: both numbers are printed on every bar. The hub is a native white disc
  (radius 156 px) with a soft shadow that covers the bar roots and holds the heading and
  the two keys (a dark and a pale gray swatch, because every bar has its own hue).
* Simplified against the original's wheel: it has 19 straight bands of constant width
  (3.5 to 4.5% W), one every 18.9 degrees from about 330 degrees, the longest clipped at
  the page edge, names and uses written along the bands, and badges wider than the bands.
  The script draws wedges that widen outward (keep `barwidth` under 0.5 so they still read
  as bars), keeps every bar inside the disc, and writes upright names.
* **C-3d-bars** for view 1: `scripts/bars3d.js`, `box=1.5,13,25,83`, one color per block
  (`colors=` with 10 hexes), `max=100 gap=0.6`, floor `#e2e5df`, values above the blocks,
  names stepping down the floor's front edge (which gives the names the room the original
  gets from setting them at 45 degrees). There is no value axis: every value is printed.
  The script has **no logarithmic scale**; the recipe's example values span one order of
  magnitude so linear heights still show them.
* **C-progress-ring** x7 for view 3: `scripts/donut.js`, `kind=ring inner=0.62 round=0`,
  placed by center, **radius = 0.64% W x square root of the percentage** so that the areas
  follow the shares; white track; the percentage in the hole (the original puts a recycling
  mark there and the percentage in the label). Keep `r` at 2.4 or more on this canvas: a
  smaller ring loses its white track.
* **C-icon-array** for the crowd: `scripts/waffle.js`, `shape=person cols=20`,
  `values=95,5` (red, blue).
* **C-stat-block** (plates) and **C-table** (the note's two columns).
* **3D spec, columns:** in the original, square prisms on one horizontal baseline, viewed
  from the front and slightly above (elevation about 15 degrees, rotated 45 degrees so one
  edge faces the viewer); front-left face in the base color, front-right face 20% darker,
  top facet 35% lighter; light from the upper left. In the recipe, isometric blocks
  (parallel view, 30 degree axes) in a row on a pale floor that runs down to the right;
  front face in the base color, top 22% lighter, right side 28% darker, light from the
  upper left. Depth encodes nothing: only the block heights carry the values.
* **3D spec, crowd:** an isometric floor (a rhombus, 30 degree axes) with figures standing
  in a 30 x 20 grid; each figure a small rounded body with a white head, red (or blue), lit
  from the upper left with a darker right side; the blue figures occupy the near right
  corner. Counts, not depth, carry the meaning: every figure stands for the same number of
  people.

## 9. Icons and illustration

* Wheel pictograms: 28 px, white, stroke 2.5, each on a 52 px black disc `#111111` with a
  3 px ring in its bar's color, centered on the middle of the bar's darker part: `bell`,
  `bolt`, `cable`, `battery-full`, `link`, `cog`, `lightbulb`, `lock`, `armchair`,
  `life-buoy`. The original: white line drawings of what each material is used for, on
  black discs 6% W across with a colored ring.
* The donuts' recycling mark is `recycle` (green `#1f9d55`), 45% of the donut's diameter,
  when the percentage moves to the label.
* No photos.

## 10. Effects and finish

* Striped ground: `scripts/pattern.js`, `kind=stripes angle=90 cell=8`, pale.
* Concentric rings on the disc: `scripts/pattern.js`, `kind=rings`, clipped to the disc
  with `mask add --from-path`.
* Soft shadows under the hub and the donuts (offset 4,6, blur 10, 25%): one
  `fx drop-shadow` on the hub disc, one on the first donut and `fx copy` to the others.

## 11. Hierarchy and eye path

1. The wheel: the white disc, the saturated bars, the black badges.
2. The title at the top left (black, but small for a title).
3. The tallest columns at the left, then the largest donut at the right.
4. The crowd block and its two heavy numbers.
5. Names and small numbers, when the reader looks up one material.

Title 46 px; plate numbers 30; right heading and hub heading 28; labels 18 in the recipe.

## 12. Build recipe

Nothing to supply. Canvas 1600 x 980. All figures are example data.

```bash
pixelox new -o spread.pxo --size 1600x980 --fill '#f4f4f0'
pixelox render js scripts/pattern.js -o stripes.png --size 1600x980 --param kind=stripes --param color='#e2e5df' --param cell=8 --param weight=0.5 --param angle=90
pixelox comp place spread.pxo stripes.png --in-place --as Stripes --at 0,0
pixelox shape ellipse spread.pxo --in-place --name Disc --rect 450,68,860x860 --fill '#fbfbf8'
pixelox render js scripts/pattern.js -o rings.png --size 1600x980 --param kind=rings --param color='#dfe3dc' --param cell=36 --param weight=0.05 --param cx=0.55 --param cy=0.508
pixelox comp place spread.pxo rings.png --in-place --as Rings --at 0,0
pixelox mask add spread.pxo --in-place --layer Rings --from-path 'M 450 498 A 430 430 0 1 0 1310 498 A 430 430 0 1 0 450 498 Z'
pixelox text add spread.pxo --in-place --name Title --content "WHICH PARTS RUN OUT FIRST?" --font @condensed-heavy --size 46 --color '#111111' --at 40,24 --anchor tl --anchor-to ink
pixelox text add spread.pxo --in-place --name AxisTitle --content "PARTS USED PER 100 BIKES, 2030" --font @condensed-heavy --size 18 --color '#111111' --at 40,108 --anchor l
pixelox render js scripts/bars3d.js -o use.png --size 1600x980 --param box=1.5,13,25,83 --param values=33,96,41,17,64,12,28,8,52,81 --param labels='Bells|Bolts|Cables|Cells|Chains|Cogs|Lamps|Locks|Seats|Tyres' --param colors='#3391bd,#189180,#ec9379,#e6c04f,#66649d,#a9cfe6,#f0a030,#529371,#c8232c,#efabb0' --param max=100 --param gap=0.6 --param floor='#e2e5df' --param ink='#111111' --param muted='#111111' --param fs=18 --param font='Avenir Next Condensed, Oswald, Barlow Condensed, Arial Narrow, sans-serif'
pixelox comp place spread.pxo use.png --in-place --as Use --at 0,0
pixelox render js scripts/radial.js -o today.png --size 1600x980 --param cx=55 --param cy=50.8 --param r=23.5 --param inner=0.4 --param barwidth=0.46 --param max=120 --param values=54,118,66,74,110,102,84,92,49,79 --param labels='Bells|Bolts|Cables|Cells|Chains|Cogs|Lamps|Locks|Seats|Tyres' --param colors='#99c8de,#8cc8c0,#f6c9bc,#f3e0a7,#b3b2ce,#d4e7f3,#f8d098,#a9c9b8,#e49196,#f7d5d8' --param ink='#111111' --param muted='#111111' --param fs=20 --param font='Avenir Next Condensed, Oswald, Barlow Condensed, Arial Narrow, sans-serif'
pixelox comp place spread.pxo today.png --in-place --as WheelToday --at 0,0
pixelox render js scripts/radial.js -o peak.png --size 1600x980 --param cx=55 --param cy=50.8 --param r=23.5 --param inner=0.4 --param barwidth=0.46 --param max=120 --param values=32,70,33,44,62,55,47,41,30,39 --param colors='#3391bd,#189180,#ec9379,#e6c04f,#66649d,#a9cfe6,#f0a030,#529371,#c8232c,#efabb0' --param ink='#111111' --param fs=18 --param font='Avenir Next Condensed, Oswald, Barlow Condensed, Arial Narrow, sans-serif'
pixelox comp place spread.pxo peak.png --in-place --as WheelPeak --at 0,0
pixelox shape ellipse spread.pxo --in-place --name Hub --rect 724,342,312x312 --fill '#ffffff'
pixelox fx drop-shadow spread.pxo --in-place --layer Hub --offset 4,6 --blur 10 --color '#000000' --opacity 25
pixelox text add spread.pxo --in-place --name HubHead --content "WEEKS OF STOCK LEFT" --font @condensed-heavy --size 28 --color '#111111' --at 880,442 --anchor c
pixelox shape rect spread.pxo --in-place --name KeyPeak --rect 782,472,16x16 --fill '#5d625d'
pixelox text add spread.pxo --in-place --name KeyPeakT --content "at last summer's peak rate" --font @neo-grotesque --size 15 --color '#111111' --at 806,480 --anchor l
pixelox shape rect spread.pxo --in-place --name KeyToday --rect 782,500,16x16 --fill '#c9cdc8'
pixelox text add spread.pxo --in-place --name KeyTodayT --content "at today's rate" --font @neo-grotesque --size 15 --color '#111111' --at 806,508 --anchor l
pixelox text add spread.pxo --in-place --name Caveat --content "Parts already on order are not counted" --font @neo-grotesque --style italic --size 13 --color '#111111' --at 880,552 --anchor c
pixelox shape ellipse spread.pxo --in-place --name B1 --rect 854,291,52x52 --fill '#111111' --stroke '#3391bd' --stroke-width 3
pixelox shape ellipse spread.pxo --in-place --name B2 --rect 981,297,52x52 --fill '#111111' --stroke '#189180' --stroke-width 3
pixelox shape ellipse spread.pxo --in-place --name B3 --rect 1027,416,52x52 --fill '#111111' --stroke '#ec9379' --stroke-width 3
pixelox shape ellipse spread.pxo --in-place --name B4 --rect 1036,531,52x52 --fill '#111111' --stroke '#e6c04f' --stroke-width 3
pixelox shape ellipse spread.pxo --in-place --name B5 --rect 977,641,52x52 --fill '#111111' --stroke '#66649d' --stroke-width 3
pixelox shape ellipse spread.pxo --in-place --name B6 --rect 854,674,52x52 --fill '#111111' --stroke '#a9cfe6' --stroke-width 3
pixelox shape ellipse spread.pxo --in-place --name B7 --rect 740,629,52x52 --fill '#111111' --stroke '#f0a030' --stroke-width 3
pixelox shape ellipse spread.pxo --in-place --name B8 --rect 674,530,52x52 --fill '#111111' --stroke '#529371' --stroke-width 3
pixelox shape ellipse spread.pxo --in-place --name B9 --rect 684,417,52x52 --fill '#111111' --stroke '#c8232c' --stroke-width 3
pixelox shape ellipse spread.pxo --in-place --name B10 --rect 744,321,52x52 --fill '#111111' --stroke '#efabb0' --stroke-width 3
pixelox shape icon spread.pxo bell --in-place --name W1 --size 28 --at 880,317 --anchor c --color '#ffffff' --stroke-width 2.5
pixelox shape icon spread.pxo bolt --in-place --name W2 --size 28 --at 1007,323 --anchor c --color '#ffffff' --stroke-width 2.5
pixelox shape icon spread.pxo cable --in-place --name W3 --size 28 --at 1053,442 --anchor c --color '#ffffff' --stroke-width 2.5
pixelox shape icon spread.pxo battery-full --in-place --name W4 --size 28 --at 1062,557 --anchor c --color '#ffffff' --stroke-width 2.5
pixelox shape icon spread.pxo link --in-place --name W5 --size 28 --at 1003,667 --anchor c --color '#ffffff' --stroke-width 2.5
pixelox shape icon spread.pxo cog --in-place --name W6 --size 28 --at 880,700 --anchor c --color '#ffffff' --stroke-width 2.5
pixelox shape icon spread.pxo lightbulb --in-place --name W7 --size 28 --at 766,655 --anchor c --color '#ffffff' --stroke-width 2.5
pixelox shape icon spread.pxo lock --in-place --name W8 --size 28 --at 700,556 --anchor c --color '#ffffff' --stroke-width 2.5
pixelox shape icon spread.pxo armchair --in-place --name W9 --size 28 --at 710,443 --anchor c --color '#ffffff' --stroke-width 2.5
pixelox shape icon spread.pxo life-buoy --in-place --name W10 --size 28 --at 770,347 --anchor c --color '#ffffff' --stroke-width 2.5
pixelox text add spread.pxo --in-place --name RightHead --content $'SHARE OF PARTS REFURBISHED,\nNOT BOUGHT NEW (%)' --font @condensed-heavy --size 28 --leading 110% --align right --color '#111111' --at 1570,30 --anchor tr
pixelox render js scripts/donut.js -o r1.png --size 1600x980 --param kind=ring --param cx=84.5 --param cy=19 --param r=3.39 --param inner=0.62 --param round=0 --param value=28 --param colors='#3391bd' --param trackcolor='#ffffff' --param ink='#111111'
pixelox render js scripts/donut.js -o r2.png --size 1600x980 --param kind=ring --param cx=93.5 --param cy=22 --param r=3.95 --param inner=0.62 --param round=0 --param value=38 --param colors='#e6c04f' --param trackcolor='#ffffff' --param ink='#111111'
pixelox render js scripts/donut.js -o r3.png --size 1600x980 --param kind=ring --param cx=86 --param cy=35.5 --param r=3 --param inner=0.62 --param round=0 --param value=22 --param colors='#ec9379' --param trackcolor='#ffffff' --param ink='#111111'
pixelox render js scripts/donut.js -o r4.png --size 1600x980 --param kind=ring --param cx=95 --param cy=38 --param r=2.39 --param inner=0.62 --param round=0 --param value=14 --param colors='#a9cfe6' --param trackcolor='#ffffff' --param ink='#111111'
pixelox render js scripts/donut.js -o r5.png --size 1600x980 --param kind=ring --param cx=84 --param cy=50 --param r=4.39 --param inner=0.62 --param round=0 --param value=47 --param colors='#f0a030' --param trackcolor='#ffffff' --param ink='#111111'
pixelox render js scripts/donut.js -o r6.png --size 1600x980 --param kind=ring --param cx=93.8 --param cy=57 --param r=5.12 --param inner=0.62 --param round=0 --param value=64 --param colors='#66649d' --param trackcolor='#ffffff' --param ink='#111111'
pixelox render js scripts/donut.js -o r7.png --size 1600x980 --param kind=ring --param cx=83.5 --param cy=64.5 --param r=2.72 --param inner=0.62 --param round=0 --param value=18 --param colors='#efabb0' --param trackcolor='#ffffff' --param ink='#111111'
pixelox comp place spread.pxo r1.png --in-place --as Ring1 --at 0,0
pixelox fx drop-shadow spread.pxo --in-place --layer Ring1 --offset 4,6 --blur 10 --color '#000000' --opacity 25
pixelox comp place spread.pxo r2.png --in-place --as Ring2 --at 0,0
pixelox comp place spread.pxo r3.png --in-place --as Ring3 --at 0,0
pixelox comp place spread.pxo r4.png --in-place --as Ring4 --at 0,0
pixelox comp place spread.pxo r5.png --in-place --as Ring5 --at 0,0
pixelox comp place spread.pxo r6.png --in-place --as Ring6 --at 0,0
pixelox comp place spread.pxo r7.png --in-place --as Ring7 --at 0,0
pixelox fx copy spread.pxo --in-place --from Ring1 --to Ring2,Ring3,Ring4,Ring5,Ring6,Ring7
pixelox text add spread.pxo --in-place --name RL1 --content "BELLS" --font @condensed --weight 700 --size 16 --color '#111111' --at 1352,258 --anchor c
pixelox text add spread.pxo --in-place --name RL2 --content "CELLS" --font @condensed --weight 700 --size 16 --color '#111111' --at 1496,295 --anchor c
pixelox text add spread.pxo --in-place --name RL3 --content "CABLES" --font @condensed --weight 700 --size 16 --color '#111111' --at 1376,412 --anchor c
pixelox text add spread.pxo --in-place --name RL4 --content "COGS" --font @condensed --weight 700 --size 16 --color '#111111' --at 1520,427 --anchor c
pixelox text add spread.pxo --in-place --name RL5 --content "LAMPS" --font @condensed --weight 700 --size 16 --color '#111111' --at 1344,576 --anchor c
pixelox text add spread.pxo --in-place --name RL6 --content "CHAINS" --font @condensed --weight 700 --size 16 --color '#111111' --at 1501,657 --anchor c
pixelox text add spread.pxo --in-place --name RL7 --content "TYRES" --font @condensed --weight 700 --size 16 --color '#111111' --at 1336,690 --anchor c
pixelox render js scripts/waffle.js -o crowd.png --size 1600x980 --param box=78.5,78,19.5,15 --param values=95,5 --param cols=20 --param shape=person --param colors='#c8232c,#3391bd' --param legend=none
pixelox comp place spread.pxo crowd.png --in-place --as Crowd --at 0,0
pixelox shape rect spread.pxo --in-place --name PlateA --rect 1256,704,180x20 --fill '#c8232c'
pixelox text add spread.pxo --in-place --name PlateAt --content "ALL RIDERS, MAY 2031" --font @condensed --weight 700 --size 13 --color '#ffffff' --at 1262,714 --anchor l
pixelox shape rect spread.pxo --in-place --name PlateAbox --rect 1256,724,180x38 --fill '#ffffff' --stroke '#111111' --stroke-width 1
pixelox text add spread.pxo --in-place --name PlateAn --content "48,000" --font @condensed-heavy --size 30 --color '#111111' --at 1264,743 --anchor l
pixelox shape rect spread.pxo --in-place --name PlateB --rect 1388,914,180x20 --fill '#3391bd'
pixelox text add spread.pxo --in-place --name PlateBt --content "THIS DEPOT, MAY 2031" --font @condensed --weight 700 --size 13 --color '#ffffff' --at 1394,924 --anchor l
pixelox shape rect spread.pxo --in-place --name PlateBbox --rect 1388,934,180x38 --fill '#ffffff' --stroke '#111111' --stroke-width 1
pixelox text add spread.pxo --in-place --name PlateBn --content "2,400" --font @condensed-heavy --size 30 --color '#111111' --at 1396,953 --anchor l
pixelox shape rect spread.pxo --in-place --name NoteBox --rect 424,836,290x126 --fill '#ffffff'
pixelox text add spread.pxo --in-place --name NoteHead --content "WITH TWICE THE FLEET" --font @condensed-heavy --size 20 --color '#111111' --at 436,854 --anchor l
pixelox text add spread.pxo --in-place --name NoteNames --content $'SEATS\nBELLS\nCABLES' --font @condensed --weight 700 --size 15 --leading 22 --color '#111111' --at 436,904 --anchor l
pixelox text add spread.pxo --in-place --name NoteRanges --content $'3 to 5 weeks\n6 to 9 weeks\n10 to 14 weeks' --font @condensed --size 15 --leading 22 --color '#111111' --at 524,904 --anchor l
pixelox text add spread.pxo --in-place --name Source --content "Example data, invented. Source: Example Depot Stock Count" --font @neo-grotesque --size 12 --color '#3a3a3a' --at 436,950 --anchor l
pixelox export spread.pxo -o spread.png
```

Reuse: keep one color per item and pass the items in the same order to `bars3d.js`, both
`radial.js` runs, and the rings. The two wheel runs must share `cx`, `cy`, `r`, `inner`,
`barwidth`, and `max`; the pale run takes the larger value of each pair and each item's
color mixed half-way to white. The badge centers are bar anchors: run
`lab/anchors.py scripts/radial.js` with the wheel's `cx`, `cy`, `r`, and `inner`, the
shorter values, and `max=240` (twice the chart's `max`), which puts every anchor at the
middle of a darker part; a 52 px disc's
`--rect` starts 26 px up and left of its anchor, and the icon sits on the anchor. Read the
anchors again after changing a value, and keep every darker part longer than a badge
(here: 30 weeks or more). A ring's `r` in % of the width is 0.64 x the square root of its
percentage (2.4 at least); keep 10 px between rings and put each name 12 px under its
ring. With more than 12 items drop the badges; the script then writes the names along the
spokes.

## 13. Variations and pairings

* **Honest charts:** replace the wheel with C-bars (`dir=h`, two series, sorted by time
  left) when the durations must be compared exactly, and the blocks with C-columns
  (`scripts/bars.js`, same `values=` and `colors=`, `axis=1`, two-letter codes as names,
  because ten words do not fit under ten thin columns) when the amounts must be.
* **One view per slide:** left, center, and right each work as a 16:9 slide with the same
  colors; the color continuity carries across slides.
* **Fewer items:** with 5 or 6 bars (`barwidth=0.3`) the badges can grow to 80 px and the
  pictograms become readable at a distance.
* **Dark ground:** `#1c2126` with stripes `#232a30`, a `#2b333a` disc, white type; lighten
  the ten item colors one step.
* Pairs with C-ranked-list (the five shortest-lived items) in the note box and with
  C-percent-bar for the two populations.

## 14. Pitfalls

* **A linear scale for values that span many orders of magnitude.** The original needs a log
  axis; on a linear one all but two or three columns are flat. If your values differ by more
  than about 50 times, do not use columns or blocks: print a ranked table, or split the
  items into two charts with their own stated scales. And say "logarithmic scale" on any
  log chart: readers take column height for amount. This is the spread's main
  data-integrity risk.
* **Two-part bars on two scales, or clipped.** Both wheel runs must use the same `max=`,
  or the darker part of a bar can no longer be compared with its paler part. The
  original's longest bars run off the page, so their lengths cannot be compared either; if
  a value does not fit, break the bar visibly and print the number.
* **Blocks read by where their tops are on the page.** The isometric row runs down to the
  right, so a block's top sits lower than the top of an equal block to its left. Keep
  every value label on, and use flat C-columns when the comparison is the point.
* **A wheel that implies a cycle.** Spokes in alphabetical order look like a sequence; say
  "in alphabetical order" or sort by value so the wheel reads as a ranking.
* **Donuts sized by diameter.** Size the rings by area (radius by the square root), or keep
  them all one size; and never let a 0% item disappear: it still needs its name.
* **Twenty colors and print-size labels.** No legend can explain 19 hues, so every mark
  must carry its name, as here; and 12 px turned or angled type is unreadable at 1600 px:
  cut the item list to 10 and set labels at 18 px, upright.
