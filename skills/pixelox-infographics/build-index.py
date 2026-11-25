#!/usr/bin/env python3
"""Rebuild the generated tables of references/index.md from the infographic files' front matter.

    python3 build-index.py          rewrite index.md in place
    python3 build-index.py --check  exit 1 if index.md is out of date, or if a script copied from
                                    pixelox-design (rays, pattern, blobs, standin) no longer matches its original

Only the regions between `<!-- BEGIN GENERATED: name -->` and `<!-- END GENERATED: name -->` are replaced;
everything else in index.md is written by hand. Run it after adding or re-tagging an infographic
description. Standard library only; output is deterministic.
"""
import colorsys
import glob
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ITEMS = os.path.join(HERE, "references", "infographics")
INDEX = os.path.join(HERE, "references", "index.md")
LIST_KEYS = ("kind", "topic", "mood", "style", "data", "charts", "palette", "needs")


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
    if l < 0.16 or (l < 0.3 and s < 0.6):
        return "dark (black, navy, charcoal)"
    if l > 0.9 or (l > 0.8 and s < 0.5):
        return "white and near-white"
    if s < 0.18:
        return "gray"
    if h < 15 or h >= 345:
        return "red"
    if h < 45:
        return "orange and amber"
    if h < 70:
        return "yellow"
    if h < 160:
        return "green"
    if h < 200:
        return "teal and cyan"
    if h < 255:
        return "blue"
    if h < 300:
        return "purple and violet"
    return "pink and magenta"


def ids(rows):
    return " ".join(r["id"] for r in rows)


def grouped(rows, key, title=None):
    groups = {}
    for r in rows:
        for v in r[key] if isinstance(r[key], list) else [r[key]]:
            groups.setdefault(v, []).append(r)
    out = ["| " + (title or key.capitalize()) + " | Count | IDs |", "| --- | --- | --- |"]
    for n in sorted(groups, key=lambda k: (-len(groups[k]), k)):
        out.append(f"| {n} | {len(groups[n])} | {ids(groups[n])} |")
    return "\n".join(out)


def build(rows):
    gen = {}
    table = ["| ID | Title | Kind | Structure | Data | Charts | Format | Palette | Needs |",
             "| --- | --- | --- | --- | --- | --- | --- | --- | --- |"]
    for r in rows:
        pal = " ".join(f"`{c}`" for c in r["palette"][:5])
        table.append(
            f"| [{r['id']}](infographics/{r['file']}) | {r['title']} | {', '.join(r['kind'])} | {r['structure']} | "
            f"{', '.join(r['data'])} | {', '.join(r['charts'][:5])} | {r['format']} | {pal} | {', '.join(r['needs'])} |")
    gen["infographics"] = "\n".join(table)
    for key, title in (("structure", "Structure"), ("kind", "Kind"), ("topic", "Topic"), ("mood", "Mood"), ("style", "Style"),
                       ("format", "Format"), ("data", "The data is"), ("charts", "Chart"), ("needs", "The design needs")):
        gen["by-" + key] = grouped(rows, key, title)
    fam = {}
    for r in rows:
        fam.setdefault(family(r["palette"][0]), []).append(r)
    out = ["| Ground color | Count | IDs |", "| --- | --- | --- |"]
    for k in sorted(fam, key=lambda k: (-len(fam[k]), k)):
        out.append(f"| {k} | {len(fam[k])} | {ids(fam[k])} |")
    gen["by-palette"] = "\n".join(out)
    gen["count"] = f"{len(rows)} infographics."
    return gen


SHARED = ("rays.js", "pattern.js", "blobs.js", "standin.js")


def shared_scripts_in_sync():
    """scripts/rays.js, pattern.js, blobs.js, and standin.js are copies of pixelox-design/scripts (so this
    skill installs alone). Each copy is the original plus one first-line note. Returns the names that differ;
    an absent pixelox-design skill is not an error."""
    other = os.path.join(HERE, "..", "pixelox-design", "scripts")
    out = []
    for name in SHARED:
        a, b = os.path.join(HERE, "scripts", name), os.path.join(other, name)
        if os.path.exists(a) and os.path.exists(b):
            mine = open(a, encoding="utf-8").read().split("\n", 1)[1]
            if mine != open(b, encoding="utf-8").read():
                out.append(name)
    return out


def main():
    drift = shared_scripts_in_sync()
    if drift:
        msg = "copied scripts differ from pixelox-design/scripts: " + ", ".join(drift) + " (copy them again, keeping the first-line note)"
        if "--check" in sys.argv:
            raise SystemExit(msg)
        print("warning: " + msg)
    rows = sorted((front_matter(p) for p in glob.glob(os.path.join(ITEMS, "i[0-9]*.md"))), key=lambda r: r["id"])
    gen = build(rows)
    old = text = open(INDEX, encoding="utf-8").read()
    seen = set()
    for name, body in gen.items():
        pat = re.compile(r"(<!-- BEGIN GENERATED: " + re.escape(name) + r" -->\n).*?(<!-- END GENERATED: " + re.escape(name) + r" -->)", re.S)
        if pat.search(text):
            seen.add(name)
            text = pat.sub(lambda m: m.group(1) + body + "\n" + m.group(2), text)
    missing = sorted(set(gen) - seen)
    if missing:
        raise SystemExit(f"no generated region named {missing} in index.md")
    if "--check" in sys.argv:
        if text != old:
            raise SystemExit("out of date: index.md: run python3 build-index.py")
        print("index.md is up to date")
    else:
        if text != old:
            open(INDEX, "w", encoding="utf-8").write(text)
        print(f"rebuilt from {len(rows)} infographic files" + (" (changed)" if text != old else " (no change)"))


if __name__ == "__main__":
    main()
