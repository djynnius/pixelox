---
id: p089
title: Red-frame news cover, annual list issue
genre: [magazine-cover, editorial]
mood: [serious, calm, futuristic]
style: [editorial, photo-composite]
format: portrait-3x4
archetype: A-cover-masthead
palette: ['#f22929', '#bfbcb1', '#a8daea', '#7c7862', '#11120e', '#ffffff']
techniques: [T-frame-border, T-text-behind-subject, T-photo-fade, T-rules]
needs: [photo, subject-cutout]
---

# p089: Red-frame news cover, annual list issue

## 1. Concept

A news-magazine cover for a yearly "best of" issue: one documentary photograph of a
humanoid machine doing housework fills a thick red frame, a ghosted serif masthead sits
behind the subject's head, a serif theme line runs across the top, and one small centered
story block sits on the calm lower-right part of the picture. It works because the frame
and the masthead are the brand and need no emphasis, so every piece of type can stay
small and white and the photograph does all the talking.

## 2. Format and grid

* Portrait 3:4. A red frame of equal pixel width on all four sides: 4.4% W (48 px at 1080
  wide, which is 3.3% H). Inside it a white keyline 1.3% W wide, then the picture window
  `5.7%,4.3%,88.6% x 91.4%`.
* Two axes inside the window. The masthead and the subject's head share the center axis
  (50% W). The story block is centered on its own axis at 69% W, in the lower-right
  quadrant, where the photo is dark and flat.
* Zones: theme line 6 to 11% H; masthead 12.8 to 27.8% H; picture subject 15 to 95% H;
  story block 70 to 93.5% H. The left half of the bottom third stays free of type.
* Two fine-print lines live inside the red frame itself: top-left and bottom-right.

## 3. Layer stack

1. Photo, full window: a bedroom interior with a window and landscape at the upper left,
   a warm beige headboard at the upper right, and a dark olive knitted blanket across the
   lower right.
2. Masthead: 4 serif capitals, white at 60% opacity, x 16 to 84% W, cap top at 12.8% H,
   cap height 15% H. It reads as a watermark in the picture, not as a label on it.
3. Subject (same photo, cut out and placed again in register): a white humanoid machine
   with a glossy black visor, leaning from the upper center down to the left, arms
   reaching right to fold clothes. Its head (43 to 58% W, 15.5 to 31% H) covers the middle
   of the masthead; the first and last letters stay clear.
4. Shade: the photo's own dark blanket in the original; in a build, a gradient from
   transparent at 62% H to 70% near-black at 96% H.
5. Theme line: white mixed-case serif, baseline 11% H, x 8.5 to 73% W, cap height 5.3% H;
   then a small italic "of" and a year, cap height 3.2% H, ending at 91.5% W on the same
   baseline. It overlaps nothing but sits only 1.8% H above the masthead.
6. Story block, centered on 69% W: two-line bold serif title (cap tops 70% and 73.5% H),
   two-line sans deck (77.6% and 79.6% H), a yellow dash 2% W long at 82% H, a four-line
   sans list (84.5 to 90.3% H), and one yellow italic line at 92.5% H.
7. Red frame `#f22929` and white keyline, above everything, so they crop the photo and
   the subject cleanly.
8. Fine print in the frame: issue date at 4.5% W, 2.3% H; web address ending at 94% W,
   97.7% H. Near-black on red, cap height 0.6% H.

## 4. Palette

| Hex | Role | Area |
| --- | --- | --- |
| `#f22929` | frame | 15% |
| `#bfbcb1` | photo: warm gray walls, machine body | 30% |
| `#a8daea` | photo: window sky | 10% |
| `#7c7862` | photo: olive blanket, the ground for the story block | 25% |
| `#11120e` | photo: visor, joints, window bars, deep shadow | 15% |
| `#ffffff` | all type, keyline | 5% |
| `#e8c62a` | dash and the last list line | under 1% |

A neutral, daylight photograph inside one saturated red. Red appears nowhere inside the
picture, which is why the frame reads as a frame. Yellow is the only accent and is used
twice, both times as "there is more".

## 5. Text elements

| Role | Content pattern | Case | Font class (look-alike / local) | Weight | Cap height | Tracking | Leading | Align | Color | Position |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Masthead | `[MASTHEAD]`, 4 letters | caps | sharp transitional serif, display cut (Playfair Display, Libre Caslon Display / Times New Roman) | 700 | 15% | 20 | - | center | `#ffffff` at 60% | 50%, cap top 12.8% |
| Theme line | 2-word list title, 4 + 10 letters | title | same serif, text cut (Libre Caslon Text, Lora / Times New Roman) | 400 | 5.3% | -10 | - | left | `#ffffff` | 8.5%, baseline 11% |
| Theme suffix | "of" + year | italic "of", figures | same | 400 | 3.2% | 0 | - | right | `#ffffff` | ends 91.5%, baseline 11% |
| Story title | 2 lines, 1 + 2 words, 6 to 9 letters each | caps | same serif | 700 | 2.7% | -10 | 88% | center | `#ffffff` | 69%, cap tops 70% and 73.5% |
| Deck | 2 lines, 5 to 6 words each | sentence | neo-grotesque (Inter, Roboto / Helvetica Neue) | 400 | 1.3% | 0 | 112% | center | `#ffffff` | 69%, 77.6% and 79.6% |
| List | 4 lines, each "A" or "The" + 3 to 5 words | sentence | same sans | 400 | 1.3% | 0 | 108% | center | `#ffffff` | 69%, 84.5%, 86.4%, 88.3%, 90.3% |
| More line | "& NN more" | lower | same sans, italic | 400 | 1.2% | 0 | - | center | `#e8c62a` | 69%, 92.5% |
| Issue date | month, day, year | caps | same sans | 700 | 0.6% | 120 | - | left | `#1a0505` | 4.5%, 2.3% |
| Web address | one short address | lower | same sans | 700 | 0.6% | 120 | - | right | `#1a0505` | 94%, 97.7% |

Two families: one serif in three roles (ghost masthead, light theme line, bold story
title) and one plain sans for everything that explains. No type is larger than the theme
line except the masthead, and the masthead is half transparent.

## 6. Imagery

One editorial photograph, shot at eye level in soft window light: a human-sized machine
(white shell, black visor, exposed dark joints) bends over a bed and folds laundry. The
figure enters from the left and its arms lead the eye to the right, to the pile of clothes
and on down to the story block. Natural color, no grade, no vignette. What matters for a
build: the subject's head must be in the upper middle (under the masthead), the picture
needs a calm dark area at least 35% W by 25% H in one lower corner for the story block, and
the subject should be doing something, not posing.

The head-over-masthead overlap is **T-text-behind-subject**: the subject is cut out of the
same photo (see "Getting a cutout" in techniques.md) and placed again, in register, above
the masthead layer.

## 7. Effects and textures

* **T-frame-border**: a red frame 48 px wide drawn as an even-odd path over the finished
  picture, plus a 14 px white keyline just inside it.
* **T-text-behind-subject**: masthead at 60% opacity between the photo and the cutout.
* **T-photo-fade** (as a shade, not a full fade): ground `#14120c`, alpha 0 at 62% H to
  70% at 96% H, only when the photo has no dark corner of its own.
* **T-rules**: one yellow dash separates the deck from the list.
* No grain, no shadows on type, no gradients: the cover is flat and photographic.

## 8. Hierarchy and eye path

1. The red frame and the black visor (strongest color, strongest contrast).
2. The theme line across the top, read as one sentence with the masthead behind it.
3. Down the subject's arms to the hands, then to the story title in the lower right.
4. Deck, list, and the yellow "more" line.

Cap heights: masthead 15 : theme line 5.3 : story title 2.7 : deck and list 1.3 : fine
print 0.6, about 11.5 : 4 : 2 : 1 : 0.5. The masthead is the largest thing and the least
important to read, so it is the faintest.

## 9. Build recipe

`photo.png` (the scene) and `subject.png` (its subject, cut out) are your images here.
With one photograph, export the cutout from that same photo and place it at the photo's
own position and size, so only the masthead ends up between them.

```bash
pixelox new -o cover.pxo --size 1080x1440 --fill '#f22929'
pixelox comp place cover.pxo photo.png --in-place --as Photo --fit cover
pixelox text add cover.pxo --in-place --name Masthead --content "SPAN" --font "Times New Roman" --weight 700 --fit-width 734 --tracking 20 --color '#ffffff' --at 50%,12.8% --anchor t --anchor-to ink
pixelox layer opacity cover.pxo --in-place --layer Masthead --value 62
pixelox comp place cover.pxo subject.png --in-place --as Subject --size x1210 --at 44%,97% --anchor b
pixelox layer add cover.pxo --in-place --name Shade
pixelox fill gradient cover.pxo --in-place --layer Shade --from 0,62% --to 0,96% --stops '#14120c00@0,#14120cb3@1'
pixelox text add cover.pxo --in-place --name Theme --content "New Machines" --font "Times New Roman" --weight 400 --fit-width 664 --tracking -10 --color '#ffffff' --at 8.5%,11% --anchor bl --anchor-to ink
pixelox text add cover.pxo --in-place --name ThemeOf --content "of" --font "Times New Roman" --weight 400 --style italic --size 4.6% --color '#ffffff' --at 72%,11.4% --anchor bl
pixelox text add cover.pxo --in-place --name ThemeYear --content "2031" --font "Times New Roman" --weight 400 --size 4.6% --color '#ffffff' --at 91.5%,11.4% --anchor br
pixelox text add cover.pxo --in-place --name Title --content $'HELPERS\nMOVE IN' --font "Times New Roman" --weight 700 --size 4.1% --leading 88% --tracking -10 --align center --color '#ffffff' --at 69%,73.2% --anchor c
pixelox text add cover.pxo --in-place --name Deck --content $'One young lab wants to change\nwho does the chores at home' --font @neo-grotesque --weight 400 --size 1.8% --leading 112% --align center --color '#ffffff' --at 69%,78.6% --anchor c
pixelox shape line cover.pxo --in-place --name Dash --from 68%,82% --to 70%,82% --stroke '#e8c62a' --stroke-width 3
pixelox text add cover.pxo --in-place --name List --content $'A fold-flat city bike\nThe first paper battery\nA rooftop rain harvester\nA pocket test for clean water' --font @neo-grotesque --weight 400 --size 1.8% --leading 108% --align center --color '#ffffff' --at 69%,87.4% --anchor c
pixelox text add cover.pxo --in-place --name More --content "& 40 more" --font @neo-grotesque --weight 400 --style italic --size 1.7% --color '#e8c62a' --at 69%,92.6% --anchor c
pixelox shape path cover.pxo --in-place --name Frame --d 'M 0 0 H 1080 V 1440 H 0 Z M 48 48 H 1032 V 1392 H 48 Z' --fill-rule evenodd --fill '#f22929'
pixelox shape rect cover.pxo --in-place --name Keyline --rect 55,55,970x1330 --fill none --stroke '#ffffff' --stroke-width 14
pixelox text add cover.pxo --in-place --name IssueDate --content "OCT. 14, 2031" --font @neo-grotesque --weight 700 --size 0.85% --tracking 120 --color '#1a0505' --at 4.5%,2.3% --anchor l
pixelox text add cover.pxo --in-place --name Site --content "example.com" --font @neo-grotesque --weight 700 --size 0.85% --tracking 120 --color '#1a0505' --at 94%,97.8% --anchor r
pixelox export cover.pxo -o cover.png
```

Change first: the masthead word (4 or 5 wide capitals; keep `--fit-width 734` so the cap
height lands near 15% H), then the subject's size and position until its head covers one
or two middle letters, then the theme line (`--fit-width 664` makes a longer title
smaller on its own). Drop the Shade layer if your photo is already dark where the story
block sits.

## 10. Variations and pairings

* Same frame, other covers: [p099](p099-stencil-portrait-news-cover.md) fills the window
  with a four-tone stencil portrait and puts an opaque masthead in front;
  [p123](p123-full-figure-list-cover.md) uses a pale backdrop, a gray tone-on-tone
  masthead, and a tall condensed list title at the left edge.
* Frame color carries the brand: `#f22929` news red, `#111111` for a memorial or serious
  issue, `#f5c518` for a business list, `#0b5fff` for a tech annual. Keep the keyline white.
* Awards and annual reports: replace the theme line with the award name, the list with
  four winners, and the yellow line with "& NN more".
* Opaque masthead (100% white or the frame color) when the photo is busy behind it; ghost
  it (50 to 65%) only over sky, wall, or backdrop.
* Square social version: keep the frame, the theme line, and the story title; drop the
  list and move the block to 50% W under the subject.
* Pairs with T-color-grade on the photo for a colder, more technical feel, and with
  T-final-grade when the photo and the cutout come from different sources.

## 11. Pitfalls

* The subject hides the masthead. Cover at most two middle letters of four; if the head is
  wider, shrink the subject or widen the masthead to 80% W.
* A white, fully opaque masthead plus a white theme line right above it: two headlines
  fighting. One of them must recede; here the masthead is ghosted to 60%.
* Story block on a bright or busy part of the photo. Put it where the picture is darker
  than 35% lightness, or add the Shade layer; never add a box behind it on this kind of
  cover.
* Frame thicker at the top than at the sides. Use one pixel inset for all four sides
  (48 px at 1080 wide), not one percentage for width and height.
* Centering the story block on the canvas. It is centered on its own axis (69% W) inside
  the calm quadrant; on the canvas axis it would sit on the subject.
