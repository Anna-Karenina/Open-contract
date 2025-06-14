(function (e, t) {
  if (typeof define === "function" && define.amd) {
    define([], t);
  } else if (typeof module === "object" && module.exports) {
    module.exports = t();
  } else {
    e.htmx = e.htmx || t();
  }
})(typeof self !== "undefined" ? self : this, function () {
  return (function () {
    "use strict";
    var Y = {
      onLoad: t,
      process: Pt,
      on: Z,
      off: K,
      trigger: fe,
      ajax: wr,
      find: E,
      findAll: f,
      closest: v,
      values: function (e, t) {
        var r = nr(e, t || "post");
        return r.values;
      },
      remove: U,
      addClass: B,
      removeClass: n,
      toggleClass: V,
      takeClass: j,
      defineExtension: qr,
      removeExtension: Hr,
      logAll: X,
      logNone: F,
      logger: null,
      config: {
        historyEnabled: true,
        historyCacheSize: 10,
        refreshOnHistoryMiss: false,
        defaultSwapStyle: "innerHTML",
        defaultSwapDelay: 0,
        defaultSettleDelay: 20,
        includeIndicatorStyles: true,
        indicatorClass: "htmx-indicator",
        requestClass: "htmx-request",
        addedClass: "htmx-added",
        settlingClass: "htmx-settling",
        swappingClass: "htmx-swapping",
        allowEval: true,
        allowScriptTags: true,
        inlineScriptNonce: "",
        attributesToSettle: ["class", "style", "width", "height"],
        withCredentials: false,
        timeout: 0,
        wsReconnectDelay: "full-jitter",
        wsBinaryType: "blob",
        disableSelector: "[hx-disable], [data-hx-disable]",
        useTemplateFragments: false,
        scrollBehavior: "smooth",
        defaultFocusScroll: false,
        getCacheBusterParam: false,
        globalViewTransitions: false,
        methodsThatUseUrlParams: ["get"],
        selfRequestsOnly: false,
      },
      parseInterval: d,
      _: e,
      createEventSource: function (e) {
        return new EventSource(e, {
          withCredentials: true,
        });
      },
      createWebSocket: function (e) {
        var t = new WebSocket(e, []);
        t.binaryType = Y.config.wsBinaryType;
        return t;
      },
      version: "1.9.6",
    };
    var r = {
      addTriggerHandler: St,
      bodyContains: oe,
      canAccessLocalStorage: M,
      findThisElement: de,
      filterValues: lr,
      hasAttribute: o,
      getAttributeValue: ee,
      getClosestAttributeValue: re,
      getClosestMatch: c,
      getExpressionVars: xr,
      getHeaders: sr,
      getInputValues: nr,
      getInternalData: ie,
      getSwapSpecification: fr,
      getTriggerSpecs: Ze,
      getTarget: ge,
      makeFragment: l,
      mergeObjects: se,
      makeSettleInfo: T,
      oobSwap: ye,
      querySelectorExt: le,
      selectAndSwap: Fe,
      settleImmediately: Wt,
      shouldCancel: tt,
      triggerEvent: fe,
      triggerErrorEvent: ue,
      withExtensions: C,
    };
    var b = ["get", "post", "put", "delete", "patch"];
    var w = b
      .map(function (e) {
        return "[hx-" + e + "], [data-hx-" + e + "]";
      })
      .join(", ");
    function d(e) {
      if (e == undefined) {
        return undefined;
      }
      if (e.slice(-2) == "ms") {
        return parseFloat(e.slice(0, -2)) || undefined;
      }
      if (e.slice(-1) == "s") {
        return parseFloat(e.slice(0, -1)) * 1e3 || undefined;
      }
      if (e.slice(-1) == "m") {
        return parseFloat(e.slice(0, -1)) * 1e3 * 60 || undefined;
      }
      return parseFloat(e) || undefined;
    }
    function Q(e, t) {
      return e.getAttribute && e.getAttribute(t);
    }
    function o(e, t) {
      return (
        e.hasAttribute && (e.hasAttribute(t) || e.hasAttribute("data-" + t))
      );
    }
    function ee(e, t) {
      return Q(e, t) || Q(e, "data-" + t);
    }
    function u(e) {
      return e.parentElement;
    }
    function te() {
      return document;
    }
    function c(e, t) {
      while (e && !t(e)) {
        e = u(e);
      }
      return e ? e : null;
    }
    function O(e, t, r) {
      var n = ee(t, r);
      var i = ee(t, "hx-disinherit");
      if (e !== t && i && (i === "*" || i.split(" ").indexOf(r) >= 0)) {
        return "unset";
      } else {
        return n;
      }
    }
    function re(t, r) {
      var n = null;
      c(t, function (e) {
        return (n = O(t, e, r));
      });
      if (n !== "unset") {
        return n;
      }
    }
    function h(e, t) {
      var r =
        e.matches ||
        e.matchesSelector ||
        e.msMatchesSelector ||
        e.mozMatchesSelector ||
        e.webkitMatchesSelector ||
        e.oMatchesSelector;
      return r && r.call(e, t);
    }
    function q(e) {
      var t = /<([a-z][^\/\0>\x20\t\r\n\f]*)/i;
      var r = t.exec(e);
      if (r) {
        return r[1].toLowerCase();
      } else {
        return "";
      }
    }
    function i(e, t) {
      var r = new DOMParser();
      var n = r.parseFromString(e, "text/html");
      var i = n.body;
      while (t > 0) {
        t--;
        i = i.firstChild;
      }
      if (i == null) {
        i = te().createDocumentFragment();
      }
      return i;
    }
    function H(e) {
      return e.match(/<body/);
    }
    function l(e) {
      var t = !H(e);
      if (Y.config.useTemplateFragments && t) {
        var r = i("<body><template>" + e + "</template></body>", 0);
        return r.querySelector("template").content;
      } else {
        var n = q(e);
        switch (n) {
          case "thead":
          case "tbody":
          case "tfoot":
          case "colgroup":
          case "caption":
            return i("<table>" + e + "</table>", 1);
          case "col":
            return i("<table><colgroup>" + e + "</colgroup></table>", 2);
          case "tr":
            return i("<table><tbody>" + e + "</tbody></table>", 2);
          case "td":
          case "th":
            return i("<table><tbody><tr>" + e + "</tr></tbody></table>", 3);
          case "script":
          case "style":
            return i("<div>" + e + "</div>", 1);
          default:
            return i(e, 0);
        }
      }
    }
    function ne(e) {
      if (e) {
        e();
      }
    }
    function L(e, t) {
      return Object.prototype.toString.call(e) === "[object " + t + "]";
    }
    function A(e) {
      return L(e, "Function");
    }
    function N(e) {
      return L(e, "Object");
    }
    function ie(e) {
      var t = "htmx-internal-data";
      var r = e[t];
      if (!r) {
        r = e[t] = {};
      }
      return r;
    }
    function I(e) {
      var t = [];
      if (e) {
        for (var r = 0; r < e.length; r++) {
          t.push(e[r]);
        }
      }
      return t;
    }
    function ae(e, t) {
      if (e) {
        for (var r = 0; r < e.length; r++) {
          t(e[r]);
        }
      }
    }
    function P(e) {
      var t = e.getBoundingClientRect();
      var r = t.top;
      var n = t.bottom;
      return r < window.innerHeight && n >= 0;
    }
    function oe(e) {
      if (e.getRootNode && e.getRootNode() instanceof window.ShadowRoot) {
        return te().body.contains(e.getRootNode().host);
      } else {
        return te().body.contains(e);
      }
    }
    function k(e) {
      return e.trim().split(/\s+/);
    }
    function se(e, t) {
      for (var r in t) {
        if (t.hasOwnProperty(r)) {
          e[r] = t[r];
        }
      }
      return e;
    }
    function S(e) {
      try {
        return JSON.parse(e);
      } catch (e) {
        y(e);
        return null;
      }
    }
    function M() {
      var e = "htmx:localStorageTest";
      try {
        localStorage.setItem(e, e);
        localStorage.removeItem(e);
        return true;
      } catch (e) {
        return false;
      }
    }
    function D(t) {
      try {
        var e = new URL(t);
        if (e) {
          t = e.pathname + e.search;
        }
        if (!t.match("^/$")) {
          t = t.replace(/\/+$/, "");
        }
        return t;
      } catch (e) {
        return t;
      }
    }
    function e(e) {
      return gr(te().body, function () {
        return eval(e);
      });
    }
    function t(t) {
      var e = Y.on("htmx:load", function (e) {
        t(e.detail.elt);
      });
      return e;
    }
    function X() {
      Y.logger = function (e, t, r) {
        if (console) {
          console.log(t, e, r);
        }
      };
    }
    function F() {
      Y.logger = null;
    }
    function E(e, t) {
      if (t) {
        return e.querySelector(t);
      } else {
        return E(te(), e);
      }
    }
    function f(e, t) {
      if (t) {
        return e.querySelectorAll(t);
      } else {
        return f(te(), e);
      }
    }
    function U(e, t) {
      e = s(e);
      if (t) {
        setTimeout(function () {
          U(e);
          e = null;
        }, t);
      } else {
        e.parentElement.removeChild(e);
      }
    }
    function B(e, t, r) {
      e = s(e);
      if (r) {
        setTimeout(function () {
          B(e, t);
          e = null;
        }, r);
      } else {
        e.classList && e.classList.add(t);
      }
    }
    function n(e, t, r) {
      e = s(e);
      if (r) {
        setTimeout(function () {
          n(e, t);
          e = null;
        }, r);
      } else {
        if (e.classList) {
          e.classList.remove(t);
          if (e.classList.length === 0) {
            e.removeAttribute("class");
          }
        }
      }
    }
    function V(e, t) {
      e = s(e);
      e.classList.toggle(t);
    }
    function j(e, t) {
      e = s(e);
      ae(e.parentElement.children, function (e) {
        n(e, t);
      });
      B(e, t);
    }
    function v(e, t) {
      e = s(e);
      if (e.closest) {
        return e.closest(t);
      } else {
        do {
          if (e == null || h(e, t)) {
            return e;
          }
        } while ((e = e && u(e)));
        return null;
      }
    }
    function g(e, t) {
      return e.substring(0, t.length) === t;
    }
    function _(e, t) {
      return e.substring(e.length - t.length) === t;
    }
    function z(e) {
      var t = e.trim();
      if (g(t, "<") && _(t, "/>")) {
        return t.substring(1, t.length - 2);
      } else {
        return t;
      }
    }
    function W(e, t) {
      if (t.indexOf("closest ") === 0) {
        return [v(e, z(t.substr(8)))];
      } else if (t.indexOf("find ") === 0) {
        return [E(e, z(t.substr(5)))];
      } else if (t.indexOf("next ") === 0) {
        return [$(e, z(t.substr(5)))];
      } else if (t.indexOf("previous ") === 0) {
        return [G(e, z(t.substr(9)))];
      } else if (t === "document") {
        return [document];
      } else if (t === "window") {
        return [window];
      } else if (t === "body") {
        return [document.body];
      } else {
        return te().querySelectorAll(z(t));
      }
    }
    var $ = function (e, t) {
      var r = te().querySelectorAll(t);
      for (var n = 0; n < r.length; n++) {
        var i = r[n];
        if (i.compareDocumentPosition(e) === Node.DOCUMENT_POSITION_PRECEDING) {
          return i;
        }
      }
    };
    var G = function (e, t) {
      var r = te().querySelectorAll(t);
      for (var n = r.length - 1; n >= 0; n--) {
        var i = r[n];
        if (i.compareDocumentPosition(e) === Node.DOCUMENT_POSITION_FOLLOWING) {
          return i;
        }
      }
    };
    function le(e, t) {
      if (t) {
        return W(e, t)[0];
      } else {
        return W(te().body, e)[0];
      }
    }
    function s(e) {
      if (L(e, "String")) {
        return E(e);
      } else {
        return e;
      }
    }
    function J(e, t, r) {
      if (A(t)) {
        return {
          target: te().body,
          event: e,
          listener: t,
        };
      } else {
        return {
          target: s(e),
          event: t,
          listener: r,
        };
      }
    }
    function Z(t, r, n) {
      Nr(function () {
        var e = J(t, r, n);
        e.target.addEventListener(e.event, e.listener);
      });
      var e = A(r);
      return e ? r : n;
    }
    function K(t, r, n) {
      Nr(function () {
        var e = J(t, r, n);
        e.target.removeEventListener(e.event, e.listener);
      });
      return A(r) ? r : n;
    }
    var he = te().createElement("output");
    function ve(e, t) {
      var r = re(e, t);
      if (r) {
        if (r === "this") {
          return [de(e, t)];
        } else {
          var n = W(e, r);
          if (n.length === 0) {
            y('The selector "' + r + '" on ' + t + " returned no matches!");
            return [he];
          } else {
            return n;
          }
        }
      }
    }
    function de(e, t) {
      return c(e, function (e) {
        return ee(e, t) != null;
      });
    }
    function ge(e) {
      var t = re(e, "hx-target");
      if (t) {
        if (t === "this") {
          return de(e, "hx-target");
        } else {
          return le(e, t);
        }
      } else {
        var r = ie(e);
        if (r.boosted) {
          return te().body;
        } else {
          return e;
        }
      }
    }
    function me(e) {
      var t = Y.config.attributesToSettle;
      for (var r = 0; r < t.length; r++) {
        if (e === t[r]) {
          return true;
        }
      }
      return false;
    }
    function pe(t, r) {
      ae(t.attributes, function (e) {
        if (!r.hasAttribute(e.name) && me(e.name)) {
          t.removeAttribute(e.name);
        }
      });
      ae(r.attributes, function (e) {
        if (me(e.name)) {
          t.setAttribute(e.name, e.value);
        }
      });
    }
    function xe(e, t) {
      var r = Lr(t);
      for (var n = 0; n < r.length; n++) {
        var i = r[n];
        try {
          if (i.isInlineSwap(e)) {
            return true;
          }
        } catch (e) {
          y(e);
        }
      }
      return e === "outerHTML";
    }
    function ye(e, i, a) {
      var t = "#" + Q(i, "id");
      var o = "outerHTML";
      if (e === "true") {
      } else if (e.indexOf(":") > 0) {
        o = e.substr(0, e.indexOf(":"));
        t = e.substr(e.indexOf(":") + 1, e.length);
      } else {
        o = e;
      }
      var r = te().querySelectorAll(t);
      if (r) {
        ae(r, function (e) {
          var t;
          var r = i.cloneNode(true);
          t = te().createDocumentFragment();
          t.appendChild(r);
          if (!xe(o, e)) {
            t = r;
          }
          var n = {
            shouldSwap: true,
            target: e,
            fragment: t,
          };
          if (!fe(e, "htmx:oobBeforeSwap", n)) return;
          e = n.target;
          if (n["shouldSwap"]) {
            De(o, e, e, t, a);
          }
          ae(a.elts, function (e) {
            fe(e, "htmx:oobAfterSwap", n);
          });
        });
        i.parentNode.removeChild(i);
      } else {
        i.parentNode.removeChild(i);
        ue(te().body, "htmx:oobErrorNoTarget", {
          content: i,
        });
      }
      return e;
    }
    function be(e, t, r) {
      var n = re(e, "hx-select-oob");
      if (n) {
        var i = n.split(",");
        for (let e = 0; e < i.length; e++) {
          var a = i[e].split(":", 2);
          var o = a[0].trim();
          if (o.indexOf("#") === 0) {
            o = o.substring(1);
          }
          var s = a[1] || "true";
          var l = t.querySelector("#" + o);
          if (l) {
            ye(s, l, r);
          }
        }
      }
      ae(f(t, "[hx-swap-oob], [data-hx-swap-oob]"), function (e) {
        var t = ee(e, "hx-swap-oob");
        if (t != null) {
          ye(t, e, r);
        }
      });
    }
    function we(e) {
      ae(f(e, "[hx-preserve], [data-hx-preserve]"), function (e) {
        var t = ee(e, "id");
        var r = te().getElementById(t);
        if (r != null) {
          e.parentNode.replaceChild(r, e);
        }
      });
    }
    function Se(o, e, s) {
      ae(e.querySelectorAll("[id]"), function (e) {
        var t = Q(e, "id");
        if (t && t.length > 0) {
          var r = t.replace("'", "\\'");
          var n = e.tagName.replace(":", "\\:");
          var i = o.querySelector(n + "[id='" + r + "']");
          if (i && i !== o) {
            var a = e.cloneNode();
            pe(e, i);
            s.tasks.push(function () {
              pe(e, a);
            });
          }
        }
      });
    }
    function Ee(e) {
      return function () {
        n(e, Y.config.addedClass);
        Pt(e);
        Ct(e);
        Ce(e);
        fe(e, "htmx:load");
      };
    }
    function Ce(e) {
      var t = "[autofocus]";
      var r = h(e, t) ? e : e.querySelector(t);
      if (r != null) {
        r.focus();
      }
    }
    function a(e, t, r, n) {
      Se(e, r, n);
      while (r.childNodes.length > 0) {
        var i = r.firstChild;
        B(i, Y.config.addedClass);
        e.insertBefore(i, t);
        if (i.nodeType !== Node.TEXT_NODE && i.nodeType !== Node.COMMENT_NODE) {
          n.tasks.push(Ee(i));
        }
      }
    }
    function Te(e, t) {
      var r = 0;
      while (r < e.length) {
        t = ((t << 5) - t + e.charCodeAt(r++)) | 0;
      }
      return t;
    }
    function Re(e) {
      var t = 0;
      if (e.attributes) {
        for (var r = 0; r < e.attributes.length; r++) {
          var n = e.attributes[r];
          if (n.value) {
            t = Te(n.name, t);
            t = Te(n.value, t);
          }
        }
      }
      return t;
    }
    function Oe(t) {
      var r = ie(t);
      if (r.onHandlers) {
        for (let e = 0; e < r.onHandlers.length; e++) {
          const n = r.onHandlers[e];
          t.removeEventListener(n.event, n.listener);
        }
        delete r.onHandlers;
      }
    }
    function qe(e) {
      var t = ie(e);
      if (t.timeout) {
        clearTimeout(t.timeout);
      }
      if (t.webSocket) {
        t.webSocket.close();
      }
      if (t.sseEventSource) {
        t.sseEventSource.close();
      }
      if (t.listenerInfos) {
        ae(t.listenerInfos, function (e) {
          if (e.on) {
            e.on.removeEventListener(e.trigger, e.listener);
          }
        });
      }
      if (t.initHash) {
        t.initHash = null;
      }
      Oe(e);
    }
    function m(e) {
      fe(e, "htmx:beforeCleanupElement");
      qe(e);
      if (e.children) {
        ae(e.children, function (e) {
          m(e);
        });
      }
    }
    function He(t, e, r) {
      if (t.tagName === "BODY") {
        return ke(t, e, r);
      } else {
        var n;
        var i = t.previousSibling;
        a(u(t), t, e, r);
        if (i == null) {
          n = u(t).firstChild;
        } else {
          n = i.nextSibling;
        }
        ie(t).replacedWith = n;
        r.elts = r.elts.filter(function (e) {
          return e != t;
        });
        while (n && n !== t) {
          if (n.nodeType === Node.ELEMENT_NODE) {
            r.elts.push(n);
          }
          n = n.nextElementSibling;
        }
        m(t);
        u(t).removeChild(t);
      }
    }
    function Le(e, t, r) {
      return a(e, e.firstChild, t, r);
    }
    function Ae(e, t, r) {
      return a(u(e), e, t, r);
    }
    function Ne(e, t, r) {
      return a(e, null, t, r);
    }
    function Ie(e, t, r) {
      return a(u(e), e.nextSibling, t, r);
    }
    function Pe(e, t, r) {
      m(e);
      return u(e).removeChild(e);
    }
    function ke(e, t, r) {
      var n = e.firstChild;
      a(e, n, t, r);
      if (n) {
        while (n.nextSibling) {
          m(n.nextSibling);
          e.removeChild(n.nextSibling);
        }
        m(n);
        e.removeChild(n);
      }
    }
    function Me(e, t, r) {
      var n = r || re(e, "hx-select");
      if (n) {
        var i = te().createDocumentFragment();
        ae(t.querySelectorAll(n), function (e) {
          i.appendChild(e);
        });
        t = i;
      }
      return t;
    }
    function De(e, t, r, n, i) {
      switch (e) {
        case "none":
          return;
        case "outerHTML":
          He(r, n, i);
          return;
        case "afterbegin":
          Le(r, n, i);
          return;
        case "beforebegin":
          Ae(r, n, i);
          return;
        case "beforeend":
          Ne(r, n, i);
          return;
        case "afterend":
          Ie(r, n, i);
          return;
        case "delete":
          Pe(r, n, i);
          return;
        default:
          var a = Lr(t);
          for (var o = 0; o < a.length; o++) {
            var s = a[o];
            try {
              var l = s.handleSwap(e, r, n, i);
              if (l) {
                if (typeof l.length !== "undefined") {
                  for (var u = 0; u < l.length; u++) {
                    var f = l[u];
                    if (
                      f.nodeType !== Node.TEXT_NODE &&
                      f.nodeType !== Node.COMMENT_NODE
                    ) {
                      i.tasks.push(Ee(f));
                    }
                  }
                }
                return;
              }
            } catch (e) {
              y(e);
            }
          }
          if (e === "innerHTML") {
            ke(r, n, i);
          } else {
            De(Y.config.defaultSwapStyle, t, r, n, i);
          }
      }
    }
    function Xe(e) {
      if (e.indexOf("<title") > -1) {
        var t = e.replace(/<svg(\s[^>]*>|>)([\s\S]*?)<\/svg>/gim, "");
        var r = t.match(/<title(\s[^>]*>|>)([\s\S]*?)<\/title>/im);
        if (r) {
          return r[2];
        }
      }
    }
    function Fe(e, t, r, n, i, a) {
      i.title = Xe(n);
      var o = l(n);
      if (o) {
        be(r, o, i);
        o = Me(r, o, a);
        we(o);
        return De(e, r, t, o, i);
      }
    }
    function Ue(e, t, r) {
      var n = e.getResponseHeader(t);
      if (n.indexOf("{") === 0) {
        var i = S(n);
        for (var a in i) {
          if (i.hasOwnProperty(a)) {
            var o = i[a];
            if (!N(o)) {
              o = {
                value: o,
              };
            }
            fe(r, a, o);
          }
        }
      } else {
        var s = n.split(",");
        for (var l = 0; l < s.length; l++) {
          fe(r, s[l].trim(), []);
        }
      }
    }
    var Be = /\s/;
    var p = /[\s,]/;
    var Ve = /[_$a-zA-Z]/;
    var je = /[_$a-zA-Z0-9]/;
    var _e = ['"', "'", "/"];
    var ze = /[^\s]/;
    function We(e) {
      var t = [];
      var r = 0;
      while (r < e.length) {
        if (Ve.exec(e.charAt(r))) {
          var n = r;
          while (je.exec(e.charAt(r + 1))) {
            r++;
          }
          t.push(e.substr(n, r - n + 1));
        } else if (_e.indexOf(e.charAt(r)) !== -1) {
          var i = e.charAt(r);
          var n = r;
          r++;
          while (r < e.length && e.charAt(r) !== i) {
            if (e.charAt(r) === "\\") {
              r++;
            }
            r++;
          }
          t.push(e.substr(n, r - n + 1));
        } else {
          var a = e.charAt(r);
          t.push(a);
        }
        r++;
      }
      return t;
    }
    function $e(e, t, r) {
      return (
        Ve.exec(e.charAt(0)) &&
        e !== "true" &&
        e !== "false" &&
        e !== "this" &&
        e !== r &&
        t !== "."
      );
    }
    function Ge(e, t, r) {
      if (t[0] === "[") {
        t.shift();
        var n = 1;
        var i = " return (function(" + r + "){ return (";
        var a = null;
        while (t.length > 0) {
          var o = t[0];
          if (o === "]") {
            n--;
            if (n === 0) {
              if (a === null) {
                i = i + "true";
              }
              t.shift();
              i += ")})";
              try {
                var s = gr(
                  e,
                  function () {
                    return Function(i)();
                  },
                  function () {
                    return true;
                  }
                );
                s.source = i;
                return s;
              } catch (e) {
                ue(te().body, "htmx:syntax:error", {
                  error: e,
                  source: i,
                });
                return null;
              }
            }
          } else if (o === "[") {
            n++;
          }
          if ($e(o, a, r)) {
            i +=
              "((" +
              r +
              "." +
              o +
              ") ? (" +
              r +
              "." +
              o +
              ") : (window." +
              o +
              "))";
          } else {
            i = i + o;
          }
          a = t.shift();
        }
      }
    }
    function x(e, t) {
      var r = "";
      while (e.length > 0 && !e[0].match(t)) {
        r += e.shift();
      }
      return r;
    }
    var Je = "input, textarea, select";
    function Ze(e) {
      var t = ee(e, "hx-trigger");
      var r = [];
      if (t) {
        var n = We(t);
        do {
          x(n, ze);
          var i = n.length;
          var a = x(n, /[,\[\s]/);
          if (a !== "") {
            if (a === "every") {
              var o = {
                trigger: "every",
              };
              x(n, ze);
              o.pollInterval = d(x(n, /[,\[\s]/));
              x(n, ze);
              var s = Ge(e, n, "event");
              if (s) {
                o.eventFilter = s;
              }
              r.push(o);
            } else if (a.indexOf("sse:") === 0) {
              r.push({
                trigger: "sse",
                sseEvent: a.substr(4),
              });
            } else {
              var l = {
                trigger: a,
              };
              var s = Ge(e, n, "event");
              if (s) {
                l.eventFilter = s;
              }
              while (n.length > 0 && n[0] !== ",") {
                x(n, ze);
                var u = n.shift();
                if (u === "changed") {
                  l.changed = true;
                } else if (u === "once") {
                  l.once = true;
                } else if (u === "consume") {
                  l.consume = true;
                } else if (u === "delay" && n[0] === ":") {
                  n.shift();
                  l.delay = d(x(n, p));
                } else if (u === "from" && n[0] === ":") {
                  n.shift();
                  var f = x(n, p);
                  if (
                    f === "closest" ||
                    f === "find" ||
                    f === "next" ||
                    f === "previous"
                  ) {
                    n.shift();
                    f += " " + x(n, p);
                  }
                  l.from = f;
                } else if (u === "target" && n[0] === ":") {
                  n.shift();
                  l.target = x(n, p);
                } else if (u === "throttle" && n[0] === ":") {
                  n.shift();
                  l.throttle = d(x(n, p));
                } else if (u === "queue" && n[0] === ":") {
                  n.shift();
                  l.queue = x(n, p);
                } else if (
                  (u === "root" || u === "threshold") &&
                  n[0] === ":"
                ) {
                  n.shift();
                  l[u] = x(n, p);
                } else {
                  ue(e, "htmx:syntax:error", {
                    token: n.shift(),
                  });
                }
              }
              r.push(l);
            }
          }
          if (n.length === i) {
            ue(e, "htmx:syntax:error", {
              token: n.shift(),
            });
          }
          x(n, ze);
        } while (n[0] === "," && n.shift());
      }
      if (r.length > 0) {
        return r;
      } else if (h(e, "form")) {
        return [
          {
            trigger: "submit",
          },
        ];
      } else if (h(e, 'input[type="button"], input[type="submit"]')) {
        return [
          {
            trigger: "click",
          },
        ];
      } else if (h(e, Je)) {
        return [
          {
            trigger: "change",
          },
        ];
      } else {
        return [
          {
            trigger: "click",
          },
        ];
      }
    }
    function Ke(e) {
      ie(e).cancelled = true;
    }
    function Ye(e, t, r) {
      var n = ie(e);
      n.timeout = setTimeout(function () {
        if (oe(e) && n.cancelled !== true) {
          if (
            !nt(
              r,
              e,
              Mt("hx:poll:trigger", {
                triggerSpec: r,
                target: e,
              })
            )
          ) {
            t(e);
          }
          Ye(e, t, r);
        }
      }, r.pollInterval);
    }
    function Qe(e) {
      return (
        location.hostname === e.hostname &&
        Q(e, "href") &&
        Q(e, "href").indexOf("#") !== 0
      );
    }
    function et(t, r, e) {
      if (
        (t.tagName === "A" &&
          Qe(t) &&
          (t.target === "" || t.target === "_self")) ||
        t.tagName === "FORM"
      ) {
        r.boosted = true;
        var n, i;
        if (t.tagName === "A") {
          n = "get";
          i = Q(t, "href");
        } else {
          var a = Q(t, "method");
          n = a ? a.toLowerCase() : "get";
          if (n === "get") {
          }
          i = Q(t, "action");
        }
        e.forEach(function (e) {
          it(
            t,
            function (e, t) {
              if (v(e, Y.config.disableSelector)) {
                m(e);
                return;
              }
              ce(n, i, e, t);
            },
            r,
            e,
            true
          );
        });
      }
    }
    function tt(e, t) {
      if (e.type === "submit" || e.type === "click") {
        if (t.tagName === "FORM") {
          return true;
        }
        if (h(t, 'input[type="submit"], button') && v(t, "form") !== null) {
          return true;
        }
        if (
          t.tagName === "A" &&
          t.href &&
          (t.getAttribute("href") === "#" ||
            t.getAttribute("href").indexOf("#") !== 0)
        ) {
          return true;
        }
      }
      return false;
    }
    function rt(e, t) {
      return (
        ie(e).boosted &&
        e.tagName === "A" &&
        t.type === "click" &&
        (t.ctrlKey || t.metaKey)
      );
    }
    function nt(e, t, r) {
      var n = e.eventFilter;
      if (n) {
        try {
          return n.call(t, r) !== true;
        } catch (e) {
          ue(te().body, "htmx:eventFilter:error", {
            error: e,
            source: n.source,
          });
          return true;
        }
      }
      return false;
    }
    function it(a, o, e, s, l) {
      var u = ie(a);
      var t;
      if (s.from) {
        t = W(a, s.from);
      } else {
        t = [a];
      }
      if (s.changed) {
        t.forEach(function (e) {
          var t = ie(e);
          t.lastValue = e.value;
        });
      }
      ae(t, function (n) {
        var i = function (e) {
          if (!oe(a)) {
            n.removeEventListener(s.trigger, i);
            return;
          }
          if (rt(a, e)) {
            return;
          }
          if (l || tt(e, a)) {
            e.preventDefault();
          }
          if (nt(s, a, e)) {
            return;
          }
          var t = ie(e);
          t.triggerSpec = s;
          if (t.handledFor == null) {
            t.handledFor = [];
          }
          if (t.handledFor.indexOf(a) < 0) {
            t.handledFor.push(a);
            if (s.consume) {
              e.stopPropagation();
            }
            if (s.target && e.target) {
              if (!h(e.target, s.target)) {
                return;
              }
            }
            if (s.once) {
              if (u.triggeredOnce) {
                return;
              } else {
                u.triggeredOnce = true;
              }
            }
            if (s.changed) {
              var r = ie(n);
              if (r.lastValue === n.value) {
                return;
              }
              r.lastValue = n.value;
            }
            if (u.delayed) {
              clearTimeout(u.delayed);
            }
            if (u.throttle) {
              return;
            }
            if (s.throttle) {
              if (!u.throttle) {
                o(a, e);
                u.throttle = setTimeout(function () {
                  u.throttle = null;
                }, s.throttle);
              }
            } else if (s.delay) {
              u.delayed = setTimeout(function () {
                o(a, e);
              }, s.delay);
            } else {
              fe(a, "htmx:trigger");
              o(a, e);
            }
          }
        };
        if (e.listenerInfos == null) {
          e.listenerInfos = [];
        }
        e.listenerInfos.push({
          trigger: s.trigger,
          listener: i,
          on: n,
        });
        n.addEventListener(s.trigger, i);
      });
    }
    var at = false;
    var ot = null;
    function st() {
      if (!ot) {
        ot = function () {
          at = true;
        };
        window.addEventListener("scroll", ot);
        setInterval(function () {
          if (at) {
            at = false;
            ae(
              te().querySelectorAll(
                "[hx-trigger='revealed'],[data-hx-trigger='revealed']"
              ),
              function (e) {
                lt(e);
              }
            );
          }
        }, 200);
      }
    }
    function lt(t) {
      if (!o(t, "data-hx-revealed") && P(t)) {
        t.setAttribute("data-hx-revealed", "true");
        var e = ie(t);
        if (e.initHash) {
          fe(t, "revealed");
        } else {
          t.addEventListener(
            "htmx:afterProcessNode",
            function (e) {
              fe(t, "revealed");
            },
            {
              once: true,
            }
          );
        }
      }
    }
    function ut(e, t, r) {
      var n = k(r);
      for (var i = 0; i < n.length; i++) {
        var a = n[i].split(/:(.+)/);
        if (a[0] === "connect") {
          ft(e, a[1], 0);
        }
        if (a[0] === "send") {
          ht(e);
        }
      }
    }
    function ft(s, r, n) {
      if (!oe(s)) {
        return;
      }
      if (r.indexOf("/") == 0) {
        var e = location.hostname + (location.port ? ":" + location.port : "");
        if (location.protocol == "https:") {
          r = "wss://" + e + r;
        } else if (location.protocol == "http:") {
          r = "ws://" + e + r;
        }
      }
      var t = Y.createWebSocket(r);
      t.onerror = function (e) {
        ue(s, "htmx:wsError", {
          error: e,
          socket: t,
        });
        ct(s);
      };
      t.onclose = function (e) {
        if ([1006, 1012, 1013].indexOf(e.code) >= 0) {
          var t = vt(n);
          setTimeout(function () {
            ft(s, r, n + 1);
          }, t);
        }
      };
      t.onopen = function (e) {
        n = 0;
      };
      ie(s).webSocket = t;
      t.addEventListener("message", function (e) {
        if (ct(s)) {
          return;
        }
        var t = e.data;
        C(s, function (e) {
          t = e.transformResponse(t, null, s);
        });
        var r = T(s);
        var n = l(t);
        var i = I(n.children);
        for (var a = 0; a < i.length; a++) {
          var o = i[a];
          ye(ee(o, "hx-swap-oob") || "true", o, r);
        }
        Wt(r.tasks);
      });
    }
    function ct(e) {
      if (!oe(e)) {
        ie(e).webSocket.close();
        return true;
      }
    }
    function ht(u) {
      var f = c(u, function (e) {
        return ie(e).webSocket != null;
      });
      if (f) {
        u.addEventListener(Ze(u)[0].trigger, function (e) {
          var t = ie(f).webSocket;
          var r = sr(u, f);
          var n = nr(u, "post");
          var i = n.errors;
          var a = n.values;
          var o = xr(u);
          var s = se(a, o);
          var l = lr(s, u);
          l["HEADERS"] = r;
          if (i && i.length > 0) {
            fe(u, "htmx:validation:halted", i);
            return;
          }
          t.send(JSON.stringify(l));
          if (tt(e, u)) {
            e.preventDefault();
          }
        });
      } else {
        ue(u, "htmx:noWebSocketSourceError");
      }
    }
    function vt(e) {
      var t = Y.config.wsReconnectDelay;
      if (typeof t === "function") {
        return t(e);
      }
      if (t === "full-jitter") {
        var r = Math.min(e, 6);
        var n = 1e3 * Math.pow(2, r);
        return n * Math.random();
      }
      y(
        'htmx.config.wsReconnectDelay must either be a function or the string "full-jitter"'
      );
    }
    function dt(e, t, r) {
      var n = k(r);
      for (var i = 0; i < n.length; i++) {
        var a = n[i].split(/:(.+)/);
        if (a[0] === "connect") {
          gt(e, a[1]);
        }
        if (a[0] === "swap") {
          mt(e, a[1]);
        }
      }
    }
    function gt(t, e) {
      var r = Y.createEventSource(e);
      r.onerror = function (e) {
        ue(t, "htmx:sseError", {
          error: e,
          source: r,
        });
        xt(t);
      };
      ie(t).sseEventSource = r;
    }
    function mt(a, o) {
      var s = c(a, yt);
      if (s) {
        var l = ie(s).sseEventSource;
        var u = function (e) {
          if (xt(s)) {
            return;
          }
          if (!oe(a)) {
            l.removeEventListener(o, u);
            return;
          }
          var t = e.data;
          C(a, function (e) {
            t = e.transformResponse(t, null, a);
          });
          var r = fr(a);
          var n = ge(a);
          var i = T(a);
          Fe(r.swapStyle, n, a, t, i);
          Wt(i.tasks);
          fe(a, "htmx:sseMessage", e);
        };
        ie(a).sseListener = u;
        l.addEventListener(o, u);
      } else {
        ue(a, "htmx:noSSESourceError");
      }
    }
    function pt(e, t, r) {
      var n = c(e, yt);
      if (n) {
        var i = ie(n).sseEventSource;
        var a = function () {
          if (!xt(n)) {
            if (oe(e)) {
              t(e);
            } else {
              i.removeEventListener(r, a);
            }
          }
        };
        ie(e).sseListener = a;
        i.addEventListener(r, a);
      } else {
        ue(e, "htmx:noSSESourceError");
      }
    }
    function xt(e) {
      if (!oe(e)) {
        ie(e).sseEventSource.close();
        return true;
      }
    }
    function yt(e) {
      return ie(e).sseEventSource != null;
    }
    function bt(e, t, r, n) {
      var i = function () {
        if (!r.loaded) {
          r.loaded = true;
          t(e);
        }
      };
      if (n) {
        setTimeout(i, n);
      } else {
        i();
      }
    }
    function wt(t, i, e) {
      var a = false;
      ae(b, function (r) {
        if (o(t, "hx-" + r)) {
          var n = ee(t, "hx-" + r);
          a = true;
          i.path = n;
          i.verb = r;
          e.forEach(function (e) {
            St(t, e, i, function (e, t) {
              if (v(e, Y.config.disableSelector)) {
                m(e);
                return;
              }
              ce(r, n, e, t);
            });
          });
        }
      });
      return a;
    }
    function St(n, e, t, r) {
      if (e.sseEvent) {
        pt(n, r, e.sseEvent);
      } else if (e.trigger === "revealed") {
        st();
        it(n, r, t, e);
        lt(n);
      } else if (e.trigger === "intersect") {
        var i = {};
        if (e.root) {
          i.root = le(n, e.root);
        }
        if (e.threshold) {
          i.threshold = parseFloat(e.threshold);
        }
        var a = new IntersectionObserver(function (e) {
          for (var t = 0; t < e.length; t++) {
            var r = e[t];
            if (r.isIntersecting) {
              fe(n, "intersect");
              break;
            }
          }
        }, i);
        a.observe(n);
        it(n, r, t, e);
      } else if (e.trigger === "load") {
        if (
          !nt(
            e,
            n,
            Mt("load", {
              elt: n,
            })
          )
        ) {
          bt(n, r, t, e.delay);
        }
      } else if (e.pollInterval) {
        t.polling = true;
        Ye(n, r, e);
      } else {
        it(n, r, t, e);
      }
    }
    function Et(e) {
      if (
        Y.config.allowScriptTags &&
        (e.type === "text/javascript" || e.type === "module" || e.type === "")
      ) {
        var t = te().createElement("script");
        ae(e.attributes, function (e) {
          t.setAttribute(e.name, e.value);
        });
        t.textContent = e.textContent;
        t.async = false;
        if (Y.config.inlineScriptNonce) {
          t.nonce = Y.config.inlineScriptNonce;
        }
        var r = e.parentElement;
        try {
          r.insertBefore(t, e);
        } catch (e) {
          y(e);
        } finally {
          if (e.parentElement) {
            e.parentElement.removeChild(e);
          }
        }
      }
    }
    function Ct(e) {
      if (h(e, "script")) {
        Et(e);
      }
      ae(f(e, "script"), function (e) {
        Et(e);
      });
    }
    function Tt() {
      return document.querySelector("[hx-boost], [data-hx-boost]");
    }
    function Rt(e) {
      var t = null;
      var r = [];
      if (document.evaluate) {
        var n = document.evaluate(
          '//*[@*[ starts-with(name(), "hx-on:") or starts-with(name(), "data-hx-on:") ]]',
          e
        );
        while ((t = n.iterateNext())) r.push(t);
      } else {
        var i = document.getElementsByTagName("*");
        for (var a = 0; a < i.length; a++) {
          var o = i[a].attributes;
          for (var s = 0; s < o.length; s++) {
            var l = o[s].name;
            if (g(l, "hx-on:") || g(l, "data-hx-on:")) {
              r.push(i[a]);
            }
          }
        }
      }
      return r;
    }
    function Ot(e) {
      if (e.querySelectorAll) {
        var t = Tt() ? ", a" : "";
        var r = e.querySelectorAll(
          w +
            t +
            ", form, [type='submit'], [hx-sse], [data-hx-sse], [hx-ws]," +
            " [data-hx-ws], [hx-ext], [data-hx-ext], [hx-trigger], [data-hx-trigger], [hx-on], [data-hx-on]"
        );
        return r;
      } else {
        return [];
      }
    }
    function qt(e) {
      var n = s("#" + Q(e, "form")) || v(e, "form");
      if (!n) {
        return;
      }
      var t = function (e) {
        var t = v(e.target, "button, input[type='submit']");
        if (t !== null) {
          var r = ie(n);
          r.lastButtonClicked = t;
        }
      };
      e.addEventListener("click", t);
      e.addEventListener("focusin", t);
      e.addEventListener("focusout", function (e) {
        var t = ie(n);
        t.lastButtonClicked = null;
      });
    }
    function Ht(e) {
      var t = We(e);
      var r = 0;
      for (let e = 0; e < t.length; e++) {
        const n = t[e];
        if (n === "{") {
          r++;
        } else if (n === "}") {
          r--;
        }
      }
      return r;
    }
    function Lt(t, e, r) {
      var n = ie(t);
      n.onHandlers = [];
      var i;
      var a = function (e) {
        return gr(t, function () {
          if (!i) {
            i = new Function("event", r);
          }
          i.call(t, e);
        });
      };
      t.addEventListener(e, a);
      n.onHandlers.push({
        event: e,
        listener: a,
      });
    }
    function At(e) {
      var t = ee(e, "hx-on");
      if (t) {
        var r = {};
        var n = t.split("\n");
        var i = null;
        var a = 0;
        while (n.length > 0) {
          var o = n.shift();
          var s = o.match(/^\s*([a-zA-Z:\-\.]+:)(.*)/);
          if (a === 0 && s) {
            o.split(":");
            i = s[1].slice(0, -1);
            r[i] = s[2];
          } else {
            r[i] += o;
          }
          a += Ht(o);
        }
        for (var l in r) {
          Lt(e, l, r[l]);
        }
      }
    }
    function Nt(t) {
      Oe(t);
      for (var e = 0; e < t.attributes.length; e++) {
        var r = t.attributes[e].name;
        var n = t.attributes[e].value;
        if (g(r, "hx-on:") || g(r, "data-hx-on:")) {
          let e = r.slice(r.indexOf(":") + 1);
          if (g(e, ":")) e = "htmx" + e;
          Lt(t, e, n);
        }
      }
    }
    function It(t) {
      if (v(t, Y.config.disableSelector)) {
        m(t);
        return;
      }
      var r = ie(t);
      if (r.initHash !== Re(t)) {
        qe(t);
        r.initHash = Re(t);
        At(t);
        fe(t, "htmx:beforeProcessNode");
        if (t.value) {
          r.lastValue = t.value;
        }
        var e = Ze(t);
        var n = wt(t, r, e);
        if (!n) {
          if (re(t, "hx-boost") === "true") {
            et(t, r, e);
          } else if (o(t, "hx-trigger")) {
            e.forEach(function (e) {
              St(t, e, r, function () {});
            });
          }
        }
        if (
          t.tagName === "FORM" ||
          (Q(t, "type") === "submit" && o(t, "form"))
        ) {
          qt(t);
        }
        var i = ee(t, "hx-sse");
        if (i) {
          dt(t, r, i);
        }
        var a = ee(t, "hx-ws");
        if (a) {
          ut(t, r, a);
        }
        fe(t, "htmx:afterProcessNode");
      }
    }
    function Pt(e) {
      e = s(e);
      if (v(e, Y.config.disableSelector)) {
        m(e);
        return;
      }
      It(e);
      ae(Ot(e), function (e) {
        It(e);
      });
      ae(Rt(e), Nt);
    }
    function kt(e) {
      return e.replace(/([a-z0-9])([A-Z])/g, "$1-$2").toLowerCase();
    }
    function Mt(e, t) {
      var r;
      if (window.CustomEvent && typeof window.CustomEvent === "function") {
        r = new CustomEvent(e, {
          bubbles: true,
          cancelable: true,
          detail: t,
        });
      } else {
        r = te().createEvent("CustomEvent");
        r.initCustomEvent(e, true, true, t);
      }
      return r;
    }
    function ue(e, t, r) {
      fe(
        e,
        t,
        se(
          {
            error: t,
          },
          r
        )
      );
    }
    function Dt(e) {
      return e === "htmx:afterProcessNode";
    }
    function C(e, t) {
      ae(Lr(e), function (e) {
        try {
          t(e);
        } catch (e) {
          y(e);
        }
      });
    }
    function y(e) {
      if (console.error) {
        console.error(e);
      } else if (console.log) {
        console.log("ERROR: ", e);
      }
    }
    function fe(e, t, r) {
      e = s(e);
      if (r == null) {
        r = {};
      }
      r["elt"] = e;
      var n = Mt(t, r);
      if (Y.logger && !Dt(t)) {
        Y.logger(e, t, r);
      }
      if (r.error) {
        y(r.error);
        fe(e, "htmx:error", {
          errorInfo: r,
        });
      }
      var i = e.dispatchEvent(n);
      var a = kt(t);
      if (i && a !== t) {
        var o = Mt(a, n.detail);
        i = i && e.dispatchEvent(o);
      }
      C(e, function (e) {
        i = i && e.onEvent(t, n) !== false && !n.defaultPrevented;
      });
      return i;
    }
    var Xt = location.pathname + location.search;
    function Ft() {
      var e = te().querySelector("[hx-history-elt],[data-hx-history-elt]");
      return e || te().body;
    }
    function Ut(e, t, r, n) {
      if (!M()) {
        return;
      }
      e = D(e);
      var i = S(localStorage.getItem("htmx-history-cache")) || [];
      for (var a = 0; a < i.length; a++) {
        if (i[a].url === e) {
          i.splice(a, 1);
          break;
        }
      }
      var o = {
        url: e,
        content: t,
        title: r,
        scroll: n,
      };
      fe(te().body, "htmx:historyItemCreated", {
        item: o,
        cache: i,
      });
      i.push(o);
      while (i.length > Y.config.historyCacheSize) {
        i.shift();
      }
      while (i.length > 0) {
        try {
          localStorage.setItem("htmx-history-cache", JSON.stringify(i));
          break;
        } catch (e) {
          ue(te().body, "htmx:historyCacheError", {
            cause: e,
            cache: i,
          });
          i.shift();
        }
      }
    }
    function Bt(e) {
      if (!M()) {
        return null;
      }
      e = D(e);
      var t = S(localStorage.getItem("htmx-history-cache")) || [];
      for (var r = 0; r < t.length; r++) {
        if (t[r].url === e) {
          return t[r];
        }
      }
      return null;
    }
    function Vt(e) {
      var t = Y.config.requestClass;
      var r = e.cloneNode(true);
      ae(f(r, "." + t), function (e) {
        n(e, t);
      });
      return r.innerHTML;
    }
    function jt() {
      var e = Ft();
      var t = Xt || location.pathname + location.search;
      var r;
      try {
        r = te().querySelector(
          '[hx-history="false" i],[data-hx-history="false" i]'
        );
      } catch (e) {
        r = te().querySelector(
          '[hx-history="false"],[data-hx-history="false"]'
        );
      }
      if (!r) {
        fe(te().body, "htmx:beforeHistorySave", {
          path: t,
          historyElt: e,
        });
        Ut(t, Vt(e), te().title, window.scrollY);
      }
      if (Y.config.historyEnabled)
        history.replaceState(
          {
            htmx: true,
          },
          te().title,
          window.location.href
        );
    }
    function _t(e) {
      if (Y.config.getCacheBusterParam) {
        e = e.replace(/org\.htmx\.cache-buster=[^&]*&?/, "");
        if (_(e, "&") || _(e, "?")) {
          e = e.slice(0, -1);
        }
      }
      if (Y.config.historyEnabled) {
        history.pushState(
          {
            htmx: true,
          },
          "",
          e
        );
      }
      Xt = e;
    }
    function zt(e) {
      if (Y.config.historyEnabled)
        history.replaceState(
          {
            htmx: true,
          },
          "",
          e
        );
      Xt = e;
    }
    function Wt(e) {
      ae(e, function (e) {
        e.call();
      });
    }
    function $t(a) {
      var e = new XMLHttpRequest();
      var o = {
        path: a,
        xhr: e,
      };
      fe(te().body, "htmx:historyCacheMiss", o);
      e.open("GET", a, true);
      e.setRequestHeader("HX-History-Restore-Request", "true");
      e.onload = function () {
        if (this.status >= 200 && this.status < 400) {
          fe(te().body, "htmx:historyCacheMissLoad", o);
          var e = l(this.response);
          e = e.querySelector("[hx-history-elt],[data-hx-history-elt]") || e;
          var t = Ft();
          var r = T(t);
          var n = Xe(this.response);
          if (n) {
            var i = E("title");
            if (i) {
              i.innerHTML = n;
            } else {
              window.document.title = n;
            }
          }
          ke(t, e, r);
          Wt(r.tasks);
          Xt = a;
          fe(te().body, "htmx:historyRestore", {
            path: a,
            cacheMiss: true,
            serverResponse: this.response,
          });
        } else {
          ue(te().body, "htmx:historyCacheMissLoadError", o);
        }
      };
      e.send();
    }
    function Gt(e) {
      jt();
      e = e || location.pathname + location.search;
      var t = Bt(e);
      if (t) {
        var r = l(t.content);
        var n = Ft();
        var i = T(n);
        ke(n, r, i);
        Wt(i.tasks);
        document.title = t.title;
        setTimeout(function () {
          window.scrollTo(0, t.scroll);
        }, 0);
        Xt = e;
        fe(te().body, "htmx:historyRestore", {
          path: e,
          item: t,
        });
      } else {
        if (Y.config.refreshOnHistoryMiss) {
          window.location.reload(true);
        } else {
          $t(e);
        }
      }
    }
    function Jt(e) {
      var t = ve(e, "hx-indicator");
      if (t == null) {
        t = [e];
      }
      ae(t, function (e) {
        var t = ie(e);
        t.requestCount = (t.requestCount || 0) + 1;
        e.classList["add"].call(e.classList, Y.config.requestClass);
      });
      return t;
    }
    function Zt(e) {
      var t = ve(e, "hx-disabled-elt");
      if (t == null) {
        t = [];
      }
      ae(t, function (e) {
        var t = ie(e);
        t.requestCount = (t.requestCount || 0) + 1;
        e.setAttribute("disabled", "");
      });
      return t;
    }
    function Kt(e, t) {
      ae(e, function (e) {
        var t = ie(e);
        t.requestCount = (t.requestCount || 0) - 1;
        if (t.requestCount === 0) {
          e.classList["remove"].call(e.classList, Y.config.requestClass);
        }
      });
      ae(t, function (e) {
        var t = ie(e);
        t.requestCount = (t.requestCount || 0) - 1;
        if (t.requestCount === 0) {
          e.removeAttribute("disabled");
        }
      });
    }
    function Yt(e, t) {
      for (var r = 0; r < e.length; r++) {
        var n = e[r];
        if (n.isSameNode(t)) {
          return true;
        }
      }
      return false;
    }
    function Qt(e) {
      if (e.name === "" || e.name == null || e.disabled) {
        return false;
      }
      if (
        e.type === "button" ||
        e.type === "submit" ||
        e.tagName === "image" ||
        e.tagName === "reset" ||
        e.tagName === "file"
      ) {
        return false;
      }
      if (e.type === "checkbox" || e.type === "radio") {
        return e.checked;
      }
      return true;
    }
    function er(e, t, r) {
      if (e != null && t != null) {
        var n = r[e];
        if (n === undefined) {
          r[e] = t;
        } else if (Array.isArray(n)) {
          if (Array.isArray(t)) {
            r[e] = n.concat(t);
          } else {
            n.push(t);
          }
        } else {
          if (Array.isArray(t)) {
            r[e] = [n].concat(t);
          } else {
            r[e] = [n, t];
          }
        }
      }
    }
    function tr(t, r, n, e, i) {
      if (e == null || Yt(t, e)) {
        return;
      } else {
        t.push(e);
      }
      if (Qt(e)) {
        var a = Q(e, "name");
        var o = e.value;
        if (e.multiple) {
          o = I(e.querySelectorAll("option:checked")).map(function (e) {
            return e.value;
          });
        }
        if (e.files) {
          o = I(e.files);
        }
        er(a, o, r);
        if (i) {
          rr(e, n);
        }
      }
      if (h(e, "form")) {
        var s = e.elements;
        ae(s, function (e) {
          tr(t, r, n, e, i);
        });
      }
    }
    function rr(e, t) {
      if (e.willValidate) {
        fe(e, "htmx:validation:validate");
        if (!e.checkValidity()) {
          t.push({
            elt: e,
            message: e.validationMessage,
            validity: e.validity,
          });
          fe(e, "htmx:validation:failed", {
            message: e.validationMessage,
            validity: e.validity,
          });
        }
      }
    }
    function nr(e, t) {
      var r = [];
      var n = {};
      var i = {};
      var a = [];
      var o = ie(e);
      var s =
        (h(e, "form") && e.noValidate !== true) ||
        ee(e, "hx-validate") === "true";
      if (o.lastButtonClicked) {
        s = s && o.lastButtonClicked.formNoValidate !== true;
      }
      if (t !== "get") {
        tr(r, i, a, v(e, "form"), s);
      }
      tr(r, n, a, e, s);
      if (
        o.lastButtonClicked ||
        e.tagName === "BUTTON" ||
        (e.tagName === "INPUT" && Q(e, "type") === "submit")
      ) {
        var l = o.lastButtonClicked || e;
        var u = Q(l, "name");
        er(u, l.value, i);
      }
      var f = ve(e, "hx-include");
      ae(f, function (e) {
        tr(r, n, a, e, s);
        if (!h(e, "form")) {
          ae(e.querySelectorAll(Je), function (e) {
            tr(r, n, a, e, s);
          });
        }
      });
      n = se(n, i);
      return {
        errors: a,
        values: n,
      };
    }
    function ir(e, t, r) {
      if (e !== "") {
        e += "&";
      }
      if (String(r) === "[object Object]") {
        r = JSON.stringify(r);
      }
      var n = encodeURIComponent(r);
      e += encodeURIComponent(t) + "=" + n;
      return e;
    }
    function ar(e) {
      var t = "";
      for (var r in e) {
        if (e.hasOwnProperty(r)) {
          var n = e[r];
          if (Array.isArray(n)) {
            ae(n, function (e) {
              t = ir(t, r, e);
            });
          } else {
            t = ir(t, r, n);
          }
        }
      }
      return t;
    }
    function or(e) {
      var t = new FormData();
      for (var r in e) {
        if (e.hasOwnProperty(r)) {
          var n = e[r];
          if (Array.isArray(n)) {
            ae(n, function (e) {
              t.append(r, e);
            });
          } else {
            t.append(r, n);
          }
        }
      }
      return t;
    }
    function sr(e, t, r) {
      var n = {
        "HX-Request": "true",
        "HX-Trigger": Q(e, "id"),
        "HX-Trigger-Name": Q(e, "name"),
        "HX-Target": ee(t, "id"),
        "HX-Current-URL": te().location.href,
      };
      dr(e, "hx-headers", false, n);
      if (r !== undefined) {
        n["HX-Prompt"] = r;
      }
      if (ie(e).boosted) {
        n["HX-Boosted"] = "true";
      }
      return n;
    }
    function lr(t, e) {
      var r = re(e, "hx-params");
      if (r) {
        if (r === "none") {
          return {};
        } else if (r === "*") {
          return t;
        } else if (r.indexOf("not ") === 0) {
          ae(r.substr(4).split(","), function (e) {
            e = e.trim();
            delete t[e];
          });
          return t;
        } else {
          var n = {};
          ae(r.split(","), function (e) {
            e = e.trim();
            n[e] = t[e];
          });
          return n;
        }
      } else {
        return t;
      }
    }
    function ur(e) {
      return Q(e, "href") && Q(e, "href").indexOf("#") >= 0;
    }
    function fr(e, t) {
      var r = t ? t : re(e, "hx-swap");
      var n = {
        swapStyle: ie(e).boosted ? "innerHTML" : Y.config.defaultSwapStyle,
        swapDelay: Y.config.defaultSwapDelay,
        settleDelay: Y.config.defaultSettleDelay,
      };
      if (ie(e).boosted && !ur(e)) {
        n["show"] = "top";
      }
      if (r) {
        var i = k(r);
        if (i.length > 0) {
          for (var a = 0; a < i.length; a++) {
            var o = i[a];
            if (o.indexOf("swap:") === 0) {
              n["swapDelay"] = d(o.substr(5));
            } else if (o.indexOf("settle:") === 0) {
              n["settleDelay"] = d(o.substr(7));
            } else if (o.indexOf("transition:") === 0) {
              n["transition"] = o.substr(11) === "true";
            } else if (o.indexOf("ignoreTitle:") === 0) {
              n["ignoreTitle"] = o.substr(12) === "true";
            } else if (o.indexOf("scroll:") === 0) {
              var s = o.substr(7);
              var l = s.split(":");
              var u = l.pop();
              var f = l.length > 0 ? l.join(":") : null;
              n["scroll"] = u;
              n["scrollTarget"] = f;
            } else if (o.indexOf("show:") === 0) {
              var c = o.substr(5);
              var l = c.split(":");
              var h = l.pop();
              var f = l.length > 0 ? l.join(":") : null;
              n["show"] = h;
              n["showTarget"] = f;
            } else if (o.indexOf("focus-scroll:") === 0) {
              var v = o.substr("focus-scroll:".length);
              n["focusScroll"] = v == "true";
            } else if (a == 0) {
              n["swapStyle"] = o;
            } else {
              y("Unknown modifier in hx-swap: " + o);
            }
          }
        }
      }
      return n;
    }
    function cr(e) {
      return (
        re(e, "hx-encoding") === "multipart/form-data" ||
        (h(e, "form") && Q(e, "enctype") === "multipart/form-data")
      );
    }
    function hr(t, r, n) {
      var i = null;
      C(r, function (e) {
        if (i == null) {
          i = e.encodeParameters(t, n, r);
        }
      });
      if (i != null) {
        return i;
      } else {
        if (cr(r)) {
          return or(n);
        } else {
          return ar(n);
        }
      }
    }
    function T(e) {
      return {
        tasks: [],
        elts: [e],
      };
    }
    function vr(e, t) {
      var r = e[0];
      var n = e[e.length - 1];
      if (t.scroll) {
        var i = null;
        if (t.scrollTarget) {
          i = le(r, t.scrollTarget);
        }
        if (t.scroll === "top" && (r || i)) {
          i = i || r;
          i.scrollTop = 0;
        }
        if (t.scroll === "bottom" && (n || i)) {
          i = i || n;
          i.scrollTop = i.scrollHeight;
        }
      }
      if (t.show) {
        var i = null;
        if (t.showTarget) {
          var a = t.showTarget;
          if (t.showTarget === "window") {
            a = "body";
          }
          i = le(r, a);
        }
        if (t.show === "top" && (r || i)) {
          i = i || r;
          i.scrollIntoView({
            block: "start",
            behavior: Y.config.scrollBehavior,
          });
        }
        if (t.show === "bottom" && (n || i)) {
          i = i || n;
          i.scrollIntoView({
            block: "end",
            behavior: Y.config.scrollBehavior,
          });
        }
      }
    }
    function dr(e, t, r, n) {
      if (n == null) {
        n = {};
      }
      if (e == null) {
        return n;
      }
      var i = ee(e, t);
      if (i) {
        var a = i.trim();
        var o = r;
        if (a === "unset") {
          return null;
        }
        if (a.indexOf("javascript:") === 0) {
          a = a.substr(11);
          o = true;
        } else if (a.indexOf("js:") === 0) {
          a = a.substr(3);
          o = true;
        }
        if (a.indexOf("{") !== 0) {
          a = "{" + a + "}";
        }
        var s;
        if (o) {
          s = gr(
            e,
            function () {
              return Function("return (" + a + ")")();
            },
            {}
          );
        } else {
          s = S(a);
        }
        for (var l in s) {
          if (s.hasOwnProperty(l)) {
            if (n[l] == null) {
              n[l] = s[l];
            }
          }
        }
      }
      return dr(u(e), t, r, n);
    }
    function gr(e, t, r) {
      if (Y.config.allowEval) {
        return t();
      } else {
        ue(e, "htmx:evalDisallowedError");
        return r;
      }
    }
    function mr(e, t) {
      return dr(e, "hx-vars", true, t);
    }
    function pr(e, t) {
      return dr(e, "hx-vals", false, t);
    }
    function xr(e) {
      return se(mr(e), pr(e));
    }
    function yr(t, r, n) {
      if (n !== null) {
        try {
          t.setRequestHeader(r, n);
        } catch (e) {
          t.setRequestHeader(r, encodeURIComponent(n));
          t.setRequestHeader(r + "-URI-AutoEncoded", "true");
        }
      }
    }
    function br(t) {
      if (t.responseURL && typeof URL !== "undefined") {
        try {
          var e = new URL(t.responseURL);
          return e.pathname + e.search;
        } catch (e) {
          ue(te().body, "htmx:badResponseUrl", {
            url: t.responseURL,
          });
        }
      }
    }
    function R(e, t) {
      return e.getAllResponseHeaders().match(t);
    }
    function wr(e, t, r) {
      e = e.toLowerCase();
      if (r) {
        if (r instanceof Element || L(r, "String")) {
          return ce(e, t, null, null, {
            targetOverride: s(r),
            returnPromise: true,
          });
        } else {
          return ce(e, t, s(r.source), r.event, {
            handler: r.handler,
            headers: r.headers,
            values: r.values,
            targetOverride: s(r.target),
            swapOverride: r.swap,
            returnPromise: true,
          });
        }
      } else {
        return ce(e, t, null, null, {
          returnPromise: true,
        });
      }
    }
    function Sr(e) {
      var t = [];
      while (e) {
        t.push(e);
        e = e.parentElement;
      }
      return t;
    }
    function Er(e, t, r) {
      var n;
      var i;
      if (typeof URL === "function") {
        i = new URL(t, document.location.href);
        var a = document.location.origin;
        n = a === i.origin;
      } else {
        i = t;
        n = g(t, document.location.origin);
      }
      if (Y.config.selfRequestsOnly) {
        if (!n) {
          return false;
        }
      }
      return fe(
        e,
        "htmx:validateUrl",
        se(
          {
            url: i,
            sameHost: n,
          },
          r
        )
      );
    }
    function ce(e, t, n, r, i, M) {
      var a = null;
      var o = null;
      i = i != null ? i : {};
      if (i.returnPromise && typeof Promise !== "undefined") {
        var s = new Promise(function (e, t) {
          a = e;
          o = t;
        });
      }
      if (n == null) {
        n = te().body;
      }
      var D = i.handler || Tr;
      if (!oe(n)) {
        ne(a);
        return s;
      }
      var l = i.targetOverride || ge(n);
      if (l == null || l == he) {
        ue(n, "htmx:targetError", {
          target: ee(n, "hx-target"),
        });
        ne(o);
        return s;
      }
      var u = ie(n);
      var f = u.lastButtonClicked;
      if (f) {
        var c = Q(f, "formaction");
        if (c != null) {
          t = c;
        }
        var h = Q(f, "formmethod");
        if (h != null) {
          e = h;
        }
      }
      if (!M) {
        var X = function () {
          return ce(e, t, n, r, i, true);
        };
        var F = {
          target: l,
          elt: n,
          path: t,
          verb: e,
          triggeringEvent: r,
          etc: i,
          issueRequest: X,
        };
        if (fe(n, "htmx:confirm", F) === false) {
          ne(a);
          return s;
        }
      }
      var v = n;
      var d = re(n, "hx-sync");
      var g = null;
      var m = false;
      if (d) {
        var p = d.split(":");
        var x = p[0].trim();
        if (x === "this") {
          v = de(n, "hx-sync");
        } else {
          v = le(n, x);
        }
        d = (p[1] || "drop").trim();
        u = ie(v);
        if (d === "drop" && u.xhr && u.abortable !== true) {
          ne(a);
          return s;
        } else if (d === "abort") {
          if (u.xhr) {
            ne(a);
            return s;
          } else {
            m = true;
          }
        } else if (d === "replace") {
          fe(v, "htmx:abort");
        } else if (d.indexOf("queue") === 0) {
          var U = d.split(" ");
          g = (U[1] || "last").trim();
        }
      }
      if (u.xhr) {
        if (u.abortable) {
          fe(v, "htmx:abort");
        } else {
          if (g == null) {
            if (r) {
              var y = ie(r);
              if (y && y.triggerSpec && y.triggerSpec.queue) {
                g = y.triggerSpec.queue;
              }
            }
            if (g == null) {
              g = "last";
            }
          }
          if (u.queuedRequests == null) {
            u.queuedRequests = [];
          }
          if (g === "first" && u.queuedRequests.length === 0) {
            u.queuedRequests.push(function () {
              ce(e, t, n, r, i);
            });
          } else if (g === "all") {
            u.queuedRequests.push(function () {
              ce(e, t, n, r, i);
            });
          } else if (g === "last") {
            u.queuedRequests = [];
            u.queuedRequests.push(function () {
              ce(e, t, n, r, i);
            });
          }
          ne(a);
          return s;
        }
      }
      var b = new XMLHttpRequest();
      u.xhr = b;
      u.abortable = m;
      var w = function () {
        u.xhr = null;
        u.abortable = false;
        if (u.queuedRequests != null && u.queuedRequests.length > 0) {
          var e = u.queuedRequests.shift();
          e();
        }
      };
      var B = re(n, "hx-prompt");
      if (B) {
        var S = prompt(B);
        if (
          S === null ||
          !fe(n, "htmx:prompt", {
            prompt: S,
            target: l,
          })
        ) {
          ne(a);
          w();
          return s;
        }
      }
      var V = re(n, "hx-confirm");
      if (V) {
        if (!confirm(V)) {
          ne(a);
          w();
          return s;
        }
      }
      var E = sr(n, l, S);
      if (i.headers) {
        E = se(E, i.headers);
      }
      var j = nr(n, e);
      var C = j.errors;
      var T = j.values;
      if (i.values) {
        T = se(T, i.values);
      }
      var _ = xr(n);
      var z = se(T, _);
      var R = lr(z, n);
      if (e !== "get" && !cr(n)) {
        E["Content-Type"] = "application/x-www-form-urlencoded";
      }
      if (Y.config.getCacheBusterParam && e === "get") {
        R["org.htmx.cache-buster"] = Q(l, "id") || "true";
      }
      if (t == null || t === "") {
        t = te().location.href;
      }
      var O = dr(n, "hx-request");
      var W = ie(n).boosted;
      var q = Y.config.methodsThatUseUrlParams.indexOf(e) >= 0;
      var H = {
        boosted: W,
        useUrlParams: q,
        parameters: R,
        unfilteredParameters: z,
        headers: E,
        target: l,
        verb: e,
        errors: C,
        withCredentials:
          i.credentials || O.credentials || Y.config.withCredentials,
        timeout: i.timeout || O.timeout || Y.config.timeout,
        path: t,
        triggeringEvent: r,
      };
      if (!fe(n, "htmx:configRequest", H)) {
        ne(a);
        w();
        return s;
      }
      t = H.path;
      e = H.verb;
      E = H.headers;
      R = H.parameters;
      C = H.errors;
      q = H.useUrlParams;
      if (C && C.length > 0) {
        fe(n, "htmx:validation:halted", H);
        ne(a);
        w();
        return s;
      }
      var $ = t.split("#");
      var G = $[0];
      var L = $[1];
      var A = t;
      if (q) {
        A = G;
        var J = Object.keys(R).length !== 0;
        if (J) {
          if (A.indexOf("?") < 0) {
            A += "?";
          } else {
            A += "&";
          }
          A += ar(R);
          if (L) {
            A += "#" + L;
          }
        }
      }
      if (!Er(n, A, H)) {
        ue(n, "htmx:invalidPath", H);
        ne(o);
        return s;
      }
      b.open(e.toUpperCase(), A, true);
      b.overrideMimeType("text/html");
      b.withCredentials = H.withCredentials;
      b.timeout = H.timeout;
      if (O.noHeaders) {
      } else {
        for (var N in E) {
          if (E.hasOwnProperty(N)) {
            var Z = E[N];
            yr(b, N, Z);
          }
        }
      }
      var I = {
        xhr: b,
        target: l,
        requestConfig: H,
        etc: i,
        boosted: W,
        pathInfo: {
          requestPath: t,
          finalRequestPath: A,
          anchor: L,
        },
      };
      b.onload = function () {
        try {
          var e = Sr(n);
          I.pathInfo.responsePath = br(b);
          D(n, I);
          Kt(P, k);
          fe(n, "htmx:afterRequest", I);
          fe(n, "htmx:afterOnLoad", I);
          if (!oe(n)) {
            var t = null;
            while (e.length > 0 && t == null) {
              var r = e.shift();
              if (oe(r)) {
                t = r;
              }
            }
            if (t) {
              fe(t, "htmx:afterRequest", I);
              fe(t, "htmx:afterOnLoad", I);
            }
          }
          ne(a);
          w();
        } catch (e) {
          ue(
            n,
            "htmx:onLoadError",
            se(
              {
                error: e,
              },
              I
            )
          );
          throw e;
        }
      };
      b.onerror = function () {
        Kt(P, k);
        ue(n, "htmx:afterRequest", I);
        ue(n, "htmx:sendError", I);
        ne(o);
        w();
      };
      b.onabort = function () {
        Kt(P, k);
        ue(n, "htmx:afterRequest", I);
        ue(n, "htmx:sendAbort", I);
        ne(o);
        w();
      };
      b.ontimeout = function () {
        Kt(P, k);
        ue(n, "htmx:afterRequest", I);
        ue(n, "htmx:timeout", I);
        ne(o);
        w();
      };
      if (!fe(n, "htmx:beforeRequest", I)) {
        ne(a);
        w();
        return s;
      }
      var P = Jt(n);
      var k = Zt(n);
      ae(["loadstart", "loadend", "progress", "abort"], function (t) {
        ae([b, b.upload], function (e) {
          e.addEventListener(t, function (e) {
            fe(n, "htmx:xhr:" + t, {
              lengthComputable: e.lengthComputable,
              loaded: e.loaded,
              total: e.total,
            });
          });
        });
      });
      fe(n, "htmx:beforeSend", I);
      var K = q ? null : hr(b, n, R);
      b.send(K);
      return s;
    }
    function Cr(e, t) {
      var r = t.xhr;
      var n = null;
      var i = null;
      if (R(r, /HX-Push:/i)) {
        n = r.getResponseHeader("HX-Push");
        i = "push";
      } else if (R(r, /HX-Push-Url:/i)) {
        n = r.getResponseHeader("HX-Push-Url");
        i = "push";
      } else if (R(r, /HX-Replace-Url:/i)) {
        n = r.getResponseHeader("HX-Replace-Url");
        i = "replace";
      }
      if (n) {
        if (n === "false") {
          return {};
        } else {
          return {
            type: i,
            path: n,
          };
        }
      }
      var a = t.pathInfo.finalRequestPath;
      var o = t.pathInfo.responsePath;
      var s = re(e, "hx-push-url");
      var l = re(e, "hx-replace-url");
      var u = ie(e).boosted;
      var f = null;
      var c = null;
      if (s) {
        f = "push";
        c = s;
      } else if (l) {
        f = "replace";
        c = l;
      } else if (u) {
        f = "push";
        c = o || a;
      }
      if (c) {
        if (c === "false") {
          return {};
        }
        if (c === "true") {
          c = o || a;
        }
        if (t.pathInfo.anchor && c.indexOf("#") === -1) {
          c = c + "#" + t.pathInfo.anchor;
        }
        return {
          type: f,
          path: c,
        };
      } else {
        return {};
      }
    }
    function Tr(l, u) {
      var f = u.xhr;
      var c = u.target;
      var e = u.etc;
      var t = u.requestConfig;
      if (!fe(l, "htmx:beforeOnLoad", u)) return;
      if (R(f, /HX-Trigger:/i)) {
        Ue(f, "HX-Trigger", l);
      }
      if (R(f, /HX-Location:/i)) {
        jt();
        var r = f.getResponseHeader("HX-Location");
        var h;
        if (r.indexOf("{") === 0) {
          h = S(r);
          r = h["path"];
          delete h["path"];
        }
        wr("GET", r, h).then(function () {
          _t(r);
        });
        return;
      }
      var n =
        R(f, /HX-Refresh:/i) && "true" === f.getResponseHeader("HX-Refresh");
      if (R(f, /HX-Redirect:/i)) {
        location.href = f.getResponseHeader("HX-Redirect");
        n && location.reload();
        return;
      }
      if (n) {
        location.reload();
        return;
      }
      if (R(f, /HX-Retarget:/i)) {
        u.target = te().querySelector(f.getResponseHeader("HX-Retarget"));
      }
      var v = Cr(l, u);
      var i = f.status >= 200 && f.status < 400 && f.status !== 204;
      var d = f.response;
      var a = f.status >= 400;
      var g = Y.config.ignoreTitle;
      var o = se(
        {
          shouldSwap: i,
          serverResponse: d,
          isError: a,
          ignoreTitle: g,
        },
        u
      );
      if (!fe(c, "htmx:beforeSwap", o)) return;
      c = o.target;
      d = o.serverResponse;
      a = o.isError;
      g = o.ignoreTitle;
      u.target = c;
      u.failed = a;
      u.successful = !a;
      if (o.shouldSwap) {
        if (f.status === 286) {
          Ke(l);
        }
        C(l, function (e) {
          d = e.transformResponse(d, f, l);
        });
        if (v.type) {
          jt();
        }
        var s = e.swapOverride;
        if (R(f, /HX-Reswap:/i)) {
          s = f.getResponseHeader("HX-Reswap");
        }
        var h = fr(l, s);
        if (h.hasOwnProperty("ignoreTitle")) {
          g = h.ignoreTitle;
        }
        c.classList.add(Y.config.swappingClass);
        var m = null;
        var p = null;
        var x = function () {
          try {
            var e = document.activeElement;
            var t = {};
            try {
              t = {
                elt: e,
                start: e ? e.selectionStart : null,
                end: e ? e.selectionEnd : null,
              };
            } catch (e) {}
            var r;
            if (R(f, /HX-Reselect:/i)) {
              r = f.getResponseHeader("HX-Reselect");
            }
            var n = T(c);
            Fe(h.swapStyle, c, l, d, n, r);
            if (t.elt && !oe(t.elt) && Q(t.elt, "id")) {
              var i = document.getElementById(Q(t.elt, "id"));
              var a = {
                preventScroll:
                  h.focusScroll !== undefined
                    ? !h.focusScroll
                    : !Y.config.defaultFocusScroll,
              };
              if (i) {
                if (t.start && i.setSelectionRange) {
                  try {
                    i.setSelectionRange(t.start, t.end);
                  } catch (e) {}
                }
                i.focus(a);
              }
            }
            c.classList.remove(Y.config.swappingClass);
            ae(n.elts, function (e) {
              if (e.classList) {
                e.classList.add(Y.config.settlingClass);
              }
              fe(e, "htmx:afterSwap", u);
            });
            if (R(f, /HX-Trigger-After-Swap:/i)) {
              var o = l;
              if (!oe(l)) {
                o = te().body;
              }
              Ue(f, "HX-Trigger-After-Swap", o);
            }
            var s = function () {
              ae(n.tasks, function (e) {
                e.call();
              });
              ae(n.elts, function (e) {
                if (e.classList) {
                  e.classList.remove(Y.config.settlingClass);
                }
                fe(e, "htmx:afterSettle", u);
              });
              if (v.type) {
                if (v.type === "push") {
                  _t(v.path);
                  fe(te().body, "htmx:pushedIntoHistory", {
                    path: v.path,
                  });
                } else {
                  zt(v.path);
                  fe(te().body, "htmx:replacedInHistory", {
                    path: v.path,
                  });
                }
              }
              if (u.pathInfo.anchor) {
                var e = E("#" + u.pathInfo.anchor);
                if (e) {
                  e.scrollIntoView({
                    block: "start",
                    behavior: "auto",
                  });
                }
              }
              if (n.title && !g) {
                var t = E("title");
                if (t) {
                  t.innerHTML = n.title;
                } else {
                  window.document.title = n.title;
                }
              }
              vr(n.elts, h);
              if (R(f, /HX-Trigger-After-Settle:/i)) {
                var r = l;
                if (!oe(l)) {
                  r = te().body;
                }
                Ue(f, "HX-Trigger-After-Settle", r);
              }
              ne(m);
            };
            if (h.settleDelay > 0) {
              setTimeout(s, h.settleDelay);
            } else {
              s();
            }
          } catch (e) {
            ue(l, "htmx:swapError", u);
            ne(p);
            throw e;
          }
        };
        var y = Y.config.globalViewTransitions;
        if (h.hasOwnProperty("transition")) {
          y = h.transition;
        }
        if (
          y &&
          fe(l, "htmx:beforeTransition", u) &&
          typeof Promise !== "undefined" &&
          document.startViewTransition
        ) {
          var b = new Promise(function (e, t) {
            m = e;
            p = t;
          });
          var w = x;
          x = function () {
            document.startViewTransition(function () {
              w();
              return b;
            });
          };
        }
        if (h.swapDelay > 0) {
          setTimeout(x, h.swapDelay);
        } else {
          x();
        }
      }
      if (a) {
        ue(
          l,
          "htmx:responseError",
          se(
            {
              error:
                "Response Status Error Code " +
                f.status +
                " from " +
                u.pathInfo.requestPath,
            },
            u
          )
        );
      }
    }
    var Rr = {};
    function Or() {
      return {
        init: function (e) {
          return null;
        },
        onEvent: function (e, t) {
          return true;
        },
        transformResponse: function (e, t, r) {
          return e;
        },
        isInlineSwap: function (e) {
          return false;
        },
        handleSwap: function (e, t, r, n) {
          return false;
        },
        encodeParameters: function (e, t, r) {
          return null;
        },
      };
    }
    function qr(e, t) {
      if (t.init) {
        t.init(r);
      }
      Rr[e] = se(Or(), t);
    }
    function Hr(e) {
      delete Rr[e];
    }
    function Lr(e, r, n) {
      if (e == undefined) {
        return r;
      }
      if (r == undefined) {
        r = [];
      }
      if (n == undefined) {
        n = [];
      }
      var t = ee(e, "hx-ext");
      if (t) {
        ae(t.split(","), function (e) {
          e = e.replace(/ /g, "");
          if (e.slice(0, 7) == "ignore:") {
            n.push(e.slice(7));
            return;
          }
          if (n.indexOf(e) < 0) {
            var t = Rr[e];
            if (t && r.indexOf(t) < 0) {
              r.push(t);
            }
          }
        });
      }
      return Lr(u(e), r, n);
    }
    var Ar = false;
    te().addEventListener("DOMContentLoaded", function () {
      Ar = true;
    });
    function Nr(e) {
      if (Ar || te().readyState === "complete") {
        e();
      } else {
        te().addEventListener("DOMContentLoaded", e);
      }
    }
    function Ir() {
      if (Y.config.includeIndicatorStyles !== false) {
        te().head.insertAdjacentHTML(
          "beforeend",
          "<style>                      ." +
            Y.config.indicatorClass +
            "{opacity:0;transition: opacity 200ms ease-in;}                      ." +
            Y.config.requestClass +
            " ." +
            Y.config.indicatorClass +
            "{opacity:1}                      ." +
            Y.config.requestClass +
            "." +
            Y.config.indicatorClass +
            "{opacity:1}                    </style>"
        );
      }
    }
    function Pr() {
      var e = te().querySelector('meta[name="htmx-config"]');
      if (e) {
        return S(e.content);
      } else {
        return null;
      }
    }
    function kr() {
      var e = Pr();
      if (e) {
        Y.config = se(Y.config, e);
      }
    }
    Nr(function () {
      kr();
      Ir();
      var e = te().body;
      Pt(e);
      var t = te().querySelectorAll(
        "[hx-trigger='restored'],[data-hx-trigger='restored']"
      );
      e.addEventListener("htmx:abort", function (e) {
        var t = e.target;
        var r = ie(t);
        if (r && r.xhr) {
          r.xhr.abort();
        }
      });
      var r = window.onpopstate;
      window.onpopstate = function (e) {
        if (e.state && e.state.htmx) {
          Gt();
          ae(t, function (e) {
            fe(e, "htmx:restored", {
              document: te(),
              triggerEvent: fe,
            });
          });
        } else {
          if (r) {
            r(e);
          }
        }
      };
      setTimeout(function () {
        fe(e, "htmx:load", {});
        e = null;
      }, 0);
    });
    return Y;
  })();
});
