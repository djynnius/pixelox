---
id: p077
title: Blue and magenta lifestyle drink ad
genre: [drink-ad, social-post, product-ad]
mood: [playful, energetic, friendly, cool]
style: [photo-composite, gradient-mesh]
format: portrait-3x4
archetype: A-text-left-product-right
palette: ['#062585', '#0a1036', '#315dbc', '#983578', '#c44c52', '#ffffff']
techniques: [T-gradient-mesh, T-cutout-drop-shadow, T-pill-button, T-label-box, T-bokeh, T-photo-fade]
needs: [subject-cutout, product-cutout, logo]
---

# p077: Blue and magenta lifestyle drink ad

## 1. Concept

A social-media drink ad with a person and the product sharing the stage: a young woman
presses a cold bottle to her cheek with her eyes closed, a giant can stands in the front
left corner, and a caption sits in a glowing rounded box at the lower right. The ground is
saturated royal blue with soft magenta light spilling in, the brand's colors turned into
atmosphere. It works by showing the feeling (the person) and the pack (the can) at two
scales in one frame.

## 2. Format and grid

* Portrait, close to 3:4. Margins 5% W. Loose three-column feel: big can at the left
  (7 to 36% W), person in the middle (20 to 86% W), brand mark and caption at the right.
* Zones: slogan line 3 to 6.5% H; person 12 to 80% H; can 50 to 93% H in front of the
  person's arm; logo block 22 to 34% H at 76 to 97% W; caption box 78.5 to 89% H at 38 to
  94% W.
* A diagonal from the bottle at the upper left of the face (28 to 41% W, 11 to 51% H)
  down to the can gives the eye a path: small pack, face, big pack.
* Out-of-focus copies of the brand's round emblem float at 76% W, 74% H and at the edges,
  for depth.

## 3. Layer stack

1. Ground: royal blue `#062585`, lighter `#073cab` at the top-left, near-black navy
   `#0a1036` at the top-right.
2. Color haze: large blurred magenta `#821950` at the left middle, pink `#983578` to pale
   pink at the bottom center, lighter blue at the bottom-left corner.
3. Floating emblems: two or three small round brand emblems, heavily blurred, 14% W and
   8% W across.
4. Person: a young woman from the chest up, hair pulled back, head tilted to her right,
   eyes closed, lips pursed toward the bottle; white collared shirt under a gray knitted
   vest. Head centered at 60% W, 30% H; the torso dissolves into the blue at the bottom.
   Lit with a warm key from the left and a blue rim from the right.
5. Bottle in her hand: a small plastic cola bottle with a blue label, upright, pressed to
   her cheek, 13% W wide, 11 to 51% H.
6. Can: a blue can with the round emblem and a white wordmark on a dark band,
   `7%,50%,29% x 43%`, wet with condensation, standing on a small pile of ice at the bottom
   edge; a soft blue glow around it.
7. Slogan line at the top: the brand name in bold red, then three words in white, on one
   line, centered at 47% W, 4.6% H.
8. Logo block, right: round emblem 8.5% W across at 86% W, 25.5% H; under it a dark navy
   pill `76%,29.8%,21% x 4.4%` with the brand name in bold white, and a soft white glow
   around the pill.
9. Caption box: rounded rectangle `38%,78.5%,56.5% x 10.5%`, corner radius 40% of its
   height, 1.5 px light-blue outline with a blue glow, translucent blue fill; inside, two
   lines of bold white tracked text.
10. A small studio credit in the bottom-right corner.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#062585` | royal blue ground | 35% |
| `#0a1036` | navy shadows, pill | 24% |
| `#315dbc` | lighter blue: can, glows | 17% |
| `#983578` | magenta and pink haze | 10% |
| `#c44c52` | red of the emblem and the brand word | 6% |
| `#ffffff` | type, shirt, highlights | 8% |

Blue dominant with magenta as the neighbor color and red as the accent; white is saved for
type and the shirt.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Slogan: brand word | `[BRAND]`, 5 letters | caps | neo-grotesque (Inter 800, Archivo 800 / Helvetica Neue) | 700 | 2.9% | 0 | - | right of pair | `#e8202a` | ends at 34% W, 4.6% |
| Slogan: words | 3 short words, an attitude line | caps | same | 400 | 2.9% | 20 | - | left of pair | `#ffffff` | starts at 38% W, 4.6% |
| Pill wordmark | `[BRAND]` | caps | same | 700 | 2.3% | 20 | - | center | `#ffffff` | 86.5%, 32% |
| Caption | 2 lines, 5 to 6 words each, title case, a benefit sentence | title | same | 700 | 1.6% | 80 | 135% | left | `#ffffff` | starts 43% W; lines at 82.5% and 85.5% |

One neo-grotesque family in two weights. The two-color slogan line (red brand word, white
words) is the only headline, and it is modest: cap height 2.9% H.

## 6. Imagery

* Person: a cutout with soft edges; the lower body fades into the ground rather than ending
  at a hard crop. The expression (a kiss toward the bottle, eyes closed) is the emotional
  hook; a build needs any warm, unposed moment with the product held near the face.
* Can: a separate, much larger cutout in the front plane, sharper and more saturated than
  the person, overlapping her arm.
* The bottle she holds belongs to the person photo. If your portrait has no product in it,
  place a small product cutout at the hand, as the recipe does.

## 7. Effects and textures

* **T-gradient-mesh**: `blobs.js` with fixed centers (light blue top-left, navy top-right,
  magenta left-middle, pink bottom-center), blurred sigma 60.
* **T-bokeh** idea with a twist: the out-of-focus discs are blurred copies of the logo.
* **T-photo-fade**: a ground-color gradient from 62% H to 88% H over the person's torso,
  then a small pink radial glow on top of it at the bottom center (60% W, 99% H).
* **T-cutout-drop-shadow**, glow variant on the can (`fx outer-glow`, blur 50, `#4f8cff`,
  50%) and a soft dark shadow to its right.
* **T-pill-button** shapes used as labels: a solid navy pill for the wordmark, an outlined,
  glowing rounded box for the caption (**T-label-box** with `--radius`).

## 8. Hierarchy and eye path

1. The face and the bottle against it (warm skin on blue, center).
2. The big can, front left.
3. The slogan line at the top; the logo block at the right.
4. The caption box, bottom right.

Cap heights: slogan 2.9 : pill 2.3 : caption 1.6, about 1.8 : 1.4 : 1. Type is secondary; the
two product sizes (13% W bottle, 29% W can) do the hierarchy.

## 9. Build recipe

`subject.png` (person cutout), `product.png` (pack cutout, used twice), and `logo.png`
(round emblem) are your images here.

```bash
pixelox render js scripts/blobs.js -o haze.png --size 1080x1440 --seed 2 --param bg='#04229a' --param colors='#0b46c8,#8a1f7a,#f06aa8,#000b42,#1059b4' --param at='0.05,0.04;0.08,0.56;0.6,1.02;0.98,0.22;0.08,0.98' --param size=0.55 --param alpha=1
pixelox new -o ad.pxo --size 1080x1440 --fill '#04229a'
pixelox comp place ad.pxo haze.png --in-place --as Haze --fit cover
pixelox filter gaussian ad.pxo --in-place --layer Haze --sigma 60
pixelox comp place ad.pxo logo.png --in-place --as FarEmblem --size 150x --at 76%,74% --anchor c
pixelox filter gaussian ad.pxo --in-place --layer FarEmblem --sigma 7
pixelox layer opacity ad.pxo --in-place --layer FarEmblem --value 80
pixelox comp place ad.pxo subject.png --in-place --as Person --size x1010 --at 60%,84% --anchor b
pixelox layer add ad.pxo --in-place --name TorsoFade
pixelox fill gradient ad.pxo --in-place --layer TorsoFade --from 0,62% --to 0,88% --stops '#06259000@0,#062590ff@1'
pixelox layer add ad.pxo --in-place --name PinkGlow
pixelox fill gradient ad.pxo --in-place --layer PinkGlow --kind radial --from 60%,99% --to 60%,84% --stops '#ff8fc0e6@0,#f06aa800@1'
pixelox comp place ad.pxo product.png --in-place --as Bottle --size x570 --at 34.5%,31% --anchor c
pixelox transform rotate ad.pxo --in-place --layer Bottle --degrees -5
pixelox comp place ad.pxo product.png --in-place --as Can --size x700 --at 21.5%,94% --anchor b
pixelox fx outer-glow ad.pxo --in-place --layer Can --blur 50 --color '#4f8cff' --opacity 50
pixelox fx drop-shadow ad.pxo --in-place --layer Can --offset 18,10 --blur 40 --color '#020a30' --opacity 60
pixelox text add ad.pxo --in-place --name SloganBrand --content "BRAND" --font "Helvetica Neue" --weight 700 --size 4% --color '#e8202a' --at 34%,4.6% --anchor r
pixelox text add ad.pxo --in-place --name SloganWords --content "OWN THE DAY" --font "Helvetica Neue" --weight 400 --size 4% --tracking 20 --color '#ffffff' --at 38%,4.6% --anchor l
pixelox comp place ad.pxo logo.png --in-place --as Emblem --size 92x --at 86.2%,25.5% --anchor c
pixelox shape rect ad.pxo --in-place --name Pill --rect 76%,29.8%,21%x4.4% --radius 32 --fill '#050a2a'
pixelox fx outer-glow ad.pxo --in-place --layer Pill --blur 24 --color '#ffffff' --opacity 35
pixelox text add ad.pxo --in-place --name PillWord --content "BRAND" --font "Helvetica Neue" --weight 700 --size 3.2% --tracking 20 --color '#ffffff' --at 86.5%,32% --anchor c
pixelox shape rect ad.pxo --in-place --name CaptionBox --rect 38%,78.5%,56.5%x10.5% --radius 60 --fill '#0a2fa0' --stroke '#7fb8ff' --stroke-width 2 --opacity 85
pixelox fx outer-glow ad.pxo --in-place --layer CaptionBox --blur 26 --color '#3f8cff' --opacity 60
pixelox text add ad.pxo --in-place --name Caption --content $'Open One And Make Any Break\nFeel Like A Small Party' --font "Helvetica Neue" --weight 700 --size 2.2% --tracking 80 --leading 135% --color '#ffffff' --at 43%,83.8% --anchor l
pixelox layer add ad.pxo --in-place --name Grain --fill '#808080'
pixelox filter add-noise ad.pxo --in-place --layer Grain --amount 10 --monochrome
pixelox layer blend ad.pxo --in-place --layer Grain --mode soft-light
pixelox layer opacity ad.pxo --in-place --layer Grain --value 30
pixelox export ad.pxo -o ad.png
```

Change first: the haze colors (the brand's main color for `bg`, its neighbor on the color
wheel for the two warm blobs), then the slogan pair and the caption, then the person's
position so the face sits in the upper-middle third.

## 10. Variations and pairings

* Orange drink: ground `#e85d04`, haze in yellow `#ffd23c` and red `#c1121f`, navy pill
  becomes dark brown. Energy drink: black ground, lime `#b6ff00` haze.
* No person: scale the can up to 55% W, center it, and keep the slogan line, pill, and
  caption box (it becomes a plain hero-object ad).
* Story 9:16: slogan at 6% H, person 15 to 70% H, can 55 to 90% H at the left, caption box
  full width at 88% H.
* Replace the caption box with a real call to action (T-pill-button, "Order Now").
* Pairs with T-lens-flare on the can's rim and T-light-leak in pink from the right edge.

## 11. Pitfalls

* Person and can at similar sizes compete. One is big and near (the can, sharp), the other
  mid-distance and emotional; overlap them.
* A hard bottom edge on the person cutout. Fade the torso into the ground or hide the crop
  behind the can and the caption box.
* Haze in a complementary color (orange on blue) turns the middle gray-brown. Use the
  ground's neighbor (magenta, violet, cyan).
* Caption text touching the rounded corners: with a large radius, inset the text by at
  least half the box height from the left.
* Red text on saturated blue vibrates when small. Keep the red word at 3% H cap height or
  more and bold; never set body text in red on blue.
