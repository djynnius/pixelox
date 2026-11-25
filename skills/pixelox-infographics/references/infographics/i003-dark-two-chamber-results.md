---
id: i003
title: Dark two-chamber results sheet with a ring hero
kind: [results, statistical]
topic: [politics]
mood: [dark, authoritative, serious, bold]
style: [flat, editorial]
format: long
structure: S-results-sheet
data: [counted-units, part-to-whole, single-stat]
charts: [C-half-donut, C-hemicycle, C-tick-ring, C-arc-label, C-stat-row, C-icon-badge, C-source-line]
palette: ['#122e39', '#0e2630', '#b3261c', '#1466a3', '#aeb4b3', '#e9e3a6', '#ffffff']
needs: nothing
---

# i003: Dark two-chamber results sheet with a ring hero

## 1. Concept

A tall results sheet on a deep blue-green ground. One large circle holds a whole parliament:
the lower house as a solid arc band across the top half, the upper house as a bowl of seat
dots across the bottom half, and a building in the hub between them. Under the circle come a
poll shown as a tick ring with four small facts in the corners, then a row of four giant
numbers. It works because the two chambers share one center and one pair of party colors, so
the balance of power is read in a single glance before any number is read, and every module
below repeats those two colors.

## 2. Data story

* **Two seat counts, same parties, different sizes.** The lower house is a large body
  (several hundred seats) split almost evenly between two parties, plus a remainder under 1%
  (independent or vacant). It is drawn as solid wedges: at that size dots would be specks.
* The upper house is small (about a hundred seats), split roughly 5 to 4 with a handful of
  others. It is drawn one dot per seat in 5 rows: at that size every seat is visible.
* **One poll:** two options about nine points apart and a small undecided share, as a ring
  of 90 ticks.
* **Eight single numbers:** four counts of offices up for election (two to three digits) and
  four national totals in millions and billions.
* Relationships: counted units and part-to-whole for the chambers; single stats below.

Placeholder data used in the recipe (example data, invented):

| Module | Values |
| --- | --- |
| Lower house, 400 seats | Party A 212, Party B 181, others and vacant 7 |
| Upper house, 90 seats | Party A 47, Party B 39, others 4 |
| Poll | Party B 51%, undecided 8%, Party A 41% |
| Corner facts | 400, 30, 64, 22 |
| Stat row | 180M, 126M, 48M+, $7B+ |

## 3. Information architecture

1. **Title** (two centered lines): the subject and the moment.
2. **Hero ring**: who holds each chamber. The eye lands on the two colored wedges and their
   seat counts, drops through the building to the dots, and reads the chamber names on the
   curved labels at the top and bottom of the ring.
3. **What is next** (a slightly darker band): a question as a caps heading, the poll ring in
   the middle with the two percentages beside it, and four facts in the corners.
4. **Scale** (stat row): four giant numbers with captions.
5. **Footnotes and sources**: two small blocks, left-aligned.

Modules are stacked on one centered axis; only the footnotes hang from the left margin.

## 4. Format and grid

* Long portrait, 1 : 2.11. Recipe canvas 1080 x 2280. Side margins 6% W.
* Title: centered, cap tops at 5.6% and 8.9% H.
* Hero ring: one center at 50% W, 38% H. Dark disc radius 47% W; arc band outer radius 43.4%
  W, inner 21% W; seat dots between 20.4% and 43.4% W; hub radius 18% W. The ring module runs
  from 11% to 61% H.
* Middle band: `0,63.2%,100% x 21%`. Heading at 64.6% H; poll ring center 50% W, 76.8% H,
  radius 14.8% W; corner facts in two columns centered on 14% and 86% W, rows at 70% and
  78.8% H.
* Stat row: four columns centered on 14.5%, 38%, 61.5%, 85.5% W; numbers at 87.1% H,
  captions from 89.1% H.
* Footnotes at 94.3% H, sources at 97.2% H, both at 6% W.

## 5. Layer stack

1. Ground: `#122e39`, flat.
2. Rays: 36 faint white wedges (3.5% opacity at the center, fading out) from the ring's
   center: barely visible, they aim the page at the hero.
3. Dark disc `#0e2630`, radius 47% W, behind the whole ring.
4. Arc band: three wedges over the top of the ring, from 86 degrees left of the top to 74
   degrees right of it: red `#b3261c` (the larger party, left), blue `#1466a3` (right), and a
   gray sliver `#aeb4b3` at the right end. Hairline gaps between wedges. The seat count sits
   in each wedge in white, 64 px, extra bold, with the party name under it.
5. Seat dots: one dot per seat (90 in the recipe) in 5 rows as a bowl under the band, 150
   degrees wide: red on the left, blue on the right, a few gray at the right end. Dot diameter about 3.5% W.
6. Hub: a disc `#16353f`, radius 18% W, holding a white building mark (the original has a
   photographic white domed building here, cut out, about 40% W wide).
7. Two gold five-point stars `#e2c12f` on the ring's horizontal line at 17.6% and 82.4% W.
8. Curved labels: the chamber name in cream caps and a white info line, on arcs above the
   band; the same pair mirrored under the dots.
9. A small "others" count with a two-line caption outside the band's right end.
10. Middle band `#0f2832`, heading, poll tick ring (blue from the top clockwise, a gray gap,
    then red), question in the hole, percentages left and right.
11. Four corner facts: an outlined rounded square with a line icon, a cream number, a caption.
12. Stat row: four numbers and captions.
13. Footnotes and sources.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#122e39` | ground | 60% | nothing |
| `#0e2630` | hero disc, middle band (`#0f2832`) | 22% | nothing: groups a module |
| `#b3261c` | Party A: wedge, dots (`#c8322b`), ticks | 7% | category: party A |
| `#1466a3` | Party B: wedge, dots (`#1f77b8`), ticks | 7% | category: party B |
| `#aeb4b3` | others, vacant, undecided | 1% | category: neither |
| `#e9e3a6` | chamber names, corner numbers | 1% | emphasis: names and counts of bodies |
| `#ffffff` | title, counts, captions | 2% | nothing |

Categorical, two hues plus gray, on a dark neutral ground. The same red and blue appear in
every module with the same meaning (two-party red and blue convention, US; swap for the
colors your parties use). Gold appears only as two stars.

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Title | 2 lines, 3 and 6 words | sentence | geometric sans (Poppins, Montserrat / Avenir Next) | 700 | 62 | 0 | line pitch 75 px | center | `#ffffff` | 50%, cap tops 5.6% and 8.9% |
| Chamber name (x2) | 2 words | caps | geometric sans | 800 | 36 | 20 | - | on an arc, centered | `#e9e3a6` | arcs of radius 49.5% W above and below the ring |
| Chamber info (x2) | "Total seats: N / term length", 6 to 8 words | sentence | geometric sans | 400 | 24 | 0 | - | on an arc, centered | `#ffffff` | radius 46% W above; 53% W below |
| Wedge count (x2) | 3 digits | - | geometric sans | 800 | 64 | 0 | - | center | `#ffffff` | middle of each wedge |
| Wedge party (x2) | 1 to 2 words | caps | geometric sans | 700 | 24 | 40 | - | center | `#ffffff` | 54 px under each count |
| Others count and caption | 1 digit; 3 words on 2 lines | sentence | geometric sans | 800 / 400 | 44 / 18 | 0 | 120% | left | `#ffffff` | 85.6% W, 34.5% H |
| Dot labels (x2) | party and count | caps | geometric sans | 700 | 26 | 0 | - | center | `#ffffff` | 13.9% and 86.1% W, 55.4% H |
| Band heading | a question, 6 to 8 words on 2 lines | caps | geometric sans | 800 | 30 | 0 | 118% | center | `#ffffff` | 50%, 64.6% |
| Poll question | 5 to 7 words on 3 lines | caps | geometric sans | 800 | 21 | 0 | 122% | center | `#ffffff` | ring center |
| Poll share (x2) | NN% and a 1 to 2 word label | caps label | geometric sans | 800 / 400 | 34 / 18 | 0 / 40 | - | center | `#ffffff` | 27.8% and 72.2% W, 76.6% H |
| Corner number (x4) | 2 to 3 digits | - | geometric sans | 800 | 40 | 0 | - | center | `#e9e3a6` | under each icon |
| Corner caption (x4) | 1 to 3 words, 1 or 2 lines | sentence | geometric sans | 400 | 21 | 0 | 125% | center | `#ffffff` | under each number |
| Stat number (x4) | 3 to 5 characters with unit letter | caps | geometric sans | 800 | 64 | 0 | - | center | `#ffffff` and `#e9e3a6`, alternating (the original: white, red, blue, gray) | 87.1% H |
| Stat caption (x4) | 5 to 8 words on 2 or 3 lines | sentence | geometric sans | 400 | 21 | 0 | 128% | center | `#ffffff` | from 89.1% H |
| Footnotes | 2 or 3 lines with * marks | sentence | geometric sans | 400 | 16 | 0 | 135% | left | `#c4cdd0` | 6%, 94.3% |
| Sources | "Sources:" and a list | sentence | geometric sans | 400 | 16 | 0 | - | left | `#ffffff` | 6%, 97.2% |

One family in two weights (800 for every number and heading, 400 for captions). Sizes 64 : 36
: 24 : 16, about 4 : 2.2 : 1.5 : 1.

## 8. Charts and diagrams

* **C-half-donut** (arc band), `scripts/donut.js` placed by center: `cx=50 cy=38 r=43.4`,
  `sweep=160 start=-86`, `inner=0.485`, `labelpos=in show=value`, `sep=none gap=0.7`,
  `fs=64`. No axis, no legend: counts are written in the wedges, names under them. The
  original writes count, share, and party name along the arc inside each wedge; upright
  labels are used here.
* **C-hemicycle**, `scripts/hemicycle.js` on the same center and radius, `flip=1`, `rows=5`,
  `inner=0.47`, `sweep=150`, groups left to right: Party A, Party B, others. No legend:
  two labels under the ends of the bowl.
* **C-tick-ring**, `scripts/donut.js` with `ticks=90`, `inner=0.72`, `cx=50 cy=76.8 r=14.8`,
  `labelpos=none`: the shares are written beside the ring with `text add`.
* **C-arc-label** for the four curved lines; **C-stat-row** for the four totals;
  **C-icon-badge** (outlined square variant, 60 px, 2 px white line) for the corner facts.

## 9. Icons and illustration

* Hub: `landmark`, 250 px, white, 13 px line, on the hub disc. The original uses a cut-out
  photo of a white domed government building with a small name plate; place your own
  `illustration.png` at 40% W on the same center if you have one.
* Corner facts: `landmark`, `scale`, `building`, `map`, 36 px, white, each in a 60 px
  outlined square with 10 px corners.
* Party marks: the original puts small party mascots beside the names and the poll shares;
  leave them out or use your parties' own marks. Do not invent emblems.
* Two gold stars (`shape star`, 5 points, outer radius 24 px).

## 10. Effects and finish

* Rays: `scripts/rays.js`, 36 wedges, `alpha=0.035`, centered on the ring. Anything stronger
  fights the wedges.
* No shadows, no gradients, no outlines on the charts: flat color on a dark ground.
* Hairline gaps between wedges (`gap=0.7`) instead of white separators.

## 11. Hierarchy and eye path

1. The red and blue wedges and their two counts (the largest colored areas, at the top).
2. Down through the hub to the dots; the chamber names on the arcs.
3. The band heading, the tick ring, the two percentages.
4. The stat row (the only other 64 px type).
5. Corner facts, then the fine print.

Counts and stat numbers 64 px, chamber names 36, captions 21 to 24, fine print 16.

## 12. Build recipe

Nothing to supply. Canvas 1080 x 2280. Example data throughout.

```bash
pixelox new -o sheet.pxo --size 1080x2280 --fill '#122e39'
pixelox render js scripts/rays.js -o rays.png --size 1080x2280 --param color='#ffffff' --param count=36 --param cx=0.5 --param cy=0.38 --param alpha=0.035 --param fade=1
pixelox comp place sheet.pxo rays.png --in-place --as Rays --at 0,0
pixelox shape ellipse sheet.pxo --in-place --name Disc --rect 32,358,1016x1016 --fill '#0e2630'
pixelox render js scripts/donut.js -o band.png --size 1080x2280 --param cx=50 --param cy=38 --param r=43.4 --param sweep=160 --param start=-86 --param inner=0.485 --param values=212,181,7 --param colors='#b3261c,#1466a3,#aeb4b3' --param labelpos=in --param show=value --param sep=none --param gap=0.7 --param fs=64 --param font='Avenir Next, Poppins, Montserrat, Century Gothic, Arial, sans-serif' --param numweight=800
pixelox comp place sheet.pxo band.png --in-place --as Band --at 0,0
pixelox render js scripts/hemicycle.js -o seats.png --size 1080x2280 --param cx=50 --param cy=38 --param r=43.4 --param flip=1 --param rows=5 --param inner=0.47 --param sweep=150 --param seats=47,39,4 --param colors='#c8322b,#1f77b8,#aeb4b3'
pixelox comp place sheet.pxo seats.png --in-place --as Seats --at 0,0
pixelox shape ellipse sheet.pxo --in-place --name Hub --rect 345,671,390x390 --fill '#16353f'
pixelox shape icon sheet.pxo landmark --in-place --name Building --size 250 --at 540,850 --anchor c --color '#f6f8f8' --stroke-width 13
pixelox shape star sheet.pxo --in-place --name StarL --center 190,866 --points 5 --outer 24 --fill '#e2c12f'
pixelox shape star sheet.pxo --in-place --name StarR --center 890,866 --points 5 --outer 24 --fill '#e2c12f'
pixelox text add sheet.pxo --in-place --name T1 --content "A divided parliament" --font @geometric-sans --weight 700 --size 62 --color '#ffffff' --at 50%,5.6% --anchor t --anchor-to ink
pixelox text add sheet.pxo --in-place --name T2 --content "and the road to the next vote" --font @geometric-sans --weight 700 --size 62 --color '#ffffff' --at 50%,8.9% --anchor t --anchor-to ink
pixelox text add sheet.pxo --in-place --name ArcTitle --content "LOWER HOUSE" --font @geometric-sans --weight 800 --size 36 --tracking 20 --color '#e9e3a6' --align center
pixelox text on-path sheet.pxo --in-place --layer ArcTitle --d 'M 5 866 A 535 535 0 0 1 1075 866' --offset 50%
pixelox text add sheet.pxo --in-place --name ArcInfo --content "Total seats: 400 / Elected for 4 years" --font @geometric-sans --size 24 --color '#ffffff' --align center
pixelox text on-path sheet.pxo --in-place --layer ArcInfo --d 'M 42 866 A 498 498 0 0 1 1038 866' --offset 50%
pixelox text add sheet.pxo --in-place --name BowlTitle --content "UPPER HOUSE" --font @geometric-sans --weight 800 --size 36 --tracking 20 --color '#e9e3a6' --align center
pixelox text on-path sheet.pxo --in-place --layer BowlTitle --d 'M 5 866 A 535 535 0 0 0 1075 866' --offset 50%
pixelox text add sheet.pxo --in-place --name BowlInfo --content "Total seats: 90 / Elected for 5 years" --font @geometric-sans --size 24 --color '#ffffff' --align center
pixelox text on-path sheet.pxo --in-place --layer BowlInfo --d 'M -32 866 A 572 572 0 0 0 1112 866' --offset 50%
pixelox text add sheet.pxo --in-place --name LabA --content "PARTY A" --font @geometric-sans --weight 700 --size 24 --tracking 40 --color '#ffffff' --at 300,668 --anchor c
pixelox text add sheet.pxo --in-place --name LabB --content "PARTY B" --font @geometric-sans --weight 700 --size 24 --tracking 40 --color '#ffffff' --at 740,635 --anchor c
pixelox text add sheet.pxo --in-place --name Other --content "7" --font @geometric-sans --weight 800 --size 44 --color '#ffffff' --at 924,786 --anchor l --anchor-to ink
pixelox text add sheet.pxo --in-place --name OtherL --content $'others and\nvacant' --font @geometric-sans --size 18 --leading 120% --color '#ffffff' --at 924,812 --anchor tl
pixelox text add sheet.pxo --in-place --name SeatsA --content "PARTY A  47" --font @geometric-sans --weight 700 --size 26 --color '#ffffff' --at 150,1262 --anchor c
pixelox text add sheet.pxo --in-place --name SeatsB --content "PARTY B  39" --font @geometric-sans --weight 700 --size 26 --color '#ffffff' --at 930,1262 --anchor c
pixelox shape rect sheet.pxo --in-place --name Mid --rect 0,1440,1080x480 --fill '#0f2832'
pixelox text add sheet.pxo --in-place --name H2 --content $'WHAT IS ON THE BALLOT\nTHIS YEAR?' --font @geometric-sans --weight 800 --size 30 --leading 118% --align center --color '#ffffff' --at 50%,1474 --anchor t
pixelox render js scripts/donut.js -o ticks.png --size 1080x2280 --param cx=50 --param cy=76.8 --param r=14.8 --param values=51,8,41 --param colors='#1f77b8,#aeb4b3,#c8322b' --param ticks=90 --param inner=0.72 --param labelpos=none
pixelox comp place sheet.pxo ticks.png --in-place --as Ticks --at 0,0
pixelox text add sheet.pxo --in-place --name Q --content $'WHICH PARTY\nDO VOTERS\nPREFER?' --font @geometric-sans --weight 800 --size 21 --leading 122% --align center --color '#ffffff' --at 540,1751 --anchor c
pixelox text add sheet.pxo --in-place --name PA --content "41%" --font @geometric-sans --weight 800 --size 34 --color '#ffffff' --at 300,1746 --anchor c
pixelox text add sheet.pxo --in-place --name PAl --content "PARTY A" --font @geometric-sans --size 18 --tracking 40 --color '#ffffff' --at 300,1782 --anchor c
pixelox text add sheet.pxo --in-place --name PB --content "51%" --font @geometric-sans --weight 800 --size 34 --color '#ffffff' --at 780,1746 --anchor c
pixelox text add sheet.pxo --in-place --name PBl --content "PARTY B" --font @geometric-sans --size 18 --tracking 40 --color '#ffffff' --at 780,1782 --anchor c
pixelox shape rect sheet.pxo --in-place --name K1 --rect 121,1570,60x60 --radius 10 --fill none --stroke '#ffffff' --stroke-width 2
pixelox shape icon sheet.pxo landmark --in-place --name K1i --size 36 --at 151,1600 --anchor c --color '#ffffff'
pixelox text add sheet.pxo --in-place --name K1n --content "400" --font @geometric-sans --weight 800 --size 40 --color '#e9e3a6' --at 151,1664 --anchor c
pixelox text add sheet.pxo --in-place --name K1c --content $'Lower house\nseats' --font @geometric-sans --size 21 --leading 125% --align center --color '#ffffff' --at 151,1692 --anchor t
pixelox shape rect sheet.pxo --in-place --name K2 --rect 121,1768,60x60 --radius 10 --fill none --stroke '#ffffff' --stroke-width 2
pixelox shape icon sheet.pxo scale --in-place --name K2i --size 36 --at 151,1798 --anchor c --color '#ffffff'
pixelox text add sheet.pxo --in-place --name K2n --content "30" --font @geometric-sans --weight 800 --size 40 --color '#e9e3a6' --at 151,1862 --anchor c
pixelox text add sheet.pxo --in-place --name K2c --content "Upper house seats" --font @geometric-sans --size 21 --color '#ffffff' --at 151,1890 --anchor t
pixelox shape rect sheet.pxo --in-place --name K3 --rect 899,1570,60x60 --radius 10 --fill none --stroke '#ffffff' --stroke-width 2
pixelox shape icon sheet.pxo building --in-place --name K3i --size 36 --at 929,1600 --anchor c --color '#ffffff'
pixelox text add sheet.pxo --in-place --name K3n --content "64" --font @geometric-sans --weight 800 --size 40 --color '#e9e3a6' --at 929,1664 --anchor c
pixelox text add sheet.pxo --in-place --name K3c --content $'Regional\nassemblies' --font @geometric-sans --size 21 --leading 125% --align center --color '#ffffff' --at 929,1692 --anchor t
pixelox shape rect sheet.pxo --in-place --name K4 --rect 899,1768,60x60 --radius 10 --fill none --stroke '#ffffff' --stroke-width 2
pixelox shape icon sheet.pxo map --in-place --name K4i --size 36 --at 929,1798 --anchor c --color '#ffffff'
pixelox text add sheet.pxo --in-place --name K4n --content "22" --font @geometric-sans --weight 800 --size 40 --color '#e9e3a6' --at 929,1862 --anchor c
pixelox text add sheet.pxo --in-place --name K4c --content "Governors" --font @geometric-sans --size 21 --color '#ffffff' --at 929,1890 --anchor t
pixelox text add sheet.pxo --in-place --name S1 --content "180M" --font @geometric-sans --weight 800 --size 64 --color '#ffffff' --at 14.5%,1986 --anchor c --anchor-to ink
pixelox text add sheet.pxo --in-place --name S2 --content "126M" --font @geometric-sans --weight 800 --size 64 --color '#e9e3a6' --at 38%,1986 --anchor c --anchor-to ink
pixelox text add sheet.pxo --in-place --name S3 --content "48M+" --font @geometric-sans --weight 800 --size 64 --color '#ffffff' --at 61.5%,1986 --anchor c --anchor-to ink
pixelox text add sheet.pxo --in-place --name S4 --content '$7B+' --font @geometric-sans --weight 800 --size 64 --color '#e9e3a6' --at 85.5%,1986 --anchor c --anchor-to ink
pixelox text add sheet.pxo --in-place --name S1c --content $'People old\nenough to vote' --font @geometric-sans --size 21 --leading 128% --align center --color '#ffffff' --at 14.5%,2032 --anchor t
pixelox text add sheet.pxo --in-place --name S2c --content $'Registered\nbefore the\ndeadline' --font @geometric-sans --size 21 --leading 128% --align center --color '#ffffff' --at 38%,2032 --anchor t
pixelox text add sheet.pxo --in-place --name S3c --content $'Ballots cast\nbefore\nelection day' --font @geometric-sans --size 21 --leading 128% --align center --color '#ffffff' --at 61.5%,2032 --anchor t
pixelox text add sheet.pxo --in-place --name S4c --content $'Spent on\ncampaign ads,\na record' --font @geometric-sans --size 21 --leading 128% --align center --color '#ffffff' --at 85.5%,2032 --anchor t
pixelox text add sheet.pxo --in-place --name Notes --content $'* Half of the upper house is elected every 30 months.\n** Registration figures as of the September deadline.' --font @geometric-sans --size 16 --leading 135% --color '#c4cdd0' --at 6%,2150 --anchor tl
pixelox text add sheet.pxo --in-place --name Src --content "Sources: Example Electoral Office; Example Polling Institute. Example data." --font @geometric-sans --size 16 --color '#ffffff' --at 6%,2216 --anchor tl
pixelox export sheet.pxo -o sheet.png
```

Reuse: change `values=` and `seats=` (keep the party order left to right the same in both
charts and in `colors=`), the two wedge counts' labels, and the poll's `values=`. The label
positions under the wedge counts are the `slice1` and `slice2` anchors of the band, 54 px
lower; the "others" label sits just outside the band's right end. A chamber with more than
about 250 seats belongs in the band; a smaller one in the dots.

## 13. Variations and pairings

* **One chamber only:** keep the band (or the dots) as a plain half circle, `sweep=180`, and
  move the stat row up; the sheet becomes a 4:5 poster.
* **Coalitions:** three to six groups work in both charts; order them as they sit, left to
  right, and add a C-legend under the ring instead of wedge labels.
* **Light version:** ground `#f4f1ea`, disc `#e7e2d6`, text `#16233b`, the same red and blue.
* **Sports or business:** the same ring holds any two-level split (home and away results,
  two divisions of a company) with an emblem in the hub.
* Pairs with C-tile-map as a fourth module ("seats won by region") between the poll and the
  stat row, and with C-percent-bar for a single split.

## 14. Pitfalls

* **Different party orders in the two charts.** If Party A is on the left in the band and on
  the right in the dots, readers compare the wrong sides. Same order, same colors, everywhere.
* **Dots for a 400-seat chamber.** They shrink to 9 px and stop reading as seats. Use the
  solid band above about 250 seats, and print the count.
* **A band that does not start at the same angle on both sides** suggests a majority that is
  not there. Wedge angles must come from the script; never nudge them by eye.
* **Shares that do not add up.** The poll's three values must total 100, and each chamber's
  groups must total the stated number of seats: check the script's logged `total`.
* **Bright rays or a lighter disc.** The background then outshouts the data; keep both within
  4% lightness of the ground.
* **Party colors used for decoration.** The original sets two of the four stat numbers in
  the party red and blue for rhythm, although they are national totals; a reader can take them
  for party figures. The recipe uses white and cream instead. Color a number red or blue only
  when it counts that party.
