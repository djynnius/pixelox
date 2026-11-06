#!/usr/bin/env bash
# Renders this case's portrait with pixelox (no image is committed): see ../face.js.
# Pimples sit at about (470,820) (720,760) (560,960) (690,900) (415,720); the beauty mark
# to keep is at (760,1000).
set -eu
here="$(cd "$(dirname "$0")" && pwd)"
pixelox render js "$here/../face.js" -o portrait.png --param tone=medium --seed 3 --overwrite
