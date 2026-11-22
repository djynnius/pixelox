---
id: p022
title: Warm gradient festival poster with bursts
genre: [event-flyer, exhibition, concert]
mood: [festive, warm, playful, dark]
style: [gradient-mesh, flat, type-led]
format: portrait-a
archetype: A-swiss-asymmetric
palette: ['#1b1615', '#76382c', '#e4561f', '#d1990b', '#e54b7b', '#ffffff']
techniques: [T-gradient-mesh, T-geo-shapes, T-rules, T-paper-grain, T-spoke-burst]
needs: nothing
---

# p022: Warm gradient festival poster with bursts

## 1. Concept

An art-festival announcement on a dark brown field whose corners glow orange, amber, and
pink, with three flat firework bursts (yellow, pink, violet) scattered around a white
two-line title in a swashy display face. It works because the glow is pushed to the
corners and the middle stays nearly black, so white type has full contrast, while the
bursts say "celebration" with the simplest possible drawing: sixteen round-capped lines.

## 2. Format and grid

* Portrait A-series (1 : 1.414). Margins 10% W left and right, 7% H top, 8% H bottom.
* Two left edges: 10% W for the header, title, and intro; 43% W for the date block in the
  lower half. The header line's right label ends at 90% W.
* Zones: header 7 to 10% H; bursts and title 16 to 50% H; intro 54 to 60% H; date block
  75 to 93% H. Between the intro and the date block, 15% H of empty dark field (the
  violet burst sits at its left edge).
* The composition is a zigzag: title top left, big burst right, small burst lower left,
  date block lower right.

## 3. Layer stack

1. Ground: very dark warm brown `#1b1615`.
2. Corner glows, heavily blurred: mauve `#91455a` at the top left, orange `#e4561f` at
   the top right, amber `#d1990b` along the bottom left and bottom center, hot pink
   `#e54b7b` at the bottom right, and a dim rust `#76382c` wash down the right side. Each
   glow reaches about 35% of the canvas inward; the area 15 to 60% W, 35 to 80% H stays
   near black.
3. Dark core: a soft radial patch of the ground color centered at 38% W, 52% H, radius
   44% H, which keeps the type area near black. Then grain: one fine grain layer over
   the glow (hides banding in the long gradients).
4. Header hairline: 1 px white at 45% opacity, 10 to 90% W, at 7% H.
5. Header labels: organizer name at 10% W and "PRESENTS" right-aligned at 90% W, 9.2% H.
6. Yellow burst `#f5b324`: center 39.4% W, 24.1% H, radius 11% W, 16 rays, stroke 0.7% W.
7. Pink burst `#f0609e`: center 76% W, 33.8% H, radius 20% W, 16 rays, stroke 1.7% W; its
   lower left rays pass behind the end of the title's second line.
8. Violet burst `#7b4fe0`: center 6.2% W, 63.4% H, radius 9.7% W, 16 rays, stroke 0.75% W,
   half cut by the left edge.
9. Title line 1 (3 letters) at 10% W, cap top 30.4% H; line 2 (8 letters) at 10% W, cap
   top 41% H, 72% W wide. White. The top of line 1 overlaps the yellow burst's lowest rays.
10. Intro: two lines of white sentence-case text at 10% W, 54.9% and 58.4% H.
11. Arrow: a thin white right-pointing arrow, 28.8 to 40.3% W, at 80% H, pointing at the
    date.
12. Date: two bold lines at 43.3% W, 77.4% and 82.6% H; doors-open line at 87.9% H; address
    line at 91.4% H.

All rays have round caps and start at 12 to 15% of the radius, leaving a small empty center.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#1b1615` | dark brown ground | 45% |
| `#76382c` | rust transition between dark and glow | 25% |
| `#e4561f` | orange glow (top right) | 8% |
| `#d1990b` | amber glow (bottom left), yellow burst `#f5b324` | 8% |
| `#e54b7b` | pink glow (bottom right), pink burst `#f0609e` | 8% |
| `#ffffff` | all type | 5% |
| `#7b4fe0` | violet burst: the one cool accent | 1% |

Analogous warm range (amber, orange, pink) on brown-black, plus one small violet as the
complement. White is reserved for type.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Organizer | `[ORGANIZER NAME]`, 2 words | caps | geometric sans (Poppins 400, Montserrat 400 / Avenir Next) | 400 | 1.5% | 30 | - | left | `#ffffff` | 10%, 9.2% |
| Presents | "PRESENTS" | caps | same | 400 | 1.5% | 30 | - | right | `#ffffff` | 90%, 9.2% |
| Title | 2 words on 2 lines: 3 letters, then 8 | caps | swashed decorative display: heavy flared strokes, curled legs, a wavy crossbar (Rakkas, Elsie Swash Caps 900, Gloock / Luminari) | 400 (the face is heavy) | 7.8% | 0 | line gap 40% of cap height | left | `#ffffff` | 10%, cap tops 30.4% and 41% |
| Intro | one sentence, 12 to 14 words on 2 lines | sentence | geometric sans (Poppins 400 / Avenir Next) | 400 | 1.75% | 10 | 146% | left | `#ffffff` | 10%, 54.9% and 58.4% |
| Date | day + month / year, 2 lines | caps | geometric sans (Poppins 700 / Avenir Next) | 700 | 3.1% | 0 | 121% | left | `#ffffff` | 43.3%, 77.4% and 82.6% |
| Doors | "DOORS OPEN 9AM" | caps | same | 400 | 2% | 20 | - | left | `#ffffff` | 43.3%, 87.9% |
| Address | street, city | title | same | 400 | 2% | 10 | - | left | `#ffffff` | 43.3%, 91.4% |

Two families: one eccentric display face used once, and one plain geometric sans for
everything else. Title to date to body is 4.5 : 1.8 : 1.

## 6. Imagery

No photograph or illustration. The only pictures are three firework bursts, each one a
single path of sixteen straight spokes with round caps and a small hollow center. They differ
in size by about 2 : 1 : 1 and in color, and each sits in a different third of the page.
The largest carries the thickest stroke.

## 7. Effects and textures

* **T-gradient-mesh** with fixed centers: `blobs.js` with `at=` listing six corner and
  edge points (some outside 0..1), `size=0.68`, then `filter gaussian` sigma 55. A
  radial gradient of the ground color (95% opaque at 38% W, 52% H, fading to nothing 44%
  H away) then re-darkens the middle so the type area is near black.
* **T-geo-shapes**: each burst is one `shape path` made of `M inner L outer` pairs, one
  per ray, with `--cap round`. For ray k of n at angle a = k x 360 / n: inner point =
  center + 0.13 x radius x (cos a, sin a), outer point = center + radius x (cos a, sin a).
* **T-rules**: the header hairline; `shape arrow` for the pointer.
* **T-paper-grain**: `filter grain --as-layer --amount 8`, added before the bursts and type,
  so it grains the glow below it and leaves the flat shapes and letters clean.
* **T-spoke-burst** in [techniques.md](../techniques.md) is the tested library version of the firework bursts (`rays.js` with `style=spokes`); start from it when reusing this idea.

## 8. Hierarchy and eye path

1. The title (largest white shape) together with the pink burst beside it.
2. The intro sentence under the title.
3. The arrow, which carries the eye across the empty band to the date (bold, second
   largest type).
4. Doors and address; header line last.

## 9. Build recipe

Nothing to supply. Canvas 1080x1528.

```bash
pixelox render js scripts/blobs.js -o glow.png --size 1080x1528 --param bg='#1b1615' --param colors='#91455a,#e4561f,#d1990b,#ca8519,#e54b7b,#7a3526' --param at='-0.02,-0.02;1.05,-0.03;-0.05,1.05;0.5,1.12;1.05,1.02;1.1,0.5' --param size=0.68 --param vary=0 --param alpha=1
pixelox new -o fest.pxo --size 1080x1528 --fill '#1b1615'
pixelox comp place fest.pxo glow.png --in-place --as Glow --fit cover
pixelox filter gaussian fest.pxo --in-place --layer Glow --sigma 55
pixelox layer add fest.pxo --in-place --name Core
pixelox fill gradient fest.pxo --in-place --layer Core --kind radial --from 38%,52% --to 38%,96% --stops '#17131200@1,#171312f2@0,#171312cc@0.45'
pixelox filter grain fest.pxo --in-place --as-layer --name Grain --amount 8
pixelox shape line fest.pxo --in-place --name Hairline --from 10%,7% --to 90%,7% --stroke '#ffffff' --stroke-width 1 --opacity 45
pixelox text add fest.pxo --in-place --name Organizer --content "HARBOR ARTS TRUST" --font @geometric-sans --weight 400 --size 2.1% --tracking 30 --color '#ffffff' --at 10%,9.2% --anchor l
pixelox text add fest.pxo --in-place --name Presents --content "PRESENTS" --font @geometric-sans --weight 400 --size 2.1% --tracking 30 --color '#ffffff' --at 90%,9.2% --anchor r
pixelox shape path fest.pxo --in-place --name BurstYellow --d 'M 444 371 L 543 391 M 441 378 L 525 434 M 436 383 L 493 467 M 430 386 L 450 485 M 423 386 L 403 485 M 416 383 L 360 467 M 411 378 L 327 435 M 408 372 L 309 392 M 408 365 L 309 345 M 411 358 L 327 302 M 416 353 L 359 269 M 422 350 L 402 251 M 429 350 L 449 251 M 436 353 L 492 269 M 441 358 L 525 301 M 444 364 L 543 344' --fill none --stroke '#f5b324' --stroke-width 8 --cap round
pixelox shape path fest.pxo --in-place --name BurstPink --d 'M 845 519 L 1036 539 M 842 527 L 1011 619 M 836 535 L 957 684 M 828 539 L 882 723 M 818 540 L 798 731 M 810 537 L 718 706 M 802 531 L 653 652 M 798 523 L 614 577 M 797 513 L 606 493 M 800 505 L 631 413 M 806 497 L 685 348 M 814 493 L 760 309 M 824 492 L 844 301 M 832 495 L 924 326 M 840 501 L 989 380 M 844 509 L 1028 455' --fill none --stroke '#f0609e' --stroke-width 18 --cap round
pixelox shape path fest.pxo --in-place --name BurstViolet --d 'M 81 969 L 172 969 M 80 974 L 164 1009 M 77 979 L 141 1043 M 72 982 L 107 1066 M 67 983 L 67 1074 M 62 982 L 27 1066 M 57 979 L -7 1043 M 54 974 L -30 1009 M 53 969 L -38 969 M 54 964 L -30 929 M 57 959 L -7 895 M 62 956 L 27 872 M 67 955 L 67 864 M 72 956 L 107 872 M 77 959 L 141 895 M 80 964 L 164 929' --fill none --stroke '#7b4fe0' --stroke-width 8 --cap round
pixelox text add fest.pxo --in-place --name T1 --content "OPEN" --font @uncial --size 11% --color '#ffffff'
pixelox transform move fest.pxo --in-place --layer T1 --to 10%,30.4% --anchor tl
pixelox text add fest.pxo --in-place --name T2 --content "STUDIOS" --font @uncial --size 11% --color '#ffffff'
pixelox transform move fest.pxo --in-place --layer T2 --to 10%,41% --anchor tl
pixelox text add fest.pxo --in-place --name Intro --content $'Three days of paint, clay, print,\nand sound across the old harbor.' --font @geometric-sans --weight 400 --size 2.4% --tracking 10 --leading 146% --color '#ffffff' --at 10%,56.6% --anchor l
pixelox shape arrow fest.pxo --in-place --name Arrow --from 28.8%,80% --to 40.3%,80% --stroke '#ffffff' --stroke-width 4 --head-size 34
pixelox text add fest.pxo --in-place --name Date --content $'21 SEPTEMBER\n2031' --font @geometric-sans --weight 700 --size 4.3% --leading 121% --color '#ffffff' --at 43.3%,80% --anchor l
pixelox text add fest.pxo --in-place --name Doors --content "DOORS OPEN 10AM" --font @geometric-sans --weight 400 --size 2.75% --tracking 20 --color '#ffffff' --at 43.3%,87.9% --anchor l
pixelox text add fest.pxo --in-place --name Address --content "12 Example Road, Your City" --font @geometric-sans --weight 400 --size 2.75% --tracking 10 --color '#ffffff' --at 43.3%,91.4% --anchor l
pixelox export fest.pxo -o fest.png
```

Change first: the title (a short first line, a longer second line; the second should span
65 to 75% W), then the three burst colors, then the corner glow colors. If you register a
swashed display font (`fonts add`), use it for T1 and T2; any heavy face with one or two
eccentric letters keeps the mood.

## 10. Variations and pairings

* Cool night version: ground `#0d1020`, glows `#3b2a8f / #1557c0 / #19c2c9 / #7a3cff`,
  bursts in white, cyan, and pink.
* Daylight version: ground `#fff4e0` with `edge=1` pastel glows, title and text in
  `#2a1a14`, bursts at full saturation.
* Swap bursts for other one-path motifs: concentric rings (`pattern.js kind=rings` in a
  small canvas), confetti (`particles.js shape=rect`), or stars (`shape star`).
* Square post: title and pink burst only, date as one bold line at the bottom.
* Suits open days, street-food markets, new-year events, school fairs. Pairs with
  T-bokeh instead of bursts and with T-pill-button for a ticket link.

## 11. Pitfalls

* Glow in the middle: white text on mid orange loses contrast. Keep all blob centers on
  or outside the canvas edge and check that the title area is darker than 20% lightness.
* Bursts with square caps or a filled center look like asterisks or warning icons. Round
  caps and a small hollow middle (rays starting at 10 to 20% of the radius) make them fireworks.
* Three bursts of the same size in a row. Use one big and two small, on a zigzag.
* A second decorative font for the date. The display face appears once; dates and
  addresses stay in the plain sans.
* Dropping the grain: a blur this wide bands visibly on dark colors.
