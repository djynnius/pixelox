// clouds.js: fractal noise (fBm) for smoke, fog, clouds, marble, and mottled color fields.
// Run:   pixelox render js scripts/clouds.js -o clouds.png --size 1080x1350 --seed 7
// Params (all optional, pass with --param k=v):
//   scale=3      number of large noise cells across the short side (2 = huge billows, 8 = fine)
//   octaves=5    detail layers (1 = smooth blobs, 6 = wispy)
//   warp=0.6     domain warp 0..2 (0 = plain clouds, 1+ = swirling smoke / marble)
//   stretch=1    >1 stretches features vertically (rising smoke), <1 horizontally (fog bands)
//   contrast=1.4 >1 pushes toward pure lo/hi, <1 flattens (2.5+ gives hard-edged marble)
//   bias=0       -0.5..0.5 shifts the balance toward lo (negative) or hi (positive)
//   lo=#000000   color where the noise is low   (use 'transparent' for an overlay layer)
//   hi=#ffffff   color where the noise is high
//   mid=         optional third color at the middle of the ramp
//   res=4        compute at 1/res resolution and upscale (clouds are soft; 4 is fast, 2 is finer)
const W = px.width, H = px.height, P = px.params;
const num = (k, d) => (P[k] === undefined || P[k] === '' ? d : parseFloat(P[k]));
const scale = num('scale', 3), octaves = Math.round(num('octaves', 5)), warp = num('warp', 0.6);
const stretch = num('stretch', 1), contrast = num('contrast', 1.4), bias = num('bias', 0);
const res = Math.max(1, Math.round(num('res', 4)));
function rgba(s) {
  if (!s || s === 'transparent') return [0, 0, 0, 0];
  let h = s.replace('#', '');
  if (h.length === 3) h = h.split('').map((c) => c + c).join('');
  const v = [0, 2, 4].map((i) => parseInt(h.slice(i, i + 2), 16));
  v.push(h.length >= 8 ? parseInt(h.slice(6, 8), 16) : 255);
  return v;
}
const lo = rgba(P.lo || '#000000'), hi = rgba(P.hi || '#ffffff'), mid = P.mid ? rgba(P.mid) : null;
// a transparent end borrows its neighbor's color so edges do not darken
if (lo[3] === 0) for (let c = 0; c < 3; c++) lo[c] = (mid || hi)[c];
if (hi[3] === 0) for (let c = 0; c < 3; c++) hi[c] = (mid || lo)[c];
// lattice of random values, bilinear + smoothstep interpolation
const N = 256, lat = new Float32Array(N * N);
for (let i = 0; i < N * N; i++) lat[i] = px.random();
const sm = (t) => t * t * (3 - 2 * t);
function vnoise(x, y) {
  const xi = Math.floor(x), yi = Math.floor(y), tx = sm(x - xi), ty = sm(y - yi);
  const x0 = xi & 255, x1 = (xi + 1) & 255, y0 = (yi & 255) * N, y1 = ((yi + 1) & 255) * N;
  const a = lat[y0 + x0], b = lat[y0 + x1], c = lat[y1 + x0], d = lat[y1 + x1];
  return a + (b - a) * tx + (c - a) * ty + (a - b - c + d) * tx * ty;
}
function fbm(x, y) {
  let s = 0, amp = 0.5, tot = 0;
  for (let o = 0; o < octaves; o++) { s += amp * vnoise(x, y); tot += amp; x = x * 2 + 17.3; y = y * 2 + 9.1; amp *= 0.5; }
  return s / tot;
}
const w = Math.ceil(W / res), h = Math.ceil(H / res), k = scale / Math.min(w, h);
const small = px.canvas(w, h), img = small.getImageData(0, 0, w, h), d = img.data;
const mix = (a, b, t) => a + (b - a) * t;
for (let y = 0; y < h; y++) for (let x = 0; x < w; x++) {
  let u = x * k, v = (y * k) / stretch;
  if (warp > 0) {
    const qx = fbm(u + 3.1, v + 7.7), qy = fbm(u + 11.5, v + 1.3);
    u += warp * 2.5 * (qx - 0.5); v += warp * 2.5 * (qy - 0.5);
  }
  let t = (fbm(u, v) - 0.5) * contrast * 1.3 + 0.5 + bias;
  t = t < 0 ? 0 : t > 1 ? 1 : t;
  const i = (y * w + x) * 4;
  for (let c = 0; c < 4; c++) {
    d[i + c] = mid ? (t < 0.5 ? mix(lo[c], mid[c], t * 2) : mix(mid[c], hi[c], t * 2 - 1)) : mix(lo[c], hi[c], t);
  }
}
small.putImageData(img, 0, 0);
const out = px.canvas(W, H);
out.drawImage(small, 0, 0, W, H);
px.output(out);
