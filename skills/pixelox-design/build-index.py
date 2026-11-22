#!/usr/bin/env python3
"""Rebuild the generated tables of references/index.md and references/catalog.md from the
poster files' front matter.

    python3 build-index.py          rewrite both files in place
    python3 build-index.py --check  exit 1 if either file is out of date (for CI)

Only the regions between `<!-- BEGIN GENERATED: name -->` and `<!-- END GENERATED: name -->`
are replaced; everything else in those files is written by hand. Run it after adding or
re-tagging a poster description. Standard library only; output is deterministic.
"""
import colorsys
import glob
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
POSTERS = os.path.join(HERE, "references", "posters")
FILES = [os.path.join(HERE, "references", n) for n in ("index.md", "catalog.md")]
LIST_KEYS = ("genre", "mood", "style", "palette", "techniques", "needs")


def front_matter(path):
    text = open(path, encoding="utf-8").read()
    m = re.match(r"---\n(.*?)\n---\n", text, re.S)
    if not m:
        raise SystemExit(f"{path}: no front matter")
    d = {"file": os.path.basename(path)}
    for line in m.group(1).splitlines():
        if ":" not in line:
            continue
        k, v = line.split(":", 1)
        k, v = k.strip(), v.strip()
        if v.startswith("["):
            v = [x.strip().strip("'\"") for x in v.strip("[]").split(",") if x.strip()]
        elif k in LIST_KEYS:
            v = [v]
        d[k] = v
    return d


def family(hex_color):
    """Name the color family of a ground color."""
    r, g, b = (int(hex_color[i:i + 2], 16) / 255 for i in (1, 3, 5))
    h, l, s = colorsys.rgb_to_hls(r, g, b)
    h *= 360
    if l < 0.16 or (l < 0.28 and s < 0.35):
        return "black and charcoal"
    if l > 0.82 and s < 0.6 or (l > 0.7 and s < 0.25):
        return "white, cream, and pale"
    if s < 0.18:
        return "gray and stone"
    if h < 15 or h >= 345:
        return "red"
    if h < 45:
        return "brown and earth" if l < 0.4 else "orange and amber"
    if h < 70:
        return "yellow and gold"
    if h < 160:
        return "green"
    if h < 200:
        return "teal and cyan"
    if h < 255:
        return "blue and navy"
    if h < 300:
        return "purple and violet"
    return "pink and magenta"


def ids(rows):
    return " ".join(r["id"] for r in rows)


def grouped(rows, key, order=None):
    groups = {}
    for r in rows:
        for v in r[key] if isinstance(r[key], list) else [r[key]]:
            groups.setdefault(v, []).append(r)
    names = order or sorted(groups, key=lambda k: (-len(groups[k]), k))
    out = ["| " + key.capitalize() + " | Count | IDs |", "| --- | --- | --- |"]
    for n in names:
        if n in groups:
            out.append(f"| {n} | {len(groups[n])} | {ids(groups[n])} |")
    return "\n".join(out)


def build(rows):
    gen = {}
    table = ["| ID | Title | Genre | Mood | Format | Archetype | Palette | Key techniques | Needs |",
             "| --- | --- | --- | --- | --- | --- | --- | --- | --- |"]
    for r in rows:
        pal = " ".join(f"`{c}`" for c in r["palette"][:5])
        table.append(
            f"| [{r['id']}](posters/{r['file']}) | {r['title']} | {', '.join(r['genre'])} | {', '.join(r['mood'])} | "
            f"{r['format'].replace('portrait-', '')} | {r['archetype']} | {pal} | {', '.join(r['techniques'][:5])} | {', '.join(r['needs'])} |")
    gen["posters"] = "\n".join(table)
    gen["by-archetype"] = grouped(rows, "archetype")
    gen["by-genre"] = grouped(rows, "genre")
    gen["by-mood"] = grouped(rows, "mood")
    gen["by-style"] = grouped(rows, "style")
    gen["by-format"] = grouped(rows, "format")
    gen["by-technique"] = grouped(rows, "techniques")
    need = {"nothing (type, shapes, and generators only)": [], "a photo (used whole, not cut out)": [],
            "a cutout of a person": [], "a cutout of a product": [], "an illustration": []}
    keymap = {"nothing": "nothing (type, shapes, and generators only)", "photo": "a photo (used whole, not cut out)",
              "subject-cutout": "a cutout of a person", "product-cutout": "a cutout of a product", "illustration": "an illustration"}
    for r in rows:
        real = [n for n in r["needs"] if n != "logo"] or ["nothing"]
        for n in real:
            need[keymap[n]].append(r)
    out = ["| The design needs | Count | IDs |", "| --- | --- | --- |"]
    for k, v in need.items():
        out.append(f"| {k} | {len(v)} | {ids(v)} |")
    gen["by-need"] = "\n".join(out)
    fam = {}
    for r in rows:
        fam.setdefault(family(r["palette"][0]), []).append(r)
    out = ["| Ground color family | Count | IDs |", "| --- | --- | --- |"]
    for k in sorted(fam, key=lambda k: (-len(fam[k]), k)):
        out.append(f"| {k} | {len(fam[k])} | {ids(fam[k])} |")
    gen["by-palette"] = "\n".join(out)
    gen["count"] = f"{len(rows)} posters."
    return gen


def main():
    rows = sorted((front_matter(p) for p in glob.glob(os.path.join(POSTERS, "p[0-9]*.md"))), key=lambda r: r["id"])
    gen = build(rows)
    stale, seen = [], set()
    for path in FILES:
        old = text = open(path, encoding="utf-8").read()
        for name, body in gen.items():
            pat = re.compile(r"(<!-- BEGIN GENERATED: " + re.escape(name) + r" -->\n).*?(<!-- END GENERATED: " + re.escape(name) + r" -->)", re.S)
            if pat.search(text):
                seen.add(name)
                text = pat.sub(lambda m: m.group(1) + body + "\n" + m.group(2), text)
        if text != old:
            stale.append(os.path.basename(path))
            if "--check" not in sys.argv:
                open(path, "w", encoding="utf-8").write(text)
    missing = sorted(set(gen) - seen)
    if missing:
        raise SystemExit(f"no generated region named {missing} in index.md or catalog.md")
    if "--check" in sys.argv:
        if stale:
            raise SystemExit(f"out of date: {', '.join(stale)}: run python3 build-index.py")
        print("index.md and catalog.md are up to date")
    else:
        print(f"rebuilt from {len(rows)} poster files" + (f" (changed: {', '.join(stale)})" if stale else " (no change)"))


main()
