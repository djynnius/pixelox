// (shared with the pixelox-design skill, scripts/rays.js: keep both copies the same)
// rays.js: a sunburst of alternating wedges on a transparent canvas (retro rays, god rays, spotlight fans).
// Run:   pixelox render js scripts/rays.js -o rays.png --size 1080x1350 --param color='#ffffff' --param cy=0.3
// Params (all optional):
//   color=#ffffff  ray color
//   count=18       number of rays
//   cx=0.5 cy=0.5  center in 0..1 canvas fractions (cy=-0.1 puts the source above the canvas)
//   alpha=0.35     opacity at the center
//   fade=1         1 = rays fade to nothing at the far corner; 0 = flat color to the edge (retro)
//   duty=0.5       ray width as a fraction of each ray+gap pair
//   uneven=0       0..1 random variation of ray widths (1 = natural light shafts)
//   spread=360     degrees covered, centered on 'dir' (60 = a spotlight fan)
//   dir=90         direction of the fan's middle in degrees (90 = down, 0 = right)
//   style=wedge    wedge = rays to the canvas edge; spokes = a firework of round-capped lines
//   inner=0.04 outer=0.2   spokes only: where each line starts and ends, as fractions of the short side
//   width=0.012    spokes only: line thickness as a fraction of the short side
const W = px.width, H = px.height, P = px.params;
const num = (k, d) => (P[k] === undefined || P[k] === '' ? d : parseFloat(P[k]));
const cx = num('cx', 0.5) * W, cy = num('cy', 0.5) * H, count = Math.round(num('count', 18));
const alpha = num('alpha', 0.35), fade = num('fade', 1), duty = num('duty', 0.5), uneven = num('uneven', 0);
const spread = (num('spread', 360) * Math.PI) / 180, dir = (num('dir', 90) * Math.PI) / 180;
const col = P.color || '#ffffff', m = /^#?([0-9a-f]{6})/i.exec(col), v = m ? parseInt(m[1], 16) : 0xffffff;
const rgb = [(v >> 16) & 255, (v >> 8) & 255, v & 255].join(',');
const R = Math.max(Math.hypot(cx, cy), Math.hypot(W - cx, cy), Math.hypot(cx, H - cy), Math.hypot(W - cx, H - cy));
const c = px.canvas(W, H), g = c.createRadialGradient(cx, cy, 0, cx, cy, R);
g.addColorStop(0, 'rgba(' + rgb + ',' + alpha + ')');
g.addColorStop(1, 'rgba(' + rgb + ',' + alpha * (1 - fade) + ')');
c.fillStyle = g;
const step = spread / count, a0 = dir - spread / 2;
if ((P.style || 'wedge') === 'spokes') {
  const S = Math.min(W, H), r0 = num('inner', 0.04) * S, r1 = num('outer', 0.2) * S;
  c.strokeStyle = 'rgba(' + rgb + ',' + (P.alpha === undefined ? 1 : alpha) + ')'; c.lineCap = 'round'; c.lineWidth = num('width', 0.012) * S;
  for (let i = 0; i < count; i++) {
    const a = a0 + i * step, k = 1 - uneven * 0.5 * px.random();
    c.beginPath(); c.moveTo(cx + Math.cos(a) * r0, cy + Math.sin(a) * r0); c.lineTo(cx + Math.cos(a) * r1 * k, cy + Math.sin(a) * r1 * k); c.stroke();
  }
  px.output(c);
} else {
for (let i = 0; i < count; i++) {
  const wdt = step * duty * (1 - uneven * px.random()), a = a0 + i * step + (uneven ? (px.random() - 0.5) * step * uneven : 0);
  c.beginPath(); c.moveTo(cx, cy);
  c.lineTo(cx + Math.cos(a) * R, cy + Math.sin(a) * R);
  c.lineTo(cx + Math.cos(a + wdt) * R, cy + Math.sin(a + wdt) * R);
  c.closePath(); c.fill();
}
px.output(c);
}
