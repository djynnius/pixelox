# Type for infographics

Infographic type has two jobs: carry numbers, and stay readable at small sizes next to
charts. This file gives the size scale, the font classes that suit the job, the family
stacks to pass to chart scripts, and the rules for setting numbers. (The pixelox-design
skill's `references/typography.md` lists all 28 font classes with look-alikes; you do not
need it for an infographic.)

## Size scale

Font sizes in px on a **1080 px wide** canvas (multiply by `width / 1080` elsewhere; a 1600
px landscape canvas uses the same numbers x 1.2 to 1.5). Infographic heights vary from 1 : 1
to 1 : 4, so sizes are tied to the width, and recipes use `--size 62`, never `--size 5%`
(`%` means percent of the canvas **height**).

| Role | Size px | Weight | Notes |
| --- | --- | --- | --- |
| Title | 54 to 72 (long formats up to 100, type-led up to 140) | 700 to 900 | 1 or 2 lines; fit a one-line title with `--fit-width` |
| Deck (subtitle) | 26 to 34 | 400 | says what is measured, where, when |
| Section or band title | 28 to 40 | 700 | caps with tracking 40 to 80, or title case |
| Hero number | 90 to 200 | 700 to 900 | the one number of the piece |
| Stat number | 44 to 72 | 700 to 800 | numbers in a stat row or a card |
| Chart labels (`fs=`) | 20 to 26 | 400, numbers 700 | never under 18 |
| Body, captions, notes | 20 to 26 | 400 | 2 or 3 lines per block, 25 to 45 characters per line |
| Callout title | 28 to 42 | 600 to 700 | |
| Source line, footnotes | 15 to 18 | 400 | muted color |

Ratios that hold across the studied designs: title : section : body : source is about
**3 : 1.4 : 1 : 0.75**; a stat number is 2 to 3 x the body; a hero number 4 to 8 x. Use four
or five sizes in all. If two sizes differ by less than 1.25 x, merge them.

## Font classes that suit infographics

Name a class in `text add` (`--font @geometric-sans`); it resolves to the best installed
face on any machine (`pixelox fonts classes` shows what each resolves to here). **Chart
scripts cannot take a class**: give them the family stack in the last column as
`--param font='...'` so the chart labels match the rest.

| Class | Look | Free look-alikes | Use for | Stack for `font=` |
| --- | --- | --- | --- | --- |
| `@neo-grotesque` | neutral, newsroom, Swiss | Inter, Roboto, Archivo | everything in data-heavy pieces; the default | (the scripts' default) |
| `@geometric-sans` | round, friendly, modern | Poppins, Montserrat, Jost | explainers, reports, tech and start-up pieces | `'Avenir Next, Poppins, Montserrat, Century Gothic, Arial, sans-serif'` |
| `@thin-sans` | light, airy, wide | Montserrat 200, Raleway 200, Jost 200 | large light titles and big numbers on calm pages; never under 40 px | `'Avenir Next, Montserrat, Raleway, Helvetica Neue, sans-serif'` with `weight=300` |
| `@condensed` | tall, economical | Oswald, Barlow Condensed | callout titles, band titles, long labels in narrow columns | `'Avenir Next Condensed, Oswald, Barlow Condensed, Arial Narrow, sans-serif'` |
| `@condensed-heavy` | tall, loud | Anton, Bebas Neue, League Gothic | poster titles, sports sheets, hero numbers | `'Impact, Anton, Bebas Neue, Arial Narrow, sans-serif'` with `numweight=400` |
| `@heavy-wide` | chunky | Archivo Black, Inter 900 | stat rows, short caps titles | `'Arial Black, Archivo Black, Avenir Next, sans-serif'` |
| `@rounded-heavy` | soft, playful | Fredoka, Nunito 900 | food, kids, health pieces | `'Arial Rounded MT Bold, Nunito, Fredoka, sans-serif'` |
| `@slab` | sturdy, almanac | Roboto Slab, Zilla Slab | map numbers, retro and civic sheets | `'Rockwell, Roboto Slab, Zilla Slab, Georgia, serif'` |
| `@oldstyle` | bookish, editorial | Lora, EB Garamond | editorial titles and numbers, quotes | `'Georgia, Baskerville, Lora, serif'` |
| `@didone` | high contrast, magazine | Playfair Display, Bodoni Moda | one elegant title; never small text | `'Didot, Playfair Display, Bodoni 72, Georgia, serif'` |
| `@mono` | technical | JetBrains Mono, Space Mono | code, tickers, tabular figures | `'Menlo, JetBrains Mono, Consolas, monospace'` |
| `@script-formal`, `@brush`, `@marker` | handwritten | Great Vibes, Pacifico, Caveat | one accent word in a title; never data | not for charts |

A stack lists the local face first, then free families, then a generic name, so the script
finds something on every machine. If a family the design depends on is missing, ask the user
before downloading it (`pixelox fonts get Poppins -y` needs their consent).

## Pairings that work

| Title | Body and labels | Reads as | Seen in |
| --- | --- | --- | --- |
| `@neo-grotesque` 700 | `@neo-grotesque` 400 | newsroom chart, report | chart cards, dense analysis sheets |
| `@geometric-sans` 800 | `@geometric-sans` 400 | friendly explainer, corporate | banded explainers, results sheets, dashboards |
| `@condensed` 600 caps | `@neo-grotesque` 400 | lively, modern | metaphor objects with callouts, process sheets |
| `@condensed-heavy` caps | `@neo-grotesque` 400 | loud, sporty | stat posters, step sheets on one bold color |
| `@oldstyle` 700 | `@neo-grotesque` 400 | editorial, serious | photo-led statistics, long reads |
| `@slab` 700 | `@neo-grotesque` 400 or `@slab` 400 | almanac, civic, retro | maps with value badges |
| `@script-formal` (one word) + `@condensed-heavy` | `@oldstyle` or `@neo-grotesque` | playful, illustrated | illustrated long explainers |

Weights that bite: a light weight (200 to 300) turns hairline-thin at 50 px and above on
some machines, so check the preview and step up to 400 if the strokes vanish; `@didone`
resolves to its bold face, so pass `--weight 400` for a regular didone; a class without an
italic (`@marker`, `@didone` bold) gets a slanted stand-in (warning PXW4002): use `@oldstyle`
when you need a true bold italic serif. Humanist sans faces (Gill Sans, Open Sans) and
rounded technical faces (DIN Rounded) have no class of their own: `@neo-grotesque` and
`@rounded-heavy` are the nearest.

Two families at most. The second family, if any, is for titles only; all numbers on charts
and all body text share one family.

## Setting numbers

* **The number is the picture.** In a stat block the number is the largest thing, in the
  heaviest weight; its unit is the same size or one step smaller ("2.9 billion", "62%"),
  never a tiny superscript.
* **Round for the reader:** 2 significant digits in a title or hero ("about 3 billion",
  "62%"), 3 in a chart label (62.4%), exact figures only in a table. Use the same rounding for
  every number in one chart.
* **Units and base:** say the unit once per chart, in the deck or the axis ("millions",
  "% of adults"). Chart scripts: `suffix=%`, `prefix=$`, `compact=1` (1.2K, 3.4M).
* **Align numbers** on the right edge in tables and ranked lists (`--align right` with
  `--anchor r`); give every row the same number of decimals.
* **Colored numbers mean something.** A number in a category's color is read as that
  category's value. Numbers that belong to no category are set in the text color.
* **Thousands separators** in anything over 9,999; a narrow no-break space or a comma, one
  style per piece.

## Rules

* **Titles state the finding** ("North leads by eleven points"), decks state the measure
  ("Share of orders by region, 2025"). Section titles name the question the module answers.
* **All text horizontal.** No rotated axis titles, no text on steep diagonals. Long category
  names: horizontal bars (C-bars), not tilted labels. Curved text only for a short name on a
  ring (C-arc-label).
* **Caps for labels up to 4 words**, with tracking 40 to 100; sentence case for anything
  longer. Never a script or a didone in caps.
* **Alignment follows the structure:** centered inside centered columns and rings; flush
  left in chart cards and dashboards. One system per module.
* **Contrast:** text on a colored ground needs a lightness difference of at least 50 points;
  on mid-tones use white type in bold. Check with
  `pixelox analyze stats preview.png --rect 80,900,400x60 --json`.
* **Line length:** captions 2 or 3 lines; notes 25 to 45 characters per line. A block that
  needs 6 lines belongs in an article, or should become a chart.
* **Read `warnings` after every `text add`:** PXW4010 means a font was substituted, PXW4013
  a weight was swapped for the nearest.
