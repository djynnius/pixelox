// donut.js: part-of-a-whole charts from data, with labels: donut, pie, half donut, arc band, gauge, progress
// ring, and tick ring. Angles are proportional to the values. Output is a transparent image.
// Run:   pixelox render js scripts/donut.js -o donut.png --size 1080x1350 --param box=10,30,80,40 --param values=45,25,20,10 --param labels='Online,Stores,Partners,Other' --param center=120K --param sub='orders'
// Place: pixelox comp place d.pxo donut.png --in-place --as Donut --at 0,0
// Data params:
//   values=45,25,20,10       one number per slice (any unit: shares are computed)
//   labels=A,B,C,D           slice names (use '|' between names that contain commas)
//   value=67 max=100         kind=ring only: one value on a track
// Form params (all optional):
//   kind=donut               donut | ring (one value on a pale track: progress ring, gauge) | rings (see below)
//   inner=0.62               hole radius as a fraction of the outer radius (0 = pie; 0.8 = thin ring)
//   sweep=360 start=0        degrees covered and where the first slice starts, clockwise from 12 o'clock
//                            (sweep=180 gives a half donut and starts at 9 o'clock unless start is given)
//   labelpos=out             out (name and number beside each slice, with a leader) | in (number inside the slice)
//                            | legend (names in a legend, numbers inside) | none
//   show=pct                 pct | value | both | none: the number written for each slice
//   center= sub=             big text and small text in the hole (ring: center defaults to the value)
//   centersize=              size of the center text in px (default: fitted to the middle)
//   gap=0 sep=#ffffff        degrees left open between slices; color of the hairline between slices (sep=none)
//   ticks=0                  N > 0 draws the ring as N radial ticks colored by share (tick ring)
//   round=1 trackcolor=      ring only: round ends; the track color (default: the color mixed toward white)
//   minlabel=4               slices smaller than this percent get no inside number
//   radii=1,0.8,0.6          length of each slice as a fraction of the ring's depth (1 = out to the full radius,
//                            0.5 = halfway from the hole's edge): a polar-area ("rose") chart whose slices also
//                            differ in length. Angles still carry the values
//   gappx=0 corner=0         a parallel gap between slices in px, and rounded slice corners in px (petals, fan panels)
//   kind=rings               several values as concentric arcs on one center, each on its own track (max=100;
//                            sweep up to 300); names and numbers sit at the arcs' starts
//   cx= cy= r=               place by center instead of box: center x in % of the width, center y in % of the height,
//                            outer radius in % of the width (for rings around a picture, or two rings on one center)
// Shared params (every chart script): box=x,y,w,h in % of the image (default 0,0,100,100); colors=#hex,#hex;
//   ink=#1f2933 (text) muted=#6b7785 (axis text) grid=#d5dbe1 bg=transparent; fs=label size in px;
//   font='Family, Fallback, sans-serif' (a family name: font classes such as @condensed do not work inside a script)
//   numfont= weight=400 numweight=700; prefix=$ suffix=% decimals=1 compact=1 (1.2K, 3.4M)
// The result's logs hold one JSON line: the center, the radii, and an anchor (canvas px and %) in the middle of every slice.
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
const kind = str('kind', 'donut').toLowerCase(), ring = kind === 'ring' || kind === 'gauge';
const vals = ring ? [Math.max(0, num('value', 67))] : (rows('values', '45,25,20,10')[0] || []).map((v) => Math.max(0, v)), labels = list('labels');
const total = ring ? num('max', 100) : vals.reduce((a, b) => a + b, 0) || 1, n = vals.length;
const sweep = Math.min(360, num('sweep', 360)), start = P.start === undefined || P.start === '' ? (sweep < 360 ? -sweep / 2 : 0) : num('start', 0);
const inner = Math.max(0, Math.min(0.95, num('inner', ring ? 0.78 : 0.62))), lp = ring || kind === 'rings' ? 'none' : str('labelpos', 'out').toLowerCase(), show = str('show', 'pct').toLowerCase();
const gapA = num('gap', 0), sep = str('sep', '#ffffff'), nticks = Math.round(num('ticks', 0)), minl = num('minlabel', 4);
const radii = rows('radii', '')[0] || [], gappx = num('gappx', 0), corner = num('corner', 0), multi = kind === 'rings';
// sector with a parallel gap of g px on both sides (the gap keeps its width from the hole to the rim)
const sectorG = (cx, cy, r0, r1, a0, a1, g) => { const h = (r) => (Math.asin(Math.min(1, g / 2 / Math.max(r, g / 2 + 0.01))) * 180) / Math.PI, ho = h(r1), hi = h(Math.max(r0, 1)), big = a1 - a0 - 2 * ho > 180 ? 1 : 0, p = (r, a) => pt(cx, cy, r, a).map(r2).join(' '); if (a1 - a0 - 2 * ho <= 0.1) return ''; return 'M ' + p(r1, a0 + ho) + ' A ' + r2(r1) + ' ' + r2(r1) + ' 0 ' + big + ' 1 ' + p(r1, a1 - ho) + (r0 <= 0.5 ? ' L ' + p(Math.max(1, (g / 2) / Math.sin((Math.max(1, Math.min(179, a1 - a0)) * Math.PI) / 360)), (a0 + a1) / 2) : ' L ' + p(r0, Math.max(a0 + hi, Math.min(a1 - hi, a1 - hi))) + ' A ' + r2(r0) + ' ' + r2(r0) + ' 0 ' + (a1 - a0 - 2 * hi > 180 ? 1 : 0) + ' 0 ' + p(r0, a0 + hi)) + ' Z'; };
const g = el(svg, 'g', {}), u = Math.max(0.6, Math.min(B.w, B.h) / 500), small = fs * 0.9;
const pctOf = (v) => (v / total) * 100, numTxt = (v) => (show === 'value' ? fmt(v) : show === 'both' ? fmt(v) + ' (' + Math.round(pctOf(v)) + '%)' : show === 'none' ? '' : (pctOf(v) >= 10 || Math.abs(pctOf(v) - Math.round(pctOf(v))) < 0.05 ? Math.round(pctOf(v)) : r1(pctOf(v))) + '%');
// unit bounding box of the arc
let ex = [1e9, -1e9, 1e9, -1e9]; const grow = (p) => { ex = [Math.min(ex[0], p[0]), Math.max(ex[1], p[0]), Math.min(ex[2], p[1]), Math.max(ex[3], p[1])]; };
for (let a = start; a <= start + sweep + 0.01; a += 1) grow(pt(0, 0, 1, a)); grow(pt(0, 0, 1, start + sweep)); grow(pt(0, 0, inner, start)); grow(pt(0, 0, inner, start + sweep)); if (inner < 0.02) grow([0, 0]);
let ax = B.x, ay = B.y, aw = B.w, ah = B.h, mx = 0, my = 0;
const items = vals.map((v, i) => ({ label: (labels[i] || '') + (lp === 'legend' && show !== 'none' ? '  ' + numTxt(v) : ''), color: col(i) }));
if (lp === 'legend') { if (B.w > B.h * 1.35) { const lw = Math.max.apply(null, items.map((it) => tw(it.label, fs))) + fs * 2.4; aw -= lw; const x = ax + aw + fs * 0.9, lh = fs * 1.75, y = B.y + B.h / 2 - (n * lh) / 2; items.forEach((it, i) => { el(g, 'rect', { x: r2(x), y: r2(y + i * lh + lh / 2 - fs * 0.4), width: r2(fs * 0.8), height: r2(fs * 0.8), rx: r2(fs * 0.16), fill: it.color }); T(g, x + fs * 1.25, y + i * lh + lh / 2 + mid(fs), it.label, { fill: muted }); }); } else { const lh = Math.ceil(items.reduce((a, it) => a + fs * 2.6 + tw(it.label, fs), 0) / B.w) * fs * 1.7; legend(g, items, B.x + B.w / 2, B.y + B.h - lh + fs * 0.3, B.w, fs); ah -= lh + fs * 0.6; } }
if (lp === 'out') { mx = Math.min(B.w * 0.3, Math.max.apply(null, vals.map((v, i) => Math.max(tw(labels[i] || '', small), tw(numTxt(v), fs * 1.1, 1)))) + fs * 1.5); my = fs * (show === 'none' ? 1.4 : 2.5); }
const fixed = P.r !== undefined && P.r !== '', R = fixed ? (W * num('r', 30)) / 100 : Math.max(10, Math.min((aw - 2 * mx) / (ex[1] - ex[0]), (ah - 2 * my) / (ex[3] - ex[2]))), r0 = R * inner;
const cx = fixed ? (W * num('cx', 50)) / 100 : ax + aw / 2 - (R * (ex[0] + ex[1])) / 2, cy = fixed ? (H * num('cy', 50)) / 100 : ay + ah / 2 - (R * (ex[2] + ex[3])) / 2;
if (ring) {
  const c = col(0), th = R - r0, rm = (R + r0) / 2, cap = on('round', 1) ? 'round' : 'butt', capA = cap === 'round' && sweep < 360 ? ((th / 2 / rm) * 180) / Math.PI : 0;
  const arc = (a0, a1) => { if (a1 - a0 >= 359.99) a1 = a0 + 359.99; const p0 = pt(cx, cy, rm, a0), p1 = pt(cx, cy, rm, a1); return 'M ' + r2(p0[0]) + ' ' + r2(p0[1]) + ' A ' + r2(rm) + ' ' + r2(rm) + ' 0 ' + (a1 - a0 > 180 ? 1 : 0) + ' 1 ' + r2(p1[0]) + ' ' + r2(p1[1]); };
  const s0 = start + capA, s1 = start + sweep - capA, f = Math.max(0, Math.min(1, vals[0] / total));
  el(g, 'path', { d: arc(s0, s1), fill: 'none', stroke: str('trackcolor', mix(c, '#ffffff', 0.82)), 'stroke-width': r2(th), 'stroke-linecap': cap });
  if (f > 0.001) el(g, 'path', { d: arc(s0, s0 + (s1 - s0) * f), fill: 'none', stroke: c, 'stroke-width': r2(th), 'stroke-linecap': cap });
  A('end', 'value end', pt(cx, cy, rm, s0 + (s1 - s0) * f)[0], pt(cx, cy, rm, s0 + (s1 - s0) * f)[1], { value: vals[0] });
} else if (nticks > 0) {
  let acc = 0; const bounds = vals.map((v) => (acc += v / total)), th = Math.max(1.5 * u, ((2 * Math.PI * (R + r0)) / 2) * (sweep / 360) / nticks * 0.55);
  for (let k = 0; k < nticks; k++) { const f = (k + 0.5) / nticks, a = start + sweep * f, i = bounds.findIndex((b) => f <= b + 1e-9), p0 = pt(cx, cy, r0, a), p1 = pt(cx, cy, R, a); el(g, 'line', { x1: r2(p0[0]), y1: r2(p0[1]), x2: r2(p1[0]), y2: r2(p1[1]), stroke: col(i < 0 ? n - 1 : i), 'stroke-width': r2(th) }); }
}
if (multi) { const mx_ = num('max', 100), sw_ = Math.min(sweep, 300), band = (R - r0) / Math.max(1, n), th = band * 0.72, st = P.start === undefined || P.start === '' ? 0 : start;
  vals.forEach((v, i) => { const rm = R - band * (i + 0.5), c = col(i), f = Math.max(0, Math.min(1, v / mx_)), arc = (a0, a1) => { const p0 = pt(cx, cy, rm, a0), p1 = pt(cx, cy, rm, Math.min(a1, a0 + 359.9)); return 'M ' + r2(p0[0]) + ' ' + r2(p0[1]) + ' A ' + r2(rm) + ' ' + r2(rm) + ' 0 ' + (a1 - a0 > 180 ? 1 : 0) + ' 1 ' + r2(p1[0]) + ' ' + r2(p1[1]); };
    el(g, 'path', { d: arc(st, st + sw_), fill: 'none', stroke: str('trackcolor', mix(c, '#ffffff', 0.84)), 'stroke-width': r2(th), 'stroke-linecap': 'round' }); if (f > 0.002) el(g, 'path', { d: arc(st, st + sw_ * f), fill: 'none', stroke: c, 'stroke-width': r2(th), 'stroke-linecap': 'round' });
    const ls = Math.min(fs, th * 0.95), ly = pt(cx, cy, rm, st)[1] + mid(ls), lx = pt(cx, cy, rm, st)[0] - th * 0.5 - ls * 0.5; T(g, lx, ly, (labels[i] ? labels[i] + '  ' : '') + fmt(v) + (suf || mx_ !== 100 ? '' : '%'), { size: ls, anchor: 'end', weight: 600, fill: ink }); const pe = pt(cx, cy, rm, st + sw_ * f); A('ring' + (i + 1), labels[i] || '', pe[0], pe[1], { value: v }); }); }
let a = start; const segs = [];
vals.forEach((v, i) => { const da = (v / total) * sweep, a0 = a, a1 = a + da; a = a1; if (ring || multi) return; const m = (a0 + a1) / 2, c = col(i);
  const Ri = radii.length ? r0 + (R - r0) * Math.max(0.05, Math.min(1, radii[i] === undefined ? 1 : radii[i])) : R;
  if (!nticks && !multi && da > 0.01) { const h = Math.min(gapA / 2, da / 2 - 0.05), cr = Math.min(corner, (Ri - r0) / 2.2);
    if (gappx > 0 || cr > 0) { const d = sectorG(cx, cy, r0 + cr, Ri - cr, a0 + h, a1 - h, gappx + 2 * cr); if (d) el(g, 'path', Object.assign({ d, fill: c }, cr > 0 ? { stroke: c, 'stroke-width': r2(2 * cr), 'stroke-linejoin': 'round' } : {})); }
    else el(g, 'path', Object.assign({ d: sector(cx, cy, r0, Ri, a0 + h, a1 - h), fill: c }, sep !== 'none' && !gapA && n > 1 ? { stroke: sep, 'stroke-width': r2(2 * u), 'stroke-linejoin': 'round' } : {})); }
  const pm = pt(cx, cy, inner < 0.02 ? Ri * 0.62 : (Ri + r0) / 2, m); A('slice' + (i + 1), labels[i] || '', pm[0], pm[1], { value: v, pct: r1(pctOf(v)), angle: r1(m), outer: Math.round(Ri) }); segs.push({ i, v, m, da, c, pm, Ri }); });
if (!ring && !nticks && (lp === 'in' || lp === 'legend') && show !== 'none') for (const s of segs) { const t = numTxt(s.v), arcLen = ((s.da * Math.PI) / 180) * (inner < 0.02 ? R * 0.62 : (R + r0) / 2); if (pctOf(s.v) >= minl && arcLen > tw(t, fs, 1) * 0.9 && (inner < 0.02 || R - r0 > tw(t, fs, 1) * 0.8)) T(g, s.pm[0], s.pm[1] + mid(fs), t, { weight: numwt, font: numfont, anchor: 'middle', fill: onc(s.c) }); }
if (lp === 'out') {
  const sides = { 1: [], '-1': [] }; for (const s of segs) { const p = pt(cx, cy, R, s.m), side = p[0] >= cx - 0.5 ? 1 : -1; sides[side].push(Object.assign(s, { p, side, y: pt(cx, cy, R * 1.14, s.m)[1] })); }
  const lh = fs * (show === 'none' ? 1.3 : 2.45);
  for (const k of [1, -1]) { const L = sides[k].sort((p, q) => p.y - q.y); for (let j = 1; j < L.length; j++) if (L[j].y - L[j - 1].y < lh) L[j].y = L[j - 1].y + lh; const over = L.length ? L[L.length - 1].y + lh / 2 - (B.y + B.h) : 0; if (over > 0) for (let j = L.length - 1; j >= 0; j--) { L[j].y -= over; if (j > 0 && L[j].y - L[j - 1].y >= lh) break; }
    for (const s of L) { const wmax = Math.max(tw(labels[s.i] || '', small), show === 'none' ? 0 : tw(numTxt(s.v), fs * 1.1, 1)), p1 = pt(cx, cy, (s.Ri || R) + R * 0.03, s.m), p2 = [pt(cx, cy, R * 1.12, s.m)[0], s.y];
      let x3 = p2[0] + k * fs * 0.9; x3 = k > 0 ? Math.min(x3, B.x + B.w - wmax - fs * 0.4) : Math.max(x3, B.x + wmax + fs * 0.4);
      el(g, 'polyline', { points: [p1, p2, [x3, s.y]].map((q) => r2(q[0]) + ',' + r2(q[1])).join(' '), fill: 'none', stroke: muted, 'stroke-width': r2(1.2 * u) });
      const tx = x3 + k * fs * 0.35, an = k > 0 ? 'start' : 'end';
      if (show === 'none') T(g, tx, s.y + mid(fs), labels[s.i] || '', { anchor: an, fill: muted });
      else { T(g, tx, s.y - fs * 0.08, numTxt(s.v), { size: fs * 1.1, weight: numwt, font: numfont, anchor: an, fill: ink }); T(g, tx, s.y + small * 1.08, labels[s.i] || '', { size: small, anchor: an, fill: muted }); } } }
}
const ctr = str('center', ring ? fmt(vals[0]) + (suf || total !== 100 ? '' : '%') : ''), sub = str('sub', '');
if (ctr || sub) { const half = sweep <= 200 && inner >= 0.02, room = (inner < 0.02 ? R : r0) * 1.62, cs = Math.min(num('centersize', (inner < 0.02 ? R : r0) * (half ? 0.4 : 0.52)), ctr ? (room / Math.max(1, tw(ctr, 1, 1))) : 1e9), ss = Math.min(fs, r0 * 0.24 || fs);
  const yc = half ? cy - (sub ? ss * 1.5 : 0) - cs * 0.02 : cy + (sub ? -ss * 0.55 : 0) + cs * 0.36;
  if (ctr) T(g, cx, yc, ctr, { size: cs, weight: numwt, font: numfont, anchor: 'middle' });
  if (sub) wrap(sub, room, ss).slice(0, 2).forEach((ln, k) => T(g, cx, (half ? cy - ss * 0.2 : yc + ss * 1.45) + k * ss * 1.2 - (half ? (wrap(sub, room, ss).slice(0, 2).length - 1 - k) * ss * 1.2 : 0), ln, { size: ss, anchor: 'middle', fill: muted })); }
A('center', 'center', cx, cy);
done('donut', { center: [Math.round(cx), Math.round(cy)], outer: Math.round(R), inner: Math.round(r0) });
