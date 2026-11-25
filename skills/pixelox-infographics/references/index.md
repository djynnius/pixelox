# Infographic reference index

<!-- BEGIN GENERATED: count -->
55 infographics.
<!-- END GENERATED: count -->
Each ID is a studied infographic with a full description in [infographics/](infographics/):
its data story, modules, grid in percent, layer stack, palette with what each color encodes,
type table, charts with their parameters, and a tested pixelox build recipe. The infographics
themselves are not included: the descriptions are the reference. A file is named
`infographics/<ID>-<slug>.md`; section 5 has one linked row per infographic.

You do not need to read this whole file. Use section 1 to get candidates from the brief, or
section 2 from the data; check section 3 for a ready combination; then open only the 2 to 4
files you picked.

## 1. Start here: pick by what the brief is for

IDs marked `+` need a photo, a cutout, or an illustration from the user; all others are built
from data, type, icons, and generated shapes only.

| The brief is for | Start with | Then consider |
| --- | --- | --- |
| Election, parliament, or vote results | i003, i004, i022 | i001 (dense briefing sheet), i018 (hex cartogram), i014 (bubble map), i054 (how a voting system works) |
| One chart for an article or a post ("visualize this data") | i002 | i017 (fan banner), i052 (half fan), i018 (a map as the chart) |
| KPI report, dashboard, one-pager | i025, i042, i044 | i031 (dark neon kit), i047 (one-hue kit), i024 (three charts, no cards) |
| Long explainer of one topic with statistics | i035, i011, i013 | i023, i037, i042 |
| How-to, process, steps | i008, i021, i010 | i007, i053, i012, i050, i032+ |
| A repeating cycle or loop | i033, i030 | i049 |
| Timeline, roadmap, history | i041, i024 | i027+ (radial timeline), i050 (road) |
| Start-up, growth, launch, goals (metaphor) | i038, i020, i039 | i040, i041, i046 |
| Ideas, creativity, learning (metaphor) | i019, i026, i036 | i012 |
| Comparison, overlap, either-or | i043, i048, i029 | i030, i052 |
| Levels, hierarchy, priorities | i016 | i045+ (stacked ribbons) |
| Sports statistics, player or team profile, ranking | i009+, i034+, i027+ | i051 |
| Editorial statistics with a photo | i028+ | i009+ |
| Geography: values by state, country, region | i014, i015, i018 | i054, i013, i004, i006 (globe) |
| Features, options, facets of one thing | i049, i052, i036 | i026, i051, i017, i045+ |
| Health, food, nutrition | i023, i029 | i013, i055 |
| Education, classroom explainer | i054, i037, i012 | i044, i011 |
| Science, environment, resources | i005, i006 | i013 |
| A labeled figure or object (anatomy) | i055, i038 | i051, i019 |

## 2. Start from the data

When the brief is only data, name its relationship first, then look up designs that show it.

<!-- BEGIN GENERATED: by-data -->
| The data is | Count | IDs |
| --- | --- | --- |
| text-only | 32 | i007 i008 i010 i012 i015 i016 i017 i019 i020 i021 i026 i028 i030 i032 i033 i036 i037 i038 i039 i040 i041 i043 i044 i045 i046 i048 i049 i050 i051 i052 i053 i055 |
| process | 17 | i007 i008 i010 i012 i020 i021 i024 i032 i033 i038 i039 i040 i041 i046 i050 i053 i054 |
| comparison | 16 | i001 i005 i006 i009 i013 i014 i018 i023 i024 i025 i031 i035 i047 i048 i049 i052 |
| part-to-whole | 16 | i001 i002 i003 i004 i005 i009 i011 i022 i023 i024 i025 i029 i031 i035 i042 i047 |
| single-stat | 12 | i003 i009 i011 i013 i014 i022 i028 i035 i037 i041 i042 i044 |
| counted-units | 10 | i001 i003 i004 i006 i011 i018 i022 i027 i044 i054 |
| geography | 7 | i004 i006 i013 i014 i015 i018 i054 |
| change-over-time | 6 | i002 i025 i027 i028 i031 i047 |
| relationship | 5 | i026 i030 i036 i043 i051 |
| ranking | 4 | i005 i015 i027 i034 |
| cycle | 2 | i030 i033 |
| hierarchy | 1 | i016 |
<!-- END GENERATED: by-data -->

Which chart suits which relationship: the chooser table at the top of [charts.md](charts.md).

## 3. Combine: worked remixes

Take the **structure** from one, the **palette and type** from another, the **chart or
illustration idea** from a third. Combinations that work, and why:

| Brief | Structure from | Palette and type from | Chart or idea from | Why it works |
| --- | --- | --- | --- | --- |
| Quarterly sales, 5 regions, no other material | i002 (one chart card: accent bar, finding as title, source) | i035 (navy, violets, lilac) | C-bars with `highlight=1` | One finding, one chart; a single hue family looks deliberate, the accent marks the story |
| City council results, 45 seats, 4 parties | i003 (ring hero, stat row under it) | i003 with the parties' own colors | C-hemicycle, C-stat-row | Seats are counted things: dots let readers count the majority |
| "How our onboarding works", 5 steps | i008 (spine, alternating sides) | i038 (white, four candy hues) | C-process-spine with icons at the anchors | A spine gives each step room for a sentence; one hue per step numbers them twice |
| Annual report one-pager, 6 KPIs | i025 (cards in a grid) | i035 (one hue family) | C-progress-ring, C-sparkline, C-stat-block | Cards make unrelated metrics equal; rings and sparklines add shape to plain numbers |
| Product launch plan | i038 (object with callouts) | i003 (dark ground, cream accents) | C-rocket, C-timeline under it | The object is the list; a timeline adds the dates the metaphor cannot carry |
| Survey: 62% agree | i035 (one chapter only, as a square) | i008 (one loud ground color, black type) | C-waffle or C-icon-array | A single share is strongest as counted units with the number as the title |
| Rates by US state, with three takeaways | i014 (map as hero, notes in the margins) | i035 (one hue family, light ground) | C-us-choropleth with `bins=5`, C-stat-row under it | The map answers "where"; three big numbers answer "so what" |
| Club season review, no photos | i042 (row of stat cards over a row of rings) | i003 (dark ground, two team colors, cream accent) | C-dot-rows, C-progress-ring | Cards carry the headline numbers; dot rows make small counts countable |

### When nothing above fits

1. **Material first.** With no photo, skip the IDs marked `+` (or section 6, "By what you must
   supply": the `nothing` row).
2. **Then the data.** Name the relationship (section 2) and take the designs listed for it.
3. **Then the format.** A phone feed wants square or portrait; a report page portrait; a
   slide landscape; a story with 3 or more modules a long page (section 6, "By format").
4. **Then the mood** (section 6, "By mood") to choose between the candidates left.
5. Open 2 to 4 files. Take the structure from the one whose modules match your content count,
   not from the prettiest.

Rules for a remix: one structure, one palette, one hero chart or object. If two picks both
want to be the hero (a seat chart and a rocket), drop one. Never rebuild one reference as it
is: change the data, the copy, and the palette or the type.

## 4. Structure archetypes

Zones are percent of the canvas height unless marked W.

| ID | Use it for | Zones |
| --- | --- | --- |
| `S-chart-card` | one chart that carries one finding | title block 6 to 22; chart 24 to 86; source line 91 to 96. Margins 7% W |
| `S-long-bands` | a topic told in 3 to 6 short chapters | dark header 0 to 14; then per chapter a full-width band (2) and a module (13 to 15) on alternating grounds; footer 5 |
| `S-long-flow` | a story told down one continuous ground | title art 0 to 18; 3 to 5 modules of unequal height separated by diagonals, curves, or 4% of space, each one stat with its picture; footer 4 |
| `S-results-sheet` | seats and votes | title 4 to 11; hero seat chart 12 to 60; supporting module 62 to 84; stat row 85 to 92; notes 93 to 98 |
| `S-dashboard-grid` | 4 to 9 unrelated metrics | title 4 to 12; cards in 2 or 3 columns from 14 to 94, gutter 2.5% W; each card a title and one chart |
| `S-spine-steps` | 4 to 8 steps with a sentence each | title 4 to 12; a vertical spine at 50% W from 16 to 92; a node per step; text alternating left and right |
| `S-row-steps` | 3 to 7 short steps | title 6 to 16; N equal columns from 25 to 88; node or number on top, title, note |
| `S-cycle-ring` | a loop of 3 to 9 steps | title in a corner or on top; a ring centered at 55, radius 28 to 32% W; labels outside; a picture or word in the middle |
| `S-hub-spokes` | one topic, 4 to 8 facets | a hub at the center; items at a radius of 30 to 38% W with connectors; each an icon, a title, a note |
| `S-object-callouts` | steps or parts shown on a metaphor object | the object on the center axis, 20 to 35% W wide, 70 to 80 tall; callouts alternate in left and right columns on leaders |
| `S-map-callouts` | values by place | title band on top; the map takes 60 to 75% of the area; value badges on it; legend and explainer in a side column or below |
| `S-stat-poster` | a person, team, or product profile in numbers | a cutout photo 40 to 55% W along one side or the center; stat modules in the remaining columns; a title bar on top |
| `S-overlap-diagram` | what things share | title 5 to 12; a Venn or overlapping shapes centered from 15 to 88; notes outside each shape |
| `S-pyramid-notes` | levels | a pyramid or funnel 45 to 50% W wide; notes left and right on leaders; title on top |
| `S-icon-sheet` | many small facts in sections | header; 3 to 5 sections, each a ruled label and a row of 2 to 4 icon-and-caption cells |
| `S-object-rows` | steps or parts listed beside a metaphor object | the object in the left 30 to 45% W, full height; one row per band at the band's height to its right (rule or bar, number, title, note); an optional summary box under the rows |
| `S-half-fan` | 3 to 7 options or shares fanned over a baseline | a hub or half ring on the lower center (center at 50% W, 62 to 80 H); segments over the upper half; labels above and beside the fan; title on top |
| `S-briefing-sections` | a dense analysis sheet | a full-width title and deck 0 to 9; a hero row of twin charts to 33; then ruled, titled full-width sections, each a strip chart, a grid of small profiles, or a row of facts; sources 96 to 100 |
| `S-stat-cards-row` | 3 to 5 headline numbers with one supporting row | title 0 to 28; a row of tall stat cards 28 to 67 (icon, big number, caption); a row of round modules or small charts 72 to 98 |
| `S-radial-hero` | a profile told around a picture | a cutout hero breaking through a ring of radial entries (ring center 50% W, 45 H, radius 34 to 40% W); a stat band 83 to 97 |
| `S-kit-sheet` | several unrelated charts shown as one set | no cards: two stacked charts in a left column 62% W wide, a full-height timeline or list in a right column 34% W wide, one palette holding them together |
| `S-split-panels` | 2 to 4 parallel topics | the canvas cut into halves or quadrants of different grounds, one module per panel; one shape (a plate, a circle, a figure) overlapping the joins |

## 5. All infographics

<!-- BEGIN GENERATED: infographics -->
| ID | Title | Kind | Structure | Data | Charts | Format | Palette | Needs |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| [i001](infographics/i001-dense-analysis-sheet-twin-arcs.md) | Dense analysis sheet with twin double arcs | results, explainer | S-briefing-sections | part-to-whole, comparison, counted-units | C-half-donut, C-arc-label, C-legend, C-percent-bar, C-label-bars | portrait | `#ffffff` `#002b5e` `#cb2027` `#69769f` `#e28e76` | nothing |
| [i002](infographics/i002-combo-chart-card.md) | Chart card with stacked columns and a share line | statistical | S-chart-card | change-over-time, part-to-whole | C-combo, C-title-block, C-legend, C-callout, C-source-line | square | `#f4f9fc` `#13897c` `#80ded7` `#eac62d` `#1b2a3a` | nothing |
| [i003](infographics/i003-dark-two-chamber-results.md) | Dark two-chamber results sheet with a ring hero | results, statistical | S-results-sheet | counted-units, part-to-whole, single-stat | C-half-donut, C-hemicycle, C-tick-ring, C-arc-label, C-stat-row | long | `#122e39` `#0e2630` `#b3261c` `#1466a3` `#aeb4b3` | nothing |
| [i004](infographics/i004-light-ring-results-seat-map.md) | Light results sheet with a ring hero and a seat map | results, geographic | S-results-sheet | counted-units, part-to-whole, geography | C-half-donut, C-hemicycle, C-tick-ring, C-us-bubble-map, C-us-choropleth | long | `#e0e0e0` `#0d69a6` `#ca261c` `#13191b` `#b2b2b2` | nothing |
| [i005](infographics/i005-three-view-resource-spread.md) | Three-view resource spread with a radial hub | statistical, comparison | S-hub-spokes | comparison, ranking, part-to-whole | C-radial-bars, C-3d-bars, C-progress-ring, C-icon-array, C-table | landscape | `#f4f4f0` `#e2e5df` `#3391bd` `#189180` `#ec9379` | nothing |
| [i006](infographics/i006-globe-hud-callouts.md) | Dark globe with heads-up callouts | geographic, dashboard | S-hub-spokes | geography, counted-units, comparison | C-globe, C-icon-array, C-negative-columns, C-stat-block, C-callout | landscape | `#273947` `#2a86ad` `#ffffff` `#b9cf8a` `#f0a04e` | nothing |
| [i007](infographics/i007-serpentine-five-step-banners.md) | Serpentine ribbon through five nodes with side banners | process, list | S-spine-steps | process, text-only | C-process-spine, C-icon-badge, C-ribbon-label | square | `#f3f3f3` `#1b7fa0` `#234a63` `#6583b5` `#3f4c6b` | nothing |
| [i008](infographics/i008-yellow-spine-speech-bubbles.md) | Four steps on a spine with icon bubbles on yellow | process, list | S-spine-steps | process, text-only | C-process-spine, C-speech-bubble, C-icon-badge | portrait | `#fecb02` `#ffffff` `#1a1a1a` | nothing |
| [i009](infographics/i009-player-profile-stat-poster.md) | Player profile poster with a cutout and stat modules | statistical, dashboard | S-stat-poster | single-stat, comparison, part-to-whole | C-grouped-columns, C-bars, C-progress-ring, C-stat-block, C-ribbon-label | portrait | `#e6e5ea` `#b1073b` `#77a5ce` `#892a34` `#ef306c` | subject-cutout |
| [i010](infographics/i010-folded-ribbon-step-tabs.md) | Four folded ribbon tabs with alternating paper panels | process, list | S-spine-steps | process, text-only | C-ribbon-label, C-icon-grid, C-title-block | portrait | `#fdfdfd` `#1f9a2e` `#e88d06` `#1893d4` `#e84701` | nothing |
| [i011](infographics/i011-giant-eye-teal-explainer.md) | One-hue illustrated explainer around a giant eye | explainer, statistical | S-long-flow | single-stat, part-to-whole, counted-units | C-pie, C-icon-array, C-line, C-stat-block, C-title-block | portrait | `#08454a` `#028470` `#ffffff` `#02dab8` `#066e67` | nothing |
| [i012](infographics/i012-pastel-blob-doodle-steps.md) | Long pastel blob sheet with five doodle steps | process, explainer | S-long-flow | process, text-only | C-bulb, C-pencil, C-icon-badge | long | `#ffffff` `#8ecdec` `#ffecb3` `#fbe7f1` `#bee7e9` | nothing |
| [i013](infographics/i013-underwater-region-map-columns.md) | Underwater long sheet with a region map and columns | explainer, geographic | S-long-flow | geography, comparison, single-stat | C-us-choropleth, C-progress-ring, C-columns, C-title-block, C-source-line | long | `#a4d6f3` `#075ea3` `#1874b3` `#fd6100` `#ef8f01` | nothing |
| [i014](infographics/i014-navy-almanac-bubble-map.md) | Navy almanac map with value bubbles | geographic, statistical | S-map-callouts | geography, comparison, single-stat | C-us-bubble-map, C-ribbon-label, C-icon-badge, C-callout, C-source-line | landscape | `#1e375c` `#dcd4c4` `#fbf6f3` `#f7a65e` `#7fb069` | nothing |
| [i015](infographics/i015-numbered-map-list-sheet.md) | Numbered map with a matching list below | geographic, list | S-map-callouts | geography, ranking, text-only | C-us-choropleth, C-icon-badge, C-callout, C-title-block | portrait | `#9bd5cb` `#59baa7` `#ffffff` `#fece13` `#0b1f1c` | nothing |
| [i016](infographics/i016-faceted-blue-pyramid-tiers.md) | Faceted blue pyramid in three labelled tiers | hierarchy | S-pyramid-notes | hierarchy, text-only | C-pyramid | landscape | `#fcfcfb` `#009ad9` `#0090c9` `#32aee1` `#0a6ebd` | nothing |
| [i017](infographics/i017-nine-part-fan-banner.md) | Banner with a nine-part fan in a warm ramp | list | S-half-fan | text-only | C-half-donut, C-polar-area, C-icon-badge | banner | `#ffffff` `#35366a` `#ba3055` `#e44353` `#fe8663` | nothing |
| [i018](infographics/i018-hex-cartogram-seven-classes.md) | Hexagon cartogram in seven diverging classes | geographic, results | S-map-callouts | geography, counted-units, comparison | C-hex-map, C-legend | landscape | `#ffffff` `#2a378f` `#c80205` `#663599` `#78a4ea` | nothing |
| [i019](infographics/i019-pencil-five-arrow-branches.md) | Pencil splitting into five labeled arrows | list, explainer | S-object-callouts | text-only | C-pencil, C-arc-label, C-icon-grid | square | `#f2f2f2` `#faca09` `#e39b01` `#ca850d` `#b46613` | nothing |
| [i020](infographics/i020-flat-rocket-three-notes.md) | Flat three-band rocket with numbered notes | process, list | S-object-rows | process, text-only | C-rocket, C-callout | landscape | `#ffffff` `#e55360` `#43aad5` `#fdae49` `#466077` | nothing |
| [i021](infographics/i021-seven-gradient-step-columns.md) | Seven rainbow step columns with ring caps | process, list | S-row-steps | process, text-only | C-process-row, C-icon-badge | landscape | `#ffffff` `#fdb94a` `#ff6a1d` `#f93a75` `#0cc6ef` | nothing |
| [i022](infographics/i022-light-hex-seat-results.md) | Light results sheet with hexagon seats | results, statistical | S-results-sheet | counted-units, part-to-whole, single-stat | C-hemicycle, C-seat-grid, C-stat-block, C-title-block, C-source-line | square | `#ffffff` `#b9e2fa` `#f0d6d9` `#2d368d` `#cb0303` | nothing |
| [i023](infographics/i023-mascot-donut-belly-cream.md) | Cream food explainer with a donut-belly mascot | explainer, statistical | S-stat-poster | part-to-whole, comparison | C-donut, C-bars, C-icon-grid, C-callout, C-ribbon-label | portrait | `#efeed0` `#2e2d0e` `#abaa11` `#689306` `#2e4006` | nothing |
| [i024](infographics/i024-pie-columns-phase-timeline.md) | Three-module sheet with pie, pill columns, and phase timeline | statistical, timeline | S-kit-sheet | part-to-whole, comparison, process | C-polar-area, C-columns, C-timeline, C-speech-bubble, C-icon-badge | square | `#f6f6f6` `#ba007a` `#860062` `#005f41` `#abcdb4` | nothing |
| [i025](infographics/i025-seven-card-report-page.md) | Long report page of seven outlined chart cards | dashboard, statistical | S-dashboard-grid | part-to-whole, comparison, change-over-time | C-card, C-line, C-donut, C-stacked-columns, C-label-bars | long | `#f6f7fe` `#ffffff` `#0d1e48` `#3f2f96` `#7a74f2` | nothing |
| [i026](infographics/i026-bulb-hub-circuit-nodes.md) | Bulb hub with icon nodes on circuit arcs | list, explainer | S-hub-spokes | relationship, text-only | C-hub-spokes, C-bulb, C-icon-badge, C-title-block | square | `#f7f7f7` `#fd4f00` `#fe9c01` `#fece01` `#feed00` | nothing |
| [i027](infographics/i027-award-ring-hero-cutout.md) | Sixty-year award ring around a hero cutout | timeline, statistical | S-radial-hero | change-over-time, ranking, counted-units | C-radial-list, C-dot-rows, C-stat-row, C-title-block, C-source-line | portrait | `#ffffff` `#111111` `#dc8420` `#b03a3c` `#f5e5a3` | subject-cutout |
| [i028](infographics/i028-dark-portrait-stat-poster.md) | Dark editorial stat poster around a portrait | statistical, explainer | S-stat-poster | single-stat, change-over-time, text-only | C-stat-block, C-multi-line, C-callout, C-title-block, C-source-line | portrait | `#0d0d0d` `#161616` `#ffffff` `#e66a45` `#bfbfbf` | photo |
| [i029](infographics/i029-plate-pie-corner-callouts.md) | Plate cut into four parts with corner callouts | statistical, explainer | S-hub-spokes | part-to-whole | C-pie, C-callout, C-stat-block, C-title-block, C-source-line | landscape | `#ffffff` `#f89d2a` `#f36523` `#bf333e` `#605371` | nothing |
| [i030](infographics/i030-three-disc-trefoil-ring.md) | Three discs fused to a ring around a white hub | cycle, comparison | S-cycle-ring | cycle, relationship, text-only | C-cycle, C-cycle-segments, C-icon-badge | square | `#ffffff` `#6fa80a` `#05cdd6` `#474aa6` `#b9e955` | nothing |
| [i031](infographics/i031-neon-dashboard-element-kit.md) | Neon dashboard element kit on deep indigo | dashboard, statistical | S-dashboard-grid | comparison, part-to-whole, change-over-time | C-3d-bars, C-ring-gauge, C-progress-bars, C-bars, C-columns | landscape | `#251f43` `#1b1835` `#df0160` `#9d00e6` `#82ecfc` | nothing |
| [i032](infographics/i032-gradient-six-step-spine.md) | Six numbered discs on a spine over a pink-to-blue mesh | process, explainer | S-spine-steps | process, text-only | C-process-spine, C-title-block, C-icon-badge | long | `#c98cff` `#e277ff` `#8cbdfe` `#f5f5f5` `#5a17b5` | illustration |
| [i033](infographics/i033-nine-step-cycle-ring.md) | Nine numbered nodes on a thin ring with a center picture | cycle, process | S-cycle-ring | cycle, process, text-only | C-cycle, C-pyramid, C-title-block | square | `#f2f2f2` `#ffffff` `#4b43ba` `#2d9cdb` `#111111` | nothing |
| [i034](infographics/i034-top-five-zigzag-cutouts.md) | Top-five ranking poster with zig-zag cutouts | list, statistical | S-stat-poster | ranking | C-ranked-list, C-stat-block, C-title-block, C-source-line | portrait | `#3f1341` `#6d4b74` `#ffffff` `#fce247` `#0d3cf5` | photo, subject-cutout, logo |
| [i035](infographics/i035-banded-long-explainer.md) | Long banded explainer in navy and violet | explainer, statistical | S-long-bands | single-stat, comparison, part-to-whole | C-section-band, C-columns, C-progress-ring, C-stat-block, C-icon-grid | long | `#f2effd` `#01014f` `#6d3fd6` `#a341b0` `#c9c1ef` | nothing |
| [i036](infographics/i036-bulb-five-panel-fan.md) | Outline bulb with a fan of five panels | list, explainer | S-hub-spokes | text-only, relationship | C-petal-wheel, C-bulb, C-columns | landscape | `#ffffff` `#f7a83d` `#da4d44` `#50abb2` `#464254` | nothing |
| [i037](infographics/i037-gray-icon-sheet-label-bands.md) | One-color icon sheet with label bands | explainer, list | S-icon-sheet | text-only, single-stat | C-section-band, C-icon-grid, C-icon-array, C-stat-block, C-pie | portrait | `#f1f1f1` `#525252` `#ffffff` `#111111` `#828282` | nothing |
| [i038](infographics/i038-rocket-four-stage-callouts.md) | Rocket in four numbered bands with side callouts | process, anatomy | S-object-callouts | process, text-only | C-rocket, C-callout, C-icon-badge | square | `#ffffff` `#fd4e7d` `#7b5cf0` `#1fe0b3` `#ffb400` | nothing |
| [i039](infographics/i039-white-rocket-five-bar-rows.md) | White rocket with five numbered bars and a summary box | process, list | S-object-rows | process, text-only | C-rocket, C-section-band, C-icon-grid | square | `#f1f1f3` `#79138e` `#43398e` `#65a6ae` `#81af25` | nothing |
| [i040](infographics/i040-rocket-fanned-exhaust-columns.md) | Rocket over four fanned exhaust columns | process, list | S-object-callouts | process, text-only | C-rocket, C-world-map, C-callout, C-icon-badge | square | `#ffffff` `#46c4dc` `#7b9aac` `#f56328` `#ffc653` | nothing |
| [i041](infographics/i041-rocket-rainbow-exhaust-steps.md) | Rocket on a six-step rainbow exhaust column | process, list | S-object-rows | process, single-stat, text-only | C-pyramid, C-rocket, C-stat-block, C-callout | landscape | `#f9f9f9` `#f88030` `#fccc2f` `#c6e92e` `#50dae3` | nothing |
| [i042](infographics/i042-four-stat-cards-three-circles.md) | Four hanging stat cards over three circles | statistical, explainer | S-stat-cards-row | single-stat, part-to-whole | C-card, C-icon-badge, C-stat-block, C-pie, C-icon-array | portrait | `#ececec` `#f15b6c` `#f86fa5` `#47b4e9` `#b3c10a` | nothing |
| [i043](infographics/i043-four-circle-venn-charcoal.md) | Four-circle Venn on charcoal with a red core | comparison, explainer | S-overlap-diagram | relationship, text-only | C-venn, C-title-block | square | `#313038` `#c1c418` `#689dbc` `#b86f80` `#9c9c9e` | nothing |
| [i044](infographics/i044-coral-yellow-split-panels.md) | Coral and yellow split panels with ring, people, and bulb | explainer, statistical | S-split-panels | single-stat, counted-units, text-only | C-progress-ring, C-icon-array, C-picture-gauge, C-progress-bars, C-icon-badge | landscape | `#f7706f` `#fcbc52` `#ffffff` `#474c51` `#656563` | nothing |
| [i045](infographics/i045-folded-ribbon-list-host.md) | Five folded ribbons with icon discs and a host figure | list | S-stat-poster | text-only | C-icon-badge | portrait | `#f2f2f2` `#f2cd4b` `#ce344c` `#902671` `#68b1c0` | subject-cutout |
| [i046](infographics/i046-dark-space-rocket-paper-columns.md) | Dark space rocket over four folded paper columns | process, list | S-object-callouts | process, text-only | C-rocket, C-globe, C-icon-badge, C-callout | square | `#2a3131` `#8da5a1` `#fbd362` `#e29149` `#3ebbc0` | nothing |
| [i047](infographics/i047-one-hue-blue-chart-kit.md) | One-hue blue chart and diagram kit | dashboard, statistical | S-dashboard-grid | part-to-whole, comparison, change-over-time | C-donut, C-polar-area, C-process-row, C-progress-ring, C-grouped-columns | long | `#f4f4f4` `#a5d6ef` `#6dc5e5` `#2f9fd1` `#018fcc` | nothing |
| [i048](infographics/i048-three-circle-pinwheel-options.md) | Three interlocked circles around a numbered hub | comparison, list | S-overlap-diagram | comparison, text-only | C-venn, C-icon-badge | square | `#eaeaea` `#42aec5` `#238af1` `#015cd1` `#f7f7f7` | nothing |
| [i049](infographics/i049-five-petal-hub-wheel.md) | Five white petals around a hub with colored frames | list, comparison | S-hub-spokes | comparison, text-only | C-petal-wheel, C-donut, C-icon-badge | square | `#eef1f8` `#fdfdfd` `#06a8bd` `#00b97a` `#f9b401` | nothing |
| [i050](infographics/i050-serpentine-curve-five-step-civic.md) | Five steps on a serpentine curve in red, white, and blue | process, explainer | S-spine-steps | process, text-only | C-half-donut, C-arc-label, C-title-block | long | `#fbf6f8` `#ced2db` `#3b4b7f` `#dd181a` `#a7b7da` | nothing |
| [i051](infographics/i051-figure-hub-bracket-modules.md) | Figure hub with six bracketed icon modules | list, explainer | S-hub-spokes | text-only, relationship | C-icon-grid, C-callout, C-title-block | square | `#ffffff` `#f3841c` `#4d4f4e` `#5e5f61` `#f4eb3b` | nothing |
| [i052](infographics/i052-half-fan-four-options.md) | Half fan of four segments over a ringed hub | list, comparison | S-half-fan | comparison, text-only | C-half-donut, C-donut, C-icon-badge | landscape | `#ffffff` `#f15a27` `#f7b038` `#39bdc9` `#474d7f` | nothing |
| [i053](infographics/i053-dark-wave-step-sails.md) | Dark wave of five gradient step tabs and sails | process, list | S-row-steps | process, text-only | C-callout, C-icon-grid | landscape | `#1d2e3e` `#d3024a` `#fb9a12` `#0ac4a6` `#fc4f0a` | nothing |
| [i054](infographics/i054-arrow-loop-explainer-class-map.md) | Arrow-loop explainer around a three-class map | explainer, geographic | S-map-callouts | geography, process, counted-units | C-us-choropleth, C-icon-array, C-pie, C-stat-block, C-title-block | portrait | `#ffffff` `#e4e1e0` `#cf1c21` `#206fb6` `#b5b3b5` | nothing |
| [i055](infographics/i055-anatomy-figure-ring-callouts.md) | Anatomy figure with ring callouts and ribbon notes | anatomy, explainer | S-object-callouts | text-only | C-icon-array, C-callout, C-icon-badge, C-ribbon-label | square | `#daeff8` `#9cc1dd` `#304860` `#addfee` `#cd5843` | nothing |
<!-- END GENERATED: infographics -->

## 6. Lists

### By structure

<!-- BEGIN GENERATED: by-structure -->
| Structure | Count | IDs |
| --- | --- | --- |
| S-hub-spokes | 7 | i005 i006 i026 i029 i036 i049 i051 |
| S-object-callouts | 5 | i019 i038 i040 i046 i055 |
| S-spine-steps | 5 | i007 i008 i010 i032 i050 |
| S-stat-poster | 5 | i009 i023 i028 i034 i045 |
| S-map-callouts | 4 | i014 i015 i018 i054 |
| S-dashboard-grid | 3 | i025 i031 i047 |
| S-long-flow | 3 | i011 i012 i013 |
| S-object-rows | 3 | i020 i039 i041 |
| S-results-sheet | 3 | i003 i004 i022 |
| S-cycle-ring | 2 | i030 i033 |
| S-half-fan | 2 | i017 i052 |
| S-overlap-diagram | 2 | i043 i048 |
| S-row-steps | 2 | i021 i053 |
| S-briefing-sections | 1 | i001 |
| S-chart-card | 1 | i002 |
| S-icon-sheet | 1 | i037 |
| S-kit-sheet | 1 | i024 |
| S-long-bands | 1 | i035 |
| S-pyramid-notes | 1 | i016 |
| S-radial-hero | 1 | i027 |
| S-split-panels | 1 | i044 |
| S-stat-cards-row | 1 | i042 |
<!-- END GENERATED: by-structure -->

### By kind

<!-- BEGIN GENERATED: by-kind -->
| Kind | Count | IDs |
| --- | --- | --- |
| list | 22 | i007 i008 i010 i015 i017 i019 i020 i021 i026 i034 i036 i037 i039 i040 i041 i045 i046 i048 i049 i051 i052 i053 |
| explainer | 20 | i001 i011 i012 i013 i019 i023 i026 i028 i029 i032 i035 i036 i037 i042 i043 i044 i050 i051 i054 i055 |
| statistical | 19 | i002 i003 i005 i009 i011 i014 i022 i023 i024 i025 i027 i028 i029 i031 i034 i035 i042 i044 i047 |
| process | 15 | i007 i008 i010 i012 i020 i021 i032 i033 i038 i039 i040 i041 i046 i050 i053 |
| geographic | 7 | i004 i006 i013 i014 i015 i018 i054 |
| comparison | 6 | i005 i030 i043 i048 i049 i052 |
| dashboard | 5 | i006 i009 i025 i031 i047 |
| results | 5 | i001 i003 i004 i018 i022 |
| anatomy | 2 | i038 i055 |
| cycle | 2 | i030 i033 |
| timeline | 2 | i024 i027 |
| hierarchy | 1 | i016 |
<!-- END GENERATED: by-kind -->

### By topic

<!-- BEGIN GENERATED: by-topic -->
| Topic | Count | IDs |
| --- | --- | --- |
| generic | 33 | i002 i007 i008 i010 i012 i015 i017 i019 i020 i021 i024 i026 i028 i030 i031 i032 i033 i036 i037 i038 i039 i040 i041 i042 i043 i044 i045 i047 i048 i049 i052 i053 i055 |
| business | 32 | i001 i005 i007 i008 i010 i015 i016 i017 i020 i021 i024 i025 i026 i028 i030 i031 i032 i033 i035 i036 i038 i039 i040 i041 i045 i046 i047 i048 i049 i052 i053 i055 |
| education | 11 | i011 i012 i014 i019 i026 i036 i037 i044 i050 i051 i054 |
| politics | 9 | i001 i002 i003 i004 i014 i018 i022 i050 i054 |
| technology | 6 | i006 i016 i031 i035 i046 i053 |
| marketing | 5 | i011 i032 i033 i042 i049 |
| society | 5 | i002 i013 i014 i023 i050 |
| sports | 5 | i009 i013 i027 i034 i051 |
| media | 4 | i011 i035 i042 i043 |
| environment | 3 | i001 i005 i006 |
| health | 3 | i013 i023 i029 |
| finance | 2 | i024 i025 |
| food | 2 | i023 i029 |
| science | 2 | i005 i006 |
<!-- END GENERATED: by-topic -->

### By mood

<!-- BEGIN GENERATED: by-mood -->
| Mood | Count | IDs |
| --- | --- | --- |
| clean | 35 | i001 i002 i004 i007 i010 i015 i016 i017 i018 i019 i020 i022 i024 i025 i027 i029 i030 i032 i033 i035 i036 i037 i039 i040 i041 i042 i043 i047 i048 i049 i050 i051 i052 i054 i055 |
| friendly | 31 | i008 i010 i012 i013 i014 i015 i017 i019 i020 i021 i023 i024 i025 i029 i030 i032 i033 i035 i036 i038 i040 i041 i042 i044 i045 i049 i050 i051 i052 i054 i055 |
| vibrant | 23 | i005 i009 i010 i017 i021 i024 i026 i029 i030 i031 i032 i034 i036 i038 i039 i040 i041 i042 i044 i045 i046 i049 i053 |
| bold | 13 | i003 i004 i005 i008 i009 i011 i013 i014 i018 i021 i029 i034 i043 |
| playful | 12 | i011 i012 i019 i023 i026 i033 i036 i038 i042 i044 i045 i046 |
| editorial | 11 | i001 i002 i005 i011 i018 i022 i027 i028 i033 i043 i054 |
| cool | 10 | i002 i006 i007 i011 i013 i016 i035 i047 i048 i055 |
| corporate | 10 | i007 i010 i016 i025 i035 i039 i047 i048 i049 i052 |
| serious | 10 | i001 i003 i004 i007 i013 i022 i028 i037 i050 i054 |
| calm | 9 | i002 i012 i015 i022 i037 i047 i048 i050 i055 |
| dark | 8 | i003 i006 i028 i031 i034 i043 i046 i053 |
| energetic | 8 | i008 i009 i021 i026 i034 i038 i051 i053 |
| warm | 8 | i008 i012 i017 i019 i023 i026 i027 i044 |
| minimal | 6 | i015 i020 i028 i030 i037 i052 |
| futuristic | 5 | i006 i031 i032 i046 i053 |
| authoritative | 4 | i001 i003 i004 i014 |
| technical | 4 | i005 i006 i016 i031 |
| retro | 2 | i014 i023 |
<!-- END GENERATED: by-mood -->

### By style

<!-- BEGIN GENERATED: by-style -->
| Style | Count | IDs |
| --- | --- | --- |
| flat | 44 | i001 i002 i003 i004 i005 i008 i009 i011 i012 i013 i014 i015 i016 i017 i018 i020 i022 i023 i024 i025 i026 i029 i030 i033 i034 i035 i036 i037 i038 i040 i041 i042 i043 i044 i045 i046 i047 i048 i049 i050 i051 i052 i054 i055 |
| line-icon | 18 | i006 i007 i008 i010 i021 i024 i026 i030 i035 i036 i037 i039 i042 i044 i048 i049 i052 i053 |
| illustrated | 16 | i011 i012 i013 i014 i019 i023 i029 i032 i033 i038 i044 i045 i050 i051 i054 i055 |
| gradient | 11 | i007 i010 i013 i017 i021 i031 i032 i038 i039 i040 i053 |
| 3d | 10 | i005 i006 i016 i019 i031 i032 i033 i039 i045 i046 |
| editorial | 10 | i001 i002 i003 i004 i005 i018 i022 i027 i028 i054 |
| paper-cut | 9 | i007 i008 i010 i021 i041 i046 i049 i052 i053 |
| photo-led | 6 | i001 i006 i009 i027 i028 i034 |
| hand-drawn | 3 | i012 i019 i023 |
| ui-cards | 2 | i025 i042 |
| neon | 1 | i031 |
<!-- END GENERATED: by-style -->

### By format

<!-- BEGIN GENERATED: by-format -->
| Format | Count | IDs |
| --- | --- | --- |
| square | 17 | i002 i007 i019 i022 i024 i026 i030 i033 i038 i039 i040 i043 i046 i048 i049 i051 i055 |
| landscape | 14 | i005 i006 i014 i016 i018 i020 i021 i029 i031 i036 i041 i044 i052 i053 |
| portrait | 14 | i001 i008 i009 i010 i011 i015 i023 i027 i028 i034 i037 i042 i045 i054 |
| long | 9 | i003 i004 i012 i013 i025 i032 i035 i047 i050 |
| banner | 1 | i017 |
<!-- END GENERATED: by-format -->

### By chart

<!-- BEGIN GENERATED: by-charts -->
| Chart | Count | IDs |
| --- | --- | --- |
| C-icon-badge | 25 | i003 i004 i007 i008 i012 i014 i015 i017 i021 i024 i026 i030 i031 i032 i037 i038 i040 i042 i044 i045 i046 i048 i049 i052 i055 |
| C-title-block | 22 | i002 i009 i010 i011 i013 i015 i022 i023 i026 i027 i028 i029 i032 i033 i034 i042 i043 i044 i047 i050 i051 i054 |
| C-source-line | 18 | i001 i002 i003 i004 i013 i014 i022 i023 i024 i025 i027 i028 i029 i031 i034 i035 i047 i054 |
| C-stat-block | 17 | i001 i005 i006 i009 i011 i022 i025 i028 i029 i031 i034 i035 i037 i041 i042 i044 i054 |
| C-callout | 16 | i002 i006 i014 i015 i020 i023 i028 i029 i038 i040 i041 i042 i046 i051 i053 i055 |
| C-icon-array | 9 | i005 i006 i011 i037 i042 i044 i047 i054 i055 |
| C-icon-grid | 8 | i010 i019 i023 i035 i037 i039 i051 i053 |
| C-progress-ring | 8 | i005 i009 i013 i025 i031 i035 i044 i047 |
| C-pie | 7 | i011 i025 i029 i031 i037 i042 i054 |
| C-ribbon-label | 7 | i004 i007 i009 i010 i014 i023 i055 |
| C-columns | 6 | i013 i024 i031 i035 i036 i037 |
| C-half-donut | 6 | i001 i003 i004 i017 i050 i052 |
| C-rocket | 6 | i020 i038 i039 i040 i041 i046 |
| C-arc-label | 5 | i001 i003 i004 i019 i050 |
| C-donut | 5 | i023 i025 i047 i049 i052 |
| C-legend | 5 | i001 i002 i018 i025 i047 |
| C-us-choropleth | 5 | i004 i013 i015 i035 i054 |
| C-progress-bars | 4 | i025 i031 i044 i047 |
| C-bars | 3 | i009 i023 i031 |
| C-bulb | 3 | i012 i026 i036 |
| C-hemicycle | 3 | i003 i004 i022 |
| C-polar-area | 3 | i017 i024 i047 |
| C-process-spine | 3 | i007 i008 i032 |
| C-pyramid | 3 | i016 i033 i041 |
| C-section-band | 3 | i035 i037 i039 |
| C-world-map | 3 | i031 i037 i040 |
| C-3d-bars | 2 | i005 i031 |
| C-card | 2 | i025 i042 |
| C-cycle | 2 | i030 i033 |
| C-globe | 2 | i006 i046 |
| C-grouped-columns | 2 | i009 i047 |
| C-label-bars | 2 | i001 i025 |
| C-line | 2 | i011 i025 |
| C-pencil | 2 | i012 i019 |
| C-petal-wheel | 2 | i036 i049 |
| C-process-row | 2 | i021 i047 |
| C-seat-grid | 2 | i001 i022 |
| C-speech-bubble | 2 | i008 i024 |
| C-stat-row | 2 | i003 i027 |
| C-table | 2 | i005 i009 |
| C-tick-ring | 2 | i003 i004 |
| C-us-bubble-map | 2 | i004 i014 |
| C-venn | 2 | i043 i048 |
| C-butterfly-bars | 1 | i047 |
| C-combo | 1 | i002 |
| C-cycle-segments | 1 | i030 |
| C-dot-map | 1 | i047 |
| C-dot-rows | 1 | i027 |
| C-hex-map | 1 | i018 |
| C-hub-spokes | 1 | i026 |
| C-multi-line | 1 | i028 |
| C-negative-columns | 1 | i006 |
| C-org-tree | 1 | i037 |
| C-percent-bar | 1 | i001 |
| C-picture-gauge | 1 | i044 |
| C-radial-bars | 1 | i005 |
| C-radial-list | 1 | i027 |
| C-ranked-list | 1 | i034 |
| C-ring-gauge | 1 | i031 |
| C-sparkline | 1 | i031 |
| C-stacked-area | 1 | i031 |
| C-stacked-columns | 1 | i025 |
| C-timeline | 1 | i024 |
<!-- END GENERATED: by-charts -->

### By what you must supply

<!-- BEGIN GENERATED: by-needs -->
| The design needs | Count | IDs |
| --- | --- | --- |
| nothing | 49 | i001 i002 i003 i004 i005 i006 i007 i008 i010 i011 i012 i013 i014 i015 i016 i017 i018 i019 i020 i021 i022 i023 i024 i025 i026 i029 i030 i031 i033 i035 i036 i037 i038 i039 i040 i041 i042 i043 i044 i046 i047 i048 i049 i050 i051 i052 i053 i054 i055 |
| subject-cutout | 4 | i009 i027 i034 i045 |
| photo | 2 | i028 i034 |
| illustration | 1 | i032 |
| logo | 1 | i034 |
<!-- END GENERATED: by-needs -->

### By ground color

<!-- BEGIN GENERATED: by-palette -->
| Ground color | Count | IDs |
| --- | --- | --- |
| white and near-white | 40 | i001 i002 i004 i005 i007 i009 i010 i012 i016 i017 i018 i019 i020 i021 i022 i023 i024 i025 i026 i027 i029 i030 i033 i035 i036 i037 i038 i039 i040 i041 i042 i045 i047 i048 i049 i050 i051 i052 i054 i055 |
| dark (black, navy, charcoal) | 9 | i003 i006 i014 i028 i031 i034 i043 i046 i053 |
| teal and cyan | 2 | i011 i015 |
| blue | 1 | i013 |
| purple and violet | 1 | i032 |
| red | 1 | i044 |
| yellow | 1 | i008 |
<!-- END GENERATED: by-palette -->
