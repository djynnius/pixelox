// A synthetic portrait for the retouching evals (rendered by `pixelox render js`, so no image
// is committed): an oval face with soft side lighting and pore texture, a few pimples, one
// beauty mark that must be kept, eyes, brows, lips, and hair on a plain backdrop.
// Params: tone=light|medium|dark (default medium). Seed with --seed for the pore texture.
const W = 1200, H = 1500;
const tones = { light: [232, 190, 165], medium: [190, 135, 100], dark: [120, 78, 55] };
const base = tones[px.params.tone || 'medium'] || tones.medium;
const c = px.canvas(W, H);
c.fillStyle = '#5d6b78'; c.fillRect(0, 0, W, H);
// hair behind the face
c.fillStyle = '#2a1d17'; c.beginPath(); c.ellipse(600, 620, 420, 560, 0, 0, Math.PI * 2); c.fill();
// face and neck with side lighting and pores, drawn per pixel
const img = c.getImageData(0, 0, W, H), d = img.data;
const inFace = (x, y) => ((x - 600) / 330) ** 2 + ((y - 720) / 450) ** 2 <= 1 || (Math.abs(x - 600) < 150 && y > 1050);
for (let y = 0; y < H; y++) for (let x = 0; x < W; x++) {
  if (!inFace(x, y)) continue;
  const light = 1.08 - 0.35 * Math.max(0, (x - 520) / 500) - 0.1 * Math.max(0, (y - 900) / 600);
  const pore = (px.random() - 0.5) * 14;
  const i = (y * W + x) * 4;
  for (let k = 0; k < 3; k++) d[i + k] = Math.max(0, Math.min(255, base[k] * light + pore * (1 - 0.15 * k)));
}
c.putImageData(img, 0, 0);
// pimples: reddish soft spots (positions logged for graders)
const pimples = [[470, 820, 9], [720, 760, 7], [560, 960, 8], [690, 900, 6], [415, 720, 7]];
for (const [x, y, r] of pimples) {
  const g = c.createRadialGradient(x, y, 0, x, y, r * 1.6);
  g.addColorStop(0, 'rgba(150,40,40,0.75)'); g.addColorStop(1, 'rgba(150,40,40,0)');
  c.fillStyle = g; c.beginPath(); c.arc(x, y, r * 1.6, 0, Math.PI * 2); c.fill();
}
// beauty mark: small, dark, crisp (keep it)
c.fillStyle = 'rgba(45,25,20,0.95)'; c.beginPath(); c.arc(760, 1000, 6, 0, Math.PI * 2); c.fill();
// brows, eyes, lips
c.fillStyle = '#2a1d17';
c.beginPath(); c.ellipse(480, 560, 90, 18, -0.1, 0, Math.PI * 2); c.fill();
c.beginPath(); c.ellipse(720, 560, 90, 18, 0.1, 0, Math.PI * 2); c.fill();
for (const ex of [480, 720]) {
  c.fillStyle = '#f2efe9'; c.beginPath(); c.ellipse(ex, 640, 60, 26, 0, 0, Math.PI * 2); c.fill();
  c.fillStyle = '#3b2a20'; c.beginPath(); c.arc(ex, 640, 22, 0, Math.PI * 2); c.fill();
}
c.fillStyle = '#a3404a'; c.beginPath(); c.ellipse(600, 1010, 110, 38, 0, 0, Math.PI * 2); c.fill();
px.log(JSON.stringify({ pimples, beauty_mark: [760, 1000, 6] }));
px.output(c);
