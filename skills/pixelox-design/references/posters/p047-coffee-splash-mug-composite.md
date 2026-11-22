---
id: p047
title: Coffee splash mug composite
genre: [drink-ad, product-ad, social-post]
mood: [warm, energetic, dramatic, bold]
style: [photo-composite]
format: portrait-4x5
archetype: A-hero-object
palette: ['#e8968c', '#571f06', '#883615', '#cc4421', '#230a02', '#f6e3c5']
techniques: [T-cutout-drop-shadow, T-grunge, T-label-box, T-gradient-type, T-water-splash, T-floor-type, T-photo-fade]
needs: product-cutout
---

# p047: Coffee splash mug composite

## 1. Concept

An instant-coffee ad built as a frozen moment: a tilted red mug hangs in the middle while
coffee and milk pour into it from above, giant beans burst into powder beside it, amber
drops fall, and a floor of ground coffee and beans fills the bottom. The brand name
appears three times at three depths: black in the top-right corner, white on the mug, and
"written in the powder" in glowing amber on the floor. Everything is warm; the only
contrast is the cool pink-gray air behind.

## 2. Format and grid

* Portrait 4:5. No strict grid: a diagonal runs from the top-left (milk stream) through the
  mug to the bottom-right (bean pile). The mug sits a little left of center and leans 12
  degrees counter-clockwise.
* Zones: air 0 to 58% H (pale gray at the very top, salmon pink below); soft horizon 55 to
  62% H; coffee floor 60 to 100% H.
* Mug: `12.5%,30%,61.5% x 46%`, handle to the right. It straddles the horizon.
* Brand block: top-right corner, 73 to 98% W, 3 to 20% H.
* Floor lettering: 17 to 83% W, 88 to 98% H.
* Falling things (streams, drops) are vertical or near-vertical; everything else is tilted.

## 3. Layer stack

1. Air: vertical gradient, pale lilac-gray `#b9b5bd` at the top to salmon `#e8968c` by
   25% H and soft pink `#eea39e` at 50% H.
2. Floor: dark roasted brown, `#883615` at the horizon to `#230a02` at the lower left,
   with a mottled, powdery texture; its top edge is blurred into the air over 7% H.
3. Floor beans: 25 to 30 roasted beans lying on the floor, 5 to 18% W each, sharp in the
   front (80 to 95% H) and blurred toward the horizon.
4. Coffee pour: a thick dark-brown `#330700` ribbon entering at the top edge, 30 to 58% W,
   bending left and narrowing into the mug at 34% W, 33% H.
5. Milk stream: a cream `#f6e3c5` beam from the top-left corner (2 to 18% W) running
   diagonally to the mug at 28% W, 35% H; it turns to brown powder where it lands.
6. Splash: a crown of coffee thrown up and right from the mug's mouth, 30 to 53% W, 13 to
   36% H, glossy, with flying droplets.
7. Mug: glossy red `#cc4421` to `#e61901`, white brand wordmark across the body, tilted so
   its rim rises to the right; dark coffee visible inside.
8. Flying beans: one huge bean cut by the left edge (0 to 13% W, 21 to 35% H), one huge
   bean at 59 to 77% W, 19 to 32% H whose right end crumbles into powder, and two small
   ones near the pour.
9. Drops: 9 or 10 amber teardrops `#c8501a` with a small round droplet under each, falling
   in two columns left and right of the mug (8 to 22% W and 67 to 82% W, 38 to 90% H).
10. Brand, top right: a black heavy wordmark with an accent stroke over its last letter;
    under it a 3-word line in black where the middle word is white on a red leaf-shaped
    swash `#f61c19`.
11. Floor lettering: the brand name in italic lowercase, amber `#dd8b1a`, skewed and
    flattened to lie on the floor, glowing as if lit from inside; a bean sits on its first
    letter.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#e8968c` | salmon air | 31% |
| `#571f06` | floor mid brown, beans | 20% |
| `#883615` | lighter floor, bean highlights | 17% |
| `#cc4421` | red mug, swash | 14% |
| `#230a02` | pour, deep shadow, black type | 13% |
| `#f6e3c5` | milk, powder highlights, mug wordmark | 5% |

An analogous warm scheme (cream, salmon, red, brown, near-black): one slice of the color
wheel from light to dark. Amber `#dd8b1a` is the accent, used only for drops and the floor
lettering.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Brand, corner | `[BRAND]`, 7 letters | caps (small-cap style) | heavy humanist sans (Alegreya Sans 900, Cabin 700 / Gill Sans) | 700 | 3.7% | 0 | - | right | `#140a06` | right edge 97.7%, 5.8% |
| Tagline | 3 short words, imperative; middle word highlighted | title | humanist sans (DejaVu Sans, Open Sans 600 / Verdana) | 400 | 2.1% | 0 | - | right | `#140a06`, middle word `#ffffff` | 73 to 98% W, 12.8% |
| Brand on mug | `[BRAND]` (printed on the product) | caps | as corner brand | 700 | 5% | 0 | - | follows the mug's tilt | `#f6e3c5` | 25 to 56% W, 47 to 58% |
| Floor lettering | `[BRAND]`, 7 letters | lower, italic | wide italic sans (Montserrat 700 Italic / Avenir Next) | 700 italic | 6% after flattening | 0 | - | center | `#dd8b1a` | 17 to 83% W, 88 to 98% |

Type is minimal and all of it is the brand. The tagline's highlighted middle word is a
T-label-box move with an organic shape instead of a rectangle.

## 6. Imagery

A composite of 15 to 20 cutouts in one warm light from the upper left: mug, liquid pour,
milk, splash crown, beans in three sizes, powder bursts, drops. Depth comes from blur: the
front beans are sharp, the far beans and the horizon are soft. The mug's tilt and the
splash say "caught mid-air". For a build, the mug is your product cutout and the liquid,
powder, and beans are generated shapes (below) or your own stock cutouts.

## 7. Effects and textures

* Air: `fill gradient`, three stops.
* Floor: one `fill noise --as-layer` line in three browns (`--scale 9%`) with a gradient
  mask that fades its top edge into the air (**T-photo-fade**: `mask add --gradient linear
  --from 0,67% --to 0,56%`), plus **T-grunge** specks in cream at low opacity as powder.
* Beans: brown dots from `particles.js`, rendered square and placed squashed to 40% of
  their height so circles become ovals lying in perspective; `fx bevel` (size 12) gives
  each one a lit top and a dark underside, and a short drop shadow seats them.
* **T-water-splash**: `render splash --kind sheet --liquid coffee --color '#4a1c08'`, thrown
  up and to the right from the mug's mouth (`--angle 72 --spread 110`) and drawn before the
  mug, so the mug covers its root. A `shape path` ribbon is the pour and a cream path the
  milk.
* Drops: amber dots rendered on a canvas twice as wide as the design and placed squeezed
  to half width, so each dot becomes a tall oval; `fx bevel` adds the glassy highlight.
* **T-cutout-drop-shadow** soft variant under the mug (a blurred dark ellipse on the floor).
* **T-label-box**, organic: a red leaf drawn with `shape path` behind one word.
* Floor lettering: italic text sized with `--fit-width 650 --scale-x 94%`, then `transform
  skew --x -28`, which leaves it 720 px wide and 130 px tall (18% of its width); then a
  **T-gradient-type** amber overlay and an outer glow in `screen` mode. The skew turns the
  text into pixels (warning PXW2003): set the word first.
* **T-floor-type** in [techniques.md](../techniques.md) is the tested library version of the type lying in perspective on the floor; start from it when reusing this idea.

## 8. Hierarchy and eye path

1. The red mug and its white wordmark (largest saturated shape, center).
2. Up the splash to the pour, across to the big bean on the right.
3. The black brand block in the top-right corner.
4. Down the falling drops to the amber lettering on the floor.

Cap heights: mug wordmark 5 : floor lettering 6 (low contrast) : corner brand 3.7 :
tagline 2.1. The mug is 46% H tall, nine times its own wordmark.

## 9. Build recipe

`product.png` is your image here: a mug, jar, or cup cutout. Replace the drawn beans with
real bean cutouts when you have them.

```bash
pixelox render js scripts/particles.js -o beans.png --size 1080x1080 --seed 7 --param shape=dot --param colors='#5a2c12,#7a431f,#4a2410,#8a5632' --param count=40 --param min=3 --param max=7 --param amin=1 --param amax=1 --param grow=0.9
pixelox render js scripts/particles.js -o drops.png --size 2160x1350 --seed 11 --param shape=dot --param colors='#c8501a,#e88a2a,#a83c10' --param count=14 --param min=1.6 --param max=3.4 --param amin=0.9 --param amax=1
pixelox render js scripts/grunge.js -o powder.png --size 1080x1350 --seed 5 --param color='#e8c9a0' --param specks=6000 --param blotches=30 --param scratches=0 --param clump=0.6
pixelox new -o ad.pxo --size 1080x1350 --fill '#e8968c'
pixelox layer add ad.pxo --in-place --name Air
pixelox fill gradient ad.pxo --in-place --layer Air --from 0,0 --to 0,58% --stops '#b9b5bd@0,#e8968c@0.45,#eea39e@1'
pixelox fill noise ad.pxo --in-place --as-layer --name Floor --seed 12 --scale 9% --warp 20 --stops '#230a02,#571f06,#a04a18'
pixelox mask add ad.pxo --in-place --layer Floor --gradient linear --from 0,67% --to 0,56%
pixelox comp place ad.pxo beans.png --in-place --as FloorBeans --size 1080x430 --at 50%,100% --anchor b
pixelox fx bevel ad.pxo --in-place --layer FloorBeans --size 12 --depth 160 --soften 4 --highlight '#d9a070' --shadow '#1a0702'
pixelox fx drop-shadow ad.pxo --in-place --layer FloorBeans --offset 0,8 --blur 14 --color '#120501' --opacity 60
pixelox comp place ad.pxo powder.png --in-place --as Powder --fit cover
pixelox layer opacity ad.pxo --in-place --layer Powder --value 30
pixelox shape path ad.pxo --in-place --name Milk --d 'M 20 0 L 195 0 L 330 470 L 292 486 Z' --fill '#f6e3c5' --opacity 92
pixelox shape path ad.pxo --in-place --name Pour --d 'M 330 0 L 632 0 C 560 130 470 250 442 430 L 336 446 C 366 300 352 130 330 0 Z' --fill '#330700'
pixelox fx overlay ad.pxo --in-place --layer Pour --stops '#1a0500@0,#6b2a10@0.5,#2a0c03@1' --angle 0
pixelox render splash ad.pxo --in-place --name Crown --kind sheet --liquid coffee --color '#4a1c08' --at 38%,34.5% --angle 72 --spread 110 --size 44% --seed 4
pixelox shape ellipse ad.pxo --in-place --name MugShadow --rect 22%,72%,50%x6% --fill '#1a0702'
pixelox filter gaussian ad.pxo --in-place --layer MugShadow --sigma 24
pixelox comp place ad.pxo product.png --in-place --as Mug --size x660 --at 43%,53% --anchor c
pixelox transform rotate ad.pxo --in-place --layer Mug --degrees -12
pixelox shape ellipse ad.pxo --in-place --name BeanLeft --rect 0-60,285,200x250 --fill '#4a2410'
pixelox fx overlay ad.pxo --in-place --layer BeanLeft --stops '#2a1206@0,#8a5632@1' --angle 60
pixelox shape ellipse ad.pxo --in-place --name BeanRight --rect 640,262,210x150 --fill '#4a2410'
pixelox fx overlay ad.pxo --in-place --layer BeanRight --stops '#2a1206@0,#a0683c@1' --angle 60
pixelox shape path ad.pxo --in-place --name BeanCrease --d 'M 652 322 C 700 300 770 350 842 318' --fill none --stroke '#1a0905' --stroke-width 9 --cap round
pixelox comp place ad.pxo drops.png --in-place --as Drops --size 1080x1350 --at 0,0
pixelox fx bevel ad.pxo --in-place --layer Drops --size 8 --depth 140 --soften 3 --highlight '#ffd9a0' --shadow '#5a1c04'
pixelox text add ad.pxo --in-place --name Brand --content "BRANDED" --font "Gill Sans" --weight 700 --size 5.2% --color '#140a06' --at 97.7%,5.8% --anchor r
pixelox shape path ad.pxo --in-place --name Leaf --d 'M 962 136 C 968 196 912 262 824 266 C 872 236 884 180 884 152 C 908 150 940 146 962 136 Z' --fill '#f61c19'
pixelox text add ad.pxo --in-place --name TagLeft --content "Brew" --font Verdana --size 2.9% --color '#140a06' --at 80.5%,12.8% --anchor r
pixelox text add ad.pxo --in-place --name TagMid --content "Your" --font Verdana --size 2.9% --color '#ffffff' --at 85.3%,12.8% --anchor c
pixelox text add ad.pxo --in-place --name TagRight --content "Day" --font Verdana --size 2.9% --color '#140a06' --at 90%,12.8% --anchor l
pixelox text add ad.pxo --in-place --name FloorWord --content "Branded" --font @geometric-sans --weight 700 --style italic --fit-width 650 --scale-x 94% --color '#dd8b1a' --at 50%,92.5% --anchor c --anchor-to ink
pixelox transform skew ad.pxo --in-place --layer FloorWord --x -28
pixelox fx overlay ad.pxo --in-place --layer FloorWord --stops '#b85a10@0,#f0a030@0.6,#ffc860@1' --angle 90
pixelox fx outer-glow ad.pxo --in-place --layer FloorWord --blur 26 --color '#ff9a2a' --opacity 40
pixelox export ad.pxo -o ad.png
```

Change first: the mug size and tilt so its mouth sits under the end of the `Pour` path
(x 336 to 442, y 430 to 446), with the splash's `--at` on the mouth; then the three brand
strings, then the air colors. With a wide mug, a crown also works: drop `--kind sheet`
and the mug hides the pool, leaving a ring of spikes above the rim.

## 10. Variations and pairings

* Chocolate drink: swap salmon for pale caramel `#e8c9a0`, the drops for milk white. Tea:
  air `#cfe3c0`, floor of dry leaves in `#3d4a1c`, amber drops stay.
* Juice or soda: replace the floor with T-waves in the drink's color and the beans with
  fruit slices; keep pour, crown, and falling drops.
* Calm version: untilt the mug, remove the milk and the flying beans, keep one pour and the
  floor lettering.
* Square post: crop to 8 to 88% H; move the brand block inside the top-right at 6% margins.
* Pairs with T-light-leak from the top-left (cream and orange) and T-vignette in dark
  brown.

## 11. Pitfalls

* Every element sharp: the scene goes flat and noisy. Blur the far third of the floor and
  the horizon (the gradient mask on `Floor`), keep only the mug and two or three front
  items sharp.
* An upright mug with a pour that misses it. The stream must end inside the mouth; tilt the
  mug toward the stream, not away from it.
* Cool or neutral colors in the cutouts (a gray shadow, a blue-white milk). Warm
  everything: cream for milk, red-brown for shadows, and finish with a soft-light wash of
  orange (`#ff8a3a`, 15 to 20%) when the cutouts still look separate.
* The floor lettering set upright in a bright color reads as a caption stuck on top. It
  needs the skew, the squash, the gradient, and something overlapping it (a bean) to lie
  on the floor.
* Three brand marks at full contrast compete. One strong (the mug), one crisp but small
  (the corner), one atmospheric (the floor).
