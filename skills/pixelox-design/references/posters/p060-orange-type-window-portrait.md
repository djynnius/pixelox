---
id: p060
title: Orange story with portrait woven into type
genre: [church, social-post, event-flyer]
mood: [bold, intense, energetic, urgent]
style: [type-led, photo-composite]
format: portrait-9x16
archetype: A-type-block
palette: ['#f94a07', '#080608', '#201319', '#8a2c13', '#ee2102']
techniques: [T-fit-width, T-image-in-type, T-text-behind-subject, T-vignette, T-paper-grain, T-texture-fill, T-stretch-type]
needs: subject-cutout
---

# p060: Orange story with portrait woven into type

## 1. Concept

A story-format reminder made of two giant black words stacked on flat orange, with a
pointing man woven through them: above the first word's baseline he stands in front of the
letters as a normal cutout, and below it his body is visible only inside the letters of
the second word. It works because one trick does all the work: the type is at once a wall
behind him and a window onto him, so two words and one photo become a single object, and
nothing else is needed on the page, not even a date.

## 2. Format and grid

* Portrait 9:16. Margins 6% W left, 4.5% W right; 16% H of empty orange above the type and
  11% H below it.
* Two lines justified to one block, 6 to 95.5% W. Line 1 (4 letters): cap top 16.2% H,
  baseline 55.8% H, cap height 39.6% H. Line 2 (6 letters): cap top 57.6% H, baseline 89% H,
  cap height 31.4% H. The gap between the lines is 1.8% H, a thin orange slit that also
  cuts across the figure.
* Figure axis at 57% W, right of center, because he points left: head top at 25% H, head 22%
  W wide, pointing hand reaching 9.5% W at 44% H, right shoulder at 90% W.
* There is no small text. The only "grid" is the block and the diagonal of the arm.

## 3. Layer stack

1. Ground: orange `#f94a07`, slightly brighter (`#fb4e02`) in the middle.
2. Vignette: orange-red `#ee2102` creeping in from the four corners, strongest at the top
   and bottom edges.
3. Headline line 1: 4 letters, near-black `#080608`, ultra-condensed, 89.5% W wide.
4. Texture in line 1: a dark red mottled pattern (short horizontal dashes and blotches,
   `#8a2c13` on black, like a crowd under red light) clipped to the letters; about 35% of
   the letter area is red, and it is strongest at the right.
5. Headline line 2: 6 letters, same face and color, 90% W wide.
6. Texture in line 2: the same red mottle, clipped; strongest at the left and right ends.
7. Figure, lower part: a man in a navy pinstripe suit; his torso, his left hand, and a
   microphone show only inside the letters of line 2 (clipped to them). Between the
   letters, the ground is plain orange.
8. Figure, upper part: the same cutout at the same position, shown only above line 1's
   baseline (a straight horizontal cut at 55.8% H). Head and glasses cover the upper half
   of letters 2 and 3; the pointing right hand covers the lower part of letters 1 and 2;
   the right shoulder covers the lower half of letter 4.
9. Grain: fine noise over everything.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#f94a07` | orange ground | 45% |
| `#080608` | black letters | 25% |
| `#201319` | navy-black suit | 15% |
| `#8a2c13` | dark red texture inside the letters | 8% |
| `#ee2102` | orange-red vignette | 5% |
| `#9e4d33` | skin, and a red tie as the only bright accent | 2% |

Orange and black only: the highest-impact two-color pair after yellow and black. The red
texture and the red tie sit between the two, so nothing new enters the scheme.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Headline 1 | `[ORG]` or a 4-letter word | caps | ultra-compressed heavy grotesque (Anton or League Gothic stretched tall / Impact stretched 170% tall) | heavy | 39.6% | 0 | - | justified, 89.5% W | `#080608` | 6%, cap top 16.2% |
| Headline 2 | a weekday or a 6-letter word | caps | same | heavy | 31.4% | 0 | - | justified, 90% W | `#080608` | 5.5%, cap top 57.6% |

One face, two lines, no other type. Each letter is about 3.2 times taller than wide: the
counters are slits and the strokes are as wide as the gaps between letters.

## 6. Imagery

One portrait cutout: a bearded man in glasses, navy suit, white shirt, and red tie, turned
three-quarters to the left and pointing out of the frame with his right hand, the other
hand holding a microphone at his waist. Color is natural and rich, with strong contrast; no
outline, no glow. The pose matters: an arm that reaches sideways across the letters gives
the weave its diagonal. The figure is used twice at the same position: once clipped to the
letters of line 2 (**T-image-in-type**) and once masked with a rectangle that ends at line
1's baseline (**T-text-behind-subject**, with the subject in front).

## 7. Effects and textures

* **T-fit-width**, both lines fitted to one block. With a standard condensed face the
  lines come out too short, so the recipe narrows the letters with `--scale-x` (52.3% and
  48%, **T-stretch-type**) and `--fit-width` then sets them taller. The text stays live.
* **T-image-in-type** twice: a red mottle clipped to each line, and the figure clipped to
  line 2. The mottle is **T-texture-fill**: `fill noise --as-layer` with small, hard-edged,
  sideways blotches (`--scale 6% --stretch 2.2 --angle 0 --contrast 35`) and a different
  `--seed` per line. Several layers can be clipped to the same base: each `layer clip`
  attaches to the nearest unclipped layer below.
* Straight cut: `mask add --from-path` with a rectangle from the top edge to line 1's
  baseline on the second copy of the figure.
* **T-vignette** in orange-red, not black: `#e5160600` to `#e51606cc`.
* **T-paper-grain**: `filter grain --as-layer`, amount 8.

## 8. Hierarchy and eye path

1. The black block as a whole (three quarters of the page).
2. The face, the only fully detailed thing, in the upper middle.
3. Along the arm to the pointing hand at the left edge, which sends the eye back to the
   first letters; the word is then read left to right.
4. Line 2, with the discovery that the body continues inside the letters.

There is no size hierarchy: the two lines are 39.6 and 31.4% H, and nothing else exists.
The surprise (the weave) replaces the usual levels.

## 9. Build recipe

`subject.png` is your image here: a cutout at least 1300 px tall, ideally with one arm
reaching sideways.

```bash
pixelox new -o weave.pxo --size 1080x1920 --fill '#f94a07'
pixelox layer add weave.pxo --in-place --name Vignette
pixelox fill gradient weave.pxo --in-place --layer Vignette --kind radial --from 50%,48% --to 50%,112% --stops '#e5160600@0.45,#e51606cc@1'
pixelox text add weave.pxo --in-place --name H1 --content "PRAY" --font @condensed-heavy --scale-x 52.3% --fit-width 966 --color '#080608' --at 50.8%,16.2% --anchor t --anchor-to ink
pixelox fill noise weave.pxo --in-place --as-layer --name Tex1 --seed 4 --scale 6% --octaves 3 --warp 12 --stretch 2.2 --angle 0 --contrast 35 --bias=-15 --stops '#070507,#8a1218' --opacity 65
pixelox layer clip weave.pxo --in-place --layer Tex1
pixelox text add weave.pxo --in-place --name H2 --content "FRIDAY" --font @condensed-heavy --scale-x 48.0% --fit-width 972 --color '#080608' --at 50.5%,57.6% --anchor t --anchor-to ink
pixelox fill noise weave.pxo --in-place --as-layer --name Tex2 --seed 9 --scale 6% --octaves 3 --warp 12 --stretch 2.2 --angle 0 --contrast 35 --bias=-15 --stops '#070507,#8a1218' --opacity 65
pixelox layer clip weave.pxo --in-place --layer Tex2
pixelox comp place weave.pxo subject.png --in-place --as Body --size x1300 --at 57%,92.7% --anchor b
pixelox layer clip weave.pxo --in-place --layer Body
pixelox comp place weave.pxo subject.png --in-place --as Bust --size x1300 --at 57%,92.7% --anchor b
pixelox mask add weave.pxo --in-place --layer Bust --from-path 'M 0 0 H 1080 V 1071 H 0 Z'
pixelox filter grain weave.pxo --in-place --as-layer --name Grain --amount 8
pixelox export weave.pxo -o weave.png
```

Change first: the two words (3 to 5 letters on top, 5 to 7 below; the `--fit-width` values
stay the same, and a smaller `--scale-x` makes a line taller), then the figure's `--at` so
the face sits over the middle letters of line 1 and no letter is hidden completely. The
mask's `V 1071` is line 1's baseline in pixels (55.8% H): change it when you move the
lines.

## 10. Variations and pairings

* Other two-color pairs: yellow `#ffd400` and black; cobalt `#1238e6` and white letters
  with a navy texture; hot pink `#ff2d78` and black; lime `#c6f000` and black.
* Reverse the weave: the figure in front of line 2 and inside line 1 (mask from the gap
  downward, clip to line 1), for a subject whose head should sit lower.
* Add one fact: a single line of white caps (cap height 1.4% H) centered in the empty
  orange under the block at 94% H. Never more than one line.
* Sermon series, album cover, athlete announcement, fashion drop, conference keynote.
  Square: one word only, figure in front above 60% H and inside the letters below.
* Pairs with T-duotone on the figure (black to orange) for a one-ink print look, and
  T-halftone-portrait in place of the red mottle.

## 11. Pitfalls

* Words with thin or light letters leave no window. The letters must be heavy enough that
  stroke width is at least 60% of the gap between strokes.
* Forgetting the straight cut: if the second copy of the figure is not masked at line 1's
  baseline, the body covers line 2 and the window effect is gone.
* The face hiding a whole letter. Move the figure until each letter keeps at least a third
  of its shape; faces go over round or diagonal letters best, where the eye fills the gap.
* The two copies of the figure at different sizes or positions: the join at the baseline
  must be seamless, so place both with identical `--size` and `--at`.
* A dark ground: black letters need a bright flat color behind them, and the figure's dark
  suit needs the letters, not the ground, behind it.
