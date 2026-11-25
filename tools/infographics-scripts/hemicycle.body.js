const seats = (rows('seats', '48,5,47')[0] || []).map((v) => Math.max(0, Math.round(v))), labels = list('labels'), N = seats.reduce((a, b) => a + b, 0);
const sweep = Math.max(60, Math.min(340, num('sweep', 180))), flip = on('flip', 0), inner = Math.max(0.1, Math.min(0.85, num('inner', 0.38)));
const KK = (2 * N * (1 - inner)) / (((sweep * Math.PI) / 180) * (1 + inner)), nrows = Math.round(num('rows', 0)) || Math.max(2, Math.min(14, Math.round((1 + Math.sqrt(1 + 4 * KK)) / 2)));
const shape = str('shape', 'circle').toLowerCase(), fillF = num('fill', 0.84), hollow = list('hollow').map((x) => parseInt(x, 10) - 1), g = el(svg, 'g', {}), u = Math.max(0.6, Math.min(B.w, B.h) / 500), small = fs * 0.9;
const names = labels.length > 0, lg = names ? str('legend', 'bottom').toLowerCase() : 'none', showCounts = on('counts', 1);
let ay = B.y, ah = B.h; const items = seats.map((v, i) => ({ label: (labels[i] || '') + (showCounts ? '  ' + v : ''), color: col(i) }));
if (lg !== 'none') { const lh = Math.ceil(items.reduce((a, it) => a + small * 2.6 + tw(it.label, small), 0) / B.w) * small * 1.7; if (lg === 'top') { legend(g, items, B.x + B.w / 2, B.y, B.w, small); ay += lh + fs * 0.5; ah -= lh + fs * 0.5; } else { legend(g, items, B.x + B.w / 2, B.y + B.h - lh + small * 0.3, B.w, small); ah -= lh + fs * 0.6; } }
// rows: radii from inner to outer in unit space; seats per row proportional to the row's arc length
const a0 = -sweep / 2, half = (sweep * Math.PI) / 360, radii = []; for (let k = 0; k < nrows; k++) radii.push(nrows === 1 ? 1 : inner + ((1 - inner) * k) / (nrows - 1));
const dr = nrows > 1 ? (1 - inner) / (nrows - 1) : 0.3, sumR = radii.reduce((a, b) => a + b, 0), per = radii.map((r) => Math.round((N * r) / sumR));
let diff = N - per.reduce((a, b) => a + b, 0); for (let k = nrows - 1; diff !== 0; k = (k - 1 + nrows) % nrows) { per[k] += diff > 0 ? 1 : -1; diff += diff > 0 ? -1 : 1; }
// unit bounding box (including the seat radius), then fit to the box
const arcGap = Math.min.apply(null, radii.map((r, k) => (per[k] > 1 ? (((sweep * Math.PI) / 180) * r) / (per[k] - 1) : 9))), sr0 = Math.min(dr, arcGap) * fillF * 0.5, ext = sweep > 180 ? Math.cos(Math.PI - half) : 0, uw = 2 * (sweep >= 180 ? 1 : Math.sin(half)) + 2 * sr0, uh = 1 + (sweep > 180 ? -Math.cos(half) : -Math.cos(half) * inner) + 2 * sr0;
const fixed = P.r !== undefined && P.r !== '', R = fixed ? ((W * num('r', 30)) / 100) / (1 + sr0) : Math.min(B.w / uw, ah / uh), cx = fixed ? (W * num('cx', 50)) / 100 : B.x + B.w / 2, top = ay + (ah - uh * R) / 2 + sr0 * R, cyUp = top + R, cy = fixed ? (H * num('cy', 50)) / 100 : flip ? ay + ah - (cyUp - ay) : cyUp, sr = sr0 * R;
const all = []; radii.forEach((r, k) => { for (let j = 0; j < per[k]; j++) { const f = per[k] === 1 ? 0.5 : j / (per[k] - 1), a = a0 + sweep * f; all.push({ a, f, r, k }); } });
all.sort((p, q) => p.f - q.f || q.r - p.r);
let idx = 0; const mids = [];
seats.forEach((cnt, gi) => { let sx = 0, sy = 0; for (let j = 0; j < cnt; j++, idx++) { const s = all[idx]; let p = pt(cx, cy, s.r * R, s.a); if (flip) p = [p[0], 2 * cy - p[1]]; sx += p[0]; sy += p[1]; const c = col(gi), hol = hollow.indexOf(gi) >= 0, st = hol ? { fill: 'none', stroke: c, 'stroke-width': r2(Math.max(1.2, sr * 0.22)) } : { fill: c }, q = hol ? sr * 0.88 : sr;
    if (shape === 'square') el(g, 'rect', Object.assign({ x: r2(p[0] - q * 0.9), y: r2(p[1] - q * 0.9), width: r2(q * 1.8), height: r2(q * 1.8), rx: r2(q * 0.2) }, st));
    else if (shape === 'hex') el(g, 'polygon', Object.assign({ points: [0, 60, 120, 180, 240, 300].map((d) => pt(p[0], p[1], q * 1.08, d + 30).map(r2).join(',')).join(' ') }, st));
    else el(g, 'circle', Object.assign({ cx: r2(p[0]), cy: r2(p[1]), r: r2(q) }, st)); }
  if (cnt) { mids.push({ gi, x: sx / cnt, y: sy / cnt }); A('group' + (gi + 1), labels[gi] || '', sx / cnt, sy / cnt, { seats: cnt }); } });
if (on('majority', 0)) { const p0 = pt(cx, cy, inner * R - sr * 1.6, 0), p1 = pt(cx, cy, R + sr * 1.6, 0), fy = (y) => (flip ? 2 * cy - y : y); el(g, 'line', { x1: r2(p0[0]), y1: r2(fy(p0[1])), x2: r2(p1[0]), y2: r2(fy(p1[1])), stroke: ink, 'stroke-width': r2(1.5 * u), 'stroke-dasharray': r2(4 * u) + ' ' + r2(3 * u) }); }
const ctr = str('center', ''), sub = str('sub', ''), room = inner * R * 1.5 - sr * 2;
if (ctr || sub) { const cs = Math.min(num('centersize', inner * R * 0.38), ctr ? room / Math.max(1, tw(ctr, 1, 1)) : 1e9), ss = Math.min(fs, inner * R * 0.2), by = flip ? cy + cs * 0.72 + (sweep > 180 ? 0 : sr) : cy - (sub ? ss * 1.5 : 0) - (sweep > 180 ? -cs * 0.3 : sr * 0.4);
  if (ctr) T(g, cx, by, ctr, { size: cs, weight: numwt, font: numfont, anchor: 'middle' }); if (sub) T(g, cx, by + ss * 1.35, sub, { size: ss, anchor: 'middle', fill: muted }); }
A('center', 'center', cx, cy);
done('hemicycle', { center: [Math.round(cx), Math.round(cy)], outer: Math.round(R), inner: Math.round(inner * R), seat_radius: r1(sr), rows: nrows, per_row: per, total: N });
