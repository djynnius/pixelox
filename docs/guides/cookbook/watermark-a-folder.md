# Watermark a folder

A pipeline per file, eight files at a time:

```yaml
# watermark.yml
version: 1
steps:
  - id: wm
    run: comp.place
    with: { input: "${item.in}", file: brand/logo.png, as: Watermark, anchor: br, at: "-48,-48", size: "220x" }
  - id: blend
    run: layer.blend
    with: { layer: Watermark, mode: multiply }
  - id: fade
    run: layer.opacity
    with: { layer: Watermark, value: 85 }
  - id: out
    run: export
    with: { output: "${item.out}", quality: 92 }
```

```bash
pixelox pipeline validate watermark.yml --json
pixelox batch run --glob 'shoot/*.jpg' --pipeline watermark.yml --out-dir out/ --jobs 8 --manifest out/manifest.json --progress json
```

Each output gets its own sidecar; `out/manifest.json` lists every item with its outputs or
its error. A failed item makes the exit code non-zero unless `--continue-on-error`.
