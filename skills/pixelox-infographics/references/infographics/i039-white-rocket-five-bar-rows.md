---
id: i039
title: White rocket with five numbered bars and a summary box
kind: [process, list]
topic: [business, generic]
mood: [clean, corporate, vibrant]
style: [gradient, 3d, line-icon]
format: square
structure: S-object-rows
data: [process, text-only]
charts: [C-rocket, C-section-band, C-icon-grid]
palette: ['#f1f1f3', '#79138e', '#43398e', '#65a6ae', '#81af25', '#e19a0b', '#e9dea8']
needs: nothing
---

# i039: White rocket with five numbered bars and a summary box

## 1. Concept

A square list graphic on pale gray: a glossy white rocket stands at the left edge, and five
colored bars come out from behind it like a stack of flags, each ending in a white number 01
to 05. Every bar continues to the right as a row: a bold label, a short note, and a line icon
at the far right. Under the rocket's flame a sixth, wider bar leads into a cream box with a
closing paragraph. The message is "five things, in this order, and here is the takeaway". It
works because the bars turn a plain five-row table into something that seems to be attached
to an object: the rocket gives the eye a starting point, and the spectrum of the bars
(purple, indigo, teal, green, yellow) gives each row an identity without any color in the
text.

## 2. Data story

No numbers. Text only, as an ordered list:

* 5 items, each a label of 2 words (up to 16 letters) and a note of 9 to 12 words (three
  lines of very small type in the original; two lines at a readable size in the recipe).
* 5 line icons, one per item.
* 1 summary: a label of 2 words and two short paragraphs of 12 to 16 words each.
* 1 title of 2 words in light capitals.

Placeholder content in the recipe (invented, a quarterly checklist): SET THE COURSE, BUILD
THE ENGINE, TALK TO USERS, TEST ONE IDEA, SHOW THE RESULT; summary AFTER LIFT-OFF.

## 3. Information architecture

1. **Title**, top right, in thin capitals, with a hairline rule to its left that spans the
   width of the bars: the rule ties the title to the stack below.
2. **The rocket**, left: decoration and anchor. Its nose tip marks the top of the first bar,
   its nozzle the bottom of the fifth.
3. **Five bars with numbers**: the order. The first bar is as tall as the nose cone (12% H),
   the other four are equal (10% H).
4. **Five rows**: label and note on a zebra of pale panels that fade out to the right, then
   one icon per row in a column near the right edge.
5. **Summary**: an orange bar under the flame and a cream box with a label and two
   paragraphs. It is separated from the rows by 4.5% H of space.

Reading path: rocket, number, label, note, icon; down row by row; then the box.

## 4. Format and grid

* Square. Recipe canvas 1080 x 1080. Margins: 6.5% W left (the fin), 8.2% W right, 7.9% H
  top, 7.6% H bottom.
* Rocket (the object column, the left 38% W): axis at 20.3% W; nose tip 15.3% H, fin tips
  79.4% H, flame tip 92.4% H; 28% W wide over the fins, body 20% W at its widest (the
  recipe's rocket, drawn with `aspect=1.15`: body 20% W, 34% W over the fins).
* Bars: `20.5%,15.3%,26.9% x 51.5%`, starting on the rocket's axis and running behind it to
  47.4% W. Rows: 15.3 to 27.5, 27.5 to 37.2, 37.2 to 47.1, 47.1 to 56.9, 56.9 to 66.8% H.
  Numbers centered on 42.5% W.
* Row panels: 47.4% to 78% W, same rows. Text column 50.2% to 79.4% W. Icon column centered
  on 88.1% W.
* Title: cap top 7.9% H, from 50.1% to 88% W. Rule: 11.25% H, from 20.5% to 47.5% W.
* Summary: orange bar `20.5%,71.3%,26.9% x 21.1%`; cream box `47.4%,71.3%,44.4% x 21.1%`.
* Alignment: three flush-left edges, at 20.5% W (bars), 47.4% W (panels and box), and 50.2%
  W (all text). Numbers and icons are centered in their own columns.

## 5. Layer stack

1. Ground: a diagonal gradient, `#d6d5db` in the bottom left corner to `#fdfdfd` in the top
   right, about `#f1f1f3` in the middle.
2. Hairline rule `#b5b5b8` and the title.
3. Five row panels: rows 1, 3, 5 gray `#dededf` at the left, fading into the ground at 78% W;
   rows 2 and 4 near-white `#f8f8f8`.
4. Five bars with a left-to-right gradient, darker at the rocket: purple `#79138e` to
   `#ac4b99`; indigo `#43398e` to `#60549e`; teal `#5b9fb0` to `#68adcd`; green `#81af25` to
   `#a0c235`; amber `#e19a0b` to yellow `#f0d41c`.
5. Summary bar, orange `#d9730f` to `#e69f35`, and the cream box `#e9dea8`.
6. Rocket, in front of the bars: white body, red nose cone, chrome porthole with blue glass,
   three white fins, a dark red collar, an orange and yellow flame that overlaps the summary
   bar. A soft shadow falls from it onto the bars.
7. Numbers 01 to 05 in white on the bars.
8. Row labels in charcoal `#3c3c3c`, notes in gray, five line icons in near-black `#1a1a1a`.
9. Summary label in dark khaki `#857b48`, paragraphs in khaki `#9a9060`.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#f1f1f3` | ground (gradient `#d6d5db` to `#fdfdfd`), row panels | 62% | nothing |
| `#79138e` | bar 01 (to `#ac4b99`) | 3% | item 1 |
| `#43398e` | bar 02 (to `#60549e`) | 2.5% | item 2 |
| `#65a6ae` | bar 03 (to `#68adcd`) | 2.5% | item 3 |
| `#81af25` | bar 04 (to `#a0c235`) | 2.5% | item 4 |
| `#e19a0b` | bar 05 (to `#f0d41c`); summary bar `#d9730f` | 8% | item 5; the summary |
| `#e9dea8` | summary box | 9% | nothing: sets the summary apart |

Categorical in spectrum order: five hues from purple through blue and green to yellow, so
the stack reads as one ramp from the nose down, and the summary continues it into orange.
The rocket is white with one red accent (the nose `#c4160c`); all text is charcoal or gray.
No color is repeated in the text: the bar touches its row, and that is the link.

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Title | 2 words, 15 to 20 letters | caps | neo-grotesque light (Lato Light, Inter 300 / Helvetica Neue Light) | 300 | 37 | 0 | - | left | `#1e1e1e` | 50.1% W, cap top 7.9% H |
| Bar number (x5) | 01 to 05 | - | condensed gothic (Oswald, Roboto Condensed / Avenir Next Condensed) | 700 | 56 (the original: 52) | 0 | - | center | `#ffffff` | 42.5% W, middle of each bar |
| Row label (x5) | 2 words, up to 16 letters | caps | neo-grotesque (Lato Bold, Inter / Helvetica Neue) | 700 | 20 (the original: 17) | 0 | - | left | `#3c3c3c` | 50.2% W, cap top 1.7 to 2.9% H under the row's top |
| Row note (x5) | 9 to 12 words on 2 lines (the original: 3 lines) | sentence | neo-grotesque | 400 | 18 (the original: 11) | 0 | 133% | left | `#8a8a8e` (the original: `#b4b4b4`) | 50.2% W, 26 px under the label's cap top |
| Summary label | 2 words | caps | neo-grotesque | 700 | 25 | 0 | - | left | `#857b48` | 50.2% W, cap top 73.9% H |
| Summary text | 2 paragraphs of 12 to 16 words, 2 lines each (the original: 3 lines) | sentence | neo-grotesque | 400 | 18 (the original: 11) | 0 | 133% | left | `#9a9060` | 50.2% W, from 77.6% and 83.9% H |

One family in three weights (300 for the title, 700 for labels, 400 for notes) plus a
condensed bold for the numbers. Sizes 56 : 37 : 25 : 20 : 18. The original sets its notes at
11 px in a gray that barely shows: they are texture, not text. The recipe uses 18 px and a
darker gray so the notes can be read; that is the one deliberate departure.

## 8. Charts and diagrams

* **C-rocket**: `scripts/object.js`, `kind=rocket`, `aspect=1.15` (the plump body, 20% W
  wide), `bands=2` (band 1 = the red nose, band 2 = the white body), `badges=none`, box
  `2.3,15.3,36,77.1`, `colors='#c4160c,#f6f6f8'`, `fincolor='#dcdce1'`, `dark='#a8323c'`
  (the collar), flame on, `shade=1`. The band 2 anchor (219, 615) takes the porthole. The
  script's own porthole (`window=1`) is not used: its pale flat ring disappears on a white
  body, and `aspect=` widens it into an oval.
* **C-section-band**, used five times as numbered bars: `shape rect` with `fx overlay
  --kind gradient --angle 0`. They are furniture, not a bar chart: all five are the same
  length, start on the rocket's axis, and end at 47.4% W.
* **C-icon-grid** in rows: label, note, icon per row; the icons form their own column.
* **3D spec of the original rocket** (for a later three.js build; the recipe draws the flat
  script rocket with a drop shadow instead). Object: a body of revolution with an ogive nose
  cone (15.3% to 27.4% H), a body bulging to 20% W at 42% H and tapering to a collar at 67%
  H, three thin swept fins at 120 degrees (left, right, and one facing the viewer, seen
  edge-on as a 1% W strip on the axis) reaching 79.4% H, a porthole on the axis at 40.9% H
  with a chrome bezel (radius 6% W) and blue glass (radius 4.4% W), and a line of rivets on
  the axis. View: straight on, elevation 0 degrees, rotation 0 degrees, orthographic. Light:
  a soft key from the front left and a warm rim from the right. Colors: body `#ffffff` in
  the lit middle to `#cbcbcb` at the edges; nose `#890114` on the left through `#c81e0a` to
  an orange rim `#f07010` on the right; bezel `#f4f5f7` (upper left) to `#8e9299` (lower
  right); glass `#a6d3f2` in the center to `#5a9bd0` at the rim; collar `#8a1030` to
  `#f06030`; flame of two nested teardrops, outer `#e8900f`, inner `#f5e515`. A soft contact
  shadow falls on the bars right of the body. Depth encodes nothing.
* Simplified against the original: the script's nose cone is longer (it ends at 38% H, not
  27%), so the porthole sits lower (57% H); the fins start higher and spread wider (34% W,
  not 28%); the flame is shorter and plain.

## 9. Icons and illustration

* Row icons, 52 px, 3 px line, `#1a1a1a`, bare (no badge), centered on 88.1% W and on each
  row's middle: `compass`, `settings`, `message-square-more`, `lightbulb`, `presentation`.
  The original shows a compass, a gear, a speech bubble with three dots, a bulb with a
  lightning bolt, and a presentation board with a rising line.
* Porthole: two `shape ellipse` layers on the band 2 anchor, 130 px and 96 px, with gradient
  overlays (a linear gray one for the bezel, a radial blue one for the glass).
* If you have a rendered rocket, place it as `illustration.png` 28% W wide with its nose tip
  at 15.3% H on the axis 20.3% W, in front of the bars.

## 10. Effects and finish

* Ground: diagonal linear gradient (`layer add`, then `fill gradient` from the bottom left
  to the top right).
* Bars: horizontal gradient on each (`fx overlay --kind gradient --angle 0`), the darker
  stop at the rocket, as if the rocket shaded them.
* Row panels: the same overlay from gray to the ground color, so each panel dissolves to the
  right and the icon column stands on clean ground.
* Rocket: `fx drop-shadow --offset 10,8 --blur 26 --opacity 28` in a dark blue-gray
  `#1c1c3a`, falling right onto the bars.
* No outlines, sharp corners everywhere.

## 11. Hierarchy and eye path

1. The rocket and the block of five colors beside it: the only saturated area.
2. The white numbers, top to bottom.
3. The labels, then the icons across the row.
4. The orange bar and the cream box.
5. The thin title: it is large (37 px) but light and gray-black, so it is read after the
   colors although it sits on top.

Numbers 56 px, title 37, summary label 25, row labels 20, notes 18.

## 12. Build recipe

Nothing to supply. Canvas 1080 x 1080. All copy is invented example content.

```bash
pixelox new -o plan.pxo --size 1080x1080 --fill '#f1f1f3'
pixelox layer add plan.pxo --in-place --name Ground
pixelox fill gradient plan.pxo --in-place --layer Ground --kind linear --from 0,100% --to 100%,0 --stops '#d6d5db@0,#f1f1f3@0.5,#fdfdfd@1'
pixelox shape line plan.pxo --in-place --name TopRule --from 221,121 --to 513,121 --stroke '#b5b5b8' --stroke-width 2
pixelox text add plan.pxo --in-place --name Title --content "LAUNCH CHECKLIST" --font @neo-grotesque --weight 300 --size 37 --color '#1e1e1e' --at 541,85 --anchor tl --anchor-to ink
pixelox shape rect plan.pxo --in-place --name Row1 --rect 512,165,330x132 --fill '#e0e0e1'
pixelox fx overlay plan.pxo --in-place --layer Row1 --kind gradient --stops '#dededf@0,#f3f3f5@1' --angle 0
pixelox shape rect plan.pxo --in-place --name Row2 --rect 512,297,330x106 --fill '#f7f7f7'
pixelox fx overlay plan.pxo --in-place --layer Row2 --kind gradient --stops '#f8f8f8@0,#f3f3f5@1' --angle 0
pixelox shape rect plan.pxo --in-place --name Row3 --rect 512,403,330x106 --fill '#e0e0e1'
pixelox fx overlay plan.pxo --in-place --layer Row3 --kind gradient --stops '#dededf@0,#f2f2f4@1' --angle 0
pixelox shape rect plan.pxo --in-place --name Row4 --rect 512,509,330x106 --fill '#f7f7f7'
pixelox fx overlay plan.pxo --in-place --layer Row4 --kind gradient --stops '#f8f8f8@0,#f2f2f4@1' --angle 0
pixelox shape rect plan.pxo --in-place --name Row5 --rect 512,615,330x106 --fill '#e0e0e1'
pixelox fx overlay plan.pxo --in-place --layer Row5 --kind gradient --stops '#dededf@0,#f0f0f2@1' --angle 0
pixelox shape rect plan.pxo --in-place --name Bar1 --rect 221,165,291x132 --fill '#79138e'
pixelox fx overlay plan.pxo --in-place --layer Bar1 --kind gradient --stops '#79138e@0,#ac4b99@1' --angle 0
pixelox shape rect plan.pxo --in-place --name Bar2 --rect 221,297,291x106 --fill '#43398e'
pixelox fx overlay plan.pxo --in-place --layer Bar2 --kind gradient --stops '#43398e@0,#60549e@1' --angle 0
pixelox shape rect plan.pxo --in-place --name Bar3 --rect 221,403,291x106 --fill '#65a6ae'
pixelox fx overlay plan.pxo --in-place --layer Bar3 --kind gradient --stops '#5b9fb0@0,#68adcd@1' --angle 0
pixelox shape rect plan.pxo --in-place --name Bar4 --rect 221,509,291x106 --fill '#81af25'
pixelox fx overlay plan.pxo --in-place --layer Bar4 --kind gradient --stops '#81af25@0,#a0c235@1' --angle 0
pixelox shape rect plan.pxo --in-place --name Bar5 --rect 221,615,291x106 --fill '#e19a0b'
pixelox fx overlay plan.pxo --in-place --layer Bar5 --kind gradient --stops '#e19a0b@0,#f0d41c@1' --angle 0
pixelox shape rect plan.pxo --in-place --name SumBar --rect 221,770,291x228 --fill '#d9730f'
pixelox fx overlay plan.pxo --in-place --layer SumBar --kind gradient --stops '#d9730f@0,#e69f35@1' --angle 0
pixelox shape rect plan.pxo --in-place --name SumBox --rect 512,770,480x228 --fill '#e9dea8'
pixelox render js scripts/object.js -o body.png --size 1080x1080 --param box=2.3,15.3,36,77.1 --param kind=rocket --param aspect=1.15 --param bands=2 --param badges=none --param colors='#c4160c,#f6f6f8' --param fincolor='#dcdce1' --param dark='#a8323c'
pixelox comp place plan.pxo body.png --in-place --as Rocket --at 0,0
pixelox fx drop-shadow plan.pxo --in-place --layer Rocket --offset 10,8 --blur 26 --color '#1c1c3a' --opacity 28
pixelox shape ellipse plan.pxo --in-place --name PortRing --rect 154,550,130x130 --fill '#c9ccd2'
pixelox fx overlay plan.pxo --in-place --layer PortRing --kind gradient --stops '#8e9299@0,#f4f5f7@1' --angle 60
pixelox shape ellipse plan.pxo --in-place --name PortGlass --rect 171,567,96x96 --fill '#6fb0e0'
pixelox fx overlay plan.pxo --in-place --layer PortGlass --kind gradient --style radial --stops '#a6d3f2@0,#5a9bd0@1'
pixelox text add plan.pxo --in-place --name N1 --content "01" --font @condensed --weight 700 --size 56 --color '#ffffff' --at 459,231 --anchor c --anchor-to ink
pixelox text add plan.pxo --in-place --name N2 --content "02" --font @condensed --weight 700 --size 56 --color '#ffffff' --at 459,350 --anchor c --anchor-to ink
pixelox text add plan.pxo --in-place --name N3 --content "03" --font @condensed --weight 700 --size 56 --color '#ffffff' --at 459,456 --anchor c --anchor-to ink
pixelox text add plan.pxo --in-place --name N4 --content "04" --font @condensed --weight 700 --size 56 --color '#ffffff' --at 459,562 --anchor c --anchor-to ink
pixelox text add plan.pxo --in-place --name N5 --content "05" --font @condensed --weight 700 --size 56 --color '#ffffff' --at 459,668 --anchor c --anchor-to ink
pixelox text add plan.pxo --in-place --name H1 --content "SET THE COURSE" --font @neo-grotesque --weight 700 --size 20 --color '#3c3c3c' --at 542,196 --anchor tl --anchor-to ink
pixelox text add plan.pxo --in-place --name B1 --content $'Agree on one goal for the quarter\nand the number that proves it.' --font @neo-grotesque --size 18 --leading 133% --color '#8a8a8e' --at 542,222 --anchor tl
pixelox text add plan.pxo --in-place --name H2 --content "BUILD THE ENGINE" --font @neo-grotesque --weight 700 --size 20 --color '#3c3c3c' --at 542,315 --anchor tl --anchor-to ink
pixelox text add plan.pxo --in-place --name B2 --content $'Automate the three tasks the team\nrepeats every single week.' --font @neo-grotesque --size 18 --leading 133% --color '#8a8a8e' --at 542,341 --anchor tl
pixelox text add plan.pxo --in-place --name H3 --content "TALK TO USERS" --font @neo-grotesque --weight 700 --size 20 --color '#3c3c3c' --at 542,421 --anchor tl --anchor-to ink
pixelox text add plan.pxo --in-place --name B3 --content $'Hold five short calls and write\ndown the words people use.' --font @neo-grotesque --size 18 --leading 133% --color '#8a8a8e' --at 542,447 --anchor tl
pixelox text add plan.pxo --in-place --name H4 --content "TEST ONE IDEA" --font @neo-grotesque --weight 700 --size 20 --color '#3c3c3c' --at 542,527 --anchor tl --anchor-to ink
pixelox text add plan.pxo --in-place --name B4 --content $'Run the cheapest experiment that\ncould prove the idea wrong.' --font @neo-grotesque --size 18 --leading 133% --color '#8a8a8e' --at 542,553 --anchor tl
pixelox text add plan.pxo --in-place --name H5 --content "SHOW THE RESULT" --font @neo-grotesque --weight 700 --size 20 --color '#3c3c3c' --at 542,633 --anchor tl --anchor-to ink
pixelox text add plan.pxo --in-place --name B5 --content $'Share one chart with everyone,\nwhether it went up or down.' --font @neo-grotesque --size 18 --leading 133% --color '#8a8a8e' --at 542,659 --anchor tl
pixelox shape icon plan.pxo compass --in-place --name I1 --size 52 --at 951,231 --anchor c --color '#1a1a1a' --stroke-width 3
pixelox shape icon plan.pxo settings --in-place --name I2 --size 52 --at 951,350 --anchor c --color '#1a1a1a' --stroke-width 3
pixelox shape icon plan.pxo message-square-more --in-place --name I3 --size 52 --at 951,456 --anchor c --color '#1a1a1a' --stroke-width 3
pixelox shape icon plan.pxo lightbulb --in-place --name I4 --size 52 --at 951,562 --anchor c --color '#1a1a1a' --stroke-width 3
pixelox shape icon plan.pxo presentation --in-place --name I5 --size 52 --at 951,668 --anchor c --color '#1a1a1a' --stroke-width 3
pixelox text add plan.pxo --in-place --name SumH --content "AFTER LIFT-OFF" --font @neo-grotesque --weight 700 --size 25 --color '#857b48' --at 542,800 --anchor tl --anchor-to ink
pixelox text add plan.pxo --in-place --name Sum1 --content $'Repeat the five steps every quarter and\nkeep the checklist where everyone sees it.' --font @neo-grotesque --size 18 --leading 133% --color '#9a9060' --at 542,838 --anchor tl
pixelox text add plan.pxo --in-place --name Sum2 --content $'Drop a step only when two quarters in a row\nshow that it changed nothing.' --font @neo-grotesque --size 18 --leading 133% --color '#9a9060' --at 542,906 --anchor tl
pixelox export plan.pxo -o plan.png
```

Reuse: the rows are plain rectangles, so the layout is a table you size yourself. The stack
runs from the rocket's nose tip (the top of the script's box, y 165) to y 721; row 1 is 132
px tall and rows 2 to 5 are 106 px. For N rows keep row 1 at 132 px and give the others
(556 - 132) / (N - 1) px each. In every row the number, the icon, and the text block share
the row's middle line: number and icon are centered on it, the label's cap top is 35 px
above it, the note starts 9 px above it. The porthole is centered on the `band2` anchor
that `scripts/object.js` logs (run the `render js` line with `--json`). Change the bar
colors as one ramp, top to bottom.

## 13. Variations and pairings

* **Three or four rows:** taller rows (140 px) hold three-line notes; drop the summary box
  and let the flame end on the ground.
* **Flat version:** `shade=0`, no drop shadow, flat bars (leave out the five `fx overlay`
  lines), a white ground: the same layout in the language of i020.
* **Bars that carry data:** if each item has a value, replace the five rectangles with
  C-bars (`scripts/bars.js`, `dir=h`, box `20.5,15.3,60,51.5`) so the lengths are true, put
  the values at the bar ends, and move the notes under the chart.
* **Dark version:** ground `#15172b`, panels `#1e2140` and `#191b33`, labels white, notes
  `#9aa3c7`, icons white; the bars keep their colors.
* **Relatives:** i020 has the same "object on the left, rows on the right" plan, flat, with
  three bands on the rocket itself and thin rules instead of bars. i038 puts four bands on
  the rocket and callouts on both sides. i041 turns the bars into a six-step exhaust column
  under the rocket. i040 and i046 fan the exhaust out into numbered columns.
* Pairs with C-timeline in place of the summary box when the five steps have dates.

## 14. Pitfalls

* **Bars of different lengths.** The moment one bar is longer, the stack reads as a bar
  chart and the numbers 01 to 05 as values. Keep all five the same length, or make the
  lengths real data with C-bars and say what they measure. This is the data-integrity rule
  here.
* **Notes too small to read.** The original's 11 px gray notes are placeholder texture. At
  1080 px wide nothing goes under 18 px: write two lines of five to seven words instead of
  three lines of filler.
* **Colored labels.** Coloring each label like its bar looks lively but the light hues
  (yellow, green) fail on a pale ground. Keep the labels charcoal; the bar is the color.
* **A ramp out of order.** The five hues work because they follow the spectrum. Shuffled
  (yellow, purple, green) they look like five categories with meanings. Keep the order, or
  use five steps of one hue.
* **The rocket in front of the text.** Its fins reach 37% W. Numbers stay right of 39% W and
  all text right of 50% W, or the fins cover them.
* **Rows without the zebra.** On a plain ground the eye slips between the label column and
  the icon column 30% W away. The fading panels are what holds each row together; keep them
  or add hairlines between rows.
