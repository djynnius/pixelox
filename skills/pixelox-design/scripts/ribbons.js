// ribbons.js: silky, twisting gradient ribbons on a transparent canvas (flowing abstract backgrounds).
// Run:   pixelox render js scripts/ribbons.js -o ribbons.png --size 1080x1350 --seed 6
// Params (all optional):
//   colors=#ff7a00,#ff2e88,#2b6bff,#18d6ff   each ribbon blends from one color to the next across its width
//   count=3        number of ribbons
//   width=0.22     ribbon width as a fraction of the short side
//   dir=v          v = ribbons run top to bottom, h = left to right
//   sway=0.22      sideways swing as a fraction of the canvas
//   twist=1.2      how many times a ribbon turns over along its length (0 = flat band)
//   strands=70     fine lines that make up each ribbon (more = smoother)
//   alpha=0.55     strand opacity (overlaps build up to solid)
const W = px.width, H = px.height, P = px.params, S = Math.min(W, H);
const num = (k, d) => (P[k] === undefined || P[k] === '' ? d : parseFloat(P[k]));
const colors = (P.colors || '#ff7a00,#ff2e88,#2b6bff,#18d6ff').split(',').map((s) => s.trim());
const count = Math.round(num('count', 3)), bw = num('width', 0.22) * S, vertical = (P.dir || 'v') !== 'h';
const sway = num('sway', 0.22), twist = num('twist', 1.2), strands = Math.round(num('strands', 70)), alpha = num('alpha', 0.55);
const rgb = (c) => { const m = /^#?([0-9a-f]{6})/i.exec(c), v = m ? parseInt(m[1], 16) : 0xffffff; return [(v >> 16) & 255, (v >> 8) & 255, v & 255]; };
const c = px.canvas(W, H), LEN = vertical ? H : W, ACR = vertical ? W : H;
c.lineCap = 'round'; c.lineJoin = 'round';
for (let r = 0; r < count; r++) {
  const c1 = rgb(colors[r % colors.length]), c2 = rgb(colors[(r + 1) % colors.length]);
  const base = ACR * (0.25 + 0.5 * px.random()), p1 = px.random() * 6.28, p2 = px.random() * 6.28, p3 = px.random() * 6.28;
  const f1 = 0.6 + 0.6 * px.random(), f2 = 1.4 + px.random();
  for (let s = 0; s < strands; s++) {
    const t = s / (strands - 1), col = c1.map((v, i) => Math.round(v + (c2[i] - v) * t));
    const shade = 0.75 + 0.25 * Math.sin(t * Math.PI); // darker edges give the band volume
    c.strokeStyle = 'rgba(' + col.map((v) => Math.round(v * shade)).join(',') + ',' + alpha + ')';
    c.lineWidth = (bw / strands) * 2.2;
    c.beginPath();
    for (let k = -2; k <= 62; k++) {
      const u = k / 60, l = u * LEN;
      const mid = base + ACR * sway * (Math.sin(u * 6.28 * f1 + p1) + 0.4 * Math.sin(u * 6.28 * f2 + p2));
      const wd = bw * Math.cos(u * Math.PI * twist + p3), a = mid + (t - 0.5) * wd;
      if (vertical) { if (k === -2) c.moveTo(a, l); else c.lineTo(a, l); } else { if (k === -2) c.moveTo(l, a); else c.lineTo(l, a); }
    }
    c.stroke();
  }
}
px.output(c);
