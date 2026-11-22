---
id: p028
title: Dotless didone title on brown cover
genre: [book-cover, editorial]
mood: [calm, elegant, minimal, warm]
style: [type-led, minimal, flat]
format: portrait-2x3
archetype: A-centered-title
palette: ['#52433a', '#e6d8c0', '#decfb0']
techniques: [T-paper-grain]
needs: nothing
---

# p028: Dotless didone title on brown cover

## 1. Concept

A self-help paperback set in two colors and no pictures: a two-word title in a big
high-contrast serif, cream on coffee brown, in which every letter i has lost its dot, while
one solid cream dot sits far away at the top of the page as the bullet of the series label.
It works because the single typographic trick is the content: the dot that left its letter
is "alone", and the reader finds it without being told. Everything else is small, tracked,
and quiet. The source shows the book lying on a cream surface; the flat cover is described.

## 2. Format and grid

* Portrait 2:3. Margins about 9% W on the left and 6% W on the right for the title; the
  small lines keep 25% W or more from the edges.
* Zones: series label with its dot 11 to 15% H; title 34 to 66% H (two lines); author at
  87% H. The bands 15 to 34% H and 66 to 85% H are empty.
* The title is ragged on purpose: line 1 starts at 9.3% W, line 2 is indented to 17.8% W
  and runs out to 93.8% W, so the block steps to the right like a staircase.
* The label is off-axis too: dot at 37.4% W, label text hung from 45.2% W. Only the author
  is centered.

## 3. Layer stack

1. Ground: warm dark brown `#52433a`, full canvas, with the faint tooth of uncoated paper.
2. Dot: a solid cream disc `#e6d8c0`, 5.2% W in diameter, centered at 37.4% W, 13.1% H.
3. Series label: two short lines of small bold tracked caps in cream, left edge at 45.2% W,
   lines centered at 12% and 14% H (the pair is centered on the dot's height).
4. Title line 1: one 5-letter word, mixed case, cream, cap top at 34.6% H, baseline at
   46.5% H, 9.3 to 57.5% W.
5. Title line 2: one 6-letter word, same size, cap top at 48.9% H, baseline at 61.3% H
   (a descender reaches 65.8% H), 17.8 to 93.8% W.
   In both lines the i is a bare stem with no dot; tall letters of line 2 rise to within
   2.4% H of line 1's baseline, so the two lines lock together.
6. Author: small tracked caps in cream, centered at 50% W, 87% H.
7. Fine grain.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#52433a` | ground | 86% |
| `#e6d8c0` | title, dot | 12% |
| `#decfb0` | small type (the same cream, a touch darker at small size) | 2% |

Two colors from one family: a dark roast brown and the cream it would be mixed with.
Contrast is high in lightness and near zero in hue, which is why it feels calm.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Series label | 2 lines, 1 to 2 words each, ending in a roman numeral | caps | geometric sans (Montserrat / Avenir Next) | 700 | 1.3% | 150 | 150% | left | `#decfb0` | left edge 45.2%, 13.1% |
| Title line 1 | 1 word, 5 letters, at least one i | title | high-contrast didone display (Playfair Display, Bodoni Moda, DM Serif Display / Didot) | 700 | 12% | -20 | - | left | `#e6d8c0` | left edge 9.3%, cap top 34.6% |
| Title line 2 | 1 word, 6 letters, at least one i | title | same | 700 | 12% | -20 | 103% of the cap height between baselines plus 2.4% H gap | left | `#e6d8c0` | left edge 17.8%, cap top 48.9% |
| Author | `[AUTHOR NAME]`, 2 short words | caps | geometric sans (Montserrat / Avenir Next) | 600 | 1.4% | 400 | - | center | `#decfb0` | 50%, 87% |

Two families: a didone for the title, one geometric sans for the two small lines. The
size jump is brutal on purpose: title cap height 12% H against 1.3 to 1.4% H, about 9 : 1,
with nothing in between.

## 6. Imagery

None. The only non-letter mark is the dot. Its diameter (5.2% W) is about 1.6 times the
width of the title's thick stems, so it reads as "one of the missing dots, enlarged"
without being a perfect match.

## 7. Effects and textures

* Dotless i: type the Unicode dotless letter (U+0131, "ı") in place of each i. Didot,
  Bodoni 72, and Big Caslon all carry the glyph; check a look-alike font before relying on
  it.
* **T-paper-grain**: `filter grain --as-layer --amount 10`, last: the tooth of an uncoated
  cover stock.
* No shadow, no gradient, no rule.

## 8. Hierarchy and eye path

1. The title (cream on brown, cap height 12% H, the middle of the page).
2. Something is odd about its letters; the eye hunts and finds the dot at the top.
3. The dot hands over to the label beside it.
4. The author at the foot.

Size ratio title : author : label = 12 : 1.4 : 1.3.

## 9. Build recipe

Needs no images. The title strings contain the dotless letter "ı"; copy it from the
recipe.

```bash
pixelox new -o cover.pxo --size 1080x1620 --fill '#52433a'
pixelox shape ellipse cover.pxo --in-place --name Dot --rect 376,184,56x56 --fill '#e6d8c0'
pixelox text add cover.pxo --in-place --name Label --content $'LEARNING TO WAIT\nVOLUME II' --font @geometric-sans --weight 700 --size 1.8% --tracking 150 --leading 150% --color '#decfb0' --at 45.2%,13.1% --anchor l
pixelox text add cover.pxo --in-place --name Line1 --content "Fırst" --font @didone --weight 700 --size 15.7% --tracking -20 --color '#e6d8c0'
pixelox transform move cover.pxo --in-place --layer Line1 --to 9.3%,34.6% --anchor tl
pixelox text add cover.pxo --in-place --name Line2 --content "Wınter" --font @didone --weight 700 --size 15.7% --tracking -20 --color '#e6d8c0'
pixelox transform move cover.pxo --in-place --layer Line2 --to 17.8%,48.9% --anchor tl
pixelox text add cover.pxo --in-place --name Author --content "AUTHOR NAME" --font @geometric-sans --weight 600 --size 1.95% --tracking 400 --color '#decfb0' --at 50%,87% --anchor c
pixelox filter grain cover.pxo --in-place --as-layer --name Grain --amount 10
pixelox export cover.pxo -o cover.png
```

Change first: the two title words (line 2 should be the wider one; if it passes 94% W,
lower both sizes together), then the ground color. Keep the dot where it is.

## 10. Variations and pairings

* Other "one altered letter" tricks in the same layout: a missing crossbar on every A, an
  O replaced by a ring or a disc, one letter dropped below the baseline, a struck-through
  word. One trick per cover.
* Palette pairs with the same logic (dark ground, its own pale tint): navy `#1d2a44` and
  `#e3e8f0`; forest `#22382d` and `#e4e6cf`; oxblood `#4a1f24` and `#f0d9cf`. Reverse any of
  them (pale ground, dark type) for a companion volume.
* A series: keep the grid, change the ground color and the numeral in the label.
* Menu, wine label, wedding or gallery invitation, podcast cover (crop to a square around
  the title and move the dot label to the top-left).
* Pairs with T-frame-border (a 1 px cream keyline at 6%) and T-rules (a short rule above
  the author).

## 11. Pitfalls

* A font whose i keeps its dot, or whose dotless glyph falls back to another face: the
  trick disappears or one letter changes style. Check the render at full size.
* Centering the title or aligning both lines to one left edge: the staircase is what makes
  two short words fill the page. Keep the 8.5% W indent.
* Loose leading: the lines must nearly touch (gap 2 to 3% H). At normal leading they read
  as two separate words, not one mark.
* A light or regular weight: thin didone hairlines vanish in cream on brown at small
  sizes. Use the bold cut and keep the cap height at 10% H or more.
* Adding a subtitle between the title and the author. The empty band is the luxury; put
  any extra line beside the dot.
