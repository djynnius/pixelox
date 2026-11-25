---
id: i002
title: Chart card with stacked columns and a share line
kind: [statistical]
topic: [politics, society, generic]
mood: [clean, editorial, calm, cool]
style: [flat, editorial]
format: square
structure: S-chart-card
data: [change-over-time, part-to-whole]
charts: [C-combo, C-title-block, C-legend, C-callout, C-source-line]
palette: ['#f4f9fc', '#13897c', '#80ded7', '#eac62d', '#1b2a3a', '#52606d', '#ffffff']
needs: nothing
---

# i002: Chart card with stacked columns and a share line

## 1. Concept

A square newsroom chart card: one finding, one chart, nothing else. Eight columns show a total
per period with its lower part tinted lighter (the part the story is about), and a yellow line
on a second axis shows that part as a share of the total. The message is "the part is taking
over the whole". It works because the same fact is drawn twice, once as an amount (the light
part of each column) and once as a rate (the line), and the one loud color on the page, the
yellow of the accent bar beside the title, is also the color of the line that carries the
finding.

## 2. Data story

* **Change over time, 8 periods** on the x-axis (every second year in the original; any
  regular step works).
* **Two amounts per period, one inside the other:** a total that swings between about half
  of the axis maximum and all of it, and a part that starts near an eighth of the maximum.
  The totals zig-zag, because two kinds of period alternate (a large one, then a small one);
  the part grows slowly, then jumps in the seventh period to nearly two thirds of the total.
* **One rate per period:** the part as a share of the total, between a fifth and a quarter
  for four periods, about a third for two, then more than 60%. It is drawn as a line on its
  own axis (0% to 80%) at the right.
* **The last period is not finished:** only the part is known. It stands as a short light
  column with its value in a small tag, has no dark part and no line point, and is explained
  by a footnote mark on its x label.
* Left axis from 0 in 4 round steps with a unit letter; right axis 0% to 80% in 4 steps: both
  axes share the same 5 gridlines.

Placeholder data used in the recipe (example data, invented: tickets for a film festival held
every second year, alternating a full and a short edition, in thousands):

| Year | 2011 | 2013 | 2015 | 2017 | 2019 | 2021 | 2023 | 2025 (so far) |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Booked online | 39 | 26 | 50 | 33 | 68 | 60 | 132 | 45 |
| Bought at the door | 141 | 94 | 130 | 87 | 127 | 105 | 78 | not yet known |
| Online share | 22% | 22% | 28% | 28% | 35% | 36% | 63% | - |

## 3. Information architecture

1. **Title block** (top left): a vertical accent bar, the finding in bold, a two-line deck that
   says what is counted and where.
2. **Legend band** (a white strip across the card): the two column keys at the left, stacked;
   the line key at the right, right-aligned, with a short yellow dash. Left keys belong to the
   left axis, the right key to the right axis: the legend's layout explains the two scales.
3. **Chart:** columns and line; the eye follows the yellow line up to its last point, then
   drops to the tagged short column at the far right.
4. **Footnotes and source** (two lines, gray): what the two marks mean, where the data is from.
5. **Signature row** (bottom): small publisher marks; a credit in the recipe.

## 4. Format and grid

* Square, recipe canvas 1080 x 1080. Margins 3.1% W on both sides: everything hangs from the
  left margin; only the line key is right-aligned to the right axis labels.
* Accent bar `3.1%,6.1%,2.1% x 14%`. Title at 6.9% W, cap top 6.3% H. Deck lines with cap tops
  at 13.7% and 17.4% H.
* Legend band `3.1%,22.2%,94.1% x 9.3%`.
* Chart box (axis labels included) `3.5%,33%,93% x 46.5%`; the plot itself is 11.6% to 89.4% W,
  34.4% to 74.8% H; x labels at 77.5% H.
* Footnotes at 84% H, source at 87.5% H, signature at 95% H.
* Vertical rhythm: title block 16% of the height, legend 8%, chart 46%, notes 8%, signature
  8%, the rest is air.

## 5. Layer stack

1. Ground: very pale cool gray `#f4f9fc`, flat.
2. Accent bar: yellow `#eac62d`, 22 x 150 px, square corners.
3. Title in navy `#1b2a3a`; deck in slate `#52606d`.
4. Legend band: a white rectangle, no outline, no shadow. In it: a 20 px square in dark teal
   `#13897c` with its name, a 20 px square in light teal `#80ded7` with its name, and at the
   right two right-aligned lines of text followed by a 24 x 6 px yellow dash.
5. Gridlines: 5 hairlines `#c9d6dc` across the plot, the bottom one (the baseline) darker.
6. Columns: 8, each 58% of its band wide, square corners. Light teal from the baseline up to
   the part's value, dark teal from there up to the total. The last column is light teal only.
7. Line: yellow, 5 px, round joins, no markers, through the middle of columns 1 to 7.
8. Value tag on the last column: a light teal label 92 x 38 px with a small pointer down to the
   column's top, the value in navy bold.
9. Axis labels: left "0K" to "240K", right "0%" to "80%", years under the columns, all in navy.
10. Footnote line and source line in slate; signature at the bottom right.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#f4f9fc` | ground | 78% | nothing |
| `#13897c` | upper part of each column | 9% | the rest of the total (series 2) |
| `#80ded7` | lower part of each column, value tag | 5% | the part the story is about (series 1) |
| `#eac62d` | accent bar, line, line key | 1% | the share (the finding) |
| `#1b2a3a` | title, axis labels, legend names | 2% | nothing |
| `#52606d` | deck, footnotes, source | 2% | nothing |
| `#ffffff` | legend band | 3% | nothing: groups the keys |

Two tints of one teal for the two amounts (they are parts of one thing), and one contrasting
yellow for the rate. Yellow is the single accent: it appears only on the accent bar and the
line.

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Title | the finding, 3 to 5 words, one line | title | neo-grotesque (Open Sans, Inter / Helvetica Neue) | 700 | 66 | 0 | - | left | `#1b2a3a` | 6.9%, cap top 6.3% |
| Deck | what is counted and where, 10 to 14 words on 2 lines | sentence | neo-grotesque | 400 | 34 | 0 | 140% (48 px pitch) | left | `#52606d` | 6.9%, cap top 13.7% |
| Column keys (x2) | 2 to 5 words each | sentence | neo-grotesque | 400 | 30 | 0 | 36 px pitch | left | `#1b2a3a` | 11.9%, 24.3% and 29.3% |
| Line key | 5 to 7 words on 2 lines | sentence | neo-grotesque | 400 | 30 | 0 | 36 px pitch | right | `#1b2a3a` | right edge 88%, 25% and 28.7% |
| Left axis | 5 numbers with a unit letter | - | neo-grotesque | 400 | 29 | 0 | - | left | `#1b2a3a` | 3.5% W, on the gridlines |
| Right axis | 5 percentages | - | neo-grotesque | 400 | 29 | 0 | - | right | `#1b2a3a` | 96.5% W, on the gridlines |
| X labels | 8 years; the last with a footnote mark | - | neo-grotesque | 400 | 32 | 0 | - | center | `#1b2a3a` | under the columns, 77.5% |
| Value tag | 2 to 4 characters | - | neo-grotesque | 700 | 24 | 0 | - | center | `#1b2a3a` | above the last column |
| Footnotes | two marks explained, 8 to 14 words, one line | sentence | neo-grotesque | 400 | 27 | 0 | - | left | `#52606d` | 3.1%, 84% |
| Source | "Source:" and one line | sentence | neo-grotesque | 400 | 27 | 0 | - | left | `#52606d` | 3.1%, 87.5% |
| Signature | a mark or a 2-word credit | - | neo-grotesque | 700 | 22 | 0 | - | right | `#1b2a3a` | 96.9%, 95% |

One family, two weights. Sizes 66 : 34 : 30 : 27, about 2.2 : 1.1 : 1 : 0.9. The labels are
large for a chart (`fs=32`, 3% of the width; the script sets axis numbers at 0.9 x `fs`): the card is made to be read on a phone.

## 8. Charts and diagrams

* **C-combo**, `scripts/bars.js` with `mode=stack`: two series per period (`values=` holds the
  part first, then the rest of the total, so the part sits on the baseline), `colors=` light
  teal then dark teal, `gap=0.42`, `radius=0`, `vlabels=0` (the axis carries the values),
  `max=240 ticks=4 suffix=K`, and the rate as `line=` with `linemax=80 linesuffix=%`. Left and
  right axes share the gridlines because both have 4 steps. `legend=none`: the legend is its
  own band above the chart.
* The original draws the part **in front of** a full-height total column (two overlapping
  columns, the legend says "total" and "part"). A stack of "part" and "rest" looks the same
  and keeps the legend honest; name the keys by what each color covers.
* The unfinished last period: a value in the first series, 0 in the second, and one value
  fewer in `line=` (the line stops at column 7).
* **C-callout** as a value tag: a `shape path` label whose pointer ends 4 px above the last
  column's top (the `bar8.1` anchor from the script's log), with the value in bold.
* **C-legend**, `scripts/legend.js` with `dir=column` for the two column keys; the line key is
  two `text add` lines and one short `shape line`.
* No value labels on the columns and no markers on the line: with two axes, more ink would
  hide the one shape that matters.

## 9. Icons and illustration

None. The only marks besides the chart are the accent bar and the publisher marks in the
bottom row (the original has three small license badges at the left and two word marks at the
right; write your own credit or place `logo.png` there, about 18% W wide).

## 10. Effects and finish

None: flat. No shadows, no gradients, no rounded corners anywhere (`radius=0` on the columns).
The white legend band on the pale ground is the only layering.

## 11. Hierarchy and eye path

1. The title (the largest and darkest type), pulled out by the yellow bar beside it.
2. The yellow line, because it repeats the bar's color: up to its last, highest point.
3. The columns under the line, then the short tagged column at the right.
4. The legend band (read only when the colors need explaining), the axes, the notes.

Title 66 px, deck 34, labels 30, notes 27.

## 12. Build recipe

Nothing to supply. Canvas 1080 x 1080. All figures are example data.

```bash
pixelox new -o card.pxo --size 1080x1080 --fill '#f4f9fc'
pixelox shape rect card.pxo --in-place --name Accent --rect 34,66,22x150 --fill '#eac62d'
pixelox text add card.pxo --in-place --name Title --content "Online Booking Takes Over" --font @neo-grotesque --weight 700 --size 66 --color '#1b2a3a' --at 74,68 --anchor tl --anchor-to ink
pixelox text add card.pxo --in-place --name Deck --content $'Tickets sold by a film festival, in thousands,\nand the share that was booked online' --font @neo-grotesque --size 34 --leading 140% --color '#52606d' --at 74,148 --anchor tl --anchor-to ink
pixelox shape rect card.pxo --in-place --name LegendBand --rect 34,240,1016x100 --fill '#ffffff'
pixelox render js scripts/legend.js -o keys.png --size 1080x1080 --param box=8.5,22.3,50,8 --param labels='Bought at the door|Booked online' --param colors='#13897c,#80ded7' --param dir=column --param muted='#1b2a3a' --param fs=30
pixelox comp place card.pxo keys.png --in-place --as Keys --at 0,0
pixelox text add card.pxo --in-place --name LineKey --content $'Share of tickets\nbooked online' --font @neo-grotesque --size 30 --leading 40 --align right --color '#1b2a3a' --at 950,290 --anchor r
pixelox shape line card.pxo --in-place --name LineDash --from 968,270 --to 992,270 --stroke '#eac62d' --stroke-width 6
pixelox render js scripts/bars.js -o combo.png --size 1080x1080 --param box=3.5,33,93,46.5 --param mode=stack --param values='39,26,50,33,68,60,132,45;141,94,130,87,127,105,78,0' --param labels='2011,2013,2015,2017,2019,2021,2023,2025**' --param colors='#80ded7,#13897c' --param line='22,22,28,28,35,36,63' --param linecolor='#eac62d' --param linesuffix=% --param linemax=80 --param max=240 --param ticks=4 --param suffix=K --param vlabels=0 --param gap=0.42 --param radius=0 --param legend=none --param ink='#1b2a3a' --param muted='#1b2a3a' --param grid='#c9d6dc' --param fs=32
pixelox comp place card.pxo combo.png --in-place --as Combo --at 0,0
pixelox shape path card.pxo --in-place --name Tag --d 'M 867 671 L 959 671 L 959 709 L 925 709 L 913 722 L 901 709 L 867 709 Z' --fill '#80ded7'
pixelox text add card.pxo --in-place --name TagValue --content "45K" --font @neo-grotesque --weight 700 --size 24 --color '#1b2a3a' --at 913,690 --anchor c --anchor-to ink
pixelox text add card.pxo --in-place --name Notes --content "* Full and short editions alternate.    ** Bookings up to 1 June 2025" --font @neo-grotesque --size 27 --color '#52606d' --at 34,907 --anchor l
pixelox text add card.pxo --in-place --name Source --content "Source: Example data, invented for this layout" --font @neo-grotesque --size 27 --color '#52606d' --at 34,945 --anchor l
pixelox text add card.pxo --in-place --name Credit --content "YOUR MARK" --font @neo-grotesque --weight 700 --size 22 --tracking 60 --color '#1b2a3a' --at 1046,1026 --anchor r
pixelox export card.pxo -o card.png
```

Reuse: change `values=` (first series = the part on the baseline, second = the rest), `line=`
(the part divided by the total, in percent), `labels=`, `max=`, and `linemax=`; keep
`ticks=4` on both so the two axes share gridlines. The tag sits on the `bar8.1` anchor
(913, 726 here): read it again with `--json` after any change to `box=` or the data, and move
the tag's path and its text by the same amounts. The title is the finding, the deck the
measure and the unit.

## 13. Variations and pairings

* **One series and a rate:** drop the second series (`mode=group`, one color): plain columns
  with a line, the textbook C-combo (revenue and margin, visitors and conversion).
* **No rate:** remove `line=` and the right key: a C-stacked-columns card. With 3 parts use
  three tints of the hue, darkest at the bottom.
* **Dark card:** ground `#16233b`, band `#1f3050`, `ink` white, `grid` `#31466b`, the same
  teals and yellow.
* **4:5 for a feed:** canvas 1080 x 1350, chart box `3.5,28,93,54`, everything else the same
  distance from the top and bottom edges.
* Pairs with C-stat-block (the last share as a hero number above the chart) or C-callout on
  the line's jump with a 6-word reason.

## 14. Pitfalls

* **Two axes that read as one.** Readers compare the line's height with the columns'. Give
  the line a color from another family, key it on the side of its own axis, and state both
  units (here: thousands in the deck, % on the right axis).
* **Scales chosen so the line crosses the columns dramatically.** `max=` and `linemax=` are
  free choices, and where the line crosses the columns means nothing. Start both at zero (the
  script does) and pick round maxima just above the data; do not tune them for a crossing.
* **Legend that says "total" for the dark color.** In a stack the dark part is only the rest;
  the total is the whole column. Name each key by what its color covers, or the part is
  counted twice.
* **An unfinished period drawn like the others.** A partial last column looks like a collapse.
  Mark it (a footnote sign on the label, a tag with the value) and leave it out of the line.
* **Value labels on every segment and point.** Sixteen numbers plus two axes bury the trend.
  Label only the exception, as the tag does.
* **A topic as the title** ("Tickets 2011 to 2025") wastes the largest type; write the finding.
