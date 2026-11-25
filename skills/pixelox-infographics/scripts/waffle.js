// waffle.js: unit charts from counts: a waffle (10 x 10 squares = 100%), an icon array of people ("4 out of 5"),
// a dot matrix, or a seat grid. One unit = one thing; exactly the given number of units is drawn.
// Run:   pixelox render js scripts/waffle.js -o waffle.png --size 1080x1350 --param box=10,30,80,40 --param values=62,23,15 --param labels='Agree,Unsure,Disagree'
// Place: pixelox comp place d.pxo waffle.png --in-place --as Waffle --at 0,0
// Data params (use one form):
//   values=62,23,15          units per group, filled in order (a group per color)
//   value=4 total=5          one share: 'value' units in the first color, the rest in the 'empty' color (value may be 3.5)
//   labels=A,B,C             group names for the legend
// Form params (all optional):
//   cols=10                  units per row (rows follow from the total)
//   shape=square             square | circle | hex | person
//   gap=0.14                 space between units as a fraction of a unit
//   from=top                 top | bottom: the row where filling starts (bottom is the classic waffle)
//   order=rows               rows | cols: fill row by row, or column by column (groups become side-by-side blocks)
//   empty=#d5dbe1            color of the unfilled units (value/total form)
//   hollow=                  1-based group numbers drawn as outlines (vacant, undecided, up for election)
//   layout=grid              grid | rows (one row of dots per group, its name on the left and its count on the
//                            right: a dot-row chart for small counts, up to about 30 per row)
//   align=center             center | left: where the grid sits in the box
//   counts=1 legend=bottom   counts after the names in the legend; bottom | top | none
// Shared params (every chart script): box=x,y,w,h in % of the image (default 0,0,100,100); colors=#hex,#hex;
//   ink=#1f2933 (text) muted=#6b7785 (axis text) grid=#d5dbe1 bg=transparent; fs=label size in px;
//   font='Family, Fallback, sans-serif' (a family name: font classes such as @condensed do not work inside a script)
//   numfont= weight=400 numweight=700; prefix=$ suffix=% decimals=1 compact=1 (1.2K, 3.4M)
// The result's logs hold one JSON line: the grid rectangle, the unit size, and an anchor at the middle of every group.
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
const single = P.value !== undefined && P.value !== '', val1 = num('value', 0), total1 = Math.round(num('total', 100));
const vals = single ? [val1, Math.max(0, total1 - val1)] : (rows('values', '62,23,15')[0] || []).map((v) => Math.max(0, Math.round(v)));
const labels = list('labels'), N = single ? total1 : vals.reduce((a, b) => a + b, 0), cols = Math.max(1, Math.round(num('cols', single && total1 <= 12 ? total1 : 10))), nr = Math.ceil(N / cols);
const shape = str('shape', 'square').toLowerCase(), gap = num('gap', shape === 'person' ? 0.22 : 0.14), asp = shape === 'person' ? 2.15 : 1, empty = str('empty', '#d5dbe1'), fromB = /^b/i.test(str('from', 'top'));
const g = el(svg, 'g', {}), small = fs * 0.9, lg = labels.length && !/^r/i.test(str('layout', 'grid')) ? str('legend', 'bottom').toLowerCase() : 'none';
let ay = B.y, ah = B.h; const items = (single ? [vals[0]] : vals).map((v, i) => ({ label: (labels[i] || '') + (on('counts', 1) ? '  ' + fmt(v) : ''), color: col(i) }));
if (lg !== 'none') { const lh = Math.ceil(items.reduce((a, it) => a + small * 2.6 + tw(it.label, small), 0) / B.w) * small * 1.7; if (lg === 'top') { legend(g, items, B.x + B.w / 2, B.y, B.w, small); ay += lh + fs * 0.5; ah -= lh + fs * 0.5; } else { legend(g, items, B.x + B.w / 2, B.y + B.h - lh + small * 0.3, B.w, small); ah -= lh + fs * 0.6; } }
if (/^r/i.test(str('layout', 'grid')) && !single) { // one row of dots per group: name on the left, count on the right
  const nrw = vals.length, maxv = Math.max.apply(null, vals.concat([1])), labw = labels.length ? Math.max.apply(null, labels.map((q) => tw(q, fs))) + fs * 0.8 : 0, cntw = on('counts', 1) ? Math.max.apply(null, vals.map((v) => tw(fmt(v), fs, 1))) + fs * 0.7 : 0;
  const rh = B.h / nrw, cwR = Math.min((B.w - labw - cntw) / (maxv + (maxv - 1) * gap), rh * 0.62), cw = cwR, pitch = cw * (1 + gap), x0r = B.x + labw;
  vals.forEach((v, i) => { const yc = B.y + rh * (i + 0.5), c = col(i); if (labels[i]) T(g, B.x, yc + mid(fs), labels[i], { fill: muted });
    for (let k = 0; k < v; k++) { const x = x0r + k * pitch, y = yc - cw / 2; if (shape === 'square') el(g, 'rect', { x: r2(x), y: r2(y), width: r2(cw), height: r2(cw), rx: r2(cw * 0.12), fill: c }); else el(g, 'circle', { cx: r2(x + cw / 2), cy: r2(yc), r: r2(cw / 2), fill: c }); }
    if (on('counts', 1)) T(g, x0r + v * pitch + fs * 0.3, yc + mid(fs), fmt(v), { weight: numwt, font: numfont }); A('group' + (i + 1), labels[i] || '', x0r + (v * pitch) / 2, yc, { units: v }); });
  done('waffle', { layout: 'rows', unit: r1(cw), rows: nrw });
} else {
const cw = Math.min(B.w / (cols + (cols - 1) * gap), ah / (nr * asp + (nr - 1) * gap * (shape === 'person' ? 0.6 : 1))), chh = cw * asp, pitchX = cw * (1 + gap), pitchY = chh + cw * gap * (shape === 'person' ? 0.6 : 1);
const gw = cols * cw + (cols - 1) * cw * gap, gh = nr * chh + (nr - 1) * (pitchY - chh), gx = /^l/i.test(str('align', 'center')) ? B.x : B.x + (B.w - gw) / 2, gy = ay + (ah - gh) / 2;
const hollow = list('hollow').map((x) => parseInt(x, 10) - 1);
const unit = (parent, x, y, c, hol) => {
  if (hol && shape !== 'person') { const lw = Math.max(1.2, cw * 0.1); if (shape === 'circle') el(parent, 'circle', { cx: r2(x + cw / 2), cy: r2(y + cw / 2), r: r2(cw / 2 - lw / 2), fill: 'none', stroke: c, 'stroke-width': r2(lw) }); else if (shape === 'hex') el(parent, 'polygon', { points: [0, 60, 120, 180, 240, 300].map((d) => pt(x + cw / 2, y + cw / 2, cw * 0.56 - lw / 2, d + 30).map(r2).join(',')).join(' '), fill: 'none', stroke: c, 'stroke-width': r2(lw) }); else el(parent, 'rect', { x: r2(x + lw / 2), y: r2(y + lw / 2), width: r2(cw - lw), height: r2(cw - lw), rx: r2(cw * 0.12), fill: 'none', stroke: c, 'stroke-width': r2(lw) }); return; }
  if (shape === 'circle') el(parent, 'circle', { cx: r2(x + cw / 2), cy: r2(y + cw / 2), r: r2(cw / 2), fill: c });
  else if (shape === 'hex') el(parent, 'polygon', { points: [0, 60, 120, 180, 240, 300].map((d) => pt(x + cw / 2, y + cw / 2, cw * 0.56, d + 30).map(r2).join(',')).join(' '), fill: c });
  else if (shape === 'person') { const w = cw; el(parent, 'circle', { cx: r2(x + w * 0.5), cy: r2(y + w * 0.2), r: r2(w * 0.2), fill: c }); el(parent, 'rect', { x: r2(x + w * 0.1), y: r2(y + w * 0.47), width: r2(w * 0.8), height: r2(w * 0.86), rx: r2(w * 0.17), fill: c }); el(parent, 'rect', { x: r2(x + w * 0.27), y: r2(y + w * 1.05), width: r2(w * 0.205), height: r2(w * 1.1), rx: r2(w * 0.09), fill: c }); el(parent, 'rect', { x: r2(x + w * 0.525), y: r2(y + w * 1.05), width: r2(w * 0.205), height: r2(w * 1.1), rx: r2(w * 0.09), fill: c }); }
  else el(parent, 'rect', { x: r2(x), y: r2(y), width: r2(cw), height: r2(cw), rx: r2(cw * 0.12), fill: c });
};
const sums = vals.map(() => [0, 0, 0]); let k = 0, gi = 0, left = vals[0] || 0;
for (let i = 0; i < N; i++) {
  while (left <= 1e-9 && gi < vals.length - 1) { gi++; left = vals[gi]; }
  const byCol = /^c/i.test(str('order', 'rows')), rr = byCol ? i % nr : Math.floor(i / cols), cc = byCol ? Math.floor(i / nr) : i % cols, x = gx + cc * pitchX, y = gy + (fromB ? nr - 1 - rr : rr) * pitchY, c = single ? (gi === 0 ? col(0) : empty) : col(gi);
  if (single && gi === 0 && left < 1 - 1e-9) { unit(g, x, y, empty); const id = 'part' + i, cp = el(defs, 'clipPath', { id }); el(cp, 'rect', { x: r2(x), y: r2(y), width: r2(cw * left), height: r2(chh) }); unit(el(g, 'g', { 'clip-path': 'url(#' + id + ')' }), x, y, col(0)); left = 0; }
  else { unit(g, x, y, c, !single && hollow.indexOf(gi) >= 0); left -= 1; }
  sums[gi][0] += x + cw / 2; sums[gi][1] += y + chh / 2; sums[gi][2]++;
}
sums.forEach((s, i) => { if (s[2]) A('group' + (i + 1), labels[i] || (single && i ? 'rest' : ''), s[0] / s[2], s[1] / s[2], { units: vals[i] }); });
done('waffle', { grid: [gx, gy, gw, gh].map(Math.round), unit: r1(cw), cols, rows: nr, total: N });
}
