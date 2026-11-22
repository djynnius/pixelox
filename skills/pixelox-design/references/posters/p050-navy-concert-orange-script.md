---
id: p050
title: Navy concert flyer with orange script
genre: [church, concert, event-flyer]
mood: [moody, dark, warm, energetic]
style: [photo-composite, duotone]
format: portrait-2x3
archetype: A-top-photo-bottom-text
palette: ['#02152d', '#141f31', '#1179e0', '#254054', '#e48020', '#ffffff']
techniques: [T-photo-fade, T-script-overlap, T-pattern, T-color-grade]
needs: [photo, logo]
---

# p050: Navy concert flyer with orange script

## 1. Concept

A worship-concert flyer in three colors: a crowd photo drowned in navy fills the top half
and fades to flat navy; one big orange script word sweeps across the middle with a white
slab-serif word tucked into it; a bright blue panel with a rough painted top edge closes
the bottom and carries the venue. It works because orange is the exact complement of the
blue ground and is used for only two things (the script word and the time), so the eye
goes to them first, and the photo is dark enough to act as a texture, not a picture.

## 2. Format and grid

* Portrait 2:3. Center axis at 50% W; side margins about 12% W for the title, 22% W for the
  small lines. Nothing is left- or right-aligned except the logo row at the top.
* Zones: logo row 1 to 5% H; photo 0 to 52% H (already half faded by 40% H); title lockup
  42 to 69% H; date line 73.6% H; blue panel from about 81% H to the bottom edge.
* The title lockup sits in the lower middle, *below* the photo's subject, not on it: the
  raised arm in the photo points up-left, the script word climbs up-right, and the two
  diagonals cross in the empty navy between them.
* The panel's top edge is not level: it peaks at 79% H near 35% W, is at 83.5% H on the
  left edge and 87% H on the right edge (a tilt of about 3 degrees, falling to the right).

## 3. Layer stack

1. Ground: navy `#011837`, full canvas.
2. Photo, full width, top edge to about 83% H: a concert crowd from behind, one man in the
   middle with his right arm stretched to the upper-left corner, other heads and hands
   out of focus at the right. Darkened and covered with navy at about 78%, so highlights
   are only `#1c3d52` to `#254054` (dull teal) and a trace of the original warm skin
   color survives at the top left.
3. Screen texture: a fine black dot grid (pitch 0.75% W), 35% opaque at the top edge and
   shrinking to nothing by mid-height; it gives the dark photo a printed, LED-wall grain.
4. Fade: ground navy from transparent at 34% H to solid at 56% H.
5. Blue panel `#0e79e1` (highlights `#58a2e9` where it was sampled through text): solid
   from 88% H down; above that a rough, torn-looking upper edge with spatter drops flying
   up to 3% H above it and navy pits 2% H into it.
6. Logo box: white outline square, 5.6% W, at 23% W, 1% H; presenter name in white caps
   to its right, starting at 31% W, centered on 2.9% H.
7. Block word: white slab-serif caps, 31 to 77.5% W, cap top 58.7% H, baseline 63.6% H.
8. Script word: orange `#f28a1e`, bounding box 12 to 93.5% W, 41.8 to 69% H, climbing about
   6 degrees to the right, drawn *over* the block word (its descenders cross two of the
   white letters), with a soft black shadow offset 0.7% W right and down.
9. Date (white) and time (orange) on one line at 73.6% H, split at 45.5% W with a 4.7% W
   gap: date right-aligned to 43.3% W, time left-aligned from 48% W.
10. On the panel: venue name in white caps at 90% H; address in small caps at 93.5% H; web
    address in lowercase at 97.7% H; all centered.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#02152d` | navy ground and the fade | 55% |
| `#141f31` | darkened photo (navy with a little warmth) | 20% |
| `#1179e0` | bright blue panel | 15% |
| `#254054` | photo highlights (dull teal) | 5% |
| `#e48020` | orange: script word and time only | 3% |
| `#ffffff` | block word, small text, logo box | 2% |

A blue monochrome (navy, teal, bright blue) plus its complement, orange, at about 3% of
the area. The 55/15/3 split between dark blue, bright blue, and orange is the whole color
idea.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Presenter | `[CHURCH NAME]`, 3 words | caps | humanist slab serif (Aleo, Arvo / Rockwell) | 400 | 1.9% | 0 | - | left | `#ffffff` | 31%, 2.9% |
| Script word | one word, 6 letters, capital with a long entry stroke | title | flowing connected script (Sacramento, Playball, Great Vibes / Snell Roundhand) | heavy cut | x-height 6.5%, whole word 27% tall | 0 | - | center | `#f28a1e` | center 52.7%, 55.5% |
| Block word | "CONCERT", 7 letters | caps | humanist slab serif (Aleo, Arvo / Rockwell) | 400 to 500 | 4.9% | 0 | - | center | `#ffffff` | 54.2%, cap top 58.7% |
| Date | "4TH APRIL"-style, 2 words | caps | slab serif | 400 | 1.9% | 0 | - | right | `#ffffff` | 43.3%, 73.6% |
| Time | "FROM 10:00AM"-style | caps | slab serif | 400 | 1.9% | 0 | - | left | `#f28a1e` | 48%, 73.6% |
| Venue | hall name, 2 words | caps | slab serif | 400 | 2.9% | 0 | - | center | `#ffffff` | 50%, 90% |
| Address | street, number, city | caps | slab serif | 400 | 0.95% | 0 | - | center | `#ffffff` | 50%, 93.5% |
| Web | one web address | lower | slab serif | 400 | 1.3% | 0 | - | center | `#ffffff` | 50%, 97.7% |

Two families: one slab serif at regular weight throughout, and one script. The script word is 81% W wide while the block word under it is 46% W: the script
is the headline and the block word its caption, the reverse of the usual pairing.

## 6. Imagery

One photograph, used as atmosphere: a low-light shot from inside a standing crowd, the
nearest figure seen from behind with an arm raised, cool stage haze beyond. It is
darkened (`adjust levels` gamma 0.85), covered by a navy layer at 78% (a cheap, soft
duotone that keeps a hint of the original color; see T-color-grade for a cleaner
version), and faded into the ground with T-photo-fade. Faces are not visible and do not
need to be. Any dim photo with one clear gesture in the upper half works: hands on a
keyboard, a singer at a microphone, a choir row.

## 7. Effects and textures

* **T-photo-fade**: ground navy, ramp 34 to 56% H, so the title sits on flat color.
* **T-pattern**: `kind=dots`, `cell=8`, `fade=down`, black at 35%, over the photo only.
* **T-script-overlap**: here the script is the larger partner (about 3.5x the block word's
  cap height), rotated -6 degrees, placed over the block word so that its lower loops
  cross the white letters; shadow offset 8,10 px, blur 14, 55%.
* Rough panel edge: a `torn.js` strip in panel blue (no fiber rim), rotated 3 degrees,
  over a solid blue rectangle; plus two narrow `grunge.js` bands, blue specks above the
  edge and navy specks below it. The original is a true dry-brush edge with horizontal
  streaks; this is the nearest build with the bundled generators.
* No grain layer; the dot screen does that job.

## 8. Hierarchy and eye path

1. The orange script word (only warm color, largest shape, on flat navy).
2. The white block word inside it.
3. Date and time just below; the orange time is the second warm spot.
4. The blue panel and the venue name.
5. The photo and the presenter row at the top are background and signature.

Size ratio: script word (x-height 6.5% H) : block word 4.9 : venue 2.9 : date and
presenter 1.9 : web 1.3 : address 0.95, about 7 : 5 : 3 : 2 : 1.4 : 1.

## 9. Build recipe

`photo.png` (a dim crowd or stage photo) and `logo.png` (a square mark; the original shows
an empty outlined square as the logo placeholder) are your images here.

```bash
pixelox render js scripts/torn.js -o edge.png --size 1300x420 --seed 4 --param color='#0e79e1' --param rim=none --param edges=t --param rough=1 --param detail=1
pixelox render js scripts/grunge.js -o spatter.png --size 1080x110 --seed 6 --param color='#0e79e1' --param specks=500 --param blotches=70 --param scratches=0 --param clump=0.7 --param size=1.6 --param alpha=1
pixelox render js scripts/grunge.js -o pits.png --size 1080x90 --seed 9 --param color='#011837' --param specks=400 --param blotches=60 --param scratches=0 --param clump=0.7 --param size=1.6 --param alpha=1
pixelox render js scripts/pattern.js -o screen.png --size 1080x1620 --param kind=dots --param color='#000000' --param cell=8 --param weight=0.5 --param fade=down --param alpha=0.35
pixelox new -o concert.pxo --size 1080x1620 --fill '#011837'
pixelox comp place concert.pxo photo.png --in-place --as Photo --size 1080x --at 0,0
pixelox adjust levels concert.pxo --in-place --layer Photo --in-black 20 --gamma 0.85
pixelox layer add concert.pxo --in-place --name Tint --fill '#02152d'
pixelox layer opacity concert.pxo --in-place --layer Tint --value 78
pixelox comp place concert.pxo screen.png --in-place --as Screen --fit cover
pixelox layer add concert.pxo --in-place --name Fade
pixelox fill gradient concert.pxo --in-place --layer Fade --from 0,34% --to 0,56% --stops '#01183700@0,#011837ff@1'
pixelox shape rect concert.pxo --in-place --name PanelBase --rect 0,88%,100%x12% --fill '#0e79e1'
pixelox comp place concert.pxo edge.png --in-place --as PanelEdge --size 1300x --at 50%,80.5% --anchor t
pixelox transform rotate concert.pxo --in-place --layer PanelEdge --degrees 3
pixelox comp place concert.pxo spatter.png --in-place --as Spatter --at 50%,81.5% --anchor b
pixelox transform rotate concert.pxo --in-place --layer Spatter --degrees 3
pixelox comp place concert.pxo pits.png --in-place --as Pits --at 50%,83.5% --anchor t
pixelox transform rotate concert.pxo --in-place --layer Pits --degrees 3
pixelox comp place concert.pxo logo.png --in-place --as Logo --size 62x --at 23%,1%
pixelox text add concert.pxo --in-place --name Presenter --content "YOUR CHURCH NAME" --font Rockwell --weight 400 --size 2.7% --color '#ffffff' --at 31%,2.9% --anchor l
pixelox text add concert.pxo --in-place --name Block --content "CONCERT" --font Rockwell --weight 400 --size 7% --color '#ffffff'
pixelox transform scale concert.pxo --in-place --layer Block --size 502x
pixelox transform move concert.pxo --in-place --layer Block --to 54.2%,58.7% --anchor t
pixelox text add concert.pxo --in-place --name Script --content "Spring" --font "Snell Roundhand" --weight 900 --size 20% --color '#f28a1e'
pixelox transform scale concert.pxo --in-place --layer Script --size 880x
pixelox transform rotate concert.pxo --in-place --layer Script --degrees -6
pixelox transform move concert.pxo --in-place --layer Script --to 52.7%,55% --anchor c
pixelox fx drop-shadow concert.pxo --in-place --layer Script --offset 8,10 --blur 14 --opacity 55
pixelox text add concert.pxo --in-place --name Date --content "9TH MAY" --font Rockwell --weight 400 --size 2.75% --color '#ffffff' --at 43.3%,73.6% --anchor r
pixelox text add concert.pxo --in-place --name Time --content "FROM 6:00PM" --font Rockwell --weight 400 --size 2.75% --color '#f28a1e' --at 48%,73.6% --anchor l
pixelox text add concert.pxo --in-place --name Venue --content "TOWN HALL" --font Rockwell --weight 400 --size 4.2% --color '#ffffff' --at 50%,90% --anchor c
pixelox text add concert.pxo --in-place --name Address --content "12 EXAMPLE ROAD, 1234, YOUR CITY" --font Rockwell --weight 400 --size 1.4% --color '#ffffff' --at 50%,93.5% --anchor c
pixelox text add concert.pxo --in-place --name Web --content "www.example.com" --font Rockwell --weight 400 --size 2% --color '#ffffff' --at 50%,97.7% --anchor c
pixelox export concert.pxo -o concert.png
```

Change first: the script word (5 to 7 letters, with a looped capital and at least one
descender, such as g, j, p, or y, so the word is tall enough to wrap the block word),
then check that its lower strokes cross the block word and
do not hide a whole letter; nudge the block word's x by a few percent if they do. Then the
two accent uses of orange, then the panel blue.

## 10. Variations and pairings

* Palette swaps that keep the complement logic: plum ground `#1d0b2e`, panel `#7a2bd6`,
  script yellow `#ffc400`; forest ground `#06231b`, panel `#0f9d58`, script coral `#ff6b4a`;
  charcoal ground `#141414`, panel `#e10600`, script white with the block word in red.
* Swap the pairing for a jazz night, open mic, carol service, album launch, or dance
  social: the structure needs only a dim photo and two words.
* Square post: photo 0 to 45% H, lockup 38 to 72% H, panel from 82% H with venue and date
  on one line.
* Replace the rough panel with T-info-bar (three columns) when there are three facts, or
  with T-torn-collage scraps for a handmade look.
* Pairs with T-duotone for a cleaner two-ink photo, T-light-leak in orange from the top
  right corner, and T-paper-grain over everything.

## 11. Pitfalls

* A bright or colorful photo under only a 50% tint: it competes with the script. The photo
  must end up darker than `#254054` everywhere; raise the Tint opacity until it does.
* Orange used a third and fourth time (venue name, web address, logo): the accent stops
  being an accent. Two uses, both in the middle band.
* A script with thin hairlines at this size on navy disappears in print. Use the heaviest
  cut, or add `fx stroke --width 2` in the same orange.
* The block word placed on top of the script: the lockup goes flat. The script must be the
  upper layer, and its shadow must fall on the white letters.
* A perfectly straight panel edge reads as a web banner. Keep the tilt (2 to 4 degrees)
  and the specks on both sides of the edge.
