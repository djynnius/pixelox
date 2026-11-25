---
name: pixelox-infographics
description: 'Design infographics and data graphics from a brief or raw data with the pixelox CLI, working like an information designer: find the one message, pick the chart that fits the data, borrow structure, palette, and illustration ideas from a library of 55 studied infographics, build charts with data-driven generator scripts (never hand-computed geometry), look at the result, and fix it. Use this whenever someone asks for an infographic, chart, graph, data visualization, dashboard, KPI or stat card, report one-pager, timeline, roadmap, process or how-it-works diagram, flowchart, cycle, comparison or versus graphic, Venn diagram, pyramid or funnel, election or results graphic, seat chart, map of values, sports stat sheet, or says "visualize this data" or "turn these numbers into a graphic", including vague requests with only numbers or only a topic. Not for posters, flyers, and ads without data (pixelox-design), photo retouching (pixelox-retouch), or plain edits such as crop and convert (pixelox).'
---

# Infographics with pixelox

You are the information designer and the production artist. An infographic is an argument
made of numbers and pictures: it says one thing, proves it with data shown honestly, and
can be read in the order you planned. A page that merely contains every figure is not done.

Run pixelox with `--json` and read `result`, `warnings`, and `error.hint`. Check flags with
`pixelox docs <command>`. Work in one `.pxo` with `--in-place`; export at the end. Needs
pixelox 0.5.1 or later; **real outline maps and globes need 0.6.0 or later** (`pixelox --version`;
if `pixelox docs atlas` prints a list of map data, they are available; ask the user before
`pixelox update`). On an older build use the tile and dot maps instead.

## What is in this skill

| File | Use it for |
| --- | --- |
| `references/charts.md` | **the chart library**: 73 tested recipes by ID (`C-bars`, `C-donut`, `C-hemicycle`, `C-us-choropleth`, `C-process-spine`, `C-rocket`, `C-stat-block` ...), a "which chart for which data" table, limits. Search for the heading; do not read it all |
| `references/index.md` | which studied infographics to start from for which brief or data shape; worked combinations; the 22 structure archetypes (`S-chart-card`, `S-long-bands`, ...) |
| `references/infographics/iNNN-*.md` | 55 studied infographics: data story, modules, grid in percent, palette with what each color encodes, type table, chart parameters, a tested build recipe, pitfalls |
| `references/palettes.md` | color roles (ground, ink, series, accent), categorical, sequential, and diverging rules, 18 named palettes, ramps |
| `references/typography.md` | size scale in px, font classes for infographics, family stacks for chart scripts, how to set numbers |
| `references/icons.md` | concept to icon name (about 200 checked Lucide names), sizes, line weights |
| `scripts/*.js` | chart generators for `render js`: bars, line, donut, radial, bars3d, hemicycle, waffle, tilemap, worldmap, steps, cycle, venn, pyramid, object, legend; for `render d3`: usmap, geo (real maps, globe); backdrops: rays, pattern, blobs; standin |

Copy `scripts/` next to your work (or use its full path). Every script documents its
parameters in its first lines.

## The one rule about charts

**Never compute chart geometry by hand.** No bar heights, arc angles, seat positions, or
label coordinates from mental arithmetic: they will be wrong. Give the data to a generator:

```bash
pixelox new -o d.pxo --size 1080x1350 --fill '#f6f3ec'
pixelox render js scripts/bars.js -o bars.png --size 1080x1350 --param box=7,24,86,56 --param values=42,31,18,9 --param labels='North,South,East,West' --param suffix=% --param highlight=1
pixelox comp place d.pxo bars.png --in-place --as Bars --at 0,0 --link
```

* Render at the **canvas size** and say where the chart goes with `box=x,y,w,h` in percent
  of the canvas (or `cx= cy= r=` for rings). Place it at `--at 0,0`. Nothing to add up.
* The script draws its own labels at `fs` px (default 2% of the width: every chart on the
  canvas matches). Colors: `colors=`, `ink=`, `muted=`, `grid=`. Fonts: `font='Family, ...'`
  (a family stack from `typography.md`; classes such as `@condensed` work only in `text add`).
* Run it with `--json`: `result.logs[0]` holds `anchors` (id, x, y in canvas px). Put icons,
  callouts, and your own labels there: `shape icon ... --at X,Y --anchor c`.
* To change a chart: render again to the same file with `--overwrite`, then
  `pixelox comp update-links d.pxo --in-place`.
* The two map scripts are run with `render d3` instead of `render js`
  (`pixelox render d3 scripts/usmap.js -o map.png --size 1080x1350 --param box=5,28,90,46 --param values='CA:31,TX:44'`);
  everything else about them is the same.
* Titles, decks, notes, sources, stat numbers, cards, bands, and icons are native commands
  (`text add`, `shape rect`, `shape icon`): the Furniture recipes in `charts.md`.

## Process

Follow the steps in order. Do not skip 1, 4, 6, or 7.

### 1. Find the message

Write one sentence a reader should be able to repeat: "North now sells more than the other
three regions together." That sentence, shortened, is the title. Then list: the **audience**
and where it will be seen (phone feed, slide, print), the **format**, the **data** you have,
and the **facts** that must appear (unit, period, source).

**When the brief is vague or is only data**, do not stall and do not ask a list of questions:

* Only numbers: look for the largest value, the largest change, or the gap between two
  values; that is the message. Say which you chose.
* Only a topic ("an infographic about sleep"): do not invent statistics. Build a process,
  list, or comparison from common knowledge, or use clearly marked example data ("Example
  data" in the source line) and say the user must replace it.
* Format: one chart is a square or 4:5 card; a "one-pager" or report page is portrait A
  (1080x1528) or at most twice as tall as wide; a story with 3 or more chapters is a long page
  (1080 wide, 2 to 3.5 times as tall); a slide is 16:9.
* Missing labels (which quarter, which currency, whose data): choose the plain reading (Q1 to
  Q4, the unit as given), write it on the page where a reader needs it (deck, axis, source
  line: "Source: figures supplied by the team"), and list every assumption when you deliver.
* Ask only for what would make the result wrong if guessed: the source of the data, a logo,
  real names. Build a first version anyway.

### 2. Shape the data, choose the charts

Name each dataset's relationship and take the chart from the table at the top of
`charts.md`: one number (C-stat-block, C-progress-ring, C-icon-array); parts of a whole
(C-donut, C-waffle, C-percent-bar); amounts across categories (C-bars, C-columns); change
over time (C-line, C-columns, C-combo); counted seats or people (C-hemicycle, C-seat-grid);
place (C-us-choropleth, C-world-map; any build: C-tile-map, C-dot-map); steps (C-process-row,
C-process-spine, C-snake-steps, C-rocket); a loop (C-cycle); dates (C-timeline); levels
(C-pyramid, C-funnel); overlap (C-venn); two groups side by side (C-butterfly-bars).

Before charting: sort categories by value unless their order means something; fold
categories past the limit into "Other"; round to 2 or 3 significant digits; compute shares
yourself only with a calculator-grade check (they must add to 100). Limits: donut 5 slices,
columns 8, bars 12, lines 4 series, stacked 3 to 4 parts, row of steps 5, legend 6 entries.

### 3. Choose references

Open `references/index.md`. Take candidates from section 1 (by brief) or 2 (by data shape),
check section 3 for a ready combination, and pick 2 to 4 infographics: the **structure**
from one, the **palette and type** from another, one **chart or illustration idea** from a
third. Read those files. Write one line on what you take from each.

### 4. Plan before any command

Write the plan as text:

* Canvas, margins (6 to 8% of the width), and the structure archetype's zones.
* Modules in reading order, each with its question, its chart ID, and its box in percent.
  One idea per module. The most important module is the largest and comes first. When the
  brief names two subjects (a process and a map), the title covers both or picks one; lead
  with the one the title is about.
* Palette by role (`palettes.md`): ground, ink, muted, grid, series colors in a fixed order,
  one accent. Write down what each color **means**; it keeps that meaning in every module.
* Type (`typography.md`): one or two font classes, the script `font=` stack that matches,
  and sizes: title 54 to 72 px, section titles 28 to 40, stat numbers 44 to 72 (a hero
  number 90 to 200), labels and body 20 to 26, source 15 to 18 (px at 1080 wide; never
  `--size N%`, which is percent of the height).
* The exact title (the finding), deck (what is measured, unit, period), and source line.

### 5. Build

Back to front: ground, panels and bands, charts, icons and objects, titles and numbers,
notes, source line.

* Name every layer (`--name`, `--as`). Positions in percent or px; sizes in px.
* One `render js` and one `comp place` per chart. Keep `fs`, `font`, `ink`, `muted`, and the
  order of `colors` identical across charts.
* Read the anchors instead of guessing where a node or slice is, and read them again after
  any change to that chart's `box`, `fs`, or data: they move.
* Size each chart's `box` to fill its card or band (padding about 5% of the card): a small
  chart in a half-empty card looks unfinished.
* Fit a one-line title with `--fit-width`; place text by its letters with `--anchor-to ink`.
* Keep every command in one shell script (`build.sh`) so the piece can be rebuilt. To run it
  again, start in an empty folder, or add `--overwrite` to your own intermediate files (the
  `new` document, chart PNGs, previews).

**Things that bite:**

* Quote every value with `#`, `|`, `;`, or spaces. Lists use commas; series use `;`; titles
  and notes use `|` between items. A dollar amount needs single quotes (`'$1,450'`).
* Line breaks in `text add` need `$'A\nB'`. One text layer has one weight and one color: a
  bold line next to a regular line is two layers.
* A `%` rect is % of width by % of height: use px for circles and squares.
* New layers land on top; add `--below NAME` or `--above NAME` to place them elsewhere.
* A chart rendered smaller than the canvas, or placed anywhere but `0,0`, is in the wrong
  place: its `box` already positions it. With `cx= cy= r=` only the ring is placed: outside
  labels (`labelpos=out`) reach about 0.5 x r further on each side, so leave that room or use
  `labelpos=legend`.
* Icons above 64 px need `--stroke-width` (about size / 16 to size / 20) or they look thin.
* Maps: US states and world countries come from `scripts/usmap.js` and `scripts/geo.js`
  (`render d3`, 0.6.0 or later; states by postal code, countries by ISO three-letter code).
  Other regions: a tile grid of your own (`tilemap.js cells=`) or the user's map image. Never
  draw an outline freehand. A chart file must not have the same name as your final export.

### 6. Look at it

```bash
pixelox export d.pxo -o preview.png --size 1080x --overwrite
```

Open `preview.png` (your image viewer, or the MCP `pixelox_preview` tool). On a long page also
look at each module at full size: `pixelox export d.pxo -o zoom.png --region 0,1800,1080x900 --overwrite`.
If you cannot view images, check by numbers: `pixelox layer list d.pxo --json` gives every
layer's `bounds` (text inside the margins, no `overflow`, no two text bounds intersecting),
and each script logs its `box`.

Critique in this order:

1. **Message.** Cover everything but the title and the hero chart: do they alone say the one
   sentence from step 1?
2. **Truth.** Every number matches the data; shares add to 100; seat and unit counts match
   the logged `total`; units, period, and source are on the page.
3. **Reading order.** Can you number the modules 1, 2, 3 without hesitating? Is the first
   one the largest?
4. **Labels.** Every mark is labeled or keyed; no label collides, is cut, or is under 18 px;
   no legend where direct labels fit.
5. **Color.** Each color means one thing; one accent; gray for "other"; text contrast of 50
   lightness points.
6. **Type.** Four or five sizes; numbers larger than their captions; all text horizontal.
7. **Alignment and space.** Module edges share lines; equal gutters; margins equal on both
   sides; at least a fifth of the page is calm, but no card is more than a third empty.
8. **Consistency.** One icon style, one corner radius, one line weight, one label size.

### 7. Iterate

Fix the biggest problem first (usually the message or a crowded module), re-export, look
again. One full revision pass at least. If a module still looks crowded, cut content: an
infographic gets better by removing.

### 8. Deliver

```bash
pixelox export d.pxo -o infographic.png
pixelox verify infographic.png.pixelox.json --json
```

Report the output path, the message, the data source as given, every assumption and any
example data still on the page, fonts substituted, and what the user should check (names,
figures). Keep the `.pxo` and the chart `--param` lines: they are the editable source.

## Data integrity (not negotiable)

* **Bars and areas start at zero.** The scripts enforce it for bars. A line may use `min=`
  only when the deck says so.
* **Area means amount.** Bubbles and icons scale by area, never by width or height. Picture
  gauges fill by height: always print the number.
* **No perspective or tilt on data.** Depth adds nothing and bends sizes. The one 3D form
  allowed is C-3d-bars (parallel view, heights stay true, every value printed), once, as a hero.
* **One scale per chart.** Never two unrelated measures on one axis; a second axis only as
  in C-combo, with both units named.
* **Counts are exact.** A seat chart draws exactly the seats; a 100-square waffle adds to 100.
* **Show the base:** the unit, the period, the sample ("2,000 adults, May 2025"), the source.
* **Do not invent data.** Missing numbers are asked for, or marked "Example data".
* **Same thing, same color and same order** in every module and legend.
* **Neutral words.** Titles state what the data shows, not what to feel about it; for
  elections and other contested topics describe counts and shares, never predict or judge.

## Amateur tells, and the fix

| Tell | Fix |
| --- | --- |
| A title that names a topic ("Sales 2025") | State the finding ("North leads by eleven points") |
| Pie with 9 slices, legend off to the side | 5 slices and "Other", labels on the slices; or C-bars |
| Rainbow colors without meaning | One hue plus an accent; or one color per category, kept everywhere |
| Every number the same size | One hero number 4 to 8 x the body; stat numbers 2 to 3 x |
| Charts drawn from rectangles "by eye" | Generator scripts; values in `--param` |
| Tiny labels, rotated axis text | `fs=22` or more; horizontal bars for long names |
| Six fonts, clip-art icons in mixed styles | Two families; one Lucide treatment (`icons.md`) |
| Modules floating at random | A structure archetype; shared edges; equal gutters |
| Decoration louder than data (gradients, shadows, textures) | Flat marks; effects only on grounds and cards |
| No source, no unit | C-source-line on every piece with data |
| A wall of text beside each chart | One caption of 2 or 3 lines; the rest goes in the article |

## With the other pixelox skills

This skill is complete by itself. If **pixelox-design** is installed, its `techniques.md`
adds background and type effects (`T-paper-grain`, `T-gradient-mesh`, `T-long-shadow`,
`T-cutout-drop-shadow`) and its poster library helps when the piece is more poster than
chart; use them on grounds, titles, and photos, never on data marks. Cutting a person out of
a photo for a stat poster: `detect subject`, then `mask refine` (ask before the model
download), as in the pixelox skill.

## Keep it original and honest

The reference descriptions teach design language; they are not templates to clone.

* Combine at least two references; change the data, the copy, and the palette or the type.
* Never reuse a reference's wording or its example numbers. Never use a real brand, party,
  team, or publisher mark unless the user supplies it; never imitate a real publication's
  signature look closely enough to be mistaken for it.
* Real people (candidates, players, executives) appear only with a photo and facts the user
  supplied. Do not attach numbers to a real name from memory.
* If the user asks for "the same as" a known infographic, build one in that spirit with
  their data and say what you changed.
