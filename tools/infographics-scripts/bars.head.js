// bars.js: columns and bars from data, with labels: single, grouped, stacked, 100% stacked, a line on a second
// axis (combo), progress bars on tracks, and two-sided (butterfly) bars. Bars always start at zero.
// Run:   pixelox render js scripts/bars.js -o bars.png --size 1080x1350 --param box=8,30,84,36 --param values=42,31,18,9 --param labels='North,South,East,West' --param suffix=%
// Place: pixelox comp place d.pxo bars.png --in-place --as Bars --at 0,0
// Data params:
//   values=42,31,18,9        one series; several series are separated by ';' (values='12,18,25;9,14,20')
//   labels=A,B,C,D           category names (use '|' between names that contain commas)
//   series=2024,2025         series names: turns the legend on
// Form params (all optional):
//   dir=v                    v = columns, h = horizontal bars (use h for long names or more than 7 categories)
//   mode=group               group | stack | percent (stack scaled to 100%) | diverge (two series as left and right
//                            bars from a middle column of names: a butterfly chart)
//   max=0                    top of the value axis (0 = a round number above the data); ticks=4 divisions
//   axis=1 gridlines=1       value axis labels and gridlines (columns: on; bars: off; give axis=1 to turn on)
//   vlabels=1                value labels: 1 (at the bar end), inside (bars), 0
//   highlight=0 accent=#f97316   1-based category drawn in the accent color; the others take the first color
//   track=0 trackcolor=      1 = progress style: every bar lies on a full-length pale track (set max=100)
//   gap=0.36 radius=3        gap as a fraction of each band; corner radius of the bar end in px
//   line= linename= linecolor=#f2b705 linemax=0 linesuffix= markers=0   columns only: a line on its own right-hand axis
//   legend=top               top | bottom | none
//   labelwidth=0.34          bars only: the most the category names may take, as a fraction of the box width
//   labelpos=left            bars only: left (names beside the bars) | above (each name on its own line over its bar:
//                            full-width bars) | inside (names inside the bars)
// Columns may go below zero (values=12,-5,8): the axis then runs from a round negative number and zero is a line.
// Shared params (every chart script): box=x,y,w,h in % of the image (default 0,0,100,100); colors=#hex,#hex;
//   ink=#1f2933 (text) muted=#6b7785 (axis text) grid=#d5dbe1 bg=transparent; fs=label size in px;
//   font='Family, Fallback, sans-serif' (a family name: font classes such as @condensed do not work inside a script)
//   numfont= weight=400 numweight=700; prefix=$ suffix=% decimals=1 compact=1 (1.2K, 3.4M)
// The result's logs hold one JSON line: the plot rectangle and an anchor (canvas px and %) for every bar end.
