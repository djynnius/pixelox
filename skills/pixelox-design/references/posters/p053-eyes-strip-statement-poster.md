---
id: p053
title: Eyes strip over giant statement type
genre: [film-poster, editorial]
mood: [serious, bold, intense, urgent]
style: [type-led, editorial, grunge]
format: portrait-2x3
archetype: A-type-block
palette: ['#030202', '#f3f3f2', '#8b8b8a', '#d7262c']
techniques: [T-fit-width, T-distress-type, T-mono-on-color, T-paper-grain]
needs: photo
---

# p053: Eyes strip over giant statement type

## 1. Concept

A documentary poster that is four fifths words: a black-and-white strip across the top
shows nothing but a man's two eyes, cropped from brow to cheekbone, and under it a
four-line statement in white heavy condensed capitals fills the black sheet from the left
edge to the bottom edge, growing wider line by line. The credits sit in red and white in
the notch that the two short top lines leave on the right. It works because the eyes look
straight at the viewer while the type speaks in the first person: the poster is a face
saying a sentence, with the mouth replaced by the words.

## 2. Format and grid

* Portrait 2:3. No left or bottom margin: the title bleeds off both. Right margin 5% W.
* Zones: photo strip 0 to 20.7% H, full width; title block 22.5 to 100% H; credit column
  59.6 to 95% W, 23 to 45% H.
* Title lines share one size (cap height about 18% H) and one left edge (0% W). Their cap
  tops are at 22.5%, 40.8%, 61%, and 81.1% H; the gap between lines is 1.5 to 2% H. The
  last line is cut by the bottom edge after about 95% of its cap height.
* Line widths step out to the right: 57% W, 57% W, 77% W, 95% W. That staircase is what
  creates the credit notch at the top right.
* The credit column is left-aligned on one edge at 59.6% W, directly above the point
  where line 3 ends.

## 3. Layer stack

1. Ground: black `#030202`, full canvas.
2. Photo strip, 0 to 20.7% H: an extreme close-up of a man's eyes in black and white,
   looking at the camera, heavy film grain, mid-gray skin `#8b8b8a`, bright catchlights;
   the brows touch the top edge. The strip ends in a hard straight edge.
3. Title: 4 lines, 2 words / 1 / 1 / 1, white `#f3f3f2`, heavy condensed grotesque caps,
   flush left at the canvas edge, sizes and positions as above.
4. Scratches in the title: fine black nicks, hairline scratches, and specks inside the
   white letters, covering 3 to 5% of their area, as on worn film leader.
5. Credit column (59.6% W): a 2-line pull quote in small tracked red caps with a tiny gray
   source line; then two pairs of "label in white, name in red" (label cap height 1.2% H,
   name 2.3% H); then a tiny white label and a red name at 1.3% H; then a 3-line gray
   billing block at 0.45% H.
6. Film grain over everything.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#030202` | ground | 55% |
| `#f3f3f2` | title, labels | 30% |
| `#8b8b8a` | photo mid-tones, billing | 13% |
| `#d7262c` | names, quote | 2% |

Black, white, and one red used only for the names of people. Red is under 2% of the area
and is still the second thing read.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Title (4 lines) | a 5-word first-person statement, 2 to 5 letters per word, split 2 / 1 / 1 / 1 | caps | heavy condensed grotesque (Barlow Condensed 800, Oswald 700, DIN-style bold / Helvetica Neue condensed black) | 900 | 18% | -10 | cap height + 1.5 to 2% H | left, flush to the edge | `#f3f3f2` | 0% W; cap tops 22.5, 40.8, 61, 81.1% |
| Quote | 2 lines, 10 words, in double quotes | caps | DIN-style sans (Barlow / DIN Alternate) | 700 | 0.8% | 150 | 160% | left | `#d7262c` | 59.6%, 23.1 and 24.4% |
| Quote source | critic, publication | caps | same | 700 | 0.45% | 100 | - | left | `#b8b8b8` | 59.6%, 25.7% |
| Label (x2) | "WRITTEN BY", "DIRECTED BY" | caps | DIN-style condensed (Barlow Condensed 600 / DIN Condensed) | 700 | 1.2% | 60 | - | left | `#f3f3f2` | 59.6%, 28.1 and 34.2% |
| Name (x2) | `[WRITER NAME]`, `[DIRECTOR NAME]` | caps | same | 700 | 2.3% | 20 | - | left | `#d7262c` | 59.6%, 31 and 36.9% |
| Small label | 4 words introducing a narrator | caps | same | 700 | 0.5% | 80 | - | left | `#f3f3f2` | 59.6%, 39.4% |
| Narrator | `[ACTOR NAME]` | caps | same | 700 | 1.3% | 20 | - | left | `#d7262c` | 59.6%, 40.5% |
| Billing | 3 lines of credits | caps | same | 400 | 0.45% | 20 | 140% | left | `#8b8b8a` | 59.6%, 42 to 45% |

One family (a DIN-like grotesque) in two widths. The title is 8 times the cap height of
the largest name; there is no middle size.

## 6. Imagery

One photograph, used as a narrow band: both eyes, the bridge of the nose, and a little of
each brow, cropped so tightly that the person is not shown, only the gaze. Black and white
(**T-mono-on-color** settings without the color: `adjust bw`, then `adjust levels
--in-black 20 --in-white 225`), with visible grain. Use an image at least 1600 px wide so
the crop stays sharp. Any direct gaze works; closed or averted eyes do not.

## 7. Effects and textures

* Strip crop: the photo is placed wide and moved until the eyes sit in the top 20.7% H;
  a black rectangle (the Field layer) covers everything below.
* **T-fit-width**, relaxed: one font size for all four lines; the words are chosen so the
  line widths grow, not scaled to equal width.
* **T-distress-type**: the four lines are merged into one layer; grunge in the ground
  black is clipped to it (`specks=4500`, `blotches=8`, `scratches=120`, `clump=0.35`).
* **T-paper-grain**, strong: amount 22, overlay, 45%, coarsened with a 0.6 px blur.

## 8. Hierarchy and eye path

1. The eyes (a face always wins, and it is at the top).
2. The statement, read down the left edge; each line is wider, so the sentence
   accelerates.
3. The red names in the notch.
4. Quote and billing.

Cap heights: title 18 : names 2.3 : labels 1.2 : quote 0.8, about 22 : 3 : 1.5 : 1.

## 9. Build recipe

`photo.png` is your image here: a close portrait; move the Eyes layer so the eyes fall in
the top strip. Choose title words so that lines 1 and 2 stay under 57% W.

```bash
pixelox render js scripts/grunge.js -o wear.png --size 1080x1620 --seed 14 --param color='#030202' --param specks=4500 --param blotches=8 --param scratches=120 --param clump=0.35
pixelox new -o doc.pxo --size 1080x1620 --fill '#030202'
pixelox comp place doc.pxo photo.png --in-place --as Eyes --size 1500x --at 50%,0 --anchor t
pixelox transform move doc.pxo --in-place --layer Eyes --by 0,-620
pixelox adjust bw doc.pxo --in-place --layer Eyes
pixelox adjust levels doc.pxo --in-place --layer Eyes --in-black 20 --in-white 225
pixelox shape rect doc.pxo --in-place --name Field --rect 0,20.7%,100%x79.3% --fill '#030202'
pixelox text add doc.pxo --in-place --name L1 --content "I DO" --font "Helvetica Neue" --weight 900 --size 24.2% --tracking -10 --color '#f3f3f2'
pixelox transform move doc.pxo --in-place --layer L1 --to 0,22.5% --anchor tl
pixelox text add doc.pxo --in-place --name L2 --content "NOT" --font "Helvetica Neue" --weight 900 --size 24.2% --tracking -10 --color '#f3f3f2'
pixelox transform move doc.pxo --in-place --layer L2 --to 0,40.8% --anchor tl
pixelox text add doc.pxo --in-place --name L3 --content "FEAR" --font "Helvetica Neue" --weight 900 --size 24.2% --tracking -10 --color '#f3f3f2'
pixelox transform move doc.pxo --in-place --layer L3 --to 0,61% --anchor tl
pixelox text add doc.pxo --in-place --name L4 --content "NIGHT" --font "Helvetica Neue" --weight 900 --size 24.2% --tracking -10 --color '#f3f3f2'
pixelox transform move doc.pxo --in-place --layer L4 --to 0,81.1% --anchor tl
pixelox layer merge doc.pxo --in-place --layers L1,L2,L3,L4 --name Title
pixelox comp place doc.pxo wear.png --in-place --as Wear --fit cover
pixelox layer clip doc.pxo --in-place --layer Wear
pixelox text add doc.pxo --in-place --name Quote --content $'"A FILM THAT LISTENS BEFORE\nIT SPEAKS. DO NOT MISS IT."' --font "DIN Alternate" --weight 700 --size 1.1% --tracking 150 --leading 160% --color '#d7262c' --at 59.6%,23.8% --anchor l
pixelox text add doc.pxo --in-place --name Source --content "CRITIC NAME, THE DAILY PAPER" --font "DIN Alternate" --weight 700 --size 0.62% --tracking 100 --color '#b8b8b8' --at 59.6%,25.9% --anchor l
pixelox text add doc.pxo --in-place --name Label1 --content "WRITTEN BY" --font "DIN Condensed" --weight 700 --size 1.65% --tracking 60 --color '#f3f3f2' --at 59.6%,28.3% --anchor l
pixelox text add doc.pxo --in-place --name Name1 --content "WRITER NAME" --font "DIN Condensed" --weight 700 --size 3.15% --tracking 20 --color '#d7262c' --at 59.6%,31.2% --anchor l
pixelox text add doc.pxo --in-place --name Label2 --content "DIRECTED BY" --font "DIN Condensed" --weight 700 --size 1.65% --tracking 60 --color '#f3f3f2' --at 59.6%,34.4% --anchor l
pixelox text add doc.pxo --in-place --name Name2 --content "DIRECTOR NAME" --font "DIN Condensed" --weight 700 --size 3.15% --tracking 20 --color '#d7262c' --at 59.6%,37.1% --anchor l
pixelox text add doc.pxo --in-place --name Label3 --content "WITH THE VOICE OF" --font "DIN Condensed" --weight 700 --size 0.75% --tracking 80 --color '#f3f3f2' --at 59.6%,39.5% --anchor l
pixelox text add doc.pxo --in-place --name Name3 --content "NARRATOR NAME" --font "DIN Condensed" --weight 700 --size 1.8% --tracking 20 --color '#d7262c' --at 59.6%,40.7% --anchor l
pixelox text add doc.pxo --in-place --name Billing --content $'A STUDIO NAME RELEASE   PRODUCED BY FIRST PRODUCER\nEDITED BY EDITOR NAME   MUSIC BY COMPOSER NAME\nCINEMATOGRAPHY CAMERA NAME   IN CINEMAS THIS SPRING' --font "DIN Condensed" --weight 400 --size 0.62% --tracking 20 --leading 140% --color '#8b8b8a' --at 59.6%,43.4% --anchor l
pixelox layer add doc.pxo --in-place --name Grain --fill '#808080'
pixelox filter add-noise doc.pxo --in-place --layer Grain --amount 22 --monochrome
pixelox filter gaussian doc.pxo --in-place --layer Grain --sigma 0.6
pixelox layer blend doc.pxo --in-place --layer Grain --mode overlay
pixelox layer opacity doc.pxo --in-place --layer Grain --value 45
pixelox export doc.pxo -o doc.png
```

Change first: the four title lines (keep the widths growing), the photo crop, then the
single accent color.

## 10. Variations and pairings

* Other strips: a mouth, two hands, a horizon, a row of windows, a crowd's raised fists.
  Always a band of 18 to 24% H, hard-edged, black and white.
* Strip at the bottom and the staircase reversed (widest line first, notch bottom right)
  for an answer or sequel poster.
* Accent swaps: yellow `#f5d000` for a labor or sport story; blue `#2f6bff` for a tech
  documentary. Keep it under 3% of the area.
* Light version: white ground, black title, the strip unchanged.
* Campaign, protest, lecture series, memoir cover, podcast episode art (square: strip 25%,
  three title lines).
* Pairs with T-halftone-portrait on the strip, T-rules (a hairline under the strip), and
  T-glitch on one title line.

## 11. Pitfalls

* Showing the whole face: it becomes a portrait poster and the type has to shrink. Crop
  to the eyes; if the strip grows past 25% H it is too much.
* Scaling every line to full width (strict T-fit-width): the notch for the credits
  disappears and the short lines become absurdly tall. One size, ragged right.
* Title set with normal leading: the block falls apart into four words. Gaps between
  lines should be 8 to 12% of the cap height.
* Credits scattered around the sheet. All small text lives in the notch on one left edge.
* Red on more than names and one quote: it stops being a signal.
