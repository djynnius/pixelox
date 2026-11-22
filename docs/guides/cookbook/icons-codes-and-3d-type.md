# Icons, QR codes, barcodes, and 3D type

Flyers and posters carry small working parts: icons in the info bar, a QR code to the tickets,
a barcode on a coupon, and display type with depth. Each of these is one command and stays a
live vector shape or a live layer style, so it moves, recolors, and scales with the layout.

## Icons in an info bar

`shape icon` draws any of the ~2,100 bundled [Lucide](https://lucide.dev) icons (ISC license, no
brand logos) as a round-capped outline. Find names first; plain words work too (`cart`,
`location`, `email`, `web`, `person`):

```bash
pixelox shape icons --search calendar --json
pixelox shape icons --search 'music note'
```

Place them at the start of each column, the same size as the text's cap height or a little
larger. Sizes that are multiples of 24 (24, 48, 72, 96) are pixel-sharp:

```bash
pixelox new -o flyer.pxo --size 1080x1350 --fill '#5a0c0a'
pixelox shape rect flyer.pxo --in-place --name Bar --rect 8%,81%,84%x11% --fill '#1c0505'
pixelox shape icon flyer.pxo calendar --in-place --size 48 --at 12%,86.5% --anchor l --color white --name IconDate
pixelox shape icon flyer.pxo clock --in-place --size 48 --at 43%,86.5% --anchor l --color white --name IconTime
pixelox shape icon flyer.pxo map-pin --in-place --size 48 --at 62%,86.5% --anchor l --color white --name IconPlace
```

`--stroke-width` sets the line weight (default size / 12, Lucide's own); `--fill` also fills the
icon's closed outlines, for solid hearts, stars, and pins:

```bash
pixelox shape icon flyer.pxo heart --in-place --size 96 --at 90%,10% --anchor c --color '#e11d48' --fill '#e11d48'
pixelox shape set flyer.pxo --in-place --layer IconDate --stroke '#ffd23c'
```

## A QR code that scans

`shape qr` encodes a URL or any text. `--size` includes the quiet zone (`--quiet-zone`, 4
modules by default); on a dark or busy background give it a plate, which makes the result a
group you move as one:

```bash
pixelox shape qr flyer.pxo --in-place --data 'https://example.com/tickets' --size 220 --background-color white --radius 16 --anchor br --at -60,-60 --json
```

`--style rounded` and `--style dots` restyle the modules; the three finder squares stay square
so every scanner finds the code. Each code is decoded back before it is added, and warning
PXW6010 flags light modules on a dark ground or low contrast.

For a logo in the middle, ask for a gap; it is allowed only when the error correction can
absorb it (level h by default), and the result's `logo_box` says where the logo goes:

```bash
pixelox shape qr card.pxo --in-place --data 'https://example.com/app' --size 300 --logo-gap 22 --background-color white --at 50%,75% --anchor c --json
pixelox comp place card.pxo logo.png --in-place --as Logo --at 50%,75% --anchor c --size 60x60
```

A gap that is too large fails with PX6034 and the largest gap that would pass.

## Barcodes

`shape barcode` draws EAN-13, EAN-8, UPC-A, Code 128, and Code 39 with their quiet zones and
human-readable line. Give EAN/UPC numbers without the check digit and pixelox adds it; a wrong
check digit fails with PX6032 and the right one:

```bash
pixelox shape barcode coupon.pxo --in-place --type ean13 --data 400638133393 --width 360 --anchor br --at -40,-40
pixelox shape barcode ticket.pxo --in-place --type code128 --data 'TKT-2027-0042' --width 50% --height 90 --at 50%,90% --anchor c --background-color white
```

## Long shadows and block lettering

`fx long-shadow` drags a flat shadow from the layer's shape until it leaves the canvas;
`fx extrude` gives it solid sides. Both follow the layer when it moves or its text changes:

```bash
pixelox new -o retro.pxo --size 1080x1350 --fill '#19b5a5'
pixelox text add retro.pxo --in-place --name Title --content "RETRO" --font "Arial Black" --size 16% --color '#fff4d6' --at 50%,30% --anchor c
pixelox fx long-shadow retro.pxo --in-place --layer Title --color '#0d6b62' --blend normal --opacity 100
pixelox text add retro.pxo --in-place --name Block --content "BLOCK" --font "Arial Black" --size 16% --color '#ffd23c' --at 50%,72% --anchor c
pixelox fx extrude retro.pxo --in-place --layer Block --depth 40 --color '#b3261e'
pixelox fx stroke retro.pxo --in-place --layer Block --width 4 --color '#3b0d0a'
```

`--color auto` (the default) colors the sides from the face, darkened by `--darken`, so a
gradient title extrudes in its own colors; `--shade` darkens the sides toward the back, and
`--outline` strokes the whole block. A long shadow on an extruded layer is cast by the block:

```bash
pixelox fx overlay retro.pxo --in-place --layer Block --stops '#ffcf56@0,#e4572e@1' --angle 90
pixelox fx extrude retro.pxo --in-place --layer Block --depth 8% --shade 40 --outline 4 --outline-color '#0b1d3a'
pixelox fx long-shadow retro.pxo --in-place --layer Block --length 30% --fade 100 --opacity 40
```

## A price burst with depth

```bash
pixelox shape star promo.pxo --in-place --name Burst --center 62%,55% --points 22 --outer 190 --inner 150 --fill '#d83a0a'
pixelox fx extrude promo.pxo --in-place --layer Burst --depth 22 --angle 300
pixelox text add promo.pxo --in-place --name Price --content "\$3.99" --font "Arial Black" --size 7% --color '#ffffff' --at 62%,55% --anchor c
pixelox fx long-shadow promo.pxo --in-place --layer Price --length 30 --color '#7a1d05' --blend normal --opacity 100
```

## A floor reflection

Trim the cutout first, so the reflection touches it, then reflect the placed layer:

```bash
pixelox transform trim product.png -o tight.png
pixelox comp place ad.pxo tight.png --in-place --as Product --size x620 --at 50%,70% --anchor b
pixelox fx reflection ad.pxo --in-place --layer Product --length 35% --opacity 40 --blur 10
```

`--gap` lifts the object off the floor, `--fade` sets how much of the length the fade takes,
and `--distance` how much the blur grows away from the floor line (100: sharp where the object
touches it, as on real glossy surfaces).

## Gradient fills on stroked shapes

`fx overlay` on a shape or text layer with both a fill and a stroke recolors the fill and keeps
the stroke on top, so a gradient pill keeps its outline:

```bash
pixelox shape rect card.pxo --in-place --name Pill --rect 10%,80%,40%x7% --radius 48 --fill white --stroke '#111827' --stroke-width 6
pixelox fx overlay card.pxo --in-place --layer Pill --stops '#ffcf56@0,#e4572e@1' --angle 0
```

Add `--cover-stroke` to paint over the stroke as well.
