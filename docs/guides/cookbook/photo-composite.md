# Blend a photo composite

The workflow of a classic Photoshop compositing tutorial (sky, layered mountains, a hero
subject, painted light and shadow, a final Camera Raw grade), step by step in pixelox. Every
step stays editable in one `.pxo` document, and the sidecar of the final export rebuilds it
bit for bit.

## 1. Canvas and sky

```bash
pixelox new -o scene.pxo --size 4000x2250 --fill '#d9e4ea'
pixelox comp place scene.pxo --in-place sky.jpg --as Sky --fit cover
# give the sky depth: pull the top corners outward
pixelox transform perspective scene.pxo --in-place --layer Sky --quad '-600,0 4600,0 4000,2250 0,2250'
pixelox adjust curves scene.pxo --in-place --points '0,30 128,150 255,255' --as-layer --name SkyCurves
pixelox layer clip scene.pxo --in-place --layer SkyCurves
pixelox adjust hsl scene.pxo --in-place --hue -8 --saturation -25 --as-layer --name SkyHue
pixelox layer clip scene.pxo --in-place --layer SkyHue
# fade the sky's lower edge with a big soft brush at low flow
pixelox mask add scene.pxo --in-place --layer Sky
pixelox paint stroke scene.pxo --in-place --layer Sky --target mask --points '0,78% 50%,80% 100%,78%' --size 700 --flow 12 --color black
```

## 2. Mountains without their sky

`select sky` is the classical equivalent of Photoshop's Select › Sky. Selecting the sky and
inverting it masks the mountains.

```bash
pixelox comp place scene.pxo --in-place far-mountains.jpg --as FarMountains --size 100%x --anchor bl --at 0,-250
pixelox select sky scene.pxo --in-place --layer FarMountains
pixelox select invert scene.pxo --in-place
pixelox mask add scene.pxo --in-place --layer FarMountains --from @sel
pixelox select none scene.pxo --in-place
# a one-pixel bright fringe on the ridge: choke the mask (Filter › Minimum)
pixelox mask contract scene.pxo --in-place --layer FarMountains --by 1
```

`mask contract --layer` edits the layer's mask in place. `@mask:FarMountains` reads it
anywhere a mask is taken; to edit a mask in steps, load it as the selection, change it, and
write it back:

```bash
pixelox select load scene.pxo @mask:FarMountains --in-place
pixelox select ellipse scene.pxo --in-place --rect 2600,900,900x500 --feather 120 --mode subtract
pixelox mask add scene.pxo --in-place --layer FarMountains --from @sel
```

## 3. Atmospheric perspective

Far things are less contrasty, less saturated, and take the sky's color. Filters and
adjustments run on a placed layer are kept as smart filters (still editable):

```bash
pixelox adjust curves scene.pxo --in-place --layer FarMountains --points '0,60 255,245'
pixelox adjust dehaze scene.pxo --in-place --layer FarMountains --amount=-40
pixelox adjust color-balance scene.pxo --in-place --midtones=-8,0,12 --shadows=-6,0,10 --as-layer --name Atmosphere
pixelox layer clip scene.pxo --in-place --layer Atmosphere
pixelox layer list scene.pxo --json
pixelox layer smart-filter scene.pxo --in-place --layer FarMountains --action disable --index 2
```

## 4. Fog between layers

```bash
pixelox comp place scene.pxo --in-place fog.jpg --as Fog --size 100%x --anchor bl --at 0,-500
pixelox layer blend scene.pxo --in-place --layer Fog --mode screen
pixelox layer blend-if scene.pxo --in-place --layer Fog --this 20,70,255,255
pixelox layer opacity scene.pxo --in-place --layer Fog --value 60
```

`--this 20,70,255,255` drops the fog image's dark parts with a soft ramp, the Blend If trick
for using a photo of fog as a texture.

## 5. The hero subject, its shadows, and the light

`detect subject` is Select Subject: a local model (BiRefNet-lite; its first use downloads
224 MB, hence `-y`) masks the subject of a layer. Without the model, cut a subject on a
plain backdrop with `select color --contiguous`, anything else with a traced path.

```bash
pixelox comp place scene.pxo --in-place camel.png --as Camel --size 420x --at 2700,1500
pixelox detect subject scene.pxo --in-place --layer Camel --layer-mask -y
# cast shadow from the subject's own outline: black copy, skewed, blurred, multiplied
pixelox layer dup scene.pxo --in-place --layer Camel --name CamelShadow
pixelox layer rasterize scene.pxo --in-place --layer CamelShadow
pixelox fill solid scene.pxo --in-place --layer CamelShadow --color black --preserve-alpha
pixelox transform skew scene.pxo --in-place --layer CamelShadow --x -55
pixelox transform scale scene.pxo --in-place --layer CamelShadow --size 100%x40% --anchor b
pixelox filter gaussian scene.pxo --in-place --layer CamelShadow --sigma 3
pixelox layer blend scene.pxo --in-place --layer CamelShadow --mode multiply
pixelox layer reorder scene.pxo --in-place --layer CamelShadow --below Camel
# painted shadows and sunlight on their own layers
pixelox layer add scene.pxo --in-place --name Shadows
pixelox layer blend scene.pxo --in-place --layer Shadows --mode multiply
pixelox paint stroke scene.pxo --in-place --layer Shadows --points '1800,1900 2600,1980 3400,1900' --size 380 --flow 10 --color '#3b2a22'
pixelox layer add scene.pxo --in-place --name Beam
pixelox layer blend scene.pxo --in-place --layer Beam --mode screen
pixelox paint stroke scene.pxo --in-place --layer Beam --points '18%,0 46%,100%' --size 30% --flow 6 --opacity 70 --taper 60%,0 --color '#fff0d2'
# shading through a luminosity curves layer, painted only where shade falls
pixelox adjust curves scene.pxo --in-place --points '0,0 128,80 255,215' --as-layer --name Shade
pixelox layer blend scene.pxo --in-place --layer Shade --mode luminosity
pixelox mask add scene.pxo --in-place --layer Shade --hide-all
pixelox paint stroke scene.pxo --in-place --layer Shade --target mask --points '2300,1300 2500,1600' --size 260 --flow 20 --color white
```

Textured brushes take a tip image: `--tip chalk.png --scatter 60 --size-jitter 40
--angle-jitter 360 --count 3` stands in for an oil-pastel brush.

## 6. Final grade (Camera Raw)

Stamp everything visible onto a new layer, then grade it:

```bash
pixelox layer stamp scene.pxo --in-place --name Grade
pixelox adjust shadows-highlights scene.pxo --in-place --layer Grade --highlights 30 --shadows 10
pixelox adjust clarity scene.pxo --in-place --layer Grade --amount 25
pixelox adjust texture scene.pxo --in-place --layer Grade --amount 15
pixelox adjust dehaze scene.pxo --in-place --layer Grade --amount 10
pixelox adjust hsl scene.pxo --in-place --layer Grade --range reds --hue 8
pixelox adjust color-balance scene.pxo --in-place --layer Grade --shadows=12,0,-8 --highlights=-6,0,10
pixelox filter vignette scene.pxo --in-place --layer Grade --amount -25 --roundness 20
pixelox filter add-noise scene.pxo --in-place --layer Grade --amount 2 --monochrome
pixelox export scene.pxo -o composite.jpg --quality 92
pixelox verify composite.jpg.pixelox.json --json
```
