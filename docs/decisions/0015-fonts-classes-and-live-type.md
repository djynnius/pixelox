# ADR 0015: Font classes, Google Fonts with consent, substitution, and live type

Status: accepted (2026-11-22). Amends ADR 0005 (fallback font).

## Context
Agents built 126 posters with pixelox (the `pixelox-design` skill). Type caused most of the
friction:

* Recipes name macOS fonts. On any other machine `--font Impact` failed with PX4102 (exit 2),
  so every recipe broke; the skill had to teach a manual "decide the class, then look up a
  family per OS" procedure, and agents still guessed.
* Weight matching put weight before width: `Helvetica Neue` 900 and `Futura` 800 silently
  became *condensed* faces, so a weight request changed the width of a headline.
* About 60 posters fitted headlines to a width by hand (`transform scale --size 920x` plus
  `transform move`), stretched type with `transform scale --size WxH` (which rasterized it,
  PXW2003), and faked warps with perspective and skew (rasterized too).
* `text on-path` let letters run past the end of the path (or overlap at the start of a
  circle) without saying so.

## Decision

### Font classes
`--font @<class>` names one of 28 classes taken from the skill's typography survey
(`condensed-heavy`, `compressed`, `condensed`, `condensed-hairline`, `heavy-wide`, `extended`,
`neo-grotesque`, `geometric-sans`, `thin-sans`, `rounded-thin`, `rounded-heavy`, `techno`,
`didone`, `didone-condensed`, `oldstyle`, `swash-serif`, `engraved`, `uncial`, `blackletter`,
`slab`, `fat-display`, `script-formal`, `script-signature`, `brush`, `marker`, `stencil`,
`mono`, `deco`). Each class (`crates/pixelox-text/src/classes.rs`) has an ordered candidate
list: macOS, Windows, and Linux families, then curated Google Fonts families. Every candidate
carries the weight, width, and style that make it look like the class (Oswald is 700 in
`@condensed-heavy` and 400 in `@condensed`; Helvetica Neue is 900 *condensed*). A candidate is
a **member** or a **stand-in** (DIN Condensed for `@compressed`). Resolution: the first
installed member, then the first installed stand-in (warning PXW4014 naming the
`pixelox fonts get @class -y` command), then a related class, then the installed class
candidate whose measured metrics are closest to the class's target (see below), then the
genre's generic family, then Tuffy. System fonts come first, as the user asked; a downloaded
Google family counts as installed. Without `--weight` the candidate's weight is used, so the
spec stores `weight: null` ("automatic") and `text add --weight` lost its default of 400
(the behavior without the flag is unchanged for plain families).

### A missing family is replaced, with a warning
A family that is not installed no longer fails. It is replaced by the closest installed face
of its class: the class that lists the family with the nearest weight and width (or one
guessed from the name: `Mono`, `Slab`, `Condensed Black`, ...), resolved as above. Warning
PXW4010 names the requested family, the substitute, the class, and the command that gets the
real font (`pixelox fonts get "Bebas Neue" -y` for a Google family, else `pixelox fonts add`).
The layer keeps the requested family, so the same document draws the real font on a machine
that has it.

Why change an error into a warning: an agent cannot install fonts mid-task, and a hard stop
on a font name made every recipe non-portable, which is worse than a typographically close
substitute that is reported precisely. A wrong *class* would be worse than an error, so
substitution never crosses classes while a class member is installed, and crosses genres
(sans, serif, script, mono, display) only when nothing of the genre exists. CI and anyone who
needs the old contract keep it: `--strict-font` on `text add/edit/style` or
`PIXELOX_STRICT_FONTS=1` fails as before (PX4102), or with PX1540 (exit 6) when the family
can be downloaded.

### Google Fonts, downloaded with consent (ADR 0012's pattern)
`assets/google-fonts.json` pins 64 open-licensed families (OFL, Apache) that cover every
class to commit `06bfe0d62a98` of github.com/google/fonts, with the size and SHA-256 of every
font file and of each family's license file. `tools/gen_gfonts.py` regenerates it.

* `pixelox fonts get <family|@class> -y` downloads into `<data dir>/fonts/google/<family>/`,
  verifies every file before keeping it (PX1541 on a mismatch), keeps the license file, and
  records the install in `installed.json`. `<data dir>/fonts` is always on the font path, so
  the family works at once. `text add/edit/style -y` downloads a missing family (or the best
  Google member of a class with no installed member) before setting the layer.
* Without consent the download is refused with PX1540 (exit 6) and the command to run;
  `--dry-run` plans without consent or network; `PIXELOX_OFFLINE=1` forbids the network;
  files copied into the cache by hand are adopted when their hashes match.
* Over MCP, the server flag `--allow-font-downloads` and the call's
  `consent: ["font-download"]` are both required, and font consent only applies to font
  commands (`fonts get`, `text add/edit/style`), so it can never authorize a model download
  (and model consent never a font download).
* Any other family of the pinned commit can be fetched by name: its `METADATA.pb` lists the
  files, and the hashes seen on download are recorded (trust on first use) in the install
  record and, through the font records, in every sidecar that uses them.

### Weight matching follows CSS
`fonts::pick` orders candidates the way CSS Fonts 4 §5.2 does: width first (at or below
normal, narrower widths closest first, then wider; above normal the reverse), then style,
then weight, and a variable face matches any weight or width its axes cover. A weight request
can no longer change the width: `Helvetica Neue` 900 draws Bold (700) and warns (PXW4013,
listing the faces, `700 condensed, 900 condensed, 100, ...`), and `--stretch condensed` asks
for the condensed width explicitly (it also sets a variable font's `wdth` axis; Roboto,
Archivo, and League Gothic have one). The reports that `Avenir Next` 300 gives Ultra Light
and that `Baskerville` 600 looks much heavier are CSS-correct: Avenir Next has no Light face
and 275 is the nearest lighter weight, and Baskerville's SemiBold (600) is a heavy design. The
new `--faux-bold` offset gives the in-between weights those families lack. Saved documents
keep the face they were set with: the recorded PostScript name and file hash are preferred
when a layer is drawn again.

### Live type
New text settings stay live and are re-applied on every edit: `--case upper|lower|title`
(the content keeps what was typed), `--scale-x/--scale-y` (the set text scales about its
anchor, box and leading included), `--faux-bold` (stems thickened by N/1000 em with mitered
corners; advances grow with it), `--fit-width` / `--fit-box` (the size that makes the ink,
stroke included, span the width or fit the box, iterated to 1e-7 because strokes and optical
sizes are not proportional), and `--anchor-to ink` (`--at` places the visible letters, not the
text box). `transform scale` with unequal factors sets the text's size and `scale_x` instead of
rasterizing it (text turned by a multiple of 90° too; other angles still rasterize).

### Warp presets on the outlines
`text warp --preset P` and `text style --warp P` apply Photoshop's 15 warp styles (arc,
arc-lower, arc-upper, arch, bulge, shell-lower, shell-upper, flag, wave, fish, rise, fisheye,
inflate, squeeze, twist) with `--bend`, `--h-distort`, `--v-distort` (-100..100) and
`--orient horizontal|vertical` to the glyph outlines, inside the bounds of the unwarped ink.
Segments are subdivided (pieces of at most 4 px) and their control points mapped, so the type
stays vector, crisp, and editable. Adobe does not publish the formulas; `warp.rs` documents
the ones used (they approximate the shapes of Photoshop's presets, not its exact envelopes)
and is the reference for the raster `transform warp --preset`, which uses the same names and
parameter meanings.

### Replay
Sidecars already recorded each font file's SHA-256; downloaded fonts now also record their
source (`google-fonts:ofl/anton@06bfe0d62a98`). `replay` and `verify --full` check every
recorded font first and name what is missing and the command that gets it (PXW4011; with
`replay --strict`, PX4112, exit 9).

## Consequences
* Recipes that name macOS fonts render everywhere with a reported substitute, and recipes can
  name classes instead of families. On the skill's 126 poster recipes (macOS), 7 outputs
  changed: four ask for `Helvetica Neue --weight 900` and now draw the normal-width Bold
  instead of Condensed Black (p005, p035, p053, p090; adding `--stretch condensed` restores
  them exactly), and three stretch text with an unequal `transform scale`, which now stays
  live and crisp instead of being resampled as pixels (p038, p060, p122).
* `text add` without `--weight` records no weight; old sidecars record 400 and replay as before.
* Measured metrics (`metrics.rs`: cap-advance width over cap height, ink darkness) only rank
  installed faces when no class candidate exists; targets come from each class's first Google
  family and are stored in the class table.
* The warp formulas are pixelox's own; they match Photoshop's shapes, not its pixels.
