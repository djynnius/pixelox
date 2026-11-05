//! Behavior tests for the render commands, run through their op specs with an in-memory
//! IO layer (encoded images are captured; `.pxo` documents live in a map).

use super::*;
use pixelox_core::ctx::{Globals, ImageInfo, ImageIo, NoProgress, OpenOptions};
use pixelox_core::envelope::Warning;
use pixelox_core::testing::{self, TempDir};
use pixelox_core::Runner;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::Instant;

#[derive(Default)]
struct MockIo {
    images: Mutex<Vec<Image>>,
    docs: Mutex<BTreeMap<PathBuf, Document>>,
}

impl ImageIo for MockIo {
    fn open(&self, path: &Path, _: &OpenOptions) -> Result<(Document, Vec<Warning>)> {
        let docs = self.docs.lock().unwrap();
        let d = docs
            .get(path)
            .cloned()
            .ok_or_else(|| Error::unsupported(format!("mock cannot open {}", path.display())))?;
        Ok((d, vec![]))
    }
    fn decode(&self, _: &[u8], _: Option<&str>, _: &OpenOptions) -> Result<(Image, ImageInfo)> {
        Err(Error::unsupported("mock decode"))
    }
    fn probe(&self, _: &Path) -> Result<ImageInfo> {
        Err(Error::unsupported("mock probe"))
    }
    fn encode(&self, image: &Image, _: Option<&Document>, path: &Path, _: &EncodeOptions) -> Result<Vec<Warning>> {
        std::fs::write(path, b"img")?;
        self.images.lock().unwrap().push(image.clone());
        Ok(vec![])
    }
    fn save_pxo(&self, doc: &Document, path: &Path) -> Result<()> {
        std::fs::write(path, b"pxo")?;
        // stored under the final name (the executor renames the temp file)
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        let real = name
            .trim_start_matches('.')
            .split(".part")
            .next()
            .unwrap_or(&name)
            .rsplit_once('.')
            .map(|(a, _)| a.to_string())
            .unwrap_or(name.clone());
        self.docs.lock().unwrap().insert(path.with_file_name(real), doc.clone());
        Ok(())
    }
    fn formats(&self) -> (Vec<&'static str>, Vec<&'static str>) {
        (vec!["png"], vec!["png"])
    }
}

struct Run {
    value: Value,
    warnings: Vec<String>,
}

fn run_with(name: &str, inputs: Vec<PathBuf>, pairs: &[(&str, &str)], g: Globals, io: &MockIo) -> Result<Run> {
    let spec = testing::find(&ops(), name);
    let a = testing::args(&spec, pairs)?;
    let mut ctx = OpCtx::new(spec.name, inputs, g, io, &NoProgress);
    let value = match spec.runner {
        Runner::Custom(f) => f(&a, &mut ctx)?,
        _ => panic!("{name} is not a custom op"),
    };
    Ok(Run { value, warnings: ctx.warnings.iter().map(|w| w.code.clone()).collect() })
}

fn globals(out: &Path) -> Globals {
    Globals { output: Some(out.display().to_string()), ..Default::default() }
}

/// Run `render js` on an inline script; returns the result and the output path.
fn js(dir: &TempDir, script: &str, out: &str, pairs: &[(&str, &str)], io: &MockIo) -> Result<(Run, PathBuf)> {
    let sp = dir.path("script.js");
    std::fs::write(&sp, script).unwrap();
    let out = dir.path(out);
    let _ = std::fs::remove_file(&out);
    let r = run_with("render js", vec![sp], pairs, globals(&out), io)?;
    Ok((r, out))
}

fn last_image(io: &MockIo) -> Image {
    io.images.lock().unwrap().last().cloned().expect("an encoded image")
}

#[test]
fn canvas_rect_draws_expected_pixels() {
    let dir = TempDir::new("render-rect");
    let io = MockIo::default();
    let script =
        "const c = px.canvas(20, 10);\nc.fillStyle = 'rgb(255, 0, 0)';\nc.fillRect(5, 2, 10, 4);\npx.output(c);\n";
    let (r, out) = js(&dir, script, "rect.png", &[], &io).unwrap();
    assert!(out.exists());
    assert_eq!(r.value["kind"], "canvas");
    let img = last_image(&io);
    assert_eq!((img.width, img.height), (20, 10));
    assert_eq!(img.get(10, 4), [1.0, 0.0, 0.0, 1.0]);
    assert_eq!(img.get(5, 2), [1.0, 0.0, 0.0, 1.0]);
    assert_eq!(img.get(4, 2)[3], 0.0);
    assert_eq!(img.get(15, 4)[3], 0.0);
    assert_eq!(img.get(10, 6)[3], 0.0);
}

#[test]
fn canvas_paths_transforms_and_gradients() {
    let dir = TempDir::new("render-paths");
    let io = MockIo::default();
    let script = r#"
        const c = px.canvas(100, 100);
        c.save();
        c.translate(50, 50);
        c.scale(2, 2);
        c.beginPath(); c.arc(0, 0, 20, 0, Math.PI * 2); c.fillStyle = '#00ff00'; c.fill();
        c.restore();
        const g = c.createLinearGradient(0, 0, 100, 0);
        g.addColorStop(0, 'black'); g.addColorStop(1, 'white');
        c.fillStyle = g; c.globalAlpha = 1; c.fillRect(0, 95, 100, 5);
        c.lineWidth = 4; c.strokeStyle = 'blue'; c.beginPath(); c.moveTo(0, 2); c.lineTo(100, 2); c.stroke();
        c.beginPath(); c.ellipse(10, 50, 5, 3, 0, 0, Math.PI * 2); c.bezierCurveTo(1, 1, 2, 2, 3, 3); c.quadraticCurveTo(1, 2, 3, 4);
        c.rect(1, 1, 2, 2); c.closePath();
        px.output(c.canvas);
    "#;
    js(&dir, script, "paths.png", &[], &io).unwrap();
    let img = last_image(&io);
    // circle of radius 40 around the center (scaled by 2)
    assert_eq!(img.get(50, 50), [0.0, 1.0, 0.0, 1.0]);
    assert_eq!(img.get(50, 88)[1], 1.0);
    assert_eq!(img.get(50, 93)[3], 0.0);
    // gradient ramps left to right
    assert!(img.get(5, 97)[0] < 0.1 && img.get(95, 97)[0] > 0.9);
    // stroke of width 4 centered on y = 2
    assert_eq!(img.get(50, 1)[2], 1.0);
    assert_eq!(img.get(50, 5)[3], 0.0);
}

const MINI_D3: &str = r#"
(function (global) {
  function Selection(nodes, parent) { this.nodes = nodes; this.parent = parent; }
  Selection.prototype.selectAll = function (sel) {
    const p = this.nodes[0];
    return new Selection(p.querySelectorAll(sel), p);
  };
  Selection.prototype.data = function (data) {
    const nodes = this.nodes.slice(0, data.length);
    nodes.forEach((n, i) => { n.__data__ = data[i]; });
    const sel = new Selection(nodes, this.parent);
    sel._enter = data.slice(nodes.length);
    sel._exit = this.nodes.slice(data.length);
    return sel;
  };
  Selection.prototype.join = function (name) {
    const p = this.parent;
    this._exit.forEach((n) => p.removeChild(n));
    const made = this._enter.map((d) => {
      const n = p.ownerDocument.createElementNS(p.namespaceURI, name);
      n.__data__ = d;
      return p.appendChild(n);
    });
    return new Selection(this.nodes.concat(made), p);
  };
  Selection.prototype.attr = function (k, v) {
    this.nodes.forEach((n, i) => n.setAttribute(k, typeof v === 'function' ? v(n.__data__, i) : v));
    return this;
  };
  Selection.prototype.append = function (name) {
    return new Selection(this.nodes.map((n) => n.appendChild(n.ownerDocument.createElementNS(n.namespaceURI, name))), null);
  };
  Selection.prototype.text = function (t) { this.nodes.forEach((n) => { n.textContent = t; }); return this; };
  Selection.prototype.call = function (f) { f(this); return this; };
  global.d3 = { select: (node) => new Selection([node], null), version: 'mini' };
})(globalThis);
"#;

const CHART: &str = r#"
const rows = px.data.rows;
const doc = px.svg(300, 200);
const sel = d3.select(doc.documentElement);
sel.selectAll('rect').data(rows).join('rect')
  .attr('x', (r, i) => 10 + i * 40).attr('y', (r) => 200 - +r.sales)
  .attr('width', 30).attr('height', (r) => +r.sales).attr('fill', '#0ea5e9');
sel.append('text').text(rows.columns.join('|') + ' & <done>');
px.output(doc);
"#;

#[test]
fn d3_preset_with_local_bundle_joins_rows_into_rects() {
    let dir = TempDir::new("render-d3");
    let io = MockIo::default();
    std::fs::write(dir.path("d3.min.js"), MINI_D3).unwrap();
    std::fs::write(dir.path("sales.csv"), "month,sales\nJan,30\nFeb,\"60\"\nMar,90\nApr,120\nMay,45\n").unwrap();
    std::fs::write(dir.path("chart.js"), CHART).unwrap();
    let out = dir.path("chart.svg");
    let lib = dir.path("d3.min.js").display().to_string();
    let data = format!("rows={}", dir.path("sales.csv").display());
    let r = run_with("render d3", vec![dir.path("chart.js")], &[("lib", &lib), ("data", &data)], globals(&out), &io)
        .unwrap();
    assert_eq!(r.value["kind"], "svg");
    assert_eq!(r.value["preset"], "d3");
    let svg = std::fs::read_to_string(&out).unwrap();
    assert!(
        svg.starts_with(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"300\" height=\"200\" viewBox=\"0 0 300 200\">"
        ),
        "{svg}"
    );
    assert_eq!(svg.matches("<rect").count(), 5, "{svg}");
    assert!(svg.contains("<rect x=\"50\" y=\"140\" width=\"30\" height=\"60\" fill=\"#0ea5e9\"/>"), "{svg}");
    assert!(svg.contains("<text>month|sales &amp; &lt;done&gt;</text>"), "{svg}");
    // the SVG parses (usvg) and rasterizes
    let pm = rasterize_svg(&svg, None).unwrap();
    assert_eq!((pm.width(), pm.height()), (300, 200));
}

#[test]
fn dom_lite_join_update_exit_and_selectors() {
    let dir = TempDir::new("render-dom");
    let io = MockIo::default();
    let script = r#"
        const doc = px.svg(100, 100);
        const root = doc.documentElement;
        const g = root.appendChild(doc.createElementNS('http://www.w3.org/2000/svg', 'g'));
        g.setAttribute('class', 'bars main');
        // a d3-style keyed join written by hand: enter 6, then update to 4 (exit 2)
        function join(data) {
          const old = g.querySelectorAll('rect.bar');
          data.forEach((d, i) => {
            let n = old[i];
            if (!n) { n = doc.createElementNS(root.namespaceURI, 'rect'); n.classList.add('bar'); g.insertBefore(n, null); }
            n.setAttribute('width', d);
          });
          old.slice(data.length).forEach((n) => n.remove());
        }
        join([1, 2, 3, 4, 5, 6]);
        join([7, 8, 9, 10]);
        const first = g.firstChild, last = g.lastChild;
        const checks = [
          g.querySelectorAll('rect').length === 4,
          root.querySelectorAll('g.bars > rect.bar:first-child').length === 1,
          root.querySelector('[width="9"]') === first.nextSibling.nextSibling,
          first.compareDocumentPosition(last) === 4,
          last.compareDocumentPosition(first) === 2,
          root.contains(last) && last.parentNode === g && g.ownerDocument === doc,
          g.classList.contains('main') && !g.classList.contains('nope'),
          (g.style.setProperty('fill', 'red'), g.getAttribute('style') === 'fill: red;'),
          g.style.getPropertyValue('fill') === 'red',
          g.children.length === 4 && g.childNodes.length === 4,
          root.matches('svg') && last.closest('g') === g,
        ];
        last.setAttributeNS('http://www.w3.org/1999/xlink', 'xlink:href', '#a');
        px.log(JSON.stringify(checks));
        px.output(doc);
    "#;
    let (r, out) = js(&dir, script, "dom.svg", &[], &io).unwrap();
    assert_eq!(r.value["logs"][0], "[true,true,true,true,true,true,true,true,true,true,true]");
    let svg = std::fs::read_to_string(out).unwrap();
    assert_eq!(svg.matches("<rect").count(), 4);
    assert!(svg.contains("xmlns:xlink=\"http://www.w3.org/1999/xlink\""), "{svg}");
    assert!(svg.contains("xlink:href=\"#a\""));
}

#[test]
fn forbidden_apis_are_absent() {
    let dir = TempDir::new("render-forbid");
    let io = MockIo::default();
    let script = r#"
        const kinds = [typeof eval, typeof fetch, typeof require, typeof setTimeout, typeof WebAssembly,
          typeof process, typeof performance, typeof XMLHttpRequest, typeof Worker, typeof std, typeof os];
        const blocked = (f) => { try { f(); return false; } catch (e) { return /PX6010/.test(String(e.message)); } };
        const fns = [
          blocked(() => (function () {}).constructor('return 1')),
          blocked(() => Function('return 1')),
          blocked(() => Object.getPrototypeOf(async function () {}).constructor('return 1')),
          blocked(() => Object.getPrototypeOf(function* () {}).constructor('yield 1')),
        ];
        const frozen = Object.isFrozen(px) && (() => { try { px.canvas = 1; } catch (e) {} return typeof px.canvas === 'function'; })();
        px.log(JSON.stringify({ kinds, fns, frozen, now: Date.now(), date: new Date().getTime(), tz: new Date(2020, 0, 2, 3).toISOString() }));
        px.output(px.canvas(1, 1));
    "#;
    let (r, _) = js(&dir, script, "f.png", &[], &io).unwrap();
    let log: Value = serde_json::from_str(r.value["logs"][0].as_str().unwrap()).unwrap();
    assert!(log["kinds"].as_array().unwrap().iter().all(|k| k == "undefined"), "{log}");
    assert!(log["fns"].as_array().unwrap().iter().all(|k| k == true), "{log}");
    assert_eq!(log["frozen"], true);
    assert_eq!(log["now"], 0);
    assert_eq!(log["date"], 0);
    assert_eq!(log["tz"], "2020-01-02T03:00:00.000Z");
    // an uncaught attempt is reported as a forbidden API
    let e = js(&dir, "Function('return 1')();", "g.png", &[], &io).err().unwrap();
    assert_eq!(e.code, "PX6010");
    assert_eq!(e.exit_code, 7);
}

#[test]
fn infinite_loop_hits_the_wall_clock_limit() {
    let dir = TempDir::new("render-loop");
    let io = MockIo::default();
    let t = Instant::now();
    let e = js(&dir, "let i = 0;\nwhile (true) { i++; }\n", "loop.png", &[("timeout", "200ms")], &io).err().unwrap();
    assert_eq!(e.code, "PX6002");
    assert_eq!(e.exit_code, 7);
    assert!(t.elapsed().as_secs_f64() < 3.0, "took {:?}", t.elapsed());
}

#[test]
fn memory_limit_is_enforced() {
    let dir = TempDir::new("render-mem");
    let io = MockIo::default();
    let e = js(
        &dir,
        "const a = [];\nwhile (true) a.push(new Array(100000).fill(1.5));\n",
        "m.png",
        &[("js-mem", "16")],
        &io,
    )
    .err()
    .unwrap();
    assert_eq!(e.code, "PX6003");
    let e = js(&dir, "px.canvas(4000, 4000);", "m2.png", &[("js-mem", "16")], &io).err().unwrap();
    assert_eq!(e.code, "PX6003");
}

#[test]
fn missing_output_and_script_errors() {
    let dir = TempDir::new("render-errors");
    let io = MockIo::default();
    let e = js(&dir, "const c = px.canvas(4, 4);\nc.fillRect(0, 0, 1, 1);\n", "none.png", &[], &io).err().unwrap();
    assert_eq!(e.code, "PX6020");
    assert!(e.hint.unwrap().contains("px.output"));
    let e = js(&dir, "const x = 1;\nnull.boom();\n", "err.png", &[], &io).err().unwrap();
    assert_eq!(e.code, "PX6001");
    assert_eq!(e.context["line"], 2, "{e:?}");
    assert!(e.context.contains_key("column"));
    assert!(e.message.contains("script.js:2"), "{}", e.message);
    let e = js(&dir, "const a = ;\n", "syn.png", &[], &io).err().unwrap();
    assert_eq!(e.code, "PX6001");
    assert_eq!(e.context["line"], 1, "{e:?}");
    let e =
        js(&dir, "px.output(px.canvas(1, 1));\npx.output(px.canvas(1, 1));\n", "twice.png", &[], &io).err().unwrap();
    assert_eq!(e.code, "PX6001");
    assert!(e.message.contains("already"));
}

const RANDOM_ART: &str = r#"
const c = px.canvas(64, 64);
for (let i = 0; i < 40; i++) {
  c.fillStyle = `hsl(${Math.floor(px.random() * 360)}, 70%, 50%)`;
  c.fillRect(Math.random() * 64, px.random() * 64, 8, 8);
}
px.log(px.params.title, px.random().toFixed(6));
px.output(c);
"#;

#[test]
fn same_inputs_and_seed_give_identical_output() {
    let dir = TempDir::new("render-det");
    let io = MockIo::default();
    let (a, _) = js(&dir, RANDOM_ART, "a.png", &[("param", "title=hi")], &io).unwrap();
    let ia = last_image(&io);
    let (b, _) = js(&dir, RANDOM_ART, "b.png", &[("param", "title=hi")], &io).unwrap();
    let ib = last_image(&io);
    assert_eq!(ia.data, ib.data);
    assert_eq!(a.value["logs"], b.value["logs"]);
    assert!(a.value["logs"][0].as_str().unwrap().starts_with("hi "));
    // another seed draws another picture
    let sp = dir.path("script.js");
    let out = dir.path("c.png");
    let g = Globals { seed: 7, ..globals(&out) };
    run_with("render js", vec![sp], &[("param", "title=hi")], g, &io).unwrap();
    assert_ne!(last_image(&io).data, ia.data);
}

#[test]
fn svg_results_rasterize_and_canvases_embed_in_svg() {
    let dir = TempDir::new("render-formats");
    let io = MockIo::default();
    let script = "const d = px.svg(40, 20);\nconst r = d.createElement('rect');\nr.setAttribute('width', 20); r.setAttribute('height', 20); r.setAttribute('fill', '#0000ff');\nd.documentElement.appendChild(r);\npx.output(d);\n";
    let (r, _) = js(&dir, script, "s.png", &[("size", "80x40")], &io).unwrap();
    assert_eq!(r.value["size"], json!([80, 40]));
    let img = last_image(&io);
    assert_eq!(img.get(10, 10), [0.0, 0.0, 1.0, 1.0]);
    assert_eq!(img.get(60, 10)[3], 0.0);
    let (r, out) =
        js(&dir, "const c = px.canvas(3, 3); c.fillText('hi', 0, 0); px.output(c);", "c.svg", &[], &io).unwrap();
    assert_eq!(r.value["kind"], "canvas");
    assert!(r.warnings.contains(&"PXW6002".to_string()));
    let svg = std::fs::read_to_string(out).unwrap();
    assert!(svg.contains("data:image/png;base64,iVBORw0KGgo"), "{svg}");
}

#[test]
fn allow_time_warns_and_data_kinds_parse() {
    let dir = TempDir::new("render-time");
    let io = MockIo::default();
    std::fs::write(dir.path("cfg.json"), r#"{"n": 3, "tags": ["a"]}"#).unwrap();
    std::fs::write(dir.path("note.txt"), "hello").unwrap();
    std::fs::write(dir.path("t.tsv"), "a\tb\n1\t2\n").unwrap();
    let d1 = format!("cfg={}", dir.path("cfg.json").display());
    let d2 = format!("note={}", dir.path("note.txt").display());
    let d3 = format!("t={}", dir.path("t.tsv").display());
    let script = "px.log(px.data.cfg.n, px.data.note, px.data.t[0].b, Date.now() > 0);\npx.output(px.canvas(1, 1));\n";
    let (r, _) =
        js(&dir, script, "t.png", &[("data", &d1), ("data", &d2), ("data", &d3), ("allow-time", "true")], &io).unwrap();
    assert_eq!(r.value["logs"][0], "3 hello 2 true");
    assert!(r.warnings.contains(&"PXW6001".to_string()));
    let e = js(&dir, "px.output(px.canvas(1,1));", "x.png", &[("data", "nofile")], &io).err().unwrap();
    assert_eq!(e.code, "PX1010");
}

#[test]
fn presets_need_a_local_bundle() {
    let dir = TempDir::new("render-preset");
    let io = MockIo::default();
    std::fs::write(dir.path("s.js"), "px.output(px.canvas(1, 1));").unwrap();
    for (cmd, lib) in [("render d3", "d3.v7.min.js"), ("render three", "three.module.js")] {
        let e = run_with(cmd, vec![dir.path("s.js")], &[], globals(&dir.path("o.svg")), &io).err().unwrap();
        assert_eq!(e.code, "PX1520");
        assert_eq!(e.exit_code, 12);
        assert!(e.hint.as_deref().unwrap().contains(&format!("--lib {lib}")), "{:?}", e.hint);
    }
    // a bundle that defines nothing is reported too
    std::fs::write(dir.path("empty.js"), "var nothing = 1;").unwrap();
    let lib = dir.path("empty.js").display().to_string();
    let e = run_with("render d3", vec![dir.path("s.js")], &[("lib", &lib)], globals(&dir.path("o.svg")), &io)
        .err()
        .unwrap();
    assert_eq!(e.code, "PX1520");
}

#[test]
fn three_preset_loads_commonjs_and_module_bundles() {
    let dir = TempDir::new("render-three");
    let io = MockIo::default();
    // CommonJS (three.cjs style) and ES module (three.module.js style) fakes
    std::fs::write(dir.path("three.cjs"), "'use strict';\nclass Vector3 { constructor(x, y, z) { this.x = x; this.y = y; this.z = z; } }\nexports.Vector3 = Vector3;\nexports.REVISION = '169';\n").unwrap();
    std::fs::write(dir.path("three.module.js"), "export class Vector3 { constructor(x, y, z) { this.x = x; this.y = y; this.z = z; } }\nexport const REVISION = '169m';\n").unwrap();
    let script = "const v = new THREE.Vector3(1, 2, 3);\nconst svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');\nsvg.setAttribute('width', 10); svg.setAttribute('height', 10);\nconst c = document.createElementNS('http://www.w3.org/2000/svg', 'circle');\nc.setAttribute('r', v.z);\nsvg.appendChild(c);\npx.log(THREE.REVISION, typeof module, typeof exports);\npx.output(svg);\n";
    std::fs::write(dir.path("scene.js"), script).unwrap();
    for (lib, rev) in [("three.cjs", "169"), ("three.module.js", "169m")] {
        let out = dir.path(&format!("{lib}.svg"));
        let l = dir.path(lib).display().to_string();
        let r = run_with("render three", vec![dir.path("scene.js")], &[("lib", &l)], globals(&out), &io).unwrap();
        assert_eq!(r.value["logs"][0], format!("{rev} undefined undefined"));
        let svg = std::fs::read_to_string(&out).unwrap();
        assert!(svg.contains("<circle r=\"3\"/>"), "{svg}");
    }
}

// ---------------------------------------------------------------------------------------
// render 3d

fn r3d(dir: &TempDir, pairs: &[(&str, &str)], out: &str, io: &MockIo) -> Result<(Run, PathBuf)> {
    let out = dir.path(out);
    let _ = std::fs::remove_file(&out);
    let r = run_with("render 3d", vec![], pairs, globals(&out), io)?;
    Ok((r, out))
}

#[test]
fn box_styles_render_and_hidden_line_has_fewer_segments() {
    let dir = TempDir::new("render-3d-box");
    let io = MockIo::default();
    let base = [("primitive", "box"), ("dims", "2,1,1"), ("camera", "iso")];
    let mut segs = BTreeMap::new();
    for style in ["wireframe", "hidden-line", "shaded"] {
        let mut p = base.to_vec();
        p.push(("style", style));
        if style == "shaded" {
            p.push(("outline", "true"));
        }
        let (r, out) = r3d(&dir, &p, &format!("{style}.svg"), &io).unwrap();
        let svg = std::fs::read_to_string(&out).unwrap();
        assert!(svg.contains("<path d=\"M"), "{style}: {svg}");
        rasterize_svg(&svg, None).unwrap();
        segs.insert(style, r.value["segments"].as_u64().unwrap());
        // PNG too
        let (_, png) = r3d(&dir, &p, &format!("{style}.png"), &io).unwrap();
        assert!(png.exists());
        let img = last_image(&io);
        assert_eq!((img.width, img.height), (1024, 768));
        assert!(testing::mean(&img, 3) > 0.005, "{style} drew nothing");
    }
    assert_eq!(segs["wireframe"], 12);
    assert!(segs["hidden-line"] < segs["wireframe"], "{segs:?}");
    assert_eq!(segs["hidden-line"], 9, "{segs:?}");
    // shaded raster: three visible faces with distinct shades
    let (_, _) = r3d(&dir, &[("primitive", "box"), ("style", "shaded"), ("size", "200x200")], "cube.png", &io).unwrap();
    let img = last_image(&io);
    let top = img.get(100, 60);
    let left = img.get(70, 120);
    let right = img.get(130, 120);
    assert_eq!(top[3], 1.0);
    assert!(top != left && left != right && top != right, "{top:?} {left:?} {right:?}");
}

#[test]
fn curved_primitives_cameras_and_perspective() {
    let dir = TempDir::new("render-3d-prims");
    let io = MockIo::default();
    for prim in PRIMS_FOR_TEST {
        for cam in ["front", "top", "dimetric", "persp"] {
            let (r, _) = r3d(
                &dir,
                &[("primitive", prim), ("camera", cam), ("size", "160x120"), ("segments", "12")],
                "p.svg",
                &io,
            )
            .unwrap();
            assert!(r.value["segments"].as_u64().unwrap() > 0, "{prim} {cam}");
        }
    }
    let (r, _) = r3d(
        &dir,
        &[("primitive", "sphere"), ("camera", "persp"), ("fov", "30"), ("look-at", "0,0,0"), ("style", "wireframe")],
        "s.svg",
        &io,
    )
    .unwrap();
    let wire = r.value["segments"].as_u64().unwrap();
    assert!(wire > 100);
    let (r, _) = r3d(&dir, &[("primitive", "sphere"), ("camera", "persp"), ("edges", "all")], "s.svg", &io).unwrap();
    let all = r.value["segments"].as_u64().unwrap();
    let (r, _) = r3d(&dir, &[("primitive", "sphere"), ("camera", "persp")], "s.svg", &io).unwrap();
    let feature = r.value["segments"].as_u64().unwrap();
    assert!(feature < all && all < wire, "{feature} {all} {wire}");
    let e = r3d(&dir, &[("primitive", "torus"), ("dims", "1,2,1")], "t.svg", &io).err().unwrap();
    assert_eq!(e.code, "PX1010");
    let e = r3d(&dir, &[("primitive", "box"), ("extrude", "x.svg")], "t.svg", &io).err().unwrap();
    assert_eq!(e.code, "PX1005");
    let e = r3d(&dir, &[("camera", "iso")], "t.svg", &io).err().unwrap();
    assert_eq!(e.code, "PX1003");
}

const PRIMS_FOR_TEST: [&str; 6] = ["box", "sphere", "cylinder", "cone", "torus", "plane"];

#[test]
fn extruded_logo_hidden_line_and_scene_with_obj() {
    let dir = TempDir::new("render-3d-logo");
    let io = MockIo::default();
    std::fs::write(
        dir.path("logo.svg"),
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="200" height="100"><g transform="translate(10 10)">
           <path d="M0 0 H80 V80 H0 Z M20 20 V60 H60 V20 Z" fill-rule="evenodd"/>
           <circle cx="140" cy="40" r="35"/></g></svg>"#,
    )
    .unwrap();
    let logo = dir.path("logo.svg").display().to_string();
    let (wire, _) = r3d(&dir, &[("extrude", &logo), ("depth", "40"), ("style", "wireframe")], "w.svg", &io).unwrap();
    let (hl, out) = r3d(
        &dir,
        &[("extrude", &logo), ("depth", "40"), ("style", "hidden-line"), ("line-width", "1.5")],
        "logo.svg.out.svg",
        &io,
    )
    .unwrap();
    assert!(hl.value["segments"].as_u64().unwrap() < wire.value["segments"].as_u64().unwrap());
    assert_eq!(hl.value["depth"], 40.0);
    assert!(std::fs::read_to_string(out).unwrap().contains("stroke-width=\"1.5\""));
    let (auto, _) = r3d(&dir, &[("extrude", &logo)], "auto.svg", &io).unwrap();
    assert!(auto.value["depth"].as_f64().unwrap() > 0.0);

    std::fs::write(dir.path("tri.obj"), "v 0 0 0\nv 1 0 0\nv 0 1 0\nv 0 0 1\nf 1 3 2\nf 1 2 4\nf 1 4 3\nf 2 3 4\n")
        .unwrap();
    std::fs::write(
        dir.path("scene.p3d.json"),
        r##"{"objects":[{"type":"box","dims":[2,1,1],"position":[0,0,0],"rotation":[0,30,0],"color":"#f97316"},
             {"type":"obj","path":"tri.obj","position":[3,0,0],"scale":1.5},
             {"type":"extrude","path":"logo.svg","depth":10,"scale":0.01,"position":[0,1.5,0]}],
            "camera":{"preset":"persp","position":[6,4,8],"look_at":[1,0,0],"fov":40},
            "light":{"direction":[0.3,1,0.6],"ambient":0.2},"background":"#ffffff"}"##,
    )
    .unwrap();
    let scene = dir.path("scene.p3d.json").display().to_string();
    let (r, _) = r3d(&dir, &[("scene", &scene), ("style", "shaded"), ("size", "320x240")], "scene.png", &io).unwrap();
    assert_eq!(r.value["objects"], 3);
    assert_eq!(r.value["camera"], "persp");
    let img = last_image(&io);
    assert_eq!(img.get(0, 0), [1.0, 1.0, 1.0, 1.0]); // scene background
    assert!(img.data.chunks(4).any(|p| p[0] > 0.6 && p[2] < 0.4), "the orange box is visible");
    let (r, out) = r3d(&dir, &[("scene", &scene), ("style", "shaded"), ("outline", "true")], "scene.svg", &io).unwrap();
    assert!(r.value["segments"].as_u64().unwrap() > 0);
    rasterize_svg(&std::fs::read_to_string(out).unwrap(), None).unwrap();
}

#[test]
fn as_layer_adds_a_raster_layer_in_place() {
    let dir = TempDir::new("render-3d-layer");
    let io = MockIo::default();
    let doc_path = dir.path("deck.pxo");
    std::fs::write(&doc_path, b"pxo").unwrap();
    io.docs.lock().unwrap().insert(doc_path.clone(), testing::doc_from(testing::solid(120, 80, Rgba::WHITE)));
    let g = Globals { in_place: true, ..Default::default() };
    let dp = doc_path.display().to_string();
    let r = run_with("render 3d", vec![], &[("primitive", "cylinder"), ("as-layer", &dp), ("style", "shaded")], g, &io)
        .unwrap();
    assert_eq!(r.value["size"], json!([120, 80]));
    assert_eq!(r.value["layer"], "3D render");
    let doc = io.docs.lock().unwrap().get(&doc_path).cloned().unwrap();
    assert_eq!(doc.layers.len(), 2);
    let img = testing::raster(&doc, "3D render");
    assert_eq!((img.width, img.height), (120, 80));
    assert!(testing::mean(img, 3) > 0.05);
    assert_eq!(doc.recipe.last().unwrap().op, "render.3d");
    // --in-place without --as-layer is refused
    let g = Globals { in_place: true, ..Default::default() };
    assert_eq!(run_with("render 3d", vec![], &[("primitive", "box")], g, &io).err().unwrap().code, "PX1402");
}

#[test]
fn op_specs_follow_the_authoring_contract() {
    let globals = [
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
    let specs = ops();
    let names: Vec<&str> = specs.iter().map(|s| s.name).collect();
    assert_eq!(names, vec!["render js", "render d3", "render three", "render 3d"]);
    for s in &specs {
        assert!(s.summary.len() <= 60 && !s.summary.ends_with('.'), "{}: summary", s.name);
        assert!(!s.about.is_empty(), "{}: about", s.name);
        assert!(s.examples.len() >= 3, "{}: examples", s.name);
        for p in &s.params {
            assert!(!p.help.is_empty(), "{}: --{} has no help", s.name, p.name);
            assert!(!globals.contains(&p.name), "{}: --{} shadows a global flag", s.name, p.name);
        }
        for ex in &s.examples {
            assert!(ex.cmd.starts_with(&format!("pixelox {} ", s.name)), "{}", ex.cmd);
            for tok in ex.cmd.split_whitespace().filter(|t| t.starts_with("--")) {
                let flag = &tok[2..];
                let global = ["seed", "in-place", "overwrite"].contains(&flag);
                assert!(global || s.find_param(flag).is_some(), "{}: unknown flag {tok}", ex.cmd);
            }
        }
    }
}

// ---------------------------------------------------------------------------------------
// Real-library smoke tests (skipped unless the bundles are supplied):
//   PIXELOX_D3_JS=/path/to/d3.min.js (7.9)       PIXELOX_THREE_JS=/path/to/three.core.js
//   PIXELOX_RENDER_DUMP=/dir writes the gallery below as PNG/SVG for a visual check.

const E9: &str = r#"
const rows = px.data.rows;
const doc  = px.svg(960, 540);
const sel  = d3.select(doc.documentElement);
const x = d3.scaleBand().domain(rows.map(r => r.month)).range([60, 920]).padding(0.2);
const y = d3.scaleLinear().domain([0, d3.max(rows, r => +r.sales)]).nice().range([500, 40]);
sel.selectAll('rect').data(rows).join('rect')
   .attr('x', r => x(r.month)).attr('y', r => y(+r.sales))
   .attr('width', x.bandwidth()).attr('height', r => 500 - y(+r.sales))
   .attr('fill', '#0ea5e9');
sel.append('g').attr('transform', 'translate(60,0)').call(d3.axisLeft(y));
sel.append('g').attr('transform', 'translate(0,500)').call(d3.axisBottom(x));
px.output(doc);
"#;

fn dump_dir() -> Option<PathBuf> {
    std::env::var_os("PIXELOX_RENDER_DUMP").map(PathBuf::from)
}

fn dump_png(name: &str, img: &Image) {
    if let Some(d) = dump_dir() {
        let png = js::image_to_pixmap(img).unwrap().encode_png().unwrap();
        std::fs::write(d.join(name), png).unwrap();
    }
}

#[test]
fn real_d3_bundle_runs_prd_example_e9() {
    let Ok(d3) = std::env::var("PIXELOX_D3_JS") else { return };
    let dir = TempDir::new("render-real-d3");
    let io = MockIo::default();
    std::fs::write(dir.path("sales.csv"), "month,sales\nJan,120\nFeb,95\nMar,140\nApr,180\nMay,160\nJun,210\n")
        .unwrap();
    std::fs::write(dir.path("chart.js"), E9).unwrap();
    let data = format!("rows={}", dir.path("sales.csv").display());
    let out = dir.path("chart.svg");
    let t = Instant::now();
    let r = run_with(
        "render d3",
        vec![dir.path("chart.js")],
        &[("lib", &d3), ("data", &data), ("size", "960x540")],
        globals(&out),
        &io,
    )
    .unwrap();
    eprintln!("E9 with real d3: {:?}", t.elapsed());
    let svg = std::fs::read_to_string(&out).unwrap();
    assert_eq!(svg.matches("<rect").count(), 6, "{svg}");
    assert!(svg.contains("class=\"tick\""), "axis ticks: {svg}");
    assert!(svg.contains(">Apr</text>"), "{svg}");
    assert_eq!(r.value["kind"], "svg");
    let pm = rasterize_svg(&svg, Some((1920, 1080))).unwrap();
    dump_png("e9-chart.png", &js::pixmap_to_image(&pm));
    if let Some(d) = dump_dir() {
        std::fs::write(d.join("e9-chart.svg"), &svg).unwrap();
    }
    // 10k-element chart stays inside the PRD budget class
    let many: String = (0..10_000).map(|i| format!("{},{}\n", i, (i * 7919) % 1000)).collect();
    std::fs::write(dir.path("big.csv"), format!("month,sales\n{many}")).unwrap();
    std::fs::write(
        dir.path("big.js"),
        "const rows = px.data.rows; const doc = px.svg(2000, 600); const x = d3.scaleBand().domain(rows.map(r => r.month)).range([0, 2000]);\nd3.select(doc.documentElement).selectAll('rect').data(rows).join('rect').attr('x', r => x(r.month)).attr('y', r => 600 - r.sales / 2).attr('width', x.bandwidth()).attr('height', r => r.sales / 2);\npx.output(doc);\n",
    )
    .unwrap();
    let data = format!("rows={}", dir.path("big.csv").display());
    let t = Instant::now();
    run_with(
        "render d3",
        vec![dir.path("big.js")],
        &[("lib", &d3), ("data", &data)],
        globals(&dir.path("big.svg")),
        &io,
    )
    .unwrap();
    eprintln!("10k-element d3 chart: {:?}", t.elapsed());
    assert_eq!(std::fs::read_to_string(dir.path("big.svg")).unwrap().matches("<rect").count(), 10_000);
}

#[test]
fn real_three_bundle_projects_geometry() {
    let Ok(three) = std::env::var("PIXELOX_THREE_JS") else { return };
    let dir = TempDir::new("render-real-three");
    let io = MockIo::default();
    let script = r#"
        const geo = new THREE.TorusKnotGeometry(1, 0.3, 64, 8);
        const cam = new THREE.PerspectiveCamera(40, 1, 0.1, 100);
        cam.position.set(3, 2, 4); cam.lookAt(0, 0, 0); cam.updateMatrixWorld();
        const pos = geo.attributes.position, idx = geo.index;
        const c = px.canvas(400, 400);
        c.fillStyle = '#ffffff'; c.fillRect(0, 0, 400, 400);
        c.strokeStyle = 'rgba(20, 30, 60, 0.6)'; c.lineWidth = 0.6;
        const v = new THREE.Vector3();
        const p = (i) => { v.fromBufferAttribute(pos, i).project(cam); return [(v.x + 1) * 200, (1 - v.y) * 200]; };
        c.beginPath();
        for (let i = 0; i < idx.count; i += 3) {
          const a = p(idx.getX(i)), b = p(idx.getX(i + 1)), d = p(idx.getX(i + 2));
          c.moveTo(a[0], a[1]); c.lineTo(b[0], b[1]); c.lineTo(d[0], d[1]); c.closePath();
        }
        c.stroke();
        px.log(THREE.REVISION, idx.count / 3);
        px.output(c);
    "#;
    std::fs::write(dir.path("knot.js"), script).unwrap();
    let t = Instant::now();
    let r =
        run_with("render three", vec![dir.path("knot.js")], &[("lib", &three)], globals(&dir.path("knot.png")), &io)
            .unwrap();
    eprintln!("three.js knot: {:?} {}", t.elapsed(), r.value["logs"]);
    let img = last_image(&io);
    dump_png("three-knot.png", &img);
    assert!(testing::mean(&img, 0) < 0.99, "lines were drawn");
}

#[test]
fn dump_gallery() {
    let Some(d) = dump_dir() else { return };
    let dir = TempDir::new("render-gallery");
    let io = MockIo::default();
    let art = r#"
        const c = px.canvas(480, 320);
        const g = c.createRadialGradient(240, 160, 10, 240, 160, 260);
        g.addColorStop(0, '#1e293b'); g.addColorStop(1, '#020617');
        c.fillStyle = g; c.fillRect(0, 0, 480, 320);
        c.globalCompositeOperation = 'lighter';
        for (let i = 0; i < 160; i++) {
          c.beginPath();
          c.arc(px.random() * 480, px.random() * 320, 4 + px.random() * 26, 0, Math.PI * 2);
          c.fillStyle = `hsla(${190 + px.random() * 80}, 80%, 60%, 0.25)`;
          c.fill();
        }
        c.globalCompositeOperation = 'source-over';
        c.setLineDash([8, 6]); c.lineWidth = 3; c.strokeStyle = '#f8fafc';
        c.beginPath(); c.moveTo(30, 290); c.bezierCurveTo(160, 40, 320, 360, 450, 60); c.stroke();
        c.setLineDash([]); c.lineJoin = 'round'; c.strokeStyle = '#fb923c'; c.lineWidth = 6;
        c.strokeRect(20, 20, 440, 280);
        c.save(); c.translate(400, 250); c.rotate(Math.PI / 6); c.fillStyle = '#fde047';
        c.beginPath(); c.roundRect(-30, -15, 60, 30, 8); c.fill(); c.restore();
        px.output(c);
    "#;
    js(&dir, art, "art.png", &[], &io).unwrap();
    dump_png("canvas-art.png", &last_image(&io));
    std::fs::write(
        dir.path("logo.svg"),
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="300" height="120">
           <path d="M10 10 H90 V110 H10 Z M30 30 V90 H70 V30 Z" fill-rule="evenodd"/>
           <circle cx="160" cy="60" r="50"/><path d="M220 110 L260 10 L300 110 Z"/></svg>"#,
    )
    .unwrap();
    let logo = dir.path("logo.svg").display().to_string();
    for (style, cam) in [("hidden-line", "iso"), ("wireframe", "iso"), ("shaded", "persp")] {
        let mut p = vec![
            ("extrude", logo.as_str()),
            ("depth", "40"),
            ("style", style),
            ("camera", cam),
            ("bg-color", "#ffffff"),
        ];
        if style == "shaded" {
            p.push(("outline", "true"));
        }
        r3d(&dir, &p, "logo.png", &io).unwrap();
        dump_png(&format!("logo-{style}.png"), &last_image(&io));
        let (_, svg) = r3d(&dir, &p, "logo.svg.svg", &io).unwrap();
        std::fs::copy(svg, d.join(format!("logo-{style}.svg"))).unwrap();
    }
    for (prim, style, edges) in [
        ("sphere", "hidden-line", "feature"),
        ("sphere", "hidden-line", "all"),
        ("torus", "shaded", "feature"),
        ("cylinder", "hidden-line", "feature"),
        ("cone", "shaded", "feature"),
    ] {
        let p = [
            ("primitive", prim),
            ("style", style),
            ("edges", edges),
            ("camera", "dimetric"),
            ("size", "400x300"),
            ("bg-color", "#ffffff"),
            ("outline", "true"),
        ];
        r3d(&dir, &p, "p.png", &io).unwrap();
        dump_png(&format!("prim-{prim}-{style}-{edges}.png"), &last_image(&io));
    }
}

#[test]
fn global_object_is_an_allowlist() {
    let dir = TempDir::new("render-globals");
    let io = MockIo::default();
    let (r, _) = js(
        &dir,
        "px.log(Object.getOwnPropertyNames(globalThis).sort().join(','));\npx.output(px.canvas(1, 1));\n",
        "g.png",
        &[],
        &io,
    )
    .unwrap();
    let names: Vec<String> = r.value["logs"][0].as_str().unwrap().split(',').map(String::from).collect();
    eprintln!("{names:?}");
    // standard ECMAScript built-ins, plus `px` and the `console` alias of px.log
    let standard = [
        "AggregateError",
        "Array",
        "ArrayBuffer",
        "AsyncDisposableStack",
        "BigInt",
        "BigInt64Array",
        "BigUint64Array",
        "Boolean",
        "DataView",
        "Date",
        "DisposableStack",
        "Error",
        "EvalError",
        "Float16Array",
        "Float32Array",
        "Float64Array",
        "Function",
        "Infinity",
        "Int16Array",
        "Int32Array",
        "Int8Array",
        "InternalError",
        "Iterator",
        "JSON",
        "Map",
        "Math",
        "NaN",
        "Number",
        "Object",
        "Promise",
        "Proxy",
        "RangeError",
        "ReferenceError",
        "Reflect",
        "RegExp",
        "Set",
        "String",
        "SuppressedError",
        "Symbol",
        "SyntaxError",
        "TypeError",
        "URIError",
        "Uint16Array",
        "Uint32Array",
        "Uint8Array",
        "Uint8ClampedArray",
        "WeakMap",
        "WeakSet",
        "console",
        "decodeURI",
        "decodeURIComponent",
        "encodeURI",
        "encodeURIComponent",
        "escape",
        "globalThis",
        "isFinite",
        "isNaN",
        "parseFloat",
        "parseInt",
        "px",
        "queueMicrotask",
        "undefined",
        "unescape",
    ];
    let unexpected: Vec<&String> = names.iter().filter(|n| !standard.contains(&n.as_str())).collect();
    assert!(unexpected.is_empty(), "unexpected globals: {unexpected:?}");
    assert!(names.contains(&"px".to_string()));
    assert!(!names.contains(&"Atomics".to_string()) && !names.contains(&"SharedArrayBuffer".to_string()));
}

#[test]
fn modules_cannot_be_imported() {
    let dir = TempDir::new("render-import");
    let io = MockIo::default();
    std::fs::write(dir.path("secret.js"), "export const s = 1;").unwrap();
    let script =
        "import('./secret.js').then(() => px.log('loaded'), (e) => px.log('blocked'));\npx.output(px.canvas(1, 1));\n";
    let (r, _) = js(&dir, script, "i.png", &[], &io).unwrap();
    assert_eq!(r.value["logs"], json!(["blocked"]));
    let e =
        js(&dir, "import { s } from './secret.js';\npx.output(px.canvas(1, 1));\n", "j.png", &[], &io).err().unwrap();
    assert_eq!(e.code, "PX6001");
}
