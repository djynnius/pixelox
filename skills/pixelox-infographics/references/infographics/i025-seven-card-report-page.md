---
id: i025
title: Long report page of seven outlined chart cards
kind: [dashboard, statistical]
topic: [business, finance]
mood: [clean, corporate, friendly]
style: [flat, ui-cards]
format: long
structure: S-dashboard-grid
data: [part-to-whole, comparison, change-over-time]
charts: [C-card, C-line, C-donut, C-stacked-columns, C-label-bars, C-progress-bars, C-pie, C-progress-ring, C-legend, C-stat-block, C-source-line]
palette: ['#f6f7fe', '#ffffff', '#0d1e48', '#3f2f96', '#7a74f2', '#feb605', '#fc8006']
needs: nothing
---

# i025: Long report page of seven outlined chart cards

## 1. Concept

A tall report page: a two-line title, then seven white cards on a pale lavender ground, two
per row for three rows and one full-width card to finish. Every card has a centered bold
title and exactly one small chart with its own key: a line, a donut, stacked columns, ranked
bars, progress pills, a pie, a row of rings. The message is "the whole market on one page".
It works because the cards make seven unrelated measures equal and separate, while one
palette of two purples and three warm colors, used in the same order everywhere, makes them
one document. The original was shown as one of three report pages side by side; this is the
flat design of the left page (the other two are in section 13).

## 2. Data story

Seven small data sets, none with more than eight values:

1. **Change over time:** one measure at 3 yearly points, rising about 20% a step, on an axis
   from 0 with 5 to 7 gridlines; the same three values repeated as big numbers under the chart.
2. **Part-to-whole, 8 parts:** shares from about 5% to 25% that add to 100, in two families
   (four in purples, four in warm colors).
3. **Totals made of 2 parts, 3 categories:** stacked columns with both parts labeled; under
   each column one growth rate (15 to 45%).
4. **Ranking:** 5 categories, sorted, falling in even steps from about 30 to 10 on an axis
   from 0, the names written inside the bars.
5. **Four shares** that add to 100, falling from about two fifths to about a seventh, as
   pills on tracks, the value in the pill's color.
6. **Part-to-whole, 5 parts:** from about 35% down to 10%.
7. **Five independent rates** from about 35% down to under 5%, one ring each.

Placeholder data in the recipe (example data, invented, about home energy):

| Card | Values |
| --- | --- |
| Installed capacity | 60, 75, 90 GW in 2019, 2020, 2021 |
| Where the money goes | 23, 17, 21, 11, 7, 9, 8, 4 (%) |
| Cost of a typical project | equipment 28, 14, 22; installation 34, 21, 27 (thousand dollars); growth 18%, 41%, 24% |
| What slows adoption | 32, 26, 21, 14, 9 (% of households) |
| Why households switch | 42, 27, 18, 13 (%) |
| How projects are paid for | 34, 26, 18, 13, 9 (%) |
| Homes with each technology | 36, 24, 19, 12, 4 (%) |

## 3. Information architecture

1. **Title**, centered, between two small plus-shaped registration marks.
2. **Row 1:** the headline trend (line, then three big numbers) beside the market's make-up
   (donut and an 8-entry key).
3. **Row 2:** amounts by category (stacked columns with a growth figure under each) beside
   the ranked obstacles (bars).
4. **Row 3:** preferences (progress pills) beside a second split (pie and key).
5. **Row 4**, full width: five rates as rings with the number under each.
6. **Source line** (added in the recipe; the original page has none).

There is no path between cards: each is read by itself, left to right and top to bottom. Row
1 is slightly taller than the others, which is the only ranking the page makes.

## 4. Format and grid

* Long portrait, 1 : 2.48. Recipe canvas 1080 x 2680. Side margins 6.7% W (72 px).
* Two columns of 42.6% W (460 px) with a 1.5% W (16 px) gutter; the same 16 px between rows.
* Title: cap tops at 5.2% and 8.3% H; plus marks at 12% and 88% W, 3.6% H.
* Card boxes: row 1 `6.7%,12.7%,42.6% x 23.3%` and `50.7%,12.7%,42.6% x 23.3%`; row 2
  at 36.6% H, 21.3% H tall; row 3 at 58.5% H, 21.3% H tall; row 4 `6.7%,80.4%,86.7% x 15.5%`.
* Inside a card: title centered 42 px under the top edge; chart from 90 px under the top
  edge; keys and numbers in the bottom third; inner padding 28 to 40 px.
* Bottom 4% H empty except the source line at 98% H.

## 5. Layer stack

1. Ground `#f6f7fe`, flat.
2. Two plus marks, 46 px, 3 px navy lines.
3. Title in navy `#0d1e48`, two centered lines.
4. Seven cards: white, 16 px corner radius, a 2 px outline in gray-lilac `#a9abc9`, no shadow.
5. Seven card titles in near-black `#111111`.
6. Card 1: gridlines `#d5dbe1`, axis numbers, an orange `#f08c1a` line with ringed markers;
   a small bold label; three orange numbers with their years under them.
7. Card 2: a donut (hole half the radius) of 8 slices clockwise from 12 o'clock: `#1d1470`,
   `#3f2f96`, `#7a74f2`, `#a3a0f6`, `#feb605`, `#fc8006`, `#f5280a`, `#9a1414`, white bold
   percentages inside; a key in two columns of four (cool colors left, warm right).
8. Card 3: a two-entry key; three wide stacked columns (periwinkle `#7a74f2` under yellow
   `#feb605`) with a value in every segment; under each column a small orange up-arrow, a
   bold orange percentage, a caption.
9. Card 4: a one-entry key (the unit); five orange `#fc8006` bars, sorted, names in white
   bold inside the bars at their left end; a value axis 0 to the maximum underneath.
10. Card 5: four rows, each a label, a pill track `#ecedfe` 51 px high with a filled pill
    from the left (indigo, periwinkle, yellow, orange), and the value at the right in the
    pill's color.
11. Card 6: a pie of 5 slices (indigo, periwinkle, yellow, orange, red) with white
    percentages; a key under it.
12. Card 7: five rings (track `#ecedfe`, a round-ended arc from 12 o'clock in indigo,
    periwinkle, yellow, orange, red), the percentage under each ring in its color, a two-line
    name under that.
13. Source line, muted.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#f6f7fe` | ground; pill and ring tracks (`#ecedfe`) | 30% | nothing; "the rest" in tracks |
| `#ffffff` | cards | 52% | nothing |
| `#0d1e48` | title; card titles and labels (`#111111`) | 4% | nothing |
| `#3f2f96` | series 1 (indigo); darkest donut slice `#1d1470` | 3% | first category of a set |
| `#7a74f2` | series 2 (periwinkle); lilac `#a3a0f6` as its tint | 4% | second category |
| `#feb605` | series 3 (yellow) | 3% | third category |
| `#fc8006` | series 4 (orange); single-series charts (`#f08c1a` line and numbers); red `#f5280a` and dark red `#9a1414` as series 5 and beyond | 4% | fourth category; the default for a chart with one series |

Categorical, in a fixed order: indigo, periwinkle, yellow, orange, red. A chart with one
series uses orange; a chart with two uses periwinkle and yellow; the 8-slice donut extends
the order with one darker and one lighter purple and a dark red. No color is tied to a
subject across cards, because the cards do not share categories: the order is the system.

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Title | 5 to 6 words on 2 lines | title | geometric sans (Poppins, Lexend / Avenir Next) | 700 | 67 | 0 | line pitch 83 px | center | `#0d1e48` | 50%, cap tops 5.2% and 8.3% |
| Card title (x7) | 3 to 5 words | title | geometric sans | 700 | 23 | 0 | - | center | `#111111` | card axis, 42 px under the card top |
| Axis and category labels | numbers, years, 1-word names | - | geometric sans | 400 | 18 to 20 | 0 | - | - | `#111111` (axis numbers `#444444`) | in the charts |
| Slice numbers | NN% | - | geometric sans | 700 | 19 to 22 | 0 | - | center | `#ffffff` | in the slices |
| Key entries | 1 to 3 words | title | geometric sans | 400 | 20 | 0 | 35 px | left | `#111111` | under or over each chart |
| Small bold label | 2 to 4 words with the unit | sentence | geometric sans | 700 | 19 | 0 | - | left | `#111111` | card 1, 30% H |
| Stat numbers (card 1, x3) | number and unit, 4 to 6 characters | caps | geometric sans | 700 | 31 (the original 37, with a currency sign) | 0 | - | left | `#f08c1a` | 10.6%, 23.8%, 36.9% W; 32.1% H |
| Stat caption (x3) | a year | - | geometric sans | 400 | 18 | 0 | - | left | `#111111` | under each number, 33.7% H |
| Growth figure (card 3, x3) | NN% | - | geometric sans | 700 | 37 | 0 | - | center | `#f08c1a` | column axes, 54.6% H |
| Growth caption (x3) | 2 words | sentence | geometric sans | 400 | 18 | 0 | - | center | `#111111` | 56% H |
| Bar names (card 4, x5) | 1 to 3 words | sentence | geometric sans | 700 (600 in the recipe) | 16 (18 in the recipe) | 0 | one per bar | left | `#ffffff` | inside each bar, 11 px from its left end (drawn by the chart script) |
| Pill label (card 5, x4) | 1 to 3 words | sentence | geometric sans | 400 | 18 | 0 | one per row, pitch 112 px | left | `#111111` | above each track, at its left end (drawn by the chart script) |
| Pill value (x4) | NN% | - | geometric sans | 700 | 37 | 0 | - | left | the pill's color | 16 px right of the track |
| Ring value (x5) | NN% | - | geometric sans | 700 | 48 | 0 | - | center | the ring's color | ring axes, 91% H |
| Ring name (x5) | 2 words on 2 lines | title | geometric sans | 400 | 18 | 0 | 26 px | center | `#111111` | from 92.7% H |
| Source line | 1 sentence | sentence | geometric sans | 400 | 16 | 0 | - | center | `#6b6f8f` | 50%, 98% |

One family, two weights. Sizes 67 : 48 : 37 : 23 : 18, about 3.7 : 2.7 : 2 : 1.3 : 1. Every
number that is a result (not an axis tick) is bold and in its series color.

## 8. Charts and diagrams

All charts share `font='Avenir Next, Poppins, Montserrat, Century Gothic, Arial, sans-serif'`.

* **C-line** (card 1): `scripts/line.js`, box `9.3,16,37.5,12.1`, 3 values, `max=100
  ticks=5`, `vlabels=none`, `width=9`, one orange series, gridlines on, axis from 0. The
  values are printed under the chart as three C-stat-block numbers instead of on the line.
* **C-donut** (card 2): `scripts/donut.js` by center `cx=72 cy=22.1 r=15.2`, `inner=0.5`,
  8 values, `labelpos=in show=pct sep=none`, `fs=19`. Key: two **C-legend** columns
  (`dir=column shape=circle fs=20`), four entries each.
* **C-stacked-columns** (card 3): `scripts/bars.js`, box `9.5,39.6,39,12.8`, `mode=stack`, 2
  series x 3 categories, `axis=0 gridlines=1`, `gap=0.3`, `prefix='$' suffix=K`, its own key
  on top. The original stacks two years on top of each other (see Pitfalls); the recipe
  stacks two parts of one cost.
* **C-label-bars**, names inside (card 4): `scripts/bars.js`, box `54.4,41.9,35.2,14.5`,
  `dir=h labelpos=inside`, 5 sorted values with their names in `labels=`, `axis=1 max=35
  ticks=7`, `vlabels=0`, `gap=0.14` (thick bars). The script writes each name in white
  inside its bar and moves a name that is longer than its bar behind the bar's end, so keep
  the shortest bar's name to one short word. A one-entry C-legend above says the unit.
* **C-progress-bars** with the names above (card 5, the `labelpos=above` form of
  C-label-bars): `scripts/bars.js`, box `11.3,62.4,27.7,16.72`, `dir=h track=1 max=100
  labelpos=above`, 4 values with their names in `labels=`, `gap=0.32` (a 26 px name line over
  a 50 px pill in each 112 px row), `trackcolor='#ecedfe'`, `vlabels=0`, `muted='#111111'`
  for the names. The big colored values to the right of the tracks are four `text add`
  layers at the logged bar anchors' y.
* **C-pie** (card 6): `scripts/donut.js`, `cx=72 cy=66.6 r=12`, `inner=0`, 5 values,
  `labelpos=in show=pct`, `fs=22`; a C-legend row under it (it wraps to two centered lines).
* **C-progress-ring** x5 (card 7): `kind=ring`, `cy=86.9 r=6.5`, `cx=17, 33.5, 50, 66.5, 83`,
  `inner=0.58`, round ends, `center=' '` (a space: the number is written under the ring, not
  in it).

Simplified: the original's key in card 6 is a two-column list with two-line names; its
stacked columns have rounded tops; its line markers are filled dots; the two shortest bars
in card 4 carry two-line names (the script sets one line).

## 9. Icons and illustration

* `plus`, 46 px, navy, 3 px line, twice: registration marks left and right above the title.
* `arrow-up`, 26 px, orange, above each growth figure in card 3.
* No pictures. Round keys (`shape=circle`) everywhere, which is the page's only "icon style".

## 10. Effects and finish

None: flat. Cards are separated from the ground by a 2 px outline and a 4% lightness step,
not by shadows. Corner radius 16 px on cards; fully round ends on pills and ring arcs; square
bars and columns.

## 11. Hierarchy and eye path

1. The title.
2. The orange line and the three orange numbers of card 1 (top left, the only large type in
   row 1), then the multicolored donut beside it.
3. Card by card in rows: in each, the bold title, the colored marks, then the bold colored
   numbers.
4. The row of five rings and their 48 px numbers closes the page: the largest numbers, at
   the bottom, in the full color order.

Title 67 px, ring numbers 48, stat numbers 31 to 37, card titles 23, labels 18 to 20.

## 12. Build recipe

Nothing to supply. Canvas 1080 x 2680. All figures are example data, invented.

```bash
pixelox new -o report.pxo --size 1080x2680 --fill '#f6f7fe'
pixelox shape icon report.pxo plus --in-place --name PlusL --size 46 --at 130,96 --anchor c --color '#0d1e48' --stroke-width 3
pixelox shape icon report.pxo plus --in-place --name PlusR --size 46 --at 950,96 --anchor c --color '#0d1e48' --stroke-width 3
pixelox text add report.pxo --in-place --name T1 --content "Home Energy Review:" --font @geometric-sans --weight 700 --size 67 --color '#0d1e48' --at 50%,139 --anchor t --anchor-to ink
pixelox text add report.pxo --in-place --name T2 --content "Costs and Adoption" --font @geometric-sans --weight 700 --size 67 --color '#0d1e48' --at 50%,222 --anchor t --anchor-to ink
pixelox shape rect report.pxo --in-place --name Card1 --rect 72,340,460x624 --radius 16 --fill '#ffffff' --stroke '#a9abc9' --stroke-width 2
pixelox shape rect report.pxo --in-place --name Card2 --rect 548,340,460x624 --radius 16 --fill '#ffffff' --stroke '#a9abc9' --stroke-width 2
pixelox shape rect report.pxo --in-place --name Card3 --rect 72,980,460x572 --radius 16 --fill '#ffffff' --stroke '#a9abc9' --stroke-width 2
pixelox shape rect report.pxo --in-place --name Card4 --rect 548,980,460x572 --radius 16 --fill '#ffffff' --stroke '#a9abc9' --stroke-width 2
pixelox shape rect report.pxo --in-place --name Card5 --rect 72,1568,460x572 --radius 16 --fill '#ffffff' --stroke '#a9abc9' --stroke-width 2
pixelox shape rect report.pxo --in-place --name Card6 --rect 548,1568,460x572 --radius 16 --fill '#ffffff' --stroke '#a9abc9' --stroke-width 2
pixelox shape rect report.pxo --in-place --name Card7 --rect 72,2156,936x416 --radius 16 --fill '#ffffff' --stroke '#a9abc9' --stroke-width 2
pixelox text add report.pxo --in-place --name H1 --content "Installed Capacity" --font @geometric-sans --weight 700 --size 23 --color '#111111' --at 302,382 --anchor c
pixelox text add report.pxo --in-place --name H2 --content "Where the Money Goes" --font @geometric-sans --weight 700 --size 23 --color '#111111' --at 778,382 --anchor c
pixelox text add report.pxo --in-place --name H3 --content "Cost of a Typical Project" --font @geometric-sans --weight 700 --size 23 --color '#111111' --at 302,1022 --anchor c
pixelox text add report.pxo --in-place --name H4 --content "What Slows Adoption" --font @geometric-sans --weight 700 --size 23 --color '#111111' --at 778,1022 --anchor c
pixelox text add report.pxo --in-place --name H5 --content "Why Households Switch" --font @geometric-sans --weight 700 --size 23 --color '#111111' --at 302,1610 --anchor c
pixelox text add report.pxo --in-place --name H6 --content "How Projects Are Paid For" --font @geometric-sans --weight 700 --size 23 --color '#111111' --at 778,1610 --anchor c
pixelox text add report.pxo --in-place --name H7 --content "Homes With Each Technology" --font @geometric-sans --weight 700 --size 23 --color '#111111' --at 540,2198 --anchor c
pixelox render js scripts/line.js -o growth.png --size 1080x2680 --param box=9.3,16,37.5,12.1 --param values=60,75,90 --param labels=2019,2020,2021 --param max=100 --param ticks=5 --param colors='#f08c1a' --param vlabels=none --param width=9 --param muted='#444444' --param fs=18 --param font='Avenir Next, Poppins, Montserrat, Century Gothic, Arial, sans-serif'
pixelox comp place report.pxo growth.png --in-place --as Growth --at 0,0
pixelox text add report.pxo --in-place --name M1L --content "Capacity (gigawatts)" --font @geometric-sans --weight 700 --size 19 --color '#111111' --at 115,805 --anchor l
pixelox text add report.pxo --in-place --name M1a --content "60 GW" --font @geometric-sans --weight 700 --size 31 --color '#f08c1a' --at 115,860 --anchor l
pixelox text add report.pxo --in-place --name M1b --content "75 GW" --font @geometric-sans --weight 700 --size 31 --color '#f08c1a' --at 257,860 --anchor l
pixelox text add report.pxo --in-place --name M1c --content "90 GW" --font @geometric-sans --weight 700 --size 31 --color '#f08c1a' --at 398,860 --anchor l
pixelox text add report.pxo --in-place --name M1ya --content "2019" --font @geometric-sans --size 18 --color '#111111' --at 115,902 --anchor l
pixelox text add report.pxo --in-place --name M1yb --content "2020" --font @geometric-sans --size 18 --color '#111111' --at 257,902 --anchor l
pixelox text add report.pxo --in-place --name M1yc --content "2021" --font @geometric-sans --size 18 --color '#111111' --at 398,902 --anchor l
pixelox render js scripts/donut.js -o segments.png --size 1080x2680 --param cx=72 --param cy=22.1 --param r=15.2 --param inner=0.5 --param values=23,17,21,11,7,9,8,4 --param colors='#1d1470,#3f2f96,#7a74f2,#a3a0f6,#feb605,#fc8006,#f5280a,#9a1414' --param labelpos=in --param show=pct --param sep=none --param fs=19 --param font='Avenir Next, Poppins, Montserrat, Century Gothic, Arial, sans-serif'
pixelox comp place report.pxo segments.png --in-place --as Segments --at 0,0
pixelox render js scripts/legend.js -o key2a.png --size 1080x2680 --param box=54,28.9,21,6 --param dir=column --param shape=circle --param labels='Heat pumps|Rooftop solar|Home batteries|Smart meters' --param colors='#1d1470,#3f2f96,#7a74f2,#a3a0f6' --param muted='#111111' --param fs=20 --param font='Avenir Next, Poppins, Montserrat, Century Gothic, Arial, sans-serif'
pixelox comp place report.pxo key2a.png --in-place --as Key2a --at 0,0
pixelox render js scripts/legend.js -o key2b.png --size 1080x2680 --param box=76.5,28.9,17,6 --param dir=column --param shape=circle --param labels='Insulation|EV chargers|Solar thermal|Other' --param colors='#feb605,#fc8006,#f5280a,#9a1414' --param muted='#111111' --param fs=20 --param font='Avenir Next, Poppins, Montserrat, Century Gothic, Arial, sans-serif'
pixelox comp place report.pxo key2b.png --in-place --as Key2b --at 0,0
pixelox render js scripts/bars.js -o cost.png --size 1080x2680 --param box=9.5,39.6,39,12.8 --param mode=stack --param values='28,14,22;34,21,27' --param labels='Heating|Solar|Storage' --param series='Equipment,Installation' --param colors='#7a74f2,#feb605' --param prefix='$' --param suffix=K --param axis=0 --param gridlines=1 --param gap=0.3 --param ink='#111111' --param muted='#111111' --param fs=20 --param font='Avenir Next, Poppins, Montserrat, Century Gothic, Arial, sans-serif'
pixelox comp place report.pxo cost.png --in-place --as Cost --at 0,0
pixelox shape icon report.pxo arrow-up --in-place --name Up1 --size 26 --at 173,1421 --anchor c --color '#f08c1a'
pixelox shape icon report.pxo arrow-up --in-place --name Up2 --size 26 --at 313,1421 --anchor c --color '#f08c1a'
pixelox shape icon report.pxo arrow-up --in-place --name Up3 --size 26 --at 454,1421 --anchor c --color '#f08c1a'
pixelox text add report.pxo --in-place --name G1 --content "18%" --font @geometric-sans --weight 700 --size 37 --color '#f08c1a' --at 173,1462 --anchor c
pixelox text add report.pxo --in-place --name G2 --content "41%" --font @geometric-sans --weight 700 --size 37 --color '#f08c1a' --at 313,1462 --anchor c
pixelox text add report.pxo --in-place --name G3 --content "24%" --font @geometric-sans --weight 700 --size 37 --color '#f08c1a' --at 454,1462 --anchor c
pixelox text add report.pxo --in-place --name G1c --content "since 2019" --font @geometric-sans --size 18 --color '#111111' --at 173,1502 --anchor c
pixelox text add report.pxo --in-place --name G2c --content "since 2019" --font @geometric-sans --size 18 --color '#111111' --at 313,1502 --anchor c
pixelox text add report.pxo --in-place --name G3c --content "since 2019" --font @geometric-sans --size 18 --color '#111111' --at 454,1502 --anchor c
pixelox render js scripts/legend.js -o key4.png --size 1080x2680 --param box=54.4,39.9,36,1.4 --param dir=column --param shape=circle --param labels='Share of households (%)' --param colors='#fc8006' --param muted='#111111' --param fs=20 --param font='Avenir Next, Poppins, Montserrat, Century Gothic, Arial, sans-serif'
pixelox comp place report.pxo key4.png --in-place --as Key4 --at 0,0
pixelox render js scripts/bars.js -o barriers.png --size 1080x2680 --param box=54.4,41.9,35.2,14.5 --param dir=h --param labelpos=inside --param values=32,26,21,14,9 --param labels='Upfront cost|Finding an installer|Permits|Roof space|Renting' --param colors='#fc8006' --param axis=1 --param max=35 --param ticks=7 --param vlabels=0 --param gap=0.14 --param muted='#111111' --param fs=18 --param font='Avenir Next, Poppins, Montserrat, Century Gothic, Arial, sans-serif'
pixelox comp place report.pxo barriers.png --in-place --as Barriers --at 0,0
pixelox render js scripts/bars.js -o reasons.png --size 1080x2680 --param box=11.3,62.4,27.7,16.72 --param dir=h --param track=1 --param max=100 --param labelpos=above --param values=42,27,18,13 --param labels='Lower bills|Comfort all year|Lower emissions|Resale value' --param colors='#3f2f96,#7a74f2,#feb605,#fc8006' --param trackcolor='#ecedfe' --param vlabels=0 --param gap=0.32 --param muted='#111111' --param fs=18 --param font='Avenir Next, Poppins, Montserrat, Century Gothic, Arial, sans-serif'
pixelox comp place report.pxo reasons.png --in-place --as Reasons --at 0,0
pixelox text add report.pxo --in-place --name P1 --content "42%" --font @geometric-sans --weight 700 --size 37 --color '#3f2f96' --at 437,1741 --anchor l
pixelox text add report.pxo --in-place --name P2 --content "27%" --font @geometric-sans --weight 700 --size 37 --color '#7a74f2' --at 437,1853 --anchor l
pixelox text add report.pxo --in-place --name P3 --content "18%" --font @geometric-sans --weight 700 --size 37 --color '#e9a400' --at 437,1965 --anchor l
pixelox text add report.pxo --in-place --name P4 --content "13%" --font @geometric-sans --weight 700 --size 37 --color '#fc8006' --at 437,2077 --anchor l
pixelox render js scripts/donut.js -o paid.png --size 1080x2680 --param cx=72 --param cy=66.6 --param r=12 --param inner=0 --param values=34,26,18,13,9 --param colors='#3f2f96,#7a74f2,#feb605,#fc8006,#f5280a' --param labelpos=in --param show=pct --param sep=none --param fs=22 --param font='Avenir Next, Poppins, Montserrat, Century Gothic, Arial, sans-serif'
pixelox comp place report.pxo paid.png --in-place --as Paid --at 0,0
pixelox render js scripts/legend.js -o key6.png --size 1080x2680 --param box=52,72.4,41,6.4 --param shape=circle --param labels='Savings|Bank loan|Public grant|Lease|Other' --param colors='#3f2f96,#7a74f2,#feb605,#fc8006,#f5280a' --param muted='#111111' --param fs=20 --param font='Avenir Next, Poppins, Montserrat, Century Gothic, Arial, sans-serif'
pixelox comp place report.pxo key6.png --in-place --as Key6 --at 0,0
pixelox render js scripts/donut.js -o ring1.png --size 1080x2680 --param kind=ring --param cx=17 --param cy=86.9 --param r=6.5 --param inner=0.58 --param value=36 --param colors='#3f2f96' --param trackcolor='#ecedfe' --param center=' '
pixelox render js scripts/donut.js -o ring2.png --size 1080x2680 --param kind=ring --param cx=33.5 --param cy=86.9 --param r=6.5 --param inner=0.58 --param value=24 --param colors='#7a74f2' --param trackcolor='#ecedfe' --param center=' '
pixelox render js scripts/donut.js -o ring3.png --size 1080x2680 --param kind=ring --param cx=50 --param cy=86.9 --param r=6.5 --param inner=0.58 --param value=19 --param colors='#feb605' --param trackcolor='#ecedfe' --param center=' '
pixelox render js scripts/donut.js -o ring4.png --size 1080x2680 --param kind=ring --param cx=66.5 --param cy=86.9 --param r=6.5 --param inner=0.58 --param value=12 --param colors='#fc8006' --param trackcolor='#ecedfe' --param center=' '
pixelox render js scripts/donut.js -o ring5.png --size 1080x2680 --param kind=ring --param cx=83 --param cy=86.9 --param r=6.5 --param inner=0.58 --param value=4 --param colors='#f5280a' --param trackcolor='#ecedfe' --param center=' '
pixelox comp place report.pxo ring1.png --in-place --as Ring1 --at 0,0
pixelox comp place report.pxo ring2.png --in-place --as Ring2 --at 0,0
pixelox comp place report.pxo ring3.png --in-place --as Ring3 --at 0,0
pixelox comp place report.pxo ring4.png --in-place --as Ring4 --at 0,0
pixelox comp place report.pxo ring5.png --in-place --as Ring5 --at 0,0
pixelox text add report.pxo --in-place --name R1 --content "36%" --font @geometric-sans --weight 700 --size 48 --color '#3f2f96' --at 17%,2439 --anchor c
pixelox text add report.pxo --in-place --name R2 --content "24%" --font @geometric-sans --weight 700 --size 48 --color '#7a74f2' --at 33.5%,2439 --anchor c
pixelox text add report.pxo --in-place --name R3 --content "19%" --font @geometric-sans --weight 700 --size 48 --color '#e9a400' --at 50%,2439 --anchor c
pixelox text add report.pxo --in-place --name R4 --content "12%" --font @geometric-sans --weight 700 --size 48 --color '#fc8006' --at 66.5%,2439 --anchor c
pixelox text add report.pxo --in-place --name R5 --content "4%" --font @geometric-sans --weight 700 --size 48 --color '#f5280a' --at 83%,2439 --anchor c
pixelox text add report.pxo --in-place --name R1c --content $'Smart\nmeter' --font @geometric-sans --size 18 --leading 26 --align center --color '#111111' --at 17%,2484 --anchor t
pixelox text add report.pxo --in-place --name R2c --content $'Heat\npump' --font @geometric-sans --size 18 --leading 26 --align center --color '#111111' --at 33.5%,2484 --anchor t
pixelox text add report.pxo --in-place --name R3c --content $'Rooftop\nsolar' --font @geometric-sans --size 18 --leading 26 --align center --color '#111111' --at 50%,2484 --anchor t
pixelox text add report.pxo --in-place --name R4c --content $'EV\ncharger' --font @geometric-sans --size 18 --leading 26 --align center --color '#111111' --at 66.5%,2484 --anchor t
pixelox text add report.pxo --in-place --name R5c --content $'Home\nbattery' --font @geometric-sans --size 18 --leading 26 --align center --color '#111111' --at 83%,2484 --anchor t
pixelox text add report.pxo --in-place --name Source --content "Example data, invented for this layout. Replace with your figures and their source." --font @geometric-sans --size 16 --color '#6b6f8f' --at 50%,2626 --anchor c
pixelox export report.pxo -o report.png
```

Reuse: one card = one `shape rect`, one title, one chart whose `box` sits inside the card
with 28 to 40 px of padding. Change `values=` first, then the titles. Anchors used: the
column centers in card 3 (173, 313, 454: `bar1.1`, `bar2.1`, `bar3.1`) and the track rows in
card 5 (the y of `bar1` to `bar4`: 1741, 1853, 1965, 2077, where the four big values sit);
read them again with `lab/anchors.py` or `--json` whenever a box or the number of bars
changes. The names in cards 4 and 5 are `labels=` of their charts: change them there. To drop a row, delete its cards and move the rows below up by
the row height plus 16 px.

## 13. Variations and pairings

* **Shorter page:** rows 1 and 2 plus the ring row make an A-series page (1080 x 1528);
  a single row of two cards under the title makes a 16:9 slide at 1600 px wide.
* **Middle page of the original set (cream, blue and orange):** ground `#fefdf4`, cards with
  a 2 px dark outline and a pill-shaped title chip (blue `#5282c2` or orange `#f28241`, white
  bold 16 px) that straddles the card's top edge; a one-line title in a light weight at about
  62 px with a small caps deck and rules above and below. Its cards: four C-gauge
  half rings with the number inside and a month under each; three milestone tracks
  (C-progress-bars) with a status word; C-grouped-columns target against actual;
  four rows of an avatar disc with a short progress pill and a percentage; C-stacked-columns
  budget against actual; a C-pie with a two-column key; C-bars for four months with one big
  rating figure beside them; and a three-task schedule of "projected" and "actual" chips on
  thin tracks under six dates.
* **Right page of the original set (grid paper, navy and vermilion):** ground `#f1f2f5` with a
  faint square grid (`scripts/pattern.js`, `kind=grid`), no cards: modules are opened by a
  short 2 px rule and a letterspaced label ("DATA 1", tracking 300), with a three-line note.
  Two colors only, navy `#304257` and vermilion `#e4573a`. Modules: C-stacked-columns of 5
  items; a vertical dotted C-process-spine of 5 icon discs alternating the two colors; a
  C-org-tree of one lead, three managers, three staff as chips with a round photo; a 4 x 3
  briefcase icon array with more than half in the accent color; a C-progress-ring; a vermilion
  world silhouette with a few navy markers (C-world-map: world countries as one shape with
  `stroke=none`, `projection=natural`, nothing encoded but the marker positions in
  `points=`; C-dot-map on builds before 0.6.0) beside three rows of ten C-icon-array
  figures, one row per year. A dark pill footer holds the web address.
* **One hue family:** replace the five series colors with four tints of one hue plus one
  accent (palette "Violet report" or "Forest report") for a calmer board.
* Pairs with C-sparkline in a card (number left, trend right) and with C-table for a card of
  exact figures.

## 14. Pitfalls

* **Stacking things that are not parts of a whole.** The original stacks one year's spending
  on top of another's, so the column height is a sum nobody needs and the upper segment
  cannot be compared. Stack parts (equipment and installation); put two years side by side
  with C-grouped-columns.
* **Eight slices in a donut.** It only reads here because the slices are labeled, and grouped
  into two color families. Past five slices, fold the small ones into "Other" or use C-bars.
* **Progress pills for shares of one total.** Four pills that add to 100 hide that they are
  one whole; a C-percent-bar says it. Use pills for independent rates and say the base.
* **A different color order in each card.** If indigo is first in one key and third in the
  next, the page looks random. Keep one order (`colors=` copied from chart to chart).
* **Cards of unequal padding.** Titles at different distances from the top edge are the
  first thing that looks unprofessional. Fix the title offset (42 px) and chart top (90 px).
* **No unit, period, or source.** Each card needs its unit in the title or the key ("% of
  households", "thousand dollars"), and the page a source line; the original has neither a
  date range nor a source.
