// cycle.js: a cycle or loop diagram: N steps placed evenly around a ring with arrows between them and their
// labels outside, or a ring cut into N arrow segments. Also a hub with N spokes. Output is a transparent image.
// Run:   pixelox render js scripts/cycle.js -o cycle.png --size 1080x1350 --param box=6,28,88,50 --param labels='Plan|Build|Measure|Learn' --param notes='Set one goal|Make the smallest thing|Count what happened|Decide what changes' --param center='Loop'
// Place: pixelox comp place d.pxo cycle.png --in-place --as Cycle --at 0,0
// Data params:
//   labels=A|B|C|D           step titles, clockwise from the top (3 to 10 work well)
//   notes=...|...            one short note per step (wrapped automatically; '\n' forces a line break)
//   badges=1,2,3             text inside each node (default 1, 2, ...; badges=none leaves them empty for icons)
// Form params (all optional):
//   style=nodes              nodes (circles on a ring, arrows between) | segments (the ring itself is cut into
//                            arrow-shaped arcs) | spokes (nodes joined to the center, no order implied)
//   start=0                  angle of the first step, degrees clockwise from 12 o'clock
//   node=0                   node diameter in px (segments: ring thickness); 0 = fitted
//   center= sub=             big and small text in the middle
//   line=#c3cad2             ring and arrow color;  nodefill=  set a color to draw outlined (ring) nodes
//   labelwidth=0.22          width kept for the labels on each side, as a fraction of the box width
//   titlesize=0              title size in px (0 = 1.15 x fs)
//   onecolor=0               1 = every node takes the first color
// Shared params (every chart script): box=x,y,w,h in % of the image (default 0,0,100,100); colors=#hex,#hex;
//   ink=#1f2933 (text) muted=#6b7785 (axis text) grid=#d5dbe1 bg=transparent; fs=label size in px;
//   font='Family, Fallback, sans-serif' (a family name: font classes such as @condensed do not work inside a script)
//   numfont= weight=400 numweight=700; prefix=$ suffix=% decimals=1 compact=1 (1.2K, 3.4M)
// The result's logs hold one JSON line: the center, the ring radius, and an anchor (canvas px and %) at every node.
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
const labels = list('labels', str('notes', '') ? '' : 'Plan|Build|Measure|Learn'), notes = list('notes'), n = Math.max(labels.length, notes.length, 2);
const bp = str('badges', ''), badges = /^(none|0|off)$/i.test(bp) ? [] : bp ? split(bp) : Array.from({ length: n }, (_, i) => String(i + 1));
const style = str('style', 'nodes').toLowerCase(), start = num('start', 0), linec = str('line', '#c3cad2'), nodefill = str('nodefill', ''), ts = num('titlesize', 0) || fs * 1.15, one = on('onecolor', 0), C = (i) => (one ? col(0) : col(i));
const g = el(svg, 'g', {}), u = Math.max(0.6, Math.min(B.w, B.h) / 500), lw = B.w * num('labelwidth', 0.22), hasText = labels.length + notes.length > 0;
const measure = (i, maxw) => { const tl = labels[i] ? wrap(labels[i], maxw, ts, 1) : [], nl = []; if (notes[i]) for (const para of notes[i].split('\\n')) for (const ln of wrap(para, maxw, fs)) nl.push(ln); return { tl, nl, h: tl.length * ts * 1.18 + nl.length * fs * 1.3 + (tl.length && nl.length ? fs * 0.3 : 0) }; };
const block = (x, y, anchor, m) => { let yy = y; for (const ln of m.tl) { T(g, x, yy + ts * 0.82, ln, { size: ts, weight: 700, anchor }); yy += ts * 1.18; } if (m.tl.length && m.nl.length) yy += fs * 0.3; for (const ln of m.nl) { T(g, x, yy + fs * 0.82, ln, { anchor, fill: muted }); yy += fs * 1.3; } };
const ms = Array.from({ length: n }, (_, i) => measure(i, lw - fs)), mh = hasText ? Math.max.apply(null, ms.map((m) => m.h)) : 0, pad = fs * 0.8;
const ang = (i) => start + (360 / n) * i, sn = (a) => Math.sin((a * Math.PI) / 180), cs_ = (a) => Math.cos((a * Math.PI) / 180), isTB = (a) => Math.abs(sn(a)) < 0.12;
const AN = Array.from({ length: n }, (_, i) => ang(i)), mx = hasText ? lw : 0, myT = hasText ? (AN.some((a) => isTB(a) && cs_(a) > 0) ? mh + pad : mh * 0.5) : 0, myB = hasText ? (AN.some((a) => isTB(a) && cs_(a) < 0) ? mh + pad : mh * 0.5) : 0;
const Rout = Math.max(20, Math.min((B.w - 2 * mx) / 2, (B.h - myT - myB) / 2)), cx = B.x + B.w / 2, cy = B.y + myT + (B.h - myT - myB) / 2;
const seg = style === 'segments', D = num('node', 0) || (seg ? Rout * 0.34 : Math.min(Rout * 0.42, (2 * Math.PI * Rout) / n * 0.5)), R = seg ? Rout - D / 2 : Rout - D / 2;
if (seg) {
  const gapA = 4, head = Math.min(9, (360 / n) * 0.14);
  for (let i = 0; i < n; i++) { const a0 = ang(i) - 180 / n + gapA / 2, a1 = ang(i) + 180 / n - gapA / 2, r0 = R - D / 2, r1 = R + D / 2, P0 = (r, a) => pt(cx, cy, r, a).map(r2).join(' ');
    const d = 'M ' + P0(r1, a0 - head) + ' A ' + r2(r1) + ' ' + r2(r1) + ' 0 0 1 ' + P0(r1, a1 - head) + ' L ' + P0(R, a1) + ' L ' + P0(r0, a1 - head) + ' A ' + r2(r0) + ' ' + r2(r0) + ' 0 0 0 ' + P0(r0, a0 - head) + ' L ' + P0(R, a0) + ' Z';
    el(g, 'path', { d, fill: C(i) }); const p = pt(cx, cy, R, ang(i) - head * 0.3), b = badges[i] || '', bs = Math.min(D * 0.46, fs * 2.2);
    if (b) T(g, p[0], p[1] + mid(bs), b, { size: bs, weight: numwt, font: numfont, anchor: 'middle', fill: onc(C(i)) }); A('node' + (i + 1), labels[i] || '', p[0], p[1], { d: Math.round(D) }); }
} else {
  if (style === 'spokes') for (let i = 0; i < n; i++) { const p = pt(cx, cy, R, ang(i)), q0 = pt(cx, cy, str('center', '') || str('sub', '') ? R * 0.42 : 0, ang(i)); el(g, 'line', { x1: r2(q0[0]), y1: r2(q0[1]), x2: r2(p[0]), y2: r2(p[1]), stroke: linec, 'stroke-width': r2(2.6 * u) }); }
  else for (let i = 0; i < n; i++) { const da = ((D / 2 + 7 * u) / R) * (180 / Math.PI), a0 = ang(i) + da, a1 = ang(i + 1) - da - (9 * u / R) * (180 / Math.PI); if (a1 <= a0) continue; const p0 = pt(cx, cy, R, a0), p1 = pt(cx, cy, R, a1);
    el(g, 'path', { d: 'M ' + r2(p0[0]) + ' ' + r2(p0[1]) + ' A ' + r2(R) + ' ' + r2(R) + ' 0 ' + (a1 - a0 > 180 ? 1 : 0) + ' 1 ' + r2(p1[0]) + ' ' + r2(p1[1]), fill: 'none', stroke: linec, 'stroke-width': r2(2.6 * u), 'stroke-linecap': 'round' });
    const t = ((a1 - 90 + 90) * Math.PI) / 180, s = 9 * u, tip = pt(cx, cy, R, a1 + (s / R) * (180 / Math.PI)); el(g, 'polygon', { points: [[s, 0], [-s * 0.7, -s * 0.85], [-s * 0.7, s * 0.85]].map((q) => r2(tip[0] - s * Math.cos(t) + q[0] * Math.cos(t) - q[1] * Math.sin(t)) + ',' + r2(tip[1] - s * Math.sin(t) + q[0] * Math.sin(t) + q[1] * Math.cos(t))).join(' '), fill: linec }); }
  for (let i = 0; i < n; i++) { const p = pt(cx, cy, R, ang(i)), c = C(i), b = badges[i] || '';
    if (nodefill) el(g, 'circle', { cx: r2(p[0]), cy: r2(p[1]), r: r2(D / 2 - D * 0.045), fill: nodefill, stroke: c, 'stroke-width': r2(D * 0.09) }); else el(g, 'circle', { cx: r2(p[0]), cy: r2(p[1]), r: r2(D / 2), fill: c });
    if (b) { const bs = Math.min(D * 0.46, (D * 0.74) / Math.max(1, tw(b, 1, 1))); T(g, p[0], p[1] + mid(bs), b, { size: bs, weight: numwt, font: numfont, anchor: 'middle', fill: nodefill ? c : onc(c) }); }
    A('node' + (i + 1), labels[i] || '', p[0], p[1], { d: Math.round(D) }); }
}
if (hasText) for (let i = 0; i < n; i++) { const a = ((ang(i) % 360) + 360) % 360, m = ms[i], p = pt(cx, cy, Rout + pad, a), y = p[1] - m.h * ((1 + cs_(a)) / 2);
  block(p[0], Math.max(B.y, Math.min(B.y + B.h - m.h, y)), isTB(a) ? 'middle' : sn(a) > 0 ? 'start' : 'end', m); }
const ctr = str('center', ''), sub = str('sub', ''), room = (R - D / 2) * (style === 'spokes' ? 0.8 : 1.5);
if (ctr || sub) { const cs = Math.min(num('centersize', room * 0.3), ctr ? room / Math.max(1, tw(ctr, 1, 1)) : 1e9), ss = Math.min(fs, room * 0.12), sl = sub ? wrap(sub, room, ss) : [];
  const y0 = cy - (sl.length * ss * 1.25) / 2 + (ctr ? cs * 0.36 : ss * 0.4); if (ctr) T(g, cx, y0, ctr, { size: cs, weight: numwt, font: numfont, anchor: 'middle' }); sl.forEach((ln, k) => T(g, cx, y0 + (ctr ? ss * 1.5 : 0) + k * ss * 1.25, ln, { size: ss, anchor: 'middle', fill: muted })); }
A('center', 'center', cx, cy);
done('cycle', { center: [Math.round(cx), Math.round(cy)], radius: Math.round(R), node: Math.round(D) });
