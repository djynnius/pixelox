// waffle.js: unit charts from counts: a waffle (10 x 10 squares = 100%), an icon array of people ("4 out of 5"),
// a dot matrix, or a seat grid. One unit = one thing; exactly the given number of units is drawn.
// Run:   pixelox render js scripts/waffle.js -o waffle.png --size 1080x1350 --param box=10,30,80,40 --param values=62,23,15 --param labels='Agree,Unsure,Disagree'
// Place: pixelox comp place d.pxo waffle.png --in-place --as Waffle --at 0,0
// Data params (use one form):
//   values=62,23,15          units per group, filled in order (a group per color)
//   value=4 total=5          one share: 'value' units in the first color, the rest in the 'empty' color (value may be 3.5)
//   labels=A,B,C             group names for the legend
// Form params (all optional):
//   cols=10                  units per row (rows follow from the total)
//   shape=square             square | circle | hex | person
//   gap=0.14                 space between units as a fraction of a unit
//   from=top                 top | bottom: the row where filling starts (bottom is the classic waffle)
//   order=rows               rows | cols: fill row by row, or column by column (groups become side-by-side blocks)
//   empty=#d5dbe1            color of the unfilled units (value/total form)
//   hollow=                  1-based group numbers drawn as outlines (vacant, undecided, up for election)
//   layout=grid              grid | rows (one row of dots per group, its name on the left and its count on the
//                            right: a dot-row chart for small counts, up to about 30 per row)
//   align=center             center | left: where the grid sits in the box
//   counts=1 legend=bottom   counts after the names in the legend; bottom | top | none
// Shared params (every chart script): box=x,y,w,h in % of the image (default 0,0,100,100); colors=#hex,#hex;
//   ink=#1f2933 (text) muted=#6b7785 (axis text) grid=#d5dbe1 bg=transparent; fs=label size in px;
//   font='Family, Fallback, sans-serif' (a family name: font classes such as @condensed do not work inside a script)
//   numfont= weight=400 numweight=700; prefix=$ suffix=% decimals=1 compact=1 (1.2K, 3.4M)
// The result's logs hold one JSON line: the grid rectangle, the unit size, and an anchor at the middle of every group.
