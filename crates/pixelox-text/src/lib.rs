//! Type layers (`text *`) and fonts (`fonts *`): shaping, layout, text on a path, text to path.
//!
//! * Fonts ([`fonts`]): fontdb over the system fonts, the folders registered with
//!   `fonts add`, the Google Fonts cache ([`gfonts`], downloaded with consent), and the
//!   embedded Tuffy fallback (public domain). Families match case-insensitively in CSS
//!   font-matching order (width, style, weight); `sans-serif`, `serif`, and `monospace` map to
//!   a system default and then to Tuffy. Font classes ([`classes`], `--font @didone`) resolve
//!   to the best installed face; a missing family is replaced by the closest installed face
//!   of its class (ADR 0015). `PIXELOX_SYSTEM_FONTS=0` skips the system fonts (fully
//!   reproducible output on any machine).
//! * Layout ([`layout`]): rustybuzz shaping (kerning, ligatures, variable axes), unicode-bidi
//!   reordering, unicode-linebreak wrapping inside `--box`, alignment and justification,
//!   tracking, leading, rotation, and text on a path.
//! * Rendering ([`render`]): unhinted glyph outlines filled anti-aliased with tiny-skia, an
//!   optional centered stroke, synthetic bold (stroke) and oblique (skew), faux bold.
//! * Warps ([`warp`]): Photoshop's warp presets applied to the glyph outlines.
//!
//! ## The text layer spec
//!
//! `LayerKind::Text { spec }` holds (keys shared with other crates marked *):
//!
//! ```json
//! {"content": "Hello"*, "font": "Inter"*, "font_sha256": "…", "font_face": {"family": "Inter",
//!  "style": "normal", "weight": 800, "sha256": "…", "postscript": "Inter-ExtraBold"},
//!  "size": "54pt"*, "weight": 800, "style": "normal", "variation": "wght=650,slnt=-5" | null,
//!  "color": "#111111"*, "stroke": "#ffffff" | null, "stroke_width": 2, "align": "left",
//!  "tracking": 60, "leading": "auto" | "120%" | "60px", "kerning": "metrics" | "none",
//!  "x": 500*, "y": 120*, "anchor": "t"*, "box": [600, 400] | [600, null] | null,
//!  "rotation": 0*, "path": "M …"* | null, "path_offset": "0" | "25%", "path_side": "outside",
//!  // written only when set (0.5):
//!  "stretch": 75, "case": "upper", "scale_x": 0.8*, "scale_y": 1.0*, "faux_bold": 20,
//!  "fit_width": 920* | "fit_box": [920, 300]*, "anchor_to": "ink", "path_fit": "shrink",
//!  "warp": {"preset": "arc", "bend": 50, "h_distort": 0, "v_distort": 0, "orient": "horizontal"}}
//! ```
//!
//! `weight` is `null` when automatic (400, or the weight of the class candidate for
//! `"font": "@condensed-heavy"`). `scale_x`/`scale_y` scale the set text about the anchor
//! (box width and leading too), so `transform scale` with unequal factors keeps text live.
//!
//! `x`/`y` are the anchor point in canvas px; `anchor` is which point of the text box sits
//! there (the box is `--box`, or the lines' extent: widest line × line count). `tracking`
//! is in 1/1000 em; `rotation` is degrees clockwise about the anchor point. With `path`, the
//! text follows it and `x`/`y`/`anchor`/`rotation` are ignored.

#![allow(clippy::too_many_arguments)]

pub mod classes;
pub mod fonts;
pub mod gfonts;
pub mod layout;
pub mod metrics;
mod ops;
pub mod render;
pub mod spec;
pub mod warp;

#[cfg(test)]
mod tests;

use pixelox_core::prelude::*;

/// Every command this crate defines.
pub fn ops() -> Vec<OpSpec> {
    let mut v = ops::text_ops();
    v.push(
        OpSpec::custom("fonts list", Category::Models, "List installed and registered fonts", fonts::fonts_list)
            .inputs(Inputs::NONE)
            .about(
                "Lists font families (system fonts, folders registered with `fonts add`, Google Fonts downloaded with \
                 `fonts get`, and the embedded Tuffy fallback) with their class (`@condensed-heavy`, ...), styles, \
                 weights, and files; the registered folders; what the generic names sans-serif, serif, and monospace \
                 resolve to; and the glyph-fallback chain. --family narrows the list (case-insensitive substring) and \
                 suggests close names, or the download command, when nothing matches; --class lists one class.",
            )
            .param(param("family", ParamKind::Str).help("Only families whose name contains this (case-insensitive)"))
            .param(param("class", ParamKind::Str).help("Only families listed in this font class, e.g. @didone"))
            .example("pixelox fonts list", "Every family on this machine")
            .example("pixelox fonts list --class @condensed-heavy --json", "Installed families of one class")
            .example("pixelox fonts list --family Inter --json", "Faces and files of one family")
            .example("pixelox fonts list --family \"Noto Sans\"", "All Noto Sans families"),
    );
    v.push(
        OpSpec::custom("fonts add", Category::Models, "Register a folder of fonts", fonts::fonts_add)
            .inputs(Inputs::NONE)
            .positional("dir")
            .about(
                "Adds a folder to the font search path (one folder per line in the fonts file under the config \
                 directory), so its families can be used with --font. The folder is scanned and the families found \
                 are reported; --dry-run reports without registering.",
            )
            .param(param("dir", ParamKind::Path).required().help("Folder holding .ttf, .otf, .ttc, or .otc files"))
            .example("pixelox fonts add ~/fonts", "Register a personal font folder")
            .example("pixelox fonts add ./brand/fonts --json", "Register a project's fonts")
            .example("pixelox fonts add /usr/share/fonts/custom --dry-run --json", "See what would be registered"),
    );
    v.push(
        OpSpec::custom(
            "fonts classes",
            Category::Models,
            "List font classes and what each resolves to",
            fonts::fonts_classes,
        )
        .inputs(Inputs::NONE)
        .positional("class")
        .about(
            "The 28 font classes usable as --font @class (condensed-heavy, didone, script-formal, brush, slab, \
                 stencil, blackletter, geometric-sans, mono, ...): what each looks like, its candidate families on \
                 macOS, Windows, and Linux and in Google Fonts, and the face it resolves to on this machine (member, \
                 stand-in, or substitute). A class without an installed member names the `pixelox fonts get @class -y` \
                 command that downloads its best free family.",
        )
        .param(param("class", ParamKind::Str).help("Show only this class, e.g. @didone"))
        .example("pixelox fonts classes", "Every class and what it resolves to here")
        .example("pixelox fonts classes @stencil --json", "One class with its candidates")
        .example("pixelox fonts classes --json", "Machine-readable, for choosing fonts in a script"),
    );
    v.push(
        OpSpec::custom("fonts get", Category::Models, "Download a Google Fonts family (consent: -y)", fonts::fonts_get)
            .inputs(Inputs::NONE)
            .positional("family")
            .about(
                "Downloads an open-licensed family from Google Fonts into the font cache (data directory), with its \
                 license file, and registers it at once. About 60 curated families covering every font class are \
                 pinned to one google/fonts commit with the SHA-256 of every file (verified, PX1541 on a mismatch); any \
                 other family of that commit can be fetched by name and its hashes are recorded. A class (@stencil) \
                 fetches its best free family; --all fetches the whole curated set. Needs consent: -y on the command \
                 line (PX1540, exit 6, otherwise), consent [\"font-download\"] over MCP; PIXELOX_OFFLINE=1 forbids the \
                 network. Files copied into the cache by hand are adopted when their hashes match.",
            )
            .param(param("family", ParamKind::Str).repeatable().help("Family name (\"Bebas Neue\") or font class (@stencil); repeatable"))
            .param(param("all", ParamKind::Flag).help("Every curated family (about 20 MB)"))
            .param(param("class-all", ParamKind::Flag).help("With a class: every Google family of the class, not just the first"))
            .example("pixelox fonts get \"Bebas Neue\" -y", "Download one family")
            .example("pixelox fonts get @stencil @blackletter -y --json", "The best free family of two classes")
            .example("pixelox fonts get Anton --dry-run --json", "What would be downloaded, without consent or network")
            .example("pixelox fonts get --all -y", "The whole curated set, for an offline machine or CI"),
    );
    v
}

/// Install the text renderer into the compositor.
pub fn install() {
    compose::set_text_renderer(render::render_text_layer);
}
