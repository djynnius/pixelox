---
id: i028
title: Dark editorial stat poster around a portrait
kind: [statistical, explainer]
topic: [business, generic]
mood: [dark, editorial, serious, minimal]
style: [editorial, photo-led]
format: portrait
structure: S-stat-poster
data: [single-stat, change-over-time, text-only]
charts: [C-stat-block, C-multi-line, C-callout, C-title-block, C-source-line]
palette: ['#0d0d0d', '#161616', '#ffffff', '#e66a45', '#bfbfbf', '#8a3f2a']
needs: photo
---

# i028: Dark editorial stat poster around a portrait

## 1. Concept

A magazine page in black: one black-and-white portrait sits in the middle of a near-black
sheet and a dozen small facts float around it without boxes, rules, or cards. A light serif
title asks a question in orange; white numbers with orange percent signs answer it; three
huge orange numerals along the bottom split a short story into three parts; one tiny line
chart closes the row. The reader takes away "this trade is changing, here are six numbers
that say so". It works because there are only three inks (black, white, one orange), the
photo is drained of color so the orange owns the page, and every module is the same small
stack: a number, a caps word, a note.

## 2. Data story

* **One headline percentage** (two digits, a rise) with a one-word caps label and a note
  that says of what and over which period.
* **One ratio** written as "N in N" (single digits), with a note.
* **Three percentages** stepping down the right side, each with a two-line name and a
  period line (a quarter and a masked year). In the original all three show the same
  two-digit value: template placeholders, not a finding.
* **Three text chapters** numbered 1 to 3, each a caps word (three time frames in the
  original) and a note of 10 to 14 words. No numbers except one inside a sentence.
* **One small trend**: 3 series over 4 periods, values within one order of magnitude; the
  top series ends with a jump, the other two dip and recover. One value is called out on an
  orange tag.
* Relationships: single stats and one change over time. Nothing is a part of a whole.

Placeholder data in the recipe (example data, invented: shared pottery studios in a region).
Headline: members up 47%. Ratio: 2 in 5 new members are beginners. Three shares of studio
members: 72% sell at markets, 58% teach, 41% share a kiln. Trend, members of the three
largest studios over four yearly counts:

| Series | Year 1 | Year 2 | Year 3 | Year 4 |
| --- | --- | --- | --- | --- |
| Studio A | 820 | 860 | 790 | 1,240 |
| Studio B | 640 | 660 | 480 | 760 |
| Studio C | 410 | 190 | 290 | 380 |

## 3. Information architecture

1. **Title**, top left: a question on three lines. The largest words on the page.
2. **Side note**, top right: a two-line caps heading and a five-line paragraph: the context.
3. **Portrait**, center: the human face of the topic. It carries no data; it stops the eye
   and gives the scattered modules a middle to stand around.
4. **Headline number**, left of the face: number, orange percent sign, caps label, note.
5. **Ratio**, under it: "N in N" and a note.
6. **Three stats**, right of the face, in a zigzag (the middle one is shifted right by 6% W),
   each centered on its own axis: number, name, period.
7. **Three numbered chapters** along the bottom, left to right: a big orange numeral, a caps
   word laid over its foot, a note.
8. **Trend**, fourth column of that row: caps heading, a value tag on a leader, three lines.
9. **Footer**: a hairline, a boxed wordmark, four columns of fine print (method, notes,
   contact).

The eye goes title, face, then left column down, right column down, bottom row across.
Numbering only appears in the bottom row, where order matters.

## 4. Format and grid

* Portrait 3 : 4. Recipe canvas 1080 x 1440. Margins 6.75% W left, 6% W right.
* Four columns for the bottom row and the footer: left edges at 6.75%, 29.7%, 52.8%, and
  75.3% W; each text column is 17% W wide, gutters 6% W.
* Title: left 6.75% W; cap tops at 6.4%, 12%, and 17.6% H; the longest line reaches 55% W.
* Side note: left 73.8% W; heading cap top 13.1% H; paragraph 19.4% to 27.6% H.
* Photo panel: `21%,22.5%,52% x 66%` (it ends just above the footer rule). The face is
  centered near 48% W, 40% H and is about 22% W wide.
* Headline number: left 6.75% W, cap top 29.2% H; caps label cap top 35.4% H; note 38.6% to
  44% H. Ratio: cap top 49.3% H; note 53% to 59.5% H.
* Three stats, each centered on an axis: 81.3% W (number center 35.9% H), 87.5% W (47.8% H),
  81.3% W (59.4% H). Under each number: the name 3% H lower, the period line 7.5% H lower.
* Bottom row: numerals from 68.5% to 77.2% H; caps words with cap top at 73.9% H (so they
  cross the numerals' feet); notes from 79% to 86.5% H.
* Trend column: heading cap top 73.9% H; tag `76.7%,77.9%,8.3% x 1.8%`; chart
  `75.3%,79.2%,18.6% x 7.6%`.
* Footer: rule at 90.35% H from 6% to 94.25% W; mark box `6%,92.6%,16% x 4%`; fine print
  columns at 27.2%, 44.3%, 61.3%, and 80.7% W, top at 93% H.
* Everything is flush left on the four column edges except the three stats, which are
  centered on their axes.

## 5. Layer stack

1. Ground: near black `#0d0d0d`, flat.
2. Photo panel: a rectangle `#161616`, one step lighter than the ground, hard edged. In the
   original this is the photo's own studio backdrop.
3. Photo: a black-and-white head-and-shoulders portrait inside the panel. The face is framed
   by two raised hands, a wide dark hat brim crosses the top of the panel, the clothes are
   black. Soft light from the front; the skin is the only light area (`#bfbfbf` to
   `#dbdada`); hat, hair, and clothes merge with the panel, so the lower third of the panel
   is almost black and can carry text.
4. Title, orange `#e66a45`.
5. Side heading and paragraph, white.
6. Headline number (white), its percent sign (orange), caps label and note (white).
7. Ratio: two white digits with a small orange caps word between them; note in white.
8. Three stats: white number, orange percent sign, white name, white period line.
9. Three orange numerals; over each one's foot a white caps word; under it a white note.
10. Trend: white caps heading; three lines (light gray `#cfc8c5`, orange `#c0603f`, rust
    `#8a3f2a`) with small white dots on the points; an orange tag `#ed6640` with white bold
    figures; a thin orange elbow leader from the tag to the top line.
11. Footer: hairline `#a5a5a5`; a pure black box `#000000` with a white wordmark and a tiny
    italic line; four columns of white fine print, the first one bold.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#0d0d0d` | ground | 70% | nothing |
| `#161616` | photo panel (the photo's dark tones run from `#0a0a0a` to `#252525`) | 18% | nothing |
| `#bfbfbf` | the lit skin in the photo | 5% | nothing: the focal point |
| `#ffffff` | numbers, caps labels, body text, fine print, chart dots | 4% | values and text |
| `#e66a45` | title, percent signs, the three numerals, the word inside the ratio, tag `#ed6640`, leader | 3% | emphasis: units, order, the called-out value |
| `#cfc8c5` | top chart line | under 1% | series A |
| `#8a3f2a` | bottom chart line (middle line `#c0603f`) | under 1% | series C (and B) |

One accent on black and white. Orange never colors a number, only what stands beside a
number (the percent sign, the "in", the numeral of a chapter); the figures themselves stay
white. The chart is the only place with tints of the accent: lighter means the larger series.

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Title | a question of 3 or 4 words on 3 lines of 6 to 10 letters | caps | didone (Playfair Display, Bodoni Moda, Besley Light / Didot) | 300 to 400 | 94 | 0 | 81 px | left | `#e66a45` | 6.75%, cap top 6.4% |
| Side heading | 2 or 3 short words on 2 lines | caps | oldstyle (Playfair Display Bold, Besley SemiBold / Baskerville Bold) | 700 | 35 | 0 | 36 px | left | `#ffffff` | 73.8%, cap top 13.1% |
| Side paragraph | 18 to 20 words on 5 lines of at most 22 characters | sentence | neo-grotesque (Archivo SemiExpanded SemiBold, Inter SemiBold / Helvetica Neue Bold) | 600 to 700 | 18 | 0 | 25 px | left | `#ffffff` | 73.8%, top 19.4% |
| Headline number | NN | - | neo-grotesque (Inter Light, Archivo Light / Helvetica Neue Light) | 300 | 71 | 0 | - | left | `#ffffff` | 6.75%, cap top 29.2% |
| Percent sign | % | - | didone italic | 400 | 71 (50 beside the three stats) | 0 | - | left | `#e66a45` | 2% W right of its number, same cap top |
| Caps label | 1 word, 6 to 8 letters | caps | oldstyle | 700 | 28 | 0 | - | left | `#ffffff` | 6.75%, cap top 35.4% |
| Notes (headline, ratio) | 8 to 12 words on 3 or 4 lines of at most 20 characters | sentence | neo-grotesque | 700 | 18 | 0 | 25 px | left | `#ffffff` | 6.75%, top 38.6% and 53% |
| Ratio digits | N and N | - | neo-grotesque | 300 | 41 | 0 | - | left | `#ffffff` | 7% and 15.2%, cap top 49.3% |
| Ratio word | "IN" | caps | oldstyle | 700 | 26 | 0 | - | left | `#e66a45` | 10.6%, centered on the digits |
| Stat number (x3) | NN | - | neo-grotesque | 300 | 50 | 0 | - | right, against its percent sign | `#ffffff` | axes 81.3%, 87.5%, 81.3%; centers 35.9%, 47.8%, 59.4% |
| Stat name (x3) | 3 words on 2 lines | sentence | neo-grotesque | 700 | 18 | 0 | 25 px | center | `#ffffff` | on the axis, top 38.8%, 50.4%, 62% |
| Period line (x3) | a quarter and a masked year (recipe: a season and "survey") | caps | neo-grotesque | 700 | 14 | 0 | - | center | `#ffffff` | on the axis, 43.2%, 54.7%, 66.3% |
| Chapter numeral (x3) | 1, 2, 3 | - | didone | 400 | 176 | 0 | - | left | `#e66a45` | 6.75%, 29.7%, 52.8%; top 68.5% |
| Chapter word (x3) and chart heading | 1 word, 4 to 8 letters | caps | oldstyle | 700 | 34 | 0 | - | left | `#ffffff` | column edges, cap top 73.9% |
| Chapter note (x3) | 10 to 14 words on 4 or 5 lines of at most 20 characters | sentence | neo-grotesque | 700 | 18 | 0 | 25 px | left | `#ffffff` | column edges, top 78.8% |
| Tag figures | a short amount, 4 to 6 characters | - | neo-grotesque | 700 | 16 | 0 | - | center | `#ffffff` | in the tag |
| Chart end labels | 2 short period names | sentence | neo-grotesque | 400 | 14 | 0 | - | left and right | `#8f8f8f` | under the chart's ends, 87.6% |
| Wordmark | 1 or 2 words | caps | neo-grotesque | 400 | 25 | 0 | - | center | `#ffffff` | in the mark box |
| Mark line | 3 words | title | neo-grotesque italic | 400 | 12 | 0 | - | center | `#c9c9c9` | under the wordmark |
| Fine print (x3) | 12 to 20 words on 3 or 4 lines; the first column bold | sentence | neo-grotesque | 700, 400, 400 | 12 (the original: 9, unreadable) | 0 | 15 px | left | `#ffffff` | 27.2%, 44.3%, 61.3%; top 93% |
| Contact lines | 3 lines: web, mail, phone | lower | neo-grotesque | 400 | 13 | 0 | 19 px | left | `#ffffff` | 80.7%, top 92.8% |

Two voices: a serif for everything that is a word of display (the light title and numerals,
and its bold for the caps labels) and one grotesque for numbers (light) and reading text
(bold, because small white type on black thins out). Sizes 176 : 94 : 71 : 50 : 34 : 18,
about 10 : 5 : 4 : 2.8 : 1.9 : 1. Didot is about a third wider than the original's face:
keep title lines to 8 letters at 94 px, or use `--fit-width 54%` on the longest line. Didot
has no bold, so the caps labels take the bold of `@oldstyle`; with Playfair Display
installed use it for both.

## 8. Charts and diagrams

* **C-multi-line**: `scripts/line.js`, box `75.3,79.2,18.6,7.6` (201 x 109 px on the recipe
  canvas: a miniature). `values='820,860,790,1240;640,660,480,760;410,190,290,380'`,
  `smooth=1 markers=1 width=5`, `vlabels=none endlabels=0 legend=none axis=0 gridlines=0`,
  `colors='#cfc8c5,#c0603f,#8a3f2a'`, `muted='#3a3a3a'` (the baseline, the only axis mark).
  The value axis starts at zero. Markers are white dots ringed in the line color, as in the
  original. No series names: at this size the heading and the tag do that work.
* **C-callout**: an orange tag (90 x 26 px) with white bold figures sits above the left
  half of the chart; a 2 px orange elbow leader runs from its right edge to the point it
  names. In the recipe that is the last point of the top line (anchor `p1.4`).
* **What changed from the original:** its three lines have no baseline, and two tiny
  percentages float beside the ends of the lowest line without saying what they measure;
  its tag points at the top line between two points. The recipe adds a faint baseline, two
  end labels that name the first and last period, and ties the tag to one real point.
* **C-stat-block** x5 without icons: the headline number, the ratio, and the three stats are
  all "number, name, note" stacks. **C-title-block** without accent bar or rule.
  **C-source-line** as the four-column footer.

## 9. Icons and illustration

* No icons. The percent signs, the word inside the ratio, and the three numerals do the
  job icons would do: they are the orange marks that make the page scannable.
* **Photo** (`photo.png`, your image here): a portrait with a calm, direct look and a dark
  surround works best: a face framed by hands, a hat, a collar, or hair, on a dark studio
  backdrop. It is converted to black and white, darkened, cropped to the panel
  (52% W x 66% H), and faded radially from the face outward so that hands and shoulders sink
  into the panel. Keep the face between 30% and 50% H and inside 35% to 62% W: the left
  column, the right column, and the bottom row all need the dark around it.
* **Wordmark**: plain text "YOUR MARK" in a black box, or `logo.png` placed 150 px wide at
  the box center.

## 10. Effects and finish

* Black and white: `adjust bw --layer Photo`.
* Low key: `adjust levels --layer Photo --in-black 12 --gamma 0.85 --out-white 215` deepens
  the shadows and caps the highlights at light gray, so the white type stays the brightest
  thing on the page.
* Crop to the panel: `mask add --layer Photo --from-path` with the panel's rectangle.
* Sink into the ground: a second `mask add --gradient radial --ease smooth --combine
  intersect`, centered on the face, radius 530 px (49% W): fully visible at the face, gone
  where the bottom row starts.
* Nothing else: no shadows, no glows, no texture, square corners, no outlines.

## 11. Hierarchy and eye path

1. The face: the only light area and the only photograph.
2. The orange title, top left (94 px on black).
3. The headline number and the three stats (white, 71 and 50 px), found by their orange
   percent signs.
4. The three orange numerals (176 px, but thin and overprinted by white words, so they read
   as markers, not as content), then the caps words on them.
5. Notes, the tiny chart, the footer.

Title 94 px, headline number 71, stats 50, caps words 34, notes 18: about 5 : 4 : 2.8 : 1.9 : 1.

## 12. Build recipe

Supply `photo.png` (your image here): a portrait, taller than wide, the face in the upper
middle. Canvas 1080 x 1440. All figures are example data, invented.

```bash
pixelox new -o poster.pxo --size 1080x1440 --fill '#0d0d0d'
pixelox shape rect poster.pxo --in-place --name Panel --rect 227,324,562x950 --fill '#161616'
pixelox comp place poster.pxo photo.png --in-place --as Photo --size x950 --at 508,1274 --anchor b
pixelox adjust bw poster.pxo --in-place --layer Photo
pixelox adjust levels poster.pxo --in-place --layer Photo --in-black 12 --gamma 0.85 --out-white 215
pixelox mask add poster.pxo --in-place --layer Photo --from-path 'M 227 324 H 789 V 1274 H 227 Z'
pixelox mask add poster.pxo --in-place --layer Photo --gradient radial --from 520,600 --to 520,1130 --ease smooth --combine intersect
pixelox text add poster.pxo --in-place --name Title --content $'WHO NOW\nRETURNS\nTO CLAY?' --font @didone --weight 400 --size 94 --leading 81 --color '#e66a45' --at 73,92 --anchor tl --anchor-to ink
pixelox text add poster.pxo --in-place --name SideHead --content $'MAKERS\nCOME BACK' --font @oldstyle --weight 700 --size 35 --leading 36 --color '#ffffff' --at 797,189 --anchor tl --anchor-to ink
pixelox text add poster.pxo --in-place --name SideText --content $'Shared pottery studios\nhave opened in most\nlarge towns, and the\nwaiting list for a\nwheel keeps growing.' --font @neo-grotesque --weight 700 --size 18 --leading 25 --color '#ffffff' --at 797,280 --anchor tl
pixelox text add poster.pxo --in-place --name BigNum --content "47" --font @neo-grotesque --weight 300 --size 71 --color '#ffffff' --at 73,421 --anchor tl --anchor-to ink
pixelox text add poster.pxo --in-place --name BigPct --content "%" --font @didone --weight 400 --style italic --size 71 --color '#e66a45' --at 178,421 --anchor tl --anchor-to ink
pixelox text add poster.pxo --in-place --name BigLabel --content "GROWTH" --font @oldstyle --weight 700 --size 28 --color '#ffffff' --at 73,510 --anchor tl --anchor-to ink
pixelox text add poster.pxo --in-place --name BigNote --content $'in shared studio\nmemberships since\nthe last survey.' --font @neo-grotesque --weight 700 --size 18 --leading 25 --color '#ffffff' --at 73,556 --anchor tl
pixelox text add poster.pxo --in-place --name RatioA --content "2" --font @neo-grotesque --weight 300 --size 41 --color '#ffffff' --at 76,710 --anchor tl --anchor-to ink
pixelox text add poster.pxo --in-place --name RatioIn --content "IN" --font @oldstyle --weight 700 --size 26 --color '#e66a45' --at 114,716 --anchor tl --anchor-to ink
pixelox text add poster.pxo --in-place --name RatioB --content "5" --font @neo-grotesque --weight 300 --size 41 --color '#ffffff' --at 164,710 --anchor tl --anchor-to ink
pixelox text add poster.pxo --in-place --name RatioNote --content $'new members had\nnever thrown a pot\nbefore their first\nevening class.' --font @neo-grotesque --weight 700 --size 18 --leading 25 --color '#ffffff' --at 73,764 --anchor tl
pixelox text add poster.pxo --in-place --name S1Num --content "72" --font @neo-grotesque --weight 300 --size 50 --color '#ffffff' --at 884,499 --anchor tr --anchor-to ink
pixelox text add poster.pxo --in-place --name S1Pct --content "%" --font @didone --weight 400 --style italic --size 50 --color '#e66a45' --at 890,499 --anchor tl --anchor-to ink
pixelox text add poster.pxo --in-place --name S1Cap --content $'Sell at local\nmarkets' --font @neo-grotesque --weight 700 --size 18 --leading 25 --align center --color '#ffffff' --at 878,558 --anchor t
pixelox text add poster.pxo --in-place --name S1When --content "SPRING SURVEY" --font @neo-grotesque --weight 700 --size 14 --color '#ffffff' --at 878,622 --anchor t
pixelox text add poster.pxo --in-place --name S2Num --content "58" --font @neo-grotesque --weight 300 --size 50 --color '#ffffff' --at 951,670 --anchor tr --anchor-to ink
pixelox text add poster.pxo --in-place --name S2Pct --content "%" --font @didone --weight 400 --style italic --size 50 --color '#e66a45' --at 957,670 --anchor tl --anchor-to ink
pixelox text add poster.pxo --in-place --name S2Cap --content $'Teach weekly\nclasses' --font @neo-grotesque --weight 700 --size 18 --leading 25 --align center --color '#ffffff' --at 945,725 --anchor t
pixelox text add poster.pxo --in-place --name S2When --content "SUMMER SURVEY" --font @neo-grotesque --weight 700 --size 14 --color '#ffffff' --at 945,787 --anchor t
pixelox text add poster.pxo --in-place --name S3Num --content "41" --font @neo-grotesque --weight 300 --size 50 --color '#ffffff' --at 884,837 --anchor tr --anchor-to ink
pixelox text add poster.pxo --in-place --name S3Pct --content "%" --font @didone --weight 400 --style italic --size 50 --color '#e66a45' --at 890,837 --anchor tl --anchor-to ink
pixelox text add poster.pxo --in-place --name S3Cap --content $'Share a kiln\nwith others' --font @neo-grotesque --weight 700 --size 18 --leading 25 --align center --color '#ffffff' --at 878,892 --anchor t
pixelox text add poster.pxo --in-place --name S3When --content "AUTUMN SURVEY" --font @neo-grotesque --weight 700 --size 14 --color '#ffffff' --at 878,954 --anchor t
pixelox text add poster.pxo --in-place --name Num1 --content "1" --font @didone --weight 400 --size 176 --color '#e66a45' --at 73,986 --anchor tl --anchor-to ink
pixelox text add poster.pxo --in-place --name Num2 --content "2" --font @didone --weight 400 --size 176 --color '#e66a45' --at 321,986 --anchor tl --anchor-to ink
pixelox text add poster.pxo --in-place --name Num3 --content "3" --font @didone --weight 400 --size 176 --color '#e66a45' --at 570,986 --anchor tl --anchor-to ink
pixelox text add poster.pxo --in-place --name Word1 --content "LEARN" --font @oldstyle --weight 700 --size 34 --color '#ffffff' --at 73,1064 --anchor tl --anchor-to ink
pixelox text add poster.pxo --in-place --name Word2 --content "MAKE" --font @oldstyle --weight 700 --size 34 --color '#ffffff' --at 321,1064 --anchor tl --anchor-to ink
pixelox text add poster.pxo --in-place --name Word3 --content "SELL" --font @oldstyle --weight 700 --size 34 --color '#ffffff' --at 570,1064 --anchor tl --anchor-to ink
pixelox text add poster.pxo --in-place --name Text1 --content $'Most members start\nwith a six-week\nbeginner course\nat a shared studio.' --font @neo-grotesque --weight 700 --size 18 --leading 25 --color '#ffffff' --at 73,1134 --anchor tl
pixelox text add poster.pxo --in-place --name Text2 --content $'After a year, half\nrent a shelf and\nfire their own\nwork every month.' --font @neo-grotesque --weight 700 --size 18 --leading 25 --color '#ffffff' --at 321,1134 --anchor tl
pixelox text add poster.pxo --in-place --name Text3 --content $'One in four goes\non to sell at fairs,\nin shops, or online\nwithin two years.' --font @neo-grotesque --weight 700 --size 18 --leading 25 --color '#ffffff' --at 570,1134 --anchor tl
pixelox text add poster.pxo --in-place --name ChartHead --content "MEMBERS" --font @oldstyle --weight 700 --size 34 --color '#ffffff' --at 813,1064 --anchor tl --anchor-to ink
pixelox render js scripts/line.js -o lines.png --size 1080x1440 --param box=75.3,79.2,18.6,7.6 --param values='820,860,790,1240;640,660,480,760;410,190,290,380' --param smooth=1 --param markers=1 --param vlabels=none --param endlabels=0 --param legend=none --param axis=0 --param gridlines=0 --param width=5 --param colors='#cfc8c5,#c0603f,#8a3f2a' --param muted='#3a3a3a' --param fs=18
pixelox comp place poster.pxo lines.png --in-place --as Lines --at 0,0
pixelox shape rect poster.pxo --in-place --name Tag --rect 828,1122,90x26 --fill '#ed6640'
pixelox text add poster.pxo --in-place --name TagText --content "1,240" --font @neo-grotesque --weight 700 --size 16 --color '#ffffff' --at 873,1135 --anchor c
pixelox shape path poster.pxo --in-place --name TagLead --d 'M 924 1135 L 990 1135 L 1005 1141' --fill none --stroke '#e66a45' --stroke-width 2
pixelox text add poster.pxo --in-place --name ChartFrom --content "Year 1" --font @neo-grotesque --size 14 --color '#8f8f8f' --at 816,1262 --anchor l
pixelox text add poster.pxo --in-place --name ChartTo --content "Year 4" --font @neo-grotesque --size 14 --color '#8f8f8f' --at 1014,1262 --anchor r
pixelox shape line poster.pxo --in-place --name FootRule --from 65,1301 --to 1018,1301 --stroke '#a5a5a5' --stroke-width 2
pixelox shape rect poster.pxo --in-place --name MarkBox --rect 65,1334,173x57 --fill '#000000'
pixelox text add poster.pxo --in-place --name Mark --content "YOUR MARK" --font @neo-grotesque --weight 400 --size 25 --color '#ffffff' --at 151,1357 --anchor c
pixelox text add poster.pxo --in-place --name MarkLine --content "Studio Name Here" --font @neo-grotesque --style italic --size 12 --color '#c9c9c9' --at 151,1379 --anchor c
pixelox text add poster.pxo --in-place --name Fine1 --content $'Example data only. Every\nfigure on this sheet was\ninvented for the layout.' --font @neo-grotesque --weight 700 --size 12 --leading 15 --color '#ffffff' --at 294,1340 --anchor tl
pixelox text add poster.pxo --in-place --name Fine2 --content $'Shares are of members\nwho answered the yearly\nstudio survey.' --font @neo-grotesque --size 12 --leading 15 --color '#ffffff' --at 478,1340 --anchor tl
pixelox text add poster.pxo --in-place --name Fine3 --content $'Season labels show when\neach question was asked.\nBases differ by question.' --font @neo-grotesque --size 12 --leading 15 --color '#ffffff' --at 662,1340 --anchor tl
pixelox text add poster.pxo --in-place --name Fine4 --content $'web address\nmail address\nphone number' --font @neo-grotesque --size 13 --leading 19 --color '#ffffff' --at 872,1337 --anchor tl
pixelox export poster.pxo -o poster.png
```

Reuse: change the title (three lines, 8 letters at most each, or set the longest with
`--fit-width 54%`), then the six numbers and their notes. Every note is cut by hand into
lines of at most 20 characters; a longer line runs into the photo's light area. Chart: change
`values=` (series separated by `;`), then run the `render js` line with `--json` and read the
anchors: with these values the top line's last point `p1.4` is at 1014,1144, and the leader
ends 9 px short of it. Put the tag's figures in the same unit as the line they point to.
Move the mask center (`--from 520,600`) onto the face of your own photo, and its radius end
(`--to`) to where the shoulders should disappear. A logo replaces the wordmark with
`comp place poster.pxo logo.png --in-place --as Logo --size 150x --at 151,1362 --anchor c`.

## 13. Variations and pairings

* **A cutout instead of a framed photo:** place `subject.png` 60% W wide, bottom at 88% H,
  with `adjust bw` and a linear `mask add --gradient linear --from 0,60% --to 0,88%
  --combine intersect`; drop the panel. The modules keep their places.
* **Other accents:** one warm or acid color on black works the same way: `#d9b44a` (brass),
  `#c8f04a` (lime), `#ff4d6d` (rose). On a paper ground `#f3efe7` swap white for `#141414`
  and keep the orange.
* **Charts for the three stats:** when the three percentages share a base, replace the
  zigzag with three C-progress-ring marks (`r=4.5`, orange on a `#2a2a2a` track) or one
  C-progress-bars block in the right column.
* **A landscape spread (1600 x 1066):** photo panel in the middle third, the title and left
  stats in the left third, the three stats and the chart in the right third, the three
  chapters as a row under the photo.
* **More trend, less text:** widen the chart to two columns (box `52.8,74,41,12`) with
  `endlabels=1` and series names, and drop chapter 3. Pairs with C-sparkline beside the
  headline number.

## 14. Pitfalls

* **Three stats with the same value.** The original repeats one percentage three times
  (a template left unfilled). Three equal numbers in a row say "nothing differs": use real,
  different values, ordered high to low down the column.
* **Percentages with hidden bases.** Six numbers from different questions sit side by side.
  Each note must say "of whom" and "since when"; the period line under each stat and the
  fine print carry the base and the source. A page of numbers with no source line is an ad.
* **A chart with no scale.** A 200 px line chart without axis, units, or series names shows
  only "up". Keep the baseline, name the first and last period, and tie the tag to one
  point with its unit; if the lines need comparing, give the chart more room.
* **White text over the lit part of the photo.** Notes that run past 20 characters reach the
  hands or the face. Shorten the line, or move the mask center away, and check with
  `analyze stats` that the ground under the text is darker than `#303030`.
* **Color in the photo.** Any hue besides the accent halves the accent's force. Convert to
  black and white first, then cap the highlights (`--out-white 215`).
* **Hairline type too small.** A didone under 60 px or a light grotesque under 40 px loses
  its thin strokes on black. Use the bold serif and the bold grotesque for everything small,
  and never set fine print under 12 px.
