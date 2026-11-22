---
id: p118
title: Neon tech flyer with diagonal photo panel
genre: [tech-ad, product-ad, corporate]
mood: [futuristic, dark, energetic, bold]
style: [neon, geometric, photo-composite]
format: portrait-a
archetype: A-split-field
palette: ['#3e033d', '#150038', '#58074a', '#ffffff', '#ff2bd6', '#19d3ff']
techniques: [T-neon-glow, T-fit-width, T-rules, T-cutout-drop-shadow, T-shape-crop, T-icon-row]
needs: [product-cutout, logo]
---

# p118: Neon tech flyer with diagonal photo panel

## 1. Concept

A technology flyer split by two diagonals: a kite-shaped picture panel in the upper left
shows a phone lying on a grid of glowing neon tubes, and a deep plum field wraps around it
on the right and bottom and carries a right-aligned italic headline. It works because
everything leans the same way: the panel edges, a handful of small neon dashes that run
parallel to them, and the slanted headline all share two angles, so a static page feels
like it is moving. Presented as a flat sheet on a lilac ground with a soft shadow; the
mockup is not part of the design.

## 2. Format and grid

* Portrait A-series (1 : 1.414). Right margin 5.5% W: every text block is right-aligned to
  x 94.5% W. Left and bottom margins 4% W for the small marks.
* The picture panel is a four-corner polygon: `0,0` to `54.5% W,0` to `73% W,43% H` to
  `0,77% H`. Its right edge falls at 73 degrees from horizontal, its lower edge rises at
  33 degrees; those are the only two angles in the design.
* The plum field is everything else: a strip down the right side (27 to 45% W wide) and
  the whole lower part below the rising diagonal.
* Text column: x 39 to 94.5% W, y 58 to 92% H, inside the lower right of the field.
  Logo line 59% H, headline 65 to 74.5% H, body 77.5 to 86.5% H, web address 91.3% H.

## 3. Layer stack

1. Panel ground: deep indigo `#150038` (seen between the neon tubes).
2. Neon tubes: six to eight straight tubes in two directions that cross at right angles
   (one family falling to the right at 30 degrees, one falling to the left at 60 degrees),
   each 1.5% W thick with a near-white core and a wide glow in magenta `#ff2bd6`, blue
   `#2f4bff`, or cyan `#19d3ff`. They run off the panel edges.
3. Product: the back of a phone (three camera lenses), dark violet-gray, lying across the
   panel at about 60 degrees, `12%,16%,61% x 37%`, cut off by the panel's right edge, with
   a dark shadow under it and rim light from the tubes.
4. Plum field: the polygon `54.5%,0` / `100%,0` / `100%,100%` / `0,100%` / `0,77%` /
   `73%,43%`, gradient from `#23022f` at the top right to `#58074a` at the bottom, casting
   a soft dark shadow back onto the panel (it reads as a layer in front).
5. Neon dashes on the field, parallel to the panel edges, 0.7% W thick with round ends:
   cyan (61 to 64% W, 2 to 9% H), hot pink (70 to 77% W, 10 to 26% H), amber (71 to 76% W,
   27 to 38% H) beside the right edge; purple (54 to 74% W, 46 to 56% H), amber (22 to
   35% W, 69.5 to 75% H), blue (8 to 22% W, 70 to 77% H) beside the lower edge.
6. Circuit traces: thin lilac lines with right-angle bends and node dots, 45% opacity, in
   the top right corner (80 to 94% W, 0 to 18% H) and at the bottom (33 to 57% W, 92 to
   100% H).
7. Small marks: two rows of five slanted hatch bars (86 to 93% W at 37.5% H, and 8 to
   18% W at 88.3% H), one white triangle (87% W, 50% H), one white cross (27.5% W, 83.3% H).
8. Logo lockup: white icon plus a one-word name, right edge at 94% W, 59.2% H.
9. Headline: two lines, white heavy italic caps, right-aligned at 94.4% W, cap height
   3.6% H, cap tops at 65% and 71.1% H, the longer line 53% W wide.
10. Body: four right-aligned lines, off-white, cap height 1.3% H, top at 77.5% H.
11. Web address: right-aligned at 94.5% W, 91.3% H.
12. Social row: four small outlined circles with icons, 3.4% W each, 3.8 to 22% W at
    96.3% H.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#3e033d` | plum field (mid) | 35% |
| `#150038` | indigo panel ground, shadows | 25% |
| `#58074a` | field, lighter bottom | 15% |
| `#23022f` | field, darker top right | 10% |
| `#ffffff` | type, tube cores | 7% |
| `#ff2bd6` | magenta glow | 5% |
| `#19d3ff`, `#2f4bff`, `#f29a1f`, `#e0125a` | cyan, blue, amber, pink accents | 3% |

Dark analogous base (indigo to plum, 40 degrees of hue) with neon accents in small doses.
The warm amber dash is the only color from the other side of the wheel; it appears twice.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Logo name | `[BRAND]`, one word: first half bold, second half regular | title | geometric sans (Montserrat / Avenir Next) | 700 + 400 | 1.5% | 0 | - | right | `#ffffff` | 94%, 59.2% |
| Headline | 2 generic words, one per line, 10 letters each | caps | heavy italic grotesque (Montserrat ExtraBold Italic, Archivo Black slanted / Avenir Next Heavy Italic) | 900 italic | 3.6% | 0 | 121% | right | `#ffffff` | 94.4%, cap top 65% |
| Body | 4 lines, 4 to 6 words each (placeholder text) | sentence | geometric sans (Montserrat / Avenir Next) | 500 | 1.3% | 0 | 128% | right | `#f1e9f4` | 94.5%, top 77.5% |
| Web address | one URL | lower | same | 400 | 1.3% | 0 | - | right | `#ffffff` | 94.5%, 91.3% |

One family. The italic is used once, on the headline, and its slant (about 12 degrees)
echoes the diagonals. Ratio headline : body = 2.8 : 1.

## 6. Imagery

A 3D render: the back of a smartphone, matte violet-gray, resting on a dark surface
crossed by glowing neon tubes; shot from above, tilted about 60 degrees, lit only by the
tubes (pink and blue rim light on the phone's edges, blue reflections in the lenses).
About 40% of the phone is hidden by the field. In the recipe the scene is rebuilt from
parts so that any product cutout can be used: tubes with **T-neon-glow** (line version),
then the product with a dark shadow (**T-cutout-drop-shadow**, soft variant). If you have
a finished dark product photo, place it with `comp place --fit cover` under the Field
layer and skip the tube and product lines.

## 7. Effects and textures

* **T-neon-glow** on `shape line` layers: near-white stroke (`#ffe9ff`), 18 px,
  `fx outer-glow --blur 44 --spread 12` in the tube color, copied with `fx copy`.
* Panel mask without a mask: the plum field is drawn last as one `shape path` polygon
  that covers everything outside the panel. `fx overlay --angle 70` gives its gradient and
  `fx drop-shadow --offset=-8,-8 --blur 36` makes it float over the picture.
* **T-rules**: the six neon dashes, the hatch bars (a row of slashes set as heavy type),
  the circuit traces (`shape path`, stroke only, 45% opacity).
* **T-fit-width** on the headline block (57 px short of the text column so the italic does
  not crowd the margin).
* **T-icon-row**, small: four `shape icon` glyphs, 24 px, each centered in a 42 px ring
  (one `shape path` of four circles, 2 px stroke), 52 px apart. The bundled icons have no
  brand logos, so the row uses neutral ones: `at-sign`, `message-circle`, `share-2`,
  `globe`.
* No grain. The glow gradients are short enough not to band.
* **T-shape-crop** in [techniques.md](../techniques.md) is the tested library version of the photo shown through a drawn shape (arch, panel, tiles, or letter); start from it when reusing this idea.

## 8. Hierarchy and eye path

1. The neon crossing at the top left (brightest pixels) and the product on it.
2. Down the rising diagonal, helped by the dashes, to the headline (largest type, white
   on plum).
3. Logo line just above the headline, then the body and the web address on the same
   right edge.
4. Small marks and social icons last; they are texture, not information.

## 9. Build recipe

`product.png` (a device or any product cutout) and `logo.png` (a small white mark) are
your images here. The stand-in product is a bottle; for a phone use `--size x700`.

```bash
pixelox new -o tech.pxo --size 1080x1528 --fill '#150038'
pixelox shape line tech.pxo --in-place --name TubeA1 --from 0,130 --to 300,308 --stroke '#ffe9ff' --stroke-width 18 --cap round
pixelox shape line tech.pxo --in-place --name TubeA2 --from 325,45 --to 85,445 --stroke '#ffe9ff' --stroke-width 18 --cap round
pixelox shape line tech.pxo --in-place --name TubeA3 --from 560,170 --to 300,640 --stroke '#ffe9ff' --stroke-width 18 --cap round
pixelox shape line tech.pxo --in-place --name TubeA4 --from 0,950 --to 175,1055 --stroke '#ffe9ff' --stroke-width 18 --cap round
pixelox shape line tech.pxo --in-place --name TubeA5 --from 120,915 --to 10,1115 --stroke '#ffe9ff' --stroke-width 18 --cap round
pixelox fx outer-glow tech.pxo --in-place --layer TubeA1 --blur 44 --spread 12 --color '#ff2bd6' --opacity 95
pixelox fx copy tech.pxo --in-place --from TubeA1 --to TubeA2,TubeA3,TubeA4,TubeA5
pixelox shape line tech.pxo --in-place --name TubeB1 --from 240,0 --to 660,245 --stroke '#e6f0ff' --stroke-width 16 --cap round
pixelox shape line tech.pxo --in-place --name TubeB2 --from 520,760 --to 420,930 --stroke '#e6fbff' --stroke-width 16 --cap round
pixelox fx outer-glow tech.pxo --in-place --layer TubeB1 --blur 44 --spread 12 --color '#2f4bff' --opacity 95
pixelox fx outer-glow tech.pxo --in-place --layer TubeB2 --blur 44 --spread 12 --color '#19d3ff' --opacity 95
pixelox comp place tech.pxo product.png --in-place --as Product --size x640 --at 38%,35% --anchor c
pixelox transform rotate tech.pxo --in-place --layer Product --degrees 62
pixelox fx drop-shadow tech.pxo --in-place --layer Product --offset 0,18 --blur 40 --color '#0a0020' --opacity 70
pixelox shape path tech.pxo --in-place --name Field --d 'M 589 0 L 1080 0 L 1080 1528 L 0 1528 L 0 1177 L 788 657 Z' --fill '#3e033d'
pixelox fx overlay tech.pxo --in-place --layer Field --stops '#58074a@0,#3e033d@0.5,#23022f@1' --angle 70
pixelox fx drop-shadow tech.pxo --in-place --layer Field --offset=-8,-8 --blur 36 --color '#0a0018' --opacity 70
pixelox shape line tech.pxo --in-place --name DashCyan --from 659,31 --to 691,138 --stroke '#1f8fd0' --stroke-width 7 --cap round
pixelox shape line tech.pxo --in-place --name DashPink --from 756,160 --to 832,397 --stroke '#e0125a' --stroke-width 8 --cap round
pixelox shape line tech.pxo --in-place --name DashAmber --from 767,413 --to 821,581 --stroke '#f29a1f' --stroke-width 7 --cap round
pixelox shape line tech.pxo --in-place --name DashPurple --from 799,703 --to 583,856 --stroke '#8a3fb5' --stroke-width 7 --cap round
pixelox shape line tech.pxo --in-place --name DashAmber2 --from 378,1062 --to 238,1146 --stroke '#f29a1f' --stroke-width 7 --cap round
pixelox shape line tech.pxo --in-place --name DashBlue --from 238,1070 --to 86,1177 --stroke '#1f8fd0' --stroke-width 8 --cap round
pixelox shape path tech.pxo --in-place --name Circuit1 --d 'M 880 0 L 880 55 L 925 100 L 925 270 M 925 150 L 990 150 L 990 215 M 925 130 L 870 130 M 960 0 L 1010 60' --fill none --stroke '#b79ad0' --stroke-width 2 --opacity 45
pixelox shape path tech.pxo --in-place --name Circuit2 --d 'M 360 1528 L 440 1450 L 560 1450 M 475 1528 L 475 1420 L 530 1420 L 530 1470 L 610 1470' --fill none --stroke '#b79ad0' --stroke-width 2 --opacity 45
pixelox text add tech.pxo --in-place --name Hatch1 --content "/////" --font @geometric-sans --weight 900 --size 1.9% --color '#cbb9d6' --at 89.5%,37.6% --anchor c
pixelox text add tech.pxo --in-place --name Hatch2 --content "/////" --font @geometric-sans --weight 900 --size 2.1% --color '#a98bb5' --at 13%,88.3% --anchor c
pixelox shape polygon tech.pxo --in-place --name Tri --center 87.3%,49.8% --radius 10 --sides 3 --rotation 20 --fill '#ffffff'
pixelox text add tech.pxo --in-place --name Cross --content "x" --font @heavy-wide --size 2.4% --color '#ffffff' --at 27.5%,83.3% --anchor c
pixelox comp place tech.pxo logo.png --in-place --as Logo --size 38x --at 77%,59.2% --anchor c
pixelox text add tech.pxo --in-place --name LogoText --content "Voltworks" --font @geometric-sans --weight 600 --size 2.1% --color '#ffffff' --at 94.2%,59.2% --anchor r
pixelox text add tech.pxo --in-place --name Headline --content $'SMART DEVICE\nREVOLUTION' --font @geometric-sans --weight 900 --style italic --fit-width 570 --leading 121% --align right --color '#ffffff' --at 94.4%,65% --anchor tr --anchor-to ink
pixelox text add tech.pxo --in-place --name Body --content $'Say what the product does in one line,\nwho it is for in the next, and why\nit matters now. Close with the one thing\nyou want them to do.' --font @geometric-sans --weight 500 --size 1.95% --leading 130% --align right --color '#f1e9f4' --at 94.5%,77% --anchor tr
pixelox text add tech.pxo --in-place --name Site --content "www.example.com" --font @geometric-sans --weight 400 --size 1.95% --color '#ffffff' --at 94.5%,91.3% --anchor r
pixelox shape path tech.pxo --in-place --name SocialRings --d 'M 41 1471 A 21 21 0 1 1 83 1471 A 21 21 0 1 1 41 1471 Z M 93 1471 A 21 21 0 1 1 135 1471 A 21 21 0 1 1 93 1471 Z M 145 1471 A 21 21 0 1 1 187 1471 A 21 21 0 1 1 145 1471 Z M 197 1471 A 21 21 0 1 1 239 1471 A 21 21 0 1 1 197 1471 Z' --fill none --stroke '#e6d9ee' --stroke-width 2
pixelox shape icon tech.pxo at-sign --in-place --name Social1 --size 24 --at 62,1471 --anchor c --color '#e6d9ee'
pixelox shape icon tech.pxo message-circle --in-place --name Social2 --size 24 --at 114,1471 --anchor c --color '#e6d9ee'
pixelox shape icon tech.pxo share-2 --in-place --name Social3 --size 24 --at 166,1471 --anchor c --color '#e6d9ee'
pixelox shape icon tech.pxo globe --in-place --name Social4 --size 24 --at 218,1471 --anchor c --color '#e6d9ee'
pixelox export tech.pxo -o tech.png
```

The social row is four bundled icons in rings (`pixelox shape icons --search WORD` finds
others); for a real network's logo place your own file there with `comp place`. Change
first: the headline (two lines of similar length, the
first slightly longer), then the product, then the three glow colors.

## 10. Variations and pairings

* Hue shifts of the same dark base: navy `#061a3a` to teal `#0a3d4a` with lime and cyan
  tubes for fintech; black to oxblood `#3a0612` with red and amber tubes for gaming;
  graphite to violet with white tubes only for a premium launch.
* Mirror it: panel in the upper right, text left-aligned at 5.5% W, when the product
  faces left.
* Square post: keep the panel polygon (`0,0`, `60%,0`, `80%,45%`, `0,72%`), drop the
  body text, keep headline and web address.
* Replace the product with a photo of a speaker, a team, or a building clipped by the
  same field polygon for a conference or webinar flyer.
* Pairs with T-glitch on the picture panel, T-pattern (dots, `fade=out`) in place of the
  circuit traces, and T-pill-button under the body text.

## 11. Pitfalls

* Unrelated angles. Every dash must be parallel to one of the two panel edges; a third
  angle (or a horizontal rule) breaks the sense of motion.
* Saturated tube cores. A magenta line with a magenta glow looks like a flat stripe; the
  core is near-white and only the glow carries the color.
* Too many small marks. One triangle, one cross, two hatch rows, two trace groups: each
  kind appears once or twice, placed in empty corners, never near the text block.
* Left-aligned or centered text. The text hangs from the right margin so its ragged left
  side follows the rising diagonal; centering it leaves an awkward gap against the panel.
* A bright, busy product photo. The panel must stay darker than the white type; grade a
  light photo down first (`adjust levels --gamma 0.7`) or tint it with T-duotone.
