# ADR 0008: MCP implemented directly on serde_json

Status: accepted (2026-11-04)

## Decision
`pixelox-mcp` implements JSON-RPC 2.0 and MCP 2025-06-18 (negotiating down to 2025-03-26
and 2024-11-05) over newline-delimited stdio, as ferraus does, instead of depending on an
SDK. Tool definitions and input schemas come from the op specs. The HTTP transport and MCP
prompts are not implemented.
