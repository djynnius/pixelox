// tilemap.js: a tile-grid map (cartogram): one equal tile per region, colored by a category or by a value.
// Built in: the 50 US states plus DC (layout=us). Any other set of regions: give your own grid with cells=.
// For real state outlines use usmap.js (pixelox 0.6.0 or later); this script runs on any build.
// This is not a geographic map: it shows which regions belong to which class, with every region equally visible.
// Run:   pixelox render js scripts/tilemap.js -o map.png --size 1080x1350 --param box=6,30,88,42 --param groups='CA:1,OR:1,WA:1,TX:2,FL:2,OH:3' --param names='Group A,Group B,Open' --param colors='#1f6fb5,#c8241b,#8b5cf6'
// Place: pixelox comp place d.pxo map.png --in-place --as Map --at 0,0
// Data params (use one form):
//   groups=CA:1,TX:2,OH:3    region code : 1-based class number (class colors come from colors=, names from names=)
//   values=CA:31,TX:44       region code : number, colored on a ramp from the smallest to the largest value
//   names=Group A,Group B    class names for the legend (groups form)
//   Both together: groups= sets the colors and values= with showvalues=1 prints a number on each tile.
// Form params (all optional):
//   layout=us                us = the built-in US grid (11 columns x 8 rows)
//   cells=AA:0:0,BB:1:0      your own grid instead: code:column:row for every tile (0-based, row 0 at the top)
//   shape=square             square | hex | circle (hex shifts every second row by half a tile)
//   gap=0.08                 space between tiles as a fraction of a tile
//   ramp=#dbeafe,#1d4ed8     values form: low color, high color (three colors make a diverging ramp)
//   min= max=                values form: the numbers the ramp ends stand for (default: the data's range)
//   empty=#e2e6ea            color of regions without data
//   codes=1 showvalues=0     write the region code on each tile; write the value under it
//   legend=bottom            bottom | top | none
// Layout note: the US grid was arranged by hand for this skill (a common tile-map arrangement); it carries no map data.
// Shared params (every chart script): box=x,y,w,h in % of the image (default 0,0,100,100); colors=#hex,#hex;
//   ink=#1f2933 (text) muted=#6b7785 (axis text) grid=#d5dbe1 bg=transparent; fs=label size in px;
//   font='Family, Fallback, sans-serif' (a family name: font classes such as @condensed do not work inside a script)
//   numfont= weight=400 numweight=700; prefix=$ suffix=% decimals=1 compact=1 (1.2K, 3.4M)
// The result's logs hold one JSON line: the tile size and an anchor at the center of every tile.
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
const US = 'AK:0:0,ME:10:0,VT:9:1,NH:10:1,WA:0:2,ID:1:2,MT:2:2,ND:3:2,MN:4:2,IL:5:2,WI:6:2,MI:7:2,NY:8:2,RI:9:2,MA:10:2,OR:0:3,NV:1:3,WY:2:3,SD:3:3,IA:4:3,IN:5:3,OH:6:3,PA:7:3,NJ:8:3,CT:9:3,CA:0:4,UT:1:4,CO:2:4,NE:3:4,MO:4:4,KY:5:4,WV:6:4,VA:7:4,MD:8:4,DE:9:4,AZ:1:5,NM:2:5,KS:3:5,AR:4:5,TN:5:5,NC:6:5,SC:7:5,DC:8:5,OK:3:6,LA:4:6,MS:5:6,AL:6:6,GA:7:6,HI:0:7,TX:3:7,FL:8:7';
const cells = split(str('cells', US)).map((c) => { const p = c.split(':'); return { code: p[0].trim(), c: parseInt(p[1], 10) || 0, r: parseInt(p[2], 10) || 0 }; });
const kv = (k) => { const m = {}; for (const it of split(str(k, ''))) { const p = it.split(':'); if (p.length > 1) m[p[0].trim().toUpperCase()] = parseFloat(p[1]); } return m; };
const groups = kv('groups'), values = kv('values'), hasVal = Object.keys(values).length > 0, byVal = hasVal && Object.keys(groups).length === 0, names = list('names'), ramp = list('ramp', '#dbeafe,#1d4ed8'), empty = str('empty', '#e2e6ea');
const shape = str('shape', 'square').toLowerCase(), gap = num('gap', 0.08), showCodes = on('codes', 1), showVals = on('showvalues', 0) && hasVal, g = el(svg, 'g', {}), small = fs * 0.9;
const vs = Object.keys(values).map((k) => values[k]), lo = P.min !== undefined && P.min !== '' ? num('min', 0) : Math.min.apply(null, vs.length ? vs : [0]), hi = P.max !== undefined && P.max !== '' ? num('max', 1) : Math.max.apply(null, vs.length ? vs : [1]);
const rampAt = (t) => { t = Math.max(0, Math.min(1, t)); if (ramp.length >= 3) return t < 0.5 ? mix(ramp[0], ramp[1], t * 2) : mix(ramp[1], ramp[2], (t - 0.5) * 2); return mix(ramp[0], ramp[1] || ramp[0], t); };
const colorOf = (code) => { const k = code.toUpperCase(); if (byVal) return values[k] === undefined ? empty : rampAt(hi > lo ? (values[k] - lo) / (hi - lo) : 1); return groups[k] ? col(Math.round(groups[k]) - 1) : empty; };
const lg = str('legend', byVal || names.length ? 'bottom' : 'none').toLowerCase(); let ay = B.y, ah = B.h;
const used = []; for (const k in groups) if (used.indexOf(Math.round(groups[k])) < 0) used.push(Math.round(groups[k])); used.sort((a, b) => a - b);
const items = used.map((i) => ({ label: names[i - 1] || 'Group ' + i, color: col(i - 1) }));
const lgH = lg === 'none' ? 0 : byVal ? small * 2.6 : Math.ceil(items.reduce((a, it) => a + small * 2.6 + tw(it.label, small), 0) / B.w) * small * 1.7;
const drawLegend = (y) => { if (byVal) { const w = Math.min(B.w * 0.42, 360 * (fs / 20)), x = B.x + (B.w - w) / 2, id = 'ramp', lgd = el(defs, 'linearGradient', { id, x1: 0, y1: 0, x2: 1, y2: 0 }); ramp.forEach((c, i) => el(lgd, 'stop', { offset: ramp.length > 1 ? i / (ramp.length - 1) : 0, 'stop-color': c })); el(g, 'rect', { x: r2(x), y: r2(y), width: r2(w), height: r2(small * 0.7), rx: r2(small * 0.2), fill: 'url(#ramp)' }); T(g, x - small * 0.5, y + small * 0.35 + mid(small), fmt(lo), { size: small, fill: muted, anchor: 'end' }); T(g, x + w + small * 0.5, y + small * 0.35 + mid(small), fmt(hi), { size: small, fill: muted }); } else legend(g, items, B.x + B.w / 2, y, B.w, small); };
if (lg === 'top') { drawLegend(B.y); ay += lgH + fs * 0.5; ah -= lgH + fs * 0.5; } else if (lg !== 'none') { drawLegend(B.y + B.h - lgH + small * 0.3); ah -= lgH + fs * 0.6; }
const nc = Math.max.apply(null, cells.map((c) => c.c)) + 1, nr = Math.max.apply(null, cells.map((c) => c.r)) + 1, hexS = shape === 'hex';
const cw = Math.min(B.w / (nc + (hexS ? 0.5 : 0)), ah / (hexS ? (nr - 1) * 0.866 + 1.1547 : nr)), ph = hexS ? cw * 0.866 : cw, gw = cw * (nc + (hexS ? 0.5 : 0)), gh = hexS ? ph * (nr - 1) + cw * 1.1547 : cw * nr;
const gx = B.x + (B.w - gw) / 2, gy = ay + (ah - gh) / 2, s = cw * (1 - gap), cs = Math.min(fs, s * (showVals ? 0.3 : 0.36));
for (const c of cells) {
  const x = gx + c.c * cw + (hexS && c.r % 2 ? cw / 2 : 0) + cw / 2, y = gy + c.r * ph + (hexS ? cw * 0.577 : cw / 2), f0 = colorOf(c.code), clear = /^(transparent|none)$/i.test(f0), f = clear ? 'none' : f0;
  if (shape === 'circle') el(g, 'circle', { cx: r2(x), cy: r2(y), r: r2(s / 2), fill: f });
  else if (hexS) el(g, 'polygon', { points: [0, 60, 120, 180, 240, 300].map((d) => pt(x, y, (s / 2) * 1.1547, d).map(r2).join(',')).join(' '), fill: f });
  else el(g, 'rect', { x: r2(x - s / 2), y: r2(y - s / 2), width: r2(s), height: r2(s), rx: r2(s * 0.1), fill: f });
  const tc = clear ? ink : onc(f), v = values[c.code.toUpperCase()];
  if (showCodes) T(g, x, y + mid(cs) - (showVals && v !== undefined ? cs * 0.55 : 0), c.code, { size: cs, weight: 700, anchor: 'middle', fill: tc });
  if (showVals && v !== undefined) T(g, x, y + mid(cs) + cs * 0.62, fmt(v), { size: cs * 0.92, anchor: 'middle', fill: tc, font: numfont });
  A(c.code, c.code, x, y, byVal ? { value: v } : { group: groups[c.code.toUpperCase()] || 0 });
}
done('tilemap', { grid: [gx, gy, gw, gh].map(Math.round), tile: r1(cw), cols: nc, rows: nr });
