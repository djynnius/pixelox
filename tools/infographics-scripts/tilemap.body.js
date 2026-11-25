const US = 'AK:0:0,ME:10:0,VT:9:1,NH:10:1,WA:0:2,ID:1:2,MT:2:2,ND:3:2,MN:4:2,IL:5:2,WI:6:2,MI:7:2,NY:8:2,RI:9:2,MA:10:2,OR:0:3,NV:1:3,WY:2:3,SD:3:3,IA:4:3,IN:5:3,OH:6:3,PA:7:3,NJ:8:3,CT:9:3,CA:0:4,UT:1:4,CO:2:4,NE:3:4,MO:4:4,KY:5:4,WV:6:4,VA:7:4,MD:8:4,DE:9:4,AZ:1:5,NM:2:5,KS:3:5,AR:4:5,TN:5:5,NC:6:5,SC:7:5,DC:8:5,OK:3:6,LA:4:6,MS:5:6,AL:6:6,GA:7:6,HI:0:7,TX:3:7,FL:8:7';
const cells = split(str('cells', US)).map((c) => { const p = c.split(':'); return { code: p[0].trim(), c: parseInt(p[1], 10) || 0, r: parseInt(p[2], 10) || 0 }; });
const kv = (k) => { const m = {}; for (const it of split(str(k, ''))) { const p = it.split(':'); if (p.length > 1) m[p[0].trim().toUpperCase()] = parseFloat(p[1]); } return m; };
const groups = kv('groups'), values = kv('values'), hasVal = Object.keys(values).length > 0, byVal = hasVal && Object.keys(groups).length === 0, names = list('names'), ramp = list('ramp', '#dbeafe,#1d4ed8'), empty = str('empty', '#e2e6ea');
const shape = str('shape', 'square').toLowerCase(), gap = num('gap', 0.08), showCodes = on('codes', 1), showVals = on('showvalues', 0) && hasVal, g = el(svg, 'g', {}), small = fs * 0.9;
const vs = Object.keys(values).map((k) => values[k]), lo = P.min !== undefined && P.min !== '' ? num('min', 0) : Math.min.apply(null, vs.length ? vs : [0]), hi = P.max !== undefined && P.max !== '' ? num('max', 1) : Math.max.apply(null, vs.length ? vs : [1]);
const rampAt = (t) => { t = Math.max(0, Math.min(1, t)); if (ramp.length >= 3) return t < 0.5 ? mix(ramp[0], ramp[1], t * 2) : mix(ramp[1], ramp[2], (t - 0.5) * 2); return mix(ramp[0], ramp[1] || ramp[0], t); };
const colorOf = (code) => { const k = code.toUpperCase(); if (byVal) return values[k] === undefined ? empty : rampAt(hi > lo ? (values[k] - lo) / (hi - lo) : 1); return groups[k] ? col(Math.round(groups[k]) - 1) : empty; };
const lg = str('legend', byVal || names.length ? 'bottom' : 'none').toLowerCase(); let ay = B.y, ah = B.h;
const used = []; for (const k in groups) if (used.indexOf(Math.round(groups[k])) < 0) used.push(Math.round(groups[k])); used.sort((a, b) => a - b);
const items = used.map((i) => ({ label: names[i - 1] || 'Group ' + i, color: col(i - 1) }));
const lgH = lg === 'none' ? 0 : byVal ? small * 2.6 : Math.ceil(items.reduce((a, it) => a + small * 2.6 + tw(it.label, small), 0) / B.w) * small * 1.7;
const drawLegend = (y) => { if (byVal) { const w = Math.min(B.w * 0.42, 360 * (fs / 20)), x = B.x + (B.w - w) / 2, id = 'ramp', lgd = el(defs, 'linearGradient', { id, x1: 0, y1: 0, x2: 1, y2: 0 }); ramp.forEach((c, i) => el(lgd, 'stop', { offset: ramp.length > 1 ? i / (ramp.length - 1) : 0, 'stop-color': c })); el(g, 'rect', { x: r2(x), y: r2(y), width: r2(w), height: r2(small * 0.7), rx: r2(small * 0.2), fill: 'url(#ramp)' }); T(g, x - small * 0.5, y + small * 0.35 + mid(small), fmt(lo), { size: small, fill: muted, anchor: 'end' }); T(g, x + w + small * 0.5, y + small * 0.35 + mid(small), fmt(hi), { size: small, fill: muted }); } else legend(g, items, B.x + B.w / 2, y, B.w, small); };
if (lg === 'top') { drawLegend(B.y); ay += lgH + fs * 0.5; ah -= lgH + fs * 0.5; } else if (lg !== 'none') { drawLegend(B.y + B.h - lgH + small * 0.3); ah -= lgH + fs * 0.6; }
const nc = Math.max.apply(null, cells.map((c) => c.c)) + 1, nr = Math.max.apply(null, cells.map((c) => c.r)) + 1, hexS = shape === 'hex';
const cw = Math.min(B.w / (nc + (hexS ? 0.5 : 0)), ah / (hexS ? (nr - 1) * 0.866 + 1.1547 : nr)), ph = hexS ? cw * 0.866 : cw, gw = cw * (nc + (hexS ? 0.5 : 0)), gh = hexS ? ph * (nr - 1) + cw * 1.1547 : cw * nr;
const gx = B.x + (B.w - gw) / 2, gy = ay + (ah - gh) / 2, s = cw * (1 - gap), cs = Math.min(fs, s * (showVals ? 0.3 : 0.36));
for (const c of cells) {
  const x = gx + c.c * cw + (hexS && c.r % 2 ? cw / 2 : 0) + cw / 2, y = gy + c.r * ph + (hexS ? cw * 0.577 : cw / 2), f0 = colorOf(c.code), clear = /^(transparent|none)$/i.test(f0), f = clear ? 'none' : f0;
  if (shape === 'circle') el(g, 'circle', { cx: r2(x), cy: r2(y), r: r2(s / 2), fill: f });
  else if (hexS) el(g, 'polygon', { points: [0, 60, 120, 180, 240, 300].map((d) => pt(x, y, (s / 2) * 1.1547, d).map(r2).join(',')).join(' '), fill: f });
  else el(g, 'rect', { x: r2(x - s / 2), y: r2(y - s / 2), width: r2(s), height: r2(s), rx: r2(s * 0.1), fill: f });
  const tc = clear ? ink : onc(f), v = values[c.code.toUpperCase()];
  if (showCodes) T(g, x, y + mid(cs) - (showVals && v !== undefined ? cs * 0.55 : 0), c.code, { size: cs, weight: 700, anchor: 'middle', fill: tc });
  if (showVals && v !== undefined) T(g, x, y + mid(cs) + cs * 0.62, fmt(v), { size: cs * 0.92, anchor: 'middle', fill: tc, font: numfont });
  A(c.code, c.code, x, y, byVal ? { value: v } : { group: groups[c.code.toUpperCase()] || 0 });
}
done('tilemap', { grid: [gx, gy, gw, gh].map(Math.round), tile: r1(cw), cols: nc, rows: nr });
