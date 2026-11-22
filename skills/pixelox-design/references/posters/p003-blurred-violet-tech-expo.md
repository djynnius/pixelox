---
id: p003
title: Thin type on blurred violet light
genre: [conference, tech-ad, event-flyer]
mood: [futuristic, cool, dark, minimal]
style: [gradient-mesh, minimal, type-led]
format: portrait-a
archetype: A-abstract-title
palette: ['#01010f', '#020166', '#3c0ba7', '#614499', '#ffffff']
techniques: [T-gradient-mesh, T-pill-button, T-pattern, T-paper-grain, T-light-streaks]
needs: nothing
---

# p003: Thin type on blurred violet light

## 1. Concept

A tech-expo poster made of almost nothing: a near-black navy field crossed by two
out-of-focus violet light streaks, and a two-line headline in hairline-thin wide capitals
sitting dead center. It works because the type is thin and the light is soft, so the
only sharp, bright thing on the page is the headline; the blur reads as "screen glow seen
up close" without showing any actual device.

## 2. Format and grid

* Portrait A-series (1 : 1.414). Side margins 10% W; top labels at 8 to 11% H; footer at
  91% H.
* One vertical axis at 50% W. Everything is centered on it except the first and last of
  the three top labels, which hang from the left and right margins (10% and 90% W).
* Zones: label row 8 to 11% H; headline 31 to 52% H (the optical middle, slightly above
  center); sub-line 58 to 62% H; button 66 to 71% H; footer 90 to 92% H.
* 20% H of empty dark space between the label row and the headline, and again between
  the button and the footer.

## 3. Layer stack

1. Ground: near-black navy `#01010f`, full canvas.
2. Light streaks: two soft violet bands running from upper left to lower right at about
   50 degrees, fully out of focus. Left band: from 22%,33% to 62%,76%, lavender `#614499`,
   widening into a paler mass at the left edge (0 to 30% W, 50 to 80% H). Right band:
   from 75%,25% to 100%,62%, saturated violet `#3c0ba7` to `#5b2fb4`. The middle of the
   canvas (40 to 60% W) stays dark.
3. Top band: deep ultramarine `#020166`, 0 to 15% H, with a soft lower edge (a 4% H ramp).
4. Dot texture: a small patch of fine light dots (pitch 1.2% W) inside the left band at
   15 to 45% W, 52 to 62% H, 25% opacity; a hint of a screen or speaker grille.
5. Three top labels, white, two lines each: left-aligned at 10% W, centered at 50% W,
   right-aligned at 90% W; block center at 9.7% H.
6. Headline: two lines of four letters, white, centered, both at one size (cap height
   8.8% H) and about 47% W wide; cap tops at 31.2% and 43.1% H.
7. Sub-line: two centered lines of small caps at 58.6% and 61.1% H.
8. Button: outlined pill `30.6%,66.1%,38.8% x 4.5%`, 2 px white stroke, label plus a thin
   arrow pointing right.
9. Footer: one centered line of small caps at 90.7% H.
10. Grain: one live grain layer (fine, monochrome) under the type.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#01010f` | near-black navy ground | 50% |
| `#020166` | ultramarine top band and deep glow | 20% |
| `#3c0ba7` | saturated violet streak (right) | 10% |
| `#614499` | lavender streak (left) | 15% |
| `#ffffff` | all type, the pill outline | 5% |

Analogous blue to violet on black, white type only. No warm color anywhere; the violet is
the single "brand" hue and appears only as light, never as a flat shape.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Top labels (x3) | 2 words on 2 lines, 3 to 7 letters each (a theme keyword pair) | caps | light wide geometric sans (Montserrat 300, Jost 300 / Avenir) | 400 | 1.2% | 20 | 125% | left / center / right | `#ffffff` | 10%, 50%, 90%; block center 9.7% |
| Headline | 2 words on 2 lines, 4 letters each | caps | thin wide geometric sans (Montserrat 200, Raleway 200 / Avenir Light) | 300 | 8.8% | 30 | line gap 35% of cap height | center | `#ffffff` | 50%, cap tops 31.2% and 43.1% |
| Sub-line | 2 lines, 3 words each, 20 to 24 characters per line | caps | same | 400 | 1.4% | 40 | 180% | center | `#ffffff` | 50%, 58.6% and 61.1% |
| Button label | 3 to 4 word imperative | caps | same | 400 | 1.3% | 20 | - | center | `#ffffff` | 47.5%, 68.3% |
| Footer | "MORE INFORMATION:" plus a web address | caps | same | 400 | 1.1% | 20 | - | center | `#ffffff` | 50%, 90.7% |

One family in two light weights. The headline strokes are about 7% of the cap height; that
thinness is the voice of the design. The small lines are one weight heavier so they stay
white at 1.2% H instead of fading to gray. Headline to sub-line is 6.3 : 1.

## 6. Imagery

No photograph. The background suggests a macro photo of glowing hardware, shot far out of
focus: two diagonal bars of violet light, one textured with a faint dot grid. It is built
from blurred color blobs (T-gradient-mesh with fixed centers) pulled into streaks with a
diagonal motion blur.

## 7. Effects and textures

* **T-gradient-mesh**, fixed centers: `blobs.js` with `bg=transparent`, six blobs placed
  along two parallel diagonals (`at=` list), `size=0.17`, `soft=0.6`; then `filter motion-blur` at
  angle -52, distance 420 (39% W) to stretch the blobs into bars, `filter gaussian` sigma
  18 to defocus them, and `screen` blending over the dark ground.
* Top band: a linear gradient of the ultramarine from solid at 11% H to transparent at
  19% H.
* **T-pattern**: `kind=dots`, `cell=13`, `weight=0.3`, `fade=out`, rendered as a small
  patch, rotated to the streak angle, 25% opacity.
* **T-pill-button** (outlined) with `shape arrow`.
* **T-paper-grain** (`filter grain --as-layer --amount 8`, added before the type so the
  thin letters stay clean): needed to hide banding in the dark blur.
* **T-light-streaks** in [techniques.md](../techniques.md) is the tested library version of the defocused light streaks; start from it when reusing this idea.

## 8. Hierarchy and eye path

1. The headline: the only large, sharp, white shape, in the dark gap between the streaks.
2. The sub-line directly below it, then the button (the outline makes it the one "object").
3. The three top labels, read left to right.
4. The footer.

Cap heights: headline 8.8 : sub-line 1.4 : labels 1.2 : footer 1.1, about 8 : 1.3 : 1.1 :
1. One giant step and then everything else at nearly one size.

## 9. Build recipe

Nothing to supply. Canvas 1080x1528.

```bash
pixelox render js scripts/blobs.js -o streaks.png --size 1080x1528 --param bg=transparent --param colors='#9a80e0,#8a6ad8,#6a3fd0,#4a14c8,#5a2ad0,#a898dc' --param at='0.22,0.38;0.36,0.55;0.5,0.72;0.80,0.27;0.95,0.48;0.06,0.66' --param size=0.17 --param vary=0.2 --param alpha=1 --param soft=0.6
pixelox render js scripts/pattern.js -o dots.png --size 380x200 --param kind=dots --param color='#b9a8ee' --param cell=13 --param weight=0.3 --param fade=out
pixelox new -o expo.pxo --size 1080x1528 --fill '#01010f'
pixelox comp place expo.pxo streaks.png --in-place --as Streaks --fit cover
pixelox filter motion-blur expo.pxo --in-place --layer Streaks --angle=-52 --distance 420
pixelox filter gaussian expo.pxo --in-place --layer Streaks --sigma 18
pixelox layer blend expo.pxo --in-place --layer Streaks --mode screen
pixelox layer add expo.pxo --in-place --name TopBand
pixelox fill gradient expo.pxo --in-place --layer TopBand --from 0,11% --to 0,19% --stops '#020166ff@0,#02016600@1'
pixelox comp place expo.pxo dots.png --in-place --as Dots --at 31%,58% --anchor c
pixelox transform rotate expo.pxo --in-place --layer Dots --degrees 20
pixelox layer opacity expo.pxo --in-place --layer Dots --value 25
pixelox filter grain expo.pxo --in-place --as-layer --name Grain --amount 8
pixelox text add expo.pxo --in-place --name LabelL --content $'OPEN\nDATA' --font Avenir --weight 400 --size 1.65% --tracking 20 --leading 125% --color '#ffffff' --at 10%,9.7% --anchor l
pixelox text add expo.pxo --in-place --name LabelC --content $'MACHINE\nVISION' --font Avenir --weight 400 --size 1.65% --tracking 20 --leading 125% --align center --color '#ffffff' --at 50%,9.7% --anchor c
pixelox text add expo.pxo --in-place --name LabelR --content $'EDGE\nCLOUD' --font Avenir --weight 400 --size 1.65% --tracking 20 --leading 125% --align right --color '#ffffff' --at 90%,9.7% --anchor r
pixelox text add expo.pxo --in-place --name H1 --content "CODE" --font Avenir --weight 300 --size 12.4% --tracking 30 --color '#ffffff' --at 50%,31.2% --anchor t --anchor-to ink
pixelox text add expo.pxo --in-place --name H2 --content "FAIR" --font Avenir --weight 300 --size 12.4% --tracking 30 --color '#ffffff' --at 50%,43.1% --anchor t --anchor-to ink
pixelox text add expo.pxo --in-place --name Sub --content $'WHERE BUILDERS SHOW\nWHAT COMES NEXT' --font Avenir --weight 400 --size 1.95% --tracking 40 --leading 180% --align center --color '#ffffff' --at 50%,59.9% --anchor c
pixelox shape rect expo.pxo --in-place --name Pill --rect 30.6%,66.1%,38.8%x4.5% --radius 34 --fill none --stroke '#ffffff' --stroke-width 2
pixelox text add expo.pxo --in-place --name PillText --content "RESERVE A PASS" --font Avenir --weight 400 --size 1.8% --tracking 20 --color '#ffffff' --at 47.5%,68.35% --anchor c
pixelox shape arrow expo.pxo --in-place --name Arrow --from 62%,68.35% --to 65.6%,68.35% --stroke '#ffffff' --stroke-width 2 --head-size 12
pixelox text add expo.pxo --in-place --name Footer --content "MORE INFORMATION: WWW.EXAMPLE.COM" --font Avenir --weight 400 --size 1.55% --tracking 20 --color '#ffffff' --at 50%,90.7% --anchor c
pixelox export expo.pxo -o expo.png
```

Change first: the two headline words (4 to 5 letters each, so both lines come out nearly
the same width at one size; do not fit them to a width, or the cap heights will differ),
then the streak colors (keep them within one hue family), then the `at=` centers if a
streak runs behind the headline.

## 10. Variations and pairings

* Hue swaps for the streaks on the same near-black: teal `#0a6a78 / #19c2c9`, magenta
  `#7a0f5c / #d13aa0`, amber `#7a3a05 / #e08a1c` (then tint the ground toward that hue,
  for example `#0f0802`).
* Change the streak angle: `--angle 0` gives horizontal light bars (a calmer, "scanner"
  look); `--angle 90` gives vertical shafts.
* Landscape banner: put the headline on one line (`--fit-width 760` on a 1600x900 canvas)
  and keep one streak per side.
* Suits product launches, hackathons, AI meetups, webinar covers, album art.
* Pairs with T-neon-glow on one short word, T-rules for a hairline under the labels, and
  the A-swiss-asymmetric archetype if the headline is hung from the left margin instead.

## 11. Pitfalls

* A bold headline. Weight 500 or more turns this into an ordinary dark flyer; stay at 200
  to 300 and let the size do the work.
* A streak directly behind the letters: thin white strokes vanish on lavender. Keep the
  canvas center darker than 15% lightness, or move the blob centers outward.
* Under-blurred blobs read as discs. The motion blur distance must be at least 30% W and
  the Gaussian sigma at least 15.
* Small type in the thin weight below 1.1% H cap height breaks up on screens; use weight
  400 for the small lines even though the headline is 300.
* Banding in the dark gradient: keep the grain layer, below the type.
