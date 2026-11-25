---
id: i033
title: Nine numbered nodes on a thin ring with a center picture
kind: [cycle, process]
topic: [marketing, business, generic]
mood: [friendly, clean, playful, editorial]
style: [flat, illustrated, 3d]
format: square
structure: S-cycle-ring
data: [cycle, process, text-only]
charts: [C-cycle, C-pyramid, C-title-block]
palette: ['#f2f2f2', '#ffffff', '#4b43ba', '#2d9cdb', '#111111', '#f4cf35']
needs: nothing
---

# i033: Nine numbered nodes on a thin ring with a center picture

## 1. Concept

A near-square cycle diagram. Nine numbered discs in three alternating colors (indigo, sky
blue, black) sit on a thin black ring that runs clockwise from the top and ends, in the
upper left, in one large black arrowhead that points back at step 1. Every disc has a short
bold title and a handwritten-style note outside the ring; a small 3D scene (a stepped podium
with ladders and a trophy) stands in the middle; the title fills the gap in the upper left,
its last word on a yellow brush stroke. The message is "nine steps, then start again". It
works because the ring is drawn as a line, not as a shape: the discs and the type stay the
heroes, the big arrowhead says "loop" once and clearly, and the three-color rhythm lets the
eye count in threes.

## 2. Data story

No numbers except the step numbers. Text only, as a loop:

* 9 steps in a fixed order that repeats. Each: a title of 1 to 2 words and a note of 2 to 6
  words on 1 to 3 lines.
* 1 title of 2 to 3 words on two lines plus one accent word on a color swatch.
* The disc colors do not encode groups: they repeat indigo, blue, black purely for rhythm.

Placeholder content in the recipe (invented, planning a newsletter): Set goals, Study, Pick
topics, Write, Build, Send a test, Collect, Report, Improve.

## 3. Information architecture

1. **Title**, upper left, inside the ring's open corner: two heavy lines and the accent word
   on its yellow stroke.
2. **Step 1** at 12 o'clock; steps 2 to 9 clockwise at equal spacing.
3. **Arrowhead** after step 9, at about 10 to 11 o'clock: the path returns to step 1.
4. **Center picture**: what the loop builds toward (a podium, a trophy on top).
5. **Labels** outside the ring, each on the outer side of its disc: right of the discs on
   the right half, left of them on the left half, above step 1, below the bottom step.

The numbers give the order; the ring and the arrowhead give the repetition. Nothing sits
between a disc and its label, so no leader lines are needed.

## 4. Format and grid

* Near square, 1 : 1.035. Recipe canvas 1080 x 1120. Text margin 6% W.
* Ring: center 50% W, 53.7% H; radius 25.6% W (277 px). Ten equal slots of 36 degrees: nine
  discs from 12 o'clock clockwise (0, 36, ... 288 degrees) and the arrowhead in the tenth
  slot (324 degrees).
* Discs: 8% W across (86 px).
* Labels: start 62 px from the disc center, away from the ring; each block up to 17% W wide.
  Right-half labels flush left, left-half labels flush right.
* Title block `6.3%,9%,38% x 24%`: two lines at 9% and 16% H, the swatch `5.5%,25.5%,25% x
  7%`. It occupies the corner the arrowhead leaves free.
* Center picture: 24% W wide, from 34% to 62% H, centered on the ring's center.
* Ground: a pale gray field covers the upper left two thirds; its edge is one long curve
  from the top right corner down through the middle to the lower left, leaving white at the
  right and bottom.

## 5. Layer stack

1. Ground: white.
2. Gray wave `#f2f2f2`: a soft organic shape from the top edge to the lower left.
3. Ring: a 2.6 px black line with a small arrowhead in front of every disc (the original: a
   plain hairline with no small arrowheads).
4. Nine discs: indigo `#4b43ba` (1, 4, 7), sky blue `#2d9cdb` (2, 5, 8), black `#111111`
   (3, 6, 9), each with a white heavy digit.
5. One large black triangle, 76 px across, in the tenth slot, pointing along the ring toward
   step 1.
6. Center picture (the original: a 3D render; the recipe: a four-tier pyramid from teal to
   indigo with a trophy and a small red flag above it).
7. Title in near-black `#222222`; yellow stroke `#f4cf35`; accent word in dark gray.
8. Nine label titles in black and nine notes in dark gray `#2b2b2b`.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#f2f2f2` | gray wave | 52% | nothing |
| `#ffffff` | ground, digits | 36% | nothing |
| `#4b43ba` | discs 1, 4, 7; podium base | 3% | nothing: rhythm (first of each three) |
| `#2d9cdb` | discs 2, 5, 8; podium middle (`#44bad8`, `#3f8fd9`) | 3% | nothing: rhythm |
| `#111111` | discs 3, 6, 9; ring; arrowhead; type | 5% | nothing: structure |
| `#f4cf35` | title swatch | 1% | emphasis: the one accent |

Two cool hues and black in strict rotation, on neutrals, with one warm accent that appears
once (the swatch). Because the three disc colors mean nothing, the design can also use them
to mark three phases (steps 1 to 3, 4 to 6, 7 to 9): then group them instead of alternating.

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Title | 2 to 3 words on 2 lines, up to 10 letters a line | title | heavy wide grotesque (Archivo Black, Inter 900 / Arial Black) | 900 | 62 | 0 | 80 px | left | `#222222` | 6.3% W, from 9% H |
| Accent word | 1 word, 5 to 8 letters | title, slanted | marker, thin and slanted (the original: a light handwritten italic; Caveat, Patrick Hand / Marker Felt) | 400 | 60 | 0 | - | left | `#2b2b2b` | on the swatch, 9.6% W, 28.8% H |
| Disc digit (x9) | 1 to 9 | - | heavy wide grotesque | 900 | 40 (drawn by the script: 0.46 x the disc) | 0 | - | center | `#ffffff` | disc centers |
| Label title (x9) | 1 to 2 words | sentence | heavy wide grotesque | 900 | 19 (the original: 17) | 0 | - | away from the ring | `#111111` | 62 px outside the disc center |
| Label note (x9) | 2 to 6 words on 2 lines | sentence | marker, thin (Caveat, Patrick Hand / Marker Felt) | 400 | 23 (the original: 21, italic) | 0 | 29 px | as its title | `#2b2b2b` | 32 px under the title's top |

Two families with opposite characters: a black-weight wide grotesque for names and numbers,
a light hand for notes. Sizes 62 : 40 : 23 : 19. The label titles are smaller than their
notes but four times heavier; weight, not size, makes them titles.

## 8. Charts and diagrams

* **C-cycle**: `scripts/cycle.js` (`style=nodes`), ten blank labels, `labelwidth=0`,
  `node=86`, `fs=10`, `box=20,24.4,60,58.6`, `badges='1,2,3,4,5,6,7,8,9, '`,
  `colors=` the three hues three times and `none` for the tenth, `line='#111111'`, digits in
  a heavy stack (`numfont=`). The tenth node is invisible: its slot holds the big arrowhead.
  The script logs the center (540,601), the radius 277, and the ten node centers: (540,324),
  (703,377), (804,516), (804,687), (703,826), (540,879), (377,826), (276,687), (276,516),
  and the empty slot (377,377).
* The big arrowhead: `shape polygon --sides 3 --radius 44 --rotation 54` at the tenth
  anchor (54 = the slot's angle, 324, plus 90: the clockwise direction of travel there).
* Labels are native text because they mix two type families; with one family, give
  `labels=` and `notes=` to the script instead (see Variations).
* **C-pyramid** as the center picture: `scripts/pyramid.js`, four blank tiers, `labelpos=in
  gap=0`, `box=38,43,24,19`, four colors from pale teal to indigo.
* No axis, no values, no legend.
* Simplified: the original spreads nine discs unevenly over about 270 degrees and leaves a
  quarter of the ring for one sweeping arrow; the recipe uses ten even slots. The original
  ring has no small arrowheads.
* **3D note** (for a later three.js build): the center is a stepped podium of four square
  slabs, each smaller and paler than the one below (indigo `#4d47ca` base, then `#3f8fd9`,
  `#44bad8`, pale teal `#a8e6e0`), seen from the front and about 15 degrees above, turned 20
  degrees; light from the upper left (top faces lightest, right faces darkest); two wooden
  ladders lean on it, a silver trophy and a red pennant stand on top, three small yellow
  figures wait at its foot. Depth encodes nothing.

## 9. Icons and illustration

* Above the pyramid: `trophy`, 72 px, gray `#8a8f98` line with a pale fill; `flag-triangle-
  right`, 44 px, red `#e8563f`.
* For the original's look supply a 3D or drawn scene as a transparent PNG, 300 to 380 px
  wide, centered on the ring's center, and drop the pyramid, trophy, and flag.
* The yellow swatch is a four-point `shape path` with a round-joined stroke in its own
  color: a brush stroke with soft corners, tilted by 6 px.
* The gray wave is one `shape path` with three curves.

## 10. Effects and finish

None: flat color. The only depth is in the center picture. No shadows under the discs.

## 11. Hierarchy and eye path

1. The title (largest, blackest), upper left.
2. Disc 1 at the top, directly right of the title; then clockwise along the ring, counting.
3. The big arrowhead, which sends the eye back to the title and disc 1.
4. The center picture, seen in passing; the notes, read step by step.

Type steps 62 : 40 : 23 : 19. The ring line is the lightest element on the page and still
organizes everything.

## 12. Build recipe

Nothing to supply. Canvas 1080 x 1120. All copy is invented.

```bash
pixelox new -o loop.pxo --size 1080x1120 --fill '#ffffff'
pixelox shape path loop.pxo --in-place --name Wave --d 'M 0 0 L 1080 0 L 1080 150 C 930 230 800 350 775 530 C 750 680 600 740 400 770 C 240 795 110 900 0 1053 Z' --fill '#f2f2f2'
pixelox render js scripts/cycle.js -o ring.png --size 1080x1120 --param box=20,24.4,60,58.6 --param labels=' | | | | | | | | | ' --param labelwidth=0 --param node=86 --param fs=10 --param badges='1,2,3,4,5,6,7,8,9, ' --param colors='#4b43ba,#2d9cdb,#111111,#4b43ba,#2d9cdb,#111111,#4b43ba,#2d9cdb,#111111,none' --param line='#111111' --param numfont='Arial Black, Archivo Black, Avenir Next, sans-serif'
pixelox comp place loop.pxo ring.png --in-place --as Ring --at 0,0
pixelox shape polygon loop.pxo --in-place --name Arrow --center 377,377 --radius 44 --sides 3 --rotation 54 --fill '#111111'
pixelox render js scripts/pyramid.js -o steps.png --size 1080x1120 --param box=38,43,24,19 --param labels=' | | | ' --param labelpos=in --param gap=0 --param colors='#a8e6e0,#44bad8,#3f8fd9,#4d47ca'
pixelox comp place loop.pxo steps.png --in-place --as Podium --at 0,0
pixelox shape icon loop.pxo trophy --in-place --name Trophy --size 72 --at 540,446 --anchor c --color '#8a8f98' --fill '#d9dde3' --stroke-width 5
pixelox shape icon loop.pxo flag-triangle-right --in-place --name Flag --size 44 --at 566,388 --anchor c --color '#e8563f' --fill '#e8563f' --stroke-width 4
pixelox text add loop.pxo --in-place --name Title --content $'Newsletter\nPlanning' --font @heavy-wide --size 62 --leading 80 --color '#222222' --at 68,100 --anchor tl
pixelox shape path loop.pxo --in-place --name Swatch --d 'M 62 290 L 322 284 L 326 354 L 58 360 Z' --fill '#f4cf35' --stroke '#f4cf35' --stroke-width 10 --join round
pixelox text add loop.pxo --in-place --name Word --content "Cycle" --font @marker --weight 400 --style italic --size 60 --color '#2b2b2b' --at 104,322 --anchor l
pixelox text add loop.pxo --in-place --name Head1 --content "Set goals" --font @heavy-wide --size 19 --color '#111111' --at 534,174 --anchor tl
pixelox text add loop.pxo --in-place --name Note1 --content $'and agree what a\ngood result is' --font @marker --weight 400 --size 23 --leading 29 --align left --color '#2b2b2b' --at 534,206 --anchor tl
pixelox text add loop.pxo --in-place --name Head2 --content "Study" --font @heavy-wide --size 19 --color '#111111' --at 765,333 --anchor tl
pixelox text add loop.pxo --in-place --name Note2 --content $'the readers and\nwhat they miss' --font @marker --weight 400 --size 23 --leading 29 --align left --color '#2b2b2b' --at 765,365 --anchor tl
pixelox text add loop.pxo --in-place --name Head3 --content "Pick topics" --font @heavy-wide --size 19 --color '#111111' --at 866,472 --anchor tl
pixelox text add loop.pxo --in-place --name Note3 --content $'for the next\nsix issues' --font @marker --weight 400 --size 23 --leading 29 --align left --color '#2b2b2b' --at 866,504 --anchor tl
pixelox text add loop.pxo --in-place --name Head4 --content "Write" --font @heavy-wide --size 19 --color '#111111' --at 866,643 --anchor tl
pixelox text add loop.pxo --in-place --name Note4 --content $'one draft\nevery week' --font @marker --weight 400 --size 23 --leading 29 --align left --color '#2b2b2b' --at 866,675 --anchor tl
pixelox text add loop.pxo --in-place --name Head5 --content "Build" --font @heavy-wide --size 19 --color '#111111' --at 765,812 --anchor tl
pixelox text add loop.pxo --in-place --name Note5 --content $'a simple page\ntemplate' --font @marker --weight 400 --size 23 --leading 29 --align left --color '#2b2b2b' --at 765,844 --anchor tl
pixelox text add loop.pxo --in-place --name Head6 --content "Send a test" --font @heavy-wide --size 19 --color '#111111' --at 476,941 --anchor tl
pixelox text add loop.pxo --in-place --name Note6 --content $'issue to twenty\nfriendly readers' --font @marker --weight 400 --size 23 --leading 29 --align left --color '#2b2b2b' --at 476,973 --anchor tl
pixelox text add loop.pxo --in-place --name Head7 --content "Collect" --font @heavy-wide --size 19 --color '#111111' --at 315,812 --anchor tr
pixelox text add loop.pxo --in-place --name Note7 --content $'replies and\nopen rates' --font @marker --weight 400 --size 23 --leading 29 --align right --color '#2b2b2b' --at 315,844 --anchor tr
pixelox text add loop.pxo --in-place --name Head8 --content "Report" --font @heavy-wide --size 19 --color '#111111' --at 214,643 --anchor tr
pixelox text add loop.pxo --in-place --name Note8 --content $'what worked\nto the team' --font @marker --weight 400 --size 23 --leading 29 --align right --color '#2b2b2b' --at 214,675 --anchor tr
pixelox text add loop.pxo --in-place --name Head9 --content "Improve," --font @heavy-wide --size 19 --color '#111111' --at 214,472 --anchor tr
pixelox text add loop.pxo --in-place --name Note9 --content $'repeat, and\nraise the bar' --font @marker --weight 400 --size 23 --leading 29 --align right --color '#2b2b2b' --at 214,504 --anchor tr
pixelox export loop.pxo -o loop.png
```

Reuse: change the title, then the nine label pairs. Node centers come from
`lab/anchors.py scripts/cycle.js --size 1080x1120 --param box=20,24.4,60,58.6 --param
labels=' | | | | | | | | | ' --param labelwidth=0 --param node=86 --param fs=10`. Label rule
for a disc at (x, y): right half, title at (x + 62, y - 44) flush left; left half, title at
(x - 62, y - 44) flush right; the discs next to the bottom one use y - 14; step 1's label
sits above it at (x - 6, y - 150); the bottom label below at (x - 64, y + 62); every note is
32 px under its title. For N steps give N + 1 blank labels, N badges and a blank, N colors
and `none`, and rotate the triangle by 450 - 360 / (N + 1) degrees.

## 13. Variations and pairings

* **Script labels:** drop the native labels, set `labelwidth=0.2 fs=22`, and pass `labels=`
  and `notes=`: one family, automatic wrapping, 20 fewer commands.
* **Fewer steps:** 5 or 6 discs with `node=110` and longer notes (up to 10 words).
* **Phases:** color discs 1 to 3 indigo, 4 to 6 blue, 7 to 9 black and add a three-key
  C-legend under the title.
* **Dark ground:** ground `#15151c`, wave `#1f1f2b`, ring and type white, black discs become
  white with dark digits.
* Pairs with C-stat-row under the ring on a 1080 x 1350 canvas (what one turn of the loop
  delivers) and with C-bulb or C-rocket as the center picture.

## 14. Pitfalls

* **Nine steps with long notes.** At 36 degrees a slot is 174 px of arc: a label wider than
  17% W or taller than three lines hits its neighbor. Keep notes to 6 words; with more to
  say, use 6 steps.
* **A cycle that does not cycle.** If step 9 does not lead back to step 1, the arrowhead
  states something false. For a one-way process use C-process-spine or C-timeline. This is
  the data-integrity rule for loops.
* **Colors that seem to group.** Three alternating colors invite the question "what do the
  blue ones share?". Either say nothing depends on color (keep the strict rotation) or make
  the colors real groups and add a legend.
* **Labels inside the ring.** Text between the discs and the center picture breaks the
  ring's clean inside and collides with the picture. All labels go outside.
* **Title under the ring.** The ring's free corner is the title's place; a title above the
  whole diagram pushes the ring down and wastes that corner.
* **Synthetic italics.** A marker face has no italic; the slant is synthesized (warning
  PXW4002). It is fine for one accent word, not for the notes: set those upright.
