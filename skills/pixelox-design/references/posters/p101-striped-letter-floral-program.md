---
id: p101
title: Striped giant letter with floral fill
genre: [concert, event-flyer, exhibition]
mood: [fresh, elegant, playful, minimal]
style: [swiss, collage, illustration]
format: portrait-a
archetype: A-swiss-asymmetric
palette: ['#ffffff', '#e5e4e4', '#111111', '#b93832', '#405241', '#d98a90']
techniques: [T-image-in-type, T-vertical-type, T-rules, T-shape-crop]
needs: illustration
---

# p101: Striped giant letter with floral fill

## 1. Concept

A monthly club program on a white sheet. The picture is one giant letter, the venue's
initial, drawn as four parallel stripes that fill the whole canvas; a vintage botanical
illustration (poppies, peonies, foxgloves) shows only inside the stripes, and the venue's
short name runs vertically in black behind them. The dense program sits in a small column
at the bottom left. It works because a strict system (equal stripes, one angle, white
gaps as wide as the stripes) is filled with something lush and organic: order and
overgrowth in the same shape.

## 2. Format and grid

* Portrait A-series (1 : 1.414). Left margin 4.8% W; the art bleeds off the top, right,
  and bottom edges.
* The giant letter is built on a stripe module: stripe width 6.8% W, gap 6.5% W, pitch
  13.3% W. Diagonals run at 25 degrees from vertical (0.47 px sideways per px down).
* The letter's three parts: a left arm of four stripes falling from the top edge
  (starting at 3.7, 17, 30.4, 43.8% W) toward the lower right; a stem of four vertical
  stripes (52, 65.2, 78.5, 91.8% W) from 71.7% H to the bottom edge; a right arm where
  each stem stripe bends at 71.7% H and rises to the upper right, off the canvas.
* Only the outermost left stripe reaches the stem. The other three end on a slanted
  line parallel to the right arm (one gap to its left), with tips at `55%,57%`,
  `62%,47%`, `68.5%,37%`.
* Text zones: vertical word in a column 74.5 to 87.5% W, 20 to 86.5% H; month label at
  5% W, 56 to 58% H; program column 4.8 to 31% W, 63 to 93.5% H; address block centered
  on 79.7% W at 92 to 96% H. The upper left quarter is empty except for the stripes.

## 3. Layer stack

1. Ground: white `#ffffff`.
2. Vertical word: 4 letters and an exclamation mark, black `#111111`, rotated to read
   from bottom to top, letter height (now horizontal) 13% W, running from 86.5% H up to
   20% H with gaps of about 6% H between letters.
3. Stripes: the giant letter as one shape, light gray `#e5e4e4`. It lies over the word,
   so the word shows only in the white gaps: about 45% of each letter is hidden.
4. Illustration: a tall bouquet, about `22%,2%,78% x 96%`, clipped to the stripes. Red
   poppies sit in the left arm at 35 to 62% H, pink peonies in the right arm at 45 to
   70% H, purple bells at 15 to 32% H, stems running down the first stem stripe. Where
   the art is empty the stripes stay gray.
5. Month label: 2 lines (month, year), black, extra-bold caps, at 5% W, 56.4% H.
6. Program: 11 entries of 1 to 4 lines in tiny tracked bold caps, each starting with a
   2-digit day; left edge 4.8% W, top 63.3% H, line pitch 0.9% H.
7. Rules: 11 black hairlines, 4.8 to 31% W, one under each entry.
8. Notes: a 2-word italic serif label, right-aligned at 31% W on the first line of 5 of
   the entries.
9. Footnote: 2 bold lines, tighter tracking, at 4.8% W, 94.5% H.
10. Address: 2 centered lines of widely tracked bold caps at 92.6% and 95% H, each
    underlined by a 2 px rule (28% W and 9.5% W long).

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#ffffff` | ground and gaps | 57% |
| `#e5e4e4` | empty stripes | 17% |
| `#111111` | all type, rules, the darkest leaves | 8% |
| `#b93832` | poppies (the accent that carries the eye) | 6% |
| `#405241` | foliage green | 6% |
| `#d98a90` | peony pink | 6% |

White, one whisper of gray, black type, and then whatever the illustration brings. The
design itself has no color: change the illustration and the palette changes with it.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Vertical word | `[VENUE]`, 4 letters + "!" | caps | heavy geometric sans (Montserrat Black / Avenir Next Heavy) | 900 | 13% W tall letters (9.2% H) | 460 | - | rotated -90 | `#111111` | left edge 74.5%, bottom 86.5% |
| Month | month / year, 2 lines | caps | same | 900 | 0.8% | 60 | 100% | left | `#111111` | 5%, top 56.4% |
| Program | 11 entries: day + act, then 0 to 3 lines of support acts starting with "+" | caps | geometric sans (Montserrat Bold / Avenir Next) | 700 | 0.6% | 220 | 104% | left | `#111111` | 4.8%, top 63.3% |
| Note | "In Concert"-style 2 words | title | transitional serif italic (Libre Baskerville Italic / Baskerville) | 400 italic | 0.55% | 0 | - | right | `#333333` | right edge 31% |
| Footnote | 2 lines, weekly regulars | caps | geometric sans (Montserrat / Avenir Next) | 700 | 0.6% | 40 | 104% | left | `#111111` | 4.8%, top 94.5% |
| Address | street line / city line | caps | same | 700 | 1% | 300 | - | center | `#111111` | 79.7%, 92.6% and 95% |

One geometric sans in two weights, plus a serif italic used five times as a tag. The
small type is very small (cap height 0.6% H): it is a printed program read at arm's
length. For a screen format raise it to 1% H and cut the list to 6 entries.

## 6. Imagery

A scanned antique botanical plate: a loose bouquet with long stems, cut out of its
background, with saturated reds and pinks against dark green leaves. It is not treated
at all (no grade, no texture); its only treatment is the mask. Any image with a wide
value range and organic detail works inside the stripes: foliage, fruit, feathers,
marbled paper, a crowd photo, fabric. Scale it so its largest forms (flower heads) are
2 to 3 stripes wide; smaller detail turns to noise behind the bars.

## 7. Effects and textures

* Stripe mask: one `shape path` whose `--d` holds seven closed polygons (the outer
  letter stroke with both arms and the stem, three shorter left-arm stripes, three more
  stem-and-right-arm stripes). Because it is a single layer, one `layer clip` puts the
  illustration into all stripes at once: the **T-image-in-type** move with a drawn letter
  in place of a typed one.
* **T-vertical-type** for the venue word, with one change: it is added before the
  stripes, so they cover it.
* **T-rules**: all eleven program rules are one `shape path` of `M x y H x` segments
  with a 1.5 px stroke; the two address underlines are a second one.
* Flat otherwise: no shadow, no grain, no gradient.
* **T-shape-crop** in [techniques.md](../techniques.md) is the tested library version of the photo shown through a drawn shape (arch, panel, tiles, or letter); start from it when reusing this idea.

## 8. Hierarchy and eye path

1. The red flowers in the left arm (the only saturated color, near the optical center).
2. The stripe letter as a whole, which the eye follows up both arms and down the stem.
3. The black vertical word, read in pieces through the gaps.
4. The month label, then the program list; the address last.

Scale: the letter is 100% H; the vertical word's letters are 9.2% H; the address 1% H;
the program 0.6% H. Ratio about 165 : 15 : 1.7 : 1. Nothing sits between "poster-size"
and "caption-size", which is what makes it feel Swiss.

## 9. Build recipe

`illustration.png` is your image here: cut-out art with transparency (or use an opaque
photo; it then fills every stripe). Polygon coordinates are for 1080 x 1528.

```bash
pixelox new -o club.pxo --size 1080x1528 --fill '#ffffff'
pixelox text add club.pxo --in-place --name Word --content "YARD!" --font "Avenir Next" --weight 900 --size 12.9% --tracking 460 --color '#111111' --at 50%,50% --anchor c
pixelox transform rotate club.pxo --in-place --layer Word --degrees -90
pixelox transform move club.pxo --in-place --layer Word --to 74.5%,86.5% --anchor bl
pixelox shape path club.pxo --in-place --name Stripes --d 'M 40 0 L 114 0 L 596 1021 L 1079 0 L 1153 0 L 635 1096 L 635 1528 L 562 1528 L 562 1105 Z M 184 0 L 259 0 L 633 793 L 596 872 Z M 328 0 L 403 0 L 705 640 L 667 719 Z M 473 0 L 547 0 L 777 487 L 740 566 Z M 704 1528 L 704 1096 L 1221 0 L 1295 0 L 778 1096 L 778 1528 Z M 848 1528 L 848 1096 L 1365 0 L 1439 0 L 921 1096 L 921 1528 Z M 991 1528 L 991 1096 L 1509 0 L 1582 0 L 1065 1096 L 1065 1528 Z' --fill '#e5e4e4'
pixelox comp place club.pxo illustration.png --in-place --as Art --size x1200 --at 62%,52% --anchor c
pixelox layer clip club.pxo --in-place --layer Art
pixelox text add club.pxo --in-place --name Month --content $'APRIL\n2031' --font "Avenir Next" --weight 900 --size 1.15% --tracking 60 --leading 100% --color '#111111' --at 5%,55.8% --anchor tl
pixelox text add club.pxo --in-place --name Program --content $'02 DJ FIRST NAME\n\n03 SWING NIGHT\nTHE HOUSE BAND\n+DJ SECOND\n\n04 DJ RADIO HOUR\n\n05 DJ LATE SHIFT\n\n10 OPEN STRINGS\n+DJ THIRD (GUEST SET)\n\n11 SPRING FESTIVAL\nCONCERT IN THE YARD\n+DJ FIRST NAME\n\n17 FLAMINGO PARTY\nVINTAGE MARKET\n13TH. THE HEADLINERS\n+DJ SECOND\n\n18 SPRING FESTIVAL\nTHE NEWCOMERS\n+DJ THIRD\n\n24 DJ SOUL HOUR\n\n25 SPRING FESTIVAL\nTHE LATE SHOW\n+DJ SECOND\n\n30 FIRST BIRTHDAY!\nDJ FIRST + DJ SECOND' --font "Avenir Next" --weight 700 --size 0.85% --tracking 220 --leading 104% --color '#111111' --at 4.8%,63% --anchor tl
pixelox shape path club.pxo --in-place --name Rules --d 'M 52 984 H 335 M 52 1038 H 335 M 52 1065 H 335 M 52 1092 H 335 M 52 1133 H 335 M 52 1187 H 335 M 52 1254 H 335 M 52 1308 H 335 M 52 1335 H 335 M 52 1389 H 335 M 52 1430 H 335' --fill none --stroke '#111111' --stroke-width 1.5
pixelox text add club.pxo --in-place --name Notes --content $'In Concert\n\n\nIn Concert\n\n\n\nIn Concert\n\n\n\n\nIn Concert\n\n\n\n\n\nIn Concert' --font Baskerville --style italic --size 0.85% --leading 104% --align right --color '#333333'
pixelox transform move club.pxo --in-place --layer Notes --to 335,1101 --anchor tr
pixelox text add club.pxo --in-place --name Footnote --content $'THURSDAYS: THROWBACK SET WITH DJ FIRST\nSUNDAYS: DJ SECOND' --font "Avenir Next" --weight 700 --size 0.8% --tracking 40 --leading 104% --color '#111111' --at 4.8%,94.3% --anchor tl
pixelox text add club.pxo --in-place --name Street --content "12 EXAMPLE STREET" --font "Avenir Next" --weight 700 --size 1.35% --tracking 300 --color '#111111' --at 79.7%,92.6% --anchor c
pixelox text add club.pxo --in-place --name City --content "YOUR CITY" --font "Avenir Next" --weight 700 --size 1.35% --tracking 300 --color '#111111' --at 79.9%,95% --anchor c
pixelox shape path club.pxo --in-place --name AddressRules --d 'M 713 1435 H 1015 M 810 1469 H 913' --fill none --stroke '#111111' --stroke-width 2
pixelox export club.pxo -o club.png
```

How the list lines up: the program is one text layer with an empty line after each entry;
its line pitch is `0.85% x 1528 x 1.04 = 13.5 px` and its first cap top lands at y 966.
A rule for the blank line number `i` (counting from 0) sits at `966 + 4.6 + 13.5 i`.
The notes layer uses the same size and leading, so blank lines inside it step in the
same rhythm. Change first: the illustration and its `--size` and `--at`; then the word
(4 to 5 characters; adjust `--tracking` until it spans 20 to 86.5% H); then the program
(recount the blank lines and move the rules).

Other letters: keep the module (stripe 74 px, pitch 143 px, slope 0.472) and redraw. A
"V" is this path without the stem; an "A" is it upside down with one horizontal stripe;
"M", "N", "W", "K", "X", "Z" all work. Round letters do not suit straight stripes.

## 10. Variations and pairings

* Dark edition: ground `#111111`, stripes `#1e1e1e`, type white, a moody illustration
  (night flowers, smoke, a T-fire-glow render placed as the art).
* One-color print: run T-duotone on the illustration (`#1b1464` to `#ff3d7f`) so the
  stripes carry a brand pair instead of full color.
* Photo edition: a portrait or a crowd photo inside the stripes for a festival or a
  fashion sale; the white gaps act like blinds.
* Fewer, wider stripes (3 at 9% W) for small formats; 6 thin ones for a quieter look.
* Suits club and venue programs, festival line-ups, garden and flower shows, spring
  sales, a monthly series where only the illustration and the list change.
* Pairs with T-paper-grain (amount 10, opacity 30) for a risograph feel and T-pattern
  stripes when you want a stripe field without the letter.

## 11. Pitfalls

* Unequal stripe widths or angles: the letter stops reading and the picture looks like
  torn strips. Compute every polygon from one width, one pitch, one slope.
* Gaps narrower than the stripes: the image wins and the letter disappears. Gaps must
  be 90 to 100% of the stripe width.
* An illustration with a solid background fills every stripe edge to edge and hides the
  gray; the letter then reads as a striped photo. Use cut-out art, or accept that look
  on purpose.
* The vertical word placed above the stripes becomes a plain label. It must go under
  them, and it must be short and widely spaced, or the hidden parts make it unreadable.
* Program text over the stripes shimmers. Keep the small text in the white areas (the
  bottom-left corner) and let only the two address lines touch the stem.
* Centering the list or the month label. Everything small hangs from the 4.8% W margin.
