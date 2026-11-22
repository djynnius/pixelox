---
id: p049
title: Red queue of doubles film poster
genre: [film-poster]
mood: [intense, mysterious, bold, dramatic]
style: [duotone, photo-composite, minimal]
format: portrait-a
archetype: A-diagonal-repeat
palette: ['#b2110f', '#090706', '#230c0b', '#711312', '#ecefc1']
techniques: [T-duotone, T-mono-on-color, T-paper-grain, T-floor-type]
needs: subject-cutout
---

# p049: Red queue of doubles film poster

## 1. Concept

A thriller poster about a man and his double: the same bearded man in a suit is repeated
about sixteen times, standing in a queue along a painted floor line that recedes from the
bottom-left corner to a vanishing point at the right edge, all of them printed black on a
flat blood red. Only one, the third in line, turns to face us. The title, the cast, and
the tagline are painted on the "floor" in the same perspective, running along the line. It
works because repetition plus one exception is the story in one glance, because two inks
make sixteen photographs into one graphic, and because the floor-painted type gives a flat
red field real depth.

## 2. Format and grid

* Portrait, close to A-series (1 : 1.44). No margins; the figures are cropped by the left
  and top edges.
* One construction: a vanishing point just outside the right edge at 103% W, 29.5% H.
  The floor line runs to it from 6% W, 71.5% H (a slope of about 31 degrees). Every
  figure's feet stand on that line; every figure's height is 1.14 times its horizontal
  distance from the vanishing point.
* Figure sizes, near to far: the first is taller than the sheet and cut off at the chest
  by the top edge and by the left edge; the second is 63% H tall; the third (the hero,
  facing us) is 49% H at 42% W; the fourth 35% H; then a dense fan shrinking to 6% H at
  the right edge. Each step is 0.78 times the size of the one before.
* Type zone: a parallelogram below the line, 21 to 85% W, 50 to 86% H, in the same
  perspective. Upper-right 15% holds the star's name, flat and upright, at 5.2% H.
* The bottom 13% and the upper-right corner are empty red.

## 3. Layer stack

1. Ground: flat red `#b2110f`, full canvas. No gradient, no vignette.
2. Floor line: a cream stripe `#ecefc1`, 1.4% H thick at the left end tapering to 0.4% H at
   the right edge, from 6% W, 71.5% H to 100% W, 30.5% H.
3. Queue: about 16 copies of one full-length photograph of a man in a dark suit, seen
   from behind or in profile, facing right along the line, each smaller than the last.
   Black `#090706` with dark red `#711312` in the lightest parts only (duotone on red).
   Near figures overlap far ones.
4. Hero: the third copy is a different photograph, the man turned toward the camera, hands
   in pockets, tie loosened. Same treatment but one step lighter: skin and shirt reach a
   dusty pink `#c66d55`, so he is the only figure with a readable face.
5. Star's name: cream condensed bold caps, upright, top right at 56.6 to 85.6% W, 5.2% H.
6. Floor type (all in floor perspective, baselines parallel to the line, letter tops
   pointing up-left toward the line), from the line downward:
   a 5-word credit line in black; a cast line of three names in cream; the title, 5 black
   heavy capitals, letters 17% H long at the near end and 9% H at the far end; the tagline
   in cream.
7. Billing block: 8 tiny lines of black credits and a row of small logos, sheared to lie
   on the floor at right angles to the title, 21.6 to 56% W, 73.5 to 86.5% H.
8. Light grain.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#b2110f` | ground | 59% |
| `#090706` | figures, title | 27% |
| `#230c0b` | figure mid-shadows, billing | 10% |
| `#711312` | lightest tone on the doubles | 3% |
| `#ecefc1` | line, cream type | 1% |

Red and black with a thread of cream. The cream is slightly green-yellow, which makes the
red look even hotter than white would.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Star | `[ACTOR NAME]`, 2 words, 14 letters | caps | condensed humanist sans (Barlow Semi Condensed 600 / Avenir Next Condensed) | 600 | 1.7% | 0 | - | center | `#ecefc1` | 71%, 5.2% |
| Credit line | "FROM THE DIRECTOR OF" + `[FILM TITLE]` | caps | geometric sans (Jost / Futura) | 500, last word 700 | 1.6% at the near end | 0 | - | floor perspective | `#090706` | starts 26.5% W, 71% H |
| Cast | 3 names with "AND" before the last | caps | same | 500 | 1.6% at the near end | 0 | - | floor perspective | `#ecefc1` | starts 31% W, 72% H; ends 71% W, 50% H |
| Title | `[FILM TITLE]`, 1 word, 5 letters | caps | heavy geometric sans (Jost 800, Outfit 800 / Futura bold) | 700 | 17% near end, 9% far end | -20 | - | floor perspective | `#090706` | 35 to 84% W, 51 to 77% H |
| Tagline | 5-word sentence with a full stop | caps | geometric sans (Jost / Futura) | 500 | 2.2% near end | 0 | - | floor perspective | `#ecefc1` | 60 to 85% W, 57 to 78% H |
| Billing | 8 lines of credits | caps | condensed sans (Oswald 300 / Avenir Next Condensed) | 500 | 0.5% | 0 | 130% | sheared | `#230c0b` | 21.6 to 56% W, 73.5 to 86.5% H |

One geometric family. What sets the title apart is weight and size, and the fact that it
alone is black among cream lines.

## 6. Imagery

Two full-length photographs of the same person in the same clothes: one seen from behind
or in profile (used about 15 times) and one facing the camera (used once). Cut out, turned
black and white, crushed (**T-mono-on-color**: `adjust levels --in-black 40 --in-white
200`), and mapped with **T-duotone** from black to the ground red, so mid-tones and
highlights sink into the background and only the dark suit shapes remain. The hero's map
ends in a pink lighter than the ground, which is the only reason he stands out.

Sizing rule for the copies: pick the vanishing point; for a copy whose feet are at
horizontal distance d from it, height = 1.14 d for a standing figure (use 0.9 d for a
head-and-shoulders cutout, as in the recipe). Each next copy stands 22 to 30% of d
further along the line.

## 7. Effects and textures

* **T-duotone** on the merged queue: `#060505` at 0, `#3a0908` at 0.5, `#b2110f` at 1.
  On the hero: `#060505`, `#7a1512` at 0.5, `#d98a70` at 1.
* Floor line: one tapered `shape path` quadrilateral.
* Floor perspective for type: set each line flat, then `transform perspective --quad`
  with four points: top-left and top-right on a line parallel to the floor line,
  bottom-right and bottom-left displaced toward the lower right, the near end (left)
  twice as tall as the far end.
* Billing block: `transform skew --x -35 --y 12`.
* `transform perspective` and `transform skew` turn the text into pixels (warning
  PXW2003): set the wording first.
* **T-paper-grain**: `filter grain --as-layer --amount 8`. Nothing else: the flatness
  matters.
* **T-floor-type** in [techniques.md](../techniques.md) is the tested library version of the type lying in perspective on the floor; start from it when reusing this idea.

## 8. Hierarchy and eye path

1. The giant dark figures at the left, then along the line of shrinking backs.
2. The one face that looks back (third figure): the exception.
3. Down from his feet to the black title on the floor, which is read up and to the right,
   in the same direction as the queue.
4. Tagline, cast, and last the star's name in the corner.

Sizes: nearest figure over 100% H, hero 49% H, title letters 17% H tapering to 9%, tagline
2.2%, star's name 1.7%.

## 9. Build recipe

`subject.png` is your image here. The numbers below suit a head-and-shoulders cutout
(height = 0.9 x distance to the vanishing point at 1113,452; each copy 30% closer to it).
With a full-length figure use `--size x` values of 1.14 x distance and steps of 22%. Ideally
the Hero layer is a second photo of the same person facing the camera.

```bash
pixelox new -o queue.pxo --size 1080x1528 --fill '#b2110f'
pixelox shape path queue.pxo --in-place --name FloorLine --d 'M 60 1084 L 1080 464 L 1080 471 L 70 1106 Z' --fill '#ecefc1'
pixelox comp place queue.pxo subject.png --in-place --as M9 --size x55 --at 1052,489 --anchor b
pixelox comp place queue.pxo subject.png --in-place --as M8 --size x78 --at 1026,505 --anchor b
pixelox comp place queue.pxo subject.png --in-place --as M7 --size x112 --at 989,528 --anchor b
pixelox comp place queue.pxo subject.png --in-place --as M6 --size x159 --at 936,561 --anchor b
pixelox comp place queue.pxo subject.png --in-place --as M5 --size x228 --at 860,607 --anchor b
pixelox comp place queue.pxo subject.png --in-place --as M4 --size x325 --at 752,674 --anchor b
pixelox comp place queue.pxo subject.png --in-place --as M2 --size x663 --at 376,904 --anchor b
pixelox comp place queue.pxo subject.png --in-place --as M1 --size x948 --at 60,1098 --anchor b
pixelox layer merge queue.pxo --in-place --layers M9,M8,M7,M6,M5,M4,M2,M1 --name Doubles
pixelox adjust bw queue.pxo --in-place --layer Doubles
pixelox adjust levels queue.pxo --in-place --layer Doubles --in-black 40 --in-white 200
pixelox adjust gradient-map queue.pxo --in-place --layer Doubles --stops '#060505@0,#3a0908@0.5,#b2110f@1'
pixelox comp place queue.pxo subject.png --in-place --as Hero --size x464 --at 597,769 --anchor b
pixelox adjust bw queue.pxo --in-place --layer Hero
pixelox adjust gradient-map queue.pxo --in-place --layer Hero --stops '#060505@0,#7a1512@0.5,#d98a70@1'
pixelox text add queue.pxo --in-place --name Star --content "LEAD ACTOR NAME" --font @condensed --weight 600 --size 2.3% --color '#ecefc1' --at 71%,5.2% --anchor c
pixelox text add queue.pxo --in-place --name Cast --content "FIRST COSTAR   SECOND COSTAR   AND   THIRD COSTAR" --font Futura --weight 500 --size 3% --color '#ecefc1'
pixelox transform perspective queue.pxo --in-place --layer Cast --quad '337,1099 765,766 781,775 364,1109'
pixelox text add queue.pxo --in-place --name Title --content "RIVAL" --font Futura --weight 700 --size 14% --tracking -20 --color '#090706'
pixelox transform perspective queue.pxo --in-place --layer Title --quad '381,1111 786,778 909,825 635,1171'
pixelox text add queue.pxo --in-place --name Tagline --content "YOU CANNOT OUTRUN YOUR OWN SHADOW." --font Futura --weight 500 --size 3% --color '#ecefc1'
pixelox transform perspective queue.pxo --in-place --layer Tagline --quad '653,1173 883,868 913,876 707,1184'
pixelox text add queue.pxo --in-place --name Billing --content $'A FILM BY DIRECTOR NAME   STUDIO NAME PRESENTS\nPRODUCED BY FIRST PRODUCER AND SECOND PRODUCER\nSCREENPLAY BY WRITER NAME   MUSIC BY COMPOSER NAME\nDIRECTOR OF PHOTOGRAPHY CAMERA NAME   EDITED BY EDITOR NAME' --font @condensed --weight 500 --size 0.95% --leading 130% --color '#230c0b' --at 22%,74% --anchor tl
pixelox transform skew queue.pxo --in-place --layer Billing --x -35 --y 12
pixelox filter grain queue.pxo --in-place --as-layer --name Grain --amount 8
pixelox export queue.pxo -o queue.png
```

Change first: the cutouts; then the title word (4 to 6 letters fits the quad); then the
ground color. To move the floor line, move the vanishing point and recompute every
`--at` on the new line.

## 10. Variations and pairings

* Same device, other subjects: a row of identical chairs with one overturned; a line of
  runners with one looking back; identical houses with one lit window; a product repeated
  with one in color (ad for "the original").
* Ground swaps: cobalt `#1438c8` with black figures and white line; mustard `#e3a217`
  with dark brown; black ground with dark gray figures and a single red hero.
* Team or staff poster: every copy a different person, same pose, same treatment.
* Landscape banner: vanishing point at the right edge, title flat and upright at the
  left, 20% H cap height.
* Pairs with T-long-shadow (cast floor shadows from each figure), T-halftone-portrait on
  the queue, and T-frame-border.

## 11. Pitfalls

* Copies in arbitrary sizes and positions: the queue reads as a collage. Compute every
  height from one vanishing point and stand every copy on the one line.
* Leaving photographic mid-tones in the doubles: sixteen faces compete with the hero. Crush
  them until only the hero has a face.
* Flat, upright title pasted over the floor: the depth collapses. Either map the type to
  the floor or keep it out of the floor zone altogether (as the star's name is).
* Far end of the floor type too small to read: keep the far cap height at 50% or more of
  the near one, and the tagline's near cap height at 2% H or more.
* Gradient or vignette on the red. The scene is graphic, not atmospheric; keep the ground
  one flat color.
