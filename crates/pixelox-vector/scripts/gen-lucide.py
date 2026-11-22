#!/usr/bin/env python3
"""Regenerate crates/pixelox-vector/data/lucide-icons.txt from a lucide-static release.

    python3 -I crates/pixelox-vector/scripts/gen-lucide.py 1.77.0

Downloads https://registry.npmjs.org/lucide-static/-/lucide-static-<version>.tgz (or reads
a tarball given with --tarball), verifies nothing beyond what npm serves, and writes one line
per icon:

    name <TAB> aliases (comma-separated) <TAB> tags (comma-separated) <TAB> SVG path data

The path data is every element of the icon (path, circle, ellipse, rect, line, polyline,
polygon) as absolute-start SVG path data on Lucide's 24 x 24 grid, drawn with a 2-unit round
stroke and no fill. Aliases are the release's deprecated names whose SVG is identical to a
current icon, plus a few common words (FRIENDLY below). Icons that depict trademarks are left
out (EXCLUDED). The license text is copied to data/LICENSE-lucide.txt.

Standard library only; run it with `python3 -I` from any directory.
"""

import argparse
import hashlib
import io
import json
import os
import re
import sys
import tarfile
import urllib.request

HERE = os.path.dirname(os.path.abspath(__file__))
DATA = os.path.join(HERE, "..", "data")

# Icons that depict a registered trademark or certification mark: not bundled.
EXCLUDED = {
    "bluetooth",
    "bluetooth-connected",
    "bluetooth-off",
    "bluetooth-searching",
    "creative-commons",
}

# Plain words agents reach for, mapped to the Lucide name.
FRIENDLY = {
    "arrow": "arrow-right",
    "cart": "shopping-cart",
    "date": "calendar",
    "email": "mail",
    "envelope": "mail",
    "location": "map-pin",
    "like": "thumbs-up",
    "people": "users",
    "person": "user",
    "social": "share-2",
    "telephone": "phone",
    "time": "clock",
    "web": "globe",
    "website": "globe",
}

MAX_TAGS = 10

# an SVG number: sign, digits, optional fraction (".5" too), optional exponent
NUM = r"[-+]?(?:\d+\.?\d*|\.\d+)(?:[eE][-+]?\d+)?"


def num(v):
    f = float(v)
    s = ("%.4f" % f).rstrip("0").rstrip(".")
    return "0" if s in ("-0", "") else s


def element_d(el, a):
    if el == "path":
        d = a["d"].strip()
        # the first moveto of an element is absolute even when written `m` (its implicit
        # follow-on pairs stay relative linetos); spell that out so elements can be joined
        if d[0] == "m":
            m = re.match(r"m\s*(" + NUM + r")\s*,?\s*(" + NUM + r")\s*,?\s*(.*)$", d, re.S)
            x, y, rest = m.group(1), m.group(2), m.group(3)
            if rest and (rest[0].isdigit() or rest[0] in "-+."):
                rest = "l" + rest
            d = "M%s %s%s" % (x, y, rest)
        return d
    if el == "circle":
        cx, cy, r = float(a["cx"]), float(a["cy"]), float(a["r"])
        d = "M%s %sa%s %s 0 1 0 %s 0a%s %s 0 1 0 %s 0Z" % (
            num(cx - r), num(cy), num(r), num(r), num(2 * r), num(r), num(r), num(-2 * r))
        if a.get("fill", "none") != "none":
            # a filled dot (r <= 1): the stroke of so small a circle leaves a pinhole in the
            # middle; a zero-length round-capped segment at its center closes it
            d += " M%s %sl0 0" % (num(cx), num(cy))
        return d
    if el == "ellipse":
        cx, cy, rx, ry = (float(a[k]) for k in ("cx", "cy", "rx", "ry"))
        return "M%s %sa%s %s 0 1 0 %s 0a%s %s 0 1 0 %s 0Z" % (
            num(cx - rx), num(cy), num(rx), num(ry), num(2 * rx), num(rx), num(ry), num(-2 * rx))
    if el == "rect":
        x, y, w, h = (float(a[k]) for k in ("x", "y", "width", "height"))
        rx = float(a.get("rx", a.get("ry", 0)))
        ry = float(a.get("ry", a.get("rx", 0)))
        rx, ry = min(rx, w / 2), min(ry, h / 2)
        if rx <= 0 or ry <= 0:
            return "M%s %sh%sv%sh%sZ" % (num(x), num(y), num(w), num(h), num(-w))
        return "M%s %sh%sa%s %s 0 0 1 %s %sv%sa%s %s 0 0 1 %s %sh%sa%s %s 0 0 1 %s %sv%sa%s %s 0 0 1 %s %sZ" % (
            num(x + rx), num(y), num(w - 2 * rx),
            num(rx), num(ry), num(rx), num(ry), num(h - 2 * ry),
            num(rx), num(ry), num(-rx), num(ry), num(-(w - 2 * rx)),
            num(rx), num(ry), num(-rx), num(-ry), num(-(h - 2 * ry)),
            num(rx), num(ry), num(rx), num(-ry))
    if el == "line":
        return "M%s %sL%s %s" % (num(a["x1"]), num(a["y1"]), num(a["x2"]), num(a["y2"]))
    if el in ("polyline", "polygon"):
        pts = re.split(r"[\s,]+", a["points"].strip())
        xy = [num(p) for p in pts if p]
        pairs = ["%s %s" % (xy[i], xy[i + 1]) for i in range(0, len(xy) - 1, 2)]
        d = "M" + pairs[0] + "".join("L" + p for p in pairs[1:])
        return d + ("Z" if el == "polygon" else "")
    raise SystemExit("unknown element %r" % el)


def inner_svg(text):
    """The drawing elements of an SVG file, whitespace-normalized (to match aliases)."""
    body = text[text.index(">", text.index("<svg")) + 1:text.rindex("</svg>")]
    return re.sub(r"\s+", " ", body).strip()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("version")
    ap.add_argument("--tarball", help="a downloaded lucide-static .tgz instead of fetching it")
    args = ap.parse_args()
    url = "https://registry.npmjs.org/lucide-static/-/lucide-static-%s.tgz" % args.version
    if args.tarball:
        raw = open(args.tarball, "rb").read()
    else:
        with urllib.request.urlopen(url) as r:
            raw = r.read()
    sha = hashlib.sha256(raw).hexdigest()
    tar = tarfile.open(fileobj=io.BytesIO(raw), mode="r:gz")

    def read(name):
        return tar.extractfile("package/" + name).read().decode("utf-8")

    meta = json.loads(read("package.json"))
    if meta["version"] != args.version or meta["license"] != "ISC":
        raise SystemExit("unexpected package: %s %s" % (meta["version"], meta["license"]))
    nodes = json.loads(read("icon-nodes.json"))
    tags = json.loads(read("tags.json"))

    svgs = {}
    for m in tar.getmembers():
        if m.name.startswith("package/icons/") and m.name.endswith(".svg"):
            svgs[m.name[len("package/icons/"):-4]] = inner_svg(tar.extractfile(m).read().decode("utf-8"))
    by_body = {}
    for name in nodes:
        by_body.setdefault(svgs[name], name)
    aliases = {name: [] for name in nodes}
    for name, body in sorted(svgs.items()):
        if name in nodes:
            continue
        target = by_body.get(body)
        if target and target not in EXCLUDED:
            aliases[target].append(name)
    taken = set(nodes) | {a for v in aliases.values() for a in v}
    for word, target in FRIENDLY.items():
        if word in taken:
            raise SystemExit("friendly alias %r collides with an icon or alias" % word)
        aliases[target].append(word)

    lines = [
        "# Lucide %s icons (lucide-static, ISC; Feather-derived icons MIT): see LICENSE-lucide.txt" % args.version,
        "# source %s sha256 %s" % (url, sha),
        "# generated by scripts/gen-lucide.py %s; do not edit. Excluded (trademarks): %s"
        % (args.version, ", ".join(sorted(EXCLUDED))),
        "# name<TAB>aliases<TAB>tags<TAB>SVG path data on the 24x24 grid (stroke 2, round caps and joins)",
    ]
    count = 0
    for name in sorted(nodes):
        if name in EXCLUDED:
            continue
        d = " ".join(element_d(el, a) for el, a in nodes[name])
        words = set(name.split("-"))
        t = []
        for tag in tags.get(name, []):
            tag = re.sub(r"\s+", " ", tag.strip().lower().replace(",", " ").replace("\t", " "))
            if tag and tag not in t and tag not in words and tag != name:
                t.append(tag)
        lines.append("\t".join([name, ",".join(sorted(aliases[name])), ",".join(t[:MAX_TAGS]), d]))
        count += 1
    os.makedirs(DATA, exist_ok=True)
    with open(os.path.join(DATA, "lucide-icons.txt"), "w", encoding="utf-8", newline="\n") as f:
        f.write("\n".join(lines) + "\n")
    with open(os.path.join(DATA, "LICENSE-lucide.txt"), "w", encoding="utf-8", newline="\n") as f:
        f.write("Lucide %s (https://lucide.dev, %s)\n\n" % (args.version, url))
        f.write(read("LICENSE"))
    print("wrote %d icons (%d aliases) from lucide-static %s" % (count, sum(len(v) for v in aliases.values()), args.version))


if __name__ == "__main__":
    sys.exit(main())
