---
id: p001
title: Dark Swiss conference poster
genre: [conference, corporate, event-flyer]
mood: [serious, minimal, cool, elegant]
style: [swiss, editorial, minimal]
format: portrait-a
archetype: A-top-photo-bottom-text
palette: ['#0b0b0b', '#5e5e5e', '#ffffff', '#bab7b7']
techniques: [T-photo-fade, T-rules, T-mono-on-color]
needs: photo
---

# p001: Dark Swiss conference poster

## 1. Concept

A design-conference poster in black, white, and one gray: a black-and-white architecture
photo fills the top half and melts into a black field that carries a small, left-aligned
headline broken into four short lines, with two quiet text columns beneath. It works
because of restraint: one typeface, three tones, a strict left edge, and a lot of empty
black. The broken headline with a trailing underscore is the only playful move.

## 2. Format and grid

* Portrait A-series (1 : 1.414). Margins 6.8% W left and right, 4.5% H at the bottom.
* Two columns: the main column starts at x 6.8% W, the side column at x 62.4% W. Every
  text block is left-aligned to one of those two lines.
* Zones: photo 0 to 47% H; headline 45 to 65% H (its gray square starts inside the photo at
  38.7% H and so ties the zones together); details 68 to 91% H; footer line at 95% H.
* The right half of the band from 47 to 67% H is deliberately empty.

## 3. Layer stack

1. Ground: near-black `#0b0b0b`, full canvas.
2. Photo: black-and-white architectural detail (a curved, perforated metal facade seen
   from below), full width, 0 to 47% H, high contrast, brightest at top center.
3. Fade: ground color from transparent at 26% H to solid at 48% H (T-photo-fade).
4. Gray square: `6.8%,38.7%,22% W square`, solid `#5e5e5e`, overlapping the photo's
   lower edge and the first two headline lines.
5. Headline: four lines, white, left at 6.8% W, cap tops at 45.5%, 50.7%, 56%, 61.2% H.
6. Date line and city line: white, 69.9% and 72.8% H.
7. Short rule: white, 2 px, from 6.8% to 11.2% W at 76.4% H.
8. Lead-in sentence (bold italic) at 79.9% H and four body lines below it.
9. Side column: two blocks at 68.5% and 83.2% H. Each has a small gray square marker
   (3.4% W) behind the first letters of a bold italic sub-heading, then 2 to 3 body lines.
10. Footer: letterspaced web address at the left margin; three small social icons and a
    handle line ending at the right margin; both at 95.2% H.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#0b0b0b` | ground, photo shadows | 70% |
| `#bab7b7` | photo highlights (light gray, not white) | 15% |
| `#5e5e5e` | gray square and markers | 5% |
| `#ffffff` | all type | 10% |

Pure monochrome. The gray square is exactly halfway between the ground and the type, which
is why white text reads on it and on the black with equal ease.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Headline | a 2-word event name of about 12 letters, broken mid-word into 4 lines of 2 to 5 letters, ending in "_" | caps | rounded geometric sans (Mulish, Nunito Sans / Avenir Next) | 500 | 3.6% | 0 | 104% | left | `#ffffff` | 6.8%, first cap top 45.5% |
| Date | day month year | sentence | same | 400 | 1.35% | 0 | - | left | `#ffffff` | 6.8%, 69.9% |
| City | 1 to 2 words | title | same, italic | 600 | 1.4% | 0 | - | left | `#ffffff` | 6.8%, 72.8% |
| Lead-in | one short sentence, 4 to 6 words | sentence | same, italic | 700 | 1.05% | 0 | - | left | `#ffffff` | 6.8%, 79.9% |
| Body | 4 short lines, 3 to 5 words each | sentence | same | 400 | 1.05% | 0 | 135% | left | `#f2f2f2` | 6.8%, top 81% |
| Sub-heading (x2) | a 3 to 4 word question | title | same, italic | 700 | 1.15% | 0 | - | left | `#ffffff` | 62.4%, 70.3% and 85.1% |
| Side body (x2) | 2 to 3 short lines | sentence | same | 400 | 1.05% | 0 | 135% | left | `#f2f2f2` | 62.4%, top 71.9% and 86.7% |
| Web address | one URL | lower | same | 700 | 0.9% | 300 | - | left | `#ffffff` | 6.8%, 95.2% |
| Handle | hashtag + handle | lower | same | 400 | 1.05% | 60 | - | right | `#ffffff` | 93.2%, 95.2% |

One family in three weights plus italic. The headline is small for a poster (cap height
3.6% H); its weight comes from position, the gray square, and the empty space around it.

## 6. Imagery

A single photograph of repeating architectural structure: a sweeping curve of perforated
metal panels shot from below, converted to black and white with crushed shadows so its
lower part is already black before the fade (T-mono-on-color settings without the color:
`adjust bw`, then `adjust levels --in-black 35 --in-white 215 --gamma 0.9`). Any photo with
a strong abstract pattern and a dark lower edge works: stairs, a bridge underside, a wave,
fabric folds, a crowd from above. No people, no horizon.

## 7. Effects and textures

* **T-photo-fade**, ground color, ramp 26 to 48% H.
* **T-rules**: one short accent rule, two gray square markers.
* No grain, no shadows, no gradients on type: the flatness is the style.

## 8. Hierarchy and eye path

1. The bright curve of the photo (largest, highest-contrast shape) leads down and left.
2. It lands on the gray square and the headline.
3. Date and city, directly below on the same left edge.
4. The lead-in and body; then across to the side column; footer last.

Cap-height ratio: headline 3.6 : date 1.35 : body 1.05 : footer 0.9, about 3.4 : 1.3 : 1 :
0.85. The whole hierarchy is carried by a 3.4x step between headline and everything else.

## 9. Build recipe

`photo.png` is your image here (a wide or portrait photo, at least 1080 px wide; it is
placed at full width from the top edge).

```bash
pixelox new -o conf.pxo --size 1080x1528 --fill '#0b0b0b'
pixelox comp place conf.pxo photo.png --in-place --as Photo --size 1080x --at 0,0
pixelox adjust bw conf.pxo --in-place --layer Photo
pixelox adjust levels conf.pxo --in-place --layer Photo --in-black 35 --in-white 215 --gamma 0.9
pixelox layer add conf.pxo --in-place --name Fade
pixelox fill gradient conf.pxo --in-place --layer Fade --from 0,26% --to 0,48% --stops '#0b0b0b00@0,#0b0b0bff@1'
pixelox shape rect conf.pxo --in-place --name Square --rect 73,591,238x238 --fill '#5e5e5e'
pixelox text add conf.pxo --in-place --name Headline --content $'MA\nKERS\nSUM\nMIT_' --font "Avenir Next" --weight 500 --size 5% --leading 104% --color '#ffffff'
pixelox transform move conf.pxo --in-place --layer Headline --to 73,695 --anchor tl
pixelox text add conf.pxo --in-place --name Date --content "14 March 2031" --font "Avenir Next" --weight 400 --size 1.9% --color '#ffffff' --at 6.8%,69.9% --anchor l
pixelox text add conf.pxo --in-place --name City --content "Your City" --font "Avenir Next" --weight 600 --style italic --size 2% --color '#ffffff' --at 6.8%,72.8% --anchor l
pixelox shape line conf.pxo --in-place --name Rule --from 6.8%,76.4% --to 11.2%,76.4% --stroke '#ffffff' --stroke-width 2
pixelox text add conf.pxo --in-place --name Lead --content "One line that sells it." --font "Avenir Next" --weight 700 --style italic --size 1.45% --color '#ffffff' --at 6.8%,79.9% --anchor l
pixelox text add conf.pxo --in-place --name Body --content $'Say who it is for,\nwhat they will leave with,\nand why this year matters.\nThen ask them to come.' --font "Avenir Next" --weight 400 --size 1.45% --leading 135% --color '#f2f2f2' --at 6.8%,81% --anchor tl
pixelox shape rect conf.pxo --in-place --name Marker1 --rect 674,1047,37x37 --fill '#5e5e5e'
pixelox text add conf.pxo --in-place --name Sub1 --content "What will you learn?" --font "Avenir Next" --weight 700 --style italic --size 1.6% --color '#ffffff' --at 62.4%,70.3% --anchor l
pixelox text add conf.pxo --in-place --name Body1 --content $'Two short lines about the\ntalks, the workshops, and\nthe people in the room.' --font "Avenir Next" --weight 400 --size 1.45% --leading 135% --color '#f2f2f2' --at 62.4%,71.9% --anchor tl
pixelox shape rect conf.pxo --in-place --name Marker2 --rect 674,1271,37x37 --fill '#5e5e5e'
pixelox text add conf.pxo --in-place --name Sub2 --content "How to join?" --font "Avenir Next" --weight 700 --style italic --size 1.6% --color '#ffffff' --at 62.4%,85.1% --anchor l
pixelox text add conf.pxo --in-place --name Body2 --content $'Tell them where to sign up\nand when booking closes.' --font "Avenir Next" --weight 400 --size 1.45% --leading 135% --color '#f2f2f2' --at 62.4%,86.7% --anchor tl
pixelox text add conf.pxo --in-place --name Site --content "www.example.com" --font "Avenir Next" --weight 700 --size 1.3% --tracking 300 --color '#ffffff' --at 6.8%,95.2% --anchor l
pixelox text add conf.pxo --in-place --name Handle --content "#yourtag  @yourhandle" --font "Avenir Next" --weight 400 --size 1.45% --tracking 60 --color '#ffffff' --at 93.2%,95.2% --anchor r
pixelox export conf.pxo -o conf.png
```

Change first: the headline (keep 2 to 5 letters per line; break mid-word on purpose), then
the photo. If the headline has a line wider than the gray square, widen the square to the
longest line of the first two lines, not more.

## 10. Variations and pairings

* One accent color: recolor the square, the markers, and the rule (`#ff4d00` or `#2f6bff`)
  and leave everything else monochrome.
* Light version: ground `#f4f2ee`, type `#111111`, square `#c9c6c0`, photo not inverted.
* T-duotone on the photo (ground color to one accent) for a branded series.
* Lecture, exhibition opening, architecture or photography event, annual report cover,
  album cover.
* Landscape: photo on the right 55% with a sideways fade, the text column on the left.

## 11. Pitfalls

* Centering anything. Every element hangs from the two left edges; one centered line
  breaks the system.
* Making the headline big "because it is a poster". The quiet size is the point; keep the
  cap height under 4.5% H and protect the empty band to its right.
* A photo with a bright lower half: the fade turns it into gray mud. Crush the shadows
  first or choose a photo that is dark at the bottom.
* Body lines longer than 30 characters: the columns are narrow on purpose (22 to 30
  characters per line). Break lines by hand with `$'...\n...'`.
* Pure white square or pure white photo highlights: they outshine the type. Keep the
  square mid-gray and the highlights near `#bab7b7`.
