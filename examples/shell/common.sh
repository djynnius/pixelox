# Shared helpers for the shell examples. Source it; do not run it.
#
# Every example takes the same optional arguments:  example.sh [SAMPLES_DIR] [OUT_DIR]
# Without them it makes fresh samples and an output folder in a temp dir.

PIXELOX=${PIXELOX:-pixelox}
EXAMPLES_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)

say() { printf '\n== %s\n' "$*"; }
note() { printf '   %s\n' "$*"; }

setup_dirs() {
  SAMPLES=${1:-}
  OUT=${2:-}
  if [[ -z $SAMPLES ]]; then
    SAMPLES=$(mktemp -d "${TMPDIR:-/tmp}/pixelox-samples.XXXXXX")
    "$EXAMPLES_DIR/make-samples.sh" "$SAMPLES" >/dev/null
  fi
  [[ -n $OUT ]] || OUT=$(mktemp -d "${TMPDIR:-/tmp}/pixelox-example.XXXXXX")
  mkdir -p "$OUT/json"
  SAMPLES=$(cd "$SAMPLES" && pwd)
  OUT=$(cd "$OUT" && pwd)
  note "samples: $SAMPLES"
  note "output:  $OUT"
}

# px NAME ARGS...: run `pixelox ARGS... --json`, keep the envelope in $OUT/json/NAME.json.
# Sets RC; never aborts, so the caller decides what an exit code means.
px() {
  local name=$1
  shift
  RC=0
  "$PIXELOX" "$@" --json >"$OUT/json/$name.json" 2>"$OUT/json/$name.log" || RC=$?
  ENV="$OUT/json/$name.json"
}

# pxx NAME ARGS...: like px, but stop the example on failure, printing message and hint.
pxx() {
  px "$@"
  if [[ $RC -ne 0 ]]; then
    echo "pixelox ${*:2} failed with exit code $RC:" >&2
    echo "  $(jget "$ENV" .error.code): $(jget "$ENV" .error.message)" >&2
    echo "  hint: $(jget "$ENV" .error.hint)" >&2
    exit "$RC"
  fi
}

# jget FILE PATH: one value from a JSON file (jq when installed, else python3).
jget() {
  if command -v jq >/dev/null 2>&1; then
    jq -c -r "$2" "$1"
  else
    python3 - "$1" "$2" <<'PY'
import json, re, sys
v = json.load(open(sys.argv[1]))
for key, idx in re.findall(r'\.([A-Za-z_][\w-]*)|\[(\d+)\]', sys.argv[2]):
    if key:
        v = v.get(key) if isinstance(v, dict) else None
    else:
        v = v[int(idx)] if isinstance(v, list) and int(idx) < len(v) else None
    if v is None:
        break
print("null" if v is None else (str(v).lower() if isinstance(v, bool) else (json.dumps(v, separators=(",", ":")) if isinstance(v, (dict, list)) else v)))
PY
  fi
}

expect_eq() {
  if [[ "$1" != "$2" ]]; then
    echo "expected '$2', got '$1' ($3)" >&2
    exit 1
  fi
}
