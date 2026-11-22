# Masking: layer masks, selections, clipping, and Blend If

Yes, pixelox has masks and mask layers, with Photoshop's semantics: **white reveals, black
hides**, gray is partly visible. Every mask is a grayscale image the size of the canvas,
so it stays put when the layer it belongs to is smaller. This page lists every way to make,
edit, and use one.

| You want | Use |
| --- | --- |
| Hide part of one layer, editably | a **layer mask**: `mask add --layer L ...` |
| Fade a photo into the background | `mask add --layer L --gradient linear --from 0,60% --to 0,100%` |
| A photo inside text or a shape | `layer clip` (photo above the text), or a live mask `mask add --from-layer` |
| Limit a filter, fill, or adjustment to a region | a **selection** or named mask: `--mask @sel` |
| Keep a region for later | a **named mask**: `select save --as sky` |
| Show a layer only over dark or light parts | **Blend If**: `layer blend-if` |
| A transparent PNG cutout | `export --apply-mask`, or `mask remove --apply` |

## Layer masks

A layer mask belongs to one layer and never changes its pixels: `mask disable` shows the
layer whole again, `mask remove` drops the mask, `mask remove --apply` bakes it into the
layer's alpha. `layer list` and `mask list` show every mask with its coverage.

```bash
# from the active selection (the default source when there is one)
pixelox select ellipse doc.pxo --in-place --rect 20%,10%,60%x60% --feather 40
pixelox mask add doc.pxo --in-place --layer Photo
# from a mask reference, an image, or SVG path data (a vector mask)
pixelox mask add doc.pxo --in-place --layer Photo --from sky --combine subtract
pixelox mask add doc.pxo --in-place --layer Photo --from-file matte.png
pixelox mask add doc.pxo --in-place --layer Top --from-path 'M 0 0 H 1080 V 500 L 0 640 Z'
# all revealed (paint black to hide) or all hidden (paint white to reveal)
pixelox mask add doc.pxo --in-place --layer Shade --hide-all
pixelox paint stroke doc.pxo --in-place --layer Shade --target mask --points '300,400 700,450' --size 200 --hardness 0 --color white
```

Live properties (change them any time, nothing is lost): `mask feather --layer L --radius
12` (a soft edge, 0 removes it), `mask density --layer L --value 60` (the mask hides at
most 60%), `mask disable` / `mask enable`. Pixel edits: `mask expand`, `mask contract`,
`mask smooth`, `mask invert`, and `mask refine` (hair-level edges: `pixelox docs
mask.refine`). Moving or transforming a layer moves its mask with it.

### Gradient masks: fade a layer out

`--gradient linear` reveals the layer fully at `--from`, hides it fully at `--to`, and
ramps in between; `--gradient radial` reveals around the center `--from` and hides at the
radius `|--to - --from|`. `--ease smooth` rounds the ends of the ramp so there is no
visible start or end line.

```bash
# the photo fades into the background color from 60% of the height down
pixelox mask add doc.pxo --in-place --layer Photo --gradient linear --from 0,60% --to 0,100%
# fade toward the top instead, with a smooth ramp
pixelox mask add doc.pxo --in-place --layer Photo --gradient linear --from 0,45% --to 0,10% --ease smooth
# a soft oval vignette
pixelox mask add doc.pxo --in-place --layer Photo --gradient radial --from 50%,45% --to 50%,100% --ease smooth
```

The area the layer fades into shows whatever is below it, usually the background color.
`--combine` merges the new mask with the layer's existing one instead of replacing it:
`intersect` (visible only where both reveal: fade the bottom of a cutout and keep its
outline), `add` (either), `subtract` (the existing minus the new).

```bash
pixelox mask add doc.pxo --in-place --layer Subject --gradient linear --from 0,70% --to 0,95% --combine intersect
```

### Masks from another layer

| Reference | Mask |
| --- | --- |
| `@alpha:<layer>` | that layer's outline: where it is opaque (styles not included) |
| `@luma:<layer>` | that layer's brightness where it is opaque: white reveals, black and transparent hide |
| `@mask:<layer>` | that layer's own layer mask |

These are copies, taken when the command runs. For a mask that **follows** another layer,
use `--from-layer`: the mask is recomputed whenever the source changes, so the text of a
title stays editable, and a shape layer used as the source is a resolution-independent
vector mask.

```bash
# a photo shows only inside the word; hide the word itself
pixelox mask add doc.pxo --in-place --layer Photo --from-layer Title
pixelox layer hide doc.pxo --in-place --layer Title
pixelox text edit doc.pxo --in-place --layer Title --content "WOLF"
# the photo everywhere except the word (the mask stays live)
pixelox mask invert doc.pxo --in-place --layer Photo
# reveal a layer where a texture is bright (a luminosity mask), live or as a copy
pixelox mask add doc.pxo --in-place --layer Glow --from-layer Clouds --channel luma
pixelox mask add doc.pxo --in-place --layer Glow --from @luma:Clouds
```

Painting into, refining, expanding, or otherwise editing a live mask's pixels turns it into
an ordinary mask holding the current shape. A layer cannot use itself or a group it is
inside as its source (PX2331).

## Clipping masks

`layer clip` attaches a layer to the nearest unclipped layer below it (the base): the
clipped layer shows only where the base is opaque, and the base's outline, opacity, and
mask apply to the whole clipping group. Clip several layers in a row onto one base; each
`layer clip` attaches one layer, `--off` releases it. Adjustment layers can be clipped too,
so they change only the base.

```bash
pixelox text add doc.pxo --in-place --name Word --content "WILD" --size 30% --at 50%,50% --anchor c --weight 900
pixelox comp place doc.pxo forest.jpg --in-place --as Photo --fit cover
pixelox layer clip doc.pxo --in-place --layer Photo
pixelox layer add doc.pxo --in-place --name Tint --fill '#ff66004d'
pixelox layer clip doc.pxo --in-place --layer Tint
pixelox adjust hsl doc.pxo --in-place --saturation -40 --as-layer --name Mute
pixelox layer clip doc.pxo --in-place --layer Mute
```

Photo, Tint, and Mute now all show only inside the word; the word stays live text.

## Selections and named masks

A selection is the document's active mask, `@sel`. `select rect`, `ellipse`, `poly`,
`path`, `color` (magic wand with `--contiguous`), `luma`, `sky`, and `skin` make one;
`--mode add|subtract|intersect` combines it with the current one and `--feather` softens
it. `select invert`, `select none`, `select bounds` (where it is). `select save --as NAME`
keeps it in the document's library of named masks; `select load NAME` brings it back.

```bash
pixelox select color doc.pxo --in-place --ref white --at 4,4 --contiguous --tolerance 40
pixelox select invert doc.pxo --in-place
pixelox select save doc.pxo --in-place --as product
pixelox mask import doc.pxo matte.png --as hair --in-place
pixelox mask export doc.pxo --mask product -o product-mask.png
```

Any command with `--mask` takes a selection, a named mask, `@alpha:`, `@luma:`, `@mask:`,
or a grayscale image path, and `--invert-mask` flips it: filters, adjustments, fills,
retouching, and paint all stay inside it. (Filters on placed, text, and shape layers are
live smart filters, which cannot take `--mask`: mask the layer itself instead.)

```bash
pixelox filter gaussian doc.pxo --in-place --layer Background --sigma 8 --mask product --invert-mask
pixelox adjust curves doc.pxo --in-place --layer Background --points '0,0 128,150 255,255' --mask @luma:Background
pixelox fill solid doc.pxo --in-place --layer Paint --color '#ffcc00' --mask @sel
```

To edit a layer mask in steps, load it as the selection, change it, and write it back:
`select load doc.pxo @mask:Photo`, then `select ellipse ... --mode subtract`, then
`mask add doc.pxo --layer Photo --from @sel`.

## Blend If

`layer blend-if` shows a layer only where its own tones (`--this`) or the tones below it
(`--underlying`) fall inside a range, with split sliders for a soft ramp: `--this 20,70,255,255`
drops a texture's dark parts, `--underlying 0,0,110,190` paints shadows only over darker
areas. It works on every layer kind, adjustment layers included.

## Cutouts

```bash
pixelox detect subject shot.pxo --in-place --layer Photo --layer-mask -y
pixelox mask refine shot.pxo --in-place --mask @mask:Photo --decontaminate
pixelox export shot.pxo -o cutout.png --apply-mask @mask:Photo
pixelox mask remove shot.pxo --in-place --layer Photo --apply
```

`detect subject` needs a one-time model download (ask first; `-y`). Without it, cut a
subject on a plain backdrop with `select color --contiguous`, anything else with `select
path`. `pixelox docs mask.refine` covers hair-level edges.
