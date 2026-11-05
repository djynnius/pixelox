//! Layers (layer *), composition, templates, and placed files (comp *) for pixelox.
//!
//! * `layer *` (PRD §8.3): list, add, dup, rm, rename, reorder, group, ungroup, merge,
//!   flatten, opacity, blend, show, hide, lock, unlock, clip, rasterize.
//! * `comp *` (PRD §8.14): place, relink, update-links, align, distribute, guides, grid,
//!   templates (init, slots, apply), and canvas presets.
//!
//! Locks: a locked layer cannot be removed, merged, moved, restyled (opacity, blend,
//! clipping), rasterized, relinked, or filled by a template (PX2304); renaming,
//! visibility, restacking, and grouping still work.

use pixelox_core::prelude::*;

mod arrange;
mod layers;
mod place;
mod template;
#[cfg(test)]
mod testio;
mod util;

/// Every command this crate defines.
pub fn ops() -> Vec<OpSpec> {
    let mut v = layers::ops();
    v.extend(place::ops());
    v.extend(arrange::ops());
    v.extend(template::ops());
    v
}

/// Install renderer hooks (if any) into the compositor. This crate renders nothing itself.
pub fn install() {}

#[cfg(test)]
mod tests {
    use super::*;

    const GLOBAL_FLAGS: &[&str] = &[
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

    const COMMANDS: &[&str] = &[
        "layer list",
        "layer add",
        "layer dup",
        "layer rm",
        "layer rename",
        "layer reorder",
        "layer group",
        "layer ungroup",
        "layer merge",
        "layer flatten",
        "layer opacity",
        "layer blend",
        "layer show",
        "layer hide",
        "layer lock",
        "layer unlock",
        "layer clip",
        "layer rasterize",
        "comp place",
        "comp relink",
        "comp update-links",
        "comp align",
        "comp distribute",
        "comp guide add",
        "comp guide list",
        "comp guide rm",
        "comp guide clear",
        "comp grid set",
        "comp template init",
        "comp template slots",
        "comp template apply",
        "comp presets",
        "layer blend-if",
        "layer stamp",
        "layer smart-filter",
    ];

    #[test]
    fn every_command_is_defined_once() {
        let ops = ops();
        let names: Vec<&str> = ops.iter().map(|o| o.name).collect();
        for c in COMMANDS {
            assert_eq!(names.iter().filter(|n| *n == c).count(), 1, "{c}");
        }
        assert_eq!(names.len(), COMMANDS.len());
    }

    /// Shell-like split that honors single and double quotes.
    fn split(cmd: &str) -> Vec<String> {
        let mut out = Vec::new();
        let mut cur = String::new();
        let mut quote: Option<char> = None;
        let mut any = false;
        for ch in cmd.chars() {
            match (quote, ch) {
                (Some(q), c) if c == q => quote = None,
                (Some(_), c) => cur.push(c),
                (None, '\'' | '"') => {
                    quote = Some(ch);
                    any = true;
                }
                (None, c) if c.is_whitespace() => {
                    if any || !cur.is_empty() {
                        out.push(std::mem::take(&mut cur));
                    }
                    any = false;
                }
                (None, c) => cur.push(c),
            }
        }
        if any || !cur.is_empty() {
            out.push(cur);
        }
        out
    }

    #[test]
    fn specs_follow_the_authoring_contract() {
        for op in ops() {
            assert!(
                !op.summary.is_empty() && op.summary.chars().count() <= 60,
                "{}: summary '{}'",
                op.name,
                op.summary
            );
            assert!(!op.summary.ends_with('.'), "{}: summary ends with a period", op.name);
            assert!(!op.about.is_empty(), "{}: no about", op.name);
            assert!(op.examples.len() >= 3, "{}: fewer than three examples", op.name);
            for p in &op.params {
                assert!(!p.help.is_empty(), "{} --{}: no help", op.name, p.name);
                assert!(!GLOBAL_FLAGS.contains(&p.name), "{} --{}: shadows a global flag", op.name, p.name);
            }
            for pos in &op.positionals {
                assert!(op.find_param(pos).is_some(), "{}: positional {pos} is not a param", op.name);
            }
            for ex in &op.examples {
                let words = split(ex.cmd);
                let prefix: Vec<&str> = std::iter::once("pixelox").chain(op.name.split(' ')).collect();
                assert_eq!(&words[..prefix.len()], &prefix[..], "{}: example '{}'", op.name, ex.cmd);
                assert!(!ex.explain.is_empty());
                // flags are params or globals, and params get values that validate
                let mut positional = 0;
                let mut i = prefix.len();
                let mut pairs: Vec<(String, String)> = Vec::new();
                while i < words.len() {
                    let w = &words[i];
                    if w == "-o" || w == "-q" || w == "-v" || w == "-y" {
                        i += if w == "-o" { 2 } else { 1 };
                        continue;
                    }
                    if let Some(flag) = w.strip_prefix("--") {
                        let (flag, inline) = match flag.split_once('=') {
                            Some((f, v)) => (f, Some(v.to_string())),
                            None => (flag, None),
                        };
                        if GLOBAL_FLAGS.contains(&flag) {
                            let takes_value = !matches!(
                                flag,
                                "json"
                                    | "dry-run"
                                    | "overwrite"
                                    | "in-place"
                                    | "yes"
                                    | "quiet"
                                    | "verbose"
                                    | "no-sidecar"
                                    | "background"
                                    | "redact-paths"
                            );
                            i += if takes_value && inline.is_none() { 2 } else { 1 };
                            continue;
                        }
                        let p = op
                            .find_param(flag)
                            .unwrap_or_else(|| panic!("{}: example uses unknown --{flag}: {}", op.name, ex.cmd));
                        if p.kind == ParamKind::Flag {
                            pairs.push((flag.to_string(), "true".into()));
                            i += 1;
                        } else {
                            let v = inline.clone().unwrap_or_else(|| words[i + 1].clone());
                            pairs.push((flag.to_string(), v));
                            i += if inline.is_some() { 1 } else { 2 };
                        }
                        continue;
                    }
                    // positionals: the input files, then the declared positional params
                    let inputs = op.inputs.max.unwrap_or(usize::MAX);
                    if positional >= inputs {
                        let k = positional - inputs;
                        let name = op
                            .positionals
                            .get(k)
                            .unwrap_or_else(|| panic!("{}: stray word '{w}' in '{}'", op.name, ex.cmd));
                        pairs.push((name.to_string(), w.clone()));
                    }
                    positional += 1;
                    i += 1;
                }
                assert!(positional >= op.inputs.min, "{}: example lacks an input: {}", op.name, ex.cmd);
                Args::build(&op, pairs)
                    .unwrap_or_else(|e| panic!("{}: example '{}' fails: {}", op.name, ex.cmd, e.message));
            }
        }
    }
}
