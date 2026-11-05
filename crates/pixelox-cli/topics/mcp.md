# MCP server

`pixelox mcp serve --allow ./work [--allow ./assets] [--read-only] [--allow-model-downloads]`
serves the Model Context Protocol over stdio for as long as the client keeps it open.

```bash
claude mcp add pixelox -- pixelox mcp serve --allow ~/work
```

| Tool | Purpose |
| --- | --- |
| `pixelox_describe` | The agent briefing (`describe --agent`); call once |
| `pixelox_schema` | JSON Schema of one command |
| `pixelox_run` | Run any command: `{command, params, dry_run, consent}` → the envelope |
| `pixelox_info` | Inspect an image or document |
| `pixelox_preview` | A downscaled PNG of a document, layer, or region (≤ 768 px) |
| `pixelox_detect` | AI detection commands |
| `pixelox_jobs` | Background jobs |
| `pixelox_docs` | Reference pages and error codes |

Security: every path must resolve, symlinks included, inside an `--allow` root (PX1310);
commands run in-process with no shell; `--read-only` refuses writing commands; model
downloads need both `--allow-model-downloads` and `"consent": ["model-download"]`; every
call is appended to an NDJSON audit log in the state directory.

A session: call `pixelox_describe` once, plan, run steps with `pixelox_run` (dry-run first
for anything that writes), look at results with `pixelox_preview`, and finish with
`verify` on the deliverable's sidecar.
