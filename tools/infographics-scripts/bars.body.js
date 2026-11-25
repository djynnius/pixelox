const S = rows('values', '42,31,18,9'), labels = list('labels'), names = list('series');
const n = Math.max.apply(null, S.map((s) => s.length)), ns = S.length;
const horiz = /^h/i.test(str('dir', 'v')), mode = str('mode', 'group').toLowerCase(), stacked = mode !== 'group' && mode !== 'diverge' && ns > 1, pct = mode === 'percent';
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
const rawMin = !stacked && !horiz && mode !== 'diverge' ? Math.min.apply(null, S.map((q) => Math.min.apply(null, q))) : 0, neg = rawMin < 0;
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
  if (showV && vl !== 'inside' && !stacked) right -= Math.max.apply(null, S.map((s) => Math.max.apply(null, s.map((v) => tw(fmt(v), fs, 1))))) + fs * 0.6;
  if (showAxis) bottom -= small * 1.7;
  x0 = left; x1 = right; y0 = top; y1 = bottom;
  const band = (y1 - y0) / n, gap = num('gap', ns > 1 && !stacked ? 0.3 : track ? 0.52 : 0.34), inner = band * (1 - gap), bh = stacked ? inner - nameH : (inner - nameH) / ns, rad = Math.min(num('radius', track ? bh : 3 * u), bh / 2);
  if (showGrid) for (const v of tickLabels) { const x = x0 + (v / sc.max) * (x1 - x0); el(g, 'line', { x1: r2(x), x2: r2(x), y1: r2(y0), y2: r2(y1), stroke: gridc, 'stroke-width': r2(u) }); }
  if (showAxis) for (const v of tickLabels) T(g, x0 + (v / sc.max) * (x1 - x0), y1 + small * 1.35, tl(v), { size: small, fill: muted, anchor: 'middle' });
  const hbar = (x, y, w, h, r, both) => (w <= 0.01 ? '' : (r = Math.min(r, w, h / 2), 'M ' + r2(x + (both ? r : 0)) + ' ' + r2(y) + ' L ' + r2(x + w - r) + ' ' + r2(y) + ' Q ' + r2(x + w) + ' ' + r2(y) + ' ' + r2(x + w) + ' ' + r2(y + r) + ' L ' + r2(x + w) + ' ' + r2(y + h - r) + ' Q ' + r2(x + w) + ' ' + r2(y + h) + ' ' + r2(x + w - r) + ' ' + r2(y + h) + ' L ' + r2(x + (both ? r : 0)) + ' ' + r2(y + h) + (both ? ' Q ' + r2(x) + ' ' + r2(y + h) + ' ' + r2(x) + ' ' + r2(y + h - r) + ' L ' + r2(x) + ' ' + r2(y + r) + ' Q ' + r2(x) + ' ' + r2(y) + ' ' + r2(x + r) + ' ' + r2(y) : '') + ' Z'));
  for (let i = 0; i < n; i++) {
    const by0 = y0 + band * i + (band - inner) / 2; let acc = 0;
    for (let s = 0; s < ns; s++) {
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
    else if (labels[i]) T(g, x0 - fs * 0.6, by0 + inner / 2 + mid(fs), labels[i], { size: fs, anchor: 'end', fill: hi && i + 1 === hi ? ink : muted, weight: hi && i + 1 === hi ? 700 : wt });
  }
  if (!track) el(g, 'line', { x1: r2(x0), x2: r2(x0), y1: r2(y0), y2: r2(y1), stroke: muted, 'stroke-width': r2(1.5 * u) });
}
done('bars', { plot: [x0, Math.min(y0, y1), x1 - x0, Math.abs(y1 - y0)].map(Math.round), max: sc.max });
