#!/usr/bin/env bash
# E11: a sticker — star burst, text on a circular path, layer styles, transparent PNG.
#   sticker.sh [SAMPLES_DIR] [OUT_DIR]
set -euo pipefail
source "$(dirname "$0")/common.sh"
setup_dirs "${1:-}" "${2:-}"
B=$OUT/badge.pxo

pxx new new -o "$B" --size 1000x1000 --fill transparent
pxx star shape star "$B" --in-place --center 500,500 --points 12 --outer 460 --inner 380 --fill '#ffd400' --stroke '#111111' --stroke-width 10 --name Burst
pxx arc text add "$B" --in-place --name Arc --content "LIMITED EDITION • SUMMER DROP • " --font sans-serif --weight 800 --size 54pt --color '#111111' --tracking 60
pxx path text on-path "$B" --in-place --layer Arc --d 'M 500,180 A 320,320 0 1 1 499.9,180'
pxx center text add "$B" --in-place --name Center --content "-30%" --font sans-serif --weight bold --size 150pt --color '#111111' --at 50%,50% --anchor c
pxx stroke fx stroke "$B" --in-place --layer Arc --width 6 --position outside --color '#ffffff'
pxx shadow fx drop-shadow "$B" --in-place --layer Burst --offset 0,10 --blur 24 --color '#00000066'
pxx png export "$B" -o "$OUT/badge.png"
pxx stats analyze stats "$OUT/badge.png"
note "badge.png alpha: $(jget "$ENV" .result.alpha)"
