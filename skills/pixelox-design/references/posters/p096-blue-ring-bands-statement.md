---
id: p096
title: Blue ring bands with stacked statement
genre: [corporate, tech-ad, conference]
mood: [bold, cool, futuristic, minimal]
style: [swiss, geometric, flat]
format: portrait-a
archetype: A-swiss-asymmetric
palette: ['#020066', '#050aff', '#00b6ff', '#0401d4', '#ffffff']
techniques: [T-geo-shapes, T-paper-grain, T-ring-arcs]
needs: nothing
---

# p096: Blue ring bands with stacked statement

## 1. Concept

A corporate statement poster in one hue: four wide, concentric ring bands in electric blues
sweep across a deep navy ground from a center far below the bottom-left corner, and a
heavy white headline, broken mid-word into five short lines, hangs from the left margin in
front of them. It works because the picture and the type use opposite geometry (big soft
curves against a hard rectangular word stack) and because every band fades into the ground
at one end, so the shapes read as motion and light instead of as clip-art.

## 2. Format and grid

* Portrait A-series (1 : 1.414). One left margin at 12.6% W; the logo, the headline, and
  the body text all hang from it. Top margin 7.5% H, bottom margin 8.5% H.
* One column, 37% W wide (12.6 to 50% W). Nothing is centered and nothing is
  right-aligned; the right half of the canvas belongs to the bands.
* Zones: logo line at 8.6% H; bands 7 to 80% H; headline 44.3 to 76.7% H; empty band of
  ground 77 to 86% H; body text 86.5 to 91.5% H.
* The band system: one center at 12% W, 120% H (below the canvas, under the left margin).
  Five radii in steps of 0.17 W: 0.89, 1.08, 1.25, 1.42, 1.59 W. Each neighboring pair of
  radii bounds one band, so the bands touch but never overlap. Each band is 17% W thick
  (12% H), about twice the headline's cap height.
* Angles are measured clockwise from straight up, seen from the center: the canvas spans
  about -6 degrees (left edge) to 50 degrees (right edge).

## 3. Layer stack

1. Ground: deep navy blue `#020066`, full canvas, flat.
2. Band 4, right part (radii 0.89 to 1.08 W, 10 to 50.5 degrees): from `24%,45%` down to a
   square-cut end at `82%,80%`. Gradient left to right: ground color, then dark blue
   `#02008e` at 40%, then `#0200de` at the cut end. Passes behind the headline.
3. Band 4, left part (same radii, -8 to 14 degrees): enters from the left edge at 44.5 to
   57.5% H. Light azure `#016ffe` at the edge, `#024dfe` at 9% W, faded into the ground by
   28% W.
4. Band 3, left part (radii 1.08 to 1.25 W, -8 to 22 degrees): enters from the left edge at
   32.5 to 44.5% H. Blue `#0401d4` at the edge, faded into the ground by 45% W.
5. Band 3, right part (same radii, from 36.5 degrees to off-canvas): a flat pure-blue
   `#0401ff` block at the right edge, `77 to 100% W, 49 to 77% H`, with a radial cut on its
   left.
6. Band 2 (radii 1.25 to 1.42 W, 16.7 degrees to off-canvas): starts with a radial cut
   from `53%,24%` to `48%,35.5%` and runs off the right edge at 43 to 58% H. Gradient
   `#050aff` at the cut to azure `#0173ff` at the right edge.
7. Band 1 (radii 1.42 to 1.59 W, 2 to 28 degrees): the top band, from 18% W to a radial cut
   at `87%,21%` to `79%,31.5%`. Gradient left to right: ground color until 8%, mid blue
   `#0156af` at 50%, cyan `#00b6ff` at 90%. It is the lightest shape and sits on top.
8. Logo wordmark: white, lowercase, bold, left at 12.6% W, vertical center 8.6% H.
9. Headline: five lines, white, left at 13% W, cap tops at 44.3, 51, 57.6, 64.4, 71% H.
10. Body: three short lines, muted lavender `#8d8dbf`, left at 12.6% W, first line 87% H.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#020066` | ground; the color every band fades into | 60% |
| `#050aff` | bands 2 and 3 (pure electric blue) | 15% |
| `#0401d4` | band 3 left, band 4 end (darker blue) | 10% |
| `#00b6ff` | band 1 end, band 4 left edge (cyan and azure, the highlights) | 8% |
| `#ffffff` | logo and headline | 6% |
| `#8d8dbf` | body text | 1% |

Monochromatic blue: one hue from navy (lightness 20%) through pure blue to cyan, plus
white. The lightest blue is used least and only at band ends, where it works as a highlight.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Logo | `[BRAND]`, one word, 8 to 11 letters | lower | neo-grotesque (Inter / Helvetica Neue) | 700 | 1.3% (x-height) | -20 | - | left | `#ffffff` | 12.6%, center 8.6% |
| Headline | a 3-word imperative of about 17 letters, broken into 5 lines of 2 to 4 letters, splitting two of the words mid-word, ending in a full stop | caps | heavy wide grotesque (Inter Black, Archivo Black / Arial Black) | 900 | 5.6% | -30 | 84% | left | `#ffffff` | 13%, first cap top 44.3% |
| Body | 3 lines, 6 to 8 words each, about 45 characters per line | sentence | neo-grotesque (Inter / Helvetica Neue) | 400 | 0.8% | 0 | 165% | left | `#8d8dbf` | 12.6%, first line 87% |

One grotesque family in three weights. The headline's line gap is only 1.1% H, a fifth of
the cap height, so the five lines fuse into one white block 37% W wide and 32% H tall.

## 6. Imagery

No photograph. The picture is the set of ring bands (layers 2 to 7): flat vector shapes
with a linear gradient inside each. Three things make them look designed and not random:
they share one center, so their curves are parallel; they have one thickness; and each
ends in a straight cut that points at the center (a radial cut), like a slice of a dial.
Read them as a signal spreading, a turbine, or a road bending: abstract enough for any
corporate message about change, growth, or direction.

## 7. Effects and textures

* **T-geo-shapes** arc math, on a larger scale: each band is one `shape path` made of an
  outer arc, a line in, an inner arc back, and `Z`.
* Fade into the ground: `fx overlay --stops` at `--angle 0` (left to right across the
  shape's own bounds), where one stop is the ground color `#020066`. No transparency is
  needed because the ground is flat.
* No shadows, outlines, or blur. The original has no grain; if the gradients show steps
  on your screen, add **T-paper-grain** at amount 8, opacity 25, below the type.
* **T-ring-arcs** in [techniques.md](../techniques.md) is the tested library version of the ring, halo, or arc shapes; start from it when reusing this idea.

## 8. Hierarchy and eye path

1. The white headline block (the only white mass, heaviest weight, left of center).
2. The cyan end of the top band (lightest color, top right); the eye then slides down the
   curves to the right edge and back along band 4 to the headline's last line.
3. The logo, top left, on the same margin.
4. The body text, after a clear gap under the headline.

Cap-height ratio: headline 5.6 : logo 1.3 (x-height) : body 0.8, about 7 : 1.6 : 1. There
is no middle level: one loud line and two whispers.

## 9. Build recipe

Needs no images. All path numbers are for a 1080 x 1528 canvas with the ring center at
`130,1834` and radii 961, 1166, 1350, 1532, 1717 px.

```bash
pixelox new -o bands.pxo --size 1080x1528 --fill '#020066'
pixelox shape path bands.pxo --in-place --name Band4Right --d 'M 332 686 A 1166 1166 0 0 1 1030 1092 L 872 1223 A 961 961 0 0 0 297 888 Z' --fill '#0200de'
pixelox fx overlay bands.pxo --in-place --layer Band4Right --stops '#020066@0,#02008e@0.4,#0200de@1' --angle 0
pixelox shape path bands.pxo --in-place --name Band4Left --d 'M -32 679 A 1166 1166 0 0 1 412 703 L 362 902 A 961 961 0 0 0 -4 882 Z' --fill '#016ffe'
pixelox fx overlay bands.pxo --in-place --layer Band4Left --stops '#016ffe@0,#024dfe@0.25,#020066@0.72' --angle 0
pixelox shape path bands.pxo --in-place --name Band3Left --d 'M -58 497 A 1350 1350 0 0 1 636 582 L 567 753 A 1166 1166 0 0 0 -32 679 Z' --fill '#0401d4'
pixelox fx overlay bands.pxo --in-place --layer Band3Left --stops '#0401d4@0,#010089@0.45,#020066@0.75' --angle 0
pixelox shape path bands.pxo --in-place --name Band3Right --d 'M 934 747 A 1352 1352 0 0 1 1251 1078 L 1097 1182 A 1166 1166 0 0 0 824 897 Z' --fill '#0401ff'
pixelox shape path bands.pxo --in-place --name Band2 --d 'M 570 367 A 1532 1532 0 0 1 1304 849 L 1164 966 A 1350 1350 0 0 0 518 541 Z' --fill '#050aff'
pixelox fx overlay bands.pxo --in-place --layer Band2 --stops '#050aff@0,#0173ff@0.75' --angle 0
pixelox shape path bands.pxo --in-place --name Band1 --d 'M 190 118 A 1717 1717 0 0 1 936 318 L 849 481 A 1532 1532 0 0 0 183 303 Z' --fill '#00b6ff'
pixelox fx overlay bands.pxo --in-place --layer Band1 --stops '#020066@0.08,#0156af@0.5,#00b6ff@0.9' --angle 0
pixelox text add bands.pxo --in-place --name Logo --content "yourbrand" --font "Helvetica Neue" --weight 700 --size 2.4% --tracking -20 --color '#ffffff' --at 12.6%,8.6% --anchor l
pixelox text add bands.pxo --in-place --name Headline --content $'MAKE\nIT\nNEW\nEACH\nDAY.' --font "Arial Black" --size 7.9% --leading 84% --tracking -30 --color '#ffffff'
pixelox transform move bands.pxo --in-place --layer Headline --to 13%,44.3% --anchor tl
pixelox text add bands.pxo --in-place --name Body --content $'Two or three calm sentences that say what is\nchanging, who it is for, and where to find out\nmore. Keep every line under 48 characters.' --font "Helvetica Neue" --weight 400 --size 1.1% --leading 165% --color '#8d8dbf' --at 12.6%,86.3% --anchor tl
pixelox export bands.pxo -o bands.png
```

Change first: the headline (keep 2 to 4 letters per line; with a longer line, lower
`--size` until the block is about 37% W wide). To redraw a band, compute its four corners
from the center: a point at radius `r` and angle `a` is `x = 130 + r sin a`,
`y = 1834 - r cos a`; the path is `M outer(a0) A r1 r1 0 0 1 outer(a1) L inner(a1) A r0 r0
0 0 0 inner(a0) Z`. Keep the radii and move only the angles.

## 10. Variations and pairings

* Other single hues: ground `#2a0050` with `#7a1bff`, `#b14dff`, `#ff5ad1`; ground
  `#00231c` with `#00a86b`, `#19e68c`, `#b8ff5a`; ground `#1a0500` with `#d43a00`,
  `#ff7a1a`, `#ffc24a`. Always ground at 20% lightness, one pure mid tone, one light tip.
* Light version: ground `#f2f4ff`, bands fading into it, headline in `#020066`.
* Mirror it (center below the bottom-right corner, text column on the right margin) or
  move the center above the top-left corner so the bands sweep under the headline.
* Suits annual-report covers, tech conferences, product launches, recruitment posters,
  slide title pages. For a landscape banner keep the center under the left margin and put
  the headline in two lines.
* Pairs with T-rules (a short white accent bar between headline and body), T-vertical-type
  (a small rotated date on the right edge), and T-pattern dots at alpha 0.15 in the empty
  bottom-right corner.

## 11. Pitfalls

* Bands with different centers or thicknesses look like scattered boomerangs. Use one
  center and one radius step for all of them.
* Ends cut horizontally or vertically break the dial feeling. The end line must point at
  the center: both end points have the same angle.
* A gradient that fades to black or to transparent gray leaves a dirty edge on navy. The
  dark stop must be exactly the ground color.
* More than one hue (a purple or green band) turns it into a generic colorful template;
  the single-hue rule is what makes it look like a brand system.
* A headline with long lines covers the bands and loses the block shape. Break the words;
  5 short lines beat 3 long ones, and the full stop at the end closes the block.
* Body text in pure white competes with the headline. Keep it a muted tint of the ground
  hue (`#8d8dbf`) and under 1% H cap height.
