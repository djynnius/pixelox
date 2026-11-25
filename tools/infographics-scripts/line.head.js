// line.js: line and area charts from data, with axis, gridlines, markers, and labels: one or several series,
// filled areas, stacked areas, or a bare sparkline. Output is a transparent image.
// Run:   pixelox render js scripts/line.js -o line.png --size 1080x1350 --param box=8,30,84,34 --param values='12,18,17,26,31,44' --param labels='2020,2021,2022,2023,2024,2025'
// Place: pixelox comp place d.pxo line.png --in-place --as Line --at 0,0
// Data params:
//   values=12,18,17,26       one series; several series are separated by ';'
//   labels=2020,2021,2022    x-axis names (one per point; long sets are thinned automatically)
//   series=Online,In store   series names: drawn at the line ends (endlabels=1) or in a legend
// Form params (all optional):
//   area=0                   0 = lines; 1 = each line filled down to the axis (soft); stack = stacked areas
//   smooth=0                 1 = rounded curve through the points (keep 0 when exact values matter)
//   markers=1                dots on the points (turned off above 24 points)
//   vlabels=last             last | all | none: which points get their value written
//   endlabels=1              series names at the right ends of the lines (0 = use the legend)
//   min=0 max=0              value axis range (max=0: a round number above the data); a min other than 0 must be stated on the chart
//                            (data below zero: the axis starts at a round number under it and zero is a line). Series may have
//                            different lengths (a year without data yet): each ends at its own last point.
//   ticks=4 axis=1 gridlines=1 legend=top
//   width=4                  line width in px (scales with the box)
//   spark=0                  1 = sparkline: no axis, no grid, no labels except the last value
// Shared params (every chart script): box=x,y,w,h in % of the image (default 0,0,100,100); colors=#hex,#hex;
//   ink=#1f2933 (text) muted=#6b7785 (axis text) grid=#d5dbe1 bg=transparent; fs=label size in px;
//   font='Family, Fallback, sans-serif' (a family name: font classes such as @condensed do not work inside a script)
//   numfont= weight=400 numweight=700; prefix=$ suffix=% decimals=1 compact=1 (1.2K, 3.4M)
// The result's logs hold one JSON line: the plot rectangle and an anchor (canvas px and %) for every point.
