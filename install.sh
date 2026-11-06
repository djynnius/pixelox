#!/bin/sh
# pixelox installer shim (Linux and macOS).
#
#   curl -fsSL https://raw.githubusercontent.com/djynnius/pixelox/main/install.sh | sh
#
# Resolves the latest (or $PIXELOX_VERSION) GitHub release and runs the installer that the
# release workflow generated and attached to it. That installer picks the build for this
# machine (glibc or static musl on Linux), verifies the archive's SHA-256, installs to
# ~/.pixelox/bin, adds it to PATH, and never asks for sudo.
#
# Options (environment variables on the `sh` side of the pipe):
#   PIXELOX_VERSION=0.2.0       install that release instead of the latest
#   PIXELOX_INSTALL_DIR=DIR     install into DIR instead of ~/.pixelox/bin
#   PIXELOX_NO_MODIFY_PATH=1    leave shell profiles and PATH alone
# e.g.  curl -fsSL https://raw.githubusercontent.com/djynnius/pixelox/main/install.sh | PIXELOX_NO_MODIFY_PATH=1 sh
set -eu

repo="djynnius/pixelox"
if [ -n "${PIXELOX_VERSION:-}" ]; then
  url="https://github.com/${repo}/releases/download/v${PIXELOX_VERSION#v}/pixelox-installer.sh"
else
  url="https://github.com/${repo}/releases/latest/download/pixelox-installer.sh"
fi

if command -v curl >/dev/null 2>&1; then
  fetch() { curl --proto '=https' --tlsv1.2 -fsSL "$1"; }
elif command -v wget >/dev/null 2>&1; then
  fetch() { wget --https-only -qO- "$1"; }
else
  echo "pixelox: need curl or wget to download the installer" >&2
  exit 1
fi

tmp="$(mktemp)"
trap 'rm -f "$tmp"' EXIT INT TERM
if ! fetch "$url" > "$tmp"; then
  echo "pixelox: could not download $url" >&2
  exit 1
fi
sh "$tmp" "$@"
