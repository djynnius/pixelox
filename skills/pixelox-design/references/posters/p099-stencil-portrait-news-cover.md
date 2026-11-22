---
id: p099
title: Stencil portrait news cover, red frame
genre: [magazine-cover, editorial]
mood: [bold, serious, retro]
style: [stencil, illustration, editorial]
format: portrait-3x4
archetype: A-cover-masthead
palette: ['#192131', '#cf3630', '#b12a30', '#78939b', '#f0e6c6', '#ffffff']
techniques: [T-stencil-poster, T-frame-border, T-label-box, T-pattern, T-fit-width]
needs: subject-cutout
---

# p099: Stencil portrait news cover, red frame

## 1. Concept

A news-magazine cover for an annual honor issue: a close-up portrait drawn as a four-ink
stencil (navy, red, dusty blue, cream) fills a red frame, the ground behind the head is
split into a red half and a blue half, a white serif masthead sits in front of the hair,
and a small dark pill across the masthead names the honor. It works because the whole
cover uses the frame's red as one of its four inks, so the illustration, the border, and
the brand read as a single printed object.

## 2. Format and grid

* Portrait 3:4. Red frame 5.6% W wide (60 px at 1080 wide) on all four sides, with a 4 px
  white keyline on its inner edge. Picture window `5.6%,4.3%,88.8% x 91.2%`.
* One vertical axis at 50% W for the masthead and the pill. The portrait is centered a
  little right of it (52% W) because the head is turned.
* The ground splits at 55.5% W: red on the left, dusty blue on the right. The split runs
  behind the head, so the lit side of the face has the blue ground and the shadow side
  the red one.
* Vertical zones inside the window: masthead 6.5 to 21% H; face 20 to 66% H; collar, tie,
  and suit 66 to 95.5% H. Nothing but the portrait below 21% H: no cover lines at all.
* Fine print lives in the frame: two lines in the top border, one in the bottom border.

## 3. Layer stack

1. Ground left: red `#b52a31`, x 5.6 to 55.5% W. Ground right: dusty blue `#78939b`.
2. Tonal collage inside both halves: fragments of ornament, map lines, a crowd, and
   buildings printed tone on tone (darker red `#912a2d` on the red, lighter blue `#8ca4a9`
   on the blue), 15 to 25% contrast, never sharper than the portrait.
3. Portrait, four hard-edged tones. Navy `#192131`: hair, brows, nostrils, lip shadow,
   suit. Red `#b12a30`: the shadow side of the face, the tie. Dusty blue `#748c93`:
   mid-tones on the lit side. Cream `#f0e6c6`: forehead, nose bridge, cheek, chin, teeth,
   shirt. Hair top at 11% H, hairline 20% H, eyes 31 to 33% H, mouth 52% H, chin 66% H;
   ears at 15% and 88% W (the head is 73% W wide). Shoulders run off both sides and the
   bottom of the window.
4. Hatch: fine horizontal cream lines (pitch 0.6% H) inside the dusty-blue areas of the
   face, the mark of a hand-cut line screen.
5. Masthead: 4 white serif capitals, x 16.5 to 83% W, cap top 6.5% H, cap height 14.5% H,
   opaque, in front of the hair, with a hard navy edge 0.3% W to the lower right.
6. Pill: `32.7%,12.8%,35.3% x 3.1%`, navy `#192131`, 2 px white outline, fully rounded.
   It crosses the two middle masthead letters just above their middle.
7. Pill text: 4 short words in white heavy caps, cap height 1.5% H, centered in the pill.
8. Red frame `#cf3630` and white keyline, above everything.
9. Fine print in the frame: an issue note at 5.6% W and a date range ending at 94.4% W,
   both at 3.6% H; a web address ending at 94.4% W, 96.6% H.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#192131` | navy: hair, suit, deepest tone, pill | 25% |
| `#cf3630` | frame red | 20% |
| `#b12a30` | red ink: left ground, shadow tone, tie | 20% |
| `#78939b` | dusty blue: right ground, mid-tones | 20% |
| `#f0e6c6` | cream: lit planes, shirt, hatch | 10% |
| `#ffffff` | masthead, keyline, pill text | 5% |

Red, white, and blue pulled toward brick, cream, and slate so it looks screen-printed. The
frame red is slightly brighter than the ink red, which keeps the border in front of the
picture. Pure white is reserved for the brand: masthead, keyline, pill text.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Masthead | `[MASTHEAD]`, 4 letters | caps | sharp transitional serif, display cut (Playfair Display, Libre Caslon Display / Times New Roman) | 700 | 14.5% | 20 | - | center | `#ffffff` | 50%, cap top 6.5% |
| Pill text | the honor: 4 words, 14 to 16 letters | caps | heavy wide grotesque (Archivo Black, Montserrat 900 / Avenir Next) | 900 | 1.5% | 10 | - | center | `#ffffff` | 50%, 14.4% |
| Issue note | "SPECIAL ISSUE"-style, 2 words | caps | neo-grotesque (Inter, Archivo / Helvetica Neue) | 700 | 0.6% | 140 | - | left | `#1a0505` | 5.6%, 3.6% |
| Date range | two dates joined by a slash | caps | same | 700 | 0.6% | 140 | - | right | `#1a0505` | 94.4%, 3.6% |
| Web address | one address | lower | same | 700 | 0.55% | 140 | - | right | `#1a0505` | 94.4%, 96.6% |

Five text elements and only two of them meant to be read. There is no headline, no name,
no cover line: the face is the headline, which only works for a face the audience already
knows or a cover whose title explains it.

## 6. Imagery

A head-and-shoulders portrait of a man in a dark suit, white shirt, and red tie, seen
slightly from below in three-quarter view, chin raised, looking up and past the viewer to
the right. Light comes from the upper right. The photograph is thrown away and only its
light-and-shadow shapes remain as four flat inks (**T-stencil-poster**): black and white,
a median filter to round the shapes, then a gradient map with hard bands at 0.3, 0.5, and
0.7. In the original the shapes were cut by hand, the red and blue swap roles across the
face (red in the shadows on the left, blue in the mid-tones on the right), and the
mid-tones carry a horizontal line screen.

For a build: a cutout portrait with hard side light, shot from slightly below, head filling
half the canvas height or more. The upward gaze is what makes it heroic rather than a mug
shot.

## 7. Effects and textures

* **T-stencil-poster** on the cutout: bands navy 0 to 0.3, red 0.3 to 0.5, dusty blue 0.5
  to 0.7, cream 0.7 to 1; `filter median` radius 5.
* Hatch: a horizontal stripe pattern (**T-pattern**, `kind=stripes`, `angle=90`, cell 9,
  cream) masked to the dusty-blue band with `select luma` (range 0.45 to 0.62 on the
  subject layer) and `mask add`, at 55% opacity.
* Tonal collage: a contrasty `fill noise` cloud layer in dark plum (`--contrast 45`,
  `multiply`, 30%), plus one faded ring pattern at 22% on the blue half (stand-ins for the
  printed fragments).
* Masthead edge: `fx drop-shadow` with blur 0, offset 3,3, navy at 70%.
* **T-label-box** (pill form): rounded rectangle with a white outline and centered text.
* **T-frame-border**: even-odd frame path plus an inner keyline, drawn last.
* **T-fit-width**: the masthead is set to 718 px (66.5% W) with `--fit-width`.

## 8. Hierarchy and eye path

1. The cream planes of the face, the eyes first (the only light area, in the center).
2. The masthead (pure white, large, top).
3. The pill, because it interrupts the masthead.
4. The split ground and collage, as atmosphere; the fine print last.

Cap heights: masthead 14.5 : pill 1.5 : fine print 0.6, about 24 : 2.5 : 1. The face is
55% H tall, almost four times the masthead.

## 9. Build recipe

`subject.png` is your image here: a cutout portrait with strong side light.

```bash
pixelox render js scripts/pattern.js -o rings.png --size 1080x1440 --param kind=rings --param color='#ffffff' --param cell=34 --param weight=0.3 --param cx=0.86 --param cy=0.74 --param fade=out --param alpha=0.5
pixelox render js scripts/pattern.js -o hatch.png --size 1080x1440 --param kind=stripes --param angle=90 --param cell=9 --param weight=0.45 --param color='#f0e6c6'
pixelox new -o cover.pxo --size 1080x1440 --fill '#b52a31'
pixelox shape rect cover.pxo --in-place --name BlueHalf --rect 55.5%,0,44.5%x100% --fill '#78939b'
pixelox fill noise cover.pxo --in-place --as-layer --name Mottle --seed 12 --scale 32% --octaves 3 --warp 12 --contrast 45 --bias=-16 --stops 'transparent,#3a1a22' --blend multiply --opacity 30
pixelox comp place cover.pxo rings.png --in-place --as Rings --fit cover
pixelox layer opacity cover.pxo --in-place --layer Rings --value 22
pixelox comp place cover.pxo subject.png --in-place --as Subject --size x1800 --at 51%,1.5% --anchor t
pixelox adjust bw cover.pxo --in-place --layer Subject
pixelox adjust levels cover.pxo --in-place --layer Subject --in-black 25 --in-white 215
pixelox filter median cover.pxo --in-place --layer Subject --radius 5
pixelox adjust gradient-map cover.pxo --in-place --layer Subject --stops '#192131@0,#192131@0.3,#b12a30@0.3,#b12a30@0.5,#748c93@0.5,#748c93@0.7,#f0e6c6@0.7,#f0e6c6@1'
pixelox select luma cover.pxo --in-place --layer Subject --range 0.45..0.62 --softness 0.02
pixelox comp place cover.pxo hatch.png --in-place --as Hatch --fit cover
pixelox mask add cover.pxo --in-place --layer Hatch --from @sel
pixelox select none cover.pxo --in-place
pixelox layer opacity cover.pxo --in-place --layer Hatch --value 55
pixelox text add cover.pxo --in-place --name Masthead --content "AXIS" --font "Times New Roman" --weight 700 --fit-width 718 --tracking 20 --color '#ffffff' --at 50%,6.5% --anchor t --anchor-to ink
pixelox fx drop-shadow cover.pxo --in-place --layer Masthead --offset 3,3 --blur 0 --color '#192131' --blend normal --opacity 70
pixelox shape rect cover.pxo --in-place --name Pill --rect 353,184,381x46 --radius 23 --fill '#192131' --stroke '#ffffff' --stroke-width 2
pixelox text add cover.pxo --in-place --name PillText --content "VOICE OF THE YEAR" --font @geometric-sans --weight 900 --size 2.1% --tracking 10 --color '#ffffff' --at 50%,207 --anchor c
pixelox shape path cover.pxo --in-place --name Frame --d 'M 0 0 H 1080 V 1440 H 0 Z M 60 62 H 1020 V 1376 H 60 Z' --fill-rule evenodd --fill '#cf3630'
pixelox shape rect cover.pxo --in-place --name Keyline --rect 62,64,956x1310 --fill none --stroke '#ffffff' --stroke-width 4
pixelox text add cover.pxo --in-place --name IssueNote --content "SPECIAL ISSUE" --font @neo-grotesque --weight 700 --size 0.85% --tracking 140 --color '#1a0505' --at 5.6%,3.6% --anchor l
pixelox text add cover.pxo --in-place --name IssueDate --content "DECEMBER 27, 2031 / JANUARY 3, 2032" --font @neo-grotesque --weight 700 --size 0.85% --tracking 140 --color '#1a0505' --at 94.4%,3.6% --anchor r
pixelox text add cover.pxo --in-place --name Site --content "www.example.com" --font @neo-grotesque --weight 700 --size 0.8% --tracking 140 --color '#1a0505' --at 94.4%,96.6% --anchor r
pixelox export cover.pxo -o cover.png
```

Change first: the three band edges in `--stops` (0.3, 0.5, 0.7) until navy holds the hair
and suit and cream sits only on the lit planes; then the `select luma` range if you moved
the 0.5 or 0.7 edge (it must cover the dusty-blue band); then the masthead word and the
pill text (widen the pill to the text width plus one pill height).

## 10. Variations and pairings

* Same frame, photographic covers: [p089](p089-red-frame-annual-list-cover.md) (ghost
  masthead behind the subject, story block in a corner) and
  [p123](p123-full-figure-list-cover.md) (full figure on a pale ground, condensed list
  title at the left). The same portrait language without a frame is
  [p025](p025-stencil-portrait-print.md), which ends in a caption band instead of a
  masthead.
* Other ink sets, one dark, two mids, one light, with the frame taking the warm mid:
  black `#111111` / orange `#e85d04` / sand `#c9b98a` / white; forest `#0b3d2e` / gold
  `#d9a441` / sage `#8fa58c` / cream.
* Move the split: 50% W for a symmetric, formal cover; a diagonal split (a `shape path`
  triangle) for more energy.
* Awards night poster, tribute, alumni or staff "of the year" card, podcast cover: keep
  the portrait, the pill, and one wordmark where the masthead is.
* Pairs with T-grunge (cream specks at 20 to 30%) and T-paper-grain for a printed
  surface, and with T-sunburst behind the head in place of the split ground.

## 11. Pitfalls

* Front-lit photo: the face becomes one flat tone. Use hard side light or raise the
  contrast with `adjust levels` before the gradient map.
* The collage competes with the face. Keep it tone on tone (multiply at 30% or less) and
  softer than the portrait's edges; if in doubt, delete it.
* Hatch over everything. It belongs only in the mid-tone band; lines across the navy or
  the cream turn the stencil into a blind.
* The pill hides the masthead. Its height is about a fifth of the masthead's cap height
  and it crosses the middle letters only; first and last letters stay untouched.
* Adding cover lines. The design has none; one more text block and it becomes an
  ordinary cover. If a name is needed, put it in the pill.
* Using this treatment on a real person to suggest an endorsement, an award, or a
  political message they did not give. The style carries that meaning; use it with
  consent.
