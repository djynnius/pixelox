// hemicycle.js: a parliament seat chart from seat counts: one dot per seat in concentric rows, grouped into
// wedges by party, with counts and a legend. Exactly sum(seats) dots are drawn. Output is a transparent image.
// Run:   pixelox render js scripts/hemicycle.js -o seats.png --size 1080x1350 --param box=8,28,84,34 --param seats=48,5,47 --param labels='Party A,Independent,Party B' --param colors='#c8241b,#9aa5b1,#1f6fb5'
// Place: pixelox comp place d.pxo seats.png --in-place --as Seats --at 0,0
// Data params:
//   seats=48,5,47            seats per group, in order from the left end of the arc to the right end
//   labels=A,B,C             group names (legend, and the count labels)
// Form params (all optional):
//   rows=0                   number of concentric rows (0 = chosen from the seat count: 3 to 12)
//   inner=0.38               radius of the empty middle as a fraction of the outer radius
//   sweep=180                degrees covered (180 = half circle; 200 to 240 = horseshoe)
//   flip=0                   1 = open side up (a bowl), for a second chamber drawn under the first
//   shape=circle             circle | hex | square
//   fill=0.84                dot size as a fraction of the row spacing
//   hollow=                  1-based group numbers drawn as outlines only (for example vacant or undecided seats)
//   center= sub=             big text and small text in the empty middle (for example the total and 'seats')
//   centersize=              size of the center text in px (default: fitted to the middle)
//   counts=1                 write each group's count under its end of the arc (groups at the ends) or in the legend
//   legend=bottom            bottom | top | none
//   majority=0               1 = a thin tick at the middle of the arc (the halfway line)
//   cx= cy= r=               place by center instead of box: center x in % of the width, center y in % of the height,
//                            outer radius (to the outside of the last row) in % of the width
// Shared params (every chart script): box=x,y,w,h in % of the image (default 0,0,100,100); colors=#hex,#hex;
//   ink=#1f2933 (text) muted=#6b7785 (axis text) grid=#d5dbe1 bg=transparent; fs=label size in px;
//   font='Family, Fallback, sans-serif' (a family name: font classes such as @condensed do not work inside a script)
//   numfont= weight=400 numweight=700; prefix=$ suffix=% decimals=1 compact=1 (1.2K, 3.4M)
// The result's logs hold one JSON line: the center, the radii, the seat radius, and an anchor at the middle of every group.
