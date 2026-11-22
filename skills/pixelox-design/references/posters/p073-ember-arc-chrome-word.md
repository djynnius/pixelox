---
id: p073
title: Chrome word under an ember light arc
genre: [conference, tech-ad, event-flyer]
mood: [dark, futuristic, dramatic, warm]
style: [type-led, gradient-mesh, minimal]
format: portrait-3x4
archetype: A-swiss-asymmetric
palette: ['#171516', '#3d1714', '#da3c05', '#ffa902', '#dbdbdb', '#ffffff']
techniques: [T-gradient-type, T-light-leak, T-rules, T-paper-grain, T-ring-arcs]
needs: nothing
---

# p073: Chrome word under an ember light arc

## 1. Concept

A coding-event poster that is nearly all black: one glowing orange arc sweeps in from the
top left like the edge of a hot planet, a second ember glows in the bottom right corner,
and across the lower middle sits a single four-letter word in huge bold type filled with
a silver gradient that fades into the dark at its right end. All the facts are small
white lines in the top right. It works because there is one light source and one word;
the metal type looks lit by the arc, so the two belong to the same scene.

## 2. Format and grid

* Portrait 3:4. Margins 6% W left, 5% W right, 9% H top, 12% H bottom.
* Two left edges for text: 6% W (web address, giant word, bottom tagline) and 74% W
  (facts column, upper right). The tagline block starts at 60% W, a step left of the
  facts column.
* Zones: light arc 0 to 50% H on the left two thirds; facts 9 to 47% H at the right;
  giant word 52 to 71% H across 5 to 89% W; footer row 81 to 88% H.
* The upper left 55% of the page carries no text at all except the web address: it
  belongs to the light.

## 3. Layer stack

1. Ground: near-black with a trace of red `#171516`.
2. Haze: a wide, soft dark-red glow `#3d1714` centered at 25% W, 30% H, radius 60% H.
3. Arc glow: a broad orange `#da3c05` band, blurred, following a curve from the left edge
   at 23% H through 30% W, 24% H and 52% W, 34% H down to 59% W, 53% H, where it dies
   out. About 14% W thick before blurring.
4. Arc core: a thin brighter line `#ff8a1a` on the same curve, lightly blurred: the hot
   rim of the arc.
5. Corner flare: yellow-orange `#ffa902` radial glow in the top left corner, radius 30%
   W, the brightest spot on the page.
6. Bottom-right ember: a second orange `#fd8803` blurred band curving from the right edge
   at 75% H down to the bottom edge at 82% W.
7. Grain: fine noise, soft-light, 30%.
8. Web address: small bold white at 6% W, 8.9% H.
9. Tagline: three lines of bold, wide-tracked white caps at 60% W, 9.7 to 16% H.
10. Facts column at 74% W: "FREE EVENT" on two lines (22.8 and 25.5% H); two speaker
    entries, each a small bold label and a larger name (32.1 / 34.4% and 38.8 / 41.3% H);
    a short white rule, 7% W long, at 46.4% H.
11. Giant word: four letters, first a capital, 83% W wide, top at 52.3% H, height 18.2% H.
    Fill: left-to-right gradient from near-white `#f2f2f2` through `#dbdbdb` and `#999999`
    to `#3a3838`, so the last letter almost sinks into the ground. Soft black shadow.
12. Footer: a thin vertical white rule at 6.3% W, 81 to 86.5% H; a three-line bold caps
    tagline at 12.1% W; a four-line bold quote at 34.5 to 80% W.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#171516` | ground | 60% |
| `#3d1714` | red haze around the arc | 15% |
| `#da3c05` | orange arc and ember | 10% |
| `#ffa902` | yellow-orange hot spots | 4% |
| `#dbdbdb` | chrome word (average) | 9% |
| `#ffffff` | small type | 2% |

Black, one warm glow family (red to yellow), and neutral metal. No cool color: the gray
of the word is exactly neutral so it picks up warmth from its surroundings.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Web address | one URL | title | neo-grotesque (Inter 700 / Helvetica Neue) | 700 | 1.6% | 0 | - | left | `#ffffff` | 6%, 8.9% |
| Tagline | 5 words on 3 lines, 6 to 10 characters per line | caps | same | 700 | 2.2% | 150 | 100% | left | `#ffffff` | 60%, 9.7 to 16% |
| Entry note | "FREE EVENT", 2 lines | caps | same | 400 | 1.5% | 120 | 125% | left | `#ffffff` | 74%, 22.8% and 25.5% |
| Speaker label (x2) | "Speaker 1 :" | title | same | 700 | 1.2% | 0 | - | left | `#ffffff` | 74%, 32.1% and 38.8% |
| Speaker name (x2) | `[SPEAKER NAME]`, 2 words | title | same | 500 | 1.9% | -10 | - | left | `#ffffff` | 74%, 34.4% and 41.3% |
| Giant word | 1 word, 4 letters, capital first | title | heavy neo-grotesque (Inter 800, Archivo 800 / Helvetica Neue Bold) | 700 | 17.5% | -40 | - | left | silver gradient | 5.3%, top 52.3% |
| Footer tagline | 3 one-word lines | caps | same | 700 | 1.5% | 150 | 125% | left | `#ffffff` | 12.1%, 81.8 to 86.2% |
| Quote | 4 lines in quotation marks, about 30 words | sentence | same | 700 | 1.1% | 0 | 125% | left | `#ffffff` | 34.5%, 81.2 to 88% |

One family. The giant word is 8x the tagline and 12x the body; nothing sits in between,
which is why the page feels like a film title.

## 6. Imagery

No photograph. The background is abstract light: a soft-edged orange arc with a brighter
rim, as if a glowing ring were seen at very close range, and an echo of it in the
opposite corner. Presented in the original as a print mockup on a blurred backdrop; that
is not part of the design.

## 7. Effects and textures

* Light arc (a drawn version of **T-light-leak**): a `shape path` cubic curve with a
  thick stroke and round caps, rasterized, `filter gaussian` (sigma 50 for the 150 px
  glow, sigma 14 for the 34 px core), `screen` blend. Two strokes on the same curve, wide
  and dim under narrow and bright, give a glow with a defined hot edge.
* Corner flare and haze: radial `fill gradient` layers with alpha stops; the flare in
  `screen` mode.
* **T-gradient-type**: `fx overlay --stops '#f2f2f2@0,#dbdbdb@0.3,#999999@0.62,#3a3838@1'
  --angle 0` on the word, plus `fx drop-shadow` (offset 0,10, blur 30, 60%).
* **T-rules**: one short horizontal rule closing the facts column, one vertical rule
  opening the footer.
* **T-paper-grain**, amount 12, soft-light 30%, below the type: essential on blurred
  orange over black, which bands badly.
* **T-ring-arcs** in [techniques.md](../techniques.md) is the tested library version of the ring, halo, or arc shapes; start from it when reusing this idea.

## 8. Hierarchy and eye path

1. The bright corner and the arc, which lead down and right.
2. The giant word, read left (bright) to right (dim).
3. Up to the tagline and the facts column at the top right.
4. The footer quote and tagline.

## 9. Build recipe

Nothing to supply. Canvas 1080x1440.

```bash
pixelox new -o code.pxo --size 1080x1440 --fill '#171516'
pixelox layer add code.pxo --in-place --name Haze
pixelox fill gradient code.pxo --in-place --layer Haze --kind radial --from 25%,30% --to 25%,90% --stops '#4a1a14e6@0,#3d171480@0.5,#3d171400@1'
pixelox shape path code.pxo --in-place --name Glow --d 'M -80 330 C 280 300 560 420 640 770 M 1160 1080 C 1000 1130 900 1290 880 1520' --fill none --stroke '#da3c05' --stroke-width 150 --cap round
pixelox layer rasterize code.pxo --in-place --layer Glow
pixelox filter gaussian code.pxo --in-place --layer Glow --sigma 50
pixelox layer blend code.pxo --in-place --layer Glow --mode screen
pixelox shape path code.pxo --in-place --name Core --d 'M -80 300 C 280 270 540 380 610 640 M 1160 1050 C 1010 1100 920 1230 890 1420' --fill none --stroke '#ff8a1a' --stroke-width 34 --cap round
pixelox layer rasterize code.pxo --in-place --layer Core
pixelox filter gaussian code.pxo --in-place --layer Core --sigma 14
pixelox layer blend code.pxo --in-place --layer Core --mode screen
pixelox layer add code.pxo --in-place --name Flare
pixelox fill gradient code.pxo --in-place --layer Flare --kind radial --from 4%,0 --to 36%,0 --stops '#ffb21aff@0,#ff8a1a99@0.4,#da3c0500@1'
pixelox layer blend code.pxo --in-place --layer Flare --mode screen
pixelox layer add code.pxo --in-place --name Grain --fill '#808080'
pixelox filter add-noise code.pxo --in-place --layer Grain --amount 12 --monochrome
pixelox layer blend code.pxo --in-place --layer Grain --mode soft-light
pixelox layer opacity code.pxo --in-place --layer Grain --value 30
pixelox text add code.pxo --in-place --name Site --content "Example.com" --font "Helvetica Neue" --weight 700 --size 2.2% --color '#ffffff' --at 6%,8.9% --anchor l
pixelox text add code.pxo --in-place --name Tagline --content $'SHIP IDEAS\nNOT JUST\nSLIDES' --font "Helvetica Neue" --weight 700 --size 3% --tracking 150 --leading 100% --color '#ffffff' --at 60%,12.9% --anchor l
pixelox text add code.pxo --in-place --name Entry --content $'FREE\nEVENT' --font "Helvetica Neue" --weight 400 --size 2.1% --tracking 120 --leading 125% --color '#ffffff' --at 74%,24.2% --anchor l
pixelox text add code.pxo --in-place --name Label1 --content "Speaker 1 :" --font "Helvetica Neue" --weight 700 --size 1.65% --color '#ffffff' --at 74%,32.1% --anchor l
pixelox text add code.pxo --in-place --name Name1 --content "Guest Engineer" --font "Helvetica Neue" --weight 500 --size 2.6% --tracking=-10 --color '#ffffff' --at 74%,34.6% --anchor l
pixelox text add code.pxo --in-place --name Label2 --content "Speaker 2 :" --font "Helvetica Neue" --weight 700 --size 1.65% --color '#ffffff' --at 74%,38.8% --anchor l
pixelox text add code.pxo --in-place --name Name2 --content "Local Founder" --font "Helvetica Neue" --weight 500 --size 2.6% --tracking=-10 --color '#ffffff' --at 74%,41.3% --anchor l
pixelox shape line code.pxo --in-place --name FactsRule --from 74%,46.4% --to 81%,46.4% --stroke '#ffffff' --stroke-width 2
pixelox text add code.pxo --in-place --name Word --content "Loop" --font "Helvetica Neue" --weight 700 --size 24% --tracking=-40 --color '#dbdbdb'
pixelox transform scale code.pxo --in-place --layer Word --size 896x
pixelox transform move code.pxo --in-place --layer Word --to 5.3%,52.3% --anchor tl
pixelox fx overlay code.pxo --in-place --layer Word --stops '#f2f2f2@0,#dbdbdb@0.3,#999999@0.62,#3a3838@1' --angle 0
pixelox fx drop-shadow code.pxo --in-place --layer Word --offset 0,10 --blur 30 --opacity 60
pixelox shape line code.pxo --in-place --name FooterRule --from 6.3%,81% --to 6.3%,86.5% --stroke '#ffffff' --stroke-width 2
pixelox text add code.pxo --in-place --name FootTag --content $'YOUR\nTAGLINE\nHERE' --font "Helvetica Neue" --weight 700 --size 2.05% --tracking 150 --leading 125% --color '#ffffff' --at 12.1%,84% --anchor l
pixelox text add code.pxo --in-place --name Quote --content $'"Spend one evening with people who\nbuild things for a living. See what\nthey are trying next, ask how it works,\nand leave with something to start."' --font "Helvetica Neue" --weight 700 --size 1.5% --leading 125% --color '#ffffff' --at 34.5%,84.6% --anchor l
pixelox export code.pxo -o code.png
```

Change first: the giant word (3 to 5 letters; it is fitted to 896 px, so shorter words
get taller), then the glow color pair (`#da3c05` and `#ff8a1a`), then the curve: the
first two numbers after each `C` pull the arc's shoulder up or down.

## 10. Variations and pairings

* Cold version: ground `#101418`, haze `#0f2a4a`, arc `#0b6fd1` with core `#5fd0ff`,
  flare `#bfe9ff`; or toxic green `#1f7a2f` / `#a6ff4d`.
* Gold word instead of chrome: overlay stops `#fff3c0@0, #f6d365@0.3, #b9861a@0.65,
  #3a2600@1`.
* Put the word at the top and the arc rising from the bottom left for a "sunrise" feel.
* Landscape 16:9: arc from the left edge, word spanning 60% W at the lower left, facts
  in a column at the right.
* Suits hackathons, product keynotes, game nights, film and album teasers. Pairs with
  T-glitch on the word, T-lens-flare at the arc's brightest point, and T-pill-button for
  registration.

## 11. Pitfalls

* A glow with no core looks like fog. Always pair the wide blurred stroke with a narrow,
  brighter one on nearly the same curve.
* A rainbow or two-hue chrome gradient on the word. Keep it neutral gray; the fade to the
  ground color at one end is what ties it to the scene.
* Fading the word so far that its last letter is unreadable: the darkest stop should
  stay at least 12% lighter than the ground.
* Text in the arc's path. Keep the upper left empty; small white type over orange glow
  loses contrast.
* Skipping the grain: blurred orange on black shows hard bands, especially after export
  to JPEG.
