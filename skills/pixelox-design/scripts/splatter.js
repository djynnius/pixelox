// splatter.js: a paint or ink splat with flying droplets on a transparent canvas.
// Run:   pixelox render js scripts/splatter.js -o splat.png --size 1080x1080 --seed 6 --param color='#b3121a'
// Params (all optional):
//   color=#b3121a
//   cx=0.5 cy=0.5  center in 0..1
//   radius=0.22    core size as a fraction of the short side
//   drops=260      flying droplets
//   reach=2.2      how far droplets fly, in core radii
//   arms=9         long thrown streaks
//   dir=           degrees; when set the splat is thrown mostly that way (0 = right, 90 = down)
const W = px.width, H = px.height, P = px.params, S = Math.min(W, H);
const num = (k, d) => (P[k] === undefined || P[k] === '' ? d : parseFloat(P[k]));
const cx = num('cx', 0.5) * W, cy = num('cy', 0.5) * H, R = num('radius', 0.22) * S;
const drops = num('drops', 260), reach = num('reach', 2.2), arms = num('arms', 9);
const dir = P.dir === undefined || P.dir === '' ? null : (parseFloat(P.dir) * Math.PI) / 180;
const c = px.canvas(W, H);
c.fillStyle = P.color || '#b3121a'; c.strokeStyle = c.fillStyle; c.lineCap = 'round';
const angle = () => (dir === null ? px.random() * 6.2832 : dir + (px.random() - 0.5) * (px.random() < 0.8 ? 1.6 : 6.2832));
const dot = (x, y, r) => { c.beginPath(); c.arc(x, y, Math.max(0.6, r), 0, 6.2832); c.fill(); };
// core: a lumpy disc made of overlapping circles
for (let i = 0; i < 46; i++) { const a = px.random() * 6.2832, d = R * 0.75 * Math.sqrt(px.random()); dot(cx + Math.cos(a) * d, cy + Math.sin(a) * d, R * (0.22 + 0.4 * px.random())); }
// arms: tapering chains of dots thrown outward
for (let i = 0; i < arms; i++) {
  const a = angle(), L = R * (1.1 + (reach - 0.9) * px.random()), w = R * (0.07 + 0.09 * px.random());
  for (let t = 0.5; t < 1; t += 0.012) { const d = R * 0.7 + (L - R * 0.7) * ((t - 0.5) * 2); dot(cx + Math.cos(a) * d, cy + Math.sin(a) * d, w * (1.08 - t) * 1.1); }
  dot(cx + Math.cos(a) * L, cy + Math.sin(a) * L, w * (0.5 + px.random()));
}
// droplets: small near the edge, tiny far away
for (let i = 0; i < drops; i++) {
  const a = angle(), f = px.random(), d = R * (0.9 + reach * f * f * 1.2);
  dot(cx + Math.cos(a) * d, cy + Math.sin(a) * d, R * 0.09 * (1 - f * 0.8) * (0.2 + px.random()));
}
px.output(c);
