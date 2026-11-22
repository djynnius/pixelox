---
id: p124
title: Light halftone collage with vertical type
genre: [exhibition, event-flyer]
mood: [bold, playful, fresh, energetic]
style: [halftone, collage, type-led]
format: portrait-2x3
archetype: A-vertical-type
palette: ['#e4e4e4', '#2b2b2b', '#fa7a14', '#ef2b47', '#da2073', '#ffd83a']
techniques: [T-vertical-type, T-halftone-portrait, T-cutout-drop-shadow]
needs: subject-cutout
---

# p124: Light halftone collage with vertical type

## 1. Concept

An art-event poster on pale gray: a halftone hand reaches diagonally from the lower left
toward a grainy cloud of magenta, orange, and yellow in the upper right, with a flat red
disc behind its wrist, and two heavy black words stand on their sides at the left and right
edges. It works because three kinds of mark are kept strictly apart: flat (disc, type),
dotted (the photo), and sprayed (the gradient). The diagonal of the hand is the only
slanted line on a sheet of verticals, so it carries all the movement.

## 2. Format and grid

* Portrait 2:3. Margins: 6% W left, 3% W right, 10.5% H top (to the first letter), 7.5% H
  bottom.
* Five vertical tracks, from the left:
  track 1, x 6 to 20.5% W: big word one;
  track 2, x 24.5 to 29% W: the date;
  track 3, x 34 to 38.5% W: the time;
  middle, x 0 to 79% W: the cutout, crossing tracks 1 to 3;
  track 4, x 81 to 97.5% W: a sideways note on top, big word two under it.
* All rotated text is turned -90 degrees (reads bottom to top). Date, time, big word two,
  and the bottom of the horizontal paragraph share one bottom line at 91.5 to 92.5% H.
  Big word one starts higher, at 73% H, and runs up to 10.5% H.
* The picture runs on the opposite diagonal to the type's weight: type is heavy at top left
  and bottom right, the image goes from bottom left (wrist) to top right (color cloud).
  The two diagonals cross at the disc.
* The right 20% of the sheet above 60% H holds only the note and the lower edge of the
  cloud; the upper left quarter beside word one is empty ground.

## 3. Layer stack

1. Ground: pale neutral gray `#e4e4e4`, full canvas.
2. Disc: flat red-pink `#ef2b47`, diameter 40.5% W, center at 10% W, 53.5% H, so the left
   quarter of it is cut off by the edge. Visible box `0%,40%,30.3% x 27%`.
3. Color cloud: top right corner, box `56.5%,0%,43.5% x 38%`. Its left side is a soft
   diagonal from 74% W at the top edge down to 61.5% W at 22% H, it ends in a point at
   56.5% W, 34.5% H, and its underside rises from 70% W, 38% H to the right edge at 31% H.
   Filled top to bottom: magenta `#da2073` (0 to 12% H), red-orange `#f0503a` (17% H),
   orange `#fa7a14` (22 to 31% H), yellow `#ffd83a` (36 to 38% H). The edge is not smooth:
   it breaks into fine grain over a band about 2% W wide, like spray paint.
4. Halftone cutout: a hand and forearm entering from the lower left corner, fingers
   spread, index finger touching the point of the cloud. Box `0%,24%,79% x 66%`; axis about
   55 degrees above horizontal. Round dots `#2b2b2b` on the ground color, pitch 1.6% W,
   screen at 45 degrees; an off-white outline `#f4f4f4`, 0.8% W wide, follows the contour
   like the white edge of a cut-out sticker. It covers the right part of the disc.
5. Big word one: black, rotated -90, x 6 to 20.5% W, from y 73% up to 10.5% H; it crosses
   the disc and the wrist and stays fully black on both.
6. Big word two: black, rotated -90, x 82.5 to 97% W, from y 91.5% up to 64% H.
7. Date: red-pink `#ef2b47`, light weight, rotated -90, x 24.5 to 29% W, y 92.5% up to 49%.
8. Time: same, x 34 to 38.5% W, y 92.5% up to 73% H.
9. Paragraph: dark gray `#2f2f2f`, horizontal, five short lines, `41%,85.5%,38% x 7.5%`.
10. Side note: dark gray, five lines rotated -90 as one block, x 81 to 97.5% W, from y 60%
    up to 23.5% H; its upper lines run over the yellow underside of the cloud.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#e4e4e4` | ground, halftone paper | 55% |
| `#2b2b2b` | halftone dots; type in `#000000` and `#2f2f2f` | 25% |
| `#fa7a14` | cloud, middle band | 7% |
| `#ef2b47` | disc, date, time | 7% |
| `#da2073` | cloud, top band | 4% |
| `#ffd83a` | cloud, lower rim | 2% |

Neutral gray and black carry four fifths of the sheet; color is one warm run from magenta
through red and orange to yellow, used in two places only. The disc color sits inside that
run (between the magenta and the orange), which is why disc and cloud look related although
one is flat and the other sprayed.

## 5. Text elements

Cap heights of rotated lines are measured across the line (their width on the canvas).

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Big word one | 1 word, 6 letters | caps | heavy geometric sans (Montserrat Black / Avenir Next) | 900 | 9.7% H (14.5% W) | 20 | - | rotated -90 | `#000000` | x 6 to 20.5%, y 73% up to 10.5% |
| Big word two | 1 word, 3 letters | caps | same | 900 | 9.7% H | 20 | - | rotated -90 | `#000000` | x 82.5 to 97%, y 91.5% up to 64% |
| Date | DD.MM.YYYY | digits | geometric sans, light (Montserrat Light / Avenir Next) | 400 | 3% H (4.5% W) | 600 | - | rotated -90 | `#ef2b47` | x 24.5 to 29%, y 92.5% up to 49% |
| Time | HH:MM | digits | same | 400 | 3% H | 600 | - | rotated -90 | `#ef2b47` | x 34 to 38.5%, y 92.5% up to 73% |
| Paragraph | 5 lines, 2 to 6 words each | caps | squarish techno sans (Exo 2, Saira / Avenir Next Condensed) | 600 | 1% H | 80 | 106% | left | `#2f2f2f` | 41%, top 85.5% |
| Side note | 5 lines, 4 to 6 words each | caps | same | 600 | 1.5% H | 60 | 108% | rotated -90, left | `#2f2f2f` | x 81 to 97.5%, y 60% up to 23.5% |

The big words are black and very heavy; the date is the same family at its lightest and in
color. That weight contrast (900 against 400) lets the date overlap the picture without
competing with the words.

## 6. Imagery

One photograph of a reaching hand with a forearm, cut out and turned so it rises from the
bottom left corner; the fingers open toward the top right. It is converted to a coarse dot
screen (**T-halftone-portrait**: `adjust bw`, `adjust levels --in-black 20 --in-white 215`,
`filter halftone --cell 17 --angle 45` with the ground color as paper), then given a thin
off-white contour (`fx stroke`, the die-cut look of **T-cutout-drop-shadow** without the
shadow). Because the paper color equals the ground, only the dots and the contour separate
the hand from the sheet. Other subjects that suit it: an arm throwing something, a dancer
in a long diagonal pose, a bird in flight, a tilted bust. The subject needs a direction; a
frontal, upright cutout makes the layout static.

## 7. Effects and textures

* Color cloud (no library technique): `select path` with `--feather 22`, a top-to-bottom
  `fill gradient` through `--mask @sel` on an empty layer, then `layer blend --mode
  dissolve`. Dissolve turns the feathered edge into grain instead of a soft fade, which is
  what makes it read as spray.
* Disc: one `shape path` circle written with two arcs so its center can sit 111 px from the
  edge with part of it off-canvas.
* **T-halftone-portrait**, cell 17 px on a 1080 px canvas (coarser than a portrait would
  take: this is a large, simple shape).
* Contour: `fx stroke --width 9 --color '#f4f4f4' --position outside`.
* **T-vertical-type** for both words, date, time, and the side note.
* No shadows and no blur anywhere: the sheet stays flat like a risograph print.

## 8. Hierarchy and eye path

1. The color cloud: the only saturated area, top right.
2. Down the diagonal of the hand to the disc at the left edge.
3. Up big word one, which passes through the disc; then across to big word two.
4. Date and time (red, beside word one), then the paragraph, then the side note.

Cap-height ratio: big words 9.7 : date 3 : side note 1.5 : paragraph 1. Color does the first
half of the work (cloud, disc, date are the only colored things and form a triangle), size
does the second.

## 9. Build recipe

`subject.png` is your image here: a cutout with transparency. The recipe rotates it 38
degrees clockwise to give an upright stand-in the lower-left to upper-right direction; a
photo that already runs diagonally needs no rotation.

```bash
pixelox new -o art.pxo --size 1080x1620 --fill '#e4e4e4'
pixelox shape path art.pxo --in-place --name Disc --d 'M -111 867 A 219 219 0 1 1 327 867 A 219 219 0 1 1 -111 867 Z' --fill '#ef2b47'
pixelox select path art.pxo --in-place --d 'M 800 -60 C 730 120 680 260 668 350 L 690 440 L 612 558 C 720 590 800 640 900 580 C 980 540 1060 510 1140 500 L 1140 -60 Z' --feather 22
pixelox layer add art.pxo --in-place --name Spray
pixelox fill gradient art.pxo --in-place --layer Spray --from 0,0 --to 0,39% --stops '#da2073@0,#da2073@0.3,#f0503a@0.45,#fa7a14@0.58,#fa8a14@0.8,#ffd83a@0.95' --mask @sel
pixelox select none art.pxo --in-place
pixelox layer blend art.pxo --in-place --layer Spray --mode dissolve
pixelox comp place art.pxo subject.png --in-place --as Subject --size x980 --at 33%,60% --anchor c
pixelox adjust bw art.pxo --in-place --layer Subject
pixelox adjust levels art.pxo --in-place --layer Subject --in-black 20 --in-white 215
pixelox filter halftone art.pxo --in-place --layer Subject --cell 17 --angle 45 --ink '#2b2b2b' --paper '#e4e4e4'
pixelox fx stroke art.pxo --in-place --layer Subject --width 9 --color '#f4f4f4' --position outside
pixelox transform rotate art.pxo --in-place --layer Subject --degrees 38
pixelox transform move art.pxo --in-place --layer Subject --by 30,-150
pixelox text add art.pxo --in-place --name Word1 --content "MAKERS" --font "Avenir Next" --weight 900 --size 13.4% --tracking 20 --color '#000000' --at 50%,50% --anchor c
pixelox transform rotate art.pxo --in-place --layer Word1 --degrees -90
pixelox transform move art.pxo --in-place --layer Word1 --to 6%,73% --anchor bl
pixelox text add art.pxo --in-place --name Word2 --content "LAB" --font "Avenir Next" --weight 900 --size 13.4% --tracking 20 --color '#000000' --at 50%,50% --anchor c
pixelox transform rotate art.pxo --in-place --layer Word2 --degrees -90
pixelox transform move art.pxo --in-place --layer Word2 --to 82.5%,91.5% --anchor bl
pixelox text add art.pxo --in-place --name Date --content "12.09.2031" --font "Avenir Next" --weight 400 --size 4% --tracking 600 --color '#ef2b47' --at 50%,50% --anchor c
pixelox transform rotate art.pxo --in-place --layer Date --degrees -90
pixelox transform move art.pxo --in-place --layer Date --to 24.5%,92.5% --anchor bl
pixelox text add art.pxo --in-place --name Time --content "18:00" --font "Avenir Next" --weight 400 --size 4% --tracking 600 --color '#ef2b47' --at 50%,50% --anchor c
pixelox transform rotate art.pxo --in-place --layer Time --degrees -90
pixelox transform move art.pxo --in-place --layer Time --to 34%,92.5% --anchor bl
pixelox text add art.pxo --in-place --name Body --content $'AN OPEN STUDIO FOR PEOPLE WHO\nBUILD THINGS BY HAND: PRINT,\nCLAY, WOOD, AND CODE.\nBRING A PROJECT OR START ONE.\nALL TOOLS PROVIDED.' --font "Avenir Next Condensed" --weight 600 --size 1.35% --leading 106% --tracking 80 --color '#2f2f2f' --at 41%,85.5% --anchor tl
pixelox text add art.pxo --in-place --name Side --content $'SATURDAY WORKSHOPS RUN ALL DAY,\nSIX TABLES, TWELVE SEATS EACH,\nNO BOOKING NEEDED BEFORE NOON.\nCOFFEE AND A SHARED LUNCH\nARE ON THE HOUSE.' --font "Avenir Next Condensed" --weight 600 --size 2% --leading 108% --tracking 60 --color '#2f2f2f' --at 50%,50% --anchor c
pixelox transform rotate art.pxo --in-place --layer Side --degrees -90
pixelox transform move art.pxo --in-place --layer Side --to 81%,60% --anchor bl
pixelox export art.pxo -o art.png
```

Change first: the two words (word one 57 to 62% H long, word two under 28% H so it stays
below the side note), then the cutout and its angle, then the cloud's four colors. The
cloud path is in canvas pixels: its point (`612 558`) should touch the tip of the subject,
so move the subject to it, not the reverse.

## 10. Variations and pairings

* Dark twin: p094 keeps this grid on black with white words, soft red and orange glows in
  place of the disc and cloud, and the cutout centered in a glowing oval.
* Other color runs for the cloud, top to bottom: `#2b3cff`, `#6a5cff`, `#00b7ff`, `#7ff0e0`
  with a disc in `#2b3cff`; or `#0b7a4b`, `#58b947`, `#d7e84a` with a disc in `#ff5a36`.
* Two cutouts instead of one (two hands reaching toward each other from opposite corners);
  put the cloud between them.
* Square post: drop the side note and the time, keep word one on the left edge and word two
  along the bottom, unrotated.
* Suits open studios, design fairs, workshops, zine and print fairs, student shows. Pairs
  with T-paper-grain (amount 20, overlay 40%) for a printed feel and with T-torn-collage
  when the cutout should look torn, not cut.

## 11. Pitfalls

* A smooth, blurred cloud. Without `dissolve` the feathered gradient looks like a web
  banner; the grain at its edge is the point. Keep the feather between 16 and 30 px: less
  gives a hard edge, more gives loose confetti.
* Halftone paper in white on a gray ground: the cutout becomes a pale box-like patch. Set
  `--paper` to the exact ground color and let the off-white contour do the separating.
* An upright, centered subject. The design needs the diagonal; rotate the cutout or choose
  a photo with a clear direction, and let it leave the canvas at a corner.
* The red date running over the darkest part of the cutout: thin red digits vanish on
  dense black dots. Keep the date track over light areas, or lighten the subject where the
  digits cross it (raise `--in-black` less, or move the cutout up).
* Color anywhere else (a colored word, a second disc). The triangle of cloud, disc, and
  date is complete; a fourth colored element flattens the hierarchy.
