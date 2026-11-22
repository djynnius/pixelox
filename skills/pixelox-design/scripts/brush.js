// brush.js: one dry-brush paint swash (streaky bristles, ragged ends) on a transparent canvas.
// Run:   pixelox render js scripts/brush.js -o swash.png --size 1000x360 --seed 2 --param color='#f5a623'
// The swash fills the canvas you ask for: size the canvas to the swash (wide and short),
// then place it and rotate it in the document.
// Params (all optional):
//   color=#f5a623
//   bristles=140   number of streaks (fewer = drier, more gaps)
//   rough=0.5      0..1 raggedness of both ends
//   dry=0.35       0..1 share of streaks that break up along their length
//   passes=2       overlapping strokes (2-3 gives the look of a re-loaded brush)
//   alpha=1
const W = px.width, H = px.height, P = px.params;
const num = (k, d) => (P[k] === undefined || P[k] === '' ? d : parseFloat(P[k]));
const bristles = Math.round(num('bristles', 140)), rough = num('rough', 0.5), dry = num('dry', 0.35);
const passes = Math.round(num('passes', 2)), c = px.canvas(W, H);
c.strokeStyle = P.color || '#f5a623'; c.lineCap = 'round';
for (let p = 0; p < passes; p++) {
  const top = H * (0.08 + 0.18 * p * px.random()), hgt = H * (0.84 - 0.25 * p * px.random());
  const x0 = W * (0.03 + 0.08 * px.random()), x1 = W * (0.97 - 0.08 * px.random());
  const bow = (px.random() - 0.5) * H * 0.12;
  for (let i = 0; i < bristles; i++) {
    const t = i / (bristles - 1), y = top + t * hgt;
    const edge = Math.pow(Math.abs(t - 0.5) * 2, 3); // outer bristles are shorter
    const a = x0 + W * rough * (0.02 + 0.2 * px.random() * (0.3 + edge));
    const b = x1 - W * rough * (0.02 + 0.3 * px.random() * (0.3 + edge));
    c.lineWidth = (hgt / bristles) * (0.9 + 1.6 * px.random());
    c.globalAlpha = num('alpha', 1) * (0.75 + 0.25 * px.random());
    let x = a;
    while (x < b) { // a dry bristle skips
      const run = px.random() < dry ? (b - a) * (0.08 + 0.3 * px.random()) : b - x;
      const e = Math.min(b, x + run), k = (x) => y + bow * Math.sin(((x - x0) / (x1 - x0)) * Math.PI);
      c.beginPath(); c.moveTo(x, k(x)); c.lineTo((x + e) / 2, k((x + e) / 2) + (px.random() - 0.5) * 2); c.lineTo(e, k(e)); c.stroke();
      x = e + (b - a) * 0.04 * px.random() + c.lineWidth;
    }
  }
}
px.output(c);
