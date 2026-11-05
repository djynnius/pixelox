#!/usr/bin/env bash
# E4: rescue an underexposed photo, grade it non-destructively, deliver a 16-bit TIFF and a
# web JPEG, and check the result.   photo-edit.sh [SAMPLES_DIR] [OUT_DIR]
set -euo pipefail
source "$(dirname "$0")/common.sh"
setup_dirs "${1:-}" "${2:-}"
D=$OUT/shot.pxo

say "open as a document and grade it"
pxx open open "$SAMPLES/photo.jpg" -o "$D"
pxx wb adjust white-balance "$D" --in-place --auto
note "white balance resolved to temp=$(jget "$ENV" .result.params.temp) tint=$(jget "$ENV" .result.params.tint)"
pxx sh adjust shadows-highlights "$D" --in-place --shadows 45 --highlights 20
pxx curves adjust curves "$D" --in-place --points '0,8 96,118 255,246'
pxx vib adjust vibrance "$D" --in-place --vibrance 25 --as-layer
pxx sharpen filter unsharp "$D" --in-place --layer Background --amount 60 --radius 1
pxx layers layer list "$D"
note "layers: $(jget "$ENV" .result.count)"

say "deliver"
pxx tiff export "$D" -o "$OUT/photo-final.tiff" --bit-depth 16
pxx web export "$D" -o "$OUT/photo-web.jpg" --size 640x --quality 88
expect_eq "$(jget "$ENV" .outputs[0].size)" "[640,427]" "web size"

say "check"
pxx stats analyze stats "$OUT/photo-final.tiff"
note "mean luma before/after:"
pxx before analyze stats "$SAMPLES/photo.jpg"
pxx verify verify "$OUT/photo-final.tiff.pixelox.json"
expect_eq "$(jget "$ENV" .result.verified)" "true" "verify"
note "the TIFF's sidecar replays the whole grade from photo.jpg: $(jget "$ENV" .result.files[1].path)"
