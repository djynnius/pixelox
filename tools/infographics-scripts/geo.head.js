// geo.js: a real world map or a globe (d3-geo): countries highlighted, colored by class or by value, with
// markers or value bubbles at longitude and latitude. Needs pixelox 0.6.0 or later (bundled d3 and world atlas)
// and is run with 'render d3'. On older builds use worldmap.js (the dot-matrix world) with the same points=.
// Run:   pixelox render d3 scripts/geo.js -o world.png --size 1080x1350 --param box=5,28,90,36 --param highlight='BRA,IND,NGA' --param points='-46.6:-23.5:City A,77.2:28.6:City B'
// Place: pixelox comp place d.pxo world.png --in-place --as World --at 0,0
// Data params (all optional; countries are named by ISO 3166 three-letter codes: USA, BRA, FRA, NGA, IND, CHN ...):
//   highlight=BRA,IND        these countries take the accent color (or give each its own: highlight='BRA:#0d9488,IND:#f97316')
//   groups=USA:1,BRA:2       country : 1-based class number (class colors from colors=, names from names=)
//   values=USA:12,BRA:30     country : number, colored on ramp= (bins=5 for classes with a stepped legend)
//   points=lon:lat:label:value   markers, comma separated; west and south negative; label and value optional;
//                            with values the marker AREAS are proportional to them
// Form params (all optional):
//   projection=equalearth    equalearth | natural | mercator | globe (an orthographic half sphere)
//   rotate=0,0               globe: the longitude,latitude at its center (rotate=20,10 centers on Africa);
//                            flat maps: the central longitude (rotate=150 puts the Pacific in the middle)
//   extent=                  flat maps: zoom to a box, lon0,lat0,lon1,lat1 (Europe: extent=-12,35,32,71)
//   antarctica=0             1 = draw Antarctica (flat maps)
//   land=#c5ccd3 ocean=      land color; ocean color (default none on flat maps, a pale blue on the globe)
//   stroke=#ffffff strokewidth=0.6   country borders (stroke=none: land as one shape)
//   graticule=0              1 = meridians and parallels every 20 degrees (on by default on the globe)
//   shade=1                  globe: a soft highlight and shadow that make it a sphere
//   accent=#f97316 ramp=#dbeafe,#1d4ed8 bins=0 min= max= names= legend=bottom   as in usmap.js
//   pointcolor=#e11d48 maxr=0.05 pointlabels=1 halo=#ffffff   markers and their labels, as in worldmap.js
// Map data: Natural Earth 1:110m (public domain) through world-atlas, bundled with pixelox; boundaries are
//   Natural Earth's de facto lines.
// Shared params (every chart script): box=x,y,w,h in % of the image (default 0,0,100,100); colors=#hex,#hex;
//   ink=#1f2933 (text) muted=#6b7785 (axis text) grid=#d5dbe1 bg=transparent; fs=label size in px;
//   font='Family, Fallback, sans-serif' (a family name: font classes such as @condensed do not work inside a script)
//   numfont= weight=400 numweight=700; prefix=$ suffix=% decimals=1 compact=1 (1.2K, 3.4M)
// The result's logs hold one JSON line: the map rectangle and an anchor for every marker and every named country.
