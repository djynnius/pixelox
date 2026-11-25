---
id: i004
title: Light results sheet with a ring hero and a seat map
kind: [results, geographic]
topic: [politics]
mood: [authoritative, bold, clean, serious]
style: [flat, editorial]
format: long
structure: S-results-sheet
data: [counted-units, part-to-whole, geography]
charts: [C-half-donut, C-hemicycle, C-tick-ring, C-us-bubble-map, C-us-choropleth, C-arc-label, C-ribbon-label, C-icon-badge, C-source-line]
palette: ['#e0e0e0', '#0d69a6', '#ca261c', '#13191b', '#b2b2b2', '#ffffff', '#e2c12f']
needs: nothing
---

# i004: Light results sheet with a ring hero and a seat map

## 1. Concept

A tall election-night results sheet on a light gray ground. One large circle holds a
two-chamber parliament: the large chamber as a solid arc band over the top, the small chamber
as a bowl of seat dots underneath, a white domed building in the hub, and each chamber's name
on a dark curved banner that hugs the ring. Below the ring a shallow arc of ticks counts a
third set of offices, and the bottom third is a real map of the states that shows the same
result place by place (seat by seat in the original; the side ahead and the number of seats
per state in the recipe). It works because the ring gives the national answer in one look and the map
then spends the rest of the page on "where", in the same blue and red; the gray wedge for
races not yet called keeps the picture honest while counting is still going on.

## 2. Data story

* **Large chamber (several hundred seats), part-to-whole in 3 parts:** the leading party a
  little over half, the other about a third, and about a tenth not yet called. Drawn as
  solid wedges of a 150 degree band: at that size dots would be specks.
* **Small chamber (about a quarter as many seats), counted units:** one side ahead by about
  a tenth of the seats, with a handful of others, one dot per seat in 5 rows. Beside each
  count a second, smaller figure says how many of those seats were already held and not up
  for election.
* **A third body (a few dozen offices), counted units:** two groups, the larger about a
  third bigger than the smaller, as one tick per office on a shallow arc.
* **Geography, counted units per region:** in the original every state holds as many small
  squares as it has seats in the large chamber (1 to a few dozen), each square colored by
  its result: won by A, won by B, A ahead, B ahead, or runoff. Most squares are in the two
  "won" colors; a few percent are pale "ahead" squares; one or two are green. The recipe
  keeps both facts at state level: the state's fill is the result that leads there (the
  same 5 classes) and a circle on it, with its **area** proportional to the number, gives
  the state's seats.
* **Two definitions** in a note beside the ring: what a majority is in each chamber, and what
  "not yet called" means.

Placeholder data used in the recipe (example data, invented):

| Module | Values |
| --- | --- |
| Lower house, 400 seats | Party A 204 (51%), not yet called 52 (13%), Party B 144 (36%) |
| Upper house, 90 seats | Party A 47 (30 not up for election), others 4, Party B 39 (25 not up) |
| Governors, 42 | Party A 24, Party B 18 |
| Map, one class per state | a made-up west and east split: 24 states called for B, 21 for A, 2 with A ahead, 2 with B ahead, 1 runoff |
| Map, seats per state | 50 invented counts from 1 to 42 (not a real apportionment) that add up to the lower house's 400 |

## 3. Information architecture

1. **Title** (two centered lines): the event and year, then the theme.
2. **Ribbon label** naming the new parliament: it says the ring below is one body.
3. **Hero ring:** dark banner with the large chamber's name on top; the arc band with three
   wedges and their counts; the building; the bowl of dots with its two counts at the ends;
   the navy banner with the small chamber's name at the bottom.
4. **Definitions** (bottom left of the ring): a check mark means "has the majority", an
   asterisk means "not yet called".
5. **Third body:** a caps word over a shallow arc of ticks, a count under each end.
6. **Map module:** a tracked caps heading, the map, a three-part legend at the lower right,
   a timestamp in italics.
7. **Sources** at the bottom left.

Everything down to the tick arc sits on one centered axis and inside one pale disc; the map
is a separate block with its own heading.

## 4. Format and grid

* Long portrait, 1 : 2.16. Recipe canvas 1080 x 2330. Side margins 4.3% W.
* Title: centered, cap tops at 4.1% and 7.4% H. Ribbon `34.6%,11.7%,30.9% x 2.1%`.
* Hero ring: one center at 50% W, 37.1% H. Arc band outer radius 38.6% W, inner 19% W, from
  75 degrees left of the top to 75 degrees right. Seat dots between 19% and 37.8% W, a bowl
  144 degrees wide. Hub radius 17.3% W. Hatched disc radius 41.2% W. Top banner: an arc
  segment 52 degrees wide between radii 39% and 47.5% W; bottom banner 50 to 56 degrees wide
  between 39% and 47.1% W.
* Pale back disc: center 50% W, 45.4% H, radius 46% W; its lower edge (66.7% H) closes the
  hero.
* Definitions: `6.7%,53.2%,22% x 6%`.
* Tick arc: part of a circle centered at 50% W, 43% H, radius 47.8% W, 77 degrees wide,
  centered on 6 o'clock; it runs from 20% to 80% W between 60.3% and 65.1% H. Its caps word
  is at 62.3% H.
* Map heading at 70.9% H. Map box `5%,72.3%,90% x 24%`: the map keeps its own 975 : 610
  shape, so it is 83% W wide and centered in the box; the legend sits right of the
  south-east coast (82% to 96% W, 87% to 96% H). Timestamp at 97% H, sources at 98.5% H.

## 5. Layer stack

1. Ground: light gray `#e0e0e0`, flat.
2. Back disc: `#e9e9e9`, radius 46% W, with a soft shadow under its lower edge.
3. Hatched disc: `#dcdcdc` with fine white diagonal hatching (lines 2.5 px, pitch 9 px, 45
   degrees), radius 41.2% W, on the ring's center.
4. Arc band: blue `#0d69a6` (left, the larger party), gray `#b2b2b2` (middle, not yet
   called), red `#ca261c` (right). No gaps: the wedges touch.
5. Seat dots: 5 rows, dot diameter 2.9% W; blue `#1a6fad`, a few gray, red `#d1291f`.
6. Hub: a white disc `#f5f5f5` with a soft shadow, holding a cut-out white domed building
   about 33% W wide whose spire rises over the gray wedge.
7. Top banner: near-black `#13191b` arc segment over the band's top; the chamber's name in
   white heavy caps on an arc, a yellow info line under it on a smaller arc.
8. Bottom banner: navy `#2d374e` arc segment under the bowl; name and info line on arcs.
9. Four round badges, one at each end of the two banners (left: party A, right: party B).
10. Wedge labels: count, share in brackets, party name; a check mark beside the leader's.
11. Bowl labels: a heavy count at each end, a gray curved line outside the dots.
12. Two definitions with small icons.
13. Tick arc: 42 to 50 short radial ticks, blue at the left, red at the right; heavy caps
    word above the middle; a curved label under each end.
14. Map heading; map (white states with seat squares in the original; in the recipe every
    state filled with the color of the result that leads there, thin borders in the ground
    gray, and a white circle with the state's seat count on it); legend swatches and text;
    timestamp; sources.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#e0e0e0` | ground; discs in `#e9e9e9` and `#dcdcdc` | 64% | nothing |
| `#ffffff` | hub, seat circles on the map, type on dark (and all the map's states in the original) | 3% | nothing |
| `#0d69a6` | wedge, dots (`#1a6fad`), ticks, map fills (`#1168a1`) | 10% | Party A, won |
| `#ca261c` | wedge, dots (`#d1291f`), ticks, map fills (`#c5271e`) | 12% | Party B, won |
| `#b2b2b2` | middle wedge, a few dots, ribbon (`#a8a8a8`) | 4% | not yet called; others |
| `#13191b` | title, top banner, heavy counts; bottom banner in navy `#2d374e` | 5% | nothing: structure |
| `#80d7fa`, `#fba5a6` | pale map fills | 1% | A ahead, B ahead (not final) |
| `#06b74c` | one map fill | under 1% | runoff |
| `#e2c12f` | info lines on the banners | under 1% | nothing: a small accent |

Categorical, two hues plus gray (two-party red and blue convention, US; swap in your
parties' colors). Lightness carries certainty on the map: saturated = won, pale = ahead.
Gray is "no result yet" in the band and "others" in the dots. The ground is gray, so white
can be used for the hub and the seat circles (and for the states in the original, where the
map's color is only the seat squares, under 3% of the sheet; the recipe's filled states
make the two party colors about twice as large).

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Title | 2 lines, 3 to 4 and 5 words | sentence | geometric sans (Poppins / Avenir Next) | 700 | 66 | -10 | line pitch 78 px | center | `#13191b` | 50%, cap tops 4.1% and 7.4% |
| Ribbon | 3 to 4 words | caps | geometric sans | 800 | 21 | 0 | - | center | `#ffffff` | 50%, 12.7% |
| Chamber name (x2) | 1 to 3 words | caps | geometric sans, heaviest face (Poppins Black / Avenir Next Heavy) | 900 | 30 | 0 | - | on an arc | `#ffffff` | arcs of radius 43.7% W (top) and 44.2% W (bottom) |
| Chamber info (x2) | "TOTAL SEATS: N / term", 6 to 7 words | caps | geometric sans | 600 | 15 | 0 | - | on an arc | `#e2c12f` | radius 40.5% W (top), 46.6% W (bottom) |
| Wedge count (x3) | 2 to 3 digits; an asterisk on the gray one | - | geometric sans, heaviest face | 900 | 56 (35 in the original, set along the radius) | 0 | - | center | `#ffffff`; dark on gray | middle of each wedge |
| Wedge share and name (x2) | "(NN%)" and 1 to 2 words | caps name | geometric sans | 400 / 500 | 22 / 20 | 0 / 40 | - | center | `#ffffff` | 46 and 74 px under each count |
| Gray wedge note | 2 to 3 words on 2 lines | sentence | geometric sans | 500 | 15 | 0 | 118% | center | `#13191b` | above its count, where the wedge is widest (under it in the original) |
| Bowl count (x2) | 2 digits | - | geometric sans, heaviest face | 900 | 44 | 0 | - | center | `#13191b` | outside each end of the bowl |
| Bowl info (x2) | "(NN%) N" and 2 to 3 words | caps | geometric sans | 400 | 20 | 0 | - | on an arc | `#8a8a8a` | radius 41.7% W, under each side |
| Others label | a digit and a word | caps | geometric sans | 700 | 18 | 40 | - | center | `#ffffff` | bottom banner, top edge |
| Definition lead (x2) | 1 to 3 words | sentence | geometric sans | 700 | 17 | 0 | - | left | `#13191b` | 8.8%, 53.5% and 56.7% |
| Definition text (x2) | 6 to 12 words on 2 or 3 lines of at most 21 characters | sentence | geometric sans | 400 | 17 | 0 | 21 px | left | `#3a3f44` | under each lead |
| Third body word | 1 word | caps | geometric sans, heaviest face | 900 | 35 | 0 | - | center | `#13191b` | 50%, 62.3% |
| Tick labels (x2) | party name and count | caps | geometric sans | 700 | 24 | 0 | - | on an arc | `#13191b` | under each end of the tick arc |
| Map heading | 3 to 4 words | caps | geometric sans | 600 | 28 | 100 | - | center | `#13191b` | 50%, 70.9% |
| Region names (the original) | full names, long ones shortened | caps | geometric sans | 800 | 11 | 0 | - | center | `#13191b` | under each region's squares |
| Seat counts (the recipe) | 1 to 2 digits | - | geometric sans | 700 | 10 to 40, set by the script from each circle's radius | 0 | - | center | `#1f2933` | in the circle on each state |
| Legend heads (x2) | 1 word and a colon | caps | geometric sans, heaviest face | 900 | 24 | 0 | - | left | `#13191b` | right of the swatches |
| Legend text (x3) | 1 to 5 words | sentence | geometric sans | 400 | 18 | 0 | 122% | left | `#3a3f44` | under each head |
| Timestamp | "Results as of" date and time | sentence | geometric sans italic | 400 | 16 | 0 | - | center | `#8a8a8a` | 50%, 97% |
| Sources | "Sources:" bold, then a list | sentence | geometric sans | 700 / 400 | 15 | 0 | - | left | `#13191b` | 6.2%, 98.5% |

One family in four weights (900 for counts and names, 700 for the title, 400 to 600 for
text). Sizes 66 : 56 : 35 : 28 : 17, about 4 : 3.3 : 2 : 1.6 : 1. The heaviest weight is
reserved for numbers and the three bodies' names.

## 8. Charts and diagrams

* **C-half-donut** (arc band), `scripts/donut.js` by center: `cx=50 cy=37.1 r=38.6`,
  `sweep=150 start=-75`, `inner=0.49`, `labelpos=in show=value`, `sep=none`, `fs=56`,
  `numweight=900`. Three values in sitting order: A, not yet called, B. The original writes
  count, share, and name **along the radius** inside each wedge (rotated 55 to 65 degrees);
  the script writes upright counts, and share and name are added under them with `text add`
  at the logged `slice1` and `slice3` anchors.
* **The two banners** are also `scripts/donut.js`: one value, one dark color,
  `labelpos=none`, a narrow sweep (52 and 50 degrees) centered on 12 and 6 o'clock, `r=47.5`
  and `47.1`, `inner=0.82` and `0.83`. Their text is **C-arc-label** on the ring's center.
* **C-hemicycle**, `scripts/hemicycle.js` on the same center: `r=38 flip=1 rows=5 inner=0.56
  sweep=144` (the inner row must clear the hub: inner row radius minus one dot radius stays
  above the hub's 17.3% W), groups left to right (A, others, B), `counts=0 legend=none`.
* **C-tick-ring** as an open arc: `scripts/donut.js` with `ticks=42` (one per office),
  `cx=50 cy=43 r=47.8 inner=0.93`, `sweep=77 start=141.5`. The sweep starts at the lower
  right, so the first value is the right-hand group.
* **C-ribbon-label** for the parliament's name; **C-icon-badge** (filled discs with a
  letter) at the banner ends.
* **C-us-bubble-map** on a **C-us-choropleth** fill, `scripts/usmap.js` (run with
  `render d3`): `groups=` with 5 classes (A won, B won, A ahead, B ahead, runoff) and
  `colors=` in that order; `bubbles=` with every state's seats; `labels=none`, `maxr=0.05`,
  `bubblecolor='#ffffff'`, `bubbleline='#13191b'`, `stroke='#e0e0e0'` (the ground color);
  box `5,72.3,90,24`. No `names=`, so the script draws no legend: the legend is native
  swatches and text. The circles' **areas** follow the seat counts, they push apart where
  they would overlap (the north-east), and the smallest ones (1 and 2 seats) are held at a
  readable minimum, which is why every circle carries its number.
  **What was simplified:** the original keeps the states white and draws one small square
  per seat, so a state can show both colors; the recipe gives each state one fill (the
  result that leads there) and its seat total in a circle. No axis, no gridlines.

**Map spec:**

* Region set: the 50 US states; the two non-contiguous states as insets at the lower left.
* Projection: Albers USA (curved northern border, the far-north state scaled down beside
  the south-west, the island state next to it), which is what `scripts/usmap.js` draws.
* Base, the original: every state filled white `#ffffff` with 1 px light gray borders
  `#d9d9d9` on the `#e0e0e0` ground; no choropleth fill. The recipe: every state filled
  with its class color, borders 1.2 px in the ground color.
* Encoding, the original: **one square per seat** (side 0.85% W, no gap) packed into a
  compact block near the state's centroid, roughly following the state's shape; 1 to a few
  dozen squares per state. Square color = result.
* Encoding, the recipe: fill = the result that leads in the state, 5 classes: `#1168a1` A
  won, `#c5271e` B won, `#80d7fa` A ahead, `#fba5a6` B ahead, `#06b74c` runoff; white
  circle = the state's seats (area proportional, number inside; the largest circle has a
  radius of 5% of the map's width).
* Labels: the original writes each state's full name in 11 px extra-bold caps directly
  under its block (long names shortened), no leader lines, the small north-eastern names
  to the right. The recipe writes no names (`labels=none`): the numbers already fill the
  north-east. A circle moved off its state keeps a thin line back to it.
* Legend: lower right, three groups stacked: a pale blue and a pale pink swatch with "LEAD:"
  and a two-line explanation; a blue and a red swatch with "WIN:"; a green swatch with one
  word. Swatches are 1.5% W wide, stacked without a gap. The line under the map says what
  the circles and the fills mean.
* Timestamp centered under the map in gray italics.
* Fallback for builds before 0.6.0: C-tile-map, `scripts/tilemap.js` (run with `render js`)
  with the same `groups=` and `colors=`, the seats as `values=`, and `showvalues=1`; say
  "one tile per state" on the image.

## 9. Icons and illustration

* Hub: `landmark`, 240 px, gray `#9aa0a6`, 12 px line, on the white hub disc. The original
  uses a cut-out rendering of a white domed government building, lit from above, about 33% W
  wide and tall enough for its spire to cross the middle wedge; place your own
  `illustration.png` there if you have one.
* `circle-check`, 34 px, white, left of the leading party's wedge count, and 30 px, near
  black, above its bowl count: "has the majority". The same icon and `asterisk` (26 px)
  open the two definitions.
* Banner badges: four 46 px discs. The original shows the two parties' mascots in them; the
  recipe uses the party color with a white letter. Do not invent emblems.

## 10. Effects and finish

* Hatching on the inner disc: `scripts/pattern.js` stripes (`cell=9 weight=0.28 angle=45`,
  white at 55%), clipped to the disc with `mask add --from-path` (a circle path).
* A soft shadow under the back disc (`fx drop-shadow`, offset 0,14, blur 40, opacity 18) and
  a smaller one under the hub: the only depth on the page.
* No gradients on the data; wedges and dots are flat.

## 11. Hierarchy and eye path

1. The blue and red wedges with their counts (largest colored areas, upper third).
2. Down through the building to the dots and the two heavy counts at the bowl's ends.
3. The dark banners, which name what was just read.
4. The tick arc and its word.
5. The map: color masses first, then single states and their seat circles; the legend; the
   fine print. The recipe's filled map is a larger color mass than the wedges; the ring
   still reads first because it is higher on the page, has the 56 px counts, and is framed
   by the dark banners.

Counts 56 px and 44 px at weight 900, title 66 px at 700, map heading 28, body 17.

## 12. Build recipe

Nothing to supply. Canvas 1080 x 2330. Example data throughout.

```bash
pixelox new -o sheet.pxo --size 1080x2330 --fill '#e0e0e0'
pixelox shape ellipse sheet.pxo --in-place --name BackDisc --rect 43,561,994x994 --fill '#e9e9e9'
pixelox fx drop-shadow sheet.pxo --in-place --layer BackDisc --offset 0,14 --blur 40 --color '#000000' --opacity 18
pixelox shape ellipse sheet.pxo --in-place --name Disc --rect 95,419,890x890 --fill '#dcdcdc'
pixelox render js scripts/pattern.js -o hatch.png --size 1080x2330 --param kind=stripes --param color='#ffffff' --param cell=9 --param weight=0.28 --param angle=45 --param alpha=0.55
pixelox comp place sheet.pxo hatch.png --in-place --as Hatch --at 0,0
pixelox mask add sheet.pxo --in-place --layer Hatch --from-path 'M 95 864 A 445 445 0 1 0 985 864 A 445 445 0 1 0 95 864 Z'
pixelox render js scripts/donut.js -o band.png --size 1080x2330 --param cx=50 --param cy=37.1 --param r=38.6 --param sweep=150 --param start=-75 --param inner=0.49 --param values=204,52,144 --param colors='#0d69a6,#b2b2b2,#ca261c' --param labelpos=in --param show=value --param sep=none --param fs=56 --param font='Avenir Next, Poppins, Montserrat, Century Gothic, Arial, sans-serif' --param numweight=900
pixelox comp place sheet.pxo band.png --in-place --as Band --at 0,0
pixelox render js scripts/hemicycle.js -o seats.png --size 1080x2330 --param cx=50 --param cy=37.1 --param r=38 --param flip=1 --param rows=5 --param inner=0.56 --param sweep=144 --param seats=47,4,39 --param colors='#1a6fad,#b2b2b2,#d1291f' --param counts=0 --param legend=none
pixelox comp place sheet.pxo seats.png --in-place --as Seats --at 0,0
pixelox shape ellipse sheet.pxo --in-place --name Hub --rect 353,677,374x374 --fill '#f5f5f5'
pixelox fx drop-shadow sheet.pxo --in-place --layer Hub --offset 0,6 --blur 18 --color '#000000' --opacity 20
pixelox shape icon sheet.pxo landmark --in-place --name Building --size 240 --at 540,852 --anchor c --color '#9aa0a6' --stroke-width 12
pixelox render js scripts/donut.js -o top.png --size 1080x2330 --param cx=50 --param cy=37.1 --param r=47.5 --param sweep=52 --param start=-26 --param inner=0.82 --param values=1 --param colors='#13191b' --param labelpos=none --param sep=none
pixelox comp place sheet.pxo top.png --in-place --as TopBanner --at 0,0
pixelox text add sheet.pxo --in-place --name TopName --content "LOWER HOUSE" --font @geometric-sans --weight 900 --size 30 --color '#ffffff' --align center
pixelox text on-path sheet.pxo --in-place --layer TopName --d 'M 68 864 A 472 472 0 0 1 1012 864' --offset 50%
pixelox text add sheet.pxo --in-place --name TopInfo --content "TOTAL SEATS: 400 / 4-YEAR TERMS" --font @geometric-sans --weight 600 --size 15 --color '#e2c12f' --align center
pixelox text on-path sheet.pxo --in-place --layer TopInfo --d 'M 103 864 A 437 437 0 0 1 977 864' --offset 50%
pixelox render js scripts/donut.js -o bottom.png --size 1080x2330 --param cx=50 --param cy=37.1 --param r=47.1 --param sweep=50 --param start=155 --param inner=0.83 --param values=1 --param colors='#2d374e' --param labelpos=none --param sep=none
pixelox comp place sheet.pxo bottom.png --in-place --as BottomBanner --at 0,0
pixelox text add sheet.pxo --in-place --name BotName --content "UPPER HOUSE" --font @geometric-sans --weight 900 --size 30 --color '#ffffff' --align center
pixelox text on-path sheet.pxo --in-place --layer BotName --d 'M 63 864 A 477 477 0 0 0 1017 864' --offset 50%
pixelox text add sheet.pxo --in-place --name BotInfo --content "TOTAL SEATS: 90 / 5-YEAR TERMS" --font @geometric-sans --weight 600 --size 15 --color '#e2c12f' --align center
pixelox text on-path sheet.pxo --in-place --layer BotInfo --d 'M 37 864 A 503 503 0 0 0 1043 864' --offset 50%
pixelox text add sheet.pxo --in-place --name Others --content "4 OTHERS" --font @geometric-sans --weight 700 --size 18 --tracking 40 --color '#ffffff' --at 540,1300 --anchor c
pixelox shape ellipse sheet.pxo --in-place --name BadgeTL --rect 291,433,46x46 --fill '#0d69a6' --stroke '#13191b' --stroke-width 4
pixelox shape ellipse sheet.pxo --in-place --name BadgeTR --rect 743,433,46x46 --fill '#ca261c' --stroke '#13191b' --stroke-width 4
pixelox shape ellipse sheet.pxo --in-place --name BadgeBL --rect 298,1252,46x46 --fill '#0d69a6' --stroke '#2d374e' --stroke-width 4
pixelox shape ellipse sheet.pxo --in-place --name BadgeBR --rect 736,1252,46x46 --fill '#ca261c' --stroke '#2d374e' --stroke-width 4
pixelox text add sheet.pxo --in-place --name BadgeTLt --content "A" --font @geometric-sans --weight 800 --size 22 --color '#ffffff' --at 314,456 --anchor c --anchor-to ink
pixelox text add sheet.pxo --in-place --name BadgeTRt --content "B" --font @geometric-sans --weight 800 --size 22 --color '#ffffff' --at 766,456 --anchor c --anchor-to ink
pixelox text add sheet.pxo --in-place --name BadgeBLt --content "A" --font @geometric-sans --weight 800 --size 22 --color '#ffffff' --at 321,1275 --anchor c --anchor-to ink
pixelox text add sheet.pxo --in-place --name BadgeBRt --content "B" --font @geometric-sans --weight 800 --size 22 --color '#ffffff' --at 759,1275 --anchor c --anchor-to ink
pixelox shape icon sheet.pxo circle-check --in-place --name CheckA --size 34 --at 254,616 --anchor c --color '#ffffff' --stroke-width 4
pixelox text add sheet.pxo --in-place --name PctA --content "(51%)" --font @geometric-sans --size 22 --color '#ffffff' --at 354,662 --anchor c
pixelox text add sheet.pxo --in-place --name NameA --content "PARTY A" --font @geometric-sans --weight 500 --size 20 --tracking 40 --color '#ffffff' --at 354,690 --anchor c
pixelox text add sheet.pxo --in-place --name Uncalled --content $'not yet\ncalled*' --font @geometric-sans --weight 500 --size 15 --leading 118% --align center --color '#13191b' --at 612,507 --anchor c
pixelox text add sheet.pxo --in-place --name PctB --content "(36%)" --font @geometric-sans --size 22 --color '#ffffff' --at 771,703 --anchor c
pixelox text add sheet.pxo --in-place --name NameB --content "PARTY B" --font @geometric-sans --weight 500 --size 20 --tracking 40 --color '#ffffff' --at 771,731 --anchor c
pixelox shape icon sheet.pxo circle-check --in-place --name CheckSeats --size 30 --at 106,956 --anchor c --color '#13191b' --stroke-width 4
pixelox text add sheet.pxo --in-place --name SeatsA --content "47" --font @geometric-sans --weight 900 --size 44 --color '#13191b' --at 106,1004 --anchor c --anchor-to ink
pixelox text add sheet.pxo --in-place --name SeatsB --content "39" --font @geometric-sans --weight 900 --size 44 --color '#13191b' --at 974,1004 --anchor c --anchor-to ink
pixelox text add sheet.pxo --in-place --name InfoA --content "(52%) 30 NOT UP" --font @geometric-sans --size 20 --color '#8a8a8a' --align center
pixelox text on-path sheet.pxo --in-place --layer InfoA --d 'M 90 864 A 450 450 0 0 0 990 864' --offset 20%
pixelox text add sheet.pxo --in-place --name InfoB --content "(43%) 25 NOT UP" --font @geometric-sans --size 20 --color '#8a8a8a' --align center
pixelox text on-path sheet.pxo --in-place --layer InfoB --d 'M 90 864 A 450 450 0 0 0 990 864' --offset 80%
pixelox shape icon sheet.pxo circle-check --in-place --name DefIcon1 --size 28 --at 72,1250 --anchor c --color '#13191b' --stroke-width 3
pixelox text add sheet.pxo --in-place --name Def1b --content "Majority" --font @geometric-sans --weight 700 --size 17 --color '#13191b' --at 95,1240 --anchor l
pixelox text add sheet.pxo --in-place --name Def1r --content $'46 seats in the upper\nhouse, 201 seats in\nthe lower house' --font @geometric-sans --size 17 --leading 21 --color '#3a3f44' --at 95,1253 --anchor tl
pixelox shape icon sheet.pxo asterisk --in-place --name DefIcon2 --size 26 --at 72,1342 --anchor c --color '#13191b' --stroke-width 3
pixelox text add sheet.pxo --in-place --name Def2b --content "Not yet called" --font @geometric-sans --weight 700 --size 17 --color '#13191b' --at 95,1332 --anchor l
pixelox text add sheet.pxo --in-place --name Def2r --content $'No winner has been\nprojected yet' --font @geometric-sans --size 17 --leading 21 --color '#3a3f44' --at 95,1345 --anchor tl
pixelox render js scripts/donut.js -o ticks.png --size 1080x2330 --param cx=50 --param cy=43 --param r=47.8 --param sweep=77 --param start=141.5 --param inner=0.93 --param values=18,24 --param colors='#ca261c,#0d69a6' --param ticks=42 --param labelpos=none
pixelox comp place sheet.pxo ticks.png --in-place --as Ticks --at 0,0
pixelox text add sheet.pxo --in-place --name Third --content "GOVERNORS" --font @geometric-sans --weight 900 --size 35 --color '#13191b' --at 540,1448 --anchor c --anchor-to ink
pixelox text add sheet.pxo --in-place --name TickA --content "PARTY A  24" --font @geometric-sans --weight 700 --size 24 --color '#13191b' --align center
pixelox text on-path sheet.pxo --in-place --layer TickA --d 'M -14 1002 A 554 554 0 0 0 1094 1002' --offset 35%
pixelox text add sheet.pxo --in-place --name TickB --content "PARTY B  18" --font @geometric-sans --weight 700 --size 24 --color '#13191b' --align center
pixelox text on-path sheet.pxo --in-place --layer TickB --d 'M -14 1002 A 554 554 0 0 0 1094 1002' --offset 65%
pixelox text add sheet.pxo --in-place --name T1 --content "General election results" --font @geometric-sans --weight 700 --size 66 --tracking=-10 --color '#13191b' --at 50%,95 --anchor t --anchor-to ink
pixelox text add sheet.pxo --in-place --name T2 --content "Who holds the two houses" --font @geometric-sans --weight 700 --size 66 --tracking=-10 --color '#13191b' --at 50%,173 --anchor t --anchor-to ink
pixelox shape path sheet.pxo --in-place --name RibbonTails --d 'M 335 285 L 395 285 L 395 334 L 335 334 L 357 309 Z M 745 285 L 685 285 L 685 334 L 745 334 L 723 309 Z' --fill '#666666'
pixelox shape rect sheet.pxo --in-place --name Ribbon --rect 373,271,334x49 --fill '#a8a8a8'
pixelox text add sheet.pxo --in-place --name RibbonText --content "THE 27TH PARLIAMENT" --font @geometric-sans --weight 800 --size 21 --color '#ffffff' --at 540,296 --anchor c --anchor-to ink
pixelox text add sheet.pxo --in-place --name MapHead --content "WHO LEADS WHERE" --font @geometric-sans --weight 600 --size 28 --tracking 100 --color '#13191b' --at 540,1651 --anchor c --anchor-to ink
pixelox render d3 scripts/usmap.js -o map.png --size 1080x2330 --param box=5,72.3,90,24 --param groups='AK:2,WA:2,ID:2,MT:2,ND:2,MN:2,OR:2,NV:2,WY:2,SD:2,IA:2,CA:2,UT:2,CO:2,NE:2,MO:2,AZ:2,NM:2,KS:2,AR:2,OK:2,LA:2,HI:2,TX:2,IL:3,KY:3,IN:4,TN:4,MS:5,ME:1,VT:1,NH:1,WI:1,MI:1,NY:1,RI:1,MA:1,OH:1,PA:1,NJ:1,CT:1,WV:1,VA:1,MD:1,DE:1,NC:1,SC:1,AL:1,GA:1,FL:1' --param bubbles='CA:42,TX:33,FL:25,NY:22,PA:15,IL:14,OH:13,GA:12,NC:11,MI:11,WA:11,NJ:10,TN:10,VA:9,MO:9,CO:9,AZ:8,MA:8,IN:7,MD:7,WI:7,MN:7,OR:7,SC:6,AL:6,OK:6,LA:5,KY:5,UT:5,IA:5,KS:5,CT:4,NM:4,NE:4,NV:3,AR:3,MS:3,ID:3,WV:3,HI:3,ME:3,MT:3,DE:2,SD:2,ND:2,AK:2,VT:2,WY:2,NH:1,RI:1' --param colors='#1168a1,#c5271e,#80d7fa,#fba5a6,#06b74c' --param stroke='#e0e0e0' --param labels=none --param maxr=0.05 --param bubblecolor='#ffffff' --param bubbleline='#13191b' --param ink='#13191b' --param fs=20 --param numfont='Avenir Next, Poppins, Montserrat, Century Gothic, Arial, sans-serif'
pixelox comp place sheet.pxo map.png --in-place --as Map --at 0,0
pixelox shape rect sheet.pxo --in-place --name KLeadA --rect 886,2030,16x34 --fill '#80d7fa'
pixelox shape rect sheet.pxo --in-place --name KLeadB --rect 886,2064,16x34 --fill '#fba5a6'
pixelox text add sheet.pxo --in-place --name KLeadH --content "LEAD:" --font @geometric-sans --weight 900 --size 24 --color '#13191b' --at 914,2042 --anchor l
pixelox text add sheet.pxo --in-place --name KLeadT --content $'Ahead, not\nyet decided' --font @geometric-sans --size 18 --leading 122% --color '#3a3f44' --at 914,2058 --anchor tl
pixelox shape rect sheet.pxo --in-place --name KWinA --rect 886,2124,16x34 --fill '#1168a1'
pixelox shape rect sheet.pxo --in-place --name KWinB --rect 886,2158,16x34 --fill '#c5271e'
pixelox text add sheet.pxo --in-place --name KWinH --content "WIN:" --font @geometric-sans --weight 900 --size 24 --color '#13191b' --at 914,2136 --anchor l
pixelox text add sheet.pxo --in-place --name KWinT --content $'Called for\none side' --font @geometric-sans --size 18 --leading 122% --color '#3a3f44' --at 914,2152 --anchor tl
pixelox shape rect sheet.pxo --in-place --name KRun --rect 886,2216,16x34 --fill '#06b74c'
pixelox text add sheet.pxo --in-place --name KRunT --content "Runoff" --font @geometric-sans --size 18 --color '#3a3f44' --at 914,2233 --anchor l
pixelox text add sheet.pxo --in-place --name Stamp --content "Results as of 6 June, 06:00. Circle area = lower house seats in the state; color = the side ahead there." --font @geometric-sans --style italic --size 16 --color '#8a8a8a' --at 540,2266 --anchor c
pixelox text add sheet.pxo --in-place --name Sources --content "Sources: Example Electoral Office. Example data, invented for this layout." --font @geometric-sans --weight 500 --size 15 --color '#13191b' --at 67,2298 --anchor l
pixelox export sheet.pxo -o sheet.png
```

Reuse: change `values=` (band), `seats=` (bowl), the ticks' `values=` and `ticks=` (their
sum), and the map's `groups=` and `bubbles=`. Keep party A on the left everywhere; the tick
arc lists the right-hand group first because its sweep starts at the lower right. Share and
name labels hang 46 and 74 px under the `slice1` and `slice3` anchors of the band (354,616
and 771,657 here), the check mark sits 100 px left of `slice1`, and the gray wedge's note
54 px above `slice2` along its radius: read the anchors again with `--json` after changing
the values. The map needs pixelox 0.6.0 or later (`render d3`); its log has every state's
center and each circle's `bx,by,br`, for a marker on one state. All arc labels are half circles on the
ring's center (540, 864) or the tick arc's center (540, 1002); the offsets 20% and 80%, 35%
and 65% slide a label along the lower half. The badges sit at the banner ends (29 and 28
degrees from the vertical at radius 467). Keep the definitions' lines under 21 characters:
they share the corner with the lower left badge and the first ticks.

## 13. Variations and pairings

* **Relative:** i003 is the dark twin (deep blue-green ground, cream chamber names written
  straight on arcs, a poll tick ring and a stat row below instead of a map). This one is
  light, puts the names on dark curved banners, shows a "not yet called" wedge, and ends in
  a map. Take i003 when the count is final and the page should feel like an evening
  broadcast; take this one for a live count with a regional breakdown.
* **Final results:** drop the gray wedge and the "ahead" classes; the map then needs only
  two colors and the legend one group.
* **No map:** end the sheet at the tick arc and add a C-stat-row: a 1 : 1.5 poster
  (1080 x 1620).
* **Other bodies:** the tick arc holds any third count of 20 to 60 units (mayors, regional
  assemblies, board seats).
* Pairs with C-percent-bar (national vote share) between the ring and the map, and with
  C-tile-map or C-hex-map (the same `groups=`) in place of the outline map when every state
  should weigh the same.

## 14. Pitfalls

* **A wedge for "not yet called" drawn in a party's tint.** It then reads as leaning. Keep
  it neutral gray, put it between the two parties, and explain the asterisk.
* **Band, bowl, and ticks in different orders.** Party A must be on the left in all three,
  and in the same blue; the tick arc's reversed value order is the usual place this breaks.
* **Counts that do not add up.** The three wedge values must total the chamber's seats (204
  + 52 + 144 = 400), the dot groups the small chamber's (47 + 4 + 39 = 90), `ticks=` the
  sum of its two values, and the map's circles the large chamber's seats again (400): check
  the `total` each script logs and add up `bubbles=`.
* **A state's fill read as its weight.** On a real map the large, thinly populated states
  cover most of the area, so the fills alone overstate the side that leads there, and one
  fill hides a split state. Keep the seat circles (area = seats, number printed), say on
  the image what fill and circle mean, and do not call the fills "seats won".
* **Rotated in-wedge labels at small sizes.** The original's radial type is 35 px and still
  hard to read on a phone; upright counts with the name underneath are safer.
* **A hub picture darker than the wedges.** The building must stay white or pale gray; a
  colored emblem in the hub competes with the two party colors.
