---
id: p013
title: Cracked profile silhouette thriller poster
genre: [film-poster]
mood: [mysterious, dramatic, gritty, warm]
style: [minimal, flat, grunge]
format: portrait-2x3
archetype: A-centered-title
palette: ['#3b3a3a', '#e29650', '#b27847', '#e0deda']
techniques: [T-silhouette, T-grunge, T-distress-type, T-vignette, T-paper-grain, T-crack]
needs: subject-cutout
---

# p013: Cracked profile silhouette thriller poster

## 1. Concept

A psychological-thriller one-sheet built from two flat inks: a charcoal head-and-shoulders
silhouette in profile on a mottled orange ground, broken open from the crown to the chest by a
lightning-shaped crack through which the orange shows, with a dozen tiny human figures
climbing inside the crack. It works because the whole plot idea (one man, many people inside
him) is told by one shape with one wound in it; the type is small and stays out of the way,
and the title sits on the dark chest like a name plate.

## 2. Format and grid

* Portrait 2:3 (close to the one-sheet ratio 27 x 40). One vertical axis at 50% W; every
  text line is centered on it.
* Margins: 10% W left and right for the top credit lines; the silhouette itself runs off
  the bottom edge and nearly touches both sides at the shoulders (16% W and 86% W).
* Zones: credits 3.5 to 8% H; silhouette 13 to 100% H (head 13 to 49% H, neck 49 to 57% H,
  shoulders and chest 57 to 100% H); crack 14 to 74% H; text on the chest 77 to 97% H.
* The crack is the only diagonal thing on the sheet: about 8% W wide at the crown, narrowing
  in 5 or 6 zigzags to a point at 65% W, 74% H.

## 3. Layer stack

1. Ground: orange `#e29650`, full canvas.
2. Silhouette: a man's head and shoulders in strict left-facing profile, flat charcoal
   `#3b3a3a`, slightly faceted (straight cuts instead of soft curves on the skull and jaw).
   Crown at 13% H, nose tip at 20% W, 38% H, chin at 48% H, back of the head at 76% W,
   shoulders spanning 16 to 86% W and leaving the canvas at the bottom.
3. Crack: a zigzag polygon in the ground orange laid over the silhouette, from the crown
   (48 to 57% W at 14% H) down to a point at 65% W, 74% H. Its edges are straight segments
   25 to 110 px long with sharp reversals, like torn card or a lightning bolt.
4. Tiny figures: 12 charcoal silhouettes of climbing people, each about 3% H tall, placed
   on the crack's ledges and corners so their bodies show against the orange; two of them
   stand on top of the head and reach for each other across the gap.
5. Paper mottle over everything: soft cloudy darkening toward `#b27847`, multiply.
6. Stains and specks: sparse brown dirt, 35%.
7. Edge shade: a brown vignette, strongest at the bottom corners.
8. Credit line 1 (7 words, light, wide tracking) at 4.6% H; credit line 2 (a name, bold,
   very wide tracking) at 6.9% H; both off-white on the orange.
9. Tagline: 4 short centered lines in small gray-white caps on the chest, 78 to 83% H.
10. Title: one 5-letter word in heavy wide capitals, off-white with horizontal scuffs and
    missing ink, 28.5 to 72% W, 85.6 to 92.9% H.
11. Date line: bold off-white caps at 96.2% H.
12. Two tiny marks (a rating box, a studio logo) in the bottom corners at 96% H, 5% W wide.
13. Fine grain over all.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#3b3a3a` | silhouette, tiny figures | 42% |
| `#e29650` | ground, crack | 40% |
| `#b27847` | mottle and edge shade of the ground | 15% |
| `#e0deda` | all type | 3% |

Two inks plus an off-white for type. The charcoal is a warm dark gray, not black, so the
orange and the gray sit at similar strength and neither one vibrates.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Credit line | 6 to 7 words introducing the filmmaker | caps | geometric sans (Montserrat / Avenir Next) | 500 | 0.95% | 350 | - | center | `#f3ece2` | 50%, 4.6% |
| Filmmaker | `[DIRECTOR NAME]`, 3 parts, 15 letters | caps | same | 700 | 1.45% | 600 | - | center | `#f3ece2` | 50%, 6.9% |
| Tagline | 4 lines of 2 to 3 words, a fact then a threat | caps | same | 600 | 0.9% | 60 | 170% | center | `#d9d6d1` | 50%, 78 to 83% |
| Title | `[FILM TITLE]`, 1 word, 5 letters | caps | heavy wide grotesque (Archivo Black / Arial Black) | 900 | 7.3% | 0 | - | center | `#e0deda` | 50%, top 85.6% |
| Date | month + day | caps | geometric sans (Montserrat / Avenir Next) | 700 | 1.1% | 60 | - | center | `#f3ece2` | 50%, 96.2% |

One sans family for the small lines and one heavy wide face for the title. The top two
lines are set to the same width (80% W) by tracking, not by size.

## 6. Imagery

No photograph is visible: the picture is a silhouette. Start from a profile cutout of a
head and shoulders (bald or short hair, jacket collar, clear nose and chin) and flatten it
with **T-silhouette** (`fx overlay` with one color). A frontal bust fails here; the nose,
lips, and chin are what make the shape a person. The original's outline is cut with a few
straight facets, as if made from paper. The tiny climbers are drawn silhouettes (your own
illustration, or simple figures built from a circle and a tapered body); they are 1/30 of
the head's height, which is what makes the head feel like a cliff.

## 7. Effects and textures

* **T-silhouette**: flat fill `#3b3a3a`, no gradient.
* Crack: one `shape path` polygon in the ground color above the silhouette (no library ID;
  see the recipe). Figures: one compound `shape path`.
* **T-grunge**: mottle (`clouds.js`, `lo='#b9834f'`, `hi='#ffffff'`, multiply 60%) and dirt
  at 35%, both above the silhouette so shape and ground share one paper surface.
* **T-vignette** in brown (`#6b3d1a`), centered on the head.
* **T-distress-type** on the title: wear in the silhouette color clipped to the letters,
  with many scratches so the damage runs in horizontal bands.
* **T-paper-grain**, amount 14, soft-light 40%.
* **T-crack** in [techniques.md](../techniques.md) is the tested library version of the jagged crack (`scripts/crack.js` draws one from two points); start from it when reusing this idea.

## 8. Hierarchy and eye path

1. The silhouette (largest shape, strongest contrast), and inside it the bright crack.
2. Down the crack, discovering the tiny figures, to its tip, which points at the tagline.
3. The title, then the date.
4. The credit lines at the top are read last.

Cap heights: title 7.3 : filmmaker 1.45 : date 1.1 : credit and tagline 0.9, about
8 : 1.6 : 1.2 : 1. The title is small for a film poster; the image is the headline.

## 9. Build recipe

`subject.png` is your image here: a profile cutout of a head and shoulders facing left
(the stand-in is frontal, so the build shows the layout, not the likeness). Redraw the
crack path to fit your head: start with two points on the crown, end with one point low on
the chest.

```bash
pixelox render js scripts/clouds.js -o mottle.png --size 1080x1620 --seed 21 --param lo='#b9834f' --param hi='#ffffff' --param scale=3 --param octaves=6 --param warp=0.5
pixelox render js scripts/grunge.js -o dirt.png --size 1080x1620 --seed 9 --param color='#7a4a22' --param specks=5000 --param blotches=70 --param scratches=20
pixelox render js scripts/grunge.js -o wear.png --size 1080x1620 --seed 14 --param color='#3b3a3a' --param specks=7000 --param blotches=120 --param scratches=260 --param clump=0.3
pixelox new -o thriller.pxo --size 1080x1620 --fill '#e29650'
pixelox comp place thriller.pxo subject.png --in-place --as Figure --size x1560 --at 50%,100% --anchor b
pixelox fx overlay thriller.pxo --in-place --layer Figure --color '#3b3a3a'
pixelox shape path thriller.pxo --in-place --name Crack --d 'M 520 205 L 466 308 L 533 343 L 459 433 L 540 518 L 560 616 L 547 734 L 608 839 L 594 944 L 634 1049 L 702 1199 L 655 1035 L 634 944 L 648 839 L 594 734 L 608 616 L 594 524 L 533 433 L 614 374 L 580 295 L 614 205 Z' --fill '#e29650'
pixelox shape path thriller.pxo --in-place --name Climbers --d 'M 461 180 a 7 7 0 1 0 14 0 a 7 7 0 1 0 -14 0 Z M 461 189 l 14 0 l 7 34 l -6 0 l -4 -16 l -5 16 l -6 0 Z M 645 182 a 7 7 0 1 0 14 0 a 7 7 0 1 0 -14 0 Z M 645 191 l 14 0 l 1 34 l -6 0 l -4 -16 l -5 16 l -6 0 Z M 533 300 a 7 7 0 1 0 14 0 a 7 7 0 1 0 -14 0 Z M 533 309 l 14 0 l 7 34 l -6 0 l -4 -16 l -5 16 l -6 0 Z M 503 392 a 7 7 0 1 0 14 0 a 7 7 0 1 0 -14 0 Z M 503 401 l 14 0 l 1 34 l -6 0 l -4 -16 l -5 16 l -6 0 Z M 568 540 a 7 7 0 1 0 14 0 a 7 7 0 1 0 -14 0 Z M 568 549 l 14 0 l 7 34 l -6 0 l -4 -16 l -5 16 l -6 0 Z M 567 680 a 7 7 0 1 0 14 0 a 7 7 0 1 0 -14 0 Z M 567 689 l 14 0 l 1 34 l -6 0 l -4 -16 l -5 16 l -6 0 Z M 615 860 a 7 7 0 1 0 14 0 a 7 7 0 1 0 -14 0 Z M 615 869 l 14 0 l 7 34 l -6 0 l -4 -16 l -5 16 l -6 0 Z M 629 990 a 7 7 0 1 0 14 0 a 7 7 0 1 0 -14 0 Z M 629 999 l 14 0 l 1 34 l -6 0 l -4 -16 l -5 16 l -6 0 Z' --fill '#3b3a3a'
pixelox comp place thriller.pxo mottle.png --in-place --as Mottle --fit cover
pixelox layer blend thriller.pxo --in-place --layer Mottle --mode multiply
pixelox layer opacity thriller.pxo --in-place --layer Mottle --value 60
pixelox comp place thriller.pxo dirt.png --in-place --as Dirt --fit cover
pixelox layer opacity thriller.pxo --in-place --layer Dirt --value 28
pixelox layer add thriller.pxo --in-place --name Vignette
pixelox fill gradient thriller.pxo --in-place --layer Vignette --kind radial --from 50%,35% --to 50%,125% --stops '#6b3d1a00@0.45,#6b3d1a99@1'
pixelox text add thriller.pxo --in-place --name Credit --content "A NEW THRILLER FROM THE WRITER AND DIRECTOR" --font "Avenir Next" --weight 500 --size 1.3% --tracking 350 --color '#f3ece2' --at 50%,4.6% --anchor c
pixelox transform scale thriller.pxo --in-place --layer Credit --size 800x
pixelox text add thriller.pxo --in-place --name Filmmaker --content "DIRECTOR NAME" --font "Avenir Next" --weight 700 --size 2% --tracking 950 --color '#f3ece2' --at 50%,6.9% --anchor c
pixelox transform scale thriller.pxo --in-place --layer Filmmaker --size 860x
pixelox text add thriller.pxo --in-place --name Tagline --content $'ONE MAN HAS\nA HOUSE FULL OF VOICES.\nTHE LAST ONE\nWANTS OUT.' --font "Avenir Next" --weight 600 --size 1.25% --tracking 60 --leading 170% --align center --color '#d9d6d1' --at 50%,80.5% --anchor c
pixelox text add thriller.pxo --in-place --name Title --content "FAULT" --font "Arial Black" --size 10% --color '#e0deda'
pixelox transform scale thriller.pxo --in-place --layer Title --size 470x
pixelox transform move thriller.pxo --in-place --layer Title --to 50%,85.6% --anchor t
pixelox comp place thriller.pxo wear.png --in-place --as Wear --fit cover
pixelox layer clip thriller.pxo --in-place --layer Wear
pixelox text add thriller.pxo --in-place --name Date --content "OCTOBER 13" --font "Avenir Next" --weight 700 --size 1.55% --tracking 60 --color '#f3ece2' --at 50%,96.2% --anchor c
pixelox layer add thriller.pxo --in-place --name Grain --fill '#808080'
pixelox filter add-noise thriller.pxo --in-place --layer Grain --amount 14 --monochrome
pixelox layer blend thriller.pxo --in-place --layer Grain --mode soft-light
pixelox layer opacity thriller.pxo --in-place --layer Grain --value 40
pixelox export thriller.pxo -o thriller.png
```

Change first: the cutout (a true profile), then the crack path, then the title word (4 to
6 letters keeps the width near 44% W). Move each climber so it touches an edge of the crack.

## 10. Variations and pairings

* Other wounds, same logic: a keyhole, a staircase, a river, a skyline, or a question mark
  cut out of the head in the ground color; a second, smaller profile inside the first.
* Palette swaps: charcoal on bone `#e9e2d0` with a blood-red crack `#b3121a`; navy
  `#14213d` on mustard `#e9b44c`; white silhouette on black with a red crack for horror.
* Book cover: move the title into the top band at cap height 9% H and the author to the
  chest; drop the credit lines.
* Square social teaser: crop to the head (13 to 60% H), keep only the title under the chin.
* Pairs with T-double-exposure (fill the silhouette with a scene instead of flat gray) and
  T-splatter (let the crack end in ink instead of a point).

## 11. Pitfalls

* A frontal or three-quarter head: the silhouette becomes a bowling pin. Use a strict
  profile, and check the outline alone before adding anything.
* A soft, wavy crack reads as a river or a vein. Use straight segments and sharp angles,
  wide at the top and closing to a point.
* Crack filled with a flat color different from the textured ground: it looks pasted on.
  Use the exact ground color and put the mottle and dirt layers above everything.
* Figures too big or too many sizes: the head stops feeling huge. Keep every figure at 2.5
  to 3.5% H and the same color as the silhouette.
* Pure black silhouette and pure white title: the poster turns into a road sign. Keep the
  warm gray `#3b3a3a` and the off-white `#e0deda`.
