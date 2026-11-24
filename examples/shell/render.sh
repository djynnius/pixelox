#!/usr/bin/env bash
# E9/E10: a data chart drawn through the sandboxed DOM, the same chart and a U.S. map with
# the bundled d3 and atlases, and a 3D wireframe of the extruded logo.
#   render.sh [SAMPLES_DIR] [OUT_DIR]
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
// A bar chart with the DOM-lite only (render d3 below does the same with d3's scales).
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

cat > "$OUT/d3chart.js" <<'JS'
// The same chart with the bundled d3 (no --lib): scales and axes.
const rows = px.data.rows;
const doc = px.svg(640, 360);
const svg = d3.select(doc.documentElement);
const x = d3.scaleBand().domain(rows.map(r => r.month)).range([50, 620]).padding(0.2);
const y = d3.scaleLinear().domain([0, d3.max(rows, r => +r.sales)]).nice().range([320, 20]);
svg.selectAll("rect").data(rows).join("rect")
  .attr("x", r => x(r.month)).attr("y", r => y(+r.sales))
  .attr("width", x.bandwidth()).attr("height", r => 320 - y(+r.sales)).attr("fill", "#0ea5e9");
svg.append("g").attr("transform", "translate(50,0)").call(d3.axisLeft(y).ticks(5));
svg.append("g").attr("transform", "translate(0,320)").call(d3.axisBottom(x));
px.output(doc);
JS
pxx d3chart render d3 "$OUT/d3chart.js" -o "$OUT/d3chart.png" --data rows="$OUT/sales.csv"
note "render d3: $(jget "$ENV" .result.libraries[0].name) $(jget "$ENV" .result.libraries[0].version)"

cat > "$OUT/usmap.js" <<'JS'
// The 50 states and DC from the bundled atlas, Alaska and Hawaii as insets.
const us = px.atlas("us-states");
const states = topojson.feature(us, us.objects.states);
const path = d3.geoPath(d3.geoAlbersUsa().fitSize([960, 600], states));
const doc = px.svg(960, 600);
d3.select(doc.documentElement).selectAll("path").data(states.features.filter(f => path(f))).join("path")
  .attr("d", path).attr("fill", f => f.properties.postal < "N" ? "#93c5fd" : "#1d4ed8")
  .attr("stroke", "#ffffff").attr("stroke-width", 0.8);
px.output(doc);
JS
pxx usmap render d3 "$OUT/usmap.js" -o "$OUT/usmap.png"
pxx usmapverify verify "$OUT/usmap.png.pixelox.json" --full
note "U.S. map verified: $(jget "$ENV" .result.full.identical)"

pxx wire render 3d --extrude "$SAMPLES/logo.svg" --depth 40 --camera iso --style hidden-line --line-width 1.5 -o "$OUT/logo-wire.svg"
pxx shaded render 3d --primitive torus --camera persp --fov 40 --style shaded -o "$OUT/torus.png" --size 600x400
note "3D: $(jget "$ENV" .outputs[0].path)"
