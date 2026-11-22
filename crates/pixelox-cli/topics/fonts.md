# Fonts: classes, Google Fonts, and substitution

pixelox draws text from the fonts installed on the machine, the folders registered with
`fonts add`, Google Fonts families downloaded with `fonts get`, and the embedded Tuffy
fallback. A recipe should name what a font *is*, not which file it is: a font class.

## Font classes

`--font @class` picks the best installed face of a class. There are 28, from the design
skill's survey of real posters:

| Class | Looks like | Free family (`fonts get`) |
| --- | --- | --- |
| `@condensed-heavy` | Impact, DIN Condensed, Helvetica Neue Condensed Black | Anton, League Gothic, Bebas Neue, Oswald 700 |
| `@compressed` | narrower than Impact: skyscraper letters | Six Caps, Antonio 700, Big Shoulders Display 900 |
| `@condensed` | condensed gothic, regular weight | Oswald, Barlow Condensed, Roboto condensed |
| `@condensed-hairline` | tall thin titles | Antonio 100, Big Shoulders Display 100 |
| `@heavy-wide` | Arial Black, Avenir Next Heavy | Archivo Black, Inter 900 |
| `@extended` | wide and fast | Archivo expanded 800, Unbounded |
| `@neo-grotesque` | Helvetica, Arial | Inter, Roboto, Archivo |
| `@geometric-sans` | Avenir, Futura | Montserrat, Poppins, Jost |
| `@thin-sans` | thin wide sans | Montserrat 200, Raleway 200 |
| `@rounded-thin` | thin monoline rounded | Quicksand, Comfortaa, Nunito 200 |
| `@rounded-heavy` | Arial Rounded MT Bold | Fredoka 700, Nunito 900, Lilita One |
| `@techno` | squared sci-fi sans | Orbitron, Rajdhani |
| `@didone` | Didot, Bodoni | Playfair Display, Bodoni Moda, DM Serif Display, Abril Fatface |
| `@didone-condensed` | tall magazine nameplates | Rozha One |
| `@oldstyle` | Baskerville, Garamond, Georgia | EB Garamond, Libre Baskerville, Lora |
| `@swash-serif` | ornate fantasy capitals | Cinzel Decorative, Playfair Display Italic |
| `@engraved` | Copperplate, Optima | Cinzel, Marcellus |
| `@uncial` | Luminari: medieval | Uncial Antiqua, MedievalSharp |
| `@blackletter` | gothic mastheads | UnifrakturMaguntia, UnifrakturCook, Pirata One |
| `@slab` | Rockwell | Roboto Slab, Zilla Slab, Alfa Slab One |
| `@fat-display` | Cooper Black, Superclarendon Black | Ultra, Titan One, Shrikhand |
| `@script-formal` | Snell Roundhand, Zapfino | Great Vibes, Allura |
| `@script-signature` | Savoye: hairline autograph | Mrs Saint Delafield, Sacramento |
| `@brush` | Brush Script, SignPainter | Pacifico, Yellowtail, Lobster |
| `@marker` | Marker Felt, Noteworthy | Permanent Marker, Caveat, Amatic SC |
| `@stencil` | military, crates | Allerta Stencil, Stardos Stencil, Saira Stencil One |
| `@mono` | Menlo, Consolas | JetBrains Mono, Space Mono |
| `@deco` | Phosphate, Broadway | Limelight, Monoton, Righteous |

```bash
pixelox fonts classes
pixelox fonts classes @stencil --json
pixelox text add poster.pxo --in-place --content "MAKE NOISE" --font @condensed-heavy --size 12% --at 50%,10% --anchor t
```

Each class lists macOS, Windows, and Linux families first, then the Google families. It
resolves to the first installed *member*; failing that to a *stand-in* (DIN Condensed for
`@compressed`, warning PXW4014 with the download command); failing that to a related class
or the installed face with the closest measured width and weight. Without `--weight` the
class's own weight is used (Oswald is 700 in `@condensed-heavy`). `fonts classes` shows what
each class resolves to on this machine.

## A family that is not installed

`--font "Bebas Neue"` on a machine without it draws the closest installed face of its class
and warns (PXW4010) with the substitute and the command that gets the real font. The layer
keeps the requested family, so the same document draws Bebas Neue where it is installed.
`--strict-font` (or `PIXELOX_STRICT_FONTS=1`, for CI) fails instead: PX4102, or PX1540
(exit 6) when the family can be downloaded.

```bash
pixelox text add poster.pxo --in-place --content "FESTIVAL" --font "Bebas Neue" --size 15% --json
pixelox text add poster.pxo --in-place --content "FESTIVAL" --font "Bebas Neue" --size 15% --strict-font --json
```

## Google Fonts, with consent

About 60 open-licensed families that cover every class are pinned to one commit of
github.com/google/fonts, with the SHA-256 of every file. `fonts get` downloads a family (with
its license file) into the data directory and it works at once. It needs consent: `-y` on
the command line (PX1540 and exit 6 without it), `consent: ["font-download"]` over MCP (the
server needs `--allow-font-downloads`). `PIXELOX_OFFLINE=1` forbids the network; files copied
into the cache by hand are adopted when their hashes match.

```bash
pixelox fonts get "Bebas Neue" -y
pixelox fonts get @stencil @blackletter -y
pixelox fonts get Anton --dry-run --json
pixelox fonts get --all -y
pixelox text add poster.pxo --in-place --content "STENCIL" --font @stencil --size 14% -y
```

`text add/edit/style -y` downloads a missing family (or the best free member of a class with
nothing installed) before setting the layer. Any other Google Fonts family can be fetched by
name too; its hashes are recorded when it is downloaded. `PIXELOX_SYSTEM_FONTS=0` hides the
system fonts but keeps the downloaded (hash-pinned) families, so a CI machine that ran
`fonts get --all -y` renders the same type everywhere.

## Weight and width

Faces are matched the CSS way: width first, then style, then weight. A weight never changes
the width: `Helvetica Neue` 900 draws Bold (700) and warns (PXW4013, listing the faces).
Ask for the width with `--stretch`:

```bash
pixelox text add poster.pxo --in-place --content "HEADLINE" --font "Helvetica Neue" --weight 900 --stretch condensed --size 10%
pixelox text add poster.pxo --in-place --content "WIDE" --font Archivo --weight 800 --stretch expanded --size 10%
```

`--stretch` takes `ultra-condensed` .. `ultra-expanded` (also `narrow`, `extended`) or a
percent, and sets the `wdth` axis of a variable font. For a weight between two faces, set the
lighter one and add `--faux-bold` (1/1000 em of extra stem; about 20 per weight step).

## Fonts and replay

Every output's sidecar records each font file's SHA-256 (and where a downloaded font came
from). `replay` and `verify --full` say which recorded fonts are missing here and the command
that gets each (PXW4011; `replay --strict` fails with PX4112).

```bash
pixelox replay poster.png.pixelox.json -o again.png --json
pixelox verify poster.png.pixelox.json --full --json
```
