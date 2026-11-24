# ADR 0018: Vendored JavaScript libraries and map data

Status: accepted (2026-11-24)

## Context
`render d3` and `render three` ran real d3 and three.js in the QuickJS sandbox, but the build
shipped neither library: without `--lib` they stopped with PX1520, so in practice nobody used
them. Agents building infographics (the `pixelox-infographics` skill) need scales, axes, arcs,
pie and stack layouts, hierarchy layouts, and geographic projections, and, for maps, the map
outlines themselves: d3-geo draws a U.S. or world map only from geometry the script supplies.
The PRD (§7.10) always meant both libraries to be bundled.

## Decision
* **What ships** (`crates/pixelox-render/vendor/`, written by `scripts/vendor-js.py`):

  | Package | Version | License | Files | Bytes | Embedded |
  | --- | --- | --- | --- | --- | --- |
  | d3 | 7.9.0 | ISC | `d3.min.js` (UMD) | 279,706 | 92,736 |
  | three | 0.187.0 (r187) | MIT | `three.core.js`, addons `Projector.js`, `SVGRenderer.js`, `RoundedBoxGeometry.js` | 1,525,903 | 302,821 |
  | topojson-client | 3.1.0 | ISC | `topojson-client.min.js` | 7,169 | 2,591 |
  | world-atlas | 2.0.2 | ISC (Natural Earth 4.1.0, public domain) | `countries-110m.json`, `countries-50m.json`, `land-110m.json` | 919,388 | 291,132 |
  | us-atlas | 3.0.1 | ISC (Census 2017 cartographic boundaries, public domain) | `states-10m.json`, `states-albers-10m.json`, `nation-10m.json` | 276,823 | 91,661 |
  | i18n-iso-countries | 7.14.0 | MIT | `codes.json` (ISO 3166-1 codes only) | 10,253 | 2,708 |
  | pixelox | - | public domain | `us-states.tsv` (FIPS, USPS code, name) | 1,030 | 696 |

  About 3.0 MB of files, 0.78 MB in the binary. The script downloads each tarball from the
  npm registry, checks it against the sha512 integrity pinned in the script, reads only the
  listed files (in memory, so a tarball cannot write outside `vendor/`), and writes them byte
  for byte with each package's license (`LICENSE-<package>.txt`, plus notes on the data
  sources) and `manifest.tsv` (path, package, version, license, bytes, SHA-256) and a
  README table. `.gitattributes` keeps the files byte-exact (`-text`).
* **three.js r187** is the newest release and loads: since r171 `three.cjs` is a stub that
  `require()`s the ES module, and `three.module.js` imports `./three.core.js`, so the bundle is
  `three.core.js`, a self-contained ES module (the WebGL renderer, which cannot run here, is
  not in it). The addons are ES modules that import `'three'` and `'./Projector.js'`; the
  engine declares the modules in dependency order under those names, so QuickJS resolves the
  imports to the already loaded modules without a module loader, and user scripts still
  cannot import anything else (`modules_cannot_be_imported`). `THREE` is one plain object
  holding the core and addon exports, the way the old `examples/js` builds attached
  `THREE.SVGRenderer`. SVGRenderer works on the DOM-lite; it paints flat-shaded polygons back
  to front, which suits bar charts and simple solids; the cookbook shows the floor-first and
  seam-stroke habits it needs.
* **Embedding.** `build.rs` deflates every manifest file into `OUT_DIR` (miniz_oxide, already
  in the tree via `png`) and lists them in a generated `vendored.rs`. A run inflates only the
  files it loads, checks each against the manifest's SHA-256 (PX1553 if the binary is
  damaged), and keeps it for the rest of the process. Loading d3 or three.js (inflate, check,
  parse) adds about 0.03 s to a run.
* **How scripts reach them.** `--with d3,three,topojson` (each optionally `name@version`,
  or `none`) loads bundled libraries before the `--lib` files and the script. Defaults:
  `render d3` → `d3,topojson`; `render three` → `three,topojson`; `render js` → none, so its
  global object stays the strict allowlist. A `--lib` file given without `--with` replaces
  the bundled d3/three (PXW1550 says so); with `--with d3` it is loaded after the bundled d3 and
  its exports join `d3`, and the CommonJS shim's `require` serves the bundled libraries
  (`require('d3-array')` is `d3`), so UMD plugins such as d3-sankey work. `--lib` keeps its
  meaning; no flag or field was removed or renamed. PX1520 stays for `render d3 --with none`
  without a `--lib`, and for a `--lib` bundle that defines no `d3`/`THREE`.
* **Maps: `px.atlas(name)`**, a method rather than a `--data name=atlas:...` form, so a
  script needs no flag and the atlases work in every command (`render js` too). It returns a
  fresh TopoJSON object: `world-110m`, `world-50m`, `land-110m`, `us-states`,
  `us-states-albers`, `us-nation`. `topojson.feature(topo, topo.objects.states)` makes GeoJSON
  for `d3.geoPath`. Features gain join codes: countries `iso_a2`/`iso_a3` from the ISO table
  by their numeric id (Kosovo gets the user-assigned XK/XKX; N. Cyprus, Somaliland, Siachen
  Glacier, and Indian Ocean Ter. have none), states `postal` and `fips`. `px.atlases()` lists
  them; `pixelox docs atlas` is generated from the same data (every state and country with its
  codes) and a contract test keeps it current. County files (0.84 MB each, 0.25 MB embedded)
  are left out; any TopoJSON still loads with `--data us=counties-10m.json`.
* **Fonts for `<text>`.** SVG results rasterize with the font database `text add` uses
  (system fonts unless `PIXELOX_SYSTEM_FONTS=0`, `fonts add` folders, `fonts get` downloads,
  the embedded Tuffy), with the CSS generic families mapped to installed faces. Where Arial,
  Times New Roman, and Courier New exist nothing changes; in a container without system
  fonts, d3's `sans-serif` axis labels now draw (in Tuffy) instead of vanishing. The faces used
  are recorded like `text add`'s, so replay names a missing one (PXW4011).
* **Determinism.** The prelude replaces `Math.random` before any library loads, so d3-random
  (`randomUniform`, `randomNormal`, ...) and `d3.shuffle`, which capture `Math.random` at load,
  draw from `--seed`; three.js's UUIDs too. d3-force and `d3.pack` use d3's own fixed LCG: the
  same layout every run, as in a browser (`simulation.randomSource(px.random)` ties it to the
  seed). d3-timer schedules callbacks through inert `setTimeout`/`setInterval` defined only when
  d3 is loaded; they never run (there is no event loop), and PX6020's hint says to run
  layouts synchronously when a script ends waiting for one. Dates stay fixed and UTC.
* **Provenance.** The recipe pins what ran (`with: d3@7.9.0,topojson@3.1.0`): a replay on a
  build that bundles another version stops with PX1552 instead of drawing something else.
  The sidecar's new `bundled` list records every library file and atlas used (kind, name,
  package, version, file, SHA-256, bytes, license), plus the code table that shaped an atlas;
  `pixelox verify` reports whether this build bundles the same files (PXW1551 when not) and
  `--full` replays. Without `--size`, the recipe now records `size: null`, so a replay
  rasterizes an SVG result at its own size again rather than at the 800x600 default.

## Consequences
* Charts and maps need no downloads and no `--lib`; weaker models write d3 the way every
  tutorial does. `render d3` rose from unusable to about 0.1 s for a chart and 0.2 to 0.6 s for
  a map (atlas parse plus projection).
* The binary grows by about 0.8 MB; the repository by 3.0 MB of vendored files.
* **Updating.** Change a version and its npm integrity in `scripts/vendor-js.py`, run it,
  run `cargo test -p pixelox-render` and `UPDATE_SNAPSHOTS=1 cargo test -p pixelox --test contract`
  (the atlas page), render the cookbook (`docs/guides/cookbook/charts-and-maps-with-d3.md`)
  and look at it, then note the change in the CHANGELOG. Old sidecars pinned to the previous
  version then stop with PX1552, which is the point: a replay never silently swaps libraries.
* Natural Earth's boundaries are de facto lines; the license notes say pixelox takes no
  position on them.
