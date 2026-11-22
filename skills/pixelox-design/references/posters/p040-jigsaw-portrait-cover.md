---
id: p040
title: Jigsaw portrait magazine cover
genre: [magazine-cover, editorial]
mood: [playful, bold, calm, friendly]
style: [photo-composite, editorial, collage]
format: portrait-3x4
archetype: A-cover-masthead
palette: ['#c6c5c3', '#c18462', '#2a221c', '#f0efec', '#857467']
techniques: [T-text-behind-subject, T-fit-width, T-pattern, T-cutout-drop-shadow, T-rules]
needs: [photo, subject-cutout]
---

# p040: Jigsaw portrait magazine cover

## 1. Concept

A culture-magazine cover built on one visual joke: the cover star's face is a huge,
half-finished jigsaw puzzle lying on a gray table of blank pieces, and a tiny second
figure of the same person, seen from above, reaches up to press the next piece into his
own cheek. It works because the idea is told entirely by the picture (the hook line only
confirms it), the palette is just gray plus skin, and the masthead and the two small
cover lines stay white and quiet so nothing competes with the gag.

## 2. Format and grid

* Portrait, close to 3:4 (1 : 1.35). Margins 3.5% W left and right.
* Masthead zone 4.4 to 18.2% H, spanning 3.4 to 96.2% W, with a tiny credit line above
  its first letters at 3% H.
* The puzzle face fills the middle: 16 to 82% W, hair top at 16.5% H (its top pieces
  overlap the bottom of the masthead), eyes at 44% H, chin at 73% H, neck to the bottom
  edge. The face is cropped tight: forehead to chin only, 66% W wide.
* Two small cover-line blocks at the same height, 21 to 32% H, each fenced by a hairline
  above and below: the star's name at the left margin, a 3-line hook right-aligned at the
  right margin. They sit in the gray corners beside the forehead.
* The small figure stands in the bottom right: 52 to 90% W, 70 to 100% H, one arm raised
  to 57% H. Loose pieces collect in the bottom left corner, along the right edge, and at
  the top center.
* An implied piece grid of about 9 columns by 12 rows (each piece 11% W) runs under
  everything.

## 3. Layer stack

1. Table: light warm gray `#c6c5c3`, lighter (`#d4d3d1`) in the middle, darker
   (`#898783`) toward the bottom left; faint embossed outlines of interlocking blank
   pieces cover it.
2. Credit line: tiny white caps, 5.1 to 14.4% W, 3% H.
3. Masthead: 7 letters, white `#f0efec`, chunky rounded geometric capitals set very
   tight, `3.4%,4.4%,92.8% x 13.8%`, with a soft gray shadow under it.
4. Face: a front-on portrait photo (short dark hair, level gaze, light stubble) shown
   only inside a jigsaw-edged window: straight piece edges with round knobs sticking out
   and round sockets cut in, a stepped outline (narrower at the hair and the neck).
   Thin dark cut lines with a light bevel divide the face into pieces.
5. Missing pieces: three or four piece-shaped holes inside and at the edge of the face
   that show the gray table, each with a small inner shadow. The largest is on the right
   cheek, 68 to 80% W, 47 to 60% H.
6. Loose pieces: single white and gray blank pieces, each rotated differently, with
   short soft shadows; a pile at `0,62%,20% x 38%`, a scatter at `78%,33%,22% x 42%`,
   a few at `35%,0,27% x 6%`.
7. Held piece: one skin-colored piece at 78 to 92% W, 53 to 60% H, tilted 25 degrees,
   on its way into the cheek hole.
8. Small figure: a man seen from above and behind, striped gray shirt, headphones around
   his neck, right arm raised to the held piece; soft shadow on the table.
9. Left block: hairline at 20.9% H (3.5 to 17% W), the name in two thin condensed
   lines, hairline at 31.9% H.
10. Right block: hairline at 20.9% H (81 to 97.3% W), three right-aligned thin
    condensed lines, hairline at 31.9% H.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#c6c5c3` | gray table and blank pieces | 45% |
| `#c18462` | skin (photo) | 28% |
| `#2a221c` | hair, brows, headphones | 12% |
| `#f0efec` | masthead, cover lines, lit piece faces | 10% |
| `#857467` | shadows, shirt stripes | 5% |

Neutral gray plus one warm skin tone. There is no accent color at all: the face is the
color.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Credit | possessive name, 2 words | caps | geometric sans (Montserrat / Avenir Next) | 700 | 0.9% | 0 | - | left | `#f0efec` | 5.1%, 3% |
| Masthead | `[MASTHEAD]`, 7 letters | caps (unicase: the E is drawn as a large lowercase form) | chunky rounded geometric display (Fredoka 700, Baloo 2 800, Righteous / Avenir Next Heavy plus a stroke) | 900 | 13.8% | -50 | - | center | `#f0efec` | 50%, cap top 4.4% |
| Star name | `[COVER STAR]`, 2 lines of 3 to 5 letters | caps | thin condensed gothic (Oswald 200, Bebas Neue Light / Avenir Next Condensed Light) | 300 | 4.2% | 0 | 84% | left | `#f0efec` | 3.5%, cap top 21.8% |
| Hook | 3 lines, 1 to 2 words each, a pun on the picture | caps | same | 300 | 2.4% | 0 | 92% | right | `#f0efec` | 97.2%, cap top 21.9% |

White on light gray is low contrast on purpose: the cover lines are found second. The
masthead is 3.3x the name and 5.8x the hook.

## 6. Imagery

Two photographs of the same person. The first is a tight studio head shot, front-on and
evenly lit, enlarged until the face alone is two thirds of the cover width. The second
is a full figure shot from directly above, reduced to a quarter of the cover height.
The scale jump between the two (about 8 to 1) is the joke; the shadow under the small
figure and under every loose piece is what makes the big face read as a flat puzzle on a
table. For a build, the face is `photo.png` clipped by a vector mask and the figure a
cutout (**T-cutout-drop-shadow**, soft variant).

## 7. Effects and textures

* Jigsaw window (no library technique): `mask add --from-path` with one path made of
  four clockwise rectangles on a 120 px grid plus ten clockwise circles (radius 26) that
  poke out as knobs. All sub-paths run clockwise, so they add up to one shape.
* Sockets: seven table-colored circles drawn over the window's edge, 14 px inside it.
* Cut lines (**T-pattern**): a `kind=grid` render with `cell=120`, dark at alpha 0.3,
  over a white copy shifted 2 px down-right at 45% for the bevel. The grid lines fall on
  60, 180, 300 px and so on across a 1080 px canvas; the window edges use the same numbers.
* Pieces: one `shape path` per piece (a 120 px square, two knobs out, two sockets in),
  `fx drop-shadow --offset 6,10 --blur 14 --opacity 40`, then `layer dup`, recolor with
  `fx overlay --color`, move, rotate.
* Hole: the same piece outline filled with the table color plus `fx inner-shadow`.
* **T-text-behind-subject**: the masthead is under the face, whose top row covers its
  lower edge. **T-fit-width** on the masthead. **T-rules**: four 2 px hairlines.

## 8. Hierarchy and eye path

1. The eyes of the big face (center, largest, the only saturated area).
2. The hole in the cheek and the small figure filling it: the moment of surprise.
3. Up to the masthead.
4. The name at the left, the hook at the right, then the loose pieces at the edges.

## 9. Build recipe

`photo.png` is your image here: a tight, front-on face (or any photo) that will become
the puzzle. `subject.png` is your image here too: a small cutout figure for the corner.
Set the held piece's fill to a color sampled from the photo where the hole is.

```bash
pixelox render js scripts/pattern.js -o cuts.png --size 1080x1440 --param kind=grid --param cell=120 --param weight=0.08 --param color='#1a1410' --param alpha=0.3
pixelox new -o cover.pxo --size 1080x1440 --fill '#c4c3c0'
pixelox layer add cover.pxo --in-place --name Light
pixelox fill gradient cover.pxo --in-place --layer Light --kind radial --from 55%,55% --to 55%,120% --stops '#e4e3e0ff@0,#e4e3e000@1'
pixelox text add cover.pxo --in-place --name Masthead --content "MONTAGE" --font "Avenir Next" --weight 900 --size 18% --tracking -50 --color '#f4f3f0'
pixelox transform scale cover.pxo --in-place --layer Masthead --size 1000x
pixelox transform move cover.pxo --in-place --layer Masthead --to 50%,4.6% --anchor t
pixelox fx stroke cover.pxo --in-place --layer Masthead --width 5 --color '#f4f3f0' --position outside
pixelox fx drop-shadow cover.pxo --in-place --layer Masthead --offset 0,6 --blur 18 --opacity 30
pixelox comp place cover.pxo photo.png --in-place --as Face --size x1300 --at 50%,58% --anchor c
pixelox mask add cover.pxo --in-place --layer Face --from-path 'M 300 240 H 780 V 360 H 300 Z M 180 360 H 900 V 960 H 180 Z M 300 960 H 900 V 1080 H 300 Z M 300 1080 H 660 V 1440 H 300 Z M 454 226 A 26 26 0 1 1 506 226 A 26 26 0 1 1 454 226 Z M 694 226 A 26 26 0 1 1 746 226 A 26 26 0 1 1 694 226 Z M 214 346 A 26 26 0 1 1 266 346 A 26 26 0 1 1 214 346 Z M 140 540 A 26 26 0 1 1 192 540 A 26 26 0 1 1 140 540 Z M 140 780 A 26 26 0 1 1 192 780 A 26 26 0 1 1 140 780 Z M 888 420 A 26 26 0 1 1 940 420 A 26 26 0 1 1 888 420 Z M 888 660 A 26 26 0 1 1 940 660 A 26 26 0 1 1 888 660 Z M 214 974 A 26 26 0 1 1 266 974 A 26 26 0 1 1 214 974 Z M 648 1260 A 26 26 0 1 1 700 1260 A 26 26 0 1 1 648 1260 Z M 754 1094 A 26 26 0 1 1 806 1094 A 26 26 0 1 1 754 1094 Z'
pixelox shape path cover.pxo --in-place --name Sockets --d 'M 574 254 A 26 26 0 1 1 626 254 A 26 26 0 1 1 574 254 Z M 168 420 A 26 26 0 1 1 220 420 A 26 26 0 1 1 168 420 Z M 168 660 A 26 26 0 1 1 220 660 A 26 26 0 1 1 168 660 Z M 860 540 A 26 26 0 1 1 912 540 A 26 26 0 1 1 860 540 Z M 860 900 A 26 26 0 1 1 912 900 A 26 26 0 1 1 860 900 Z M 814 1066 A 26 26 0 1 1 866 1066 A 26 26 0 1 1 814 1066 Z M 288 1140 A 26 26 0 1 1 340 1140 A 26 26 0 1 1 288 1140 Z' --fill '#d2d1ce'
pixelox shape path cover.pxo --in-place --name Hole --d 'M 660 600 H 704 A 22 22 0 1 1 736 600 H 780 V 644 A 22 22 0 1 1 780 676 V 720 H 736 A 22 22 0 1 0 704 720 H 660 V 676 A 22 22 0 1 0 660 644 Z' --fill '#d8d7d4'
pixelox fx inner-shadow cover.pxo --in-place --layer Hole --offset 3,4 --blur 8 --opacity 45
pixelox comp place cover.pxo cuts.png --in-place --as CutsLight --fit cover
pixelox fx overlay cover.pxo --in-place --layer CutsLight --color '#ffffff'
pixelox transform move cover.pxo --in-place --layer CutsLight --by 2,2
pixelox layer opacity cover.pxo --in-place --layer CutsLight --value 45
pixelox comp place cover.pxo cuts.png --in-place --as Cuts --fit cover
pixelox shape path cover.pxo --in-place --name PieceHeld --d 'M 830 520 H 874 A 22 22 0 1 1 906 520 H 950 V 564 A 22 22 0 1 1 950 596 V 640 H 906 A 22 22 0 1 0 874 640 H 830 V 596 A 22 22 0 1 0 830 564 Z' --fill '#c9906c'
pixelox fx drop-shadow cover.pxo --in-place --layer PieceHeld --offset 6,10 --blur 14 --opacity 40
pixelox transform rotate cover.pxo --in-place --layer PieceHeld --degrees 24
pixelox layer dup cover.pxo --in-place --layer PieceHeld --name Loose1
pixelox fx overlay cover.pxo --in-place --layer Loose1 --color '#ecebe8'
pixelox transform move cover.pxo --in-place --layer Loose1 --to 12%,86% --anchor c
pixelox layer dup cover.pxo --in-place --layer Loose1 --name Loose2
pixelox transform move cover.pxo --in-place --layer Loose2 --to 7%,72% --anchor c
pixelox transform rotate cover.pxo --in-place --layer Loose2 --degrees 70
pixelox layer dup cover.pxo --in-place --layer Loose1 --name Loose3
pixelox transform move cover.pxo --in-place --layer Loose3 --to 92%,40% --anchor c
pixelox transform rotate cover.pxo --in-place --layer Loose3 --degrees -40
pixelox layer reorder cover.pxo --in-place --layer Loose3 --index 1
pixelox comp place cover.pxo subject.png --in-place --as Builder --size x430 --at 74%,100% --anchor b
pixelox fx drop-shadow cover.pxo --in-place --layer Builder --offset 8,12 --blur 24 --opacity 40
pixelox text add cover.pxo --in-place --name Credit --content "THE EDITORS'" --font "Avenir Next" --weight 700 --size 1.2% --color '#f4f3f0' --at 5.1%,3% --anchor l
pixelox shape line cover.pxo --in-place --name RuleL1 --from 3.5%,20.9% --to 17%,20.9% --stroke '#f4f3f0' --stroke-width 2
pixelox text add cover.pxo --in-place --name StarName --content $'ARLO\nVANCE' --font "Avenir Next Condensed" --weight 300 --size 5.5% --leading 84% --color '#f4f3f0'
pixelox transform move cover.pxo --in-place --layer StarName --to 3.5%,21.9% --anchor tl
pixelox shape line cover.pxo --in-place --name RuleL2 --from 3.5%,31.9% --to 17%,31.9% --stroke '#f4f3f0' --stroke-width 2
pixelox shape line cover.pxo --in-place --name RuleR1 --from 81%,20.9% --to 97.3%,20.9% --stroke '#f4f3f0' --stroke-width 2
pixelox text add cover.pxo --in-place --name Hook --content $'FINDING\nTHE LAST\nFEW PARTS' --font "Avenir Next Condensed" --weight 300 --size 3.3% --leading 92% --align right --color '#f4f3f0'
pixelox transform move cover.pxo --in-place --layer Hook --to 97.2%,21.9% --anchor tr
pixelox shape line cover.pxo --in-place --name RuleR2 --from 81%,31.9% --to 97.3%,31.9% --stroke '#f4f3f0' --stroke-width 2
pixelox export cover.pxo -o cover.png
```

How the piece path works: a 120 px square drawn clockwise; on the top and right edges
an arc with flags `1 1` bulges out as a knob, on the bottom and left edges an arc with
flags `1 0` cuts in as a socket (radius 22, neck 32 px wide). To make a piece elsewhere,
add the same offset to every x or every y. `layer dup` leaves the copy active directly
above its source; the last `layer reorder --index 1` puts the final loose piece back on
top so the figure and the text that follow land above it.

Change first: the photo and its position inside the window, then which cell is the hole
(shift the Hole path by multiples of 120), then the masthead word.

## 10. Variations and pairings

* Other "unfinished object" versions of the same gag with the same parts: a face of
  mosaic tiles (drop the knobs, use a 60 px grid and remove 8 to 10 cells), a face of
  sticky notes (cells in 4 pastel overlays), a pixel portrait (`filter pixelate` inside
  the window).
* Dark table: `#1e1f22` ground, light cut lines, white masthead, for a night or tech mood.
* Square post: window 6 by 6 cells centered, masthead reduced to a small logo, one hole
  and one held piece, no cover lines.
* Suits team and recruiting posters ("the missing piece"), product launches (the
  product photo as the puzzle), strategy or education event flyers.
* Pairs with T-paper-grain (amount 10), T-vignette at low strength toward the table's
  corners, and T-label-box for a date tag in place of the right cover line.

## 11. Pitfalls

* Mixed winding in the mask path. A counter-clockwise circle on a clockwise rectangle
  punches a hole where a knob should be; write every sub-path clockwise (rectangles as
  `H` right, `V` down, `H` left; circles as two arcs with sweep flag 1).
* Cut lines that miss the window edges. Pick the cell size first and put every window
  edge, hole, and socket on a grid line; check one corner at full zoom.
* Flat pieces. Without the white bevel copy, the hole's inner shadow, and the drop
  shadows, the picture reads as a photo with a table grid over it, not as cardboard.
* Too many holes in the face. Keep eyes, nose, and mouth whole; remove pieces at the
  cheek, jaw, and hairline only.
* Loud cover lines. Bold or colored type beside the forehead fights the gag; keep the
  lines thin, white, small, and fenced by hairlines.
