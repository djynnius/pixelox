// cycle.js: a cycle or loop diagram: N steps placed evenly around a ring with arrows between them and their
// labels outside, or a ring cut into N arrow segments. Also a hub with N spokes. Output is a transparent image.
// Run:   pixelox render js scripts/cycle.js -o cycle.png --size 1080x1350 --param box=6,28,88,50 --param labels='Plan|Build|Measure|Learn' --param notes='Set one goal|Make the smallest thing|Count what happened|Decide what changes' --param center='Loop'
// Place: pixelox comp place d.pxo cycle.png --in-place --as Cycle --at 0,0
// Data params:
//   labels=A|B|C|D           step titles, clockwise from the top (3 to 10 work well)
//   notes=...|...            one short note per step (wrapped automatically; '\n' forces a line break)
//   badges=1,2,3             text inside each node (default 1, 2, ...; badges=none leaves them empty for icons)
// Form params (all optional):
//   style=nodes              nodes (circles on a ring, arrows between) | segments (the ring itself is cut into
//                            arrow-shaped arcs) | spokes (nodes joined to the center, no order implied)
//   start=0                  angle of the first step, degrees clockwise from 12 o'clock
//   node=0                   node diameter in px (segments: ring thickness); 0 = fitted
//   center= sub=             big and small text in the middle
//   line=#c3cad2             ring and arrow color;  nodefill=  set a color to draw outlined (ring) nodes
//   labelwidth=0.22          width kept for the labels on each side, as a fraction of the box width
//   titlesize=0              title size in px (0 = 1.15 x fs)
//   onecolor=0               1 = every node takes the first color
// Shared params (every chart script): box=x,y,w,h in % of the image (default 0,0,100,100); colors=#hex,#hex;
//   ink=#1f2933 (text) muted=#6b7785 (axis text) grid=#d5dbe1 bg=transparent; fs=label size in px;
//   font='Family, Fallback, sans-serif' (a family name: font classes such as @condensed do not work inside a script)
//   numfont= weight=400 numweight=700; prefix=$ suffix=% decimals=1 compact=1 (1.2K, 3.4M)
// The result's logs hold one JSON line: the center, the ring radius, and an anchor (canvas px and %) at every node.
