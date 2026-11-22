---
id: p014
title: New-product poster with see-through word
genre: [product-ad, fashion, social-post]
mood: [fresh, minimal, bold, playful]
style: [minimal, photo-composite, type-led]
format: portrait-a
archetype: A-text-behind-subject
palette: ['#e6e7ee', '#1c1d1c', '#fcea0c', '#f4d93a', '#ffffff']
techniques: [T-text-behind-subject, T-outline-type, T-fit-width, T-stretch-type, T-pill-button, T-spotlight, T-ghost-type]
needs: product-cutout
---

# p014: New-product poster with see-through word

## 1. Concept

A launch poster for a pair of yellow sneakers on a pale gray page. A short heavy word and
a very tall condensed word are stacked above the product; the shoes sit on top of the tall
word, and where they cover it the letters continue as a thin outline drawn over the
product. The word is therefore both behind and in front of the shoes. It works because
there is one color (the product's yellow, repeated once in the button) and one trick (the
solid-to-outline switch); everything else is gray, black, and quiet.

## 2. Format and grid

* Portrait A-series (1 : 1.414). Everything is centered on the 50% W axis. Side margins
  14% W for the big word, 19% W for the body copy.
* Zones from the top: brand line at 9% H; word one 16 to 25% H; word two 28 to 48% H;
  product 30 to 68% H (it overlaps the lower two thirds of word two); body copy 71 to
  79% H; button 81.5 to 88% H; web address at 91.6% H.
* Vertical rhythm: the gaps between brand, word one, and word two are 6% and 3% H; the
  blocks below the product are 3% H apart. The top 8% and the bottom 7% are empty.

## 3. Layer stack

1. Ground: flat pale cool gray `#e6e7ee`, full canvas.
2. Brand line: one word in title case, near-black `#1c1d1c`, regular weight, cap height
   1.6% H, centered at 9% H.
3. Word one: 3 letters, near-black, heavy retro display face with one quirky letter (a
   crescent-shaped "e" among capitals), cap height 9% H, 39% W wide, top at 15.8% H.
4. Word two, solid: 7 letters, near-black, ultra-condensed gothic capitals, cap height
   20% H, 72% W wide (`13.8%,28%,72.4% x 19.8%`). Each letter is about 2.7 times as tall
   as it is wide.
5. Floor shadow: soft gray `#a1a1a6` blur under the shoes, `14%,60%,74% x 8%`, strongest
   at the center, gone by 70% H.
6. Product: a pair of low canvas sneakers, yellow `#f4d93a` uppers with white toe caps,
   soles, and laces, one shoe leaning on the other; `11%,30%,80% x 38%`. It hides the
   lower half of the five middle letters of word two.
7. Word two, outline: the same word in the same place, no fill, 2 px near-black outline,
   above the product. It is visible only on the product (on the gray it coincides with
   the solid letters), so the letters appear to switch from solid to wireframe where the
   shoes pass over them.
8. Body copy: three centered lines, near-black, regular, cap height 1.9% H, tops at
   71.5%, 74.6%, 77.7% H; the longest line 60% W.
9. Button: pill `33.9%,81.5%,32.2% x 6.3%`, yellow `#fcea0c`, fully rounded ends; label
   "Shop Now" in bold near-black, cap height 2.4% H.
10. Web address: lowercase, letterspaced very wide, near-black, cap height 1.6% H, 60% W
    wide, centered at 91.6% H.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#e6e7ee` | ground | 65% |
| `#1c1d1c` | all type | 17% |
| `#f4d93a` | product yellow | 9% |
| `#ffffff` | soles, laces, toe caps (photo) | 5% |
| `#fcea0c` | button | 2% |
| `#a1a1a6` | floor shadow | 2% |

Neutral gray and black with a single yellow accent that appears exactly twice (product,
button). The button is slightly greener and brighter than the product so it reads as
"interface", not as part of the photo.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Brand | `[BRAND]`, 1 word | title | geometric sans (Poppins, Jost / Avenir Next) | 400 | 1.6% | 0 | - | center | `#1c1d1c` | 50%, 9% |
| Word one | "NEW" | caps with one unicase letter | heavy retro display (Righteous, Bungee, Limelight / Futura 700) | 700 | 9% | -20 | - | center | `#1c1d1c` | 50%, top 15.8% |
| Word two | 7-letter noun | caps | ultra-condensed gothic (League Gothic, Anton / DIN Condensed stretched) | 700 | 20% | 0 | - | center | `#1c1d1c` solid, then outline 2 px | 50%, top 28% |
| Body | 3 lines, about 14 words, a benefit sentence | sentence | geometric sans (Poppins / Avenir Next) | 400 | 1.9% | 10 | 128% | center | `#1c1d1c` | 50%, top 71.5% |
| Button | "Shop Now" | title | same | 700 | 2.4% | 0 | - | center | `#1c1d1c` | 50%, 84.7% |
| Web address | one URL | lower | same | 400 | 1.6% | 280 | - | center | `#1c1d1c` | 50%, 91.6% |

Three faces: a wide heavy display word, an ultra-condensed word (contrast in width, not in
weight), and one geometric sans for all small text.

## 6. Imagery

One studio product shot, cut out: two sneakers seen from the side at a slight downward
angle, one resting on the other so the pair forms a low, wide triangle. Even soft light,
saturated yellow, a soft gray shadow directly below (rebuilt in the design, not kept from
the photo). The product is wider than the tall word (80% against 72% W) and overlaps it
by roughly 45% of the word's height, leaving the letter tops clear so the word still
reads. Any product with a strong single color and a clean outline works: a handbag, a
chair, headphones, a bottle. Treatment: **T-text-behind-subject** plus the outline copy
on top (**T-outline-type**).

## 7. Effects and textures

* **T-text-behind-subject**: solid word, then product, then the outline copy.
* **T-outline-type**: `--color transparent --stroke '#1c1d1c' --stroke-width 2`. No mask
  is needed: over the gray ground the outline lies exactly on the solid letters. The
  solid word carries the same 2 px stroke in its own color, so both copies take the same
  flags and fit to the same size.
* **T-fit-width** with **T-stretch-type**: `--scale-x 70.4% --fit-width 778` sets both the
  width and the cap height of word two (the narrower the scale, the taller the fitted
  line; here 306 px), which turns an ordinary condensed face into an ultra-condensed one.
  The text stays live.
* **T-spotlight** shadow only: a `shape ellipse` with `filter gaussian --sigma 12` on it.
  The blur is live and is not cut off at the shape's box.
* **T-pill-button**: filled pill in the accent color.
* No grain, no gradients: a clean catalog look.
* **T-ghost-type** in [techniques.md](../techniques.md) is the tested library version of the faint, defocused, or outlined giant word; start from it when reusing this idea.

## 8. Hierarchy and eye path

1. The yellow product in the middle (only saturated area).
2. Word two, because the outline pulls the eye along the letters across the product, then
   up to word one.
3. Down to the yellow button (second and last yellow spot).
4. Body copy, web address, brand line.

Cap heights: word two 20 : word one 9 : button 2.4 : body 1.9 : address 1.6, about
10.5 : 4.7 : 1.3 : 1 : 0.85.

## 9. Build recipe

`product.png` is your image here. The stand-in is a tall bottle; for a wide product (a
pair of shoes, a bag) use `--size 860x` so it spans 80% of the width and still ends at
68% H.

```bash
pixelox new -o launch.pxo --size 1080x1528 --fill '#e6e7ee'
pixelox text add launch.pxo --in-place --name Brand --content "Northlane" --font @geometric-sans --weight 400 --size 2.2% --color '#1c1d1c' --at 50%,9% --anchor c
pixelox text add launch.pxo --in-place --name Word1 --content "NEW" --font Futura --weight 700 --fit-width 417 --tracking -20 --color '#1c1d1c' --at 50%,15.8% --anchor t --anchor-to ink
pixelox text add launch.pxo --in-place --name Word2 --content "ARRIVAL" --font "DIN Condensed" --weight 700 --scale-x 70.4% --fit-width 778 --color '#1c1d1c' --stroke '#1c1d1c' --stroke-width 2 --at 50%,28% --anchor t --anchor-to ink
pixelox shape ellipse launch.pxo --in-place --name Shadow --rect 16%,61.5%,68%x5.5% --fill '#8e8e96'
pixelox filter gaussian launch.pxo --in-place --layer Shadow --sigma 12
pixelox comp place launch.pxo product.png --in-place --as Product --size x600 --at 50%,68% --anchor b
pixelox text add launch.pxo --in-place --name Outline --content "ARRIVAL" --font "DIN Condensed" --weight 700 --scale-x 70.4% --fit-width 778 --color transparent --stroke '#1c1d1c' --stroke-width 2 --at 50%,28% --anchor t --anchor-to ink
pixelox text add launch.pxo --in-place --name Body --content $'Built light for long days on your feet:\ncushioned, breathable, and ready\nfor the street.' --font @geometric-sans --weight 400 --size 2.6% --tracking 10 --leading 128% --align center --color '#1c1d1c' --at 50%,71.5% --anchor t --anchor-to ink
pixelox shape rect launch.pxo --in-place --name Pill --rect 33.9%,81.5%,32.2%x6.3% --radius 48 --fill '#fcea0c'
pixelox text add launch.pxo --in-place --name PillText --content "Shop Now" --font @geometric-sans --weight 700 --size 3.3% --color '#1c1d1c' --at 50%,84.7% --anchor c --anchor-to ink
pixelox text add launch.pxo --in-place --name Site --content "www.example.com" --font @geometric-sans --weight 400 --size 2.2% --tracking 280 --color '#1c1d1c' --at 50%,91.6% --anchor c --anchor-to ink
pixelox export launch.pxo -o launch.png
```

Change first: the product, then word two on both copies (6 to 8 letters; keep
`--fit-width 778` and change `--scale-x` until the letters are about 306 px tall: a
smaller scale gives taller letters), then the accent yellow in the pill to a color
sampled from your product and brightened.

## 10. Variations and pairings

* Accent swaps: product red with a `#ff3b30` pill; cobalt with `#2f6bff` and white label;
  on a dark version use ground `#151517`, type `#f2f2f2`, and a neon pill `#c6ff3d`.
* Reverse the trick: outline word behind, solid letters in front, clipped to the product
  with `layer clip` (the product then looks stamped with the word).
* Square post: keep word two and the product, move the button to the lower right, drop
  the body copy.
* Suits footwear, bags, furniture, small appliances, cosmetics; any "new arrival" or
  restock announcement.
* Pairs with T-cutout-drop-shadow (soft studio shadow in place of the ellipse),
  T-brush-swash in the accent color behind the product, and T-paper-grain at amount 8.

## 11. Pitfalls

* The outline copy shifted by even 2 px: a black edge appears beside the solid letters.
  Give both copies the same content, font, `--scale-x`, `--fit-width`, stroke, and `--at`;
  only `--color` differs.
* The product covering the whole word. Keep the top 40 to 55% of the letters clear and
  let the product hide only the lower part of the middle letters.
* A dark or busy product: a thin black outline disappears on it. Use a white outline on
  dark products, or raise the stroke to 3 px.
* Narrowing a wide face. Only a condensed face survives `--scale-x 70%`; a regular
  grotesque gets hairline verticals and looks broken.
* Two accent colors. The yellow may appear on the product and the button, nowhere else;
  a colored headline or address line destroys the calm.
