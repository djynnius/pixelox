#!/usr/bin/env bash
# The agent contract in action: envelopes, exit codes, dry runs, sidecars, replay, verify,
# tamper detection, and history.   inspect-and-verify.sh [SAMPLES_DIR] [OUT_DIR]
set -euo pipefail
source "$(dirname "$0")/common.sh"
setup_dirs "${1:-}" "${2:-}"
cd "$OUT"
cp "$SAMPLES/photo.png" photo.png

say "one-shot edit; the input is untouched"
pxx blur filter gaussian photo.png -o soft.png --sigma 3
note "wrote $(jget "$ENV" .outputs[0].path) sha256 $(jget "$ENV" .outputs[0].sha256)"

say "exit codes"
px again filter gaussian photo.png -o soft.png --sigma 3
expect_eq "$RC" "4" "refuse to overwrite"
note "$(jget "$ENV" .error.code) (exit $RC): $(jget "$ENV" .error.hint)"
px inplace filter gaussian photo.png --in-place --sigma 3
expect_eq "$(jget "$ENV" .error.code)" "PX1108" "never edit sources in place"
px badflag filter gaussian photo.png -o x.png --sigma -4
note "bad value: $(jget "$ENV" .error.code) (exit $RC)"
px missing info nope.png
expect_eq "$RC" "3" "missing input"

say "dry run writes nothing"
pxx plan filter gaussian photo.png -o planned.png --sigma 3 --dry-run
[[ ! -e planned.png ]] || { echo "dry run wrote a file" >&2; exit 1; }
note "plan: $(jget "$ENV" .result.plan.outputs_predicted)"

say "documents keep history"
pxx open open photo.png -o doc.pxo
pxx a adjust posterize doc.pxo --in-place --levels 6
pxx b filter pixelate doc.pxo --in-place --cell 12
pxx hist history list doc.pxo
note "$(jget "$ENV" .result.text)"
pxx undo history undo doc.pxo --in-place
pxx out export doc.pxo -o posterized.png

say "replay and verify"
pxx replay replay posterized.png.pixelox.json -o posterized-again.png
expect_eq "$(jget "$ENV" .result.identical)" "true" "bit-identical replay"
pxx verify verify posterized.png.pixelox.json --full
printf 'x' >> posterized.png
px tamper verify posterized.png.pixelox.json
expect_eq "$RC" "9" "verify detects a changed output"
note "after tampering: $(jget "$ENV" .error.code) (exit $RC)"
