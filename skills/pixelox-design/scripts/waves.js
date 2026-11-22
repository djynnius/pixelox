// waves.js: stacked flowing wave bands or contour lines on a transparent canvas.
// Run:   pixelox render js scripts/waves.js -o waves.png --size 1080x1350 --seed 8 --param colors='#0b2a6f,#1557c0,#3aa0ff'
// Params (all optional):
//   colors=#0b2a6f,#1557c0,#3aa0ff  comma list, cycled from the first band down
//   count=5       number of bands or lines
//   mode=fill     fill = solid bands stacked to the bottom; line = stroked contour lines
//   top=0.45      where the first band starts (0..1 from the top)
//   bottom=1      where the last band starts
//   amp=0.06      wave height as a fraction of the canvas height
//   freq=1.2      waves across the width
//   chaos=0.5     0..1 how much each band differs from the one above
//   width=3       line width in px (mode=line)
//   alpha=1
const W = px.width, H = px.height, P = px.params;
const num = (k, d) => (P[k] === undefined || P[k] === '' ? d : parseFloat(P[k]));
const colors = (P.colors || '#0b2a6f,#1557c0,#3aa0ff').split(',').map((s) => s.trim());
const count = Math.round(num('count', 5)), mode = P.mode || 'fill', top = num('top', 0.45), bottom = num('bottom', 1);
const amp = num('amp', 0.06) * H, freq = num('freq', 1.2), chaos = num('chaos', 0.5), c = px.canvas(W, H);
c.globalAlpha = num('alpha', 1); c.lineWidth = num('width', 3); c.lineJoin = 'round';
let ph1 = px.random() * 6.28, ph2 = px.random() * 6.28, f2 = 2.3 + px.random();
for (let i = 0; i < count; i++) {
  const y0 = H * (top + ((bottom - top) * i) / Math.max(1, count - (mode === 'fill' ? 0 : 1)));
  ph1 += chaos * (0.6 + px.random() * 1.6); ph2 += chaos * (px.random() * 2.4);
  const a2 = amp * (0.25 + 0.3 * chaos * px.random());
  c.beginPath();
  for (let x = -8; x <= W + 8; x += 8) {
    const u = (x / W) * 6.2832;
    const y = y0 + amp * Math.sin(u * freq + ph1) + a2 * Math.sin(u * freq * f2 + ph2);
    if (x === -8) c.moveTo(x, y); else c.lineTo(x, y);
  }
  const col = colors[i % colors.length];
  if (mode === 'line') { c.strokeStyle = col; c.stroke(); }
  else { c.lineTo(W + 8, H + 8); c.lineTo(-8, H + 8); c.closePath(); c.fillStyle = col; c.fill(); }
}
px.output(c);
