---
id: p038
title: Cracked giant vertical word cover
genre: [book-cover, film-poster]
mood: [dark, intense, gritty, bold]
style: [type-led, grunge, minimal]
format: portrait-2x3
archetype: A-type-block
palette: ['#0f1414', '#f2f0de', '#e3e0c7', '#3d3f3a']
techniques: [T-vertical-type, T-fit-width, T-distress-type, T-grunge, T-paper-grain, T-crack]
needs: nothing
---

# p038: Cracked giant vertical word cover

## 1. Concept

A mid-century thriller jacket made of one word: six cream capitals in an ultra-condensed
heavy grotesque, turned 90 degrees and blown up until the word runs off the top and the
bottom of a black sheet, then torn in two along its whole length by a thin jagged crack.
The author's name sits small in italic serif in the top-left corner. It works because the
title is treated as an object and the plot is done to it: the word about a broken mind is
itself broken, and the crop makes it too big to take in at once.

## 2. Format and grid

* Portrait, about 1 : 1.59 (a pocket-book jacket); built here at 2:3.
* Two columns: a black strip 0 to 25% W on the left that carries the small text and
  nothing else, and the word, whose letters stand between 25% W (their feet) and 94% W
  (their heads). The word is rotated clockwise, so it reads from the top down.
* The word is longer than the page: 106% H, with the first letter cut by the top edge and
  the last by the bottom edge. Each letter takes about 17% H of length.
* The crack runs top to bottom at 55 to 60% W, wandering 3% W either way; one side branch
  leaves it at 54% H and runs right to 91% W.
* Small text: left edge at 1.2% W, two lines at 8.6% and 11.4% H.

## 3. Layer stack

1. Ground: black with a green cast, `#0f1414`, full canvas.
2. Word: 6 capitals, cream `#f2f0de`, rotated 90 degrees clockwise. Cap height 69% W
   (25 to 94% W), length 106% H, letter gaps about 2.5% H. The face is so condensed that
   each letter is only 0.4 times as wide as it is tall; counters are thin slots.
3. Ink wear inside the letters: scattered specks, a few scratches, and soft gray-green
   smudges (`#e3e0c7`, darker patches to `#8a8b83`), as on an old offset print.
4. Crack: a black fissure 0.5 to 1.6% W wide with sharp changes of direction every 4 to 6%
   H, cutting through every letter; the side branch tapers from 0.8% W to a hairline.
5. Scuffs on the black: sparse cream specks and short scratches, about 20% strength,
   mostly in the left strip.
6. Author: cream bold italic serif, 2 words, at 1.2% W, 8.6% H.
7. Series or imprint line: smaller italic serif, 4 words, at 1.2% W, 11.4% H.
8. Grain.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#0f1414` | ground, crack | 52% |
| `#f2f0de` | word, small type | 39% |
| `#e3e0c7` | smudges inside the letters | 7% |
| `#3d3f3a` | scuffs on the black | 2% |

Two inks. The cream is yellowed, not white, and the black is cold: together they read as
aged paper and old film.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Word | `[BOOK TITLE]`, 1 word, 6 letters | caps | ultra-condensed heavy grotesque (Anton, League Gothic / Impact, stretched) | 400 (the face is heavy) | 69% of W (46% of H) | 30 | - | rotated 90 degrees clockwise | `#f2f0de` | centered at 59.5% W, 50% H |
| Author | `[AUTHOR NAME]`, 2 words | title | bold italic old-style serif (Libre Caslon Text Italic, Libre Baskerville Italic / Baskerville) | 600 italic | 2% | 0 | - | left | `#f2f0de` | 1.2%, 8.6% |
| Imprint line | article + 2-word series name + genre word | title | same | 600 italic | 1.3% | 20 | - | left | `#f2f0de` | 1.2%, 11.4% |

The contrast between the two faces is the whole typographic idea: a brutal sans at 46% H
against a polite italic at 2% H, a ratio of 23 : 1.

## 6. Imagery

None. The word is the image, and the crack is the only drawing. The crack is not a clean
lightning bolt: it is a tear, nearly vertical, with small jogs, and its width changes at
every jog. The side branch gives it a direction and stops it from looking like a ruled
line.

## 7. Effects and textures

* **T-vertical-type** with **T-fit-width**: the word is set flat and centered at 59.5% W,
  50% H with `--fit-width 1720` (its length) and `--scale-x 57.3%` (letters narrowed, so the
  cap height comes out at 745 px), then rotated 90 degrees. The text stays live.
* Crack: two filled `shape path` polygons in the ground color (main fissure, side branch).
  No library ID; the point lists are in the recipe.
* **T-distress-type**: grunge in the ground color clipped to the word (`specks=5000`,
  `blotches=25`, `scratches=60`, `clump=0.5`), 70%.
* Smudges: `fill noise --as-layer` (`--scale 20%`, stops `#8f907f` to white) in multiply at
  30%, clipped to the word.
* **T-grunge** (dirt only) in cream over the black at 20%.
* **T-paper-grain**: `filter grain --as-layer --amount 15`, last.
* **T-crack** in [techniques.md](../techniques.md) is the tested library version of the jagged crack (`scripts/crack.js` draws one from two points); start from it when reusing this idea.

## 8. Hierarchy and eye path

1. The word, entered at the top and read downward.
2. The crack, which the eye follows the same way and which leads to the branch.
3. The author's name, top left; then the imprint line.

Size ratio word : author : imprint = 46 : 2 : 1.3 (cap heights in % H).

## 9. Build recipe

Needs no images. The word should have 5 to 7 letters; with more, lower `--fit-width`
(`1720`, the word's length) no further than 1620 and let the cap height shrink instead
(or lower `--scale-x` to keep it).

```bash
pixelox render js scripts/grunge.js -o wear.png --size 1080x1620 --seed 14 --param color='#0f1414' --param specks=5000 --param blotches=25 --param scratches=60 --param clump=0.5
pixelox render js scripts/grunge.js -o scuffs.png --size 1080x1620 --seed 4 --param color='#f2f0de' --param specks=2500 --param blotches=25 --param scratches=30 --param clump=0.7
pixelox new -o jacket.pxo --size 1080x1620 --fill '#0f1414'
pixelox text add jacket.pxo --in-place --name Word --content "TREMOR" --font @condensed-heavy --scale-x 57.3% --fit-width 1720 --tracking 30 --color '#f2f0de' --at 59.5%,50% --anchor c --anchor-to ink
pixelox transform rotate jacket.pxo --in-place --layer Word --degrees 90
pixelox fill noise jacket.pxo --in-place --as-layer --name Smudge --seed 21 --scale 20% --warp 20 --contrast 10 --stops '#8f907f,#ffffff' --blend multiply --opacity 30
pixelox layer clip jacket.pxo --in-place --layer Smudge
pixelox comp place jacket.pxo wear.png --in-place --as Wear --fit cover
pixelox layer clip jacket.pxo --in-place --layer Wear
pixelox layer opacity jacket.pxo --in-place --layer Wear --value 70
pixelox shape path jacket.pxo --in-place --name Crack --d 'M 600 -6 L 593 49 L 598 103 L 622 166 L 622 229 L 617 311 L 636 373 L 610 419 L 618 481 L 645 550 L 632 611 L 604 665 L 623 730 L 613 791 L 614 852 L 600 929 L 602 994 L 570 1050 L 568 1095 L 587 1170 L 598 1235 L 585 1291 L 596 1353 L 597 1421 L 595 1486 L 589 1544 L 583 1630 L 593 1630 L 598 1538 L 612 1483 L 603 1422 L 612 1353 L 597 1288 L 607 1231 L 601 1166 L 579 1091 L 580 1056 L 611 998 L 611 927 L 628 851 L 623 789 L 633 734 L 609 669 L 641 616 L 652 551 L 624 481 L 626 414 L 646 378 L 630 309 L 637 226 L 628 162 L 609 106 L 604 43 L 607 -11 Z' --fill '#0f1414'
pixelox shape path jacket.pxo --in-place --name Branch --d 'M 622 852 L 685 863 L 730 855 L 775 860 L 820 859 L 865 838 L 910 834 L 985 859 L 910 838 L 865 843 L 820 864 L 775 866 L 730 862 L 685 871 L 622 866 Z' --fill '#0f1414'
pixelox comp place jacket.pxo scuffs.png --in-place --as Scuffs --fit cover
pixelox layer opacity jacket.pxo --in-place --layer Scuffs --value 20
pixelox text add jacket.pxo --in-place --name Author --content "Author Name" --font @oldstyle --weight 600 --style italic --size 3.4% --color '#f2f0de' --at 1.2%,8.6% --anchor l
pixelox text add jacket.pxo --in-place --name Imprint --content "A Midnight Shelf Mystery" --font @oldstyle --weight 600 --style italic --size 2.2% --tracking 20 --color '#f2f0de' --at 1.2%,11.4% --anchor l
pixelox filter grain jacket.pxo --in-place --as-layer --name Grain --amount 15
pixelox export jacket.pxo -o jacket.png
```

Change first: the word; then redraw the crack (list the left edge from top to bottom, then
the right edge from bottom to top, 5 to 17 px apart, a new point every 60 px or so); then
swap the two colors' temperature.

## 10. Variations and pairings

* Other damage, same word-as-object idea: a horizontal slice with the lower half shifted
  5% W; a bite or a bullet hole taken out of one letter; the word melting (T-brush-swash
  drips); the last letter fallen to the bottom edge.
* Colors: blood red `#b3121a` letters on black; black letters on signal yellow `#f5d000`
  for a hazard look; white on ultramarine `#1b2bd6` for a modern reissue.
* Film or theatre poster: add a credit block in the left strip, rotated to read upward
  (T-vertical-type), at cap height 0.9% H.
* Landscape banner: do not rotate; let the word run off the left and right instead.
* Pairs with T-glitch (channel split on the word), T-image-in-type (a face inside the
  letters), and T-frame-border.

## 11. Pitfalls

* Fitting the whole word inside the page: it becomes a normal vertical title. The crop at
  both ends is what makes it huge; keep the length at 104 to 108% H.
* A wide typeface: letters turn into squares and the counters open up. Use the most
  condensed heavy face available and narrow it further with `--scale-x` below 100%; never
  widen it.
* A crack that is straight, evenly wide, or curved like a wave. Jog it every 60 to 100 px
  and vary the width between 5 and 17 px.
* Too much distress: when more than about 10% of the cream is eaten, the letters stop
  reading. Keep the wear at 70% opacity or lower.
* Small text in a sans or in capitals: it then competes with the word. Italic serif,
  mixed case, tucked in the corner.
