# Charts and maps with d3 and three.js

`render d3` runs a script with the bundled **d3 7.9** and **topojson-client 3.1** already
loaded, and `render three` with the bundled **three.js r187**: no `--lib`, no downloads.
Scripts draw into `px.svg()`, end with `px.output()`, and pixelox writes SVG or a PNG.
Maps come from the bundled atlases through `px.atlas(name)`: world countries (Natural
Earth) and U.S. states (Census Bureau), with codes for joining your data
(`pixelox docs atlas` lists every name and code). Every run is deterministic, recorded in
the sidecar with the library versions and hashes, and rebuilt bit for bit by
`pixelox replay`.

Each recipe below is a script file and a command. Write the files (the data files too),
then run the commands from the same folder.

## A bar chart with axes

<!-- file: sales.csv -->
```csv
month,sales
Jan,120
Feb,95
Mar,140
Apr,180
May,160
Jun,210
```

<!-- file: bar.js -->
```js
// A bar chart with a gridded y axis, value labels, and a title.
const rows = px.data.rows;                        // CSV rows: strings, so use +d.sales
const W = 960, H = 540, m = { top: 70, right: 30, bottom: 50, left: 60 };
const doc = px.svg(W, H);
const svg = d3.select(doc.documentElement).attr('font-family', 'Helvetica, Arial, sans-serif');
svg.append('rect').attr('width', W).attr('height', H).attr('fill', '#ffffff');
const x = d3.scaleBand().domain(rows.map(d => d.month)).range([m.left, W - m.right]).padding(0.25);
const y = d3.scaleLinear().domain([0, d3.max(rows, d => +d.sales)]).nice().range([H - m.bottom, m.top]);
svg.append('g').attr('transform', `translate(${m.left},0)`)
  .call(d3.axisLeft(y).ticks(5).tickSize(-(W - m.left - m.right)))
  .call(g => g.select('.domain').remove())
  .call(g => g.selectAll('.tick line').attr('stroke', '#e5e7eb'))
  .call(g => g.selectAll('text').attr('font-size', 14).attr('fill', '#6b7280'));
svg.append('g').selectAll('rect').data(rows).join('rect')
  .attr('x', d => x(d.month)).attr('y', d => y(+d.sales))
  .attr('width', x.bandwidth()).attr('height', d => y(0) - y(+d.sales))
  .attr('rx', 4).attr('fill', '#0ea5e9');
svg.append('g').selectAll('text').data(rows).join('text')
  .attr('x', d => x(d.month) + x.bandwidth() / 2).attr('y', d => y(+d.sales) - 8)
  .attr('text-anchor', 'middle').attr('font-size', 15).attr('font-weight', 700).attr('fill', '#0f172a')
  .text(d => d3.format(',')(+d.sales));
svg.append('g').attr('transform', `translate(0,${H - m.bottom})`)
  .call(d3.axisBottom(x).tickSizeOuter(0))
  .call(g => g.selectAll('text').attr('font-size', 15).attr('fill', '#374151'));
svg.append('text').attr('x', m.left).attr('y', 38)
  .attr('font-size', 26).attr('font-weight', 700).attr('fill', '#0f172a').text('Monthly sales, first half');
px.output(doc);
```

```bash
pixelox render d3 bar.js --data rows=sales.csv -o bar.png
pixelox render d3 bar.js --data rows=sales.csv -o bar.svg                         # the vector original
pixelox render d3 bar.js --data rows=sales.csv -o bar@2x.png --size 1920x1080     # rasterized at 2x
```

A PNG is drawn at the SVG's own size; `--size` scales it. `<text>` uses the installed
fonts (d3's axes ask for `sans-serif`, which maps to Arial, Helvetica, or another installed
sans); `pixelox fonts get Inter -y` (with consent) adds a family that `font-family` can
then name.

## A donut with labels and a legend

<!-- file: share.csv -->
```csv
source,value
Search,4820
Direct,2950
Social,1830
Email,1240
Referral,760
```

<!-- file: donut.js -->
```js
// A donut chart: d3.pie + d3.arc, percentage labels on the ring, a legend, and the total.
const rows = px.data.share;                       // columns: source,value
const W = 900, H = 560, cx = 300, cy = 290, R = 220, r = 130;
const doc = px.svg(W, H);
const svg = d3.select(doc.documentElement).attr('font-family', 'Helvetica, Arial, sans-serif');
svg.append('rect').attr('width', W).attr('height', H).attr('fill', '#ffffff');
const total = d3.sum(rows, d => +d.value);
const color = d3.scaleOrdinal(rows.map(d => d.source), ['#2563eb', '#f59e0b', '#10b981', '#ef4444', '#8b5cf6', '#64748b']);
const pie = d3.pie().value(d => +d.value).sort(null).padAngle(0.012);
const arc = d3.arc().innerRadius(r).outerRadius(R).cornerRadius(3);
const label = d3.arc().innerRadius((r + R) / 2).outerRadius((r + R) / 2);
const g = svg.append('g').attr('transform', `translate(${cx},${cy})`);
const slices = pie(rows);
g.selectAll('path').data(slices).join('path').attr('d', arc).attr('fill', d => color(d.data.source));
g.selectAll('text').data(slices.filter(d => d.endAngle - d.startAngle > 0.25)).join('text')
  .attr('transform', d => `translate(${label.centroid(d)})`)
  .attr('text-anchor', 'middle').attr('dominant-baseline', 'central')
  .attr('font-size', 18).attr('font-weight', 700).attr('fill', '#ffffff')
  .text(d => d3.format('.0%')(+d.data.value / total));
g.append('text').attr('text-anchor', 'middle').attr('y', -8).attr('font-size', 44).attr('font-weight', 800)
  .attr('fill', '#0f172a').text(d3.format(',')(total));
g.append('text').attr('text-anchor', 'middle').attr('y', 26).attr('font-size', 17).attr('fill', '#64748b').text('visits this week');
const legend = svg.append('g').attr('transform', 'translate(590,150)');
const item = legend.selectAll('g').data(rows).join('g').attr('transform', (d, i) => `translate(0,${i * 44})`);
item.append('rect').attr('width', 22).attr('height', 22).attr('rx', 5).attr('fill', d => color(d.source));
item.append('text').attr('x', 34).attr('y', 16).attr('font-size', 19).attr('fill', '#0f172a').text(d => d.source);
item.append('text').attr('x', 270).attr('y', 16).attr('text-anchor', 'end').attr('font-size', 19)
  .attr('fill', '#64748b').text(d => d3.format(',')(+d.value));
px.output(doc);
```

```bash
pixelox render d3 donut.js --data share=share.csv -o donut.png
```

Labels go only on slices wide enough to hold them (`endAngle - startAngle > 0.25`); the
legend carries every value.

## A U.S. choropleth

`px.atlas('us-states')` is TopoJSON; `topojson.feature` turns its `states` object into
GeoJSON features, each with `id` (the FIPS code, `"06"`) and `properties.name`,
`properties.postal` (`"CA"`), and `properties.fips`. `d3.geoAlbersUsa()` places Alaska and
Hawaii as insets. Join your data on the postal code (or the name).

<!-- file: scores.csv -->
```csv
state,score
AL,3.1
AK,4.6
AZ,3.9
AR,3.3
CA,5.3
CO,3.4
CT,4.2
DE,4.0
DC,5.1
FL,3.0
GA,3.5
HI,3.0
ID,3.2
IL,4.7
IN,3.6
IA,2.9
KS,2.8
KY,4.2
LA,4.4
ME,3.0
MD,2.9
MA,3.8
MI,4.1
MN,3.0
MS,3.2
MO,3.4
MT,3.1
NE,2.5
NV,5.4
NH,2.6
NJ,4.6
NM,4.0
NY,4.4
NC,3.6
ND,2.3
OH,4.0
OK,3.3
OR,4.1
PA,3.9
RI,4.3
SC,3.0
SD,2.1
TN,3.4
TX,4.1
UT,2.8
VT,2.3
VA,2.9
WA,4.5
WV,4.1
WI,3.3
WY,3.0
```

<!-- file: choropleth.js -->
```js
// A U.S. choropleth: px.atlas('us-states') joined to a CSV on the state's postal code.
const W = 975, H = 640;
const score = new Map(px.data.scores.map(d => [d.state, +d.score]));   // columns: state (CA), score
const us = px.atlas('us-states');
const states = topojson.feature(us, us.objects.states);
const projection = d3.geoAlbersUsa().fitExtent([[10, 70], [W - 10, H - 10]], states);
const path = d3.geoPath(projection);
const color = d3.scaleQuantize([2, 6], d3.schemeBlues[8]);

const doc = px.svg(W, H);
const svg = d3.select(doc.documentElement).attr('font-family', 'Helvetica, Arial, sans-serif');
svg.append('rect').attr('width', W).attr('height', H).attr('fill', '#ffffff');
svg.append('g').selectAll('path')
  .data(states.features.filter(f => path(f)))      // geoAlbersUsa draws the 50 states and DC
  .join('path')
  .attr('d', path)
  .attr('fill', f => score.has(f.properties.postal) ? color(score.get(f.properties.postal)) : '#e5e7eb');
svg.append('path')                                  // borders between states, drawn once
  .attr('d', path(topojson.mesh(us, us.objects.states, (a, b) => a !== b)))
  .attr('fill', 'none').attr('stroke', '#ffffff').attr('stroke-width', 0.8).attr('stroke-linejoin', 'round');
svg.append('text').attr('x', 20).attr('y', 40).attr('font-size', 26).attr('font-weight', 700)
  .attr('fill', '#0f172a').text('Survey score by state (sample data)');

// legend: one swatch per color step
const legend = svg.append('g').attr('transform', `translate(${W - 340},28)`);
const steps = color.range().map(c => color.invertExtent(c));
legend.selectAll('rect').data(steps).join('rect')
  .attr('x', (d, i) => i * 40).attr('width', 40).attr('height', 12).attr('fill', d => color(d[0]));
legend.selectAll('text').data(steps).join('text')
  .attr('x', (d, i) => i * 40).attr('y', 30).attr('font-size', 12).attr('fill', '#475569')
  .attr('text-anchor', 'middle').text(d => d[0]);
px.output(doc);
```

```bash
pixelox render d3 choropleth.js --data scores=scores.csv -o choropleth.png
```

`us-states-albers` is the same map already projected into a 975 × 610 frame: draw it with
`d3.geoPath()` and no projection. `us-nation` is the outline alone.

## A world map with highlighted countries

World countries carry `id` (ISO 3166-1 numeric, `"840"`), `properties.name` (Natural
Earth's short name), and `properties.iso_a2` / `iso_a3` (`"US"` / `"USA"`): join on
`iso_a3`.

<!-- file: world.js -->
```js
// A world map with a highlighted set of countries (ISO 3166 alpha-3 codes) and their names.
const W = 1200, H = 640;
const pick = new Set((px.params.highlight || 'BRA,NGA,IND,IDN,DEU,AUS').split(','));
const world = px.atlas('world-110m');
const countries = topojson.feature(world, world.objects.countries);
const projection = d3.geoEqualEarth().fitExtent([[20, 70], [W - 20, H - 20]], { type: 'Sphere' });
const path = d3.geoPath(projection);

const doc = px.svg(W, H);
const svg = d3.select(doc.documentElement).attr('font-family', 'Helvetica, Arial, sans-serif');
svg.append('rect').attr('width', W).attr('height', H).attr('fill', '#ffffff');
svg.append('path').attr('d', path({ type: 'Sphere' })).attr('fill', '#eef6fb');
svg.append('path').attr('d', path(d3.geoGraticule10())).attr('fill', 'none').attr('stroke', '#d7e6ef').attr('stroke-width', 0.6);
svg.append('g').selectAll('path').data(countries.features).join('path')
  .attr('d', path)
  .attr('fill', f => pick.has(f.properties.iso_a3) ? '#e11d48' : '#cbd5e1')
  .attr('stroke', '#ffffff').attr('stroke-width', 0.5);
svg.append('g').selectAll('text').data(countries.features.filter(f => pick.has(f.properties.iso_a3))).join('text')
  .attr('transform', f => `translate(${path.centroid(f)})`)
  .attr('text-anchor', 'middle').attr('dy', '0.35em')
  .attr('font-size', 13).attr('font-weight', 700).attr('fill', '#111827')
  .attr('stroke', '#ffffff').attr('stroke-width', 3).attr('paint-order', 'stroke')
  .text(f => f.properties.name);
svg.append('text').attr('x', 24).attr('y', 44).attr('font-size', 26).attr('font-weight', 700)
  .attr('fill', '#0f172a').text('Where we opened offices in 2026');
px.output(doc);
```

```bash
pixelox render d3 world.js -o world.png
pixelox render d3 world.js -o world-americas.png --param highlight=USA,CAN,MEX,BRA,ARG,CHL
```

`world-50m` has finer coasts and 241 countries and territories (small islands too);
`land-110m` is the land alone, for a quiet background. Any d3 projection works:
`d3.geoNaturalEarth1()`, `d3.geoMercator()`, or `d3.geoOrthographic().rotate([-20, -30])`
for a globe.

## A treemap

<!-- file: budget.json -->
```json
{"name": "Budget", "children": [
  {"name": "Engineering", "children": [
    {"name": "Platform", "value": 420}, {"name": "Mobile", "value": 260}, {"name": "Data", "value": 210}, {"name": "Security", "value": 120}]},
  {"name": "Marketing", "children": [
    {"name": "Brand", "value": 180}, {"name": "Paid ads", "value": 240}, {"name": "Events", "value": 90}]},
  {"name": "Sales", "children": [
    {"name": "Enterprise", "value": 300}, {"name": "SMB", "value": 150}]},
  {"name": "Operations", "children": [
    {"name": "Support", "value": 160}, {"name": "Finance", "value": 80}, {"name": "People", "value": 70}]}
]}
```

<!-- file: treemap.js -->
```js
// A treemap: d3.hierarchy + d3.treemap from nested JSON, colored by top-level group.
const W = 1000, H = 620, top = 64;
const root = d3.hierarchy(px.data.budget).sum(d => d.value || 0).sort((a, b) => b.value - a.value);
d3.treemap().size([W, H - top]).paddingOuter(4).paddingTop(26).paddingInner(3).round(true)(root);
const color = d3.scaleOrdinal(root.children.map(d => d.data.name), ['#2563eb', '#db2777', '#059669', '#d97706']);

const doc = px.svg(W, H);
const svg = d3.select(doc.documentElement).attr('font-family', 'Helvetica, Arial, sans-serif');
svg.append('rect').attr('width', W).attr('height', H).attr('fill', '#ffffff');
svg.append('text').attr('x', 8).attr('y', 40).attr('font-size', 26).attr('font-weight', 700)
  .attr('fill', '#0f172a').text(`Budget, $${d3.format(',')(root.value)}k`);
const g = svg.append('g').attr('transform', `translate(0,${top})`);
const group = g.selectAll('g.group').data(root.children).join('g').attr('class', 'group');
group.append('rect').attr('x', d => d.x0).attr('y', d => d.y0).attr('width', d => d.x1 - d.x0)
  .attr('height', d => d.y1 - d.y0).attr('fill', d => color(d.data.name)).attr('opacity', 0.18);
group.append('text').attr('x', d => d.x0 + 8).attr('y', d => d.y0 + 19).attr('font-size', 15)
  .attr('font-weight', 700).attr('fill', d => color(d.data.name)).text(d => `${d.data.name} · ${d.value}`);
const leaf = g.selectAll('g.leaf').data(root.leaves()).join('g').attr('class', 'leaf')
  .attr('transform', d => `translate(${d.x0},${d.y0})`);
leaf.append('rect').attr('width', d => d.x1 - d.x0).attr('height', d => d.y1 - d.y0)
  .attr('rx', 3).attr('fill', d => color(d.parent.data.name));
leaf.filter(d => d.x1 - d.x0 > 60 && d.y1 - d.y0 > 40).append('text').attr('x', 8).attr('y', 22)
  .attr('font-size', 15).attr('font-weight', 700).attr('fill', '#ffffff').text(d => d.data.name);
leaf.filter(d => d.x1 - d.x0 > 60 && d.y1 - d.y0 > 40).append('text').attr('x', 8).attr('y', 42)
  .attr('font-size', 14).attr('fill', '#ffffff').attr('opacity', 0.85).text(d => `$${d.value}k`);
px.output(doc);
```

```bash
pixelox render d3 treemap.js --data budget=budget.json -o treemap.png
```

The same hierarchy feeds `d3.pack()` (circle packing), `d3.partition()` (sunburst), and
`d3.tree()` (org charts).

## A 3D bar chart with three.js

`render three` has no WebGL. `THREE.SVGRenderer` draws a lit scene as flat-shaded polygons,
painted back to front; draw floors and backgrounds yourself, behind everything, and place
labels by projecting 3D points (`vector.project(camera)`). This script also uses d3 for
colors and number formats, so it asks for both libraries with `--with`.

<!-- file: bars3d.js -->
```js
// A shaded 3D bar chart with three.js: boxes lit by a sun, drawn by THREE.SVGRenderer, with a
// floor and labels placed by projecting 3D points to the screen.
const rows = px.data.rows;                        // columns: month,sales
const W = 1200, H = 800, NS = 'http://www.w3.org/2000/svg';
const scene = new THREE.Scene();
scene.add(new THREE.AmbientLight(0xffffff, 0.55));
const sun = new THREE.DirectionalLight(0xffffff, 0.9);
sun.position.set(-4, 6, 5);
scene.add(sun);

const max = d3.max(rows, d => +d.sales);
const color = i => d3.interpolateBlues(0.45 + 0.5 * i / (rows.length - 1));
const gap = 1.6, x0 = -((rows.length - 1) * gap) / 2, height = d => (+d.sales / max) * 5;
rows.forEach((d, i) => {
  const bar = new THREE.Mesh(new THREE.BoxGeometry(1, height(d), 1),
    new THREE.MeshLambertMaterial({ color: new THREE.Color(color(i)) }));
  bar.position.set(x0 + i * gap, height(d) / 2, 0);
  scene.add(bar);
});

const camera = new THREE.PerspectiveCamera(30, W / H, 0.1, 100);
camera.position.set(7, 8, 14);
camera.lookAt(0, 1.8, 0);
camera.updateMatrixWorld();
const renderer = new THREE.SVGRenderer();
renderer.setSize(W, H);
renderer.setPrecision(2);
renderer.render(scene, camera);
const svg = renderer.domElement;                  // the viewBox is centered on 0,0

// a stroke in each face's own color hides the hairline seams between triangles
for (const p of svg.querySelectorAll('path')) {
  const fill = /fill:([^;]+)/.exec(p.getAttribute('style'));
  if (fill) p.setAttribute('style', p.getAttribute('style') + ';stroke:' + fill[1] + ';stroke-width:0.8;stroke-linejoin:round');
}
const el = (name, attrs, text) => {
  const e = document.createElementNS(NS, name);
  for (const [k, v] of Object.entries(attrs)) e.setAttribute(k, v);
  if (text !== undefined) e.textContent = text;
  return e;
};
const toScreen = (x, y, z) => {
  const v = new THREE.Vector3(x, y, z).project(camera);
  return [v.x * W / 2, -v.y * H / 2];
};
// background and floor go first: the painter's algorithm draws everything else over them
const half = (rows.length * gap) / 2 + 0.3;
const floor = [[-half, 0, -1.2], [half, 0, -1.2], [half, 0, 1.2], [-half, 0, 1.2]].map(p => toScreen(...p));
svg.insertBefore(el('polygon', { points: floor.map(p => p.join(',')).join(' '), fill: '#e2e8f0' }), svg.firstChild);
svg.insertBefore(el('rect', { x: -W / 2, y: -H / 2, width: W, height: H, fill: '#ffffff' }), svg.firstChild);
const font = { 'font-family': 'Helvetica, Arial, sans-serif', 'text-anchor': 'middle' };
rows.forEach((d, i) => {
  const [x, y] = toScreen(x0 + i * gap, height(d), -0.5);   // above the back edge of the top
  svg.appendChild(el('text', { ...font, x, y: y - 12, 'font-size': 22, 'font-weight': 700, fill: '#0f172a' }, d3.format(',')(+d.sales)));
  const [lx, ly] = toScreen(x0 + i * gap, 0, 1.25);
  svg.appendChild(el('text', { ...font, x: lx, y: ly + 22, 'font-size': 20, fill: '#475569' }, d.month));
});
svg.appendChild(el('text', { ...font, x: 0, y: -H / 2 + 60, 'font-size': 34, 'font-weight': 700, fill: '#0f172a' }, 'Monthly sales, first half'));
px.output(svg);
```

```bash
pixelox render three bars3d.js --with three,d3 --data rows=sales.csv -o bars3d.png
```

`THREE.RoundedBoxGeometry(w, h, d, segments, radius)` gives soft-edged bars. For line art
(wireframes, globes drawn as meridians), project points the same way and draw them with
`px.svg()` or `px.canvas()`.

## Good to know

* **Libraries.** `render d3` loads `d3,topojson`; `render three` loads `three,topojson`;
  `render js` loads none until `--with d3` (or `three`, `topojson`) asks. `--lib my-d3.js`
  replaces the bundled d3 (warning PXW1550); `--with d3 --lib d3-sankey.min.js` adds a UMD
  plugin to it instead (`d3.sankey()`).
* **Randomness.** `Math.random`, `px.random()`, and `d3.randomUniform`/`randomNormal`/
  `d3.shuffle` draw from `--seed`. d3's force and pack layouts use d3's own fixed generator,
  so they come out the same every run; pass `simulation.randomSource(px.random)` to vary
  them with the seed.
* **No timers.** Transitions and timers never fire: run a force layout with
  `simulation.stop(); simulation.tick(300);`, then draw.
* **Proof.** The sidecar pins the versions (`with: d3@7.9.0,topojson@3.1.0`) and lists every
  bundled file and atlas with its SHA-256 under `bundled`; `pixelox verify bar.png.pixelox.json --full`
  replays and compares.
* **Look at it.** `pixelox analyze stats choropleth.png --json` catches an empty or
  all-white render.
