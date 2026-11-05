# ADR 0009: Layer styles are composited into the layer before blending

Status: accepted (2026-11-04)

## Context
In Photoshop each effect beneath a layer (drop shadow, outer glow) blends onto the backdrop
in its own mode.

## Decision
The fx hook receives a layer's content and returns it with every effect composited, so
effects beneath the content composite onto the backdrop with the layer's blend mode
(`normal` by default) rather than their own. Black shadows look the same; colored glows in
`screen` mode are lighter in Photoshop.

## Consequences
A backdrop-aware hook would remove the difference; the fx parameters already record each
effect's blend mode, so the file format does not change.
