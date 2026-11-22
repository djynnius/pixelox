---
id: p055
title: Photo-filled letter grid cover
genre: [book-cover, exhibition, editorial]
mood: [minimal, serious, calm, cool]
style: [type-led, minimal, swiss]
format: portrait-2x3
archetype: A-type-block
palette: ['#f1f1ef', '#8d8a89', '#b5b3b1', '#383735']
techniques: [T-image-in-type, T-fit-width, T-mono-on-color, T-paper-grain]
needs: photo
---

# p055: Photo-filled letter grid cover

## 1. Concept

A science-history cover whose title is its own illustration: a four-letter word is broken
into a two-by-two grid of giant geometric capitals on a pale gray-white sheet, and one
black-and-white photograph of a cratered, airless landscape shows through all four letters as
if they were windows cut in the paper. The two round letters are drawn as solid discs with
no hole, so each one becomes a small globe of that landscape. It works because the subject, the word, and the
picture are the same thing, and because the huge letters carry all the texture while the
rest of the page is silent.

## 2. Format and grid

* Portrait 2:3 (close to it: 1 : 1.55). Margins 6 to 8% W left and right.
* One center axis for the small lines; a two-column, two-row grid for the letters:
  columns 7.6 to 46.4% W and 50.6 to 93% W (gutter 4% W); rows 22.2 to 50% H and 55.5 to
  83.5% H (gutter 5.5% H).
* Stack from the top: subtitle 6.2% H; article 14.6% H; letter grid 22 to 84% H; author
  89.8% H; one more small line 96.1% H.
* Each letter is about 28% H tall and 39 to 43% W wide, so the four letters cover 60% of
  the sheet's height. The discs are 43% W across, slightly larger than the straight
  letters, and positioned diagonally (top right, bottom left).

## 3. Layer stack

1. Ground: gray-white `#f1f1ef`, full canvas.
2. Letter shapes (the window): row 1, a straight-sided capital at 7.6 to 46.4% W and a
   solid disc centered at 71.8% W, 36.3% H; row 2, a solid disc centered at 27.3% W, 69.5%
   H and a straight-sided capital at 53.6 to 92.4% W. Plus the article above them: 3
   heavy caps, cap height 4.8% H, 35 to 64.4% W, widely tracked.
3. Photo inside those shapes only: a high-contrast black-and-white aerial view of a
   cratered, dusty surface lit from the side: mid-gray plains `#8d8a89`, pale rims
   `#b5b3b1`, hard near-black shadows `#383735` inside craters and behind ridges. The
   picture is continuous: a ridge that leaves one letter continues in the next.
4. Subtitle: one line of tracked dark caps across 84% W at 6.2% H.
5. Author: one line of light, tracked dark caps, 72.8% W wide, at 89.8% H.
6. Small line: lowercase words followed by tracked caps, centered at 96.1% H.
7. Very light grain.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#f1f1ef` | paper | 62% |
| `#8d8a89` | photo mid-gray | 17% |
| `#b5b3b1` | photo light gray | 12% |
| `#383735` | photo shadows, all small type | 9% |

No color at all: four grays. The paper is the lightest value on the page; the photo never
reaches white, so the letter shapes stay solid.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Subtitle | 5 words | caps | geometric sans (Montserrat / Avenir Next) | 600 | 1.7% | 350 | - | center, 84% W wide | `#383735` | 50%, 6.2% |
| Article | "THE" | caps | geometric sans, heavy (Montserrat 800 / Avenir Next) | 800 | 4.8% | 400 | - | center | photo fill | 49.7%, 14.6% |
| Title letters | `[BOOK TITLE]`: 1 word, 4 letters, the middle two round, set 2 + 2 | caps | geometric sans, bold (Montserrat 700, Outfit 700 / Avenir Next), round letters replaced by discs | 700 | 28% | - | 33% H row pitch | grid | photo fill | see section 2 |
| Author | `[AUTHOR NAME]`, 2 words | caps | geometric sans, light | 400 | 2.7% | 300 | - | center, 72.8% W wide | `#383735` | 50%, 89.8% |
| Small line | "author of" + `[EARLIER TITLE]` | lower + caps | same | 500 | 1.4% | 150 | - | center | `#383735` | 50%, 96.1% |

One geometric family throughout. Size ratio title letters : article : author : subtitle :
small line = 28 : 4.8 : 2.7 : 1.7 : 1.4.

## 6. Imagery

One photograph, seen only through the letters. It must be a texture, not a scene: craters,
sand ripples, bark, waves, a city from above, a crowd, marble. Side light and hard shadows matter
more than subject, because the letter edges are defined by the contrast between photo and
paper. Convert to black and white and raise the contrast until the darkest tenth is
near-black (**T-mono-on-color** settings: `adjust bw`, `adjust levels --in-black 30
--in-white 215 --out-white 205`), and cap the highlights below the paper's lightness.

## 7. Effects and textures

* **T-image-in-type**: all window shapes (two text letters, two `shape ellipse` discs,
  the article) are merged into one layer; the photo is placed above it and clipped with
  `layer clip`.
* **T-fit-width**: each straight letter is its own text layer scaled to an exact box
  (`transform scale --size 419x452`, a non-uniform scale that makes the letter a little
  taller than the font draws it).
* **T-paper-grain**, amount 8, soft-light 25%. No shadows, no outlines on the letters.

## 8. Hierarchy and eye path

1. The four letter shapes, read as a Z: top-left, top-right, bottom-left, bottom-right.
2. The article above them completes the title.
3. The author's name below.
4. Subtitle at the very top, then the small line at the foot.

## 9. Build recipe

`photo.png` is your image here: a textured surface with strong side light. The recipe
spells a different 4-letter word with two round middle letters; for other words, any
letter can be text (scale it to its grid cell) and any round letter a disc.

```bash
pixelox new -o grid.pxo --size 1080x1620 --fill '#f1f1ef'
pixelox text add grid.pxo --in-place --name LetterA --content "N" --font "Avenir Next" --weight 700 --size 30% --color '#383735'
pixelox transform scale grid.pxo --in-place --layer LetterA --size 419x452
pixelox transform move grid.pxo --in-place --layer LetterA --to 7.6%,22.2% --anchor tl
pixelox shape ellipse grid.pxo --in-place --name DiscA --rect 543,356,464x464 --fill '#383735'
pixelox shape ellipse grid.pxo --in-place --name DiscB --rect 61,892,468x468 --fill '#383735'
pixelox text add grid.pxo --in-place --name LetterB --content "N" --font "Avenir Next" --weight 700 --size 30% --color '#383735'
pixelox transform scale grid.pxo --in-place --layer LetterB --size 419x449
pixelox transform move grid.pxo --in-place --layer LetterB --to 53.6%,55.5% --anchor tl
pixelox text add grid.pxo --in-place --name Article --content "THE" --font "Avenir Next" --weight 800 --size 6.5% --tracking 400 --color '#383735'
pixelox transform scale grid.pxo --in-place --layer Article --size 318x
pixelox transform move grid.pxo --in-place --layer Article --to 49.7%,14.6% --anchor c
pixelox layer merge grid.pxo --in-place --layers LetterA,DiscA,DiscB,LetterB,Article --name Window
pixelox comp place grid.pxo photo.png --in-place --as Surface --fit cover
pixelox layer clip grid.pxo --in-place --layer Surface
pixelox adjust bw grid.pxo --in-place --layer Surface
pixelox adjust levels grid.pxo --in-place --layer Surface --in-black 30 --in-white 215 --out-white 205
pixelox text add grid.pxo --in-place --name Subtitle --content "A FIELD GUIDE TO THE SKY" --font "Avenir Next" --weight 600 --size 2.3% --tracking 350 --color '#383735'
pixelox transform scale grid.pxo --in-place --layer Subtitle --size 907x
pixelox transform move grid.pxo --in-place --layer Subtitle --to 50%,6.2% --anchor c
pixelox text add grid.pxo --in-place --name Author --content "AUTHOR NAME" --font "Avenir Next" --weight 400 --size 3.7% --tracking 300 --color '#383735'
pixelox transform scale grid.pxo --in-place --layer Author --size 786x
pixelox transform move grid.pxo --in-place --layer Author --to 50%,89.8% --anchor c
pixelox text add grid.pxo --in-place --name SmallLine --content "author of  EARLIER BOOK" --font "Avenir Next" --weight 500 --size 1.9% --tracking 150 --color '#383735' --at 50%,96.1% --anchor c
pixelox layer add grid.pxo --in-place --name Grain --fill '#808080'
pixelox filter add-noise grid.pxo --in-place --layer Grain --amount 8 --monochrome
pixelox layer blend grid.pxo --in-place --layer Grain --mode soft-light
pixelox layer opacity grid.pxo --in-place --layer Grain --value 25
pixelox export grid.pxo -o grid.png
```

Change first: the photo (try `transform move --layer Surface` to put its strongest
shadow inside a disc), then the four letters, then add one tint with T-duotone if the
subject needs color.

## 10. Variations and pairings

* Subject swaps: water photo for a 4-letter water word, bark or leaves for a tree word,
  a night-city aerial for a place name, sand, coral, fabric. Round letters stay
  discs (suns, wheels, coins, plates).
* Six-letter words in a 3 x 2 grid (letters 18% H tall); eight letters in 4 x 2 on a
  landscape or square canvas.
* One color: **T-duotone** on the Surface layer (navy to pale blue, rust to sand) with
  the type left dark gray.
* Inverted: black paper `#101010`, the same photo, white small type.
* Museum poster, annual-report cover, album sleeve, travel brochure.
* Pairs with T-frame-border and T-rules (a thin rule under the subtitle).

## 11. Pitfalls

* A photo with large pale areas: parts of letters vanish into the paper. Keep the photo's
  highlights under 80% lightness or add a 1 px outline to the window layer.
* A photo with one small subject (a person, a building): each letter shows an unreadable
  fragment. Use an all-over texture.
* Thin or light letterforms: the window is too narrow to show a picture. Bold weight or
  heavier, cap height 25% H or more.
* Clipping the photo to each letter separately with different crops: the surface no
  longer continues from letter to letter. Merge the shapes first, clip once.
* Letters set as one normal word across the page: at four letters it needs the 2 x 2
  break to reach this scale.
