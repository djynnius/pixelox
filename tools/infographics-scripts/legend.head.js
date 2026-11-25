// legend.js: a legend by itself: color keys with names, in a centered row (wrapping) or a column. Use it when
// one legend serves several charts, or when a chart's own legend is turned off.
// Run:   pixelox render js scripts/legend.js -o legend.png --size 1080x1350 --param box=8,80,84,6 --param labels='Party A|Vacant|Party B' --param colors='#c8241b,#9aa5b1,#1f6fb5'
// Place: pixelox comp place d.pxo legend.png --in-place --as Legend --at 0,0
// Params:
//   labels=A|B|C             the names, in the order of colors=
//   dir=row                  row (centered in the box, wraps to more lines) | column (left-aligned at the box's left edge)
//   shape=square             square | circle | line (a short stroke, for line charts)
//   bold=0                   1 = names in bold
// Shared params (every chart script): box=x,y,w,h in % of the image (default 0,0,100,100); colors=#hex,#hex;
//   ink=#1f2933 (text) muted=#6b7785 (axis text) grid=#d5dbe1 bg=transparent; fs=label size in px;
//   font='Family, Fallback, sans-serif' (a family name: font classes such as @condensed do not work inside a script)
//   numfont= weight=400 numweight=700; prefix=$ suffix=% decimals=1 compact=1 (1.2K, 3.4M)
