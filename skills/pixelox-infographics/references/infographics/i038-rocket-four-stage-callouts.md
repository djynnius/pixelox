---
id: i038
title: Rocket in four numbered bands with side callouts
kind: [process, anatomy]
topic: [business, generic]
mood: [vibrant, playful, energetic, friendly]
style: [flat, gradient, illustrated]
format: square
structure: S-object-callouts
data: [process, text-only]
charts: [C-rocket, C-callout, C-icon-badge]
palette: ['#ffffff', '#fd4e7d', '#7b5cf0', '#1fe0b3', '#ffb400', '#3d3566', '#d5f6f1']
needs: nothing
---

# i038: Rocket in four numbered bands with side callouts

## 1. Concept

A square process graphic on white: a candy-colored rocket stands in the middle, cut into four
horizontal bands numbered 01 to 04 from the nose down, and each band has a titled callout
with an icon, hung alternately to the right and to the left on a thin leader. The message is
"four stages take you to launch". It works because the object is the list: the reader counts
the stages on the rocket before reading a word, and the band colors tie each callout to its
stage without a legend.

## 2. Data story

No numbers. Text only, as an ordered list:

* 4 stages, each a title of 1 to 2 words and a note of 10 to 14 words on 3 lines.
* 2 extra options (labelled A and B), each a note of 5 to 7 words on 2 lines.
* 1 two-part title (one word, then 1 to 3 words).

Placeholder content in the recipe (invented): stages THE IDEA, THE TEAM, THE PRODUCT, THE
FUEL with one-sentence notes; options "Plan A" and "Plan B".

## 3. Information architecture

1. **Title**, top left: what the object stands for.
2. **The rocket**, center: four bands, 01 at the nose, 04 above the engine. Reading order is
   top to bottom, which is also the direction of a countdown, not of flight; the numbers
   make the order explicit.
3. **Four callouts**: 01 right, 02 left, 03 right, 04 left. Each: icon, title, a rule, a
   3-line note. A leader runs from the rule's inner end to the band's edge.
4. **Two options**, bottom right: block arrows A and B pointing up, a note beside each.
5. **Cloud**, bottom: a pale launch cloud that grounds the rocket and closes the page.

## 4. Format and grid

* Square. Recipe canvas 1080 x 1080. Margins: 9% W left and right for text.
* Vertical axis of the rocket at 48.4% W. Rocket box `32.8%,7.2%,31.3% x 79%` (nose tip at
  7.2% H, flame tip at 86% H); the body itself is 18% W wide, the fins reach 31% W.
* Left callout column: 8.7% to 36% W. Right callout column: 63.5% to 89% W.
* Callout rules at 17.7% H (01), 33.4% H (02), 44.7% H (03), 65.4% H (04): each sits 4 to 5%
  H **above** its band's middle, so the leader slants down toward the rocket.
* Title block at `9.3%,8.1%`; cloud `17.6%,79.6%,65% x 20%`; options `64%,58%,26% x 23%`.

## 5. Layer stack

1. Ground: white.
2. Cloud: three overlapping ellipses in pale mint `#d5f6f1`.
3. Rocket (one generated image): nose band pink `#fd4e7d`, then violet `#7b5cf0`, mint
   `#1fe0b3`, amber `#ffb400`; two magenta fins `#e0207f` beside bands 3 and 4; a dark
   nozzle; an orange flame with a yellow core; a soft light strip down the left of the body.
   Band numbers in a darker, saturated tone of each band (red on pink, blue on violet, green
   on mint, red-orange on amber).
4. Title: one tall condensed word in dark purple `#3d3566`, a magenta line of light caps
   under it.
5. Four leaders: 2 px dark purple lines, one elbow each.
6. Four icons (60 px line icons in magenta, violet, teal, amber) and four titles.
7. Four notes in gray `#6b6b76`.
8. Two block arrows (pink `#f6679b`, mint `#5fe3cf`) with white letters, and their notes.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#ffffff` | ground | 72% | nothing |
| `#fd4e7d` | band 01, arrow A (lighter `#f6679b`), icon 01 (`#e0409a`) | 4% | stage 1 |
| `#7b5cf0` | band 02, icon 02 (`#8d4fd0`) | 4% | stage 2 |
| `#1fe0b3` | band 03, icon 03 (`#19c6a6`), arrow B (`#5fe3cf`) | 4% | stage 3 |
| `#ffb400` | band 04, icon 04 (`#f5b800`), flame | 4% | stage 4 |
| `#3d3566` | title, callout titles, leaders | 3% | nothing: all structure |
| `#d5f6f1` | cloud | 8% | nothing |

Categorical: four saturated hues of equal weight, one per stage, on white, with one dark
neutral for all type. Each callout's icon repeats its band's hue; that is the legend.

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Title | 1 word, 7 to 8 letters | caps | condensed gothic (Oswald, Barlow Condensed / Avenir Next Condensed) | 600 | 104 | 0 | - | left | `#3d3566` | 9.3%, cap top 8.1% |
| Subtitle | 1 to 3 words | caps | geometric sans (Montserrat / Avenir Next) | 500 (the original: light) | 34 | 30 | - | left | `#d12f8e` | 9.4%, cap top 16.9% |
| Band number (x4) | 01 to 04 | - | condensed gothic | 600 | fitted: about 80 | 0 | - | center | a dark tone of the band | band centers |
| Callout title (x4) | 1 to 2 words, up to 11 letters | caps | condensed gothic | 600 | 42 | 0 | - | left of or right of its icon | `#3d3566` | 2.5% H above the rule |
| Callout note (x4) | 10 to 14 words on 3 lines | sentence | neo-grotesque (Inter, Roboto / Helvetica Neue) | 400 | 20 | 0 | 132% | center | `#6b6b76` | under the rule, centered on the column |
| Arrow letter (x2) | A, B | caps | condensed gothic | 600 | 40 | 0 | - | center | `#ffffff` | arrow shafts |
| Option note (x2) | 5 to 7 words on 2 lines | sentence | neo-grotesque | 400 | 18 | 0 | 130% | left | `#6b6b76` | right of each arrow |

Two families: a condensed gothic for everything that names (title, numbers, callout titles)
and a neutral sans for the notes. Sizes 104 : 42 : 20, about 5 : 2 : 1.

## 8. Charts and diagrams

* **C-rocket**: `scripts/object.js`, `kind=rocket`, `bands=4`, box `32.8,7.2,31.3,79`,
  `colors=` the four band hues, `badgecolor=` four darker tones, `fincolor='#e0207f'`,
  numbers in a condensed stack (`numfont=`). The script logs each band's center and its left
  and right edge points; the leaders end 16 px outside those points.
* **C-callout** x4: `shape path` with one elbow: from the band edge up to the rule's inner
  end, then along the rule (265 px long).
* The original rocket has gradient bands, a darker collar between the nose and band 02, and
  bulging body sections; the script draws flat bands with one highlight strip.

## 9. Icons and illustration

* Callout icons, 60 px, 5 px line, each in its stage's hue: `chart-column` (01), `target`
  (02), `settings` (03), `lightbulb` (04). The original uses filled two-tone icons (a falling
  bar chart, a target with an arrow, two gears, a glowing bulb).
* Icons sit beside the callout titles: right column icon first, then the title; left column
  title first, then the icon, so icons always face the rocket.
* Block arrows: `shape path`, 88 px wide heads on 44 px shafts.

## 10. Effects and finish

* The rocket's light strip (`shade=1`, the default) is the only shading.
* No shadows, no outlines. The original adds soft gradients inside each band (lighter at the
  top); add `fx overlay` with a white-to-transparent gradient on the placed rocket layer if
  you want that gloss.

## 11. Hierarchy and eye path

1. The rocket: the only large colored shape, on the center axis.
2. The band numbers, top to bottom.
3. The title, top left (dark, tall, but outside the color).
4. Callouts in number order, zig-zagging right, left, right, left.
5. Arrows A and B, then the cloud.

Title 104 px, callout titles 42, notes 20: 5 : 2 : 1.

## 12. Build recipe

Nothing to supply. Canvas 1080 x 1080.

```bash
pixelox new -o rocket.pxo --size 1080x1080 --fill '#ffffff'
pixelox shape ellipse rocket.pxo --in-place --name Cloud1 --rect 190,930,700x170 --fill '#d5f6f1'
pixelox shape ellipse rocket.pxo --in-place --name Cloud2 --rect 290,885,290x170 --fill '#d5f6f1'
pixelox shape ellipse rocket.pxo --in-place --name Cloud3 --rect 470,860,330x210 --fill '#d5f6f1'
pixelox render js scripts/object.js -o body.png --size 1080x1080 --param box=32.8,7.2,31.3,79 --param kind=rocket --param bands=4 --param colors='#fd4e7d,#7b5cf0,#1fe0b3,#ffb400' --param badgecolor='#e8121d,#2b2fd8,#0fa37f,#e8321a' --param fincolor='#e0207f' --param numfont='Avenir Next Condensed, Oswald, Arial Narrow, sans-serif' --param numweight=600
pixelox comp place rocket.pxo body.png --in-place --as Rocket --at 0,0
pixelox text add rocket.pxo --in-place --name Title --content "LIFT-OFF" --font @condensed --weight 600 --size 104 --color '#3d3566' --at 100,88 --anchor tl --anchor-to ink
pixelox text add rocket.pxo --in-place --name Sub --content "A FOUR-STAGE PLAN" --font @geometric-sans --weight 500 --size 34 --tracking 30 --color '#d12f8e' --at 102,182 --anchor tl --anchor-to ink
pixelox shape path rocket.pxo --in-place --name Lead1 --d 'M 622 241 L 686 191 L 951 191' --fill none --stroke '#3d3566' --stroke-width 2
pixelox shape icon rocket.pxo chart-column --in-place --name Ic1 --size 60 --at 724,144 --anchor c --color '#e0409a' --stroke-width 5
pixelox text add rocket.pxo --in-place --name H1 --content "THE IDEA" --font @condensed --weight 600 --size 42 --color '#3d3566' --at 772,146 --anchor l
pixelox text add rocket.pxo --in-place --name B1 --content $'Find one problem that\ncosts people time or money\nevery single week.' --font @neo-grotesque --size 20 --leading 132% --align center --color '#6b6b76' --at 819,206 --anchor t
pixelox shape path rocket.pxo --in-place --name Lead2 --d 'M 410 401 L 361 361 L 94 361' --fill none --stroke '#3d3566' --stroke-width 2
pixelox text add rocket.pxo --in-place --name H2 --content "THE TEAM" --font @condensed --weight 600 --size 42 --color '#3d3566' --at 112,316 --anchor l
pixelox shape icon rocket.pxo target --in-place --name Ic2 --size 60 --at 306,312 --anchor c --color '#8d4fd0' --stroke-width 5
pixelox text add rocket.pxo --in-place --name B2 --content $'Two or three people who\ncover building, selling,\nand keeping the books.' --font @neo-grotesque --size 20 --leading 132% --align center --color '#6b6b76' --at 227,376 --anchor t
pixelox shape path rocket.pxo --in-place --name Lead3 --d 'M 637 538 L 678 483 L 959 483' --fill none --stroke '#3d3566' --stroke-width 2
pixelox shape icon rocket.pxo settings --in-place --name Ic3 --size 60 --at 742,436 --anchor c --color '#19c6a6' --stroke-width 5
pixelox text add rocket.pxo --in-place --name H3 --content "THE PRODUCT" --font @condensed --weight 600 --size 42 --color '#3d3566' --at 790,438 --anchor l
pixelox text add rocket.pxo --in-place --name B3 --content $'Ship the smallest version\nthat solves it, then improve\nit with every user.' --font @neo-grotesque --size 20 --leading 132% --align center --color '#6b6b76' --at 819,498 --anchor t
pixelox shape path rocket.pxo --in-place --name Lead4 --d 'M 410 676 L 376 706 L 100 706' --fill none --stroke '#3d3566' --stroke-width 2
pixelox text add rocket.pxo --in-place --name H4 --content "THE FUEL" --font @condensed --weight 600 --size 42 --color '#3d3566' --at 116,661 --anchor l
pixelox shape icon rocket.pxo lightbulb --in-place --name Ic4 --size 60 --at 296,655 --anchor c --color '#f5b800' --stroke-width 5
pixelox text add rocket.pxo --in-place --name B4 --content $'Raise only what the\nnext stage needs, from\npeople in your market.' --font @neo-grotesque --size 20 --leading 132% --align center --color '#6b6b76' --at 232,722 --anchor t
pixelox shape path rocket.pxo --in-place --name ArrowA --d 'M 756 628 L 800 674 L 778 674 L 778 752 L 734 752 L 734 674 L 712 674 Z' --fill '#f6679b'
pixelox text add rocket.pxo --in-place --name LA --content "A" --font @condensed --weight 600 --size 40 --color '#ffffff' --at 756,714 --anchor c --anchor-to ink
pixelox text add rocket.pxo --in-place --name NA --content $'Plan A: grow on\nrevenue alone.' --font @neo-grotesque --size 18 --leading 130% --color '#6b6b76' --at 818,696 --anchor tl
pixelox shape path rocket.pxo --in-place --name ArrowB --d 'M 736 752 L 780 798 L 758 798 L 758 866 L 714 866 L 714 798 L 692 798 Z' --fill '#5fe3cf'
pixelox text add rocket.pxo --in-place --name LB --content "B" --font @condensed --weight 600 --size 40 --color '#ffffff' --at 736,832 --anchor c --anchor-to ink
pixelox text add rocket.pxo --in-place --name NB --content $'Plan B: raise a\nsmall first round.' --font @neo-grotesque --size 18 --leading 130% --color '#6b6b76' --at 798,812 --anchor tl
pixelox export rocket.pxo -o rocket.png
```

Reuse: change `bands=` and the four callouts. Run `scripts/object.js` with `--json` and read
the `band1` ... anchors: each leader starts at (`rx` + 16, `ry`) on the right or (`lx` - 16,
`ly`) on the left, rises 40 to 50 px to the rule, and the rule runs 265 px outward. Titles
sit 45 px above the rule, notes start 15 px under it. With 5 or 6 bands shrink the notes to 2
lines.

## 13. Variations and pairings

* **Other objects, same layout:** `kind=bulb` for ideas, `kind=pencil` for writing or
  planning, `kind=thermometer` or `battery` with `value=` for one level instead of stages.
* **Climbing rocket:** `rotate=30` and callouts stacked on the left only; the anchors follow
  the rotation.
* **Dark sky version:** ground `#101322`, type white, cloud `#1c2140`, stars as small white
  dots; keep the four band hues.
* **With numbers:** put a stat in each callout title place ("42%") and the label in the
  note: the layout becomes a four-fact sheet.
* Pairs with C-timeline along the bottom (dates of each stage) or a C-stat-row under the
  cloud in a 4:5 format.

## 14. Pitfalls

* **Callouts on the same height as their bands.** Horizontal leaders hide behind the fins
  and the text collides with them. Lift each rule above its band and slant the leader.
* **Text over the fins.** The fins reach 31% W. Keep the left column's right edge at 36% W
  only above band 3; beside the fins end the notes at 31% W.
* **Order unclear.** A rocket flies upward but the list reads downward. The numbers carry
  the order: never drop them, and never number from the bottom while the callouts read from
  the top.
* **Band heights that look like data.** The bands are equal steps, not amounts. If your
  stages have sizes (budget, weeks), do not resize the bands by eye: put the numbers in the
  callouts, or use C-funnel or C-stacked-columns. This is the data-integrity rule for every
  metaphor object.
* **Seven colors for four stages.** Icons and arrows reuse the band hues; adding new hues
  for them breaks the link between callout and band.
