//! Type layers (`text *`) and fonts (`fonts *`): shaping, layout, text on a path, text to path.
//!
//! * Fonts ([`fonts`]): fontdb over the system fonts, the folders registered with
//!   `fonts add`, and the embedded Tuffy fallback (public domain). Families match
//!   case-insensitively with CSS-style weight/style selection; `sans-serif`, `serif`, and
//!   `monospace` map to a system default and then to Tuffy. `PIXELOX_SYSTEM_FONTS=0` skips
//!   the system fonts (fully reproducible output on any machine).
//! * Layout ([`layout`]): rustybuzz shaping (kerning, ligatures, variable axes), unicode-bidi
//!   reordering, unicode-linebreak wrapping inside `--box`, alignment and justification,
//!   tracking, leading, rotation, and text on a path.
//! * Rendering ([`render`]): unhinted glyph outlines filled anti-aliased with tiny-skia, an
//!   optional centered stroke, synthetic bold (stroke) and oblique (skew).
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
//!  "rotation": 0*, "path": "M …"* | null, "path_offset": "0" | "25%", "path_side": "outside"}
//! ```
//!
//! `x`/`y` are the anchor point in canvas px; `anchor` is which point of the text box sits
//! there (the box is `--box`, or the lines' extent: widest line × line count). `tracking`
//! is in 1/1000 em; `rotation` is degrees clockwise about the anchor point. With `path`, the
//! text follows it and `x`/`y`/`anchor`/`rotation` are ignored.

#![allow(clippy::too_many_arguments)]

pub mod fonts;
pub mod layout;
mod ops;
pub mod render;
pub mod spec;

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
                "Lists font families (system fonts, folders registered with `fonts add`, and the embedded Tuffy \
                 fallback) with their styles, weights, and files; the registered folders; what the generic names \
                 sans-serif, serif, and monospace resolve to; and the glyph-fallback chain. --family narrows the list \
                 (case-insensitive substring) and suggests close names when nothing matches.",
            )
            .param(param("family", ParamKind::Str).help("Only families whose name contains this (case-insensitive)"))
            .example("pixelox fonts list", "Every family on this machine")
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
    v
}

/// Install the text renderer into the compositor.
pub fn install() {
    compose::set_text_renderer(render::render_text_layer);
}
