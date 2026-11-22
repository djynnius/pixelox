---
id: p048
title: Dark phone launch with halo ring
genre: [tech-ad, product-ad]
mood: [dark, luxurious, minimal, futuristic]
style: [3d-render, minimal, photo-composite]
format: portrait-9x16
archetype: A-hero-object
palette: ['#050609', '#1a1c1f', '#3e4247', '#84888e', '#ffffff']
techniques: [T-neon-glow, T-fit-width, T-rules, T-photo-fade, T-paper-grain, T-ring-arcs, T-floor-reflection]
needs: product-cutout
---

# p048: Dark phone launch with halo ring

## 1. Concept

A flagship-phone launch poster in pure graphite monochrome: the phone stands upright on a
dark rocky landscape with a thin glowing ring behind it like an eclipse, a thin wide-spaced
product name above, and one row of three specs below. It works because there is no color
at all, so the only bright things are the hairline ring and the type; the phone itself is
dark and is drawn by the light of the ring behind it. Presented as a backlit panel in a
dark wall frame; the frame is not part of the design.

## 2. Format and grid

* Tall portrait, about 3:5 (1 : 1.68) in the original; build it on the 9:16 canvas.
  Everything hangs on the center axis. Type margins 8% W left and right.
* Zones: name block 5 to 21% H; hero 25 to 80% H (ring 27 to 70% H, phone 25 to 80% H,
  landscape 55 to 80% H); spec row 84.5 to 88.5% H; footer lines 91.5 to 96% H.
* The spec row is a three-column grid split by hairlines at 33% and 67% W. Each column is
  icon plus text, left-aligned inside the column.
* A band of clear black (21 to 25% H) separates the name from the phone, and another
  (80 to 84% H) separates the phone from the specs.

## 3. Layer stack

1. Ground: blue-black `#050609`, full canvas.
2. Halo disc: a circle, center 51% W, 48.8% H, radius 36% W, filled with smoky dark gray
   (`#3a3e44` at the top to `#50555c` at the bottom), so the inside of the ring is lighter
   than the outside.
3. Halo ring: the same circle as a 0.4% W white hairline with a soft cool glow (blur 3%
   W), brightest at the upper left and upper right.
4. Landscape: dark rock dunes in four gray tones (`#222428` to `#090a0c`) rising at the
   left (0 to 33% W, top at 57% H) and the right (67 to 100% W, top at 54% H), low in the
   middle. It hides the bottom of the ring.
5. Floor fade: the ground color from transparent at 76% H to solid at 85% H, which makes
   a calm black area for the specs.
6. Phone: a graphite phone seen from the back, three-quarter view, upright,
   `34%,25%,33% x 55%`, bottom edge on the floor at 80% H, with a faint reflection below.
7. Brand line: 4 letters, white, thin, cap height 2.8% H, 18% W wide, cap top at 5.3% H.
8. Model name: 3 letters, white, thin, very wide tracking, cap height 6.6% H, 45% W wide,
   cap top at 10.2% H.
9. Tagline: 2 words, white caps, cap height 1.4% H, 41% W wide, centered at 20.1% H.
10. Spec row: three white line icons (5.4% W) at 10%, 40%, and 75% W, centered at
    86.5% H; beside each, two bold caps lines (cap height 0.85% H) and one tiny line
    (0.5% H); hairline dividers at 33% and 67% W, 84.5 to 88.5% H, 35% opacity.
11. Memory line: two short spec phrases split by a short vertical hairline, centered at
    92% H; a horizontal hairline from 24 to 76% W at 93.6% H.
12. Web-address-style line: tracked caps, centered at 95.5% H.
13. Fine grain over everything, soft-light, very low strength.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#050609` | ground, floor | 45% |
| `#1a1c1f` | landscape shadows, phone shadow side | 25% |
| `#3e4247` | halo disc, phone body | 20% |
| `#84888e` | phone highlights, rock highlights | 7% |
| `#ffffff` | ring, type, icons | 3% |

Monochrome with a 2% blue cast in the darks. White covers about 3% of the area, which is
why it glows.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Brand line | `[BRAND]`, 4 letters | caps | thin wide geometric sans (Montserrat 300, Jost 300 / Avenir 300) | 300 | 2.8% | 150 | - | center | `#ffffff` | 50%, cap top 5.3% |
| Model name | `[MODEL]`, 3 letters | caps | same | 300 | 6.6% | 400 | - | center | `#ffffff` | 50%, cap top 10.2% |
| Tagline | 2 words, 6 to 8 letters each | caps | same | 400 | 1.4% | 350 | - | center | `#ffffff` | 50%, 20.1% |
| Spec title (x3) | value + component, 2 lines of 5 to 11 characters | caps | geometric sans (Montserrat 600 / Avenir Next 600) | 600 | 0.85% | 40 | 130% | left | `#ffffff` | 15%, 45%, 80% W; lines at 85.1% and 86.7% |
| Spec note (x3) | 3 words | caps | same | 500 | 0.5% | 60 | - | left | `#b9bcc2` | same x, 88.3% |
| Memory line | "12GB RAM" + "256GB STORAGE" pattern | caps | same | 500 | 0.9% | 150 | - | center | `#ffffff` | 50%, 92% |
| Web line | `[BRAND].[MODEL]`, 8 characters | caps | thin wide geometric sans | 400 | 1% | 500 | - | center | `#ffffff` | 50%, 95.5% |

One geometric family in three weights. The top block uses the light weight at large sizes
with wide tracking; the bottom block uses a semibold at very small sizes. The model name's
tracking (400) is wider than the brand line's (150), so three letters span 45% W.

## 6. Imagery

A studio render of a phone from the back: matte graphite body, a raised square camera
island with three lenses at the upper left, polished side rail catching a thin highlight
on the right. It stands exactly upright, turned a few degrees so the right edge shows.
Light comes from behind (the ring) and from the upper left, so the face of the phone is a
smooth dark-to-mid gray gradient. The landscape is a dark, wet, rocky terrain in the same
gray, sharp at the sides and dissolving into black toward the bottom (**T-photo-fade**
with the ground color). For a build, any product cutout that stands upright works (phone,
bottle, speaker, watch on a stand); the recipe turns it black and white and darkens it so
nothing in the picture has a hue.

## 7. Effects and textures

* Halo: a filled `shape ellipse` with a vertical `fx overlay` gradient, then the same
  ellipse as a 4 px white stroke with `fx outer-glow` (blur 40, spread 10, opacity 90, cool white
  `#dfe6f5`), the tube-and-glow part of **T-neon-glow** without the blurred halo copy.
* Landscape: `scripts/waves.js` with four near-black grays, `chaos=0.9`, blurred by 2 px;
  a real terrain photo or render is better when you have one.
* **T-photo-fade** toward the bottom: ramp 76 to 85% H.
* Reflection: a vertically flipped copy of the product below the floor line at 30%
  opacity, placed under the floor fade so it dies out within 5% H.
* Mono: one `adjust bw --as-layer` above the product and below the type removes every
  hue from the product, its reflection, and the landscape at once.
* **T-rules**: two vertical column dividers, one short divider in the memory line, one
  horizontal hairline, all white at 35 to 60% opacity.
* **T-paper-grain**: amount 6 in soft-light at full opacity, to stop banding in the dark
  gradients.
* **T-ring-arcs** in [techniques.md](../techniques.md) is the tested library version of the ring, halo, or arc shapes; start from it when reusing this idea.
* **T-floor-reflection** in [techniques.md](../techniques.md) is the tested library version of the mirrored floor reflection; start from it when reusing this idea.
* **T-fit-width**: headline lines are set to exact widths with `transform scale` in the recipe.

## 8. Hierarchy and eye path

1. The ring and the phone inside it: the only large shape, and the only light source.
2. Up to the model name (largest type, cap height 6.6% H), then the brand line above it
   and the tagline below.
3. Down the phone to the spec row, read left to right.
4. Memory line and web line last.

Cap-height ratio: model 6.6 : brand 2.8 : tagline 1.4 : spec 0.85, about 7.8 : 3.3 : 1.6 :
1. Wide tracking, not weight, makes the top block feel large.

## 9. Build recipe

`product.png` is your image here: an upright product cutout, tall and narrow. A phone is
about half as wide as it is tall; keep `--size x1040` (54% of the canvas height).

```bash
pixelox render js scripts/waves.js -o terrain.png --size 1080x1920 --seed 5 --param colors='#222428,#191b1e,#111214,#090a0c' --param count=4 --param top=0.58 --param bottom=0.75 --param amp=0.035 --param freq=0.9 --param chaos=0.9
pixelox new -o launch.pxo --size 1080x1920 --fill '#050609'
pixelox shape ellipse launch.pxo --in-place --name Disc --rect 159,545,784x784 --fill '#33373d'
pixelox fx overlay launch.pxo --in-place --layer Disc --stops '#4a4f57@0,#26292e@1' --angle 90
pixelox shape ellipse launch.pxo --in-place --name Ring --rect 159,545,784x784 --fill none --stroke '#ffffff' --stroke-width 4
pixelox fx outer-glow launch.pxo --in-place --layer Ring --blur 40 --spread 10 --color '#dfe6f5' --opacity 90
pixelox comp place launch.pxo terrain.png --in-place --as Terrain --fit cover
pixelox filter gaussian launch.pxo --in-place --layer Terrain --sigma 2
pixelox comp place launch.pxo product.png --in-place --as Mirror --size x1040 --at 50.5%,80% --anchor t
pixelox transform flip launch.pxo --in-place --layer Mirror --v
pixelox layer opacity launch.pxo --in-place --layer Mirror --value 30
pixelox layer add launch.pxo --in-place --name FloorFade
pixelox fill gradient launch.pxo --in-place --layer FloorFade --from 0,76% --to 0,85% --stops '#05060900@0,#050609ff@1'
pixelox comp place launch.pxo product.png --in-place --as Product --size x1040 --at 50.5%,80% --anchor b
pixelox adjust levels launch.pxo --in-place --layer Product --gamma 0.8 --out-white 190
pixelox adjust bw launch.pxo --in-place --as-layer --name Mono
pixelox text add launch.pxo --in-place --name Brand --content "NOVA" --font Avenir --weight 300 --size 3.9% --tracking 150 --color '#ffffff'
pixelox transform move launch.pxo --in-place --layer Brand --to 50%,5.3% --anchor t
pixelox text add launch.pxo --in-place --name Model --content "ARC" --font Avenir --weight 300 --size 9% --tracking 400 --color '#ffffff'
pixelox transform scale launch.pxo --in-place --layer Model --size 500x
pixelox transform move launch.pxo --in-place --layer Model --to 50%,10.2% --anchor t
pixelox text add launch.pxo --in-place --name Tagline --content "QUIETLY BRILLIANT" --font Avenir --weight 400 --size 1.9% --tracking 350 --color '#ffffff' --at 50.8%,20.1% --anchor c
pixelox shape ellipse launch.pxo --in-place --name IconLens --rect 83,1633,56x56 --fill none --stroke '#ffffff' --stroke-width 3
pixelox shape ellipse launch.pxo --in-place --name IconLensCore --rect 100,1650,22x22 --fill none --stroke '#ffffff' --stroke-width 3
pixelox shape rect launch.pxo --in-place --name IconChip --rect 405,1634,54x54 --radius 8 --fill none --stroke '#ffffff' --stroke-width 3
pixelox shape rect launch.pxo --in-place --name IconChipCore --rect 421,1650,22x22 --fill '#ffffff'
pixelox shape rect launch.pxo --in-place --name IconCell --rect 797,1636,32x54 --radius 5 --fill none --stroke '#ffffff' --stroke-width 3
pixelox shape rect launch.pxo --in-place --name IconCellFill --rect 805,1660,16x22 --fill '#ffffff'
pixelox text add launch.pxo --in-place --name Spec1 --content $'64MP\nNIGHT CAMERA' --font "Avenir Next" --weight 600 --size 1.2% --tracking 40 --leading 130% --color '#ffffff' --at 15%,85.9% --anchor l
pixelox text add launch.pxo --in-place --name Spec2 --content $'OCTA-CORE\n3NM CHIP' --font "Avenir Next" --weight 600 --size 1.2% --tracking 40 --leading 130% --color '#ffffff' --at 45%,85.9% --anchor l
pixelox text add launch.pxo --in-place --name Spec3 --content $'5200mAh\nBATTERY' --font "Avenir Next" --weight 600 --size 1.2% --tracking 40 --leading 130% --color '#ffffff' --at 80%,85.9% --anchor l
pixelox text add launch.pxo --in-place --name Note1 --content "SEE IN THE DARK" --font "Avenir Next" --weight 500 --size 0.7% --tracking 60 --color '#b9bcc2' --at 15%,88.3% --anchor l
pixelox text add launch.pxo --in-place --name Note2 --content "SPEED TO SPARE" --font "Avenir Next" --weight 500 --size 0.7% --tracking 60 --color '#b9bcc2' --at 45%,88.3% --anchor l
pixelox text add launch.pxo --in-place --name Note3 --content "TWO-DAY POWER" --font "Avenir Next" --weight 500 --size 0.7% --tracking 60 --color '#b9bcc2' --at 80%,88.3% --anchor l
pixelox shape line launch.pxo --in-place --name Div1 --from 33%,84.5% --to 33%,88.5% --stroke '#ffffff' --stroke-width 1 --opacity 35
pixelox shape line launch.pxo --in-place --name Div2 --from 67%,84.5% --to 67%,88.5% --stroke '#ffffff' --stroke-width 1 --opacity 35
pixelox text add launch.pxo --in-place --name Memory --content "12GB RAM      256GB STORAGE" --font "Avenir Next" --weight 500 --size 1.25% --tracking 150 --color '#ffffff' --at 50%,92% --anchor c
pixelox shape line launch.pxo --in-place --name MemoryDiv --from 45.4%,91.4% --to 45.4%,92.6% --stroke '#ffffff' --stroke-width 2 --opacity 60
pixelox shape line launch.pxo --in-place --name Hairline --from 24%,93.6% --to 76%,93.6% --stroke '#ffffff' --stroke-width 1 --opacity 35
pixelox text add launch.pxo --in-place --name Web --content "NOVA.ARC" --font Avenir --weight 400 --size 1.4% --tracking 500 --color '#ffffff' --at 51%,95.5% --anchor c
pixelox layer add launch.pxo --in-place --name Grain --fill '#808080'
pixelox filter add-noise launch.pxo --in-place --layer Grain --amount 6 --monochrome
pixelox layer blend launch.pxo --in-place --layer Grain --mode soft-light
pixelox export launch.pxo -o launch.png
```

Change first: the brand and model strings (keep the model at 2 to 4 letters so the wide
tracking holds), then the three specs. Move `MemoryDiv` to the gap between the two memory
phrases after you change them. When the canvas is not 1080 px wide, scale the ring
rectangle, the icon rectangles, and the glow blur by `width / 1080`.

## 10. Variations and pairings

* One accent color: tint the ring glow and the three icons (`#5fd0ff` ice blue, `#ff8a2a`
  ember, `#c9a35a` gold) and leave everything else gray. A gold ring with a black watch
  or a perfume bottle reads as luxury.
* Light version: ground `#eceef1`, disc `#ffffff`, ring `#9aa0a8` without glow, black
  type; use a silver or white product.
* 4:5 or square: drop the memory line and the web line, keep three specs, shrink the
  ring to a radius of 30% W.
* Suits any single premium object: headphones, a camera, a car key, a bottle of spirits.
* Pairs with T-smoke (thin cold fog behind the product instead of the landscape),
  T-lens-flare at the ring's upper edge, and T-vignette for extra depth at the corners.

## 11. Pitfalls

* A colorful product photo. One blue reflection or a warm label breaks the monochrome;
  keep the `Mono` adjustment layer and darken the product until its brightest part is
  mid-gray.
* A thick ring. Above 6 px on a 1080 px canvas it reads as a hoop, not as light; keep the
  stroke hairline-thin and let the glow do the work.
* Bold type at the top. The name block must be light (300) with wide tracking; a bold
  model name turns the poster into a discount ad.
* Specs set too large. The spec row is deliberately tiny (cap height under 1% H) and
  needs the clean black floor fade behind it to stay readable.
* The product floating. Its bottom edge sits on the floor line at 80% H and the landscape
  rises on both sides of it; without the flipped reflection or a contact shadow it looks
  pasted on.
