---
id: p079
title: Ice headline splash ad with model
genre: [drink-ad, product-ad, social-post]
mood: [cool, energetic, bold, dramatic]
style: [photo-composite, 3d-render]
format: portrait-4x5
archetype: A-text-left-product-right
palette: ['#040819', '#051c54', '#1447a7', '#bdd7f3', '#6596db', '#9d443a']
techniques: [T-spotlight, T-fit-width, T-gradient-type, T-extrude-type, T-cutout-drop-shadow, T-vignette, T-water-splash, T-ice-cubes, T-condensation, T-smoke]
needs: [subject-cutout, product-cutout, logo]
---

# p079: Ice headline splash ad with model

## 1. Concept

A cold-drink ad staged like a sports-drink commercial: a man in a beanie looks up and away
with quiet confidence, a blue can explodes out of water and ice in front of him at the
left, and a three-word headline carved from ice blocks is stacked at the right, tilted
slightly upward. Everything is one electric blue on near-black, so the only warm things
are skin and the red half of the emblem. Person for attitude, can for product, ice type
for the message: three heroes that share the frame by taking a third each.

## 2. Format and grid

* Portrait 4:5. Margins 3% W (the headline almost touches the right edge on purpose).
* Thirds: can in the left third (11 to 37% W, 41 to 91% H); person in the middle (33 to 85%
  W, 4 to 78% H); headline in the right third and overlapping the person's shoulder (58 to
  99% W, 43 to 81% H).
* Logo in the top-right corner (86 to 97% W, 4 to 15% H).
* The headline is right-aligned at 97.5% W and rotated about 4 degrees counter-clockwise,
  so its lines rise to the right; the sub-line under it is level.
* Water arcs diagonally from the lower left, around the can, up to the right edge at 30% H.

## 3. Layer stack

1. Ground: near-black navy `#040819`.
2. Blue light: an electric-blue `#1447a7` pool behind the person and the can, brightest at
   60 to 100% W, 50 to 75% H; darkest in the top-left corner.
3. Mist and bokeh: pale blue droplets out of focus across the upper left and right.
4. Splash: light-blue water `#bdd7f3` thrown up around the can (0 to 45% W, 30 to 95% H)
   and a second arc at the right edge (70 to 100% W, 22 to 45% H); hundreds of fine drops.
5. Ice cubes: 6 to 8 clear cubes, 8 to 13% W each, flying at the upper left (14 to 27% W,
   13 to 23% H) and scattered along the bottom edge.
6. Person: a man from the chest up in a gray knitted beanie and a dark blue zip jacket over
   a navy crew-neck top, short beard, small earring; face turned three-quarters to his right
   (viewer's left), chin lifted. Warm key light from the left, blue rim on the right. His
   lower body is hidden by the splash and the headline.
7. Can: a royal-blue can with a round red-and-blue brand emblem and a lowercase white wordmark,
   `11%,41%,26% x 50%`, upright, frosted with droplets, sitting in the splash.
8. Headline: three lines of heavy capitals that look like blocks of ice: pale blue-white
   faces with a beveled edge, a darker blue extruded side, cracks and frost; line widths
   37%, 27%, and 40% W.
9. Sub-line: two short sentences in white condensed italic capitals under the headline,
   right-aligned, 60 to 97.5% W at 84% H.
10. Logo: round emblem 10% W across with a lowercase wordmark under it, top right.
11. Dark vignette in the corners.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#040819` | near-black navy ground | 28% |
| `#051c54` | deep blue shadows, jacket | 28% |
| `#1447a7` | electric blue light, can | 17% |
| `#bdd7f3` | ice, splash, headline faces | 12% |
| `#6596db` | mid blue of water and the headline's sides | 11% |
| `#9d443a` | skin, red of the emblem | 4% |

A blue monochrome from black to ice-white; skin and the emblem's red are the only warm
notes and both sit on the focal diagonal.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Headline 1 | 1 verb, 7 letters | caps | heavy condensed grotesque (Anton, Bebas Neue / Impact) | 400 (the face is heavy) | 10.5% | 0 | - | right | ice gradient `#7fb4e8` to `#ffffff` | right edge 97%, top 43% |
| Headline 2 | 1 word, 4 letters | caps | same | same | 10.5% | 0 | - | right | same | right edge 97%, top 55.7% |
| Headline 3 | 1 noun, 5 letters | caps | same | same | 12% | 0 | - | right | same | right edge 99%, top 67.5% |
| Sub-line | two 2-word sentences with periods, about 21 characters | caps | condensed bold italic (Barlow Condensed 700 Italic / Avenir Next Condensed) | 700 italic | 2.3% | 20 | - | right | `#ffffff` | right edge 97.5%, 84% |
| Logo wordmark | `[BRAND]`, lowercase | lower | geometric sans (Poppins 500 / Avenir Next) | 500 | 1.5% (x-height) | 0 | - | center | `#ffffff` | 91.5%, 13.8% |

One heavy condensed face for the headline and its italic condensed cousin for the
sub-line. The three headline lines are different widths: a ragged-left, flush-right stack.

## 6. Imagery

* Person: a studio portrait cutout with strong two-color lighting (warm front, blue rim).
  The upward three-quarter look matters: he looks out of the frame, away from the product,
  which reads as attitude instead of a sales pitch.
* Can: a clean pack shot with heavy condensation, placed in front of the person at the
  left, overlapping his arm.
* Water and ice: stock splash arcs and cubes, tinted the ground blue. They hide every
  cutout edge at the bottom of the frame.

## 7. Effects and textures

* **T-spotlight** in blue: radial glow `#1a57c8` at 62% W, 60% H on the navy ground.
* Mist: **T-smoke** (`fill noise --kind smoke`, stops `transparent` to `#3f8fe8`, `screen`,
  45%).
* **T-water-splash**: `render splash` tinted `#8cc8ff`. The can stands in a crown: the back
  half is drawn before the can and the front half after it, with the same `--at`, `--size`,
  and `--seed`. The arc at the right edge is a second splash, `--kind sheet`, thrown from
  just outside the canvas (`--at 108%,46%`).
* **T-ice-cubes**: `render ice`, two groups: three small, slightly blurred cubes at the
  upper left before the person, four larger ones along the bottom after the can. `--tint
  '#9fd0ff' --frost 35` keeps them blue and clear on the dark ground.
* **T-condensation**: `render droplets --layer Can`, run right after the can is placed and
  before its glow style (beads drawn after a glow also land on the glow).
* Ice type: **T-fit-width** per line; **T-gradient-type** (`fx overlay` bottom to top
  `#7fb4e8`, `#e6f3ff`, `#ffffff`); `fx bevel` (inner, size 7) for the chiseled edge;
  **T-extrude-type** (`fx extrude --depth 12 --angle 310`, sides `#0d3f9a`) for the solid
  block; `fx stroke` 2 px navy; `fx copy` carries the four styles to the other two lines;
  then each line is rotated -4 degrees.
* **T-cutout-drop-shadow**, glow variant: blue outer glow on the can.
* **T-vignette**, navy, 70% at the corners.

## 8. Hierarchy and eye path

1. The face (warm on blue, upper middle).
2. The headline to its right, read top to bottom.
3. The can at the left, linked by the splash.
4. Sub-line, then the logo top right.

Cap heights: headline 10.5 to 12 : sub-line 2.3 : logo 1.5, about 5 : 1 : 0.65.

## 9. Build recipe

`subject.png` (person cutout), `product.png` (can cutout), and `logo.png` (round emblem)
are your images here.

```bash
pixelox new -o ad.pxo --size 1080x1350 --fill '#040819'
pixelox layer add ad.pxo --in-place --name BlueLight
pixelox fill gradient ad.pxo --in-place --layer BlueLight --kind radial --from 62%,60% --to 62%,125% --stops '#1a57c8ff@0,#0a2a78cc@0.5,#04081900@1'
pixelox fill noise ad.pxo --in-place --as-layer --name Mist --kind smoke --seed 6 --stops 'transparent,#3f8fe8' --blend screen --opacity 45
pixelox render splash ad.pxo --in-place --name SplashRight --kind sheet --color '#8cc8ff' --at 108%,46% --angle 150 --spread 70 --size 55% --seed 9
pixelox render ice ad.pxo --in-place --name IceFar --at 22%,18% --count 3 --size 10% --spread 14% --tint '#9fd0ff' --frost 35 --seed 5
pixelox filter gaussian ad.pxo --in-place --layer IceFar --sigma 2
pixelox comp place ad.pxo subject.png --in-place --as Person --size x1060 --at 56%,82% --anchor b
pixelox layer add ad.pxo --in-place --name LowerShade
pixelox fill gradient ad.pxo --in-place --layer LowerShade --from 0,62% --to 0,100% --stops '#06123a00@0,#06123ae6@1'
pixelox render splash ad.pxo --in-place --name SplashBack --part back --color '#8cc8ff' --at 24.5%,88% --size 62% --seed 6
pixelox comp place ad.pxo product.png --in-place --as Can --size x740 --at 24.5%,93% --anchor b
pixelox render droplets ad.pxo --in-place --layer Can --density 55 --trickles 4
pixelox fx outer-glow ad.pxo --in-place --layer Can --blur 60 --color '#5fb0ff' --opacity 55
pixelox render splash ad.pxo --in-place --name SplashFront --part front --color '#8cc8ff' --at 24.5%,88% --size 62% --seed 6
pixelox render ice ad.pxo --in-place --name IceNear --at 50%,95% --count 4 --size 11% --spread 40% --tint '#9fd0ff' --frost 35 --seed 12
pixelox text add ad.pxo --in-place --name H1 --content "RESTART" --font @condensed-heavy --fit-width 404 --color '#ffffff' --at 97%,43.5% --anchor tr --anchor-to ink
pixelox fx overlay ad.pxo --in-place --layer H1 --stops '#7fb4e8@0,#e6f3ff@0.55,#ffffff@1' --angle 90
pixelox fx bevel ad.pxo --in-place --layer H1 --size 7 --depth 140 --highlight '#ffffff' --shadow '#1a4fa8'
pixelox fx stroke ad.pxo --in-place --layer H1 --width 2 --color '#0a2a6a'
pixelox fx extrude ad.pxo --in-place --layer H1 --depth 12 --angle 310 --color '#0d3f9a'
pixelox text add ad.pxo --in-place --name H2 --content "YOUR" --font @condensed-heavy --fit-width 268 --color '#ffffff' --at 97%,56.5% --anchor tr --anchor-to ink
pixelox text add ad.pxo --in-place --name H3 --content "NIGHT" --font @condensed-heavy --fit-width 372 --color '#ffffff' --at 98%,69% --anchor tr --anchor-to ink
pixelox fx copy ad.pxo --in-place --from H1 --to H2,H3
pixelox transform rotate ad.pxo --in-place --layer H1 --degrees -4
pixelox transform rotate ad.pxo --in-place --layer H2 --degrees -4
pixelox transform rotate ad.pxo --in-place --layer H3 --degrees -4
pixelox text add ad.pxo --in-place --name Subline --content "STAY COLD. STAY LOUD." --font @condensed --weight 700 --style italic --size 3.2% --tracking 20 --color '#ffffff' --at 97.5%,84.5% --anchor r
pixelox comp place ad.pxo logo.png --in-place --as Logo --size 104x --at 91.5%,8% --anchor c
pixelox text add ad.pxo --in-place --name LogoWord --content "brand" --font @geometric-sans --weight 500 --size 2.8% --color '#ffffff' --at 91.5%,13.8% --anchor c
pixelox layer add ad.pxo --in-place --name Vignette
pixelox fill gradient ad.pxo --in-place --layer Vignette --kind radial --from 55%,50% --to 55%,125% --stops '#02040c00@0.5,#02040cb3@1'
pixelox export ad.pxo -o ad.png
```

Change first: the three headline words (shortest in the middle) and their `--fit-width`
values, then the blue (`#1a57c8` and the related tints, including the splash `--color` and
the ice `--tint`), then the person's scale so the face is at 15 to 22% H. Ice and droplets
show what is behind them when their command runs: after swapping the can or the person,
run the recipe again from the top.

## 10. Variations and pairings

* Fire version of the same layout: black ground, T-fire-glow light, headline with a red to
  yellow gradient and a dark red extrusion, embers instead of ice (a spicy flavor, a
  barbecue sauce).
* Gym or sports drink: swap the model for an athlete mid-movement and add T-motion-streak.
* No person: can centered at 55% W tall, headline split above and below it.
* Square: can left, headline right, person removed or reduced to a head-and-shoulders crop
  behind both.
* Pairs with T-lens-flare (`35mm`) on the can's rim and T-bokeh in pale blue.

## 11. Pitfalls

* Flat white headline: it reads as a caption on a photo. The ice look needs all four
  parts: gradient face, bevel, extruded side, thin dark stroke.
* Equal line widths with unequal words force the short word huge. Let the lines differ in
  width and align them flush right.
* Rotating the three lines as separate layers without a common alignment makes them drift
  apart. Position them first (flush right), then rotate each by the same angle.
* Person looking at the camera and smiling turns it into a testimonial. The off-camera
  gaze is part of the cool mood.
* Splash in front of the face or the headline. Water belongs around the can and at the
  frame's edges; keep the face and the type clear.
