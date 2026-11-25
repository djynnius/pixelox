const vals = (rows('values', '52,80,31,60')[0] || []).map((v) => Math.max(0, v)), labels = list('labels'), n = vals.length, cyl = /^c/i.test(str('shape', 'box'));
const depth = num('depth', 1), gap = num('gap', 0.7), vmax = num('max', 0) > 0 ? num('max', 0) : Math.max.apply(null, vals.concat([1])), floorc = str('floor', '#e2e6ea'), showV = on('vlabels', 1), lt = num('light', 0.22), dk = num('dark', 0.28);
const g = el(svg, 'g', {}), small = fs * 0.9, C30 = Math.cos(Math.PI / 6), S30 = 0.5;
// unit space: blocks of width 1 along x, depth 'depth' along z, height up to hmax; iso: sx = (x - z) * C30, sy = (x + z) * S30 - y
const span = n + (n - 1) * gap, pad = 0.35, top = showV ? fs * 1.7 : fs * 0.4, bot = labels.length ? small * 2.1 : fs * 0.4;
const isoW = (span + 2 * pad + depth + 2 * pad) * C30, baseH = (span + 2 * pad + depth + 2 * pad) * S30;
const u = Math.min(B.w / isoW, (B.h - top - bot) / (baseH + Math.max(1.2, span * 0.55))), hmax = (B.h - top - bot - baseH * u) / u;
const ox = B.x + (B.w - isoW * u) / 2 + (depth + 2 * pad) * C30 * u, oy = B.y + top + hmax * u;
const Pj = (x, y, z) => [ox + (x - z) * C30 * u, oy + (x + z) * S30 * u - y * u], poly = (pts, fill) => el(g, 'polygon', { points: pts.map((p) => r2(p[0]) + ',' + r2(p[1])).join(' '), fill, stroke: fill, 'stroke-width': 0.6, 'stroke-linejoin': 'round' });
if (floorc !== 'none') poly([Pj(-pad, 0, -pad), Pj(span + pad, 0, -pad), Pj(span + pad, 0, depth + pad), Pj(-pad, 0, depth + pad)], floorc);
for (let i = 0; i < n; i++) { const x0 = i * (1 + gap), x1 = x0 + 1, h = (vals[i] / vmax) * hmax, c = COLORS.length > 1 ? col(i) : col(0), cT = mix(c, '#ffffff', lt), cR = mix(c, '#000000', dk);
  if (cyl) { const cb = Pj(x0 + 0.5, 0, depth / 2), ct = Pj(x0 + 0.5, h, depth / 2), rx = 0.5 * 1.2247 * u * Math.min(1, (depth + 1) / 2), ry = rx * 0.5774, gid = 'cy' + i, gr = el(defs, 'linearGradient', { id: gid, x1: 0, y1: 0, x2: 1, y2: 0 }); el(gr, 'stop', { offset: 0, 'stop-color': mix(c, '#ffffff', lt * 0.4) }); el(gr, 'stop', { offset: 0.45, 'stop-color': c }); el(gr, 'stop', { offset: 1, 'stop-color': cR });
    el(g, 'path', { d: 'M ' + r2(cb[0] - rx) + ' ' + r2(ct[1]) + ' L ' + r2(cb[0] - rx) + ' ' + r2(cb[1]) + ' A ' + r2(rx) + ' ' + r2(ry) + ' 0 0 0 ' + r2(cb[0] + rx) + ' ' + r2(cb[1]) + ' L ' + r2(cb[0] + rx) + ' ' + r2(ct[1]) + ' Z', fill: 'url(#' + gid + ')' }); el(g, 'ellipse', { cx: r2(ct[0]), cy: r2(ct[1]), rx: r2(rx), ry: r2(ry), fill: cT }); }
  else { poly([Pj(x0, 0, depth), Pj(x1, 0, depth), Pj(x1, h, depth), Pj(x0, h, depth)], c); poly([Pj(x1, 0, 0), Pj(x1, 0, depth), Pj(x1, h, depth), Pj(x1, h, 0)], cR); poly([Pj(x0, h, 0), Pj(x1, h, 0), Pj(x1, h, depth), Pj(x0, h, depth)], cT); }
  const tp = Pj(x0 + 0.5, h, depth / 2), bp = Pj(x0 + 0.5, 0, depth / 2), fp = Pj(x0 + 0.5, 0, depth + pad), topY = cyl ? tp[1] - 0.36 * u * Math.min(1, (depth + 1) / 2) : Pj(x0, h, 0)[1];
  if (showV) T(g, tp[0], topY - fs * 0.45, fmt(vals[i]), { weight: numwt, font: numfont, anchor: 'middle' });
  if (labels[i]) T(g, fp[0] - 0.25 * u * C30, fp[1] + small * 1.25, labels[i], { size: small, fill: muted, anchor: 'middle' });
  A('block' + (i + 1), labels[i] || '', tp[0], tp[1], { value: vals[i], bx: Math.round(bp[0]), by: Math.round(bp[1]) }); }
done('bars3d', { unit: r1(u), max: vmax });
