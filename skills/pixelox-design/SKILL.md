---
name: pixelox-design
description: 'Design posters, flyers, ads, banners, social posts, covers, invitations, and other graphic layouts from a brief with the pixelox CLI, working like an art director: pick a layout, palette, and type system from a library of 126 studied poster designs, build it in layers, look at it, critique it, and fix it. Use this whenever someone asks to make, design, or mock up a poster, flyer, ad, banner, social media post or story, thumbnail, magazine or book or album cover, event or church or concert flyer, sale or product promo, invitation, announcement, quote card, or "a graphic for X", including vague requests with no reference image ("make me something cool for our launch"), and when they ask for a named look (grunge, smoky, neon, duotone, halftone, Swiss, retro, stencil, gradient mesh, text behind the subject) or why a design looks amateur. Not for infographics, charts, or data graphics (pixelox-infographics), retouching a photo (pixelox-retouch), or plain edits such as crop, resize, and convert (pixelox).'
---

# Graphic design with pixelox

You are the art director and the production artist. pixelox places the pixels; you decide
what the design says first, second, and third, and you do not deliver until you have looked
at the result and fixed what is weak. A design that merely contains all the requested text
is not done.

Run pixelox with `--json` and read `result` (text layers report `bounds` as `[x, y, w, h]`
and `overflow`), `warnings`, and `error.hint`. Check flags with `pixelox docs <command>`.
Work in one `.pxo` with `--in-place`; export at the end.

Needs pixelox 0.5.0 or later (`pixelox update`; ask the user before updating). Check with
`pixelox --version`: older versions lack `--fit-width`, font classes, `fill noise`,
`render splash`, `shape icon`, and the other commands used here.

## What is in this skill

| File | Use it for |
| --- | --- |
| `references/index.md` | **read first**: which designs to start from for which brief, 12 worked combinations, the layout archetypes, lists by genre, mood, and needed material |
| `references/catalog.md` | one linked row per design; lists by color, style, format, technique |
| `references/posters/pNNN-*.md` | 126 studied designs: grid in percent, layer stack, palette, type table, a tested build recipe, variations, pitfalls |
| `references/techniques.md` | 80 tested effect recipes by ID (`T-smoke`, `T-grunge`, `T-duotone`, `T-fit-width`, `T-water-splash`, `T-info-bar`, ...); search for the heading, do not read it all |
| `references/typography.md` | fonts: the 28 font classes (`--font @condensed-heavy`) with free look-alikes, size scale, spacing, pairing |
| `references/palettes.md` | named palettes with hexes and area ratios |
| `scripts/*.js` | generators for `render js`, for what pixelox has no command for: blobs, grunge, rays, particles, brush, splatter, torn, crack, waves, ribbons, pattern, tiles, standin |

Copy `scripts/` next to your work (or use its full path); every script documents its
parameters in its first lines.

## Process

Follow the seven steps in order. Do not skip 3, 5, or 6.

### 1. Read the brief

Write down, one line each: the **purpose** (announce, sell, invite, inform), the **one
message** a passer-by must get in 2 seconds, the **facts** that must appear (what, who,
when, where, price, call to action), the **format**, the **material** you have (photos,
logo, brand colors), and the **mood** in three words.

**When the brief is vague** ("a poster for a jazz night next Friday", no image, no copy):
do not stall and do not ask a list of questions. Decide yourself:

* Format: social portrait 4:5 unless print is mentioned (then A4).
* Material: none, so choose designs that need no image (starred in `index.md`).
* Copy: write it. A headline of 1 to 4 words, one supporting line, and the facts, with
  honest placeholders for what you cannot know ("Venue Name", "8 PM", "12 Example Road").
  Work out real dates when the brief implies them ("next Friday").
* Mood: three words from the subject (jazz: warm, late, smooth).

State these assumptions when you deliver. Ask only for what would make the result wrong
if guessed (a real name, a logo, a photo the idea depends on), and build a first version
anyway. When the obvious idea needs a photo you do not have (a product ad with no product
shot, a speaker flyer with no portrait), deliver a finished photo-free version (type-led,
starred designs) and offer the photo-led layout as the next step.

Cut the copy. A poster carries a headline of 1 to 6 words, one supporting line, and the
facts. Longer text belongs in a caption or on a linked page; say so.

### 2. Choose references

Open `references/index.md`. Take candidates from section 1 (by use), check section 2 for
a ready combination, and pick 2 to 4 designs: the **layout archetype** from one, the
**palette** from another (or `palettes.md`), one **type treatment or effect** from a
third. Read those poster files. Write one line on what you take from each.

### 3. Plan before any command

Write the plan as text:

* Canvas and margins (6 to 9% of the width).
* Grid: the archetype's zones in percent; one alignment system (all centered, or all hung
  from one or two left edges).
* Palette: 3 to 5 hexes with area ratios near 60/30/10, and the type colors.
* Fonts: two at most, chosen by **class** with `references/typography.md`
  (`--font @condensed-heavy`, `@geometric-sans`, `@script-formal`). **Run
  `pixelox fonts classes` now** to see what each class draws on this machine. A class
  marked `stand-in` or `substitute`, or a named family that is missing, is drawn with the
  closest installed face and a warning (PXW4010, PXW4014), never an error. If the design
  depends on the real face, ask the user whether to download the free family, and only
  after a yes run `pixelox fonts get @stencil -y` (or a family name).
* Text table: every text element with its level, cap height in % of the height, position.
* Layer stack, back to front, with the technique ID for each effect.

Size scale for a portrait poster, cap heights in % of the canvas height (font size is cap
height x 1.39): headline 8 to 16 (type-led up to 30), key fact or sub-headline 2.5 to 4.5,
details 1.3 to 1.9, fine print 0.8 to 1.1. Ratio about **10 : 3 : 1.3 : 0.8**; every level
at least 1.6x the next; three or four sizes in total.

### 4. Build

| Format | Start with |
| --- | --- |
| Social portrait 4:5 | `pixelox new -o d.pxo --preset ig-portrait --fill '#101014'` |
| Square post | `pixelox new -o d.pxo --preset ig-post` |
| Story / reel cover 9:16 | `pixelox new -o d.pxo --preset ig-story` |
| Print poster or flyer | `pixelox new -o d.pxo --preset a4` (also `a3`, `letter`, `poster-18x24`) |
| Wide banner, thumbnail | `pixelox new -o d.pxo --preset x-post` (also `yt-thumb`, `og-card`, `fb-cover`) |

Build back to front: ground, background effect, imagery, big type, subject over type,
panels, small type, finishing.

* Name every layer (`--name`, `--as`) and address layers by name.
* Use percent for positions and sizes. Pixel values in recipes (blur, offsets, stroke
  widths) are for a 1080 px wide canvas: multiply by `width / 1080`.
* Fit headlines by width, not by guessing a size: give `text add` `--fit-width 920`
  instead of `--size`, and `--at 50%,18% --anchor t --anchor-to ink` to place the visible
  letters exactly (T-fit-width).
* One text layer per headline line. Body copy: `--content $'line one\nline two'` or `--box`.
* Use the native commands before a script: `fill noise` (smoke, clouds, mottle), `filter
  grain --as-layer`, `render splash|droplets|ice`, `shape icon|qr|barcode`, `fx
  long-shadow|extrude|reflection`, `text warp`, `mask add --gradient` (fades).
* Past a dozen commands, put them in a pipeline file (`pixelox docs pipelines`).

**Things that bite** (each cost a designer an hour):

* Line breaks need `$'A\nB'`; `"A\nB"` prints a backslash. No `|` and no three dots in
  `--content`. Quote every color. A value starting with a minus needs `=`
  (`--angle=-52`).
* A negative coordinate counts from the far edge (`--at -60,-60` is 60 px in from the
  right and bottom). To go **past** the left or top edge, write a sum: `--at 0-80,40%` is
  80 px left of the canvas, `50%-120` is 120 px left of center.
* New layers land on top of the stack. To put one elsewhere, add `--below NAME`, `--above
  NAME`, or `--into GROUP` to the command that creates it.
* Read `warnings` after every `text add`: a font that is not installed is replaced
  (PXW4010), and a weight the family lacks is swapped for the nearest (PXW4013).
* Filters are live on text, shapes, and placed images (blur a shape for a glow or a soft
  shadow; nothing to rasterize). A blur fades where its shape runs off the canvas: for
  light that bleeds off an edge use `fill gradient` or `paint stroke`.
* `fill ...` and `paint stroke` need a pixel layer (`layer add`). `transform skew`,
  `perspective`, and `flip` turn text into pixels (warning PXW2003): finish the wording
  first.
* `render droplets` and `render ice` capture what is behind them when they run: add them
  after the background and the product, and run them again after either changes. Bead a
  product before giving it a glow or shadow style, or the beads land on the glow too.
* Random effects (`fill noise`, `render splash`, `render ice`, `filter grain`) depend on
  `--seed` and on how many commands ran before them: after inserting or removing earlier
  lines, look again and re-pick `--seed` where smoke, a cube, or a splash now covers
  something.
* `layer clip` attaches a layer to the nearest unclipped layer below it.
* A `%` rect is % of width by % of height: use pixels for true circles and squares.
* `fx overlay --angle 90` runs bottom to top.

**Stand-ins and real photos.** With no photo yet, lay out with a stand-in
(`pixelox render js scripts/standin.js -o subject.png --size 900x1200 --param kind=person`;
kinds: person, bottle, can, box, skyline, logo, scene). Recipes are sized for the
stand-ins, which have empty space around the object. When the real image arrives: cut it
out (ask before downloading the model: `pixelox detect subject shot.pxo --in-place --save-mask subject -y`,
`pixelox mask refine shot.pxo --in-place --mask subject --decontaminate`,
`pixelox export shot.pxo -o subject.png --apply-mask subject`), trim it
(`pixelox transform trim subject.png -o subject-tight.png`), swap it in
(`pixelox comp relink d.pxo --in-place --layer Subject --to subject-tight.png`), then
re-check size and overlaps by eye. A stand-in proves the layout only. Never deliver one;
if no photo will come, switch to a design that needs none. Liquid, ice, and condensation
belong to the photo-led version: once the real product is in, stand it in a splash
(T-water-splash), bead it (T-condensation), and scatter ice (T-ice-cubes).

### 5. Look at it

```bash
pixelox export d.pxo -o preview.png --size 1080x --overwrite
```

Open `preview.png` and look at it (your image viewer, or the MCP `pixelox_preview` tool).
`--overwrite` is for your own preview file only. If you cannot view images, check by
numbers: every text layer's `bounds` lie inside the margins, no `overflow`, no two text
bounds intersect, and the ground behind each text block (`pixelox analyze stats preview.png --rect 80,900,400x60 --json`)
differs from the text color by at least 50 lightness points.

Critique against this list, in this order:

1. **Squint test.** At 25% size, is there one clear focal point, and is the headline
   still readable? If everything is equally loud, nothing is.
2. **Hierarchy.** Can you name the first, second, and third thing read? Is the biggest
   text the most important message, and at least 1.6x the next level?
3. **Edges and margins.** Nothing important within 4% of an edge; text margins equal on
   both sides; elements either clearly inside or clearly bleeding off, never just touching.
4. **Alignment.** Every text edge lines up with another edge or the center axis.
5. **Contrast.** All text readable at phone size; no text over a busy part of a photo
   without a panel, fade, or shadow.
6. **Color.** Only the planned palette; one accent; no accidental gray or muddy mixes.
7. **Space.** At least a quarter of the canvas is calm. Related items close together,
   unrelated items clearly apart.
8. **Type.** Two families at most; no lone word on a last line; tracking wide on tiny
   caps, tight on huge caps.
9. **Finish.** Grain or texture binds the layers (T-paper-grain); cutout edges are clean;
   shadows share one direction.
10. **Truth.** Every fact from the brief is present and spelled exactly as given.

### 6. Iterate

Fix the biggest problem first (usually hierarchy or contrast), re-export, and look again.
Make at least one full revision pass; two is normal. When a fix is a matter of taste
(palette A or B), build both and show both.

### 7. Deliver

```bash
pixelox export d.pxo -o poster.png
pixelox verify poster.png.pixelox.json --json
```

Report the output paths, the fonts used (and any substitution), your assumptions and
placeholder copy, anything still a stand-in, and one sentence on the design idea. Keep the
`.pxo`: it is the editable source.

## The rules behind the checklist

* **One hero.** One dominant element (a word, a face, or an object) takes 30 to 60% of
  the area. Everything else supports it or gets smaller.
* **Group the facts.** Date, time, and place form one block with an icon each
  (T-info-bar); the call to action is one separate element (T-pill-button, or T-qr-code
  when there is a link); the organizer's mark sits small at the top or bottom edge.
* **Overlap creates depth.** Let the subject cross the headline (T-text-behind-subject),
  a badge cross the product, a panel cross the photo's edge.
* **Repeat to unify.** The same accent color, corner radius, line weight, and shadow
  direction everywhere. Pull a color from the photo into the type.
* **Bleed or breathe.** Big shapes and photos run off the canvas; text stays inside the
  margins.
* **Texture last.** Grain, a soft vignette, or paper texture at low strength, above the
  art and below the small text.

## Amateur tells, and the fix

| Tell | Fix |
| --- | --- |
| Everything centered and the same size | Pick a hero; apply the 10 : 3 : 1.3 scale |
| Headline fills 60% of the width | Fit it to the margins, or make it clearly small |
| Three or more fonts; a script in all caps | Two families; script for one mixed-case word |
| Text straight on a busy photo | Fade, panel, or darken the photo behind it |
| Pure red on pure blue, rainbow gradients | One palette from `palettes.md`; one accent |
| Shadows, strokes, and glows on everything | One effect on one element |
| Photo pasted in with its rectangular background | Cut it out, or use it full-bleed |
| Clip-art icons, emoji, star bursts everywhere | One icon set (`shape icon`), one size, one color; or none |
| Stretched text or a squashed photo | Fit type with `--fit-width`; scale photos uniformly; crop, do not squeeze |
| Tiny text to fit long copy | Cut the copy |
| Equal gaps everywhere | Tight inside a group, wide between groups |

## Keep it original

The reference descriptions teach design language; they are not templates to clone.

* Combine at least two references, and change at least two of copy, palette, imagery, and
  type from any single one.
* Write new copy. Never reuse a reference's wording, and never use a real brand's name,
  logo, slogan, or signature look unless the user owns it or supplies it.
* Use only images the user supplied or that you generated; say so when an image is a
  stand-in. Do not imitate a real person's likeness or imply an endorsement.
* If the user asks for "the same as" a known poster, build something in that spirit and
  say what you changed.
