"""Build the hash-pinned Google Fonts registry for pixelox-text.

usage: python3 -I gen_gfonts.py COMMIT DOWNLOAD_DIR OUT_JSON
Reads METADATA.pb of each family at the pinned commit of github.com/google/fonts, downloads
every font file and the license file, and records name, size, and SHA-256.
"""
import hashlib, json, os, re, sys, urllib.parse, urllib.request

FAMILIES = [
    "Anton", "Bebas Neue", "League Gothic", "Oswald", "Six Caps", "Antonio", "Big Shoulders Display",
    "Barlow Condensed", "Archivo Black", "Archivo", "Inter", "Unbounded", "Roboto", "Montserrat",
    "Poppins", "Jost", "Raleway", "Quicksand", "Comfortaa", "Nunito", "Fredoka", "Lilita One",
    "Orbitron", "Rajdhani", "Playfair Display", "Bodoni Moda", "DM Serif Display", "Abril Fatface",
    "Rozha One", "EB Garamond", "Libre Baskerville", "Lora", "Cinzel Decorative", "Cinzel", "Marcellus",
    "Uncial Antiqua", "MedievalSharp", "UnifrakturMaguntia", "UnifrakturCook", "Pirata One", "Roboto Slab",
    "Zilla Slab", "Alfa Slab One", "Ultra", "Titan One", "Shrikhand", "Great Vibes", "Allura",
    "Mrs Saint Delafield", "Sacramento", "Pacifico", "Yellowtail", "Lobster", "Permanent Marker", "Caveat",
    "Amatic SC", "Allerta Stencil", "Stardos Stencil", "Saira Stencil One", "Space Mono", "JetBrains Mono",
    "Limelight", "Monoton", "Righteous",
]
LICENSE_FILE = {"ofl": "OFL.txt", "apache": "LICENSE.txt", "ufl": "UFL.txt"}


def get(url):
    req = urllib.request.Request(url, headers={"User-Agent": "registry-builder"})
    with urllib.request.urlopen(req, timeout=60) as r:
        return r.read()


def raw(commit, path):
    return f"https://raw.githubusercontent.com/google/fonts/{commit}/" + urllib.parse.quote(path)


def parse_metadata(text):
    fonts, axes = [], []
    name = re.search(r'^name:\s*"([^"]*)"', text, re.M).group(1)
    lic = re.search(r'^license:\s*"([^"]*)"', text, re.M)
    for block in re.findall(r'^fonts\s*\{(.*?)^\}', text, re.M | re.S):
        def g(k):
            m = re.search(rf'^\s*{k}:\s*"?([^"\n]*)"?', block, re.M)
            return m.group(1) if m else None
        fonts.append({"filename": g("filename"), "style": g("style"), "weight": int(g("weight"))})
    for block in re.findall(r'^axes\s*\{(.*?)^\}', text, re.M | re.S):
        tag = re.search(r'tag:\s*"([^"]*)"', block).group(1)
        lo = float(re.search(r'min_value:\s*([-\d.]+)', block).group(1))
        hi = float(re.search(r'max_value:\s*([-\d.]+)', block).group(1))
        axes.append({"tag": tag, "min": lo, "max": hi})
    return name, (lic.group(1) if lic else None), fonts, axes


def main():
    commit, dl, out = sys.argv[1], sys.argv[2], sys.argv[3]
    only = sys.argv[4:]
    result = {"source": "https://github.com/google/fonts", "commit": commit, "families": []}
    for fam in (only or FAMILIES):
        d = re.sub(r"[^a-z0-9]", "", fam.lower())
        meta = None
        lic = None
        for lic in ("ofl", "apache", "ufl"):
            try:
                meta = get(raw(commit, f"{lic}/{d}/METADATA.pb")).decode("utf-8")
                break
            except Exception:
                continue
        if meta is None:
            print("MISSING", fam, file=sys.stderr)
            continue
        name, license_name, fonts, axes = parse_metadata(meta)
        files = []
        seen = set()
        for f in fonts:
            if f["filename"] in seen:
                continue
            seen.add(f["filename"])
            data = get(raw(commit, f"{lic}/{d}/{f['filename']}"))
            os.makedirs(os.path.join(dl, d), exist_ok=True)
            with open(os.path.join(dl, d, f["filename"]), "wb") as fh:
                fh.write(data)
            files.append({"name": f["filename"], "style": f["style"], "weight": f["weight"],
                          "bytes": len(data), "sha256": hashlib.sha256(data).hexdigest()})
        lf = LICENSE_FILE[lic]
        ldata = get(raw(commit, f"{lic}/{d}/{lf}"))
        with open(os.path.join(dl, d, lf), "wb") as fh:
            fh.write(ldata)
        result["families"].append({
            "family": name, "dir": f"{lic}/{d}", "license": license_name or lic.upper(),
            "license_file": {"name": lf, "bytes": len(ldata), "sha256": hashlib.sha256(ldata).hexdigest()},
            "axes": axes, "files": files,
        })
        print(fam, "->", name, lic, len(files), "files", sum(x["bytes"] for x in files) // 1024, "KB",
              file=sys.stderr)
    with open(out, "w") as fh:
        json.dump(result, fh, indent=1)
        fh.write("\n")


main()
