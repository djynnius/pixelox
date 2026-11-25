---
id: i042
title: Four hanging stat cards over three circles
kind: [statistical, explainer]
topic: [marketing, media, generic]
mood: [friendly, playful, clean, vibrant]
style: [flat, ui-cards, line-icon]
format: portrait
structure: S-stat-cards-row
data: [single-stat, part-to-whole]
charts: [C-card, C-icon-badge, C-stat-block, C-pie, C-icon-array, C-callout, C-title-block]
palette: ['#ececec', '#f15b6c', '#f86fa5', '#47b4e9', '#b3c10a', '#20558b', '#7a8696']
needs: nothing
---

# i042: Four hanging stat cards over three circles

## 1. Concept

A short, light "why it matters" sheet: a hairline title on pale gray, four tall candy-colored
cards that hang from a rule like swatches (each: a label strip, a line icon in a disc, one big
percentage, a four-line caption), and under them three circles whose captions join into one
sentence. The message is "four numbers say you need this, and here is who it is for". It works
because every card is the same machine in a different color, so the eye compares four numbers
in two seconds, and the ultra-thin type keeps eight colors from shouting.

## 2. Data story

* **Four unrelated percentages**, one per card, from four different sources and bases: two
  small ones (a quarter or less), one close to the whole, and one above 100% (a growth
  figure).
  They are single statistics, not parts of one whole, and are never compared on a common
  scale: no bars, no axis.
* **One two-part split** as a pie: about four fifths to one fifth, each slice named on a
  small gray tag.
* **One closing percentage** (a large majority) inside a plain disc.
* A three-part closing sentence of 1, 3, and 1 words, one part under each circle.

Placeholder data in the recipe (example data, invented, about bike lanes): 40% fewer
injuries, 75% of new riders, 150% more trips, 16% more sales; split 79 to 21 (ride, drive);
64% of trips are under five kilometers.

## 3. Information architecture

1. **Decoration**: a fan of short slanted stripes hangs from the top edge and points at the
   title.
2. **Title block**: one word in hairline caps, a broken rule, a subtitle of 4 words, a
   two-line deck.
3. **Card row**: four cards hang from a full-width rule. Each answers one "how much?" Read
   left to right; all four are equal.
4. **Section label**, flush left: the question the bottom row answers.
5. **Circle row**: a pictogram disc (who), a pie (the split), a number disc (how many). The
   large words under the circles read across as one sentence.

Size leads the eye: the title, then the four white numbers, then the bottom sentence.

## 4. Format and grid

* Portrait, 1 : 1.2. Recipe canvas 1080 x 1290. Side margins 3.5% W.
* Fan: `15%,0%,70% x 4.5%`, centered. Title cap top at 9.7% H; broken rule at 17.8% H;
  subtitle cap top at 19.8% H; deck 23.6% to 27% H.
* Full-width rule at 28.3% H. Cards hang from it: four columns, each 20.7% W wide, gutters
  3.3% W, at x = 3.5%, 27.4%, 51.3%, 75.2% W; card box `x,28.3%,20.7% x 38.2%` (bottom at
  66.5% H). Label strip: the top 3.4% H of each card.
* Inside a card, on its center axis: disc center at 38.7% H, diameter 13.3% W; number
  centered at 48% H; caption lines centered between 53.5% and 64% H.
* Soft shadows: ellipses 16.7% W wide centered at 70% H under each card.
* Section label at 4.2% W, 74% H.
* Circles: diameter 20.2% W, centers at 17.6%, 50.5%, 83.3% W and 85.7% H. Closing words
  centered under them at 96.9% H.

## 5. Layer stack

1. Ground `#ececec`, flat, with a fine paper grain.
2. Fan: 26 slanted bars (13 leaning right on the left half, 13 mirrored on the right), each
   about 1.6% W wide, 20 to 58 px long, longest at the center, cycling lime `#d7ea33`, coral
   `#f46374`, sky `#49b7ed`.
3. Title in pink `#f47eab`; a broken hairline rule in blue `#2ca0c5` (a short dash, a line, a
   long line, a line, a short dash); subtitle in the same pink; deck in teal `#3fa2bd`.
4. Four blurred gray ellipses (the floating shadows).
5. Four cards: coral `#f15b6c`, pink `#f86fa5`, sky `#47b4e9`, lime `#b3c10a`, square corners.
6. A label strip across the top of each card in a darker tone of its color (`#c23f50`,
   `#d7407b`, `#1d86bb`, `#869500`) with a white bold word.
7. The rule the cards hang from: 3 px, dark rose `#bc4f6f`, edge to edge, over the card tops.
8. In each card: a disc in the darker tone, a white line icon, the number, the caption.
9. Section label in pink.
10. Left circle slate `#7a8696` with three white figures; pie in navy `#20558b` with a sky
    `#42b5ee` slice; two gray tags `#a0a6ae` on elbow leaders; right circle gray `#a3a9b1`
    with a number and two caption lines.
11. Three closing word groups in light teal-blue `#4bb0cf`.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#ececec` | ground | 56% | nothing |
| `#f15b6c` | card 1 (strip and disc `#c23f50`) | 8% | card 1's topic |
| `#f86fa5` | card 2 (strip `#d7407b`); title and labels (`#f47eab`) | 9% | card 2's topic; headings |
| `#47b4e9` | card 3 (strip `#1d86bb`); pie's small slice; rule and closing words | 8% | card 3's topic; the minority slice |
| `#b3c10a` | card 4 (strip `#869500`) | 8% | card 4's topic |
| `#20558b` | pie's large slice | 2% | the majority share |
| `#7a8696` | pictogram disc; number disc (`#a3a9b1`); tags (`#a0a6ae`) | 7% | nothing: neutral holders |

Categorical: four equally loud hues, one per card, each with one darker tone for its strip
and disc. The bottom row deliberately drops to grays and blues so it reads as a different
kind of content. White is the only type color on a card.

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Title | 1 or 2 words, 10 to 12 letters | caps | thin sans (Raleway Thin, Josefin Sans Thin / Avenir Next Ultra Light) | 200 | 106 | 0 | - | center | `#f47eab` | 50%, cap top 9.7% |
| Subtitle | 4 words | caps | thin sans | 200 | 41 | 0 | - | center | `#f47eab` | 50%, cap top 19.8% |
| Deck | 18 to 24 words on 2 lines | sentence | geometric sans (Montserrat, Nunito / Avenir Next) | 500 | 21 | 0 | 130% | center | `#3fa2bd` | 50%, top 23.6% |
| Card label (x4) | 1 word | title | geometric sans | 700 | 19 | 0 | - | center | `#ffffff` | strip center, 30% H |
| Card number (x4) | NN% or NNN% | - | thin sans | 200 | 69 | 0 | - | center | `#ffffff` | card axis, 48% |
| Card caption (x4) | 7 to 9 words on 4 lines of at most 13 characters | sentence | thin sans | 400 (the original: light) | 30 | 0 | 43 px | center | `#ffffff` | card axis, block center 58.8% |
| Section label | 3 words | title | thin sans | 400 | 36 | 0 | - | left | `#f47eab` | 4.2%, 74% |
| Tag (x2) | 1 word | title | geometric sans | 700 | 19 | 0 | - | center | `#ffffff` | in the tags |
| Disc number | NN% | - | thin sans | 200 | 75 | 0 | - | center | `#ffffff` | 83.3%, 82.3% |
| Disc caption | 3 words on 2 lines | title | thin sans | 400 | 32 | 0 | 43 px | center | `#ffffff` | 83.3%, 89% |
| Closing words (x3) | 1 to 3 words each | title | thin sans | 200 | 47 | 0 | - | center | `#4bb0cf` | circle axes, 96.9% |

One thin family for everything large and a regular geometric sans for the three small roles
(deck, card labels, tags). Sizes 106 : 69 : 47 : 30 : 20, about 5 : 3.5 : 2.3 : 1.5 : 1. The
thin weight only works because nothing set in it is under 30 px.

## 8. Charts and diagrams

* **C-pie**: `scripts/donut.js` placed by center, `cx=50.5 cy=85.7 r=10.4`, `inner=0`,
  `values=79,21`, `start=90` (the large slice starts at 3 o'clock, so the small slice sits in
  the upper right), `labelpos=none sep=none`. No numbers on the pie in the original: only a
  name tag per slice. That hides the shares; the recipe keeps the tags but the closing disc
  carries the number.
* **C-callout** x2: an elbow leader (2 px, `#5d6670`) from each slice's logged anchor
  (`slice1`, `slice2`) to a gray tag 110 x 36 px with a white bold word.
* **C-icon-array**: `scripts/waffle.js`, `shape=person`, `value=3 total=3`, white, in the left
  disc, box `11,80,13.2,11.4`. The original draws one tall figure between two short ones (a
  family); the script draws three equal figures.
* **C-card** x4 without radius or shadow layer style: the shadow is a separate blurred
  ellipse 34 px under each card, so the cards seem to float above the page.
* **C-stat-block** inside each card (icon badge, number, caption), **C-icon-badge** for the
  discs, **C-title-block** (centered variant) for the head.

## 9. Icons and illustration

* Card icons: white line icons, 88 px, 6 px line, centered on a 144 px disc: `shield`,
  `bike`, `timer`, `store` in the recipe. The original shows a cloud, a plug, a postcard, and
  a seedling in a pot, in the same rounded line style.
* Left circle: three white standing figures (see C-icon-array above).
* Fan: three `shape path` layers, one per color, each a set of parallelograms 17 px wide
  slanted 24 degrees from vertical. It is decoration: any bunting, confetti, or ribbon that
  hangs from the top edge does the same job.

## 10. Effects and finish

* Floating shadows: `shape ellipse` 180 x 22 px in `#9aa0a6`, then `filter gaussian --sigma
  10` on that layer. Kept under the cards, never on them.
* Paper grain over everything: `filter grain --as-layer --amount 10`.
* No gradients, no outlines, square card corners. The darker strips and discs are flat tones.

## 11. Hierarchy and eye path

1. The title (largest, pink on gray, pointed at by the fan).
2. The four white numbers, left to right across the colored cards.
3. Icons and captions inside each card.
4. The closing sentence under the circles (47 px), then the circles above each part.
5. Deck, section label, tags.

Title 106 px, card numbers 69, closing words 47, captions 30, small labels 19 to 21.

## 12. Build recipe

Nothing to supply. Canvas 1080 x 1290. All figures are example data, invented.

```bash
pixelox new -o cards.pxo --size 1080x1290 --fill '#ececec'
pixelox shape path cards.pxo --in-place --name FanA --d 'M 160 0 L 177 0 L 186 20 L 169 20 Z M 920 0 L 903 0 L 894 20 L 911 20 Z M 241 0 L 258 0 L 272 30 L 255 30 Z M 839 0 L 822 0 L 808 30 L 825 30 Z M 322 0 L 339 0 L 357 39 L 340 39 Z M 758 0 L 741 0 L 723 39 L 740 39 Z M 403 0 L 420 0 L 442 48 L 425 48 Z M 677 0 L 660 0 L 638 48 L 655 48 Z M 484 0 L 501 0 L 527 58 L 510 58 Z M 596 0 L 579 0 L 553 58 L 570 58 Z' --fill '#d7ea33'
pixelox shape path cards.pxo --in-place --name FanB --d 'M 187 0 L 204 0 L 214 23 L 197 23 Z M 893 0 L 876 0 L 866 23 L 883 23 Z M 268 0 L 285 0 L 300 33 L 283 33 Z M 812 0 L 795 0 L 780 33 L 797 33 Z M 349 0 L 366 0 L 385 42 L 368 42 Z M 731 0 L 714 0 L 695 42 L 712 42 Z M 430 0 L 447 0 L 470 52 L 453 52 Z M 650 0 L 633 0 L 610 52 L 627 52 Z' --fill '#f46374'
pixelox shape path cards.pxo --in-place --name FanC --d 'M 214 0 L 231 0 L 243 26 L 226 26 Z M 866 0 L 849 0 L 837 26 L 854 26 Z M 295 0 L 312 0 L 328 36 L 311 36 Z M 785 0 L 768 0 L 752 36 L 769 36 Z M 376 0 L 393 0 L 413 45 L 396 45 Z M 704 0 L 687 0 L 667 45 L 684 45 Z M 457 0 L 474 0 L 499 55 L 482 55 Z M 623 0 L 606 0 L 581 55 L 598 55 Z' --fill '#49b7ed'
pixelox text add cards.pxo --in-place --name Title --content "CITY CYCLING" --font @thin-sans --weight 200 --size 106 --color '#f47eab' --at 50%,125 --anchor t --anchor-to ink
pixelox shape path cards.pxo --in-place --name Rule --d 'M 36 230 L 50 230 M 58 230 L 246 230 M 254 230 L 826 230 M 834 230 L 1022 230 M 1030 230 L 1044 230' --fill none --stroke '#2ca0c5' --stroke-width 2
pixelox text add cards.pxo --in-place --name Sub --content "WHY CITIES BUILD LANES" --font @thin-sans --weight 200 --size 41 --color '#f47eab' --at 50%,255 --anchor t --anchor-to ink
pixelox text add cards.pxo --in-place --name Deck --content $'A protected lane changes who rides, how fast a street moves,\nand what the shops along it earn.' --font @geometric-sans --weight 500 --size 21 --leading 130% --align center --color '#3fa2bd' --at 50%,304 --anchor t
pixelox shape ellipse cards.pxo --in-place --name Sh1 --rect 60,892,180x22 --fill '#9aa0a6'
pixelox shape ellipse cards.pxo --in-place --name Sh2 --rect 318,892,180x22 --fill '#9aa0a6'
pixelox shape ellipse cards.pxo --in-place --name Sh3 --rect 576,892,180x22 --fill '#9aa0a6'
pixelox shape ellipse cards.pxo --in-place --name Sh4 --rect 834,892,180x22 --fill '#9aa0a6'
pixelox filter gaussian cards.pxo --in-place --layer Sh1 --sigma 10
pixelox filter gaussian cards.pxo --in-place --layer Sh2 --sigma 10
pixelox filter gaussian cards.pxo --in-place --layer Sh3 --sigma 10
pixelox filter gaussian cards.pxo --in-place --layer Sh4 --sigma 10
pixelox shape rect cards.pxo --in-place --name Card1 --rect 38,365,224x493 --fill '#f15b6c'
pixelox shape rect cards.pxo --in-place --name Card2 --rect 296,365,224x493 --fill '#f86fa5'
pixelox shape rect cards.pxo --in-place --name Card3 --rect 554,365,224x493 --fill '#47b4e9'
pixelox shape rect cards.pxo --in-place --name Card4 --rect 812,365,224x493 --fill '#b3c10a'
pixelox shape rect cards.pxo --in-place --name Head1 --rect 38,365,224x44 --fill '#c23f50'
pixelox shape rect cards.pxo --in-place --name Head2 --rect 296,365,224x44 --fill '#d7407b'
pixelox shape rect cards.pxo --in-place --name Head3 --rect 554,365,224x44 --fill '#1d86bb'
pixelox shape rect cards.pxo --in-place --name Head4 --rect 812,365,224x44 --fill '#869500'
pixelox shape line cards.pxo --in-place --name TopRule --from 0,365 --to 1080,365 --stroke '#bc4f6f' --stroke-width 3
pixelox text add cards.pxo --in-place --name HT1 --content "Safety" --font @geometric-sans --weight 700 --size 19 --color '#ffffff' --at 150,387 --anchor c
pixelox text add cards.pxo --in-place --name HT2 --content "Riders" --font @geometric-sans --weight 700 --size 19 --color '#ffffff' --at 408,387 --anchor c
pixelox text add cards.pxo --in-place --name HT3 --content "Journeys" --font @geometric-sans --weight 700 --size 19 --color '#ffffff' --at 666,387 --anchor c
pixelox text add cards.pxo --in-place --name HT4 --content "Trade" --font @geometric-sans --weight 700 --size 19 --color '#ffffff' --at 924,387 --anchor c
pixelox shape ellipse cards.pxo --in-place --name Disc1 --rect 78,427,144x144 --fill '#c54959'
pixelox shape ellipse cards.pxo --in-place --name Disc2 --rect 336,427,144x144 --fill '#d94584'
pixelox shape ellipse cards.pxo --in-place --name Disc3 --rect 594,427,144x144 --fill '#1d87ba'
pixelox shape ellipse cards.pxo --in-place --name Disc4 --rect 852,427,144x144 --fill '#879501'
pixelox shape icon cards.pxo shield --in-place --name Ic1 --size 88 --at 150,499 --anchor c --color '#ffffff' --stroke-width 6
pixelox shape icon cards.pxo bike --in-place --name Ic2 --size 88 --at 408,499 --anchor c --color '#ffffff' --stroke-width 6
pixelox shape icon cards.pxo timer --in-place --name Ic3 --size 88 --at 666,499 --anchor c --color '#ffffff' --stroke-width 6
pixelox shape icon cards.pxo store --in-place --name Ic4 --size 88 --at 924,499 --anchor c --color '#ffffff' --stroke-width 6
pixelox text add cards.pxo --in-place --name N1 --content "40%" --font @thin-sans --weight 200 --size 69 --color '#ffffff' --at 150,619 --anchor c --anchor-to ink
pixelox text add cards.pxo --in-place --name N2 --content "75%" --font @thin-sans --weight 200 --size 69 --color '#ffffff' --at 408,619 --anchor c --anchor-to ink
pixelox text add cards.pxo --in-place --name N3 --content "150%" --font @thin-sans --weight 200 --size 69 --color '#ffffff' --at 666,619 --anchor c --anchor-to ink
pixelox text add cards.pxo --in-place --name N4 --content "16%" --font @thin-sans --weight 200 --size 69 --color '#ffffff' --at 924,619 --anchor c --anchor-to ink
pixelox text add cards.pxo --in-place --name C1 --content $'Fewer injuries\non streets\nwith a safe\nbike lane' --font @thin-sans --weight 400 --size 30 --leading 43 --align center --color '#ffffff' --at 150,758 --anchor c
pixelox text add cards.pxo --in-place --name C2 --content $'Of new riders\nsay the lane\nis why they\nstarted' --font @thin-sans --weight 400 --size 30 --leading 43 --align center --color '#ffffff' --at 408,758 --anchor c
pixelox text add cards.pxo --in-place --name C3 --content $'More trips\nby bike in\nthe first\nthree years' --font @thin-sans --weight 400 --size 30 --leading 43 --align center --color '#ffffff' --at 666,758 --anchor c
pixelox text add cards.pxo --in-place --name C4 --content $'Avg rise in\nsales for\nshops on\nthe street' --font @thin-sans --weight 400 --size 30 --leading 43 --align center --color '#ffffff' --at 924,758 --anchor c
pixelox text add cards.pxo --in-place --name Label --content "Who Rides to Work" --font @thin-sans --weight 400 --size 36 --color '#f47eab' --at 45,955 --anchor l
pixelox shape ellipse cards.pxo --in-place --name CircleL --rect 81,997,218x218 --fill '#7a8696'
pixelox render js scripts/waffle.js -o people.png --size 1080x1290 --param box=11,80,13.2,11.4 --param value=3 --param total=3 --param shape=person --param colors='#ffffff' --param legend=none
pixelox comp place cards.pxo people.png --in-place --as People --at 0,0
pixelox render js scripts/donut.js -o pie.png --size 1080x1290 --param cx=50.5 --param cy=85.7 --param r=10.4 --param inner=0 --param values=79,21 --param start=90 --param colors='#20558b,#42b5ee' --param labelpos=none --param sep=none
pixelox comp place cards.pxo pie.png --in-place --as Pie --at 0,0
pixelox shape path cards.pxo --in-place --name Lead1 --d 'M 600 1063 L 640 1037 L 668 1037' --fill none --stroke '#5d6670' --stroke-width 2
pixelox shape rect cards.pxo --in-place --name Tag1 --rect 668,1019,110x36 --fill '#a0a6ae'
pixelox text add cards.pxo --in-place --name Tag1T --content "Drive" --font @geometric-sans --weight 700 --size 19 --color '#ffffff' --at 723,1037 --anchor c
pixelox shape path cards.pxo --in-place --name Lead2 --d 'M 490 1148 L 452 1190 L 434 1190' --fill none --stroke '#5d6670' --stroke-width 2
pixelox shape rect cards.pxo --in-place --name Tag2 --rect 324,1172,110x36 --fill '#a0a6ae'
pixelox text add cards.pxo --in-place --name Tag2T --content "Ride" --font @geometric-sans --weight 700 --size 19 --color '#ffffff' --at 379,1190 --anchor c
pixelox shape ellipse cards.pxo --in-place --name CircleR --rect 791,997,218x218 --fill '#a3a9b1'
pixelox text add cards.pxo --in-place --name RN --content "64%" --font @thin-sans --weight 200 --size 75 --color '#ffffff' --at 900,1061 --anchor c --anchor-to ink
pixelox text add cards.pxo --in-place --name RC --content $'Of Trips\nUnder' --font @thin-sans --weight 400 --size 32 --leading 43 --align center --color '#ffffff' --at 900,1148 --anchor c
pixelox text add cards.pxo --in-place --name W1 --content "Most People" --font @thin-sans --weight 200 --size 47 --color '#4bb0cf' --at 190,1250 --anchor c
pixelox text add cards.pxo --in-place --name W2 --content "Choose to Ride" --font @thin-sans --weight 200 --size 47 --color '#4bb0cf' --at 545,1250 --anchor c
pixelox text add cards.pxo --in-place --name W3 --content "Five Km" --font @thin-sans --weight 200 --size 47 --color '#4bb0cf' --at 900,1250 --anchor c
pixelox filter grain cards.pxo --in-place --as-layer --name Paper --amount 10
pixelox export cards.pxo -o cards.png
```

Reuse: change the four numbers, labels, icons, and captions first; keep every caption to four
lines of at most 13 characters or drop the size to 27. The pie's tags hang on the `slice1`
and `slice2` anchors (490,1148 and 600,1063 here): read them again with `--json` when you
change `values=` or `start=`. With three cards use columns of 28% W at 5%, 36%, 67%; with
five, 17% W each and 26 px captions. Add a source line at 99% H when the numbers are real.

## 13. Variations and pairings

* **Square:** drop the circle row and the section label; cards at 30% to 90% H of a
  1080 x 1080 canvas.
* **One hue:** four tints of one color (for example `#0b3d2e`, `#2e8b57`, `#57a773`,
  `#7fc8a9`) when the four numbers describe one subject; four hues when they are four topics.
* **Dark ground:** `#20232b` with the same four card colors; shadows become glows, so leave
  them out.
* **Charts in the cards:** swap each icon disc for a C-progress-ring (`r=6.6`, white on the
  darker tone as the track) when all four numbers are shares of 100.
* The circle row can be replaced by a C-stat-row or a C-process-row; the card row also works
  as the "key numbers" module of an S-long-bands explainer.

## 14. Pitfalls

* **Numbers that look comparable but are not.** Four big percentages in a row invite
  comparison although their bases differ (and one is above 100%). Each caption must name its
  own base ("of new riders", "more trips"); never add bars or a shared scale.
* **A pie without its numbers.** The original labels the slices with names only. If the
  split matters, print the shares on the tags ("Ride 79%"); if it does not, drop the pie.
* **A closing number that contradicts the pie.** The disc beside the pie reads as the pie's
  value. Either make it the same split or caption it clearly as a different measure.
* **Thin type too small.** A 200-weight face under 30 px breaks up on a colored card. Use
  the regular weight for captions, or raise the size.
* **Shadows on the cards.** A drop shadow attached to each card makes them look like buttons.
  The detached blurred ellipse a little below is what makes them float.
* **More than five cards.** Captions then need five or six lines. Split into two rows or
  use C-icon-grid.
