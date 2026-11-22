// standin.js: a neutral placeholder subject, so a layout can be built and judged before the real
// photo arrives. Swap it for the real cutout with `comp relink` or by placing the photo instead.
// Never deliver a design that still shows a stand-in.
// Run:   pixelox render js scripts/standin.js -o subject.png --size 900x1200 --param kind=person
// Params (all optional):
//   kind=person   person (head-and-shoulders bust) | bottle | can | box | skyline | logo (a round mark) | scene (opaque landscape photo)
//   tone=#b98a6a  main surface color (skin, product body, building color)
//   dark=#2a2630  clothing / cap / label color
//   bg=transparent  background ('transparent' gives a cutout with alpha; scene is always opaque)
const W = px.width, H = px.height, P = px.params, kind = P.kind || 'person';
const tone = P.tone || '#b98a6a', dark = P.dark || '#2a2630', c = px.canvas(W, H);
if (P.bg && P.bg !== 'transparent') { c.fillStyle = P.bg; c.fillRect(0, 0, W, H); }
const ell = (x, y, rx, ry, col) => { c.fillStyle = col; c.beginPath(); c.ellipse(x, y, rx, ry, 0, 0, 6.2832); c.fill(); };
const shade = (x0, y0, x1, y1, a, b) => { const g = c.createLinearGradient(x0, y0, x1, y1); g.addColorStop(0, a); g.addColorStop(1, b); return g; };
function rr(x, y, w, h, r, col) { c.fillStyle = col; c.beginPath(); c.moveTo(x + r, y); c.lineTo(x + w - r, y); c.quadraticCurveTo(x + w, y, x + w, y + r); c.lineTo(x + w, y + h - r); c.quadraticCurveTo(x + w, y + h, x + w - r, y + h); c.lineTo(x + r, y + h); c.quadraticCurveTo(x, y + h, x, y + h - r); c.lineTo(x, y + r); c.quadraticCurveTo(x, y, x + r, y); c.closePath(); c.fill(); }
if (kind === 'person') {
  const cx = W / 2, hw = W * 0.2, hy = H * 0.3, hh = hw * 1.28;
  // shoulders and chest
  c.fillStyle = dark; c.beginPath(); c.moveTo(W * 0.04, H); c.quadraticCurveTo(W * 0.06, H * 0.66, cx - hw * 0.75, H * 0.6);
  c.lineTo(cx + hw * 0.75, H * 0.6); c.quadraticCurveTo(W * 0.94, H * 0.66, W * 0.96, H); c.closePath(); c.fill();
  c.fillStyle = shade(0, 0, W, 0, 'rgba(255,255,255,0.16)', 'rgba(0,0,0,0.38)'); c.fill();
  // neck, collar opening
  rr(cx - hw * 0.42, hy + hh * 0.6, hw * 0.84, H * 0.2, hw * 0.2, tone);
  c.fillStyle = 'rgba(0,0,0,0.3)'; c.fillRect(cx - hw * 0.42, hy + hh * 0.86, hw * 0.84, H * 0.05);
  c.fillStyle = '#f2f0ec'; c.beginPath(); c.moveTo(cx - hw * 0.6, H * 0.6); c.lineTo(cx, H * 0.78); c.lineTo(cx + hw * 0.6, H * 0.6); c.closePath(); c.fill();
  // head with side light, ears, hair
  ell(cx - hw * 0.98, hy + hh * 0.05, hw * 0.13, hh * 0.2, tone); ell(cx + hw * 0.98, hy + hh * 0.05, hw * 0.13, hh * 0.2, tone);
  ell(cx, hy, hw, hh, tone);
  c.fillStyle = shade(cx - hw, 0, cx + hw, 0, 'rgba(255,255,255,0.22)', 'rgba(0,0,0,0.5)'); c.beginPath(); c.ellipse(cx, hy, hw, hh, 0, 0, 6.2832); c.fill();
  c.fillStyle = '#17141a'; c.beginPath(); c.ellipse(cx, hy - hh * 0.52, hw * 1.04, hh * 0.56, 0, Math.PI, 6.2832); c.fill();
  c.beginPath(); c.ellipse(cx, hy - hh * 0.5, hw * 1.04, hh * 0.2, 0, 0, Math.PI); c.fill();
  // features: brows, eye sockets, nose shadow, mouth
  c.fillStyle = 'rgba(20,14,16,0.85)';
  for (const s of [-1, 1]) { c.beginPath(); c.ellipse(cx + s * hw * 0.42, hy - hh * 0.12, hw * 0.24, hh * 0.035, s * 0.08, 0, 6.2832); c.fill(); c.beginPath(); c.ellipse(cx + s * hw * 0.42, hy - hh * 0.02, hw * 0.13, hh * 0.045, 0, 0, 6.2832); c.fill(); }
  c.fillStyle = 'rgba(0,0,0,0.28)'; c.beginPath(); c.moveTo(cx, hy - hh * 0.02); c.lineTo(cx + hw * 0.2, hy + hh * 0.3); c.lineTo(cx - hw * 0.06, hy + hh * 0.3); c.closePath(); c.fill();
  ell(cx, hy + hh * 0.52, hw * 0.34, hh * 0.055, 'rgba(90,30,36,0.9)');
} else if (kind === 'bottle' || kind === 'can') {
  const cx = W / 2, bw = W * (kind === 'can' ? 0.3 : 0.2), top = H * (kind === 'can' ? 0.12 : 0.34), bot = H * 0.95;
  if (kind === 'bottle') { rr(cx - bw * 0.33, H * 0.06, bw * 0.66, H * 0.32, bw * 0.15, tone); rr(cx - bw * 0.4, H * 0.04, bw * 0.8, H * 0.045, 6, dark); c.fillStyle = tone; c.beginPath(); c.moveTo(cx - bw * 0.33, H * 0.26); c.quadraticCurveTo(cx - bw, H * 0.3, cx - bw, top + H * 0.04); c.lineTo(cx + bw, top + H * 0.04); c.quadraticCurveTo(cx + bw, H * 0.3, cx + bw * 0.33, H * 0.26); c.closePath(); c.fill(); }
  rr(cx - bw, top, bw * 2, bot - top, bw * (kind === 'can' ? 0.16 : 0.3), tone);
  if (kind === 'can') { rr(cx - bw * 0.92, top - H * 0.012, bw * 1.84, H * 0.03, 8, '#c9ccd1'); rr(cx - bw * 0.92, bot - H * 0.015, bw * 1.84, H * 0.03, 8, '#9a9da3'); }
  rr(cx - bw, top + (bot - top) * 0.3, bw * 2, (bot - top) * 0.34, 0, dark);
  ell(cx, top + (bot - top) * 0.47, bw * 0.5, bw * 0.5, '#f2f0ec');
  // cylinder shading: dark edges, a bright vertical highlight
  const g = c.createLinearGradient(cx - bw, 0, cx + bw, 0);
  g.addColorStop(0, 'rgba(0,0,0,0.55)'); g.addColorStop(0.22, 'rgba(255,255,255,0.35)'); g.addColorStop(0.36, 'rgba(255,255,255,0)'); g.addColorStop(0.75, 'rgba(0,0,0,0.12)'); g.addColorStop(1, 'rgba(0,0,0,0.6)');
  c.globalCompositeOperation = 'source-atop'; c.fillStyle = g; c.fillRect(0, 0, W, H); c.globalCompositeOperation = 'source-over';
} else if (kind === 'box') {
  const x = W * 0.16, y = H * 0.18, w = W * 0.68, h = H * 0.68;
  c.fillStyle = dark; c.beginPath(); c.moveTo(x, y + h * 0.12); c.lineTo(x + w * 0.62, y); c.lineTo(x + w, y + h * 0.1); c.lineTo(x + w * 0.38, y + h * 0.24); c.closePath(); c.fill();
  c.fillStyle = tone; c.beginPath(); c.moveTo(x, y + h * 0.12); c.lineTo(x + w * 0.38, y + h * 0.24); c.lineTo(x + w * 0.38, y + h); c.lineTo(x, y + h * 0.86); c.closePath(); c.fill();
  c.beginPath(); c.moveTo(x + w * 0.38, y + h * 0.24); c.lineTo(x + w, y + h * 0.1); c.lineTo(x + w, y + h * 0.84); c.lineTo(x + w * 0.38, y + h); c.closePath(); c.fill();
  c.fillStyle = 'rgba(0,0,0,0.32)'; c.fill();
} else if (kind === 'logo') {
  const r = Math.min(W, H) * 0.46; ell(W / 2, H / 2, r, r, tone);
  c.fillStyle = dark; c.beginPath(); c.moveTo(W / 2, H / 2 - r * 0.55); c.lineTo(W / 2 + r * 0.5, H / 2 + r * 0.38); c.lineTo(W / 2 - r * 0.5, H / 2 + r * 0.38); c.closePath(); c.fill();
} else if (kind === 'skyline') {
  c.fillStyle = tone; let x = 0;
  while (x < W) { const w = W * (0.04 + 0.07 * px.random()), h = H * (0.25 + 0.7 * px.random() * px.random()); c.fillRect(x, H - h, w + 1, h); if (px.random() < 0.3) c.fillRect(x + w * 0.45, H - h - H * 0.06, w * 0.1, H * 0.06); x += w; }
} else { // scene: sky, sun, haze, three ridge lines
  c.fillStyle = shade(0, 0, 0, H, '#27457a', '#f0b27a'); c.fillRect(0, 0, W, H);
  const g = c.createRadialGradient(W * 0.68, H * 0.5, 0, W * 0.68, H * 0.5, H * 0.45); g.addColorStop(0, 'rgba(255,244,214,1)'); g.addColorStop(0.12, 'rgba(255,226,170,0.8)'); g.addColorStop(1, 'rgba(255,200,140,0)');
  c.fillStyle = g; c.fillRect(0, 0, W, H);
  ['#5b6f95', '#394a6b', '#1b2338'].forEach((col, i) => {
    c.fillStyle = col; c.beginPath(); c.moveTo(0, H); let y = H * (0.55 + i * 0.12);
    for (let x = 0; x <= W; x += W / 40) { y += (px.random() - 0.5) * H * 0.035; y = Math.max(H * (0.45 + i * 0.12), Math.min(H * (0.68 + i * 0.12), y)); c.lineTo(x, y); }
    c.lineTo(W, H); c.closePath(); c.fill();
  });
}
px.output(c);
