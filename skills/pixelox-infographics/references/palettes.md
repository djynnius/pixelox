# Palettes for infographics

In an infographic a color is a statement: "this is group A", "this is more", "look here".
Pick the roles first, then the hexes. Every palette below gives the roles a chart needs, taken
from the studied designs, with the `--param` values to pass to the chart scripts.

## Color roles

| Role | What it does | How many |
| --- | --- | --- |
| Ground | the page; also `bg`-colored separators (`sep=`, `halo=`) | 1, plus 1 or 2 panel tints |
| Ink | titles, numbers, labels (`ink=`) | 1 |
| Muted | secondary text, axis labels (`muted=`) | 1 |
| Grid | gridlines, rules, dividers, empty tracks (`grid=`) | 1 |
| Series | the data: one color per category, in a fixed order (`colors=`) | 1 to 6 |
| Accent | the one thing the title talks about (`accent=`, `highlight=`) | 1 |
| Neutral data | "other", "vacant", "undecided", "the rest" | 1 gray |

## Three kinds of data color

* **Categorical** (groups with no order: parties, channels, products): hues that differ
  clearly, similar in lightness so none shouts. 6 at most; the 7th group is "Other" in gray.
* **Sequential** (more or less of one thing: a value per region, a level): one hue from light
  (low) to dark (high). Pass two hexes: `ramp='#dbeafe,#1d4ed8'`.
* **Diverging** (two directions from a middle: lean A to lean B, below to above average): two
  hues meeting in a light neutral. Pass three: `ramp='#c8241b,#f1f1f1,#1f4ea3'`, or five
  classes as categorical colors.

Rules that keep color honest:

1. **Same color, same meaning, everywhere.** If Party A is red in the seat chart it is red in
   the poll ring, the stat row, and the legend. Never reuse a data color as decoration.
2. **One accent.** Make everything gray or one calm hue and spend the accent on the bar,
   slice, or number the title names.
3. **Gray is data too:** use it for "other", "no answer", past periods, and context series.
4. **Order the legend like the chart**, and prefer labels on the marks to a legend.
5. **Do not rely on red against green.** About 1 man in 12 cannot tell them apart: pair blue
   with orange, or add icons, labels, or a lightness difference.
6. **Dark grounds:** lighten every series one step (a 500 becomes a 400), keep `ink` white
   and `muted` at 65 to 75% lightness, and set `grid` 6 to 10% lighter than the ground.
7. **Conventions carry meaning:** red and blue for the two large US parties; green up and
   red down for change; red for heat or danger. Follow them or say clearly that you do not.

## Named palettes

Hexes are in the order ground, panel tint, ink, muted, grid; then the series; then the accent.

| Name | Ground, panel, ink, muted, grid | Series (in order) | Accent | Mood, use |
| --- | --- | --- | --- | --- |
| Newsroom light | `#f4f6f8` `#ffffff` `#16233b` `#52606d` `#d5dbe1` | `#12897b` `#7fdfd4` `#3b7ddd` `#9aa5b1` | `#f2b705` | clean, editorial: chart cards |
| Paper and ember | `#f6f3ec` `#ffffff` `#1f2933` `#52606d` `#d9d2c3` | `#9aa5b1` `#52606d` `#cbd2d9` | `#e4572e` | calm with one loud accent: a single finding |
| Civic dark | `#122e39` `#0e2630` `#ffffff` `#c4cdd0` `#24424d` | `#b3261c` `#1466a3` `#aeb4b3` | `#e9e3a6` | authoritative: results sheets |
| Violet report | `#f2effd` `#ffffff` `#01014f` `#4a4a82` `#d9d4f3` | `#01014f` `#a341b0` `#6d3fd6` `#c9c1ef` | `#6d3fd6` | corporate, friendly: banded explainers |
| Report cards | `#eef0fb` `#ffffff` `#1e1b6b` `#5b5b8a` `#dfe2f5` | `#3f37c9` `#7b6cf6` `#f2b705` `#f97316` `#e4572e` | `#f97316` | dashboards of cards |
| Ember and slate | `#f4f6f8` `#ffffff` `#22303f` `#5c6b7a` `#d5dbe1` | `#22303f` `#f0452c` | `#f0452c` | two-color data sheets, strong contrast |
| Candy steps | `#ffffff` `#f7f5fb` `#3d3566` `#6b6b76` `#e3e0ee` | `#fd4e7d` `#7b5cf0` `#1fe0b3` `#ffb400` | `#d12f8e` | playful: processes, metaphor objects |
| Signal yellow | `#ffc900` `#ffffff` `#1a1a1a` `#1a1a1a` `#1a1a1a` | `#ffffff` `#1a1a1a` | `#1a1a1a` | bold: step sheets on one color |
| Deep teal | `#0f5c55` `#0b4a44` `#ffffff` `#c9f2ea` `#1b7268` | `#25d0b4` `#ffffff` `#128a78` | `#25d0b4` | one-hue illustrated explainers |
| Neon board | `#1c1b3a` `#25244a` `#ffffff` `#9aa3c7` `#2f2e58` | `#22d3ee` `#e879f9` `#facc15` `#fb7185` `#a78bfa` | `#facc15` | futuristic dashboards |
| Almanac | `#1f3a68` `#fbf7ef` `#fbf7ef` `#c9d3e6` `#3a5488` | `#ddd6c5` `#fbf7ef` `#7fb069` `#e4572e` | `#f2a541` | retro maps with value badges |
| Coral and gold | `#f76c6c` `#ffc145` `#ffffff` `#fff3d6` `#f99a9a` | `#ffc145` `#ffffff` `#5a2a2a` | `#ffffff` | warm two-panel sheets |
| Ballot light | `#ffffff` `#f4f4f2` `#2b2b2b` `#6b6b6b` `#dcdcdc` | `#2b3990` `#a9d8ee` `#c1272d` `#f1c7cb` `#f4e27a` | `#f7941d` | light results sheets: dark and pale pairs per bloc |
| Olive on cream | `#f3efdc` `#e8e3c8` `#2f3319` `#5c6238` `#d6d1b0` | `#5e6b1f` `#9aa63a` `#c9cf7a` `#2f3319` | `#d9662b` | earthy illustrated explainers: food, farming |
| Candy cards | `#f4f5f7` `#ffffff` `#3a3f4a` `#7b8190` `#e1e4ea` | `#ef4b6c` `#f6a03c` `#3cb8c9` `#8bc34a` | `#2f6fb3` | bright stat cards on pale gray |
| Slate blues | `#f5f7fa` `#ffffff` `#22344d` `#5b6b80` `#d9e0e8` | `#22344d` `#3f6b9a` `#6fa3cf` `#a9cbe6` | `#f2b705` | calm corporate process sheets |
| Spectrum steps | `#f5f5f5` `#ffffff` `#2b2b2b` `#6b6b6b` `#dcdcdc` | `#f6a21b` `#f0563a` `#e02d74` `#18a7e0` `#7a6fd0` `#19b394` `#8fd14f` | `#2b2b2b` | one hue per step, 5 to 7 steps in spectrum order |
| Forest report | `#e9f5ef` `#ffffff` `#0b3d2e` `#3f6b5a` `#cfe6da` | `#0b3d2e` `#2e8b57` `#7fc8a9` | `#f2b705` | environment, health |

### Ramps

| Name | `ramp=` | Use |
| --- | --- | --- |
| Blues | `'#dbeafe,#1d4ed8'` | any neutral quantity |
| Teals | `'#d5f5ef,#0f766e'` | health, environment |
| Ambers | `'#fef3c7,#b45309'` | heat, cost, risk |
| Violets | `'#ede9fe,#5b21b6'` | tech, media |
| Slate | `'#e2e8f0,#1e293b'` | print, grayscale-safe |
| Lean (diverging, 3) | `'#c8241b,#f1f1f1,#1f4ea3'` | two sides around even |
| Lean (7 classes) | `colors='#1b3a8f,#3f6fd1,#9dbbe8,#7b3fa0,#f1b3ad,#e0584f,#b3140f'` | win A, likely A, leaning A, open, leaning B, likely B, win B |
| Spectrum | `colors='#f6a21b,#f0563a,#e02d74,#7a6fd0,#18a7e0,#19b394'` | ordered steps that should each keep their own hue |
| Lean (5 classes) | `colors='#1f4ea3,#9dbbe8,#c9ced6,#f1b3ad,#c8241b'` | solid A, leaning A, open, leaning B, solid B |
| Change (diverging) | `'#b45309,#f5f5f4,#0f766e'` | below and above a baseline, colorblind-safe |

## Using a palette

```bash
pixelox new -o d.pxo --size 1080x1080 --fill '#122e39'
pixelox render js scripts/bars.js -o bars.png --size 1080x1080 --param box=8,24,84,62 --param dir=h --param values=48,37,9,6 --param labels='Party A|Party B|Party C|Others' --param colors='#b3261c,#1466a3,#e9e3a6,#aeb4b3' --param suffix=% --param ink='#ffffff' --param muted='#c4cdd0' --param grid='#24424d' --param fs=26
pixelox comp place d.pxo bars.png --in-place --as Bars --at 0,0
pixelox text add d.pxo --in-place --name Title --content "Party A leads by eleven points" --font @geometric-sans --weight 800 --size 56 --color '#ffffff' --at 8%,8% --anchor tl --anchor-to ink
pixelox text add d.pxo --in-place --name Source --content "Example data" --font @geometric-sans --size 18 --color '#c4cdd0' --at 8%,94% --anchor bl
pixelox export d.pxo -o palette-civic-dark.png
```

* Give every chart on the page the same `ink`, `muted`, `grid`, and `colors` order.
* Sample a brand's colors from its logo with `pixelox analyze colors logo.png --k 5 --json`,
  then assign roles: the darkest as ink, the most saturated as accent, tints of one hue as
  series.
* Check text contrast on the export, not by eye on the palette: a lightness difference of 50
  points or more between text and its ground.
