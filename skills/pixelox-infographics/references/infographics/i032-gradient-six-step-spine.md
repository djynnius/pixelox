---
id: i032
title: Six numbered discs on a spine over a pink-to-blue mesh
kind: [process, explainer]
topic: [marketing, business, generic]
mood: [vibrant, friendly, futuristic, clean]
style: [gradient, illustrated, 3d]
format: long
structure: S-spine-steps
data: [process, text-only]
charts: [C-process-spine, C-title-block, C-icon-badge]
palette: ['#c98cff', '#e277ff', '#8cbdfe', '#f5f5f5', '#5a17b5', '#0d0d12']
needs: illustration
---

# i032: Six numbered discs on a spine over a pink-to-blue mesh

## 1. Concept

A tall how-to sheet on a soft gradient mesh that runs from orchid pink on the left to sky blue
on the right. A thin white line drops down the middle through six large off-white discs, each
holding one bold black digit. Beside every disc there is a short violet heading with two or
three lines of dark text on one side and a glossy 3D picture on the other; the sides swap at
every step, and faint dashed white lines mark the seams. The message is "six steps, in
order". It works because the page has one loud element repeated six times (the numbered
disc): the count is obvious from across the room, the gradient supplies the mood, and the
text can stay small and calm.

## 2. Data story

No numbers except the step numbers. Text only, as an ordered list:

* 6 steps. Each: a heading that starts with the step's number, 6 to 10 words on 2 to 3
  lines, and a note of 8 to 25 words on 1 to 4 lines.
* 1 title in two parts: a lead-in of 3 words in regular weight, then 4 to 5 words in bold on
  2 lines.
* 1 tiny category label of one word in spaced caps, top right.
* 7 pictures: one group of objects beside the title, one object or small scene per step.

Placeholder copy in the recipe (invented, starting a shared tool library): six headings
of 7 to 9 words and six notes of 10 to 15 words.

## 3. Information architecture

1. **Label**, top right: the kind of document.
2. **Title**, top left, with the **title picture** to its right.
3. **Step 1**: disc on the spine; heading and note to the right; picture to the left.
4. **Step 2**: mirrored (text left, picture right). Steps 3 to 6 keep alternating.
5. **Dashed seams**: after each step a dashed line runs from the outer margin to the spine,
   on the left after odd steps and on the right after even steps. Each one underlines the
   picture above it and overlines the text below it.

The digits carry the order; the spine joins them into one path. All text is flush left on
both sides (it does not hug the spine), so every heading starts at one of two fixed x
positions and the page scans like two columns.

## 4. Format and grid

* Long portrait, 1 : 1.98. Recipe canvas 1080 x 2140. Margins 10% W left, 5% W right.
* Spine at 50% W. Six discs of 18.5% W (200 px) at 23.3%, 35.9%, 48.4%, 61%, 73.6%, 86.1% H:
  one step every 12.6% H (269 px). The spine runs from the first disc to the last.
* Header zone 0 to 17% H: label at 95% W (right edge), 3.7% H; lead-in cap top 5.2% H; bold
  title lines from 7.5% to 14% H, left edge 10% W; title picture `52%,3.4%,37% x 12%`.
* Text columns: left 10.2% to 38% W; right 63.6% to 92% W; 28% W (300 px) wide. A step's
  heading starts 52 px above its disc's center, the note 18 px below it.
* Pictures: 14 to 23% W wide, centered on 24.4% W (left) or 75.6% W (right), level with the
  disc.
* Dashed seams midway between discs (29.6%, 42.1%, 54.7%, 67.3%, 79.9% H), from 9% to 49% W
  or from 51% to 91% W.
* Footer zone 90 to 100% H: empty gradient (room for a source line).

## 5. Layer stack

1. Ground: a gradient mesh. Base orchid `#c98cff`; soft blobs of magenta pink `#e277ff` (top
   left), sky blue `#8cbdfe` (top right), light orchid `#ea9aff` (middle left), blue
   `#83c2fe` (middle right), periwinkle `#7f97ff` (bottom left), lavender `#aa91ff` (bottom
   right). No hard edge anywhere.
2. Spine: a white line about 4 px wide.
3. Six discs in off-white `#f5f5f5` with dark digits 1 to 6 (bold, 76 px).
4. Five dashed seams: white, 2 px, dashes 10 px on and 7 off, 75% opacity.
5. Label in near-black; lead-in in deep violet `#5a17b5`; bold title in near-black `#0d0d12`.
6. Title picture (the original: a 3D group of a clipboard, a megaphone, a gold ring with an
   "i", a pie chart, and a coin on a small white bench).
7. Six step pictures (the original: glossy 3D renders; the recipe: 150 px icons with a
   violet line and a white fill).
8. Six headings in bold deep violet and six notes in dark ink `#1f1f38`.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#c98cff` | ground base (mesh mid tone) | 30% | nothing |
| `#e277ff` | mesh, left side (with `#ea9aff`) | 25% | nothing |
| `#8cbdfe` | mesh, right side (with `#83c2fe`, `#7f97ff`, `#aa91ff`) | 30% | nothing |
| `#f5f5f5` | discs, spine, seams, picture fills | 8% | "a step is here" |
| `#5a17b5` | lead-in, headings, picture lines | 3% | emphasis: what to do |
| `#0d0d12` | title, digits, notes (`#1f1f38`) | 4% | nothing |

No categorical color: all six steps share one treatment. The mesh stays light (65 to 80%
lightness) so dark type works everywhere; the single accent is the deep violet of the
headings, which sits inside the mesh's own hue range.

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Label | 1 to 2 words | caps | neo-grotesque (Roboto, Inter / Helvetica Neue) | 500 | 18 | 200 | - | right | `#1f1f38` | right edge 95% W, 3.7% H |
| Title lead-in | 3 words | title | neo-grotesque | 400 | 44 | 0 | - | left | `#5a17b5` | 10.2% W, cap top 5.2% H |
| Title | 4 to 5 words on 2 lines | title | neo-grotesque | 700 | 62 | 0 | 84 px | left | `#0d0d12` | 10% W, lines from 7.5% H |
| Disc digit (x6) | 1 to 6 | - | neo-grotesque | 700 | 76 (drawn by the script) | 0 | - | center | `#1f2933` (the original: black) | disc centers |
| Step heading (x6) | number, full stop, 6 to 10 words on 2 lines | sentence | neo-grotesque | 700 | 25 (the original: 23, on 3 lines) | 0 | 32 px | left | `#5a17b5` | 63.6% W or 10.2% W; top 52 px above the disc center |
| Step note (x6) | 8 to 25 words on 2 to 3 lines | sentence | neo-grotesque | 400 | 19 (the original: 18) | 0 | 25 px | left | `#1f1f38` | same left edge; top 18 px below the disc center; box 304 px |

One family in three weights. Sizes 76 : 62 : 44 : 25 : 19, about 4 : 3.3 : 2.3 : 1.3 : 1.
The heading repeats the step number ("1. ...") so a step still makes sense when it is quoted
without its disc.

## 8. Charts and diagrams

* **C-process-spine**: `scripts/steps.js` with `dir=v side=alt node=200`,
  `badges='1,2,3,4,5,6'`, six blank labels, `box=10,17,80,75.4`, `colors='#f5f5f5'
  onecolor=1`, `line='#ffffff'`, `numweight=700`. The script draws the spine, the discs, and
  the digits and logs the disc centers: x = 540; y = 498, 767, 1036, 1305, 1574, 1843.
* The headings and notes are native `text add` layers, not the script's own labels, because
  the original sets all text flush left at two fixed columns; the script's alternating
  layout would right-align the left-hand blocks toward the spine. (With real `labels=` and
  `notes=` the script version is a quick alternative: see Variations.)
* Dashed seams: `shape line --dash 10,7`, halfway between two disc centers.
* No axis, no values, no legend.
* **3D note** (for a later three.js build): the original's pictures are small 3D scenes seen
  from the front and slightly above (elevation about 15 degrees, rotation 20 to 30 degrees
  to the left), lit softly from the upper left, with glossy plastic materials in red
  `#e8483f`, amber `#f2b632`, blue `#4a8fe0`, lilac `#b99cf5`, and white, each casting a faint
  contact shadow. Depth encodes nothing. The recipe replaces them with flat two-tone icons.

## 9. Icons and illustration

* Title picture: `illustration.png` (your image here), 330 px wide, centered at 73% W, 9.6%
  H: a group of 3 to 5 objects on the subject, on a transparent ground.
* Step pictures, 150 px, 8 px line in `#5a17b5`, white fill, centered at (263, y) or (817,
  y) on the side opposite the text: `messages-square`, `key`, `hammer`, `clipboard-list`,
  `clock`, `chart-column`. The original showed 3D renders: a row of ring binders with a
  clipboard; two speech bubbles; a browser window with a magnifier; a presentation board on
  an easel; a magnet pulling coins; a funnel with cards and tokens.
* To use real renders, place six transparent PNGs at 180 to 250 px wide on the same centers
  and drop the icons.

## 10. Effects and finish

* The mesh: `scripts/blobs.js` with six fixed centers (`at=` the four corners and the two
  side midpoints), `size=0.8`, `vary=0`, `alpha=1`, on the base color. Change the six
  `colors=` to change the mood.
* No shadows on the discs in the recipe (the original has a barely visible one). No
  outlines, no texture.

## 11. Hierarchy and eye path

1. The bold title, top left (the darkest, largest type).
2. Disc 1 and its digit; then down the spine, 2 to 6.
3. At each disc: the violet heading on one side, the picture on the other.
4. The notes.

Type steps 76 : 62 : 25 : 19. The discs take about 11% of the page area but nearly all of
its contrast.

## 12. Build recipe

Supply `illustration.png` (your image here: the title picture, with transparency). Canvas
1080 x 2140. All copy is invented.

```bash
pixelox new -o guide.pxo --size 1080x2140 --fill '#c98cff'
pixelox render js scripts/blobs.js -o mesh.png --size 1080x2140 --param bg='#c98cff' --param colors='#e277ff,#8cbdfe,#ea9aff,#83c2fe,#7f97ff,#aa91ff' --param at='0,0;1,0;0,0.5;1,0.45;0,1;1,1' --param size=0.8 --param vary=0 --param alpha=1
pixelox comp place guide.pxo mesh.png --in-place --as Mesh --at 0,0
pixelox render js scripts/steps.js -o spine.png --size 1080x2140 --param box=10,17,80,75.4 --param dir=v --param side=alt --param node=200 --param badges='1,2,3,4,5,6' --param labels=' | | | | | ' --param colors='#f5f5f5' --param onecolor=1 --param line='#ffffff' --param numweight=700
pixelox comp place guide.pxo spine.png --in-place --as Spine --at 0,0
pixelox text add guide.pxo --in-place --name Tag --content "HOW-TO GUIDE" --font @neo-grotesque --weight 500 --size 18 --tracking 200 --color '#1f1f38' --at 1026,79 --anchor r --anchor-to ink
pixelox text add guide.pxo --in-place --name Lead --content "How to Start" --font @neo-grotesque --size 44 --color '#5a17b5' --at 110,112 --anchor tl --anchor-to ink
pixelox text add guide.pxo --in-place --name Title --content $'A Neighborhood\nTool Library' --font @neo-grotesque --weight 700 --size 62 --leading 84 --color '#0d0d12' --at 108,160 --anchor tl
pixelox comp place guide.pxo illustration.png --in-place --as TitleArt --size 330x --at 790,206 --anchor c
pixelox shape line guide.pxo --in-place --name Dash1 --from 97,633 --to 530,633 --stroke '#ffffff' --stroke-width 2 --dash 10,7 --opacity 75
pixelox shape line guide.pxo --in-place --name Dash2 --from 550,902 --to 983,902 --stroke '#ffffff' --stroke-width 2 --dash 10,7 --opacity 75
pixelox shape line guide.pxo --in-place --name Dash3 --from 97,1171 --to 530,1171 --stroke '#ffffff' --stroke-width 2 --dash 10,7 --opacity 75
pixelox shape line guide.pxo --in-place --name Dash4 --from 550,1440 --to 983,1440 --stroke '#ffffff' --stroke-width 2 --dash 10,7 --opacity 75
pixelox shape line guide.pxo --in-place --name Dash5 --from 97,1709 --to 530,1709 --stroke '#ffffff' --stroke-width 2 --dash 10,7 --opacity 75
pixelox shape icon guide.pxo messages-square --in-place --name Pic1 --size 150 --at 263,498 --anchor c --color '#5a17b5' --fill '#ffffff' --stroke-width 8
pixelox text add guide.pxo --in-place --name Head1 --content $'1. Ask ten neighbors\nwhat they would borrow.' --font @neo-grotesque --weight 700 --size 25 --leading 32 --color '#5a17b5' --at 687,446 --anchor tl
pixelox text add guide.pxo --in-place --name Body1 --content "A drill, a ladder, and a wheelbarrow usually top the list." --font @neo-grotesque --size 19 --leading 25 --box 304x110 --color '#1f1f38' --at 687,516 --anchor tl
pixelox shape icon guide.pxo key --in-place --name Pic2 --size 150 --at 817,767 --anchor c --color '#5a17b5' --fill '#ffffff' --stroke-width 8
pixelox text add guide.pxo --in-place --name Head2 --content $'2. Find a dry room with\na door that locks.' --font @neo-grotesque --weight 700 --size 25 --leading 32 --color '#5a17b5' --at 110,715 --anchor tl
pixelox text add guide.pxo --in-place --name Body2 --content "A garage bay, a shed, or one shelf in the community hall is enough to begin." --font @neo-grotesque --size 19 --leading 25 --box 304x110 --color '#1f1f38' --at 110,785 --anchor tl
pixelox shape icon guide.pxo hammer --in-place --name Pic3 --size 150 --at 263,1036 --anchor c --color '#5a17b5' --fill '#ffffff' --stroke-width 8
pixelox text add guide.pxo --in-place --name Head3 --content $'3. Collect the tools\npeople already own.' --font @neo-grotesque --weight 700 --size 25 --leading 32 --color '#5a17b5' --at 687,984 --anchor tl
pixelox text add guide.pxo --in-place --name Body3 --content "Most households have duplicates. Label every item with a number and its owner." --font @neo-grotesque --size 19 --leading 25 --box 304x110 --color '#1f1f38' --at 687,1054 --anchor tl
pixelox shape icon guide.pxo clipboard-list --in-place --name Pic4 --size 150 --at 817,1305 --anchor c --color '#5a17b5' --fill '#ffffff' --stroke-width 8
pixelox text add guide.pxo --in-place --name Head4 --content $'4. Keep the lending\nrules to one page.' --font @neo-grotesque --weight 700 --size 25 --leading 32 --color '#5a17b5' --at 110,1253 --anchor tl
pixelox text add guide.pxo --in-place --name Body4 --content "Seven days per loan, return it clean, and tell us if it breaks." --font @neo-grotesque --size 19 --leading 25 --box 304x110 --color '#1f1f38' --at 110,1323 --anchor tl
pixelox shape icon guide.pxo clock --in-place --name Pic5 --size 150 --at 263,1574 --anchor c --color '#5a17b5' --fill '#ffffff' --stroke-width 8
pixelox text add guide.pxo --in-place --name Head5 --content $'5. Open the same two\nhours every week.' --font @neo-grotesque --weight 700 --size 25 --leading 32 --color '#5a17b5' --at 687,1522 --anchor tl
pixelox text add guide.pxo --in-place --name Body5 --content "A fixed slot beats long opening times that nobody remembers." --font @neo-grotesque --size 19 --leading 25 --box 304x110 --color '#1f1f38' --at 687,1592 --anchor tl
pixelox shape icon guide.pxo chart-column --in-place --name Pic6 --size 150 --at 817,1843 --anchor c --color '#5a17b5' --fill '#ffffff' --stroke-width 8
pixelox text add guide.pxo --in-place --name Head6 --content $'6. Count the loans and\nshare the number.' --font @neo-grotesque --weight 700 --size 25 --leading 32 --color '#5a17b5' --at 110,1791 --anchor tl
pixelox text add guide.pxo --in-place --name Body6 --content "A simple tally shows the street what it saved together." --font @neo-grotesque --size 19 --leading 25 --box 304x110 --color '#1f1f38' --at 110,1861 --anchor tl
pixelox export guide.pxo -o guide.png
```

Reuse: change the title and the six headings and notes; write each heading on two lines
with `\n`. The disc centers come from `lab/anchors.py scripts/steps.js --size 1080x2140
--param box=10,17,80,75.4 --param dir=v --param side=alt --param node=200 --param
labels=' | | | | | '`. For a disc at y: heading at y - 52, note at y + 18, picture at y, the
seam after it at y + 135. Odd steps have text at x = 687 and the picture at 263; even steps
text at 110 and the picture at 817. One more step adds 269 px to the canvas height: raise
the box height by the same share and add a label.

## 13. Variations and pairings

* **Script text:** give `labels=` and `notes=` to `scripts/steps.js` and drop the native
  text: the blocks then hug the spine (right-aligned on the left side). Faster, slightly
  less tidy.
* **Other meshes:** peach to mint (`#ffd3b0`, `#ffe9a8`, `#b8f0d2`, `#9be3e6`), or a dark
  mesh (`#1b1140`, `#3a1c71`, `#0f3b6e`) with white type and `#f5f5f5` discs.
* **Four steps:** canvas 1080 x 1620, `box=10,22,80,66`.
* **With figures:** put a statistic in each heading ("3. 40% reply within a day") and add a
  C-source-line in the footer zone.
* Pairs with C-stat-row above the footer (three results of following the steps) and with
  the palette of i035 for a flat, corporate version.

## 14. Pitfalls

* **Dark blobs under text.** A saturated corner of the mesh under 19 px notes kills
  contrast. Keep every mesh color above 60% lightness or move the dark blob to the header.
* **Pictures in six styles.** One 3D render, one flat icon, one photo: the page falls apart.
  All seven pictures share one style, one light direction, one palette.
* **Headings without numbers.** The discs and the headings are 150 px apart; when a step is
  quoted or cropped the number in the heading is what keeps the order. Keep both.
* **Steps that are not sequential.** Six parallel tips on a numbered spine misstate the
  material. For tips, drop the digits and the spine and use C-icon-grid. This is the
  data-integrity rule for step sheets: a number must mean an order.
* **Text hugging the spine on one side only.** Mixed alignment (flush left on the right,
  centered or right-aligned on the left) looks accidental. Choose one system for both sides.
* **Long notes.** Four lines is the limit beside a 200 px disc; a fifth runs into the seam.
