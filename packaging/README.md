# Packaging

| Channel | Status |
| --- | --- |
| `cargo install --path crates/pixelox-cli` | works from a checkout |
| Docker (`packaging/docker/Dockerfile`) | builds a slim image; uses the embedded font only |
| Homebrew, scoop, MSI, apt/dnf, curl installer, static musl tarball (PRD §11.9) | not set up: they need a release repository, signing keys, and CI release jobs |

Releases should come only from CI, as in ferraus: tagged builds per target, checksummed and
signed, with `self-update` enabled only in builds from that channel.
