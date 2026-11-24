#!/usr/bin/env python3
"""Regenerate crates/pixelox-render/vendor/ (bundled JavaScript libraries and map data, ADR 0018).

    python3 -I crates/pixelox-render/scripts/vendor-js.py
    python3 -I crates/pixelox-render/scripts/vendor-js.py --tarballs DIR   # offline: DIR holds <pkg>-<version>.tgz

Fetches each pinned npm package tarball from https://registry.npmjs.org (or reads it from
--tarballs), checks it against the sha512 integrity pinned in PACKAGES below (the value npm
publishes as `dist.integrity`), reads only the listed files from the tarball in memory, and
writes:

    vendor/<package>/<file>       the files, byte for byte as published
    vendor/LICENSE-<package>.txt  each package's license text, with its source and notes
    vendor/us-states.tsv          FIPS code, USPS code, and name of the 56 us-atlas states
                                  and territories (U.S. Census Bureau; public domain),
                                  checked against the atlas
    vendor/manifest.tsv           path, package, version, license, bytes, SHA-256 of every
                                  file (read by build.rs, which embeds the files compressed,
                                  and by the runtime, which checks every file it loads)
    vendor/README.md              the same as a table, with the update procedure

To update a package: change its version and integrity here (the script prints the
registry's integrity when they disagree), run the script, run
`cargo test -p pixelox-render`, look at the cookbook renders, and record the change in the
CHANGELOG and ADR 0018. Standard library only.
"""

import argparse
import base64
import hashlib
import io
import json
import os
import sys
import tarfile
import urllib.request

HERE = os.path.dirname(os.path.abspath(__file__))
VENDOR = os.path.normpath(os.path.join(HERE, "..", "vendor"))

# package, version, license, npm dist.integrity, [(file in the tarball, path under vendor/<package>/)]
PACKAGES = [
    ("d3", "7.9.0", "ISC",
     "sha512-e1U46jVP+w7Iut8Jt8ri1YsPOvFpg46k+K8TpCb0P+zjCkjkPnV7WzfDJzMHy1LnA+wj5pLT1wjO901gLXeEhA==",
     [("dist/d3.min.js", "d3.min.js")]),
    ("topojson-client", "3.1.0", "ISC",
     "sha512-605uxS6bcYxGXw9qi62XyrV6Q3xwbndjachmNxu8HWTtVPxZfEJN9fd/SZS1Q54Sn2y0TMyMxFj/cJINqGHrKw==",
     [("dist/topojson-client.min.js", "topojson-client.min.js")]),
    ("three", "0.187.0", "MIT",
     "sha512-Qiu7R+Cmh8CstZsY1uJOSWnVlfo7ZtNLV7mF8Jltk3LB3AtDu9cRZlcnRKkAk8Y4z6KKw35z7Y7Urd7jWY8etA==",
     [("build/three.core.js", "three.core.js"),
      ("examples/jsm/renderers/Projector.js", "addons/renderers/Projector.js"),
      ("examples/jsm/renderers/SVGRenderer.js", "addons/renderers/SVGRenderer.js"),
      ("examples/jsm/geometries/RoundedBoxGeometry.js", "addons/geometries/RoundedBoxGeometry.js")]),
    ("world-atlas", "2.0.2", "ISC",
     "sha512-IXfV0qwlKXpckz1FhwXVwKRjiIhOnWttOskm5CtxMsjgE/MXAYRHWJqgXOpM8IkcPBoXnyTU5lFHcYa5ChG0LQ==",
     [("countries-110m.json", "countries-110m.json"),
      ("countries-50m.json", "countries-50m.json"),
      ("land-110m.json", "land-110m.json")]),
    ("us-atlas", "3.0.1", "ISC",
     "sha512-wEIZCq0ImPvGblTd8gZMqNNCPkQshugMUG/8nkSWXb02+XqNFREg9atHOKP9w6prLZTpqcLhSvdBW81MkV3/0Q==",
     [("states-10m.json", "states-10m.json"),
      ("states-albers-10m.json", "states-albers-10m.json"),
      ("nation-10m.json", "nation-10m.json")]),
    ("i18n-iso-countries", "7.14.0", "MIT",
     "sha512-nXHJZYtNrfsi1UQbyRqm3Gou431elgLjKl//CYlnBGt5aTWdRPH1PiS2T/p/n8Q8LnqYqzQJik3Q7mkwvLokeg==",
     [("codes.json", "codes.json")]),
]

NOTES = {
    "d3": "d3.min.js is the UMD bundle of the d3 modules (d3-array ... d3-zoom, all ISC,\n"
          "Copyright Mike Bostock) and their dependencies internmap (ISC, Copyright Mike\n"
          "Bostock), delaunator (ISC, Copyright Mapbox), and robust-predicates (Unlicense,\n"
          "Vladimir Agafonkin).",
    "three": "three.core.js is the self-contained core ES module of three.js. The addons\n"
             "(Projector.js, SVGRenderer.js, RoundedBoxGeometry.js) come from examples/jsm in\n"
             "the same package and the same license.",
    "world-atlas": "The geometry is Natural Earth's Admin 0 country and land boundaries (version\n"
                   "4.1.0, 1:110m and 1:50m), which Natural Earth places in the public domain\n"
                   "(https://www.naturalearthdata.com/about/terms-of-use/). Country boundaries are\n"
                   "Natural Earth's de facto boundaries; pixelox takes no position on them.",
    "us-atlas": "The geometry is the U.S. Census Bureau's cartographic boundary files, 2017\n"
                "edition (a work of the U.S. federal government, public domain).",
    "i18n-iso-countries": "Only codes.json (ISO 3166-1 alpha-2, alpha-3, and numeric codes) is bundled;\n"
                          "pixelox uses it to add iso_a2 and iso_a3 to the world atlases' countries.",
}

# FIPS state code, USPS code, name as us-atlas spells it (U.S. Census Bureau; public domain).
US_STATES = [
    ("01", "AL", "Alabama"), ("02", "AK", "Alaska"), ("04", "AZ", "Arizona"),
    ("05", "AR", "Arkansas"), ("06", "CA", "California"), ("08", "CO", "Colorado"),
    ("09", "CT", "Connecticut"), ("10", "DE", "Delaware"), ("11", "DC", "District of Columbia"),
    ("12", "FL", "Florida"), ("13", "GA", "Georgia"), ("15", "HI", "Hawaii"),
    ("16", "ID", "Idaho"), ("17", "IL", "Illinois"), ("18", "IN", "Indiana"),
    ("19", "IA", "Iowa"), ("20", "KS", "Kansas"), ("21", "KY", "Kentucky"),
    ("22", "LA", "Louisiana"), ("23", "ME", "Maine"), ("24", "MD", "Maryland"),
    ("25", "MA", "Massachusetts"), ("26", "MI", "Michigan"), ("27", "MN", "Minnesota"),
    ("28", "MS", "Mississippi"), ("29", "MO", "Missouri"), ("30", "MT", "Montana"),
    ("31", "NE", "Nebraska"), ("32", "NV", "Nevada"), ("33", "NH", "New Hampshire"),
    ("34", "NJ", "New Jersey"), ("35", "NM", "New Mexico"), ("36", "NY", "New York"),
    ("37", "NC", "North Carolina"), ("38", "ND", "North Dakota"), ("39", "OH", "Ohio"),
    ("40", "OK", "Oklahoma"), ("41", "OR", "Oregon"), ("42", "PA", "Pennsylvania"),
    ("44", "RI", "Rhode Island"), ("45", "SC", "South Carolina"), ("46", "SD", "South Dakota"),
    ("47", "TN", "Tennessee"), ("48", "TX", "Texas"), ("49", "UT", "Utah"),
    ("50", "VT", "Vermont"), ("51", "VA", "Virginia"), ("53", "WA", "Washington"),
    ("54", "WV", "West Virginia"), ("55", "WI", "Wisconsin"), ("56", "WY", "Wyoming"),
    ("60", "AS", "American Samoa"), ("66", "GU", "Guam"),
    ("69", "MP", "Commonwealth of the Northern Mariana Islands"), ("72", "PR", "Puerto Rico"),
    ("78", "VI", "United States Virgin Islands"),
]


def tarball_url(pkg, version):
    return "https://registry.npmjs.org/%s/-/%s-%s.tgz" % (pkg, pkg.split("/")[-1], version)


def find_local(root, name):
    for d, _, files in os.walk(root):
        if name in files:
            return os.path.join(d, name)
    return None


def fetch(pkg, version, integrity, tarballs):
    name = "%s-%s.tgz" % (pkg, version)
    if tarballs:
        path = find_local(tarballs, name)
        if not path:
            raise SystemExit("%s not found under %s" % (name, tarballs))
        raw = open(path, "rb").read()
    else:
        with urllib.request.urlopen(tarball_url(pkg, version)) as r:
            raw = r.read()
    got = "sha512-" + base64.b64encode(hashlib.sha512(raw).digest()).decode()
    if got != integrity:
        raise SystemExit("%s %s: integrity %s does not match the pinned %s" % (pkg, version, got, integrity))
    return raw


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--tarballs", help="a folder holding the downloaded <pkg>-<version>.tgz files")
    args = ap.parse_args()
    rows = []
    sources = []
    atlas_states = None
    for pkg, version, lic, integrity, files in PACKAGES:
        raw = fetch(pkg, version, integrity, args.tarballs)
        sources.append((pkg, version, lic, tarball_url(pkg, version), integrity, hashlib.sha256(raw).hexdigest()))
        tar = tarfile.open(fileobj=io.BytesIO(raw), mode="r:gz")

        def read(member):
            f = tar.extractfile("package/" + member)
            if f is None:
                raise SystemExit("%s %s has no %s" % (pkg, version, member))
            return f.read()

        meta = json.loads(read("package.json"))
        if meta["version"] != version or meta.get("license") != lic:
            raise SystemExit("unexpected package.json: %s %s %s" % (pkg, meta["version"], meta.get("license")))
        for member, dest in files:
            data = read(member)
            out = os.path.join(VENDOR, pkg, dest)
            os.makedirs(os.path.dirname(out), exist_ok=True)
            with open(out, "wb") as f:
                f.write(data)
            rows.append(("%s/%s" % (pkg, dest), pkg, version, lic, len(data), hashlib.sha256(data).hexdigest()))
            if pkg == "us-atlas" and dest == "states-10m.json":
                atlas_states = json.loads(data)
        lic_text = read("LICENSE").decode("utf-8").rstrip() + "\n"
        with open(os.path.join(VENDOR, "LICENSE-%s.txt" % pkg), "w", encoding="utf-8", newline="\n") as f:
            f.write("%s %s (%s)\n\n" % (pkg, version, tarball_url(pkg, version)))
            f.write(lic_text)
            if pkg in NOTES:
                f.write("\n---\n\n" + NOTES[pkg] + "\n")

    # the state table must name every state the atlas has, exactly as it spells them
    names = {g["id"]: g["properties"]["name"] for g in atlas_states["objects"]["states"]["geometries"]}
    table = {fips: name for fips, _, name in US_STATES}
    if names != table:
        raise SystemExit("us-states table disagrees with us-atlas: %s" % sorted(set(names.items()) ^ set(table.items())))
    tsv = "# fips\tpostal\tname (U.S. Census Bureau FIPS state codes and USPS abbreviations; public domain)\n"
    tsv += "".join("%s\t%s\t%s\n" % s for s in US_STATES)
    with open(os.path.join(VENDOR, "us-states.tsv"), "w", encoding="utf-8", newline="\n") as f:
        f.write(tsv)
    data = tsv.encode("utf-8")
    rows.append(("us-states.tsv", "pixelox", "-", "public domain", len(data), hashlib.sha256(data).hexdigest()))

    with open(os.path.join(VENDOR, "manifest.tsv"), "w", encoding="utf-8", newline="\n") as f:
        f.write("# generated by scripts/vendor-js.py; do not edit (ADR 0018)\n")
        f.write("# path\tpackage\tversion\tlicense\tbytes\tsha256\n")
        for r in rows:
            f.write("%s\t%s\t%s\t%s\t%d\t%s\n" % r)

    md = ["# Vendored JavaScript libraries and map data", "",
          "Generated by `scripts/vendor-js.py`; do not edit by hand. See ADR 0018",
          "(`docs/decisions/0018-vendored-js-libraries-and-map-data.md`) for how pixelox uses these",
          "files and how to update them. `build.rs` embeds every file listed in `manifest.tsv`",
          "(deflate-compressed); the runtime checks each file's SHA-256 before it runs or parses it.",
          "", "## Packages", "",
          "| Package | Version | License | Tarball | npm integrity | Tarball SHA-256 |",
          "| --- | --- | --- | --- | --- | --- |"]
    for pkg, version, lic, url, integrity, sha in sources:
        md.append("| %s | %s | %s | %s | `%s` | `%s` |" % (pkg, version, lic, url, integrity, sha))
    md += ["", "## Files", "", "| File | Package | Version | License | Bytes | SHA-256 |", "| --- | --- | --- | --- | --- | --- |"]
    for r in rows:
        md.append("| `%s` | %s | %s | %s | %d | `%s` |" % r)
    md += ["", "`us-states.tsv` is written by the script (FIPS and USPS codes, public domain) and",
           "checked against `us-atlas/states-10m.json`. License texts: `LICENSE-<package>.txt`.", ""]
    with open(os.path.join(VENDOR, "README.md"), "w", encoding="utf-8", newline="\n") as f:
        f.write("\n".join(md))
    total = sum(r[4] for r in rows)
    print("wrote %d files (%d bytes) from %d packages into %s" % (len(rows), total, len(PACKAGES), VENDOR))


if __name__ == "__main__":
    sys.exit(main())
