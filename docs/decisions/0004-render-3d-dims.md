# ADR 0004: `render 3d` takes primitive dimensions as `--dims`

Status: accepted (2026-11-04)

## Context
PRD §8.16 writes `render 3d --primitive box --size 2,1,1`, while `--size WxH` means an
output size in every other command.

## Decision
Primitive dimensions are `--dims X,Y,Z`; `--size WxH` is the output size. The contract
test lists this as a known PRD divergence.
