// donut.js: part-of-a-whole charts from data, with labels: donut, pie, half donut, arc band, gauge, progress
// ring, and tick ring. Angles are proportional to the values. Output is a transparent image.
// Run:   pixelox render js scripts/donut.js -o donut.png --size 1080x1350 --param box=10,30,80,40 --param values=45,25,20,10 --param labels='Online,Stores,Partners,Other' --param center=120K --param sub='orders'
// Place: pixelox comp place d.pxo donut.png --in-place --as Donut --at 0,0
// Data params:
//   values=45,25,20,10       one number per slice (any unit: shares are computed)
//   labels=A,B,C,D           slice names (use '|' between names that contain commas)
//   value=67 max=100         kind=ring only: one value on a track
// Form params (all optional):
//   kind=donut               donut | ring (one value on a pale track: progress ring, gauge) | rings (see below)
//   inner=0.62               hole radius as a fraction of the outer radius (0 = pie; 0.8 = thin ring)
//   sweep=360 start=0        degrees covered and where the first slice starts, clockwise from 12 o'clock
//                            (sweep=180 gives a half donut and starts at 9 o'clock unless start is given)
//   labelpos=out             out (name and number beside each slice, with a leader) | in (number inside the slice)
//                            | legend (names in a legend, numbers inside) | none
//   show=pct                 pct | value | both | none: the number written for each slice
//   center= sub=             big text and small text in the hole (ring: center defaults to the value)
//   centersize=              size of the center text in px (default: fitted to the middle)
//   gap=0 sep=#ffffff        degrees left open between slices; color of the hairline between slices (sep=none)
//   ticks=0                  N > 0 draws the ring as N radial ticks colored by share (tick ring)
//   round=1 trackcolor=      ring only: round ends; the track color (default: the color mixed toward white)
//   minlabel=4               slices smaller than this percent get no inside number
//   radii=1,0.8,0.6          length of each slice as a fraction of the ring's depth (1 = out to the full radius,
//                            0.5 = halfway from the hole's edge): a polar-area ("rose") chart whose slices also
//                            differ in length. Angles still carry the values
//   gappx=0 corner=0         a parallel gap between slices in px, and rounded slice corners in px (petals, fan panels)
//   kind=rings               several values as concentric arcs on one center, each on its own track (max=100;
//                            sweep up to 300); names and numbers sit at the arcs' starts
//   cx= cy= r=               place by center instead of box: center x in % of the width, center y in % of the height,
//                            outer radius in % of the width (for rings around a picture, or two rings on one center)
// Shared params (every chart script): box=x,y,w,h in % of the image (default 0,0,100,100); colors=#hex,#hex;
//   ink=#1f2933 (text) muted=#6b7785 (axis text) grid=#d5dbe1 bg=transparent; fs=label size in px;
//   font='Family, Fallback, sans-serif' (a family name: font classes such as @condensed do not work inside a script)
//   numfont= weight=400 numweight=700; prefix=$ suffix=% decimals=1 compact=1 (1.2K, 3.4M)
// The result's logs hold one JSON line: the center, the radii, and an anchor (canvas px and %) in the middle of every slice.
