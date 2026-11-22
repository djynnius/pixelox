# Addressing layers, masks, geometry, and colors

## Inputs and outputs

`pixelox <group> <command> INPUT [flags]`. INPUT is an image (JPEG, PNG, TIFF, WebP, GIF,
BMP, PSD, SVG) or a `.pxo` document. Plain images open as a one-layer document whose layer
is named `Background`, so every command works one-shot:
`pixelox filter gaussian in.jpg -o out.jpg --sigma 3`.

## Layers

`--layer REF`: a unique layer name, a layer id, `#N` (1 = the top of the panel, counting
into groups), `@top`, `@bottom`, or `@active`. Single-layer documents need no `--layer`;
documents with several layers require it where it matters (PX2302). Several layers:
`--layers a,b,c`.

## Masks

`--mask REF`: a named mask from the document's library, `@sel` (the active selection),
`@alpha:<layer>` (a layer's transparency), `@luma:<layer>` (a layer's brightness: white
reveals, black and transparent hide), `@mask:<layer>` (a layer's layer mask, like
Ctrl-clicking its thumbnail), or a grayscale image path (luma, or alpha when the image has
transparency) of the canvas size. `--invert-mask` flips it. Without a mask an op applies
everywhere. Every masking feature, with examples: `pixelox docs masking`.

To edit a layer mask in steps, load it as the selection, change the selection, and write
it back: `select load doc.pxo @mask:Sky`, then `select ellipse ... --mode subtract
--feather 80`, then `mask add doc.pxo --layer Sky --from @sel`. `paint stroke --target
mask` paints into a layer mask directly.

## Geometry

| Form | Meaning |
| --- | --- |
| `X,Y,WxH` | Rectangle (`--rect`, `--region`) |
| `X,Y` | Point (`--at`, `--from`, `--to`) |
| `WxH`, `2048x`, `x1080` | Size; a blank side keeps the aspect ratio |
| `--anchor tl\|t\|tr\|l\|c\|r\|bl\|b\|br` | Which point of a box `--at` places |

Units: px by default; `%` of the canvas; `pt`, `mm`, `in` through the document's dpi.
Negative X or Y count from the right or bottom edge: `--anchor br --at -48,-48` puts a box's
bottom-right corner 48 px from the corner.

A coordinate can also be a sum or a difference of two lengths, `A+B` or `A-B`, with no
spaces. A sum always counts from the left or top edge, so it is how you write a true
negative coordinate, for something hanging off the left or top edge:

| Value | On a 1000 px axis |
| --- | --- |
| `-80` | 920 (80 px from the far edge, as always) |
| `0-80` | −80 (80 px past the left or top edge) |
| `50%-120` | 380 (120 px before the center) |
| `100%+40` | 1040 (40 px past the far edge) |
| `10%+2in` | 100 px plus 2 inches at the document's dpi |

Sums work in every point, rectangle, size, and point list: `--at 0-120,40%`,
`--rect 0-200,0-200,600x600`, `--from 0-50,0`, `--size 100%-80x`, `--points '0-40,10 300,10'`.
They never need quoting or `--flag=` (they do not start with a minus).

Directions are degrees counter-clockwise from the right, as on a math plot: 0 points right,
90 up, 180 left, −90 down. `filter motion-blur`, `fx overlay` gradients, light directions
(`fx bevel`, `filter emboss`), flames (`fx fire`), and hatching and screen angles all use
this. Rotations that turn an object (`transform rotate --degrees`, `transform straighten`,
`shape polygon|star --rotation`) are clockwise, as on screen. Canvas scripts (`render js`)
draw in screen coordinates, where y grows downward, so a direction of 52° there leans the
other way: the same direction is `--angle=-52` here.

## Colors and gradients

`#RRGGBB`, `#RRGGBBAA`, `#RGB`, `rgb(…)`, `rgba(…)`, `hsl(…)`, CSS names, `transparent`.
Gradient stops: `'#000@0,#ff8800@0.5,#fff@1'` (positions 0..1 or percentages).

## Amounts

Photoshop-familiar ranges: opacity and densities 0–100, exposure in stops, hue ±180,
saturation ±100. Each command's page lists its ranges; out-of-range values fail with PX1013.
