---
id: i052
title: Half fan of four segments over a ringed hub
kind: [list, comparison]
topic: [business, generic]
mood: [clean, corporate, friendly, minimal]
style: [flat, paper-cut, line-icon]
format: landscape
structure: S-half-fan
data: [comparison, text-only]
charts: [C-half-donut, C-donut, C-icon-badge]
palette: ['#ffffff', '#f15a27', '#f7b038', '#39bdc9', '#474d7f', '#f2f4f3', '#333333']
needs: nothing
---

# i052: Half fan of four segments over a ringed hub

## 1. Concept

A landscape "four options" figure on white. A thick half ring, cut into four equal segments
(orange-red, amber, turquoise, navy), arches over a pale round hub that carries the subject
in two lines and a four-color rule; a thin ring in the same four colors, turned by a third
of a segment, wraps the hub. Each segment holds one white icon, and its title and note stand
outside the arch, on a white paper flap that seems to fold out from behind the segment.
The message is "one subject, four ways". It works because the arch uses only the top half
of the circle: all four labels sit above or beside it, upright and unobstructed, and the
empty band under the hub gives the figure a base to stand on.

## 2. Data story

No numbers. Text only:

* 4 parallel options, equal in weight, in a left-to-right order that need not mean anything
  (the original letters them A to D). Each: a title of 2 words in caps, a note of 14 to 20
  words on 4 to 5 centered lines, and one icon.
* 1 hub label of 2 words on 2 lines.
* The four segments are equal (45 degrees each): the arch is a layout, not a chart.

Placeholder content in the recipe (invented, four ways to sign up for a service): AT HOME,
ONLINE, BY POST, IN PERSON, each with a note of 15 to 17 words; hub "WAYS TO / JOIN IN".

## 3. Information architecture

1. **Hub**, lower center: the subject, with a rule in the four segment colors as a key.
2. **Segments**, left to right: A (9 to 10:30 on a clock face), B (10:30 to 12), C (12 to
   1:30), D (1:30 to 3). One white icon in the middle of each.
3. **Labels**: A and D beside the arch, level with the lower segments; B and C above the
   arch. Each label is a title over a centered note, on the same axis as its segment.
4. **Paper flaps**: faint shadow lines run outward from the arch along the three inner
   segment boundaries and along the base line, dividing the white ground into four fields,
   one per label.

Read hub first, then the arch left to right; or follow any shadow line from the hub outward
to a label.

## 4. Format and grid

* Landscape, 1.58 : 1. Recipe canvas 1600 x 1010.
* One center for everything: 50.3% W, 68.6% H (805, 693 px). The base line of the arch runs
  through it.
* Arch: outer radius 25.2% W (403 px), inner radius 12.5% W (200 px); it spans 25.1% to
  75.5% W, from 28.7% H (top) to 68.6% H (base).
* Hub: disc of radius 11.4% W (182 px); hub ring from 11.4% to 12.2% W, with four 5 degree
  gaps, its color changes turned 60 degrees counter-clockwise from the arch's base.
* Labels: A centered on 17.8% W, title at 43.4% H; B on 36.9% W, title at 12.7% H; C on
  62.8% W, 12.7% H; D on 82.6% W, 43.4% H. Notes start 30 px under the title's center in a
  box 15.3% W (244 px) wide.
* Flap lines: from the center straight up, and at 45 degrees up-left and up-right to the top
  edge; a horizontal shadow along the base line across the page.
* Free band under the base line: 31% H, crossed only by the lower half of the hub.

## 5. Layer stack

1. Ground: white.
2. Upper sheet: a white rectangle from the top edge to the base line with a soft shadow
   under its lower edge (the horizontal fold).
3. Two white flaps: triangles from the center to the top edge, one for the B field (between
   the 45 degree line and the vertical), one for the C field, each with a soft shadow. Only
   the shadows show; they draw the three radial folds.
4. Arch: four flat segments, left to right orange-red `#f15a27`, amber `#f7b038`, turquoise
   `#39bdc9`, navy `#474d7f`, touching, no gaps.
5. Hub ring: a thin four-color ring (same hues, starting with orange-red at the top) with a
   soft shadow that also grounds the hub.
6. Hub: pale gray-white disc `#f2f4f3`.
7. Hub label in dark gray `#333333`; a rule of four 50 x 7 px bars in the four hues.
8. Four white line icons, 64 px, at the segment middles.
9. Four titles `#333333` and four notes `#3a3a3a`.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#ffffff` | ground, flaps, icons | 77% | nothing |
| `#f15a27` | segment A, ring arc, rule bar | 4% | option A |
| `#f7b038` | segment B, ring arc, rule bar | 4% | option B |
| `#39bdc9` | segment C, ring arc, rule bar | 4% | option C |
| `#474d7f` | segment D, ring arc, rule bar | 4% | option D |
| `#f2f4f3` | hub | 6% | nothing |
| `#333333` | all type | 1% | nothing |

Categorical: two warm and two cool hues of similar strength, warm on the left, cool on the
right. The labels are not colored: position alone ties a label to its segment, so the type
can stay one dark gray.

## 7. Text elements

Sizes are px on the 1600 px wide canvas.

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Hub label | 2 words on 2 lines, up to 11 letters a line | caps | neo-grotesque (Roboto, Inter / Helvetica Neue) | 400 | 36 | 0 | 42 px | center | `#333333` | 50.3% W, 65.9% H |
| Option title (x4) | 2 words | caps | neo-grotesque | 400 | 34 | 0 | - | center | `#333333` | label axes, 43.4% H (A, D) and 12.7% H (B, C) |
| Option note (x4) | 14 to 20 words on 4 lines (the original: 5) | sentence | neo-grotesque | 400 | 18 (the original: 17) | 0 | 23 px | center | `#3a3a3a` | under each title; box 244 px |

One family, one weight: regular throughout, no bold at all. Hierarchy comes from caps and
size only (36 and 34 against 18, about 2 : 1). This restraint is what makes the colored
arch the single loud thing on the page.

## 8. Charts and diagrams

* **C-half-donut**: `scripts/donut.js` with `sweep=180` (the arc starts at 9 o'clock),
  `values=1,1,1,1`, `inner=0.495`, placed by center (`cx=50.3 cy=68.6 r=25.2`),
  `labelpos=none sep=none`, four colors. It logs the segment middles for the icons:
  (526,578), (689,414), (920,414), (1083,578), and the center (805,693).
* **C-donut** for the hub ring: the same center, `r=12.2`, `inner=0.93`, `values=1,1,1,1`,
  `start=-60`, `gap=5`, `labelpos=none sep=none`, the same four colors.
* The hub disc is a native `shape ellipse` of radius 182 px on the center.
* Equal values on purpose: no axis, no numbers, no legend. See Variations for real shares.
* Simplified: the original flaps fade out about 250 px from the arch and each has its own
  slightly different shadow; the recipe's fold lines run to the top edge.

## 9. Icons and illustration

* Four line icons, 64 px, 5 px line, white, on the logged segment middles: `house` (A),
  `globe` (B), `send` (C), `user` (D). The original uses solid white glyphs of the same four
  subjects (a house, a globe, a paper plane, a bust).
* No photos.

## 10. Effects and finish

* Fold shadows: `fx drop-shadow --offset 0,8 --blur 26 --opacity 20` on the upper sheet;
  `--offset 0,6 --blur 30 --opacity 22` on the two flaps (`fx copy` from one to the other).
  The shapes are white on white: only their shadows are seen.
* Hub ring shadow: `--offset 0,10 --blur 30 --opacity 30` on the placed ring layer; it
  falls into the empty band below and lifts the hub.
* The arch is flat (the original adds a faint darker edge where one segment overlaps the
  next).

## 11. Hierarchy and eye path

1. The arch: the only saturated shape, slightly above the middle of the page.
2. The hub and its label, inside the arch.
3. The four titles, in a larger arc around the arch: A, B, C, D from left to right.
4. The notes.

Title and note sizes 34 : 18. Nothing is bold; the eye is steered by color and by the fan
of fold lines.

## 12. Build recipe

Nothing to supply. Canvas 1600 x 1010. All copy is invented.

```bash
pixelox new -o fan.pxo --size 1600x1010 --fill '#ffffff'
pixelox shape rect fan.pxo --in-place --name Sheet --rect 0,0,1600x693 --fill '#ffffff'
pixelox fx drop-shadow fan.pxo --in-place --layer Sheet --offset 0,8 --blur 26 --color '#000000' --opacity 20
pixelox shape path fan.pxo --in-place --name FlapB --d 'M 805 693 L 112 0 L 805 0 Z' --fill '#ffffff'
pixelox shape path fan.pxo --in-place --name FlapC --d 'M 805 693 L 1498 0 L 805 0 Z' --fill '#ffffff'
pixelox fx drop-shadow fan.pxo --in-place --layer FlapB --offset 0,6 --blur 30 --color '#000000' --opacity 22
pixelox fx copy fan.pxo --in-place --from FlapB --to FlapC
pixelox render js scripts/donut.js -o segments.png --size 1600x1010 --param cx=50.3 --param cy=68.6 --param r=25.2 --param sweep=180 --param inner=0.495 --param values=1,1,1,1 --param labelpos=none --param sep=none --param colors='#f15a27,#f7b038,#39bdc9,#474d7f'
pixelox comp place fan.pxo segments.png --in-place --as Fan --at 0,0
pixelox render js scripts/donut.js -o rim.png --size 1600x1010 --param cx=50.3 --param cy=68.6 --param r=12.2 --param inner=0.93 --param start=-60 --param gap=5 --param values=1,1,1,1 --param labelpos=none --param sep=none --param colors='#f15a27,#f7b038,#39bdc9,#474d7f'
pixelox comp place fan.pxo rim.png --in-place --as Rim --at 0,0
pixelox fx drop-shadow fan.pxo --in-place --layer Rim --offset 0,10 --blur 30 --color '#000000' --opacity 30
pixelox shape ellipse fan.pxo --in-place --name Hub --rect 623,511,364x364 --fill '#f2f4f3'
pixelox text add fan.pxo --in-place --name HubLabel --content $'WAYS TO\nJOIN IN' --font @neo-grotesque --weight 400 --size 36 --leading 42 --align center --color '#333333' --at 805,666 --anchor c
pixelox shape rect fan.pxo --in-place --name Rule1 --rect 705,732,50x7 --fill '#f15a27'
pixelox shape rect fan.pxo --in-place --name Rule2 --rect 755,732,50x7 --fill '#f7b038'
pixelox shape rect fan.pxo --in-place --name Rule3 --rect 805,732,50x7 --fill '#39bdc9'
pixelox shape rect fan.pxo --in-place --name Rule4 --rect 855,732,50x7 --fill '#474d7f'
pixelox shape icon fan.pxo house --in-place --name IconA --size 64 --at 526,578 --anchor c --color '#ffffff' --stroke-width 5
pixelox shape icon fan.pxo globe --in-place --name IconB --size 64 --at 689,414 --anchor c --color '#ffffff' --stroke-width 5
pixelox shape icon fan.pxo send --in-place --name IconC --size 64 --at 920,414 --anchor c --color '#ffffff' --stroke-width 5
pixelox shape icon fan.pxo user --in-place --name IconD --size 64 --at 1083,578 --anchor c --color '#ffffff' --stroke-width 5
pixelox text add fan.pxo --in-place --name HeadA --content "AT HOME" --font @neo-grotesque --size 34 --color '#333333' --at 284,438 --anchor c --anchor-to ink
pixelox text add fan.pxo --in-place --name BodyA --content "A volunteer brings the forms to your door and helps you fill them in, any weekday evening." --font @neo-grotesque --size 18 --leading 23 --align center --box 244x130 --color '#3a3a3a' --at 284,468 --anchor t
pixelox text add fan.pxo --in-place --name HeadB --content "ONLINE" --font @neo-grotesque --size 34 --color '#333333' --at 591,128 --anchor c --anchor-to ink
pixelox text add fan.pxo --in-place --name BodyB --content "Sign up on the website in five minutes. You only need an email address and a phone." --font @neo-grotesque --size 18 --leading 23 --align center --box 244x130 --color '#3a3a3a' --at 591,158 --anchor t
pixelox text add fan.pxo --in-place --name HeadC --content "BY POST" --font @neo-grotesque --size 34 --color '#333333' --at 1005,128 --anchor c --anchor-to ink
pixelox text add fan.pxo --in-place --name BodyC --content "Ask for the paper pack, fill it in at your own pace, and send it back free of charge." --font @neo-grotesque --size 18 --leading 23 --align center --box 244x130 --color '#3a3a3a' --at 1005,158 --anchor t
pixelox text add fan.pxo --in-place --name HeadD --content "IN PERSON" --font @neo-grotesque --size 34 --color '#333333' --at 1322,438 --anchor c --anchor-to ink
pixelox text add fan.pxo --in-place --name BodyD --content "Drop in at the town library on Saturdays between ten and two. No appointment needed." --font @neo-grotesque --size 18 --leading 23 --align center --box 244x130 --color '#3a3a3a' --at 1322,468 --anchor t
pixelox export fan.pxo -o fan.png
```

Reuse: change the hub label and the four title-and-note pairs, then the four icons. All
geometry hangs on the center (805,693): if you move the figure, change `cx`, `cy` in both
chart lines, the hub rect, the flap paths' first point, and the sheet's height. For 3
options use `values=1,1,1` (segments of 60 degrees; fold lines at 60 and 120 degrees; one
label on top, two at the sides); for 5 use `values=1,1,1,1,1` and three labels above.

## 13. Variations and pairings

* **Real shares:** give the arch real values (`values=40,30,20,10`) and write each share in
  its title ("ONLINE 40%"): the figure becomes a half donut chart with long labels. Order
  the segments by size and add a C-source-line in the lower band.
* **A gauge:** one value on the arch (`kind=ring sweep=180 value=68`) with the number in the
  hub (C-gauge) for a single percentage in the same look.
* **Dark ground:** ground `#1f2430`, hub `#2b3242`, type `#ffffff`, fold shadows at 40%
  opacity; the four hues stay.
* **Portrait:** put the arch at 40% H on a 1080 x 1350 canvas and stack the four labels in a
  2 x 2 grid below it, each with a small color dot.
* Pairs with C-comparison-columns below the figure when the options need a point-by-point
  table, and with C-title-block above when it is used as a whole slide.

## 14. Pitfalls

* **Equal segments mistaken for data.** A half donut reads as a chart of shares. With
  equal segments and no numbers it is a layout: do not add a percent sign anywhere unless
  the angles follow the values. This is the data-integrity rule here.
* **Labels that do not line up with their segments.** A and D sit beside the lower
  segments, B and C above the upper ones, each on its segment's axis. A label moved to a
  free corner breaks the only link there is (no leaders, no color).
* **Bold titles.** The design uses one weight. Bold caps titles at 34 px compete with the
  arch; if emphasis is needed, color the title in its segment's hue instead.
* **Hub text too long.** The hub holds two lines of about 11 caps at 36 px. A third line
  or a longer word touches the ring: shrink to 30 px or cut.
* **Shadows too strong.** The folds are whispers (20% opacity, 26 to 30 px blur). At 40% on
  white they look like dirty creases.
