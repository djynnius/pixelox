---
id: p033
title: Ember-orange script fantasy cover
genre: [book-cover]
mood: [dark, mysterious, dramatic, warm]
style: [photo-composite, illustration]
format: portrait-2x3
archetype: A-scene-title
palette: ['#1e2421', '#3f2e27', '#663a2c', '#d3764e', '#e8622f', '#f1ba8e']
techniques: [T-bokeh, T-smoke, T-duotone, T-spotlight, T-neon-glow, T-fit-width, T-vignette, T-paper-grain]
needs: subject-cutout
---

# p033: Ember-orange script fantasy cover

## 1. Concept

A dark-fantasy romance cover: a clawed hand rises from the bottom edge holding a glass
sphere that glows from inside (a filigree butterfly is trapped in it), against a smoky
green-black ground with ember-red bokeh. The title is drawn in the same ember orange, as
two words whose capitals are huge calligraphic swashes while the remaining letters are a
small upright serif, with hairline flourishes looping across the full width. It works
because the picture has one light source and one hue: the glow, the bokeh, the title, and
the author's name are all the same orange, so the type looks lit by the sphere.

## 2. Format and grid

* Portrait 2:3. Centered composition with a slight lean to the right: the sphere's center
  is at 61% W, 50% H.
* Zones: series line at 3% H; title 6 to 42% H (word 1 on 7 to 26% H, word 2 on 28 to 42%
  H, stepping right); sphere 35 to 65% H; hand and wrist 45 to 100% H; author 86 to 92% H.
* Margins for type: 11% W left and right (author line); the flourishes ignore the margins
  and leave the canvas on both sides.
* The title overlaps the top of the sphere by about 5% H; the author's name runs across
  the wrist.

## 3. Layer stack

1. Ground: green-black `#1e2421`, full canvas.
2. Bokeh: large soft discs in ember red and orange (`#8a2414`, `#c8401f`, `#e0683a`),
   6 to 14% W across, 40 to 55% opacity, clustered in the top-left quarter, down the right
   edge, and in the bottom-left corner; the middle stays dark.
3. Smoke: brown-gray haze `#3f2e27` drifting across the lower half, 35%.
4. Hand: a dark, scaled, long-clawed hand entering from the bottom right, fingers wrapped
   around the sphere from below and behind; graded to one warm ramp from near-black through
   rust `#663a2c` to peach `#f1ba8e` on the knuckles that face the light.
5. Sphere: a glass ball 44% W in diameter centered at 61% W, 50% H: a soft orange core
   glow, a thin bright rim, and inside it a pale metal-lace butterfly 30% W wide.
6. Sparks: fine orange dust and a few glassy shards thrown to the left of the sphere,
   10 to 45% W, 38 to 62% H.
7. Vignette: black, 70% at the corners.
8. Series line: tiny tracked serif (roman and italic mixed), orange, centered at 3.1% H.
9. Title word 1: a swash capital 19% H tall (29 to 50% W, 7 to 26% H) followed by 8 small
   serif letters with x-height 4.3% H on a baseline at 20% H, 50 to 93% W.
10. Title word 2: a swash capital 13% H tall at 42 to 52% W, 28 to 41% H, followed by 3
    small serif letters on a baseline at 41% H, 60 to 75% W.
11. Flourishes: two or three hairline curves (2 to 3 px at 1080 px wide) that sweep from
    off the left edge through the capitals, loop once or twice between the two words, and
    exit on the right.
12. Author: light sans caps, orange `#e8622f`, 11 to 90% W, cap height 5.2% H, centered at
    89.5% H.
13. Grain.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#1e2421` | ground, deep shadow | 35% |
| `#3f2e27` | smoke, hand shadows | 30% |
| `#663a2c` | hand mid-tones, dark bokeh | 17% |
| `#d3764e` | bokeh, glow edge | 8% |
| `#e8622f` | title, author | 7% |
| `#f1ba8e` | sphere core, highlights | 3% |

One hue (orange, from rust to peach) on a neutral green-black. The faint green in the
black is the complement of the orange and makes it burn hotter than pure black would.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Series line | series name + "Book 2", 5 words | title | old-style serif, partly italic (Cormorant / Baskerville) | 400 | 1.1% | 150 | - | center | `#d3764e` | 50%, 3.1% |
| Title capitals (x2) | the first letter of each title word | cap | formal script with long swashes (Pinyon Script, Great Vibes / Zapfino) | 400 | 19% and 13% | 0 | - | - | `#e8622f` | 29 to 50%, 7 to 26%; 42 to 52%, 28 to 41% |
| Title word 1, rest | 8 lowercase letters | lower | didone serif (Bodoni Moda, Playfair Display / Bodoni 72) | 400 | x-height 4.3% | 40 | - | left | `#e8622f` | left 50%, baseline 20% |
| Title word 2, rest | 3 lowercase letters | lower | same | 400 | x-height 4.3% | 40 | - | left | `#e8622f` | left 60%, baseline 41% |
| Author | `[AUTHOR NAME]`, 2 words, 10 to 11 letters | caps | light geometric sans (Josefin Sans 300, Jost 300 / Avenir Next) | 400 | 5.2% | 80 | - | center | `#e8622f` | 50%, 89.5% |

Three voices: a swash script for two letters only, a small didone for the rest of the
title, and a light wide sans for the author. The author's name is larger in cap height
than the title's lowercase (5.2 against 4.3): genre convention for a known author.

## 6. Imagery

One composite photograph or painting: a creature's hand (dark skin with a scale texture,
claws 5% H long) cradling a glass sphere. The sphere is the light source, so the hand is
lit from the palm outward and falls to black at the wrist and the canvas edges. For a
build, use any cutout of a hand or figure holding an object, and grade it with
**T-duotone** on a warm ramp (`#120b08` to `#5a2e1c` to `#c9703f` to `#f1ba8e`). Then
paint the light: a radial glow in screen mode at the object (**T-spotlight** logic, with
the glow in front of the subject, not behind), a thin bright ring for the glass, and
sparks drifting away from it.

## 7. Effects and textures

* **T-bokeh**: one layer of large discs, `edge=0.8` so they stay out of the middle,
  `--sigma 10`, screen.
* **T-smoke**: warm haze (`hi='#8a5a44'`), screen, 35%.
* **T-duotone** on the subject (four stops, above).
* Glow: `fill gradient --kind radial` from `#ffd9a0` (opaque) to `#f08a3c` (alpha 0),
  screen; rim: `shape ellipse` with a 3 px stroke and `fx outer-glow`.
* Sparks: `particles.js` with `shape=glow` and `focus` beside the sphere, screen.
* Title glow (**T-neon-glow**, tight glow only): `fx outer-glow --blur 18 --opacity 45` in
  `#ff7a3c` on every title layer, so the letters seem to emit light.
* **T-vignette** (black, strong) and **T-paper-grain** (amount 12, soft-light 40%).
* **T-fit-width**: headline lines are set to exact widths with `transform scale` in the recipe.

## 8. Hierarchy and eye path

1. The glowing sphere (brightest spot, center).
2. Up the flourishes to the two swash capitals and the title.
3. Down the hand to the author's name.
4. The series line last.

Sizes: swash capital 19% H : author caps 5.2% : title lowercase x-height 4.3% : series 1.1%.

## 9. Build recipe

`subject.png` is your image here: a cutout of a hand (or a figure) holding the object that
glows; put the object near 61% W, 50% H and move the Glow and Rim layers onto it. The
flourish paths are drawn for these two invented words; redraw them so each starts at a
capital and loops through the gap between the words.

```bash
pixelox render js scripts/blobs.js -o bokeh.png --size 1080x1620 --seed 4 --param bg=transparent --param colors='#c8401f,#e0683a,#8a2414' --param count=40 --param size=0.1 --param soft=0.3 --param alpha=0.55 --param edge=0.8 --param mode=screen
pixelox render js scripts/clouds.js -o haze.png --size 1080x1620 --seed 3 --param lo=transparent --param hi='#8a5a44' --param scale=2.5 --param warp=1.2 --param stretch=0.7 --param bias=-0.1
pixelox render js scripts/particles.js -o sparks.png --size 1080x1620 --seed 5 --param shape=glow --param colors='#ffd27a,#ff8a3c,#fff1c9' --param count=160 --param min=0.1 --param max=0.6 --param focus='0.34,0.5,0.22'
pixelox new -o cover.pxo --size 1080x1620 --fill '#1e2421'
pixelox comp place cover.pxo bokeh.png --in-place --as Bokeh --fit cover
pixelox filter gaussian cover.pxo --in-place --layer Bokeh --sigma 10
pixelox layer blend cover.pxo --in-place --layer Bokeh --mode screen
pixelox comp place cover.pxo haze.png --in-place --as Haze --fit cover
pixelox layer blend cover.pxo --in-place --layer Haze --mode screen
pixelox layer opacity cover.pxo --in-place --layer Haze --value 35
pixelox comp place cover.pxo subject.png --in-place --as Hand --size x1000 --at 60%,101% --anchor b
pixelox adjust gradient-map cover.pxo --in-place --layer Hand --stops '#120b08@0,#5a2e1c@0.4,#c9703f@0.75,#f1ba8e@1'
pixelox layer add cover.pxo --in-place --name Glow
pixelox fill gradient cover.pxo --in-place --layer Glow --kind radial --from 61%,50% --to 61%,68% --stops '#ffd9a0ff@0,#f08a3cb3@0.4,#f08a3c00@1'
pixelox layer blend cover.pxo --in-place --layer Glow --mode screen
pixelox shape ellipse cover.pxo --in-place --name Rim --rect 421,572,476x476 --fill none --stroke '#ffcf9a' --stroke-width 3 --opacity 70
pixelox fx outer-glow cover.pxo --in-place --layer Rim --blur 24 --color '#ff8a3c' --opacity 70
pixelox comp place cover.pxo sparks.png --in-place --as Sparks --fit cover
pixelox layer blend cover.pxo --in-place --layer Sparks --mode screen
pixelox layer add cover.pxo --in-place --name Vignette
pixelox fill gradient cover.pxo --in-place --layer Vignette --kind radial --from 58%,50% --to 58%,118% --stops '#00000000@0.4,#000000b3@1'
pixelox text add cover.pxo --in-place --name Series --content "The Hollow Court Series  Book 2" --font Baskerville --style italic --size 1.6% --tracking 150 --color '#d3764e' --at 50%,3.1% --anchor c
pixelox shape path cover.pxo --in-place --name Flourish1 --d 'M -20 200 C 160 120 330 250 470 150 S 760 60 1100 170' --fill none --stroke '#e8622f' --stroke-width 3
pixelox shape path cover.pxo --in-place --name Flourish2 --d 'M -20 470 C 150 380 300 560 420 470 S 330 330 300 430 S 520 600 680 520 S 930 430 1100 500' --fill none --stroke '#e8622f' --stroke-width 2.5
pixelox shape path cover.pxo --in-place --name Flourish3 --d 'M 60 330 C 20 250 150 220 180 300 S 60 420 250 400 S 520 250 700 330' --fill none --stroke '#e8622f' --stroke-width 2.5
pixelox text add cover.pxo --in-place --name Cap1 --content "R" --font Zapfino --size 14% --color '#e8622f'
pixelox transform move cover.pxo --in-place --layer Cap1 --to 24%,7% --anchor tl
pixelox text add cover.pxo --in-place --name Rest1 --content "avenous" --font "Bodoni 72" --weight 400 --size 9% --tracking 40 --color '#e8622f'
pixelox transform move cover.pxo --in-place --layer Rest1 --to 50%,20% --anchor bl
pixelox text add cover.pxo --in-place --name Cap2 --content "M" --font Zapfino --size 9% --color '#e8622f'
pixelox transform move cover.pxo --in-place --layer Cap2 --to 30%,29% --anchor tl
pixelox text add cover.pxo --in-place --name Rest2 --content "oon" --font "Bodoni 72" --weight 400 --size 9% --tracking 40 --color '#e8622f'
pixelox transform move cover.pxo --in-place --layer Rest2 --to 56%,41% --anchor bl
pixelox fx outer-glow cover.pxo --in-place --layer Cap1 --blur 18 --color '#ff7a3c' --opacity 45
pixelox fx copy cover.pxo --in-place --from Cap1 --to Rest1,Cap2,Rest2
pixelox text add cover.pxo --in-place --name Author --content "AUTHOR NAME" --font "Avenir Next" --weight 400 --size 7% --tracking 80 --color '#e8622f'
pixelox transform scale cover.pxo --in-place --layer Author --size 850x
pixelox transform move cover.pxo --in-place --layer Author --to 50%,89.5% --anchor c
pixelox layer add cover.pxo --in-place --name Grain --fill '#808080'
pixelox filter add-noise cover.pxo --in-place --layer Grain --amount 12 --monochrome
pixelox layer blend cover.pxo --in-place --layer Grain --mode soft-light
pixelox layer opacity cover.pxo --in-place --layer Grain --value 40
pixelox export cover.pxo -o cover.png
```

Change first: the subject and the glow position; then the two title words (each needs a
capital with a good swash: R, M, A, K, Q, and T work; I and O do not); then the hue.

## 10. Variations and pairings

* Hue swaps (ground stays near-black with a hint of the complement): ice blue `#7fd4ff`
  on `#1c1a22` for a winter court; poison green `#9be36a` on `#221c22`; gold `#e9b44c` on
  `#16181f` for a heist or royal story.
* Other glowing objects: a lantern, a crown, a vial, a pocket watch, a rose under glass.
  Anything held in a hand and lit from inside.
* Event or perfume poster: drop the series line, replace the author with a date line in
  the same light sans.
* A-centered-title version: sphere alone in the middle with no hand, title above, author
  below.
* Pairs with T-lens-flare (a small flare on the rim), T-fire-type (flames on the author
  line for a hotter look), and T-color-grade over the finished stack.

## 11. Pitfalls

* Title in a second hue (white, gold): the picture loses its single light source. Sample
  the title color from the glow.
* A whole word in swash script at this size is unreadable. Script for the capital only;
  the rest in a clear serif with x-height at least 4% H.
* Flourishes as thick as the letters look like spaghetti. Keep them at 2 to 3 px and let
  them leave the canvas.
* Bokeh in the middle of the canvas fights the sphere. Push it to the edges (`edge=0.8`).
* A glow that is white at the center and gray at the edge. Both stops must be warm: pale
  peach at the core, saturated orange at alpha 0 outside.
