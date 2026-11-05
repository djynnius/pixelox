# Make a poster from a template

A template is a document with typed slots: text, image, and color.

```bash
pixelox new -o poster.pxo --preset a3 --fill '#101014'
pixelox shape rect poster.pxo --in-place --rect 0,0,100%x12% --fill '#0ea5e9' --name Accent
pixelox text add poster.pxo --in-place --name headline --content "HEADLINE" --font sans-serif --size 140pt --color white --at 50%,20% --anchor t
pixelox comp place poster.pxo product.png --in-place --as product --at 50%,60% --anchor c --size 70%x
pixelox comp template init poster.pxo --in-place --slot text:headline:required --slot image:product:required:fit=contain --slot color:accent:layer=Accent
pixelox comp template slots poster.pxo --json
```

Fill it:

```bash
pixelox comp template apply poster.pxo --set headline="FALL DROP 09" --set-image product=hero.png --set-color accent='#f97316' -o work/fall.pxo
pixelox export work/fall.pxo -o dist/fall-a3.png
pixelox export work/fall.pxo -o dist/fall-og.png --size 1200x630 --region 0,840,3508x1842
```

A missing required slot fails with PX3202 and names the slot. In a pipeline, the whole
build is one reproducible unit (`pixelox docs pipelines`).
