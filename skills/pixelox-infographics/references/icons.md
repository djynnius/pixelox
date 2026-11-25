# Icons for infographics

pixelox draws icons with `shape icon NAME` from the bundled Lucide set (about 2,000 line icons). This file maps
the concepts infographics need to names that exist in that set; every name below was checked against
`pixelox shape icons`. For anything else, search: `pixelox shape icons --search money --json`.

## How to use icons

```bash
pixelox new -o d.pxo --size 1080x360 --fill '#ffffff'
pixelox shape ellipse d.pxo --in-place --name Badge --rect 100,100,160x160 --fill '#2563eb'
pixelox shape icon d.pxo trending-up --in-place --name Icon --size 84 --at 180,180 --anchor c --color '#ffffff' --stroke-width 6
pixelox shape icon d.pxo lightbulb --in-place --name Big --size 200 --at 540,180 --anchor c --color '#1f2933' --stroke-width 10
pixelox shape icon d.pxo mouse-pointer-2 --in-place --name Filled --size 120 --at 900,180 --anchor c --color '#1f2933' --fill '#f59e0b' --stroke-width 7
pixelox export d.pxo -o icons.png
```

* `--at X,Y --anchor c` centers the icon on a point: use a chart script's logged anchor to put an icon on a
  node, a tier, or a slice.
* **Line weight:** the default is size / 12. Keep it for icons up to 64 px. Larger icons look thin: set
  `--stroke-width` to about size / 16 for 80 to 120 px and size / 20 above that, so all icons on a page have
  lines of similar weight.
* **Fill:** `--fill '#hex'` colors the closed areas of the icon and keeps the outline: a two-tone look.
* **One set, one treatment per infographic:** all icons the same size within a module, the same line weight,
  the same badge style (all in filled discs, all in outlined squares, or all bare). Mixed styles are the most
  common amateur tell.
* **Sizes at 1080 px wide:** inline with text 28 to 36; in a badge 56 to 96 (badge = 1.8 to 2 x the icon); as a
  module's picture 150 to 220; a hero illustration 250 to 320.
* **Color:** the module's type color, or the color of the data the icon stands next to. Never a new color.
* An icon decorates a label; it does not replace it. Every icon sits with a word.
* Lucide has no brand logos, no flags, no party emblems, and no country shapes. Use the user's own image
  (`comp place logo.png`) for those; never draw a logo.

## Concept to icon name

### Numbers and charts

| Concept | Icon name |
| --- | --- |
| growth, increase | `trending-up` |
| decline, decrease | `trending-down` |
| bar chart, statistics | `chart-column` |
| ranking bars | `chart-bar` |
| share, pie | `chart-pie` |
| trend line | `chart-line` |
| area, volume | `chart-area` |
| percent, rate | `percent` |
| target, goal | `target` |
| goal reached | `goal` |
| activity, pulse | `activity` |
| gauge, level | `gauge` |
| calculation | `calculator` |
| data, records | `database` |
| layers, levels | `layers` |
| filter, funnel | `funnel` |
| milestone | `milestone` |
| award, best | `award` |
| trophy, winner | `trophy` |
| medal, rank | `medal` |
| podium, top three | `podium` |

### Money and business

| Concept | Icon name |
| --- | --- |
| payment, donation | `hand-coins` |
| price, cost | `circle-dollar-sign` |
| money, price | `dollar-sign` |
| cash | `banknote` |
| coins, savings | `coins` |
| savings | `piggy-bank` |
| wallet, spending | `wallet` |
| card payment | `credit-card` |
| receipt, bill | `receipt` |
| briefcase, work | `briefcase` |
| company, office | `building` |
| factory, industry | `factory` |
| store, retail | `store` |
| shopping, cart | `shopping-cart` |
| bag, purchase | `shopping-bag` |
| package, delivery | `package` |
| truck, logistics | `truck` |
| deal, partnership | `handshake` |
| presentation | `presentation` |
| strategy, route | `route` |
| workflow | `workflow` |
| network, org | `network` |
| tag, offer | `tag` |
| gift, bonus | `gift` |

### People and society

| Concept | Icon name |
| --- | --- |
| avatar, profile | `circle-user` |
| pedestrian, figure | `person-standing` |
| person | `user` |
| people, team, audience | `users` |
| add a member | `user-plus` |
| verified person | `user-check` |
| contact | `contact` |
| baby, birth | `baby` |
| accessibility | `accessibility` |
| handshake, agreement | `handshake` |
| care, charity | `hand-heart` |
| heart, like | `heart` |
| thumbs up, approval | `thumbs-up` |
| star, rating | `star` |
| speech, statement | `speech` |
| comment | `message-circle` |
| discussion | `messages-square` |
| quote | `quote` |
| languages | `languages` |
| home, housing | `house` |
| city hall, government, bank | `landmark` |
| vote, ballot | `vote` |
| justice, balance | `scale` |
| law, ruling | `gavel` |
| flag, nation, milestone | `flag` |
| crown, leader | `crown` |

### Process and time

| Concept | Icon name |
| --- | --- |
| magnet, attract | `magnet` |
| scan, inspect, optical sort | `scan-eye` |
| done, confirmed | `circle-check` |
| footnote mark, note | `asterisk` |
| search, research | `search` |
| idea | `lightbulb` |
| design, draft | `pen-tool` |
| plan, measure | `pencil-ruler` |
| write, edit | `pencil` |
| build, make | `hammer` |
| tools, fix | `wrench` |
| settings, system | `settings` |
| launch | `rocket` |
| send, deliver | `send` |
| checklist | `list-checks` |
| task done | `clipboard-check` |
| list, agenda | `clipboard-list` |
| done, yes | `check` |
| no, remove | `x` |
| info | `info` |
| repeat, cycle | `repeat` |
| refresh, renew | `refresh-cw` |
| next, forward | `arrow-right` |
| up | `arrow-up` |
| down | `arrow-down` |
| clock, time | `clock` |
| timer, deadline | `timer` |
| hourglass, waiting | `hourglass` |
| calendar, date | `calendar` |
| schedule | `calendar-days` |
| date confirmed | `calendar-check` |
| puzzle, fit | `puzzle` |
| key, access | `key` |
| lock, security | `lock` |
| shield, protection | `shield` |

### Technology and media

| Concept | Icon name |
| --- | --- |
| phone, mobile | `smartphone` |
| laptop | `laptop` |
| screen, desktop | `monitor` |
| devices | `monitor-smartphone` |
| tablet | `tablet` |
| tv | `tv` |
| radio | `radio` |
| headphones, audio | `headphones` |
| microphone | `mic` |
| video | `video` |
| camera, photo | `camera` |
| image | `image` |
| play | `play` |
| wifi, connection | `wifi` |
| cloud | `cloud` |
| server | `server` |
| code | `code` |
| chip, processor | `cpu` |
| brain, thinking, AI | `brain` |
| sparkle, new, AI | `sparkles` |
| qr code | `qr-code` |
| link | `link` |
| mail, email | `mail` |
| inbox | `inbox` |
| bell, alert | `bell` |
| megaphone, promotion | `megaphone` |
| share | `share-2` |
| news | `newspaper` |
| feed | `rss` |
| globe, web, world | `globe` |
| cursor, click | `mouse-pointer-2` |
| window, app | `app-window` |

### Places and travel

| Concept | Icon name |
| --- | --- |
| train, rail | `train-front` |
| sailing, boat | `sailboat` |
| rescue, safety, support | `life-buoy` |
| location, pin | `map-pin` |
| map, region | `map` |
| compass, direction | `compass` |
| navigation | `navigation` |
| world | `earth` |
| mountain, challenge | `mountain` |
| signpost, choice | `signpost` |
| plane, travel | `plane` |
| car, driving | `car` |
| bus, transit | `bus` |
| ship, shipping | `ship` |
| bike, cycling | `bike` |
| walking, steps | `footprints` |
| fuel | `fuel` |
| hospital | `hospital` |
| school | `school` |

### Health, food, nature

| Concept | Icon name |
| --- | --- |
| tree, park | `tree-deciduous` |
| wine, bottle | `bottle-wine` |
| health, heartbeat | `heart-pulse` |
| doctor | `stethoscope` |
| medicine | `pill` |
| vaccine | `syringe` |
| temperature | `thermometer` |
| fitness | `dumbbell` |
| food, meal | `utensils` |
| fruit | `apple` |
| vegetable | `carrot` |
| salad, healthy | `salad` |
| coffee, break | `coffee` |
| pizza, fast food | `pizza` |
| water, drop | `droplet` |
| fire, energy, burn | `flame` |
| energy, power | `zap` |
| battery | `battery-full` |
| plug, electricity | `plug` |
| sun, solar | `sun` |
| moon, night | `moon` |
| wind | `wind` |
| leaf, eco | `leaf` |
| sprout, growth | `sprout` |
| tree, forest | `tree-pine` |
| recycle | `recycle` |
| fish | `fish` |
| bird | `bird` |
| dog, pets | `dog` |
| bug, defect | `bug` |

### Education and science

| Concept | Icon name |
| --- | --- |
| graduation, degree | `graduation-cap` |
| book, reading | `book-open` |
| books, library | `book` |
| school | `school` |
| ruler, measure | `ruler` |
| microscope, research | `microscope` |
| lab, experiment | `flask-conical` |
| atom, science | `atom` |
| document, report | `file-text` |
| bookmark | `bookmark` |
| palette, design | `palette` |
| brush, art | `brush` |
| scissors, cut | `scissors` |
| gem, quality | `gem` |

### Sports

| Concept | Icon name |
| --- | --- |
| pennant, corner flag | `flag-triangle-right` |
| trophy | `trophy` |
| medal | `medal` |
| whistle, referee | `whistle` |
| ball (volleyball) | `volleyball` |
| jersey | `shirt` |
| fitness | `dumbbell` |
| timer | `timer` |
| flag | `flag` |
| target | `target` |

## What the set does not have

Lucide is a line-icon set. It has no solid (filled) glyphs, no combined scenes (gears with a
magnifier, people with speech marks), no money bag, top hat, gloves, scoreboard, running
figure, club crest, flag, or party emblem. Use the nearest single icon (`coins` for a money
bag, `person-standing` for a figure, `shield` for a crest outline), fill it with `--fill` for a
solid look, or build the scene from two icons that overlap. Never draw a real crest or emblem.

## Picture objects that are not icons

Bigger metaphors come from `scripts/object.js` (rocket, bulb, pencil, thermometer, battery: C-rocket, C-bulb,
C-pencil, C-picture-gauge in [charts.md](charts.md)) and people pictograms from `scripts/waffle.js` with
`shape=person` (C-icon-array). A photo or a drawn illustration is the user's to supply: name it
`illustration.png` or `subject.png` in the recipe and say what it should show.
