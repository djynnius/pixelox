# Getting started

## Install

```bash
cargo install --path crates/pixelox-cli     # from a checkout
pixelox doctor                              # what this build can do
```

## Your first edit

Every command works on plain images: write the result with `-o`.

```bash
pixelox info photo.jpg
pixelox filter gaussian photo.jpg -o soft.jpg --sigma 3
pixelox transform resize photo.jpg -o small.jpg --size 1600x
```

The input is never changed. Each output gets a sidecar, `soft.jpg.pixelox.json`, that
records exactly how it was made.

## Work in a document

For more than one step, open the image as a document. A `.pxo` keeps layers, masks, the
selection, and the full history; `--in-place` edits it.

```bash
pixelox open photo.jpg -o photo.pxo
pixelox adjust curves photo.pxo --in-place --points '0,0 64,56 192,206 255,255'
pixelox text add photo.pxo --in-place --name Caption --content "Lagos, 2026" --font sans-serif --size 36pt --color white --at 50%,-40 --anchor b
pixelox layer list photo.pxo
pixelox export photo.pxo -o photo-final.jpg --quality 90
```

## For scripts and agents

Add `--json` to get one envelope on stdout; plan with `--dry-run`:

```bash
pixelox filter unsharp photo.pxo --amount 80 --radius 1.5 --dry-run --json
pixelox describe --agent        # the whole command surface, compactly
pixelox docs contract           # the rules every command follows
```

Exit codes say what happened (4: the output exists; pass `--overwrite` to replace it).
`pixelox docs errors` lists every error code.

## Check your work

```bash
pixelox analyze stats photo-final.jpg --json
pixelox verify photo-final.jpg.pixelox.json
pixelox replay photo-final.jpg.pixelox.json -o again.jpg
```
