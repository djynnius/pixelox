// bars.js: columns and bars from data, with labels: single, grouped, stacked, 100% stacked, a line on a second
// axis (combo), progress bars on tracks, and two-sided (butterfly) bars. Bars always start at zero.
// Run:   pixelox render js scripts/bars.js -o bars.png --size 1080x1350 --param box=8,30,84,36 --param values=42,31,18,9 --param labels='North,South,East,West' --param suffix=%
// Place: pixelox comp place d.pxo bars.png --in-place --as Bars --at 0,0
// Data params:
//   values=42,31,18,9        one series; several series are separated by ';' (values='12,18,25;9,14,20')
//   labels=A,B,C,D           category names (use '|' between names that contain commas)
//   series=2024,2025         series names: turns the legend on
// Form params (all optional):
//   dir=v                    v = columns, h = horizontal bars (use h for long names or more than 7 categories)
//   mode=group               group | stack | percent (two or more series stacked to 100%; one series draws its plain
//                            values) | diverge (two series as left and right bars from a middle column of names: a butterfly chart)
//   max=0                    top of the value axis (0 = a round number above the data); ticks=4 divisions
//   axis=1 gridlines=1       value axis labels and gridlines (columns: on; bars: off; give axis=1 to turn on)
//   vlabels=1                value labels: 1 (at the bar end), inside (bars), 0
//   highlight=0 accent=#f97316   1-based category drawn in the accent color; the others take the first color
//   track=0 trackcolor=      1 = progress style: every bar lies on a full-length pale track (set max=100)
//   gap=0.36 radius=3        gap as a fraction of each band; corner radius of the bar end in px
//   line= linename= linecolor=#f2b705 linemax=0 linesuffix= markers=0   columns only: a line on its own right-hand axis
//   legend=top               top | bottom | none
//   labelwidth=0.34          bars only: the most the category names may take, as a fraction of the box width
//   labelpos=left            bars only: left (names beside the bars) | above (each name on its own line over its bar:
//                            full-width bars) | inside (names inside the bars)
// Columns and bars may go below zero in mode=group (values=12,-5,8): the axis then runs from a round negative number
// and zero is a line. stack, percent, diverge, and track=1 draw a negative value as 0, and the log's "warnings" says so.
// Shared params (every chart script): box=x,y,w,h in % of the image (default 0,0,100,100); colors=#hex,#hex;
//   ink=#1f2933 (text) muted=#6b7785 (axis text) grid=#d5dbe1 bg=transparent; fs=label size in px;
//   font='Family, Fallback, sans-serif' (a family name: font classes such as @condensed do not work inside a script)
//   numfont= weight=400 numweight=700; prefix=$ suffix=% decimals=1 compact=1 (1.2K, 3.4M)
// The result's logs hold one JSON line: the plot rectangle and an anchor (canvas px and %) for every bar end.
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
const S = rows('values', '42,31,18,9'), labels = list('labels'), names = list('series');
const n = Math.max.apply(null, S.map((s) => s.length)), ns = S.length;
const horiz = /^h/i.test(str('dir', 'v')), mode = str('mode', 'group').toLowerCase(), stacked = mode !== 'group' && mode !== 'diverge' && ns > 1, pct = stacked && mode === 'percent'; // percent needs two or more series
const tot = []; for (let i = 0; i < n; i++) tot.push(S.reduce((a, s) => a + Math.max(0, s[i] || 0), 0));
const val = (s, i) => (pct ? (tot[i] ? ((S[s][i] || 0) / tot[i]) * 100 : 0) : S[s][i] || 0);
const rawMax = pct ? 100 : stacked ? Math.max.apply(null, tot) : Math.max.apply(null, S.map((s) => Math.max.apply(null, s)));
const nt = Math.round(num('ticks', 4)), sc = num('max', 0) > 0 ? { max: num('max', 0), step: num('max', 0) / nt } : nice(rawMax * (on('vlabels', 1) && !horiz && !stacked ? 1.04 : 1), nt);
const u = Math.max(0.6, Math.min(B.w, B.h) / 500), small = fs * 0.9, track = on('track', 0), showAxis = on('axis', horiz || track ? 0 : 1), showGrid = on('gridlines', showAxis), vl = str('vlabels', track ? 'end' : '1').toLowerCase();
const showV = !/^(0|no|none|off|false)$/.test(vl), hi = Math.round(num('highlight', 0)), accent = str('accent', '#f97316'), perCat = ns === 1 && COLORS.length > 1 && !hi;
const fill = (s, i) => (hi ? (i + 1 === hi ? accent : col(0)) : perCat ? col(i) : col(s));
const line = rows('line', '')[0] || null, linec = str('linecolor', '#f2b705'), lsuf = str('linesuffix', suf);
const lsc = line ? (num('linemax', 0) > 0 ? { max: num('linemax', 0), step: num('linemax', 0) / nt } : nice(Math.max.apply(null, line), nt)) : null;
const g = el(svg, 'g', {});
// layout inside the box
let top = B.y, bottom = B.y + B.h, left = B.x, right = B.x + B.w;
const lg = mode !== 'diverge' && (names.length || (line && str('linename', ''))) ? str('legend', 'top').toLowerCase() : 'none', items = names.map((nm, s) => ({ label: nm, color: col(s) }));
if (line && str('linename', '')) items.push({ label: str('linename', ''), color: linec, line: 1 });
if (lg === 'top') top += legend(g, items, B.x + B.w / 2, top, B.w, small) + fs * 0.4;
if (lg === 'bottom') { const lh = Math.ceil(items.reduce((a, it) => a + small * 2.55 + tw(it.label, small), 0) / B.w) * small * 1.7; legend(g, items, B.x + B.w / 2, bottom - lh + small * 0.3, B.w, small); bottom -= lh + fs * 0.5; }
const rawMin = !stacked && mode !== 'diverge' && !track ? Math.min.apply(null, S.map((q) => Math.min.apply(null, q))) : 0, neg = rawMin < 0;
const clamped = rawMin === 0 && S.some((q) => q.some((v) => v < 0)); // stack, percent, diverge, and track draw a negative value as 0: said in the log
const nstep = neg ? stepFor(rawMin, rawMax, nt) : sc.step, loS = neg ? Math.floor(rawMin / nstep) * nstep : 0, hiS = neg ? Math.max(nstep, Math.ceil(Math.max(0, rawMax) / nstep - 1e-9) * nstep) : sc.max;
const tickLabels = []; for (let v = loS; v <= hiS + 1e-9; v += nstep) tickLabels.push(Math.abs(v) < 1e-9 ? 0 : v);
const tl = (v) => fmt(v) + (pct && !suf ? '%' : '');
let x0, x1, y0, y1;
if (mode === 'diverge' && ns >= 2) {
  const gut = (labels.length ? Math.max.apply(null, labels.map((q) => tw(q, fs))) : 0) + fs * 1.4, vw = showV ? Math.max.apply(null, S.slice(0, 2).map((q) => Math.max.apply(null, q.map((v) => tw(fmt(v), fs, 1))))) + fs * 0.5 : 0;
  const cxm = B.x + B.w / 2, sideW = (B.w - gut) / 2 - vw, dmax = num('max', 0) > 0 ? num('max', 0) : nice(Math.max.apply(null, S.slice(0, 2).map((q) => Math.max.apply(null, q))), nt).max;
  if (names.length) { top += fs * 0.2; [0, 1].forEach((k) => T(g, cxm + (k ? 1 : -1) * (gut / 2 + sideW / 2), top + small * 0.9, names[k] || '', { size: small, weight: 700, anchor: 'middle', fill: col(k) })); top += small * 1.9; }
  x0 = cxm - gut / 2 - sideW; x1 = cxm + gut / 2 + sideW; y0 = top; y1 = bottom;
  const band = (y1 - y0) / n, gap = num('gap', 0.3), bh = band * (1 - gap), rad = Math.min(num('radius', 3 * u), bh / 2);
  for (let i = 0; i < n; i++) { const y = y0 + band * i + (band - bh) / 2; [0, 1].forEach((k) => { const v = Math.max(0, S[k][i] || 0), w = (v / dmax) * sideW, dir = k ? 1 : -1, xs = cxm + dir * (gut / 2), c = col(k);
      if (track) el(g, 'rect', { x: r2(k ? xs : xs - sideW), y: r2(y), width: r2(sideW), height: r2(bh), rx: r2(rad), fill: str('trackcolor', mix(c, '#ffffff', 0.84)) });
      if (w > 0.01) el(g, 'rect', { x: r2(k ? xs : xs - w), y: r2(y), width: r2(w), height: r2(bh), rx: r2(Math.min(rad, w / 2)), fill: c });
      A('bar' + (i + 1) + '.' + (k + 1), (labels[i] || '') + (names[k] ? ' / ' + names[k] : ''), xs + dir * w, y + bh / 2, { value: v });
      if (showV) T(g, xs + dir * (w + fs * 0.4), y + bh / 2 + mid(fs), fmt(S[k][i] || 0), { weight: numwt, font: numfont, anchor: k ? 'start' : 'end' }); });
    if (labels[i]) T(g, cxm, y + bh / 2 + mid(fs), labels[i], { anchor: 'middle', fill: muted }); }
} else if (!horiz) {
  const maxLab = labels.reduce((a, s) => Math.max(a, tw(s, small)), 0);
  if (showAxis) left += Math.max.apply(null, tickLabels.map((v) => tw(tl(v), small))) + fs * 0.6;
  if (line && showAxis) right -= tw(fmt(lsc.max).replace(suf, '') + lsuf, small) + fs * 0.6;
  const band0 = (right - left) / n, two = labels.length && maxLab > band0 * 0.96;
  if (labels.length) bottom -= small * (two ? 2.6 : 1.5) + fs * 0.25;
  if (showV && !stacked) top += fs * 1.35; else top += small * 0.5;
  if (neg && showV) bottom -= fs * 1.3;
  x0 = left; x1 = right; y0 = bottom; y1 = top;
  const band = (x1 - x0) / n, gap = num('gap', ns > 1 && !stacked ? 0.28 : 0.36), inner = band * (1 - gap), bw = stacked ? inner : inner / ns, rad = Math.min(num('radius', 3 * u), bw / 2);
  if (neg && showV) bottom -= 0; const YV = (v) => y0 - ((v - loS) / (hiS - loS)) * (y0 - y1), base = YV(0);
  if (showGrid) for (const v of tickLabels) { const y = YV(v); el(g, 'line', { x1: r2(x0), x2: r2(x1), y1: r2(y), y2: r2(y), stroke: gridc, 'stroke-width': r2(u) }); }
  if (showAxis) for (const v of tickLabels) T(g, x0 - fs * 0.5, YV(v) + mid(small), tl(v), { size: small, fill: muted, anchor: 'end' });
  const vbar = (x, y, w, h, r) => (h <= 0.01 ? '' : (r = Math.min(r, h, w / 2), 'M ' + r2(x) + ' ' + r2(y + h) + ' L ' + r2(x) + ' ' + r2(y + r) + ' Q ' + r2(x) + ' ' + r2(y) + ' ' + r2(x + r) + ' ' + r2(y) + ' L ' + r2(x + w - r) + ' ' + r2(y) + ' Q ' + r2(x + w) + ' ' + r2(y) + ' ' + r2(x + w) + ' ' + r2(y + r) + ' L ' + r2(x + w) + ' ' + r2(y + h) + ' Z'));
  for (let i = 0; i < n; i++) {
    const bx0 = x0 + band * i + (band - inner) / 2; let acc = 0;
    for (let s = 0; s < ns; s++) {
      if (neg) { const v0 = S[s][i] || 0, c = fill(s, i), x = bx0 + bw * s, yt = YV(Math.max(v0, 0)), yb = YV(Math.min(v0, 0)), wB = bw - (ns > 1 ? u : 0); if (yb - yt > 0.01) el(g, 'rect', { x: r2(x), y: r2(yt), width: r2(wB), height: r2(yb - yt), rx: r2(Math.min(rad, (yb - yt) / 2)), fill: c });
        A('bar' + (i + 1) + (ns > 1 ? '.' + (s + 1) : ''), labels[i] || '', x + bw / 2, v0 >= 0 ? yt : yb, { value: v0, base: Math.round(base) }); if (showV) T(g, x + bw / 2, v0 >= 0 ? yt - fs * 0.42 : yb + fs * 1.15, fmt(v0), { size: ns > 2 ? small : fs, weight: numwt, font: numfont, anchor: 'middle', fill: hi && i + 1 === hi ? accent : ink }); continue; }
      const v = Math.max(0, val(s, i)), h = (v / sc.max) * (y0 - y1), x = stacked ? bx0 : bx0 + bw * s, y = y0 - (stacked ? acc + h : h), last = !stacked || s === ns - 1, c = fill(s, i);
      if (track && !stacked) el(g, 'path', { d: vbar(x, y1, bw - (ns > 1 ? u : 0), y0 - y1, rad), fill: str('trackcolor', mix(c, '#ffffff', 0.82)) });
      if (h > 0.01) el(g, 'path', { d: vbar(x, y, bw - (ns > 1 && !stacked ? u : 0), h, last ? rad : 0), fill: c });
      A('bar' + (i + 1) + (ns > 1 ? '.' + (s + 1) : ''), (labels[i] || '') + (names[s] ? ' / ' + names[s] : ''), x + bw / 2, y, { value: S[s][i] || 0, base: Math.round(y0) });
      if (showV && !stacked) T(g, x + bw / 2, y - fs * 0.42, fmt(S[s][i] || 0), { size: ns > 2 ? small : fs, weight: numwt, font: numfont, anchor: 'middle', fill: hi && i + 1 === hi ? accent : ink });
      if (showV && stacked && h > small * 1.5) T(g, x + bw / 2, y + h / 2 + mid(small), pct ? Math.round(v) + '%' : fmt(S[s][i] || 0), { size: small, weight: numwt, font: numfont, anchor: 'middle', fill: onc(c) });
      acc += h;
    }
    if (labels[i]) { const ls = two ? wrap(labels[i], band * 0.96, small) : [labels[i]]; ls.slice(0, 2).forEach((s, k) => T(g, x0 + band * (i + 0.5), y0 + small * (1.35 + k * 1.15), s, { size: small, fill: hi && i + 1 === hi ? ink : muted, weight: hi && i + 1 === hi ? 700 : wt, anchor: 'middle' })); }
  }
  el(g, 'line', { x1: r2(x0), x2: r2(x1), y1: r2(base), y2: r2(base), stroke: muted, 'stroke-width': r2(1.5 * u) });
  if (line) {
    const pts = line.map((v, i) => [x0 + band * (i + 0.5), y0 - (v / lsc.max) * (y0 - y1)]);
    el(g, 'polyline', { points: pts.map((p) => r2(p[0]) + ',' + r2(p[1])).join(' '), fill: 'none', stroke: linec, 'stroke-width': r2(num('linewidth', 5 * u)), 'stroke-linejoin': 'round', 'stroke-linecap': 'round' });
    if (on('markers', 0)) pts.forEach((p) => el(g, 'circle', { cx: r2(p[0]), cy: r2(p[1]), r: r2(5.5 * u), fill: '#ffffff', stroke: linec, 'stroke-width': r2(3 * u) }));
    if (showAxis) for (let v = 0; v <= lsc.max + 1e-9; v += lsc.step) T(g, x1 + fs * 0.5, y0 - (v / lsc.max) * (y0 - y1) + mid(small), fmt(v).replace(suf, '') + lsuf, { size: small, fill: muted });
    pts.forEach((p, i) => A('line' + (i + 1), labels[i] || '', p[0], p[1], { value: line[i] }));
  }
} else {
  const lpos = str('labelpos', 'left').toLowerCase(), labw = labels.length && lpos === 'left' ? Math.min(B.w * num('labelwidth', 0.34), Math.max.apply(null, labels.map((s) => tw(s, fs))) + fs * 0.7) : 0, nameH = lpos === 'above' && labels.length ? fs * 1.45 : 0;
  left += labw;
  const negRoom = neg && showV ? Math.max.apply(null, [].concat.apply([], S).filter((v) => v < 0).map((v) => tw(fmt(v), fs, 1))) + fs * 0.6 : 0; // room for the values of bars that run left of zero
  left += negRoom;
  if (showV && vl !== 'inside' && !stacked) right -= Math.max.apply(null, S.map((s) => Math.max.apply(null, s.map((v) => tw(fmt(v), fs, 1))))) + fs * 0.6;
  if (showAxis) bottom -= small * 1.7;
  x0 = left; x1 = right; y0 = top; y1 = bottom;
  const band = (y1 - y0) / n, gap = num('gap', ns > 1 && !stacked ? 0.3 : track ? 0.52 : 0.34), inner = band * (1 - gap), bh = stacked ? inner - nameH : (inner - nameH) / ns, rad = Math.min(num('radius', track ? bh : 3 * u), bh / 2);
  const XV = (v) => (neg ? x0 + ((v - loS) / (hiS - loS)) * (x1 - x0) : x0 + (v / sc.max) * (x1 - x0)), xz = XV(0);
  if (showGrid) for (const v of tickLabels) { const x = XV(v); el(g, 'line', { x1: r2(x), x2: r2(x), y1: r2(y0), y2: r2(y1), stroke: gridc, 'stroke-width': r2(u) }); }
  if (showAxis) for (const v of tickLabels) T(g, XV(v), y1 + small * 1.35, tl(v), { size: small, fill: muted, anchor: 'middle' });
  const hbar = (x, y, w, h, r, both) => (w <= 0.01 ? '' : (r = Math.min(r, w, h / 2), 'M ' + r2(x + (both ? r : 0)) + ' ' + r2(y) + ' L ' + r2(x + w - r) + ' ' + r2(y) + ' Q ' + r2(x + w) + ' ' + r2(y) + ' ' + r2(x + w) + ' ' + r2(y + r) + ' L ' + r2(x + w) + ' ' + r2(y + h - r) + ' Q ' + r2(x + w) + ' ' + r2(y + h) + ' ' + r2(x + w - r) + ' ' + r2(y + h) + ' L ' + r2(x + (both ? r : 0)) + ' ' + r2(y + h) + (both ? ' Q ' + r2(x) + ' ' + r2(y + h) + ' ' + r2(x) + ' ' + r2(y + h - r) + ' L ' + r2(x) + ' ' + r2(y + r) + ' Q ' + r2(x) + ' ' + r2(y) + ' ' + r2(x + r) + ' ' + r2(y) : '') + ' Z'));
  for (let i = 0; i < n; i++) {
    const by0 = y0 + band * i + (band - inner) / 2; let acc = 0;
    for (let s = 0; s < ns; s++) {
      if (neg) { const v0 = S[s][i] || 0, c = fill(s, i), y = nameH + by0 + bh * s, hh = bh - (ns > 1 ? u : 0), xa = XV(Math.min(v0, 0)), xb = XV(Math.max(v0, 0)); if (xb - xa > 0.01) el(g, 'rect', { x: r2(xa), y: r2(y), width: r2(xb - xa), height: r2(hh), rx: r2(Math.min(rad, (xb - xa) / 2)), fill: c });
        A('bar' + (i + 1) + (ns > 1 ? '.' + (s + 1) : ''), (labels[i] || '') + (names[s] ? ' / ' + names[s] : ''), v0 >= 0 ? xb : xa, y + hh / 2, { value: v0, base: Math.round(xz) });
        if (showV) T(g, v0 >= 0 ? xb + fs * 0.4 : xa - fs * 0.4, y + hh / 2 + mid(ns > 2 ? small : fs), fmt(v0), { size: ns > 2 ? small : fs, weight: numwt, font: numfont, anchor: v0 >= 0 ? 'start' : 'end', fill: hi && i + 1 === hi ? accent : ink }); continue; }
      const v = Math.max(0, val(s, i)), w = (v / sc.max) * (x1 - x0), y = nameH + (stacked ? by0 : by0 + bh * s), x = x0 + (stacked ? acc : 0), last = !stacked || s === ns - 1, c = fill(s, i), hh = bh - (ns > 1 && !stacked ? u : 0);
      if (track && !stacked) el(g, 'path', { d: hbar(x0, y, x1 - x0, hh, rad, 1), fill: str('trackcolor', mix(c, '#ffffff', 0.82)) });
      if (w > 0.01) el(g, 'path', { d: hbar(x, y, Math.max(w, track ? hh : 0), hh, last ? rad : 0, track), fill: c });
      A('bar' + (i + 1) + (ns > 1 ? '.' + (s + 1) : ''), (labels[i] || '') + (names[s] ? ' / ' + names[s] : ''), x + w, y + hh / 2, { value: S[s][i] || 0, base: Math.round(x0) });
      if (showV && !stacked) { const s0 = fmt(S[s][i] || 0), inside = vl === 'inside' && w > tw(s0, fs, 1) + fs; T(g, vl === 'inside' ? (inside ? x + w - fs * 0.5 : x + w + fs * 0.4) : track ? x1 + fs * 0.6 : x + w + fs * 0.4, y + hh / 2 + mid(ns > 2 ? small : fs), s0, { size: ns > 2 ? small : fs, weight: numwt, font: numfont, anchor: inside ? 'end' : 'start', fill: inside ? onc(c) : hi && i + 1 === hi ? accent : ink }); }
      if (showV && stacked && w > tw(fmt(S[s][i] || 0), small, 1) + small * 0.8) T(g, x + w / 2, y + hh / 2 + mid(small), pct ? Math.round(v) + '%' : fmt(S[s][i] || 0), { size: small, weight: numwt, font: numfont, anchor: 'middle', fill: onc(c) });
      acc += w;
    }
    if (labels[i] && lpos === 'above') T(g, x0, by0 + fs * 0.95, labels[i], { size: fs, fill: hi && i + 1 === hi ? ink : muted, weight: hi && i + 1 === hi ? 700 : wt });
    else if (labels[i] && lpos === 'inside') { const w0 = (Math.max(0, val(0, i)) / sc.max) * (x1 - x0), fits = stacked || w0 > tw(labels[i], fs, 1) + fs * 1.2 + (vl === 'inside' ? tw(fmt(S[0][i] || 0), fs, 1) + fs : 0); T(g, fits ? x0 + fs * 0.6 : x0 + w0 + fs * 0.5 + (showV && vl !== 'inside' && !track ? tw(fmt(S[0][i] || 0), fs, 1) + fs * 0.6 : 0), by0 + inner / 2 + mid(fs), labels[i], { size: fs, weight: 600, fill: fits ? onc(fill(0, i)) : ink }); }
    else if (labels[i]) T(g, x0 - negRoom - fs * 0.6, by0 + inner / 2 + mid(fs), labels[i], { size: fs, anchor: 'end', fill: hi && i + 1 === hi ? ink : muted, weight: hi && i + 1 === hi ? 700 : wt });
  }
  if (!track) el(g, 'line', { x1: r2(xz), x2: r2(xz), y1: r2(y0), y2: r2(y1), stroke: muted, 'stroke-width': r2(1.5 * u) });
}
done('bars', { plot: [x0, Math.min(y0, y1), x1 - x0, Math.abs(y1 - y0)].map(Math.round), min: neg ? loS : 0, max: neg ? hiS : sc.max, warnings: clamped ? ['negative values are drawn as 0 in mode=' + mode + (track ? ' with track=1' : '') + '; use mode=group without a track to show them below zero'] : undefined });
