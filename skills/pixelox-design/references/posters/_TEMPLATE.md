# Poster description template

One file per poster: `references/posters/<id>-<slug>.md` (`p008-fiery-revival-flyer.md`).
The slug is 2 to 5 lowercase words that describe the design, never a brand, person, title,
or the source file's name. Copy the skeleton at the end of this file and fill every section.
Finished examples: [p001](p001-dark-swiss-conference.md), [p008](p008-fiery-revival-flyer.md),
[p025](p025-stencil-portrait-print.md).

The reader is an agent that has **never seen the poster** and must build something in the
same design language with the pixelox CLI. Write numbers, not adjectives: "cap height 16% of
the canvas height", not "huge"; `#b3200e`, not "deep red".

## Content rules (not optional)

1. **Describe the design, not the brand.** Replace every real brand, logo, product name,
   film, show, or book title, publication name, church or company name, and real person's
   name with a neutral placeholder or a generic noun: `[BRAND]`, `[EVENT NAME]`,
   `[SPEAKER NAME]`, "a cola bottle", "a male singer with a guitar", "a news-magazine
   masthead". Never describe who a real person is; describe pose, framing, clothing, light.
2. **Do not quote the copy.** Only generic words may appear as they are ("SALE", "SUNDAY
   SERVICE", "PRESENTS", "FREE ENTRY", dates, times, prices). For everything else give the
   pattern: "2-word all-caps headline, 5 to 7 letters per line", "3-line italic quote, about
   12 words". In the build recipe, write **new, invented copy** with the same shape.
3. **Never mention the source file name, a URL, a watermark, or a stock-site mark.**
   Ignore watermarks and template badges ("editable in ...") as if they were not there.
   If the image is a mockup (a framed print, a book on a table, a billboard), describe the
   flat design inside it and say in one line how it was presented.
4. **Reusable language, not a clone.** Say why it works and what to vary. The recipe must
   build an original piece in this style, not a replica of the poster.

## How to measure

* Origin is the top-left corner. Positions and sizes are **percent of the canvas width (W)
  or height (H)**: x and widths in %W, y and heights in %H. Give boxes as
  `x,y,w x h` (`7%,81%,86% x 11%`).
* Type size is the **cap height in %H** (height of a capital letter). Font size for
  `--size` is about cap height / 0.72 (Impact: / 0.8); see `typography.md`.
* Colors are sampled, not guessed: `pixelox analyze stats view/p008.png --rect 540,300,9x9 --json`
  gives the mean color of a 9 px square, and `pixelox analyze colors view/p008.png --json`
  the dominant palette. Round to a clean hex. Area ratios are estimates to the nearest 5%.
* For fine print, crop the original into the scratchpad and read that
  (`pixelox export <original> -o lab/crop.png --region X,Y,WxH`). Never write into the
  originals' folder.

## How to write commands

The repo's contract test will parse every documented command, and your recipe will be run.

* Put commands in fenced `bash` blocks, one command per line, each line starting with
  `pixelox ` (no variables, no `cd`, no pipes, no loops). A long line may be split with a
  trailing `\`.
* In running text, name commands **without** the `pixelox ` prefix (`fx fire`,
  `layer clip`). An inline code span that starts with `pixelox ` must be a complete, valid
  command.
* Quote every color (`'#ff5a1f'`): an unquoted `#` starts a shell comment.
  Inside a text string, never put a space before a `#` ("#tag @name", not "@name #tag"):
  the checker cuts a command at a space followed by `#`.
* Do not use `<placeholders>`, `...`, `$(...)`, or `${...}` inside a command.
* A value that begins with a minus sign needs `=` (`--shadows=-12,0,18`).
* The recipe starts with `new`, works in one `.pxo` with `--in-place`, and ends with one
  `export ... -o <name>.png`. Canvas: 1080 px wide at the poster's aspect ratio
  (4:5 = 1080x1350, A-series = 1080x1528, 2:3 = 1080x1620, 9:16 = 1080x1920, square =
  1080x1080, 3:4 = 1080x1440, landscape 16:9 = 1600x900).
* Images the user must supply are named `subject.png` (cutout with transparency),
  `subject2.png`, `product.png` (cutout), `photo.png` and `photo2.png` (opaque photos),
  `illustration.png` (drawn art with transparency), `logo.png`. Mark each one "your image
  here" in the recipe notes; do not generate stand-ins inside the recipe. Generators for
  backgrounds and textures are `scripts/<name>.js`.
* Use fonts that `fonts list` reports, and name the ideal free look-alike in the text
  table. A missing font is an error, not a fallback.
* Cite techniques by ID from [../techniques.md](../techniques.md) and reuse their commands;
  change only the parameters. If the poster needs an effect the library lacks, build your
  best approximation in the recipe and say so in section 7, so a technique can be added.
* No `|` and no three dots in a row inside `--content` (such commands are skipped by the
  contract test), and no real brand, title, or person's name in any recipe string.

## Sections (all required, in this order)

**Front matter.** YAML between `---` lines, used to build the index:

| Key | Value |
| --- | --- |
| `id` | `p008` |
| `title` | 3 to 7 words, the design in plain words |
| `genre` | 1 to 3 of: event-flyer, church, concert, conference, product-ad, food-ad, drink-ad, tech-ad, fashion, magazine-cover, book-cover, film-poster, exhibition, corporate, sale-promo, social-post, sports, editorial |
| `mood` | 2 to 4 of: bold, calm, dark, dramatic, elegant, energetic, festive, fresh, friendly, futuristic, gritty, intense, luxurious, minimal, moody, mysterious, playful, reverent, retro, serious, urgent, warm, cool |
| `style` | 1 to 3 of: swiss, editorial, minimal, photo-composite, type-led, illustration, geometric, gradient-mesh, grunge, collage, stencil, retro, duotone, halftone, 3d-render, neon, flat |
| `format` | `portrait-4x5`, `portrait-a`, `portrait-2x3`, `portrait-3x4`, `portrait-9x16`, `square`, `landscape` |
| `archetype` | one layout archetype ID from [../index.md](../index.md) |
| `palette` | 3 to 6 hexes, largest area first |
| `techniques` | technique IDs used |
| `needs` | what the user must supply: `nothing`, `subject-cutout`, `product-cutout`, `photo`, `illustration`, or a list |

**1. Concept.** One or two sentences: what it is, and the single idea that makes it work.

**2. Format and grid.** Aspect ratio; margins in %W; the column or axis structure; where
the zones are (for example "top 47% photo, bottom 53% text on black").

**3. Layer stack.** Numbered, back to front. Every visible thing appears once, with its
box in % and its color or treatment.

**4. Palette.** Table: hex, role, area %. Then one line on the scheme (for example
"monochrome red plus white; 60/30/10").

**5. Text elements.** Table with one row per text element:
role | content pattern | case | font class (free look-alike / local stand-in) | weight |
cap height %H | tracking | leading | align | color | position (x %W, y %H of the anchor).

**6. Imagery.** What the picture shows in neutral terms, its crop and scale, lighting,
and how it was treated (cutout, grade, duotone, blur), citing technique IDs.

**7. Effects and textures.** Each effect, where it sits in the stack, and its technique
ID with the parameters that matter.

**8. Hierarchy and eye path.** What is read first, second, third, and why (size, contrast,
position). The size ratio between the levels.

**9. Build recipe.** One `bash` block that runs start to finish, plus notes on which
inputs are "your image here" and what to change first.

**10. Variations and pairings.** 3 to 5 concrete ways to vary it (palette swaps, other
genres it suits, other formats), and which other techniques or archetypes it combines with.

**11. Pitfalls.** 3 to 5 specific ways a copy of this design goes wrong, each with the fix.

## Verify before you hand in

1. The recipe block runs with every command exiting 0, in an empty folder that holds only
   `scripts/` and the stand-in images.
2. You exported the result and looked at it next to the poster: same layout zones, same
   hierarchy, same palette, same kind of type. Fix the recipe until it reads as the same
   design language.
3. No brand, title, real name, verbatim copy, file name, or URL appears anywhere in the file.
4. Every technique ID you cite exists in `techniques.md`, and the archetype in `index.md`.
5. Run `python3 build-index.py` in the skill folder: it rebuilds the tables in `index.md`
   and `catalog.md` from the front matter.

## Skeleton

````markdown
---
id: pNNN
title: Short plain-words title
genre: [event-flyer]
mood: [bold, warm]
style: [photo-composite]
format: portrait-4x5
archetype: A-event-stack
palette: ['#000000', '#ffffff', '#ff0000']
techniques: [T-fit-width, T-paper-grain]
needs: subject-cutout
---

# pNNN: Short plain-words title

## 1. Concept

## 2. Format and grid

## 3. Layer stack

1. ...

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |

## 6. Imagery

## 7. Effects and textures

## 8. Hierarchy and eye path

## 9. Build recipe

```bash
pixelox new -o poster.pxo --size 1080x1350 --fill '#000000'
pixelox export poster.pxo -o poster.png
```

## 10. Variations and pairings

## 11. Pitfalls
````
