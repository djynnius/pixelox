// usmap.js: a real map of the US states (Albers USA, with Alaska and Hawaii as insets) colored by class or by
// value (a choropleth), with state labels, value bubbles, and a legend. Needs pixelox 0.6.0 or later (bundled
// d3 and US atlas) and is run with 'render d3'. On older builds use tilemap.js with the same groups=/values=.
// Run:   pixelox render d3 scripts/usmap.js -o map.png --size 1080x1350 --param box=5,28,90,46 --param values='CA:31,TX:44,FL:12,NY:27' --param bins=5
// Place: pixelox comp place d.pxo map.png --in-place --as Map --at 0,0
// Data params (all optional; use groups or values for the fill):
//   groups=CA:1,TX:2,OH:3    state code : 1-based class number (class colors from colors=, names from names=)
//   values=CA:31,TX:44       state code : number, colored on ramp= from the smallest to the largest value
//   names=Group A,Group B    class names for the legend (groups form)
//   highlight=CA,TX,NY       only these states take the accent color; all others take the 'empty' color
//   bubbles=CA:31,TX:44      a circle on each listed state, its AREA proportional to the number
// Form params (all optional):
//   bins=0                   values form: 0 = a smooth ramp; 3 to 7 = that many classes between round limits, with a
//                            stepped legend (the count can differ by one when the data's range is awkward)
//   breaks=10,20,30          values form: your own class limits instead of bins (4 classes here)
//   ramp=#dbeafe,#1d4ed8     low color, high color (three colors make a diverging ramp); min= max= fix its ends
//   empty=#e2e6ea            fill of states without data; accent=#f97316 for highlight=
//   stroke=#ffffff strokewidth=1.2   state borders (set stroke to your ground color)
//   outline=                 a color draws the country's outer outline as well (coast and land borders)
//   labels=codes             codes | values | both | none: what is written on each state
//                            (labels turn dark on light and mid-tone fills and white on dark ones; keep the two
//                            middle classes of a ramp clearly lighter or darker than mid-gray so both stay readable)
//   smalls=1                 list the small north-eastern states in a column beside the map (0 = leave them unlabeled)
//   bubblecolor=#ffffff bubbleline= maxr=0.07 bubblelabels=1   bubble fill, outline color (default: ink), largest radius
//                            as a fraction of the map width, and whether the number is written in each bubble
//   legend=bottom            bottom | top | none
// Map data: U.S. Census Bureau cartographic boundaries (public domain) through us-atlas, bundled with pixelox.
// Shared params (every chart script): box=x,y,w,h in % of the image (default 0,0,100,100); colors=#hex,#hex;
//   ink=#1f2933 (text) muted=#6b7785 (axis text) grid=#d5dbe1 bg=transparent; fs=label size in px;
//   font='Family, Fallback, sans-serif' (a family name: font classes such as @condensed do not work inside a script)
//   numfont= weight=400 numweight=700; prefix=$ suffix=% decimals=1 compact=1 (1.2K, 3.4M)
// The result's logs hold one JSON line: the map rectangle and an anchor (canvas px and %) at every state's center.
