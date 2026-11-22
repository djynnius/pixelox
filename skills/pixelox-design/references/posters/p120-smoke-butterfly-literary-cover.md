---
id: p120
title: Thin title over pigment field and smoke
genre: [book-cover]
mood: [moody, mysterious, elegant, warm]
style: [illustration, gradient-mesh, editorial]
format: portrait-2x3
archetype: A-scene-title
palette: ['#bd302c', '#7e824c', '#58a287', '#cf6530', '#281d1c', '#e3ebec']
techniques: [T-gradient-mesh, T-grunge, T-text-behind-subject, T-smoke, T-paper-grain]
needs: illustration
---

# p120: Thin title over pigment field and smoke

## 1. Concept

A literary-novel cover: a three-line title in thin white capitals floats over a mottled
field of pigment that slides from teal-green at the top to red and orange at the bottom,
while a pale blue butterfly at the right dissolves into a ribbon of dark ink smoke that
threads down through the letters. It works because the type is as light as type can be
and the ground is as rich as a ground can be: the thin white strokes stay readable only
because nothing else on the cover is white, and the one cool object (the butterfly) sits
exactly where the warm and cool halves of the ground meet.

## 2. Format and grid

* Portrait 2:3. Margins 10% W left, 8% W right (the widest title line runs 10.5 to 92.5% W).
* One vertical axis at 51% W carries the author line and all three title lines.
* Zones: author 9.5 to 12.5% H; title 25 to 62.5% H (three lines, pitch 13.8% H);
  illustration 15 to 90% H on a diagonal from upper right (wings centered at 78%, 40%) to
  lower left (smoke curl at 20 to 52% W, 80 to 88% H); tagline at 85.3% H, right of the
  axis.
* The bottom 35% carries no title type: it belongs to the smoke and the red ground. The
  tagline sits in it off-axis (centered at 66% W), at the tip of the smoke curl.

## 3. Layer stack

1. Color wash, full canvas, top to bottom: teal-green `#53b58d` at 0%, sage `#5f9c66` at
   12%, olive `#7e824c` at 27%, brown `#85553a` at 42%, red `#b03a30` at 55%, strong red
   `#c42a2b` at 82%, red-orange `#cf4a2b` at 100%.
2. Stains: large soft patches that break the bands: orange `#e5772d` in the lower-left
   corner (0 to 25% W, 72 to 100% H), teal `#46a57c` top right, olive-brown `#6e6837` at
   the right edge 15 to 35% H, dull red `#a2403d` at the left edge 40 to 50% H, and a
   dark brown shade `#4a342c` in the middle (30 to 60% W, 40 to 52% H).
3. Mottle: cloudy light and dark blotches, 3 to 8% W across, over the whole ground, as
   if the pigment had pooled on wet paper.
4. Ink ribbon (lower copy): a dark blue-black `#14222b` line, 3% W at its thickest,
   tapering at both ends, from 75%, 54% down-left to 19%, 86%, then curling right to 55%,
   85%.
5. Illustration: a butterfly seen from above, wings pale blue `#a4d6e9` to `#4897bb` with
   black borders, wingspan 55% W, turned 25 degrees counter-clockwise, centered at 78%,
   40%; its right wing bleeds off the canvas. Dark translucent smoke drifts from its body
   down and to the left, joining the ribbon.
6. Author name: white, centered on the axis, cap top 9.5% H, 67% W wide.
7. Title: three centered lines, white with a faint gray-to-white vertical gradient and a
   soft shadow; cap tops at 25.3%, 39.1%, 52.9% H; widths 31%, 36%, 82% W. It lies over the
   butterfly's left wing.
8. Ink ribbon (upper copy): the same ribbon again at 40% opacity above the title, so the
   line seems to pass both behind and through the letters of the last title line.
9. Tagline: small white tracked caps centered at 66%, 85.3%.
10. Paper grain: monochrome noise, overlay, 45%.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#bd302c` | red lower half | 35% |
| `#7e824c` | olive band | 20% |
| `#58a287` | teal-green top | 15% |
| `#cf6530` | orange corner and edges | 10% |
| `#281d1c` | ink ribbon, smoke, wing borders | 10% |
| `#e3ebec` | all type | 5% |
| `#a4d6e9` | butterfly wings | 5% |

A complementary pair (green over red) joined by olive and brown, so the join is earthy
instead of harsh. White type; the pale blue wings are the only cool light color.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Author | `[AUTHOR NAME]`, 2 words, 13 letters | caps | light geometric sans (Montserrat 300, Jost 300 / Avenir 300) | 300 | 3% | 160 | - | center | `#ffffff` | 51%, cap top 9.5%; 67% W wide |
| Title | `[TITLE]` on 3 lines: an article (3 letters), a 4-letter word, a 7-letter word | caps | same | 300 | 9.5% | 50 | 104.5% of the font size (pitch 13.8% H) | center | `#ffffff` over `#cdd5d2` | 51%, cap tops 25.3%, 39.1%, 52.9% |
| Tagline | "A NOVEL" | caps | same | 500 | 1.3% | 260 | - | center | `#ffffff` | 66%, 85.3% |

One family, two weights. The stroke of the title letters is only 7% of their cap height;
the open counters let the ground show through, which is the whole effect.

## 6. Imagery

One painted or photographed subject on transparency: a blue butterfly with open wings,
tilted, placed right of center so about a fifth of it leaves the canvas. It turns into
ink dropped in water: dark, soft, drifting down and to the left. Supply the subject as
`illustration.png` (a bird, a flower head, a feather, a key, a hand also work: one object
with a clear outline and one cool or pale color). The smoke is made in the build with
`fx smoke` (drift 235 degrees, down-left), and the ribbon with one tapered `paint stroke`
along a curve, so the illustration itself only has to be the object.

## 7. Effects and textures

* Ground: a 7-stop vertical `fill gradient`, then **T-gradient-mesh** blobs at fixed `at`
  points, blurred with sigma 50, as stains; then a **T-grunge** mottle (`clouds.js`,
  scale 4, 6 octaves) in overlay mode at 55%.
* **T-smoke** as a live style on the illustration: `fx smoke` with drift 235, height
  170%, density 95, color `#0f1b23`.
* **T-text-behind-subject**, adapted: the ribbon is drawn once below the title and
  duplicated above it at 40% opacity, which reads as weaving without any masking.
* Title: `fx overlay` gradient `#cdd5d2` (bottom) to white, and `fx drop-shadow` offset
  2,4, blur 8, 45%.
* **T-paper-grain**, amount 24, overlay 45%: visible grain, part of the painted look.

## 8. Hierarchy and eye path

1. The title: the only large white shapes, in the optical center.
2. The butterfly, caught by the last letters of the first two lines.
3. The ribbon carries the eye down and left, around the curl, and delivers it to the
   tagline.
4. The author line, read last although it is at the top: it is small and thin.

Cap heights: title 9.5 : author 3 : tagline 1.3, about 7.3 : 2.3 : 1.

## 9. Build recipe

`illustration.png` is your image here (one object on transparency, about 600 px wide after
placing). The stand-in will not look like a butterfly; judge the ground, the type, and the
ribbon.

```bash
pixelox render js scripts/blobs.js -o stains.png --size 1080x1620 --seed 3 --param bg=transparent --param colors='#e5772d,#e06a2c,#46a57c,#6e6837,#a2403d,#7a853e,#4a342c' --param at='0.02,0.94;0.06,0.78;0.9,0.02;0.96,0.24;0.02,0.44;0.08,0.16;0.45,0.46' --param size=0.36 --param vary=0.2 --param alpha=0.9
pixelox render js scripts/clouds.js -o mottle.png --size 1080x1620 --seed 17 --param lo='#2c2018' --param hi='#fff4e0' --param scale=4 --param octaves=6 --param warp=0.9 --param contrast=1.5
pixelox new -o cover.pxo --size 1080x1620 --fill '#bd302c'
pixelox layer add cover.pxo --in-place --name Wash
pixelox fill gradient cover.pxo --in-place --layer Wash --from 0,0 --to 0,100% --stops '#4fb48c@0,#5f9c66@0.12,#7a8048@0.27,#85553a@0.42,#ad362f@0.55,#c4282a@0.82,#cf4a2b@1'
pixelox comp place cover.pxo stains.png --in-place --as Stains --fit cover
pixelox filter gaussian cover.pxo --in-place --layer Stains --sigma 50
pixelox comp place cover.pxo mottle.png --in-place --as Mottle --fit cover
pixelox layer blend cover.pxo --in-place --layer Mottle --mode overlay
pixelox layer opacity cover.pxo --in-place --layer Mottle --value 55
pixelox layer add cover.pxo --in-place --name Trail
pixelox paint stroke cover.pxo --in-place --layer Trail --d 'M 810 880 C 650 900 540 1000 430 1080 S 215 1180 205 1390 S 400 1470 600 1385' --size 34 --hardness 85 --spacing 4 --taper 15%,45% --color '#14222b'
pixelox comp place cover.pxo illustration.png --in-place --as Art --size 620x --at 77%,39% --anchor c
pixelox transform rotate cover.pxo --in-place --layer Art --degrees -24
pixelox fx smoke cover.pxo --in-place --layer Art --height 170% --density 95 --turbulence 75 --drift 235 --color '#0f1b23' --variant 7
pixelox text add cover.pxo --in-place --name Author --content "ELEANOR VOSS" --font Avenir --weight 300 --size 4.2% --tracking 160 --color '#ffffff'
pixelox transform scale cover.pxo --in-place --layer Author --size 724x
pixelox transform move cover.pxo --in-place --layer Author --to 51%,9.5% --anchor t
pixelox text add cover.pxo --in-place --name Title --content $'THE\nTHIN\nSILENCE' --font Avenir --weight 300 --size 13.2% --leading 104.5% --tracking 50 --align center --color '#ffffff'
pixelox transform move cover.pxo --in-place --layer Title --to 51%,25.3% --anchor t
pixelox fx overlay cover.pxo --in-place --layer Title --stops '#cdd5d2@0,#ffffff@0.55' --angle 90
pixelox fx drop-shadow cover.pxo --in-place --layer Title --offset 2,4 --blur 8 --opacity 45
pixelox layer dup cover.pxo --in-place --layer Trail --name TrailOver
pixelox layer reorder cover.pxo --in-place --layer TrailOver --above Title
pixelox layer opacity cover.pxo --in-place --layer TrailOver --value 40
pixelox text add cover.pxo --in-place --name Tag --content "A NOVEL" --font Avenir --weight 500 --size 1.85% --tracking 260 --color '#ffffff' --at 66%,85.3% --anchor c
pixelox layer add cover.pxo --in-place --name Grain --fill '#808080'
pixelox filter add-noise cover.pxo --in-place --layer Grain --amount 24 --monochrome
pixelox layer blend cover.pxo --in-place --layer Grain --mode overlay
pixelox layer opacity cover.pxo --in-place --layer Grain --value 45
pixelox export cover.pxo -o cover.png
```

Change first: the three title words (keep the last line the longest; if it is wider than
890 px add `transform scale --size 890x` on the Title layer before the move), then the
start of the `paint stroke` path so the ribbon leaves from your object's lower edge, then
the seven wash stops.

## 10. Variations and pairings

* Palette swaps for the wash, top to bottom: dusk `#27408b`, `#5a3a8a`, `#b0306a`,
  `#e0603a` (the build of its twin, p121); sea `#0e3b4a`, `#1f6f78`, `#6aa58b`,
  `#d8c9a0` with a rust-colored object; ash `#3a3a3a`, `#6a625a`, `#a89880`, `#d9cdb8`
  with a red object.
* Change the object and keep the smoke: a moth, a feather, a wilted flower, a match head.
* For a single short title word, set it at cap height 14% H on one line at 45% H and
  let the object overlap its last letter.
* Film festival or concert poster: author line becomes the presenter, tagline becomes
  date and place; add one more small line at 90% H.
* Pairs with T-double-exposure (a landscape inside the object), T-vignette in dark red at
  60% strength, and T-final-grade with clarity 15 to bind object and ground.

## 11. Pitfalls

* A bolder font "for readability". Weight 300 on a busy ground is the look; readability
  comes from size (cap height 9.5% H), white, and the soft shadow. Do not go above 400.
* A smooth gradient ground. Without the stains and the mottle it reads as a stock
  gradient; the bands must be broken and blotchy.
* White or light patches in the ground behind the title: thin strokes vanish. Keep the
  ground between 25 and 60% lightness everywhere under the type.
* The object centered under the title. It must sit off-axis and leave the canvas on one
  side, so the title stays the center.
* The upper ribbon copy at full opacity cuts letters in two. Keep it at 35 to 45%.
