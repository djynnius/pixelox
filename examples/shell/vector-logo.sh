#!/usr/bin/env bash
# E12: vectorize a scanned mark, recolor it, optimize the SVG, and measure the trace's
# fidelity against the scan.   vector-logo.sh [SAMPLES_DIR] [OUT_DIR]
set -euo pipefail
source "$(dirname "$0")/common.sh"
setup_dirs "${1:-}" "${2:-}"

pxx trace trace "$SAMPLES/scan.png" -o "$OUT/logo.svg" --mode binary --threshold 128 --speckle 8
note "paths: $(jget "$ENV" .result.paths)"
pxx recolor svg el set "$OUT/logo.svg" --in-place --where path --attr fill=#0b5fff
pxx opt svg optimize "$OUT/logo.svg" -o "$OUT/logo.min.svg" --precision 2
pxx info info "$SAMPLES/scan.png"
SIZE="$(jget "$ENV" .result.size[0])x$(jget "$ENV" .result.size[1])"
pxx raster svg rasterize "$OUT/logo.min.svg" -o "$OUT/check.png" --size "$SIZE" --background-color white
pxx gray adjust bw "$OUT/check.png" -o "$OUT/check-gray.png"
pxx bin adjust threshold "$OUT/check-gray.png" -o "$OUT/check-bin.png" --level 200
pxx diff analyze diff "$SAMPLES/scan.png" "$OUT/check-bin.png"
note "trace fidelity: SSIM $(jget "$ENV" .result.ssim), changed $(jget "$ENV" .result.changed_percent)%"
