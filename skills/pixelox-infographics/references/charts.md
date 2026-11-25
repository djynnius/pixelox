# Chart and diagram library

Named, tested recipes for every chart, diagram, and piece of infographic furniture this skill
uses. Infographic descriptions cite them by ID (`C-donut`, `C-hemicycle`). Every block below
was run end to end with pixelox 0.6.0 and its output was looked at (the recipes that say
`render d3` need 0.6.0 or later; everything else also runs on 0.5.1). Find a recipe by
searching this file for its heading (`## C-donut`); do not read the whole file.

## How charts are built

**Never compute chart geometry by hand.** Bar heights, arc angles, seat positions, and label
coordinates come from the generator scripts in `scripts/`, run with `render js`. You give the
data as `--param` values; the script draws the chart with its labels and writes a transparent
PNG. Then you place it and add the title, notes, and source with `text add`.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#f6f3ec'
pixelox render js scripts/bars.js -o bars.png --size 1080x1350 --param box=7,24,86,56 --param values=42,31,18,9 --param labels='North,South,East,West' --param suffix=%
pixelox comp place d.pxo bars.png --in-place --as Bars --at 0,0
pixelox text add d.pxo --in-place --name Title --content "North leads by eleven points" --font @neo-grotesque --weight 700 --size 60 --color '#1f2933' --at 7%,7% --anchor tl --anchor-to ink
pixelox text add d.pxo --in-place --name Source --content "Source: example survey, 2,000 adults" --font @neo-grotesque --size 18 --color '#7b8794' --at 7%,95% --anchor bl
pixelox export d.pxo -o chart.png
```

The four rules that make this reliable:

1. **Render every chart at the canvas size** (`--size` = the document's size) and say where it
   goes with `box=x,y,w,h` in **percent of the canvas** (`box=7,24,86,56` = left 7%, top 24%,
   86% wide, 56% high). Everything the chart draws, labels and legend included, stays inside
   the box. Place the PNG at `--at 0,0`: it lands exactly where the box says. No offsets to add.
2. **Read the anchors.** Run the `render js` line with `--json`: `result.logs[0]` is one JSON
   line with `anchors`, each an `id`, a `label`, and `x`, `y` in canvas pixels (`xp`, `yp` in
   percent). Use them to put an icon on a node (`shape icon ... --at X,Y --anchor c`), a callout
   on a bar, or your own `text add` label on a slice.
3. **One label size per canvas.** Every script sets its labels at `fs` px, by default 2% of the
   canvas width (22 px at 1080), so all charts on one canvas match without any setting. Give
   the same `fs=` to every chart if you change it. Minimum 18 px at 1080 wide.
4. **To change a chart, render it again.** Place charts with `--link`
   (`comp place d.pxo bars.png --in-place --as Bars --at 0,0 --link`). Then edit the `--param`
   values, write the PNG again to the same name (`--overwrite` is fine for your own chart
   file), and run `pixelox comp update-links d.pxo --in-place`: the layer refreshes in place.
   Anchors move with the data, the `box`, and `fs`: read them again before you re-place icons
   and callouts that hang on the chart.

### Parameters every script takes

| Param | Default | Meaning |
| --- | --- | --- |
| `box` | `0,0,100,100` | where the chart goes: x, y, width, height in % of the image |
| `colors` | a built-in set of 8 | comma-separated hexes, one per series, slice, group, or step |
| `ink`, `muted`, `grid` | `#1f2933`, `#6b7785`, `#d5dbe1` | text color, secondary text color, gridline color. On a dark ground give `ink='#ffffff'`, a pale `muted`, and a `grid` a little lighter than the ground |
| `fs` | 2% of the width | label size in px |
| `font` | Helvetica Neue, Helvetica, Arial, Liberation Sans, DejaVu Sans, sans-serif | a family name or a comma-separated fallback list. **Font classes (`@condensed`) do not work inside a script**: ask what a class resolves to with `pixelox fonts classes @geometric-sans --json` (`result.classes[0].resolves_to.family`) and pass that family name |
| `numfont`, `weight`, `numweight` | the label font, 400, 700 | font of the numbers; label weight; number weight |
| `prefix`, `suffix`, `decimals`, `compact` | none | number format: `prefix=$`, `suffix=%`, `decimals=1`, `compact=1` writes 1.2K and 3.4M |
| `bg` | `transparent` | a color fills the whole image (only for looking at a chart by itself) |

Lists use commas (`values=42,31,18`); several series are separated by semicolons inside quotes
(`values='12,18,25;9,14,20'`). Names that contain commas, and all titles and notes, use `|`
between items (`labels='Plan it|Build it, fast|Ship it'`). Quote every value that holds `#`,
`|`, `;`, or a space. `\n` inside a note forces a line break.

### Which chart for which data

| The data is | Say it with | Not with |
| --- | --- | --- |
| one number that matters | C-stat-block; as a share: C-progress-ring, C-icon-array, C-picture-gauge | a chart with one bar |
| parts of one whole, 2 to 5 parts | C-donut, C-half-donut, C-waffle, C-percent-bar | a pie with 8 slices; 3D pies |
| amounts across categories | C-bars (ranked, long names), C-columns (few, short names, or time periods) | a donut; areas |
| two or three measures per category | C-grouped-columns, C-stacked-columns; two groups: C-butterfly-bars | two separate charts with different scales |
| change that goes up and down | C-negative-columns, C-line | stacked bars |
| change over time | C-line, C-area, C-columns (up to 8 periods), C-combo, C-timeline for events | a pie per year |
| seats, votes, people counted one by one | C-hemicycle, C-seat-grid, C-icon-array | a bar chart alone (loses the "one seat" feel) |
| values by place | C-us-choropleth, C-us-bubble-map, C-world-map (real outlines, pixelox 0.6.0 or later); C-tile-map, C-hex-map, C-dot-map, C-bubble-map (any build) | an outline drawn freehand; a globe for comparing values |
| ordered steps | C-process-row (3 to 5), C-process-spine (4 to 8), C-chevrons, C-rocket and other objects | a paragraph |
| a repeating loop | C-cycle, C-cycle-segments | a row of steps with an arrow back |
| events with dates | C-timeline | a table |
| levels or drop-off | C-pyramid (levels), C-funnel (stage counts) | a stacked bar |
| what things share | C-venn (2 to 4 sets) | a table of ticks |
| one topic with facets | C-hub-spokes, C-icon-grid | a bulleted list |
| a ranking with text | C-ranked-list, C-bars | a word cloud |

### Limits that keep charts readable

| Chart | Limit |
| --- | --- |
| donut, pie, half donut | 5 slices at most; fold the rest into "Other"; start the largest at 12 o'clock |
| columns | 8 at most, names of 10 characters or fewer; otherwise horizontal bars |
| bars | 12 at most; sort by value unless the order means something |
| grouped or stacked | 3 series (4 at the very most) |
| lines | 4 series; label the line ends instead of a legend |
| hemicycle | 2 to 8 groups; 20 to 700 seats |
| waffle, icon array | 100 units at most for a waffle; 5 to 20 for people icons |
| process row | 5 steps at 1080 px wide; more goes on a spine |
| cycle | 3 to 9 steps |
| legend | 6 entries at most; better: label directly |

## Index

| Group | IDs |
| --- | --- |
| Bars and columns | C-columns, C-bars, C-grouped-columns, C-stacked-columns, C-percent-bar, C-combo, C-progress-bars, C-label-bars, C-butterfly-bars, C-negative-columns |
| Lines and areas | C-line, C-multi-line, C-area, C-stacked-area, C-sparkline |
| Part of a whole | C-donut, C-pie, C-half-donut, C-progress-ring, C-gauge, C-tick-ring, C-polar-area, C-petal-wheel, C-ring-gauge |
| Counted units | C-hemicycle, C-seat-grid, C-waffle, C-icon-array, C-dot-rows |
| Maps | outline maps (pixelox 0.6.0 or later): C-us-choropleth, C-us-bubble-map, C-world-map, C-globe; stylized (any build): C-tile-map, C-hex-map, C-dot-map, C-bubble-map; C-map-image |
| Process and time | C-process-row, C-process-spine, C-chevrons, C-timeline, C-snake-steps, C-cycle, C-cycle-segments, C-hub-spokes |
| Radial and 3D | C-radial-bars, C-radial-list, C-3d-bars |
| Concepts | C-venn, C-pyramid, C-funnel |
| Metaphor objects | C-rocket, C-bulb, C-pencil, C-picture-gauge |
| Furniture (native commands) | C-title-block, C-section-band, C-card, C-stat-block, C-stat-row, C-icon-badge, C-icon-grid, C-callout, C-speech-bubble, C-ribbon-label, C-legend, C-ranked-list, C-table, C-comparison-columns, C-org-tree, C-arc-label, C-source-line |

All recipes use a 1080 px wide canvas. Sizes in px scale with the width: multiply by
`width / 1080` on another canvas. `scripts/` is this skill's `scripts/` folder: copy it next to
your work or write its full path.

---

# Bars and columns

Script: `scripts/bars.js`. Bars always start at zero; the script has no way to cut the axis.

## C-columns

**Use for** amounts across up to 8 categories with short names, or one measure over a few
periods. One color for all columns; `highlight` marks the one the title talks about.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#f6f3ec'
pixelox render js scripts/bars.js -o cols.png --size 1080x1080 --param box=8,20,84,64 --param values=42,31,18,9,6 --param labels='North,South,East,West,Other' --param suffix=% --param colors='#9aa5b1' --param highlight=1 --param accent='#e4572e' --param fs=24
pixelox comp place d.pxo cols.png --in-place --as Columns --at 0,0
pixelox text add d.pxo --in-place --name Title --content "North leads by eleven points" --font @neo-grotesque --weight 700 --size 54 --color '#1f2933' --at 8%,7% --anchor tl --anchor-to ink
pixelox text add d.pxo --in-place --name Source --content "Share of orders by region. Source: example data" --font @neo-grotesque --size 18 --color '#7b8794' --at 8%,94% --anchor bl
pixelox export d.pxo -o C-columns.png
```

* `highlight=N` (1-based) draws that column in `accent` and its label in bold; all others take
  the first color. Use a gray for the rest and one accent.
* `max=50 ticks=5` sets the axis yourself. `axis=0 gridlines=0` drops the axis when every
  column carries its value (the cleaner look for 6 columns or fewer).
* `radius=10` rounds the tops; `radius=999` makes pill tops.
* **Fails when** names are long: they wrap to two lines, then collide. Switch to C-bars.

## C-bars

**Use for** a ranking, long category names, or more than 7 categories. Sort the values from
large to small before you pass them.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#ffffff'
pixelox render js scripts/bars.js -o bars.png --size 1080x1080 --param box=8,20,84,66 --param dir=h --param values=78,64,51,37,22 --param labels='Regulatory change|Economic downturn|Technology shift|Supply chain|New competitors' --param colors='#f59e0b' --param fs=26
pixelox comp place d.pxo bars.png --in-place --as Bars --at 0,0
pixelox text add d.pxo --in-place --name Title --content "What worries founders most" --font @neo-grotesque --weight 700 --size 54 --color '#1f2933' --at 8%,7% --anchor tl --anchor-to ink
pixelox text add d.pxo --in-place --name Deck --content "Impact score out of 100" --font @neo-grotesque --size 28 --color '#52606d' --at 8%,14% --anchor tl --anchor-to ink
pixelox export d.pxo -o C-bars.png
```

* `vlabels=inside` writes the value inside the bar end in white (use with thick bars);
  `labelwidth=0.45` gives long names more room (a fraction of the box width).
* `axis=1` adds a value axis under the bars; with value labels on every bar you rarely need it.
* **Fails when** the bars are unsorted: the eye cannot rank them. Sort, or keep a natural
  order (age groups, months) and say so.

## C-grouped-columns

**Use for** the same measure for 2 or 3 series across categories (this year against last).

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#ffffff'
pixelox render js scripts/bars.js -o grouped.png --size 1080x1080 --param box=8,18,84,70 --param values='62,48,35,71;70,52,31,84' --param labels='Revenue,Retention,Churn,Completion' --param series='Target,Actual' --param colors='#f97316,#3b7ddd' --param suffix=% --param max=100 --param fs=24
pixelox comp place d.pxo grouped.png --in-place --as Grouped --at 0,0
pixelox text add d.pxo --in-place --name Title --content "Targets against results" --font @neo-grotesque --weight 700 --size 54 --color '#1f2933' --at 8%,7% --anchor tl --anchor-to ink
pixelox export d.pxo -o C-grouped-columns.png
```

* `series=` names turn the legend on (`legend=top|bottom|none`).
* With 3 series the value labels shrink; with 4, set `vlabels=0` and keep the axis.
* **Fails when** the two series have different units: never share one axis between dollars
  and percent. Use C-combo or two charts.

## C-stacked-columns

**Use for** totals that are made of 2 to 4 parts, across a few categories. The bottom part is
the only one that can be compared exactly, so put the part that matters at the bottom.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#ffffff'
pixelox render js scripts/bars.js -o stacked.png --size 1080x1080 --param box=8,18,84,70 --param mode=stack --param values='30,15,25,40;35,20,30,22;10,12,8,15' --param labels='Q1,Q2,Q3,Q4' --param series='Hardware,Software,Services' --param colors='#1e3a5f,#3b7ddd,#a9c9f5' --param fs=24
pixelox comp place d.pxo stacked.png --in-place --as Stacked --at 0,0
pixelox text add d.pxo --in-place --name Title --content "Revenue by line, in millions" --font @neo-grotesque --weight 700 --size 54 --color '#1f2933' --at 8%,7% --anchor tl --anchor-to ink
pixelox export d.pxo -o C-stacked-columns.png
```

* Colors: one hue in 2 to 4 steps from dark (bottom) to light (top) reads as "parts of one
  thing"; unrelated hues read as rivals.
* `mode=percent` rescales every column to 100% (shares instead of amounts).
* **Fails when** a segment is too thin for its number: the script leaves the number out. Say
  the small values in a note instead.

## C-percent-bar

**Use for** one whole split into 2 to 5 shares when a donut would take too much room: a single
horizontal strip, with the shares written inside it.

```bash
pixelox new -o d.pxo --size 1080x540 --fill '#ffffff'
pixelox render js scripts/bars.js -o strip.png --size 1080x540 --param box=8,34,84,40 --param dir=h --param mode=percent --param values='96;48;41;31;24' --param series='Staff,Rent,Stock,Marketing,Other' --param colors='#1f2933,#52606d,#7b8794,#9aa5b1,#cbd2d9' --param gap=0.1 --param legend=bottom --param fs=24
pixelox comp place d.pxo strip.png --in-place --as Strip --at 0,0
pixelox text add d.pxo --in-place --name Title --content "Where the 240,000 budget goes" --font @neo-grotesque --weight 700 --size 48 --color '#1f2933' --at 8%,11% --anchor tl --anchor-to ink
pixelox export d.pxo -o C-percent-bar.png
```

* Each share is its own series with one value: note the semicolons in `values='96;48;41;31;24'`.
* `mode=stack` instead of `percent` writes the amounts, not the percentages.
* **Fails when** there are more than 5 shares or one is under 4%: its label does not fit. Fold
  small shares into "Other".

## C-combo

**Use for** an amount (columns) and a rate (line) over the same periods, each with its own
axis: left for the columns, right for the line.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#f4f6f8'
pixelox render js scripts/bars.js -o combo.png --size 1080x1080 --param box=7,26,86,58 --param values='84,96,91,110,124,131,150;21,26,30,41,52,63,84' --param labels='2019,2020,2021,2022,2023,2024,2025' --param series='All orders,App orders' --param colors='#12897b,#7fdfd4' --param line='25,27,33,37,42,48,56' --param linename='App share' --param linesuffix=% --param linemax=80 --param max=160 --param suffix=K --param vlabels=0 --param gap=0.3 --param fs=24
pixelox comp place d.pxo combo.png --in-place --as Combo --at 0,0
pixelox shape rect d.pxo --in-place --name Bar --rect 7%,6%,1.6%x12% --fill '#f2b705'
pixelox text add d.pxo --in-place --name Title --content "The app now takes most orders" --font @neo-grotesque --weight 700 --size 52 --color '#16233b' --at 10.5%,6% --anchor tl --anchor-to ink
pixelox text add d.pxo --in-place --name Deck --content "Orders a year, and the share placed in the app" --font @neo-grotesque --size 28 --color '#52606d' --at 10.5%,13.5% --anchor tl --anchor-to ink
pixelox text add d.pxo --in-place --name Source --content "Source: example data" --font @neo-grotesque --size 18 --color '#7b8794' --at 7%,94% --anchor bl
pixelox export d.pxo -o C-combo.png
```

* The line has its own scale: `linemax=` and `linesuffix=`. State both units in the deck.
* `markers=1` puts dots on the line's points.
* **Fails when** readers compare the line's height with the columns': they share no scale.
  Keep the two in clearly different colors and name each in the legend.

## C-progress-bars

**Use for** several percentages that do not add up to one whole (survey answers, skill
levels, goals reached): a pill on a pale track for each.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#ffffff'
pixelox render js scripts/bars.js -o progress.png --size 1080x1080 --param box=8,22,84,62 --param dir=h --param track=1 --param max=100 --param values=46,31,18,12 --param labels='Free delivery|Easy returns|Loyalty points|Gift wrapping' --param colors='#3f37c9,#7b6cf6,#f2b705,#f97316' --param suffix=% --param fs=26
pixelox comp place d.pxo progress.png --in-place --as Progress --at 0,0
pixelox text add d.pxo --in-place --name Title --content "What customers ask for" --font @neo-grotesque --weight 700 --size 54 --color '#1f2933' --at 8%,7% --anchor tl --anchor-to ink
pixelox export d.pxo -o C-progress-bars.png
```

* `track=1` needs `max=` (the value a full track stands for, usually 100).
* `trackcolor='#2a2f4a'` on a dark ground (the default track is the bar color mixed to white).
* `gap=0.6` makes thinner pills; `radius=0` square ends.
* **Fails when** the values are parts of one whole: then use C-percent-bar or C-donut.

## C-label-bars

**Use for** bars whose names are long or part of the message: the name sits on its own line
above each bar, or inside it, so the bars can use the full width.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#ffffff'
pixelox render js scripts/bars.js -o above.png --size 1080x1080 --param box=8,18,84,36 --param dir=h --param labelpos=above --param track=1 --param max=100 --param values=64,48,31 --param labels='Read the label before buying|Sort waste at home every week|Repair instead of replacing' --param colors='#0d9488' --param suffix=% --param fs=26
pixelox comp place d.pxo above.png --in-place --as Above --at 0,0
pixelox render js scripts/bars.js -o inside.png --size 1080x1080 --param box=8,60,84,32 --param dir=h --param labelpos=inside --param values=78,64,51,37 --param labels='Regulation|Costs|Technology|Supply' --param colors='#1e3a5f' --param gap=0.22 --param fs=26
pixelox comp place d.pxo inside.png --in-place --as Inside --at 0,0
pixelox text add d.pxo --in-place --name Title --content "Names above, names inside" --font @neo-grotesque --weight 700 --size 54 --color '#1f2933' --at 8%,7% --anchor tl --anchor-to ink
pixelox export d.pxo -o C-label-bars.png
```

* `labelpos=above` needs about 1.5 x `fs` more height per bar; `labelpos=inside` needs thick
  bars (`gap=0.22`) and names that fit the shortest bar (a name that does not fit is written
  after the bar).
* Both drop the name column, so the bars start at the box's left edge.

## C-butterfly-bars

**Use for** two groups compared across the same categories (women and men by age, before and
after by department): bars grow left and right from a middle column of names.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#ffffff'
pixelox render js scripts/bars.js -o fly.png --size 1080x1080 --param box=7,20,86,70 --param mode=diverge --param values='42,38,30,22,12;36,40,33,25,18' --param labels='18 to 29|30 to 44|45 to 59|60 to 74|75 and over' --param series='Group A,Group B' --param colors='#e4572e,#1e3a5f' --param suffix=% --param fs=26
pixelox comp place d.pxo fly.png --in-place --as Butterfly --at 0,0
pixelox text add d.pxo --in-place --name Title --content "Younger people lean to A" --font @neo-grotesque --weight 700 --size 54 --color '#1f2933' --at 7%,7% --anchor tl --anchor-to ink
pixelox export d.pxo -o C-butterfly-bars.png
```

* Exactly two series; both sides share one scale (`max=` sets it), so lengths compare across
  the middle. `track=1` puts both sides on pale tracks.
* **Fails when** the two sides have different units or bases: then they are two charts.

## C-negative-columns

**Use for** change that goes both ways (growth and decline, profit and loss): columns above
and below a zero line.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#ffffff'
pixelox render js scripts/bars.js -o change.png --size 1080x1080 --param box=8,22,84,66 --param values=12,-5,8,-14,20,6 --param labels='Jan,Feb,Mar,Apr,May,Jun' --param suffix=% --param colors='#3b7ddd' --param fs=24
pixelox comp place d.pxo change.png --in-place --as Change --at 0,0
pixelox text add d.pxo --in-place --name Title --content "Two bad months, then a record" --font @neo-grotesque --weight 700 --size 54 --color '#1f2933' --at 8%,7% --anchor tl --anchor-to ink
pixelox export d.pxo -o C-negative-columns.png
```

* Any negative value switches the axis to run below zero; zero stays a firm line.
* To color gains and losses differently, pass `colors=` with one color per column (a blue
  for each gain, a red for each loss).

---

# Lines and areas

Script: `scripts/line.js`. A line may use an axis that does not start at zero (`min=`); say
so in the deck when it does.

## C-line

**Use for** one measure over 5 or more points in time. The last value is written at the end
of the line.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#ffffff'
pixelox render js scripts/line.js -o line.png --size 1080x1080 --param box=8,22,84,64 --param values='12,18,17,26,31,44' --param labels='2020,2021,2022,2023,2024,2025' --param suffix=K --param colors='#e4572e' --param fs=24
pixelox comp place d.pxo line.png --in-place --as Line --at 0,0
pixelox text add d.pxo --in-place --name Title --content "Members nearly quadrupled" --font @neo-grotesque --weight 700 --size 54 --color '#1f2933' --at 8%,7% --anchor tl --anchor-to ink
pixelox text add d.pxo --in-place --name Deck --content "Paying members at year end, thousands" --font @neo-grotesque --size 28 --color '#52606d' --at 8%,14% --anchor tl --anchor-to ink
pixelox export d.pxo -o C-line.png
```

* `vlabels=all` writes every point's value (8 points or fewer); `vlabels=none` writes none.
* `smooth=1` rounds the line; keep `smooth=0` for exact data.
* `width=6` for a heavier line on a poster.
* **Fails when** there are 3 points or fewer: use C-columns or three C-stat-blocks.

## C-multi-line

**Use for** 2 to 4 series over the same periods. Names sit at the line ends in the line's
color, so no legend is needed.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#101322'
pixelox render js scripts/line.js -o lines.png --size 1080x1080 --param box=8,22,84,64 --param values='32,38,35,44,52,61,58,70;20,24,31,30,36,35,44,49;8,12,11,18,17,25,24,30' --param labels='Jan,Feb,Mar,Apr,May,Jun,Jul,Aug' --param series='Online,In store,Partners' --param colors='#22d3ee,#e879f9,#facc15' --param smooth=1 --param markers=0 --param ink='#ffffff' --param muted='#9aa3c7' --param grid='#2a2f4a' --param fs=24
pixelox comp place d.pxo lines.png --in-place --as Lines --at 0,0
pixelox text add d.pxo --in-place --name Title --content "Online pulls away" --font @neo-grotesque --weight 700 --size 54 --color '#ffffff' --at 8%,7% --anchor tl --anchor-to ink
pixelox text add d.pxo --in-place --name Deck --content "Monthly sales by channel, thousands" --font @neo-grotesque --size 28 --color '#9aa3c7' --at 8%,14% --anchor tl --anchor-to ink
pixelox export d.pxo -o C-multi-line.png
```

* `endlabels=0` moves the names into a legend on top.
* End labels that would overlap are pushed apart automatically.
* **Fails when** more than 4 lines cross: gray out all but the one or two the story is about
  (`colors='#e4572e,#cbd2d9,#cbd2d9,#cbd2d9'`).

## C-area

**Use for** one series whose volume matters (a total growing over time). The fill is a soft
tint of the line color.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#ffffff'
pixelox render js scripts/line.js -o area.png --size 1080x1080 --param box=8,22,84,64 --param values='4,6,9,15,22,34,41,58' --param labels='2018,2019,2020,2021,2022,2023,2024,2025' --param area=1 --param colors='#0d9488' --param suffix=M --param fs=24
pixelox comp place d.pxo area.png --in-place --as Area --at 0,0
pixelox text add d.pxo --in-place --name Title --content "Streams keep compounding" --font @neo-grotesque --weight 700 --size 54 --color '#1f2933' --at 8%,7% --anchor tl --anchor-to ink
pixelox export d.pxo -o C-area.png
```

* An area must start at zero (keep `min` at 0): the filled height is read as the amount.
* **Fails when** two soft areas overlap and hide each other: use C-multi-line or C-stacked-area.

## C-stacked-area

**Use for** a total over time made of 2 to 4 parts.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#ffffff'
pixelox render js scripts/line.js -o stack.png --size 1080x1080 --param box=8,20,84,68 --param values='20,24,31,30,36,35,44,49;12,14,13,18,21,25,24,30;8,12,11,14,17,15,19,22' --param labels='2018,2019,2020,2021,2022,2023,2024,2025' --param series='Solar,Wind,Hydro' --param area=stack --param colors='#f59e0b,#38bdf8,#1e3a5f' --param fs=24
pixelox comp place d.pxo stack.png --in-place --as Stack --at 0,0
pixelox text add d.pxo --in-place --name Title --content "Clean power, by source" --font @neo-grotesque --weight 700 --size 54 --color '#1f2933' --at 8%,7% --anchor tl --anchor-to ink
pixelox export d.pxo -o C-stacked-area.png
```

* The first series is the bottom layer: put the largest or the steadiest there.
* **Fails when** readers try to read the upper layers' own values: they sit on a moving
  base. Add the latest values in a note or use C-multi-line.

## C-sparkline

**Use for** a tiny trend beside a number in a stat card: no axis, no labels except the last
value.

```bash
pixelox new -o d.pxo --size 1080x540 --fill '#ffffff'
pixelox render js scripts/line.js -o spark.png --size 1080x540 --param box=50,30,42,36 --param values='12,18,17,26,22,31,29,44' --param spark=1 --param area=1 --param width=8 --param colors='#16a34a' --param vlabels=none
pixelox comp place d.pxo spark.png --in-place --as Spark --at 0,0
pixelox text add d.pxo --in-place --name Number --content "44K" --font @neo-grotesque --weight 700 --size 150 --color '#1f2933' --at 8%,50% --anchor l --anchor-to ink
pixelox text add d.pxo --in-place --name Label --content "active members, up 3.7x in eight quarters" --font @neo-grotesque --size 26 --color '#52606d' --at 8%,78% --anchor l
pixelox export d.pxo -o C-sparkline.png
```

* A sparkline's vertical range is fitted to the data: it shows shape, not size. Always pair
  it with the number.

---

# Part of a whole

Script: `scripts/donut.js`. Angles are computed from the values; you never write an arc.

## C-donut

**Use for** 2 to 5 shares of one whole, with the total (or the message) in the hole.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#f6f3ec'
pixelox render js scripts/donut.js -o donut.png --size 1080x1080 --param box=8,20,84,70 --param values=45,25,20,10 --param labels='Online|Stores|Partners|Other' --param colors='#1e3a5f,#3b7ddd,#f59e0b,#cbd2d9' --param center=120K --param sub='orders' --param sep='#f6f3ec' --param fs=26
pixelox comp place d.pxo donut.png --in-place --as Donut --at 0,0
pixelox text add d.pxo --in-place --name Title --content "Nearly half of orders start online" --font @neo-grotesque --weight 700 --size 54 --color '#1f2933' --at 8%,7% --anchor tl --anchor-to ink
pixelox export d.pxo -o C-donut.png
```

* `labelpos=out` (default) writes the number and name beside each slice on a short leader;
  `labelpos=in` writes only the number inside the slice; `labelpos=legend` puts names in a
  legend (to the right in a wide box, underneath in a tall one) and numbers inside.
* `show=pct|value|both|none` picks the number: the share, the raw value, or both.
* `sep=` is the hairline between slices: set it to your ground color. `gap=2` opens real
  gaps (degrees) instead.
* `inner=0.62` is the hole; 0.75 makes a thin ring, 0.5 a heavy one.
* Order: largest first, clockwise from 12 o'clock; "Other" last, in gray.
* **Fails when** there are 6 or more slices, or two slices within 2 points of each other that
  the story needs compared: use C-bars.

## C-pie

**Use for** 2 or 3 shares where one clearly dominates. Same script, no hole.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#ffffff'
pixelox render js scripts/donut.js -o pie.png --size 1080x1080 --param box=10,20,80,72 --param values=52,24,16,8 --param labels='Search|Newsletter|Referrals|Events' --param inner=0 --param labelpos=legend --param colors='#f97316,#fdba74,#3b7ddd,#93c5fd' --param fs=26
pixelox comp place d.pxo pie.png --in-place --as Pie --at 0,0
pixelox text add d.pxo --in-place --name Title --content "Where new customers come from" --font @neo-grotesque --weight 700 --size 54 --color '#1f2933' --at 8%,7% --anchor tl --anchor-to ink
pixelox export d.pxo -o C-pie.png
```

* Slices under `minlabel=4` percent get no inside number: name them in a note.
* **Fails when** drawn in 3D or exploded: tilted slices lie about their size. Keep it flat.

## C-half-donut

**Use for** a two-sided balance (for and against, party A and party B) or seats as solid
bands: a half ring with the total under the arch. Also the arc band of results graphics.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#16233b'
pixelox render js scripts/donut.js -o half.png --size 1080x1080 --param box=8,26,84,50 --param values=212,7,181 --param labels='Party A|Vacant|Party B' --param colors='#c8241b,#9aa5b1,#1f6fb5' --param sweep=180 --param inner=0.52 --param show=value --param labelpos=in --param sep=none --param gap=1 --param center=400 --param sub='seats in the chamber' --param ink='#ffffff' --param muted='#b8c4d6' --param fs=30
pixelox comp place d.pxo half.png --in-place --as Half --at 0,0
pixelox text add d.pxo --in-place --name Title --content "A thirty-one seat margin" --font @neo-grotesque --weight 700 --size 60 --color '#ffffff' --at 50%,9% --anchor t --anchor-to ink
pixelox render js scripts/legend.js -o legend.png --size 1080x1080 --param box=8,80,84,8 --param labels='Party A  212|Vacant  7|Party B  181' --param colors='#c8241b,#9aa5b1,#1f6fb5' --param muted='#b8c4d6' --param fs=26
pixelox comp place d.pxo legend.png --in-place --as Legend --at 0,0
pixelox export d.pxo -o C-half-donut.png
```

* `sweep=180` starts at 9 o'clock and runs over the top, so the first value is the left end.
  Other sweeps: `sweep=240` (a horseshoe), `start=` sets the first angle yourself.
* Two rings (a projection outside a current count inside): render the script twice into the
  same box, the inner one with a smaller box centered on the same point, and place both.
* The legend here is drawn by `scripts/legend.js` (see C-legend), because the numbers sit
  inside the band and the names would crowd it.
* **Fails when** used for parts that are not two sides of one whole; a half ring suggests a
  balance tipping left or right.

## C-progress-ring

**Use for** one percentage as a hero: a thick arc on a pale track with the number inside.
Several in a row compare a few independent percentages.

```bash
pixelox new -o d.pxo --size 1080x640 --fill '#ffffff'
pixelox render js scripts/donut.js -o r1.png --size 1080x640 --param box=6,16,26,56 --param kind=ring --param value=58 --param colors='#1e1b6b' --param trackcolor='#cfc9f7'
pixelox render js scripts/donut.js -o r2.png --size 1080x640 --param box=37,16,26,56 --param kind=ring --param value=37 --param colors='#a13db5' --param trackcolor='#ecd0f2'
pixelox render js scripts/donut.js -o r3.png --size 1080x640 --param box=68,16,26,56 --param kind=ring --param value=81 --param colors='#1e1b6b' --param trackcolor='#cfc9f7'
pixelox comp place d.pxo r1.png --in-place --as Ring1 --at 0,0
pixelox comp place d.pxo r2.png --in-place --as Ring2 --at 0,0
pixelox comp place d.pxo r3.png --in-place --as Ring3 --at 0,0
pixelox text add d.pxo --in-place --name L1 --content $'of members\nrenew' --font @neo-grotesque --size 24 --align center --color '#1f2933' --at 19%,80% --anchor t
pixelox text add d.pxo --in-place --name L2 --content $'bring a friend\nwithin a year' --font @neo-grotesque --size 24 --align center --color '#1f2933' --at 50%,80% --anchor t
pixelox text add d.pxo --in-place --name L3 --content $'visit\nevery week' --font @neo-grotesque --size 24 --align center --color '#1f2933' --at 81%,80% --anchor t
pixelox export d.pxo -o C-progress-ring.png
```

* `inner=0.78` is the default ring weight; `inner=0.6` is heavy, `inner=0.88` hairline.
* `center=` overrides the number in the middle (`center='4 of 5'`); `sub=` adds a small line.
* `round=0` gives square arc ends.
* One ring per number: each ring is its own whole, so the rings need not add up to 100.

## C-gauge

**Use for** a level between empty and full (score, capacity, progress to a goal): the same
ring opened to a half circle.

```bash
pixelox new -o d.pxo --size 1080x640 --fill '#fffaf0'
pixelox render js scripts/donut.js -o g1.png --size 1080x640 --param box=8,22,38,44 --param kind=ring --param sweep=180 --param value=64 --param colors='#3b7ddd' --param sub='Week 1' --param fs=24
pixelox render js scripts/donut.js -o g2.png --size 1080x640 --param box=54,22,38,44 --param kind=ring --param sweep=180 --param value=82 --param colors='#f97316' --param sub='Week 2' --param fs=24
pixelox comp place d.pxo g1.png --in-place --as Gauge1 --at 0,0
pixelox comp place d.pxo g2.png --in-place --as Gauge2 --at 0,0
pixelox text add d.pxo --in-place --name Title --content "Progress to the weekly goal" --font @neo-grotesque --weight 700 --size 40 --color '#1f2933' --at 50%,8% --anchor t --anchor-to ink
pixelox export d.pxo -o C-gauge.png
```

* `max=` is what a full gauge stands for (default 100); `suffix=` and `prefix=` format the number.
* `sweep=240` gives a deeper dial.

## C-tick-ring

**Use for** a split shown as a ring of fine ticks (a poll, a vote share): lighter than a donut
and it leaves the middle free for a question.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#16233b'
pixelox render js scripts/donut.js -o ticks.png --size 1080x1080 --param box=6,16,88,70 --param values=41,8,51 --param labels='Option A|Undecided|Option B' --param colors='#e2493f,#9aa5b1,#3d8fd1' --param ticks=100 --param inner=0.72 --param sub='Which option do voters prefer?' --param ink='#ffffff' --param muted='#b8c4d6' --param fs=28
pixelox comp place d.pxo ticks.png --in-place --as Ticks --at 0,0
pixelox text add d.pxo --in-place --name Title --content "A ten-point lead" --font @neo-grotesque --weight 700 --size 56 --color '#ffffff' --at 50%,6% --anchor t --anchor-to ink
pixelox export d.pxo -o C-tick-ring.png
```

* `ticks=100` gives one tick per percent, so the count of ticks is the share.
* Start a two-sided split at the top so the sides meet at 12 o'clock.

## C-polar-area

**Use for** a few parts that should also differ in reach: slices whose **angle** is the value
and whose length steps down (a "rose"), or a fan of equal slices of different lengths.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#ffffff'
pixelox render js scripts/donut.js -o rose.png --size 1080x1080 --param box=8,18,84,76 --param values=30,25,20,15,10 --param labels='Housing|Food|Travel|Leisure|Other' --param radii=1,0.88,0.76,0.64,0.52 --param inner=0.22 --param gappx=6 --param colors='#1e3a5f,#2563eb,#60a5fa,#f59e0b,#cbd2d9' --param fs=26
pixelox comp place d.pxo rose.png --in-place --as Rose --at 0,0
pixelox text add d.pxo --in-place --name Title --content "Where a salary goes" --font @neo-grotesque --weight 700 --size 54 --color '#1f2933' --at 8%,7% --anchor tl --anchor-to ink
pixelox export d.pxo -o C-polar-area.png
```

* `radii=` gives each slice's length as a fraction of the ring's depth (1 reaches the full
  radius, 0.5 stops halfway from the hole's edge). The values are
  still read from the angles; the lengths only rank the slices. Do not make length carry a
  second measure unless you print both numbers.
* Half fan over a baseline: `sweep=180` with equal `values=1,1,1,1` and falling `radii=`.
* **Fails when** readers compare areas: a long thin slice looks bigger than a short wide one.
  Print the numbers (the default labels do).

## C-petal-wheel

**Use for** 3 to 8 options or facets as rounded panels around a hub: each panel holds an
icon and a title, the hub holds the topic.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#eef1f8'
pixelox render js scripts/donut.js -o petals.png --size 1080x1080 --param cx=50 --param cy=54 --param r=40 --param values=1,1,1,1,1 --param colors='#ffffff' --param inner=0.4 --param gappx=22 --param corner=16 --param labelpos=none --param sep=none
pixelox comp place d.pxo petals.png --in-place --as Petals --at 0,0
pixelox fx drop-shadow d.pxo --in-place --layer Petals --offset 0,10 --blur 28 --color '#1e2a55' --opacity 14
pixelox shape ellipse d.pxo --in-place --name Hub --rect 400,443,280x280 --fill '#ffffff'
pixelox text add d.pxo --in-place --name HubN --content "5" --font @geometric-sans --weight 700 --size 110 --color '#1e3a5f' --at 540,560 --anchor c --anchor-to ink
pixelox text add d.pxo --in-place --name HubT --content "OPTIONS" --font @geometric-sans --weight 600 --size 22 --tracking 80 --color '#6b7785' --at 540,640 --anchor c
pixelox shape icon d.pxo search --in-place --name I1 --size 64 --at 718,339 --anchor c --color '#0d9488' --stroke-width 5
pixelox shape icon d.pxo lightbulb --in-place --name I2 --size 64 --at 828,677 --anchor c --color '#16a34a' --stroke-width 5
pixelox shape icon d.pxo settings --in-place --name I3 --size 64 --at 540,886 --anchor c --color '#f59e0b' --stroke-width 5
pixelox shape icon d.pxo clock --in-place --name I4 --size 64 --at 252,677 --anchor c --color '#f97316' --stroke-width 5
pixelox shape icon d.pxo target --in-place --name I5 --size 64 --at 362,339 --anchor c --color '#e11d48' --stroke-width 5
pixelox text add d.pxo --in-place --name Title --content "Five ways to start" --font @neo-grotesque --weight 700 --size 50 --color '#1f2933' --at 50%,5% --anchor t --anchor-to ink
pixelox export d.pxo -o C-petal-wheel.png
```

* `gappx=` is a gap of constant width between panels; `corner=` rounds every corner.
  Equal `values=` make equal panels.
* The icons sit on the `slice1` ... anchors (the middle of each panel); put each panel's
  title 50 px under its icon, in the icon's color.
* Fewer degrees: `sweep=235 start=-117` leaves the bottom open for a caption.

## C-ring-gauge

**Use for** 2 to 5 percentages of the same kind compared on one center (campaign reach,
clicks, leads, sales): concentric arcs, each on its own track.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#1c1b3a'
pixelox render js scripts/donut.js -o rings.png --size 1080x1080 --param box=12,20,76,72 --param kind=rings --param values=82,64,45,30 --param labels='Reach|Clicks|Leads|Sales' --param colors='#22d3ee,#e879f9,#facc15,#fb7185' --param trackcolor='#2f2e58' --param inner=0.3 --param ink='#ffffff' --param fs=28
pixelox comp place d.pxo rings.png --in-place --as Rings --at 0,0
pixelox text add d.pxo --in-place --name Title --content "From reach to sales" --font @neo-grotesque --weight 700 --size 54 --color '#ffffff' --at 50%,7% --anchor t --anchor-to ink
pixelox export d.pxo -o C-ring-gauge.png
```

* The arcs start at 12 o'clock and may cover up to 300 degrees (`sweep=`); `max=` is what a
  full arc stands for. Outer rings are longer for the same value: read the numbers, not the
  arc lengths, across rings.

---

# Counted units

One mark per seat, person, or percent. The scripts draw exactly as many marks as the numbers
say: check the logged `total`.

## C-hemicycle

Script: `scripts/hemicycle.js`. **Use for** seats in a chamber, by party or bloc.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#ffffff'
pixelox render js scripts/hemicycle.js -o seats.png --size 1080x1080 --param box=7,22,86,64 --param seats=48,5,47 --param labels='Party A|Independent|Party B' --param colors='#1f4ea3,#9aa5b1,#c8241b' --param center=100 --param sub='seats' --param fs=26
pixelox comp place d.pxo seats.png --in-place --as Seats --at 0,0
pixelox text add d.pxo --in-place --name Title --content "The chamber, seat by seat" --font @neo-grotesque --weight 700 --size 54 --color '#1f2933' --at 7%,7% --anchor tl --anchor-to ink
pixelox text add d.pxo --in-place --name Note --content "51 seats make a majority" --font @neo-grotesque --size 26 --color '#52606d' --at 7%,14% --anchor tl --anchor-to ink
pixelox export d.pxo -o C-hemicycle.png
```

* `seats=` lists the groups from the left end of the arc to the right end. Order them as they
  sit, with small or neutral groups in the middle.
* `rows=0` picks the number of rows from the seat count; set `rows=5` to match a real chamber.
* `flip=1` opens the arc upward (a bowl): stack a second chamber under the first.
* `shape=hex|square`, `hollow=2` draws group 2 as outlines (vacant, undecided, up for election).
* `sweep=200` to `240` wraps the arc into a horseshoe; `majority=1` adds the halfway line.
* `inner=0.38` sets the empty middle; make it larger (0.5) to hold a picture or a big total.
* Group anchors (`group1`, ...) give the middle of each wedge for a label or a party mark.
* **Fails when** the colors carry meaning you did not check: use the convention of the
  country (in the US, blue and red for the two large parties), gray for vacant or other.

## C-seat-grid

Script: `scripts/waffle.js`. **Use for** a large body as a block of seats (hundreds of units),
filled column by column so each group is a solid block.

```bash
pixelox new -o d.pxo --size 1080x720 --fill '#ffffff'
pixelox render js scripts/waffle.js -o grid.png --size 1080x720 --param box=6,22,88,62 --param values=205,4,191 --param cols=25 --param order=cols --param shape=hex --param colors='#a9d8ee,#f4e27a,#f1c7cb' --param labels='Bloc A|Independent|Bloc B' --param fs=24
pixelox comp place d.pxo grid.png --in-place --as Grid --at 0,0
pixelox text add d.pxo --in-place --name Title --content "All 400 seats are up for election" --font @neo-grotesque --weight 700 --size 46 --color '#1f2933' --at 6%,7% --anchor tl --anchor-to ink
pixelox export d.pxo -o C-seat-grid.png
```

* `cols=` sets the block's width in units; rows follow from the total.
* `order=rows` fills row by row instead.
* **Fails when** the units become smaller than about 14 px: use more columns in a wider box,
  or C-half-donut.

## C-waffle

Script: `scripts/waffle.js`. **Use for** percentages that should feel countable: 100 squares,
one per percent.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#f6f3ec'
pixelox render js scripts/waffle.js -o waffle.png --size 1080x1080 --param box=14,20,72,72 --param values=62,23,15 --param labels='Agree|Not sure|Disagree' --param colors='#0d9488,#cbd2d9,#e4572e' --param from=bottom --param fs=26
pixelox comp place d.pxo waffle.png --in-place --as Waffle --at 0,0
pixelox text add d.pxo --in-place --name Title --content "Six in ten agree" --font @neo-grotesque --weight 700 --size 54 --color '#1f2933' --at 50%,7% --anchor t --anchor-to ink
pixelox export d.pxo -o C-waffle.png
```

* The values must add up to 100 for a percent waffle (the script draws their sum).
* `from=bottom` fills upward like a level; `shape=circle` gives a dot matrix.
* **Fails when** values are rounded so they add to 99 or 101: fix the rounding, do not hide it.

## C-icon-array

Script: `scripts/waffle.js`. **Use for** "4 out of 5 people": a row of figures with some in the
accent color. The strongest way to say a simple ratio.

```bash
pixelox new -o d.pxo --size 1080x720 --fill '#0f5c55'
pixelox render js scripts/waffle.js -o people.png --size 1080x720 --param box=16,14,68,52 --param value=3 --param total=5 --param shape=person --param colors='#ffffff' --param empty='#25d0b4'
pixelox comp place d.pxo people.png --in-place --as People --at 0,0
pixelox text add d.pxo --in-place --name Number --content "3 out of 5" --font @oldstyle --weight 700 --size 84 --color '#ffffff' --at 50%,72% --anchor t --anchor-to ink
pixelox text add d.pxo --in-place --name Label --content "commuters cycle at least once a week" --font @neo-grotesque --size 30 --color '#c9f2ea' --at 50%,87% --anchor t --anchor-to ink
pixelox export d.pxo -o C-icon-array.png
```

* `value=3.5 total=5` fills half of the fourth figure. `total=10` or `20` for finer ratios;
  `cols=10` wraps 20 figures into two rows.
* `shape=circle|square` for abstract units. For another pictogram (cups, houses), place one
  `shape icon` per unit in a row, as in C-icon-grid, and color the first few.
* **Fails when** the ratio is forced (62% drawn as "3 out of 5"): use C-waffle for exact shares.

## C-dot-rows

Script: `scripts/waffle.js`. **Use for** small counts per category (titles per club, medals
per team): a row of dots for each, name on the left, count on the right.

```bash
pixelox new -o d.pxo --size 1080x720 --fill '#2a1247'
pixelox render js scripts/waffle.js -o rows.png --size 1080x720 --param box=8,24,84,62 --param layout=rows --param shape=circle --param values=12,8,8,6,3 --param labels='Club A|Club B|Club C|Club D|Club E' --param colors='#f2b705' --param ink='#ffffff' --param muted='#cdbfe6' --param fs=26
pixelox comp place d.pxo rows.png --in-place --as Rows --at 0,0
pixelox text add d.pxo --in-place --name Title --content "Titles won since 2000" --font @heavy-wide --size 44 --color '#ffffff' --at 8%,8% --anchor tl --anchor-to ink
pixelox export d.pxo -o C-dot-rows.png
```

* Up to about 30 dots per row; the longest row sets the dot size. One color for all rows, or
  `colors=` for one per row.

---

# Maps

Two kinds of map, for two jobs:

* **Outline maps** (C-us-choropleth, C-us-bubble-map, C-world-map, C-globe): real state and
  country shapes on a projection, from the map data bundled with pixelox **0.6.0 or later**.
  These scripts are run with `render d3` (not `render js`). Check `pixelox --version`.
* **Stylized maps** (C-tile-map, C-hex-map, C-dot-map, C-bubble-map): one equal tile per
  region, or a dot-matrix world. They run on any build, need no map data, and keep small
  regions visible. Use them on older builds, or when equal weight per region is the point
  (one tile per state says "51 contests", an outline map says "land area").

The data params are the same in both families (`groups=`, `values=`, `points=`), so a tile map
becomes an outline map by changing the script name. Never draw a country or state outline
freehand with `shape path`: a wrong map is worse than a tile map.

## C-us-choropleth

Script: `scripts/usmap.js` (run with `render d3`). **Use for** a class or a value per US state
on the real map: Albers USA projection, Alaska and Hawaii as insets, the small north-eastern
states listed beside the map.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#ffffff'
pixelox render d3 scripts/usmap.js -o usmap.png --size 1080x1080 --param box=4,20,92,66 --param values='TX:48,CA:41,FL:27,NY:22,IL:19,OH:16,GA:15,PA:14,NC:13,MI:12,WA:11,AZ:10,VA:10,CO:9,MA:9,NJ:9,TN:8,MN:8,IN:8,MO:7,WI:7,MD:7,OR:6,SC:6,AL:6,KY:5,LA:5,OK:5,UT:5,NV:4,IA:4,KS:4,AR:4,CT:4,NM:3,MS:3,NE:3,ID:3,NH:2,ME:2,WV:2,HI:2,MT:2,RI:2,DE:2,SD:1,ND:1,AK:1,VT:1,WY:1,DC:1' --param bins=5 --param ramp='#dbeafe,#1e3a8a' --param labels=both --param fs=22
pixelox comp place d.pxo usmap.png --in-place --as Map --at 0,0
pixelox text add d.pxo --in-place --name Title --content "Stores per state" --font @neo-grotesque --weight 700 --size 48 --color '#1f2933' --at 4%,7% --anchor tl --anchor-to ink
pixelox text add d.pxo --in-place --name Source --content "Example data" --font @neo-grotesque --size 18 --color '#7b8794' --at 4%,95% --anchor bl
pixelox export d.pxo -o C-us-choropleth.png
```

* `values=` with `bins=5` sorts the states into 5 classes between round limits (now and then
  4 or 6, when the data's range is awkward) with a stepped legend; `breaks=10,20,40` sets your own limits; `bins=0` gives a smooth ramp. 4 to 6
  classes is the readable range.
* `groups='CA:1,TX:2'` with `colors=` and `names=` colors by class (results, categories);
  `highlight='CA,TX,NY'` marks a few states in `accent` and grays the rest.
* Labels are dark on light and mid-tone classes and white on dark ones. Pick a ramp whose
  ends are far apart (`'#dbeafe,#1e3a8a'`, `'#d8ede2,#0a5a3c'`); a ramp between two mid-tones
  leaves the labels faint.
* `labels=codes|values|both|none`; `stroke=` is the border color (your ground color);
  `smalls=0` drops the side column for the small states.
* Sequential ramps go from light (low) to dark (high). For a two-sided measure give three
  colors (`ramp='#c8241b,#f1f1f1,#1f4ea3'`) and `min=`/`max=` symmetric around the middle.
* Large states dominate the picture whatever their value: when every state should count the
  same, use C-tile-map.
* Each state's center is logged as an anchor under its code, for callouts.

## C-us-bubble-map

**Use for** a count per state shown as a numbered badge on the map: the bubble's **area**
follows the number, and bubbles push apart where they would overlap.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#1f3a68'
pixelox render d3 scripts/usmap.js -o bubbles.png --size 1080x1080 --param box=4,20,92,70 --param bubbles='TX:48,CA:41,FL:27,NY:22,IL:19,OH:16,GA:15,PA:14,NC:13,MI:12,WA:11,AZ:10,VA:10,CO:9,MA:9,NJ:9,TN:8,MN:8,IN:8,MO:7,WI:7,MD:7,OR:6,SC:6,AL:6,KY:5,LA:5,OK:5,UT:5,NV:4,IA:4,KS:4,AR:4,CT:4,NM:3,MS:3,NE:3,ID:3,NH:2,ME:2,WV:2,HI:2,MT:2,RI:2,DE:2,SD:1,ND:1,AK:1,VT:1,WY:1,DC:1' --param empty='#ddd6c5' --param stroke='#1f3a68' --param bubblecolor='#fbf7ef' --param ink='#1f3a68' --param muted='#1f3a68' --param numfont='Rockwell, Roboto Slab, Georgia, serif' --param labels=none --param fs=22
pixelox comp place d.pxo bubbles.png --in-place --as Bubbles --at 0,0
pixelox text add d.pxo --in-place --name Title --content "ORDERS BY STATE" --font @slab --weight 700 --size 64 --color '#fbf7ef' --at 50%,7% --anchor t --anchor-to ink
pixelox export d.pxo -o C-us-bubble-map.png
```

* `maxr=0.07` is the largest bubble's radius as a fraction of the map width. A bubble moved
  off its state keeps a thin line back to it.
* Combine with `groups=` to color the states under the bubbles.
* Markers (gained, lost, new) go on the logged `bx,by,br` of a state: a small `shape ellipse`
  at the bubble's upper right.

## C-world-map

Script: `scripts/geo.js` (run with `render d3`). **Use for** countries highlighted or colored
by value on a real world map, with markers or value bubbles.

```bash
pixelox new -o d.pxo --size 1080x720 --fill '#ffffff'
pixelox render d3 scripts/geo.js -o world.png --size 1080x720 --param box=3,20,94,72 --param values='USA:420,CAN:60,BRA:120,DEU:90,FRA:80,GBR:85,IND:300,CHN:560,JPN:110,AUS:40,NGA:70,ZAF:35,MEX:75,IDN:130,EGY:55' --param bins=4 --param ramp='#fde7c7,#b45309' --param land='#e6e9ed' --param points='-74:40.7:Office A,77.2:28.6:Office B' --param fs=22
pixelox comp place d.pxo world.png --in-place --as World --at 0,0
pixelox text add d.pxo --in-place --name Title --content "Users by country, millions" --font @neo-grotesque --weight 700 --size 46 --color '#1f2933' --at 3%,7% --anchor tl --anchor-to ink
pixelox export d.pxo -o C-world-map.png
```

* Countries are named by ISO three-letter codes (`pixelox docs atlas` lists them).
  `highlight='BRA,IND'` (or `'BRA:#0d9488,IND:#f97316'`) marks a few; `groups=` and `values=`
  work as in C-us-choropleth.
* `projection=equalearth` (default; areas are true), `natural`, or `mercator` (only when the
  piece is about directions or a zoomed region: it inflates the north).
* Zoom to a region with `extent=lon0,lat0,lon1,lat1`: Europe `-12,35,32,71`, Africa
  `-20,-36,52,38`, South-East Asia `92,-11,142,24`. `rotate=150` centers the Pacific.
* `points='lon:lat:label:value'` adds markers; with values their **areas** are proportional.
* `stroke=none` draws the land without borders (a quiet backdrop behind other modules).

## C-globe

**Use for** "worldwide", a region seen on the sphere, or a hero picture for a global topic.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#101322'
pixelox render d3 scripts/geo.js -o globe.png --size 1080x1080 --param box=12,16,76,76 --param projection=globe --param rotate=15,20 --param land='#3ecf8e' --param ocean='#1f4e8c' --param stroke=none --param points='31.2:30:Hub A,3.4:6.5:Hub B,28:-26.2:Hub C' --param pointcolor='#facc15' --param ink='#ffffff' --param halo='#101322' --param fs=24
pixelox comp place d.pxo globe.png --in-place --as Globe --at 0,0
pixelox text add d.pxo --in-place --name Title --content "Three hubs, one continent" --font @neo-grotesque --weight 700 --size 50 --color '#ffffff' --at 50%,6% --anchor t --anchor-to ink
pixelox export d.pxo -o C-globe.png
```

* `rotate=lon,lat` is the point facing the viewer. Markers on the far side are not drawn.
* `shade=0` makes a flat disc; `graticule=0` removes the grid lines; `highlight=` and
  `values=` color countries as on the flat map.
* A globe hides half the world and squeezes the edges: never use it to compare values across
  countries. It is a picture, with at most a few markers.

## C-tile-map

Script: `scripts/tilemap.js`. **Use for** a class or a value per US state (or per region of
your own grid). Every region gets one equal tile, so small states are as visible as large
ones. It is a diagram of regions, not geography: say "tile map" in the note.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#ffffff'
pixelox render js scripts/tilemap.js -o tiles.png --size 1080x1080 --param box=5,20,90,70 --param groups='WA:1,OR:1,CA:1,NV:1,ID:1,UT:1,AZ:1,MT:1,WY:1,CO:1,NM:1,AK:1,HI:1,ND:2,SD:2,NE:2,KS:2,OK:2,TX:2,MN:2,IA:2,MO:2,AR:2,LA:2,WI:2,IL:2,MS:2,MI:3,IN:3,OH:3,KY:3,TN:3,AL:3,GA:3,FL:3,SC:3,NC:3,VA:3,WV:3,PA:3,NY:3,NJ:3,DE:3,MD:3,DC:3,CT:3,RI:3,MA:3,VT:3,NH:3,ME:3' --param names='West team|Central team|East team' --param colors='#0d9488,#f59e0b,#3b7ddd' --param fs=24
pixelox comp place d.pxo tiles.png --in-place --as Tiles --at 0,0
pixelox text add d.pxo --in-place --name Title --content "Three sales teams, 51 territories" --font @neo-grotesque --weight 700 --size 54 --color '#1f2933' --at 5%,7% --anchor tl --anchor-to ink
pixelox export d.pxo -o C-tile-map.png
```

* `groups='CA:1,TX:2'` gives each region a class number; `colors=` and `names=` follow the
  class numbers. Regions you leave out take the `empty` color.
* `values='CA:54,TX:40'` colors by number on `ramp='#dbeafe,#1d4ed8'` (light to dark; three
  colors make a diverging ramp) and adds a ramp legend; `showvalues=1` writes the numbers.
* Another country or any set of regions: `cells='N:1:0,W:0:1,C:1:1,E:2:1,S:1:2'` (code, column,
  row). Lay the grid out on paper first so neighbors stay neighbors.
* Tile anchors are logged under each region's code, for a marker or callout on one tile.

## C-hex-map

**Use for** the same data with a softer, more map-like feel: hexagons, every second row shifted.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#ffffff'
pixelox render js scripts/tilemap.js -o hex.png --size 1080x1080 --param box=4,20,92,72 --param shape=hex --param showvalues=1 --param values='TX:48,CA:41,FL:27,NY:22,IL:19,OH:16,GA:15,PA:14,NC:13,MI:12,WA:11,AZ:10,VA:10,CO:9,MA:9,NJ:9,TN:8,MN:8,IN:8,MO:7,WI:7,MD:7,OR:6,SC:6,AL:6,KY:5,LA:5,OK:5,UT:5,NV:4,IA:4,KS:4,AR:4,CT:4,NM:3,MS:3,NE:3,ID:3,NH:2,ME:2,WV:2,HI:2,MT:2,RI:2,DE:2,SD:1,ND:1,AK:1,VT:1,WY:1,DC:1' --param ramp='#cfe0f1,#17365d' --param fs=22
pixelox comp place d.pxo hex.png --in-place --as Hex --at 0,0
pixelox text add d.pxo --in-place --name Title --content "Stores per state" --font @neo-grotesque --weight 700 --size 54 --color '#1f2933' --at 5%,7% --anchor tl --anchor-to ink
pixelox export d.pxo -o C-hex-map.png
```

* A true cartogram (several hexagons per region, sized by a value) is not built in: write the
  value on each tile, as here, or pair the map with C-bars for the sizes.

## C-dot-map

Script: `scripts/worldmap.js`. **Use for** "where in the world": a dot-matrix world with
continents tinted and cities marked.

```bash
pixelox new -o d.pxo --size 1080x720 --fill '#ffffff'
pixelox render js scripts/worldmap.js -o world.png --size 1080x720 --param box=4,22,92,66 --param highlight='europe:#2563eb,africa:#f97316' --param points='-74:40.7:Office A,2.3:48.9:Office B,139.7:35.7:Office C,-46.6:-23.5:Office D' --param fs=22
pixelox comp place d.pxo world.png --in-place --as World --at 0,0
pixelox text add d.pxo --in-place --name Title --content "Four offices, two focus regions" --font @neo-grotesque --weight 700 --size 46 --color '#1f2933' --at 4%,7% --anchor tl --anchor-to ink
pixelox export d.pxo -o C-dot-map.png
```

* `points='lon:lat:label'`: west and south are negative. Continent names for `highlight`:
  `namerica`, `samerica`, `europe`, `africa`, `asia`, `oceania`.
* `halo=` is the outline behind the labels: set it to your ground color. `dot=` is the land color.
* The land grid is coarse (5 degrees) and hand-drawn: fine as a locator, wrong for anything
  about borders, areas, or small countries.

## C-bubble-map

**Use for** a value per world region: bubbles whose **areas** follow the values, on the dot map.

```bash
pixelox new -o d.pxo --size 1080x720 --fill '#16233b'
pixelox render js scripts/worldmap.js -o bubbles.png --size 1080x720 --param box=4,22,92,66 --param dot='#3a4a68' --param points='-100:42:North America:420,10:50:Europe:310,105:35:Asia:560,20:0:Africa:90,-60:-15:South America:120,135:-25:Oceania:40' --param pointcolor='#f97316' --param halo='#16233b' --param ink='#ffffff' --param muted='#b8c4d6' --param suffix=M --param fs=22
pixelox comp place d.pxo bubbles.png --in-place --as Bubbles --at 0,0
pixelox text add d.pxo --in-place --name Title --content "Users by region, millions" --font @neo-grotesque --weight 700 --size 46 --color '#ffffff' --at 4%,7% --anchor tl --anchor-to ink
pixelox export d.pxo -o C-bubble-map.png
```

* A fourth field in `points` is the value; `maxr=0.05` is the largest bubble's radius as a
  fraction of the map width.
* Always print the numbers: people judge circle sizes badly.

## C-map-image

**Use for** anything that needs real outlines (a choropleth, a route, a country shape):
bring your own map and let pixelox label it. An SVG or PNG map that you have the right to
use (public-domain sources exist for most countries) is placed like a photo; callouts,
value badges, and the legend are native.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#f6f3ec'
pixelox comp place d.pxo photo.png --in-place --as Map --size 64%x --at 50%,57% --anchor c
pixelox shape ellipse d.pxo --in-place --name Bubble --rect 430,380,120x120 --fill '#ffffff' --stroke '#16233b' --stroke-width 4
pixelox text add d.pxo --in-place --name Value --content "37" --font @neo-grotesque --weight 700 --size 52 --color '#16233b' --at 490,440 --anchor c --anchor-to ink
pixelox shape line d.pxo --in-place --name Leader --from 560,440 --to 700,440 --stroke '#16233b' --stroke-width 2 --dash 3,6
pixelox text add d.pxo --in-place --name Label --content "Largest region" --font @neo-grotesque --size 24 --color '#16233b' --at 712,440 --anchor l
pixelox text add d.pxo --in-place --name Title --content "Your map here" --font @neo-grotesque --weight 700 --size 54 --color '#1f2933' --at 8%,7% --anchor tl --anchor-to ink
pixelox export d.pxo -o C-map-image.png
```

* `photo.png` stands for **your map image**. Recolor a single-color map with
  `fx overlay --kind color`. Value bubbles: one `shape ellipse` and one `text add` per region,
  with the circle's area (not its width) proportional to the value: diameter = k x square
  root of the value.
* Value bubbles need their **areas** proportional to the values.

---

# Process and time

Scripts: `scripts/steps.js` (rows, spines, chevrons, timelines) and `scripts/cycle.js` (loops,
hubs). Both wrap the titles and notes for you and log the center of every node. For icons in
the nodes, pass `badges=none` and place one `shape icon` at each logged anchor.

## C-process-row

**Use for** 3 to 5 steps read left to right: a node per step, a title and a note under it.

```bash
pixelox new -o d.pxo --size 1080x720 --fill '#ffffff'
pixelox render js scripts/steps.js -o row.png --size 1080x720 --param box=6,30,88,56 --param badges=none --param labels='Research|Design|Build|Launch' --param notes='Talk to ten users|Sketch three routes|Ship a first version|Measure and adjust' --param colors='#2563eb,#7c3aed,#db2777,#f97316' --param fs=24
pixelox comp place d.pxo row.png --in-place --as Row --at 0,0
pixelox shape icon d.pxo search --in-place --name I1 --size 60 --at 184,357 --anchor c --color '#ffffff'
pixelox shape icon d.pxo pen-tool --in-place --name I2 --size 60 --at 421,357 --anchor c --color '#ffffff'
pixelox shape icon d.pxo hammer --in-place --name I3 --size 60 --at 659,357 --anchor c --color '#ffffff'
pixelox shape icon d.pxo rocket --in-place --name I4 --size 60 --at 896,357 --anchor c --color '#ffffff'
pixelox text add d.pxo --in-place --name Title --content "From idea to launch in four steps" --font @neo-grotesque --weight 700 --size 50 --color '#1f2933' --at 50%,10% --anchor t --anchor-to ink
pixelox export d.pxo -o C-process-row.png
```

* The icon positions are the `node1` to `node4` anchors from the script's log (run the
  `render js` line with `--json`). Icon size: half the node diameter (`node` in the log).
* Without icons, drop `badges=none`: the nodes carry 01, 02, 03. `style=ring` gives white
  nodes with colored outlines; `onecolor=1` paints every node in the first color.
* `connector=arrow|dashed|none`; `side=alt` puts every second text block above the row
  (needs a taller box).
* **Fails when** there are 6 or more steps or long notes: the columns get too narrow. Use
  C-process-spine.

## C-process-spine

**Use for** 4 to 8 steps read top to bottom on a vertical line, text alternating left and
right: the classic tall process infographic.

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#ffc900'
pixelox render js scripts/steps.js -o spine.png --size 1080x1350 --param box=10,16,80,78 --param dir=v --param side=alt --param badges=none --param node=190 --param labels='01. Collect the numbers|02. Share with the team|03. Search for patterns|04. Find the idea' --param notes='Pull last year from every source into one sheet|Three people look before anyone concludes|Sort, group, and chart until something repeats|Write the one sentence the data supports' --param colors='#ffffff' --param line='#1a1a1a' --param ink='#1a1a1a' --param muted='#1a1a1a' --param fs=26
pixelox comp place d.pxo spine.png --in-place --as Spine --at 0,0
pixelox fx drop-shadow d.pxo --in-place --layer Spine --offset 0,8 --blur 22 --color '#7a5c00' --opacity 30
pixelox shape icon d.pxo chart-column --in-place --name I1 --size 96 --at 540,348 --anchor c --color '#1a1a1a'
pixelox shape icon d.pxo users --in-place --name I2 --size 96 --at 540,611 --anchor c --color '#1a1a1a'
pixelox shape icon d.pxo search --in-place --name I3 --size 96 --at 540,874 --anchor c --color '#1a1a1a'
pixelox shape icon d.pxo lightbulb --in-place --name I4 --size 96 --at 540,1137 --anchor c --color '#1a1a1a'
pixelox text add d.pxo --in-place --name Title --content "HOW WE READ DATA" --font @condensed-heavy --size 72 --color '#1a1a1a' --at 50%,6% --anchor t --anchor-to ink
pixelox export d.pxo -o C-process-spine.png
```

* `side=one` keeps all text to the right of the spine (nodes at the left edge of the box):
  better for long notes.
* `node=` sets the node diameter; the default fits the box.
* The drop shadow on the placed chart lifts nodes and text alike; for a shadow under the
  nodes only, render the script twice (once with `labels` and `notes` empty) and shadow that layer.
* **Fails when** titles are long: at `side=alt` each text block has under half the width.
  Keep titles to 4 words and notes to 12.

## C-chevrons

**Use for** stages of a pipeline that hand over to each other: a band of interlocking arrows.

```bash
pixelox new -o d.pxo --size 1080x540 --fill '#ffffff'
pixelox render js scripts/steps.js -o chev.png --size 1080x540 --param box=5,30,90,56 --param style=chevron --param labels='Idea|Team|Launch|Growth|Goal' --param notes='Find the gap|Hire for it|Ship early|Double down|Reach it' --param colors='#f59e0b,#f97316,#ec4899,#8b5cf6,#10b981' --param fs=24
pixelox comp place d.pxo chev.png --in-place --as Chevrons --at 0,0
pixelox text add d.pxo --in-place --name Title --content "Five stages, one direction" --font @neo-grotesque --weight 700 --size 46 --color '#1f2933' --at 50%,9% --anchor t --anchor-to ink
pixelox export d.pxo -o C-chevrons.png
```

* `node=` sets the band height; `badges='A,B,C,D,E'` or `badges=none`.
* A single hue in five steps from light to dark says "progress" better than five hues.

## C-timeline

**Use for** dated events: dots on a line, the date in bold at each dot.

```bash
pixelox new -o d.pxo --size 1080x720 --fill '#ffffff'
pixelox render js scripts/steps.js -o time.png --size 1080x720 --param box=5,24,90,68 --param style=dot --param side=alt --param badges='2019,2021,2023,2024,2026' --param labels='Founded|First product|Series A|Global launch|One million users' --param notes='Two people, one desk|Built in a garage|Twelve new hires|Nine markets at once|And still growing' --param onecolor=1 --param colors='#e4572e' --param fs=24
pixelox comp place d.pxo time.png --in-place --as Timeline --at 0,0
pixelox text add d.pxo --in-place --name Title --content "Seven years in five moments" --font @neo-grotesque --weight 700 --size 50 --color '#1f2933' --at 5%,8% --anchor tl --anchor-to ink
pixelox export d.pxo -o C-timeline.png
```

* `dir=v` makes a vertical timeline (an agenda, a history): dates to the left of the line,
  text to the right.
* The dots are evenly spaced whatever the dates: it is a sequence, not a time scale. When the
  gaps between dates matter, say so in the notes ("two years later").

## C-snake-steps

**Use for** a journey of 4 to 8 stages: nodes zig-zag down the page on a thick curved ribbon
(or a road with a dashed center line), text on the outer side of each node.

```bash
pixelox new -o d.pxo --size 1080x1528 --fill '#f4f1ea'
pixelox render js scripts/steps.js -o road.png --size 1080x1528 --param box=6,14,88,82 --param style=snake --param connector=dashed --param labels='Declare|Campaign|First votes|Nomination|Election day' --param notes='File the papers and name a team|Meet voters in every region|Win delegates region by region|The party names its candidate|Voters choose' --param colors='#1f4ea3,#c8241b' --param line='#22303f' --param fs=26
pixelox comp place d.pxo road.png --in-place --as Road --at 0,0
pixelox text add d.pxo --in-place --name Title --content "The road to election day" --font @slab --weight 700 --size 60 --color '#22303f' --at 50%,5% --anchor t --anchor-to ink
pixelox export d.pxo -o C-snake-steps.png
```

* `connector=line` gives a plain ribbon in `line=`; `linewidth=` sets its thickness;
  `swing=0.13` is how far the nodes swing from the middle; `badges=none` leaves the nodes
  empty for icons at the logged anchors.
* Two colors alternating (as here) or one color per step.

## C-cycle

**Use for** a loop of 3 to 9 steps that starts again: nodes on a ring, arrows between them.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#ffffff'
pixelox render js scripts/cycle.js -o cycle.png --size 1080x1080 --param box=4,18,92,78 --param labels='Plan|Build|Measure|Learn' --param notes='Set one goal for the cycle|Make the smallest thing that tests it|Count what happened|Decide what changes next' --param center='Loop' --param sub='every two weeks' --param colors='#4b3fc4,#2f9bdc,#111111,#4b3fc4' --param fs=26
pixelox comp place d.pxo cycle.png --in-place --as Cycle --at 0,0
pixelox text add d.pxo --in-place --name Title --content "The improvement loop" --font @neo-grotesque --weight 700 --size 54 --color '#1f2933' --at 50%,6% --anchor t --anchor-to ink
pixelox export d.pxo -o C-cycle.png
```

* Steps run clockwise from the top; `start=45` turns the ring.
* `labelwidth=0.22` is the room for labels on each side (a fraction of the box width): lower
  it for short labels and the ring grows; raise it (0.28) for long notes.
* 7 to 9 steps: keep notes to 3 words.
* Put an illustration or icon in the middle at the `center` anchor instead of `center=` text.

## C-cycle-segments

**Use for** a loop where the stages themselves are the picture: the ring is cut into arrow
segments, numbered.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#ffffff'
pixelox render js scripts/cycle.js -o seg.png --size 1080x1080 --param box=4,18,92,78 --param style=segments --param labels='Reduce|Reuse|Repair|Recycle|Recover' --param notes='Buy less|Use again|Fix it first|Sort it well|The last resort' --param center=5 --param sub='habits' --param colors='#0d9488,#14b8a6,#2dd4bf,#f59e0b,#f97316' --param fs=26
pixelox comp place d.pxo seg.png --in-place --as Segments --at 0,0
pixelox text add d.pxo --in-place --name Title --content "Waste less in five moves" --font @neo-grotesque --weight 700 --size 54 --color '#1f2933' --at 50%,6% --anchor t --anchor-to ink
pixelox export d.pxo -o C-cycle-segments.png
```

* `node=` sets the ring thickness.

## C-hub-spokes

**Use for** one topic with 4 to 8 facets and no order: a hub in the middle, a node per facet.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#101322'
pixelox render js scripts/cycle.js -o hub.png --size 1080x1080 --param box=5,18,90,76 --param style=spokes --param badges=none --param labels='Search|Social|Email|Events|Partners|Referrals' --param center='Leads' --param labelwidth=0.16 --param ink='#ffffff' --param muted='#aab3d1' --param line='#3a4166' --param colors='#22d3ee,#e879f9,#facc15,#fb7185,#a78bfa,#4ade80' --param fs=26
pixelox comp place d.pxo hub.png --in-place --as Hub --at 0,0
pixelox text add d.pxo --in-place --name Title --content "Six ways leads arrive" --font @neo-grotesque --weight 700 --size 54 --color '#ffffff' --at 50%,6% --anchor t --anchor-to ink
pixelox export d.pxo -o C-hub-spokes.png
```

* For icons in the nodes, read the `node1` ... anchors from the log and place a `shape icon`
  at each (size = half the logged `node` diameter, dark on light nodes).
* A hub implies "all of these feed one thing". For steps in order use C-cycle.

---

# Concepts

## C-venn

Script: `scripts/venn.js`. **Use for** what 2 to 4 things share. The overlaps mix by
themselves (`blend=multiply` on light grounds, `blend=screen` on dark).

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#ffffff'
pixelox render js scripts/venn.js -o venn.png --size 1080x1080 --param box=10,18,80,78 --param labels='Useful|Usable|Beautiful' --param overlaps='works|admired|pleasant' --param center='Loved' --param colors='#2563eb,#f97316,#0d9488' --param fs=26
pixelox comp place d.pxo venn.png --in-place --as Venn --at 0,0
pixelox text add d.pxo --in-place --name Title --content "What makes a product loved" --font @neo-grotesque --weight 700 --size 54 --color '#1f2933' --at 50%,6% --anchor t --anchor-to ink
pixelox export d.pxo -o C-venn.png
```

* Order of `overlaps` for 3 sets: top with bottom-left, top with bottom-right, the two bottom
  ones. For 2 sets: one label. For 4 sets (top, right, bottom, left): the four neighbor
  overlaps clockwise from the top-right; `notes=` adds a line outside each circle.
* `overlap=0.3` to `0.7` moves the circles apart or together; `opacity=` and `labelcolor=`
  tune contrast; `caps=1` sets the names in tracked capitals.
* The areas mean nothing: never read sizes from a Venn. For real set sizes use C-bars.

## C-pyramid

Script: `scripts/pyramid.js`. **Use for** levels that rest on each other (needs, priorities,
a strategy stack): equal-height tiers, names beside them on leaders.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#ffffff'
pixelox render js scripts/pyramid.js -o pyr.png --size 1080x1080 --param box=8,20,84,72 --param labels='Vision|Strategy|Projects|Daily tasks' --param notes='Why we exist|Where we choose to play|What we build this year|What we do today' --param colors='#0b3c6f,#1263a8,#2b8fd8,#7cc0f0' --param fs=26
pixelox comp place d.pxo pyr.png --in-place --as Pyramid --at 0,0
pixelox text add d.pxo --in-place --name Title --content "Every task hangs from the vision" --font @neo-grotesque --weight 700 --size 50 --color '#1f2933' --at 8%,7% --anchor tl --anchor-to ink
pixelox export d.pxo -o C-pyramid.png
```

* `labelpos=in` writes the names inside the tiers (short names only; the top tier is narrow);
  `labelpos=left` mirrors the layout; `flip=1` points the pyramid down.
* Tier anchors (`tier1` ...) take icons: white, half the tier height.
* The tiers are equal in height on purpose: a pyramid shows order, not amounts.

## C-funnel

**Use for** counts that shrink stage by stage (visitors to customers). Tier widths follow the
values.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#ffffff'
pixelox render js scripts/pyramid.js -o funnel.png --size 1080x1080 --param box=8,20,84,72 --param values=1000,620,240,80 --param labels='Visitors|Sign-ups|Trials|Customers' --param notes='Everyone who landed|Left an email|Started the trial|Paid in month one' --param labelpos=right --param pct=1 --param colors='#1e3a5f,#2563eb,#60a5fa,#f97316' --param fs=26
pixelox comp place d.pxo funnel.png --in-place --as Funnel --at 0,0
pixelox text add d.pxo --in-place --name Title --content "Eight in a hundred become customers" --font @neo-grotesque --weight 700 --size 48 --color '#1f2933' --at 8%,7% --anchor tl --anchor-to ink
pixelox export d.pxo -o C-funnel.png
```

* `labelpos=in` (the default for a funnel) writes name and value inside each tier.
* `pct=1` adds each stage's share of the first stage.
* Widths are proportional, areas are not: always print the numbers.

---

# Metaphor objects

Script: `scripts/object.js` draws a flat object from shapes and cuts it into bands: one band
per step or part. The script logs, for every band, its center and the points on its left and
right edges, so you can hang a title and a note on each band with a leader line.

## C-rocket

**Use for** steps toward a launch or a goal: growth, start-up, roadmap.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#ffffff'
pixelox shape ellipse d.pxo --in-place --name Cloud1 --rect 300,860,300x150 --fill '#dff5f1'
pixelox shape ellipse d.pxo --in-place --name Cloud2 --rect 470,840,340x190 --fill '#dff5f1'
pixelox shape ellipse d.pxo --in-place --name Cloud3 --rect 200,900,700x160 --fill '#dff5f1'
pixelox render js scripts/object.js -o rocket.png --size 1080x1080 --param box=36,6,28,88 --param kind=rocket --param bands=4 --param colors='#ff5a7a,#8b5cf6,#2dd4bf,#fbbf24'
pixelox comp place d.pxo rocket.png --in-place --as Rocket --at 0,0
pixelox shape line d.pxo --in-place --name L1 --from 630,304 --to 760,304 --stroke '#52606d' --stroke-width 2
pixelox text add d.pxo --in-place --name T1 --content "DREAM" --font @condensed --weight 700 --size 40 --color '#ff5a7a' --at 776,290 --anchor l
pixelox text add d.pxo --in-place --name N1 --content $'Name the one thing\nyou want to exist' --font @neo-grotesque --size 22 --leading 130% --color '#52606d' --at 776,318 --anchor tl
pixelox shape line d.pxo --in-place --name L2 --from 437,447 --to 310,447 --stroke '#52606d' --stroke-width 2
pixelox text add d.pxo --in-place --name T2 --content "PLAN" --font @condensed --weight 700 --size 40 --color '#8b5cf6' --at 294,433 --anchor r
pixelox text add d.pxo --in-place --name N2 --content $'Cut it down to what\nfits in ninety days' --font @neo-grotesque --size 22 --leading 130% --align right --color '#52606d' --at 294,461 --anchor tr
pixelox shape line d.pxo --in-place --name L3 --from 643,571 --to 760,571 --stroke '#52606d' --stroke-width 2
pixelox text add d.pxo --in-place --name T3 --content "BUILD" --font @condensed --weight 700 --size 40 --color '#14b8a6' --at 776,557 --anchor l
pixelox text add d.pxo --in-place --name N3 --content $'Ship something small\nevery single week' --font @neo-grotesque --size 22 --leading 130% --color '#52606d' --at 776,585 --anchor tl
pixelox shape line d.pxo --in-place --name L4 --from 437,694 --to 310,694 --stroke '#52606d' --stroke-width 2
pixelox text add d.pxo --in-place --name T4 --content "LAUNCH" --font @condensed --weight 700 --size 40 --color '#d97706' --at 294,680 --anchor r
pixelox text add d.pxo --in-place --name N4 --content $'Tell everyone, then\nlisten to what they say' --font @neo-grotesque --size 22 --leading 130% --align right --color '#52606d' --at 294,708 --anchor tr
pixelox text add d.pxo --in-place --name Title --content "LIFT-OFF PLAN" --font @condensed --weight 700 --size 64 --color '#3b3561' --at 7%,7% --anchor tl --anchor-to ink
pixelox export d.pxo -o C-rocket.png
```

* Leaders start 16 px outside the band's edge point (`rx` + 16 on the right, `lx` - 16 on
  the left) and run 120 to 130 px; the title sits at the leader's end, the note under it.
  Alternate sides so the text blocks never stack.
* `rotate=30` tilts the rocket to climb right (the anchors follow); `window=1` adds a
  porthole; `flame=0` removes the exhaust.
* The clouds are three pale ellipses behind the rocket: any soft base (a platform, a ground
  line) works.

## C-bulb

**Use for** ideas, insight, energy: a bulb cut into 2 to 4 bands, or a hub for callouts.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#fff8e6'
pixelox render js scripts/object.js -o bulb.png --size 1080x1080 --param box=30,16,40,74 --param kind=bulb --param bands=3 --param rays=1 --param colors='#fbbf24,#f97316,#ef4444'
pixelox comp place d.pxo bulb.png --in-place --as Bulb --at 0,0
pixelox text add d.pxo --in-place --name Title --content "Three parts of a good idea" --font @neo-grotesque --weight 700 --size 50 --color '#1f2933' --at 50%,5% --anchor t --anchor-to ink
pixelox export d.pxo -o C-bulb.png
```

* Label the bands exactly as in C-rocket: leaders from the `lx`/`rx` anchors.
* `badges=none` for plain bands; `bands=1` and one color for a simple icon-like bulb to put
  in the middle of C-hub-spokes or C-cycle.

## C-pencil

**Use for** writing, education, drafting stages: a pencil whose barrel is the band of steps.

```bash
pixelox new -o d.pxo --size 1080x720 --fill '#ffffff'
pixelox render js scripts/object.js -o pencil.png --size 1080x720 --param box=6,28,88,36 --param kind=pencil --param bands=5 --param rotate=-90 --param colors='#22c55e,#14b8a6,#3b82f6,#8b5cf6,#ec4899'
pixelox comp place d.pxo pencil.png --in-place --as Pencil --at 0,0
pixelox text add d.pxo --in-place --name Title --content "Five drafts to a finished page" --font @neo-grotesque --weight 700 --size 46 --color '#1f2933' --at 50%,9% --anchor t --anchor-to ink
pixelox export d.pxo -o C-pencil.png
```

* `rotate=-90` lays the pencil down with its point to the right, so band 01 is on the left and
  the steps read toward the tip; `rotate=90` points it left; `rotate=30` leans it as if writing.
* On a lying pencil the band anchors' `lx,ly` and `rx,ry` are the top and bottom edge points:
  hang the labels above and below.

## C-picture-gauge

**Use for** one level shown as a filled object: a thermometer (heat, risk, progress), a
battery (charge, capacity, energy), a bulb (a share of "ideas" or power).

```bash
pixelox new -o d.pxo --size 1080x720 --fill '#ffffff'
pixelox render js scripts/object.js -o thermo.png --size 1080x720 --param box=8,14,20,72 --param kind=thermometer --param value=68 --param colors='#ef4444'
pixelox render js scripts/object.js -o battery.png --size 1080x720 --param box=40,20,20,60 --param kind=battery --param value=40 --param colors='#16a34a'
pixelox render js scripts/object.js -o bulb.png --size 1080x720 --param box=70,16,22,68 --param kind=bulb --param value=75 --param colors='#f59e0b' --param empty='#fde9b8'
pixelox comp place d.pxo thermo.png --in-place --as Thermo --at 0,0
pixelox comp place d.pxo battery.png --in-place --as Battery --at 0,0
pixelox comp place d.pxo bulb.png --in-place --as Bulb --at 0,0
pixelox text add d.pxo --in-place --name V1 --content "68%" --font @neo-grotesque --weight 700 --size 44 --color '#ef4444' --at 18%,92% --anchor b
pixelox text add d.pxo --in-place --name V2 --content "40%" --font @neo-grotesque --weight 700 --size 44 --color '#16a34a' --at 50%,92% --anchor b
pixelox text add d.pxo --in-place --name V3 --content "75%" --font @neo-grotesque --weight 700 --size 44 --color '#d97706' --at 81%,92% --anchor b
pixelox export d.pxo -o C-picture-gauge.png
```

* `value=` and `max=` set the fill height. Height is proportional; the filled **area** of a
  bulb is not: always print the number beside the object.
* `kind=battery --param bands=5 --param badges=none` draws a battery of 5 cells instead.

---

# Radial and 3D

## C-radial-bars

Script: `scripts/radial.js`. **Use for** a cycle of values (months, hours, a set of 8 to 24
items) as bars radiating from a hub, with the total in the middle. Decorative: lengths are
exact, but bars in different directions are harder to compare than C-bars.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#ffffff'
pixelox render js scripts/radial.js -o radial.png --size 1080x1080 --param box=6,16,88,80 --param values=62,48,80,35,55,71,44,90,66,58,39,73 --param labels='Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec' --param colors='#9aa5b1' --param highlight=8 --param accent='#e4572e' --param rings=4 --param center=721 --param sub='orders this year' --param fs=24
pixelox comp place d.pxo radial.png --in-place --as Radial --at 0,0
pixelox text add d.pxo --in-place --name Title --content "August was the peak" --font @neo-grotesque --weight 700 --size 54 --color '#1f2933' --at 50%,5% --anchor t --anchor-to ink
pixelox export d.pxo -o C-radial-bars.png
```

* Bars start at the hub's edge (`inner=0.36`), which stands for zero. `max=` fixes the value
  of a full-length bar; `rings=4` adds guide rings; `track=1` puts each bar on a pale track.
* Above 12 bars the names are written along the spokes (`rotate=1`).
* **Fails when** the order is not a cycle and the gaps matter: use C-bars.

## C-radial-list

**Use for** many short entries around a picture or a number (a career by season, the parts of
a wheel): spokes with the entries written along them.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#fbf7ef'
pixelox render js scripts/radial.js -o spokes.png --size 1080x1080 --param box=3,12,94,86 --param labels='1998 First cap|2001 League title|2003 Cup final|2005 Top scorer|2008 Record fee|2010 Captain|2012 Second title|2014 Hundred goals|2016 Final season|2018 Coach|2020 First trophy|2022 National team|2024 Hall of fame|2025 Academy' --param rotate=1 --param colors='#b3261c' --param center=27 --param sub=seasons --param fs=24
pixelox comp place d.pxo spokes.png --in-place --as Spokes --at 0,0
pixelox text add d.pxo --in-place --name Title --content "A career, season by season" --font @oldstyle --weight 700 --size 50 --color '#22303f' --at 50%,4% --anchor t --anchor-to ink
pixelox export d.pxo -o C-radial-list.png
```

* Without `values=` the script draws a ring of spokes and labels only. Entries read
  clockwise from 12 o'clock; labels on the left half are turned to stay readable.
* Put a cutout photo or an object on the `center` anchor; keep entries to 3 or 4 words.

## C-3d-bars

Script: `scripts/bars3d.js`. **Use for** a hero chart of 3 to 6 values as isometric blocks.
The view is parallel, so block heights stay proportional and every value is printed; it is
still harder to read than flat columns. Use it once, large, never for a dashboard of charts.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#1c1b3a'
pixelox render js scripts/bars3d.js -o blocks.png --size 1080x1080 --param box=8,22,84,68 --param values=52,80,31,60,44 --param labels='North,South,East,West,Online' --param colors='#a855f7,#ec4899,#f97316,#facc15,#22d3ee' --param floor='#25244a' --param ink='#ffffff' --param muted='#9aa3c7' --param suffix=% --param fs=26
pixelox comp place d.pxo blocks.png --in-place --as Blocks --at 0,0
pixelox text add d.pxo --in-place --name Title --content "South leads every channel" --font @neo-grotesque --weight 700 --size 54 --color '#ffffff' --at 8%,7% --anchor tl --anchor-to ink
pixelox export d.pxo -o C-3d-bars.png
```

* `shape=cylinder` for round columns; `depth=0.6` for slabs; one color (`colors='#2563eb'`)
  is calmer than five.
* True perspective, lit scenes, and free 3D objects are possible with `render three`
  (`pixelox docs atlas` and the cookbook `charts-and-maps-with-d3` show how), but perspective
  bends heights: never put data on a perspective view.

---

# Furniture

Built with native commands: nothing here needs a script. Coordinates are px on a 1080 px wide
canvas; keep left edges on your margin (76 px = 7% is used below).

## C-title-block

**Use for** the head of every infographic: an accent mark, a title that states the finding, a
deck that says what is measured, and a rule.

```bash
pixelox new -o d.pxo --size 1080x400 --fill '#f4f6f8'
pixelox shape rect d.pxo --in-place --name Accent --rect 76,70,18x168 --fill '#f2b705'
pixelox text add d.pxo --in-place --name Title --content "The rise of the four-day week" --font @neo-grotesque --weight 700 --size 68 --color '#16233b' --at 122,70 --anchor tl --anchor-to ink
pixelox text add d.pxo --in-place --name Deck --content $'Companies with a four-day week,\nas a share of all employers' --font @neo-grotesque --size 32 --leading 130% --color '#52606d' --at 122,152 --anchor tl --anchor-to ink
pixelox shape line d.pxo --in-place --name Rule --from 76,300 --to 1004,300 --stroke '#cbd2d9' --stroke-width 2
pixelox export d.pxo -o C-title-block.png
```

* The title is a sentence with a finding ("North leads by eleven points"), not a topic
  ("Regional sales"). Fit a one-line title to the margins with `--fit-width 928` instead of `--size`.
* Centered variant: `--at 50%,70 --anchor t` on both lines and no accent bar.

## C-section-band

**Use for** dividing a long infographic into modules: a full-width band or a pill with the
module's name.

```bash
pixelox new -o d.pxo --size 1080x420 --fill '#f1effc'
pixelox shape rect d.pxo --in-place --name Band --rect 0,60,1080x84 --fill '#6d3fd6'
pixelox text add d.pxo --in-place --name BandText --content "Who is watching" --font @geometric-sans --weight 700 --size 34 --color '#ffffff' --at 50%,102 --anchor c --anchor-to ink
pixelox shape rect d.pxo --in-place --name Pill --rect 76,240,360x64 --radius 32 --fill '#f97316'
pixelox text add d.pxo --in-place --name PillText --content "KEY NUMBERS" --font @geometric-sans --weight 700 --size 26 --tracking 80 --color '#ffffff' --at 256,272 --anchor c --anchor-to ink
pixelox shape line d.pxo --in-place --name PillRule --from 436,272 --to 1004,272 --stroke '#f97316' --stroke-width 3
pixelox export d.pxo -o C-section-band.png
```

* Band height 7 to 8% of the width; text 3 to 3.4% of the width. Alternate the ground color
  of the modules between bands (light, dark, light) so each module is one block.

## C-card

**Use for** dashboards: every chart sits in its own white panel with a soft shadow and a
centered title. The chart's `box` is the card's inside.

```bash
pixelox new -o d.pxo --size 1080x720 --fill '#eef0fb'
pixelox shape rect d.pxo --in-place --name Card --rect 60,60,960x600 --radius 24 --fill '#ffffff'
pixelox fx drop-shadow d.pxo --in-place --layer Card --offset 0,10 --blur 30 --color '#1e1b6b' --opacity 14
pixelox text add d.pxo --in-place --name CardTitle --content "Customer preferences" --font @geometric-sans --weight 700 --size 32 --color '#1e1b6b' --at 50%,112 --anchor c --anchor-to ink
pixelox render js scripts/bars.js -o prefs.png --size 1080x720 --param box=11,24,78,60 --param dir=h --param track=1 --param max=100 --param values=46,31,18,12 --param labels='Free delivery|Easy returns|Loyalty points|Gift wrapping' --param colors='#3f37c9,#7b6cf6,#f2b705,#f97316' --param suffix=% --param fs=24
pixelox comp place d.pxo prefs.png --in-place --as Prefs --at 0,0
pixelox export d.pxo -o C-card.png
```

* Card padding 5% of the card width; radius 16 to 24 px; shadow opacity 10 to 18 in a dark
  tint of the ground's hue, never pure black.
* A grid of cards: same gutter between cards as the outer margin's half (for example margin
  60, gutter 30); card heights may differ, tops and left edges must align.
* Outline variant: `--fill none --stroke '#c7cbe8' --stroke-width 2` and no shadow.

## C-stat-block

**Use for** one number that matters: icon, hero number, caption.

```bash
pixelox new -o d.pxo --size 1080x540 --fill '#0b0b4f'
pixelox shape ellipse d.pxo --in-place --name Badge --rect 76,190,160x160 --fill '#6d3fd6'
pixelox shape icon d.pxo trending-up --in-place --name Icon --size 84 --at 156,270 --anchor c --color '#ffffff'
pixelox text add d.pxo --in-place --name Number --content "1.4 billion" --font @geometric-sans --weight 700 --size 110 --color '#ffffff' --at 280,240 --anchor l --anchor-to ink
pixelox text add d.pxo --in-place --name Caption --content "people ride a bike every week" --font @geometric-sans --size 32 --color '#c9c4f5' --at 284,330 --anchor l
pixelox export d.pxo -o C-stat-block.png
```

* Number 3 to 6 times the caption size, in the heaviest weight; the unit ("billion", "%")
  in the same size or one step smaller, never tiny.
* Round honestly: "2.91 billion" or "about 3 billion", not "3 billion" for 2.6.

## C-stat-row

**Use for** 3 or 4 key numbers side by side under a hero chart.

```bash
pixelox new -o d.pxo --size 1080x420 --fill '#16233b'
pixelox text add d.pxo --in-place --name N1 --content "180M" --font @heavy-wide --size 72 --color '#ffffff' --at 16.5%,150 --anchor c --anchor-to ink
pixelox text add d.pxo --in-place --name N2 --content "126M" --font @heavy-wide --size 72 --color '#e2493f' --at 50%,150 --anchor c --anchor-to ink
pixelox text add d.pxo --in-place --name N3 --content "48M" --font @heavy-wide --size 72 --color '#3d8fd1' --at 83.5%,150 --anchor c --anchor-to ink
pixelox text add d.pxo --in-place --name C1 --content $'eligible\nvoters' --font @neo-grotesque --size 26 --leading 130% --align center --color '#d5dbe1' --at 16.5%,215 --anchor t
pixelox text add d.pxo --in-place --name C2 --content $'registered\nto vote' --font @neo-grotesque --size 26 --leading 130% --align center --color '#d5dbe1' --at 50%,215 --anchor t
pixelox text add d.pxo --in-place --name C3 --content $'voted\nearly' --font @neo-grotesque --size 26 --leading 130% --align center --color '#d5dbe1' --at 83.5%,215 --anchor t
pixelox shape line d.pxo --in-place --name D1 --from 33.3%,110 --to 33.3%,290 --stroke '#3a4a68' --stroke-width 2
pixelox shape line d.pxo --in-place --name D2 --from 66.6%,110 --to 66.6%,290 --stroke '#3a4a68' --stroke-width 2
pixelox export d.pxo -o C-stat-row.png
```

* Column centers for N columns inside the margins: 3 columns at 16.5%, 50%, 83.5%; 4 columns
  at 14%, 38%, 62%, 86%.
* Color each number like the thing it counts in the charts above (the same red and blue).

## C-icon-badge

**Use for** marking a module, a step, or a fact with one pictogram: an icon in a disc.

```bash
pixelox new -o d.pxo --size 1080x360 --fill '#ffffff'
pixelox shape ellipse d.pxo --in-place --name B1 --rect 120,100,160x160 --fill '#2563eb'
pixelox shape icon d.pxo globe --in-place --name I1 --size 84 --at 200,180 --anchor c --color '#ffffff'
pixelox shape ellipse d.pxo --in-place --name B2 --rect 460,100,160x160 --fill '#ffffff' --stroke '#f97316' --stroke-width 6
pixelox shape icon d.pxo wallet --in-place --name I2 --size 84 --at 540,180 --anchor c --color '#f97316'
pixelox shape rect d.pxo --in-place --name B3 --rect 800,100,160x160 --radius 36 --fill '#e6f6f3'
pixelox shape icon d.pxo clock --in-place --name I3 --size 84 --at 880,180 --anchor c --color '#0d9488'
pixelox export d.pxo -o C-icon-badge.png
```

* Icon size about half the badge. One badge style per infographic: all filled, all outlined,
  or all tinted squares. Icon names: [icons.md](icons.md).
* `--stroke-width 5` on `shape icon` for a bolder line at large sizes (default is size / 12).

## C-icon-grid

**Use for** 4 to 9 facts of equal weight, each an icon, a bold label, and a short line.

```bash
pixelox new -o d.pxo --size 1080x480 --fill '#f6f3ec'
pixelox shape icon d.pxo megaphone --in-place --name I1 --size 80 --at 20%,170 --anchor c --color '#e4572e'
pixelox shape icon d.pxo handshake --in-place --name I2 --size 80 --at 50%,170 --anchor c --color '#e4572e'
pixelox shape icon d.pxo target --in-place --name I3 --size 80 --at 80%,170 --anchor c --color '#e4572e'
pixelox text add d.pxo --in-place --name H1 --content "Attract" --font @neo-grotesque --weight 700 --size 32 --color '#1f2933' --at 20%,250 --anchor t
pixelox text add d.pxo --in-place --name H2 --content "Agree" --font @neo-grotesque --weight 700 --size 32 --color '#1f2933' --at 50%,250 --anchor t
pixelox text add d.pxo --in-place --name H3 --content "Deliver" --font @neo-grotesque --weight 700 --size 32 --color '#1f2933' --at 80%,250 --anchor t
pixelox text add d.pxo --in-place --name P1 --content "Say one thing to one audience, clearly." --font @neo-grotesque --size 24 --leading 135% --align center --color '#52606d' --box 250x120 --at 20%,300 --anchor t
pixelox text add d.pxo --in-place --name P2 --content "Put the scope and the price on one page." --font @neo-grotesque --size 24 --leading 135% --align center --color '#52606d' --box 250x120 --at 50%,300 --anchor t
pixelox text add d.pxo --in-place --name P3 --content "Hit the date you named, every time." --font @neo-grotesque --size 24 --leading 135% --align center --color '#52606d' --box 250x120 --at 80%,300 --anchor t
pixelox shape line d.pxo --in-place --name R1 --from 35%,130 --to 35%,400 --stroke '#d9d2c3' --stroke-width 2
pixelox shape line d.pxo --in-place --name R2 --from 65%,130 --to 65%,400 --stroke '#d9d2c3' --stroke-width 2
pixelox export d.pxo -o C-icon-grid.png
```

* `--box WxH` wraps the paragraph; keep each to 2 or 3 lines of the same length.
* A second row repeats the three columns 330 px lower. All icons one size and one color.

## C-callout

**Use for** pointing at one part of a chart or picture: a dot on the spot, a thin leader, a
short label.

```bash
pixelox new -o d.pxo --size 1080x540 --fill '#ffffff'
pixelox render js scripts/line.js -o line.png --size 1080x540 --param box=6,12,60,76 --param values='12,18,17,26,31,44' --param labels='2020,2021,2022,2023,2024,2025' --param vlabels=none --param colors='#2563eb' --param fs=22
pixelox comp place d.pxo line.png --in-place --as Line --at 0,0
pixelox shape ellipse d.pxo --in-place --name Dot --rect 334,303,22x22 --fill '#e4572e'
pixelox shape path d.pxo --in-place --name Leader --d 'M 345 314 L 345 180 L 740 180' --fill none --stroke '#e4572e' --stroke-width 2
pixelox text add d.pxo --in-place --name CallT --content "Prices rose in 2022" --font @neo-grotesque --weight 700 --size 26 --color '#e4572e' --at 752,170 --anchor l
pixelox text add d.pxo --in-place --name CallN --content $'The only year members\nfell, by about 5%' --font @neo-grotesque --size 22 --leading 130% --color '#52606d' --at 752,196 --anchor tl
pixelox export d.pxo -o C-callout.png
```

* The dot's center (345,314) is the `p1.3` anchor from the line script's log.
* Leaders are horizontal, vertical, or one elbow; never a free diagonal across the chart.
  Dashed variant: `--dash 3,6`.

## C-speech-bubble

**Use for** a value badge on a map or a node with a pointer: a circle or a rounded box with a
small tail.

```bash
pixelox new -o d.pxo --size 1080x420 --fill '#1f3a68'
pixelox shape path d.pxo --in-place --name Bubble1 --d 'M 250 90 A 110 110 0 1 1 204 300 L 150 350 L 168 276 A 110 110 0 0 1 250 90 Z' --fill '#fbf7ef'
pixelox text add d.pxo --in-place --name V1 --content "37" --font @slab --weight 700 --size 84 --color '#1f3a68' --at 250,200 --anchor c --anchor-to ink
pixelox shape path d.pxo --in-place --name Bubble2 --d 'M 560 110 L 900 110 A 24 24 0 0 1 924 134 L 924 250 A 24 24 0 0 1 900 274 L 700 274 L 660 330 L 660 274 L 560 274 A 24 24 0 0 1 536 250 L 536 134 A 24 24 0 0 1 560 110 Z' --fill '#f2a541'
pixelox text add d.pxo --in-place --name V2 --content "Largest bloc" --font @neo-grotesque --weight 700 --size 40 --color '#1f3a68' --at 730,192 --anchor c --anchor-to ink
pixelox export d.pxo -o C-speech-bubble.png
```

* To move a bubble, add the same number to every x (or every y) in its path. To resize the
  round one, keep its form: center (cx, cy), radius r, tail tip at (cx - 0.9r, cy + 1.36r).
* Size the circle's **area** by the value when bubbles encode data, and print the value.

## C-ribbon-label

**Use for** a module title that should feel like a banner: a strip with folded ends.

```bash
pixelox new -o d.pxo --size 1080x300 --fill '#e9e9e9'
pixelox shape path d.pxo --in-place --name Tails --d 'M 250 120 L 330 120 L 330 196 L 250 196 L 284 158 Z M 830 120 L 750 120 L 750 196 L 830 196 L 796 158 Z' --fill '#55595f'
pixelox shape rect d.pxo --in-place --name Strip --rect 300,100,480x76 --fill '#8a8f96'
pixelox text add d.pxo --in-place --name RibbonText --content "THE NEW CHAMBER" --font @heavy-wide --size 30 --tracking 40 --color '#ffffff' --at 540,138 --anchor c --anchor-to ink
pixelox export d.pxo -o C-ribbon-label.png
```

* The tails are a darker shade of the strip and sit 20 px lower, behind it.
* Curved banners: write the text with `text add`, then bend it with `text warp --preset arc`.

## C-legend

**Use for** one key shared by several charts. Script: `scripts/legend.js`.

```bash
pixelox new -o d.pxo --size 1080x300 --fill '#ffffff'
pixelox render js scripts/legend.js -o legend.png --size 1080x300 --param box=8,20,84,20 --param labels='Solid A|Leaning A|Toss-up|Leaning B|Solid B' --param colors='#1f4ea3,#9dbbe8,#c9ced6,#f1b3ad,#c8241b' --param shape=circle --param fs=24
pixelox comp place d.pxo legend.png --in-place --as Legend --at 0,0
pixelox render js scripts/legend.js -o key.png --size 1080x300 --param box=8,50,40,40 --param labels='This year|Last year' --param colors='#2563eb,#cbd2d9' --param dir=column --param shape=line --param fs=24
pixelox comp place d.pxo key.png --in-place --as Key --at 0,0
pixelox export d.pxo -o C-legend.png
```

* Order the keys as the colors appear in the chart (left to right, or top to bottom).
* Better than any legend: labels on the marks themselves. Use a legend only when direct
  labels do not fit.

## C-ranked-list

**Use for** a top 5: rank badge, name, value, a hairline between rows.

```bash
pixelox new -o d.pxo --size 1080x720 --fill '#2a1247'
pixelox render js scripts/steps.js -o ranks.png --size 1080x720 --param box=7,16,86,76 --param dir=v --param connector=none --param badges='1,2,3,4,5' --param node=72 --param labels='Striker A|Striker B|Striker C|Striker D|Striker E' --param notes='388 games for two clubs|342 games for one club|457 games for three clubs|396 games for four clubs|301 games for one club' --param colors='#f2b705' --param onecolor=1 --param ink='#ffffff' --param muted='#cdbfe6' --param fs=24
pixelox comp place d.pxo ranks.png --in-place --as Ranks --at 0,0
pixelox text add d.pxo --in-place --name Vals --content $'241\n206\n193\n174\n158' --font @heavy-wide --size 48 --leading 110 --align right --color '#f2b705' --at 93%,54% --anchor r
pixelox text add d.pxo --in-place --name Title --content "TOP FIVE SCORERS" --font @heavy-wide --size 44 --color '#ffffff' --at 7%,6% --anchor tl --anchor-to ink
pixelox export d.pxo -o C-ranked-list.png
```

* The rows are a vertical `steps.js` with `connector=none`: row pitch = box height / rows
  (here 547 / 5 = 109 px, so the value column uses `--leading 110`).
* Add C-bars beside the names when the gaps between values matter.

## C-table

**Use for** exact values in rows and columns (prices, specs, a schedule): header row, rules,
right-aligned numbers.

```bash
pixelox new -o d.pxo --size 1080x540 --fill '#ffffff'
pixelox shape rect d.pxo --in-place --name Head --rect 76,70,928x64 --fill '#16233b'
pixelox shape rect d.pxo --in-place --name Zebra1 --rect 76,204,928x70 --fill '#f1f4f7'
pixelox shape rect d.pxo --in-place --name Zebra2 --rect 76,344,928x70 --fill '#f1f4f7'
pixelox text add d.pxo --in-place --name HeadL --content "Plan" --font @neo-grotesque --weight 700 --size 26 --color '#ffffff' --at 100,102 --anchor l
pixelox text add d.pxo --in-place --name HeadM --content "Seats" --font @neo-grotesque --weight 700 --size 26 --color '#ffffff' --at 700,102 --anchor r
pixelox text add d.pxo --in-place --name HeadR --content "Per month" --font @neo-grotesque --weight 700 --size 26 --color '#ffffff' --at 980,102 --anchor r
pixelox text add d.pxo --in-place --name ColL --content $'Starter\nTeam\nBusiness\nEnterprise' --font @neo-grotesque --size 26 --leading 70 --color '#1f2933' --at 100,274 --anchor l
pixelox text add d.pxo --in-place --name ColM --content $'3\n10\n50\n250' --font @neo-grotesque --size 26 --leading 70 --align right --color '#1f2933' --at 700,274 --anchor r
pixelox text add d.pxo --in-place --name ColR --content $'$19\n$49\n$199\n$899' --font @neo-grotesque --weight 700 --size 26 --leading 70 --align right --color '#1f2933' --at 980,274 --anchor r
pixelox export d.pxo -o C-table.png
```

* One text layer per column with a fixed `--leading` equal to the row height (70 px here)
  keeps rows aligned; the column is centered on the middle of the body (`--anchor l` or `r`
  at the body's middle y).
* Text left-aligned, numbers right-aligned with the same number of decimals. 6 rows at most
  on a poster; more belongs in a document.

## C-comparison-columns

**Use for** two options side by side, point for point.

```bash
pixelox new -o d.pxo --size 1080x720 --fill '#f6f3ec'
pixelox shape rect d.pxo --in-place --name PanelA --rect 60,120,465x540 --radius 24 --fill '#ffffff'
pixelox shape rect d.pxo --in-place --name PanelB --rect 555,120,465x540 --radius 24 --fill '#16233b'
pixelox shape rect d.pxo --in-place --name ChipA --rect 180,90,225x60 --radius 30 --fill '#0d9488'
pixelox shape rect d.pxo --in-place --name ChipB --rect 675,90,225x60 --radius 30 --fill '#e4572e'
pixelox text add d.pxo --in-place --name NameA --content "RENT" --font @neo-grotesque --weight 700 --size 28 --tracking 80 --color '#ffffff' --at 292,120 --anchor c --anchor-to ink
pixelox text add d.pxo --in-place --name NameB --content "BUY" --font @neo-grotesque --weight 700 --size 28 --tracking 80 --color '#ffffff' --at 787,120 --anchor c --anchor-to ink
pixelox text add d.pxo --in-place --name BigA --content '$1,450' --font @neo-grotesque --weight 700 --size 84 --color '#16233b' --at 292,250 --anchor c --anchor-to ink
pixelox text add d.pxo --in-place --name BigB --content '$2,100' --font @neo-grotesque --weight 700 --size 84 --color '#ffffff' --at 787,250 --anchor c --anchor-to ink
pixelox text add d.pxo --in-place --name SubA --content "a month, all in" --font @neo-grotesque --size 24 --color '#52606d' --at 292,320 --anchor c
pixelox text add d.pxo --in-place --name SubB --content "a month, all in" --font @neo-grotesque --size 24 --color '#b8c4d6' --at 787,320 --anchor c
pixelox text add d.pxo --in-place --name ListA --content $'Move in a month\nNo repairs to pay\nNothing builds up' --font @neo-grotesque --size 26 --leading 64 --color '#1f2933' --at 170,480 --anchor l
pixelox text add d.pxo --in-place --name ListB --content $'Yours in 25 years\nRepairs are on you\nEquity builds up' --font @neo-grotesque --size 26 --leading 64 --color '#ffffff' --at 665,480 --anchor l
pixelox shape icon d.pxo check --in-place --name A1 --size 32 --at 130,416 --anchor c --color '#0d9488' --stroke-width 4
pixelox shape icon d.pxo check --in-place --name A2 --size 32 --at 130,480 --anchor c --color '#0d9488' --stroke-width 4
pixelox shape icon d.pxo x --in-place --name A3 --size 32 --at 130,544 --anchor c --color '#e4572e' --stroke-width 4
pixelox shape icon d.pxo check --in-place --name B1 --size 32 --at 625,416 --anchor c --color '#5eead4' --stroke-width 4
pixelox shape icon d.pxo x --in-place --name B2 --size 32 --at 625,480 --anchor c --color '#fca5a5' --stroke-width 4
pixelox shape icon d.pxo check --in-place --name B3 --size 32 --at 625,544 --anchor c --color '#5eead4' --stroke-width 4
pixelox export d.pxo -o C-comparison-columns.png
```

* Same rows in the same order on both sides; the icons sit one `--leading` apart, centered
  on the list's middle line.
* Compare like with like: the same unit, the same period, the same rounding on both sides.
* A dollar amount goes in **single** quotes (`'$1,450'`): in double quotes the shell eats `$1`.

## C-org-tree

**Use for** a small hierarchy (one lead, up to 6 reports): boxes and elbow connectors.
Approximate by hand: there is no tree generator, so keep it to two levels.

```bash
pixelox new -o d.pxo --size 1080x540 --fill '#f4f6f8'
pixelox shape path d.pxo --in-place --name Links --d 'M 540 150 L 540 250 M 200 250 L 880 250 M 200 250 L 200 330 M 540 250 L 540 330 M 880 250 L 880 330' --fill none --stroke '#9aa5b1' --stroke-width 3
pixelox shape rect d.pxo --in-place --name Top --rect 390,70,300x90 --radius 12 --fill '#f0452c'
pixelox shape rect d.pxo --in-place --name C1 --rect 70,330,260x90 --radius 12 --fill '#16233b'
pixelox shape rect d.pxo --in-place --name C2 --rect 410,330,260x90 --radius 12 --fill '#16233b'
pixelox shape rect d.pxo --in-place --name C3 --rect 750,330,260x90 --radius 12 --fill '#16233b'
pixelox text add d.pxo --in-place --name TopT --content "Director" --font @neo-grotesque --weight 700 --size 30 --color '#ffffff' --at 540,115 --anchor c
pixelox text add d.pxo --in-place --name C1T --content "Design" --font @neo-grotesque --weight 700 --size 28 --color '#ffffff' --at 200,375 --anchor c
pixelox text add d.pxo --in-place --name C2T --content "Engineering" --font @neo-grotesque --weight 700 --size 28 --color '#ffffff' --at 540,375 --anchor c
pixelox text add d.pxo --in-place --name C3T --content "Operations" --font @neo-grotesque --weight 700 --size 28 --color '#ffffff' --at 880,375 --anchor c
pixelox export d.pxo -o C-org-tree.png
```

* Child centers for N children across the margins: divide the width into N equal columns and
  use each column's middle. One path draws every connector: trunk, bar, drops.

## C-arc-label

**Use for** a name that follows a ring: a chamber's name over a seat chart, a title around a
donut or a badge.

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#122e39'
pixelox render js scripts/hemicycle.js -o seats.png --size 1080x1080 --param cx=50 --param cy=58 --param r=40 --param seats=47,4,39 --param colors='#c8322b,#aeb4b3,#1f77b8' --param inner=0.42
pixelox comp place d.pxo seats.png --in-place --as Seats --at 0,0
pixelox text add d.pxo --in-place --name ArcTitle --content "UPPER HOUSE" --font @geometric-sans --weight 800 --size 44 --tracking 20 --color '#e9e3a6' --align center
pixelox text on-path d.pxo --in-place --layer ArcTitle --d 'M 48 626 A 492 492 0 0 1 1032 626' --offset 50%
pixelox text add d.pxo --in-place --name ArcInfo --content "90 seats, half elected every three years" --font @geometric-sans --size 26 --color '#ffffff' --align center
pixelox text on-path d.pxo --in-place --layer ArcInfo --d 'M 88 626 A 452 452 0 0 1 992 626' --offset 50%
pixelox export d.pxo -o C-arc-label.png
```

* The path is a half circle on the chart's own center (cx, cy), with a radius R a little
  larger than the chart's: `M cx-R cy A R R 0 0 1 cx+R cy` runs over the top, left to right.
  `--align center` on the text and `--offset 50%` on the path center the words on the top.
* Under a ring, use `M cx-R cy A R R 0 0 0 cx+R cy` (the last flag before the end point is
  0): the words then read left to right along the bottom, upright, just inside the path.
* Two lines: two text layers on two radii about 1.1 x the font size apart.
* Keep arc labels to 4 words; long sentences on a curve are hard to read.

## C-source-line

**Use for** the foot of every infographic that shows data: where the numbers come from, what
they cover, and any footnote marks used above.

```bash
pixelox new -o d.pxo --size 1080x260 --fill '#f6f3ec'
pixelox shape line d.pxo --in-place --name Rule --from 76,70 --to 1004,70 --stroke '#cbd2d9' --stroke-width 2
pixelox text add d.pxo --in-place --name Note --content "* Figures for 2025 are estimates. Shares may not add to 100 because of rounding." --font @neo-grotesque --size 18 --color '#7b8794' --at 76,100 --anchor tl
pixelox text add d.pxo --in-place --name Source --content "Source: Example Statistics Office, annual survey, 2,000 adults, May 2025" --font @neo-grotesque --size 18 --color '#52606d' --at 76,134 --anchor tl
pixelox text add d.pxo --in-place --name Credit --content "Design: Your Studio" --font @neo-grotesque --weight 700 --size 18 --color '#52606d' --at 1004,190 --anchor r
pixelox export d.pxo -o C-source-line.png
```

* Size 15 to 18 px at 1080 wide, in the muted text color. Never leave it out: a chart
  without a source is a decoration.
* When the data is the user's own, write "Source: company data, 2025". When you invented
  placeholder numbers, write "Example data" in plain sight.
