# pixelox

**A command-line raster and vector graphics workstation for scripts and AI agents.**

Photoshop's working vocabulary, without the GUI: layers, masks, selections, blend modes,
adjustment layers, layer styles, type with real shaping, vector shapes, filters,
retouching, templates, SVG authoring, tracing, and scripted rendering. One binary. Every
command speaks versioned JSON, and every output carries a provenance sidecar that replays
it bit for bit. (The *ox* is for oxide: *pixel oxide*, PxO₂.)

pixelox is the image sibling of [ferraus](https://djynnius.github.io/ferraus), the
command-line audio workstation, and shares its agent contract.

```bash
pixelox open shot.jpg -o shot.pxo                                  # a layered document
pixelox adjust curves shot.pxo --in-place --points '0,8 96,118 255,246'
pixelox select color shot.pxo --in-place --ref white --at 5,5 --contiguous --tolerance 40
pixelox select invert shot.pxo --in-place                          # the subject, not the backdrop
pixelox export shot.pxo -o cutout.png --apply-mask @sel            # transparent PNG
pixelox verify cutout.png.pixelox.json                             # prove how it was made
```

* **Documents**: open JPEG, PNG, TIFF, WebP, GIF, BMP, layered PSD, and SVG; save `.pxo`
  documents (an inspectable ZIP); export PNG/TIFF at 8 or 16 bits, JPEG, WebP, GIF, BMP,
  flattened or basic-layered PSD, and size/format ladders in one pass.
* **Layers**: raster, text, shape, adjustment, group, and placed (smart-object) layers with
  smart filters; all 28 Photoshop blend modes; Blend If; opacity, masks with density and
  feather, clipping masks, merge, flatten, stamp visible, rasterize.
* **Selections and masks**: rectangle, ellipse, polygon, path, color range and magic wand,
  luminosity, sky (classical, no AI); add/subtract/intersect; feather, expand, contract, smooth; a named mask
  library usable by every region op.
* **Adjustments** (destructive or as adjustment layers): levels, curves, exposure,
  shadows/highlights, clarity, texture, dehaze, tone mapping, hue/saturation, color balance, vibrance, white balance,
  selective color, channel mixer, gradient map, 3D LUTs, black & white, auto tone.
* **Filters and retouching**: Gaussian, box, motion, radial, lens, surface, and median
  blurs; unsharp and smart sharpen; noise, denoise, dust & scratches; emboss, edges, high
  pass, pixelate, vignette, lens correction; heal, clone, dodge, burn, red eye, erase;
  solid, gradient, and pattern fills; brush strokes (`paint stroke`: hardness, flow,
  opacity, image tips, jitter and scatter) onto pixels, layer masks, or selections.
* **Local AI**: `detect subject` (BiRefNet-lite: background removal, cutouts, subject
  masks) and `detect salient` (U²-Netp) run on the CPU through ONNX Runtime. Models are
  never bundled: the first use downloads them with `-y` (224 MB and 5 MB), verified by
  SHA-256; results are reproducible and replay like every other step.
* **Type and vectors**: HarfBuzz-class shaping (RTL, CJK, ligatures, variable fonts), text on
  a path, text to outlines; rectangles, ellipses, polygons, stars, arrows, Bézier paths,
  boolean path ops, stroke outlining.
* **Layer styles**: drop and inner shadows, outer and inner glows, bevel & emboss, satin,
  color/gradient/pattern overlays, stroke.
* **Composition**: place files, align, distribute, guides, grids, canvas presets, and
  templates with typed slots: data in, finished poster out.
* **Vector and generative**: author SVG element by element, rasterize it, trace bitmaps to
  SVG, run sandboxed JavaScript (Canvas2D and a DOM for d3-style data graphics), and render
  3D-lite wireframes and shaded mockups.
* **Automation**: pipeline files, batches over globs, background jobs, replay, verify.

## For AI agents

Every command takes `--json` and prints one versioned envelope; `--dry-run` plans without
writing; nothing is overwritten without `--overwrite`; source images are never edited in
place. `pixelox describe --agent` prints the whole surface in under 4,000 tokens, and
`pixelox schema <command>` prints any command's JSON Schema. See [AGENTS.md](AGENTS.md).

pixelox is also an MCP server:

```bash
claude mcp add pixelox -- pixelox mcp serve --allow ~/work
```

The skill in [skills/pixelox/SKILL.md](skills/pixelox/SKILL.md) teaches an agent when and how
to use it. In Claude Code: `claude plugin marketplace add djynnius/pixelox`, then
`claude plugin install pixelox@pixelox`.

## A few recipes

```bash
# Rescue an underexposed interior, grade it, deliver a 16-bit TIFF
pixelox open interior.jpg -o shot.pxo
pixelox adjust white-balance shot.pxo --in-place --auto
pixelox adjust shadows-highlights shot.pxo --in-place --shadows 45 --highlights 20
pixelox export shot.pxo -o interior-final.tiff --bit-depth 16
pixelox analyze stats interior-final.tiff --json

# A sticker: star burst, type, styles, transparent PNG
pixelox new -o badge.pxo --size 1000x1000 --fill transparent
pixelox shape star badge.pxo --in-place --center 500,500 --points 12 --outer 460 --inner 380 --fill '#ffd400' --stroke '#111111' --stroke-width 10 --name Burst
pixelox text add badge.pxo --in-place --name Title --content "SUMMER DROP" --font sans-serif --size 64pt --color '#111111' --at 50%,50% --anchor c
pixelox fx drop-shadow badge.pxo --in-place --layer Burst --offset 0,10 --blur 24 --color '#00000066'
pixelox export badge.pxo -o badge.png

# A poster from a template
pixelox comp template apply poster.pxo --set headline="FALL DROP 09" --set-image product=hero-cut.png --set-color accent='#0ea5e9' -o work/poster.pxo
pixelox export work/poster.pxo -o dist/poster.png

# Convert a folder, four at a time
pixelox batch run --glob 'shoot/*.jpg' --cmd 'export {in} -o web/{stem}.webp' --jobs 4
```

More in [examples/](examples/) (every script runs in CI) and the guides under
[docs/guides](docs/guides/).

## Install

| Platform | Command |
| --- | --- |
| macOS and Linux | `curl -fsSL https://raw.githubusercontent.com/djynnius/pixelox/main/install.sh \| sh` |
| Windows (PowerShell) | `powershell -ExecutionPolicy ByPass -c "irm https://raw.githubusercontent.com/djynnius/pixelox/main/install.ps1 \| iex"` |
| From source | `cargo install --path crates/pixelox-cli` |

The installers fetch the latest release, verify its SHA-256, put `pixelox` in
`~/.pixelox/bin`, and add that to your PATH, without sudo. Options are environment
variables of the installing shell, so with `curl` they go on the `sh` side of the pipe:

| Variable | Effect |
| --- | --- |
| `PIXELOX_VERSION=0.2.0` | install that release instead of the latest |
| `PIXELOX_INSTALL_DIR=~/bin` | install there instead of `~/.pixelox/bin` |
| `PIXELOX_NO_MODIFY_PATH=1` | leave shell profiles and the user PATH alone |

```bash
curl -fsSL https://raw.githubusercontent.com/djynnius/pixelox/main/install.sh | PIXELOX_NO_MODIFY_PATH=1 sh
```

```powershell
$env:PIXELOX_NO_MODIFY_PATH = 1; irm https://raw.githubusercontent.com/djynnius/pixelox/main/install.ps1 | iex
```

Builds: macOS (Apple silicon and Intel), Linux x86-64 and ARM64 (glibc 2.38+,
with a static musl fallback for older systems), Windows x86-64 and ARM64. Local AI models
run on Apple silicon, glibc Linux, and Windows (which needs the Visual C++ Redistributable);
the Intel macOS and musl builds have no ML runtime.

## Build

```bash
cargo build --release -p pixelox      # target/release/pixelox
cargo install --path crates/pixelox-cli
pixelox doctor
```

Rust 1.97 (pinned in `rust-toolchain.toml`). The build is pure Rust apart from the
vendored QuickJS engine that the JS renderer compiles with `cc`.

## What this build does not include

This is the v0.1 document engine with much of later milestones. These parts of the PRD
are not in this build, and the commands say so with exit code 12 and a remedy:

* **The rest of the AI ops** (`detect objects|faces|text|segment`, `mask refine`,
  `fill content-aware`): their models are not in this release. The commands, flags,
  schemas, and `--dry-run` plans are all present. (`detect subject` and `detect salient`
  run locally; see below.) Builds for Intel macOS and musl Linux have no ML runtime at all.
* **Color-management conversions**: ICC profiles are kept and re-embedded, not converted.
* **Lossy WebP and progressive JPEG** encoding; HEIC and RAW input (PRD stretch items).
* **Vendored d3 and three.js**: `render d3` and `render three` load a local copy given with
  `--lib`.

`pixelox doctor` lists the build's features and known gaps.

## Documentation

* `pixelox docs <command>` prints a command's full reference page offline;
  `pixelox docs contract`, `addressing`, `pipelines`, `sidecars`, `mcp`, and `errors`
  cover the concepts.
* `pixelox docs build-site --out target/docs-site` writes the mdBook site source.
* [PRD.md](PRD.md), [architecture decisions](docs/decisions/), and the
  [contributor guide](docs/dev/AUTHORING.md).

## License

Apache-2.0 OR MIT, at your option. The embedded fallback font (Tuffy) is in the public
domain.
