---
id: p117
title: Spy film poster with spiral iris
genre: [film-poster]
mood: [dramatic, moody, retro, serious]
style: [photo-composite, retro, flat]
format: portrait-a
archetype: A-film-one-sheet
palette: ['#3f3230', '#171813', '#795e4e', '#c68520', '#dcd2ce']
techniques: [T-gradient-type, T-block-shadow, T-photo-fade, T-torn-collage, T-paper-grain, T-credit-block]
needs: subject-cutout
---

# p117: Spy film poster with spiral iris

## 1. Concept

A personalized spy-film one-sheet: a man in a dark suit, cut out, stands in front of a
flat brown spiral whose black center sits behind his head like a halo, with a gold
five-letter title across his waist and a film credit block below. It works because the
ground is a graphic (flat browns, black lines, no photo), so the one photographic thing,
the person, pops without any glow or outline, and because the gold title is the only
saturated color on the sheet. Presented as a printed sheet with a thin white edge and a
drop shadow; that edge is not part of the design.

## 2. Format and grid

* Portrait, close to A-series (1 : 1.39). Center axis at 50% W; side margins 15% W for
  the text lines.
* Vertical zones: actor name 7 to 9.5% H; hero figure 15 to 80% H; title 63.5 to 76.7% H,
  in front of the figure; tagline 78 to 80% H; credit block 82.5 to 89.5% H; footer mark
  at 92% H.
* The spiral's center is at 46.5% W, 27.5% H: on the figure's face, 3.5% left of the
  axis, because the figure looks to his right. The black disc at the center has a radius
  of 24% W, a little more than the head's height.
* Tone runs dark, mid, dark from top to bottom: dark brown 0 to 28% H, tan 28 to 72% H,
  black 75 to 100% H. The title sits on the tan-to-black edge.

## 3. Layer stack

1. Ground: dark brown `#3f3230`.
2. Tan field `#795e4e`: 28 to 78% H, full width, with a ragged, torn-looking upper edge
   that climbs from 30% H at the left to 20% H at the right.
3. Spiral arms: six black `#0f0e0c` lines, 1.2% W thick, leaving the edge of the disc
   almost tangentially and winding clockwise outward until they leave the canvas; the
   gaps between them widen from 4% W near the disc to 30% W at the edges.
4. Black disc: center 46.5%, 27.5%, radius 24% W.
5. Hero: waist-up cutout of a man in a black suit, white shirt, dark tie, adjusting his
   cuff, head turned to his right; top of head 15% H, shoulders 22 to 75% W at 43% H;
   natural color, slightly cool.
6. Bottom shade: black `#171813`, transparent at 60% H to solid at 79% H, over the
   figure's legs and the ground.
7. Reticle: thin gold `#c68520` target, two rings (radii 9% and 4.5% W) and a cross
   (horizontal 10 to 37% W, vertical 60.5 to 79% H), centered at 23.5%, 70.2%, behind the
   title's first letter.
8. Title: one word of five squared capitals, first letter 13.5% H tall, the other four
   9.3% H, all on a baseline at 76.7% H, from 23.5 to 76% W; gold gradient with a thin
   bevel and a hard dark offset shadow.
9. Tagline: white squared capitals, centered at 79.2% H, 68.5% W wide.
10. Credit block: three centered lines of narrow gray capitals at 83.5%, 86%, 88.6% H.
11. Footer mark: a web address in bold condensed gray capitals, centered at 92% H.
12. Fine grain.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#3f3230` | dark brown ground | 30% |
| `#171813` | black: disc, arms, bottom, suit | 35% |
| `#795e4e` | tan field | 22% |
| `#c68520` | gold title and reticle | 5% |
| `#dcd2ce` | name, credits (white for the tagline) | 4% |
| `#a8785a` | skin and warm photo tones | 4% |

Browns and black with one metal accent. The tan is close to a skin shadow tone, so the
figure belongs to the ground instead of sitting on it.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Actor | `[ACTOR NAME]`, 2 words | caps and small caps | wide squared technical sans (Michroma, Orbitron 700, Aldrich / Copperplate 700) | 700 | 1.9% (small caps 1.5%) | 30 | - | center | `#dcd2ce` | 49%, 8.2% |
| Title | `[TITLE]`, one 5-letter word, first letter 45% taller | caps and small caps | same face, bold | 700 | 13.5% and 9.3% | 0 | - | left to right | gold `#f0b03a` to `#8a5c14` | 23.5 to 76% W, baseline 76.7% |
| Tagline | two 2-word sentences, each ending in a full stop | caps | same face | 700 | 2% | 0 | - | center | `#ffffff` | 50%, 79.2%; 68.5% W wide |
| Credits | 3 lines: presenter line; three role-and-name pairs; three more pairs | caps, role words at 55% of the name size | ultra-condensed gothic (Six Caps, League Gothic, Bebas Neue / Avenir Next Condensed 500 squeezed to 60% width) | 400 to 500 | 1.6% | 20 | 160% of cap height | center | `#b9b5b0` | 50%; 83.5%, 86%, 88.6%; 26%, 54%, 45% W wide |
| Footer | one web address | caps | bold condensed sans | 700 | 1.6% | 40 | - | center | `#8a8580` | 50%, 92% |

Two families: a wide squared display face for name, title, and tagline, and a tall narrow
face for the credits. The width contrast between them (each title letter is about as
wide as it is tall; each credit letter is a quarter as wide as tall) is the film-poster
signature.

## 6. Imagery

One studio portrait cut out cleanly (see "Getting a cutout" in techniques.md): waist-up,
dark clothing, body turned three-quarters, head turned the other way, one hand raised to
the chest. Dark clothes matter: below the chest the figure merges with the black bottom
and the title can cross it. No outline, no shadow, no glow.

## 7. Effects and textures

* Spiral: one `shape path` with six sub-paths, each a logarithmic spiral from radius 22%
  W to 160% W over 205 degrees, drawn as cubic curves, stroked 13 px with round caps.
* Tan field: a **T-torn-collage** scrap (`torn.js`, top edge torn, `rough=0.9`, no rim)
  rotated -9 degrees: a quick way to get a ragged painted edge.
* **T-photo-fade** at the bottom: ground black from 60 to 79% H, above the figure.
* **T-gradient-type**: gold `fx overlay`, dark `#8a5c14` at the bottom through `#c98a1f`
  to `#f0b03a` at the top; `fx bevel` size 4 at 50%.
* **T-block-shadow** without the stroke: `fx drop-shadow` offset 5,5, blur 0, color
  `#24180a`.
* **T-paper-grain** amount 10, soft-light 35%.
* **T-credit-block** in [techniques.md](../techniques.md) is the tested library version of the billing block and star row; start from it when reusing this idea.

## 8. Hierarchy and eye path

1. The face, framed by the black disc (the spiral lines all lead to it).
2. Down the tie and the raised hand to the gold title.
3. Tagline directly under the title.
4. Actor name at the top, then the credit block and footer.

Cap heights: title 13.5 and 9.3 : tagline 2 : name 1.9 : credits 1.6, about 8 : 6 : 1.25
: 1.2 : 1. A two-level system: the title, and everything else at almost one size.

## 9. Build recipe

`subject.png` is your image here (a waist-up cutout in dark clothes). The squeezes
(`transform scale` with both a width and a height) rasterize the text (warning PXW2003):
type the final words first.

```bash
pixelox render js scripts/torn.js -o tan.png --size 1500x900 --seed 6 --param color='#7c6150' --param rim=none --param edges=t --param rough=0.9 --param detail=0.8
pixelox new -o film.pxo --size 1080x1528 --fill '#3f3230'
pixelox comp place film.pxo tan.png --in-place --as Tan --at 50%,58% --anchor c
pixelox transform rotate film.pxo --in-place --layer Tan --degrees -9
pixelox shape path film.pxo --in-place --name Arms --d 'M 738 462 C 765 539 763 652 692 766 C 620 880 474 994 254 1019 C 34 1044 -262 974 -520 718 C -779 463 -993 10 -932 -584 M 584 646 C 530 707 432 762 297 757 C 163 752 -9 682 -141 505 C -272 327 -359 35 -267 -316 C -175 -668 109 -1080 655 -1323 M 348 604 C 267 588 170 530 108 411 C 45 292 19 109 107 -94 C 195 -297 405 -519 755 -614 C 1105 -710 1605 -670 2088 -320 M 266 378 C 239 301 241 188 312 74 C 384 -40 530 -154 750 -179 C 970 -204 1266 -134 1524 122 C 1783 377 1997 830 1936 1424 M 420 194 C 474 133 572 78 707 83 C 841 88 1013 158 1145 335 C 1276 513 1363 805 1271 1156 C 1179 1508 895 1920 349 2163 M 656 236 C 737 252 834 310 896 429 C 959 548 985 731 897 934 C 809 1137 599 1359 249 1454 C -101 1550 -601 1510 -1084 1160' --fill none --stroke '#0f0e0c' --stroke-width 13 --cap round
pixelox shape ellipse film.pxo --in-place --name Disc --rect 247,165,510x510 --fill '#0f0e0c'
pixelox comp place film.pxo subject.png --in-place --as Subject --size x980 --at 48%,81% --anchor b
pixelox layer add film.pxo --in-place --name Shade
pixelox fill gradient film.pxo --in-place --layer Shade --from 0,60% --to 0,79% --stops '#17181300@0,#171813ff@1'
pixelox shape path film.pxo --in-place --name Reticle --d 'M 157 1073 A 97 97 0 1 1 351 1073 A 97 97 0 1 1 157 1073 M 205 1073 A 49 49 0 1 1 303 1073 A 49 49 0 1 1 205 1073 M 108 1073 L 400 1073 M 254 924 L 254 1207' --fill none --stroke '#c68520' --stroke-width 3
pixelox text add film.pxo --in-place --name Initial --content "D" --font Copperplate --weight 700 --size 24% --color '#c98a1f'
pixelox transform scale film.pxo --in-place --layer Initial --size 150x200
pixelox transform move film.pxo --in-place --layer Initial --to 23.5%,76.7% --anchor bl
pixelox text add film.pxo --in-place --name Title --content "ECOY" --font Copperplate --weight 700 --size 16.6% --color '#c98a1f'
pixelox transform scale film.pxo --in-place --layer Title --size 400x142
pixelox transform move film.pxo --in-place --layer Title --to 39%,76.7% --anchor bl
pixelox fx overlay film.pxo --in-place --layer Title --stops '#8a5c14@0,#c98a1f@0.5,#f0b03a@1' --angle 90
pixelox fx bevel film.pxo --in-place --layer Title --size 4 --depth 80 --opacity 50
pixelox fx drop-shadow film.pxo --in-place --layer Title --offset 5,5 --blur 0 --color '#24180a' --blend normal --opacity 100
pixelox fx copy film.pxo --in-place --from Title --to Initial
pixelox text add film.pxo --in-place --name Actor --content "Jordan Ellis" --font Copperplate --weight 700 --size 2.9% --tracking 30 --color '#dcd2ce' --at 49%,8.2% --anchor c
pixelox text add film.pxo --in-place --name Tagline --content "QUIET HANDS. LOUD SECRETS." --font Copperplate --weight 700 --size 3.4% --color '#ffffff'
pixelox transform scale film.pxo --in-place --layer Tagline --size 740x
pixelox transform move film.pxo --in-place --layer Tagline --to 50%,79.2% --anchor c
pixelox text add film.pxo --in-place --name Credits --content $'STUDIO NAME presents a YOUR COMPANY film\nproduced by ANA REYES  executive producer TOM HALE  costume design by JO MARSH\ncasting by SAM OKORO  directed by LEE VARGA  music by THE NIGHT OWLS' --font "Avenir Next Condensed" --weight 500 --size 2.1% --leading 125% --align center --color '#b9b5b0'
pixelox transform scale film.pxo --in-place --layer Credits --size 600x112
pixelox transform move film.pxo --in-place --layer Credits --to 50%,86% --anchor c
pixelox text add film.pxo --in-place --name Site --content "WWW.EXAMPLE.COM" --font "Avenir Next Condensed" --weight 700 --size 2.2% --tracking 40 --color '#8a8580' --at 50%,92% --anchor c
pixelox layer add film.pxo --in-place --name Grain --fill '#808080'
pixelox filter add-noise film.pxo --in-place --layer Grain --amount 10 --monochrome
pixelox layer blend film.pxo --in-place --layer Grain --mode soft-light
pixelox layer opacity film.pxo --in-place --layer Grain --value 35
pixelox export film.pxo -o film.png
```

Change first: the title (initial in `Initial`, the rest in `Title`; keep 4 to 6 letters),
then the three credit lines, then the disc and arm center if your subject's head is not
near 47% W, 28% H (move the `Disc` rect and redraw the arms around the new center). Role words in the
credits are lowercase to stand in for their smaller size.

## 10. Variations and pairings

* Colorways: navy `#1b2435`, steel `#5d6b80`, silver title (`#f2f4f7` to `#7d8794`);
  or oxblood `#3a1114`, brick `#8c3b32`, cream title. Keep the ground flat and the title
  the only metal.
* Swap the spiral for another drawn ground behind the same stack: T-sunburst rays in two
  browns centered on the head, or concentric rings (`pattern.js`, `kind=rings`).
* Birthday, retirement, or team-event parody posters: the format is made for "starring
  you" gifts; the credit block carries the in-jokes.
* Pairs with T-lens-flare on the title's top edge, T-vignette at light strength, and
  T-final-grade to settle the cutout into the browns.

## 11. Pitfalls

* A photographic background. The flat drawn ground is what makes the cutout read as a
  star; a busy photo behind a cutout looks like a holiday snapshot.
* Light clothing: the bottom fade and the title crossing the body both fail on a white
  shirt front. Dark jacket, or darken the lower body with a stronger shade.
* Gold as a flat yellow. It needs the dark-to-light gradient, the bevel, and the dark
  offset shadow; and only one gold element besides the thin reticle.
* Credits set in the display face or at body-text width: they must be tall, narrow, gray,
  and small (cap height 1.6% H), or they compete with the tagline.
* Spiral center off the head. Put the disc behind the face first, then everything else.
