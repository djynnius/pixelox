---
id: i020
title: Flat three-band rocket with numbered notes
kind: [process, list]
topic: [business, generic]
mood: [clean, friendly, minimal]
style: [flat]
format: landscape
structure: S-object-rows
data: [process, text-only]
charts: [C-rocket, C-callout]
palette: ['#ffffff', '#e55360', '#43aad5', '#fdae49', '#466077', '#7a7a7a']
needs: nothing
---

# i020: Flat three-band rocket with numbered notes

## 1. Concept

A landscape slide on white: a chubby flat rocket stands in the left third, cut into three
bands (orange nose, blue body with a porthole, red base with fins), and each band sends a
short thick rule to the right, where a giant number, a caps title, and a paragraph wait in
the band's color. The message is "three stages, read top to bottom". It works because color
does all the linking: rule, number, and title repeat the band's hue, so the reader ties each
paragraph to its part of the rocket without a legend, and the rocket itself carries no text
at all.

## 2. Data story

No numbers. Text only, as an ordered list of three:

* 3 stages, each a title of 1 to 3 words (up to 14 letters) and a note of 25 to 35 words on
  5 or 6 lines.
* 1 closing remark of 12 to 15 words on 3 lines, set under the third note as its second
  paragraph.
* 1 two-word wordmark under the rocket (about 5 and 2 letters), the first word in slate, the
  second in the base band's red.

Placeholder content in the recipe (invented, about starting a small product): FIND THE GAP,
BUILD THE CORE, LIGHT THE ENGINE, each with a five- or six-line note; a three-line remark
about the first ninety days; the wordmark SHIP IT.

## 3. Information architecture

1. **The rocket**, left: three bands of three colors. It answers "how many stages" before a
   word is read. The order runs from the nose down.
2. **Three rules**: one per band, leaving the rocket at the band's middle and all ending on
   the same vertical line. They are the leaders, drawn as thick as a letter stroke.
3. **Three numbers** 01 to 03, directly after the rule ends: the reading order.
4. **Three text blocks** right of the numbers: title on the number's top line, note under it.
5. **Wordmark**, under the rocket between its fins: the name of the whole thing. It is read
   last, although it is the second-largest type.

The eye goes rocket, then right along a rule, number, title, note, and drops to the next
rule. There is no title line on top: the wordmark under the object is the title.

## 4. Format and grid

* Landscape, 1.36 : 1 (close to 4:3). Recipe canvas 1600 x 1180.
* Margins: 10.7% W left, 8.5% W right, 8.8% H top, 7.3% H bottom.
* Object column (the left 45% W, nearly full height): 10.7% to 44% W in the original, 8.8%
  to 45.8% W in the recipe. Rocket axis at 27.3% W; nose tip at 8.8% H, fin tips at 89.5% H;
  the body is 24% W wide and the fins span 33% W in the original (22% and 37% W in the
  recipe, which draws the stout rocket with `aspect=1.4`).
* Rule zone: from 3% W outside the rocket's edge to a common end at 52.7% W. The three rules
  start at 37%, 43%, and 47% W (40%, 41%, and 49% W in the recipe): their left ends follow
  the rocket's silhouette.
* Number column: 54.5% to 61% W. Text column: 64.3% to 91.5% W (27% W wide).
* Rows: rule centers at 17.6%, 42.7%, and 67.7% H, one every 25% H (the recipe hangs them on
  the script's band anchors at 23.9%, 45.4%, and 68.1% H).
* Wordmark: centered on the rocket axis, cap top at 86.8% H, 20 to 24% W wide.
* Everything in the text column is flush left on 64.3% W.

## 5. Layer stack

1. Ground: white, flat.
2. Rocket (one generated image): nose band orange `#fdae49` from 8.8% to 28.9% H; body band
   blue `#43aad5` from 30.5% to 54.9% H; base band red `#e55360` from 56.5% to 74.1% H,
   narrowing downward; two red fins curving out to 33% W; a slate nozzle `#466077` from
   74.7% to 82.5% H (three stacked steps in the original, one trapezoid in the recipe).
3. Two white gaps, 1.2 to 1.6% H tall, between the bands: the bands read as separate parts.
4. Porthole in the middle of the blue band: a ring `#f2f2f2` (radius 7.5% W in the original,
   6.9% W in the recipe, 14 px thick), a glass disc in the band's own blue, and a lighter
   crescent `#64b8de` over the glass's upper half.
5. Wordmark: two words, slate then red.
6. Three rules, 12 px thick with round ends, each in its band's color.
7. Three numbers in the band colors.
8. Three titles in the band colors and four gray paragraphs `#7a7a7a`.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#ffffff` | ground, gaps between bands | 79% | nothing |
| `#e55360` | base band and fins, rule 03, number 03, title 03, second word of the wordmark | 7% | stage 3 |
| `#43aad5` | body band, porthole glass (`#64b8de` highlight), rule 02, number 02, title 02 | 5% | stage 2 |
| `#fdae49` | nose band, rule 01, number 01, title 01 | 4% | stage 1 |
| `#466077` | nozzle, first word of the wordmark | 2% | nothing: structure |
| `#7a7a7a` | all paragraphs | 3% | nothing |

Categorical: three hues of similar lightness and saturation (a warm orange, a sky blue, a
soft red), one per stage, on white. Slate is the only dark and is used twice. There is no
separate accent: the three stage colors are the accents.

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Wordmark, word 1 | 1 word, 4 to 5 letters | caps | neo-grotesque (Roboto, Inter / Helvetica Neue) | 700 | 96 | 0 | - | right edge on the gap | `#466077` | ends at 31% W, cap top 86.8% H |
| Wordmark, word 2 | 1 word, 2 to 3 letters | caps | neo-grotesque | 700 | 96 | 0 | - | left | `#e55360` | starts 20 px after word 1 |
| Number (x3) | 01, 02, 03 | - | condensed gothic (Roboto Condensed, Oswald / Avenir Next Condensed) | 700 | 120 | 0 | - | left | the band's color | 54.5% W, centered on the rule |
| Block title (x3) | 1 to 3 words, up to 14 letters | caps | neo-grotesque (the original: a humanist sans such as Lato) | 700 | 35 | 0 | - | left | the band's color | 64.3% W, cap top level with the number's top (43 px above the rule) |
| Note (x3) | 25 to 35 words on 5 or 6 lines of up to 42 characters | sentence | neo-grotesque (Open Sans, Inter / Helvetica Neue) | 400 | 21 (the original: 20) | 0 | 135% (28 px) | left | `#7a7a7a` | 64.3% W, first line just under the rule's height |
| Closing remark | 12 to 15 words on 3 lines | sentence | neo-grotesque | 400 | 21 | 0 | 135% | left | `#7a7a7a` | 64.3% W, one blank line under note 3, ending at 92% H |

Two families: a condensed bold for the three numbers, a neutral sans for everything else.
Sizes 120 : 96 : 35 : 21, about 5.7 : 4.6 : 1.7 : 1. The numbers are taller than the titles
and notes they introduce by more than three times; nothing sits between 35 and 96 px.

## 8. Charts and diagrams

* **C-rocket**: `scripts/object.js`, `kind=rocket`, `aspect=1.4` (the stout body: 22% W wide
  at 78% H tall), `bands=3`, `badges=none` (the bands carry no numbers: the numbers stand
  outside), `flame=0`, `shade=0` (fully flat), box `8,6,38.6,94`, `colors=` orange, blue,
  red from the nose down, `fincolor=` the base band's red, `dark=` slate. The script logs
  each band's center and edge points: band 1 at y 282, band 2 at y 535, band 3 at y 804 on
  the 1600 x 1180 canvas, right edges at x 587, 615, 615.
* **C-callout**, straight variant, x3: a `shape line` with `--cap round`, 12 px, in the
  band's color, from 48 px outside the band's right edge (`rx` + 48) to x 843 (52.7% W). The
  third starts 165 px outside the body (48 px outside the fin) because the fin is in the
  way. No dot, no elbow.
* Simplified against the original: the original has a three-step nozzle and curved
  horn-shaped fins with a darker strip `#c94a5a` on their inner edge. The script's bands are
  not evenly tall, so the three rows sit at 24%, 45%, and 68% H instead of every 25% H. The
  white gaps and the porthole are added with native shapes: the gaps lie half a band above
  and below band 2's anchor. The script's own porthole (`window=1`) is not used here: it
  has fixed colors (a pale ring, sky-blue glass), is half the body wide, and is widened into
  an oval by `aspect=`, while this design's porthole is round, 62% of the body wide, and in
  the band's own blue.

## 9. Icons and illustration

* No icons. The only picture is the rocket.
* Porthole: three native shapes on band 2's anchor (437, 535): `shape ellipse` 220 px in
  `#f2f2f2`, `shape ellipse` 192 px in the band's blue, and a `shape path` crescent in
  `#64b8de` over the upper half of the glass.
* If you have a drawn rocket, place it as `illustration.png` at 33% W on the same axis and
  keep the rules: sample its three main colors for the rules, numbers, and titles.

## 10. Effects and finish

None: flat. No shadows, no gradients, no outlines. The only depth cues in the original are
the crescent highlight in the porthole and the darker strip inside each fin. Rules have round
ends; everything else has sharp corners.

## 11. Hierarchy and eye path

1. The rocket: the only large colored shape, 80% of the height.
2. The numbers 01, 02, 03: 120 px, in color, in a straight column at 54.5% W.
3. The wordmark: 96 px but low on the page and partly in slate, so it is read after the
   numbers.
4. Titles, then notes, one row at a time.

Numbers 120 px, wordmark 96, titles 35, notes 21: about 5.7 : 4.6 : 1.7 : 1.

## 12. Build recipe

Nothing to supply. Canvas 1600 x 1180. All copy is invented example content.

```bash
pixelox new -o rocket.pxo --size 1600x1180 --fill '#ffffff'
pixelox render js scripts/object.js -o body.png --size 1600x1180 --param box=8,6,38.6,94 --param kind=rocket --param aspect=1.4 --param bands=3 --param badges=none --param flame=0 --param shade=0 --param colors='#fdae49,#43aad5,#e55360' --param fincolor='#e55360' --param dark='#466077'
pixelox comp place rocket.pxo body.png --in-place --as Rocket --at 0,0
pixelox shape rect rocket.pxo --in-place --name Gap1 --rect 259,394,356x14 --fill '#ffffff'
pixelox shape rect rocket.pxo --in-place --name Gap2 --rect 259,663,356x14 --fill '#ffffff'
pixelox shape ellipse rocket.pxo --in-place --name Port --rect 327,425,220x220 --fill '#f2f2f2'
pixelox shape ellipse rocket.pxo --in-place --name Glass --rect 341,439,192x192 --fill '#43aad5'
pixelox shape path rocket.pxo --in-place --name Glint --d 'M 353 525 A 84 84 0 0 1 521 525 Q 437 560 353 525 Z' --fill '#64b8de'
pixelox text add rocket.pxo --in-place --name Word1 --content "SHIP" --font @neo-grotesque --weight 700 --size 96 --color '#466077' --at 496,1024 --anchor tr --anchor-to ink
pixelox text add rocket.pxo --in-place --name Word2 --content "IT" --font @neo-grotesque --weight 700 --size 96 --color '#e55360' --at 518,1024 --anchor tl --anchor-to ink
pixelox shape line rocket.pxo --in-place --name Rule1 --from 635,282 --to 843,282 --stroke '#fdae49' --stroke-width 12 --cap round
pixelox text add rocket.pxo --in-place --name Num1 --content "01" --font @condensed --weight 700 --size 120 --color '#fdae49' --at 872,282 --anchor l --anchor-to ink
pixelox text add rocket.pxo --in-place --name Head1 --content "FIND THE GAP" --font @neo-grotesque --weight 700 --size 35 --color '#fdae49' --at 1029,239 --anchor tl --anchor-to ink
pixelox text add rocket.pxo --in-place --name Note1 --content $'Talk to twenty people who live with the\nproblem before you build anything. Ask\nwhat they use today, what it costs them,\nand what they would pay to make it go\naway. Write down their exact words and\nread them back to the team.' --font @neo-grotesque --size 21 --leading 135% --color '#7a7a7a' --at 1029,284 --anchor tl
pixelox shape line rocket.pxo --in-place --name Rule2 --from 663,535 --to 843,535 --stroke '#43aad5' --stroke-width 12 --cap round
pixelox text add rocket.pxo --in-place --name Num2 --content "02" --font @condensed --weight 700 --size 120 --color '#43aad5' --at 872,535 --anchor l --anchor-to ink
pixelox text add rocket.pxo --in-place --name Head2 --content "BUILD THE CORE" --font @neo-grotesque --weight 700 --size 35 --color '#43aad5' --at 1029,492 --anchor tl --anchor-to ink
pixelox text add rocket.pxo --in-place --name Note2 --content $'Ship the one feature that solves it and\nnothing else. A plain version in the hands\nof five real users teaches more than a\npolished one that nobody has tried. Fix\nwhat they trip over first.' --font @neo-grotesque --size 21 --leading 135% --color '#7a7a7a' --at 1029,537 --anchor tl
pixelox shape line rocket.pxo --in-place --name Rule3 --from 780,804 --to 843,804 --stroke '#e55360' --stroke-width 12 --cap round
pixelox text add rocket.pxo --in-place --name Num3 --content "03" --font @condensed --weight 700 --size 120 --color '#e55360' --at 872,804 --anchor l --anchor-to ink
pixelox text add rocket.pxo --in-place --name Head3 --content "LIGHT THE ENGINE" --font @neo-grotesque --weight 700 --size 35 --color '#e55360' --at 1029,761 --anchor tl --anchor-to ink
pixelox text add rocket.pxo --in-place --name Note3 --content $'Pick one channel and one message, then\nspend a month on only that. Count the\nsign-ups every week, keep what works,\nand drop the rest. Growth comes from\nrepeating the one thing that already\nworked.' --font @neo-grotesque --size 21 --leading 135% --color '#7a7a7a' --at 1029,806 --anchor tl
pixelox text add rocket.pxo --in-place --name Extra --content $'After launch, choose one number to watch\nand review it with the whole team every\nMonday for the first ninety days.' --font @neo-grotesque --size 21 --leading 135% --color '#7a7a7a' --at 1029,1004 --anchor tl
pixelox export rocket.pxo -o rocket.png
```

Reuse: the three row heights are the `ry` values of the `band1`, `band2`, `band3` anchors
(run the `render js` line with `--json` and read `result.logs`). For each row: the rule runs
from `rx` + 48 to x 843 at `ry`; the number sits at x 872, centered on `ry`; the title's cap
top is `ry` - 43; the note starts at `ry` + 2. The porthole is centered on band 2's anchor,
and the two white gaps sit 134 px above and below it (half the distance between the band 2
and band 3 anchors). `aspect=` sets how stout the rocket is (1 = slim); it is fitted by
height, so keep the box at least 0.4 x aspect x its height wide (in px) and read the `rx`
values again after changing it. With 4 bands shorten the notes to 4 lines; with 5, to 3 lines and 100 px
numbers.

## 13. Variations and pairings

* **More stages:** `bands=4` or `bands=5` with notes of 3 or 4 lines; above five bands the
  rows collide: use C-process-spine.
* **Mirrored:** rocket on the right at 72.7% W, rules running left to 47.3% W, numbers and
  text right-aligned on 35.7% W. Good as the second slide of a pair.
* **Other objects, same rows:** `kind=bulb` (three bands for "idea, test, proof") or
  `kind=pencil`; the rules, numbers, and blocks do not change.
* **Numbers that are data:** replace 01 to 03 with three values ("12K", "48%", "3.2x") in
  the same 120 px condensed bold, and use the title line as the unit: the sheet becomes three
  key figures tied to three parts of one object.
* **Relatives:** i038 is the same idea on a square canvas with four candy-colored bands,
  numbers inside the bands, and icon callouts alternating left and right on elbow leaders;
  this one keeps all text in one right-hand column and puts the numbers outside. i039 runs
  five colored bars from a glossy white rocket to five text rows; i041 turns the bands into
  the rocket's exhaust column; i040 and i046 fan the exhaust into numbered columns.
* Pairs with C-timeline under the wordmark for dates, on a taller canvas (1600 x 1400).

## 14. Pitfalls

* **Text on the bands.** This design's bands are empty on purpose: a number or a title inside
  the blue band fights the porthole. Keep the object clean and let the color link do the work.
* **Rules of three different lengths chosen by eye.** They must all end on one vertical line
  (52.7% W) and start the same distance from the rocket's edge; otherwise the numbers do not
  align and the column falls apart.
* **Colors that do not match exactly.** A rule in a slightly different orange than its band
  breaks the only link between text and object. Use the same hex for band, rule, number, and
  title.
* **Band sizes read as amounts.** The nose is the tallest band only because a rocket is
  drawn that way. If your three stages have sizes (weeks, budget, head count), never suggest
  them with band heights: print the figures in the notes or add C-bars. This is the
  data-integrity rule for every metaphor object.
* **Long notes.** Six lines of 42 characters is the limit at this row height; a seventh line
  runs into the next title. Cut the copy, or drop to two stages per slide.
* **A fourth color for the wordmark.** The wordmark reuses slate and the base band's red;
  a new hue there starts a fourth category that does not exist.
