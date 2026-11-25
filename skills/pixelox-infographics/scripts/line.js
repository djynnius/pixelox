// line.js: line and area charts from data, with axis, gridlines, markers, and labels: one or several series,
// filled areas, stacked areas, or a bare sparkline. Output is a transparent image.
// Run:   pixelox render js scripts/line.js -o line.png --size 1080x1350 --param box=8,30,84,34 --param values='12,18,17,26,31,44' --param labels='2020,2021,2022,2023,2024,2025'
// Place: pixelox comp place d.pxo line.png --in-place --as Line --at 0,0
// Data params:
//   values=12,18,17,26       one series; several series are separated by ';'
//   labels=2020,2021,2022    x-axis names (one per point; long sets are thinned automatically)
//   series=Online,In store   series names: drawn at the line ends (endlabels=1) or in a legend
// Form params (all optional):
//   area=0                   0 = lines; 1 = each line filled down to the axis (soft); stack = stacked areas
//   smooth=0                 1 = rounded curve through the points (keep 0 when exact values matter)
//   markers=1                dots on the points (turned off above 24 points)
//   vlabels=last             last | all | none: which points get their value written
//   endlabels=1              series names at the right ends of the lines (0 = use the legend)
//   min=0 max=0              value axis range (max=0: a round number above the data); a min other than 0 must be stated on the chart
//   ticks=4 axis=1 gridlines=1 legend=top
//   width=4                  line width in px (scales with the box)
//   spark=0                  1 = sparkline: no axis, no grid, no labels except the last value
// Shared params (every chart script): box=x,y,w,h in % of the image (default 0,0,100,100); colors=#hex,#hex;
//   ink=#1f2933 (text) muted=#6b7785 (axis text) grid=#d5dbe1 bg=transparent; fs=label size in px;
//   font='Family, Fallback, sans-serif' (a family name: font classes such as @condensed do not work inside a script)
//   numfont= weight=400 numweight=700; prefix=$ suffix=% decimals=1 compact=1 (1.2K, 3.4M)
// The result's logs hold one JSON line: the plot rectangle and an anchor (canvas px and %) for every point.
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
const S = rows('values', '12,18,17,26,31,44'), labels = list('labels'), names = list('series'), ns = S.length, n = Math.max.apply(null, S.map((s) => s.length));
const spark = on('spark', 0), areaP = str('area', '0').toLowerCase(), stack = areaP === 'stack' && ns > 1, area = stack || on('area', 0), smooth = on('smooth', 0);
const cum = S.map((s) => s.slice()); if (stack) for (let s = 1; s < ns; s++) for (let i = 0; i < n; i++) cum[s][i] = (cum[s - 1][i] || 0) + (S[s][i] || 0);
const dmax = Math.max.apply(null, cum.map((s) => Math.max.apply(null, s))), dmin = Math.min.apply(null, S.map((s) => Math.min.apply(null, s)));
const nt = Math.round(num('ticks', 4)), lo = spark ? dmin - (dmax - dmin) * 0.1 : num('min', 0), sc0 = num('max', 0) > lo ? { max: num('max', 0), step: (num('max', 0) - lo) / nt } : nice(dmax - lo, nt), hiV = spark ? dmax + (dmax - dmin) * 0.1 : num('max', 0) > lo ? sc0.max : lo + sc0.max;
const u = Math.max(0.6, Math.min(B.w, B.h) / 500), small = fs * 0.9, showAxis = !spark && on('axis', 1), showGrid = !spark && on('gridlines', 1), lw = num('width', 4) * u;
const vl = str('vlabels', 'last').toLowerCase(), endl = !spark && names.length > 0 && on('endlabels', 1) && !stack, markers = !spark && on('markers', 1) && n <= 24 && !stack;
const g = el(svg, 'g', {});
let top = B.y, bottom = B.y + B.h, left = B.x, right = B.x + B.w;
const lg = names.length && !endl && !spark ? str('legend', 'top').toLowerCase() : 'none';
if (lg === 'top') top += legend(g, names.map((nm, s) => ({ label: nm, color: col(s) })), B.x + B.w / 2, top, B.w, small) + fs * 0.4;
const ticks = []; for (let v = lo; v <= hiV + 1e-9; v += sc0.step) ticks.push(v);
if (showAxis) left += Math.max.apply(null, ticks.map((v) => tw(fmt(v), small))) + fs * 0.6;
if (endl) right -= Math.max.apply(null, names.map((s) => tw(s, small, 1))) + fs * 0.8; else if (vl !== 'none') right -= tw(fmt(dmax), fs, 1) * (spark ? 1 : 0.55) + fs * 0.5;
if (labels.length && !spark) bottom -= small * 1.75;
top += vl === 'none' || spark ? lw : fs * 1.4;
if (!showAxis && !spark) left += labels.length ? tw(labels[0], small) / 2 : lw;
if (spark) { left += lw; bottom -= lw; }
const x0 = left, x1 = right, y0 = bottom, y1 = top, X = (i) => (n > 1 ? x0 + ((x1 - x0) * i) / (n - 1) : (x0 + x1) / 2), Y = (v) => y0 - ((v - lo) / (hiV - lo)) * (y0 - y1);
if (showGrid) for (const v of ticks) el(g, 'line', { x1: r2(x0), x2: r2(x1), y1: r2(Y(v)), y2: r2(Y(v)), stroke: gridc, 'stroke-width': r2(u) });
if (showAxis) for (const v of ticks) T(g, x0 - fs * 0.5, Y(v) + mid(small), fmt(v), { size: small, fill: muted, anchor: 'end' });
const path = (pts) => { if (!smooth || pts.length < 3) return pts.map((p, i) => (i ? 'L ' : 'M ') + r2(p[0]) + ' ' + r2(p[1])).join(' '); let d = 'M ' + r2(pts[0][0]) + ' ' + r2(pts[0][1]); for (let i = 0; i < pts.length - 1; i++) { const p0 = pts[Math.max(0, i - 1)], p1 = pts[i], p2 = pts[i + 1], p3 = pts[Math.min(pts.length - 1, i + 2)], t = 0.18; d += ' C ' + r2(p1[0] + (p2[0] - p0[0]) * t) + ' ' + r2(p1[1] + (p2[1] - p0[1]) * t) + ' ' + r2(p2[0] - (p3[0] - p1[0]) * t) + ' ' + r2(p2[1] - (p3[1] - p1[1]) * t) + ' ' + r2(p2[0]) + ' ' + r2(p2[1]); } return d; };
const P_ = cum.map((s) => s.map((v, i) => [X(i), Y(v)]));
for (let s = ns - 1; s >= 0; s--) if (area) {
  const base = stack && s > 0 ? P_[s - 1].slice().reverse() : [[X(n - 1), y0], [X(0), y0]];
  el(g, 'path', { d: path(P_[s]) + ' ' + base.map((p) => 'L ' + r2(p[0]) + ' ' + r2(p[1])).join(' ') + ' Z', fill: col(s), 'fill-opacity': stack ? 1 : ns > 1 ? 0.22 : 0.18 });
}
if (!spark) el(g, 'line', { x1: r2(x0), x2: r2(x1), y1: r2(y0), y2: r2(y0), stroke: muted, 'stroke-width': r2(1.5 * u) });
const step = Math.max(1, Math.ceil((labels.reduce((a, s) => Math.max(a, tw(s, small)), 0) + small) / Math.max(1, (x1 - x0) / Math.max(1, n - 1))));
if (!spark) labels.forEach((s, i) => { if (i % step === 0 || i === n - 1 && (n - 1) % step >= step / 2) T(g, X(i), y0 + small * 1.45, s, { size: small, fill: muted, anchor: 'middle' }); });
let ends = [];
for (let s = 0; s < ns; s++) {
  if (!stack) el(g, 'path', { d: path(P_[s]), fill: 'none', stroke: col(s), 'stroke-width': r2(lw), 'stroke-linejoin': 'round', 'stroke-linecap': 'round' });
  if (markers) P_[s].forEach((p) => el(g, 'circle', { cx: r2(p[0]), cy: r2(p[1]), r: r2(lw * 1.25), fill: bg === 'transparent' ? '#ffffff' : bg, stroke: col(s), 'stroke-width': r2(lw * 0.7) }));
  if (spark) { const p = P_[s][n - 1]; el(g, 'circle', { cx: r2(p[0]), cy: r2(p[1]), r: r2(lw * 1.3), fill: col(s) }); }
  P_[s].forEach((p, i) => { A('p' + (s + 1) + '.' + (i + 1), (labels[i] || '') + (names[s] ? ' / ' + names[s] : ''), p[0], p[1], { value: S[s][i] }); if (vl === 'all' && !stack) T(g, p[0], p[1] - fs * 0.65, fmt(S[s][i]), { size: small, weight: numwt, font: numfont, anchor: 'middle', fill: ns > 1 ? col(s) : ink }); });
  ends.push({ s, y: P_[s][n - 1][1] });
}
// end labels and last values, nudged apart so they never overlap
ends.sort((a, b) => a.y - b.y); const lh = (endl && vl === 'last' ? small * 2.25 : fs * 1.2); for (let k = 1; k < ends.length; k++) if (ends[k].y - ends[k - 1].y < lh) ends[k].y = ends[k - 1].y + lh;
for (const e of ends) { const s = e.s, xe = X(n - 1) + lw * 1.6 + fs * 0.25;
  if (endl) { T(g, xe, e.y + (vl === 'last' ? -small * 0.1 : mid(small)), names[s], { size: small, weight: 700, fill: col(s) }); if (vl === 'last') T(g, xe, e.y + small * 1.05, fmt(S[s][n - 1]), { size: small, fill: muted, font: numfont }); }
  else if (vl === 'last' && !stack) T(g, xe, e.y + mid(fs), fmt(S[s][n - 1]), { size: fs, weight: numwt, font: numfont, fill: ns > 1 ? col(s) : ink });
}
done('line', { plot: [x0, y1, x1 - x0, y0 - y1].map(Math.round), min: lo, max: hiV });
