// (shared with the pixelox-design skill, scripts/blobs.js: keep both copies the same)
// blobs.js: big soft color blobs for gradient-mesh / aura backgrounds, light leaks, and bokeh.
// Run:   pixelox render js scripts/blobs.js -o mesh.png --size 1080x1350 --seed 4 --param colors='#ff5a1f,#ffb000,#e83e8c'
// Params (all optional):
//   colors=#ff5a1f,#ffb000,#e83e8c   comma list, used in turn
//   bg=#0b0b0d    background color, or 'transparent' for an overlay layer
//   count=6       number of blobs
//   size=0.55     blob radius as a fraction of the short side
//   vary=0.4      0..1 random size variation
//   soft=1        1 = fades from the center to nothing (mesh, glow); 0.2 = disc with a soft rim (bokeh)
//   alpha=0.9     blob opacity 0..1
//   edge=0        0 = anywhere; 1 = centers pushed out to the canvas border (leaves the middle clear)
//   mode=source-over   or screen / lighter (bokeh, light leaks) / multiply
//   at=           optional fixed centers 'x,y;x,y' in 0..1 canvas fractions (overrides random placement)
const W = px.width, H = px.height, P = px.params, S = Math.min(W, H);
const num = (k, d) => (P[k] === undefined || P[k] === '' ? d : parseFloat(P[k]));
const colors = (P.colors || '#ff5a1f,#ffb000,#e83e8c').split(',').map((s) => s.trim());
const bg = P.bg || '#0b0b0d', count = Math.round(num('count', 6)), size = num('size', 0.55);
const vary = num('vary', 0.4), soft = Math.max(0.02, Math.min(1, num('soft', 1))), alpha = num('alpha', 0.9), edge = num('edge', 0);
const fixed = P.at ? P.at.split(';').map((s) => s.split(',').map(parseFloat)) : null;
const c = px.canvas(W, H);
if (bg !== 'transparent') { c.fillStyle = bg; c.fillRect(0, 0, W, H); }
c.globalCompositeOperation = P.mode || 'source-over';
const n = fixed ? fixed.length : count;
for (let i = 0; i < n; i++) {
  let x = px.random(), y = px.random();
  if (edge > 0) { // push the center toward the nearest border
    const dx = x < 0.5 ? -x : 1 - x, dy = y < 0.5 ? -y : 1 - y;
    if (Math.abs(dx) < Math.abs(dy)) x += dx * edge; else y += dy * edge;
  }
  if (fixed) { x = fixed[i][0]; y = fixed[i][1]; }
  const r = S * size * (1 - vary * px.random()), X = x * W, Y = y * H;
  // fade the same hue to transparent (a plain 'transparent' stop would leave a dark rim)
  const col = colors[i % colors.length], m = /^#?([0-9a-f]{6})/i.exec(col), v = m ? parseInt(m[1], 16) : 0xffffff;
  const rgb = [(v >> 16) & 255, (v >> 8) & 255, v & 255].join(','), g = c.createRadialGradient(X, Y, 0, X, Y, r);
  g.addColorStop(0, 'rgba(' + rgb + ',1)'); g.addColorStop(1 - soft, 'rgba(' + rgb + ',1)');
  if (soft > 0.5) g.addColorStop(1 - soft * 0.5, 'rgba(' + rgb + ',0.35)');
  g.addColorStop(1, 'rgba(' + rgb + ',0)');
  c.fillStyle = g;
  c.globalAlpha = alpha * (fixed ? 1 : 0.6 + 0.4 * px.random());
  c.beginPath(); c.arc(X, Y, r, 0, Math.PI * 2); c.fill();
}
px.output(c);
