---
id: i043
title: Four-circle Venn on charcoal with a red core
kind: [comparison, explainer]
topic: [media, generic]
mood: [dark, bold, editorial, clean]
style: [flat]
format: square
structure: S-overlap-diagram
data: [relationship, text-only]
charts: [C-venn, C-title-block]
palette: ['#313038', '#c1c418', '#689dbc', '#b86f80', '#9c9c9e', '#b3242c', '#ffffff']
needs: nothing
---

# i043: Four-circle Venn on charcoal with a red core

## 1. Concept

A near-square concept diagram on a charcoal ground: four large translucent circles (top,
left, right, bottom) overlap so deeply that they form a flower with a small red core where
all four meet. Each circle is one ingredient named in tall black capitals; every overlap
carries one lower-case word that names what you get with only those ingredients; the red
core carries the answer to the title's question in white capitals. The message is "you need
all four; here is what each missing one costs you". It works because the reader's eye goes
straight to the only saturated red, then outward through the named failures, and the one-word
labels turn a diagram into thirteen tiny verdicts.

## 2. Data story

No numbers. A relationship between 4 sets, text only:

* 4 set names, 1 word each, 4 to 12 letters.
* 4 pairwise overlap words (two neighboring sets only), 1 word of 6 to 14 letters each.
* 4 triple overlap words (three sets, one missing), 1 word of 6 to 9 letters each.
* 1 core name, 2 words on 2 lines.
* 4 outside notes that list what each set consists of: 3 or 4 single words. Top and bottom
  notes are one line with commas; left and right notes are stacks of 4 one-word lines.
* 1 title: a question of 5 to 6 words on one line.

13 regions in all. The layout cannot show two opposite sets without a neighbor (top with
bottom only, left with right only): see Pitfalls.

Placeholder content in the recipe (invented, about giving a talk): sets IDEA, SLIDES, VOICE,
REHEARSAL; pair words lecture, speech, polished, smooth; triple words rambling, flat,
abstract, empty; core GREAT TALK.

## 3. Information architecture

1. **Title**: the question, centered, with a hairline under it.
2. **The Venn**: the answer. Reading order is core first (red, white caps), then the four
   set names (largest type in the diagram), then the triple words around the core, then the
   pair words in the petals.
3. **Outside notes**: one small gray list beside each circle, on the side away from the
   center: what that set is made of.
4. **Credit**: two short right-aligned lines under a second hairline.

The two hairlines frame the diagram like a slide; nothing else divides the page.

## 4. Format and grid

* Near square, 1.044 : 1. Recipe canvas 1080 x 1030. Side margins 5.9% W (the hairlines).
* Title centered on 50% W at 5% H, 68% W wide.
* Hairlines at 9.1% H and 91.9% H, from 5.9% to 94.6% W.
* Diagram center at 50.2% W, 50.2% H. Circle radius 19.6% W. Circle centers 12.6% W from the
  diagram center (0.64 of the radius): straight up, right, down, and left. The whole flower
  spans 18.2% to 82% W and 16.6% to 84% H.
* Core (the region inside all four circles): a square with bulging sides, 14% W across.
* Set names: 0.62 to 0.72 of the radius outside each circle's center: 50.4% W, 22.6% H (top);
  25.5% W, 49.4% H (left); 75.4% W, 49.4% H (right); 50.4% W, 76.1% H (bottom).
* Pair words at 39.2% and 61.3% W on 35.4% and 64.6% H. Triple words at 50.1% W on 40.1% and
  59.8% H, and at 38.8% and 61.6% W on 50.1% H.
* Outside notes: top at 12.7% H, bottom at 87.2% H, both centered on 50.2% W; left stack
  centered on 10.9% W, right stack on 89.5% W, both centered on 50% H.
* Credit right-aligned at 94.6% W, between 94% and 97% H.

## 5. Layer stack

1. Ground: charcoal with a blue cast, `#2a2931` at the corners rising to `#3f3e46` in the
   upper middle (a soft radial vignette), with a fine linen-like grain.
2. Two hairlines `#5a5962`, 2 px.
3. Four circles, flat fills at about 94% opacity: top olive yellow `#c1c418`, left blue
   `#689dbc`, right dusty pink `#b86f80`, bottom gray `#9c9c9e`. No outlines.
4. Overlap regions, each its own color in the original (section 8 lists the thirteen hexes):
   green, gold, slate, and magenta petals; brick, orange, purple, and dark green triples.
5. Core: red `#b3242c`, a little brighter in the middle (`#cf2f38`).
6. Set names in black `#111111`; overlap words and the core name in white.
7. Outside notes in gray `#77767e`.
8. Title in white; credit in gray (the original: a name in gray and a wordmark in blue).

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#313038` | ground (`#2a2931` to `#3f3e46`) | 68% | nothing |
| `#c1c418` | top circle | 5% | set 1 |
| `#689dbc` | left circle | 5% | set 4 |
| `#b86f80` | right circle | 5% | set 2 |
| `#9c9c9e` | bottom circle | 5% | set 3 |
| mixed tones | petals and triples | 10% | which sets are present there |
| `#b3242c` | core | 2% | all four sets: the answer |
| `#ffffff` | title, overlap words, core name | 1% | nothing |

Categorical: four mid-tone hues of similar lightness (55 to 65%), so black type sits on
every one of them and white type sits on every overlap. Red is the single accent and appears
nowhere else. Gray is used as a real category here (the fourth set), not as "other".

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Title | a question, 5 to 6 words, one line | title | condensed gothic (League Gothic, Oswald 700 / Avenir Next Condensed) | 700 | 66 | 0 | - | center | `#ffffff` | 50%, 5% |
| Set name (x4) | 1 word, 4 to 12 letters | caps | condensed gothic | 700 | 54 | 0 | - | center | `#111111` | 50.4%, 22.6%; 25.5%, 49.4%; 75.4%, 49.4%; 50.4%, 76.1% |
| Pair word (x4) | 1 word, 6 to 14 letters | lower | humanist sans (Open Sans, Source Sans 3 / Helvetica Neue through `@neo-grotesque`) | 400 | 22 | 0 | - | center | `#ffffff` | 39.2% and 61.3% W; 35.4% and 64.6% H |
| Triple word (x4) | 1 word, 6 to 9 letters | lower | humanist sans | 400 | 18 | 0 | - | center | `#ffffff` | 50.1%, 40.1%; 38.8%, 50.1%; 61.6%, 50.1%; 50.1%, 59.8% |
| Core name | 2 words on 2 lines | caps | condensed gothic | 700 | 34 | 0 | 37 px pitch | center | `#ffffff` | 50.2%, 48.4% and 52% |
| Note, top and bottom | 3 to 4 words with commas, one line | lower | humanist sans | 400 | 21 | 0 | - | center | `#77767e` | 50.2%, 12.7% and 87.2% |
| Note, left and right | 4 words on 4 lines | lower | humanist sans | 400 | 21 | 0 | 105% | center | `#77767e` | 10.9% and 89.5% W, 50% H |
| Credit | 2 to 3 words (the original: two right-aligned lines) | title | condensed gothic | 600 | 26 | 0 | - | right | `#77767e` | 94.6%, 95.5% |

Two families: a condensed gothic for everything that names (title, sets, core) and a
humanist sans in lower case for everything that comments. Sizes 66 : 54 : 34 : 22 : 18,
about 3 : 2.5 : 1.5 : 1. The original's gothic is narrower and blacker than Avenir Next
Condensed; the wide gap between naming and commenting type matters more than the face.

## 8. Charts and diagrams

* **C-venn**: `scripts/venn.js` with 4 sets, box `18,16.5,64.4,67.5` (a 695 px square on the
  recipe canvas), `overlap=0.7` (circle centers 0.64 of the radius from the middle; the
  script then logs radius 212 px and center 542, 518), `blend=multiply`, `opacity=0.94`,
  `colors=` in the script's order top, right, bottom, left. `labels=' | | | '` gives four
  empty names, so the script draws the circles only and all type is set with `text add`
  (three colors and two families are needed; a script has one of each).
* No axis, no legend, no values. Areas mean nothing.
* **Core patch**: the script cannot color one region, so the recipe covers the middle with a
  `shape path` of four arcs. With center (cx, cy), radius R, and c = 0.31 x R at
  `overlap=0.7` (0.25 x R at 0.6, 0.18 x R at 0.5), the path is
  `M cx-c cy-c A R R 0 0 1 cx+c cy-c A R R 0 0 1 cx+c cy+c A R R 0 0 1 cx-c cy+c A R R 0 0 1 cx-c cy-c Z`.
* **Simplified:** the original gives each of the thirteen regions a hand-picked color; the
  script mixes the four fills by multiplication, so petals come out close (green, brown
  gold, slate, mauve) and the four triples come out dark instead of colored. Original region
  colors, for a build with per-region fills:

| Region | Hex |
| --- | --- |
| top, left, right, bottom alone | `#c1c418`, `#689dbc`, `#b86f80`, `#9c9c9e` |
| top with left; top with right | `#7d9c3c`; `#bfa64a` |
| left with bottom; right with bottom | `#50647d`; `#ab577b` |
| triples: without bottom; without right; without left; without top | `#b54f2e`; `#3a4a2c`; `#bc7d38`; `#5c3274` |
| core (all four) | `#af252c` |

## 9. Icons and illustration

None. The diagram is the picture. The original's credit carries a small publisher wordmark
in blue: place your own `logo.png` there at about 110 px wide if you have one, or leave the
gray line.

## 10. Effects and finish

* Ground vignette: a layer filled with a radial gradient from `#3f3e46` at 46% W, 30% H to
  `#2a2931` at the far corner (`fill gradient --kind radial --dither`).
* Grain: `filter grain --as-layer --amount 12 --size 1.2` over the ground only (it is added
  before the diagram). The original's texture is a fine woven pattern; grain is the stand-in.
* Core glow: `fx overlay --style radial` from `#cf2f38` to `#a81f27` on the core patch.
* Circles: flat, no outline, no shadow. Translucency is the only effect in the diagram.

## 11. Hierarchy and eye path

1. The red core and its two white words: the only saturated warm color, dead center.
2. The four black set names: the largest type inside the diagram, one per petal direction.
3. The title: white on charcoal, larger but far from the color.
4. Triple words around the core (what one missing set costs), then the pair words.
5. The gray notes outside; the credit.

Title 66, set names 54, core 34, overlap words 22 and 18: the names are two and a half
times the comments, so the diagram still reads from across a room.

## 12. Build recipe

Nothing to supply. Canvas 1080 x 1030. Placeholder content, no data.

```bash
pixelox new -o venn.pxo --size 1080x1030 --fill '#2a2931'
pixelox layer add venn.pxo --in-place --name Glow
pixelox fill gradient venn.pxo --in-place --layer Glow --kind radial --from 46%,30% --to 100%,100% --stops '#3f3e46@0,#2a2931@1' --dither
pixelox filter grain venn.pxo --in-place --as-layer --name Grain --amount 12 --size 1.2
pixelox text add venn.pxo --in-place --name Title --content "What Makes a Talk Worth Hearing?" --font @condensed --weight 700 --size 66 --color '#ffffff' --at 50%,5% --anchor c --anchor-to ink
pixelox shape line venn.pxo --in-place --name RuleTop --from 64,94 --to 1022,94 --stroke '#5a5962' --stroke-width 2
pixelox shape line venn.pxo --in-place --name RuleBottom --from 64,946 --to 1022,946 --stroke '#5a5962' --stroke-width 2
pixelox render js scripts/venn.js -o circles.png --size 1080x1030 --param box=18,16.5,64.4,67.5 --param labels=' | | | ' --param overlap=0.7 --param colors='#c1c418,#b86f80,#9c9c9e,#689dbc' --param blend=multiply --param opacity=0.94
pixelox comp place venn.pxo circles.png --in-place --as Venn --at 0,0
pixelox shape path venn.pxo --in-place --name Core --d 'M 476.2 451.8 A 212 212 0 0 1 607.8 451.8 A 212 212 0 0 1 607.8 583.4 A 212 212 0 0 1 476.2 583.4 A 212 212 0 0 1 476.2 451.8 Z' --fill '#b3242c'
pixelox fx overlay venn.pxo --in-place --layer Core --kind gradient --style radial --stops '#cf2f38@0,#a81f27@1'
pixelox text add venn.pxo --in-place --name SetTop --content "IDEA" --font @condensed --weight 700 --size 54 --color '#111111' --at 544,233 --anchor c --anchor-to ink
pixelox text add venn.pxo --in-place --name SetLeft --content "SLIDES" --font @condensed --weight 700 --size 54 --color '#111111' --at 275,511 --anchor c --anchor-to ink
pixelox text add venn.pxo --in-place --name SetRight --content "VOICE" --font @condensed --weight 700 --size 54 --color '#111111' --at 814,511 --anchor c --anchor-to ink
pixelox text add venn.pxo --in-place --name SetBottom --content "REHEARSAL" --font @condensed --weight 700 --size 54 --color '#111111' --at 544,786 --anchor c --anchor-to ink
pixelox text add venn.pxo --in-place --name PairTL --content "lecture" --font @neo-grotesque --size 22 --color '#ffffff' --at 423,365 --anchor c
pixelox text add venn.pxo --in-place --name PairTR --content "speech" --font @neo-grotesque --size 22 --color '#ffffff' --at 662,365 --anchor c
pixelox text add venn.pxo --in-place --name PairBL --content "polished" --font @neo-grotesque --size 22 --color '#ffffff' --at 404,666 --anchor c
pixelox text add venn.pxo --in-place --name PairBR --content "smooth" --font @neo-grotesque --size 22 --color '#ffffff' --at 662,666 --anchor c
pixelox text add venn.pxo --in-place --name TriTop --content "rambling" --font @neo-grotesque --size 18 --color '#ffffff' --at 542,413 --anchor c
pixelox text add venn.pxo --in-place --name TriLeft --content "flat" --font @neo-grotesque --size 18 --color '#ffffff' --at 421,517 --anchor c
pixelox text add venn.pxo --in-place --name TriRight --content "abstract" --font @neo-grotesque --size 18 --color '#ffffff' --at 664,517 --anchor c
pixelox text add venn.pxo --in-place --name TriBottom --content "empty" --font @neo-grotesque --size 18 --color '#ffffff' --at 542,618 --anchor c
pixelox text add venn.pxo --in-place --name Core1 --content "GREAT" --font @condensed --weight 700 --size 34 --color '#ffffff' --at 542,499 --anchor c --anchor-to ink
pixelox text add venn.pxo --in-place --name Core2 --content "TALK" --font @condensed --weight 700 --size 34 --color '#ffffff' --at 542,536 --anchor c --anchor-to ink
pixelox text add venn.pxo --in-place --name NoteTop --content "clear, new, useful" --font @neo-grotesque --size 21 --color '#77767e' --at 542,131 --anchor c
pixelox text add venn.pxo --in-place --name NoteBottom --content "timing, run-through, feedback, backup" --font @neo-grotesque --size 21 --color '#77767e' --at 542,898 --anchor c
pixelox text add venn.pxo --in-place --name NoteLeft --content $'big type\nfew words\none chart\ncontrast' --font @neo-grotesque --size 21 --leading 105% --align center --color '#77767e' --at 118,514 --anchor c
pixelox text add venn.pxo --in-place --name NoteRight --content $'pace\npauses\nvolume\neye contact' --font @neo-grotesque --size 21 --leading 105% --align center --color '#77767e' --at 967,511 --anchor c
pixelox text add venn.pxo --in-place --name Credit --content "Example content" --font @condensed --weight 600 --size 26 --color '#77767e' --at 1022,984 --anchor r
pixelox export venn.pxo -o venn.png
```

Reuse:

* Run `lab/anchors.py scripts/venn.js` with the same `--size` and `--param` values: `set1`
  to `set4` are the circle centers (top, right, bottom, left), `center` is the middle, and
  `radius` is R. Set names go 0.65 x R outside each set's center, away from the middle. Pair
  words go on the diagonals, 0.9 x R from the middle (0.6 x R right or left and 0.7 x R up
  or down). Triple words go 0.5 x R above and below the middle and 0.57 x R left and right.
* Which word goes where: PairTL is top with left; PairTR top with right; PairBL left with
  bottom; PairBR right with bottom. TriTop is everything except the bottom set; TriLeft
  everything except the right set; TriRight except the left; TriBottom except the top.
* For a blacker, narrower look closer to the original, set the title, the set names, and
  the core in `@condensed-heavy` at 60, 50, and 31 px.
* A long set name (over 9 letters) belongs at the top or bottom, where the petal is wide.

## 13. Variations and pairings

* **Three sets:** `labels=' | | '` and `overlap=0.5` give the classic triangle with three
  pair words and one core; drop the triple words. Two sets: one lens, no core patch.
* **Light ground:** ground `#f4f1ea`, the same four fills at `opacity=0.8`, names in
  `#1f2933`, overlap words in white only where the overlap is dark enough; core `#c62828`.
* **Brand hues:** four tints of one hue do not work (the overlaps turn to mud); pick four
  hues at least 60 degrees apart with similar lightness, plus one accent for the core.
* **As a poster:** 4:5 (1080 x 1350) with the title on two lines and the outside notes
  expanded to short sentences above and below; move the left and right notes under the
  bottom hairline as two columns.
* Relatives: i048 and i030 for other overlap and comparison layouts. Pairs with C-table when
  the reader needs the full list of combinations, and with C-source-line for the credit.

## 14. Pitfalls

* **Combinations the picture cannot show.** Four circles in a cross have 13 regions, but 4
  sets have 15 combinations: "top and bottom only" and "left and right only" have no region.
  Check your content against the 13: if one of your claims is about two opposite sets, put
  those sets next to each other, or the diagram says something you did not mean. This is the
  data-integrity rule for every Venn: each word must be true of exactly the sets its region
  lies in.
* **Reading sizes.** The petals and the core are whatever the geometry leaves; they are not
  shares. Never print percentages in a Venn drawn this way; for real set sizes use C-bars.
* **White words on a light overlap.** At `blend=screen` or with pale fills the overlaps get
  lighter than the circles and white lower-case words vanish. Keep `blend=multiply`, fills
  between 55 and 65% lightness, and check each word against its region.
* **Names in the overlaps.** Set names must sit in the part of each circle that belongs to
  no other set (0.62 to 0.72 of the radius out from the circle's center); a name that
  touches a petal reads as that petal's label.
* **A core that is not centered.** The core patch is computed from the script's center and
  radius. If you change the box or `overlap=` and leave the path alone, the red drifts off
  the middle. Read the anchors again and rebuild the path.
* **Sentences in the regions.** One word per region, two for the core. A phrase of three
  words no longer fits a triple region at any readable size.
