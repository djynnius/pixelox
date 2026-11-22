---
id: p018
title: Magenta three-speaker flyer, paper-texture headline
genre: [church, event-flyer]
mood: [festive, energetic, warm, bold]
style: [photo-composite, type-led]
format: portrait-a
archetype: A-speakers-over-title
palette: ['#300d54', '#9b1965', '#c71872', '#f2ebe5', '#e3c56b']
techniques: [T-spotlight, T-image-in-type, T-script-overlap, T-cutout-drop-shadow, T-rules, T-texture-fill]
needs: [subject-cutout, logo]
---

# p018: Magenta three-speaker flyer, paper-texture headline

## 1. Concept

An all-night church meeting announced on a hot magenta-to-violet gradient: three speaker
cutouts stand in a triangle in the upper half and dissolve at the waist into a white glow,
and out of that glow rises a short, very wide white headline whose letters look like
crumpled paper, with a gold script connecting word tucked beside the second line. It works
because the glow does two jobs at once: it hides the cut-off bottoms of three unrelated
photos and it lights the headline from behind, so people and title read as one picture.

## 2. Format and grid

* Portrait A-series (1 : 1.414). Center axis; side margins 5% W for fine print, 13% W for
  the headline.
* Zones: presenter strip 0 to 11% H; speakers 9 to 50% H; headline 47 to 70% H; details
  block 72 to 86% H; two fine-print lines at 89.3% and 95.5% H.
* The speakers form a triangle: the senior figure top right (largest, head at 9% H), the
  guest at the left (head at 17% H), the host in the middle and in front (head at 23% H).
* The details block is split at 37.5% W by a thin vertical gold rule: date stack
  right-aligned to its left, venue lines left-aligned to its right.

## 3. Layer stack

1. Ground: vertical gradient, hot magenta `#c71872` at the top, `#9b1965` at 20% H,
   plum `#57165a` at 50% H, deep violet `#2a0c56` at the bottom.
2. Ghost image: a pale raised-hand silhouette at the top left (x 0 to 30% W, y 0 to 30% H),
   about 12% opacity: only a hint of texture.
3. Speaker right: man in a blue suit with a red patterned cap and stole, head and chest,
   x 55 to 93% W, y 9 to 45% H.
4. Speaker left: man in a red suit, arms folded, x 11 to 44% W, y 17 to 48% H.
5. Speaker center: smiling woman in white, x 39 to 63% W, y 23 to 52% H, overlapping both.
6. Glow: radial, near-white `#f6f2ed` at the center (50% W, 46% H), gone by a radius of
   17% H; it sits in front of the speakers and wipes out everything below their chests.
7. Presenter strip: a white pennant with a round seal (x 22.5 to 32.5% W, y 0 to 7.5% H);
   the organization name in white condensed caps at 34% W, 4.3% H; a two-line descriptor
   in small regular caps to its right.
8. "presents" in white brush script, centered at 9% H, with a thin swash under it.
9. Role labels beside each speaker: a pale-gold condensed bold label ("Guest:", "Host:",
   a 2-word office) and the name in white light sans one line below. Left label at 13% W,
   21.5% H (right-aligned to 18% W); right label at 75% W, 18.5% H; center label at 64% W,
   39.5% H.
10. Headline line 1: a 3-letter word in white geometric bold caps, x 13 to 85% W, cap top
    47% H, baseline 62% H (cap height 15% H), with a crumpled-paper texture inside the
    letters (gray creases on white).
11. Script word: a 2-letter connecting word in gold brush script, x 9 to 24% W, y 61 to
    69% H, tilted up about 8 degrees, its tail under the first letter of line 1.
12. Headline line 2: an 8-letter word in the same white bold caps, x 24 to 83% W, cap top
    63.2% H, baseline 69.4% H (cap height 6.2% H), same paper texture.
13. Date stack, right edge at 36% W: day line (cap 1.7% H) at 73.8% H, month and year
    (cap 2.6% H each) at 76.3 and 79.2% H, a hairline, the time range (cap 1.3% H) at
    81.6% H between two hairlines.
14. Gold vertical rule: 37.5% W, from 73 to 82.6% H, 3 px.
15. Venue lines, left at 40% W: a heavy line with "@" (cap 2.4% H) at 74.2% H, then two
    light lines of the same size at 77.8 and 81.3% H.
16. Extra-speaker line, centered at 84.8% H: pale-gold condensed label plus a white light
    name (cap 1.9% H).
17. Scripture line: white italic, centered, 89.3% H. Contact line: pale gold, centered,
    95.5% H.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#300d54` | deep violet, lower half of the ground | 35% |
| `#9b1965` | magenta mid-tone of the ground | 25% |
| `#c71872` | hot magenta top edge | 10% |
| `#f2ebe5` | glow, headline, white text | 20% |
| `#e3c56b` | pale gold labels, script word, rule, contact line | 5% |
| `#c0283a` | reds of the speakers' clothes (incidental) | 5% |

Analogous magenta to violet, with white as the light and one small gold accent. The gold
is used only for labels and connectors, never for facts, so the facts stay white.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Organization | `[CHURCH NAME]`, 2 words | caps | condensed heavy grotesque (Oswald 700 / Impact) | heavy | 1.9% | 0 | - | left | `#ffffff` | 34%, 4.3% |
| Descriptor | 2 lines, 1 to 2 words each | caps | geometric sans (Jost / Avenir Next) | 500 | 1.0% | 20 | 105% | left | `#ffffff` | 59.5%, 4.3% |
| Presents | "presents" | lower | brush script (Yellowtail / Brush Script MT) | 400 | x-height 1.2% | 0 | - | center | `#ffffff` | 49%, 9.2% |
| Role label (x3) | 1 to 2 words + colon | title | condensed heavy grotesque (Oswald 700 / Impact) | heavy | 1.0% | 20 | - | left or right | `#e3c56b` | see stack |
| Role name (x3) | title + `[NAME]` | title | geometric sans light (Jost 300 / Avenir Next) | 400 | 0.95% | 0 | - | same as its label | `#ffffff` | 2% H below its label |
| Headline 1 | 3-letter word | caps | geometric sans bold (Montserrat 800 / Avenir Next) | 700 | 15% | 60 | - | center | `#ffffff` + texture | 49%, cap top 47% |
| Script word | 2-letter word | title | brush script (Kaushan Script / Brush Script MT) | 400 | cap 5.5% | 0 | - | left | `#d9b45c` | 9%, 61% (top-left) |
| Headline 2 | 8-letter word | caps | same as headline 1 | 700 | 6.2% | 40 | - | left | `#ffffff` + texture | 24%, cap top 63.2% |
| Day line | weekday + ordinal day, no space | caps | geometric sans bold (Jost 700 / Futura) | 700 | 1.7% | 0 | - | right | `#ffffff` | 36%, 73.8% |
| Month, year | 1 word, 4 digits | caps | same | 700 | 2.6% | 0 | 108% | right | `#ffffff` | 36%, 76.3% and 79.2% |
| Time range | start and end time, "8:00pm - 5:00am" style | lower | geometric sans (Jost 600 / Avenir Next) | 600 | 1.3% | 0 | - | right | `#ffffff` | 36.5%, 81.6% |
| Venue head | "@" + place + 1 word | caps | wide heavy geometric (Montserrat 900 / Avenir Next) | 900 | 2.4% | 20 | - | left | `#ffffff` | 40%, 74.2% |
| Venue lines | 2 lines, 3 to 4 words each | caps | geometric sans light (Jost 300 / Avenir Next) | 400 | 2.4% | 0 | - | left | `#ffffff` | 40%, 77.8% and 81.3% |
| Extra speaker | label + `[NAME]` | caps + title | condensed heavy label, light name | heavy / 400 | 1.9% | 0 | - | center | `#e3c56b` / `#ffffff` | 51%, 84.8% |
| Scripture | reference + 1 sentence, about 20 words | sentence | geometric sans italic | 400 | 0.9% | 0 | - | center | `#ffffff` | 50%, 89.3% |
| Contact | lead-in + 4 phone numbers | title | geometric sans | 500 | 1.1% | 30 | - | center | `#f3dc8a` | 50%, 95.5% |

Three voices: a geometric sans in light, bold, and black; a condensed heavy face for
labels; a brush script for two short words. That is one more than the usual limit, and it
holds only because the condensed face and the script never exceed two words.

## 6. Imagery

Three separate studio portraits, cut out (see "Getting a cutout" in techniques.md), each
with its own lighting: a man in a red suit with folded arms, a woman in white, an older
man in a blue suit with a red cap and stole. They are not graded to match; the white glow
and the magenta ground unify them. Scale encodes rank: the senior figure is about 1.25x
the others and highest; the host is smallest but in front. All three are cropped at the
chest by the glow, never by a hard edge. A circular seal sits in a white pennant at the
top (your logo; the pennant gives any logo a clean white ground on a loud background).

## 7. Effects and textures

* **T-spotlight** (glow layer only), placed in front of the speakers instead of behind a
  product: center 50% W, 45% H, radius 17% H, near-white to transparent white.
* Crumpled-paper letters: a high-contrast cloud texture (white with pale gray creases)
  clipped into each headline line, the method of **T-image-in-type**. A photo of real
  crumpled paper clipped the same way is better.
* **T-script-overlap**: the gold script word overlaps the lower-left corner of line 1 and
  the left end of line 2.
* **T-cutout-drop-shadow**, soft variant, on the headline only (offset 0,6, blur 18, 40%).
* **T-rules**: the gold vertical divider and two white hairlines around the time.
* No grain: the look is glossy and digital.
* **T-texture-fill** in [techniques.md](../techniques.md) is the tested library version of the generated texture clipped into the type or shapes; start from it when reusing this idea.

## 8. Hierarchy and eye path

1. The 3-letter headline word: brightest, widest, in the middle of the page.
2. Up into the glow to the three faces (the senior figure first: largest and highest).
3. Down through the script word to headline line 2.
4. The details block: heavy venue line, then the date stack.
5. Extra-speaker line, labels beside the faces, fine print.

Cap heights: headline 15 : line two 6.2 : month and venue 2.5 : extra speaker 1.9 : fine
print 1, about 15 : 6 : 2.5 : 2 : 1.

## 9. Build recipe

`subject.png` and `subject2.png` are your images here (three speaker cutouts in the real
job; the recipe reuses one stand-in twice), and `logo.png` is your round logo.

```bash
pixelox render js scripts/clouds.js -o paper.png --size 1080x1528 --seed 31 --param lo='#c9c2c0' --param hi='#ffffff' --param scale=7 --param octaves=3 --param warp=1.4 --param contrast=2.6 --param bias=0.3 --param res=2
pixelox new -o trio.pxo --size 1080x1528 --fill '#2a0c56'
pixelox layer add trio.pxo --in-place --name Grad
pixelox fill gradient trio.pxo --in-place --layer Grad --from 0,0 --to 0,100% --stops '#c71872@0,#9b1965@0.2,#57165a@0.5,#2a0c56@1'
pixelox comp place trio.pxo subject.png --in-place --as Senior --size x660 --at 74%,52% --anchor b
pixelox comp place trio.pxo subject2.png --in-place --as Guest --size x520 --at 27%,51% --anchor b
pixelox comp place trio.pxo subject.png --in-place --as Host --size x460 --at 51%,53% --anchor b
pixelox layer add trio.pxo --in-place --name Glow
pixelox fill gradient trio.pxo --in-place --layer Glow --kind radial --from 50%,45% --to 50%,62% --stops '#f6f2edff@0,#f6f2eddd@0.3,#f6dfe900@1'
pixelox shape path trio.pxo --in-place --name Pennant --d 'M 243 0 L 351 0 L 351 115 L 297 98 L 243 115 Z' --fill '#ffffff'
pixelox comp place trio.pxo logo.png --in-place --as Logo --size 92x --at 297,6 --anchor t
pixelox text add trio.pxo --in-place --name Org --content "GRACE HARVEST" --font Impact --size 2.5% --color '#ffffff' --at 34%,4.3% --anchor l
pixelox text add trio.pxo --in-place --name OrgSub --content $'FAMILY CHAPEL\nINTERNATIONAL' --font "Avenir Next" --weight 500 --size 1.4% --tracking 20 --leading 105% --color '#ffffff' --at 59.5%,4.3% --anchor l
pixelox text add trio.pxo --in-place --name Presents --content "presents" --font "Brush Script MT" --size 3.6% --color '#ffffff' --at 49%,9.2% --anchor c
pixelox shape line trio.pxo --in-place --name Swash --from 41%,10.3% --to 60%,8.9% --stroke '#ffffff' --stroke-width 2
pixelox text add trio.pxo --in-place --name Role1 --content "Guest:" --font Impact --size 1.3% --tracking 20 --color '#e3c56b' --at 18%,21.5% --anchor r
pixelox text add trio.pxo --in-place --name Name1 --content "Rev. First Speaker" --font "Avenir Next" --weight 400 --size 1.3% --color '#ffffff' --at 18%,23.5% --anchor r
pixelox text add trio.pxo --in-place --name Role2 --content "Senior Pastor:" --font Impact --size 1.3% --tracking 20 --color '#e3c56b' --at 96%,14.5% --anchor r
pixelox text add trio.pxo --in-place --name Name2 --content "Dr. Second Speaker" --font "Avenir Next" --weight 400 --size 1.3% --color '#ffffff' --at 96%,16.5% --anchor r
pixelox text add trio.pxo --in-place --name Role3 --content "Host:" --font Impact --size 1.3% --tracking 20 --color '#e3c56b' --at 64%,41% --anchor l
pixelox text add trio.pxo --in-place --name Name3 --content "Rev. Third Speaker" --font "Avenir Next" --weight 400 --size 1.3% --color '#ffffff' --at 64%,43% --anchor l
pixelox text add trio.pxo --in-place --name H1 --content "DAY" --font "Avenir Next" --weight 700 --size 20.8% --tracking 60 --color '#ffffff'
pixelox transform move trio.pxo --in-place --layer H1 --to 49%,47% --anchor t
pixelox fx drop-shadow trio.pxo --in-place --layer H1 --offset 0,6 --blur 18 --opacity 40
pixelox comp place trio.pxo paper.png --in-place --as Paper1 --fit cover
pixelox layer clip trio.pxo --in-place --layer Paper1
pixelox text add trio.pxo --in-place --name H2 --content "WONDERS" --font "Avenir Next" --weight 700 --size 8.6% --tracking 40 --color '#ffffff'
pixelox transform move trio.pxo --in-place --layer H2 --to 24%,63.2% --anchor tl
pixelox comp place trio.pxo paper.png --in-place --as Paper2 --fit cover
pixelox layer clip trio.pxo --in-place --layer Paper2
pixelox text add trio.pxo --in-place --name Of --content "Of" --font "Brush Script MT" --size 10.5% --color '#d9b45c'
pixelox transform rotate trio.pxo --in-place --layer Of --degrees -8
pixelox transform move trio.pxo --in-place --layer Of --to 9.5%,61% --anchor tl
pixelox text add trio.pxo --in-place --name Day --content "FRI 12TH" --font Futura --weight 700 --size 2.3% --color '#ffffff' --at 36%,73.2% --anchor r
pixelox text add trio.pxo --in-place --name MonthYear --content $'JUNE\n2031' --font Futura --weight 700 --size 3.4% --leading 92% --align right --color '#ffffff' --at 36%,77.6% --anchor r
pixelox text add trio.pxo --in-place --name Time --content "8:00pm - 5:00am" --font "Avenir Next" --weight 600 --size 1.8% --color '#ffffff' --at 36.5%,82.2% --anchor r
pixelox shape line trio.pxo --in-place --name TimeRule --from 20.5%,81% --to 36.5%,81% --stroke '#ffffff' --stroke-width 1 --opacity 70
pixelox shape line trio.pxo --in-place --name Divider --from 37.6%,73% --to 37.6%,82.6% --stroke '#e3c56b' --stroke-width 3
pixelox text add trio.pxo --in-place --name VenueHead --content "@CITY CAMPUS" --font "Avenir Next" --weight 900 --size 3.3% --tracking 20 --color '#ffffff' --at 40%,74.2% --anchor l
pixelox text add trio.pxo --in-place --name Venue --content $'VENUE: EXAMPLE\nHALL, MAIN ROAD' --font "Avenir Next" --weight 400 --size 3.3% --leading 106% --color '#ffffff' --at 40%,79.6% --anchor l
pixelox text add trio.pxo --in-place --name AlsoLabel --content "ALSO SPEAKING:" --font Impact --size 2.4% --color '#e3c56b' --at 43.5%,84.9% --anchor r
pixelox text add trio.pxo --in-place --name AlsoName --content "Pastor Fourth Speaker" --font "Avenir Next" --weight 400 --size 2.6% --color '#ffffff' --at 45%,84.8% --anchor l
pixelox text add trio.pxo --in-place --name Scripture --content "Psalm 00:00  one short verse that states the theme of the night, set small and in italics." --font "Avenir Next" --weight 400 --style italic --size 1.25% --color '#ffffff' --at 50%,89.3% --anchor c
pixelox text add trio.pxo --in-place --name Contact --content "For more information, contact  000 000 0001,  000 000 0002,  000 000 0003" --font "Avenir Next" --weight 500 --size 1.5% --tracking 30 --color '#f3dc8a' --at 50%,95.5% --anchor c
pixelox export trio.pxo -o trio.png
```

Change first: the three headline strings (a 3 to 4 letter word, a 2-letter script word, a 6
to 9 letter word), then the two ends of the gradient. With more or fewer speakers, keep
the triangle: the tallest figure at one side, the smallest in front in the middle.

## 10. Variations and pairings

* Palette swaps on the same structure: royal blue `#1238c7` to navy `#0a0f3a` with gold;
  emerald `#0d8a5a` to `#062b24` with cream; orange `#e8590c` to oxblood `#3d0a12` with
  white. Keep the glow near-white and warm.
* One speaker: a single cutout on the axis, head at 12% H, the glow at 48% H unchanged.
* Five or more speakers: a row of circles (portraits clipped to ellipses, 16% W each) at
  30 to 42% H replaces the triangle.
* Square post: drop the fine print and the extra-speaker line; speakers 5 to 45% H,
  headline 45 to 72% H, details in one line at 82% H.
* Pairs with T-gradient-mesh (magenta, violet, orange blobs) in place of the plain
  gradient, T-bokeh behind the speakers, and T-info-bar in place of the two-column block.

## 11. Pitfalls

* Hard-cropped speakers: three photo bottoms ending at three heights look like stickers.
  The glow must cover every bottom edge; widen it before you move the people.
* A pure white headline on a pure white glow loses its top edge. The texture and the soft
  shadow are what separate them; keep at least one, and keep the glow center above the
  letters' mid-line.
* All three speakers at the same size and height reads as a line-up. Vary scale by about
  25% and stagger the heads by 6 to 8% H.
* Gold used for facts (date, venue) turns to mud on magenta at small sizes. Gold is for
  labels and rules; facts are white.
* Too many type styles: this layout is already at the limit. Do not add an outline, a
  second script, or a gradient to the headline.
