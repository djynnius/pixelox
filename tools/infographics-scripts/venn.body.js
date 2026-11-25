const labels = list('labels', 'Useful|Usable|Beautiful'), n = Math.max(2, Math.min(4, labels.length)), overlaps = list('overlaps'), notes = list('notes'), ctr = str('center', '');
const op = num('opacity', 0.78), blend = str('blend', 'multiply').toLowerCase(), ov = Math.max(0.2, Math.min(0.8, num('overlap', 0.5))), caps = on('caps', 0), ts = num('titlesize', 0) || fs * 1.35, lc = str('labelcolor', '');
const g = el(svg, 'g', {}), small = fs * 0.95;
// unit layout: circle radius 1, centers on a ring of radius d around the origin
const d = n === 2 ? 1 - ov * 0.9 : n === 3 ? 1.15 - ov * 0.9 : 1.2 - ov * 0.8, angs = n === 2 ? [270, 90] : n === 3 ? [0, 120, 240] : [0, 90, 180, 270], U = angs.map((a) => pt(0, 0, d, a));
const nh = notes.length ? fs * 3 : 0, nwid = notes.length && n !== 3 ? B.w * 0.16 : 0;
const minx = Math.min.apply(null, U.map((p) => p[0])) - 1, maxx = Math.max.apply(null, U.map((p) => p[0])) + 1, miny = Math.min.apply(null, U.map((p) => p[1])) - 1, maxy = Math.max.apply(null, U.map((p) => p[1])) + 1;
const R = Math.min((B.w - 2 * nwid) / (maxx - minx), (B.h - (n === 4 ? 2 * nh : n === 2 ? 0 : 0)) / (maxy - miny)), cx = B.x + B.w / 2 - (R * (minx + maxx)) / 2, cy = B.y + B.h / 2 - (R * (miny + maxy)) / 2;
const Cn = U.map((p) => [cx + p[0] * R, cy + p[1] * R]), gg = el(g, 'g', { style: 'isolation:isolate' });
Cn.forEach((c, i) => { el(gg, 'circle', { cx: r2(c[0]), cy: r2(c[1]), r: r2(R), fill: col(i), 'fill-opacity': op, style: blend === 'normal' ? undefined : 'mix-blend-mode:' + blend }); A('set' + (i + 1), labels[i] || '', c[0], c[1], { r: Math.round(R) }); });
const fit = (s, size, maxw, bold) => Math.min(size, (maxw / Math.max(1, tw(s, 1, bold))));
// set names: pushed outward from the middle into each circle's own area
Cn.forEach((c, i) => { const out = pt(c[0], c[1], R * (n === 2 ? 0.42 : n === 3 ? 0.38 : 0.45), angs[i]), s = caps ? (labels[i] || '').toUpperCase() : labels[i] || '', f = lc || onc(mix(col(i), blend === 'screen' ? '#000000' : '#ffffff', 1 - op)), lines = wrap(s, R * (n === 2 ? 0.95 : 1.05), ts, 1), sz = Math.min.apply(null, lines.map((ln) => fit(ln, ts, R * 1.1, 1)));
  lines.forEach((ln, k) => T(g, out[0], out[1] + mid(sz) + (k - (lines.length - 1) / 2) * sz * 1.15, ln, { size: sz, weight: 700, anchor: 'middle', fill: f, spacing: caps ? r2(sz * 0.08) : undefined })); });
// pairwise overlaps
const pairs = n === 2 ? [[0, 1]] : n === 3 ? [[0, 2], [0, 1], [1, 2]] : [[0, 1], [1, 2], [2, 3], [3, 0]];
pairs.forEach((pr, k) => { const a = Cn[pr[0]], b = Cn[pr[1]], m = [(a[0] + b[0]) / 2, (a[1] + b[1]) / 2], dx = m[0] - cx, dy = m[1] - cy, len = Math.hypot(dx, dy) || 1, push = n === 2 ? 0 : R * (n === 3 ? 0.3 : 0.24), p = [m[0] + (dx / len) * push, m[1] + (dy / len) * push];
  A('overlap' + (k + 1), overlaps[k] || '', p[0], p[1]); if (!overlaps[k]) return; const maxw = n === 2 ? R * (2 - 2 * d) * 0.82 : R * 0.62, lines = wrap(overlaps[k], maxw, small), sz = Math.min.apply(null, lines.map((ln) => fit(ln, small, maxw * 1.1)));
  lines.forEach((ln, j) => T(g, p[0], p[1] + mid(sz) + (j - (lines.length - 1) / 2) * sz * 1.2, ln, { size: sz, anchor: 'middle', fill: lc || (blend === 'screen' ? '#1f2933' : '#ffffff'), weight: 500 })); });
if (ctr && n > 2) { const maxw = R * (n === 3 ? 0.62 : 0.6), lines = wrap(caps ? ctr.toUpperCase() : ctr, maxw, ts * 0.9, 1), sz = Math.min.apply(null, lines.map((ln) => fit(ln, ts * 0.9, maxw, 1))); lines.forEach((ln, j) => T(g, cx, cy + mid(sz) + (j - (lines.length - 1) / 2) * sz * 1.12, ln, { size: sz, weight: 700, anchor: 'middle', fill: lc || (blend === 'screen' ? '#1f2933' : '#ffffff') })); }
A('center', ctr, cx, cy);
// notes outside the circles
if (notes.length && n !== 3) Cn.forEach((c, i) => { if (!notes[i]) return; const a = angs[i], horiz = a === 90 || a === 270, lines = []; for (const para of notes[i].split('\\n')) for (const ln of wrap(para, horiz ? nwid - fs * 0.6 : R * 1.6, small)) lines.push(ln);
  if (horiz) { const x = a === 90 ? c[0] + R + fs * 0.6 : c[0] - R - fs * 0.6; lines.forEach((ln, k) => T(g, x, c[1] + mid(small) + (k - (lines.length - 1) / 2) * small * 1.3, ln, { size: small, fill: muted, anchor: a === 90 ? 'start' : 'end' })); }
  else lines.forEach((ln, k) => T(g, c[0], a === 0 ? c[1] - R - fs * 0.7 - (lines.length - 1 - k) * small * 1.3 : c[1] + R + fs * 1.4 + k * small * 1.3, ln, { size: small, fill: muted, anchor: 'middle' })); });
done('venn', { radius: Math.round(R), sets: n });
