---
id: p070
title: Teal moon and desert figure cover
genre: [book-cover, film-poster]
mood: [dramatic, mysterious, warm, retro]
style: [illustration, retro, flat]
format: portrait-2x3
archetype: A-centered-title
palette: ['#ee9a1e', '#552301', '#af4b1f', '#019c87', '#782b11', '#d9c24e']
techniques: [T-silhouette, T-gradient-type, T-fit-width, T-photo-fade, T-smoke, T-paper-grain]
needs: subject-cutout
---

# p070: Teal moon and desert figure cover

## 1. Concept

A deluxe-edition science-fiction cover built on one color clash: a burnt-orange desert
sky filled almost edge to edge by a giant moon that fades from teal at the top through
green and gold into the orange haze, with a lone cloaked figure standing in front of it
as a rust-brown silhouette. The title sits below, on dark brown, in huge inline art-deco
capitals that repeat the moon's teal-to-gold fade; the author's name closes the sheet in
pale amber. Everything is buried under coarse sand-like grain. It works because the
palette is only two families (orange-rust and teal) and the three key shapes (disc,
figure, title) are stacked dead center, largest to smallest.

## 2. Format and grid

* Portrait 2:3. Strict center axis at 50% W; margins 7% W for the title.
* Stack: moon 10 to 57% H (71% W across, centered at 52% W, 33.7% H); figure 25 to 69.5%
  H, 35% W wide, standing on the axis so the head is at the moon's center; title 71 to
  88.4% H; author at 94% H.
* The sky is a vertical ramp in three zones: dark red-brown at the top (0 to 10% H),
  brightening through rust to a glowing orange-gold band at 44 to 64% H (the horizon
  haze, behind the figure's legs), then falling quickly to dark brown from 76% H to the
  bottom. The title and author therefore sit on the dark foot of the sheet.
* The figure's cloak overlaps the lower third of the moon and dissolves into the haze
  band; its tatters stop just above the title.

## 3. Layer stack

1. Sky: vertical gradient `#701c01` (top) to `#af4b1f` (19% H) to `#c56021` (32%) to
   `#ee9a1e` (44%) to `#f4a521` (55%) to `#d77a27` (69%) to `#9a4c15` (76%) to `#6b2b03`
   (84%) to `#552301` (bottom).
2. Sand streaks: faint, long horizontal wisps of lighter amber across the bright band.
3. Moon: a disc with a vertical fade, teal `#02917d` at its top, green `#82ab51` one third
   down, gold `#e0a030` two thirds down, and the sky's orange at the bottom, so the lower
   edge of the disc is lost in the haze. Fine cracks and contour lines in darker teal
   cover its surface.
4. Figure: a standing person in a long ragged cloak, three-quarter view, hair blown to
   one side; a flat silhouette graded from rust `#a04a1e` at the head to dark `#5a2303` at
   the hem, with two tiny pale glints for eyes. The hem breaks into windblown shreds.
5. Haze: a band of orange-gold over the figure's feet and the moon's lower edge, 58 to
   72% H.
6. Title: 4 capitals in a condensed inline display face (each stroke carries a thin
   parallel line inside it), 87% W wide, cap height 17.3% H; fade from teal `#019c87` at
   the top of the letters to sand-gold `#b69b58` at the bottom.
7. Author: bold tracked caps in pale amber `#f0b862`, 61% W wide, cap height 3.6% H, at
   94% H.
8. Grain: coarse, strong noise over everything, with a slight horizontal smear.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#ee9a1e` | sky, bright band | 28% |
| `#552301` | sky foot and top edge | 22% |
| `#af4b1f` | sky rust, upper third | 15% |
| `#019c87` | moon teal, top of the title | 18% |
| `#782b11` | figure | 12% |
| `#d9c24e` | gold: lower moon, lower title, author (paler) | 5% |

Complementary pair, orange against teal, with gold as the bridge between them. Teal is
used only for the moon and the title; the author's name takes the gold side.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Title | `[BOOK TITLE]`, 1 word, 4 letters | caps | condensed art-deco inline display (Bungee Inline, Monoton, Limelight / Phosphate) | 400 | 17.3% | 20 | - | center, 87% W wide | `#019c87` to `#b69b58`, top to bottom | 50%, 71 to 88.4% |
| Author | `[AUTHOR NAME]`, 2 words, 12 letters | caps | geometric sans (Montserrat 700 / Avenir Next) | 700 | 3.6% | 150 | - | center, 61% W wide | `#f0b862` | 50%, 94% |

Two lines of type only. Ratio 4.8 : 1. The author line is large for a cover (3.6% H)
because the name is the selling point of a classic.

## 6. Imagery

An illustration in flat shapes with printed texture: a disc and a silhouette. For a
build, the figure is any full-length or three-quarter cutout with a strong outline (cloak,
coat, hat, staff), turned into a graded silhouette with **T-silhouette** (`fx overlay
--stops` from dark at the bottom to rust at the top). The moon is a `shape ellipse` with a
four-stop gradient overlay whose last color equals the sky behind it. Place the figure's
head at the disc's center: the disc then works as a halo.

## 7. Effects and textures

* Sky: `fill gradient` with nine stops sampled down the sheet.
* Streaks: **T-smoke** clouds with `stretch=0.2` (flat bands), amber, screen, 30%.
* Moon surface: a high-contrast `clouds.js` render (`contrast=2.6`, `warp=1.5`) clipped to
  the disc in multiply at 25%.
* **T-silhouette** with a gradient on the figure; **T-photo-fade** as a haze band
  (orange-gold from alpha 0 at 58% H to 85% at 67% H and back to 0 at 72% H).
* **T-gradient-type** on the title (`--angle 90`, the bottom color first).
* **T-paper-grain**, heavy: amount 34, overlay 55%, then `filter motion-blur --angle 0
  --distance 3` on the grain for the sand-blast streak.
* **T-fit-width**: headline lines are set to exact widths with `transform scale` in the recipe.

## 8. Hierarchy and eye path

1. The moon (largest shape, the only cool color in a hot field).
2. The figure in front of it, head at its center.
3. Down the cloak to the title, whose colors answer the moon's.
4. The author.

Sizes: moon 47% H across, figure 44% H tall, title cap height 17.3% H, author 3.6% H.

## 9. Build recipe

`subject.png` is your image here: a cutout with a clear outline; it will be flattened to a
silhouette.

```bash
pixelox render js scripts/clouds.js -o streaks.png --size 1080x1620 --seed 8 --param lo=transparent --param hi='#f7c860' --param scale=3 --param warp=0.6 --param stretch=0.2 --param bias=-0.15
pixelox render js scripts/clouds.js -o surface.png --size 1080x1620 --seed 12 --param lo='#0b3d3a' --param hi='#ffffff' --param scale=5 --param octaves=6 --param warp=1.5 --param contrast=2.6
pixelox new -o saga.pxo --size 1080x1620 --fill '#ee9a1e'
pixelox layer add saga.pxo --in-place --name Sky
pixelox fill gradient saga.pxo --in-place --layer Sky --from 0,0 --to 0,100% --stops '#701c01@0,#782502@0.08,#af4b1f@0.19,#c56021@0.32,#ee9a1e@0.44,#f4a521@0.55,#d77a27@0.69,#9a4c15@0.76,#6b2b03@0.84,#552301@1'
pixelox comp place saga.pxo streaks.png --in-place --as Streaks --fit cover
pixelox layer blend saga.pxo --in-place --layer Streaks --mode screen
pixelox layer opacity saga.pxo --in-place --layer Streaks --value 30
pixelox shape ellipse saga.pxo --in-place --name Moon --rect 178,162,767x767 --fill '#429370'
pixelox fx overlay saga.pxo --in-place --layer Moon --stops '#f09c20@0,#e0a030@0.3,#82ab51@0.62,#02917d@1' --angle 90
pixelox comp place saga.pxo surface.png --in-place --as Surface --fit cover
pixelox layer clip saga.pxo --in-place --layer Surface
pixelox layer blend saga.pxo --in-place --layer Surface --mode multiply
pixelox layer opacity saga.pxo --in-place --layer Surface --value 25
pixelox comp place saga.pxo subject.png --in-place --as Figure --size x720 --at 48%,69.5% --anchor b
pixelox fx overlay saga.pxo --in-place --layer Figure --stops '#5a2303@0,#a04a1e@1' --angle 90
pixelox layer add saga.pxo --in-place --name Haze
pixelox fill gradient saga.pxo --in-place --layer Haze --from 0,58% --to 0,72% --stops '#ee9a1e00@0,#ee9a1ed9@0.65,#d77a2700@1'
pixelox text add saga.pxo --in-place --name Title --content "MESA" --font Phosphate --size 24% --tracking 20 --color '#019c87'
pixelox transform scale saga.pxo --in-place --layer Title --size 940x
pixelox transform move saga.pxo --in-place --layer Title --to 50%,71% --anchor t
pixelox fx overlay saga.pxo --in-place --layer Title --stops '#b69b58@0,#5fa37a@0.5,#019c87@1' --angle 90
pixelox text add saga.pxo --in-place --name Author --content "AUTHOR NAME" --font "Avenir Next" --weight 700 --size 4.9% --tracking 150 --color '#f0b862'
pixelox transform scale saga.pxo --in-place --layer Author --size 660x
pixelox transform move saga.pxo --in-place --layer Author --to 50%,94% --anchor c
pixelox layer add saga.pxo --in-place --name Grain --fill '#808080'
pixelox filter add-noise saga.pxo --in-place --layer Grain --amount 34 --monochrome
pixelox filter motion-blur saga.pxo --in-place --layer Grain --angle 0 --distance 3
pixelox layer blend saga.pxo --in-place --layer Grain --mode overlay
pixelox layer opacity saga.pxo --in-place --layer Grain --value 55
pixelox export saga.pxo -o saga.png
```

Change first: the cutout (scale it so the head sits at the moon's center), the title word
(3 to 5 letters; the scale to 940 px sets its height), then the pair of color families.

## 10. Variations and pairings

* Swap the clash: violet sky `#3a1f5c` with a coral-to-yellow sun; deep green jungle
  haze with a magenta moon; black space with an ice-blue planet and a white title fade.
  Keep "one family for the ground, the opposite for the sky object and the title".
* Other discs: an eclipse (dark disc, bright rim with `fx outer-glow`), a ringed planet
  (add a thin tilted `shape ellipse` stroke), a porthole, a clock face.
* Film poster: move the author line to the top as the star's name; add a credit line at
  97% H in pale amber at cap height 0.9% H.
* Game or album cover (square): disc 70% of the width, figure cropped at the knees,
  title overlapping the disc's lower edge.
* Pairs with T-sunburst (faint rays behind the disc), T-vignette in rust, and
  T-distress-type on the title.

## 11. Pitfalls

* A moon whose lower edge is crisp: it turns into a sticker. Its last gradient stop must
  equal the sky color behind it, and the haze band must cross its bottom.
* A third color family (purple shadows, red title): the clash loses its force. Two
  families and the gold between them.
* The figure in black. It should be a dark tone of the ground's own hue (rust, not
  black), lighter toward the head.
* A title on the bright orange band: teal on mid-orange vibrates and the gold end of the
  fade disappears. The title needs the dark foot of the sky behind it.
* Light grain: at amount 10 the cover looks like a vector file. This style wants grain
  you can see from across a room.
