// particles.js: scattered sparks, embers, dust, confetti, droplets, or stars on a transparent canvas.
// Run:   pixelox render js scripts/particles.js -o sparks.png --size 1080x1350 --seed 5 --param shape=streak --param colors='#ffd27a,#ff8a1f'
// Params (all optional):
//   shape=dot       dot | glow (soft dot) | streak (motion line) | rect (confetti) | star (4-point sparkle) | ring (bubble)
//                   | cube (ice cube, translucent with highlights) | drop (water bead with a highlight, for condensation)
//   colors=#ffffff  comma list
//   count=250
//   min=0.2 max=1.2 size range in % of the short side
//   angle=-70       streak / rect direction in degrees (-90 = straight up)
//   jitter=25       random spread of that direction in degrees (rect uses any angle when 180)
//   amin=0.3 amax=1 opacity range
//   focus=          'x,y,r' in 0..1: cluster around a point with radius r (default: whole canvas)
//   grow=0          >0 makes particles larger toward the bottom (1 = up to 2x), <0 toward the top
const W = px.width, H = px.height, P = px.params, S = Math.min(W, H) / 100;
const num = (k, d) => (P[k] === undefined || P[k] === '' ? d : parseFloat(P[k]));
const shape = P.shape || 'dot', colors = (P.colors || '#ffffff').split(',').map((s) => s.trim());
const count = Math.round(num('count', 250)), mn = num('min', 0.2), mx = num('max', 1.2);
const ang = (num('angle', -70) * Math.PI) / 180, jit = (num('jitter', 25) * Math.PI) / 180;
const amin = num('amin', 0.3), amax = num('amax', 1), grow = num('grow', 0);
const focus = P.focus ? P.focus.split(',').map(parseFloat) : null;
const hex = (col) => { const m = /^#?([0-9a-f]{6})/i.exec(col), v = m ? parseInt(m[1], 16) : 0xffffff; return [(v >> 16) & 255, (v >> 8) & 255, v & 255].join(','); };
const c = px.canvas(W, H);
c.lineCap = 'round';
for (let i = 0; i < count; i++) {
  let x = px.random() * W, y = px.random() * H;
  if (focus) { // gaussian-ish cluster
    const a = px.random() * 6.2832, d = focus[2] * Math.min(W, H) * Math.sqrt(-2 * Math.log(1 - px.random() * 0.98)) * 0.5;
    x = focus[0] * W + Math.cos(a) * d; y = focus[1] * H + Math.sin(a) * d;
  }
  const t = px.random(), col = colors[i % colors.length];
  let r = S * (mn + (mx - mn) * t * t) * (1 + grow * (grow > 0 ? y / H : -(1 - y / H)));
  const a = ang + (px.random() - 0.5) * 2 * jit;
  c.globalAlpha = amin + (amax - amin) * px.random();
  c.fillStyle = col; c.strokeStyle = col;
  if (shape === 'glow') {
    const g = c.createRadialGradient(x, y, 0, x, y, r * 2);
    g.addColorStop(0, 'rgba(' + hex(col) + ',1)'); g.addColorStop(0.25, 'rgba(' + hex(col) + ',0.6)'); g.addColorStop(1, 'rgba(' + hex(col) + ',0)');
    c.fillStyle = g; c.beginPath(); c.arc(x, y, r * 2, 0, 6.2832); c.fill();
  } else if (shape === 'streak') {
    const L = r * (4 + 10 * px.random());
    c.lineWidth = Math.max(1, r * 0.5); c.beginPath(); c.moveTo(x, y); c.lineTo(x + Math.cos(a) * L, y + Math.sin(a) * L); c.stroke();
  } else if (shape === 'rect') {
    c.save(); c.translate(x, y); c.rotate(a); c.fillRect(-r, -r * 0.45, r * 2, r * 0.9); c.restore();
  } else if (shape === 'star') {
    c.beginPath();
    for (let k = 0; k < 8; k++) { const rr = k % 2 ? r * 0.18 : r * 1.6, th = (k * Math.PI) / 4; c.lineTo(x + Math.cos(th) * rr, y + Math.sin(th) * rr); }
    c.closePath(); c.fill();
  } else if (shape === 'cube') { // a tilted rounded block: pale body, lighter inner facet, bright top-left edges
    const k = hex(col), q = r * 0.28;
    c.save(); c.translate(x, y); c.rotate(a);
    const rr = (s, f) => { c.fillStyle = f; c.beginPath(); c.moveTo(-s + q, -s); c.lineTo(s - q, -s); c.quadraticCurveTo(s, -s, s, -s + q); c.lineTo(s, s - q); c.quadraticCurveTo(s, s, s - q, s); c.lineTo(-s + q, s); c.quadraticCurveTo(-s, s, -s, s - q); c.lineTo(-s, -s + q); c.quadraticCurveTo(-s, -s, -s + q, -s); c.closePath(); c.fill(); };
    const g = c.createLinearGradient(-r, -r, r, r); g.addColorStop(0, 'rgba(' + k + ',0.75)'); g.addColorStop(0.5, 'rgba(' + k + ',0.28)'); g.addColorStop(1, 'rgba(' + k + ',0.6)');
    rr(r, g); rr(r * 0.62, 'rgba(255,255,255,0.22)');
    c.strokeStyle = 'rgba(255,255,255,0.9)'; c.lineWidth = Math.max(1, r * 0.09); c.beginPath(); c.moveTo(-r * 0.8, r * 0.3); c.lineTo(-r * 0.8, -r * 0.8); c.lineTo(r * 0.3, -r * 0.8); c.stroke();
    c.restore();
  } else if (shape === 'drop') { // a bead of water: faint body, dark lower rim, bright upper highlight
    const k = hex(col), ry = r * (1 + 0.5 * px.random());
    c.fillStyle = 'rgba(' + k + ',0.28)'; c.beginPath(); c.ellipse(x, y, r, ry, 0, 0, 6.2832); c.fill();
    c.strokeStyle = 'rgba(0,0,0,0.28)'; c.lineWidth = Math.max(0.8, r * 0.16); c.beginPath(); c.ellipse(x, y, r, ry, 0, 0.3, 2.6); c.stroke();
    c.fillStyle = 'rgba(255,255,255,0.9)'; c.beginPath(); c.ellipse(x - r * 0.3, y - ry * 0.4, r * 0.32, ry * 0.2, -0.5, 0, 6.2832); c.fill();
  } else if (shape === 'ring') {
    c.lineWidth = Math.max(1, r * 0.18); c.beginPath(); c.arc(x, y, r, 0, 6.2832); c.stroke();
  } else { c.beginPath(); c.arc(x, y, r, 0, 6.2832); c.fill(); }
}
px.output(c);
