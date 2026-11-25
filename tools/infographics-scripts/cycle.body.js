const labels = list('labels', 'Plan|Build|Measure|Learn'), notes = list('notes'), n = Math.max(labels.length, notes.length, 2);
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
