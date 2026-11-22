---
id: p121
title: Pigment-wash novel cover with ink ribbon
genre: [book-cover]
mood: [moody, mysterious, elegant, warm]
style: [illustration, gradient-mesh, editorial]
format: portrait-2x3
archetype: A-scene-title
palette: ['#bd302c', '#7e824c', '#58a287', '#cf6530', '#281d1c', '#e3ebec']
techniques: [T-gradient-mesh, T-grunge, T-text-behind-subject, T-smoke, T-paper-grain]
needs: illustration
---

# p121: Pigment-wash novel cover with ink ribbon

## 1. Concept

The cover of a literary novel in three ingredients: a blotchy watercolor ground that
changes temperature from cool green (top) to hot red (bottom), a stack of very thin white
capitals in the middle, and one small living thing (a pale blue butterfly) that burns
down into a ribbon of black ink. The idea is a boundary made visible: the title sits
exactly on the line where the green turns red, and the ink ribbon crosses that line
through the letters. Thin type plus heavy color is the contrast that carries it.

## 2. Format and grid

* Portrait 2:3. Type margins: 10% W on the left, 8% W on the right.
* Axis at 51% W for the author line and the title. The illustration ignores the axis: it
  runs on a diagonal from 78%, 40% (wings) to 20%, 86% (end of the ribbon), an angle of
  about 50 degrees from the horizontal.
* Vertical rhythm in units of the title's line pitch (13.8% H): author one unit above
  the first title line; three title lines at 25.3%, 39.1%, 52.9% H (cap tops); one and a
  half units of empty ground; tagline at 85.3% H.
* The color break (green to red) falls between 40 and 50% H, behind title line 2.
* Tagline off-axis, centered at 66% W, on the same height as the ribbon's curl.

## 3. Layer stack

1. Wash, top to bottom: `#53b58d` teal-green, `#5f9c66` sage (12%), `#7e824c` olive
   (27%), `#85553a` brown (42%), `#b03a30` red (55%), `#c42a2b` (82%), `#cf4a2b`
   red-orange (100%).
2. Stains, each a soft patch 30 to 40% W across: orange `#e5772d` bottom-left; teal
   `#46a57c` top-right; olive-brown `#6e6837` right edge at 25% H; dull red `#a2403d`
   left edge at 45% H; yellow-olive `#7a853e` left edge at 15% H; dark brown `#4a342c`
   in the middle at 46% H, which is the shadow the title's second line sits on.
3. Mottle: pooled-pigment blotches over everything, darker and lighter by about 15%.
4. Ink ribbon, lower copy: blue-black `#14222b`, 3% W thick in the middle, pointed at
   both ends, one S-curve from 75%, 54% to 19%, 86% with a hook back to 55%, 85%.
5. Illustration: butterfly from above, pale blue wings (`#a4d6e9`, veins and inner
   shading `#4897bb`), black wing borders, wingspan 55% W, rotated 25 degrees
   counter-clockwise, center 78%, 40%, right wing cropped by the canvas edge. Wisps of
   dark, half-transparent smoke fall from it toward the lower left.
6. Author: one line of white tracked caps, cap top 9.5% H, from 18 to 85% W.
7. Title: three lines of white caps, cap height 9.5% H each, widths 31%, 36%, 82% W,
   soft shadow, in front of the wings.
8. Ink ribbon, upper copy at 40% opacity, in front of the title.
9. Tagline: white, centered at 66%, 85.3%.
10. Grain: coarse monochrome noise in overlay mode over everything.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#bd302c` | red, lower half | 35% |
| `#7e824c` | olive, upper middle | 20% |
| `#58a287` | teal-green, top | 15% |
| `#cf6530` | orange, lower-left corner and bottom | 10% |
| `#281d1c` | ink: ribbon, smoke, wing borders | 10% |
| `#e3ebec` | type | 5% |
| `#a4d6e9` | wings | 5% |

Green against red (complements) with every edge between them muddied into olive and
brown. Lightness stays in a narrow middle band (30 to 55%) across the whole ground,
which is what lets thin white type sit anywhere on it.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Author | `[AUTHOR NAME]`, first name 7 letters, surname 6 | caps | light geometric sans (Montserrat 300, Jost 300 / Avenir 300) | 300 | 3% | 160 | - | center | `#ffffff` | 51%, cap top 9.5% |
| Title line 1 | 3-letter word | caps | same | 300 | 9.5% | 50 | pitch 13.8% H | center | `#ffffff`, lower third `#cdd5d2` | 51%, cap top 25.3% |
| Title line 2 | 4-letter word | caps | same | 300 | 9.5% | 50 | same | center | same | 51%, cap top 39.1% |
| Title line 3 | 7-letter word | caps | same | 300 | 9.5% | 50 | same | center | same | 51%, cap top 52.9% |
| Tagline | "A NOVEL" | caps | same | 500 | 1.3% | 260 | - | center | `#ffffff` | 66%, 85.3% |

The three title lines are not fitted to one width: they keep one size, so the block
widens downward (31, 36, 82% W) like a pedestal. The gap between lines (4.3% H) is a
little under half the cap height, loose for capitals, and it is what makes the thin
letters feel calm.

## 6. Imagery

A single realistic subject on transparency, lit softly, with one clear local color that
is absent from the ground (here pale blue against green and red). It is small (a quarter
of the canvas) and partly cropped by the right edge. Its lower part is replaced by ink:
a calligraphic ribbon and loose smoke. Supply the subject as `illustration.png`; the
build adds the smoke with `fx smoke` drifting down-left and paints the ribbon. Good
subjects: a butterfly or moth, a small bird, a flower, a leaf, a paper boat.

## 7. Effects and textures

* **T-gradient-mesh** blobs with fixed `at` positions, blurred (sigma 50), laid over a
  7-stop linear wash: the stains.
* **T-grunge** mottle only (no specks or scratches): `clouds.js` scale 4, octaves 6,
  contrast 1.5, overlay at 55%.
* **T-smoke** as the live `fx smoke` style: drift 235 (down-left), height 170%, density
  95, a near-black tinted toward the ground's darkest hue.
* **T-text-behind-subject** by duplication: ribbon under the title at 100%, the same
  ribbon over the title at 40%.
* Title finish: bottom-to-top `fx overlay` from light gray to white, `fx drop-shadow`
  offset 2,4, blur 8, opacity 45.
* **T-paper-grain**: amount 24, overlay, 45%.

## 8. Hierarchy and eye path

1. Title (largest light shapes, center).
2. Butterfly (the one cool, light object; touches the ends of title lines 1 and 2).
3. Down the ribbon to its curl, then right to the tagline.
4. Author line.

Cap heights 9.5 : 3 : 1.3 (title : author : tagline), about 7.3 : 2.3 : 1. The author is
a third of the title: present, not competing.

## 9. Build recipe

`illustration.png` is your image here. This build keeps the layout and swaps the wash for
a dusk palette (blue, violet, magenta, orange) to show that the structure survives a
change of hue; for the green-to-red original use the stops in section 3.

```bash
pixelox render js scripts/blobs.js -o stains.png --size 1080x1620 --seed 9 --param bg=transparent --param colors='#f08a3c,#e9703a,#2f6fb0,#3a2f7a,#8a2f6a,#4a58a8,#2a1e3c' --param at='0.03,0.95;0.08,0.8;0.92,0.03;0.97,0.26;0.02,0.46;0.1,0.14;0.44,0.47' --param size=0.36 --param vary=0.2 --param alpha=0.9
pixelox render js scripts/clouds.js -o mottle.png --size 1080x1620 --seed 41 --param lo='#1c1830' --param hi='#fff0e6' --param scale=4 --param octaves=6 --param warp=0.9 --param contrast=1.5
pixelox new -o cover.pxo --size 1080x1620 --fill '#b0306a'
pixelox layer add cover.pxo --in-place --name Wash
pixelox fill gradient cover.pxo --in-place --layer Wash --from 0,0 --to 0,100% --stops '#2c64b8@0,#3a4ea8@0.12,#51409a@0.27,#74357f@0.42,#a42f6c@0.55,#c8345a@0.82,#e0603a@1'
pixelox comp place cover.pxo stains.png --in-place --as Stains --fit cover
pixelox filter gaussian cover.pxo --in-place --layer Stains --sigma 50
pixelox comp place cover.pxo mottle.png --in-place --as Mottle --fit cover
pixelox layer blend cover.pxo --in-place --layer Mottle --mode overlay
pixelox layer opacity cover.pxo --in-place --layer Mottle --value 55
pixelox layer add cover.pxo --in-place --name Ribbon
pixelox paint stroke cover.pxo --in-place --layer Ribbon --d 'M 800 870 C 660 880 560 1010 440 1070 S 230 1200 215 1380 S 380 1480 590 1390' --size 34 --hardness 85 --spacing 4 --taper 15%,45% --color '#15122a'
pixelox comp place cover.pxo illustration.png --in-place --as Art --size 620x --at 77%,39% --anchor c
pixelox transform rotate cover.pxo --in-place --layer Art --degrees -24
pixelox fx smoke cover.pxo --in-place --layer Art --height 170% --density 95 --turbulence 75 --drift 235 --color '#120f24' --variant 11
pixelox text add cover.pxo --in-place --name Author --content "MARGOT ELLERY" --font Avenir --weight 300 --size 4.2% --tracking 160 --color '#ffffff'
pixelox transform scale cover.pxo --in-place --layer Author --size 724x
pixelox transform move cover.pxo --in-place --layer Author --to 51%,9.5% --anchor t
pixelox text add cover.pxo --in-place --name Title --content $'ALL\nTHAT\nLINGERS' --font Avenir --weight 300 --size 13.2% --leading 104.5% --tracking 50 --align center --color '#ffffff'
pixelox transform move cover.pxo --in-place --layer Title --to 51%,25.3% --anchor t
pixelox fx overlay cover.pxo --in-place --layer Title --stops '#d6d0e0@0,#ffffff@0.55' --angle 90
pixelox fx drop-shadow cover.pxo --in-place --layer Title --offset 2,4 --blur 8 --opacity 45
pixelox layer dup cover.pxo --in-place --layer Ribbon --name RibbonOver
pixelox layer reorder cover.pxo --in-place --layer RibbonOver --above Title
pixelox layer opacity cover.pxo --in-place --layer RibbonOver --value 40
pixelox text add cover.pxo --in-place --name Tag --content "A NOVEL" --font Avenir --weight 500 --size 1.85% --tracking 260 --color '#ffffff' --at 66%,85.3% --anchor c
pixelox layer add cover.pxo --in-place --name Grain --fill '#808080'
pixelox filter add-noise cover.pxo --in-place --layer Grain --amount 24 --monochrome
pixelox layer blend cover.pxo --in-place --layer Grain --mode overlay
pixelox layer opacity cover.pxo --in-place --layer Grain --value 45
pixelox export cover.pxo -o cover.png
```

Change first: the wash stops (keep seven stops, cool at the top, hot at the bottom, the
break between 40 and 55%), then the title words, then the ribbon path. `--size 13.2%`
gives the 9.5% cap height in Avenir; the font's own letter widths decide the line widths,
so check that the longest line stays under 83% W.

## 10. Variations and pairings

* Twin: p120 is the same cover; its recipe builds the original green-to-red wash with
  other copy. Use the two files together as one layout in two colorways.
* Mirror it: object at 23% W rotated +24 degrees, smoke drift 305, ribbon path flipped
  (x becomes 1080 minus x), tagline at 34% W. Good for a second volume.
* One-hue version for crime or nonfiction: wash from `#101820` to `#3a4a58`, a single
  red object, title unchanged.
* Landscape banner (1600x900): title on one line at cap height 12% H, object at the right
  third, ribbon running left along the baseline.
* Pairs with T-vignette (dark version of the bottom color, strength `99`), T-color-grade
  as a last adjustment layer below the type, and T-double-exposure inside the object.

## 11. Pitfalls

* Even color bands. If the wash looks like a striped flag, the stains are too weak:
  raise their `alpha` or `size` until no horizontal edge survives.
* Mottle in `multiply` drags every color toward brown; `overlay` at 50 to 60% keeps the
  saturation.
* Smoke lighter than the ground looks like fog, not ink. Use a color darker than every
  ground color and density above 85.
* Tracking the title wider than 80: the thin letters fall apart into separate sticks.
* A ribbon that ends in mid-air. Start it under the object and end it in a curl near the
  tagline, so it connects two things.
