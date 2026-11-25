# Infographic description template

One file per infographic: `references/infographics/<id>-<slug>.md`
(`i038-rocket-four-stage-callouts.md`). The slug is 2 to 5 lowercase words that describe the
design, never a brand, person, event, publication, or the source file's name. Copy the
skeleton at the end of this file and fill every section. Finished examples:
[i003](i003-dark-two-chamber-results.md), [i035](i035-banded-long-explainer.md),
[i038](i038-rocket-four-stage-callouts.md).

The reader is an agent that has **never seen the infographic**, often a small model, and must
build an original piece in the same design language with the pixelox CLI from data it is
given. Write numbers, not adjectives: "font size 62 px, bold", not "big"; `#b3261c`, not
"deep red"; "5 bars falling from about 40% to 8%", not "some bars".

## Content rules (not optional)

1. **Describe the design, not the brand or the event.** Replace every real brand, publisher,
   logo, product, organization, party, team, competition, place-specific event, and real
   person (candidates, players, authors, designers) with a neutral placeholder or a generic
   noun: `[PUBLISHER MARK]`, "Party A", "Striker A", "a two-chamber parliament", "a streaming
   service". Never say who a real person is; describe pose, framing, clothing, light.
2. **Do not copy the data.** Never write the original's figures, names, dates, or rankings.
   Describe the data's **shape**: what kind of data, how many categories or series, the range
   and pattern of the values ("5 categories, falling from about 40% to 8%"; "two blocs within
   2% of each other and a 1% remainder"). In the recipe use **invented placeholder data** with
   the same shape, and mark it as example data in the recipe's source line.
3. **Do not quote the copy.** Only generic words may appear as they are ("TOTAL", "STEP 01",
   "Source:", month names). For everything else give the pattern: "title of 5 to 7 words on
   two lines", "4 notes of 8 to 12 words". Placeholder text (lorem ipsum) is described as its
   pattern and length, never transcribed. In the recipe write **new, plain copy** of the same
   shape, about an invented, neutral subject.
4. **Political and electoral graphics: form only.** Describe the chart form and the color
   convention neutrally ("two-party red and blue convention, US"; "gray for vacant seats").
   No opinions, no claims about outcomes, no real party, candidate, or office-holder names:
   "Party A", "Party B", "Bloc A", "the lower house".
5. **Never mention the source file name, a URL, a watermark, a stock-site or template-site
   mark, or a credit line.** Ignore watermarks and "made with" badges as if they were not
   there. If the image is a mockup or shows several sheets, describe the flat design of one
   sheet and say in one line how it was presented.
6. **Reusable language, not a clone.** Say why it works and what to vary. The recipe builds an
   original piece in this style.

## How to measure

* Origin is the top-left corner. Positions and sizes are **percent of the canvas width (W) or
  height (H)**: x and widths in %W, y and heights in %H. Give boxes as `x,y,w x h`
  (`7%,24%,86% x 30%`). Radii of circles are in %W.
* **Type size is the font size in px on the recipe canvas** (1080 px wide; landscape 1600).
  Measure a capital letter's height in the viewing copy, then
  font px = cap height x (canvas width / viewing-copy width) / 0.72. Long infographics have
  very different heights, so sizes are never given in % of the height, and recipes never use
  `--size N%` (that is % of the height).
* Colors are sampled, not guessed (`lab/eyedrop.py view/i008.png 300,420 --k 6`). Round to a
  clean hex. Area ratios are estimates to the nearest 5%.
* For fine print, crop the original into the scratchpad and read that. Never write into the
  originals' folder.

## How to write commands

The repo's contract test parses every documented command, and your recipe will be run.

* Put commands in one fenced `bash` block, one command per line, each line starting with
  `pixelox ` (no variables, no `cd`, no pipes, no loops). A long line may be split with a
  trailing `\`.
* In running text, name commands **without** the `pixelox ` prefix (`shape icon`,
  `comp place`). An inline code span that starts with `pixelox ` must be a complete, valid
  command.
* **Charts come from the generator scripts**, never from hand-computed shapes: cite the chart
  ID from [../charts.md](../charts.md), reuse its commands, change the parameters. Render each
  chart at the canvas size with `box=` (or `cx= cy= r=`) and place it at `--at 0,0`. The map
  scripts (`usmap.js`, `geo.js`) are run with `render d3`, all others with `render js`. Icons,
  leaders, and labels that hang on a chart use the anchors the script logs
  (`lab/anchors.py scripts/steps.js --size 1080x1350 --param ...` prints them).
* Sizes: `--size 62` (px). Quote every color (`'#ff5a1f'`). A value that begins with a minus
  needs `=` (`--param rotate=-90` is fine; `--angle=-52`). Dollar amounts go in single quotes
  (`'$1,450'`). Line breaks: `--content $'line one\nline two'`. Apostrophes are the
  typographic `’` (`$'the team’s plan'`), never an escaped `\'`: the contract test cannot
  parse it, and the curly one is better typography.
* Inside a text string never put a space before a `#`, and no `<placeholders>`, `$(...)`, or
  `${...}` anywhere.
* The recipe starts with `new`, works in one `.pxo` with `--in-place`, and ends with one
  `export ... -o <name>.png`. Canvas: **1080 px wide** at the infographic's aspect ratio
  (square 1080x1080, 4:5 1080x1350, A-series 1080x1528, 2:3 1080x1620; long formats 1080 x
  the ratio, rounded to 10); landscape **1600 px wide** (3:2 1600x1066, 16:9 1600x900).
* Fonts: name a font **class** in `text add` (`--font @geometric-sans`, see
  [../typography.md](../typography.md)); in script `font=` params give the class's family
  stack from the same file. Name the ideal free look-alike in the text table.
* Images the user must supply are named `subject.png` and `subject2.png` (cutouts with
  transparency), `photo.png` and `photo2.png` (opaque photos), `product.png` (cutout),
  `illustration.png` (drawn art with transparency), `logo.png`. Mark each one "your image
  here" in the recipe notes. Where an icon or a `scripts/object.js` object can stand in for an
  illustration, use it and say what the original showed.
* Recipes are 20 to 90 commands. A long infographic builds every module; when cards repeat
  (six identical stat cards), build them all: a half-built sheet teaches nothing.
* No real brand, title, party, or person's name in any recipe string.

## Sections (all required, in this order)

**Front matter.** YAML between `---` lines, used to build the index:

| Key | Value |
| --- | --- |
| `id` | `i038` |
| `title` | 3 to 8 words, the design in plain words |
| `kind` | 1 or 2 of: statistical, results, process, timeline, cycle, comparison, hierarchy, geographic, list, anatomy, dashboard, explainer |
| `topic` | 1 to 3 of: politics, business, marketing, finance, technology, education, health, food, sports, science, environment, media, society, generic |
| `mood` | 2 to 4 of: authoritative, bold, calm, clean, corporate, dark, editorial, energetic, friendly, futuristic, minimal, playful, serious, technical, vibrant, warm, cool, retro |
| `style` | 1 to 3 of: flat, editorial, ui-cards, gradient, paper-cut, line-icon, illustrated, photo-led, neon, hand-drawn, 3d |
| `format` | `long` (taller than 1:2), `portrait`, `square`, `landscape`, `banner` (wider than 2:1) |
| `structure` | one structure ID from [../index.md](../index.md) (`S-long-bands`, `S-chart-card`, ...) |
| `data` | 1 to 3 of: single-stat, part-to-whole, comparison, ranking, change-over-time, counted-units, geography, process, cycle, hierarchy, relationship, text-only |
| `charts` | the chart IDs from charts.md that the recipe uses, most important first |
| `palette` | 3 to 7 hexes, the ground first, then by area |
| `needs` | what the user must supply besides data and copy: `nothing`, `photo`, `subject-cutout`, `illustration`, `logo`, `map-image`, or a list |

**1. Concept.** Two to four sentences: what it is, the one message a reader takes away, and
the single idea that makes it work.

**2. Data story.** What the data is (kind, unit), how many categories, series, steps, or
seats, the range and pattern of the values, and which relationship is shown (see `data`
above). Then the **placeholder data** the recipe uses, as a small table or list. When the
piece has no numbers (a process, a concept), give the count and length of the text items.

**3. Information architecture.** The modules in reading order, numbered: what each one
answers, and how the reader is led from one to the next (size, numbering, arrows, bands).

**4. Format and grid.** Aspect ratio and recipe canvas; margins in %W; columns and gutters;
each module's box as `x,y,w x h`; the alignment system (centered axis, or left edges).

**5. Layer stack.** Numbered, back to front. Every visible thing appears once, with its box
or center and radius, and its color or treatment.

**6. Palette.** Table: hex, role, area %, **what it encodes** (a category, "larger = darker",
emphasis, or nothing: neutral). Then one line on the scheme (categorical, sequential,
diverging; which color is the single accent).

**7. Text elements.** Table with one row per text element:
role | content pattern | case | font class (free look-alike / local) | weight | size px |
tracking | leading | align | color | position (x %W, y %H of the anchor).
Then one line on the type system (how many families, the size ratio between levels).

**8. Charts and diagrams.** For every chart: its ID, the script and the parameters that
matter, its box (or center and radius), axis and gridlines (present or not, range, ticks),
value labels, legend or direct labels, and what was simplified compared with the original.
For a **map**, give the map spec: region set (US states, world countries, a zoomed region),
projection (Albers USA, Equal Earth, Mercator, a globe and its center), what is encoded
(choropleth: the classes and their hexes; bubbles: what their area means; highlighted
regions), the legend, the labels, and insets. Build it with the outline-map scripts
(C-us-choropleth, C-us-bubble-map, C-world-map, C-globe; `render d3`, pixelox 0.6.0 or later)
unless the original itself is a tile or hex cartogram (then C-tile-map or C-hex-map), and
name the stylized fallback for older builds in one line. For a **3D** element (isometric
blocks, cylinders, a layered stack), give the object, the view angle, the light direction,
and the face colors (top, left, right), say that depth encodes nothing, and build it with
C-3d-bars or a flat equivalent.

**9. Icons and illustration.** Every icon with its Lucide name (`shape icons --search WORD`),
size, color, and position; drawn objects (`scripts/object.js` kind and bands); photos and
illustrations in neutral terms, with crop, scale, and treatment.

**10. Effects and finish.** Shadows, gradients, textures, rounded corners, outlines: where,
how strong, and the command that makes each. Say "none: flat" when there are none.

**11. Hierarchy and eye path.** What is read first, second, third, and why (size, contrast,
position, numbering). The size ratio between the type levels.

**12. Build recipe.** One `bash` block that runs start to finish, plus notes: which inputs
are "your image here", where the anchors came from, and what to change first when reusing it
with other data (the `values=` and `labels=` params, the title).

**13. Variations and pairings.** 3 to 5 concrete ways to vary it: other data shapes it suits,
palette swaps, other formats, modules to add or drop, and which other chart IDs or structures
it combines with.

**14. Pitfalls.** 3 to 6 specific ways a copy of this design goes wrong, each with the fix.
At least one must be about **data integrity** (a truncated axis, areas that do not match
values, a legend that does not match the colors, shares that do not add up, a missing unit or
source).

## Verify before you hand in

1. `lab/check_info.py <your file>` passes: front matter, sections, chart and structure IDs,
   and the recipe runs with every command exiting 0 in an empty folder that holds only
   `scripts/` and the stand-in images. Read every LINT line.
2. You looked at the export next to the original (`lab/sheet.sh`): same modules in the same
   order, same hierarchy, same palette, same kind of type, charts of the same form. Fix the
   recipe and the numbers in the description until it reads as the same design language.
3. No brand, publication, party, person, real figure, verbatim copy, file name, or URL appears
   anywhere in the file.
4. Every number in the recipe's charts is placeholder data, and the recipe's source line says
   "Example data".

## Skeleton

````markdown
---
id: iNNN
title: Short plain-words title
kind: [statistical]
topic: [business]
mood: [clean, corporate]
style: [flat]
format: portrait
structure: S-chart-card
data: [comparison]
charts: [C-columns, C-title-block, C-source-line]
palette: ['#ffffff', '#1f2933', '#e4572e']
needs: nothing
---

# iNNN: Short plain-words title

## 1. Concept

## 2. Data story

## 3. Information architecture

1. ...

## 4. Format and grid

## 5. Layer stack

1. ...

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |

## 8. Charts and diagrams

## 9. Icons and illustration

## 10. Effects and finish

## 11. Hierarchy and eye path

## 12. Build recipe

```bash
pixelox new -o info.pxo --size 1080x1350 --fill '#ffffff'
pixelox export info.pxo -o info.png
```

## 13. Variations and pairings

## 14. Pitfalls
````
