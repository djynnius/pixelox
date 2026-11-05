# The agent contract

pixelox is built to be driven by scripts and AI agents. These rules hold for every command.

## One envelope on stdout

With `--json`, stdout carries exactly one JSON object; logs and progress go to stderr.

```json
{"tool": "pixelox", "version": "0.1.0", "envelope": 1, "command": "filter.gaussian",
 "ok": true, "dry_run": false, "seed": 0,
 "result": {"op": "filter.gaussian", "params": {"sigma": "8"}, "document": {"size": [6000, 4000]}},
 "inputs": [{"path": "walk.pxo", "sha256": "9f2a…", "bytes": 24117248}],
 "outputs": [{"path": "walk.pxo", "sha256": "41c7…", "bytes": 24201114, "sidecar": "walk.pxo.pixelox.json"}],
 "warnings": [], "timing_ms": 412}
```

On failure `ok` is false and `error` holds `code` (`PX####`), `message`, `hint` (the next
action), `docs` (`errors/PX1401`), `context`, and `exit_code`. The exit code is authoritative.
Fields are added, never removed or retyped, within envelope version 1.

## Plan, then act

* `--dry-run` on any command that writes: validates inputs, resolves layers and masks,
  and returns `result.plan` (ops, predicted outputs, models and consent required). Nothing
  is written, not even sidecars.
* Read the plan, then run the same command without `--dry-run`.

## Consent gates

* Existing files are never replaced without `--overwrite` (exit 4, PX1401).
* A `.pxo` document is changed only with `--in-place`; otherwise mutating commands need
  `-o`. Source images (JPEG, PNG, PSD, …) are never edited in place (PX1108).
* Network actions (model downloads) need `-y` (exit 6, PX1502 otherwise).
* Writes are atomic: a failed command leaves no partial output.

## Provenance

Every written output `X` gets `X.pixelox.json`, a sidecar with the full recipe back to the
source files and their SHA-256 hashes. `pixelox replay X.pixelox.json -o Y` rebuilds it;
`pixelox verify X.pixelox.json` checks it. See `pixelox docs sidecars`.

## Determinism

Same pixelox version and platform, same inputs, same seed ⇒ byte-identical outputs. The
seed defaults to 0; `--seed random` draws one and records it. Thread count never changes
output bytes.

## Global options

| Flag | Meaning |
| --- | --- |
| `-o, --output PATH` | Output; the extension picks the format (`.pxo` keeps layers) |
| `--json` | One envelope on stdout |
| `--dry-run` | Plan only |
| `--overwrite` | Allow replacing outputs |
| `--in-place` | Rewrite the input `.pxo` |
| `--seed N\|random` | Seed for every random draw |
| `-y, --yes` | Consent to downloads for this run |
| `-q`, `-v` | Quieter, louder stderr |
| `--threads N`, `--mem-budget MB` | Resources |
| `--progress bar\|json\|none` | Progress on stderr (`json`: NDJSON events) |
| `--no-sidecar`, `--redact-paths` | Sidecar control |
| `--background` | Run as a job (`pixelox jobs …`) |
| `--config PATH` | Alternate config file |
| `--limit-pixels N` | Decompression-bomb guard |

Precedence: flag > `PIXELOX_*` environment variable > `config.toml` > default.

## Exit codes

| Exit | Meaning |
| --- | --- |
| 0 | Success |
| 1 | Runtime failure |
| 2 | Usage error |
| 3 | Input missing |
| 4 | Refused to overwrite or mutate |
| 5 | Unsupported format |
| 6 | Consent required |
| 7 | Script sandbox |
| 8 | Job error |
| 9 | Verification mismatch |
| 10 | Interrupted |
| 11 | Memory budget |
| 12 | Not in this build |
