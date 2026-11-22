---
id: p116
title: Glowing sigil fantasy novel cover
genre: [book-cover]
mood: [dark, mysterious, moody, dramatic]
style: [illustration, neon]
format: portrait-2x3
archetype: A-centered-title
palette: ['#0f131b', '#1a5351', '#1ca489', '#29e3b9', '#b5f5ec']
techniques: [T-smoke, T-neon-glow, T-image-in-type, T-texture-fill, T-vignette]
needs: nothing
---

# p116: Glowing sigil fantasy novel cover

## 1. Concept

A dark-fantasy novel cover built from one light source: a pale mint emblem (a spike, two
crescents, and crossing blades) and a pair of slit-pupil cat eyes glow out of blue-black
smoke, and the two-line title below is lit with the same mint glow. It works because
everything that shines is one hue (teal to mint) and everything else is nearly black, so
the eyes, the emblem, and the title read as a single lit object in a dark room.

## 2. Format and grid

* Portrait 2:3 (the source is a little narrower, 1 : 1.545). No visible margins: the art
  bleeds; type keeps 11% W clear on each side.
* Strict vertical symmetry on the 50% W axis: author, emblem, both eyes (mirrored at 21%
  and 79% W), title, and tagline are all centered.
* Zones: author 6 to 9% H; emblem and eyes 16 to 62% H (the picture, 46% of the height);
  title 63.5 to 88% H; tagline at 93.7% H.
* The emblem is wider than the title (96% W against 77% W), so the silhouette of the whole
  cover is a wide top tapering to a narrower block: a shield shape.

## 3. Layer stack

1. Ground: blue-black `#0f131b`, full canvas.
2. Smoke: large soft swirls from blue-black through slate `#182838` to dark teal
   `#1a5351`; the suggestion of a face is made only by where the smoke is darker.
3. Aura: a radial teal glow `#128882` centered at 50% W, 45% H, radius about 33% H, fading
   to nothing; it is the "light" the emblem casts on the smoke.
4. Floor glow: teal `#1d6c5f` rising from the bottom edge, 0% at 76% H to about 65% at the
   bottom, strongest in the lower right.
5. Vignette: blue-black corners, clear in the middle 40%.
6. Eyes: two almond shapes, each 17% W x 8% H, centers at 21%, 46.5% and 79%, 46.5%; fill
   from light green `#8dffa0` in the middle to `#0fb98c` at the rim, with a wide green
   glow; a vertical slit pupil 1.2% W x 5.5% H, near-black green, in each.
7. Emblem, pale mint `#c4f8ef`, one flat shape made of six blades: a spike (apex 49.5%,
   16.5%; base 45 to 56.5% W at 30% H); two legs that continue the spike downward and
   outward to tips at 22.5%, 61.5% and 77%, 61.5%; a crescent hugging the outside of each
   eye (the left one from 14%, 31.5% out to 2% W and back to 16%, 56%; the right one
   mirrored, 88 to 98% W); one thin diagonal blade from the left crescent's top tip down
   to 64%, 51%. Tight mint glow `#22dcb6` around all of it.
8. Smoke veil: one soft dark form, 40 to 94% W x 27.5 to 39.5% H, in front of the emblem,
   so the spike is cut off from its legs and the emblem sits inside the smoke, not on it.
9. Lightning: two thin jagged white-mint lines with a glow, one from 40%, 10.5% down to
   56%, 30%, one from 70%, 44% down to 54%, 64%.
10. Author name: white, centered, cap top 6% H, 65% W wide.
11. Title: two centered lines, mint-white with faint marble veins inside the letters and a
    wide teal glow; cap tops at 63.5% and 77% H.
12. Tagline: white, centered at 93.7% H.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#0f131b` | blue-black ground, corners | 35% |
| `#1a5351` | dark teal smoke and aura edge | 35% |
| `#1ca489` | mid teal: glows around emblem and title | 15% |
| `#29e3b9` | bright green-teal: eyes, glow cores | 5% |
| `#b5f5ec` | pale mint: emblem, title fill | 10% |
| `#f2f6f5` | author and tagline | under 2% |

Monochrome teal on blue-black. Value does all the work: the ground stays under 15%
lightness, the glow sits near 40%, and only the emblem and the letters go above 85%.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Author | `[AUTHOR NAME]`, 2 words, 10 to 12 letters | caps | inscriptional serif (Cinzel, Cormorant SC / Big Caslon) | 400 | 3% | 170 | - | center | `#f2f6f5` | 50%, cap top 6%; 65% W wide |
| Title line 1 | the article "THE" | caps | display inscriptional serif with sharp wedge serifs (Cinzel Decorative, Cinzel 700 / Big Caslon) | 500 to 700 | 11% | 0 | line pitch 13.5% H | center | `#d6fbf5` with teal glow | 50%, cap top 63.5%; 46% W wide |
| Title line 2 | `[TITLE]`, one 5-letter noun | caps | same | same | 11% | 0 | - | center | same | 50%, cap top 77%; 77% W wide |
| Tagline | "A NOVEL" | caps | same serif, regular | 400 | 1.8% | 220 | - | center | `#f2f6f5` | 50%, 93.7% |

One serif family throughout. Both title lines share one cap height, so the short article
is narrow and the noun is wide: the title is a stepped block, not a fitted rectangle.

## 6. Imagery

A digital painting, not a photo: blue-black smoke that almost forms an animal's face, two
glowing green cat eyes with slit pupils, and a flat pale emblem that looks like a rune or
a maker's mark. There is no outline drawing; forms exist only as light against dark. The
build draws the eyes and the emblem with `shape path` and makes the smoke with **T-smoke**
settings, so no image is needed. To use your own painted creature or symbol instead, place
it as `illustration.png` between the aura and the smoke veil and give it the same glow.

## 7. Effects and textures

* **T-smoke** as an opaque ground: one `fill noise --as-layer` line with the three dark
  colors as `--stops`, `--scale 60%` for large forms, and `--warp 120` for the swirls.
* **T-neon-glow** on the emblem, the eyes, the lightning, and the title: a near-white fill
  plus `fx outer-glow` in saturated teal (blur 44 to 46 px, spread 6 to 8, opacity 95);
  the emblem also takes `fx inner-glow` so its edges tint toward teal.
* **T-texture-fill**: a `fill noise --kind ridged` layer (`--warp 120`, stops `transparent`
  to teal `#58d9c4`, 40% opacity) clipped to the title gives the letters the look of
  veined stone. It is the same clip as **T-image-in-type**, with a generated texture.
* **T-vignette** under the type, color `#06070c`, starting at 0.4.
* The smoke veil in front of the emblem is a blurred dark ellipse (sigma 28).

## 8. Hierarchy and eye path

1. The two eyes: the most saturated spots, placed where a face is expected.
2. The emblem around them, then down its two legs, which point at the title.
3. The title (cap height 11% H, the lightest large area).
4. Author at the top, tagline at the bottom.

Cap heights: title 11 : author 3 : tagline 1.8, about 6 : 1.7 : 1. The picture and the
title are almost equal in visual weight; the author line is deliberately quiet.

## 9. Build recipe

No inputs. The emblem is one `shape path` with six sub-paths on a 1080x1620 canvas; redraw
it for your own story (keep it symmetric in weight, 90% W wide or more, and let the eyes
sit in its two openings).

```bash
pixelox new -o cover.pxo --size 1080x1620 --fill '#0b0d15'
pixelox fill noise cover.pxo --in-place --as-layer --name Smoke --seed 12 --scale 60% --octaves 5 --warp 120 --contrast 15 --stops '#0b0d15,#141d2b,#1c4c4f'
pixelox layer add cover.pxo --in-place --name Aura
pixelox fill gradient cover.pxo --in-place --layer Aura --kind radial --from 50%,45% --to 50%,78% --stops '#14a493bf@0,#0f6f6a66@0.5,#0f6f6a00@1'
pixelox layer add cover.pxo --in-place --name Floor
pixelox fill gradient cover.pxo --in-place --layer Floor --from 0,76% --to 0,100% --stops '#1d6c5f00@0,#1d6c5fa6@1'
pixelox layer add cover.pxo --in-place --name Vignette
pixelox fill gradient cover.pxo --in-place --layer Vignette --kind radial --from 50%,48% --to 50%,112% --stops '#06070c00@0.4,#06070ce6@1'
pixelox shape path cover.pxo --in-place --name EyeL --d 'M 132 754 Q 226 650 322 742 Q 236 842 132 754 Z' --fill '#1be1a8'
pixelox fx overlay cover.pxo --in-place --layer EyeL --style radial --stops '#8dffa0@0,#2cf09a@0.5,#0fb98c@1'
pixelox fx outer-glow cover.pxo --in-place --layer EyeL --blur 46 --color '#19e0a6' --opacity 85
pixelox shape path cover.pxo --in-place --name EyeR --d 'M 948 754 Q 854 650 758 742 Q 844 842 948 754 Z' --fill '#1be1a8'
pixelox fx copy cover.pxo --in-place --from EyeL --to EyeR
pixelox shape path cover.pxo --in-place --name Pupils --d 'M 232 704 Q 244 750 232 796 Q 220 750 232 704 Z M 858 704 Q 870 750 858 796 Q 846 750 858 704 Z' --fill '#06231f'
pixelox shape path cover.pxo --in-place --name Sigil --d 'M 535 267 L 610 478 L 486 502 Z M 445 600 L 508 632 L 243 996 Z M 594 648 L 640 630 L 832 996 Z M 151 510 A 211 211 0 0 0 173 907 A 265 265 0 0 1 151 510 Z M 151 510 L 700 800 L 668 838 Z M 950 580 A 215 215 0 0 1 925 962 L 745 826 L 772 800 L 915 890 A 275 275 0 0 0 950 580 Z' --fill '#c4f8ef'
pixelox fx outer-glow cover.pxo --in-place --layer Sigil --blur 44 --spread 6 --color '#22dcb6' --opacity 95
pixelox fx inner-glow cover.pxo --in-place --layer Sigil --blur 16 --color '#5fe9d0' --opacity 55
pixelox shape ellipse cover.pxo --in-place --name Veil --rect 40%,27.5%,54%x12% --fill '#102a33'
pixelox filter gaussian cover.pxo --in-place --layer Veil --sigma 28
pixelox shape path cover.pxo --in-place --name Bolts --d 'M 432 170 L 448 262 L 492 300 L 520 352 L 566 384 L 610 486 M 760 714 L 742 800 L 700 846 L 716 900 L 640 960 L 660 1010 L 580 1040' --fill none --stroke '#dcfff7' --stroke-width 3 --join miter
pixelox fx outer-glow cover.pxo --in-place --layer Bolts --blur 14 --color '#39ecc9' --opacity 90
pixelox text add cover.pxo --in-place --name Author --content "MARA ELLISON" --font "Big Caslon" --fit-width 700 --tracking 170 --color '#f2f6f5' --at 50%,6% --anchor t --anchor-to ink
pixelox text add cover.pxo --in-place --name Title --content $'THE\nSIGIL' --font "Big Caslon" --size 15% --leading 90% --align center --color '#d6fbf5' --at 50%,63.5% --anchor t --anchor-to ink
pixelox fill noise cover.pxo --in-place --as-layer --name Marble --seed 31 --kind ridged --scale 30% --warp 120 --stops 'transparent,#58d9c4' --opacity 40
pixelox layer clip cover.pxo --in-place --layer Marble
pixelox fx outer-glow cover.pxo --in-place --layer Title --blur 46 --spread 8 --color '#1fd6b2' --opacity 95
pixelox text add cover.pxo --in-place --name Tag --content "A NOVEL" --font "Big Caslon" --size 2.5% --tracking 220 --color '#f2f6f5' --at 50%,93.7% --anchor c
pixelox export cover.pxo -o cover.png
```

Change first: the title words (font size 15% gives the 11% cap height; a second line with
wide letters such as M, O, or W needs `--fit-width 830` in place of `--size 15%` on the
Title, which sizes both lines together so the wider one spans 77% W), then the glow hue:
swap every teal and mint for one other hue family and leave the ground alone. A new
`--seed` on `Smoke` moves the swirls.

## 10. Variations and pairings

* Hue swaps, ground unchanged: ember (`#ffd9a8` fill, `#ff7a1a` glow, eyes `#ffb347`) for
  a dragon or fire story; violet (`#e9d6ff` fill, `#8a4dff` glow); ice (`#dff4ff` fill,
  `#3aa0ff` glow). One glow hue only.
* Replace the eyes with a single centered motif (a moon disc, a key, a crown) inside the
  emblem for a series: same emblem, new object per volume.
* Film-poster or game-cover version on 1080x1620: drop the tagline, add a credit line at
  96% H in the author's face at cap height 1%.
* Square audiobook tile: keep eyes and emblem in the top 62%, set the title on one line.
* Pairs with T-smoke wisps in `screen` mode above the title for mist over the letters,
  T-gradient-type (mint to teal, bottom to top) instead of the marble fill, and
  T-paper-grain at amount 10 to hide banding in the aura.

## 11. Pitfalls

* Saturated letters. A title filled with the glow color looks flat and dim: the fill must
  be almost white (`#d6fbf5`), and only the glow carries the teal.
* A second hue (red eyes, gold title). It breaks the one-light-source idea; keep every
  bright thing in one hue family.
* A ground that is merely dark gray or dark teal all over. The corners must fall to
  blue-black (lightness under 8%) or the glows have nothing to shine against; keep the
  vignette, and keep it below the type so the author line stays white.
* An emblem with thin strokes. Blades thinner than 2% W vanish inside their own glow; make
  them taper from 4 to 5% W to a point.
* Glow blur too wide (over 6% W): the title turns to fog. Use 4 to 4.5% W with a spread of
  6 to 8 so the edge stays crisp.
