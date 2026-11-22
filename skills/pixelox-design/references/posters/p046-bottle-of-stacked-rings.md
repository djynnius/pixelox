---
id: p046
title: Bottle drawn as stacked rings
genre: [drink-ad, exhibition, product-ad]
mood: [minimal, elegant, retro, bold]
style: [illustration, minimal, flat]
format: portrait-3x4
archetype: A-centered-title
palette: ['#e61d2a', '#ffffff', '#c9323c']
techniques: [T-frame-border, T-vertical-type, T-rules, T-paper-grain]
needs: nothing
---

# p046: Bottle drawn as stacked rings

## 1. Concept

An anniversary art print: a bottle is drawn with nothing but about 80 thin white ellipses
stacked from cap to base on a flat red field, like a coiled spring, and one continuous
white thread enters at the top with one year written in it and leaves at the bottom with
another. The wordmark is "woven" into the middle of the coil in the ground color. It works
because a single hairline, repeated, is enough to make a famous outline, and the thread
turns the object into a timeline.

## 2. Format and grid

* Portrait, close to 3:4, presented as a print with a white paper border: the red field is
  inset 6.3% W on the left and right and 4.6% H at the top and bottom.
* One vertical axis at 50% W. The bottle is centered on it, `37%,15%,26% x 69%`.
* The thread gives the only asymmetry: it comes in from the top edge of the field at 75% W
  and goes out through the bottom edge at 33% W, so the eye crosses the sheet diagonally
  through the bottle.
* A credit line runs vertically up the right side of the field at 91.7% W, 50 to 89% H,
  with a tiny script mark below it at 92% H.

## 3. Layer stack

1. Paper: white `#ffffff`, full canvas.
2. Field: flat red `#e61d2a`, `6.3%,4.6%,87.4% x 90.8%`.
3. Ring stack: 50 to 80 white ellipses, stroke 0.15% W, no fill, each centered on 50% W,
   spaced 0.85 to 1.4% H apart from 15% H to 84% H. Each ellipse is as wide as the bottle
   at that height (10% W at the neck, 26% W at the shoulder, 23% W at the waist, 26% W near
   the base) and about one eighth as tall as it is wide, so neighbors overlap and the
   stack looks like a transparent solid.
4. Wordmark: the brand's script lettering in the field red, 25% W wide, centered at 48.5%
   H, in front of the rings: the rings seem to stop behind the letters, so the word reads
   as red-on-white.
5. Thread, top: a 1.5 px white curve from the top edge of the field at 75% W, sweeping down
   and left to the bottle's mouth at 55% W, 15% H; a 4-digit year (the later one) written
   small in looping script where it meets the mouth.
6. Thread, bottom: the same line leaving the base at 50% W, 84.5% H, with the earlier year
   written in it, curving down and left to the bottom edge of the field at 33% W.
7. Credit: one line of tiny white text, rotated to read upward, at 91.7% W; a tiny white
   script mark under it.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#e61d2a` | field, wordmark | 66% |
| `#ffffff` | paper border, rings, thread, credit | 30% |
| `#c9323c` | optical mix where rings are dense | 4% |

Two inks. Where the hairlines crowd together (at the shoulder and base) the eye mixes a
third, paler red, which gives the drawing its shading for free.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Wordmark | `[BRAND]` script lettering, 2 short words | title | heavy brush script (Yellowtail, Pacifico / SignPainter) | 400 | 6% | 0 | - | center | `#e61d2a` | 50%, 48.5% |
| Year, top | 4 digits (the present) | - | looping monoline script (Sacramento, Pinyon Script / Snell Roundhand) | 500 | 1.3% | 0 | - | left | `#ffffff` | 57%, 14.3% |
| Year, bottom | 4 digits (the origin) | - | same | 500 | 1.3% | 0 | - | left | `#ffffff` | 51%, 85.8% |
| Credit | one line, about 110 characters: copyright, the occasion, the maker | title | small geometric sans (Montserrat 500 / Avenir Next) | 500 | 0.75% W | 30 | - | rotated -90 | `#ffffff` | 91.7% W, 50 to 89% H |
| Credit mark | `[BRAND]` script | title | script as above | 400 | 1% W | 0 | - | rotated -90 | `#ffffff` | 91.7% W, 92% |

There is no headline. The years and the wordmark are parts of the drawing; the credit is
the only "typeset" line and it is turned sideways to keep out of the picture.

## 6. Imagery

Pure vector line art, no photo. The bottle is a "lathe" drawing: ellipses of changing width
on a common axis. The profile that the widths trace is a curvy glass bottle with a
straight neck, a rounded shoulder, a waist, and a flared base. Any object with a round
cross-section can be drawn this way (a vase, a lamp, a wine glass, a lighthouse, a chess
piece). The thread is drawn by hand as two smooth curves and should look like one line
that passes through the object.

## 7. Effects and textures

* **T-frame-border**, inverted: a white border around a color field instead of a color
  border around a photo. Top and bottom insets are equal in pixels to the side insets.
* Ring stack: one `shape path` whose data is a list of ellipses. Each ellipse is written
  with two relative arcs: `M (cx - rx) y a rx ry 0 1 0 (2 rx) 0 a rx ry 0 1 0 (-2 rx) 0`.
  Step y by 0.9 to 1.4% H (14 px here); set `ry = 0.11 rx + 5`.
* Wordmark knock-out: text in the field color with a 2 px `fx stroke` in the same color, so
  the hairlines end a little before the letters. A second path adds 12 rings between the
  first ones in the band behind the word (twice the density there), which gives the red
  letters enough white to read against.
* **T-vertical-type** for the credit; **T-rules** thinking for the thread (a hairline with
  a job: it carries the two dates).
* Optional **T-paper-grain** at low strength for a printed feel. No shadows, no gradients.

## 8. Hierarchy and eye path

1. The ring bottle (the only large shape; 69% H tall).
2. The red wordmark in its middle, found a moment later.
3. The thread: up to one year, down to the other.
4. The vertical credit, if at all.

Sizes: wordmark cap height 6% H; years 1.3% H; credit under 1% H. Line weight is the same
everywhere (about 1.5 to 2 px on a 1080 px canvas), which is what makes it feel drawn in
one sitting.

## 9. Build recipe

Needs no image. The long `--d` value is 72 ellipses following a bottle profile; replace it
with your own object's profile using the formula in section 7. `RingsDense` doubles the
rings from 43 to 54% H, behind the wordmark.

```bash
pixelox new -o print.pxo --size 1080x1440 --fill '#ffffff'
pixelox shape rect print.pxo --in-place --name Field --rect 68,68,944x1304 --fill '#e61d2a'
pixelox shape path print.pxo --in-place --name Rings --d 'M 482 216 a 58 11 0 1 0 116 0 a 58 11 0 1 0 -116 0 M 483 230 a 57 11 0 1 0 114 0 a 57 11 0 1 0 -114 0 M 484 244 a 56 11 0 1 0 112 0 a 56 11 0 1 0 -112 0 M 486 258 a 54 11 0 1 0 108 0 a 54 11 0 1 0 -108 0 M 486 272 a 54 11 0 1 0 108 0 a 54 11 0 1 0 -108 0 M 487 286 a 53 11 0 1 0 106 0 a 53 11 0 1 0 -106 0 M 488 300 a 52 11 0 1 0 104 0 a 52 11 0 1 0 -104 0 M 488 314 a 52 11 0 1 0 104 0 a 52 11 0 1 0 -104 0 M 488 328 a 52 11 0 1 0 104 0 a 52 11 0 1 0 -104 0 M 487 341 a 53 11 0 1 0 106 0 a 53 11 0 1 0 -106 0 M 486 355 a 54 11 0 1 0 108 0 a 54 11 0 1 0 -108 0 M 486 369 a 54 11 0 1 0 108 0 a 54 11 0 1 0 -108 0 M 486 383 a 54 11 0 1 0 108 0 a 54 11 0 1 0 -108 0 M 483 397 a 57 11 0 1 0 114 0 a 57 11 0 1 0 -114 0 M 479 411 a 61 12 0 1 0 122 0 a 61 12 0 1 0 -122 0 M 474 425 a 66 12 0 1 0 132 0 a 66 12 0 1 0 -132 0 M 467 439 a 73 13 0 1 0 146 0 a 73 13 0 1 0 -146 0 M 461 453 a 79 14 0 1 0 158 0 a 79 14 0 1 0 -158 0 M 454 467 a 86 14 0 1 0 172 0 a 86 14 0 1 0 -172 0 M 449 481 a 91 15 0 1 0 182 0 a 91 15 0 1 0 -182 0 M 446 495 a 94 15 0 1 0 188 0 a 94 15 0 1 0 -188 0 M 444 509 a 96 16 0 1 0 192 0 a 96 16 0 1 0 -192 0 M 443 523 a 97 16 0 1 0 194 0 a 97 16 0 1 0 -194 0 M 440 537 a 100 16 0 1 0 200 0 a 100 16 0 1 0 -200 0 M 435 551 a 105 17 0 1 0 210 0 a 105 17 0 1 0 -210 0 M 429 565 a 111 17 0 1 0 222 0 a 111 17 0 1 0 -222 0 M 422 579 a 118 18 0 1 0 236 0 a 118 18 0 1 0 -236 0 M 415 592 a 125 19 0 1 0 250 0 a 125 19 0 1 0 -250 0 M 410 606 a 130 19 0 1 0 260 0 a 130 19 0 1 0 -260 0 M 407 620 a 133 20 0 1 0 266 0 a 133 20 0 1 0 -266 0 M 406 634 a 134 20 0 1 0 268 0 a 134 20 0 1 0 -268 0 M 407 648 a 133 20 0 1 0 266 0 a 133 20 0 1 0 -266 0 M 408 662 a 132 20 0 1 0 264 0 a 132 20 0 1 0 -264 0 M 410 676 a 130 19 0 1 0 260 0 a 130 19 0 1 0 -260 0 M 411 690 a 129 19 0 1 0 258 0 a 129 19 0 1 0 -258 0 M 412 704 a 128 19 0 1 0 256 0 a 128 19 0 1 0 -256 0 M 412 718 a 128 19 0 1 0 256 0 a 128 19 0 1 0 -256 0 M 413 732 a 127 19 0 1 0 254 0 a 127 19 0 1 0 -254 0 M 415 746 a 125 19 0 1 0 250 0 a 125 19 0 1 0 -250 0 M 416 760 a 124 19 0 1 0 248 0 a 124 19 0 1 0 -248 0 M 416 774 a 124 19 0 1 0 248 0 a 124 19 0 1 0 -248 0 M 415 788 a 125 19 0 1 0 250 0 a 125 19 0 1 0 -250 0 M 412 802 a 128 19 0 1 0 256 0 a 128 19 0 1 0 -256 0 M 409 816 a 131 19 0 1 0 262 0 a 131 19 0 1 0 -262 0 M 406 830 a 134 20 0 1 0 268 0 a 134 20 0 1 0 -268 0 M 404 843 a 136 20 0 1 0 272 0 a 136 20 0 1 0 -272 0 M 404 857 a 136 20 0 1 0 272 0 a 136 20 0 1 0 -272 0 M 405 871 a 135 20 0 1 0 270 0 a 135 20 0 1 0 -270 0 M 407 885 a 133 20 0 1 0 266 0 a 133 20 0 1 0 -266 0 M 410 899 a 130 19 0 1 0 260 0 a 130 19 0 1 0 -260 0 M 411 913 a 129 19 0 1 0 258 0 a 129 19 0 1 0 -258 0 M 412 927 a 128 19 0 1 0 256 0 a 128 19 0 1 0 -256 0 M 413 941 a 127 19 0 1 0 254 0 a 127 19 0 1 0 -254 0 M 416 955 a 124 19 0 1 0 248 0 a 124 19 0 1 0 -248 0 M 420 969 a 120 18 0 1 0 240 0 a 120 18 0 1 0 -240 0 M 424 983 a 116 18 0 1 0 232 0 a 116 18 0 1 0 -232 0 M 427 997 a 113 17 0 1 0 226 0 a 113 17 0 1 0 -226 0 M 428 1011 a 112 17 0 1 0 224 0 a 112 17 0 1 0 -224 0 M 427 1025 a 113 17 0 1 0 226 0 a 113 17 0 1 0 -226 0 M 424 1039 a 116 18 0 1 0 232 0 a 116 18 0 1 0 -232 0 M 421 1053 a 119 18 0 1 0 238 0 a 119 18 0 1 0 -238 0 M 418 1067 a 122 18 0 1 0 244 0 a 122 18 0 1 0 -244 0 M 416 1081 a 124 19 0 1 0 248 0 a 124 19 0 1 0 -248 0 M 416 1094 a 124 19 0 1 0 248 0 a 124 19 0 1 0 -248 0 M 413 1108 a 127 19 0 1 0 254 0 a 127 19 0 1 0 -254 0 M 410 1122 a 130 19 0 1 0 260 0 a 130 19 0 1 0 -260 0 M 406 1136 a 134 20 0 1 0 268 0 a 134 20 0 1 0 -268 0 M 404 1150 a 136 20 0 1 0 272 0 a 136 20 0 1 0 -272 0 M 404 1164 a 136 20 0 1 0 272 0 a 136 20 0 1 0 -272 0 M 407 1178 a 133 20 0 1 0 266 0 a 133 20 0 1 0 -266 0 M 411 1192 a 129 19 0 1 0 258 0 a 129 19 0 1 0 -258 0 M 412 1206 a 128 19 0 1 0 256 0 a 128 19 0 1 0 -256 0' --fill none --stroke '#ffffff' --stroke-width 2
pixelox shape path print.pxo --in-place --name RingsDense --d 'M 406 627 a 134 20 0 1 0 268 0 a 134 20 0 1 0 -268 0 M 406 641 a 134 20 0 1 0 268 0 a 134 20 0 1 0 -268 0 M 407 655 a 133 20 0 1 0 266 0 a 133 20 0 1 0 -266 0 M 409 669 a 131 19 0 1 0 262 0 a 131 19 0 1 0 -262 0 M 410 683 a 130 19 0 1 0 260 0 a 130 19 0 1 0 -260 0 M 412 697 a 128 19 0 1 0 256 0 a 128 19 0 1 0 -256 0 M 412 711 a 128 19 0 1 0 256 0 a 128 19 0 1 0 -256 0 M 413 725 a 127 19 0 1 0 254 0 a 127 19 0 1 0 -254 0 M 414 739 a 126 19 0 1 0 252 0 a 126 19 0 1 0 -252 0 M 415 753 a 125 19 0 1 0 250 0 a 125 19 0 1 0 -250 0 M 416 767 a 124 19 0 1 0 248 0 a 124 19 0 1 0 -248 0 M 415 781 a 125 19 0 1 0 250 0 a 125 19 0 1 0 -250 0' --fill none --stroke '#ffffff' --stroke-width 2
pixelox text add print.pxo --in-place --name Mark --content "Fizz Club" --font SignPainter --size 9% --color '#e61d2a'
pixelox transform scale print.pxo --in-place --layer Mark --size 244x
pixelox transform move print.pxo --in-place --layer Mark --to 540,700 --anchor c
pixelox fx stroke print.pxo --in-place --layer Mark --width 2 --color '#e61d2a' --position outside
pixelox shape path print.pxo --in-place --name ThreadTop --d 'M 812 68 C 806 140 748 196 690 200 C 650 203 626 198 598 214' --fill none --stroke '#ffffff' --stroke-width 2 --cap round
pixelox text add print.pxo --in-place --name YearTop --content "2031" --font "Snell Roundhand" --weight 500 --size 2.6% --color '#ffffff' --at 620,196 --anchor l
pixelox transform rotate print.pxo --in-place --layer YearTop --degrees -8
pixelox shape path print.pxo --in-place --name ThreadBottom --d 'M 540 1226 C 520 1250 470 1246 430 1272 C 392 1298 364 1336 352 1372' --fill none --stroke '#ffffff' --stroke-width 2 --cap round
pixelox text add print.pxo --in-place --name YearBottom --content "1931" --font "Snell Roundhand" --weight 500 --size 2.6% --color '#ffffff' --at 548,1246 --anchor l
pixelox transform rotate print.pxo --in-place --layer YearBottom --degrees -8
pixelox text add print.pxo --in-place --name Credit --content "(c) 2031 Your Company. All rights reserved. Ninety years of one shape, drawn by Your Studio" --font "Avenir Next" --weight 500 --size 1.05% --tracking 30 --color '#ffffff' --at 50%,50% --anchor c
pixelox transform rotate print.pxo --in-place --layer Credit --degrees -90
pixelox transform move print.pxo --in-place --layer Credit --to 991,1282 --anchor b
pixelox text add print.pxo --in-place --name CreditMark --content "Fizz Club" --font SignPainter --size 1.5% --color '#ffffff' --at 50%,50% --anchor c
pixelox transform rotate print.pxo --in-place --layer CreditMark --degrees -90
pixelox transform move print.pxo --in-place --layer CreditMark --to 991,1352 --anchor b
pixelox export print.pxo -o print.png
```

Change first: the ring data (your object), the two years, the wordmark; then the field
color. Keep the stroke at 2 px: thicker rings close up into a solid.

## 10. Variations and pairings

* Invert it: red rings on white paper, or white rings on black for a gallery poster.
* Two colors of ring: switch the stroke color for the band of rings where the label sits
  (draw that band as a second `shape path` in pale pink `#ffd2d2`).
* Other subjects: a wine glass for a tasting, a lighthouse for a coastal festival, a
  microphone for a podcast, a trophy for an awards night.
* Add a real title: one line of tracked caps (cap height 1.4% H) centered at 92% H inside
  the field, in white.
* Pairs with T-arc-type for an anniversary seal in a corner, and T-paper-grain. Avoid
  photographs: the charm is that everything is one line weight.

## 11. Pitfalls

* Rings too far apart (more than 2% H) and the object falls apart into separate hoops; too
  close (under 0.7% H) and it fills in. 50 to 80 rings over the object's height is right.
* Ellipses too tall (ry above 0.2 rx) make a stack of plates, not a transparent solid. Keep
  ry near one eighth of rx, plus a few pixels so the neck rings stay visible.
* A wordmark placed over the rings in white disappears into them. Use the field color and
  give it a same-color stroke so it reads as a gap in the coil.
* A thread that touches the object at a random place. It must enter at the mouth and
  leave at the base center, as if it had passed through.
* Unequal paper borders: compute the field from the same inset on all four sides (68 px
  here).
* Rebuilding a known brand's trade dress. A white script wordmark and ribbon on this exact red
  is one company's signature. Take the idea (what the object, the mark, and the ground do
  to each other) and build it with your own brand's colors, lettering, and shapes.
