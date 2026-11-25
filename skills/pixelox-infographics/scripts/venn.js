// venn.js: a Venn diagram of 2, 3, or 4 overlapping sets: translucent circles whose overlaps mix, a name in
// each set, a label in each overlap, and one in the middle. Shows what things share; areas are not to scale.
// Run:   pixelox render js scripts/venn.js -o venn.png --size 1080x1350 --param box=8,28,84,46 --param labels='Useful|Usable|Beautiful' --param overlaps='works|admired|pleasant' --param center='Loved'
// Place: pixelox comp place d.pxo venn.png --in-place --as Venn --at 0,0
// Data params:
//   labels=A|B|C             2 to 4 set names (2: side by side; 3: one on top, two below; 4: top, right, bottom, left)
//   overlaps=ab|ac|bc        labels of the pairwise overlaps: 2 sets: ab; 3 sets: ab, ac, bc (top-left, top-right,
//                            bottom); 4 sets: the four neighbor overlaps clockwise from the top-right
//   center=                  label of the area all sets share
//   notes=...|...            a short note outside each circle (2 or 4 sets), same order as labels
// Form params (all optional):
//   opacity=0.78             circle opacity
//   blend=multiply           multiply (light grounds: overlaps darken) | screen (dark grounds: overlaps glow) | normal
//   overlap=0.5              how far the circles overlap: 0.3 (a sliver) to 0.7 (a lot)
//   labelcolor=              color of the names inside the circles (default: white or ink, by contrast)
//   caps=0                   1 = set names in capitals with wide tracking
//   titlesize=0              set name size in px (0 = 1.35 x fs)
// Shared params (every chart script): box=x,y,w,h in % of the image (default 0,0,100,100); colors=#hex,#hex;
//   ink=#1f2933 (text) muted=#6b7785 (axis text) grid=#d5dbe1 bg=transparent; fs=label size in px;
//   font='Family, Fallback, sans-serif' (a family name: font classes such as @condensed do not work inside a script)
//   numfont= weight=400 numweight=700; prefix=$ suffix=% decimals=1 compact=1 (1.2K, 3.4M)
// The result's logs hold one JSON line: the circle radius and an anchor at every circle center, overlap, and the middle.
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
const labels = list('labels', 'Useful|Usable|Beautiful'), n = Math.max(2, Math.min(4, labels.length)), overlaps = list('overlaps'), notes = list('notes'), ctr = str('center', '');
const op = num('opacity', 0.78), blend = str('blend', 'multiply').toLowerCase(), ov = Math.max(0.2, Math.min(0.8, num('overlap', 0.5))), caps = on('caps', 0), ts = num('titlesize', 0) || fs * 1.35, lc = str('labelcolor', '');
const g = el(svg, 'g', {}), small = fs * 0.95;
// unit layout: circle radius 1, centers on a ring of radius d around the origin
const d = n === 2 ? 1 - ov * 0.9 : n === 3 ? 1.15 - ov * 0.9 : 1.2 - ov * 0.8, angs = n === 2 ? [270, 90] : n === 3 ? [0, 120, 240] : [0, 90, 180, 270], U = angs.map((a) => pt(0, 0, d, a));
const nh = notes.length ? fs * 3 : 0, nwid = notes.length && n !== 3 ? B.w * 0.16 : 0;
const minx = Math.min.apply(null, U.map((p) => p[0])) - 1, maxx = Math.max.apply(null, U.map((p) => p[0])) + 1, miny = Math.min.apply(null, U.map((p) => p[1])) - 1, maxy = Math.max.apply(null, U.map((p) => p[1])) + 1;
const R = Math.min((B.w - 2 * nwid) / (maxx - minx), (B.h - (n === 4 ? 2 * nh : n === 2 ? 0 : 0)) / (maxy - miny)), cx = B.x + B.w / 2 - (R * (minx + maxx)) / 2, cy = B.y + B.h / 2 - (R * (miny + maxy)) / 2;
const Cn = U.map((p) => [cx + p[0] * R, cy + p[1] * R]), gg = el(g, 'g', { style: 'isolation:isolate' });
Cn.forEach((c, i) => { el(gg, 'circle', { cx: r2(c[0]), cy: r2(c[1]), r: r2(R), fill: col(i), 'fill-opacity': op, style: blend === 'normal' ? undefined : 'mix-blend-mode:' + blend }); A('set' + (i + 1), labels[i] || '', c[0], c[1], { r: Math.round(R) }); });
const fit = (s, size, maxw, bold) => Math.min(size, (maxw / Math.max(1, tw(s, 1, bold))));
// set names: pushed outward from the middle into each circle's own area
Cn.forEach((c, i) => { const out = pt(c[0], c[1], R * (n === 2 ? 0.42 : n === 3 ? 0.38 : 0.45), angs[i]), s = caps ? (labels[i] || '').toUpperCase() : labels[i] || '', f = lc || onc(mix(col(i), blend === 'screen' ? '#000000' : '#ffffff', 1 - op)), lines = wrap(s, R * (n === 2 ? 0.95 : 1.05), ts, 1), sz = Math.min.apply(null, lines.map((ln) => fit(ln, ts, R * 1.1, 1)));
  lines.forEach((ln, k) => T(g, out[0], out[1] + mid(sz) + (k - (lines.length - 1) / 2) * sz * 1.15, ln, { size: sz, weight: 700, anchor: 'middle', fill: f, spacing: caps ? r2(sz * 0.08) : undefined })); });
// pairwise overlaps
const pairs = n === 2 ? [[0, 1]] : n === 3 ? [[0, 2], [0, 1], [1, 2]] : [[0, 1], [1, 2], [2, 3], [3, 0]];
pairs.forEach((pr, k) => { const a = Cn[pr[0]], b = Cn[pr[1]], m = [(a[0] + b[0]) / 2, (a[1] + b[1]) / 2], dx = m[0] - cx, dy = m[1] - cy, len = Math.hypot(dx, dy) || 1, push = n === 2 ? 0 : R * (n === 3 ? 0.3 : 0.24), p = [m[0] + (dx / len) * push, m[1] + (dy / len) * push];
  A('overlap' + (k + 1), overlaps[k] || '', p[0], p[1]); if (!overlaps[k]) return; const maxw = n === 2 ? R * (2 - 2 * d) * 0.82 : R * 0.62, lines = wrap(overlaps[k], maxw, small), sz = Math.min.apply(null, lines.map((ln) => fit(ln, small, maxw * 1.1)));
  lines.forEach((ln, j) => T(g, p[0], p[1] + mid(sz) + (j - (lines.length - 1) / 2) * sz * 1.2, ln, { size: sz, anchor: 'middle', fill: lc || (blend === 'screen' ? '#1f2933' : '#ffffff'), weight: 500 })); });
if (ctr && n > 2) { const maxw = R * (n === 3 ? 0.62 : 0.6), lines = wrap(caps ? ctr.toUpperCase() : ctr, maxw, ts * 0.9, 1), sz = Math.min.apply(null, lines.map((ln) => fit(ln, ts * 0.9, maxw, 1))); lines.forEach((ln, j) => T(g, cx, cy + mid(sz) + (j - (lines.length - 1) / 2) * sz * 1.12, ln, { size: sz, weight: 700, anchor: 'middle', fill: lc || (blend === 'screen' ? '#1f2933' : '#ffffff') })); }
A('center', ctr, cx, cy);
// notes outside the circles
if (notes.length && n !== 3) Cn.forEach((c, i) => { if (!notes[i]) return; const a = angs[i], horiz = a === 90 || a === 270, lines = []; for (const para of notes[i].split('\\n')) for (const ln of wrap(para, horiz ? nwid - fs * 0.6 : R * 1.6, small)) lines.push(ln);
  if (horiz) { const x = a === 90 ? c[0] + R + fs * 0.6 : c[0] - R - fs * 0.6; lines.forEach((ln, k) => T(g, x, c[1] + mid(small) + (k - (lines.length - 1) / 2) * small * 1.3, ln, { size: small, fill: muted, anchor: a === 90 ? 'start' : 'end' })); }
  else lines.forEach((ln, k) => T(g, c[0], a === 0 ? c[1] - R - fs * 0.7 - (lines.length - 1 - k) * small * 1.3 : c[1] + R + fs * 1.4 + k * small * 1.3, ln, { size: small, fill: muted, anchor: 'middle' })); });
done('venn', { radius: Math.round(R), sets: n });
