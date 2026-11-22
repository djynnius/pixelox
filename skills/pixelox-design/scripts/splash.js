// splash.js: a liquid splash (water, cola, milk, juice) on a transparent canvas: curved tongues
// thrown from a center, a crown at the base, flying drops, and fine spray.
// Run:   pixelox render js scripts/splash.js -o splash.png --size 1080x1350 --seed 3 --param color='#9fd4ff'
// Put it behind (and a second, smaller one in front of) the product; `screen` mode on dark
// grounds, normal mode on light ones. Blur a duplicate for glow.
// Params (all optional):
//   color=#9fd4ff   liquid tint (water: #9fd4ff, cola: #5a2a12, milk: #ffffff, orange: #ff9a1f)
//   shine=#ffffff   highlight color
//   cx=0.5 cy=0.6   center in 0..1
//   radius=0.42     reach of the longest tongues as a fraction of the short side
//   tongues=16      thrown arcs of liquid
//   drops=260       flying drops
//   dir=-90         main throw direction in degrees (-90 = up, 0 = right)
//   arc=220         how wide the splash fans out, in degrees (360 = all around)
//   alpha=0.75      body opacity (highlights stay brighter)
const W = px.width, H = px.height, P = px.params, S = Math.min(W, H);
const num = (k, d) => (P[k] === undefined || P[k] === '' ? d : parseFloat(P[k]));
const cx = num('cx', 0.5) * W, cy = num('cy', 0.6) * H, R = num('radius', 0.42) * S;
const tongues = Math.round(num('tongues', 16)), drops = Math.round(num('drops', 260));
const dir = (num('dir', -90) * Math.PI) / 180, arc = (num('arc', 220) * Math.PI) / 180, alpha = num('alpha', 0.75);
const hex = (c) => { const m = /^#?([0-9a-f]{6})/i.exec(c), v = m ? parseInt(m[1], 16) : 0x9fd4ff; return [(v >> 16) & 255, (v >> 8) & 255, v & 255].join(','); };
const body = hex(P.color || '#9fd4ff'), shine = hex(P.shine || '#ffffff'), c = px.canvas(W, H);
const rnd = () => px.random(), ang = () => dir + (rnd() - 0.5) * arc;
function drop(x, y, r, a, stretch) { // an elongated drop with a small highlight
  c.save(); c.translate(x, y); c.rotate(a);
  c.fillStyle = 'rgba(' + body + ',' + alpha + ')'; c.beginPath(); c.ellipse(0, 0, r * stretch, r, 0, 0, 6.2832); c.fill();
  c.fillStyle = 'rgba(' + shine + ',0.85)'; c.beginPath(); c.ellipse(-r * 0.25 * stretch, -r * 0.3, r * 0.35 * stretch, r * 0.22, 0, 0, 6.2832); c.fill();
  c.restore();
}
// tongues: tapered curved ribbons ending in a drop
for (let i = 0; i < tongues; i++) {
  const a = ang(), L = R * (0.45 + 0.55 * rnd()), bend = (rnd() - 0.5) * 0.9, w0 = R * (0.05 + 0.07 * rnd());
  const N = 26, L_ = [], R_ = [];
  let tipx = cx, tipy = cy, tipa = a;
  for (let k = 0; k <= N; k++) {
    const t = k / N, aa = a + bend * t * t, d = L * t;
    const x = cx + Math.cos(aa) * d, y = cy + Math.sin(aa) * d + 0.18 * L * t * t; // slight fall
    const w = w0 * (1 - t) * (0.75 + 0.25 * Math.sin(t * 9 + i)) + 1.2, nx = -Math.sin(aa), ny = Math.cos(aa);
    L_.push([x + nx * w, y + ny * w]); R_.push([x - nx * w, y - ny * w]); tipx = x; tipy = y; tipa = aa;
  }
  const g = c.createLinearGradient(cx, cy, tipx, tipy);
  g.addColorStop(0, 'rgba(' + body + ',' + alpha * 0.55 + ')'); g.addColorStop(1, 'rgba(' + body + ',' + alpha + ')');
  c.fillStyle = g; c.beginPath(); L_.forEach((p, k) => (k ? c.lineTo(p[0], p[1]) : c.moveTo(p[0], p[1])));
  for (let k = N; k >= 0; k--) c.lineTo(R_[k][0], R_[k][1]); c.closePath(); c.fill();
  c.strokeStyle = 'rgba(' + shine + ',0.8)'; c.lineWidth = Math.max(1, w0 * 0.22); c.lineCap = 'round';
  c.beginPath(); for (let k = 2; k <= N - 3; k++) { const p = L_[k], q = R_[k], x = p[0] * 0.72 + q[0] * 0.28, y = p[1] * 0.72 + q[1] * 0.28; if (k === 2) c.moveTo(x, y); else c.lineTo(x, y); } c.stroke();
  drop(tipx, tipy, w0 * (0.45 + 0.5 * rnd()), tipa, 1.5);
}
// crown: a lumpy bright pool at the base
for (let i = 0; i < 26; i++) { const a = rnd() * 6.2832, d = R * 0.16 * Math.sqrt(rnd()); drop(cx + Math.cos(a) * d * 1.6, cy + Math.sin(a) * d * 0.7, R * (0.03 + 0.05 * rnd()), 0, 1.4); }
// flying drops, smaller the farther they go, then fine spray
for (let i = 0; i < drops; i++) { const a = ang(), f = rnd(), d = R * (0.25 + 1.05 * f); drop(cx + Math.cos(a) * d, cy + Math.sin(a) * d + 0.12 * R * f * f, R * 0.028 * (1.15 - f) * (0.3 + rnd()) + 0.8, a, 1 + 1.2 * rnd()); }
c.fillStyle = 'rgba(' + shine + ',0.7)';
for (let i = 0; i < drops * 2; i++) { const a = ang(), d = R * (0.3 + 1.2 * rnd()); c.beginPath(); c.arc(cx + Math.cos(a) * d, cy + Math.sin(a) * d, 0.6 + 1.6 * rnd() * rnd(), 0, 6.2832); c.fill(); }
px.output(c);
