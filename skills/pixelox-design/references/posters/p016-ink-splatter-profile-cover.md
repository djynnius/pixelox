---
id: p016
title: Ink-splatter profile book cover
genre: [book-cover]
mood: [minimal, dramatic, moody, serious]
style: [minimal, grunge, illustration]
format: portrait-2x3
archetype: A-centered-title
palette: ['#f7f7f6', '#24090c', '#6b1c1f', '#a1262d', '#2b2b2b']
techniques: [T-splatter, T-silhouette, T-grunge, T-distress-type, T-fit-width, T-paper-grain]
needs: subject-cutout
---

# p016: Ink-splatter profile book cover

## 1. Concept

A memoir or literary-fiction cover that is almost empty: scratched white paper, a thin
letterspaced title, and one small image in the middle, a man's bowed head and shoulders in
profile made entirely of dark red ink that breaks up into a splatter with drips where the
chest should be. It works because one saturated color sits alone on 85% white, and because
the figure is unfinished: the body dissolving into a stain says "wounded" without showing
anything. The source shows it as a hardcover standing on a shelf; the flat cover is
described here.

## 2. Format and grid

* Portrait 2:3. One center axis at 50% W; all type is centered on it.
* Margins: 18% W left and right for the title (it spans 19 to 79% W); 5% H at the bottom
  under the author line.
* Zones: title at 19% H; figure 28 to 78% H (head and neck 28 to 50% H, shoulders 50 to 62%
  H, splatter and drips 56 to 78% H); tagline 83 to 87% H; author at 94.3% H.
* The figure is small: 50% W wide at its widest (20 to 70% W), on a page that is otherwise
  blank. The empty band between the title and the head is 7% H; between the last drip and
  the tagline 5% H.

## 3. Layer stack

1. Ground: off-white `#f7f7f6`, full canvas.
2. Paper wear: faint gray clouding (about 8% darker in patches), plus fine gray scratches
   and specks, heaviest near the edges and corners.
3. Ink figure: a head-and-shoulders silhouette in profile facing right, chin lowered,
   20 to 70% W, 28 to 62% H. The back of the head is at 44% W, the nose tip at 67% W, 40% H.
4. Splatter: a ragged ink stain that replaces the chest and left shoulder, core about 30% W
   across centered at 42% W, 64% H, with satellite drops out to 20% W and 69% W and three
   thin vertical drips running down to 78% H. A second smaller blot at 65% W, 68% H.
   Figure and splatter are one continuous shape.
5. Ink texture inside that shape: mottled from near-black red `#24090c` through `#6b1c1f`
   to a bright blood red `#a1262d`, with lighter red scratch lines crossing it diagonally.
6. Title: one word, thin dark-gray caps with very wide tracking, centered at 19% H; a few
   specks of missing ink in the strokes.
7. Tagline: two short centered lines in lowercase, dark gray, at 83.2% and 86.3% H.
8. Author: bold dark-gray caps with wide tracking, centered at 94.3% H.
9. Fine grain.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#f7f7f6` | paper | 83% |
| `#24090c` | darkest ink | 6% |
| `#6b1c1f` | middle ink | 5% |
| `#a1262d` | bright ink and scratch lines | 3% |
| `#2b2b2b` | type | 3% |

White plus one hue (red, from almost black to arterial). The type is dark gray, not red:
the color belongs to the figure alone.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Title | `[BOOK TITLE]`, 1 word, 7 to 9 letters | caps | light wide geometric sans (Montserrat 400, Jost 400 / Avenir Next) | 500 | 3% | 450 | - | center | `#2b2b2b` | 50%, 19% |
| Tagline | 2 lines, about 8 words: whose view, then a list of three themes | sentence | geometric sans with round bowls (Josefin Sans 600 / Futura) | 500 | 1.5% (x-height 1.1%) | 100 | 200% | center | `#2b2b2b` | 50%, 83.2% and 86.3% |
| Author | `[AUTHOR NAME]`, 2 words | caps | geometric sans (Montserrat / Avenir Next) | 700 | 1.7% | 500 | - | center | `#2b2b2b` | 50%, 94.3% |

Three sizes, all small. The title's weight comes from its tracking (60% W for 8 letters)
and from the white around it, not from stroke weight.

## 6. Imagery

A profile bust of a man with short hair, head tilted slightly down, reduced to its outline
(**T-silhouette**) and joined to a thrown-ink stain (**T-splatter**) so the lower third of
the body is replaced by the stain. The joined shape is then filled with a blotchy red ink
texture. Any profile works; a bowed head reads as grief, a lifted chin as defiance. Keep the
figure under 55% W: the smallness is the mood.

## 7. Effects and textures

* **T-silhouette** plus **T-splatter**, merged: the cutout is cropped with a slanted
  polygon mask (`mask add --from-path`), two splats (`dir=90` throws the main one downward
  into drips) are placed over the cut, and `layer merge` bakes the three into one layer
  named Ink.
* Ink fill: a `clouds.js` render (`scale=6`, `warp=1.4`, `contrast=2`) in the three reds,
  clipped to Ink with `layer clip`; a second clipped layer of `grunge.js` scratches in
  `#c8363d` at 70% gives the lighter streaks.
* **T-grunge**, light version: mottle in multiply at 50% and gray marks at 60% on the
  paper only (they sit under the ink).
* **T-distress-type** on the title, very sparse (`specks=2500`, paper color).
* **T-paper-grain**, amount 10, soft-light 35%.
* **T-fit-width**: headline lines are set to exact widths with `transform scale` in the recipe.

## 8. Hierarchy and eye path

1. The red figure: the only color and the only dense shape.
2. Up to the title (dark, wide, the nearest thing above the head).
3. Down the drips to the tagline, then the author.

Cap heights: title 3 : author 1.7 : tagline 1.5, about 2 : 1.1 : 1. The image carries the
hierarchy: its height (50% H) is 17 times the title's cap height.

## 9. Build recipe

`subject.png` is your image here: a profile cutout, head and shoulders (the stand-in is
frontal). The mask polygon keeps the head, the neck, and a slanted slice of the shoulders;
move its lower points to where your figure should start to dissolve.

```bash
pixelox render js scripts/clouds.js -o ink.png --size 1080x1620 --seed 5 --param lo='#1e090b' --param mid='#6b1c1f' --param hi='#b3242b' --param scale=6 --param octaves=6 --param warp=1.4 --param contrast=2 --param res=2
pixelox render js scripts/splatter.js -o splat.png --size 640x640 --seed 6 --param color='#3a0d10' --param radius=0.22 --param drops=260 --param reach=1.5 --param arms=8 --param dir=90
pixelox render js scripts/splatter.js -o splat2.png --size 340x340 --seed 12 --param color='#3a0d10' --param radius=0.17 --param drops=120 --param reach=1.6 --param arms=4
pixelox render js scripts/grunge.js -o marks.png --size 1080x1620 --seed 9 --param color='#8f8f8b' --param specks=6000 --param blotches=6 --param scratches=70 --param clump=0.4
pixelox render js scripts/grunge.js -o scuffs.png --size 1080x1620 --seed 3 --param color='#c8363d' --param specks=1500 --param blotches=20 --param scratches=420 --param clump=0.2 --param size=2
pixelox render js scripts/grunge.js -o wear.png --size 1080x1620 --seed 14 --param color='#f7f7f6' --param specks=2500 --param blotches=60 --param scratches=20 --param clump=0.3
pixelox render js scripts/clouds.js -o mottle.png --size 1080x1620 --seed 21 --param lo='#d9d9d5' --param hi='#ffffff' --param scale=4 --param octaves=6 --param warp=0.4
pixelox new -o cover.pxo --size 1080x1620 --fill '#f7f7f6'
pixelox comp place cover.pxo mottle.png --in-place --as Mottle --fit cover
pixelox layer blend cover.pxo --in-place --layer Mottle --mode multiply
pixelox layer opacity cover.pxo --in-place --layer Mottle --value 50
pixelox comp place cover.pxo marks.png --in-place --as Marks --fit cover
pixelox layer opacity cover.pxo --in-place --layer Marks --value 60
pixelox comp place cover.pxo subject.png --in-place --as Figure --size x700 --at 54%,23.8% --anchor t
pixelox mask add cover.pxo --in-place --layer Figure --from-path 'M 300 380 H 860 V 900 L 700 1000 L 480 1040 L 300 940 Z'
pixelox comp place cover.pxo splat.png --in-place --as Splat --at 45%,61% --anchor c
pixelox comp place cover.pxo splat2.png --in-place --as Splat2 --at 63%,65% --anchor c
pixelox layer merge cover.pxo --in-place --layers Figure,Splat,Splat2 --name Ink
pixelox comp place cover.pxo ink.png --in-place --as InkFill --fit cover
pixelox layer clip cover.pxo --in-place --layer InkFill
pixelox comp place cover.pxo scuffs.png --in-place --as Scuffs --fit cover
pixelox layer clip cover.pxo --in-place --layer Scuffs
pixelox layer opacity cover.pxo --in-place --layer Scuffs --value 70
pixelox text add cover.pxo --in-place --name Title --content "UNDERTOW" --font "Avenir Next" --weight 500 --size 4.2% --tracking 450 --color '#2b2b2b'
pixelox transform scale cover.pxo --in-place --layer Title --size 650x
pixelox transform move cover.pxo --in-place --layer Title --to 50%,19% --anchor c
pixelox comp place cover.pxo wear.png --in-place --as Wear --fit cover
pixelox layer clip cover.pxo --in-place --layer Wear
pixelox text add cover.pxo --in-place --name Tagline --content $'One daughter on salt,\ndistance, and coming back' --font Futura --weight 500 --size 2.1% --tracking 100 --leading 150% --align center --color '#2b2b2b' --at 50%,84.8% --anchor c
pixelox text add cover.pxo --in-place --name Author --content "AUTHOR NAME" --font "Avenir Next" --weight 700 --size 2.35% --tracking 500 --color '#2b2b2b' --at 50%,94.3% --anchor c
pixelox layer add cover.pxo --in-place --name Grain --fill '#808080'
pixelox filter add-noise cover.pxo --in-place --layer Grain --amount 10 --monochrome
pixelox layer blend cover.pxo --in-place --layer Grain --mode soft-light
pixelox layer opacity cover.pxo --in-place --layer Grain --value 35
pixelox export cover.pxo -o cover.png
```

Change first: the cutout and its mask polygon; then the two splat positions so the stain
hides the cut edge; then the three ink colors.

## 10. Variations and pairings

* One-hue swaps: ink blue `#0b1f4b` / `#1f4fa8` / `#4a86e8` for a sea or grief story;
  black to graphite for crime; forest `#0b3d2e` to moss for nature writing. Keep the
  paper white and the type gray.
* Inverted: black paper `#101010`, white or gold ink, white type (T-grunge dark-wall
  settings).
* Other silhouettes: a bird, a house, a hand, a city skyline dissolving the same way; two
  profiles back to back sharing one stain.
* Poster or album cover: scale the figure to 70% W and move the title below it.
* Pairs with T-double-exposure (a landscape instead of the ink texture inside the shape)
  and T-brush-swash (a single stroke under the figure instead of a splat).

## 11. Pitfalls

* A hard straight edge where the cutout ends: the stain must overlap the cut by at least a
  third of its core, and the figure and stain must be merged before the texture is clipped,
  or they read as two objects.
* Splatter thrown in all directions looks like an explosion. Throw it down (`dir=90`) so
  it drips, and keep the head clean.
* Bright red only: the figure turns into a logo. It needs the near-black red for most of
  its area, with the bright red as veins.
* Title made bolder or larger "to be readable": it then fights the figure. Keep it thin,
  3% H, and let the tracking do the work.
* Too much paper dirt: specks near the tagline look like punctuation. Keep marks faint and
  clear of the text lines.
