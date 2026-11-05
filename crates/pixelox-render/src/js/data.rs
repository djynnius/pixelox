//! `--data name=file` parsing: CSV/TSV become arrays of row objects (string values, like
//! `d3.csvParse`, with a `columns` property), JSON is parsed, anything else is text.

use pixelox_core::error::{Error, Result};
use serde_json::{json, Map, Value};
use std::path::Path;

/// RFC 4180 fields: quoted fields, doubled quotes, CRLF/LF/CR line ends, a leading BOM.
pub fn parse_delimited(text: &str, delim: char) -> Vec<Vec<String>> {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let mut rows = Vec::new();
    let mut row: Vec<String> = Vec::new();
    let mut field = String::new();
    let mut chars = text.chars().peekable();
    let mut quoted = false;
    let mut any = false;
    while let Some(c) = chars.next() {
        any = true;
        if quoted {
            if c == '"' {
                if chars.peek() == Some(&'"') {
                    field.push('"');
                    chars.next();
                } else {
                    quoted = false;
                }
            } else {
                field.push(c);
            }
            continue;
        }
        match c {
            '"' if field.is_empty() => quoted = true,
            c if c == delim => row.push(std::mem::take(&mut field)),
            '\r' | '\n' => {
                if c == '\r' && chars.peek() == Some(&'\n') {
                    chars.next();
                }
                row.push(std::mem::take(&mut field));
                rows.push(std::mem::take(&mut row));
                any = false;
            }
            c => field.push(c),
        }
    }
    if any {
        row.push(field);
        rows.push(row);
    }
    rows
}

/// Rows of a delimited table as JSON objects keyed by the header row.
pub fn table_json(text: &str, delim: char) -> (Value, Vec<String>) {
    let mut rows = parse_delimited(text, delim).into_iter();
    let columns: Vec<String> = rows.next().unwrap_or_default();
    let mut out = Vec::new();
    for r in rows {
        if r.len() == 1 && r[0].is_empty() {
            continue; // blank line
        }
        let mut m = Map::new();
        for (i, col) in columns.iter().enumerate() {
            m.insert(col.clone(), Value::String(r.get(i).cloned().unwrap_or_default()));
        }
        out.push(Value::Object(m));
    }
    (Value::Array(out), columns)
}

/// The `cfg.data` entry for one file: `{kind, text, columns?}`.
pub fn data_entry(path: &Path, bytes: &[u8]) -> Result<Value> {
    let text = String::from_utf8(bytes.to_vec()).map_err(|_| {
        Error::new("PX1010", format!("--data {}: not UTF-8 text", path.display()))
            .with_context("path", path.display().to_string())
    })?;
    let ext = path.extension().map(|e| e.to_string_lossy().to_ascii_lowercase()).unwrap_or_default();
    Ok(match ext.as_str() {
        "csv" | "tsv" => {
            let (rows, columns) = table_json(&text, if ext == "tsv" { '\t' } else { ',' });
            json!({"kind": "table", "text": rows.to_string(), "columns": columns})
        }
        "json" | "geojson" | "topojson" => {
            serde_json::from_str::<Value>(&text).map_err(|e| {
                Error::new("PX1010", format!("--data {}: invalid JSON: {e}", path.display()))
                    .with_context("path", path.display().to_string())
            })?;
            json!({"kind": "json", "text": text})
        }
        _ => json!({"kind": "text", "text": text}),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csv_quotes_and_line_ends() {
        let rows = parse_delimited("a,b\r\n\"x, y\",\"he said \"\"hi\"\"\"\n1,2", ',');
        assert_eq!(rows, vec![vec!["a", "b"], vec!["x, y", "he said \"hi\""], vec!["1", "2"]]);
        let (v, cols) = table_json("\u{feff}month,sales\nJan,10\nFeb,20\n", ',');
        assert_eq!(cols, vec!["month", "sales"]);
        assert_eq!(v[1]["sales"], "20");
        assert_eq!(v.as_array().unwrap().len(), 2);
    }
}
