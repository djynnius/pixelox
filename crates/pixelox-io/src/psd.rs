//! PSD: layered read through the `psd` crate (PRD §7.6.1) and a writer for flattened and
//! "basic layered" PSD (raster layers with names, groups, opacity, blend modes, clipping,
//! visibility, and raster layer masks; other layer kinds are rasterized with a warning).

use crate::codec::warning;
use pixelox_core::blend::BlendMode;
use pixelox_core::doc::{Document, Layer, LayerKind};
use pixelox_core::envelope::Warning;
use pixelox_core::{compose, Error, Image, Mask, Result};

fn kebab(s: &str) -> String {
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                out.push('-');
            }
            out.extend(c.to_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

fn map_blend(b: impl std::fmt::Debug) -> BlendMode {
    BlendMode::parse(&kebab(&format!("{b:?}"))).unwrap_or(BlendMode::Normal)
}

fn crop_canvas_rgba(rgba: &[u8], cw: u32, ch: u32, l: i32, t: i32, r: i32, b: i32) -> (Image, i32, i32) {
    let x0 = l.clamp(0, cw as i32);
    let y0 = t.clamp(0, ch as i32);
    let x1 = r.clamp(0, cw as i32);
    let y1 = b.clamp(0, ch as i32);
    let (w, h) = ((x1 - x0).max(0) as u32, (y1 - y0).max(0) as u32);
    let mut img = Image::new(w, h);
    for y in 0..h {
        for x in 0..w {
            let si = (((y0 as u32 + y) * cw + (x0 as u32 + x)) * 4) as usize;
            if si + 3 < rgba.len() {
                img.set(
                    x,
                    y,
                    [
                        rgba[si] as f32 / 255.0,
                        rgba[si + 1] as f32 / 255.0,
                        rgba[si + 2] as f32 / 255.0,
                        rgba[si + 3] as f32 / 255.0,
                    ],
                );
            }
        }
    }
    (img, x0, y0)
}

/// Read a PSD into a document. Returns the document and warnings.
pub fn read(bytes: &[u8]) -> Result<(Document, Vec<Warning>)> {
    let parsed = std::panic::catch_unwind(|| psd::Psd::from_bytes(bytes));
    let p = match parsed {
        Ok(Ok(p)) => p,
        Ok(Err(e)) => return Err(Error::new("PX1103", format!("cannot read PSD: {e}"))),
        Err(_) => {
            return Err(Error::new("PX1103", "cannot read PSD: the file uses structures this reader does not support"))
        }
    };
    let mut warnings = Vec::new();
    match p.color_mode() {
        psd::ColorMode::Rgb => {}
        other => {
            return Err(Error::new("PX1203", format!("PSD color mode {other:?} is not supported; only RGB is"))
                .with_hint("convert the document to RGB in its source application"))
        }
    }
    if !matches!(p.depth(), psd::PsdDepth::Eight) {
        warnings.push(warning("PXW1203", format!("PSD depth {:?} is read through the 8-bit path", p.depth())));
    }
    let (cw, ch) = (p.width(), p.height());
    let mut doc = Document::new(cw, ch);
    doc.meta.source_format = Some("psd".into());
    let composite_rgba = std::panic::catch_unwind(|| p.rgba()).ok();
    if p.layers().is_empty() {
        let rgba = composite_rgba.ok_or_else(|| Error::new("PX1103", "cannot read the PSD composite"))?;
        let img = Image::from_rgba8(cw, ch, &rgba);
        let l = doc.new_layer("Background", LayerKind::Raster { image: img });
        doc.insert_above(None, l);
        return Ok((doc, warnings));
    }
    // groups by id → (name, visible, opacity, blend, parent)
    let groups = p.groups();
    // build nested structure: path of group ids for each layer, bottom → top
    let mut root: Vec<Layer> = Vec::new();
    // stack of (group id, Layer) currently open, outermost first
    fn ancestry(groups: &std::collections::HashMap<u32, psd::PsdGroup>, mut id: Option<u32>) -> Vec<u32> {
        let mut chain = Vec::new();
        let mut guard = 0;
        while let Some(g) = id {
            chain.push(g);
            id = groups.get(&g).and_then(|gr| gr.parent_id());
            guard += 1;
            if guard > 64 {
                break;
            }
        }
        chain.reverse();
        chain
    }
    fn group_vec<'a>(root: &'a mut Vec<Layer>, path: &[u32], ids: &[(Vec<u32>, usize)]) -> &'a mut Vec<Layer> {
        // find the group node for `path` (created on demand by the caller)
        let mut cur = root;
        for depth in 1..=path.len() {
            let key = path[..depth].to_vec();
            let idx = ids.iter().find(|(k, _)| *k == key).map(|(_, i)| *i).expect("group created");
            cur = match &mut cur[idx].content {
                LayerKind::Group { children } => children,
                _ => unreachable!(),
            };
        }
        cur
    }
    let mut created: Vec<(Vec<u32>, usize)> = Vec::new();
    let mut layer_count = 0usize;
    // The psd crate returns layers top-first (it reverses Photoshop's bottom-first record
    // order despite its docs); build the stack bottom-first.
    for pl in p.layers().iter().rev() {
        let chain = ancestry(groups, pl.parent_id());
        // create missing groups along the chain
        for depth in 1..=chain.len() {
            let key = chain[..depth].to_vec();
            if created.iter().any(|(k, _)| *k == key) {
                continue;
            }
            let g = &groups[&chain[depth - 1]];
            let mut gl = doc.new_layer(g.name(), LayerKind::Group { children: vec![] });
            // Photoshop sets flag bit 1 for *hidden* layers; the psd crate reports that bit as
            // `visible()`, so invert it (psd-tools and Photoshop agree on hidden)
            gl.visible = !g.visible();
            gl.opacity = g.opacity() as f32 / 255.0 * 100.0;
            gl.blend = map_blend(g.blend_mode());
            let parent = group_vec(&mut root, &chain[..depth - 1], &created);
            parent.push(gl);
            let idx = parent.len() - 1;
            created.push((key, idx));
        }
        let rgba = std::panic::catch_unwind(|| pl.rgba())
            .map_err(|_| Error::new("PX1103", format!("cannot decode PSD layer '{}'", pl.name())))?;
        // the psd crate reports right/bottom inclusively (PSD stores them exclusive)
        let (img, x, y) = crop_canvas_rgba(
            &rgba,
            cw,
            ch,
            pl.layer_left(),
            pl.layer_top(),
            pl.layer_right() + 1,
            pl.layer_bottom() + 1,
        );
        let mut l = doc.new_layer(pl.name(), LayerKind::Raster { image: img });
        l.x = x;
        l.y = y;
        l.visible = !pl.visible();
        l.opacity = pl.opacity() as f32 / 255.0 * 100.0;
        l.blend = map_blend(pl.blend_mode());
        if l.blend == BlendMode::PassThrough {
            l.blend = BlendMode::Normal;
        }
        // the crate's is_clipping_mask() is true for a *base* layer (clipping byte 0)
        l.clip = !pl.is_clipping_mask();
        let parent = group_vec(&mut root, &chain, &created);
        parent.push(l);
        layer_count += 1;
    }
    doc.layers = root;
    doc.active = doc.layers.last().map(|l| l.id.clone());
    // compare against the stored composite: masks, adjustment layers, text, and styles are
    // not visible to this reader, so keep the composite for reference when they differ
    if let (Some(rgba), Ok(ours)) = (composite_rgba, compose::composite(&doc)) {
        let stored = Image::from_rgba8(cw, ch, &rgba);
        let diff: f64 = stored.data.iter().zip(&ours.data).map(|(a, b)| (a - b).abs() as f64).sum::<f64>()
            / stored.data.len().max(1) as f64;
        if diff > 2.0 / 255.0 {
            warnings.push(warning(
                "PXW1203",
                format!("{layer_count} PSD layers were imported, but the file uses features this reader renders from the PSD composite (layer masks, adjustment layers, type, or styles); a hidden 'PSD composite' layer holds the saved result"),
            ));
            let mut l = doc.new_layer("PSD composite", LayerKind::Raster { image: stored });
            l.visible = false;
            l.locked = true;
            doc.layers.push(l);
        }
    }
    Ok((doc, warnings))
}

// ---------------------------------------------------------------------------------------
// writing

fn packbits(row: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let n = row.len();
    let mut i = 0;
    while i < n {
        // run of identical bytes
        let mut run = 1;
        while i + run < n && run < 128 && row[i + run] == row[i] {
            run += 1;
        }
        if run >= 2 {
            out.push((257 - run) as u8);
            out.push(row[i]);
            i += run;
            continue;
        }
        // literal run
        let start = i;
        let mut lit = 0;
        while i < n && lit < 128 {
            if i + 1 < n && row[i] == row[i + 1] {
                break;
            }
            i += 1;
            lit += 1;
        }
        out.push((lit - 1) as u8);
        out.extend_from_slice(&row[start..start + lit]);
    }
    out
}

/// RLE-compress planes; returns (row byte counts, data) for `planes` of `w`×`h`.
fn rle_planes(planes: &[Vec<u8>], w: usize, h: usize) -> (Vec<u16>, Vec<u8>) {
    let mut counts = Vec::new();
    let mut data = Vec::new();
    for p in planes {
        for y in 0..h {
            let enc = packbits(&p[y * w..(y + 1) * w]);
            counts.push(enc.len() as u16);
            data.extend(enc);
        }
    }
    (counts, data)
}

fn planes_of(img: &Image, alpha: bool) -> Vec<Vec<u8>> {
    let px = img.to_rgba8();
    let n = if alpha { 4 } else { 3 };
    (0..n).map(|c| px.chunks(4).map(|p| p[c]).collect()).collect()
}

fn pascal_name(name: &str) -> Vec<u8> {
    let bytes: Vec<u8> = name.chars().map(|c| if c.is_ascii() { c as u8 } else { b'?' }).take(255).collect();
    let mut v = vec![bytes.len() as u8];
    v.extend(bytes);
    while v.len() % 4 != 0 {
        v.push(0);
    }
    v
}

fn unicode_name_block(name: &str) -> Vec<u8> {
    let utf16: Vec<u16> = name.encode_utf16().collect();
    let mut body = Vec::new();
    body.extend_from_slice(&(utf16.len() as u32).to_be_bytes());
    for u in &utf16 {
        body.extend_from_slice(&u.to_be_bytes());
    }
    while body.len() % 4 != 0 {
        body.push(0);
    }
    let mut v = b"8BIMluni".to_vec();
    v.extend_from_slice(&(body.len() as u32).to_be_bytes());
    v.extend(body);
    v
}

fn section_divider(kind: u32, blend: &str) -> Vec<u8> {
    let mut body = kind.to_be_bytes().to_vec();
    body.extend_from_slice(b"8BIM");
    body.extend_from_slice(blend.as_bytes());
    let mut v = b"8BIMlsct".to_vec();
    v.extend_from_slice(&(body.len() as u32).to_be_bytes());
    v.extend(body);
    v
}

struct PsdLayerOut {
    name: String,
    rect: (i32, i32, i32, i32),
    image: Option<Image>,
    mask: Option<(Mask, i32, i32, i32, i32, u8)>,
    opacity: u8,
    blend: &'static str,
    clip: bool,
    hidden: bool,
    divider: Option<u32>,
}

fn flatten_tree(
    doc: &Document,
    layers: &[Layer],
    out: &mut Vec<PsdLayerOut>,
    rasterized: &mut Vec<String>,
) -> Result<()> {
    for l in layers {
        let opacity = (l.opacity / 100.0 * 255.0).round().clamp(0.0, 255.0) as u8;
        if let LayerKind::Group { children } = &l.content {
            // bottom divider, children, then the group's own record on top
            out.push(PsdLayerOut {
                name: "</Layer group>".into(),
                rect: (0, 0, 0, 0),
                image: None,
                mask: None,
                opacity: 255,
                blend: "norm",
                clip: false,
                hidden: false,
                divider: Some(3),
            });
            flatten_tree(doc, children, out, rasterized)?;
            let key = if l.blend == BlendMode::PassThrough { "pass" } else { l.blend.psd_key() };
            out.push(PsdLayerOut {
                name: l.name.clone(),
                rect: (0, 0, 0, 0),
                image: None,
                mask: None,
                opacity,
                blend: key,
                clip: false,
                hidden: !l.visible,
                divider: Some(1),
            });
            continue;
        }
        if matches!(l.content, LayerKind::Adjustment { .. }) {
            rasterized.push(format!("{} (adjustment, dropped)", l.name));
            continue;
        }
        let rendered = match &l.content {
            LayerKind::Raster { image } => Some((image.clone(), l.x, l.y)),
            _ => {
                rasterized.push(l.name.clone());
                let mut tmp = l.clone();
                tmp.fx.clear();
                compose::render_layer(doc, &tmp)?.map(|r| (r.image, r.x, r.y))
            }
        };
        if !l.fx.is_empty() {
            rasterized.push(format!("{} (styles not written)", l.name));
        }
        let (img, x, y) = rendered.unwrap_or_else(|| (Image::new(0, 0), 0, 0));
        let rect = (y, x, y + img.height as i32, x + img.width as i32);
        let mask = l.mask.as_ref().map(|lm| {
            let m = &lm.mask;
            (m.clone(), 0, 0, m.height as i32, m.width as i32, if lm.enabled { 0 } else { 2 })
        });
        out.push(PsdLayerOut {
            name: l.name.clone(),
            rect,
            image: Some(img),
            mask,
            opacity,
            blend: l.blend.psd_key(),
            clip: l.clip,
            hidden: !l.visible,
            divider: None,
        });
    }
    Ok(())
}

/// Write a PSD. `layered`: include layer records (basic layered); otherwise flattened.
pub fn write(composite: &Image, doc: Option<&Document>, layered: bool) -> Result<(Vec<u8>, Vec<Warning>)> {
    let mut warnings = Vec::new();
    let (w, h) = (composite.width as usize, composite.height as usize);
    if w > 30_000 || h > 30_000 {
        return Err(Error::new("PX1107", "PSD is limited to 30000 px per side"));
    }
    let alpha = !composite.is_opaque();
    let mut f = Vec::new();
    f.extend_from_slice(b"8BPS");
    f.extend_from_slice(&1u16.to_be_bytes());
    f.extend_from_slice(&[0u8; 6]);
    f.extend_from_slice(&(if alpha { 4u16 } else { 3 }).to_be_bytes());
    f.extend_from_slice(&(h as u32).to_be_bytes());
    f.extend_from_slice(&(w as u32).to_be_bytes());
    f.extend_from_slice(&8u16.to_be_bytes());
    f.extend_from_slice(&3u16.to_be_bytes());
    f.extend_from_slice(&0u32.to_be_bytes()); // color mode data
    f.extend_from_slice(&0u32.to_be_bytes()); // image resources
                                              // layer and mask information
    let mut lmi = Vec::new();
    if let (true, Some(doc)) = (layered, doc) {
        let mut recs = Vec::new();
        let mut rasterized = Vec::new();
        flatten_tree(doc, &doc.layers, &mut recs, &mut rasterized)?;
        if !rasterized.is_empty() {
            warnings
                .push(warning("PXW1205", format!("written as raster layers in the PSD: {}", rasterized.join(", "))));
        }
        let mut info = Vec::new();
        info.extend_from_slice(&(recs.len() as i16).to_be_bytes());
        let mut channel_data = Vec::new();
        for r in &recs {
            let (t, l, b, rt) = r.rect;
            info.extend_from_slice(&t.to_be_bytes());
            info.extend_from_slice(&l.to_be_bytes());
            info.extend_from_slice(&b.to_be_bytes());
            info.extend_from_slice(&rt.to_be_bytes());
            let (lw, lh) = ((rt - l).max(0) as usize, (b - t).max(0) as usize);
            let mut chans: Vec<(i16, Vec<u8>)> = Vec::new();
            let img = r.image.clone().unwrap_or_else(|| Image::new(0, 0));
            let planes = planes_of(&img, true);
            for (id, plane) in [(-1i16, 3usize), (0, 0), (1, 1), (2, 2)] {
                let p = if planes.is_empty() { Vec::new() } else { planes[plane].clone() };
                let (counts, data) = rle_planes(&[p], lw, lh);
                let mut c = 1u16.to_be_bytes().to_vec();
                for cnt in counts {
                    c.extend_from_slice(&cnt.to_be_bytes());
                }
                c.extend(data);
                chans.push((id, c));
            }
            if let Some((m, mt, ml, mb, mr, _)) = &r.mask {
                let mw = (mr - ml) as usize;
                let mh = (mb - mt) as usize;
                let (counts, data) = rle_planes(&[m.to_gray8()], mw, mh);
                let mut c = 1u16.to_be_bytes().to_vec();
                for cnt in counts {
                    c.extend_from_slice(&cnt.to_be_bytes());
                }
                c.extend(data);
                chans.push((-2, c));
            }
            info.extend_from_slice(&(chans.len() as u16).to_be_bytes());
            for (id, c) in &chans {
                info.extend_from_slice(&id.to_be_bytes());
                info.extend_from_slice(&(c.len() as u32).to_be_bytes());
            }
            info.extend_from_slice(b"8BIM");
            info.extend_from_slice(r.blend.as_bytes());
            info.push(r.opacity);
            info.push(u8::from(r.clip));
            let mut flags = 0u8;
            if r.hidden {
                flags |= 0x02;
            }
            if r.divider.is_some() {
                flags |= 0x18;
            }
            info.push(flags);
            info.push(0);
            let mut extra = Vec::new();
            match &r.mask {
                Some((_, mt, ml, mb, mr, mflags)) => {
                    extra.extend_from_slice(&20u32.to_be_bytes());
                    for v in [mt, ml, mb, mr] {
                        extra.extend_from_slice(&v.to_be_bytes());
                    }
                    extra.push(0);
                    extra.push(*mflags);
                    extra.extend_from_slice(&[0, 0]);
                }
                None => extra.extend_from_slice(&0u32.to_be_bytes()),
            }
            extra.extend_from_slice(&0u32.to_be_bytes()); // blending ranges
            extra.extend(pascal_name(&r.name));
            extra.extend(unicode_name_block(&r.name));
            if let Some(kind) = r.divider {
                extra.extend(section_divider(kind, r.blend));
            }
            info.extend_from_slice(&(extra.len() as u32).to_be_bytes());
            info.extend(extra);
            for (_, c) in chans {
                channel_data.extend(c);
            }
        }
        info.extend(channel_data);
        if info.len() % 2 != 0 {
            info.push(0);
        }
        lmi.extend_from_slice(&(info.len() as u32).to_be_bytes());
        lmi.extend(info);
        lmi.extend_from_slice(&0u32.to_be_bytes()); // global layer mask info
    }
    f.extend_from_slice(&(lmi.len() as u32).to_be_bytes());
    f.extend(lmi);
    // merged image data
    let planes = planes_of(composite, alpha);
    let (counts, data) = rle_planes(&planes, w, h);
    f.extend_from_slice(&1u16.to_be_bytes());
    for c in counts {
        f.extend_from_slice(&c.to_be_bytes());
    }
    f.extend(data);
    Ok((f, warnings))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pixelox_core::Rgba;

    #[test]
    fn packbits_round_trip() {
        fn unpack(src: &[u8]) -> Vec<u8> {
            let mut out = Vec::new();
            let mut i = 0;
            while i < src.len() {
                let n = src[i] as i8;
                i += 1;
                if n >= 0 {
                    out.extend_from_slice(&src[i..i + n as usize + 1]);
                    i += n as usize + 1;
                } else if n != -128 {
                    out.extend(std::iter::repeat_n(src[i], (1 - n as i32) as usize));
                    i += 1;
                }
            }
            out
        }
        for row in [vec![1u8; 300], vec![1, 2, 3, 4, 4, 4, 4, 5, 6], (0..=255).collect::<Vec<u8>>(), vec![7]] {
            assert_eq!(unpack(&packbits(&row)), row);
        }
    }

    #[test]
    fn groups_clipping_and_hidden_layers_round_trip() {
        let mut doc = Document::from_image(Image::filled(10, 10, Rgba::WHITE));
        let mut g = doc.new_layer("Group", LayerKind::Group { children: vec![] });
        let inner =
            doc.new_layer("Inner", LayerKind::Raster { image: Image::filled(4, 4, Rgba::new(0.0, 1.0, 0.0, 1.0)) });
        let mut clipped =
            doc.new_layer("Clipped", LayerKind::Raster { image: Image::filled(10, 10, Rgba::new(1.0, 0.0, 0.0, 1.0)) });
        clipped.clip = true;
        g.children_mut().unwrap().extend([inner, clipped]);
        doc.insert_above(None, g);
        let mut hidden = doc.new_layer("Hidden", LayerKind::Raster { image: Image::filled(10, 10, Rgba::BLACK) });
        hidden.visible = false;
        doc.insert_above(None, hidden);
        let comp = compose::composite(&doc).unwrap();
        let (bytes, _) = write(&comp, Some(&doc), true).unwrap();
        let (back, warns) = read(&bytes).unwrap();
        assert!(warns.is_empty(), "{warns:?}");
        let order: Vec<String> = back.walk().iter().map(|(_, l)| l.name.clone()).collect();
        assert_eq!(order, ["Hidden", "Group", "Clipped", "Inner", "Background"]);
        let l = |n: &str| back.layer(&back.resolve(n).unwrap()).clone();
        assert!(!l("Hidden").visible && l("Group").visible && l("Background").visible);
        assert!(l("Clipped").clip && !l("Inner").clip);
        let again = compose::composite(&back).unwrap();
        assert!(pixelox_core::testing::max_diff(&comp, &again) < 2.0 / 255.0);
    }

    #[test]
    fn layered_psd_round_trips_through_the_reader() {
        let mut doc = Document::from_image(Image::filled(8, 6, Rgba::new(0.0, 0.0, 1.0, 1.0)));
        let mut top =
            doc.new_layer("Red box", LayerKind::Raster { image: Image::filled(4, 2, Rgba::new(1.0, 0.0, 0.0, 1.0)) });
        top.x = 2;
        top.y = 1;
        top.opacity = 50.0;
        top.blend = BlendMode::Multiply;
        doc.insert_above(None, top);
        let comp = compose::composite(&doc).unwrap();
        let (bytes, _) = write(&comp, Some(&doc), true).unwrap();
        let (back, _) = read(&bytes).unwrap();
        assert_eq!((back.width, back.height), (8, 6));
        let names: Vec<String> = back.walk().iter().map(|(_, l)| l.name.clone()).collect();
        assert!(names.contains(&"Red box".to_string()), "{names:?}");
        let l = back.layer(&back.resolve("Red box").unwrap());
        assert_eq!(l.blend, BlendMode::Multiply);
        assert!((l.opacity - 50.0).abs() < 0.5);
        assert_eq!((l.x, l.y), (2, 1));
        // pixels survive, not just structure
        let bg = back.layer(&back.resolve("Background").unwrap());
        assert_eq!(bg.raster().unwrap().get(0, 0), [0.0, 0.0, 1.0, 1.0], "background pixels");
        assert_eq!(l.raster().unwrap().get(1, 1), [1.0, 0.0, 0.0, 1.0], "layer pixels");
        assert!(back.find_by_name("PSD composite").is_none(), "our own PSD must not need the composite fallback");
        let (flat, _) = write(&comp, None, false).unwrap();
        let (fb, _) = read(&flat).unwrap();
        assert_eq!(fb.layer_count(), 1);
    }
}
