---
id: p011
title: White bottle afloat on wave ribbons
genre: [drink-ad, product-ad]
mood: [minimal, bold, fresh, playful]
style: [flat, minimal, illustration]
format: portrait-a
archetype: A-centered-title
palette: ['#e61a2b', '#ffffff', '#be1c28']
techniques: [T-waves, T-silhouette, T-script-overlap]
needs: product-cutout
---

# p011: White bottle afloat on wave ribbons

## 1. Concept

A two-color soft-drink poster: a flat white bottle silhouette bobs in white ribbon "waves"
on a solid red field, under one tiny lowercase line that turns the picture into a pun (the
bottle is a message afloat at sea). It works because a ribbon shape from the brand's
identity becomes the water, so the whole scene is built from only two inks and the
product's outline.

## 2. Format and grid

* Portrait A-series (1 : 1.414). Symmetric around the vertical axis at 50% W; the bottle
  leans 1.5% W left of it, which makes it feel afloat.
* Zones: empty red 0 to 20% H; tagline at 23% H; empty red 26 to 45% H; bottle 45.7 to
  74% H; wave ribbons 57 to 86% H (they cross the canvas edge to edge); white floor 86 to
  100% H carrying the brand disc at 92.3% H.
* More than half of the canvas is flat red with nothing on it. The tagline sits exactly
  halfway between the top edge and the bottle's cap.

## 3. Layer stack

1. Ground: flat red `#e61a2b`, full canvas. No gradient, no texture.
2. Back ribbons: two white bands with long, slow curves entering from the right edge at
   57% H and the left edge at 59% H, crossing behind the bottle; red shows between them.
3. Bottle: a pure white silhouette of a curvy glass bottle, `39%,45.7%,20% x 28.3%`,
   upright. No shading at all: only its outline tells you what it is.
4. Mark on the bottle: the brand's script wordmark in the ground red, 17% W wide, across
   the widest part of the bottle at 65 to 72% H.
5. Front ribbons: red and white bands, starting at 74% H, that pass in front of the
   bottle's base so it sits in the water. Each band tapers to a point at one side. The last
   band is white and runs to the bottom edge.
6. Tagline: white, centered at 23% H.
7. Brand disc: red circle, diameter 11.6% W, centered at 50% W, 92.3% H, on the white floor,
   with the script wordmark in white inside it (wordmark 9.5% W wide).

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#e61a2b` | ground, ribbons between the white bands, disc, mark on the bottle | 71% |
| `#ffffff` | bottle, ribbons, floor, tagline | 28% |
| `#be1c28` | none in the flat art; anti-aliased edges only | 1% |

Two flat inks. Every element is one of the two, which is why the red mark on the white
bottle and the white mark on the red disc read as the same object seen twice.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Tagline | 4 short words, a familiar phrase that becomes a pun next to the picture | lower | light neo-grotesque (Inter 300, Roboto 300 / Helvetica Neue) | 300 | 1.6% | 0 | - | center | `#ffffff` | 50%, 23% |
| Mark on bottle | `[BRAND]` script wordmark | title | formal script (Great Vibes / Snell Roundhand) | 900 | 3.5% | 0 | - | center | `#e61a2b` | 48.5%, 68.5% |
| Mark in disc | `[BRAND]` script wordmark | title | same script | 900 | 1.6% | 0 | - | center | `#ffffff` | 50%, 92.3% |

One whisper-quiet sans line and the brand's script. The tagline is 29% W wide: small
enough that the viewer sees the bottle first and reads the line second, as a caption.

## 6. Imagery

No photograph. The bottle is a silhouette (T-silhouette with a flat white fill) of a product
whose outline is famous enough to be recognized without detail; any product with a
distinctive contour works (a sauce bottle, a perfume flask, a sneaker in profile). The
ribbons are four to six flat bands with slow S-curves, each fully crossing the canvas, that
alternate white and red and thin to points: they are the "sea". Two bands lie behind the
product and the rest in front of its base.

## 7. Effects and textures

* **T-waves**, two renders: a back set (2 bands: one thin white ribbon, then red to the
  bottom; top 0.6) placed under the bottle and a front set (4 bands, red first, last one
  white, top 0.765) placed over it. `amp` 0.022 to 0.03, `freq` 0.8, `chaos` 0.8 for long
  unequal curves.
* **T-silhouette**: `fx overlay --color '#ffffff'` on the product cutout.
* **T-script-overlap** in spirit: a script mark laid across the product in the ground
  color. No shadow.
* Nothing else. No grain, no shadow, no gradient: the flatness is the style.

## 8. Hierarchy and eye path

1. The white bottle (largest white shape with a recognizable outline, just below center).
2. Up to the tagline, the only thing in the empty red upper half.
3. Down along the ribbons to the disc at the bottom.

Cap heights: mark on bottle 3.5 : tagline 1.6 : disc mark 1.6. The bottle is 28% H tall, 18
times the tagline's cap height; the hierarchy is picture first, words far behind.

## 9. Build recipe

`product.png` is your image here: a cutout with a clear outline (only its shape is used).

```bash
pixelox render js scripts/waves.js -o back.png --size 1080x1528 --seed 8 --param colors='#ffffff,#e61a2b' --param count=2 --param top=0.6 --param bottom=0.65 --param amp=0.03 --param freq=0.8 --param chaos=0.8
pixelox render js scripts/waves.js -o front.png --size 1080x1528 --seed 5 --param colors='#e61a2b,#ffffff,#e61a2b,#ffffff' --param count=4 --param top=0.765 --param bottom=0.88 --param amp=0.022 --param freq=0.8 --param chaos=0.8
pixelox new -o ad.pxo --size 1080x1528 --fill '#e61a2b'
pixelox comp place ad.pxo back.png --in-place --as BackWaves --fit cover
pixelox comp place ad.pxo product.png --in-place --as Bottle --size x500 --at 48.5%,77.5% --anchor b
pixelox fx overlay ad.pxo --in-place --layer Bottle --color '#ffffff'
pixelox text add ad.pxo --in-place --name Mark --content "Brand" --font "Snell Roundhand" --weight 900 --size 5.4% --color '#e61a2b' --at 48.5%,68.5% --anchor c
pixelox transform scale ad.pxo --in-place --layer Mark --size 96x
pixelox comp place ad.pxo front.png --in-place --as FrontWaves --fit cover
pixelox text add ad.pxo --in-place --name Tagline --content "a letter from summer" --font "Helvetica Neue" --weight 300 --size 2.25% --color '#ffffff' --at 50%,23% --anchor c
pixelox shape ellipse ad.pxo --in-place --name Disc --rect 477,1348,126x126 --fill '#e61a2b'
pixelox text add ad.pxo --in-place --name DiscMark --content "Brand" --font "Snell Roundhand" --weight 900 --size 2.6% --color '#ffffff' --at 540,1411 --anchor c
pixelox transform scale ad.pxo --in-place --layer DiscMark --size 100x
pixelox export ad.pxo -o ad.png
```

Change first: the tagline (keep it under 22 characters), then the two wave seeds until one
white band passes behind the product and one red band cuts its base; then widen `Mark` to
85% of your product's width (96 px fits the narrow stand-in bottle; a wide bottle takes
180 px).

## 10. Variations and pairings

* Any two-ink pair with strong contrast: navy `#0b2a6f` and white, black and yellow
  `#ffd400`, forest `#0b3d2e` and cream `#f3ead7`.
* Change the scene by changing the bands: jagged peaks for mountains, concentric arcs for a
  sunrise (T-sunburst with `fade=0`), vertical bands for rain. The product silhouette stays
  white and flat.
* Summer campaign, sustainability message (the bottle returning to shore), anniversary
  poster, a series with one new pun per poster.
* Square: tagline at 14% H, bottle 38 to 70% H, ribbons 55 to 84% H, disc at 91% H.
* Pairs with T-paper-grain at low strength for a screen-print feel; do not add anything
  else.

## 11. Pitfalls

* Shading or a drop shadow on the bottle. The moment it has volume, the flat sea stops
  being water. Keep it one flat color.
* A product with an anonymous outline (a plain can, a box) is unreadable as a silhouette.
  Use this only when the shape alone identifies the product; otherwise keep the label art.
* Ribbons that are parallel stripes. They need to cross each other's paths and taper;
  raise `chaos`, lower `freq`, and try other seeds.
* A big, bold tagline. At more than 2.5% H cap height it fights the bottle; the joke works
  because the line is small.
* The base of the bottle visible on top of every ribbon: it then stands on the waves
  instead of floating in them. One band must overlap the lowest 5 to 10% of the bottle.
* Rebuilding a known brand's trade dress. A white script wordmark and ribbon on this exact red
  is one company's signature. Take the idea (what the object, the mark, and the ground do
  to each other) and build it with your own brand's colors, lettering, and shapes.
