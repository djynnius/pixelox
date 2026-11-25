// venn.js: a Venn diagram of 2, 3, or 4 overlapping sets: translucent circles whose overlaps mix, a name in
// each set, a label in each overlap, and one in the middle. Shows what things share; areas are not to scale.
// Run:   pixelox render js scripts/venn.js -o venn.png --size 1080x1350 --param box=8,28,84,46 --param labels='Useful|Usable|Beautiful' --param overlaps='works|admired|pleasant' --param center='Loved'
// Place: pixelox comp place d.pxo venn.png --in-place --as Venn --at 0,0
// Data params:
//   labels=A|B|C             2 to 4 set names (2: side by side; 3: one on top, two below; 4: top, right, bottom, left)
//   overlaps=ab|ac|bc        labels of the pairwise overlaps: 2 sets: ab; 3 sets: ab, ac, bc (top-left, top-right,
//                            bottom); 4 sets: the four neighbor overlaps clockwise from the top-right
//   center=                  label of the area all sets share
//   notes=...|...            a short note outside each circle (2 or 4 sets), same order as labels
// Form params (all optional):
//   opacity=0.78             circle opacity
//   blend=multiply           multiply (light grounds: overlaps darken) | screen (dark grounds: overlaps glow) | normal
//   overlap=0.5              how far the circles overlap: 0.3 (a sliver) to 0.7 (a lot)
//   labelcolor=              color of the names inside the circles (default: white or ink, by contrast)
//   caps=0                   1 = set names in capitals with wide tracking
//   titlesize=0              set name size in px (0 = 1.35 x fs)
// Shared params (every chart script): box=x,y,w,h in % of the image (default 0,0,100,100); colors=#hex,#hex;
//   ink=#1f2933 (text) muted=#6b7785 (axis text) grid=#d5dbe1 bg=transparent; fs=label size in px;
//   font='Family, Fallback, sans-serif' (a family name: font classes such as @condensed do not work inside a script)
//   numfont= weight=400 numweight=700; prefix=$ suffix=% decimals=1 compact=1 (1.2K, 3.4M)
// The result's logs hold one JSON line: the circle radius and an anchor at every circle center, overlap, and the middle.
