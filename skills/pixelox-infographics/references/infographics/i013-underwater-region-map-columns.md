---
id: i013
title: Underwater long sheet with a region map and columns
kind: [explainer, geographic]
topic: [health, society, sports]
mood: [cool, friendly, bold, serious]
style: [illustrated, gradient, flat]
format: long
structure: S-long-flow
data: [geography, comparison, single-stat]
charts: [C-us-choropleth, C-progress-ring, C-columns, C-title-block, C-source-line]
palette: ['#a4d6f3', '#075ea3', '#1874b3', '#fd6100', '#ef8f01', '#0083b3', '#ffffff']
needs: nothing
---

# i013: Underwater long sheet with a region map and columns

## 1. Concept

A tall safety-campaign sheet that sinks as you read it: pale sky and a waterline at the top,
then the page darkens from light blue to deep blue as the content goes down, with bubbles
rising and an empty bottle drifting to the bottom. Three things are said in three stops: a
huge two-word slogan with one paragraph, a map of five regions each stamped with a
percentage inside a lifebuoy, and a small chart that gives the counts behind those
percentages; a second slogan closes the page. It works because the ground itself tells the
story (going under), the lifebuoys make each number feel like a rescue, and the same five
region colors link the map to the columns.

## 2. Data story

* **One opening statistic** inside the intro paragraph (a share of about one fifth).
* **Geography, 5 regions:** the country's states grouped into five named regions; one
  percentage per region. One region is clearly highest (about a third), the other four lie
  between about 15% and 25%. The percentages are **not** parts of one whole: each is "the share
  within that region".
* **Comparison, 5 regions x 2 counts:** for each region the total (from under 100 to about
  200, one region far ahead) and the subset behind the percentage (from under 20 to the low
  40s). The subset is always a seventh to a good third of its total, so every pair is a
  tall column beside a short one.
* A closing slogan of 6 to 8 words and a source block of 2 entries.

Placeholder data used in the recipe (example data, invented: paddlers counted at launch
sites, and those wearing a life jacket):

| Region | Lakes | North-east | Coast | South | Mountain |
| --- | --- | --- | --- | --- | --- |
| Share wearing a jacket | 37% | 24% | 15% | 23% | 20% |
| Paddlers counted | 119 | 84 | 96 | 180 | 120 |
| Wearing a jacket | 44 | 20 | 14 | 41 | 24 |

The shares are the second row divided by the first: the map and the chart agree.

## 3. Information architecture

1. **Slogan** (two words, the full width) and a four-line intro with the key statistic.
2. **Surface scene:** a wave, a small boat at the left, a cluster of bubbles on the crest, a
   bottle sinking at the right, bubble trails leading the eye down.
3. **Module 1, the map:** a three-line caps heading at the left, then the regions with one
   lifebuoy badge each; the largest badge marks the highest share.
4. A white hairline.
5. **Module 2, the counts:** a three-line caps heading, then five pairs of columns with
   their values on top and the region names underneath.
6. A white hairline.
7. **Closing slogan**, centered, two lines.
8. **Sources** at the bottom left, publisher mark at the bottom right.

The reader is led by depth: each module sits on a darker blue than the one before, and the
hairlines mark the stops.

## 4. Format and grid

* Long portrait, 1 : 2.47. Recipe canvas 1080 x 2670. Side margins 6.3% W.
* Slogan: 6.3% to 93% W, cap top 2.5% H, cap height 4.8% H. Intro `6.3%,8.7%,86% x 4.5%`.
* Surface scene: 12% to 33% H; waterline from 19.4% H at the left edge over a crest at 16%
  H (53% W) to 17.5% H at the right.
* Heading 1 at 6.6% W, 27.2% to 31% H. Map `6%,33.5%,77% x 23.5%` in the original (the
  mainland from 6% to 83% W, the two insets under its south-west). Recipe box
  `4.5%,33.5%,88% x 23.5%`: the script's map frame is a little wider than the land.
* Badges (center x %W, y %H, radius %W) in the original: `12, 44.1, 7.8`; `40.2, 44.1, 8.4`;
  `61.4, 38.9, 11.9`; `88.2, 39.8, 9.3`; `69.1, 50.6, 8.8`. The right-hand one hangs half
  off the land. The recipe centers each badge on the logged center of one state of its
  region: `11.9, 44.6`; `34.4, 44.7`; `58.2, 40.2`; `85.5, 40.4`; `70.9, 49.7`.
* Hairline 1 at 59.9% H, 6.5% to 93.3% W.
* Heading 2 at 6.6% W, 62.9% to 66% H. Columns `14%,65%,73% x 12.4%`, baseline at 77.4% H;
  names at 78.6% H.
* Hairline 2 at 80.4% H (81% in the recipe, to make room for a key line).
* Closing slogan centered, lines at 85.2% and 88.1% H. Sources at 6.6% W, 94.3% H; mark
  right-aligned to 93% W at 95% H.

## 5. Layer stack

1. Ground: a vertical gradient: sky `#a4d6f3` at the top, almost white `#d9f0fb` at 20% H
   (the light on the surface), `#a9dcf4` at 30%, `#7bc1e5` at 50%, `#3892c7` at 72%, deep
   `#075ea3` at the bottom.
2. Bubble texture: small soft white discs scattered over the whole height, 20 to 50% opaque.
3. Waterline: a dark blue curve `#1874b3`, 3 px, with a white highlight above it.
4. A cluster of five glossy bubbles on the crest (mid blue with white rims), 18 to 46 px in
   radius.
5. Boat: a small white motor yacht, 20% W wide, sitting on the wave at the left (a photo
   cut-out in the original).
6. Bottle: an empty clear glass bottle, 30% W tall, tilted 15 degrees, sinking at the
   right; two more bottles drift at the upper right of the chart.
7. Slogan in blue `#1478bd`; intro in the same blue.
8. Heading 1 in white.
9. Map: the real outline of the states, five regions in teal-blue `#0083b3`, orange
   `#fd6100`, pale blue `#a7d6ec`, amber `#ef8f01`, and pale gray `#e1e2e4`; thin darker
   borders between states (1 px in the water color `#8fcbea` in the recipe).
10. Five lifebuoy badges: a white ring crossed by four orange bands `#f26b20` (at 12, 3, 6,
    and 9 o'clock), a soft shadow, a disc in a tint of the region's color in the middle, the
    percentage in navy `#1d3f7a`; the region's name under the badge in white.
11. Hairline; heading 2; five column pairs in the regions' colors (the short column in a
    lighter tint), each column with a dark blue offset shadow at its right; values in white
    above; names in white caps below.
12. Hairline; closing slogan in white.
13. Large dark bubbles in the bottom 12% of the page; sources; publisher mark.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#a4d6f3` to `#075ea3` | ground gradient | 80% | nothing: "deeper" as you read |
| `#1874b3` | slogan, intro, waterline | 6% | nothing |
| `#ffffff` | headings, values, closing slogan, badge rings | 4% | nothing |
| `#0083b3` | region 1 on the map and in the chart (tint `#18b4f0`) | 2% | Coast |
| `#fd6100` | region 2 (tint `#ff7a1a`); badge bands `#f26b20` | 3% | Mountain |
| `#a7d6ec` | region 3 (chart `#5aa9d6`, tint `#a9e4ff`) | 2% | Lakes |
| `#ef8f01` | region 4 (chart `#d47f00`, tint `#fd9700`) | 2% | South |
| `#e1e2e4` | region 5 (chart `#d1eaf5`, tint `#eef6fb`) | 1% | North-east |

Categorical on a sequential ground: five region colors, two warm and three cool, repeated in
the chart with a lighter tint for the subset. Orange is the accent (the lifebuoy bands and
the two warm regions) against an all-blue page.

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Slogan | 2 words, 8 to 9 characters | caps | heavy grotesque (Archivo Black, Inter 900 / Arial Black) | 900 | fitted to 86.7% W: about 175 | 0 | - | left | `#1478bd` | 6.3%, cap top 2.5% |
| Intro | 35 to 45 words on 4 lines, one statistic, one footnote mark | sentence | neo-grotesque (Inter, Roboto / Helvetica Neue) | 500 | 30 | 10 | 37 px | left | `#1478bd` | 6.3%, 8.7% |
| Module heading (x2) | 9 to 11 words on 3 lines | caps | neo-grotesque | 700 | 29 | 10 | 39 px | left | `#ffffff` | 6.6%, 27.2% and 62.9% |
| Badge percentage (x5) | NN% | - | neo-grotesque | 700 | 40 to 62, about 0.48 x the badge radius | 0 | - | center | `#1d3f7a` | badge centers |
| Region name on the map (x5) | 1 word | title | neo-grotesque | 500 | 18 | 0 | - | center | `#ffffff` (navy inside the badge in the recipe) | under each badge |
| Column value (x10) | 2 to 3 digits | - | neo-grotesque | 400 | 29 | 0 | - | center | `#ffffff` | above each column |
| Column name (x5) | 1 word | caps | condensed, light (Barlow Condensed, Oswald light / Avenir Next Condensed) | 400 | 24 | 20 | - | center | `#ffffff` | 78.6% H under each pair |
| Bar captions (the original) | 2 words, turned 90 degrees beside each column | title | condensed | 400 | 10 | 0 | - | - | `#ffffff` | left of each column |
| Closing slogan | 6 to 8 words on 2 lines | sentence | geometric sans (Montserrat, Poppins / Avenir Next) | 700 | 58 | 0 | 76 px | center | `#ffffff` | 50%, 85.2% and 88.1% |
| Sources | "Sources:" bold, then 2 or 3 lines | sentence | neo-grotesque | 700 / 400 | 15 | 0 | 19 px | left | `#ffffff` | 6.6%, 94.3% |

Three families: a black grotesque for the slogan, a neutral grotesque for text and numbers,
and a rounder geometric bold for the closing line. Sizes 175 : 58 : 30 : 29, about 6 : 2 : 1
: 1. The slogan is far outside the usual title range: this is a poster first and a
chart second.

## 8. Charts and diagrams

* **C-us-choropleth** as the region map: `scripts/usmap.js` (run with `render d3`) with
  `groups=` (every state gets its region number 1 to 5), `colors=` the five region colors,
  `labels=none`, `stroke='#8fcbea' strokewidth=1` (the water color at that height), box
  `4.5,33.5,88,23.5`. No `names=`, so no legend is drawn: the badges label the regions.
* **Lifebuoy badges** are built from **C-progress-ring**'s script used as a donut:
  `scripts/donut.js` by center with eight slices `values=3,1,3,1,3,1,3,1` (white, orange,
  ...), `start=11.25` so the four orange bands sit at 3, 6, 9, and 12 o'clock, `inner=0.62`,
  `labelpos=none sep=none`. A tinted disc (0.64 x the radius) and the percentage go on top.
  **Badge areas follow the values:** radius = 21.6 px x square root of the percentage (37%
  gives 131 px, 15% gives 84 px). Each badge is centered on a state anchor that the map
  script logs (`cx=` and `cy=` are the anchor's `xp` and `yp`): one state near the middle
  of each region, chosen so the five badges do not touch.
* **C-columns** x5, one small `scripts/bars.js` chart per region so each pair can take its
  region's colors: two series of one value each (`values='total;subset'`), `colors=` the
  region color and its tint, **the same `max=200` in all five** so the columns share one scale,
  `axis=0 gridlines=0`, `gap=0.3 radius=0`, `fs=29`, `ink='#ffffff'`. Boxes are 13% W wide
  from 13.5% W in steps of 14.6% W, all at `y=65.5 h=12`.
* The offset shadow behind the columns is `fx drop-shadow` (offset 7,0, no blur) on each
  placed chart.
* Simplified: the original's rotated captions beside every column are replaced by one key
  line under the chart; the region names sit inside the badges instead of under them.

**Map spec:**

* Region set: the 50 US states grouped into 5 regions; the two non-contiguous states as
  insets at the lower left (same color as the west coast region).
* Projection: Albers USA, map box `4.5%,33.5%,88% x 23.5%`.
* Encoding: fill by region (5 classes, hexes in section 6). State borders inside a region:
  1 px in a darker shade of the region's color in the original, one pale water color for
  all borders in the recipe; no heavier line between regions (the color change is the
  border). The percentages are carried by the badges, not by the fill.
* Badges: one per region on the center of a state near the region's middle (California,
  Colorado, Wisconsin, Massachusetts, and Georgia here), radius = 2% W x square root of the
  percentage; white ring, four orange bands, tinted center, percentage in navy; soft shadow
  (offset 0,6, blur 14, 30%).
* Labels: the region's name in white, 18 px, centered under each badge in the original (in
  navy inside the badge in the recipe); no state names.
* Legend: none (direct labels).
* Fallback for builds before 0.6.0: C-tile-map, `scripts/tilemap.js` (run with `render js`)
  with the same `groups=` and `colors=`, `codes=0 gap=0.06 legend=none`, and the badges on
  its tile anchors.

## 9. Icons and illustration

* Boat: `ship`, 170 px, white, 9 px line, on the wave at the left. The original is a
  cut-out photo of a white motor yacht; place your own `illustration.png` 215 px wide there.
* Bottle: `bottle-wine`, 300 px, pale `#eaf6fd`, 12 px line, at the right under the
  waterline; a second one, 170 px, right of the chart heading. The original bottles are
  transparent glass renderings, tilted.
* Bubbles: five `shape ellipse` discs with white rims on the crest; the texture is
  `scripts/blobs.js` with small soft discs.
* Lifebuoys are charts (section 8). The lucide `life-buoy` icon works for small versions.

## 10. Effects and finish

* The ground gradient (`fill gradient` with six stops) is the main effect.
* Soft shadows under the badges (`fx drop-shadow`, offset 0,6, blur 14, navy, 30%), copied
  to all five with `fx copy`.
* Hard offset shadows on the columns (offset 7,0, blur 0, `#0d5a9e`, 70%): a flat way to
  get the original's raised bars.
* The original also has a white glow behind the slogan and glossy highlights on bubbles and
  bottles; the recipe leaves those out.

## 11. Hierarchy and eye path

1. The slogan (175 px, the darkest thing on the pale top).
2. The largest lifebuoy and its percentage, then the other four.
3. The tallest column pair, then the chart's other pairs.
4. The closing slogan (white on the deepest blue).
5. Intro, headings, sources.

Slogan 175 px; badge numbers 62 to 40; closing line 58; headings 29; text 30.

## 12. Build recipe

Nothing to supply. Canvas 1080 x 2670. All figures are example data.

```bash
pixelox new -o deep.pxo --size 1080x2670 --fill '#a4d6f3'
pixelox layer add deep.pxo --in-place --name Water
pixelox fill gradient deep.pxo --in-place --layer Water --kind linear --from 0,0 --to 0,100% --stops '#a4d6f3@0,#d9f0fb@0.2,#a9dcf4@0.3,#7bc1e5@0.5,#3892c7@0.72,#075ea3@1'
pixelox render js scripts/blobs.js -o bubbles.png --size 1080x2670 --seed 7 --param colors='#ffffff,#d6eefb' --param bg=transparent --param count=46 --param size=0.022 --param vary=0.7 --param soft=0.25 --param alpha=0.35
pixelox comp place deep.pxo bubbles.png --in-place --as Bubbles --at 0,0
pixelox shape ellipse deep.pxo --in-place --name Deep1 --rect 60,2350,420x420 --fill '#0b62a6' --opacity 35
pixelox shape ellipse deep.pxo --in-place --name Deep2 --rect 420,2420,330x330 --fill '#0b62a6' --opacity 30
pixelox shape ellipse deep.pxo --in-place --name Deep3 --rect 700,2330,460x460 --fill '#0b62a6' --opacity 35
pixelox shape path deep.pxo --in-place --name Waterline --d 'M 0 519 C 140 600 300 590 420 510 S 620 420 760 452 S 980 500 1080 468' --fill none --stroke '#1874b3' --stroke-width 3
pixelox shape ellipse deep.pxo --in-place --name Bub1 --rect 484,398,92x92 --fill '#3d9bd4' --stroke '#ffffff' --stroke-width 3 --opacity 80
pixelox shape ellipse deep.pxo --in-place --name Bub2 --rect 556,372,104x104 --fill '#2f8cc2' --stroke '#ffffff' --stroke-width 3 --opacity 80
pixelox shape ellipse deep.pxo --in-place --name Bub3 --rect 640,400,80x80 --fill '#3d9bd4' --stroke '#ffffff' --stroke-width 3 --opacity 80
pixelox shape ellipse deep.pxo --in-place --name Bub4 --rect 668,470,46x46 --fill '#2f8cc2' --stroke '#ffffff' --stroke-width 3 --opacity 80
pixelox shape ellipse deep.pxo --in-place --name Bub5 --rect 652,560,28x28 --fill '#3d9bd4' --stroke '#ffffff' --stroke-width 2 --opacity 80
pixelox shape icon deep.pxo ship --in-place --name Boat --size 170 --at 224,468 --anchor c --color '#ffffff' --stroke-width 9
pixelox shape icon deep.pxo bottle-wine --in-place --name Bottle --size 300 --at 968,560 --anchor c --color '#eaf6fd' --stroke-width 12
pixelox text add deep.pxo --in-place --name Slogan --content "JACKET ON" --font @heavy-wide --fit-width 936 --color '#1478bd' --at 68,67 --anchor tl --anchor-to ink
pixelox text add deep.pxo --in-place --name Intro --content "Most people who drown while paddling were not wearing a life jacket: where the cause was known, a missing jacket played a part in 4 of 5 deaths.* Safe paddling means putting the jacket on before you leave the shore." --font @neo-grotesque --weight 500 --size 30 --tracking 10 --leading 37 --color '#1478bd' --box 940x170 --at 68,236 --anchor tl
pixelox text add deep.pxo --in-place --name Head1 --content $'SHARE OF PADDLERS\nWHO WEAR A LIFE JACKET,\nBY REGION, LAST SUMMER' --font @neo-grotesque --weight 700 --size 29 --tracking 10 --leading 39 --color '#ffffff' --at 71,727 --anchor tl --anchor-to ink
pixelox render d3 scripts/usmap.js -o regions.png --size 1080x2670 --param box=4.5,33.5,88,23.5 --param groups='AK:1,WA:1,OR:1,CA:1,HI:1,NV:1,ID:2,MT:2,WY:2,UT:2,CO:2,AZ:2,NM:2,ND:2,SD:2,NE:2,KS:2,OK:2,TX:2,MN:3,IA:3,MO:3,IL:3,WI:3,IN:3,MI:3,OH:3,KY:3,AR:4,LA:4,MS:4,AL:4,TN:4,GA:4,FL:4,SC:4,NC:4,VA:4,WV:4,PA:5,NY:5,NJ:5,CT:5,RI:5,MA:5,VT:5,NH:5,ME:5,MD:5,DE:5,DC:5' --param colors='#0083b3,#fd6100,#a7d6ec,#ef8f01,#e1e2e4' --param labels=none --param stroke='#8fcbea' --param strokewidth=1
pixelox comp place deep.pxo regions.png --in-place --as Regions --at 0,0
pixelox render js scripts/donut.js -o buoy1.png --size 1080x2670 --param cx=11.9 --param cy=44.6 --param r=7.7 --param values=3,1,3,1,3,1,3,1 --param colors='#ffffff,#f26b20,#ffffff,#f26b20,#ffffff,#f26b20,#ffffff,#f26b20' --param start=11.25 --param inner=0.62 --param labelpos=none --param sep=none
pixelox render js scripts/donut.js -o buoy2.png --size 1080x2670 --param cx=34.4 --param cy=44.7 --param r=8.9 --param values=3,1,3,1,3,1,3,1 --param colors='#ffffff,#f26b20,#ffffff,#f26b20,#ffffff,#f26b20,#ffffff,#f26b20' --param start=11.25 --param inner=0.62 --param labelpos=none --param sep=none
pixelox render js scripts/donut.js -o buoy3.png --size 1080x2670 --param cx=58.2 --param cy=40.2 --param r=12.2 --param values=3,1,3,1,3,1,3,1 --param colors='#ffffff,#f26b20,#ffffff,#f26b20,#ffffff,#f26b20,#ffffff,#f26b20' --param start=11.25 --param inner=0.62 --param labelpos=none --param sep=none
pixelox render js scripts/donut.js -o buoy4.png --size 1080x2670 --param cx=85.5 --param cy=40.4 --param r=9.8 --param values=3,1,3,1,3,1,3,1 --param colors='#ffffff,#f26b20,#ffffff,#f26b20,#ffffff,#f26b20,#ffffff,#f26b20' --param start=11.25 --param inner=0.62 --param labelpos=none --param sep=none
pixelox render js scripts/donut.js -o buoy5.png --size 1080x2670 --param cx=70.9 --param cy=49.7 --param r=9.6 --param values=3,1,3,1,3,1,3,1 --param colors='#ffffff,#f26b20,#ffffff,#f26b20,#ffffff,#f26b20,#ffffff,#f26b20' --param start=11.25 --param inner=0.62 --param labelpos=none --param sep=none
pixelox comp place deep.pxo buoy1.png --in-place --as Buoy1 --at 0,0
pixelox fx drop-shadow deep.pxo --in-place --layer Buoy1 --offset 0,6 --blur 14 --color '#0b2f5e' --opacity 30
pixelox comp place deep.pxo buoy2.png --in-place --as Buoy2 --at 0,0
pixelox comp place deep.pxo buoy3.png --in-place --as Buoy3 --at 0,0
pixelox comp place deep.pxo buoy4.png --in-place --as Buoy4 --at 0,0
pixelox comp place deep.pxo buoy5.png --in-place --as Buoy5 --at 0,0
pixelox fx copy deep.pxo --in-place --from Buoy1 --to Buoy2,Buoy3,Buoy4,Buoy5
pixelox shape ellipse deep.pxo --in-place --name Core1 --rect 76,1139,106x106 --fill '#bfe3f0'
pixelox text add deep.pxo --in-place --name Pct1 --content "15%" --font @neo-grotesque --weight 700 --size 40 --color '#1d3f7a' --at 129,1184 --anchor c --anchor-to ink
pixelox text add deep.pxo --in-place --name Reg1 --content "COAST" --font @neo-grotesque --weight 700 --size 14 --tracking 40 --color '#1d3f7a' --at 129,1220 --anchor c
pixelox shape ellipse deep.pxo --in-place --name Core2 --rect 311,1132,122x122 --fill '#fbc7a4'
pixelox text add deep.pxo --in-place --name Pct2 --content "20%" --font @neo-grotesque --weight 700 --size 46 --color '#1d3f7a' --at 372,1185 --anchor c --anchor-to ink
pixelox text add deep.pxo --in-place --name Reg2 --content "MOUNTAIN" --font @neo-grotesque --weight 700 --size 14 --tracking 40 --color '#1d3f7a' --at 372,1225 --anchor c
pixelox shape ellipse deep.pxo --in-place --name Core3 --rect 544,989,168x168 --fill '#d6ecf7'
pixelox text add deep.pxo --in-place --name Pct3 --content "37%" --font @neo-grotesque --weight 700 --size 62 --color '#1d3f7a' --at 628,1065 --anchor c --anchor-to ink
pixelox text add deep.pxo --in-place --name Reg3 --content "LAKES" --font @neo-grotesque --weight 700 --size 16 --tracking 40 --color '#1d3f7a' --at 628,1117 --anchor c
pixelox shape ellipse deep.pxo --in-place --name Core4 --rect 855,1011,136x136 --fill '#eef1f6'
pixelox text add deep.pxo --in-place --name Pct4 --content "24%" --font @neo-grotesque --weight 700 --size 50 --color '#1d3f7a' --at 923,1071 --anchor c --anchor-to ink
pixelox text add deep.pxo --in-place --name Reg4 --content "NORTH-EAST" --font @neo-grotesque --weight 700 --size 14 --tracking 40 --color '#1d3f7a' --at 923,1114 --anchor c
pixelox shape ellipse deep.pxo --in-place --name Core5 --rect 700,1262,132x132 --fill '#fbdba6'
pixelox text add deep.pxo --in-place --name Pct5 --content "23%" --font @neo-grotesque --weight 700 --size 50 --color '#1d3f7a' --at 766,1320 --anchor c --anchor-to ink
pixelox text add deep.pxo --in-place --name Reg5 --content "SOUTH" --font @neo-grotesque --weight 700 --size 14 --tracking 40 --color '#1d3f7a' --at 766,1363 --anchor c
pixelox shape line deep.pxo --in-place --name Rule1 --from 70,1599 --to 1008,1599 --stroke '#ffffff' --stroke-width 2
pixelox text add deep.pxo --in-place --name Head2 --content $'PADDLERS COUNTED AND\nPADDLERS IN A JACKET,\nBY REGION, LAST SUMMER' --font @neo-grotesque --weight 700 --size 29 --tracking 10 --leading 39 --color '#ffffff' --at 71,1678 --anchor tl --anchor-to ink
pixelox shape icon deep.pxo bottle-wine --in-place --name Bottle2 --size 170 --at 962,1760 --anchor c --color '#bfe3f7' --stroke-width 8
pixelox render js scripts/bars.js -o col1.png --size 1080x2670 --param box=13.5,65.5,13,12 --param values='119;44' --param colors='#5aa9d6,#a9e4ff' --param max=200 --param axis=0 --param gridlines=0 --param gap=0.3 --param radius=0 --param ink='#ffffff' --param muted='#ffffff' --param fs=29 --param numweight=400
pixelox render js scripts/bars.js -o col2.png --size 1080x2670 --param box=28.1,65.5,13,12 --param values='84;20' --param colors='#d1eaf5,#eef6fb' --param max=200 --param axis=0 --param gridlines=0 --param gap=0.3 --param radius=0 --param ink='#ffffff' --param muted='#ffffff' --param fs=29 --param numweight=400
pixelox render js scripts/bars.js -o col3.png --size 1080x2670 --param box=42.7,65.5,13,12 --param values='96;14' --param colors='#0089bb,#18b4f0' --param max=200 --param axis=0 --param gridlines=0 --param gap=0.3 --param radius=0 --param ink='#ffffff' --param muted='#ffffff' --param fs=29 --param numweight=400
pixelox render js scripts/bars.js -o col4.png --size 1080x2670 --param box=57.3,65.5,13,12 --param values='180;41' --param colors='#d47f00,#fd9700' --param max=200 --param axis=0 --param gridlines=0 --param gap=0.3 --param radius=0 --param ink='#ffffff' --param muted='#ffffff' --param fs=29 --param numweight=400
pixelox render js scripts/bars.js -o col5.png --size 1080x2670 --param box=71.9,65.5,13,12 --param values='120;24' --param colors='#d55100,#ff7a1a' --param max=200 --param axis=0 --param gridlines=0 --param gap=0.3 --param radius=0 --param ink='#ffffff' --param muted='#ffffff' --param fs=29 --param numweight=400
pixelox comp place deep.pxo col1.png --in-place --as Col1 --at 0,0
pixelox fx drop-shadow deep.pxo --in-place --layer Col1 --offset 7,0 --blur 0 --color '#0d5a9e' --opacity 70
pixelox comp place deep.pxo col2.png --in-place --as Col2 --at 0,0
pixelox comp place deep.pxo col3.png --in-place --as Col3 --at 0,0
pixelox comp place deep.pxo col4.png --in-place --as Col4 --at 0,0
pixelox comp place deep.pxo col5.png --in-place --as Col5 --at 0,0
pixelox fx copy deep.pxo --in-place --from Col1 --to Col2,Col3,Col4,Col5
pixelox text add deep.pxo --in-place --name Name1 --content "LAKES" --font @condensed --weight 400 --size 24 --tracking 20 --color '#ffffff' --at 216,2096 --anchor c
pixelox text add deep.pxo --in-place --name Name2 --content "NORTH-EAST" --font @condensed --weight 400 --size 24 --tracking 20 --color '#ffffff' --at 374,2096 --anchor c
pixelox text add deep.pxo --in-place --name Name3 --content "COAST" --font @condensed --weight 400 --size 24 --tracking 20 --color '#ffffff' --at 531,2096 --anchor c
pixelox text add deep.pxo --in-place --name Name4 --content "SOUTH" --font @condensed --weight 400 --size 24 --tracking 20 --color '#ffffff' --at 689,2096 --anchor c
pixelox text add deep.pxo --in-place --name Name5 --content "MOUNTAIN" --font @condensed --weight 400 --size 24 --tracking 20 --color '#ffffff' --at 847,2096 --anchor c
pixelox text add deep.pxo --in-place --name Key --content "Left column: paddlers counted. Right column: wearing a jacket." --font @neo-grotesque --size 19 --color '#ffffff' --at 540,2134 --anchor c
pixelox shape line deep.pxo --in-place --name Rule2 --from 70,2165 --to 1008,2165 --stroke '#ffffff' --stroke-width 2
pixelox text add deep.pxo --in-place --name Close1 --content "Paddle smart. Paddle far." --font @geometric-sans --weight 700 --size 58 --color '#ffffff' --at 50%,2275 --anchor c
pixelox text add deep.pxo --in-place --name Close2 --content "And put the jacket on." --font @geometric-sans --weight 700 --size 58 --color '#ffffff' --at 50%,2351 --anchor c
pixelox text add deep.pxo --in-place --name SrcHead --content "Sources:" --font @neo-grotesque --weight 700 --size 15 --color '#ffffff' --at 71,2516 --anchor tl
pixelox text add deep.pxo --in-place --name Src --content $'Example Water Safety Council, launch-site count\n* Example Coroners Review. Example data, invented.' --font @neo-grotesque --size 15 --leading 19 --color '#ffffff' --at 71,2537 --anchor tl
pixelox text add deep.pxo --in-place --name Mark --content "YOUR MARK" --font @geometric-sans --weight 700 --size 30 --tracking 40 --color '#ffffff' --at 1004,2548 --anchor r
pixelox export deep.pxo -o deep.png
```

Reuse: change `groups=` for your regions and move the five badges to state anchors inside
them (`lab/anchors.py scripts/usmap.js` with the same `--size` and `box=`, or the
`render d3` line with `--json`; here CA 129,1192, CO 372,1193, WI 628,1073, MA 923,1079,
GA 766,1328); a badge's `r` in % of the width is 2 x the square root of its percentage
(12.2 for 37%), its tinted core is a disc of 0.64 x that radius on the same center, the
percentage sits 8 px above the center and the name a third of the radius below it. Check
that the distance between two centers is larger than the sum of the two radii, and take
a neighboring state's anchor when it is not. The map needs pixelox 0.6.0 or later. In the chart keep `max=` identical in all five
runs and above the largest total. The five names are centered under their pairs (20%, 34.6%,
49.2%, 63.8%, 78.4% W): each chart box is 13% W wide, so its middle is its left edge plus
6.5% W.

## 13. Variations and pairings

* **Other "going down" stories:** depth of a mine, floors of a building, layers of soil or
  atmosphere (flip the gradient: dark at the top for space). The ground gradient is the
  reusable idea.
* **One module only:** slogan, intro, map with badges, closing line: a 4:5 poster
  (1080 x 1350) for a feed.
* **Rates on the columns:** replace module 2 with C-progress-bars of the five shares when
  the counts are not known.
* **Fewer colors:** all regions in one teal with only the highest in orange, and the
  highlight repeated in the chart: stronger when the story is "one region stands out".
* Pairs with C-icon-array for the opening statistic ("4 of 5") beside the intro, and with
  C-stat-row above the closing slogan.

## 14. Pitfalls

* **Five percentages that look like one whole.** Badges on one map invite adding them up
  (they come to more than 100 here). Say "share within each region" in the heading and never
  draw them as a pie.
* **Five mini charts with five scales.** If `max=` is left to each chart, every region's
  tall column reaches the same height and the regions cannot be compared. One shared `max`.
  This is the data-integrity check for small multiples.
* **Badge sizes chosen by eye.** The largest badge must belong to the largest value and
  areas must follow the values; scale the radius by the square root, or make all badges one
  size.
* **Turned captions.** The original labels each column with 10 px type rotated 90 degrees;
  nobody reads it. Use one key line or two legend keys.
* **White type on the pale top.** White headings only work from about 27% H down, where the
  ground is darker than `#8cc8ea`; above that use the dark blue.
* **Pale region on a pale sea.** The gray-white and the pale blue regions nearly match the
  light water around them, and the outline map draws no coastline; give them tints that
  differ from the ground at that height, or move the map down to darker water.
