const kind = str('kind', 'rocket').toLowerCase(), gauge = P.value !== undefined && P.value !== '', frac = Math.max(0, Math.min(1, num('value', 0) / (num('max', 100) || 1)));
const nb = gauge ? 1 : Math.max(1, Math.round(num('bands', 4))), bp = str('badges', ''), badges = gauge || /^(none|0|off)$/i.test(bp) ? [] : bp ? split(bp) : Array.from({ length: nb }, (_, i) => (i < 9 ? '0' : '') + (i + 1));
const BC = list('badgecolor'), rot = num('rotate', 0), shade = on('shade', 1), dark = str('dark', '#374151'), empty = str('empty', '#e2e6ea');
// unit geometry: x from -0.5 to 0.5, y from 0 (top) down; 'sil' is the area that takes the bands
const K = {
  rocket: { w: 1.04, h: 2.62, y0: 0, y1: 2.05, first: 0.78, sil: 'M 0 0 C 0.16 0.18 0.3 0.45 0.3 0.78 L 0.3 1.9 L 0.2 2.05 L -0.2 2.05 L -0.3 1.9 L -0.3 0.78 C -0.3 0.45 -0.16 0.18 0 0 Z', half: (y) => (y < 0.78 ? 0.3 * Math.sin((Math.max(0, y) / 0.78) * 1.5708) : y > 1.9 ? 0.3 - ((y - 1.9) / 0.15) * 0.1 : 0.3) },
  bulb: { w: 1.0, h: 1.5, y0: 0, y1: 1.08, sil: 'M -0.2 1.08 C -0.2 0.92 -0.5 0.8 -0.5 0.5 A 0.5 0.5 0 1 1 0.5 0.5 C 0.5 0.8 0.2 0.92 0.2 1.08 Z', half: (y) => (y <= 0.78 ? Math.sqrt(Math.max(0, 0.25 - (y - 0.5) * (y - 0.5))) : 0.42 - ((y - 0.78) / 0.3) * 0.22) },
  pencil: { w: 0.5, h: 3.2, y0: 0.5, y1: 2.5, sil: 'M -0.25 0.5 L 0.25 0.5 L 0.25 2.5 L -0.25 2.5 Z', half: () => 0.25 },
  thermometer: { w: 0.8, h: 3.0, y0: 0.12, y1: 2.9, sil: 'M -0.15 0.15 A 0.15 0.15 0 0 1 0.15 0.15 L 0.15 2.2 A 0.36 0.36 0 1 1 -0.15 2.2 Z', half: (y) => (y < 2.2 ? 0.15 : 0.36) },
  battery: { w: 1.0, h: 1.9, y0: 0.26, y1: 1.8, sil: 'M -0.4 0.26 L 0.4 0.26 L 0.4 1.8 L -0.4 1.8 Z', half: () => 0.4 },
}[kind] || null;
if (!K) throw new Error('kind must be rocket, bulb, pencil, thermometer, or battery');
const asp = Math.max(0.5, Math.min(2.5, num('aspect', 1))), outline = on('outline', 0); K.w *= asp; const half0 = K.half; K.half = (y) => half0(y) * asp;
const rad = (rot * Math.PI) / 180, cs = Math.abs(Math.cos(rad)), sn = Math.abs(Math.sin(rad)), bw = K.w * cs + K.h * sn, bh = K.w * sn + K.h * cs, s = Math.min(B.w / bw, B.h / bh), cx = B.x + B.w / 2, cy = B.y + B.h / 2;
// unit point -> canvas point
const X = (x, y) => { const px0 = x * s, py0 = (y - K.h / 2) * s; return [cx + px0 * Math.cos(rad) - py0 * Math.sin(rad), cy + px0 * Math.sin(rad) + py0 * Math.cos(rad)]; };
const g = el(svg, 'g', { transform: 'translate(' + r2(cx) + ' ' + r2(cy) + ') rotate(' + rot + ') scale(' + r2(s * asp) + ' ' + r2(s) + ') translate(0 ' + r2(-K.h / 2) + ')' });
const P_ = (d, fill, extra) => el(g, 'path', Object.assign({ d, fill }, extra || {}));
const c0 = col(0), lastc = col(nb - 1);
// parts behind the banded body
if (kind === 'rocket') { const fc = str('fincolor', mix(col(Math.min(1, nb - 1)), '#000000', 0.18)); P_('M -0.3 1.3 C -0.5 1.5 -0.54 1.85 -0.5 2.14 L -0.29 1.92 Z', fc); P_('M 0.3 1.3 C 0.5 1.5 0.54 1.85 0.5 2.14 L 0.29 1.92 Z', fc);
  if (on('flame', 1)) { P_('M -0.17 2.15 C -0.22 2.38 -0.07 2.5 0 2.62 C 0.07 2.5 0.22 2.38 0.17 2.15 Z', '#f97316'); P_('M -0.09 2.15 C -0.12 2.3 -0.04 2.38 0 2.46 C 0.04 2.38 0.12 2.3 0.09 2.15 Z', '#fde047'); }
  P_('M -0.2 2.04 L 0.2 2.04 L 0.16 2.17 L -0.16 2.17 Z', dark); }
if (kind === 'bulb' && on('rays', 0)) for (let k = -3; k <= 3; k++) { const a = k * 32, p0 = pt(0, 0.5, 0.58, a), p1 = pt(0, 0.5, 0.72, a); el(g, 'line', { x1: r2(p0[0]), y1: r2(p0[1]), x2: r2(p1[0]), y2: r2(p1[1]), stroke: c0, 'stroke-width': 0.035, 'stroke-linecap': 'round' }); }
// the banded (or filled) body
const cp = el(defs, 'clipPath', { id: 'sil' }); el(cp, 'path', { d: K.sil });
const body = el(g, 'g', { 'clip-path': 'url(#sil)' }), span = K.y1 - K.y0, edges = [K.y0];
for (let i = 1; i <= nb; i++) edges.push(K.first && nb > 1 ? (i === 1 ? K.first : K.first + ((K.y1 - K.first) * (i - 1)) / (nb - 1)) : K.y0 + (span * i) / nb);
if (gauge) { el(body, 'rect', { x: -0.6, y: K.y0 - 0.2, width: 1.2, height: span + 0.4, fill: empty }); const yf = K.y1 - span * frac; el(body, 'rect', { x: -0.6, y: r2(yf), width: 1.2, height: r2(K.y1 - yf + 0.2), fill: c0 }); const p = X(0, yf), l = X(-K.half(yf), yf), r = X(K.half(yf), yf); A('level', 'fill level', p[0], p[1], { lx: Math.round(l[0]), ly: Math.round(l[1]), rx: Math.round(r[0]), ry: Math.round(r[1]), value: num('value', 0) }); }
else for (let i = 0; i < nb; i++) el(body, 'rect', { x: -0.6, y: r2(edges[i] - (i ? 0 : 0.2)), width: 1.2, height: r2(edges[i + 1] - edges[i] + (i ? 0 : 0.2) + (i === nb - 1 ? 0.2 : 0.004)), fill: col(i) });
if (outline) { while (body.firstChild) body.removeChild(body.firstChild); P_(K.sil, str('glass', 'none'), { stroke: c0, 'stroke-width': r2(num('outlinewidth', 0.055) / Math.sqrt(asp)), 'stroke-linejoin': 'round' }); }
if (shade && !outline) { const hm = half0((K.y0 + K.y1) / 2); el(body, 'rect', { x: -0.5, y: K.y0 - 0.2, width: 0.5 - hm * 0.45, height: span + 0.4, fill: '#ffffff', 'fill-opacity': 0.16 }); el(body, 'rect', { x: r2(hm * 0.62), y: K.y0 - 0.2, width: 0.5, height: span + 0.4, fill: '#000000', 'fill-opacity': 0.1 }); }
if (!gauge && nb > 1 && (kind === 'rocket' || kind === 'pencil')) for (let i = 1; i < nb; i++) el(body, 'rect', { x: -0.6, y: r2(edges[i] - 0.012), width: 1.2, height: 0.024, fill: '#000000', 'fill-opacity': 0.12 });
// parts in front
if (kind === 'bulb') { [1.1, 1.2, 1.3].forEach((y, k) => el(g, 'rect', { x: r2(-0.2 + k * 0.012), y, width: r2(0.4 - k * 0.024), height: 0.085, rx: 0.04, fill: dark })); el(g, 'rect', { x: -0.09, y: 1.4, width: 0.18, height: 0.08, rx: 0.04, fill: dark }); if (shade) P_('M -0.33 0.42 A 0.36 0.36 0 0 1 -0.12 0.17', 'none', { stroke: '#ffffff', 'stroke-opacity': 0.7, 'stroke-width': 0.05, 'stroke-linecap': 'round' }); }
if (kind === 'pencil') { el(g, 'rect', { x: -0.25, y: 0, width: 0.5, height: 0.32, rx: 0.1, fill: '#f4a3a8' }); el(g, 'rect', { x: -0.25, y: 0.24, width: 0.5, height: 0.27, fill: '#b8c0cc' }); el(g, 'rect', { x: -0.25, y: 0.31, width: 0.5, height: 0.03, fill: '#8892a0' }); el(g, 'rect', { x: -0.25, y: 0.41, width: 0.5, height: 0.03, fill: '#8892a0' }); P_('M -0.25 2.5 L 0.25 2.5 L 0 3.2 Z', '#f1d2a4'); P_('M -0.082 2.97 L 0.082 2.97 L 0 3.2 Z', dark); }
if (kind === 'thermometer') { P_(K.sil, 'none', { stroke: dark, 'stroke-width': 0.05 }); for (let k = 0; k <= 4; k++) el(g, 'line', { x1: 0.24, x2: 0.36, y1: r2(0.3 + k * 0.42), y2: r2(0.3 + k * 0.42), stroke: dark, 'stroke-width': 0.035, 'stroke-linecap': 'round' }); }
if (kind === 'battery') { el(g, 'rect', { x: -0.47, y: 0.19, width: 0.94, height: 1.68, rx: 0.12, fill: 'none', stroke: dark, 'stroke-width': 0.07 }); el(g, 'rect', { x: -0.18, y: 0.02, width: 0.36, height: 0.15, rx: 0.05, fill: dark }); if (!gauge) for (let i = 1; i < nb; i++) el(g, 'rect', { x: -0.42, y: r2(edges[i] - 0.03), width: 0.84, height: 0.06, fill: bg === 'transparent' ? '#ffffff' : bg }); }
if (kind === 'rocket' && on('window', 0)) { const wy = nb > 1 ? (edges[1] + edges[2]) / 2 : 1.0, wr = num('windowsize', 0.15); el(g, 'ellipse', { cx: 0, cy: r2(wy), rx: r2(wr / asp), ry: r2(wr), fill: str('windowring', '#e5edf5') }); el(g, 'ellipse', { cx: 0, cy: r2(wy), rx: r2((wr * 0.67) / asp), ry: r2(wr * 0.67), fill: str('windowglass', '#7cc4f2') }); }
// badges and anchors (drawn unrotated so the numbers stay upright)
for (let i = 0; i < nb && !gauge; i++) { const ym = kind === 'rocket' && i === 0 && nb > 1 ? edges[1] * 0.64 : (edges[i] + edges[i + 1]) / 2, hw = K.half(ym), p = X(0, ym), l = X(-hw, ym), r = X(hw, ym), b = badges[i] || '';
  A('band' + (i + 1), b, p[0], p[1], { lx: Math.round(l[0]), ly: Math.round(l[1]), rx: Math.round(r[0]), ry: Math.round(r[1]) });
  if (b && !outline && !(kind === 'rocket' && on('window', 0) && i === 1)) { const bs = Math.min((edges[i + 1] - edges[i]) * s * 0.5, (hw * 2 * s * 0.7) / Math.max(1, tw(b, 1, 1))); T(svg, p[0], p[1] + mid(bs), b, { size: bs, weight: numwt, font: numfont, anchor: 'middle', fill: BC.length ? BC[i % BC.length] : onc(col(i)) }); } }
done('object', { kind, scale: r2(s), rotate: rot });
