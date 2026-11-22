// tiles.js: a mosaic of square tiles, each holding one simple motif (quarter disc, half disc, dot,
// triangle, ring, bars): the geometric-pattern look of corporate and festival posters.
// Run:   pixelox render js scripts/tiles.js -o tiles.png --size 640x960 --seed 2 --param cols=4
// The canvas you ask for is filled edge to edge: size it to the panel the mosaic should cover.
// Params (all optional):
//   colors=#e63946,#f1c40f,#1d3557,#2ec4b6,#ff7f11,#f4f1de   palette; tile ground and motif are two different picks
//   cols=4          tiles across (rows follow from the canvas shape)
//   motifs=qhdtrb   which motifs to use: q quarter disc, h half disc, d dot, t triangle, r ring, b bars, s solid
//   gap=0           space between tiles in px (shows through as transparent)
const W = px.width, H = px.height, P = px.params;
const num = (k, d) => (P[k] === undefined || P[k] === '' ? d : parseFloat(P[k]));
const colors = (P.colors || '#e63946,#f1c40f,#1d3557,#2ec4b6,#ff7f11,#f4f1de').split(',').map((s) => s.trim());
const cols = Math.max(1, Math.round(num('cols', 4))), T = W / cols, rows = Math.max(1, Math.round(H / T)), TH = H / rows;
const motifs = (P.motifs || 'qhdtrb').split(''), gap = num('gap', 0), c = px.canvas(W, H);
const pick = (not) => { let k; do { k = colors[Math.floor(px.random() * colors.length)]; } while (k === not && colors.length > 1); return k; };
for (let j = 0; j < rows; j++) for (let i = 0; i < cols; i++) {
  const x = i * T + gap / 2, y = j * TH + gap / 2, w = T - gap, h = TH - gap, bg = pick(null), fg = pick(bg);
  const m = motifs[Math.floor(px.random() * motifs.length)], q = Math.floor(px.random() * 4);
  c.save(); c.beginPath(); c.rect(x, y, w, h); c.clip();
  c.fillStyle = bg; c.fillRect(x, y, w, h); c.fillStyle = fg; c.strokeStyle = fg;
  c.translate(x + w / 2, y + h / 2); c.rotate((q * Math.PI) / 2); const s = Math.max(w, h) / 2;
  c.beginPath();
  if (m === 'q') { c.moveTo(-s, -s); c.arc(-s, -s, s * 2, 0, Math.PI / 2); c.closePath(); c.fill(); }
  else if (m === 'h') { c.arc(0, -s, s, 0, Math.PI); c.closePath(); c.fill(); }
  else if (m === 'd') { c.arc(0, 0, s * 0.55, 0, 6.2832); c.fill(); }
  else if (m === 't') { c.moveTo(-s, -s); c.lineTo(s, -s); c.lineTo(-s, s); c.closePath(); c.fill(); }
  else if (m === 'r') { c.lineWidth = s * 0.34; c.arc(-s, -s, s * 1.3, 0, Math.PI / 2); c.stroke(); }
  else if (m === 'b') { for (let k = 0; k < 3; k++) c.fillRect(-s + (k * 2 * s) / 3, -s, s / 3, 2 * s); }
  c.restore();
}
px.output(c);
