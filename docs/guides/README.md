# pixelox

**A command-line raster and vector graphics workstation for scripts and AI agents.**
Layers, masks, selections, blend modes, adjustment layers, layer styles, type, vector
shapes, filters, retouching, templates, SVG authoring, tracing, and scripted rendering —
every command with a JSON envelope, a JSON Schema, a provenance sidecar, and an MCP tool.

* **New here?** Start with [Getting started](getting-started.md), then the tutorials:
  [edit a photo](tutorials/edit-a-photo.md), [make a poster from a template](tutorials/poster-from-template.md),
  and [cut out a product shot](tutorials/cut-out-a-product.md).
* **Building an agent?** Read the [agent guide](agents.md), the [contract](topics/contract.md),
  and the cookbook: [watermark a folder](cookbook/watermark-a-folder.md),
  [social cards from a CSV](cookbook/social-cards-from-csv.md),
  [MCP tool calls](cookbook/mcp-tool-calls.md), [ImageMagick equivalents](cookbook/imagemagick-equivalents.md),
  [blend a photo composite](cookbook/photo-composite.md).
* **Looking up a command?** Every command has a page under *Command reference*;
  `pixelox docs <command>` prints the same page offline.
* **Something wrong?** The [error catalog](errors.md) lists every code with what to do.

Command pages are generated from the same definitions as the binary, and every command
line in these guides is checked against the CLI in CI.
