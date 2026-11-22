---
id: p106
title: Giant ring letters as photo window
genre: [conference, tech-ad, corporate]
mood: [bold, energetic, serious]
style: [swiss, geometric, photo-composite]
format: portrait-a
archetype: A-split-field
palette: ['#f31d01', '#10160f', '#ffffff', '#0b0b0b']
techniques: [T-image-in-type, T-vertical-type, T-rules, T-geo-shapes, T-shape-crop]
needs: photo
---

# p106: Giant ring letters as photo window

## 1. Concept

A tech-conference poster in three vertical fields (white, signal red, black). The
event's four-letter lowercase wordmark is blown up until each letter is 45% W wide, set
in two rows of two, and used as a window: a dark portrait photo shows inside the thick
ring strokes, while the counters (the holes of the letters) are punched back out in red.
The word is too big and too cropped to read, but the small wordmark at the top left
tells the viewer what the shapes are. It works because one bold system (a circle, a hole,
a bar) produces both the logo and the whole picture. Presented as a print leaning on a
wall; the wall is not part of the design.

## 2. Format and grid

* Portrait A-series (1 : 1.414), full bleed.
* Three vertical fields: white 0 to 25% W; red 25 to 90% W; black strip 90 to 100% W.
  All three run the full height.
* Letter module: every letter is a disc of radius 22.7% W (16% H) with a concentric
  hole of radius 10.5% W, so the ring stroke is 12.2% W thick. Stems are bars 13% W
  wide (25 to 38% W): the bar stands exactly on the white-to-red border.
* Letter centers: row 1 at 12.5% H, x 47.5% and 90% W (the first is cut by the top
  edge, the second by the top edge and the black strip); row 2 at 42% H, x 13.5% W
  (hanging out into the white field, cut by the left edge) and 60% W. Rows overlap by
  2% H, neighbors by 2% W, so the four rings fuse into one dark mass, 0 to 58% H.
* Small-text axes: 5.5% W (wordmark); 26.5% W (dates, on the bar); 38% W (label,
  place, sponsor block); 94.5% W (center of the black strip). Text zone 2: 69 to 74% H.
  Text zone 3: 85 to 97% H, rotated. The red field from 59 to 68% H is empty.

## 3. Layer stack

1. Ground: white `#ffffff`.
2. Red field: `25%,0,65% x 100%`, flat `#f31d01`.
3. Letter mass (the photo window), one shape: four discs of radius 22.7% W at the
   centers above, plus the bar `25%,0,13% x 58%`.
4. Photo, clipped to the letter mass: a man in a white short-sleeved shirt with folded arms in front
   of a dark hedge. The backdrop is almost black (`#10160f`), so the letters read as dark
   shapes; the figure stands at 47 to 70% W with his head at 18 to 28% H, in the joint
   between the rows, and his torso inside the second letter of row 2.
5. Counters, red, on top of the photo: three full discs of radius 10.5% W (both row-1
   letters and the first row-2 letter); for the last letter the hole is a disc crossed
   by a 5.8% H photo bar (a crossbar): a red cap above (34 to 39.5% H), a red segment
   below (45.3 to 49.8% H), and a red slit `60%,45.3%,24% x 3.2%` that opens to the
   right. In the original the first row-1 letter also has a slit; the recipe closes it.
6. White wedge: triangle `0,0` to `33.5%,0` to `25%,16%`: it slants the top of the bar
   and keeps the white field clean above the hanging letter.
7. Black strip: `90%,0,10% x 100%`, `#0b0b0b`, over the letters.
8. Dotted hairlines (white, 3 px dots every 9 px, vertical): 12.3% W from 25.5 to
   32.5% H; 36.5% W from 52 to 67% H and from 81 to 87.5% H; 94.5% W from 11 to 18% H.
9. Wordmark: black, lowercase, at `5.5%,3.6%`, 16.5% W wide.
10. Dates: white, light. The first month name is right-aligned to 23.6% W, inside the
    red counter of the hanging letter; "DD, dash" and the second line "Month DD" start
    at 26.5% W. Lines at 39.8% and 43.8% H.
11. Label (tracked caps) at `38%,69.8%` and place name at `38%,72.6%`, white.
12. Sponsor block: 7 rotated lines (reading upward) at 37.8 to 55.5% W, bottom 97% H.
13. Strip text, rotated, reading upward: hashtag in gray at 21 to 31% H; three speaker
    names in red (bottoms at 85, 73.5, 61% H) with a role line in white beside each;
    a lead-in line at 88 to 97% H.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#f31d01` | red field, counters, speaker names | 40% |
| `#10160f` | photo backdrop inside the letters | 30% |
| `#ffffff` | left field, wedge, most small type | 20% |
| `#0b0b0b` | right strip, wordmark | 10% |

Red, white, and black: the classic Swiss triad at full strength. The photo adds no
color of its own (a dark green-black backdrop, a white shirt, skin), so the red stays
the only hue.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Wordmark | `[BRAND]`, 4 letters | lower | heavy geometric sans (Poppins Black, Outfit Black / Avenir Next Heavy) | 900 | 2.8% (x-height) | -60 | - | left | `#0b0b0b` | 5.5%, top 3.6% |
| Month | 3-letter month | title | squarish light sans (Titillium Web Light, Exo 2 Light / Avenir Next) | 400 | 3% | 0 | - | right | `#ffffff` | ends 23.6%, 39.8% |
| Dates | "DD" + dash / "Mon DD" | title | same | 400 | 3% | 0 | 96% | left | `#ffffff` | 26.5%, 39.8% and 43.8% |
| Label | event name + year, 3 words | caps | same | 500 | 0.7% | 250 | - | left | `#ffffff` | 38%, 69.8% |
| Place | city, state | title | same | 400 | 1.9% | 0 | - | left | `#ffffff` | 38%, 72.6% |
| Sponsors | lead-in line + 6 names, one per line | sentence | same | 500 | 0.55% | 0 | 250% | rotated -90 | `#ffffff` | 37.8%, bottom 97% |
| Hashtag | 1 tag, 8 letters | caps | same | 500 | 0.7% | 400 | - | rotated -90 | `#8a8a8a` | 94.5%, 21 to 31% |
| Speaker names | 3 names | title | same | 700 | 0.7% | 0 | - | rotated -90 | `#f31d01` | 93.3%, bottoms 85, 73.5, 61% |
| Speaker roles | lead-in + 3 roles | sentence | same | 500 | 0.5% | 0 | - | rotated -90 | `#ffffff` | 95.6%, bottom 97% |

One light technical sans for everything except the black wordmark. The dates (cap
height 3% H) are the only mid-size type; everything else is 0.5 to 0.7% H, a quarter of
it. The loud part of the poster is not type at all.

## 6. Imagery

One portrait photo, used whole (no cutout): a figure from the hips up, arms folded,
looking at the camera, against a dark and even backdrop. Shadows are crushed
(`adjust levels --in-black 40 --gamma 0.6` on a flat photo; less on one that is already
dark) so that the letter strokes stay near-black and only the figure is light. Place the
head in a joint where two rings overlap, never under a counter. Good substitutes:
a product on black, hands at work, a city at night. Avoid bright backdrops: the letter
shapes then lose their edge against the white field.

## 7. Effects and textures

* **T-image-in-type** with drawn letters: the letter mass is one `shape path` (four
  two-arc circles and a rectangle); `layer clip` puts the photo into it.
* Counters are a second `shape path` in the field color, above the photo, so the holes
  can cover parts of the figure: red in front, photo behind, red field behind that.
* **T-geo-shapes** arc rules for the half discs: a cap is `M left y A r r 0 0 1 right y
  Z`, a lower segment `M right y A r r 0 0 1 left y Z`.
* **T-rules** dotted lines (`--dash 0,9 --cap round`), all in one path, vertical.
* **T-vertical-type** for the sponsor block and the strip text: add, rotate -90, then
  move by the visible bottom-left corner.
* Flat color, no grain, no shadows.
* **T-shape-crop** in [techniques.md](../techniques.md) is the tested library version of the photo shown through a drawn shape (arch, panel, tiles, or letter); start from it when reusing this idea.

## 8. Hierarchy and eye path

1. The red counters on the dark mass (strongest contrast, largest shapes): the eye
   bounces between the four holes.
2. The face, in the middle of them.
3. The white dates across the left counter and the bar.
4. The small black wordmark, which explains the shapes.
5. Label and place name in the empty red; rotated fine print last.

Scale: letter height 32% H; dates 3% H; place 1.9% H; fine print 0.5 to 0.7% H. About
60 : 6 : 4 : 1.

## 9. Build recipe

`photo.png` is your image here (opaque, at least 1080 px wide, subject right of center).
All path numbers are for 1080 x 1528: ring radius 245, hole radius 113, bar 270 to 410.

```bash
pixelox new -o conf.pxo --size 1080x1528 --fill '#ffffff'
pixelox shape rect conf.pxo --in-place --name Field --rect 25%,0,65%x100% --fill '#f31d01'
pixelox shape path conf.pxo --in-place --name Letters --d 'M 268 191 A 245 245 0 1 1 758 191 A 245 245 0 1 1 268 191 Z M 727 191 A 245 245 0 1 1 1217 191 A 245 245 0 1 1 727 191 Z M -99 642 A 245 245 0 1 1 391 642 A 245 245 0 1 1 -99 642 Z M 270 0 H 410 V 886 H 270 Z M 403 648 A 245 245 0 1 1 893 648 A 245 245 0 1 1 403 648 Z' --fill '#10160f'
pixelox comp place conf.pxo photo.png --in-place --as Photo --size 1080x --at 0,0
pixelox adjust levels conf.pxo --in-place --layer Photo --in-black 40 --gamma 0.6
pixelox layer clip conf.pxo --in-place --layer Photo
pixelox shape path conf.pxo --in-place --name Counters --d 'M 400 191 A 113 113 0 1 1 626 191 A 113 113 0 1 1 400 191 Z M 859 191 A 113 113 0 1 1 1085 191 A 113 113 0 1 1 859 191 Z M 33 642 A 113 113 0 1 1 259 642 A 113 113 0 1 1 33 642 Z M 544 604 A 113 113 0 0 1 752 604 Z M 752 692 A 113 113 0 0 1 544 692 Z M 648 692 H 905 V 741 H 648 Z' --fill '#f31d01'
pixelox shape path conf.pxo --in-place --name Wedge --d 'M 0 0 H 362 L 270 245 H 0 Z' --fill '#ffffff'
pixelox shape rect conf.pxo --in-place --name Strip --rect 90%,0,10%x100% --fill '#0b0b0b'
pixelox shape path conf.pxo --in-place --name Dots --d 'M 133 390 V 497 M 394 795 V 1024 M 394 1238 V 1337 M 1021 168 V 275' --fill none --stroke '#ffffff' --stroke-width 3 --dash 0,9 --cap round --opacity 80
pixelox text add conf.pxo --in-place --name Logo --content "bode" --font @geometric-sans --weight 900 --fit-width 178 --tracking -60 --color '#0b0b0b' --at 5.5%,3.6% --anchor-to ink
pixelox text add conf.pxo --in-place --name Month --content "Oct" --font @geometric-sans --weight 400 --size 4.2% --color '#ffffff' --at 23.6%,39.8% --anchor r
pixelox text add conf.pxo --in-place --name Dates --content $'30 —\nNov 01' --font @geometric-sans --weight 400 --size 4.2% --leading 96% --color '#ffffff' --at 26.5%,41.8% --anchor l
pixelox text add conf.pxo --in-place --name Label --content "BODE CONFERENCE 2031" --font @geometric-sans --weight 500 --size 1% --tracking 250 --color '#ffffff' --at 38%,69.8% --anchor l
pixelox text add conf.pxo --in-place --name Place --content "Harbor City, ST" --font @geometric-sans --weight 400 --size 2.6% --color '#ffffff' --at 38%,72.6% --anchor l
pixelox text add conf.pxo --in-place --name Sponsors --content $'Brought to you by:\nFirst Partner Group\nSecond Partner Co.\nThird Media\nFourth Labs\nFifth Studio\nSixth Fund' --font @geometric-sans --weight 500 --size 0.8% --leading 250% --color '#ffffff'
pixelox transform rotate conf.pxo --in-place --layer Sponsors --degrees -90
pixelox transform move conf.pxo --in-place --layer Sponsors --to 37.8%,97% --anchor bl
pixelox text add conf.pxo --in-place --name Tag --content "#BODECON" --font @geometric-sans --weight 500 --size 1% --tracking 400 --color '#8a8a8a'
pixelox transform rotate conf.pxo --in-place --layer Tag --degrees -90
pixelox transform move conf.pxo --in-place --layer Tag --to 94.5%,31% --anchor b
pixelox text add conf.pxo --in-place --name Speakers --content "First Speaker                      Second Speaker                      Third Speaker" --font @geometric-sans --weight 700 --size 0.95% --color '#f31d01'
pixelox transform rotate conf.pxo --in-place --layer Speakers --degrees -90
pixelox transform move conf.pxo --in-place --layer Speakers --to 92.8%,85% --anchor bl
pixelox text add conf.pxo --in-place --name Roles --content "2031 speakers include:               Founder, Studio Name                      Head of Design, Company                      Partner, Fund Name" --font @geometric-sans --weight 500 --size 0.72% --color '#ffffff'
pixelox transform rotate conf.pxo --in-place --layer Roles --degrees -90
pixelox transform move conf.pxo --in-place --layer Roles --to 95.2%,97% --anchor bl
pixelox export conf.pxo -o conf.png
```

The recipe spells "bode": row 1 is a "b" (ring 1 plus the shared bar on its left) and
an "o"; row 2 is a "d" (ring 3 plus the same bar on its right) and an "e". Change
first: the word. Letters this module can draw: o (ring), c (ring plus a slit to the
right edge), e (ring, bar across the hole, slit), b, d, p, q (ring plus a bar on one
side, up or down). Then the photo position (`--at`), so the face clears the counters.
A circle is two arcs: `M cx-r cy A r r 0 1 1 cx+r cy A r r 0 1 1 cx-r cy Z`. Speaker
names and roles are single lines padded with spaces; pad until each role sits beside its
name.

## 10. Variations and pairings

* Field color swap: cobalt `#1437e6`, acid yellow `#ffe600` (then make the small type
  black), or emerald `#00a86b`. Keep white and black as the two side fields.
* T-duotone on the photo (black to the field color) for a two-ink version; the counters
  then become white.
* No photo: fill the letter mass with flat `#0b0b0b` or with a T-pattern dot field.
* Landscape 16:9: one row of four rings across the width, white field on top (0 to
  20% H), black strip at the bottom.
* Suits conferences, product launches, studio open days, annual reports with a short
  brand name (3 to 5 lowercase letters). Pairs with A-swiss-asymmetric text columns in
  the empty red area and with T-label-box for a "sold out" tag on the strip.

## 11. Pitfalls

* Counters drawn under the photo: the holes vanish and the letters turn into blobs. The
  counter path goes above the clipped photo.
* Rings that do not touch: the mass breaks into four badges. Overlap rows by 2% H and
  neighbors by 2% W.
* A photo with a bright or busy backdrop: strokes stop reading as letters. Crush the
  shadows, or use a subject cutout on a `#10160f` rectangle.
* The face under a red hole or cut by the bar's edge. Move the photo, not the letters:
  the letters are the grid.
* Setting the dates bold or large. They stay light and at 3% H; the contrast with the
  heavy rings is the point.
* Too much fine print on the red field. The original keeps 59 to 68% H empty and turns
  the long lists sideways so they read as texture, not as paragraphs.
