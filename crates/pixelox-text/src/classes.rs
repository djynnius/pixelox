//! Font classes: `--font @condensed-heavy`, `@didone`, `@script-formal`, ... (ADR 0015).
//!
//! The 28 classes come from the design skill's typography survey of 126 posters. Each has an
//! ordered candidate list: macOS, Windows, and common Linux families, then the curated Google
//! Fonts families (`gfonts`). A candidate is either a *member* (a real face of the class) or a
//! *stand-in* (the nearest thing a platform ships, e.g. DIN Condensed for ultra-compressed
//! type). A class resolves to the first installed member, then to the first installed
//! stand-in; failing both, to a related class, then to the installed face whose measured
//! width and darkness are closest to the class's target (`metrics`).
//!
//! The same table classifies family names, so a missing `--font "Bebas Neue"` is replaced by
//! the best installed face of its class (`@condensed-heavy`) instead of failing.

use crate::fonts::FontStyle;

/// The broad family a class belongs to; substitution never crosses genres unless nothing of
/// the same genre is installed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Genre {
    Sans,
    Serif,
    Script,
    Mono,
    Display,
}

impl Genre {
    pub fn name(self) -> &'static str {
        match self {
            Genre::Sans => "sans",
            Genre::Serif => "serif",
            Genre::Script => "script",
            Genre::Mono => "mono",
            Genre::Display => "display",
        }
    }

    /// The generic family a genre falls back to last.
    pub fn generic(self) -> &'static str {
        match self {
            Genre::Serif => "serif",
            Genre::Mono => "monospace",
            _ => "sans-serif",
        }
    }
}

/// One candidate family with the weight, width (CSS keyword 1..9, 5 = normal), and style
/// that make it look like the class.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cand {
    pub family: &'static str,
    pub weight: u16,
    pub stretch: u8,
    pub italic: bool,
    /// A real member of the class (false: the nearest stand-in a platform ships).
    pub member: bool,
}

const fn m(family: &'static str, weight: u16, stretch: u8) -> Cand {
    Cand { family, weight, stretch, italic: false, member: true }
}
const fn mi(family: &'static str, weight: u16) -> Cand {
    Cand { family, weight, stretch: 5, italic: true, member: true }
}
const fn s(family: &'static str, weight: u16, stretch: u8) -> Cand {
    Cand { family, weight, stretch, italic: false, member: false }
}
const fn si(family: &'static str, weight: u16) -> Cand {
    Cand { family, weight, stretch: 5, italic: true, member: false }
}

impl Cand {
    pub fn style(&self) -> FontStyle {
        if self.italic {
            FontStyle::Italic
        } else {
            FontStyle::Normal
        }
    }
}

/// A font class.
#[derive(Debug)]
pub struct FontClass {
    /// `condensed-heavy`; used as `--font @condensed-heavy`.
    pub id: &'static str,
    pub title: &'static str,
    pub uses: &'static str,
    pub genre: Genre,
    pub macos: &'static [Cand],
    pub windows: &'static [Cand],
    pub linux: &'static [Cand],
    /// Curated Google Fonts families (downloadable with `pixelox fonts get`).
    pub google: &'static [Cand],
    /// Classes to try next when nothing of this class is installed.
    pub related: &'static [&'static str],
    /// Target metrics (cap-advance width / cap height, ink darkness), measured on the first
    /// Google family; used to rank installed faces when no class member is available.
    pub target: (f32, f32),
}

impl FontClass {
    /// Every candidate in resolution order: system families (macOS, Windows, Linux), then
    /// Google Fonts families.
    pub fn candidates(&self) -> impl Iterator<Item = &'static Cand> {
        self.macos.iter().chain(self.windows.iter()).chain(self.linux.iter()).chain(self.google.iter())
    }

    pub fn alias(&self) -> String {
        format!("@{}", self.id)
    }
}

pub static CLASSES: &[FontClass] = &[
    FontClass {
        id: "condensed-heavy",
        title: "Ultra-condensed heavy grotesque",
        uses: "tall, tight, loud: event flyers, film titles, type blocks",
        genre: Genre::Sans,
        macos: &[
            m("Impact", 400, 3),
            m("DIN Condensed", 700, 3),
            m("Helvetica Neue", 900, 3),
            m("Avenir Next Condensed", 900, 3),
            m("Futura", 800, 3),
        ],
        windows: &[m("Bahnschrift", 700, 3), m("Arial Narrow", 700, 3)],
        linux: &[m("Liberation Sans Narrow", 700, 3), m("DejaVu Sans Condensed", 700, 3), m("DejaVu Sans", 700, 3)],
        google: &[m("Anton", 400, 5), m("League Gothic", 400, 5), m("Bebas Neue", 400, 5), m("Oswald", 700, 5)],
        related: &["compressed", "condensed", "heavy-wide"],
        target: (0.542, 0.692),
    },
    FontClass {
        id: "compressed",
        title: "Ultra-compressed display (narrower than Impact)",
        uses: "skyscraper letters that fill a tall column",
        genre: Genre::Sans,
        macos: &[s("DIN Condensed", 700, 3), s("Impact", 400, 3)],
        windows: &[s("Bahnschrift", 700, 1), s("Impact", 400, 3)],
        linux: &[s("Liberation Sans Narrow", 700, 3)],
        google: &[m("Six Caps", 400, 5), m("Antonio", 700, 5), m("Big Shoulders Display", 900, 5)],
        related: &["condensed-heavy", "condensed"],
        target: (0.273, 0.483),
    },
    FontClass {
        id: "condensed",
        title: "Condensed gothic, regular weight",
        uses: "sub-heads, dates, billing blocks, labels",
        genre: Genre::Sans,
        macos: &[m("Avenir Next Condensed", 500, 3), m("Arial Narrow", 400, 3), m("Futura", 500, 3)],
        windows: &[m("Bahnschrift", 400, 3), m("Arial Narrow", 400, 3)],
        linux: &[
            m("Liberation Sans Narrow", 400, 3),
            m("DejaVu Sans Condensed", 400, 3),
            m("Ubuntu Condensed", 400, 5),
        ],
        google: &[m("Oswald", 400, 5), m("Barlow Condensed", 500, 5), m("Roboto", 400, 3)],
        related: &["condensed-heavy", "neo-grotesque"],
        target: (0.604, 0.430),
    },
    FontClass {
        id: "condensed-hairline",
        title: "Hairline condensed caps",
        uses: "elegant tall thin titles, fashion",
        genre: Genre::Sans,
        macos: &[m("Avenir Next Condensed", 275, 3), s("Helvetica Neue", 100, 5)],
        windows: &[m("Bahnschrift", 300, 3)],
        linux: &[s("Liberation Sans Narrow", 400, 3)],
        google: &[m("Antonio", 100, 5), m("Big Shoulders Display", 100, 5), m("Oswald", 200, 5)],
        related: &["condensed", "thin-sans"],
        target: (0.473, 0.343),
    },
    FontClass {
        id: "heavy-wide",
        title: "Heavy wide grotesque",
        uses: "chunky, confident: sale, sport, protest",
        genre: Genre::Sans,
        macos: &[m("Arial Black", 900, 5), m("Avenir Next", 900, 5)],
        windows: &[m("Arial Black", 900, 5), m("Segoe UI Black", 900, 5), m("Segoe UI", 900, 5)],
        linux: &[m("Noto Sans", 900, 5), m("DejaVu Sans", 700, 5), s("Liberation Sans", 700, 5)],
        google: &[m("Archivo Black", 400, 5), m("Inter", 900, 5)],
        related: &["extended", "condensed-heavy", "neo-grotesque"],
        target: (1.109, 0.612),
    },
    FontClass {
        id: "extended",
        title: "Extended grotesque, upright or italic",
        uses: "wide and fast: cars, sport, youth church, tech",
        genre: Genre::Sans,
        macos: &[s("Arial Black", 900, 5)],
        windows: &[s("Arial Black", 900, 5)],
        linux: &[s("DejaVu Sans", 700, 5)],
        google: &[m("Archivo", 800, 7), m("Unbounded", 700, 5)],
        related: &["heavy-wide", "techno"],
        target: (1.315, 0.541),
    },
    FontClass {
        id: "neo-grotesque",
        title: "Neo-grotesque",
        uses: "neutral, Swiss, corporate",
        genre: Genre::Sans,
        macos: &[m("Helvetica Neue", 400, 5), m("Helvetica", 400, 5), m("Arial", 400, 5)],
        windows: &[m("Arial", 400, 5), m("Segoe UI", 400, 5)],
        linux: &[m("Liberation Sans", 400, 5), m("DejaVu Sans", 400, 5), m("Noto Sans", 400, 5)],
        google: &[m("Inter", 400, 5), m("Roboto", 400, 5), m("Archivo", 400, 5)],
        related: &["geometric-sans"],
        target: (0.924, 0.324),
    },
    FontClass {
        id: "geometric-sans",
        title: "Geometric sans",
        uses: "round, modern, friendly tech",
        genre: Genre::Sans,
        macos: &[m("Avenir Next", 500, 5), m("Futura", 500, 5), m("Avenir", 500, 5)],
        windows: &[m("Century Gothic", 400, 5), s("Segoe UI", 400, 5)],
        linux: &[s("Ubuntu", 400, 5), s("Cantarell", 400, 5), s("Noto Sans", 400, 5)],
        google: &[m("Montserrat", 500, 5), m("Poppins", 500, 5), m("Jost", 500, 5)],
        related: &["neo-grotesque"],
        target: (1.022, 0.327),
    },
    FontClass {
        id: "thin-sans",
        title: "Thin wide sans",
        uses: "airy, premium tech, fashion",
        genre: Genre::Sans,
        macos: &[m("Avenir Next", 275, 5), m("Helvetica Neue", 200, 5)],
        windows: &[m("Segoe UI Light", 300, 5), m("Segoe UI", 300, 5)],
        linux: &[m("Ubuntu Light", 300, 5), m("Ubuntu", 300, 5), s("Noto Sans", 300, 5)],
        google: &[m("Montserrat", 200, 5), m("Raleway", 200, 5), m("Jost", 200, 5)],
        related: &["rounded-thin", "condensed-hairline", "geometric-sans"],
        target: (1.007, 0.121),
    },
    FontClass {
        id: "rounded-thin",
        title: "Thin monoline rounded sans",
        uses: "soft, friendly, wellness, kids tech",
        genre: Genre::Sans,
        macos: &[m("SF Pro Rounded", 300, 5), s("Avenir Next", 275, 5)],
        windows: &[s("Segoe UI", 300, 5)],
        linux: &[s("Ubuntu", 300, 5)],
        google: &[m("Quicksand", 400, 5), m("Comfortaa", 300, 5), m("Nunito", 200, 5)],
        related: &["thin-sans", "geometric-sans"],
        target: (0.917, 0.231),
    },
    FontClass {
        id: "rounded-heavy",
        title: "Heavy rounded grotesque",
        uses: "food, kids, playful retail, chunky statements",
        genre: Genre::Sans,
        macos: &[m("Arial Rounded MT Bold", 400, 5), m("SF Pro Rounded", 800, 5)],
        windows: &[m("Arial Rounded MT Bold", 400, 5), s("Segoe UI Black", 900, 5)],
        linux: &[s("Ubuntu", 700, 5)],
        google: &[m("Fredoka", 700, 5), m("Nunito", 900, 5), m("Lilita One", 400, 5)],
        related: &["heavy-wide", "fat-display"],
        target: (0.909, 0.602),
    },
    FontClass {
        id: "techno",
        title: "Squared techno sans",
        uses: "sci-fi, gaming, fintech, devices",
        genre: Genre::Sans,
        macos: &[s("DIN Alternate", 700, 5)],
        windows: &[m("Agency FB", 700, 5), m("Bahnschrift", 600, 5)],
        linux: &[s("Ubuntu", 700, 5)],
        google: &[m("Orbitron", 700, 5), m("Rajdhani", 600, 5)],
        related: &["neo-grotesque", "extended"],
        target: (1.100, 0.462),
    },
    FontClass {
        id: "didone",
        title: "High-contrast didone",
        uses: "glamorous, editorial, mastheads",
        genre: Genre::Serif,
        macos: &[m("Didot", 700, 5), m("Bodoni 72", 700, 5)],
        windows: &[m("Bodoni MT", 700, 5), s("Georgia", 700, 5)],
        linux: &[m("Noto Serif Display", 700, 5), s("DejaVu Serif", 700, 5)],
        google: &[
            m("Playfair Display", 700, 5),
            m("Bodoni Moda", 700, 5),
            m("DM Serif Display", 400, 5),
            m("Abril Fatface", 400, 5),
        ],
        related: &["didone-condensed", "oldstyle"],
        target: (0.962, 0.373),
    },
    FontClass {
        id: "didone-condensed",
        title: "Condensed didone masthead",
        uses: "tall magazine nameplates",
        genre: Genre::Serif,
        macos: &[s("Didot", 700, 5), s("Big Caslon", 500, 5)],
        windows: &[m("Bodoni MT Condensed", 700, 3), s("Bodoni MT", 700, 5)],
        linux: &[m("DejaVu Serif Condensed", 700, 3)],
        google: &[m("Rozha One", 400, 5), s("Bodoni Moda", 800, 5)],
        related: &["didone", "oldstyle"],
        target: (0.750, 0.420),
    },
    FontClass {
        id: "oldstyle",
        title: "Old-style / transitional serif",
        uses: "literary, calm, book covers",
        genre: Genre::Serif,
        macos: &[m("Baskerville", 400, 5), m("Hoefler Text", 400, 5), m("Palatino", 400, 5), m("Georgia", 400, 5)],
        windows: &[m("Georgia", 400, 5), m("Palatino Linotype", 400, 5), m("Cambria", 400, 5)],
        linux: &[m("Liberation Serif", 400, 5), m("Noto Serif", 400, 5), m("DejaVu Serif", 400, 5)],
        google: &[m("EB Garamond", 400, 5), m("Libre Baskerville", 400, 5), m("Lora", 400, 5)],
        related: &["didone", "engraved"],
        target: (1.008, 0.264),
    },
    FontClass {
        id: "swash-serif",
        title: "Swash and decorative serif",
        uses: "fantasy and romance titles, one ornate capital",
        genre: Genre::Serif,
        macos: &[m("Apple Chancery", 400, 5), si("Baskerville", 400)],
        windows: &[m("Gabriola", 400, 5), m("Monotype Corsiva", 400, 5)],
        linux: &[si("Liberation Serif", 400)],
        google: &[m("Cinzel Decorative", 400, 5), mi("Playfair Display", 400)],
        related: &["engraved", "oldstyle", "script-formal"],
        target: (0.717, 0.156),
    },
    FontClass {
        id: "engraved",
        title: "Engraved / inscriptional caps",
        uses: "film credits, luxury, ceremony",
        genre: Genre::Serif,
        macos: &[m("Copperplate", 400, 5), m("Optima", 400, 5), m("Big Caslon", 500, 5)],
        windows: &[m("Copperplate Gothic Bold", 400, 5), m("Perpetua Titling MT", 400, 5)],
        linux: &[s("DejaVu Serif", 400, 5)],
        google: &[m("Cinzel", 400, 5), m("Marcellus", 400, 5)],
        related: &["oldstyle", "didone"],
        target: (0.978, 0.201),
    },
    FontClass {
        id: "uncial",
        title: "Uncial / medieval",
        uses: "fantasy, folklore, heritage",
        genre: Genre::Display,
        macos: &[m("Luminari", 400, 5), m("Herculanum", 400, 5)],
        windows: &[m("Viner Hand ITC", 400, 5), s("Old English Text MT", 400, 5)],
        linux: &[],
        google: &[m("Uncial Antiqua", 400, 5), m("MedievalSharp", 400, 5)],
        related: &["blackletter", "swash-serif", "engraved"],
        target: (0.855, 0.270),
    },
    FontClass {
        id: "blackletter",
        title: "Blackletter",
        uses: "gothic, metal, tradition, mastheads",
        genre: Genre::Display,
        macos: &[s("Luminari", 400, 5)],
        windows: &[m("Old English Text MT", 400, 5)],
        linux: &[],
        google: &[m("UnifrakturMaguntia", 400, 5), m("UnifrakturCook", 700, 5), m("Pirata One", 400, 5)],
        related: &["uncial", "engraved"],
        target: (0.888, 0.287),
    },
    FontClass {
        id: "slab",
        title: "Slab serif",
        uses: "sturdy, retro, western, editorial",
        genre: Genre::Serif,
        macos: &[m("Rockwell", 700, 5), m("American Typewriter", 400, 5), m("Superclarendon", 400, 5)],
        windows: &[m("Rockwell", 700, 5), s("Cambria", 700, 5), s("Courier New", 700, 5)],
        linux: &[s("DejaVu Serif", 700, 5), s("Noto Serif", 700, 5)],
        google: &[m("Roboto Slab", 700, 5), m("Zilla Slab", 700, 5), m("Alfa Slab One", 400, 5)],
        related: &["fat-display", "oldstyle"],
        target: (0.978, 0.488),
    },
    FontClass {
        id: "fat-display",
        title: "Fat display: chunky slab, cut-paper, Cooper style",
        uses: "1970s warmth, food, soul and funk posters, hand-cut lettering",
        genre: Genre::Display,
        macos: &[m("Superclarendon", 900, 5), s("Arial Rounded MT Bold", 400, 5)],
        windows: &[m("Cooper Black", 400, 5), m("Rockwell Extra Bold", 800, 5)],
        linux: &[s("DejaVu Serif", 700, 5)],
        google: &[m("Ultra", 400, 5), m("Titan One", 400, 5), m("Shrikhand", 400, 5)],
        related: &["slab", "rounded-heavy", "heavy-wide"],
        target: (1.150, 0.725),
    },
    FontClass {
        id: "script-formal",
        title: "Formal script",
        uses: "weddings, worship, elegance",
        genre: Genre::Script,
        macos: &[m("Snell Roundhand", 700, 5), m("Zapfino", 400, 5)],
        windows: &[m("Edwardian Script ITC", 400, 5), m("Segoe Script", 400, 5)],
        linux: &[],
        google: &[m("Great Vibes", 400, 5), m("Allura", 400, 5)],
        related: &["script-signature", "brush", "swash-serif"],
        target: (0.991, 0.175),
    },
    FontClass {
        id: "script-signature",
        title: "Hairline and signature script",
        uses: "thin autograph accents, fashion, beauty",
        genre: Genre::Script,
        macos: &[m("Savoye LET", 400, 5), m("Snell Roundhand", 500, 5), s("Bradley Hand", 700, 5)],
        windows: &[m("Lucida Handwriting", 400, 5), m("Freestyle Script", 400, 5)],
        linux: &[],
        google: &[m("Mrs Saint Delafield", 400, 5), m("Sacramento", 400, 5)],
        related: &["script-formal", "marker"],
        target: (0.672, 0.182),
    },
    FontClass {
        id: "brush",
        title: "Brush script and heavy script wordmarks",
        uses: "energy, handmade, lifestyle, retro soda marks",
        genre: Genre::Script,
        macos: &[m("SignPainter", 400, 5), m("Brush Script MT", 400, 5)],
        windows: &[m("Brush Script MT", 400, 5), s("Segoe Script", 700, 5)],
        linux: &[],
        google: &[m("Pacifico", 400, 5), m("Yellowtail", 400, 5), m("Lobster", 400, 5)],
        related: &["script-formal", "marker"],
        target: (0.908, 0.439),
    },
    FontClass {
        id: "marker",
        title: "Marker and handwriting, dry-brush caps",
        uses: "notes, street, zines, shouted captions",
        genre: Genre::Script,
        macos: &[
            m("Marker Felt", 400, 5),
            m("Noteworthy", 400, 5),
            m("Bradley Hand", 700, 5),
            m("Chalkduster", 400, 5),
        ],
        windows: &[m("Ink Free", 400, 5), m("Segoe Print", 400, 5)],
        linux: &[],
        google: &[m("Permanent Marker", 400, 5), m("Caveat", 500, 5), m("Amatic SC", 700, 5)],
        related: &["brush", "script-signature"],
        target: (0.893, 0.522),
    },
    FontClass {
        id: "stencil",
        title: "Stencil",
        uses: "military, street, industrial",
        genre: Genre::Display,
        macos: &[m("Stencil", 400, 5), m("Stencil Std", 700, 5)],
        windows: &[m("Stencil", 400, 5)],
        linux: &[],
        google: &[m("Allerta Stencil", 400, 5), m("Stardos Stencil", 700, 5), m("Saira Stencil One", 400, 5)],
        related: &["condensed-heavy", "slab"],
        target: (0.915, 0.359),
    },
    FontClass {
        id: "mono",
        title: "Monospace",
        uses: "code, tech, receipts, captions",
        genre: Genre::Mono,
        macos: &[m("Menlo", 400, 5), m("SF Mono", 400, 5), m("Monaco", 400, 5), m("Courier New", 400, 5)],
        windows: &[m("Consolas", 400, 5), m("Cascadia Mono", 400, 5), m("Courier New", 400, 5)],
        linux: &[
            m("DejaVu Sans Mono", 400, 5),
            m("Liberation Mono", 400, 5),
            m("Noto Sans Mono", 400, 5),
            m("Ubuntu Mono", 400, 5),
        ],
        google: &[m("JetBrains Mono", 400, 5), m("Space Mono", 400, 5)],
        related: &[],
        target: (0.822, 0.331),
    },
    FontClass {
        id: "deco",
        title: "Display art-deco, unicase, retro",
        uses: "1920s to 1970s posters, club nights",
        genre: Genre::Display,
        macos: &[m("Phosphate", 400, 5), s("Copperplate", 700, 5), s("Futura", 700, 5)],
        windows: &[m("Broadway", 400, 5), m("Gill Sans Ultra Bold", 400, 5)],
        linux: &[],
        google: &[m("Limelight", 400, 5), m("Monoton", 400, 5), m("Righteous", 400, 5)],
        related: &["engraved", "geometric-sans", "fat-display"],
        target: (0.975, 0.416),
    },
];

/// The class named `id` (with or without the leading `@`; case and punctuation ignored).
pub fn find(id: &str) -> Option<&'static FontClass> {
    let n = crate::fonts::norm(id.trim().trim_start_matches('@'));
    CLASSES.iter().find(|c| crate::fonts::norm(c.id) == n).or_else(|| {
        ALIASES
            .iter()
            .find(|(a, _)| crate::fonts::norm(a) == n)
            .and_then(|(_, id)| CLASSES.iter().find(|c| c.id == *id))
    })
}

/// Extra spellings accepted after `@`.
const ALIASES: &[(&str, &str)] = &[
    ("impact", "condensed-heavy"),
    ("heavy-condensed", "condensed-heavy"),
    ("ultra-condensed", "condensed-heavy"),
    ("ultra-compressed", "compressed"),
    ("condensed-regular", "condensed"),
    ("hairline", "condensed-hairline"),
    ("heavy", "heavy-wide"),
    ("wide", "extended"),
    ("grotesque", "neo-grotesque"),
    ("sans", "neo-grotesque"),
    ("swiss", "neo-grotesque"),
    ("geometric", "geometric-sans"),
    ("thin", "thin-sans"),
    ("rounded", "rounded-heavy"),
    ("bodoni", "didone"),
    ("didot", "didone"),
    ("masthead", "didone-condensed"),
    ("serif", "oldstyle"),
    ("book", "oldstyle"),
    ("transitional", "oldstyle"),
    ("swash", "swash-serif"),
    ("inscriptional", "engraved"),
    ("medieval", "uncial"),
    ("fraktur", "blackletter"),
    ("slab-serif", "slab"),
    ("cooper", "fat-display"),
    ("script", "script-formal"),
    ("signature", "script-signature"),
    ("brush-script", "brush"),
    ("handwriting", "marker"),
    ("monospace", "mono"),
    ("art-deco", "deco"),
    ("retro", "deco"),
];

/// Is `family` a class reference (`@...`)?
pub fn is_class_ref(family: &str) -> bool {
    family.trim_start().starts_with('@')
}

/// The class a named family belongs to: the first class listing it (members before
/// stand-ins), then hints in the name (`Mono`, `Slab`, `Script`, `Condensed`, ...).
pub fn class_of_family(family: &str) -> Option<(&'static FontClass, &'static Cand)> {
    let n = crate::fonts::norm(family);
    for want_member in [true, false] {
        for c in CLASSES {
            if let Some(cand) = c.candidates().find(|k| k.member == want_member && crate::fonts::norm(k.family) == n) {
                return Some((c, cand));
            }
        }
    }
    None
}

/// A class guessed from words in an unknown family name.
pub fn guess_class(family: &str) -> &'static FontClass {
    let n = family.to_ascii_lowercase();
    let has = |w: &str| n.contains(w);
    let id = if has("mono") || has("code") || has("courier") || has("typewriter") {
        "mono"
    } else if has("stencil") {
        "stencil"
    } else if has("fraktur") || has("blackletter") || has("textura") {
        "blackletter"
    } else if has("uncial") || has("medieval") {
        "uncial"
    } else if has("slab") {
        "slab"
    } else if has("didot") || has("bodoni") || has("didone") {
        "didone"
    } else if has("brush") {
        "brush"
    } else if has("marker") || has("hand") || has("chalk") {
        "marker"
    } else if has("script") || has("signature") || has("calligraph") {
        "script-formal"
    } else if has("round") {
        "rounded-heavy"
    } else if (has("condensed") || has("narrow") || has("compressed"))
        && (has("black") || has("heavy") || has("bold") || has("extra"))
    {
        "condensed-heavy"
    } else if has("compressed") {
        "compressed"
    } else if has("condensed") || has("narrow") {
        "condensed"
    } else if has("extended") || has("expanded") || has("wide") {
        "extended"
    } else if has("thin") || has("hairline") || has("light") {
        "thin-sans"
    } else if has("black") || has("heavy") {
        "heavy-wide"
    } else if has("deco") {
        "deco"
    } else if (has("serif") && !has("sans")) || has("garamond") || has("caslon") || has("roman") || has("book") {
        "oldstyle"
    } else {
        "neo-grotesque"
    };
    find(id).unwrap_or(&CLASSES[6])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes_are_well_formed() {
        assert_eq!(CLASSES.len(), 28);
        let mut ids: Vec<&str> = CLASSES.iter().map(|c| c.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), 28, "class ids are unique");
        for c in CLASSES {
            assert!(!c.google.is_empty(), "{} has a Google family", c.id);
            assert!(c.google.iter().any(|g| g.member), "{} has a downloadable member", c.id);
            for r in c.related {
                assert!(find(r).is_some(), "{} relates to unknown {r}", c.id);
            }
            for g in c.google {
                assert!(crate::gfonts::curated(g.family).is_some(), "{}: {} is not in the curated set", c.id, g.family);
            }
            assert!(c.target.0 > 0.0 && c.target.1 > 0.0, "{} has target metrics", c.id);
            for k in c.candidates() {
                assert!((1..=9).contains(&k.stretch) && (1..=1000).contains(&k.weight), "{}: {}", c.id, k.family);
            }
        }
        for (a, id) in ALIASES {
            assert!(find(id).is_some(), "alias {a} -> {id}");
        }
    }

    #[test]
    fn find_and_classify() {
        assert_eq!(find("@condensed-heavy").unwrap().id, "condensed-heavy");
        assert_eq!(find("@Didone").unwrap().id, "didone");
        assert_eq!(find("@monospace").unwrap().id, "mono");
        assert!(find("@nope").is_none());
        assert_eq!(class_of_family("Impact").unwrap().0.id, "condensed-heavy");
        assert_eq!(class_of_family("bebas neue").unwrap().0.id, "condensed-heavy");
        assert_eq!(class_of_family("Great Vibes").unwrap().0.id, "script-formal");
        assert_eq!(guess_class("Foo Sans Mono").id, "mono");
        assert_eq!(guess_class("Acme Condensed Black").id, "condensed-heavy");
        assert_eq!(guess_class("Acme Serif Text").id, "oldstyle");
        assert_eq!(guess_class("Acme Grotesk").id, "neo-grotesque");
    }
}
