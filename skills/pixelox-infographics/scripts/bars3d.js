// bars3d.js: isometric columns: each value is a block with a lit top and two shaded sides, standing on a floor.
// The view is parallel (no perspective), so the block HEIGHTS stay proportional to the values and every value
// is printed. A look for hero charts and dark dashboards, not a more exact chart: when precision matters use bars.js.
// Run:   pixelox render js scripts/bars3d.js -o blocks.png --size 1080x1350 --param box=8,26,84,50 --param values=52,80,31,60 --param labels='North,South,East,West' --param suffix=%
// Place: pixelox comp place d.pxo blocks.png --in-place --as Blocks --at 0,0
// Data params:
//   values=52,80,31,60       one number per block (zero or more)
//   labels=A,B,C,D           names under the blocks
// Form params (all optional):
//   shape=box                box (square prism) | cylinder
//   depth=1                  block depth as a multiple of its width (0.6 = slabs, 1 = square)
//   gap=0.7                  space between blocks as a fraction of a block's width
//   max=0                    the value a full-height block stands for (0 = the largest value)
//   floor=#e2e6ea            floor color (floor=none: no floor)
//   vlabels=1                values above the blocks
//   light=0.22 dark=0.28     how much the top is lightened and the right side darkened
// One color for all blocks reads calmest; with colors= each block takes its own.
// Shared params (every chart script): box=x,y,w,h in % of the image (default 0,0,100,100); colors=#hex,#hex;
//   ink=#1f2933 (text) muted=#6b7785 (axis text) grid=#d5dbe1 bg=transparent; fs=label size in px;
//   font='Family, Fallback, sans-serif' (a family name: font classes such as @condensed do not work inside a script)
//   numfont= weight=400 numweight=700; prefix=$ suffix=% decimals=1 compact=1 (1.2K, 3.4M)
// The result's logs hold one JSON line with an anchor at the top center and the base center of every block.
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
const vals = (rows('values', '52,80,31,60')[0] || []).map((v) => Math.max(0, v)), labels = list('labels'), n = vals.length, cyl = /^c/i.test(str('shape', 'box'));
const depth = num('depth', 1), gap = num('gap', 0.7), vmax = num('max', 0) > 0 ? num('max', 0) : vmaxOf(vals), floorc = str('floor', '#e2e6ea'), showV = on('vlabels', 1), lt = num('light', 0.22), dk = num('dark', 0.28);
const g = el(svg, 'g', {}), small = fs * 0.9, C30 = Math.cos(Math.PI / 6), S30 = 0.5;
// unit space: blocks of width 1 along x, depth 'depth' along z, height up to hmax; iso: sx = (x - z) * C30, sy = (x + z) * S30 - y
const span = n + (n - 1) * gap, pad = 0.35, top = showV ? fs * 1.7 : fs * 0.4, bot = labels.length ? small * 2.1 : fs * 0.4;
const isoW = (span + 2 * pad + depth + 2 * pad) * C30, baseH = (span + 2 * pad + depth + 2 * pad) * S30;
const u = Math.min(B.w / isoW, (B.h - top - bot) / (baseH + Math.max(1.2, span * 0.55))), hmax = (B.h - top - bot - baseH * u) / u;
const ox = B.x + (B.w - isoW * u) / 2 + (depth + 2 * pad) * C30 * u, oy = B.y + top + hmax * u;
const Pj = (x, y, z) => [ox + (x - z) * C30 * u, oy + (x + z) * S30 * u - y * u], poly = (pts, fill) => el(g, 'polygon', { points: pts.map((p) => r2(p[0]) + ',' + r2(p[1])).join(' '), fill, stroke: fill, 'stroke-width': 0.6, 'stroke-linejoin': 'round' });
if (floorc !== 'none') poly([Pj(-pad, 0, -pad), Pj(span + pad, 0, -pad), Pj(span + pad, 0, depth + pad), Pj(-pad, 0, depth + pad)], floorc);
for (let i = 0; i < n; i++) { const x0 = i * (1 + gap), x1 = x0 + 1, h = (vals[i] / vmax) * hmax, c = COLORS.length > 1 ? col(i) : col(0), cT = mix(c, '#ffffff', lt), cR = mix(c, '#000000', dk);
  if (cyl) { const cb = Pj(x0 + 0.5, 0, depth / 2), ct = Pj(x0 + 0.5, h, depth / 2), rx = 0.5 * 1.2247 * u * Math.min(1, (depth + 1) / 2), ry = rx * 0.5774, gid = 'cy' + i, gr = el(defs, 'linearGradient', { id: gid, x1: 0, y1: 0, x2: 1, y2: 0 }); el(gr, 'stop', { offset: 0, 'stop-color': mix(c, '#ffffff', lt * 0.4) }); el(gr, 'stop', { offset: 0.45, 'stop-color': c }); el(gr, 'stop', { offset: 1, 'stop-color': cR });
    el(g, 'path', { d: 'M ' + r2(cb[0] - rx) + ' ' + r2(ct[1]) + ' L ' + r2(cb[0] - rx) + ' ' + r2(cb[1]) + ' A ' + r2(rx) + ' ' + r2(ry) + ' 0 0 0 ' + r2(cb[0] + rx) + ' ' + r2(cb[1]) + ' L ' + r2(cb[0] + rx) + ' ' + r2(ct[1]) + ' Z', fill: 'url(#' + gid + ')' }); el(g, 'ellipse', { cx: r2(ct[0]), cy: r2(ct[1]), rx: r2(rx), ry: r2(ry), fill: cT }); }
  else { poly([Pj(x0, 0, depth), Pj(x1, 0, depth), Pj(x1, h, depth), Pj(x0, h, depth)], c); poly([Pj(x1, 0, 0), Pj(x1, 0, depth), Pj(x1, h, depth), Pj(x1, h, 0)], cR); poly([Pj(x0, h, 0), Pj(x1, h, 0), Pj(x1, h, depth), Pj(x0, h, depth)], cT); }
  const tp = Pj(x0 + 0.5, h, depth / 2), bp = Pj(x0 + 0.5, 0, depth / 2), fp = Pj(x0 + 0.5, 0, depth + pad), topY = cyl ? tp[1] - 0.36 * u * Math.min(1, (depth + 1) / 2) : Pj(x0, h, 0)[1];
  if (showV) T(g, tp[0], topY - fs * 0.45, fmt(vals[i]), { weight: numwt, font: numfont, anchor: 'middle' });
  if (labels[i]) T(g, fp[0] - 0.25 * u * C30, fp[1] + small * 1.25, labels[i], { size: small, fill: muted, anchor: 'middle' });
  A('block' + (i + 1), labels[i] || '', tp[0], tp[1], { value: vals[i], bx: Math.round(bp[0]), by: Math.round(bp[1]) }); }
done('bars3d', { unit: r1(u), max: vmax });
