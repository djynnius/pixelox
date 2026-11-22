---
id: p094
title: Dark halftone poster with vertical type
genre: [exhibition, event-flyer, concert]
mood: [bold, dark, dramatic, energetic]
style: [halftone, collage, type-led]
format: portrait-2x3
archetype: A-vertical-type
palette: ['#000000', '#ffa436', '#ff0a00', '#ffffff', '#f3edea']
techniques: [T-vertical-type, T-halftone-portrait, T-gradient-mesh]
needs: subject-cutout
---

# p094: Dark halftone poster with vertical type

## 1. Concept

An art-event poster on black: one halftone cutout (a shouting mouth) sits in a glowing
orange oval in the middle, and every word on the sheet except one small paragraph is turned
90 degrees and stood along the left and right edges. It works because the layout has only
two voices: a soft, round, glowing picture and hard, upright columns of type. The two big
words become walls that frame the image.

## 2. Format and grid

* Portrait 2:3. Margins: 5.8% W left, 3% W right, 11.5% H top (to the first letter), 7.5% H
  bottom.
* The grid is five vertical tracks, read from the left:
  track 1, x 5.8 to 21.5% W: big word one;
  track 2, x 24.5 to 29% W: the date;
  track 3, x 34 to 38.5% W: the time;
  center, x 30.5 to 79% W: the picture (it overlaps tracks 2 and 3 above 68% H);
  track 4, x 81.5 to 97% W: a small sideways note on top, big word two below it.
* All rotated text is turned -90 degrees (it reads from the bottom up), and every small
  element starts from one common bottom line at 91.5 to 92.5% H: date, time, big word two,
  and the bottom of the only horizontal paragraph.
* Big word one is the exception: it starts at 73% H and runs up to 11.5% H, so it sits 19%
  H higher than everything else and leaves the lower left corner to a red glow.
* Tracks 1 and 4 are the same width (15.7% W, the cap height of the big words), which is
  what makes the two words read as a pair across the sheet.

## 3. Layer stack

1. Ground: black `#000000`, full canvas.
2. Corner glows, two soft discs that fade to nothing: orange `#ff8a00` centered at 88% W,
   7% H, and red `#ff0a00` centered at 14% W, 84% H; each has a radius of about 36% W, so
   they stop just short of each other on the diagonal.
3. Halo: an upright oval, 66% W wide and 58% H tall, centered at 55% W, 49% H. Flat warm
   orange `#ffa436` from the center out to two thirds of the radius, then a pale gray-beige
   rim `#b9a492` at 70% opacity, gone by the edge. It looks like a lamp behind the cutout.
4. Halftone cutout: box `30.5%,29%,48.5% x 38.5%`, hard outline, round dots in near-black
   `#1c1413` on off-white `#f3edea`, dot pitch 0.95% W, screen on a straight grid (angle 0).
5. Spot tint: one part of the cutout (the tongue) is tinted red `#e0283c`, about
   `41%,50%,30% x 9%`; the dots stay visible through it.
6. Big word one: white, rotated -90, x 5.8 to 21.5% W, from y 73% up to 11.5% H.
7. Big word two: white, rotated -90, x 81.5 to 97% W, from y 91.5% up to 64% H.
8. Date: cream `#f6ecdf`, rotated -90, x 24.5 to 29% W, from y 92% up to 49% H.
9. Time: cream, rotated -90, x 34 to 38.5% W, from y 92.5% up to 72.5% H.
10. Paragraph: white, horizontal, five short lines, `41%,85.3%,38% x 7.2%`.
11. Side note: light beige `#e8ddd0`, five lines rotated -90 as one block, x 81.5 to 97% W,
    from y 60% up to 23.5% H, directly above big word two.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#000000` | ground | 45% |
| `#ffa436` | halo, orange corner glow (`#ff8a00` at its core) | 20% |
| `#f3edea` | halftone paper; with `#1c1413` dots it reads as gray | 15% |
| `#ff0a00` | red corner glow, red spot tint (`#e0283c`) | 10% |
| `#ffffff` | big words, paragraph; date and note in cream `#f6ecdf` | 10% |

Black plus one warm family (red, orange, pale beige): an analogous scheme with no cool
color at all. White is kept for type only; the paper of the halftone is slightly off-white
so the picture never outshines the words.

## 5. Text elements

Cap heights of rotated lines are measured across the line (their width on the canvas).

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Big word one | 1 word, 6 letters | caps | heavy geometric sans (Montserrat ExtraBold / Avenir Next) | 900 | 10.5% H (15.7% W) | 0 | - | rotated -90 | `#ffffff` | x 5.8 to 21.5%, y 73% up to 11.5% |
| Big word two | 1 word, 3 letters | caps | same | 900 | 10.5% H | 0 | - | rotated -90 | `#ffffff` | x 81.5 to 97%, y 91.5% up to 64% |
| Date | DD.MM.YYYY | digits | geometric sans (Montserrat / Avenir Next) | 600 | 3% H (4.5% W) | 600 | - | rotated -90 | `#f6ecdf` | x 24.5 to 29%, y 92% up to 49% |
| Time | HH:MM | digits | same | 600 | 3% H | 600 | - | rotated -90 | `#f6ecdf` | x 34 to 38.5%, y 92.5% up to 72.5% |
| Paragraph | 5 lines, 2 to 6 words each | caps | squarish techno sans (Exo 2, Saira / Avenir Next Condensed) | 600 | 1% H | 80 | 106% | left | `#ffffff` | 41%, top 85.3% |
| Side note | 5 lines, 4 to 6 words each | caps | same | 600 | 1.5% H | 60 | 108% | rotated -90, left | `#e8ddd0` | x 81.5 to 97%, y 60% up to 23.5% |

Two families: one geometric sans in two weights (900 for the words, 600 for the numbers)
and one narrow technical sans for the small print. The date is tracked so wide (600) that
ten characters stretch over 43% of the height; that turns a small line into a third column.

## 6. Imagery

One photograph of a wide-open mouth (upper teeth, tongue, lower lip), cropped very tight
and cut out along a rounded outline a little wider than the lips, so it reads as a sticker
rather than a face. It is printed as a coarse black dot screen on off-white
(**T-halftone-portrait**: `adjust bw`, `adjust levels --in-black 25 --in-white 210`,
`filter halftone --cell 10 --angle 0`), and one feature is tinted red by a flat shape
clipped to the cutout in multiply mode. Any single, loud detail works: an eye, a hand, an
ear with an earring, a statue's head, a megaphone, a flower. It must have deep blacks and
clear whites, and it should fill the box (49% W wide); a subject smaller than 40 dots
across dissolves.

## 7. Effects and textures

* **T-gradient-mesh**, reduced to two placed blobs (`blobs.js` with `at='0.88,0.07;0.14,0.84'`,
  `size=0.55`, `vary=0`, transparent background). They sit in opposite corners, on the
  diagonal that the picture does not use.
* Halo: a radial `fill gradient` on its own layer, squeezed into an oval with a non-uniform
  `transform scale --size 720x950`. The stops keep the inside flat and put the falloff in
  the last third (`@0.66` orange, `@0.82` pale beige at alpha `b3`, `@1` clear).
* **T-halftone-portrait** on the cutout, cell 10 px on a 1080 px canvas, straight grid.
* Spot tint: `shape ellipse` above the cutout, `layer clip`, `layer blend --mode multiply`.
* **T-vertical-type** for five of the six text elements.
* No grain, no shadows, no outlines: glow and dot screen are the only textures.

## 8. Hierarchy and eye path

1. The cutout in its halo: the brightest, most detailed thing, dead center.
2. Big word one on the left, read upward; the eye then jumps to big word two at the lower
   right, because it has the same size, weight, and color.
3. Date and time beside word one (same direction, so no second tilt of the head).
4. The horizontal paragraph, then the side note.

Cap-height ratio: big words 10.5 : date 3 : side note 1.5 : paragraph 1, about 10 : 3 : 1.5 : 1.
The 3.5x step between the words and the date is what keeps six text elements from feeling
crowded.

## 9. Build recipe

`subject.png` is your image here: a cutout with transparency, tight on one feature. The
stand-in is placed 660 px tall; size yours so it fills about 49% W by 38% H.

```bash
pixelox render js scripts/blobs.js -o glow.png --size 1080x1620 --param bg=transparent --param colors='#ff8a00,#ff0a00' --param at='0.88,0.07;0.14,0.84' --param size=0.55 --param vary=0 --param alpha=1
pixelox new -o art.pxo --size 1080x1620 --fill '#000000'
pixelox comp place art.pxo glow.png --in-place --as Glow --fit cover
pixelox layer add art.pxo --in-place --name Halo
pixelox fill gradient art.pxo --in-place --layer Halo --kind radial --from 50%,50% --to 50%,79.5% --stops '#ffa436ff@0,#ffa436ff@0.66,#b9a492b3@0.82,#b9a49200@1'
pixelox transform scale art.pxo --in-place --layer Halo --size 720x950 --anchor c
pixelox transform move art.pxo --in-place --layer Halo --by 50,-20
pixelox comp place art.pxo subject.png --in-place --as Subject --size x660 --at 54.5%,48.5% --anchor c
pixelox adjust bw art.pxo --in-place --layer Subject
pixelox adjust levels art.pxo --in-place --layer Subject --in-black 25 --in-white 210
pixelox filter halftone art.pxo --in-place --layer Subject --cell 10 --angle 0 --ink '#1c1413' --paper '#f3edea'
pixelox shape ellipse art.pxo --in-place --name Tint --rect 40%,49%,30%x10% --fill '#e0283c'
pixelox layer clip art.pxo --in-place --layer Tint
pixelox layer blend art.pxo --in-place --layer Tint --mode multiply
pixelox text add art.pxo --in-place --name Word1 --content "SPOKEN" --font @geometric-sans --weight 900 --size 13.4% --tracking 30 --color '#ffffff' --at 50%,50% --anchor c
pixelox transform rotate art.pxo --in-place --layer Word1 --degrees -90
pixelox transform move art.pxo --in-place --layer Word1 --to 5.8%,73% --anchor bl
pixelox text add art.pxo --in-place --name Word2 --content "ACT" --font @geometric-sans --weight 900 --size 13.4% --tracking 30 --color '#ffffff' --at 50%,50% --anchor c
pixelox transform rotate art.pxo --in-place --layer Word2 --degrees -90
pixelox transform move art.pxo --in-place --layer Word2 --to 81.5%,91.5% --anchor bl
pixelox text add art.pxo --in-place --name Date --content "24.04.2031" --font @geometric-sans --weight 600 --size 4% --tracking 600 --color '#f6ecdf' --at 50%,50% --anchor c
pixelox transform rotate art.pxo --in-place --layer Date --degrees -90
pixelox transform move art.pxo --in-place --layer Date --to 24.5%,92% --anchor bl
pixelox text add art.pxo --in-place --name Time --content "20:30" --font @geometric-sans --weight 600 --size 4% --tracking 600 --color '#f6ecdf' --at 50%,50% --anchor c
pixelox transform rotate art.pxo --in-place --layer Time --degrees -90
pixelox transform move art.pxo --in-place --layer Time --to 34%,92.5% --anchor bl
pixelox text add art.pxo --in-place --name Body --content $'DOORS OPEN AT SEVEN FOR A NIGHT\nOF NEW WORK BY SIX ARTISTS,\nWITH LIVE SOUND AND FILM\nIN THE MAIN HALL. ENTRY IS FREE\nBEFORE EIGHT.' --font @condensed --weight 600 --size 1.35% --leading 106% --tracking 80 --color '#ffffff' --at 41%,85.3% --anchor tl
pixelox text add art.pxo --in-place --name Side --content $'A SECOND SHORT NOTE GOES HERE,\nFIVE LINES OF SMALL CAPITALS,\nSET ON ITS SIDE TO ECHO\nTHE TWO BIG WORDS AND FILL\nTHE RIGHT EDGE.' --font @condensed --weight 600 --size 2% --leading 108% --tracking 60 --color '#e8ddd0' --at 50%,50% --anchor c
pixelox transform rotate art.pxo --in-place --layer Side --degrees -90
pixelox transform move art.pxo --in-place --layer Side --to 81.5%,60% --anchor bl
pixelox export art.pxo -o art.png
```

Change first: the two words. Both keep the same `--size`; word one should come out 57 to
62% H long (6 wide letters) and word two under 28% H (3 letters), or it runs into the side
note above it. Then the cutout, then the two glow colors. Every rotated layer is placed by
its bottom-left corner (`--anchor bl`): x is the left edge of its track, y is where the
line starts.

## 10. Variations and pairings

* Light twin: p124 is the same grid on pale gray with black words, a flat red disc instead
  of the red glow, and a grainy spray gradient instead of the orange glow.
* Cold version: glows `#00b7ff` and `#3a2bff`, halo `#7fe0ff` with a rim of `#9aa8b9`, tint
  `#2b6bff`. Acid version: glows `#c6ff00` and `#00d26a`, halo `#e6ff6a`.
* Swap the dot screen for `--shape line` (engraved lines) or for T-duotone in the two glow
  colors when the subject is a full portrait.
* Suits gallery openings, club and festival nights, lecture series, record sleeves. As a
  series, keep the type tracks fixed and change only the cutout and one glow.
* Pairs with T-paper-grain (`--amount 14`) over the glows to hide banding, and with
  T-glitch on the cutout for electronic music.

## 11. Pitfalls

* Rotating the two big words in opposite directions (one up, one down). The reader then
  tilts twice. Turn every vertical line -90 degrees.
* Building the halo from one blurred ellipse (`shape ellipse` with `filter gaussian`): it
  fades evenly from the middle and has no pale rim. Use a radial gradient with a flat
  inside on a full layer and squeeze it with `transform scale`, as in the recipe.
* A halo that is brightest in the middle only. Behind the cutout nothing shows; the color
  must stay flat out to two thirds of the radius so an even ring appears around the subject.
* Glows in adjacent corners, or a third glow. Two glows on one diagonal leave black in the
  other two corners, and that black is where the white words get their contrast.
* A halftone cell above 1.5% W on a small subject: the feature stops being readable. Keep at
  least 40 dots across the subject, and raise contrast before the screen.
* Small type on the halo's rim: cream on pale beige disappears. Keep the date left of the
  halo's bright part or move it down so it sits on black and red.
