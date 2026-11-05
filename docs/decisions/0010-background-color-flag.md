# ADR 0010: Background colors are `--background-color`

Status: accepted (2026-11-04)

## Context
PRD §8.1 makes `--background` a global flag (run the command as a background job), while
§8.15 and examples E10 and E12 give `svg rasterize` a `--background C` color. One flag
cannot mean both.

## Decision
The global keeps `--background`. Color flags are named `--background-color`
(`svg rasterize`, `export`), as rsvg-convert names it. A contract test rejects any command
parameter named like a global flag.

## Consequences
E10 and E12 read `--background-color '#ffffff'` in this build; the contract test lists them
as known PRD divergences.
