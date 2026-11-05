# HOWTO

Short answers for people driving pixelox by hand. Agents: see [AGENTS.md](AGENTS.md).

**Resize a photo for the web.**
`pixelox transform resize photo.jpg -o web.jpg --size 1600x`

**Convert formats.** `pixelox export photo.png -o photo.webp` (the extension picks the format).

**Make a transparent cutout of a product on white.**
`pixelox open p.jpg -o p.pxo`, then
`pixelox select color p.pxo --in-place --ref white --at 4,4 --contiguous --tolerance 40`,
`pixelox select invert p.pxo --in-place`, and
`pixelox export p.pxo -o cut.png --apply-mask @sel`.

**Blur everything except a region.** Save the region as a mask
(`pixelox select ellipse doc.pxo --in-place --rect 100,80,300x300` then
`pixelox select save doc.pxo --in-place --as subject`), then
`pixelox filter gaussian doc.pxo --in-place --sigma 8 --mask subject --invert-mask`.

**Add a caption.**
`pixelox text add doc.pxo --in-place --content "Lagos, 2026" --font sans-serif --size 36pt --color white --at 50%,-40 --anchor b`

**Undo.** `pixelox history undo doc.pxo --in-place` (it rebuilds the document from its recipe).

**See what a command takes.** `pixelox docs filter.gaussian` or `pixelox filter gaussian --help`.

**Replace an existing output.** Add `--overwrite`; pixelox never replaces files otherwise.

**Find out how a file was made.** Read `file.png.pixelox.json`, or rebuild it with
`pixelox replay file.png.pixelox.json -o copy.png`.
