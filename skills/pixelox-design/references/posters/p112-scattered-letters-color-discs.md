---
id: p112
title: Orange poster with scattered letters and discs
genre: [exhibition, event-flyer, editorial]
mood: [playful, bold, energetic, warm]
style: [geometric, flat, type-led]
format: portrait-a
archetype: A-geometric-cluster
palette: ['#ef7429', '#1f8f6a', '#722867', '#f33c14', '#ffffff', '#020602']
techniques: [T-geo-shapes, T-image-in-type, T-rules, T-fit-width, T-texture-fill]
needs: nothing
---

# p112: Orange poster with scattered letters and discs

## 1. Concept

An abstract-art poster where a three-letter word is taken apart: each giant white
capital sits on its own flat color disc (purple, green, red) and the three are staggered
down the page like stepping stones. A fourth disc and two triangles are filled with
brush-painted texture, and black is used for a heading, a stacked date, and a small
paragraph. It works because the letters are big enough to read even when scattered, each
disc acts as a spotlight for one letter, and the painted shapes add one rough texture to
an otherwise flat, paper-cut composition.

## 2. Format and grid

* Portrait A-series (1 : 1.414). Margins 6 to 13% W; nothing is centered.
* The cluster of discs occupies the middle: 17 to 92% W, 17 to 77% H. Purple disc: center
  47.6% W, 30.6% H, radius 20.2% W. Green disc: center 38.2% W, 60.2% H, radius 23.8% W.
  Red disc: center 73.6% W, 57.1% H, radius 18% W. Painted disc: center 74.3% W, 29.6%
  H, radius 14.4% W, overlapping the purple disc's right edge.
* Letters: first letter 18 to 48% W, 27 to 46.4% H (on the purple disc, hanging off its
  left side); second 31.7 to 55.6% W, 48.5 to 67.3% H (on the green disc); third 64.2 to
  84.4% W, 39.6 to 59.2% H (bridging the painted disc and the red disc). Read order is a
  zigzag: top left, bottom middle, middle right.
* Painted triangles point into the cluster from two corners: one at the top left (6 to
  30% W, 5.6 to 25.5% H), one at the bottom (47 to 84.5% W, 68 to 94% H).
* Type blocks fill the remaining corners: heading top right, date at the left edge,
  statement at the right edge, paragraph bottom left.

## 3. Layer stack

1. Ground: orange `#ef7429`.
2. Purple disc `#722867`, green disc `#1f8f6a`, red disc `#f33c14`, in that order (the
   green overlaps the purple's lower edge; the red overlaps the green's right edge).
3. Painted shapes (one disc, two triangles): patches of pink `#d860a0`, purple, green,
   white, and red with brushy edges.
4. Three white capitals, each with a cap height of about 19.5% H.
5. Heading: one word in heavy black caps at 35.8 to 87.6% W, 9.4 to 14.5% H.
6. Date: two 2-digit numbers stacked, black, with a thick black bar between them (12.7
   to 28.1% W; numbers at 51.5 to 57.1% H and 62.4 to 68.4% H; bar at 59.2 to 60.4% H).
7. Statement: five lines of bold white caps at 70.3 to 95.2% W, 61.2 to 70.9% H, partly
   on the red disc, partly on the orange.
8. Paragraph: eight lines of small black text at 15.2 to 41.8% W, 75.7 to 86.9% H, across
   the green disc's lower edge.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#ef7429` | ground | 50% |
| `#1f8f6a` | green disc, green paint | 14% |
| `#722867` | purple disc, purple paint | 10% |
| `#f33c14` | red disc, red paint dabs | 8% |
| `#ffffff` | letters, statement, white paint | 10% |
| `#020602` | heading, date, paragraph | 4% |
| `#d860a0` | pink paint | 4% |

Orange with its neighbor red, plus green and purple: three secondary colors and one
primary, all at similar saturation, with white and black for type. Pink appears only in
the paint.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Letters (x3) | one capital each, together a 3-letter word | caps | heavy geometric sans (Jost 800, League Spartan 800 / Futura Bold) | 700 | 19.5% | - | - | free | `#ffffff` | top-left corners at 18%, 27%; 31.7%, 48.5%; 64.2%, 39.6% |
| Heading | 1 word, 8 letters | caps | same | 700 | 5.1% | 0 | - | left | `#020602` | 35.8 to 87.6% W, top 9.4% |
| Date numbers | "DD" over "MM" | numerals | same | 700 | 5.6% | 0 | - | center on 20.4% W | `#020602` | tops 51.5% and 62.4% |
| Statement | about 12 words in 5 lines of 11 to 14 characters | caps | same | 700 | 1.4% | 10 | 114% (pitch 2.05% H) | left | `#ffffff` | 70.3%, top 61.2% |
| Paragraph | about 40 words in 8 lines of 26 to 30 characters | sentence | geometric sans (Jost 500 / Futura Medium) | 500 | 0.9% | 0 | 124% (pitch 1.55% H) | left | `#020602` | 15.2%, top 75.7% |

One family. Four sizes: letters 19.5, date 5.6 (heading 5.1), statement 1.4, paragraph
0.9; each step is at least 3.5x.

## 6. Imagery

No photograph. The discs are flat. The paint texture (the same as in p111) fills one
disc and two triangles: gestural strokes in pink, purple, green, white, and red. It is
generated with `fill noise --as-layer` (close stop positions for hard-edged patches) and
`filter oil-paint`, or replaced by a scan of real paint.

## 7. Effects and textures

* **T-geo-shapes**: three `shape ellipse` discs and one `shape path` holding the painted
  disc and both triangles.
* **T-image-in-type** on shapes: paint layers placed above that path and clipped with
  `layer clip` (several clipped layers in a row share the base).
* Paint: `fill noise --as-layer` with positioned `--stops` (green up to 0.40, pink from
  0.44 to 0.58, purple from 0.62; the narrow gaps are the hard edges), then `filter
  oil-paint` for brush direction. Two more noise layers add dabs: stops from `transparent`
  to white (0.63 to 0.66) and to the red of the red disc (0.65 to 0.68), so only the
  highest part of each noise shows. The dabs are white and red, not the ground orange: a
  ground-colored dab at the rim reads as a bite out of the shape.
* Single letters as shapes: each capital is its own text layer, sized by cap height with
  `--fit-box 600x296` (the box is wider than any letter, so the height decides) and
  placed by the top-left corner of its visible shape (`--anchor-to ink`).
* **T-rules**: the thick bar between the date numbers (1.2% H tall, as wide as the
  numbers).
* **T-fit-width** for the heading (559 px).
* Flat: no shadows or grain.
* **T-texture-fill** in [techniques.md](../techniques.md) is the tested library version of the generated texture clipped into the type or shapes; start from it when reusing this idea.

## 8. Hierarchy and eye path

1. The three white letters, in zigzag order.
2. The black heading at the top (the word that completes the title).
3. The date at the left edge.
4. Statement, then paragraph.

## 9. Build recipe

Nothing to supply. Canvas 1080x1528.

```bash
pixelox new -o letters.pxo --size 1080x1528 --fill '#ef7429'
pixelox shape ellipse letters.pxo --in-place --name Purple --rect 296,250,436x436 --fill '#722867'
pixelox shape ellipse letters.pxo --in-place --name Green --rect 156,662,514x514 --fill '#1f8f6a'
pixelox shape ellipse letters.pxo --in-place --name Red --rect 600,678,390x390 --fill '#f33c14'
pixelox shape path letters.pxo --in-place --name PaintShapes --d 'M 647 452 A 156 156 0 1 1 959 452 A 156 156 0 1 1 647 452 Z M 65 164 L 319 86 L 296 390 Z M 506 1044 L 912 1208 L 639 1434 Z' --fill '#7b2a7a'
pixelox fill noise letters.pxo --in-place --as-layer --name Paint --seed 4 --scale 22% --octaves 5 --warp 80 --stops '#1f8f6a@0.40,#d860a0@0.44,#d860a0@0.58,#7b2a7a@0.62'
pixelox filter oil-paint letters.pxo --in-place --layer Paint --brush 5 --sharpness 90 --impasto 60
pixelox layer clip letters.pxo --in-place --layer Paint
pixelox fill noise letters.pxo --in-place --as-layer --name DabsWhite --seed 19 --scale 12% --octaves 4 --warp 30 --stops 'transparent@0.63,#ffffff@0.66'
pixelox layer clip letters.pxo --in-place --layer DabsWhite
pixelox fill noise letters.pxo --in-place --as-layer --name DabsRed --seed 5 --scale 12% --octaves 4 --warp 30 --stops 'transparent@0.65,#f33c14@0.68'
pixelox layer clip letters.pxo --in-place --layer DabsRed
pixelox text add letters.pxo --in-place --name L1 --content "H" --font Futura --weight 700 --fit-box 600x296 --color '#ffffff' --at 195,413 --anchor-to ink
pixelox text add letters.pxo --in-place --name L2 --content "U" --font Futura --weight 700 --fit-box 600x292 --color '#ffffff' --at 343,740 --anchor-to ink
pixelox text add letters.pxo --in-place --name L3 --content "E" --font Futura --weight 700 --fit-box 600x299 --color '#ffffff' --at 694,605 --anchor-to ink
pixelox text add letters.pxo --in-place --name Heading --content "PIGMENTS" --font Futura --weight 700 --fit-width 559 --color '#020602' --at 35.8%,9.4% --anchor-to ink
pixelox text add letters.pxo --in-place --name DateTop --content "07" --font Futura --weight 700 --size 7.6% --color '#020602' --at 20.4%,51.5% --anchor t --anchor-to ink
pixelox shape rect letters.pxo --in-place --name DateBar --rect 12.7%,59.2%,15.4%x1.2% --fill '#020602'
pixelox text add letters.pxo --in-place --name DateBottom --content "03" --font Futura --weight 700 --size 7.6% --color '#020602' --at 20.4%,62.4% --anchor t --anchor-to ink
pixelox text add letters.pxo --in-place --name Statement --content $'THREE COLORS\nWALK INTO ONE\nROOM AND NONE\nWANTS TO SIT\nIN THE BACK' --font Futura --weight 700 --size 1.8% --tracking 10 --leading 114% --color '#ffffff' --at 70.3%,61.2% --anchor tl
pixelox text add letters.pxo --in-place --name Para --content $'New paintings, prints, and paper\nworks from twelve studios, hung\nby color instead of by name.\nCome and argue about which\nwall is the loudest. Open daily\nfrom ten to six, late on Fridays,\nwith tours for schools on\nrequest. Entry is free.' --font Futura --weight 500 --size 1.25% --leading 124% --color '#020602' --at 15.2%,75.7% --anchor tl
pixelox export letters.pxo -o letters.png
```

Change first: the three letters and the heading word, then the three disc colors, then
the positions: move a letter together with its disc so the letter always overlaps its
disc by at least two thirds. A new `--seed` on `Paint` gives another pattern in the
painted shapes.

## 10. Variations and pairings

* Twin: p111 is the same series (same paint texture, type family, and copy shapes) with a
  different structure: one black blob on red, the word kept whole, the paint seen through
  three round windows. Use p111 when the word must read instantly, p112 when it can be
  played with.
* Four or five letters: add a disc per letter and reduce cap height to 15% H; keep the
  zigzag.
* Calm palette: ground `#f1e9da`, discs `#274c77`, `#a3b18a`, `#e07a5f`, black letters.
* Swap the paint for photographs clipped into the same shapes (a triangle of sky, a disc
  of crowd).
* Suits exhibitions, workshops, kids' art classes, festival identities, zine covers.
  Pairs with T-paper-grain for a printed finish and T-vertical-type for the date.

## 11. Pitfalls

* Letters scattered so far apart or out of order that the word cannot be recovered. Keep
  the reading path a simple zigzag and the gaps under one letter-width.
* Discs the same size and evenly spaced look like traffic lights. Vary radii by 15 to 30%
  and let them overlap.
* White letters on the orange ground alone are weak (contrast is low). Each letter needs
  a darker disc behind most of it.
* Painted shapes larger than the flat discs take over the poster. Keep the paint to about
  12% of the area: one disc, two small triangles.
* Black body text over the green or purple discs is hard to read; keep small black text
  on the orange and small white text on the dark discs.
