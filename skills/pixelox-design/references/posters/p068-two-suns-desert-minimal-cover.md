---
id: p068
title: Two-suns desert minimalist cover
genre: [book-cover, film-poster]
mood: [calm, minimal, retro, warm]
style: [minimal, flat, retro]
format: portrait-a
archetype: A-centered-title
palette: ['#f1d292', '#f4cc86', '#f4b663', '#342818']
techniques: [T-geo-shapes, T-fit-width, T-grunge, T-vignette, T-paper-grain]
needs: nothing
---

# p068: Two-suns desert minimalist cover

## 1. Concept

A minimalist paperback cover for a desert science-fiction epic, drawn with almost nothing:
sand-colored paper, two flat orange discs for suns, and a hair-thin caravan of about
forty-five tiny black figures walking down a long sand ridge, one of them fallen out of the row.
Author and title sit at the top in heavy black serif capitals fitted to one width. It
works through scale: the figures are 1.6% of the sheet's height and the suns 30%, so the
emptiness between them becomes the desert, and the single fallen figure is the entire
plot.

## 2. Format and grid

* Portrait A-series (1 : 1.43). Margins 8 to 8.6% W left and right; everything typographic
  is fitted to the same measure, 8 to 92% W.
* Top block (justified to the measure): author 4.4 to 10.4% H; title 12 to 25% H; tagline
  one tiny line at 27.2% H.
* Picture zone 45 to 83% H: small sun centered at 68.6% W, 54% H; large sun centered at
  37% W, 69% H; ridge line falling from 76.6% H at the left edge to 82% H at the right; the
  caravan stands on that line from 5% W to 94% W.
* Empty bands: 29 to 45% H (between tagline and suns) and 84 to 100% H (below the ridge).
  The lower band is slightly lighter than the sky.

## 3. Layer stack

1. Ground: aged sand paper, `#f1d292` at the top grading to `#f4cc86` at the bottom, with
   a soft brown mottle and darker edges (old paperback).
2. Small sun: flat disc `#efb56e`, 25.7% W across, centered at 68.6% W, 54% H.
3. Large sun: flat disc `#f4b663`, 43% W across, centered at 37% W, 69% H; its lower fifth
   is hidden behind the ridge.
4. Sand ridge: the area below a straight, slightly falling line (77.9% H at the left, 82.6% H at
   the right), filled a shade lighter than the sky (`#f6d596`), covering the bottom of the
   large sun.
5. Caravan: about 45 black figures in single file on the ridge line, each a vertical tick
   1.6% H tall with a dot for a head, spaced 2% W apart, slightly irregular; a few carry
   poles.
6. Lone figure: one figure lying or kneeling 2.5% H below the line at 48% W, with a short
   cast mark beside it.
7. Author: 2 words in regular-weight serif caps, 84% W wide, cap height 6% H.
8. Title: 4 letters in bold serif caps, 84% W wide, cap height 13% H.
9. Tagline: one line of tiny serif caps, the same 84% W wide, cap height 1.2% H.
10. Paper wear: faint specks, a few stains, fine grain over everything.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#f1d292` | sky, upper paper | 48% |
| `#f4cc86` | lower paper, sand ridge | 30% |
| `#f4b663` | suns | 15% |
| `#342818` | type, figures | 7% |

Three sands and one dark brown. The suns are only about 10% darker and more saturated
than the sky: they are meant to be felt before they are seen.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Author | `[AUTHOR NAME]`, 2 words, 12 to 13 letters | caps | wide transitional serif (Libre Baskerville, Gelasio / Georgia) | 400 | 6% | 20 | - | justified to 84% W | `#342818` | 8 to 92% W, top 4.4% |
| Title | `[BOOK TITLE]`, 1 word, 4 letters | caps | same, bold | 700 | 13% | 0 | - | justified to 84% W | `#342818` | 8 to 92% W, top 12% |
| Tagline | one sentence, 9 to 11 words | caps | same | 400 | 1.2% | 40 | - | justified to 84% W | `#342818` | 8 to 92% W, 27.2% |

One serif family, three sizes, one width. The title is 2.2 times the author's cap height
and 11 times the tagline's; all three lines share both edges, which makes a solid slab of
type above a nearly empty picture.

## 6. Imagery

No photograph. The picture is two discs, one line, and a row of marks. What sells it:
the discs overlap nothing (the small one floats clear, upper right of the large one); the
ridge line is straight, not wavy; the caravan is perfectly thin and long, its figures close
enough to read as a line from a distance and as people up close. In a build, the figures
are a dashed stroke (bodies) plus a dotted stroke of round caps (heads). The idea extends
to any "tiny life against huge nature" subject: a ship and a moon, a climber and a peak, a
bird and a storm cloud.

## 7. Effects and textures

* **T-geo-shapes**: two `shape ellipse` discs and one `shape path` ridge.
* Caravan: `shape line` with `--stroke-width 24 --dash 7,14` (ticks 7 px wide every 21 px)
  and, 16 px higher, a second line with `--stroke-width 10 --dash 0,21 --cap round` (the
  heads). No library ID.
* **T-fit-width**: each of the three text lines is scaled to 907 px (84% W).
* **T-grunge**, light: mottle in multiply at 35%, dirt at 20% (aged paper).
* **T-vignette** in brown `#8a5a20`, alpha 0 to 45%.
* **T-paper-grain**, amount 14, overlay 35%.

## 8. Hierarchy and eye path

1. The title (the only large dark thing).
2. Down across the empty band to the large sun, up to the small one.
3. Along the caravan from left to right; the gap in its rhythm leads to the lone figure.
4. Author and tagline are read with the title on the way.

## 9. Build recipe

Needs no images.

```bash
pixelox render js scripts/clouds.js -o mottle.png --size 1080x1528 --seed 21 --param lo='#b9905a' --param hi='#ffffff' --param scale=3 --param octaves=6 --param warp=0.4
pixelox render js scripts/grunge.js -o dirt.png --size 1080x1528 --seed 9 --param color='#6b4a1c' --param specks=4000 --param blotches=20 --param scratches=15 --param clump=0.7
pixelox new -o desert.pxo --size 1080x1528 --fill '#f1d292'
pixelox layer add desert.pxo --in-place --name Warmth
pixelox fill gradient desert.pxo --in-place --layer Warmth --from 0,30% --to 0,100% --stops '#f4c57a00@0,#f4c57aff@1'
pixelox shape ellipse desert.pxo --in-place --name SmallSun --rect 602,686,278x278 --fill '#efb56e'
pixelox shape ellipse desert.pxo --in-place --name LargeSun --rect 170,823,463x463 --fill '#f4b663'
pixelox shape path desert.pxo --in-place --name Ridge --d 'M 0 1190 L 1080 1262 L 1080 1528 L 0 1528 Z' --fill '#f6d596'
pixelox shape line desert.pxo --in-place --name Bodies --from 56,1178 --to 1018,1246 --stroke '#342818' --stroke-width 24 --dash 7,14
pixelox shape line desert.pxo --in-place --name Heads --from 59.5,1162 --to 1021.5,1230 --stroke '#342818' --stroke-width 10 --dash 0,21 --cap round
pixelox shape path desert.pxo --in-place --name Lone --d 'M 508 1276 L 536 1268 L 538 1276 L 510 1284 Z M 540 1262 a 6 6 0 1 0 12 0 a 6 6 0 1 0 -12 0 Z' --fill '#342818'
pixelox comp place desert.pxo mottle.png --in-place --as Mottle --fit cover
pixelox layer blend desert.pxo --in-place --layer Mottle --mode multiply
pixelox layer opacity desert.pxo --in-place --layer Mottle --value 35
pixelox comp place desert.pxo dirt.png --in-place --as Dirt --fit cover
pixelox layer opacity desert.pxo --in-place --layer Dirt --value 20
pixelox layer add desert.pxo --in-place --name Vignette
pixelox fill gradient desert.pxo --in-place --layer Vignette --kind radial --from 50%,50% --to 50%,125% --stops '#8a5a2000@0.5,#8a5a2073@1'
pixelox text add desert.pxo --in-place --name Author --content "AUTHOR SURNAME" --font Georgia --weight 400 --size 8% --tracking 20 --color '#342818'
pixelox transform scale desert.pxo --in-place --layer Author --size 907x
pixelox transform move desert.pxo --in-place --layer Author --to 50%,4.4% --anchor t
pixelox text add desert.pxo --in-place --name Title --content "DUSK" --font Georgia --weight 700 --size 18% --color '#342818'
pixelox transform scale desert.pxo --in-place --layer Title --size 907x
pixelox transform move desert.pxo --in-place --layer Title --to 50%,11.6% --anchor t
pixelox text add desert.pxo --in-place --name Tagline --content "TWO SUNS, ONE ROAD, AND NO WATER FOR A THOUSAND MILES" --font Georgia --weight 400 --size 1.7% --tracking 40 --color '#342818'
pixelox transform scale desert.pxo --in-place --layer Tagline --size 907x
pixelox transform move desert.pxo --in-place --layer Tagline --to 50%,28.6% --anchor c
pixelox layer add desert.pxo --in-place --name Grain --fill '#808080'
pixelox filter add-noise desert.pxo --in-place --layer Grain --amount 14 --monochrome
pixelox layer blend desert.pxo --in-place --layer Grain --mode overlay
pixelox layer opacity desert.pxo --in-place --layer Grain --value 35
pixelox export desert.pxo -o desert.png
```

Change first: the title (4 to 5 letters; with more, the cap height drops and the author
line should shrink with it), the two sun positions, then the paper color. If the title's
fitted height changes, move the tagline to sit 2% H under its baseline.

## 10. Variations and pairings

* Other worlds, same recipe: night (ground `#1d2742`, moons `#c9d2e0` and `#8fa0c2`,
  figures and type cream); ice (ground `#e4eef2`, suns `#b9d3e0`, type navy); red planet
  (ground `#e8b79a`, suns `#d9744a`, type `#3a1410`).
* Swap the caravan for a train, a fence line, a flock, a row of pylons, a queue of cars:
  any long thin repetition on a single line.
* Film poster: title at 86 to 96% H under the picture, credits in the tagline's size.
* Square: suns and caravan fill 30 to 90% H, title in the top quarter.
* Pairs with T-sunburst (very faint rays from the large sun), T-frame-border (a paper
  margin), and T-distress-type on the title.

## 11. Pitfalls

* Suns in a strong orange or red: the cover turns into a flag. Keep them within 10 to 15%
  of the ground's lightness.
* Figures too big (over 2.5% H) or too few: it becomes "some people" rather than a
  caravan lost in a landscape. 40 or more, each under 2% H.
* A curved, hilly ridge line: it reads as cartoon hills. One straight edge with a slope of
  about 4 degrees.
* Title, author, and tagline at three different widths: the top block loses its slab
  shape. Fit all three to the same measure.
* Clean flat digital fill with no paper texture: the design looks unfinished rather than
  minimal. Keep mottle, vignette, and grain, all subtle.
