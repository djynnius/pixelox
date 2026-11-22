---
id: p126
title: Sliced black type lecture poster
genre: [conference, exhibition, event-flyer]
mood: [bold, serious, minimal]
style: [swiss, type-led, minimal]
format: portrait-a
archetype: A-type-block
palette: ['#eeeaec', '#0a0a0a', '#55505a']
techniques: [T-fit-width, T-rules, T-paper-grain, T-sliced-type]
needs: nothing
---

# p126: Sliced black type lecture poster

## 1. Concept

A lecture-series poster made of nothing but black type on off-white paper: a two-word
title in a very heavy grotesque is broken into five lines of two to four letters, stacked
with almost no line gap, and seven of its letters are sliced by 45-degree cuts as if the
sheet had been folded or scored. The small text answers the cuts: the speaker list steps
to the right line by line, along the same diagonal. One idea (the diagonal), applied to
both the giant and the tiny type, is the whole design. Presented as a print on a gray
wall; the wall is not part of the design.

## 2. Format and grid

* Portrait A-series (1 : 1.414). Margins: 4% W left, 3.5% H top, 8% H bottom.
* Two vertical axes. The main axis is at 25% W: lines 2, 4, and 5 of the title, the
  second letter of lines 1 and 3, and the fine print all start there. A hanging column
  (4 to 23.5% W) holds one letter on line 1 and one on line 3: the first and the last
  letter of the first word. The 1.5% W gap between hanging column and main axis is the
  normal letter gap, so the lines still read as words.
* Title rows: five rows with a pitch of 14.5% H, cap tops at 3.5, 18, 32.5, 47, 61.5% H.
  Cap height 13.5 to 14% H, so the gap between rows is under 1% H.
* The title fills 4 to 80% W, 3.5 to 75.5% H. Its right edge is ragged (rows end at 50,
  79, 80, 80, 61% W); the last row is short and leaves a pocket at its right.
* Small text: sub-title and date in that pocket (68.5% W, 61 to 69.5% H); speaker list
  below it (starts 54% W, 76% H, each line 2.5% W further right); fine print at the main
  axis (25.5% W, 86 to 92% H). Bottom-left and top-right corners stay empty.

## 3. Layer stack

1. Ground: warm off-white `#eeeaec`, full canvas.
2. Title, five rows, near-black `#0a0a0a`. Row 1: hanging letter + 2 letters. Row 2:
   3 letters. Row 3: hanging letter + 3 letters. Row 4: 3 letters. Row 5: 2 letters.
3. Cuts, in the ground color, all running from upper left to lower right at 45 degrees:
   * four corner wedges: right triangles with 7% W legs that remove the lower-left
     corner of a letter (first letter of row 2, second letter of row 3, last letter of
     row 4, last letter of row 5);
   * three seams: bands 2 to 3% W wide and 12 to 20% W long that cross a letter completely
     (the hanging letter of row 3, the first letter of row 3, the middle letter of row 4).
   No cut touches row 1, so the first row stays whole and readable.
4. Sub-title: 2 lines, bold, black, at 68.5% W, cap top 61.5% H; second line indented.
5. Date and time: 2 lines, regular, dark gray `#55505a`, at 73.3% W, 65.8% H; second
   line indented.
6. Speaker list: 8 names, medium weight, dark gray, first at 54% W, 76.2% H; line pitch
   2.1% H; every line starts 2.5% W right of the one above (a 50-degree staircase).
7. Fine print: 3 bold lines (venue), a blank line, 2 bold lines (booking contact), black,
   at 25.5% W, 86 to 92% H.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#eeeaec` | paper ground and the cuts | 68% |
| `#0a0a0a` | title, sub-title, fine print | 30% |
| `#55505a` | date, time, speaker names | 2% |

Black on off-white, with one gray for the second level of small text. No color at all:
adding one would compete with the cuts.

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Title | 2 words, 7 + 8 letters, broken 3 / 3 / 1+3 / 3 / 2 with no hyphens | caps | heavy wide grotesque (Archivo Black, Inter Black / Arial Black) | 900 | 13.5% | -50 | row pitch 14.5% H | left, two axes | `#0a0a0a` | 25% (hanging letters end at 23.5%), first cap top 3.5% |
| Sub-title | 2 lines, 3 + 1 words | title | neo-grotesque (Inter / Helvetica Neue) | 700 | 1.35% | 0 | 112% | left, stepped | `#0a0a0a` | 68.5%, top 61.5% |
| Date, time | date line, time-range line | sentence | same | 400 | 1.35% | 0 | 112% | left, stepped | `#55505a` | 73.3%, top 65.8% |
| Speakers | 8 names, 2 words each | title | same | 500 | 1.5% | 0 | 102% | left, stepped 2.5% W per line | `#55505a` | 54%, top 76.2% |
| Fine print | 3 lines venue, blank line, 2 lines contact | sentence | same | 700 | 0.75% | 0 | 112% | left | `#0a0a0a` | 25.5%, top 86% |

Two faces from one grotesque family: the black weight for the title, the text weights for
everything else. The jump from title to small text is 9 to 1, with nothing in between.

## 6. Imagery

No image. The title is the picture: black letter shapes about 1 cap height wide each,
packed so tightly that the block reads as one dark mass with a stepped silhouette. The
cuts turn letters into abstract shapes; the reader reconstructs them, which is why the
poster holds attention longer than plain type would.

## 7. Effects and textures

* Sliced type (no library ID): shapes in the ground color laid over live text. A wedge is
  a `shape path` triangle `M x y L x y+75 L x+75 y+75 Z` placed on a letter's lower-left
  corner; a seam is a `shape line` with `--stroke` in the ground color, 22 to 30 px wide,
  drawn at exactly 45 degrees (equal x and y distance).
* Row setting follows **T-fit-width** practice: each row is its own text layer, placed by
  the top-left corner of its visible letters (`--at X,Y --anchor-to ink`); hanging letters
  are placed by their top-right corner (`--anchor tr --anchor-to ink`).
* Stepped lists: one multi-line text layer; each line begins with three more spaces than
  the line above (three spaces are about 2.5% W at this size).
* **T-rules**: none drawn; the axes are implied by the letter edges. The original is a
  flat print; **T-paper-grain** at amount 8 is optional.
* **T-sliced-type** in [techniques.md](../techniques.md) is the tested library version of the cut-and-shifted headline; start from it when reusing this idea.

## 8. Hierarchy and eye path

1. Row 1 of the title, top left, uncut: the entry point.
2. Down the main axis through rows 2 to 5; the cuts slow the reading on purpose.
3. Off the end of the short last row into the bold sub-title, then date and time.
4. Down the staircase of names to the bottom right.
5. Fine print, back at the main axis.

Cap-height ratio: title 13.5 : names 1.5 : sub-title 1.35 : fine print 0.75, about
18 : 2 : 1.8 : 1.

## 9. Build recipe

Needs no images. Cut coordinates belong to this title: after changing the words, read
each row's `bounds` (add `--json`) and move the wedges to letter corners. The cuts are
shapes laid over live text, so the letters stay editable.

```bash
pixelox new -o talk.pxo --size 1080x1528 --fill '#eeeaec'
pixelox text add talk.pxo --in-place --name Hang1 --content "P" --font @heavy-wide --size 18.6% --color '#0a0a0a' --at 23.5%,3.5% --anchor tr --anchor-to ink
pixelox text add talk.pxo --in-place --name Row1 --content "RI" --font @heavy-wide --size 18.6% --tracking -50 --color '#0a0a0a' --at 25%,3.5% --anchor-to ink
pixelox text add talk.pxo --in-place --name Row2 --content "NTE" --font @heavy-wide --size 18.6% --tracking -50 --color '#0a0a0a' --at 25%,18% --anchor-to ink
pixelox text add talk.pxo --in-place --name Hang3 --content "D" --font @heavy-wide --size 18.6% --color '#0a0a0a' --at 23.5%,32.5% --anchor tr --anchor-to ink
pixelox text add talk.pxo --in-place --name Row3 --content "SEN" --font @heavy-wide --size 18.6% --tracking -50 --color '#0a0a0a' --at 25%,32.5% --anchor-to ink
pixelox text add talk.pxo --in-place --name Row4 --content "TEN" --font @heavy-wide --size 18.6% --tracking -50 --color '#0a0a0a' --at 25%,47% --anchor-to ink
pixelox text add talk.pxo --in-place --name Row5 --content "CE" --font @heavy-wide --size 18.6% --tracking -50 --color '#0a0a0a' --at 25%,61.5% --anchor-to ink
pixelox shape path talk.pxo --in-place --name Wedge1 --d 'M 262 412 L 262 487 L 337 487 Z' --fill '#eeeaec'
pixelox shape line talk.pxo --in-place --name Seam1 --from 50,560 --to 200,710 --stroke '#eeeaec' --stroke-width 26
pixelox shape line talk.pxo --in-place --name Seam2 --from 340,486 --to 470,616 --stroke '#eeeaec' --stroke-width 30
pixelox shape path talk.pxo --in-place --name Wedge2 --d 'M 462 636 L 462 712 L 538 712 Z' --fill '#eeeaec'
pixelox shape line talk.pxo --in-place --name Seam3 --from 505,712 --to 665,872 --stroke '#eeeaec' --stroke-width 22
pixelox shape path talk.pxo --in-place --name Wedge3 --d 'M 650 856 L 650 928 L 722 928 Z' --fill '#eeeaec'
pixelox shape path talk.pxo --in-place --name Wedge4 --d 'M 478 1082 L 478 1156 L 552 1156 Z' --fill '#eeeaec'
pixelox text add talk.pxo --in-place --name Sub --content $'A Series of Open\n   Conversations' --font @neo-grotesque --weight 700 --size 1.9% --leading 112% --color '#0a0a0a' --at 68.5%,61% --anchor tl
pixelox text add talk.pxo --in-place --name When --content $'12 March 2031\n   10:00 - 18:00' --font @neo-grotesque --weight 400 --size 1.9% --leading 112% --color '#55505a' --at 73.3%,65.3% --anchor tl
pixelox text add talk.pxo --in-place --name Names --content $'First Speaker\n   Second Name\n      Third Person\n         Fourth Guest\n            Fifth Speaker\n               Sixth Name\n                  Seventh Person\n                     Eighth Guest' --font @neo-grotesque --weight 500 --size 2.05% --leading 102% --color '#55505a' --at 54%,75.6% --anchor tl
pixelox text add talk.pxo --in-place --name Venue --content $'Main Lecture Hall\nYour College of\nArt and Design\n\nTo book, email:\nhello@example.com' --font @neo-grotesque --weight 700 --size 1% --leading 112% --color '#0a0a0a' --at 25.5%,85.6% --anchor tl
pixelox export talk.pxo -o talk.png
```

Change first: the title. Pick words whose rows come out 2 to 4 letters long and whose
last row is the shortest (two narrow letters), so the sub-title pocket exists. Then move
the cuts. With a narrower heavy face (Archivo Black, Inter Black) raise `--size` to 19.4%
for the original's 14% H cap height.

## 10. Variations and pairings

* Inverted: ground `#0a0a0a`, type `#eeeaec`, cuts black, names `#9a959c`.
* One accent: keep everything black and make the seams (not the wedges) a color, such as
  `#ff3b1f` or `#1437e6`; they then read as tape or laser lines instead of folds.
* Cut direction: mirror all cuts (upper right to lower left) and step the name list to
  the left for a right-hung version with the main axis at 75% W.
* Works for talks, degree shows, architecture and type events, book fairs, festival
  line-ups (the staircase takes 6 to 12 names). Square format: three rows of 4 letters.
* Pairs with T-vertical-type (a rotated date on the right edge), T-image-in-type (a photo
  inside row 3 only), and T-distress-type at low strength for a printed-ink feel.

## 11. Pitfalls

* Cutting every letter, or the first row: the word stops being readable. Leave row 1 and
  at least half of all letters whole, and never cut two neighbors in the same way.
* Cuts at mixed angles look like damage. Every cut is 45 degrees in the same direction,
  and the stepped text follows that direction too.
* Wide line gaps: the block falls apart into five words. Keep the gap under 8% of the
  cap height; rows may almost touch.
* A regular or bold weight instead of a black one: thin stems vanish under a seam. The
  stem must be at least a quarter of the cap height, wider than any seam.
* Centering the small text under the block. It belongs in the pocket at the end of the
  last row and on the main axis, never on the canvas center.
* Hyphens at the row breaks. Break without them; the tight stack makes the reading order
  obvious.
