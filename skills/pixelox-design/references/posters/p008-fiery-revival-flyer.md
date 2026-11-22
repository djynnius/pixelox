---
id: p008
title: Fiery revival flyer with speaker cutout
genre: [church, event-flyer]
mood: [intense, urgent, warm, dramatic]
style: [photo-composite, type-led]
format: portrait-4x5
archetype: A-event-stack
palette: ['#a93a32', '#351c1c', '#ffffff', '#ff8a2a', '#0a0000']
techniques: [T-fire-glow, T-fit-width, T-text-behind-subject, T-info-bar, T-paper-grain]
needs: [subject-cutout, logo]
---

# p008: Fiery revival flyer with speaker cutout

## 1. Concept

A multi-night church event announced by a three-line white headline stacked on a swirling
red-orange "fire" field, with a smiling speaker cut out in front of the last headline line
and a dark info bar across his chest. It works because there are only three things to read
(what, who, when) and each has its own zone, and because the white type on a red field has
the highest possible contrast without using black.

## 2. Format and grid

* Portrait 4:5. Everything sits on the vertical center axis; margins 8% W left and right.
* Four stacked zones: presenter block 3 to 14% H; headline 17 to 62% H; speaker 52 to 100% H
  (overlapping the headline's last line); info bar 81 to 92% H, in front of the speaker.
* Only two things break the symmetry: the speaker's name, set to the left of his head, and
  the three unequal columns inside the info bar.

## 3. Layer stack

1. Ground: dark maroon `#351c1c`, full canvas.
2. Fire field: blurred, swirled red-orange clouds, full canvas; brightest (orange
   `#ff8a2a` to `#d58e69`) at the left edge 25 to 45% H and the right edge 10 to 35% H,
   darkest in the lower middle (T-fire-glow).
3. Flame wisps: thin orange streaks in screen mode, 40% opacity.
4. Radial shade: transparent in the middle, 80% dark maroon at the corners.
5. Logo: white line icon (a small building), 10% W wide, centered, top at 3% H.
6. Presenter name: white bold caps, centered at 11% H; "PRESENTS" below at 13.5% H.
7. Headline line 1: white, centered, cap height 9% H, about 39% W wide, top at 17.5% H.
8. Headline line 2: white, centered, cap height 16% H, about 68% W wide, top at 28% H.
9. Headline line 3: white, centered, cap height 16% H, about 60% W wide, top at 46% H.
   All three lines carry a soft black shadow (offset 0,8 px, blur 22, 35%).
10. Speaker cutout: head and torso, centered, shoulders 42% W wide, top of head at 52% H,
    running off the bottom edge. Covers the middle two letters of headline line 3.
11. Speaker name: two short white bold lines at x 22% W, y 66% H, left of the head.
12. Info bar: `7.8%,81.3%,86% x 10.7%`, gradient from near-black `#0a0000` (left) to dark
    red `#6c1b1c` (right), with a 1 px dotted white outline at 70%.
13. Bar text: three columns in white (date, time, place), split at 41% and 59% W.
14. Grain: fine monochrome noise, soft-light, 40%.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#a93a32` | fire field mid red | 40% |
| `#351c1c` | dark maroon ground and shade | 25% |
| `#ffffff` | all type, the shirt of the subject | 20% |
| `#ff8a2a` | hot orange highlights in the field | 10% |
| `#0a0000` | info bar | 5% |

Analogous warm scheme (maroon, red, orange) with white as the only contrast color. No
second hue anywhere: that restraint is what keeps a busy background readable.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Presenter | `[CHURCH NAME]`, 2 words | caps | geometric sans (Montserrat / Avenir Next) | 700 | 1.4% | 0 | - | center | `#ffffff` | 50%, 11.3% |
| Presents | "PRESENTS" | caps | geometric sans (Montserrat / Avenir Next) | 600 | 0.9% | 500 | - | center | `#ffffff` | 50%, 13.6% |
| Headline 1 | number + unit, 2 words, 6 to 8 characters | caps | ultra-condensed heavy grotesque (Anton, League Gothic / Impact) | 400 (the face is heavy) | 9% | 0 | - | center | `#ffffff` | 50%, top 17.5% |
| Headline 2 | 1 word + "&", 8 to 9 characters | caps | same | same | 16% | 0 | - | center | `#ffffff` | 50%, top 28% |
| Headline 3 | 1 word, 7 characters | caps | same | same | 16% | 0 | - | center | `#ffffff` | 50%, top 46% |
| Speaker name | `[SPEAKER NAME]`, 2 lines | caps | geometric sans (Montserrat / Avenir Next) | 700 | 1.3% | 0 | 100% | left | `#ffffff` | 22%, 66% |
| Date | day range / month + year, 2 lines | caps | wide geometric sans (Montserrat ExtraBold / Avenir Next Heavy) | 900 | 2.4% | 0 | 92% | left | `#ffffff` | 17.5%, 86.7% |
| Time | "6PM"-style, 3 characters | caps | same | 900 | 3% | 0 | - | center | `#ffffff` | 49%, 85.6% |
| Time note | 2 words | caps | same | 700 | 1% | 0 | - | center | `#ffffff` | 49%, 88.6% |
| Place | street / city, 2 lines | caps | same | 900 | 1.7% | 0 | 100% | left | `#ffffff` | 64.5%, 86.7% |

Two families only: an ultra-condensed heavy face for the headline and one geometric sans in
two weights for everything else.

## 6. Imagery

One studio portrait of a smiling man in a white shirt, chin resting on his fist, arms
folded low: cut out cleanly (see "Getting a cutout" in techniques.md), no outline, no glow,
natural color. He faces the camera; the white shirt repeats the white of the headline, so
subject and type read as one unit. The bar crosses him at chest height and his forearms
continue below it, which locks him into the layout instead of floating on it.
Two small white line icons (a calendar, a map pin) sit left of the date and the place.

## 7. Effects and textures

* **T-fire-glow** for the ground: colors `#2a0503 / #b3200e / #ff8a2a`, blur 9, spin 16
  around 55%,40%; plus two or three orange glow blobs at the edges in screen mode and a
  streaky wisp layer (clouds with `warp=1.9`, `stretch=1.8`) for the look of flames.
* **T-text-behind-subject**: the cutout is placed after the headline, the bar after the
  cutout.
* **T-info-bar**: gradient bar with a dotted outline.
* **T-paper-grain** at low strength to bind the blurred field and the sharp type.

## 8. Hierarchy and eye path

1. Headline lines 2 and 3 (cap height 16% H, white on red, upper middle).
2. The face (the only photograph, dead center, overlapping the headline).
3. Headline line 1 (the number), then down to the info bar: time first (largest in the
   bar), then date and place.
4. Presenter block and speaker name last.

Size ratio, cap heights: headline 16 : number line 9 : bar time 3 : bar date 2.4 : names
1.3, about 12 : 7 : 2.3 : 1.8 : 1.

## 9. Build recipe

`subject.png` and `logo.png` are your images here. With the ideal font registered
(`fonts add` a folder that holds Anton or League Gothic), use it for H1 to H3 and keep the
same `transform scale` widths: the narrower letters come out taller, close to the 16% cap
height of the description. Impact gives about 12%.

```bash
pixelox render js scripts/clouds.js -o heat.png --size 1080x1350 --seed 11 --param lo='#2a0503' --param mid='#b3200e' --param hi='#ff8a2a' --param scale=1.8 --param octaves=4 --param warp=1.6 --param contrast=1.7
pixelox new -o flyer.pxo --size 1080x1350 --fill '#2a0503'
pixelox comp place flyer.pxo heat.png --in-place --as Heat --fit cover
pixelox filter gaussian flyer.pxo --in-place --layer Heat --sigma 9
pixelox filter radial-blur flyer.pxo --in-place --layer Heat --kind spin --amount 16 --center 55%,40%
pixelox render js scripts/blobs.js -o glow.png --size 1080x1350 --param bg=transparent --param colors='#ff9a2a,#ffc45a,#ff6a1a' --param at='0.02,0.3;0.98,0.22;0.3,0.02' --param size=0.42 --param alpha=0.85
pixelox comp place flyer.pxo glow.png --in-place --as Glow --fit cover
pixelox filter gaussian flyer.pxo --in-place --layer Glow --sigma 40
pixelox layer blend flyer.pxo --in-place --layer Glow --mode screen
pixelox render js scripts/clouds.js -o wisps.png --size 1080x1350 --seed 23 --param lo=transparent --param hi='#ffb347' --param scale=2.2 --param warp=1.9 --param stretch=1.8 --param bias=-0.2 --param contrast=1.6 --param res=2
pixelox comp place flyer.pxo wisps.png --in-place --as Wisps --fit cover
pixelox filter gaussian flyer.pxo --in-place --layer Wisps --sigma 5
pixelox layer blend flyer.pxo --in-place --layer Wisps --mode screen
pixelox layer opacity flyer.pxo --in-place --layer Wisps --value 40
pixelox layer add flyer.pxo --in-place --name Shade
pixelox fill gradient flyer.pxo --in-place --layer Shade --kind radial --from 50%,38% --to 50%,118% --stops '#14020100@0.3,#140201cc@1'
pixelox comp place flyer.pxo logo.png --in-place --as Logo --size 100x --at 50%,3.2% --anchor t
pixelox text add flyer.pxo --in-place --name Org --content "CHURCH NAME" --font "Avenir Next" --weight 700 --size 1.9% --color '#ffffff' --at 50%,11.3% --anchor c
pixelox text add flyer.pxo --in-place --name Presents --content "PRESENTS" --font "Avenir Next" --weight 600 --size 1.25% --tracking 500 --color '#ffffff' --at 50.8%,13.6% --anchor c
pixelox text add flyer.pxo --in-place --name H1 --content "7 NIGHTS" --font Impact --size 10% --color '#ffffff'
pixelox transform scale flyer.pxo --in-place --layer H1 --size 430x
pixelox transform move flyer.pxo --in-place --layer H1 --to 50%,17.5% --anchor t
pixelox text add flyer.pxo --in-place --name H2 --content "REVIVE &" --font Impact --size 10% --color '#ffffff'
pixelox transform scale flyer.pxo --in-place --layer H2 --size 770x
pixelox transform move flyer.pxo --in-place --layer H2 --to 50%,27.2% --anchor t
pixelox text add flyer.pxo --in-place --name H3 --content "RESTORE" --font Impact --size 10% --color '#ffffff'
pixelox transform scale flyer.pxo --in-place --layer H3 --size 690x
pixelox transform move flyer.pxo --in-place --layer H3 --to 50%,42.5% --anchor t
pixelox fx drop-shadow flyer.pxo --in-place --layer H1 --offset 0,8 --blur 22 --opacity 35
pixelox fx copy flyer.pxo --in-place --from H1 --to H2,H3
pixelox comp place flyer.pxo subject.png --in-place --as Subject --size 640x --at 50%,101% --anchor b
pixelox text add flyer.pxo --in-place --name Speaker --content $'SPEAKER\nNAME' --font "Avenir Next" --weight 700 --size 1.75% --leading 100% --color '#ffffff' --at 14%,62% --anchor l
pixelox shape rect flyer.pxo --in-place --name Bar --rect 7.8%,81.3%,86%x10.7% --fill '#000000'
pixelox fx overlay flyer.pxo --in-place --layer Bar --stops '#0a0000@0,#6d0c08@1' --angle 0
pixelox shape rect flyer.pxo --in-place --name BarEdge --rect 7.8%,81.3%,86%x10.7% --fill none --stroke '#ffffff' --stroke-width 1 --dash 2,4 --opacity 70
pixelox text add flyer.pxo --in-place --name Date --content $'3RD - 9TH\nMAY 2031' --font "Avenir Next" --weight 900 --size 3.3% --leading 92% --color '#ffffff' --at 17.5%,86.7% --anchor l
pixelox text add flyer.pxo --in-place --name Time --content "7PM" --font "Avenir Next" --weight 900 --size 4.2% --color '#ffffff' --at 49%,85.6% --anchor c
pixelox text add flyer.pxo --in-place --name TimeNote --content "EACH NIGHT" --font "Avenir Next" --weight 700 --size 1.35% --color '#ffffff' --at 49%,88.6% --anchor c
pixelox text add flyer.pxo --in-place --name Place --content $'12 EXAMPLE ROAD,\nYOUR CITY 00000' --font "Avenir Next" --weight 900 --size 2.35% --leading 100% --color '#ffffff' --at 64.5%,86.7% --anchor l
pixelox layer add flyer.pxo --in-place --name Grain --fill '#808080'
pixelox filter add-noise flyer.pxo --in-place --layer Grain --amount 12 --monochrome
pixelox layer blend flyer.pxo --in-place --layer Grain --mode soft-light
pixelox layer opacity flyer.pxo --in-place --layer Grain --value 40
pixelox export flyer.pxo -o flyer.png
```

Change first: the three headline strings (keep line 1 short, lines 2 and 3 at 7 to 9
characters), then the `transform scale --size` widths if a line looks too small, then the
three fire colors.

## 10. Variations and pairings

* Swap the temperature: `#02122e / #0b4fd1 / #5fd0ff` gives a water or night-of-worship
  version; `#12051f / #6a1bb3 / #ff5ad1` a youth or concert version. Keep the type white.
* Two speakers: place two cutouts side by side at 34% and 66% W, each 34% W wide, with a
  name under each instead of beside.
* No photo: drop layers 10 and 11, scale the headline to 86% W with T-fit-width, and move
  the bar up to 70% H.
* Square post: keep the headline and the bar, crop the speaker at the chest, drop the
  presenter block to one line.
* Pairs well with T-script-overlap (one script word over headline line 2), T-sunburst
  behind the head, and T-light-leak.

## 11. Pitfalls

* A wide font makes the headline run off the canvas or forces it small. Use a condensed
  face and set each line's width with `transform scale`, not by guessing a font size.
* The face covers too much of line 3. Keep the head over 2 letters in the middle of the
  word; the first and last letters must stay visible.
* The background competes with the type. Blur it (sigma 9 or more), keep it within one hue
  family, and keep the shade layer.
* Gray or pure-black bar on a red field looks dirty. Use a near-black with red in it
  (`#0a0000`) fading to dark red.
* More than three facts in the bar. If there is a fourth, it goes under the bar in one small
  centered line.
