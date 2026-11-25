const labels = list('labels', str('notes', '') ? '' : 'Research|Design|Build|Launch'), notes = list('notes'), n = Math.max(labels.length, notes.length, 1);
const bp = str('badges', ''), badges = /^(none|0|off)$/i.test(bp) ? [] : bp ? split(bp) : Array.from({ length: n }, (_, i) => (i < 9 ? '0' : '') + (i + 1));
const vert = /^v/i.test(str('dir', 'h')), alt = /^a/i.test(str('side', 'one')), style = str('style', 'circle').toLowerCase(), conn = str('connector', 'line').toLowerCase();
const linec = str('line', '#c3cad2'), nodefill = str('nodefill', '#ffffff'), ts = num('titlesize', 0) || fs * 1.15, one = on('onecolor', 0), C = (i) => (one ? col(0) : col(i));
const g = el(svg, 'g', {}), u = Math.max(0.6, Math.min(B.w, B.h) / 500), dot = style === 'dot';
const measure = (i, maxw) => { const tl = labels[i] ? wrap(labels[i], maxw, ts, 1) : [], nl = []; if (notes[i]) for (const para of notes[i].split('\\n')) for (const ln of wrap(para, maxw, fs)) nl.push(ln); return { tl, nl, h: tl.length * ts * 1.18 + nl.length * fs * 1.3 + (tl.length && nl.length ? fs * 0.3 : 0) }; };
const block = (i, x, y, maxw, anchor, m) => { m = m || measure(i, maxw); let yy = y; for (const ln of m.tl) { T(g, x, yy + ts * 0.82, ln, { size: ts, weight: 700, anchor }); yy += ts * 1.18; } if (m.tl.length && m.nl.length) yy += fs * 0.3; for (const ln of m.nl) { T(g, x, yy + fs * 0.82, ln, { anchor, fill: muted }); yy += fs * 1.3; } return m.h; };
const node = (i, x, y, D) => { const c = C(i), b = badges[i] || '';
  if (dot) { el(g, 'circle', { cx: r2(x), cy: r2(y), r: r2(D / 2), fill: nodefill, stroke: c, 'stroke-width': r2(D * 0.26) }); }
  else if (style === 'ring') el(g, 'circle', { cx: r2(x), cy: r2(y), r: r2(D / 2 - D * 0.045), fill: nodefill, stroke: c, 'stroke-width': r2(D * 0.09) });
  else el(g, 'circle', { cx: r2(x), cy: r2(y), r: r2(D / 2), fill: c });
  if (b && !dot) { const bs = Math.min(D * 0.38, (D * 0.74) / Math.max(1, tw(b, 1, 1))); T(g, x, y + mid(bs), b, { size: bs, weight: numwt, font: numfont, anchor: 'middle', fill: style === 'ring' ? c : onc(c) }); }
  A('node' + (i + 1), labels[i] || '', x, y, { d: Math.round(D) }); };
const link = (xa, ya, xb, yb) => { if (conn === 'none') return; el(g, 'line', Object.assign({ x1: r2(xa), y1: r2(ya), x2: r2(xb), y2: r2(yb), stroke: linec, 'stroke-width': r2(num('linewidth', 0) || 2.6 * u), 'stroke-linecap': 'round' }, conn === 'dashed' ? { 'stroke-dasharray': r2(1 * u) + ' ' + r2(8 * u) } : {})); };
const arrow = (x, y, ang) => { const s = 9 * u; el(g, 'polygon', { points: [[s, 0], [-s * 0.7, -s * 0.8], [-s * 0.7, s * 0.8]].map((p) => r2(x + p[0] * Math.cos(ang) - p[1] * Math.sin(ang)) + ',' + r2(y + p[0] * Math.sin(ang) + p[1] * Math.cos(ang))).join(' '), fill: linec }); };
let D;
if (style === 'snake') { // nodes zig-zag down the page on a thick S-curved ribbon; text on the outer side of each node
  const band = B.h / n, off = B.w * num('swing', 0.13), cxs = B.x + B.w / 2; D = num('node', 0) || Math.max(fs * 2, Math.min(band * 0.62, B.w * 0.15));
  const Xn = (i) => cxs + (i % 2 ? off : -off), Yn = (i) => B.y + band * (i + 0.5), rw = num('linewidth', 0) || D * 0.42, padS = fs * 0.9, tw0 = B.w / 2 - off - D / 2 - padS;
  let d = 'M ' + r2(Xn(0)) + ' ' + r2(Yn(0)); for (let i = 0; i < n - 1; i++) { const ym = (Yn(i) + Yn(i + 1)) / 2; d += ' C ' + r2(Xn(i)) + ' ' + r2(ym) + ' ' + r2(Xn(i + 1)) + ' ' + r2(ym) + ' ' + r2(Xn(i + 1)) + ' ' + r2(Yn(i + 1)); }
  if (conn !== 'none') { el(g, 'path', { d, fill: 'none', stroke: linec, 'stroke-width': r2(rw), 'stroke-linecap': 'round', 'stroke-linejoin': 'round' }); if (conn === 'dashed') el(g, 'path', { d, fill: 'none', stroke: nodefill, 'stroke-width': r2(Math.max(1.5, rw * 0.1)), 'stroke-dasharray': r2(rw * 0.5) + ' ' + r2(rw * 0.45) }); }
  for (let i = 0; i < n; i++) { node(i, Xn(i), Yn(i), D); const right = i % 2 === 1, m = measure(i, tw0); block(i, right ? Xn(i) + D / 2 + padS : Xn(i) - D / 2 - padS, Yn(i) - m.h / 2, tw0, right ? 'start' : 'end', m); }
} else if (style === 'chevron' && !vert) {
  const notch0 = Math.min(B.h * 0.12, (B.w / n) * 0.2), bandc = (B.w - notch0) / n, chH = num('node', 0) || Math.min(bandc * 0.5, B.h * 0.36), notch = Math.min(notch0, chH * 0.42), gapc = 5 * u, y0 = B.y, y1 = B.y + chH, ym = (y0 + y1) / 2; D = chH;
  for (let i = 0; i < n; i++) { const x0 = B.x + i * bandc, x1 = x0 + bandc + notch - gapc, c = C(i), pts = [[x0, y0], [x1 - notch, y0], [x1, ym], [x1 - notch, y1], [x0, y1]]; if (i) pts.push([x0 + notch, ym]);
    el(g, 'polygon', { points: pts.map((p) => r2(p[0]) + ',' + r2(p[1])).join(' '), fill: c });
    const mx = (x0 + (i ? notch : 0) + x1 - notch) / 2, b = badges[i] || '', bs = Math.min(chH * 0.4, ((bandc - notch) * 0.8) / Math.max(1, tw(b, 1, 1)));
    if (b) T(g, mx, ym + mid(bs), b, { size: bs, weight: numwt, font: numfont, anchor: 'middle', fill: onc(c) });
    A('node' + (i + 1), labels[i] || '', mx, ym, { d: Math.round(chH) });
    block(i, x0 + (bandc + notch) / 2 - gapc, y1 + fs * 0.9, bandc * 0.86, 'middle'); }
} else if (!vert) {
  const band = B.w / n, tw0 = band * 0.88, ms = labels.map((_, i) => measure(i, tw0)).concat(n > labels.length ? Array.from({ length: n - labels.length }, (_, k) => measure(labels.length + k, tw0)) : []), mh = Math.max.apply(null, ms.map((m) => m.h));
  D = num('node', 0) || (dot ? fs * 0.95 : Math.max(fs * 2, Math.min(band * 0.5, alt ? (B.h - 2 * mh - fs * 2) : B.h - mh - fs * 1.2)));
  const bl = dot && badges.length ? ts * 1.5 : 0, cy = alt ? B.y + B.h / 2 : B.y + Math.max(0, (B.h - (bl + D + fs * 0.8 + mh)) / 2) + bl + D / 2, X = (i) => B.x + band * (i + 0.5);
  link(X(0), cy, X(n - 1), cy); if (conn === 'arrow') for (let i = 0; i < n - 1; i++) arrow((X(i) + X(i + 1)) / 2, cy, 0);
  for (let i = 0; i < n; i++) { node(i, X(i), cy, D); const up = alt && i % 2 === 1, m = ms[i];
    if (dot && badges[i]) T(g, X(i), up ? cy + D / 2 + ts * 1.25 : cy - D / 2 - ts * 0.5, badges[i], { size: ts * 1.1, weight: numwt, font: numfont, anchor: 'middle', fill: C(i) });
    block(i, X(i), up ? cy - D / 2 - fs * 0.8 - m.h : cy + D / 2 + fs * 0.8, tw0, 'middle', m); }
} else {
  const band = B.h / n; D = num('node', 0) || (dot ? fs * 0.95 : Math.max(fs * 2, Math.min(band * 0.66, B.w * 0.17)));
  const bw = dot && badges.length ? Math.max.apply(null, badges.map((b) => tw(b, ts * 1.1, 1))) + fs * 0.8 : 0, cx = alt ? B.x + B.w / 2 : B.x + bw + D / 2, Y = (i) => B.y + band * (i + 0.5), pad = fs * 0.9, tw0 = alt ? B.w / 2 - D / 2 - pad : B.w - bw - D - pad;
  link(cx, Y(0), cx, Y(n - 1)); if (conn === 'arrow') for (let i = 0; i < n - 1; i++) arrow(cx, (Y(i) + Y(i + 1)) / 2, Math.PI / 2);
  for (let i = 0; i < n; i++) { node(i, cx, Y(i), D); const left = alt && i % 2 === 1, m = measure(i, tw0);
    if (dot && badges[i]) T(g, left || !alt ? (alt ? cx + D / 2 + pad : cx - D / 2 - fs * 0.6) : cx - D / 2 - pad, Y(i) + mid(ts * 1.1), badges[i], { size: ts * 1.1, weight: numwt, font: numfont, anchor: left ? 'start' : 'end', fill: C(i) });
    block(i, left ? cx - D / 2 - pad : cx + D / 2 + pad, Y(i) - m.h / 2, tw0, left ? 'end' : 'start', m); }
}
done('steps', { node: Math.round(D), steps: n });
