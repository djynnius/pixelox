---
id: p103
title: Object-mosaic portrait film poster
genre: [film-poster, editorial]
mood: [serious, dramatic, bold, gritty]
style: [photo-composite, minimal, halftone]
format: portrait-2x3
archetype: A-cover-masthead
palette: ['#fdfdfc', '#140e0b', '#472b20', '#825742', '#b88e71', '#e2d4c2']
techniques: [T-duotone, T-pattern, T-rules]
needs: subject-cutout
---

# p103: Object-mosaic portrait film poster

## 1. Concept

A film poster about an arms dealer in which the star's face, shoulders, suit, and tie are
a mosaic made entirely of rifle cartridges standing on end: brass for the skin, dark
steel for the suit, each bullet one "pixel". From across the street it is a calm frontal
portrait on white; from one step away it is thousands of rounds of ammunition. The title
block is small, in quiet classical capitals, at the top. It works because the material of the picture is the
subject of the film, so the image explains the man without a single prop, and because
the white ground and the dissolving edge keep a very busy texture clean.

## 2. Format and grid

* Portrait 2:3. Strict center axis; margins 11% W for the title.
* Top block, 2.5 to 16% H: star's name (cap height 2% H) at 3.7% H; title (cap height
  7.7% H, 80% W wide) centered at 9.5% H; a hairline and a one-line tagline at 14.5 to
  15.5% H.
* Portrait, 18 to 100% H: a frontal head and shoulders, symmetric. Hair top at 18% H, eyes
  at 45% H, chin at 72% H; the face is 60% W wide (20 to 80% W); the shoulders reach both
  edges at 84% H and run off the bottom.
* The portrait has no outline: along the hair and shoulders the mosaic breaks into single
  loose pieces over a band about 5% W wide, then stops. Everything else is white.

## 3. Layer stack

1. Ground: white `#fdfdfc`, full canvas.
2. Loose pieces: a few hundred single upright cartridges scattered in a halo around the
   head and shoulders, denser near the figure, in brass and dark brown.
3. Portrait mosaic: a man's face, front view, neutral expression, pale blue-gray eyes,
   dark suit, light shirt, patterned tie. Built from upright capsule shapes about 1.5% W
   wide and 4% H tall in tight columns; tones from near-black `#140e0b` and dark brown
   `#472b20` (suit, hair, shadows) through copper `#825742` and brass `#b88e71` (skin) to
   pale cream `#e2d4c2` (highlights, shirt). Thin white gaps separate the pieces.
4. Star's name: small charcoal serif caps, tracked, centered, 3.7% H.
5. Title: two words in classical inscriptional capitals (thin serifs, wide round letters,
   regular weight) in charcoal `#2a2624`, with a small 2-letter word set between them at
   30% of their height; centered, 80% W wide.
6. Hairline under the title (60% W wide) and a tagline in tiny tracked gray caps.
7. Optional: two faint columns of tiny pale-gray text running down the side margins
   (3 to 8% W and 92 to 97% W), so light they read as texture.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#fdfdfc` | ground, gaps | 40% |
| `#140e0b` | suit, hair, type | 18% |
| `#472b20` | dark brown shadows | 13% |
| `#825742` | copper mid-tones | 10% |
| `#b88e71` | brass skin tones | 10% |
| `#e2d4c2` | cream highlights | 9% |

A metal ramp (black-brown to brass to cream) on white. The eyes keep a trace of cold
blue-gray, the only non-warm color, which is why they hold the gaze.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Star | `[ACTOR NAME]`, 2 words | caps | inscriptional serif (Cinzel, Marcellus / Big Caslon) | 500 | 2% | 120 | - | center | `#4a4441` | 50%, 3.7% |
| Title words (x2) | `[FILM TITLE]`: 4 letters and 3 to 4 letters | caps | inscriptional serif (Cinzel, Marcellus / Big Caslon) | 500 | 7.7% | 0 | - | center, 80% W in all | `#2a2624` | 50%, 9.5% |
| Small connector | "OF" | caps | same | 500 | 2.3% | 0 | - | center, between the two words | `#2a2624` | 49.5%, 9.5% |
| Tagline | one sentence, 6 to 9 words | caps | same | 500 | 0.7% | 150 | - | center | `#6b6460` | 50%, 15.3% |

One classical serif in capitals, never bold: the calm, expensive voice of the type is the
irony against a face made of ammunition. The title block is deliberately modest (16% of
the height) so the portrait can be 82%.

## 6. Imagery

A frontal studio portrait, symmetric, evenly lit, turned into a mosaic of one repeated
object. The real thing is a hand-built photomosaic (each tile a photo of the object,
tinted to the tone it stands for). A buildable version keeps the two things that matter:
the picture is quantized into small upright tiles with gaps, and the tones are mapped to
the object's material colors (**T-duotone** with five stops on the metal ramp). Choose
the tile for the story: coins for a banker, pills, film frames, keys, flowers, matches,
bricks, punch cards. A frontal face with large simple planes survives the coarse tiling
best.

## 7. Effects and textures

* Tone map: `adjust levels --in-black 20 --in-white 225`, then **T-duotone**:
  `#140e0b`, `#472b20`, `#825742`, `#b88e71`, `#e2d4c2`.
* Tiling: `filter pixelate --cell 14` on the cutout (alpha is quantized too, so the
  outline becomes steps).
* Gaps (**T-pattern**): white vertical stripes every 14 px (weight 0.22) and white
  horizontal stripes every 42 px (weight 0.07), each clipped to the portrait: tiles
  become upright bars 14 by 42 px.
* Loose pieces: `particles.js` with `shape=rect`, upright (`angle=-90`, `jitter=4`), in
  the ramp's colors, clustered around the head, placed under the portrait.
* **T-rules**: one hairline under the title. No grain, no shadow: the ground stays pure
  white.

## 8. Hierarchy and eye path

1. The face (it is 60% of the width and looks at the viewer), the eyes first.
2. The texture: the viewer steps in and reads what the face is made of.
3. Up to the title, then the star's name above it.
4. The tagline.

Sizes: face 54% H tall; title caps 7.7% H; name 2% H; tagline 0.7% H.

## 9. Build recipe

`subject.png` is your image here: a frontal head-and-shoulders cutout with even light.

```bash
pixelox render js scripts/pattern.js -o gaps-v.png --size 1080x1620 --param kind=stripes --param color='#fdfdfc' --param cell=14 --param weight=0.22 --param angle=0
pixelox render js scripts/pattern.js -o gaps-h.png --size 1080x1620 --param kind=stripes --param color='#fdfdfc' --param cell=42 --param weight=0.07 --param angle=90
pixelox render js scripts/particles.js -o loose.png --size 1080x1620 --seed 5 --param shape=rect --param colors='#825742,#b88e71,#472b20,#140e0b' --param count=420 --param min=0.7 --param max=1.3 --param angle=-90 --param jitter=4 --param focus='0.5,0.6,0.5'
pixelox new -o mosaic.pxo --size 1080x1620 --fill '#fdfdfc'
pixelox comp place mosaic.pxo loose.png --in-place --as Loose --fit cover
pixelox shape rect mosaic.pxo --in-place --name TopClear --rect 0,0,100%x17% --fill '#fdfdfc'
pixelox comp place mosaic.pxo subject.png --in-place --as Portrait --size x1500 --at 50%,10% --anchor t
pixelox adjust levels mosaic.pxo --in-place --layer Portrait --in-black 20 --in-white 225
pixelox adjust gradient-map mosaic.pxo --in-place --layer Portrait --stops '#140e0b@0,#472b20@0.28,#825742@0.52,#b88e71@0.75,#e2d4c2@1'
pixelox filter pixelate mosaic.pxo --in-place --layer Portrait --cell 14
pixelox comp place mosaic.pxo gaps-v.png --in-place --as GapsV --fit cover
pixelox layer clip mosaic.pxo --in-place --layer GapsV
pixelox comp place mosaic.pxo gaps-h.png --in-place --as GapsH --fit cover
pixelox layer clip mosaic.pxo --in-place --layer GapsH
pixelox text add mosaic.pxo --in-place --name Star --content "LEAD ACTOR NAME" --font "Big Caslon" --weight 500 --size 2.8% --tracking 120 --color '#4a4441' --at 50%,3.7% --anchor c
pixelox text add mosaic.pxo --in-place --name Word1 --content "DUKE" --font "Big Caslon" --weight 500 --size 9% --color '#2a2624' --at 49.5%,9.5% --anchor r
pixelox text add mosaic.pxo --in-place --name Small --content "OF" --font "Big Caslon" --weight 500 --size 3% --color '#2a2624' --at 54%,9.5% --anchor c
pixelox text add mosaic.pxo --in-place --name Word2 --content "ASH" --font "Big Caslon" --weight 500 --size 9% --color '#2a2624' --at 58.5%,9.5% --anchor l
pixelox shape line mosaic.pxo --in-place --name Hairline --from 20%,14.4% --to 80%,14.4% --stroke '#2a2624' --stroke-width 2 --opacity 50
pixelox text add mosaic.pxo --in-place --name Tagline --content "EVERY EMPIRE KEEPS A PRICE LIST" --font "Big Caslon" --weight 500 --size 1% --tracking 150 --color '#6b6460' --at 50%,15.4% --anchor c
pixelox export mosaic.pxo -o mosaic.png
```

Change first: the cutout; the cell size (14 px here; a face needs at least 30 tiles
across, so scale the cutout until the face is 420 px wide or more); then the five ramp colors to match your object's material.

## 10. Variations and pairings

* Material ramps: gold coins (`#3a2a05`, `#8a6a12`, `#d4a827`, `#f3dc8a`, `#fff7d6`);
  banknote green; newsprint gray with black type tiles; rainbow candy for a comedy.
* Hex tiles (`filter pixelate --kind hex`) for honeycomb, scales, or bolts; skip the
  stripe gaps.
* Line-screen version: `filter halftone --shape line --angle 90` in one ink for an
  engraved-banknote portrait.
* Magazine cover or annual-report cover: masthead in the top block, three short cover
  lines at 6% W left of the shoulders.
* Pairs with T-halftone-portrait, T-stencil-poster (posterize before tiling), and
  T-frame-border.

## 11. Pitfalls

* Tiles too large: under 30 tiles across the face it stops being a person. Too small
  (under 8 px): the texture is invisible and the idea is lost.
* A three-quarter or tilted pose: the grid fights the face. Frontal and level.
* A gray or tinted ground: the dissolving edge turns muddy. Pure white (or pure black
  with an inverted ramp).
* Mapping to a ramp that does not belong to the object (blue bullets): the material
  joke fails. Sample the ramp from a photo of the real object.
* A large or heavy title: the poster's power is the big silent face. Keep the type block
  in the top 16% and in a regular-weight classical serif.
