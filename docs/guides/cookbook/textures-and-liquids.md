# Clouds, grain, distortion, and liquid splashes

Procedural textures, film grain, distortions, and liquid are native commands: no scripts,
no intermediate PNGs, and every result is seeded (`--seed`), recorded in the sidecar, and
rebuilt bit for bit by `pixelox replay`.

## Clouds, smoke, and noise textures

`fill noise` paints seeded noise through color stops, onto a layer like `fill gradient`, or
onto a new canvas-sized layer with `--as-layer` (which then takes `--blend` and `--opacity`
as the layer's mode and opacity).

```bash
# Photoshop's Render Clouds, then Difference Clouds on top
pixelox fill noise sky.pxo --in-place --layer Sky --stops '#1b2a4a,#f2f4f8'
pixelox fill noise sky.pxo --in-place --layer Sky --blend difference
# rising smoke on a dark ground: one layer, already in screen mode
pixelox fill noise night.pxo --in-place --as-layer --name Smoke --kind smoke --stops 'transparent,#c9d2e0' --blend screen --opacity 85
# mottled paper multiplied over the design
pixelox fill noise poster.pxo --in-place --as-layer --name Mottle --scale 25% --stops '#8a7a62,#f6eedc' --blend multiply --opacity 60
# a seamless marble tile
pixelox fill noise tile.png -o marble.png --kind ridged --warp 120 --scale 60% --stops '#2b2b30,#f4f1ea' --tile
```

| `--kind` | Looks like |
| --- | --- |
| `clouds` | soft fractal clouds, fog, mottling (Render Clouds) |
| `smoke` | thin curling wisps rising along `--angle` over a soft haze |
| `turbulence` | billows with dark creases: thick smoke, fire, puffs |
| `ridged` | sharp bright veins: marble (with `--warp`), lightning, mountains |
| `cells` | round Worley cells: bubbles, scales, stone |
| `cell-edges` | the network between cells: cracks, crackle, dried mud |

`--scale` is the size of the biggest features (`%` of the shorter canvas side), `--octaves`
adds detail, `--warp` swirls, `--stretch` with `--angle` elongates (90 = vertical),
`--contrast` hardens the edges, and `--bias` grows (+) or shrinks (−) the high-color areas.
`transparent` as a stop fades without a dark fringe. Two layers filled with the same `--seed`
line up, because the noise is anchored to the canvas.

## Film grain

`filter grain` is film grain, not digital noise: soft clumps about `--size` px across (auto
scales with the canvas), strongest in the midtones, the same visible strength at every size.

```bash
# the whole poster, live: one grain adjustment layer above everything it should grain
pixelox filter grain poster.pxo --in-place --as-layer --amount 25
# a coarser, slightly colored grain on one photo layer
pixelox filter grain poster.pxo --in-place --layer Photo --amount 45 --size 3 --color 30
# grain on live type, which stays editable
pixelox fx grain poster.pxo --in-place --layer Title --amount 40
```

Put small text above the grain layer to keep it crisp
(`pixelox layer reorder poster.pxo --in-place --layer Grain --below Body`).

## Distortions

```bash
pixelox filter distort photo.jpg -o twirl.jpg --kind twirl --angle 180
pixelox filter distort flag.pxo --in-place --layer Flag --kind wave --amplitude 14 --wavelength 160
pixelox filter distort ring.png -o polar.png --kind polar
pixelox filter displace photo.pxo --in-place --layer Photo --amount 1.5% --scale 4%
pixelox filter displace shirt.pxo --in-place --layer Logo --map-layer Folds --mode slope --amount 18
pixelox filter glitch photo.jpg -o glitch.jpg --split 2% --slices 8
```

`filter displace` without a map generates a noise map (heat haze, water); `--mode slope`
moves pixels along a map's brightness slopes (glass, fabric). On a placed layer every one of
these filters (with the noise map) is recorded as a smart filter.

Warp presets bend a layer's visible bounds like Photoshop's Warp: arc, arc-lower, arc-upper,
arch, bulge, shell-lower, shell-upper, flag, wave, fish, rise, fisheye, inflate, squeeze, and
twist, with `--bend`, `--h-distort`, `--v-distort`, and `--orient`. On a placed layer the warp
stays live.

```bash
pixelox transform warp badge.pxo --in-place --layer Logo --preset arc --bend 40
pixelox transform warp ad.pxo --in-place --layer Banner --preset flag --bend 30 --h-distort 20
```

## Liquid: splashes and condensation

`render splash` draws liquid lit like glass (a clear body with a dark refracting rim, a
caustic, and highlights) on its own layer. A crown rises from a pool; a sheet is thrown from a
point. To stand a product in a crown, draw the back half below it and the front half above
it with the same seed and geometry:

```bash
pixelox render splash ad.pxo --in-place --part back --at 50%,90% --size 95% --seed 3 --name SplashBack
pixelox comp place ad.pxo can.png --in-place --as Can --size x900 --at 50%,92% --anchor b
pixelox render splash ad.pxo --in-place --part front --at 50%,90% --size 95% --seed 3 --name SplashFront
# juice thrown up behind a product; milk; a recolored soda
pixelox render splash ad.pxo --in-place --kind sheet --liquid juice --at 50%,80% --size 85% --spread 150
pixelox render splash ad.pxo --in-place --kind sheet --liquid milk --at 30%,80% --angle 60 --size 90%
pixelox render splash ad.pxo --in-place --liquid water --color '#ff5a1f' --at 50%,85%
```

`render droplets` beads a product with condensation on a layer clipped to it. Each bead
shows the product behind it upside down, as real water does:

```bash
pixelox render droplets ad.pxo --in-place --layer Can
pixelox render droplets ad.pxo --in-place --layer Bottle --density 70 --trickles 6 --mist 60
```

## Lens flare color

`render lens-flare --color` recolors the whole flare (glow, streak, halo, and ghosts; the
core stays white-hot):

```bash
pixelox render lens-flare night.pxo --in-place --at 50%,18% --lens movie --color '#ff9a3c'
```
