const S = rows('values', '12,18,17,26,31,44'), labels = list('labels'), names = list('series'), ns = S.length, n = Math.max.apply(null, S.map((s) => s.length));
const spark = on('spark', 0), areaP = str('area', '0').toLowerCase(), stack = areaP === 'stack' && ns > 1, area = stack || on('area', 0), smooth = on('smooth', 0);
const cum = S.map((s) => s.slice()); if (stack) for (let s = 1; s < ns; s++) for (let i = 0; i < n; i++) cum[s][i] = (cum[s - 1][i] || 0) + (S[s][i] || 0);
const dmax = Math.max.apply(null, cum.map((s) => Math.max.apply(null, s))), dmin = Math.min.apply(null, S.map((s) => Math.min.apply(null, s)));
// value range: from min= (default 0), or from a round number under the data when it goes below zero; a flat sparkline gets a padded range
const nt = Math.round(num('ticks', 4)), pad = dmax > dmin ? (dmax - dmin) * 0.1 : Math.abs(dmax) * 0.1 || 1;
const negStep = !spark && str('min', '') === '' && dmin < 0 ? stepFor(dmin, Math.max(dmax, 0), nt) : 0;
const lo = spark ? dmin - pad : negStep ? Math.floor(dmin / negStep) * negStep : num('min', 0);
const sc0 = negStep ? { step: negStep, max: (num('max', 0) > 0 ? num('max', 0) : Math.max(negStep, Math.ceil(Math.max(dmax, 0) / negStep - 1e-9) * negStep)) - lo } : num('max', 0) > lo ? { max: num('max', 0), step: (num('max', 0) - lo) / nt } : nice(dmax - lo, nt);
const hiV = spark ? dmax + pad : negStep ? lo + sc0.max : num('max', 0) > lo ? sc0.max : lo + sc0.max, span = hiV - lo > 0 ? hiV - lo : 1;
const u = Math.max(0.6, Math.min(B.w, B.h) / 500), small = fs * 0.9, showAxis = !spark && on('axis', 1), showGrid = !spark && on('gridlines', 1), lw = num('width', 4) * u;
const vl = str('vlabels', 'last').toLowerCase(), endl = !spark && names.length > 0 && on('endlabels', 1) && !stack, markers = !spark && on('markers', 1) && n <= 24 && !stack;
const g = el(svg, 'g', {});
let top = B.y, bottom = B.y + B.h, left = B.x, right = B.x + B.w;
const lg = names.length && !endl && !spark ? str('legend', 'top').toLowerCase() : 'none';
if (lg === 'top') top += legend(g, names.map((nm, s) => ({ label: nm, color: col(s) })), B.x + B.w / 2, top, B.w, small) + fs * 0.4;
const ticks = []; for (let v = lo; v <= hiV + 1e-9; v += sc0.step) ticks.push(v);
if (showAxis) left += Math.max.apply(null, ticks.map((v) => tw(fmt(v), small))) + fs * 0.6;
if (endl) right -= Math.max.apply(null, names.map((s) => tw(s, small, 1))) + fs * 0.8; else if (vl !== 'none') right -= tw(fmt(dmax), fs, 1) * (spark ? 1 : 0.55) + fs * 0.5;
if (labels.length && !spark) bottom -= small * 1.75;
top += vl === 'none' || spark ? lw : fs * 1.4;
if (!showAxis && !spark) left += labels.length ? tw(labels[0], small) / 2 : lw;
if (spark) { left += lw; bottom -= lw; }
const x0 = left, x1 = right, y0 = bottom, y1 = top, X = (i) => (n > 1 ? x0 + ((x1 - x0) * i) / (n - 1) : (x0 + x1) / 2), Y = (v) => y0 - ((v - lo) / span) * (y0 - y1), yz = lo < 0 && hiV > 0 && !spark ? Y(0) : y0;
if (showGrid) for (const v of ticks) el(g, 'line', { x1: r2(x0), x2: r2(x1), y1: r2(Y(v)), y2: r2(Y(v)), stroke: gridc, 'stroke-width': r2(u) });
if (showAxis) for (const v of ticks) T(g, x0 - fs * 0.5, Y(v) + mid(small), fmt(v), { size: small, fill: muted, anchor: 'end' });
const path = (pts) => { if (!smooth || pts.length < 3) return pts.map((p, i) => (i ? 'L ' : 'M ') + r2(p[0]) + ' ' + r2(p[1])).join(' '); let d = 'M ' + r2(pts[0][0]) + ' ' + r2(pts[0][1]); for (let i = 0; i < pts.length - 1; i++) { const p0 = pts[Math.max(0, i - 1)], p1 = pts[i], p2 = pts[i + 1], p3 = pts[Math.min(pts.length - 1, i + 2)], t = 0.18; d += ' C ' + r2(p1[0] + (p2[0] - p0[0]) * t) + ' ' + r2(p1[1] + (p2[1] - p0[1]) * t) + ' ' + r2(p2[0] - (p3[0] - p1[0]) * t) + ' ' + r2(p2[1] - (p3[1] - p1[1]) * t) + ' ' + r2(p2[0]) + ' ' + r2(p2[1]); } return d; };
const P_ = cum.map((s) => s.map((v, i) => [X(i), Y(v)]));
for (let s = ns - 1; s >= 0; s--) if (area) {
  const base = stack && s > 0 ? P_[s - 1].slice().reverse() : [[X(P_[s].length - 1), yz], [X(0), yz]];
  el(g, 'path', { d: path(P_[s]) + ' ' + base.map((p) => 'L ' + r2(p[0]) + ' ' + r2(p[1])).join(' ') + ' Z', fill: col(s), 'fill-opacity': stack ? 1 : ns > 1 ? 0.22 : 0.18 });
}
if (!spark) el(g, 'line', { x1: r2(x0), x2: r2(x1), y1: r2(yz), y2: r2(yz), stroke: muted, 'stroke-width': r2(1.5 * u) });
const step = Math.max(1, Math.ceil((labels.reduce((a, s) => Math.max(a, tw(s, small)), 0) + small) / Math.max(1, (x1 - x0) / Math.max(1, n - 1))));
if (!spark) labels.forEach((s, i) => { if (i % step === 0 || i === n - 1 && (n - 1) % step >= step / 2) T(g, X(i), y0 + small * 1.45, s, { size: small, fill: muted, anchor: 'middle' }); });
let ends = [];
for (let s = 0; s < ns; s++) {
  if (!stack) el(g, 'path', { d: path(P_[s]), fill: 'none', stroke: col(s), 'stroke-width': r2(lw), 'stroke-linejoin': 'round', 'stroke-linecap': 'round' });
  if (markers) P_[s].forEach((p) => el(g, 'circle', { cx: r2(p[0]), cy: r2(p[1]), r: r2(lw * 1.25), fill: bg === 'transparent' ? '#ffffff' : bg, stroke: col(s), 'stroke-width': r2(lw * 0.7) }));
  if (spark) { const p = P_[s][P_[s].length - 1]; el(g, 'circle', { cx: r2(p[0]), cy: r2(p[1]), r: r2(lw * 1.3), fill: col(s) }); }
  P_[s].forEach((p, i) => { A('p' + (s + 1) + '.' + (i + 1), (labels[i] || '') + (names[s] ? ' / ' + names[s] : ''), p[0], p[1], { value: S[s][i] }); if (vl === 'all' && !stack) T(g, p[0], p[1] - fs * 0.65, fmt(S[s][i]), { size: small, weight: numwt, font: numfont, anchor: 'middle', fill: ns > 1 ? col(s) : ink }); });
  ends.push({ s, i: P_[s].length - 1, y: P_[s][P_[s].length - 1][1] }); // a shorter series ends at its own last point
}
// end labels and last values, nudged apart so they never overlap
ends.sort((a, b) => a.y - b.y); const lh = (endl && vl === 'last' ? small * 2.25 : fs * 1.2); for (let k = 1; k < ends.length; k++) if (ends[k].y - ends[k - 1].y < lh) ends[k].y = ends[k - 1].y + lh;
for (const e of ends) { const s = e.s, xe = X(n - 1) + lw * 1.6 + fs * 0.25;
  if (endl) { T(g, xe, e.y + (vl === 'last' ? -small * 0.1 : mid(small)), names[s], { size: small, weight: 700, fill: col(s) }); if (vl === 'last') T(g, xe, e.y + small * 1.05, fmt(S[s][e.i]), { size: small, fill: muted, font: numfont }); }
  else if (vl === 'last' && !stack) T(g, xe, e.y + mid(fs), fmt(S[s][e.i]), { size: fs, weight: numwt, font: numfont, fill: ns > 1 ? col(s) : ink });
}
done('line', { plot: [x0, y1, x1 - x0, y0 - y1].map(Math.round), min: lo, max: hiV });
