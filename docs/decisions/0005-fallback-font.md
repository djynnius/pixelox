# ADR 0005: Tuffy is the embedded fallback font

Status: accepted (2026-11-04)

## Context
PRD §7.7 embeds a Noto Sans subset (OFL-1.1) so text renders identically on bare CI hosts.

## Decision
The embedded fallback is Tuffy Regular (public domain, 1,502 glyphs: Latin, Greek,
Cyrillic), from `crates/pixelox-text/assets/`. Generic family names resolve to a system
font first; `PIXELOX_SYSTEM_FONTS=0` skips system fonts so output is the same on every
machine.

## Consequences
Scripts outside Tuffy's coverage (Arabic, Hebrew, CJK) need a system or registered font
(`pixelox fonts add DIR`). Swapping in Noto Sans later only changes the asset.
