---
id: p091
title: Coffee jar with orbiting light swirl
genre: [drink-ad, product-ad, social-post]
mood: [warm, dramatic, energetic, bold]
style: [photo-composite, 3d-render]
format: portrait-4x5
archetype: A-hero-object
palette: ['#b31d11', '#820f06', '#f22b20', '#530604', '#e69470', '#fcecd6']
techniques: [T-pattern, T-neon-glow, T-motion-streak, T-cutout-drop-shadow, T-vignette, T-paper-grain, T-ring-arcs]
needs: product-cutout
---

# p091: Coffee jar with orbiting light swirl

## 1. Concept

A showpiece product visual for instant coffee: a glass jar with a red lid stands on a
glossy red stage in front of red paneled walls, a ribbon of warm light circles it like a
comet's orbit, and huge coffee beans fly past in motion blur. The brand glows out of focus
on a sign behind. A small justified paragraph tells the brand story in the top-left and a
two-line credit sits at the bottom. Everything is red, so the white-gold swirl is the one
thing that moves the eye.

## 2. Format and grid

* Portrait 4:5. Text margin 4% W; the product is centered at 49% W.
* Zones: back wall 0 to 78% H (vertical panels); floor 78 to 100% H; sign 10 to 22% H
  behind the jar's lid; jar 19 to 80% H; credit 93 to 97% H.
* The jar is 38% W wide and 61% H tall.
* The swirl is an ellipse 66% W long and 24% H high, tilted 28 degrees counter-clockwise,
  centered at 57% W, 50% H: it passes in front of the jar at the lower left, climbs to the
  upper right at 88% W, 33% H, and goes behind the jar's shoulder.
* Flying beans are placed on the swirl's path, outside the jar: left-middle, top-right,
  right-middle, each 20 to 28% W long.
* Paragraph: `4%,17.5%,26.5% x 16.5%`, the only block in the upper-left; it sits on a dark
  panel so it stays readable.

## 3. Layer stack

1. Wall: red `#b31d11` with vertical panels (alternating lighter and darker slats, each
   about 4.5% W wide, darker `#820f06` grooves), falling to deep maroon `#530604` at the
   left and right edges.
2. Sign: a bright red rounded box `#f22b20`, `23%,10%,53% x 12%`, with the brand in white
   capitals, all strongly out of focus (blur radius about 2% W) so it is a glowing bar.
3. Floor: glossy red `#971414` from 78% H, darker toward the bottom corners, with a faint
   mirror image of the jar.
4. Far beans: three large roasted beans, soft and streaked sideways by motion blur, at
   10 to 35% W, 35 to 48% H; 80 to 100% W, 12 to 25% H; 62 to 90% W, 60 to 73% H.
5. Swirl, back half: a glowing arc behind the jar.
6. Jar: clear glass filled with dark coffee granules, red screw lid (33 to 66% W, 19 to
   29% H), a round red label with the brand in white capitals, a variant word under it,
   and a small picture of a red cup; fine print at the lower right of the label.
7. Swirl, front half: white-gold core `#fcecd6`, about 1% W thick, with an orange halo
   `#e69470` 5% W wide; brightest where it crosses the jar at 30 to 60% W, 55 to 68% H.
8. Bean pile: 15 to 20 small sharp beans with two green leaves at the bottom-left of the
   jar (5 to 37% W, 67 to 82% H); two or three single beans on the floor.
9. Paragraph: 11 lines of tiny white text, justified, top-left.
10. Credit: two centered lines of small white tracked capitals at 93.5% and 96% H
    (a "designed by" line and a name).

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#b31d11` | wall panels, lid, label | 27% |
| `#820f06` | grooves, floor shadows | 26% |
| `#f22b20` | sign, hottest reds | 16% |
| `#530604` | edges and corners | 14% |
| `#e69470` | swirl halo, bean highlights | 9% |
| `#fcecd6` | swirl core, type | 8% |

Red monochrome plus the warm white of light. Brown (coffee) is so dark it reads as a shade
of the red.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Paragraph | about 75 words on the brand's history, 11 lines of 28 to 32 characters | sentence | light squarish sans (Rajdhani 400, Saira 300 / Avenir Next Condensed) | 400 | 0.75% | 0 | 135% | justify | `#ffffff` | box 4%,17.5%, 26.5% W wide |
| Credit line 1 | "DESIGNED BY" | caps | geometric sans (Montserrat 500 / Avenir Next) | 500 | 0.9% | 500 | - | center | `#ffffff` | 52%, 93.8% |
| Credit line 2 | `[DESIGNER NAME]`, about 16 characters | caps | same | 500 | 0.9% | 500 | - | center | `#ffffff` | 52%, 96.2% |
| Sign | `[BRAND]`, 7 letters, blurred | caps | heavy sans (Archivo Black / Avenir Next) | 800 | 4.5% | 60 | - | center | `#ffffff`, defocused | 50%, 16% |

All typeset text is small. The piece is a visual; the paragraph is texture as much as
information.

## 6. Imagery

A product render with theatrical lighting: the jar frontal and sharp, the light ribbon
wrapping it in 3D (one half behind, one half in front), beans frozen at different depths.
Motion blur on the big beans and focus blur on the sign give the still image speed and
depth. For a build: your jar or pack cutout; the swirl from two stroked shapes with glows
(a full ellipse under the product and its lower arc over it); beans from drawn ellipses
with `filter motion-blur`.

## 7. Effects and textures

* **T-pattern**: vertical stripes (`kind=stripes`, `angle=0`, cell 96 px, weight 0.5) in a
  dark red at 45% as the wall panels; a reflected side shade darkens both edges.
* Defocused sign: a rounded rect and a text layer, each rasterized and blurred sigma 14.
* **T-neon-glow** for the swirl: near-white stroke, `fx outer-glow` in orange (blur 40,
  spread 10). The ellipse is drawn level and then rotated -28 degrees around its center;
  the front arc is the lower half of the same ellipse, rotated the same way and placed
  above the product.
* **T-motion-streak**: the three far beans are one `shape path` (three ellipses written
  with relative arcs), rasterized, then `filter motion-blur --angle -20 --distance 90`.
* **T-cutout-drop-shadow**: warm outer glow on the jar (blur 40, `#ff8a4a`, 30%) and a dark
  contact shadow on the floor.
* **T-vignette** in maroon; **T-paper-grain** amount 10.
* **T-ring-arcs** in [techniques.md](../techniques.md) is the tested library version of the ring, halo, or arc shapes; start from it when reusing this idea.

## 8. Hierarchy and eye path

1. The jar's label (sharp, centered, inside the loop of light).
2. Around the swirl: lower left, up to the right, past the flying beans.
3. The glowing sign behind the lid.
4. The paragraph top-left; the credit last.

The jar is 61% H tall against type with cap heights under 1% H: the type is a footnote.

## 9. Build recipe

`product.png` is your image here: a jar, tin, or bottle cutout, front view.

```bash
pixelox render js scripts/pattern.js -o slats.png --size 1080x1350 --param kind=stripes --param angle=0 --param cell=96 --param weight=0.5 --param color='#5a0402' --param alpha=0.45
pixelox render js scripts/particles.js -o pile.png --size 1080x1350 --seed 6 --param shape=dot --param colors='#4a2410,#6b3a1c,#3a1a0a' --param count=26 --param min=1.4 --param max=2.6 --param amin=1 --param amax=1 --param focus='0.2,0.77,0.11'
pixelox new -o ad.pxo --size 1080x1350 --fill '#b31d11'
pixelox comp place ad.pxo slats.png --in-place --as Slats --fit cover
pixelox layer add ad.pxo --in-place --name SideShade
pixelox fill gradient ad.pxo --in-place --layer SideShade --kind reflected --from 50%,0 --to 100%,0 --stops '#53060400@0.35,#530604d9@1'
pixelox shape rect ad.pxo --in-place --name Sign --rect 23%,10%,53%x12% --radius 30 --fill '#f22b20'
pixelox layer rasterize ad.pxo --in-place --layer Sign
pixelox filter gaussian ad.pxo --in-place --layer Sign --sigma 14
pixelox text add ad.pxo --in-place --name SignWord --content "BRANDED" --font "Avenir Next" --weight 800 --size 6.2% --tracking 60 --color '#ffffff' --at 50%,16% --anchor c
pixelox layer rasterize ad.pxo --in-place --layer SignWord
pixelox filter gaussian ad.pxo --in-place --layer SignWord --sigma 12
pixelox shape rect ad.pxo --in-place --name Floor --rect 0,78%,100%x22% --fill '#8a0f0a'
pixelox fx overlay ad.pxo --in-place --layer Floor --stops '#4a0503@0,#a01814@1' --angle 90
pixelox shape path ad.pxo --in-place --name FarBeans --d 'M 108 560 a 135 88 0 1 0 270 0 a 135 88 0 1 0 -270 0 M 860 250 a 120 85 0 1 0 240 0 a 120 85 0 1 0 -240 0 M 670 898 a 150 88 0 1 0 300 0 a 150 88 0 1 0 -300 0' --fill '#7a431f'
pixelox layer rasterize ad.pxo --in-place --layer FarBeans
pixelox filter motion-blur ad.pxo --in-place --layer FarBeans --angle -20 --distance 90
pixelox shape ellipse ad.pxo --in-place --name SwirlBack --rect 260,513,712x324 --fill none --stroke '#fff0d0' --stroke-width 9
pixelox fx outer-glow ad.pxo --in-place --layer SwirlBack --blur 40 --spread 10 --color '#ff8a3a' --opacity 85
pixelox transform rotate ad.pxo --in-place --layer SwirlBack --degrees -28 --around 616,675
pixelox comp place ad.pxo product.png --in-place --as Jar --size x900 --at 49%,82.5% --anchor b
pixelox fx outer-glow ad.pxo --in-place --layer Jar --blur 40 --color '#ff8a4a' --opacity 30
pixelox shape path ad.pxo --in-place --name SwirlFront --d 'M 260 675 A 356 162 0 0 0 972 675' --fill none --stroke '#fff0d0' --stroke-width 9 --cap round
pixelox fx outer-glow ad.pxo --in-place --layer SwirlFront --blur 40 --spread 10 --color '#ff8a3a' --opacity 85
pixelox transform rotate ad.pxo --in-place --layer SwirlFront --degrees -28 --around 616,675
pixelox comp place ad.pxo pile.png --in-place --as Pile --fit cover
pixelox fx bevel ad.pxo --in-place --layer Pile --size 8 --depth 150 --soften 3 --highlight '#d9a070' --shadow '#1a0702'
pixelox shape path ad.pxo --in-place --name Leaf --d 'M 70 1010 C 110 940 190 915 260 925 C 215 985 150 1020 70 1010 Z' --fill '#3f8a24'
pixelox text add ad.pxo --in-place --name Story --content "BRANDED is a household name for instant coffee. It began ninety years ago as a way to keep a good harvest from going to waste, and the simple idea of coffee that only needs hot water travelled the world. Today the range runs from classic to gold blend, and every jar still starts with roasted beans and a short list of ingredients." --font "Avenir Next Condensed" --weight 400 --size 1.2% --leading 135% --align justify --box 290x260 --color '#ffffff' --at 4%,17.5%
pixelox text add ad.pxo --in-place --name Credit --content $'DESIGNED BY\nYOUR STUDIO NAME' --font "Avenir Next" --weight 500 --size 1.25% --tracking 500 --leading 190% --align center --color '#ffffff' --at 52%,95% --anchor c
pixelox layer add ad.pxo --in-place --name Vignette
pixelox fill gradient ad.pxo --in-place --layer Vignette --kind radial --from 50%,48% --to 50%,125% --stops '#2a020100@0.5,#2a0201b3@1'
pixelox layer add ad.pxo --in-place --name Grain --fill '#808080'
pixelox filter add-noise ad.pxo --in-place --layer Grain --amount 10 --monochrome
pixelox layer blend ad.pxo --in-place --layer Grain --mode soft-light
pixelox layer opacity ad.pxo --in-place --layer Grain --value 30
pixelox export ad.pxo -o ad.png
```

Change first: the swirl (its ellipse box, the matching arc radii, and the shared rotation
center must change together), the wall color, then the story text and the credit.

## 10. Variations and pairings

* Cold product: blue slats `#0b3a8c`, a cyan-white swirl, ice chips in place of beans.
  Luxury: black slats, gold swirl `#f6d365`, no flying props.
* Two swirls crossing (rotate a second pair by +28 degrees) for an atom-like emblem
  around a tech product or a perfume bottle.
* Use the swirl alone on a plain T-spotlight ground when the brief is minimal.
* Square: jar 14 to 82% H, drop the paragraph, keep the credit.
* Pairs with T-lens-flare at the swirl's brightest point and T-bokeh in warm tones behind
  the jar.

## 11. Pitfalls

* A swirl drawn entirely in front of the product looks like a hoop leaning on it. Split
  it: full ring under the product layer, lower arc over it.
* Front arc and back ring out of register. Both must use the same center, radii, and
  rotation (`--around` the same point).
* Swirl core in saturated orange: flat. The core is nearly white; the glow carries the
  color.
* Every bean sharp. The large ones need motion blur and the small pile stays sharp, or
  the depth collapses.
* A paragraph of tiny white text on a light red is unreadable. Keep it over the darkened
  edge, 28 to 32 characters per line, and never smaller than 0.7% H cap height.
