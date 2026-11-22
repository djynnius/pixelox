---
id: p010
title: Color-block fashion cover with heavy masthead
genre: [magazine-cover, fashion, editorial]
mood: [bold, energetic, cool, playful]
style: [editorial, photo-composite]
format: portrait-3x4
archetype: A-cover-masthead
palette: ['#0292c2', '#b92630', '#e9d273', '#08154e', '#ffffff', '#2a1814']
techniques: [T-text-behind-subject, T-fit-width, T-vertical-type, T-color-grade, T-paper-grain]
needs: subject-cutout
---

# p010: Color-block fashion cover with heavy masthead

## 1. Concept

A youth-culture fashion cover built from four saturated color blocks: a cyan studio wall,
a red wedge rising from the lower left, a yellow dress, and a royal blue coat, with a
white `[MASTHEAD]` in the heaviest possible capitals across the top and the cover star's
head pushed up in front of its middle letter. All the text besides the masthead is one
thin, quiet column at the left edge. It works because the type is either enormous or
hair-thin, never in between, so the colors and the stare do the talking.

## 2. Format and grid

* Portrait, close to 3:4 (1 : 1.30). Margins 2.5% W at the top, left, and right: the
  masthead almost touches three edges.
* Masthead zone 2.4 to 21% H. Main word 2.2 to 92% W (cap height 18.7% H); a second short
  word runs vertically in the strip 93 to 96% W, top to bottom over the same height.
* Figure: three-quarter length, seated, centered at 52% W, head top at 5% H (inside the
  masthead), body from 15 to 95% W, running off the bottom.
* One text column, centered on x 13.2% W and never wider than 21% W: cover line 43 to
  61.5% H, list 64 to 86% H. It sits on the red wedge.
* Set geometry: the red wedge's upper edge is a straight diagonal from `0,33% H` down to
  `40% W,78% H` (56 degrees from horizontal) with a 2.3% W tan band along it; a small
  orange wedge fills the bottom right corner (91 to 100% W, 76 to 100% H).

## 3. Layer stack

1. Wall: cyan `#0292c2`, lighter (`#12c0dc`) toward the right edge at mid height, a touch
   darker (`#0282bb`) along the top.
2. Red wedge: polygon `0,33%` / `40%,78%` / `40%,100%` / `0,100%`, red `#b92630`.
3. Wedge edge: a tan band `#d9a85e`, 2.3% W wide, along the wedge's diagonal (it reads as
   the lit edge of a painted flat).
4. Orange wedge: bottom right triangle, `#b07a15`.
5. Masthead: 5 letters, white, ultra-heavy grotesque capitals set so tight the letters
   nearly touch, `2.2%,2.4%,89.8% x 18.7%`.
6. Vertical word: an ampersand plus one 8-letter word, same face, white, rotated 90
   degrees to read downward, `93%,2.4%,3.1% x 18.6%`.
7. Cover star cutout: a woman with close-cropped hair and blue-green eye shadow, chin
   resting on the back of her hand, elbow on a raised knee; yellow sleeveless dress with
   large green jewels, royal blue coat with yellow jewels hung over one shoulder, a stack
   of bracelets, striped sock. Glossy, very saturated. Her head hides the masthead's
   middle letter and the inner edges of its neighbors.
8. Cover line: four stacked one-word lines in hair-thin tracked capitals, white, centered
   at 13.2% W, cap tops at 43, 47.8, 52.8, and 57.8% H.
9. List: a lowercase lead-in word and 12 names, one per line, pale pink `#f6c9c9`,
   monospaced, centered at 13.2% W, 64.3 to 85.5% H.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#0292c2` | cyan wall | 30% |
| `#b92630` | red wedge | 15% |
| `#e9d273` | yellow dress | 15% |
| `#2a1814` | skin and hair (photo) | 15% |
| `#08154e` | royal blue coat | 10% |
| `#ffffff` | masthead, cover line | 12% |
| `#d9a85e`, `#b07a15` | tan band, orange wedge | 3% |

A primary triad (blue, red, yellow) at full saturation with white type. Each color owns
one large, simple shape; nothing is blended or graded into anything else.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Masthead | `[MASTHEAD]`, 5 letters | caps | ultra-heavy grotesque, slightly condensed (Archivo Black, Anton set wide / Avenir Next Condensed Heavy plus a stroke) | 900 | 18.7% | -20 | - | left | `#ffffff` | 2.2%, cap top 2.4% |
| Vertical word | "&" plus one word, 8 letters | caps | same | 900 | 2.4% (letters 3.1% W tall when rotated) | 0 | - | rotated 90, reads downward | `#ffffff` | right edge 96%, top 2.4% |
| Cover line | 4 one-word lines of 5, 4, 3, 5 letters | caps | hair-thin monoline sans, slightly condensed (Raleway 100, Jost 100 / Avenir Next Condensed Ultra Light) | 275 | 3.6% | 150 | 100% (pitch 5% H) | center | `#ffffff` | 13.2%, cap top 43% |
| List | "featuring" then 12 names, 1 to 3 words each | lower lead-in, title names | thin monospace (IBM Plex Mono Light, Space Mono / Menlo) | 400 | 1.1% | 0 | 114% (pitch 1.65% H) | center | `#f6c9c9` | 13.2%, top 64.3% |

Three voices: the heaviest face on the page, the thinnest, and a typewriter mono. Cap
heights 18.7 : 3.6 : 1.1, about 17 : 3.3 : 1.

## 6. Imagery

A studio fashion portrait against a painted set: hard frontal flash with a strong specular
sheen on the skin, deep clean shadows, and colors pushed to the limit. The pose makes a
triangle (knee, elbow, chin) that fills the lower two thirds; the head is small in the
frame (24% W) and sits high, so it can overlap the masthead without hiding more than one
letter (**T-text-behind-subject**). The set's diagonal supplies the red wedge; in a build
the wedges are drawn as shapes behind a cutout. Push the cutout's color with
**T-color-grade** settings (vibrance 30 to 40, no hue shift).

## 7. Effects and textures

* **T-text-behind-subject**: masthead, then cutout.
* **T-fit-width**: the main masthead word is fitted to 89% W (`--fit-width 960`); `fx
  stroke` in white, 7 px outside, fattens the stand-in face and closes the gaps between
  the letters.
* **T-vertical-type**: the second word is fitted to the masthead's height while still
  level (`--fit-width 262`), rotated 90 degrees (reads top to bottom), and anchored by its
  top right corner.
* Color blocks: `shape path` polygons; one radial gradient lightens the wall at the right.
* **T-paper-grain** (`filter grain --as-layer --amount 8`): just enough to stop the flat
  cyan from banding.
* No shadows, glows, or outlines on any type.

## 8. Hierarchy and eye path

1. The masthead (white on cyan, 19% H tall) and, inside it, the face.
2. The eyes: the cyan eye shadow repeats the wall color, the gaze is straight at the
   reader.
3. Down the arm to the yellow dress and the jewels.
4. Left to the thin cover line on red, then the list beneath it.

## 9. Build recipe

`subject.png` is your image here: a cutout of a seated or three-quarter figure, about
1400 px tall. With a real figure the head should be near 24% W wide; the stand-in bust is
larger-headed, so it hides more of the word than the original does.

```bash
pixelox new -o cover.pxo --size 1080x1440 --fill '#0292c2'
pixelox layer add cover.pxo --in-place --name Glow
pixelox fill gradient cover.pxo --in-place --layer Glow --kind radial --from 98%,58% --to 35%,58% --stops '#12c0dcff@0,#12c0dc00@1'
pixelox shape path cover.pxo --in-place --name RedPanel --d 'M 0 475 L 432 1132 L 432 1440 L 0 1440 Z' --fill '#b92630'
pixelox shape path cover.pxo --in-place --name PanelEdge --d 'M 0 437 L 0 475 L 432 1132 L 457 1132 Z' --fill '#d9a85e'
pixelox shape path cover.pxo --in-place --name Wedge --d 'M 1080 1100 L 1080 1440 L 985 1440 Z' --fill '#b07a15'
pixelox text add cover.pxo --in-place --name Masthead --content "VIVID" --font @condensed --weight 900 --fit-width 960 --tracking -20 --color '#ffffff' --at 2.6%,2.6% --anchor-to ink
pixelox fx stroke cover.pxo --in-place --layer Masthead --width 7 --color '#ffffff' --position outside
pixelox text add cover.pxo --in-place --name SideWord --content "& CURIOUS" --font @condensed --weight 900 --fit-width 262 --color '#ffffff'
pixelox transform rotate cover.pxo --in-place --layer SideWord --degrees 90
pixelox transform move cover.pxo --in-place --layer SideWord --to 96.2%,2.6% --anchor tr
pixelox comp place cover.pxo subject.png --in-place --as Subject --size x1400 --at 53%,100% --anchor b
pixelox adjust vibrance cover.pxo --in-place --layer Subject --vibrance 35
pixelox text add cover.pxo --in-place --name CoverLine --content $'COLOR\nRUNS\nTHE\nSHOW' --font @condensed-hairline --size 5% --leading 100% --tracking 150 --align center --color '#ffffff' --at 13.4%,43% --anchor t --anchor-to ink
pixelox text add cover.pxo --in-place --name List --content $'featuring\nAda Marlowe\nJune Okoye\nTess Rivera\nPaper Lanterns\nImani Castellan\nRosa van Dalen\nNoor Fields\nKit Hollis\nMarta Quinn\nDelia Sato\nHalcyon\nWren Abara' --font @mono --weight 400 --size 1.45% --leading 114% --align center --color '#f6c9c9' --at 13.2%,64.3% --anchor t --anchor-to ink
pixelox filter grain cover.pxo --in-place --as-layer --name Grain --amount 8
pixelox export cover.pxo -o cover.png
```

Change first: the masthead word (4 to 6 letters; `--fit-width` stays 960), then the
three block colors, taking the wedge color from an accent in the subject's clothing. If
the photo already has a colored backdrop, place it full-bleed under the Masthead layer
(`comp place --fit cover`), then place the cutout of the same photo on top at the same
position so the masthead ends up between them.

## 10. Variations and pairings

* Other triads at full saturation: magenta wall `#d4126f`, orange wedge `#f26a1b`, teal
  clothing; sunflower wall `#f6c21a`, cobalt wedge `#1d3fc4`, red clothing; mint wall,
  violet wedge, coral clothing. Type stays white.
* Black masthead on a pale wall for a daytime, softer issue.
* Move the wedge to the right and mirror the text column when the subject looks left.
* Use it for a music release, a festival line-up (the list becomes the acts), a
  conference speaker roster, or a sports roster poster.
* Pairs with T-frame-border (a white 3% border makes it a framed print), T-duotone on
  the figure for a two-ink version, and T-label-box behind the cover line if the wedge is
  dropped.

## 11. Pitfalls

* A regular-weight masthead. The contrast between the fattest and the thinnest type is
  the whole typographic idea; fatten a too-light face with `fx stroke` or it looks like a
  catalog.
* Thin type on a busy area. Hairline capitals survive only on a flat block (the red
  wedge); over the dress or the skin they vanish.
* Head too large or too low. It must rise into the masthead and hide one letter, at most
  two; a head below the masthead turns the cover into a poster with a title.
* Gradients or shadows between the blocks. The blocks meet at hard edges; the only soft
  transition is the gentle lightening of the wall.
* A long list set big. Twelve names fit because the cap height is 1.1% H and the column
  is centered and narrow; do not widen it past 22% W or it will collide with the figure.
