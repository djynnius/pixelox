#!/usr/bin/env python3
"""Drive pixelox from Python and handle envelopes and exit codes (standard library only).

    python3 examples/agents/parse_envelope.py --samples samples --out out/py
"""
import argparse, json, os, subprocess

ap = argparse.ArgumentParser()
ap.add_argument("--samples", required=True)
ap.add_argument("--out", required=True)
a = ap.parse_args()
os.makedirs(a.out, exist_ok=True)
PX = os.environ.get("PIXELOX", "pixelox")

def pixelox(*argv):
    """Run a command; return (exit code, envelope)."""
    p = subprocess.run([PX, *argv, "--json"], capture_output=True, text=True)
    return p.returncode, json.loads(p.stdout)

photo = os.path.join(a.samples, "photo.png")
code, env = pixelox("info", photo)
print("size:", env["result"]["size"], "layers:", env["result"]["layer_count"])

out = os.path.join(a.out, "small.webp")
code, env = pixelox("transform", "resize", photo, "-o", out, "--size", "480x")
assert code == 0, env["error"]
print("wrote", env["outputs"][0]["path"], env["outputs"][0]["size"])

# the same command again: the output exists → exit 4, error PX1401, a hint saying what to do
code, env = pixelox("transform", "resize", photo, "-o", out, "--size", "480x")
assert code == 4 and env["error"]["code"] == "PX1401"
print("refused:", env["error"]["code"], "-", env["error"]["hint"])

# warnings never change the exit code
code, env = pixelox("export", photo, "-o", os.path.join(a.out, "flat.jpg"))
print("warnings:", [w["code"] for w in env["warnings"]] or "none")

# AI commands are gated in this build: exit 12 with a remedy
code, env = pixelox("detect", "subject", photo, "--cutout", os.path.join(a.out, "cut.png"))
print("detect:", code, env["error"]["code"], "-", env["error"]["hint"])
