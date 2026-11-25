---
id: i015
title: Numbered map with a matching list below
kind: [geographic, list]
topic: [business, generic]
mood: [calm, clean, friendly, minimal]
style: [flat]
format: portrait
structure: S-map-callouts
data: [geography, ranking, text-only]
charts: [C-us-choropleth, C-icon-badge, C-callout, C-title-block]
palette: ['#9bd5cb', '#59baa7', '#ffffff', '#fece13', '#0b1f1c']
needs: nothing
---

# i015: Numbered map with a matching list below

## 1. Concept

A calm one-page "top N places" sheet on a mint ground. The upper half is an outline map of a
country drawn in white lines on the ground color, with seven regions filled in a darker
green and each tagged with a small white number disc; the lower half is a three-column list
in which the same numbers open seven short paragraphs. Three yellow sticky notes with dashed
leaders add side remarks. It works because the map answers "where" and the list answers
"what" with one shared key, the numbers, and because the page uses a single hue for
everything except the notes, so the yellow is noticed exactly three times.

## 2. Data story

* **Geography as a selection:** 7 regions out of about 50 are picked out. No values on the
  map: a region is either highlighted or not.
* **A numbered list of 7:** each item is a region's name and a paragraph of 12 to 30 words
  (2 to 6 lines of about 30 characters). The numbers run roughly west to east across the
  map, so the list order can be found on the map without searching; when the list is a real
  ranking the numbers are the ranks.
* **3 side notes** of 3 to 6 words, each tied to one of the seven regions.
* 1 title of 5 words that states the count ("The Top 7 ...") and a subheading of 4 words.

Placeholder content in the recipe (example data, invented: where a retailer opens stores):
1 Washington, 2 Colorado, 3 Minnesota, 4 Oklahoma, 5 Ohio, 6 Georgia, 7 Maine, each with a
one-sentence plan; notes on regions 2, 6, and 7.

## 3. Information architecture

1. **Title and subheading**, centered: the count and the subject.
2. **Map:** seven darker regions with numbers 1 to 7; the eye hops from number to number.
3. **Sticky notes:** three asides, read after the numbers because they sit outside the map.
4. **List:** three columns read top to bottom, then left to right; each entry starts with
   its number and name in bold, so the reader can jump from a map number straight to its
   paragraph.

The map takes the upper 57% of the page, the list the lower 38%: picture first, text second.

## 4. Format and grid

* Portrait, 1 : 1.29. Recipe canvas 1080 x 1400. Side margins 6% W.
* Title centered at 50% W, cap top 5.8% H; subheading centered, cap top 11.7% H.
* Map `4.4%,15%,90% x 42%` (the two remote regions as insets at the lower left, inside the
  box). Recipe box `9%,15.5%,88% x 43%`.
* Notes: 12% W x 4.4% H each, at `73.8%,15.8%`, `5.8%,35.5%`, `82%,55.1%` in the original
  (top right, left edge, bottom right): always outside the land. In the recipe, 12% W x 4%
  H at `74.8%,15.9%`, `5.2%,43.1%`, `82.8%,50%`.
* List: three columns starting at 6%, 37.3%, and 68.7% W, each 25% W wide, gutter 6.3% W.
  First headings at 62.6% H; entries follow each other with a gap of 1.6 line heights. The
  text flows from column to column like a newspaper (an entry may break across columns).
* Alignment: centered head and map, flush-left list.

## 5. Layer stack

1. Ground: mint `#9bd5cb`, flat.
2. Map: every region filled with the ground color and outlined in white, 2.5 px (the map
   looks like a white line drawing); coastlines detailed, islands included.
3. Seven regions filled with darker green `#59baa7`, same white outline.
4. Seven number discs: white, 24 px across, the digit in near-black, placed on the edge of
   their region (usually the west or north side) so they never hide a small region. The
   recipe uses 30 px discs on each region's logged center.
5. Three sticky notes: yellow rectangles `#fece13`, 130 x 62 px, square corners, with a
   small line of near-black text at the top left.
6. Three dashed white leaders (2.5 px, dashes 8 px, gaps 6 px), each with one right angle,
   from a note's edge to its region's number disc.
7. Title and subheading in near-black `#0b1f1c`.
8. List: bold entry heads and regular paragraphs in near-black.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#9bd5cb` | ground and plain regions | 86% | nothing (not selected) |
| `#59baa7` | highlighted regions | 5% | selected: one of the seven |
| `#ffffff` | map outlines, number discs, leaders | 4% | nothing: line work |
| `#fece13` | sticky notes | 2% | a remark: the single accent |
| `#0b1f1c` | all type | 3% | nothing |

Monochrome plus one accent: two steps of one green (ground and highlight), white lines, and
yellow only for the notes. The highlight is the same hue as the ground, about 15 points
darker, so the selection is clear but quiet.

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Title | "The Top N" and 2 words, one line | title | neo-grotesque (Inter, Archivo / Helvetica Neue) | 400 | 58 | -10 | - | center | `#0b1f1c` | 50%, cap top 5.8% |
| Subheading | 4 to 6 words | sentence | neo-grotesque | 400 | 31 | 0 | - | center | `#0b1f1c` | 50%, cap top 11.7% |
| Number (x7) | 1 digit | - | neo-grotesque | 700 | 15 | 0 | - | center | `#0b1f1c` | in the discs |
| Note (x3) | 3 to 6 words on 1 or 2 lines | sentence | neo-grotesque | 500 | 13 (12 in the original) | 0 | 120% | left | `#0b1f1c` | 8 px inside the note's top left corner |
| Entry head (x7) | number, period, region name | title | neo-grotesque | 700 | 19 | 0 | - | left | `#0b1f1c` | column left edges |
| Entry text (x7) | 12 to 30 words, 2 to 6 lines of about 30 characters | sentence | neo-grotesque | 400 | 19 | 0 | 23 px | left | `#0b1f1c` | 34 px under its head |

One family, two weights, three sizes: 58 : 31 : 19, about 3 : 1.6 : 1. The title is large
but regular weight: with so little on the page, size alone is enough.

## 8. Charts and diagrams

* **C-us-choropleth** in its two-class form: `scripts/usmap.js` (run with `render d3`)
  with `highlight=` (the seven picked states), `accent='#59baa7'`, `empty='#9bd5cb'` (the
  ground color, so plain states are only outlines), `stroke='#ffffff' strokewidth=2.5`,
  `outline='#ffffff'` (the script's parameter for the coastline: without it a map whose
  land is the ground color has no outer edge), `labels=none`, box `9,15.5,88,43`. No
  legend: the list is the legend.
* **C-icon-badge** as number discs: a white `shape ellipse` (30 px) and a digit, centered
  on each picked state's logged anchor.
* **C-callout** x3: a yellow `shape rect` note in the water or north of the border, and a
  dashed white `shape path` with one elbow that ends at the edge of the region's disc.
* Simplified: the original puts each disc on its region's boundary; the recipe puts it on
  the logged center, which works while a picked region is wider than about two discs.

**Map spec:**

* Region set: the 50 US states; the two non-contiguous ones as insets at the lower left
  (the northern one scaled down), inside the map box.
* Projection: Albers USA (curved northern border).
* Encoding: two classes only. Not selected: fill `#9bd5cb` (the ground). Selected (7
  regions): fill `#59baa7`.
* Borders: white `#ffffff`, 2.5 px on a 1080 px canvas, on every region including the coast.
* Labels: no names on the map. Each selected region gets a white disc with its list number
  (15 px, bold): 24 px on the region's boundary in the original, 30 px on its center in the
  recipe; no leader.
* Callouts: three yellow notes of about 130 x 60 px outside the land, each joined to one
  selected region's disc by a dashed white elbow line.
* Legend: none.
* Fallback for builds before 0.6.0: C-tile-map, `scripts/tilemap.js` with the picks as
  `groups=`, `empty='#9bd5cb'`, and `gap=0.07`, over an all-white `gap=0` copy for the lines.

## 9. Icons and illustration

None. The number discs and sticky notes are the only marks. If the entries have themes, a
28 px `shape icon` in near-black can follow each entry head (for example `store`, `truck`,
`building`), all one size.

## 10. Effects and finish

None: flat. No shadows on the notes, no gradients, square corners. Dashes are the only
texture.

## 11. Hierarchy and eye path

1. The title (58 px, the only large type).
2. The seven darker regions and their numbers, west to east.
3. The three yellow notes (the only warm color).
4. The list, column by column; bold heads act as anchors for scanning.

Title 58 px, subheading 31, list 19.

## 12. Build recipe

Nothing to supply. Canvas 1080 x 1400. The picks and the texts are example data.

```bash
pixelox new -o top7.pxo --size 1080x1400 --fill '#9bd5cb'
pixelox text add top7.pxo --in-place --name Title --content "The Top 7 Regions for 2026" --font @neo-grotesque --weight 400 --size 58 --tracking=-10 --color '#0b1f1c' --at 50%,81 --anchor t --anchor-to ink
pixelox text add top7.pxo --in-place --name Sub --content "Where we open new stores" --font @neo-grotesque --size 31 --color '#0b1f1c' --at 50%,164 --anchor t --anchor-to ink
pixelox render d3 scripts/usmap.js -o map.png --size 1080x1400 --param box=9,15.5,88,43 --param highlight='WA,CO,MN,OK,OH,GA,ME' --param accent='#59baa7' --param empty='#9bd5cb' --param stroke='#ffffff' --param strokewidth=2.5 --param outline='#ffffff' --param labels=none
pixelox comp place top7.pxo map.png --in-place --as Map --at 0,0
pixelox shape rect top7.pxo --in-place --name NoteA --rect 808,222,130x56 --fill '#fece13'
pixelox shape path top7.pxo --in-place --name LeadA --d 'M 938 250 L 993 250 L 993 297' --fill none --stroke '#ffffff' --stroke-width 2.5 --dash 8,6
pixelox text add top7.pxo --in-place --name NoteAt --content $'Open only\nMay to October' --font @neo-grotesque --weight 500 --size 13 --leading 120% --color '#0b1f1c' --at 816,230 --anchor tl
pixelox shape rect top7.pxo --in-place --name NoteB --rect 56,604,130x56 --fill '#fece13'
pixelox shape path top7.pxo --in-place --name LeadB --d 'M 80 604 L 80 503 L 405 503' --fill none --stroke '#ffffff' --stroke-width 2.5 --dash 8,6
pixelox text add top7.pxo --in-place --name NoteBt --content $'First flagship\nstore' --font @neo-grotesque --weight 500 --size 13 --leading 120% --color '#0b1f1c' --at 64,612 --anchor tl
pixelox shape rect top7.pxo --in-place --name NoteC --rect 894,700,130x56 --fill '#fece13'
pixelox shape path top7.pxo --in-place --name LeadC --d 'M 830 638 L 960 638 L 960 700' --fill none --stroke '#ffffff' --stroke-width 2.5 --dash 8,6
pixelox text add top7.pxo --in-place --name NoteCt --content $'New regional\noffice' --font @neo-grotesque --weight 500 --size 13 --leading 120% --color '#0b1f1c' --at 902,708 --anchor tl
pixelox shape ellipse top7.pxo --in-place --name B1 --rect 203,263,30x30 --fill '#ffffff'
pixelox text add top7.pxo --in-place --name B1t --content "1" --font @neo-grotesque --weight 700 --size 15 --color '#0b1f1c' --at 218,278 --anchor c --anchor-to ink
pixelox shape ellipse top7.pxo --in-place --name B2 --rect 405,488,30x30 --fill '#ffffff'
pixelox text add top7.pxo --in-place --name B2t --content "2" --font @neo-grotesque --weight 700 --size 15 --color '#0b1f1c' --at 420,503 --anchor c --anchor-to ink
pixelox shape ellipse top7.pxo --in-place --name B3 --rect 594,334,30x30 --fill '#ffffff'
pixelox text add top7.pxo --in-place --name B3t --content "3" --font @neo-grotesque --weight 700 --size 15 --color '#0b1f1c' --at 609,349 --anchor c --anchor-to ink
pixelox shape ellipse top7.pxo --in-place --name B4 --rect 541,572,30x30 --fill '#ffffff'
pixelox text add top7.pxo --in-place --name B4t --content "4" --font @neo-grotesque --weight 700 --size 15 --color '#0b1f1c' --at 556,587 --anchor c --anchor-to ink
pixelox shape ellipse top7.pxo --in-place --name B5 --rect 788,452,30x30 --fill '#ffffff'
pixelox text add top7.pxo --in-place --name B5t --content "5" --font @neo-grotesque --weight 700 --size 15 --color '#0b1f1c' --at 803,467 --anchor c --anchor-to ink
pixelox shape ellipse top7.pxo --in-place --name B6 --rect 800,623,30x30 --fill '#ffffff'
pixelox text add top7.pxo --in-place --name B6t --content "6" --font @neo-grotesque --weight 700 --size 15 --color '#0b1f1c' --at 815,638 --anchor c --anchor-to ink
pixelox shape ellipse top7.pxo --in-place --name B7 --rect 978,297,30x30 --fill '#ffffff'
pixelox text add top7.pxo --in-place --name B7t --content "7" --font @neo-grotesque --weight 700 --size 15 --color '#0b1f1c' --at 993,312 --anchor c --anchor-to ink
pixelox text add top7.pxo --in-place --name H1 --content "1. Washington" --font @neo-grotesque --weight 700 --size 19 --color '#0b1f1c' --at 65,877 --anchor tl
pixelox text add top7.pxo --in-place --name P1 --content "Three stores open in the spring, all within reach of the new light-rail line." --font @neo-grotesque --size 19 --leading 23 --color '#0b1f1c' --box 280x150 --at 65,911 --anchor tl
pixelox text add top7.pxo --in-place --name H2 --content "2. Colorado" --font @neo-grotesque --weight 700 --size 19 --color '#0b1f1c' --at 65,1040 --anchor tl
pixelox text add top7.pxo --in-place --name P2 --content "A first flagship in the capital, followed by two mountain towns." --font @neo-grotesque --size 19 --leading 23 --color '#0b1f1c' --box 280x150 --at 65,1074 --anchor tl
pixelox text add top7.pxo --in-place --name H3 --content "3. Minnesota" --font @neo-grotesque --weight 700 --size 19 --color '#0b1f1c' --at 65,1180 --anchor tl
pixelox text add top7.pxo --in-place --name P3 --content "Five small stores inside partner locations, opening one by one through the summer months." --font @neo-grotesque --size 19 --leading 23 --color '#0b1f1c' --box 280x150 --at 65,1214 --anchor tl
pixelox text add top7.pxo --in-place --name H4 --content "4. Oklahoma" --font @neo-grotesque --weight 700 --size 19 --color '#0b1f1c' --at 403,877 --anchor tl
pixelox text add top7.pxo --in-place --name P4 --content "One distribution hub that cuts delivery times across four neighboring states." --font @neo-grotesque --size 19 --leading 23 --color '#0b1f1c' --box 280x150 --at 403,911 --anchor tl
pixelox text add top7.pxo --in-place --name H5 --content "5. Ohio" --font @neo-grotesque --weight 700 --size 19 --color '#0b1f1c' --at 403,1040 --anchor tl
pixelox text add top7.pxo --in-place --name P5 --content "Stores in three university cities, timed for the start of the academic year and staffed by students." --font @neo-grotesque --size 19 --leading 23 --color '#0b1f1c' --box 280x150 --at 403,1074 --anchor tl
pixelox text add top7.pxo --in-place --name H6 --content "6. Georgia" --font @neo-grotesque --weight 700 --size 19 --color '#0b1f1c' --at 403,1203 --anchor tl
pixelox text add top7.pxo --in-place --name P6 --content "A second regional office and two stores close to the port." --font @neo-grotesque --size 19 --leading 23 --color '#0b1f1c' --box 280x150 --at 403,1237 --anchor tl
pixelox text add top7.pxo --in-place --name H7 --content "7. Maine" --font @neo-grotesque --weight 700 --size 19 --color '#0b1f1c' --at 742,877 --anchor tl
pixelox text add top7.pxo --in-place --name P7 --content "A seasonal store on the coast, open from May to October. Example data, invented for this layout." --font @neo-grotesque --size 19 --leading 23 --color '#0b1f1c' --box 280x150 --at 742,911 --anchor tl
pixelox export top7.pxo -o top7.png
```

Reuse: change `highlight=` to your regions (any number from 3 to 10), then read the state
anchors again (`lab/anchors.py scripts/usmap.js` with the same `--size` and `box=`, or the
`render d3` line with `--json`) and center each number disc on its state's anchor (here WA
218,278, CO 420,503, MN 609,349, OK 556,587, OH 803,467, GA 815,638, ME 993,312). Leaders
start or end at a disc's edge (15 px from its center) and have one right angle; notes go
into the water or north of the border, never on the land. The map needs pixelox 0.6.0 or
later. In the list, an entry head sits 34 px above its paragraph and the next head 1.6
lines under the paragraph's last line: count the wrapped lines (about 30 characters each
at 19 px in a 280 px box). For another country use C-world-map with `extent=` and
`highlight=`, or C-tile-map with your own `cells=`.

## 13. Variations and pairings

* **A real ranking:** number by rank instead of by position, and put the measure in each
  entry head ("1. Washington, 14 stores"). Add C-bars beside the list when the gaps matter.
* **Values on the map:** replace `highlight=` with `values=`, `bins=4`, and a ramp
  (`ramp='#c9ebe4,#1f7a68'`): the sheet becomes a choropleth with a commentary list.
* **Dark version:** ground `#12332d`, plain regions `#1c4a41`, highlight `#59baa7`, white
  type, the same yellow notes.
* **Landscape slide:** map on the left 60%, list in one column on the right, notes dropped.
* Pairs with C-stat-row under the title (three totals for all picked regions) and with
  C-ranked-list in place of the paragraphs when each entry is one line.

## 14. Pitfalls

* **Numbers that mean nothing.** If the list is not a ranking, readers still assume 1 is
  best. Either number west to east and say so ("in no particular order"), or rank by a
  stated measure. A "top 7" with no measure and no source is a claim without evidence: name
  the criterion in the subheading. This is the sheet's data-integrity rule.
* **Discs that hide their region.** A 30 px disc on the center covers most of a small
  north-eastern state. When a picked region is narrower than about two discs, move its
  disc onto the boundary or just outside it, as the original does for all seven.
* **Map order and list order that disagree.** The number on the map must be the number in
  the list; after reordering the list, renumber the discs.
* **A highlight too close to the ground.** `#59baa7` on `#9bd5cb` is the minimum; a lighter
  highlight vanishes on a projector.
* **More than three notes.** Each yellow rectangle takes attention from the seven regions;
  past three, the notes become the page.
* **Paragraphs of uneven length** make ragged columns with large holes. Keep entries
  between 2 and 5 lines, or switch to one line each.
