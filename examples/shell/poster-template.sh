#!/usr/bin/env bash
# E5: build a template with typed slots, fill it, and export a print and an OG card.
#   poster-template.sh [SAMPLES_DIR] [OUT_DIR]
set -euo pipefail
source "$(dirname "$0")/common.sh"
setup_dirs "${1:-}" "${2:-}"
T=$OUT/poster.pxo

say "the template"
pxx new new -o "$T" --size 1200x1600 --fill '#101014'
pxx accent shape rect "$T" --in-place --rect 0,0,100%x10% --fill '#0ea5e9' --name Accent
pxx headline text add "$T" --in-place --name headline --content "HEADLINE" --font sans-serif --weight bold --size 96pt --color white --at 50%,14% --anchor t --box 1080x320 --align center
pxx product comp place "$T" "$SAMPLES/product.png" --in-place --as product --at 50%,64% --anchor c --size 760x760
pxx init comp template init "$T" --in-place --slot text:headline:required:max-lines=2 --slot image:product:required:fit=contain --slot color:accent:layer=Accent
pxx slots comp template slots "$T"
note "slots: $(jget "$ENV" .result.slots)"

say "a missing required slot is an error"
px missing comp template apply "$T" --set headline="NO PRODUCT" -o "$OUT/bad.pxo"
note "$(jget "$ENV" .error.code): $(jget "$ENV" .error.message)"
expect_eq "$(jget "$ENV" .error.code)" "PX3202" "missing slot"

say "fill and export"
pxx apply comp template apply "$T" --set headline="FALL DROP 09" --set-image product="$SAMPLES/product.png" --set-color accent='#f97316' -o "$OUT/fall.pxo"
pxx print export "$OUT/fall.pxo" -o "$OUT/fall-poster.png"
pxx og export "$OUT/fall.pxo" -o "$OUT/fall-og.png" --size 1200x630 --region 0,300,1200x630
expect_eq "$(jget "$ENV" .outputs[0].size)" "[1200,630]" "og size"
