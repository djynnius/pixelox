// torn.js: a strip of torn paper (ragged edges with a pale fiber rim) on a transparent canvas.
// Run:   pixelox render js scripts/torn.js -o strip.png --size 1080x420 --seed 3 --param color='#efe9dc'
// Size the canvas to the strip. Use it as a collage scrap, a label behind text, or
// (mask import) as a ragged-edge mask for a photo.
// Params (all optional):
//   color=#efe9dc   paper color
//   rim=#ffffff     fiber rim color at the torn edges ('none' to skip)
//   edges=tb        which sides are torn: any of t, b, l, r (others stay straight and touch the canvas edge)
//   rough=0.5       0..1 depth of the tear
//   detail=0.6      0..1 amount of fine jaggedness
//   part=both       both | inner (the paper area only, without the rim: same shape for the same seed,
//                   use it as a clip window for a photo) | rim (the rim shape only)
const W = px.width, H = px.height, P = px.params;
const num = (k, d) => (P[k] === undefined || P[k] === '' ? d : parseFloat(P[k]));
const edges = P.edges || 'tb', rough = num('rough', 0.5), detail = num('detail', 0.6), c = px.canvas(W, H);
const depth = Math.min(W, H) * 0.09 * (0.4 + rough);
function edge(len) { // a 1-D rough profile: big wobble + small jags
  const n = Math.max(8, Math.round(len / 9)), out = [];
  let big = px.random(), tgt = px.random(), k = 0;
  for (let i = 0; i <= n; i++) {
    if (k-- <= 0) { tgt = px.random(); k = 4 + Math.floor(px.random() * 10); }
    big += (tgt - big) * 0.25;
    out.push(depth * (0.25 + 0.6 * big + detail * 0.35 * (px.random() - 0.5)));
  }
  return out;
}
function outline(inset) {
  const pts = [], has = (s) => edges.indexOf(s) >= 0;
  const T = has('t') ? edge(W) : null, B = has('b') ? edge(W) : null, L = has('l') ? edge(H) : null, R = has('r') ? edge(H) : null;
  const pad = (e) => (e ? depth * 0.2 + inset : 0);
  const n = (T || B || [0]).length - 1 || 60, m = (L || R || [0]).length - 1 || 40;
  for (let i = 0; i <= n; i++) pts.push([pad(L) + ((W - pad(L) - pad(R)) * i) / n, T ? T[i] + inset : 0]);
  for (let i = 0; i <= m; i++) pts.push([R ? W - R[i] - inset : W, pad(T) + ((H - pad(T) - pad(B)) * i) / m]);
  for (let i = n; i >= 0; i--) pts.push([pad(L) + ((W - pad(L) - pad(R)) * i) / n, B ? H - B[i] - inset : H]);
  for (let i = m; i >= 0; i--) pts.push([L ? L[i] + inset : 0, pad(T) + ((H - pad(T) - pad(B)) * i) / m]);
  return pts;
}
function fill(pts, col) { c.fillStyle = col; c.beginPath(); pts.forEach((p, i) => (i ? c.lineTo(p[0], p[1]) : c.moveTo(p[0], p[1]))); c.closePath(); c.fill(); }
const rim = P.rim || '#ffffff', part = P.part || 'both', seedPts = outline(0);
const inner = seedPts.map((p) => [p[0] + (p[0] < W / 2 ? 1 : -1) * depth * 0.1, p[1] + (p[1] < H / 2 ? 1 : -1) * depth * 0.22 * (0.5 + px.random())]);
if (part === 'inner') fill(inner, P.color || '#efe9dc');
else if (part === 'rim' || rim === 'none') fill(seedPts, part === 'rim' && rim !== 'none' ? rim : P.color || '#efe9dc');
else { fill(seedPts, rim); fill(inner, P.color || '#efe9dc'); }
px.output(c);
