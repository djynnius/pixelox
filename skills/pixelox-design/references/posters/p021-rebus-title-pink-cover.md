---
id: p021
title: Rebus title on flat pink cover
genre: [book-cover, editorial]
mood: [minimal, calm, playful, elegant]
style: [minimal, flat, photo-composite]
format: portrait-2x3
archetype: A-centered-title
palette: ['#fbcec9', '#281c19', '#46312c', '#c99a96']
techniques: [T-mono-on-color, T-spotlight]
needs: subject-cutout
---

# p021: Rebus title on flat pink cover

## 1. Concept

A literary-fiction cover that is 96% empty pink. The title is a rebus: a small photograph
of a black bird stands where the title's noun should be, with the article set to its left,
the possessive "'s" hooked onto its open beak, and the last word dropped a line lower to
the right. It works because the reader has to say the missing word in their head, which
makes a three-word title unforgettable, and because one small, dark, sharply detailed
object on a soft flat color has enormous pull.

## 2. Format and grid

* Portrait 2:3. No frame, no rules; margins are simply everything: the nearest element to
  an edge is 26% W away from it.
* The cluster of image and title words sits in the optical center, 34 to 63% H, and spans
  26.6 to 82% W. It reads as a sentence on two lines that step down to the right:
  line 1 (article, image, possessive) centered at 43.5% H, line 2 (last word) with its cap
  top at 58.2% H and its left edge at 62% W, directly under the possessive.
* The author sits alone at 90.4% H, centered (optically, at 51% W).
* Empty bands: 0 to 34% H and 63 to 88% H. Do not fill them.

## 3. Layer stack

1. Ground: flat pink `#fbcec9`, full canvas. No texture, no gradient.
2. Contact shadow: a soft ellipse of darker pink `#c99a96`, 39 to 59% W, 53.6 to 55.8% H,
   blurred, 60%. It is what makes the bird stand on the page instead of floating.
3. Bird: a photographic cutout of a black crow seen from the side, facing right, head
   thrown back and beak open as if calling, standing on two legs; box 30 to 61% W, 34 to
   55% H. Natural color: near-black feathers `#1f1714` with a dull brown sheen, a spot of
   pink inside the beak that picks up the ground color.
4. Article: a 3-letter word in a dark serif, right edge at 38.8% W, centered at 43.6% H.
5. Possessive: a curly apostrophe and a lowercase s in the same serif at the large size of
   the last word, left edge at 60.5% W, centered at 42.6% H, just clear of the beak's tip.
6. Last title word: 3 to 4 letters, the same serif at 1.65 times the article's size, cap top at
   58.2% H, left edge at 62.2% W.
7. Author: the same serif, smaller, letterspaced, centered at 51% W, 90.4% H.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#fbcec9` | ground | 96% |
| `#281c19` | type, feathers | 3% |
| `#46312c` | lighter feather tone | 0.5% |
| `#c99a96` | contact shadow | 0.5% |

One pale warm tint and one warm near-black. The type color is sampled from the bird, so
image and words are the same ink.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Article | "The" | title | transitional serif (Libre Baskerville, Libre Caslon Text / Baskerville) | 400 to 500 | 2.7% | 80 | - | right | `#281c19` | right edge 38.8%, 43.6% |
| Possessive | curly apostrophe + "s" | lower | same | 400 to 500 | x-height 2.9% (set at the last word's size) | 0 | - | left | `#281c19` | left edge 60.5%, 42.6% |
| Last word | 1 short noun, 3 to 4 letters | title | same | 400 to 500 | 4.5% | 20 | - | left | `#281c19` | left edge 62.2%, cap top 58.2% |
| Author | `[AUTHOR NAME]`, 2 words | title | same | 400 to 500 | 2.3% | 100 | - | center | `#281c19` | 51%, 90.4% |

One serif family at regular-to-medium weight in three sizes (Baskerville 400 locally; its
600 is too heavy). The possessive and the last word share the large size, so the eye links
them across the line break. The title words are mixed case, not caps: with so little on
the page, capitals would shout.

## 6. Imagery

One cutout photograph of a small animal or object that names the missing word (in the
original, a crow). It must be recognizable in outline at 20% of the canvas height, dark
against the ground, and it needs a gesture that connects it to the type: here the open
beak points at the possessive. Natural photographic detail is kept (this is not a flat
silhouette); the treatment is the **T-mono-on-color** idea with a black subject on one
flat color. The only added effect is the contact shadow from **T-spotlight**.

## 7. Effects and textures

* Contact shadow (the shadow step of **T-spotlight**): `shape ellipse` in a darker shade of
  the ground, blurred with `filter gaussian --sigma 12` (the filter stays live on the
  shape), 60% opacity, 20% W wide and 2.2% H tall, centered under the feet.
* **T-mono-on-color**: `adjust bw` on the cutout; no other grade.
* Nothing else. No grain, no vignette: the flatness is what makes it feel expensive.

## 8. Hierarchy and eye path

1. The bird (the only image, the darkest mass, dead center).
2. Left to the article, back across the bird to the possessive: the reader says the
   missing word.
3. Down and right to the last word (the largest type).
4. The author, far below.

Cap heights: last word 4.5 : article 2.7 : author 2.3, about 2 : 1.2 : 1. The bird is 21%
H tall, 4.7 times the largest cap height.

## 9. Build recipe

`subject.png` is your image here: a cutout of the thing the title leaves unsaid (an
animal, a key, a glove, a chair), dark and clear in outline. The two `adjust` lines only
darken the stand-in; with a naturally dark subject keep `adjust bw` and drop `levels`.

```bash
pixelox new -o rebus.pxo --size 1080x1620 --fill '#fbcec9'
pixelox shape ellipse rebus.pxo --in-place --name Shadow --rect 36%,53.7%,20%x2.2% --fill '#c99a96'
pixelox filter gaussian rebus.pxo --in-place --layer Shadow --sigma 12
pixelox layer opacity rebus.pxo --in-place --layer Shadow --value 60
pixelox comp place rebus.pxo subject.png --in-place --as Creature --size x340 --at 46%,55% --anchor b
pixelox adjust bw rebus.pxo --in-place --layer Creature
pixelox adjust levels rebus.pxo --in-place --layer Creature --gamma 0.55 --out-white 120
pixelox text add rebus.pxo --in-place --name Article --content "The" --font @oldstyle --weight 400 --size 4% --tracking 80 --color '#281c19' --at 38.8%,43.6% --anchor r
pixelox text add rebus.pxo --in-place --name Possessive --content "’s" --font @oldstyle --weight 400 --size 6.6% --color '#281c19' --at 60.5%,42.6% --anchor l
pixelox text add rebus.pxo --in-place --name LastWord --content "Oath" --font @oldstyle --weight 400 --size 6.6% --tracking 20 --color '#281c19'
pixelox transform move rebus.pxo --in-place --layer LastWord --to 62.2%,58.2% --anchor tl
pixelox text add rebus.pxo --in-place --name Author --content "Author Name" --font @oldstyle --weight 400 --size 3.4% --tracking 100 --color '#281c19' --at 51%,90.4% --anchor c
pixelox export rebus.pxo -o rebus.png
```

Change first: the cutout; then nudge the article and the possessive until each sits 3 to
5% W from the cutout's edge at the height of its "mouth" or most active point; then the
ground color.

## 10. Variations and pairings

* Ground swaps with the same near-black: butter `#f6e3a1`, sage `#cfdcc4`, powder blue
  `#cddff0`, bone `#efe9dc`. Keep the saturation low and the lightness above 80%.
* Other rebus positions: the image as the first word (no article), or as a letter inside
  a word (a moon for an O, a ladder for an H).
* Poetry pamphlet, theatre poster, wine label, a quiet product ad ("The [object]'s
  secret"): the same three-part sentence around a product cutout.
* Square post: same cluster at 30 to 66% H, author replaced by a small logo at 90% H.
* Pairs with T-silhouette (flat-fill the cutout when no good photo exists) and
  T-frame-border (a thin keyline at 6% for a classic-series look).

## 11. Pitfalls

* Making the image big. Above 30% H it becomes an ordinary picture cover and the joke of
  the missing word is lost. Keep it between 18 and 24% H.
* Type that does not touch the picture's logic: the article and the possessive must sit on
  the picture's horizontal middle, within 5% W of its edges, or the sentence falls apart.
* A cutout with a pale or busy outline on a pale ground: it dissolves. Choose a dark
  subject, or darken it.
* No contact shadow: the subject looks pasted. Too strong a shadow: it becomes a hole.
  Use the ground color darkened by about 20%, blurred, at 60%.
* Filling the empty top and bottom with quotes, badges, or a subtitle. If a line must be
  added, put it at 6% H in the author's size and nothing else.
