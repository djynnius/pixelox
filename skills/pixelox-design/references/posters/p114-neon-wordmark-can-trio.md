---
id: p114
title: Neon script wordmark behind three cans
genre: [drink-ad, product-ad]
mood: [dark, dramatic, bold, moody]
style: [3d-render, neon]
format: landscape
archetype: A-text-behind-subject
palette: ['#000000', '#121212', '#e6031c', '#a00414', '#c8c8c8']
techniques: [T-neon-glow, T-text-behind-subject, T-fit-width, T-floor-reflection]
needs: product-cutout
---

# p114: Neon script wordmark behind three cans

## 1. Concept

A landscape hero image for a soft drink: the `[BRAND]` script wordmark, as wide as the
frame and glowing neon red on a black wall, with three cola cans standing in front of its
middle on a glossy black floor that mirrors both the cans and the red light. It works
because the logo is the lighting: one color lights the whole scene, the product blocks
part of the sign (so the sign reads as a real object in a real room), and everything else
is black.

## 2. Format and grid

* Landscape 16:9. Strict symmetry on the vertical axis at 50% W: wordmark, can group, and
  fine print are all centered.
* One horizon at 76% H splits the frame into wall (0 to 76% H, pure black) and floor
  (76 to 100% H, glossy near-black).
* Wordmark box `7%,13%,86% x 49%`: it leaves 7% W at each side and 13% H above.
* Can group box `39.6%,37.5%,22.8% x 42.5%`: it stands on the floor 4% H in front of the
  horizon line and hides the middle quarter of the wordmark.
* Fine print in the bottom 6% H. Nothing else: no headline, no logo lockup, no button.

## 3. Layer stack

1. Wall: black `#000000`, full canvas.
2. Floor: `0,76%,100% x 24%`, near-black `#121212`, its far edge a crisp straight line
   against the wall. It is added before the sign so the sign's reflection falls on it.
3. Wordmark aura: a wide, soft red haze around the letters (dark red `#230002` between
   the strokes, reaching about 5% W out).
4. Wordmark: two-part script name, flat saturated red `#e6031c`, `7%,13%,86% x 49%`,
   capital height about 36% H, with a tight glow of lighter red `#ff1030` about 1.6% W
   wide hugging every stroke. The letters stay red; they do not burn out to white.
5. Wordmark reflection: the mirrored sign as blurred red light on the floor, strongest at
   88 to 100% H between 10 to 35% W and 62 to 90% W (dark red `#510202`).
6. Can reflections: each can mirrored below its own base, 78 to 100% H, at about 40%
   strength, sharp at the base, blurring with distance, fading to nothing.
7. Cans: two cans behind (tops at 40% H, bases at 78% H, turned outward to show the side
   label) and one in front (top 37.5% H, base 80% H, 13% W wide, facing the camera). Red
   bodies, silver rims, white script on the front can, small water drops.
8. Fine print: two centered lines of small tracked gray caps at 95.3% and 97.2% H.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#000000` | wall | 55% |
| `#121212` | floor | 20% |
| `#e6031c` | wordmark, can bodies | 15% |
| `#a00414` | glow falloff, shaded can sides, floor reflection | 7% |
| `#c8c8c8` | can rims, white lettering on the cans, fine print (as `#8a8a8a`) | 3% |

Black plus one red. Silver and white appear only on the product, which is why the cans
separate from a sign of the same hue.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Wordmark | `[BRAND]`: two 4-letter parts joined by a hyphen, a long swash under the first part | title | flowing formal script with heavy strokes (Pinyon Script, Great Vibes / Snell Roundhand) | 900 | 36% (whole mark 49% H tall) | 0 | - | center | `#e6031c` | 50%, top 13% |
| Fine print 1 | one sentence, about 20 words (a production credit) | caps | geometric sans (Montserrat / Avenir Next) | 400 | 0.8% | 40 | - | center | `#8a8a8a` | 50%, 95.3% |
| Fine print 2 | half a sentence, about 9 words | caps | same | 400 | 0.8% | 40 | - | center | `#8a8a8a` | 50%, 97.2% |

The wordmark is the only real type. Ratio to the fine print: 45 : 1. There is no middle
level, which is what makes it a hero image and not an ad layout.

## 6. Imagery

Three identical drink cans rendered in 3D (a photographed cutout works the same): one
facing the camera, two half a can-width behind it and turned 30 degrees outward so the
group reads as a tight triangle seen from slightly below eye level. Hard, small highlights
down each can and condensation drops say "cold". The cans are lit from the front and
above, not by the sign, so their fronts are bright while the wall stays black. The group is
small: 23% W by 42% H. The wordmark is nearly four times wider (**T-text-behind-subject**
with the word, not the subject, as the big element).

## 7. Effects and textures

* **T-neon-glow**, light-box variant: the letters keep the full red (no near-white tube),
  `fx outer-glow --blur 26 --spread 8` in `#ff1030`, plus a second copy of the text under
  it, blurred with sigma 36 (the blur is live; the text stays editable), in `screen` mode
  at 70% for the aura.
* **T-fit-width**: `--fit-width 1340` sets the wordmark to 84% W.
* **T-floor-reflection** on the product: `fx reflection --length 45% --opacity 42 --blur
  12` on one can, then `fx copy` to the other two. Each can mirrors about its own base, so
  the back cans (78% H) and the front can (80% H) all touch their reflections; the blur
  grows away from the base and the image fades out by itself.
* **T-floor-reflection** on the sign: `fx reflection` on the wordmark with `--gap 330` (the
  mirror image starts at 89% H, low on the floor), `--length 70% --fade 50` (it stays
  strong to the bottom edge), `--blur 64 --distance 0` (an even blur, sigma 32), `--blend
  screen`, 70%.
* No grain, no vignette: the black is clean, as in a studio render.

## 8. Hierarchy and eye path

1. The wordmark: 86% of the width, the only saturated light source.
2. The cans: sharp detail and the only white and silver, exactly on the axis where the
   wordmark is interrupted.
3. Down the reflections to the floor glow.
4. The fine print, almost hidden on purpose.

## 9. Build recipe

`product.png` is your image here: one can, bottle, or pack as a cutout, used three times.
The stand-in is tall and narrow; for a can keep the heights (`x350`, `x385`) and move the
side cans to 43% and 57% W so they tuck behind the front one.

```bash
pixelox new -o hero.pxo --size 1600x900 --fill '#000000'
pixelox shape rect hero.pxo --in-place --name Floor --rect 0,76%,100%x24% --fill '#121212'
pixelox text add hero.pxo --in-place --name Halo --content "Rosso" --font @script-formal --weight 900 --fit-width 1340 --color '#e6031c' --at 50%,13% --anchor t --anchor-to ink
pixelox filter gaussian hero.pxo --in-place --layer Halo --sigma 36
pixelox layer blend hero.pxo --in-place --layer Halo --mode screen
pixelox layer opacity hero.pxo --in-place --layer Halo --value 70
pixelox text add hero.pxo --in-place --name Wordmark --content "Rosso" --font @script-formal --weight 900 --fit-width 1340 --color '#e6031c' --at 50%,13% --anchor t --anchor-to ink
pixelox fx stroke hero.pxo --in-place --layer Wordmark --width 2 --color '#e6031c' --position outside
pixelox fx outer-glow hero.pxo --in-place --layer Wordmark --blur 26 --spread 8 --color '#ff1030' --opacity 85
pixelox fx reflection hero.pxo --in-place --layer Wordmark --gap 330 --length 70% --fade 50 --blur 64 --distance 0 --blend screen --opacity 70
pixelox comp place hero.pxo product.png --in-place --as CanLeft --size x350 --at 42.5%,78% --anchor b
pixelox comp place hero.pxo product.png --in-place --as CanRight --size x350 --at 57.5%,78% --anchor b
pixelox comp place hero.pxo product.png --in-place --as CanFront --size x385 --at 50%,80% --anchor b
pixelox fx reflection hero.pxo --in-place --layer CanLeft --length 45% --opacity 42 --blur 12
pixelox fx copy hero.pxo --in-place --from CanLeft --to CanRight,CanFront
pixelox text add hero.pxo --in-place --name Fine1 --content "RENDERED ENTIRELY IN 3D. NO CAMERA WAS USED TO MAKE THIS PICTURE; EVERY SURFACE IS MODELED AND LIT BY HAND." --font @geometric-sans --weight 400 --size 1.15% --tracking 40 --color '#8a8a8a' --at 50%,95.3% --anchor c
pixelox text add hero.pxo --in-place --name Fine2 --content "A STUDY PIECE, MADE FOR THE LOVE OF THE CRAFT." --font @geometric-sans --weight 400 --size 1.15% --tracking 40 --color '#8a8a8a' --at 50%,97.2% --anchor c
pixelox export hero.pxo -o hero.png
```

Two things in this recipe are easy to get wrong. A reflection is part of its layer, so
the floor must be below the wordmark and the cans: it is added first, right after the
wall. And a reflection touches the product only if the cutout has no empty border: trim
yours first (`pixelox transform trim product.png -o tight.png`). The reflections are live:
edit the wordmark or move a can and they follow.

Change first: the wordmark (5 to 8 letters; a real logo placed as `logo.png` and tinted
with `fx overlay --color` replaces both text layers), then the one color.

## 10. Variations and pairings

* Other single hues: electric blue `#0a6cff` with glow `#3d8bff` for an energy drink,
  green `#19c15a` for a lime soda, amber `#ffab1a` for beer, white `#f4f4f4` on black for
  a premium spirit. Keep wall and floor black.
* Portrait 4:5: stack the wordmark on two lines (each 84% W), stand one large bottle in
  front of the gap between them, horizon at 82% H.
* Swap the script for a heavy condensed sans in outline (T-outline-type) with the same
  glow for a sports or tech product.
* One hero product instead of three, 60% H tall, with T-spotlight on the floor behind it.
* Pairs with T-smoke (thin red-tinted haze low on the floor), T-lens-flare on the rim of
  the front can, and T-vignette when the wall is not pure black.

## 11. Pitfalls

* The product hides too much of the name. Keep the group narrower than a quarter of the
  wordmark and over its middle, so the first and last letters and the overall shape stay
  readable.
* White-hot letters. With a script this heavy a near-white core turns the sign into a
  blank shape; keep the fill saturated and let only the glow be lighter.
* A reflection that is as bright as the object. Keep the cans' `--opacity` at 40 to 45,
  blur it, and let it fade out; a full-strength mirror looks like a second row of cans.
* A gray floor. The floor is only 7% lighter than the wall; any lighter and the picture
  splits into two bands and the sign stops lighting the room.
* Adding a headline. The wordmark already is the headline; a second line of large type
  competes with it. Put any message in the fine-print band or leave it out.
* Rebuilding a known brand's trade dress. A white script wordmark and ribbon on this exact red
  is one company's signature. Take the idea (what the object, the mark, and the ground do
  to each other) and build it with your own brand's colors, lettering, and shapes.
