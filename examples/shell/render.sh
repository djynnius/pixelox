#!/usr/bin/env bash
# E9/E10 without vendored libraries: a data chart drawn through the sandboxed DOM, and a
# 3D wireframe of the extruded logo.   render.sh [SAMPLES_DIR] [OUT_DIR]
set -euo pipefail
source "$(dirname "$0")/common.sh"
setup_dirs "${1:-}" "${2:-}"

cat > "$OUT/sales.csv" <<'CSV'
month,sales
Jan,120
Feb,180
Mar,150
Apr,240
May,210
Jun,300
CSV
cat > "$OUT/chart.js" <<'JS'
// A bar chart with the DOM-lite only (d3 is optional: render d3 --lib d3.min.js).
const rows = px.data.rows;
const doc = px.svg(640, 360);
const svg = doc.documentElement;
const max = Math.max(...rows.map(r => +r.sales));
const bw = 560 / rows.length;
rows.forEach((r, i) => {
  const h = (+r.sales / max) * 280;
  const rect = doc.createElementNS("http://www.w3.org/2000/svg", "rect");
  rect.setAttribute("x", 50 + i * bw + 6);
  rect.setAttribute("y", 320 - h);
  rect.setAttribute("width", bw - 12);
  rect.setAttribute("height", h);
  rect.setAttribute("fill", px.params.color || "#0ea5e9");
  svg.appendChild(rect);
});
px.output(doc);
JS
pxx chart render js "$OUT/chart.js" -o "$OUT/chart.svg" --size 640x360 --data rows="$OUT/sales.csv" --param color=#f97316
pxx chartpng svg rasterize "$OUT/chart.svg" -o "$OUT/chart@2x.png" --scale 2
note "chart: $(jget "$ENV" .outputs[0].size)"

px nod3 render d3 "$OUT/chart.js" -o "$OUT/never.svg" --data rows="$OUT/sales.csv"
note "render d3 without --lib: $(jget "$ENV" .error.code) (exit $RC)"

pxx wire render 3d --extrude "$SAMPLES/logo.svg" --depth 40 --camera iso --style hidden-line --line-width 1.5 -o "$OUT/logo-wire.svg"
pxx shaded render 3d --primitive torus --camera persp --fov 40 --style shaded -o "$OUT/torus.png" --size 600x400
note "3D: $(jget "$ENV" .outputs[0].path)"
