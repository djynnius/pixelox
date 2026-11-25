const kind = str('kind', 'donut').toLowerCase(), ring = kind === 'ring' || kind === 'gauge';
const vals = ring ? [Math.max(0, num('value', 67))] : (rows('values', '45,25,20,10')[0] || []).map((v) => Math.max(0, v)), labels = list('labels');
const total = ring ? num('max', 100) : vals.reduce((a, b) => a + b, 0) || 1, n = vals.length;
const sweep = Math.min(360, num('sweep', 360)), start = P.start === undefined || P.start === '' ? (sweep < 360 ? -sweep / 2 : 0) : num('start', 0);
const inner = Math.max(0, Math.min(0.95, num('inner', ring ? 0.78 : 0.62))), lp = ring || kind === 'rings' ? 'none' : str('labelpos', 'out').toLowerCase(), show = str('show', 'pct').toLowerCase();
const gapA = num('gap', 0), sep = str('sep', '#ffffff'), nticks = Math.round(num('ticks', 0)), minl = num('minlabel', 4);
const radii = rows('radii', '')[0] || [], gappx = num('gappx', 0), corner = num('corner', 0), multi = kind === 'rings';
// sector with a parallel gap of g px on both sides (the gap keeps its width from the hole to the rim)
const sectorG = (cx, cy, r0, r1, a0, a1, g) => { const h = (r) => (Math.asin(Math.min(1, g / 2 / Math.max(r, g / 2 + 0.01))) * 180) / Math.PI, ho = h(r1), hi = h(Math.max(r0, 1)), big = a1 - a0 - 2 * ho > 180 ? 1 : 0, p = (r, a) => pt(cx, cy, r, a).map(r2).join(' '); if (a1 - a0 - 2 * ho <= 0.1) return ''; return 'M ' + p(r1, a0 + ho) + ' A ' + r2(r1) + ' ' + r2(r1) + ' 0 ' + big + ' 1 ' + p(r1, a1 - ho) + (r0 <= 0.5 ? ' L ' + p(Math.max(1, (g / 2) / Math.sin((Math.max(1, Math.min(179, a1 - a0)) * Math.PI) / 360)), (a0 + a1) / 2) : ' L ' + p(r0, Math.max(a0 + hi, Math.min(a1 - hi, a1 - hi))) + ' A ' + r2(r0) + ' ' + r2(r0) + ' 0 ' + (a1 - a0 - 2 * hi > 180 ? 1 : 0) + ' 0 ' + p(r0, a0 + hi)) + ' Z'; };
const g = el(svg, 'g', {}), u = Math.max(0.6, Math.min(B.w, B.h) / 500), small = fs * 0.9;
const pctOf = (v) => (v / total) * 100, numTxt = (v) => (show === 'value' ? fmt(v) : show === 'both' ? fmt(v) + ' (' + Math.round(pctOf(v)) + '%)' : show === 'none' ? '' : (pctOf(v) >= 10 || Math.abs(pctOf(v) - Math.round(pctOf(v))) < 0.05 ? Math.round(pctOf(v)) : r1(pctOf(v))) + '%');
// unit bounding box of the arc
let ex = [1e9, -1e9, 1e9, -1e9]; const grow = (p) => { ex = [Math.min(ex[0], p[0]), Math.max(ex[1], p[0]), Math.min(ex[2], p[1]), Math.max(ex[3], p[1])]; };
for (let a = start; a <= start + sweep + 0.01; a += 1) grow(pt(0, 0, 1, a)); grow(pt(0, 0, 1, start + sweep)); grow(pt(0, 0, inner, start)); grow(pt(0, 0, inner, start + sweep)); if (inner < 0.02) grow([0, 0]);
let ax = B.x, ay = B.y, aw = B.w, ah = B.h, mx = 0, my = 0;
const items = vals.map((v, i) => ({ label: (labels[i] || '') + (lp === 'legend' && show !== 'none' ? '  ' + numTxt(v) : ''), color: col(i) }));
if (lp === 'legend') { if (B.w > B.h * 1.35) { const lw = Math.max.apply(null, items.map((it) => tw(it.label, fs))) + fs * 2.4; aw -= lw; const x = ax + aw + fs * 0.9, lh = fs * 1.75, y = B.y + B.h / 2 - (n * lh) / 2; items.forEach((it, i) => { el(g, 'rect', { x: r2(x), y: r2(y + i * lh + lh / 2 - fs * 0.4), width: r2(fs * 0.8), height: r2(fs * 0.8), rx: r2(fs * 0.16), fill: it.color }); T(g, x + fs * 1.25, y + i * lh + lh / 2 + mid(fs), it.label, { fill: muted }); }); } else { const lh = Math.ceil(items.reduce((a, it) => a + fs * 2.6 + tw(it.label, fs), 0) / B.w) * fs * 1.7; legend(g, items, B.x + B.w / 2, B.y + B.h - lh + fs * 0.3, B.w, fs); ah -= lh + fs * 0.6; } }
if (lp === 'out') { mx = Math.min(B.w * 0.3, Math.max.apply(null, vals.map((v, i) => Math.max(tw(labels[i] || '', small), tw(numTxt(v), fs * 1.1, 1)))) + fs * 1.5); my = fs * (show === 'none' ? 1.4 : 2.5); }
const fixed = P.r !== undefined && P.r !== '', R = fixed ? (W * num('r', 30)) / 100 : Math.max(10, Math.min((aw - 2 * mx) / (ex[1] - ex[0]), (ah - 2 * my) / (ex[3] - ex[2]))), r0 = R * inner;
const cx = fixed ? (W * num('cx', 50)) / 100 : ax + aw / 2 - (R * (ex[0] + ex[1])) / 2, cy = fixed ? (H * num('cy', 50)) / 100 : ay + ah / 2 - (R * (ex[2] + ex[3])) / 2;
if (ring) {
  const c = col(0), th = R - r0, rm = (R + r0) / 2, cap = on('round', 1) ? 'round' : 'butt', capA = cap === 'round' && sweep < 360 ? ((th / 2 / rm) * 180) / Math.PI : 0;
  const arc = (a0, a1) => { if (a1 - a0 >= 359.99) a1 = a0 + 359.99; const p0 = pt(cx, cy, rm, a0), p1 = pt(cx, cy, rm, a1); return 'M ' + r2(p0[0]) + ' ' + r2(p0[1]) + ' A ' + r2(rm) + ' ' + r2(rm) + ' 0 ' + (a1 - a0 > 180 ? 1 : 0) + ' 1 ' + r2(p1[0]) + ' ' + r2(p1[1]); };
  const s0 = start + capA, s1 = start + sweep - capA, f = Math.max(0, Math.min(1, vals[0] / total));
  el(g, 'path', { d: arc(s0, s1), fill: 'none', stroke: str('trackcolor', mix(c, '#ffffff', 0.82)), 'stroke-width': r2(th), 'stroke-linecap': cap });
  if (f > 0.001) el(g, 'path', { d: arc(s0, s0 + (s1 - s0) * f), fill: 'none', stroke: c, 'stroke-width': r2(th), 'stroke-linecap': cap });
  A('end', 'value end', pt(cx, cy, rm, s0 + (s1 - s0) * f)[0], pt(cx, cy, rm, s0 + (s1 - s0) * f)[1], { value: vals[0] });
} else if (nticks > 0) {
  let acc = 0; const bounds = vals.map((v) => (acc += v / total)), th = Math.max(1.5 * u, ((2 * Math.PI * (R + r0)) / 2) * (sweep / 360) / nticks * 0.55);
  for (let k = 0; k < nticks; k++) { const f = (k + 0.5) / nticks, a = start + sweep * f, i = bounds.findIndex((b) => f <= b + 1e-9), p0 = pt(cx, cy, r0, a), p1 = pt(cx, cy, R, a); el(g, 'line', { x1: r2(p0[0]), y1: r2(p0[1]), x2: r2(p1[0]), y2: r2(p1[1]), stroke: col(i < 0 ? n - 1 : i), 'stroke-width': r2(th) }); }
}
if (multi) { const mx_ = num('max', 100), sw_ = Math.min(sweep, 300), band = (R - r0) / Math.max(1, n), th = band * 0.72, st = P.start === undefined || P.start === '' ? 0 : start;
  vals.forEach((v, i) => { const rm = R - band * (i + 0.5), c = col(i), f = Math.max(0, Math.min(1, v / mx_)), arc = (a0, a1) => { const p0 = pt(cx, cy, rm, a0), p1 = pt(cx, cy, rm, Math.min(a1, a0 + 359.9)); return 'M ' + r2(p0[0]) + ' ' + r2(p0[1]) + ' A ' + r2(rm) + ' ' + r2(rm) + ' 0 ' + (a1 - a0 > 180 ? 1 : 0) + ' 1 ' + r2(p1[0]) + ' ' + r2(p1[1]); };
    el(g, 'path', { d: arc(st, st + sw_), fill: 'none', stroke: str('trackcolor', mix(c, '#ffffff', 0.84)), 'stroke-width': r2(th), 'stroke-linecap': 'round' }); if (f > 0.002) el(g, 'path', { d: arc(st, st + sw_ * f), fill: 'none', stroke: c, 'stroke-width': r2(th), 'stroke-linecap': 'round' });
    const ls = Math.min(fs, th * 0.95), ly = pt(cx, cy, rm, st)[1] + mid(ls), lx = pt(cx, cy, rm, st)[0] - th * 0.5 - ls * 0.5; T(g, lx, ly, (labels[i] ? labels[i] + '  ' : '') + fmt(v) + (suf || mx_ !== 100 ? '' : '%'), { size: ls, anchor: 'end', weight: 600, fill: ink }); const pe = pt(cx, cy, rm, st + sw_ * f); A('ring' + (i + 1), labels[i] || '', pe[0], pe[1], { value: v }); }); }
let a = start; const segs = [];
vals.forEach((v, i) => { const da = (v / total) * sweep, a0 = a, a1 = a + da; a = a1; if (ring || multi) return; const m = (a0 + a1) / 2, c = col(i);
  const Ri = radii.length ? r0 + (R - r0) * Math.max(0.05, Math.min(1, radii[i] === undefined ? 1 : radii[i])) : R;
  if (!nticks && !multi && da > 0.01) { const h = Math.min(gapA / 2, da / 2 - 0.05), cr = Math.min(corner, (Ri - r0) / 2.2);
    if (gappx > 0 || cr > 0) { const d = sectorG(cx, cy, r0 + cr, Ri - cr, a0 + h, a1 - h, gappx + 2 * cr); if (d) el(g, 'path', Object.assign({ d, fill: c }, cr > 0 ? { stroke: c, 'stroke-width': r2(2 * cr), 'stroke-linejoin': 'round' } : {})); }
    else el(g, 'path', Object.assign({ d: sector(cx, cy, r0, Ri, a0 + h, a1 - h), fill: c }, sep !== 'none' && !gapA && n > 1 ? { stroke: sep, 'stroke-width': r2(2 * u), 'stroke-linejoin': 'round' } : {})); }
  const pm = pt(cx, cy, inner < 0.02 ? Ri * 0.62 : (Ri + r0) / 2, m); A('slice' + (i + 1), labels[i] || '', pm[0], pm[1], { value: v, pct: r1(pctOf(v)), angle: r1(m), outer: Math.round(Ri) }); segs.push({ i, v, m, da, c, pm, Ri }); });
if (!ring && !nticks && (lp === 'in' || lp === 'legend') && show !== 'none') for (const s of segs) { const t = numTxt(s.v), arcLen = ((s.da * Math.PI) / 180) * (inner < 0.02 ? R * 0.62 : (R + r0) / 2); if (pctOf(s.v) >= minl && arcLen > tw(t, fs, 1) * 0.9 && (inner < 0.02 || R - r0 > tw(t, fs, 1) * 0.8)) T(g, s.pm[0], s.pm[1] + mid(fs), t, { weight: numwt, font: numfont, anchor: 'middle', fill: onc(s.c) }); }
if (lp === 'out') {
  const sides = { 1: [], '-1': [] }; for (const s of segs) { const p = pt(cx, cy, R, s.m), side = p[0] >= cx - 0.5 ? 1 : -1; sides[side].push(Object.assign(s, { p, side, y: pt(cx, cy, R * 1.14, s.m)[1] })); }
  const lh = fs * (show === 'none' ? 1.3 : 2.45);
  for (const k of [1, -1]) { const L = sides[k].sort((p, q) => p.y - q.y); for (let j = 1; j < L.length; j++) if (L[j].y - L[j - 1].y < lh) L[j].y = L[j - 1].y + lh; const over = L.length ? L[L.length - 1].y + lh / 2 - (B.y + B.h) : 0; if (over > 0) for (let j = L.length - 1; j >= 0; j--) { L[j].y -= over; if (j > 0 && L[j].y - L[j - 1].y >= lh) break; }
    for (const s of L) { const wmax = Math.max(tw(labels[s.i] || '', small), show === 'none' ? 0 : tw(numTxt(s.v), fs * 1.1, 1)), p1 = pt(cx, cy, (s.Ri || R) + R * 0.03, s.m), p2 = [pt(cx, cy, R * 1.12, s.m)[0], s.y];
      let x3 = p2[0] + k * fs * 0.9; x3 = k > 0 ? Math.min(x3, B.x + B.w - wmax - fs * 0.4) : Math.max(x3, B.x + wmax + fs * 0.4);
      el(g, 'polyline', { points: [p1, p2, [x3, s.y]].map((q) => r2(q[0]) + ',' + r2(q[1])).join(' '), fill: 'none', stroke: muted, 'stroke-width': r2(1.2 * u) });
      const tx = x3 + k * fs * 0.35, an = k > 0 ? 'start' : 'end';
      if (show === 'none') T(g, tx, s.y + mid(fs), labels[s.i] || '', { anchor: an, fill: muted });
      else { T(g, tx, s.y - fs * 0.08, numTxt(s.v), { size: fs * 1.1, weight: numwt, font: numfont, anchor: an, fill: ink }); T(g, tx, s.y + small * 1.08, labels[s.i] || '', { size: small, anchor: an, fill: muted }); } } }
}
const ctr = str('center', ring ? fmt(vals[0]) + (suf || total !== 100 ? '' : '%') : ''), sub = str('sub', '');
if (ctr || sub) { const half = sweep <= 200 && inner >= 0.02, room = (inner < 0.02 ? R : r0) * 1.62, cs = Math.min(num('centersize', (inner < 0.02 ? R : r0) * (half ? 0.4 : 0.52)), ctr ? (room / Math.max(1, tw(ctr, 1, 1))) : 1e9), ss = Math.min(fs, r0 * 0.24 || fs);
  const yc = half ? cy - (sub ? ss * 1.5 : 0) - cs * 0.02 : cy + (sub ? -ss * 0.55 : 0) + cs * 0.36;
  if (ctr) T(g, cx, yc, ctr, { size: cs, weight: numwt, font: numfont, anchor: 'middle' });
  if (sub) wrap(sub, room, ss).slice(0, 2).forEach((ln, k) => T(g, cx, (half ? cy - ss * 0.2 : yc + ss * 1.45) + k * ss * 1.2 - (half ? (wrap(sub, room, ss).slice(0, 2).length - 1 - k) * ss * 1.2 : 0), ln, { size: ss, anchor: 'middle', fill: muted })); }
A('center', 'center', cx, cy);
done('donut', { center: [Math.round(cx), Math.round(cy)], outer: Math.round(R), inner: Math.round(r0) });
