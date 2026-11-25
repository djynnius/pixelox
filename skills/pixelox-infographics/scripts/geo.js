// geo.js: a real world map or a globe (d3-geo): countries highlighted, colored by class or by value, with
// markers or value bubbles at longitude and latitude. Needs pixelox 0.6.0 or later (bundled d3 and world atlas)
// and is run with 'render d3'. On older builds use worldmap.js (the dot-matrix world) with the same points=.
// Run:   pixelox render d3 scripts/geo.js -o world.png --size 1080x1350 --param box=5,28,90,36 --param highlight='BRA,IND,NGA' --param points='-46.6:-23.5:City A,77.2:28.6:City B'
// Place: pixelox comp place d.pxo world.png --in-place --as World --at 0,0
// Data params (all optional; countries are named by ISO 3166 three-letter codes: USA, BRA, FRA, NGA, IND, CHN ...):
//   highlight=BRA,IND        these countries take the accent color (or give each its own: highlight='BRA:#0d9488,IND:#f97316')
//   groups=USA:1,BRA:2       country : 1-based class number (class colors from colors=, names from names=)
//   values=USA:12,BRA:30     country : number, colored on ramp= (bins=5 for classes with a stepped legend)
//   points=lon:lat:label:value   markers, comma separated; west and south negative; label and value optional;
//                            with values the marker AREAS are proportional to them
// Form params (all optional):
//   projection=equalearth    equalearth | natural | mercator | globe (an orthographic half sphere)
//   rotate=0,0               globe: the longitude,latitude at its center (rotate=20,10 centers on Africa);
//                            flat maps: the central longitude (rotate=150 puts the Pacific in the middle)
//   extent=                  flat maps: zoom to a box, lon0,lat0,lon1,lat1 (Europe: extent=-12,35,32,71)
//   antarctica=0             1 = draw Antarctica (flat maps)
//   land=#c5ccd3 ocean=      land color; ocean color (default none on flat maps, a pale blue on the globe)
//   stroke=#ffffff strokewidth=0.6   country borders (stroke=none: land as one shape)
//   graticule=0              1 = meridians and parallels every 20 degrees (on by default on the globe)
//   shade=1                  globe: a soft highlight and shadow that make it a sphere
//   accent=#f97316 ramp=#dbeafe,#1d4ed8 bins=0 min= max= names= legend=bottom   as in usmap.js
//   pointcolor=#e11d48 maxr=0.05 pointlabels=1 halo=#ffffff   markers and their labels, as in worldmap.js
// Map data: Natural Earth 1:110m (public domain) through world-atlas, bundled with pixelox; boundaries are
//   Natural Earth's de facto lines.
// Shared params (every chart script): box=x,y,w,h in % of the image (default 0,0,100,100); colors=#hex,#hex;
//   ink=#1f2933 (text) muted=#6b7785 (axis text) grid=#d5dbe1 bg=transparent; fs=label size in px;
//   font='Family, Fallback, sans-serif' (a family name: font classes such as @condensed do not work inside a script)
//   numfont= weight=400 numweight=700; prefix=$ suffix=% decimals=1 compact=1 (1.2K, 3.4M)
// The result's logs hold one JSON line: the map rectangle and an anchor for every marker and every named country.
// ---- shared prelude: the same lines open every chart script of this skill (change all copies together) ----
const W = px.width, H = px.height, P = px.params;
const str = (k, d) => (P[k] === undefined || P[k] === '' ? d : String(P[k]));
const num = (k, d) => { const v = parseFloat(P[k]); return isNaN(v) ? d : v; };
const on = (k, d) => !/^(0|false|no|off|none)$/i.test(str(k, d ? '1' : '0'));
const split = (s) => (s === '' ? [] : s.split(s.indexOf('|') >= 0 ? '|' : ',').map((x) => x.trim()));
const list = (k, d) => split(str(k, d || ''));
const rows = (k, d) => str(k, d || '').split(';').filter((s) => s.trim() !== '').map((s) => s.split(',').map((x) => parseFloat(x) || 0));
const PAL = ['#2563eb', '#f97316', '#0d9488', '#e11d48', '#7c3aed', '#ca8a04', '#0891b2', '#65a30d'];
const COLORS = list('colors'), col = (i) => (COLORS.length ? COLORS[i % COLORS.length] : PAL[i % PAL.length]);
const ink = str('ink', '#1f2933'), muted = str('muted', '#6b7785'), gridc = str('grid', '#d5dbe1'), bg = str('bg', 'transparent');
const font = str('font', "'Helvetica Neue', Helvetica, Arial, 'Liberation Sans', 'DejaVu Sans', sans-serif");
const numfont = str('numfont', font), wt = num('weight', 400), numwt = num('numweight', 700);
const bx = list('box', '0,0,100,100').map(parseFloat);
const B = { x: (W * bx[0]) / 100, y: (H * bx[1]) / 100, w: (W * bx[2]) / 100, h: (H * bx[3]) / 100 };
const fs = num('fs', Math.max(11, Math.round(W * 0.0205))); // label size: the same on every chart of one canvas unless fs= is given
const r2 = (v) => Math.round(v * 100) / 100, r1 = (v) => Math.round(v * 10) / 10;
const doc = px.svg(W, H), svg = doc.documentElement, NS = 'http://www.w3.org/2000/svg';
const el = (parent, name, attrs, text) => { const e = doc.createElementNS(NS, name); for (const k in attrs) if (attrs[k] !== undefined && attrs[k] !== null) e.setAttribute(k, attrs[k]); if (text !== undefined) e.appendChild(doc.createTextNode(String(text))); parent.appendChild(e); return e; };
if (bg !== 'transparent') el(svg, 'rect', { x: 0, y: 0, width: W, height: H, fill: bg });
const defs = el(svg, 'defs', {});
// text: y is the baseline; o = {size, weight, fill, anchor: start|middle|end, rotate, spacing, font, opacity, stretch, halo}
const T = (g, x, y, s, o) => { o = o || {}; const a = { x: r2(x), y: r2(y), 'font-family': o.font || font, 'font-size': r2(o.size || fs), 'font-weight': o.weight || wt, fill: o.fill || ink, 'text-anchor': o.anchor || 'start' }; if (o.spacing) a['letter-spacing'] = o.spacing; if (o.rotate) a.transform = 'rotate(' + o.rotate + ' ' + r2(x) + ' ' + r2(y) + ')'; if (o.opacity !== undefined) a['fill-opacity'] = o.opacity; if (o.stretch) a['font-stretch'] = o.stretch; if (o.halo && o.halo !== 'none') el(g, 'text', Object.assign({}, a, { fill: o.halo, stroke: o.halo, 'stroke-width': r2((o.size || fs) * 0.28), 'stroke-linejoin': 'round' }), s); return el(g, 'text', a, s); };
const mid = (size) => size * 0.35; // add to a center y to get the baseline of vertically centered text
// estimated text width (the sandbox cannot measure type): good to about 8% for grotesque sans faces
const tw = (s, size, bold) => { let w = 0; for (const ch of String(s)) w += /[iljtfI.,:;!'| ]/.test(ch) ? 0.29 : /[mwMW@%]/.test(ch) ? 0.86 : /[A-Z]/.test(ch) ? 0.68 : /[0-9$+\-]/.test(ch) ? 0.56 : 0.53; return w * size * (bold ? 1.07 : 1); };
const wrap = (s, maxw, size, bold) => { const out = []; let line = ''; for (const word of String(s).split(/\s+/)) { const t = line ? line + ' ' + word : word; if (line && tw(t, size, bold) > maxw) { out.push(line); line = word; } else line = t; } if (line) out.push(line); return out; };
// multi-line text block; returns the y after the last line. o.anchor as in T; o.lh = line height factor (1.25)
const TB = (g, x, y, s, maxw, o) => { o = o || {}; const size = o.size || fs, lh = size * (o.lh || 1.25); let yy = y; for (const para of String(s).split('\\n')) for (const line of wrap(para, maxw, size, (o.weight || wt) >= 600)) { T(g, x, yy, line, o); yy += lh; } return yy; };
// color -> [r, g, b]: #hex (3, 4, 6, or 8 digits), common CSS names, rgb()/rgba(), hsl()/hsla(); anything else is mid gray
const NAMED = { black: '000000', white: 'ffffff', red: 'ff0000', green: '008000', blue: '0000ff', yellow: 'ffff00', orange: 'ffa500', purple: '800080', pink: 'ffc0cb', gray: '808080', grey: '808080', silver: 'c0c0c0', navy: '000080', teal: '008080', maroon: '800000', olive: '808000', lime: '00ff00', aqua: '00ffff', cyan: '00ffff', fuchsia: 'ff00ff', magenta: 'ff00ff', gold: 'ffd700', coral: 'ff7f50', crimson: 'dc143c', indigo: '4b0082', violet: 'ee82ee', turquoise: '40e0d0', tomato: 'ff6347', salmon: 'fa8072', khaki: 'f0e68c', beige: 'f5f5dc', ivory: 'fffff0', brown: 'a52a2a', tan: 'd2b48c', chocolate: 'd2691e', goldenrod: 'daa520', orchid: 'da70d6', plum: 'dda0dd', lavender: 'e6e6fa', skyblue: '87ceeb', steelblue: '4682b4', royalblue: '4169e1', slategray: '708090', darkgray: 'a9a9a9', lightgray: 'd3d3d3', whitesmoke: 'f5f5f5', darkblue: '00008b', darkgreen: '006400', darkred: '8b0000', forestgreen: '228b22', seagreen: '2e8b57' };
const rgb = (c) => {
  const s = String(c).trim().toLowerCase(), f = /^(rgba?|hsla?)\(([^)]*)\)$/.exec(s);
  if (f) {
    const parts = f[2].split(/[\s,\/]+/).filter(Boolean), p = parts.map(parseFloat);
    if (f[1][0] === 'r') return p.slice(0, 3).map((v, i) => (/%$/.test(parts[i]) ? v * 2.55 : v));
    const hh = (((p[0] % 360) + 360) % 360) / 360, sat = p[1] / 100, l = p[2] / 100, q = l < 0.5 ? l * (1 + sat) : l + sat - l * sat, pp = 2 * l - q;
    const hue = (t) => { t = (t + 1) % 1; return t < 1 / 6 ? pp + (q - pp) * 6 * t : t < 1 / 2 ? q : t < 2 / 3 ? pp + (q - pp) * (2 / 3 - t) * 6 : pp; };
    return [hue(hh + 1 / 3), hue(hh), hue(hh - 1 / 3)].map((v) => v * 255);
  }
  const m = /^#?([0-9a-f]{3,8})$/.exec(s);
  let h = m ? m[1] : NAMED[s] || '808080'; if (h.length < 6) h = h.split('').slice(0, 3).map((x) => x + x).join('');
  return [0, 2, 4].map((i) => parseInt(h.substr(i, 2), 16));
};
const hex = (a) => '#' + a.map((v) => ('0' + Math.max(0, Math.min(255, Math.round(v))).toString(16)).slice(-2)).join('');
const mix = (a, b, t) => { const x = rgb(a), y = rgb(b); return hex(x.map((v, i) => v + (y[i] - v) * t)); };
const lum = (c) => { const v = rgb(c).map((x) => { x /= 255; return x <= 0.03928 ? x / 12.92 : Math.pow((x + 0.055) / 1.055, 2.4); }); return 0.2126 * v[0] + 0.7152 * v[1] + 0.0722 * v[2]; };
const onc = (c) => (lum(c) > 0.3 ? '#1f2933' : '#ffffff'); // readable text color on a fill: dark on light and mid tones, white on dark
const pre = str('prefix', ''), suf = str('suffix', ''), dec = num('decimals', -1), compact = on('compact', 0);
const fmt = (v, d) => { let a = Math.abs(v), u = ''; if (compact) { if (a >= 1e9) { a /= 1e9; u = 'B'; } else if (a >= 1e6) { a /= 1e6; u = 'M'; } else if (a >= 1e3) { a /= 1e3; u = 'K'; } } const k = d !== undefined ? d : dec >= 0 ? dec : Math.abs(a - Math.round(a)) < 1e-9 ? 0 : a < 100 ? 1 : 0; const p = a.toFixed(k).split('.'); if (!u) p[0] = p[0].replace(/\B(?=(\d{3})+(?!\d))/g, ','); return (v < 0 ? '-' : '') + pre + p.join('.') + u + suf; };
const vmaxOf = (a) => { const m = Math.max.apply(null, a.concat([0])); return m > 0 ? m : 1; }; // the largest value, or 1 when nothing is above zero (fractions keep their full length)
const nice1 = (max, n) => { const raw = max / Math.max(1, n), p = Math.pow(10, Math.floor(Math.log10(raw))), f = raw / p, step = (f <= 1 ? 1 : f <= 2 ? 2 : f <= 2.5 ? 2.5 : f <= 5 ? 5 : 10) * p; return { step, max: Math.ceil(max / step - 1e-9) * step }; };
const nice = (max, n) => { if (!(max > 0)) return { step: 1, max: 1 }; const a = nice1(max, n), b = nice1(max, n + 1); return b.max < a.max ? b : a; }; // a round axis top just above the data
const stepFor = (lo, hi, n) => { const range = hi - lo; if (!(range > 0)) return 1; const p = Math.pow(10, Math.floor(Math.log10(range / Math.max(1, n)))); let best = p, bd = 1e9; for (const m of [1, 2, 2.5, 5, 10, 20, 25, 50]) { const st = m * p, cnt = Math.ceil((hi - Math.floor(lo / st) * st) / st - 1e-9), d = Math.abs(cnt - n) + (cnt > n ? 0.1 : 0); if (d < bd) { bd = d; best = st; } } return best; }; // class width that gives n classes (or the nearest count) between round limits
const pt = (cx, cy, r, deg) => { const a = ((deg - 90) * Math.PI) / 180; return [cx + r * Math.cos(a), cy + r * Math.sin(a)]; }; // 0 deg = 12 o'clock, clockwise
// ring sector between radii r0 < r1 from angle a0 to a1 (degrees, clockwise from 12 o'clock)
const sector = (cx, cy, r0, r1, a0, a1) => { if (a1 - a0 >= 359.999) a1 = a0 + 359.999; const big = a1 - a0 > 180 ? 1 : 0, p = (r, a) => pt(cx, cy, r, a).map(r2).join(' '); return r0 <= 0.01 ? 'M ' + r2(cx) + ' ' + r2(cy) + ' L ' + p(r1, a0) + ' A ' + r2(r1) + ' ' + r2(r1) + ' 0 ' + big + ' 1 ' + p(r1, a1) + ' Z' : 'M ' + p(r1, a0) + ' A ' + r2(r1) + ' ' + r2(r1) + ' 0 ' + big + ' 1 ' + p(r1, a1) + ' L ' + p(r0, a1) + ' A ' + r2(r0) + ' ' + r2(r0) + ' 0 ' + big + ' 0 ' + p(r0, a0) + ' Z'; };
// a row of legend keys centered on cx; wraps past maxw; returns the height used
const legend = (g, items, cx, y, maxw, size) => { size = size || fs; const sw = size * 0.8, gap = size * 1.3, lines = [[]]; let cur = 0; for (const it of items) { const w = sw + size * 0.45 + tw(it.label, size) + gap; if (cur + w > maxw + gap && lines[lines.length - 1].length) { lines.push([]); cur = 0; } lines[lines.length - 1].push([it, w]); cur += w; } let yy = y; for (const ln of lines) { let x = cx - (ln.reduce((s, q) => s + q[1], 0) - gap) / 2; for (const q of ln) { el(g, q[0].line ? 'rect' : 'rect', { x: r2(x), y: r2(yy + (q[0].line ? sw * 0.38 : 0)), width: r2(sw), height: r2(q[0].line ? sw * 0.24 : sw), rx: r2(q[0].line ? sw * 0.12 : sw * 0.2), fill: q[0].color }); T(g, x + sw + size * 0.45, yy + sw * 0.5 + mid(size), q[0].label, { size, fill: muted }); x += q[1]; } yy += size * 1.7; } return yy - y; };
const anchors = [], A = (id, label, x, y, extra) => anchors.push(Object.assign({ id, label, x: Math.round(x), y: Math.round(y), xp: r1((x / W) * 100), yp: r1((y / H) * 100) }, extra || {}));
const done = (name, extra) => { px.log(JSON.stringify(Object.assign({ chart: name, size: [W, H], box: [B.x, B.y, B.w, B.h].map(Math.round), anchors }, extra || {}))); px.output(doc); };
// ---- end of the shared prelude ----
const world = px.atlas('world-110m'), all = topojson.feature(world, world.objects.countries).features, proj0 = str('projection', 'equalearth').toLowerCase(), globe = /^(globe|ortho)/.test(proj0);
const kv = (k) => { const m = {}; for (const it of split(str(k, ''))) { const p = it.split(':'); if (p.length > 1 && !isNaN(parseFloat(p[1]))) m[p[0].trim().toUpperCase()] = parseFloat(p[1]); } return m; };
const groups = kv('groups'), values = kv('values'), names = list('names'), accent = str('accent', '#f97316'), hlc = {}; for (const it of split(str('highlight', ''))) { const p = it.split(':'); hlc[p[0].trim().toUpperCase()] = (p[1] || '').trim() || accent; }
const byVal = Object.keys(values).length > 0, byGrp = Object.keys(groups).length > 0, ramp = list('ramp', '#dbeafe,#1d4ed8'), land = str('land', '#c5ccd3'), ocean = str('ocean', globe ? '#e3edf5' : ''), stroke = str('stroke', '#ffffff'), sw = num('strokewidth', 0.6);
const feats = all.filter((f) => globe || on('antarctica', 0) || f.properties.iso_a3 !== 'ATA'), small = fs * 0.9, g = el(svg, 'g', {});
const vs = Object.keys(values).map((k) => values[k]), lo = P.min !== undefined && P.min !== '' ? num('min', 0) : Math.min.apply(null, vs.length ? vs : [0]), hi = P.max !== undefined && P.max !== '' ? num('max', 1) : Math.max.apply(null, vs.length ? vs : [1]);
const rampAt = (t) => { t = Math.max(0, Math.min(1, t)); if (ramp.length >= 3) return t < 0.5 ? mix(ramp[0], ramp[1], t * 2) : mix(ramp[1], ramp[2], (t - 0.5) * 2); return mix(ramp[0], ramp[1] || ramp[0], t); };
const nbP = Math.round(num('bins', 0)), nstep = nbP > 1 ? stepFor(lo, hi, nbP) : 1, nstart = Math.floor(lo / nstep) * nstep, limits = nbP > 1 ? Array.from({ length: 12 }, (_, i) => r2(nstart + nstep * (i + 1))).filter((v) => v < hi) : [], nb = limits.length ? limits.length + 1 : 0;
const cls = (v) => { let i = 0; while (i < limits.length && v >= limits[i]) i++; return i; }, clsColor = (i) => rampAt(nb > 1 ? i / (nb - 1) : 1);
const fillOf = (c) => (hlc[c] ? hlc[c] : byGrp && groups[c] ? col(Math.round(groups[c]) - 1) : byVal && values[c] !== undefined ? (nb > 1 ? clsColor(cls(values[c])) : rampAt(hi > lo ? (values[c] - lo) / (hi - lo) : 1)) : land);
const lg = str('legend', byVal || names.length ? 'bottom' : 'none').toLowerCase(); let ay = B.y, ah = B.h;
const used = []; for (const k in groups) if (used.indexOf(Math.round(groups[k])) < 0) used.push(Math.round(groups[k])); used.sort((a, b) => a - b);
const rng = (i) => (i === 0 ? 'under ' + fmt(limits[0]) : i === nb - 1 ? fmt(limits[nb - 2]) + ' or more' : fmt(limits[i - 1]) + ' to ' + fmt(limits[i]));
const items = byGrp ? used.map((i) => ({ label: names[i - 1] || 'Group ' + i, color: col(i - 1) })) : nb > 1 ? Array.from({ length: nb }, (_, i) => ({ label: rng(i), color: clsColor(i) })) : [];
const lgH = lg === 'none' ? 0 : items.length ? Math.ceil(items.reduce((a, it) => a + small * 2.6 + tw(it.label, small), 0) / B.w) * small * 1.7 : small * 2.6;
const drawLegend = (y) => { if (items.length) legend(g, items, B.x + B.w / 2, y, B.w, small); else { const w = Math.min(B.w * 0.42, 360 * (fs / 20)), x = B.x + (B.w - w) / 2, lgd = el(defs, 'linearGradient', { id: 'ramp', x1: 0, y1: 0, x2: 1, y2: 0 }); ramp.forEach((c, i) => el(lgd, 'stop', { offset: ramp.length > 1 ? i / (ramp.length - 1) : 0, 'stop-color': c })); el(g, 'rect', { x: r2(x), y: r2(y), width: r2(w), height: r2(small * 0.7), rx: r2(small * 0.2), fill: 'url(#ramp)' }); T(g, x - small * 0.5, y + small * 0.35 + mid(small), fmt(lo), { size: small, fill: muted, anchor: 'end' }); T(g, x + w + small * 0.5, y + small * 0.35 + mid(small), fmt(hi), { size: small, fill: muted }); } };
if (lg === 'top') { drawLegend(B.y); ay += lgH + fs * 0.5; ah -= lgH + fs * 0.5; } else if (lg !== 'none') { drawLegend(B.y + B.h - lgH + small * 0.3); ah -= lgH + fs * 0.6; }
// projection fitted to the box
const rot = (rows('rotate', '0,0')[0] || [0, 0]), ext = rows('extent', '')[0] || [];
const proj = globe ? d3.geoOrthographic().rotate([-rot[0], -(rot[1] || 0)]) : (proj0.charAt(0) === 'm' ? d3.geoMercator() : proj0.charAt(0) === 'n' ? d3.geoNaturalEarth1() : d3.geoEqualEarth()).rotate([-rot[0], 0]);
const fc = { type: 'FeatureCollection', features: feats }, frame = ext.length === 4 ? { type: 'Feature', geometry: { type: 'MultiPoint', coordinates: (() => { const q = []; for (let i = 0; i <= 20; i++) { const t = i / 20, lx = ext[0] + (ext[2] - ext[0]) * t, ly = ext[1] + (ext[3] - ext[1]) * t; q.push([lx, ext[1]], [lx, ext[3]], [ext[0], ly], [ext[2], ly]); } return q; })() } } : globe ? { type: 'Sphere' } : proj0.charAt(0) === 'm' ? fc : { type: 'Sphere' };
proj.fitExtent([[B.x, ay], [B.x + B.w, ay + ah]], !globe && ext.length !== 4 && !on('antarctica', 0) ? fc : frame);
const gp = d3.geoPath(proj), cid = 'clip', cp = el(defs, 'clipPath', { id: cid }); el(cp, 'rect', { x: r2(B.x), y: r2(ay), width: r2(B.w), height: r2(ah) });
const mg = el(g, 'g', ext.length === 4 ? { 'clip-path': 'url(#clip)' } : {});
if (ocean && ocean !== 'none') el(mg, 'path', { d: gp({ type: 'Sphere' }), fill: ocean });
if (on('graticule', globe ? 1 : 0)) el(mg, 'path', { d: gp(d3.geoGraticule().step([20, 20])()), fill: 'none', stroke: str('gridcolor', globe ? '#ffffff' : gridc), 'stroke-width': 0.8, 'stroke-opacity': 0.9 });
const cent = {};
if (stroke === 'none') for (const f of feats) { const d = gp(f), fc0 = fillOf(f.properties.iso_a3); if (d) el(mg, 'path', { d, fill: fc0, stroke: fc0, 'stroke-width': 0.5, 'stroke-linejoin': 'round' }); }
else for (const f of feats) { const d = gp(f); if (d) el(mg, 'path', { d, fill: fillOf(f.properties.iso_a3), stroke, 'stroke-width': sw, 'stroke-linejoin': 'round' }); }
for (const f of feats) { const c = f.properties.iso_a3; if (hlc[c] || groups[c] || values[c] !== undefined) { const p = gp.centroid(f); if (p && !isNaN(p[0])) cent[c] = [p, f.properties.name]; } }
if (globe && on('shade', 1)) { const b = gp.bounds({ type: 'Sphere' }), cxg = (b[0][0] + b[1][0]) / 2, cyg = (b[0][1] + b[1][1]) / 2, rg = (b[1][0] - b[0][0]) / 2, gr = el(defs, 'radialGradient', { id: 'sph', cx: 0.36, cy: 0.3, r: 0.75 }); el(gr, 'stop', { offset: 0, 'stop-color': '#ffffff', 'stop-opacity': 0.38 }); el(gr, 'stop', { offset: 0.5, 'stop-color': '#ffffff', 'stop-opacity': 0 }); el(gr, 'stop', { offset: 1, 'stop-color': '#0b1b33', 'stop-opacity': 0.34 }); el(mg, 'circle', { cx: r2(cxg), cy: r2(cyg), r: r2(rg), fill: 'url(#sph)' }); A('center', 'globe center', cxg, cyg, { r: Math.round(rg) }); }
// markers and bubbles
const pts = split(str('points', '')).map((s) => { const p = s.split(':'); return { lon: parseFloat(p[0]), lat: parseFloat(p[1]), label: (p[2] || '').trim(), v: p[3] === undefined ? null : parseFloat(p[3]) }; }).filter((p) => !isNaN(p.lon) && !isNaN(p.lat));
const pc = str('pointcolor', '#e11d48'), halo = str('halo', '#ffffff'), vmax = Math.max.apply(null, pts.map((p) => p.v || 0).concat([0])), mb = gp.bounds({ type: 'Sphere' }), mapW = Math.min(B.w, mb[1][0] - mb[0][0]), maxr = num('maxr', 0.05) * mapW, showL = on('pointlabels', 1), dotr = Math.max(4, fs * 0.3);
const visible = (p) => !globe || d3.geoDistance([p.lon, p.lat], [rot[0], rot[1] || 0]) < Math.PI / 2 - 0.02;
pts.sort((a, b) => (b.v || 0) - (a.v || 0)).forEach((p, i) => { if (!visible(p)) return; const q = proj([p.lon, p.lat]); if (!q) return; const x = q[0], y = q[1], r = vmax > 0 && p.v !== null ? Math.max(dotr, maxr * Math.sqrt(Math.max(0, p.v) / vmax)) : dotr;
  if (vmax > 0) el(g, 'circle', { cx: r2(x), cy: r2(y), r: r2(r), fill: pc, 'fill-opacity': 0.75, stroke: '#ffffff', 'stroke-width': 1.5 }); else { el(g, 'circle', { cx: r2(x), cy: r2(y), r: r2(r * 2), fill: pc, 'fill-opacity': 0.22 }); el(g, 'circle', { cx: r2(x), cy: r2(y), r: r2(r), fill: pc, stroke: '#ffffff', 'stroke-width': 1.5 }); }
  if (showL && p.label) { const right = x + r + small * 0.4 + tw(p.label, small, 1) < B.x + B.w, tx = x + (right ? 1 : -1) * (r + small * 0.4), an = right ? 'start' : 'end', two = p.v !== null && vmax > 0; T(g, tx, y + mid(small) - (two ? small * 0.55 : 0), p.label, { size: small, weight: 700, anchor: an, halo }); if (two) T(g, tx, y + mid(small) + small * 0.6, fmt(p.v), { size: small, anchor: an, fill: muted, font: numfont, halo }); }
  A('point' + (i + 1), p.label, x, y, p.v === null ? {} : { value: p.v, r: Math.round(r) }); });
for (const c in cent) A(c, cent[c][1], cent[c][0][0], cent[c][0][1], values[c] !== undefined ? { value: values[c] } : {});
done('geo', { map: [mb[0][0], mb[0][1], mb[1][0] - mb[0][0], mb[1][1] - mb[0][1]].map(Math.round), projection: globe ? 'globe' : proj0 });
