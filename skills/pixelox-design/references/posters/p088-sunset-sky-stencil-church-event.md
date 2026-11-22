---
id: p088
title: Sunset sky church event with stencil type
genre: [church, event-flyer]
mood: [warm, calm, reverent, friendly]
style: [illustration, flat, type-led]
format: portrait-a
archetype: A-swiss-asymmetric
palette: ['#efc49a', '#ca73ab', '#d88237', '#000000', '#ffffff', '#ffd400']
techniques: [T-smoke, T-label-box, T-rules, T-fit-width, T-paper-grain, T-stencil-type]
needs: illustration
---

# p088: Sunset sky church event with stencil type

## 1. Concept

A church-activities poster on a soft sunset sky that runs from orchid pink at the top
through peach to orange at the bottom, with a two-line black headline in a wide,
stencil-cut display face hung from the left margin, two white line-art doves, and a solid
black cross in the lower right. It works because the sky is pale and low in contrast, so
pure black type and pure white birds both read on it without panels or shadows, and
because every element keeps to the left margin except the cross and one dove, which
balance the corners.

## 2. Format and grid

* Portrait A-series (1 : 1.414). Left margin 10.3% W; right margin 7.5% W; top 4% H;
  bottom 4% H.
* One left edge for all text (10.3% W): label 15.5 to 18.4% H; headline 20.5 to 43% H;
  rule 48.7% H; body 54.5 to 61% H; welcome lines 65.5 to 69% H.
* Corner objects: dove A top right (61 to 92% W, 4 to 18% H); dove B bottom left (11 to
  55% W, 70 to 96% H); cross bottom right (75.5 to 93% W, 73.6 to 96% H).
* The right half of the band from 44 to 70% H is empty sky.

## 3. Layer stack

1. Sky gradient, top to bottom: orchid `#ca73ab` at 0, pink `#e280b1` at 6% H, salmon
   `#eeaaa9` at 30% H, peach `#efc49a` at 50% H, pink-peach `#f7baac` at 64% H, orange
   `#d88237` at the bottom.
2. Clouds: large soft cumulus shapes, lighter than the sky (cream `#fff1e0`), strongest
   in a diagonal band from the right edge at 40% H to the left edge at 62% H; a few
   darker mauve wisps in the top third. Low contrast: lightness differs from the sky by
   under 10%.
3. Label: a yellow `#ffd400` pill, `10.3%,15.5%,59.4% x 2.9%`, fully round ends, with a
   line of small black tracked caps centered inside.
4. Headline line 1: 6 letters, black, 75.5% W wide, cap height 10.6% H, top at 20.5% H.
   Line 2: 5 letters, 58% W wide, cap height 9.2% H, top at 33.7% H. Each letter is cut
   by thin stencil gaps (1 to 3 per letter, 0.8% W wide) through which the sky shows.
5. Rule: black, 0.4% H thick, from 10.3 to 27.5% W at 48.7% H.
6. Body: three lines of small black text, 55.4, 57.8, 60.2% H, up to 65% W wide.
7. Welcome: two lines of bold, wide-tracked black caps at 66% and 68.3% H.
8. Dove A (top right): a white dove in flight with a thin black outline, wings up, an
   olive sprig in its beak, facing left; 31% W wide.
9. Dove B (bottom left): the same style, larger (44% W), wings spread downward, facing
   right.
10. Cross: a solid black Latin cross, 17.6% W wide and 22.3% H tall, bar thickness 4.5%
    W, crossbar 31% of the way down.
11. Grain: very light (`filter grain`, amount 8), over the sky only.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#efc49a` | peach middle of the sky | 35% |
| `#ca73ab` | orchid top (through `#e280b1`) | 25% |
| `#d88237` | orange bottom | 20% |
| `#000000` | headline, text, cross, outlines | 10% |
| `#ffffff` | doves | 7% |
| `#ffd400` | label pill | 3% |

An analogous sunset run (orchid, pink, peach, orange) with black and white as the only
contrast and one spot of yellow. Nothing is gray: the black is pure, which is what gives
a pastel background some authority.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Label | 4 words, about 35 characters | caps | geometric sans (Montserrat 500 / Avenir Next) | 500 | 1.4% | 100 | - | center in pill | `#000000` | 40%, 17% |
| Headline 1 | 1 word, 6 letters | caps | wide heavy stencil display (Saira Stencil One, Big Shoulders Stencil 800, Stardos Stencil 700 / Avenir Next Heavy plus cut gaps) | 900 | 10.6% | 10 | - | left | `#000000` | 10.3%, top 20.5% |
| Headline 2 | 1 word, 5 letters | caps | same | 900 | 9.2% | 10 | - | left | `#000000` | 10.3%, top 33.7% |
| Body | 3 lines, 9 to 12 words each | sentence | humanist sans (Open Sans 400, Noto Sans 400 / Avenir Next) | 400 | 1.2% | 0 | 141% (pitch 2.4% H) | left | `#000000` | 10.3%, 55.4 to 60.2% |
| Welcome | "WELCOME TO" / "THE `[CHURCH NAME]`" | caps | geometric sans (Montserrat 700 / Avenir Next) | 700 | 1.4% | 150 | 118% | left | `#000000` | 10.3%, 66% and 68.3% |

Headline to small text is 7.5 : 1 with nothing in between; the label pill and the rule
act as the middle level.

## 6. Imagery

Two drawn doves with transparency (`illustration.png`): white fills, even black
outlines about 0.3% W thick, feather lines on wings and tail, an olive branch in the
beak. One is used small in the top right and one large in the bottom left, facing each
other across the page diagonal. Any white line-art motif with a black outline works
(lilies, hands, candles, a chalice); keep the fill pure white so it separates from the
pastel sky. The cross is built from two rectangles.

The sky is not a photograph: a six-stop vertical gradient with soft cloud noise on top.

## 7. Effects and textures

* Sky: `fill gradient` with six stops; clouds from `fill noise --as-layer` (the **T-smoke**
  command with the default `clouds` kind and warm cream stops, `--scale 45%`, `--warp 36`,
  `--bias=-10`), `screen` at 30%; a second, mauve noise layer (`--scale 33%`, `--stretch
  0.6`, `--bias=-36`) in `multiply` at 18% for the darker wisps. Blend mode and opacity
  are set on the same command.
* Stencil gaps: with no stencil font installed, cut the gaps with vector masks. `mask add
  --from-path` on each headline layer with a path made of full-height bands that leave
  thin vertical strips uncovered: one 9 px slit through the middle of each letter (none
  through an I). The sky shows through the cuts, as with a real stencil. A band from x1
  to x2 is `M x1 0 H x2 V 1528 H x1 Z`; consecutive bands leave the 9 px gaps.
* **T-label-box**: the yellow pill (radius = half its height).
* **T-rules**: the short thick rule between headline and body.
* **T-fit-width**: each headline line fitted to its own width (`--fit-width 815` and
  `626`).
* **T-paper-grain**: `filter grain --as-layer --amount 8`, added over the sky and under
  the type.
* Mirrored dove: `transform flip illustration.png -o illustration-mirrored.png --h` makes
  a mirrored copy of the file first, and that copy is placed. Flipping the placed layer
  instead would rasterize it (warning PXW2003).
* **T-stencil-type** in [techniques.md](../techniques.md) is the tested library version of the stencil cuts through the letters; start from it when reusing this idea.

## 8. Hierarchy and eye path

1. The black headline (largest, darkest, upper left).
2. The white dove at the top right, then diagonally down to the large dove.
3. The cross, the only other solid black shape, anchoring the lower right.
4. Label pill, body text, welcome lines.

The two doves and the cross form a triangle around the text column.

## 9. Build recipe

`illustration.png` is your image here: a white line-art motif with transparency, wider
than tall (it is placed twice; the first copy is mirrored as a file before it is
placed). Canvas 1080x1528.

```bash
pixelox new -o sky.pxo --size 1080x1528 --fill '#efc49a'
pixelox layer add sky.pxo --in-place --name Sunset
pixelox fill gradient sky.pxo --in-place --layer Sunset --from 0,0 --to 0,100% --stops '#ca73ab@0,#e280b1@0.06,#eeaaa9@0.3,#efc49a@0.5,#f7baac@0.64,#d88237@1'
pixelox fill noise sky.pxo --in-place --as-layer --name Clouds --seed 12 --scale 45% --octaves 5 --warp 36 --bias=-10 --stops 'transparent,#fff1e0' --blend screen --opacity 30
pixelox fill noise sky.pxo --in-place --as-layer --name Wisps --seed 31 --scale 33% --warp 44 --stretch 0.6 --angle 90 --bias=-36 --stops 'transparent,#9a5a8a' --blend multiply --opacity 18
pixelox filter grain sky.pxo --in-place --as-layer --name Grain --amount 8
pixelox shape rect sky.pxo --in-place --name Pill --rect 10.3%,15.5%,59.4%x2.9% --radius 22 --fill '#ffd400'
pixelox text add sky.pxo --in-place --name PillText --content "A WEEKEND OF SONG AND SERVICE" --font @geometric-sans --weight 500 --size 1.9% --tracking 100 --color '#000000' --at 40%,17% --anchor c
pixelox text add sky.pxo --in-place --name H1 --content "SPRING" --font @geometric-sans --weight 900 --fit-width 815 --tracking 10 --color '#000000' --at 10.3%,20.5% --anchor-to ink
pixelox mask add sky.pxo --in-place --layer H1 --from-path 'M 0 0 H 165 V 1528 H 0 Z M 174 0 H 310 V 1528 H 174 Z M 319 0 H 460 V 1528 H 319 Z M 469 0 H 685 V 1528 H 469 Z M 694 0 H 850 V 1528 H 694 Z M 859 0 H 1080 V 1528 H 859 Z'
pixelox text add sky.pxo --in-place --name H2 --content "FEAST" --font @geometric-sans --weight 900 --fit-width 626 --tracking 10 --color '#000000' --at 10.3%,33.7% --anchor-to ink
pixelox mask add sky.pxo --in-place --layer H2 --from-path 'M 0 0 H 160 V 1528 H 0 Z M 169 0 H 290 V 1528 H 169 Z M 299 0 H 430 V 1528 H 299 Z M 439 0 H 567 V 1528 H 439 Z M 576 0 H 687 V 1528 H 576 Z M 696 0 H 1080 V 1528 H 696 Z'
pixelox shape line sky.pxo --in-place --name Rule --from 10.3%,48.7% --to 27.5%,48.7% --stroke '#000000' --stroke-width 6
pixelox text add sky.pxo --in-place --name Body --content $'Three days of shared meals, music, and practical help for our neighbors.\nCome for one hour or the whole weekend; there is a place for every pair\nof hands, and a seat at the table when the work is done.' --font @geometric-sans --weight 400 --size 1.7% --leading 141% --color '#000000' --at 10.3%,57.8% --anchor l
pixelox text add sky.pxo --in-place --name Welcome --content $'WELCOME TO\nTHE COMMUNITY CHURCH' --font @geometric-sans --weight 700 --size 1.95% --tracking 150 --leading 118% --color '#000000' --at 10.3%,67.2% --anchor l
pixelox transform flip illustration.png -o illustration-mirrored.png --h
pixelox comp place sky.pxo illustration-mirrored.png --in-place --as DoveA --size 300x --at 79%,9% --anchor c
pixelox comp place sky.pxo illustration.png --in-place --as DoveB --size 480x --at 33%,84% --anchor c
pixelox shape rect sky.pxo --in-place --name CrossPost --rect 883,1125,49x341 --fill '#000000'
pixelox shape rect sky.pxo --in-place --name CrossBar --rect 815,1196,190x47 --fill '#000000'
pixelox export sky.pxo -o sky.png
```

Change first: the two headline words (5 to 7 letters each), then look at the export and
move the slits (the band edges in the two mask paths) so that each falls on the middle
of a letter, not between letters. With a real stencil font registered
(`fonts add`), drop both `mask add` lines.

## 10. Variations and pairings

* Dawn version: `#9fb7e8` (top), `#f3d6c6`, `#ffe2a8` (bottom) with navy `#14213d` type
  instead of black.
* Night vigil: `#141a3a` to `#3a2a5a` to `#b0563a`, white type and cross, doves kept
  white; add T-bokeh for stars.
* Replace doves with other line art: palm branches, a bread-and-cup motif, music notes
  for a choir event.
* Square post: label, headline, and one dove; drop the body.
* Pairs with T-sunburst behind the cross (rays in cream at 25%), T-info-bar for date,
  time, and place, and T-script-overlap for a one-word script accent over line 2.

## 11. Pitfalls

* A saturated or high-contrast sky fights the black type. Keep cloud contrast under 10%
  lightness and the hues within pink to orange.
* Gray or brown type "to match the sunset" turns muddy. Pure black is the anchor.
* Stencil gaps that are too wide or too many shred the letters. Gap width 5 to 7% of the
  cap height, at most three cuts per letter.
* Doves with gray shading or drop shadows lose the paper-cut feel; flat white with one
  black outline weight.
* The cross drawn with thin bars looks like a plus sign. Bar thickness should be a
  quarter of the cross's width, and the crossbar sits in the upper third.
