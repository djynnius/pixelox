---
id: p045
title: Golden sunburst flyer with cross
genre: [church, event-flyer]
mood: [warm, festive, reverent, bold]
style: [illustration, type-led, flat]
format: portrait-2x3
archetype: A-centered-title
palette: ['#f2c037', '#fbe547', '#e69c21', '#a15b1a', '#523615', '#f9f1b2']
techniques: [T-sunburst, T-gradient-type, T-vignette]
needs: nothing
---

# p045: Golden sunburst flyer with cross

## 1. Concept

A church-event flyer drawn entirely with vector shapes and light: a plain golden cross
stands in a burst of thin white rays above a bank of clouds, the whole page glows from
white-yellow in the middle to burnt orange at the corners, and a two-line dark brown
headline sits right on the brightest spot. It works because the lightest area of the page
is directly behind the darkest type, so the headline has maximum contrast while everything
around it stays within one warm hue family.

## 2. Format and grid

* Portrait 2:3. Strict center axis at 50% W; every element is centered. Margins 9.5% W
  (the headline's first line touches them), 2.5% H top and bottom.
* Zones: presenter line 2.7% H; cross 8.7 to 52% H with its arms at 18 to 21% H; cloud
  bank 22 to 38% H; headline 35 to 63% H; script line 66 to 72% H; second script line
  78 to 85% H; three small information lines 88 to 97% H.
* Two light sources share the axis: the ray origin at 50% W, 34% H (behind the clouds, at
  the top edge of the headline) and the glow core at 50% W, 46% H (behind the headline).
* The upper 35% is picture, the middle 30% headline, the lower 35% details on a darkening
  floor.

## 3. Layer stack

1. Field: radial gradient centered at 50%,46%: cream `#fbf4b6` core, lemon `#fef132` at
   15% H radius, golden yellow `#ffd801` at the left and right edges, orange `#f0a301`
   toward the top corners, burnt orange `#ae5d0d` in the bottom corners and top right.
2. Cloud bank: two symmetric heaps of soft white and pale khaki (`#d1c881`) puffs, left
   heap 4 to 46% W, right heap 54 to 96% W, tops at 22 to 26% H, dissolving downward into
   the glow by 38% H; 75% opaque.
3. Rays: about 34 thin white lines (each 0.25 to 0.6% W thick, widening outward) fanning
   from 50%,34% over the upper 250 degrees, 75% opaque near the origin, fading to 45% at
   the canvas edge. None point straight down.
4. Cross: upright beam `47.9%,8.7%,4.2% x 43.5%`; arm `27.8%,18.1%,44.4% x 2.7%`. Flat gold
   with a soft sheen (`#d99a0a` edges, `#f8e05a` center line). The lower beam dissolves
   into the glow behind the headline.
5. Glow core: white-cream radial, 50%,46%, radius 20% H, over the cross foot.
6. Floor shade: burnt orange `#9c500a` rising from 0% at 58% H to 85% at the bottom edge.
7. Sparkles: about 45 soft white and pale yellow light points, 0.5 to 2.5% W across,
   scattered over the whole page, a few large ones near the cross arms and at 76% H.
8. Presenter line: thin white lowercase, centered, 2.7% H.
9. Headline line 1 (6 letters): dark brown, 9.5 to 90.7% W, cap top 35% H, baseline 47.2% H.
10. Headline line 2 (5 letters): same size, 19.6 to 83.2% W, cap top 50.9% H, baseline 63% H.
11. Script line 1: dark brown brush script, 2 words, 22.5 to 77.5% W, 66.5 to 72% H.
12. Script line 2: white brush script, 3 words, 17.5 to 84% W, 78 to 85.5% H, soft shadow.
13. Date and city line (white bold caps) at 88.9% H; two lines of fine print (brown) at
    92 and 93.7% H; contact line (white bold caps) at 96.4% H.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#f2c037` | golden yellow mid field, cross | 25% |
| `#fbe547` | lemon inner glow | 20% |
| `#e69c21` | orange outer field | 20% |
| `#a15b1a` | burnt orange floor and corners | 15% |
| `#f9f1b2` | cream core, clouds, rays, white type | 10% |
| `#523615` | dark brown headline, script, fine print | 10% |

Analogous yellow to orange with brown as the dark and near-white as the light: one hue
family at five lightness steps. No blue, no black; the "black" is a brown taken from the
darkest orange.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Presenter | "`[org]` presents", 3 words | lower | neo-grotesque (Arimo, Inter / Helvetica Neue) | 400 | 1.25% | 0 | - | center | `#ffffff` | 50%, 2.7% |
| Headline 1 | "CHURCH"-type word, 6 letters | caps | bold grotesque, slightly narrow (Archivo Narrow 700, Arimo Bold at 85% width / Arial Narrow) | 700 | 12.2% | 15 | - | center | `#523615` | 50%, cap top 35% |
| Headline 2 | "EVENT"-type word, 5 letters | caps | same | 700 | 12.2% | 15 | - | center | `#523615` | 50%, cap top 50.9% |
| Script 1 | "Sunday Service"-type, 2 words, 14 characters | title | slanted brush script (Yellowtail, Kaushan Script / Brush Script MT) | regular | 3.9% | 0 | - | center | `#523615` | 50%, top 66.5% |
| Script 2 | `[Church Name]`, 3 words, 16 characters | title | same | regular | 5% | 0 | - | center | `#ffffff` | 50%, top 78% |
| Date line | weekday, month, day, year, city | caps | bold neo-grotesque | 700 | 1.6% | 0 | - | center | `#ffffff` | 50%, 88.9% |
| Fine print | 2 lines, about 90 characters each | sentence | narrow sans (Pathway Gothic One, Archivo Narrow / Arial Narrow) | 400 | 1.2% | 0 | 125% | center | `#523615` | 50%, 92% and 93.7% |
| Contact | "MORE INFO:" + address, 1 line | caps | bold neo-grotesque | 700 | 1.6% | 0 | - | center | `#ffffff` | 50%, 96.4% |

Three voices: a plain bold grotesque (narrow for the headline, regular width for the two
white lines), a brush script (two lines in two colors), and a narrow sans for fine print. Colors alternate brown, brown,
white, white, brown, white down the page, which keeps six stacked centered lines from
merging.

## 6. Imagery

No photograph. The picture is built from five simple parts: a radial field, a row of
puffy clouds, a fan of thin rays, a two-rectangle cross, and light points. The cross is a
plain Latin cross with proportions 1 : 10 (beam width to height), arm at one quarter of the
height, no bevel, no outline. The clouds are translucent and symmetric, like stage scenery
parted to both sides of the beam. Everything is soft except the cross and the type.

## 7. Effects and textures

* **T-sunburst**, thin-line variant: `rays.js` with `duty=0.07` (hairline rays), `count=34`,
  `spread=250`, `dir=-90` (fan opens upward), `fade=0.4`; the Core gradient of the
  technique moved to 46% H so the hot spot is behind the headline, not behind the cross.
* Clouds: `blobs.js` discs with `soft=0.3` at twelve fixed centers, each drawn twice (a
  khaki disc 1.2% H lower, then a white one on top) so every puff has a shaded underside;
  blurred sigma 4, 80% opaque.
* **T-gradient-type** gold stops applied to shapes: `fx overlay` on the two cross
  rectangles, across the beam's width and across the arm's height.
* **T-vignette** as a one-sided floor: a linear burnt-orange shade over the bottom 42% so
  white type reads there.
* Sparkles: `particles.js` `shape=glow` in screen mode.
* Soft shadow on the white script only (offset 0,4, blur 10, 40%, color `#6b3200`).

## 8. Hierarchy and eye path

1. The headline (darkest shape on the brightest ground, middle of the page).
2. Up the beam to the cross and out along the rays.
3. Down to the brown script line (what kind of event), then the white script (who).
4. The white date line and contact line; the brown fine print is read last or not at all.

Cap-height ratio: headline 12.2 : white script 5 : brown script 3.9 : date and contact
1.6 : presenter 1.25 : fine print 1.2, about 10 : 4 : 3.2 : 1.3 : 1 : 1.

## 9. Build recipe

Nothing to supply: every layer is generated.

```bash
pixelox render js scripts/rays.js -o rays.png --size 1080x1620 --param color='#ffffff' --param cx=0.5 --param cy=0.34 --param count=34 --param duty=0.07 --param spread=250 --param dir=-90 --param alpha=0.75 --param fade=0.4
pixelox render js scripts/blobs.js -o clouds.png --size 1080x1620 --param bg=transparent --param colors='#c2b67e,#ffffff' --param at='0.25,0.285;0.25,0.27;0.75,0.285;0.75,0.27;0.36,0.312;0.36,0.3;0.64,0.312;0.64,0.3;0.14,0.322;0.14,0.31;0.86,0.322;0.86,0.31;0.05,0.345;0.05,0.335;0.95,0.345;0.95,0.335;0.3,0.34;0.3,0.33;0.7,0.34;0.7,0.33;0.2,0.35;0.2,0.34;0.8,0.35;0.8,0.34' --param size=0.095 --param vary=0.1 --param soft=0.3 --param alpha=0.85
pixelox render js scripts/particles.js -o sparks.png --size 1080x1620 --seed 7 --param shape=glow --param colors='#ffffff,#fff6b0' --param count=46 --param min=0.3 --param max=1.3 --param amin=0.5
pixelox new -o glory.pxo --size 1080x1620 --fill '#f0a301'
pixelox layer add glory.pxo --in-place --name Field
pixelox fill gradient glory.pxo --in-place --layer Field --kind radial --from 50%,46% --to 50%,112% --stops '#fbf4b6@0,#fef132@0.2,#ffd801@0.45,#f0a301@0.72,#b2600e@1'
pixelox comp place glory.pxo clouds.png --in-place --as Clouds --fit cover
pixelox filter gaussian glory.pxo --in-place --layer Clouds --sigma 4
pixelox layer opacity glory.pxo --in-place --layer Clouds --value 80
pixelox comp place glory.pxo rays.png --in-place --as Rays --fit cover
pixelox shape rect glory.pxo --in-place --name CrossV --rect 47.9%,8.7%,4.2%x43.5% --fill '#e9b417'
pixelox fx overlay glory.pxo --in-place --layer CrossV --stops '#d99a0a@0,#f8e05a@0.5,#e0a40e@1' --angle 0
pixelox shape rect glory.pxo --in-place --name CrossH --rect 27.8%,18.1%,44.4%x2.7% --fill '#e9b417'
pixelox fx overlay glory.pxo --in-place --layer CrossH --stops '#d99a0a@0,#f8e05a@0.5,#e0a40e@1' --angle 90
pixelox layer add glory.pxo --in-place --name Core
pixelox fill gradient glory.pxo --in-place --layer Core --kind radial --from 50%,46% --to 50%,66% --stops '#fffbe0f2@0,#fff6b000@1'
pixelox layer add glory.pxo --in-place --name Floor
pixelox fill gradient glory.pxo --in-place --layer Floor --from 0,58% --to 0,100% --stops '#9c500a00@0,#9c500ad9@1'
pixelox comp place glory.pxo sparks.png --in-place --as Sparks --fit cover
pixelox layer blend glory.pxo --in-place --layer Sparks --mode screen
pixelox text add glory.pxo --in-place --name Presenter --content "your church presents" --font "Helvetica Neue" --weight 400 --size 2.1% --color '#ffffff' --at 50%,2.7% --anchor c
pixelox text add glory.pxo --in-place --name H1 --content "FAMILY" --font "Arial Narrow" --weight 700 --size 17% --tracking 15 --color '#523615'
pixelox transform move glory.pxo --in-place --layer H1 --to 50%,35% --anchor t
pixelox text add glory.pxo --in-place --name H2 --content "FEAST" --font "Arial Narrow" --weight 700 --size 17% --tracking 15 --color '#523615'
pixelox transform move glory.pxo --in-place --layer H2 --to 50%,50.9% --anchor t
pixelox text add glory.pxo --in-place --name Script1 --content "Harvest Sunday" --font "Brush Script MT" --size 7% --color '#523615'
pixelox transform scale glory.pxo --in-place --layer Script1 --size 594x
pixelox transform move glory.pxo --in-place --layer Script1 --to 50%,66.5% --anchor t
pixelox text add glory.pxo --in-place --name Script2 --content "Hope Community" --font "Brush Script MT" --size 9% --color '#ffffff'
pixelox transform scale glory.pxo --in-place --layer Script2 --size 718x
pixelox transform move glory.pxo --in-place --layer Script2 --to 50%,78% --anchor t
pixelox fx drop-shadow glory.pxo --in-place --layer Script2 --offset 0,4 --blur 10 --color '#6b3200' --opacity 40
pixelox text add glory.pxo --in-place --name DateLine --content "SUNDAY OCT 12  2031  YOUR CITY" --font "Helvetica Neue" --weight 700 --size 2.2% --color '#ffffff' --at 50%,88.9% --anchor c
pixelox text add glory.pxo --in-place --name FinePrint --content $'A shared table after the morning service: bring one dish, one guest, and one story to tell.\nChildren are welcome, parking is free, and the doors open half an hour before we begin.' --font "Arial Narrow" --weight 400 --size 1.65% --leading 125% --align center --color '#523615' --at 50%,92.9% --anchor c
pixelox text add glory.pxo --in-place --name Contact --content "MORE INFO: WWW.EXAMPLE.COM" --font "Helvetica Neue" --weight 700 --size 2.2% --color '#ffffff' --at 50%,96.4% --anchor c
pixelox export glory.pxo -o glory.png
```

Change first: the two headline words. They share one font size (`--size 17%`), so the
longer word sets the limit: 6 letters fill about 80% W; for 7 letters use `--size 14.5%`,
for 8 use `--size 12.5%`, and keep the cap tops at 35% and (35% + cap height + 3.7%) H. Then the
script lines (the `transform scale` widths are 55% and 66.5% W).

## 10. Variations and pairings

* Cool dawn: field `#e9f6ff / #9fd8ff / #3d9be0 / #1f5fb0 / #123a78`, cross white with a
  pale blue sheen, headline navy `#0c2350`. Same structure, baptism or Easter-morning feel.
* Night of prayer: field `#fff3c4 / #f0a301 / #7a3206 / #2a0f03 / #140600` (a small hot
  core in the dark), headline cream instead of brown, rays at alpha 0.35.
* Replace the cross with another flat emblem built from 2 or 3 shapes (a dove from two
  arcs, a star, a crown, a trophy for an awards night) and keep rays, clouds, and glow.
* Swap the grotesque headline for a heavy condensed face fitted with T-fit-width when the
  words are longer than 7 letters.
* Pairs with T-bokeh (larger, fewer discs instead of sparkles), T-arc-type for a roundel
  at the cross center, and T-paper-grain (amount 10, soft-light 30%) to hide banding in
  the big radial field.

## 11. Pitfalls

* Putting the glow behind the cross instead of behind the headline. The brown type then
  sits on mid-orange and loses half its contrast. Core at 46% H, rays from 34% H.
* Thick sunburst wedges (`duty` 0.5): they turn the calm glow into a retro sale poster
  and fight the headline. Keep `duty` under 0.1 here.
* White headline type on this ground: the middle of the page is almost white. White is
  for the lower third only, on the Floor shade; the headline must be the dark brown.
* Black instead of brown: pure black on yellow reads as a warning sign. Use `#523615` or
  another brown with the field's hue.
* Clouds that are too opaque or gray look like smoke. Keep them at 80% or less, tinted
  toward the field's cream and khaki, and let the glow eat their lower edge.
