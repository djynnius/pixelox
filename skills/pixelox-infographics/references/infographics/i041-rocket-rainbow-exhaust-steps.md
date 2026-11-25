---
id: i041
title: Rocket on a six-step rainbow exhaust column
kind: [process, list]
topic: [business, generic]
mood: [clean, friendly, vibrant]
style: [flat, paper-cut]
format: landscape
structure: S-object-rows
data: [process, single-stat, text-only]
charts: [C-pyramid, C-rocket, C-stat-block, C-callout]
palette: ['#f9f9f9', '#f88030', '#fccc2f', '#c6e92e', '#50dae3', '#4fa3e1', '#2974a3']
needs: nothing
---

# i041: Rocket on a six-step rainbow exhaust column

## 1. Concept

A landscape sheet on near-white: a small pale rocket sits at the top of a tall exhaust column
that widens toward the ground and is cut into six colored steps, numbered 6 at the nozzle
down to 1 in the launch clouds. To the right, six text blocks stand level with the six
steps, each announced by a small dark arrow, and above them three key figures in thin
numerals end in a green check mark. The message is "six steps carry the project up, and
here is what it took". It works because the column is the list and the direction at once:
step 1 is the wide base, every later step stands on the one before, and the rocket is where
the steps lead.

## 2. Data story

* **Six ordered steps**, text only: each a title of 2 words (10 to 13 letters) and a
  paragraph. They are numbered from the bottom: 1 is the foundation, 6 the last step before
  launch, so the list is read upward although the page is read downward.
* **Three key figures** (single stats, no chart): a duration in days (3 digits), a budget
  (5 digits with a thousands separator and a currency mark), and a head-count range (two
  numbers with a dash and a person pictogram).
* **One status mark**: a check in a ring, "done" or "approved".

Placeholder content in the recipe (example data, invented): steps THE PROBLEM, THE TEAM, THE
DESIGN, FIRST VERSION, PUBLIC TRIAL, FINAL CHECKS; figures 240 days, 36,000 USD, 8 to 12
people.

## 3. Information architecture

1. **Key figures**, top right: what the whole project cost in time, money, and people. Thin
   gray numerals, dark units, three staggered lines.
2. **Check mark**, under the figures: the outcome.
3. **Rocket and column**, left third: six steps, 6 on top, 1 at the base.
4. **Six text blocks**, right: one per step at the step's own height, title in the step's
   color. A small arrow between column and block points from the step to its text.
5. **Clouds** at the foot of the column: the ground, and the end of the page.

The blocks' left edges step to the right, block by block, parallel to the column's slanted
side, so the gap between column and text stays the same from top to bottom.

## 4. Format and grid

* Landscape, 1.2 : 1. Recipe canvas 1600 x 1340. A hairline frame runs around the original.
* Object zone: the left 45% W, full height (rocket, column, clouds); the rows stand to its
  right, one per step at the step's height.
* Rocket and column axis at 29.5% W. Rocket: nose tip 3.7% H, nozzle end 30.9% H; body 16% W
  wide, fins from 19% to 40% W (the recipe's rocket, drawn with `aspect=2`: body 12.6% W,
  fins from 19% to 40% W).
* Column: from 30.9% to 96.4% H; 4.6% W wide at the top, 17.7% W at the bottom; six steps of
  10.9% H each. Step centers at 36.3, 47.3, 58.2, 69.1, 80.0, and 90.9% H.
* Arrows: at each step's center height, starting at 44% W for step 6 and 1.2% W further
  right for every step below.
* Text blocks: left edge 49% W for step 6, plus 1.2% W per step down to 55% W for step 1;
  41% W wide; title cap tops 5% H above each step's center.
* Key figures: cap tops at 4.7, 9.4, and 14.3% H; the three lines start at 41, 49, and 45% W
  (deliberately ragged). Check mark centered on 67.3% W, 21.9% H, ring radius 4.1% W.
* Clouds: 7% to 53% W, 87% to 96.8% H. Margins: 7% W left of the clouds, 8% W right.

## 5. Layer stack

1. Ground: `#f9f9f9`, flat.
2. Exhaust column: six steps, top to bottom orange `#f88030`, amber `#fccc2f`, lime
   `#c6e92e`, cyan `#50dae3`, blue `#4fa3e1`, deep blue `#2974a3`. The right half of the
   column is one shade darker (about 9% black over it): a paper fold on the axis.
3. Rocket on the column's top: pale blue-gray body `#dbe9ea` (right half `#bedcda`), an oval
   porthole with a white rim and cyan glass `#62ebec`, two dark teal fins `#4e7e7f` and
   `#32605d`, a teal nozzle `#456e67`. No flame: the column is the flame.
4. Clouds: a back row of gray-blue lobes `#a0adb0`, a front row of pale lobes `#dbe9ea`,
   hiding the bottom third of step 1.
5. Step numbers 6 to 1: thin white numerals on the axis.
6. Key figures: numerals `#b4b5b4`, units `#3c3c3c`; a filled person pictogram.
7. Check mark: a green ring `#7ac142`, 5 px, and a heavy green tick that breaks out of the
   ring at the top right.
8. Six arrows: small dark teal triangles `#3f6f72` pointing right.
9. Six titles in the step colors, six gray paragraphs.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#f9f9f9` | ground | 77% | nothing |
| `#f88030` | step 6, its title | 1% | step 6 |
| `#fccc2f` | step 5, its title (`#e3b21e` for type) | 1.5% | step 5 |
| `#c6e92e` | step 4, its title (`#a5c626` for type) | 2% | step 4 |
| `#50dae3` | step 3, its title (`#35bcc6` for type) | 2% | step 3 |
| `#4fa3e1` | step 2, its title (`#4a92cc` for type) | 2.5% | step 2 |
| `#2974a3` | step 1, its title | 2% | step 1 |

Also: rocket and front clouds `#dbe9ea` (10%), fins and arrows `#3f6f72`, check `#7ac142`,
units `#3c3c3c`, paragraphs `#8a8f90`. Categorical in spectrum order, warm at the nozzle and
cold at the ground, like a flame cooling into the sky. The titles reuse the step colors,
darkened one notch where the hue is too light for type on white (amber, lime, cyan). Green
appears only in the check mark.

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Key figure, number (x3) | 3 digits; 5 digits with separator; a range | - | geometric sans, light and wide (Raleway Light, Jost 300 / Avenir Next) | 300 | 64 | 0 | line pitch 70 px | right edge on the unit | `#a4a6a5` (the original: `#b4b5b4`) | lines at 4.7, 9.4, 14.3% H, starting 41, 49, 45% W |
| Key figure, unit (x2) | 1 word or a currency code | caps | geometric sans | 400 | 64 | 0 | - | left | `#3c3c3c` | 18 px after the number |
| Step number (x6) | 6 down to 1 | - | geometric sans, light | 300 | 60 | 0 | - | center | `#ffffff` | 29.5% W, step centers |
| Step title (x6) | 2 words, 10 to 13 letters | caps | geometric sans, light | 300 | 33 | 20 | - | left | the step's color | 49% to 55% W, cap top 5% H above the step's center |
| Step paragraph (x6) | the original: about 50 words on 4 justified lines with a first-line indent; the recipe: 18 to 20 words on 2 lines | sentence | geometric sans | 400 | 21 (the original: 14) | 0 | 135% | left | `#8a8f90` (the original: `#b0b0b0`) | same left edge, 38 px under the title's cap top |

One family, a light wide geometric sans, in two weights. The only heavy things on the page
are the units of the key figures and the tick. Sizes 64 : 60 : 33 : 21, about 3 : 3 : 1.6 :
1. The original's 14 px justified paragraphs are filler texture; the recipe sets real
sentences at 21 px.

## 8. Charts and diagrams

* **C-pyramid** as the exhaust column: `scripts/pyramid.js`, box `20.65,9.05,17.7,87.35`,
  **8 tiers** with empty labels (`labels='|||||||'`), `labelpos=in`, `gap=0`. The top two
  tiers hide behind the rocket and take step 6's orange; tiers 3 to 8 are steps 6 to 1. That
  gives a column that is already 4.4% W wide where it leaves the nozzle, as in the original,
  instead of starting at a point. The script logs the centers `tier3` to `tier8`
  (x 472; y 487, 633, 780, 926, 1072, 1219): the numbers and the arrows hang on them.
* The fold: one `shape path` triangle in black at 9% opacity over the column's right half
  (apex, bottom right corner, bottom center of the pyramid's box).
* **C-rocket**: `scripts/object.js`, `kind=rocket`, `aspect=2` (a short, stout rocket: twice
  the script's normal width at the same height), `bands=1`, `badges=none`, `flame=0`,
  `shade=0`, box `18.5,3.7,22,32.8`, body `#dbe9ea`, `fincolor='#3f6f72'`, `dark='#456e67'`.
  Its nozzle ends at 30.9% H, the top of tier 3. The rocket's fold: a `shape rect` in black
  at 9% opacity over everything right of the axis, attached to the rocket with `layer clip`
  so it darkens only the rocket's right half and right fin. The porthole: two native ovals
  (`shape ellipse`, 112 x 70 px white and 88 x 48 px cyan `#62ebec`) 17 px above the script's
  `band1` anchor (472, 222). The script's own porthole (`window=1`) is not used: its pale
  rim does not show on this pale body and its glass is a fixed sky blue.
* **C-callout**, minimal variant: no leader line at all, only a 27 px triangle between each
  step and its block.
* **C-stat-block** x3, type only, stacked and ragged.
* The steps are equal in height: they show order, not amounts. The column's widening is the
  shape of an exhaust plume and encodes nothing.
* Simplified against the original: the original rocket is egg-shaped (body 16% W, curved
  sides) with angular fins; its porthole is larger (9% W wide against 7% W) and casts a long
  diagonal shadow. The original numerals are a hairline wide face; the recipe uses the
  lightest geometric sans installed.

## 9. Icons and illustration

* Person pictogram after the third figure: `user`, 60 px, `#3c3c3c`, filled.
* Check mark: `shape ellipse` ring, 130 px, 5 px green line; `check`, 150 px, 13 px line,
  placed 14 px right of and 21 px above the ring's center so its long arm leaves the ring.
* Arrows: six `shape path` triangles, 27 px wide and 24 px high.
* Clouds: eight `shape ellipse` lobes in two tones, cut off at the bottom by a ground-colored
  strip so they stand on a straight line at 96.8% H.
* Porthole: two `shape ellipse` layers on the rocket's upper body, a white rim and cyan
  glass (section 8).
* The rocket could be your own `illustration.png`: 21% W wide, nozzle on the axis at 30.9% H.

## 10. Effects and finish

Flat with a paper-cut touch: every solid (rocket, column) has a lighter left half and a
darker right half meeting on the axis, and nothing else is shaded. No drop shadows, no
gradients, no outlines except the check ring. In the recipe a 9% black rectangle clipped to
the rocket (`layer clip`) and a 9% black triangle over the column darken the right halves;
the rocket is rendered with `shade=0` so the fold is its only shading.

## 11. Hierarchy and eye path

1. The rainbow column: the only saturated area, running the full height.
2. The rocket on top of it, then the key figures beside the rocket (64 px, but pale).
3. The green check.
4. Down the right side, block by block, each arrow pulling the eye from a step to its title.

Figures 64 px, step numbers 60, titles 33, paragraphs 21. Weight, not size, makes the
ranking: the units and the tick are the only dark, heavy marks, so "DAYS", the currency, and
the check are noticed first among the type.

## 12. Build recipe

Nothing to supply. Canvas 1600 x 1340. All figures and copy are example data, invented.

```bash
pixelox new -o launch.pxo --size 1600x1340 --fill '#f9f9f9'
pixelox render js scripts/pyramid.js -o plume.png --size 1600x1340 --param box=20.65,9.05,17.7,87.35 --param labels='|||||||' --param labelpos=in --param gap=0 --param colors='#f88030,#f88030,#f88030,#fccc2f,#c6e92e,#50dae3,#4fa3e1,#2974a3'
pixelox comp place launch.pxo plume.png --in-place --as Plume --at 0,0
pixelox shape path launch.pxo --in-place --name Fold --d 'M 472 121 L 614 1292 L 472 1292 Z' --fill '#000000' --opacity 9
pixelox render js scripts/object.js -o body.png --size 1600x1340 --param box=18.5,3.7,22,32.8 --param kind=rocket --param aspect=2 --param bands=1 --param badges=none --param flame=0 --param shade=0 --param colors='#dbe9ea' --param fincolor='#3f6f72' --param dark='#456e67'
pixelox comp place launch.pxo body.png --in-place --as Rocket --at 0,0
pixelox shape rect launch.pxo --in-place --name RocketFold --rect 472,40,190x380 --fill '#000000' --opacity 9
pixelox layer clip launch.pxo --in-place --layer RocketFold
pixelox shape ellipse launch.pxo --in-place --name PortRim --rect 416,170,112x70 --fill '#ffffff'
pixelox shape ellipse launch.pxo --in-place --name PortGlass --rect 428,181,88x48 --fill '#62ebec'
pixelox shape ellipse launch.pxo --in-place --name CloudB1 --rect 170,1218,250x150 --fill '#a0adb0'
pixelox shape ellipse launch.pxo --in-place --name CloudB2 --rect 570,1196,200x170 --fill '#a0adb0'
pixelox shape ellipse launch.pxo --in-place --name CloudB3 --rect 690,1246,170x110 --fill '#a0adb0'
pixelox shape ellipse launch.pxo --in-place --name CloudF1 --rect 110,1262,300x120 --fill '#dbe9ea'
pixelox shape ellipse launch.pxo --in-place --name CloudF2 --rect 250,1226,220x170 --fill '#dbe9ea'
pixelox shape ellipse launch.pxo --in-place --name CloudF3 --rect 420,1250,220x150 --fill '#dbe9ea'
pixelox shape ellipse launch.pxo --in-place --name CloudF4 --rect 530,1206,230x190 --fill '#dbe9ea'
pixelox shape ellipse launch.pxo --in-place --name CloudF5 --rect 680,1270,190x100 --fill '#dbe9ea'
pixelox shape rect launch.pxo --in-place --name Floor --rect 0,1298,1600x42 --fill '#f9f9f9'
pixelox text add launch.pxo --in-place --name S6 --content "6" --font @geometric-sans --weight 300 --size 60 --color '#ffffff' --at 472,487 --anchor c --anchor-to ink
pixelox text add launch.pxo --in-place --name S5 --content "5" --font @geometric-sans --weight 300 --size 60 --color '#ffffff' --at 472,633 --anchor c --anchor-to ink
pixelox text add launch.pxo --in-place --name S4 --content "4" --font @geometric-sans --weight 300 --size 60 --color '#ffffff' --at 472,780 --anchor c --anchor-to ink
pixelox text add launch.pxo --in-place --name S3 --content "3" --font @geometric-sans --weight 300 --size 60 --color '#ffffff' --at 472,926 --anchor c --anchor-to ink
pixelox text add launch.pxo --in-place --name S2 --content "2" --font @geometric-sans --weight 300 --size 60 --color '#ffffff' --at 472,1072 --anchor c --anchor-to ink
pixelox text add launch.pxo --in-place --name S1 --content "1" --font @geometric-sans --weight 300 --size 60 --color '#ffffff' --at 472,1219 --anchor c --anchor-to ink
pixelox text add launch.pxo --in-place --name K1n --content "240" --font @geometric-sans --weight 300 --size 64 --color '#a4a6a5' --at 790,63 --anchor tr --anchor-to ink
pixelox text add launch.pxo --in-place --name K1u --content "DAYS" --font @geometric-sans --weight 400 --size 64 --color '#3c3c3c' --at 808,63 --anchor tl --anchor-to ink
pixelox text add launch.pxo --in-place --name K2n --content "36,000" --font @geometric-sans --weight 300 --size 64 --color '#a4a6a5' --at 1000,133 --anchor tr --anchor-to ink
pixelox text add launch.pxo --in-place --name K2u --content "USD" --font @geometric-sans --weight 400 --size 64 --color '#3c3c3c' --at 1018,133 --anchor tl --anchor-to ink
pixelox text add launch.pxo --in-place --name K3n --content "8-12" --font @geometric-sans --weight 300 --size 64 --color '#a4a6a5' --at 900,203 --anchor tr --anchor-to ink
pixelox shape icon launch.pxo user --in-place --name K3u --size 60 --at 942,228 --anchor c --color '#3c3c3c' --fill '#3c3c3c' --stroke-width 4
pixelox shape ellipse launch.pxo --in-place --name CheckRing --rect 1013,228,130x130 --fill none --stroke '#7ac142' --stroke-width 5
pixelox shape icon launch.pxo check --in-place --name Check --size 150 --at 1092,272 --anchor c --color '#7ac142' --stroke-width 13
pixelox shape path launch.pxo --in-place --name A6 --d 'M 703 475 L 730 487 L 703 499 Z' --fill '#3f6f72'
pixelox text add launch.pxo --in-place --name T6 --content "FINAL CHECKS" --font @geometric-sans --weight 300 --size 33 --tracking 20 --color '#f88030' --at 785,420 --anchor tl --anchor-to ink
pixelox text add launch.pxo --in-place --name P6 --content $'Walk through the launch list with the whole team, fix\nthe last blockers, and agree on who decides to go.' --font @geometric-sans --size 21 --leading 135% --color '#8a8f90' --at 785,458 --anchor tl
pixelox shape path launch.pxo --in-place --name A5 --d 'M 722 621 L 749 633 L 722 645 Z' --fill '#3f6f72'
pixelox text add launch.pxo --in-place --name T5 --content "PUBLIC TRIAL" --font @geometric-sans --weight 300 --size 33 --tracking 20 --color '#e3b21e' --at 804,566 --anchor tl --anchor-to ink
pixelox text add launch.pxo --in-place --name P5 --content $'Open the product to two hundred outside users and\nwatch where they stop, ask, or give up.' --font @geometric-sans --size 21 --leading 135% --color '#8a8f90' --at 804,604 --anchor tl
pixelox shape path launch.pxo --in-place --name A4 --d 'M 741 768 L 768 780 L 741 792 Z' --fill '#3f6f72'
pixelox text add launch.pxo --in-place --name T4 --content "FIRST VERSION" --font @geometric-sans --weight 300 --size 33 --tracking 20 --color '#a5c626' --at 823,713 --anchor tl --anchor-to ink
pixelox text add launch.pxo --in-place --name P4 --content $'Build the smallest thing that does the main job from\nstart to finish, with no extras at all.' --font @geometric-sans --size 21 --leading 135% --color '#8a8f90' --at 823,751 --anchor tl
pixelox shape path launch.pxo --in-place --name A3 --d 'M 760 914 L 787 926 L 760 938 Z' --fill '#3f6f72'
pixelox text add launch.pxo --in-place --name T3 --content "THE DESIGN" --font @geometric-sans --weight 300 --size 33 --tracking 20 --color '#35bcc6' --at 842,859 --anchor tl --anchor-to ink
pixelox text add launch.pxo --in-place --name P3 --content $'Sketch three ways to solve it, test them on paper\nwith five people, and keep the clearest one.' --font @geometric-sans --size 21 --leading 135% --color '#8a8f90' --at 842,897 --anchor tl
pixelox shape path launch.pxo --in-place --name A2 --d 'M 779 1060 L 806 1072 L 779 1084 Z' --fill '#3f6f72'
pixelox text add launch.pxo --in-place --name T2 --content "THE TEAM" --font @geometric-sans --weight 300 --size 33 --tracking 20 --color '#4a92cc' --at 861,1005 --anchor tl --anchor-to ink
pixelox text add launch.pxo --in-place --name P2 --content $'Find the three skills the plan cannot do without and\none person who truly owns each of them.' --font @geometric-sans --size 21 --leading 135% --color '#8a8f90' --at 861,1043 --anchor tl
pixelox shape path launch.pxo --in-place --name A1 --d 'M 798 1207 L 825 1219 L 798 1231 Z' --fill '#3f6f72'
pixelox text add launch.pxo --in-place --name T1 --content "THE PROBLEM" --font @geometric-sans --weight 300 --size 33 --tracking 20 --color '#2974a3' --at 880,1152 --anchor tl --anchor-to ink
pixelox text add launch.pxo --in-place --name P1 --content $'Write down who has the problem, how often it hurts,\nand what they pay today to live with it.' --font @geometric-sans --size 21 --leading 135% --color '#8a8f90' --at 880,1190 --anchor tl
pixelox export launch.pxo -o launch.png
```

Reuse: everything on the right hangs on the tier anchors that `scripts/pyramid.js` logs
(run its `render js` line with `--json`). For a step whose tier center is at y: the number
sits at (472, y); the arrow's tip is at y; the title's cap top is y - 67; the paragraph
starts at y - 29. Each step down moves the arrow and the block 19 px to the right. For N
steps use N + 2 tiers (two hidden behind the rocket), give the first three colors the top
step's hue, and keep the box: the tiers get taller or shorter by themselves. The fold
triangle uses three corners of the pyramid's box: top center, bottom right, bottom center.

## 13. Variations and pairings

* **Four or five steps:** `labels='|||||'` (6 tiers) or `'||||||'` (7 tiers); the steps
  grow to 220 or 175 px and take three-line paragraphs.
* **Read downward:** number the steps 1 at the nozzle to 6 at the ground when the story is
  a countdown or a descent (a rollout from head office to branches), and say so in a title.
* **One hue:** six steps of one blue from `#d6ecfb` at the nozzle to `#1f5f8f` at the base
  read as "building up" more calmly than the rainbow; keep the green check.
* **Real quantities:** if the steps have sizes (weeks, cost), keep the column as it is and
  put the figure at the end of each title ("THE TEAM, 6 WEEKS"); for a true comparison add
  C-bars to the right of the blocks instead of resizing the steps.
* **Relatives:** i038 puts the bands on the rocket itself, with callouts on both sides;
  i020 has three bands on a flat rocket and one column of numbered notes; i039 runs five
  bars sideways from a white rocket. i040 and i046 are the closest in idea: there the
  exhaust fans out into four numbered columns that stand side by side. Here it stays one
  stack, which leaves the whole right half for text and suits six or more steps.
* Pairs with C-timeline along the bottom in place of the clouds when the steps have dates.

## 14. Pitfalls

* **Numbering against the reading order without saying it.** The steps are numbered from
  the ground up, the text is read from the top down, so a reader meets step 6 first. That is
  right for "what it stands on", wrong for instructions. For a how-to, number from the top.
* **Step heights or widths read as amounts.** The base step is four times as wide as the top
  one only because a plume spreads. Never let the widening stand for a real quantity, and
  never resize steps by eye to suggest effort: print the figures. This is the data-integrity
  rule of the design.
* **Key figures without units.** Thin gray numerals alone say nothing; each needs its dark
  unit beside it (a word, a currency code, a pictogram), and the budget needs its currency
  and period in the text if it is not a one-off.
* **Titles in the raw step colors.** Amber, lime, and cyan vanish on a near-white ground in
  a light weight. Darken them one notch for type (`#e3b21e`, `#a5c626`, `#35bcc6`) or set
  all titles in `#3c3c3c` and let the arrows link them.
* **Text blocks on one vertical line.** Then the gap to the column grows from 10% W at the
  base to 16% W at the top and the upper blocks seem to float away. Step the blocks with the
  column's slope.
* **A flame under the rocket.** The column is the exhaust; a second flame (`flame=1`) covers
  step 6's number.
