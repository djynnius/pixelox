const labels = list('labels', str('values', '') || str('notes', '') ? '' : 'Vision|Strategy|Projects|Daily tasks'), notes = list('notes'), vals = rows('values', '')[0] || [], funnel = /^f/i.test(str('kind', vals.length ? 'funnel' : 'pyramid'));
const n = Math.max(labels.length, vals.length, notes.length, 2), flip = on('flip', 0), lp = str('labelpos', funnel ? 'in' : 'right').toLowerCase(), gap = num('gap', 6), ts = num('titlesize', 0) || fs * 1.15, linec = str('line', '#c3cad2');
const g = el(svg, 'g', {}), u = Math.max(0.6, Math.min(B.w, B.h) / 500), side = lp === 'right' || lp === 'left', showV = funnel && on('showvalues', 1), showP = on('pct', 0);
const sw = side ? B.w * num('width', 0.5) : B.w, sx = lp === 'left' ? B.x + B.w - sw : B.x, cx = sx + sw / 2, th = (B.h - gap * (n - 1)) / n, vmax = vmaxOf(vals);
// half-width of the shape at height fraction t (0 = top of the box, 1 = bottom)
const wAt = (i, top) => { if (funnel) { const a = (vals[i] || 0) / vmax, b = i + 1 < n ? (vals[i + 1] || 0) / vmax : a * 0.72; return (sw / 2) * (top ? a : b); } const y = (i * (th + gap) + (top ? 0 : th)) / B.h, t = flip ? 1 - y : y; return (sw / 2) * t; };
const measure = (i, maxw) => { const tl = labels[i] ? wrap(labels[i], maxw, ts, 1) : [], nl = []; if (notes[i]) for (const para of notes[i].split('\\n')) for (const ln of wrap(para, maxw, fs)) nl.push(ln); return { tl, nl, h: tl.length * ts * 1.18 + nl.length * fs * 1.3 + (tl.length && nl.length ? fs * 0.25 : 0) }; };
for (let i = 0; i < n; i++) {
  const y0 = B.y + i * (th + gap), y1 = y0 + th, w0 = wAt(i, 1), w1 = wAt(i, 0), c = col(i), ym = (y0 + y1) / 2, wm = (w0 + w1) / 2;
  el(g, 'polygon', { points: [[cx - w0, y0], [cx + w0, y0], [cx + w1, y1], [cx - w1, y1]].map((p) => r2(p[0]) + ',' + r2(p[1])).join(' '), fill: c, 'stroke-linejoin': 'round' });
  A('tier' + (i + 1), labels[i] || '', cx, ym, { half_width: Math.round(wm) });
  const vt = showV && vals[i] !== undefined ? fmt(vals[i]) + (showP && i ? ' (' + Math.round((vals[i] / (vals[0] || 1)) * 100) + '%)' : '') : '';
  if (side) {
    const dir = lp === 'right' ? 1 : -1, xe = cx + dir * (wm + 8 * u), xt = lp === 'right' ? sx + sw + fs * 1.6 : sx - fs * 1.6, tw0 = B.w - sw - fs * 1.8, m = measure(i, tw0), an = lp === 'right' ? 'start' : 'end';
    el(g, 'line', { x1: r2(xe), y1: r2(ym), x2: r2(xt - dir * fs * 0.5), y2: r2(ym), stroke: linec, 'stroke-width': r2(2 * u), 'stroke-dasharray': r2(2 * u) + ' ' + r2(5 * u), 'stroke-linecap': 'round' }); el(g, 'circle', { cx: r2(xe), cy: r2(ym), r: r2(4.5 * u), fill: c });
    let yy = ym - (m.h + (vt ? ts * 1.2 : 0)) / 2; if (vt) { T(g, xt, yy + ts * 0.85, vt, { size: ts * 1.05, weight: numwt, font: numfont, anchor: an, fill: c }); yy += ts * 1.25; }
    for (const ln of m.tl) { T(g, xt, yy + ts * 0.82, ln, { size: ts, weight: 700, anchor: an }); yy += ts * 1.18; } if (m.tl.length && m.nl.length) yy += fs * 0.25; for (const ln of m.nl) { T(g, xt, yy + fs * 0.82, ln, { anchor: an, fill: muted }); yy += fs * 1.3; }
  } else {
    const s = labels[i] || '', maxw = Math.max(wm * 1.7, 1), tsz = Math.min(ts, th * 0.34, maxw / Math.max(1, tw(s, 1, 1))), inside = tsz >= fs * 0.62, f = onc(c);
    if (inside) { T(g, cx, ym + mid(tsz) - (vt ? tsz * 0.62 : 0), s, { size: tsz, weight: num('labelweight', 700), anchor: 'middle', fill: f }); if (vt) T(g, cx, ym + mid(tsz) + tsz * 0.68, vt, { size: tsz * 0.92, anchor: 'middle', fill: f, font: numfont, opacity: 0.92 }); }
    else T(g, cx + wm + fs * 0.6, ym + mid(fs), s + (vt ? '  ' + vt : ''), { anchor: 'start', weight: 700 });
  }
}
done('pyramid', { shape: [sx, B.y, sw, B.h].map(Math.round), tiers: n });
