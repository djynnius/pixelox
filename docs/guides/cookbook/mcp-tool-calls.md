# MCP tool calls

```bash
claude mcp add pixelox -- pixelox mcp serve --allow ~/work
```

A session, as JSON-RPC `tools/call` arguments:

```json
{"name": "pixelox_describe", "arguments": {}}
{"name": "pixelox_run", "arguments": {"command": "open", "params": {"input": "work/shot.jpg", "output": "work/shot.pxo"}}}
{"name": "pixelox_run", "arguments": {"command": "adjust.curves", "params": {"input": "work/shot.pxo", "in_place": true, "points": "0,0 64,56 192,206 255,255"}, "dry_run": true}}
{"name": "pixelox_preview", "arguments": {"input": "work/shot.pxo", "max_size": 768}}
{"name": "pixelox_run", "arguments": {"command": "export", "params": {"input": "work/shot.pxo", "output": "work/final.jpg", "quality": 90}}}
{"name": "pixelox_run", "arguments": {"command": "verify", "params": {"input": "work/final.jpg.pixelox.json"}}}
```

`params` are the command's flags with underscores (`in_place`, `apply_mask`), plus
`input`/`inputs` and `output`. The server refuses paths outside `--allow` (PX1310), and
model downloads need `--allow-model-downloads` on the server and
`"consent": ["model-download"]` on the call.
