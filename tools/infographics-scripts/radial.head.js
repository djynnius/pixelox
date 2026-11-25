// radial.js: bars that radiate from a hub (a radial bar chart), or a ring of labeled spokes without values (a
// radial list or timeline). Bar LENGTHS are proportional to the values, measured from the hub's edge.
// Run:   pixelox render js scripts/radial.js -o radial.png --size 1080x1350 --param box=6,26,88,56 --param values=62,48,80,35,55,71,44,90 --param labels='Jan|Feb|Mar|Apr|May|Jun|Jul|Aug' --param center=485 --param sub=total
// Place: pixelox comp place d.pxo radial.png --in-place --as Radial --at 0,0
// Data params:
//   values=62,48,80          one number per bar, clockwise from the top (omit for a ring of labels only)
//   labels=A|B|C             one name per bar or spoke
// Form params (all optional):
//   inner=0.36               hub radius as a fraction of the outer radius
//   sweep=360 start=0        degrees covered; angle of the first bar, clockwise from 12 o'clock
//   rotate=auto              1 = names written along the spokes (many items, long names); 0 = upright names
//                            outside the bar ends; auto = along the spokes above 12 items
//   barwidth=0.62            bar thickness as a fraction of its slot (values form)
//   max=0 rings=0            the value a full-length bar stands for; number of faint guide rings
//   vlabels=1                the value at each bar's end
//   track=0                  1 = every bar lies on a pale full-length track
//   center= sub=             big and small text in the hub;  hub=  a color fills the hub disc
//   cx= cy= r=               place by center instead of box (% of the width, % of the height, outer radius in % of the width)
// One color for all bars is the default; colors= gives one per bar; highlight=N accent=#hex marks one.
// Shared params (every chart script): box=x,y,w,h in % of the image (default 0,0,100,100); colors=#hex,#hex;
//   ink=#1f2933 (text) muted=#6b7785 (axis text) grid=#d5dbe1 bg=transparent; fs=label size in px;
//   font='Family, Fallback, sans-serif' (a family name: font classes such as @condensed do not work inside a script)
//   numfont= weight=400 numweight=700; prefix=$ suffix=% decimals=1 compact=1 (1.2K, 3.4M)
// The result's logs hold one JSON line: the center, the radii, and an anchor at every bar's end and at the hub.
