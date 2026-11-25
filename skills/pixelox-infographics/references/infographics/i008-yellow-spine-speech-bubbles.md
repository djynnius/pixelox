---
id: i008
title: Four steps on a spine with icon bubbles on yellow
kind: [process, list]
topic: [business, generic]
mood: [bold, energetic, friendly, warm]
style: [flat, line-icon, paper-cut]
format: portrait
structure: S-spine-steps
data: [process, text-only]
charts: [C-process-spine, C-speech-bubble, C-icon-badge]
palette: ['#fecb02', '#ffffff', '#1a1a1a']
needs: nothing
---

# i008: Four steps on a spine with icon bubbles on yellow

## 1. Concept

A tall four-step sheet on one loud yellow. A thin black line runs down the middle with a
small ring at each step; on one side of every ring sits a large white speech bubble holding a
line icon, its tail touching the ring, and on the other side the step's number, a two-line
caps title, and two lines of text. Sides swap at every step. The message is "do these four
things, top to bottom". It works because there are only three colors and two shapes: the
white bubbles are the only light areas on the page, so the eye hops from bubble to bubble,
and each tail points at the exact place on the line where its text begins.

## 2. Data story

No numbers except the step numbers. Text only, as an ordered list:

* 4 steps. Each: a number with a full stop ("01." to "04."), a title of 4 to 6 words in caps
  on 2 lines of up to 15 characters, and a note of 8 to 10 words on 2 lines of up to 28
  characters.
* No title, no source line: the sheet is a module meant to sit under a headline or inside a
  slide.

Placeholder content in the recipe (invented, about reading data): COLLECT EVERY NUMBER WE
HAVE; SHARE THEM WITH THE WHOLE TEAM; SEARCH THE DATA FOR PATTERNS; WRITE DOWN THE ONE BIG
IDEA, each with a one-sentence note.

## 3. Information architecture

1. **Step 01**: bubble on the left, text on the right, both level with the first ring.
2. **Step 02**: mirrored: text on the left (right-aligned to the line), bubble on the right.
3. **Step 03**: as step 01.
4. **Step 04**: as step 02.

The line is the reading rail. It starts a little above the first ring and ends a little
below the last, so the sequence has a clear start and stop. Within a step the order is
bubble (picture), number, title, note. The alternation keeps the four heavy white discs from
stacking into a column and gives each text block the full half-width.

## 4. Format and grid

* Portrait, about 2 : 3 (1 : 1.48). Recipe canvas 1080 x 1600.
* Spine at 50% W, from 9.3% to 90.4% H, 5 px thick.
* Rings on the spine at 16.3%, 38.7%, 61.1%, 83.5% H: one step every 22.4% H (358 px).
* Bubbles: discs of radius 14.4% W (156 px), centers at 28.3% W (left) and 71.7% W (right),
  level with their ring; the tail's tip stops 40 px short of the spine (46.3% or 53.7% W).
* Text blocks start 42 px from the spine: left edge 53.9% W on the right side, right edge
  46.1% W on the left side; up to 39% W (422 px) wide; the block of title and note is
  centered on the ring's height and the number sits above it.
* Outer margins: 7% W left and right; nothing touches the edges.

## 5. Layer stack

1. Ground: yellow `#fecb02`, flat.
2. Spine: a black `#1a1a1a` vertical line, 5 px.
3. Four rings on the spine: 32 px across, yellow inside, black outline.
4. Step titles (bold caps) and notes (regular) in black, alternating right, left, right, left.
5. Four white speech bubbles: a disc with a short triangular tail that points at the ring.
   Soft shadow to the lower right in dark yellow-brown.
6. Four black line icons, 170 px, centered in the bubbles.
7. Four step numbers in black, above each title.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#fecb02` | ground; inside of the rings | 74% | nothing |
| `#ffffff` | speech bubbles | 19% | nothing: marks "a step is here" |
| `#1a1a1a` | spine, rings, icons, all type | 7% | nothing |

Three flat colors, no accent: every step is equal. The bubble shadow `#7a5c00` at 45% is a
darker yellow, never gray, so the ground stays clean. Because color encodes nothing, the
ground can be swapped for any saturated mid-light hue.

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Step number (x4) | two digits and a full stop | - | neo-grotesque (the original: a humanist sans such as Open Sans or Source Sans / Helvetica Neue) | 700 | 56 | 0 | - | toward the spine | `#1a1a1a` | 54% W (right side) or 46% W (left side); cap top 8.8% H above the ring |
| Step title (x4) | 4 to 6 words on 2 lines, up to 15 characters a line | caps | neo-grotesque | 700 | 38 | 0 | 118% (45 px) | toward the spine | `#1a1a1a` | first line 5.5% H above the ring; second line level with the ring |
| Step note (x4) | 8 to 10 words on 2 lines | sentence | neo-grotesque | 400 | 30 | 0 | 130% (39 px) | toward the spine | `#1a1a1a` | under the title, 9 px gap |

One family, two weights. Sizes 56 : 38 : 30, about 1.9 : 1.3 : 1: a narrow range, because
the sheet is narrow and every line must stay readable on a phone. Text on the right side is
flush left, text on the left side is flush right, so both hug the spine.

## 8. Charts and diagrams

* **C-process-spine**: `scripts/steps.js` with `dir=v side=alt style=dot badges=none
  connector=none`, `box=7,5.1,86,89.6`, `node=32`, `nodefill=` the ground color,
  `colors='#1a1a1a' onecolor=1`, `fs=30 titlesize=38`, `ink` and `muted` both black. The
  script draws the four rings and the four text blocks (title bold, note regular), wraps the
  titles, and puts the blocks on alternating sides, right first. It logs the ring centers:
  (540,261), (540,619), (540,978), (540,1336).
* The spine is a native `shape line` because the original's line runs past the first and
  last ring by 7% H; the script's own connector (turned off here) stops at the end nodes.
* **C-speech-bubble** x4: one `shape path` each, a disc of radius 156 px whose center is 234
  px left or right of the ring, with a tail whose tip is 40 px from the spine. The tail's
  base spans 58 px above and below the ring's height.
* No axis, no values, no legend. Simplified: the original's rings have a thinner outline
  (5 px on a 35 px ring); the script's dot style draws about 8 px.

## 9. Icons and illustration

* Four line icons, 170 px, 7 px line, black, centered in the bubbles (bubble diameter 312 px,
  so the icon is 55% of it): `chart-no-axes-combined` (01), `users` (02), `search` (03),
  `lightbulb` (04).
* The original's icons are small scenes in a thin, even outline: columns with a trend line
  above them; three people under three speech marks; a magnifier among three cogs; a bulb
  with rays. A single Lucide icon is simpler; for the same richness place a second, smaller
  icon in the bubble (for example `settings` at 70 px beside `search`).
* No photos.

## 10. Effects and finish

* Bubble shadow: `fx drop-shadow --offset 10,14 --blur 26 --color '#7a5c00' --opacity 45`,
  copied to all four with `fx copy`. It makes the bubbles read as cut paper lying on the
  yellow.
* Everything else is flat. No gradients, no outlines except the rings.

## 11. Hierarchy and eye path

1. The four white bubbles (largest, lightest shapes), top left first.
2. The spine, which links them and gives the direction.
3. Each step's number, then its caps title, then the note.

Type steps 56 : 38 : 30. The icons, not the type, carry the weight: each bubble is about ten
times the area of its number.

## 12. Build recipe

Nothing to supply. Canvas 1080 x 1600. All copy is invented.

```bash
pixelox new -o steps.pxo --size 1080x1600 --fill '#fecb02'
pixelox shape line steps.pxo --in-place --name Spine --from 540,149 --to 540,1446 --stroke '#1a1a1a' --stroke-width 5
pixelox render js scripts/steps.js -o spine.png --size 1080x1600 --param box=7,5.1,86,89.6 --param dir=v --param side=alt --param style=dot --param badges=none --param connector=none --param node=32 --param nodefill='#fecb02' --param colors='#1a1a1a' --param onecolor=1 --param labels='COLLECT EVERY NUMBER WE HAVE|SHARE THEM WITH THE WHOLE TEAM|SEARCH THE DATA FOR PATTERNS|WRITE DOWN THE ONE BIG IDEA' --param notes='Pull last year from every\nsource into one sheet.|Three people look before\nanyone draws a conclusion.|Sort, group, and chart\nuntil something repeats.|One sentence the data\nsupports, and nothing more.' --param ink='#1a1a1a' --param muted='#1a1a1a' --param fs=30 --param titlesize=38
pixelox comp place steps.pxo spine.png --in-place --as Steps --at 0,0
pixelox shape path steps.pxo --in-place --name Bubble1 --d 'M 451 203 L 500 261 L 451 319 A 156 156 0 1 1 451 203 Z' --fill '#ffffff'
pixelox shape path steps.pxo --in-place --name Bubble2 --d 'M 629 561 L 580 619 L 629 677 A 156 156 0 1 0 629 561 Z' --fill '#ffffff'
pixelox shape path steps.pxo --in-place --name Bubble3 --d 'M 451 920 L 500 978 L 451 1036 A 156 156 0 1 1 451 920 Z' --fill '#ffffff'
pixelox shape path steps.pxo --in-place --name Bubble4 --d 'M 629 1278 L 580 1336 L 629 1394 A 156 156 0 1 0 629 1278 Z' --fill '#ffffff'
pixelox fx drop-shadow steps.pxo --in-place --layer Bubble1 --offset 10,14 --blur 26 --color '#7a5c00' --opacity 45
pixelox fx copy steps.pxo --in-place --from Bubble1 --to Bubble2,Bubble3,Bubble4
pixelox shape icon steps.pxo chart-no-axes-combined --in-place --name Ic1 --size 170 --at 306,261 --anchor c --color '#1a1a1a' --stroke-width 7
pixelox shape icon steps.pxo users --in-place --name Ic2 --size 170 --at 774,619 --anchor c --color '#1a1a1a' --stroke-width 7
pixelox shape icon steps.pxo search --in-place --name Ic3 --size 170 --at 306,978 --anchor c --color '#1a1a1a' --stroke-width 7
pixelox shape icon steps.pxo lightbulb --in-place --name Ic4 --size 170 --at 774,1336 --anchor c --color '#1a1a1a' --stroke-width 7
pixelox text add steps.pxo --in-place --name N1 --content "01." --font @neo-grotesque --weight 700 --size 56 --color '#1a1a1a' --at 583,121 --anchor tl --anchor-to ink
pixelox text add steps.pxo --in-place --name N2 --content "02." --font @neo-grotesque --weight 700 --size 56 --color '#1a1a1a' --at 497,479 --anchor tr --anchor-to ink
pixelox text add steps.pxo --in-place --name N3 --content "03." --font @neo-grotesque --weight 700 --size 56 --color '#1a1a1a' --at 583,838 --anchor tl --anchor-to ink
pixelox text add steps.pxo --in-place --name N4 --content "04." --font @neo-grotesque --weight 700 --size 56 --color '#1a1a1a' --at 497,1196 --anchor tr --anchor-to ink
pixelox export steps.pxo -o steps.png
```

Reuse: change `labels=` and `notes=` first; `\n` in a note forces its line break, titles wrap
by themselves at 422 px (about 15 caps). The ring centers come from
`lab/anchors.py scripts/steps.js --size 1080x1600 --param box=7,5.1,86,89.6 --param dir=v
--param side=alt --param style=dot --param node=32 --param labels='A|B|C|D'`. For a ring at
(540, cy): a left bubble is `M 451 cy-58 L 500 cy L 451 cy+58 A 156 156 0 1 1 451 cy-58 Z`
with its icon at (306, cy); a right bubble mirrors it around the spine (629, 580, sweep flag
0, icon at 774); the number's cap top is at cy - 140, 43 px from the spine on the text side.
Odd steps have the text on the right. With 5 steps use a 1080 x 1960 canvas and keep the
358 px step.

## 13. Variations and pairings

* **Other grounds:** coral `#ff5a4d`, teal `#12b5a6`, or violet `#6d3fd6` with white type and
  white bubbles holding dark icons; keep exactly three colors.
* **With a headline:** add 14% H on top (canvas 1080 x 1860) for a C-title-block in black
  caps; start the spine under it.
* **Numbers in the bubbles:** replace each icon with a hero number ("62%") in 110 px bold
  and move the step number into the title line: the sheet becomes a four-fact list
  (C-stat-block on a spine).
* **Timeline:** give `badges='2019,2021,2023,2025'` and drop the native numbers: the years
  appear beside the rings (C-timeline with `dir=v`).
* Pairs with C-source-line when the bubbles hold figures, and with i038's four candy hues
  (one bubble outline color per step) when the steps need telling apart.

## 14. Pitfalls

* **Bubbles that float.** A bubble whose tail does not point at its ring reads as
  decoration. The tail's tip, the ring, and the title's second line share one height.
* **Long titles.** At 38 px a half-column holds about 15 caps per line. A title of three
  lines pushes the number into the step above; cut words, do not shrink.
* **Gray shadows on yellow.** A black or gray shadow dirties the ground. Use a dark tone of
  the ground color at 40 to 50%.
* **A sequence that is not one.** The spine and the numbers promise an order. For four
  parallel tips use the same bubbles in a 2 x 2 grid (C-icon-grid) without the line or the
  numbers: otherwise the layout states something the content does not. This is the
  data-integrity rule for step diagrams.
* **Icons of mixed weight.** One icon with a 3 px line beside one with 8 px looks like two
  sets. Give all four the same size and `--stroke-width`.
* **Centered text.** Text centered in its half loses the link to the spine; keep it flush to
  the line on both sides.
