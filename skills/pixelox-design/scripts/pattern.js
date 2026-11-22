// pattern.js: flat graphic patterns on a transparent canvas: dots, stripes, grid, rings, checker,
// zigzag, scallop (fish-scale arcs), and barcode.
// Run:   pixelox render js scripts/pattern.js -o dots.png --size 1080x1350 --param kind=dots --param fade=down
// Params (all optional):
//   kind=dots     dots | stripes | grid | rings | checker | zigzag | scallop | barcode (random bar widths; cell = average bar pitch)
//   color=#ffffff
//   cell=28       spacing in px
//   weight=0.35   mark size as a fraction of the cell (dot radius x2, stripe or line width)
//   angle=0       rotation in degrees (stripes: 45 = diagonal)
//   fade=none     none | down | up | left | right | out (dots and lines shrink to nothing that way)
//   cx=0.5 cy=0.5 center for rings and for fade=out
//   alpha=1
const W = px.width, H = px.height, P = px.params;
const num = (k, d) => (P[k] === undefined || P[k] === '' ? d : parseFloat(P[k]));
const kind = P.kind || 'dots', cell = num('cell', 28), weight = num('weight', 0.35), fade = P.fade || 'none';
const cx = num('cx', 0.5) * W, cy = num('cy', 0.5) * H, ang = (num('angle', 0) * Math.PI) / 180, c = px.canvas(W, H);
c.fillStyle = P.color || '#ffffff'; c.strokeStyle = c.fillStyle; c.globalAlpha = num('alpha', 1);
const D = Math.hypot(W, H);
function k(x, y) { // 1 = full size, 0 = gone
  if (fade === 'down') return 1 - y / H; if (fade === 'up') return y / H;
  if (fade === 'right') return 1 - x / W; if (fade === 'left') return x / W;
  if (fade === 'out') return Math.max(0, 1 - Math.hypot(x - cx, y - cy) / (Math.min(W, H) * 0.6));
  return 1;
}
if (kind === 'barcode') {
  let x = 0; while (x < W) { const w = cell * (0.15 + 0.7 * px.random()) * (weight / 0.35); c.fillRect(x, 0, w, H); x += w + cell * (0.2 + 0.6 * px.random()); }
} else if (kind === 'zigzag' || kind === 'scallop') {
  c.translate(W / 2, H / 2); c.rotate(ang); c.lineWidth = cell * weight * 0.5; c.lineJoin = 'miter';
  const n = Math.ceil(D / cell / 2) + 1;
  for (let j = -n; j <= n; j++) {
    c.beginPath();
    for (let i = -n; i <= n; i++) {
      const x = i * cell, y = j * cell;
      if (kind === 'zigzag') { if (i === -n) c.moveTo(x, y); c.lineTo(x + cell / 2, y - cell / 2); c.lineTo(x + cell, y); }
      else { const ox = (j & 1) * cell / 2; c.moveTo(x + ox + cell, y); c.arc(x + ox + cell / 2, y, cell / 2, 0, Math.PI, true); }
    }
    c.stroke();
  }
} else if (kind === 'rings') {
  for (let r = cell; r < D; r += cell) { const f = fade === 'out' ? Math.max(0, 1 - r / (Math.min(W, H) * 0.6)) : 1; if (f <= 0) break; c.lineWidth = cell * weight * f; c.beginPath(); c.arc(cx, cy, r, 0, 6.2832); c.stroke(); }
} else {
  c.translate(W / 2, H / 2); c.rotate(ang);
  const n = Math.ceil(D / cell / 2) + 1, co = Math.cos(ang), si = Math.sin(ang);
  const back = (x, y) => [W / 2 + x * co - y * si, H / 2 + x * si + y * co]; // rotated point in canvas space
  for (let j = -n; j <= n; j++) for (let i = -n; i <= n; i++) {
    const x = i * cell, y = j * cell, q = back(x, y);
    if (q[0] < -cell || q[0] > W + cell || q[1] < -cell || q[1] > H + cell) continue;
    const f = Math.max(0, Math.min(1, k(q[0], q[1])));
    if (f <= 0.02) continue;
    if (kind === 'dots') { c.beginPath(); c.arc(x, y, (cell * weight * f) / 1, 0, 6.2832); c.fill(); }
    else if (kind === 'checker') { if ((i + j) & 1) c.fillRect(x - cell / 2, y - cell / 2, cell, cell); }
    else if (kind === 'stripes') { const w = cell * weight * f; c.fillRect(x - w / 2, y - cell / 2 - 0.5, w, cell + 1); }
    else if (kind === 'grid') { const w = Math.max(1, cell * weight * 0.25 * f); c.fillRect(x - w / 2, y - cell / 2 - 0.5, w, cell + 1); c.fillRect(x - cell / 2 - 0.5, y - w / 2, cell + 1, w); }
  }
}
px.output(c);
