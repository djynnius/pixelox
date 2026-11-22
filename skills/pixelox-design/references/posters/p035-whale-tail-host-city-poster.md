---
id: p035
title: Whale-tail host-city tournament poster
genre: [sports, event-flyer, exhibition]
mood: [fresh, bold, playful, cool]
style: [illustration, flat, retro]
format: portrait-2x3
archetype: A-scene-title
palette: ['#65bda5', '#072c20', '#245b47', '#a5c856', '#f4faf8']
techniques: [T-waves, T-geo-shapes, T-pattern, T-paper-grain]
needs: nothing
---

# p035: Whale-tail host-city tournament poster

## 1. Concept

A host-city poster for an international football tournament, drawn like a modern woodcut
in five flat colors: a whale's tail rises out of patterned waves in the middle of a teal
sheet, a snow peak sits behind its notch, a line-drawn skyline runs along the horizon, and
a football hangs in the sky top right like a lime-green moon. The event name is one small
bold line top left. It works because one local symbol (the whale) is made monumental and
perfectly symmetric, and because every shape is filled with the same engraved line
texture, so sea, animal, mountain, and ball look cut from one block.

## 2. Format and grid

* Portrait 2:3 (close to it: 1 : 1.52). Margins 5.3% W; text hangs from the left margin.
* Vertical axis at 51 to 53% W: tail notch, tail stem, and mountain peak all sit on it.
* Horizontal bands: sky 0 to 48% H (headline 6 to 12% H, ball 16 to 29% H, mountain 23 to
  34% H, tail flukes 29 to 44% H); horizon with skyline 48 to 55% H; open water 55 to 69%
  H; wave pattern 69 to 100% H.
* The tail is the frame: its flukes span 6 to 95% W, almost the full width, and its stem
  drops from the notch to the waves, splitting the skyline into a left and a right city.
* The ball (20% W across, centered at 85% W, 22.5% H) is the only element off the axis;
  the emblem bottom right (82 to 95% W, 81 to 96% H) balances the headline top left.

## 3. Layer stack

1. Ground: teal `#65bda5`, full canvas (sky and water are the same color).
2. Mountain: a white peak `#f4faf8` with pale teal shading on its left faces, base 36 to
   76% W at 34% H, summit at 53.6% W, 23% H.
3. Skyline: small line-art buildings in white with dark outlines standing on the horizon
   line at 55% H: one cluster with a needle tower at 3 to 30% W, one with towers at 55 to
   94% W, each 5 to 8% H tall; a tiny ferry on the water at 80 to 95% W, 59% H.
4. Water lines: short dark-green horizontal dashes and ripples between 56 and 68% H.
5. Tail: green-black `#072c20`. Two flukes like a wide flattened letter M (tips drooping to
   42% H at 6% W and 95% W, upper edge peaking at 29% H at about 30% W and 72% W, notch at
   51% W, 34% H) on a stem that narrows to 11% W at 47% H and widens to 22% W where it
   enters the waves at 70% H.
6. Tail pattern: fine concentric contour lines in mid green `#245b47` inside the tail, and
   a row of lime `#a5c856` triangles (like teeth) following the upper edge of each fluke.
7. Ball: a lime disc with green-black pentagons and seams, centered at 85% W, 22.5% H.
8. Waves: the bottom 31% is a dense pattern of scalloped, curling wave crests in white,
   lime, and mid green, each filled with parallel contour lines in green-black.
9. Headline: one line of bold condensed caps in green-black, left at 5.3% W, 8.3% H, with a
   two-digit year at its end; a tiny tracked date range under it at 11.3% H.
10. Emblem: a stacked two-digit numeral in lime with a small label box under it, bottom
    right, 13% W wide.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#65bda5` | sky and water | 47% |
| `#072c20` | tail, outlines, type | 28% |
| `#245b47` | contour lines, wave shadow | 10% |
| `#a5c856` | ball, teeth, wave crests, emblem | 8% |
| `#f4faf8` | mountain, wave foam, skyline | 7% |

An analogous green family (teal, forest, lime) plus white. No warm color at all: the
poster owns one corner of the color wheel, which is why it reads as a place.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Headline | `[EVENT NAME]` + 2-digit year, 3 words, about 16 characters | caps | bold condensed grotesque (Oswald 700, Archivo Narrow 700 / Helvetica Neue condensed black) | 900 | 3.1% | 0 | - | left | `#072c20` | 5.3%, 8.3% |
| Dates | month day, dash, month day | caps | same family, regular width (Helvetica Neue) | 700 | 0.9% | 150 | - | left | `#072c20` | 5.3%, 11.3% |
| Emblem numeral | 2 digits, stacked | - | heavy rounded grotesque (Archivo Black / Arial Black) | 900 | 5.5% each | 0 | 90% | center | `#a5c856` | 88.5%, 82 to 93% |
| Emblem label | `[HOST CITY]`, 1 word | caps | Helvetica Neue | 700 | 0.8% | 100 | - | center | `#f4faf8` on `#072c20` | 88.5%, 95% |

Type is deliberately minor: the headline's cap height is 3.1% H on a poster whose main
shape is 41% H tall. One grotesque family does everything.

## 6. Imagery

A flat illustration, no photograph. Subjects: a whale's tail (from behind, symmetric), a
snow-capped volcano, a harbor-city skyline with one needle tower, a car ferry, a football,
stylized waves. Drawing rules that give the woodcut look: (1) every shape is a flat fill;
(2) shading is done with parallel contour lines 2 to 3 px wide in the next darker green,
never with gradients; (3) outlines are green-black and of one weight; (4) waves are
scallops with a curl, repeated like fish scales, alternating white, lime, and green.
To localize the idea, swap the animal and the landmark for those of another city (a
heron and a bridge, a bull and a cathedral) and keep the symmetric "creature frames the
landmark" composition.

## 7. Effects and textures

* **T-geo-shapes**: mountain, tail, teeth, ball, skyline are `shape path`, `shape ellipse`,
  and `shape polygon` layers.
* **T-pattern**: `pattern.js kind=rings` (cell 14, weight 0.25) clipped to the tail for
  the engraved contour lines.
* **T-waves** twice: filled bands (`count=8`, `freq=5`, `amp=0.016`, `chaos=0.6`) for the
  sea, then `mode=line` (`count=30`) in green-black over them; a third, flat line set for
  the open water.
* **T-paper-grain**, very light (amount 8, soft-light 25%): the print is nearly clean.

## 8. Hierarchy and eye path

1. The tail (darkest, largest, centered).
2. The white peak inside its notch, then right to the ball.
3. Down the stem to the horizon and the two skylines.
4. The headline top left, then the emblem bottom right.

The image takes about 85% of the attention. Sizes: tail 41% H tall, ball 13% H, headline
cap height 3.1% H, dates 0.9% H.

## 9. Build recipe

Needs no images: every shape is drawn. Replace the tail, peak, and skyline paths with your
own symbols; keep the symmetric frame and the wave band.

```bash
pixelox render js scripts/pattern.js -o rings.png --size 1080x1620 --param kind=rings --param color='#2f7a5e' --param cell=14 --param weight=0.25 --param cx=0.51 --param cy=0.36 --param alpha=0.7
pixelox render js scripts/waves.js -o water.png --size 1080x1620 --seed 3 --param mode=line --param colors='#245b47' --param count=9 --param top=0.565 --param bottom=0.68 --param amp=0.004 --param freq=6 --param chaos=0.8 --param width=3 --param alpha=0.6
pixelox render js scripts/waves.js -o sea.png --size 1080x1620 --seed 8 --param colors='#0d3a2b,#a5c856,#f4faf8,#245b47,#a5c856,#f4faf8,#245b47,#0d3a2b' --param count=8 --param top=0.7 --param bottom=0.97 --param amp=0.016 --param freq=5 --param chaos=0.6
pixelox render js scripts/waves.js -o crests.png --size 1080x1620 --seed 5 --param mode=line --param colors='#072c20' --param count=30 --param top=0.71 --param bottom=1 --param amp=0.012 --param freq=5 --param chaos=0.5 --param width=3 --param alpha=0.75
pixelox new -o host.pxo --size 1080x1620 --fill '#65bda5'
pixelox shape path host.pxo --in-place --name Peak --d 'M 389 551 L 500 440 L 540 400 L 579 373 L 625 410 L 700 470 L 821 551 Z' --fill '#f4faf8'
pixelox shape path host.pxo --in-place --name PeakShade --d 'M 389 551 L 579 373 L 562 455 L 590 500 L 540 551 Z' --fill '#c5e2dd'
pixelox shape path host.pxo --in-place --name Skyline --d 'M 30 890 V 850 H 60 V 820 H 85 V 850 H 110 V 800 H 118 V 745 H 124 V 800 H 132 V 850 H 170 V 830 H 200 V 860 H 240 V 840 H 270 V 870 H 330 V 890 Z M 600 890 V 860 H 640 V 840 H 670 V 810 H 700 V 780 H 730 V 830 H 760 V 800 H 800 V 770 H 830 V 820 H 870 V 790 H 900 V 840 H 950 V 810 H 990 V 890 Z' --fill '#f4faf8' --stroke '#072c20' --stroke-width 3
pixelox shape line host.pxo --in-place --name Horizon --from 0,890 --to 1080,890 --stroke '#072c20' --stroke-width 3
pixelox comp place host.pxo water.png --in-place --as Water --fit cover
pixelox shape path host.pxo --in-place --name Tail --d 'M 404 1140 C 430 980 470 860 500 760 C 440 700 250 610 67 690 C 90 560 200 470 330 470 C 430 470 520 510 555 548 C 590 510 680 470 780 470 C 900 470 1000 560 1026 700 C 850 610 660 700 610 760 C 620 880 630 1000 640 1140 Z' --fill '#072c20'
pixelox comp place host.pxo rings.png --in-place --as Engraving --fit cover
pixelox layer clip host.pxo --in-place --layer Engraving
pixelox shape path host.pxo --in-place --name Teeth --d 'M 140 600 L 194 562 L 167 638 Z M 194 562 L 249 536 L 221 606 Z M 249 536 L 303 521 L 276 585 Z M 303 521 L 357 517 L 330 576 Z M 357 517 L 411 525 L 384 578 Z M 411 525 L 466 544 L 439 591 Z M 466 544 L 520 575 L 493 616 Z M 590 575 L 643 544 L 616 616 Z M 643 544 L 696 525 L 669 591 Z M 696 525 L 749 517 L 722 578 Z M 749 517 L 801 521 L 775 576 Z M 801 521 L 854 536 L 828 585 Z M 854 536 L 907 562 L 881 606 Z M 907 562 L 960 600 L 934 638 Z' --fill '#a5c856'
pixelox shape ellipse host.pxo --in-place --name Ball --rect 810,257,216x216 --fill '#a5c856' --stroke '#072c20' --stroke-width 5
pixelox shape polygon host.pxo --in-place --name Patch --center 918,365 --radius 46 --sides 5 --rotation -90 --fill '#072c20'
pixelox shape path host.pxo --in-place --name Seams --d 'M 918 319 L 918 259 M 962 351 L 1019 332 M 945 402 L 980 451 M 891 402 L 856 451 M 874 351 L 817 332' --fill none --stroke '#072c20' --stroke-width 5
pixelox comp place host.pxo sea.png --in-place --as Sea --fit cover
pixelox comp place host.pxo crests.png --in-place --as Crests --fit cover
pixelox text add host.pxo --in-place --name Headline --content "HARBOR CITY CUP 31" --font "Helvetica Neue" --weight 900 --size 4.1% --color '#072c20' --at 5.3%,8.3% --anchor l
pixelox text add host.pxo --in-place --name Dates --content "JUNE 4 - JULY 12" --font "Helvetica Neue" --weight 700 --size 1.2% --tracking 150 --color '#072c20' --at 5.4%,11.3% --anchor l
pixelox text add host.pxo --in-place --name Numeral --content $'3\n1' --font "Arial Black" --size 7.6% --leading 90% --align center --color '#a5c856' --at 88.5%,87.5% --anchor c
pixelox fx stroke host.pxo --in-place --layer Numeral --width 4 --color '#072c20' --position outside
pixelox shape rect host.pxo --in-place --name LabelBox --rect 82%,93.8%,13%x2.4% --fill '#072c20'
pixelox text add host.pxo --in-place --name Label --content "YOUR CITY" --font "Helvetica Neue" --weight 700 --size 1.1% --tracking 100 --color '#f4faf8' --at 88.5%,95% --anchor c
pixelox layer add host.pxo --in-place --name Grain --fill '#808080'
pixelox filter add-noise host.pxo --in-place --layer Grain --amount 8 --monochrome
pixelox layer blend host.pxo --in-place --layer Grain --mode soft-light
pixelox layer opacity host.pxo --in-place --layer Grain --value 25
pixelox export host.pxo -o host.png
```

Change first: the Tail path (your own symmetric creature or object), the Skyline path,
then the five colors. Keep the Sea and Crests layers above the tail so it rises out of
the water.

## 10. Variations and pairings

* Other places, same frame: stag antlers around a castle; a palm crown around a
  lighthouse; a bridge's twin towers around a sunset disc. Symmetric symbol, landmark in
  its gap, pattern band at the foot.
* One-corner palettes: desert (sand `#e9c98f`, rust `#a23b1e`, ochre `#e0a030`, cream,
  dark brown); night (indigo `#27306b`, navy `#0c1030`, violet `#7a6cd6`, moon yellow,
  white); coral reef (coral `#f4735c`, maroon, peach, turquoise accent).
* Travel poster, city festival, marathon, food market, tourism board, craft-beer label.
* Square format: crop to 20 to 86% H, move the headline into the sky left of the ball.
* Pairs with T-sunburst (hard flat rays behind the peak), T-frame-border (a white print
  margin), and T-arc-type (the event name on an arc over the ball).

## 11. Pitfalls

* Gradients or soft shadows anywhere: the woodcut look dies at once. Shade with lines and
  flat second tones only.
* An asymmetric or small central symbol: the composition loses its frame. The symbol
  should span 85 to 95% W and sit on the center axis.
* More than five colors, or one warm accent "for pop": it breaks the single-corner
  palette. Use lime as the accent.
* Wave lines too thin (1 px) or too regular: they turn into gray mush or a mechanical
  hatch. Keep 3 px at 1080 px wide and a `chaos` of 0.6 or more.
* A large headline over the illustration. The type stays in the empty sky corner at 3% H
  cap height; the picture is the headline.
