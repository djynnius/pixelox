---
id: i035
title: Long banded explainer in navy and violet
kind: [explainer, statistical]
topic: [media, technology, business]
mood: [clean, corporate, friendly, cool]
style: [flat, line-icon]
format: long
structure: S-long-bands
data: [single-stat, comparison, part-to-whole]
charts: [C-section-band, C-columns, C-progress-ring, C-stat-block, C-icon-grid, C-us-choropleth, C-source-line]
palette: ['#f2effd', '#01014f', '#6d3fd6', '#a341b0', '#c9c1ef', '#ffffff']
needs: nothing
---

# i035: Long banded explainer in navy and violet

## 1. Concept

A tall, narrow explainer that tells one topic in five short chapters. Each chapter opens with
a full-width colored band carrying its title and holds exactly one idea: a hero number with a
small bar chart, four share rings, three icon facts, an icon beside a map, one closing
forecast. The message is "this thing has grown, here is how big, who, why it matters, and
where it goes". It works because the bands turn a long scroll into five cards, the grounds
alternate light, white, dark, light, light so no two neighbors look alike, and the whole page
uses one navy, two violets, and one lilac.

## 2. Data story

A mix of single statistics and two small charts; no long series anywhere:

* 1 hero total (a count in the hundreds of millions or low billions).
* 1 comparison of 3 categories with fairly close values, on an axis from 0 to a round
  maximum, 4 or 5 gridlines.
* 4 independent percentages, from under half to most of the group, each shown as its own ring.
* 3 facts, each one number in a sentence (a growth rate, a share, a money amount).
* 2 facts about decline and share (a percentage drop; a share now and before).
* 1 forecast (a money amount and a year).

Placeholder data in the recipe (invented, about podcast listening): 464 million listeners;
apps A, B, C at 180, 150, 95 million; rings 58%, 37%, 81%, 44%; 18% a year; 7 in 10; $2.3
billion; minus 21%; a third, up from 22%; $4.2 billion by 2030.

## 3. Information architecture

1. **Header** (dark): an icon, a two-line title, a one-line deck in small caps.
2. **Chapter 1, "how big"**: left a large line icon over the hero number; right a 3-column
   chart with its unit caption.
3. **Chapter 2, "who"**: four rings in a zig-zag (high, low, high, low), captions in the
   gaps, hairline dividers between the four columns.
4. **Chapter 3, "trends"** (dark): three line icons in three columns, a bold fact under each.
5. **Chapter 4, "effect"**: left an icon and a two-line fact; right a map and a three-line fact.
6. **Chapter 5, "next"**: left a large illustration; right a lead-in and the forecast in the
   largest type of the body.
7. **Footer** (dark): sources.

Every chapter is read left to right, then the band below announces the next. The page can be
cut at any band and each piece still makes sense.

## 4. Format and grid

* Long portrait, 1 : 3.18. Recipe canvas 1080 x 3440. Content margins 9% W; the bands and
  dark grounds bleed to both edges.
* Vertical zones in %H: header 0 to 14.2; band 14.2 to 16.2; chapter 1 to 30.7; band to 32.7;
  chapter 2 to 46; band to 48; chapter 3 to 62.7; band to 64.8; chapter 4 to 78.3; band to
  80.4; chapter 5 to 95.3; footer to 100. Bands are 2% H (70 px) tall; chapters 13 to 15% H.
* Two-column chapters split at 43% W: a narrow left column centered on 23% W and a wide right
  column centered on 71% W. Chapter 2 uses four equal columns centered on 14.5%, 38%, 62%,
  85.5% W; chapter 3 three columns centered on 17%, 50%, 83% W.
* Everything inside a column is centered on the column's axis.

## 5. Layer stack

1. Ground: lavender white `#f2effd` (chapters 1, 4, 5).
2. Header block navy `#01014f`, `0,0,100% x 14.2%`; header icon in lilac `#c8befd`.
3. Five bands, alternating violet `#6d3fd6`, navy, magenta-violet `#a341b0`, violet,
   magenta-violet, each with a centered white title.
4. Chapter 2 ground white; chapter 3 ground navy.
5. Chapter 1: a 200 px navy line icon with a small magenta-violet accent inside it; caption;
   hero number; a 3-column chart (navy, magenta-violet, violet columns with round tops, pale
   gridlines) and its caption.
6. Chapter 2: four rings (outer radius 9.4% W, ring 40% of the radius thick, lilac tracks
   `#c9c1ef`, values alternately navy and magenta-violet, flat ends, the percentage in the
   middle); two-line captions (one regular, one bold line); three hairline dividers `#d9d4f3`.
7. Chapter 3: a small lead-in label, three white 150 px line icons, a caption under each,
   two dividers `#5a5a94`.
8. Chapter 4: a 210 px navy line icon and its two-line fact; a divider; a lilac map; a
   three-line fact with the middle line bold.
9. Chapter 5: a browser window drawn from a lilac rounded box with a 9 px navy outline, a
   title bar line, a globe icon, and a violet-filled cursor; the lead-in; the forecast on two
   lines.
10. Footer navy `0,95.3%,100% x 4.7%` with two centered lines.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#f2effd` | ground of the light chapters | 38% | nothing |
| `#01014f` | header, chapter 3, footer, one band; all dark type and icons; chart color 1 | 30% | first category; otherwise structure |
| `#ffffff` | chapter 2 ground; type on dark | 13% | nothing |
| `#6d3fd6` | two bands; chart color 3; cursor fill | 6% | third category |
| `#a341b0` | two bands; chart color 2; every second ring | 6% | second category |
| `#c9c1ef` | ring tracks, map, window fill, dividers (`#d9d4f3`) | 7% | "the rest" in rings; neutral elsewhere |

Analogous: one hue family from navy through violet to lilac, no warm color at all. The three
chart colors are the three darkest steps. Light chapters use dark type; dark chapters use
white type and white icons.

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Title | 2 lines, 3 words each | title | geometric sans (Poppins, Montserrat / Avenir Next) | 800 | 62 | 0 | line pitch 78 px | center | `#ffffff` | 50%, cap tops 6.6% and 8.9% |
| Deck | 9 to 12 words, one line | caps | geometric sans | 500 | 20 | 40 | - | center | `#c8befd` | 50%, 11.9% |
| Band title (x5) | 3 to 6 words | title | geometric sans | 700 | 28 | 0 | - | center | `#ffffff` | band centers |
| Hero caption | 3 to 5 words on 2 lines | sentence | geometric sans | 400 | 22 | 0 | 130% | center | `#01014f` | 23%, 24.7% |
| Hero number | number and unit word | lower | geometric sans | 700 | 46 | 0 | - | center | `#01014f` | 23%, 27% |
| Chart labels | axis numbers, 3 category names | - | geometric sans | 400 | 20 | 0 | - | - | `#01014f` | in the chart |
| Chart caption | 3 to 4 words with the unit | sentence | geometric sans | 700 | 20 | 0 | - | center | `#01014f` | 69.4%, 29.1% |
| Ring number (x4) | NN% | - | geometric sans | 400 | 46 | 0 | - | center | the ring's color | ring centers |
| Ring caption (x4) | 2 lines of 2 to 3 words, one bold | sentence | geometric sans | 400 / 700 | 21 | 0 | 32 px pitch | center | `#01014f` | under the high rings, above the low ones |
| Lead-in label | 2 words and a colon | sentence | geometric sans | 400 | 20 | 0 | - | left | `#ffffff` | 10%, 50.1% |
| Fact caption (x3) | 4 to 9 words on 2 or 3 lines | sentence | geometric sans | 700 or 400 | 21 | 0 | 135% | center | `#ffffff` | column centers, 58.4% |
| Chapter 4 facts | 2 and 3 lines, one bold line each | sentence | geometric sans | 400 / 700 | 21 | 0 | 32 px pitch | center | `#01014f` | 21.9% and 71.5% W, 74.2% H |
| Forecast lead-in | 9 to 11 words on 3 lines | sentence | geometric sans | 400 | 21 | 0 | 135% | center | `#01014f` | 71.5%, 84.9% |
| Forecast | amount, then "by" and a year | lower | geometric sans | 700 | 56 | 0 | line pitch 72 px | center | `#01014f` | 71.5%, cap tops 88.7% and 90.8% |
| Sources | "Sources:" bold, then one line | sentence | geometric sans | 700 / 400 | 19 | 0 | - | center | `#ffffff` | 50%, 96.7% and 97.7% |

One family, three weights. Sizes 62 : 56 : 46 : 28 : 21, with body text at 21 px throughout:
only numbers and titles ever get bigger.

## 8. Charts and diagrams

* **C-columns** (chapter 1): `scripts/bars.js`, box `45,17.4,49,10.8`, 3 values, one color
  per column (navy, magenta-violet, violet), `max=200 ticks=4`, `vlabels=0`, `radius=14`
  (round tops), gridlines `#d9d4f3`, axis numbers in navy. The original has no category
  names under the columns: a brand mark stands on each column and a three-key legend sits
  under the caption. The recipe writes neutral names under the columns instead.
* **C-progress-ring** x4 (chapter 2): `scripts/donut.js`, `kind=ring`, placed by center
  (`cx`, `cy`, `r=9.4`), `inner=0.6`, `round=0`, `trackcolor='#c9c1ef'`, the number in the
  ring's own color at 46 px, regular weight. Rings 1 and 3 sit 5% H higher than rings 2 and 4.
* **C-us-choropleth** (chapter 4), used as a plain picture: `scripts/usmap.js` run with
  `render d3`, no `groups=` or `values=`, so every state takes `empty='#c9c1ef'`; borders in
  the ground color (`stroke='#f2effd'`, 1.5 px); `labels=none legend=none`; box
  `47,65.6,48,8.9`. Map spec: US states (lower 48 with Alaska and Hawaii as insets), Albers
  USA, no encoding, no legend, no labels: the map only says "in this country". On a build
  before 0.6.0 use `scripts/tilemap.js` with `empty='#c9c1ef' codes=0 legend=none` in the
  same box.
* **C-section-band**, **C-stat-block**, **C-icon-grid**, **C-source-line** for the rest.

## 9. Icons and illustration

All line icons, round caps, in the type color of their ground:

* Header: `headphones`, 112 px, lilac, 7 px line (the original: a monitor with a phone and a
  tablet, as flat lilac shapes).
* Chapter 1: `smartphone`, 200 px, navy, 9 px line, with `audio-lines` (70 px, magenta-violet)
  inside it (the original: a television showing a grid of colored tiles, with sparkles).
* Chapter 3: `trending-up`, `car`, `chart-column`, 150 px, white, 6 px line (the original:
  three small scenes drawn in white outline: a picture with an eye, a laptop with a
  thumbs-up, a presentation board).
* Chapter 4: `radio`, 210 px, navy (the original: a television with a remote control).
* Chapter 5: a browser window built from `shape rect` and `shape line`, `globe` 150 px inside
  it, `mouse-pointer-2` 130 px with a violet fill overlapping its lower right corner.

Icon line weight grows with size (about 1/20 of the icon size) so large icons do not look
thin.

## 10. Effects and finish

None: flat color, no shadows, no gradients, no texture. Rounded column tops (14 px) and the
22 px radius of the browser window are the only soft corners.

## 11. Hierarchy and eye path

1. The title on the dark header.
2. The first band, then the hero number (the first large dark type on a light ground).
3. Band by band down the page: each band is a stop, each chapter one glance.
4. The four ring numbers; the three white icons; the map.
5. The forecast: the largest type in the body, placed last so the page ends on its point.

Type steps: title 62, forecast 56, hero and ring numbers 46, band titles 28, body 21.

## 12. Build recipe

Nothing to supply. Canvas 1080 x 3440. All figures are example data.

```bash
pixelox new -o story.pxo --size 1080x3440 --fill '#f2effd'
pixelox shape rect story.pxo --in-place --name Head --rect 0,0,1080x489 --fill '#01014f'
pixelox shape icon story.pxo headphones --in-place --name HeadIcon --size 112 --at 540,124 --anchor c --color '#c8befd' --stroke-width 7
pixelox text add story.pxo --in-place --name T1 --content "The Rise of" --font @geometric-sans --weight 800 --size 62 --color '#ffffff' --at 50%,228 --anchor t --anchor-to ink
pixelox text add story.pxo --in-place --name T2 --content "Podcast Listening" --font @geometric-sans --weight 800 --size 62 --color '#ffffff' --at 50%,306 --anchor t --anchor-to ink
pixelox text add story.pxo --in-place --name Deck --content "HOW SPOKEN AUDIO IS CHANGING THE WAY WE SPEND AN HOUR" --font @geometric-sans --weight 500 --size 20 --tracking 40 --color '#c8befd' --at 50%,410 --anchor t --anchor-to ink
pixelox shape rect story.pxo --in-place --name Band1 --rect 0,489,1080x70 --fill '#6d3fd6'
pixelox text add story.pxo --in-place --name Band1T --content "Where People Listen" --font @geometric-sans --weight 700 --size 28 --color '#ffffff' --at 50%,524 --anchor c
pixelox shape icon story.pxo smartphone --in-place --name M1Icon --size 200 --at 250,706 --anchor c --color '#01014f' --stroke-width 9
pixelox shape icon story.pxo audio-lines --in-place --name M1Wave --size 70 --at 250,700 --anchor c --color '#a341b0' --stroke-width 7
pixelox text add story.pxo --in-place --name M1Cap --content $'Monthly listeners\nworldwide:' --font @geometric-sans --size 22 --leading 130% --align center --color '#01014f' --at 250,848 --anchor t
pixelox text add story.pxo --in-place --name M1Num --content "464 million" --font @geometric-sans --weight 700 --size 46 --color '#01014f' --at 250,928 --anchor t --anchor-to ink
pixelox render js scripts/bars.js -o bars.png --size 1080x3440 --param box=45,17.4,49,10.8 --param values=180,150,95 --param labels='App A,App B,App C' --param colors='#01014f,#a341b0,#6d3fd6' --param max=200 --param ticks=4 --param vlabels=0 --param radius=14 --param gap=0.3 --param ink='#01014f' --param muted='#01014f' --param grid='#d9d4f3' --param fs=20 --param font='Avenir Next, Poppins, Montserrat, Century Gothic, Arial, sans-serif'
pixelox comp place story.pxo bars.png --in-place --as Bars --at 0,0
pixelox text add story.pxo --in-place --name BarsCap --content "Listeners worldwide, millions" --font @geometric-sans --weight 700 --size 20 --color '#01014f' --at 750,1000 --anchor c
pixelox shape rect story.pxo --in-place --name Band2 --rect 0,1056,1080x70 --fill '#01014f'
pixelox text add story.pxo --in-place --name Band2T --content "Who Is Listening" --font @geometric-sans --weight 700 --size 28 --color '#ffffff' --at 50%,1091 --anchor c
pixelox shape rect story.pxo --in-place --name White2 --rect 0,1126,1080x456 --fill '#ffffff'
pixelox render js scripts/donut.js -o ring1.png --size 1080x3440 --param kind=ring --param cx=14.5 --param cy=36.4 --param r=9.4 --param inner=0.6 --param round=0 --param value=58 --param colors='#01014f' --param trackcolor='#c9c1ef' --param ink='#01014f' --param numweight=400 --param centersize=46 --param font='Avenir Next, Poppins, Montserrat, Century Gothic, Arial, sans-serif'
pixelox render js scripts/donut.js -o ring2.png --size 1080x3440 --param kind=ring --param cx=38 --param cy=41.4 --param r=9.4 --param inner=0.6 --param round=0 --param value=37 --param colors='#a341b0' --param trackcolor='#c9c1ef' --param ink='#a341b0' --param numweight=400 --param centersize=46 --param font='Avenir Next, Poppins, Montserrat, Century Gothic, Arial, sans-serif'
pixelox render js scripts/donut.js -o ring3.png --size 1080x3440 --param kind=ring --param cx=62 --param cy=36.4 --param r=9.4 --param inner=0.6 --param round=0 --param value=81 --param colors='#01014f' --param trackcolor='#c9c1ef' --param ink='#01014f' --param numweight=400 --param centersize=46 --param font='Avenir Next, Poppins, Montserrat, Century Gothic, Arial, sans-serif'
pixelox render js scripts/donut.js -o ring4.png --size 1080x3440 --param kind=ring --param cx=85.5 --param cy=41.4 --param r=9.4 --param inner=0.6 --param round=0 --param value=44 --param colors='#a341b0' --param trackcolor='#c9c1ef' --param ink='#a341b0' --param numweight=400 --param centersize=46 --param font='Avenir Next, Poppins, Montserrat, Century Gothic, Arial, sans-serif'
pixelox comp place story.pxo ring1.png --in-place --as Ring1 --at 0,0
pixelox comp place story.pxo ring2.png --in-place --as Ring2 --at 0,0
pixelox comp place story.pxo ring3.png --in-place --as Ring3 --at 0,0
pixelox comp place story.pxo ring4.png --in-place --as Ring4 --at 0,0
pixelox text add story.pxo --in-place --name R1a --content "of adults" --font @geometric-sans --size 21 --color '#01014f' --at 157,1392 --anchor t
pixelox text add story.pxo --in-place --name R1b --content "listen every month" --font @geometric-sans --weight 700 --size 21 --color '#01014f' --at 157,1424 --anchor t
pixelox text add story.pxo --in-place --name R2a --content "listeners who use" --font @geometric-sans --size 21 --color '#01014f' --at 410,1226 --anchor t
pixelox text add story.pxo --in-place --name R2b --content "two or more apps" --font @geometric-sans --weight 700 --size 21 --color '#01014f' --at 410,1258 --anchor t
pixelox text add story.pxo --in-place --name R3a --content "finish most" --font @geometric-sans --weight 700 --size 21 --color '#01014f' --at 670,1392 --anchor t
pixelox text add story.pxo --in-place --name R3b --content "episodes they start" --font @geometric-sans --size 21 --color '#01014f' --at 670,1424 --anchor t
pixelox text add story.pxo --in-place --name R4a --content "listeners" --font @geometric-sans --size 21 --color '#01014f' --at 923,1226 --anchor t
pixelox text add story.pxo --in-place --name R4b --content "under 35" --font @geometric-sans --weight 700 --size 21 --color '#01014f' --at 923,1258 --anchor t
pixelox shape path story.pxo --in-place --name Div2 --d 'M 283 1166 L 283 1542 M 540 1166 L 540 1542 M 797 1166 L 797 1542' --fill none --stroke '#d9d4f3' --stroke-width 2
pixelox shape rect story.pxo --in-place --name Band3 --rect 0,1582,1080x70 --fill '#a341b0'
pixelox text add story.pxo --in-place --name Band3T --content "Three Trends" --font @geometric-sans --weight 700 --size 28 --color '#ffffff' --at 50%,1617 --anchor c
pixelox shape rect story.pxo --in-place --name Dark3 --rect 0,1652,1080x505 --fill '#01014f'
pixelox text add story.pxo --in-place --name M3Lead --content "Expected growth:" --font @geometric-sans --size 20 --color '#ffffff' --at 108,1722 --anchor l
pixelox shape icon story.pxo trending-up --in-place --name M3I1 --size 150 --at 184,1872 --anchor c --color '#ffffff' --stroke-width 6
pixelox shape icon story.pxo car --in-place --name M3I2 --size 150 --at 540,1872 --anchor c --color '#ffffff' --stroke-width 6
pixelox shape icon story.pxo chart-column --in-place --name M3I3 --size 150 --at 896,1872 --anchor c --color '#ffffff' --stroke-width 6
pixelox text add story.pxo --in-place --name M3a --content $'18% a year\nuntil 2030' --font @geometric-sans --weight 700 --size 21 --leading 135% --align center --color '#ffffff' --at 184,2010 --anchor t
pixelox text add story.pxo --in-place --name M3b --content $'7 in 10 people listen\nwhile they drive\nor commute' --font @geometric-sans --size 21 --leading 135% --align center --color '#ffffff' --at 540,2010 --anchor t
pixelox text add story.pxo --in-place --name M3c --content $'Ad revenue:\n$2.3 billion' --font @geometric-sans --weight 700 --size 21 --leading 135% --align center --color '#ffffff' --at 896,2010 --anchor t
pixelox shape path story.pxo --in-place --name Div3 --d 'M 362 1740 L 362 2100 M 718 1740 L 718 2100' --fill none --stroke '#5a5a94' --stroke-width 2
pixelox shape rect story.pxo --in-place --name Band4 --rect 0,2157,1080x70 --fill '#6d3fd6'
pixelox text add story.pxo --in-place --name Band4T --content "What It Means for Radio" --font @geometric-sans --weight 700 --size 28 --color '#ffffff' --at 50%,2192 --anchor c
pixelox shape icon story.pxo radio --in-place --name M4Icon --size 210 --at 236,2400 --anchor c --color '#01014f' --stroke-width 8
pixelox text add story.pxo --in-place --name M4a --content "Live radio listening" --font @geometric-sans --size 21 --color '#01014f' --at 236,2552 --anchor t
pixelox text add story.pxo --in-place --name M4b --content "fell by 21%" --font @geometric-sans --weight 700 --size 21 --color '#01014f' --at 236,2584 --anchor t
pixelox shape line story.pxo --in-place --name Div4 --from 468,2290 --to 468,2640 --stroke '#c9c1ef' --stroke-width 2
pixelox render d3 scripts/usmap.js -o map.png --size 1080x3440 --param box=47,65.6,48,8.9 --param empty='#c9c1ef' --param stroke='#f2effd' --param strokewidth=1.5 --param labels=none --param legend=none
pixelox comp place story.pxo map.png --in-place --as Map --at 0,0
pixelox text add story.pxo --in-place --name M4c --content "Podcasts now take" --font @geometric-sans --size 21 --color '#01014f' --at 772,2566 --anchor t
pixelox text add story.pxo --in-place --name M4d --content "a third of all audio time," --font @geometric-sans --weight 700 --size 21 --color '#01014f' --at 772,2598 --anchor t
pixelox text add story.pxo --in-place --name M4e --content "up from 22% five years ago" --font @geometric-sans --size 21 --color '#01014f' --at 772,2630 --anchor t
pixelox shape rect story.pxo --in-place --name Band5 --rect 0,2694,1080x70 --fill '#a341b0'
pixelox text add story.pxo --in-place --name Band5T --content "Where It Goes Next" --font @geometric-sans --weight 700 --size 28 --color '#ffffff' --at 50%,2729 --anchor c
pixelox shape rect story.pxo --in-place --name Win --rect 100,2866,400x300 --radius 22 --fill '#c9c1ef' --stroke '#01014f' --stroke-width 9
pixelox shape line story.pxo --in-place --name WinBar --from 100,2932 --to 500,2932 --stroke '#01014f' --stroke-width 9
pixelox shape icon story.pxo globe --in-place --name WinGlobe --size 150 --at 300,3052 --anchor c --color '#01014f' --stroke-width 8
pixelox shape icon story.pxo mouse-pointer-2 --in-place --name Cursor --size 130 --at 470,3170 --anchor c --color '#01014f' --fill '#6d3fd6' --stroke-width 8
pixelox text add story.pxo --in-place --name M5a --content $'The world market for\npodcast advertising\nis expected to reach' --font @geometric-sans --size 21 --leading 135% --align center --color '#01014f' --at 772,2920 --anchor t
pixelox text add story.pxo --in-place --name M5b --content '$4.2 billion' --font @geometric-sans --weight 700 --size 56 --color '#01014f' --at 772,3050 --anchor t --anchor-to ink
pixelox text add story.pxo --in-place --name M5c --content "by 2030" --font @geometric-sans --weight 700 --size 56 --color '#01014f' --at 772,3122 --anchor t --anchor-to ink
pixelox shape rect story.pxo --in-place --name Foot --rect 0,3278,1080x162 --fill '#01014f'
pixelox text add story.pxo --in-place --name Src1 --content "Sources:" --font @geometric-sans --weight 700 --size 19 --color '#ffffff' --at 50%,3326 --anchor t
pixelox text add story.pxo --in-place --name Src2 --content "Example data, invented for this layout. Replace with your own figures and their sources." --font @geometric-sans --size 19 --color '#ffffff' --at 50%,3362 --anchor t
pixelox export story.pxo -o story.png
```

Reuse: keep the zone heights and change the content chapter by chapter. Swap any chapter's
module for another one-idea module of the same height (C-waffle, C-bars with up to 5 bars,
C-process-row, C-stat-row). The rings are placed by center: change `value=` only. If a
chapter needs more than three lines of text, the idea is too big for this format: split it
into two chapters.

## 13. Variations and pairings

* **Shorter:** drop chapters 3 and 4 for a 1 : 2 page (1080 x 2160) with header, three
  chapters, footer.
* **Other palettes:** any one-hue family works: forest `#0b3d2e` with `#2e8b57`, `#7fc8a9`,
  `#e9f5ef`; or brick `#5a1e12` with `#c2452d`, `#f0a58f`, `#fbeee9`. Keep three chart colors
  and one tint for tracks.
* **Report page:** the same bands at A4 (1080 x 1528) hold three chapters; use two columns
  inside every chapter.
* **Square carousel:** each band plus its chapter is one 1080 x 1080 slide; repeat the header
  icon small in a corner.
* Pairs with C-timeline as an extra chapter ("how we got here") and C-ranked-list for a top 5.

## 14. Pitfalls

* **Two ideas in one chapter.** The format's strength is one glance per band. A chart, a
  number, and a paragraph together make a chapter read like a slide. Cut or split.
* **Rings that look like one chart.** Four rings in a row invite adding them up. They are
  separate questions: caption each with its own base ("of adults", "of listeners") and never
  give them a shared legend.
* **A truncated column axis.** When the values are close it is tempting to start the axis
  high to dramatize the gap; `scripts/bars.js` always starts at zero. Keep it, and
  let the caption say the difference in words.
* **A map with nothing on it that looks like data.** Keep a decorative map in one flat tint;
  the moment regions differ in color, readers look for a legend.
* **Thin icons at large sizes.** A 200 px Lucide icon at the default line weight looks like
  a wireframe; set `--stroke-width` to about 1/20 of the size.
* **Mixed weights faked in one line.** One text layer has one weight: the bold line of each
  caption is its own layer, 32 px under the regular line.
