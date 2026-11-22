// grunge.js: specks, scratches, and blotches on a transparent canvas: a distress / dirt overlay.
// Run:   pixelox render js scripts/grunge.js -o grunge.png --size 1080x1350 --seed 9
// Place it over a design (normal for dark dirt, screen for white wear) or clip it to type
// in the BACKGROUND color to make the letters look worn.
// Params (all optional):
//   color=#000000  ink color of the marks
//   specks=5000    tiny dots
//   blotches=60    larger irregular stains
//   scratches=30   thin long lines
//   clump=0.6      0 = even coverage, 1 = strongly clustered (leaves clean areas)
//   size=1         scale every mark (2 = coarser)
//   alpha=0.9      opacity of the heaviest marks
const W = px.width, H = px.height, P = px.params, S = Math.min(W, H) / 1080;
const num = (k, d) => (P[k] === undefined || P[k] === '' ? d : parseFloat(P[k]));
const color = P.color || '#000000', specks = num('specks', 5000), blotches = num('blotches', 60);
const scratches = num('scratches', 30), clump = num('clump', 0.6), size = num('size', 1) * S, alpha = num('alpha', 0.9);
const c = px.canvas(W, H);
c.fillStyle = color; c.strokeStyle = color;
// a coarse random field decides where marks cluster
const G = 12, field = [];
for (let i = 0; i < (G + 1) * (G + 1); i++) field.push(px.random());
function dens(x, y) {
  const u = (x / W) * G, v = (y / H) * G, i = Math.floor(u), j = Math.floor(v), a = u - i, b = v - j;
  const f = (ii, jj) => field[Math.min(G, jj) * (G + 1) + Math.min(G, ii)];
  const t = f(i, j) * (1 - a) * (1 - b) + f(i + 1, j) * a * (1 - b) + f(i, j + 1) * (1 - a) * b + f(i + 1, j + 1) * a * b;
  return 1 - clump + clump * Math.max(0, Math.min(1, (t - 0.35) * 3));
}
for (let i = 0; i < specks; i++) {
  const x = px.random() * W, y = px.random() * H, k = dens(x, y);
  if (px.random() > k) continue;
  c.globalAlpha = alpha * (0.3 + 0.7 * px.random());
  const r = size * (0.5 + 2.2 * px.random() * px.random());
  c.beginPath(); c.arc(x, y, r, 0, 6.2832); c.fill();
}
for (let i = 0; i < blotches; i++) {
  const x = px.random() * W, y = px.random() * H;
  if (px.random() > dens(x, y)) continue;
  const R = size * (6 + 30 * px.random() * px.random()), n = 6 + Math.floor(px.random() * 10);
  c.globalAlpha = alpha * (0.25 + 0.5 * px.random());
  for (let j = 0; j < n; j++) { // a blotch is a cluster of overlapping dots
    const a = px.random() * 6.2832, d = R * px.random();
    c.beginPath(); c.arc(x + Math.cos(a) * d, y + Math.sin(a) * d, R * (0.15 + 0.45 * px.random()), 0, 6.2832); c.fill();
  }
}
for (let i = 0; i < scratches; i++) {
  const x = px.random() * W, y = px.random() * H, a = (px.random() - 0.5) * 1.2 + (px.random() < 0.5 ? 0 : 1.5708);
  const L = size * (40 + 260 * px.random());
  c.globalAlpha = alpha * (0.2 + 0.5 * px.random()); c.lineWidth = size * (0.6 + 1.2 * px.random());
  c.beginPath(); c.moveTo(x, y);
  c.quadraticCurveTo(x + Math.cos(a) * L * 0.5 + (px.random() - 0.5) * 20 * size, y + Math.sin(a) * L * 0.5, x + Math.cos(a) * L, y + Math.sin(a) * L);
  c.stroke();
}
px.output(c);
