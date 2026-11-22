# Typography for posters

Type does most of the work on a poster. This file covers how pixelox sets type, which font
to use for which look (with free look-alikes and safe fallbacks), how to pair fonts, and
the size, spacing, and placement numbers that separate designed from default.

## How pixelox sets type

| You want | Do this |
| --- | --- |
| A font size | `--size 12%` is 12% of the canvas **height** (best: scales with the canvas); also `64px`, `48pt` |
| A cap height | font size = cap height / 0.72 for most faces (Impact / 0.8, Copperplate / 0.56) |
| An exact line width | add the text at any size, then `transform scale --layer NAME --size 920x` (stays live text; with two numbers, `920x300`, it stretches and turns the text into pixels) |
| An exact position of the visible letters | `transform move --layer NAME --to X,Y --anchor tl` (X,Y take px or %; anchor `t` centers on X) |
| A position by text box | `--at X,Y --anchor c` on `text add`; the box includes space above the caps, so tops land 10 to 25% of the font size lower than Y |
| Several lines in one layer | `--content $'LINE ONE\nLINE TWO'` (the `$'...'` form; a plain "\n" in double quotes prints a backslash) |
| A wrapped paragraph | `--box 600x400` wraps at 600 px; `--align left`, `center`, `right`, `justify` |
| Weight | `--weight 100..900`; pixelox picks the nearest face in the family |
| Italic | `--style italic` (a missing italic is faked by slanting, warning PXW4002: avoid on display type) |
| Letter spacing | `--tracking N` in 1/1000 em: 100 = 10% of the font size |
| Line spacing | `--leading 95%` (percent of the font size) |
| All caps | type the content in capitals; there is no case switch |
| Outline, gradient, shadow | `--stroke` on the text; `fx overlay`, `fx drop-shadow`, `fx stroke` on the layer |
| Vertical, curved | `transform rotate --degrees -90`; `text on-path` |
| Restyle later | `text style --layer NAME --font F --tracking N`; `text edit` changes the words |

Things that bite:

* **A font that is not installed is an error** (PX4102, exit 2), not a silent fallback:
  follow "Fonts on your machine" below before setting any type.
* **The nearest weight may be a condensed face.** On macOS, `Helvetica Neue` at 900 gives
  Condensed Black and `Futura` at 800 gives Condensed ExtraBold. Read `font.postscript` in
  the JSON result to see what you got.
* **Width differs wildly between fonts.** Never guess the size of a headline that must fit
  a width: fit it with `transform scale` (T-fit-width).
* Filters and `fill` need pixels: run `layer rasterize` on a text layer first, and do it
  last, because rasterized text can no longer be edited.
* Do not put a `|` character or three dots in a row in `--content`; draw a divider with
  `shape line`, and use a single ellipsis-free phrase.
* `text on-path` silently drops letters that do not fit on the path: check `bounds` and
  the render.

## Fonts on your machine: do this first

Recipes in this skill name macOS fonts. On another machine those names fail (PX4102), so
before you set any type:

1. **List what is installed.** `pixelox fonts list --json` returns `result.families[]`
   (each with `family` and `faces[].weight`). To test one name:
   `pixelox fonts list --family "Arial Black" --json` (an empty list means missing).
2. **Decide the class, then the font.** For each role in your design (headline, body,
   accent), pick the font *class* you want from the table below, then take the first family
   in that row that is installed, reading left to right: look-alike (if the user has it),
   macOS, Windows, Linux.
3. **Translate recipe fonts.** When a recipe names a font you do not have, look up its
   class in "Recipe fonts and their classes" and substitute the same way. Change every
   `--font` (and drop a `--weight` the substitute lacks; pixelox picks the nearest).
4. **Re-fit.** Widths differ between fonts. After substituting, fit headline lines again
   with `transform scale --size Wx` (T-fit-width) and look at the result.
5. **Nothing installed for the class?** Three options, in order:
   * Ask the user for the font files, or for permission to download a free look-alike
     (all the Google Fonts named here are open-licensed), put the `.ttf` / `.otf` files in
     a folder, and register it: `pixelox fonts add ./fonts`. The family name then works.
   * Shape a font you do have: T-stretch-type compresses, widens, slants, or fattens;
     T-stencil-type cuts stencil gaps.
   * Use a generic name: `sans-serif`, `serif`, or `monospace` always resolve (to the
     system default, and on a machine with no fonts at all to the embedded **Tuffy**, a
     plain medium-weight sans). With Tuffy alone, build hierarchy from size, capitals,
     tracking, color, and a same-color `fx stroke` for "bold", not from font changes.

Record the fonts you used, and any substitution, in your delivery note.

### Recipe fonts and their classes

| Font named in recipes | Class to substitute from |
| --- | --- |
| Impact, DIN Condensed, `Helvetica Neue` 900, Arial Narrow 700, `Futura` 800 | Ultra-condensed heavy grotesque |
| Avenir Next Condensed | Condensed gothic (regular weights) or, at 700 to 900, ultra-condensed heavy |
| Arial Black, `Avenir Next` 900, Gill Sans 700 | Heavy wide grotesque |
| Helvetica Neue, Helvetica, Arial, Verdana, Trebuchet MS, DIN Alternate | Neo-grotesque |
| Avenir Next, Avenir, Futura | Geometric sans (at 100 to 300: thin wide sans) |
| Arial Rounded MT Bold | Heavy rounded grotesque |
| Didot, Bodoni 72, Big Caslon | High-contrast didone |
| Baskerville, Times New Roman, Georgia | Old-style / transitional serif |
| Copperplate | Engraved / inscriptional caps |
| Rockwell, Superclarendon | Slab serif (Superclarendon 900: fat display) |
| Snell Roundhand, Zapfino, Savoye LET | Formal script (Savoye LET: hairline script) |
| Brush Script MT, SignPainter | Brush script |
| Marker Felt, Noteworthy, Bradley Hand | Marker and handwriting |
| Luminari | Uncial / medieval |
| Phosphate | Display art-deco / retro |
| Menlo | Monospace |

## Font classes

Seen across the poster set. "Look-alike" fonts are free (Google Fonts) and are the ideal;
the system columns are stand-ins that ship with the OS (Windows entries marked * come
with Microsoft Office). Pick the first one that `fonts list` reports.

| Class | Look, where it is used | Free look-alikes | macOS | Windows | Linux |
| --- | --- | --- | --- | --- | --- |
| Ultra-condensed heavy grotesque | tall, tight, loud; event flyers, film titles, type blocks | Anton, League Gothic, Bebas Neue, Oswald 700 | Impact, DIN Condensed, `Helvetica Neue` 900 | Impact, Bahnschrift Condensed (700) | Liberation Sans Narrow Bold, DejaVu Sans Condensed Bold |
| Ultra-compressed display (narrower than Impact) | skyscraper letters that fill a tall column | Six Caps, Antonio 700, Big Shoulders Display 900 | DIN Condensed plus T-stretch-type | Bahnschrift Condensed, Onyx* | Liberation Sans Narrow Bold plus T-stretch-type |
| Condensed gothic, regular weight | sub-heads, dates, billing blocks, labels | Oswald 400, Barlow Condensed, Roboto Condensed, Antonio 300 | Avenir Next Condensed 400 to 600, Arial Narrow | Bahnschrift Condensed (400), Arial Narrow* | Liberation Sans Narrow, DejaVu Sans Condensed |
| Hairline condensed caps | elegant tall thin titles, fashion | Antonio 100, Big Shoulders Display 100, Oswald 200 | `Avenir Next Condensed` 275 | Bahnschrift Light Condensed | Liberation Sans Narrow (no hairline: use small, tracked) |
| Heavy wide grotesque | chunky, confident; sale, sport, protest | Archivo Black, Inter 900, Rubik 900 | Arial Black, `Avenir Next` 900 | Arial Black, Segoe UI Black | DejaVu Sans Bold, Noto Sans Black |
| Extended grotesque, upright or italic | wide and fast: cars, sport, youth church, tech | Archivo Expanded 800, Unbounded, Krona One, Syncopate | Arial Black plus T-stretch-type (widen 1.2x, skew -12) | Arial Black plus T-stretch-type | DejaVu Sans Bold (already wide) plus skew |
| Neo-grotesque | neutral, Swiss, corporate | Inter, Archivo, Roboto, Work Sans | Helvetica Neue, Helvetica, Arial | Arial, Segoe UI | Liberation Sans, DejaVu Sans, Noto Sans |
| Geometric sans | round, modern, friendly tech | Montserrat, Poppins, Outfit, Jost, Mulish | Avenir Next, Futura, Avenir | Century Gothic*, Segoe UI | Noto Sans, Ubuntu, Cantarell |
| Thin wide sans | airy, premium tech, fashion | Montserrat 200, Raleway 200, Jost 200 | `Avenir Next` 275, `Helvetica Neue` 100 | Segoe UI Light | Noto Sans Light, Ubuntu Light |
| Thin monoline rounded sans | soft, friendly, wellness, kids tech | Quicksand, Comfortaa, Nunito 200, Varela Round | `Avenir Next` 275, Arial Rounded MT Bold (heavier) | Segoe UI Light | Ubuntu Light |
| Heavy rounded grotesque | food, kids, playful retail, chunky statements | Fredoka 700, Nunito 900, Baloo 2 800, Lilita One, Rubik 800 | Arial Rounded MT Bold (fatten with T-stretch-type) | Arial Rounded MT Bold*, Segoe UI Black | Ubuntu Bold |
| Squared techno sans | sci-fi, gaming, fintech, devices | Orbitron, Rajdhani, Chakra Petch, Oxanium, Michroma | DIN Alternate, Futura 700 caps tracked 60 | Bahnschrift, Agency FB* | Ubuntu Bold, DejaVu Sans Bold |
| High-contrast didone | glamorous, editorial, mastheads | Playfair Display, Bodoni Moda, DM Serif Display, Abril Fatface | Didot, Bodoni 72 | Bodoni MT*, Georgia Bold | DejaVu Serif Bold, Noto Serif Display |
| Condensed didone masthead | tall magazine nameplates | Bodoni Moda (condensed cut), Rozha One, Chonburi | Didot 700 plus T-stretch-type (0.8x width), Big Caslon | Bodoni MT Condensed*, Onyx* | DejaVu Serif Condensed Bold |
| Old-style / transitional serif | literary, calm, book covers | EB Garamond, Cormorant, Libre Baskerville, Lora | Baskerville, Hoefler Text, Palatino, Georgia | Georgia, Palatino Linotype, Cambria | Liberation Serif, Noto Serif, DejaVu Serif |
| Swash and decorative serif | fantasy and romance titles, one ornate capital | Cinzel Decorative, Cormorant Garamond Italic, IM Fell English, Playfair Display Italic | a Zapfino capital with Baskerville lowercase; Apple Chancery | Gabriola, Monotype Corsiva* | Liberation Serif Italic |
| Engraved / inscriptional caps | film credits, luxury, ceremony | Cinzel, Marcellus, Cormorant SC | Copperplate, Optima, Big Caslon | Copperplate Gothic*, Perpetua Titling* | DejaVu Serif (caps, tracked 100) |
| Uncial / medieval | fantasy, folklore, heritage | Uncial Antiqua, MedievalSharp, Metamorphous, New Rocker | Luminari, Herculanum | Old English Text MT*, Viner Hand ITC* | (register a font) |
| Blackletter | gothic, metal, tradition, mastheads | UnifrakturCook, Pirata One | none close: Luminari is the nearest | Old English Text MT* | (register a font) |
| Slab serif | sturdy, retro, western, editorial | Roboto Slab, Zilla Slab, Bitter, Arvo, Alfa Slab One | Rockwell, American Typewriter, Superclarendon | Rockwell*, Cambria Bold, Courier New Bold | DejaVu Serif Bold, Noto Serif Bold |
| Fat display: chunky slab, cut-paper, Cooper style | 1970s warmth, food, soul and funk posters, hand-cut lettering | Ultra, Chango, Titan One, Bowlby One, Shrikhand, Rubik 900 | `Superclarendon` 900; Arial Rounded MT Bold fattened | Cooper Black*, Rockwell Extra Bold* | DejaVu Serif Bold fattened |
| Formal script | weddings, worship, elegance | Great Vibes, Allura, Pinyon Script | Snell Roundhand, Zapfino | Segoe Script, Edwardian Script ITC* | (register a font) |
| Hairline and signature script | thin autograph accents, fashion, beauty | Mrs Saint Delafield, Sacramento, Parisienne, Dancing Script | Savoye LET, `Snell Roundhand` 500, Bradley Hand | Lucida Handwriting, Freestyle Script* | (register a font) |
| Brush script, also heavy script wordmarks | energy, handmade, lifestyle, retro soda marks | Yellowtail, Pacifico, Lobster, Kaushan Script | SignPainter, Brush Script MT | Brush Script MT*, Segoe Script Bold | (register a font) |
| Marker and handwriting, dry-brush caps | notes, street, zines, shouted captions | Permanent Marker, Rock Salt, Caveat Brush, Amatic SC, Patrick Hand SC | Marker Felt, Noteworthy, Bradley Hand, Chalkduster; or Impact with T-distress-type | Ink Free, Segoe Print | (register a font) |
| Stencil | military, street, industrial | Stardos Stencil, Allerta Stencil, Saira Stencil One | none: T-stencil-type on Impact | Stencil* | T-stencil-type on Liberation Sans Narrow Bold |
| Monospace | code, tech, receipts, captions | JetBrains Mono, IBM Plex Mono, Space Mono | Menlo, Courier New, Monaco | Consolas, Courier New | DejaVu Sans Mono, Liberation Mono |
| Display art-deco, unicase, retro | 1920s to 1970s posters, club nights | Limelight, Monoton, Righteous, Bungee, Fascinate | Phosphate (inline), Copperplate, Futura 700 | Broadway*, Gill Sans Ultra Bold* | (register a font) |

Family quirks on macOS: DIN Condensed exists only in bold; `Avenir Next` 300 snaps to
Ultra Light (use 400); `Baskerville` 600 is much heavier than 400; `Helvetica Neue` 900 and
`Futura` 800 are condensed faces.

### How wide a headline will be

Width of a 9-character all-caps line, as a multiple of its cap height (measured with
pixelox on macOS faces). Use it to predict whether a headline fits before you set it.

| Font (weight) | Width / cap height | Cap height / font size |
| --- | --- | --- |
| League Gothic, Anton (look-alikes) | about 3.0 to 3.7 | 0.72 |
| DIN Condensed (700) | 4.3 | 0.73 |
| Impact | 4.8 | 0.83 |
| Avenir Next Condensed (700) | 5.3 | 0.75 |
| Helvetica Neue (900, condensed black), Arial Narrow (700), Futura (800) | 5.7 | 0.75 to 0.79 |
| Helvetica Neue (300 / 700) | 6.5 / 7.0 | 0.75 |
| Avenir Next (500 / 900), Futura (700) | 6.7 / 7.3 | 0.75 to 0.80 |
| Didot (700), Bodoni 72 (700), Baskerville (700), Rockwell (700) | 6.8 to 7.7 | 0.69 to 0.77 |
| Arial Black, Gill Sans (700), Superclarendon (900) | 8.0 to 8.5 | 0.70 to 0.78 |
| Copperplate (700) | 9.7 | 0.56 |

Example: a 9-letter line in Impact with a cap height of 16% H on a 1080x1350 canvas is
`4.8 x 216 = 1037 px` wide: too wide for a canvas with 8% margins (918 px), so either the
cap height drops to 14% or the face changes to a narrower one.

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

Proven pairs (display / text), each with system stand-ins:

| Mood | Display | Text |
| --- | --- | --- |
| Loud event | Anton (Impact) | Montserrat 500 and 800 (Avenir Next) |
| Swiss / corporate | Inter 800 (Helvetica Neue 700) | Inter 400 (Helvetica Neue 400) |
| Editorial / fashion | Playfair Display 700 (Didot 700) | Jost 300, tracked caps (Avenir Next 400) |
| Elegant worship / gala | Great Vibes (Snell Roundhand 700) + Cinzel (Copperplate) | Montserrat 400 (Avenir Next 400) |
| Tech launch | Montserrat 200, wide (Avenir Next 275) | JetBrains Mono 400 (Menlo) |
| Retro / diner | Alfa Slab One (Rockwell 700) | Yellowtail (Brush Script MT) accent |
| Street / grunge | League Gothic (DIN Condensed) | Space Mono (Courier New 700) |
| Friendly food | Lilita One (Arial Rounded MT Bold) | Poppins 500 (Avenir Next 500) |

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

Run this to see the stand-in classes on your machine (macOS families; swap the names for
your system first, or the first missing family stops it). It is also a quick test that the fonts you plan to use resolve.

```bash
pixelox new -o specimen.pxo --size 1080x1350 --fill '#f3efe6'
pixelox text add specimen.pxo --in-place --name S1 --content "CONDENSED HEAVY" --font Impact --size 6% --color '#111111' --at 7%,5%
pixelox text add specimen.pxo --in-place --name S2 --content "DIN CONDENSED" --font "DIN Condensed" --weight 700 --size 6% --color '#111111' --at 66%,5%
pixelox text add specimen.pxo --in-place --name S3 --content "HEAVY WIDE" --font "Arial Black" --size 5% --color '#111111' --at 7%,13%
pixelox text add specimen.pxo --in-place --name S4 --content "Neo-grotesque" --font "Helvetica Neue" --weight 700 --size 5% --color '#111111' --at 7%,21%
pixelox text add specimen.pxo --in-place --name S5 --content "Geometric sans" --font "Avenir Next" --weight 500 --size 5% --color '#111111' --at 7%,29%
pixelox text add specimen.pxo --in-place --name S6 --content "THIN WIDE SANS" --font "Avenir Next" --weight 275 --size 4.4% --tracking 200 --color '#111111' --at 7%,37%
pixelox text add specimen.pxo --in-place --name S7 --content "Didone" --font Didot --weight 700 --size 6.5% --color '#111111' --at 7%,44%
pixelox text add specimen.pxo --in-place --name S8 --content "Old-style serif" --font Baskerville --size 5% --color '#111111' --at 45%,45%
pixelox text add specimen.pxo --in-place --name S9 --content "ENGRAVED CAPS" --font Copperplate --weight 400 --size 4.6% --tracking 120 --color '#111111' --at 7%,54%
pixelox text add specimen.pxo --in-place --name S10 --content "Slab serif" --font Rockwell --weight 700 --size 5.4% --color '#111111' --at 7%,61%
pixelox text add specimen.pxo --in-place --name S11 --content "Formal script" --font "Snell Roundhand" --weight 700 --size 6.5% --color '#111111' --at 7%,68%
pixelox text add specimen.pxo --in-place --name S12 --content "Brush script" --font "Brush Script MT" --size 6.5% --color '#111111' --at 62%,68%
pixelox text add specimen.pxo --in-place --name S13 --content "Rounded display" --font "Arial Rounded MT Bold" --size 5% --color '#111111' --at 7%,77%
pixelox text add specimen.pxo --in-place --name S14 --content "monospace 0123" --font Menlo --size 4% --color '#111111' --at 7%,85%
pixelox text add specimen.pxo --in-place --name S15 --content "INLINE DECO" --font Phosphate --size 5% --color '#111111' --at 7%,91%
pixelox export specimen.pxo -o specimen.png
```
