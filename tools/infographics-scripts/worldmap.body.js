const LAND = [
  '...................NNNNN.NNNNNNN........................................',
  '............NNNNNNNNN.NNNNNNNNNN......EEE..............RR...............',
  '...........NNNNNNNNNN....NNNNNNN..............EE....RRRRRRR..RRRR.......',
  'RR.NNNNNNNNNNNNNNNNNNNNN..NNNNN........EEEEEEEEERRRRRRRRRRRRRRRRRRRRRRRR',
  '...NNNNNNNNNNNNNNN...NNN..NN....EE...EEEEEEEEEEERRRRRRRRRRRRRRRRRRRRRRR.',
  '....NN..NNNNNNNNN....NNN...........E.EEEEEEEEEEERRRRRRRRRRRRRRRR...RR...',
  '..........NNNNNNNNNNNNNNNN........EEEEEEEEEEEEEERRRRRRRRRRRRRRRRR..R....',
  '...........NNNNNNNNNNNNN...........EEEEEEEEEEEEERRRRRRRRRRRRRRRRR.......',
  '...........NNNNNNNNNNN............EEEEEEEE..RR.RRRRRRRRRRRRRRR..R.......',
  '...........NNNNNNNNNN.............EE..E.ERRRRR.RRRRRRRRRRRRRRR.RR.......',
  '............NNNNNNNNN.............AAAAAAAAARRRRRRRRRRRRRRRRRR.RR........',
  '.............NNNN..N.............AAAAAAAAAARRRRRRRRRRRRRRRRRR...........',
  '....N.........NNN.NNNN..........AAAAAAAAAAAARRRR.RRRRRRRRRRRR...........',
  '...............NNNN..NN.........AAAAAAAAAAAARRR...RRRR.RRR..R...........',
  '.................NNN............AAAAAAAAAAAAAA.....RR..RRR..RR..........',
  '...................NSSSSS........AAAAAAAAAAAAA......R..RR..RRR..........',
  '....................SSSSSS............AAAAAAAA.........RRRRRRR..........',
  '...................SSSSSSSSSS.........AAAAAAA...........RRRRR.OOO.......',
  '...................SSSSSSSSSS.........AAAAAA.............RR.RR.OOO......',
  '....................SSSSSSSSS.........AAAAAAAA................OOO.......',
  '.....................SSSSSSSS.........AAAAAAAA..............OOOOOO......',
  '......................SSSSSS..........AAAAA.AA............OOOOOOOO......',
  '.....................SSSSSS............AAAA...............OOOOOOOOO.....',
  '.....................SSSSS.............AAA.................OO..OOOO.....',
  '.....................SSSS.......................................OO....OO',
  '.....................SSS.........................................O...OO.',
  '.....................SS..............................................O..',
  '.....................SS.................................................',
];
const NAMES = { namerica: 'N', samerica: 'S', europe: 'E', africa: 'A', asia: 'R', oceania: 'O' }, LONG = { N: 'North America', S: 'South America', E: 'Europe', A: 'Africa', R: 'Asia', O: 'Oceania' };
const hl = {}; for (const it of split(str('highlight', ''))) { const p = it.split(':'), k = NAMES[p[0].trim().toLowerCase().replace(/[^a-z]/g, '')]; if (k && p[1]) hl[k] = p[1].trim(); }
const dotc = str('dot', '#c5ccd3'), shape = str('shape', 'circle').toLowerCase(), fillF = num('fill', 0.74), g = el(svg, 'g', {}), small = fs * 0.9;
const NC = 72, NR = LAND.length, cw = Math.min(B.w / NC, B.h / NR), gw = cw * NC, gh = cw * NR, gx = B.x + (B.w - gw) / 2, gy = B.y + (B.h - gh) / 2;
const sums = {};
for (let r = 0; r < NR; r++) for (let c = 0; c < NC; c++) { const ch = LAND[r][c]; if (ch === '.') continue; const x = gx + (c + 0.5) * cw, y = gy + (r + 0.5) * cw, f = hl[ch] || dotc, s = cw * fillF;
  if (shape === 'square') el(g, 'rect', { x: r2(x - s / 2), y: r2(y - s / 2), width: r2(s), height: r2(s), rx: r2(s * 0.15), fill: f }); else el(g, 'circle', { cx: r2(x), cy: r2(y), r: r2(s / 2), fill: f });
  const q = sums[ch] || (sums[ch] = [0, 0, 0]); q[0] += x; q[1] += y; q[2]++; }
for (const k in sums) A(k, LONG[k], sums[k][0] / sums[k][2], sums[k][1] / sums[k][2]);
const X = (lon) => gx + ((lon + 180) / 360) * gw, Y = (lat) => gy + ((85 - lat) / (NR * 5)) * gh;
const pts = split(str('points', '')).map((s) => { const p = s.split(':'); return { lon: parseFloat(p[0]), lat: parseFloat(p[1]), label: (p[2] || '').trim(), v: p[3] === undefined ? null : parseFloat(p[3]) }; }).filter((p) => !isNaN(p.lon) && !isNaN(p.lat));
const halo = str('halo', '#ffffff'), pc = str('pointcolor', '#e11d48'), vmax = Math.max.apply(null, pts.map((p) => p.v || 0).concat([0])), maxr = num('maxr', 0.05) * gw, showL = on('pointlabels', 1);
pts.sort((a, b) => (b.v || 0) - (a.v || 0)).forEach((p, i) => { const x = X(p.lon), y = Y(p.lat), r = vmax > 0 && p.v !== null ? Math.max(cw * 0.5, maxr * Math.sqrt(Math.max(0, p.v) / vmax)) : cw * 0.75;
  if (vmax > 0) el(g, 'circle', { cx: r2(x), cy: r2(y), r: r2(r), fill: pc, 'fill-opacity': 0.72, stroke: '#ffffff', 'stroke-width': r2(Math.max(1, cw * 0.12)) });
  else { el(g, 'circle', { cx: r2(x), cy: r2(y), r: r2(r * 1.9), fill: pc, 'fill-opacity': 0.22 }); el(g, 'circle', { cx: r2(x), cy: r2(y), r: r2(r), fill: pc, stroke: '#ffffff', 'stroke-width': r2(Math.max(1, cw * 0.14)) }); }
  if (showL && p.label) { const right = x + r + small * 0.4 + tw(p.label, small, 1) < B.x + B.w; T(g, x + (right ? 1 : -1) * (r + small * 0.4), y + mid(small) - (p.v !== null && vmax > 0 ? small * 0.55 : 0), p.label, { size: small, weight: 700, anchor: right ? 'start' : 'end', halo }); if (p.v !== null && vmax > 0) T(g, x + (right ? 1 : -1) * (r + small * 0.4), y + mid(small) + small * 0.6, fmt(p.v), { size: small, anchor: right ? 'start' : 'end', fill: muted, font: numfont, halo }); }
  A('point' + (i + 1), p.label, x, y, p.v === null ? {} : { value: p.v, r: Math.round(r) }); });
done('worldmap', { map: [gx, gy, gw, gh].map(Math.round), cell: r1(cw) });
