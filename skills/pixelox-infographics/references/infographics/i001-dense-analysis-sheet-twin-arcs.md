---
id: i001
title: Dense analysis sheet with twin double arcs
kind: [results, explainer]
topic: [politics, environment, business]
mood: [authoritative, editorial, serious, clean]
style: [flat, editorial, photo-led]
format: portrait
structure: S-briefing-sections
data: [part-to-whole, comparison, counted-units]
charts: [C-half-donut, C-arc-label, C-legend, C-percent-bar, C-label-bars, C-seat-grid, C-stat-block, C-source-line]
palette: ['#ffffff', '#002b5e', '#cb2027', '#69769f', '#e28e76', '#939598', '#000000']
needs: nothing
---

# i001: Dense analysis sheet with twin double arcs

## 1. Concept

A one-page briefing of the kind an analyst firm publishes before a vote: what could change
in a two-chamber parliament, and what that means for one policy field. The top carries the
only color on the page, two half-ring charts side by side (one per chamber), each with an
inner arc for the seats as they stand and an outer arc for the forecast in five shades from
"solid A" to "solid B". Everything below is black, white, and gray: a spending strip, a grid
of ten race profiles with small portraits, and two ballot measures with bars. It works
because color is rationed: red and blue mean the two parties and nothing else, so the eye
finds the political answer first and then reads the consequences in neutral gray.

## 2. Data story

* **Two chambers, each shown twice (part-to-whole):**
  the small chamber (on the order of a hundred seats, about a third of them contested):
  today the two parties are level or a seat or two apart; the forecast splits all seats
  into 7 groups along the arc: A not up for election (about a third), solid A, leaning A,
  toss-up, leaning B (these four are single digits), solid B (mid teens), B not up (a
  little under a third). The large chamber (several hundred seats, all up): today a narrow
  lead for A with a few vacancies; the forecast in 5 groups: two solid blocs of roughly
  40% and just under half, and three thin groups between them of a few percent each.
* **One budget split into 7 parts** (a total in the hundreds of billions): one part is
  nearly half, three are 8 to 15% each, three are small; drawn as a proportional strip.
* **Ten race profiles** (text): region name, two candidates (portraits), 30 to 45 words.
* **Two ballot measures:** a rate before and after (two values a little apart, as two
  bars), a revenue range, a list of 3 uses; and a borrowing total split into 3 parts of
  similar size (a three-part strip) with 1 to 3 uses under each.

Placeholder data used in the recipe (example data, invented):

| Module | Values |
| --- | --- |
| Upper house, 90 seats | today: A 46, B 44; forecast: A not up 30, solid A 8, leaning A 4, toss-up 5, leaning B 2, solid B 14, B not up 27 (33 seats up: 16 of A's, 17 of B's) |
| Lower house, 400 seats | today: A 204, vacant 4, B 192; forecast: solid A 158, leaning A 20, toss-up 28, leaning B 14, solid B 180 |
| Eight-year transit plan, $320B | rail 150, buses 48, stations 44, roads 32, paths 18, other 28 |
| Measure 1 | fuel levy 12.5% now, 14% after; revenue up $2B to $2.8B a year |
| Measure 2, $3.6B in bonds | cleaner buses 1.3, station repairs 0.9, paths 1.4 |

## 3. Information architecture

1. **Title** (one line of caps across the page) and a four-line deck.
2. **"Control of the two houses":** a legend row of five forecast keys; the two double arcs;
   between them a small line drawing of the parliament building and the two-party key;
   under each arc one sentence saying what the trailing party needs.
3. **"What it means for the plan":** a two-line deck, a bold chart title, the spending
   strip with a label in each part.
4. **"Key regional races":** a ruled heading, then 2 rows of 5 profiles: region name, two
   ringed portraits (blue ring, red ring), a short paragraph with the names in bold.
5. **"Key ballot measures":** a ruled heading, then two side-by-side modules, each with a
   bold subtitle, a hero figure, a bar or strip, and bulleted uses.
6. **Footer:** publisher mark at the left, sources in the middle, credits at the right.

Bold section headings with a hairline running to the right edge divide the page; within a
section everything is flush left.

## 4. Format and grid

* Portrait, 1 : 1.18. Recipe canvas 1080 x 1270. Margins 1.5% W: the sheet is set tight.
* Structure `S-briefing-sections`: a full-width title and deck (0% to 9% H), a hero row of
  twin charts (10% to 33% H), then three ruled, titled full-width sections (a strip chart,
  a grid of small profiles, a row of two fact modules), sources from 96% H.
* Title 0.3% to 2.6% H; deck 3.5% to 9% H.
* Section 1: heading at 11% H; legend row at 11.2% H from 25% to 77% W. Arc centers at 26%
  and 73.8% W, 29.6% H; outer ring radius 17.8% W (inner edge at 77% of it), inner ring
  radius 11.6% W (inner edge at 62%); a white gap of 2% W between the rings holds a curved
  label. Notes at 31.4% H.
* Section 2: heading 35.8% H; deck 37.5% to 39.5%; chart title 41.2%; strip
  `1.2%,42.2%,97.6% x 5.4%`.
* Section 3: heading 50.8% H; five columns starting at 1.7%, 21.3%, 40.9%, 60.6%, 80.2% W,
  each 17.6% W wide; row 1 from 53% to 65% H, row 2 from 66.5% to 78% H. In a card: name at
  the left, portraits (5% W across) at 11.8% and 17.1% W from the card's left edge, text
  from 3.5% H below the name.
* Section 4: heading 79% H; left module 1.5% to 56% W, right module 56.3% to 98.8% W, both
  81% to 94.5% H.
* Footer from 96% H.

## 5. Layer stack

1. Ground: white.
2. Title in near-black; deck in dark gray `#444444`.
3. Section headings (bold, sentence case) with 1 px gray rules.
4. Legend row: tracked caps in gray with five 14 px dots.
5. Per chamber: outer half ring (5 or 7 segments: navy `#002b5e`, blue-gray `#69769f`,
   pale blue `#c5d6e3`, gray `#c5c7c8`, pale pink `#e6d4d4`, salmon `#e28e76`, red
   `#cb2027`) with white counts in the segments; inner half ring (navy, red, a black sliver
   for vacancies) with white counts at its two ends; curved gray caps between the rings;
   the chamber's name and a two-line gray caps status under the arch.
6. Center piece: gray line icons (the two party mascots and a domed building in the
   original) over two legend keys.
7. Spending strip: six dark gray parts `#939598` separated by white hairlines and one pale
   part `#c6c8c9`; in each part a white bold amount and a 1 to 4 line name; two thin parts
   are labelled outside with tick lines.
8. Ten profiles: pale gray region silhouettes `#e3e3e3` behind the names, two round
   portraits each with a 3 px ring (navy for party A's candidate, red for party B's), text.
9. Measures: gray silhouettes, heavy black figures, two black bars with white text, a
   three-part strip in black, mid gray, and pale gray, small square bullets.
10. Footer: red publisher mark, gray sources.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#ffffff` | ground | 72% | nothing |
| `#002b5e` | inner arc, "not up" seats, portrait rings | 3% | Party A, held or certain |
| `#cb2027` | inner arc, "not up" seats, portrait rings, publisher mark | 3% | Party B, held or certain |
| `#69769f`, `#c5d6e3` | outer arc | 2% | forecast: solid A, leaning A |
| `#e28e76`, `#e6d4d4` | outer arc | 3% | forecast: solid B, leaning B |
| `#c5c7c8` | outer arc; pale strip parts | 3% | toss-up; "other" |
| `#939598` | spending strip, silhouettes (`#e3e3e3`), secondary type | 9% | amounts with no party meaning |
| `#000000` | title, figures, bars | 5% | nothing |

Diverging for the forecast (two hues, each in a strong and a pale step, around gray), with
the saturated navy and red reserved for seats that are certain. Below the arcs the sheet is
grayscale on purpose: budget figures and measures must not look partisan (two-party red and
blue convention, US).

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Title | event, colon, 4 to 5 words; one line | caps | condensed grotesque, bold (Barlow Condensed, Archivo Narrow / Avenir Next Condensed) | 700 | fitted to 87% W: about 40 | 0 | - | left | `#111111` | 1.5%, 0.5% |
| Deck | 60 to 75 words on 4 lines | sentence | neo-grotesque (Inter, Roboto / Helvetica Neue) | 400 | 17 | 0 | 22 px | left | `#444444` | 1.5%, 3.8% |
| Section heading (x4) | 3 to 5 words | sentence | neo-grotesque | 700 | 21 | 0 | - | left | `#111111` | 1.5%; 11%, 35.8%, 50.8%, 79% |
| Legend keys | "PROJECTIONS:" and 5 keys of 1 to 2 words | caps | neo-grotesque | 700 | 13 | 120 | - | left | `#7a7c80` | 25% to 77% W, 11.2% H |
| Arc count (x16) | 1 to 3 digits | - | neo-grotesque | 700 | 17 | 0 | - | center | `#ffffff` | in the segments |
| Curved label (x2) | "N SEATS UP" and a split | caps | neo-grotesque | 600 | 13 | 80 | - | on an arc | `#7a7c80` | between the rings |
| Chamber name (x2) | 1 to 2 words | caps | neo-grotesque | 800 | 22 | 120 | - | center | `#111111` | under each arch, 25% H |
| Chamber status (x2) | "CURRENT MAJORITY" and a letter, 2 lines | caps | neo-grotesque | 500 | 15 | 120 | 20 px | center | `#939598` | under the name |
| Arc note (x2) | one sentence, 12 to 16 words | sentence | neo-grotesque | 400 | 15 | 0 | - | center | `#444444` | 31.4% H |
| Section deck | 30 to 35 words on 2 lines, 3 phrases bold | sentence | neo-grotesque | 400 / 700 | 17 | 0 | 22 px | left | `#444444` | 1.5%, 37.5% |
| Chart title | 10 to 12 words | sentence | neo-grotesque | 700 | 15 | 0 | - | left | `#111111` | 2.5%, 41.2% |
| Strip labels (x7) | an amount, then 2 to 8 words | sentence | neo-grotesque | 700 | 13 | 0 | 14 px | left | `#ffffff` (dark on the pale part) | top left of each part |
| Profile name (x10) | 1 word, at most 8 characters | title | neo-grotesque | 700 | 15 (17 in the original) | 0 | - | left | `#111111` | card top left |
| Profile text (x10) | 30 to 45 words, 5 to 7 lines of about 28 characters, names bold | sentence | neo-grotesque | 400 / 700 | 14 | 0 | 17 px | left | `#444444` | under the portraits |
| Measure subtitle (x2) | 3 to 12 words | title | neo-grotesque | 700 | 16 | 0 | 20 px | left | `#111111` | module top left |
| Measure figure (x4) | "+N.NN%", "$NB" | - | neo-grotesque | 700 | 30 | 0 | - | left | `#000000` | beside small 13 px captions |
| Bar text (x2) | a label and a rate | sentence | neo-grotesque | 700 | 13 | 0 | - | left and right | `#ffffff` | inside the black bars |
| Strip amounts (x3) | "$N.NB" | - | neo-grotesque | 700 | 28 | 0 | - | left | `#ffffff` | inside the parts |
| Bullets | 1 to 4 words each | sentence | neo-grotesque | 400 | 13 | 0 | 17 px | left | `#444444` | under each part |
| Sources, credits | 2 to 3 lines | sentence | neo-grotesque | 400 | 13 | 0 | 17 px | left; right | `#6b6b6b` | 96% to 99% H |

Two families: a condensed bold for the single title line, one neutral grotesque for all
else, in two weights. Sizes 40 : 30 : 21 : 17 : 13, about 3.1 : 2.3 : 1.6 : 1.3 : 1. The
sheet is dense: body at 14 to 17 px is the floor, and it only holds because lines are short
and bold lead-ins mark every entry.

## 8. Charts and diagrams

* **C-half-donut** x4, `scripts/donut.js` by center, two rings on each center. Outer ring:
  `r=17.8 inner=0.77 sweep=180`, 7 or 5 values in seating order with the forecast colors,
  `labelpos=in show=value`, `sep='#ffffff'`, `fs=17`. Inner ring: `r=11.6 inner=0.62`, 2 or
  3 values (A, vacant, B). The white gap between the rings (125 to 148 px from the center
  at 1080 wide) is deliberate: it holds the curved label.
* **C-arc-label** x2 for "N seats up": a half circle of radius 131 px on each center.
* **C-legend** x2, `scripts/legend.js`: the five forecast keys in a row (`shape=circle`),
  and the two party keys in a column between the charts.
* **C-percent-bar** for the spending strip: `scripts/bars.js`, `dir=h mode=stack`, six
  series of one value, `max=` the total so the strip fills its box, amounts written inside
  (`prefix=$ suffix=B`), names in the legend under it. One gray in six steps from dark to
  pale replaces the original's single gray with white separators.
* **C-label-bars** for the two rates: `scripts/bars.js`, `dir=h labelpos=inside
  vlabels=inside`, black, `radius=0`, `max=15`: each name is written inside its bar at the
  left and the rate inside at the right end, as in the original. The bars start at zero, so
  a 12.5 to 14 difference looks as small as it is.
* **C-percent-bar** again for the bond split (three parts: black, mid gray, pale gray).
* **C-seat-grid**'s script (`scripts/waffle.js`, `shape=circle`, one row of 5, a wide
  `gap`) draws the ten pairs of portrait discs: four runs per row (ring color, then a pale
  disc inside it) instead of forty hand-placed circles.
* Simplified: the original writes counts at the ring ends on the baseline and labels thin
  segments outside; the script writes counts in the middle of each segment and leaves
  segments under 4% unlabelled.

## 9. Icons and illustration

* **Portraits:** 20 round head-and-shoulders photos, about 5% W across, each cropped to a
  circle with a 3 px ring in its party's color; neutral studio or press shots, faces
  centered, in full color (the only photographs on the page). Never name the people. The
  recipe draws ringed pale discs; to use photos, place `subject.png` and `subject2.png` at
  44 px on each disc's center and clip each with `mask add --from-path` (a circle).
* **Region silhouettes:** pale gray outlines of each region behind its name (about 8% W),
  and larger ones (14% W) in the two measure modules. They are locators only. Supply them
  as images (C-map-image); do not draw outlines by hand. The recipe leaves them out.
* Center piece: `landmark`, 44 px, gray `#939598` (the original adds the two party mascots
  as line drawings; do not invent emblems).

## 10. Effects and finish

None: flat, white, hairline rules. No shadows, no gradients, no rounded corners.

## 11. Hierarchy and eye path

1. The two colored arcs (the only color), left then right.
2. The title above them (large, black).
3. Section headings down the left edge, which let the reader pick a section.
4. Inside a section: the heavy black figures and bars first, then the gray strips, then
   the small text.
5. Portrait rings, which repeat the party colors and tie the profiles back to the arcs.

Title 40 px; figures 30; headings 21; text 14 to 17; labels 13.

## 12. Build recipe

Nothing to supply. Canvas 1080 x 1270. All figures are example data.

```bash
pixelox new -o brief.pxo --size 1080x1270 --fill '#ffffff'
pixelox text add brief.pxo --in-place --name Title --content "GENERAL ELECTION 2031: THE TRANSIT PLAN AT STAKE" --font @condensed --weight 700 --fit-width 940 --color '#111111' --at 16,8 --anchor tl --anchor-to ink
pixelox text add brief.pxo --in-place --name Deck --content "Voters elect both houses of the national assembly this autumn. This sheet shows how the seats stand today, how forecasters rate the open races, and what the next assembly will decide about the eight-year transit plan and two regional ballot measures. Parties, regions, and all figures on this sheet are example data, invented for the layout." --font @neo-grotesque --size 17 --leading 22 --color '#444444' --box 1048x96 --at 16,50 --anchor tl
pixelox text add brief.pxo --in-place --name Head1 --content "Control of the two houses" --font @neo-grotesque --weight 700 --size 21 --color '#111111' --at 16,150 --anchor l
pixelox text add brief.pxo --in-place --name LegendLabel --content "PROJECTIONS:" --font @neo-grotesque --weight 700 --size 13 --tracking 120 --color '#111111' --at 300,151 --anchor l
pixelox render js scripts/legend.js -o keys.png --size 1080x1270 --param box=39,10.9,52,2.2 --param labels='SOLID A|LEANING A|TOSS-UP|LEANING B|SOLID B' --param colors='#69769f,#c5d6e3,#c5c7c8,#e6d4d4,#e28e76' --param shape=circle --param muted='#7a7c80' --param fs=14
pixelox comp place brief.pxo keys.png --in-place --as Keys --at 0,0
pixelox render js scripts/donut.js -o upOut.png --size 1080x1270 --param cx=26 --param cy=29.6 --param r=17.8 --param inner=0.77 --param sweep=180 --param values=30,8,4,5,2,14,27 --param colors='#002b5e,#69769f,#c5d6e3,#c5c7c8,#e6d4d4,#e28e76,#cb2027' --param labelpos=in --param show=value --param sep='#ffffff' --param fs=17
pixelox render js scripts/donut.js -o upIn.png --size 1080x1270 --param cx=26 --param cy=29.6 --param r=11.6 --param inner=0.62 --param sweep=180 --param values=46,44 --param colors='#002b5e,#cb2027' --param labelpos=in --param show=value --param sep='#ffffff' --param fs=17
pixelox render js scripts/donut.js -o lowOut.png --size 1080x1270 --param cx=73.8 --param cy=29.6 --param r=17.8 --param inner=0.77 --param sweep=180 --param values=158,20,28,14,180 --param colors='#69769f,#c5d6e3,#c5c7c8,#e6d4d4,#e28e76' --param labelpos=in --param show=value --param sep='#ffffff' --param fs=17
pixelox render js scripts/donut.js -o lowIn.png --size 1080x1270 --param cx=73.8 --param cy=29.6 --param r=11.6 --param inner=0.62 --param sweep=180 --param values=204,4,192 --param colors='#002b5e,#000000,#cb2027' --param labelpos=in --param show=value --param sep='#ffffff' --param fs=17
pixelox comp place brief.pxo upOut.png --in-place --as UpperForecast --at 0,0
pixelox comp place brief.pxo upIn.png --in-place --as UpperNow --at 0,0
pixelox comp place brief.pxo lowOut.png --in-place --as LowerForecast --at 0,0
pixelox comp place brief.pxo lowIn.png --in-place --as LowerNow --at 0,0
pixelox text add brief.pxo --in-place --name UpArc --content "33 SEATS UP: 16 A, 17 B" --font @neo-grotesque --weight 600 --size 13 --tracking 80 --color '#7a7c80' --align center
pixelox text on-path brief.pxo --in-place --layer UpArc --d 'M 150 376 A 131 131 0 0 1 412 376' --offset 50%
pixelox text add brief.pxo --in-place --name LowArc --content "ALL 400 SEATS UP" --font @neo-grotesque --weight 600 --size 13 --tracking 80 --color '#7a7c80' --align center
pixelox text on-path brief.pxo --in-place --layer LowArc --d 'M 666 376 A 131 131 0 0 1 928 376' --offset 50%
pixelox text add brief.pxo --in-place --name UpName --content "UPPER" --font @neo-grotesque --weight 800 --size 22 --tracking 120 --color '#111111' --at 281,322 --anchor c
pixelox text add brief.pxo --in-place --name UpStatus --content $'CURRENT\nMAJORITY | A' --font @neo-grotesque --weight 500 --size 14 --tracking 100 --leading 19 --align center --color '#939598' --at 281,354 --anchor c
pixelox text add brief.pxo --in-place --name LowName --content "LOWER" --font @neo-grotesque --weight 800 --size 22 --tracking 120 --color '#111111' --at 797,322 --anchor c
pixelox text add brief.pxo --in-place --name LowStatus --content $'CURRENT\nMAJORITY | A' --font @neo-grotesque --weight 500 --size 14 --tracking 100 --leading 19 --align center --color '#939598' --at 797,354 --anchor c
pixelox shape icon brief.pxo landmark --in-place --name Building --size 44 --at 540,232 --anchor c --color '#939598' --stroke-width 3
pixelox render js scripts/legend.js -o parties.png --size 1080x1270 --param box=45.5,20.6,12,4.6 --param labels='PARTY A|PARTY B' --param colors='#002b5e,#cb2027' --param dir=column --param shape=circle --param bold=1 --param muted='#7a7c80' --param fs=14
pixelox comp place brief.pxo parties.png --in-place --as Parties --at 0,0
pixelox text add brief.pxo --in-place --name UpNote --content "A majority is 46 seats: Party B needs 19 of the 33 open races" --font @neo-grotesque --size 15 --color '#444444' --at 281,402 --anchor c
pixelox text add brief.pxo --in-place --name LowNote --content "A majority is 201 seats: Party B holds 192 and needs 9 more" --font @neo-grotesque --size 15 --color '#444444' --at 797,402 --anchor c
pixelox text add brief.pxo --in-place --name Head2 --content "What it means for the transit plan" --font @neo-grotesque --weight 700 --size 21 --color '#111111' --at 16,456 --anchor l
pixelox text add brief.pxo --in-place --name Deck2 --content "The next assembly votes each year on how fast the plan is paid out. The strip shows where the money is meant to go: rail takes nearly half of it, buses and stations a little over a quarter." --font @neo-grotesque --size 17 --leading 22 --color '#444444' --box 1048x50 --at 16,472 --anchor tl
pixelox text add brief.pxo --in-place --name StripTitle --content 'The plan sets aside $320 billion for transit over eight years' --font @neo-grotesque --weight 700 --size 15 --color '#111111' --at 28,532 --anchor l
pixelox render js scripts/bars.js -o plan.png --size 1080x1270 --param box=1.2,42.8,97.6,8 --param dir=h --param mode=stack --param values='150;48;44;32;18;28' --param series='Rail lines|Bus fleets|Stations|Road repairs|Bike and foot paths|Other items' --param colors='#55585c,#6d7074,#85888b,#9c9fa2,#b3b5b8,#cfd1d3' --param max=320 --param prefix='$' --param suffix=B --param legend=bottom --param fs=15
pixelox comp place brief.pxo plan.png --in-place --as PlanStrip --at 0,0
pixelox text add brief.pxo --in-place --name Head3 --content "Key regional races" --font @neo-grotesque --weight 700 --size 21 --color '#111111' --at 16,662 --anchor l
pixelox shape line brief.pxo --in-place --name Rule3 --from 216,663 --to 1064,663 --stroke '#9a9a9a' --stroke-width 1
pixelox render js scripts/waffle.js -o a1.png --size 1080x1270 --param box=9.26,54.1,83.52,4.25 --param values=5 --param cols=5 --param shape=circle --param gap=2.926 --param colors='#002b5e' --param legend=none
pixelox render js scripts/waffle.js -o b1.png --size 1080x1270 --param box=14.63,54.1,83.52,4.25 --param values=5 --param cols=5 --param shape=circle --param gap=2.926 --param colors='#cb2027' --param legend=none
pixelox render js scripts/waffle.js -o a1in.png --size 1080x1270 --param box=9.72,54.49,82.6,3.46 --param values=5 --param cols=5 --param shape=circle --param gap=3.818 --param colors='#dfe1e4' --param legend=none
pixelox render js scripts/waffle.js -o b1in.png --size 1080x1270 --param box=15.09,54.49,82.6,3.46 --param values=5 --param cols=5 --param shape=circle --param gap=3.818 --param colors='#dfe1e4' --param legend=none
pixelox render js scripts/waffle.js -o a2.png --size 1080x1270 --param box=9.26,67.2,83.52,4.25 --param values=5 --param cols=5 --param shape=circle --param gap=2.926 --param colors='#002b5e' --param legend=none
pixelox render js scripts/waffle.js -o b2.png --size 1080x1270 --param box=14.63,67.2,83.52,4.25 --param values=5 --param cols=5 --param shape=circle --param gap=2.926 --param colors='#cb2027' --param legend=none
pixelox render js scripts/waffle.js -o a2in.png --size 1080x1270 --param box=9.72,67.59,82.6,3.46 --param values=5 --param cols=5 --param shape=circle --param gap=3.818 --param colors='#dfe1e4' --param legend=none
pixelox render js scripts/waffle.js -o b2in.png --size 1080x1270 --param box=15.09,67.59,82.6,3.46 --param values=5 --param cols=5 --param shape=circle --param gap=3.818 --param colors='#dfe1e4' --param legend=none
pixelox comp place brief.pxo a1.png --in-place --as RingsA1 --at 0,0
pixelox comp place brief.pxo b1.png --in-place --as RingsB1 --at 0,0
pixelox comp place brief.pxo a1in.png --in-place --as FacesA1 --at 0,0
pixelox comp place brief.pxo b1in.png --in-place --as FacesB1 --at 0,0
pixelox comp place brief.pxo a2.png --in-place --as RingsA2 --at 0,0
pixelox comp place brief.pxo b2.png --in-place --as RingsB2 --at 0,0
pixelox comp place brief.pxo a2in.png --in-place --as FacesA2 --at 0,0
pixelox comp place brief.pxo b2in.png --in-place --as FacesB2 --at 0,0
pixelox text add brief.pxo --in-place --name N1 --content "Norfield" --font @neo-grotesque --weight 700 --size 15 --color '#111111' --at 18,714 --anchor l
pixelox text add brief.pxo --in-place --name N2 --content "Eastport" --font @neo-grotesque --weight 700 --size 15 --color '#111111' --at 230,714 --anchor l
pixelox text add brief.pxo --in-place --name N3 --content "Lakeland" --font @neo-grotesque --weight 700 --size 15 --color '#111111' --at 442,714 --anchor l
pixelox text add brief.pxo --in-place --name N4 --content "Riverton" --font @neo-grotesque --weight 700 --size 15 --color '#111111' --at 654,714 --anchor l
pixelox text add brief.pxo --in-place --name N5 --content "Highmoor" --font @neo-grotesque --weight 700 --size 15 --color '#111111' --at 866,714 --anchor l
pixelox text add brief.pxo --in-place --name P1 --content "The next governor appoints the transit board; both candidates have published a route plan." --font @neo-grotesque --size 14 --leading 17 --color '#444444' --box 192x110 --at 18,750 --anchor tl
pixelox text add brief.pxo --in-place --name P2 --content "Three assembly seats here decide whether the regional bus contract is renewed next spring." --font @neo-grotesque --size 14 --leading 17 --color '#444444' --box 192x110 --at 230,750 --anchor tl
pixelox text add brief.pxo --in-place --name P3 --content "Both candidates back the harbour line; they differ on how to pay for its second phase." --font @neo-grotesque --size 14 --leading 17 --color '#444444' --box 192x110 --at 442,750 --anchor tl
pixelox text add brief.pxo --in-place --name P4 --content "An open seat after two terms. The winner inherits a half-built depot and its budget." --font @neo-grotesque --size 14 --leading 17 --color '#444444' --box 192x110 --at 654,750 --anchor tl
pixelox text add brief.pxo --in-place --name P5 --content "The lake crossing is the main issue: one side wants a bridge, the other a ferry service." --font @neo-grotesque --size 14 --leading 17 --color '#444444' --box 192x110 --at 866,750 --anchor tl
pixelox text add brief.pxo --in-place --name N6 --content "Southbay" --font @neo-grotesque --weight 700 --size 15 --color '#111111' --at 18,880 --anchor l
pixelox text add brief.pxo --in-place --name N7 --content "Westvale" --font @neo-grotesque --weight 700 --size 15 --color '#111111' --at 230,880 --anchor l
pixelox text add brief.pxo --in-place --name N8 --content "Pinewood" --font @neo-grotesque --weight 700 --size 15 --color '#111111' --at 442,880 --anchor l
pixelox text add brief.pxo --in-place --name N9 --content "Stoneham" --font @neo-grotesque --weight 700 --size 15 --color '#111111' --at 654,880 --anchor l
pixelox text add brief.pxo --in-place --name N10 --content "Clearbay" --font @neo-grotesque --weight 700 --size 15 --color '#111111' --at 866,880 --anchor l
pixelox text add brief.pxo --in-place --name P6 --content "The clean bus rules passed here last year come up for their first review after the vote." --font @neo-grotesque --size 14 --leading 17 --color '#444444' --box 192x110 --at 18,916 --anchor tl
pixelox text add brief.pxo --in-place --name P7 --content "A recount is possible: the last two contests here were decided by under one point." --font @neo-grotesque --size 14 --leading 17 --color '#444444' --box 192x110 --at 230,916 --anchor tl
pixelox text add brief.pxo --in-place --name P8 --content "The regional rail compact needs this assembly's vote to be renewed for four more years." --font @neo-grotesque --size 14 --leading 17 --color '#444444' --box 192x110 --at 442,916 --anchor tl
pixelox text add brief.pxo --in-place --name P9 --content "The roads commission chair is on the ballot after a winter of bridge closures." --font @neo-grotesque --size 14 --leading 17 --color '#444444' --box 192x110 --at 654,916 --anchor tl
pixelox text add brief.pxo --in-place --name P10 --content "Voters also choose a new transit auditor, who signs off every project in the plan." --font @neo-grotesque --size 14 --leading 17 --color '#444444' --box 192x110 --at 866,916 --anchor tl
pixelox text add brief.pxo --in-place --name Head4 --content "Key ballot measures" --font @neo-grotesque --weight 700 --size 21 --color '#111111' --at 16,1022 --anchor l
pixelox shape line brief.pxo --in-place --name Rule4 --from 224,1023 --to 1064,1023 --stroke '#9a9a9a' --stroke-width 1
pixelox text add brief.pxo --in-place --name M1Head --content "Lakeland: Measure 12" --font @neo-grotesque --weight 700 --size 16 --color '#111111' --at 16,1050 --anchor l
pixelox text add brief.pxo --in-place --name M1Cap --content $'Additional levy of\n\n\non fuel sales' --font @neo-grotesque --size 13 --leading 17 --color '#444444' --at 16,1072 --anchor tl
pixelox text add brief.pxo --in-place --name M1Fig --content "+1.5%" --font @neo-grotesque --weight 700 --size 30 --color '#000000' --at 16,1106 --anchor l --anchor-to ink
pixelox render js scripts/bars.js -o rates.png --size 1080x1270 --param box=15.5,84,37,6 --param dir=h --param labelpos=inside --param vlabels=inside --param values=12.5,14 --param labels='Current rate|Rate after the measure' --param colors='#000000' --param max=15 --param suffix=% --param decimals=1 --param gap=0.25 --param radius=0 --param fs=14
pixelox comp place brief.pxo rates.png --in-place --as Rates --at 0,0
pixelox text add brief.pxo --in-place --name M1Funds --content 'Will fund: new trams, depot upgrades, safer crossings. Raises revenue by $2B to $2.8B a year.' --font @neo-grotesque --size 13 --leading 17 --color '#444444' --box 560x40 --at 16,1160 --anchor tl
pixelox text add brief.pxo --in-place --name M2Head --content "Riverton: Clean Stations and Paths Bond" --font @neo-grotesque --weight 700 --size 16 --color '#111111' --at 608,1050 --anchor l
pixelox text add brief.pxo --in-place --name M2Cap --content "Would allow borrowing of up to" --font @neo-grotesque --weight 700 --size 13 --color '#111111' --at 608,1082 --anchor l
pixelox text add brief.pxo --in-place --name M2Fig --content '$3.6B' --font @neo-grotesque --weight 700 --size 30 --color '#000000' --at 840,1082 --anchor l --anchor-to ink
pixelox render js scripts/bars.js -o bond.png --size 1080x1270 --param box=56.3,87,42.5,8 --param dir=h --param mode=stack --param values='1.3;0.9;1.4' --param series='Cleaner buses|Station repairs|New paths' --param colors='#000000,#8a8c8f,#c6c8c9' --param max=3.6 --param prefix='$' --param suffix=B --param decimals=1 --param legend=bottom --param fs=15
pixelox comp place brief.pxo bond.png --in-place --as BondStrip --at 0,0
pixelox text add brief.pxo --in-place --name Mark --content "YOUR MARK" --font @neo-grotesque --weight 800 --size 20 --color '#cb2027' --at 16,1240 --anchor l
pixelox text add brief.pxo --in-place --name Sources --content $'Sources: Example Elections Office, Example Budget Review.\nExample data, invented for this layout.' --font @neo-grotesque --size 13 --leading 17 --color '#6b6b6b' --at 200,1240 --anchor l
pixelox export brief.pxo -o brief.png
```

Reuse: change the four `values=` lists of the arcs (seating order, A to B; the inner ring's
values must add up to the outer ring's), the two curved labels, and the notes (the "seats
up" count is the outer ring without its two end segments; the majority is half the total
plus one). The rate bars carry their own names (`labelpos=inside`): keep each name shorter
than the shorter bar. Portrait
discs: each run draws one row of five 54 px discs with a 212 px pitch (`gap=2.926`); the
pale inner discs are 44 px (`gap=3.818`) on the same centers, and party B's runs are
shifted 58 px (5.37% W) to the right. The strips fill their boxes because `max=` equals the
sum of the values. In a profile keep the name to 8 characters (it shares its line with the two
discs) and the text under 110 characters, and write no real person's name.

## 13. Variations and pairings

* **Relatives:** i003 and i004 are results posters built around one ring; this sheet is
  the analyst's version: two small arcs, then consequences. i022 is the light "before the
  vote" poster with seats as units.
* **One chamber:** a single double arc at the center (`r=24`) with the legend to its right;
  the two notes become one line.
* **No profiles:** drop section 3 for a 4:3 sheet (1080 x 810) with arcs, strip, and one
  measure.
* **Business use:** the double arc holds "today against forecast" for any two-sided split
  (market share now and in five years; budget against actuals); keep the gray lower half.
* Pairs with C-tile-map in place of the profile grid (which regions hold a key race) and
  with C-table for the measures when there are more than two.

## 14. Pitfalls

* **Inner and outer arcs with different totals.** Both rings stand for the same chamber:
  30 + 8 + 4 + 5 + 2 + 14 + 27 must equal 46 + 44. If they do not, the two arcs cannot be
  compared and the forecast looks rigged. Check both scripts' logged totals. This is the
  sheet's first data-integrity check.
* **Forecast colors reused for certain seats.** Pale steps mean "leaning", strong steps
  "solid", saturated navy and red "held": swapping any of them changes the claim. Keep the
  legend in arc order and next to the arcs.
* **Party colors below the arcs.** A red bar in the budget strip reads as "Party B's
  money". Everything that is not a seat stays gray or black.
* **Two rates drawn from a cut axis.** 12.5% and 14% differ by a little; bars from zero
  show that. A truncated axis would turn a 1.5 point rise into a doubling.
* **Portraits of different crops and sizes.** Twenty faces only read as one system when the
  circles, the ring widths, and the head sizes match; and a ring color is a claim about a
  person's party, so check each one.
* **A wall of 14 px text.** The profile grid works because every paragraph has the same
  length and a bold lead; one long entry breaks the grid.
