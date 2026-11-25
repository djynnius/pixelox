const labels = list('labels', 'Series A|Series B'), dir = str('dir', 'row').toLowerCase(), shape = str('shape', 'square').toLowerCase(), bold = on('bold', 0), g = el(svg, 'g', {});
const key = (x, yc, c) => { const s = fs * 0.8; if (shape === 'circle') el(g, 'circle', { cx: r2(x + s / 2), cy: r2(yc), r: r2(s / 2), fill: c }); else if (shape === 'line') el(g, 'rect', { x: r2(x), y: r2(yc - s * 0.13), width: r2(s * 1.4), height: r2(s * 0.26), rx: r2(s * 0.13), fill: c }); else el(g, 'rect', { x: r2(x), y: r2(yc - s / 2), width: r2(s), height: r2(s), rx: r2(s * 0.2), fill: c }); return shape === 'line' ? s * 1.4 : s; };
if (dir.charAt(0) === 'c') labels.forEach((s, i) => { const yc = B.y + fs * 0.7 + i * fs * 1.75, w = key(B.x, yc, col(i)); T(g, B.x + w + fs * 0.5, yc + mid(fs), s, { fill: muted, weight: bold ? 700 : wt }); A('key' + (i + 1), s, B.x, yc); });
else { const gap = fs * 1.4, kw = fs * (shape === 'line' ? 1.12 : 0.8) + fs * 0.5, lines = [[]]; let cur = 0;
  labels.forEach((s, i) => { const w = kw + tw(s, fs, bold) + gap; if (cur + w > B.w + gap && lines[lines.length - 1].length) { lines.push([]); cur = 0; } lines[lines.length - 1].push([s, i, w]); cur += w; });
  const lh = fs * 1.75, y0 = B.y + (B.h - lines.length * lh) / 2 + lh / 2;
  lines.forEach((ln, r) => { let x = B.x + B.w / 2 - (ln.reduce((a, q) => a + q[2], 0) - gap) / 2; for (const q of ln) { const yc = y0 + r * lh, w = key(x, yc, col(q[1])); T(g, x + w + fs * 0.5, yc + mid(fs), q[0], { fill: muted, weight: bold ? 700 : wt }); A('key' + (q[1] + 1), q[0], x, yc); x += q[2]; } }); }
done('legend', {});
