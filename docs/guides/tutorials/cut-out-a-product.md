# Cut out a product shot

On a plain backdrop, a magic-wand selection of the background, inverted, is the product.

```bash
pixelox open product.jpg -o p.pxo
pixelox select color p.pxo --in-place --ref white --at 4,4 --contiguous --tolerance 40
pixelox select invert p.pxo --in-place
pixelox mask expand p.pxo --in-place --mask @sel --by 1
pixelox mask feather p.pxo --in-place --mask @sel --radius 1.5
pixelox select save p.pxo --in-place --as product
pixelox export p.pxo -o product-cut.png --apply-mask product
```

Check the coverage before exporting: `pixelox select bounds p.pxo --json` reports the
selection's box and how much of the canvas it covers.

The AI path handles busy backgrounds and hair; it runs BiRefNet-lite locally:

```bash
pixelox detect subject product.jpg --cutout cut.png -y
```

Its first use downloads the model (224 MB, once), so without `-y` it stops with exit 6 and
the command that downloads it; `--dry-run` shows the model, its size, and whether it is
installed.

Place the cutout on a new background:

```bash
pixelox new -o ad.pxo --preset ig-post --fill '#f4efe6'
pixelox comp place ad.pxo product-cut.png --in-place --as Product --fit contain
pixelox fx drop-shadow ad.pxo --in-place --layer Product --offset 0,18 --blur 30 --color '#00000055'
pixelox export ad.pxo -o ad.png
```
