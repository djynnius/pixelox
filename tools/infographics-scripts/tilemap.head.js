// tilemap.js: a tile-grid map (cartogram): one equal tile per region, colored by a category or by a value.
// Built in: the 50 US states plus DC (layout=us). Any other set of regions: give your own grid with cells=.
// For real state outlines use usmap.js (pixelox 0.6.0 or later); this script runs on any build.
// This is not a geographic map: it shows which regions belong to which class, with every region equally visible.
// Run:   pixelox render js scripts/tilemap.js -o map.png --size 1080x1350 --param box=6,30,88,42 --param groups='CA:1,OR:1,WA:1,TX:2,FL:2,OH:3' --param names='Group A,Group B,Open' --param colors='#1f6fb5,#c8241b,#8b5cf6'
// Place: pixelox comp place d.pxo map.png --in-place --as Map --at 0,0
// Data params (use one form):
//   groups=CA:1,TX:2,OH:3    region code : 1-based class number (class colors come from colors=, names from names=)
//   values=CA:31,TX:44       region code : number, colored on a ramp from the smallest to the largest value
//   names=Group A,Group B    class names for the legend (groups form)
//   Both together: groups= sets the colors and values= with showvalues=1 prints a number on each tile.
// Form params (all optional):
//   layout=us                us = the built-in US grid (11 columns x 8 rows)
//   cells=AA:0:0,BB:1:0      your own grid instead: code:column:row for every tile (0-based, row 0 at the top)
//   shape=square             square | hex | circle (hex shifts every second row by half a tile)
//   gap=0.08                 space between tiles as a fraction of a tile
//   ramp=#dbeafe,#1d4ed8     values form: low color, high color (three colors make a diverging ramp)
//   min= max=                values form: the numbers the ramp ends stand for (default: the data's range)
//   empty=#e2e6ea            color of regions without data
//   codes=1 showvalues=0     write the region code on each tile; write the value under it
//   legend=bottom            bottom | top | none
// Layout note: the US grid was arranged by hand for this skill (a common tile-map arrangement); it carries no map data.
// Shared params (every chart script): box=x,y,w,h in % of the image (default 0,0,100,100); colors=#hex,#hex;
//   ink=#1f2933 (text) muted=#6b7785 (axis text) grid=#d5dbe1 bg=transparent; fs=label size in px;
//   font='Family, Fallback, sans-serif' (a family name: font classes such as @condensed do not work inside a script)
//   numfont= weight=400 numweight=700; prefix=$ suffix=% decimals=1 compact=1 (1.2K, 3.4M)
// The result's logs hold one JSON line: the tile size and an anchor at the center of every tile.
