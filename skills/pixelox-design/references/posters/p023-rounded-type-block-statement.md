---
id: p023
title: Rounded four-line type block statement
genre: [editorial, social-post, sale-promo]
mood: [bold, playful, friendly, warm]
style: [type-led, flat, minimal]
format: portrait-a
archetype: A-type-block
palette: ['#f3472b', '#ffd1e1', '#ffffff']
techniques: [T-fit-width, T-frame-border, T-stretch-type]
needs: nothing
---

# p023: Rounded four-line type block statement

## 1. Concept

A four-word statement print: fat, round-cornered capitals in pale pink, one word per
line, each line stretched to exactly the same width, on a flat red-orange panel with a
white border. Nothing else: no image, no small print. It works because the fitted lines
turn four words of different lengths into one solid rectangle of type, and because pink
on red is a close, warm pair that feels cheerful where white on red would feel like a
warning sign.

## 2. Format and grid

* Portrait A-series (close to 1 : 1.41). White border 6% W wide on all four sides (the
  same pixel width top and side).
* Type block: 72.4% W wide (14.9 to 87.3% W), so it sits 9% W inside the panel edge on
  each side. Vertically centered: block top at 15% H, bottom at 84.4% H.
* Four lines, one word each, all flush left and flush right. Line gaps are equal: 3.1% H.
* Cap heights follow from the fit: the shorter the word, the taller the line.

## 3. Layer stack

1. Paper: white `#ffffff`, full canvas (shows only as the border).
2. Panel: flat red-orange `#f3472b`, `6%,4.2%,88% x 91.6%`.
3. Line 1: 4 letters, pink `#ffd1e1`, cap top 15% H, cap height 15.2% H.
4. Line 2: 6 letters, pink, cap top 33.4% H, cap height 10.6% H (the longest word, so
   the smallest line).
5. Line 3: 4 letters including a narrow one, pink, cap top 47.2% H, cap height 18.9% H
   (the tallest line).
6. Line 4: 4 letters, pink, cap top 69.2% H, cap height 15.2% H.

No texture, shadow, or outline. Every edge is flat.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#f3472b` | red-orange panel | 53% |
| `#ffd1e1` | pale pink type | 27% |
| `#ffffff` | border | 20% |

Two warm neighbors (red-orange and pink) plus white. The type is the lighter of the two
at a lightness difference of about 35 points: enough to read at any size, soft enough to
stay friendly.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Line 1 | 1 word, 4 letters | caps | heavy rounded grotesque: square shapes with rounded corners (Rubik 900, Baloo 2 800, Lilita One / Arial Rounded MT Bold) | 900 | 15.2% | -10 (stand-in: 20, plus a same-color stroke) | - | justified by fit | `#ffd1e1` | 14.9 to 87.3% W, top 15% |
| Line 2 | 1 word, 6 letters | caps | same | 900 | 10.6% | -10 | - | justified by fit | `#ffd1e1` | same width, top 33.4% |
| Line 3 | 1 word, 4 letters (one of them an I) | caps | same | 900 | 18.9% | -10 | - | justified by fit | `#ffd1e1` | same width, top 47.2% |
| Line 4 | 1 word, 4 letters | caps | same | 900 | 15.2% | -10 | - | justified by fit | `#ffd1e1` | same width, top 69.2% |

One face, one color. Stroke thickness is about 27% of the cap height; counters are small
slots. Line gap 3.1% H = one fifth of the average cap height, close to the white space
inside the letters, which is what fuses the block.

## 6. Imagery

None. The letters are the image: together they cover half of the panel.

## 7. Effects and textures

* **T-fit-width**: every line is added at any size, then `transform scale --size 782x`
  sets its visible width to 72.4% W; `transform move --anchor tl` stacks the lines with a
  fixed gap. Each line's height comes from the `bounds` in the JSON result of the scale
  step; the next line's top = previous top + previous height + gap.
* Faux weight: `text add --stroke` in the fill color with `--stroke-width 9%` (percent of
  the font size) thickens a too-light face by 4.5% of the font size on every side. Add
  tracking 20 so the fattened letters do not touch.
* **T-frame-border**: the border is the white canvas showing around a panel inset by the
  same pixel amount (65 px on a 1080 px wide canvas) on all sides.
* **T-stretch-type** in [techniques.md](../techniques.md) is the tested library version of the stretched, slanted, or fattened lettering; start from it when reusing this idea.

## 8. Hierarchy and eye path

Top to bottom, like reading a sentence; the tallest line (line 3, the verb or key word)
gets the stress. Ratio of cap heights 15.2 : 10.6 : 18.9 : 15.2. Because short words grow
and long words shrink, choose the word order so the word that matters most is the
shortest.

## 9. Build recipe

Nothing to supply. Canvas 1080x1528. The line tops below were computed for these four
words in this font (heights after fitting: 196, 157, 221, 241 px; gap 47 px; block
centered). The stand-in face is not heavy enough on its own, so each line carries a
stroke in its own color, 9% of the font size wide, which fattens the letters and keeps
the corners round.

```bash
pixelox new -o statement.pxo --size 1080x1528 --fill '#ffffff'
pixelox shape rect statement.pxo --in-place --name Panel --rect 65,65,950x1398 --fill '#f3472b'
pixelox text add statement.pxo --in-place --name L1 --content "GOOD" --font "Arial Rounded MT Bold" --size 10% --tracking 20 --color '#ffd1e1' --stroke '#ffd1e1' --stroke-width 9%
pixelox transform scale statement.pxo --in-place --layer L1 --size 782x --anchor tl
pixelox transform move statement.pxo --in-place --layer L1 --to 152,286 --anchor tl
pixelox text add statement.pxo --in-place --name L2 --content "THINGS" --font "Arial Rounded MT Bold" --size 10% --tracking 20 --color '#ffd1e1' --stroke '#ffd1e1' --stroke-width 9%
pixelox transform scale statement.pxo --in-place --layer L2 --size 782x --anchor tl
pixelox transform move statement.pxo --in-place --layer L2 --to 152,529 --anchor tl
pixelox text add statement.pxo --in-place --name L3 --content "TAKE" --font "Arial Rounded MT Bold" --size 10% --tracking 20 --color '#ffd1e1' --stroke '#ffd1e1' --stroke-width 9%
pixelox transform scale statement.pxo --in-place --layer L3 --size 782x --anchor tl
pixelox transform move statement.pxo --in-place --layer L3 --to 152,733 --anchor tl
pixelox text add statement.pxo --in-place --name L4 --content "TIME" --font "Arial Rounded MT Bold" --size 10% --tracking 20 --color '#ffd1e1' --stroke '#ffd1e1' --stroke-width 9%
pixelox transform scale statement.pxo --in-place --layer L4 --size 782x --anchor tl
pixelox transform move statement.pxo --in-place --layer L4 --to 152,1001 --anchor tl
pixelox export statement.pxo -o statement.png
```

Change first: the four words. After changing them, run each `transform scale` with
`--json`, read the new height (fourth number of `bounds`), and recompute the tops: first
top = (1528 - sum of heights - 3 x 47) / 2, then add height + 47 for each next line. With
a truly heavy rounded face registered (`fonts add` a folder with Rubik 900 or Lilita One),
drop the `--stroke` flags and set the tracking to -10. With the stroke, the fitted lines
come out about 1% narrower than the `--size` value and may differ by a few pixels at the
right edge; that is within tolerance.

## 10. Variations and pairings

* Close-pair palettes: cobalt `#1f3fd6` with sky `#bcd4ff`; forest `#1f6b45` with mint
  `#c9f2d6`; plum `#5b2a86` with lilac `#e3ccff`; mustard `#e3a51a` with cream `#fff3c9`.
  Always panel dark and saturated, type light, same hue family or its neighbor.
* Three or five lines instead of four; keep one word per line and equal gaps.
* Add one tiny credit line (cap height 0.9% H, in the type color) centered in the lower
  panel margin for a shop, studio, or event name.
* Square post: border 5% W, block 76% W, two or three lines.
* Pairs with T-distress-type and T-paper-grain for a screen-printed look, T-long-shadow
  for a retro sign, and T-outline-type on one line (hollow) to stress the other three.

## 11. Pitfalls

* Setting all lines at one font size: the block gets a ragged right edge and the idea is
  gone. Fit every line to the same pixel width.
* A word of 9 or more letters next to a 2-letter word: one line becomes a thin strip and
  the other a wall. Keep word lengths between 3 and 7 letters, or split a long word.
* Unequal gaps. Lines are positioned by their visible tops (`--anchor tl` after scaling),
  never by guessing text-box offsets.
* Thin or condensed type: fitted lines then differ wildly in height. Use the heaviest,
  widest rounded face available.
* Pure white type on this red reads as a sale sign; pure black reads as a warning. The
  pale tint of a neighboring hue is the point.
