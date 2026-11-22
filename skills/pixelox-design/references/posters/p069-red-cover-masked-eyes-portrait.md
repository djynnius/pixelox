---
id: p069
title: Red cover with masked-eyes portrait
genre: [book-cover, fashion, editorial]
mood: [bold, playful, cool, energetic]
style: [photo-composite, flat, editorial]
format: portrait-2x3
archetype: A-portrait-type-stack
palette: ['#ee353d', '#3b3932', '#f3f0ea', '#d8e89c', '#161614']
techniques: [T-mono-on-color, T-label-box, T-paper-grain]
needs: [subject-cutout, logo]
---

# p069: Red cover with masked-eyes portrait

## 1. Concept

A contemporary novel cover in three flat colors and one photograph: a black-and-white
cutout of a young woman on a signal-red ground holds a red card in front of her eyes, and
the card, being the same red as the ground, punches a hole straight through her face. The
genre line is printed on the card. Below, three lines of heavy grotesque capitals (the
title in pale lime, the author's two names in black) are stacked hard against the left
margin, on top of her body. It works because of one visual joke with a point (a person
hiding in plain sight) and because the type is as loud and as flat as the color, so photo
and letters feel like cut paper.

## 2. Format and grid

* Portrait 2:3 (close to it: 1 : 1.53). One left margin at 5% W carries everything typed:
  quote, title, both author lines. Nothing is centered.
* Zones: quote 2 to 7.5% H; logo top right at 90% W, 10% H; figure 11 to 88% H, 8.5 to 80%
  W; card over the eyes 26 to 35% H, 35.7 to 68% W; type stack 60 to 94% H.
* Type stack: three lines of the same size, cap height about 9.6% H, cap tops at 60.1%,
  72.7%, and 84.7% H (gap between lines about 3% H). Line widths differ: 74% W, 42% W,
  80% W, ragged right.
* The stack overlaps the lower 30% of the figure; the figure's lower-left is cut away on a
  slant so the two black lines sit mostly on red.

## 3. Layer stack

1. Ground: flat red `#ee353d`, full canvas.
2. Figure: a black-and-white studio portrait, cut out: a woman with long straight
   center-parted hair, dark crew-neck sweater `#3b3932`, one hand raised to her face.
   Head top at 11% H; the body ends on a rough diagonal from 69% H at the left to 88% H at
   the right. High contrast: skin near-white `#f3f0ea`, sweater near-black, few mid-tones.
3. Card: a rectangle of the ground red, 32% W by 9% H, tilted about 3 degrees
   counter-clockwise, covering her eyes from temple to temple; her fingers (still black
   and white) overlap its right end, which is what makes it an object in her hand rather
   than a censor bar.
4. Card text: 2 short words ("A NOVEL") in black bold italic caps, cap height 3.2% H,
   centered on the card and tilted with it.
5. Title: 6 heavy caps in pale lime `#d8e89c`, left at 5% W, cap top at 60.1% H, over the
   sweater.
6. Author, line 1: 3 heavy black caps, left at 5% W, cap top 72.7% H.
7. Author, line 2: 6 heavy black caps, left at 5% W, cap top 84.7% H.
8. Quote: two lines of small black sans at the top left (a sentence in quotes, then a
   dash, a name, and a book title in caps).
9. Publisher mark: a small oval logo, 6.7% W wide, top right.
10. Faint grain.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#ee353d` | ground, card | 56% |
| `#3b3932` | sweater, hair shadows | 23% |
| `#f3f0ea` | skin, hair highlights | 6% |
| `#d8e89c` | title | 8% |
| `#161614` | author, small type | 7% |

Red plus black-and-white is the classic three; the pale lime is the surprise. It is the
complement of the red, pushed almost to white, so it glows on both the red and the dark
sweater.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Quote | 1 sentence of 7 to 9 words in quotes; then dash, `[REVIEWER NAME]`, 3 words, a title in caps | sentence | neo-grotesque (Inter, Archivo / Helvetica Neue) | 400 | 1.3% | 0 | 160% | left | `#161614` | 5%, 3.2% and 6.3% |
| Card text | "A NOVEL" | caps | neo-grotesque bold italic | 700 italic | 3.2% | 0 | - | center on the card, tilted -3 degrees | `#161614` | 52%, 30.5% |
| Title | `[BOOK TITLE]`, 1 word, 6 letters (a place name) | caps | neo-grotesque bold (Inter 800, Archivo 800 / Helvetica Neue) | 700 | 9.6% | -20 | - | left | `#d8e89c` | 5%, cap top 60.1% |
| Author 1 | first name, 3 letters | caps | same | 700 | 9.6% | -20 | - | left | `#161614` | 5%, cap top 72.7% |
| Author 2 | surname, 6 letters | caps | same | 700 | 9.6% | -20 | - | left | `#161614` | 5%, cap top 84.7% |

One family. Title and author are the same size and weight and differ only in color: the
title is the light line, the author the dark lines.

## 6. Imagery

One portrait, cut out and converted with **T-mono-on-color** (`adjust bw`, then `adjust
levels --in-black 25 --in-white 215`): few grays, a dark garment, pale skin. The pose
needs a raised hand near the eyes so the card can be "held". If the photo has no hand,
the card still works as a bar, but tilt it and let hair overlap one end. The figure sits
off-center to the left of the sheet's middle and is cut off below the chest on a slant.

## 7. Effects and textures

* **T-mono-on-color** on the cutout.
* Slanted crop: `mask add --from-path` with a polygon whose lower edge runs from 69% H at
  the left to 88% H at the right.
* Card: **T-label-box** logic with the box in the ground color: `shape rect`, rotated -3
  degrees; the text is rotated by the same angle.
* The three stack lines are placed by their letters, not their text boxes (`--anchor tl
  --anchor-to ink`), so `--at 5%,60.1%` is the exact left edge and cap top.
* **T-paper-grain**: `filter grain --as-layer`, amount 8. No shadows anywhere.

## 8. Hierarchy and eye path

1. The face, and at once the missing eyes: the card.
2. The words on the card.
3. Down the body to the lime title, then the two black author lines.
4. Back up to the quote; the logo last.

Cap heights: stack 9.6 : card text 3.2 : quote 1.3, about 7.4 : 2.5 : 1.

## 9. Build recipe

`subject.png` (a portrait cutout) and `logo.png` are your images here. Slide the Card
rectangle so it covers the eyes of your subject, and keep the card text centered on it.

```bash
pixelox new -o novel.pxo --size 1080x1620 --fill '#ee353d'
pixelox comp place novel.pxo subject.png --in-place --as Figure --size x1250 --at 52%,11% --anchor t
pixelox mask add novel.pxo --in-place --layer Figure --from-path 'M 0 0 H 1080 V 1420 L 560 1300 L 0 1120 Z'
pixelox adjust bw novel.pxo --in-place --layer Figure
pixelox adjust levels novel.pxo --in-place --layer Figure --in-black 25 --in-white 215
pixelox shape rect novel.pxo --in-place --name Card --rect 387,421,350x146 --fill '#ee353d'
pixelox transform rotate novel.pxo --in-place --layer Card --degrees -3
pixelox text add novel.pxo --in-place --name CardText --content "A NOVEL" --font @neo-grotesque --weight 700 --style italic --size 4.3% --color '#161614' --at 52%,30.5% --anchor c
pixelox transform rotate novel.pxo --in-place --layer CardText --degrees -3
pixelox text add novel.pxo --in-place --name Title --content "HARBOR" --font @neo-grotesque --weight 700 --size 12.8% --tracking -20 --color '#d8e89c' --at 5%,60.1% --anchor tl --anchor-to ink
pixelox text add novel.pxo --in-place --name Author1 --content "IDA" --font @neo-grotesque --weight 700 --size 12.8% --tracking -20 --color '#161614' --at 5%,72.7% --anchor tl --anchor-to ink
pixelox text add novel.pxo --in-place --name Author2 --content "AUTHOR" --font @neo-grotesque --weight 700 --size 12.8% --tracking -20 --color '#161614' --at 5%,84.7% --anchor tl --anchor-to ink
pixelox text add novel.pxo --in-place --name Quote --content $'"Quick, bright, and unsparing about a city and a self."\n-Critic Name, author of ANOTHER BOOK' --font @neo-grotesque --weight 400 --size 1.75% --leading 160% --color '#161614' --at 5%,4.8% --anchor l
pixelox comp place novel.pxo logo.png --in-place --as Logo --size 72x --at 89.7%,10% --anchor c
pixelox filter grain novel.pxo --in-place --as-layer --name Grain --amount 8
pixelox export novel.pxo -o novel.png
```

Change first: the cutout and the card position; then the three stack words (if the
longest passes 82% W, lower all three sizes together); then the ground and title colors
as a complementary pair.

## 10. Variations and pairings

* Color pairs (ground / title, author stays black): cobalt `#1f4fd8` / pale peach
  `#ffd9c2`; emerald `#0f9d6b` / pale pink `#ffd3e0`; yellow `#ffd400` / white with a
  black card instead of a ground-colored one.
* Other hole-punch objects in the ground color: a strip over the mouth, a disc over the
  whole face, a torn-out silhouette of a bird across the chest.
* Fashion or concert poster: replace the author lines with date and venue; replace the
  card text with the act's name.
* Square post: figure cropped at the shoulders, card kept, a single line of type.
* Pairs with T-halftone-portrait (coarse dots on the cutout), T-torn-collage (a torn
  paper card), and T-frame-border.

## 11. Pitfalls

* A card in any color other than the ground: it becomes a censor bar and the hole-in-the-
  face effect is gone. Same hex as the ground, no shadow, no outline.
* Leaving mid-gray skin tones: the cutout looks like a dull photo on red. Push the levels
  until skin is nearly white and the garment nearly black.
* Stack lines in different sizes fitted to one width: the calm, blunt rhythm is lost.
  Same size, ragged right.
* Black type crossing the dark garment: the author's name disappears. Cut the figure away
  under the black lines, or move the figure up.
* Centering the stack. It hangs from the same 5% margin as the quote.
