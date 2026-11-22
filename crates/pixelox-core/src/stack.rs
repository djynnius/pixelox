//! Where new layers go (ADR 0014). A newly created layer lands on top of the stack
//! unless the command is told otherwise with `--above <layer>`, `--below <layer>`, or
//! `--into <group>`; the active layer never decides it. Every layer-creating command takes
//! these three flags (`common::placement()`) and inserts through [`insert_new`] or
//! [`insert_at`].

use crate::doc::{Document, Layer, LayerKind, LayerPath};
use crate::error::{Error, Result};
use crate::op::{param, Args, ParamKind, ParamSpec};

/// Where to insert a new layer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Placement {
    /// On top of the whole stack (the default).
    Top,
    /// Directly above the layer at this path (in its group, if it has one).
    Above(LayerPath),
    /// Directly below the layer at this path.
    Below(LayerPath),
    /// On top of this group's children.
    Into(LayerPath),
}

/// `--above`, `--below`, `--into`: the placement flags of a layer-creating command.
pub fn params() -> Vec<ParamSpec> {
    vec![
        param("above", ParamKind::Layer)
            .help("Insert the new layer directly above this layer (default: on top of the stack)"),
        param("below", ParamKind::Layer).help("Insert the new layer directly below this layer"),
        param("into", ParamKind::Layer).help("Insert the new layer on top of this group's children"),
    ]
}

/// True when any placement flag was given.
pub fn given(args: &Args) -> bool {
    ["above", "below", "into"].iter().any(|k| args.has(k))
}

/// The placement the flags ask for (`None` when none was given). PX1005 for more than one
/// flag, PX2330 when `--into` names a layer that is not a group.
pub fn from_args(args: &Args, doc: &Document) -> Result<Option<Placement>> {
    let set: Vec<&str> = ["above", "below", "into"].into_iter().filter(|k| args.has(k)).collect();
    if set.len() > 1 {
        return Err(Error::new(
            "PX1005",
            format!("--{} and --{} both place the new layer; give only one", set[0], set[1]),
        )
        .with_hint("use one of --above <layer>, --below <layer>, or --into <group>"));
    }
    let Some(&flag) = set.first() else { return Ok(None) };
    let r = args.raw(flag).unwrap_or_default();
    let p = doc.resolve(r)?;
    Ok(Some(match flag {
        "above" => Placement::Above(p),
        "below" => Placement::Below(p),
        _ => {
            let l = doc.layer(&p);
            if !matches!(l.content, LayerKind::Group { .. }) {
                return Err(Error::new(
                    "PX2330",
                    format!("--into needs a group, and layer '{}' is a {} layer", l.name, l.kind_name()),
                )
                .with_hint(format!(
                    "place it next to the layer instead (--above '{}'), or make a group first: pixelox layer group <doc> --layers '{}' --in-place",
                    l.name, l.name
                ))
                .with_context("layer", &l.name)
                .with_context("kind", l.kind_name()));
            }
            Placement::Into(p)
        }
    }))
}

/// Insert `layer` at `at` and make it active. Returns its path.
pub fn insert_at(doc: &mut Document, layer: Layer, at: &Placement) -> LayerPath {
    match at {
        Placement::Top => doc.insert_above(None, layer),
        Placement::Above(p) => doc.insert_above(Some(p), layer),
        Placement::Below(p) => {
            doc.active = Some(layer.id.clone());
            let idx = *p.last().expect("a layer path");
            doc.siblings_mut(p).insert(idx, layer);
            p.clone()
        }
        Placement::Into(g) => {
            doc.active = Some(layer.id.clone());
            let children = doc.layer_mut(g).children_mut().expect("checked to be a group");
            children.push(layer);
            let mut np = g.clone();
            np.push(children.len() - 1);
            np
        }
    }
}

/// Insert a new layer where the placement flags say (default: on top of the stack) and
/// make it active. Resolve the placement before building the layer when the build can
/// fail, so a bad flag is reported first; this resolves it again, which is cheap.
pub fn insert_new(args: &Args, doc: &mut Document, layer: Layer) -> Result<LayerPath> {
    let at = from_args(args, doc)?.unwrap_or(Placement::Top);
    Ok(insert_at(doc, layer, &at))
}

/// Like [`insert_new`], but the default (no flag given) is directly above `source`: for
/// layers derived from another one (`layer dup`, `adjust --as-layer --layer X`).
pub fn insert_derived(args: &Args, doc: &mut Document, layer: Layer, source: Option<&[usize]>) -> Result<LayerPath> {
    let at = match from_args(args, doc)? {
        Some(at) => at,
        None => match source {
            Some(p) => Placement::Above(p.to_vec()),
            None => Placement::Top,
        },
    };
    Ok(insert_at(doc, layer, &at))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Rgba;
    use crate::image::Image;
    use crate::op::{Category, OpSpec};
    use crate::testing;
    use serde_json::Value;

    fn spec() -> OpSpec {
        fn noop(_: &Args, _: &mut Document, _: &mut crate::OpCtx) -> crate::Result<Value> {
            Ok(Value::Null)
        }
        OpSpec::doc("layer add", Category::Layer, "x", noop).params(params())
    }

    fn args(pairs: &[(&str, &str)]) -> Args {
        Args::build(&spec(), pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()).unwrap()
    }

    fn doc() -> Document {
        let mut d = testing::doc_layers(4, 4, vec![("A", Image::new(4, 4)), ("B", Image::new(4, 4))]);
        let mut g = d.new_layer("G", LayerKind::Group { children: vec![] });
        g.children_mut().unwrap().push(d.new_layer("Inner", LayerKind::Raster { image: Image::new(4, 4) }));
        d.insert_above(None, g);
        // the active layer is low in the stack: it must not matter
        d.active = Some(d.layer(&d.resolve("A").unwrap()).id.clone());
        d
    }

    fn names(d: &Document) -> Vec<String> {
        d.walk().into_iter().map(|(_, l)| l.name.clone()).collect()
    }

    fn new(d: &mut Document) -> Layer {
        d.new_layer("N", LayerKind::Raster { image: Image::filled(1, 1, Rgba::WHITE) })
    }

    #[test]
    fn default_is_the_top_whatever_is_active() {
        let mut d = doc();
        let l = new(&mut d);
        let p = insert_new(&args(&[]), &mut d, l).unwrap();
        assert_eq!(p, vec![3]);
        assert_eq!(names(&d)[0], "N");
        assert_eq!(d.layer(&d.resolve("@active").unwrap()).name, "N");
    }

    #[test]
    fn above_below_into() {
        let mut d = doc();
        let l = new(&mut d);
        insert_new(&args(&[("above", "A")]), &mut d, l).unwrap();
        assert_eq!(names(&d), ["G", "Inner", "B", "N", "A"]);
        let mut d = doc();
        let l = new(&mut d);
        insert_new(&args(&[("below", "A")]), &mut d, l).unwrap();
        assert_eq!(names(&d), ["G", "Inner", "B", "A", "N"]);
        let mut d = doc();
        let l = new(&mut d);
        let p = insert_new(&args(&[("into", "G")]), &mut d, l).unwrap();
        assert_eq!(p, vec![2, 1]);
        assert_eq!(names(&d), ["G", "N", "Inner", "B", "A"]);
        let mut d = doc();
        let l = new(&mut d);
        insert_new(&args(&[("below", "Inner")]), &mut d, l).unwrap();
        assert_eq!(names(&d), ["G", "Inner", "N", "B", "A"]);
    }

    #[test]
    fn derived_defaults_to_above_the_source() {
        let mut d = doc();
        let l = new(&mut d);
        let src = d.resolve("A").unwrap();
        insert_derived(&args(&[]), &mut d, l, Some(&src)).unwrap();
        assert_eq!(names(&d), ["G", "Inner", "B", "N", "A"]);
    }

    #[test]
    fn errors() {
        let d = doc();
        let e = from_args(&args(&[("above", "A"), ("below", "B")]), &d).unwrap_err();
        assert_eq!(e.code, "PX1005");
        let e = from_args(&args(&[("into", "A")]), &d).unwrap_err();
        assert_eq!(e.code, "PX2330");
        let e = from_args(&args(&[("above", "nope")]), &d).unwrap_err();
        assert_eq!(e.code, "PX2301");
    }
}
