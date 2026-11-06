# Extract a subject (hair and all)

Photoshop's Extract filter cut out hair and fur by solving the uncertain edge pixel by pixel
and cleaning the background's color out of it. `mask refine` does the same from any rough
mask: start with `detect subject` (or a selection), then refine the edge.

```bash
pixelox open portrait.jpg -o cut.pxo
pixelox detect subject cut.pxo --in-place --save-mask subject -y
pixelox mask refine cut.pxo --in-place --mask subject --decontaminate
pixelox export cut.pxo -o cutout.png --apply-mask subject
```

`detect subject` downloads its model (224 MB) the first time: ask before passing `-y`.
`--decontaminate` replaces the colors in the edge band with the subject's own, so a blue
backdrop does not leave a blue fringe; without it, only the mask changes.

## Hard edges: paint Extract's highlighter

The band `mask refine` solves is `--band` px either side of the rough edge (auto: 0.8% of
the long side). Strands that fly further out need a wider band, or paint the uncertain
region yourself, the way Extract's highlighter worked:

```bash
pixelox paint stroke cut.pxo --in-place --into band --points '1700,520 2100,640 2450,1100' --size 260 --color white
pixelox mask refine cut.pxo --in-place --mask subject --unknown band --decontaminate
```

For hair of one solid color against a busy backdrop, Extract's force-foreground option
helps: `pixelox mask refine cut.pxo --in-place --mask subject --force-foreground '#5a3b28'`.

## Without the model

Any rough mask works: a magic-wand selection, a traced path, or `select sky` inverted. For a
product on a plain white backdrop:

```bash
pixelox open product.jpg -o product.pxo
pixelox select color product.pxo --in-place --ref white --at 4,4 --contiguous --tolerance 40
pixelox select invert product.pxo --in-place
pixelox mask refine product.pxo --in-place --mask @sel --into subject --decontaminate
pixelox export product.pxo -o product-cut.png --apply-mask subject
```

## Check it

Put the cutout on a contrasting color and look at the hair at 100%:

```bash
pixelox new -o check.pxo --size 3000x4000 --fill '#7f8c8d'
pixelox comp place check.pxo --in-place cutout.png --as Cutout
pixelox export check.pxo -o check.png --region 1650,450,900x900
```

A blue or green rim means the band missed some edge (widen `--band` or paint `--unknown`);
gray mush instead of strands means the band is too wide for the backdrop's detail.
