---
id: p009
title: Teal ring corporate flyer with round photo
genre: [corporate, tech-ad, conference]
mood: [fresh, calm, friendly, cool]
style: [geometric, minimal, swiss]
format: portrait-a
archetype: A-top-photo-bottom-text
palette: ['#fafafa', '#7fc4bf', '#ececec', '#121315', '#cdd1d8']
techniques: [T-geo-shapes, T-rules, T-mono-on-color, T-ring-arcs]
needs: [photo, logo]
---

# p009: Teal ring corporate flyer with round photo

## 1. Concept

A one-page company flyer on white: an office photo cropped to a circle, wrapped by a
thick teal ring that is three-quarters complete, with a heavy two-line black headline
under it and two tiny info columns at the bottom, the second one sitting on a teal disc.
It works because every shape is a circle or an arc of one (ring, photo, hairlines, disc,
bullet dots), in a single accent color, so a plain stock photo and a plain message look
like a designed system. Shown as a sheet lying on a teal backdrop; the backdrop is not
part of the design.

## 2. Format and grid

* Portrait A-series (1 : 1.414). Left text margin 9.5% W; right margin 8% W; bottom 5% H.
* Zones: circle composition 3 to 66% H; headline 68 to 80% H; info columns 86 to 96% H.
* The ring's center is at 48% W, 34% H; its outer radius is 43.5% W, so it nearly touches
  the left margin and its right side leaves room for the logo at the top right.
* Two bottom columns: left edge 9.5% W and left edge 57.5% W. The headline hangs from the
  first of them; the teal disc is centered under the second.
* All text is left-aligned. Nothing is centered except the photo in its ring.

## 3. Layer stack

1. Ground: off-white `#fafafa`.
2. Ghost ring: a full ring, center 48.2% W, 34% H, outer radius 43.5% W, band width 9% W,
   pale gray `#ececec`. Only its upper right quarter stays visible.
3. Teal ring: the same ring in teal `#7fc4bf`, drawn from the 12:30 position
   counter-clockwise through 9 and 6 o'clock to 3:30 (about 270 degrees), with square
   ends. It covers the ghost ring except in the upper right.
4. Photo disc: radius 30.8% W, center 51.4% W, 31.5% H: set 3% W up and to the right of
   the ring's center, so the white gap between photo and ring is a crescent, widest at
   the lower left (5% W) and closed at the upper right.
5. Photo, clipped to the disc: a bright, nearly colorless office scene.
6. Hairline circles, 1.5 px: one teal circle of radius 56% W around the composition
   (center 50% W, 33% H; only three arcs of it fall on the page, at the top left, top
   right, and lower left); one white circle of radius 27% W inside the photo, 70%
   opacity; one teal circle of radius 11.6% W cut by the right edge at 65% H.
7. Teal disc: radius 24% W, center 73% W, 92.6% H, lighter teal `#85cbc6`, cut by the
   bottom edge. Its top passes behind the last letters of the headline's second line.
8. Logo: a small teal mark, 5.5% W wide, at 90% W, 7.5% H, with a two-word label in tiny
   bold black caps under it at 11.8% H.
9. Headline: two lines of heavy black caps, left at 9.5% W, cap tops 68% and 75.3% H;
   line 2 spans 83% W.
10. Column 1: a teal dot and a teal bold label at 86.9% H; three lines of black body
    text at 90.5%, 92.6%, 94.7% H.
11. Column 2 (on the disc): a white dot and a white bold label at 57.5% W, 86.9% H; three
    white lines, each a bold field name followed by a regular value.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#fafafa` | ground | 50% |
| `#7fc4bf` | ring, disc (`#85cbc6`), labels, hairlines | 25% |
| `#cdd1d8` | photo mid-tones (cool gray) | 15% |
| `#ececec` | ghost ring | 5% |
| `#121315` | headline and body | 5% |

White plus one soft accent plus black type. The photo is pulled almost to gray so that the
teal is the only color; 50 / 25 / 25.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Logo label | `[BRAND]`, 2 short words | caps | heavy geometric sans (Montserrat 800 / Avenir Next) | 700 | 0.9% | 40 | - | center | `#121315` | 90%, 11.8% |
| Headline | 3 words on 2 lines: 10 letters, then 3 + 10 letters | caps | heavy wide geometric sans (Montserrat 800 / Avenir Next Heavy) | 900 | 4.3% | 10 | line gap 70% of cap height | left | `#121315` | 9.5%, cap tops 68% and 75.3% |
| Column label (x2) | 1 word, 5 to 7 letters | caps | same | 700 | 1.8% | 10 | - | left | `#7fc4bf` / `#ffffff` | 9.5% and 57.5%, 86.9% |
| Body | 3 lines, 6 to 7 words each, 38 to 42 characters | sentence | neo-grotesque (Roboto 400 / Helvetica Neue) | 400 | 0.9% | 0 | 168% (pitch 2.1% H) | left | `#121315` | 9.5%, lines at 90.5%, 92.6%, 94.7% |
| Contact lines | 3 lines: field name + value (address, phone, website) | title | same, field name 700 | 400 | 0.9% | 0 | 168% (pitch 2.1% H) | left | `#ffffff` | 57.5%, same three lines |

Headline to label to body is 4.8 : 2 : 1. The headline is small for a flyer (4.3% H) but
it is the only black mass on the page.

## 6. Imagery

One photograph: a tidy desk by a window (a monitor seen from behind, a laptop, a plant,
city buildings outside), daylight, no people. Crop: the disc shows the middle of the
photo, objects cut by the circle on all sides. Treatment: saturation pulled down by 75,
gamma raised to 1.25 and blacks lifted (`adjust levels --out-black 30`), so it is pale and
cool and cannot compete with the ring (the T-mono-on-color idea, kept just short of full
black and white). Any calm workplace, product, or architecture photo works.

## 7. Effects and textures

* **T-geo-shapes**: all arcs are `shape path` strokes with `--fill none`. A partial ring
  is an open arc with a thick stroke: `M x1 y1 A r r 0 1 0 x2 y2` (large-arc 1, sweep 0
  for the long way round, counter-clockwise), where r is the ring's middle radius.
* Photo in a circle: `shape ellipse` disc, photo placed above it, `layer clip`.
* **T-rules**: hairline circles (1.5 px) and bullet dots before the two labels.
* Flat color only. No shadows, no gradients, no grain.
* **T-ring-arcs** in [techniques.md](../techniques.md) is the tested library version of the ring, halo, or arc shapes; start from it when reusing this idea.

## 8. Hierarchy and eye path

1. The ring and the photo inside it (60% of the page, the only image).
2. The black headline directly below (heaviest type).
3. The teal label and body at the lower left.
4. The teal disc with the contact lines; the logo at the top right last.

The ring's open quarter points at the logo; its closed lower left sweeps the eye down into
the headline.

## 9. Build recipe

`photo.png` (a bright workplace or product photo) and `logo.png` are your images here.
Canvas 1080x1528.

```bash
pixelox new -o flyer.pxo --size 1080x1528 --fill '#fafafa'
pixelox shape path flyer.pxo --in-place --name GhostRing --d 'M 99 520 A 422 422 0 1 1 943 520 A 422 422 0 1 1 99 520 Z' --fill none --stroke '#ececec' --stroke-width 97
pixelox shape path flyer.pxo --in-place --name TealRing --d 'M 630 112 A 422 422 0 1 0 929 629' --fill none --stroke '#7fc4bf' --stroke-width 97
pixelox shape ellipse flyer.pxo --in-place --name PhotoDisc --rect 222,148,666x666 --fill '#d9dde2'
pixelox comp place flyer.pxo photo.png --in-place --as Photo --size 760x --at 555,481 --anchor c
pixelox layer clip flyer.pxo --in-place --layer Photo
pixelox adjust hsl flyer.pxo --in-place --layer Photo --saturation=-75
pixelox adjust levels flyer.pxo --in-place --layer Photo --gamma 1.25 --out-black 30
pixelox shape path flyer.pxo --in-place --name Orbit --d 'M -65 505 A 605 605 0 1 1 1145 505 A 605 605 0 1 1 -65 505 Z' --fill none --stroke '#7fc4bf' --stroke-width 1.5
pixelox shape ellipse flyer.pxo --in-place --name InnerLine --rect 250,180,580x580 --fill none --stroke '#ffffff' --stroke-width 1.5 --opacity 70
pixelox shape path flyer.pxo --in-place --name SideCircle --d 'M 993 989 A 126 126 0 1 1 1245 989 A 126 126 0 1 1 993 989 Z' --fill none --stroke '#7fc4bf' --stroke-width 1.5
pixelox shape path flyer.pxo --in-place --name Disc --d 'M 529 1415 A 261 261 0 1 1 1051 1415 A 261 261 0 1 1 529 1415 Z' --fill '#85cbc6'
pixelox comp place flyer.pxo logo.png --in-place --as Logo --size 60x --at 90%,7.5% --anchor c
pixelox text add flyer.pxo --in-place --name LogoLabel --content "NORTH DESK" --font @geometric-sans --weight 700 --size 1.25% --tracking 40 --color '#121315' --at 90%,11.8% --anchor c
pixelox text add flyer.pxo --in-place --name H1 --content "BUILT FOR" --font @geometric-sans --weight 900 --size 6% --tracking 10 --color '#121315' --at 9.5%,68% --anchor tl --anchor-to ink
pixelox text add flyer.pxo --in-place --name H2 --content "BOLD THINKERS" --font @geometric-sans --weight 900 --size 6% --tracking 10 --color '#121315' --at 9.5%,75.3% --anchor tl --anchor-to ink
pixelox shape ellipse flyer.pxo --in-place --name Dot1 --rect 7.1%,86.5%,1.2%x0.85% --fill '#7fc4bf'
pixelox text add flyer.pxo --in-place --name Label1 --content "ABOUT" --font @geometric-sans --weight 700 --size 2.5% --tracking 10 --color '#7fc4bf' --at 9.5%,86.9% --anchor l
pixelox text add flyer.pxo --in-place --name Body --content $'We plan, build, and run the software\nthat keeps small teams moving, from\nthe first sketch to the day it ships.' --font @neo-grotesque --weight 400 --size 1.25% --leading 168% --color '#121315' --at 9.5%,89.6% --anchor tl
pixelox shape ellipse flyer.pxo --in-place --name Dot2 --rect 55.1%,86.5%,1.2%x0.85% --fill '#ffffff'
pixelox text add flyer.pxo --in-place --name Label2 --content "CONTACT" --font @geometric-sans --weight 700 --size 2.5% --tracking 10 --color '#ffffff' --at 57.5%,86.9% --anchor l
pixelox text add flyer.pxo --in-place --name Fields --content $'Address\nPhone\nWebsite' --font @neo-grotesque --weight 700 --size 1.25% --leading 168% --color '#ffffff' --at 57.5%,89.6% --anchor tl
pixelox text add flyer.pxo --in-place --name Values --content $'12 Example Road, Your City\n000 555 0100\nwww.example.com' --font @neo-grotesque --weight 400 --size 1.25% --leading 168% --color '#ffffff' --at 64.8%,89.6% --anchor tl
pixelox export flyer.pxo -o flyer.png
```

Change first: the photo and its position inside the disc (`--at` moves it, `--size`
zooms), then the headline (line 2 is the long one; keep it under 14 characters at this
size), then the accent color in every `#7fc4bf` and `#85cbc6`.

## 10. Variations and pairings

* Accent swaps on white: coral `#f08a7a`, cobalt `#3d6fe0`, mustard `#e3b23c`, lime
  `#b9d94c`. Keep the photo gray; with a dark accent make the disc text white as here.
* Dark version: ground `#101418`, ghost ring `#1c2228`, headline white, same teal.
* Rotate the ring's gap to another quarter (start the arc at 3:30 and end at 6:30) and
  move the logo into the gap; the gap should always point at something.
* Square post: keep the ring and photo, set the headline on one line under it, drop the
  columns.
* Works for agencies, clinics, schools, property, annual-report covers. Pairs with
  T-duotone (teal to white) on the photo and T-pill-button in place of the contact disc.

## 11. Pitfalls

* Photo and ring perfectly concentric: it turns into a target. Offset the photo disc 3% W
  toward the ring's gap so the white crescent appears.
* A colorful photo: the teal stops being the accent. Desaturate by at least 60.
* Arc direction mistakes: with `A r r 0 1 0` the stroke runs counter-clockwise from the
  start point. If the short quarter is drawn instead of the long three-quarters, flip the
  large-arc flag (the fourth number) to 1.
* Body text on the disc in black: on mid teal only white reads (and only at weight 400 or
  more). Keep the disc lighter than the ring so it reads as a second layer.
* Hairline circles thicker than 2 px start to compete with the ring; keep them at 1 to
  2 px and let them leave the page.
