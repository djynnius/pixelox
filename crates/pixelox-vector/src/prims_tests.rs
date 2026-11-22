//! Tests of the generated primitives: the bundled icons, QR codes read back from the
//! rendered pixels, and barcodes scanned back from a rendered row of pixels.

use crate::{barcode, icons, ops, qr};
use pixelox_core::prelude::*;
use pixelox_core::testing::{self, find};

fn canvas(w: u32, h: u32, bg: Rgba) -> Document {
    crate::install();
    let mut d = Document::new(w, h);
    let l = d.new_layer("Background", LayerKind::Raster { image: Image::filled(w, h, bg) });
    d.insert_above(None, l);
    d
}

fn run(doc: &mut Document, name: &str, pairs: &[(&str, &str)]) -> Result<Value> {
    testing::run(&find(&ops(), name), pairs, doc)
}

fn warnings(doc: &mut Document, name: &str, pairs: &[(&str, &str)]) -> Vec<String> {
    testing::run_with_warnings(&find(&ops(), name), pairs, doc).unwrap().1
}

/// Decode every QR code in an image with rqrr's detector (as a phone camera would).
fn scan_qr(img: &Image) -> Vec<String> {
    let mut p = rqrr::PreparedImage::prepare_from_greyscale(img.width as usize, img.height as usize, |x, y| {
        let c = img.get(x as u32, y as u32);
        (color::luma(c[0], c[1], c[2]) * 255.0).round() as u8
    });
    p.detect_grids().into_iter().filter_map(|g| g.decode().ok().map(|(_, s)| s)).collect()
}

// ---------------------------------------------------------------------------------------
// icons

#[test]
fn the_set_has_the_event_and_info_icons() {
    assert!(icons::all().len() > 2000, "{} icons", icons::all().len());
    assert!(icons::version().starts_with('1'), "{}", icons::version());
    for name in [
        "calendar",
        "clock",
        "map-pin",
        "phone",
        "mail",
        "globe",
        "ticket",
        "user",
        "users",
        "music",
        "mic",
        "church",
        "heart",
        "star",
        "check",
        "arrow",
        "arrow-right",
        "play",
        "wifi",
        "cart",
        "shopping-cart",
        "tag",
        "gift",
        "share-2",
        "message-circle",
        "thumbs-up",
        "at-sign",
        "link",
        "camera",
        "send",
        "rss",
        "hash",
    ] {
        assert!(icons::find(name).is_some(), "missing icon {name}");
    }
    assert_eq!(icons::find("cart").unwrap().name, "shopping-cart");
    assert_eq!(icons::find("Map Pin").unwrap().name, "map-pin");
    // trademarks are left out
    assert!(icons::find("bluetooth").is_none() && icons::find("creative-commons").is_none());
    // every icon's path data parses
    for i in icons::all() {
        crate::spec::parse_d(i.d).unwrap_or_else(|e| panic!("{}: {e}", i.name));
    }
}

#[test]
fn search_ranks_names_before_tags() {
    let hits: Vec<&str> = icons::search("calendar").iter().map(|i| i.name).collect();
    assert_eq!(hits[0], "calendar");
    assert!(hits.iter().all(|h| h.contains("calendar")
        || icons::find(h).unwrap().aliases.iter().any(|a| a.contains("calendar"))
        || icons::find(h).unwrap().tags.iter().any(|t| t.contains("calendar"))));
    let ev: Vec<&str> = icons::search("event").iter().map(|i| i.name).collect();
    assert!(ev.contains(&"calendar") && ev.contains(&"ticket"), "{ev:?}");
    assert_eq!(icons::search("arrow")[0].name, "arrow-right", "the alias wins");
    assert!(icons::search("music note").iter().any(|i| i.name == "eighth-note"));
}

#[test]
fn icon_layers_are_live_round_stroked_shapes() {
    let mut d = canvas(200, 200, Rgba::WHITE);
    let r = run(&mut d, "shape icon", &[("icon", "calendar"), ("size", "48"), ("at", "20,30")]).unwrap();
    assert_eq!(r["layer"], "calendar");
    assert_eq!(r["stroke_width"], 4.0);
    let l = d.layer(&d.find_by_name("calendar").unwrap());
    let LayerKind::Shape { spec } = &l.content else { panic!("not a shape") };
    assert_eq!(spec["cap"], "round");
    assert_eq!(spec["geometry"]["icon"], "calendar");
    // the drawing stays inside its box (plus half the line width)
    let b = r["bbox"].as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect::<Vec<_>>();
    assert!(
        b[0] >= 20.0 - 2.0 && b[1] >= 30.0 - 2.0 && b[0] + b[2] <= 68.0 + 2.0 && b[1] + b[3] <= 78.0 + 2.0,
        "{b:?}"
    );
    let img = compose::composite(&d).unwrap();
    // the top line of the calendar (y = 4..6 of 24 → 38..42 px) is dark, the middle is not
    assert!(img.get(44, 36)[0] < 0.2, "{:?}", img.get(44, 36));
    assert!(img.get(44, 60)[0] > 0.9);
    // fill, color, and an unknown name
    run(
        &mut d,
        "shape icon",
        &[("icon", "heart"), ("size", "96"), ("anchor", "br"), ("fill", "#ff0000"), ("color", "#ff0000")],
    )
    .unwrap();
    let img = compose::composite(&d).unwrap();
    assert!(img.get(152, 155)[0] > 0.9 && img.get(152, 155)[1] < 0.1, "a solid red heart");
    // new layers go on top, or where the placement flags say
    assert_eq!(d.layers.last().unwrap().name, "heart");
    run(&mut d, "shape icon", &[("icon", "star"), ("below", "calendar")]).unwrap();
    let names: Vec<&str> = d.layers.iter().map(|l| l.name.as_str()).collect();
    assert_eq!(names, ["Background", "star", "calendar", "heart"]);
    let e = run(&mut d, "shape icon", &[("icon", "calender")]).unwrap_err();
    assert_eq!(e.code, "PX6030");
    assert!(e.hint.unwrap().contains("calendar"));
}

// ---------------------------------------------------------------------------------------
// QR codes

#[test]
fn qr_codes_scan_in_every_style() {
    // read back from the rendered pixels by a camera-style detector, at small and large sizes
    for data in ["https://example.com/tickets?id=2027", "WIFI:T:WPA;S:Guest;P:secret;;", "TEL:+15551234567"] {
        for style in ["square", "rounded", "dots"] {
            for (size, ec) in [("120", "m"), ("170", "h"), ("300", "q")] {
                let mut d = canvas(320, 320, Rgba::WHITE);
                let r =
                    run(&mut d, "shape qr", &[("data", data), ("size", size), ("style", style), ("ec", ec)]).unwrap();
                assert_eq!(r["kind"], "shape");
                let img = compose::composite(&d).unwrap();
                assert_eq!(scan_qr(&img), vec![data.to_string()], "{data} {style} {size} {ec}");
            }
        }
    }
}

#[test]
fn a_plate_makes_a_group_and_the_code_scans_on_a_dark_poster() {
    let mut d = canvas(400, 400, Rgba::from_u8(20, 24, 40, 255));
    let (r, w) = testing::run_with_warnings(
        &find(&ops(), "shape qr"),
        &[("data", "HELLO"), ("size", "240"), ("background-color", "white"), ("radius", "20"), ("style", "rounded")],
        &mut d,
    )
    .unwrap();
    assert!(w.is_empty(), "no contrast warning with a plate: {w:?}");
    assert_eq!(r["kind"], "group");
    assert_eq!(r["children"], json!(["QR code background", "QR code code"]));
    assert_eq!(scan_qr(&compose::composite(&d).unwrap()), vec!["HELLO".to_string()]);
    // without a plate the same code warns
    let w = warnings(&mut d, "shape qr", &[("data", "HELLO"), ("size", "120"), ("at", "0,0")]);
    assert!(w.iter().any(|w| w.contains("PXW6010")), "{w:?}");
}

#[test]
fn logo_gaps_only_where_error_correction_holds() {
    let data = "https://example.com/app";
    let mut d = canvas(360, 360, Rgba::WHITE);
    let r = run(&mut d, "shape qr", &[("data", data), ("size", "340"), ("logo-gap", "22")]).unwrap();
    assert_eq!(r["ec"], "h", "a gap picks the highest level");
    let lb = r["logo_box"].as_array().unwrap();
    assert!(lb[2].as_f64().unwrap() > 50.0);
    // the gap is empty, and the code still scans with a logo-sized blob in it
    let img = compose::composite(&d).unwrap();
    let (cx, cy) = (lb[0].as_f64().unwrap() + lb[2].as_f64().unwrap() / 2.0, lb[1].as_f64().unwrap() + 5.0);
    assert!(img.get(cx as u32, cy as u32)[0] > 0.99);
    let mut with_logo = img.clone();
    let (x0, y0, s) = (lb[0].as_f64().unwrap() as u32, lb[1].as_f64().unwrap() as u32, lb[2].as_f64().unwrap() as u32);
    for y in y0 + 4..y0 + s - 4 {
        for x in x0 + 4..x0 + s - 4 {
            with_logo.set(x, y, [0.8, 0.1, 0.1, 1.0]);
        }
    }
    assert_eq!(scan_qr(&with_logo), vec![data.to_string()]);
    // too large for the level: refused with the largest gap that would work
    let e = run(&mut d, "shape qr", &[("data", data), ("logo-gap", "30"), ("ec", "l")]).unwrap_err();
    assert_eq!(e.code, "PX6034");
    assert!(e.hint.unwrap().contains("--ec h"));
    let e = run(&mut d, "shape qr", &[("data", &"x".repeat(3000)), ("ec", "h")]).unwrap_err();
    assert_eq!(e.code, "PX6033");
}

#[test]
fn traced_outlines_cover_exactly_the_dark_modules() {
    let code = qr::encode("TRACE", qrcode::EcLevel::M).unwrap();
    let n = code.width();
    let g = qr::Grid { n, dark: code.to_colors().into_iter().map(|c| c == qrcode::Color::Dark).collect() };
    let path = qr::grid_path(&g, "square", 0.0, 0.0, 1.0);
    // the even-odd and non-zero areas agree (outer contours clockwise, holes the other way)
    let area: f64 = kurbo::Shape::area(&path).abs();
    let dark = g.dark.iter().filter(|d| **d).count() as f64;
    assert!((area - dark).abs() < 1e-6, "area {area} vs {dark} dark modules");
    assert_eq!(qr::read_back(&g).as_deref(), Some("TRACE".as_bytes()));
}

// ---------------------------------------------------------------------------------------
// barcodes

/// Scan the pixel row `y` of `img` between `x0` and `x1` into module bits of width `m`.
fn scan_row(img: &Image, y: u32, x0: f64, m: f64, n: usize) -> Vec<bool> {
    (0..n)
        .map(|i| {
            let x = (x0 + (i as f64 + 0.5) * m) as u32;
            let p = img.get(x, y);
            color::luma(p[0], p[1], p[2]) < 0.5
        })
        .collect()
}

#[test]
fn rendered_barcodes_scan_back_to_their_modules() {
    for (kind, data) in [
        ("ean13", "400638133393"),
        ("code128", "TKT-2027-0042"),
        ("code39", "PX-42"),
        ("upca", "03600029145"),
        ("ean8", "9638507"),
    ] {
        let mut d = canvas(800, 400, Rgba::WHITE);
        let r =
            run(&mut d, "shape barcode", &[("type", kind), ("data", data), ("module", "3"), ("at", "10,10")]).unwrap();
        let sym = barcode::encode(kind, data).unwrap();
        let img = compose::composite(&d).unwrap();
        let x0 = 10.0 + sym.quiet.0 as f64 * 3.0;
        let bits = scan_row(&img, 10 + 6 + 20, x0, 3.0, sym.modules.len());
        assert_eq!(bits, sym.modules, "{kind}");
        // quiet zones are clear
        assert!(scan_row(&img, 36, 10.0, 3.0, sym.quiet.0).iter().all(|b| !b), "{kind} left quiet zone");
        assert_eq!(r["module_px"], 3.0);
    }
}

#[test]
fn barcode_text_and_errors() {
    let mut d = canvas(600, 300, Rgba::WHITE);
    let r =
        run(&mut d, "shape barcode", &[("type", "ean13"), ("data", "400638133393"), ("width", "400"), ("at", "0,0")])
            .unwrap();
    assert_eq!(r["data"], "4006381333931");
    assert_eq!(r["check_digit"], "1");
    let h = r["box"][3].as_f64().unwrap();
    let with_text = h;
    let r2 = run(
        &mut d,
        "shape barcode",
        &[("type", "ean13"), ("data", "400638133393"), ("width", "400"), ("text", "off"), ("at", "0,150")],
    )
    .unwrap();
    assert!(r2["box"][3].as_f64().unwrap() < with_text, "the text line takes room");
    let e = run(&mut d, "shape barcode", &[("type", "ean13"), ("data", "4006381333932")]).unwrap_err();
    assert_eq!(e.code, "PX6032");
    let e = run(&mut d, "shape barcode", &[("type", "code39"), ("data", "abc")]).unwrap_err();
    assert_eq!(e.code, "PX6031");
    let e = run(&mut d, "shape barcode", &[("type", "upca"), ("data", "123")]).unwrap_err();
    assert_eq!(e.code, "PX6031");
}
