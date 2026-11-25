---
id: i034
title: Top-five ranking poster with zig-zag cutouts
kind: [list, statistical]
topic: [sports]
mood: [bold, energetic, dark, vibrant]
style: [photo-led, flat]
format: portrait
structure: S-stat-poster
data: [ranking]
charts: [C-ranked-list, C-stat-block, C-title-block, C-source-line]
palette: ['#3f1341', '#6d4b74', '#ffffff', '#fce247', '#0d3cf5', '#fd3e36', '#3e0b7a']
needs: [photo, subject-cutout, logo]
---

# i034: Top-five ranking poster with zig-zag cutouts

## 1. Concept

A tall fan poster that ranks five people by one career total. A stadium photo tinted to a
single deep purple fills the sheet; five full-color cutouts of athletes overlap in a chain
that steps down the page from top center to bottom center, swinging left and right; beside
each figure stands a small text block: a rank number in a colored tile, a two-line name, a
tiny label, a list, and the total in yellow. The reader takes away "these five, in this
order, with these totals". It works because the backdrop is one hue, so the full-color
figures are the only natural color on the page, and because yellow is used for exactly two
things: the title's hook and the five answers.

## 2. Data story

One ranking: **5 people, one count each** (a career total in the low hundreds), sorted from
largest to smallest. The first is clearly ahead; the second and third are close to each
other; the fourth and fifth are close again, a step below. Each person also has a short
text list (1 to 3 organizations they played for). The values are printed, not drawn: there are no bars, so the
only relationships shown are order (rank tiles, top to bottom) and the printed numbers.

Placeholder data in the recipe (example data, invented: stage wins in an invented national
cycling tour):

| Rank | Name (two lines) | List | Total |
| --- | --- | --- | --- |
| 1 | Rider / Alpha | North Team, Harbor Team, Lake Team | 171 wins |
| 2 | Rider / Bravo | Valley Team, Coast Team | 134 wins |
| 3 | Rider / Charlie | Hill Team | 126 wins |
| 4 | Rider / Delta | Coast Team, River Team | 108 wins |
| 5 | Rider / Echo | Lake Team, Dune Team | 103 wins |

## 3. Information architecture

1. **Emblem and title block**, top: a white emblem at the left names the competition; the
   tilted title says "top 5" in yellow, then what is counted in two white lines; a pill tag
   names the kind of piece.
2. **Rank 1**, top center: the figure's head is right under the title; its text block
   stands to the left.
3. **Rank 2**, right: text block at the right edge, figure center right.
4. **Rank 3**, left: text block at the left, figure center left, at the same height as
   rank 2's figure.
5. **Rank 4**, lower left, and **rank 5**, lower right: the two front figures, cut by the
   bottom edge.
6. **Source block**, bottom left.

The reader follows the numbers 1 to 5, not a straight line: the tiles alternate left,
right, left, left, right, and each sits level with its figure's head. Every block has the
same five lines in the same order, so after the first one the others are read at a glance.

## 4. Format and grid

* Portrait, about 2:3 (1 : 1.53). Recipe canvas 1080 x 1660. Margins 5% W left, 4% W right.
* No column grid: a center lane from 25% to 75% W holds the figures; two outer lanes hold
  the text blocks (left blocks end at 23 to 36% W, right blocks start at 76 to 77% W).
* Emblem box `6.3%,7.2%,18.4% x 15.2%`.
* Title block, tilted 7 degrees counter-clockwise (it rises to the right): yellow line
  centered at (43.8%, 9.1%), 37.5% W wide; two white lines starting at 27% W, 49% and 63% W
  wide, block center (58%, 15.8%); pill 24.4% W x 3.4% H centered at (73%, 19.8%).
* Sparks: 83 to 99% W, 0 to 10.5% H.
* Rank tile 4.9% W x 4.7% H (53 x 78 px, taller than wide). Tile top-left corners:
  rank 1 `30.8%,22.7%`; rank 2 `76.1%,29.1%`; rank 3 `22.2%,43.6%`; rank 4 `17.4%,69%`;
  rank 5 `77.1%,65.1%`. A block is 15 to 17.5% H tall (tile, 1.8% H gap, name, label, list,
  total). Blocks left of the figures are right-aligned on the tile's right edge; blocks
  right of the figures are left-aligned on the tile's left edge.
* Figures (head top, center x, width): rank 1 at 18.3% H, 54% W, 34% W wide; rank 3 at 38.3%
  H, 37.5% W; rank 2 at 38.7% H, 68% W (one arm reaches 95% W); rank 4 at 53.8% H, 39% W;
  rank 5 at 66% H, 62.5% W. Each is 33 to 45% W wide and 30 to 46% H tall; ranks 4 and 5 run
  off the bottom edge. Each figure overlaps the one above by a quarter to a third.
* Source block: left edge 5.1% W, heading at 95.4% H, line at 97.5% H.

## 5. Layer stack

1. A stadium photo, full bleed: stands and roof in the top 30%, pitch below, a few distant
   figures. Mapped to one hue: shadows `#2c0b30`, midtones `#3f1341` to `#4c224f`, lights
   `#6d4b74` to `#8a6690`. Low contrast, so type reads anywhere on it.
2. Cutout, rank 1 (full color), behind all other figures.
3. Cutout, rank 3; then rank 2 (its shoulder covers rank 3's arm); then rank 4; then rank 5
   in front of everything.
4. White emblem, top left.
5. Title: the yellow line `#fce247` with a hard violet shadow `#3e0b7a` 6 px left and 7 px
   down; the two white lines with the same shadow 4 px left, 5 px down; all tilted -7 degrees.
6. Pill tag: a rounded rectangle in translucent lilac (reads as `#9a84a0`), same tilt, dark
   purple caps `#33143a`.
7. Sparks: four white dashes 5 px thick at about 27 degrees, a small x, and a dot, top right.
8. Five rank blocks: tile `#0d3cf5` (ranks 1, 3, 5) or `#fd3e36` (ranks 2, 4) with a white
   digit; white name; pale lilac `#f3dcf5` label; white list; yellow total.
9. Source block in white, bottom left.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#3f1341` | photo ground, dark and mid tones | 45% | nothing: backdrop |
| `#6d4b74` | photo ground, light tones (stands, haze) | 15% | nothing: backdrop |
| `#ffffff` | emblem, title lines 2 and 3, names, lists, digits, sparks | 6% | nothing: text |
| `#fce247` | title hook, the five totals | 2% | emphasis: the answer |
| `#0d3cf5` | rank tiles 1, 3, 5 | 0.7% | nothing: alternation |
| `#fd3e36` | rank tiles 2, 4 | 0.5% | nothing: alternation |
| `#3e0b7a` | hard shadow of the title | 1% | nothing: depth |

The five cutouts keep their natural colors and cover about 30% of the sheet. Scheme: one
dark hue for everything that is not content, white for words, one accent (yellow) for the
hook and the values, and two saturated tile colors that only alternate. The lilac of the
pill (`#9a84a0`) and of the small labels (`#f3dcf5`) are tints of the ground.

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Title hook | 1 word and a digit | caps | `@heavy-wide` (Archivo Black / Arial Black) | 900 | 125 | 40 | - | center | `#fce247` | center 43.8%, 9.1%; tilted -7 |
| Title lines | 2 lines of 2 or 3 words (16 and 18 letters) | caps | `@heavy-wide` | 900 | 51 (45 in Arial Black, which runs wider) | 0 | 56 px | left | `#ffffff` | center 58%, 15.8%; tilted -7 |
| Pill | 1 word, 11 letters | caps | `@geometric-sans` (Poppins / Avenir Next) | 600 | 21 | 150 | - | center | `#33143a` | pill center 73%, 19.8% |
| Tile digit (x5) | 1 digit | - | `@heavy-wide` | 900 | 45 | 0 | - | center | `#ffffff` | tile center |
| Name (x5) | first name, then surname: 2 lines of 4 to 7 letters | caps | `@heavy-wide` | 900 | 30 | 0 | 37 px | right or left | `#ffffff` | cap top 6.5% H under the tile top |
| Label (x5) | 3 words, the same in every block | caps | `@condensed` (Oswald / Avenir Next Condensed) | 400 | 15 | 120 | - | right or left | `#f3dcf5` | 11.4% H under the tile top |
| List (x5) | 1 to 3 names, comma-separated, 1 or 2 lines | title | `@neo-grotesque` (Archivo, Inter / Helvetica Neue) | 400 | 16 | 0 | 23 px | right or left | `#ffffff` | 13% H under the tile top |
| Total (x5) | 3 digits and a unit word | caps | `@heavy-wide` | 900 | 21 | 0 | - | right or left | `#fce247` | 15% H (one list line) or 16.7% H (two) under the tile top |
| Source heading | 1 word | caps | `@condensed` | 600 | 18 | 100 | - | left | `#ffffff` | 5.1%, 95.2% |
| Source line | one reference, 35 to 45 characters | caps | `@condensed` | 400 | 15 | 60 | - | left | `#ffffff` | 5.1%, 97.3% |

Three families: a heavy wide grotesque for everything that shouts (hook, title, digits,
names, totals), a condensed sans for the small caps labels, a neutral sans for the lists.
Sizes 125 : 51 : 30 : 21 : 15, so the hook is 4 times a name and a name is twice a list line.

## 8. Charts and diagrams

There is no drawn chart. The poster is a **hand-placed relative of C-ranked-list**.

* **C-ranked-list** in the library is a straight list built with `scripts/steps.js`
  (`dir=v`, `connector=none`): five rows of equal pitch, a round rank badge at the left, the
  name and a note beside it, a right-aligned value column, hairlines between rows. This
  poster keeps the same five fields per item (rank, name, note, value) but:
  * the rows are **not aligned**: each block is placed beside its own cutout, at the height
    of that figure's head, alternately left and right of the figure chain;
  * the badge is a **tall rounded tile** (53 x 78 px, radius 4) above the name, not a disc
    beside it, and the tiles alternate between two colors;
  * the value sits **under** the name as the last line of the block (C-stat-block turned
    upside down: label and list first, the number last, in the accent color);
  * there are no hairlines and no connector: the overlapping figures are the connector.
* Each block is five `text add` layers and one `shape rect`, all hung from the tile's
  top-left corner (x0, y0). Left block (right-aligned, xr = x0 + 53): digit center
  (x0 + 26, y0 + 39); name `--at xr,y0+108 --anchor tr --anchor-to ink`; label center line
  y0 + 190; list y0 + 216 (one line) or y0 + 228 (the middle of two lines); total y0 + 250
  or y0 + 278. A right block uses the same offsets with xl = x0 and left anchors.
* **C-title-block**, tilted variant: no accent bar and no rule; a two-size title with a
  hard shadow, rotated as one group, plus a pill tag.
* **C-source-line**: a two-line block (heading, reference) in the bottom-left corner.
* Nothing is drawn to scale, so nothing was simplified. If the gaps between the totals
  matter, use the library's C-ranked-list with C-bars beside it instead of this layout.

## 9. Icons and illustration

* **Backdrop photo** (`photo.png`, your image here): a wide stadium interior shot from a
  low corner, stands across the top third, an empty or nearly empty pitch below. Any
  photo with large calm areas works, because it is reduced to one hue.
* **Five cutouts** (the recipe alternates `subject.png` and `subject2.png`; supply five
  different cutouts, one per ranked person, your images here): athletes in match kit,
  three-quarter length (head to knee), each in a different pose and facing a different way:
  rank 1 upright and frontal; rank 3 turned right, shoulders hunched in a run; rank 2 arms
  spread, pointing right; rank 4 mid-stride, smiling, leaning right; rank 5 frontal,
  walking toward the viewer. Five kits of five different colors (one striped, four plain),
  which is what makes the chain readable as five people.
  Edges are clean, with no outline, glow, or drop shadow. Heads are 7 to 9% W wide.
* **Emblem** (`logo.png`, your image here): a one-color white mark, 18% W wide. Recolor any
  one-color logo with `fx overlay --layer Emblem --kind color --color '#ffffff'`.
* **Sparks:** four dashes, an x, and a dot in white, drawn with `shape path` and
  `shape ellipse`. No icons.

## 10. Effects and finish

* **One-hue backdrop:** `adjust gradient-map --stops '#2c0b30@0,#472049@0.45,#8a6690@1'
  --as-layer`, added right after the photo so it tints only the photo (an adjustment layer
  changes what lies below it). A style gives one hue too, but keeps the photo's full
  contrast, so bright areas stay bright behind the type:
  `fx overlay --layer Stadium --kind color --color '#4a1a52' --blend color --opacity 100`.
* **Hard shadow on the title:** `fx drop-shadow --offset=-6,7 --blur 0 --color '#3e0b7a'
  --blend normal --opacity 100` (no blur: a second, shifted copy of the letters).
* **Tilt:** `transform rotate --layer L --degrees=-7` on the hook, the title lines, the
  pill, and the pill text, each about its own center.
* **Fading bases (stand-ins only):** the stand-in cutouts are busts with a flat lower edge,
  so the recipe fades the lowest third of ranks 1 to 4 with
  `mask add --layer P1 --gradient linear --from 0,650 --to 0,833`. Real three-quarter
  cutouts need no fade: the next figure covers each lower edge.
* Tiles have a 4 px corner radius; the pill 14 px. Everything else is flat.

## 11. Hierarchy and eye path

1. The yellow hook (the largest type, the only yellow at the top, tilted).
2. The face of rank 1 directly below it, then down the chain of faces: left, right, left,
   right.
3. The blue and red tiles (small, but the most saturated marks on the page), in number order.
4. The white names, then the yellow totals under them.
5. The emblem, the pill, the source block.

Hook 125 px, title 51, digits 45, names 30, totals 21, lists and labels 15 to 16: the
totals are small for a ranking's key values and are carried by color alone.

## 12. Build recipe

Supply `photo.png` (the backdrop, your image here), five cutouts with transparency (the
recipe places `subject.png` three times and `subject2.png` twice: replace each with your
own image), and `logo.png` (your image here). Canvas 1080 x 1660. All names and figures are
example data, invented.

```bash
pixelox new -o top5.pxo --size 1080x1660 --fill '#3f1341'
pixelox comp place top5.pxo photo.png --in-place --as Stadium --fit cover
pixelox adjust gradient-map top5.pxo --in-place --stops '#2c0b30@0,#472049@0.45,#8a6690@1' --as-layer --name Tint
pixelox comp place top5.pxo subject.png --in-place --as P1 --size 400x --at 590,300 --anchor t
pixelox mask add top5.pxo --in-place --layer P1 --gradient linear --from 0,650 --to 0,833
pixelox comp place top5.pxo subject2.png --in-place --as P3 --size 400x --at 410,620 --anchor t
pixelox mask add top5.pxo --in-place --layer P3 --gradient linear --from 0,970 --to 0,1153
pixelox comp place top5.pxo subject.png --in-place --as P2 --size 380x --at 700,640 --anchor t
pixelox mask add top5.pxo --in-place --layer P2 --gradient linear --from 0,960 --to 0,1147
pixelox comp place top5.pxo subject.png --in-place --as P4 --size 420x --at 430,900 --anchor t
pixelox mask add top5.pxo --in-place --layer P4 --gradient linear --from 0,1290 --to 0,1460
pixelox comp place top5.pxo subject2.png --in-place --as P5 --size 500x --at 690,1090 --anchor t
pixelox comp place top5.pxo logo.png --in-place --as Emblem --size 210x --at 167,245 --anchor c
pixelox text add top5.pxo --in-place --name Top --content "TOP 5" --font @heavy-wide --size 125 --tracking 40 --color '#fce247' --at 472,151 --anchor c
pixelox fx drop-shadow top5.pxo --in-place --layer Top --offset=-6,7 --blur 0 --color '#3e0b7a' --blend normal --opacity 100
pixelox transform rotate top5.pxo --in-place --layer Top --degrees=-7
pixelox text add top5.pxo --in-place --name Sub --content $'CAREER STAGE WINS\nIN THE NATIONAL TOUR' --font @heavy-wide --size 45 --leading 56 --color '#ffffff' --at 630,262 --anchor c
pixelox fx drop-shadow top5.pxo --in-place --layer Sub --offset=-4,5 --blur 0 --color '#3e0b7a' --blend normal --opacity 100
pixelox transform rotate top5.pxo --in-place --layer Sub --degrees=-7
pixelox shape rect top5.pxo --in-place --name Pill --rect 660,300,263x57 --radius 14 --fill '#9a84a0'
pixelox transform rotate top5.pxo --in-place --layer Pill --degrees=-7
pixelox text add top5.pxo --in-place --name PillT --content "RANKING" --font @geometric-sans --weight 600 --size 21 --tracking 150 --color '#33143a' --at 791,328 --anchor c
pixelox transform rotate top5.pxo --in-place --layer PillT --degrees=-7
pixelox shape path top5.pxo --in-place --name Sparks --d 'M 940 82 L 1010 46 M 985 86 L 1062 44 M 905 130 L 975 94 M 1030 20 L 1066 2 M 899 99 L 913 113 M 913 99 L 899 113' --fill none --stroke '#ffffff' --stroke-width 5
pixelox shape ellipse top5.pxo --in-place --name SparkDot --rect 1059,160,12x12 --fill '#ffffff'
pixelox shape rect top5.pxo --in-place --name T1 --rect 332,377,53x78 --radius 4 --fill '#0d3cf5'
pixelox text add top5.pxo --in-place --name N1 --content "1" --font @heavy-wide --size 45 --color '#ffffff' --at 358,416 --anchor c --anchor-to ink
pixelox text add top5.pxo --in-place --name Name1 --content $'RIDER\nALPHA' --font @heavy-wide --size 30 --leading 37 --align right --color '#ffffff' --at 385,485 --anchor tr --anchor-to ink
pixelox text add top5.pxo --in-place --name Lab1 --content "TEAMS RIDDEN FOR" --font @condensed --size 15 --tracking 120 --color '#f3dcf5' --at 385,567 --anchor r
pixelox text add top5.pxo --in-place --name List1 --content $'North Team,\nHarbor Team, Lake Team' --font @neo-grotesque --size 16 --leading 23 --align right --color '#ffffff' --at 385,605 --anchor r
pixelox text add top5.pxo --in-place --name Tot1 --content "171 WINS" --font @heavy-wide --size 21 --color '#fce247' --at 385,655 --anchor r
pixelox shape rect top5.pxo --in-place --name T2 --rect 822,483,53x78 --radius 4 --fill '#fd3e36'
pixelox text add top5.pxo --in-place --name N2 --content "2" --font @heavy-wide --size 45 --color '#ffffff' --at 848,522 --anchor c --anchor-to ink
pixelox text add top5.pxo --in-place --name Name2 --content $'RIDER\nBRAVO' --font @heavy-wide --size 30 --leading 37 --color '#ffffff' --at 822,591 --anchor tl --anchor-to ink
pixelox text add top5.pxo --in-place --name Lab2 --content "TEAMS RIDDEN FOR" --font @condensed --size 15 --tracking 120 --color '#f3dcf5' --at 822,673 --anchor l
pixelox text add top5.pxo --in-place --name List2 --content $'Valley Team,\nCoast Team' --font @neo-grotesque --size 16 --leading 23 --color '#ffffff' --at 822,711 --anchor l
pixelox text add top5.pxo --in-place --name Tot2 --content "134 WINS" --font @heavy-wide --size 21 --color '#fce247' --at 822,761 --anchor l
pixelox shape rect top5.pxo --in-place --name T3 --rect 240,724,53x78 --radius 4 --fill '#0d3cf5'
pixelox text add top5.pxo --in-place --name N3 --content "3" --font @heavy-wide --size 45 --color '#ffffff' --at 266,763 --anchor c --anchor-to ink
pixelox text add top5.pxo --in-place --name Name3 --content $'RIDER\nCHARLIE' --font @heavy-wide --size 30 --leading 37 --align right --color '#ffffff' --at 293,832 --anchor tr --anchor-to ink
pixelox text add top5.pxo --in-place --name Lab3 --content "TEAMS RIDDEN FOR" --font @condensed --size 15 --tracking 120 --color '#f3dcf5' --at 293,914 --anchor r
pixelox text add top5.pxo --in-place --name List3 --content "Hill Team" --font @neo-grotesque --size 16 --color '#ffffff' --at 293,940 --anchor r
pixelox text add top5.pxo --in-place --name Tot3 --content "126 WINS" --font @heavy-wide --size 21 --color '#fce247' --at 293,974 --anchor r
pixelox shape rect top5.pxo --in-place --name T4 --rect 188,1146,53x78 --radius 4 --fill '#fd3e36'
pixelox text add top5.pxo --in-place --name N4 --content "4" --font @heavy-wide --size 45 --color '#ffffff' --at 214,1185 --anchor c --anchor-to ink
pixelox text add top5.pxo --in-place --name Name4 --content $'RIDER\nDELTA' --font @heavy-wide --size 30 --leading 37 --align right --color '#ffffff' --at 241,1254 --anchor tr --anchor-to ink
pixelox text add top5.pxo --in-place --name Lab4 --content "TEAMS RIDDEN FOR" --font @condensed --size 15 --tracking 120 --color '#f3dcf5' --at 241,1336 --anchor r
pixelox text add top5.pxo --in-place --name List4 --content $'Coast Team,\nRiver Team' --font @neo-grotesque --size 16 --leading 23 --align right --color '#ffffff' --at 241,1374 --anchor r
pixelox text add top5.pxo --in-place --name Tot4 --content "108 WINS" --font @heavy-wide --size 21 --color '#fce247' --at 241,1424 --anchor r
pixelox shape rect top5.pxo --in-place --name T5 --rect 833,1081,53x78 --radius 4 --fill '#0d3cf5'
pixelox text add top5.pxo --in-place --name N5 --content "5" --font @heavy-wide --size 45 --color '#ffffff' --at 859,1120 --anchor c --anchor-to ink
pixelox text add top5.pxo --in-place --name Name5 --content $'RIDER\nECHO' --font @heavy-wide --size 30 --leading 37 --color '#ffffff' --at 833,1189 --anchor tl --anchor-to ink
pixelox text add top5.pxo --in-place --name Lab5 --content "TEAMS RIDDEN FOR" --font @condensed --size 15 --tracking 120 --color '#f3dcf5' --at 833,1271 --anchor l
pixelox text add top5.pxo --in-place --name List5 --content "Lake Team, Dune Team" --font @neo-grotesque --size 16 --color '#ffffff' --at 833,1297 --anchor l
pixelox text add top5.pxo --in-place --name Tot5 --content "103 WINS" --font @heavy-wide --size 21 --color '#fce247' --at 833,1331 --anchor l
pixelox text add top5.pxo --in-place --name SrcH --content "SOURCE" --font @condensed --weight 600 --size 18 --tracking 100 --color '#ffffff' --at 55,1581 --anchor l
pixelox text add top5.pxo --in-place --name Src --content "EXAMPLE DATA, INVENTED FOR THIS RECIPE" --font @condensed --size 15 --tracking 60 --color '#ffffff' --at 55,1615 --anchor l
pixelox export top5.pxo -o top5.png
```

Reuse: change the hook, the two title lines (keep the longer one to 20 characters at this
size, or lower `--size`), then the five blocks. Move a block by changing its tile's
`--rect` and shifting its five text layers by the same amount (the offsets are in section
8). Place cutouts back to front in the order 1, 3, 2, 4, 5, each `--anchor t` at its head
position; put each tile level with its figure's head on the side the figure faces away
from. With real three-quarter cutouts, delete the four `mask add` lines and size ranks 4
and 5 so they run past the bottom edge (`--size x800`). The source line states where the
totals come from and the date they were counted.

## 13. Variations and pairings

* **Aligned list instead of a zig-zag:** when the five values must be compared by eye, use
  the library's C-ranked-list (rows from `scripts/steps.js`) with C-bars beside the names,
  and keep only the rank 1 cutout as a hero (S-stat-poster with one figure).
* **Top 3:** a podium: rank 1 in the center and largest, ranks 2 and 3 lower at the left
  and right, blocks under each figure; same tiles, same title block.
* **Other subjects:** products (five cutout bottles with units sold), employees of the
  year, cities (five skyline cutouts): anything with one cutout and one number per item.
* **Palette swaps:** map the backdrop to the organization's darkest color (navy, forest,
  maroon) and keep one warm accent for hook and totals; keep the tile pair saturated and
  unlike the ground. For a light version use Paper and ember from
  [../palettes.md](../palettes.md) with a pale gradient map and dark type.
* **Square post:** ranks 1 to 3 only on 1080 x 1080, title block reduced to 80 px.
* Pairs with C-stat-row (three career facts of rank 1 across the bottom) and C-timeline
  (when each record was set).

## 14. Pitfalls

* **A ranking without a date or scope.** An all-time table changes while people on it are
  still active, and "top" depends on the competition counted. Print the cut-off date and
  the source in the source block, and name the unit in every total ("171 wins").
* **Ties and near-ties hidden by rank tiles.** Two totals 5 apart look as far apart as two
  totals 50 apart, because nothing is drawn to scale. If the gaps are the story, add
  C-bars or print the difference to the next rank; give equal totals the same rank number.
* **Tile colors read as groups.** Blue and red alternate by rank only. Readers look for a
  meaning (two teams, two eras). Use one tile color, or make the colors mean something
  and add a key.
* **Figure size read as importance.** Rank 5 stands in front and is drawn largest here.
  Keep all five heads the same size (7 to 9% W) and let position, not scale, carry order;
  or scale rank 1 up on purpose and say so.
* **Totals too small.** At 21 px the key numbers are a sixth of the hook. On a phone they
  vanish: raise them to 28 to 34 px before shrinking names or lists.
* **Flat-based cutouts.** A bust with a straight lower edge floats in mid-air. Hide every
  base behind the next figure, run it off the canvas, or fade it with a gradient mask.
* **Text over busy photo areas.** The one-hue map keeps contrast low, but floodlights and
  white lines can still sit behind a name. Check each block on the export and move it, or
  darken the map's light stop.
