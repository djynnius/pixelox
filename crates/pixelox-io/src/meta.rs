//! Metadata helpers: EXIF orientation, ICC profile names, and the EXIF dump behind
//! `analyze exif`.

use image::metadata::Orientation;
use serde_json::{json, Map, Value};

/// Set the EXIF orientation tag to 1 (pixels were already rotated on open).
pub fn reset_orientation(exif: &mut [u8]) {
    let _ = Orientation::remove_from_exif_chunk(exif);
}

/// The EXIF orientation value (1..8), if present.
pub fn orientation(exif: &[u8]) -> Option<u8> {
    Orientation::from_exif_chunk(exif).map(|o| o.to_exif())
}

/// Human-readable name of an ICC profile (its `desc` tag), or a size note.
pub fn icc_name(icc: &[u8]) -> String {
    let rd32 = |o: usize| -> Option<u32> { icc.get(o..o + 4).map(|b| u32::from_be_bytes([b[0], b[1], b[2], b[3]])) };
    let count = rd32(128).unwrap_or(0) as usize;
    for i in 0..count.min(256) {
        let base = 132 + i * 12;
        let (Some(sig), Some(off), Some(len)) = (icc.get(base..base + 4), rd32(base + 4), rd32(base + 8)) else {
            break;
        };
        if sig != b"desc" {
            continue;
        }
        let (off, len) = (off as usize, len as usize);
        let Some(tag) = icc.get(off..off + len) else { break };
        if tag.starts_with(b"desc") && tag.len() > 12 {
            let n = u32::from_be_bytes([tag[8], tag[9], tag[10], tag[11]]) as usize;
            if let Some(s) = tag.get(12..12 + n) {
                return String::from_utf8_lossy(s).trim_end_matches('\0').trim().to_string();
            }
        }
        if tag.starts_with(b"mluc") && tag.len() > 28 {
            let first_len = u32::from_be_bytes([tag[20], tag[21], tag[22], tag[23]]) as usize;
            let first_off = u32::from_be_bytes([tag[24], tag[25], tag[26], tag[27]]) as usize;
            if let Some(s) = tag.get(first_off..first_off + first_len) {
                let utf16: Vec<u16> =
                    s.chunks(2).filter(|c| c.len() == 2).map(|c| u16::from_be_bytes([c[0], c[1]])).collect();
                return String::from_utf16_lossy(&utf16).trim_end_matches('\0').trim().to_string();
            }
        }
    }
    format!("ICC profile ({} bytes)", icc.len())
}

/// True when the profile is (a variant of) sRGB.
pub fn is_srgb(icc: &[u8]) -> bool {
    icc_name(icc).to_ascii_lowercase().contains("srgb")
}

/// EXIF fields as `{tag: value}` (primary image IFD first, then thumbnail as `thumb.*`).
pub fn exif_fields(exif: &[u8]) -> Value {
    let reader = exif::Reader::new();
    let Ok(data) = reader.read_raw(exif.to_vec()) else {
        return json!({"error": "EXIF block could not be parsed"});
    };
    let mut m = Map::new();
    for f in data.fields() {
        let key = if f.ifd_num == exif::In::PRIMARY { f.tag.to_string() } else { format!("thumb.{}", f.tag) };
        m.insert(key, Value::String(f.display_value().with_unit(&data).to_string()));
    }
    Value::Object(m)
}

/// A minimal little-endian EXIF block carrying only an orientation tag (tests).
pub fn tests_exif_with_orientation(o: u16) -> Vec<u8> {
    let mut v = Vec::new();
    v.extend_from_slice(b"II*\0");
    v.extend_from_slice(&8u32.to_le_bytes());
    v.extend_from_slice(&1u16.to_le_bytes());
    v.extend_from_slice(&0x0112u16.to_le_bytes());
    v.extend_from_slice(&3u16.to_le_bytes());
    v.extend_from_slice(&1u32.to_le_bytes());
    v.extend_from_slice(&o.to_le_bytes());
    v.extend_from_slice(&[0, 0]);
    v.extend_from_slice(&0u32.to_le_bytes());
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orientation_round_trip() {
        let mut e = tests_exif_with_orientation(6);
        assert_eq!(orientation(&e), Some(6));
        reset_orientation(&mut e);
        assert_eq!(orientation(&e), Some(1));
        let f = exif_fields(&e);
        assert!(f.get("Orientation").is_some(), "{f}");
    }

    #[test]
    fn icc_desc_parsing() {
        // a fake v2 profile with one desc tag
        let mut icc = vec![0u8; 132];
        icc.extend_from_slice(&1u32.to_be_bytes()[..]);
        icc[128..132].copy_from_slice(&1u32.to_be_bytes());
        icc.truncate(132);
        icc.extend_from_slice(b"desc");
        icc.extend_from_slice(&144u32.to_be_bytes());
        let name = b"sRGB IEC61966-2.1\0";
        let tag_len = 12 + name.len();
        icc.extend_from_slice(&(tag_len as u32).to_be_bytes());
        icc.extend_from_slice(b"desc\0\0\0\0");
        icc.extend_from_slice(&(name.len() as u32).to_be_bytes());
        icc.extend_from_slice(name);
        assert_eq!(icc_name(&icc), "sRGB IEC61966-2.1");
        assert!(is_srgb(&icc));
        assert!(icc_name(&[0u8; 10]).starts_with("ICC profile"));
    }
}
