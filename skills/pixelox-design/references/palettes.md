# Palettes

Palettes sampled from the poster set, named by mood. Each row gives the colors in order of
area and how much of the canvas each should cover. Hexes were measured with
`analyze colors` and rounded.

## Rules that make a palette work

* **60 / 30 / 10.** One dominant color (the ground, about 60% of the area), one secondary
  (large shapes or the photo, about 30%), one accent (about 10%: the call to action, one
  word, a badge). Type color comes on top, usually white, near-black, or the accent.
* **Three hues at most**, plus black or white. More colors need a system (T-geo-shapes
  uses five, all at the same saturation, on black).
* **One color shouts.** If the ground is saturated, the photo is neutral or monochrome
  (T-mono-on-color); if the photo is colorful, the ground and type are neutral.
* **Tint your neutrals.** Black with a little of the main hue in it (`#0a0000` on red,
  `#07080c` on blue) and off-whites (`#f4f1ea`, `#f3efe6`) look printed; pure `#000000`
  and `#ffffff` look like defaults. Pure white is fine for type.
* **Contrast check.** Text needs a lightness gap of at least 50 points from its ground.
  White on a mid-tone orange, yellow, or light blue fails: use near-black there.
* **Pull colors from the photo.** `pixelox analyze colors photo.jpg --k 6 --json` lists a
  photo's dominant colors: use its darkest for the ground and its most saturated for the
  accent, and the design will look matched.

## Dark and dramatic

| Name | Colors (area) | Use | Seen in |
| --- | --- | --- | --- |
| Ember | `#351c1c` 25, `#a93a32` 40, `#ff8a2a` 10, `#ffffff` 20, `#0a0000` 5 | revival, concert, sport; white type only | p008 |
| Stage gold | `#15120d` 55, `#5c3a26` 20, `#b98447` 12, `#dab875` 8, `#ffffff` 5 | gala, praise night, awards; gold script accents | p062, p066 |
| Night altar | `#04050b` 60, `#1a1b24` 20, `#946e57` 10, `#e3c8a5` 10 | cinematic, reverent; warm light against cold black | p109 |
| Code heat | `#1c1616` 60, `#532a24` 15, `#fa9113` 8, `#eae7e5` 17 | tech event, launch; one orange glow on charcoal | p073 |
| Ultraviolet | `#010110` 45, `#0b056f` 25, `#533a94` 20, `#e4e3ed` 10 | tech expo, AI, club; thin white type | p003 |
| Deep field | `#020167` 65, `#0031fb` 15, `#1c94f0` 10, `#fafbfa` 10 | corporate bold, fintech; white heavy type | p096 |
| Ice blue | `#040819` 30, `#051c54` 30, `#1447a7` 20, `#bdd7f3` 20 | cold drinks, sport, water energy | p079 |
| Witch teal | `#141923` 35, `#1f333f` 25, `#1b5954` 20, `#44d9b3` 12, `#b9f9ef` 8 | fantasy, thriller, gaming; glowing teal on slate | p116 |
| Neon plum | `#1b0435` 20, `#470743` 25, `#815aa1` 45, `#ff3d9a` 5, `#35e0ff` 5 | nightlife, tech promo; neon line accents | p118 |
| Dusk magenta | `#201b1c` 60, `#7e255d` 15, `#cb3c58` 17, `#f0e2d8` 8 | film night, rooftop events | p115 |
| Mono noir | `#0b0b0b` 70, `#5e5e5e` 5, `#bab7b7` 15, `#ffffff` 10 | Swiss, architecture, photography | p001, p098 |

## Bright and bold

| Name | Colors (area) | Use | Seen in |
| --- | --- | --- | --- |
| Signal red | `#e61a2b` 70, `#ffffff` 28, `#be1c28` 2 | iconic product ads; two colors only | p011 |
| Hot coral | `#f3472b` 55, `#ffd1e1` 25, `#ffffff` 20 | type-only statements; pink type on red-orange | p023 |
| Blaze and black | `#f43706` 50, `#0a070a` 35, `#2f181b` 10, `#b9abad` 5 | bold church and sport; black type on orange | p060, p111 |
| Acid sport | `#e7f719` 35, `#020202` 45, `#51512b` 10, `#ffffff` 10 | streetwear, fitness, youth; lime with black only | p087 |
| Sun and chili | `#f6ec18` 50, `#d63619` 20, `#2c2019` 12, `#e37b19` 10, `#4c7f39` 8 | book covers, festivals; red figure on yellow | p039 |
| Glory gold | `#eeb230` 25, `#fadf41` 25, `#d98413` 20, `#a75e19` 15, `#523615` 15 | sunrise, harvest, celebration; brown type | p045, p057 |
| Candy gradient | `#391b58` 35, `#0d7fc0` 15, `#5cc2de` 20, `#f2a348` 15, `#d14b68` 15 | music, summer party; white type on purple | p051 |
| Poster primaries | `#000000` 60, `#f6c13c` 10, `#ff5c8a` 5, `#12a150` 5, `#3ec5f4` 8, `#ffffff` 12 | exhibitions; flat shapes on black | p036 |
| Editorial red | `#ee353d` 55, `#3b3932` 25, `#dce3be` 10, `#171411` 10 | book and magazine; B&W photo plus one pale accent | p069 |

## Light and calm

| Name | Colors (area) | Use | Seen in |
| --- | --- | --- | --- |
| Dune | `#f0cb87` 50, `#f2d59a` 30, `#ebb66b` 15, `#342818` 5 | minimalist film and book covers | p068 |
| Blush minimal | `#fbcec9` 95, `#281c19` 5 | literary, wedding, quiet luxury; one small dark motif | p021 |
| Rose editorial | `#f7d5cc` 40, `#d6476d` 30, `#2d2429` 20, `#ffffff` 10 | fashion covers; giant pink type behind subject | p067 |
| Sea glass | `#f0f1f1` 35, `#7fbdba` 35, `#4f8381` 15, `#212324` 15 | clean corporate, health, tech services | p009, p041 |
| Kraft and ketchup | `#e3c7aa` 60, `#daa440` 15, `#cf3d0d` 15, `#4a2d1b` 10 | food ads; red type on kraft beige | p004 |
| Heritage green | `#fcf5d9` 50, `#09510e` 28, `#b9a27e` 12, `#3b2108` 10 | traditional, community, eco; green panel on cream | p065 |
| Newsprint and orange | `#c9bdb7` 30, `#100b09` 35, `#dc853d` 25, `#583b2c` 10 | grunge collage; black type, orange paint | p031 |
| Campaign print | `#0d2637` 40, `#6c929e` 25, `#a82c2b` 20, `#e6dcc0` 15 | stencil portraits, retro politics | p025, p099 |
| Retro moon | `#030303` 30, `#fcfdf7` 20, `#9ea19d` 25, `#fbed8d` 20, `#dd3128` 5 | mid-century illustration; yellow sun, red accent | p100 |

## Building your own

1. Start from the brief's one fixed color (a brand color, the product, the photo).
2. Choose the ground: that color at full strength (bold), its darkest shade (dramatic), or
   its palest tint (calm).
3. Choose one accent: the complement for energy (blue ground, orange accent), a neighbor
   on the wheel for harmony (red ground, orange accent), or white or black for restraint.
4. Fix the type colors: one for the headline, one for body. Check contrast.
5. Write the palette down as hexes with area percentages before placing anything, and
   use only those hexes in every command.
