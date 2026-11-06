# pixelox examples

Runnable examples for scripts and agents. They build their own test material with pixelox
(`make-samples.sh`: a landscape photo, a product on white, an SVG logo, a scanned mark,
animation frames, a CSV), so nothing binary is checked in. CI runs all of them:

```bash
cargo build -p pixelox
PIXELOX=target/debug/pixelox examples/run-all.sh
```

Every shell script takes `[SAMPLES_DIR] [OUT_DIR]`, keeps each call's JSON envelope under
`OUT_DIR/json/`, and checks what it expects (exit codes, sizes, verification).

| Example | Shows |
| --- | --- |
| [shell/photo-edit.sh](shell/photo-edit.sh) | E4: open, auto white balance, shadows/highlights, curves, an adjustment layer, sharpening, 16-bit TIFF and web JPEG, stats, verify |
| [shell/cutout.sh](shell/cutout.sh) | E1 without downloading a model: `detect subject` planned, then its consent gate (exit 6), a magic-wand cutout, a transparent PNG placed on a new background with a shadow |
| [shell/sticker.sh](shell/sticker.sh) | E11: star burst, text on a circular path, stroke and drop shadow, transparent PNG |
| [shell/poster-template.sh](shell/poster-template.sh) | E5: build a template with typed slots, PX3202 for a missing slot, fill it, print and OG exports |
| [shell/vector-logo.sh](shell/vector-logo.sh) | E12: trace a scan, recolor with a selector, optimize, rasterize, measure fidelity with `analyze diff` |
| [shell/inspect-and-verify.sh](shell/inspect-and-verify.sh) | the contract: exit codes 4/3/2, PX1108, dry runs, history undo, bit-identical replay, tamper detection (exit 9) |
| [shell/frames.sh](shell/frames.sh) | animated GIF assemble, frame open, extract |
| [shell/render.sh](shell/render.sh) | E9/E10: a data chart in the JS sandbox's DOM, `render d3` without `--lib` (exit 12), hidden-line and shaded 3D |
| [pipelines/poster.yml](pipelines/poster.yml) | §9.2: a template built, filled, and exported in one process |
| [pipelines/social-cards.yml](pipelines/social-cards.yml) | E6: `foreach` over CSV rows |
| [pipelines/watermark.yml](pipelines/watermark.yml) | E7: per-file pipeline for `batch run` |
| [mcp/client.py](mcp/client.py) | an MCP client over stdio: tools, a dry run, a run, a preview, verify, the sandbox |
| [agents/parse_envelope.py](agents/parse_envelope.py) | envelopes, exit codes, warnings, and the ML gate from Python |
