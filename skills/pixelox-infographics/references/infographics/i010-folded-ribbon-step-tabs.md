---
id: i010
title: Four folded ribbon tabs with alternating paper panels
kind: [process, list]
topic: [business, generic]
mood: [clean, vibrant, friendly, corporate]
style: [paper-cut, gradient, line-icon]
format: portrait
structure: S-spine-steps
data: [process, text-only]
charts: [C-ribbon-label, C-icon-grid, C-title-block]
palette: ['#fdfdfd', '#1f9a2e', '#e88d06', '#1893d4', '#e84701', '#606060', '#c2c2c2']
needs: nothing
---

# i010: Four folded ribbon tabs with alternating paper panels

## 1. Concept

A portrait four-step sheet on white. Four upright colored tabs (green, orange, blue,
red-orange) stand in a stack on the center axis like a folded paper ribbon; each tab carries a
thin step number and the word STEP, and a white paper panel slides out of it to the left or
the right with an icon, a short heading, and a note. A pale secondary note fills the empty
side. The message is "four steps, each with one thing to do". It works because nearly
everything is white: the four saturated tabs are the only color, so the count and the order
are read in a second, and the fold triangles and soft shadows make flat shapes feel like one
physical strip.

Presented as a sheet between two gray strips; this description covers the sheet only.

## 2. Data story

No numbers except the step numbers. Text only, as an ordered list:

* 4 steps. Each: a two-digit number and the word STEP on the tab; in the panel a heading of
  2 to 3 words and a note of 10 to 14 words on 3 to 4 lines.
* 3 secondary notes of 14 to 18 words on 4 lines, in very pale gray, opposite the panels of
  steps 2, 3, and 4 (step 1's opposite side holds the title).
* 1 title in two parts: one or a few words in small bold caps, then 1 to 2 words in large
  light caps, with 4 color chips under it (one per step).

Placeholder content in the recipe (invented, a first product sale): Set a date, Price it,
Test one idea, Measure it, each with a one-sentence note and a pale aside.

## 3. Information architecture

1. **Title**, upper right, beside the first tab: what the four steps lead to; four chips
   preview the tab colors.
2. **Step 01**: green tab; panel to the left (icon, heading, note).
3. **Step 02**: orange tab; panel to the right; pale aside on the left.
4. **Step 03**: blue tab; panel to the left; pale aside on the right.
5. **Step 04**: red-orange tab; panel to the right; pale aside on the left.

The reader goes down the stack of tabs (number, STEP), and at each tab looks to the side
where the panel is. The asides are deliberately faint: optional reading, and a counterweight
that keeps the page from leaning.

## 4. Format and grid

* Portrait, 1 : 1.435. Recipe canvas 1080 x 1550.
* Tabs on the center axis: x from 44.6% to 55.4% W (116 px wide). One tab every 21.2% H
  (328 px); tab tops at 6.7%, 27.9%, 49%, 70.2% H. Each tab is a trapezoid: its tall edge is
  345 px (22.3% H), its short edge 251 px (16.2% H), top and bottom edges slanting 47 px.
  Odd tabs have the tall edge on the left, even tabs on the right: the tall edge is the side
  the panel leaves from.
* Panels: 266 px wide (24.6% W), as tall as the tab's short edge at the tab and 28 px taller
  at the far end. Left panels `20%,9.7%,24.6% x 16.2%` (step 1; step 3 is 42.3% H lower);
  right panels `55.4%,30.9%,24.6% x 16.2%` (step 2; step 4 is 42.3% H lower).
* Two text columns centered on 32.3% W and 67.7% W; column width 22% W.
* Title block centered on 72.7% W, lines at 12.8% and 15.4% H, chips at 19.2% H.
* Outer margins: 18% W left and right stay empty. The sheet is mostly air.

## 5. Layer stack

1. Ground: near-white `#fdfdfd`.
2. Four white panels `#ffffff`, each a slightly flared trapezoid, with a soft gray shadow all
   around (in the original the panel's far end fades into the ground and only its top and
   bottom edges cast a shadow).
3. Eight fold triangles (two per tab, 32 px wide, 47 px tall) at the tall edge's top and
   bottom corners, in a dark tone of the tab: `#016737`, `#e8621a`, `#1d68b8`, `#a81921`.
   They read as the back of the ribbon passing behind the panel.
4. Four tabs with a top-to-bottom gradient (lighter on top): green `#37ab46` to `#098815`,
   orange `#f59b0a` to `#da7f03`, blue `#28a8e2` to `#097fc7`, red-orange `#f76300` to
   `#d92c02`. Each casts a soft shadow onto its panel.
5. On each tab: the step number in white light numerals, and STEP in white bold caps.
6. In each panel: a 48 px line icon in the tab's hue, a heading in the same hue, a note in
   gray `#8f8f8f`.
7. Three pale asides in `#c2c2c2`.
8. Title in gray `#606060`; four rounded chips, 38 x 12 px, in the tab hues.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#fdfdfd` | ground and panels (`#ffffff`) | 86% | nothing |
| `#1f9a2e` | tab 1 (gradient `#37ab46` to `#098815`); heading and icon `#1f9d55` | 3% | step 1 |
| `#e88d06` | tab 2 (`#f59b0a` to `#da7f03`); heading and icon `#f39c12` | 3% | step 2 |
| `#1893d4` | tab 3 (`#28a8e2` to `#097fc7`); heading and icon `#2b8fd0` | 3% | step 3 |
| `#e84701` | tab 4 (`#f76300` to `#d92c02`); heading and icon `#e8562a` | 3% | step 4 |
| `#606060` | title; notes in `#8f8f8f` | 1% | nothing |
| `#c2c2c2` | pale asides | 1% | "secondary: skip if in a hurry" |

Categorical: four saturated hues of similar weight on white, one per step; the heading and
icon of a panel repeat their tab's hue. All other type is gray, never black.

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Title, small line | 1 to 4 words | caps | geometric sans (Montserrat, Poppins / Avenir Next) | 700 | 28 | 0 | - | center | `#606060` | 72.7% W, cap top 12.8% H |
| Title, large line | 1 to 2 words, up to 12 letters | caps | geometric sans | 300 | 44 (the original: 37 for a 12-letter word) | 0 | - | center | `#606060` | 72.7% W, cap top 15.4% H |
| Tab number (x4) | 01 to 04 | - | geometric sans, light (the original: a squarish thin display sans; Exo 2 Light, Jost Light / Avenir Next) | 300 | 58 | 0 | - | center | `#ffffff` | 50% W; cap top 134 px under the tab's top corner |
| Tab word (x4) | STEP | caps | geometric sans | 700 | 28 | 0 | - | center | `#ffffff` | 50% W; cap top 56 px under the number's |
| Panel heading (x4) | 2 to 3 words | sentence | geometric sans | 500 | 28 | 0 | - | center | the tab's hue | column center; cap top 143 px under the tab's top corner |
| Panel note (x4) | 10 to 14 words on 3 lines | sentence | neo-grotesque (Inter, Roboto / Helvetica Neue) | 400 | 18 (the original: 11 to 12, too small to copy) | 0 | 23 px | center | `#8f8f8f` | column center, 43 px under the heading |
| Pale aside (x3) | 14 to 18 words on 4 lines | sentence | neo-grotesque | 400 | 19 (the original: 16) | 0 | 27 px | center | `#c2c2c2` | the other column, centered on the tab's middle height |

Two families: a geometric sans in three weights for everything on or beside the tabs, a
neutral sans for the notes. Sizes 58 : 44 : 28 : 18, about 3.2 : 2.4 : 1.6 : 1. The contrast
of weights (300 numbers, 700 STEP) matters more than the sizes.

## 8. Charts and diagrams

* No data chart. The diagram is a stack of four step tabs, built natively: no script draws
  folded tabs (the nearest library chart is C-process-spine, a line with round nodes and
  alternating text; see Variations).
* **C-ribbon-label** logic for each tab: a main strip (`shape path`, four points) and two
  darker fold triangles behind its tall edge. For a tab whose top corner is at y0: odd tab
  `M 482 y0 L 598 y0+47 L 598 y0+298 L 482 y0+345 Z`, folds at x 450 to 482; even tab
  mirrored (`M 482 y0+47 L 598 y0 L 598 y0+345 L 482 y0+298 Z`, folds at x 598 to 630).
  y0 = 104, 432, 760, 1088.
* **C-icon-grid** cell in each panel: icon, heading, note on one centered axis.
* **C-title-block**, centered variant, with a row of four chips instead of a rule.
* Simplified: the original panels are drawn in perspective (top and bottom edges converge
  toward the tab and fade out at the far end); the recipe uses a flared trapezoid with an
  even shadow.

## 9. Icons and illustration

* Four line icons, 48 px, 4 px line, each in its step's hue, at the top of its panel:
  `clock` (01), `house` (02), `lightbulb` (03), `chart-no-axes-column-increasing` (04). The
  original: a clock with a turning arrow, a house with a currency sign, a hanging bulb, three
  rising columns.
* Four chips: `shape rect` 38 x 12 px, radius 6, 48 px apart, centered under the title.
* No photos.

## 10. Effects and finish

* Tab gradient: `fx overlay --kind gradient --stops 'DARK@0,LIGHT@1' --angle 90` (90 runs
  bottom to top, so the light stop is on top).
* Tab shadow: `fx drop-shadow --offset 0,8 --blur 22 --opacity 30`; it darkens the panel
  where it meets the tab, which is what sells the fold.
* Panel shadow: `fx drop-shadow --offset 0,6 --blur 30 --opacity 20`, copied with `fx copy`.
* No outlines, no texture. Everything else flat.

## 11. Hierarchy and eye path

1. The four tabs: the only saturated shapes, stacked on the axis; green first because it is
   on top.
2. The numbers on them, 01 to 04.
3. Each panel's colored heading, found by its hue next to the tab.
4. The title, gray, upper right.
5. Notes, then the pale asides.

Type steps 58 : 44 : 28 : 18. Color, not size, ranks the page: saturated, then gray, then
pale gray.

## 12. Build recipe

Nothing to supply. Canvas 1080 x 1550. All copy is invented.

```bash
pixelox new -o tabs.pxo --size 1080x1550 --fill '#fdfdfd'
pixelox shape path tabs.pxo --in-place --name Panel1 --d 'M 216 137 L 482 151 L 482 402 L 216 416 Z' --fill '#ffffff'
pixelox shape path tabs.pxo --in-place --name Panel2 --d 'M 598 479 L 864 465 L 864 744 L 598 730 Z' --fill '#ffffff'
pixelox shape path tabs.pxo --in-place --name Panel3 --d 'M 216 793 L 482 807 L 482 1058 L 216 1072 Z' --fill '#ffffff'
pixelox shape path tabs.pxo --in-place --name Panel4 --d 'M 598 1135 L 864 1121 L 864 1400 L 598 1386 Z' --fill '#ffffff'
pixelox fx drop-shadow tabs.pxo --in-place --layer Panel1 --offset 0,6 --blur 30 --color '#000000' --opacity 20
pixelox fx copy tabs.pxo --in-place --from Panel1 --to Panel2,Panel3,Panel4
pixelox shape path tabs.pxo --in-place --name Fold1 --d 'M 482 104 L 482 151 L 450 151 Z M 482 449 L 482 402 L 450 402 Z' --fill '#016737'
pixelox shape path tabs.pxo --in-place --name Fold2 --d 'M 598 432 L 598 479 L 630 479 Z M 598 777 L 598 730 L 630 730 Z' --fill '#e8621a'
pixelox shape path tabs.pxo --in-place --name Fold3 --d 'M 482 760 L 482 807 L 450 807 Z M 482 1105 L 482 1058 L 450 1058 Z' --fill '#1d68b8'
pixelox shape path tabs.pxo --in-place --name Fold4 --d 'M 598 1088 L 598 1135 L 630 1135 Z M 598 1433 L 598 1386 L 630 1386 Z' --fill '#a81921'
pixelox shape path tabs.pxo --in-place --name Tab1 --d 'M 482 104 L 598 151 L 598 402 L 482 449 Z' --fill '#1f9a2e'
pixelox shape path tabs.pxo --in-place --name Tab2 --d 'M 482 479 L 598 432 L 598 777 L 482 730 Z' --fill '#e88d06'
pixelox shape path tabs.pxo --in-place --name Tab3 --d 'M 482 760 L 598 807 L 598 1058 L 482 1105 Z' --fill '#1893d4'
pixelox shape path tabs.pxo --in-place --name Tab4 --d 'M 482 1135 L 598 1088 L 598 1433 L 482 1386 Z' --fill '#e84701'
pixelox fx overlay tabs.pxo --in-place --layer Tab1 --kind gradient --stops '#098815@0,#37ab46@1' --angle 90
pixelox fx overlay tabs.pxo --in-place --layer Tab2 --kind gradient --stops '#da7f03@0,#f59b0a@1' --angle 90
pixelox fx overlay tabs.pxo --in-place --layer Tab3 --kind gradient --stops '#097fc7@0,#28a8e2@1' --angle 90
pixelox fx overlay tabs.pxo --in-place --layer Tab4 --kind gradient --stops '#d92c02@0,#f76300@1' --angle 90
pixelox fx drop-shadow tabs.pxo --in-place --layer Tab1 --offset 0,8 --blur 22 --color '#000000' --opacity 30
pixelox fx drop-shadow tabs.pxo --in-place --layer Tab2 --offset 0,8 --blur 22 --color '#000000' --opacity 30
pixelox fx drop-shadow tabs.pxo --in-place --layer Tab3 --offset 0,8 --blur 22 --color '#000000' --opacity 30
pixelox fx drop-shadow tabs.pxo --in-place --layer Tab4 --offset 0,8 --blur 22 --color '#000000' --opacity 30
pixelox text add tabs.pxo --in-place --name Num1 --content "01" --font @geometric-sans --weight 300 --size 58 --color '#ffffff' --at 540,238 --anchor t --anchor-to ink
pixelox text add tabs.pxo --in-place --name Step1 --content "STEP" --font @geometric-sans --weight 700 --size 28 --color '#ffffff' --at 540,294 --anchor t --anchor-to ink
pixelox text add tabs.pxo --in-place --name Num2 --content "02" --font @geometric-sans --weight 300 --size 58 --color '#ffffff' --at 540,566 --anchor t --anchor-to ink
pixelox text add tabs.pxo --in-place --name Step2 --content "STEP" --font @geometric-sans --weight 700 --size 28 --color '#ffffff' --at 540,622 --anchor t --anchor-to ink
pixelox text add tabs.pxo --in-place --name Num3 --content "03" --font @geometric-sans --weight 300 --size 58 --color '#ffffff' --at 540,894 --anchor t --anchor-to ink
pixelox text add tabs.pxo --in-place --name Step3 --content "STEP" --font @geometric-sans --weight 700 --size 28 --color '#ffffff' --at 540,950 --anchor t --anchor-to ink
pixelox text add tabs.pxo --in-place --name Num4 --content "04" --font @geometric-sans --weight 300 --size 58 --color '#ffffff' --at 540,1222 --anchor t --anchor-to ink
pixelox text add tabs.pxo --in-place --name Step4 --content "STEP" --font @geometric-sans --weight 700 --size 28 --color '#ffffff' --at 540,1278 --anchor t --anchor-to ink
pixelox shape icon tabs.pxo clock --in-place --name Ic1 --size 48 --at 349,202 --anchor c --color '#1f9d55' --stroke-width 4
pixelox text add tabs.pxo --in-place --name H1 --content "Set a date" --font @geometric-sans --weight 500 --size 28 --color '#1f9d55' --at 349,247 --anchor t --anchor-to ink
pixelox text add tabs.pxo --in-place --name B1 --content $'Pick the day the offer\ngoes live and count\nback from it.' --font @neo-grotesque --size 18 --leading 23 --align center --color '#8f8f8f' --at 349,290 --anchor t
pixelox shape icon tabs.pxo house --in-place --name Ic2 --size 48 --at 731,530 --anchor c --color '#f39c12' --stroke-width 4
pixelox text add tabs.pxo --in-place --name H2 --content "Price it" --font @geometric-sans --weight 500 --size 28 --color '#f39c12' --at 731,575 --anchor t --anchor-to ink
pixelox text add tabs.pxo --in-place --name B2 --content $'Ask five customers\nwhat they pay today\nfor the same job.' --font @neo-grotesque --size 18 --leading 23 --align center --color '#8f8f8f' --at 731,618 --anchor t
pixelox text add tabs.pxo --in-place --name P2 --content $'A price is a guess until\nsomeone pays it. Start\nhigher than feels safe\nand watch who stays.' --font @neo-grotesque --size 19 --leading 27 --align center --color '#c2c2c2' --at 349,604 --anchor c
pixelox shape icon tabs.pxo lightbulb --in-place --name Ic3 --size 48 --at 349,858 --anchor c --color '#2b8fd0' --stroke-width 4
pixelox text add tabs.pxo --in-place --name H3 --content "Test one idea" --font @geometric-sans --weight 500 --size 28 --color '#2b8fd0' --at 349,903 --anchor t --anchor-to ink
pixelox text add tabs.pxo --in-place --name B3 --content $'Show a plain page to\ntwenty people and\ncount the sign-ups.' --font @neo-grotesque --size 18 --leading 23 --align center --color '#8f8f8f' --at 349,946 --anchor t
pixelox text add tabs.pxo --in-place --name P3 --content $'One idea per test. Two\nchanges at once and you\nwill not know which one\nmoved the number.' --font @neo-grotesque --size 19 --leading 27 --align center --color '#c2c2c2' --at 731,932 --anchor c
pixelox shape icon tabs.pxo chart-no-axes-column-increasing --in-place --name Ic4 --size 48 --at 731,1186 --anchor c --color '#e8562a' --stroke-width 4
pixelox text add tabs.pxo --in-place --name H4 --content "Measure it" --font @geometric-sans --weight 500 --size 28 --color '#e8562a' --at 731,1231 --anchor t --anchor-to ink
pixelox text add tabs.pxo --in-place --name B4 --content $'Track sales per week,\nnot per day, for the\nfirst two months.' --font @neo-grotesque --size 18 --leading 23 --align center --color '#8f8f8f' --at 731,1274 --anchor t
pixelox text add tabs.pxo --in-place --name P4 --content $'Weekly numbers smooth\nout the noise. Decide in\nadvance what result\nmeans stop or go.' --font @neo-grotesque --size 19 --leading 27 --align center --color '#c2c2c2' --at 349,1260 --anchor c
pixelox text add tabs.pxo --in-place --name T1 --content "FOUR STEPS TO A" --font @geometric-sans --weight 700 --size 28 --color '#606060' --at 785,198 --anchor t --anchor-to ink
pixelox text add tabs.pxo --in-place --name T2 --content "FIRST SALE" --font @geometric-sans --weight 300 --size 44 --color '#606060' --at 785,238 --anchor t --anchor-to ink
pixelox shape rect tabs.pxo --in-place --name Chip1 --rect 694,292,38x12 --radius 6 --fill '#2dae64'
pixelox shape rect tabs.pxo --in-place --name Chip2 --rect 742,292,38x12 --radius 6 --fill '#f39003'
pixelox shape rect tabs.pxo --in-place --name Chip3 --rect 790,292,38x12 --radius 6 --fill '#34abe3'
pixelox shape rect tabs.pxo --in-place --name Chip4 --rect 838,292,38x12 --radius 6 --fill '#ed4e18'
pixelox export tabs.pxo -o tabs.png
```

Reuse: change the headings, notes, and icons first. Every position follows from a tab's top
corner y0 (104, 432, 760, 1088; step 328): number cap top y0 + 134, STEP y0 + 190, icon
center y0 + 98, heading y0 + 143, note y0 + 186, aside centered on y0 + 172. Column centers
are 349 (left) and 731 (right). For 5 tabs use a 1080 x 1880 canvas; for 3 use 1080 x 1220.
Keep panel notes to 3 lines of about 24 characters.

## 13. Variations and pairings

* **Library version:** C-process-spine with `side=alt`, `style=circle`, the four hues as
  `colors=`, and the numbers as badges gives the same reading order with round nodes; use it
  when there are 5 to 8 steps.
* **Dark sheet:** ground `#1d2330`, panels `#2a3142`, notes `#aab3c5`, asides `#5d6680`; the
  tab gradients stay.
* **One hue:** four tabs in four steps of one color (`#bfe3f7`, `#6fc0ec`, `#1e96d6`,
  `#0b5f95`) say "progress" instead of "four different things".
* **With figures:** put a number in the heading place ("3 weeks", "12 calls") at 40 px bold
  and the label in the note: a four-fact sheet. Add C-source-line at 96% H.
* **Square carousel:** one tab and its panel per 1080 x 1080 slide, the tab at the left edge.

## 14. Pitfalls

* **Panels on the wrong side of the fold.** The panel leaves from the tab's tall edge, where
  the fold triangles are. Put it on the short side and the ribbon stops making sense.
* **Asides as dark as the notes.** Seven text blocks of equal weight turn a clean sheet into
  a page of gray. Keep the asides two steps paler, or delete them.
* **Tiny notes.** The original sets notes at 11 px on a 1080 px sheet: unreadable on a
  phone. Use 18 px and fewer words.
* **Uneven steps passed off as equal.** Four identical tabs say the steps are the same size.
  If one step takes ten times longer, say so in its heading ("Weeks 2 to 8"); do not stretch
  one tab by eye. This is the data-integrity rule here: no size without a number.
* **Gradients that flip.** Every tab is lighter on top. One tab lit from below breaks the
  single light source and the stack stops looking like one ribbon.
