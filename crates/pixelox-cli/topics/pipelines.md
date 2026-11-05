# Pipelines and batches

A pipeline file runs several steps in one process: one decode, N ops, one save. Prefer one
for anything over three steps.

```yaml
version: 1
name: poster
vars: { headline: "FALL DROP 09" }
inputs:
  template: { path: assets/poster.pxo, sha256: "c41d…" }   # optional integrity pin
steps:
  - id: fill
    run: comp.template.apply
    with: { input: "${inputs.template}", set: { headline: "${vars.headline}" }, output: work/poster.pxo }
  - id: print
    run: export
    needs: [fill]
    with: { input: work/poster.pxo, output: dist/poster.png }
outputs: [dist/poster.png]
```

* `run` is a dotted command id; `with` holds the command's parameters by their flag names
  (`strip-metadata`, `in-place`), plus `input`, `inputs`, `output`.
* A map value expands a repeatable flag: `set: {headline: X}` is `--set headline=X`.
* Steps without `input` work on the document produced by the step they `need` (default: the
  previous step), in memory.
* `${vars.k}` (override with `--var k=v`), `${inputs.k}`, `${item.in}`, `${item.stem}`,
  `${item.ext}`, `${item.dir}`, `${item.out}`, `${item.n}`, `${row.<column>}`.
* `foreach: { glob: 'in/*.jpg', as: item }` or `foreach: { csv: rows.csv, as: row }` fans a
  step out; items run in parallel up to `--jobs`.
* `defaults: { seed: 0, on-error: stop|continue }`.
* `outputs` are checked at the end (PX7104 when missing).

Commands: `pipeline run p.yml [--var k=v] [--jobs N]`, `pipeline validate`,
`pipeline graph [--format dot]`, `pipeline new -o p.yml [--from-sidecar out.png.pixelox.json]`.

## Batches

`pixelox batch run --glob 'shoot/*.jpg' --pipeline watermark.yml --out-dir out/ --jobs 8`
runs a pipeline per file (`${item.in}`, `${item.out}` = out-dir/stem.ext), or a command
template: `--cmd 'export {in} -o out/{stem}.webp'` (`{in} {stem} {ext} {dir} {n} {out}`).
Per-item results stream as NDJSON with `--progress json`; the envelope lists every item.
A failed item makes the exit code non-zero unless `--continue-on-error`.
