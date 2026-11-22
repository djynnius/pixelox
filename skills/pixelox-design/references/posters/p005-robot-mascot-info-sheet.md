---
id: p005
title: Robot mascot info sheet with bubbles
genre: [tech-ad, corporate, editorial]
mood: [friendly, calm, futuristic, fresh]
style: [illustration, 3d-render, flat]
format: portrait-a
archetype: A-info-sheet
palette: ['#eae7d0', '#262831', '#929397', '#77707a', '#7a3fb8', '#3fbf5a']
techniques: [T-label-box, T-cutout-drop-shadow]
needs: subject-cutout
---

# p005: Robot mascot info sheet with bubbles

## 1. Concept

An explainer poster about technology trends: a pale ecru page, a squared techno headline at
the top left, a small friendly robot standing in the lower left, and three pill-shaped
section headers with bullet points stacked down the right. Two large soap-bubble spheres
with iridescent violet and teal rims sit behind the robot and in the top right corner. It
works because the light ground and the single cute character make a dry, list-heavy topic
feel approachable, and the pills turn nine lines of body text into three scannable chunks.

## 2. Format and grid

* Portrait A-series (1 : 1.414). Left margin 10% W, right margin 6% W, top 5.5% H, bottom
  4.5% H.
* Two columns. Left column 10 to 47% W: brand label, headline, sub-headline on top
  (5 to 31% H), the character below (33 to 97% H, bleeding off the left edge). Right
  column 47 to 94% W: three section blocks at 35 to 52%, 54 to 71%, and 73 to 90% H.
* The headline spans both columns' upper left (up to 58% W); the top right corner is
  filled by a bubble, so the two columns swap weight between the top and bottom halves:
  text top left, object top right; object bottom left, text bottom right.
* Each section block is one pill (5.9% H tall) plus four body lines at a pitch of 2.4% H,
  with 2.3% H of air between the pill and the first line.

## 3. Layer stack

1. Ground: pale ecru `#eae7d0`, full canvas, flat.
2. Bubble A (top right): a circle of radius 25% W centered at 88% W, 15.4% H, cut off by
   the top and right edges. The body is a shade lighter than the ground (`#f1efdf`); the
   rim is violet `#7a3fb8` at the very edge, turning teal `#4fb8aa` and fading out 5% W
   inward.
3. Bubble B (behind the character's head): radius 27% W centered at 21.7% W, 51% H, same
   treatment; its left third leaves the canvas.
4. One soft white highlight arc on each bubble, on the upper left of the rim, 60% opacity.
5. Contact shadow: a blurred gray ellipse under the character's feet at 95.5% H.
6. Character: a white-and-black toy robot with a ball head as wide as its body is tall
   (head diameter 43% W), two small green triangular eyes, shown full length, turned
   slightly right. Box `-3%,35%,45% x 62%`.
7. Brand label: small display caps at 10% W, cap top 5.6% H.
8. Headline: two lines of dark display caps, left at 10% W, cap tops 11.1% and 17.9% H,
   widths 40% and 48% W.
9. Sub-headline: two lines of light sentence-case sans at 10% W, 25.6% and 28.6% H.
10. Three pills: `47.3%,35.4%,46.1% x 5.9%`, `48.6%,54.1%,46.2% x 5.9%`,
    `48.6%,72.9%,46.2% x 5.9%`; fully round ends; gradient from gray-violet `#776d80` on
    the left to near-white `#f3f2ee` on the right; 1.5 px dark outline.
11. Pill titles: dark display caps, centered in each pill.
12. Bullet lists: two bullets of two lines each under every pill, dark gray, text left
    edge at 55 to 56% W, bullet dots 1.8% W to the left of it.
13. Web address: display caps, centered at 55.6% W, 94.7% H.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#eae7d0` | ecru ground | 65% |
| `#262831` | headline, pill titles, robot joints | 8% |
| `#929397` | robot body shading, pill mid-tone | 15% |
| `#77707a` | pill start color, bubble shadow side | 7% |
| `#7a3fb8` | violet of the bubble rims (with teal `#4fb8aa`) | 4% |
| `#3fbf5a` | the robot's eyes: the only saturated spot | under 1% |

Warm neutral ground with cool grays; color appears only as thin iridescent rims and two
green eyes. Roughly 65 / 30 / 5.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Brand label | `[BRAND]`, 1 word, 8 letters | caps | squared techno display (Squada One, Chakra Petch 700, Teko 700 / Helvetica Neue Condensed Black) | 900 | 1.9% | 60 | - | left | `#262831` | 10%, cap top 5.6% |
| Headline | 3 words on 2 lines, 9 to 10 characters per line | caps | same | 700 | 4.8% | 60 | line gap 40% of cap height | left | `#262831` | 10%, cap tops 11.1% and 17.9% |
| Sub-headline | 4 to 5 words on 2 lines | title | geometric sans (Montserrat 300 / Avenir) | 300 | 1.9% | 20 | 150% | left | `#4b4a4c` | 10%, cap tops 25.6% and 28.6% |
| Pill title (x3) | a 2 to 3 word topic, 16 to 23 characters | caps | squared techno display | 700 | 2.1% | 60 | - | center | `#262831` | pill centers 70.4%, 71.7%, 71.7% W; 38.4%, 57.1%, 75.8% H |
| Bullets (x3) | 2 bullets, each 3 to 4 words on 2 lines | sentence | geometric sans (Montserrat 400 / Avenir) | 400 | 1.6% | 30 | 110% (pitch 2.4% H) | left | `#4b4a4c` | 54.8 to 56.2% W; first lines 43.7%, 62.5%, 81.2% H |
| Web address | one URL | caps | squared techno display | 700 | 1.7% | 100 | - | center | `#262831` | 55.6%, 94.7% |

Two families: a squared, slightly condensed display face for every title, and a light
geometric sans for reading text. Headline cap height is 2.3x the pill titles and 3x the
body.

## 6. Imagery

One character render with transparency: a toy-like robot, glossy white shell with black
joints, an oversized spherical head, small green eyes, short legs, standing with its arms
hanging. It is cropped by the left edge so it seems to walk in. Soft studio light from
the upper right; a faint gray contact shadow under the feet (T-cutout-drop-shadow, soft
variant, drawn as a blurred ellipse). Any single mascot, device render, or product on a
transparent background works; keep it low-saturation so the rims stay the only color.

The two bubbles are drawn, not photographed: a faint white disc, an inner glow that runs
violet to teal from the edge inward, and one white highlight arc.

## 7. Effects and textures

* Bubbles: one `shape path` holding both discs in `#f1efdf`, then one `fx inner-glow` on
  it with gradient stops read from the edge inward (`--stops`: violet `#7a3fb8` to 12%,
  teal `#4fb8aa` at 40%, transparent at the end; `--blur 90`, `--blend normal`, opacity
  100). The glow follows the circles only, also where they leave the canvas.
* Highlight arcs: one `shape path` with two open arcs, white 14 px round-capped stroke,
  blurred with sigma 6, 60% opacity.
* **T-label-box** (pill form): `shape rect` with `--radius` = half the height, a
  left-to-right `fx overlay` gradient, and a 2 px inside `fx stroke` (a stroke set on the
  shape itself would be hidden by the overlay).
* No grain, no vignette: the page is clean.

## 8. Hierarchy and eye path

1. The robot's head (largest shape, the only face) and its bubble.
2. Up to the headline, the darkest and heaviest type.
3. Across to the first pill and down the three pills (the gradient pills read as a
   column of buttons).
4. Bullets inside each block; the web address last.

Cap heights: headline 4.8 : pill titles 2.1 : sub-headline 1.9 : body 1.6, about 3 : 1.3 :
1.2 : 1. The steps are small; the dark display face against the light sans supplies the
contrast that size does not.

## 9. Build recipe

`subject.png` is your image here: a mascot, robot, or product render with transparency
(tall format). Canvas 1080x1528.

```bash
pixelox new -o sheet.pxo --size 1080x1528 --fill '#eae7d0'
pixelox shape path sheet.pxo --in-place --name Bubbles --d 'M 682 235 A 268 268 0 1 1 1218 235 A 268 268 0 1 1 682 235 Z M -56 780 A 290 290 0 1 1 524 780 A 290 290 0 1 1 -56 780 Z' --fill '#f1efdf'
pixelox fx inner-glow sheet.pxo --in-place --layer Bubbles --blur 90 --stops '#7a3fb8@0,#7a3fb8@0.12,#4fb8aa@0.4,#4fb8aa00@1' --blend normal --opacity 100
pixelox shape path sheet.pxo --in-place --name Shine --d 'M 722 190 A 235 235 0 0 1 860 22 M 20 640 A 255 255 0 0 1 190 528' --fill none --stroke '#ffffff' --stroke-width 14 --cap round
pixelox filter gaussian sheet.pxo --in-place --layer Shine --sigma 6
pixelox layer opacity sheet.pxo --in-place --layer Shine --value 60
pixelox shape ellipse sheet.pxo --in-place --name Shadow --rect 3%,93.8%,36%x3.4% --fill '#8a8878'
pixelox filter gaussian sheet.pxo --in-place --layer Shadow --sigma 14
pixelox comp place sheet.pxo subject.png --in-place --as Mascot --size x800 --at 0-110,97% --anchor bl
pixelox text add sheet.pxo --in-place --name Brand --content "GRIDLOOM" --font "Helvetica Neue" --weight 900 --stretch condensed --size 2.6% --tracking 60 --color '#262831' --at 10%,6.5% --anchor l
pixelox text add sheet.pxo --in-place --name H1 --content "MACHINES" --font "Helvetica Neue" --weight 900 --stretch condensed --size 6.6% --tracking 60 --color '#262831' --at 10%,11.1% --anchor tl --anchor-to ink
pixelox text add sheet.pxo --in-place --name H2 --content "THAT LEARN" --font "Helvetica Neue" --weight 900 --stretch condensed --size 6.6% --tracking 60 --color '#262831' --at 10%,17.9% --anchor tl --anchor-to ink
pixelox text add sheet.pxo --in-place --name Sub --content $'Four Ideas Changing\nHow We Work' --font Avenir --weight 300 --size 2.65% --tracking 20 --leading 150% --color '#4b4a4c' --at 10%,28.1% --anchor l
pixelox shape rect sheet.pxo --in-place --name Pill1 --rect 47.3%,35.4%,46.1%x5.9% --radius 45 --fill '#776d80'
pixelox fx overlay sheet.pxo --in-place --layer Pill1 --stops '#776d80@0,#f3f2ee@1' --angle 0
pixelox fx stroke sheet.pxo --in-place --layer Pill1 --width 2 --position inside --color '#3a3a42'
pixelox shape rect sheet.pxo --in-place --name Pill2 --rect 48.6%,54.1%,46.2%x5.9% --radius 45 --fill '#776d80'
pixelox shape rect sheet.pxo --in-place --name Pill3 --rect 48.6%,72.9%,46.2%x5.9% --radius 45 --fill '#776d80'
pixelox fx copy sheet.pxo --in-place --from Pill1 --to Pill2,Pill3
pixelox text add sheet.pxo --in-place --name T1 --content "ADAPTIVE SOFTWARE" --font "Helvetica Neue" --weight 900 --stretch condensed --size 2.9% --tracking 60 --color '#262831' --at 70.4%,38.6% --anchor c
pixelox text add sheet.pxo --in-place --name T2 --content "ROBOTS ON THE FLOOR" --font "Helvetica Neue" --weight 900 --stretch condensed --size 2.9% --tracking 60 --color '#262831' --at 71.7%,57.3% --anchor c
pixelox text add sheet.pxo --in-place --name T3 --content "CONNECTED THINGS" --font "Helvetica Neue" --weight 900 --stretch condensed --size 2.9% --tracking 60 --color '#262831' --at 71.7%,76% --anchor c
pixelox text add sheet.pxo --in-place --name Dots1 --content $'•\n\n•' --font Avenir --weight 400 --size 2.2% --leading 110% --color '#4b4a4c' --at 52.8%,43% --anchor tl
pixelox text add sheet.pxo --in-place --name B1 --content $'Tools that adjust to\nthe way you work\nHelp that arrives\nbefore you ask' --font Avenir --weight 400 --size 2.2% --tracking 30 --leading 110% --color '#4b4a4c' --at 54.8%,43% --anchor tl
pixelox text add sheet.pxo --in-place --name Dots2 --content $'•\n\n•' --font Avenir --weight 400 --size 2.2% --leading 110% --color '#4b4a4c' --at 54.2%,61.8% --anchor tl
pixelox text add sheet.pxo --in-place --name B2 --content $'Safer, steadier\nproduction lines\nPeople freed for\nharder problems' --font Avenir --weight 400 --size 2.2% --tracking 30 --leading 110% --color '#4b4a4c' --at 56.2%,61.8% --anchor tl
pixelox text add sheet.pxo --in-place --name Dots3 --content $'•\n\n•' --font Avenir --weight 400 --size 2.2% --leading 110% --color '#4b4a4c' --at 54.2%,80.5% --anchor tl
pixelox text add sheet.pxo --in-place --name B3 --content $'Homes and offices\nthat sense and respond\nLive data from\nevery device' --font Avenir --weight 400 --size 2.2% --tracking 30 --leading 110% --color '#4b4a4c' --at 56.2%,80.5% --anchor tl
pixelox text add sheet.pxo --in-place --name Site --content "WWW.EXAMPLE.COM" --font "Helvetica Neue" --weight 900 --stretch condensed --size 2.35% --tracking 100 --color '#262831' --at 55.6%,94.7% --anchor c
pixelox export sheet.pxo -o sheet.png
```

Change first: the mascot (set `--size` so its head is about 43% W wide and sits inside
the lower bubble; `--at 0-110,97%` starts it 110 px left of the canvas edge), then the
three pill titles (16 to 23 characters) and bullets (two per block, 16 to 24 characters
per line). With a real squared display font registered (`fonts add`), use it for every
Helvetica Neue 900 line and drop the tracking to 20.

## 10. Variations and pairings

* Dark mode: ground `#14151c`, type `#eceadb`, pills from `#3a3550` to `#14151c`, the same
  rims; the bubble discs then become `#1b1c26`, one shade lighter than the ground.
* Swap the three pills for three numbered blocks ("01", "02", "03" in the display face at
  cap height 3% H) for a process or how-it-works sheet.
* Other subjects: a product render with feature bullets, a class syllabus with a mascot,
  a "meet the team" sheet with one cutout person.
* Square carousel slide: keep one pill block and the character, drop the sub-headline.
* Pairs with T-pattern (a faint dot grid fading out behind the right column), T-rules
  (a hairline rule separating the blocks instead of air), and T-spotlight under the character.

## 11. Pitfalls

* Bubbles as flat colored circles. A bubble is a rim, not a disc: keep the fill within
  3% lightness of the ground and put the color in the inner glow only.
* Three pills of different widths or with different left edges. Use one width; the
  original shifts the first pill 1.3% W to the left only because the bubble rim passes
  beside it.
* Body lines that wrap into three lines: the block rhythm (pill, four lines, gap) breaks.
  Keep each bullet to two lines of at most 24 characters.
* A saturated mascot: it fights the iridescent rims. Desaturate it
  (`adjust vibrance --vibrance -40` on the placed layer) or recolor the rims to match it.
* Headline in the same light sans as the body: the page turns into a document. The
  squared display face is what makes it a poster.
