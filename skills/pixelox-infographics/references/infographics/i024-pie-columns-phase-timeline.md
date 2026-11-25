---
id: i024
title: Three-module sheet with pie, pill columns, and phase timeline
kind: [statistical, timeline]
topic: [business, finance, generic]
mood: [clean, friendly, vibrant]
style: [flat, line-icon]
format: square
structure: S-kit-sheet
data: [part-to-whole, comparison, process]
charts: [C-polar-area, C-columns, C-timeline, C-speech-bubble, C-icon-badge, C-source-line]
palette: ['#f6f6f6', '#ba007a', '#860062', '#005f41', '#abcdb4', '#0c9bdc', '#0b5a7a']
needs: nothing
---

# i024: Three-module sheet with pie, pill columns, and phase timeline

## 1. Concept

A square sheet that holds three independent modules on a pale gray ground, each in its own
color family: an exploded four-slice pie in magentas (top left) with an icon badge and a
short note pointing at every slice; six round-topped columns in greens from pale to nearly
black (bottom left), each with a white icon in its head; and a vertical four-phase timeline
in blues (right) whose phases sit in big speech-bubble panels. The message is "three views of
one subject: the split, the amounts, the plan". It works because color does the grouping: no
frames, no cards, no rules, yet nobody confuses a magenta callout with a green caption, and
inside each family the steps from light to dark give order.

Presented on a sheet with an unused band along the bottom; this description covers the
design area only.

## 2. Data story

* **Pie: part of a whole, 4 categories.** Four shares between a fifth and a third, close
  enough that the angles alone do not show the order. The slices are pulled apart and each
  has a different length; the original prints no values, so there the pie is decorative.
* **Columns: 6 amounts, one series.** Two columns at the maximum, the other four between 60%
  and 80% of it. No axis and no value labels in the original; the order is the category
  order (not sorted), and the colors darken left to right.
* **Timeline: 4 phases**, numbered 01 at the bottom to 04 at the top, each a title and a
  note of 8 to 12 words. No dates.
* Text items: 4 pie notes and 6 column notes of 10 to 14 words, each under a 2-word heading.

Placeholder data in the recipe (example data, invented, a household budget): shares 30%,
26%, 24%, 20%; savings per month of $380, $520, $310, $350, $520, $400; four plan phases.

## 3. Information architecture

1. **Pie with four callouts** (top left): how the whole splits. Each slice has a ring badge
   with a tail aimed at it and a heading in the slice's color.
2. **Six columns** (bottom left): how one amount varies across six categories or periods. An
   icon in each column head names it; a heading and a note stand under it.
3. **Phase timeline** (right, full height): what happens in which order. The spine carries a
   dot and a small calendar icon per phase; the bubble to its right holds an icon, PHASE 0N,
   and a note. Phase 01 is at the bottom: the plan climbs.

There is no title and no numbering between modules: the sheet is a kit, and the three parts
can be read in any order. The left column reads top to bottom; the timeline bottom to top.

## 4. Format and grid

* Square. Recipe canvas 1080 x 1080. Margins 3.3% W left, 2.9% W right.
* Left column 3% to 64% W: pie module `3%,5%,61% x 40%`, column module `5.2%,50%,55.8% x
  42%`. Right column 66% to 97% W: timeline `66%,8%,31% x 78%`.
* Pie: center 27.8% W, 28.9% H; the longest slice reaches 16% W, the others 11.8% to 14.1%
  W. Badges are 8.1% W rings standing 3 to 7% W outside their slice's rim, at 15% W / 10.5% H, 50.7% W / 20.6% H, 44.8% W / 39.4% H, and
  8.3% W / 41.1% H; each badge's text block sits beside it, away from the pie.
* Columns: 6 bars 7.7% W wide on a 9.3% W pitch, baseline at 82% H, the tallest reaching
  51% H; headings at 83.5% H, notes to 90% H.
* Timeline: spine at 71.6% W from 9.7% to 86% H; dots at 9.7%, 29.1%, 48.5%, 67.9% H (one
  phase every 19.4% H); calendars at 66.7% W; bubbles `73.7%,y,23.4% x 12.1%` with y 1.7% H
  above each dot, tails reaching 15.3% H below the dot.
* Source line at 5.2% W, 95.7% H.

## 5. Layer stack

1. Ground: pale gray `#f6f6f6`, flat.
2. Pie: four slices cut apart by a cross of parallel 18 px gaps, each slice a different
   length (the larger the share, the longer), no outline: dark magenta `#860062` (the largest,
   right), plum `#600046` (bottom), pink `#cb77a8` (lower left), magenta `#ba007a` (upper
   left).
3. Four callout badges: a tail triangle in the slice color, a ring (ground-colored disc with
   a 9 px outline in the slice color), a line icon in the same color.
4. Four callout headings in the slice colors and notes in dark gray `#333333`.
5. Six columns with fully round heads: `#abcdb4`, `#85b29c`, `#67947c`, `#34765d`, `#005f41`,
   `#003425`; a thin baseline under them.
6. Six white icons, one in each column head.
7. Six headings (in the column colors, the two palest deepened for contrast) and notes.
8. Timeline spine: four 9 px segments, each in the color of the phase above it: `#79bbe9`,
   `#0c9bdc`, `#1a70a8`, `#0b5a7a`, the last running on below the bottom dot.
9. Four dots (22 px) on the spine and four calendar icons left of it, in the phase colors.
10. Four bubbles in the phase colors: a rounded panel whose lower left corner is drawn out
    into a curved tail that drops along the spine.
11. In each bubble: a white icon, PHASE 0N in white caps, a white note.
12. Source line in mid gray `#8a8a8a`.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#f6f6f6` | ground | 62% | nothing |
| `#ba007a` | pie slice, badge, heading (with `#860062`, `#600046`, `#cb77a8`) | 7% | the four pie categories: one hue, four tones |
| `#860062` | largest slice | 3% | the largest share |
| `#005f41` | dark columns (`#34765d`, `#003425`) | 8% | columns 4 to 6 |
| `#abcdb4` | pale columns (`#85b29c`, `#67947c`) | 7% | columns 1 to 3 |
| `#0c9bdc` | upper phases (`#79bbe9`) | 6% | phases 03 and 04 |
| `#0b5a7a` | lower phases (`#1a70a8`) | 6% | phases 01 and 02 |

Three hue families, one per module: magenta = the split, green = the amounts, blue = the
plan. Inside a family the tones are ordered steps (sequential), not separate categories:
greens darken left to right, blues darken toward phase 01. No accent color.

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Callout heading (x4) | 2 words, here a name and a share | sentence | condensed gothic (Oswald, Fjalla One / Avenir Next Condensed) | 700 | 24 (the original: 18) | 0 | - | left | the slice's color | beside each badge: 20.3% W / 5.7% H; 56.1% / 16.5%; 50% / 35.7%; 3.3% / 27.8% |
| Callout note (x4) | 6 to 10 words on 2 to 3 lines (the original: 5 lines) | sentence | neo-grotesque (Inter, Arimo / Helvetica Neue) | 400 | 16 (the original: 11) | 0 | 20 px | left | `#333333` | 30 px under the heading |
| Column heading (x6) | 1 to 2 words | sentence | condensed gothic | 700 | 22 | 0 | - | center | the column's color | column centers, cap top 83.5% H |
| Column note (x6) | 3 to 5 words on 2 lines, here the value and a reason | sentence | neo-grotesque | 400 | 15 (the original: 11) | 0 | 19 px | center | `#333333` | 30 px under the heading |
| Phase title (x4) | PHASE and a two-digit number | caps | condensed gothic | 700 | 28 | 0 | - | left | `#ffffff` | 83% W, cap top 12 px under the dot's height |
| Phase note (x4) | 5 to 8 words on 2 lines | sentence | neo-grotesque | 400 | 15 (the original: 10) | 0 | 19 px | left | `#ffffff` | 83% W, 44 px under the dot's height |
| Source | one line | sentence | neo-grotesque | 400 | 16 | 0 | - | left | `#8a8a8a` | 5.2% W, 95.7% H |

Two families: a bold condensed gothic for every heading, a neutral sans for notes. Sizes
28 : 24 : 22 : 16. There is no title and no large number; the charts are the display
elements. The original's notes are 10 to 11 px on a 1080 px sheet and cannot be read at
normal size: the recipe uses 15 to 16 px and fewer words.

## 8. Charts and diagrams

* **C-polar-area**: `scripts/donut.js` with `inner=0`, placed by center (`cx=27.8 cy=28.9
  r=16`), `values=30,24,20,26`, `start=6` (the largest slice begins 6 degrees right of 12
  o'clock), `radii=1,0.8,0.74,0.88` (the larger the share, the longer the slice: the angles
  carry the values, the lengths only rank them), `gappx=18` (gaps with parallel sides, so
  the slices look pulled apart along a tilted cross), `sep=none labelpos=none`. No labels
  from the script: the callouts carry the names and shares. Slice anchors from the log give
  each badge its direction.
* **C-callout** as a ring badge with a tail (C-speech-bubble logic): the tail's tip lies 88
  px from the badge center on the line toward the pie's center; its base is a chord of the
  ring.
* **C-columns**: `scripts/bars.js`, `box=5.2,50,55.8,35.2`, six values, blank labels,
  `axis=0 gridlines=0 vlabels=0`, `max=52` (the tallest value, so two columns touch the top
  of the plot), `gap=0.17`, `radius=42` (half the bar width: a full round head), one green
  per bar. The script logs each bar's top (x = 106, 207, 307, 408, 508, 609; y = 640, 550,
  685, 659, 550, 627) and the baseline (885): icons go 46 px under each top, headings 17 px
  under the baseline.
* **C-timeline**, vertical: `scripts/steps.js` with `dir=v node=22 connector=none
  badges=none`, four blank labels, `box=70.6,0,6,77.6`, one blue per dot; dots at (773,105),
  (773,314), (773,524), (773,733). The spine is four native `shape line` segments because it
  changes color at each dot and runs on below the last one.
* **Simplified:** in the original the slice lengths follow no order (about 81%, 100%, 74%,
  and 88% of the longest, clockwise from the upper left) and each slice is nudged outward by
  a different amount; the recipe ranks the lengths by share and keeps one center. The
  original has no baseline under the columns; the script draws a thin one.

## 9. Icons and illustration

* Pie badges, 46 px, 5 px line, in the slice color: `user`, `phone`, `house`, `utensils` (the
  original: solid glyphs of a bust, a desk telephone, a house, a spoon and fork).
* Column heads, 46 px, 4 px line, white: `circle-dollar-sign`, `search`, `briefcase`,
  `shopping-cart`, `lightbulb`, `piggy-bank` (the original: the same six subjects as solid
  white glyphs).
* Timeline: `calendar` 56 px, 6 px line, in the phase color, left of each dot; in the
  bubbles 56 px white icons: `users` (04), `settings` (03), `search` (02), `lightbulb` (01).
* The original uses filled silhouettes; Lucide icons are outlines. Raise `--stroke-width`
  to keep them as heavy as the type.

## 10. Effects and finish

None: flat color, no shadows, no gradients, no outlines except the badge rings. Soft shapes
do the styling: round column heads (`radius=42`), 48 px bubble corners, round dots.

## 11. Hierarchy and eye path

1. The pie: the largest saturated shape, top left.
2. The blue bubbles down the right edge: the largest area of one hue.
3. The green columns, read left to right as they darken.
4. Headings in color, then notes.

No module dominates by size (each takes about a quarter of the sheet); hue separates them.
Type steps 28 : 24 : 22 : 16.

## 12. Build recipe

Nothing to supply. Canvas 1080 x 1080. All figures are example data.

```bash
pixelox new -o kit.pxo --size 1080x1080 --fill '#f6f6f6'
pixelox render js scripts/donut.js -o pie.png --size 1080x1080 --param cx=27.8 --param cy=28.9 --param r=16 --param inner=0 --param values=30,24,20,26 --param radii=1,0.8,0.74,0.88 --param gappx=18 --param labelpos=none --param sep=none --param start=6 --param colors='#860062,#600046,#cb77a8,#ba007a'
pixelox comp place kit.pxo pie.png --in-place --as Pie --at 0,0
pixelox shape path kit.pxo --in-place --name TailA --d 'M 170 156 L 212 185 L 200 136 Z' --fill '#ba007a'
pixelox shape ellipse kit.pxo --in-place --name RingA --rect 118,69,88x88 --fill '#f6f6f6' --stroke '#ba007a' --stroke-width 9
pixelox shape icon kit.pxo user --in-place --name IconA --size 46 --at 162,113 --anchor c --color '#ba007a' --stroke-width 5
pixelox text add kit.pxo --in-place --name HeadA --content "Housing 26%" --font @condensed --weight 700 --size 24 --color '#ba007a' --at 219,62 --anchor tl --anchor-to ink
pixelox text add kit.pxo --in-place --name NoteA --content $'Rent, power, and\nwater for a flat\nof two rooms.' --font @neo-grotesque --size 16 --leading 20 --color '#333333' --at 219,92 --anchor tl
pixelox shape path kit.pxo --in-place --name TailB --d 'M 504 219 L 465 252 L 516 253 Z' --fill '#860062'
pixelox shape ellipse kit.pxo --in-place --name RingB --rect 504,178,88x88 --fill '#f6f6f6' --stroke '#860062' --stroke-width 9
pixelox shape icon kit.pxo phone --in-place --name IconB --size 46 --at 548,222 --anchor c --color '#860062' --stroke-width 5
pixelox text add kit.pxo --in-place --name HeadB --content "Food 30%" --font @condensed --weight 700 --size 24 --color '#860062' --at 606,178 --anchor tl --anchor-to ink
pixelox text add kit.pxo --in-place --name NoteB --content $'Groceries and\ntwo meals out\nevery week.' --font @neo-grotesque --size 16 --leading 20 --color '#333333' --at 606,208 --anchor tl
pixelox shape path kit.pxo --in-place --name TailC --d 'M 459 390 L 409 380 L 440 420 Z' --fill '#600046'
pixelox shape ellipse kit.pxo --in-place --name RingC --rect 440,382,88x88 --fill '#f6f6f6' --stroke '#600046' --stroke-width 9
pixelox shape icon kit.pxo house --in-place --name IconC --size 46 --at 484,426 --anchor c --color '#600046' --stroke-width 5
pixelox text add kit.pxo --in-place --name HeadC --content "Savings 24%" --font @condensed --weight 700 --size 24 --color '#600046' --at 540,386 --anchor tl --anchor-to ink
pixelox text add kit.pxo --in-place --name NoteC --content $'Moved on pay day,\nbefore anything\nelse is spent.' --font @neo-grotesque --size 16 --leading 20 --color '#333333' --at 540,416 --anchor tl
pixelox shape path kit.pxo --in-place --name TailD --d 'M 134 438 L 165 397 L 114 407 Z' --fill '#cb77a8'
pixelox shape ellipse kit.pxo --in-place --name RingD --rect 46,400,88x88 --fill '#f6f6f6' --stroke '#cb77a8' --stroke-width 9
pixelox shape icon kit.pxo utensils --in-place --name IconD --size 46 --at 90,444 --anchor c --color '#cb77a8' --stroke-width 5
pixelox text add kit.pxo --in-place --name HeadD --content "Leisure 20%" --font @condensed --weight 700 --size 24 --color '#cb77a8' --at 36,300 --anchor tl --anchor-to ink
pixelox text add kit.pxo --in-place --name NoteD --content $'Trips, films,\nand gifts.' --font @neo-grotesque --size 16 --leading 20 --color '#333333' --at 36,330 --anchor tl
pixelox render js scripts/bars.js -o bars.png --size 1080x1080 --param box=5.2,50,55.8,35.2 --param values=38,52,31,35,52,40 --param labels=' | | | | | ' --param axis=0 --param gridlines=0 --param vlabels=0 --param gap=0.17 --param radius=42 --param max=52 --param colors='#abcdb4,#85b29c,#67947c,#34765d,#005f41,#003425'
pixelox comp place kit.pxo bars.png --in-place --as Bars --at 0,0
pixelox shape icon kit.pxo circle-dollar-sign --in-place --name BarIcon1 --size 46 --at 106,686 --anchor c --color '#ffffff' --stroke-width 4
pixelox shape icon kit.pxo search --in-place --name BarIcon2 --size 46 --at 207,596 --anchor c --color '#ffffff' --stroke-width 4
pixelox shape icon kit.pxo briefcase --in-place --name BarIcon3 --size 46 --at 307,731 --anchor c --color '#ffffff' --stroke-width 4
pixelox shape icon kit.pxo shopping-cart --in-place --name BarIcon4 --size 46 --at 408,705 --anchor c --color '#ffffff' --stroke-width 4
pixelox shape icon kit.pxo lightbulb --in-place --name BarIcon5 --size 46 --at 508,596 --anchor c --color '#ffffff' --stroke-width 4
pixelox shape icon kit.pxo piggy-bank --in-place --name BarIcon6 --size 46 --at 609,673 --anchor c --color '#ffffff' --stroke-width 4
pixelox text add kit.pxo --in-place --name Month1 --content "January" --font @condensed --weight 700 --size 22 --color '#7fae8f' --at 106,902 --anchor t --anchor-to ink
pixelox text add kit.pxo --in-place --name Saved1 --content $'$380 saved,\nafter a trip' --font @neo-grotesque --size 15 --leading 19 --align center --color '#333333' --at 106,932 --anchor t
pixelox text add kit.pxo --in-place --name Month2 --content "February" --font @condensed --weight 700 --size 22 --color '#6a9c85' --at 207,902 --anchor t --anchor-to ink
pixelox text add kit.pxo --in-place --name Saved2 --content $'$520 saved,\nno takeaway' --font @neo-grotesque --size 15 --leading 19 --align center --color '#333333' --at 207,932 --anchor t
pixelox text add kit.pxo --in-place --name Month3 --content "March" --font @condensed --weight 700 --size 22 --color '#67947c' --at 307,902 --anchor t --anchor-to ink
pixelox text add kit.pxo --in-place --name Saved3 --content $'$310 saved,\ncar repair' --font @neo-grotesque --size 15 --leading 19 --align center --color '#333333' --at 307,932 --anchor t
pixelox text add kit.pxo --in-place --name Month4 --content "April" --font @condensed --weight 700 --size 22 --color '#34765d' --at 408,902 --anchor t --anchor-to ink
pixelox text add kit.pxo --in-place --name Saved4 --content $'$350 saved,\nnew phone' --font @neo-grotesque --size 15 --leading 19 --align center --color '#333333' --at 408,932 --anchor t
pixelox text add kit.pxo --in-place --name Month5 --content "May" --font @condensed --weight 700 --size 22 --color '#005f41' --at 508,902 --anchor t --anchor-to ink
pixelox text add kit.pxo --in-place --name Saved5 --content $'$520 saved,\nbonus month' --font @neo-grotesque --size 15 --leading 19 --align center --color '#333333' --at 508,932 --anchor t
pixelox text add kit.pxo --in-place --name Month6 --content "June" --font @condensed --weight 700 --size 22 --color '#003425' --at 609,902 --anchor t --anchor-to ink
pixelox text add kit.pxo --in-place --name Saved6 --content $'$400 saved,\non track' --font @neo-grotesque --size 15 --leading 19 --align center --color '#333333' --at 609,932 --anchor t
pixelox shape line kit.pxo --in-place --name Spine1 --from 773,105 --to 773,314 --stroke '#79bbe9' --stroke-width 9
pixelox shape line kit.pxo --in-place --name Spine2 --from 773,314 --to 773,524 --stroke '#0c9bdc' --stroke-width 9
pixelox shape line kit.pxo --in-place --name Spine3 --from 773,524 --to 773,733 --stroke '#1a70a8' --stroke-width 9
pixelox shape line kit.pxo --in-place --name Spine4 --from 773,733 --to 773,930 --stroke '#0b5a7a' --stroke-width 9
pixelox render js scripts/steps.js -o dots.png --size 1080x1080 --param box=70.6,0,6,77.6 --param dir=v --param node=22 --param connector=none --param badges=none --param labels=' | | | ' --param colors='#79bbe9,#0c9bdc,#1a70a8,#0b5a7a'
pixelox comp place kit.pxo dots.png --in-place --as Dots --at 0,0
pixelox shape path kit.pxo --in-place --name Bubble1 --d 'M 844 87 L 1001 87 A 48 48 0 0 1 1049 135 L 1049 170 A 48 48 0 0 1 1001 218 L 880 218 Q 810 222 797 270 L 796 135 A 48 48 0 0 1 844 87 Z' --fill '#79bbe9'
pixelox shape icon kit.pxo calendar --in-place --name Cal1 --size 56 --at 720,126 --anchor c --color '#79bbe9' --stroke-width 6
pixelox shape icon kit.pxo users --in-place --name PhIcon1 --size 56 --at 852,151 --anchor c --color '#ffffff' --stroke-width 5
pixelox text add kit.pxo --in-place --name Phase1 --content "PHASE 04" --font @condensed --weight 700 --size 28 --color '#ffffff' --at 896,117 --anchor tl --anchor-to ink
pixelox text add kit.pxo --in-place --name PhNote1 --content $'Review as a pair\nonce a month' --font @neo-grotesque --size 15 --leading 19 --color '#ffffff' --at 896,149 --anchor tl
pixelox shape path kit.pxo --in-place --name Bubble2 --d 'M 844 296 L 1001 296 A 48 48 0 0 1 1049 344 L 1049 379 A 48 48 0 0 1 1001 427 L 880 427 Q 810 431 797 479 L 796 344 A 48 48 0 0 1 844 296 Z' --fill '#0c9bdc'
pixelox shape icon kit.pxo calendar --in-place --name Cal2 --size 56 --at 720,335 --anchor c --color '#0c9bdc' --stroke-width 6
pixelox shape icon kit.pxo settings --in-place --name PhIcon2 --size 56 --at 852,360 --anchor c --color '#ffffff' --stroke-width 5
pixelox text add kit.pxo --in-place --name Phase2 --content "PHASE 03" --font @condensed --weight 700 --size 28 --color '#ffffff' --at 896,326 --anchor tl --anchor-to ink
pixelox text add kit.pxo --in-place --name PhNote2 --content $'Automate transfers\non every pay day' --font @neo-grotesque --size 15 --leading 19 --color '#ffffff' --at 896,358 --anchor tl
pixelox shape path kit.pxo --in-place --name Bubble3 --d 'M 844 506 L 1001 506 A 48 48 0 0 1 1049 554 L 1049 589 A 48 48 0 0 1 1001 637 L 880 637 Q 810 641 797 689 L 796 554 A 48 48 0 0 1 844 506 Z' --fill '#1a70a8'
pixelox shape icon kit.pxo calendar --in-place --name Cal3 --size 56 --at 720,545 --anchor c --color '#1a70a8' --stroke-width 6
pixelox shape icon kit.pxo search --in-place --name PhIcon3 --size 56 --at 852,570 --anchor c --color '#ffffff' --stroke-width 5
pixelox text add kit.pxo --in-place --name Phase3 --content "PHASE 02" --font @condensed --weight 700 --size 28 --color '#ffffff' --at 896,536 --anchor tl --anchor-to ink
pixelox text add kit.pxo --in-place --name PhNote3 --content $'Track each expense\nfor thirty days' --font @neo-grotesque --size 15 --leading 19 --color '#ffffff' --at 896,568 --anchor tl
pixelox shape path kit.pxo --in-place --name Bubble4 --d 'M 844 715 L 1001 715 A 48 48 0 0 1 1049 763 L 1049 798 A 48 48 0 0 1 1001 846 L 880 846 Q 810 850 797 898 L 796 763 A 48 48 0 0 1 844 715 Z' --fill '#0b5a7a'
pixelox shape icon kit.pxo calendar --in-place --name Cal4 --size 56 --at 720,754 --anchor c --color '#0b5a7a' --stroke-width 6
pixelox shape icon kit.pxo lightbulb --in-place --name PhIcon4 --size 56 --at 852,779 --anchor c --color '#ffffff' --stroke-width 5
pixelox text add kit.pxo --in-place --name Phase4 --content "PHASE 01" --font @condensed --weight 700 --size 28 --color '#ffffff' --at 896,745 --anchor tl --anchor-to ink
pixelox text add kit.pxo --in-place --name PhNote4 --content $'Name one goal\nand its price' --font @neo-grotesque --size 15 --leading 19 --color '#ffffff' --at 896,777 --anchor tl
pixelox text add kit.pxo --in-place --name Source --content "Example data, invented for this layout." --font @neo-grotesque --size 16 --color '#8a8a8a' --at 56,1034 --anchor l
pixelox export kit.pxo -o kit.png
```

Reuse: change `values=` in the two chart lines first, then the headings that repeat those
values (the charts print no numbers, so every figure must appear in a heading). In the pie
line keep `radii=` in the rank order of `values=` (largest share = 1) and check on the
export that every badge's tail still points at its own slice: the badges do not move with
the slices (turn the pie with `start=`, or move a badge). Anchors:
`lab/anchors.py scripts/bars.js --size 1080x1080 --param box=5.2,50,55.8,35.2 --param
values=38,52,31,35,52,40 --param labels=' | | | | | ' --param axis=0 --param vlabels=0
--param gap=0.17 --param max=52` prints the bar tops; run it again after changing the values
and move the six icons. For a dot at (773, y) the bubble's top edge is y - 18 and every
bubble point shifts by the same amount; calendar at (720, y + 21), icon at (852, y + 46),
title at (896, y + 12).

## 13. Variations and pairings

* **With a title:** shrink the three modules to start at 14% H and add a C-title-block on
  top; the sheet becomes a report page.
* **Labels from the scripts:** `labelpos=out` on the pie and `vlabels=1` with real `labels=`
  on the columns remove the callouts and captions: quicker, plainer, and every value is
  printed.
* **Other families:** any three hues about 120 degrees apart (orange, teal, violet); keep
  four to six tones per family and the same light-to-dark direction.
* **One module alone:** the bubble timeline at 1080 x 1350 with six phases; or the pill
  columns as a single chart card (S-chart-card).
* Pairs with C-donut in place of the pie when a total belongs in the middle, with C-pie
  (drop `radii=` and `gappx=`) when readers must compare the shares by eye, and with
  C-progress-bars in place of the columns when the six values are percentages.

## 14. Pitfalls

* **Charts without numbers.** The original prints no value anywhere: the pie and the columns
  are shapes, not data. Put each figure in its heading or turn the script labels on, and
  keep a source line. This is the first data-integrity rule.
* **Slice lengths that contradict the shares.** With unequal radii, area and angle can
  disagree: a small share drawn long looks largest (the original's lengths follow no
  order). Keep `radii=` in the rank order of `values=`, print every share in its heading,
  and never let length stand for a second measure.
* **Columns not starting at zero.** With values this close it is tempting to cut the axis.
  `scripts/bars.js` always starts at zero; do not fake taller differences by cropping.
* **Darkness read as value.** The greens darken left to right whatever the heights. If the
  order is not time or rank, use one green for all six and a second tone for the column the
  text talks about (`highlight=`).
* **Pale headings.** A heading in the palest green on pale gray has almost no contrast.
  Deepen the two lightest heading colors (here `#7fae8f`, `#6a9c85`) or set all headings in
  the darkest tone.
* **Notes wider than their column.** A column is 100 px from its neighbor: two lines of 12
  characters at 15 px is the limit. Longer notes collide; drop them first.
