// steps.js: a numbered process or a timeline: nodes joined by a line (or a band of chevrons), each with a badge,
// a title, and a short note, laid out in a row or down a spine. Output is a transparent image.
// Run:   pixelox render js scripts/steps.js -o steps.png --size 1080x1350 --param box=8,30,84,30 --param labels='Research|Design|Build|Launch' --param notes='Talk to ten users|Sketch three routes|Ship a first version|Measure and adjust'
// Place: pixelox comp place d.pxo steps.png --in-place --as Steps --at 0,0
// Data params:
//   labels=A|B|C|D           step titles, in order ('|' between items; commas also work when no title has one)
//   notes=...|...            one short note per step (wrapped automatically; '\n' forces a line break)
//   badges=01,02,03          text inside each node (default: 01, 02, ...; years for a timeline; badges=none
//                            leaves the nodes empty for icons: place them at the logged anchors with shape icon)
// Form params (all optional):
//   dir=h                    h = a row, read left to right; v = a column on a vertical spine, read top to bottom
//   side=one                 one = all text on one side (below the row, right of the spine); alt = alternating sides
//   style=circle             circle (filled nodes) | ring (white nodes with a colored outline) | dot (small dots:
//                            the badge is written beside the dot, for timelines) | chevron (arrow band; dir=h only)
//                            | snake (nodes zig-zag down the page on a thick S-curved ribbon; connector=dashed draws a road)
//   node=0                   node diameter (chevron: band height) in px; 0 = fitted to the box
//   linewidth=0              connector thickness in px (0 = thin line; snake: 0.42 x the node);  swing=0.13  snake: how far the
//                            nodes swing left and right of the middle, as a fraction of the box width
//   connector=line           line | dashed | arrow | none
//   line=#c3cad2             connector color;  nodefill=#ffffff  inside of ring nodes
//   titlesize=0              title size in px (0 = 1.15 x fs); notes are set at fs
//   onecolor=0               1 = every node takes the first color (use for a calm, single-accent design)
// Shared params (every chart script): box=x,y,w,h in % of the image (default 0,0,100,100); colors=#hex,#hex;
//   ink=#1f2933 (text) muted=#6b7785 (axis text) grid=#d5dbe1 bg=transparent; fs=label size in px;
//   font='Family, Fallback, sans-serif' (a family name: font classes such as @condensed do not work inside a script)
//   numfont= weight=400 numweight=700; prefix=$ suffix=% decimals=1 compact=1 (1.2K, 3.4M)
// The result's logs hold one JSON line: the node diameter and an anchor (canvas px and %) at the center of every node.
