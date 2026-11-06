# Using pixelox from an AI agent

pixelox is a command-line image editor built to be driven by scripts and agents. This file
is the short operating manual. `pixelox describe --agent` prints the whole command surface
compactly (also in [docs/agents/catalog.md](docs/agents/catalog.md)); `pixelox docs <command>`
prints any command's reference page.

## Contract

* Add `--json` to every call. stdout then holds exactly one envelope: `ok`, `command`,
  `result` (doc ops include `op`, `params`, `document`), `inputs` and `outputs` (paths with
  SHA-256), `warnings` (`PXW####`), `timing_ms`; on failure `error` with `code` (`PX####`),
  `message`, `hint` (the next action), `docs`, `context`, and `exit_code`.
* Logs and progress go to stderr; `--progress json` gives NDJSON events.
* `pixelox schema <command>` prints the JSON Schema of its parameters and envelope.
* Never prompts. Never overwrites without `--overwrite` (exit 4). Changes a `.pxo` only with
  `--in-place`; never edits a source image in place. Writes are atomic.
* Every output gets `<output>.pixelox.json`, the full recipe back to its sources.
  `pixelox replay <sidecar> -o <out>` rebuilds it; `pixelox verify <sidecar>` checks it.
* Exit codes: 0 ok, 1 runtime, 2 usage, 3 input missing, 4 refused overwrite, 5 unsupported
  format, 6 consent required, 7 script sandbox, 8 job, 9 verification, 10 interrupted,
  11 memory, 12 not in this build.

## Working habits that pay off

1. Inspect first: `pixelox info photo.jpg --json`, `pixelox layer list doc.pxo --json`.
2. Keep work in a document: `pixelox open photo.jpg -o doc.pxo`, then `--in-place`
   edits; export deliverables at the end.
3. Plan with `--dry-run --json` before anything that writes.
4. More than three steps: write a pipeline file (`pixelox docs pipelines`) and
   `pixelox pipeline run job.yml`; one decode, many ops, one save.
5. Look at your work: `pixelox analyze stats out.png --json`,
   `pixelox analyze diff before.png after.png --json`, or the MCP `pixelox_preview` tool.
6. Finish with `pixelox verify out.png.pixelox.json --json` and report paths and hashes.
7. Models download only with `-y` and the user's consent. `detect subject` (BiRefNet-lite,
   224 MB) and `detect salient` (U²-Netp, 5 MB) run locally once downloaded; until then
   they exit 6 (PX1502) with the download command: ask the user, then add `-y`. The other
   AI commands (`detect objects|faces|text|segment`, `mask refine`, `fill content-aware`)
   exit 12; make those masks with `select sky`, `select color --contiguous`, `select luma`,
   or `select path`.
8. Brushwork is `paint stroke` (points or an SVG path; size, hardness, flow, opacity, tip
   images, jitter) onto pixels, a layer mask (`--target mask`), or the selection. Filters
   on a placed layer stay editable as smart filters; `layer stamp` merges what is visible
   into a new layer for a final grade. Walkthrough: `docs/guides/cookbook/photo-composite.md`.
9. Portrait retouching: `select skin --at X,Y` (sample a cheek; add a shadow sample in hard
   light) gives the skin mask; `retouch blemishes --mask skin` lists spots with their
   `redness` (pimples are red, moles are not: `--min-redness 10 --heal` keeps moles);
   `retouch frequency-split` builds exact Low/High layers; smooth only `Low`. The playbook
   is `skills/pixelox-retouch/SKILL.md`.

## Addressing

`--layer name|#N|@top|@bottom|@active` · `--mask name|@sel|@alpha:LAYER|@mask:LAYER|mask.png`
(`--invert-mask`) · geometry `X,Y,WxH`, `X,Y`, `WxH` in px, `%`, `pt`, `mm`, `in`
(negative = from the right/bottom; `--anchor tl..br`) · colors `#rrggbb[aa]`, `rgb()`,
`hsl()`, CSS names, `transparent`. Details: `pixelox docs addressing`.

## Recipes

```bash
# cut out a subject on a plain backdrop
pixelox open product.jpg -o p.pxo
pixelox select color p.pxo --in-place --ref white --at 4,4 --contiguous --tolerance 40
pixelox select invert p.pxo --in-place
pixelox mask feather p.pxo --in-place --mask @sel --radius 2
pixelox export p.pxo -o product-cut.png --apply-mask @sel

# non-destructive grade, then a web ladder
pixelox adjust curves p.pxo --in-place --points '0,0 64,56 192,206 255,255' --as-layer
pixelox export p.pxo --ladder 2048,1024,512 --formats jpg,webp -o web/

# a social card from a template
pixelox comp template apply card.pxo --set name="Ada Lovelace" --set-image headshot=ada.jpg -o out/ada.pxo
pixelox export out/ada.pxo -o out/ada.png
```

## MCP

```bash
claude mcp add pixelox -- pixelox mcp serve --allow ~/work
```

Tools: `pixelox_describe`, `pixelox_schema`, `pixelox_run`, `pixelox_info`,
`pixelox_preview`, `pixelox_detect`, `pixelox_jobs`, `pixelox_docs`. See `pixelox docs mcp`.

## Working on the pixelox codebase

* Build: `cargo build -p pixelox`; run: `target/debug/pixelox`.
* Test everything: `cargo test --workspace`; the contract suite (schema snapshots, the
  briefing budget, every documented example parsing) is
  `cargo test -p pixelox --test contract`; update snapshots after an intentional contract
  change with `UPDATE_SNAPSHOTS=1 cargo test -p pixelox --test contract`.
* Lint: `cargo clippy --workspace --all-targets -- -D warnings`; format: `cargo fmt --all`.
* Examples: `examples/run-all.sh` (needs a built binary; `PIXELOX=target/debug/pixelox`).
* How commands are defined: [docs/dev/AUTHORING.md](docs/dev/AUTHORING.md). Decisions:
  [docs/decisions/](docs/decisions/). Schemas live in the op specs; snapshots in
  `crates/pixelox-cli/tests/snapshots/schema/`.
