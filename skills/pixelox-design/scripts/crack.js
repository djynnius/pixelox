// crack.js: one jagged crack (a thin lightning-shaped gap) on a transparent canvas.
// Run:   pixelox render js scripts/crack.js -o crack.png --size 1080x1350 --seed 5 --param color='#f1ede4'
// Draw it in the GROUND color over a word, a face, or a shape so the thing looks split;
// in a light color with a glow it is lightning.
// Params (all optional):
//   color=#f1ede4
//   from=0.5,0      start point in 0..1 canvas fractions
//   to=0.48,1       end point
//   width=0.018     widest gap as a fraction of the short side
//   jag=0.5         0..1 how far the path zigzags sideways
//   steps=14        number of kinks
//   branches=3      short side cracks
//   taper=0.6       0 = even width, 1 = needle-thin ends
const W = px.width, H = px.height, P = px.params, S = Math.min(W, H);
const num = (k, d) => (P[k] === undefined || P[k] === '' ? d : parseFloat(P[k]));
const pt = (s, d) => (s ? s.split(',').map(parseFloat) : d);
const A = pt(P.from, [0.5, 0]), B = pt(P.to, [0.48, 1]), wmax = num('width', 0.018) * S, jag = num('jag', 0.5);
const steps = Math.round(num('steps', 14)), branches = Math.round(num('branches', 3)), taper = num('taper', 0.6), c = px.canvas(W, H);
c.fillStyle = P.color || '#f1ede4';
function crack(x0, y0, x1, y1, n, w) {
  const dx = x1 - x0, dy = y1 - y0, len = Math.hypot(dx, dy), nx = -dy / len, ny = dx / len, mid = [], pts = [];
  for (let i = 0; i <= n; i++) {
    const t = i / n, off = i === 0 || i === n ? 0 : (px.random() - 0.5) * jag * len / n * 2.2;
    mid.push([x0 + dx * t + nx * off, y0 + dy * t + ny * off, t]);
  }
  const L = [], R = [];
  for (const m of mid) { const k = 1 - taper * Math.pow(Math.abs(m[2] - 0.5) * 2, 1.5), ww = w * k * (0.6 + 0.8 * px.random()) / 2; L.push([m[0] + nx * ww, m[1] + ny * ww]); R.push([m[0] - nx * ww, m[1] - ny * ww]); }
  c.beginPath(); L.forEach((p, i) => (i ? c.lineTo(p[0], p[1]) : c.moveTo(p[0], p[1]))); for (let i = R.length - 1; i >= 0; i--) c.lineTo(R[i][0], R[i][1]); c.closePath(); c.fill();
  return mid;
}
const main = crack(A[0] * W, A[1] * H, B[0] * W, B[1] * H, steps, wmax);
for (let b = 0; b < branches; b++) {
  const m = main[2 + Math.floor(px.random() * (main.length - 4))], a = Math.atan2(B[1] * H - A[1] * H, B[0] * W - A[0] * W) + (px.random() < 0.5 ? 1 : -1) * (0.5 + 0.6 * px.random());
  const L = S * (0.06 + 0.12 * px.random());
  crack(m[0], m[1], m[0] + Math.cos(a) * L, m[1] + Math.sin(a) * L, 5, wmax * 0.45);
}
px.output(c);
