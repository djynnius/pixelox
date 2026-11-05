#!/usr/bin/env python3
"""A minimal MCP client for `pixelox mcp serve` over stdio (standard library only).

    python3 examples/mcp/client.py --samples samples --out out/mcp

Shows: initialize, tools/list, pixelox_describe, a dry run then a real run through
pixelox_run, pixelox_preview (image content), and the sandbox refusing an outside path.
"""
import argparse, json, os, shutil, subprocess, sys

ap = argparse.ArgumentParser()
ap.add_argument("--samples", required=True)
ap.add_argument("--out", required=True)
args = ap.parse_args()
os.makedirs(args.out, exist_ok=True)
work = os.path.abspath(args.out)
shutil.copy(os.path.join(args.samples, "photo.png"), os.path.join(work, "photo.png"))

px = os.environ.get("PIXELOX", "pixelox")
proc = subprocess.Popen([px, "mcp", "serve", "--allow", work], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True)
next_id = 0

def call(method, params=None):
    global next_id
    next_id += 1
    msg = {"jsonrpc": "2.0", "id": next_id, "method": method}
    if params is not None:
        msg["params"] = params
    proc.stdin.write(json.dumps(msg) + "\n")
    proc.stdin.flush()
    while True:
        line = proc.stdout.readline()
        if not line:
            sys.exit("server closed the connection")
        r = json.loads(line)
        if r.get("id") == next_id:
            if "error" in r:
                sys.exit(f"{method} failed: {r['error']}")
            return r["result"]

def tool(name, arguments):
    return call("tools/call", {"name": name, "arguments": arguments})

init = call("initialize", {"protocolVersion": "2025-06-18", "capabilities": {}, "clientInfo": {"name": "example", "version": "1"}})
print("server:", init["serverInfo"]["name"], init["serverInfo"]["version"], "protocol", init["protocolVersion"])
proc.stdin.write(json.dumps({"jsonrpc": "2.0", "method": "notifications/initialized"}) + "\n")
proc.stdin.flush()

names = [t["name"] for t in call("tools/list")["tools"]]
print("tools:", ", ".join(names))
assert "pixelox_run" in names and "pixelox_preview" in names

brief = tool("pixelox_describe", {})["content"][0]["text"]
print("briefing:", len(brief), "characters")

src, doc = os.path.join(work, "photo.png"), os.path.join(work, "photo.pxo")
r = tool("pixelox_run", {"command": "open", "params": {"input": src, "output": doc}})
assert r["structuredContent"]["ok"], r
plan = tool("pixelox_run", {"command": "adjust.curves", "params": {"input": doc, "in_place": True, "points": "0,0 64,56 192,206 255,255"}, "dry_run": True})
print("planned:", json.dumps(plan["structuredContent"]["result"]["plan"]["outputs_predicted"]))
r = tool("pixelox_run", {"command": "adjust.curves", "params": {"input": doc, "in_place": True, "points": "0,0 64,56 192,206 255,255"}})
assert r["structuredContent"]["ok"], r
out = os.path.join(work, "final.jpg")
r = tool("pixelox_run", {"command": "export", "params": {"input": doc, "output": out, "quality": 88}})
env = r["structuredContent"]
print("exported:", env["outputs"][0]["path"], env["outputs"][0]["sha256"][:12])

prev = tool("pixelox_preview", {"input": doc, "max_size": 256})
img = [c for c in prev["content"] if c["type"] == "image"][0]
print("preview:", img["mimeType"], len(img["data"]), "base64 characters")

r = tool("pixelox_run", {"command": "verify", "params": {"input": out + ".pixelox.json"}})
assert r["structuredContent"]["ok"], r
print("verified:", r["structuredContent"]["result"]["verified"])

r = tool("pixelox_run", {"command": "info", "params": {"input": "/etc/hosts"}})
assert r.get("isError"), r
print("sandbox:", r["structuredContent"]["error"]["code"] if "structuredContent" in r else r["content"][0]["text"][:60])

proc.stdin.close()
proc.wait(timeout=10)
