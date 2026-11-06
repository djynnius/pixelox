# ADR 0012: Local models through ONNX Runtime

Status: accepted (2026-11-05). Supersedes ADR 0006 for `detect subject` and `detect salient`.

## Context
Background removal (Photoshop's Select Subject) was the one step of a long compositing
tutorial that pixelox could not reproduce. ADR 0006 had shipped the build without a model
runtime.

## Decision
* **Runtime.** ONNX Runtime through `ort` `=2.0.0-rc.14` (the version ferraus pins), behind
  the `ml` feature (on by default). `ort` ships prebuilt libraries for glibc Linux,
  Apple-silicon macOS, and Windows; `build.rs` sets `cfg(pixelox_ml)` only there, so Intel
  macOS and musl builds compile without it and keep exit 12 (PX1510).
* **CPU only, reproducible.** Every session resets `ort`'s automatic device policy (it
  silently picks CoreML; compiling BiRefNet for the Neural Engine never finished) and
  registers the CPU provider, with 4 intra-op threads (a constant), one inter-op thread, and
  deterministic compute. Output bytes were identical at 4 and 8 threads and on repeated
  runs; masks are rounded to the document depth before use.
* **Models.** BiRefNet-lite (Swin-T, MIT, 224 MB, 1024 px) from its author's release for
  `detect subject`; U²-Netp (Apache-2.0, 4.6 MB, 320 px) for `detect salient` and
  `detect subject --fast`. Both pinned by URL and SHA-256 in `crates/pixelox-ml/src/registry.rs`.
  Weights are never bundled.
* **Consent.** A missing model fails with PX1502 (exit 6) and the command that downloads it;
  `-y` downloads (resumable, SHA-256 verified before use). A file copied into the models
  directory by hand is adopted when its hash matches (offline installs);
  `PIXELOX_OFFLINE=1` forbids the network. Over MCP the server flag and the call's
  `consent: ["model-download"]` are both required.
* **Edges.** The model's map is brought to full resolution with guided joint upsampling
  (a fast guided filter fitted at the map's resolution, applied with the full-resolution
  image as the guide); on an upsampled test mask it cut the edge error by about 40% compared with
  bilinear.
* **Outputs and replay.** `detect subject` is a document command whose result goes to the
  selection, a named mask, or a layer mask. `--cutout` and `--mask-out` also work without a
  document; their sidecars record the detection (as a selection) followed by
  `export --apply-mask @sel` or `mask export --mask @sel`, so they replay bit for bit. Steps
  re-run by `replay` skip side files (`OpCtx::replaying`).

## Consequences
On an Apple M-series CPU, BiRefNet-lite takes about 8 s of inference (10 s end to end) at
1024 px, which misses the PRD's 2.5 s budget; U²-Netp takes under 1 s. GPU providers
(CoreML, CUDA, DirectML) would be faster but are not bit-stable across machines; they are
left for an opt-in flag. The other detect commands, `mask refine`, `fill content-aware`, and
`models add` stay planned (exit 12, PX1520).
