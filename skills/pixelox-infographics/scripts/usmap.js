// usmap.js: a real map of the US states (Albers USA, with Alaska and Hawaii as insets) colored by class or by
// value (a choropleth), with state labels, value bubbles, and a legend. Needs pixelox 0.6.0 or later (bundled
// d3 and US atlas) and is run with 'render d3'. On older builds use tilemap.js with the same groups=/values=.
// Run:   pixelox render d3 scripts/usmap.js -o map.png --size 1080x1350 --param box=5,28,90,46 --param values='CA:31,TX:44,FL:12,NY:27' --param bins=5
// Place: pixelox comp place d.pxo map.png --in-place --as Map --at 0,0
// Data params (all optional; use groups or values for the fill):
//   groups=CA:1,TX:2,OH:3    state code : 1-based class number (class colors from colors=, names from names=)
//   values=CA:31,TX:44       state code : number, colored on ramp= from the smallest to the largest value
//   names=Group A,Group B    class names for the legend (groups form)
//   highlight=CA,TX,NY       only these states take the accent color; all others take the 'empty' color
//   bubbles=CA:31,TX:44      a circle on each listed state, its AREA proportional to the number
// Form params (all optional):
//   bins=0                   values form: 0 = a smooth ramp; 3 to 7 = that many classes between round limits, with a
//                            stepped legend (the count can differ by one when the data's range is awkward)
//   breaks=10,20,30          values form: your own class limits instead of bins (4 classes here)
//   ramp=#dbeafe,#1d4ed8     low color, high color (three colors make a diverging ramp); min= max= fix its ends
//   empty=#e2e6ea            fill of states without data; accent=#f97316 for highlight=
//   stroke=#ffffff strokewidth=1.2   state borders (set stroke to your ground color)
//   outline=                 a color draws the country's outer outline as well (coast and land borders)
//   labels=codes             codes | values | both | none: what is written on each state
//                            (labels turn dark on light and mid-tone fills and white on dark ones; keep the two
//                            middle classes of a ramp clearly lighter or darker than mid-gray so both stay readable)
//   smalls=1                 list the small north-eastern states in a column beside the map (0 = leave them unlabeled)
//   bubblecolor=#ffffff bubbleline= maxr=0.07 bubblelabels=1   bubble fill, outline color (default: ink), largest radius
//                            as a fraction of the map width, and whether the number is written in each bubble
//   legend=bottom            bottom | top | none
// Map data: U.S. Census Bureau cartographic boundaries (public domain) through us-atlas, bundled with pixelox.
// Shared params (every chart script): box=x,y,w,h in % of the image (default 0,0,100,100); colors=#hex,#hex;
//   ink=#1f2933 (text) muted=#6b7785 (axis text) grid=#d5dbe1 bg=transparent; fs=label size in px;
//   font='Family, Fallback, sans-serif' (a family name: font classes such as @condensed do not work inside a script)
//   numfont= weight=400 numweight=700; prefix=$ suffix=% decimals=1 compact=1 (1.2K, 3.4M)
// The result's logs hold one JSON line: the map rectangle and an anchor (canvas px and %) at every state's center.
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
const rgb = (c) => { let m = /^#?([0-9a-f]{3,8})$/i.exec(String(c).trim()), h = m ? m[1] : '808080'; if (h.length < 6) h = h.split('').slice(0, 3).map((x) => x + x).join(''); return [0, 2, 4].map((i) => parseInt(h.substr(i, 2), 16)); };
const hex = (a) => '#' + a.map((v) => ('0' + Math.max(0, Math.min(255, Math.round(v))).toString(16)).slice(-2)).join('');
const mix = (a, b, t) => { const x = rgb(a), y = rgb(b); return hex(x.map((v, i) => v + (y[i] - v) * t)); };
const lum = (c) => { const v = rgb(c).map((x) => { x /= 255; return x <= 0.03928 ? x / 12.92 : Math.pow((x + 0.055) / 1.055, 2.4); }); return 0.2126 * v[0] + 0.7152 * v[1] + 0.0722 * v[2]; };
const onc = (c) => (lum(c) > 0.3 ? '#1f2933' : '#ffffff'); // readable text color on a fill: dark on light and mid tones, white on dark
const pre = str('prefix', ''), suf = str('suffix', ''), dec = num('decimals', -1), compact = on('compact', 0);
const fmt = (v, d) => { let a = Math.abs(v), u = ''; if (compact) { if (a >= 1e9) { a /= 1e9; u = 'B'; } else if (a >= 1e6) { a /= 1e6; u = 'M'; } else if (a >= 1e3) { a /= 1e3; u = 'K'; } } const k = d !== undefined ? d : dec >= 0 ? dec : Math.abs(a - Math.round(a)) < 1e-9 ? 0 : a < 100 ? 1 : 0; const p = a.toFixed(k).split('.'); if (!u) p[0] = p[0].replace(/\B(?=(\d{3})+(?!\d))/g, ','); return (v < 0 ? '-' : '') + pre + p.join('.') + u + suf; };
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
const us = px.atlas('us-states-albers'), feats = topojson.feature(us, us.objects.states).features, gp = d3.geoPath();
const kv = (k) => { const m = {}; for (const it of split(str(k, ''))) { const p = it.split(':'); if (p.length > 1 && !isNaN(parseFloat(p[1]))) m[p[0].trim().toUpperCase()] = parseFloat(p[1]); } return m; };
const groups = kv('groups'), values = kv('values'), bub = kv('bubbles'), names = list('names'), hl = list('highlight').map((s) => s.toUpperCase());
const byVal = Object.keys(values).length > 0, byGrp = Object.keys(groups).length > 0, ramp = list('ramp', '#dbeafe,#1d4ed8'), empty = str('empty', '#e2e6ea'), accent = str('accent', '#f97316');
const stroke = str('stroke', '#ffffff'), sw = num('strokewidth', 1.2), lab = str('labels', 'codes').toLowerCase(), smalls = on('smalls', 1) && lab !== 'none', small = fs * 0.9, g = el(svg, 'g', {});
const vs = Object.keys(values).map((k) => values[k]), lo = P.min !== undefined && P.min !== '' ? num('min', 0) : Math.min.apply(null, vs.length ? vs : [0]), hi = P.max !== undefined && P.max !== '' ? num('max', 1) : Math.max.apply(null, vs.length ? vs : [1]);
const rampAt = (t) => { t = Math.max(0, Math.min(1, t)); if (ramp.length >= 3) return t < 0.5 ? mix(ramp[0], ramp[1], t * 2) : mix(ramp[1], ramp[2], (t - 0.5) * 2); return mix(ramp[0], ramp[1] || ramp[0], t); };
const brk = (rows('breaks', '')[0] || []).slice().sort((a, b) => a - b), nb0 = 0;
const nbP = Math.round(num('bins', 0)), nstep = nbP > 1 ? stepFor(lo, hi, nbP) : 1, nstart = Math.floor(lo / nstep) * nstep;
const limits = brk.length ? brk : nbP > 1 ? Array.from({ length: 12 }, (_, i) => r2(nstart + nstep * (i + 1))).filter((v) => v < hi) : [];
const nb = limits.length ? limits.length + 1 : 0, cls = (v) => { let i = 0; while (i < limits.length && v >= limits[i]) i++; return i; }, clsColor = (i) => rampAt(nb > 1 ? i / (nb - 1) : 1);
const fillOf = (code) => { if (hl.length) return hl.indexOf(code) >= 0 ? accent : empty; if (byVal) return values[code] === undefined ? (byGrp && groups[code] ? col(Math.round(groups[code]) - 1) : empty) : nb > 1 ? clsColor(cls(values[code])) : rampAt(hi > lo ? (values[code] - lo) / (hi - lo) : 1); return groups[code] ? col(Math.round(groups[code]) - 1) : empty; };
const colorByGroup = byGrp && !hl.length, fillFor = (code) => (colorByGroup && groups[code] ? col(Math.round(groups[code]) - 1) : fillOf(code));
// legend
const lg = str('legend', (byVal && !colorByGroup) || names.length ? 'bottom' : 'none').toLowerCase(); let ay = B.y, ah = B.h;
const used = []; for (const k in groups) if (used.indexOf(Math.round(groups[k])) < 0) used.push(Math.round(groups[k])); used.sort((a, b) => a - b);
const rng = (i) => (i === 0 ? 'under ' + fmt(limits[0]) : i === nb - 1 ? fmt(limits[nb - 2]) + ' or more' : fmt(limits[i - 1]) + ' to ' + fmt(limits[i]));
const items = colorByGroup ? used.map((i) => ({ label: names[i - 1] || 'Group ' + i, color: col(i - 1) })) : nb > 1 ? Array.from({ length: nb }, (_, i) => ({ label: rng(i), color: clsColor(i) })) : [];
const lgH = lg === 'none' ? 0 : items.length ? Math.ceil(items.reduce((a, it) => a + small * 2.6 + tw(it.label, small), 0) / B.w) * small * 1.7 : small * 2.6;
const drawLegend = (y) => { if (items.length) legend(g, items, B.x + B.w / 2, y, B.w, small); else { const w = Math.min(B.w * 0.42, 360 * (fs / 20)), x = B.x + (B.w - w) / 2, lgd = el(defs, 'linearGradient', { id: 'ramp', x1: 0, y1: 0, x2: 1, y2: 0 }); ramp.forEach((c, i) => el(lgd, 'stop', { offset: ramp.length > 1 ? i / (ramp.length - 1) : 0, 'stop-color': c })); el(g, 'rect', { x: r2(x), y: r2(y), width: r2(w), height: r2(small * 0.7), rx: r2(small * 0.2), fill: 'url(#ramp)' }); T(g, x - small * 0.5, y + small * 0.35 + mid(small), fmt(lo), { size: small, fill: muted, anchor: 'end' }); T(g, x + w + small * 0.5, y + small * 0.35 + mid(small), fmt(hi), { size: small, fill: muted }); } };
if (lg === 'top') { drawLegend(B.y); ay += lgH + fs * 0.5; ah -= lgH + fs * 0.5; } else if (lg !== 'none') { drawLegend(B.y + B.h - lgH + small * 0.3); ah -= lgH + fs * 0.6; }
// the atlas is already projected into a 975 x 610 frame
const SMALL = ['VT', 'NH', 'MA', 'RI', 'CT', 'NJ', 'DE', 'MD', 'DC'], colW = smalls && Object.keys(bub).length === 0 ? fs * (lab === 'both' || lab === 'values' ? 5.2 : 3.4) : 0;
const s = Math.min((B.w - colW) / 975, ah / 610), gx = B.x + (B.w - colW - 975 * s) / 2, gy = ay + (ah - 610 * s) / 2, X = (p) => [gx + p[0] * s, gy + p[1] * s];
const mg = el(g, 'g', { transform: 'translate(' + r2(gx) + ' ' + r2(gy) + ') scale(' + r2(s) + ')' });
const info = {};
for (const f of feats) { const code = f.properties.postal, fill = fillFor(code); el(mg, 'path', { d: gp(f), fill }); const c = X(gp.centroid(f)), b = gp.bounds(f); info[code] = { c, w: (b[1][0] - b[0][0]) * s, h: (b[1][1] - b[0][1]) * s, fill, name: f.properties.name }; }
if (stroke !== 'none') el(mg, 'path', { d: gp(topojson.mesh(us, us.objects.states, (a, b) => a !== b)), fill: 'none', stroke, 'stroke-width': r2(sw / s), 'stroke-linejoin': 'round' });
if (P.outline) el(mg, 'path', { d: gp(topojson.mesh(us, us.objects.states, (a, b) => a === b)), fill: 'none', stroke: str('outline', ink), 'stroke-width': r2((sw * 1.2) / s), 'stroke-linejoin': 'round' });
const NUDGE = { FL: [0.1, 0.02], LA: [-0.25, 0], MI: [0.25, 0.3], HI: [0.3, 0.1], CA: [-0.1, 0], ID: [0, 0.25], KY: [0.1, 0.05], WV: [-0.08, 0.05], VA: [0.1, 0.1], MN: [-0.12, 0], NY: [0.1, 0.05] };
const hasB = Object.keys(bub).length > 0, text = (code) => (lab === 'values' ? (values[code] !== undefined ? fmt(values[code]) : '') : code), ls = Math.max(11, fs * 0.72);
if (lab !== 'none' && !hasB) for (const code in info) { const o = info[code]; if (SMALL.indexOf(code) >= 0) continue; const n = NUDGE[code] || [0, 0], x = o.c[0] + n[0] * o.w, y = o.c[1] + n[1] * o.h, both = lab === 'both' && values[code] !== undefined, t = text(code); if (!t || (code !== 'HI' && (o.w < tw(t, ls, 1) * 1.05 || o.h < ls * (both ? 2.3 : 1.2)))) continue;
  const tc = code === 'HI' ? ink : onc(o.fill); T(g, x, y + mid(ls) - (both ? ls * 0.55 : 0), t, { size: ls, weight: 700, anchor: 'middle', fill: tc }); if (both) T(g, x, y + mid(ls) + ls * 0.6, fmt(values[code]), { size: ls * 0.92, anchor: 'middle', fill: tc, font: numfont }); }
if (smalls && !hasB) { const L = SMALL.filter((c) => info[c]).sort((a, b) => info[a].c[1] - info[b].c[1]), x = gx + 975 * s + fs * 0.6, lh = ls * 1.75, y0 = gy + 610 * s * 0.2;
  L.forEach((code, i) => { const o = info[code], y = y0 + i * lh; el(g, 'line', { x1: r2(o.c[0]), y1: r2(o.c[1]), x2: r2(x - ls * 0.3), y2: r2(y), stroke: muted, 'stroke-width': 0.8, 'stroke-opacity': 0.6 }); el(g, 'rect', { x: r2(x), y: r2(y - ls * 0.5), width: r2(ls), height: r2(ls), rx: r2(ls * 0.18), fill: o.fill, stroke: stroke === 'none' ? undefined : muted, 'stroke-width': 0.6 });
    T(g, x + ls * 1.4, y + mid(ls), code + ((lab === 'both' || lab === 'values') && values[code] !== undefined ? '  ' + fmt(values[code]) : ''), { size: ls, weight: 700, fill: ink }); }); }
// bubbles: area proportional to the value; pushed apart where they would overlap
if (hasB) { const bmax = Math.max.apply(null, Object.keys(bub).map((k) => bub[k])), maxr = num('maxr', 0.07) * 975 * s, bc = str('bubblecolor', '#ffffff'), bl = str('bubbleline', ink), showN = on('bubblelabels', 1);
  const N = Object.keys(bub).filter((k) => info[k]).map((k) => ({ k, v: bub[k], r: Math.max(fs * 0.55, maxr * Math.sqrt(Math.max(0, bub[k]) / bmax)), x: info[k].c[0], y: info[k].c[1], ox: info[k].c[0], oy: info[k].c[1] }));
  for (let it = 0; it < 120; it++) for (let i = 0; i < N.length; i++) for (let j = i + 1; j < N.length; j++) { const a = N[i], b = N[j], dx = b.x - a.x, dy = b.y - a.y, d = Math.hypot(dx, dy) || 0.01, need = a.r + b.r + 2; if (d < need) { const push = (need - d) / 2, ux = dx / d, uy = dy / d, wa = b.r / (a.r + b.r), wb = a.r / (a.r + b.r); a.x -= ux * push * wa * 1.0; a.y -= uy * push * wa; b.x += ux * push * wb; b.y += uy * push * wb; } }
  for (const q of N) { q.x = Math.max(gx + q.r, Math.min(gx + 975 * s - q.r, q.x)); q.y = Math.max(gy + q.r, Math.min(gy + 610 * s - q.r, q.y)); }
  N.sort((a, b) => b.r - a.r).forEach((n) => { if (Math.hypot(n.x - n.ox, n.y - n.oy) > n.r) el(g, 'line', { x1: r2(n.ox), y1: r2(n.oy), x2: r2(n.x), y2: r2(n.y), stroke: bl, 'stroke-width': 1 }); el(g, 'circle', { cx: r2(n.x), cy: r2(n.y), r: r2(n.r), fill: bc, stroke: bl, 'stroke-width': r2(Math.max(1.2, fs * 0.07)) });
    if (showN) { const t = fmt(n.v), sz = Math.min(n.r * 0.95, (n.r * 1.5) / Math.max(0.6, tw(t, 1, 1))); T(g, n.x, n.y + mid(sz), t, { size: sz, weight: numwt, font: numfont, anchor: 'middle', fill: onc(bc) }); }
    if (lab !== 'none') T(g, n.x, n.y + n.r + ls * 1.05, n.k, { size: ls, weight: 700, anchor: 'middle', fill: muted, halo: str('halo', '') || undefined }); info[n.k].b = [n.x, n.y, n.r]; }); }
for (const code in info) A(code, info[code].name, info[code].c[0], info[code].c[1], Object.assign(values[code] !== undefined ? { value: values[code] } : {}, info[code].b ? { bx: Math.round(info[code].b[0]), by: Math.round(info[code].b[1]), br: Math.round(info[code].b[2]) } : {}));
done('usmap', { map: [gx, gy, 975 * s, 610 * s].map(Math.round), classes: nb > 1 ? limits.map((v) => r2(v)) : [] });
