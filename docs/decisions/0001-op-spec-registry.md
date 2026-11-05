# ADR 0001: One op-spec registry generates the CLI, schemas, MCP tools, and docs

Status: accepted (2026-11-04)

## Context
PRD §5 requires that agents and scripts are first-class users and that schemas, docs, and
help never drift. The sibling project ferraus solved this with a single op-spec registry.

## Decision
Every command is an `OpSpec` (`crates/pixelox-core/src/op.rs`): name, category, summary,
typed parameters, inputs, positionals, examples, and a runner (`doc`, `source`, or
`custom`). The clap tree (`pixelox-cli/src/cli.rs`), JSON Schemas (`core/schema.rs`),
pipeline steps, MCP tools, `describe --agent`, reference pages, and the docs site are all
generated from it. Crates contribute commands through `ops()`; the CLI never hard-codes one.

## Consequences
* Parameter values are strings until an op reads them through the shared parser, so
  `50%`, `-48,-48`, `#0ea5e9`, and `'0,0 64,80'` mean the same on the CLI, in YAML, and in
  MCP JSON.
* Contract tests snapshot every schema and parse every documented example against the
  real CLI (`crates/pixelox-cli/tests/contract.rs`).
