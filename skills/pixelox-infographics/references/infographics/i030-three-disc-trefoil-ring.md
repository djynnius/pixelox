---
id: i030
title: Three discs fused to a ring around a white hub
kind: [cycle, comparison]
topic: [business, generic]
mood: [clean, friendly, vibrant, minimal]
style: [flat, line-icon]
format: square
structure: S-cycle-ring
data: [cycle, relationship, text-only]
charts: [C-cycle, C-cycle-segments, C-icon-badge]
palette: ['#ffffff', '#6fa80a', '#05cdd6', '#474aa6', '#b9e955', '#68e7ea', '#6766da']
needs: nothing
---

# i030: Three discs fused to a ring around a white hub

## 1. Concept

A square three-part diagram on white. Three large discs (leaf green on top, cyan at the lower
left, indigo at the lower right) sit on the corners of a triangle and melt into a thick ring
that joins them; in the middle a white disc with a soft shadow holds the subject as a gray
icon and two small lines. Each disc carries a white line icon and two lines of white text.
The message is "this one thing rests on three connected parts". It works because the form is
closed and symmetrical: no part comes first, the ring says the parts feed each other, and the
white hub is the only light shape, so the eye starts there and travels around.

## 2. Data story

No numbers. Text only:

* 1 subject in the hub: a label of 1 word in spaced caps and a second line of 2 to 4 words in
  italics.
* 3 parts, equal in weight, each a note of 5 to 7 words on 2 lines. No titles, no numbers on
  the discs: the icon is the title.
* The relationship is a loop without direction (no arrowheads): three aspects that depend
  on each other, or three stages that repeat with no fixed start.

Placeholder content in the recipe (invented): hub TEAMWORK; parts about trust, skills, and a
shared goal.

## 3. Information architecture

1. **Hub**, center: what the diagram is about.
2. **Part 1**, top disc: the first aspect (read first because it is on top).
3. **Part 2**, lower right, and **part 3**, lower left: clockwise from the top.
4. **Ring**: between two neighboring discs the ring takes one color family (the stretch from
   the top disc down the left is green, the bottom stretch cyan, the right stretch indigo),
   with a lighter rim on the outside, so each third of the ring belongs to one disc and
   leads to the next.

There is no title. The diagram is a module: put a headline above it or use it as one slide.

## 4. Format and grid

* Near square (1.05 : 1 in the original). Recipe canvas 1080 x 1080, the diagram centered on
  50% W, 55.6% H; move it to 50% H when nothing sits above it.
* Hub: disc of radius 14.4% W (155 px).
* Ring: inner radius 15% W (162 px), outer radius 22.8% W (246 px): 84 px thick. Its outer
  40 px is the lighter rim.
* Three discs of radius 13.6% W (147 px), centers 28.2% W (305 px) from the middle, at 12,
  4, and 8 o'clock: (50%, 27.4%), (74.5%, 69.8%), (25.5%, 69.8%). Each disc overlaps the
  ring fully and stops 3 px short of the hub.
* Whole figure: 76% W wide and 70% H tall; 12% W free on each side.
* Inside a disc: icon center 37 px above the disc center, text from 35 px below it.

## 5. Layer stack

1. Ground: white.
2. Ring, full thickness, in three 120 degree segments that meet behind the discs: indigo
   `#474aa6` (right), cyan `#05cdd6` (bottom), green `#6fa80a` (left).
3. Rim: the outer 40 px of the ring again, in the light tints: periwinkle `#6766da` (right),
   pale cyan `#68e7ea` (bottom), lime `#b9e955` (left).
4. Three discs: green `#6fa80a` (top), indigo `#474aa6` (lower right), cyan `#05cdd6` (lower
   left). They hide the ends of the ring segments.
5. Hub: white disc with a soft dark shadow all round, so it seems to float above the ring.
6. Hub content: a gray line bulb `#8a9391`, the caps label, the italic line `#a3abaa`.
7. Three white line icons and three white notes on the discs.

In the original the discs and the ring are one merged outline: where a disc meets the ring
the corner is filled in with a concave curve, like drops of liquid joining, and each disc's
color flows into its third of the ring as a ribbon. The recipe overlaps plain circles on
plain ring segments.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#ffffff` | ground, hub, icons and type on color | 62% | nothing |
| `#6fa80a` | top disc, left third of the ring | 9% | part 1 |
| `#474aa6` | lower right disc, right third of the ring | 9% | part 2 |
| `#05cdd6` | lower left disc, bottom third of the ring | 9% | part 3 |
| `#b9e955` | rim, left | 2% | part 1, lighter: "leads on" |
| `#6766da` | rim, right | 2% | part 2, lighter |
| `#68e7ea` | rim, bottom | 2% | part 3, lighter |

Categorical: three saturated hues about a third of the color wheel apart, each with one
light tint. Gray `#8a9391` is used only in the hub, which keeps the subject neutral: it
belongs to all three parts.

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Hub label | 1 word, up to 11 letters | caps | neo-grotesque (Roboto, Inter / Helvetica Neue) | 700 | 18 | 160 | - | center | `#8a9391` | 50% W, 60.2% H |
| Hub line | 2 to 4 words | sentence, italic | neo-grotesque italic | 400 | 18 | 0 | - | center | `#a3abaa` | 50% W, 63.1% H |
| Disc note (x3) | 5 to 7 words on 2 lines | sentence | neo-grotesque | 400 | 20 (the original: 17) | 0 | 25 px | center | `#ffffff` | disc center x; first line 35 px under the disc center |

One family, three treatments (bold spaced caps, italic, regular). All type is small: 18 to
20 px. The shapes are the headline; type only labels them.

## 8. Charts and diagrams

* **C-cycle-segments**, twice, for the ring: `scripts/cycle.js style=segments start=60`,
  three blank labels, `labelwidth=0 badges=none fs=10`, `box=27.2,32.9,45.6,46.4`.
  First with `node=84` (the ring's full thickness) in the three dark hues, then with
  `node=40` (the rim) in the three tints. Both runs log the same center (540,602) and share
  the outer radius 246 px; `start=60` puts the joints of the segments at 12, 4, and 8
  o'clock, under the discs. The script's arrow tips are hidden there too, which removes the
  direction, as in the original.
* **C-cycle** for the discs: `style=nodes`, `node=294`, `line=none` (no connecting arcs),
  `badges=none`, `labelwidth=0`, `box=8,13,84,84.5`; it logs the disc centers (540,295),
  (804,753), (276,753) and the center (540,601). Colors are given clockwise from the top.
* The hub is a native `shape ellipse` at the logged center.
* No axis, no values, no legend. Disc size encodes nothing: all three are equal.
* Simplified: the fused, liquid outline and the ribbon twist of the original (see section
  5). A script option for filleted joins would be needed to match it.

## 9. Icons and illustration

* Disc icons, 110 px, 4 px line, white, centered 37 px above each disc center: `users`
  (top), `brain` (lower right), `chart-no-axes-column-increasing` (lower left). The
  original: three standing figures; a head in profile with a cog inside; rising steps with
  an upward arrow. All thin, even outlines.
* Hub icon: `lightbulb`, 96 px, 4 px line, gray `#8a9391` (the original: a bulb with rays).
* No photos.

## 10. Effects and finish

* Hub shadow: `fx drop-shadow --offset 0,6 --blur 26 --color '#1d2b28' --opacity 34`. It is
  the only effect and gives the single layer of depth.
* Everything else flat.

## 11. Hierarchy and eye path

1. The white hub (highest contrast against the colored ring), with its icon.
2. The top disc, then clockwise: lower right, lower left.
3. The notes inside the discs.

The three discs have identical size, icon size, and text size: nothing ranks them. Only
position (top first) gives a soft order.

## 12. Build recipe

Nothing to supply. Canvas 1080 x 1080. All copy is invented.

```bash
pixelox new -o trio.pxo --size 1080x1080 --fill '#ffffff'
pixelox render js scripts/cycle.js -o ring.png --size 1080x1080 --param box=27.2,32.9,45.6,46.4 --param style=segments --param start=60 --param labels=' | | ' --param labelwidth=0 --param badges=none --param node=84 --param fs=10 --param colors='#474aa6,#05cdd6,#6fa80a'
pixelox comp place trio.pxo ring.png --in-place --as Ring --at 0,0
pixelox render js scripts/cycle.js -o rim.png --size 1080x1080 --param box=27.2,32.9,45.6,46.4 --param style=segments --param start=60 --param labels=' | | ' --param labelwidth=0 --param badges=none --param node=40 --param fs=10 --param colors='#6766da,#68e7ea,#b9e955'
pixelox comp place trio.pxo rim.png --in-place --as Rim --at 0,0
pixelox render js scripts/cycle.js -o nodes.png --size 1080x1080 --param box=8,13,84,84.5 --param labels=' | | ' --param labelwidth=0 --param badges=none --param node=294 --param line=none --param fs=10 --param colors='#6fa80a,#474aa6,#05cdd6'
pixelox comp place trio.pxo nodes.png --in-place --as Nodes --at 0,0
pixelox shape ellipse trio.pxo --in-place --name Hub --rect 385,446,310x310 --fill '#ffffff'
pixelox fx drop-shadow trio.pxo --in-place --layer Hub --offset 0,6 --blur 26 --color '#1d2b28' --opacity 34
pixelox shape icon trio.pxo lightbulb --in-place --name HubIcon --size 96 --at 540,566 --anchor c --color '#8a9391' --stroke-width 4
pixelox text add trio.pxo --in-place --name HubLabel --content "TEAMWORK" --font @neo-grotesque --weight 700 --size 18 --tracking 160 --color '#8a9391' --at 540,650 --anchor c --anchor-to ink
pixelox text add trio.pxo --in-place --name HubSub --content "what makes it work" --font @neo-grotesque --style italic --size 18 --color '#a3abaa' --at 540,682 --anchor c
pixelox shape icon trio.pxo users --in-place --name Icon1 --size 110 --at 540,258 --anchor c --color '#ffffff' --stroke-width 4
pixelox text add trio.pxo --in-place --name Note1 --content $'People who trust\neach other to deliver' --font @neo-grotesque --size 20 --leading 25 --align center --color '#ffffff' --at 540,330 --anchor t
pixelox shape icon trio.pxo brain --in-place --name Icon2 --size 110 --at 804,716 --anchor c --color '#ffffff' --stroke-width 4
pixelox text add trio.pxo --in-place --name Note2 --content $'Skills that cover\nthe whole problem' --font @neo-grotesque --size 20 --leading 25 --align center --color '#ffffff' --at 804,788 --anchor t
pixelox shape icon trio.pxo chart-no-axes-column-increasing --in-place --name Icon3 --size 110 --at 276,716 --anchor c --color '#ffffff' --stroke-width 4
pixelox text add trio.pxo --in-place --name Note3 --content $'A goal everyone\ncan measure weekly' --font @neo-grotesque --size 20 --leading 25 --align center --color '#ffffff' --at 276,788 --anchor t
pixelox export trio.pxo -o trio.png
```

Reuse: change the hub label and the three notes; pick three icons. The disc centers come
from `lab/anchors.py scripts/cycle.js --size 1080x1080 --param box=8,13,84,84.5 --param
labels=' | | ' --param labelwidth=0 --param node=294 --param fs=10`. For four parts give four
blank labels in all three runs, `start=45` for the segments, `node=250` for the discs, and
four colors in each list: the discs then sit at 12, 3, 6, and 9 o'clock. Keep the three
runs' `colors=` in step: the segment list starts with the segment clockwise of the top disc.

## 13. Variations and pairings

* **A real cycle:** leave the discs smaller (`node=200`) so the segment arrow tips show, and
  number the discs (drop `badges=none`): the diagram then reads clockwise as steps.
* **Hub and spokes:** `style=spokes` with the same three colors when the parts do not feed
  each other (C-hub-spokes).
* **Dark ground:** `#14182b` behind, hub `#ffffff`, the same hues; lighten the indigo to
  `#6f73e0` so it does not sink.
* **With titles:** put a 28 px bold caps word above each note and cut the note to one line.
* Pairs with C-title-block above (shift the figure to 58% H) and a C-stat-row under it on a
  1080 x 1350 canvas, one number per part in the part's hue.

## 14. Pitfalls

* **Text too long for a disc.** A 294 px disc with a 110 px icon holds two lines of about 20
  characters at 20 px. A third line touches the edge: cut words or drop the icon.
* **A loop that is really a list.** A ring says the parts connect and repeat. For three
  unrelated facts use C-icon-grid; for three ordered steps add numbers and visible arrows.
  Drawing a relationship the content does not have is this design's data-integrity risk.
* **Unequal discs.** Making the "most important" disc bigger turns a balance diagram into a
  bubble chart with no scale. Keep all three equal, or use C-bars if there are amounts.
* **Segment colors out of step.** The left third of the ring must match the top disc's hue
  family, and so on round. Mixed families look like six unrelated colors.
* **Gray text on the hub that vanishes.** The hub label is small and gray by design; below
  `#8a9391` it fails on white. Do not lighten it further.
