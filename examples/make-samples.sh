#!/usr/bin/env bash
# Build the test material every example uses, with pixelox itself (nothing binary is
# checked in):  make-samples.sh [DIR]
set -euo pipefail
PIXELOX=${PIXELOX:-pixelox}
DIR=${1:-samples}
mkdir -p "$DIR"
cd "$DIR"
p() { "$PIXELOX" "$@" -q --no-sidecar --overwrite; }

# photo.png: a "landscape" — sky gradient, sun, hills, a little grain
p new -o photo.pxo --size 960x640 --fill '#87a7c7'
p fill gradient photo.pxo --in-place --kind linear --from 0,0 --to 0,640 --stops '#3b6ea5@0,#e8c39e@0.7,#5b6b3a@1'
p shape ellipse photo.pxo --in-place --rect 650,120,140x140 --fill '#fff3c4' --name Sun
p shape path photo.pxo --in-place --d 'M 0 520 C 200 420 380 470 520 500 C 700 540 820 450 960 480 L 960 640 L 0 640 Z' --fill '#2f4a2a' --name Hills
p layer flatten photo.pxo --in-place
p filter add-noise photo.pxo --in-place --amount 4 --monochrome
p adjust brightness-contrast photo.pxo --in-place --brightness -25 --contrast -10
p export photo.pxo -o photo.png
p export photo.pxo -o photo.jpg --quality 90

# product.png: an object on a plain white backdrop
p new -o product.pxo --size 800x800 --fill white
p shape rect product.pxo --in-place --rect 250,180,300x440 --radius 40 --fill '#c2410c' --name Body
p shape ellipse product.pxo --in-place --rect 300,120,200x120 --fill '#7c2d12' --name Cap
p layer flatten product.pxo --in-place
p export product.pxo -o product.png

# logo.svg: a small vector logo
p svg new -o logo.svg --size 200x200
p svg el add logo.svg --in-place --kind circle --id ring --attr cx=100 --attr cy=100 --attr r=80 --attr fill=none --attr stroke=#0b5fff --attr stroke-width=18
p svg el add logo.svg --in-place --kind rect --id bar --attr x=60 --attr y=88 --attr width=80 --attr height=24 --attr fill=#0b5fff

# scan.png: a black-on-white mark for tracing
p svg rasterize logo.svg -o scan-color.png --size 400x400 --background-color white
p adjust threshold scan-color.png -o scan.png --level 160

# frames for an animation
mkdir -p frames
for i in 1 2 3 4; do
  p new -o "frames/f$i.pxo" --size 120x120 --fill '#111827'
  p shape ellipse "frames/f$i.pxo" --in-place --rect "$((i * 20)),40,40x40" --fill '#facc15'
  p export "frames/f$i.pxo" -o "frames/f$i.png"
  rm -f "frames/f$i.pxo"
done

# speakers.csv for the social-card example
cat > speakers.csv <<'CSV'
slug,name,title
ada,Ada Lovelace,Analyst
grace,Grace Hopper,Rear Admiral
CSV
rm -f ./*.pixelox.json frames/*.pixelox.json
echo "samples in $(pwd)"
