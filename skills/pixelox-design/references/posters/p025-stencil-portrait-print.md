---
id: p025
title: Four-tone stencil portrait print
genre: [film-poster, editorial, exhibition]
mood: [bold, serious, retro, gritty]
style: [stencil, illustration, flat]
format: portrait-3x4
archetype: A-portrait-caption
palette: ['#0d2637', '#6c929e', '#a82c2b', '#e6dcc0']
techniques: [T-stencil-poster, T-fit-width, T-grunge, T-paper-grain]
needs: subject-cutout
---

# p025: Four-tone stencil portrait print

## 1. Concept

A head-and-shoulders portrait reduced to four flat inks (navy, red, dusty blue, cream) on a
background split down the middle into red and blue, with a two-word caption in condensed
capitals along the bottom. It borrows the language of campaign posters and street
stencils: a face becomes an icon because all photographic detail is thrown away and only
the light-and-shadow shapes remain. Presented as a framed print; the frame is not part of
the design.

## 2. Format and grid

* Portrait 3:4. No margins: the art bleeds to the edge. One vertical axis at 50% W.
* The background is split at 50% W: red on the left, dusty blue on the right. The split
  runs behind the face, so the face has a different ground on each side.
* The figure fills the canvas: hat brim at 6% H, chin at 62% H, shoulders running off both
  sides and the bottom. Head width about 55% W.
* Caption band: the bottom 20% H, navy, merging with the figure's dark clothing.

## 3. Layer stack

1. Ground left half: red `#a82c2b`. Ground right half (x 50 to 100% W): dusty blue
   `#6c929e`.
2. Portrait, four hard-edged tones mapped from dark to light: navy `#0d2637` (hat, beard,
   clothing, deep shadow), red `#a82c2b` (shadow side of the face, collar), dusty blue
   `#6c929e` (mid-tones), cream `#e6dcc0` (lit planes of the face, shirt collar). The light
   comes from the left, so cream sits on the left of the face and red on the right.
3. Caption band: navy, 0,80%,100% x 20%.
4. Caption: two short words, dusty light blue `#7fa3b0`, centered, cap height 9% H,
   about 72% W wide, baseline at 95% H.
5. Print wear: sparse cream specks and scratches over everything, 30% opacity.
6. Paper grain: one grain layer over everything.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#0d2637` | navy: darkest tone, caption band | 40% |
| `#6c929e` | dusty blue: right ground, mid-tones | 25% |
| `#a82c2b` | red: left ground, shadow tone | 20% |
| `#e6dcc0` | cream: highlights only | 10% |
| `#7fa3b0` | caption | 5% |

A red, white, and blue triad pulled toward teal and cream so it looks printed, not
digital. Cream is the rarest color and marks the focal planes of the face.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Caption | 2-word imperative or slogan, 4 to 5 letters per word | caps | condensed bold gothic (League Gothic, Oswald Bold / Impact, DIN Condensed) | 700 | 9% | 20 | - | center | `#7fa3b0` | 50%, baseline 95% |

One line of type. It is muted (blue on navy, not white) so the face stays first.

## 6. Imagery

A three-quarter portrait of a man in a brimmed hat and glasses with a goatee, wearing a
dark jacket over an open collar, lit hard from the left. For a build, any portrait with
strong side light, a clear outline (hat, hair, beard, collar) and a calm expression works.
Treatment: **T-stencil-poster**. Black and white first, a median filter to round the
shapes, then a gradient map with four hard bands. In the original the bands were drawn by
hand and include fine hatch lines in the mid-tones; the filter version is cleaner.

## 7. Effects and textures

* **T-stencil-poster**: bands at 0 to 0.3 navy, 0.3 to 0.5 red, 0.5 to 0.72 dusty blue,
  0.72 to 1 cream; `median` radius 5.
* **T-grunge** (dirt layer only) in cream at 30%, for scuffed ink.
* **T-paper-grain**: `filter grain --as-layer --amount 20`, last (visible print grain).
* **T-fit-width**: the caption is one `text add --fit-width 780`, placed by its visible
  letters with `--anchor b --anchor-to ink`, so its baseline sits exactly at 95% H.

## 8. Hierarchy and eye path

1. The cream planes of the face (the only light color), the eyes first.
2. The dark hat and beard outline.
3. The caption.

There is no real typographic hierarchy: face about 70% of the area, caption cap height 9% H.

## 9. Build recipe

`subject.png` is your image here: a cutout portrait with side lighting.

```bash
pixelox new -o print.pxo --size 1080x1440 --fill '#a82c2b'
pixelox shape rect print.pxo --in-place --name RightHalf --rect 50%,0,50%x100% --fill '#6c929e'
pixelox comp place print.pxo subject.png --in-place --as Subject --size x1330 --at 50%,100% --anchor b
pixelox adjust bw print.pxo --in-place --layer Subject
pixelox adjust levels print.pxo --in-place --layer Subject --in-black 25 --in-white 215
pixelox filter median print.pxo --in-place --layer Subject --radius 5
pixelox adjust gradient-map print.pxo --in-place --layer Subject --stops '#0d2637@0,#0d2637@0.3,#a82c2b@0.3,#a82c2b@0.5,#6c929e@0.5,#6c929e@0.72,#e6dcc0@0.72,#e6dcc0@1'
pixelox shape rect print.pxo --in-place --name Band --rect 0,80%,100%x20% --fill '#0d2637'
pixelox text add print.pxo --in-place --name Caption --content "STAY SHARP" --font @condensed-heavy --fit-width 780 --color '#7fa3b0' --at 50%,95% --anchor b --anchor-to ink
pixelox render js scripts/grunge.js -o wear.png --size 1080x1440 --seed 5 --param color='#e6dcc0' --param specks=5000 --param blotches=40 --param scratches=30 --param clump=0.5
pixelox comp place print.pxo wear.png --in-place --as Wear --fit cover
pixelox layer opacity print.pxo --in-place --layer Wear --value 30
pixelox filter grain print.pxo --in-place --as-layer --name Grain --amount 20
pixelox export print.pxo -o print.png
```

Change first: the band edges in `--stops` (0.3, 0.5, 0.72) until navy holds the hair and
deep shadows and cream sits only on the lit planes; then the caption.

## 10. Variations and pairings

* Other triads: black `#111111` / orange `#e85d04` / sand `#e9d8a6` / white; forest
  `#0b3d2e` / gold `#e9b44c` / cream / rust `#9b2915`. Keep one dark, one light, two mids.
* Three tones instead of four (drop the red band) for a harder stencil.
* Add a masthead and a border (T-frame-border) for a magazine-cover version.
* Works for speakers, artists, athletes, memorial and tribute posters, podcast covers.
* Pairs with T-halftone-portrait (halftone only the dusty-blue band), T-sunburst behind the
  head, and T-distress-type on the caption.

## 11. Pitfalls

* Flat, front-lit photos give a face of one tone. Choose hard side light, or raise the
  contrast before mapping.
* Too many small islands of color (noise in the bands): raise the `median` radius to 7 or
  8, or blur slightly before the gradient map.
* Skipping the cream: with only dark and mid tones the face has no focus.
* A white or bright caption steals attention from the eyes. Keep it a mid-tone from the
  palette.
* Using this on a real person to imply an endorsement or a political message they did not
  give. The style carries that meaning; use it with consent.
