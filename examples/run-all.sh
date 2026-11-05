#!/usr/bin/env bash
# Run every example against freshly generated samples and fail if any fails.
#   PIXELOX=target/debug/pixelox examples/run-all.sh [WORK_DIR]
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
PIXELOX=${PIXELOX:-pixelox}
if [[ $PIXELOX == */* ]]; then PIXELOX=$(cd "$(dirname "$PIXELOX")" && pwd)/$(basename "$PIXELOX"); fi
export PIXELOX PYTHONDONTWRITEBYTECODE=1
command -v "$PIXELOX" >/dev/null || { echo "run-all: pixelox not found ($PIXELOX)" >&2; exit 2; }
created=0
if [[ -n ${1:-} ]]; then WORK=$1; mkdir -p "$WORK"; else WORK=$(mktemp -d "${TMPDIR:-/tmp}/pixelox-examples.XXXXXX"); created=1; fi
WORK=$(cd "$WORK" && pwd)
export PIXELOX_HOME=$WORK/pixelox-home          # private config/state: never touches yours
export PIXELOX_SYSTEM_FONTS=0                   # the embedded font: identical output everywhere
LOGS=$WORK/logs
mkdir -p "$LOGS"
echo "pixelox: $("$PIXELOX" --version)  work: $WORK"

results=()
failed=0
step() {
  local name=$1; shift
  local start=$SECONDS rc=0 log
  log=$LOGS/$(printf '%s' "$name" | tr -c 'A-Za-z0-9.-' '_').log
  printf '%-44s ' "$name"
  "$@" >"$log" 2>&1 || rc=$?
  if [[ $rc -eq 0 ]]; then echo "ok    $((SECONDS - start))s"; results+=("ok|$name")
  else echo "FAIL  (exit $rc) $log"; tail -25 "$log" | sed 's/^/    | /'; results+=("FAIL|$name"); failed=$((failed + 1)); fi
}

pipelines() {
  cd "$WORK"
  mkdir -p brand shoot work
  cp "$here"/pipelines/*.yml .
  "$PIXELOX" svg rasterize samples/logo.svg -o brand/logo.png --size 220x220 -q --overwrite
  cp samples/photo.jpg shoot/a.jpg
  cp samples/product.png shoot/b.png
  "$PIXELOX" pipeline validate poster.yml --json >"$LOGS/poster.validate.json"
  "$PIXELOX" pipeline run poster.yml --var headline="WINTER DROP 01" --json >"$LOGS/poster.run.json"
  python3 -c 'import json,sys; e=json.load(open(sys.argv[1])); assert e["ok"], e.get("error"); print("poster outputs:", e["result"]["outputs"])' "$LOGS/poster.run.json"
  "$PIXELOX" pipeline graph poster.yml --format dot >"$LOGS/poster.dot"
  "$PIXELOX" new -o work/card.pxo --preset og-card --fill '#1e293b' -q --overwrite
  "$PIXELOX" pipeline run social-cards.yml --jobs 2 --json >"$LOGS/cards.json"
  ls dist/cards/ada.png dist/cards/grace.png
  "$PIXELOX" batch run --glob 'shoot/*' --pipeline watermark.yml --out-dir out --jobs 2 --manifest out/manifest.json --json >"$LOGS/batch.json"
  python3 -c 'import json,sys; e=json.load(open(sys.argv[1])); assert e["ok"], e.get("error"); r=e["result"]; print(r["succeeded"], "watermarked;", r["failed"], "failed")' "$LOGS/batch.json"
  "$PIXELOX" batch run --glob 'out/*.jpg' --cmd 'export {in} -o web/{stem}.webp' --json >"$LOGS/batch-cmd.json"
  ls web/a.webp
  "$PIXELOX" pipeline new -o again.yml --from-sidecar dist/poster.png.pixelox.json --json >/dev/null
  "$PIXELOX" pipeline validate again.yml --json >/dev/null
}

step make-samples "$here/make-samples.sh" "$WORK/samples"
for ex in photo-edit cutout sticker poster-template vector-logo inspect-and-verify frames render; do
  step "shell/$ex.sh" "$here/shell/$ex.sh" "$WORK/samples" "$WORK/out/$ex"
done
step "pipelines + batch" pipelines
step "mcp/client.py" python3 "$here/mcp/client.py" --samples "$WORK/samples" --out "$WORK/out/mcp"
step "agents/parse_envelope.py" python3 "$here/agents/parse_envelope.py" --samples "$WORK/samples" --out "$WORK/out/py"

echo
echo "== summary: $(( ${#results[@]} - failed )) ok, $failed failed; logs in $LOGS"
[[ $failed -eq 0 ]] || exit 1
[[ $created -eq 1 ]] && rm -rf "$WORK"
exit 0
