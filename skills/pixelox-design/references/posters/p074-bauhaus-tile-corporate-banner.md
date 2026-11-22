---
id: p074
title: Bauhaus tile grid corporate banner
genre: [corporate, conference, event-flyer]
mood: [bold, playful, energetic, retro]
style: [geometric, flat, swiss]
format: landscape
archetype: A-split-field
palette: ['#efefef', '#ff162e', '#56ffff', '#666be3', '#5b2690', '#fed403']
techniques: [T-geo-shapes, T-tile-mosaic]
needs: nothing
---

# p074: Bauhaus tile grid corporate banner

## 1. Concept

A landscape announcement split in two: on the left a pale gray field with a quiet,
left-aligned text column (event type, red title, date, small paragraph); on the right a
full-height mosaic of square tiles, each holding one elementary shape (quarter disc,
circle, four dots, triangle, half split) in loud flat colors. It works because the two
halves are opposites, calm text against a noisy pattern, and because the mosaic is strict:
one tile size, five shapes, eight colors, so the noise still reads as order.

## 2. Format and grid

* Landscape, about 4:3 in the original; built here at 16:9. Left margin 6.3% W, top
  margin 8% H.
* Vertical split at 55% W: text field 0 to 55% W, tile mosaic 55 to 100% W.
* Mosaic: square tiles whose side is one fifth of the canvas height (20% H), 5 rows by 4
  columns at 16:9 (3 columns in the original), bleeding off the top, right, and bottom
  edges with no gaps.
* Text column: all lines hang from 6.3% W; the widest line (the sub-title) is 40% W, so
  15% W of empty gray separates text and mosaic.
* Vertical rhythm of the text: label 8 to 12% H; title 17 to 34% H; sub-title 37 to 43%
  H; date block 49 to 76% H; paragraph 83 to 96% H. Gaps of 6 to 7% H split it into three
  groups (label and title, date, paragraph).

## 3. Layer stack

1. Ground: pale gray `#efefef`.
2. Mosaic base: a red `#ff162e` rectangle, 55 to 100% W, full height (red tiles are simply
   left uncovered).
3. Tile grounds in the other colors: dark red `#b9212e`, cyan `#56ffff`, purple
   `#5b2690`, periwinkle `#666be3`, yellow `#fed403`.
4. Motifs, one per tile, always in a color close to its tile's ground or its opposite:
   * quarter disc with radius = tile side, centered on a tile corner (periwinkle on
     purple, purple on periwinkle, periwinkle on cyan, cyan on dark red, red on cyan);
     two neighboring quarter discs join into a half disc across the tile border;
   * full circle, diameter 90% of the tile (orange `#fe7400` on yellow);
   * four small circles in a 2 by 2 arrangement, each 48% of the tile (dark red on red);
   * diagonal half (a right triangle; dark red on red, teal `#0a8ec0` on cyan, purple on
     periwinkle);
   * vertical half split (orange and yellow; red and dark red).
5. Label: one word of thin wide caps, dark gray, at 6.3% W, cap top 7.9% H.
6. Title: two lines of heavy red caps, cap tops 17% and 26.2% H.
7. Sub-title: one line of light italic, dark gray, at 40% H.
8. Date block: weekday in bold title case (52% H); month and day in heavy black caps
   (cap top 56% H) with a raised ordinal suffix; year below (cap top 67.6% H).
9. Paragraph: five lines of small gray text, 83 to 96% H, 38% W wide.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#efefef` | text field | 55% |
| `#ff162e` | red tiles, title (with dark red `#b9212e`) | 14% |
| `#56ffff` | cyan tiles (with teal `#0a8ec0`) | 9% |
| `#666be3` | periwinkle tiles and quarters | 8% |
| `#5b2690` | purple tiles and quarters | 6% |
| `#fed403` | yellow tiles (with orange `#fe7400`) | 6% |
| `#111111` | date block and body | 2% |

Three warm colors (red, orange, yellow) against three cool ones (cyan, periwinkle,
purple), each with one darker partner. The title takes the mosaic's red; all other type
is black or dark gray.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Label | 1 word, 9 letters | caps | thin wide geometric sans (Montserrat 300 / Avenir Light) | 300 | 4.3% | 40 | - | left | `#222222` | 6.3%, cap top 7.9% |
| Title | 2 words on 2 lines, 4 to 6 letters each | caps | heavy wide geometric sans (Montserrat 800 / Avenir Next Heavy) | 900 | 7.3% | 10 | line gap 25% of cap height | left | `#ff162e` | 6.3%, cap tops 17% and 26.2% |
| Sub-title | 2 words | title | light italic geometric sans (Montserrat 300 italic / Avenir Light Oblique) | 300 italic | 3.9% | 20 | - | left | `#222222` | 6.3%, 40% |
| Weekday | 1 word | title | geometric sans (Montserrat 700 / Avenir Next) | 700 | 4.1% | 0 | - | left | `#111111` | 6.3%, 52% |
| Month and day | "MAY 21" style, with a raised 2-letter suffix at 45% size | caps | heavy wide geometric sans | 900 | 9% | 10 | - | left | `#111111` | 6.3%, cap top 56% |
| Year | 4 digits | numerals | same | 900 | 8.3% | 10 | - | left | `#111111` | 6.3%, cap top 67.6% |
| Paragraph | 5 lines, about 11 words each | sentence | neo-grotesque (Inter 400 / Helvetica Neue) | 400 | 1.5% | 0 | 130% | left | `#444444` | 6.3%, top 83% |

These are the values for the 16:9 build. The original, at about 4:3 with three tile
columns, uses cap heights of 3.4, 5.6, 3, 3.2, 7, 6.4, and 1.3% H: on a wider canvas the
type is set 1.3x larger so that the text column keeps its share of the width.

One geometric family from thin to heavy plus a plain text face. The date (cap height 9%
H) is larger than the title (7.3% H): on an announcement the date is the news.

## 6. Imagery

No photograph. The mosaic is a pattern in the Bauhaus tile tradition: identical squares,
each cut by one arc, circle, or diagonal. Neighboring tiles sometimes complete each other
(two quarter discs make a dome; a quarter and a triangle make a leaf), which creates
larger shapes two tiles wide without breaking the grid.

## 7. Effects and textures

* **T-geo-shapes**, on a strict tile unit: every tile ground of one color goes into one
  `shape path` (several `M x y h T v T h -T Z` squares), every motif of one color into
  another. Ten path layers draw the whole 20-tile mosaic.
* Quarter disc in a tile with top-left corner (x, y) and side T, centered on the
  bottom-right corner: `M x+T y A T T 0 0 0 x y+T L x+T y+T Z`. For the bottom-left
  corner: `M x y A T T 0 0 1 x+T y+T L x y+T Z`.
* Layer order decides what a motif can sit on: grounds first, motifs after, and a color
  that serves as both (purple here) gets two layers.
* The mosaic is a hand-built repeat pattern: the `pattern.js` generator (T-pattern)
  draws dots, stripes, grids, and rings, not tiles like these.
* Completely flat. No gradients, shadows, or grain.
* **T-tile-mosaic** in [techniques.md](../techniques.md) is the tested library version of the tile mosaic (`scripts/tiles.js` generates one); start from it when reusing this idea.

## 8. Hierarchy and eye path

1. The mosaic (45% of the area, all the color).
2. The red title on the left, linked to the mosaic by color.
3. The black date block, the heaviest type.
4. Label, sub-title, paragraph.

## 9. Build recipe

Nothing to supply. Canvas 1600x900; tile side 180 px; mosaic from x 880.

```bash
pixelox new -o banner.pxo --size 1600x900 --fill '#efefef'
pixelox shape rect banner.pxo --in-place --name RedBase --rect 880,0,720x900 --fill '#ff162e'
pixelox shape path banner.pxo --in-place --name DarkRed --d 'M 1240 720 h 180 v 180 h -180 Z M 880 0 L 1060 0 L 1060 180 Z M 882 405 A 43 43 0 1 1 968 405 A 43 43 0 1 1 882 405 Z M 882 495 A 43 43 0 1 1 968 495 A 43 43 0 1 1 882 495 Z M 972 405 A 43 43 0 1 1 1058 405 A 43 43 0 1 1 972 405 Z M 972 495 A 43 43 0 1 1 1058 495 A 43 43 0 1 1 972 495 Z M 1330 540 h 90 v 180 h -90 Z M 1422 585 A 43 43 0 1 1 1508 585 A 43 43 0 1 1 1422 585 Z M 1422 675 A 43 43 0 1 1 1508 675 A 43 43 0 1 1 1422 675 Z M 1512 585 A 43 43 0 1 1 1598 585 A 43 43 0 1 1 1512 585 Z M 1512 675 A 43 43 0 1 1 1598 675 A 43 43 0 1 1 1512 675 Z M 1062 765 A 43 43 0 1 1 1148 765 A 43 43 0 1 1 1062 765 Z M 1062 855 A 43 43 0 1 1 1148 855 A 43 43 0 1 1 1062 855 Z M 1152 765 A 43 43 0 1 1 1238 765 A 43 43 0 1 1 1152 765 Z M 1152 855 A 43 43 0 1 1 1238 855 A 43 43 0 1 1 1152 855 Z' --fill '#b9212e'
pixelox shape path banner.pxo --in-place --name Cyan --d 'M 1060 0 h 180 v 180 h -180 Z M 1060 360 h 180 v 180 h -180 Z M 1420 360 h 180 v 180 h -180 Z M 880 540 h 180 v 180 h -180 Z M 1420 720 A 180 180 0 0 0 1240 900 L 1420 900 Z' --fill '#56ffff'
pixelox shape path banner.pxo --in-place --name PurpleTiles --d 'M 1420 0 h 180 v 180 h -180 Z M 880 180 h 180 v 180 h -180 Z' --fill '#5b2690'
pixelox shape path banner.pxo --in-place --name Periwinkle --d 'M 1060 180 h 180 v 180 h -180 Z M 1060 540 h 180 v 180 h -180 Z M 1420 720 h 180 v 180 h -180 Z M 1060 0 A 180 180 0 0 1 1240 180 L 1060 180 Z M 1600 0 A 180 180 0 0 0 1420 180 L 1600 180 Z M 1060 180 A 180 180 0 0 0 880 360 L 1060 360 Z M 1420 180 A 180 180 0 0 1 1240 360 L 1240 180 Z' --fill '#666be3'
pixelox shape path banner.pxo --in-place --name PurpleMotifs --d 'M 1060 180 A 180 180 0 0 1 1240 360 L 1060 360 Z M 1060 540 L 1060 720 L 1240 720 Z M 1600 720 A 180 180 0 0 1 1420 900 L 1420 720 Z' --fill '#5b2690'
pixelox shape path banner.pxo --in-place --name Yellow --d 'M 1240 0 h 180 v 180 h -180 Z M 1420 180 h 180 v 180 h -180 Z M 1240 360 h 180 v 180 h -180 Z M 880 720 h 180 v 180 h -180 Z' --fill '#fed403'
pixelox shape path banner.pxo --in-place --name Orange --d 'M 1240 0 h 90 v 180 h -90 Z M 1429 270 A 81 81 0 1 1 1591 270 A 81 81 0 1 1 1429 270 Z M 1249 450 A 81 81 0 1 1 1411 450 A 81 81 0 1 1 1249 450 Z M 889 810 A 81 81 0 1 1 1051 810 A 81 81 0 1 1 889 810 Z' --fill '#fe7400'
pixelox shape path banner.pxo --in-place --name Teal --d 'M 1420 360 L 1600 360 L 1600 540 Z M 880 540 L 1060 540 L 1060 720 Z' --fill '#0a8ec0'
pixelox shape path banner.pxo --in-place --name RedMotif --d 'M 1060 360 A 180 180 0 0 1 1240 540 L 1060 540 Z' --fill '#ff162e'
pixelox text add banner.pxo --in-place --name Label --content "COMMUNITY" --font Avenir --weight 300 --size 6% --tracking 40 --color '#222222' --at 6.3%,7.9% --anchor-to ink
pixelox text add banner.pxo --in-place --name Title --content $'OPEN\nFORUM' --font @geometric-sans --weight 900 --size 10% --tracking 10 --leading 92% --color '#ff162e' --at 6.3%,17% --anchor-to ink
pixelox text add banner.pxo --in-place --name Sub --content "Quarterly Update" --font Avenir --weight 300 --style italic --size 5.4% --tracking 20 --color '#222222' --at 6.3%,40% --anchor l
pixelox text add banner.pxo --in-place --name Weekday --content "Thursday" --font @geometric-sans --weight 700 --size 5.6% --color '#111111' --at 6.3%,52% --anchor l
pixelox text add banner.pxo --in-place --name MonthDay --content "JUNE 12" --font @geometric-sans --weight 900 --size 12.4% --tracking 10 --color '#111111' --at 6.3%,56% --anchor-to ink
pixelox text add banner.pxo --in-place --name Suffix --content "th" --font @geometric-sans --weight 900 --size 5.4% --color '#111111' --at 38%,57.6% --anchor l
pixelox text add banner.pxo --in-place --name Year --content "2031" --font @geometric-sans --weight 900 --size 11.4% --tracking 10 --color '#111111' --at 6.3%,67.6% --anchor-to ink
pixelox text add banner.pxo --in-place --name Para --content $'Join the whole team for an open hour of news, numbers, and questions.\nWe will share what shipped this quarter, what slipped and why,\nand what we plan to build next. Bring a question or send it ahead;\nevery one of them gets an answer on the day or in writing after.\nCoffee from nine, start at half past, done by eleven.' --font @neo-grotesque --weight 400 --size 2.1% --leading 130% --color '#444444' --at 6.3%,83% --anchor tl
pixelox export banner.pxo -o banner.png
```

Change first: the title and date. Then recolor the mosaic by editing the eight fills.
To rearrange tiles, move whole squares: tile (column c, row r) has its top-left corner at
x = 880 + 180 c, y = 180 r; keep each motif's coordinates inside its tile.

## 10. Variations and pairings

* Portrait A-series: mosaic as a band across the top 40% H (tile side = W / 6), text
  below at the same left margin.
* Two-color mosaic for a brand: one hue in three shades plus white, same shapes.
* Use the mosaic as a frame: a one-tile-wide border of tiles around a white card
  (A-frame-card).
* Mirror it (mosaic left, text right, right-aligned) for a facing pair of slides.
* Suits company announcements, open days, design talks, school calendars, report
  covers. Pairs with T-paper-grain for a printed feel and T-label-box for a "save the
  date" tag.

## 11. Pitfalls

* Tiles of two sizes or with gaps between them: the mosaic stops being a grid. One side
  length, edge to edge.
* Random colors per tile. Limit each tile to a pair: a color and its darker partner, or
  a warm ground with a cool motif; never three colors in one tile.
* Motifs that do not reach the tile edge (a quarter disc with radius 0.8 T). Radius must
  equal the tile side so arcs meet at the corners and can join across borders.
* A title in a color that is not in the mosaic disconnects the halves; take the title
  color from the most frequent tile color.
* Centering the text column. It hangs from one left edge; the wide empty band before the
  mosaic is deliberate.
