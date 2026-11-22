---
id: p123
title: Full-figure list cover, pale ground
genre: [magazine-cover, editorial, fashion]
mood: [elegant, bold, warm]
style: [editorial, photo-composite]
format: portrait-3x4
archetype: A-cover-masthead
palette: ['#f1f0ec', '#f22929', '#ce8f44', '#7e4a2c', '#d4d2ce', '#111111']
techniques: [T-frame-border, T-text-behind-subject, T-rules]
needs: subject-cutout
---

# p123: Full-figure list cover, pale ground

## 1. Concept

A news-magazine cover for an annual list issue: a full-length figure in a golden gown
stands on a pale, almost white ground inside a red frame; the serif masthead is printed
in light gray behind her, a tall condensed list title hangs from the left edge in gold
and black, and a two-line strip of names runs across the top. It works because the
picture is nearly empty: one warm figure, one cold white field, and type that is narrow
enough to stand beside the figure instead of on top of it.

## 2. Format and grid

* Portrait 3:4. Red frame 4.4% W wide (48 px at 1080 wide) on all four sides; no keyline,
  because the pale picture already separates from the red. Picture window
  `4.4%,3.3%,91.2% x 93.4%`.
* Two columns. A narrow text column hangs from x 6.6% W and is never wider than 30% W.
  The figure owns the rest: her body axis is at 57% W and she spans 18 to 96% W.
* Vertical zones: name strip 4.6 to 7.9% H; masthead 9.3 to 23.8% H; figure 24 to 94% H;
  list title 31 to 58.5% H; featured name block 59.5 to 73.6% H; ground strip 80 to 96.6% H.
* The text column starts where the masthead ends plus 7% H, and stops 7% H above the
  ground strip: it floats in the middle of the left edge with white above and below.

## 3. Layer stack

1. Ground: off-white `#f1f0ec` (a white cloth backdrop under daylight), a touch grayer
   toward the lower left.
2. Ground strip: dry grass and red-brown earth across the bottom, 80 to 96.6% H, with an
   uneven, soft upper edge.
3. Masthead: 4 serif capitals in light gray `#d4d2ce`, x 13.5 to 86% W, cap top 9.3% H,
   cap height 14.5% H. Tone on tone: it is read as part of the backdrop.
4. Figure: a woman standing full length, weight on one leg, one hand on her hip, the other
   arm raised out to the left (hand at 20% W, 31% H); a draped satin gown in golden ochre
   with a slit and a train that runs off the lower right corner; head at 52 to 60% W,
   24.5 to 33% H. A long braid swings up from her head in a thin black arc that crosses
   the first two masthead letters and peaks at 22% W, 5% H.
5. Name strip: two centered lines of condensed caps, baselines 5.8% and 7.7% H, spanning
   13 to 87% W. Names are bold; each is followed by a smaller, lighter "BY" + name.
6. List title, left at 6.6% W: two gold lines (cap tops 31.1% and 36.9% H), then three
   black lines (42.3%, 48%, 53.6% H). One size: cap height 4.9% H, line pitch 5.65% H.
7. Rule: black, 2 px, 6.6 to 12.5% W at 59.5% H.
8. Kicker: small tracked caps at 61.6% H. Featured name: two black condensed lines, cap
   tops 63.2% and 67% H, cap height 3.2% H.
9. Rule at 71.3% H, same length. Byline: small tracked caps at 73.1% H.
10. Red frame `#f22929`, above everything.
11. Fine print in the frame: issue note at 4.5% W and date range ending at 95.5% W, both
    at 2.4% H; web address ending at 95.5% W, 97.7% H.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#f1f0ec` | backdrop | 45% |
| `#f22929` | frame | 15% |
| `#ce8f44` | gown (lit folds `#e6b145`, shadow folds `#9a6829`) | 15% |
| `#7e4a2c` | skin, earth | 10% |
| `#d4d2ce` | masthead | 8% |
| `#111111` | list title, names, braid | 5% |
| `#e2a93b` | first two lines of the list title | 2% |

Warm ochre and brown on cool white, boxed in red. The gold of the type is sampled from
the gown, which ties the text column to the figure across the white gap.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Masthead | `[MASTHEAD]`, 4 letters | caps | sharp transitional serif, display cut (Playfair Display, Libre Caslon Display / Times New Roman) | 700 | 14.5% | 20 | - | center | `#d4d2ce` | 50%, cap top 9.3% |
| Strip names | 5 names of 9 to 17 letters, then "+ NN MORE" | caps | condensed gothic (Oswald 600, League Gothic / DIN Condensed) | 700 | 1.2% | 60 | - | center, 2 lines | `#111111` | 50%, baselines 5.8% and 7.7% |
| Strip credits | "BY" + name after each strip name | caps | condensed sans (Oswald 400 / Avenir Next Condensed) | 500 | 0.8% | 110 | - | inline | `#111111` | same baselines |
| List title, gold | "THE" + a number, 2 lines | caps | ultra-condensed gothic, regular weight (League Gothic, Oswald 400 / DIN Condensed) | 400 | 4.9% | 0 | 84% | left | `#e2a93b` | 6.6%, cap tops 31.1% and 36.9% |
| List title, black | 3 lines, one word each, 4 to 11 letters | caps | same | 400 | 4.9% | 0 | 84% | left | `#111111` | 6.6%, cap tops 42.3%, 48%, 53.6% |
| Kicker | 2-word descriptor of the person | caps | condensed sans | 500 | 1.2% | 110 | - | left | `#111111` | 6.6%, 61.6% |
| Featured name | `[PERSON NAME]`, 2 lines, 5 to 10 letters each | caps | ultra-condensed gothic | 400 | 3.2% | 0 | 86% | left | `#111111` | 6.6%, cap tops 63.2% and 67% |
| Byline | "BY" + writer name | caps | condensed sans | 500 | 1.1% | 110 | - | left | `#111111` | 6.6%, 73.1% |
| Fine print (x3) | issue note; date range; web address | caps, lower | neo-grotesque (Inter / Helvetica Neue) | 700 | 0.6% | 140 | - | left, right, right | `#1a0505` | in the frame |

The widest line of the list title (11 letters) is 29.6% W: a 9-letter line in this face
is only 3.7 cap heights wide. That narrowness is the layout: a normal-width sans at the
same cap height would run into the figure.

## 6. Imagery

One fashion photograph taken outdoors against a hung white cloth: hard, high sun from
the upper left, so the gown has bright folds and deep creases and the skin is glossy.
The pose makes a wide triangle (raised arm, hip, trailing train) and the braid adds a
drawn line above it. Natural color; no grade. The figure is in front of the masthead
(**T-text-behind-subject**): only the braid and the top of the head touch the letters, so
the masthead stays fully readable.

For a build: a full-length or three-quarter cutout with a strong outline, one color
family in the clothing, and something that reaches up or out (an arm, hair, a prop) to
cross the masthead. The ground strip keeps the figure from floating; a soft gradient in
an earth or floor tone is enough.

## 7. Effects and textures

* **T-frame-border**: 48 px red frame as an even-odd path, drawn last; no keyline.
* **T-text-behind-subject**: gray masthead between the ground and the cutout.
* **T-rules**: two short 2 px rules (5.9% W long) bracket the kicker and the featured
  name.
* Floor: a linear gradient from transparent at 74% H to sand `#b9976d` at 92% H.
* Text column: each stacked block is one `text add` placed by the top-left corner of its
  visible letters (`--at 6.6%,Y --anchor-to ink`), so every block and both rules share one
  exact left edge.
* Nothing else. No shadow, no grain, no gradient on type.

## 8. Hierarchy and eye path

1. The gold gown against white (the only saturated area inside the frame).
2. Along the raised arm to the gold top of the list title, then down its five lines.
3. Rule, kicker, featured name, byline.
4. Up to the name strip; the gray masthead is noticed last.

Cap heights: masthead 14.5 (gray) : list title 4.9 : featured name 3.2 : strip names and
kicker 1.2 : credits 0.8 : fine print 0.6. In black type the ratio is about 4 : 2.7 : 1.

## 9. Build recipe

`subject.png` is your image here: a cutout figure. The strip is built from separate layers
because the two sizes alternate inside one line: add them once with `--json`, read each
layer's `bounds`, and set the next x to the previous right edge plus 10 px (name to
credit) or 18 px (credit to the next name); start each line at
`(canvas width - line width) / 2`.

```bash
pixelox new -o cover.pxo --size 1080x1440 --fill '#f1f0ec'
pixelox layer add cover.pxo --in-place --name Floor
pixelox fill gradient cover.pxo --in-place --layer Floor --from 0,74% --to 0,92% --stops '#b9976d00@0,#b9976dff@1'
pixelox text add cover.pxo --in-place --name Masthead --content "LENS" --font "Times New Roman" --weight 700 --fit-width 783 --tracking 20 --color '#d4d2ce' --at 50%,9.3% --anchor t --anchor-to ink
pixelox comp place cover.pxo subject.png --in-place --as Subject --size x1160 --at 67%,94.4% --anchor b
pixelox text add cover.pxo --in-place --name N1 --content "ADA OKAFOR" --font "DIN Condensed" --weight 700 --size 1.8% --tracking 60 --color '#111111' --at 186,5.4% --anchor l
pixelox text add cover.pxo --in-place --name B1 --content "BY LEO MARCH" --font @condensed --weight 500 --size 1.15% --tracking 110 --color '#111111' --at 312,5.4% --anchor l
pixelox text add cover.pxo --in-place --name N2 --content "MEI TANAKA" --font "DIN Condensed" --weight 700 --size 1.8% --tracking 60 --color '#111111' --at 438,5.4% --anchor l
pixelox text add cover.pxo --in-place --name B2 --content "BY SAM ORTIZ" --font @condensed --weight 500 --size 1.15% --tracking 110 --color '#111111' --at 557,5.4% --anchor l
pixelox text add cover.pxo --in-place --name N3 --content "IDRIS COLE" --font "DIN Condensed" --weight 700 --size 1.8% --tracking 60 --color '#111111' --at 677,5.4% --anchor l
pixelox text add cover.pxo --in-place --name B3 --content "BY NORA VALE" --font @condensed --weight 500 --size 1.15% --tracking 110 --color '#111111' --at 789,5.4% --anchor l
pixelox text add cover.pxo --in-place --name N4 --content "PRIYA ANAND-REYES" --font "DIN Condensed" --weight 700 --size 1.8% --tracking 60 --color '#111111' --at 184,7.3% --anchor l
pixelox text add cover.pxo --in-place --name B4 --content "BY TOM ELLERY" --font @condensed --weight 500 --size 1.15% --tracking 110 --color '#111111' --at 384,7.3% --anchor l
pixelox text add cover.pxo --in-place --name N5 --content "JONAS WEBER" --font "DIN Condensed" --weight 700 --size 1.8% --tracking 60 --color '#111111' --at 514,7.3% --anchor l
pixelox text add cover.pxo --in-place --name B5 --content "BY ROSA ITURBE" --font @condensed --weight 500 --size 1.15% --tracking 110 --color '#111111' --at 655,7.3% --anchor l
pixelox text add cover.pxo --in-place --name More --content "+ 94 MORE" --font "DIN Condensed" --weight 700 --size 1.8% --tracking 60 --color '#111111' --at 793,7.3% --anchor l
pixelox text add cover.pxo --in-place --name ListGold --content $'THE\n50' --font "DIN Condensed" --weight 700 --size 6.7% --leading 84% --color '#e2a93b' --at 6.6%,31.1% --anchor-to ink
pixelox text add cover.pxo --in-place --name ListBlack --content $'MOST\nINVENTIVE\nMAKERS' --font "DIN Condensed" --weight 700 --size 6.7% --leading 84% --color '#111111' --at 6.6%,42.3% --anchor-to ink
pixelox shape line cover.pxo --in-place --name Rule1 --from 6.6%,59.5% --to 12.5%,59.5% --stroke '#111111' --stroke-width 2
pixelox text add cover.pxo --in-place --name Kicker --content "TEXTILE PIONEER" --font @condensed --weight 500 --size 1.65% --tracking 110 --color '#111111' --at 6.6%,61.6% --anchor l
pixelox text add cover.pxo --in-place --name Name --content $'AMARA DIALLO\nBENSON' --font "DIN Condensed" --weight 700 --size 4.4% --leading 86% --color '#111111' --at 6.6%,63.2% --anchor-to ink
pixelox shape line cover.pxo --in-place --name Rule2 --from 6.6%,71.3% --to 12.5%,71.3% --stroke '#111111' --stroke-width 2
pixelox text add cover.pxo --in-place --name Byline --content "BY KOFI A. MENSAH" --font @condensed --weight 500 --size 1.5% --tracking 110 --color '#111111' --at 6.6%,73.1% --anchor l
pixelox shape path cover.pxo --in-place --name Frame --d 'M 0 0 H 1080 V 1440 H 0 Z M 48 48 H 1032 V 1392 H 48 Z' --fill-rule evenodd --fill '#f22929'
pixelox text add cover.pxo --in-place --name IssueNote --content "SPECIAL ISSUE" --font @neo-grotesque --weight 700 --size 0.85% --tracking 140 --color '#1a0505' --at 4.5%,2.4% --anchor l
pixelox text add cover.pxo --in-place --name IssueDate --content "OCT. 6 / OCT. 13, 2031" --font @neo-grotesque --weight 700 --size 0.85% --tracking 140 --color '#1a0505' --at 95.5%,2.4% --anchor r
pixelox text add cover.pxo --in-place --name Site --content "example.com" --font @neo-grotesque --weight 700 --size 0.8% --tracking 140 --color '#1a0505' --at 95.5%,97.7% --anchor r
pixelox export cover.pxo -o cover.png
```

Change first: the five list-title lines (keep every line under 12 letters, one word per
line), then the gold (sample it from your subject's clothing), then the subject's size
and x position until its outline passes 2% W to the right of the longest text line.
DIN Condensed is bold only; with League Gothic or Oswald 400 registered the title gets
the lighter color of the original.

## 10. Variations and pairings

* Same frame, other pictures: [p089](p089-red-frame-annual-list-cover.md) (documentary
  photo, ghost masthead, story block in a corner) and
  [p099](p099-stencil-portrait-news-cover.md) (stencil portrait, opaque masthead, no cover
  lines).
* Dark version: ground `#141414`, masthead `#2a2a2a`, list title white with the first
  two lines in the accent, frame unchanged.
* Without the frame it becomes a fashion or awards poster: keep the gray wordmark behind
  the figure and the condensed column at the left.
* Team, alumni, or "NN under NN" lists: the strip holds the other honorees, the column
  holds the list name and the featured person.
* Landscape banner: figure at 70% W, the title column at 6% W with cap height 9% H, the
  masthead dropped.
* Pairs with T-cutout-drop-shadow (soft studio shadow under the feet) and T-color-grade
  to pull a cool backdrop toward the same warm white.

## 11. Pitfalls

* A normal-width font in the title column. At cap height 4.9% H it will be 45% W wide
  and collide with the figure; use an ultra-condensed gothic or drop the cap height.
* Coloring the whole title gold. Gold on white has low contrast; it carries only the two
  shortest lines, and black does the reading.
* A masthead dark enough to compete (darker than `#c4c2be` on this ground). It should be
  12 to 15% darker than the backdrop, no more.
* The figure covering the masthead's first or last letter. Only thin things (hair, a
  hand, a prop) cross it; the head sits just under it.
* Strip segments placed by eye. Mixed sizes in one line need measured positions (read
  `bounds`), or the gaps come out uneven and the line stops looking typeset.
