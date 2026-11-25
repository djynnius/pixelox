// pyramid.js: a pyramid of equal-height tiers (hierarchy, levels, priorities) or a funnel whose tier widths
// follow the values (drop-off between stages), with labels inside the tiers or beside them on leaders.
// Run:   pixelox render js scripts/pyramid.js -o pyramid.png --size 1080x1350 --param box=8,28,84,44 --param labels='Vision|Strategy|Projects|Daily tasks' --param notes='Why we exist|Where we play|What we build|What we do today'
// Place: pixelox comp place d.pxo pyramid.png --in-place --as Pyramid --at 0,0
// Data params:
//   labels=A|B|C|D           tier names from the top tier down
//   notes=...|...            a short note per tier (side labels only)
//   values=1000,620,240,80   funnel: one number per tier (widths are proportional; the first is the widest)
// Form params (all optional):
//   kind=pyramid             pyramid | funnel
//   flip=0                   pyramid only: 1 = point down
//   labelpos=right           right | left (name and note beside each tier, on a leader) | in (name inside the tier)
//   gap=6                    space between tiers in px
//   width=0.5                side labels: the share of the box width the shape takes
//   showvalues=1             funnel: write each value; pct=1 adds its share of the first tier
//   titlesize=0              tier name size in px (0 = 1.15 x fs); labelweight=700 weight of names inside tiers
//   line=#c3cad2             leader color
// Shared params (every chart script): box=x,y,w,h in % of the image (default 0,0,100,100); colors=#hex,#hex;
//   ink=#1f2933 (text) muted=#6b7785 (axis text) grid=#d5dbe1 bg=transparent; fs=label size in px;
//   font='Family, Fallback, sans-serif' (a family name: font classes such as @condensed do not work inside a script)
//   numfont= weight=400 numweight=700; prefix=$ suffix=% decimals=1 compact=1 (1.2K, 3.4M)
// The result's logs hold one JSON line: the shape rectangle and an anchor (canvas px and %) at the center of every tier.
