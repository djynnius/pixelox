# Sidecars, replay, and verify

Every written output `X` gets `X.pixelox.json`:

```json
{"pixelox_sidecar": 1,
 "tool": {"name": "pixelox", "version": "0.1.0", "build": "git:a1b2c3d", "target": "aarch64-apple-darwin", "profile": "release"},
 "created_utc": "2026-11-04T14:03:22Z", "base_dir": "/work",
 "command": {"id": "export", "argv": ["export", "walk.pxo", "-o", "walk-blur.png"], "params": {}, "seed": 0},
 "recipe": [
   {"op": "open", "params": {}, "inputs": [{"role": "source", "path": "walk.jpg", "sha256": "9f2a…", "bytes": 8812344}]},
   {"op": "mask.feather", "params": {"mask": "dog", "radius": "12"}},
   {"op": "filter.gaussian", "params": {"sigma": "8", "mask": "dog", "invert_mask": "true"}},
   {"op": "export", "params": {}}],
 "inputs": [{"role": "source", "path": "walk.jpg", "sha256": "9f2a…", "bytes": 8812344}],
 "models": [], "fonts": [],
 "output": {"path": "walk-blur.png", "sha256": "41c7…", "bytes": 19233807, "size": [6000, 4000], "format": "png", "bit_depth": 8},
 "env": {"threads": 8, "mem_budget_mb": 4096, "locale": "C"},
 "determinism": {"bit_exact_scope": "same-version-same-target", "perceptual": {"ssim_min": 0.999, "de00_p99_max": 1.0}}}
```

The recipe is the whole chain back to the source files, so replay needs no intermediate
`.pxo`. Values detected at run time (an auto angle, an auto white balance) are recorded as
plain values, so a replay does not detect again.

* `pixelox replay X.pixelox.json -o Y [--input role=path] [--allow-input-drift] [--strict]`
  re-runs the recipe. Recorded input hashes are checked first (PX1601 on drift). `--strict`
  requires a bit-identical result (PX1602 otherwise).
* `pixelox verify X.pixelox.json [--full]` checks that the output and its inputs still match
  their hashes; `--full` replays into a temporary directory and compares.
* `pixelox pipeline new -o p.yml --from-sidecar X.pixelox.json` turns a recipe into a
  reusable pipeline.
* `pixelox history list|undo|checkpoint doc.pxo` works on a document's recipe.
* `--redact-paths` replaces directories with hashes for sidecars you share; `--no-sidecar`
  skips them.
