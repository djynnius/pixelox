---
id: i045
title: Five folded ribbons with icon discs and a host figure
kind: [list]
topic: [business, generic]
mood: [vibrant, friendly, playful]
style: [flat, illustrated, 3d]
format: portrait
structure: S-stat-poster
data: [text-only]
charts: [C-icon-badge]
palette: ['#f2f2f2', '#f2cd4b', '#ce344c', '#902671', '#68b1c0', '#3d6dad', '#2c3d44']
needs: subject-cutout
---

# i045: Five folded ribbons with icon discs and a host figure

## 1. Concept

A portrait list graphic on a pale gray ground: five wide ribbons in five saturated colors
are stacked down the left, each starting in a round cap that holds a white disc with an icon
and carrying a short title and a note in white. At a fold line past the middle every ribbon
bends away, runs down to the right in a darker shade, and ends as a narrow strip behind a
full-length cartoon figure who stands at the right edge and smiles at the reader. The
message is "five things worth knowing, presented by a person". It works because the fold
turns a plain list into one object with depth, the receding strips point at the figure, and
each color appears three times (disc icon, ribbon, strip), so the five points stay apart
without numbers.

## 2. Data story

No numbers. A list of 5 parallel points, text only:

* 5 titles of 2 words (up to 13 characters), caps.
* 5 notes of about 17 words on 4 lines of up to 32 characters.
* 5 pictograms, one per point.
* 1 two-word header, one word per line, in a corner.

The points have no order and no amounts: the ribbons are equal in size.

Placeholder content in the recipe (invented, basics for a small shop): SET A PRICE, TRACK
SALES, SAVE TIME, SELL ONLINE, KNOW BUYERS, each with a four-line note.

## 3. Information architecture

1. **Header**, top right: what the list is about, small and dark, in the only empty corner.
2. **Five ribbons**, left: read top to bottom. Inside each, left to right: icon disc, then a
   title with a hairline under it, then the note.
3. **The folds**: darker panels that carry the eye from the ends of the ribbons down to the
   right.
4. **The figure**, right: stands in front of the five narrow strips, looking out of the
   page. A host, not data.

## 4. Format and grid

* Portrait, 1 : 1.24, close to 4:5. Recipe canvas 1080 x 1340. (The copy studied was
  stretched vertically by about a quarter, so its discs were ovals; the grid below keeps the
  portrait proportions and draws true circles.)
* Left margin 2.1% W. Fold line at 56.6% W. Second fold at 77.3% W; the strips then run to
  the right edge.
* Ribbon fronts: `2.1%,16.2%,54.5% x 13.7%` for the first; the next four start at 31.7%,
  47.2%, 62.8%, and 78.3% H. Pitch 15.5% H, gap 1.8% H. The left end of each front is a
  half circle (radius 8.5% W).
* Back strips: `77.3%,55.4%,22.7% x 7.3%` for the first; the next four start at 63.7%, 72%,
  80.4%, and 88.7% H. Pitch 8.3% H, gap 1% H.
* Fold panels: four-sided shapes that join each front's right edge (13.7% H tall) to its
  strip's left edge (7.3% H tall). The first falls steeply (39% H over 21% W), the last
  almost not at all (10% H).
* Discs: centered 10.6% W from the left edge on each ribbon's middle line; ring diameter
  17% W (the ribbon's height), white disc 14% W.
* Text column on the ribbons: 22.7% to 55% W.
* Figure: `76%,25%,22% x 74%`, feet 1% H above the bottom edge.
* Header: right-aligned at 95.9% W, lines at 3.4% and 7.3% H.

## 5. Layer stack

1. Ground: pale gray, `#f7f7f7` in the upper middle falling to `#e6e6e8` in the corners.
2. Five back strips, each in the darkest shade of its hue.
3. Five fold panels in the middle shade.
4. Five ribbon fronts in the full hue: yellow `#f2cd4b`, crimson `#ce344c`, purple
   `#902671`, teal `#68b1c0`, blue `#3d6dad`.
5. Five rings (the middle shade of each hue) and five white discs on them.
6. Five icons in the ribbon's hue, on the discs.
7. Titles, hairlines, and notes in white on the fronts.
8. The figure, in front of the strips.
9. Header, dark slate `#2c3d44`.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#f2f2f2` | ground (`#f7f7f7` to `#e6e6e8`) | 38% | nothing |
| `#f2cd4b` | ribbon 1; fold `#e7a63b`; strip `#cd8d33` | 12% | point 1 |
| `#ce344c` | ribbon 2; fold `#bd2d36`; strip `#b2252b` | 11% | point 2 |
| `#902671` | ribbon 3; fold `#73225c`; strip `#6b1e52` | 11% | point 3 |
| `#68b1c0` | ribbon 4; fold `#59a3be`; strip `#589bb5` | 10% | point 4 |
| `#3d6dad` | ribbon 5; fold `#2f5582`; strip `#2c4f79` | 10% | point 5 |
| `#ffffff` | discs, all type on the ribbons | 4% | nothing |
| `#2c3d44` | header | 1% | nothing |

Categorical: five hues from warm at the top to cool at the bottom, each in three shades
(front, fold about 12% darker, strip about 20% darker). The shades model the fold; they
carry no meaning. The figure's clothes are muted (gray-blue, tan) so the ribbons stay the
loudest thing on the page.

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Header line 1 | 1 word, 10 to 11 letters | caps | neo-grotesque (Inter, Roboto / Helvetica Neue) | 500 | 27 | 0 | - | right | `#2c3d44` | 95.9%, 3.4% |
| Header line 2 | 1 word, 6 to 8 letters | caps | neo-grotesque | 700 | 40 | 0 | - | right | `#2c3d44` | 95.9%, 7.3% |
| Ribbon title (x5) | 2 words, up to 13 characters | caps | geometric sans, light and narrow in the original (Jost 300, Montserrat 300 / Avenir Next) | 400 (the original: 300) | 32 | 0 | - | left | `#ffffff` | 22.7% W, 42 px under the ribbon's top |
| Ribbon note (x5) | about 17 words on 4 lines of up to 32 characters | sentence | neo-grotesque | 400 | 18 | 0 | 128% | left | `#ffffff` | 22.7% W, first line 74 px under the ribbon's top |

Two families, both sans: a light geometric for the ribbon titles, a grotesque for the notes
and the header. Sizes 40 : 32 : 18. The header is deliberately small: the ribbons are the
headline.

## 8. Charts and diagrams

No data chart. The ribbons are furniture built from native shapes:

* **Fronts**: `shape rect`, 496 x 184 px, starting at the disc's center line (x = 115) and
  ending at the fold (x = 611).
* **Caps**: **C-icon-badge** x5: a ring (`shape ellipse`, 184 px, the fold shade), a white
  disc (152 px), and an 84 px icon in the ribbon's hue.
* **Fold panels**: `shape path`, four points: the front's two right corners and the strip's
  two left corners.
* **Strips**: `shape rect`, 245 x 98 px, from x = 835 to the right edge.
* A white hairline (2 px, 301 px long) under each title.

**3D spec (for a later three.js build, or for drawing other counts by hand).** Object: five
flat bands of equal width, each folded twice into a Z: a front part facing the viewer, a
middle part receding to the right and back, a rear part facing the viewer again, farther
away. View: frontal, eye level above the stack, with strong perspective: the rear parts are
0.53 times as tall as the fronts (7.3% against 13.7% H) and their pitch is 0.54 times the
front pitch (8.3% against 15.5% H); the rear stack sits lower (centered on 76% H, the fronts
on 54% H). Light from the front: faces that turn away are darker (fold panel about 12%
darker than the front, rear strip about 20% darker). No cast shadows, no outlines, no gloss.
Depth, the fold angle, and the size of the rear strips encode nothing.

For N ribbons: front height h = 0.88 x pitch; the rear stack keeps the same ratio at 0.54
times the scale and ends 3% H above the bottom edge.

## 9. Icons and illustration

* Disc icons, 84 px, 7 px line, each in its ribbon's hue: `dollar-sign`,
  `chart-column-increasing`, `clock`, `earth`, `user`. The original uses solid pictograms (a
  dollar sign, rising bars with an arrow, a clock face, a globe, a head and shoulders).
* Figure: `subject.png`, your image here (a cutout on transparency). The original shows a
  full-length cartoon man in flat vector style with soft shading: standing straight, facing
  the viewer, smiling, both hands in his trouser pockets, a gray-blue sweater over a dark
  shirt and red tie, tan trousers, brown shoes. He is 22% W wide and 74% H tall, overlaps the
  five strips, and his head rises above the top strip into the empty ground; no floor
  shadow. Any cutout works (a drawn character, a photographed person, a product on a stand)
  if its colors are quieter than the ribbons.
* Sizing: the recipe places a half-length cutout 420 px wide (39% W), cut off by the bottom
  edge. For a **full-length** figure set the height instead: `--size x990 --at 896,1330
  --anchor b` (74% H tall, feet 1% H above the bottom edge), so the head lands at 25% H.

## 10. Effects and finish

* Ground: a radial gradient on its own layer (`fill gradient --kind radial`), lightest at
  50% W, 35% H.
* Ribbons: flat fills in three shades per hue; the shading is done by color, not by an
  effect.
* No shadows in the recipe. The original has a faint soft shadow on each ring's inner edge;
  add `fx drop-shadow --offset 0,3 --blur 8 --opacity 20` to the disc layers for that.

## 11. Hierarchy and eye path

1. The five colored fronts: 55% of the width, the strongest colors on the page.
2. The white discs and their icons, down the left edge.
3. The titles (white, 32 px, underlined), then the notes.
4. Along the folds to the figure's face.
5. The header in the top right corner.

Titles 32, notes 18 (1.8 : 1). The icon discs are 5 times the title height: the list can be
scanned by icon alone.

## 12. Build recipe

Supply `subject.png` (your image here: a figure cut out on transparency; see section 9 for
the size of a full-length one).
Canvas 1080 x 1340. Placeholder copy, no data.

```bash
pixelox new -o ribbons.pxo --size 1080x1340 --fill '#e8e8ea'
pixelox layer add ribbons.pxo --in-place --name Glow
pixelox fill gradient ribbons.pxo --in-place --layer Glow --kind radial --from 50%,35% --to 100%,100% --stops '#f7f7f7@0,#f2f2f2@0.6,#e6e6e8@1' --dither
pixelox shape rect ribbons.pxo --in-place --name Back1 --rect 835,742,245x98 --fill '#cd8d33'
pixelox shape path ribbons.pxo --in-place --name Fold1 --d 'M 611 217 L 835 742 L 835 840 L 611 401 Z' --fill '#e7a63b'
pixelox shape rect ribbons.pxo --in-place --name Front1 --rect 115,217,496x184 --fill '#f2cd4b'
pixelox shape ellipse ribbons.pxo --in-place --name Ring1 --rect 23,217,184x184 --fill '#e7a63b'
pixelox shape ellipse ribbons.pxo --in-place --name Disc1 --rect 39,233,152x152 --fill '#ffffff'
pixelox shape icon ribbons.pxo dollar-sign --in-place --name Icon1 --size 84 --at 115,309 --anchor c --color '#f2cd4b' --stroke-width 7
pixelox text add ribbons.pxo --in-place --name Title1 --content "SET A PRICE" --font @geometric-sans --weight 400 --size 32 --color '#ffffff' --at 245,259 --anchor l
pixelox shape line ribbons.pxo --in-place --name Rule1 --from 229,280 --to 530,280 --stroke '#ffffff' --stroke-width 2
pixelox text add ribbons.pxo --in-place --name Body1 --content $'Count every cost first, then add\nthe margin you need to grow.\nRound it to a number that is\neasy to say.' --font @neo-grotesque --size 18 --leading 128% --color '#ffffff' --at 245,291 --anchor tl
pixelox shape rect ribbons.pxo --in-place --name Back2 --rect 835,854,245x98 --fill '#b2252b'
pixelox shape path ribbons.pxo --in-place --name Fold2 --d 'M 611 425 L 835 854 L 835 952 L 611 609 Z' --fill '#bd2d36'
pixelox shape rect ribbons.pxo --in-place --name Front2 --rect 115,425,496x184 --fill '#ce344c'
pixelox shape ellipse ribbons.pxo --in-place --name Ring2 --rect 23,425,184x184 --fill '#bd2d36'
pixelox shape ellipse ribbons.pxo --in-place --name Disc2 --rect 39,441,152x152 --fill '#ffffff'
pixelox shape icon ribbons.pxo chart-column-increasing --in-place --name Icon2 --size 84 --at 115,517 --anchor c --color '#ce344c' --stroke-width 7
pixelox text add ribbons.pxo --in-place --name Title2 --content "TRACK SALES" --font @geometric-sans --weight 400 --size 32 --color '#ffffff' --at 245,467 --anchor l
pixelox shape line ribbons.pxo --in-place --name Rule2 --from 229,488 --to 530,488 --stroke '#ffffff' --stroke-width 2
pixelox text add ribbons.pxo --in-place --name Body2 --content $'Write down every order for a\nmonth. Compare the weeks, not\nthe days, and note what sold\nbest.' --font @neo-grotesque --size 18 --leading 128% --color '#ffffff' --at 245,499 --anchor tl
pixelox shape rect ribbons.pxo --in-place --name Back3 --rect 835,965,245x98 --fill '#6b1e52'
pixelox shape path ribbons.pxo --in-place --name Fold3 --d 'M 611 633 L 835 965 L 835 1063 L 611 817 Z' --fill '#73225c'
pixelox shape rect ribbons.pxo --in-place --name Front3 --rect 115,633,496x184 --fill '#902671'
pixelox shape ellipse ribbons.pxo --in-place --name Ring3 --rect 23,633,184x184 --fill '#73225c'
pixelox shape ellipse ribbons.pxo --in-place --name Disc3 --rect 39,649,152x152 --fill '#ffffff'
pixelox shape icon ribbons.pxo clock --in-place --name Icon3 --size 84 --at 115,725 --anchor c --color '#902671' --stroke-width 7
pixelox text add ribbons.pxo --in-place --name Title3 --content "SAVE TIME" --font @geometric-sans --weight 400 --size 32 --color '#ffffff' --at 245,675 --anchor l
pixelox shape line ribbons.pxo --in-place --name Rule3 --from 229,696 --to 530,696 --stroke '#ffffff' --stroke-width 2
pixelox text add ribbons.pxo --in-place --name Body3 --content $'List the jobs you repeat each\nweek. Pick the slowest one and\nfind a quicker way to do it\nthis month.' --font @neo-grotesque --size 18 --leading 128% --color '#ffffff' --at 245,707 --anchor tl
pixelox shape rect ribbons.pxo --in-place --name Back4 --rect 835,1076,245x98 --fill '#589bb5'
pixelox shape path ribbons.pxo --in-place --name Fold4 --d 'M 611 841 L 835 1076 L 835 1174 L 611 1025 Z' --fill '#59a3be'
pixelox shape rect ribbons.pxo --in-place --name Front4 --rect 115,841,496x184 --fill '#68b1c0'
pixelox shape ellipse ribbons.pxo --in-place --name Ring4 --rect 23,841,184x184 --fill '#59a3be'
pixelox shape ellipse ribbons.pxo --in-place --name Disc4 --rect 39,857,152x152 --fill '#ffffff'
pixelox shape icon ribbons.pxo earth --in-place --name Icon4 --size 84 --at 115,933 --anchor c --color '#68b1c0' --stroke-width 7
pixelox text add ribbons.pxo --in-place --name Title4 --content "SELL ONLINE" --font @geometric-sans --weight 400 --size 32 --color '#ffffff' --at 245,883 --anchor l
pixelox shape line ribbons.pxo --in-place --name Rule4 --from 229,904 --to 530,904 --stroke '#ffffff' --stroke-width 2
pixelox text add ribbons.pxo --in-place --name Body4 --content $'Put your five best products on\none simple page. Add clear\nphotos, the price, and how to\norder.' --font @neo-grotesque --size 18 --leading 128% --color '#ffffff' --at 245,915 --anchor tl
pixelox shape rect ribbons.pxo --in-place --name Back5 --rect 835,1188,245x98 --fill '#2c4f79'
pixelox shape path ribbons.pxo --in-place --name Fold5 --d 'M 611 1049 L 835 1188 L 835 1286 L 611 1233 Z' --fill '#2f5582'
pixelox shape rect ribbons.pxo --in-place --name Front5 --rect 115,1049,496x184 --fill '#3d6dad'
pixelox shape ellipse ribbons.pxo --in-place --name Ring5 --rect 23,1049,184x184 --fill '#2f5582'
pixelox shape ellipse ribbons.pxo --in-place --name Disc5 --rect 39,1065,152x152 --fill '#ffffff'
pixelox shape icon ribbons.pxo user --in-place --name Icon5 --size 84 --at 115,1141 --anchor c --color '#3d6dad' --stroke-width 7
pixelox text add ribbons.pxo --in-place --name Title5 --content "KNOW BUYERS" --font @geometric-sans --weight 400 --size 32 --color '#ffffff' --at 245,1091 --anchor l
pixelox shape line ribbons.pxo --in-place --name Rule5 --from 229,1112 --to 530,1112 --stroke '#ffffff' --stroke-width 2
pixelox text add ribbons.pxo --in-place --name Body5 --content $'Ask ten customers why they\nchose you. Use their own\nwords in your next advert\nand on your sign.' --font @neo-grotesque --size 18 --leading 128% --color '#ffffff' --at 245,1123 --anchor tl
pixelox comp place ribbons.pxo subject.png --in-place --as Figure --size 420x --at 880,1330 --anchor b
pixelox text add ribbons.pxo --in-place --name Head1 --content "SMALL SHOP" --font @neo-grotesque --weight 500 --size 27 --color '#2c3d44' --at 1036,46 --anchor r
pixelox text add ribbons.pxo --in-place --name Head2 --content "BASICS" --font @neo-grotesque --weight 700 --size 40 --color '#2c3d44' --at 1036,98 --anchor r
pixelox export ribbons.pxo -o ribbons.png
```

Reuse:

* Ribbon n (counting from 0) has its front top at y = 217 + 208 x n and its strip top at
  y = 742 + 111.5 x n. The fold path is `M 611 y L 835 Y L 835 Y+98 L 611 y+184 Z`. Disc
  center (115, y + 92); title at y + 42; hairline at y + 63; note from y + 74.
* Four ribbons: pitch 260 and height 230 for the fronts, pitch 139 and height 122 for the
  strips, same start points. Six: front pitch 173, height 152; strip pitch 93, height 81.
* Keep each ribbon's three shades in order (front lightest, strip darkest); make a shade by
  mixing the hue with 12% and 20% black.
* Titles longer than 13 characters run past the hairline: shorten them or lengthen every
  hairline together.

## 13. Variations and pairings

* **Numbered steps:** put 01 to 05 in the discs (`text add`, 64 px, bold, in the ribbon's
  hue) instead of icons; the list becomes a process and the top-to-bottom order means
  something.
* **With numbers:** replace each title with a stat ("42%", 56 px bold) and keep the note as
  its caption: a five-fact sheet. Then cite the source under the last ribbon with
  C-source-line.
* **Mirrored:** figure on the left, ribbons folding to the left; flip every x around the
  center line.
* **One hue:** five steps of one color from light to dark for a calm, corporate look; keep
  the icons in the darkest step.
* **Without a figure:** let the strips end in a C-rocket or C-bulb object, or drop the folds
  and run the ribbons to 92% W as a plain list.
* Relatives: i016 (levels as a pyramid) when the points rest on each other; i038 when the
  points are steps on one object.

## 14. Pitfalls

* **White type on the yellow ribbon.** White on `#f2cd4b` has almost no contrast (the
  original has this flaw). Darken the first hue to about `#e0a100`, or set that ribbon's type
  in `#3a2e00`. Check every ribbon: text and ground need a lightness difference of 50 points.
* **Order and size that look like data.** Five equal ribbons say "five equal points". If your
  points are ranked, number the discs; if they have amounts, do not vary the ribbon lengths
  by eye: use C-bars and print the values. The fold's perspective must never stand for a
  value. This is the data-integrity rule for decorated lists.
* **Oval discs.** If the artwork is stretched to fit another format, the discs turn into
  ovals and the icons distort. Change the canvas and rebuild; never scale one axis.
* **A figure louder than the list.** A character in saturated clothes, or larger than 25% of
  the width, takes the first look away from the ribbons. Mute it, or shrink it.
* **Notes that reach the fold.** Text must end 2% W before the fold line; a line that crosses
  onto the darker panel looks like a mistake. Break lines by hand at 32 characters.
* **Different shades by guesswork.** If the fold panel is lighter than the front on one
  ribbon and darker on another, the object stops reading as folded paper.
