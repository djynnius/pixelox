//! Behavior tests for `fx grain`.

use pixelox_core::prelude::*;
use pixelox_core::testing::{self, find, run};

fn doc() -> Document {
    pixelox_fx::install();
    let sq = Image::from_fn(80, 60, |x, y| {
        if (20..60).contains(&x) && (10..50).contains(&y) {
            [0.5, 0.5, 0.5, 1.0]
        } else {
            [0.0; 4]
        }
    });
    testing::doc_layers(80, 60, vec![("Box", sq)])
}

fn fx(name: &str, pairs: &[(&str, &str)], d: &mut Document) -> Value {
    run(&find(&pixelox_fx::ops(), name), pairs, d).unwrap_or_else(|e| panic!("{name}: {e:?}"))
}

fn render(d: &Document) -> Rendered {
    let l = d.layer(&d.resolve("Box").unwrap()).clone();
    compose::render_layer(d, &l).unwrap().unwrap()
}

#[test]
fn grain_is_live_keeps_coverage_and_moves_with_the_layer() {
    let mut d = doc();
    let before = render(&d);
    let v = fx("fx grain", &[("layer", "Box"), ("amount", "60")], &mut d);
    assert_eq!(v["action"], "added");
    let l = d.layer(&d.resolve("Box").unwrap());
    assert_eq!(l.fx[0].kind, "grain");
    assert!(l.fx[0].params["variant"].as_str().unwrap().parse::<u64>().is_ok(), "the pattern is stored");
    let after = render(&d);
    // same coverage, changed colors
    assert_eq!(
        (after.x, after.y, after.image.width, after.image.height),
        (before.x, before.y, before.image.width, before.image.height)
    );
    let mut changed = 0;
    for (p, q) in after.image.data.chunks(4).zip(before.image.data.chunks(4)) {
        assert_eq!(p[3], q[3]);
        if (p[0] - q[0]).abs() > 1e-4 {
            changed += 1;
        }
    }
    assert!(changed > 1000, "{changed}");
    // moving the layer moves its grain unchanged
    let p = d.resolve("Box").unwrap();
    geom::translate_layer(&mut d, &p, 7.0, 3.0).unwrap();
    let moved = render(&d);
    assert_eq!((moved.x, moved.y), (after.x + 7, after.y + 3));
    assert_eq!(moved.image, after.image);
    // off keeps the settings; rm removes it
    fx("fx grain", &[("layer", "Box"), ("enabled", "off")], &mut d);
    assert_eq!(render(&d).image, before.image);
    let r = fx("fx rm", &[("layer", "Box"), ("kind", "grain")], &mut d);
    assert_eq!(r["count"], 1);
}

#[test]
fn grain_variant_is_kept_when_replaced() {
    let mut d = doc();
    fx("fx grain", &[("layer", "Box"), ("variant", "12")], &mut d);
    fx("fx grain", &[("layer", "Box"), ("amount", "50")], &mut d);
    let l = d.layer(&d.resolve("Box").unwrap());
    assert_eq!(l.fx.len(), 1);
    assert_eq!(l.fx[0].params["variant"], "12");
}
