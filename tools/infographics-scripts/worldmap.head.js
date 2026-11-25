// worldmap.js: a dot-matrix world map: land drawn as a grid of dots (5-degree cells), continents tinted by
// name, and markers or value bubbles at longitude and latitude. A stylized locator, not an accurate map:
// use it to say "where in the world", never to compare areas or show borders.
// For real country outlines and a globe use geo.js (pixelox 0.6.0 or later); this script runs on any build.
// Run:   pixelox render js scripts/worldmap.js -o world.png --size 1080x1350 --param box=6,30,88,32 --param highlight='europe:#2563eb,africa:#f97316' --param points='-74:40.7:City A,2.3:48.9:City B,139.7:35.7:City C'
// Place: pixelox comp place d.pxo world.png --in-place --as World --at 0,0
// Data params (all optional):
//   highlight=europe:#2563eb   continent:color pairs; continents: namerica, samerica, europe, africa, asia, oceania
//   points=lon:lat:label:value  markers, comma separated; lon -180..180 (west negative), lat -55..85 (south negative);
//                              label and value are optional; with values the marker areas are proportional to them
// Form params (all optional):
//   dot=#c5ccd3              color of the land dots
//   shape=circle             circle | square
//   fill=0.74                dot size as a fraction of the cell
//   pointcolor=#e11d48       marker color; maxr=0.05 largest bubble radius as a fraction of the map width
//   pointlabels=1            write each marker's label beside it
//   halo=#ffffff             outline color that keeps labels readable over the dots: set it to your background color (halo=none: off)
// Data note: the land grid in this file was drawn by hand for this skill at 5-degree resolution (no third-party
//   dataset; coastlines are approximate, small islands are missing).
// Shared params (every chart script): box=x,y,w,h in % of the image (default 0,0,100,100); colors=#hex,#hex;
//   ink=#1f2933 (text) muted=#6b7785 (axis text) grid=#d5dbe1 bg=transparent; fs=label size in px;
//   font='Family, Fallback, sans-serif' (a family name: font classes such as @condensed do not work inside a script)
//   numfont= weight=400 numweight=700; prefix=$ suffix=% decimals=1 compact=1 (1.2K, 3.4M)
// The result's logs hold one JSON line: the map rectangle, the cell size, and an anchor for every marker and continent.
