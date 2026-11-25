---
id: i014
title: Navy almanac map with value bubbles
kind: [geographic, statistical]
topic: [politics, education, society]
mood: [retro, authoritative, friendly, bold]
style: [flat, illustrated]
format: landscape
structure: S-map-callouts
data: [geography, comparison, single-stat]
charts: [C-us-bubble-map, C-ribbon-label, C-icon-badge, C-callout, C-source-line]
palette: ['#1e375c', '#dcd4c4', '#fbf6f3', '#f7a65e', '#7fb069', '#e86a5f']
needs: nothing
---

# i014: Navy almanac map with value bubbles

## 1. Concept

A landscape wall-chart in the manner of an old school almanac: a sand-colored map of a
country on a deep navy field, framed by a cream border, with a cream bubble on every region
that carries one number, the region's count of votes in an indirect election. The bubbles
grow with the numbers, so the handful of heavy regions jump out of a sea of small ones, and
small green and red arrow dots mark who gained and who lost since the last count. Short
explanations sit in the navy margins like captions in an atlas. It works because position
says "which region", bubble size says "how much", and the number inside removes any need to
guess; everything else on the sheet is quiet and in two colors.

## 2. Data story

* **One whole number per region, 51 regions:** from a fixed minimum (a low single digit) to
  about 18 times that. Strongly skewed: several regions at the minimum, a median under 10,
  most between 4 and 12, a handful above 16, and one far ahead of all. Relationship:
  geography plus comparison of sizes.
* **Change since the last count, for about a quarter of the regions:** roughly half of
  them gained, half lost, nearly all by one. A yes-or-no flag per region, shown as a small
  arrow dot on the bubble.
* **A worked sum** (how a region's number is made: one part by population plus a fixed part)
  and **the national sum** (three addends, a total in the 500s, and the majority needed).
* **One summary statistic:** the median, in a dashed circle.
* A legend that reuses one real bubble: "number", "gained", and one sentence saying that
  bubble size follows the number.

Placeholder data used in the recipe (example data, invented: seats per region in a
441-seat national council; every region has 1 fixed seat plus seats by population):
CA 47, TX 36, FL 27, NY 24, PA 16, IL 16, OH 14, GA 13, NC 13, MI 12, VA 12, AZ 12, NJ 11,
WA 11, then 37 regions from 10 down to 2 (median 6, 7 regions at the minimum). The counts
are made up and follow no real apportionment. Gained a seat: WA, AZ, MN. Lost a seat: IA,
MA.

## 3. Information architecture

1. **Title** in huge block caps across the top; under it a ribbon with the edition and three
   stars on each side; a script aside to the right.
2. **Publisher emblem** in the top left corner (a round badge).
3. **The map** with its bubbles: the eye goes to the three or four largest bubbles, then
   wanders.
4. **Left margin:** three short paragraphs (what this is, how often it changes, what the
   arrows mean); lower down a tabbed label with the formula and a worked example for one
   small region; at the bottom the median in a dashed circle.
5. **Right margin:** a tabbed label with the national sum; under it the legend built on one
   of the map's own bubbles.
6. **Bottom right:** site ribbon with a small round portrait, credits and source, a code to
   scan on an orange card.

The margins are read after the map, in any order: each block is self-contained and tied to
the map by a dashed line or by position.

## 4. Format and grid

* Landscape 3:2. Recipe canvas 1600 x 1066.
* Cream paper border 1.6% W on all sides; navy panel inside it; a 2 px cream keyline 0.5% W
  inside the panel's edge.
* Title: 20.5% to 79.4% W, cap top 8.4% H, cap height 5.9% H. Ribbon `42.9%,17%,14.2% x
  3.4%`; stars 4.5% W to each side of it.
* Emblem: a circle of 6.6% W centered at 8.6% W, 11.6% H.
* Map: the mainland from 17% to 87% W and 19.3% to 86% H; two remote regions as insets at
  the lower left (17% to 40% W, 73% to 92% H). Recipe box `14.5%,19.5%,72% x 67%`.
* Left margin column: 4.1% to 13.2% W; paragraphs 24% to 46% H; tab at 66.9% H (62% in the
  recipe, so the worked example ends above the inset's island tail); worked example to 82%
  H; median circle at 5.9% W, 91% H.
* Right margin column: 86.4% to 96% W; tab at 55.5% H; sum table 58.5% to 70% H; legend
  labels 73% to 83% H (in the recipe 78% to 86% W, 69% to 81% H, right of the sample
  bubble).
* Bottom right: site ribbon `75%,88.3%,14.6% x 2.7%`; credits under it; code card
  `92%,86.8%,4.6% x 8%`.

## 5. Layer stack

1. Paper: cream `#fbf6f3`.
2. Panel: navy `#1e375c`; keyline in cream, 2 px.
3. Map: regions in sand `#dcd4c4` with navy borders 2.5 px wide (the panel color shows
   between regions); coastlines detailed.
4. Bubbles: cream discs with a 2.5 px navy outline and a small pointed tail, one per region;
   diameter from 2.1% W (the minimum value) to about 8.5% W (the maximum). The largest
   overflow their regions.
   Regions too small for a bubble (a cluster in the north-east) have theirs offshore, joined
   by a dashed cream line. In the recipe the bubbles are plain circles, the script pushes
   crowded ones apart, and one that had to leave its region keeps a thin navy line to it.
5. Numbers: navy, bold slab, sized with the bubble (from 22 to 60 px).
6. Region codes: two-letter caps, 15 px bold, navy on the sand under each bubble, cream
   beside the offshore ones.
7. Change dots: 18 px discs at the bubble's upper right, green `#7fb069` with a white up
   arrow or red `#e86a5f` with a white down arrow.
8. Title (cream block caps), ribbon (orange `#f7a65e`, folds `#d9782f`, navy text), six
   cream stars, script aside in cream with a dashed curved arrow.
9. Emblem: a blue disc `#27477a` with a cream ring and a script word mark.
10. Margin texts in cream; two orange tabs with a notched right end and navy caps; sums
    with hairline rules; dashed arrows and leaders in cream (1.5 px, dashes 4 px).
11. Bottom right: orange site ribbon, round portrait (a man in a suit, monochrome), three
    lines of credits, a code on an orange card with a two-word caption.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#1e375c` | panel, all dark type, region borders | 52% | nothing |
| `#dcd4c4` | land | 22% | nothing: every region the same |
| `#fbf6f3` | paper border, bubbles, title, margin text | 18% | bubble area = the value |
| `#f7a65e` | ribbon, two tabs, site ribbon, code card | 2% | nothing: the single accent, marks labels |
| `#7fb069` | up-arrow dots | under 1% | gained since the last count |
| `#e86a5f` | down-arrow dots | under 1% | lost since the last count |

Three colors carry the whole sheet (navy, sand, cream); orange labels the furniture, and
green and red appear only as 18 px dots with arrows, so the arrow, not the color, carries
the meaning. The land is deliberately one flat color: the value is in the bubbles, and a
choropleth underneath would say it twice.

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Title | 3 words, one line | caps | slab, squared collegiate (Graduate, Roboto Slab / Rockwell) | 700 to 800 | 98 (fitted to 59% W) | 20 | - | center | `#fbf6f3` | 50%, cap top 8.4% |
| Ribbon | year and 2 words | caps | slab | 700 | 24 | 20 | - | center | `#1e375c` | 50%, 18.7% |
| Script aside | 4 to 5 words | sentence | brush script (Pacifico, Kaushan Script / SignPainter) | 400 | 26 | 0 | - | left | `#fbf6f3` | 65.7%, 16.9% |
| Bubble number (x51) | 1 to 2 digits | - | slab | 700 | 22 to 60, about 0.95 x the bubble radius (13 to 63 in the recipe, set by the script) | 0 | - | center | `#1e375c` | bubble centers |
| Region code (x51) | 2 letters | caps | slab | 700 | 15 (14 in the recipe) | 20 | - | center | `#1e375c` on land, `#fbf6f3` at sea (navy with a sand halo everywhere in the recipe) | under each bubble |
| Margin paragraph (x3) | 12 to 22 words each, lines of about 22 characters | sentence | condensed grotesque (Barlow Condensed, Oswald light / Avenir Next Condensed) | 500 | 16 (15 in the original) | 0 | 21 px | left | `#fbf6f3` | 4.1%, 24% to 46% |
| Tab label (x2) | 2 to 3 words | caps | slab | 700 | 17 to 20 (a wide slab such as Rockwell needs 17) | 20 | - | left | `#1e375c` | on the tabs |
| Formula text | 18 to 24 words | sentence | condensed grotesque | 500 | 16 | 0 | 21 px | left | `#fbf6f3` | under the left tab |
| Sum rows | a number and 1 to 3 words per row | sentence | slab for numbers, condensed for words | 700 / 500 | 17 / 16 | 0 | 22 px | numbers right, words left | `#fbf6f3` | both margins |
| Median | 1 digit; 5 words on 2 lines | sentence | slab / condensed | 700 / 500 | 34 / 16 | 0 | 21 px | center / left | `#fbf6f3` | 5.9%, 91% |
| Legend labels (x2, x3 in the recipe) and note | 3 to 4 words; a sentence on 2 or 3 lines | sentence | condensed grotesque | 500 | 16 | 0 | 21 px | left | `#fbf6f3` | right of the sample bubble |
| Site ribbon | a web address | caps | slab | 700 | 15 to 18 | 20 | - | left | `#1e375c` | 77%, 89.7% |
| Credits | 3 lines: design, source, rights | sentence | condensed grotesque | 400 | 12 | 0 | 16 px | left | `#fbf6f3` | 76%, 92% to 95% |

Three families: a squared slab for the title, every number, and every label; a condensed
grotesque for running text; one script line as a human touch. Sizes 98 : 60 to 22 : 20 : 16,
about 6 : 3.5 : 1.25 : 1. The numbers scale with their bubbles, so type size is itself data.

## 8. Charts and diagrams

* **C-us-bubble-map**, `scripts/usmap.js` (run with `render d3`): `bubbles=` with all 51
  regions, `empty='#dcd4c4'` (one flat land color), `stroke='#1e375c' strokewidth=2.5`,
  `bubblecolor='#fbf6f3' bubbleline='#1e375c'`, `labels=codes` with `muted='#1e375c'` and
  `halo='#dcd4c4'` (navy codes under the bubbles, with a sand halo so they also read on the
  water), `smalls=0`, `maxr=0.058` (the largest bubble's radius is 5.8% of the map's width:
  66 px for 47), `fs=20`, the slab stack as `font=` and `numfont=`, box `14.5,19.5,72,67`.
  The script makes the bubbles' **areas** proportional to the values, writes each number at
  about 0.95 x the radius, pushes bubbles apart where they would overlap, and draws a thin
  line back to the region when a bubble had to move more than its own radius. No axis, no
  legend from the script.
* **C-icon-badge** as change dots: a 26 px disc with a 16 px white `arrow-up` or
  `arrow-down` on the bubble's rim at 45 degrees (upper right). The script logs each
  bubble as `bx, by, br`; the dot's center is 0.71 x `br` right of and above `bx, by`.
* **C-ribbon-label** for the edition ribbon; the two tabs and the site ribbon are one
  `shape path` each (a strip with a notch cut into its right end).
* **C-callout** for the legend: a dashed leader from the sample bubble to its label, with
  the two change dots keyed above it.
* Simplified: the bubbles have no pointed tail; the script, not the designer, decides where
  crowded bubbles go, so the north-eastern ones are not lined up in an offshore column, and
  in the tightest spot a smaller bubble can cover a larger neighbor's code; the original's
  legend hangs "gained" and "number" on one bubble, the recipe keys the dots separately.

**Map spec:**

* Region set: 50 US states plus the capital district. The two non-contiguous states are
  insets at the lower left (in the original the island chain at 17% to 26% W, 73% to 79% H
  and the northern state, scaled to about 35%, at 30% to 40% W, 73% to 92% H; the script
  puts the northern state at the left and the island chain right of it).
* Projection: Albers USA (curved northern border, slightly tilted east coast).
* Land: one flat fill `#dcd4c4` for every region; borders `#1e375c`, 2.5 px; water is the
  panel color.
* Encoding: one bubble per region at its centroid, **area proportional to the value**:
  in the original diameter = 1.2% W x square root of the value (the minimum gives 2.1% W,
  the maximum 8.5% W). Fill `#fbf6f3`, outline `#1e375c`, and in the original a small tail
  at the lower right pointing into the region. The value is written in the bubble in the
  slab face at about 0.95 x the radius.
* Offshore bubbles, the original: nine small north-eastern regions have their bubbles in
  the water to the east, in a column from 80% to 87% W, each joined to its region by a
  dashed cream line (1.5 px, dash 4, gap 4) and with its code to the right in cream.
* Labels: two-letter codes, 15 px bold slab, navy, centered under each bubble; an asterisk
  on two codes refers to a footnote at the top right (not in the recipe).
* Change dots: discs on the bubble's rim at about 45 degrees (upper right), 18 px in the
  original and 26 px in the recipe: green with an up arrow for a gain, red with a down
  arrow for a loss.
* Legend: no separate key in the original; the south-eastern peninsula's bubble has two
  dashed leaders to the labels "gained" and "number", and a three-line note on bubble size
  under them. The recipe keeps the leader to "number" and the note, and keys the two dots
  above them.
* Callouts, the original: dashed curved arrows from two margin notes to single regions.
* Fallback for builds before 0.6.0: C-tile-map, `scripts/tilemap.js` (run with
  `render js`) with the same numbers as `values=`, `ramp='#dcd4c4,#dcd4c4'`,
  `showvalues=1`, `gap=0.07`, `legend=none`: every tile prints its code and value, and the
  bubbles are left out.

## 9. Icons and illustration

* `arrow-up` and `arrow-down`, 16 px, white, 3 px line, in the green and red dots.
* Stars: six five-point `shape star` marks, outer radius 9 px, cream.
* Emblem: the original is a round badge with the publisher's name in white script; the
  recipe draws the disc and writes "Your Mark" (or place `logo.png` at 106 px).
* Portrait: a 44 px round monochrome portrait of a man with a moustache in a suit, used as
  a mascot beside the site ribbon; the recipe uses the `user` icon in a cream disc. Never
  name the person.
* Code to scan: `shape qr` on a white plate inside an orange card.

## 10. Effects and finish

None: flat colors, no shadows, no gradients, no texture. The "printed" feel comes from the
cream border, the keyline, the slab type, and the limited palette. Line work is consistent:
2.5 px solid for borders and bubble outlines, 1.5 px dashed for every leader and arrow.

## 11. Hierarchy and eye path

1. The title (98 px, cream on navy, full width).
2. The largest bubble, then the next three or four: size ranks them without reading.
3. The colored arrow dots (the only green and red).
4. The orange ribbon and tabs, which open the margin texts.
5. Small bubbles and codes, when the reader looks for their own region.

Title 98 px; bubble numbers 60 down to 22 (63 down to 13 in the recipe); tabs 20; text 16; credits 12.

## 12. Build recipe

Nothing to supply. Canvas 1600 x 1066. All values are example data.

```bash
pixelox new -o almanac.pxo --size 1600x1066 --fill '#fbf6f3'
pixelox shape rect almanac.pxo --in-place --name Panel --rect 25,25,1550x1016 --fill '#1e375c'
pixelox shape rect almanac.pxo --in-place --name Keyline --rect 33,33,1534x1000 --fill none --stroke '#fbf6f3' --stroke-width 2
pixelox shape ellipse almanac.pxo --in-place --name Emblem --rect 84,70,106x106 --fill '#27477a' --stroke '#fbf6f3' --stroke-width 3
pixelox text add almanac.pxo --in-place --name EmblemText --content $'Your\nMark' --font @brush --size 30 --leading 90% --align center --color '#fbf6f3' --at 137,123 --anchor c
pixelox text add almanac.pxo --in-place --name Title --content "THE NATIONAL COUNCIL" --font @slab --weight 700 --tracking 20 --fit-width 940 --color '#fbf6f3' --at 50%,89 --anchor t --anchor-to ink
pixelox shape path almanac.pxo --in-place --name RibbonTails --d 'M 660 191 L 700 191 L 700 227 L 660 227 L 676 209 Z M 940 191 L 900 191 L 900 227 L 940 227 L 924 209 Z' --fill '#d9782f'
pixelox shape rect almanac.pxo --in-place --name Ribbon --rect 686,181,228x36 --fill '#f7a65e'
pixelox text add almanac.pxo --in-place --name RibbonText --content "2026 EDITION" --font @slab --weight 700 --size 24 --tracking 20 --color '#1e375c' --at 800,199 --anchor c --anchor-to ink
pixelox shape star almanac.pxo --in-place --name Star1 --center 584,203 --points 5 --outer 9 --fill '#fbf6f3'
pixelox shape star almanac.pxo --in-place --name Star2 --center 612,203 --points 5 --outer 9 --fill '#fbf6f3'
pixelox shape star almanac.pxo --in-place --name Star3 --center 640,203 --points 5 --outer 9 --fill '#fbf6f3'
pixelox shape star almanac.pxo --in-place --name Star4 --center 960,203 --points 5 --outer 9 --fill '#fbf6f3'
pixelox shape star almanac.pxo --in-place --name Star5 --center 988,203 --points 5 --outer 9 --fill '#fbf6f3'
pixelox shape star almanac.pxo --in-place --name Star6 --center 1016,203 --points 5 --outer 9 --fill '#fbf6f3'
pixelox text add almanac.pxo --in-place --name Aside --content "where every region counts" --font @brush --size 28 --color '#fbf6f3' --at 1060,186 --anchor l
pixelox render d3 scripts/usmap.js -o bubbles.png --size 1600x1066 --param box=14.5,19.5,72,67 --param bubbles='CA:47,TX:36,FL:27,NY:24,PA:16,IL:16,OH:14,GA:13,NC:13,MI:12,VA:12,AZ:12,NJ:11,WA:11,TN:10,WI:9,CO:9,MA:8,IN:8,AL:8,MO:7,MD:7,MN:7,KY:7,OR:7,SC:6,OK:6,LA:5,UT:5,IA:5,NV:5,MS:5,CT:4,NM:4,NE:4,AR:3,KS:3,ID:3,WV:3,HI:3,NH:3,ME:3,MT:3,RI:3,DE:2,SD:2,ND:2,AK:2,DC:2,VT:2,WY:2' --param empty='#dcd4c4' --param stroke='#1e375c' --param strokewidth=2.5 --param bubblecolor='#fbf6f3' --param bubbleline='#1e375c' --param ink='#1e375c' --param muted='#1e375c' --param halo='#dcd4c4' --param labels=codes --param smalls=0 --param maxr=0.058 --param fs=20 --param font='Rockwell, Roboto Slab, Zilla Slab, Georgia, serif' --param numfont='Rockwell, Roboto Slab, Zilla Slab, Georgia, serif'
pixelox comp place almanac.pxo bubbles.png --in-place --as Bubbles --at 0,0
pixelox shape ellipse almanac.pxo --in-place --name UpWA --rect 393,241,26x26 --fill '#7fb069' --stroke '#fbf6f3' --stroke-width 2
pixelox shape icon almanac.pxo arrow-up --in-place --name UpWAi --size 16 --at 406,254 --anchor c --color '#ffffff' --stroke-width 3
pixelox shape ellipse almanac.pxo --in-place --name UpAZ --rect 491,620,26x26 --fill '#7fb069' --stroke '#fbf6f3' --stroke-width 2
pixelox shape icon almanac.pxo arrow-up --in-place --name UpAZi --size 16 --at 504,633 --anchor c --color '#ffffff' --stroke-width 3
pixelox shape ellipse almanac.pxo --in-place --name UpMN --rect 857,331,26x26 --fill '#7fb069' --stroke '#fbf6f3' --stroke-width 2
pixelox shape icon almanac.pxo arrow-up --in-place --name UpMNi --size 16 --at 870,344 --anchor c --color '#ffffff' --stroke-width 3
pixelox shape ellipse almanac.pxo --in-place --name DownIA --rect 872,445,26x26 --fill '#e86a5f' --stroke '#fbf6f3' --stroke-width 2
pixelox shape icon almanac.pxo arrow-down --in-place --name DownIAi --size 16 --at 885,458 --anchor c --color '#ffffff' --stroke-width 3
pixelox shape ellipse almanac.pxo --in-place --name DownMA --rect 1307,362,26x26 --fill '#e86a5f' --stroke '#fbf6f3' --stroke-width 2
pixelox shape icon almanac.pxo arrow-down --in-place --name DownMAi --size 16 --at 1320,375 --anchor c --color '#ffffff' --stroke-width 3
pixelox text add almanac.pxo --in-place --name Intro --content $'Every region sends delegates to the national council.\n\nSeats are shared out again after each census, in proportion to residents.\n\nArrows mark regions that gained or lost a seat this time.' --font @condensed --weight 500 --size 16 --leading 21 --color '#fbf6f3' --box 170x330 --at 66,262 --anchor tl
pixelox shape path almanac.pxo --in-place --name TabL --d 'M 66 663 L 232 663 L 220 677 L 232 691 L 66 691 Z' --fill '#f7a65e'
pixelox text add almanac.pxo --in-place --name TabLt --content "THE FORMULA" --font @slab --weight 700 --size 17 --tracking 20 --color '#1e375c' --at 76,677 --anchor l --anchor-to ink
pixelox text add almanac.pxo --in-place --name Formula --content "Each region has 1 fixed seat plus seats for its residents. The smallest regions:" --font @condensed --weight 500 --size 16 --leading 21 --color '#fbf6f3' --box 170x100 --at 66,705 --anchor tl
pixelox text add almanac.pxo --in-place --name ExampleN --content $'1\n+ 1\n2' --font @slab --weight 700 --size 17 --leading 24 --align right --color '#fbf6f3' --at 100,828 --anchor r
pixelox text add almanac.pxo --in-place --name ExampleW --content $'fixed seat\nby population\nseats' --font @condensed --weight 500 --size 16 --leading 24 --color '#fbf6f3' --at 110,828 --anchor l
pixelox shape line almanac.pxo --in-place --name ExampleRule --from 66,840 --to 216,840 --stroke '#fbf6f3' --stroke-width 1.5
pixelox shape ellipse almanac.pxo --in-place --name MedianRing --rect 66,946,56x56 --fill none --stroke '#fbf6f3' --stroke-width 1.5 --dash 5,5
pixelox text add almanac.pxo --in-place --name MedianN --content "6" --font @slab --weight 700 --size 34 --color '#fbf6f3' --at 94,974 --anchor c --anchor-to ink
pixelox text add almanac.pxo --in-place --name MedianT --content $'The median number\nof seats.' --font @condensed --weight 500 --size 16 --leading 21 --color '#fbf6f3' --at 136,974 --anchor l
pixelox shape path almanac.pxo --in-place --name TabR --d 'M 1383 592 L 1532 592 L 1520 606 L 1532 620 L 1383 620 Z' --fill '#f7a65e'
pixelox text add almanac.pxo --in-place --name TabRt --content "THE SUM" --font @slab --weight 700 --size 17 --tracking 20 --color '#1e375c' --at 1393,606 --anchor l --anchor-to ink
pixelox text add almanac.pxo --in-place --name SumN --content $'390\n+ 51\n441' --font @slab --weight 700 --size 17 --leading 24 --align right --color '#fbf6f3' --at 1432,668 --anchor r
pixelox text add almanac.pxo --in-place --name SumW --content $'by population\nfixed, 1 each\nseats in total' --font @condensed --weight 500 --size 16 --leading 24 --color '#fbf6f3' --at 1442,668 --anchor l
pixelox shape line almanac.pxo --in-place --name SumRule --from 1383,680 --to 1540,680 --stroke '#fbf6f3' --stroke-width 1.5
pixelox text add almanac.pxo --in-place --name Majority --content "221 seats make a majority" --font @condensed --weight 600 --size 16 --color '#fbf6f3' --at 1383,728 --anchor l
pixelox shape ellipse almanac.pxo --in-place --name KeyUp --rect 1250,731,26x26 --fill '#7fb069' --stroke '#fbf6f3' --stroke-width 2
pixelox shape icon almanac.pxo arrow-up --in-place --name KeyUpi --size 16 --at 1263,744 --anchor c --color '#ffffff' --stroke-width 3
pixelox text add almanac.pxo --in-place --name KeyUpT --content "Gained a seat" --font @condensed --weight 500 --size 16 --color '#fbf6f3' --at 1286,744 --anchor l
pixelox shape ellipse almanac.pxo --in-place --name KeyDown --rect 1250,763,26x26 --fill '#e86a5f' --stroke '#fbf6f3' --stroke-width 2
pixelox shape icon almanac.pxo arrow-down --in-place --name KeyDowni --size 16 --at 1263,776 --anchor c --color '#ffffff' --stroke-width 3
pixelox text add almanac.pxo --in-place --name KeyDownT --content "Lost a seat" --font @condensed --weight 500 --size 16 --color '#fbf6f3' --at 1286,776 --anchor l
pixelox shape line almanac.pxo --in-place --name Lead --from 1192,812 --to 1240,812 --stroke '#fbf6f3' --stroke-width 1.5 --dash 4,4
pixelox text add almanac.pxo --in-place --name KeyNum --content "Number of seats" --font @condensed --weight 500 --size 16 --color '#fbf6f3' --at 1250,812 --anchor l
pixelox text add almanac.pxo --in-place --name KeyNote --content $'A bubble’s area follows\nthe number of seats.' --font @condensed --weight 500 --size 16 --leading 21 --color '#fbf6f3' --at 1250,828 --anchor tl
pixelox shape path almanac.pxo --in-place --name SiteRibbon --d 'M 1216 941 L 1450 941 L 1438 956 L 1450 971 L 1216 971 Z' --fill '#f7a65e'
pixelox shape ellipse almanac.pxo --in-place --name Portrait --rect 1180,934,44x44 --fill '#fbf6f3' --stroke '#1e375c' --stroke-width 2
pixelox shape icon almanac.pxo user --in-place --name PortraitIcon --size 28 --at 1202,956 --anchor c --color '#1e375c'
pixelox text add almanac.pxo --in-place --name Site --content "WWW.EXAMPLE.COM" --font @slab --weight 700 --size 15 --tracking 20 --color '#1e375c' --at 1234,956 --anchor l --anchor-to ink
pixelox text add almanac.pxo --in-place --name Credits --content $'Design: Your Studio\nSource: Example Census Office\nExample data, invented' --font @condensed --size 13 --leading 16 --color '#fbf6f3' --at 1216,979 --anchor tl
pixelox shape rect almanac.pxo --in-place --name CodeCard --rect 1460,918,90x104 --radius 6 --fill '#f7a65e'
pixelox shape qr almanac.pxo --in-place --name Code --data 'www.example.com' --size 74 --at 1505,962 --anchor c --background-color white
pixelox text add almanac.pxo --in-place --name CodeCap --content "Scan for more." --font @condensed --weight 600 --size 13 --color '#1e375c' --at 1505,1010 --anchor c
pixelox export almanac.pxo -o almanac.png
```

Reuse: change `bubbles=` (and `maxr=` when the largest bubble should be larger or smaller).
Then read the anchors again (`lab/anchors.py scripts/usmap.js` with the same `--size` and
params, or the `render d3` line with `--json`): every region logs its bubble as `bx, by,
br`. A change dot goes 0.71 x `br` right of and above the bubble's center (WA: 383,277,32
gives 406,254), and the legend leader starts 6 px right of the sample bubble (FL:
1136,812,50). The map needs pixelox 0.6.0 or later. Keep margin lines under 22 characters:
the left column is 170 px wide.

## 13. Variations and pairings

* **Tile version:** when every region should be equally easy to find, or on a build before
  0.6.0, C-tile-map with `showvalues=1` (see the map spec); `shape=circle` when the values
  are close together and size coding is not needed.
* **Light almanac:** paper `#fbf7ef` as the panel, land `#e6dcc6`, navy type and borders,
  the same orange tabs; bubbles become navy with cream numbers.
* **Other data:** stores, members, or medals per region; any count with a long tail. With
  rates or percentages do not use bubbles: color the regions instead (C-us-choropleth with
  `values=` and `bins=`).
* **Portrait poster:** title on two lines, map in the upper 60%, the margin blocks in a row
  of three under it.
* Pairs with C-bars (the top 10 regions ranked) in the right margin in place of the sum, and
  with C-stat-row for total, median, and majority.

## 14. Pitfalls

* **Bubbles scaled by diameter.** A value 16 times larger must have 16 times the area, that
  is 4 times the diameter. Scaling the diameter by the value makes the leader look 16 times
  too heavy. `bubbles=` scales the areas for you: never resize one bubble by hand, and
  print every number. This is the data-integrity rule for all bubble maps.
* **A floor that flattens the small values.** The script never draws a bubble smaller than
  0.55 x `fs`. With a long tail, check the smallest logged `br` against that floor (here
  14 px against 11); if several values sit on it they look equal, so raise `maxr=` or
  lower `fs=`.
* **Red and green as the only signal.** The gain and loss dots differ by arrow direction as
  well as color; keep the arrows.
* **A choropleth under the bubbles.** Coloring the land by the same value doubles the ink
  and hides the borders the bubbles sit on. One encoding per variable.
* **Big bubbles hiding small neighbors.** The script draws from largest to smallest and
  pushes crowded bubbles apart, but codes can still vanish under a neighbor: lower `maxr=`
  until the north-east reads, or set `labels=none` and add the few codes that matter with
  `text add` under their logged bubbles.
* **Margin text that outgrows the margin.** The side columns are 9 to 10% W wide; more than
  about 60 words per side turns the map into an illustration for an essay.
