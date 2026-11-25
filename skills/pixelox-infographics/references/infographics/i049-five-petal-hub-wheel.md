---
id: i049
title: Five white petals around a hub with colored frames
kind: [list, comparison]
topic: [business, marketing, generic]
mood: [clean, friendly, vibrant, corporate]
style: [paper-cut, flat, line-icon]
format: square
structure: S-hub-spokes
data: [comparison, text-only]
charts: [C-petal-wheel, C-donut, C-icon-badge]
palette: ['#eef1f8', '#fdfdfd', '#06a8bd', '#00b97a', '#f9b401', '#f88d19', '#ec3c51']
needs: nothing
---

# i049: Five white petals around a hub with colored frames

## 1. Concept

A square "five options" wheel on a pale blue-gray ground. Five white wedge-shaped cards fan
out around a white hub like petals; a colored ring hugs the hub and sends a colored spoke
into every gap between two petals, each spoke split lengthwise into the colors of its two
neighbors. So every petal is framed in its own color at its inner edge and along both
sides, and repeats that color in its title, icon, and number. The hub states the count: a
big 5, a small caption, a label, a five-color rule. The message is "five parts of one
thing, all equal". It works because the cards are white and large (room for text) while
color is kept to thin frames: the wheel is colorful and still calm, and one hue ties each
card's pieces together.

## 2. Data story

No numbers except the count and the item numbers. Text only:

* 5 parallel items. Each: a title of one caps word, a note of 8 to 20 words on 2 to 5
  lines, a line icon, and a two-digit number (01 to 05).
* 1 hub: the count as one digit, a one-word caption, a 1 to 2 word label.
* Order: clockwise from the upper right (01) to the upper left (05). The numbers sit at the
  petals' outer corners, small: the order is weak on purpose.

Placeholder content in the recipe (invented, five reasons to switch to a product): PRICE,
IDEAS, SETUP, SPEED, FOCUS, each with a note of 9 to 10 words.

## 3. Information architecture

1. **Hub**, center: how many items and of what.
2. **Petal 01**, upper right: icon nearest the hub, title in the widest part, note under
   it, number at the outer right corner.
3. **Petals 02 to 05**, clockwise: right, bottom, left, upper left. The side petals put the
   icon beside the hub and the text further out; the bottom petal stacks icon, title, note
   downward.
4. **Frames**: the ring and the split spokes. They are the only link between hub and
   petals and act as each petal's color key.

The eye starts at the hub's digit, steps out along any spoke, and lands on a colored title.
Icons sit on the inner third of every petal, so they form a second, loose ring.

## 4. Format and grid

* Square. Recipe canvas 1080 x 1080. Everything on one center: 50% W, 50% H.
* Hub: radius 12.4% W (134 px).
* Colored frame disc: a ring from 12.8% W to 31.2% W (138 to 337 px), cut into five equal
  72 degree sectors starting at 12 o'clock.
* Petals: five sectors from 16% W to 46.2% W (173 to 499 px), separated by straight gaps
  6.2% W (67 px) wide with parallel sides, every corner rounded at 14 px; the gaps are
  centered on 12 o'clock and every 72 degrees from there, exactly over the color changes of
  the frame disc. Margin to the canvas edge: 3.8% W.
* Visible color: a ring 39 px wide between hub and petals, and five spokes of constant
  width (67 px) in the gaps, ending 31.2% W from the center.
* In a petal (distances from the center): icon at 22% W, title at 35% W, note under the
  title in a box 19% W wide, number at 41 to 46% W near the corner that comes first
  clockwise for 01 to 03 and last for 04 and 05 (always an outer corner).

## 5. Layer stack

1. Ground: pale blue-gray `#eef1f8`.
2. Frame disc: five flat sectors, clockwise from 12 o'clock: teal `#06a8bd`, green
   `#00b97a`, amber `#f9b401`, orange `#f88d19`, red `#ec3c51`.
3. Five petals in off-white `#fdfdfd` with rounded corners and a soft blue-gray shadow to
   the lower right. They cover the frame disc except in the straight, parallel-sided gaps
   and next to the hub.
4. Hub: off-white disc `#fbfbfb` with a soft shadow, covering the middle of the frame disc.
5. Hub type: digit in deep teal `#16758b`; caption and label in gray `#6b6b6b`; a rule 80 x
   4 px running through the five hues.
6. Per petal, in its hue: number, 58 px line icon, caps title. Notes in gray `#8a8f98`.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#eef1f8` | ground | 36% | nothing |
| `#fdfdfd` | petals and hub | 54% | nothing |
| `#06a8bd` | frame, title, icon, number of item 01 | 2% | item 01 |
| `#00b97a` | the same for item 02 | 2% | item 02 |
| `#f9b401` | the same for item 03 | 2% | item 03 |
| `#f88d19` | the same for item 04 | 2% | item 04 |
| `#ec3c51` | the same for item 05 | 2% | item 05 |

Categorical: five saturated hues in spectrum order around the wheel (teal, green, amber,
orange, red), each used in small amounts on white. Neutrals: `#16758b` for the hub digit,
`#6b6b6b` and `#8a8f98` for words. Color covers only a tenth of the page.

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Hub digit | 1 digit | - | geometric sans (Montserrat, Poppins / Avenir Next) | 700 | 94 | 0 | - | center | `#16758b` | 50% W, 46.3% H |
| Hub caption | 1 word | caps | geometric sans | 500 | 15 (the original: 12) | 60 | - | center | `#6b6b6b` | 50% W, 51.9% H |
| Hub label | 1 to 2 words, up to 11 letters | caps | geometric sans | 700 | 27 | 20 | - | center | `#6b6b6b` | 50% W, 54.9% H |
| Item title (x5) | 1 word | caps | geometric sans | 700 | 26 | 0 | - | center | the item's hue | 69.3% W / 18.1% H; 82.2% / 56.5%; 50% / 79.4%; 17.7% / 56.5%; 30.3% / 18.1% |
| Item note (x5) | 8 to 12 words on 2 to 3 lines (the original: 5 lines) | sentence | geometric sans | 400 | 16 (the original: 9) | 0 | 21 px | left (the original: justified) | `#8a8f98` | a 205 px box centered under the title, 26 px below its center |
| Item number (x5) | 01 to 05 | - | geometric sans | 700 | 24 | 0 | - | center | the item's hue | outer corners: 87.6% W / 32% H; 79.3% / 79.6%; 31.1% / 86.9%; 9.1% / 43.7%; 43% / 7.8% |

One family, two weights. Sizes 94 : 27 : 26 : 16, about 6 : 1.7 : 1.6 : 1. The original's
notes are 9 px on a 1080 px sheet, unreadable; the recipe trades words for size.

## 8. Charts and diagrams

* **C-petal-wheel** for the petals over a **C-donut** for the colored frame, both
  `scripts/donut.js` with five equal values (`values=1,1,1,1,1`): the angles are all 72
  degrees, so the "chart" is a five-part wheel, not data.
  1. Frame disc (C-donut): `cx=50 cy=50 r=31.2`, `inner=0.41`, five hues,
     `labelpos=none sep=none`.
  2. Petals (C-petal-wheel): `cx=50 cy=50 r=46.2`, `inner=0.346`, `gappx=67` (a gap of
     constant width between petals), `corner=14` (every corner rounded), one color
     `#fdfdfd`, `labelpos=none sep=none`. The gaps let the frame disc show through as
     split spokes of constant width.
* The petal run logs each petal's middle: (737,268), (859,644), (540,876), (221,644),
  (343,268), and the center (540,540). Titles sit near these points; icons halfway from
  there to the hub's edge.
* The hub is a native `shape ellipse` of radius 134 px on the center.
* No axis, no values, no legend. Petal size encodes nothing.
* Simplified: nothing in the wheel's geometry (parallel gaps and rounded corners as in the
  original). The nearest named diagram for the same information is C-hub-spokes (nodes on
  connectors), which loses the card area.

## 9. Icons and illustration

* Five line icons, 58 px, 4 px line, each in its item's hue, on the petal's inner third:
  `search` (01), `lightbulb` (02), `settings` (03), `clock` (04), `target` (05). The original
  shows the same five subjects (a magnifier, a bulb with rays, a cog, a clock, a target with
  an arrow) as two-weight outline icons.
* Icon centers: (671,364), (752,605), (539,764), (327,605), (406,364): about 238 px from the
  center on each petal's axis, shifted toward the title on the side petals.
* No photos.

## 10. Effects and finish

* Petal shadow: `fx drop-shadow --offset 4,10 --blur 24 --color '#42507a' --opacity 22` on
  the placed petal layer: the cards lift off the ground and off the colored disc.
* Hub shadow: `--offset 4,8 --blur 22 --color '#1d2740' --opacity 34`.
* Hub rule: `fx overlay --kind gradient` with five stops, left to right.
* No outlines, no gradients on the wheel itself.

## 11. Hierarchy and eye path

1. The hub digit: largest type, dead center.
2. The colored ring and spokes: the only saturated shapes.
3. The five colored titles, taken clockwise from the upper right or in any order.
4. Icons, then notes; the corner numbers last.

Type steps 94 : 26 : 16. All five petals are identical in size and treatment.

## 12. Build recipe

Nothing to supply. Canvas 1080 x 1080. All copy is invented.

```bash
pixelox new -o five.pxo --size 1080x1080 --fill '#eef1f8'
pixelox render js scripts/donut.js -o frame.png --size 1080x1080 --param cx=50 --param cy=50 --param r=31.2 --param inner=0.41 --param values=1,1,1,1,1 --param labelpos=none --param sep=none --param colors='#06a8bd,#00b97a,#f9b401,#f88d19,#ec3c51'
pixelox comp place five.pxo frame.png --in-place --as Frame --at 0,0
pixelox render js scripts/donut.js -o petals.png --size 1080x1080 --param cx=50 --param cy=50 --param r=46.2 --param inner=0.346 --param values=1,1,1,1,1 --param labelpos=none --param gappx=67 --param corner=14 --param sep=none --param colors='#fdfdfd'
pixelox comp place five.pxo petals.png --in-place --as Petals --at 0,0
pixelox fx drop-shadow five.pxo --in-place --layer Petals --offset 4,10 --blur 24 --color '#42507a' --opacity 22
pixelox shape ellipse five.pxo --in-place --name Hub --rect 406,406,268x268 --fill '#fbfbfb'
pixelox fx drop-shadow five.pxo --in-place --layer Hub --offset 4,8 --blur 22 --color '#1d2740' --opacity 34
pixelox text add five.pxo --in-place --name HubNumber --content "5" --font @geometric-sans --weight 700 --size 94 --color '#16758b' --at 540,500 --anchor c --anchor-to ink
pixelox text add five.pxo --in-place --name HubSmall --content "REASONS" --font @geometric-sans --weight 500 --size 15 --tracking 60 --color '#6b6b6b' --at 540,560 --anchor c --anchor-to ink
pixelox text add five.pxo --in-place --name HubLabel --content "TO SWITCH" --font @geometric-sans --weight 700 --size 27 --tracking 20 --color '#6b6b6b' --at 540,593 --anchor c --anchor-to ink
pixelox shape rect five.pxo --in-place --name HubRule --rect 500,626,80x4 --radius 2 --fill '#06a8bd'
pixelox fx overlay five.pxo --in-place --layer HubRule --kind gradient --stops '#06a8bd@0,#00b97a@0.25,#f9b401@0.5,#f88d19@0.75,#ec3c51@1' --angle 0
pixelox text add five.pxo --in-place --name No1 --content "01" --font @geometric-sans --weight 700 --size 24 --color '#06a8bd' --at 946,346 --anchor c --anchor-to ink
pixelox shape icon five.pxo search --in-place --name Icon1 --size 58 --at 671,364 --anchor c --color '#06a8bd' --stroke-width 4
pixelox text add five.pxo --in-place --name Head1 --content "PRICE" --font @geometric-sans --weight 700 --size 26 --color '#06a8bd' --at 748,196 --anchor c --anchor-to ink
pixelox text add five.pxo --in-place --name Body1 --content "Under the market average for the first full year." --font @geometric-sans --size 16 --leading 21 --box 205x70 --color '#8a8f98' --at 646,222 --anchor tl
pixelox text add five.pxo --in-place --name No2 --content "02" --font @geometric-sans --weight 700 --size 24 --color '#00b97a' --at 856,860 --anchor c --anchor-to ink
pixelox shape icon five.pxo lightbulb --in-place --name Icon2 --size 58 --at 752,605 --anchor c --color '#00b97a' --stroke-width 4
pixelox text add five.pxo --in-place --name Head2 --content "IDEAS" --font @geometric-sans --weight 700 --size 26 --color '#00b97a' --at 888,610 --anchor c --anchor-to ink
pixelox text add five.pxo --in-place --name Body2 --content "A new feature ships every month, picked by users." --font @geometric-sans --size 16 --leading 21 --box 205x70 --color '#8a8f98' --at 786,636 --anchor tl
pixelox text add five.pxo --in-place --name No3 --content "03" --font @geometric-sans --weight 700 --size 24 --color '#f9b401' --at 336,938 --anchor c --anchor-to ink
pixelox shape icon five.pxo settings --in-place --name Icon3 --size 58 --at 539,764 --anchor c --color '#f9b401' --stroke-width 4
pixelox text add five.pxo --in-place --name Head3 --content "SETUP" --font @geometric-sans --weight 700 --size 26 --color '#f9b401' --at 539,858 --anchor c --anchor-to ink
pixelox text add five.pxo --in-place --name Body3 --content "Ready in ten minutes, with no code and no training." --font @geometric-sans --size 16 --leading 21 --box 205x70 --color '#8a8f98' --at 437,884 --anchor tl
pixelox text add five.pxo --in-place --name No4 --content "04" --font @geometric-sans --weight 700 --size 24 --color '#f88d19' --at 98,472 --anchor c --anchor-to ink
pixelox shape icon five.pxo clock --in-place --name Icon4 --size 58 --at 327,605 --anchor c --color '#f88d19' --stroke-width 4
pixelox text add five.pxo --in-place --name Head4 --content "SPEED" --font @geometric-sans --weight 700 --size 26 --color '#f88d19' --at 191,610 --anchor c --anchor-to ink
pixelox text add five.pxo --in-place --name Body4 --content "Answers in under two seconds on any connection." --font @geometric-sans --size 16 --leading 21 --box 205x70 --color '#8a8f98' --at 89,636 --anchor tl
pixelox text add five.pxo --in-place --name No5 --content "05" --font @geometric-sans --weight 700 --size 24 --color '#ec3c51' --at 464,84 --anchor c --anchor-to ink
pixelox shape icon five.pxo target --in-place --name Icon5 --size 58 --at 406,364 --anchor c --color '#ec3c51' --stroke-width 4
pixelox text add five.pxo --in-place --name Head5 --content "FOCUS" --font @geometric-sans --weight 700 --size 26 --color '#ec3c51' --at 327,196 --anchor c --anchor-to ink
pixelox text add five.pxo --in-place --name Body5 --content "One job done well beats ten features nobody opens." --font @geometric-sans --size 16 --leading 21 --box 205x70 --color '#8a8f98' --at 225,222 --anchor tl
pixelox export five.pxo -o five.png
```

Reuse: change the hub's three lines and the five title-and-note pairs. Keep both donut
runs on the same center, and keep `gappx=` and `corner=` on the petal run only. For 4 items
use `values=1,1,1,1` in both runs and four colors (gaps then fall at 12, 3, 6, and 9
o'clock); for 6 items use `gappx=50` and notes of one line. Read the new petal middles with
`lab/anchors.py scripts/donut.js --size 1080x1080 --param cx=50 --param cy=50 --param r=46.2
--param inner=0.346 --param values=1,1,1,1,1 --param labelpos=none --param gappx=67` and put
each title about 75 px from that point toward the outside for the upper petals, on it for
the lower ones.

## 13. Variations and pairings

* **Real shares:** give the petal run real values (`values=40,25,15,12,8`) and the frame
  disc the same values: the wheel becomes a labeled donut with room for a note per slice.
  Then print each share in its title; slices under 10% get too narrow for text.
* **Dark ground:** ground `#141a2b`, petals `#1f2740`, notes `#aab3c9`, hub `#ffffff`; the
  five hues stay and now glow.
* **One hue:** five steps of one color for ranked levels, lightest to darkest clockwise.
* **Cycle:** number the petals large (48 px, near the hub, in place of the icons) and add
  `repeat` as the hub icon when the five items are stages that recur.
* Pairs with C-title-block on a 1080 x 1350 canvas (wheel in the lower 80%) and with
  C-icon-grid as the plain alternative when there are more than six items.

## 14. Pitfalls

* **Equal petals read as equal shares.** A wheel of five equal slices looks like a pie
  chart of 20% each. If the items have sizes, either use the real values (see Variations)
  or say plainly that the wheel is a list. This is the data-integrity rule for wheels.
* **Text that crosses a gap.** A petal's usable width shrinks toward the hub: 205 px boxes
  fit only in the outer half. Keep titles and notes beyond 30% W from the center.
* **Frame colors out of step with the gaps.** The gaps must sit exactly on the frame disc's
  color changes (both runs start at 12 o'clock). A rotated petal run gives each petal two
  frame colors.
* **Gray notes too pale.** `#8a8f98` on white is the lightest that still reads at 16 px;
  the original's pale gray at 9 px does not.
* **Hub smaller than the frame's hole.** If the hub radius drops under the frame disc's
  inner radius, the ground shows as a thin ring. Keep the hub 4 px larger or more.
