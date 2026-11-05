# Social cards from a CSV

```yaml
# cards.yml
version: 1
vars: { csv: speakers.csv }
steps:
  - id: cards
    foreach: { csv: "${vars.csv}", as: row }
    run: comp.template.apply
    with:
      input: card.pxo
      set: { name: "${row.name}", title: "${row.title}" }
      set-image: { headshot: "${row.photo}" }
      output: "dist/cards/${row.slug}.png"
```

```bash
pixelox pipeline run cards.yml --var csv=speakers.csv --jobs 4 --dry-run --json
pixelox pipeline run cards.yml --var csv=speakers.csv --jobs 4 --json
```

The dry run resolves the first row and validates every step's parameters; the real run
writes one card per row, each with a sidecar.
