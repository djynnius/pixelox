# ADR 0007: ICC profiles are kept and re-embedded, not converted

Status: accepted (2026-11-04)

## Context
PRD §7.4 converts tagged inputs through lcms2.

## Decision
No color-management engine is linked. On open, an embedded profile is kept with the
document (`--icc honor`, the default, warns PXW0101 when it is not sRGB; `assign` keeps it
silently; `ignore` drops it) and is embedded again on export, so pixel values and their
meaning travel together. `export --profile` other than the document's own fails with
PX1520 (exit 12).

## Consequences
Wide-gamut and print-profile sources round-trip correctly but cannot be converted to sRGB
yet. A pure-Rust CMS (PRD Q5) can be added behind the same flags.
