---
id: p097
title: Pink music-magazine portrait cover
genre: [magazine-cover, editorial]
mood: [bold, playful, energetic, warm]
style: [editorial, photo-composite]
format: portrait-3x4
archetype: A-cover-masthead
palette: ['#e887a8', '#0d1106', '#c61e24', '#ffffff', '#b85878', '#cc3433']
techniques: [T-text-behind-subject, T-gradient-type, T-outline-type, T-fit-width, T-photo-fade]
needs: subject-cutout
---

# p097: Pink music-magazine portrait cover

## 1. Concept

An independent music-magazine cover: a tight studio portrait of a young man with vivid
red hair on a candy-pink backdrop, a lowercase two-tone masthead across the top with his
hair pushing up into it, three small right-aligned cover lines beside his face, and his
name in two huge white lines over his dark clothes at the bottom. It works because of
three colors only (pink, red, black) with white type, and because the top word and the
bottom name are set to exactly the same width, so the cover has a firm rectangle of type
around a loose, tilted portrait.

## 2. Format and grid

* Portrait, letter proportion (1 : 1.294, a little squarer than 3:4; built at 1080x1400).
  Full-bleed photo, no border.
* One measure for everything: left edge 9.3% W, right edge 91% W. The masthead and the
  name both span it edge to edge; the issue line hangs from both of its ends; cover lines
  are right-aligned to its right edge; the deck is left-aligned to its left edge.
* Vertical zones: masthead 5.3 to 15% H; issue line 18% H; face 12 to 60% H; cover lines
  26 to 52% H (right of the face); deck 71% H; name 73.7 to 92.8% H; bottom margin 7.2% H.
* The picture is dark below 60% H and pink above it: light type zone on top, heavy type
  zone at the bottom.

## 3. Layer stack

1. Backdrop: pink `#e887a8` at the top left, deepening to magenta-pink `#b85878` at the
   right edge around 35 to 60% H (a soft diagonal studio falloff).
2. Masthead: two lowercase words set as one, ending in a period. Ascender top 5.3% H,
   baseline 15% H, x-height 7% H; x 9.3 to 91% W. The first word (to 54.5% W) is red
   `#cc3433`, the second word and the period are white.
3. Portrait (in front of the masthead): head tilted about 20 degrees to the right, resting
   against a raised hand at the left edge; shaggy bright red hair from 12% H, covering the
   lower half of masthead letters 3 to 5; face centered at 40% W, 42% H; chin at 60% H.
   Black open-knit cardigan over a pale pinstriped shirt and loose dark tie fills
   everything below 60% H and runs off three edges.
4. Issue line, white bold lowercase, baseline 18% H: issue number at 9.3% W, season and
   year ending at 91% W.
5. Cover lines, right-aligned at 91% W, three blocks of 3, 4, and 3 lines (line centers
   27.1, 29, 31.2 / 36, 38.2, 40.3, 42.2 / 47, 49.1, 51% H). Light lowercase lines in
   pink-white, names in bold white caps, titles in light italic.
6. Deck: one light lowercase line, 9 to 73.5% W, centered on 71% H.
7. Name ghost: the name's outline only, red `#e0514f`, 2 to 3 px, shifted 0.7% W right
   and down.
8. Name: two lines of bold white geometric caps. Line 1 (7 letters) cap top 73.7% H,
   cap height 8.8% H, 9 to 82% W. Line 2 (8 letters) cap top 84.2% H, 9 to 90.5% W.
   Gap between the lines 1.7% H.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#e887a8` | backdrop pink | 35% |
| `#0d1106` | black knit, the ground for the name | 30% |
| `#c61e24` | hair | 15% |
| `#ffffff` | name, second masthead word, cover lines | 10% |
| `#b85878` | backdrop falloff | 8% |
| `#cc3433` | first masthead word, name ghost (`#e0514f`) | 2% |

Pink and red are neighbors on the wheel, so the red hair glows instead of clashing, and
the masthead's red word is simply the hair color put into type. Black anchors the bottom.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Masthead | `[MASTHEAD]`: 2 short words closed up, 5 + 4 letters, then a period | lower | geometric sans (Montserrat, Poppins / Avenir Next) | 700 | x-height 7%, ascender 9.7% | -20 | - | justified to the measure | `#cc3433` then `#ffffff` | 9.3%, baseline 15% |
| Issue | "issue NN" | lower | same | 700 | 1.6% | 0 | - | left | `#ffffff` | 9.3%, baseline 18% |
| Season | season + year | lower | same | 700 | 1.6% | 0 | - | right | `#ffffff` | 91%, baseline 18% |
| Cover line, light | 3 to 5 words, a teaser | lower | same | 300 to 400 | 1.6% | 0 | 86% | right | `#ffe9f0` | 91%, see layer 5 |
| Cover line, name | `[ARTIST NAME]`, 2 words | caps | same | 700 | 1.65% | 0 | - | right | `#ffffff` | 91%, 31.2%, 38.2%, 47% |
| Cover line, title | a release title, 1 to 3 words | caps or lower, italic | same | 300 to 400 | 1.6% | 0 | - | right | `#ffe9f0` | 91%, 42.2%, 51% |
| Deck | one line, 6 to 8 words ending in "with" | lower | same | 300 to 400 | 1.7% | 0 | - | left | `#ffffff` | 9%, 71% |
| Name | `[ARTIST NAME]`, 2 lines, 7 and 8 letters | caps | same | 700 to 800 | 8.8% | -10 | 86% | left, fitted to the measure | `#ffffff` | 9%, cap tops 73.7% and 84.2% |

One family in three weights, plus its italic. Hierarchy comes from case and weight alone:
light lowercase talks, bold caps names.

## 6. Imagery

A waist-up studio portrait against seamless pink paper, lit softly from the front left so
the backdrop falls off to the right. The subject leans his head on his hand, mouth
slightly open, looking at the camera from under a heavy fringe: a casual, tilted pose
that contrasts with the strict type. Hair dyed the color of the masthead; black nail
polish, a chunky ring, a chain; nothing else colored. Natural grade with strong
saturation.

The hair overlaps the masthead (**T-text-behind-subject**). With a single photograph,
cut the subject out of it and place the cutout in register above the masthead; with a
cutout only, build the backdrop as a gradient, as the recipe does.

## 7. Effects and textures

* Two-tone masthead: one text layer with `fx overlay` and two hard stops at 0.56 of its
  width (**T-gradient-type** used as a hard split, not a blend). The split must fall in
  the gap between the two words.
* **T-text-behind-subject**: masthead, then the cutout.
* **T-outline-type**: a duplicate of the name (`layer dup --below Name`, so it lands under
  the white name) with a transparent fill and a 3 px red stroke, moved 8 px right and
  down. It reads as a misregistered second print pass.
* **T-fit-width**: masthead and name both set with `--fit-width 882` (81.7% W).
* **T-photo-fade** as a shade: near-black, alpha 0 at 58% H to 90% at 84% H, only needed
  when the subject's clothing is not already dark.

## 8. Hierarchy and eye path

1. The red hair and the face (largest saturated shape, center).
2. The name at the bottom (largest type, white on black).
3. The masthead, read left to right from red into white.
4. Cover lines down the right edge; the deck; the issue line.

Sizes: name cap height 8.8 : masthead x-height 7 : deck 1.7 : cover lines 1.65, about
5.3 : 4.2 : 1 : 1. Only two sizes exist: huge and small.

## 9. Build recipe

`subject.png` is your image here: a cutout portrait, ideally with dark clothing.

```bash
pixelox new -o cover.pxo --size 1080x1400 --fill '#e887a8'
pixelox layer add cover.pxo --in-place --name Backdrop
pixelox fill gradient cover.pxo --in-place --layer Backdrop --from 30%,0 --to 95%,75% --stops '#e887a8ff@0,#cf6f90ff@0.55,#b85878ff@1'
pixelox text add cover.pxo --in-place --name Masthead --content "sortafun." --font @geometric-sans --weight 700 --fit-width 882 --tracking -20 --color '#ffffff' --at 9.3%,15% --anchor bl --anchor-to ink
pixelox fx overlay cover.pxo --in-place --layer Masthead --stops '#cc3433@0,#cc3433@0.56,#ffffff@0.56,#ffffff@1' --angle 0
pixelox comp place cover.pxo subject.png --in-place --as Subject --size x1750 --at 40%,0% --anchor t
pixelox layer add cover.pxo --in-place --name Shade
pixelox fill gradient cover.pxo --in-place --layer Shade --from 0,58% --to 0,84% --stops '#0d110600@0,#0d1106e6@1'
pixelox text add cover.pxo --in-place --name Issue --content "issue 27" --font @geometric-sans --weight 700 --size 2.3% --color '#ffffff' --at 9.3%,17.6% --anchor l
pixelox text add cover.pxo --in-place --name Season --content "spring 2031" --font @geometric-sans --weight 700 --size 2.3% --color '#ffffff' --at 91%,17.6% --anchor r
pixelox text add cover.pxo --in-place --name C1a --content $'an open conversation with\nrising film composer' --font @geometric-sans --weight 400 --size 2.2% --leading 86% --align right --color '#ffe9f0' --at 91%,27.8% --anchor r
pixelox text add cover.pxo --in-place --name C1b --content "INES MORROW" --font @geometric-sans --weight 700 --size 2.3% --color '#ffffff' --at 91%,31.2% --anchor r
pixelox text add cover.pxo --in-place --name C2a --content "chasing the quiet:" --font @geometric-sans --weight 400 --size 2.2% --color '#ffe9f0' --at 91%,35.7% --anchor r
pixelox text add cover.pxo --in-place --name C2b --content "TALA REYES" --font @geometric-sans --weight 700 --size 2.3% --color '#ffffff' --at 91%,38.2% --anchor r
pixelox text add cover.pxo --in-place --name C2c --content "shares her debut EP" --font @geometric-sans --weight 400 --size 2.2% --color '#ffe9f0' --at 91%,40.1% --anchor r
pixelox text add cover.pxo --in-place --name C2d --content "UNDERTOW" --font @geometric-sans --weight 400 --style italic --size 2.2% --color '#ffe9f0' --at 91%,42.3% --anchor r
pixelox text add cover.pxo --in-place --name C3a --content "THE LANTERNS" --font @geometric-sans --weight 700 --size 2.3% --color '#ffffff' --at 91%,47% --anchor r
pixelox text add cover.pxo --in-place --name C3b --content "open up their new album" --font @geometric-sans --weight 400 --size 2.2% --color '#ffe9f0' --at 91%,48.9% --anchor r
pixelox text add cover.pxo --in-place --name C3c --content "slow weather" --font @geometric-sans --weight 400 --style italic --size 2.2% --color '#ffe9f0' --at 91%,50.9% --anchor r
pixelox text add cover.pxo --in-place --name Deck --content "turning the page: five years on the road with" --font @geometric-sans --weight 400 --size 2.4% --color '#ffffff' --at 9%,71% --anchor l
pixelox text add cover.pxo --in-place --name Name --content $'DOMINIC\nCALLOWAY' --font @geometric-sans --weight 700 --fit-width 882 --leading 86% --tracking -10 --color '#ffffff' --at 9%,73.7% --anchor-to ink
pixelox layer dup cover.pxo --in-place --layer Name --name Ghost --below Name
pixelox text style cover.pxo --in-place --layer Ghost --color transparent --stroke '#e0514f' --stroke-width 3
pixelox transform move cover.pxo --in-place --layer Ghost --by 8,8
pixelox export cover.pxo -o cover.png
```

Change first: the masthead string and its split position (the second pair of stops is
`x of the gap between the words / layer width`; look at the render and nudge it by 0.01),
then the name (7 to 9 letters per line keeps the cap height near 8 to 9% H at this
width), then the backdrop color: sample a second color from the subject (hair, a jacket,
lipstick) for the masthead's first word.

## 10. Variations and pairings

* Palette swaps that keep the logic "backdrop + its darker neighbor + black + white":
  butter `#f5d76e` with orange `#e8590c`; sky `#9fd3f0` with cobalt `#1f4fd8`; mint
  `#a8e0c2` with forest `#1d6b4a`.
* Zine or gig poster: replace the cover lines with a three-act lineup and the deck with
  date and venue; the name stays.
* Podcast or playlist cover (square): crop to the face, keep the masthead and the name,
  drop the cover lines.
* A red-framed news weekly treats the same ingredients formally: see
  [p089](p089-red-frame-annual-list-cover.md) and [p123](p123-full-figure-list-cover.md).
* Pairs with T-mono-on-color (black-and-white portrait on the pink, red kept only in the
  type), T-paper-grain at amount 12, and T-label-box for a "new issue" tag at the
  portrait's shoulder.

## 11. Pitfalls

* Masthead and name set to different widths. Their shared left and right edges are the
  whole structure; give both the same `--fit-width`.
* The hair or head hiding more than the lower half of two or three masthead letters. The
  word must still be readable at a glance; move the subject down or left.
* Cover lines on the face. They belong on clear backdrop beside it; if the head is wide,
  shift the subject toward the opposite edge or cut a block.
* White name over light clothing. Use the Shade layer, or choose a photo with dark
  clothes: the name needs a ground darker than 20% lightness.
* A thick or far-offset ghost outline: more than 3 px or more than 1% W of shift turns a
  print accent into a second, blurry name.
