---
id: i048
title: Three interlocked circles around a numbered hub
kind: [comparison, list]
topic: [business, generic]
mood: [clean, corporate, cool, calm]
style: [flat, line-icon]
format: square
structure: S-overlap-diagram
data: [comparison, text-only]
charts: [C-venn, C-icon-badge]
palette: ['#eaeaea', '#42aec5', '#238af1', '#015cd1', '#f7f7f7', '#1b7a96']
needs: nothing
---

# i048: Three interlocked circles around a numbered hub

## 1. Concept

A square "three options" diagram on pale gray. Three very large flat circles (teal on top,
bright blue at the lower right, royal blue at the lower left) overlap like the blades of a
pinwheel: each one lies over its neighbor on one side and under its neighbor on the other. A
white disc with a soft shadow covers the middle and states the count: a big 3, a one-word
caption, a label, a short gradient rule. Every circle carries a caps title, a short
paragraph, a white line icon, and its number (01, 02, 03) set in the overlap next to the
hub. The message is "three choices of equal standing". It works because the interlock makes
ranking impossible (no circle is simply on top), the three blues keep the choices in one
family, and the hub gives the eye a place to start.

## 2. Data story

No numbers except the count and the option numbers. Text only:

* 3 parallel options. Each: a title of one caps word, a note of 13 to 22 words on 3 to 5
  lines, an icon, and a two-digit number.
* 1 hub: the count as a single digit, a one-word caption, and a one-word label.
* The overlaps hold no shared content: they only carry the numbers. This is not a true Venn
  diagram.

Placeholder content in the recipe (invented, three price plans): BASIC, PLUS, PRO, each with
a note of 13 to 22 words; hub 3, PLANS, PRICING.

## 3. Information architecture

1. **Hub**, center: how many options there are and what they are options for.
2. **Option 01**, top circle: title and note left of the circle's middle, icon upper right,
   the number at 9 o'clock from the hub.
3. **Option 02**, lower right circle: title and note right of the hub, icon below them, the
   number at 1 o'clock from the hub.
4. **Option 03**, lower left circle: icon at the far left, title and note below the hub's
   level, the number at 5 o'clock from the hub.

Each number sits on the patch of overlap that has its own circle's color, right beside the
hub, so the hub's "3" unfolds into 01, 02, 03 clockwise. Inside a circle the text goes
where the circle is widest and unobstructed; the icon takes the outer bulge.

## 4. Format and grid

* Square. Recipe canvas 1080 x 1080. The figure is centered on 50% W, 50% H and fills 86% W.
* Three circles of radius 28.7% W (310 px). Their centers lie 16.6% W (179 px) from the
  middle at 12, 4, and 8 o'clock: (50%, 33.4%), (64.4%, 58.3%), (35.6%, 58.3%). With this
  ratio (radius = 1.73 x the center distance) every pair of circles crosses exactly at the
  third circle's center.
* Hub: radius 12.7% W (137 px).
* Top circle: title centered at 40% W, 20.7% H; note `26.2%,22.8%,27.8% x 8%`; icon at 62.7%
  W, 19.4% H.
* Right circle: title at 79.3% W, 51.4% H; note `67.6%,53.6%,23.4% x 8%`; icon at 71.3% W,
  75.1% H.
* Left circle: icon at 18.3% W, 54.2% H; title at 32.3% W, 66.8% H; note `20.5%,69%,23.6% x
  8%`.
* Numbers at 31.9% W / 47.6% H, 61% W / 35.5% H, and 56.3% W / 67.5% H.
* Free ground: 7% W left and right, 4.7% H on top, 13% H at the bottom (the figure sits high;
  use the bottom band for a source line or leave it).

## 5. Layer stack

1. Ground: pale gray `#eaeaea`, flat.
2. Three opaque circles, stacked top circle first: teal `#42aec5` (top), bright blue
   `#238af1` (lower right), royal blue `#015cd1` (lower left).
3. A teal patch: the lens where the top and lower left circles overlap, redrawn in teal.
   Together with the stacking this gives the pinwheel: bright blue over teal, royal over
   bright blue, teal over royal.
4. Hub: off-white disc `#f7f7f7` with a soft navy shadow falling to the lower right.
5. Hub type: the digit in deep teal `#1b7a96`, two gray lines `#6b6b6b`, and a 80 x 4 px
   rule with a teal-to-royal gradient.
6. Three numbers in white, bold, in the overlaps.
7. Three white line icons, three white titles, three white notes.

## 6. Palette

| Hex | Role | Area | Encodes |
| --- | --- | --- | --- |
| `#eaeaea` | ground | 36% | nothing |
| `#42aec5` | top circle and its overlap patch | 20% | option 01 |
| `#238af1` | lower right circle | 19% | option 02 |
| `#015cd1` | lower left circle | 20% | option 03 |
| `#f7f7f7` | hub; all type and icons on color are `#ffffff` | 5% | nothing |
| `#1b7a96` | hub digit; gray `#6b6b6b` for the hub's words | under 1% | emphasis: the count |

Categorical within one family: three blues from teal to royal, close enough to read as one
set, different enough to tell apart. No warm color. The hub rule's gradient runs through the
same three hues: a tiny legend of the whole figure.

## 7. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Size px | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Hub digit | 1 digit | - | geometric sans (Montserrat, Poppins / Avenir Next) | 700 | 94 | 0 | - | center | `#1b7a96` | 50% W, 46.3% H |
| Hub caption | 1 word | caps | geometric sans | 500 | 15 (the original: 12) | 60 | - | center | `#6b6b6b` | 50% W, 51.9% H |
| Hub label | 1 word, up to 11 letters | caps | geometric sans | 700 | 27 | 20 | - | center | `#6b6b6b` | 50% W, 54.9% H |
| Option number (x3) | 01 to 03 | - | geometric sans | 700 | 49 | 0 | - | center | `#ffffff` | in the overlaps beside the hub |
| Option title (x3) | 1 word | caps | geometric sans | 700 | 28 | 0 | - | center | `#ffffff` | above each note, centered on it |
| Option note (x3) | 13 to 22 words on 3 lines (the original: 5 lines) | sentence | geometric sans | 500 | 17 (the original: 12) | 0 | 22 px | justify | `#ffffff` | boxes of 253 to 300 px, 23 px under the title's center |

One family, two weights. Sizes 94 : 49 : 28 : 17, about 5.5 : 2.9 : 1.6 : 1. The hub digit
is the only large type; the option numbers echo it at half size.

## 8. Charts and diagrams

* **C-venn**, used for its three-circle layout, not as a Venn: `scripts/venn.js` with three
  blank names, `opacity=1 blend=normal` (opaque, no color mixing), `overlap=0.636`,
  `box=6.95,4.7,86.1,82.3`, `colors=` teal, bright blue, royal. The script logs the circle
  radius (310) and the centers: set1 (540,361), set2 (695,629), set3 (385,629), and the
  middle (540,540). It stacks set1 at the bottom and set3 on top.
* **The interlock** is one native `shape path`: the lens shared by set1 and set3, filled
  with set1's color. Because `overlap=0.636` makes each pair of circles cross at the third
  circle's center, the lens runs from set2's center (695,629) to the mirror point set1 +
  set3 - set2 = (230,361): two arcs of radius 310 with large-arc flag 0 and sweep flag 1,
  out and back. A 2 px stroke in the same color hides the seam.
* The hub hides the small area where all three circles meet, so no fourth patch is needed.
  Keep the hub radius at 0.44 x the circle radius or more.
* No axis, no values, no legend. Circle size encodes nothing.
* Simplified: nothing structural; the original's hub has a slightly glossy edge.

## 9. Icons and illustration

* Three line icons, 112 px, 4 px line, white: `search` (top circle), `settings` (lower
  right), `mountain-snow` (lower left). The original: a magnifier, a cog with a ring inside,
  two mountain peaks with a flag on the taller one, all in a thin even outline.
* Each icon sits in its circle's outer bulge, as far from the hub as the notes are, on the
  side the text leaves free.
* No photos.

## 10. Effects and finish

* Hub shadow: `fx drop-shadow --offset 6,10 --blur 26 --color '#0b2a4a' --opacity 40`: it
  lifts the hub off the circles and is the only depth cue.
* Hub rule gradient: `fx overlay --kind gradient --stops '#42aec5@0,#015cd1@1' --angle 0`.
* Everything else flat; no outlines.

## 11. Hierarchy and eye path

1. The white hub and its digit: the lightest shape, in the exact middle.
2. The three numbers around it: 01 at the left, 02 above right, 03 below.
3. From each number outward into its circle: title, note, icon.

The circles are equal in size, type, and icon size. Only the reading habit (top first, then
clockwise) orders them.

## 12. Build recipe

Nothing to supply. Canvas 1080 x 1080. All copy is invented.

```bash
pixelox new -o three.pxo --size 1080x1080 --fill '#eaeaea'
pixelox render js scripts/venn.js -o circles.png --size 1080x1080 --param box=6.95,4.7,86.1,82.3 --param labels=' | | ' --param overlap=0.636 --param opacity=1 --param blend=normal --param colors='#42aec5,#238af1,#015cd1'
pixelox comp place three.pxo circles.png --in-place --as Circles --at 0,0
pixelox shape path three.pxo --in-place --name Tuck --d 'M 695 629 A 310 310 0 0 1 230 361 A 310 310 0 0 1 695 629 Z' --fill '#42aec5' --stroke '#42aec5' --stroke-width 2
pixelox shape ellipse three.pxo --in-place --name Hub --rect 403,403,274x274 --fill '#f7f7f7'
pixelox fx drop-shadow three.pxo --in-place --layer Hub --offset 6,10 --blur 26 --color '#0b2a4a' --opacity 40
pixelox text add three.pxo --in-place --name HubNumber --content "3" --font @geometric-sans --weight 700 --size 94 --color '#1b7a96' --at 540,500 --anchor c --anchor-to ink
pixelox text add three.pxo --in-place --name HubSmall --content "PLANS" --font @geometric-sans --weight 500 --size 15 --tracking 60 --color '#6b6b6b' --at 540,560 --anchor c --anchor-to ink
pixelox text add three.pxo --in-place --name HubLabel --content "PRICING" --font @geometric-sans --weight 700 --size 27 --tracking 20 --color '#6b6b6b' --at 540,593 --anchor c --anchor-to ink
pixelox shape rect three.pxo --in-place --name HubRule --rect 500,626,80x4 --radius 2 --fill '#42aec5'
pixelox fx overlay three.pxo --in-place --layer HubRule --kind gradient --stops '#42aec5@0,#015cd1@1' --angle 0
pixelox text add three.pxo --in-place --name No1 --content "01" --font @geometric-sans --weight 700 --size 49 --color '#ffffff' --at 344,514 --anchor c --anchor-to ink
pixelox text add three.pxo --in-place --name No2 --content "02" --font @geometric-sans --weight 700 --size 49 --color '#ffffff' --at 659,383 --anchor c --anchor-to ink
pixelox text add three.pxo --in-place --name No3 --content "03" --font @geometric-sans --weight 700 --size 49 --color '#ffffff' --at 608,729 --anchor c --anchor-to ink
pixelox shape icon three.pxo search --in-place --name Icon1 --size 112 --at 677,209 --anchor c --color '#ffffff' --stroke-width 4
pixelox text add three.pxo --in-place --name Head1 --content "BASIC" --font @geometric-sans --weight 700 --size 28 --color '#ffffff' --at 433,223 --anchor c --anchor-to ink
pixelox text add three.pxo --in-place --name Body1 --content "One seat and the core tools. Enough to try the product on a real project before anyone else has to learn it." --font @geometric-sans --weight 500 --size 17 --leading 22 --align justify --box 300x90 --color '#ffffff' --at 283,246 --anchor tl
pixelox shape icon three.pxo settings --in-place --name Icon2 --size 112 --at 770,811 --anchor c --color '#ffffff' --stroke-width 4
pixelox text add three.pxo --in-place --name Head2 --content "PLUS" --font @geometric-sans --weight 700 --size 28 --color '#ffffff' --at 856,555 --anchor c --anchor-to ink
pixelox text add three.pxo --in-place --name Body2 --content "Five seats, shared folders, and rules you set once for the whole team." --font @geometric-sans --weight 500 --size 17 --leading 22 --align justify --box 253x90 --color '#ffffff' --at 730,579 --anchor tl
pixelox shape icon three.pxo mountain-snow --in-place --name Icon3 --size 112 --at 198,585 --anchor c --color '#ffffff' --stroke-width 4
pixelox text add three.pxo --in-place --name Head3 --content "PRO" --font @geometric-sans --weight 700 --size 28 --color '#ffffff' --at 349,721 --anchor c --anchor-to ink
pixelox text add three.pxo --in-place --name Body3 --content "Unlimited seats, an audit log, and a person to call when it matters." --font @geometric-sans --weight 500 --size 17 --leading 22 --align justify --box 255x90 --color '#ffffff' --at 221,745 --anchor tl
pixelox export three.pxo -o three.png
```

Reuse: change the hub's three lines and the three title-and-note pairs. Keep
`overlap=0.636`: the patch path depends on it. If you move or resize the figure, read the
new centers with `lab/anchors.py scripts/venn.js --size 1080x1080 --param
box=6.95,4.7,86.1,82.3 --param labels=' | | ' --param overlap=0.636` and rewrite the patch as
`M set2 A r r 0 0 1 (set1 + set3 - set2) A r r 0 0 1 set2 Z`. Text boxes: the top circle
takes 300 px lines, the lower circles 253 px.

## 13. Variations and pairings

* **Plain stack:** drop the patch for a simpler look (the lower left circle then lies on
  top of both others); fine when the options are ranked and the top one is the
  recommendation.
* **A real Venn:** `opacity=0.78 blend=multiply`, names in the circles, `overlaps=` and
  `center=` filled in, no hub: use it when the shared areas mean something (C-venn).
* **Other families:** three greens (`#8fd14f`, `#3fa34d`, `#1b6e3c`) or three warm tones
  (`#ffb238`, `#ff7a3d`, `#e23b5a`); keep the hub digit in the darkest of the three.
* **Four options:** four circles need two patches and a larger hub; prefer C-hub-spokes or a
  2 x 2 C-icon-grid.
* Pairs with C-title-block above (shrink the box to 70% W) and C-comparison-columns on a
  second slide for the point-by-point detail.

## 14. Pitfalls

* **Readers look for meaning in the overlaps.** Overlapping circles say "these share
  something". Here they share nothing: the numbers fill the overlaps so they do not look
  like data. If your options do overlap, show that properly with C-venn; if they do not and
  the audience is literal-minded, use separate cards. This is the data-integrity point.
* **Patch out of register.** With any `overlap=` other than 0.636 the lens corners move and
  the patch shows as a wrong-colored sliver. Recompute from the anchors or drop the patch.
* **Text across an edge.** A note that crosses from one circle into another changes color
  under the words and looks like two options. Keep each note inside the unobstructed part of
  its circle (boxes of 23 to 28% W).
* **Three blues too close.** On a projector teal, azure, and royal can merge. Check that
  neighbors differ by at least 15 points of lightness, or add a 3 px ground-colored gap.
* **Tiny notes.** The original sets notes at 12 px. Use 17 px and three lines.
