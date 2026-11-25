---
id: i037
title: One-color icon sheet with label bands
kind: [explainer, list]
topic: [education, generic]
mood: [clean, minimal, serious, calm]
style: [flat, line-icon]
format: portrait
structure: S-icon-sheet
data: [text-only, single-stat]
charts: [C-section-band, C-icon-grid, C-icon-array, C-stat-block, C-pie, C-columns, C-org-tree, C-world-map, C-icon-badge]
palette: ['#f1f1f1', '#525252', '#ffffff', '#111111', '#828282', '#bdbdbd']
needs: nothing
---

# i037: One-color icon sheet with label bands

## 1. Concept

A classroom-style explainer printed in one ink: a dark gray header bar with a question, then
four pale gray panels separated by thin white gaps. The first panel defines the subject in
three cells; each of the other three opens with a dark label band that sits on the gap above
it and holds small pictograms, miniature charts, a silhouette map, and one-sentence facts.
The message is "here is what this thing is, in about fifteen small pictures". It works
because every picture is the same dark gray and about the same size, so nothing competes,
and the label bands give the page three clear stops.

## 2. Data story

Almost no data: this is a definition sheet.

* 3 defining words (1 or 2 words each), each with a picture and a caption of 2 to 3 words.
* 3 sections with a label of 3 or 4 words.
* Section A: 3 short statements (4 to 6 words) illustrated by 3 people pictograms, three
  miniature charts **without values** (a 3-slice pie, 3 falling columns, a 6-box tree), and a
  world silhouette; plus **one hero statistic** (a high percentage and one word).
* Section B: 4 benefits, each an icon and a sentence of 2 to 5 words, in a 2 x 2 grid.
* Section C: 4 facts, each an icon and a two-line caption; the last one is bold.

The miniature charts are pictures of charts, not charts of anything: they show what a pie or
a bar chart is. Placeholder content in the recipe (invented, about dashboards; the one figure,
80%, is example data).

## 3. Information architecture

1. **Header bar**: the question the sheet answers.
2. **Definition panel**: three cells side by side (word, picture, caption), split by two
   vertical rules.
3. **Section A** (label band): what the thing is made of. Left two thirds: pictograms with a
   sentence, the hero statistic, three mini charts over a one-line sentence. Right third,
   behind a vertical rule: a map and a two-line sentence.
4. **Section B** (label band): why to use it. Four icon-and-sentence pairs, two per row.
5. **Section C** (label band): four facts in four columns, icon above caption. The fourth
   icon is larger and its caption bold: the sheet ends on its rule of thumb.

Each band is centered and half the page wide, so the eye drops down the middle from band to
band and reads each panel left to right.

## 4. Format and grid

* Portrait, 1 : 1.39. Recipe canvas 1080 x 1500. Panels bleed to both edges; content
  margins 4% W.
* Vertical zones in %H: header 0 to 11.2; panel 1 to 34; gap; panel 2 from 34.9 to 61.4;
  gap; panel 3 from 62.3 to 83.3; gap; panel 4 from 84.1 to 100. Gaps are about 0.9% H.
* Label bands: `26.2%,34.3%,47.6% x 3.1%`, the same box at 61.9% H and 83.6% H. Each band
  starts 0.6% H above its panel, covering the gap.
* Panel 1: three columns centered on 16.7%, 50%, 83.3% W; rules at 33.3% and 66.7% W from
  17% to 32% H; words at 15% H, pictures centered at 24.3% H (16 to 22% W wide), captions at
  32% H.
* Panel 2: vertical rule at 62.3% W from 38.3% to 59.9% H. Left zone: pictograms
  `1.5%,39.1%,17.9% x 7.9%`, their sentence from 21.3% W, hero statistic centered on 50.7%
  W at 41.3% H with its word at 45.6% H; pie center 10.8% W, 53% H, radius 4.8% W; columns
  `23.1%,49%,8.6% x 7.8%`; tree `39%,49%,17% x 7.6%`; sentence at 4.7% W, 59.4% H. Right
  zone: map `64.7%,43.3%,32.3% x 11.7%`, its sentence at 73% W, 58.2% H.
* Panel 3: icon centers at 9% and 61.5% W; sentences start at 16.9% and 67.4% W; rows at
  70.3% and 77.8% H.
* Panel 4: four columns centered on 12.8%, 36.2%, 61.3%, 86% W; icons centered at 91% H;
  captions centered at 96.5% H.

## 5. Layer stack

1. Ground white `#ffffff` (it shows only as the gaps between panels).
2. Header bar `#525252`, full width, with the title in white.
3. Four panels `#f1f1f1`, full width.
4. Panel 1: three bold caps words in near black; two vertical rules, 3 px, `#525252`; three
   pictures in `#525252` (a jigsaw grid drawn in outline, a solid disc with a white serif
   "i", a magnifier holding a jigsaw piece); three captions.
5. Three label bands `#525252` with white bold caps.
6. Panel 2: three standing figures; a two-line sentence; the hero percentage in `#525252`
   with a black caps word under it; a pie in three grays with pale separators; three
   columns; a tree of six outlined boxes joined by elbow lines, each box holding a short
   dash; a one-line sentence; a vertical rule; a world silhouette; a two-line sentence.
7. Panel 3: four icons (an eye, a head with gears, a bulb with rays, a billboard), each with
   a sentence to its right.
8. Panel 4: a dark rounded square with white list bars, the letters "ABC", a starburst
   outline around a number sign, a prohibition sign over a camera; four captions.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#f1f1f1` | panels | 62% | nothing |
| `#525252` | header, bands, every icon and chart mark, rules | 19% | nothing: the one ink |
| `#ffffff` | gaps, type on dark, box fills | 11% | nothing |
| `#111111` | words, sentences, captions | 4% | nothing |
| `#828282` | pie's second slice | 1% | a second category (in a picture of a chart) |
| `#bdbdbd` | pie's third slice | 1% | a third category |

Monochrome: one dark gray on one pale gray. The two extra grays appear only inside the pie.
The whole sheet survives a photocopier, which is the point of the style; add one accent
color only for the hero statistic if you need a focus.

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Title | a question of 4 words | title | neo-grotesque (the original: a humanist sans such as Open Sans, Source Sans 3 / Helvetica Neue) | 700 | 64 | 0 | - | center | `#ffffff` | 50%, 5.6% |
| Cell word (x3) | 1 or 2 words, up to 11 letters | caps | neo-grotesque | 700 | 43 | 0 | - | center | `#111111` | column centers, 15% |
| Cell caption (x3) | 2 to 4 words | sentence | neo-grotesque | 400 | 23 | 0 | - | center | `#1a1a1a` | column centers, 32% |
| Band label (x3) | 3 or 4 words | caps | neo-grotesque (Arimo, Inter / Helvetica Neue) | 700 | 28 | 20 | - | center | `#ffffff` | 50%, band centers 35.9%, 63.4%, 85.1% |
| Statement (x3, panel 2) | 4 to 6 words on 1 or 2 lines | sentence | neo-grotesque | 500 to 600 | 26 | 0 | 37 px | left | `#111111` | 21.3%, 44.1% / 4.7%, 59.4% / 73%, 58.2% |
| Hero number | NN% | - | neo-grotesque | 700 | 90 | 0 | - | center | `#525252` | 50.7%, 41.3% |
| Hero word | 1 word | caps | neo-grotesque | 700 | 36 | 0 | - | center | `#111111` | 50.7%, 45.6% |
| Benefit (x4) | 2 to 5 words | sentence | neo-grotesque | 500 to 600 | 28 | 0 | - | left | `#111111` | 16.9% and 67.4% W; 70.3% and 77.8% H |
| Letters picture | "ABC" | caps | neo-grotesque | 700 | 47 | 80 | - | center | `#525252` | 36.2%, 91.5% |
| Fact caption (x3) | 5 to 7 words on 2 lines | sentence | neo-grotesque | 400 | 23 | 0 | 32 px | center | `#1a1a1a` | column centers, 96.5% |
| Closing fact | 5 words on 2 lines | sentence | neo-grotesque | 700 | 28 | 0 | 38 px | center | `#111111` | 86%, 96.6% |

One family, three weights. Sizes 90 : 64 : 43 : 28 : 23, about 4 : 2.8 : 1.9 : 1.2 : 1. Only
the hero number is larger than the title.

## 8. Charts and diagrams

All charts here are small pictures, about 9 to 18% W wide, with no axis, numbers, or legend.

* **C-icon-array**: `scripts/waffle.js`, `shape=person value=3 total=3`, `#525252`,
  `legend=none`, box `1.5,39.1,17.9,7.9`.
* **C-pie**: `scripts/donut.js` by center, `cx=10.8 cy=53 r=4.8`, `inner=0`,
  `values=26,8,66` (a medium slice from 12 o'clock, a sliver, then the dark rest),
  `labelpos=none`, `sep='#f1f1f1'`.
* **C-columns**: `scripts/bars.js`, box `23.1,49,8.6,7.8`, `values=9,6,4`, one gray,
  `axis=0 gridlines=0 vlabels=0 gap=0.25 radius=0`, no labels.
* **C-org-tree**: by hand, three `shape path` layers (links, five white boxes with 3 px
  outlines, a dash in each box): one box on top, one hanging to the left of the trunk, three
  in a row under a bar. The original has six boxes; five keep it readable at this size.
* **C-world-map**: `scripts/geo.js` run with `render d3`, box `64.7,43.3,32.3,11.7`,
  `projection=natural`, `land='#525252'`, `stroke='#525252' strokewidth=1.5` (borders in the
  land color close the seams between countries, so the map reads as one silhouette;
  `stroke=none` would also draw Antarctica), no points. **Map spec:** region set world
  countries without Antarctica; projection Natural Earth (the original is a rounded
  pseudo-cylindrical world, Robinson or similar, about 2 : 1); encoding none: one flat fill
  `#525252`, no visible borders; no labels, no legend, no markers. It only says "maps
  exist". Builds before 0.6.0: C-dot-map (`scripts/worldmap.js` with the same box,
  `shape=square fill=1 dot='#525252'`).
* **C-stat-block** for the hero percentage; **C-section-band** (the short centered variant)
  for the three labels; **C-icon-grid** for panels 1, 3, and 4.

## 9. Icons and illustration

One treatment: `#525252`, no second color. Lucide line icons replace the original's solid
glyphs.

* Panel 1 (pictures about 180 px): `layout-dashboard` 176 px, 9 px line (the original: a
  4 x 5 jigsaw grid in outline); a 180 px dark disc with `gauge` 112 px in white (the
  original: a white serif "i" in the disc); `search` 184 px, 10 px line, with `chart-pie`
  64 px inside the lens (the original: a magnifier over a jigsaw piece). The lens of
  `search` is centered 8 px up and left of the icon's center at this size.
* Panel 3 (100 px, 7 px line): `eye`, `brain-cog`, `lightbulb`, `presentation` (the
  original: an eye, a head in profile with two gears, a bulb with rays, a billboard on a
  post).
* Panel 4: a 92 px dark rounded square (radius 18) with `layout-list` 58 px in white; the
  letters as type; `shape star` with 14 points (outer 50, inner 37, 3 px outline) around
  `hash` 40 px; `camera` 84 px under `ban` 156 px, 9 px line (the prohibition sign is 1.6
  times the other icons and rises toward the band).

## 10. Effects and finish

None: flat, no shadows, no gradients, no rounded panels. The white gaps between panels are
the only ornament. The original is a small grayscale scan with soft edges; build it crisp.

## 11. Hierarchy and eye path

1. The header question (white on the darkest, widest shape).
2. The three bold words of panel 1 and their pictures.
3. The first band, then the hero percentage (the largest type on the sheet).
4. Down the bands: each is a stop; under each, icons first, sentences second.
5. The prohibition sign and the bold closing fact at the lower right.

Hero 90 px, title 64, cell words 43, sentences 26 to 28, captions 23.

## 12. Build recipe

Nothing to supply. Canvas 1080 x 1500. The one figure (80%) is example data.

```bash
pixelox new -o sheet.pxo --size 1080x1500 --fill '#ffffff'
pixelox shape rect sheet.pxo --in-place --name Header --rect 0,0,1080x168 --fill '#525252'
pixelox text add sheet.pxo --in-place --name Title --content "What Is a Dashboard?" --font @neo-grotesque --weight 700 --size 64 --color '#ffffff' --at 540,84 --anchor c --anchor-to ink
pixelox shape rect sheet.pxo --in-place --name Panel1 --rect 0,168,1080x342 --fill '#f1f1f1'
pixelox shape rect sheet.pxo --in-place --name Panel2 --rect 0,524,1080x397 --fill '#f1f1f1'
pixelox shape rect sheet.pxo --in-place --name Panel3 --rect 0,935,1080x315 --fill '#f1f1f1'
pixelox shape rect sheet.pxo --in-place --name Panel4 --rect 0,1261,1080x239 --fill '#f1f1f1'
pixelox text add sheet.pxo --in-place --name H1 --content "OVERVIEW" --font @neo-grotesque --weight 700 --size 43 --color '#111111' --at 180,225 --anchor c --anchor-to ink
pixelox text add sheet.pxo --in-place --name H2 --content "STATUS" --font @neo-grotesque --weight 700 --size 43 --color '#111111' --at 540,225 --anchor c --anchor-to ink
pixelox text add sheet.pxo --in-place --name H3 --content "DETAIL" --font @neo-grotesque --weight 700 --size 43 --color '#111111' --at 900,225 --anchor c --anchor-to ink
pixelox shape path sheet.pxo --in-place --name Dividers1 --d 'M 360 255 L 360 482 M 720 255 L 720 482' --fill none --stroke '#525252' --stroke-width 3
pixelox shape icon sheet.pxo layout-dashboard --in-place --name Pic1 --size 176 --at 180,365 --anchor c --color '#525252' --stroke-width 9
pixelox shape ellipse sheet.pxo --in-place --name Disc2 --rect 450,275,180x180 --fill '#525252'
pixelox shape icon sheet.pxo gauge --in-place --name Pic2 --size 112 --at 540,365 --anchor c --color '#ffffff' --stroke-width 9
pixelox shape icon sheet.pxo search --in-place --name Pic3 --size 184 --at 900,365 --anchor c --color '#525252' --stroke-width 10
pixelox shape icon sheet.pxo chart-pie --in-place --name Pic3b --size 64 --at 892,357 --anchor c --color '#525252' --stroke-width 5
pixelox text add sheet.pxo --in-place --name C1 --content "Show the whole picture." --font @neo-grotesque --size 23 --color '#1a1a1a' --at 180,480 --anchor c
pixelox text add sheet.pxo --in-place --name C2 --content "Report where things stand." --font @neo-grotesque --size 23 --color '#1a1a1a' --at 540,480 --anchor c
pixelox text add sheet.pxo --in-place --name C3 --content "Open any number up." --font @neo-grotesque --size 23 --color '#1a1a1a' --at 900,480 --anchor c
pixelox shape rect sheet.pxo --in-place --name Band1 --rect 283,515,514x46 --fill '#525252'
pixelox text add sheet.pxo --in-place --name Band1T --content "CHARTS CARRY THE NUMBERS" --font @neo-grotesque --weight 700 --size 28 --tracking 20 --color '#ffffff' --at 540,538 --anchor c --anchor-to ink
pixelox render js scripts/waffle.js -o people.png --size 1080x1500 --param box=1.5,39.1,17.9,7.9 --param value=3 --param total=3 --param shape=person --param colors='#525252' --param legend=none
pixelox comp place sheet.pxo people.png --in-place --as People --at 0,0
pixelox text add sheet.pxo --in-place --name PeopleT --content $'Pictograms\ncount people.' --font @neo-grotesque --weight 500 --size 26 --leading 37 --color '#111111' --at 230,661 --anchor l
pixelox text add sheet.pxo --in-place --name Hero --content "80%" --font @neo-grotesque --weight 700 --size 90 --color '#525252' --at 548,620 --anchor c --anchor-to ink
pixelox text add sheet.pxo --in-place --name HeroL --content "CHARTS" --font @neo-grotesque --weight 700 --size 36 --color '#111111' --at 548,684 --anchor c --anchor-to ink
pixelox render js scripts/donut.js -o pie.png --size 1080x1500 --param cx=10.8 --param cy=53 --param r=4.8 --param inner=0 --param values=26,8,66 --param colors='#828282,#bdbdbd,#525252' --param labelpos=none --param sep='#f1f1f1'
pixelox comp place sheet.pxo pie.png --in-place --as Pie --at 0,0
pixelox render js scripts/bars.js -o cols.png --size 1080x1500 --param box=23.1,49,8.6,7.8 --param values=9,6,4 --param colors='#525252' --param axis=0 --param gridlines=0 --param vlabels=0 --param gap=0.25 --param radius=0
pixelox comp place sheet.pxo cols.png --in-place --as Columns --at 0,0
pixelox shape path sheet.pxo --in-place --name TreeLinks --d 'M 512 763 L 512 813 M 449 813 L 577 813 M 449 813 L 449 823 M 512 813 L 512 823 M 577 813 L 577 823 M 512 789 L 492 789' --fill none --stroke '#525252' --stroke-width 3
pixelox shape path sheet.pxo --in-place --name TreeBoxes --d 'M 485 737 H 539 V 763 H 485 Z M 438 776 H 492 V 802 H 438 Z M 422 823 H 476 V 849 H 422 Z M 485 823 H 539 V 849 H 485 Z M 550 823 H 604 V 849 H 550 Z' --fill '#ffffff' --stroke '#525252' --stroke-width 3
pixelox shape path sheet.pxo --in-place --name TreeMarks --d 'M 502 750 H 522 M 455 789 H 475 M 439 836 H 459 M 502 836 H 522 M 567 836 H 587' --fill none --stroke '#525252' --stroke-width 4
pixelox text add sheet.pxo --in-place --name ChartsT --content "Pies, bars, and trees show how parts relate." --font @neo-grotesque --weight 500 --size 26 --color '#111111' --at 51,891 --anchor l
pixelox shape line sheet.pxo --in-place --name Divider2 --from 673,574 --to 673,898 --stroke '#525252' --stroke-width 3
pixelox render d3 scripts/geo.js -o world.png --size 1080x1500 --param box=64.7,43.3,32.3,11.7 --param projection=natural --param land='#525252' --param stroke='#525252' --param strokewidth=1.5
pixelox comp place sheet.pxo world.png --in-place --as World --at 0,0
pixelox text add sheet.pxo --in-place --name MapT --content $'Maps show\nwhere.' --font @neo-grotesque --weight 500 --size 26 --leading 37 --color '#111111' --at 790,873 --anchor l
pixelox shape rect sheet.pxo --in-place --name Band2 --rect 283,928,514x46 --fill '#525252'
pixelox text add sheet.pxo --in-place --name Band2T --content "WHY TEAMS USE ONE" --font @neo-grotesque --weight 700 --size 28 --tracking 20 --color '#ffffff' --at 540,951 --anchor c --anchor-to ink
pixelox shape icon sheet.pxo eye --in-place --name W1 --size 104 --at 97,1054 --anchor c --color '#525252' --stroke-width 7
pixelox shape icon sheet.pxo brain-cog --in-place --name W2 --size 100 --at 97,1167 --anchor c --color '#525252' --stroke-width 7
pixelox shape icon sheet.pxo lightbulb --in-place --name W3 --size 100 --at 664,1054 --anchor c --color '#525252' --stroke-width 7
pixelox shape icon sheet.pxo presentation --in-place --name W4 --size 100 --at 664,1167 --anchor c --color '#525252' --stroke-width 7
pixelox text add sheet.pxo --in-place --name W1T --content "See problems early." --font @neo-grotesque --weight 500 --size 28 --color '#111111' --at 182,1054 --anchor l
pixelox text add sheet.pxo --in-place --name W2T --content "Decide with evidence." --font @neo-grotesque --weight 500 --size 28 --color '#111111' --at 182,1167 --anchor l
pixelox text add sheet.pxo --in-place --name W3T --content "Spot what is changing." --font @neo-grotesque --weight 500 --size 28 --color '#111111' --at 728,1054 --anchor l
pixelox text add sheet.pxo --in-place --name W4T --content "Share one view of it." --font @neo-grotesque --weight 500 --size 28 --color '#111111' --at 728,1167 --anchor l
pixelox shape rect sheet.pxo --in-place --name Band3 --rect 283,1254,514x46 --fill '#525252'
pixelox text add sheet.pxo --in-place --name Band3T --content "GOOD TO KNOW" --font @neo-grotesque --weight 700 --size 28 --tracking 20 --color '#ffffff' --at 540,1277 --anchor c --anchor-to ink
pixelox shape rect sheet.pxo --in-place --name K1 --rect 92,1318,92x92 --radius 18 --fill '#525252'
pixelox shape icon sheet.pxo layout-list --in-place --name K1i --size 58 --at 138,1364 --anchor c --color '#ffffff' --stroke-width 5
pixelox text add sheet.pxo --in-place --name K2 --content "ABC" --font @neo-grotesque --weight 700 --size 47 --tracking 80 --color '#525252' --at 391,1372 --anchor c --anchor-to ink
pixelox shape star sheet.pxo --in-place --name K3 --center 662,1366 --points 14 --outer 50 --inner 37 --fill none --stroke '#525252' --stroke-width 3
pixelox shape icon sheet.pxo hash --in-place --name K3i --size 40 --at 662,1366 --anchor c --color '#525252' --stroke-width 4
pixelox shape icon sheet.pxo camera --in-place --name K4i --size 84 --at 928,1340 --anchor c --color '#525252' --stroke-width 7
pixelox shape icon sheet.pxo ban --in-place --name K4 --size 156 --at 928,1340 --anchor c --color '#525252' --stroke-width 9
pixelox text add sheet.pxo --in-place --name K1T --content $'Numbers are grouped\ninto cards.' --font @neo-grotesque --size 23 --leading 32 --align center --color '#1a1a1a' --at 138,1447 --anchor c
pixelox text add sheet.pxo --in-place --name K2T --content $'Every chart is labeled\nin plain words.' --font @neo-grotesque --size 23 --leading 32 --align center --color '#1a1a1a' --at 391,1447 --anchor c
pixelox text add sheet.pxo --in-place --name K3T --content $'Key numbers are\nset the largest.' --font @neo-grotesque --size 23 --leading 32 --align center --color '#1a1a1a' --at 662,1447 --anchor c
pixelox text add sheet.pxo --in-place --name K4T --content $'Charts are live,\nnot screenshots.' --font @neo-grotesque --weight 700 --size 28 --leading 38 --align center --color '#111111' --at 928,1449 --anchor c
pixelox export sheet.pxo -o sheet.png
```

Reuse: keep the zones and swap the content panel by panel. Every panel is a C-icon-grid
variant: 3 cells with rules, 2 x 2 icon-and-sentence pairs, or 4 columns. Change the band
labels to the questions your panels answer. The tree's three paths share one origin: add the
same number to every x to move it. If a panel needs real data, replace a mini chart with a
labeled one (C-bars with `fs=20`) and give it the full left zone; add a source line in the
last panel. A statement that needs a third line is too long for this sheet.

## 13. Variations and pairings

* **One accent:** keep everything gray and set the hero number and the three bands in one
  color (for example `#e4572e` or `#1466a3`).
* **Dark sheet:** panels `#2b2b2b`, gaps `#1c1c1c`, icons and type `#eaeaea`, bands `#eaeaea`
  with dark type.
* **Three sections instead of four:** drop panel 3 for a 1080 x 1200 handout.
* **Glossary sheet:** every panel as four columns (icon, term, one line): 12 terms per page.
* Pairs with C-process-row in place of the 2 x 2 panel when the benefits are steps, and
  with C-table in the last panel for a quick reference.

## 14. Pitfalls

* **Mini charts that look like data.** A pie and bars without numbers are fine as pictures
  of charts, as here. The moment the sheet is about a real subject, a reader takes them for
  results: label them with values and a source, or replace them with icons (`chart-pie`,
  `chart-column`).
* **A hero number without its base.** "NN%" over one word says nothing checkable. Add what
  it is a share of and where it comes from in the sentence beside it.
* **Mixed icon styles.** Solid glyphs beside line icons beside clip art break the one-ink
  idea. One set, one line weight (about size / 14), one gray.
* **Bands as wide as the page.** Full-width dark bands compete with the header. Keep them
  to about half the width, centered, straddling the gap.
* **Sentences set too small to keep everything.** Fifteen pictures tempt you to shrink the
  text to 16 px. Keep 23 px as the floor and cut a picture instead.
* **Gray on gray without contrast.** `#525252` on `#f1f1f1` has enough contrast; lighter
  icons (`#9a9a9a`) do not. Check small captions on the export.
