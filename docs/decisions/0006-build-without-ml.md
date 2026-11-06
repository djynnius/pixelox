# ADR 0006: This build ships without the `ml` feature

Status: accepted (2026-11-04); superseded for `detect subject` and `detect salient` by ADR 0012

## Context
PRD §7.8 runs local ONNX models (BiRefNet-lite, MobileSAM, ViTMatte, LaMa, YOLOX-s, YuNet,
PP-OCR) through ONNX Runtime, downloaded on consent with pinned SHA-256 hashes. That needs
the ORT library and verified hashes for published weights; the PRD schedules it for v0.2
and already defines a build variant without `ml` (the static musl build).

## Decision
This build is the no-`ml` variant. `detect *`, `mask refine`, `fill content-aware`, and the
`models` download commands are registered with their full flags and schemas. `--dry-run`
returns the plan (model, license, size, consent needed) and a real run fails with PX1510,
exit 12, naming classical alternatives. `models list` shows the registry.

## Consequences
Agents can plan ML workflows and discover the gap through the contract instead of a
missing command. Background removal on plain backdrops works with `select color
--contiguous`; small-region repair with `retouch heal`.
