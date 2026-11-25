---
id: i006
title: Dark globe with heads-up callouts
kind: [geographic, dashboard]
topic: [science, environment, technology]
mood: [dark, futuristic, technical, cool]
style: [3d, photo-led, line-icon]
format: landscape
structure: S-hub-spokes
data: [geography, counted-units, comparison]
charts: [C-globe, C-icon-array, C-negative-columns, C-stat-block, C-callout]
palette: ['#273947', '#2a86ad', '#ffffff', '#b9cf8a', '#f0a04e', '#e2575a', '#5c707d']
needs: nothing
---

# i006: Dark globe with heads-up callouts

## 1. Concept

A landscape "mission control" sheet: a large, realistically shaded globe floats in the
middle of a dark slate ground, lit from the upper left with a warm glow behind its rim, and
small white read-outs hang around it on hairline leaders like a heads-up display. Each
read-out is a different tiny chart: a rated list, a mini column chart, two rows of people,
an icon with notes, one big specification number. The message is "one planet, many small
facts". It works because the globe is the only thing with color and volume, so it is the
unquestioned center, while every read-out is thin, white, and small enough to feel like an
instrument reading; none of them competes with it.

## 2. Data story

A sampler of small data shapes around one geographic subject (in the original the texts are
tiny placeholders; only the forms are meaningful):

* **A rated list, 4 rows:** each an icon, a two-digit value in thousands (the last under
  half of the first), and a row of 8 dots of which a falling number are filled (a rating,
  or the value on a coarse scale).
* **A mini series, about 16 periods:** small columns rising to a hump, then falling below
  the baseline (a diverging column chart: values above and below zero).
* **Two shares as people:** two rows of 10 figures, some of them highlighted (more in one
  row than in the other), each with a value in thousands.
* **One specification:** a small label, one large number with its unit, one detail line.
* **Two text notes** beside an icon (2-line headings, 5 to 8 very short lines each).
* **Places on the globe:** a pin, two ring markers, and a translucent disc mark four spots.

Placeholder data used in the recipe (example data, invented: travel between the Americas):
trips by mode in thousands 68, 49, 33, 18 with 7, 5, 3, 2 of 8 dots (one dot per ten
thousand); a 15-period series of yearly changes, seven rises of 1 to 5 and then eight falls
of 1 to 5; coastal residents 74K and 26K as 7 and 3 of 10 figures; "48 routes".

## 3. Information architecture

1. **Title** (top left): a two-part name in bold white, and a version line with a small
   triangle and a red number.
2. **The globe** (center): the subject; markers on it show where the read-outs point.
3. **Left column**, top to bottom: the rated list, the mini column chart, the
   specification block.
4. **Right column**: an upper callout (heading, two lines, the two people rows inside a
   bracket) on a leader from the globe's upper right; a lower callout (an icon and two
   text columns) above a long leader from a ring on the globe's lower right.

There is no reading order beyond "title, globe, then anything": the leaders tie each
read-out to a place, and each block stands alone.

## 4. Format and grid

* Landscape 3:2. Recipe canvas 1600 x 1066.
* Globe: center 50% W, 58% H, radius 22.8% W (it spans 24% to 93% of the height).
* Left column from 12% W: title cap top 7.7% H; version line 14.6% H; list heading 34.5% H;
  list rows at 38.8%, 42.8%, 46.7%, 50.7% H (icon at 12.8% W, value at 15.3% W, dots 18.7%
  to 27% W); chart heading 57.2% H; chart `11.9%,60%,14.7% x 8.5%` (20.6% W wide in the original, where it slips under the globe's edge); specification 81% to
  91.5% H.
* Right column, right edge at 87.5% W: callout 1 leader at 28% H, heading above it, people
  rows at 36% and 40% H from 73.7% W; callout 2 headings at 53% H (73.9% and 82.5% W),
  icon at 77% W, 62% H; long leader at 78.4% H.
* Columns are flush left on 12% W at the left and flush right on 87.5% W at the right: the
  read-outs frame the globe symmetrically.

## 5. Layer stack

1. Ground: slate blue `#273947`, with a soft radial lift to `#3f5262` behind the globe
   (corners darkest).
2. Rim glow: a warm orange haze `#f0a04e` behind the globe's upper left, reaching 8% W past
   the rim.
3. Globe (a rendered sphere in the original; see section 8 for the map and 3D spec): ocean
   `#2a86ad` shading to `#0d4f73` at the lower left, land in greens and browns, white cloud
   bands, a thin bright atmosphere line on the lit side, a soft shadow under it.
4. Globe markers in white: a map pin on the northern land mass, a small ring, a translucent
   white disc (radius 3.9% W, 25% opaque) over the sea, a larger ring at the lower right, a
   hairline dropping from mid-globe to a two-line caption.
5. Leaders: white hairlines, 1.5 px, 60% opaque, each with one bend.
6. Read-outs in white and pale gray `#c9d3d8`; unfilled dots and figures in `#5c707d`.
7. Title in white; version number in red `#e2575a`.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#273947` | ground (`#3f5262` at the center) | 66% | nothing |
| `#2a86ad` | ocean | 14% | nothing: the picture |
| `#b9cf8a` | land | 6% | nothing: the picture |
| `#ffffff` | type, filled dots and figures, columns, leaders | 6% | "counted" or "filled" in the small charts |
| `#5c707d` | unfilled dots and figures, chart rules | 2% | the remainder of each scale |
| `#f0a04e` | rim glow | 4% | nothing: light |
| `#e2575a` | version number | under 1% | the single accent |

All data is white on dark: filled against unfilled is the only encoding. Color belongs to
the globe. One small red accent marks the version number.

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Title | 3 to 4 words with a comma | sentence | geometric sans (Jost, Montserrat / Avenir Next) | 700 | 68 | 0 | - | left | `#ffffff` | 12%, cap top 7.7% |
| Version line | "V." and a 2-digit number | caps | geometric sans | 700 | 44 | 0 | - | left | `#ffffff`, number `#e2575a` | 14.5%, 16.6% |
| Read-out heading (x5) | 3 to 5 words on 1 or 2 lines | title | geometric sans | 600 | 15 | 0 | 19 px | left; right in callout 1 | `#ffffff` | above each read-out |
| List value (x4) | NNK | caps | geometric sans | 600 | 20 | 0 | 42 px pitch | left | `#c9d3d8` | 15.3% W |
| People value (x2) | NNK | caps | geometric sans | 600 | 20 | 0 | 40 px pitch | left | `#c9d3d8` | 71.2% W |
| Note text (x3) | 2 to 8 lines of at most 18 characters | sentence | geometric sans | 400 | 13 (about 7 in the original: unreadable) | 0 | 17 px | left or right | `#c9d3d8` | under their headings |
| Spec label | 2 words and a colon | title | geometric sans | 400 | 20 | 0 | - | left | `#c9d3d8` | 12%, 81.4% |
| Spec number | a number and a unit | caps | geometric sans | 700 | 54 | 0 | - | left | `#ffffff` | 12%, 85.8% |
| Spec detail | 2 to 4 words, the unit bold | caps | geometric sans | 400 | 24 | 0 | - | left | `#ffffff` | 12%, 90.5% |

One family, three weights. Sizes 68 : 54 : 20 : 15 : 13. Everything except the title and the
one specification number is at caption size: the sheet is meant to be looked at, then
zoomed.

## 8. Charts and diagrams

* **C-globe** (`scripts/geo.js`, run with `render d3`; pixelox 0.6.0 or later). Map spec:
  region set = the world's land as one shape, the visible half of the sphere; projection =
  a globe (orthographic) centered on 85 degrees west, 15 degrees north
  (`projection=globe rotate=-85,15`), north up, so the northern continent sits upper left,
  the southern lower right, and the isthmus crosses the center; box
  `27.19,23.73,45.63,68.48` (a 730 px square: radius 22.8% W). Nothing is encoded: land
  `#b9cf8a`, ocean `#2a86ad`, `stroke=none` (no borders), `graticule=0`, the script's
  sphere shading (`shade=1`: a highlight at the upper left, a darker limb). No legend, no
  labels, no insets. The five markers are geographic points whose canvas positions come
  from the script's anchors (see the notes under the recipe).
* **Fallback for builds before 0.6.0:** an ocean disc (`shape ellipse`) with **C-dot-map**
  (`scripts/worldmap.js`, same `points=`) scaled up so one hemisphere fills the disc and
  clipped to it with `mask add --from-path`.
* **3D spec of the original (for your own render):** a sphere of radius 22.8% W, the same
  view, no tilt. Texture: natural-color land (greens `#6f8f4e`, browns `#a8956a`), ocean
  `#2a86ad` with lighter shelves, a semi-transparent white cloud layer with visible
  shadows. One directional light from the upper left front (about 40 degrees elevation, 35
  degrees to the left of the camera): the lower right limb falls to `#0d4f73`. Atmosphere: a
  1% W bright blue-white rim on the lit side; behind the upper left limb a warm orange halo
  `#f0a04e` fading out over 8% W. A soft contact shadow under the sphere. Depth and shading
  encode nothing. The script's globe is flat-colored: no relief, no clouds.
* **Rated list:** four `scripts/waffle.js` rows, `shape=circle cols=8`, `value=N total=8`,
  white on `empty='#5c707d'`; icons and values are native.
* **C-negative-columns** as the mini chart: `scripts/bars.js`, 15 values, the first seven
  above zero and the rest below, `axis=0 gridlines=0 vlabels=0`, no labels, all white,
  `gap=0.25 radius=0`, between two hairline rules; the script draws the zero line
  (`grid='#8fa0ab'`). This is the original's form: a hump above the baseline, then a
  trough below it.
* **C-icon-array** x2: `shape=person cols=10`, `value=6 total=10` and `value=4 total=10`.
* **C-stat-block** for the specification; **C-callout** for the two leaders.

## 9. Icons and illustration

* List icons, 26 px, white: `plane`, `car`, `train-front`, `ship`.
* `map-pin`, 34 px, white, its tip on a point of the globe (`--anchor b`); two
  `shape ellipse` rings (radius 8 and 14 px, 1.5 to 2 px line) and one translucent white
  disc, each centered on a point.
* `tree-deciduous`, 96 px, white, 5 px line, in the lower right callout (the original: a
  solid white tree silhouette).
* A small gray triangle (`shape polygon`, 3 sides, turned to point right) before the
  version line.
* The globe itself is the illustration: the script draws a clean vector globe; place your
  own photoreal render (`illustration.png`, 730 px across, transparent) over it when you
  have one.

## 10. Effects and finish

* Background lift: a radial `fill gradient` from `#3f5262` at the globe to `#243542` at the
  corners.
* Rim glow: one soft orange blob (`scripts/blobs.js`, `soft=1`, one fixed center) behind
  the globe's upper left.
* Globe shading: the script's `shade=1` (a soft highlight toward the upper left, a
  darker limb) and `fx drop-shadow` under the globe layer (offset 0,30, blur 60, 45%).
* Leaders and rules at 60% opacity so they sit behind the type.

## 11. Hierarchy and eye path

1. The globe: color, size, light.
2. The title, top left, in the only large type.
3. The specification number at the bottom left (the second largest type).
4. The read-outs, in any order, each found by following a leader from the globe.

Title 68 px, specification 54, values 20, headings 15, notes 13.

## 12. Build recipe

Nothing to supply. Canvas 1600 x 1066. All figures are example data.

```bash
pixelox new -o globe.pxo --size 1600x1066 --fill '#273947'
pixelox layer add globe.pxo --in-place --name Lift
pixelox fill gradient globe.pxo --in-place --layer Lift --kind radial --from 50%,58% --to 100%,100% --stops '#3f5262@0,#243542@1'
pixelox render js scripts/blobs.js -o glow.png --size 1600x1066 --param colors='#f0a04e' --param bg=transparent --param count=1 --param size=0.42 --param vary=0 --param soft=1 --param alpha=0.9 --param at='0.41,0.46'
pixelox comp place globe.pxo glow.png --in-place --as Glow --at 0,0
pixelox render d3 scripts/geo.js -o earth.png --size 1600x1066 --param box=27.19,23.73,45.63,68.48 --param projection=globe --param rotate=-85,15 --param land='#b9cf8a' --param ocean='#2a86ad' --param stroke=none --param graticule=0
pixelox comp place globe.pxo earth.png --in-place --as Globe --at 0,0
pixelox fx drop-shadow globe.pxo --in-place --layer Globe --offset 0,30 --blur 60 --color '#000000' --opacity 45
pixelox shape icon globe.pxo map-pin --in-place --name Pin --size 34 --at 695,376 --anchor b --color '#ffffff' --stroke-width 3
pixelox shape ellipse globe.pxo --in-place --name Ring1 --rect 783,422,16x16 --fill none --stroke '#ffffff' --stroke-width 1.5
pixelox shape ellipse globe.pxo --in-place --name Zone --rect 941,546,124x124 --fill '#ffffff' --opacity 25
pixelox shape ellipse globe.pxo --in-place --name Ring2 --rect 918,785,28x28 --fill none --stroke '#ffffff' --stroke-width 2
pixelox shape path globe.pxo --in-place --name Lead2 --d 'M 946 799 L 990 836 L 1400 836' --fill none --stroke '#ffffff' --stroke-width 1.5 --opacity 60
pixelox shape path globe.pxo --in-place --name Lead1 --d 'M 1063 366 L 1130 300 L 1400 300' --fill none --stroke '#ffffff' --stroke-width 1.5 --opacity 60
pixelox shape line globe.pxo --in-place --name Drop --from 721,502 --to 721,838 --stroke '#ffffff' --stroke-width 1.5 --opacity 60
pixelox text add globe.pxo --in-place --name DropCap --content $'Deepest trench\n8,400 m' --font @geometric-sans --size 13 --leading 17 --align right --color '#ffffff' --at 709,818 --anchor r
pixelox text add globe.pxo --in-place --name Title --content "The Americas, in motion" --font @geometric-sans --weight 700 --size 68 --color '#ffffff' --at 191,83 --anchor tl --anchor-to ink
pixelox shape polygon globe.pxo --in-place --name Tri --center 204,178 --radius 14 --sides 3 --rotation 90 --fill '#7d8d98'
pixelox text add globe.pxo --in-place --name Ver --content "V." --font @geometric-sans --weight 700 --size 44 --color '#ffffff' --at 232,178 --anchor l
pixelox text add globe.pxo --in-place --name VerN --content "04" --font @geometric-sans --weight 700 --size 44 --color '#e2575a' --at 290,178 --anchor l
pixelox text add globe.pxo --in-place --name ListHead --content $'Trips per year\nby mode, thousands' --font @geometric-sans --weight 600 --size 15 --leading 19 --color '#ffffff' --at 191,352 --anchor tl
pixelox shape icon globe.pxo plane --in-place --name Li1 --size 26 --at 205,414 --anchor c --color '#ffffff'
pixelox shape icon globe.pxo car --in-place --name Li2 --size 26 --at 205,456 --anchor c --color '#ffffff'
pixelox shape icon globe.pxo train-front --in-place --name Li3 --size 26 --at 205,498 --anchor c --color '#ffffff'
pixelox shape icon globe.pxo ship --in-place --name Li4 --size 26 --at 205,540 --anchor c --color '#ffffff'
pixelox text add globe.pxo --in-place --name ListVals --content $'68K\n49K\n33K\n18K' --font @geometric-sans --weight 600 --size 20 --leading 42 --color '#c9d3d8' --at 245,477 --anchor l
pixelox render js scripts/waffle.js -o d1.png --size 1600x1066 --param box=18.7,38.1,8.3,1.4 --param value=7 --param total=8 --param cols=8 --param shape=circle --param colors='#ffffff' --param empty='#5c707d' --param legend=none
pixelox render js scripts/waffle.js -o d2.png --size 1600x1066 --param box=18.7,42.1,8.3,1.4 --param value=5 --param total=8 --param cols=8 --param shape=circle --param colors='#ffffff' --param empty='#5c707d' --param legend=none
pixelox render js scripts/waffle.js -o d3.png --size 1600x1066 --param box=18.7,46,8.3,1.4 --param value=3 --param total=8 --param cols=8 --param shape=circle --param colors='#ffffff' --param empty='#5c707d' --param legend=none
pixelox render js scripts/waffle.js -o d4.png --size 1600x1066 --param box=18.7,50,8.3,1.4 --param value=2 --param total=8 --param cols=8 --param shape=circle --param colors='#ffffff' --param empty='#5c707d' --param legend=none
pixelox comp place globe.pxo d1.png --in-place --as Dots1 --at 0,0
pixelox comp place globe.pxo d2.png --in-place --as Dots2 --at 0,0
pixelox comp place globe.pxo d3.png --in-place --as Dots3 --at 0,0
pixelox comp place globe.pxo d4.png --in-place --as Dots4 --at 0,0
pixelox text add globe.pxo --in-place --name ChartHead --content "Crossings, yearly change" --font @geometric-sans --weight 600 --size 15 --color '#ffffff' --at 191,610 --anchor l
pixelox shape path globe.pxo --in-place --name ChartRules --d 'M 191 636 L 425 636 M 191 736 L 425 736' --fill none --stroke '#5c707d' --stroke-width 1.5
pixelox render js scripts/bars.js -o series.png --size 1600x1066 --param box=12.5,60.2,13.5,8.4 --param values=1,2,4,5,4,3,1,-1,-2,-3,-5,-3,-2,-1,-3 --param colors='#ffffff' --param grid='#8fa0ab' --param axis=0 --param gridlines=0 --param vlabels=0 --param gap=0.25 --param radius=0
pixelox comp place globe.pxo series.png --in-place --as Series --at 0,0
pixelox text add globe.pxo --in-place --name SpecL --content "Data set:" --font @geometric-sans --size 20 --color '#c9d3d8' --at 191,868 --anchor l
pixelox text add globe.pxo --in-place --name SpecN --content "48 ROUTES" --font @geometric-sans --weight 700 --size 54 --color '#ffffff' --at 191,915 --anchor l --anchor-to ink
pixelox text add globe.pxo --in-place --name SpecD --content "12 COUNTRIES, 4 MODES" --font @geometric-sans --size 24 --color '#ffffff' --at 191,965 --anchor l
pixelox text add globe.pxo --in-place --name C1Head --content "People on the coast" --font @geometric-sans --weight 600 --size 15 --color '#ffffff' --at 1400,276 --anchor r
pixelox text add globe.pxo --in-place --name C1Note --content $'Residents within 50 km\nof the sea, thousands' --font @geometric-sans --size 13 --leading 17 --align right --color '#c9d3d8' --at 1400,330 --anchor r
pixelox shape path globe.pxo --in-place --name Bracket --d 'M 1128 366 Q 1118 366 1118 380 L 1118 396 Q 1118 405 1110 405 Q 1118 405 1118 414 L 1118 430 Q 1118 444 1128 444' --fill none --stroke '#ffffff' --stroke-width 2
pixelox text add globe.pxo --in-place --name C1Vals --content $'74K\n26K' --font @geometric-sans --weight 600 --size 20 --leading 40 --color '#c9d3d8' --at 1140,405 --anchor l
pixelox render js scripts/waffle.js -o p1.png --size 1600x1066 --param box=74.4,34.4,13.2,3 --param value=7 --param total=10 --param cols=10 --param shape=person --param colors='#ffffff' --param empty='#5c707d' --param legend=none
pixelox render js scripts/waffle.js -o p2.png --size 1600x1066 --param box=74.4,38.2,13.2,3 --param value=3 --param total=10 --param cols=10 --param shape=person --param colors='#ffffff' --param empty='#5c707d' --param legend=none
pixelox comp place globe.pxo p1.png --in-place --as People1 --at 0,0
pixelox comp place globe.pxo p2.png --in-place --as People2 --at 0,0
pixelox text add globe.pxo --in-place --name C2Head1 --content $'Forest cover\nby region' --font @geometric-sans --weight 600 --size 15 --leading 19 --color '#ffffff' --at 1182,566 --anchor tl
pixelox shape icon globe.pxo tree-deciduous --in-place --name Tree --size 96 --at 1232,668 --anchor c --color '#ffffff' --stroke-width 5
pixelox text add globe.pxo --in-place --name C2Text1 --content $'Half of the south\nis still forest,\na third of the north.' --font @geometric-sans --size 13 --leading 17 --color '#c9d3d8' --at 1182,730 --anchor tl
pixelox text add globe.pxo --in-place --name C2Head2 --content $'What the map\nleaves out' --font @geometric-sans --weight 600 --size 15 --leading 19 --color '#ffffff' --at 1320,566 --anchor tl
pixelox text add globe.pxo --in-place --name C2Text2 --content $'Short hops\nunder 50 km,\nfreight, and\nall crossings\non foot are\nnot counted.' --font @geometric-sans --size 13 --leading 17 --color '#c9d3d8' --at 1320,612 --anchor tl
pixelox text add globe.pxo --in-place --name Source --content "Example data, invented for this layout" --font @geometric-sans --size 13 --color '#8fa0ab' --at 1400,1016 --anchor r
pixelox export globe.pxo -o globe.png
```

Reuse: to show another part of the world change `rotate=lon,lat` (the point that faces
the viewer; west and south are negative). The markers are places: run
`lab/anchors.py scripts/geo.js` with the globe's params plus
`points='-115:55:pin,-87:46:ring,-50:14:zone,-63:-16:ring2,-100:33:drop'` and it prints
the canvas position of each (here 695,376; 791,430; 1003,608; 932,799; 721,502). The pin's
tip, the centers of the rings and the disc, and the top of the drop line sit on those
points; a leader starts at its ring's edge. A point on the far side of the globe is not
listed. Each dot row's box is one dot high (1.4% H) and sits 0.7% H above its row's center;
one dot stands for ten thousand, so fill round(value / 10) dots. Swap any read-out for
another small chart of the same footprint (C-sparkline, C-progress-ring, C-percent-bar);
keep all of them white.

## 13. Variations and pairings

* **Another globe:** place a photoreal render over the drawn one (`comp place
  illustration.png --size 730x --at 800,618 --anchor c`) and keep the glow, shadow, markers,
  and leaders; or tint two or three countries on the drawn one with
  `highlight='BRA,MEX'`, in one tint.
* **Flat world instead of a globe:** C-world-map (`scripts/geo.js` without
  `projection=globe`; C-dot-map on older builds) across the middle band, read-outs above
  and below: better when places must be compared.
* **Other subjects:** a product photo, a machine, or a building as the hub with the same
  read-outs (C-hub-spokes logic with charts as the facets).
* **Light version:** ground `#eef2f4`, type `#1f2d38`, unfilled marks `#c3ccd2`, glow
  dropped; the globe needs a stronger shadow.
* Pairs with C-stat-row along the bottom edge and C-timeline under the title for a dated
  story.

## 14. Pitfalls

* **Read-outs nobody can read.** The original's notes are about 7 px on a 1600 px canvas:
  decoration, not information. Keep notes at 13 px or more and under 18 characters a line,
  or delete them.
* **Mini charts without units or scale.** A row of dots or a nameless column chart says
  nothing unless its heading names the measure and the unit ("thousands", "per year") and
  the dots have a stated scale (here 8 dots = 80 thousand). This is the sheet's
  data-integrity weak point: every read-out needs a heading with a unit, and the sheet needs
  a source line.
* **A globe used to compare places.** A globe shows half the world and squeezes everything
  near its rim: areas there look smaller than they are, and a marker on the far side is
  not drawn at all. Use it as a picture with a few markers near the middle; when values by
  place must be compared, use a flat C-world-map.
* **Columns below zero without a zero line or a unit.** A trough only reads as "decline"
  when the zero line is drawn and the heading says what changes against what (here:
  change on the year before). Never clip negative periods to zero.
* **Colored read-outs.** The design depends on the globe owning all color; a blue chart or
  a green icon in the margins breaks the hierarchy at once.
* **Leaders crossing each other or the title.** One bend each, never crossing; move the
  read-out, not the line.
