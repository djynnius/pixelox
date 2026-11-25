// radial.js: bars that radiate from a hub (a radial bar chart), or a ring of labeled spokes without values (a
// radial list or timeline). Bar LENGTHS are proportional to the values, measured from the hub's edge.
// Run:   pixelox render js scripts/radial.js -o radial.png --size 1080x1350 --param box=6,26,88,56 --param values=62,48,80,35,55,71,44,90 --param labels='Jan|Feb|Mar|Apr|May|Jun|Jul|Aug' --param center=485 --param sub=total
// Place: pixelox comp place d.pxo radial.png --in-place --as Radial --at 0,0
// Data params:
//   values=62,48,80          one number per bar, clockwise from the top (omit for a ring of labels only)
//   labels=A|B|C             one name per bar or spoke
// Form params (all optional):
//   inner=0.36               hub radius as a fraction of the outer radius
//   sweep=360 start=0        degrees covered; angle of the first bar, clockwise from 12 o'clock
//   rotate=auto              1 = names written along the spokes (many items, long names); 0 = upright names
//                            outside the bar ends; auto = along the spokes above 12 items
//   barwidth=0.62            bar thickness as a fraction of its slot (values form)
//   max=0 rings=0            the value a full-length bar stands for; number of faint guide rings
//   vlabels=1                the value at each bar's end
//   track=0                  1 = every bar lies on a pale full-length track
//   center= sub=             big and small text in the hub;  hub=  a color fills the hub disc
//   cx= cy= r=               place by center instead of box (% of the width, % of the height, outer radius in % of the width)
// One color for all bars is the default; colors= gives one per bar; highlight=N accent=#hex marks one.
// Shared params (every chart script): box=x,y,w,h in % of the image (default 0,0,100,100); colors=#hex,#hex;
//   ink=#1f2933 (text) muted=#6b7785 (axis text) grid=#d5dbe1 bg=transparent; fs=label size in px;
//   font='Family, Fallback, sans-serif' (a family name: font classes such as @condensed do not work inside a script)
//   numfont= weight=400 numweight=700; prefix=$ suffix=% decimals=1 compact=1 (1.2K, 3.4M)
// The result's logs hold one JSON line: the center, the radii, and an anchor at every bar's end and at the hub.
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
const vals = (rows('values', '')[0] || []).map((v) => Math.max(0, v)), labels = list('labels'), hasV = vals.length > 0, n = Math.max(vals.length, labels.length, 1);
const sweep = Math.min(360, num('sweep', 360)), start = num('start', 0), inner = Math.max(0.1, Math.min(0.9, num('inner', hasV ? 0.36 : 0.62))), rp = str('rotate', 'auto').toLowerCase(), rot = rp === 'auto' ? n > 12 : on('rotate', 0);
const bwid = num('barwidth', 0.62), vmax = num('max', 0) > 0 ? num('max', 0) : vmaxOf(vals), nr = Math.round(num('rings', 0)), showV = hasV && on('vlabels', 1), track = on('track', 0), hi = Math.round(num('highlight', 0)), accent = str('accent', '#f97316');
const g = el(svg, 'g', {}), small = fs * 0.9, slot = sweep / (sweep >= 360 ? n : Math.max(1, n - 1 + 1)), ang = (i) => start + slot * (i + (sweep >= 360 ? 0 : 0.5)), C = (i) => (hi ? (i + 1 === hi ? accent : col(0)) : COLORS.length > 1 ? col(i) : col(0));
const labLen = labels.reduce((a, s) => Math.max(a, tw(s, small)), 0), vLen = showV ? Math.max.apply(null, vals.map((v) => tw(fmt(v), small, 1))) + small * 0.5 : 0;
const fixed = P.r !== undefined && P.r !== '', margin = rot ? labLen + vLen + small * 0.9 : 0, mx = rot ? 0 : labLen + small * 0.9, my = rot ? 0 : small * 1.6;
const R = fixed ? (W * num('r', 30)) / 100 : Math.max(20, rot ? Math.min(B.w, B.h) / 2 - margin : Math.min((B.w - 2 * mx) / 2, (B.h - 2 * my) / 2) - (showV ? vLen : 0)), r0 = R * inner;
const cx = fixed ? (W * num('cx', 50)) / 100 : B.x + B.w / 2, cy = fixed ? (H * num('cy', 50)) / 100 : B.y + B.h / 2, u = Math.max(0.6, R / 250);
for (let k = 1; k <= nr; k++) el(g, 'circle', { cx: r2(cx), cy: r2(cy), r: r2(r0 + ((R - r0) * k) / nr), fill: 'none', stroke: gridc, 'stroke-width': r2(u) });
if (P.hub) el(g, 'circle', { cx: r2(cx), cy: r2(cy), r: r2(r0 - 4 * u), fill: str('hub', '#ffffff') });
for (let i = 0; i < n; i++) { const a = ang(i), c = C(i), half = (slot * bwid) / 2, len = hasV ? ((vals[i] || 0) / vmax) * (R - r0) : 0, re = r0 + len, aa = ((a % 360) + 360) % 360, right = aa <= 180;
  if (hasV) { if (track) el(g, 'path', { d: sector(cx, cy, r0, R, a - half, a + half), fill: str('trackcolor', mix(c, '#ffffff', 0.84)) }); if (len > 0.5) el(g, 'path', { d: sector(cx, cy, r0, re, a - half, a + half), fill: c }); }
  else { const p0 = pt(cx, cy, r0, a), p1 = pt(cx, cy, r0 + (R - r0) * 0.5, a); el(g, 'line', { x1: r2(p0[0]), y1: r2(p0[1]), x2: r2(p1[0]), y2: r2(p1[1]), stroke: c, 'stroke-width': r2(2.2 * u), 'stroke-linecap': 'round' }); el(g, 'circle', { cx: r2(p1[0]), cy: r2(p1[1]), r: r2(4.5 * u), fill: c }); }
  const endR = hasV ? (track ? R : re) : r0 + (R - r0) * 0.5 + 6 * u, pe = pt(cx, cy, hasV ? re : endR, a); A('bar' + (i + 1), labels[i] || '', pe[0], pe[1], hasV ? { value: vals[i], angle: r1(a) } : { angle: r1(a) });
  if (rot) { const p = pt(cx, cy, endR + small * 0.5, a), tr = right ? aa - 90 : aa + 90, vt = showV ? fmt(vals[i]) : '', gap2 = vt ? tw(vt, small, 1) + small * 0.45 : 0;
    if (vt) T(g, p[0], p[1], vt, { size: small, weight: numwt, font: numfont, anchor: right ? 'start' : 'end', rotate: r1(tr), fill: hi && i + 1 === hi ? accent : ink });
    if (labels[i]) { const q = pt(cx, cy, endR + small * 0.5 + gap2, a); T(g, q[0], q[1], labels[i], { size: small, anchor: right ? 'start' : 'end', rotate: r1(tr), fill: muted }); } }
  else { const tb = Math.abs(Math.sin((aa * Math.PI) / 180)) < 0.2, an = tb ? 'middle' : right ? 'start' : 'end', p = pt(cx, cy, endR + small * 0.7, a), up = Math.cos((aa * Math.PI) / 180), two = showV && labels[i], y = p[1] + mid(small) + (tb ? (up > 0 ? -small * (two ? 0.9 : 0.2) : small * 0.5) : 0);
    if (showV) T(g, p[0], y - (two ? small * 0.55 : 0), fmt(vals[i]), { size: small, weight: numwt, font: numfont, anchor: an, fill: hi && i + 1 === hi ? accent : ink });
    if (labels[i]) T(g, p[0], y + (two ? small * 0.6 : 0), labels[i], { size: small, anchor: an, fill: muted }); } }
const ctr = str('center', ''), sub = str('sub', ''), room = r0 * 1.55;
if (ctr || sub) { const cs = Math.min(num('centersize', r0 * 0.55), ctr ? room / Math.max(1, tw(ctr, 1, 1)) : 1e9), ss = Math.min(fs, r0 * 0.22), sl = sub ? wrap(sub, room, ss).slice(0, 3) : [], y0 = cy - (sl.length * ss * 1.25) / 2 + (ctr ? cs * 0.36 : ss * 0.4);
  if (ctr) T(g, cx, y0, ctr, { size: cs, weight: numwt, font: numfont, anchor: 'middle' }); sl.forEach((ln, k) => T(g, cx, y0 + (ctr ? ss * 1.5 : 0) + k * ss * 1.25, ln, { size: ss, anchor: 'middle', fill: muted })); }
A('center', 'center', cx, cy);
done('radial', { center: [Math.round(cx), Math.round(cy)], outer: Math.round(R), inner: Math.round(r0) });
