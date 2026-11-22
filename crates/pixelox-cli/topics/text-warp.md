# Live type: fit, scale, case, faux bold, and warps

Every setting here stays live: the layer is still text, `text edit` changes the words and the
setting is applied again, and the outlines stay vector (crisp at any size, `text to-path`
keeps them).

## Fit a width or a box

`--fit-width W` sizes the type so its visible letters (the ink, stroke included) span exactly
`W` px; `--fit-box WxH` makes it as large as fits. With `--anchor-to ink`, `--at` places the
visible letters instead of the text box, so a stacked block needs one command per line:

```bash
pixelox new -o block.pxo --size 1080x1350 --fill '#f1ede4'
pixelox text add block.pxo --in-place --name L1 --content "MAKE" --font @condensed-heavy --fit-width 920 --at 80,90 --anchor-to ink --color '#111111'
pixelox text add block.pxo --in-place --name L2 --content "NOISE" --font @condensed-heavy --fit-width 920 --at 80,457 --anchor-to ink --color '#e4002b'
pixelox text edit block.pxo --in-place --layer L2 --content "SIGNAL"
```

The result's `bounds` give the next line's position (`y + h + gap`). `--size` given later
replaces the fit; `--fit-width 0` removes it.

## Scale, case, faux bold

```bash
pixelox text style block.pxo --in-place --layer L1 --scale-x 80%
pixelox text add block.pxo --in-place --content "night of praise" --case upper --font @geometric-sans --size 3% --tracking 200
pixelox text style block.pxo --in-place --layer L1 --faux-bold 25
pixelox transform scale block.pxo --in-place --layer L1 --size 920x300
```

`--scale-x` / `--scale-y` stretch the set text about its anchor (Photoshop's character
scaling). `transform scale` with two different factors now sets the font size and
`scale_x` instead of turning the text into pixels. `--case` sets the content in upper, lower,
or title case without retyping it. `--faux-bold` thickens every stem by N/1000 em with sharp
corners.

## Warp presets

`text warp --preset P` (or `text style --warp P`) bends the glyph outlines with one of
Photoshop's 15 warp styles. `--bend` is the strength (-100..100, default 50; negative bends
the other way), `--h-distort` and `--v-distort` add a perspective that grows the right side
or the bottom, and `--orient vertical` bends along the vertical axis. The warp fills the
bounds of the unwarped letters.

| Preset | Positive bend |
| --- | --- |
| `arc` | the line bends into a ring sector of bend x 180° (a frown) |
| `arc-lower` | top straight, bottom sags in the middle |
| `arc-upper` | bottom straight, top rises in the middle |
| `arch` | every column lifts, letters stay upright |
| `bulge` | the middle grows up and down |
| `shell-lower` | fan: top straight, bottom spreads and sags |
| `shell-upper` | fan: bottom straight, top spreads and rises |
| `flag` | one wave period, letters stay upright |
| `wave` | like flag with the bottom a quarter period ahead |
| `fish` | swells on the left, narrows to the right |
| `rise` | climbs to the right in an S |
| `fisheye` | the middle letters grow, the ends shrink |
| `inflate` | a rounder, stronger swell |
| `squeeze` | the middle letters are pressed flat |
| `twist` | a vortex: the middle turns, the ends stay |

```bash
pixelox text warp poster.pxo --in-place --layer Title --preset arc --bend 35
pixelox text warp poster.pxo --in-place --layer Title --preset rise --bend 30 --h-distort 25
pixelox text style poster.pxo --in-place --layer Title --warp flag --bend -20
pixelox text warp poster.pxo --in-place --layer Title --preset none
```

Text on a path is not warped (PXW4015); the warp applies again when the text leaves the path.
The same preset names and parameters work on pixel layers with `transform warp --preset`.

## Text on a path that does not fit

`text on-path` reports the glyphs that run past the end of an open path, or overlap the start
of a closed one, with PXW4012 (`overflow_glyphs` in the result). `--fit shrink` lowers the
size until the line fits:

```bash
pixelox text on-path badge.pxo --in-place --layer Arc --follow Ring --fit shrink
```
