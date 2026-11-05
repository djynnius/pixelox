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
`@alpha:<layer>` (a layer's transparency), `@mask:<layer>` (a layer's layer mask, like
Ctrl-clicking its thumbnail), or a grayscale image path (luma, or alpha when the image has
transparency) of the canvas size. `--invert-mask` flips it. Without a mask an op applies
everywhere.

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

## Colors and gradients

`#RRGGBB`, `#RRGGBBAA`, `#RGB`, `rgb(…)`, `rgba(…)`, `hsl(…)`, CSS names, `transparent`.
Gradient stops: `'#000@0,#ff8800@0.5,#fff@1'` (positions 0..1 or percentages).

## Amounts

Photoshop-familiar ranges: opacity and densities 0–100, exposure in stops, hue ±180,
saturation ±100. Each command's page lists its ranges; out-of-range values fail with PX1013.
