// pixelox JS sandbox prelude. Evaluated once per run as a function expression and called
// with the native bindings (`N`) and the run configuration (`cfg`). It defines the frozen
// global `px` (Canvas2D contexts implemented in Rust, a DOM-lite SVG document implemented
// here), then locks the realm down: no eval, no Function constructor, a fixed clock, UTC
// dates, and a seeded Math.random. Nothing from `N` is reachable from user scripts.
(function (N, cfg) {
  'use strict';
  const G = globalThis;
  const SVG_NS = 'http://www.w3.org/2000/svg';
  const XLINK_NS = 'http://www.w3.org/1999/xlink';
  const XML_NS = 'http://www.w3.org/XML/1998/namespace';
  const XMLNS_NS = 'http://www.w3.org/2000/xmlns/';
  const freeze = Object.freeze;
  const defineProperty = Object.defineProperty;

  function deepFreeze(o) {
    if (o && (typeof o === 'object' || typeof o === 'function') && !Object.isFrozen(o)) {
      freeze(o);
      for (const k of Object.getOwnPropertyNames(o)) {
        const d = Object.getOwnPropertyDescriptor(o, k);
        if (d && 'value' in d) deepFreeze(d.value);
      }
    }
    return o;
  }

  // ---------------------------------------------------------------------------------
  // DOM-lite: enough of the DOM for d3-selection (select/selectAll/data/join/enter/
  // append/insert/attr/style/classed/text/call/order/raise/lower/remove) on SVG.

  const ELEMENT = 1, TEXT = 3, COMMENT = 8, DOCUMENT = 9, FRAGMENT = 11, RAW = 99;

  function precedes(x, y) {
    // true when sibling x comes before sibling y (walks both ways from x)
    let f = x._next, b = x._prev;
    while (f || b) {
      if (f === y) return true;
      if (b === y) return false;
      if (f) f = f._next;
      if (b) b = b._prev;
    }
    return false;
  }

  class Node {
    constructor(doc, type, name) {
      this.ownerDocument = doc;
      this.nodeType = type;
      this.nodeName = name;
      this.parentNode = null;
      this._first = null;
      this._last = null;
      this._prev = null;
      this._next = null;
    }
    get parentElement() {
      const p = this.parentNode;
      return p && p.nodeType === ELEMENT ? p : null;
    }
    get firstChild() { return this._first; }
    get lastChild() { return this._last; }
    get nextSibling() { return this._next; }
    get previousSibling() { return this._prev; }
    get childNodes() {
      const out = [];
      for (let c = this._first; c; c = c._next) out.push(c);
      return out;
    }
    hasChildNodes() { return this._first !== null; }
    get isConnected() {
      let n = this;
      while (n.parentNode) n = n.parentNode;
      return n.nodeType === DOCUMENT;
    }
    appendChild(child) { return this.insertBefore(child, null); }
    insertBefore(child, ref) {
      if (!(child instanceof Node)) throw new TypeError('insertBefore: argument is not a node');
      if (ref === undefined) ref = null;
      if (ref !== null && (!(ref instanceof Node) || ref.parentNode !== this)) {
        throw new Error('NotFoundError: the reference node is not a child of this node');
      }
      if (child.nodeType === FRAGMENT) {
        for (const k of child.childNodes) this.insertBefore(k, ref);
        return child;
      }
      if (child.nodeType === DOCUMENT) throw new Error('HierarchyRequestError: cannot insert a document');
      for (let p = this; p; p = p.parentNode) {
        if (p === child) throw new Error('HierarchyRequestError: a node cannot contain itself');
      }
      if (child === ref) return child;
      if (child.parentNode) child.parentNode._unlink(child);
      child.parentNode = this;
      if (ref === null) {
        child._prev = this._last;
        child._next = null;
        if (this._last) this._last._next = child; else this._first = child;
        this._last = child;
      } else {
        child._next = ref;
        child._prev = ref._prev;
        if (ref._prev) ref._prev._next = child; else this._first = child;
        ref._prev = child;
      }
      return child;
    }
    _unlink(child) {
      if (child._prev) child._prev._next = child._next; else this._first = child._next;
      if (child._next) child._next._prev = child._prev; else this._last = child._prev;
      child._prev = child._next = null;
      child.parentNode = null;
    }
    removeChild(child) {
      if (!(child instanceof Node) || child.parentNode !== this) {
        throw new Error('NotFoundError: the node to remove is not a child of this node');
      }
      this._unlink(child);
      return child;
    }
    replaceChild(node, old) {
      this.insertBefore(node, old);
      return this.removeChild(old);
    }
    remove() {
      if (this.parentNode) this.parentNode._unlink(this);
    }
    contains(n) {
      for (; n; n = n.parentNode) if (n === this) return true;
      return false;
    }
    compareDocumentPosition(other) {
      if (other === this) return 0;
      const a = [], b = [];
      for (let n = this; n; n = n.parentNode) a.push(n);
      for (let n = other; n; n = n.parentNode) b.push(n);
      if (a[a.length - 1] !== b[b.length - 1]) return 1 | 32 | 2;
      if (a.indexOf(other) >= 0) return 2 | 8;
      if (b.indexOf(this) >= 0) return 4 | 16;
      let i = a.length - 1, j = b.length - 1;
      while (a[i] === b[j]) { i--; j--; }
      return precedes(a[i], b[j]) ? 4 : 2;
    }
    get textContent() {
      let s = '';
      const walk = (n) => {
        for (let c = n._first; c; c = c._next) {
          if (c.nodeType === TEXT) s += c.data;
          else if (c.nodeType === ELEMENT || c.nodeType === FRAGMENT) walk(c);
        }
      };
      walk(this);
      return s;
    }
    set textContent(v) {
      while (this._first) this._unlink(this._first);
      const s = v == null ? '' : String(v);
      if (s !== '') this.insertBefore(new Text(this.ownerDocument || this, s), null);
    }
    get nodeValue() { return null; }
    set nodeValue(v) {}
    addEventListener() {}
    removeEventListener() {}
    dispatchEvent() { return true; }
    append(...nodes) {
      for (const n of nodes) this.insertBefore(this._toNode(n), null);
    }
    prepend(...nodes) {
      const first = this._first;
      for (const n of nodes) this.insertBefore(this._toNode(n), first);
    }
    _toNode(n) {
      return n instanceof Node ? n : new Text(this.ownerDocument || this, String(n));
    }
    getRootNode() {
      let n = this;
      while (n.parentNode) n = n.parentNode;
      return n;
    }
  }

  class Text extends Node {
    constructor(doc, data) {
      super(doc, TEXT, '#text');
      this.data = String(data);
    }
    get nodeValue() { return this.data; }
    set nodeValue(v) { this.data = String(v); }
    get textContent() { return this.data; }
    set textContent(v) { this.data = v == null ? '' : String(v); }
    get length() { return this.data.length; }
    get wholeText() { return this.data; }
    cloneNode() { return new Text(this.ownerDocument, this.data); }
  }

  class Comment extends Node {
    constructor(doc, data) {
      super(doc, COMMENT, '#comment');
      this.data = String(data);
    }
    get nodeValue() { return this.data; }
    set nodeValue(v) { this.data = String(v); }
    get textContent() { return this.data; }
    set textContent(v) { this.data = v == null ? '' : String(v); }
    cloneNode() { return new Comment(this.ownerDocument, this.data); }
  }

  // Raw markup assigned through innerHTML: serialized verbatim, never parsed.
  class Raw extends Node {
    constructor(doc, markup) {
      super(doc, RAW, '#raw');
      this.markup = String(markup);
    }
    get textContent() { return ''; }
    set textContent(v) {}
    cloneNode() { return new Raw(this.ownerDocument, this.markup); }
  }

  class Fragment extends Node {
    constructor(doc) { super(doc, FRAGMENT, '#document-fragment'); }
    get children() { return elementChildren(this); }
    querySelector(sel) { return querySelector(this, sel); }
    querySelectorAll(sel) { return querySelectorAll(this, sel); }
    cloneNode(deep) {
      const f = new Fragment(this.ownerDocument);
      if (deep) for (let c = this._first; c; c = c._next) f.insertBefore(c.cloneNode(true), null);
      return f;
    }
  }

  function elementChildren(n) {
    const out = [];
    for (let c = n._first; c; c = c._next) if (c.nodeType === ELEMENT) out.push(c);
    return out;
  }

  function kebab(prop) {
    if (prop.startsWith('--')) return prop;
    return prop.replace(/[A-Z]/g, (m) => '-' + m.toLowerCase());
  }

  function parseStyle(text) {
    const out = [];
    if (!text) return out;
    for (const part of String(text).split(';')) {
      const i = part.indexOf(':');
      if (i < 0) continue;
      const name = part.slice(0, i).trim();
      let value = part.slice(i + 1).trim();
      if (!name) continue;
      let priority = '';
      const m = /\s*!\s*important\s*$/i.exec(value);
      if (m) { value = value.slice(0, m.index); priority = 'important'; }
      out.push([name, value, priority]);
    }
    return out;
  }

  class StyleDecl {
    constructor(el) { defineProperty(this, '_el', { value: el }); }
    _list() { return parseStyle(this._el.getAttribute('style')); }
    _write(list) {
      if (list.length === 0) this._el.removeAttribute('style');
      else this._el.setAttribute('style', list.map(([n, v, p]) => n + ': ' + v + (p ? ' !important' : '')).join('; ') + ';');
    }
    setProperty(name, value, priority) {
      name = String(name);
      if (value === null || value === undefined || value === '') return this.removeProperty(name);
      const list = this._list();
      const item = [name, String(value), priority ? 'important' : ''];
      const i = list.findIndex((e) => e[0] === name);
      if (i >= 0) list[i] = item; else list.push(item);
      this._write(list);
    }
    getPropertyValue(name) {
      const e = this._list().find((x) => x[0] === String(name));
      return e ? e[1] : '';
    }
    getPropertyPriority(name) {
      const e = this._list().find((x) => x[0] === String(name));
      return e ? e[2] : '';
    }
    removeProperty(name) {
      const list = this._list();
      const i = list.findIndex((e) => e[0] === String(name));
      if (i < 0) return '';
      const old = list[i][1];
      list.splice(i, 1);
      this._write(list);
      return old;
    }
    get cssText() { return this._el.getAttribute('style') || ''; }
    set cssText(v) { this._write(parseStyle(v)); }
    get length() { return this._list().length; }
    item(i) { const e = this._list()[i]; return e ? e[0] : ''; }
  }

  function styleProxy(el) {
    const decl = new StyleDecl(el);
    return new Proxy(decl, {
      get(t, prop) {
        if (typeof prop !== 'string' || prop in t) return Reflect.get(t, prop, t);
        return t.getPropertyValue(kebab(prop));
      },
      set(t, prop, value) {
        if (typeof prop !== 'string' || prop === 'cssText') return Reflect.set(t, prop, value, t);
        t.setProperty(kebab(prop), value);
        return true;
      },
    });
  }

  class ClassList {
    constructor(el) { defineProperty(this, '_el', { value: el }); }
    _get() { return (this._el.getAttribute('class') || '').split(/\s+/).filter(Boolean); }
    _set(list) { this._el.setAttribute('class', list.join(' ')); }
    add(...names) {
      const l = this._get();
      for (const n of names) if (l.indexOf(String(n)) < 0) l.push(String(n));
      this._set(l);
    }
    remove(...names) {
      const drop = names.map(String);
      this._set(this._get().filter((c) => drop.indexOf(c) < 0));
    }
    contains(name) { return this._get().indexOf(String(name)) >= 0; }
    toggle(name, force) {
      const has = this.contains(name);
      const want = force === undefined ? !has : !!force;
      if (want && !has) this.add(name);
      if (!want && has) this.remove(name);
      return want;
    }
    replace(a, b) {
      const l = this._get();
      const i = l.indexOf(String(a));
      if (i < 0) return false;
      l[i] = String(b);
      this._set(l);
      return true;
    }
    item(i) { return this._get()[i] || null; }
    get length() { return this._get().length; }
    get value() { return this._el.getAttribute('class') || ''; }
    toString() { return this.value; }
    [Symbol.iterator]() { return this._get()[Symbol.iterator](); }
  }

  class Element extends Node {
    constructor(doc, ns, qname) {
      qname = String(qname);
      super(doc, ELEMENT, qname);
      const i = qname.indexOf(':');
      this.namespaceURI = ns == null || ns === '' ? null : String(ns);
      this.prefix = i >= 0 ? qname.slice(0, i) : null;
      this.localName = i >= 0 ? qname.slice(i + 1) : qname;
      this.tagName = qname;
      this._attrs = [];
    }
    _find(name) {
      name = String(name);
      for (const a of this._attrs) if (a.name === name) return a;
      return null;
    }
    _findNS(ns, local) {
      ns = ns == null || ns === '' ? null : String(ns);
      local = String(local);
      for (const a of this._attrs) if (a.ns === ns && a.local === local) return a;
      return null;
    }
    get attributes() {
      return this._attrs.map((a) => ({ name: a.name, localName: a.local, namespaceURI: a.ns, value: a.value }));
    }
    hasAttributes() { return this._attrs.length > 0; }
    getAttributeNames() { return this._attrs.map((a) => a.name); }
    getAttribute(name) { const a = this._find(name); return a ? a.value : null; }
    hasAttribute(name) { return this._find(name) !== null; }
    setAttribute(name, value) {
      name = String(name);
      const a = this._find(name);
      if (a) a.value = String(value);
      else {
        const i = name.indexOf(':');
        this._attrs.push({ name, local: i >= 0 ? name.slice(i + 1) : name, ns: null, value: String(value) });
      }
    }
    removeAttribute(name) {
      const a = this._find(name);
      if (a) this._attrs.splice(this._attrs.indexOf(a), 1);
    }
    toggleAttribute(name, force) {
      const has = this.hasAttribute(name);
      const want = force === undefined ? !has : !!force;
      if (want && !has) this.setAttribute(name, '');
      if (!want && has) this.removeAttribute(name);
      return want;
    }
    getAttributeNS(ns, local) { const a = this._findNS(ns, local); return a ? a.value : null; }
    hasAttributeNS(ns, local) { return this._findNS(ns, local) !== null; }
    setAttributeNS(ns, qname, value) {
      qname = String(qname);
      const i = qname.indexOf(':');
      const local = i >= 0 ? qname.slice(i + 1) : qname;
      const a = this._findNS(ns, local);
      if (a) a.value = String(value);
      else this._attrs.push({ name: qname, local, ns: ns == null || ns === '' ? null : String(ns), value: String(value) });
    }
    removeAttributeNS(ns, local) {
      const a = this._findNS(ns, local);
      if (a) this._attrs.splice(this._attrs.indexOf(a), 1);
    }
    get id() { return this.getAttribute('id') || ''; }
    set id(v) { this.setAttribute('id', v); }
    get className() { return this.getAttribute('class') || ''; }
    set className(v) { this.setAttribute('class', v); }
    get classList() { return new ClassList(this); }
    get style() { return styleProxy(this); }
    set style(v) { this.setAttribute('style', String(v)); }
    get children() { return elementChildren(this); }
    get childElementCount() { return elementChildren(this).length; }
    get firstElementChild() {
      for (let c = this._first; c; c = c._next) if (c.nodeType === ELEMENT) return c;
      return null;
    }
    get lastElementChild() {
      for (let c = this._last; c; c = c._prev) if (c.nodeType === ELEMENT) return c;
      return null;
    }
    get nextElementSibling() {
      for (let c = this._next; c; c = c._next) if (c.nodeType === ELEMENT) return c;
      return null;
    }
    get previousElementSibling() {
      for (let c = this._prev; c; c = c._prev) if (c.nodeType === ELEMENT) return c;
      return null;
    }
    get ownerSVGElement() {
      for (let p = this.parentNode; p && p.nodeType === ELEMENT; p = p.parentNode) {
        if (p.localName === 'svg') return p;
      }
      return null;
    }
    querySelector(sel) { return querySelector(this, sel); }
    querySelectorAll(sel) { return querySelectorAll(this, sel); }
    getElementsByTagName(name) {
      return querySelectorAll(this, '*').filter((e) => name === '*' || e.localName === name || e.tagName === name);
    }
    getElementsByClassName(names) {
      const want = String(names).split(/\s+/).filter(Boolean);
      return querySelectorAll(this, '*').filter((e) => {
        const has = (e.getAttribute('class') || '').split(/\s+/);
        return want.every((w) => has.indexOf(w) >= 0);
      });
    }
    matches(sel) { return matchesSelector(this, compile(sel)); }
    closest(sel) {
      const c = compile(sel);
      for (let e = this; e && e.nodeType === ELEMENT; e = e.parentNode) if (matchesSelector(e, c)) return e;
      return null;
    }
    get innerHTML() {
      let s = '';
      for (let c = this._first; c; c = c._next) s += serialize(c, this.namespaceURI, {});
      return s;
    }
    set innerHTML(v) {
      while (this._first) this._unlink(this._first);
      const s = v == null ? '' : String(v);
      if (s !== '') this.insertBefore(new Raw(this.ownerDocument, s), null);
    }
    get outerHTML() { return serializeRoot(this); }
    cloneNode(deep) {
      const e = new Element(this.ownerDocument, this.namespaceURI, this.tagName);
      e._attrs = this._attrs.map((a) => Object.assign({}, a));
      if (deep) for (let c = this._first; c; c = c._next) e.insertBefore(c.cloneNode(true), null);
      return e;
    }
    toString() { return serializeRoot(this); }
  }

  const defaultView = freeze({
    getComputedStyle(node) {
      return { getPropertyValue: (name) => (node && node.style ? node.style.getPropertyValue(name) : '') };
    },
  });

  class Document extends Node {
    constructor(w, h) {
      super(null, DOCUMENT, '#document');
      this.defaultView = defaultView;
      this.contentType = 'image/svg+xml';
      const root = this.createElementNS(SVG_NS, 'svg');
      if (w !== undefined) root.setAttribute('width', String(w));
      if (h !== undefined) root.setAttribute('height', String(h));
      if (w !== undefined && h !== undefined) root.setAttribute('viewBox', '0 0 ' + w + ' ' + h);
      this.insertBefore(root, null);
    }
    get documentElement() {
      for (let c = this._first; c; c = c._next) if (c.nodeType === ELEMENT) return c;
      return null;
    }
    get children() { return elementChildren(this); }
    createElementNS(ns, qname) { return new Element(this, ns, qname); }
    createElement(name) { return new Element(this, SVG_NS, name); }
    createTextNode(data) { return new Text(this, data); }
    createComment(data) { return new Comment(this, data); }
    createDocumentFragment() { return new Fragment(this); }
    getElementById(id) {
      id = String(id);
      return querySelectorAll(this, '*').find((e) => e.getAttribute('id') === id) || null;
    }
    getElementsByTagName(name) {
      return querySelectorAll(this, '*').filter((e) => name === '*' || e.localName === name || e.tagName === name);
    }
    querySelector(sel) { return querySelector(this, sel); }
    querySelectorAll(sel) { return querySelectorAll(this, sel); }
    cloneNode(deep) {
      const d = new Document();
      d._unlink(d._first);
      if (deep) for (let c = this._first; c; c = c._next) d.insertBefore(adopt(c.cloneNode(true), d), null);
      return d;
    }
    toString() { const r = this.documentElement; return r ? serializeRoot(r) : ''; }
  }

  function adopt(n, doc) {
    n.ownerDocument = doc;
    for (let c = n._first; c; c = c._next) adopt(c, doc);
    return n;
  }

  // ---- selectors: type, *, #id, .class, [attr], [attr op value], :first-child,
  // :last-child, :nth-child(n), :not(simple); combinators ' ', '>', '+', '~'; ',' groups.

  const selectorCache = new Map();

  function compile(sel) {
    sel = String(sel);
    let c = selectorCache.get(sel);
    if (!c) {
      c = parseSelectorList(sel);
      if (selectorCache.size > 500) selectorCache.clear();
      selectorCache.set(sel, c);
    }
    return c;
  }

  function parseSelectorList(src) {
    let i = 0;
    const s = src;
    const err = () => new SyntaxError("'" + src + "' is not a supported selector");
    const ident = () => {
      const m = /^(?:\\.|[\w -￿-])+/.exec(s.slice(i));
      if (!m) throw err();
      i += m[0].length;
      return m[0].replace(/\\(.)/g, '$1');
    };
    const ws = () => { while (i < s.length && /\s/.test(s[i])) i++; };
    const compound = () => {
      const c = { tag: null, id: null, classes: [], attrs: [], pseudos: [] };
      let any = false;
      if (s[i] === '*') { i++; any = true; }
      else if (i < s.length && /[\w -￿-]/.test(s[i])) { c.tag = ident(); any = true; }
      if (i < s.length && s[i] === '|') { i++; c.tag = s[i] === '*' ? (i++, null) : ident(); }
      for (;;) {
        const ch = s[i];
        if (ch === '#') { i++; c.id = ident(); any = true; }
        else if (ch === '.') { i++; c.classes.push(ident()); any = true; }
        else if (ch === '[') {
          i++; ws();
          const name = ident(); ws();
          let op = null, value = null;
          const m = /^([~|^$*]?=)/.exec(s.slice(i));
          if (m) {
            op = m[1]; i += op.length; ws();
            if (s[i] === '"' || s[i] === "'") {
              const q = s[i++];
              const j = s.indexOf(q, i);
              if (j < 0) throw err();
              value = s.slice(i, j); i = j + 1;
            } else value = ident();
            ws();
          }
          if (s[i] !== ']') throw err();
          i++;
          c.attrs.push({ name, op, value });
          any = true;
        } else if (ch === ':') {
          i++;
          const name = ident().toLowerCase();
          let arg = null;
          if (s[i] === '(') {
            const j = s.indexOf(')', i);
            if (j < 0) throw err();
            arg = s.slice(i + 1, j).trim(); i = j + 1;
          }
          if (name === 'not') c.pseudos.push({ name, arg: parseSelectorList(arg) });
          else if (['first-child', 'last-child', 'only-child', 'nth-child', 'root', 'empty', 'scope'].indexOf(name) >= 0) c.pseudos.push({ name, arg });
          else throw err();
          any = true;
        } else break;
      }
      if (!any) throw err();
      return c;
    };
    const groups = [];
    ws();
    for (;;) {
      const parts = [];
      let comb = null;
      for (;;) {
        const c = compound();
        c.comb = comb;
        parts.push(c);
        const before = i;
        ws();
        if (i >= s.length || s[i] === ',') break;
        if (s[i] === '>' || s[i] === '+' || s[i] === '~') { comb = s[i]; i++; ws(); }
        else if (i > before) comb = ' ';
        else throw err();
      }
      groups.push(parts);
      if (i >= s.length) break;
      i++; ws();
    }
    return groups;
  }

  function nthIndex(e) {
    let n = 1;
    for (let p = e.previousElementSibling; p; p = p.previousElementSibling) n++;
    return n;
  }

  function matchNth(arg, n) {
    const t = arg.replace(/\s+/g, '').toLowerCase();
    if (t === 'odd') return n % 2 === 1;
    if (t === 'even') return n % 2 === 0;
    const m = /^([+-]?\d*)n([+-]\d+)?$/.exec(t);
    if (m) {
      const a = m[1] === '' || m[1] === '+' ? 1 : m[1] === '-' ? -1 : parseInt(m[1], 10);
      const b = m[2] ? parseInt(m[2], 10) : 0;
      if (a === 0) return n === b;
      const k = (n - b) / a;
      return k >= 0 && Number.isInteger(k);
    }
    return n === parseInt(t, 10);
  }

  function matchCompound(e, c) {
    if (c.tag !== null && e.localName !== c.tag && e.tagName !== c.tag) return false;
    if (c.id !== null && e.getAttribute('id') !== c.id) return false;
    if (c.classes.length) {
      const cls = (e.getAttribute('class') || '').split(/\s+/);
      for (const k of c.classes) if (cls.indexOf(k) < 0) return false;
    }
    for (const a of c.attrs) {
      const v = e.getAttribute(a.name);
      if (v === null) return false;
      switch (a.op) {
        case null: break;
        case '=': if (v !== a.value) return false; break;
        case '~=': if (v.split(/\s+/).indexOf(a.value) < 0) return false; break;
        case '|=': if (v !== a.value && !v.startsWith(a.value + '-')) return false; break;
        case '^=': if (!a.value || !v.startsWith(a.value)) return false; break;
        case '$=': if (!a.value || !v.endsWith(a.value)) return false; break;
        case '*=': if (!a.value || v.indexOf(a.value) < 0) return false; break;
        default: return false;
      }
    }
    for (const p of c.pseudos) {
      switch (p.name) {
        case 'first-child': if (e.previousElementSibling) return false; break;
        case 'last-child': if (e.nextElementSibling) return false; break;
        case 'only-child': if (e.previousElementSibling || e.nextElementSibling) return false; break;
        case 'nth-child': if (!matchNth(p.arg || '', nthIndex(e))) return false; break;
        case 'root': if (!e.parentNode || e.parentNode.nodeType !== DOCUMENT) return false; break;
        case 'empty': if (e._first) return false; break;
        case 'scope': break;
        case 'not': if (matchesSelector(e, p.arg)) return false; break;
        default: return false;
      }
    }
    return true;
  }

  function matchParts(e, parts, k) {
    if (!matchCompound(e, parts[k])) return false;
    if (k === 0) return true;
    switch (parts[k].comb) {
      case '>': {
        const p = e.parentNode;
        return !!p && p.nodeType === ELEMENT && matchParts(p, parts, k - 1);
      }
      case ' ':
        for (let p = e.parentNode; p && p.nodeType === ELEMENT; p = p.parentNode) if (matchParts(p, parts, k - 1)) return true;
        return false;
      case '+': {
        const s = e.previousElementSibling;
        return !!s && matchParts(s, parts, k - 1);
      }
      case '~':
        for (let s = e.previousElementSibling; s; s = s.previousElementSibling) if (matchParts(s, parts, k - 1)) return true;
        return false;
      default:
        return false;
    }
  }

  function matchesSelector(e, groups) {
    for (const parts of groups) if (matchParts(e, parts, parts.length - 1)) return true;
    return false;
  }

  function querySelectorAll(root, sel) {
    const groups = compile(sel);
    const out = [];
    const walk = (n) => {
      for (let c = n._first; c; c = c._next) {
        if (c.nodeType === ELEMENT) {
          if (matchesSelector(c, groups)) out.push(c);
          walk(c);
        } else if (c.nodeType === FRAGMENT) walk(c);
      }
    };
    walk(root);
    return out;
  }

  function querySelector(root, sel) {
    const groups = compile(sel);
    let found = null;
    const walk = (n) => {
      for (let c = n._first; c && !found; c = c._next) {
        if (c.nodeType === ELEMENT) {
          if (matchesSelector(c, groups)) { found = c; return; }
          walk(c);
        }
      }
    };
    walk(root);
    return found;
  }

  // ---- serialization

  function escText(s) {
    return s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
  }
  function escAttr(s) {
    return s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/"/g, '&quot;').replace(/\n/g, '&#10;');
  }

  function attrName(a, flags) {
    if (a.ns === XLINK_NS) { flags.xlink = true; return 'xlink:' + a.local; }
    if (a.ns === XML_NS) return 'xml:' + a.local;
    if (a.ns === XMLNS_NS) return a.local === 'xmlns' ? 'xmlns' : 'xmlns:' + a.local;
    return a.name;
  }

  function serialize(n, parentNs, flags) {
    switch (n.nodeType) {
      case TEXT: return escText(n.data);
      case COMMENT: return '<!--' + n.data.replace(/--/g, '- -') + '-->';
      case RAW: return n.markup;
      case FRAGMENT: case DOCUMENT: {
        let s = '';
        for (let c = n._first; c; c = c._next) s += serialize(c, parentNs, flags);
        return s;
      }
      case ELEMENT: {
        let s = '<' + n.tagName;
        const ns = n.namespaceURI;
        if (ns !== parentNs && !n.prefix && !n.hasAttribute('xmlns')) s += ' xmlns="' + escAttr(ns || '') + '"';
        for (const a of n._attrs) s += ' ' + attrName(a, flags) + '="' + escAttr(a.value) + '"';
        if (!n._first) return s + '/>';
        s += '>';
        for (let c = n._first; c; c = c._next) s += serialize(c, ns, flags);
        return s + '</' + n.tagName + '>';
      }
      default: return '';
    }
  }

  function serializeRoot(el) {
    const flags = { xlink: false };
    let s = serialize(el, null, flags);
    if (flags.xlink && !el.hasAttribute('xmlns:xlink')) {
      const open = s.indexOf(' ');
      const at = open > 0 && open < s.indexOf('>') ? open : s.search(/\/?>/);
      s = s.slice(0, at) + ' xmlns:xlink="' + XLINK_NS + '"' + s.slice(at);
    }
    return s;
  }

  // ---------------------------------------------------------------------------------
  // Canvas2D: a thin wrapper over the Rust implementation (tiny-skia).

  const OP = {
    fillRect: 1, strokeRect: 2, clearRect: 3, beginPath: 4, closePath: 5, moveTo: 6, lineTo: 7,
    bezierCurveTo: 8, quadraticCurveTo: 9, arc: 10, arcTo: 11, ellipse: 12, rect: 13, fill: 14,
    stroke: 15, clip: 16, save: 17, restore: 18, translate: 19, rotate: 20, scale: 21, transform: 22,
    setTransform: 23, resetTransform: 24, lineWidth: 25, globalAlpha: 26, miterLimit: 27,
    lineDashOffset: 28, roundRect: 29,
  };
  const STR = { lineCap: 1, lineJoin: 2, globalCompositeOperation: 3 };
  const canvasIds = new WeakMap();

  class CanvasGradient {
    constructor(id) { defineProperty(this, '_id', { value: id }); }
    addColorStop(offset, color) {
      offset = +offset;
      if (!(offset >= 0 && offset <= 1)) throw new RangeError('IndexSizeError: color stop offset must be in 0..1');
      if (!N.gradStop(this._id, offset, String(color))) throw new SyntaxError("SyntaxError: '" + color + "' is not a valid color");
    }
  }

  function textWidth(font, text) {
    const m = /(\d+(?:\.\d+)?)px/.exec(font);
    const size = m ? +m[1] : 10;
    return String(text).length * size * 0.55;
  }

  class CanvasRenderingContext2D {
    constructor(w, h) {
      const id = N.cvNew(w, h);
      defineProperty(this, '_id', { value: id });
      defineProperty(this, '_s', { value: [] , writable: true });
      this._st = {
        fillStyle: '#000000', strokeStyle: '#000000', lineWidth: 1, lineCap: 'butt', lineJoin: 'miter',
        miterLimit: 10, globalAlpha: 1, globalCompositeOperation: 'source-over', lineDashOffset: 0,
        dash: [], font: '10px sans-serif', textAlign: 'start', textBaseline: 'alphabetic',
        shadowBlur: 0, shadowColor: 'rgba(0, 0, 0, 0)', shadowOffsetX: 0, shadowOffsetY: 0,
        imageSmoothingEnabled: true, direction: 'inherit', filter: 'none',
      };
      const self = this;
      this.canvas = freeze({ width: w, height: h, getContext() { return self; } });
      canvasIds.set(this, id);
    }
    getContext() { return this; }
    get fillStyle() { return this._st.fillStyle; }
    set fillStyle(v) { this._paint(0, 'fillStyle', v); }
    get strokeStyle() { return this._st.strokeStyle; }
    set strokeStyle(v) { this._paint(1, 'strokeStyle', v); }
    _paint(which, key, v) {
      if (v instanceof CanvasGradient) { N.cvGrad(this._id, which, v._id); this._st[key] = v; }
      else if (v !== null && typeof v === 'object') throw new TypeError('patterns are not supported in the pixelox canvas');
      else if (N.cvColor(this._id, which, String(v))) this._st[key] = String(v);
    }
    get lineWidth() { return this._st.lineWidth; }
    set lineWidth(v) { v = +v; if (v > 0 && isFinite(v)) { this._st.lineWidth = v; N.cvOp(this._id, OP.lineWidth, v); } }
    get miterLimit() { return this._st.miterLimit; }
    set miterLimit(v) { v = +v; if (v > 0 && isFinite(v)) { this._st.miterLimit = v; N.cvOp(this._id, OP.miterLimit, v); } }
    get globalAlpha() { return this._st.globalAlpha; }
    set globalAlpha(v) { v = +v; if (v >= 0 && v <= 1) { this._st.globalAlpha = v; N.cvOp(this._id, OP.globalAlpha, v); } }
    get lineDashOffset() { return this._st.lineDashOffset; }
    set lineDashOffset(v) { v = +v; if (isFinite(v)) { this._st.lineDashOffset = v; N.cvOp(this._id, OP.lineDashOffset, v); } }
    get lineCap() { return this._st.lineCap; }
    set lineCap(v) { this._str('lineCap', v); }
    get lineJoin() { return this._st.lineJoin; }
    set lineJoin(v) { this._str('lineJoin', v); }
    get globalCompositeOperation() { return this._st.globalCompositeOperation; }
    set globalCompositeOperation(v) { this._str('globalCompositeOperation', v); }
    _str(key, v) { v = String(v); if (N.cvStr(this._id, STR[key], v)) this._st[key] = v; }
    get font() { return this._st.font; }
    set font(v) { this._st.font = String(v); }
    get textAlign() { return this._st.textAlign; }
    set textAlign(v) { this._st.textAlign = String(v); }
    get textBaseline() { return this._st.textBaseline; }
    set textBaseline(v) { this._st.textBaseline = String(v); }
    get direction() { return this._st.direction; }
    set direction(v) { this._st.direction = String(v); }
    get filter() { return this._st.filter; }
    set filter(v) { this._st.filter = String(v); }
    get shadowBlur() { return this._st.shadowBlur; }
    set shadowBlur(v) { this._st.shadowBlur = +v; }
    get shadowColor() { return this._st.shadowColor; }
    set shadowColor(v) { this._st.shadowColor = String(v); }
    get shadowOffsetX() { return this._st.shadowOffsetX; }
    set shadowOffsetX(v) { this._st.shadowOffsetX = +v; }
    get shadowOffsetY() { return this._st.shadowOffsetY; }
    set shadowOffsetY(v) { this._st.shadowOffsetY = +v; }
    get imageSmoothingEnabled() { return this._st.imageSmoothingEnabled; }
    set imageSmoothingEnabled(v) { this._st.imageSmoothingEnabled = !!v; }

    save() {
      N.cvOp(this._id, OP.save);
      const st = Object.assign({}, this._st);
      st.dash = this._st.dash.slice();
      this._s.push(st);
    }
    restore() {
      N.cvOp(this._id, OP.restore);
      if (this._s.length) this._st = this._s.pop();
    }
    reset() {
      while (this._s.length) this.restore();
      this.resetTransform();
      this.clearRect(0, 0, this.canvas.width, this.canvas.height);
      this.beginPath();
    }

    fillRect(x, y, w, h) { N.cvOp(this._id, OP.fillRect, +x, +y, +w, +h); }
    strokeRect(x, y, w, h) { N.cvOp(this._id, OP.strokeRect, +x, +y, +w, +h); }
    clearRect(x, y, w, h) { N.cvOp(this._id, OP.clearRect, +x, +y, +w, +h); }
    beginPath() { N.cvOp(this._id, OP.beginPath); }
    closePath() { N.cvOp(this._id, OP.closePath); }
    moveTo(x, y) { N.cvOp(this._id, OP.moveTo, +x, +y); }
    lineTo(x, y) { N.cvOp(this._id, OP.lineTo, +x, +y); }
    bezierCurveTo(a, b, c, d, e, f) { N.cvOp(this._id, OP.bezierCurveTo, +a, +b, +c, +d, +e, +f); }
    quadraticCurveTo(a, b, c, d) { N.cvOp(this._id, OP.quadraticCurveTo, +a, +b, +c, +d); }
    arc(x, y, r, a0, a1, ccw) {
      if (+r < 0) throw new RangeError('IndexSizeError: negative radius');
      N.cvOp(this._id, OP.arc, +x, +y, +r, +a0, +a1, ccw ? 1 : 0);
    }
    arcTo(x1, y1, x2, y2, r) {
      if (+r < 0) throw new RangeError('IndexSizeError: negative radius');
      N.cvOp(this._id, OP.arcTo, +x1, +y1, +x2, +y2, +r);
    }
    ellipse(x, y, rx, ry, rot, a0, a1, ccw) {
      if (+rx < 0 || +ry < 0) throw new RangeError('IndexSizeError: negative radius');
      N.cvOp(this._id, OP.ellipse, +x, +y, +rx, +ry, +rot, +a0, +a1, ccw ? 1 : 0);
    }
    rect(x, y, w, h) { N.cvOp(this._id, OP.rect, +x, +y, +w, +h); }
    roundRect(x, y, w, h, radii) {
      let r = radii === undefined ? [0] : Array.isArray(radii) ? radii : [radii];
      r = r.map((v) => (typeof v === 'object' && v !== null ? +v.x : +v));
      if (r.length < 1 || r.length > 4) throw new RangeError('roundRect: 1 to 4 radii');
      const [a, b, c, d] = r.length === 1 ? [r[0], r[0], r[0], r[0]] : r.length === 2 ? [r[0], r[1], r[0], r[1]]
        : r.length === 3 ? [r[0], r[1], r[2], r[1]] : r;
      N.cvOp(this._id, OP.roundRect, +x, +y, +w, +h, a, b, c, d);
    }
    fill(a, b) {
      if (a !== undefined && typeof a === 'object') throw new TypeError('Path2D is not supported in the pixelox canvas');
      N.cvOp(this._id, OP.fill, (a === undefined ? b : a) === 'evenodd' ? 1 : 0);
    }
    stroke(p) {
      if (p !== undefined) throw new TypeError('Path2D is not supported in the pixelox canvas');
      N.cvOp(this._id, OP.stroke);
    }
    clip(a, b) {
      if (a !== undefined && typeof a === 'object') throw new TypeError('Path2D is not supported in the pixelox canvas');
      N.cvOp(this._id, OP.clip, (a === undefined ? b : a) === 'evenodd' ? 1 : 0);
    }
    translate(x, y) { N.cvOp(this._id, OP.translate, +x, +y); }
    rotate(a) { N.cvOp(this._id, OP.rotate, +a); }
    scale(x, y) { N.cvOp(this._id, OP.scale, +x, +y); }
    transform(a, b, c, d, e, f) { N.cvOp(this._id, OP.transform, +a, +b, +c, +d, +e, +f); }
    setTransform(a, b, c, d, e, f) {
      if (a === undefined) return this.resetTransform();
      if (typeof a === 'object' && a !== null) {
        const m = a;
        return N.cvOp(this._id, OP.setTransform, +(m.a ?? m.m11 ?? 1), +(m.b ?? m.m12 ?? 0), +(m.c ?? m.m21 ?? 0),
          +(m.d ?? m.m22 ?? 1), +(m.e ?? m.m41 ?? 0), +(m.f ?? m.m42 ?? 0));
      }
      N.cvOp(this._id, OP.setTransform, +a, +b, +c, +d, +e, +f);
    }
    resetTransform() { N.cvOp(this._id, OP.resetTransform); }
    getTransform() {
      const [a, b, c, d, e, f] = N.cvGetTransform(this._id);
      return { a, b, c, d, e, f, m11: a, m12: b, m21: c, m22: d, m41: e, m42: f, is2D: true };
    }
    setLineDash(seg) {
      if (!seg || typeof seg.length !== 'number') throw new TypeError('setLineDash expects an array');
      const v = Array.from(seg, (x) => +x);
      if (v.some((x) => !isFinite(x) || x < 0)) return;
      const dash = v.length % 2 ? v.concat(v) : v;
      this._st.dash = dash;
      N.cvDash(this._id, ...dash);
    }
    getLineDash() { return this._st.dash.slice(); }
    createLinearGradient(x0, y0, x1, y1) { return new CanvasGradient(N.gradNew(0, +x0, +y0, 0, +x1, +y1, 0)); }
    createRadialGradient(x0, y0, r0, x1, y1, r1) {
      if (+r0 < 0 || +r1 < 0) throw new RangeError('IndexSizeError: negative radius');
      return new CanvasGradient(N.gradNew(1, +x0, +y0, +r0, +x1, +y1, +r1));
    }
    createConicGradient() { throw new TypeError('conic gradients are not supported in the pixelox canvas'); }
    createPattern() { throw new TypeError('patterns are not supported in the pixelox canvas'); }
    drawImage(src, ...a) {
      const id = canvasIds.get(src) ?? (src && src.getContext ? canvasIds.get(src.getContext('2d')) : undefined);
      if (id === undefined) throw new TypeError('drawImage: the source must be a px.canvas() context');
      if (a.length !== 2 && a.length !== 4 && a.length !== 8) throw new TypeError('drawImage takes 3, 5, or 9 arguments');
      N.cvDraw(this._id, id, ...a.map((v) => +v));
    }
    createImageData(w, h) {
      if (typeof w === 'object' && w) { h = w.height; w = w.width; }
      w = Math.abs(w | 0); h = Math.abs(h | 0);
      return { width: w, height: h, data: new Uint8ClampedArray(w * h * 4), colorSpace: 'srgb' };
    }
    getImageData(x, y, w, h) {
      x |= 0; y |= 0; w |= 0; h |= 0;
      if (w <= 0 || h <= 0) throw new RangeError('IndexSizeError: getImageData needs a positive size');
      const buf = N.cvGetImage(this._id, x, y, w, h);
      return { width: w, height: h, data: new Uint8ClampedArray(buf), colorSpace: 'srgb' };
    }
    putImageData(img, dx, dy) {
      if (!img || !img.data) throw new TypeError('putImageData expects an ImageData');
      const u8 = img.data instanceof Uint8ClampedArray || img.data instanceof Uint8Array
        ? new Uint8Array(img.data) : Uint8Array.from(img.data);
      N.cvPutImage(this._id, u8.buffer, img.width | 0, img.height | 0, dx | 0, dy | 0);
    }
    fillText(text, x, y) { N.cvText(this._id); }
    strokeText(text, x, y) { N.cvText(this._id); }
    measureText(text) {
      const w = textWidth(this._st.font, text);
      return { width: w, actualBoundingBoxLeft: 0, actualBoundingBoxRight: w, actualBoundingBoxAscent: 0, actualBoundingBoxDescent: 0 };
    }
    isPointInPath() { return false; }
    isPointInStroke() { return false; }
  }

  // ---------------------------------------------------------------------------------
  // px

  let outputs = 0;
  const px = {
    version: cfg.version,
    width: cfg.width,
    height: cfg.height,
    params: cfg.params,
    data: {},
    canvas(w, h) {
      w = w === undefined ? cfg.width : Math.floor(+w);
      h = h === undefined ? cfg.height : Math.floor(+h);
      if (!(w >= 1 && h >= 1)) throw new RangeError('px.canvas(w, h) needs a positive width and height');
      return new CanvasRenderingContext2D(w, h);
    },
    svg(w, h) {
      w = w === undefined ? cfg.width : +w;
      h = h === undefined ? cfg.height : +h;
      if (!(w > 0 && h > 0)) throw new RangeError('px.svg(w, h) needs a positive width and height');
      return new Document(w, h);
    },
    random() { return N.random(); },
    log(...args) {
      N.log(args.map((a) => {
        if (typeof a === 'string') return a;
        try { const j = JSON.stringify(a); return j === undefined ? String(a) : j; } catch (e) { return String(a); }
      }).join(' '));
    },
    output(target) {
      if (outputs > 0) throw new Error('px.output() was already called; call it exactly once');
      if (target instanceof CanvasRenderingContext2D || (target && canvasIds.has(target))) {
        outputs++;
        return N.outCanvas(target._id);
      }
      if (target && target.getContext && canvasIds.has(target.getContext('2d'))) {
        outputs++;
        return N.outCanvas(target.getContext('2d')._id);
      }
      let el = null;
      if (target instanceof Document) el = target.documentElement;
      else if (target instanceof Element) el = target;
      if (!el) throw new TypeError('px.output() takes a px.canvas() context, a px.svg() document, or an SVG element');
      let root = el;
      if (el.localName !== 'svg' || el.namespaceURI !== SVG_NS) {
        root = new Element(el.ownerDocument, SVG_NS, 'svg');
        root.setAttribute('width', String(cfg.width));
        root.setAttribute('height', String(cfg.height));
        root.setAttribute('viewBox', '0 0 ' + cfg.width + ' ' + cfg.height);
        root._first = root._last = null;
        const clone = el.cloneNode(true);
        root.insertBefore(clone, null);
      }
      outputs++;
      return N.outSvg(serializeRoot(root));
    },
  };
  for (const [name, entry] of Object.entries(cfg.data)) {
    let v = entry.text;
    if (entry.kind === 'json' || entry.kind === 'table') {
      v = JSON.parse(entry.text);
      if (entry.kind === 'table') {
        const cols = entry.columns;
        defineProperty(v, 'columns', { value: cols, enumerable: false });
      }
    }
    px.data[name] = v;
  }
  defineProperty(px, 'Document', { value: Document });
  defineProperty(px, 'Element', { value: Element });
  deepFreeze(px.params);
  deepFreeze(px.data);
  freeze(px);
  defineProperty(G, 'px', { value: px, writable: false, enumerable: false, configurable: false });

  // console is a convenience alias for px.log (libraries such as three.js warn through it)
  const logFn = (...a) => px.log(...a);
  defineProperty(G, 'console', {
    value: freeze({ log: logFn, info: logFn, warn: logFn, error: logFn, debug: logFn, trace: logFn }),
    writable: false, enumerable: false, configurable: false,
  });

  // ---------------------------------------------------------------------------------
  // lockdown

  const forbid = (what) => function () {
    throw new EvalError('PX6010: ' + what + ' is not available in the pixelox sandbox');
  };
  delete G.eval;
  const FnProto = Function.prototype;
  const NoFunction = forbid('the Function constructor');
  NoFunction.prototype = FnProto;
  defineProperty(FnProto, 'constructor', { value: NoFunction, writable: false, configurable: false });
  defineProperty(G, 'Function', { value: NoFunction, writable: false, enumerable: false, configurable: false });
  const kinds = [
    ['AsyncFunction', async function () {}],
    ['GeneratorFunction', function* () {}],
    ['AsyncGeneratorFunction', async function* () {}],
  ];
  for (const [name, f] of kinds) {
    const proto = Object.getPrototypeOf(f);
    const stub = forbid('the ' + name + ' constructor');
    stub.prototype = proto;
    defineProperty(proto, 'constructor', { value: stub, writable: false, configurable: false });
  }
  // Atomics.wait could block the thread where the wall-clock interrupt cannot fire
  for (const name of ['WebAssembly', 'fetch', 'XMLHttpRequest', 'require', 'process', 'setTimeout', 'setInterval',
    'clearTimeout', 'clearInterval', 'setImmediate', 'Worker', 'performance', 'std', 'os', 'Atomics',
    'SharedArrayBuffer']) {
    if (name in G) delete G[name];
  }

  // seeded Math.random (same stream as px.random)
  defineProperty(Math, 'random', { value: function random() { return N.random(); }, writable: false, configurable: false });

  // dates: fixed clock (unless --allow-time) and UTC everywhere
  const RealDate = Date;
  const isoLocal = /^\d{4}-\d{2}-\d{2}(T\d{2}:\d{2}(:\d{2}(\.\d+)?)?)?$/;
  const fixIso = (s) => (typeof s === 'string' && isoLocal.test(s.trim()) && s.indexOf('T') > 0 ? s.trim() + 'Z' : s);
  const now = cfg.allowTime ? () => { N.timeRead(); return RealDate.now(); } : () => 0;
  function PxDate(...args) {
    if (!new.target) return new RealDate(now()).toUTCString();
    let a;
    if (args.length === 0) a = [now()];
    else if (args.length === 1) a = [fixIso(args[0])];
    else a = [RealDate.UTC(...args)];
    return Reflect.construct(RealDate, a, new.target === PxDate ? RealDate : new.target);
  }
  PxDate.prototype = RealDate.prototype;
  PxDate.now = function now_() { return now(); };
  PxDate.UTC = RealDate.UTC;
  PxDate.parse = function parse(s) { return RealDate.parse(fixIso(s)); };
  defineProperty(RealDate.prototype, 'constructor', { value: PxDate, writable: true, configurable: true });
  const DP = RealDate.prototype;
  for (const m of ['FullYear', 'Month', 'Date', 'Day', 'Hours', 'Minutes', 'Seconds', 'Milliseconds']) {
    defineProperty(DP, 'get' + m, { value: DP['getUTC' + m], writable: true, configurable: true });
    if (m !== 'Day') defineProperty(DP, 'set' + m, { value: DP['setUTC' + m], writable: true, configurable: true });
  }
  defineProperty(DP, 'getTimezoneOffset', { value: function getTimezoneOffset() { return 0; }, writable: true, configurable: true });
  const utcString = DP.toUTCString;
  const isoString = DP.toISOString;
  for (const m of ['toString', 'toLocaleString', 'toTimeString']) {
    defineProperty(DP, m, { value: function () { return utcString.call(this); }, writable: true, configurable: true });
  }
  for (const m of ['toDateString', 'toLocaleDateString']) {
    defineProperty(DP, m, { value: function () { return isoString.call(this).slice(0, 10); }, writable: true, configurable: true });
  }
  defineProperty(DP, 'toLocaleTimeString', { value: function () { return isoString.call(this).slice(11, 19); }, writable: true, configurable: true });
  defineProperty(G, 'Date', { value: PxDate, writable: false, enumerable: false, configurable: false });

  // ---------------------------------------------------------------------------------
  // hooks for the host

  return {
    // presets: a DOM-lite `document` for libraries that create elements globally
    defineDocument() {
      const doc = new Document(cfg.width, cfg.height);
      defineProperty(G, 'document', { value: doc, writable: true, enumerable: false, configurable: true });
    },
    // CommonJS shim while loading a preset library bundle
    beginLib() {
      const module = { exports: {} };
      defineProperty(G, 'module', { value: module, writable: true, configurable: true });
      defineProperty(G, 'exports', { value: module.exports, writable: true, configurable: true });
    },
    endLib(name) {
      const m = G.module;
      delete G.module;
      delete G.exports;
      if (name && G[name] === undefined && m && m.exports && Object.keys(m.exports).length) {
        defineProperty(G, name, { value: m.exports, writable: true, enumerable: false, configurable: true });
      }
      return name ? G[name] !== undefined : true;
    },
    exposeModule(name, ns) {
      if (name) {
        if (G[name] === undefined) defineProperty(G, name, { value: ns, writable: true, enumerable: false, configurable: true });
      } else {
        for (const k of Object.keys(ns)) if (k !== 'default' && !(k in G)) G[k] = ns[k];
      }
    },
    outputs() { return outputs; },
  };
});
