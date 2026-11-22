# ADR 0016: Procedural textures, grain, distortion, and liquid are native

Status: accepted (2026-11-22)

## Context
Agents building 126 posters wrote `render js` scripts for clouds, smoke, grunge, grain,
splashes, condensation, and ice, then placed, blurred, and blended the PNGs in five to
twelve more steps (`skills/pixelox-design`, T-smoke, T-paper-grain, T-water-splash,
T-condensation, T-glitch, T-warp-type). The scripts used value noise on a 256 lattice
upscaled from quarter resolution, ellipses for liquid, and a gray layer with Gaussian noise
in overlay mode for grain. 53 posters added grain and about 25 used clouds or smoke.

## Decision
* **Shared math lives in core.** `pixelox_core::noise` (Perlin, simplex, and value noise,
  fBm, turbulence, ridged, Worley F1 and F2 − F1, periodic lattices), `pixelox_core::grain`
  (film grain), and `pixelox_core::warp_preset` (Photoshop's 15 warp styles) are pure
  functions of a seed and coordinates, so they render identically on any thread count and
  can be used by every crate: fills and filters, the fx renderer, the adjustment hook, and
  live text (WS2 implements warp presets for text; its map should be this one).
* **Normalized noise.** A fractal sum is divided by the basis's measured standard deviation
  (sampled once at a fixed set of points) times the root of the summed squared octave
  amplitudes, so every kind, octave count, and persistence has mean 0.5 and spread 0.18:
  `--contrast` and `--bias` mean the same thing everywhere. `--tile` makes lattices periodic
  over the canvas (rounding the scale to whole cells, the lacunarity to an integer, and
  falling back from simplex to Perlin, with PXW3010).
* **Smoke is advected, not thresholded.** Thresholded or warped fBm reads as marble or
  satin. `--kind smoke` averages a fine fiber texture along the streamlines of a
  divergence-free flow (curl of a stream function plus a drift along `--angle`), i.e. line
  integral convolution, over a soft haze and a slow density field that leaves gaps. Flow and
  fibers are precomputed on grids; large canvases run at half resolution.
* **Grain is band-limited and anchored.** White noise on a lattice anchored to absolute
  coordinates is blurred to the grain size and rescaled by the exact discrete kernel norm
  (so strength does not change with size), mixed with a finer field for roughness and with
  per-channel fields for color, and added in sRGB-encoded values weighted by a midtone
  response. `filter grain` anchors to the canvas; `fx grain` anchors to the layer's content
  (it moves with the layer) and runs after every other effect, as a post step of the fx hook
  so the style stack itself is untouched. `filter grain --as-layer` stores an adjustment
  layer with op `grain`, which the adjuster delegates to core: one live layer grains the
  whole design below it.
* **Distortions are inverse maps.** `filter distort`, `filter displace`, and `filter glitch`
  map each output pixel to a source position and sample premultiplied. Their centers and
  boxes default to the layer's visible bounds, which on a placed layer's smart-filter canvas
  is the placed content, so they register as smart filters the standard way (ADR 0011). A
  map file or map layer is not re-read at render time: on a placed, text, or shape layer only
  the noise map is recorded (PX3342 for a map layer). Waves, ripples, displacement, and
  glitch move content past where it was, so they grow a small layer by their reach first
  (`apply_to_raster_grow`, ADR 0014); the other distortions stay inside the content.
* **Warp presets reuse the mesh warp's inversion.** `transform warp --preset` samples the
  preset's forward map on an 8 px mesh and inverts cells bilinearly (the same code as the
  points mode). On a placed, text, or shape layer the warp is recorded as a smart filter whose op is
  `transform.warp` with an advanced `--keep-bounds` flag, so the re-run keeps the canvas-sized
  image the smart-filter hook requires; it stays live instead of rasterizing.
* **Liquid is a lit metaball field.** Splashes and beads are tapered capsules and spheres
  whose fields add up (so neighbors merge with liquid bridges) or, within a chain, combine by
  maximum (smooth joints). A height map grown from the outline and scaled by the local strand
  radius gives normals; the shader is a clear or opaque body, a refracting rim, a caustic,
  Fresnel reflection, and specular highlights. Condensation beads refract the product layer
  (rendered once, so re-run after changing the product) and are clipped to it.
* **Ice cubes are not native yet.** A convincing cube needs a refracting, internally
  cracked 3-D volume; the metaball shader does not produce one, and a half-convincing cube
  would be worse than the skill's stylized workaround. Left for a ray-marched renderer.

* **Placement follows ADR 0014.** New layers (`fill noise --as-layer`, `filter grain
  --as-layer`, `render splash`, `render droplets`) go on top of the stack, directly above
  `--layer` when one is named (the layer they belong to), or where `--above`, `--below`, or
  `--into` say. `render droplets --layer P` clips to P only in its default place, directly
  above P.

## Consequences
New commands: `fill noise`, `filter grain`, `filter distort`, `filter displace`, `filter
glitch`, `fx grain`, `render splash`, `render droplets`, and `transform warp --preset`
(about 130 tokens of the agent briefing). `render lens-flare --color` now recolors every
element (a fix). `fx rm --kind` accepts `grain`. The noise normalization constants are
measured at startup from a fixed sample, so they are deterministic but not hand-tunable;
changing the basis functions changes every noise render (a replay-visible change).
