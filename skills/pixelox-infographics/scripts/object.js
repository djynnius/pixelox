// object.js: a metaphor object drawn from shapes and cut into colored bands, one band per step or part: a rocket,
// a light bulb, a pencil, a thermometer, or a battery. With value= it becomes a picture gauge filled from the
// bottom. The script draws the object and the band numbers; put titles and notes beside it with text add, at
// the anchors it logs. Output is a transparent image.
// Run:   pixelox render js scripts/object.js -o rocket.png --size 1080x1080 --param box=34,8,32,84 --param kind=rocket --param bands=4 --param colors='#ff5a7a,#8b5cf6,#2dd4bf,#fbbf24'
// Place: pixelox comp place d.pxo rocket.png --in-place --as Rocket --at 0,0
// Params (all optional):
//   kind=rocket              rocket | bulb | pencil | thermometer | battery
//   bands=4                  number of colored bands, counted from the top (rocket: band 1 is the nose cone)
//   badges=01,02,03          text on each band (default 01, 02, ...; badges=none for plain bands)
//   value= max=100           gauge form: fill the object from the bottom to value/max in the first color;
//                            the rest takes the 'empty' color (bands and badges are then ignored)
//   empty=#e2e6ea            gauge form: color of the unfilled part
//   aspect=1                 width factor: 1.4 gives a stout rocket or a fat pencil, 0.8 a slim one
//   outline=0                1 = no bands: the body is drawn as an outline in the first color (glass= fills it):
//                            a line-art bulb or rocket to use as a hub or an icon
//                            (outlinewidth=0.055: line weight as a fraction of the object's unit; 0.075 is bold)
//   rotate=0                 degrees clockwise (rocket at 30 = climbing to the right; pencil at 90 = lying down)
//   shade=1                  a soft highlight down one side (0 = flat)
//   badgecolor=              color of the band numbers: one hex, or one per band (default: white or ink, by contrast)
//   flame=1 window=0         rocket: exhaust flame; a porthole on band 2 (windowsize=0.15 of the body height unit,
//                            windowring=#e5edf5, windowglass=#7cc4f2)
//   fincolor=                rocket: fin color (default: a darker shade of band 2)
//   rays=0                   bulb: short rays around the glass
//   dark=#374151             color of the dark parts (nozzle, bulb base, pencil lead, outline of the gauge forms)
// The object keeps its proportions and is fitted into box= (height first; a tall narrow box suits all kinds).
// Shared params (every chart script): box=x,y,w,h in % of the image (default 0,0,100,100); colors=#hex,#hex;
//   ink=#1f2933 (text) muted=#6b7785 (axis text) grid=#d5dbe1 bg=transparent; fs=label size in px;
//   font='Family, Fallback, sans-serif' (a family name: font classes such as @condensed do not work inside a script)
//   numfont= weight=400 numweight=700; prefix=$ suffix=% decimals=1 compact=1 (1.2K, 3.4M)
// The result's logs hold one JSON line with an anchor per band: its center (x, y) and the points on its left and
// right edges (lx, ly, rx, ry), in canvas px, for leader lines and labels.
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
const kind = str('kind', 'rocket').toLowerCase(), gauge = P.value !== undefined && P.value !== '', frac = Math.max(0, Math.min(1, num('value', 0) / (num('max', 100) || 1)));
const nb = gauge ? 1 : Math.max(1, Math.round(num('bands', 4))), bp = str('badges', ''), badges = gauge || /^(none|0|off)$/i.test(bp) ? [] : bp ? split(bp) : Array.from({ length: nb }, (_, i) => (i < 9 ? '0' : '') + (i + 1));
const BC = list('badgecolor'), rot = num('rotate', 0), shade = on('shade', 1), dark = str('dark', '#374151'), empty = str('empty', '#e2e6ea');
// unit geometry: x from -0.5 to 0.5, y from 0 (top) down; 'sil' is the area that takes the bands
const K = {
  rocket: { w: 1.04, h: 2.62, y0: 0, y1: 2.05, first: 0.78, sil: 'M 0 0 C 0.16 0.18 0.3 0.45 0.3 0.78 L 0.3 1.9 L 0.2 2.05 L -0.2 2.05 L -0.3 1.9 L -0.3 0.78 C -0.3 0.45 -0.16 0.18 0 0 Z', half: (y) => (y < 0.78 ? 0.3 * Math.sin((Math.max(0, y) / 0.78) * 1.5708) : y > 1.9 ? 0.3 - ((y - 1.9) / 0.15) * 0.1 : 0.3) },
  bulb: { w: 1.0, h: 1.5, y0: 0, y1: 1.08, sil: 'M -0.2 1.08 C -0.2 0.92 -0.5 0.8 -0.5 0.5 A 0.5 0.5 0 1 1 0.5 0.5 C 0.5 0.8 0.2 0.92 0.2 1.08 Z', half: (y) => (y <= 0.78 ? Math.sqrt(Math.max(0, 0.25 - (y - 0.5) * (y - 0.5))) : 0.42 - ((y - 0.78) / 0.3) * 0.22) },
  pencil: { w: 0.5, h: 3.2, y0: 0.5, y1: 2.5, sil: 'M -0.25 0.5 L 0.25 0.5 L 0.25 2.5 L -0.25 2.5 Z', half: () => 0.25 },
  thermometer: { w: 0.8, h: 3.0, y0: 0.12, y1: 2.9, sil: 'M -0.15 0.15 A 0.15 0.15 0 0 1 0.15 0.15 L 0.15 2.2 A 0.36 0.36 0 1 1 -0.15 2.2 Z', half: (y) => (y < 2.2 ? 0.15 : 0.36) },
  battery: { w: 1.0, h: 1.9, y0: 0.26, y1: 1.8, sil: 'M -0.4 0.26 L 0.4 0.26 L 0.4 1.8 L -0.4 1.8 Z', half: () => 0.4 },
}[kind] || null;
if (!K) throw new Error('kind must be rocket, bulb, pencil, thermometer, or battery');
const asp = Math.max(0.5, Math.min(2.5, num('aspect', 1))), outline = on('outline', 0); K.w *= asp; const half0 = K.half; K.half = (y) => half0(y) * asp;
const rad = (rot * Math.PI) / 180, cs = Math.abs(Math.cos(rad)), sn = Math.abs(Math.sin(rad)), bw = K.w * cs + K.h * sn, bh = K.w * sn + K.h * cs, s = Math.min(B.w / bw, B.h / bh), cx = B.x + B.w / 2, cy = B.y + B.h / 2;
// unit point -> canvas point
const X = (x, y) => { const px0 = x * s, py0 = (y - K.h / 2) * s; return [cx + px0 * Math.cos(rad) - py0 * Math.sin(rad), cy + px0 * Math.sin(rad) + py0 * Math.cos(rad)]; };
const g = el(svg, 'g', { transform: 'translate(' + r2(cx) + ' ' + r2(cy) + ') rotate(' + rot + ') scale(' + r2(s * asp) + ' ' + r2(s) + ') translate(0 ' + r2(-K.h / 2) + ')' });
const P_ = (d, fill, extra) => el(g, 'path', Object.assign({ d, fill }, extra || {}));
const c0 = col(0), lastc = col(nb - 1);
// parts behind the banded body
if (kind === 'rocket') { const fc = str('fincolor', mix(col(Math.min(1, nb - 1)), '#000000', 0.18)); P_('M -0.3 1.3 C -0.5 1.5 -0.54 1.85 -0.5 2.14 L -0.29 1.92 Z', fc); P_('M 0.3 1.3 C 0.5 1.5 0.54 1.85 0.5 2.14 L 0.29 1.92 Z', fc);
  if (on('flame', 1)) { P_('M -0.17 2.15 C -0.22 2.38 -0.07 2.5 0 2.62 C 0.07 2.5 0.22 2.38 0.17 2.15 Z', '#f97316'); P_('M -0.09 2.15 C -0.12 2.3 -0.04 2.38 0 2.46 C 0.04 2.38 0.12 2.3 0.09 2.15 Z', '#fde047'); }
  P_('M -0.2 2.04 L 0.2 2.04 L 0.16 2.17 L -0.16 2.17 Z', dark); }
if (kind === 'bulb' && on('rays', 0)) for (let k = -3; k <= 3; k++) { const a = k * 32, p0 = pt(0, 0.5, 0.58, a), p1 = pt(0, 0.5, 0.72, a); el(g, 'line', { x1: r2(p0[0]), y1: r2(p0[1]), x2: r2(p1[0]), y2: r2(p1[1]), stroke: c0, 'stroke-width': 0.035, 'stroke-linecap': 'round' }); }
// the banded (or filled) body
const cp = el(defs, 'clipPath', { id: 'sil' }); el(cp, 'path', { d: K.sil });
const body = el(g, 'g', { 'clip-path': 'url(#sil)' }), span = K.y1 - K.y0, edges = [K.y0];
for (let i = 1; i <= nb; i++) edges.push(K.first && nb > 1 ? (i === 1 ? K.first : K.first + ((K.y1 - K.first) * (i - 1)) / (nb - 1)) : K.y0 + (span * i) / nb);
if (gauge) { el(body, 'rect', { x: -0.6, y: K.y0 - 0.2, width: 1.2, height: span + 0.4, fill: empty }); const yf = K.y1 - span * frac; el(body, 'rect', { x: -0.6, y: r2(yf), width: 1.2, height: r2(K.y1 - yf + 0.2), fill: c0 }); const p = X(0, yf), l = X(-K.half(yf), yf), r = X(K.half(yf), yf); A('level', 'fill level', p[0], p[1], { lx: Math.round(l[0]), ly: Math.round(l[1]), rx: Math.round(r[0]), ry: Math.round(r[1]), value: num('value', 0) }); }
else for (let i = 0; i < nb; i++) el(body, 'rect', { x: -0.6, y: r2(edges[i] - (i ? 0 : 0.2)), width: 1.2, height: r2(edges[i + 1] - edges[i] + (i ? 0 : 0.2) + (i === nb - 1 ? 0.2 : 0.004)), fill: col(i) });
if (outline) { while (body.firstChild) body.removeChild(body.firstChild); P_(K.sil, str('glass', 'none'), { stroke: c0, 'stroke-width': r2(num('outlinewidth', 0.055) / Math.sqrt(asp)), 'stroke-linejoin': 'round' }); }
if (shade && !outline) { const hm = half0((K.y0 + K.y1) / 2); el(body, 'rect', { x: -0.5, y: K.y0 - 0.2, width: 0.5 - hm * 0.45, height: span + 0.4, fill: '#ffffff', 'fill-opacity': 0.16 }); el(body, 'rect', { x: r2(hm * 0.62), y: K.y0 - 0.2, width: 0.5, height: span + 0.4, fill: '#000000', 'fill-opacity': 0.1 }); }
if (!gauge && nb > 1 && (kind === 'rocket' || kind === 'pencil')) for (let i = 1; i < nb; i++) el(body, 'rect', { x: -0.6, y: r2(edges[i] - 0.012), width: 1.2, height: 0.024, fill: '#000000', 'fill-opacity': 0.12 });
// parts in front
if (kind === 'bulb') { [1.1, 1.2, 1.3].forEach((y, k) => el(g, 'rect', { x: r2(-0.2 + k * 0.012), y, width: r2(0.4 - k * 0.024), height: 0.085, rx: 0.04, fill: dark })); el(g, 'rect', { x: -0.09, y: 1.4, width: 0.18, height: 0.08, rx: 0.04, fill: dark }); if (shade) P_('M -0.33 0.42 A 0.36 0.36 0 0 1 -0.12 0.17', 'none', { stroke: '#ffffff', 'stroke-opacity': 0.7, 'stroke-width': 0.05, 'stroke-linecap': 'round' }); }
if (kind === 'pencil') { el(g, 'rect', { x: -0.25, y: 0, width: 0.5, height: 0.32, rx: 0.1, fill: '#f4a3a8' }); el(g, 'rect', { x: -0.25, y: 0.24, width: 0.5, height: 0.27, fill: '#b8c0cc' }); el(g, 'rect', { x: -0.25, y: 0.31, width: 0.5, height: 0.03, fill: '#8892a0' }); el(g, 'rect', { x: -0.25, y: 0.41, width: 0.5, height: 0.03, fill: '#8892a0' }); P_('M -0.25 2.5 L 0.25 2.5 L 0 3.2 Z', '#f1d2a4'); P_('M -0.082 2.97 L 0.082 2.97 L 0 3.2 Z', dark); }
if (kind === 'thermometer') { P_(K.sil, 'none', { stroke: dark, 'stroke-width': 0.05 }); for (let k = 0; k <= 4; k++) el(g, 'line', { x1: 0.24, x2: 0.36, y1: r2(0.3 + k * 0.42), y2: r2(0.3 + k * 0.42), stroke: dark, 'stroke-width': 0.035, 'stroke-linecap': 'round' }); }
if (kind === 'battery') { el(g, 'rect', { x: -0.47, y: 0.19, width: 0.94, height: 1.68, rx: 0.12, fill: 'none', stroke: dark, 'stroke-width': 0.07 }); el(g, 'rect', { x: -0.18, y: 0.02, width: 0.36, height: 0.15, rx: 0.05, fill: dark }); if (!gauge) for (let i = 1; i < nb; i++) el(g, 'rect', { x: -0.42, y: r2(edges[i] - 0.03), width: 0.84, height: 0.06, fill: bg === 'transparent' ? '#ffffff' : bg }); }
if (kind === 'rocket' && on('window', 0)) { const wy = nb > 1 ? (edges[1] + edges[2]) / 2 : 1.0, wr = num('windowsize', 0.15); el(g, 'ellipse', { cx: 0, cy: r2(wy), rx: r2(wr / asp), ry: r2(wr), fill: str('windowring', '#e5edf5') }); el(g, 'ellipse', { cx: 0, cy: r2(wy), rx: r2((wr * 0.67) / asp), ry: r2(wr * 0.67), fill: str('windowglass', '#7cc4f2') }); }
// badges and anchors (drawn unrotated so the numbers stay upright)
for (let i = 0; i < nb && !gauge; i++) { const ym = kind === 'rocket' && i === 0 && nb > 1 ? edges[1] * 0.64 : (edges[i] + edges[i + 1]) / 2, hw = K.half(ym), p = X(0, ym), l = X(-hw, ym), r = X(hw, ym), b = badges[i] || '';
  A('band' + (i + 1), b, p[0], p[1], { lx: Math.round(l[0]), ly: Math.round(l[1]), rx: Math.round(r[0]), ry: Math.round(r[1]) });
  if (b && !outline && !(kind === 'rocket' && on('window', 0) && i === 1)) { const bs = Math.min((edges[i + 1] - edges[i]) * s * 0.5, (hw * 2 * s * 0.7) / Math.max(1, tw(b, 1, 1))); T(svg, p[0], p[1] + mid(bs), b, { size: bs, weight: numwt, font: numfont, anchor: 'middle', fill: BC.length ? BC[i % BC.length] : onc(col(i)) }); } }
done('object', { kind, scale: r2(s), rotate: rot });
