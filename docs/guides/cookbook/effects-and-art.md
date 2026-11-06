# Lens flares, fire, and art filters

## Lens flare

`render lens-flare` adds a camera flare as its own screen-mode layer, so it stays movable:

```bash
pixelox render lens-flare photo.pxo --in-place --at 70%,18% --lens zoom
pixelox render lens-flare photo.pxo --in-place --at 1400,300 --lens movie --brightness 140
```

Lenses: `zoom` (50-300 mm), `35mm`, `105mm`, and `movie` (anamorphic streak).

## Fire and smoke

Fire and smoke are live layer styles: they rise from a layer's outline, follow the layer when
it moves, and come off with `fx rm`.

```bash
pixelox fx fire title.pxo --in-place --layer Title --height 60%
pixelox fx smoke portrait.pxo --in-place --layer Cutout --height 80% --density 40
pixelox fx rm title.pxo --in-place --layer Title --kind fire
```

`--variant` picks another random pattern; `--front` lets some flames lick over the edges.

## Art filters

| Look | Command |
| --- | --- |
| Pencil sketch | `pixelox filter pencil photo.jpg -o sketch.png` |
| Colored pencil | `pixelox filter pencil photo.jpg -o sketch.png --color` |
| Charcoal | `pixelox filter charcoal photo.jpg -o charcoal.png` |
| Ink / graphic pen | `pixelox filter ink photo.jpg -o ink.png --shading hatch` |
| Oil painting | `pixelox filter oil-paint photo.jpg -o oil.png` |
| Watercolor | `pixelox filter watercolor photo.jpg -o watercolor.png` |
| Cartoon | `pixelox filter cartoon photo.jpg -o cartoon.png` |
| Paper cutout | `pixelox filter cutout photo.jpg -o cutout.png --levels 6` |
| Halftone print | `pixelox filter halftone photo.jpg -o print.png --mode cmyk` |

All of them take `--mask` (apply to part of the picture), scale with the image size, and
work as smart filters on placed layers.
