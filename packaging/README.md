# Packaging

| Channel | Status |
| --- | --- |
| `curl … install.sh \| sh` (macOS, Linux) | the root `install.sh` runs the `pixelox-installer.sh` attached to the latest GitHub release |
| `irm … install.ps1 \| iex` (Windows) | the root `install.ps1` runs the release's `pixelox-installer.ps1` |
| GitHub releases | `.github/workflows/release.yml` (cargo-dist 0.32, config in `dist-workspace.toml`): on a `v*` tag it builds 8 targets, attests them, and publishes archives, checksums, and both installers |
| `cargo install --path crates/pixelox-cli` | works from a checkout |
| Docker (`packaging/docker/Dockerfile`) | builds a slim image; uses the embedded font only |
| Homebrew, scoop, MSI, apt/dnf (PRD §11.9) | not set up yet; ferraus's `dist-workspace.toml` shows the Homebrew and MSI settings to add |

To release: bump `version` in the root `Cargo.toml`, add the version's section to
`CHANGELOG.md`, merge, then push the tag (`git tag v0.2.0 && git push origin v0.2.0`). The
release workflow does the rest. The repository must be public for the installers to
download release assets without a token.

ONNX Runtime (local models) is linked into the glibc Linux, Apple-silicon macOS, and Windows
builds from `ort`'s prebuilt libraries; Intel macOS and musl builds compile without it
(`crates/pixelox-ml/build.rs`). Windows builds use the DLL C runtime
(`msvc-crt-static = false`), as ONNX Runtime requires.
