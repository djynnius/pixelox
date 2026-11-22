---
id: p017
title: Chrome rings behind a sliced headline
genre: [conference, tech-ad, event-flyer]
mood: [dark, futuristic, bold, cool]
style: [3d-render, type-led, minimal]
format: portrait-a
archetype: A-abstract-title
palette: ['#1e1d20', '#ffffff', '#d2d9e1', '#73787f', '#313338']
techniques: [T-sliced-type, T-fit-width, T-paper-grain]
needs: nothing
---

# p017: Chrome rings behind a sliced headline

## 1. Concept

A talk announcement in charcoal and white only: glossy chrome tube rings loop out of the
top right corner, and a huge two-line headline in heavy, tightly set lowercase-and-capital
grotesque sits over them, cut once by a horizontal slit with the lower strip nudged
sideways. It works because the page has one material (liquid metal) and one voice (fat
white type), and the single glitch cut makes static type look as if it were moving.

## 2. Format and grid

* Portrait A-series (1 : 1.414). Margins 10% W left and right, 7% H top and bottom.
* Center axis for the headline and everything under it; the two corner blocks (studio
  name top left, footer pair at the bottom) hang from the left and right margins.
* Zones: studio block 8 to 19% H; headline 29 to 61% H; sub-line 64 to 71% H; date line
  77 to 81% H; address 84 to 86% H; footer 92 to 94% H.
* The chrome occupies the top right (40 to 100% W, 0 to 38% H) and dips behind the first
  headline line; fainter chrome shapes sit in the top left and bottom right corners.

## 3. Layer stack

1. Ground: charcoal `#1e1d20`.
2. Dim chrome, top left: part of a ring in dark grays at 45% opacity, box
   `-12%,-6%,40% x 30%`.
3. Dim chrome, bottom right: same, box `72%,66%,40% x 28%`.
4. Chrome ring B: a tube ring seen at an angle, outer diameter 65% W, tube thickness 12%
   W, centered at 62% W, 12% H, rotated 55 degrees; mostly hidden behind ring A.
5. Chrome ring A: same ring, outer diameter 70% W, centered at 74% W, 22% H, rotated -38
   degrees so its lower left arc reaches 45% W, 37% H, behind the headline. Surface:
   white broad faces, black reflection bands 1 to 2% W wide along the edges, mid-gray
   `#73787f` transitions.
6. Headline line 1 (4 letters, first letter capital), white, 63% W wide, cap height
   14.5% H, cap top 29.5% H. Cut by a horizontal slit at 68% of its height (y 39.5% H):
   the strip below the cut is shifted 2.4% W to the right, and a 0.5% H gap of ground
   color separates the two parts.
7. Headline line 2 (4 letters and a period), white, same size, cap top 45.5% H, centered;
   the line gap is only 10% of the cap height.
8. Sub-line: two centered lines of light caps, 65.5% and 69% H.
9. Date line: `[DATE] / [TIME]` in square brackets, centered at 79% H, 70% W wide.
10. Address line, small, centered at 84.8% H.
11. Footer: bold call to action at the left margin, web address at the right margin,
    both at 93% H.
12. Studio block: two-line bold name at 10% W, 9 to 13% H; small handle at 17.8% H.
13. Grain: one live grain layer under the type.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#1e1d20` | charcoal ground | 62% |
| `#ffffff` | type, chrome highlights | 15% |
| `#d2d9e1` | chrome light faces (cool white) | 8% |
| `#73787f` | chrome mid-tones | 8% |
| `#313338` | dim chrome in the corners | 7% |

Strictly achromatic with a cool cast. The ground is not pure black: `#1e1d20` leaves room
for the chrome's black bands to read as darker than the page.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Headline | 2 words on 2 lines, 4 letters each, ending in a period | title | heavy neo-grotesque (Inter 800, Archivo 800 / Helvetica Neue Bold) | 700 | 14.5% | -70 | line gap 10% of cap height | center | `#ffffff` | 50%, cap tops 29.5% and 45.5% |
| Sub-line | 2 lines, 5 words, 17 and 13 characters | caps | light neo-grotesque (Inter 300 / Helvetica Neue Light) | 300 | 2.6% | -40 | 100% (pitch 3.5% H) | center | `#ffffff` | 50%, 65.5% and 69% |
| Date line | `[MON DD] / [H-H]`, about 19 characters | caps | neo-grotesque (Inter 500 / Helvetica Neue Medium) | 500 | 3.6% | -40 | - | center | `#ffffff` | 50%, 79% |
| Address | "Will be held in" + street, city | sentence | neo-grotesque (Inter 400 / Helvetica Neue) | 400 | 1.3% | -20 | - | center | `#ffffff` | 50%, 84.8% |
| Call to action | 2 words | caps | same | 700 | 1.2% | -20 | - | left | `#ffffff` | 10%, 93% |
| Web address | one URL | caps | same | 400 | 1.2% | -20 | - | right | `#ffffff` | 90%, 93% |
| Studio name | `[STUDIO NAME]`, 2 lines of 6 to 7 letters | title | same | 700 | 2% | -50 | 95% | left | `#ffffff` | 10%, 9% to 13% |
| Handle | one social handle | lower | same | 400 | 1% | -20 | - | left | `#ffffff` | 10%, 17.8% |

One family in four weights, all tracked tight (negative): the tight setting is what makes
a neutral grotesque look like display type. Headline to date line to sub-line is 5.6 : 1.4 :
1, and the small lines are a tenth of the headline.

## 6. Imagery

No photograph. The picture is a 3D render of polished metal tube rings (two interlocked
loops) lit from above: big white faces, thin black reflection bands, on the charcoal
ground. Here it is built with `render 3d --primitive torus --style shaded` and then turned
into chrome: `adjust levels` stretches the soft shading to full range, and a banded
`adjust gradient-map` (dark, light, dark, light) produces the alternating reflections. The
same render, mapped to dark grays and set to 45% opacity, fills two corners so the forms seem to continue around the
type.

## 7. Effects and textures

* Chrome from a matte render: levels `--in-black 95 --in-white 212`; gradient map stops
  `#060607@0, #3a3d43@0.14, #0c0c0e@0.26, #aeb5bf@0.4, #f4f6f8@0.5, #2a2c31@0.6,
  #0a0a0c@0.7, #c9cfd7@0.86, #ffffff@1`; `filter gaussian` sigma 1.6 to melt the facets.
  More alternations = more liquid; fewer = brushed steel.
* Sliced type (**T-sliced-type** with a level cut): two copies of the same text layer,
  each with a vector mask (`mask add --from-path`), one showing everything above the cut,
  one everything below it; the lower copy is moved 26 px right. `filter glitch`
  (**T-glitch**) is not used here: it shifts thin random bands, and this design needs one
  clean cut with a gap.
* **T-fit-width** for the two headline lines (one width for line 1, line 2 set to the
  same cap height).
* **T-paper-grain** (`filter grain --as-layer --amount 8`), added before the type so the
  letters stay clean.
* **T-sliced-type** in [techniques.md](../techniques.md) is the tested library version of the cut-and-shifted headline; start from it when reusing this idea.

## 8. Hierarchy and eye path

1. The headline: 30% of the page height, pure white, dead center.
2. The chrome ring it overlaps (the brightest non-type shape), which pulls the eye up and
   right, then back down the tube into the headline.
3. The bracketed date line (second largest type), then the sub-line above it.
4. Address, footer pair, studio block.

## 9. Build recipe

Nothing to supply. Canvas 1080x1528.

```bash
pixelox new -o talk.pxo --size 1080x1528 --fill '#1e1d20'
pixelox render 3d --primitive torus --dims 1,0.2,1 --camera iso --style shaded --color '#c8ccd2' --segments 200 -o ring.png --size 900x900
pixelox render 3d --primitive torus --dims 1,0.2,1 --camera dimetric --style shaded --color '#c8ccd2' --segments 200 -o ring2.png --size 900x900
pixelox comp place talk.pxo ring2.png --in-place --as DimA --size 700x --at 6%,8% --anchor c
pixelox comp place talk.pxo ring.png --in-place --as DimB --size 640x --at 94%,80% --anchor c
pixelox comp place talk.pxo ring2.png --in-place --as RingB --size 700x --at 62%,12% --anchor c
pixelox comp place talk.pxo ring.png --in-place --as RingA --size 760x --at 74%,22% --anchor c
pixelox adjust levels talk.pxo --in-place --layer RingA --in-black 95 --in-white 212
pixelox adjust gradient-map talk.pxo --in-place --layer RingA --stops '#060607@0,#3a3d43@0.14,#0c0c0e@0.26,#aeb5bf@0.4,#f4f6f8@0.5,#2a2c31@0.6,#0a0a0c@0.7,#c9cfd7@0.86,#ffffff@1'
pixelox filter gaussian talk.pxo --in-place --layer RingA --sigma 1.6
pixelox adjust levels talk.pxo --in-place --layer RingB --in-black 95 --in-white 212
pixelox adjust gradient-map talk.pxo --in-place --layer RingB --stops '#060607@0,#3a3d43@0.14,#0c0c0e@0.26,#aeb5bf@0.4,#f4f6f8@0.5,#2a2c31@0.6,#0a0a0c@0.7,#c9cfd7@0.86,#ffffff@1'
pixelox filter gaussian talk.pxo --in-place --layer RingB --sigma 1.6
pixelox adjust gradient-map talk.pxo --in-place --layer DimA --stops '#0a0a0c@0,#2c2e33@0.5,#8a9099@1'
pixelox adjust gradient-map talk.pxo --in-place --layer DimB --stops '#0a0a0c@0,#2c2e33@0.5,#8a9099@1'
pixelox transform rotate talk.pxo --in-place --layer RingA --degrees=-38
pixelox transform rotate talk.pxo --in-place --layer RingB --degrees 55
pixelox transform rotate talk.pxo --in-place --layer DimA --degrees 70
pixelox transform rotate talk.pxo --in-place --layer DimB --degrees=-60
pixelox layer opacity talk.pxo --in-place --layer DimA --value 45
pixelox layer opacity talk.pxo --in-place --layer DimB --value 45
pixelox filter grain talk.pxo --in-place --as-layer --name Grain --amount 8
pixelox text add talk.pxo --in-place --name H1 --content "Deep" --font @neo-grotesque --weight 700 --fit-width 680 --tracking=-70 --color '#ffffff' --at 50%,29.5% --anchor t --anchor-to ink
pixelox layer dup talk.pxo --in-place --layer H1 --name H1Low
pixelox mask add talk.pxo --in-place --layer H1 --from-path 'M 0 0 H 1080 V 600 H 0 Z'
pixelox mask add talk.pxo --in-place --layer H1Low --from-path 'M 0 608 H 1080 V 1528 H 0 Z'
pixelox transform move talk.pxo --in-place --layer H1Low --by 26,0
pixelox text add talk.pxo --in-place --name H2 --content "Dive." --font @neo-grotesque --weight 700 --fit-width 640 --tracking=-70 --color '#ffffff' --at 51%,45.5% --anchor t --anchor-to ink
pixelox text add talk.pxo --in-place --name Sub --content $'A NIGHT ON MACHINES\nTHAT TEACH US' --font @neo-grotesque --weight 300 --size 3.5% --tracking=-40 --leading 100% --align center --color '#ffffff' --at 50%,67.3% --anchor c
pixelox text add talk.pxo --in-place --name Date --content "[MAY 09] / [6PM-8PM]" --font @neo-grotesque --weight 500 --size 4.9% --tracking=-40 --color '#ffffff' --at 50%,79% --anchor c
pixelox text add talk.pxo --in-place --name Address --content "Will be held at 12 Example Road, Your City" --font @neo-grotesque --weight 400 --size 1.75% --tracking=-20 --color '#ffffff' --at 50%,84.8% --anchor c
pixelox text add talk.pxo --in-place --name Cta --content "REGISTER NOW" --font @neo-grotesque --weight 700 --size 1.65% --tracking=-20 --color '#ffffff' --at 10%,93% --anchor l
pixelox text add talk.pxo --in-place --name Site --content "WWW.EXAMPLE.COM" --font @neo-grotesque --weight 400 --size 1.65% --tracking=-20 --color '#ffffff' --at 90%,93% --anchor r
pixelox text add talk.pxo --in-place --name Studio --content $'Studio\nNorthlight' --font @neo-grotesque --weight 700 --size 2.8% --tracking=-50 --leading 95% --color '#ffffff' --at 10%,11% --anchor l
pixelox text add talk.pxo --in-place --name Handle --content "@yourhandle" --font @neo-grotesque --weight 400 --size 1.4% --tracking=-20 --color '#ffffff' --at 10%,17.8% --anchor l
pixelox export talk.pxo -o talk.png
```

Change first: the two headline words (4 to 5 letters each; set the two `--fit-width`
values so both lines have the same cap height), then the cut height: the two mask paths
meet at y 600 and 608 px here, which must fall inside line 1 (between 55% and 75% of its
height reads best).

## 10. Variations and pairings

* Gold or copper instead of chrome: change the gradient map to
  `#1a0f02@0, #7a4b00@0.2, #f6d365@0.45, #3a2200@0.6, #fff3c0@0.85, #d99a1c@1` and warm
  the ground to `#1c1712`.
* One colored light: keep the chrome gray and add a blurred color blob in `screen` mode
  over the rings (T-light-leak) in a single hue, for a "studio gel" look.
* Other primitives: `--primitive sphere` (chrome balls), `cylinder` (pipes), or an
  extruded logo (`--extrude logo.svg`) through the same levels and gradient-map steps.
* Light version: ground `#e9eaec`, type `#111111`, the same chrome (drop its black bands
  to `#3a3d43`).
* Suits talks, launches, club nights, portfolio covers. Pairs with T-outline-type for a
  ghost repeat of the headline and with A-type-block if the rings are dropped.

## 11. Pitfalls

* A chrome render with soft, even shading looks like plastic. The gradient map must jump
  from near-black to near-white at least twice; check that `adjust levels` has stretched
  the render first, or the bands all land in one narrow tone.
* Faceted rings: raise `--segments` to 160 or more and keep the small Gaussian blur.
* Loose tracking on the headline. At 0 tracking this face looks like a default document
  heading; -50 to -80 is the style. Letters may touch; they must not overlap.
* The slit through the x-height band of every letter makes the word hard to read. One cut,
  one line, shift under 3% W.
* Rings centered behind the headline: the white faces swallow the white letters. Keep the
  big ring in a corner, with only one arc passing behind a letter or two.
