#!/usr/bin/env bash
# Animated GIF: assemble stills, inspect, extract a frame, and explode it back to stills.
#   frames.sh [SAMPLES_DIR] [OUT_DIR]
set -euo pipefail
source "$(dirname "$0")/common.sh"
setup_dirs "${1:-}" "${2:-}"

pxx gif frames assemble "$SAMPLES/frames/f1.png" "$SAMPLES/frames/f2.png" "$SAMPLES/frames/f3.png" "$SAMPLES/frames/f4.png" -o "$OUT/ball.gif" --fps 8 --colors 16 --dither none
pxx info info "$OUT/ball.gif"
note "frames: $(jget "$ENV" .result.file.frames)"
expect_eq "$(jget "$ENV" .result.file.frames)" "4" "frame count"
pxx third open "$OUT/ball.gif" -o "$OUT/frame3.png" --frame 3
pxx explode frames extract "$OUT/ball.gif" -o "$OUT/stills"
note "extracted $(jget "$ENV" .result.frames) stills into $OUT/stills"
