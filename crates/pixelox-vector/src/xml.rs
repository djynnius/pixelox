//! An owned, editable XML DOM for SVG files: parsed with roxmltree (namespace prefixes
//! and declarations reconstructed), serialized deterministically (attribute order kept,
//! element-only content re-indented, mixed content such as `<text>` written verbatim),
//! plus the `--where` selector subset: `tag`, `*`, `.class`, `#id`, `[attr]`,
//! `[attr=value]`, compounds (`path.icon[fill=red]`), descendant (` `) and child (`>`)
//! combinators, and `,` groups.

use pixelox_core::prelude::*;
use std::collections::BTreeSet;
use usvg::roxmltree;

pub const SVG_NS: &str = "http://www.w3.org/2000/svg";
pub const XLINK_NS: &str = "http://www.w3.org/1999/xlink";
const XML_NS: &str = "http://www.w3.org/XML/1998/namespace";

#[derive(Clone, Debug, PartialEq)]
pub enum XNode {
    El(XEl),
    Text(String),
    Comment(String),
    Pi { target: String, value: String },
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct XEl {
    /// Qualified name as written (`svg`, `inkscape:label`).
    pub name: String,
    /// Attributes in document order, `xmlns` declarations included.
    pub attrs: Vec<(String, String)>,
    pub children: Vec<XNode>,
}

/// Path to an element: child indices (over all nodes) from the root element.
pub type ElPath = Vec<usize>;

#[derive(Clone, Debug, PartialEq)]
pub struct XDoc {
    /// Comments and processing instructions before the root element.
    pub prolog: Vec<XNode>,
    pub root: XEl,
    pub epilog: Vec<XNode>,
}

impl XEl {
    pub fn new(name: &str) -> XEl {
        XEl { name: name.to_string(), attrs: Vec::new(), children: Vec::new() }
    }

    pub fn attr(mut self, k: &str, v: impl Into<String>) -> XEl {
        self.set(k, v);
        self
    }

    pub fn child(mut self, el: XEl) -> XEl {
        self.children.push(XNode::El(el));
        self
    }

    /// Local name (prefix stripped).
    pub fn local(&self) -> &str {
        local_name(&self.name)
    }

    pub fn get(&self, k: &str) -> Option<&str> {
        self.attrs.iter().find(|(n, _)| n == k).map(|(_, v)| v.as_str())
    }

    /// Set an attribute, keeping its position when it exists.
    pub fn set(&mut self, k: &str, v: impl Into<String>) {
        let v = v.into();
        match self.attrs.iter_mut().find(|(n, _)| n == k) {
            Some(a) => a.1 = v,
            None => self.attrs.push((k.to_string(), v)),
        }
    }

    pub fn remove(&mut self, k: &str) -> bool {
        let n = self.attrs.len();
        self.attrs.retain(|(a, _)| a != k);
        n != self.attrs.len()
    }

    pub fn id(&self) -> Option<&str> {
        self.get("id")
    }

    pub fn elements(&self) -> impl Iterator<Item = &XEl> {
        self.children.iter().filter_map(|c| match c {
            XNode::El(e) => Some(e),
            _ => None,
        })
    }

    pub fn has_element_children(&self) -> bool {
        self.elements().next().is_some()
    }

    /// Concatenated text of this element and its descendants.
    pub fn text_content(&self) -> String {
        let mut s = String::new();
        for c in &self.children {
            match c {
                XNode::Text(t) => s.push_str(t),
                XNode::El(e) => s.push_str(&e.text_content()),
                _ => {}
            }
        }
        s
    }

    pub fn set_text(&mut self, t: &str) {
        self.children = vec![XNode::Text(t.to_string())];
    }

    pub fn push(&mut self, el: XEl) {
        self.children.push(XNode::El(el));
    }

    fn walk_into<'a>(&'a self, path: &mut ElPath, out: &mut Vec<(ElPath, &'a XEl)>) {
        out.push((path.clone(), self));
        for (i, c) in self.children.iter().enumerate() {
            if let XNode::El(e) = c {
                path.push(i);
                e.walk_into(path, out);
                path.pop();
            }
        }
    }
}

pub fn local_name(n: &str) -> &str {
    n.rsplit(':').next().unwrap_or(n)
}

fn ns_set(node: roxmltree::Node) -> Vec<(Option<String>, String)> {
    node.namespaces()
        .filter(|ns| ns.name() != Some("xml"))
        .map(|ns| (ns.name().map(String::from), ns.uri().to_string()))
        .collect()
}

fn prefixed(node: roxmltree::Node, uri: &str) -> Option<String> {
    if uri == XML_NS {
        return Some("xml".into());
    }
    node.namespaces().find(|ns| ns.uri() == uri && ns.name().is_some()).and_then(|ns| ns.name().map(String::from))
}

fn convert(node: roxmltree::Node, parent_ns: &[(Option<String>, String)]) -> XEl {
    let tn = node.tag_name();
    let name = match tn.namespace() {
        None => tn.name().to_string(),
        Some(uri) if node.lookup_namespace_uri(None) == Some(uri) => tn.name().to_string(),
        Some(uri) => match prefixed(node, uri) {
            Some(p) => format!("{p}:{}", tn.name()),
            None => tn.name().to_string(),
        },
    };
    let mine = ns_set(node);
    let mut attrs = Vec::new();
    for (prefix, uri) in mine.iter().filter(|n| !parent_ns.contains(n)) {
        match prefix {
            None => attrs.push(("xmlns".to_string(), uri.clone())),
            Some(p) => attrs.push((format!("xmlns:{p}"), uri.clone())),
        }
    }
    for a in node.attributes() {
        let n = match a.namespace() {
            None => a.name().to_string(),
            Some(uri) => match prefixed(node, uri) {
                Some(p) => format!("{p}:{}", a.name()),
                None => a.name().to_string(),
            },
        };
        attrs.push((n, a.value().to_string()));
    }
    let mut children = Vec::new();
    for c in node.children() {
        if c.is_element() {
            children.push(XNode::El(convert(c, &mine)));
        } else if c.is_text() {
            children.push(XNode::Text(c.text().unwrap_or("").to_string()));
        } else if c.is_comment() {
            children.push(XNode::Comment(c.text().unwrap_or("").to_string()));
        } else if let Some(pi) = c.pi() {
            children.push(XNode::Pi { target: pi.target.to_string(), value: pi.value.unwrap_or("").to_string() });
        }
    }
    XEl { name, attrs, children }
}

/// Serialization options.
#[derive(Clone, Copy, Debug)]
pub struct WriteOpts {
    /// Indent element-only content by two spaces per level.
    pub pretty: bool,
    /// Emit `<?xml version="1.0" encoding="UTF-8"?>`.
    pub decl: bool,
}

impl WriteOpts {
    pub const PRETTY: WriteOpts = WriteOpts { pretty: true, decl: true };
    pub const MINIFIED: WriteOpts = WriteOpts { pretty: false, decl: false };
}

/// Elements whose whitespace is significant: always written verbatim.
const VERBATIM: &[&str] = &["text", "tspan", "textPath", "title", "desc", "style", "script", "foreignObject", "pre"];

fn esc_attr(s: &str, out: &mut String) {
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '"' => out.push_str("&quot;"),
            '\n' => out.push_str("&#10;"),
            '\r' => out.push_str("&#13;"),
            '\t' => out.push_str("&#9;"),
            c => out.push(c),
        }
    }
}

fn esc_text(s: &str, out: &mut String) {
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            c => out.push(c),
        }
    }
}

fn is_ws(n: &XNode) -> bool {
    matches!(n, XNode::Text(t) if t.trim().is_empty())
}

fn write_node(n: &XNode, depth: usize, pretty: bool, out: &mut String) {
    match n {
        XNode::El(e) => write_el(e, depth, pretty, out),
        XNode::Text(t) => esc_text(t, out),
        XNode::Comment(c) => {
            out.push_str("<!--");
            out.push_str(&c.replace("--", "- -"));
            out.push_str("-->");
        }
        XNode::Pi { target, value } => {
            out.push_str("<?");
            out.push_str(target);
            if !value.is_empty() {
                out.push(' ');
                out.push_str(value);
            }
            out.push_str("?>");
        }
    }
}

fn write_el(e: &XEl, depth: usize, pretty: bool, out: &mut String) {
    out.push('<');
    out.push_str(&e.name);
    for (k, v) in &e.attrs {
        out.push(' ');
        out.push_str(k);
        out.push_str("=\"");
        esc_attr(v, out);
        out.push('"');
    }
    let element_only =
        !VERBATIM.contains(&e.local()) && e.children.iter().all(|c| !matches!(c, XNode::Text(_)) || is_ws(c));
    let kids: Vec<&XNode> =
        if element_only { e.children.iter().filter(|c| !is_ws(c)).collect() } else { e.children.iter().collect() };
    if kids.is_empty() {
        out.push_str("/>");
        return;
    }
    out.push('>');
    if element_only {
        for c in &kids {
            if pretty {
                out.push('\n');
                out.push_str(&"  ".repeat(depth + 1));
            }
            write_node(c, depth + 1, pretty, out);
        }
        if pretty {
            out.push('\n');
            out.push_str(&"  ".repeat(depth));
        }
    } else {
        // mixed content: whitespace is meaningful, write it exactly
        for c in &kids {
            write_node(c, depth + 1, false, out);
        }
    }
    out.push_str("</");
    out.push_str(&e.name);
    out.push('>');
}

impl XDoc {
    /// Parse SVG/XML text (DTDs allowed; entities expanded; CDATA becomes text).
    pub fn parse(text: &str) -> Result<XDoc> {
        let text = text.strip_prefix('\u{feff}').unwrap_or(text);
        let opts = roxmltree::ParsingOptions { allow_dtd: true, ..Default::default() };
        let d = roxmltree::Document::parse_with_options(text, opts).map_err(|e| {
            Error::new("PX1103", format!("not well-formed XML: {e}")).with_hint("check the file is an SVG document")
        })?;
        let mut prolog = Vec::new();
        let mut epilog = Vec::new();
        let mut root = None;
        for c in d.root().children() {
            let node = if c.is_element() {
                root = Some(convert(c, &[]));
                continue;
            } else if c.is_comment() {
                XNode::Comment(c.text().unwrap_or("").to_string())
            } else if let Some(pi) = c.pi() {
                XNode::Pi { target: pi.target.to_string(), value: pi.value.unwrap_or("").to_string() }
            } else {
                continue;
            };
            if root.is_none() {
                prolog.push(node);
            } else {
                epilog.push(node);
            }
        }
        let root = root.ok_or_else(|| Error::new("PX1103", "the document has no root element"))?;
        Ok(XDoc { prolog, root, epilog })
    }

    /// Parse and require an `<svg>` root (PX1105 otherwise).
    pub fn parse_svg(text: &str) -> Result<XDoc> {
        let d = XDoc::parse(text)?;
        if d.root.local() != "svg" {
            return Err(Error::new("PX1105", format!("the root element is <{}>, not <svg>", d.root.name)));
        }
        Ok(d)
    }

    pub fn new(root: XEl) -> XDoc {
        XDoc { prolog: Vec::new(), root, epilog: Vec::new() }
    }

    pub fn serialize(&self, o: WriteOpts) -> String {
        let mut out = String::new();
        if o.decl {
            out.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        }
        for n in &self.prolog {
            write_node(n, 0, false, &mut out);
            if o.pretty {
                out.push('\n');
            }
        }
        write_el(&self.root, 0, o.pretty, &mut out);
        for n in &self.epilog {
            if o.pretty {
                out.push('\n');
            }
            write_node(n, 0, false, &mut out);
        }
        out.push('\n');
        out
    }

    pub fn el(&self, path: &[usize]) -> &XEl {
        let mut cur = &self.root;
        for &i in path {
            cur = match &cur.children[i] {
                XNode::El(e) => e,
                _ => panic!("element path through a non-element"),
            };
        }
        cur
    }

    pub fn el_mut(&mut self, path: &[usize]) -> &mut XEl {
        let mut cur = &mut self.root;
        for &i in path {
            cur = match &mut cur.children[i] {
                XNode::El(e) => e,
                _ => panic!("element path through a non-element"),
            };
        }
        cur
    }

    /// Every element with its path, in document order (the root first).
    pub fn walk(&self) -> Vec<(ElPath, &XEl)> {
        let mut out = Vec::new();
        self.root.walk_into(&mut Vec::new(), &mut out);
        out
    }

    pub fn find_id(&self, id: &str) -> Option<ElPath> {
        self.walk().into_iter().find(|(_, e)| e.id() == Some(id)).map(|(p, _)| p)
    }

    pub fn ids(&self) -> BTreeSet<String> {
        self.walk().into_iter().filter_map(|(_, e)| e.id().map(String::from)).collect()
    }

    /// Remove the element at `path` (not the root).
    pub fn remove(&mut self, path: &[usize]) -> XNode {
        let (last, parent) = path.split_last().expect("not the root");
        self.el_mut(parent).children.remove(*last)
    }

    /// Paths of the elements matching `sel`, in document order.
    pub fn select(&self, sel: &Selector) -> Vec<ElPath> {
        let mut out = Vec::new();
        let mut chain: Vec<&XEl> = Vec::new();
        fn rec<'a>(e: &'a XEl, path: &mut ElPath, chain: &mut Vec<&'a XEl>, sel: &Selector, out: &mut Vec<ElPath>) {
            chain.push(e);
            if sel.matches(chain) {
                out.push(path.clone());
            }
            for (i, c) in e.children.iter().enumerate() {
                if let XNode::El(ch) = c {
                    path.push(i);
                    rec(ch, path, chain, sel, out);
                    path.pop();
                }
            }
            chain.pop();
        }
        rec(&self.root, &mut Vec::new(), &mut chain, sel, &mut out);
        out
    }

    /// Declare a namespace prefix on the root when an attribute or tag uses it.
    pub fn ensure_xlink(&mut self) {
        let uses = self
            .walk()
            .iter()
            .any(|(_, e)| e.name.starts_with("xlink:") || e.attrs.iter().any(|(k, _)| k.starts_with("xlink:")));
        if uses && self.root.get("xmlns:xlink").is_none() {
            self.root.set("xmlns:xlink", XLINK_NS);
        }
    }
}

// ---------------------------------------------------------------------------------------
// selectors

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Compound {
    pub tag: Option<String>,
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub attrs: Vec<(String, Option<String>)>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Combinator {
    Descendant,
    Child,
}

/// One comma-separated alternative: compounds left to right; `rel[i]` joins `parts[i]`
/// and `parts[i + 1]`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Complex {
    pub parts: Vec<Compound>,
    pub rel: Vec<Combinator>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Selector {
    pub groups: Vec<Complex>,
}

fn sel_err(s: &str, why: &str) -> Error {
    Error::new("PX1010", format!("--where: cannot parse selector '{s}': {why}"))
        .with_hint("supported: tag, *, .class, #id, [attr], [attr=value], 'g.axis text', 'g > rect', 'a, b'")
        .with_context("selector", s)
}

fn is_ident(c: char) -> bool {
    c.is_alphanumeric() || matches!(c, '-' | '_' | ':' | '\\') || !c.is_ascii()
}

impl Selector {
    pub fn parse(s: &str) -> Result<Selector> {
        let chars: Vec<char> = s.chars().collect();
        let mut groups = Vec::new();
        let mut cx = Complex::default();
        let mut cur: Option<Compound> = None;
        let mut pending: Option<Combinator> = None;
        let mut i = 0;
        let ident = |i: &mut usize| -> String {
            let st = *i;
            while *i < chars.len() && is_ident(chars[*i]) {
                *i += 1;
            }
            chars[st..*i].iter().collect()
        };
        let finish = |cur: &mut Option<Compound>, cx: &mut Complex, pending: &mut Option<Combinator>| {
            if let Some(c) = cur.take() {
                if !cx.parts.is_empty() {
                    cx.rel.push(pending.take().unwrap_or(Combinator::Descendant));
                }
                cx.parts.push(c);
            }
        };
        while i < chars.len() {
            let c = chars[i];
            match c {
                ' ' | '\t' | '\n' => {
                    finish(&mut cur, &mut cx, &mut pending);
                    i += 1;
                }
                '>' => {
                    finish(&mut cur, &mut cx, &mut pending);
                    if cx.parts.is_empty() {
                        return Err(sel_err(s, "'>' needs a selector on its left"));
                    }
                    pending = Some(Combinator::Child);
                    i += 1;
                }
                ',' => {
                    finish(&mut cur, &mut cx, &mut pending);
                    if cx.parts.is_empty() || pending.is_some() {
                        return Err(sel_err(s, "empty selector before ','"));
                    }
                    groups.push(std::mem::take(&mut cx));
                    i += 1;
                }
                '*' => {
                    cur.get_or_insert_with(Compound::default);
                    i += 1;
                }
                '#' | '.' => {
                    i += 1;
                    let name = ident(&mut i);
                    if name.is_empty() {
                        return Err(sel_err(s, &format!("'{c}' needs a name")));
                    }
                    let comp = cur.get_or_insert_with(Compound::default);
                    if c == '#' {
                        comp.id = Some(name);
                    } else {
                        comp.classes.push(name);
                    }
                }
                '[' => {
                    let close = chars[i..].iter().position(|&x| x == ']').ok_or_else(|| sel_err(s, "missing ']'"))? + i;
                    let inner: String = chars[i + 1..close].iter().collect();
                    let (k, v) = match inner.split_once('=') {
                        Some((k, v)) => {
                            let v = v.trim();
                            let v = v
                                .strip_prefix('"')
                                .and_then(|x| x.strip_suffix('"'))
                                .or_else(|| v.strip_prefix('\'').and_then(|x| x.strip_suffix('\'')))
                                .unwrap_or(v);
                            (k.trim().to_string(), Some(v.to_string()))
                        }
                        None => (inner.trim().to_string(), None),
                    };
                    if k.is_empty() || !k.chars().all(is_ident) {
                        return Err(sel_err(s, &format!("bad attribute test [{inner}]")));
                    }
                    cur.get_or_insert_with(Compound::default).attrs.push((k, v));
                    i = close + 1;
                }
                c if is_ident(c) => {
                    if cur.as_ref().map(|c| c != &Compound::default()).unwrap_or(false) {
                        return Err(sel_err(s, "a tag name must come first in a compound"));
                    }
                    let name = ident(&mut i);
                    cur.get_or_insert_with(Compound::default).tag = Some(name);
                }
                other => return Err(sel_err(s, &format!("unsupported character '{other}'"))),
            }
        }
        finish(&mut cur, &mut cx, &mut pending);
        if pending.is_some() {
            return Err(sel_err(s, "dangling '>'"));
        }
        if cx.parts.is_empty() {
            return Err(sel_err(s, "empty selector"));
        }
        groups.push(cx);
        Ok(Selector { groups })
    }

    /// Does the last element of `chain` (ancestors first) match?
    pub fn matches(&self, chain: &[&XEl]) -> bool {
        self.groups.iter().any(|g| match_at(g, g.parts.len() - 1, chain, chain.len() - 1))
    }
}

impl Compound {
    fn matches(&self, e: &XEl) -> bool {
        if let Some(t) = &self.tag {
            if e.local() != t && &e.name != t {
                return false;
            }
        }
        if let Some(id) = &self.id {
            if e.id() != Some(id.as_str()) {
                return false;
            }
        }
        if !self.classes.is_empty() {
            let cls: Vec<&str> = e.get("class").map(|c| c.split_whitespace().collect()).unwrap_or_default();
            if !self.classes.iter().all(|c| cls.contains(&c.as_str())) {
                return false;
            }
        }
        self.attrs.iter().all(|(k, v)| match (e.get(k), v) {
            (None, _) => false,
            (Some(_), None) => true,
            (Some(a), Some(v)) => a == v,
        })
    }
}

fn match_at(g: &Complex, ci: usize, chain: &[&XEl], ei: usize) -> bool {
    if !g.parts[ci].matches(chain[ei]) {
        return false;
    }
    if ci == 0 {
        return true;
    }
    match g.rel[ci - 1] {
        Combinator::Child => ei > 0 && match_at(g, ci - 1, chain, ei - 1),
        Combinator::Descendant => (0..ei).rev().any(|j| match_at(g, ci - 1, chain, j)),
    }
}

/// Is `id` a valid XML id (NCName)?
pub fn valid_id(id: &str) -> bool {
    let mut cs = id.chars();
    match cs.next() {
        Some(c) if c.is_alphabetic() || c == '_' => {}
        _ => return false,
    }
    cs.all(|c| c.is_alphanumeric() || matches!(c, '-' | '_' | '.'))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r##"<?xml version="1.0"?>
<!-- generator -->
<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="100" height="50">
  <g class="axis main" id="ax">
    <text x="1" y="2">a &amp; <tspan>b</tspan> </text>
    <rect id="r1" width="10" height="5" fill="red"/>
  </g>
  <use xlink:href="#r1" x="5"/>
</svg>"##;

    #[test]
    fn parse_serialize_round_trip_is_stable() {
        let d = XDoc::parse(SAMPLE).unwrap();
        assert_eq!(d.root.get("xmlns"), Some(SVG_NS));
        assert_eq!(d.root.get("xmlns:xlink"), Some(XLINK_NS));
        let out = d.serialize(WriteOpts::PRETTY);
        assert!(out.contains("<use xlink:href=\"#r1\" x=\"5\"/>"), "{out}");
        assert!(out.contains("<text x=\"1\" y=\"2\">a &amp; <tspan>b</tspan> </text>"), "{out}");
        assert!(out.contains("<!-- generator -->"));
        let again = XDoc::parse(&out).unwrap();
        assert_eq!(again, d);
        assert_eq!(again.serialize(WriteOpts::PRETTY), out);
    }

    #[test]
    fn selectors_match_the_subset() {
        let d = XDoc::parse(SAMPLE).unwrap();
        let n = |s: &str| d.select(&Selector::parse(s).unwrap()).len();
        assert_eq!(n("rect"), 1);
        assert_eq!(n("g.axis text"), 1);
        assert_eq!(n("g.axis.main > rect"), 1);
        assert_eq!(n("svg > rect"), 0);
        assert_eq!(n("#r1"), 1);
        assert_eq!(n("[fill=red]"), 1);
        assert_eq!(n("[fill='red']"), 1);
        assert_eq!(n("[x]"), 2);
        assert_eq!(n("rect, use"), 2);
        assert_eq!(n("svg tspan"), 1);
        assert_eq!(n("*"), 6);
        assert!(Selector::parse("a >").is_err());
        assert!(Selector::parse("[x").is_err());
        assert!(Selector::parse("a ~ b").is_err());
    }

    #[test]
    fn ids_are_ncnames() {
        assert!(valid_id("logo-1"));
        assert!(!valid_id("1abc"));
        assert!(!valid_id("a b"));
    }
}
