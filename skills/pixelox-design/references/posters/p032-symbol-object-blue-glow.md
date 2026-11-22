---
id: p032
title: Corporate symbol object on blue glow
genre: [corporate, tech-ad]
mood: [cool, serious, calm]
style: [photo-composite, minimal, 3d-render]
format: portrait-3x4
archetype: A-hero-object
palette: ['#04426b', '#041c3b', '#0184b4', '#43bbe6', '#f0c020', '#ffffff']
techniques: [T-spotlight, T-pattern, T-smoke, T-vignette, T-rules, T-label-box]
needs: product-cutout
---

# p032: Corporate symbol object on blue glow

## 1. Concept

A corporate image ad for a telecom and engineering services group: one composite object
(a small globe wearing a yellow hard hat and a call-center headset) floats in the middle
of a deep blue field that glows cyan behind it, with a photographic cloud drifting in from
the left. Three symbols fused into one object say "engineers, on the phone, worldwide"
without a headline. The copy is tiny: a logo badge in the top right corner, a three-line
paragraph, and a two-word tagline at the bottom. It works because the picture is the
headline and nothing competes with it.

## 2. Format and grid

* Portrait, about 3:4. One center axis at 50% W: the object, the paragraph, and the
  tagline are centered on it. Only the logo badge (top right) and the cloud (left) break
  the symmetry, and they balance each other diagonally.
* Zones: logo badge 1.5 to 5.4% H; empty dark sky to 24% H; cloud 24 to 47% H; object
  36 to 65% H (visual center at 51% H, slightly above the middle); floor glow 64 to 70% H;
  empty band 70 to 81% H; paragraph 81.5 to 85.6% H; tagline at 92.2% H.
* Margins: paragraph column 22 to 78.5% W (56% wide); badge 4.7% W from the right edge.
* The object takes only 42% W x 29% H. About 70% of the canvas is empty blue.

## 3. Layer stack

1. Ground: radial gradient centered at 50% W, 58% H: cyan `#1a9fd0` at the center,
   `#0184b4` at 30% of the radius, `#04507a` at the left and right edges, navy
   `#041c3b` in the four corners.
2. Grid texture: a fine dark square grid (cell 1.1% W, hairline) over the whole field,
   visible in the dark outer areas and washed out by the glow in the middle.
3. Center glow: a second, softer cyan glow over the grid, radius 30% W, so the texture
   disappears behind the object.
4. Cloud: a white cumulus cloud bank, `0%,24%,56% x 23%`, brightest at its top edge near
   35% W, 29% H, dissolving to nothing at its lower edge and to the right; a few thin
   wisps continue to 78% W at 33% H, behind the object.
5. Floor glow: a flat, bright cyan ellipse `#43bbe6`, `28%,63.5%,44% x 6%`, heavily
   feathered: the light the object "stands" on.
6. Object: `29.3%,36%,42% x 29.5%`. A glossy yellow hard hat `#f0c020` (36.8 to 57.4% H)
   on a globe (blue ocean, green land `#3aa23a`, 30 to 67% W, 44 to 65% H); a black
   headset band arcs over the hat from the top to the right ear and its microphone boom
   swings down to the lower left. Lit from the top left, with a sharp highlight on the hat.
7. Logo badge: rounded rectangle `74.2%,1.5%,21.1% x 3.9%`, mid blue `#1b5cb8`; inside, the
   brand name in thin white letters, tracked wide, with a 2 px underline (white for the
   left 80%, light blue for the rest).
8. Accent bar: a vertical orange-red `#d9603c` bar, 0.5% W wide, 21.5% W, 81.6 to 85.6% H,
   at the left of the paragraph.
9. Paragraph: three justified lines of small pale text (`#cfe3ee`), 22.8 to 78.5% W,
   first two words in bold white; cap height 0.8% H, line pitch 1.32% H.
10. Tagline: two words, white, centered at 92.2% H, 26% W wide: the first in regular
    weight, the second in bold; cap height 1.75% H.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#04426b` | mid blue of the field | 40% |
| `#041c3b` | navy corners | 25% |
| `#0184b4` | inner glow (to `#1a9fd0`, floor `#43bbe6`) | 20% |
| `#ffffff` | cloud, type | 7% |
| `#f0c020` | hard hat | 5% |
| `#3aa23a` | land on the globe | 1% |
| `#1b5cb8` | logo badge | 1% |
| `#d9603c` | accent bar | under 1% |

Monochrome blue from navy to cyan, with one yellow object. Yellow against blue is the
strongest warm-cool contrast available, and the yellow appears nowhere else, so the
object needs no outline, no frame, and no caption.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Logo | `[BRAND]`, one compound word, mid-word capital | title | thin wide sans (Jost 300, Raleway 300 / Avenir Next 275 to 400) | 300 | 1.7% | 160 | - | center | `#ffffff` | 84.7%, 3.2% |
| Paragraph | 3 lines, about 40 words; opens with `[COMPANY NAME]` in bold | sentence | geometric sans (Questrial, Jost / Avenir Next, Futura) | 400, lead-in 700 | 0.8% | 0 | 118% | justify | `#cfe3ee` | 22.8 to 78.5%, top 81.6% |
| Tagline word 1 | short word or contraction, 5 to 6 letters | title | same | 400 | 1.75% | 0 | - | right, ends at 46.5% | `#ffffff` | 46.5%, 92.2% |
| Tagline word 2 | one strong word, 9 letters | title | same | 700 | 1.75% | 0 | - | left, starts at 47.6% | `#ffffff` | 47.6%, 92.2% |

One geometric family. The only typographic event is the weight change inside the
tagline: light for the lead-in, bold for the claim.

## 6. Imagery

A photo-realistic composite of three objects that each stand for one part of the
business: a globe (reach), a hard hat (engineering), a headset (service). They are fused
into a single character: the hat sits on the globe like on a head and the headset is
worn over it. It is rendered or retouched to look like one studio object with one light
(top left) and one soft ground reflection. For a build, supply the finished composite as
one cutout, or use any single object that sums up the company (a tool, a device, a
model building). The cloud is a real sky photo cut out softly; the recipe draws a
stand-in cloud with the fractal noise generator of **T-smoke** at high contrast.

## 7. Effects and textures

* Ground glow: `fill gradient --kind radial` with four stops; this is **T-spotlight**
  scaled up to the whole canvas, combined with the dark corners of **T-vignette** in the
  same gradient.
* **T-pattern**, `kind=grid`, `cell=12`, `weight=0.1`, black at alpha 0.35, placed under a
  second, smaller radial glow so it only shows toward the edges.
* Cloud: `clouds.js` with `lo=transparent`, `hi='#ffffff'`, `contrast=2.2`,
  `bias=-0.1`, masked with a feathered ellipse (`select ellipse --feather 60`, then
  `mask add --from @sel`) and softened with `--sigma 3`.
* Floor glow: feathered ellipse filled on an empty layer (the unclipped form of the
  **T-spotlight** shadow, here used as light).
* Object: `fx drop-shadow --offset 0,18 --blur 40 --color '#021a33' --opacity 45`.
* **T-label-box** for the logo badge; **T-rules** for the underline and the accent bar.

## 8. Hierarchy and eye path

1. The yellow hat (the single warm, saturated shape, at the brightest point of the field).
2. Around the object: the headset arc, the microphone, the globe.
3. Left along the cloud, then up and across to the logo badge in the corner.
4. Down the center axis to the tagline (bold word first), then the paragraph.

Cap heights: tagline 1.75 : logo 1.7 : paragraph 0.8, about 2.2 : 2.1 : 1. All type is
small; the object is 16 times taller than the tagline's capitals.

## 9. Build recipe

`product.png` is your image here: the symbolic object as one cutout. For an object that
is wider than tall, use `--size 450x` instead of `--size x420`.

```bash
pixelox render js scripts/pattern.js -o grid.png --size 1080x1440 --param kind=grid --param color='#000000' --param cell=12 --param weight=0.1 --param alpha=0.35
pixelox render js scripts/clouds.js -o cloud.png --size 1080x1440 --seed 5 --param lo=transparent --param hi='#ffffff' --param scale=4 --param octaves=6 --param warp=0.5 --param contrast=2.2 --param bias=-0.1 --param res=2
pixelox new -o corp.pxo --size 1080x1440 --fill '#041c3b'
pixelox layer add corp.pxo --in-place --name Field
pixelox fill gradient corp.pxo --in-place --layer Field --kind radial --from 50%,58% --to 50%,126% --stops '#1a9fd0ff@0,#0184b4ff@0.3,#04507aff@0.6,#041c3bff@1'
pixelox comp place corp.pxo grid.png --in-place --as Grid --fit cover
pixelox layer add corp.pxo --in-place --name CenterGlow
pixelox fill gradient corp.pxo --in-place --layer CenterGlow --kind radial --from 50%,56% --to 50%,96% --stops '#1a9fd0ff@0,#1392c4b3@0.3,#0a7fb055@0.6,#0184b414@0.85,#0184b400@1'
pixelox comp place corp.pxo cloud.png --in-place --as Cloud --fit cover
pixelox select ellipse corp.pxo --in-place --rect 0%,25%,54%x21% --feather 60
pixelox mask add corp.pxo --in-place --layer Cloud --from @sel
pixelox select none corp.pxo --in-place
pixelox filter gaussian corp.pxo --in-place --layer Cloud --sigma 3
pixelox layer add corp.pxo --in-place --name FloorGlow
pixelox select ellipse corp.pxo --in-place --rect 26%,63%,48%x6.5% --feather 46
pixelox fill solid corp.pxo --in-place --layer FloorGlow --color '#5cc8ee' --mask @sel
pixelox select none corp.pxo --in-place
pixelox layer opacity corp.pxo --in-place --layer FloorGlow --value 75
pixelox comp place corp.pxo product.png --in-place --as Object --size x420 --at 50%,65.5% --anchor b
pixelox fx drop-shadow corp.pxo --in-place --layer Object --offset 0,18 --blur 40 --color '#021a33' --opacity 45
pixelox shape rect corp.pxo --in-place --name Badge --rect 74.2%,1.5%,21.1%x3.9% --radius 8 --fill '#1b5cb8'
pixelox text add corp.pxo --in-place --name Logo --content "NetWorks" --font "Avenir Next" --weight 400 --size 2.4% --tracking 160 --color '#ffffff'
pixelox transform move corp.pxo --in-place --layer Logo --to 84.9%,3.1% --anchor c
pixelox shape line corp.pxo --in-place --name Under1 --from 76.4%,4.55% --to 90%,4.55% --stroke '#ffffff' --stroke-width 2
pixelox shape line corp.pxo --in-place --name Under2 --from 90%,4.55% --to 93.2%,4.55% --stroke '#6fb6f2' --stroke-width 2
pixelox shape rect corp.pxo --in-place --name AccentBar --rect 21.5%,81.6%,0.5%x4% --fill '#d9603c'
pixelox text add corp.pxo --in-place --name Para --content "NetWorks Group runs field engineering and customer support for carriers in nine countries. Our crews build and maintain the towers, and our service desks answer for them, day and night, in six languages, under one contract." --font "Avenir Next" --weight 400 --size 1.12% --leading 118% --box 602x80 --align justify --color '#cfe3ee'
pixelox transform move corp.pxo --in-place --layer Para --to 22.8%,81.6% --anchor tl
pixelox text add corp.pxo --in-place --name Tag1 --content "Always" --font "Avenir Next" --weight 400 --size 2.45% --color '#ffffff' --at 46.5%,92.2% --anchor r
pixelox text add corp.pxo --in-place --name Tag2 --content "Reachable" --font "Avenir Next" --weight 700 --size 2.45% --color '#ffffff' --at 47.6%,92.2% --anchor l
pixelox export corp.pxo -o corp.png
```

Change first: the object, then the hue of the field (keep four stops running from a
light center to a near-black corner of the same hue), then the tagline. The bold lead-in
of the paragraph is left out of the recipe: add it as a separate bold text layer over
the first two words if you need it.

## 10. Variations and pairings

* Hue swaps for other sectors: green `#0b3d2e` to `#35c48a` (energy, agriculture) with a
  white or orange object; charcoal `#101214` to `#5a6470` (industry) with a red object;
  deep purple `#1a0b3d` to `#7a4cff` (software) with a lime object.
* Replace the cloud with T-bokeh lights, T-smoke, or T-sunburst shafts from the top
  (`uneven=0.8`): the field needs one soft secondary element, not more.
* Landscape banner: object at 35% W, tagline and paragraph left-aligned at 58% W, logo
  badge top right.
* Add a real headline: one line, cap height 4% H, at 76% H, in the tagline's two weights.
* Pairs with T-lens-flare on the object's highlight, T-paper-grain at amount 8 to hide
  banding in the big gradient, and T-pill-button under the tagline for a web version.

## 11. Pitfalls

* Banding in the radial gradient: large smooth gradients show steps. Keep the grid
  texture, or add T-paper-grain.
* An object that is too big. At more than 50% W it stops floating and the calm goes;
  keep it between 35 and 45% W with empty field on all sides.
* Several unrelated objects side by side. The idea needs one fused object; if the parts
  cannot be combined into one silhouette, use only one of them.
* The grid at full strength across the center: it cuts through the object's edges. Wash
  it out with the center glow layer.
* White text set large on the glow. The bright center has too little contrast with
  white; keep all type in the dark outer band (above 6% H or below 80% H).
