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
