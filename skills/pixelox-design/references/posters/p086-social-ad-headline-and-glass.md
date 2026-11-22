---
id: p086
title: Red social ad with headline and glass
genre: [drink-ad, social-post, sale-promo]
mood: [bold, energetic, warm, dark]
style: [photo-composite, flat]
format: square
archetype: A-text-left-product-right
palette: ['#520404', '#760907', '#a01610', '#f8f0e5', '#c77252', '#240907']
techniques: [T-spotlight, T-pill-button, T-rules, T-splatter, T-cutout-drop-shadow]
needs: product-cutout
---

# p086: Red social ad with headline and glass

## 1. Concept

A ready-made social media ad for a cold drink: on the left a three-line italic headline, a
white "Shop Now" pill, and a web address; on the right a tilted glass of cola with ice,
splashing over its rim. The ground is deep red with a brighter pool behind the glass, and
a row of slanted white bars repeats in two corners as the brand device. It is a template
layout: text column left, product right, logo top-right, link bottom-left.

## 2. Format and grid

* Square. Margins 7.5% W on every side; all text hangs from the left margin.
* Two columns: text 7.5 to 46% W; product 40 to 91% W. They overlap by 6% W (the splash
  reaches toward the headline).
* Left column, top to bottom: bars 11.5 to 16% H; headline 19 to 42.5% H; empty 43 to 62% H;
  button 63 to 70% H; web address at 92.8% H.
* Right column: logo at 7% H, right-aligned at 92.5% W; glass 17 to 84% H, leaning 14
  degrees clockwise; second set of bars 91 to 95.5% H, ending at 92.5% W.
* Diagonal echo: bars top-left and bottom-right; logo top-right and address bottom-left.

## 3. Layer stack

1. Ground: deep red `#520404`, darker `#460000` at the left edge and corners.
2. Curved panel: a very large disc in slightly lighter red `#760907`, its edge sweeping
   from the top center down through the lower right; it shows as a soft curve behind the
   glass.
3. Glow: bright red `#a01610` to `#b91214` behind the glass, centered at 62% W, 50% H.
4. Bars, top-left: five white parallelograms, each 2.4% W wide and 4.4% H tall, slanted
   20 degrees to the right, 1% W apart; `7.5%,11.5%,16.5% x 4.4%`.
5. Headline: three lines of white heavy italic capitals, left-aligned at 7.5% W; cap
   height 6.3% H; line tops at 19%, 27.7%, and 36.3% H. Line lengths 7, 3, and 5 letters;
   the longest is 38% W.
6. Button: white pill `7.5%,63%,24% x 7%`, fully rounded, with a two-word label in dark red
   bold italic.
7. Web address: one line of small bold white lowercase with wide letterspacing, 7.5 to
   50% W at 92.8% H.
8. Logo: the word "LOGO" as a placeholder in white heavy caps, right-aligned at 92.5% W,
   7% H.
9. Floor shadow: a soft dark ellipse under the glass at 60 to 90% W, 82 to 86% H.
10. Glass: a curved tumbler of cola with ice cubes, `52%,26%,39% x 58%` after tilting,
    amber-orange `#f18111` where light passes through the ice, cream foam at the top.
11. Splash: cola and foam `#c77252` to `#f8f0e5` leaping from the rim up and to the left,
    reaching 39% W, 17% H, with flying drops.
12. Bars, bottom-right: the same five bars, `76%,91%,16.5% x 4.4%`.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#520404` | deep red ground | 33% |
| `#760907` | curved panel, mid reds | 30% |
| `#a01610` | glow behind the glass | 24% |
| `#f8f0e5` | type, bars, button, foam | 6% |
| `#c77252` | cola and ice highlights | 4% |
| `#240907` | cola shadows, floor shadow | 3% |

Monochrome red with white for everything that must be read. The amber inside the glass is
the only other hue and belongs to the product.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Headline | 3 lines: verb (7 letters), "THE", noun (5 letters) | caps | wide heavy italic sans (Montserrat 800 Italic / Avenir Next) | 800 italic | 6.3% | 0 | 98% | left | `#ffffff` | 7.5%, tops 19%, 27.7%, 36.3% |
| Button label | "Shop Now" | title | same | 700 italic | 2.2% | 0 | - | center | `#6a0808` | 19.5%, 66.5% |
| Web address | one URL | lower | same, upright | 700 | 1.3% | 300 | - | left | `#ffffff` | 7.5%, 92.8% |
| Logo | "LOGO" (placeholder for a wordmark) | caps | same, upright | 800 | 2.9% | 20 | - | right | `#ffffff` | 92.5%, 7% |

One family, mostly bold italic. The italic and the slanted bars lean the same way, which
is what makes the static layout feel fast.

## 6. Imagery

One product cutout: a glass of iced cola frozen mid-splash, tilted as if just set down
hard. It is lit from behind so the drink glows amber at the ice, and it casts a short soft
shadow on an implied floor. In a build the product cutout is rotated 14 degrees and a
splatter in foam colors is placed at its rim. Any product works if it is tilted and has
something leaving it (splash, crumbs, steam, sparks).

## 7. Effects and textures

* **T-spotlight**: radial glow `#b91214` at 62% W, 50% H; a darker shade at the left edge
  under the text.
* Curved panel: one oversized `shape ellipse` at 50% opacity.
* **T-rules**, as a device: five slanted bars drawn as one `shape path` with five closed
  sub-paths, used twice.
* **T-pill-button**: filled white pill, `--radius` half the height, label at 45% of the
  pill height.
* **T-splatter** in cola and foam tones with `dir=-110`; **T-cutout-drop-shadow** soft
  floor ellipse (sigma 18).

## 8. Hierarchy and eye path

1. Headline (largest white shape, upper left).
2. The glass and splash to its right (the splash points back at the headline).
3. Down the left edge to the white button.
4. Web address, logo, bars.

Cap heights: headline 6.3 : logo 2.9 : button label 2.2 : web address 1.3, about
4.8 : 2.2 : 1.7 : 1.

## 9. Build recipe

`product.png` is your image here: a glass, bottle, or can cutout.

```bash
pixelox render js scripts/splatter.js -o splash.png --size 560x560 --seed 3 --param color='#e8c9a8' --param radius=0.1 --param drops=380 --param arms=14 --param reach=3.6 --param dir=-110
pixelox new -o post.pxo --size 1080x1080 --fill '#520404'
pixelox shape ellipse post.pxo --in-place --name Panel --rect 430,0,1100x1250 --fill '#8a0c0a' --opacity 50
pixelox transform move post.pxo --in-place --layer Panel --by 0,-220
pixelox layer add post.pxo --in-place --name Glow
pixelox fill gradient post.pxo --in-place --layer Glow --kind radial --from 62%,50% --to 62%,95% --stops '#c01616e6@0,#a0161000@1'
pixelox layer add post.pxo --in-place --name LeftShade
pixelox fill gradient post.pxo --in-place --layer LeftShade --from 0,0 --to 45%,0 --stops '#3a000099@0,#3a000000@1'
pixelox shape path post.pxo --in-place --name BarsTop --d 'M 98 124 L 124 124 L 107 172 L 81 172 Z M 135 124 L 161 124 L 144 172 L 118 172 Z M 172 124 L 198 124 L 181 172 L 155 172 Z M 209 124 L 235 124 L 218 172 L 192 172 Z M 246 124 L 272 124 L 255 172 L 229 172 Z' --fill '#ffffff'
pixelox text add post.pxo --in-place --name Headline --content $'CAPTURE\nTHE\nCOLD' --font "Avenir Next" --weight 800 --style italic --size 8.8% --leading 98% --color '#ffffff'
pixelox transform move post.pxo --in-place --layer Headline --to 7.5%,19% --anchor tl
pixelox shape rect post.pxo --in-place --name Button --rect 7.5%,63%,24%x7% --radius 38 --fill '#ffffff'
pixelox text add post.pxo --in-place --name ButtonLabel --content "Shop Now" --font "Avenir Next" --weight 700 --style italic --size 3.2% --color '#6a0808' --at 19.5%,66.5% --anchor c
pixelox text add post.pxo --in-place --name Site --content "www.example.com" --font "Avenir Next" --weight 700 --size 2.5% --tracking 300 --color '#ffffff' --at 7.5%,92.8% --anchor l
pixelox text add post.pxo --in-place --name Logo --content "LOGO" --font "Avenir Next" --weight 800 --size 4% --tracking 20 --color '#ffffff' --at 92.5%,7% --anchor r
pixelox shape ellipse post.pxo --in-place --name FloorShadow --rect 60%,81.5%,30%x4.5% --fill '#240303'
pixelox layer rasterize post.pxo --in-place --layer FloorShadow
pixelox filter gaussian post.pxo --in-place --layer FloorShadow --sigma 18
pixelox comp place post.pxo splash.png --in-place --as Splash --at 60%,27% --anchor c
pixelox comp place post.pxo product.png --in-place --as Glass --size x720 --at 69%,53% --anchor c
pixelox transform rotate post.pxo --in-place --layer Glass --degrees 14
pixelox fx drop-shadow post.pxo --in-place --layer Glass --offset 0,12 --blur 40 --color '#2a0303' --opacity 50
pixelox shape path post.pxo --in-place --name BarsBottom --d 'M 838 983 L 864 983 L 847 1031 L 821 1031 Z M 875 983 L 901 983 L 884 1031 L 858 1031 Z M 912 983 L 938 983 L 921 1031 L 895 1031 Z M 949 983 L 975 983 L 958 1031 L 932 1031 Z M 986 983 L 1012 983 L 995 1031 L 969 1031 Z' --fill '#ffffff'
pixelox export post.pxo -o post.png
```

Change first: the three headline lines (keep the middle one short), the button label, the
ground red (then the panel and glow as lighter steps of it), and the device: replace the
bars with your brand's own repeating mark.

## 10. Variations and pairings

* Recolor for any brand: navy `#0b1f4b` with a cyan glow, black with a lime glow, cream
  `#f3ead7` with dark type and a dark button.
* Story 9:16: stack it. Logo at 6% H, headline 12 to 30% H, product 32 to 78% H, button at
  84% H, address at 93% H, everything centered.
* Sale version: swap the button for a T-price-burst overlapping the product's lower left.
* Food, cosmetics, gadgets: the left column stays; only the cutout and the splash prop
  change (crumbs, petals, sparks).
* Pairs with T-pattern (diagonal stripes in the device's angle, low alpha, in one corner)
  and T-paper-grain.

## 11. Pitfalls

* Product and headline both centered on the canvas collide. Keep the text in the left 46%
  and the product's center at 65 to 70% W.
* An upright italic mix: italic headline with upright bars, or the reverse, looks
  accidental. Slant the device to the italic angle (about 20 degrees from vertical).
* Button label in pure black on white reads as a different brand. Use the darkest red of
  the palette.
* A flat ground makes the product float. It needs the glow behind it and the soft floor
  shadow under it.
* More than one button, or a long sentence under the headline. This layout carries one
  headline, one action, one link.
