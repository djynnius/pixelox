//! `.cube` color lookup tables (Adobe/Resolve format): `LUT_3D_SIZE`, `LUT_1D_SIZE`,
//! `DOMAIN_MIN`/`DOMAIN_MAX` (and Resolve's `LUT_*_INPUT_RANGE`), with trilinear or
//! tetrahedral interpolation of the 3-D table.

use pixelox_core::prelude::*;

#[derive(Clone, Debug)]
pub struct Lut {
    pub title: Option<String>,
    /// Edge length of the 3-D table (0 = none).
    pub size3: usize,
    /// Red varies fastest: index `r + g·n + b·n²`.
    pub table3: Vec<[f32; 3]>,
    /// Length of the 1-D shaper table (0 = none).
    pub size1: usize,
    pub table1: Vec<[f32; 3]>,
    pub domain_min: [f32; 3],
    pub domain_max: [f32; 3],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Interp {
    Trilinear,
    Tetrahedral,
}

fn bad(line: usize, msg: impl Into<String>) -> Error {
    let msg = msg.into();
    let full =
        if line > 0 { format!("invalid .cube LUT (line {line}): {msg}") } else { format!("invalid .cube LUT: {msg}") };
    Error::new("PX3311", full).with_context("line", line)
}

fn floats(parts: &[&str], line: usize) -> Result<Vec<f32>> {
    parts
        .iter()
        .map(|p| {
            p.parse::<f32>().ok().filter(|v| v.is_finite()).ok_or_else(|| bad(line, format!("'{p}' is not a number")))
        })
        .collect()
}

impl Lut {
    /// Parse `.cube` text. Every malformation is PX3311 with the offending line.
    pub fn parse(text: &str) -> Result<Lut> {
        let mut lut = Lut {
            title: None,
            size3: 0,
            table3: Vec::new(),
            size1: 0,
            table1: Vec::new(),
            domain_min: [0.0; 3],
            domain_max: [1.0; 3],
        };
        let mut data: Vec<[f32; 3]> = Vec::new();
        for (i, raw) in text.lines().enumerate() {
            let ln = i + 1;
            let line = raw.split('#').next().unwrap_or("").trim();
            if line.is_empty() {
                continue;
            }
            let parts: Vec<&str> = line.split_whitespace().collect();
            let key = parts[0];
            if key.starts_with(|c: char| c.is_ascii_alphabetic()) {
                if !data.is_empty() {
                    return Err(bad(ln, format!("keyword '{key}' after the table data")));
                }
                match key {
                    "TITLE" => lut.title = Some(line["TITLE".len()..].trim().trim_matches('"').to_string()),
                    "LUT_3D_SIZE" | "LUT_1D_SIZE" => {
                        let n: usize = parts
                            .get(1)
                            .and_then(|v| v.parse().ok())
                            .ok_or_else(|| bad(ln, format!("{key} needs a whole number")))?;
                        if key == "LUT_3D_SIZE" {
                            if !(2..=256).contains(&n) {
                                return Err(bad(ln, format!("LUT_3D_SIZE {n} is outside 2..256")));
                            }
                            lut.size3 = n;
                        } else {
                            if !(2..=65536).contains(&n) {
                                return Err(bad(ln, format!("LUT_1D_SIZE {n} is outside 2..65536")));
                            }
                            lut.size1 = n;
                        }
                    }
                    "DOMAIN_MIN" | "DOMAIN_MAX" => {
                        if parts.len() != 4 {
                            return Err(bad(ln, format!("{key} needs three numbers")));
                        }
                        let v = floats(&parts[1..], ln)?;
                        let d = [v[0], v[1], v[2]];
                        if key == "DOMAIN_MIN" {
                            lut.domain_min = d;
                        } else {
                            lut.domain_max = d;
                        }
                    }
                    "LUT_1D_INPUT_RANGE" | "LUT_3D_INPUT_RANGE" => {
                        if parts.len() != 3 {
                            return Err(bad(ln, format!("{key} needs two numbers")));
                        }
                        let v = floats(&parts[1..], ln)?;
                        lut.domain_min = [v[0]; 3];
                        lut.domain_max = [v[1]; 3];
                    }
                    // other keywords (vendor extensions) carry no table semantics
                    _ => {}
                }
                continue;
            }
            if parts.len() != 3 {
                return Err(bad(ln, format!("a table row needs three numbers, got {}", parts.len())));
            }
            let v = floats(&parts, ln)?;
            data.push([v[0], v[1], v[2]]);
        }
        if lut.size3 == 0 && lut.size1 == 0 {
            return Err(bad(0, "no LUT_3D_SIZE or LUT_1D_SIZE line"));
        }
        for c in 0..3 {
            if lut.domain_max[c] <= lut.domain_min[c] {
                return Err(bad(0, format!("DOMAIN_MAX must exceed DOMAIN_MIN (channel {c})")));
            }
        }
        let want = lut.size1 + lut.size3.pow(3);
        if data.len() != want {
            return Err(bad(0, format!("expected {want} table rows, found {}", data.len()))
                .with_context("expected_rows", want)
                .with_context("rows", data.len()));
        }
        lut.table1 = data[..lut.size1].to_vec();
        lut.table3 = data[lut.size1..].to_vec();
        Ok(lut)
    }

    #[inline]
    fn at(&self, r: usize, g: usize, b: usize) -> [f32; 3] {
        let n = self.size3;
        self.table3[r + g * n + b * n * n]
    }

    /// Map an encoded RGB triple through the LUT.
    pub fn apply(&self, rgb: [f32; 3], interp: Interp) -> [f32; 3] {
        let mut t: [f32; 3] = std::array::from_fn(|c| {
            ((rgb[c] - self.domain_min[c]) / (self.domain_max[c] - self.domain_min[c])).clamp(0.0, 1.0)
        });
        if self.size1 > 0 {
            let n = self.size1;
            t = std::array::from_fn(|c| {
                let f = t[c] * (n - 1) as f32;
                let i = (f.floor() as usize).min(n - 2);
                let d = f - i as f32;
                self.table1[i][c] * (1.0 - d) + self.table1[i + 1][c] * d
            });
            if self.size3 == 0 {
                return t;
            }
            // the shaper's output is the 3-D table's input, in the same domain
            for v in t.iter_mut() {
                *v = v.clamp(0.0, 1.0);
            }
        }
        let n = self.size3;
        let scale = (n - 1) as f32;
        let (fr, fg, fb) = (t[0] * scale, t[1] * scale, t[2] * scale);
        let (r0, g0, b0) =
            ((fr.floor() as usize).min(n - 2), (fg.floor() as usize).min(n - 2), (fb.floor() as usize).min(n - 2));
        let (dr, dg, db) = (fr - r0 as f32, fg - g0 as f32, fb - b0 as f32);
        let c000 = self.at(r0, g0, b0);
        let c111 = self.at(r0 + 1, g0 + 1, b0 + 1);
        let mix = |terms: &[(f32, [f32; 3])]| -> [f32; 3] {
            let mut o = [0f32; 3];
            for (w, c) in terms {
                for k in 0..3 {
                    o[k] += w * c[k];
                }
            }
            o
        };
        match interp {
            Interp::Tetrahedral => {
                if dr > dg {
                    if dg > db {
                        let c100 = self.at(r0 + 1, g0, b0);
                        let c110 = self.at(r0 + 1, g0 + 1, b0);
                        mix(&[(1.0 - dr, c000), (dr - dg, c100), (dg - db, c110), (db, c111)])
                    } else if dr > db {
                        let c100 = self.at(r0 + 1, g0, b0);
                        let c101 = self.at(r0 + 1, g0, b0 + 1);
                        mix(&[(1.0 - dr, c000), (dr - db, c100), (db - dg, c101), (dg, c111)])
                    } else {
                        let c001 = self.at(r0, g0, b0 + 1);
                        let c101 = self.at(r0 + 1, g0, b0 + 1);
                        mix(&[(1.0 - db, c000), (db - dr, c001), (dr - dg, c101), (dg, c111)])
                    }
                } else if db > dg {
                    let c001 = self.at(r0, g0, b0 + 1);
                    let c011 = self.at(r0, g0 + 1, b0 + 1);
                    mix(&[(1.0 - db, c000), (db - dg, c001), (dg - dr, c011), (dr, c111)])
                } else if db > dr {
                    let c010 = self.at(r0, g0 + 1, b0);
                    let c011 = self.at(r0, g0 + 1, b0 + 1);
                    mix(&[(1.0 - dg, c000), (dg - db, c010), (db - dr, c011), (dr, c111)])
                } else {
                    let c010 = self.at(r0, g0 + 1, b0);
                    let c110 = self.at(r0 + 1, g0 + 1, b0);
                    mix(&[(1.0 - dg, c000), (dg - dr, c010), (dr - db, c110), (db, c111)])
                }
            }
            Interp::Trilinear => {
                let mut o = [0f32; 3];
                for (ib, wb) in [(0, 1.0 - db), (1, db)] {
                    for (ig, wg) in [(0, 1.0 - dg), (1, dg)] {
                        for (ir, wr) in [(0, 1.0 - dr), (1, dr)] {
                            let w = wr * wg * wb;
                            if w == 0.0 {
                                continue;
                            }
                            let c = self.at(r0 + ir, g0 + ig, b0 + ib);
                            for k in 0..3 {
                                o[k] += w * c[k];
                            }
                        }
                    }
                }
                o
            }
        }
    }
}

/// An identity 3-D LUT as `.cube` text.
#[cfg(test)]
pub fn identity_cube(n: usize) -> String {
    let mut s = format!("TITLE \"identity\"\nLUT_3D_SIZE {n}\n");
    let d = (n - 1) as f32;
    for b in 0..n {
        for g in 0..n {
            for r in 0..n {
                s.push_str(&format!("{} {} {}\n", r as f32 / d, g as f32 / d, b as f32 / d));
            }
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_round_trips_both_interpolations() {
        let lut = Lut::parse(&identity_cube(5)).unwrap();
        assert_eq!(lut.title.as_deref(), Some("identity"));
        for rgb in [[0.1f32, 0.5, 0.9], [0.33, 0.33, 0.8], [1.0, 0.0, 0.25]] {
            for interp in [Interp::Trilinear, Interp::Tetrahedral] {
                let o = lut.apply(rgb, interp);
                for k in 0..3 {
                    assert!((o[k] - rgb[k]).abs() < 1e-5, "{interp:?} {rgb:?} -> {o:?}");
                }
            }
        }
    }

    #[test]
    fn swaps_channels_and_honors_domain() {
        // a 2-point LUT that outputs (b, g, r)
        let mut s = String::from("LUT_3D_SIZE 2\nDOMAIN_MIN 0 0 0\nDOMAIN_MAX 2 2 2\n");
        for b in 0..2 {
            for g in 0..2 {
                for r in 0..2 {
                    s.push_str(&format!("{b} {g} {r}\n"));
                }
            }
        }
        let lut = Lut::parse(&s).unwrap();
        let o = lut.apply([1.0, 0.5, 0.0], Interp::Tetrahedral);
        assert!((o[0] - 0.0).abs() < 1e-6 && (o[1] - 0.25).abs() < 1e-6 && (o[2] - 0.5).abs() < 1e-6, "{o:?}");
    }

    #[test]
    fn one_d_luts() {
        let lut = Lut::parse("LUT_1D_SIZE 2\n1 1 1\n0 0 0\n").unwrap();
        let o = lut.apply([0.25, 0.5, 1.0], Interp::Tetrahedral);
        assert!((o[0] - 0.75).abs() < 1e-6 && (o[2]).abs() < 1e-6);
    }

    #[test]
    fn malformed_files_are_px3311() {
        for bad in [
            "",
            "LUT_3D_SIZE 2\n0 0 0\n",
            "LUT_3D_SIZE x\n",
            "LUT_3D_SIZE 1\n0 0 0\n",
            "LUT_3D_SIZE 2\n0 0\n",
            "LUT_3D_SIZE 2\n0 0 abc\n0 0 0\n0 0 0\n0 0 0\n0 0 0\n0 0 0\n0 0 0\n0 0 0\n",
            "LUT_3D_SIZE 2\nDOMAIN_MIN 1 1 1\nDOMAIN_MAX 0 0 0\n",
        ] {
            assert_eq!(Lut::parse(bad).unwrap_err().code, "PX3311", "{bad:?}");
        }
    }
}
