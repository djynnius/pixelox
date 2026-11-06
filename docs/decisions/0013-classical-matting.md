# ADR 0013: `mask refine` is classical matting (Photoshop's Extract, rebuilt)

Status: accepted (2026-11-06). Supersedes ADR 0006 for `mask refine`.

## Context
Photoshop 6/7/CS's Extract filter made near-perfect cutouts of hair and fur: the user painted
a band over the uncertain edge, filled the inside, and Extract solved the alpha in the band
and removed the background's color from the edge pixels. pixelox's `detect subject`
(BiRefNet at 1024 px) cuts subjects cleanly overall but, on a 12 MP portrait with curly hair
against a dark-blue flag, lost flyaway strands and left a blue fringe on 62% of the edge
pixels. `mask refine` existed only as a plan waiting for a ViTMatte model.

## Decision
* `mask refine` moves to pixelox-select and solves the edge without a model, from published
  methods (not Adobe's code):
  * **Trimap** from any rough mask (a `detect subject` mask, a selection, a traced path):
    foreground and background split at 0.5, unknown = within `--band` of the edge (exact
    distance transform) plus wherever the rough mask is soft, plus an optional painted band
    (`--unknown`, Extract's highlighter).
  * **Alpha**: closed-form matting (Levin, Lischinski, Weiss 2008; 3×3 windows, ε = 1e-4)
    with a confidence-weighted color term (robust-matting style, from pull-push spread known
    colors) and a weak pull toward the rough mask, solved coarse to fine with
    Jacobi-preconditioned conjugate gradients. ε = 1e-6 showed 8×8 JPEG-block artifacts;
    1e-3 smeared strands.
  * **Colors**: `--decontaminate` estimates foreground colors with fast multi-level
    foreground estimation (Germer et al. 2021) and writes them into the band, as Extract did.
  * `--force-foreground COLOR`: Extract's single-color option.
* `--model vitmatte` stays for a later learned refiner and exits 12 (PX1520).
* Deterministic: fixed iteration caps and tolerances, chunked sums combined in order, the
  auto band recorded at full precision for replay.

## Consequences
On the 12 MP portrait: blue-dominant edge pixels 62.4% → 0.2%, about 1,200 lost strand pixels
recovered, about 1–1.4 s and 0.6 GB extra memory. Limits: strands outside the band stay cut
(widen `--band` or paint `--unknown`); dark strands over a dark backdrop stay partly
transparent where the colors truly overlap; one color sample pair per pixel rather than
multi-candidate sampling.
