#!/usr/bin/env bash
# E1 without models: cut a product out of a plain backdrop with a magic-wand selection,
# then place it on a new background.   cutout.sh [SAMPLES_DIR] [OUT_DIR]
set -euo pipefail
source "$(dirname "$0")/common.sh"
setup_dirs "${1:-}" "${2:-}"
D=$OUT/p.pxo

say "the AI path is planned only in this build"
px plan detect subject "$SAMPLES/product.png" --cutout "$OUT/never.png" --dry-run
note "dry run ok=$(jget "$ENV" .ok); model: $(jget "$ENV" .result.plan.models_required[0].name)"
px real detect subject "$SAMPLES/product.png" --cutout "$OUT/never.png"
expect_eq "$RC" "12" "detect exits 12 without ML"
note "real run: $(jget "$ENV" .error.code) (exit $RC)"

say "select the backdrop, invert, refine"
pxx open open "$SAMPLES/product.png" -o "$D"
pxx wand select color "$D" --in-place --ref white --at 4,4 --contiguous --tolerance 40
pxx inv select invert "$D" --in-place
pxx feather mask feather "$D" --in-place --mask @sel --radius 1.5
pxx save select save "$D" --in-place --as product
pxx bounds select bounds "$D"
note "product covers $(jget "$ENV" .result.coverage)% of the canvas, bounds $(jget "$ENV" .result.bounds)"

say "export the cutout and composite it"
pxx cut export "$D" -o "$OUT/product-cut.png" --apply-mask product
pxx bg new -o "$OUT/ad.pxo" --preset ig-post --fill '#f4efe6'
pxx place comp place "$OUT/ad.pxo" "$OUT/product-cut.png" --in-place --as Product --fit contain
pxx shadow fx drop-shadow "$OUT/ad.pxo" --in-place --layer Product --offset 0,18 --blur 30 --color '#00000055'
pxx ad export "$OUT/ad.pxo" -o "$OUT/ad.png"
note "ad: $(jget "$ENV" .outputs[0].path)"
