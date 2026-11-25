---
id: i022
title: Light results sheet with hexagon seats
kind: [results, statistical]
topic: [politics]
mood: [clean, calm, editorial, serious]
style: [flat, editorial]
format: square
structure: S-results-sheet
data: [counted-units, part-to-whole, single-stat]
charts: [C-hemicycle, C-seat-grid, C-stat-block, C-title-block, C-source-line]
palette: ['#ffffff', '#b9e2fa', '#f0d6d9', '#2d368d', '#cb0303', '#f7941d', '#2b2b2b']
needs: nothing
---

# i022: Light results sheet with hexagon seats

## 1. Concept

A nearly square briefing sheet on white that shows, before a vote, which seats of a
two-chamber parliament are being contested. The small chamber is an arch of about a hundred
hexagon seats, the large one a solid block of hundreds of the same hexagons, and a third
module is only a number and a sentence. Dark hexagons are seats that are safe this time; pale
hexagons with a thin gray outline are the seats up for election, in a tint of their holder's
color; an outline without a fill marks an independent or a vacant seat. It works because one small mark, the
hexagon, means "one seat" everywhere on the sheet, and the dark-against-pale contrast answers
the sheet's only question (what is at stake?) without a single chart label.

## 2. Data story

* **Chamber 1, counted units in 5 classes:** on the order of a hundred seats. Two blocs of
  similar size; each bloc's seats are split into "not up" (dark) and "up for election"
  (pale). About a third of the chamber is up. A few seats belong to independents (an orange
  outline). Order along the arch, left to right: A not up, A up, independents, B up, B not up.
* **Chamber 2, counted units in 3 classes plus vacancies:** several hundred seats, **all** up
  for election, so every hexagon is pale: Bloc A's block at the left (a little more than
  half), one independent seat on the seam, Bloc B's block at the right. A handful of vacant
  seats (empty outlines) hang in a small tab under the block.
* **Three single numbers:** seats up in chamber 1; all seats of chamber 2; a count of regional
  races, with a sentence that splits it into two kinds of territory.
* **Six short facts:** three beside each chamber (seats per region, minimum age, term length),
  each a plain sentence with one number or phrase in bold.

Placeholder data used in the recipe (example data, invented):

| Module | Values |
| --- | --- |
| Upper house, 90 seats | Bloc A not up 27, Bloc A up 15, independent (not up) 3, Bloc B up 14, Bloc B not up 31; 29 seats up |
| Lower house, 406 seats | Bloc A 212, independent 2, Bloc B 186, vacant 6; all up |
| Regional races | 8, in 5 regions and 3 territories |

## 3. Information architecture

1. **Title block** (top left): a kicker in bold caps (subject and year), a light-weight title
   in orange, a two-line deck.
2. **Chamber 1** (a three-column row): hero number with caption at the left, the arch in the
   middle with the chamber's name inside it, three facts at the right.
3. A hairline rule.
4. **Chamber 2:** the chamber's name centered in italic serif, the seat block across the full
   width inside a dotted frame, then a row with the hero number at the left and three facts
   beside it.
5. A hairline rule.
6. **Regional races:** a centered italic heading, a number with a two-line statement; the
   **legend** in a small outlined box at the right.
7. **Footer:** publisher mark, source and date in gray.

Each module is: italic name, picture, big number, facts. The rules and the repeated order do
the guiding; there are no arrows and no numbering.

## 4. Format and grid

* Near square, 1 : 1.02. Recipe canvas 1080 x 1100. Margins 3% W for the title block; charts
  and rules are inset to 6.5% W.
* Title block: kicker cap top 3.3% H, title cap top 6% H, deck cap tops 11.3% and 14.1% H,
  all at 3% W.
* Chamber 1 row, 17% to 39% H: number centered on 16.4% W, 27.8% H; arch center 48.4% W,
  38% H, outer radius 20.2% W, dotted guide arc at 21.6% W; facts from 72.7% W, 23% to 37% H.
* Rule 1 at 40.3% H, from 6.5% to 93.7% W.
* Chamber 2: name at 42.1% H; seat block `8.3%,44.3%,83.6% x 20.5%` (40 to 44 columns by 10
  rows); dotted frame 1% W outside it; vacant tab centered under the frame at 66.5% H; number
  centered on 33.5% W, 71.5% H; facts from 41% W, 69.5% to 75% H.
* Rule 2 at 77.9% H.
* Regional races: heading at 81.1% H; number centered on 36% W, 86.6% H; statement from 40.3%
  W; legend box `76.3%,79.3%,21.5% x 16%`.
* Footer baseline at 96.5% H.
* Axis: module names and seat charts are centered on 48.4% W (slightly left of the middle,
  because the right column of facts is wider than the left number column); the title block
  and footer hang from the left margin.

## 5. Layer stack

1. Ground: white.
2. Kicker (near-black bold caps), title (orange, light weight), deck (near-black regular).
3. Arch: about a hundred hexagon seats in 5 rows over a half circle, about 1.9% W across
   each: navy `#2d368d`, light blue `#b9e2fa`, pale pink `#f0d6d9`, red `#cb0303`; the
   independents are outlines in orange `#f0a431`. Every pale seat has a thin gray outline
   (1 px `#8a8a8a` in the original; 2 px `#9a9a9a` in the build).
4. A dotted gray half-circle guide just outside the arch (1.5 px, dots 2 px, gaps 4 px).
5. Chamber name in bold italic serif inside the arch, the seat total under it in small caps.
6. Hero number (very heavy, dark gray `#333333`) with a one-line caption; three facts.
7. Rule `#dcdcdc`, 2 px.
8. Chamber 2 name; seat block: pale blue block, one or two yellow seats `#feff9a` on the
   seam, pale pink block, all with the thin gray outline; a dotted frame around the block
   with a small tab under its middle holding the vacant seats (white hexagons with a gray
   outline `#8a8a8a`).
9. "All", hero number, caption; three facts with bold figures.
10. Rule.
11. Regional heading; hero number; two-line statement (second line bold); an italic serif
    aside in gray.
12. Legend box: 1 px gray outline, seven hexagon keys (two columns in the original, one in
    the build), tiny caps.
13. Footer: mark, source line.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#ffffff` | ground | 79% | nothing |
| `#b9e2fa` | pale blue seats | 6% | Bloc A, seat up for election |
| `#f0d6d9` | pale pink seats | 5% | Bloc B, seat up for election |
| `#2d368d` | navy seats | 1.5% | Bloc A, seat not up |
| `#cb0303` | red seats | 1.5% | Bloc B, seat not up |
| `#feff9a` | yellow seats | under 1% | independent, seat up for election |
| `#f0a431` | orange outlines | under 1% | independent, seat not up |
| `#8a8a8a` | gray outlines (`#9a9a9a` around pale seats) | 1% | vacant seat; the edge of a pale seat |
| `#f7941d` | title, handle in the footer | 1% | nothing: the single accent |
| `#2b2b2b` | kicker, deck, numbers (`#333333`), facts (`#555555`) | 5% | nothing |

Categorical with a twist: two hues (blue and red, the two-party convention in the US), each
in a dark and a pale step. Hue = bloc; lightness = status (dark is settled, pale is open). The
accent orange belongs to neither bloc and is used for the title only.

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Kicker | subject and year, 2 to 3 words | caps | geometric sans (Montserrat / Avenir Next) | 800 | 23 | -10 | - | left | `#2b2b2b` | 3%, cap top 3.3% |
| Title | 2 to 4 words | sentence | geometric sans | 300 (400 in the build: a face thinner than light breaks up) | 51 | -20 | - | left | `#f7941d` | 3%, cap top 6% |
| Deck | 14 to 18 words on 2 lines | sentence | geometric sans | 400 | 25 | 0 | 31 px pitch | left | `#2b2b2b` | 3%, cap top 11.3% |
| Chamber name (x3) | 1 to 3 words | title | bold italic serif, class oldstyle (Playfair Display bold italic / Baskerville bold italic) | 700 | 27 | 0 | - | center | `#333333` | 48.4% W; inside the arch at 34.2% H, above the block at 42.1% H, above the last module at 81.1% H |
| Seat total | number and "SEATS" | caps | geometric sans | 500 | 17 | 20 | - | center | `#555555` | 48.4%, 36.5% |
| Hero number (x3) | 1 to 3 digits | - | geometric sans, heaviest face (Montserrat Black / Avenir Next Heavy) | 900 | 58 (the last: 68) | -20 | - | center | `#333333` | 16.4%,27.8%; 33.5%,71.5%; 36%,86.6% |
| Number caption (x2) | 4 to 5 words | sentence | geometric sans | 400 | 18 | 0 | - | center | `#555555` | under each number |
| Fact (x6) | 4 to 9 words, one number or phrase in bold | sentence | geometric sans | 400 with 700 | 18 to 19 | 0 | 24 to 29 px pitch | left | `#555555`, bold part `#2b2b2b` | right column of each chamber row |
| Statement | 9 to 11 words on 2 lines, the counts bold | sentence | geometric sans | 400 / 700 | 25 | 0 | 27 px pitch | left | `#555555` / `#2b2b2b` | 40.3%, 84.6% and 87% |
| Aside | 3 to 5 words in brackets | sentence | oldstyle italic | 400 | 19 | 0 | - | left | `#8a8a8a` | 40.3%, 89.5% |
| Legend keys (x7) | 2 to 4 words | caps (sentence in the build) | geometric sans | 400 | 12 in the original; 15 in the build | 20 (0 in the build) | 23 px pitch | left | `#555555` | in the legend box |
| Source | "Source:", a list, a date | sentence | geometric sans | 500 | 16 | 0 | - | left | `#8a8a8a` | 17.6%, 96.5% |

Two families: a geometric sans in four weights (300 title, 400 text, 700 to 800 emphasis, 900
numbers) and a bold italic serif used only for the three module names. Sizes 58 : 51 : 27 :
25 : 18, about 3.2 : 2.8 : 1.5 : 1.4 : 1. The numbers are heavy but not huge; the weight
contrast (900 against 300 and 400) does the work that size does elsewhere.

## 8. Charts and diagrams

* **C-hemicycle**, `scripts/hemicycle.js` by center: `cx=48.4 cy=38 r=20.2`, `rows=5`,
  `inner=0.5`, `shape=hex`, `seats=` five groups in the order they sit, `colors=` navy, pale
  blue, orange, pale pink, red, `hollow=3` (the independents are outlines only),
  `counts=0 legend=none` (the counts are told by the hero number and the legend box). No
  majority line. The middle holds the chamber's name.
* **Outlined pale seats:** a second run of each seat chart with the same geometry and
  data, placed on top, draws only outlines: `hollow=` lists the pale groups, their color
  is the gray `#9a9a9a`, and the other groups get the color `none`. Arch:
  `colors='none,#9a9a9a,none,#9a9a9a,none' hollow=2,4`; block: `colors='#9a9a9a'
  hollow=1,2,3`.
* **C-seat-grid**, `scripts/waffle.js`: `cols=40 order=cols shape=hex gap=0.1`, three groups
  (Bloc A, independent, Bloc B), box `8.3,44.3,83.6,20.5`: 10 rows, filled column by column,
  so each bloc is one solid block and the independent seats land on the seam. The three
  values add up to a multiple of 10, so the block is a full rectangle.
* **Vacant seats:** a `scripts/waffle.js` run with `values=6 cols=6 hollow=1` in gray
  `#8a8a8a`, in a small box under the block: empty outlines the same size as the block's
  units (set the box height to one unit).
* **Legend:** the same script as a column of seven keys (`cols=1`, seven values of 1,
  `gap=0.4375` for a 23 px pitch, `hollow=5,7`), an outline run for the three pale keys,
  and one seven-line text layer with `--leading 23`, inside an outlined box. This keeps the
  keys the same marks as the seats, hexagons and outlines included, which the C-legend
  script cannot draw. The original sets the keys in two columns with two-line caps labels.
* **C-stat-block** x3 for the hero numbers (no icons).
* Simplified: the script's outlines are about 2 px (a tenth of a unit) where the
  original's are 1 px, so the build uses a lighter gray; in the original the independents
  in the arch keep a pale fill inside their orange outline.

## 9. Icons and illustration

None. The original's footer carries a publisher emblem (about 9% W wide, gray), a license
badge, and an event mark in the bottom right corner built from small red and blue bars; use
your own `logo.png` there or a text credit. No party marks, no portraits.

## 10. Effects and finish

None: flat, white, no shadows. The finish is in the line work: dotted guides around both seat
charts and two pale hairline rules between the modules.

## 11. Hierarchy and eye path

1. The dark seats at both ends of the arch (the only saturated color on the sheet).
2. The pale block of the second chamber, the largest shape.
3. The three heavy numbers, top to bottom, each with its caption.
4. The orange title (large but thin, so it is read after the pictures), then the deck.
5. Facts, legend, footer.

Hero numbers 58 px at weight 900, title 51 px at weight 300, names 27, facts 18.

## 12. Build recipe

Nothing to supply. Canvas 1080 x 1100. Example data throughout.

```bash
pixelox new -o sheet.pxo --size 1080x1100 --fill '#ffffff'
pixelox text add sheet.pxo --in-place --name Kicker --content "NATIONAL ELECTIONS 2031" --font @geometric-sans --weight 800 --size 23 --tracking=-10 --color '#2b2b2b' --at 33,36 --anchor tl --anchor-to ink
pixelox text add sheet.pxo --in-place --name Title --content "What is at stake" --font @geometric-sans --weight 400 --size 51 --tracking=-20 --color '#f7941d' --at 33,66 --anchor tl --anchor-to ink
pixelox text add sheet.pxo --in-place --name Deck --content $'Voters choose a third of the upper house and every seat\nin the lower house on the first Sunday of June.' --font @geometric-sans --size 25 --leading 31 --color '#2b2b2b' --at 33,124 --anchor tl --anchor-to ink
pixelox render js scripts/hemicycle.js -o arch.png --size 1080x1100 --param cx=48.4 --param cy=38 --param r=20.2 --param rows=5 --param inner=0.5 --param shape=hex --param seats=27,15,3,14,31 --param colors='#2d368d,#b9e2fa,#f0a431,#f0d6d9,#cb0303' --param hollow=3 --param counts=0 --param legend=none
pixelox comp place sheet.pxo arch.png --in-place --as Arch --at 0,0
pixelox render js scripts/hemicycle.js -o archlines.png --size 1080x1100 --param cx=48.4 --param cy=38 --param r=20.2 --param rows=5 --param inner=0.5 --param shape=hex --param seats=27,15,3,14,31 --param colors='none,#9a9a9a,none,#9a9a9a,none' --param hollow=2,4 --param counts=0 --param legend=none
pixelox comp place sheet.pxo archlines.png --in-place --as ArchOutlines --at 0,0
pixelox shape path sheet.pxo --in-place --name ArchGuide --d 'M 290 418 A 233 233 0 0 1 756 418' --fill none --stroke '#8a8a8a' --stroke-width 1.5 --dash 2,4
pixelox text add sheet.pxo --in-place --name Name1 --content "Upper house" --font @oldstyle --style italic --weight 700 --size 27 --color '#333333' --at 523,382 --anchor c
pixelox text add sheet.pxo --in-place --name Seats1 --content "90 SEATS" --font @geometric-sans --weight 500 --size 17 --tracking 20 --color '#555555' --at 523,408 --anchor c
pixelox text add sheet.pxo --in-place --name Big1 --content "29" --font @geometric-sans --weight 900 --size 58 --tracking=-20 --color '#333333' --at 177,306 --anchor c --anchor-to ink
pixelox text add sheet.pxo --in-place --name Big1Cap --content "seats will be contested" --font @geometric-sans --size 18 --color '#555555' --at 177,348 --anchor c
pixelox text add sheet.pxo --in-place --name F1b --content "4 members" --font @geometric-sans --weight 700 --size 18 --color '#2b2b2b' --at 785,262 --anchor l
pixelox text add sheet.pxo --in-place --name F1r --content "sit for every region" --font @geometric-sans --size 18 --color '#555555' --at 785,286 --anchor l
pixelox text add sheet.pxo --in-place --name F2b --content "40 or older" --font @geometric-sans --weight 700 --size 18 --color '#2b2b2b' --at 785,322 --anchor l
pixelox text add sheet.pxo --in-place --name F2r --content "is the rule for candidates" --font @geometric-sans --size 18 --color '#555555' --at 785,346 --anchor l
pixelox text add sheet.pxo --in-place --name F3b --content "8-year term" --font @geometric-sans --weight 700 --size 18 --color '#2b2b2b' --at 785,382 --anchor l
pixelox text add sheet.pxo --in-place --name F3r --content "for each member" --font @geometric-sans --size 18 --color '#555555' --at 785,406 --anchor l
pixelox shape line sheet.pxo --in-place --name Rule1 --from 70,443 --to 1012,443 --stroke '#dcdcdc' --stroke-width 2
pixelox text add sheet.pxo --in-place --name Name2 --content "Lower house" --font @oldstyle --style italic --weight 700 --size 27 --color '#333333' --at 523,463 --anchor c
pixelox render js scripts/waffle.js -o block.png --size 1080x1100 --param box=8.3,44.3,83.6,20.5 --param values=212,2,186 --param cols=40 --param order=cols --param shape=hex --param gap=0.1 --param colors='#b9e2fa,#feff9a,#f0d6d9' --param legend=none
pixelox comp place sheet.pxo block.png --in-place --as Block --at 0,0
pixelox render js scripts/waffle.js -o blocklines.png --size 1080x1100 --param box=8.3,44.3,83.6,20.5 --param values=212,2,186 --param cols=40 --param order=cols --param shape=hex --param gap=0.1 --param colors='#9a9a9a' --param hollow=1,2,3 --param legend=none
pixelox comp place sheet.pxo blocklines.png --in-place --as BlockOutlines --at 0,0
pixelox shape rect sheet.pxo --in-place --name BlockFrame --rect 78,477,927x246 --fill none --stroke '#8a8a8a' --stroke-width 1.5 --dash 2,4
pixelox render js scripts/waffle.js -o vacant.png --size 1080x1100 --param box=42.8,66.3,14.4,1.9 --param values=6 --param cols=6 --param shape=hex --param gap=0.1 --param colors='#8a8a8a' --param hollow=1 --param legend=none
pixelox comp place sheet.pxo vacant.png --in-place --as Vacant --at 0,0
pixelox shape path sheet.pxo --in-place --name VacantTab --d 'M 452 723 L 452 757 L 628 757 L 628 723' --fill none --stroke '#8a8a8a' --stroke-width 1.5 --dash 2,4
pixelox text add sheet.pxo --in-place --name All --content "All" --font @geometric-sans --size 18 --color '#555555' --at 296,797 --anchor r
pixelox text add sheet.pxo --in-place --name Big2 --content "406" --font @geometric-sans --weight 900 --size 58 --tracking=-20 --color '#333333' --at 362,787 --anchor c --anchor-to ink
pixelox text add sheet.pxo --in-place --name Big2Cap --content "seats will be contested" --font @geometric-sans --size 18 --color '#555555' --at 337,826 --anchor c
pixelox text add sheet.pxo --in-place --name G2b --content $'3 years\n21\n1 seat' --font @geometric-sans --weight 700 --size 19 --leading 29 --align right --color '#2b2b2b' --at 536,796 --anchor r
pixelox text add sheet.pxo --in-place --name G2r --content $'is one term for every member\nis the youngest a candidate may be\nfor about 90,000 residents' --font @geometric-sans --size 19 --leading 29 --color '#555555' --at 546,796 --anchor l
pixelox shape line sheet.pxo --in-place --name Rule2 --from 70,857 --to 1012,857 --stroke '#dcdcdc' --stroke-width 2
pixelox text add sheet.pxo --in-place --name Name3 --content "Regional races" --font @oldstyle --style italic --weight 700 --size 27 --color '#333333' --at 523,892 --anchor c
pixelox text add sheet.pxo --in-place --name Big3 --content "8" --font @geometric-sans --weight 900 --size 68 --color '#333333' --at 392,953 --anchor c --anchor-to ink
pixelox text add sheet.pxo --in-place --name S3a --content "regional leaders are chosen" --font @geometric-sans --size 25 --color '#555555' --at 435,937 --anchor l
pixelox text add sheet.pxo --in-place --name S3b --content "in 5 regions and 3 territories" --font @geometric-sans --weight 700 --size 25 --color '#2b2b2b' --at 435,966 --anchor l
pixelox text add sheet.pxo --in-place --name S3c --content "(the three island territories)" --font @oldstyle --style italic --weight 400 --size 19 --color '#8a8a8a' --at 435,996 --anchor l
pixelox shape rect sheet.pxo --in-place --name LegendBox --rect 824,872,232x176 --fill none --stroke '#8a8a8a' --stroke-width 1
pixelox render js scripts/waffle.js -o keys.png --size 1080x1100 --param box=77.4,80.27,1.6,14 --param values=1,1,1,1,1,1,1 --param cols=1 --param shape=hex --param gap=0.4375 --param colors='#2d368d,#b9e2fa,#f0d6d9,#cb0303,#f0a431,#feff9a,#8a8a8a' --param hollow=5,7 --param legend=none
pixelox comp place sheet.pxo keys.png --in-place --as Keys --at 0,0
pixelox render js scripts/waffle.js -o keylines.png --size 1080x1100 --param box=77.4,80.27,1.6,14 --param values=1,1,1,1,1,1,1 --param cols=1 --param shape=hex --param gap=0.4375 --param colors='none,#9a9a9a,#9a9a9a,none,none,#9a9a9a,none' --param hollow=2,3,6 --param legend=none
pixelox comp place sheet.pxo keylines.png --in-place --as KeyOutlines --at 0,0
pixelox text add sheet.pxo --in-place --name KeyText --content $'Bloc A, not up\nBloc A, seat up\nBloc B, seat up\nBloc B, not up\nIndependent, not up\nIndependent, seat up\nVacant' --font @geometric-sans --size 15 --leading 23 --color '#555555' --at 864,960 --anchor l
pixelox text add sheet.pxo --in-place --name Mark --content "YOUR MARK" --font @geometric-sans --weight 800 --size 18 --tracking 60 --color '#555555' --at 33,1062 --anchor l
pixelox text add sheet.pxo --in-place --name Source --content "Source: Example Electoral Office. Example data, updated 14 May" --font @geometric-sans --weight 500 --size 16 --color '#8a8a8a' --at 190,1062 --anchor l
pixelox export sheet.pxo -o sheet.png
```

Reuse: change `seats=` (keep the five groups in sitting order: A dark, A pale, others, B
pale, B dark) and the block's `values=`; pick `cols=` so the block fills whole columns
(block total / 10 here) and put the remainder in the vacant tab. Every outline run repeats
its chart's geometry and data exactly; only `colors=` and `hollow=` differ. The vacant
run's box height is one unit of the block (read `unit` in the block's log: 20.6 px here =
1.9% H) so its hexagons match, and its box is `cols=` units wide. The dotted arc is a half
circle on the arch's center (523, 418) with a radius 15 px larger than the arch's. The
legend text is centered on the key column's middle (y 960) with a leading equal to the key
pitch (unit x (1 + gap) = 23 px). In the facts, a bold lead (figure) and a regular line are
two layers; the second chamber's facts are two three-line layers, bold figures
right-aligned against x 536 and the sentences starting at 546.

## 13. Variations and pairings

* **After the vote:** the same sheet with the pale seats recolored dark for the winner;
  i004 and i003 are the results versions (solid arc band for the large chamber, dots for the
  small one, dark ground in i003). This sheet is their "before" relative: it adds the
  dark-and-pale status coding and replaces the arc band with a seat block.
* **One chamber:** keep the title block and the arch row, enlarge the arch to `r=30`, drop
  the block; a 4:3 card.
* **Other counted bodies:** board seats up for renewal, contracts expiring this year, fleet
  vehicles due for replacement: dark = fixed, pale = open, hue = owner.
* **Dark ground:** `#16233b` with the pale steps kept and the dark steps brightened
  (`#5b6fe0`, `#f0524f`); the outline runs become unnecessary.
* Pairs with C-tile-map (which regions hold a race) in place of the third module, and with
  C-percent-bar for the share of seats at stake.

## 14. Pitfalls

* **Pale tints that vanish on white.** `#f0d6d9` on `#ffffff` has almost no contrast; the
  original saves it with a gray outline on every pale seat, and so does the build (the
  outline runs). If you drop them, darken the tints one step (`#9fd3f5`, `#eab9bf`) or put
  the charts on a `#f4f4f2` panel.
* **Seat counts that do not match the numbers beside them.** The hero number (29) must equal
  the pale seats in the arch (15 + 14) and the block's groups plus the vacant tab must equal
  the stated total (212 + 2 + 186 + 6 = 406). Check the `total` each script logs. This is the
  data-integrity check for every seat chart.
* **An outline run that drifts from its chart.** The outlines only land on the seats when
  both runs have the same `seats=` or `values=`, box, and form params; change the data in
  one and the other shows outlines around the wrong seats.
* **Different orders in the two charts.** Bloc A is on the left in both the arch and the
  block; swapping one of them makes readers compare the wrong sides.
* **A legend that explains too little.** The original labels both pale colors with the
  same phrase and lets the hue tell the bloc, in 12 px type: the smallest text on the sheet
  and the one a first-time reader needs most. Name the bloc in every key and keep keys at
  15 px or more, or label the seat groups directly.
* **A thin orange title on white** drops below readable contrast at small sizes; keep it at
  48 px or more, or use weight 500.
