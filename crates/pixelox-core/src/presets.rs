//! Canvas presets (PRD §8.14), shared by `new --preset` and `comp presets`.

/// `(name, width, height, dpi, description)`.
pub const CANVAS_PRESETS: &[(&str, u32, u32, f32, &str)] = &[
    ("a4", 2480, 3508, 300.0, "ISO A4 portrait, print"),
    ("a3", 3508, 4961, 300.0, "ISO A3 portrait, print"),
    ("letter", 2550, 3300, 300.0, "US Letter portrait, print"),
    ("business-card", 1050, 600, 300.0, "3.5 x 2 in business card, print"),
    ("poster-18x24", 2700, 3600, 150.0, "18 x 24 in poster"),
    ("flier-letter", 2550, 3300, 300.0, "US Letter flier, print"),
    ("ig-post", 1080, 1080, 72.0, "Instagram square post"),
    ("ig-portrait", 1080, 1350, 72.0, "Instagram 4:5 portrait post"),
    ("ig-story", 1080, 1920, 72.0, "Instagram / TikTok story 9:16"),
    ("x-post", 1600, 900, 72.0, "X (Twitter) post image 16:9"),
    ("yt-thumb", 1280, 720, 72.0, "YouTube thumbnail"),
    ("og-card", 1200, 630, 72.0, "Open Graph link card"),
    ("fb-cover", 820, 312, 72.0, "Facebook page cover"),
];

pub fn find(name: &str) -> Option<(u32, u32, f32)> {
    CANVAS_PRESETS.iter().find(|p| p.0 == name).map(|p| (p.1, p.2, p.3))
}

pub fn names() -> Vec<&'static str> {
    CANVAS_PRESETS.iter().map(|p| p.0).collect()
}
