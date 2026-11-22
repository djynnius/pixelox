---
id: p108
title: Fluid gradient ribbons on navy
genre: [corporate, tech-ad]
mood: [energetic, futuristic, bold]
style: [gradient-mesh, illustration]
format: portrait-a
archetype: A-abstract-title
palette: ['#0e1442', '#fd4e1b', '#1786e3', '#8a34c6', '#febd03', '#f51e7d']
techniques: [T-waves, T-geo-shapes, T-fluid-ribbons]
needs: nothing
---

# p108: Fluid gradient ribbons on navy

## 1. Concept

A business flyer cover that is almost all picture: a broad, twisting ribbon of red, orange,
and yellow pours from the top right corner to the bottom edge of a deep navy sheet, crossed
by cooler ribbons in blue and violet, a veil of fine violet lines, and a few hairline curves
that end in white sparkles. The only text is a small, heavy, three-line headline in the top
left corner. It works because the shapes obey one flow (everything streams from the upper
right toward the lower left) and one temperature rule (warm in front, cool behind), so a
very colorful image still reads as a single gesture, and the headline sits in the one
corner the flow never enters.

## 2. Format and grid

* Portrait A-series (1 : 1.414). Text margin 3.8% W left, 3% H top.
* The sheet splits on a diagonal from 60% W at the top edge to the bottom left corner.
  Left and above that line: navy, headline, thin lines. Right and below: solid color.
* Headline block: `3.8%,3%,47% x 11%`, left-aligned, three lines. It is the only thing
  aligned to an edge; everything else is curved.
* The warm ribbon is the spine. Its width changes like a twisted strip of paper: 36% W at
  the top edge, 16% W at 36% H, 31% W at 55% H, 26% W at its waist (76% H), 30% W at the
  bottom edge.
* Empty navy: the left 30% of the sheet from 15 to 60% H, and the lower right corner. Keep
  both; they are where further text (date, logo, contact) goes when the cover is used.

## 3. Layer stack

Back to front:

1. Ground: deep navy `#0e1442`.
2. Glow: a soft blue patch `#0f4a8a` at 80% opacity, centered 30% W, 60% H, radius 35% H,
   fading to nothing.
3. Speckles: about 160 tiny blue dots (`#2433ff`, 0.15 to 0.7% W), clustered in the lower
   right corner.
4. Line veil: 34 parallel hairlines (2 px, violet `#a743c4`, `#8c1ac8`, `#c04fe0`) forming
   a band 11.5% W wide that bends gently and runs from the top edge at 60% W to the left
   edge between 63 and 77% H.
5. Violet band: enters at the right edge between 20 and 42% H and sweeps down-left to
   65% W, 51% H; `#8a34c6` at the top to `#4a39a8` at the bottom.
6. Blue band: enters at the right edge between 37 and 66% H and sweeps down-left to 56% W,
   59 to 73% H; dark blue `#063578` at its lower edge, azure `#1786e3`, royal blue
   `#2a5fe0` at its upper edge. A cyan sliver `#09c0f8` lies inside it, from the right edge
   (54 to 61% H) to a point at 59% W, 65% H.
7. Magenta tails: two tapering tongues that leave the warm ribbon's waist and run left:
   one ends in a point at 5% W, 88% H, the other leaves the bottom edge at 6 to 14% W.
   Magenta `#d81ad0` at the tips to pink `#ff2e6e` at the root.
8. Warm ribbon: covers the top edge from 64% W to the corner, narrows to 65 to 81% W at
   36% H, swings left to 44 to 75% W at 55% H, reaches its leftmost point (33% W) at 76% H,
   then returns right and leaves the bottom edge between 55 and 85% W. Orange `#fd7a0a`
   on its left side to red `#ff2e30` on its right.
9. Yellow fold: a crescent along the ribbon's left edge from 78% W, 16% H to 36% W, 79% H,
   up to 8% W wide; yellow `#ffd000` at the top through `#fea503` to orange `#fd6d0c`.
10. Pink fold: a sliver on the ribbon's upper left edge, from the top edge (63 to 69% W) to
    a point at 65% W, 37% H; `#f51e7d` to `#ff3a5e`.
11. Hairlines: 3 px curves. Two in pink-red `#ff3a5e` (one from 22% W, 26% H wandering up
    to 55% W, 8% H; one from the left edge at 66% H looping under the waist and back up to
    31% W, 56% H) and one in light blue `#3ab8ff` (from 29% W, 98% H up to 91% W, 59% H).
12. Glints: 9 white lens-shaped streaks, 3.7% W long and 0.55% W thick, with a 12 px white
    glow, at the ends and bends of the hairlines.
13. Headline: white `#f2f3f9`, top left.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#0e1442` | navy ground | 45% |
| `#fd4e1b` | warm ribbon (from `#fd7a0a` to `#ff2e30`) | 24% |
| `#1786e3` | blue band (with `#063578`, `#2a5fe0`, cyan `#09c0f8`) | 10% |
| `#8a34c6` | violet band and line veil | 9% |
| `#febd03` | yellow fold | 5% |
| `#f51e7d` | pink fold, magenta tails | 5% |
| `#f2f3f9` | headline, glints | 2% |

A full spectrum held together by a dark ground and by ordering: yellow, orange, red, pink,
magenta, violet, blue, cyan follow each other in space as they do on the wheel, so no two
clashing hues ever touch. Warm colors are opaque and in front; cool ones recede.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Headline | 3 lines: 1 word, 2 words, 1 word; 6 to 14 characters per line | caps | heavy geometric sans (Poppins ExtraBold, Montserrat ExtraBold / Avenir Next) | 900 | 2.8% | 0 | 102% | left | `#f2f3f9` | 3.8%, top 3% |

One text element. It is small for a cover (cap height 2.8% H) and very heavy, with lines
almost touching (gap about 40% of the cap height): a compact white block that holds its
corner against a very loud picture. A cover like this normally gains a date line and a
logo; set them at cap height 1.3% H on the same left edge, in the empty navy.

## 6. Imagery

No photograph. The image is vector-like: seven filled Bezier shapes with linear gradients
(the ribbons and folds), a band of generated contour lines (**T-waves** in line mode), three
stroked hairlines, and small white ellipses with a glow. The ribbons are drawn as closed
paths whose two long edges are S-curves that approach and separate, which is what makes a
flat shape look like a twisting strip; folds are narrower crescents laid along one edge in
a lighter or hotter color, which reads as the strip's other side turning toward the light.

## 7. Effects and textures

* Ribbons (no library technique): `shape path` with cubic curves, then `fx overlay --stops`
  as a two or three stop gradient. Angle 0 (left to right) on the warm ribbon makes its
  left side orange and its right side red; angle 90 (bottom to top) on bands and folds.
  Paths may start and end outside the canvas (`-20`, `1100`, `1548`) so edges bleed.
* Line veil: **T-waves** with `mode=line`, `count=34`, `chaos=0.07` (nearly parallel lines
  that drift apart and together), rendered on a 1700 x 520 strip and rotated -59 degrees.
* Glints: one `shape path` holding nine flat ellipses (`a 20 3 0 1 0 40 0` arcs), white,
  with `fx outer-glow --blur 12`.
* Ground glow: radial `fill gradient` with alpha `cc` to `00`. Speckles: `particles.js`
  with `focus='0.95,0.85,0.3'`.
* **T-geo-shapes** discipline applies to the curves: few control points, shared edges
  (the folds reuse the ribbon's edge coordinates), every shape leaving the canvas.
* No blur, no grain, no shadows between ribbons: overlaps are crisp.
* **T-fluid-ribbons** in [techniques.md](../techniques.md) is the tested library version of the flowing gradient ribbons (`scripts/ribbons.js` draws them directly); start from it when reusing this idea.

## 8. Hierarchy and eye path

1. The yellow fold in the warm ribbon: the lightest, most saturated shape, mid-sheet.
2. Up the ribbon to the top right, then across to the white headline (same height on the
   sheet, strongest light-dark contrast).
3. Down the line veil to the left edge, along the hairlines and glints to the blue band.

There is no type hierarchy: one headline at cap height 2.8% H. Picture to type area is
about 25 : 1. The headline survives only because of isolation and contrast (white on navy
in an empty corner), not size.

## 9. Build recipe

No images needed. Path data is in pixels for a 1080 x 1528 canvas.

```bash
pixelox render js scripts/waves.js -o lines.png --size 1700x520 --seed 4 --param mode=line --param colors='#a743c4,#8c1ac8,#c04fe0' --param count=34 --param top=0.38 --param bottom=0.62 --param amp=0.2 --param freq=0.7 --param chaos=0.07 --param width=2 --param alpha=0.9
pixelox render js scripts/particles.js -o specks.png --size 1080x1528 --seed 6 --param shape=dot --param colors='#2433ff,#1b2ad0' --param count=160 --param min=0.15 --param max=0.7 --param amin=0.2 --param amax=0.7 --param focus='0.95,0.85,0.3'
pixelox new -o flow.pxo --size 1080x1528 --fill '#0e1442'
pixelox layer add flow.pxo --in-place --name Glow
pixelox fill gradient flow.pxo --in-place --layer Glow --kind radial --from 30%,60% --to 30%,95% --stops '#0f4a8acc@0,#0f4a8a00@1'
pixelox comp place flow.pxo specks.png --in-place --as Specks --fit cover
pixelox comp place flow.pxo lines.png --in-place --as Lines --size 1700x --at 30%,36% --anchor c
pixelox transform rotate flow.pxo --in-place --layer Lines --degrees -59
pixelox shape path flow.pxo --in-place --name Violet --d 'M 1100 300 C 980 380 900 520 860 700 L 700 780 C 800 640 960 600 1100 640 Z' --fill '#8a34c6'
pixelox fx overlay flow.pxo --in-place --layer Violet --stops '#4a39a8@0,#8a34c6@1' --angle 90
pixelox shape path flow.pxo --in-place --name Blue --d 'M 1100 560 C 960 640 760 760 600 900 L 620 1120 C 760 1040 940 1000 1100 1010 Z' --fill '#1786e3'
pixelox fx overlay flow.pxo --in-place --layer Blue --stops '#063578@0,#1786e3@0.45,#2a5fe0@1' --angle 90
pixelox shape path flow.pxo --in-place --name Cyan --d 'M 1100 820 C 960 840 800 900 640 1000 C 800 950 960 930 1100 940 Z' --fill '#09c0f8'
pixelox fx overlay flow.pxo --in-place --layer Cyan --stops '#09c0f8@0,#3ad8ff@1' --angle 0
pixelox shape path flow.pxo --in-place --name Tails --d 'M 400 1210 C 300 1230 160 1290 50 1340 C 180 1330 320 1290 420 1260 Z M 440 1260 C 330 1300 160 1380 60 1548 L 150 1548 C 220 1420 360 1340 470 1300 Z' --fill '#f51e7d'
pixelox fx overlay flow.pxo --in-place --layer Tails --stops '#d81ad0@0,#ff2e6e@1' --angle 0
pixelox shape path flow.pxo --in-place --name Warm --d 'M 690 -20 C 700 200 760 300 700 480 C 640 640 480 700 470 880 C 460 980 340 1040 360 1160 C 380 1280 560 1300 600 1548 L 920 1548 C 900 1400 640 1300 640 1140 C 640 960 800 860 850 600 C 880 440 1000 320 1100 260 L 1100 -20 Z' --fill '#fd4e1b'
pixelox fx overlay flow.pxo --in-place --layer Warm --stops '#fd7a0a@0,#fd4e1b@0.45,#ff2e30@1' --angle 0
pixelox shape path flow.pxo --in-place --name Fold --d 'M 840 250 C 820 420 740 540 670 640 C 590 740 560 800 560 920 C 560 1060 470 1160 390 1210 C 365 1100 420 1000 470 880 C 480 700 640 640 700 480 C 730 400 800 320 840 250 Z' --fill '#febd03'
pixelox fx overlay flow.pxo --in-place --layer Fold --stops '#fd6d0c@0,#fea503@0.45,#ffd000@1' --angle 90
pixelox shape path flow.pxo --in-place --name Pink --d 'M 680 -20 C 690 200 750 300 700 480 C 690 520 690 540 700 560 C 760 420 800 330 770 200 C 750 110 740 50 750 -20 Z' --fill '#f51e7d'
pixelox fx overlay flow.pxo --in-place --layer Pink --stops '#f51e7d@0,#ff3a5e@1' --angle 90
pixelox shape path flow.pxo --in-place --name HairWarm --d 'M 240 400 C 340 460 380 420 420 300 C 450 220 560 240 600 120 M 30 1010 C 140 1040 200 1100 260 1260 C 290 1320 360 1330 420 1240 C 470 1160 420 1000 330 860' --fill none --stroke '#ff3a5e' --stroke-width 3 --cap round
pixelox shape path flow.pxo --in-place --name HairCool --d 'M 310 1500 C 420 1440 500 1340 560 1260 C 640 1200 800 1260 880 1080 C 900 1000 960 960 980 900' --fill none --stroke '#3ab8ff' --stroke-width 3 --cap round
pixelox shape path flow.pxo --in-place --name Glints --d 'M 230 398 a 20 3 0 1 0 40 0 a 20 3 0 1 0 -40 0 Z M 10 1010 a 20 3 0 1 0 40 0 a 20 3 0 1 0 -40 0 Z M 240 1262 a 20 3 0 1 0 40 0 a 20 3 0 1 0 -40 0 Z M 400 1242 a 20 3 0 1 0 40 0 a 20 3 0 1 0 -40 0 Z M 310 862 a 20 3 0 1 0 40 0 a 20 3 0 1 0 -40 0 Z M 290 1500 a 20 3 0 1 0 40 0 a 20 3 0 1 0 -40 0 Z M 540 1258 a 20 3 0 1 0 40 0 a 20 3 0 1 0 -40 0 Z M 860 1082 a 20 3 0 1 0 40 0 a 20 3 0 1 0 -40 0 Z M 960 900 a 20 3 0 1 0 40 0 a 20 3 0 1 0 -40 0 Z' --fill '#ffffff'
pixelox fx outer-glow flow.pxo --in-place --layer Glints --blur 12 --spread 0 --color '#ffffff' --opacity 90
pixelox text add flow.pxo --in-place --name Headline --content $'ANNUAL\nPARTNER SUMMIT\nPROGRAM' --font "Avenir Next" --weight 900 --size 3.9% --leading 102% --color '#f2f3f9'
pixelox transform move flow.pxo --in-place --layer Headline --to 3.8%,3% --anchor tl
pixelox export flow.pxo -o flow.png
```

Change first: the headline; then the gradient stops (each ribbon needs only a lighter and
a darker step of one hue). To reshape a ribbon, edit its path in two halves: the first run
of `C` curves is one long edge going down the sheet, the run after the `L` is the other
edge coming back up. Keep the two edges 150 to 350 px apart, and let them pinch once. The
`Fold` path reuses the ribbon's left-edge points (`700 480`, `470 880`), so change both
together.

## 10. Variations and pairings

* Calm corporate: three hues only (navy ground, a teal ribbon `#0fb5a6` to `#0a6f8f`, a
  gold fold `#f2c14e`); drop the violet and magenta shapes.
* Light ground: `#f5f6fa` with the same ribbons, navy headline, line veil in `#c9cbe0`,
  no glints (they need a dark ground).
* Landscape banner (1600 x 900): rotate the whole flow 90 degrees so the ribbon runs left
  to right along the lower half, headline top left.
* Use as a background plate: export without the headline and place it under an
  A-frame-card or A-type-block layout with a navy panel for the text.
* Pairs with T-paper-grain (amount 10, soft-light) to hide gradient banding, with
  T-pill-button under the headline, and with T-vignette to push the corners darker.

## 11. Pitfalls

* Ribbons with parallel edges look like road markings. The two long edges must converge
  and diverge (a pinch of 150 px or less somewhere, a bulge of 300 px or more elsewhere).
* Gradients that run through several hues inside one shape turn it into a rainbow slick.
  One hue per shape, two or three steps of lightness; change hue only between shapes.
* Cool shapes in front of warm ones: the depth flips and the picture goes flat. Draw
  violet and blue first, the warm ribbon and its folds last.
* A big headline. The picture already fills three quarters of the sheet; type larger than
  cap height 4% H competes with it. Keep the block small, heavy, and in the empty corner.
* Glints scattered at random look like dust. Put each one on a hairline's end or bend, and
  keep them thin (6 px tall) with only a small glow.
* Line veil too sparse or too thick: under 25 lines or over 3 px it reads as stripes, not
  as a veil. Use 30 to 40 lines at 2 px.
