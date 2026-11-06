//! Filters (`filter *`), retouching (`retouch *`), fills (`fill *`), and painting
//! (`paint stroke`) for pixelox.
//!
//! Every command is a region op on a raster layer: `--layer` picks the layer (the only
//! one by default) and `--mask` / `--invert-mask` limit the effect (`paint stroke` can also
//! paint layer masks, the selection, and named masks). Two retouching commands build on
//! the others: `retouch frequency-split` adds a Low/High group above a layer, and
//! `retouch blemishes` finds spots (and heals them with the `retouch heal` machinery).
//! Kernels are O(pixels) per op; the few that are not radius-independent document their
//! bounds (`util.rs`). `fill content-aware` (inpainting) lives in pixelox-ml.

#![allow(clippy::needless_range_loop, clippy::too_many_arguments)]

use pixelox_core::prelude::*;

mod art;
mod blemish;
mod blur;
mod fill;
mod freqsep;
mod kern;
mod noise;
mod paint;
mod retouch;
mod sharpen;
mod stylize;
pub mod util;

/// Every command this crate defines.
pub fn ops() -> Vec<OpSpec> {
    let mut v = Vec::new();
    v.extend(blur::ops());
    v.extend(sharpen::ops());
    v.extend(noise::ops());
    v.extend(stylize::ops());
    v.extend(art::ops());
    v.extend(retouch::ops());
    v.extend(freqsep::ops());
    v.extend(blemish::ops());
    v.extend(fill::ops());
    v.extend(paint::ops());
    v
}

/// Install renderer hooks (none: this crate only defines commands).
pub fn install() {}

#[cfg(test)]
mod contract {
    use super::*;
    use std::collections::BTreeSet;

    const EXPECTED: &[&str] = &[
        "filter gaussian",
        "filter box",
        "filter motion-blur",
        "filter radial-blur",
        "filter lens-blur",
        "filter surface-blur",
        "filter median",
        "filter unsharp",
        "filter smart-sharpen",
        "filter add-noise",
        "filter denoise",
        "filter despeckle",
        "filter dust",
        "filter emboss",
        "filter find-edges",
        "filter high-pass",
        "filter pixelate",
        "filter vignette",
        "filter lens-correct",
        "filter pencil",
        "filter charcoal",
        "filter ink",
        "filter oil-paint",
        "filter watercolor",
        "filter cartoon",
        "filter cutout",
        "filter halftone",
        "retouch erase",
        "retouch heal",
        "retouch clone",
        "retouch dodge",
        "retouch burn",
        "retouch sharpen",
        "retouch blur",
        "retouch redeye",
        "retouch frequency-split",
        "retouch blemishes",
        "fill solid",
        "fill gradient",
        "fill pattern",
        "paint stroke",
    ];

    const GLOBALS: &[&str] = &[
        "output",
        "format",
        "json",
        "dry-run",
        "overwrite",
        "in-place",
        "seed",
        "yes",
        "quiet",
        "verbose",
        "threads",
        "mem-budget",
        "progress",
        "no-sidecar",
        "background",
        "config",
        "redact-paths",
        "limit-pixels",
    ];

    #[test]
    fn every_command_is_defined_once() {
        let names: Vec<&str> = ops().iter().map(|o| o.name).collect();
        let set: BTreeSet<&str> = names.iter().copied().collect();
        assert_eq!(set.len(), names.len(), "duplicate command");
        assert_eq!(set, EXPECTED.iter().copied().collect::<BTreeSet<_>>());
    }

    #[test]
    fn specs_follow_the_authoring_contract() {
        for op in ops() {
            assert!(op.summary.len() <= 60 && !op.summary.ends_with('.'), "{}: summary '{}'", op.name, op.summary);
            assert!(!op.about.is_empty(), "{}: about", op.name);
            assert!(op.examples.len() >= 3, "{}: examples", op.name);
            let mut seen = BTreeSet::new();
            for p in &op.params {
                assert!(!p.help.is_empty(), "{} --{}: help", op.name, p.name);
                assert!(!GLOBALS.contains(&p.name), "{} --{}: global flag name", op.name, p.name);
                assert!(seen.insert(p.name), "{} --{}: duplicate", op.name, p.name);
                if let Some(d) = p.default {
                    p.validate(d).unwrap_or_else(|e| panic!("{} --{} default '{d}': {}", op.name, p.name, e.message));
                }
            }
            for ex in &op.examples {
                let prefix = format!("pixelox {} ", op.name);
                assert!(ex.cmd.starts_with(&prefix), "{}: example '{}'", op.name, ex.cmd);
                check_example(&op, ex.cmd);
            }
        }
    }

    /// Split a command line like a POSIX shell (single and double quotes).
    fn shell_words(s: &str) -> Vec<String> {
        let mut out = Vec::new();
        let mut cur = String::new();
        let mut quote: Option<char> = None;
        let mut any = false;
        for c in s.chars() {
            match quote {
                Some(q) if c == q => quote = None,
                Some(_) => cur.push(c),
                None if c == '\'' || c == '"' => {
                    quote = Some(c);
                    any = true;
                }
                None if c.is_whitespace() => {
                    if any || !cur.is_empty() {
                        out.push(std::mem::take(&mut cur));
                    }
                    any = false;
                }
                None => cur.push(c),
            }
        }
        if any || !cur.is_empty() {
            out.push(cur);
        }
        out
    }

    /// Every `--flag value` of an example must be a param (or a global) with a valid value.
    fn check_example(op: &OpSpec, cmd: &str) {
        let words = shell_words(cmd);
        let skip = 1 + op.name.split(' ').count();
        let mut i = skip;
        let mut pairs = Vec::new();
        let mut inputs = 0;
        while i < words.len() {
            let w = &words[i];
            if let Some(flag) = w.strip_prefix("--") {
                if GLOBALS.contains(&flag) {
                    i += if matches!(flag, "in-place" | "dry-run" | "json" | "overwrite" | "yes" | "quiet" | "verbose")
                    {
                        1
                    } else {
                        2
                    };
                    continue;
                }
                let p = op.find_param(flag).unwrap_or_else(|| panic!("{}: unknown flag --{flag} in '{cmd}'", op.name));
                if p.kind == ParamKind::Flag {
                    pairs.push((flag.to_string(), "true".to_string()));
                    i += 1;
                } else {
                    let v =
                        words.get(i + 1).unwrap_or_else(|| panic!("{}: --{flag} needs a value in '{cmd}'", op.name));
                    pairs.push((flag.to_string(), v.clone()));
                    i += 2;
                }
            } else if w == "-o" {
                i += 2;
            } else {
                inputs += 1;
                i += 1;
            }
        }
        assert_eq!(inputs, 1, "{}: expected one input in '{cmd}'", op.name);
        Args::build(op, pairs).unwrap_or_else(|e| panic!("{}: '{cmd}': {}", op.name, e.message));
    }
}

/// Every command limited by a mask leaves the rest alone, and none panics on 1×1 or
/// fully transparent layers.
#[cfg(test)]
mod regions {
    use super::*;
    use pixelox_core::testing;

    /// Params that make each command visibly change pixels (pattern needs IO: fill.rs).
    const CASES: &[(&str, &[(&str, &str)])] = &[
        ("filter gaussian", &[("sigma", "3")]),
        ("filter box", &[("radius", "3")]),
        ("filter motion-blur", &[("distance", "8"), ("angle", "20")]),
        ("filter radial-blur", &[("amount", "30")]),
        ("filter lens-blur", &[("radius", "5")]),
        ("filter surface-blur", &[("radius", "3"), ("threshold", "60")]),
        ("filter median", &[("radius", "2")]),
        ("filter unsharp", &[("amount", "200"), ("radius", "2")]),
        ("filter smart-sharpen", &[("amount", "200"), ("radius", "2")]),
        ("filter add-noise", &[("amount", "20")]),
        ("filter denoise", &[("strength", "80")]),
        ("filter despeckle", &[]),
        ("filter dust", &[("radius", "2")]),
        ("filter emboss", &[]),
        ("filter find-edges", &[]),
        ("filter high-pass", &[("radius", "3")]),
        ("filter pixelate", &[("cell", "4")]),
        ("filter vignette", &[("amount", "80")]),
        ("filter lens-correct", &[("distortion", "50")]),
        ("filter pencil", &[]),
        ("filter pencil", &[("color", "true")]),
        ("filter charcoal", &[]),
        ("filter ink", &[("shading", "hatch")]),
        ("filter oil-paint", &[]),
        ("filter watercolor", &[]),
        ("filter cartoon", &[]),
        ("filter cutout", &[]),
        ("filter halftone", &[("mode", "cmyk")]),
        ("retouch erase", &[]),
        ("retouch heal", &[]),
        ("retouch clone", &[("from", "3,3"), ("to", "9,9"), ("radius", "30")]),
        ("retouch dodge", &[("exposure", "0.8")]),
        ("retouch burn", &[("exposure", "0.8")]),
        ("retouch sharpen", &[("amount", "200")]),
        ("retouch blur", &[("sigma", "3")]),
        ("retouch redeye", &[("at", "4,4"), ("radius", "30"), ("strength", "100")]),
        ("fill solid", &[("color", "#ff00ff")]),
        ("fill gradient", &[("stops", "#f00@0,#00f@1")]),
        ("paint stroke", &[("points", "0,8 24,8"), ("size", "40"), ("color", "#00ff00")]),
    ];

    fn texture(w: u32, h: u32) -> Image {
        Image::from_fn(w, h, |x, y| {
            let v = ((x * 37 + y * 91) % 17) as f32 / 17.0;
            [0.6 + 0.4 * v, 0.2 * v, 0.3 + 0.2 * v, 1.0]
        })
    }

    #[test]
    fn masks_limit_every_command() {
        for (name, params) in CASES {
            let spec = testing::find(&ops(), name);
            let src = texture(24, 16);
            let mut doc = testing::doc_from(src.clone());
            doc.put_mask("left", testing::half_mask(24, 16), false).unwrap();
            let mut p = params.to_vec();
            p.push(("mask", "left"));
            testing::run(&spec, &p, &mut doc).unwrap_or_else(|e| panic!("{name}: {}", e.message));
            let r = testing::raster(&doc, "Background");
            let mut changed_left = false;
            for y in 0..16 {
                for x in 0..24 {
                    if x >= 12 {
                        assert_eq!(r.get(x, y), src.get(x, y), "{name}: pixel {x},{y} outside the mask changed");
                    } else if r.get(x, y) != src.get(x, y) {
                        changed_left = true;
                    }
                }
            }
            assert!(changed_left, "{name}: nothing changed inside the mask");
        }
    }

    #[test]
    fn tiny_and_transparent_layers() {
        // codes an op may legitimately answer with on degenerate input
        let ok_codes = ["PX3320", "PX1003"];
        for (name, params) in CASES {
            let spec = testing::find(&ops(), name);
            let mut p = params.to_vec();
            if matches!(*name, "retouch erase") {
                p.push(("rect", "0,0,2x2"));
            }
            if matches!(*name, "retouch dodge" | "retouch burn" | "retouch sharpen" | "retouch blur") {
                p.push(("mask", "all"));
            }
            if *name == "retouch heal" {
                p.push(("at", "0,0"));
            }
            for img in [testing::solid(1, 1, Rgba::new(0.8, 0.2, 0.2, 1.0)), Image::new(4, 4)] {
                let mut doc = testing::doc_from(img.clone());
                doc.put_mask("all", Mask::filled(img.width, img.height, 1.0), false).unwrap();
                match testing::run(&spec, &p, &mut doc) {
                    Ok(_) => {
                        let r = testing::raster(&doc, "Background");
                        assert_eq!((r.width, r.height), (img.width, img.height), "{name}");
                        assert!(
                            r.data.iter().all(|v| v.is_finite() && (0.0..=1.0).contains(v)),
                            "{name}: values out of range"
                        );
                        if img.width == 4 && !name.starts_with("fill") && !name.starts_with("paint") {
                            assert_eq!(r.alpha_bbox(0.0), None, "{name}: transparent layer gained alpha");
                        }
                    }
                    Err(e) => assert!(
                        ok_codes.contains(&e.code.as_str()),
                        "{name} on {}x{}: {} {}",
                        img.width,
                        img.height,
                        e.code,
                        e.message
                    ),
                }
            }
        }
    }
}

/// Thread count must never change output bytes (PRD §6.6).
#[cfg(test)]
mod determinism {
    use super::*;
    use pixelox_core::testing;

    fn test_image() -> Image {
        Image::from_fn(300, 200, |x, y| {
            let n = ((x.wrapping_mul(2_654_435_761) ^ y.wrapping_mul(40_503)) % 97) as f32 / 97.0 * 0.2;
            let v = 0.2 + 0.5 * ((x as f32 * 0.05).sin() * (y as f32 * 0.07).cos()).abs() + n;
            [v, 1.0 - v, (v * 1.7) % 1.0, if x < 10 { 0.5 } else { 1.0 }]
        })
    }

    fn check(cases: &[(&str, &[(&str, &str)])]) {
        let img = test_image();
        let run_with = |threads: usize, name: &str, params: &[(&str, &str)]| {
            let pool = rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap();
            let spec = testing::find(&ops(), name);
            let mut doc = testing::doc_from(img.clone());
            pool.install(|| testing::run(&spec, params, &mut doc)).unwrap();
            testing::raster(&doc, "Background").clone()
        };
        for (name, params) in cases {
            let a = run_with(1, name, params);
            let b = run_with(3, name, params);
            assert!(
                a.data.iter().zip(&b.data).all(|(x, y)| x.to_bits() == y.to_bits()),
                "{name} {params:?} differs across thread counts"
            );
        }
    }

    #[test]
    fn art_filters_do_not_depend_on_thread_count() {
        check(&[
            ("filter pencil", &[("color", "true"), ("stroke", "2")]),
            ("filter charcoal", &[("smudge", "80")]),
            ("filter ink", &[("shading", "stipple")]),
            ("filter oil-paint", &[("brush", "9")]),
            ("filter watercolor", &[]),
            ("filter cartoon", &[("smooth", "14")]),
            ("filter cutout", &[("levels", "5")]),
            ("filter halftone", &[("mode", "cmyk"), ("shape", "line")]),
        ]);
    }

    #[test]
    fn outputs_do_not_depend_on_thread_count() {
        let cases: &[(&str, &[(&str, &str)])] = &[
            ("filter gaussian", &[("sigma", "2")]),
            ("filter gaussian", &[("sigma", "7")]),
            ("filter box", &[("radius", "9"), ("passes", "2")]),
            ("filter motion-blur", &[("angle", "30"), ("distance", "25")]),
            ("filter motion-blur", &[("angle", "100"), ("distance", "17")]),
            ("filter radial-blur", &[("kind", "spin"), ("amount", "30")]),
            ("filter lens-blur", &[("radius", "12"), ("highlights", "40")]),
            ("filter surface-blur", &[("radius", "3"), ("threshold", "20")]),
            ("filter surface-blur", &[("radius", "9"), ("threshold", "20")]),
            ("filter median", &[("radius", "1")]),
            ("filter median", &[("radius", "4")]),
            ("filter add-noise", &[("amount", "10")]),
            ("filter denoise", &[("strength", "60")]),
            ("filter dust", &[("radius", "2"), ("threshold", "8")]),
            ("filter smart-sharpen", &[("radius", "3")]),
            ("filter pixelate", &[("cell", "7"), ("kind", "hex")]),
            ("fill gradient", &[("dither", "true"), ("kind", "angular")]),
            (
                "paint stroke",
                &[
                    ("points", "10,20 290,180; 50,150 250,30"),
                    ("size", "40"),
                    ("flow", "30"),
                    ("size-jitter", "40"),
                    ("angle-jitter", "90"),
                    ("scatter", "50"),
                    ("count", "2"),
                    ("roundness", "60"),
                ],
            ),
            (
                "paint stroke",
                &[("d", "M 0 100 C 100 0 200 200 300 100"), ("size", "8%"), ("blend", "dissolve"), ("opacity", "60")],
            ),
        ];
        check(cases);
    }
}

/// Release-mode timings on a 24 MP layer (PRD §7.3 budgets). Run with
/// `cargo test -p pixelox-filter --release -- --ignored --nocapture bench_24mp`.
#[cfg(test)]
mod bench {
    use super::*;
    use pixelox_core::testing;

    #[test]
    #[ignore]
    fn bench_24mp() {
        let (w, h) = (6000u32, 4000u32);
        let img = Image::from_fn(w, h, |x, y| {
            let n = ((x.wrapping_mul(2_654_435_761) ^ y.wrapping_mul(40_503)) % 255) as f32 / 255.0 * 0.1;
            let v = 0.3 + 0.4 * ((x as f32 * 0.01).sin() * (y as f32 * 0.013).cos()).abs() + n;
            [v, v * 0.9, (1.0 - v).clamp(0.0, 1.0), 1.0]
        });
        let cases: &[(&str, &[(&str, &str)])] = &[
            ("filter gaussian", &[("sigma", "8")]),
            ("filter box", &[("radius", "20")]),
            ("filter motion-blur", &[("angle", "30"), ("distance", "60")]),
            ("filter radial-blur", &[("kind", "spin"), ("amount", "10")]),
            ("filter lens-blur", &[("radius", "20")]),
            ("filter surface-blur", &[("radius", "5"), ("threshold", "15")]),
            ("filter median", &[("radius", "3")]),
            ("filter median", &[("radius", "30")]),
            ("filter unsharp", &[("amount", "100"), ("radius", "2")]),
            ("filter smart-sharpen", &[("radius", "2")]),
            ("filter add-noise", &[("amount", "10")]),
            ("filter denoise", &[("strength", "50")]),
            ("filter despeckle", &[]),
            ("filter dust", &[("radius", "2"), ("threshold", "10")]),
            ("filter emboss", &[]),
            ("filter find-edges", &[]),
            ("filter high-pass", &[("radius", "10")]),
            ("filter pixelate", &[("cell", "16")]),
            ("filter pixelate", &[("cell", "16"), ("kind", "hex")]),
            ("filter vignette", &[("amount", "50")]),
            ("filter lens-correct", &[("distortion", "20"), ("ca", "1,-1"), ("vignette", "30")]),
            ("retouch dodge", &[("mask", "all"), ("exposure", "0.3")]),
            ("fill gradient", &[("kind", "radial"), ("dither", "true")]),
            ("filter pencil", &[]),
            ("filter pencil", &[("color", "true")]),
            ("filter charcoal", &[]),
            ("filter ink", &[]),
            ("filter ink", &[("shading", "hatch")]),
            ("filter oil-paint", &[]),
            ("filter watercolor", &[]),
            ("filter cartoon", &[]),
            ("filter cutout", &[]),
            ("filter halftone", &[]),
            ("filter halftone", &[("mode", "cmyk")]),
            // worst cases
            ("filter surface-blur", &[("radius", "3"), ("threshold", "15")]),
            ("filter surface-blur", &[("radius", "20"), ("threshold", "4")]),
            ("filter surface-blur", &[("radius", "100"), ("threshold", "50")]),
            ("filter motion-blur", &[("angle", "80"), ("distance", "400")]),
            ("filter radial-blur", &[("kind", "spin"), ("amount", "100")]),
            ("filter radial-blur", &[("kind", "zoom"), ("amount", "100")]),
            ("filter lens-blur", &[("radius", "8"), ("shape", "circle"), ("highlights", "50")]),
            ("filter median", &[("radius", "100")]),
            ("filter denoise", &[("strength", "100"), ("preserve-details", "50")]),
            ("filter smart-sharpen", &[("radius", "8"), ("kind", "motion"), ("angle", "30")]),
            ("filter box", &[("radius", "500"), ("passes", "3")]),
            ("retouch heal", &[("at", "3000,2000"), ("radius", "40")]),
            ("retouch clone", &[("from", "1000,1000"), ("to", "3000,2000"), ("radius", "300")]),
            ("retouch erase", &[("rect", "0,0,50%x50%")]),
            ("retouch frequency-split", &[]),
            ("retouch blemishes", &[]),
            ("retouch blemishes", &[("kind", "both"), ("min-size", "4"), ("max-size", "200")]),
            ("paint stroke", &[("points", "5%,50% 95%,55%"), ("size", "400"), ("flow", "8")]),
            ("paint stroke", &[("d", "M 0 0 L 6000 4000"), ("size", "60"), ("spacing", "2")]),
            (
                "paint stroke",
                &[
                    ("points", "10%,10% 90%,30% 10%,60% 90%,90%"),
                    ("size", "40"),
                    ("scatter", "200"),
                    ("count", "8"),
                    ("size-jitter", "50"),
                    ("angle-jitter", "360"),
                ],
            ),
            ("paint stroke", &[("points", "0,50% 100%,50%"), ("size", "3000"), ("spacing", "1")]),
        ];
        let only = std::env::var("PIXELOX_BENCH").unwrap_or_default();
        for (name, params) in cases.iter().filter(|(n, _)| n.contains(only.as_str())) {
            let spec = testing::find(&ops(), name);
            let mut doc = testing::doc_from(img.clone());
            if params.iter().any(|(k, _)| *k == "mask") {
                doc.put_mask("all", Mask::filled(w, h, 1.0), false).unwrap();
            }
            let t = std::time::Instant::now();
            testing::run(&spec, params, &mut doc).unwrap();
            println!("{name:<22} {params:?}: {} ms", t.elapsed().as_millis());
        }
        if "filter lens-blur".contains(only.as_str()) {
            let td = testing::TempDir::new("bench-depth");
            let depth = Image::from_fn(w, h, |x, _| {
                let v = x as f32 / w as f32;
                [v, v, v, 1.0]
            });
            let p = td.path("depth.pxt");
            std::fs::write(&p, crate::util::test_io::encode(&depth)).unwrap();
            let spec = testing::find(&ops(), "filter lens-blur");
            let mut doc = testing::doc_from(img.clone());
            let t = std::time::Instant::now();
            crate::util::test_io::run_io(&spec, &[("radius", "24"), ("depth", p.to_str().unwrap())], &mut doc).unwrap();
            println!("filter lens-blur       depth map, radius 24: {} ms", t.elapsed().as_millis());
        }
    }
}
