# Pixelox — Product Requirements Document

> **PxO₂** — *pixel oxide*. A command-line raster and vector graphics workstation for scripts and AI agents.

| Field | Value |
|---|---|
| **Product** | Pixelox |
| **Codename** | PxO₂ ("pixel oxide") |
| **Version (target)** | 0.1.0 |
| **Status** | Draft |
| **Date** | 2026-11-04 |
| **Authors** | Ikakke Ikpe (product owner) with Claude Fable 5 |
| **License (planned)** | Apache-2.0 OR MIT |
| **Sibling project** | [ferraus](https://github.com/djynnius/ferraus) — command-line audio workstation for scripts and AI agents |
| **Document format** | Keep-a-Changelog-era professional PRD; change history in §15 |

---

## Table of contents

1. [Executive summary & vision](#1-executive-summary--vision)
2. [Background & motivation](#2-background--motivation)
3. [Personas & user stories](#3-personas--user-stories)
4. [Goals & non-goals](#4-goals--non-goals)
5. [Design principles](#5-design-principles)
6. [The agent contract](#6-the-agent-contract)
7. [Core architecture](#7-core-architecture)
8. [Command surface](#8-command-surface)
9. [Pipeline files](#9-pipeline-files)
10. [MCP server & skill](#10-mcp-server--skill)
11. [Non-functional requirements](#11-non-functional-requirements)
12. [Milestones & roadmap](#12-milestones--roadmap)
13. [Open questions & risks](#13-open-questions--risks)
14. [Appendices](#14-appendices)
15. [Document history](#15-document-history)

---

## 1. Executive summary & vision

**Pixelox is Photoshop for agentic workflows.** It is a single cross-platform binary that gives shell scripts and AI agents the full working vocabulary of a professional image editor — layers, masks, selections, blend modes, layer styles, adjustment layers, type, vector shapes, filters, retouching, AI-assisted segmentation and inpainting, SVG authoring, and data-driven/3D-lite rendering — with none of the GUI. Every operation is a command; every command speaks versioned JSON; every output carries provenance and can be replayed bit-for-bit.

Pixelox is the photo-and-graphics sibling of **ferraus**, the command-line audio workstation. The two share a design contract: agents and scripts are first-class users, results are reproducible and traceable, output is machine-readable and versioned, destructive actions are opt-in, plans precede execution, documentation works offline, and every example in the docs runs in CI. Where ferraus gives an agent `analyze tempo` and `midi quantize`, Pixelox gives it `detect subject` and `mask feather`. An agent that has learned one tool already knows how to hold the other.

The elevator demo:

```bash
# Remove the background from a product shot and export a transparent PNG — one verified step.
pixelox detect subject shot.jpg --matte --cutout shot-cutout.png --json
```

One command, one JSON envelope on stdout, one `shot-cutout.png.pixelox.json` provenance sidecar that lets anyone — human, CI job, or another agent — verify (`pixelox verify`) or reproduce (`pixelox replay`) the result.

### 1.1 Why now

AI agents can already write copy, code, and plans, but when asked to *"put the headline on the hero image, knock out the background, and export a story-format variant"*, they are reduced to stringing together ImageMagick incantations that have no concept of a layer, a named mask, or a reproducible recipe. GUI editors are unusable without a human at the pointer; existing CLI tools are pixel pushers, not document editors. Pixelox fills the gap: a real layered, non-destructive document model behind a contract designed for machine callers.

### 1.2 What Pixelox is

- A **raster photo editor**: tonal and color adjustments, filters, retouching, healing, content-aware fill, full layer stack with Photoshop's blend-mode set, layer masks, clipping masks, layer styles (fx), and type with proper shaping.
- A **compositor and layout tool**: place, align, distribute; guides and grids; canvas presets; reusable templates that turn data into fliers, posters, and social cards.
- A **vector author**: programmatic SVG scene-graph authoring, Bézier paths and booleans, bitmap tracing, and an embedded JS sandbox that runs d3-style data graphics and three.js-style 3D-lite scenes to raster or SVG.
- An **agent platform**: JSON envelopes, JSON Schemas per command, a sub-4k-token `describe --agent` surface, dry-run planning, provenance sidecars, `replay`/`verify`, a built-in MCP server, a Claude Code skill, and an `AGENTS.md` contract.

### 1.3 What Pixelox is not

No GUI, no CAD precision, no video, no full-fidelity PSD writing, no bundled generative diffusion models. §4.2 enumerates non-goals precisely.

### 1.4 Success, in one sentence per audience

- **An AI agent** can take "make me a 1080×1920 promo story from these three photos and this headline" from zero to finished PNG without a human touching a pointer, and prove how it did it.
- **A pipeline engineer** can run the same recipe over 10,000 images tonight and get bit-identical results next quarter.
- **A designer-developer** can encode a brand template once and let scripts fill it forever.

---

## 2. Background & motivation

### 2.1 Why agents need this

1. **GUI tools are invisible to agents.** Photoshop, Affinity, GIMP, Figma: their power is behind pointers, panels, and modal dialogs. Screen-driving them is slow, brittle, non-deterministic, and unauditable. Scripting bridges (ExtendScript/UXP, GIMP Script-Fu) require a running host app, have weak error contracts, and were designed for humans writing plugins, not agents making thousands of small verified calls.
2. **Existing CLI tools stop at pixels.** ImageMagick can blur and composite, but it has no persistent layer stack, no masks-as-named-objects, no adjustment layers, no layer styles, no text shaping worth shipping, no document format, and its output is human-oriented text. An agent cannot ask ImageMagick "what layers does this document have?" because ImageMagick has no documents.
3. **No agent contract anywhere.** None of the incumbents offer machine-readable versioned output, per-command JSON Schemas, dry-run planning, consent gates for destructive or network actions, or provenance. Agents operating without these guardrails silently overwrite files, hallucinate flags, and produce unreproducible artifacts.
4. **Reproducibility is the product.** Agent workflows are judged by whether a result can be audited and regenerated. A sidecar that records the exact op-graph, input hashes, model versions, and seed turns "the agent made an image" into "the agent produced this verifiable artifact."

### 2.2 Gaps in existing command-line tools

| Tool | Strengths | Gaps Pixelox closes |
|---|---|---|
| **ImageMagick / GraphicsMagick** | Ubiquitous; broad codec support; one-shot filters & composites | No layer stack or document model; no named masks/selections; no adjustment layers or layer fx; no text shaping (RTL/CJK/ligatures); human-oriented output, no JSON contract or schemas; no provenance/replay; no ML ops; quoting-hostile `-draw` micro-language |
| **libvips (`vips` CLI)** | Extremely fast, streaming, low memory | Low-level op library, not an editor; no layers/masks/styles/type; no agent contract; C dependency web for full codec set |
| **Photoshop + ExtendScript/UXP** | Full feature set | Requires licensed GUI host; headless automation unsupported/fragile; no JSON contract; not cross-platform in CI; per-seat cost |
| **GIMP batch (Script-Fu)** | Free, layered model exists | Arcane Scheme interface; heavyweight startup; weak errors; no schemas, provenance, or agent surface |
| **Inkscape CLI / rsvg-convert** | Good SVG rendering | Vector-render only (rsvg) or slow actions API (Inkscape); no raster editing, retouch, or ML; no agent contract |
| **ffmpeg (stills)** | Great at video | Image stills are an afterthought: scaling/transcode only |
| **ferraus** | The agent contract, proven — for audio | Does not do images; Pixelox ports the contract to raster+vector |

### 2.3 Positioning

| Dimension | Photoshop | Illustrator | Fireworks (EOL) | ImageMagick | ferraus | **Pixelox** |
|---|---|---|---|---|---|---|
| Primary user | Human artist | Human artist | Human web designer | Shell scripts | Scripts & AI agents (audio) | **Scripts & AI agents (images)** |
| Interface | GUI | GUI | GUI | CLI | CLI + MCP | **CLI + MCP** |
| Layers, masks, styles | Yes | Partial | Yes | No | n/a | **Yes** |
| Vector authoring | Partial | Yes | Yes | No | n/a | **Yes (SVG-native)** |
| Non-destructive recipe | Partial (smart objects) | Partial | No | No | Yes (sidecars) | **Yes (op-graph + sidecars)** |
| Machine-readable output | No | No | No | Partial (`-format`) | Yes (versioned JSON) | **Yes (versioned JSON + schemas)** |
| Provenance & replay | No | No | No | No | Yes | **Yes** |
| ML-assisted selection/fill | Yes (cloud/local) | Limited | No | No | Yes (audio models) | **Yes (local ONNX, consent-gated)** |
| Deterministic & seedable | No | No | No | Partial | Yes | **Yes** |
| Headless CI use | No | No | No | Yes | Yes | **Yes** |
| Cost / license | Subscription | Subscription | Discontinued | Free (ImageMagick lic.) | Apache-2.0 OR MIT | **Apache-2.0 OR MIT** |

Positioning statement: *for AI agents and automation engineers who must produce finished image deliverables, Pixelox is the command-line image workstation that provides a real layered document model behind a verifiable machine contract — unlike ImageMagick, which edits pixels but not documents, and unlike Photoshop, which edits documents but only for humans.*

### 2.4 Relationship to ferraus

Pixelox adopts ferraus's conventions wholesale so that agents transfer knowledge between the two tools: the `group command` CLI shape, `--json` envelopes, `schema`/`describe --agent`/`docs`, `--dry-run`/`--overwrite`/`-y` consent gates, `<output>.pixelox.json` provenance sidecars, `replay`/`verify`, `jobs *` and `pipeline *`, a built-in MCP server with directory allow-listing, a Claude Code skill and `AGENTS.md`, the same packaging matrix, and the same repo topology (`crates/`, `docs/decisions/`, `examples/` run in CI, `packaging/`, `skills/`, `tools/`, `.claude-plugin/`). Where the domains differ (layer stacks and color management have no audio analog), Pixelox extends the contract rather than breaking it. Divergences from ferraus conventions require an ADR in `docs/decisions/`.

---

## 3. Personas & user stories

### 3.1 Persona A — "Clyde", an AI coding agent (via MCP or skill)

An LLM agent (e.g., Claude Code) operating a repository or a marketing pipeline. Calls Pixelox through the MCP server or the installed skill. Needs: a discoverable surface that fits in context, machine-parseable results, hard guarantees it cannot destroy user files accidentally, and a way to verify its own work.

| # | Story | Acceptance criteria |
|---|---|---|
| A1 | As an agent, I need the full command surface in under 4,000 tokens so I can plan without exhausting context. | `pixelox describe --agent` emits the complete command index, global flags, and envelope cheat-sheet in ≤ 4,000 tokens (CI-enforced token count). |
| A2 | As an agent, I want to remove the background from `hero.jpg` and get a transparent PNG. | `detect subject` + `export --apply-mask` produces RGBA PNG; envelope reports output path, SHA-256, and mask coverage %; sidecar written; exit 0. |
| A3 | As an agent, I must never clobber user files. | Any command writing to an existing path without `--overwrite` fails with exit 4 and error `PX1401`, writing nothing (atomic temp-file + rename everywhere). |
| A4 | As an agent, I want to check my own output. | `analyze diff a.png b.png --json` returns SSIM, ΔE00 percentiles, and changed-pixel count; `pixelox verify out.png.pixelox.json` confirms hashes. |
| A5 | As an agent, I plan before I act. | Every mutating command accepts `--dry-run` and returns the planned op-graph, predicted outputs (paths, dimensions, formats), and any consent requirements — writing nothing. |

### 3.2 Persona B — "Priya", pipeline engineer

Owns a media pipeline in CI (e-commerce imagery: derivatives, thumbnails, watermarks, format conversions). Needs: determinism across runs and machines, throughput, tiled memory behavior on 100-megapixel inputs, exit codes that gate pipelines, and no surprise network access.

| # | Story | Acceptance criteria |
|---|---|---|
| B1 | As a pipeline engineer, I run one recipe over 10k images nightly. | `pixelox batch run --glob 'in/*.jpg' --pipeline derivatives.yml --jobs 8` processes with bounded memory (`--mem-budget`), emits NDJSON progress, nonzero exit if any item fails (with per-item report), and supports `--continue-on-error`. |
| B2 | As a pipeline engineer, re-running last quarter's job must produce identical bytes. | Same binary version + target + inputs + seed ⇒ bit-identical outputs (CI contract test). |
| B3 | As a pipeline engineer, CI must never hit the network. | Default-deny network; model-dependent commands fail with exit 6 / `PX1502` and a named remedy (`pixelox models pull …`) when weights are absent. |
| B4 | As a pipeline engineer, I need 24 MP throughput that keeps up with the catalog. | Common single ops on 24 MP complete < 1 s on the reference 8-core box (§11.1 budgets). |

### 3.3 Persona C — "Mara", designer-developer automating marketing assets

Builds brand templates once, then generates campaign assets (posters, fliers, social cards) from copy decks and product shots. Needs: text that shapes correctly (including RTL and CJK), layer styles that match what design signed off in Photoshop, templates with typed slots, canvas presets, and PSD import of existing brand files.

| # | Story | Acceptance criteria |
|---|---|---|
| C1 | As a designer-developer, I turn `poster.pxo` + headline + product shot into a finished A3 poster in one command. | `comp template apply` fills text/image/color slots by name, re-flows text within slot constraints, errors `PX3202` on missing required slot, and exports print-resolution PNG/TIFF. |
| C2 | As a designer-developer, I import the brand's layered PSD and keep editing. | `open brand.psd -o brand.pxo` preserves layer tree, names, groups, opacity, blend modes, raster layer masks; unsupported features degrade per the documented PSD matrix (§7.6.1) with warnings in the envelope, never silently. |
| C3 | As a designer-developer, my Arabic and Japanese variants must not be mangled. | Text layers shape via HarfBuzz-class engine: RTL with bidi, CJK, ligatures, kerning; golden-image tests per script. |
| C4 | As a designer-developer, headline styling matches the approved comp. | `fx` set (drop/inner shadow, glows, bevel, stroke, overlays, satin) renders within perceptual tolerance of Photoshop reference renders on the golden corpus. |

### 3.4 Persona D — "Tomas", batch photo processor

Real-estate/product photographer processing hundreds of shots per shoot: straighten, white balance, shadows/highlights lift, vignette removal, watermark, resize, export ladder. Needs: good automatic corrections, EXIF handling, speed, and simple one-shot commands that don't require learning the document model.

| # | Story | Acceptance criteria |
|---|---|---|
| D1 | As a photographer, I watermark 500 images in one command. | One-shot pipeline over a glob places logo bottom-right at 85% opacity, multiply blend; 500 × 24 MP completes within the §11.1 batch budget; originals untouched. |
| D2 | As a photographer, I rescue underexposed interiors. | `adjust auto --tone`, `adjust shadows-highlights`, `adjust white-balance --auto` produce corrections within tolerance of the golden reference set; EXIF preserved by default, `--strip-metadata` to remove. |
| D3 | As a photographer, I fix tilted horizons without opening anything. | `transform straighten --angle auto` detects the horizon, rotates, and auto-crops; `--angle <deg>` overrides. |
| D4 | As a photographer, I export an entire size ladder at once. | One pipeline emits 2048/1024/512 px JPEG + WebP variants with correct ICC handling and sidecars for each. |

---

## 4. Goals & non-goals

### 4.1 Goals (v1.0 horizon)

| # | Goal | Measure |
|---|---|---|
| G1 | **Agent-complete contract**: JSON envelopes, schemas, describe/docs, dry-run, consent gates, sidecars, replay/verify on 100% of commands. | CI contract tests; `describe --agent` ≤ 4k tokens. |
| G2 | **Photoshop-analog raster editing**: layers, masks, selections, blend modes, adjustment layers, layer styles, filters, retouch, type. | Coverage map (§14.2) fully shipped through v0.2 scope. |
| G3 | **AI-assisted editing, locally**: segmentation, subject/salient detection, faces, OCR regions, matting, inpainting via local ONNX models, consent-gated downloads, SHA-256 verified. | §7.8 model registry shipped; budgets in §11.1 met on CPU. |
| G4 | **Vector authoring**: SVG scene-graph authoring, path booleans, trace, text-to-path. | §8.15–8.17 commands shipped; SVG output valid per usvg round-trip. |
| G5 | **Generative rendering**: sandboxed JS (d3/three) and native 3D-lite to raster or SVG. | §8.16 shipped; deterministic given seed; sandbox escapes = release blockers. |
| G6 | **Deliverable composition**: templates, presets, align/distribute, guides — data-to-poster in one command. | Persona C stories pass. |
| G7 | **Reproducibility**: bit-exact per version+target; perceptually verifiable cross-platform. | §11.3 determinism tests green on full matrix. |
| G8 | **Run anywhere**: Linux (gnu+musl), macOS, Windows, x86-64 + aarch64, one static-friendly binary, offline by default. | §11.4 platform matrix; packaging per §11.8. |

### 4.2 Non-goals

| # | Non-goal | Rationale / boundary |
|---|---|---|
| N1 | **GUI** of any kind (incl. TUI canvas preview) | The product is the contract. `export --scale` + the OS viewer is the preview. |
| N2 | **CAD-grade 3D** | §8.16 3D is wireframe/mockup/diagram grade: primitives, extrusions, OBJ/glTF import, iso/perspective cameras. No NURBS, no measurements, no tolerances. |
| N3 | **Video** | ferraus owns time-based audio; video is neither. Animated GIF frame extract/assemble is in scope (§8.18); timelines are not. |
| N4 | **Full-fidelity PSD write** | We write flattened PSD + "basic layered" PSD (§7.6.1). Round-tripping Photoshop's full feature set is explicitly out; it is a decades-deep proprietary surface. |
| N5 | **Bundled generative image models** (diffusion text-to-image) | Restoration-class models (matting, inpainting) are in scope. Generation is an integration point: agents can call external generators and `comp place` the result. Revisit post-1.0. |
| N6 | **Print prepress** | No CMYK separations, trapping, or imposition in v1. sRGB/Display-P3/AdobeRGB RGB workflows with ICC awareness only. CMYK PSD/TIFF *read* (converted to RGB) is stretch. |
| N7 | **Asset management / cloud service** | Pixelox is a local tool. No accounts, no telemetry, no upload. |
| N8 | **Plugin ABI in v1** | Extensibility via pipelines, JS render scripts, and external process composition; a stable plugin ABI is post-1.0. |

Stretch items throughout this PRD are marked **[stretch]** and are scheduled or explicitly deferred in §12.

---

## 5. Design principles

Adapted from the ferraus philosophy; these are binding on every design decision in this PRD and every PR against the repo. Violations require an ADR.

1. **Scripts and agents are first-class users.** Every feature ships CLI-first. If a capability cannot be expressed as a composable command with a JSON result, it does not ship.
2. **Plan before execute.** Every mutating command supports `--dry-run` and returns its plan. Agents are expected to plan, inspect, then commit.
3. **Destructive actions are opt-in.** Overwriting needs `--overwrite`; in-place document mutation needs `--in-place`; network (model/asset download) needs `-y` or a config consent; nothing deletes user data, ever (no `rm`-like commands on user files).
4. **Every result is reproducible and traceable.** Every written output gets a `<output>.pixelox.json` sidecar containing the full recipe; `replay` regenerates it; `verify` proves it.
5. **Machine-readable, versioned output.** `--json` on every command returns the stable envelope (§6.1). Fields are additive-only within a major envelope version. Schemas are part of the public API and snapshot-tested in CI.
6. **Non-destructive by default.** The document model is an op-graph over sources, not baked pixels. "Apply destructively" is an explicit flatten/merge/export step.
7. **Deterministic unless told otherwise.** Default seed is `0`; all randomness derives from the seed; threading never changes results. `--seed random` opts into entropy, and the drawn seed is recorded in envelope and sidecar.
8. **Offline-first, self-contained.** One binary; embedded fallback font and JS runtime; docs available via `pixelox docs` with no network; models download only on consent and verify by SHA-256.
9. **Fail loudly, precisely, machine-readably.** Stable error codes (`PX####`), a documented catalog, actionable hints, and meaningful exit codes. No silent fallbacks, no partial writes.
10. **Correct color before clever color.** sRGB assumed where untagged, ICC honored where tagged, linear-light math where correctness demands it, and the chosen behavior documented per op (§7.6).
11. **Docs and examples are executable.** Every example in README/PRD/`docs/` is extracted and run in CI against golden outputs — the ferraus rule.
12. **Small vendored C is acceptable; dependency webs are not.** Pure Rust preferred; a self-contained vendored C library (lcms2, libwebp) that compiles statically everywhere is acceptable; anything dragging a system dependency web (glib, system codecs) is not. This keeps the single-static-binary promise honest.
13. **One binary, no daemons.** Background jobs are detached child processes with on-disk state, not a service. `mcp serve` runs only as long as the client holds it open.

---

## 6. The agent contract

This section is the normative core of Pixelox. Everything here is stable API: envelope, sidecar, exit codes, error codes, and schema surface are versioned and regression-tested. The contract deliberately mirrors ferraus so agents transfer across the siblings.

### 6.1 The JSON envelope

Every command, with `--json`, writes exactly one envelope object to **stdout**. Human-readable logs and progress go to **stderr**. Without `--json`, stdout carries concise human output; the envelope is still available via `--json` in all modes including failures.

```json
{
  "tool": "pixelox",
  "version": "0.1.0",
  "envelope": 1,
  "command": "filter.gaussian",
  "ok": true,
  "dry_run": false,
  "seed": 0,
  "result": {
    "op": "filter.gaussian",
    "params": { "sigma": 8.0, "layer": "Background", "mask": "dog", "invert_mask": true },
    "document": { "path": "walk.pxo", "size": [6000, 4000], "color": "srgb", "depth": 8, "layers": 1 }
  },
  "inputs":  [ { "path": "/work/walk.pxo", "sha256": "9f2ab41c…", "bytes": 24117248 } ],
  "outputs": [ { "path": "/work/walk.pxo", "sha256": "41c7d88e…", "bytes": 24201114,
                 "sidecar": "/work/walk.pxo.pixelox.json" } ],
  "warnings": [],
  "timing_ms": 412
}
```

Envelope rules:

| Rule | Specification |
|---|---|
| Versioning | `envelope` is an integer, currently `1`. Fields are additive-only within a version; removal/retyping bumps it (major event, CHANGELOG + migration notes). |
| Command id | Canonical dotted id (`filter.gaussian`) — the CLI's `group command` joined with `.`. Used in envelopes, sidecars, pipelines, schemas, and MCP. |
| `ok` / `error` | `ok: true` ⇒ `result` present. `ok: false` ⇒ `error` object present (§6.7), `result` absent, exit code per §6.6. |
| `result` | Command-specific, described by `pixelox schema <command>` (the `result` sub-schema). |
| Hashing | Every input/output path reported with SHA-256 and byte size. Directories report a Merkle-style digest of contained files. |
| `warnings[]` | Structured: `{ "code": "PXW0201", "message": "...", "context": {...} }`. Warnings never change exit code. |
| Streaming progress | Long ops with `--progress json` emit NDJSON events to stderr: `{"event":"progress","command":"detect.subject","step":"inference","pct":62}`. Stdout stays reserved for the single final envelope. |
| Failure envelopes | Failures still emit a complete envelope (with `error`) on stdout when `--json` is set; exit code is authoritative for scripting. |
| Encoding | UTF-8, no ANSI in JSON mode, newline-terminated single object (not NDJSON) for the envelope itself. |

### 6.2 `schema`, `describe`, `docs`

| Command | Behavior |
|---|---|
| `pixelox schema <command>` | Prints the command's JSON Schema (draft 2020-12): `params`, `result`, and `errors` it can raise. `--all` emits every schema as one bundle. Schemas carry `x-pixelox-schema-version` (semver per command) and are snapshot-tested; additive changes bump minor, breaking changes are forbidden within envelope v1 post-1.0. |
| `pixelox describe` | Human overview of the full surface. `--agent` emits the compressed agent briefing: command index with one-liners, global flags, envelope cheat-sheet, mask/layer addressing rules, and the five golden workflows — **hard budget ≤ 4,000 tokens, enforced by a CI token-count test**. `--json` emits the machine index (ids, synopses, schema versions). |
| `pixelox docs <command>` | Full offline reference page per command (compiled into the binary): synopsis, every flag, semantics, examples, related commands, error codes. `pixelox docs errors/PX1401` resolves catalog entries. `--format md|text`. |

### 6.3 Plan and consent: `--dry-run`, `--overwrite`, `-y`

- **`--dry-run`** (all mutating commands): validates inputs, resolves references (layers, masks, fonts, models), computes the op plan, and returns the envelope with `"dry_run": true` and a `plan` block — predicted outputs (paths, dimensions, formats, estimated bytes), ops to be applied, models required (with download size if absent), and consent flags that would be needed. Writes nothing, including sidecars; `-o`/`--in-place` may be omitted under `--dry-run`. Read-only commands accept and ignore it (warning `PXW0001`).
- **`--overwrite`**: required to replace any existing file. Default behavior is refuse with exit 4 / `PX1401`. All writes are atomic: temp file in the destination directory, fsync, rename; a failed command never leaves partial output or a stale sidecar.
- **`--in-place`**: required to mutate a `.pxo` given as input; otherwise mutating commands demand `-o`. Applies only to `.pxo` documents — Pixelox never edits a source image file in place.
- **`-y` / `--yes`**: grants consent for network actions (model/asset download) for this invocation. Config `models.auto_consent = true` grants it standingly; CI images can bake models in and never consent. Absent consent, the command fails with exit 6 / `PX1502` naming the exact `pixelox models pull <id>` remedy.

### 6.4 Provenance sidecar

Every written output `X` gets `X.pixelox.json` (suppress with `--no-sidecar`; forbidden to suppress in `pipeline run` unless the pipeline file says so). The sidecar is self-contained: it embeds the **full recipe** (op chain back to source inputs), not just the last command, so replay does not require the intermediate `.pxo`.

```json
{
  "pixelox_sidecar": 1,
  "tool": { "name": "pixelox", "version": "0.1.0", "build": "git:a1b2c3d",
            "target": "x86_64-unknown-linux-gnu", "profile": "release" },
  "created_utc": "2026-11-04T14:03:22Z",
  "command": {
    "id": "export",
    "argv": ["pixelox","export","walk.pxo","-o","walk-blur.png","--bit-depth","8"],
    "params": { "format": "png", "bit_depth": 8, "scale": 1.0 },
    "seed": 0
  },
  "recipe": [
    { "op": "open",            "params": { "path": "walk.jpg" }, "input_sha256": "9f2ab41c…" },
    { "op": "detect.subject",  "params": { "save_mask": "dog" },
      "model": { "name": "birefnet-lite", "version": "1.0.0", "sha256": "77aa01…" } },
    { "op": "mask.feather",    "params": { "mask": "dog", "radius": 12 } },
    { "op": "filter.gaussian", "params": { "sigma": 8.0, "mask": "dog", "invert_mask": true } },
    { "op": "export",          "params": { "format": "png", "bit_depth": 8 } }
  ],
  "inputs":  [ { "role": "source", "path": "walk.jpg", "sha256": "9f2ab41c…", "bytes": 8812344 } ],
  "models":  [ { "name": "birefnet-lite", "version": "1.0.0", "sha256": "77aa01…" } ],
  "fonts":   [],
  "output":  { "path": "walk-blur.png", "sha256": "41c7d88e…", "bytes": 19233807,
               "size": [6000, 4000], "color_profile": "sRGB IEC61966-2.1", "bit_depth": 8 },
  "env": { "threads": 8, "mem_budget_mb": 4096, "locale": "C" },
  "determinism": { "bit_exact_scope": "same-version-same-target",
                   "perceptual": { "ssim_min": 0.999, "de00_p99_max": 1.0 } }
}
```

Notes: paths are stored as given (relative stays relative) plus a `base_dir`; `--redact-paths` replaces directories with hashes for shareable sidecars. Sidecar schema is versioned (`pixelox_sidecar: 1`) and published via `pixelox schema sidecar`.

### 6.5 `replay` and `verify`

| Command | Behavior |
|---|---|
| `pixelox replay <sidecar> -o <out>` | Re-executes the embedded recipe. Locates inputs by recorded path (override with `--input role=path`), verifies their SHA-256 first (mismatch ⇒ exit 9 / `PX1601` unless `--allow-input-drift`), pulls required models (consent rules apply), and re-renders. `--strict` requires bit-exact match with recorded output hash; default compares perceptually when the current target differs from the recorded one. |
| `pixelox verify <sidecar>` | `--fast` (default): confirm the referenced output and inputs still match recorded hashes. `--full`: replay into a temp dir and compare (bit-exact same-target, perceptual cross-target per the sidecar's `determinism` block). Exit 0 verified; exit 9 with a structured mismatch report otherwise. |

### 6.6 Determinism & seeded randomness

- **Seed model.** Global `--seed <u64>` (default `0`). Every stochastic op (dissolve blend, noise, dithering, k-means in `analyze colors`, JS `px.random()`, model samplers) draws from a ChaCha12 stream keyed by `(seed, op-index-in-recipe, op-id)`, so inserting an op does not shift unrelated ops' draws. `--seed random` pulls from OS entropy and records the value.
- **Threading.** Tile scheduling and parallel reductions use fixed-order combination; thread count never affects output bytes.
- **Bit-exactness scope.** Same binary version + same target triple ⇒ bit-identical outputs. Across targets/versions: perceptual equivalence guaranteed (SSIM ≥ 0.999, ΔE00 p99 ≤ 1.0 on the golden corpus); float kernel differences across architectures make cross-target bit-exactness an explicit non-promise (integer-only 8-bit op chains are bit-exact cross-platform in practice and CI-tested, but not contractually promised).
- **Time and locale.** No wall-clock reads inside ops; sidecar timestamps are metadata only. All parsing/formatting is locale-independent (`C` semantics).
- **ML determinism.** ONNX Runtime pinned per release; single-threaded deterministic EP mode available via `--ml-deterministic` (slower, bit-stable); default mode is deterministic per version+target like the rest of the pipeline.

### 6.7 Exit codes

| Code | Meaning |
|---|---|
| 0 | Success (warnings allowed). |
| 1 | Operation failed at runtime (op error, codec failure, corrupt input). |
| 2 | Usage error: unknown command/flag, invalid params (schema violation). |
| 3 | Input missing or unreadable. |
| 4 | Refusing to overwrite / mutate without `--overwrite` / `--in-place`. |
| 5 | Unsupported format or feature in this build (e.g., HEIC without feature). |
| 6 | Consent required: model/asset download needed and `-y` absent. |
| 7 | JS sandbox violation, script error, or resource-limit kill. |
| 8 | Job error: unknown id, already finished, cancel failed. |
| 9 | Verification/replay mismatch. |
| 10 | Interrupted or timed out. |
| 11 | Memory budget exceeded (op aborted cleanly). |
| 12 | License/feature gate: command present but excluded from this build variant. |

### 6.8 Error catalog

Stable identifiers `PX####`, allocated in ranges; the catalog ships in the binary (`pixelox docs errors`) and in `docs/errors.md`. Every error carries `code`, `message`, `hint` (next action), and `context` (offending values). Codes are append-only; meanings never change.

| Range | Domain | Example |
|---|---|---|
| PX10xx | CLI usage & params | `PX1002` unknown flag; `PX1010` param fails schema |
| PX11xx–13xx | IO, formats, codecs | `PX1203` unsupported PSD color mode (CMYK) |
| PX14xx | Write protection | `PX1401` output exists, `--overwrite` missing |
| PX15xx | Models & network | `PX1502` model absent, consent required; `PX1503` SHA-256 mismatch |
| PX16xx | Provenance | `PX1601` input hash drift on replay |
| PX2xxx | Document model | `PX2301` no such layer; `PX2310` mask name collision |
| PX3xxx | Ops & params semantics | `PX3202` template slot unfilled; `PX3310` curve points not monotonic |
| PX4xxx | Text & fonts | `PX4102` font family not found (lists candidates) |
| PX5xxx | ML inference | `PX5201` model input too large for budget |
| PX6xxx | JS sandbox | `PX6002` wall-clock limit exceeded; `PX6010` forbidden API |
| PX7xxx | Jobs & pipelines | `PX7103` cyclic pipeline graph |
| PX9xxx | Internal invariants | `PX9001` panic trap — always a bug, auto-suggests issue template |

Error envelope example:

```json
{
  "tool": "pixelox", "version": "0.1.0", "envelope": 1,
  "command": "export", "ok": false, "dry_run": false, "seed": 0,
  "error": {
    "code": "PX1401",
    "message": "output 'walk-blur.png' exists and --overwrite was not given",
    "hint": "re-run with --overwrite, or choose a new path with -o",
    "docs": "errors/PX1401",
    "context": { "path": "/work/walk-blur.png" }
  },
  "warnings": [], "timing_ms": 3
}
```

---

## 7. Core architecture

### 7.1 Stack recommendation

**Decision: a Rust workspace, mirroring ferraus, compiling to one static-friendly binary.** Rust gives memory-safe codec and kernel code (image parsing is a fuzzing minefield), fearless data-parallel tiling (rayon), first-class cross-compilation incl. musl static builds, and direct reuse of the ferraus packaging/CI machinery. The alternatives were examined and rejected: C++ (unsafe codec surface, packaging pain), Go (weaker SIMD/image ecosystem, cgo wall for ONNX/Skia), Python (startup time, packaging, GIL — disqualified by the <50 ms startup budget alone).

Key stack decisions, each argued in its subsection below:

| Decision | Choice | Rejected alternative | Section |
|---|---|---|---|
| Raster paint/compositing backend | **tiny-skia + custom f32 layer compositor** | skia-safe (full Skia bindings) | §7.5 |
| Codecs | **image-rs family + targeted pure-Rust crates + small vendored C** | libvips binding | §7.6 |
| SVG parse/render | **usvg + resvg** (render to tiny-skia) | librsvg, Skia SVG module | §7.9 |
| Text shaping/layout | **fontdb + rustybuzz + cosmic-text** | HarfBuzz C bindings, Pango | §7.7 |
| ML runtime | **ONNX Runtime via `ort`**, CPU EP default | candle (tracked), tract | §7.8 |
| JS engine | **QuickJS-ng via `rquickjs`** + Canvas2D binding + DOM-lite | deno_core/V8 (fallback), headless Chromium (rejected) | §7.10 |
| Path booleans / geometry | **kurbo + i_overlay-class boolean engine** | lyon only, Clipper C++ | §7.9 |
| Color management | **lcms2 (vendored C, MIT)** | qcms/moxcms pure-Rust (tracked) | §7.4 |
| CLI framework | **clap v4** (derive), lazy help assembly | — | §7.2 |

### 7.2 Workspace layout

```
pixelox/
├── crates/
│   ├── pixelox-core/      # document model: layer tree, op-graph, scheduler, tiles, caching, color pipeline
│   ├── pixelox-raster/    # pixel kernels: filters, adjustments, blends, resampling, retouch, quantize/dither
│   ├── pixelox-vector/    # paths (kurbo), shapes, booleans, SVG scene model + writer, usvg/resvg bridge
│   ├── pixelox-text/      # fontdb discovery, rustybuzz shaping, cosmic-text layout, text-on-path, to-path
│   ├── pixelox-ml/        # ort runtime host, model registry/downloader/verifier, detect/matte/inpaint ops
│   ├── pixelox-io/        # codecs (read/write), PSD read/basic-write, .pxo container, sidecar read/write, EXIF/ICC
│   ├── pixelox-js/        # rquickjs sandbox, px.* API, Canvas2D binding, DOM-lite, vendored d3/three bundles
│   ├── pixelox-3d/        # native 3D-lite: primitives, extrude, OBJ/glTF import, cameras, hidden-line + flat shading
│   ├── pixelox-cli/       # clap surface, envelopes, errors, jobs, pipelines, replay/verify, docs/describe/schema
│   └── pixelox-mcp/       # MCP server (stdio transport), tool defs, allow-listing
├── docs/                  # reference docs (compiled into binary), errors.md, decisions/ (ADRs)
├── examples/              # shell scripts, pipeline files, MCP client, Claude agent loop — all run in CI
├── packaging/             # brew, scoop, MSI/wix, deb/rpm, Docker, curl|sh + ps1 installers
├── skills/pixelox/        # SKILL.md (+ recipes) for Claude Code
├── tools/                 # xtask: golden-corpus runner, token-budget check, schema snapshots, docs extraction
├── .claude-plugin/        # plugin manifest exposing the skill
├── testdata/              # small golden corpus (<20 MB); large corpus in pixelox-testdata repo, pulled in CI
├── AGENTS.md  CHANGELOG.md  PRD.md  README.md  HOWTO.md  LICENSE-APACHE  LICENSE-MIT
```

Feature flags per build variant: `default = ["ml", "webp-lossy"]`; `musl-static` builds default minus `ml` (§7.8); `mozjpeg`, `heic`, `raw`, `pdf-export` are opt-in features (the latter three **[stretch]**). `pixelox doctor` and exit 12 report gated features at runtime.

### 7.3 Document model: op-graph + layer tree

The document is **a tree of layers whose pixels are defined by ops, not stored as baked state** — Photoshop's mental model implemented as a build system.

- **Layer tree.** Node kinds: `raster`, `text`, `shape` (vector), `adjustment`, `group`, `placed` (linked/embedded external file — the smart-object analog). Every node has: id (ULID) + unique name, visibility, lock, opacity (0–100), blend mode (§14.3 full set incl. `pass-through` for groups), optional **layer mask** (8/16-bit alpha, with density and feather parameters, enable/disable), optional vector mask, clip flag (clipping mask onto the nearest non-clipped base below), and an fx stack (§8.13).
- **Ops.** Every mutation is an op record `{ op_id, params, seed_lane }` appended to the layer or document op list. Raster layers hold a source ref (file hash or generated) plus their op chain; adjustment layers are ops lifted into the tree that apply to the composite below them, restricted by their mask; `placed` layers re-render from source on `comp update-links`.
- **Evaluation.** Rendering is a pull-based DAG evaluation: `composite(root) ← blend(children)` with per-node caches keyed by `(node content hash, op chain hash, tile id)`. Editing one layer invalidates only dependent cache entries. Tiles are 512×512, planar f32 (or u8/u16 fast paths for integer-safe chains), evaluated in parallel with fixed-order reduction (§6.6).
- **Selections & masks.** The document carries one **active selection** (soft 8-bit coverage map, addressable as `@sel`) and a **named mask library** (document-level grayscale images). Layer masks are attachments on nodes. All three are interconvertible (§8.4–8.5); every region-scoped op takes `--mask <ref>`.
- **History.** The op list is the history. `pixelox history list|undo|checkpoint` **[v0.2]** manipulates the tail; `.pxo` stores the full chain, so any historical state is recoverable and the sidecar recipe is a projection of it.

Complexity stance (binding): compositing and filters are O(pixels) per op with separable/IIR implementations for large kernels (Gaussian via recursive filter at σ>10, box via summed-area/sliding window, median via constant-time histogram method); resampling is separable two-pass; no op may be accidentally quadratic in image dimension; peak memory respects `--mem-budget` via tile streaming (§11.2).

### 7.4 Color management

- Internal working space: document color space (default sRGB) with 8/16-bit integer or f32 channels; all f32 math in **premultiplied alpha**, un-premultiplied at codec boundaries.
- **lcms2 (vendored, MIT, static-friendly C)** performs ICC transforms: honored on open (tagged files converted or assigned per `--icc honor|assign|ignore`), embedded on export (`--profile srgb|display-p3|adobe-rgb|file.icc`). Untagged inputs are assumed sRGB. Pure-Rust CMS (qcms/moxcms) is tracked as a swap (§13).
- **Linear-light correctness:** operations that are physically additive run in linear by default — resampling **[default gamma-space to match Photoshop expectations, `--linear` flag]**, Gaussian/lens blurs (`--linear`, default off for PS parity), gradient interpolation (`--interp linear|perceptual|oklab`), and compositing (document setting `blend-space: perceptual|linear`, default perceptual = PS-compatible). Defaults favor *visual parity with Photoshop*; correctness switches are always available and recorded in the recipe.
- 16-bit sources stay 16-bit end-to-end; adjustment math is f32 regardless of storage depth. HDR (f32 EXR-class) is post-1.0.

### 7.5 Raster backend: tiny-skia, not skia-safe

**Decision: tiny-skia as the path rasterizer and 8-bit paint backend, plus our own f32 compositor and filter kernels in `pixelox-raster`.**

Rationale:

1. **Distribution is a feature.** skia-safe drags in the full Skia C++ build (or prebuilt binaries per-target), breaking musl static builds, slowing CI by tens of minutes, and bloating the binary ~20 MB before we write a line. tiny-skia is pure Rust, compiles everywhere Rust does, and is already the proven backend under resvg (§7.9), so raster and vector share one rasterizer.
2. **We need our own compositor anyway.** Photoshop's blend-mode set includes modes Skia lacks entirely (linear burn, vivid/linear/pin light, hard mix, darker/lighter color, dissolve, subtract, divide), and our layer engine needs 16-bit/f32 pipelines that tiny-skia's premultiplied-u8 pixmaps don't provide. Full Skia would still leave us writing the blend/adjustment core — so pay for the small rasterizer, not the large one.
3. **Scope fit.** What we use a paint backend for — anti-aliased path fill/stroke with transforms, gradients, and pattern shaders — is exactly tiny-skia's surface. Filters, adjustments, and retouch are bespoke SIMD kernels (via `wide`/`std::simd` + rayon tiles) regardless of backend.

Costs accepted: we implement ~10 extra blend modes, high-quality gradient dithering, and all filter kernels ourselves (they are the product; owning them is correct), and we forgo Skia's GPU path (GPU is post-1.0, via wgpu compute if ever — §13). Reversal trigger: if golden-corpus parity for fx rendering proves materially harder than budgeted, revisit skia-safe behind a feature flag — recorded as ADR-0002.

### 7.6 Codecs: image-rs family, not libvips

**Decision: pure-Rust codec stack (image-rs ecosystem) with targeted vendored-C upgrades; no libvips.** libvips is the fastest streaming library alive, but binding it imports a glib-sized dependency web that kills static distribution, and our tiled op-graph (§7.3) already provides the streaming behavior we need at the document level.

Format matrix (R = read, W = write; bracketed items **[stretch]**):

| Format | Support | Implementation | Notes |
|---|---|---|---|
| JPEG | R/W | zune-jpeg (decode), jpeg-encoder (encode); `mozjpeg` feature for max-quality encode | EXIF/ICC honored; progressive R/W |
| PNG | R/W | `png` crate | 8/16-bit, palette, **transparent PNG**, APNG R/W |
| GIF | R/W | `gif` crate | Animated: first frame on `open` by default, `--frame N`, full extract/assemble via `frames *` (§8.18); pure-Rust quantizer + FS/ordered dithering (libimagequant excluded — GPL-dual) |
| TIFF | R/W | `tiff` crate | Multi-page via `--page N`; 8/16-bit; LZW/Deflate; stripped & tiled |
| WebP | R/W | image-webp (R + lossless W); vendored libwebp for lossy W (`webp-lossy` feature, default on) | Animated WebP read **[stretch]** |
| BMP | R/W | image-rs | — |
| **PSD** | **R (layered!)** / W (scoped) | `psd` crate, extended/forked as needed | §7.6.1 fidelity matrix |
| SVG | R/W | usvg (R), own scene-graph writer (W) | §7.9 |
| HEIC | R **[stretch]** | system libheif via dynamic load, `heic` feature | Never bundled: LGPL + HEVC patent pool (§13) |
| RAW (CR3/NEF/ARW/DNG…) | R **[stretch]** | rawler-class pure-Rust decoder | rawler is LGPL-2.1 — isolate as separately-built helper or dynamic feature (§13) |
| PDF | W (single page) **[stretch]** | svg2pdf + pdf-writer | Vector layers stay vector; raster embedded |
| `.pxo` | R/W (native) | pixelox-io | §7.11 |

Safety: every decoder is fuzzed (cargo-fuzz targets in CI); decompression-bomb guard rejects > 512 MP by default (`--limit-pixels` overrides); EXIF orientation auto-applied on open (`--no-auto-orient` opts out).

#### 7.6.1 PSD fidelity matrix (read) and PSD write scope

| PSD feature | v0.1 read behavior |
|---|---|
| Raster layers, names, groups, visibility, opacity | Full |
| Blend modes | Full mapping (§14.3) |
| Layer masks (raster) | Full, incl. density/feather where present |
| 8-bit RGB/RGBA | Full. 16-bit **[stretch v0.3]**; CMYK/Lab → converted to RGB with warning **[stretch]** |
| Text layers | Imported as raster (PSD stores a composite) + metadata (content string, font name, size) preserved in layer props, so agents can re-set them as native text |
| Adjustment layers | Imported as pixelox adjustment layers for the supported set (levels, curves, HSL, B&W, …); unsupported kinds fall back to the flattened composite for that subtree, with warning `PXW1203` |
| Smart objects | Rasterized composite + original bytes preserved as attachment |
| Layer styles (fx) | Parsed to pixelox fx where parameters map (shadow/glow/stroke/overlays); others warn + render from PSD's baked results when present |

**PSD write:** `export doc.pxo -o out.psd` writes **flattened** PSD always, or `--layered-basic`: raster layers with names, groups, opacity, blend modes, raster masks. Text/shape/adjustment layers are rasterized on write (warning lists them). Full-fidelity PSD write is a **non-goal** (§4.2 N4).

### 7.7 Text stack (`pixelox-text`)

- **Discovery:** fontdb enumerates system fonts plus `fonts add` directories; an embedded Noto Sans subset (OFL-1.1) is the guaranteed fallback, so text renders — identically — on bare CI containers.
- **Shaping:** rustybuzz (the HarfBuzz port): bidi/RTL, script itemization, ligatures, kerning, OpenType features; variable-font axes (`--variation`) applied at shaping and outline time.
- **Layout:** cosmic-text for line breaking, paragraph wrapping (`--box`), alignment/justification; tracking/leading/kerning overrides applied on top.
- **Rendering:** glyph outlines (ttf-parser) filled as paths through the §7.5 pipeline — no hinting or embedded-bitmap path, so type is resolution-independent and byte-identical across platforms. `text to-path` reuses the same outlines.
- **Text-on-path:** glyph advances mapped along a kurbo path, each glyph rotated to the baseline tangent; `--offset`/`--side` control placement.
- **Reproducibility:** the resolved face (family, style, variation coords) and its file SHA-256 are recorded in sidecars (§6.4 `fonts`); a missing font at replay raises `PX4102` with candidates and the `fonts add` remedy.
- Color/emoji fonts (COLR/CPAL, SVG-in-OT) **[stretch]**; family fallback chains are reported by `doctor`.

### 7.8 ML subsystem (`pixelox-ml`)

**Runtime decision: ONNX Runtime via the `ort` crate, CPU execution provider by default.** ORT has the operator coverage to run every candidate model unmodified, mature quantization support, and optional acceleration EPs (CoreML on macOS, DirectML on Windows — opt-in flags, off by default for determinism). Cost: ORT links ~30–40 MB and resists musl static linking — therefore **the `musl-static` build ships without `ml`** and `doctor` says so; the standard gnu/darwin/windows builds include it. `candle` (pure Rust, musl-clean) is the tracked escape hatch (§13) and may later power a `ml-lite` static variant.

Model registry — downloaded on first use to the platform data dir, consent-gated (`-y`/config), SHA-256 pinned in the binary per release, listed/managed via `models *` (§8.21). License policy: **Apache-2.0/MIT-licensed weights only in the default registry** (this excludes RMBG (CC-BY-NC) and YOLOv8 (AGPL) deliberately); users can register custom ONNX models via `models add --from file.onnx` at their own license risk.

| Task | Default model | Class / license | Size (approx) | Used by |
|---|---|---|---|---|
| Promptable segmentation | **MobileSAM** | SAM-class, Apache-2.0 | ~40 MB | `detect segment` (point/box/text-prompted masks) |
| High-quality segmentation **[stretch]** | SAM 2.1-small | Apache-2.0 | ~180 MB | `detect segment --model sam2` |
| Subject / background removal | **BiRefNet-lite** | DIS-class, MIT | ~170 MB (fp16 ~85) | `detect subject` |
| Fast salient preview | u2netp | U²-Net, Apache-2.0 | ~5 MB | `detect salient --fast` |
| Edge-grade matting (hair/fur) | **ViTMatte-S** | Apache-2.0 | ~100 MB | `mask refine --model vitmatte` |
| Inpainting / content-aware fill | **LaMa (big-lama)** | Apache-2.0 | ~200 MB | `fill content-aware` |
| Object detection (80-class) | **YOLOX-s** | Apache-2.0 | ~35 MB | `detect objects` |
| Open-vocabulary detection **[stretch]** | GroundingDINO-T | Apache-2.0 | ~700 MB | `detect objects --prompt "red car"` |
| Face detection + landmarks | **YuNet** | MIT | ~1 MB | `detect faces`, `retouch redeye --auto` |
| Text detection (OCR regions) | **PP-OCRv4 det** | Apache-2.0 | ~5 MB | `detect text` |
| Text recognition **[stretch]** | PP-OCRv4 rec (en + multilingual packs) | Apache-2.0 | ~10 MB/pack | `detect text --recognize` |

Inference pipeline: pre/post-processing (resize-to-model, normalization, tiling for large inputs, logits→mask with configurable threshold) lives in `pixelox-ml`, versioned as part of the op (model swap = param change = visible in recipe). Int8-quantized variants offered where quality holds (`--model-precision int8|fp16|fp32`).

### 7.9 Vector stack

- **Geometry core:** kurbo (Bézier math) + lyon (tessellation where needed). **Path booleans** (unite/subtract/intersect/exclude) via an i_overlay-class pure-Rust boolean engine operating on adaptively flattened curves at tolerance (default 0.1 px at document scale), with curve re-fitting **[stretch]**; potrace is GPL and is not linked.
- **SVG in:** usvg parses/normalizes; resvg renders to tiny-skia pixmaps (full static SVG 1.1 incl. filter primitives `feGaussianBlur`, `feDropShadow`, `feBlend`, `feColorMatrix`, `feComponentTransfer`, `feComposite`, `feConvolveMatrix`, `feDiffuseLighting`, `feSpecularLighting`, `feDisplacementMap`, `feFlood`, `feImage`, `feMerge`, `feMorphology`, `feOffset`, `feTile`, `feTurbulence`; no SMIL/scripting). External hrefs disabled by default (`--allow-external`).
- **SVG out:** pixelox owns a writable SVG scene-graph (the `svg *` surface, §8.15) serialized via a streaming XML writer; `svg from-doc` exports a document's vector/text/shape layers as structured SVG (raster layers embedded as PNG images or dropped with `--vector-only`).
- **Trace:** vtracer-class pure-Rust tracing (color and binary modes: pixel→spline clustering, corner/segment fitting) exposed as `trace` (§8.17). Centerline tracing **[stretch]**.

### 7.10 JS render sandbox (`pixelox-js`): QuickJS, not V8, not a browser

Requirement 14 asks for d3-style data-driven 2D and three.js-style 3D rendered onto a canvas. Three candidate designs:

| Option | Pros | Cons |
|---|---|---|
| Headless Chromium | Perfect web compat | +150–300 MB, non-deterministic (GPU/raster/scheduling), un-sandboxable to our standard, destroys single-binary distribution. **Rejected for core** (may exist someday as an external integration recipe, never a dependency). |
| deno_core (V8) | JIT speed for heavy data crunching; real ES toolchain | +25–35 MB binary, rusty_v8 build/prebuilt complexity per target, snapshot management, larger attack surface; V8 determinism is good but configuration-sensitive. |
| **QuickJS-ng via rquickjs** | ~1.5 MB, trivially embedded + instruction/memory metering, deterministic, musl-clean, full ES2023 | Interpreter speed (~10–50× slower than V8 on pure-JS hot loops) |

**Decision: QuickJS-ng.** The deciding insight: in render scripts, pixel-heavy work (canvas ops, path filling, layout) executes in **Rust** behind the Canvas API — JS only orchestrates. d3 building 10k SVG nodes or three.js projecting 50k triangles is well within interpreter budgets (§11.1 sets 2 s/5 s targets). If a real workload class breaks the budget, `deno_core` becomes a feature-gated alternate engine behind the same `px.*` API — ADR-0003 documents the trigger. A headless browser is rejected outright: it cannot meet determinism, size, or sandbox requirements.

Sandbox design:

- **API surface:** a frozen global `px` object only: `px.canvas(w,h)` → Canvas2D-compatible context implemented over pixelox-raster/vector; `px.svg()` → DOM-lite SVG document; `px.data` (inputs passed via `--data name=file`, pre-parsed CSV/JSON/text); `px.random()` (seeded, §6.6); `px.params` (from `--param k=v`); `px.log()`; `px.output(nodeOrCanvas)`. No `fetch`, no filesystem, no `eval` of external code, no timers, no workers, no WebAssembly. `Date.now()` returns 0 (`--allow-time` opts in, recorded in recipe as a determinism breach warning).
- **DOM-lite:** a vendored, pinned, pure-JS minimal DOM (linkedom-derived, trimmed to SVG + events-free subset) so **d3-selection operates on real nodes**; the resulting SVG tree serializes into the §7.9 pipeline. 
- **Bundled libraries:** `d3` (pinned 7.9.x) and `three` (pinned r169) compiled into the binary (compressed, ~1 MB total); `render d3|three` presets preload them. three.js renders via its `SVGRenderer` (projected vector output — ideal for wireframe/line-art) or via a `PxCanvas` software path for shaded raster **[v0.4]**. User scripts may not import arbitrary modules; `--lib file.js` loads additional local libraries explicitly (hash recorded in sidecar).
- **Limits:** wall clock 30 s (`--timeout`), heap 512 MB (`--js-mem`), interpreter fuel metering, output canvas ≤ document pixel limit. Violations: exit 7, `PX60xx`.
- **Determinism:** same script + data + params + seed + pixelox version ⇒ identical output. Library versions are pinned per pixelox release and recorded in sidecars.

### 7.11 3D-lite subsystem (`pixelox-3d`)

Native Rust mini-pipeline, independent of the JS path, for requirement 15 — *mockup/wireframe/diagram grade, explicitly not CAD*:

- **Scene:** declarative JSON/TOML scene files (`.p3d.json`) or flags: primitives (box, sphere, cylinder, cone, torus, plane), 2D-profile **extrusions** (from an SVG path or a document shape layer — the "extruded logo" case), and mesh import via `gltf` and OBJ parsing **[glTF v0.4, OBJ v0.4]**.
- **Cameras:** isometric/orthographic presets (`--camera iso|front|top|dimetric`) and perspective (`--camera persp --fov 40 --look-at …`).
- **Rendering:** pure-CPU: (a) **hidden-line wireframe** → vector output (SVG paths with occlusion-culled segments) or raster; (b) **flat/Lambert shaded** → raster with one directional + ambient light, optional outline pass. No PBR, no shadows-mapping, no GPU requirement — eliminating the headless-GPU availability risk entirely (§13).
- Output feeds the normal pipeline: `-o out.svg` or `-o out.png`, or `--as-layer` into a document.

### 7.12 The `.pxo` native format

A **ZIP container** (deflate, zip64 for >4 GB) — chosen over a bespoke binary for inspectability (`unzip -l` is debugging) and incremental update support; OpenRaster/`.ora` validated the approach, and `.pxo` deliberately rhymes with it:

```
doc.pxo
├── mimetype                     # "application/x-pixelox", stored uncompressed, first entry
├── manifest.json                # format_version: 1, tool version, created, doc summary, integrity table
├── doc.json                     # layer tree: nodes, props, fx, masks refs, op chains, guides, grids, template slots
├── recipe.json                  # linearized full op history (the replay projection)
├── assets/                      # content-addressed source bytes: sha256-<hash>.<ext> (original files, placed links' snapshots)
├── masks/                       # named mask library + layer masks: 8/16-bit grayscale PNG
├── cache/                       # OPTIONAL rendered tiles/previews; always safe to delete; `save --no-cache` omits
└── thumbs/preview.png           # 512px composite preview for humans and MCP clients
```

Properties: assets are content-addressed (dedup across layers); `doc.json` references by hash; byte-identical sources stored once. **Forward compatibility:** readers must ignore unknown JSON fields; `format_version` bumps are migration events with documented, reversible migrations (`pixelox doctor --migrate-pxo`). Determinism: `save` normalizes zip metadata (fixed timestamps, stable entry order) so identical documents produce identical bytes. Embedded-vs-linked: `comp place --link` stores a reference + snapshot hash; `--embed` (default) copies bytes into `assets/`.

### 7.13 Jobs, pipelines, and process model

- **No daemon.** `--background` on any long command forks a detached worker; job metadata/logs/envelopes live under the platform state dir (`~/.local/state/pixelox/jobs/<id>/`). `jobs list|status|wait|cancel|result` read/manage that state; `jobs result <id>` replays the final envelope verbatim.
- **Pipelines** (§9) execute a declared step graph in one process — one document load, N ops, one save — amortizing per-invocation open/decode costs, which matters: chaining 12 CLI invocations re-decodes 12 times; the equivalent pipeline decodes once. Agents should prefer pipelines for >3-step workflows; `describe --agent` says so.
- **Config precedence:** CLI flag > `PIXELOX_*` env > `~/.config/pixelox/config.toml` > built-in default. `config get|set|list|path` manages it; every effective value is visible in `doctor --json`.

---

## 8. Command surface

The complete v1.0 surface, grouped ferraus-style (`pixelox <group> <command> …`). Every command: accepts `--json`; mutating commands accept `--dry-run` and write via `-o/--output` or (for `.pxo` inputs) `--in-place`; every written output gets a sidecar. Milestone tags appear where a group lands after v0.1 (§12 has the full schedule).

### 8.1 Conventions & addressing

```
pixelox <group> <command> [INPUT] [flags]
```

| Convention | Rule |
|---|---|
| `INPUT` | An image file (JPEG/PNG/…/PSD/SVG) or a `.pxo` document. Non-`.pxo` inputs are loaded into an implicit single-layer document named `Background` — every command works one-shot on plain images. |
| Output | `-o, --output <path>` (format inferred from extension; `--format` overrides). Mutating a `.pxo` input requires `--in-place` instead. Exactly one of the two for mutating commands. |
| Overwrite | Existing paths require `--overwrite` (§6.3). |
| Layers | `--layer <ref>`: unique layer name, `#N` (1-based from top), `@top`, `@bottom`, `@active`. Default for single-layer docs: that layer; multi-layer docs: required where ambiguous (`PX2302`). `--layers a,b,c` where plural. |
| Masks | `--mask <ref>`: named mask, `@sel` (active selection), `@alpha:<layer>`, or a grayscale image path. `--invert-mask` complements it. Omitted ⇒ op applies to the full region. |
| Geometry | `--rect X,Y,WxH`, `--at X,Y`, `--size WxH` (one dimension may be blank to preserve aspect: `2048x`), `--anchor tl|t|tr|l|c|r|bl|b|br`. Units: px default; `%` of canvas; `pt/mm/in` where DPI-meaningful. Negative X,Y measured from right/bottom. |
| Color | `#RRGGBB[AA]`, `rgb(…)`, `hsl(…)`, CSS named colors, `transparent`. |
| Amounts | Photoshop-familiar ranges: opacities/densities 0–100, exposure in stops, hue ±180, etc. Documented per command in `docs`. |
| Global flags | `--json`, `--dry-run`, `--overwrite`, `--in-place`, `--seed <u64|random>`, `-y/--yes`, `-q/--quiet`, `-v/--verbose`, `--threads N`, `--mem-budget MB`, `--progress bar|json|none`, `--no-sidecar`, `--background`, `--config <path>`. |

### 8.2 Documents & files — `new`, `open`, `save`, `export`, `info`

| Command & key flags | Does |
|---|---|
| `new -o doc.pxo --size 1080x1350 [--fill COLOR] [--dpi N] [--color-space srgb\|display-p3\|adobe-rgb] [--depth 8\|16]` | Create an empty document. |
| `new -o doc.pxo --preset <name>` | Create from a canvas preset (§8.14 table): `a4`, `letter`, `a3`, `ig-post`, `ig-portrait`, `ig-story`, `x-post`, `yt-thumb`, `og-card`, `fb-cover`, `business-card`, … |
| `open IN [-o doc.pxo] [--page N] [--frame N] [--icc honor\|assign\|ignore]` | Load any readable format; with `-o`, convert to `.pxo` (PSD layers preserved per §7.6.1). Without `-o`, acts as inspect: prints document summary. |
| `info IN [--json]` | Read-only summary: size, color space, depth, layer tree, masks, fonts used, EXIF presence. |
| `save doc.pxo --in-place [--no-cache]` | Normalize/garbage-collect and rewrite a document (drops orphaned assets, optionally caches). |
| `export IN -o OUT [--format F] [--quality 1..100] [--bit-depth 8\|16] [--scale X\|--size WxH] [--layer REF] [--apply-mask REF] [--flatten] [--strip-metadata] [--profile P] [--region X,Y,WxH]` | Render and write a deliverable. `--apply-mask` multiplies into output alpha (the cutout path). `--layer` exports one layer/group. PNG supports `--bit-depth 16`; JPEG `--progressive`; TIFF `--compression lzw\|deflate\|none`; PSD `--layered-basic`. |
| `export IN --ladder 2048,1024,512 --formats png,webp -o outdir/` | Export a size/format ladder in one pass (one decode, N encodes). |

### 8.3 `layer` — the layer stack

| Command & key flags | Does |
|---|---|
| `layer list [--tree]` | List layers: id, name, kind, blend, opacity, mask, fx, bounds. |
| `layer add [--name N] [--size WxH] [--at X,Y] [--fill COLOR]` | New empty/solid raster layer. |
| `layer dup --layer REF [--name N]` | Duplicate a layer (with mask + fx). |
| `layer rm --layer REF` | Remove layer (document op, undoable via history; never touches files). |
| `layer rename --layer REF --to NAME` | Rename (names stay unique). |
| `layer reorder --layer REF (--above REF2\|--below REF2\|--index N)` | Restack. |
| `layer group --layers a,b,c --name G` / `layer ungroup --layer G` | Group/ungroup; groups default to `pass-through`. |
| `layer merge --layers a,b [--name N]` / `layer merge --down --layer REF` | Merge to one raster layer (bakes ops/fx). |
| `layer flatten` | Flatten entire document to `Background`. |
| `layer opacity --layer REF --value 0..100` | Set opacity. |
| `layer blend --layer REF --mode <mode>` | Set blend mode — full set: `normal dissolve darken multiply color-burn linear-burn darker-color lighten screen color-dodge linear-dodge lighter-color overlay soft-light hard-light vivid-light linear-light pin-light hard-mix difference exclusion subtract divide hue saturation color luminosity pass-through` (§14.3). |
| `layer show\|hide --layer REF` / `layer lock\|unlock --layer REF` | Visibility & lock. |
| `layer clip --layer REF [--off]` | Toggle clipping mask onto the layer below (Photoshop semantics). |
| `layer rasterize --layer REF` | Convert text/shape/placed → raster. |

### 8.4 `select` — selections (soft, 8-bit coverage)

All selection commands take `--mode replace|add|subtract|intersect` (default `replace`) — the boolean ops.

| Command & key flags | Does |
|---|---|
| `select rect --rect X,Y,WxH [--feather R] [--mode M]` | Rectangular selection. |
| `select ellipse --rect X,Y,WxH [--feather R]` | Elliptical (inscribed in rect). |
| `select poly --points x1,y1 x2,y2 … [--feather R]` | Polygon from vertices. |
| `select path (--d 'M…Z' \| --points-file pts.json) [--feather R]` | Freehand/Bézier region from SVG path data or a points file. |
| `select color --ref COLOR [--at X,Y] [--tolerance 0..255] [--contiguous]` | Color-range selection (magic-wand with `--contiguous --at`). |
| `select luma --range 0.0..1.0 [--channel l\|r\|g\|b\|a]` | Luminosity/channel range selection. |
| `select all` / `select none` / `select invert` | The classics. |
| `select save --as NAME` | Store active selection in the named-mask library. |
| `select load NAME [--mode M]` | Named mask → active selection (boolean-combinable). |
| `select bounds [--json]` | Report selection bbox, area, coverage %. |

### 8.5 `mask` — named masks & layer masks

Refinement commands accept any mask ref including `@sel`, so selection refinement and mask refinement are one surface.

| Command & key flags | Does |
|---|---|
| `mask list` | Named masks + per-layer masks, with coverage stats. |
| `mask import FILE --as NAME` / `mask export --mask REF -o out.png` | Grayscale image ↔ named mask. |
| `mask rm --mask NAME` | Remove from library. |
| `mask add --layer REF (--from @sel\|--from NAME\|--from-file m.png\|--reveal-all\|--hide-all)` | Attach a **layer mask** (non-destructive per-layer alpha, Photoshop semantics). `--from-path 'M…Z'` attaches a **vector mask** **[v0.3]**. |
| `mask remove --layer REF [--apply]` | Delete layer mask, or `--apply` bake into layer alpha. |
| `mask enable\|disable --layer REF` | Toggle layer mask without deleting. |
| `mask density --layer REF --value 0..100` / `mask density --mask NAME --value …` | Mask density (max opacity of the mask effect). |
| `mask feather --mask REF --radius Rpx` | Gaussian feather. |
| `mask expand --mask REF --by Npx` / `mask contract --mask REF --by Npx` | Morphological grow/shrink. |
| `mask smooth --mask REF --radius Rpx` | Median-smooth jagged edges. |
| `mask invert --mask REF` | Complement. |
| `mask refine --mask REF --model vitmatte [--band Npx]` | **Edge refinement / hair-fur matting**: trimap band around the mask edge re-solved by the matting model **[v0.2]**. |

### 8.6 `detect` — AI-assisted element identification **[v0.2]**

Every `detect` command returns structured detections in the envelope (class, score, bbox, polygon/mask ref) and can materialize results four ways: `--save-mask NAME[,…]` (document mask library), `--select` (active selection), `--mask-out out.png` (grayscale mask file — the one-shot path), `--annotate out.png` (debug overlay). Box-producing detectors (`objects`, `faces`, `text`) materialize masks by auto-chaining their boxes through the segmenter — recorded as two ops in the recipe.

| Command & key flags | Does |
|---|---|
| `detect objects [--prompt "text"] [--classes dog,car] [--min-score 0.5]` | Object detection (YOLOX-s); `--prompt` uses open-vocab model **[stretch]**. |
| `detect subject [--matte] [--cutout out.png]` | Primary subject / background separation (BiRefNet-lite) → soft mask. `--matte` refines edges via ViTMatte; `--cutout` writes the RGBA cutout directly. |
| `detect salient [--fast]` | Salient-region map (u2netp fast path). |
| `detect faces [--landmarks]` | Faces + 5-point landmarks (YuNet). |
| `detect text [--recognize] [--lang en]` | OCR text regions (+ recognition **[stretch]**) → boxes/polygons, strings. |
| `detect segment (--point X,Y … \| --box X,Y,WxH \| --prompt "the red car")` | Promptable segmentation (MobileSAM): point/box prompts; text prompt routes through open-vocab detect **[stretch]**. |
| `detect list-models` | Which models would serve which command, present or not (alias into `models list --for detect`). |

### 8.7 `adjust` — tonal & color adjustments

All `adjust` commands: apply destructively to `--layer` by default, or **`--as-layer [--name N]` creates an adjustment layer** (masked by `--mask`, reorderable, non-destructive). All are 8/16-bit-safe, f32 math.

| Command & key flags | Does |
|---|---|
| `adjust brightness-contrast --brightness -100..100 --contrast -100..100` | The basic pair. |
| `adjust levels [--channel rgb\|r\|g\|b] --in-black N --in-white N [--gamma G] [--out-black N] [--out-white N]` | Levels. |
| `adjust curves (--points '0,0 64,80 255,255' \| --curve file.json) [--channel …]` | Curves (monotone cubic through points). |
| `adjust exposure --ev -5..5 [--offset] [--gamma]` | Exposure in stops (linear-light). |
| `adjust shadows-highlights --shadows 0..100 --highlights 0..100 [--radius R]` | Shadow lift / highlight recovery (the owner's "lightening"). |
| `adjust tone-map [--strength 0..100]` | Local HDR-ish tone mapping. |
| `adjust hsl [--range master\|reds\|yellows\|…] --hue ±180 --saturation ±100 --lightness ±100` | Hue/Saturation incl. per-color ranges; `--colorize`. |
| `adjust color-balance --shadows CR,MG,YB --midtones … --highlights … [--preserve-luma]` | Color balance. |
| `adjust vibrance --vibrance ±100 [--saturation ±100]` | Vibrance. |
| `adjust white-balance (--auto \| --temp ±100 --tint ±100 \| --from-point X,Y)` | Temperature/tint; gray-point pick. |
| `adjust selective-color --target reds\|…\|neutrals --c ±100 --m ±100 --y ±100 --k ±100 [--relative]` | Selective color. |
| `adjust channel-mixer --out r\|g\|b --r % --g % --b % [--constant %] [--monochrome]` | Channel mixer. |
| `adjust gradient-map --stops '#000@0,#ff8800@0.5,#fff@1' [--interp oklab]` | Gradient map. |
| `adjust lut --cube file.cube [--strength 0..100]` | Apply .cube 3D LUT. |
| `adjust bw [--preset red-filter\|…] [--r % --g % --b % …]` | Black & white conversion with channel weights. |
| `adjust invert` / `adjust equalize` | Invert; histogram equalize. |
| `adjust posterize --levels 2..255` / `adjust threshold --level 1..255` | Posterize; threshold. |
| `adjust auto (--levels\|--contrast\|--color\|--tone)` | Auto corrections (per-channel endpoints, contrast stretch, cast removal, combined). |

### 8.8 `filter` — convolution, blur, sharpen, noise, stylize

All filters honor `--layer`, `--mask/--invert-mask`, and `--linear` where physically meaningful.

| Command & key flags | Does |
|---|---|
| `filter gaussian --sigma S` | Gaussian blur (separable; IIR at large σ). |
| `filter box --radius R [--passes N]` | Box blur. |
| `filter motion-blur --angle DEG --distance PX` | Linear motion blur. |
| `filter radial-blur --kind spin\|zoom --amount N [--center X,Y]` | Spin/zoom blur. |
| `filter lens-blur (--depth map.png\|--depth auto) --radius R [--shape hex\|circle] [--highlights N]` | Depth-aware lens blur (bokeh); `--depth auto` estimates via subject mask **[v0.2]**. |
| `filter surface-blur --radius R --threshold N` | Edge-preserving (bilateral). |
| `filter median --radius R` | Median (constant-time histogram). |
| `filter unsharp --amount % --radius R --threshold N` | Unsharp mask. |
| `filter smart-sharpen --amount % --radius R [--reduce-noise N] [--kind lens\|gaussian\|motion --angle DEG]` | Smart sharpen with halo suppression. |
| `filter add-noise --amount % [--kind gaussian\|uniform] [--monochrome]` | Seeded noise. |
| `filter denoise --strength N [--preserve-details N]` | Noise reduction (edge-aware NLM-class). |
| `filter despeckle` / `filter dust --radius R --threshold N` | Despeckle; dust & scratches. |
| `filter emboss --angle DEG --height PX --amount %` | Emboss. |
| `filter find-edges` | Edge detection (Sobel-class, PS-style inverted). |
| `filter high-pass --radius R` | High pass (frequency-separation workflows). |
| `filter pixelate --cell PX [--kind square\|hex]` | Mosaic/pixelate. |
| `filter vignette --amount ±100 [--midpoint N] [--roundness N]` | Add (+) or remove (−) vignette. |
| `filter lens-correct [--distortion ±100] [--ca R,B] [--vignette ±100]` | Simple lens corrections: barrel/pincushion, chromatic aberration, vignetting. |

### 8.9 `retouch` & `fill` — repair, paint-class region ops

| Command & key flags | Does |
|---|---|
| `retouch erase (--mask REF\|--rect …) [--layer REF]` | Set region alpha to 0 (respects layer mask semantics via `--to-mask`). |
| `retouch heal (--at X,Y --radius R \| --mask REF)` | Spot heal: texture-synthesizing small-region repair (clone + blend). |
| `retouch clone --from X,Y --to X,Y --radius R [--opacity %]` | Clone stamp between regions. |
| `retouch dodge --mask REF --exposure 0..1 [--range shadows\|midtones\|highlights]` | Dodge as region op. |
| `retouch burn --mask REF --exposure 0..1 [--range …]` | Burn as region op. |
| `retouch sharpen --mask REF --amount %` / `retouch blur --mask REF --sigma S` | Brush-equivalent regional sharpen/blur. |
| `retouch redeye (--auto \| --at X,Y) [--strength %]` | Red-eye removal (`--auto` uses `detect faces`). |
| `fill solid --color C [--mask REF] [--preserve-alpha]` | Solid fill. |
| `fill gradient --kind linear\|radial\|angular\|reflected --from X,Y --to X,Y --stops '#000@0,#fff@1' [--interp perceptual\|linear\|oklab] [--dither]` | Gradient fill (all four geometries). |
| `fill pattern --from tile.png [--scale %] [--angle DEG]` | Pattern fill. |
| `fill content-aware --mask REF [--model lama] [--expand Npx]` | **Content-aware fill / inpainting** (LaMa-class) **[v0.2]**; alias `fill inpaint`. |

### 8.10 `text` — type layers

| Command & key flags | Does |
|---|---|
| `text add --content "…" --font FAMILY [--size 48pt] [--weight 100..900\|bold] [--style italic] [--variation 'wght=650,slnt=-5'] [--color C] [--stroke C --stroke-width W] [--align left\|center\|right\|justify] [--tracking N] [--leading N\|auto] [--kerning metrics\|none] [--at X,Y --anchor A] [--box WxH] [--name N]` | New text layer. `--box` enables paragraph wrapping; variable-font axes via `--variation`. Shaping is HarfBuzz-class: bidi/RTL, CJK, ligatures. |
| `text edit --layer REF [--content "…"] [any style flag]` | Re-set content and/or styles; text layers stay live (non-destructive). |
| `text style --layer REF <style flags>` | Styling-only alias of `edit`. |
| `text on-path --layer REF (--d 'M…' \| --follow SHAPELAYER) [--offset %] [--side inside\|outside]` | Flow text along a Bézier path or an existing shape layer. |
| `text to-path --layer REF` | Convert glyphs to a vector shape layer (enables `path *` booleans, outline fx). |
| `text list [--fonts]` | Text layers and the fonts they use. |

### 8.11 `shape` & `path` — vector layers on the canvas

Shapes create vector shape layers; common paint flags: `--fill C`, `--stroke C`, `--stroke-width W`, `--dash '4,2'`, `--opacity`, `--name`.

| Command & key flags | Does |
|---|---|
| `shape rect --rect X,Y,WxH [--radius R\|'tl,tr,br,bl']` | Rectangle (rounded via radius). |
| `shape ellipse --rect X,Y,WxH` | Ellipse/circle. |
| `shape polygon --center X,Y --radius R --sides N [--rotation DEG]` | Regular polygon. |
| `shape star --center X,Y --points N --outer R --inner R2` | Star. |
| `shape line --from X,Y --to X,Y [--cap butt\|round\|square]` | Line. |
| `shape arrow --from X,Y --to X,Y [--head-size N] [--tail-head]` | Arrow. |
| `shape path --d 'M 10 10 C …' ` | Arbitrary Bézier path layer (SVG path data). |
| `shape set --layer REF [paint flags]` | Restyle an existing shape layer. |
| `path unite\|subtract\|intersect\|exclude --layers A,B [--into NAME]` | Boolean ops between shape layers (result replaces or `--into` news) **[v0.3]**. |
| `path simplify --layer REF --tolerance T` | Reduce anchor count **[v0.3]**. |
| `path outline-stroke --layer REF` | Convert stroke to filled path **[v0.3]**. |

### 8.12 `transform` — geometry (layers and document)

Layer-scoped when `--layer` given; document-scoped commands noted. `--interp nearest|bilinear|bicubic|lanczos3` (default lanczos3 for down, bicubic for up); `--expand` grows bounds instead of clipping.

| Command & key flags | Does |
|---|---|
| `transform move --layer REF (--to X,Y \| --by DX,DY) [--anchor A]` | Move. |
| `transform scale --layer REF (--to 50% \| --size WxH) [--anchor A]` | Scale. |
| `transform rotate [--layer REF] --degrees D [--expand] [--around X,Y]` | Rotate layer, or whole canvas (90/180/270 lossless fast path). |
| `transform flip [--layer REF] --h\|--v` | Flip. |
| `transform skew --layer REF --x DEG --y DEG` | Skew/shear. |
| `transform perspective --layer REF --quad 'x1,y1 x2,y2 x3,y3 x4,y4'` | Map layer corners to a quad. |
| `transform warp --layer REF --grid 3x3 --points warp.json` | Grid-mesh warp (control points file; `pixelox schema transform.warp` documents it). |
| `transform crop --rect X,Y,WxH` / `--auto trim` / `--ratio 4:5 [--gravity A]` | Document crop. `--content-aware` (seam-carving retarget) **[stretch]**. |
| `transform straighten (--angle auto\|DEG) [--crop auto\|none]` | Horizon/edge straighten + auto-crop. |
| `transform canvas --size WxH\|'+200x+0' [--anchor A] [--fill COLOR\|transparent]` | Canvas resize (no resample). |
| `transform resize --size WxH [--interp F] [--sharpen auto]` | Document resample with quality filters (nearest/bilinear/bicubic/lanczos3). |
| `transform trim [--threshold N]` | Auto-crop uniform/transparent border. |

### 8.13 `fx` — layer styles (Photoshop "fx") **[v0.2]**

Non-destructive style stack per layer; rendered live, scale with `fx scale`. Common flags: `--blend MODE`, `--opacity %`.

| Command & key flags | Does |
|---|---|
| `fx drop-shadow --layer REF --offset DX,DY --blur R [--spread %] [--color C]` | Drop shadow. |
| `fx inner-shadow --layer REF --offset DX,DY --blur R [--choke %] [--color C]` | Inner shadow. |
| `fx outer-glow --layer REF --blur R [--spread %] [--color C\|--stops …]` | Outer glow. |
| `fx inner-glow --layer REF --blur R [--choke %] [--source edge\|center]` | Inner glow. |
| `fx bevel --layer REF --style inner\|outer\|emboss\|pillow --depth % --size R [--soften N] [--angle DEG --altitude DEG]` | Bevel & emboss (incl. contour/texture **[stretch]**). |
| `fx satin --layer REF --offset DX,DY --blur R [--color C] [--invert]` | Satin. |
| `fx overlay --layer REF --kind color\|gradient\|pattern (--color C \| --stops … --angle DEG \| --from tile.png --scale %)` | Color / gradient / pattern overlay. |
| `fx stroke --layer REF --width W --position inside\|center\|outside (--color C\|--stops …\|--from tile.png)` | Stroke. |
| `fx list --layer REF` / `fx rm --layer REF [--kind K\|--all]` | Inspect / remove. |
| `fx copy --from REF --to REF[,REF…]` | Copy style stack between layers. |
| `fx scale --layer REF --by %` | Scale all effect geometry. |

### 8.14 `comp` — composition, templates, deliverables

| Command & key flags | Does |
|---|---|
| `comp place FILE [--as NAME] [--at X,Y --anchor A] [--size WxH\|--fit cover\|contain] [--link\|--embed]` | Import a file as a new (smart-object-like) placed layer; `--link` keeps external reference. |
| `comp relink --layer REF --to FILE` / `comp update-links [--check]` | Manage linked sources; `--check` reports drift via hashes. |
| `comp align --layers a,b,c --x left\|center\|right --y top\|middle\|bottom [--to canvas\|selection\|layer:REF]` | Align layers. |
| `comp distribute --layers a,b,c --axis x\|y [--spacing N\|even]` | Distribute. |
| `comp guide add (--v POS\|--h POS)` / `comp guide list\|rm\|clear` | Guides (snapping targets for place/align; positions in px or %). |
| `comp grid set --size N[,N2] [--off]` | Document grid. |
| `comp template init [--slot text:headline:required …]` | Declare template slots on a document: typed (`text`, `image`, `color`), required/optional, fit rules for images (`cover|contain|fill`), constraints (max lines, min font size with auto-shrink). |
| `comp template slots TEMPLATE.pxo [--json]` | List a template's slots and constraints. |
| `comp template apply TEMPLATE.pxo --set headline="…" --set-image product=shot.jpg [--set-color accent='#0ea5e9'] -o out.pxo` | Fill slots → finished document. Missing required slot ⇒ `PX3202`. |
| `comp presets [--json]` | List canvas presets (below). |

Canvas presets (extensible via config):

| Preset | Pixels | DPI | Preset | Pixels | DPI |
|---|---|---|---|---|---|
| `a4` | 2480×3508 | 300 | `ig-post` | 1080×1080 | 72 |
| `a3` | 3508×4961 | 300 | `ig-portrait` | 1080×1350 | 72 |
| `letter` | 2550×3300 | 300 | `ig-story` | 1080×1920 | 72 |
| `business-card` | 1050×600 | 300 | `x-post` | 1600×900 | 72 |
| `poster-18x24` | 2700×3600 | 150 | `yt-thumb` | 1280×720 | 72 |
| `flier-letter` | 2550×3300 | 300 | `og-card` | 1200×630 | 72 |
| | | | `fb-cover` | 820×312 | 72 |

### 8.15 `svg` — vector authoring (the Illustrator/Fireworks analog) **[v0.3]**

Operates on standalone SVG files (scene-graph in, SVG out) — the full SVG 1.1 core authoring surface: paths, shapes, groups, transforms, gradients, patterns, clip paths, masks, filters, symbols/`use`, text. Elements addressed by `--id` (CSS-style selectors in `--where` for bulk ops).

| Command & key flags | Does |
|---|---|
| `svg new -o f.svg --size WxH [--viewbox …]` | New SVG document. |
| `svg el add f.svg --in-place --kind rect\|circle\|ellipse\|line\|polyline\|polygon\|path\|text\|g\|image --id ID [--parent ID] [--attr k=v …] [--d '…'] [--text "…"]` | Add any element with attributes (presentation attrs incl. `transform`, `fill`, `stroke`, `stroke-dasharray`, …). |
| `svg el set f.svg --in-place (--id ID\|--where 'g.axis text') --attr k=v … [--text "…"]` | Set attributes/content on matching elements. |
| `svg el rm f.svg --in-place --id ID\|--where SEL` | Remove elements. |
| `svg defs add f.svg --in-place --kind linear-gradient\|radial-gradient\|pattern\|clip-path\|mask\|filter\|symbol\|marker --id ID --spec spec.json` | Add defs; filter specs compose SVG primitives (`feGaussianBlur`, `feDropShadow`, `feColorMatrix`, `feTurbulence`, `feDisplacementMap`, full list §7.9). Mesh gradients **[stretch]**. |
| `svg use f.svg --in-place --ref '#sym' --at X,Y [--id ID]` | Instance a symbol. |
| `svg query f.svg --where SEL [--json]` | Inspect matching elements (tag, attrs, computed bbox). |
| `svg optimize f.svg -o min.svg [--precision N]` | Normalize + minify (svgo-class, conservative defaults). |
| `svg rasterize f.svg -o out.png [--size WxH\|--scale X] [--background C]` | Render via resvg. |
| `svg from-doc doc.pxo -o out.svg [--vector-only]` | Export document vector/text/shape layers as structured SVG; raster layers embedded as images unless `--vector-only`. |

### 8.16 `render` — generative JS and 3D-lite **[v0.4]**

| Command & key flags | Does |
|---|---|
| `render js script.js -o out.(svg\|png) --size WxH [--data name=file.csv …] [--param k=v …] [--lib extra.js] [--format svg\|png] [--timeout S] [--js-mem MB]` | Run a sandboxed script (§7.10): script builds via `px.canvas()` / `px.svg()` and calls `px.output(...)`. |
| `render d3 script.js -o chart.svg --data rows=sales.csv [--size 1200x800]` | `render js` preset with vendored d3 + DOM-lite preloaded; script receives `d3` global. |
| `render three script.js -o scene.svg\|png [--size …]` | Preset with vendored three.js; `SVGRenderer` for vector output, `PxCanvas` raster path for shaded output. |
| `render 3d (--scene scene.p3d.json \| --primitive box --size 2,1,1 \| --extrude logo.svg --depth 40) -o out.(svg\|png) --camera iso\|persp [--fov N --look-at X,Y,Z] --style wireframe\|hidden-line\|shaded [--line-width W] [--as-layer DOC --in-place]` | Native 3D-lite (§7.11): primitives, SVG-profile extrusions, OBJ/glTF import via scene file. Not CAD. |

### 8.17 `trace` — bitmap → vector **[v0.3]**

| Command & key flags | Does |
|---|---|
| `trace IN -o out.svg [--mode color\|binary] [--colors N] [--threshold N] [--speckle N] [--corner N] [--precision N]` | Potrace-class tracing (pure Rust): binary (single-color silhouette) or multi-color posterized tracing; speckle/corner/precision tune fidelity. Centerline mode **[stretch]**. |

### 8.18 `frames` — multi-frame images (animated GIF, multi-page TIFF)

| Command & key flags | Does |
|---|---|
| `frames extract IN -o dir/ [--frames 1..10\|all] [--pattern 'frame-{n:04}.png']` | Explode animated GIF / multi-page TIFF to stills (+ `frames.json` manifest with delays). |
| `frames assemble 'dir/frame-*.png' -o out.gif --fps 12 [--loop N\|forever] [--colors 256] [--dither fs\|ordered\|none]` | Build animated GIF from stills (seeded quantizer + dithering). |

### 8.19 `analyze` — measurement & verification

| Command & key flags | Does |
|---|---|
| `analyze histogram IN [--channel rgb\|r\|g\|b\|l\|a] [--bins 256]` | Histogram (JSON bins + text sparkline). |
| `analyze stats IN [--layer REF] [--rect X,Y,WxH]` | Min/max/mean/median/stddev per channel, clipping %, alpha coverage; `--rect` samples a region (1×1 = eyedropper). |
| `analyze colors IN --k 5 [--space oklab]` | Dominant colors via seeded k-means: hex, share %, swatch SVG via `-o`. |
| `analyze exif IN [--raw]` | EXIF/XMP/ICC metadata dump. |
| `analyze diff A B [--threshold N] [--out heatmap.png]` | Perceptual comparison: SSIM, ΔE00 mean/p95/p99, changed-pixel count/%, optional heatmap — the agent self-verification tool. |

### 8.20 `batch`, `pipeline`, `jobs`

| Command & key flags | Does |
|---|---|
| `batch run --glob 'in/*.jpg' (--pipeline p.yml \| --cmd 'export {in} -o out/{stem}.webp --quality 80') --out-dir out/ [--jobs N] [--continue-on-error] [--manifest out/manifest.json]` | Apply a pipeline or command template per matched file. Template vars: `{in} {stem} {ext} {dir} {n}`. Per-item envelopes stream as NDJSON; summary envelope at end; nonzero exit if any item failed (unless `--continue-on-error`, which still reports failures). |
| `pipeline run p.yml [--var k=v …] [--dry-run]` | Execute a pipeline file (§9) in one process. `--dry-run` prints the resolved plan/DAG. |
| `pipeline validate p.yml` | Schema + reference checks (files, fonts, models, slots) without running. |
| `pipeline new -o p.yml [--from-sidecar out.png.pixelox.json]` | Scaffold a pipeline — including **from a sidecar recipe** (provenance → reusable pipeline). |
| `pipeline graph p.yml [--format dot]` | Print the step DAG. |
| `jobs list [--all]` / `jobs status ID` / `jobs wait ID [--timeout S]` / `jobs cancel ID` / `jobs result ID` | Manage `--background` jobs (§7.13): wait blocks until done and exits with the job's exit code; `result` replays its final envelope. |

### 8.21 Models, fonts, config, meta

| Command & key flags | Does |
|---|---|
| `models list [--for detect]` / `models pull NAME [-y]` / `models rm NAME` / `models verify [NAME]` | Registry management: list (installed/available, sizes, licenses), download (SHA-256 verified, consent-gated), remove, re-verify hashes. |
| `models add --from file.onnx --task segment --name custom-x` | Register a user model (own license risk; recorded in sidecars). |
| `fonts list [--family F]` / `fonts add DIR` | System + registered fonts (fontdb); add search dirs. Embedded fallback (Noto Sans subset, OFL) guarantees text works on bare systems. |
| `config get K` / `config set K V` / `config list` / `config path` | Configuration (§7.13 precedence). |
| `schema <command>\|--all\|sidecar\|pipeline` | JSON Schemas (§6.2). |
| `describe [--agent\|--json]` | Surface overview / agent briefing / machine index (§6.2). |
| `docs <command\|topic>` | Offline reference (§6.2). |
| `doctor [--json] [--migrate-pxo FILE]` | Environment diagnostics: build features, codecs, models present, fonts found, config effective values, state dirs, known issues. |
| `replay SIDECAR -o OUT [--strict]` / `verify SIDECAR [--fast\|--full]` | Provenance (§6.5). |
| `mcp serve --allow DIR [--allow DIR2 …] [--read-only] [--allow-model-downloads]` | MCP server over stdio (§10). |
| `self-update [--channel stable] [--check]` | Signed self-update (minisign-verified; disabled in distro-packaged builds). |
| `completions SHELL` / `version [--json]` | Shell completions; version/build/features. |

### 8.22 Worked examples

Twelve end-to-end workflows. Every one of these is shipped under `examples/` and executed in CI against golden outputs. All assume the flag conventions of §8.1; `$` prompts omitted.

#### E1 — Remove a background, export transparent PNG

```bash
# One-shot (the 95% case):
pixelox detect subject shot.jpg --matte --cutout cutout.png --json

# Or through the document model, when you intend to keep editing:
pixelox open shot.jpg -o shot.pxo
pixelox detect subject shot.pxo --in-place --save-mask subject
pixelox mask refine shot.pxo --in-place --mask subject --model vitmatte --band 16
pixelox export shot.pxo -o cutout.png --apply-mask subject
```

#### E2 — Detect the dog, feather the mask 12 px, blur everything else

```bash
pixelox open walk.jpg -o walk.pxo
# `detect objects --save-mask` auto-chains the dog's box through the segmenter (two ops in the recipe):
pixelox detect objects walk.pxo --in-place --classes dog --min-score 0.5 --save-mask dog
pixelox mask feather walk.pxo --in-place --mask dog --radius 12
pixelox filter gaussian walk.pxo --in-place --sigma 8 --mask dog --invert-mask
pixelox export walk.pxo -o walk-dreamy.jpg --quality 90
```

#### E3 — Remove power lines with content-aware fill (plan first)

```bash
pixelox open street.jpg -o street.pxo
pixelox select path street.pxo --in-place --d 'M 120,40 L 2900,210 L 2900,260 L 120,90 Z'
pixelox select save street.pxo --in-place --as wires
pixelox mask expand street.pxo --in-place --mask wires --by 6

# Plan: what would happen, which model is needed, is it installed?
pixelox fill content-aware street.pxo --mask wires --dry-run --json
```

Dry-run envelope (abridged) — the agent reads the plan, sees the consent requirement, then commits:

```json
{ "ok": true, "dry_run": true, "command": "fill.content-aware",
  "result": { "plan": {
    "ops": [ { "op": "fill.content-aware", "params": { "mask": "wires", "model": "lama" } } ],
    "models_required": [ { "name": "lama", "installed": false, "download_bytes": 207904768,
                           "remedy": "pixelox models pull lama -y" } ],
    "consent_required": ["model-download"],
    "outputs_predicted": [ { "path": "street.pxo", "action": "mutate-in-place" } ] } } }
```

```bash
pixelox fill content-aware street.pxo --in-place --mask wires -y
pixelox export street.pxo -o street-clean.jpg --quality 92
```

#### E4 — Rescue an underexposed interior, grade it, export 16-bit TIFF

```bash
pixelox open interior.jpg -o shot.pxo
pixelox adjust white-balance    shot.pxo --in-place --auto
pixelox adjust shadows-highlights shot.pxo --in-place --shadows 45 --highlights 20
pixelox adjust curves           shot.pxo --in-place --points '0,8 96,118 255,246'
pixelox adjust lut              shot.pxo --in-place --cube luts/teal-orange.cube --strength 40
pixelox export shot.pxo -o interior-final.tiff --bit-depth 16
pixelox analyze stats interior-final.tiff --json          # verify: no clipped channels
```

#### E5 — Build a poster from a template: headline + product shot

```bash
# The template (poster-a3.pxo) declares slots: text:headline, image:product (fit=cover), color:accent.
pixelox comp template slots poster-a3.pxo --json

pixelox detect subject hero.jpg --matte --cutout work/hero-cut.png
pixelox comp template apply poster-a3.pxo \
  --set headline="FALL DROP 09" \
  --set-image product=work/hero-cut.png \
  --set-color accent='#0ea5e9' \
  -o work/poster.pxo
pixelox export work/poster.pxo -o dist/poster-a3.png
```

(§9's pipeline file runs this whole build — including the cutout — as one reproducible unit.)

#### E6 — A social-card series from a CSV

```yaml
# social-cards.yml (abridged; full spec in §9)
version: 1
steps:
  - id: cards
    foreach: { csv: "${vars.csv}", as: row }
    run: comp.template.apply
    with:
      input: card.pxo
      set:       { name: "${row.name}", title: "${row.title}" }
      set-image: { headshot: "${row.photo}" }
      output: "dist/cards/${row.slug}.png"
```

```bash
pixelox pipeline run social-cards.yml --var csv=speakers.csv --json
```

#### E7 — Batch-watermark 500 images

```yaml
# watermark.yml — multi-step per item, so a pipeline (not --cmd) is the right tool
version: 1
steps:
  - id: wm
    run: comp.place
    with: { input: "${item.in}", file: brand/logo.png, as: Watermark,
            anchor: br, at: "-48,-48", size: "220x" }
  - id: blend
    run: layer.blend
    with: { layer: Watermark, mode: multiply }
    needs: [wm]
  - id: fade
    run: layer.opacity
    with: { layer: Watermark, value: 85 }
    needs: [blend]
  - id: out
    run: export
    with: { output: "${item.out}", quality: 92, strip-metadata: false }
    needs: [fade]
```

```bash
pixelox batch run --glob 'shoot/*.jpg' --pipeline watermark.yml \
  --out-dir out/ --jobs 8 --manifest out/manifest.json --progress json
```

#### E8 — Open a layered PSD, replace the headline, re-export

```bash
pixelox open brand.psd -o brand.pxo --json      # layer tree preserved per §7.6.1
pixelox info brand.pxo --json | jq '.result.layers[] | select(.name=="Headline")'
# PSD text imports as raster + preserved type metadata; re-set it as live text:
pixelox layer rm brand.pxo --in-place --layer Headline
pixelox text add brand.pxo --in-place --name Headline --content "FALL DROP 09" \
  --font "Archivo Black" --size 64pt --color '#101014' --at 50%,120 --anchor t --align center
pixelox export brand.pxo -o fall.png
pixelox export brand.pxo -o fall.psd --layered-basic
```

#### E9 — d3 bar chart from CSV → SVG and a 2× PNG

```js
// chart.js — runs in the pixelox sandbox; `d3` and the DOM-lite are preloaded by `render d3`
const rows = px.data.rows;                                  // --data rows=sales.csv, pre-parsed
const doc  = px.svg(960, 540);
const sel  = d3.select(doc.documentElement);
const x = d3.scaleBand().domain(rows.map(r => r.month)).range([60, 920]).padding(0.2);
const y = d3.scaleLinear().domain([0, d3.max(rows, r => +r.sales)]).nice().range([500, 40]);
sel.selectAll('rect').data(rows).join('rect')
   .attr('x', r => x(r.month)).attr('y', r => y(+r.sales))
   .attr('width', x.bandwidth()).attr('height', r => 500 - y(+r.sales))
   .attr('fill', '#0ea5e9');
sel.append('g').attr('transform', 'translate(60,0)').call(d3.axisLeft(y));
sel.append('g').attr('transform', 'translate(0,500)').call(d3.axisBottom(x));
px.output(doc);
```

```bash
pixelox render d3 chart.js --data rows=sales.csv -o chart.svg --size 960x540 --seed 7
pixelox svg rasterize chart.svg -o chart@2x.png --scale 2
```

#### E10 — 3D wireframe of an extruded logo

```bash
pixelox render 3d --extrude brand/logo.svg --depth 40 \
  --camera iso --style hidden-line --line-width 1.5 -o logo-wire.svg
pixelox svg rasterize logo-wire.svg -o logo-wire.png --size 1600x --background '#ffffff'
# or straight into a document as a layer:
pixelox render 3d --extrude brand/logo.svg --depth 40 --camera iso --style hidden-line \
  --as-layer deck.pxo --in-place
```

#### E11 — A sticker: star burst, text on a circular path, layer styles

```bash
pixelox new -o badge.pxo --size 1000x1000 --fill transparent
pixelox shape star badge.pxo --in-place --center 500,500 --points 12 --outer 460 --inner 380 \
  --fill '#ffd400' --stroke '#111111' --stroke-width 10 --name Burst
pixelox text add badge.pxo --in-place --name Arc --content "LIMITED EDITION • SUMMER DROP • " \
  --font Inter --weight 800 --size 54pt --color '#111111' --tracking 60
pixelox text on-path badge.pxo --in-place --layer Arc --d 'M 500,180 A 320,320 0 1 1 499.9,180'
pixelox fx stroke      badge.pxo --in-place --layer Arc   --width 6 --position outside --color '#ffffff'
pixelox fx drop-shadow badge.pxo --in-place --layer Burst --offset 0,10 --blur 24 --color '#00000066'
pixelox export badge.pxo -o badge.png                      # transparent PNG
```

#### E12 — Vectorize a scanned logo, recolor, optimize

```bash
pixelox trace scan.png -o logo.svg --mode binary --threshold 128 --speckle 8
pixelox svg el set logo.svg --in-place --where 'path' --attr fill='#0b5fff'
pixelox svg optimize logo.svg -o logo.min.svg --precision 2

# Self-check: rasterize at the scan's size and measure trace fidelity.
SIZE=$(pixelox info scan.png --json | jq -r '.result.size | "\(.[0])x\(.[1])"')
pixelox svg rasterize logo.min.svg -o check.png --size "$SIZE" --background '#ffffff'
pixelox analyze diff scan.png check.png --json
```

---

## 9. Pipeline files

Declarative multi-step recipes executed by `pipeline run` in a single process (one decode, N ops, atomic outputs). YAML is canonical; the same structure in JSON is accepted. `pixelox schema pipeline` publishes the schema; `pipeline validate` checks a file without running it.

### 9.1 Format specification

| Key | Meaning |
|---|---|
| `version` | Pipeline format version (integer, currently `1`). |
| `name`, `description` | Metadata, recorded in sidecars. |
| `defaults` | `seed`, `color-space`, `mem-budget`, `on-error: stop\|continue`. |
| `vars` | Named values; overridable via `pipeline run --var k=v`. `${vars.k}` interpolation. |
| `inputs` | Named input files. Optional integrity pin: `product: { path: shots/hero.jpg, sha256: "9f2a…" }` — mismatch aborts with `PX1601`. |
| `steps[]` | Each: `id` (unique), `run` (canonical dotted command id), `with` (params exactly as the command's JSON Schema defines — flag names sans `--`), optional `needs: [ids]` (DAG edges; default: previous step), optional `foreach`. |
| `foreach` | Fan-out: `{ glob: 'in/*.jpg', as: item }` or `{ csv: file.csv, as: row }` — the step runs per item/row with `${item.*}` / `${row.*}` bindings; globs provide `${item.in}`, `${item.stem}`, and — under `batch run --out-dir` — `${item.out}` (out-dir + stem + ext). Fan-outs run in parallel up to `--jobs`. |
| `outputs[]` | Declared deliverables; `pipeline run` fails if any is missing at the end. |
| Sidecars | Every written output gets its sidecar; additionally one pipeline-level sidecar records the whole run. `pipeline new --from-sidecar` inverts: sidecar → pipeline file. |
| Context | Steps share one in-memory document context per chain: a step's mutated document flows to `needs`-dependents without touching disk until an `export`/`output` step. |
| Determinism | The pipeline seed cascades per §6.6; `--dry-run` prints the fully resolved plan and DAG. |

### 9.2 Worked pipeline: the poster build

```yaml
# poster.yml — product shot + headline → print poster and OG card
version: 1
name: poster-fall-launch
description: A3 launch poster from brand template, plus a social crop
defaults: { seed: 0, on-error: stop }

vars:
  headline: "FALL DROP 09"
  accent:   "#0ea5e9"

inputs:
  template: { path: assets/poster-a3.pxo, sha256: "c41d22ab…" }
  product:  shots/hero.jpg
  logo:     brand/logo.svg

steps:
  - id: cutout
    run: detect.subject
    with: { input: "${inputs.product}", matte: true, cutout: work/hero-cut.png }

  - id: fill
    run: comp.template.apply
    needs: [cutout]
    with:
      input: "${inputs.template}"
      set:       { headline: "${vars.headline}" }
      set-image: { product: work/hero-cut.png, logo: "${inputs.logo}" }
      set-color: { accent: "${vars.accent}" }
      output: work/poster.pxo

  - id: print
    run: export
    needs: [fill]
    with: { input: work/poster.pxo, output: dist/poster-a3.png }

  - id: og
    run: export
    needs: [fill]
    with: { input: work/poster.pxo, output: dist/poster-og.png,
            size: 1200x630, region: "0,840,3508x1842" }

outputs: [dist/poster-a3.png, dist/poster-og.png]
```

```bash
pixelox pipeline run poster.yml --var headline="WINTER DROP 01" --json
```

---

## 10. MCP server & skill

### 10.1 `pixelox mcp serve`

Built-in MCP server (stdio transport; spawned and owned by the client — no daemon). Launch: `pixelox mcp serve --allow ./work --allow ./assets [--read-only] [--allow-model-downloads]`.

**Tool surface** — deliberately coarse so the whole surface costs little context; `pixelox_run` is the workhorse and reuses the CLI's schemas:

| MCP tool | Purpose |
|---|---|
| `pixelox_describe` | Returns the agent briefing (same ≤ 4k-token content as `describe --agent`). Call once, cache. |
| `pixelox_schema` | JSON Schema for a canonical command id (on-demand detail). |
| `pixelox_run` | Execute any command: `{ "command": "filter.gaussian", "params": {…}, "dry_run": false, "consent": [] }` → the standard envelope. Params validate against the command schema before execution. |
| `pixelox_info` | Fast read-only inspect of a document/image (the `info` command). |
| `pixelox_preview` | Render document / layer / region, downscaled (default ≤ 768 px), returned as MCP image content — the agent's eyes on its own work. |
| `pixelox_detect` | First-class detect convenience (most common agent entry point). |
| `pixelox_jobs` | `list / status / wait / cancel / result` for background jobs. |
| `pixelox_docs` | Offline reference page for a command or error code. |

**Resources:** sidecars under allowed roots and doc pages are exposed as MCP resources (`pixelox://sidecar/...`, `pixelox://docs/...`).

**Security model:**

- Every path in every call must resolve (post-symlink) inside an `--allow` root; violations return `PX1310` without touching the filesystem. The models dir is implicitly readable; nothing else outside the roots is.
- Commands execute **in-process** — there is no shell, so there is no injection surface via params.
- `--read-only` restricts to non-mutating commands. Model downloads require the server flag **and** `"consent": ["model-download"]` on the call — two keys to the network.
- Every call is audit-logged (NDJSON: timestamp, tool, params hash, outcome) under the state dir.
- The server inherits the binary's posture: no telemetry, no network except consented model pulls.

**Session flow (Claude-class agent):**

1. Host launches `pixelox mcp serve --allow ./work` alongside the session.
2. Agent calls `pixelox_describe` once; plans the workflow.
3. `pixelox_run {command:"open", params:{input:"work/shot.jpg", output:"work/shot.pxo"}}`.
4. `pixelox_run {command:"detect.subject", params:{input:"work/shot.pxo", in_place:true, save_mask:"subject"}, dry_run:true}` → plan says the model is missing → host surfaces the consent → re-run with `consent:["model-download"]`.
5. Iterate ops; after each milestone, `pixelox_preview` to visually confirm.
6. `pixelox_run {command:"export", …}`; then `pixelox_run {command:"verify", params:{sidecar:"work/out.png.pixelox.json"}}`; report paths + hashes.

### 10.2 Claude Code skill & agent files

- **`skills/pixelox/SKILL.md`** — installable as a Claude Code plugin via `.claude-plugin/` (marketplace manifest). Contents: when to trigger (any image/photo/SVG manipulation ask), the §8.1 conventions digest, the pipelines-over-chaining rule (>3 steps), ten recipe cards (the §8.22 set condensed), exit-code/error-code triage table, and the iron rules: always `--json`, always `--dry-run` before mutations on user files, never `--overwrite` without the user asking, verify with `analyze diff`/`verify` before declaring success.
- **`AGENTS.md`** (repo root) — the standing contract for any agent landing in the repository: build/test commands, where schemas live, how to run the golden suite, and a pointer to `describe --agent`.
- **Token budget:** SKILL.md ≤ 2,500 tokens; `describe --agent` ≤ 4,000 tokens; both CI-enforced.

---

## 11. Non-functional requirements

### 11.1 Performance budgets

Reference machine: 8-core/16-thread x86-64 (AVX2), 32 GB RAM, NVMe. Budgets are p95, release build, warm page cache, measured by the criterion benchmark suite in CI (regression gate: >10% slowdown fails the build). 24 MP = 6000×4000.

| Operation | Budget |
|---|---|
| `pixelox --help` | < 50 ms |
| `pixelox describe --agent` | < 120 ms |
| Minimal op round-trip (`new` 64×64 + `export` PNG) | < 150 ms |
| Decode 24 MP JPEG | < 350 ms |
| `adjust levels/curves` on 24 MP | < 150 ms |
| `filter gaussian --sigma 5` on 24 MP | < 450 ms |
| `transform resize` 24 MP → 2048 px (lanczos3) | < 300 ms |
| Encode 24 MP JPEG q85 | < 400 ms |
| **Any single common op, 24 MP, end to end (decode → op → encode)** | **< 1 s** |
| Composite 10-layer 24 MP document (mixed blends, 2 masks, 2 fx) | < 1.5 s |
| `detect subject` (BiRefNet-lite, 24 MP in, 1024 px inference) | < 2.5 s CPU |
| `detect segment` (MobileSAM, one prompt) | < 2.0 s CPU |
| `mask refine` ViTMatte, 2K band | < 4 s CPU |
| `fill content-aware` (LaMa), 512×512 hole | < 6 s CPU |
| `trace` 2 MP binary mode | < 3 s |
| `render d3` 10k-element chart → SVG | < 2 s |
| `render 3d` hidden-line, 50k-triangle mesh | < 5 s |
| Batch overhead per item (pipeline, excl. ops) | < 60 ms |

### 11.2 Memory

- Tiled evaluation (§7.3): peak RSS respects `--mem-budget` (default `min(50% RAM, 4 GB)`); exceeding it aborts cleanly with exit 11, never the OOM killer.
- A 100 MP document with a 10-layer stack must render inside 1.5 GB via tile streaming + cache eviction (LRU by tile, budget-aware).
- Codec streaming: progressive/row-based decode where formats allow; no full-file double buffering on export (`--ladder` shares the decode).

### 11.3 Determinism (contract summary; normative text in §6.6)

Bit-exact: same version + target. Perceptual (SSIM ≥ 0.999, ΔE00 p99 ≤ 1.0): across targets. Seeded everything; thread-count-invariant; CI runs the golden corpus on every matrix cell and cross-compares.

### 11.4 Cross-platform matrix

| Target | Tier | Notes |
|---|---|---|
| x86_64-unknown-linux-gnu | 1 | Full features incl. `ml` |
| aarch64-unknown-linux-gnu | 1 | Full features incl. `ml` |
| x86_64/aarch64-unknown-linux-musl | 1 | Static binary; **no `ml`** (§7.8); `doctor` reports |
| x86_64/aarch64-apple-darwin | 1 | Full; CoreML EP opt-in |
| x86_64-pc-windows-msvc | 1 | Full; DirectML EP opt-in |
| aarch64-pc-windows-msvc | 2 | Builds + smoke tests in CI |

Tier 1: full test suite + golden corpus green per release. Tier 2: builds, unit tests, smoke examples.

### 11.5 Size & startup

| Artifact | Budget |
|---|---|
| CLI binary, default features minus `ml` (stripped) | ≤ 30 MB (incl. embedded font subset, JS runtime, vendored d3/three, docs) |
| CLI binary with `ml` (ORT static CPU) | ≤ 75 MB |
| Docker image (distroless, with `ml`) | ≤ 120 MB |
| Model weights | Never bundled; on-demand per §7.8 |
| Cold start to first pixel op | ≤ 150 ms (no lazy JIT, no Python) |

### 11.6 Color correctness

- Untagged = sRGB; tagged inputs honored via lcms2; exports embed the working profile; `--profile` converts on the way out.
- Round-trip guarantee: open → export with no ops is lossless for same-format same-depth (bit-identical for PNG/TIFF; JPEG re-encode only when ops occurred, otherwise stream-copied with metadata edits).
- Linear-light behavior per op documented and recipe-recorded (§7.4); golden tests include a color-accuracy suite (ColorChecker synthetic renders, ΔE00 ≤ 1 after identity pipelines).
- Premultiplied-alpha discipline: all internal compositing premultiplied; codecs receive straight alpha; fringe-artifact regression tests on the corpus.

### 11.7 Test strategy

| Layer | Approach |
|---|---|
| Golden-image tests | Checked-in small corpus (`testdata/`, < 20 MB) + extended corpus repo pulled in CI; every op and fx rendered and compared with perceptual diff (SSIM/ΔE00 thresholds per §6.6); per-target goldens where float-sensitive. |
| CLI contract tests | trycmd-style: every command's help, envelope shape, exit codes, error codes; schema snapshot tests (breaking schema change fails CI). |
| Property tests | proptest on op algebra: `select invert ∘ invert = id`, boolean-op set laws, `mask feather --radius 0 = id`, resize up/down bounds, blend-mode commutativity where mathematically guaranteed, codec round-trips on random valid images. |
| Unit/integration | Per-crate; PSD fixture suite (generated from scripted Photoshop exports, checked in); SVG conformance subset (resvg test suite reuse). |
| Fuzzing | cargo-fuzz targets per decoder + `.pxo` reader + pipeline parser; scheduled weekly + on codec PRs. |
| Examples in CI | Every §8.22 example and every README/docs snippet extracted and executed against goldens (the ferraus rule; `tools/run-doc-examples`). |
| Determinism CI | Same-cell repeat runs byte-compared; cross-cell perceptual compare; thread-count sweep (1, 4, 16) byte-compared. |
| Budgets in CI | §11.1 criterion gates; binary-size gate; `describe --agent` + SKILL.md token-count gates. |

### 11.8 Security

| Surface | Posture |
|---|---|
| Network | **None by default.** Only `models pull` / consented auto-pull and `self-update` touch the network; both HTTPS with SHA-256 pins embedded in the release binary; `self-update` additionally minisign-verified. |
| JS sandbox | §7.10: no fs/net/env/timers/eval-imports; fuel, heap, wall-clock limits; sandbox escapes are release-blocking security bugs with CVE process. |
| Input parsing | All codecs memory-safe Rust or vendored C behind fuzzed safe wrappers; decompression-bomb pixel limit (512 MP default); SVG external references disabled by default; XML entity expansion disabled. |
| MCP | Path allow-listing with symlink resolution, in-process execution (no shell), read-only mode, two-key model-download consent, NDJSON audit log (§10.1). |
| Filesystem | Atomic writes everywhere; never follows output-path symlinks out of the target dir in MCP mode; no implicit deletion of user files anywhere in the surface. |
| Privacy | No telemetry, no crash upload; sidecars support `--redact-paths`; EXIF preserved by default but `--strip-metadata` is one flag away and documented in the skill. |
| Supply chain | `cargo-deny` (licenses + advisories) in CI; `cargo auditable` SBOM embedded in release binaries; pinned lockfile; release provenance (SLSA-style attestations). |

### 11.9 Distribution & packaging

Mirrors ferraus: `cargo install pixelox`; `curl -fsSL https://pixelox.dev/install.sh | sh` (macOS/Linux) and PowerShell installer + scoop + MSI (Windows); Homebrew tap; apt/dnf repos; Docker (`ghcr.io/…/pixelox`, amd64+arm64); plain static musl tarball. All artifacts checksummed + minisign-signed. Distro-packaged builds disable `self-update`. Model weights always download on first use, SHA-256 verified (§7.8).

---

## 12. Milestones & roadmap

Assumes a team of 3–4 senior engineers. Each milestone ships tagged binaries for all Tier-1 targets, a CHANGELOG entry, updated docs, and green CI including golden corpus, examples, and budget gates. Acceptance criteria are cumulative — later milestones must not regress earlier ones.

### v0.1 — "Document engine" (target: 2027-03)

Scope: the agent contract + core raster editing. Crates: core, raster, io, text, cli.

- Formats: JPEG/PNG/GIF(first-frame + `frames *`)/TIFF/WebP/BMP read-write per §7.6; **PSD layered read** per §7.6.1; `.pxo` v1.
- Document model: full layer stack (raster/text/shape-basic/group/adjustment), all 27 blend modes + pass-through, layer masks, clipping masks, opacity/lock/visibility.
- Selections & masks: full `select` + `mask` groups (except `mask refine --model`).
- `adjust` complete; `filter` complete except `lens-blur --depth auto`; `retouch` complete except `redeye --auto`; `fill` solid/gradient/pattern.
- `transform`, `text` (shaping incl. RTL/CJK, variable fonts, on-path, to-path), `shape` (all primitives; `path` booleans arrive in v0.3), `comp` (place/align/distribute/guides/presets; templates **init/slots/apply**), `analyze`, `frames`, `export` incl. ladder.
- Agent contract: envelope v1, schemas, describe/docs, dry-run, overwrite/in-place gates, sidecars, replay/verify, exit codes, error catalog, seed model. `pipeline` + `batch` + `jobs`.
- **Acceptance:** personas B and D stories pass; E4, E5 (with a pre-cut product image standing in for the v0.2 `detect` step), E7, E8, E11 run in CI; §11.1 non-ML budgets met; determinism CI green; `describe --agent` ≤ 4k tokens.

### v0.2 — "Eyes" (target: 2027-05)

Scope: `pixelox-ml` + everything AI-assisted + fx completion.

- `detect` group complete (objects/subject/salient/faces/text-regions/segment with point+box prompts); model registry, consent gates, SHA-256 verification; `--mask-out/--save-mask/--select/--cutout` materialization incl. box→segment chaining.
- `mask refine` (ViTMatte), `fill content-aware` (LaMa), `filter lens-blur --depth auto`, `retouch redeye --auto`, `transform straighten --angle auto`.
- **Layer fx complete** (all ten styles, `copy`/`scale`), PSD fx import mapping.
- History commands (`history list|undo|checkpoint`).
- **Acceptance:** persona A stories pass end-to-end; E1 (both forms), E2, E3, E5 (full) in CI; ML budgets (§11.1) met on reference CPU; model licensing audit documented; fx golden parity vs Photoshop reference renders within perceptual thresholds.

### v0.3 — "Vector" (target: 2027-07)

Scope: `pixelox-vector` surface + bridges.

- `svg` group complete (authoring, defs incl. filters, query, optimize, rasterize, from-doc); boolean `path` ops on shape layers; `trace` (binary + color); `shape`/`path` stretch polish (dash offsets, markers).
- PSD 16-bit read **[stretch pull-in if schedule allows]**.
- **Acceptance:** E9's rasterize leg, E10's rasterize leg, E12 in CI; SVG output validates and round-trips through usvg; boolean-op property suite green; persona C story C1 with vector logo slots.

### v0.4 — "Generative" (target: 2027-09)

Scope: `pixelox-js` + `pixelox-3d`.

- `render js|d3|three` with the §7.10 sandbox, vendored pinned d3/three, DOM-lite, Canvas2D binding, limits + determinism; `render 3d` native (primitives, extrude, OBJ+glTF import, iso/persp cameras, wireframe/hidden-line/shaded, `--as-layer`).
- **Acceptance:** E9, E10 fully in CI; sandbox escape test-suite green (red-team cases: fs/net/env/time/import attempts all `PX60xx`); render budgets met; identical outputs across Tier-1 targets for the JS corpus (per §6.6 perceptual rule).

### v0.5 — "Agent-native" (target: 2027-11)

Scope: MCP + skill + packaging completeness.

- `mcp serve` full tool surface + resources + audit log + allow-listing (§10.1); `skills/pixelox/SKILL.md` + `.claude-plugin/` + `AGENTS.md`; `pipeline new --from-sidecar`; `self-update` signed.
- Packaging matrix complete (§11.9): installers, brew, scoop/MSI, apt/dnf, Docker, musl tarball.
- **Acceptance:** persona A full MCP session flow (§10.1) reproduced in CI with a scripted MCP client from `examples/`; skill token budget enforced; install smoke tests on all package channels.

### v1.0 — "Hardening" (target: 2028-Q1)

- Six weeks of API freeze: schema/envelope/sidecar/pxo versions locked; deprecation policy (one minor of warnings) in effect.
- Fuzzing campaign (≥ 10^9 execs/decoder cumulative), third-party security review of sandbox + MCP, performance soak on the extended corpus, docs completeness audit (every command's `docs` page reviewed), ≥ 95% of PRD acceptance criteria green, remaining stretch items re-triaged publicly.
- Stretch pulls (as capacity allows, else explicitly re-scheduled): HEIC read, RAW read, PDF export, `detect text --recognize`, open-vocab prompts, SAM 2.1, content-aware crop, mesh gradients, centerline trace.

---

## 13. Open questions & risks

### 13.1 Open questions

| # | Question | Decision needed by | Current lean |
|---|---|---|---|
| Q1 | tiny-skia feature gaps: will fx parity (contoured bevels, satin) need Skia-grade path effects? | v0.2 design spike, week 2 | Own kernels suffice; ADR-0002 documents the skia-safe reversal trigger. |
| Q2 | QuickJS throughput on worst-case d3 workloads (50k+ nodes)? | v0.4 spike (prototype exists by v0.2) | Acceptable; deno_core feature-gated fallback per ADR-0003 if not. |
| Q3 | `psd` crate extension vs. fork vs. clean-room reader for groups/16-bit/fx parsing? | v0.1, week 4 | Extend upstream; fork if velocity demands. |
| Q4 | Ship a `ml-lite` candle-based static build (musl + subject/segment only)? | post-v0.2 review | Yes if candle op coverage holds for BiRefNet/MobileSAM. |
| Q5 | Pure-Rust CMS (qcms/moxcms) to replace lcms2 and drop the last always-on C? | v0.3 | Swap when ICC v4 + ΔE test suite passes. |
| Q6 | Default blend space: stay PS-parity (perceptual) or linear-by-default with `--ps-compat`? | v0.1, week 2 (hard to reverse) | PS-parity default; linear via doc setting. Decided here unless golden tests argue otherwise. |
| Q7 | `.pxo` cache format: keep rendered tiles in-container or sibling cache dir? | v0.1, week 6 | In-container optional (`save --no-cache` default off for small docs, on for >100 MP). |
| Q8 | Windows aarch64 ORT availability/quality? | v0.2 | Tier 2 without `ml` until ORT proves out. |

### 13.2 Risks

| # | Risk | L×I | Mitigation |
|---|---|---|---|
| R1 | **skia-safe reversal**: fx golden parity misses budget on tiny-skia + own kernels, forcing backend rework. | M×H | Week-2 v0.2 spike renders the 3 hardest fx vs PS references; reversal trigger + feature-flag path pre-planned (ADR-0002). |
| R2 | **PSD read fidelity** disappoints on real-world brand files (groups-in-groups, 16-bit, exotic fx). | H×M | Fixture corpus from real agency files early; §7.6.1 matrix sets expectations; composite fallback guarantees *something correct* always renders; warnings are loud. |
| R3 | **Model licensing drift** (weights relicensed, mirrors vanish). | M×H | Apache/MIT-only default registry; licenses recorded per model; weights mirrored to project-controlled storage; `models verify` + pinned hashes. |
| R4 | **ORT + musl conflict** fragments the "one binary" story. | H×M | Accepted and documented (§7.8): musl ships without `ml`; Q4 `ml-lite` path; `doctor` makes the variant visible. |
| R5 | **QuickJS perf cliff** on heavy generative scripts. | M×M | Rust-side Canvas/layout does the heavy work; budgets in CI from v0.4 day one; deno_core fallback (Q2). |
| R6 | **Headless-GPU expectations**: users ask for shaded 3D beyond CPU budget. | M×L | Scope fence in §4.2/§7.11 (wireframe/flat only, CPU); wgpu offscreen is a post-1.0 investigation, never a v1 dependency. |
| R7 | **HEIC/HEVC patents**; RAW decoder licenses (LGPL). | M×M | HEIC: system-lib dynamic feature only, never bundled. RAW: isolate rawler behind dynamic feature or separate helper binary; legal review before enabling in official builds. |
| R8 | **Color-management correctness bugs** erode the core trust proposition. | M×H | ColorChecker golden suite from v0.1; round-trip guarantees tested; lcms2 battle-tested; linear/perceptual behavior explicit per op. |
| R9 | **Scope gravity**: Photoshop parity is infinite; v0.1 slips. | H×H | The §14.2 coverage map is the fence: features not in it don't enter 0.x; PRD change control via ADRs; milestone acceptance criteria are the contract. |
| R10 | **Text shaping edge cases** (Indic scripts, vertical CJK, emoji/color fonts). | M×M | rustybuzz inherits HarfBuzz behavior; golden tests per script family; emoji/color fonts documented as [stretch]; fallback chain explicit in `doctor`. |
| R11 | **Dependency license audit surprises** (GPL transitive, e.g., quantizers/tracers). | M×M | `cargo-deny` allowlist (Apache/MIT/BSD/MPL/Zlib/OFL) enforced in CI from day one; libimagequant and potrace already excluded by design. |
| R12 | **Golden-test flake across targets** (float drift) burns CI trust. | M×M | Per-target goldens where needed; perceptual thresholds not bit-equality cross-target (§6.6); quarantine lane + drift dashboards. |
| R13 | **Sidecar/privacy**: recipes leak local paths or prompts in shared files. | L×M | `--redact-paths`, documented in skill; MCP audit log stays local; no network egress to leak through. |
| R14 | **Binary-size creep** past §11.5 budgets (ORT, vendored JS, docs). | M×L | Size gate in CI per target; docs compressed; models never embedded. |

---

## 14. Appendices

### 14.1 Glossary

| Term | Meaning |
|---|---|
| **Adjustment layer** | A non-destructive tonal/color op lifted into the layer tree, affecting the composite below it, scoped by its mask. |
| **Agent contract** | The stable machine interface of §6: envelope, schemas, describe/docs, consent gates, sidecars, replay/verify, exit/error codes. |
| **Blend mode** | Per-layer pixel compositing function (§14.3). |
| **Clipping mask** | A layer whose visibility is confined to the opaque area of the layer below (`layer clip`). |
| **Content-aware fill / inpainting** | Model-assisted reconstruction of a masked region from its surroundings (`fill content-aware`). |
| **Envelope** | The single versioned JSON object every command emits with `--json` (§6.1). |
| **fx / layer style** | Non-destructive per-layer effect stack: shadows, glows, bevel, satin, overlays, stroke (§8.13). |
| **Golden test** | A rendered output compared against a stored reference (bit-exact same-target, perceptual cross-target). |
| **Layer mask** | Per-layer grayscale alpha attachment controlling layer visibility non-destructively (`mask add`). |
| **Matting** | Estimating soft alpha at object edges (hair/fur grade); `mask refine --model vitmatte`. |
| **MCP** | Model Context Protocol; `pixelox mcp serve` exposes the tool surface to agent hosts (§10). |
| **Named mask** | Document-level stored grayscale mask, addressable by name anywhere `--mask` is accepted (§8.5). |
| **Op-graph** | The DAG of recorded operations that defines document pixels; rendering evaluates it with caching (§7.3). |
| **Placed layer** | Smart-object analog: a layer referencing an external or embedded source file, re-renderable on change (`comp place`). |
| **Provenance sidecar** | `<output>.pixelox.json`: the self-contained recipe + hashes that make a result reproducible (§6.4). |
| **`.pxo`** | Pixelox's native zip-container document format (§7.12). |
| **Recipe** | The linearized op history embedded in sidecars and `.pxo`; input to `replay`. |
| **Seed lane** | The per-op deterministic RNG stream derived from `(seed, op index, op id)` (§6.6). |
| **Selection** | The document's active soft coverage map (`@sel`), convertible to/from masks (§8.4). |
| **Template slot** | A typed, named fill-in point (text/image/color) declared on a document (`comp template *`). |
| **Trace** | Bitmap→vector conversion (`trace`, §8.17). |
| **Trimap** | Known-fg / known-bg / unknown-band map fed to matting models (built automatically by `mask refine --band`). |

### 14.2 Photoshop-feature coverage map

The scope fence (risk R9): parity targets live here; a Photoshop feature absent from this table is out of scope for 0.x. "—" = deliberately not planned.

| Area | Photoshop feature | Pixelox | Milestone |
|---|---|---|---|
| File | Open / Place / Export (incl. Export As ladder) | `open` / `comp place` / `export [--ladder]` | 0.1 |
| File | Layered PSD import | `open in.psd` (§7.6.1 matrix) | 0.1 |
| File | PSD export (flattened / basic layered) | `export … -o out.psd [--layered-basic]` | 0.1 |
| Image | Image Size / Canvas Size / Rotate / Trim | `transform resize / canvas / rotate / trim` | 0.1 |
| Image | Crop & Straighten | `transform crop / straighten` | 0.1 (`--angle auto` 0.2) |
| Image | Mode: 8/16-bit, color profiles | `--depth`, `--profile`, `--icc` | 0.1 |
| Adjust | Brightness/Contrast · Levels · Curves · Exposure | `adjust brightness-contrast / levels / curves / exposure` | 0.1 |
| Adjust | Shadows/Highlights · HDR Toning | `adjust shadows-highlights / tone-map` | 0.1 |
| Adjust | Hue/Saturation · Color Balance · Vibrance | `adjust hsl / color-balance / vibrance` | 0.1 |
| Adjust | Photo Filter (warm/cool) | `adjust white-balance --temp/--tint` | 0.1 |
| Adjust | Black & White · Channel Mixer · Selective Color | `adjust bw / channel-mixer / selective-color` | 0.1 |
| Adjust | Gradient Map · Color Lookup (LUT) | `adjust gradient-map / lut` | 0.1 |
| Adjust | Invert · Equalize · Posterize · Threshold | `adjust invert / equalize / posterize / threshold` | 0.1 |
| Adjust | Auto Tone/Contrast/Color | `adjust auto --tone/--contrast/--color` | 0.1 |
| Adjust | Adjustment layers (all above, non-destructive) | any `adjust * --as-layer` | 0.1 |
| Select | All / Deselect / Inverse / marquee / lasso / polygonal | `select all / none / invert / rect / ellipse / poly / path` | 0.1 |
| Select | Magic Wand / Color Range / luminosity masks | `select color [--contiguous] / luma` | 0.1 |
| Select | Select Subject / Object Selection | `detect subject / segment` | 0.2 |
| Select | Select & Mask (Refine Edge) | `mask refine --model vitmatte` | 0.2 |
| Select | Modify: Feather/Expand/Contract/Smooth | `mask feather / expand / contract / smooth` (any mask incl. `@sel`) | 0.1 |
| Select | Save / Load Selection | `select save / load` | 0.1 |
| Layer | New/Duplicate/Delete/Group/Merge/Flatten/reorder | `layer *` | 0.1 |
| Layer | Blend modes (full set) + opacity | `layer blend / opacity` (§14.3) | 0.1 |
| Layer | Layer masks (+ density) / clipping masks | `mask add / density` · `layer clip` | 0.1 |
| Layer | Vector masks | `mask add --from-path` | 0.3 |
| Layer | Smart Objects (linked/embedded) | `comp place --link/--embed`, `relink`, `update-links` | 0.1 |
| Layer | Layer styles: shadows, glows, bevel/emboss, satin, overlays, stroke | `fx *` (all ten) | 0.2 |
| Layer | Align / Distribute | `comp align / distribute` | 0.1 |
| Type | Type layers: family/size/weight/style, fill/stroke, kerning/tracking/leading, alignment | `text add / edit / style` | 0.1 |
| Type | Type on a path · Convert to shape | `text on-path / to-path` | 0.1 |
| Type | Variable fonts · complex shaping (RTL/CJK/ligatures) | `--variation`; rustybuzz shaping | 0.1 |
| Edit | Free Transform: move/scale/rotate/flip/skew/distort/perspective/warp | `transform *` (warp = grid mesh) | 0.1 |
| Edit | Content-Aware Fill | `fill content-aware` | 0.2 |
| Edit | Content-Aware Scale / Crop | `transform crop --content-aware` | stretch |
| Edit | Fill (solid/gradient/pattern) · Stroke | `fill *` · `fx stroke` / shape strokes | 0.1 |
| Edit | Actions / batch automation | `pipeline *` / `batch run` (superior analog) | 0.1 |
| Tools | Clone Stamp / Healing / Spot Healing | `retouch clone / heal` | 0.1 |
| Tools | Dodge / Burn / Sharpen / Blur tools | `retouch dodge / burn / sharpen / blur` (region ops) | 0.1 |
| Tools | Red Eye | `retouch redeye` | 0.1 (`--auto` 0.2) |
| Tools | Eraser | `retouch erase` | 0.1 |
| Tools | Eyedropper / sampling | `analyze stats --rect X,Y,1x1` | 0.1 |
| Tools | Pen & shape tools (incl. boolean ops) | `shape *` / `path *` | 0.1 (booleans 0.3) |
| Filter | Blur: Gaussian/Box/Motion/Radial(spin/zoom)/Lens/Surface/Median | `filter *` | 0.1 (`lens --depth auto` 0.2) |
| Filter | Sharpen: Unsharp Mask / Smart Sharpen | `filter unsharp / smart-sharpen` | 0.1 |
| Filter | Noise: Add/Reduce/Despeckle/Dust & Scratches | `filter add-noise / denoise / despeckle / dust` | 0.1 |
| Filter | Stylize: Emboss / Find Edges / High Pass / Mosaic | `filter emboss / find-edges / high-pass / pixelate` | 0.1 |
| Filter | Lens Correction (distortion/CA/vignette) | `filter lens-correct / vignette` | 0.1 |
| Filter | Liquify | `transform warp` covers mesh warp; push/bloat brushes — | 0.1 partial |
| Filter | Vanishing Point · Sky Replacement · Neural Filters | — (N5 integration point) | — |
| View | Guides / Grids / snapping | `comp guide / grid` | 0.1 |
| Doc | Artboards / deliverable presets | `new --preset` + templates | 0.1 |
| Analysis | Histogram / Info panel | `analyze histogram / stats / colors / exif` | 0.1 |
| Illustrator | Programmatic vector authoring (SVG) | `svg *` | 0.3 |
| Illustrator | Image Trace | `trace` | 0.3 |
| Illustrator | Pathfinder booleans | `path unite/subtract/intersect/exclude` | 0.3 |
| Fireworks | Web/social deliverables from templates | `comp template *` + presets | 0.1 |
| — | Data-driven graphics (d3) · 3D-lite | `render d3 / three / 3d` | 0.4 |

### 14.3 Blend-mode parity table (PSD interchange)

All modes ship in v0.1, implemented in the f32 compositor (§7.5); PSD keys are the 4-byte signatures used on read/write. Classes: PD = Porter-Duff/basic, S = separable (per-channel), NS = non-separable (HSL-space).

| Pixelox mode | PSD key | Class | Note |
|---|---|---|---|
| `pass-through` | `pass` | — | Groups only (default for groups). |
| `normal` | `norm` | PD | — |
| `dissolve` | `diss` | PD | Stochastic coverage; seeded (§6.6) ⇒ deterministic. |
| `darken` | `dark` | S | — |
| `multiply` | `mul ` | S | — |
| `color-burn` | `idiv` | S | — |
| `linear-burn` | `lbrn` | S | Not in Skia; own kernel. |
| `darker-color` | `dkCl` | NS | Compares composite luminance. |
| `lighten` | `lite` | S | — |
| `screen` | `scrn` | S | — |
| `color-dodge` | `div ` | S | — |
| `linear-dodge` | `lddg` | S | a.k.a. Add. |
| `lighter-color` | `lgCl` | NS | — |
| `overlay` | `over` | S | — |
| `soft-light` | `sLit` | S | W3C/PS formula variant: PS parity chosen, documented. |
| `hard-light` | `hLit` | S | — |
| `vivid-light` | `vLit` | S | Own kernel. |
| `linear-light` | `lLit` | S | Own kernel. |
| `pin-light` | `pLit` | S | Own kernel. |
| `hard-mix` | `hMix` | S | Own kernel. |
| `difference` | `diff` | S | — |
| `exclusion` | `smud` | S | — |
| `subtract` | `fsub` | S | Own kernel. |
| `divide` | `fdiv` | S | Own kernel. |
| `hue` | `hue ` | NS | HSL-space per PDF/PS spec. |
| `saturation` | `sat ` | NS | — |
| `color` | `colr` | NS | — |
| `luminosity` | `lum ` | NS | — |

---

## 15. Document history

| Version | Date | Author | Changes |
|---|---|---|---|
| 0.1.0-draft | 2026-11-04 | Ikakke Ikpe (product owner) with Claude Fable 5 | Founding PRD: vision, agent contract, architecture, full command surface, pipelines, MCP/skill, NFRs, roadmap v0.1→1.0, risk register, appendices. |

*This PRD is the product's contract until superseded. Scope changes land as PRs against this file with an ADR in `docs/decisions/`; reality diverging from this document is a bug in one of them — surface it, don't paper over it.*
