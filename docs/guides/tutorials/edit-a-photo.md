# Edit a photo

This tutorial grades an underexposed interior and delivers a print TIFF and a web JPEG.

```bash
pixelox open interior.jpg -o shot.pxo
pixelox adjust white-balance shot.pxo --in-place --auto
pixelox adjust shadows-highlights shot.pxo --in-place --shadows 45 --highlights 20
pixelox adjust curves shot.pxo --in-place --points '0,8 96,118 255,246'
pixelox adjust vibrance shot.pxo --in-place --vibrance 25 --as-layer
pixelox filter smart-sharpen shot.pxo --in-place --amount 60 --radius 1
```

`--as-layer` made the vibrance boost an adjustment layer: change its opacity later with
`pixelox layer opacity shot.pxo --in-place --layer Vibrance --value 60`, or hide it.

Straighten a tilted horizon and crop to 4:5:

```bash
pixelox transform straighten shot.pxo --in-place --angle auto
pixelox transform crop shot.pxo --in-place --ratio 4:5 --gravity c
```

Deliver:

```bash
pixelox export shot.pxo -o interior-final.tiff --bit-depth 16
pixelox export shot.pxo -o interior-web.jpg --size 2048x --quality 88
pixelox analyze stats interior-final.tiff --json
```

`analyze stats` reports per-channel clipping: if the highlights clip, lower the curve's
top point and export again with `--overwrite`.
