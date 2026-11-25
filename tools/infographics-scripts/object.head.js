// object.js: a metaphor object drawn from shapes and cut into colored bands, one band per step or part: a rocket,
// a light bulb, a pencil, a thermometer, or a battery. With value= it becomes a picture gauge filled from the
// bottom. The script draws the object and the band numbers; put titles and notes beside it with text add, at
// the anchors it logs. Output is a transparent image.
// Run:   pixelox render js scripts/object.js -o rocket.png --size 1080x1080 --param box=34,8,32,84 --param kind=rocket --param bands=4 --param colors='#ff5a7a,#8b5cf6,#2dd4bf,#fbbf24'
// Place: pixelox comp place d.pxo rocket.png --in-place --as Rocket --at 0,0
// Params (all optional):
//   kind=rocket              rocket | bulb | pencil | thermometer | battery
//   bands=4                  number of colored bands, counted from the top (rocket: band 1 is the nose cone)
//   badges=01,02,03          text on each band (default 01, 02, ...; badges=none for plain bands)
//   value= max=100           gauge form: fill the object from the bottom to value/max in the first color;
//                            the rest takes the 'empty' color (bands and badges are then ignored)
//   empty=#e2e6ea            gauge form: color of the unfilled part
//   aspect=1                 width factor: 1.4 gives a stout rocket or a fat pencil, 0.8 a slim one
//   outline=0                1 = no bands: the body is drawn as an outline in the first color (glass= fills it):
//                            a line-art bulb or rocket to use as a hub or an icon
//                            (outlinewidth=0.055: line weight as a fraction of the object's unit; 0.075 is bold)
//   rotate=0                 degrees clockwise (rocket at 30 = climbing to the right; pencil at 90 = lying down)
//   shade=1                  a soft highlight down one side (0 = flat)
//   badgecolor=              color of the band numbers: one hex, or one per band (default: white or ink, by contrast)
//   flame=1 window=0         rocket: exhaust flame; a porthole on band 2 (windowsize=0.15 of the body height unit,
//                            windowring=#e5edf5, windowglass=#7cc4f2)
//   fincolor=                rocket: fin color (default: a darker shade of band 2)
//   rays=0                   bulb: short rays around the glass
//   dark=#374151             color of the dark parts (nozzle, bulb base, pencil lead, outline of the gauge forms)
// The object keeps its proportions and is fitted into box= (height first; a tall narrow box suits all kinds).
// Shared params (every chart script): box=x,y,w,h in % of the image (default 0,0,100,100); colors=#hex,#hex;
//   ink=#1f2933 (text) muted=#6b7785 (axis text) grid=#d5dbe1 bg=transparent; fs=label size in px;
//   font='Family, Fallback, sans-serif' (a family name: font classes such as @condensed do not work inside a script)
//   numfont= weight=400 numweight=700; prefix=$ suffix=% decimals=1 compact=1 (1.2K, 3.4M)
// The result's logs hold one JSON line with an anchor per band: its center (x, y) and the points on its left and
// right edges (lx, ly, rx, ry), in canvas px, for leader lines and labels.
