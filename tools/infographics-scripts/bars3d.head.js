// bars3d.js: isometric columns: each value is a block with a lit top and two shaded sides, standing on a floor.
// The view is parallel (no perspective), so the block HEIGHTS stay proportional to the values and every value
// is printed. A look for hero charts and dark dashboards, not a more exact chart: when precision matters use bars.js.
// Run:   pixelox render js scripts/bars3d.js -o blocks.png --size 1080x1350 --param box=8,26,84,50 --param values=52,80,31,60 --param labels='North,South,East,West' --param suffix=%
// Place: pixelox comp place d.pxo blocks.png --in-place --as Blocks --at 0,0
// Data params:
//   values=52,80,31,60       one number per block (zero or more)
//   labels=A,B,C,D           names under the blocks
// Form params (all optional):
//   shape=box                box (square prism) | cylinder
//   depth=1                  block depth as a multiple of its width (0.6 = slabs, 1 = square)
//   gap=0.7                  space between blocks as a fraction of a block's width
//   max=0                    the value a full-height block stands for (0 = the largest value)
//   floor=#e2e6ea            floor color (floor=none: no floor)
//   vlabels=1                values above the blocks
//   light=0.22 dark=0.28     how much the top is lightened and the right side darkened
// One color for all blocks reads calmest; with colors= each block takes its own.
// Shared params (every chart script): box=x,y,w,h in % of the image (default 0,0,100,100); colors=#hex,#hex;
//   ink=#1f2933 (text) muted=#6b7785 (axis text) grid=#d5dbe1 bg=transparent; fs=label size in px;
//   font='Family, Fallback, sans-serif' (a family name: font classes such as @condensed do not work inside a script)
//   numfont= weight=400 numweight=700; prefix=$ suffix=% decimals=1 compact=1 (1.2K, 3.4M)
// The result's logs hold one JSON line with an anchor at the top center and the base center of every block.
