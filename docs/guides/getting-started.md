# Getting started

## Install

```bash
# macOS and Linux
curl -fsSL https://raw.githubusercontent.com/djynnius/pixelox/main/install.sh | sh
# Windows (PowerShell)
powershell -ExecutionPolicy ByPass -c "irm https://raw.githubusercontent.com/djynnius/pixelox/main/install.ps1 | iex"
# from a checkout
cargo install --path crates/pixelox-cli
pixelox doctor                              # what this build can do
pixelox update --check                      # is a newer release out? (pixelox update installs it)
```

The installers pick the right build, verify its SHA-256, install to `~/.pixelox/bin`, and
add it to your PATH. Options go on the `sh` side of the pipe (in PowerShell, set them with
`$env:` first): `PIXELOX_VERSION=0.2.0` pins a release, `PIXELOX_INSTALL_DIR=~/bin` installs
elsewhere, and `PIXELOX_NO_MODIFY_PATH=1` leaves your shell profiles alone, e.g.
`curl -fsSL https://raw.githubusercontent.com/djynnius/pixelox/main/install.sh | PIXELOX_NO_MODIFY_PATH=1 sh`.

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
