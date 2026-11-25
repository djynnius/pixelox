---
id: i029
title: Plate cut into four parts with corner callouts
kind: [statistical, explainer]
topic: [food, health]
mood: [friendly, bold, vibrant, clean]
style: [flat, illustrated]
format: landscape
structure: S-hub-spokes
data: [part-to-whole]
charts: [C-pie, C-callout, C-stat-block, C-title-block, C-source-line]
palette: ['#ffffff', '#f89d2a', '#f36523', '#bf333e', '#605371', '#52afa4', '#000000']
needs: nothing
---

# i029: Plate cut into four parts with corner callouts

## 1. Concept

A white sheet with one round "plate" low in the middle: a thick orange rim around a disc cut
into four colored parts, each part holding three or four small flat pictures of what belongs
in it. A callout sits in each corner of the page (a larger picture, a two-word heading, a
short paragraph, a colored rule that runs into the plate, and one big percentage). The
message is "one whole, four groups, this much of each". It works because the object is the
chart: a plate (or a bed, a wallet, a clock face) already means "a whole to be divided", so
no axis or legend is needed, and the rules tie every number to its part.

## 2. Data story

* **One whole in four parts**: four categories, each with one percentage (two digits). The
  relationship is part-to-whole.
* In the original the four parts are drawn as **equal quarters** while the printed
  percentages differ widely (the largest is several times the smallest) and **add up to
  well over 100**, so the picture and the numbers disagree (see Pitfalls). The parts
  are only a picture of "four groups"; the percentages are loose labels.
* Text: 4 headings of 2 words; 4 paragraphs of about 18 words on 5 lines (placeholder Latin
  in the original); a title of 2 words.

Placeholder data in the recipe (example data, invented: how the area of a kitchen garden is
divided). The four shares add to 100 and the slices follow them:

| Part (clockwise from the upper right) | Share | Color |
| --- | --- | --- |
| Fruit crops | 34% | `#bf333e` |
| Leafy greens | 26% | `#52afa4` |
| Root crops | 22% | `#605371` |
| Herb border | 18% | `#f36523` |

## 3. Information architecture

1. **Title**, centered on top: one heavy word, one light word under it, set to the same width.
2. **The plate**, center, lower two thirds: the whole and its four parts. Seen first after
   the title because it is the only large colored shape.
3. **Four callouts**, one per corner, each beside its part: picture, heading, paragraph,
   rule, percentage. The left callouts are flush left with the number at the rule's outer
   (left) end; the right callouts put the number at the rule's right end. The rule's inner
   end enters the plate, so it works as a leader.
4. Reading order after the plate: upper left, upper right, lower left, lower right (a Z).

## 4. Format and grid

* Landscape, 1.13 : 1. Recipe canvas 1600 x 1410. Outer margins about 5% W.
* Title: centered at 50% W; cap tops at 9.9% H (line 1) and 17.4% H (line 2); both lines
  about 41% W wide.
* Plate: center 49.7% W, 64.7% H; rim outer radius 23.4% W; the colored disc inside it has a
  radius of 21.3% W (rim 2.1% W thick). Part boundaries cross at the center.
* Left callout column from 5% to 30% W; right callout column from 74% to 95% W.
* Upper callouts: picture box about `8%,19%,13% x 14%` (left) and `76%,20%,13% x 12%`
  (right); heading cap top at 34% to 35.5% H; paragraph to 47% H; rule at 48% H; percentage
  cap top at 49.5% H.
* Lower callouts: picture box about `8%,56%,12% x 16%` and `77%,55%,10% x 16%`; heading cap
  top at 72% to 73% H; paragraph to 85% H; rule at 85 to 86% H; percentage at 87 to 88% H.
* Rules: upper left from 5.6% to 34.6% W, upper right from 63% to 93% W, lower left from
  4.8% W into the plate, lower right from 58% to 95% W. Each ends 1 to 3% W inside the disc.

## 5. Layer stack

1. Ground: white, flat.
2. Title: line 1 black, heavy; line 2 black, light.
3. Rim: an orange disc `#f89d2a`, diameter 46.9% W.
4. The four parts, on the same center: orange-red `#f36523` (upper left), crimson `#bf333e`
   (upper right), dusty purple `#605371` (lower left), teal `#52afa4` (lower right), with
   hairlines in the rim's orange between them.
5. Pictures inside the parts: three or four flat color illustrations in each, about 6% W
   wide, loosely scattered and never touching the hairlines (fruit in one, produce in the
   next, dairy and meat in the third, vegetables in the fourth).
6. Four corner pictures, about 12% W wide: flat illustrations with one highlight shape and
   no outlines (a green apple with a leaf, a halved avocado, two cartons, a red pepper).
7. Four headings (black, heavy caps) and four paragraphs (near black, semibold).
8. Four rules, 5 px, drawn over the plate's edge: orange-red, brick `#b1483e`, green
   `#92d050`, dark maroon `#5c0a0a` in the original; in the recipe each takes its part's color.
9. Four percentages, heavy: orange-red `#f46727`, brick `#b24a41`, green `#95d154`, brick
   `#b1483e` in the original; in the recipe each takes its part's color.
10. Source line (added in the recipe; the original has none).

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#ffffff` | ground | 72% | nothing |
| `#f89d2a` | rim, hairlines between parts | 4% | nothing: the object's edge |
| `#f36523` | part 4 (upper left), its rule and number | 4% | category 4 |
| `#bf333e` | part 1 (upper right), its rule and number | 5% | category 1 |
| `#605371` | part 3 (lower left), its rule and number | 4% | category 3 |
| `#52afa4` | part 2 (lower right); rule and number in the darker `#3c9a8f` | 4% | category 2 |
| `#000000` | title, headings; paragraphs in `#14101e` | 3% | nothing |

Categorical: four hues of similar weight, one per part, held together by the warm orange
rim. Black is the only type color apart from the four numbers. In the original two of the
numbers do not use their part's color; treat that as a flaw, not a feature.

## 7. Text elements

Sizes are px on the 1600 px canvas (multiply by 0.675 for a 1080 px one).

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Title line 1 | 1 word, 9 letters | caps | geometric sans (Poppins Black, Montserrat Black / Avenir Next Heavy) | 800 to 900 | 111 | 0 | - | center | `#000000` | 50%, cap top 9.9% |
| Title line 2 | 1 or 2 words, 11 letters | caps | geometric sans (Poppins Light / Avenir Next Regular) | 300 to 400 | 89 | 10 | - | center | `#000000` | 50%, cap top 17.4% |
| Callout heading (x4) | 2 words, 10 to 12 letters | caps | geometric sans | 800 | 33 | 0 | - | left | `#000000` | 8.5%, 34% / 73.8%, 35.5% / 7.3%, 73% / 73.8%, 72% |
| Callout paragraph (x4) | 16 to 20 words on 4 or 5 lines of about 26 characters | sentence | geometric sans | 600 to 700 | 24 (the original: 19, too small) | 0 | 33 px | left | `#14101e` | 3% H under its heading |
| Percentage (x4) | NN% | - | geometric sans | 800 | 67 | 0 | - | left (left callouts), right (right callouts) | its part's color | 1.5% H under its rule, at the rule's outer end |
| Source line | what the shares are of; "Example data" | sentence | geometric sans | 500 | 22 | 0 | - | center | `#6b6b6b` | 50%, 97.3% |

One family in three weights: heavy for the title word, headings, and numbers; light for the
second title line; semibold for paragraphs. Sizes 111 : 89 : 67 : 33 : 24, about 4.6 : 3.7 :
2.8 : 1.4 : 1. The heavy and light title pair at equal width is the piece's only flourish.

## 8. Charts and diagrams

* **C-pie**: `scripts/donut.js` placed by center, `cx=49.7 cy=64.75 r=21.3`, `inner=0`,
  `values=34,26,22,18`, `start=-14`, `labelpos=none`, `sep='#f89d2a'` (hairlines in the rim's
  color). The rim is a plain `shape ellipse` 750 px across behind it. No numbers on the
  slices: each number sits in its callout. `start=-14` turns the pie so that each slice faces
  its own corner; the slice anchors (`slice1` to `slice4`) give the middle of each part.
* **What changed from the original:** the original is not a chart. It is a disc cut by a
  cross into four equal quarters, with a percentage beside each that does not match a
  quarter and does not add to 100 with the others. The recipe draws a real pie with the same
  rim, colors, hairlines, and callouts. To copy the equal-quarter look honestly, use
  `values=25,25,25,25 start=0` and leave the percentages out (four groups, no amounts).
* **C-callout** x4: the rule under each paragraph is the leader. It is horizontal, 5 px, in
  the part's color, and its inner end stops 20 to 60 px inside its own slice.
* **C-stat-block** x4 (picture, heading, paragraph, number) and a centered **C-title-block**.

## 9. Icons and illustration

* The original uses flat color illustrations without outlines: 14 small ones inside the
  plate (berries, a banana, an apple, an avocado, broccoli, a peach, a pear, cartons, a roast
  bird, a steak, a pepper, tomatoes, a cauliflower) and 4 large ones in the corners. They
  are the user's to supply: place each as `illustration.png` at 180 to 220 px wide for the
  corners and 90 to 110 px inside the plate.
* The recipe uses Lucide line icons instead. Inside the plate: three white icons per slice,
  96 px, 7 px line, placed around the slice anchor: `cherry`, `apple`, `citrus`;
  `leafy-green`, `salad`, `sprout`; `carrot`, `bean`, `nut`; `flower-2`, `leaf`, `flower`.
* Corner pictures: one icon of 190 px, 10 px line, in the part's color with a pale tint of
  that color as `--fill` (two-tone): `flower-2`, `cherry`, `carrot`, `leafy-green`.
* One style only: all line icons, or all flat illustrations. A mix reads as clip art.

## 10. Effects and finish

None: flat color, no shadows, no gradients, no texture. The original's illustrations carry
one lighter highlight shape each; that is the only modelling on the page.

## 11. Hierarchy and eye path

1. The heavy title word (the darkest, largest type).
2. The plate: four saturated parts inside the orange rim.
3. The four percentages (67 px, colored), each found by following a rule out of the plate.
4. The corner pictures and headings.
5. The paragraphs.

Title 111 px, numbers 67, headings 33, paragraphs 24: about 4.6 : 2.8 : 1.4 : 1.

## 12. Build recipe

Nothing to supply. Canvas 1600 x 1410. Example data, invented; the four shares add to 100.

```bash
pixelox new -o plate.pxo --size 1600x1410 --fill '#ffffff'
pixelox text add plate.pxo --in-place --name Title1 --content "VEGETABLE" --font @geometric-sans --weight 800 --size 111 --color '#000000' --at 50%,140 --anchor t --anchor-to ink
pixelox text add plate.pxo --in-place --name Title2 --content "GARDEN PLAN" --font @geometric-sans --weight 400 --size 89 --tracking 10 --color '#000000' --at 50%,246 --anchor t --anchor-to ink
pixelox shape ellipse plate.pxo --in-place --name Rim --rect 420,538,750x750 --fill '#f89d2a'
pixelox render js scripts/donut.js -o pie.png --size 1600x1410 --param cx=49.7 --param cy=64.75 --param r=21.3 --param inner=0 --param values=34,26,22,18 --param start=-14 --param colors='#bf333e,#52afa4,#605371,#f36523' --param labelpos=none --param sep='#f89d2a'
pixelox comp place plate.pxo pie.png --in-place --as Pie --at 0,0
pixelox shape icon plate.pxo cherry --in-place --name S1a --size 96 --at 863,725 --anchor c --color '#ffffff' --stroke-width 7
pixelox shape icon plate.pxo apple --in-place --name S1b --size 96 --at 992,775 --anchor c --color '#ffffff' --stroke-width 7
pixelox shape icon plate.pxo citrus --in-place --name S1c --size 96 --at 964,898 --anchor c --color '#ffffff' --stroke-width 7
pixelox shape icon plate.pxo leafy-green --in-place --name S2a --size 96 --at 971,1061 --anchor c --color '#ffffff' --stroke-width 7
pixelox shape icon plate.pxo salad --in-place --name S2b --size 96 --at 857,1145 --anchor c --color '#ffffff' --stroke-width 7
pixelox shape icon plate.pxo sprout --in-place --name S2c --size 96 --at 850,1008 --anchor c --color '#ffffff' --stroke-width 7
pixelox shape icon plate.pxo carrot --in-place --name S3a --size 96 --at 629,1079 --anchor c --color '#ffffff' --stroke-width 7
pixelox shape icon plate.pxo bean --in-place --name S3b --size 96 --at 570,961 --anchor c --color '#ffffff' --stroke-width 7
pixelox shape icon plate.pxo nut --in-place --name S3c --size 96 --at 700,968 --anchor c --color '#ffffff' --stroke-width 7
pixelox shape icon plate.pxo flower-2 --in-place --name S4a --size 96 --at 587,793 --anchor c --color '#ffffff' --stroke-width 7
pixelox shape icon plate.pxo leaf --in-place --name S4b --size 96 --at 678,710 --anchor c --color '#ffffff' --stroke-width 7
pixelox shape icon plate.pxo flower --in-place --name S4c --size 96 --at 717,835 --anchor c --color '#ffffff' --stroke-width 7
pixelox shape icon plate.pxo flower-2 --in-place --name PicTL --size 190 --at 232,365 --anchor c --color '#f36523' --fill '#fbd0b8' --stroke-width 10
pixelox text add plate.pxo --in-place --name HeadTL --content "HERB BORDER" --font @geometric-sans --weight 800 --size 33 --color '#000000' --at 136,480 --anchor tl --anchor-to ink
pixelox text add plate.pxo --in-place --name TextTL --content $'Basil, chives, and marigolds\nalong the sunny edge bring\nin bees and keep pests\noff the beds beside them.' --font @geometric-sans --weight 700 --size 24 --leading 33 --color '#14101e' --at 136,524 --anchor tl
pixelox shape line plate.pxo --in-place --name RuleTL --from 90,678 --to 554,678 --stroke '#f36523' --stroke-width 5
pixelox text add plate.pxo --in-place --name PctTL --content "18%" --font @geometric-sans --weight 800 --size 67 --color '#f36523' --at 90,698 --anchor tl --anchor-to ink
pixelox shape icon plate.pxo cherry --in-place --name PicTR --size 190 --at 1324,372 --anchor c --color '#bf333e' --fill '#f0c4c8' --stroke-width 10
pixelox text add plate.pxo --in-place --name HeadTR --content "FRUIT CROPS" --font @geometric-sans --weight 800 --size 33 --color '#000000' --at 1180,500 --anchor tl --anchor-to ink
pixelox text add plate.pxo --in-place --name TextTR --content $'Tomatoes, peppers, and\nsquash take the largest\nshare: they need the most\nroom, sun, and water.' --font @geometric-sans --weight 700 --size 24 --leading 33 --color '#14101e' --at 1180,544 --anchor tl
pixelox shape line plate.pxo --in-place --name RuleTR --from 1010,678 --to 1490,678 --stroke '#bf333e' --stroke-width 5
pixelox text add plate.pxo --in-place --name PctTR --content "34%" --font @geometric-sans --weight 800 --size 67 --color '#bf333e' --at 1490,698 --anchor tr --anchor-to ink
pixelox shape icon plate.pxo carrot --in-place --name PicBL --size 190 --at 217,904 --anchor c --color '#605371' --fill '#d6d0de' --stroke-width 10
pixelox text add plate.pxo --in-place --name HeadBL --content "ROOT CROPS" --font @geometric-sans --weight 800 --size 33 --color '#000000' --at 116,1030 --anchor tl --anchor-to ink
pixelox text add plate.pxo --in-place --name TextBL --content $'Carrots, beets, and bush\nbeans fill the cool side\nand follow the leaf crops\nin next year’s rotation.' --font @geometric-sans --weight 700 --size 24 --leading 33 --color '#14101e' --at 116,1074 --anchor tl
pixelox shape line plate.pxo --in-place --name RuleBL --from 76,1214 --to 620,1214 --stroke '#605371' --stroke-width 5
pixelox text add plate.pxo --in-place --name PctBL --content "22%" --font @geometric-sans --weight 800 --size 67 --color '#605371' --at 96,1240 --anchor tl --anchor-to ink
pixelox shape icon plate.pxo leafy-green --in-place --name PicBR --size 190 --at 1312,890 --anchor c --color '#3c9a8f' --fill '#c9e8e4' --stroke-width 10
pixelox text add plate.pxo --in-place --name HeadBR --content "LEAFY GREENS" --font @geometric-sans --weight 800 --size 33 --color '#000000' --at 1180,1016 --anchor tl --anchor-to ink
pixelox text add plate.pxo --in-place --name TextBR --content $'Lettuce, chard, and kale\nare sown every three\nweeks so one row is\nalways ready to cut.' --font @geometric-sans --weight 700 --size 24 --leading 33 --color '#14101e' --at 1180,1060 --anchor tl
pixelox shape line plate.pxo --in-place --name RuleBR --from 930,1196 --to 1524,1196 --stroke '#3c9a8f' --stroke-width 5
pixelox text add plate.pxo --in-place --name PctBR --content "26%" --font @geometric-sans --weight 800 --size 67 --color '#3c9a8f' --at 1516,1224 --anchor tr --anchor-to ink
pixelox text add plate.pxo --in-place --name Source --content "Share of bed area. Example data." --font @geometric-sans --weight 500 --size 22 --color '#6b6b6b' --at 50%,1372 --anchor c
pixelox export plate.pxo -o plate.png
```

Reuse: change `values=` (clockwise from the upper right; they must add to 100), then the
four numbers. Run the `render js` line with `--json` and read `slice1` to `slice4`: with
these values they are 950,769; 884,1105; 609,1013; 642,767. The icons in a slice sit within
about 120 px of its anchor, and each rule must end inside its own slice: if a slice gets
narrow, move the rule's inner end toward that anchor or stop the rule at the rim. Turn the
pie with `start=` until every slice faces its callout. Your own pictures replace the corner
icons: `comp place plate.pxo illustration.png --in-place --as PicTL --size 200x --at 232,365
--anchor c` (your image here).

## 13. Variations and pairings

* **Five or six parts:** three callouts per side in a 1080 x 1350 portrait canvas, plate at
  50% W, 50% H; headings only, no paragraphs.
* **A hole in the middle:** `inner=0.45` and a total or a word in the hole (C-donut) when the
  whole has a number of its own ("120 m2").
* **Other objects that mean "a whole":** a clock face for hours of a day, a coin for a
  budget, a round table for seats. Keep the rim and change its color.
* **Four equal groups, no numbers:** `values=25,25,25,25 start=0`, drop the percentages, and
  the piece becomes a "four kinds of X" explainer.
* Pairs with C-percent-bar under the plate for the same shares as a strip, or with a
  C-stat-row of three totals in a taller format.

## 14. Pitfalls

* **Equal parts, unequal numbers.** The original draws four equal quarters and prints four
  very different percentages. The eye believes the areas. Let the script size the slices
  from the values, or print no numbers.
* **Shares that do not add to 100.** Four percentages around one plate are read as parts of
  one whole. If they are separate measures (each "of the daily need"), a divided disc is the
  wrong picture: use four rings (C-progress-ring) instead.
* **A number in the wrong color.** In the original a green number belongs to a purple part
  and a red number to a teal one, so color no longer links callout and part. Give the rule
  and the number the slice's exact color.
* **Rules that end in a neighbor's slice.** After changing the values, check each rule's
  inner end against the slice boundaries.
* **Paragraphs at 13 px.** The original's body text is unreadable on a phone. Four lines at
  24 px (on 1600) are the minimum; cut words, not size.
* **Pictures crossing the hairlines.** An item that straddles two parts belongs to neither.
  Keep every picture at least 20 px inside its slice.
