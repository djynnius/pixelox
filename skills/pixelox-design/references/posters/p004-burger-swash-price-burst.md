---
id: p004
title: Burger ad with brush swash and price burst
genre: [food-ad, sale-promo, product-ad]
mood: [bold, warm, playful, friendly]
style: [photo-composite, flat]
format: portrait-a
archetype: A-hero-object
palette: ['#e3c7aa', '#d23200', '#e8a53a', '#ffffff']
techniques: [T-brush-swash, T-price-burst, T-fit-width, T-stretch-type]
needs: product-cutout
---

# p004: Burger ad with brush swash and price burst

## 1. Concept

A fast-food offer poster: one oversized burger cutout sits on a mustard brush swash in the
middle of a flat tan page, a chunky two-line product name above it, a jagged price sticker
biting into its lower right corner, and one offer line at the bottom. It works because it
is a three-step sentence read top to bottom (what, how much, the deal) in only three warm
colors that are all taken from the food itself: bun tan, ketchup red, mustard yellow.

## 2. Format and grid

* Portrait A-series (1 : 1.414). One center axis at 50% W; the headline, the burger, and
  the offer line are centered on it. Margins about 20% W for type, 3% W for the swash.
* Zones: headline 8 to 28% H; hero 31 to 85% H (burger 31 to 76% H, swash 49 to 85% H);
  offer line 88 to 92.5% H. The bottom 7.5% and the top 8% stay empty.
* Two things sit off the axis on purpose and balance each other on a diagonal: the white
  "pop" strokes at the upper left of the burger (5 to 22% W, 31 to 53% H) and the price
  burst at the lower right (60 to 93% W, 58 to 82% H).

## 3. Layer stack

1. Ground: flat tan `#e3c7aa`, full canvas. No gradient, no texture.
2. Brush swash: three to four overlapping horizontal dry-brush strokes in mustard
   `#e8a53a`, together `3%,49%,94% x 36%`, ragged at both ends, with small flecks around.
3. Headline line 1: 4 letters, red-orange `#d23200`, centered, cap height 9.5% H, 42% W
   wide, cap top at 7.9% H.
4. Headline line 2: 6 letters, same color and size, 60% W wide, cap top at 18% H (gap
   between the lines 0.7% H: almost touching).
5. Burger cutout: `15%,31%,71% x 45%`, natural color, no shadow and no outline; its lower
   two thirds overlap the swash, its bun rises above it onto the tan.
6. Pop strokes: five short white rounded strokes (3% W thick, 4 to 12% W long) fanning out
   from the burger's upper-left edge like a comic "shine" mark.
7. Price burst: 24-point sticker star, red-orange `#d23200`, center 77% W, 70% H, outer
   diameter 33% W, short teeth (inner radius 85% of the outer). It covers the burger's
   lower right corner.
8. Price: white, heavy wide grotesque, cap height 4.3% H, 22% W wide, centered on the burst.
9. Offer line: "BUY 1 GET 1 FREE!", red-orange, the headline face, cap height 4.3% H,
   57% W wide, centered at 90.4% H.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#e3c7aa` | tan ground | 60% |
| `#d23200` | red-orange: headline, burst, offer line | 15% |
| `#e8a53a` | mustard swash | 15% |
| `#ffffff` | pop strokes, price | 3% |
| food colors (green `#8fb23a`, brown `#4a2d1b`) | only inside the photo | 7% |

Analogous warm scheme: tan, mustard, red-orange, all within 40 degrees of hue. The only
cool color is the lettuce inside the photo, so the eye goes to the food.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Headline 1 | size word, 4 to 5 letters | caps | chunky rounded display with angled cuts (Lilita One, Bowlby One, Titan One / Arial Rounded MT Bold) | 400 (the face is heavy) | 9.5% | 0 | - | center | `#d23200` | 50%, cap top 7.9% |
| Headline 2 | product noun, 6 letters | caps | same | same | 9.5% | 0 | lines 0.7% H apart | center | `#d23200` | 50%, cap top 18% |
| Price | "$3.99" pattern, 5 characters | - | heavy wide grotesque (Archivo Black / Arial Black) | 900 | 4.3% | 20 | - | center | `#ffffff` | 77%, 70% |
| Offer | "BUY 1 GET 1 FREE!" | caps | the headline face | same | 4.3% | 0 | - | center | `#d23200` | 50%, 90.4% |

Two families: a rounded, chunky display face for everything red, and a wide grotesque for
the one white number, so the price looks stamped on by a different hand.

## 6. Imagery

One product photograph, cut out cleanly: a tall stacked burger seen from the side at eye
level (bun, lettuce, tomato, onion, glazed meat, pickles, white sauce dripping), lit
softly from the front with saturated, appetizing color. It is big: 71% of the width and
45% of the height, larger than the headline. No drop shadow; the swash acts as its
"plate" and stops it from floating (**T-brush-swash**). Any single dish, drink, or packaged
product shot from the side works. Keep the cutout edge clean (see "Getting a cutout" in
techniques.md); a fringe shows at once on a flat tan ground.

## 7. Effects and textures

* **T-brush-swash**: two renders stacked (seeds differ) to get the look of three or four
  passes; `bristles=120`, `passes=3`, `rough=0.7`; the lower one a little narrower and
  tilted 2 degrees the other way. Width 94% W.
* **T-price-burst**: `--points 24`, `--outer 178`, `--inner 150` on a 1080 px canvas,
  tilted 8 degrees; no drop shadow here (the design is completely flat).
* **T-fit-width** on both headline lines and on the offer line, so the block widths are
  42 to 48%, 60%, and 57% W.
* Fattened type: `fx stroke` in the fill color (9 px outside on the headline, 4 px on the
  offer line) turns a medium rounded face into a chunky one.
* Pop strokes: five `shape line` layers with `--cap round`.
* No grain, no shadows, no gradients. Everything except the photo is a flat shape.
* **T-stretch-type** in [techniques.md](../techniques.md) is the tested library version of the stretched, slanted, or fattened lettering; start from it when reusing this idea.

## 8. Hierarchy and eye path

1. The burger: the only photograph, the largest and most detailed object, dead center.
2. Up to the headline (largest type, highest contrast against the tan).
3. Down the diagonal to the price burst: a saturated solid shape with white type on it.
4. The offer line at the bottom, read last, same color as the headline so it feels like
   the end of the same sentence.

Cap-height ratio: headline 9.5 : price 4.3 : offer 4.3, about 2.2 : 1 : 1. The price wins
over the offer line by contrast (white on red), not size.

## 9. Build recipe

`product.png` is your image here: a food or product cutout. For a wide product such as a
burger, replace `--size x690` with `--size 770x` so it spans 71% of the width.

```bash
pixelox render js scripts/brush.js -o swash1.png --size 1020x330 --seed 2 --param color='#e8a53a' --param bristles=120 --param passes=3 --param rough=0.7
pixelox render js scripts/brush.js -o swash2.png --size 940x300 --seed 9 --param color='#e8a53a' --param bristles=110 --param passes=3 --param rough=0.7 --param dry=0.45
pixelox new -o ad.pxo --size 1080x1528 --fill '#e3c7aa'
pixelox comp place ad.pxo swash1.png --in-place --as Swash1 --at 50%,59% --anchor c
pixelox transform rotate ad.pxo --in-place --layer Swash1 --degrees -2
pixelox comp place ad.pxo swash2.png --in-place --as Swash2 --at 52%,75% --anchor c
pixelox transform rotate ad.pxo --in-place --layer Swash2 --degrees 2
pixelox text add ad.pxo --in-place --name H1 --content "GIANT" --font @rounded-heavy --fit-width 520 --color '#d23200' --at 50%,8.3% --anchor t --anchor-to ink
pixelox text add ad.pxo --in-place --name H2 --content "BURGER" --font @rounded-heavy --fit-width 644 --color '#d23200' --at 50%,18.4% --anchor t --anchor-to ink
pixelox fx stroke ad.pxo --in-place --layer H1 --width 9 --color '#d23200' --position outside
pixelox fx copy ad.pxo --in-place --from H1 --to H2
pixelox comp place ad.pxo product.png --in-place --as Product --size x690 --at 50%,76% --anchor b
pixelox shape line ad.pxo --in-place --name Pop1 --from 14%,32.5% --to 20%,37.5% --stroke '#ffffff' --stroke-width 34 --cap round
pixelox shape line ad.pxo --in-place --name Pop2 --from 14.5%,38.5% --to 16.5%,39.5% --stroke '#ffffff' --stroke-width 26 --cap round
pixelox shape line ad.pxo --in-place --name Pop3 --from 7%,41% --to 15%,43% --stroke '#ffffff' --stroke-width 34 --cap round
pixelox shape line ad.pxo --in-place --name Pop4 --from 12.5%,46.5% --to 14.5%,46.2% --stroke '#ffffff' --stroke-width 22 --cap round
pixelox shape line ad.pxo --in-place --name Pop5 --from 10.5%,52% --to 15%,49% --stroke '#ffffff' --stroke-width 30 --cap round
pixelox shape star ad.pxo --in-place --name Burst --center 77%,70% --points 24 --outer 178 --inner 150 --fill '#d23200'
pixelox transform rotate ad.pxo --in-place --layer Burst --degrees 8
pixelox text add ad.pxo --in-place --name Price --content "\$4.50" --font @heavy-wide --fit-width 236 --tracking 20 --color '#ffffff' --at 77%,70% --anchor c --anchor-to ink
pixelox text add ad.pxo --in-place --name Offer --content "BUY 1 GET 1 FREE!" --font @rounded-heavy --fit-width 620 --color '#d23200' --at 50%,90.4% --anchor c --anchor-to ink
pixelox fx stroke ad.pxo --in-place --layer Offer --width 4 --color '#d23200' --position outside
pixelox export ad.pxo -o ad.png
```

The `fx stroke` in the letters' own color is what makes the stand-in face chunky: it
thickens every stem by 9 px and rounds the corners. With a registered chunky face (Lilita
One, Bowlby One) drop those two stroke lines and keep the same `--fit-width` values.

Change first: the two headline words (keep the top line shorter than the bottom line),
the price, then the three colors if the food is not warm-toned.

## 10. Variations and pairings

* Palette swaps that keep the "colors from the food" rule: mint `#cfe8d5` ground, cocoa
  `#4a2a1a` type, pink `#f29bb2` swash for ice cream; pale blue `#d6ecf5`, navy `#12305a`,
  lime `#b8d430` for drinks; charcoal `#1f1b18` ground with cream type and a chili-red
  swash for a late-night menu.
* Square post: drop the offer line into the burst (two lines), keep headline 8 to 30% H
  and the product 30 to 92% H.
* Story 9:16: more air above the headline, the offer line becomes a T-pill-button.
* Works for any single-item deal: pizza slice, coffee cup, sneaker, cosmetics tube.
* Pairs with T-sunburst behind the product (same tan, lighter rays at alpha 0.25),
  T-cutout-drop-shadow (soft studio shadow) when there is no swash, and T-paper-grain at
  amount 10 for a printed menu feel.

## 11. Pitfalls

* A small product. The cutout must be wider than the headline; under 55% W the page looks
  empty and the swash takes over.
* The swash as a neat rectangle. Keep both ends ragged, stack two renders, and let it run
  to within 3% of the canvas edges.
* The burst floating beside the product. It has to overlap the product's corner by about a
  quarter of its own width and never touch the headline.
* A fourth color. Green or blue type on this palette fights the food; every graphic
  element is tan, mustard, red-orange, or white.
* Thin or condensed type. The headline face must be as chunky as the burger; a light
  grotesque makes the same layout look like a pharmacy leaflet.
