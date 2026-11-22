---
id: p007
title: Fashion cover with ghost-outline masthead
genre: [magazine-cover, fashion, editorial]
mood: [bold, elegant, serious, warm]
style: [editorial, photo-composite]
format: portrait-4x5
archetype: A-cover-masthead
palette: ['#d4babc', '#8f8c5e', '#b60c10', '#571318', '#ffffff']
techniques: [T-text-behind-subject, T-outline-type, T-gradient-type, T-fit-width, T-paper-grain, T-ghost-type]
needs: subject-cutout
---

# p007: Fashion cover with ghost-outline masthead

## 1. Concept

A fashion-magazine cover: a four-letter `[MASTHEAD]` in giant red capitals runs across the
top behind the cover star's head, and the parts of the letters that the head hides are
drawn again on top of her as a thin white outline, so the name stays readable while the
portrait stays in front. Each letter is a different red, stepping from dark maroon at the
left to bright red at the right, and that red is picked from the star's beret and collar.
Small condensed cover lines hang in the left and right margins.

## 2. Format and grid

* Portrait 4:5. Margins 5% W left and 5.5% W right; top line at 3% H, footer line at
  97.5% H.
* Masthead zone 7 to 37% H, spanning 5.5 to 94.5% W (89% W): one word, four letters, cap
  height 30% H.
* Portrait: head and shoulders, a little left of center (head axis at 48% W), beret top at
  11% H, shoulders running off the left and bottom edges and ending at 78% W on the right.
  The head covers the two middle letters; the first and last letters are fully visible.
* Two cover-line columns, both left-aligned: left column at x 5% W (over the shoulder),
  right column at x 78% W (over the empty ground beside the neck). Lines sit between 47
  and 86% H, never on the face.
* Barcode block bottom right: `65%,86%,29.5% x 8.5%`.

## 3. Layer stack

1. Ground: dusty pink `#d4babc`, full canvas, with a faint paper grain.
2. Issue line: small dark maroon text, top left, 5% W, 3.2% H.
3. Masthead: four heavy condensed capitals, 5.5 to 94.5% W, 7.1 to 36.6% H. Letter colors
   from left to right: `#571318`, `#7a1418`, `#b60c10`, `#c8101a` (flat per letter).
4. Tagline: one white sentence at 5% W, 39.2% H, running under the masthead's first two
   letters and disappearing behind the head.
5. Cover star cutout: a woman in three-quarter profile facing right, red beret, oversized
   yellow-tinted aviator glasses, olive trench coat with epaulettes over a red collar.
   Natural color, soft even light, no shadow on the ground.
6. Masthead outline: the same word in the same place, no fill, white 2 px stroke, visible
   only where the cutout is (the hidden halves of the two middle letters).
7. Left trio: three one-word lines at 5% W, 47.8 / 50.3 / 52.8% H, maroon, white, maroon.
8. Left headline: two white lines at 5.1% W, cap tops 74.9% and 78.6% H, on the coat;
   two tiny white lines under it at 83% H.
9. Right headline 1: two maroon lines at 78% W, cap tops 49.2% and 53.6% H; three tiny
   white lines under it at 58% H.
10. Right headline 2: two maroon lines at 78% W, cap tops 70.2% and 73.6% H; two tiny
    white lines at 78.3% H.
11. Barcode: white plate with a 4 px red border and red bars.
12. Footer: one tiny, widely tracked white line centered at 97.4% H.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#d4babc` | dusty pink ground | 40% |
| `#8f8c5e` | olive coat (photo) | 22% |
| `#b60c10` | red: beret, collar, right half of the masthead, barcode | 15% |
| `#571318` | maroon: left half of the masthead, cover lines | 10% |
| `#4a2a1a` | skin shadow, hair (photo) | 8% |
| `#ffffff` | outline, white cover lines, barcode plate | 3% |
| `#d9b21f` | yellow lenses (photo) | 2% |

Red and its complement olive green, both muted by a pink ground that is simply the red at
low saturation. The yellow lenses are the single accent and sit at the exact center.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Masthead | `[MASTHEAD]`, 4 letters | caps | heavy condensed grotesque (Anton, Oswald 700 / Impact) | 400 (the face is heavy) | 30% | 0 | - | center | 4 reds, dark to bright | 50%, cap top 7.1% |
| Issue line | volume, issue, month year | title | geometric sans (Montserrat / Avenir Next) | 500 | 1% | 0 | - | left | `#4a1a1a` | 5%, 3.2% |
| Tagline | one sentence, 6 words | sentence | same | 500 | 1.1% | 0 | - | left | `#ffffff` | 5%, 39.2% |
| Left trio | 3 one-word lines, 10 to 13 letters, all starting with the same prefix | caps | condensed bold gothic (Oswald, Bebas Neue / DIN Condensed) | 700 | 1.2% | 40 | 2.5% H pitch | left | maroon / white / maroon | 5%, 47.8% to 52.8% |
| Left headline | 2 lines, 2 to 3 short words | caps | same | 700 | 2.4% | 0 | 112% | left | `#ffffff` | 5.1%, cap top 74.9% |
| Right headline 1 | 2 lines, 7 to 9 characters each | caps | same | 700 | 3.05% | 0 | 105% | left | `#571318` | 78%, cap top 49.2% |
| Right headline 2 | 2 lines, 7 to 11 characters each | caps | same | 700 | 2.7% | 0 | 105% | left | `#571318` | 78%, cap top 70.2% |
| Sub lines (x3) | 2 to 3 lines, 3 to 4 words each | sentence | geometric sans (Montserrat / Avenir Next) | 500 | 0.8% | 0 | 130% | left | `#ffffff` | under each headline, 1.5% H below |
| Footer | 5-word descriptor of the publication | title | same | 600 | 0.8% | 300 | - | center | `#ffffff` | 50%, 97.4% |

Two families: one heavy condensed gothic in three sizes (30 : 3 : 1.2) and one geometric
sans for everything tiny. White and maroon alternate so neighboring blocks never share a
color.

## 6. Imagery

One studio portrait, cut out: head turned 30 degrees to the right, chin level, mouth
closed, eyes hidden behind large tinted lenses; lit softly from the front left so the
skin has a gentle sheen and the hat keeps its shape. The crop is tight: the head is 52% W
wide and the shoulders bleed off two edges, which makes the figure feel larger than the
page. Clothing carries the palette (red hat and collar, olive coat). For a build, choose
a subject with one strong accessory in the masthead color and place the head over the
middle of the word (**T-text-behind-subject**).

## 7. Effects and textures

* **T-text-behind-subject**: masthead first, cutout second.
* Ghost outline (**T-outline-type** plus a mask): a second copy of the masthead with
  `--color transparent --stroke '#ffffff' --stroke-width 2px`, placed above the cutout
  and masked with `mask add --from @alpha:Subject`, so the outline exists only on the
  subject.
* Per-letter color (**T-gradient-type** with hard stops): `fx overlay --angle 0` and each
  color written twice, at the start and the end of its letter (stops near 0.24, 0.5,
  0.76 for four letters of similar width).
* **T-fit-width**: the masthead is scaled to 89% W.
* Barcode: two `pattern.js` stripe renders of different pitch laid over each other on a
  white plate; the mismatch gives uneven bar widths.
* **T-paper-grain**, amount 12, soft-light, 35%.
* **T-ghost-type** in [techniques.md](../techniques.md) is the tested library version of the faint, defocused, or outlined giant word; start from it when reusing this idea.

## 8. Hierarchy and eye path

1. The masthead and the red hat read together as one red mass at the top.
2. The yellow lenses and the face (center, the only yellow).
3. Right headline 1 (largest cover line, at eye level beside the face), then right
   headline 2 below it.
4. Left trio and left headline; barcode and footer last.

Cap-height ratio: masthead 30 : main cover line 3 : small cover line 1.2 : sub text 0.8.
The 10x jump from masthead to cover lines is what makes it read as a magazine and not a
poster.

## 9. Build recipe

`subject.png` is your image here: a head-and-shoulders cutout, at least 1300 px tall.
Move it sideways until the head covers the two middle letters.

```bash
pixelox render js scripts/pattern.js -o bars1.png --size 290x92 --param kind=stripes --param color='#b60c10' --param cell=9 --param weight=0.45
pixelox render js scripts/pattern.js -o bars2.png --size 290x92 --param kind=stripes --param color='#b60c10' --param cell=23 --param weight=0.3
pixelox new -o cover.pxo --size 1080x1350 --fill '#d4babc'
pixelox text add cover.pxo --in-place --name Masthead --content "ECHO" --font Impact --size 30% --color '#b60c10'
pixelox transform scale cover.pxo --in-place --layer Masthead --size 962x
pixelox transform move cover.pxo --in-place --layer Masthead --to 50%,7.1% --anchor t
pixelox fx overlay cover.pxo --in-place --layer Masthead --stops '#571318@0,#571318@0.24,#7a1418@0.24,#7a1418@0.5,#b60c10@0.5,#b60c10@0.76,#c8101a@0.76,#c8101a@1' --angle 0
pixelox text add cover.pxo --in-place --name IssueLine --content "Volume 3, Issue 7, March 2031" --font "Avenir Next" --weight 500 --size 1.5% --color '#4a1a1a' --at 5%,3.2% --anchor l
pixelox text add cover.pxo --in-place --name Tagline --content "Quiet voices that carry the furthest" --font "Avenir Next" --weight 500 --size 1.6% --color '#ffffff' --at 5%,39.2% --anchor l
pixelox comp place cover.pxo subject.png --in-place --as Subject --size x1300 --at 42%,100% --anchor b
pixelox text add cover.pxo --in-place --name MastOutline --content "ECHO" --font Impact --size 30% --color transparent --stroke '#ffffff' --stroke-width 2px
pixelox transform scale cover.pxo --in-place --layer MastOutline --size 962x
pixelox transform move cover.pxo --in-place --layer MastOutline --to 50%,7.1% --anchor t
pixelox mask add cover.pxo --in-place --layer MastOutline --from @alpha:Subject
pixelox text add cover.pxo --in-place --name Trio1 --content "UNHURRIED" --font "DIN Condensed" --weight 700 --size 1.9% --tracking 40 --color '#571318' --at 5%,47.8% --anchor l
pixelox text add cover.pxo --in-place --name Trio2 --content "UNBOTHERED" --font "DIN Condensed" --weight 700 --size 1.9% --tracking 40 --color '#ffffff' --at 5%,50.3% --anchor l
pixelox text add cover.pxo --in-place --name Trio3 --content "UNMISTAKABLE" --font "DIN Condensed" --weight 700 --size 1.9% --tracking 40 --color '#571318' --at 5%,52.8% --anchor l
pixelox text add cover.pxo --in-place --name LeftHead --content $'CALM IS\nTHE NEW LOUD' --font "DIN Condensed" --weight 700 --size 3.3% --leading 112% --color '#ffffff'
pixelox transform move cover.pxo --in-place --layer LeftHead --to 5.1%,74.9% --anchor tl
pixelox text add cover.pxo --in-place --name LeftSub --content $'Dressing slowly\nin a fast decade' --font "Avenir Next" --weight 500 --size 1.15% --leading 130% --color '#ffffff' --at 5.1%,83% --anchor tl
pixelox text add cover.pxo --in-place --name RightHead1 --content $'IN A YEAR\nOF HASTE' --font "DIN Condensed" --weight 700 --size 4.2% --leading 105% --color '#571318'
pixelox transform move cover.pxo --in-place --layer RightHead1 --to 78%,49.2% --anchor tl
pixelox text add cover.pxo --in-place --name RightSub1 --content $'Twelve designers on why\nthey stopped chasing\nthe next big thing' --font "Avenir Next" --weight 500 --size 1.15% --leading 130% --color '#ffffff' --at 78%,58% --anchor tl
pixelox text add cover.pxo --in-place --name RightHead2 --content $'THE UNIFORM\nRETURNS' --font "DIN Condensed" --weight 700 --size 3.7% --leading 105% --color '#571318'
pixelox transform move cover.pxo --in-place --layer RightHead2 --to 78%,70.2% --anchor tl
pixelox text add cover.pxo --in-place --name RightSub2 --content $'Khaki, brass buttons,\nand one red accent' --font "Avenir Next" --weight 500 --size 1.15% --leading 130% --color '#ffffff' --at 78%,78.3% --anchor tl
pixelox shape rect cover.pxo --in-place --name CodePlate --rect 65%,86%,29.5%x8.4% --fill '#ffffff' --stroke '#b60c10' --stroke-width 4
pixelox comp place cover.pxo bars1.png --in-place --as Bars1 --at 79.75%,90.2% --anchor c
pixelox comp place cover.pxo bars2.png --in-place --as Bars2 --at 79.75%,90.2% --anchor c
pixelox text add cover.pxo --in-place --name Footer --content "Style Journal of the Decade" --font "Avenir Next" --weight 600 --size 1.2% --tracking 300 --color '#ffffff' --at 50%,97.4% --anchor c
pixelox layer add cover.pxo --in-place --name Grain --fill '#808080'
pixelox filter add-noise cover.pxo --in-place --layer Grain --amount 12 --monochrome
pixelox layer blend cover.pxo --in-place --layer Grain --mode soft-light
pixelox layer opacity cover.pxo --in-place --layer Grain --value 35
pixelox export cover.pxo -o cover.png
```

Change first: the masthead word (4 to 5 letters; with letters of unequal width move the
three inner stop positions to the gaps between the letters), then the four reds (sample
them from the subject's clothing), then the cover lines. The white sub lines are readable
only on a mid-tone ground like this one; on a paler ground set them in the maroon.

## 10. Variations and pairings

* Other "color from the wardrobe" sets: cobalt hat on a pale blue ground `#c9d6e6` with
  a navy-to-cobalt masthead; mustard on cream `#efe6cf` with brown-to-mustard letters;
  emerald on mint.
* One flat masthead color instead of four: calmer, more classic.
* Full outline masthead (no fill anywhere) over a full-bleed photo with T-photo-fade at
  the bottom for the cover lines.
* Works for a profile poster, a podcast cover, a lookbook front, a speaker announcement
  (masthead becomes the event name, cover lines become date and place).
* Pairs with T-frame-border (thin keyline 5% in), T-mono-on-color for the portrait with
  only the hat left in color, and T-label-box behind one cover line.

## 11. Pitfalls

* The head hides the whole word. With four letters the head may cover two; with six or
  more it may cover three. First and last letters stay clear.
* A thick ghost outline. Over 3 px it competes with the face; 2 px white (or the ground
  color) is enough because the eye completes the letters.
* Outline visible over the ground too. Without the `@alpha:Subject` mask the outline
  rings the filled letters and the trick is lost.
* Cover lines on the face or across the hat. Keep both columns in the outer 22% of the
  width and below the chin line.
* Too many reds elsewhere. Cover lines use only the darkest maroon and white; the bright
  reds belong to the masthead, the clothing, and the barcode.
