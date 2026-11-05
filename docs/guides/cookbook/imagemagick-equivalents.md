# ImageMagick equivalents

| ImageMagick | pixelox |
| --- | --- |
| `convert in.png out.jpg` | `pixelox export in.png -o out.jpg` |
| `convert in.jpg -resize 50% out.jpg` | `pixelox transform resize in.jpg -o out.jpg --size 50%` |
| `convert in.jpg -resize 1600x out.jpg` | `pixelox transform resize in.jpg -o out.jpg --size 1600x` |
| `convert in.jpg -crop 400x300+10+20 out.jpg` | `pixelox transform crop in.jpg -o out.jpg --rect 10,20,400x300` |
| `convert in.jpg -rotate 90 out.jpg` | `pixelox transform rotate in.jpg -o out.jpg --degrees 90` |
| `convert in.jpg -blur 0x3 out.jpg` | `pixelox filter gaussian in.jpg -o out.jpg --sigma 3` |
| `convert in.jpg -unsharp 0x1 out.jpg` | `pixelox filter unsharp in.jpg -o out.jpg --amount 100 --radius 1` |
| `convert in.jpg -colorspace Gray out.jpg` | `pixelox adjust bw in.jpg -o out.jpg` |
| `convert in.jpg -negate out.jpg` | `pixelox adjust invert in.jpg -o out.jpg` |
| `convert in.jpg -trim out.png` | `pixelox transform trim in.jpg -o out.png` |
| `composite -gravity southeast logo.png in.jpg out.jpg` | `pixelox comp place in.jpg logo.png -o out.jpg --anchor br --at -20,-20` |
| `identify -verbose in.jpg` | `pixelox info in.jpg --json` and `pixelox analyze exif in.jpg --json` |
| `compare -metric SSIM a.png b.png diff.png` | `pixelox analyze diff a.png b.png --out diff.png --json` |

The differences that matter: pixelox keeps layers and masks in a document, never
overwrites without `--overwrite`, reports results as JSON, and records how every output
was made.
