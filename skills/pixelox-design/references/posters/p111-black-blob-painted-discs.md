---
id: p111
title: Red poster with black blob and painted discs
genre: [exhibition, event-flyer, editorial]
mood: [bold, playful, energetic, warm]
style: [geometric, flat, type-led]
format: portrait-a
archetype: A-abstract-title
palette: ['#f33c14', '#010101', '#ffffff', '#d860a0', '#1f8f6a', '#7b2a7a']
techniques: [T-geo-shapes, T-image-in-type, T-fit-width, T-texture-fill]
needs: nothing
---

# p111: Red poster with black blob and painted discs

## 1. Concept

An abstract-art poster in three flat layers: a red-orange page, one big black organic
blob shaped like two commas chasing each other, and a three-letter word in huge white
capitals laid across the blob so that its last letter hangs out over the red. Three round
"windows" in the blob show thick brush-painted color (pink, purple, green, white,
orange). It works because the flat, hard-edged shapes make the small areas of loose
paint look precious, and because white on black on red gives the title two steps of
contrast at once.

## 2. Format and grid

* Portrait A-series (1 : 1.414). Text margins 11% W left, 7.6% W right, 8% H top and
  bottom.
* The blob fills the middle: its upper lobe spans 3 to 81% W, 6 to 88% H; its lower lobe
  spans 48 to 94% W, 40 to 93% H. A curved red channel, about 6% W wide, separates the
  two lobes from 83% W, 40% H down to 48% W, 76% H.
* Title: 12.3 to 88% W, 31.6 to 51.5% H: slightly above center, crossing both lobes and
  the channel.
* Painted discs on a loose diagonal: 37.8% W, 16.3% H (radius 14.4% W); 28% W, 62% H
  (radius 12% W); 70.4% W, 77.5% H (radius 13.3% W).
* Two text blocks in opposite corners on the red: a small paragraph top right (67.5 to
  92.4% W, 8 to 20% H) and a bold statement bottom left (11 to 48% W, 78.6 to 92.3% H),
  which overlaps the blob's lower edge.

## 3. Layer stack

1. Ground: red-orange `#f33c14`.
2. Blob, black `#010101`: two rounded comma shapes drawn with smooth curves, no corners.
   The upper one is widest at 45% H and tapers to a rounded tail at the lower left; the
   lower one is a teardrop with its point at the upper right.
3. Three discs cut into the blob, each filled with paint texture: irregular patches of
   pink `#d860a0`, purple `#7b2a7a`, and green `#1f8f6a` with small dabs of white and
   orange, dry-brush edges, visible bristle direction.
4. Title: three white capitals, cap height 19.9% H, 75.7% W wide. The last letter's
   right half lies on the red ground.
5. Paragraph (top right): nine short justified lines in white.
6. Statement (bottom left): five lines of bold white caps, ragged right.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#f33c14` | ground | 45% |
| `#010101` | blob | 35% |
| `#ffffff` | title and text | 12% |
| `#d860a0` | pink paint | 3% |
| `#1f8f6a` | green paint | 3% |
| `#7b2a7a` | purple paint | 2% |

Red, black, and white do the layout; the three paint colors (a pink, its darker purple,
and a green complement) appear only inside the discs, under 10% of the area.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Title | 1 word, 3 letters | caps | heavy geometric sans (Jost 800, League Spartan 800 / Futura Bold) | 700 | 19.9% | -10 | - | left | `#ffffff` | 12.3 to 88% W, top 31.6% |
| Paragraph | about 45 words in 9 justified lines of 26 to 30 characters | sentence | geometric sans (Jost 400 / Futura Medium) | 500 | 0.9% | 10 | 135% | justify | `#ffffff` | 67.5%, top 7.9% |
| Statement | about 12 words in 5 lines of 13 to 19 characters | caps | heavy geometric sans (Jost 700 / Futura Bold) | 700 | 1.5% | 10 | 131% (pitch 2.75% H) | left | `#ffffff` | 11.1%, top 78.6% |

One family. Title to statement to paragraph is 22 : 1.7 : 1: a giant word and two
footnotes.

## 6. Imagery

No photograph. The paint texture is the only "image": gestural acrylic or gouache
strokes in four or five colors, seen through circular holes. In the recipe it is
generated: `fill noise --as-layer` with three colors at close stop positions makes the
hard-edged color patches, two more sparse noise layers add white and orange dabs, and
`filter oil-paint` gives the base brush direction. A scan or photo of real paint, clipped the same way, is
better if you have one.

## 7. Effects and textures

* **T-geo-shapes**: the blob is one `shape path` of two closed cubic-curve outlines
  (`C` segments only, so there are no corners). The discs are a second path of three
  circles.
* **T-image-in-type**, applied to shapes instead of letters: the paint layers sit
  directly above the disc layer and each is clipped with `layer clip`; several
  consecutive clipped layers share the same base.
* Paint: `fill noise --as-layer` (`--scale 22%`, `--warp 80`) with positioned `--stops`:
  green up to 0.40, pink from 0.44 to 0.58, purple from 0.62. The narrow gaps between the
  positions are the hard edges of the patches, and pink gets the widest band. Then `filter
  oil-paint --brush 5 --sharpness 90 --impasto 60` on this base layer.
* Dabs: two more noise layers whose stops go from `transparent` to white or orange between
  0.63 and 0.67, so only the highest part of the noise shows (about a sixth of the area).
  Raise both positions for fewer dabs. `--bias` cannot thin a hard-edged layer this far;
  the stop positions can.
* **T-fit-width** on the title (818 px).
* Flat everywhere else: no shadows, no grain.
* **T-texture-fill** in [techniques.md](../techniques.md) is the tested library version of the generated texture clipped into the type or shapes; start from it when reusing this idea.

## 8. Hierarchy and eye path

1. The white title on black.
2. The top painted disc (the brightest mix of colors, just above the title), then down
   the diagonal to the other two.
3. The bold statement at the bottom left.
4. The small paragraph at the top right.

The blob's S-curve channel leads from the title's last letter down to the third disc.

## 9. Build recipe

Nothing to supply. Canvas 1080x1528.

```bash
pixelox new -o blob.pxo --size 1080x1528 --fill '#f33c14'
pixelox shape path blob.pxo --in-place --name Blob --d 'M 296 94 C 514 0 732 94 748 374 C 756 499 818 545 803 639 C 779 748 701 795 608 857 C 514 919 468 997 444 1091 C 413 1216 374 1309 265 1247 C 125 1184 31 997 62 810 C 94 655 125 592 140 468 C 156 312 156 156 296 94 Z M 935 608 C 1029 732 1013 873 997 997 C 982 1184 997 1403 810 1426 C 655 1442 499 1340 522 1122 C 530 1060 600 966 670 919 C 779 857 842 810 873 686 C 881 647 904 616 935 608 Z' --fill '#010101'
pixelox shape path blob.pxo --in-place --name Discs --d 'M 252 249 A 156 156 0 1 1 564 249 A 156 156 0 1 1 252 249 Z M 175 948 A 129 129 0 1 1 433 948 A 129 129 0 1 1 175 948 Z M 618 1184 A 143 143 0 1 1 904 1184 A 143 143 0 1 1 618 1184 Z' --fill '#7b2a7a'
pixelox fill noise blob.pxo --in-place --as-layer --name Paint --seed 4 --scale 22% --octaves 5 --warp 80 --stops '#1f8f6a@0.40,#d860a0@0.44,#d860a0@0.58,#7b2a7a@0.62'
pixelox filter oil-paint blob.pxo --in-place --layer Paint --brush 5 --sharpness 90 --impasto 60
pixelox layer clip blob.pxo --in-place --layer Paint
pixelox fill noise blob.pxo --in-place --as-layer --name DabsWhite --seed 19 --scale 12% --octaves 4 --warp 30 --stops 'transparent@0.63,#ffffff@0.66'
pixelox layer clip blob.pxo --in-place --layer DabsWhite
pixelox fill noise blob.pxo --in-place --as-layer --name DabsOrange --seed 23 --scale 12% --octaves 4 --warp 30 --stops 'transparent@0.64,#f37a1f@0.67'
pixelox layer clip blob.pxo --in-place --layer DabsOrange
pixelox text add blob.pxo --in-place --name Title --content "POP" --font Futura --weight 700 --fit-width 818 --tracking=-10 --color '#ffffff' --at 12.3%,31.6% --anchor-to ink
pixelox text add blob.pxo --in-place --name Para --content "A season of new painting from the studios on the east side: loud color, thick paint, and work made fast. Open every evening for two weeks, with the artists in the room on the first and last nights. Entry is free and everyone is welcome." --font Futura --weight 500 --size 1.25% --tracking 10 --leading 135% --align justify --box 270x320 --color '#ffffff' --at 67.5%,7.9% --anchor tl
pixelox text add blob.pxo --in-place --name Statement --content $'COLOR IS A VERB\nAND THIS SHOW IS\nA WHOLE SENTENCE\nSHOUTED ACROSS\nONE ROOM' --font Futura --weight 700 --size 2.1% --tracking 10 --leading 131% --color '#ffffff' --at 11.1%,78.6% --anchor tl
pixelox export blob.pxo -o blob.png
```

Change first: the title (3 letters; 4 at most), then the blob: edit the `C` control points
or replace it with your own two-lobe outline, keeping a channel of ground color between
the lobes. Then the paint colors (the three colors in the `--stops` of `Paint`) and its
`--seed` for a different pattern in the discs.

## 10. Variations and pairings

* Ground swaps with the same black blob: cobalt `#1f3fd6`, yellow `#ffd400` (then title
  stays white, text on the ground turns black), emerald `#0f9d58`.
* Show a photo through the discs instead of paint (place it above the Discs layer and
  clip it): portraits for a line-up, products for a sale.
* Invert: white blob, black title, colored ground.
* Sibling layout: see p112, which uses the same paint texture and type on an orange
  ground with flat color discs and triangles instead of a blob.
* Suits exhibitions, festivals, club nights, book covers, record sleeves. Pairs with
  T-paper-grain (amount 20) for a screen-print feel and T-vertical-type for
  a date along the edge.

## 11. Pitfalls

* A blob with corners or straight segments looks like a broken polygon. Use only cubic
  curves and keep every handle long (at least a quarter of the segment length).
* Too much paint: if the texture covers more than 10 to 12% of the page it stops being a
  jewel and starts fighting the red. Three discs, each under 15% W in radius.
* Paint without brush direction is just a blurry gradient. Keep neighboring stop positions
  close (0.40 and 0.44) for hard patches and run `filter oil-paint` with `--sharpness` 60
  or more.
* The title fully inside the blob loses the layered effect. Let one letter cross the
  blob's edge or the channel.
* Body text on the black blob in the red ground color vibrates and is hard to read;
  keep all text white.
