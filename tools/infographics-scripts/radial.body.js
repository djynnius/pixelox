const vals = (rows('values', '')[0] || []).map((v) => Math.max(0, v)), labels = list('labels'), hasV = vals.length > 0, n = Math.max(vals.length, labels.length, 1);
const sweep = Math.min(360, num('sweep', 360)), start = num('start', 0), inner = Math.max(0.1, Math.min(0.9, num('inner', hasV ? 0.36 : 0.62))), rp = str('rotate', 'auto').toLowerCase(), rot = rp === 'auto' ? n > 12 : on('rotate', 0);
const bwid = num('barwidth', 0.62), vmax = num('max', 0) > 0 ? num('max', 0) : Math.max.apply(null, vals.concat([1])), nr = Math.round(num('rings', 0)), showV = hasV && on('vlabels', 1), track = on('track', 0), hi = Math.round(num('highlight', 0)), accent = str('accent', '#f97316');
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
