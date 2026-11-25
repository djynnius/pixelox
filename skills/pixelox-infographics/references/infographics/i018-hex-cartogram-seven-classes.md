---
id: i018
title: Hexagon cartogram in seven diverging classes
kind: [geographic, results]
topic: [politics]
mood: [clean, bold, editorial]
style: [flat, editorial]
format: landscape
structure: S-map-callouts
data: [geography, counted-units, comparison]
charts: [C-hex-map, C-legend]
palette: ['#ffffff', '#2a378f', '#c80205', '#663599', '#78a4ea', '#fc6769', '#1363ef']
needs: nothing
---

# i018: Hexagon cartogram in seven diverging classes

## 1. Concept

A map as the whole figure: the United States rebuilt from several hundred small hexagons, one per
vote in an indirect election, so each state is a cluster whose size is its weight in the count
and not its land area. Every cluster takes one of seven colors on a scale from "Bloc A wins"
through "open" to "Bloc B wins". The message is "this is how the count stands, and these are
the states that decide it". It works because area finally means what the reader thinks it
means: a big empty state shrinks to a few hexagons, a small crowded one grows, and the dark,
light, and purple areas can be compared by eye as shares of the total.

## 2. Data story

* **Geography with a weight:** 51 regions (50 states and a capital district), each with a
  whole-number weight from a fixed minimum (a low single digit) to about 18 times that; the
  weights sum to several hundred. A few regions are very large, most are between 4 and 12,
  several are at the minimum.
* **One class per region, 7 ordered classes on a diverging scale:** A wins, likely A, leaning
  A, open, leaning B, likely B, B wins. The two "wins" classes together hold more than half
  of all hexagons, the open class about a fifth, and the rest is spread over the four
  "likely" and "leaning" classes.
* Relationship shown: where the classes are (geography) and how much each weighs (counted
  units: one hexagon is one vote).

Placeholder data used in the recipe (example data, invented; the classes follow a made-up
north-to-south gradient and are not a forecast of anything):

| Class | Regions (two-letter codes) |
| --- | --- |
| 1 Bloc A wins | AK ME VT NH WA ID MT ND MN NY RI MA |
| 2 Likely A | IL WI MI OR CT |
| 3 Leaning A | NV WY SD IA NJ PA |
| 4 Open | IN OH CA UT CO MD DE |
| 5 Leaning B | NE MO KY WV VA DC |
| 6 Likely B | AZ NM KS AR TN NC SC |
| 7 Bloc B wins | OK LA MS AL GA HI TX FL |

## 3. Information architecture

1. **The map** (87% of the height): the reader sees three color masses first (dark blue,
   red, purple), then the pale classes between them.
2. **Region codes:** a two-letter code in small white caps near the middle of each cluster.
3. **A one-sentence note** in italic serif at the right, in the empty area beside the
   south-east: what a hexagon stands for.
4. **Legend** (a strip of seven tabs along the bottom): the classes in scale order, left to
   right, A to B, each tab filled with its class color and labelled in white caps.

There is no title on the image: it was made to sit inside running text, under a headline.
Add a C-title-block above it when it must stand alone (see Variations).

## 4. Format and grid

* Landscape 3:2, recipe canvas 1600 x 1066. Ground white, no frame.
* Map: `12.6%,2.3%,77.4% x 84.7%` in the original; the two far-away regions sit as small
  clusters at the lower left, set off by two thin dotted lines.
* Note: left edge 70.5% W, lines at 61.6% and 64.8% H.
* Legend strip: `15.7%,92.2%,68.7% x 5.8%`, seven equal tabs of 9.8% W each, touching.
* Everything is centered on the canvas; the legend is narrower than the map and centered
  under it.

## 5. Layer stack

1. Ground: white `#ffffff`.
2. Hexagons: pointy-top, 2.2% W wide in the original, packed with hairline gaps; each state's
   cluster is outlined by a 1 px white line, so neighbors of the same color still read as
   separate states.
3. Region codes: white, 12 px regular caps (one per cluster, on the hexagon nearest the
   cluster's center).
4. Two dotted gray lines `#c9c9c9` (1 px, 2 px dots) fencing off the two remote regions at
   the lower left.
5. Note: two lines of bold italic serif in dark gray `#4a4a4a`.
6. Legend: seven rectangles in the class colors, a 1 px white seam between them, white
   extra-bold caps centered in each, on 1 to 3 lines.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#ffffff` | ground, seams, codes, legend type | 62% | nothing |
| `#2a378f` | class 1 | 11% | Bloc A wins (certain) |
| `#1363ef` | class 2 | 2% | likely A |
| `#78a4ea` | class 3 | 6% | leaning A |
| `#663599` | class 4 | 6% | open: neither side |
| `#e09292` | class 5 | 3% | leaning B |
| `#fc6769` | class 6 | 4% | likely B |
| `#c80205` | class 7 | 5% | Bloc B wins (certain) |

Diverging, seven classes: two hues (blue and red, the two-party convention in the US) each in
three steps from saturated and dark (certain) to pale (leaning), meeting in a purple middle
(the mix of the two) for "open". Certainty is lightness; side is hue. No accent color: the
data is the only color on the page.

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Region code (x51) | 2 letters | caps | geometric sans (Montserrat / Avenir Next) | 400 in the original; 700 in the build | 12 in the original; 22 in the build | 0 | - | center | `#ffffff` | middle of each cluster |
| Note | one sentence, 7 to 9 words on 2 lines | sentence | bold italic serif, class oldstyle (Playfair Display bold italic / Baskerville bold italic) | 700 | 26 | 0 | 125% | left | `#4a4a4a` | 70.5% W, 61.6% H |
| Legend label (x7) | 1 to 3 words, one word per line | caps | geometric sans (Montserrat ExtraBold / Avenir Next) | 800 | 20 | 0 | 100% | center | `#ffffff` | centered in each tab |
| Source | "Example data" or the source, one line | sentence | geometric sans | 400 | 18 | 0 | - | left | `#8a8a8a` | under the note |

Two families: a geometric sans for every label, and one italic serif sentence as the only
"voice" on the page. Sizes 26 : 20 : 12 in the original; nothing is large, because the map is
the headline.

## 8. Charts and diagrams

* **C-hex-map**, `scripts/tilemap.js` with `shape=hex`, `groups=` (region:class for all 51
  regions), `colors=` the seven class colors in scale order, `gap=0.04`, `codes=1`,
  `legend=none`, box `10,3,80,86`. The script draws **one hexagon per region**. The original
  is a true cartogram (as many hexagons per region as its weight); the script cannot build
  that, so the weights are lost in the recipe and the note says so. Pair the map with C-bars
  of the class totals when the weights matter.
* **Legend:** seven `shape rect` tabs and seven centered `text add` labels (a filled-tab
  legend; `scripts/legend.js` draws small keys with names beside them, which is weaker here
  because the tab strip doubles as the color scale).

**Map spec (the original is a cartogram; the recipe keeps one hexagon per region):**

* Region set: the 50 US states plus the capital district; the two non-contiguous states as
  separate clusters at the lower left behind dotted divider lines.
* Projection: none in the cartogram. Clusters keep rough geographic neighbors (west coast at
  the left, north-east at the upper right, the south-east peninsula hanging at the lower
  right). As a plain choropleth on real outlines: C-us-choropleth (`scripts/usmap.js`,
  `render d3`, pixelox 0.6.0 or later) with the same `groups=` and `colors=`, Albers USA
  with the two remote states inset at the lower left; land area then replaces the weights,
  which is further from the original than the hexagons are.
* Encoding: fill = one of 7 ordered classes (hexes in section 6, in legend order). In the
  cartogram, the number of hexagons per region = the region's integer weight (from the fixed
  minimum to about 18 times that); hexagon width 2.2% W, pointy-top, gap 1 px.
* Borders: 1 px white around each region's cluster; none inside a cluster.
* Labels: two-letter codes, white, 12 px, centered in each cluster; no leader lines (the
  smallest clusters are only a few hexagons and still hold a code).
* Legend: seven filled tabs in a row under the map, `15.7%,92.2%,68.7% x 5.8%`, class names
  in white caps.
* Callout: one italic sentence right of the south-east, explaining the unit.

## 9. Icons and illustration

None. No icons, no party marks, no photos: the map and the legend carry everything.

## 10. Effects and finish

None: flat. No shadows, gradients, or outlines other than the white seams between regions.

## 11. Hierarchy and eye path

1. The largest dark mass (class 1) and the red mass (class 7): the two certain blocs.
2. The purple regions between them: the open ones, which are the story.
3. The pale classes, then the codes inside the regions the reader cares about.
4. The legend, to check a color; the italic note last.

Type is deliberately small (26, 20, 12 px on a 1600 px canvas): no text competes with the map.

## 12. Build recipe

Nothing to supply. Canvas 1600 x 1066. The classes are example data.

```bash
pixelox new -o map.pxo --size 1600x1066 --fill '#ffffff'
pixelox render js scripts/tilemap.js -o hex.png --size 1600x1066 --param box=10,3,80,86 --param shape=hex --param gap=0.04 --param groups='AK:1,ME:1,VT:1,NH:1,WA:1,ID:1,MT:1,ND:1,MN:1,NY:1,RI:1,MA:1,IL:2,WI:2,MI:2,OR:2,CT:2,NV:3,WY:3,SD:3,IA:3,NJ:3,PA:3,IN:4,OH:4,CA:4,UT:4,CO:4,MD:4,DE:4,NE:5,MO:5,KY:5,WV:5,VA:5,DC:5,AZ:6,NM:6,KS:6,AR:6,TN:6,NC:6,SC:6,OK:7,LA:7,MS:7,AL:7,GA:7,HI:7,TX:7,FL:7' --param colors='#2a378f,#1363ef,#78a4ea,#663599,#e09292,#fc6769,#c80205' --param legend=none --param fs=22 --param font='Avenir Next, Poppins, Montserrat, Century Gothic, Arial, sans-serif'
pixelox comp place map.pxo hex.png --in-place --as Hex --at 0,0
pixelox shape line map.pxo --in-place --name Fence1 --from 150,236 --to 300,236 --stroke '#c9c9c9' --stroke-width 2 --dash 2,6
pixelox shape line map.pxo --in-place --name Fence2 --from 190,746 --to 356,746 --stroke '#c9c9c9' --stroke-width 2 --dash 2,6
pixelox text add map.pxo --in-place --name Note --content $'Every state is one hexagon,\nwhatever its size or weight' --font @oldstyle --style italic --weight 700 --size 26 --leading 125% --color '#4a4a4a' --at 1250,690 --anchor tl
pixelox text add map.pxo --in-place --name Source --content "Example data, invented for this layout" --font @geometric-sans --size 18 --color '#8a8a8a' --at 1250,772 --anchor tl
pixelox shape rect map.pxo --in-place --name Tab1 --rect 251,983,155x62 --fill '#2a378f'
pixelox shape rect map.pxo --in-place --name Tab2 --rect 408,983,155x62 --fill '#1363ef'
pixelox shape rect map.pxo --in-place --name Tab3 --rect 565,983,155x62 --fill '#78a4ea'
pixelox shape rect map.pxo --in-place --name Tab4 --rect 722,983,155x62 --fill '#663599'
pixelox shape rect map.pxo --in-place --name Tab5 --rect 879,983,155x62 --fill '#e09292'
pixelox shape rect map.pxo --in-place --name Tab6 --rect 1036,983,155x62 --fill '#fc6769'
pixelox shape rect map.pxo --in-place --name Tab7 --rect 1193,983,155x62 --fill '#c80205'
pixelox text add map.pxo --in-place --name L1 --content $'BLOC A\nWIN' --font @geometric-sans --weight 800 --size 20 --leading 100% --align center --color '#ffffff' --at 328,1014 --anchor c
pixelox text add map.pxo --in-place --name L2 --content $'LIKELY\nBLOC A' --font @geometric-sans --weight 800 --size 20 --leading 100% --align center --color '#ffffff' --at 485,1014 --anchor c
pixelox text add map.pxo --in-place --name L3 --content $'LEANING\nBLOC A' --font @geometric-sans --weight 800 --size 20 --leading 100% --align center --color '#ffffff' --at 642,1014 --anchor c
pixelox text add map.pxo --in-place --name L4 --content "OPEN" --font @geometric-sans --weight 800 --size 20 --align center --color '#ffffff' --at 799,1014 --anchor c
pixelox text add map.pxo --in-place --name L5 --content $'LEANING\nBLOC B' --font @geometric-sans --weight 800 --size 20 --leading 100% --align center --color '#ffffff' --at 956,1014 --anchor c
pixelox text add map.pxo --in-place --name L6 --content $'LIKELY\nBLOC B' --font @geometric-sans --weight 800 --size 20 --leading 100% --align center --color '#ffffff' --at 1113,1014 --anchor c
pixelox text add map.pxo --in-place --name L7 --content $'BLOC B\nWIN' --font @geometric-sans --weight 800 --size 20 --leading 100% --align center --color '#ffffff' --at 1270,1014 --anchor c
pixelox export map.pxo -o map.png
```

Reuse: change `groups=` (every region needs a class; a region left out is drawn in the pale
`empty` color) and the seven tab labels. The tabs are 155 px wide with a 2 px seam, starting at
x 251, so the strip is centered (251 to 1348). The two dotted "fence" lines sit under the
`AK` tile and above the `HI` tile (their anchors: 216,153 and 271,828); drop them for a grid of
your own. For another set of regions pass `cells=` with your own grid.

## 13. Variations and pairings

* **Stand-alone card:** shrink the map box to `10,16,80,72`, add a C-title-block with the
  finding ("Seven states are still open") and a C-source-line under the legend.
* **Five classes or three:** drop the "likely" steps (`colors='#2a378f,#78a4ea,#663599,#e09292,#c80205'`)
  or keep only A, open, B; the tab strip gets wider tabs.
* **Weights on the tiles:** add `values='CA:47,TX:36,...'` (your weights) and `showvalues=1`
  to the same `groups=` run: every hexagon keeps its class color and carries its weight
  under the code, which restores what the one-tile-per-region build loses. Without
  `groups=`, the same `values=` on `ramp='#dbeafe,#1d4ed8'` makes a sequential map.
* **Other subjects:** any ordered opinion scale by region (strongly for to strongly against),
  risk levels, adoption stages. Change the two hues, keep "lightness = certainty".
* Pairs with C-percent-bar above the map (the weights summed per class: the finishing line of
  the count) and C-stat-row for the two totals.

## 14. Pitfalls

* **Equal tiles read as equal weights.** In the recipe the lightest region and one that
  weighs about 18 times as much are the same size, which is the opposite of the original's
  point. Say "one hexagon per
  state" on the image, and print the weights (`showvalues=1`) or add a bar of class totals.
  This is the data-integrity rule for every tile map.
* **A legend out of scale order.** The seven tabs are the color scale: A to B, certain to
  leaning to open to leaning to certain. Shuffled tabs destroy the diverging reading.
* **Pale classes that vanish on white.** The two "leaning" tints are the lightest colors and
  sit on a white ground: keep them at least as dark as `#78a4ea` and `#e09292`.
* **Purple that leans to one side.** The middle class must look like neither hue; a bluish
  violet reads as a fourth A class. Use a balanced purple or a neutral gray.
* **Red and green for the two sides** fails for color-blind readers; blue and red (or blue and
  orange) with a lightness ramp survives.
* **Codes in dark type on dark tiles.** Keep every code white, and bold enough (700) to read
  on the palest tint.
