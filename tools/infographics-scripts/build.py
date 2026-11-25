#!/usr/bin/env python3
"""Build the pixelox-infographics chart generators from their sources.

Each script in skills/pixelox-infographics/scripts/<name>.js is
<name>.head.js (its documented parameters) + prelude.js (shared helpers) + <name>.body.js.
Edit the sources here, never the built scripts, then run:

    python3 tools/infographics-scripts/build.py           rebuild the scripts
    python3 tools/infographics-scripts/build.py --check   exit 1 if a built script is out of date (CI)
"""
import glob, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "..", "..", "skills", "pixelox-infographics", "scripts")


def main() -> int:
    check = "--check" in sys.argv
    pre = open(os.path.join(HERE, "prelude.js"), encoding="utf-8").read()
    stale = []
    for body in sorted(glob.glob(os.path.join(HERE, "*.body.js"))):
        name = os.path.basename(body)[: -len(".body.js")]
        head = open(os.path.join(HERE, name + ".head.js"), encoding="utf-8").read()
        text = head.rstrip("\n") + "\n" + pre + open(body, encoding="utf-8").read()
        dest = os.path.join(OUT, name + ".js")
        current = open(dest, encoding="utf-8").read() if os.path.exists(dest) else None
        if current == text:
            continue
        if check:
            stale.append(name)
        else:
            with open(dest, "w", encoding="utf-8") as f:
                f.write(text)
            print("built", name)
    if stale:
        print("out of date (run tools/infographics-scripts/build.py):", ", ".join(stale))
        return 1
    if check:
        print("infographics scripts are up to date")
    return 0


if __name__ == "__main__":
    sys.exit(main())
