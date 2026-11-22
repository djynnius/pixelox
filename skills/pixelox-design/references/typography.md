# Typography for posters

Type does most of the work on a poster. This file covers how pixelox sets type, which font
class to use for which look (with free look-alikes), how to pair fonts, and the size,
spacing, and placement numbers that separate designed from default.

## How pixelox sets type

| You want | Do this |
| --- | --- |
| A font | `--font @condensed-heavy` (a class: the best installed face of that kind) or `--font "Bebas Neue"` (one family) |
| A font size | `--size 12%` is 12% of the canvas **height** (best: scales with the canvas); also `64px`, `48pt` |
| A cap height | font size = cap height / 0.72 for most faces (Impact / 0.8, Copperplate / 0.56) |
| An exact line width | `--fit-width 920` (px) or `--fit-width 85%` (of the canvas width) instead of `--size`; `--fit-box 920x300` for the largest size that fits a box. The fit is redone when the words change |
| An exact position of the visible letters | add `--anchor-to ink`: `--at 80,90` then places the top-left of the letters (with `--anchor c`, their center) instead of the text box |
| A position by text box | `--at X,Y --anchor c` alone; the box includes space above the caps, so tops land 10 to 25% of the font size lower than Y |
| Several lines in one layer | `--content $'LINE ONE\nLINE TWO'` (the `$'...'` form; a plain "\n" in double quotes prints a backslash) |
| A wrapped paragraph | `--box 600x400` wraps at 600 px; `--align left`, `center`, `right`, `justify` |
| Weight | `--weight 100..900`; pixelox picks the nearest weight of the **same width** and says so (PXW4013) |
| A narrower or wider face | `--stretch condensed` or `--stretch expanded` picks a real face of that width when the family has one |
| Stretched letters | `--scale-x 80%` (narrower) or `150%` (wider); `--scale-y`; the text stays live |
| A little heavier | `--faux-bold 20` (about one weight step; up to 80) |
| Italic | `--style italic` (a missing italic is faked by slanting, warning PXW4002: avoid on display type) |
| Letter spacing | `--tracking N` in 1/1000 em: 100 = 10% of the font size |
| Line spacing | `--leading 95%` (percent of the font size) |
| All caps | `--case upper` (also `lower`, `title`); the content keeps what you typed |
| Outline, gradient, shadow | `--stroke` on the text; `fx overlay`, `fx drop-shadow`, `fx stroke`, `fx extrude`, `fx long-shadow` on the layer |
| Arched, waving, rising | `--warp arc --bend 35`, or `pixelox text warp d.pxo --in-place --layer NAME --preset rise --bend 40` (T-warp-type) |
| Vertical, on a circle | `transform rotate --degrees -90`; `text on-path --fit shrink` (T-arc-type) |
| Restyle later | `text style --layer NAME --font F --tracking N --fit-width W`; `text edit` changes the words |

Everything in this table keeps the layer live text. Filters (`filter gaussian`, `filter
glitch`) are live on text as well. Only `transform skew`, `transform perspective`, and
`transform flip` turn text into pixels (warning PXW2003): finish the wording first.

Things that bite:

* **A font that is not installed is replaced, not refused.** The result's `warnings` say
  what was drawn instead (PXW4010) and how to get the real one. Read them: a substitute can
  be wider, and a class with nothing installed (`@stencil` on most machines) falls back to
  a different kind of face.
* **A weight never changes the width.** `"Helvetica Neue" --weight 900` draws Bold and warns
  (PXW4013), because the only 900 face is condensed: ask for it with `--stretch condensed`.
* **Display classes have one weight.** Give `--weight` with the text classes
  (`@neo-grotesque`, `@geometric-sans`, `@condensed`, `@oldstyle`, `@didone`, `@slab`,
  `@script-formal`); leave it out with `@condensed-heavy`, `@heavy-wide`, `@rounded-heavy`,
  `@brush`, and the other display classes, or the face is thickened artificially (PXW4002).
* **Width differs wildly between fonts.** Never guess the size of a headline that must fit
  a width: use `--fit-width` (T-fit-width).
* Do not put a `|` character or three dots in a row in `--content`; draw a divider with
  `shape line`, and use a single ellipsis-free phrase.
* `text on-path` reports letters that do not fit on the path (PXW4012): add `--fit shrink`.

## Fonts: name the class

Recipes in this skill name most fonts by **class**: `--font @condensed-heavy` means "the
tall, tight, loud one", and pixelox draws the best face of that kind that is installed:
Impact on macOS and Windows, Liberation Sans Narrow Bold on Linux, Anton when it has been
downloaded. The same command works on every machine.

1. **See what you have.** `pixelox fonts classes` lists all 28 classes with the face each
   one draws here and its status: `member` (a real face of the class), `stand-in` (close,
   warning PXW4014), or `substitute` (nothing of the class is installed, warning PXW4010).
   `pixelox fonts classes @didone --json` shows one class with every candidate.
2. **Pick a class for each role** (headline, body, accent) from the table below and write
   `--font @class`. Two classes at most per design.
3. **A stand-in or substitute where the look matters?** The free families in the table are
   open-licensed Google Fonts that pixelox can download. **Ask the user first**, then run
   `pixelox fonts get @stencil -y` (the best free member of a class) or
   `pixelox fonts get "Bebas Neue" -y` (one family). Without `-y` the command refuses
   (PX1540, exit 6). Never add `-y` on your own. A downloaded family works at once, by
   class and by name.
4. **Want one exact face?** Name the family: `--font "DIN Condensed"`. Where it is not
   installed, pixelox draws the closest face of its class and warns (PXW4010); the layer
   keeps the name, so the document draws the real font wherever it exists.
5. **Re-fit after any substitution.** Widths differ between fonts. Lines set with
   `--fit-width` refit themselves; check lines set with `--size` for overflow and look at
   the result.
6. **Nothing installed at all?** The generic names `sans-serif`, `serif`, and `monospace`
   always resolve (on a machine with no fonts, to the embedded **Tuffy**, a plain
   medium-weight sans). With Tuffy alone, build hierarchy from size, capitals, tracking,
   color, and `--faux-bold`, not from font changes, and offer the download.

Record the fonts you used, and any substitution the warnings reported, in your delivery
note. For a build that must fail instead of substituting (CI), add `--strict-font`.

### Families that recipes name directly

A few recipes name a family because that exact face is the look. Elsewhere each is
replaced from the class shown; use the class yourself when you adapt the recipe.

| Family named in recipes | Its class |
| --- | --- |
| DIN Condensed, `"Helvetica Neue" --weight 900 --stretch condensed`, Arial Narrow 700 | `@condensed-heavy` |
| Avenir, Futura, Gill Sans, Trebuchet MS | `@geometric-sans` (Futura 700 in tracked caps: `@deco`) |
| Verdana | `@neo-grotesque` |
| DIN Alternate | `@techno` |
| Bodoni 72 | `@didone` |
| Big Caslon | `@engraved` (as a text face: `@oldstyle`) |
| Times New Roman, Georgia | `@oldstyle` |
| Brush Script MT | `@brush` |
| Noteworthy, Bradley Hand | `@marker` |
| Zapfino | `@script-formal` |

## Font classes

Seen across the poster set. The free families are the ideal for each class (step 3 above
gets them); the system columns are what the class draws when nothing has been
downloaded (Windows entries marked * come with Microsoft Office).

| Class (`--font`) | Look, where it is used | Free families | macOS | Windows | Linux |
| --- | --- | --- | --- | --- | --- |
| `@condensed-heavy` | tall, tight, loud; event flyers, film titles, type blocks | Anton, League Gothic, Bebas Neue, Oswald 700 | Impact, DIN Condensed, `Helvetica Neue` 900 condensed | Impact, Bahnschrift Condensed (700) | Liberation Sans Narrow Bold, DejaVu Sans Condensed Bold |
| `@compressed` | skyscraper letters, narrower than Impact, that fill a tall column | Six Caps, Antonio 700, Big Shoulders Display 900 | DIN Condensed (stand-in; add `--scale-x 70%`) | Bahnschrift Condensed, Onyx* | Liberation Sans Narrow Bold plus `--scale-x 70%` |
| `@condensed` | condensed gothic at regular weights: sub-heads, dates, billing blocks, labels | Oswald 400, Barlow Condensed, Roboto Condensed, Antonio 300 | Avenir Next Condensed 400 to 700, Arial Narrow | Bahnschrift Condensed (400), Arial Narrow* | Liberation Sans Narrow, DejaVu Sans Condensed |
| `@condensed-hairline` | elegant tall thin titles, fashion | Antonio 100, Big Shoulders Display 100, Oswald 200 | Avenir Next Condensed Ultra Light | Bahnschrift Light Condensed | Liberation Sans Narrow (no hairline: use small, tracked) |
| `@heavy-wide` | chunky, confident; sale, sport, protest | Archivo Black, Inter 900, Rubik 900 | Arial Black, `Avenir Next` 900 | Arial Black, Segoe UI Black | DejaVu Sans Bold, Noto Sans Black |
| `@extended` | wide and fast, upright or italic: cars, sport, youth church, tech | Archivo Expanded 800, Unbounded, Krona One, Syncopate | Arial Black (stand-in; add `--scale-x 125% --style italic`) | Arial Black plus `--scale-x 125%` | DejaVu Sans Bold (already wide) |
| `@neo-grotesque` | neutral, Swiss, corporate | Inter, Archivo, Roboto, Work Sans | Helvetica Neue, Helvetica, Arial | Arial, Segoe UI | Liberation Sans, DejaVu Sans, Noto Sans |
| `@geometric-sans` | round, modern, friendly tech | Montserrat, Poppins, Outfit, Jost, Mulish | Avenir Next, Futura, Avenir | Century Gothic*, Segoe UI | Noto Sans, Ubuntu, Cantarell |
| `@thin-sans` | thin and wide: airy, premium tech, fashion | Montserrat 200, Raleway 200, Jost 200 | Avenir Next Ultra Light, `Helvetica Neue` 100 | Segoe UI Light | Noto Sans Light, Ubuntu Light |
| `@rounded-thin` | thin monoline rounded: soft, friendly, wellness, kids tech | Quicksand, Comfortaa, Nunito 200, Varela Round | Avenir Next Ultra Light (stand-in) | Segoe UI Light | Ubuntu Light |
| `@rounded-heavy` | food, kids, playful retail, chunky statements | Fredoka 700, Nunito 900, Baloo 2 800, Lilita One, Rubik 800 | Arial Rounded MT Bold (fatten with `--faux-bold 40`) | Arial Rounded MT Bold*, Segoe UI Black | Ubuntu Bold |
| `@techno` | squared sans: sci-fi, gaming, fintech, devices | Orbitron, Rajdhani, Chakra Petch, Oxanium, Michroma | DIN Alternate (stand-in), Futura 700 caps tracked 60 | Bahnschrift, Agency FB* | Ubuntu Bold, DejaVu Sans Bold |
| `@didone` | high contrast: glamorous, editorial, mastheads | Playfair Display, Bodoni Moda, DM Serif Display, Abril Fatface | Didot, Bodoni 72 | Bodoni MT*, Georgia Bold | DejaVu Serif Bold, Noto Serif Display |
| `@didone-condensed` | tall magazine nameplates | Rozha One, Bodoni Moda (condensed cut), Chonburi | Didot 700 (stand-in; add `--scale-x 80%`), Big Caslon | Bodoni MT Condensed*, Onyx* | DejaVu Serif Condensed Bold |
| `@oldstyle` | old-style and transitional serifs: literary, calm, book covers | EB Garamond, Cormorant, Libre Baskerville, Lora | Baskerville, Hoefler Text, Palatino, Georgia | Georgia, Palatino Linotype, Cambria | Liberation Serif, Noto Serif, DejaVu Serif |
| `@swash-serif` | fantasy and romance titles, one ornate capital | Cinzel Decorative, Cormorant Garamond Italic, IM Fell English, Playfair Display Italic | Apple Chancery; a Zapfino capital with Baskerville lowercase | Gabriola, Monotype Corsiva* | Liberation Serif Italic |
| `@engraved` | inscriptional caps: film credits, luxury, ceremony | Cinzel, Marcellus, Cormorant SC | Copperplate, Optima, Big Caslon | Copperplate Gothic*, Perpetua Titling* | DejaVu Serif (caps, tracked 100) |
| `@uncial` | medieval: fantasy, folklore, heritage | Uncial Antiqua, MedievalSharp, Metamorphous, New Rocker | Luminari, Herculanum | Old English Text MT*, Viner Hand ITC* | (download one) |
| `@blackletter` | gothic, metal, tradition, mastheads | UnifrakturMaguntia, UnifrakturCook, Pirata One | Luminari (stand-in) | Old English Text MT* | (download one) |
| `@slab` | sturdy, retro, western, editorial | Roboto Slab, Zilla Slab, Bitter, Arvo, Alfa Slab One | Rockwell, American Typewriter, Superclarendon | Rockwell*, Cambria Bold, Courier New Bold | DejaVu Serif Bold, Noto Serif Bold |
| `@fat-display` | chunky slab, cut-paper, Cooper style: 1970s warmth, food, soul and funk posters | Ultra, Chango, Titan One, Bowlby One, Shrikhand, Rubik 900 | Superclarendon Black; Arial Rounded MT Bold fattened | Cooper Black*, Rockwell Extra Bold* | DejaVu Serif Bold fattened |
| `@script-formal` | weddings, worship, elegance | Great Vibes, Allura, Pinyon Script | Snell Roundhand, Zapfino | Segoe Script, Edwardian Script ITC* | (download one) |
| `@script-signature` | hairline autograph accents, fashion, beauty | Mrs Saint Delafield, Sacramento, Parisienne, Dancing Script | Savoye LET, `Snell Roundhand` 500, Bradley Hand | Lucida Handwriting, Freestyle Script* | (download one) |
| `@brush` | brush script and heavy script wordmarks: energy, handmade, lifestyle, retro soda marks | Yellowtail, Pacifico, Lobster, Kaushan Script | SignPainter, Brush Script MT | Brush Script MT*, Segoe Script Bold | (download one) |
| `@marker` | marker, handwriting, dry-brush caps: notes, street, zines, shouted captions | Permanent Marker, Rock Salt, Caveat Brush, Amatic SC, Patrick Hand SC | Marker Felt, Noteworthy, Bradley Hand, Chalkduster | Ink Free, Segoe Print | (download one) |
| `@stencil` | military, street, industrial | Allerta Stencil, Stardos Stencil, Saira Stencil One | none: download one, or cut the gaps (T-stencil-type) | Stencil* | none: download one, or T-stencil-type |
| `@mono` | code, tech, receipts, captions | JetBrains Mono, IBM Plex Mono, Space Mono | Menlo, Courier New, Monaco | Consolas, Courier New | DejaVu Sans Mono, Liberation Mono |
| `@deco` | art-deco, unicase, retro display: 1920s to 1970s posters, club nights | Limelight, Monoton, Righteous, Bungee, Fascinate | Phosphate (inline), Copperplate, Futura 700 | Broadway*, Gill Sans Ultra Bold* | (download one) |

Family quirks on macOS: DIN Condensed exists only in bold; `Avenir Next` has no Light, so
300 draws Ultra Light (use 400) and 800 draws Heavy; `Baskerville` 600 is much heavier than
400; the 900 face of `Helvetica Neue` and the 800 face of `Futura` are condensed, so they
need `--stretch condensed`.

### How wide a headline will be

Width of a 9-character all-caps line, as a multiple of its cap height (measured with
pixelox on macOS faces). Use it to predict whether a headline fits before you set it.

| Font (weight) | Width / cap height | Cap height / font size |
| --- | --- | --- |
| League Gothic, Anton (free `@condensed-heavy` families) | about 3.0 to 3.7 | 0.72 |
| DIN Condensed (700) | 4.3 | 0.73 |
| Impact (`@condensed-heavy` on macOS and Windows) | 4.8 | 0.83 |
| Avenir Next Condensed (700) (`@condensed --weight 700`) | 5.3 | 0.75 |
| Helvetica Neue (900, `--stretch condensed`), Arial Narrow (700), Futura (800, `--stretch condensed`) | 5.7 | 0.75 to 0.79 |
| Helvetica Neue (300 / 700) (`@neo-grotesque`) | 6.5 / 7.0 | 0.75 |
| Avenir Next (500 / 900) (`@geometric-sans`), Futura (700) | 6.7 / 7.3 | 0.75 to 0.80 |
| Didot (700), Bodoni 72 (700), Baskerville (700), Rockwell (700) | 6.8 to 7.7 | 0.69 to 0.77 |
| Arial Black (`@heavy-wide`), Gill Sans (700), Superclarendon (900) | 8.0 to 8.5 | 0.70 to 0.78 |
| Copperplate (700) (`@engraved --weight 700`) | 9.7 | 0.56 |

Example: a 9-letter line in Impact with a cap height of 16% H on a 1080x1350 canvas is
`4.8 x 216 = 1037 px` wide: too wide for a canvas with 8% margins (918 px), so either the
cap height drops to 14% or the face changes to a narrower one. Or skip the arithmetic: set
the line with `--fit-width 918` and read the height from `bounds`.

## Size scale

All values are **cap heights in % of the canvas height**; multiply by 1.39 for `--size`.
Pick one row for the format and keep the ratios.

| Level | Portrait poster / flyer | Square post | Story 9:16 | Landscape banner |
| --- | --- | --- | --- | --- |
| Headline (1 to 3 lines) | 8 to 16 (type-led: up to 30) | 9 to 16 | 6 to 11 | 12 to 22 |
| Sub-headline / key fact | 2.5 to 4.5 | 3 to 5 | 2 to 3.5 | 4 to 7 |
| Body, details | 1.3 to 1.9 | 1.8 to 2.6 | 1.2 to 1.7 | 2.4 to 3.5 |
| Fine print, credits | 0.8 to 1.1 | 1.2 to 1.5 | 0.8 to 1 | 1.6 to 2 |

* Ratio headline : sub-head : body : fine print is about **10 : 3 : 1.3 : 0.8**. Each level
  is at least 1.6x the next; steps smaller than 1.3x look like mistakes.
* Three, at most four, sizes on one design. If two texts are nearly the same size, make
  them the same size, or clearly different.
* The smallest text must stay readable at the size the design is seen: for social images
  keep cap heights at or above 1.2% H (about 16 px on a 1350 px canvas).
* A headline that fills 80 to 100% of the width between the margins looks intended; one
  that fills 55 to 70% looks unfinished. Fit it, or make it clearly small (under 40%).

## Spacing

| Setting | Display type (headlines) | Small caps lines, labels | Body |
| --- | --- | --- | --- |
| Tracking | -30 to 0 for heavy faces at large sizes; 0 for condensed | 80 to 300 (the smaller the type, the wider) | 0 to 20 |
| Leading | 82 to 100% for all-caps stacks; 100 to 110% for mixed case | 120 to 140% | 130 to 150% |
| Line length | 1 to 3 words per line | up to 30 characters | 28 to 45 characters per line |

* Wide tracking (200 to 500) on a short all-caps line of small light type reads as
  premium; on lowercase or on bold display type it reads as broken.
* Tight leading on all-caps stacks: the gap between lines should be 15 to 30% of the cap
  height, close to the letters' stroke width.
* Leave space of at least one headline cap height between the headline block and the next
  text block, and half of that between related lines.

## Pairing rules

1. **Two families at most.** One for display, one for text. A third only as a single
   script or mono accent word.
2. **Pair by contrast, not similarity.** Condensed heavy sans + light or regular geometric
   sans. Didone display + neutral grotesque. Script accent + all-caps sans. Slab + mono.
   Two different sans faces of similar width and weight never work together.
3. **One family, many weights** is the safest strong choice (the Swiss approach): 900 for
   the headline, 400 for body, 600 italic for emphasis.
4. **Script and blackletter are accents**: one word, mixed case, 1.1 to 1.5x the size of
   the neighboring caps, never tracked, never all caps (T-script-overlap).
5. **Match the voice to the brief**: church and gala lean serif or script plus clean sans;
   tech leans thin or geometric sans and mono; street and sport lean condensed heavy and
   distressed; food leans rounded and brush; fashion leans didone or wide thin sans.

Proven pairs (display / text), as classes with a free family and a system face for each:

| Mood | Display | Text |
| --- | --- | --- |
| Loud event | `@condensed-heavy` (Anton, Impact) | `@geometric-sans` 500 and 800 (Montserrat, Avenir Next) |
| Swiss / corporate | `@neo-grotesque` 700 or 800 (Inter, Helvetica Neue) | `@neo-grotesque` 400 |
| Editorial / fashion | `@didone` 700 (Playfair Display, Didot) | `@geometric-sans` 300 or 400, tracked caps (Jost, Avenir Next) |
| Elegant worship / gala | `@script-formal` (Great Vibes, Snell Roundhand) + `@engraved` (Cinzel, Copperplate) | `@geometric-sans` 400 |
| Tech launch | `@thin-sans`, wide (Montserrat 200, Avenir Next Ultra Light) | `@mono` (JetBrains Mono, Menlo) |
| Retro / diner | `@slab` 700 (Alfa Slab One, Rockwell) | `@brush` accent (Yellowtail, SignPainter) |
| Street / grunge | `@condensed-heavy` (League Gothic, DIN Condensed) | `@mono` 700 (Space Mono, Courier New) |
| Friendly food | `@rounded-heavy` (Lilita One, Arial Rounded MT Bold) | `@geometric-sans` 500 (Poppins, Avenir Next) |

## Placement

* Margins: 6 to 9% of the width on every side for text; nothing important closer than 4%.
* Choose one alignment system per design: all centered on the axis, **or** all hung from
  one or two left edges. Mixing centered and left-aligned blocks is the most common
  amateur tell.
* Align text edges to something: another text edge, the edge of a shape, the subject's
  shoulder line, a margin.
* Text over a photo needs a calm area. If there is none, make one: T-photo-fade, a solid
  panel (T-label-box, T-info-bar), a darkened overlay, or a blurred copy of the photo.
* Contrast: light text on a ground darker than 30% lightness, dark text on a ground
  lighter than 75%. Mid-tone grounds (40 to 65%) fight both; add a panel or a shadow.

## Specimen

Run this to see what every class draws on your machine: 28 lines, one per class, each fitted
into the same box. Any line with a PXW4010 or PXW4014 warning is a substitute or a
stand-in; the free family for it is in the table above.

```bash
pixelox new -o specimen.pxo --size 1080x1350 --fill '#f3efe6'
pixelox text add specimen.pxo --in-place --name S1 --content "CONDENSED HEAVY" --font @condensed-heavy --fit-box 440x52 --at 6%,4.0% --anchor-to ink --color '#111111'
pixelox text add specimen.pxo --in-place --name S2 --content "COMPRESSED" --font @compressed --fit-box 440x52 --at 53%,4.0% --anchor-to ink --color '#111111'
pixelox text add specimen.pxo --in-place --name S3 --content "Condensed gothic" --font @condensed --fit-box 440x52 --at 6%,10.6% --anchor-to ink --color '#111111'
pixelox text add specimen.pxo --in-place --name S4 --content "CONDENSED HAIRLINE" --font @condensed-hairline --fit-box 440x52 --at 53%,10.6% --anchor-to ink --color '#111111'
pixelox text add specimen.pxo --in-place --name S5 --content "HEAVY WIDE" --font @heavy-wide --fit-box 440x52 --at 6%,17.2% --anchor-to ink --color '#111111'
pixelox text add specimen.pxo --in-place --name S6 --content "EXTENDED" --font @extended --fit-box 440x52 --at 53%,17.2% --anchor-to ink --color '#111111'
pixelox text add specimen.pxo --in-place --name S7 --content "Neo-grotesque" --font @neo-grotesque --fit-box 440x52 --at 6%,23.8% --anchor-to ink --color '#111111'
pixelox text add specimen.pxo --in-place --name S8 --content "Geometric sans" --font @geometric-sans --fit-box 440x52 --at 53%,23.8% --anchor-to ink --color '#111111'
pixelox text add specimen.pxo --in-place --name S9 --content "THIN SANS" --font @thin-sans --fit-box 440x52 --at 6%,30.4% --anchor-to ink --color '#111111'
pixelox text add specimen.pxo --in-place --name S10 --content "Rounded thin" --font @rounded-thin --fit-box 440x52 --at 53%,30.4% --anchor-to ink --color '#111111'
pixelox text add specimen.pxo --in-place --name S11 --content "Rounded heavy" --font @rounded-heavy --fit-box 440x52 --at 6%,37.0% --anchor-to ink --color '#111111'
pixelox text add specimen.pxo --in-place --name S12 --content "TECHNO 2049" --font @techno --fit-box 440x52 --at 53%,37.0% --anchor-to ink --color '#111111'
pixelox text add specimen.pxo --in-place --name S13 --content "Didone" --font @didone --fit-box 440x52 --at 6%,43.6% --anchor-to ink --color '#111111'
pixelox text add specimen.pxo --in-place --name S14 --content "DIDONE CONDENSED" --font @didone-condensed --fit-box 440x52 --at 53%,43.6% --anchor-to ink --color '#111111'
pixelox text add specimen.pxo --in-place --name S15 --content "Old-style serif" --font @oldstyle --fit-box 440x52 --at 6%,50.2% --anchor-to ink --color '#111111'
pixelox text add specimen.pxo --in-place --name S16 --content "Swash Serif" --font @swash-serif --fit-box 440x52 --at 53%,50.2% --anchor-to ink --color '#111111'
pixelox text add specimen.pxo --in-place --name S17 --content "ENGRAVED CAPS" --font @engraved --fit-box 440x52 --at 6%,56.8% --anchor-to ink --color '#111111'
pixelox text add specimen.pxo --in-place --name S18 --content "Uncial" --font @uncial --fit-box 440x52 --at 53%,56.8% --anchor-to ink --color '#111111'
pixelox text add specimen.pxo --in-place --name S19 --content "Blackletter" --font @blackletter --fit-box 440x52 --at 6%,63.4% --anchor-to ink --color '#111111'
pixelox text add specimen.pxo --in-place --name S20 --content "Slab serif" --font @slab --fit-box 440x52 --at 53%,63.4% --anchor-to ink --color '#111111'
pixelox text add specimen.pxo --in-place --name S21 --content "Fat display" --font @fat-display --fit-box 440x52 --at 6%,70.0% --anchor-to ink --color '#111111'
pixelox text add specimen.pxo --in-place --name S22 --content "Formal script" --font @script-formal --fit-box 440x52 --at 53%,70.0% --anchor-to ink --color '#111111'
pixelox text add specimen.pxo --in-place --name S23 --content "Signature script" --font @script-signature --fit-box 440x52 --at 6%,76.6% --anchor-to ink --color '#111111'
pixelox text add specimen.pxo --in-place --name S24 --content "Brush script" --font @brush --fit-box 440x52 --at 53%,76.6% --anchor-to ink --color '#111111'
pixelox text add specimen.pxo --in-place --name S25 --content "Marker hand" --font @marker --fit-box 440x52 --at 6%,83.2% --anchor-to ink --color '#111111'
pixelox text add specimen.pxo --in-place --name S26 --content "STENCIL" --font @stencil --fit-box 440x52 --at 53%,83.2% --anchor-to ink --color '#111111'
pixelox text add specimen.pxo --in-place --name S27 --content "monospace 0123" --font @mono --fit-box 440x52 --at 6%,89.8% --anchor-to ink --color '#111111'
pixelox text add specimen.pxo --in-place --name S28 --content "ART DECO" --font @deco --fit-box 440x52 --at 53%,89.8% --anchor-to ink --color '#111111'
pixelox export specimen.pxo -o specimen.png
```
