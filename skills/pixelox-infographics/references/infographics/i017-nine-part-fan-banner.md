---
id: i017
title: Banner with a nine-part fan in a warm ramp
kind: [list]
topic: [business, generic]
mood: [vibrant, clean, warm, friendly]
style: [flat, gradient]
format: banner
structure: S-half-fan
data: [text-only]
charts: [C-half-donut, C-polar-area, C-icon-badge]
palette: ['#ffffff', '#35366a', '#ba3055', '#e44353', '#fe8663', '#ffcb6d', '#5a5b82']
needs: nothing
---

# i017: Banner with a nine-part fan in a warm ramp

## 1. Concept

A wide white banner whose only picture is half a ring standing on the bottom edge, cut into
nine equal wedges that run from deep navy at the left through crimson and orange to pale
yellow at the right. Each wedge has a round marker on its rim, and nine small text blocks
climb over the arch like a staircase, each titled in its wedge's color. The message is "nine
options, in an order". It works because the color ramp does the numbering: the eye sweeps
the arch from dark to light before reading anything, and the matching title colors tie each
block to its wedge without a single leader line.

## 2. Data story

No numbers. A list of 9 items of equal weight in a fixed order:

* 9 titles of the form "word and two-digit number" (a category word, then 01 to 09).
* 9 notes. In the original each is a block of 5 justified lines of about 45 characters of
  placeholder text set far too small to read; a usable version has 2 lines of at most 24
  characters.
* 1 summary in the ring's hole: the count and one word ("9" and a plural noun).
* 1 footer line of 3 to 4 words with a small mark.

The wedges are **equal** (20 degrees each): the fan is a list, not a pie. The original adds a
darker inner step to every wedge, of varying depth, and markers of varying size; neither
encodes anything.

Placeholder content in the recipe (invented): nine content formats, "Format 01" to "Format
09", each with a two-line note; "9 formats" in the hole.

## 3. Information architecture

1. **The fan** (center, bottom): read left to right, dark to light.
2. **Nine text blocks** around it, in the same order: 01 at the lower left beside the first
   wedge, 05 above the top, 09 at the lower right. Every block is left-aligned, title first.
3. **The count** in the hole, as a quiet summary.
4. **Footer** (bottom right): a small ring mark and a short title for the whole graphic.

The banner has no headline: the footer line is the title, placed last because the picture
explains itself.

## 4. Format and grid

* Banner, 2.57 : 1. Recipe canvas 1600 x 620. Structure `S-half-fan` with two departures
  that the banner format forces: the fan stands on a baseline near the bottom edge
  (84.3% H), and the title is a footer line under it, not a headline above.
* Fan: center 50% W, 84.3% H (on the baseline). In the original the half ring is slightly
  flattened (25.9% W wide to each side, 19.3% W high); the recipe builds a true half circle
  of radius 21% W, hole 48% of the radius.
* Text blocks, left edge and title top in %W, %H: 01 `10.6, 66.3`; 02 `11.7, 45.7`; 03
  `18.3, 28`; 04 `33.1, 15.5`; 05 `48.9, 10`; 06 `64.9, 16.3`; 07 `73.2, 34.9`; 08
  `75.6, 50.6`; 09 `80.2, 70.9`. Block width 10 to 12.5% W. The left blocks step up and to
  the right, the right blocks step down and to the right, so each block starts just outside
  its wedge's rim.
* Count: centered in the hole at 50% W, 76% H.
* Footer: mark at 68.5% W and text from 70.5% W, both at 93% H.
* A hairline under the fan from 11% to 89% W at 84.3% H.

## 5. Layer stack

1. Ground: white.
2. Baseline: a pale 2 px line `#e6e0e6` (the original: a soft shadow that fades to both
   sides).
3. Fan: nine wedges, no gaps and no separators: `#35366a`, `#9a2957`, `#ba3055`, `#da304f`,
   `#e44353`, `#fe8663`, `#ffa868`, `#ffcb6d`, `#ffdc76`.
4. Inner step: the inner part of every wedge is one step deeper in its own hue (a
   multiplied warm red band at 25%); its outer edge moves between 54% and 76% of the radius from wedge to wedge, so
   the fan looks like two stacked rings with a stepped seam.
5. Hole: white, with the count.
6. Nine markers on the rim, each at the middle angle of its wedge: a white disc (radius 14
   to 32 px) holding a smaller disc in the wedge's color. In the original the colored disc is
   drawn as diagonal stripes.
7. Nine titles in the wedge colors and nine notes in gray `#6b6b80`.
8. Footer mark (a small open ring in yellow with a tiny red ring) and footer text in slate
   `#4a4b78`.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#ffffff` | ground, hole, marker discs | 79% | nothing |
| `#35366a` | wedge 01, its title | 2% | position 1 of 9 |
| `#9a2957`, `#ba3055` | wedges 02 and 03 | 5% | positions 2 and 3 |
| `#da304f`, `#e44353` | wedges 04 and 05; the count's digit | 5% | positions 4 and 5 |
| `#fe8663`, `#ffa868` | wedges 06 and 07 | 5% | positions 6 and 7 |
| `#ffcb6d`, `#ffdc76` | wedges 08 and 09 | 4% | positions 8 and 9 |
| `#5a5b82` | notes (`#6b6b80`), count word, footer (`#4a4b78`) | 1% | nothing |

Sequential in look, ordinal in meaning: one ramp through four hues (navy, crimson, orange,
yellow), dark to light, used only to show order. There is no accent: the ramp itself is the
identity. Titles take the wedge colors; the three yellow titles are darkened one step in the
recipe (`#f3a53a`, `#eeb02e`, `#e2a81f`) so they can be read on white.

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Block title (x9) | a word and a two-digit number | title | geometric sans, wide (Montserrat, Poppins / Avenir Next) | 700 | 26 (18 in the original) | 10 | - | left | its wedge's color | see section 4 |
| Block note (x9) | 2 lines of at most 24 characters (the original: 5 justified lines of filler) | sentence | geometric sans | 400 | 17 (9 in the original) | 0 | 22 px | left | `#6b6b80` | 38 px under the title's top |
| Count digit | 1 digit | - | geometric sans | 700 | 34 | 0 | - | left | `#da304f` | in the hole |
| Count word | 1 word | lower | geometric sans | 500 | 34 | 10 | - | left | `#5a5b82` | right of the digit |
| Footer | 3 to 5 words | sentence | geometric sans | 700 | 26 | 10 | - | left | `#4a4b78` | 70.5%, 93% |

One family, two weights, three sizes: 34 : 26 : 17, about 2 : 1.5 : 1. The original's type is
half that size and is the weakest part of the design; the block positions leave room for the
larger sizes.

## 8. Charts and diagrams

* **C-half-donut**, `scripts/donut.js` by center: `cx=50 cy=84.3 r=21`, `sweep=180`,
  `inner=0.48`, nine equal values (`values=1,1,1,1,1,1,1,1,1`), nine `colors=`,
  `labelpos=none`, `sep=none`. No axis, no legend, no numbers: equal wedges carry no data.
* **C-polar-area** for the inner step: a second run of the same script on the same center
  and radius (`cx=50 cy=84.3 r=21 sweep=180 inner=0.48`, nine equal values) with
  `radii=0.74,0.56,0.64,0.54,0.68,0.76,0.56,0.68,0.76` (each wedge's outer edge as a
  fraction of the fan's radius) and one warm red `#c0392b`, `labelpos=none sep=none`,
  placed over the fan and set to 25% opacity in multiply mode with `layer opacity` and
  `layer blend` (a warm multiply deepens the yellows to gold; a dark one turns them olive).
  The radii only shape the seam: they rank nothing and no number goes with them.
* **Rim markers** sit on anchors read from the script: run it once more with
  `r=22.05 inner=0.905` (a thin ring whose middle line is the fan's rim) through
  `lab/anchors.py` or `--json`, and use `slice1` to `slice9`.
* Simplified: the original's half ring is slightly flattened, each wedge has a faint
  gradient toward the rim, and the marker discs are striped; the scripts draw a true half
  circle, flat fills, and plain discs.

## 9. Icons and illustration

* No pictograms. The nine rim markers are **C-icon-badge** discs without icons: white disc,
  colored disc inside at 72% of its radius. Put a 24 to 30 px `shape icon` in white on the
  larger ones if each item has a symbol.
* Footer mark: a 26 px yellow ring with a 10 px red ring at its upper left (two
  `shape ellipse` outlines).

## 10. Effects and finish

* The inner step (multiply, 25%, a different depth in every wedge) is the only shading; it
  makes the flat fan look like two stacked rings.
* The original has a faint gradient across each wedge (lighter toward the rim), a soft
  shadow under the baseline, and soft shadows under the white marker discs. The recipe adds
  one `fx drop-shadow` to the largest markers' discs and leaves the rest flat.

## 11. Hierarchy and eye path

1. The fan: the only large shape, and the only color.
2. Along the ramp from the dark left end to the pale right end.
3. The titles in ramp order, 01 to 09, climbing and descending around the arch.
4. The count in the hole; the footer.

Count 34 px, titles 26, notes 17.

## 12. Build recipe

Nothing to supply. Canvas 1600 x 620. The list is example content.

```bash
pixelox new -o fan.pxo --size 1600x620 --fill '#ffffff'
pixelox shape line fan.pxo --in-place --name Baseline --from 176,524 --to 1424,524 --stroke '#e6e0e6' --stroke-width 2
pixelox render js scripts/donut.js -o wedges.png --size 1600x620 --param cx=50 --param cy=84.3 --param r=21 --param sweep=180 --param inner=0.48 --param values=1,1,1,1,1,1,1,1,1 --param colors='#35366a,#9a2957,#ba3055,#da304f,#e44353,#fe8663,#ffa868,#ffcb6d,#ffdc76' --param labelpos=none --param sep=none
pixelox comp place fan.pxo wedges.png --in-place --as Fan --at 0,0
pixelox render js scripts/donut.js -o step.png --size 1600x620 --param cx=50 --param cy=84.3 --param r=21 --param sweep=180 --param inner=0.48 --param values=1,1,1,1,1,1,1,1,1 --param radii=0.74,0.56,0.64,0.54,0.68,0.76,0.56,0.68,0.76 --param colors='#c0392b' --param labelpos=none --param sep=none
pixelox comp place fan.pxo step.png --in-place --as Step --at 0,0
pixelox layer opacity fan.pxo --in-place --layer Step --value 25
pixelox layer blend fan.pxo --in-place --layer Step --mode multiply
pixelox shape ellipse fan.pxo --in-place --name M1 --rect 437,432,64x64 --fill '#ffffff'
pixelox fx drop-shadow fan.pxo --in-place --layer M1 --offset 0,4 --blur 10 --color '#35366a' --opacity 22
pixelox shape ellipse fan.pxo --in-place --name M1c --rect 446,441,46x46 --fill '#35366a'
pixelox shape ellipse fan.pxo --in-place --name M2 --rect 495,341,28x28 --fill '#ffffff'
pixelox shape ellipse fan.pxo --in-place --name M2c --rect 499,345,20x20 --fill '#9a2957'
pixelox shape ellipse fan.pxo --in-place --name M3 --rect 554,235,60x60 --fill '#ffffff'
pixelox shape ellipse fan.pxo --in-place --name M3c --rect 562,243,44x44 --fill '#ba3055'
pixelox shape ellipse fan.pxo --in-place --name M4 --rect 663,185,44x44 --fill '#ffffff'
pixelox shape ellipse fan.pxo --in-place --name M4c --rect 669,191,32x32 --fill '#da304f'
pixelox shape ellipse fan.pxo --in-place --name M5 --rect 774,161,52x52 --fill '#ffffff'
pixelox shape ellipse fan.pxo --in-place --name M5c --rect 781,168,38x38 --fill '#e44353'
pixelox shape ellipse fan.pxo --in-place --name M6 --rect 883,175,64x64 --fill '#ffffff'
pixelox fx copy fan.pxo --in-place --from M1 --to M6
pixelox shape ellipse fan.pxo --in-place --name M6c --rect 892,184,46x46 --fill '#fe8663'
pixelox shape ellipse fan.pxo --in-place --name M7 --rect 992,241,48x48 --fill '#ffffff'
pixelox shape ellipse fan.pxo --in-place --name M7c --rect 999,248,34x34 --fill '#ffa868'
pixelox shape ellipse fan.pxo --in-place --name M8 --rect 1073,337,36x36 --fill '#ffffff'
pixelox shape ellipse fan.pxo --in-place --name M8c --rect 1078,342,26x26 --fill '#ffcb6d'
pixelox shape ellipse fan.pxo --in-place --name M9 --rect 1103,436,56x56 --fill '#ffffff'
pixelox shape ellipse fan.pxo --in-place --name M9c --rect 1111,444,40x40 --fill '#ffdc76'
pixelox text add fan.pxo --in-place --name T1 --content "Format 01" --font @geometric-sans --weight 700 --size 26 --tracking 10 --color '#35366a' --at 170,411 --anchor tl
pixelox text add fan.pxo --in-place --name N1 --content $'Articles that answer\none clear question' --font @geometric-sans --size 17 --leading 22 --color '#6b6b80' --at 170,449 --anchor tl
pixelox text add fan.pxo --in-place --name T2 --content "Format 02" --font @geometric-sans --weight 700 --size 26 --tracking 10 --color '#9a2957' --at 187,283 --anchor tl
pixelox text add fan.pxo --in-place --name N2 --content $'Short videos made\nfor small screens' --font @geometric-sans --size 17 --leading 22 --color '#6b6b80' --at 187,321 --anchor tl
pixelox text add fan.pxo --in-place --name T3 --content "Format 03" --font @geometric-sans --weight 700 --size 26 --tracking 10 --color '#ba3055' --at 293,174 --anchor tl
pixelox text add fan.pxo --in-place --name N3 --content $'A podcast episode\nevery second week' --font @geometric-sans --size 17 --leading 22 --color '#6b6b80' --at 293,212 --anchor tl
pixelox text add fan.pxo --in-place --name T4 --content "Format 04" --font @geometric-sans --weight 700 --size 26 --tracking 10 --color '#da304f' --at 530,96 --anchor tl
pixelox text add fan.pxo --in-place --name N4 --content $'Email letters with\none idea each' --font @geometric-sans --size 17 --leading 22 --color '#6b6b80' --at 530,134 --anchor tl
pixelox text add fan.pxo --in-place --name T5 --content "Format 05" --font @geometric-sans --weight 700 --size 26 --tracking 10 --color '#e44353' --at 782,50 --anchor tl
pixelox text add fan.pxo --in-place --name N5 --content $'Live sessions with\nquestions at the end' --font @geometric-sans --size 17 --leading 22 --color '#6b6b80' --at 782,88 --anchor tl
pixelox text add fan.pxo --in-place --name T6 --content "Format 06" --font @geometric-sans --weight 700 --size 26 --tracking 10 --color '#f5714f' --at 1038,101 --anchor tl
pixelox text add fan.pxo --in-place --name N6 --content $'Reports built on\nyour own numbers' --font @geometric-sans --size 17 --leading 22 --color '#6b6b80' --at 1038,139 --anchor tl
pixelox text add fan.pxo --in-place --name T7 --content "Format 07" --font @geometric-sans --weight 700 --size 26 --tracking 10 --color '#f3a53a' --at 1171,216 --anchor tl
pixelox text add fan.pxo --in-place --name N7 --content $'Customer stories\ntold by customers' --font @geometric-sans --size 17 --leading 22 --color '#6b6b80' --at 1171,254 --anchor tl
pixelox text add fan.pxo --in-place --name T8 --content "Format 08" --font @geometric-sans --weight 700 --size 26 --tracking 10 --color '#eeb02e' --at 1210,314 --anchor tl
pixelox text add fan.pxo --in-place --name N8 --content $'Checklists people\nprint and keep' --font @geometric-sans --size 17 --leading 22 --color '#6b6b80' --at 1210,352 --anchor tl
pixelox text add fan.pxo --in-place --name T9 --content "Format 09" --font @geometric-sans --weight 700 --size 26 --tracking 10 --color '#e2a81f' --at 1283,440 --anchor tl
pixelox text add fan.pxo --in-place --name N9 --content $'Events that bring\nreaders together' --font @geometric-sans --size 17 --leading 22 --color '#6b6b80' --at 1283,478 --anchor tl
pixelox text add fan.pxo --in-place --name Count --content "9" --font @geometric-sans --weight 700 --size 34 --color '#da304f' --at 726,472 --anchor l
pixelox text add fan.pxo --in-place --name CountWord --content "formats" --font @geometric-sans --weight 500 --size 34 --tracking 10 --color '#5a5b82' --at 756,472 --anchor l
pixelox shape ellipse fan.pxo --in-place --name Mark --rect 1083,564,26x26 --fill none --stroke '#f5b83d' --stroke-width 4
pixelox shape ellipse fan.pxo --in-place --name MarkDot --rect 1076,557,10x10 --fill none --stroke '#da304f' --stroke-width 3
pixelox text add fan.pxo --in-place --name Footer --content "Content formats at a glance" --font @geometric-sans --weight 700 --size 26 --tracking 10 --color '#4a4b78' --at 1128,577 --anchor l
pixelox export fan.pxo -o fan.png
```

Reuse: for N items pass N equal values and N colors, read the rim anchors again (same
command with `r=22.05 inner=0.905`), and center one white disc and one colored disc on each
anchor (`--rect` = anchor minus the radius). Titles keep their wedge's color. The step run
takes the fan's own `cx`, `cy`, `r`, `inner`, and values plus one `radii=` entry per wedge,
each between the fan's `inner` + 0.06 and 0.8. With 5 to 7 items the blocks can hold three
lines.

## 13. Variations and pairings

* **Fewer parts:** 5 or 7 wedges on the same half circle (36 or 25.7 degrees each); skip
  every second ramp color so the ends stay navy and yellow.
* **With data:** real shares instead of equal values make it a plain C-half-donut; then
  write the percentage in each title, sort the values, and drop the varying marker sizes
  and the stepped seam (give the step one radius for all wedges). For a fan whose wedges
  shrink along the ramp put `radii=` on the fan itself (equal values, radii falling from 1
  to 0.6: the half-fan form of C-polar-area) and number the titles, because the lengths
  then look like a ranking.
* **Square version:** a full ring (`sweep=360`) with the blocks around it becomes
  C-hub-spokes or C-cycle-segments; use those scripts for the labels.
* **Other ramps:** teal to lime (`#0f4c5c` to `#d9ed92`), or one hue in nine steps
  (`#0b3c6f` to `#cfe6fb`) for a calmer, corporate look.
* Pairs with C-stat-block in the hole (a total instead of the count) and with C-title-block
  above it in a 16:9 slide.

## 14. Pitfalls

* **Equal wedges read as data.** A fan looks like a pie; nine identical slices invite the
  question "nine equal shares of what?". Say the count ("9 formats") in the hole and never
  put percentages on equal wedges. If the items have sizes, use the real values: this is the
  data-integrity rule for every decorative chart shape.
* **Marker sizes and step depths that look like values.** The original varies both for
  rhythm. A reader will rank the items by them. Vary them only if the size means something;
  otherwise make all markers one size.
* **Text nobody can read.** Five lines at 9 px are decoration. Two lines at 17 px, at most
  24 characters each, are the most this banner holds.
* **Yellow titles on white.** The ramp's light end fails as a text color: darken the last
  three title colors, or set all titles in the dark slate and color only the markers.
* **Blocks that drift from their wedges.** Without leaders, position is the only link: each
  block must start beside its own marker, and the staircase must keep the fan's order.
* **Labels crossing the rim.** Top blocks (04 to 06) sit closest to the arch; keep 10 px
  between their last line and the markers.
