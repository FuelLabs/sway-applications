var ie =
  typeof globalThis < 'u'
    ? globalThis
    : typeof window < 'u'
    ? window
    : typeof global < 'u'
    ? global
    : typeof self < 'u'
    ? self
    : {};
function Kf(r) {
  return r && r.__esModule && Object.prototype.hasOwnProperty.call(r, 'default') ? r.default : r;
}
function Wa(r) {
  var e = r.default;
  if (typeof e == 'function') {
    var t = function n() {
      if (this instanceof n) {
        var i = [null];
        i.push.apply(i, arguments);
        var a = Function.bind.apply(e, i);
        return new a();
      }
      return e.apply(this, arguments);
    };
    t.prototype = e.prototype;
  } else t = {};
  return (
    Object.defineProperty(t, '__esModule', { value: !0 }),
    Object.keys(r).forEach(function (n) {
      var i = Object.getOwnPropertyDescriptor(r, n);
      Object.defineProperty(
        t,
        n,
        i.get
          ? i
          : {
              enumerable: !0,
              get: function () {
                return r[n];
              },
            }
      );
    }),
    t
  );
}
const Xf = 'logger/5.7.0';
let Ss = !1,
  As = !1;
const vi = { debug: 1, default: 2, info: 2, warning: 3, error: 4, off: 5 };
let Es = vi.default,
  da = null;
function Zf() {
  try {
    const r = [];
    if (
      (['NFD', 'NFC', 'NFKD', 'NFKC'].forEach((e) => {
        try {
          if ('test'.normalize(e) !== 'test') throw new Error('bad normalize');
        } catch {
          r.push(e);
        }
      }),
      r.length)
    )
      throw new Error('missing ' + r.join(', '));
    if (String.fromCharCode(233).normalize('NFD') !== String.fromCharCode(101, 769))
      throw new Error('broken implementation');
  } catch (r) {
    return r.message;
  }
  return null;
}
const Is = Zf();
var Pa;
(function (r) {
  (r.DEBUG = 'DEBUG'),
    (r.INFO = 'INFO'),
    (r.WARNING = 'WARNING'),
    (r.ERROR = 'ERROR'),
    (r.OFF = 'OFF');
})(Pa || (Pa = {}));
var Qt;
(function (r) {
  (r.UNKNOWN_ERROR = 'UNKNOWN_ERROR'),
    (r.NOT_IMPLEMENTED = 'NOT_IMPLEMENTED'),
    (r.UNSUPPORTED_OPERATION = 'UNSUPPORTED_OPERATION'),
    (r.NETWORK_ERROR = 'NETWORK_ERROR'),
    (r.SERVER_ERROR = 'SERVER_ERROR'),
    (r.TIMEOUT = 'TIMEOUT'),
    (r.BUFFER_OVERRUN = 'BUFFER_OVERRUN'),
    (r.NUMERIC_FAULT = 'NUMERIC_FAULT'),
    (r.MISSING_NEW = 'MISSING_NEW'),
    (r.INVALID_ARGUMENT = 'INVALID_ARGUMENT'),
    (r.MISSING_ARGUMENT = 'MISSING_ARGUMENT'),
    (r.UNEXPECTED_ARGUMENT = 'UNEXPECTED_ARGUMENT'),
    (r.CALL_EXCEPTION = 'CALL_EXCEPTION'),
    (r.INSUFFICIENT_FUNDS = 'INSUFFICIENT_FUNDS'),
    (r.NONCE_EXPIRED = 'NONCE_EXPIRED'),
    (r.REPLACEMENT_UNDERPRICED = 'REPLACEMENT_UNDERPRICED'),
    (r.UNPREDICTABLE_GAS_LIMIT = 'UNPREDICTABLE_GAS_LIMIT'),
    (r.TRANSACTION_REPLACED = 'TRANSACTION_REPLACED'),
    (r.ACTION_REJECTED = 'ACTION_REJECTED');
})(Qt || (Qt = {}));
const Rs = '0123456789abcdef';
class Te {
  constructor(e) {
    Object.defineProperty(this, 'version', { enumerable: !0, value: e, writable: !1 });
  }
  _log(e, t) {
    const n = e.toLowerCase();
    vi[n] == null && this.throwArgumentError('invalid log level name', 'logLevel', e),
      !(Es > vi[n]) && console.log.apply(console, t);
  }
  debug(...e) {
    this._log(Te.levels.DEBUG, e);
  }
  info(...e) {
    this._log(Te.levels.INFO, e);
  }
  warn(...e) {
    this._log(Te.levels.WARNING, e);
  }
  makeError(e, t, n) {
    if (As) return this.makeError('censored error', t, {});
    t || (t = Te.errors.UNKNOWN_ERROR), n || (n = {});
    const i = [];
    Object.keys(n).forEach((w) => {
      const y = n[w];
      try {
        if (y instanceof Uint8Array) {
          let M = '';
          for (let S = 0; S < y.length; S++) (M += Rs[y[S] >> 4]), (M += Rs[y[S] & 15]);
          i.push(w + '=Uint8Array(0x' + M + ')');
        } else i.push(w + '=' + JSON.stringify(y));
      } catch {
        i.push(w + '=' + JSON.stringify(n[w].toString()));
      }
    }),
      i.push(`code=${t}`),
      i.push(`version=${this.version}`);
    const a = e;
    let c = '';
    switch (t) {
      case Qt.NUMERIC_FAULT: {
        c = 'NUMERIC_FAULT';
        const w = e;
        switch (w) {
          case 'overflow':
          case 'underflow':
          case 'division-by-zero':
            c += '-' + w;
            break;
          case 'negative-power':
          case 'negative-width':
            c += '-unsupported';
            break;
          case 'unbound-bitwise-result':
            c += '-unbound-result';
            break;
        }
        break;
      }
      case Qt.CALL_EXCEPTION:
      case Qt.INSUFFICIENT_FUNDS:
      case Qt.MISSING_NEW:
      case Qt.NONCE_EXPIRED:
      case Qt.REPLACEMENT_UNDERPRICED:
      case Qt.TRANSACTION_REPLACED:
      case Qt.UNPREDICTABLE_GAS_LIMIT:
        c = t;
        break;
    }
    c && (e += ' [ See: https://links.ethers.org/v5-errors-' + c + ' ]'),
      i.length && (e += ' (' + i.join(', ') + ')');
    const v = new Error(e);
    return (
      (v.reason = a),
      (v.code = t),
      Object.keys(n).forEach(function (w) {
        v[w] = n[w];
      }),
      v
    );
  }
  throwError(e, t, n) {
    throw this.makeError(e, t, n);
  }
  throwArgumentError(e, t, n) {
    return this.throwError(e, Te.errors.INVALID_ARGUMENT, { argument: t, value: n });
  }
  assert(e, t, n, i) {
    e || this.throwError(t, n, i);
  }
  assertArgument(e, t, n, i) {
    e || this.throwArgumentError(t, n, i);
  }
  checkNormalize(e) {
    Is &&
      this.throwError(
        'platform missing String.prototype.normalize',
        Te.errors.UNSUPPORTED_OPERATION,
        { operation: 'String.prototype.normalize', form: Is }
      );
  }
  checkSafeUint53(e, t) {
    typeof e == 'number' &&
      (t == null && (t = 'value not safe'),
      (e < 0 || e >= 9007199254740991) &&
        this.throwError(t, Te.errors.NUMERIC_FAULT, {
          operation: 'checkSafeInteger',
          fault: 'out-of-safe-range',
          value: e,
        }),
      e % 1 &&
        this.throwError(t, Te.errors.NUMERIC_FAULT, {
          operation: 'checkSafeInteger',
          fault: 'non-integer',
          value: e,
        }));
  }
  checkArgumentCount(e, t, n) {
    n ? (n = ': ' + n) : (n = ''),
      e < t &&
        this.throwError('missing argument' + n, Te.errors.MISSING_ARGUMENT, {
          count: e,
          expectedCount: t,
        }),
      e > t &&
        this.throwError('too many arguments' + n, Te.errors.UNEXPECTED_ARGUMENT, {
          count: e,
          expectedCount: t,
        });
  }
  checkNew(e, t) {
    (e === Object || e == null) &&
      this.throwError('missing new', Te.errors.MISSING_NEW, { name: t.name });
  }
  checkAbstract(e, t) {
    e === t
      ? this.throwError(
          'cannot instantiate abstract class ' +
            JSON.stringify(t.name) +
            ' directly; use a sub-class',
          Te.errors.UNSUPPORTED_OPERATION,
          { name: e.name, operation: 'new' }
        )
      : (e === Object || e == null) &&
        this.throwError('missing new', Te.errors.MISSING_NEW, { name: t.name });
  }
  static globalLogger() {
    return da || (da = new Te(Xf)), da;
  }
  static setCensorship(e, t) {
    if (
      (!e &&
        t &&
        this.globalLogger().throwError(
          'cannot permanently disable censorship',
          Te.errors.UNSUPPORTED_OPERATION,
          { operation: 'setCensorship' }
        ),
      Ss)
    ) {
      if (!e) return;
      this.globalLogger().throwError(
        'error censorship permanent',
        Te.errors.UNSUPPORTED_OPERATION,
        { operation: 'setCensorship' }
      );
    }
    (As = !!e), (Ss = !!t);
  }
  static setLogLevel(e) {
    const t = vi[e.toLowerCase()];
    if (t == null) {
      Te.globalLogger().warn('invalid log level - ' + e);
      return;
    }
    Es = t;
  }
  static from(e) {
    return new Te(e);
  }
}
Te.errors = Qt;
Te.levels = Pa;
const Yf = 'bytes/5.7.0',
  Vr = new Te(Yf);
function Bo(r) {
  return !!r.toHexString;
}
function Fn(r) {
  return (
    r.slice ||
      (r.slice = function () {
        const e = Array.prototype.slice.call(arguments);
        return Fn(new Uint8Array(Array.prototype.slice.apply(r, e)));
      }),
    r
  );
}
function Ns(r) {
  return typeof r == 'number' && r == r && r % 1 === 0;
}
function Fo(r) {
  if (r == null) return !1;
  if (r.constructor === Uint8Array) return !0;
  if (typeof r == 'string' || !Ns(r.length) || r.length < 0) return !1;
  for (let e = 0; e < r.length; e++) {
    const t = r[e];
    if (!Ns(t) || t < 0 || t >= 256) return !1;
  }
  return !0;
}
function V(r, e) {
  if ((e || (e = {}), typeof r == 'number')) {
    Vr.checkSafeUint53(r, 'invalid arrayify value');
    const t = [];
    for (; r; ) t.unshift(r & 255), (r = parseInt(String(r / 256)));
    return t.length === 0 && t.push(0), Fn(new Uint8Array(t));
  }
  if (
    (e.allowMissingPrefix && typeof r == 'string' && r.substring(0, 2) !== '0x' && (r = '0x' + r),
    Bo(r) && (r = r.toHexString()),
    Ga(r))
  ) {
    let t = r.substring(2);
    t.length % 2 &&
      (e.hexPad === 'left'
        ? (t = '0' + t)
        : e.hexPad === 'right'
        ? (t += '0')
        : Vr.throwArgumentError('hex data is odd-length', 'value', r));
    const n = [];
    for (let i = 0; i < t.length; i += 2) n.push(parseInt(t.substring(i, i + 2), 16));
    return Fn(new Uint8Array(n));
  }
  return Fo(r)
    ? Fn(new Uint8Array(r))
    : Vr.throwArgumentError('invalid arrayify value', 'value', r);
}
function se(r) {
  const e = r.map((i) => V(i)),
    t = e.reduce((i, a) => i + a.length, 0),
    n = new Uint8Array(t);
  return e.reduce((i, a) => (n.set(a, i), i + a.length), 0), Fn(n);
}
function Ga(r, e) {
  return !(typeof r != 'string' || !r.match(/^0x[0-9A-Fa-f]*$/) || (e && r.length !== 2 + 2 * e));
}
const ha = '0123456789abcdef';
function K(r, e) {
  if ((e || (e = {}), typeof r == 'number')) {
    Vr.checkSafeUint53(r, 'invalid hexlify value');
    let t = '';
    for (; r; ) (t = ha[r & 15] + t), (r = Math.floor(r / 16));
    return t.length ? (t.length % 2 && (t = '0' + t), '0x' + t) : '0x00';
  }
  if (typeof r == 'bigint') return (r = r.toString(16)), r.length % 2 ? '0x0' + r : '0x' + r;
  if (
    (e.allowMissingPrefix && typeof r == 'string' && r.substring(0, 2) !== '0x' && (r = '0x' + r),
    Bo(r))
  )
    return r.toHexString();
  if (Ga(r))
    return (
      r.length % 2 &&
        (e.hexPad === 'left'
          ? (r = '0x0' + r.substring(2))
          : e.hexPad === 'right'
          ? (r += '0')
          : Vr.throwArgumentError('hex data is odd-length', 'value', r)),
      r.toLowerCase()
    );
  if (Fo(r)) {
    let t = '0x';
    for (let n = 0; n < r.length; n++) {
      let i = r[n];
      t += ha[(i & 240) >> 4] + ha[i & 15];
    }
    return t;
  }
  return Vr.throwArgumentError('invalid hexlify value', 'value', r);
}
function Ka(r, e, t) {
  return (
    typeof r != 'string'
      ? (r = K(r))
      : (!Ga(r) || r.length % 2) && Vr.throwArgumentError('invalid hexData', 'value', r),
    (e = 2 + 2 * e),
    t != null ? '0x' + r.substring(e, 2 + 2 * t) : '0x' + r.substring(e)
  );
}
function la(r) {
  if (r !== void 0) {
    let e = r.toString();
    if (e !== 'true') return e;
  }
  return '0.0.0';
}
function Qf() {
  return { FUELS: la('0.27.0'), FUEL_CORE: la('0.15.1'), FORC: la('0.32.2') };
}
var Mn = Qf(),
  Uo = { exports: {} };
const ec = {},
  tc = Object.freeze(
    Object.defineProperty({ __proto__: null, default: ec }, Symbol.toStringTag, { value: 'Module' })
  ),
  Xa = Wa(tc);
(function (r) {
  (function (e, t) {
    function n(u, s) {
      if (!u) throw new Error(s || 'Assertion failed');
    }
    function i(u, s) {
      u.super_ = s;
      var o = function () {};
      (o.prototype = s.prototype), (u.prototype = new o()), (u.prototype.constructor = u);
    }
    function a(u, s, o) {
      if (a.isBN(u)) return u;
      (this.negative = 0),
        (this.words = null),
        (this.length = 0),
        (this.red = null),
        u !== null &&
          ((s === 'le' || s === 'be') && ((o = s), (s = 10)),
          this._init(u || 0, s || 10, o || 'be'));
    }
    typeof e == 'object' ? (e.exports = a) : (t.BN = a), (a.BN = a), (a.wordSize = 26);
    var c;
    try {
      typeof window < 'u' && typeof window.Buffer < 'u' ? (c = window.Buffer) : (c = Xa.Buffer);
    } catch {}
    (a.isBN = function (s) {
      return s instanceof a
        ? !0
        : s !== null &&
            typeof s == 'object' &&
            s.constructor.wordSize === a.wordSize &&
            Array.isArray(s.words);
    }),
      (a.max = function (s, o) {
        return s.cmp(o) > 0 ? s : o;
      }),
      (a.min = function (s, o) {
        return s.cmp(o) < 0 ? s : o;
      }),
      (a.prototype._init = function (s, o, l) {
        if (typeof s == 'number') return this._initNumber(s, o, l);
        if (typeof s == 'object') return this._initArray(s, o, l);
        o === 'hex' && (o = 16),
          n(o === (o | 0) && o >= 2 && o <= 36),
          (s = s.toString().replace(/\s+/g, ''));
        var g = 0;
        s[0] === '-' && (g++, (this.negative = 1)),
          g < s.length &&
            (o === 16
              ? this._parseHex(s, g, l)
              : (this._parseBase(s, o, g), l === 'le' && this._initArray(this.toArray(), o, l)));
      }),
      (a.prototype._initNumber = function (s, o, l) {
        s < 0 && ((this.negative = 1), (s = -s)),
          s < 67108864
            ? ((this.words = [s & 67108863]), (this.length = 1))
            : s < 4503599627370496
            ? ((this.words = [s & 67108863, (s / 67108864) & 67108863]), (this.length = 2))
            : (n(s < 9007199254740992),
              (this.words = [s & 67108863, (s / 67108864) & 67108863, 1]),
              (this.length = 3)),
          l === 'le' && this._initArray(this.toArray(), o, l);
      }),
      (a.prototype._initArray = function (s, o, l) {
        if ((n(typeof s.length == 'number'), s.length <= 0))
          return (this.words = [0]), (this.length = 1), this;
        (this.length = Math.ceil(s.length / 3)), (this.words = new Array(this.length));
        for (var g = 0; g < this.length; g++) this.words[g] = 0;
        var m,
          b,
          f = 0;
        if (l === 'be')
          for (g = s.length - 1, m = 0; g >= 0; g -= 3)
            (b = s[g] | (s[g - 1] << 8) | (s[g - 2] << 16)),
              (this.words[m] |= (b << f) & 67108863),
              (this.words[m + 1] = (b >>> (26 - f)) & 67108863),
              (f += 24),
              f >= 26 && ((f -= 26), m++);
        else if (l === 'le')
          for (g = 0, m = 0; g < s.length; g += 3)
            (b = s[g] | (s[g + 1] << 8) | (s[g + 2] << 16)),
              (this.words[m] |= (b << f) & 67108863),
              (this.words[m + 1] = (b >>> (26 - f)) & 67108863),
              (f += 24),
              f >= 26 && ((f -= 26), m++);
        return this._strip();
      });
    function v(u, s) {
      var o = u.charCodeAt(s);
      if (o >= 48 && o <= 57) return o - 48;
      if (o >= 65 && o <= 70) return o - 55;
      if (o >= 97 && o <= 102) return o - 87;
      n(!1, 'Invalid character in ' + u);
    }
    function w(u, s, o) {
      var l = v(u, o);
      return o - 1 >= s && (l |= v(u, o - 1) << 4), l;
    }
    a.prototype._parseHex = function (s, o, l) {
      (this.length = Math.ceil((s.length - o) / 6)), (this.words = new Array(this.length));
      for (var g = 0; g < this.length; g++) this.words[g] = 0;
      var m = 0,
        b = 0,
        f;
      if (l === 'be')
        for (g = s.length - 1; g >= o; g -= 2)
          (f = w(s, o, g) << m),
            (this.words[b] |= f & 67108863),
            m >= 18 ? ((m -= 18), (b += 1), (this.words[b] |= f >>> 26)) : (m += 8);
      else {
        var p = s.length - o;
        for (g = p % 2 === 0 ? o + 1 : o; g < s.length; g += 2)
          (f = w(s, o, g) << m),
            (this.words[b] |= f & 67108863),
            m >= 18 ? ((m -= 18), (b += 1), (this.words[b] |= f >>> 26)) : (m += 8);
      }
      this._strip();
    };
    function y(u, s, o, l) {
      for (var g = 0, m = 0, b = Math.min(u.length, o), f = s; f < b; f++) {
        var p = u.charCodeAt(f) - 48;
        (g *= l),
          p >= 49 ? (m = p - 49 + 10) : p >= 17 ? (m = p - 17 + 10) : (m = p),
          n(p >= 0 && m < l, 'Invalid character'),
          (g += m);
      }
      return g;
    }
    (a.prototype._parseBase = function (s, o, l) {
      (this.words = [0]), (this.length = 1);
      for (var g = 0, m = 1; m <= 67108863; m *= o) g++;
      g--, (m = (m / o) | 0);
      for (var b = s.length - l, f = b % g, p = Math.min(b, b - f) + l, h = 0, x = l; x < p; x += g)
        (h = y(s, x, x + g, o)),
          this.imuln(m),
          this.words[0] + h < 67108864 ? (this.words[0] += h) : this._iaddn(h);
      if (f !== 0) {
        var O = 1;
        for (h = y(s, x, s.length, o), x = 0; x < f; x++) O *= o;
        this.imuln(O), this.words[0] + h < 67108864 ? (this.words[0] += h) : this._iaddn(h);
      }
      this._strip();
    }),
      (a.prototype.copy = function (s) {
        s.words = new Array(this.length);
        for (var o = 0; o < this.length; o++) s.words[o] = this.words[o];
        (s.length = this.length), (s.negative = this.negative), (s.red = this.red);
      });
    function M(u, s) {
      (u.words = s.words), (u.length = s.length), (u.negative = s.negative), (u.red = s.red);
    }
    if (
      ((a.prototype._move = function (s) {
        M(s, this);
      }),
      (a.prototype.clone = function () {
        var s = new a(null);
        return this.copy(s), s;
      }),
      (a.prototype._expand = function (s) {
        for (; this.length < s; ) this.words[this.length++] = 0;
        return this;
      }),
      (a.prototype._strip = function () {
        for (; this.length > 1 && this.words[this.length - 1] === 0; ) this.length--;
        return this._normSign();
      }),
      (a.prototype._normSign = function () {
        return this.length === 1 && this.words[0] === 0 && (this.negative = 0), this;
      }),
      typeof Symbol < 'u' && typeof Symbol.for == 'function')
    )
      try {
        a.prototype[Symbol.for('nodejs.util.inspect.custom')] = S;
      } catch {
        a.prototype.inspect = S;
      }
    else a.prototype.inspect = S;
    function S() {
      return (this.red ? '<BN-R: ' : '<BN: ') + this.toString(16) + '>';
    }
    var I = [
        '',
        '0',
        '00',
        '000',
        '0000',
        '00000',
        '000000',
        '0000000',
        '00000000',
        '000000000',
        '0000000000',
        '00000000000',
        '000000000000',
        '0000000000000',
        '00000000000000',
        '000000000000000',
        '0000000000000000',
        '00000000000000000',
        '000000000000000000',
        '0000000000000000000',
        '00000000000000000000',
        '000000000000000000000',
        '0000000000000000000000',
        '00000000000000000000000',
        '000000000000000000000000',
        '0000000000000000000000000',
      ],
      E = [
        0, 0, 25, 16, 12, 11, 10, 9, 8, 8, 7, 7, 7, 7, 6, 6, 6, 6, 6, 6, 6, 5, 5, 5, 5, 5, 5, 5, 5,
        5, 5, 5, 5, 5, 5, 5, 5,
      ],
      R = [
        0, 0, 33554432, 43046721, 16777216, 48828125, 60466176, 40353607, 16777216, 43046721, 1e7,
        19487171, 35831808, 62748517, 7529536, 11390625, 16777216, 24137569, 34012224, 47045881,
        64e6, 4084101, 5153632, 6436343, 7962624, 9765625, 11881376, 14348907, 17210368, 20511149,
        243e5, 28629151, 33554432, 39135393, 45435424, 52521875, 60466176,
      ];
    (a.prototype.toString = function (s, o) {
      (s = s || 10), (o = o | 0 || 1);
      var l;
      if (s === 16 || s === 'hex') {
        l = '';
        for (var g = 0, m = 0, b = 0; b < this.length; b++) {
          var f = this.words[b],
            p = (((f << g) | m) & 16777215).toString(16);
          (m = (f >>> (24 - g)) & 16777215),
            (g += 2),
            g >= 26 && ((g -= 26), b--),
            m !== 0 || b !== this.length - 1 ? (l = I[6 - p.length] + p + l) : (l = p + l);
        }
        for (m !== 0 && (l = m.toString(16) + l); l.length % o !== 0; ) l = '0' + l;
        return this.negative !== 0 && (l = '-' + l), l;
      }
      if (s === (s | 0) && s >= 2 && s <= 36) {
        var h = E[s],
          x = R[s];
        l = '';
        var O = this.clone();
        for (O.negative = 0; !O.isZero(); ) {
          var C = O.modrn(x).toString(s);
          (O = O.idivn(x)), O.isZero() ? (l = C + l) : (l = I[h - C.length] + C + l);
        }
        for (this.isZero() && (l = '0' + l); l.length % o !== 0; ) l = '0' + l;
        return this.negative !== 0 && (l = '-' + l), l;
      }
      n(!1, 'Base should be between 2 and 36');
    }),
      (a.prototype.toNumber = function () {
        var s = this.words[0];
        return (
          this.length === 2
            ? (s += this.words[1] * 67108864)
            : this.length === 3 && this.words[2] === 1
            ? (s += 4503599627370496 + this.words[1] * 67108864)
            : this.length > 2 && n(!1, 'Number can only safely store up to 53 bits'),
          this.negative !== 0 ? -s : s
        );
      }),
      (a.prototype.toJSON = function () {
        return this.toString(16, 2);
      }),
      c &&
        (a.prototype.toBuffer = function (s, o) {
          return this.toArrayLike(c, s, o);
        }),
      (a.prototype.toArray = function (s, o) {
        return this.toArrayLike(Array, s, o);
      });
    var T = function (s, o) {
      return s.allocUnsafe ? s.allocUnsafe(o) : new s(o);
    };
    (a.prototype.toArrayLike = function (s, o, l) {
      this._strip();
      var g = this.byteLength(),
        m = l || Math.max(1, g);
      n(g <= m, 'byte array longer than desired length'), n(m > 0, 'Requested array length <= 0');
      var b = T(s, m),
        f = o === 'le' ? 'LE' : 'BE';
      return this['_toArrayLike' + f](b, g), b;
    }),
      (a.prototype._toArrayLikeLE = function (s, o) {
        for (var l = 0, g = 0, m = 0, b = 0; m < this.length; m++) {
          var f = (this.words[m] << b) | g;
          (s[l++] = f & 255),
            l < s.length && (s[l++] = (f >> 8) & 255),
            l < s.length && (s[l++] = (f >> 16) & 255),
            b === 6
              ? (l < s.length && (s[l++] = (f >> 24) & 255), (g = 0), (b = 0))
              : ((g = f >>> 24), (b += 2));
        }
        if (l < s.length) for (s[l++] = g; l < s.length; ) s[l++] = 0;
      }),
      (a.prototype._toArrayLikeBE = function (s, o) {
        for (var l = s.length - 1, g = 0, m = 0, b = 0; m < this.length; m++) {
          var f = (this.words[m] << b) | g;
          (s[l--] = f & 255),
            l >= 0 && (s[l--] = (f >> 8) & 255),
            l >= 0 && (s[l--] = (f >> 16) & 255),
            b === 6
              ? (l >= 0 && (s[l--] = (f >> 24) & 255), (g = 0), (b = 0))
              : ((g = f >>> 24), (b += 2));
        }
        if (l >= 0) for (s[l--] = g; l >= 0; ) s[l--] = 0;
      }),
      Math.clz32
        ? (a.prototype._countBits = function (s) {
            return 32 - Math.clz32(s);
          })
        : (a.prototype._countBits = function (s) {
            var o = s,
              l = 0;
            return (
              o >= 4096 && ((l += 13), (o >>>= 13)),
              o >= 64 && ((l += 7), (o >>>= 7)),
              o >= 8 && ((l += 4), (o >>>= 4)),
              o >= 2 && ((l += 2), (o >>>= 2)),
              l + o
            );
          }),
      (a.prototype._zeroBits = function (s) {
        if (s === 0) return 26;
        var o = s,
          l = 0;
        return (
          o & 8191 || ((l += 13), (o >>>= 13)),
          o & 127 || ((l += 7), (o >>>= 7)),
          o & 15 || ((l += 4), (o >>>= 4)),
          o & 3 || ((l += 2), (o >>>= 2)),
          o & 1 || l++,
          l
        );
      }),
      (a.prototype.bitLength = function () {
        var s = this.words[this.length - 1],
          o = this._countBits(s);
        return (this.length - 1) * 26 + o;
      });
    function z(u) {
      for (var s = new Array(u.bitLength()), o = 0; o < s.length; o++) {
        var l = (o / 26) | 0,
          g = o % 26;
        s[o] = (u.words[l] >>> g) & 1;
      }
      return s;
    }
    (a.prototype.zeroBits = function () {
      if (this.isZero()) return 0;
      for (var s = 0, o = 0; o < this.length; o++) {
        var l = this._zeroBits(this.words[o]);
        if (((s += l), l !== 26)) break;
      }
      return s;
    }),
      (a.prototype.byteLength = function () {
        return Math.ceil(this.bitLength() / 8);
      }),
      (a.prototype.toTwos = function (s) {
        return this.negative !== 0 ? this.abs().inotn(s).iaddn(1) : this.clone();
      }),
      (a.prototype.fromTwos = function (s) {
        return this.testn(s - 1) ? this.notn(s).iaddn(1).ineg() : this.clone();
      }),
      (a.prototype.isNeg = function () {
        return this.negative !== 0;
      }),
      (a.prototype.neg = function () {
        return this.clone().ineg();
      }),
      (a.prototype.ineg = function () {
        return this.isZero() || (this.negative ^= 1), this;
      }),
      (a.prototype.iuor = function (s) {
        for (; this.length < s.length; ) this.words[this.length++] = 0;
        for (var o = 0; o < s.length; o++) this.words[o] = this.words[o] | s.words[o];
        return this._strip();
      }),
      (a.prototype.ior = function (s) {
        return n((this.negative | s.negative) === 0), this.iuor(s);
      }),
      (a.prototype.or = function (s) {
        return this.length > s.length ? this.clone().ior(s) : s.clone().ior(this);
      }),
      (a.prototype.uor = function (s) {
        return this.length > s.length ? this.clone().iuor(s) : s.clone().iuor(this);
      }),
      (a.prototype.iuand = function (s) {
        var o;
        this.length > s.length ? (o = s) : (o = this);
        for (var l = 0; l < o.length; l++) this.words[l] = this.words[l] & s.words[l];
        return (this.length = o.length), this._strip();
      }),
      (a.prototype.iand = function (s) {
        return n((this.negative | s.negative) === 0), this.iuand(s);
      }),
      (a.prototype.and = function (s) {
        return this.length > s.length ? this.clone().iand(s) : s.clone().iand(this);
      }),
      (a.prototype.uand = function (s) {
        return this.length > s.length ? this.clone().iuand(s) : s.clone().iuand(this);
      }),
      (a.prototype.iuxor = function (s) {
        var o, l;
        this.length > s.length ? ((o = this), (l = s)) : ((o = s), (l = this));
        for (var g = 0; g < l.length; g++) this.words[g] = o.words[g] ^ l.words[g];
        if (this !== o) for (; g < o.length; g++) this.words[g] = o.words[g];
        return (this.length = o.length), this._strip();
      }),
      (a.prototype.ixor = function (s) {
        return n((this.negative | s.negative) === 0), this.iuxor(s);
      }),
      (a.prototype.xor = function (s) {
        return this.length > s.length ? this.clone().ixor(s) : s.clone().ixor(this);
      }),
      (a.prototype.uxor = function (s) {
        return this.length > s.length ? this.clone().iuxor(s) : s.clone().iuxor(this);
      }),
      (a.prototype.inotn = function (s) {
        n(typeof s == 'number' && s >= 0);
        var o = Math.ceil(s / 26) | 0,
          l = s % 26;
        this._expand(o), l > 0 && o--;
        for (var g = 0; g < o; g++) this.words[g] = ~this.words[g] & 67108863;
        return l > 0 && (this.words[g] = ~this.words[g] & (67108863 >> (26 - l))), this._strip();
      }),
      (a.prototype.notn = function (s) {
        return this.clone().inotn(s);
      }),
      (a.prototype.setn = function (s, o) {
        n(typeof s == 'number' && s >= 0);
        var l = (s / 26) | 0,
          g = s % 26;
        return (
          this._expand(l + 1),
          o
            ? (this.words[l] = this.words[l] | (1 << g))
            : (this.words[l] = this.words[l] & ~(1 << g)),
          this._strip()
        );
      }),
      (a.prototype.iadd = function (s) {
        var o;
        if (this.negative !== 0 && s.negative === 0)
          return (this.negative = 0), (o = this.isub(s)), (this.negative ^= 1), this._normSign();
        if (this.negative === 0 && s.negative !== 0)
          return (s.negative = 0), (o = this.isub(s)), (s.negative = 1), o._normSign();
        var l, g;
        this.length > s.length ? ((l = this), (g = s)) : ((l = s), (g = this));
        for (var m = 0, b = 0; b < g.length; b++)
          (o = (l.words[b] | 0) + (g.words[b] | 0) + m),
            (this.words[b] = o & 67108863),
            (m = o >>> 26);
        for (; m !== 0 && b < l.length; b++)
          (o = (l.words[b] | 0) + m), (this.words[b] = o & 67108863), (m = o >>> 26);
        if (((this.length = l.length), m !== 0)) (this.words[this.length] = m), this.length++;
        else if (l !== this) for (; b < l.length; b++) this.words[b] = l.words[b];
        return this;
      }),
      (a.prototype.add = function (s) {
        var o;
        return s.negative !== 0 && this.negative === 0
          ? ((s.negative = 0), (o = this.sub(s)), (s.negative ^= 1), o)
          : s.negative === 0 && this.negative !== 0
          ? ((this.negative = 0), (o = s.sub(this)), (this.negative = 1), o)
          : this.length > s.length
          ? this.clone().iadd(s)
          : s.clone().iadd(this);
      }),
      (a.prototype.isub = function (s) {
        if (s.negative !== 0) {
          s.negative = 0;
          var o = this.iadd(s);
          return (s.negative = 1), o._normSign();
        } else if (this.negative !== 0)
          return (this.negative = 0), this.iadd(s), (this.negative = 1), this._normSign();
        var l = this.cmp(s);
        if (l === 0) return (this.negative = 0), (this.length = 1), (this.words[0] = 0), this;
        var g, m;
        l > 0 ? ((g = this), (m = s)) : ((g = s), (m = this));
        for (var b = 0, f = 0; f < m.length; f++)
          (o = (g.words[f] | 0) - (m.words[f] | 0) + b),
            (b = o >> 26),
            (this.words[f] = o & 67108863);
        for (; b !== 0 && f < g.length; f++)
          (o = (g.words[f] | 0) + b), (b = o >> 26), (this.words[f] = o & 67108863);
        if (b === 0 && f < g.length && g !== this)
          for (; f < g.length; f++) this.words[f] = g.words[f];
        return (
          (this.length = Math.max(this.length, f)), g !== this && (this.negative = 1), this._strip()
        );
      }),
      (a.prototype.sub = function (s) {
        return this.clone().isub(s);
      });
    function q(u, s, o) {
      o.negative = s.negative ^ u.negative;
      var l = (u.length + s.length) | 0;
      (o.length = l), (l = (l - 1) | 0);
      var g = u.words[0] | 0,
        m = s.words[0] | 0,
        b = g * m,
        f = b & 67108863,
        p = (b / 67108864) | 0;
      o.words[0] = f;
      for (var h = 1; h < l; h++) {
        for (
          var x = p >>> 26,
            O = p & 67108863,
            C = Math.min(h, s.length - 1),
            L = Math.max(0, h - u.length + 1);
          L <= C;
          L++
        ) {
          var D = (h - L) | 0;
          (g = u.words[D] | 0),
            (m = s.words[L] | 0),
            (b = g * m + O),
            (x += (b / 67108864) | 0),
            (O = b & 67108863);
        }
        (o.words[h] = O | 0), (p = x | 0);
      }
      return p !== 0 ? (o.words[h] = p | 0) : o.length--, o._strip();
    }
    var Y = function (s, o, l) {
      var g = s.words,
        m = o.words,
        b = l.words,
        f = 0,
        p,
        h,
        x,
        O = g[0] | 0,
        C = O & 8191,
        L = O >>> 13,
        D = g[1] | 0,
        Z = D & 8191,
        j = D >>> 13,
        le = g[2] | 0,
        Ae = le & 8191,
        Q = le >>> 13,
        Re = g[3] | 0,
        Ee = Re & 8191,
        ne = Re >>> 13,
        ke = g[4] | 0,
        De = ke & 8191,
        ae = ke >>> 13,
        Fe = g[5] | 0,
        $e = Fe & 8191,
        ce = Fe >>> 13,
        Ue = g[6] | 0,
        Ge = Ue & 8191,
        oe = Ue >>> 13,
        ze = g[7] | 0,
        rt = ze & 8191,
        pe = ze >>> 13,
        Ke = g[8] | 0,
        Xe = Ke & 8191,
        be = Ke >>> 13,
        nt = g[9] | 0,
        it = nt & 8191,
        me = nt >>> 13,
        Ze = m[0] | 0,
        at = Ze & 8191,
        ge = Ze >>> 13,
        Je = m[1] | 0,
        Be = Je & 8191,
        he = Je >>> 13,
        He = m[2] | 0,
        We = He & 8191,
        ue = He >>> 13,
        st = m[3] | 0,
        ot = st & 8191,
        ye = st >>> 13,
        ft = m[4] | 0,
        ct = ft & 8191,
        ve = ft >>> 13,
        Ye = m[5] | 0,
        Le = Ye & 8191,
        we = Ye >>> 13,
        ut = m[6] | 0,
        dt = ut & 8191,
        xe = ut >>> 13,
        ht = m[7] | 0,
        Ce = ht & 8191,
        Me = ht >>> 13,
        lt = m[8] | 0,
        pt = lt & 8191,
        _e = lt >>> 13,
        vt = m[9] | 0,
        qe = vt & 8191,
        Qe = vt >>> 13;
      (l.negative = s.negative ^ o.negative),
        (l.length = 19),
        (p = Math.imul(C, at)),
        (h = Math.imul(C, ge)),
        (h = (h + Math.imul(L, at)) | 0),
        (x = Math.imul(L, ge));
      var pr = (((f + p) | 0) + ((h & 8191) << 13)) | 0;
      (f = (((x + (h >>> 13)) | 0) + (pr >>> 26)) | 0),
        (pr &= 67108863),
        (p = Math.imul(Z, at)),
        (h = Math.imul(Z, ge)),
        (h = (h + Math.imul(j, at)) | 0),
        (x = Math.imul(j, ge)),
        (p = (p + Math.imul(C, Be)) | 0),
        (h = (h + Math.imul(C, he)) | 0),
        (h = (h + Math.imul(L, Be)) | 0),
        (x = (x + Math.imul(L, he)) | 0);
      var vr = (((f + p) | 0) + ((h & 8191) << 13)) | 0;
      (f = (((x + (h >>> 13)) | 0) + (vr >>> 26)) | 0),
        (vr &= 67108863),
        (p = Math.imul(Ae, at)),
        (h = Math.imul(Ae, ge)),
        (h = (h + Math.imul(Q, at)) | 0),
        (x = Math.imul(Q, ge)),
        (p = (p + Math.imul(Z, Be)) | 0),
        (h = (h + Math.imul(Z, he)) | 0),
        (h = (h + Math.imul(j, Be)) | 0),
        (x = (x + Math.imul(j, he)) | 0),
        (p = (p + Math.imul(C, We)) | 0),
        (h = (h + Math.imul(C, ue)) | 0),
        (h = (h + Math.imul(L, We)) | 0),
        (x = (x + Math.imul(L, ue)) | 0);
      var br = (((f + p) | 0) + ((h & 8191) << 13)) | 0;
      (f = (((x + (h >>> 13)) | 0) + (br >>> 26)) | 0),
        (br &= 67108863),
        (p = Math.imul(Ee, at)),
        (h = Math.imul(Ee, ge)),
        (h = (h + Math.imul(ne, at)) | 0),
        (x = Math.imul(ne, ge)),
        (p = (p + Math.imul(Ae, Be)) | 0),
        (h = (h + Math.imul(Ae, he)) | 0),
        (h = (h + Math.imul(Q, Be)) | 0),
        (x = (x + Math.imul(Q, he)) | 0),
        (p = (p + Math.imul(Z, We)) | 0),
        (h = (h + Math.imul(Z, ue)) | 0),
        (h = (h + Math.imul(j, We)) | 0),
        (x = (x + Math.imul(j, ue)) | 0),
        (p = (p + Math.imul(C, ot)) | 0),
        (h = (h + Math.imul(C, ye)) | 0),
        (h = (h + Math.imul(L, ot)) | 0),
        (x = (x + Math.imul(L, ye)) | 0);
      var mr = (((f + p) | 0) + ((h & 8191) << 13)) | 0;
      (f = (((x + (h >>> 13)) | 0) + (mr >>> 26)) | 0),
        (mr &= 67108863),
        (p = Math.imul(De, at)),
        (h = Math.imul(De, ge)),
        (h = (h + Math.imul(ae, at)) | 0),
        (x = Math.imul(ae, ge)),
        (p = (p + Math.imul(Ee, Be)) | 0),
        (h = (h + Math.imul(Ee, he)) | 0),
        (h = (h + Math.imul(ne, Be)) | 0),
        (x = (x + Math.imul(ne, he)) | 0),
        (p = (p + Math.imul(Ae, We)) | 0),
        (h = (h + Math.imul(Ae, ue)) | 0),
        (h = (h + Math.imul(Q, We)) | 0),
        (x = (x + Math.imul(Q, ue)) | 0),
        (p = (p + Math.imul(Z, ot)) | 0),
        (h = (h + Math.imul(Z, ye)) | 0),
        (h = (h + Math.imul(j, ot)) | 0),
        (x = (x + Math.imul(j, ye)) | 0),
        (p = (p + Math.imul(C, ct)) | 0),
        (h = (h + Math.imul(C, ve)) | 0),
        (h = (h + Math.imul(L, ct)) | 0),
        (x = (x + Math.imul(L, ve)) | 0);
      var kt = (((f + p) | 0) + ((h & 8191) << 13)) | 0;
      (f = (((x + (h >>> 13)) | 0) + (kt >>> 26)) | 0),
        (kt &= 67108863),
        (p = Math.imul($e, at)),
        (h = Math.imul($e, ge)),
        (h = (h + Math.imul(ce, at)) | 0),
        (x = Math.imul(ce, ge)),
        (p = (p + Math.imul(De, Be)) | 0),
        (h = (h + Math.imul(De, he)) | 0),
        (h = (h + Math.imul(ae, Be)) | 0),
        (x = (x + Math.imul(ae, he)) | 0),
        (p = (p + Math.imul(Ee, We)) | 0),
        (h = (h + Math.imul(Ee, ue)) | 0),
        (h = (h + Math.imul(ne, We)) | 0),
        (x = (x + Math.imul(ne, ue)) | 0),
        (p = (p + Math.imul(Ae, ot)) | 0),
        (h = (h + Math.imul(Ae, ye)) | 0),
        (h = (h + Math.imul(Q, ot)) | 0),
        (x = (x + Math.imul(Q, ye)) | 0),
        (p = (p + Math.imul(Z, ct)) | 0),
        (h = (h + Math.imul(Z, ve)) | 0),
        (h = (h + Math.imul(j, ct)) | 0),
        (x = (x + Math.imul(j, ve)) | 0),
        (p = (p + Math.imul(C, Le)) | 0),
        (h = (h + Math.imul(C, we)) | 0),
        (h = (h + Math.imul(L, Le)) | 0),
        (x = (x + Math.imul(L, we)) | 0);
      var gr = (((f + p) | 0) + ((h & 8191) << 13)) | 0;
      (f = (((x + (h >>> 13)) | 0) + (gr >>> 26)) | 0),
        (gr &= 67108863),
        (p = Math.imul(Ge, at)),
        (h = Math.imul(Ge, ge)),
        (h = (h + Math.imul(oe, at)) | 0),
        (x = Math.imul(oe, ge)),
        (p = (p + Math.imul($e, Be)) | 0),
        (h = (h + Math.imul($e, he)) | 0),
        (h = (h + Math.imul(ce, Be)) | 0),
        (x = (x + Math.imul(ce, he)) | 0),
        (p = (p + Math.imul(De, We)) | 0),
        (h = (h + Math.imul(De, ue)) | 0),
        (h = (h + Math.imul(ae, We)) | 0),
        (x = (x + Math.imul(ae, ue)) | 0),
        (p = (p + Math.imul(Ee, ot)) | 0),
        (h = (h + Math.imul(Ee, ye)) | 0),
        (h = (h + Math.imul(ne, ot)) | 0),
        (x = (x + Math.imul(ne, ye)) | 0),
        (p = (p + Math.imul(Ae, ct)) | 0),
        (h = (h + Math.imul(Ae, ve)) | 0),
        (h = (h + Math.imul(Q, ct)) | 0),
        (x = (x + Math.imul(Q, ve)) | 0),
        (p = (p + Math.imul(Z, Le)) | 0),
        (h = (h + Math.imul(Z, we)) | 0),
        (h = (h + Math.imul(j, Le)) | 0),
        (x = (x + Math.imul(j, we)) | 0),
        (p = (p + Math.imul(C, dt)) | 0),
        (h = (h + Math.imul(C, xe)) | 0),
        (h = (h + Math.imul(L, dt)) | 0),
        (x = (x + Math.imul(L, xe)) | 0);
      var yr = (((f + p) | 0) + ((h & 8191) << 13)) | 0;
      (f = (((x + (h >>> 13)) | 0) + (yr >>> 26)) | 0),
        (yr &= 67108863),
        (p = Math.imul(rt, at)),
        (h = Math.imul(rt, ge)),
        (h = (h + Math.imul(pe, at)) | 0),
        (x = Math.imul(pe, ge)),
        (p = (p + Math.imul(Ge, Be)) | 0),
        (h = (h + Math.imul(Ge, he)) | 0),
        (h = (h + Math.imul(oe, Be)) | 0),
        (x = (x + Math.imul(oe, he)) | 0),
        (p = (p + Math.imul($e, We)) | 0),
        (h = (h + Math.imul($e, ue)) | 0),
        (h = (h + Math.imul(ce, We)) | 0),
        (x = (x + Math.imul(ce, ue)) | 0),
        (p = (p + Math.imul(De, ot)) | 0),
        (h = (h + Math.imul(De, ye)) | 0),
        (h = (h + Math.imul(ae, ot)) | 0),
        (x = (x + Math.imul(ae, ye)) | 0),
        (p = (p + Math.imul(Ee, ct)) | 0),
        (h = (h + Math.imul(Ee, ve)) | 0),
        (h = (h + Math.imul(ne, ct)) | 0),
        (x = (x + Math.imul(ne, ve)) | 0),
        (p = (p + Math.imul(Ae, Le)) | 0),
        (h = (h + Math.imul(Ae, we)) | 0),
        (h = (h + Math.imul(Q, Le)) | 0),
        (x = (x + Math.imul(Q, we)) | 0),
        (p = (p + Math.imul(Z, dt)) | 0),
        (h = (h + Math.imul(Z, xe)) | 0),
        (h = (h + Math.imul(j, dt)) | 0),
        (x = (x + Math.imul(j, xe)) | 0),
        (p = (p + Math.imul(C, Ce)) | 0),
        (h = (h + Math.imul(C, Me)) | 0),
        (h = (h + Math.imul(L, Ce)) | 0),
        (x = (x + Math.imul(L, Me)) | 0);
      var wr = (((f + p) | 0) + ((h & 8191) << 13)) | 0;
      (f = (((x + (h >>> 13)) | 0) + (wr >>> 26)) | 0),
        (wr &= 67108863),
        (p = Math.imul(Xe, at)),
        (h = Math.imul(Xe, ge)),
        (h = (h + Math.imul(be, at)) | 0),
        (x = Math.imul(be, ge)),
        (p = (p + Math.imul(rt, Be)) | 0),
        (h = (h + Math.imul(rt, he)) | 0),
        (h = (h + Math.imul(pe, Be)) | 0),
        (x = (x + Math.imul(pe, he)) | 0),
        (p = (p + Math.imul(Ge, We)) | 0),
        (h = (h + Math.imul(Ge, ue)) | 0),
        (h = (h + Math.imul(oe, We)) | 0),
        (x = (x + Math.imul(oe, ue)) | 0),
        (p = (p + Math.imul($e, ot)) | 0),
        (h = (h + Math.imul($e, ye)) | 0),
        (h = (h + Math.imul(ce, ot)) | 0),
        (x = (x + Math.imul(ce, ye)) | 0),
        (p = (p + Math.imul(De, ct)) | 0),
        (h = (h + Math.imul(De, ve)) | 0),
        (h = (h + Math.imul(ae, ct)) | 0),
        (x = (x + Math.imul(ae, ve)) | 0),
        (p = (p + Math.imul(Ee, Le)) | 0),
        (h = (h + Math.imul(Ee, we)) | 0),
        (h = (h + Math.imul(ne, Le)) | 0),
        (x = (x + Math.imul(ne, we)) | 0),
        (p = (p + Math.imul(Ae, dt)) | 0),
        (h = (h + Math.imul(Ae, xe)) | 0),
        (h = (h + Math.imul(Q, dt)) | 0),
        (x = (x + Math.imul(Q, xe)) | 0),
        (p = (p + Math.imul(Z, Ce)) | 0),
        (h = (h + Math.imul(Z, Me)) | 0),
        (h = (h + Math.imul(j, Ce)) | 0),
        (x = (x + Math.imul(j, Me)) | 0),
        (p = (p + Math.imul(C, pt)) | 0),
        (h = (h + Math.imul(C, _e)) | 0),
        (h = (h + Math.imul(L, pt)) | 0),
        (x = (x + Math.imul(L, _e)) | 0);
      var xr = (((f + p) | 0) + ((h & 8191) << 13)) | 0;
      (f = (((x + (h >>> 13)) | 0) + (xr >>> 26)) | 0),
        (xr &= 67108863),
        (p = Math.imul(it, at)),
        (h = Math.imul(it, ge)),
        (h = (h + Math.imul(me, at)) | 0),
        (x = Math.imul(me, ge)),
        (p = (p + Math.imul(Xe, Be)) | 0),
        (h = (h + Math.imul(Xe, he)) | 0),
        (h = (h + Math.imul(be, Be)) | 0),
        (x = (x + Math.imul(be, he)) | 0),
        (p = (p + Math.imul(rt, We)) | 0),
        (h = (h + Math.imul(rt, ue)) | 0),
        (h = (h + Math.imul(pe, We)) | 0),
        (x = (x + Math.imul(pe, ue)) | 0),
        (p = (p + Math.imul(Ge, ot)) | 0),
        (h = (h + Math.imul(Ge, ye)) | 0),
        (h = (h + Math.imul(oe, ot)) | 0),
        (x = (x + Math.imul(oe, ye)) | 0),
        (p = (p + Math.imul($e, ct)) | 0),
        (h = (h + Math.imul($e, ve)) | 0),
        (h = (h + Math.imul(ce, ct)) | 0),
        (x = (x + Math.imul(ce, ve)) | 0),
        (p = (p + Math.imul(De, Le)) | 0),
        (h = (h + Math.imul(De, we)) | 0),
        (h = (h + Math.imul(ae, Le)) | 0),
        (x = (x + Math.imul(ae, we)) | 0),
        (p = (p + Math.imul(Ee, dt)) | 0),
        (h = (h + Math.imul(Ee, xe)) | 0),
        (h = (h + Math.imul(ne, dt)) | 0),
        (x = (x + Math.imul(ne, xe)) | 0),
        (p = (p + Math.imul(Ae, Ce)) | 0),
        (h = (h + Math.imul(Ae, Me)) | 0),
        (h = (h + Math.imul(Q, Ce)) | 0),
        (x = (x + Math.imul(Q, Me)) | 0),
        (p = (p + Math.imul(Z, pt)) | 0),
        (h = (h + Math.imul(Z, _e)) | 0),
        (h = (h + Math.imul(j, pt)) | 0),
        (x = (x + Math.imul(j, _e)) | 0),
        (p = (p + Math.imul(C, qe)) | 0),
        (h = (h + Math.imul(C, Qe)) | 0),
        (h = (h + Math.imul(L, qe)) | 0),
        (x = (x + Math.imul(L, Qe)) | 0);
      var Mr = (((f + p) | 0) + ((h & 8191) << 13)) | 0;
      (f = (((x + (h >>> 13)) | 0) + (Mr >>> 26)) | 0),
        (Mr &= 67108863),
        (p = Math.imul(it, Be)),
        (h = Math.imul(it, he)),
        (h = (h + Math.imul(me, Be)) | 0),
        (x = Math.imul(me, he)),
        (p = (p + Math.imul(Xe, We)) | 0),
        (h = (h + Math.imul(Xe, ue)) | 0),
        (h = (h + Math.imul(be, We)) | 0),
        (x = (x + Math.imul(be, ue)) | 0),
        (p = (p + Math.imul(rt, ot)) | 0),
        (h = (h + Math.imul(rt, ye)) | 0),
        (h = (h + Math.imul(pe, ot)) | 0),
        (x = (x + Math.imul(pe, ye)) | 0),
        (p = (p + Math.imul(Ge, ct)) | 0),
        (h = (h + Math.imul(Ge, ve)) | 0),
        (h = (h + Math.imul(oe, ct)) | 0),
        (x = (x + Math.imul(oe, ve)) | 0),
        (p = (p + Math.imul($e, Le)) | 0),
        (h = (h + Math.imul($e, we)) | 0),
        (h = (h + Math.imul(ce, Le)) | 0),
        (x = (x + Math.imul(ce, we)) | 0),
        (p = (p + Math.imul(De, dt)) | 0),
        (h = (h + Math.imul(De, xe)) | 0),
        (h = (h + Math.imul(ae, dt)) | 0),
        (x = (x + Math.imul(ae, xe)) | 0),
        (p = (p + Math.imul(Ee, Ce)) | 0),
        (h = (h + Math.imul(Ee, Me)) | 0),
        (h = (h + Math.imul(ne, Ce)) | 0),
        (x = (x + Math.imul(ne, Me)) | 0),
        (p = (p + Math.imul(Ae, pt)) | 0),
        (h = (h + Math.imul(Ae, _e)) | 0),
        (h = (h + Math.imul(Q, pt)) | 0),
        (x = (x + Math.imul(Q, _e)) | 0),
        (p = (p + Math.imul(Z, qe)) | 0),
        (h = (h + Math.imul(Z, Qe)) | 0),
        (h = (h + Math.imul(j, qe)) | 0),
        (x = (x + Math.imul(j, Qe)) | 0);
      var _r = (((f + p) | 0) + ((h & 8191) << 13)) | 0;
      (f = (((x + (h >>> 13)) | 0) + (_r >>> 26)) | 0),
        (_r &= 67108863),
        (p = Math.imul(it, We)),
        (h = Math.imul(it, ue)),
        (h = (h + Math.imul(me, We)) | 0),
        (x = Math.imul(me, ue)),
        (p = (p + Math.imul(Xe, ot)) | 0),
        (h = (h + Math.imul(Xe, ye)) | 0),
        (h = (h + Math.imul(be, ot)) | 0),
        (x = (x + Math.imul(be, ye)) | 0),
        (p = (p + Math.imul(rt, ct)) | 0),
        (h = (h + Math.imul(rt, ve)) | 0),
        (h = (h + Math.imul(pe, ct)) | 0),
        (x = (x + Math.imul(pe, ve)) | 0),
        (p = (p + Math.imul(Ge, Le)) | 0),
        (h = (h + Math.imul(Ge, we)) | 0),
        (h = (h + Math.imul(oe, Le)) | 0),
        (x = (x + Math.imul(oe, we)) | 0),
        (p = (p + Math.imul($e, dt)) | 0),
        (h = (h + Math.imul($e, xe)) | 0),
        (h = (h + Math.imul(ce, dt)) | 0),
        (x = (x + Math.imul(ce, xe)) | 0),
        (p = (p + Math.imul(De, Ce)) | 0),
        (h = (h + Math.imul(De, Me)) | 0),
        (h = (h + Math.imul(ae, Ce)) | 0),
        (x = (x + Math.imul(ae, Me)) | 0),
        (p = (p + Math.imul(Ee, pt)) | 0),
        (h = (h + Math.imul(Ee, _e)) | 0),
        (h = (h + Math.imul(ne, pt)) | 0),
        (x = (x + Math.imul(ne, _e)) | 0),
        (p = (p + Math.imul(Ae, qe)) | 0),
        (h = (h + Math.imul(Ae, Qe)) | 0),
        (h = (h + Math.imul(Q, qe)) | 0),
        (x = (x + Math.imul(Q, Qe)) | 0);
      var rr = (((f + p) | 0) + ((h & 8191) << 13)) | 0;
      (f = (((x + (h >>> 13)) | 0) + (rr >>> 26)) | 0),
        (rr &= 67108863),
        (p = Math.imul(it, ot)),
        (h = Math.imul(it, ye)),
        (h = (h + Math.imul(me, ot)) | 0),
        (x = Math.imul(me, ye)),
        (p = (p + Math.imul(Xe, ct)) | 0),
        (h = (h + Math.imul(Xe, ve)) | 0),
        (h = (h + Math.imul(be, ct)) | 0),
        (x = (x + Math.imul(be, ve)) | 0),
        (p = (p + Math.imul(rt, Le)) | 0),
        (h = (h + Math.imul(rt, we)) | 0),
        (h = (h + Math.imul(pe, Le)) | 0),
        (x = (x + Math.imul(pe, we)) | 0),
        (p = (p + Math.imul(Ge, dt)) | 0),
        (h = (h + Math.imul(Ge, xe)) | 0),
        (h = (h + Math.imul(oe, dt)) | 0),
        (x = (x + Math.imul(oe, xe)) | 0),
        (p = (p + Math.imul($e, Ce)) | 0),
        (h = (h + Math.imul($e, Me)) | 0),
        (h = (h + Math.imul(ce, Ce)) | 0),
        (x = (x + Math.imul(ce, Me)) | 0),
        (p = (p + Math.imul(De, pt)) | 0),
        (h = (h + Math.imul(De, _e)) | 0),
        (h = (h + Math.imul(ae, pt)) | 0),
        (x = (x + Math.imul(ae, _e)) | 0),
        (p = (p + Math.imul(Ee, qe)) | 0),
        (h = (h + Math.imul(Ee, Qe)) | 0),
        (h = (h + Math.imul(ne, qe)) | 0),
        (x = (x + Math.imul(ne, Qe)) | 0);
      var Vt = (((f + p) | 0) + ((h & 8191) << 13)) | 0;
      (f = (((x + (h >>> 13)) | 0) + (Vt >>> 26)) | 0),
        (Vt &= 67108863),
        (p = Math.imul(it, ct)),
        (h = Math.imul(it, ve)),
        (h = (h + Math.imul(me, ct)) | 0),
        (x = Math.imul(me, ve)),
        (p = (p + Math.imul(Xe, Le)) | 0),
        (h = (h + Math.imul(Xe, we)) | 0),
        (h = (h + Math.imul(be, Le)) | 0),
        (x = (x + Math.imul(be, we)) | 0),
        (p = (p + Math.imul(rt, dt)) | 0),
        (h = (h + Math.imul(rt, xe)) | 0),
        (h = (h + Math.imul(pe, dt)) | 0),
        (x = (x + Math.imul(pe, xe)) | 0),
        (p = (p + Math.imul(Ge, Ce)) | 0),
        (h = (h + Math.imul(Ge, Me)) | 0),
        (h = (h + Math.imul(oe, Ce)) | 0),
        (x = (x + Math.imul(oe, Me)) | 0),
        (p = (p + Math.imul($e, pt)) | 0),
        (h = (h + Math.imul($e, _e)) | 0),
        (h = (h + Math.imul(ce, pt)) | 0),
        (x = (x + Math.imul(ce, _e)) | 0),
        (p = (p + Math.imul(De, qe)) | 0),
        (h = (h + Math.imul(De, Qe)) | 0),
        (h = (h + Math.imul(ae, qe)) | 0),
        (x = (x + Math.imul(ae, Qe)) | 0);
      var Sr = (((f + p) | 0) + ((h & 8191) << 13)) | 0;
      (f = (((x + (h >>> 13)) | 0) + (Sr >>> 26)) | 0),
        (Sr &= 67108863),
        (p = Math.imul(it, Le)),
        (h = Math.imul(it, we)),
        (h = (h + Math.imul(me, Le)) | 0),
        (x = Math.imul(me, we)),
        (p = (p + Math.imul(Xe, dt)) | 0),
        (h = (h + Math.imul(Xe, xe)) | 0),
        (h = (h + Math.imul(be, dt)) | 0),
        (x = (x + Math.imul(be, xe)) | 0),
        (p = (p + Math.imul(rt, Ce)) | 0),
        (h = (h + Math.imul(rt, Me)) | 0),
        (h = (h + Math.imul(pe, Ce)) | 0),
        (x = (x + Math.imul(pe, Me)) | 0),
        (p = (p + Math.imul(Ge, pt)) | 0),
        (h = (h + Math.imul(Ge, _e)) | 0),
        (h = (h + Math.imul(oe, pt)) | 0),
        (x = (x + Math.imul(oe, _e)) | 0),
        (p = (p + Math.imul($e, qe)) | 0),
        (h = (h + Math.imul($e, Qe)) | 0),
        (h = (h + Math.imul(ce, qe)) | 0),
        (x = (x + Math.imul(ce, Qe)) | 0);
      var Xt = (((f + p) | 0) + ((h & 8191) << 13)) | 0;
      (f = (((x + (h >>> 13)) | 0) + (Xt >>> 26)) | 0),
        (Xt &= 67108863),
        (p = Math.imul(it, dt)),
        (h = Math.imul(it, xe)),
        (h = (h + Math.imul(me, dt)) | 0),
        (x = Math.imul(me, xe)),
        (p = (p + Math.imul(Xe, Ce)) | 0),
        (h = (h + Math.imul(Xe, Me)) | 0),
        (h = (h + Math.imul(be, Ce)) | 0),
        (x = (x + Math.imul(be, Me)) | 0),
        (p = (p + Math.imul(rt, pt)) | 0),
        (h = (h + Math.imul(rt, _e)) | 0),
        (h = (h + Math.imul(pe, pt)) | 0),
        (x = (x + Math.imul(pe, _e)) | 0),
        (p = (p + Math.imul(Ge, qe)) | 0),
        (h = (h + Math.imul(Ge, Qe)) | 0),
        (h = (h + Math.imul(oe, qe)) | 0),
        (x = (x + Math.imul(oe, Qe)) | 0);
      var Ar = (((f + p) | 0) + ((h & 8191) << 13)) | 0;
      (f = (((x + (h >>> 13)) | 0) + (Ar >>> 26)) | 0),
        (Ar &= 67108863),
        (p = Math.imul(it, Ce)),
        (h = Math.imul(it, Me)),
        (h = (h + Math.imul(me, Ce)) | 0),
        (x = Math.imul(me, Me)),
        (p = (p + Math.imul(Xe, pt)) | 0),
        (h = (h + Math.imul(Xe, _e)) | 0),
        (h = (h + Math.imul(be, pt)) | 0),
        (x = (x + Math.imul(be, _e)) | 0),
        (p = (p + Math.imul(rt, qe)) | 0),
        (h = (h + Math.imul(rt, Qe)) | 0),
        (h = (h + Math.imul(pe, qe)) | 0),
        (x = (x + Math.imul(pe, Qe)) | 0);
      var Er = (((f + p) | 0) + ((h & 8191) << 13)) | 0;
      (f = (((x + (h >>> 13)) | 0) + (Er >>> 26)) | 0),
        (Er &= 67108863),
        (p = Math.imul(it, pt)),
        (h = Math.imul(it, _e)),
        (h = (h + Math.imul(me, pt)) | 0),
        (x = Math.imul(me, _e)),
        (p = (p + Math.imul(Xe, qe)) | 0),
        (h = (h + Math.imul(Xe, Qe)) | 0),
        (h = (h + Math.imul(be, qe)) | 0),
        (x = (x + Math.imul(be, Qe)) | 0);
      var On = (((f + p) | 0) + ((h & 8191) << 13)) | 0;
      (f = (((x + (h >>> 13)) | 0) + (On >>> 26)) | 0),
        (On &= 67108863),
        (p = Math.imul(it, qe)),
        (h = Math.imul(it, Qe)),
        (h = (h + Math.imul(me, qe)) | 0),
        (x = Math.imul(me, Qe));
      var Tn = (((f + p) | 0) + ((h & 8191) << 13)) | 0;
      return (
        (f = (((x + (h >>> 13)) | 0) + (Tn >>> 26)) | 0),
        (Tn &= 67108863),
        (b[0] = pr),
        (b[1] = vr),
        (b[2] = br),
        (b[3] = mr),
        (b[4] = kt),
        (b[5] = gr),
        (b[6] = yr),
        (b[7] = wr),
        (b[8] = xr),
        (b[9] = Mr),
        (b[10] = _r),
        (b[11] = rr),
        (b[12] = Vt),
        (b[13] = Sr),
        (b[14] = Xt),
        (b[15] = Ar),
        (b[16] = Er),
        (b[17] = On),
        (b[18] = Tn),
        f !== 0 && ((b[19] = f), l.length++),
        l
      );
    };
    Math.imul || (Y = q);
    function Se(u, s, o) {
      (o.negative = s.negative ^ u.negative), (o.length = u.length + s.length);
      for (var l = 0, g = 0, m = 0; m < o.length - 1; m++) {
        var b = g;
        g = 0;
        for (
          var f = l & 67108863, p = Math.min(m, s.length - 1), h = Math.max(0, m - u.length + 1);
          h <= p;
          h++
        ) {
          var x = m - h,
            O = u.words[x] | 0,
            C = s.words[h] | 0,
            L = O * C,
            D = L & 67108863;
          (b = (b + ((L / 67108864) | 0)) | 0),
            (D = (D + f) | 0),
            (f = D & 67108863),
            (b = (b + (D >>> 26)) | 0),
            (g += b >>> 26),
            (b &= 67108863);
        }
        (o.words[m] = f), (l = b), (b = g);
      }
      return l !== 0 ? (o.words[m] = l) : o.length--, o._strip();
    }
    function de(u, s, o) {
      return Se(u, s, o);
    }
    (a.prototype.mulTo = function (s, o) {
      var l,
        g = this.length + s.length;
      return (
        this.length === 10 && s.length === 10
          ? (l = Y(this, s, o))
          : g < 63
          ? (l = q(this, s, o))
          : g < 1024
          ? (l = Se(this, s, o))
          : (l = de(this, s, o)),
        l
      );
    }),
      (a.prototype.mul = function (s) {
        var o = new a(null);
        return (o.words = new Array(this.length + s.length)), this.mulTo(s, o);
      }),
      (a.prototype.mulf = function (s) {
        var o = new a(null);
        return (o.words = new Array(this.length + s.length)), de(this, s, o);
      }),
      (a.prototype.imul = function (s) {
        return this.clone().mulTo(s, this);
      }),
      (a.prototype.imuln = function (s) {
        var o = s < 0;
        o && (s = -s), n(typeof s == 'number'), n(s < 67108864);
        for (var l = 0, g = 0; g < this.length; g++) {
          var m = (this.words[g] | 0) * s,
            b = (m & 67108863) + (l & 67108863);
          (l >>= 26), (l += (m / 67108864) | 0), (l += b >>> 26), (this.words[g] = b & 67108863);
        }
        return l !== 0 && ((this.words[g] = l), this.length++), o ? this.ineg() : this;
      }),
      (a.prototype.muln = function (s) {
        return this.clone().imuln(s);
      }),
      (a.prototype.sqr = function () {
        return this.mul(this);
      }),
      (a.prototype.isqr = function () {
        return this.imul(this.clone());
      }),
      (a.prototype.pow = function (s) {
        var o = z(s);
        if (o.length === 0) return new a(1);
        for (var l = this, g = 0; g < o.length && o[g] === 0; g++, l = l.sqr());
        if (++g < o.length)
          for (var m = l.sqr(); g < o.length; g++, m = m.sqr()) o[g] !== 0 && (l = l.mul(m));
        return l;
      }),
      (a.prototype.iushln = function (s) {
        n(typeof s == 'number' && s >= 0);
        var o = s % 26,
          l = (s - o) / 26,
          g = (67108863 >>> (26 - o)) << (26 - o),
          m;
        if (o !== 0) {
          var b = 0;
          for (m = 0; m < this.length; m++) {
            var f = this.words[m] & g,
              p = ((this.words[m] | 0) - f) << o;
            (this.words[m] = p | b), (b = f >>> (26 - o));
          }
          b && ((this.words[m] = b), this.length++);
        }
        if (l !== 0) {
          for (m = this.length - 1; m >= 0; m--) this.words[m + l] = this.words[m];
          for (m = 0; m < l; m++) this.words[m] = 0;
          this.length += l;
        }
        return this._strip();
      }),
      (a.prototype.ishln = function (s) {
        return n(this.negative === 0), this.iushln(s);
      }),
      (a.prototype.iushrn = function (s, o, l) {
        n(typeof s == 'number' && s >= 0);
        var g;
        o ? (g = (o - (o % 26)) / 26) : (g = 0);
        var m = s % 26,
          b = Math.min((s - m) / 26, this.length),
          f = 67108863 ^ ((67108863 >>> m) << m),
          p = l;
        if (((g -= b), (g = Math.max(0, g)), p)) {
          for (var h = 0; h < b; h++) p.words[h] = this.words[h];
          p.length = b;
        }
        if (b !== 0)
          if (this.length > b)
            for (this.length -= b, h = 0; h < this.length; h++) this.words[h] = this.words[h + b];
          else (this.words[0] = 0), (this.length = 1);
        var x = 0;
        for (h = this.length - 1; h >= 0 && (x !== 0 || h >= g); h--) {
          var O = this.words[h] | 0;
          (this.words[h] = (x << (26 - m)) | (O >>> m)), (x = O & f);
        }
        return (
          p && x !== 0 && (p.words[p.length++] = x),
          this.length === 0 && ((this.words[0] = 0), (this.length = 1)),
          this._strip()
        );
      }),
      (a.prototype.ishrn = function (s, o, l) {
        return n(this.negative === 0), this.iushrn(s, o, l);
      }),
      (a.prototype.shln = function (s) {
        return this.clone().ishln(s);
      }),
      (a.prototype.ushln = function (s) {
        return this.clone().iushln(s);
      }),
      (a.prototype.shrn = function (s) {
        return this.clone().ishrn(s);
      }),
      (a.prototype.ushrn = function (s) {
        return this.clone().iushrn(s);
      }),
      (a.prototype.testn = function (s) {
        n(typeof s == 'number' && s >= 0);
        var o = s % 26,
          l = (s - o) / 26,
          g = 1 << o;
        if (this.length <= l) return !1;
        var m = this.words[l];
        return !!(m & g);
      }),
      (a.prototype.imaskn = function (s) {
        n(typeof s == 'number' && s >= 0);
        var o = s % 26,
          l = (s - o) / 26;
        if ((n(this.negative === 0, 'imaskn works only with positive numbers'), this.length <= l))
          return this;
        if ((o !== 0 && l++, (this.length = Math.min(l, this.length)), o !== 0)) {
          var g = 67108863 ^ ((67108863 >>> o) << o);
          this.words[this.length - 1] &= g;
        }
        return this._strip();
      }),
      (a.prototype.maskn = function (s) {
        return this.clone().imaskn(s);
      }),
      (a.prototype.iaddn = function (s) {
        return (
          n(typeof s == 'number'),
          n(s < 67108864),
          s < 0
            ? this.isubn(-s)
            : this.negative !== 0
            ? this.length === 1 && (this.words[0] | 0) <= s
              ? ((this.words[0] = s - (this.words[0] | 0)), (this.negative = 0), this)
              : ((this.negative = 0), this.isubn(s), (this.negative = 1), this)
            : this._iaddn(s)
        );
      }),
      (a.prototype._iaddn = function (s) {
        this.words[0] += s;
        for (var o = 0; o < this.length && this.words[o] >= 67108864; o++)
          (this.words[o] -= 67108864),
            o === this.length - 1 ? (this.words[o + 1] = 1) : this.words[o + 1]++;
        return (this.length = Math.max(this.length, o + 1)), this;
      }),
      (a.prototype.isubn = function (s) {
        if ((n(typeof s == 'number'), n(s < 67108864), s < 0)) return this.iaddn(-s);
        if (this.negative !== 0)
          return (this.negative = 0), this.iaddn(s), (this.negative = 1), this;
        if (((this.words[0] -= s), this.length === 1 && this.words[0] < 0))
          (this.words[0] = -this.words[0]), (this.negative = 1);
        else
          for (var o = 0; o < this.length && this.words[o] < 0; o++)
            (this.words[o] += 67108864), (this.words[o + 1] -= 1);
        return this._strip();
      }),
      (a.prototype.addn = function (s) {
        return this.clone().iaddn(s);
      }),
      (a.prototype.subn = function (s) {
        return this.clone().isubn(s);
      }),
      (a.prototype.iabs = function () {
        return (this.negative = 0), this;
      }),
      (a.prototype.abs = function () {
        return this.clone().iabs();
      }),
      (a.prototype._ishlnsubmul = function (s, o, l) {
        var g = s.length + l,
          m;
        this._expand(g);
        var b,
          f = 0;
        for (m = 0; m < s.length; m++) {
          b = (this.words[m + l] | 0) + f;
          var p = (s.words[m] | 0) * o;
          (b -= p & 67108863),
            (f = (b >> 26) - ((p / 67108864) | 0)),
            (this.words[m + l] = b & 67108863);
        }
        for (; m < this.length - l; m++)
          (b = (this.words[m + l] | 0) + f), (f = b >> 26), (this.words[m + l] = b & 67108863);
        if (f === 0) return this._strip();
        for (n(f === -1), f = 0, m = 0; m < this.length; m++)
          (b = -(this.words[m] | 0) + f), (f = b >> 26), (this.words[m] = b & 67108863);
        return (this.negative = 1), this._strip();
      }),
      (a.prototype._wordDiv = function (s, o) {
        var l = this.length - s.length,
          g = this.clone(),
          m = s,
          b = m.words[m.length - 1] | 0,
          f = this._countBits(b);
        (l = 26 - f), l !== 0 && ((m = m.ushln(l)), g.iushln(l), (b = m.words[m.length - 1] | 0));
        var p = g.length - m.length,
          h;
        if (o !== 'mod') {
          (h = new a(null)), (h.length = p + 1), (h.words = new Array(h.length));
          for (var x = 0; x < h.length; x++) h.words[x] = 0;
        }
        var O = g.clone()._ishlnsubmul(m, 1, p);
        O.negative === 0 && ((g = O), h && (h.words[p] = 1));
        for (var C = p - 1; C >= 0; C--) {
          var L = (g.words[m.length + C] | 0) * 67108864 + (g.words[m.length + C - 1] | 0);
          for (L = Math.min((L / b) | 0, 67108863), g._ishlnsubmul(m, L, C); g.negative !== 0; )
            L--, (g.negative = 0), g._ishlnsubmul(m, 1, C), g.isZero() || (g.negative ^= 1);
          h && (h.words[C] = L);
        }
        return (
          h && h._strip(),
          g._strip(),
          o !== 'div' && l !== 0 && g.iushrn(l),
          { div: h || null, mod: g }
        );
      }),
      (a.prototype.divmod = function (s, o, l) {
        if ((n(!s.isZero()), this.isZero())) return { div: new a(0), mod: new a(0) };
        var g, m, b;
        return this.negative !== 0 && s.negative === 0
          ? ((b = this.neg().divmod(s, o)),
            o !== 'mod' && (g = b.div.neg()),
            o !== 'div' && ((m = b.mod.neg()), l && m.negative !== 0 && m.iadd(s)),
            { div: g, mod: m })
          : this.negative === 0 && s.negative !== 0
          ? ((b = this.divmod(s.neg(), o)),
            o !== 'mod' && (g = b.div.neg()),
            { div: g, mod: b.mod })
          : this.negative & s.negative
          ? ((b = this.neg().divmod(s.neg(), o)),
            o !== 'div' && ((m = b.mod.neg()), l && m.negative !== 0 && m.isub(s)),
            { div: b.div, mod: m })
          : s.length > this.length || this.cmp(s) < 0
          ? { div: new a(0), mod: this }
          : s.length === 1
          ? o === 'div'
            ? { div: this.divn(s.words[0]), mod: null }
            : o === 'mod'
            ? { div: null, mod: new a(this.modrn(s.words[0])) }
            : { div: this.divn(s.words[0]), mod: new a(this.modrn(s.words[0])) }
          : this._wordDiv(s, o);
      }),
      (a.prototype.div = function (s) {
        return this.divmod(s, 'div', !1).div;
      }),
      (a.prototype.mod = function (s) {
        return this.divmod(s, 'mod', !1).mod;
      }),
      (a.prototype.umod = function (s) {
        return this.divmod(s, 'mod', !0).mod;
      }),
      (a.prototype.divRound = function (s) {
        var o = this.divmod(s);
        if (o.mod.isZero()) return o.div;
        var l = o.div.negative !== 0 ? o.mod.isub(s) : o.mod,
          g = s.ushrn(1),
          m = s.andln(1),
          b = l.cmp(g);
        return b < 0 || (m === 1 && b === 0)
          ? o.div
          : o.div.negative !== 0
          ? o.div.isubn(1)
          : o.div.iaddn(1);
      }),
      (a.prototype.modrn = function (s) {
        var o = s < 0;
        o && (s = -s), n(s <= 67108863);
        for (var l = (1 << 26) % s, g = 0, m = this.length - 1; m >= 0; m--)
          g = (l * g + (this.words[m] | 0)) % s;
        return o ? -g : g;
      }),
      (a.prototype.modn = function (s) {
        return this.modrn(s);
      }),
      (a.prototype.idivn = function (s) {
        var o = s < 0;
        o && (s = -s), n(s <= 67108863);
        for (var l = 0, g = this.length - 1; g >= 0; g--) {
          var m = (this.words[g] | 0) + l * 67108864;
          (this.words[g] = (m / s) | 0), (l = m % s);
        }
        return this._strip(), o ? this.ineg() : this;
      }),
      (a.prototype.divn = function (s) {
        return this.clone().idivn(s);
      }),
      (a.prototype.egcd = function (s) {
        n(s.negative === 0), n(!s.isZero());
        var o = this,
          l = s.clone();
        o.negative !== 0 ? (o = o.umod(s)) : (o = o.clone());
        for (
          var g = new a(1), m = new a(0), b = new a(0), f = new a(1), p = 0;
          o.isEven() && l.isEven();

        )
          o.iushrn(1), l.iushrn(1), ++p;
        for (var h = l.clone(), x = o.clone(); !o.isZero(); ) {
          for (var O = 0, C = 1; !(o.words[0] & C) && O < 26; ++O, C <<= 1);
          if (O > 0)
            for (o.iushrn(O); O-- > 0; )
              (g.isOdd() || m.isOdd()) && (g.iadd(h), m.isub(x)), g.iushrn(1), m.iushrn(1);
          for (var L = 0, D = 1; !(l.words[0] & D) && L < 26; ++L, D <<= 1);
          if (L > 0)
            for (l.iushrn(L); L-- > 0; )
              (b.isOdd() || f.isOdd()) && (b.iadd(h), f.isub(x)), b.iushrn(1), f.iushrn(1);
          o.cmp(l) >= 0 ? (o.isub(l), g.isub(b), m.isub(f)) : (l.isub(o), b.isub(g), f.isub(m));
        }
        return { a: b, b: f, gcd: l.iushln(p) };
      }),
      (a.prototype._invmp = function (s) {
        n(s.negative === 0), n(!s.isZero());
        var o = this,
          l = s.clone();
        o.negative !== 0 ? (o = o.umod(s)) : (o = o.clone());
        for (var g = new a(1), m = new a(0), b = l.clone(); o.cmpn(1) > 0 && l.cmpn(1) > 0; ) {
          for (var f = 0, p = 1; !(o.words[0] & p) && f < 26; ++f, p <<= 1);
          if (f > 0) for (o.iushrn(f); f-- > 0; ) g.isOdd() && g.iadd(b), g.iushrn(1);
          for (var h = 0, x = 1; !(l.words[0] & x) && h < 26; ++h, x <<= 1);
          if (h > 0) for (l.iushrn(h); h-- > 0; ) m.isOdd() && m.iadd(b), m.iushrn(1);
          o.cmp(l) >= 0 ? (o.isub(l), g.isub(m)) : (l.isub(o), m.isub(g));
        }
        var O;
        return o.cmpn(1) === 0 ? (O = g) : (O = m), O.cmpn(0) < 0 && O.iadd(s), O;
      }),
      (a.prototype.gcd = function (s) {
        if (this.isZero()) return s.abs();
        if (s.isZero()) return this.abs();
        var o = this.clone(),
          l = s.clone();
        (o.negative = 0), (l.negative = 0);
        for (var g = 0; o.isEven() && l.isEven(); g++) o.iushrn(1), l.iushrn(1);
        do {
          for (; o.isEven(); ) o.iushrn(1);
          for (; l.isEven(); ) l.iushrn(1);
          var m = o.cmp(l);
          if (m < 0) {
            var b = o;
            (o = l), (l = b);
          } else if (m === 0 || l.cmpn(1) === 0) break;
          o.isub(l);
        } while (!0);
        return l.iushln(g);
      }),
      (a.prototype.invm = function (s) {
        return this.egcd(s).a.umod(s);
      }),
      (a.prototype.isEven = function () {
        return (this.words[0] & 1) === 0;
      }),
      (a.prototype.isOdd = function () {
        return (this.words[0] & 1) === 1;
      }),
      (a.prototype.andln = function (s) {
        return this.words[0] & s;
      }),
      (a.prototype.bincn = function (s) {
        n(typeof s == 'number');
        var o = s % 26,
          l = (s - o) / 26,
          g = 1 << o;
        if (this.length <= l) return this._expand(l + 1), (this.words[l] |= g), this;
        for (var m = g, b = l; m !== 0 && b < this.length; b++) {
          var f = this.words[b] | 0;
          (f += m), (m = f >>> 26), (f &= 67108863), (this.words[b] = f);
        }
        return m !== 0 && ((this.words[b] = m), this.length++), this;
      }),
      (a.prototype.isZero = function () {
        return this.length === 1 && this.words[0] === 0;
      }),
      (a.prototype.cmpn = function (s) {
        var o = s < 0;
        if (this.negative !== 0 && !o) return -1;
        if (this.negative === 0 && o) return 1;
        this._strip();
        var l;
        if (this.length > 1) l = 1;
        else {
          o && (s = -s), n(s <= 67108863, 'Number is too big');
          var g = this.words[0] | 0;
          l = g === s ? 0 : g < s ? -1 : 1;
        }
        return this.negative !== 0 ? -l | 0 : l;
      }),
      (a.prototype.cmp = function (s) {
        if (this.negative !== 0 && s.negative === 0) return -1;
        if (this.negative === 0 && s.negative !== 0) return 1;
        var o = this.ucmp(s);
        return this.negative !== 0 ? -o | 0 : o;
      }),
      (a.prototype.ucmp = function (s) {
        if (this.length > s.length) return 1;
        if (this.length < s.length) return -1;
        for (var o = 0, l = this.length - 1; l >= 0; l--) {
          var g = this.words[l] | 0,
            m = s.words[l] | 0;
          if (g !== m) {
            g < m ? (o = -1) : g > m && (o = 1);
            break;
          }
        }
        return o;
      }),
      (a.prototype.gtn = function (s) {
        return this.cmpn(s) === 1;
      }),
      (a.prototype.gt = function (s) {
        return this.cmp(s) === 1;
      }),
      (a.prototype.gten = function (s) {
        return this.cmpn(s) >= 0;
      }),
      (a.prototype.gte = function (s) {
        return this.cmp(s) >= 0;
      }),
      (a.prototype.ltn = function (s) {
        return this.cmpn(s) === -1;
      }),
      (a.prototype.lt = function (s) {
        return this.cmp(s) === -1;
      }),
      (a.prototype.lten = function (s) {
        return this.cmpn(s) <= 0;
      }),
      (a.prototype.lte = function (s) {
        return this.cmp(s) <= 0;
      }),
      (a.prototype.eqn = function (s) {
        return this.cmpn(s) === 0;
      }),
      (a.prototype.eq = function (s) {
        return this.cmp(s) === 0;
      }),
      (a.red = function (s) {
        return new A(s);
      }),
      (a.prototype.toRed = function (s) {
        return (
          n(!this.red, 'Already a number in reduction context'),
          n(this.negative === 0, 'red works only with positives'),
          s.convertTo(this)._forceRed(s)
        );
      }),
      (a.prototype.fromRed = function () {
        return (
          n(this.red, 'fromRed works only with numbers in reduction context'),
          this.red.convertFrom(this)
        );
      }),
      (a.prototype._forceRed = function (s) {
        return (this.red = s), this;
      }),
      (a.prototype.forceRed = function (s) {
        return n(!this.red, 'Already a number in reduction context'), this._forceRed(s);
      }),
      (a.prototype.redAdd = function (s) {
        return n(this.red, 'redAdd works only with red numbers'), this.red.add(this, s);
      }),
      (a.prototype.redIAdd = function (s) {
        return n(this.red, 'redIAdd works only with red numbers'), this.red.iadd(this, s);
      }),
      (a.prototype.redSub = function (s) {
        return n(this.red, 'redSub works only with red numbers'), this.red.sub(this, s);
      }),
      (a.prototype.redISub = function (s) {
        return n(this.red, 'redISub works only with red numbers'), this.red.isub(this, s);
      }),
      (a.prototype.redShl = function (s) {
        return n(this.red, 'redShl works only with red numbers'), this.red.shl(this, s);
      }),
      (a.prototype.redMul = function (s) {
        return (
          n(this.red, 'redMul works only with red numbers'),
          this.red._verify2(this, s),
          this.red.mul(this, s)
        );
      }),
      (a.prototype.redIMul = function (s) {
        return (
          n(this.red, 'redMul works only with red numbers'),
          this.red._verify2(this, s),
          this.red.imul(this, s)
        );
      }),
      (a.prototype.redSqr = function () {
        return (
          n(this.red, 'redSqr works only with red numbers'),
          this.red._verify1(this),
          this.red.sqr(this)
        );
      }),
      (a.prototype.redISqr = function () {
        return (
          n(this.red, 'redISqr works only with red numbers'),
          this.red._verify1(this),
          this.red.isqr(this)
        );
      }),
      (a.prototype.redSqrt = function () {
        return (
          n(this.red, 'redSqrt works only with red numbers'),
          this.red._verify1(this),
          this.red.sqrt(this)
        );
      }),
      (a.prototype.redInvm = function () {
        return (
          n(this.red, 'redInvm works only with red numbers'),
          this.red._verify1(this),
          this.red.invm(this)
        );
      }),
      (a.prototype.redNeg = function () {
        return (
          n(this.red, 'redNeg works only with red numbers'),
          this.red._verify1(this),
          this.red.neg(this)
        );
      }),
      (a.prototype.redPow = function (s) {
        return (
          n(this.red && !s.red, 'redPow(normalNum)'), this.red._verify1(this), this.red.pow(this, s)
        );
      });
    var H = { k256: null, p224: null, p192: null, p25519: null };
    function F(u, s) {
      (this.name = u),
        (this.p = new a(s, 16)),
        (this.n = this.p.bitLength()),
        (this.k = new a(1).iushln(this.n).isub(this.p)),
        (this.tmp = this._tmp());
    }
    (F.prototype._tmp = function () {
      var s = new a(null);
      return (s.words = new Array(Math.ceil(this.n / 13))), s;
    }),
      (F.prototype.ireduce = function (s) {
        var o = s,
          l;
        do
          this.split(o, this.tmp), (o = this.imulK(o)), (o = o.iadd(this.tmp)), (l = o.bitLength());
        while (l > this.n);
        var g = l < this.n ? -1 : o.ucmp(this.p);
        return (
          g === 0
            ? ((o.words[0] = 0), (o.length = 1))
            : g > 0
            ? o.isub(this.p)
            : o.strip !== void 0
            ? o.strip()
            : o._strip(),
          o
        );
      }),
      (F.prototype.split = function (s, o) {
        s.iushrn(this.n, 0, o);
      }),
      (F.prototype.imulK = function (s) {
        return s.imul(this.k);
      });
    function J() {
      F.call(
        this,
        'k256',
        'ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff fffffffe fffffc2f'
      );
    }
    i(J, F),
      (J.prototype.split = function (s, o) {
        for (var l = 4194303, g = Math.min(s.length, 9), m = 0; m < g; m++) o.words[m] = s.words[m];
        if (((o.length = g), s.length <= 9)) {
          (s.words[0] = 0), (s.length = 1);
          return;
        }
        var b = s.words[9];
        for (o.words[o.length++] = b & l, m = 10; m < s.length; m++) {
          var f = s.words[m] | 0;
          (s.words[m - 10] = ((f & l) << 4) | (b >>> 22)), (b = f);
        }
        (b >>>= 22),
          (s.words[m - 10] = b),
          b === 0 && s.length > 10 ? (s.length -= 10) : (s.length -= 9);
      }),
      (J.prototype.imulK = function (s) {
        (s.words[s.length] = 0), (s.words[s.length + 1] = 0), (s.length += 2);
        for (var o = 0, l = 0; l < s.length; l++) {
          var g = s.words[l] | 0;
          (o += g * 977), (s.words[l] = o & 67108863), (o = g * 64 + ((o / 67108864) | 0));
        }
        return (
          s.words[s.length - 1] === 0 && (s.length--, s.words[s.length - 1] === 0 && s.length--), s
        );
      });
    function X() {
      F.call(this, 'p224', 'ffffffff ffffffff ffffffff ffffffff 00000000 00000000 00000001');
    }
    i(X, F);
    function W() {
      F.call(this, 'p192', 'ffffffff ffffffff ffffffff fffffffe ffffffff ffffffff');
    }
    i(W, F);
    function G() {
      F.call(this, '25519', '7fffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffed');
    }
    i(G, F),
      (G.prototype.imulK = function (s) {
        for (var o = 0, l = 0; l < s.length; l++) {
          var g = (s.words[l] | 0) * 19 + o,
            m = g & 67108863;
          (g >>>= 26), (s.words[l] = m), (o = g);
        }
        return o !== 0 && (s.words[s.length++] = o), s;
      }),
      (a._prime = function (s) {
        if (H[s]) return H[s];
        var o;
        if (s === 'k256') o = new J();
        else if (s === 'p224') o = new X();
        else if (s === 'p192') o = new W();
        else if (s === 'p25519') o = new G();
        else throw new Error('Unknown prime ' + s);
        return (H[s] = o), o;
      });
    function A(u) {
      if (typeof u == 'string') {
        var s = a._prime(u);
        (this.m = s.p), (this.prime = s);
      } else n(u.gtn(1), 'modulus must be greater than 1'), (this.m = u), (this.prime = null);
    }
    (A.prototype._verify1 = function (s) {
      n(s.negative === 0, 'red works only with positives'),
        n(s.red, 'red works only with red numbers');
    }),
      (A.prototype._verify2 = function (s, o) {
        n((s.negative | o.negative) === 0, 'red works only with positives'),
          n(s.red && s.red === o.red, 'red works only with red numbers');
      }),
      (A.prototype.imod = function (s) {
        return this.prime
          ? this.prime.ireduce(s)._forceRed(this)
          : (M(s, s.umod(this.m)._forceRed(this)), s);
      }),
      (A.prototype.neg = function (s) {
        return s.isZero() ? s.clone() : this.m.sub(s)._forceRed(this);
      }),
      (A.prototype.add = function (s, o) {
        this._verify2(s, o);
        var l = s.add(o);
        return l.cmp(this.m) >= 0 && l.isub(this.m), l._forceRed(this);
      }),
      (A.prototype.iadd = function (s, o) {
        this._verify2(s, o);
        var l = s.iadd(o);
        return l.cmp(this.m) >= 0 && l.isub(this.m), l;
      }),
      (A.prototype.sub = function (s, o) {
        this._verify2(s, o);
        var l = s.sub(o);
        return l.cmpn(0) < 0 && l.iadd(this.m), l._forceRed(this);
      }),
      (A.prototype.isub = function (s, o) {
        this._verify2(s, o);
        var l = s.isub(o);
        return l.cmpn(0) < 0 && l.iadd(this.m), l;
      }),
      (A.prototype.shl = function (s, o) {
        return this._verify1(s), this.imod(s.ushln(o));
      }),
      (A.prototype.imul = function (s, o) {
        return this._verify2(s, o), this.imod(s.imul(o));
      }),
      (A.prototype.mul = function (s, o) {
        return this._verify2(s, o), this.imod(s.mul(o));
      }),
      (A.prototype.isqr = function (s) {
        return this.imul(s, s.clone());
      }),
      (A.prototype.sqr = function (s) {
        return this.mul(s, s);
      }),
      (A.prototype.sqrt = function (s) {
        if (s.isZero()) return s.clone();
        var o = this.m.andln(3);
        if ((n(o % 2 === 1), o === 3)) {
          var l = this.m.add(new a(1)).iushrn(2);
          return this.pow(s, l);
        }
        for (var g = this.m.subn(1), m = 0; !g.isZero() && g.andln(1) === 0; ) m++, g.iushrn(1);
        n(!g.isZero());
        var b = new a(1).toRed(this),
          f = b.redNeg(),
          p = this.m.subn(1).iushrn(1),
          h = this.m.bitLength();
        for (h = new a(2 * h * h).toRed(this); this.pow(h, p).cmp(f) !== 0; ) h.redIAdd(f);
        for (
          var x = this.pow(h, g), O = this.pow(s, g.addn(1).iushrn(1)), C = this.pow(s, g), L = m;
          C.cmp(b) !== 0;

        ) {
          for (var D = C, Z = 0; D.cmp(b) !== 0; Z++) D = D.redSqr();
          n(Z < L);
          var j = this.pow(x, new a(1).iushln(L - Z - 1));
          (O = O.redMul(j)), (x = j.redSqr()), (C = C.redMul(x)), (L = Z);
        }
        return O;
      }),
      (A.prototype.invm = function (s) {
        var o = s._invmp(this.m);
        return o.negative !== 0 ? ((o.negative = 0), this.imod(o).redNeg()) : this.imod(o);
      }),
      (A.prototype.pow = function (s, o) {
        if (o.isZero()) return new a(1).toRed(this);
        if (o.cmpn(1) === 0) return s.clone();
        var l = 4,
          g = new Array(1 << l);
        (g[0] = new a(1).toRed(this)), (g[1] = s);
        for (var m = 2; m < g.length; m++) g[m] = this.mul(g[m - 1], s);
        var b = g[0],
          f = 0,
          p = 0,
          h = o.bitLength() % 26;
        for (h === 0 && (h = 26), m = o.length - 1; m >= 0; m--) {
          for (var x = o.words[m], O = h - 1; O >= 0; O--) {
            var C = (x >> O) & 1;
            if ((b !== g[0] && (b = this.sqr(b)), C === 0 && f === 0)) {
              p = 0;
              continue;
            }
            (f <<= 1),
              (f |= C),
              p++,
              !(p !== l && (m !== 0 || O !== 0)) && ((b = this.mul(b, g[f])), (p = 0), (f = 0));
          }
          h = 26;
        }
        return b;
      }),
      (A.prototype.convertTo = function (s) {
        var o = s.umod(this.m);
        return o === s ? o.clone() : o;
      }),
      (A.prototype.convertFrom = function (s) {
        var o = s.clone();
        return (o.red = null), o;
      }),
      (a.mont = function (s) {
        return new d(s);
      });
    function d(u) {
      A.call(this, u),
        (this.shift = this.m.bitLength()),
        this.shift % 26 !== 0 && (this.shift += 26 - (this.shift % 26)),
        (this.r = new a(1).iushln(this.shift)),
        (this.r2 = this.imod(this.r.sqr())),
        (this.rinv = this.r._invmp(this.m)),
        (this.minv = this.rinv.mul(this.r).isubn(1).div(this.m)),
        (this.minv = this.minv.umod(this.r)),
        (this.minv = this.r.sub(this.minv));
    }
    i(d, A),
      (d.prototype.convertTo = function (s) {
        return this.imod(s.ushln(this.shift));
      }),
      (d.prototype.convertFrom = function (s) {
        var o = this.imod(s.mul(this.rinv));
        return (o.red = null), o;
      }),
      (d.prototype.imul = function (s, o) {
        if (s.isZero() || o.isZero()) return (s.words[0] = 0), (s.length = 1), s;
        var l = s.imul(o),
          g = l.maskn(this.shift).mul(this.minv).imaskn(this.shift).mul(this.m),
          m = l.isub(g).iushrn(this.shift),
          b = m;
        return (
          m.cmp(this.m) >= 0 ? (b = m.isub(this.m)) : m.cmpn(0) < 0 && (b = m.iadd(this.m)),
          b._forceRed(this)
        );
      }),
      (d.prototype.mul = function (s, o) {
        if (s.isZero() || o.isZero()) return new a(0)._forceRed(this);
        var l = s.mul(o),
          g = l.maskn(this.shift).mul(this.minv).imaskn(this.shift).mul(this.m),
          m = l.isub(g).iushrn(this.shift),
          b = m;
        return (
          m.cmp(this.m) >= 0 ? (b = m.isub(this.m)) : m.cmpn(0) < 0 && (b = m.iadd(this.m)),
          b._forceRed(this)
        );
      }),
      (d.prototype.invm = function (s) {
        var o = this.imod(s._invmp(this.m).mul(this.r2));
        return o._forceRed(this);
      });
  })(r, ie);
})(Uo);
const oi = Uo.exports;
function rc(r, e) {
  let { precision: t = 9, minPrecision: n = 3 } = e || {},
    [i = '0', a = '0'] = String(r || '0.0').split('.'),
    c = /(\d)(?=(\d{3})+\b)/g,
    v = i.replace(c, '$1,'),
    w = a.slice(0, t);
  if (n < t) {
    let M = w.match(/.*[1-9]{1}/),
      S = M?.[0].length || 0,
      I = Math.max(n, S);
    w = w.slice(0, I);
  }
  let y = w ? `.${w}` : '';
  return `${v}${y}`;
}
var xt = class extends oi {
    constructor(e, t, n) {
      if (xt.isBN(e)) {
        super(e.toArray(), t, n);
        return;
      }
      if (typeof e == 'string' && e.slice(0, 2) === '0x') {
        super(e.substring(2), t || 'hex', n);
        return;
      }
      let i = e ?? 0;
      super(i, t, n);
    }
    toString(e, t) {
      let n = super.toString(e, t);
      return e === 16 || e === 'hex' ? `0x${n}` : n;
    }
    toHex(e) {
      let t = (e || 0) * 2;
      if (this.isNeg()) throw new Error('cannot convert negative value to hex');
      if (e && this.byteLength() > e) throw new Error(`value ${this} exceeds bytes ${e}`);
      return this.toString(16, t);
    }
    toBytes(e) {
      if (this.isNeg()) throw new Error('cannot convert negative value to Bytes');
      return Uint8Array.from(this.toArray(void 0, e));
    }
    toJSON() {
      return this.toString(16);
    }
    valueOf() {
      return this.toString();
    }
    format(e) {
      let { units: t = 9, precision: n = 9, minPrecision: i = 3 } = e || {},
        a = this.formatUnits(t),
        c = rc(a, { precision: n, minPrecision: i });
      if (!parseFloat(c)) {
        let [, v = '0'] = a.split('.'),
          w = v.match(/[1-9]/);
        if (w && w.index && w.index + 1 > n) {
          let [y = '0'] = c.split('.');
          return `${y}.${v.slice(0, w.index + 1)}`;
        }
      }
      return c;
    }
    formatUnits(e = 9) {
      let t = this.toString().slice(0, e * -1),
        n = this.toString().slice(e * -1),
        i = n.length,
        a = Array.from({ length: e - i })
          .fill('0')
          .join('');
      return `${t ? `${t}.` : '0.'}${a}${n}`;
    }
    add(e) {
      return this.caller(e, 'add');
    }
    pow(e) {
      return this.caller(e, 'pow');
    }
    sub(e) {
      return this.caller(e, 'sub');
    }
    div(e) {
      return this.caller(e, 'div');
    }
    mul(e) {
      return this.caller(e, 'mul');
    }
    mod(e) {
      return this.caller(e, 'mod');
    }
    divRound(e) {
      return this.caller(e, 'divRound');
    }
    lt(e) {
      return this.caller(e, 'lt');
    }
    lte(e) {
      return this.caller(e, 'lte');
    }
    gt(e) {
      return this.caller(e, 'gt');
    }
    gte(e) {
      return this.caller(e, 'gte');
    }
    eq(e) {
      return this.caller(e, 'eq');
    }
    cmp(e) {
      return this.caller(e, 'cmp');
    }
    sqr() {
      return new xt(super.sqr().toArray());
    }
    neg() {
      return new xt(super.neg().toArray());
    }
    abs() {
      return new xt(super.abs().toArray());
    }
    toTwos(e) {
      return new xt(super.toTwos(e).toArray());
    }
    fromTwos(e) {
      return new xt(super.fromTwos(e).toArray());
    }
    caller(e, t) {
      let n = super[t](new xt(e));
      return xt.isBN(n) ? new xt(n.toArray()) : n;
    }
    clone() {
      return new xt(this.toArray());
    }
    mulTo(e, t) {
      let n = new oi(this.toArray()).mulTo(e, t);
      return new xt(n.toArray());
    }
    egcd(e) {
      let { a: t, b: n, gcd: i } = new oi(this.toArray()).egcd(e);
      return { a: new xt(t.toArray()), b: new xt(n.toArray()), gcd: new xt(i.toArray()) };
    }
    divmod(e, t, n) {
      let { div: i, mod: a } = new oi(this.toArray()).divmod(new xt(e), t, n);
      return { div: new xt(i?.toArray()), mod: new xt(a?.toArray()) };
    }
  },
  B = (r, e, t) => new xt(r, e, t);
B.parseUnits = (r, e = 9) => {
  let t = r === '.' ? '0.' : r,
    [n = '0', i = '0'] = t.split('.'),
    a = i.length;
  if (a > e) throw new Error("Decimal can't be bigger than the units");
  let c = Array.from({ length: e }).fill('0');
  c.splice(0, a, i);
  let v = `${n.replace(',', '')}${c.join('')}`;
  return B(v);
};
function nr(r) {
  return B(r).toNumber();
}
function Za(r, e) {
  return B(r).toHex(e);
}
function or(r, e) {
  return B(r).toBytes(e);
}
function nc(...r) {
  return r.reduce((e, t) => (B(t).gt(e) ? B(t) : e), B(0));
}
function ic(...r) {
  return B(Math.ceil(r.reduce((e, t) => B(e).mul(t), B(1)).toNumber()));
}
const ac = 'strings/5.7.0',
  zo = new Te(ac);
var _i;
(function (r) {
  (r.current = ''), (r.NFC = 'NFC'), (r.NFD = 'NFD'), (r.NFKC = 'NFKC'), (r.NFKD = 'NFKD');
})(_i || (_i = {}));
var Ft;
(function (r) {
  (r.UNEXPECTED_CONTINUE = 'unexpected continuation byte'),
    (r.BAD_PREFIX = 'bad codepoint prefix'),
    (r.OVERRUN = 'string overrun'),
    (r.MISSING_CONTINUE = 'missing continuation byte'),
    (r.OUT_OF_RANGE = 'out of UTF-8 range'),
    (r.UTF16_SURROGATE = 'UTF-16 surrogate'),
    (r.OVERLONG = 'overlong representation');
})(Ft || (Ft = {}));
function sc(r, e, t, n, i) {
  return zo.throwArgumentError(`invalid codepoint at offset ${e}; ${r}`, 'bytes', t);
}
function jo(r, e, t, n, i) {
  if (r === Ft.BAD_PREFIX || r === Ft.UNEXPECTED_CONTINUE) {
    let a = 0;
    for (let c = e + 1; c < t.length && t[c] >> 6 === 2; c++) a++;
    return a;
  }
  return r === Ft.OVERRUN ? t.length - e - 1 : 0;
}
function oc(r, e, t, n, i) {
  return r === Ft.OVERLONG ? (n.push(i), 0) : (n.push(65533), jo(r, e, t));
}
const fc = Object.freeze({ error: sc, ignore: jo, replace: oc });
function cc(r, e) {
  e == null && (e = fc.error), (r = V(r));
  const t = [];
  let n = 0;
  for (; n < r.length; ) {
    const i = r[n++];
    if (!(i >> 7)) {
      t.push(i);
      continue;
    }
    let a = null,
      c = null;
    if ((i & 224) === 192) (a = 1), (c = 127);
    else if ((i & 240) === 224) (a = 2), (c = 2047);
    else if ((i & 248) === 240) (a = 3), (c = 65535);
    else {
      (i & 192) === 128
        ? (n += e(Ft.UNEXPECTED_CONTINUE, n - 1, r, t))
        : (n += e(Ft.BAD_PREFIX, n - 1, r, t));
      continue;
    }
    if (n - 1 + a >= r.length) {
      n += e(Ft.OVERRUN, n - 1, r, t);
      continue;
    }
    let v = i & ((1 << (8 - a - 1)) - 1);
    for (let w = 0; w < a; w++) {
      let y = r[n];
      if ((y & 192) != 128) {
        (n += e(Ft.MISSING_CONTINUE, n, r, t)), (v = null);
        break;
      }
      (v = (v << 6) | (y & 63)), n++;
    }
    if (v !== null) {
      if (v > 1114111) {
        n += e(Ft.OUT_OF_RANGE, n - 1 - a, r, t, v);
        continue;
      }
      if (v >= 55296 && v <= 57343) {
        n += e(Ft.UTF16_SURROGATE, n - 1 - a, r, t, v);
        continue;
      }
      if (v <= c) {
        n += e(Ft.OVERLONG, n - 1 - a, r, t, v);
        continue;
      }
      t.push(v);
    }
  }
  return t;
}
function uc(r, e = _i.current) {
  e != _i.current && (zo.checkNormalize(), (r = r.normalize(e)));
  let t = [];
  for (let n = 0; n < r.length; n++) {
    const i = r.charCodeAt(n);
    if (i < 128) t.push(i);
    else if (i < 2048) t.push((i >> 6) | 192), t.push((i & 63) | 128);
    else if ((i & 64512) == 55296) {
      n++;
      const a = r.charCodeAt(n);
      if (n >= r.length || (a & 64512) !== 56320) throw new Error('invalid utf-8 string');
      const c = 65536 + ((i & 1023) << 10) + (a & 1023);
      t.push((c >> 18) | 240),
        t.push(((c >> 12) & 63) | 128),
        t.push(((c >> 6) & 63) | 128),
        t.push((c & 63) | 128);
    } else t.push((i >> 12) | 224), t.push(((i >> 6) & 63) | 128), t.push((i & 63) | 128);
  }
  return V(t);
}
function dc(r) {
  return r
    .map((e) =>
      e <= 65535
        ? String.fromCharCode(e)
        : ((e -= 65536), String.fromCharCode(((e >> 10) & 1023) + 55296, (e & 1023) + 56320))
    )
    .join('');
}
function hc(r, e) {
  return dc(cc(r, e));
}
const lc = 'properties/5.7.0';
globalThis && globalThis.__awaiter;
new Te(lc);
function fi(r, e, t) {
  Object.defineProperty(r, e, { enumerable: !0, value: t, writable: !1 });
}
var $r = {},
  Oe = {},
  Xr = Vo;
function Vo(r, e) {
  if (!r) throw new Error(e || 'Assertion failed');
}
Vo.equal = function (e, t, n) {
  if (e != t) throw new Error(n || 'Assertion failed: ' + e + ' != ' + t);
};
var hn = { exports: {} };
typeof Object.create == 'function'
  ? (hn.exports = function (e, t) {
      t &&
        ((e.super_ = t),
        (e.prototype = Object.create(t.prototype, {
          constructor: { value: e, enumerable: !1, writable: !0, configurable: !0 },
        })));
    })
  : (hn.exports = function (e, t) {
      if (t) {
        e.super_ = t;
        var n = function () {};
        (n.prototype = t.prototype), (e.prototype = new n()), (e.prototype.constructor = e);
      }
    });
var pc = Xr,
  vc = hn.exports;
Oe.inherits = vc;
function bc(r, e) {
  return (r.charCodeAt(e) & 64512) !== 55296 || e < 0 || e + 1 >= r.length
    ? !1
    : (r.charCodeAt(e + 1) & 64512) === 56320;
}
function mc(r, e) {
  if (Array.isArray(r)) return r.slice();
  if (!r) return [];
  var t = [];
  if (typeof r == 'string')
    if (e) {
      if (e === 'hex')
        for (
          r = r.replace(/[^a-z0-9]+/gi, ''), r.length % 2 !== 0 && (r = '0' + r), i = 0;
          i < r.length;
          i += 2
        )
          t.push(parseInt(r[i] + r[i + 1], 16));
    } else
      for (var n = 0, i = 0; i < r.length; i++) {
        var a = r.charCodeAt(i);
        a < 128
          ? (t[n++] = a)
          : a < 2048
          ? ((t[n++] = (a >> 6) | 192), (t[n++] = (a & 63) | 128))
          : bc(r, i)
          ? ((a = 65536 + ((a & 1023) << 10) + (r.charCodeAt(++i) & 1023)),
            (t[n++] = (a >> 18) | 240),
            (t[n++] = ((a >> 12) & 63) | 128),
            (t[n++] = ((a >> 6) & 63) | 128),
            (t[n++] = (a & 63) | 128))
          : ((t[n++] = (a >> 12) | 224),
            (t[n++] = ((a >> 6) & 63) | 128),
            (t[n++] = (a & 63) | 128));
      }
  else for (i = 0; i < r.length; i++) t[i] = r[i] | 0;
  return t;
}
Oe.toArray = mc;
function gc(r) {
  for (var e = '', t = 0; t < r.length; t++) e += Ho(r[t].toString(16));
  return e;
}
Oe.toHex = gc;
function Jo(r) {
  var e = (r >>> 24) | ((r >>> 8) & 65280) | ((r << 8) & 16711680) | ((r & 255) << 24);
  return e >>> 0;
}
Oe.htonl = Jo;
function yc(r, e) {
  for (var t = '', n = 0; n < r.length; n++) {
    var i = r[n];
    e === 'little' && (i = Jo(i)), (t += Wo(i.toString(16)));
  }
  return t;
}
Oe.toHex32 = yc;
function Ho(r) {
  return r.length === 1 ? '0' + r : r;
}
Oe.zero2 = Ho;
function Wo(r) {
  return r.length === 7
    ? '0' + r
    : r.length === 6
    ? '00' + r
    : r.length === 5
    ? '000' + r
    : r.length === 4
    ? '0000' + r
    : r.length === 3
    ? '00000' + r
    : r.length === 2
    ? '000000' + r
    : r.length === 1
    ? '0000000' + r
    : r;
}
Oe.zero8 = Wo;
function wc(r, e, t, n) {
  var i = t - e;
  pc(i % 4 === 0);
  for (var a = new Array(i / 4), c = 0, v = e; c < a.length; c++, v += 4) {
    var w;
    n === 'big'
      ? (w = (r[v] << 24) | (r[v + 1] << 16) | (r[v + 2] << 8) | r[v + 3])
      : (w = (r[v + 3] << 24) | (r[v + 2] << 16) | (r[v + 1] << 8) | r[v]),
      (a[c] = w >>> 0);
  }
  return a;
}
Oe.join32 = wc;
function xc(r, e) {
  for (var t = new Array(r.length * 4), n = 0, i = 0; n < r.length; n++, i += 4) {
    var a = r[n];
    e === 'big'
      ? ((t[i] = a >>> 24),
        (t[i + 1] = (a >>> 16) & 255),
        (t[i + 2] = (a >>> 8) & 255),
        (t[i + 3] = a & 255))
      : ((t[i + 3] = a >>> 24),
        (t[i + 2] = (a >>> 16) & 255),
        (t[i + 1] = (a >>> 8) & 255),
        (t[i] = a & 255));
  }
  return t;
}
Oe.split32 = xc;
function Mc(r, e) {
  return (r >>> e) | (r << (32 - e));
}
Oe.rotr32 = Mc;
function _c(r, e) {
  return (r << e) | (r >>> (32 - e));
}
Oe.rotl32 = _c;
function Sc(r, e) {
  return (r + e) >>> 0;
}
Oe.sum32 = Sc;
function Ac(r, e, t) {
  return (r + e + t) >>> 0;
}
Oe.sum32_3 = Ac;
function Ec(r, e, t, n) {
  return (r + e + t + n) >>> 0;
}
Oe.sum32_4 = Ec;
function Ic(r, e, t, n, i) {
  return (r + e + t + n + i) >>> 0;
}
Oe.sum32_5 = Ic;
function Rc(r, e, t, n) {
  var i = r[e],
    a = r[e + 1],
    c = (n + a) >>> 0,
    v = (c < n ? 1 : 0) + t + i;
  (r[e] = v >>> 0), (r[e + 1] = c);
}
Oe.sum64 = Rc;
function Nc(r, e, t, n) {
  var i = (e + n) >>> 0,
    a = (i < e ? 1 : 0) + r + t;
  return a >>> 0;
}
Oe.sum64_hi = Nc;
function Oc(r, e, t, n) {
  var i = e + n;
  return i >>> 0;
}
Oe.sum64_lo = Oc;
function Tc(r, e, t, n, i, a, c, v) {
  var w = 0,
    y = e;
  (y = (y + n) >>> 0),
    (w += y < e ? 1 : 0),
    (y = (y + a) >>> 0),
    (w += y < a ? 1 : 0),
    (y = (y + v) >>> 0),
    (w += y < v ? 1 : 0);
  var M = r + t + i + c + w;
  return M >>> 0;
}
Oe.sum64_4_hi = Tc;
function Cc(r, e, t, n, i, a, c, v) {
  var w = e + n + a + v;
  return w >>> 0;
}
Oe.sum64_4_lo = Cc;
function Pc(r, e, t, n, i, a, c, v, w, y) {
  var M = 0,
    S = e;
  (S = (S + n) >>> 0),
    (M += S < e ? 1 : 0),
    (S = (S + a) >>> 0),
    (M += S < a ? 1 : 0),
    (S = (S + v) >>> 0),
    (M += S < v ? 1 : 0),
    (S = (S + y) >>> 0),
    (M += S < y ? 1 : 0);
  var I = r + t + i + c + w + M;
  return I >>> 0;
}
Oe.sum64_5_hi = Pc;
function kc(r, e, t, n, i, a, c, v, w, y) {
  var M = e + n + a + v + y;
  return M >>> 0;
}
Oe.sum64_5_lo = kc;
function $c(r, e, t) {
  var n = (e << (32 - t)) | (r >>> t);
  return n >>> 0;
}
Oe.rotr64_hi = $c;
function Dc(r, e, t) {
  var n = (r << (32 - t)) | (e >>> t);
  return n >>> 0;
}
Oe.rotr64_lo = Dc;
function Lc(r, e, t) {
  return r >>> t;
}
Oe.shr64_hi = Lc;
function qc(r, e, t) {
  var n = (r << (32 - t)) | (e >>> t);
  return n >>> 0;
}
Oe.shr64_lo = qc;
var _n = {},
  Os = Oe,
  Bc = Xr;
function Ui() {
  (this.pending = null),
    (this.pendingTotal = 0),
    (this.blockSize = this.constructor.blockSize),
    (this.outSize = this.constructor.outSize),
    (this.hmacStrength = this.constructor.hmacStrength),
    (this.padLength = this.constructor.padLength / 8),
    (this.endian = 'big'),
    (this._delta8 = this.blockSize / 8),
    (this._delta32 = this.blockSize / 32);
}
_n.BlockHash = Ui;
Ui.prototype.update = function (e, t) {
  if (
    ((e = Os.toArray(e, t)),
    this.pending ? (this.pending = this.pending.concat(e)) : (this.pending = e),
    (this.pendingTotal += e.length),
    this.pending.length >= this._delta8)
  ) {
    e = this.pending;
    var n = e.length % this._delta8;
    (this.pending = e.slice(e.length - n, e.length)),
      this.pending.length === 0 && (this.pending = null),
      (e = Os.join32(e, 0, e.length - n, this.endian));
    for (var i = 0; i < e.length; i += this._delta32) this._update(e, i, i + this._delta32);
  }
  return this;
};
Ui.prototype.digest = function (e) {
  return this.update(this._pad()), Bc(this.pending === null), this._digest(e);
};
Ui.prototype._pad = function () {
  var e = this.pendingTotal,
    t = this._delta8,
    n = t - ((e + this.padLength) % t),
    i = new Array(n + this.padLength);
  i[0] = 128;
  for (var a = 1; a < n; a++) i[a] = 0;
  if (((e <<= 3), this.endian === 'big')) {
    for (var c = 8; c < this.padLength; c++) i[a++] = 0;
    (i[a++] = 0),
      (i[a++] = 0),
      (i[a++] = 0),
      (i[a++] = 0),
      (i[a++] = (e >>> 24) & 255),
      (i[a++] = (e >>> 16) & 255),
      (i[a++] = (e >>> 8) & 255),
      (i[a++] = e & 255);
  } else
    for (
      i[a++] = e & 255,
        i[a++] = (e >>> 8) & 255,
        i[a++] = (e >>> 16) & 255,
        i[a++] = (e >>> 24) & 255,
        i[a++] = 0,
        i[a++] = 0,
        i[a++] = 0,
        i[a++] = 0,
        c = 8;
      c < this.padLength;
      c++
    )
      i[a++] = 0;
  return i;
};
var Sn = {},
  dr = {},
  Fc = Oe,
  ir = Fc.rotr32;
function Uc(r, e, t, n) {
  if (r === 0) return Go(e, t, n);
  if (r === 1 || r === 3) return Xo(e, t, n);
  if (r === 2) return Ko(e, t, n);
}
dr.ft_1 = Uc;
function Go(r, e, t) {
  return (r & e) ^ (~r & t);
}
dr.ch32 = Go;
function Ko(r, e, t) {
  return (r & e) ^ (r & t) ^ (e & t);
}
dr.maj32 = Ko;
function Xo(r, e, t) {
  return r ^ e ^ t;
}
dr.p32 = Xo;
function zc(r) {
  return ir(r, 2) ^ ir(r, 13) ^ ir(r, 22);
}
dr.s0_256 = zc;
function jc(r) {
  return ir(r, 6) ^ ir(r, 11) ^ ir(r, 25);
}
dr.s1_256 = jc;
function Vc(r) {
  return ir(r, 7) ^ ir(r, 18) ^ (r >>> 3);
}
dr.g0_256 = Vc;
function Jc(r) {
  return ir(r, 17) ^ ir(r, 19) ^ (r >>> 10);
}
dr.g1_256 = Jc;
var ln = Oe,
  Hc = _n,
  Wc = dr,
  pa = ln.rotl32,
  Cn = ln.sum32,
  Gc = ln.sum32_5,
  Kc = Wc.ft_1,
  Zo = Hc.BlockHash,
  Xc = [1518500249, 1859775393, 2400959708, 3395469782];
function fr() {
  if (!(this instanceof fr)) return new fr();
  Zo.call(this),
    (this.h = [1732584193, 4023233417, 2562383102, 271733878, 3285377520]),
    (this.W = new Array(80));
}
ln.inherits(fr, Zo);
var Zc = fr;
fr.blockSize = 512;
fr.outSize = 160;
fr.hmacStrength = 80;
fr.padLength = 64;
fr.prototype._update = function (e, t) {
  for (var n = this.W, i = 0; i < 16; i++) n[i] = e[t + i];
  for (; i < n.length; i++) n[i] = pa(n[i - 3] ^ n[i - 8] ^ n[i - 14] ^ n[i - 16], 1);
  var a = this.h[0],
    c = this.h[1],
    v = this.h[2],
    w = this.h[3],
    y = this.h[4];
  for (i = 0; i < n.length; i++) {
    var M = ~~(i / 20),
      S = Gc(pa(a, 5), Kc(M, c, v, w), y, n[i], Xc[M]);
    (y = w), (w = v), (v = pa(c, 30)), (c = a), (a = S);
  }
  (this.h[0] = Cn(this.h[0], a)),
    (this.h[1] = Cn(this.h[1], c)),
    (this.h[2] = Cn(this.h[2], v)),
    (this.h[3] = Cn(this.h[3], w)),
    (this.h[4] = Cn(this.h[4], y));
};
fr.prototype._digest = function (e) {
  return e === 'hex' ? ln.toHex32(this.h, 'big') : ln.split32(this.h, 'big');
};
var pn = Oe,
  Yc = _n,
  An = dr,
  Qc = Xr,
  Zt = pn.sum32,
  eu = pn.sum32_4,
  tu = pn.sum32_5,
  ru = An.ch32,
  nu = An.maj32,
  iu = An.s0_256,
  au = An.s1_256,
  su = An.g0_256,
  ou = An.g1_256,
  Yo = Yc.BlockHash,
  fu = [
    1116352408, 1899447441, 3049323471, 3921009573, 961987163, 1508970993, 2453635748, 2870763221,
    3624381080, 310598401, 607225278, 1426881987, 1925078388, 2162078206, 2614888103, 3248222580,
    3835390401, 4022224774, 264347078, 604807628, 770255983, 1249150122, 1555081692, 1996064986,
    2554220882, 2821834349, 2952996808, 3210313671, 3336571891, 3584528711, 113926993, 338241895,
    666307205, 773529912, 1294757372, 1396182291, 1695183700, 1986661051, 2177026350, 2456956037,
    2730485921, 2820302411, 3259730800, 3345764771, 3516065817, 3600352804, 4094571909, 275423344,
    430227734, 506948616, 659060556, 883997877, 958139571, 1322822218, 1537002063, 1747873779,
    1955562222, 2024104815, 2227730452, 2361852424, 2428436474, 2756734187, 3204031479, 3329325298,
  ];
function cr() {
  if (!(this instanceof cr)) return new cr();
  Yo.call(this),
    (this.h = [
      1779033703, 3144134277, 1013904242, 2773480762, 1359893119, 2600822924, 528734635, 1541459225,
    ]),
    (this.k = fu),
    (this.W = new Array(64));
}
pn.inherits(cr, Yo);
var Qo = cr;
cr.blockSize = 512;
cr.outSize = 256;
cr.hmacStrength = 192;
cr.padLength = 64;
cr.prototype._update = function (e, t) {
  for (var n = this.W, i = 0; i < 16; i++) n[i] = e[t + i];
  for (; i < n.length; i++) n[i] = eu(ou(n[i - 2]), n[i - 7], su(n[i - 15]), n[i - 16]);
  var a = this.h[0],
    c = this.h[1],
    v = this.h[2],
    w = this.h[3],
    y = this.h[4],
    M = this.h[5],
    S = this.h[6],
    I = this.h[7];
  for (Qc(this.k.length === n.length), i = 0; i < n.length; i++) {
    var E = tu(I, au(y), ru(y, M, S), this.k[i], n[i]),
      R = Zt(iu(a), nu(a, c, v));
    (I = S), (S = M), (M = y), (y = Zt(w, E)), (w = v), (v = c), (c = a), (a = Zt(E, R));
  }
  (this.h[0] = Zt(this.h[0], a)),
    (this.h[1] = Zt(this.h[1], c)),
    (this.h[2] = Zt(this.h[2], v)),
    (this.h[3] = Zt(this.h[3], w)),
    (this.h[4] = Zt(this.h[4], y)),
    (this.h[5] = Zt(this.h[5], M)),
    (this.h[6] = Zt(this.h[6], S)),
    (this.h[7] = Zt(this.h[7], I));
};
cr.prototype._digest = function (e) {
  return e === 'hex' ? pn.toHex32(this.h, 'big') : pn.split32(this.h, 'big');
};
var ka = Oe,
  e0 = Qo;
function Nr() {
  if (!(this instanceof Nr)) return new Nr();
  e0.call(this),
    (this.h = [
      3238371032, 914150663, 812702999, 4144912697, 4290775857, 1750603025, 1694076839, 3204075428,
    ]);
}
ka.inherits(Nr, e0);
var cu = Nr;
Nr.blockSize = 512;
Nr.outSize = 224;
Nr.hmacStrength = 192;
Nr.padLength = 64;
Nr.prototype._digest = function (e) {
  return e === 'hex'
    ? ka.toHex32(this.h.slice(0, 7), 'big')
    : ka.split32(this.h.slice(0, 7), 'big');
};
var Lt = Oe,
  uu = _n,
  du = Xr,
  ar = Lt.rotr64_hi,
  sr = Lt.rotr64_lo,
  t0 = Lt.shr64_hi,
  r0 = Lt.shr64_lo,
  Pr = Lt.sum64,
  va = Lt.sum64_hi,
  ba = Lt.sum64_lo,
  hu = Lt.sum64_4_hi,
  lu = Lt.sum64_4_lo,
  pu = Lt.sum64_5_hi,
  vu = Lt.sum64_5_lo,
  n0 = uu.BlockHash,
  bu = [
    1116352408, 3609767458, 1899447441, 602891725, 3049323471, 3964484399, 3921009573, 2173295548,
    961987163, 4081628472, 1508970993, 3053834265, 2453635748, 2937671579, 2870763221, 3664609560,
    3624381080, 2734883394, 310598401, 1164996542, 607225278, 1323610764, 1426881987, 3590304994,
    1925078388, 4068182383, 2162078206, 991336113, 2614888103, 633803317, 3248222580, 3479774868,
    3835390401, 2666613458, 4022224774, 944711139, 264347078, 2341262773, 604807628, 2007800933,
    770255983, 1495990901, 1249150122, 1856431235, 1555081692, 3175218132, 1996064986, 2198950837,
    2554220882, 3999719339, 2821834349, 766784016, 2952996808, 2566594879, 3210313671, 3203337956,
    3336571891, 1034457026, 3584528711, 2466948901, 113926993, 3758326383, 338241895, 168717936,
    666307205, 1188179964, 773529912, 1546045734, 1294757372, 1522805485, 1396182291, 2643833823,
    1695183700, 2343527390, 1986661051, 1014477480, 2177026350, 1206759142, 2456956037, 344077627,
    2730485921, 1290863460, 2820302411, 3158454273, 3259730800, 3505952657, 3345764771, 106217008,
    3516065817, 3606008344, 3600352804, 1432725776, 4094571909, 1467031594, 275423344, 851169720,
    430227734, 3100823752, 506948616, 1363258195, 659060556, 3750685593, 883997877, 3785050280,
    958139571, 3318307427, 1322822218, 3812723403, 1537002063, 2003034995, 1747873779, 3602036899,
    1955562222, 1575990012, 2024104815, 1125592928, 2227730452, 2716904306, 2361852424, 442776044,
    2428436474, 593698344, 2756734187, 3733110249, 3204031479, 2999351573, 3329325298, 3815920427,
    3391569614, 3928383900, 3515267271, 566280711, 3940187606, 3454069534, 4118630271, 4000239992,
    116418474, 1914138554, 174292421, 2731055270, 289380356, 3203993006, 460393269, 320620315,
    685471733, 587496836, 852142971, 1086792851, 1017036298, 365543100, 1126000580, 2618297676,
    1288033470, 3409855158, 1501505948, 4234509866, 1607167915, 987167468, 1816402316, 1246189591,
  ];
function tr() {
  if (!(this instanceof tr)) return new tr();
  n0.call(this),
    (this.h = [
      1779033703, 4089235720, 3144134277, 2227873595, 1013904242, 4271175723, 2773480762,
      1595750129, 1359893119, 2917565137, 2600822924, 725511199, 528734635, 4215389547, 1541459225,
      327033209,
    ]),
    (this.k = bu),
    (this.W = new Array(160));
}
Lt.inherits(tr, n0);
var i0 = tr;
tr.blockSize = 1024;
tr.outSize = 512;
tr.hmacStrength = 192;
tr.padLength = 128;
tr.prototype._prepareBlock = function (e, t) {
  for (var n = this.W, i = 0; i < 32; i++) n[i] = e[t + i];
  for (; i < n.length; i += 2) {
    var a = Iu(n[i - 4], n[i - 3]),
      c = Ru(n[i - 4], n[i - 3]),
      v = n[i - 14],
      w = n[i - 13],
      y = Au(n[i - 30], n[i - 29]),
      M = Eu(n[i - 30], n[i - 29]),
      S = n[i - 32],
      I = n[i - 31];
    (n[i] = hu(a, c, v, w, y, M, S, I)), (n[i + 1] = lu(a, c, v, w, y, M, S, I));
  }
};
tr.prototype._update = function (e, t) {
  this._prepareBlock(e, t);
  var n = this.W,
    i = this.h[0],
    a = this.h[1],
    c = this.h[2],
    v = this.h[3],
    w = this.h[4],
    y = this.h[5],
    M = this.h[6],
    S = this.h[7],
    I = this.h[8],
    E = this.h[9],
    R = this.h[10],
    T = this.h[11],
    z = this.h[12],
    q = this.h[13],
    Y = this.h[14],
    Se = this.h[15];
  du(this.k.length === n.length);
  for (var de = 0; de < n.length; de += 2) {
    var H = Y,
      F = Se,
      J = _u(I, E),
      X = Su(I, E),
      W = mu(I, E, R, T, z),
      G = gu(I, E, R, T, z, q),
      A = this.k[de],
      d = this.k[de + 1],
      u = n[de],
      s = n[de + 1],
      o = pu(H, F, J, X, W, G, A, d, u, s),
      l = vu(H, F, J, X, W, G, A, d, u, s);
    (H = xu(i, a)), (F = Mu(i, a)), (J = yu(i, a, c, v, w)), (X = wu(i, a, c, v, w, y));
    var g = va(H, F, J, X),
      m = ba(H, F, J, X);
    (Y = z),
      (Se = q),
      (z = R),
      (q = T),
      (R = I),
      (T = E),
      (I = va(M, S, o, l)),
      (E = ba(S, S, o, l)),
      (M = w),
      (S = y),
      (w = c),
      (y = v),
      (c = i),
      (v = a),
      (i = va(o, l, g, m)),
      (a = ba(o, l, g, m));
  }
  Pr(this.h, 0, i, a),
    Pr(this.h, 2, c, v),
    Pr(this.h, 4, w, y),
    Pr(this.h, 6, M, S),
    Pr(this.h, 8, I, E),
    Pr(this.h, 10, R, T),
    Pr(this.h, 12, z, q),
    Pr(this.h, 14, Y, Se);
};
tr.prototype._digest = function (e) {
  return e === 'hex' ? Lt.toHex32(this.h, 'big') : Lt.split32(this.h, 'big');
};
function mu(r, e, t, n, i) {
  var a = (r & t) ^ (~r & i);
  return a < 0 && (a += 4294967296), a;
}
function gu(r, e, t, n, i, a) {
  var c = (e & n) ^ (~e & a);
  return c < 0 && (c += 4294967296), c;
}
function yu(r, e, t, n, i) {
  var a = (r & t) ^ (r & i) ^ (t & i);
  return a < 0 && (a += 4294967296), a;
}
function wu(r, e, t, n, i, a) {
  var c = (e & n) ^ (e & a) ^ (n & a);
  return c < 0 && (c += 4294967296), c;
}
function xu(r, e) {
  var t = ar(r, e, 28),
    n = ar(e, r, 2),
    i = ar(e, r, 7),
    a = t ^ n ^ i;
  return a < 0 && (a += 4294967296), a;
}
function Mu(r, e) {
  var t = sr(r, e, 28),
    n = sr(e, r, 2),
    i = sr(e, r, 7),
    a = t ^ n ^ i;
  return a < 0 && (a += 4294967296), a;
}
function _u(r, e) {
  var t = ar(r, e, 14),
    n = ar(r, e, 18),
    i = ar(e, r, 9),
    a = t ^ n ^ i;
  return a < 0 && (a += 4294967296), a;
}
function Su(r, e) {
  var t = sr(r, e, 14),
    n = sr(r, e, 18),
    i = sr(e, r, 9),
    a = t ^ n ^ i;
  return a < 0 && (a += 4294967296), a;
}
function Au(r, e) {
  var t = ar(r, e, 1),
    n = ar(r, e, 8),
    i = t0(r, e, 7),
    a = t ^ n ^ i;
  return a < 0 && (a += 4294967296), a;
}
function Eu(r, e) {
  var t = sr(r, e, 1),
    n = sr(r, e, 8),
    i = r0(r, e, 7),
    a = t ^ n ^ i;
  return a < 0 && (a += 4294967296), a;
}
function Iu(r, e) {
  var t = ar(r, e, 19),
    n = ar(e, r, 29),
    i = t0(r, e, 6),
    a = t ^ n ^ i;
  return a < 0 && (a += 4294967296), a;
}
function Ru(r, e) {
  var t = sr(r, e, 19),
    n = sr(e, r, 29),
    i = r0(r, e, 6),
    a = t ^ n ^ i;
  return a < 0 && (a += 4294967296), a;
}
var $a = Oe,
  a0 = i0;
function Or() {
  if (!(this instanceof Or)) return new Or();
  a0.call(this),
    (this.h = [
      3418070365, 3238371032, 1654270250, 914150663, 2438529370, 812702999, 355462360, 4144912697,
      1731405415, 4290775857, 2394180231, 1750603025, 3675008525, 1694076839, 1203062813,
      3204075428,
    ]);
}
$a.inherits(Or, a0);
var Nu = Or;
Or.blockSize = 1024;
Or.outSize = 384;
Or.hmacStrength = 192;
Or.padLength = 128;
Or.prototype._digest = function (e) {
  return e === 'hex'
    ? $a.toHex32(this.h.slice(0, 12), 'big')
    : $a.split32(this.h.slice(0, 12), 'big');
};
Sn.sha1 = Zc;
Sn.sha224 = cu;
Sn.sha256 = Qo;
Sn.sha384 = Nu;
Sn.sha512 = i0;
var s0 = {},
  Gr = Oe,
  Ou = _n,
  ci = Gr.rotl32,
  Ts = Gr.sum32,
  Pn = Gr.sum32_3,
  Cs = Gr.sum32_4,
  o0 = Ou.BlockHash;
function ur() {
  if (!(this instanceof ur)) return new ur();
  o0.call(this),
    (this.h = [1732584193, 4023233417, 2562383102, 271733878, 3285377520]),
    (this.endian = 'little');
}
Gr.inherits(ur, o0);
s0.ripemd160 = ur;
ur.blockSize = 512;
ur.outSize = 160;
ur.hmacStrength = 192;
ur.padLength = 64;
ur.prototype._update = function (e, t) {
  for (
    var n = this.h[0],
      i = this.h[1],
      a = this.h[2],
      c = this.h[3],
      v = this.h[4],
      w = n,
      y = i,
      M = a,
      S = c,
      I = v,
      E = 0;
    E < 80;
    E++
  ) {
    var R = Ts(ci(Cs(n, Ps(E, i, a, c), e[Pu[E] + t], Tu(E)), $u[E]), v);
    (n = v),
      (v = c),
      (c = ci(a, 10)),
      (a = i),
      (i = R),
      (R = Ts(ci(Cs(w, Ps(79 - E, y, M, S), e[ku[E] + t], Cu(E)), Du[E]), I)),
      (w = I),
      (I = S),
      (S = ci(M, 10)),
      (M = y),
      (y = R);
  }
  (R = Pn(this.h[1], a, S)),
    (this.h[1] = Pn(this.h[2], c, I)),
    (this.h[2] = Pn(this.h[3], v, w)),
    (this.h[3] = Pn(this.h[4], n, y)),
    (this.h[4] = Pn(this.h[0], i, M)),
    (this.h[0] = R);
};
ur.prototype._digest = function (e) {
  return e === 'hex' ? Gr.toHex32(this.h, 'little') : Gr.split32(this.h, 'little');
};
function Ps(r, e, t, n) {
  return r <= 15
    ? e ^ t ^ n
    : r <= 31
    ? (e & t) | (~e & n)
    : r <= 47
    ? (e | ~t) ^ n
    : r <= 63
    ? (e & n) | (t & ~n)
    : e ^ (t | ~n);
}
function Tu(r) {
  return r <= 15
    ? 0
    : r <= 31
    ? 1518500249
    : r <= 47
    ? 1859775393
    : r <= 63
    ? 2400959708
    : 2840853838;
}
function Cu(r) {
  return r <= 15
    ? 1352829926
    : r <= 31
    ? 1548603684
    : r <= 47
    ? 1836072691
    : r <= 63
    ? 2053994217
    : 0;
}
var Pu = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 7, 4, 13, 1, 10, 6, 15, 3, 12, 0, 9, 5, 2,
    14, 11, 8, 3, 10, 14, 4, 9, 15, 8, 1, 2, 7, 0, 6, 13, 11, 5, 12, 1, 9, 11, 10, 0, 8, 12, 4, 13,
    3, 7, 15, 14, 5, 6, 2, 4, 0, 5, 9, 7, 12, 2, 10, 14, 1, 3, 8, 11, 6, 15, 13,
  ],
  ku = [
    5, 14, 7, 0, 9, 2, 11, 4, 13, 6, 15, 8, 1, 10, 3, 12, 6, 11, 3, 7, 0, 13, 5, 10, 14, 15, 8, 12,
    4, 9, 1, 2, 15, 5, 1, 3, 7, 14, 6, 9, 11, 8, 12, 2, 10, 0, 4, 13, 8, 6, 4, 1, 3, 11, 15, 0, 5,
    12, 2, 13, 9, 7, 10, 14, 12, 15, 10, 4, 1, 5, 8, 7, 6, 2, 13, 14, 0, 3, 9, 11,
  ],
  $u = [
    11, 14, 15, 12, 5, 8, 7, 9, 11, 13, 14, 15, 6, 7, 9, 8, 7, 6, 8, 13, 11, 9, 7, 15, 7, 12, 15, 9,
    11, 7, 13, 12, 11, 13, 6, 7, 14, 9, 13, 15, 14, 8, 13, 6, 5, 12, 7, 5, 11, 12, 14, 15, 14, 15,
    9, 8, 9, 14, 5, 6, 8, 6, 5, 12, 9, 15, 5, 11, 6, 8, 13, 12, 5, 12, 13, 14, 11, 8, 5, 6,
  ],
  Du = [
    8, 9, 9, 11, 13, 15, 15, 5, 7, 7, 8, 11, 14, 14, 12, 6, 9, 13, 15, 7, 12, 8, 9, 11, 7, 7, 12, 7,
    6, 15, 13, 11, 9, 7, 15, 11, 8, 6, 6, 14, 12, 13, 5, 14, 13, 13, 7, 5, 15, 5, 8, 11, 14, 14, 6,
    14, 6, 9, 12, 9, 12, 5, 15, 8, 8, 5, 12, 9, 12, 5, 14, 6, 8, 13, 6, 5, 15, 13, 11, 11,
  ],
  Lu = Oe,
  qu = Xr;
function vn(r, e, t) {
  if (!(this instanceof vn)) return new vn(r, e, t);
  (this.Hash = r),
    (this.blockSize = r.blockSize / 8),
    (this.outSize = r.outSize / 8),
    (this.inner = null),
    (this.outer = null),
    this._init(Lu.toArray(e, t));
}
var Bu = vn;
vn.prototype._init = function (e) {
  e.length > this.blockSize && (e = new this.Hash().update(e).digest()),
    qu(e.length <= this.blockSize);
  for (var t = e.length; t < this.blockSize; t++) e.push(0);
  for (t = 0; t < e.length; t++) e[t] ^= 54;
  for (this.inner = new this.Hash().update(e), t = 0; t < e.length; t++) e[t] ^= 106;
  this.outer = new this.Hash().update(e);
};
vn.prototype.update = function (e, t) {
  return this.inner.update(e, t), this;
};
vn.prototype.digest = function (e) {
  return this.outer.update(this.inner.digest()), this.outer.digest(e);
};
(function (r) {
  var e = r;
  (e.utils = Oe),
    (e.common = _n),
    (e.sha = Sn),
    (e.ripemd = s0),
    (e.hmac = Bu),
    (e.sha1 = e.sha.sha1),
    (e.sha256 = e.sha.sha256),
    (e.sha224 = e.sha.sha224),
    (e.sha384 = e.sha.sha384),
    (e.sha512 = e.sha.sha512),
    (e.ripemd160 = e.ripemd.ripemd160);
})($r);
var jn;
(function (r) {
  (r.sha256 = 'sha256'), (r.sha512 = 'sha512');
})(jn || (jn = {}));
const Fu = 'sha2/5.7.0',
  Uu = new Te(Fu);
function zu(r) {
  return '0x' + $r.ripemd160().update(V(r)).digest('hex');
}
function Pt(r) {
  return '0x' + $r.sha256().update(V(r)).digest('hex');
}
function Si(r, e, t) {
  return (
    jn[r] ||
      Uu.throwError('unsupported algorithm ' + r, Te.errors.UNSUPPORTED_OPERATION, {
        operation: 'hmac',
        algorithm: r,
      }),
    '0x' + $r.hmac($r[r], V(e)).update(V(t)).digest('hex')
  );
}
var f0 = (r, e, t) => {
    if (!e.has(r)) throw TypeError('Cannot ' + t);
  },
  Un = (r, e, t) => (f0(r, e, 'read from private field'), t ? t.call(r) : e.get(r)),
  Da = (r, e, t) => {
    if (e.has(r)) throw TypeError('Cannot add the same private member more than once');
    e instanceof WeakSet ? e.add(r) : e.set(r, t);
  },
  La = (r, e, t, n) => (f0(r, e, 'write to private field'), n ? n.call(r, t) : e.set(r, t), t),
  ju = new Te(Mn.FUELS),
  Ie = class {
    constructor(e, t, n) {
      (this.name = e), (this.type = t), (this.encodedLength = n);
    }
    throwError(e, t) {
      throw (ju.throwArgumentError(e, this.name, t), new Error('unreachable'));
    }
    setOffset(e) {
      this.offset = e;
    }
  },
  Et = class extends Ie {
    constructor(e, t) {
      super('array', `[${e.type}; ${t}]`, t * e.encodedLength), (this.coder = e), (this.length = t);
    }
    encode(e) {
      return (
        Array.isArray(e) || this.throwError('expected array value', e),
        this.length !== e.length && this.throwError('Types/values length mismatch', e),
        se(Array.from(e).map((t) => this.coder.encode(t)))
      );
    }
    decode(e, t) {
      let n = t;
      return [
        Array(this.length)
          .fill(0)
          .map(() => {
            let i;
            return ([i, n] = this.coder.decode(e, n)), i;
          }),
        n,
      ];
    }
  },
  U = class extends Ie {
    constructor() {
      super('b256', 'b256', 32);
    }
    encode(e) {
      let t;
      try {
        t = V(e);
      } catch {
        this.throwError(`Invalid ${this.type}`, e);
      }
      return t.length !== 32 && this.throwError(`Invalid ${this.type}`, e), t;
    }
    decode(e, t) {
      let n = e.slice(t, t + 32);
      return (
        B(n).isZero() && (n = new Uint8Array(32)),
        n.length !== 32 && this.throwError('Invalid size for b256', n),
        [Za(n, 32), t + 32]
      );
    }
  },
  Vu = class extends Ie {
    constructor() {
      super('b512', 'b512', 64);
    }
    encode(e) {
      let t;
      try {
        t = V(e);
      } catch {
        this.throwError(`Invalid ${this.type}`, e);
      }
      return t.length !== 64 && this.throwError(`Invalid ${this.type}`, e), t;
    }
    decode(e, t) {
      let n = e.slice(t, t + 64);
      return (
        B(n).isZero() && (n = new Uint8Array(64)),
        n.length !== 64 && this.throwError('Invalid size for b512', n),
        [Za(n, 64), t + 64]
      );
    }
  },
  Ju = class extends Ie {
    constructor() {
      super('boolean', 'boolean', 8);
    }
    encode(e) {
      let t;
      try {
        t = or(e ? 1 : 0);
      } catch {
        this.throwError('Invalid bool', e);
      }
      return t.length > 1 && this.throwError('Invalid bool', e), or(t, 8);
    }
    decode(e, t) {
      let n = B(e.slice(t, t + 8));
      return n.isZero()
        ? [!1, t + 8]
        : (n.eq(B(1)) || this.throwError('Invalid boolean value', n), [!0, t + 8]);
    }
  },
  Hu = class extends Ie {
    constructor() {
      super('byte', 'byte', 8);
    }
    encode(e) {
      let t;
      try {
        t = or(e, 1);
      } catch {
        this.throwError('Invalid Byte', e);
      }
      return or(t, 8);
    }
    decode(e, t) {
      let n = e.slice(t, t + 8),
        i = B(n);
      return i.gt(B(255)) && this.throwError('Invalid Byte', i), [Number(i), t + 8];
    }
  },
  $ = class extends Ie {
    constructor() {
      super('u64', 'u64', 8);
    }
    encode(e) {
      let t;
      try {
        t = or(e, 8);
      } catch {
        this.throwError(`Invalid ${this.type}`, e);
      }
      return t;
    }
    decode(e, t) {
      let n = e.slice(t, t + 8);
      return (n = n.slice(0, 8)), [B(n), t + 8];
    }
  },
  bi,
  Dn,
  c0 = class extends Ie {
    constructor(e, t) {
      let n = new $(),
        i = Object.values(t).reduce((a, c) => Math.max(a, c.encodedLength), 0);
      super('enum', `enum ${e}`, n.encodedLength + i),
        Da(this, bi, void 0),
        Da(this, Dn, void 0),
        (this.name = e),
        (this.coders = t),
        La(this, bi, n),
        La(this, Dn, i);
    }
    encode(e) {
      let [t, ...n] = Object.keys(e);
      if (!t) throw new Error('A field for the case must be provided');
      if (n.length !== 0) throw new Error('Only one field must be provided');
      let i = this.coders[t],
        a = Object.keys(this.coders).indexOf(t),
        c = i.encode(e[t]),
        v = new Uint8Array(Un(this, Dn) - i.encodedLength);
      return se([Un(this, bi).encode(a), v, c]);
    }
    decode(e, t) {
      let n = t,
        i;
      [i, n] = new $().decode(e, n);
      let a = nr(i),
        c = Object.keys(this.coders)[a];
      if (!c) throw new Error(`Invalid caseIndex "${a}". Valid cases: ${Object.keys(this.coders)}`);
      let v = this.coders[c];
      return (n += Un(this, Dn) - v.encodedLength), ([i, n] = v.decode(e, n)), [{ [c]: i }, n];
    }
  };
(bi = new WeakMap()), (Dn = new WeakMap());
var te = class extends Ie {
    constructor(r) {
      switch ((super('number', r, 8), (this.baseType = r), r)) {
        case 'u8':
          this.length = 1;
          break;
        case 'u16':
          this.length = 2;
          break;
        case 'u32':
        default:
          this.length = 4;
          break;
      }
    }
    encode(r) {
      let e;
      try {
        e = or(r);
      } catch {
        this.throwError(`Invalid ${this.baseType}`, r);
      }
      return (
        e.length > this.length && this.throwError(`Invalid ${this.baseType}. Too many bytes.`, r),
        or(e, 8)
      );
    }
    decode(r, e) {
      let t = r.slice(e, e + 8);
      return (t = t.slice(8 - this.length, 8)), [nr(t), e + 8];
    }
  },
  Ln,
  Wu = class extends Ie {
    constructor(e) {
      let t = (8 - e) % 8;
      (t = t < 0 ? t + 8 : t),
        super('string', `str[${e}]`, e + t),
        Da(this, Ln, void 0),
        (this.length = e),
        La(this, Ln, t);
    }
    encode(e) {
      let t = uc(e.slice(0, this.length)),
        n = new Uint8Array(Un(this, Ln));
      return se([t, n]);
    }
    decode(e, t) {
      let n = e.slice(t, t + this.length),
        i = hc(n),
        a = Un(this, Ln);
      return [i, t + this.length + a];
    }
  };
Ln = new WeakMap();
var u0 = class extends c0 {
    encode(e) {
      return super.encode(this.toSwayOption(e));
    }
    toSwayOption(e) {
      return e !== void 0 ? { Some: e } : { None: [] };
    }
    decode(e, t) {
      let [n, i] = super.decode(e, t);
      return [this.toOption(n), i];
    }
    toOption(e) {
      if (e && 'Some' in e) return e.Some;
    }
  },
  zi = class extends Ie {
    constructor(e, t) {
      let n = Object.values(t).reduce((i, a) => i + a.encodedLength, 0);
      super('struct', `struct ${e}`, n), (this.name = e), (this.coders = t);
    }
    encode(e) {
      let t = Object.keys(this.coders).map((n) => {
        let i = this.coders[n],
          a = e[n];
        return (
          !(i instanceof u0) &&
            a == null &&
            this.throwError(`Invalid ${this.type}. Field "${n}" not present.`, e),
          i.encode(a)
        );
      });
      return se(t);
    }
    decode(e, t) {
      let n = t;
      return [
        Object.keys(this.coders).reduce((i, a) => {
          let c = this.coders[a],
            v;
          return ([v, n] = c.decode(e, n)), (i[a] = v), i;
        }, {}),
        n,
      ];
    }
  },
  ma = class extends Ie {
    constructor(e) {
      let t = e.reduce((n, i) => n + i.encodedLength, 0);
      super('tuple', `(${e.map((n) => n.type).join(', ')})`, t), (this.coders = e);
    }
    encode(e) {
      return (
        this.coders.length !== e.length &&
          this.throwError('Types/values length mismatch', { value: e }),
        se(this.coders.map((t, n) => t.encode(e[n])))
      );
    }
    decode(e, t) {
      let n = t;
      return [
        this.coders.map((i) => {
          let a;
          return ([a, n] = i.decode(e, n)), a;
        }),
        n,
      ];
    }
  },
  d0 = 'enum Option',
  Gu = 'struct Vec',
  Ku = /str\[(?<length>[0-9]+)\]/,
  Xu = /\[(?<item>[\w\s\\[\]]+);\s*(?<length>[0-9]+)\]/,
  Zu = /^struct (?<name>\w+)$/,
  Yu = /^enum (?<name>\w+)$/,
  Qu = /^\((?<items>.*)\)$/,
  ga = 8,
  ed = 32,
  td = 32,
  rd = 10240,
  nd = 104,
  id = 3,
  mi = class extends Ie {
    constructor(e) {
      super('struct', 'struct Vec', 0), (this.coder = e);
    }
    static getBaseOffset() {
      return id * 8;
    }
    getEncodedVectorData(e) {
      Array.isArray(e) || this.throwError('expected array value', e);
      let t = Array.from(e).map((n) => this.coder.encode(n));
      return se(t);
    }
    encode(e) {
      Array.isArray(e) || this.throwError('expected array value', e);
      let t = [],
        n = this.offset || 0;
      return (
        t.push(new $().encode(n)),
        t.push(new $().encode(e.length)),
        t.push(new $().encode(e.length)),
        se(t)
      );
    }
    decode(e, t) {
      return this.throwError('unexpected Vec decode', 'not implemented'), [void 0, t];
    }
  };
function ks(r) {
  return r.filter((e) => e?.type !== '()' && e !== '()');
}
function ad(r) {
  return r.some((e) => e?.type === d0);
}
function sd(r, e, t = 0) {
  let n = [],
    i = r.map((v, w) => {
      if (!(v instanceof mi)) return { byteLength: v.encodedLength };
      let y = v.getEncodedVectorData(e[w]);
      return n.push(y), { vecByteLength: y.byteLength };
    }),
    a = n.length * mi.getBaseOffset() + t,
    c = r.map((v, w) =>
      v instanceof mi
        ? i.reduce(
            (y, M, S) =>
              'byteLength' in M
                ? y + M.byteLength
                : S === 0 && S === w
                ? a
                : S < w
                ? y + M.vecByteLength + a
                : y,
            0
          )
        : 0
    );
  return r.forEach((v, w) => v.setOffset(c[w])), n;
}
var ui = new Te(Mn.FUELS),
  Ai = class {
    constructor() {
      ui.checkNew(new.target, Ai);
    }
    getCoder(e) {
      var t, n, i, a, c, v;
      switch (e.type) {
        case 'u8':
        case 'u16':
        case 'u32':
          return new te(e.type);
        case 'u64':
        case 'raw untyped ptr':
          return new $();
        case 'bool':
          return new Ju();
        case 'byte':
          return new Hu();
        case 'b256':
          return new U();
        case 'b512':
          return new Vu();
      }
      let w = (t = Xu.exec(e.type)) == null ? void 0 : t.groups;
      if (w) {
        let I = parseInt(w.length, 10),
          E = (n = e.components) == null ? void 0 : n[0];
        if (!E) throw new Error('Expected array type to have an item component');
        let R = this.getCoder(E);
        return new Et(R, I);
      }
      let y = (i = Ku.exec(e.type)) == null ? void 0 : i.groups;
      if (y) {
        let I = parseInt(y.length, 10);
        return new Wu(I);
      }
      if (e.type === Gu && Array.isArray(e.typeArguments)) {
        let I = e.typeArguments[0];
        if (!I) throw new Error('Expected Vec type to have a type argument');
        let E = this.getCoder(I);
        return new mi(E);
      }
      let M = (a = Zu.exec(e.type)) == null ? void 0 : a.groups;
      if (M && Array.isArray(e.components)) {
        let I = e.components.reduce((E, R) => ((E[R.name] = this.getCoder(R)), E), {});
        return new zi(M.name, I);
      }
      let S = (c = Yu.exec(e.type)) == null ? void 0 : c.groups;
      if (S && Array.isArray(e.components)) {
        let I = e.components.reduce((E, R) => ((E[R.name] = this.getCoder(R)), E), {});
        return e.type === d0 ? new u0(S.name, I) : new c0(S.name, I);
      }
      if ((v = Qu.exec(e.type)) != null && v.groups && Array.isArray(e.components)) {
        let I = e.components.map((E) => this.getCoder(E));
        return new ma(I);
      }
      return ui.throwArgumentError('Invalid type', 'type', e.type);
    }
    encode(e, t, n = 0) {
      let i = ks(e),
        a = t.slice();
      Array.isArray(t) &&
        i.length !== t.length &&
        (ad(e)
          ? ((a.length = e.length), a.fill(void 0, t.length))
          : ui.throwError(
              'Types/values length mismatch during encode',
              Te.errors.INVALID_ARGUMENT,
              {
                count: { types: e.length, nonEmptyTypes: i.length, values: t.length },
                value: { types: e, nonEmptyTypes: i, values: t },
              }
            ));
      let c = i.map((y) => this.getCoder(y)),
        v = sd(c, a, n),
        w = new ma(c).encode(a);
      return se([w, se(v)]);
    }
    decode(e, t) {
      let n = V(t),
        i = ks(e),
        a = (M) => {
          M !== n.length &&
            ui.throwError(
              'Types/values length mismatch during decode',
              Te.errors.INVALID_ARGUMENT,
              {
                count: { types: e.length, nonEmptyTypes: i.length, values: n.length },
                value: { types: e, nonEmptyTypes: i, values: n },
              }
            );
        };
      if (e.length === 0 || i.length === 0) {
        a(n.length ? 8 : 0);
        return;
      }
      let c = i.map((M) => this.getCoder(M)),
        v = new ma(c),
        [w, y] = v.decode(n, 0);
      return a(y), w;
    }
  };
new Te(Mn.FUELS);
var od = class {},
  fd = class {},
  h0 = class {};
function l0(r, e, t, n, i) {
  (r = V(r)), (e = V(e));
  let a,
    c = 1;
  const v = new Uint8Array(n),
    w = new Uint8Array(e.length + 4);
  w.set(e);
  let y, M;
  for (let S = 1; S <= c; S++) {
    (w[e.length] = (S >> 24) & 255),
      (w[e.length + 1] = (S >> 16) & 255),
      (w[e.length + 2] = (S >> 8) & 255),
      (w[e.length + 3] = S & 255);
    let I = V(Si(i, r, w));
    a || ((a = I.length), (M = new Uint8Array(a)), (c = Math.ceil(n / a)), (y = n - (c - 1) * a)),
      M.set(I);
    for (let T = 1; T < t; T++) {
      I = V(Si(i, r, I));
      for (let z = 0; z < a; z++) M[z] ^= I[z];
    }
    const E = (S - 1) * a,
      R = S === c ? y : a;
    v.set(V(M).slice(0, R), E);
  }
  return K(v);
}
var $s = ((r) =>
    typeof require < 'u'
      ? require
      : typeof Proxy < 'u'
      ? new Proxy(r, { get: (e, t) => (typeof require < 'u' ? require : e)[t] })
      : r)(function (r) {
    if (typeof require < 'u') return require.apply(this, arguments);
    throw new Error('Dynamic require of "' + r + '" is not supported');
  }),
  Ei,
  Ya = 'Node';
typeof globalThis < 'u' && globalThis.crypto && ((Ei = globalThis.crypto), (Ya = 'Web'));
if (!Ei && typeof $s == 'function')
  try {
    (Ei = $s('crypto')), (Ya = 'Node');
  } catch (r) {
    console.error('keystore expects a standard Web browser or Node environment.', r);
  }
var Dr = Ei,
  Kn = Ya;
function Rr(r, e = 'base64') {
  return Kn === 'Node'
    ? Buffer.from(r, e)
    : e === 'utf-8'
    ? new TextEncoder().encode(r)
    : new Uint8Array(
        atob(r)
          .split('')
          .map((t) => t.charCodeAt(0))
      );
}
function cn(r, e = 'base64') {
  return Kn === 'Node'
    ? Buffer.from(r).toString(e)
    : btoa(String.fromCharCode.apply(null, new Uint8Array(r)));
}
function ji(r, e) {
  let t = Rr(String(r).normalize('NFKC'), 'utf-8'),
    n = l0(t, e, 1e5, 32, 'sha256');
  return V(n);
}
var Tr = (r) => (Kn === 'Node' ? Dr.randomBytes(r) : Dr.getRandomValues(new Uint8Array(r))),
  p0 = 'aes-256-ctr';
async function cd(r, e) {
  let t = Tr(16),
    n = Tr(32),
    i = ji(r, n),
    a = Uint8Array.from(Buffer.from(JSON.stringify(e), 'utf-8')),
    c = Dr.createCipheriv(p0, i, t),
    v = c.update(a);
  return (v = Buffer.concat([v, c.final()])), { data: cn(v), iv: cn(t), salt: cn(n) };
}
async function ud(r, e) {
  let t = Rr(e.iv),
    n = Rr(e.salt),
    i = ji(r, n),
    a = Rr(e.data),
    c = Dr.createDecipheriv(p0, i, t),
    v = c.update(a),
    w = Buffer.concat([v, c.final()]),
    y = Buffer.from(w).toString('utf-8');
  try {
    return JSON.parse(y);
  } catch {
    throw new Error('Invalid credentials');
  }
}
var v0 = 'AES-CTR';
async function dd(r, e) {
  let t = Tr(16),
    n = Tr(32),
    i = ji(r, n),
    a = JSON.stringify(e),
    c = Rr(a, 'utf-8'),
    v = { name: v0, counter: t, length: 64 },
    w = await Dr.subtle.importKey('raw', i, v, !1, ['encrypt']),
    y = await Dr.subtle.encrypt(v, w, c);
  return { data: cn(y), iv: cn(t), salt: cn(n) };
}
async function hd(r, e) {
  let t = Rr(e.iv),
    n = Rr(e.salt),
    i = ji(r, n),
    a = Rr(e.data),
    c = { name: v0, counter: t, length: 64 },
    v = await Dr.subtle.importKey('raw', i, c, !1, ['decrypt']),
    w = await Dr.subtle.decrypt(c, v, a),
    y = new TextDecoder().decode(w);
  try {
    return JSON.parse(y);
  } catch {
    throw new Error('Invalid credentials');
  }
}
async function Mv(r, e) {
  return Kn === 'Node' ? cd(r, e) : dd(r, e);
}
async function _v(r, e) {
  return Kn === 'Node' ? ud(r, e) : hd(r, e);
}
var Vn = {};
Object.defineProperty(Vn, '__esModule', { value: !0 });
var bn = (Vn.bech32m = Vn.bech32 = void 0);
const Ii = 'qpzry9x8gf2tvdw0s3jn54khce6mua7l',
  b0 = {};
for (let r = 0; r < Ii.length; r++) {
  const e = Ii.charAt(r);
  b0[e] = r;
}
function un(r) {
  const e = r >> 25;
  return (
    ((r & 33554431) << 5) ^
    (-((e >> 0) & 1) & 996825010) ^
    (-((e >> 1) & 1) & 642813549) ^
    (-((e >> 2) & 1) & 513874426) ^
    (-((e >> 3) & 1) & 1027748829) ^
    (-((e >> 4) & 1) & 705979059)
  );
}
function Ds(r) {
  let e = 1;
  for (let t = 0; t < r.length; ++t) {
    const n = r.charCodeAt(t);
    if (n < 33 || n > 126) return 'Invalid prefix (' + r + ')';
    e = un(e) ^ (n >> 5);
  }
  e = un(e);
  for (let t = 0; t < r.length; ++t) {
    const n = r.charCodeAt(t);
    e = un(e) ^ (n & 31);
  }
  return e;
}
function Qa(r, e, t, n) {
  let i = 0,
    a = 0;
  const c = (1 << t) - 1,
    v = [];
  for (let w = 0; w < r.length; ++w)
    for (i = (i << e) | r[w], a += e; a >= t; ) (a -= t), v.push((i >> a) & c);
  if (n) a > 0 && v.push((i << (t - a)) & c);
  else {
    if (a >= e) return 'Excess padding';
    if ((i << (t - a)) & c) return 'Non-zero padding';
  }
  return v;
}
function ld(r) {
  return Qa(r, 8, 5, !0);
}
function pd(r) {
  const e = Qa(r, 5, 8, !1);
  if (Array.isArray(e)) return e;
}
function vd(r) {
  const e = Qa(r, 5, 8, !1);
  if (Array.isArray(e)) return e;
  throw new Error(e);
}
function m0(r) {
  let e;
  r === 'bech32' ? (e = 1) : (e = 734539939);
  function t(c, v, w) {
    if (((w = w || 90), c.length + 7 + v.length > w)) throw new TypeError('Exceeds length limit');
    c = c.toLowerCase();
    let y = Ds(c);
    if (typeof y == 'string') throw new Error(y);
    let M = c + '1';
    for (let S = 0; S < v.length; ++S) {
      const I = v[S];
      if (I >> 5) throw new Error('Non 5-bit word');
      (y = un(y) ^ I), (M += Ii.charAt(I));
    }
    for (let S = 0; S < 6; ++S) y = un(y);
    y ^= e;
    for (let S = 0; S < 6; ++S) {
      const I = (y >> ((5 - S) * 5)) & 31;
      M += Ii.charAt(I);
    }
    return M;
  }
  function n(c, v) {
    if (((v = v || 90), c.length < 8)) return c + ' too short';
    if (c.length > v) return 'Exceeds length limit';
    const w = c.toLowerCase(),
      y = c.toUpperCase();
    if (c !== w && c !== y) return 'Mixed-case string ' + c;
    c = w;
    const M = c.lastIndexOf('1');
    if (M === -1) return 'No separator character for ' + c;
    if (M === 0) return 'Missing prefix for ' + c;
    const S = c.slice(0, M),
      I = c.slice(M + 1);
    if (I.length < 6) return 'Data too short';
    let E = Ds(S);
    if (typeof E == 'string') return E;
    const R = [];
    for (let T = 0; T < I.length; ++T) {
      const z = I.charAt(T),
        q = b0[z];
      if (q === void 0) return 'Unknown character ' + z;
      (E = un(E) ^ q), !(T + 6 >= I.length) && R.push(q);
    }
    return E !== e ? 'Invalid checksum for ' + c : { prefix: S, words: R };
  }
  function i(c, v) {
    const w = n(c, v);
    if (typeof w == 'object') return w;
  }
  function a(c, v) {
    const w = n(c, v);
    if (typeof w == 'object') return w;
    throw new Error(w);
  }
  return { decodeUnsafe: i, decode: a, encode: t, toWords: ld, fromWordsUnsafe: pd, fromWords: vd };
}
Vn.bech32 = m0('bech32');
bn = Vn.bech32m = m0('bech32m');
var bd = new Te(Mn.FUELS),
  Ri = 'fuel';
function es(r) {
  return bn.decode(r);
}
function Ls(r) {
  return bn.encode(Ri, bn.toWords(V(K(r))));
}
function gi(r) {
  return typeof r == 'string' && r.indexOf(Ri + 1) === 0 && es(r).prefix === Ri;
}
function md(r) {
  return (r.length === 66 || r.length === 64) && /(0x)?[0-9a-f]{64}$/i.test(r);
}
function gd(r) {
  return (r.length === 130 || r.length === 128) && /(0x)?[0-9a-f]{128}$/i.test(r);
}
function g0(r) {
  return new Uint8Array(bn.fromWords(es(r).words));
}
function yd(r) {
  return gi(r) || bd.throwArgumentError('Invalid Bech32 Address', 'address', r), K(g0(r));
}
function wd(r) {
  let { words: e } = es(r);
  return bn.encode(Ri, e);
}
var an = (r) => (r instanceof h0 ? r.address : r instanceof fd ? r.id : r),
  xd = () => K(Tr(32)),
  qs = new Te(Mn.FUELS),
  yt = class extends od {
    constructor(r) {
      super(),
        qs.checkNew(new.target, yt),
        (this.bech32Address = wd(r)),
        gi(this.bech32Address) || qs.throwArgumentError('Invalid Bech32 Address', 'address', r);
    }
    toAddress() {
      return this.bech32Address;
    }
    toB256() {
      return yd(this.bech32Address);
    }
    toBytes() {
      return g0(this.bech32Address);
    }
    toHexString() {
      return this.toB256();
    }
    toString() {
      return this.bech32Address;
    }
    toJSON() {
      return this.toString();
    }
    valueOf() {
      return this.toString();
    }
    equals(r) {
      return this.bech32Address === r.bech32Address;
    }
    static fromPublicKey(r) {
      let e = Pt(r);
      return new yt(Ls(e));
    }
    static fromB256(r) {
      return new yt(Ls(r));
    }
    static fromRandom() {
      return this.fromB256(xd());
    }
    static fromString(r) {
      return gi(r) ? new yt(r) : this.fromB256(r);
    }
    static fromAddressOrString(r) {
      return typeof r == 'string' ? this.fromString(r) : r;
    }
    static fromDynamicInput(r) {
      if (gd(r)) return yt.fromPublicKey(r);
      if (gi(r)) return new yt(r);
      if (md(r)) return yt.fromB256(r);
      throw new Error('Unknown address format: only Bech32, B256, or Public Key (512) supported');
    }
  },
  gt = '0x0000000000000000000000000000000000000000000000000000000000000000',
  Ht = gt,
  Ni = function () {
    return (
      (Ni =
        Object.assign ||
        function (e) {
          for (var t, n = 1, i = arguments.length; n < i; n++) {
            t = arguments[n];
            for (var a in t) Object.prototype.hasOwnProperty.call(t, a) && (e[a] = t[a]);
          }
          return e;
        }),
      Ni.apply(this, arguments)
    );
  };
function Sv(r, e) {
  var t = {};
  for (var n in r) Object.prototype.hasOwnProperty.call(r, n) && e.indexOf(n) < 0 && (t[n] = r[n]);
  if (r != null && typeof Object.getOwnPropertySymbols == 'function')
    for (var i = 0, n = Object.getOwnPropertySymbols(r); i < n.length; i++)
      e.indexOf(n[i]) < 0 &&
        Object.prototype.propertyIsEnumerable.call(r, n[i]) &&
        (t[n[i]] = r[n[i]]);
  return t;
}
function Av(r, e, t) {
  if (t || arguments.length === 2)
    for (var n = 0, i = e.length, a; n < i; n++)
      (a || !(n in e)) && (a || (a = Array.prototype.slice.call(e, 0, n)), (a[n] = e[n]));
  return r.concat(a || Array.prototype.slice.call(e));
}
function yi(r, e) {
  if (!Boolean(r)) throw new Error(e);
}
function Md(r) {
  return typeof r == 'object' && r !== null;
}
function _d(r, e) {
  if (!Boolean(r)) throw new Error(e ?? 'Unexpected invariant triggered.');
}
const Sd = /\r\n|[\n\r]/g;
function qa(r, e) {
  let t = 0,
    n = 1;
  for (const i of r.body.matchAll(Sd)) {
    if ((typeof i.index == 'number' || _d(!1), i.index >= e)) break;
    (t = i.index + i[0].length), (n += 1);
  }
  return { line: n, column: e + 1 - t };
}
function Ad(r) {
  return y0(r.source, qa(r.source, r.start));
}
function y0(r, e) {
  const t = r.locationOffset.column - 1,
    n = ''.padStart(t) + r.body,
    i = e.line - 1,
    a = r.locationOffset.line - 1,
    c = e.line + a,
    v = e.line === 1 ? t : 0,
    w = e.column + v,
    y = `${r.name}:${c}:${w}
`,
    M = n.split(/\r\n|[\n\r]/g),
    S = M[i];
  if (S.length > 120) {
    const I = Math.floor(w / 80),
      E = w % 80,
      R = [];
    for (let T = 0; T < S.length; T += 80) R.push(S.slice(T, T + 80));
    return (
      y +
      Bs([
        [`${c} |`, R[0]],
        ...R.slice(1, I + 1).map((T) => ['|', T]),
        ['|', '^'.padStart(E)],
        ['|', R[I + 1]],
      ])
    );
  }
  return (
    y +
    Bs([
      [`${c - 1} |`, M[i - 1]],
      [`${c} |`, S],
      ['|', '^'.padStart(w)],
      [`${c + 1} |`, M[i + 1]],
    ])
  );
}
function Bs(r) {
  const e = r.filter(([n, i]) => i !== void 0),
    t = Math.max(...e.map(([n]) => n.length));
  return e.map(([n, i]) => n.padStart(t) + (i ? ' ' + i : '')).join(`
`);
}
function Ed(r) {
  const e = r[0];
  return e == null || 'kind' in e || 'length' in e
    ? { nodes: e, source: r[1], positions: r[2], path: r[3], originalError: r[4], extensions: r[5] }
    : e;
}
class ts extends Error {
  constructor(e, ...t) {
    var n, i, a;
    const { nodes: c, source: v, positions: w, path: y, originalError: M, extensions: S } = Ed(t);
    super(e),
      (this.name = 'GraphQLError'),
      (this.path = y ?? void 0),
      (this.originalError = M ?? void 0),
      (this.nodes = Fs(Array.isArray(c) ? c : c ? [c] : void 0));
    const I = Fs(
      (n = this.nodes) === null || n === void 0
        ? void 0
        : n.map((R) => R.loc).filter((R) => R != null)
    );
    (this.source = v ?? (I == null || (i = I[0]) === null || i === void 0 ? void 0 : i.source)),
      (this.positions = w ?? I?.map((R) => R.start)),
      (this.locations = w && v ? w.map((R) => qa(v, R)) : I?.map((R) => qa(R.source, R.start)));
    const E = Md(M?.extensions) ? M?.extensions : void 0;
    (this.extensions = (a = S ?? E) !== null && a !== void 0 ? a : Object.create(null)),
      Object.defineProperties(this, {
        message: { writable: !0, enumerable: !0 },
        name: { enumerable: !1 },
        nodes: { enumerable: !1 },
        source: { enumerable: !1 },
        positions: { enumerable: !1 },
        originalError: { enumerable: !1 },
      }),
      M != null && M.stack
        ? Object.defineProperty(this, 'stack', { value: M.stack, writable: !0, configurable: !0 })
        : Error.captureStackTrace
        ? Error.captureStackTrace(this, ts)
        : Object.defineProperty(this, 'stack', {
            value: Error().stack,
            writable: !0,
            configurable: !0,
          });
  }
  get [Symbol.toStringTag]() {
    return 'GraphQLError';
  }
  toString() {
    let e = this.message;
    if (this.nodes)
      for (const t of this.nodes)
        t.loc &&
          (e +=
            `

` + Ad(t.loc));
    else if (this.source && this.locations)
      for (const t of this.locations)
        e +=
          `

` + y0(this.source, t);
    return e;
  }
  toJSON() {
    const e = { message: this.message };
    return (
      this.locations != null && (e.locations = this.locations),
      this.path != null && (e.path = this.path),
      this.extensions != null &&
        Object.keys(this.extensions).length > 0 &&
        (e.extensions = this.extensions),
      e
    );
  }
}
function Fs(r) {
  return r === void 0 || r.length === 0 ? void 0 : r;
}
function Mt(r, e, t) {
  return new ts(`Syntax Error: ${t}`, { source: r, positions: [e] });
}
class Id {
  constructor(e, t, n) {
    (this.start = e.start),
      (this.end = t.end),
      (this.startToken = e),
      (this.endToken = t),
      (this.source = n);
  }
  get [Symbol.toStringTag]() {
    return 'Location';
  }
  toJSON() {
    return { start: this.start, end: this.end };
  }
}
class w0 {
  constructor(e, t, n, i, a, c) {
    (this.kind = e),
      (this.start = t),
      (this.end = n),
      (this.line = i),
      (this.column = a),
      (this.value = c),
      (this.prev = null),
      (this.next = null);
  }
  get [Symbol.toStringTag]() {
    return 'Token';
  }
  toJSON() {
    return { kind: this.kind, value: this.value, line: this.line, column: this.column };
  }
}
const x0 = {
    Name: [],
    Document: ['definitions'],
    OperationDefinition: ['name', 'variableDefinitions', 'directives', 'selectionSet'],
    VariableDefinition: ['variable', 'type', 'defaultValue', 'directives'],
    Variable: ['name'],
    SelectionSet: ['selections'],
    Field: ['alias', 'name', 'arguments', 'directives', 'selectionSet'],
    Argument: ['name', 'value'],
    FragmentSpread: ['name', 'directives'],
    InlineFragment: ['typeCondition', 'directives', 'selectionSet'],
    FragmentDefinition: [
      'name',
      'variableDefinitions',
      'typeCondition',
      'directives',
      'selectionSet',
    ],
    IntValue: [],
    FloatValue: [],
    StringValue: [],
    BooleanValue: [],
    NullValue: [],
    EnumValue: [],
    ListValue: ['values'],
    ObjectValue: ['fields'],
    ObjectField: ['name', 'value'],
    Directive: ['name', 'arguments'],
    NamedType: ['name'],
    ListType: ['type'],
    NonNullType: ['type'],
    SchemaDefinition: ['description', 'directives', 'operationTypes'],
    OperationTypeDefinition: ['type'],
    ScalarTypeDefinition: ['description', 'name', 'directives'],
    ObjectTypeDefinition: ['description', 'name', 'interfaces', 'directives', 'fields'],
    FieldDefinition: ['description', 'name', 'arguments', 'type', 'directives'],
    InputValueDefinition: ['description', 'name', 'type', 'defaultValue', 'directives'],
    InterfaceTypeDefinition: ['description', 'name', 'interfaces', 'directives', 'fields'],
    UnionTypeDefinition: ['description', 'name', 'directives', 'types'],
    EnumTypeDefinition: ['description', 'name', 'directives', 'values'],
    EnumValueDefinition: ['description', 'name', 'directives'],
    InputObjectTypeDefinition: ['description', 'name', 'directives', 'fields'],
    DirectiveDefinition: ['description', 'name', 'arguments', 'locations'],
    SchemaExtension: ['directives', 'operationTypes'],
    ScalarTypeExtension: ['name', 'directives'],
    ObjectTypeExtension: ['name', 'interfaces', 'directives', 'fields'],
    InterfaceTypeExtension: ['name', 'interfaces', 'directives', 'fields'],
    UnionTypeExtension: ['name', 'directives', 'types'],
    EnumTypeExtension: ['name', 'directives', 'values'],
    InputObjectTypeExtension: ['name', 'directives', 'fields'],
  },
  Rd = new Set(Object.keys(x0));
function Us(r) {
  const e = r?.kind;
  return typeof e == 'string' && Rd.has(e);
}
var sn;
(function (r) {
  (r.QUERY = 'query'), (r.MUTATION = 'mutation'), (r.SUBSCRIPTION = 'subscription');
})(sn || (sn = {}));
var Ba;
(function (r) {
  (r.QUERY = 'QUERY'),
    (r.MUTATION = 'MUTATION'),
    (r.SUBSCRIPTION = 'SUBSCRIPTION'),
    (r.FIELD = 'FIELD'),
    (r.FRAGMENT_DEFINITION = 'FRAGMENT_DEFINITION'),
    (r.FRAGMENT_SPREAD = 'FRAGMENT_SPREAD'),
    (r.INLINE_FRAGMENT = 'INLINE_FRAGMENT'),
    (r.VARIABLE_DEFINITION = 'VARIABLE_DEFINITION'),
    (r.SCHEMA = 'SCHEMA'),
    (r.SCALAR = 'SCALAR'),
    (r.OBJECT = 'OBJECT'),
    (r.FIELD_DEFINITION = 'FIELD_DEFINITION'),
    (r.ARGUMENT_DEFINITION = 'ARGUMENT_DEFINITION'),
    (r.INTERFACE = 'INTERFACE'),
    (r.UNION = 'UNION'),
    (r.ENUM = 'ENUM'),
    (r.ENUM_VALUE = 'ENUM_VALUE'),
    (r.INPUT_OBJECT = 'INPUT_OBJECT'),
    (r.INPUT_FIELD_DEFINITION = 'INPUT_FIELD_DEFINITION');
})(Ba || (Ba = {}));
var fe;
(function (r) {
  (r.NAME = 'Name'),
    (r.DOCUMENT = 'Document'),
    (r.OPERATION_DEFINITION = 'OperationDefinition'),
    (r.VARIABLE_DEFINITION = 'VariableDefinition'),
    (r.SELECTION_SET = 'SelectionSet'),
    (r.FIELD = 'Field'),
    (r.ARGUMENT = 'Argument'),
    (r.FRAGMENT_SPREAD = 'FragmentSpread'),
    (r.INLINE_FRAGMENT = 'InlineFragment'),
    (r.FRAGMENT_DEFINITION = 'FragmentDefinition'),
    (r.VARIABLE = 'Variable'),
    (r.INT = 'IntValue'),
    (r.FLOAT = 'FloatValue'),
    (r.STRING = 'StringValue'),
    (r.BOOLEAN = 'BooleanValue'),
    (r.NULL = 'NullValue'),
    (r.ENUM = 'EnumValue'),
    (r.LIST = 'ListValue'),
    (r.OBJECT = 'ObjectValue'),
    (r.OBJECT_FIELD = 'ObjectField'),
    (r.DIRECTIVE = 'Directive'),
    (r.NAMED_TYPE = 'NamedType'),
    (r.LIST_TYPE = 'ListType'),
    (r.NON_NULL_TYPE = 'NonNullType'),
    (r.SCHEMA_DEFINITION = 'SchemaDefinition'),
    (r.OPERATION_TYPE_DEFINITION = 'OperationTypeDefinition'),
    (r.SCALAR_TYPE_DEFINITION = 'ScalarTypeDefinition'),
    (r.OBJECT_TYPE_DEFINITION = 'ObjectTypeDefinition'),
    (r.FIELD_DEFINITION = 'FieldDefinition'),
    (r.INPUT_VALUE_DEFINITION = 'InputValueDefinition'),
    (r.INTERFACE_TYPE_DEFINITION = 'InterfaceTypeDefinition'),
    (r.UNION_TYPE_DEFINITION = 'UnionTypeDefinition'),
    (r.ENUM_TYPE_DEFINITION = 'EnumTypeDefinition'),
    (r.ENUM_VALUE_DEFINITION = 'EnumValueDefinition'),
    (r.INPUT_OBJECT_TYPE_DEFINITION = 'InputObjectTypeDefinition'),
    (r.DIRECTIVE_DEFINITION = 'DirectiveDefinition'),
    (r.SCHEMA_EXTENSION = 'SchemaExtension'),
    (r.SCALAR_TYPE_EXTENSION = 'ScalarTypeExtension'),
    (r.OBJECT_TYPE_EXTENSION = 'ObjectTypeExtension'),
    (r.INTERFACE_TYPE_EXTENSION = 'InterfaceTypeExtension'),
    (r.UNION_TYPE_EXTENSION = 'UnionTypeExtension'),
    (r.ENUM_TYPE_EXTENSION = 'EnumTypeExtension'),
    (r.INPUT_OBJECT_TYPE_EXTENSION = 'InputObjectTypeExtension');
})(fe || (fe = {}));
function Fa(r) {
  return r === 9 || r === 32;
}
function Jn(r) {
  return r >= 48 && r <= 57;
}
function M0(r) {
  return (r >= 97 && r <= 122) || (r >= 65 && r <= 90);
}
function _0(r) {
  return M0(r) || r === 95;
}
function Nd(r) {
  return M0(r) || Jn(r) || r === 95;
}
function Od(r) {
  var e;
  let t = Number.MAX_SAFE_INTEGER,
    n = null,
    i = -1;
  for (let c = 0; c < r.length; ++c) {
    var a;
    const v = r[c],
      w = Td(v);
    w !== v.length &&
      ((n = (a = n) !== null && a !== void 0 ? a : c), (i = c), c !== 0 && w < t && (t = w));
  }
  return r
    .map((c, v) => (v === 0 ? c : c.slice(t)))
    .slice((e = n) !== null && e !== void 0 ? e : 0, i + 1);
}
function Td(r) {
  let e = 0;
  for (; e < r.length && Fa(r.charCodeAt(e)); ) ++e;
  return e;
}
function Cd(r, e) {
  const t = r.replace(/"""/g, '\\"""'),
    n = t.split(/\r\n|[\n\r]/g),
    i = n.length === 1,
    a = n.length > 1 && n.slice(1).every((E) => E.length === 0 || Fa(E.charCodeAt(0))),
    c = t.endsWith('\\"""'),
    v = r.endsWith('"') && !c,
    w = r.endsWith('\\'),
    y = v || w,
    M = !(e != null && e.minimize) && (!i || r.length > 70 || y || a || c);
  let S = '';
  const I = i && Fa(r.charCodeAt(0));
  return (
    ((M && !I) || a) &&
      (S += `
`),
    (S += t),
    (M || y) &&
      (S += `
`),
    '"""' + S + '"""'
  );
}
var P;
(function (r) {
  (r.SOF = '<SOF>'),
    (r.EOF = '<EOF>'),
    (r.BANG = '!'),
    (r.DOLLAR = '$'),
    (r.AMP = '&'),
    (r.PAREN_L = '('),
    (r.PAREN_R = ')'),
    (r.SPREAD = '...'),
    (r.COLON = ':'),
    (r.EQUALS = '='),
    (r.AT = '@'),
    (r.BRACKET_L = '['),
    (r.BRACKET_R = ']'),
    (r.BRACE_L = '{'),
    (r.PIPE = '|'),
    (r.BRACE_R = '}'),
    (r.NAME = 'Name'),
    (r.INT = 'Int'),
    (r.FLOAT = 'Float'),
    (r.STRING = 'String'),
    (r.BLOCK_STRING = 'BlockString'),
    (r.COMMENT = 'Comment');
})(P || (P = {}));
class Pd {
  constructor(e) {
    const t = new w0(P.SOF, 0, 0, 0, 0);
    (this.source = e),
      (this.lastToken = t),
      (this.token = t),
      (this.line = 1),
      (this.lineStart = 0);
  }
  get [Symbol.toStringTag]() {
    return 'Lexer';
  }
  advance() {
    return (this.lastToken = this.token), (this.token = this.lookahead());
  }
  lookahead() {
    let e = this.token;
    if (e.kind !== P.EOF)
      do
        if (e.next) e = e.next;
        else {
          const t = $d(this, e.end);
          (e.next = t), (t.prev = e), (e = t);
        }
      while (e.kind === P.COMMENT);
    return e;
  }
}
function kd(r) {
  return (
    r === P.BANG ||
    r === P.DOLLAR ||
    r === P.AMP ||
    r === P.PAREN_L ||
    r === P.PAREN_R ||
    r === P.SPREAD ||
    r === P.COLON ||
    r === P.EQUALS ||
    r === P.AT ||
    r === P.BRACKET_L ||
    r === P.BRACKET_R ||
    r === P.BRACE_L ||
    r === P.PIPE ||
    r === P.BRACE_R
  );
}
function En(r) {
  return (r >= 0 && r <= 55295) || (r >= 57344 && r <= 1114111);
}
function Vi(r, e) {
  return S0(r.charCodeAt(e)) && A0(r.charCodeAt(e + 1));
}
function S0(r) {
  return r >= 55296 && r <= 56319;
}
function A0(r) {
  return r >= 56320 && r <= 57343;
}
function Kr(r, e) {
  const t = r.source.body.codePointAt(e);
  if (t === void 0) return P.EOF;
  if (t >= 32 && t <= 126) {
    const n = String.fromCodePoint(t);
    return n === '"' ? `'"'` : `"${n}"`;
  }
  return 'U+' + t.toString(16).toUpperCase().padStart(4, '0');
}
function mt(r, e, t, n, i) {
  const a = r.line,
    c = 1 + t - r.lineStart;
  return new w0(e, t, n, a, c, i);
}
function $d(r, e) {
  const t = r.source.body,
    n = t.length;
  let i = e;
  for (; i < n; ) {
    const a = t.charCodeAt(i);
    switch (a) {
      case 65279:
      case 9:
      case 32:
      case 44:
        ++i;
        continue;
      case 10:
        ++i, ++r.line, (r.lineStart = i);
        continue;
      case 13:
        t.charCodeAt(i + 1) === 10 ? (i += 2) : ++i, ++r.line, (r.lineStart = i);
        continue;
      case 35:
        return Dd(r, i);
      case 33:
        return mt(r, P.BANG, i, i + 1);
      case 36:
        return mt(r, P.DOLLAR, i, i + 1);
      case 38:
        return mt(r, P.AMP, i, i + 1);
      case 40:
        return mt(r, P.PAREN_L, i, i + 1);
      case 41:
        return mt(r, P.PAREN_R, i, i + 1);
      case 46:
        if (t.charCodeAt(i + 1) === 46 && t.charCodeAt(i + 2) === 46)
          return mt(r, P.SPREAD, i, i + 3);
        break;
      case 58:
        return mt(r, P.COLON, i, i + 1);
      case 61:
        return mt(r, P.EQUALS, i, i + 1);
      case 64:
        return mt(r, P.AT, i, i + 1);
      case 91:
        return mt(r, P.BRACKET_L, i, i + 1);
      case 93:
        return mt(r, P.BRACKET_R, i, i + 1);
      case 123:
        return mt(r, P.BRACE_L, i, i + 1);
      case 124:
        return mt(r, P.PIPE, i, i + 1);
      case 125:
        return mt(r, P.BRACE_R, i, i + 1);
      case 34:
        return t.charCodeAt(i + 1) === 34 && t.charCodeAt(i + 2) === 34 ? zd(r, i) : qd(r, i);
    }
    if (Jn(a) || a === 45) return Ld(r, i, a);
    if (_0(a)) return jd(r, i);
    throw Mt(
      r.source,
      i,
      a === 39
        ? `Unexpected single quote character ('), did you mean to use a double quote (")?`
        : En(a) || Vi(t, i)
        ? `Unexpected character: ${Kr(r, i)}.`
        : `Invalid character: ${Kr(r, i)}.`
    );
  }
  return mt(r, P.EOF, n, n);
}
function Dd(r, e) {
  const t = r.source.body,
    n = t.length;
  let i = e + 1;
  for (; i < n; ) {
    const a = t.charCodeAt(i);
    if (a === 10 || a === 13) break;
    if (En(a)) ++i;
    else if (Vi(t, i)) i += 2;
    else break;
  }
  return mt(r, P.COMMENT, e, i, t.slice(e + 1, i));
}
function Ld(r, e, t) {
  const n = r.source.body;
  let i = e,
    a = t,
    c = !1;
  if ((a === 45 && (a = n.charCodeAt(++i)), a === 48)) {
    if (((a = n.charCodeAt(++i)), Jn(a)))
      throw Mt(r.source, i, `Invalid number, unexpected digit after 0: ${Kr(r, i)}.`);
  } else (i = ya(r, i, a)), (a = n.charCodeAt(i));
  if (
    (a === 46 && ((c = !0), (a = n.charCodeAt(++i)), (i = ya(r, i, a)), (a = n.charCodeAt(i))),
    (a === 69 || a === 101) &&
      ((c = !0),
      (a = n.charCodeAt(++i)),
      (a === 43 || a === 45) && (a = n.charCodeAt(++i)),
      (i = ya(r, i, a)),
      (a = n.charCodeAt(i))),
    a === 46 || _0(a))
  )
    throw Mt(r.source, i, `Invalid number, expected digit but got: ${Kr(r, i)}.`);
  return mt(r, c ? P.FLOAT : P.INT, e, i, n.slice(e, i));
}
function ya(r, e, t) {
  if (!Jn(t)) throw Mt(r.source, e, `Invalid number, expected digit but got: ${Kr(r, e)}.`);
  const n = r.source.body;
  let i = e + 1;
  for (; Jn(n.charCodeAt(i)); ) ++i;
  return i;
}
function qd(r, e) {
  const t = r.source.body,
    n = t.length;
  let i = e + 1,
    a = i,
    c = '';
  for (; i < n; ) {
    const v = t.charCodeAt(i);
    if (v === 34) return (c += t.slice(a, i)), mt(r, P.STRING, e, i + 1, c);
    if (v === 92) {
      c += t.slice(a, i);
      const w =
        t.charCodeAt(i + 1) === 117
          ? t.charCodeAt(i + 2) === 123
            ? Bd(r, i)
            : Fd(r, i)
          : Ud(r, i);
      (c += w.value), (i += w.size), (a = i);
      continue;
    }
    if (v === 10 || v === 13) break;
    if (En(v)) ++i;
    else if (Vi(t, i)) i += 2;
    else throw Mt(r.source, i, `Invalid character within String: ${Kr(r, i)}.`);
  }
  throw Mt(r.source, i, 'Unterminated string.');
}
function Bd(r, e) {
  const t = r.source.body;
  let n = 0,
    i = 3;
  for (; i < 12; ) {
    const a = t.charCodeAt(e + i++);
    if (a === 125) {
      if (i < 5 || !En(n)) break;
      return { value: String.fromCodePoint(n), size: i };
    }
    if (((n = (n << 4) | qn(a)), n < 0)) break;
  }
  throw Mt(r.source, e, `Invalid Unicode escape sequence: "${t.slice(e, e + i)}".`);
}
function Fd(r, e) {
  const t = r.source.body,
    n = zs(t, e + 2);
  if (En(n)) return { value: String.fromCodePoint(n), size: 6 };
  if (S0(n) && t.charCodeAt(e + 6) === 92 && t.charCodeAt(e + 7) === 117) {
    const i = zs(t, e + 8);
    if (A0(i)) return { value: String.fromCodePoint(n, i), size: 12 };
  }
  throw Mt(r.source, e, `Invalid Unicode escape sequence: "${t.slice(e, e + 6)}".`);
}
function zs(r, e) {
  return (
    (qn(r.charCodeAt(e)) << 12) |
    (qn(r.charCodeAt(e + 1)) << 8) |
    (qn(r.charCodeAt(e + 2)) << 4) |
    qn(r.charCodeAt(e + 3))
  );
}
function qn(r) {
  return r >= 48 && r <= 57
    ? r - 48
    : r >= 65 && r <= 70
    ? r - 55
    : r >= 97 && r <= 102
    ? r - 87
    : -1;
}
function Ud(r, e) {
  const t = r.source.body;
  switch (t.charCodeAt(e + 1)) {
    case 34:
      return { value: '"', size: 2 };
    case 92:
      return { value: '\\', size: 2 };
    case 47:
      return { value: '/', size: 2 };
    case 98:
      return { value: '\b', size: 2 };
    case 102:
      return { value: '\f', size: 2 };
    case 110:
      return {
        value: `
`,
        size: 2,
      };
    case 114:
      return { value: '\r', size: 2 };
    case 116:
      return { value: '	', size: 2 };
  }
  throw Mt(r.source, e, `Invalid character escape sequence: "${t.slice(e, e + 2)}".`);
}
function zd(r, e) {
  const t = r.source.body,
    n = t.length;
  let i = r.lineStart,
    a = e + 3,
    c = a,
    v = '';
  const w = [];
  for (; a < n; ) {
    const y = t.charCodeAt(a);
    if (y === 34 && t.charCodeAt(a + 1) === 34 && t.charCodeAt(a + 2) === 34) {
      (v += t.slice(c, a)), w.push(v);
      const M = mt(
        r,
        P.BLOCK_STRING,
        e,
        a + 3,
        Od(w).join(`
`)
      );
      return (r.line += w.length - 1), (r.lineStart = i), M;
    }
    if (
      y === 92 &&
      t.charCodeAt(a + 1) === 34 &&
      t.charCodeAt(a + 2) === 34 &&
      t.charCodeAt(a + 3) === 34
    ) {
      (v += t.slice(c, a)), (c = a + 1), (a += 4);
      continue;
    }
    if (y === 10 || y === 13) {
      (v += t.slice(c, a)),
        w.push(v),
        y === 13 && t.charCodeAt(a + 1) === 10 ? (a += 2) : ++a,
        (v = ''),
        (c = a),
        (i = a);
      continue;
    }
    if (En(y)) ++a;
    else if (Vi(t, a)) a += 2;
    else throw Mt(r.source, a, `Invalid character within String: ${Kr(r, a)}.`);
  }
  throw Mt(r.source, a, 'Unterminated string.');
}
function jd(r, e) {
  const t = r.source.body,
    n = t.length;
  let i = e + 1;
  for (; i < n; ) {
    const a = t.charCodeAt(i);
    if (Nd(a)) ++i;
    else break;
  }
  return mt(r, P.NAME, e, i, t.slice(e, i));
}
const Vd = 10,
  E0 = 2;
function I0(r) {
  return Ji(r, []);
}
function Ji(r, e) {
  switch (typeof r) {
    case 'string':
      return JSON.stringify(r);
    case 'function':
      return r.name ? `[function ${r.name}]` : '[function]';
    case 'object':
      return Jd(r, e);
    default:
      return String(r);
  }
}
function Jd(r, e) {
  if (r === null) return 'null';
  if (e.includes(r)) return '[Circular]';
  const t = [...e, r];
  if (Hd(r)) {
    const n = r.toJSON();
    if (n !== r) return typeof n == 'string' ? n : Ji(n, t);
  } else if (Array.isArray(r)) return Gd(r, t);
  return Wd(r, t);
}
function Hd(r) {
  return typeof r.toJSON == 'function';
}
function Wd(r, e) {
  const t = Object.entries(r);
  return t.length === 0
    ? '{}'
    : e.length > E0
    ? '[' + Kd(r) + ']'
    : '{ ' + t.map(([i, a]) => i + ': ' + Ji(a, e)).join(', ') + ' }';
}
function Gd(r, e) {
  if (r.length === 0) return '[]';
  if (e.length > E0) return '[Array]';
  const t = Math.min(Vd, r.length),
    n = r.length - t,
    i = [];
  for (let a = 0; a < t; ++a) i.push(Ji(r[a], e));
  return (
    n === 1 ? i.push('... 1 more item') : n > 1 && i.push(`... ${n} more items`),
    '[' + i.join(', ') + ']'
  );
}
function Kd(r) {
  const e = Object.prototype.toString
    .call(r)
    .replace(/^\[object /, '')
    .replace(/]$/, '');
  if (e === 'Object' && typeof r.constructor == 'function') {
    const t = r.constructor.name;
    if (typeof t == 'string' && t !== '') return t;
  }
  return e;
}
const Xd = function (e, t) {
  return e instanceof t;
};
class R0 {
  constructor(e, t = 'GraphQL request', n = { line: 1, column: 1 }) {
    typeof e == 'string' || yi(!1, `Body must be a string. Received: ${I0(e)}.`),
      (this.body = e),
      (this.name = t),
      (this.locationOffset = n),
      this.locationOffset.line > 0 ||
        yi(!1, 'line in locationOffset is 1-indexed and must be positive.'),
      this.locationOffset.column > 0 ||
        yi(!1, 'column in locationOffset is 1-indexed and must be positive.');
  }
  get [Symbol.toStringTag]() {
    return 'Source';
  }
}
function Zd(r) {
  return Xd(r, R0);
}
function N0(r, e) {
  return new Xn(r, e).parseDocument();
}
function Yd(r, e) {
  const t = new Xn(r, e);
  t.expectToken(P.SOF);
  const n = t.parseValueLiteral(!1);
  return t.expectToken(P.EOF), n;
}
function Qd(r, e) {
  const t = new Xn(r, e);
  t.expectToken(P.SOF);
  const n = t.parseConstValueLiteral();
  return t.expectToken(P.EOF), n;
}
function eh(r, e) {
  const t = new Xn(r, e);
  t.expectToken(P.SOF);
  const n = t.parseTypeReference();
  return t.expectToken(P.EOF), n;
}
class Xn {
  constructor(e, t = {}) {
    const n = Zd(e) ? e : new R0(e);
    (this._lexer = new Pd(n)), (this._options = t), (this._tokenCounter = 0);
  }
  parseName() {
    const e = this.expectToken(P.NAME);
    return this.node(e, { kind: fe.NAME, value: e.value });
  }
  parseDocument() {
    return this.node(this._lexer.token, {
      kind: fe.DOCUMENT,
      definitions: this.many(P.SOF, this.parseDefinition, P.EOF),
    });
  }
  parseDefinition() {
    if (this.peek(P.BRACE_L)) return this.parseOperationDefinition();
    const e = this.peekDescription(),
      t = e ? this._lexer.lookahead() : this._lexer.token;
    if (t.kind === P.NAME) {
      switch (t.value) {
        case 'schema':
          return this.parseSchemaDefinition();
        case 'scalar':
          return this.parseScalarTypeDefinition();
        case 'type':
          return this.parseObjectTypeDefinition();
        case 'interface':
          return this.parseInterfaceTypeDefinition();
        case 'union':
          return this.parseUnionTypeDefinition();
        case 'enum':
          return this.parseEnumTypeDefinition();
        case 'input':
          return this.parseInputObjectTypeDefinition();
        case 'directive':
          return this.parseDirectiveDefinition();
      }
      if (e)
        throw Mt(
          this._lexer.source,
          this._lexer.token.start,
          'Unexpected description, descriptions are supported only on type definitions.'
        );
      switch (t.value) {
        case 'query':
        case 'mutation':
        case 'subscription':
          return this.parseOperationDefinition();
        case 'fragment':
          return this.parseFragmentDefinition();
        case 'extend':
          return this.parseTypeSystemExtension();
      }
    }
    throw this.unexpected(t);
  }
  parseOperationDefinition() {
    const e = this._lexer.token;
    if (this.peek(P.BRACE_L))
      return this.node(e, {
        kind: fe.OPERATION_DEFINITION,
        operation: sn.QUERY,
        name: void 0,
        variableDefinitions: [],
        directives: [],
        selectionSet: this.parseSelectionSet(),
      });
    const t = this.parseOperationType();
    let n;
    return (
      this.peek(P.NAME) && (n = this.parseName()),
      this.node(e, {
        kind: fe.OPERATION_DEFINITION,
        operation: t,
        name: n,
        variableDefinitions: this.parseVariableDefinitions(),
        directives: this.parseDirectives(!1),
        selectionSet: this.parseSelectionSet(),
      })
    );
  }
  parseOperationType() {
    const e = this.expectToken(P.NAME);
    switch (e.value) {
      case 'query':
        return sn.QUERY;
      case 'mutation':
        return sn.MUTATION;
      case 'subscription':
        return sn.SUBSCRIPTION;
    }
    throw this.unexpected(e);
  }
  parseVariableDefinitions() {
    return this.optionalMany(P.PAREN_L, this.parseVariableDefinition, P.PAREN_R);
  }
  parseVariableDefinition() {
    return this.node(this._lexer.token, {
      kind: fe.VARIABLE_DEFINITION,
      variable: this.parseVariable(),
      type: (this.expectToken(P.COLON), this.parseTypeReference()),
      defaultValue: this.expectOptionalToken(P.EQUALS) ? this.parseConstValueLiteral() : void 0,
      directives: this.parseConstDirectives(),
    });
  }
  parseVariable() {
    const e = this._lexer.token;
    return this.expectToken(P.DOLLAR), this.node(e, { kind: fe.VARIABLE, name: this.parseName() });
  }
  parseSelectionSet() {
    return this.node(this._lexer.token, {
      kind: fe.SELECTION_SET,
      selections: this.many(P.BRACE_L, this.parseSelection, P.BRACE_R),
    });
  }
  parseSelection() {
    return this.peek(P.SPREAD) ? this.parseFragment() : this.parseField();
  }
  parseField() {
    const e = this._lexer.token,
      t = this.parseName();
    let n, i;
    return (
      this.expectOptionalToken(P.COLON) ? ((n = t), (i = this.parseName())) : (i = t),
      this.node(e, {
        kind: fe.FIELD,
        alias: n,
        name: i,
        arguments: this.parseArguments(!1),
        directives: this.parseDirectives(!1),
        selectionSet: this.peek(P.BRACE_L) ? this.parseSelectionSet() : void 0,
      })
    );
  }
  parseArguments(e) {
    const t = e ? this.parseConstArgument : this.parseArgument;
    return this.optionalMany(P.PAREN_L, t, P.PAREN_R);
  }
  parseArgument(e = !1) {
    const t = this._lexer.token,
      n = this.parseName();
    return (
      this.expectToken(P.COLON),
      this.node(t, { kind: fe.ARGUMENT, name: n, value: this.parseValueLiteral(e) })
    );
  }
  parseConstArgument() {
    return this.parseArgument(!0);
  }
  parseFragment() {
    const e = this._lexer.token;
    this.expectToken(P.SPREAD);
    const t = this.expectOptionalKeyword('on');
    return !t && this.peek(P.NAME)
      ? this.node(e, {
          kind: fe.FRAGMENT_SPREAD,
          name: this.parseFragmentName(),
          directives: this.parseDirectives(!1),
        })
      : this.node(e, {
          kind: fe.INLINE_FRAGMENT,
          typeCondition: t ? this.parseNamedType() : void 0,
          directives: this.parseDirectives(!1),
          selectionSet: this.parseSelectionSet(),
        });
  }
  parseFragmentDefinition() {
    const e = this._lexer.token;
    return (
      this.expectKeyword('fragment'),
      this._options.allowLegacyFragmentVariables === !0
        ? this.node(e, {
            kind: fe.FRAGMENT_DEFINITION,
            name: this.parseFragmentName(),
            variableDefinitions: this.parseVariableDefinitions(),
            typeCondition: (this.expectKeyword('on'), this.parseNamedType()),
            directives: this.parseDirectives(!1),
            selectionSet: this.parseSelectionSet(),
          })
        : this.node(e, {
            kind: fe.FRAGMENT_DEFINITION,
            name: this.parseFragmentName(),
            typeCondition: (this.expectKeyword('on'), this.parseNamedType()),
            directives: this.parseDirectives(!1),
            selectionSet: this.parseSelectionSet(),
          })
    );
  }
  parseFragmentName() {
    if (this._lexer.token.value === 'on') throw this.unexpected();
    return this.parseName();
  }
  parseValueLiteral(e) {
    const t = this._lexer.token;
    switch (t.kind) {
      case P.BRACKET_L:
        return this.parseList(e);
      case P.BRACE_L:
        return this.parseObject(e);
      case P.INT:
        return this.advanceLexer(), this.node(t, { kind: fe.INT, value: t.value });
      case P.FLOAT:
        return this.advanceLexer(), this.node(t, { kind: fe.FLOAT, value: t.value });
      case P.STRING:
      case P.BLOCK_STRING:
        return this.parseStringLiteral();
      case P.NAME:
        switch ((this.advanceLexer(), t.value)) {
          case 'true':
            return this.node(t, { kind: fe.BOOLEAN, value: !0 });
          case 'false':
            return this.node(t, { kind: fe.BOOLEAN, value: !1 });
          case 'null':
            return this.node(t, { kind: fe.NULL });
          default:
            return this.node(t, { kind: fe.ENUM, value: t.value });
        }
      case P.DOLLAR:
        if (e)
          if ((this.expectToken(P.DOLLAR), this._lexer.token.kind === P.NAME)) {
            const n = this._lexer.token.value;
            throw Mt(this._lexer.source, t.start, `Unexpected variable "$${n}" in constant value.`);
          } else throw this.unexpected(t);
        return this.parseVariable();
      default:
        throw this.unexpected();
    }
  }
  parseConstValueLiteral() {
    return this.parseValueLiteral(!0);
  }
  parseStringLiteral() {
    const e = this._lexer.token;
    return (
      this.advanceLexer(),
      this.node(e, { kind: fe.STRING, value: e.value, block: e.kind === P.BLOCK_STRING })
    );
  }
  parseList(e) {
    const t = () => this.parseValueLiteral(e);
    return this.node(this._lexer.token, {
      kind: fe.LIST,
      values: this.any(P.BRACKET_L, t, P.BRACKET_R),
    });
  }
  parseObject(e) {
    const t = () => this.parseObjectField(e);
    return this.node(this._lexer.token, {
      kind: fe.OBJECT,
      fields: this.any(P.BRACE_L, t, P.BRACE_R),
    });
  }
  parseObjectField(e) {
    const t = this._lexer.token,
      n = this.parseName();
    return (
      this.expectToken(P.COLON),
      this.node(t, { kind: fe.OBJECT_FIELD, name: n, value: this.parseValueLiteral(e) })
    );
  }
  parseDirectives(e) {
    const t = [];
    for (; this.peek(P.AT); ) t.push(this.parseDirective(e));
    return t;
  }
  parseConstDirectives() {
    return this.parseDirectives(!0);
  }
  parseDirective(e) {
    const t = this._lexer.token;
    return (
      this.expectToken(P.AT),
      this.node(t, {
        kind: fe.DIRECTIVE,
        name: this.parseName(),
        arguments: this.parseArguments(e),
      })
    );
  }
  parseTypeReference() {
    const e = this._lexer.token;
    let t;
    if (this.expectOptionalToken(P.BRACKET_L)) {
      const n = this.parseTypeReference();
      this.expectToken(P.BRACKET_R), (t = this.node(e, { kind: fe.LIST_TYPE, type: n }));
    } else t = this.parseNamedType();
    return this.expectOptionalToken(P.BANG) ? this.node(e, { kind: fe.NON_NULL_TYPE, type: t }) : t;
  }
  parseNamedType() {
    return this.node(this._lexer.token, { kind: fe.NAMED_TYPE, name: this.parseName() });
  }
  peekDescription() {
    return this.peek(P.STRING) || this.peek(P.BLOCK_STRING);
  }
  parseDescription() {
    if (this.peekDescription()) return this.parseStringLiteral();
  }
  parseSchemaDefinition() {
    const e = this._lexer.token,
      t = this.parseDescription();
    this.expectKeyword('schema');
    const n = this.parseConstDirectives(),
      i = this.many(P.BRACE_L, this.parseOperationTypeDefinition, P.BRACE_R);
    return this.node(e, {
      kind: fe.SCHEMA_DEFINITION,
      description: t,
      directives: n,
      operationTypes: i,
    });
  }
  parseOperationTypeDefinition() {
    const e = this._lexer.token,
      t = this.parseOperationType();
    this.expectToken(P.COLON);
    const n = this.parseNamedType();
    return this.node(e, { kind: fe.OPERATION_TYPE_DEFINITION, operation: t, type: n });
  }
  parseScalarTypeDefinition() {
    const e = this._lexer.token,
      t = this.parseDescription();
    this.expectKeyword('scalar');
    const n = this.parseName(),
      i = this.parseConstDirectives();
    return this.node(e, {
      kind: fe.SCALAR_TYPE_DEFINITION,
      description: t,
      name: n,
      directives: i,
    });
  }
  parseObjectTypeDefinition() {
    const e = this._lexer.token,
      t = this.parseDescription();
    this.expectKeyword('type');
    const n = this.parseName(),
      i = this.parseImplementsInterfaces(),
      a = this.parseConstDirectives(),
      c = this.parseFieldsDefinition();
    return this.node(e, {
      kind: fe.OBJECT_TYPE_DEFINITION,
      description: t,
      name: n,
      interfaces: i,
      directives: a,
      fields: c,
    });
  }
  parseImplementsInterfaces() {
    return this.expectOptionalKeyword('implements')
      ? this.delimitedMany(P.AMP, this.parseNamedType)
      : [];
  }
  parseFieldsDefinition() {
    return this.optionalMany(P.BRACE_L, this.parseFieldDefinition, P.BRACE_R);
  }
  parseFieldDefinition() {
    const e = this._lexer.token,
      t = this.parseDescription(),
      n = this.parseName(),
      i = this.parseArgumentDefs();
    this.expectToken(P.COLON);
    const a = this.parseTypeReference(),
      c = this.parseConstDirectives();
    return this.node(e, {
      kind: fe.FIELD_DEFINITION,
      description: t,
      name: n,
      arguments: i,
      type: a,
      directives: c,
    });
  }
  parseArgumentDefs() {
    return this.optionalMany(P.PAREN_L, this.parseInputValueDef, P.PAREN_R);
  }
  parseInputValueDef() {
    const e = this._lexer.token,
      t = this.parseDescription(),
      n = this.parseName();
    this.expectToken(P.COLON);
    const i = this.parseTypeReference();
    let a;
    this.expectOptionalToken(P.EQUALS) && (a = this.parseConstValueLiteral());
    const c = this.parseConstDirectives();
    return this.node(e, {
      kind: fe.INPUT_VALUE_DEFINITION,
      description: t,
      name: n,
      type: i,
      defaultValue: a,
      directives: c,
    });
  }
  parseInterfaceTypeDefinition() {
    const e = this._lexer.token,
      t = this.parseDescription();
    this.expectKeyword('interface');
    const n = this.parseName(),
      i = this.parseImplementsInterfaces(),
      a = this.parseConstDirectives(),
      c = this.parseFieldsDefinition();
    return this.node(e, {
      kind: fe.INTERFACE_TYPE_DEFINITION,
      description: t,
      name: n,
      interfaces: i,
      directives: a,
      fields: c,
    });
  }
  parseUnionTypeDefinition() {
    const e = this._lexer.token,
      t = this.parseDescription();
    this.expectKeyword('union');
    const n = this.parseName(),
      i = this.parseConstDirectives(),
      a = this.parseUnionMemberTypes();
    return this.node(e, {
      kind: fe.UNION_TYPE_DEFINITION,
      description: t,
      name: n,
      directives: i,
      types: a,
    });
  }
  parseUnionMemberTypes() {
    return this.expectOptionalToken(P.EQUALS)
      ? this.delimitedMany(P.PIPE, this.parseNamedType)
      : [];
  }
  parseEnumTypeDefinition() {
    const e = this._lexer.token,
      t = this.parseDescription();
    this.expectKeyword('enum');
    const n = this.parseName(),
      i = this.parseConstDirectives(),
      a = this.parseEnumValuesDefinition();
    return this.node(e, {
      kind: fe.ENUM_TYPE_DEFINITION,
      description: t,
      name: n,
      directives: i,
      values: a,
    });
  }
  parseEnumValuesDefinition() {
    return this.optionalMany(P.BRACE_L, this.parseEnumValueDefinition, P.BRACE_R);
  }
  parseEnumValueDefinition() {
    const e = this._lexer.token,
      t = this.parseDescription(),
      n = this.parseEnumValueName(),
      i = this.parseConstDirectives();
    return this.node(e, { kind: fe.ENUM_VALUE_DEFINITION, description: t, name: n, directives: i });
  }
  parseEnumValueName() {
    if (
      this._lexer.token.value === 'true' ||
      this._lexer.token.value === 'false' ||
      this._lexer.token.value === 'null'
    )
      throw Mt(
        this._lexer.source,
        this._lexer.token.start,
        `${di(this._lexer.token)} is reserved and cannot be used for an enum value.`
      );
    return this.parseName();
  }
  parseInputObjectTypeDefinition() {
    const e = this._lexer.token,
      t = this.parseDescription();
    this.expectKeyword('input');
    const n = this.parseName(),
      i = this.parseConstDirectives(),
      a = this.parseInputFieldsDefinition();
    return this.node(e, {
      kind: fe.INPUT_OBJECT_TYPE_DEFINITION,
      description: t,
      name: n,
      directives: i,
      fields: a,
    });
  }
  parseInputFieldsDefinition() {
    return this.optionalMany(P.BRACE_L, this.parseInputValueDef, P.BRACE_R);
  }
  parseTypeSystemExtension() {
    const e = this._lexer.lookahead();
    if (e.kind === P.NAME)
      switch (e.value) {
        case 'schema':
          return this.parseSchemaExtension();
        case 'scalar':
          return this.parseScalarTypeExtension();
        case 'type':
          return this.parseObjectTypeExtension();
        case 'interface':
          return this.parseInterfaceTypeExtension();
        case 'union':
          return this.parseUnionTypeExtension();
        case 'enum':
          return this.parseEnumTypeExtension();
        case 'input':
          return this.parseInputObjectTypeExtension();
      }
    throw this.unexpected(e);
  }
  parseSchemaExtension() {
    const e = this._lexer.token;
    this.expectKeyword('extend'), this.expectKeyword('schema');
    const t = this.parseConstDirectives(),
      n = this.optionalMany(P.BRACE_L, this.parseOperationTypeDefinition, P.BRACE_R);
    if (t.length === 0 && n.length === 0) throw this.unexpected();
    return this.node(e, { kind: fe.SCHEMA_EXTENSION, directives: t, operationTypes: n });
  }
  parseScalarTypeExtension() {
    const e = this._lexer.token;
    this.expectKeyword('extend'), this.expectKeyword('scalar');
    const t = this.parseName(),
      n = this.parseConstDirectives();
    if (n.length === 0) throw this.unexpected();
    return this.node(e, { kind: fe.SCALAR_TYPE_EXTENSION, name: t, directives: n });
  }
  parseObjectTypeExtension() {
    const e = this._lexer.token;
    this.expectKeyword('extend'), this.expectKeyword('type');
    const t = this.parseName(),
      n = this.parseImplementsInterfaces(),
      i = this.parseConstDirectives(),
      a = this.parseFieldsDefinition();
    if (n.length === 0 && i.length === 0 && a.length === 0) throw this.unexpected();
    return this.node(e, {
      kind: fe.OBJECT_TYPE_EXTENSION,
      name: t,
      interfaces: n,
      directives: i,
      fields: a,
    });
  }
  parseInterfaceTypeExtension() {
    const e = this._lexer.token;
    this.expectKeyword('extend'), this.expectKeyword('interface');
    const t = this.parseName(),
      n = this.parseImplementsInterfaces(),
      i = this.parseConstDirectives(),
      a = this.parseFieldsDefinition();
    if (n.length === 0 && i.length === 0 && a.length === 0) throw this.unexpected();
    return this.node(e, {
      kind: fe.INTERFACE_TYPE_EXTENSION,
      name: t,
      interfaces: n,
      directives: i,
      fields: a,
    });
  }
  parseUnionTypeExtension() {
    const e = this._lexer.token;
    this.expectKeyword('extend'), this.expectKeyword('union');
    const t = this.parseName(),
      n = this.parseConstDirectives(),
      i = this.parseUnionMemberTypes();
    if (n.length === 0 && i.length === 0) throw this.unexpected();
    return this.node(e, { kind: fe.UNION_TYPE_EXTENSION, name: t, directives: n, types: i });
  }
  parseEnumTypeExtension() {
    const e = this._lexer.token;
    this.expectKeyword('extend'), this.expectKeyword('enum');
    const t = this.parseName(),
      n = this.parseConstDirectives(),
      i = this.parseEnumValuesDefinition();
    if (n.length === 0 && i.length === 0) throw this.unexpected();
    return this.node(e, { kind: fe.ENUM_TYPE_EXTENSION, name: t, directives: n, values: i });
  }
  parseInputObjectTypeExtension() {
    const e = this._lexer.token;
    this.expectKeyword('extend'), this.expectKeyword('input');
    const t = this.parseName(),
      n = this.parseConstDirectives(),
      i = this.parseInputFieldsDefinition();
    if (n.length === 0 && i.length === 0) throw this.unexpected();
    return this.node(e, {
      kind: fe.INPUT_OBJECT_TYPE_EXTENSION,
      name: t,
      directives: n,
      fields: i,
    });
  }
  parseDirectiveDefinition() {
    const e = this._lexer.token,
      t = this.parseDescription();
    this.expectKeyword('directive'), this.expectToken(P.AT);
    const n = this.parseName(),
      i = this.parseArgumentDefs(),
      a = this.expectOptionalKeyword('repeatable');
    this.expectKeyword('on');
    const c = this.parseDirectiveLocations();
    return this.node(e, {
      kind: fe.DIRECTIVE_DEFINITION,
      description: t,
      name: n,
      arguments: i,
      repeatable: a,
      locations: c,
    });
  }
  parseDirectiveLocations() {
    return this.delimitedMany(P.PIPE, this.parseDirectiveLocation);
  }
  parseDirectiveLocation() {
    const e = this._lexer.token,
      t = this.parseName();
    if (Object.prototype.hasOwnProperty.call(Ba, t.value)) return t;
    throw this.unexpected(e);
  }
  node(e, t) {
    return (
      this._options.noLocation !== !0 &&
        (t.loc = new Id(e, this._lexer.lastToken, this._lexer.source)),
      t
    );
  }
  peek(e) {
    return this._lexer.token.kind === e;
  }
  expectToken(e) {
    const t = this._lexer.token;
    if (t.kind === e) return this.advanceLexer(), t;
    throw Mt(this._lexer.source, t.start, `Expected ${O0(e)}, found ${di(t)}.`);
  }
  expectOptionalToken(e) {
    return this._lexer.token.kind === e ? (this.advanceLexer(), !0) : !1;
  }
  expectKeyword(e) {
    const t = this._lexer.token;
    if (t.kind === P.NAME && t.value === e) this.advanceLexer();
    else throw Mt(this._lexer.source, t.start, `Expected "${e}", found ${di(t)}.`);
  }
  expectOptionalKeyword(e) {
    const t = this._lexer.token;
    return t.kind === P.NAME && t.value === e ? (this.advanceLexer(), !0) : !1;
  }
  unexpected(e) {
    const t = e ?? this._lexer.token;
    return Mt(this._lexer.source, t.start, `Unexpected ${di(t)}.`);
  }
  any(e, t, n) {
    this.expectToken(e);
    const i = [];
    for (; !this.expectOptionalToken(n); ) i.push(t.call(this));
    return i;
  }
  optionalMany(e, t, n) {
    if (this.expectOptionalToken(e)) {
      const i = [];
      do i.push(t.call(this));
      while (!this.expectOptionalToken(n));
      return i;
    }
    return [];
  }
  many(e, t, n) {
    this.expectToken(e);
    const i = [];
    do i.push(t.call(this));
    while (!this.expectOptionalToken(n));
    return i;
  }
  delimitedMany(e, t) {
    this.expectOptionalToken(e);
    const n = [];
    do n.push(t.call(this));
    while (this.expectOptionalToken(e));
    return n;
  }
  advanceLexer() {
    const { maxTokens: e } = this._options,
      t = this._lexer.advance();
    if (e !== void 0 && t.kind !== P.EOF && (++this._tokenCounter, this._tokenCounter > e))
      throw Mt(
        this._lexer.source,
        t.start,
        `Document contains more that ${e} tokens. Parsing aborted.`
      );
  }
}
function di(r) {
  const e = r.value;
  return O0(r.kind) + (e != null ? ` "${e}"` : '');
}
function O0(r) {
  return kd(r) ? `"${r}"` : r;
}
const th = Object.freeze(
  Object.defineProperty(
    { __proto__: null, parse: N0, parseValue: Yd, parseConstValue: Qd, parseType: eh, Parser: Xn },
    Symbol.toStringTag,
    { value: 'Module' }
  )
);
function rh(r) {
  return `"${r.replace(nh, ih)}"`;
}
const nh = /[\x00-\x1f\x22\x5c\x7f-\x9f]/g;
function ih(r) {
  return ah[r.charCodeAt(0)];
}
const ah = [
    '\\u0000',
    '\\u0001',
    '\\u0002',
    '\\u0003',
    '\\u0004',
    '\\u0005',
    '\\u0006',
    '\\u0007',
    '\\b',
    '\\t',
    '\\n',
    '\\u000B',
    '\\f',
    '\\r',
    '\\u000E',
    '\\u000F',
    '\\u0010',
    '\\u0011',
    '\\u0012',
    '\\u0013',
    '\\u0014',
    '\\u0015',
    '\\u0016',
    '\\u0017',
    '\\u0018',
    '\\u0019',
    '\\u001A',
    '\\u001B',
    '\\u001C',
    '\\u001D',
    '\\u001E',
    '\\u001F',
    '',
    '',
    '\\"',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '\\\\',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '\\u007F',
    '\\u0080',
    '\\u0081',
    '\\u0082',
    '\\u0083',
    '\\u0084',
    '\\u0085',
    '\\u0086',
    '\\u0087',
    '\\u0088',
    '\\u0089',
    '\\u008A',
    '\\u008B',
    '\\u008C',
    '\\u008D',
    '\\u008E',
    '\\u008F',
    '\\u0090',
    '\\u0091',
    '\\u0092',
    '\\u0093',
    '\\u0094',
    '\\u0095',
    '\\u0096',
    '\\u0097',
    '\\u0098',
    '\\u0099',
    '\\u009A',
    '\\u009B',
    '\\u009C',
    '\\u009D',
    '\\u009E',
    '\\u009F',
  ],
  sh = Object.freeze({});
function oh(r, e, t = x0) {
  const n = new Map();
  for (const q of Object.values(fe)) n.set(q, fh(e, q));
  let i,
    a = Array.isArray(r),
    c = [r],
    v = -1,
    w = [],
    y = r,
    M,
    S;
  const I = [],
    E = [];
  do {
    v++;
    const q = v === c.length,
      Y = q && w.length !== 0;
    if (q) {
      if (((M = E.length === 0 ? void 0 : I[I.length - 1]), (y = S), (S = E.pop()), Y))
        if (a) {
          y = y.slice();
          let de = 0;
          for (const [H, F] of w) {
            const J = H - de;
            F === null ? (y.splice(J, 1), de++) : (y[J] = F);
          }
        } else {
          y = Object.defineProperties({}, Object.getOwnPropertyDescriptors(y));
          for (const [de, H] of w) y[de] = H;
        }
      (v = i.index), (c = i.keys), (w = i.edits), (a = i.inArray), (i = i.prev);
    } else if (S) {
      if (((M = a ? v : c[v]), (y = S[M]), y == null)) continue;
      I.push(M);
    }
    let Se;
    if (!Array.isArray(y)) {
      var R, T;
      Us(y) || yi(!1, `Invalid AST Node: ${I0(y)}.`);
      const de = q
        ? (R = n.get(y.kind)) === null || R === void 0
          ? void 0
          : R.leave
        : (T = n.get(y.kind)) === null || T === void 0
        ? void 0
        : T.enter;
      if (((Se = de?.call(e, y, M, S, I, E)), Se === sh)) break;
      if (Se === !1) {
        if (!q) {
          I.pop();
          continue;
        }
      } else if (Se !== void 0 && (w.push([M, Se]), !q))
        if (Us(Se)) y = Se;
        else {
          I.pop();
          continue;
        }
    }
    if ((Se === void 0 && Y && w.push([M, y]), q)) I.pop();
    else {
      var z;
      (i = { inArray: a, index: v, keys: c, edits: w, prev: i }),
        (a = Array.isArray(y)),
        (c = a ? y : (z = t[y.kind]) !== null && z !== void 0 ? z : []),
        (v = -1),
        (w = []),
        S && E.push(S),
        (S = y);
    }
  } while (i !== void 0);
  return w.length !== 0 ? w[w.length - 1][1] : r;
}
function fh(r, e) {
  const t = r[e];
  return typeof t == 'object'
    ? t
    : typeof t == 'function'
    ? { enter: t, leave: void 0 }
    : { enter: r.enter, leave: r.leave };
}
function ch(r) {
  return oh(r, dh);
}
const uh = 80,
  dh = {
    Name: { leave: (r) => r.value },
    Variable: { leave: (r) => '$' + r.name },
    Document: {
      leave: (r) =>
        ee(
          r.definitions,
          `

`
        ),
    },
    OperationDefinition: {
      leave(r) {
        const e = Ne('(', ee(r.variableDefinitions, ', '), ')'),
          t = ee([r.operation, ee([r.name, e]), ee(r.directives, ' ')], ' ');
        return (t === 'query' ? '' : t + ' ') + r.selectionSet;
      },
    },
    VariableDefinition: {
      leave: ({ variable: r, type: e, defaultValue: t, directives: n }) =>
        r + ': ' + e + Ne(' = ', t) + Ne(' ', ee(n, ' ')),
    },
    SelectionSet: { leave: ({ selections: r }) => Yt(r) },
    Field: {
      leave({ alias: r, name: e, arguments: t, directives: n, selectionSet: i }) {
        const a = Ne('', r, ': ') + e;
        let c = a + Ne('(', ee(t, ', '), ')');
        return (
          c.length > uh &&
            (c =
              a +
              Ne(
                `(
`,
                wi(
                  ee(
                    t,
                    `
`
                  )
                ),
                `
)`
              )),
          ee([c, ee(n, ' '), i], ' ')
        );
      },
    },
    Argument: { leave: ({ name: r, value: e }) => r + ': ' + e },
    FragmentSpread: { leave: ({ name: r, directives: e }) => '...' + r + Ne(' ', ee(e, ' ')) },
    InlineFragment: {
      leave: ({ typeCondition: r, directives: e, selectionSet: t }) =>
        ee(['...', Ne('on ', r), ee(e, ' '), t], ' '),
    },
    FragmentDefinition: {
      leave: ({
        name: r,
        typeCondition: e,
        variableDefinitions: t,
        directives: n,
        selectionSet: i,
      }) => `fragment ${r}${Ne('(', ee(t, ', '), ')')} on ${e} ${Ne('', ee(n, ' '), ' ')}` + i,
    },
    IntValue: { leave: ({ value: r }) => r },
    FloatValue: { leave: ({ value: r }) => r },
    StringValue: { leave: ({ value: r, block: e }) => (e ? Cd(r) : rh(r)) },
    BooleanValue: { leave: ({ value: r }) => (r ? 'true' : 'false') },
    NullValue: { leave: () => 'null' },
    EnumValue: { leave: ({ value: r }) => r },
    ListValue: { leave: ({ values: r }) => '[' + ee(r, ', ') + ']' },
    ObjectValue: { leave: ({ fields: r }) => '{' + ee(r, ', ') + '}' },
    ObjectField: { leave: ({ name: r, value: e }) => r + ': ' + e },
    Directive: { leave: ({ name: r, arguments: e }) => '@' + r + Ne('(', ee(e, ', '), ')') },
    NamedType: { leave: ({ name: r }) => r },
    ListType: { leave: ({ type: r }) => '[' + r + ']' },
    NonNullType: { leave: ({ type: r }) => r + '!' },
    SchemaDefinition: {
      leave: ({ description: r, directives: e, operationTypes: t }) =>
        Ne(
          '',
          r,
          `
`
        ) + ee(['schema', ee(e, ' '), Yt(t)], ' '),
    },
    OperationTypeDefinition: { leave: ({ operation: r, type: e }) => r + ': ' + e },
    ScalarTypeDefinition: {
      leave: ({ description: r, name: e, directives: t }) =>
        Ne(
          '',
          r,
          `
`
        ) + ee(['scalar', e, ee(t, ' ')], ' '),
    },
    ObjectTypeDefinition: {
      leave: ({ description: r, name: e, interfaces: t, directives: n, fields: i }) =>
        Ne(
          '',
          r,
          `
`
        ) + ee(['type', e, Ne('implements ', ee(t, ' & ')), ee(n, ' '), Yt(i)], ' '),
    },
    FieldDefinition: {
      leave: ({ description: r, name: e, arguments: t, type: n, directives: i }) =>
        Ne(
          '',
          r,
          `
`
        ) +
        e +
        (js(t)
          ? Ne(
              `(
`,
              wi(
                ee(
                  t,
                  `
`
                )
              ),
              `
)`
            )
          : Ne('(', ee(t, ', '), ')')) +
        ': ' +
        n +
        Ne(' ', ee(i, ' ')),
    },
    InputValueDefinition: {
      leave: ({ description: r, name: e, type: t, defaultValue: n, directives: i }) =>
        Ne(
          '',
          r,
          `
`
        ) + ee([e + ': ' + t, Ne('= ', n), ee(i, ' ')], ' '),
    },
    InterfaceTypeDefinition: {
      leave: ({ description: r, name: e, interfaces: t, directives: n, fields: i }) =>
        Ne(
          '',
          r,
          `
`
        ) + ee(['interface', e, Ne('implements ', ee(t, ' & ')), ee(n, ' '), Yt(i)], ' '),
    },
    UnionTypeDefinition: {
      leave: ({ description: r, name: e, directives: t, types: n }) =>
        Ne(
          '',
          r,
          `
`
        ) + ee(['union', e, ee(t, ' '), Ne('= ', ee(n, ' | '))], ' '),
    },
    EnumTypeDefinition: {
      leave: ({ description: r, name: e, directives: t, values: n }) =>
        Ne(
          '',
          r,
          `
`
        ) + ee(['enum', e, ee(t, ' '), Yt(n)], ' '),
    },
    EnumValueDefinition: {
      leave: ({ description: r, name: e, directives: t }) =>
        Ne(
          '',
          r,
          `
`
        ) + ee([e, ee(t, ' ')], ' '),
    },
    InputObjectTypeDefinition: {
      leave: ({ description: r, name: e, directives: t, fields: n }) =>
        Ne(
          '',
          r,
          `
`
        ) + ee(['input', e, ee(t, ' '), Yt(n)], ' '),
    },
    DirectiveDefinition: {
      leave: ({ description: r, name: e, arguments: t, repeatable: n, locations: i }) =>
        Ne(
          '',
          r,
          `
`
        ) +
        'directive @' +
        e +
        (js(t)
          ? Ne(
              `(
`,
              wi(
                ee(
                  t,
                  `
`
                )
              ),
              `
)`
            )
          : Ne('(', ee(t, ', '), ')')) +
        (n ? ' repeatable' : '') +
        ' on ' +
        ee(i, ' | '),
    },
    SchemaExtension: {
      leave: ({ directives: r, operationTypes: e }) =>
        ee(['extend schema', ee(r, ' '), Yt(e)], ' '),
    },
    ScalarTypeExtension: {
      leave: ({ name: r, directives: e }) => ee(['extend scalar', r, ee(e, ' ')], ' '),
    },
    ObjectTypeExtension: {
      leave: ({ name: r, interfaces: e, directives: t, fields: n }) =>
        ee(['extend type', r, Ne('implements ', ee(e, ' & ')), ee(t, ' '), Yt(n)], ' '),
    },
    InterfaceTypeExtension: {
      leave: ({ name: r, interfaces: e, directives: t, fields: n }) =>
        ee(['extend interface', r, Ne('implements ', ee(e, ' & ')), ee(t, ' '), Yt(n)], ' '),
    },
    UnionTypeExtension: {
      leave: ({ name: r, directives: e, types: t }) =>
        ee(['extend union', r, ee(e, ' '), Ne('= ', ee(t, ' | '))], ' '),
    },
    EnumTypeExtension: {
      leave: ({ name: r, directives: e, values: t }) =>
        ee(['extend enum', r, ee(e, ' '), Yt(t)], ' '),
    },
    InputObjectTypeExtension: {
      leave: ({ name: r, directives: e, fields: t }) =>
        ee(['extend input', r, ee(e, ' '), Yt(t)], ' '),
    },
  };
function ee(r, e = '') {
  var t;
  return (t = r?.filter((n) => n).join(e)) !== null && t !== void 0 ? t : '';
}
function Yt(r) {
  return Ne(
    `{
`,
    wi(
      ee(
        r,
        `
`
      )
    ),
    `
}`
  );
}
function Ne(r, e, t = '') {
  return e != null && e !== '' ? r + e + t : '';
}
function wi(r) {
  return Ne(
    '  ',
    r.replace(
      /\n/g,
      `
  `
    )
  );
}
function js(r) {
  var e;
  return (e = r?.some((t) =>
    t.includes(`
`)
  )) !== null && e !== void 0
    ? e
    : !1;
}
const hh = Object.freeze(
  Object.defineProperty({ __proto__: null, print: ch }, Symbol.toStringTag, { value: 'Module' })
);
var xi = new Map(),
  Ua = new Map(),
  T0 = !0,
  Oi = !1;
function C0(r) {
  return r.replace(/[\s,]+/g, ' ').trim();
}
function lh(r) {
  return C0(r.source.body.substring(r.start, r.end));
}
function ph(r) {
  var e = new Set(),
    t = [];
  return (
    r.definitions.forEach(function (n) {
      if (n.kind === 'FragmentDefinition') {
        var i = n.name.value,
          a = lh(n.loc),
          c = Ua.get(i);
        c && !c.has(a)
          ? T0 &&
            console.warn(
              'Warning: fragment with name ' +
                i +
                ` already exists.
graphql-tag enforces all fragment names across your application to be unique; read more about
this in the docs: http://dev.apollodata.com/core/fragments.html#unique-names`
            )
          : c || Ua.set(i, (c = new Set())),
          c.add(a),
          e.has(a) || (e.add(a), t.push(n));
      } else t.push(n);
    }),
    Ni(Ni({}, r), { definitions: t })
  );
}
function vh(r) {
  var e = new Set(r.definitions);
  e.forEach(function (n) {
    n.loc && delete n.loc,
      Object.keys(n).forEach(function (i) {
        var a = n[i];
        a && typeof a == 'object' && e.add(a);
      });
  });
  var t = r.loc;
  return t && (delete t.startToken, delete t.endToken), r;
}
function bh(r) {
  var e = C0(r);
  if (!xi.has(e)) {
    var t = N0(r, { experimentalFragmentVariables: Oi, allowLegacyFragmentVariables: Oi });
    if (!t || t.kind !== 'Document') throw new Error('Not a valid GraphQL document.');
    xi.set(e, vh(ph(t)));
  }
  return xi.get(e);
}
function mn(r) {
  for (var e = [], t = 1; t < arguments.length; t++) e[t - 1] = arguments[t];
  typeof r == 'string' && (r = [r]);
  var n = r[0];
  return (
    e.forEach(function (i, a) {
      i && i.kind === 'Document' ? (n += i.loc.source.body) : (n += i), (n += r[a + 1]);
    }),
    bh(n)
  );
}
function mh() {
  xi.clear(), Ua.clear();
}
function gh() {
  T0 = !1;
}
function yh() {
  Oi = !0;
}
function wh() {
  Oi = !1;
}
var kn = {
  gql: mn,
  resetCaches: mh,
  disableFragmentWarnings: gh,
  enableExperimentalFragmentVariables: yh,
  disableExperimentalFragmentVariables: wh,
};
(function (r) {
  (r.gql = kn.gql),
    (r.resetCaches = kn.resetCaches),
    (r.disableFragmentWarnings = kn.disableFragmentWarnings),
    (r.enableExperimentalFragmentVariables = kn.enableExperimentalFragmentVariables),
    (r.disableExperimentalFragmentVariables = kn.disableExperimentalFragmentVariables);
})(mn || (mn = {}));
mn.default = mn;
const Pe = mn;
var P0 = (r, e, t) => {
    if (!e.has(r)) throw TypeError('Cannot ' + t);
  },
  hi = (r, e, t) => (P0(r, e, 'read from private field'), t ? t.call(r) : e.get(r)),
  xh = (r, e, t) => {
    if (e.has(r)) throw TypeError('Cannot add the same private member more than once');
    e instanceof WeakSet ? e.add(r) : e.set(r, t);
  },
  Mh = (r, e, t, n) => (P0(r, e, 'write to private field'), n ? n.call(r, t) : e.set(r, t), t),
  zr,
  et = class extends Ie {
    constructor(e) {
      let t = (8 - (e % 8)) % 8,
        n = e + t;
      super('ByteArray', `[u64; ${n / 4}]`, n),
        xh(this, zr, void 0),
        (this.length = e),
        Mh(this, zr, t);
    }
    encode(e) {
      let t = [],
        n = V(e);
      return t.push(n), hi(this, zr) && t.push(new Uint8Array(hi(this, zr))), se(t);
    }
    decode(e, t) {
      let n,
        i = t;
      [n, i] = [K(e.slice(i, i + this.length)), i + this.length];
      let a = n;
      return hi(this, zr) && ([n, i] = [null, i + hi(this, zr)]), [a, i];
    }
  };
zr = new WeakMap();
var gn = class extends zi {
    constructor() {
      super('TxPointer', { blockHeight: new te('u32'), txIndex: new te('u16') });
    }
  },
  Ti = class extends zi {
    constructor() {
      super('UtxoId', { transactionId: new U(), outputIndex: new te('u8') });
    }
  },
  Rt = ((r) => (
    (r[(r.Coin = 0)] = 'Coin'),
    (r[(r.Contract = 1)] = 'Contract'),
    (r[(r.Message = 2)] = 'Message'),
    r
  ))(Rt || {}),
  Vs = class extends Ie {
    constructor() {
      super('InputCoin', 'struct InputCoin', 0);
    }
    encode(r) {
      let e = [];
      return (
        e.push(new Ti().encode(r.utxoID)),
        e.push(new U().encode(r.owner)),
        e.push(new $().encode(r.amount)),
        e.push(new U().encode(r.assetId)),
        e.push(new gn().encode(r.txPointer)),
        e.push(new te('u8').encode(r.witnessIndex)),
        e.push(new te('u32').encode(r.maturity)),
        e.push(new te('u16').encode(r.predicateLength)),
        e.push(new te('u16').encode(r.predicateDataLength)),
        e.push(new et(r.predicateLength).encode(r.predicate)),
        e.push(new et(r.predicateDataLength).encode(r.predicateData)),
        se(e)
      );
    }
    decode(r, e) {
      let t,
        n = e;
      [t, n] = new Ti().decode(r, n);
      let i = t;
      [t, n] = new U().decode(r, n);
      let a = t;
      [t, n] = new $().decode(r, n);
      let c = t;
      [t, n] = new U().decode(r, n);
      let v = t;
      [t, n] = new gn().decode(r, n);
      let w = t;
      [t, n] = new te('u8').decode(r, n);
      let y = Number(t);
      [t, n] = new te('u32').decode(r, n);
      let M = t;
      [t, n] = new te('u16').decode(r, n);
      let S = t;
      [t, n] = new te('u16').decode(r, n);
      let I = t;
      [t, n] = new et(S).decode(r, n);
      let E = t;
      return (
        ([t, n] = new et(I).decode(r, n)),
        [
          {
            type: 0,
            utxoID: i,
            owner: a,
            amount: c,
            assetId: v,
            txPointer: w,
            witnessIndex: y,
            maturity: M,
            predicateLength: S,
            predicateDataLength: I,
            predicate: E,
            predicateData: t,
          },
          n,
        ]
      );
    }
  },
  Js = class extends Ie {
    constructor() {
      super('InputContract', 'struct InputContract', 0);
    }
    encode(e) {
      let t = [];
      return (
        t.push(new Ti().encode(e.utxoID)),
        t.push(new U().encode(e.balanceRoot)),
        t.push(new U().encode(e.stateRoot)),
        t.push(new gn().encode(e.txPointer)),
        t.push(new U().encode(e.contractID)),
        se(t)
      );
    }
    decode(e, t) {
      let n,
        i = t;
      [n, i] = new Ti().decode(e, i);
      let a = n;
      [n, i] = new U().decode(e, i);
      let c = n;
      [n, i] = new U().decode(e, i);
      let v = n;
      [n, i] = new gn().decode(e, i);
      let w = n;
      return (
        ([n, i] = new U().decode(e, i)),
        [{ type: 1, utxoID: a, balanceRoot: c, stateRoot: v, txPointer: w, contractID: n }, i]
      );
    }
  },
  Hn = class extends Ie {
    constructor() {
      super('InputMessage', 'struct InputMessage', 0);
    }
    static getMessageId(e) {
      let t = [];
      return (
        t.push(new et(32).encode(e.sender)),
        t.push(new et(32).encode(e.recipient)),
        t.push(new $().encode(e.nonce)),
        t.push(new $().encode(e.amount)),
        t.push(new et(e.dataLength).encode(e.data)),
        Pt(se(t))
      );
    }
    encode(e) {
      let t = [],
        n = new et(e.dataLength).encode(e.data),
        i = Hn.getMessageId(e);
      return (
        t.push(new et(32).encode(i)),
        t.push(new et(32).encode(e.sender)),
        t.push(new et(32).encode(e.recipient)),
        t.push(new $().encode(e.amount)),
        t.push(new $().encode(e.nonce)),
        t.push(new te('u8').encode(e.witnessIndex)),
        t.push(new te('u16').encode(n.length)),
        t.push(new te('u16').encode(e.predicateLength)),
        t.push(new te('u16').encode(e.predicateDataLength)),
        t.push(n),
        t.push(new et(e.predicateLength).encode(e.predicate)),
        t.push(new et(e.predicateDataLength).encode(e.predicateData)),
        se(t)
      );
    }
    static decodeData(e) {
      let t = V(e),
        n = t.length,
        [i] = new et(n).decode(t, 0);
      return V(i);
    }
    decode(e, t) {
      let n,
        i = t;
      [n, i] = new U().decode(e, i);
      let a = n;
      [n, i] = new U().decode(e, i);
      let c = n;
      [n, i] = new $().decode(e, i);
      let v = n;
      [n, i] = new $().decode(e, i);
      let w = n;
      [n, i] = new te('u8').decode(e, i);
      let y = Number(n);
      [n, i] = new te('u16').decode(e, i);
      let M = n;
      [n, i] = new te('u16').decode(e, i);
      let S = n;
      [n, i] = new te('u16').decode(e, i);
      let I = n;
      [n, i] = new et(M).decode(e, i);
      let E = n;
      [n, i] = new et(S).decode(e, i);
      let R = n;
      return (
        ([n, i] = new et(I).decode(e, i)),
        [
          {
            type: 2,
            sender: a,
            recipient: c,
            amount: v,
            witnessIndex: y,
            nonce: w,
            data: E,
            dataLength: M,
            predicateLength: S,
            predicateDataLength: I,
            predicate: R,
            predicateData: n,
          },
          i,
        ]
      );
    }
  },
  Ci = class extends Ie {
    constructor() {
      super('Input', 'struct Input', 0);
    }
    encode(e) {
      let t = [];
      switch ((t.push(new te('u8').encode(e.type)), e.type)) {
        case 0: {
          t.push(new Vs().encode(e));
          break;
        }
        case 1: {
          t.push(new Js().encode(e));
          break;
        }
        case 2: {
          t.push(new Hn().encode(e));
          break;
        }
        default:
          throw new Error('Invalid Input type');
      }
      return se(t);
    }
    decode(e, t) {
      let n,
        i = t;
      switch ((([n, i] = new te('u8').decode(e, i)), n)) {
        case 0:
          return ([n, i] = new Vs().decode(e, i)), [n, i];
        case 1:
          return ([n, i] = new Js().decode(e, i)), [n, i];
        case 2:
          return ([n, i] = new Hn().decode(e, i)), [n, i];
        default:
          throw new Error('Invalid Input type');
      }
    }
  },
  je = ((r) => (
    (r[(r.Coin = 0)] = 'Coin'),
    (r[(r.Contract = 1)] = 'Contract'),
    (r[(r.Message = 2)] = 'Message'),
    (r[(r.Change = 3)] = 'Change'),
    (r[(r.Variable = 4)] = 'Variable'),
    (r[(r.ContractCreated = 5)] = 'ContractCreated'),
    r
  ))(je || {}),
  Hs = class extends Ie {
    constructor() {
      super('OutputCoin', 'struct OutputCoin', 0);
    }
    encode(e) {
      let t = [];
      return (
        t.push(new U().encode(e.to)),
        t.push(new $().encode(e.amount)),
        t.push(new U().encode(e.assetId)),
        se(t)
      );
    }
    decode(e, t) {
      let n,
        i = t;
      [n, i] = new U().decode(e, i);
      let a = n;
      [n, i] = new $().decode(e, i);
      let c = n;
      return ([n, i] = new U().decode(e, i)), [{ type: 0, to: a, amount: c, assetId: n }, i];
    }
  },
  Ws = class extends Ie {
    constructor() {
      super('OutputContract', 'struct OutputContract', 0);
    }
    encode(e) {
      let t = [];
      return (
        t.push(new te('u8').encode(e.inputIndex)),
        t.push(new U().encode(e.balanceRoot)),
        t.push(new U().encode(e.stateRoot)),
        se(t)
      );
    }
    decode(e, t) {
      let n,
        i = t;
      [n, i] = new te('u8').decode(e, i);
      let a = n;
      [n, i] = new U().decode(e, i);
      let c = n;
      return (
        ([n, i] = new U().decode(e, i)),
        [{ type: 1, inputIndex: a, balanceRoot: c, stateRoot: n }, i]
      );
    }
  },
  Gs = class extends Ie {
    constructor() {
      super('OutputMessage', 'struct OutputMessage', 0);
    }
    encode(e) {
      let t = [];
      return t.push(new U().encode(e.recipient)), t.push(new $().encode(e.amount)), se(t);
    }
    decode(e, t) {
      let n,
        i = t;
      [n, i] = new U().decode(e, i);
      let a = n;
      return ([n, i] = new $().decode(e, i)), [{ type: 2, recipient: a, amount: n }, i];
    }
  },
  Ks = class extends Ie {
    constructor() {
      super('OutputChange', 'struct OutputChange', 0);
    }
    encode(e) {
      let t = [];
      return (
        t.push(new U().encode(e.to)),
        t.push(new $().encode(e.amount)),
        t.push(new U().encode(e.assetId)),
        se(t)
      );
    }
    decode(e, t) {
      let n,
        i = t;
      [n, i] = new U().decode(e, i);
      let a = n;
      [n, i] = new $().decode(e, i);
      let c = n;
      return ([n, i] = new U().decode(e, i)), [{ type: 3, to: a, amount: c, assetId: n }, i];
    }
  },
  Xs = class extends Ie {
    constructor() {
      super('OutputVariable', 'struct OutputVariable', 0);
    }
    encode(e) {
      let t = [];
      return (
        t.push(new U().encode(e.to)),
        t.push(new $().encode(e.amount)),
        t.push(new U().encode(e.assetId)),
        se(t)
      );
    }
    decode(e, t) {
      let n,
        i = t;
      [n, i] = new U().decode(e, i);
      let a = n;
      [n, i] = new $().decode(e, i);
      let c = n;
      return ([n, i] = new U().decode(e, i)), [{ type: 4, to: a, amount: c, assetId: n }, i];
    }
  },
  Zs = class extends Ie {
    constructor() {
      super('OutputContractCreated', 'struct OutputContractCreated', 0);
    }
    encode(e) {
      let t = [];
      return t.push(new U().encode(e.contractId)), t.push(new U().encode(e.stateRoot)), se(t);
    }
    decode(e, t) {
      let n,
        i = t;
      [n, i] = new U().decode(e, i);
      let a = n;
      return ([n, i] = new U().decode(e, i)), [{ type: 5, contractId: a, stateRoot: n }, i];
    }
  },
  yn = class extends Ie {
    constructor() {
      super('Output', ' struct Output', 0);
    }
    encode(e) {
      let t = [];
      switch ((t.push(new te('u8').encode(e.type)), e.type)) {
        case 0: {
          t.push(new Hs().encode(e));
          break;
        }
        case 1: {
          t.push(new Ws().encode(e));
          break;
        }
        case 2: {
          t.push(new Gs().encode(e));
          break;
        }
        case 3: {
          t.push(new Ks().encode(e));
          break;
        }
        case 4: {
          t.push(new Xs().encode(e));
          break;
        }
        case 5: {
          t.push(new Zs().encode(e));
          break;
        }
        default:
          throw new Error('Invalid Output type');
      }
      return se(t);
    }
    decode(e, t) {
      let n,
        i = t;
      switch ((([n, i] = new te('u8').decode(e, i)), n)) {
        case 0:
          return ([n, i] = new Hs().decode(e, i)), [n, i];
        case 1:
          return ([n, i] = new Ws().decode(e, i)), [n, i];
        case 2:
          return ([n, i] = new Gs().decode(e, i)), [n, i];
        case 3:
          return ([n, i] = new Ks().decode(e, i)), [n, i];
        case 4:
          return ([n, i] = new Xs().decode(e, i)), [n, i];
        case 5:
          return ([n, i] = new Zs().decode(e, i)), [n, i];
        default:
          throw new Error('Invalid Output type');
      }
    }
  },
  Nt = ((r) => (
    (r[(r.Call = 0)] = 'Call'),
    (r[(r.Return = 1)] = 'Return'),
    (r[(r.ReturnData = 2)] = 'ReturnData'),
    (r[(r.Panic = 3)] = 'Panic'),
    (r[(r.Revert = 4)] = 'Revert'),
    (r[(r.Log = 5)] = 'Log'),
    (r[(r.LogData = 6)] = 'LogData'),
    (r[(r.Transfer = 7)] = 'Transfer'),
    (r[(r.TransferOut = 8)] = 'TransferOut'),
    (r[(r.ScriptResult = 9)] = 'ScriptResult'),
    (r[(r.MessageOut = 10)] = 'MessageOut'),
    r
  ))(Nt || {}),
  Ys = class extends Ie {
    constructor() {
      super('ReceiptCall', 'struct ReceiptCall', 0);
    }
    encode(e) {
      let t = [];
      return (
        t.push(new U().encode(e.from)),
        t.push(new U().encode(e.to)),
        t.push(new $().encode(e.amount)),
        t.push(new U().encode(e.assetId)),
        t.push(new $().encode(e.gas)),
        t.push(new $().encode(e.param1)),
        t.push(new $().encode(e.param2)),
        t.push(new $().encode(e.pc)),
        t.push(new $().encode(e.is)),
        se(t)
      );
    }
    decode(e, t) {
      let n,
        i = t;
      [n, i] = new U().decode(e, i);
      let a = n;
      [n, i] = new U().decode(e, i);
      let c = n;
      [n, i] = new $().decode(e, i);
      let v = n;
      [n, i] = new U().decode(e, i);
      let w = n;
      [n, i] = new $().decode(e, i);
      let y = n;
      [n, i] = new $().decode(e, i);
      let M = n;
      [n, i] = new $().decode(e, i);
      let S = n;
      [n, i] = new $().decode(e, i);
      let I = n;
      return (
        ([n, i] = new $().decode(e, i)),
        [
          {
            type: 0,
            from: a,
            to: c,
            amount: v,
            assetId: w,
            gas: y,
            param1: M,
            param2: S,
            pc: I,
            is: n,
          },
          i,
        ]
      );
    }
  },
  Qs = class extends Ie {
    constructor() {
      super('ReceiptReturn', 'struct ReceiptReturn', 0);
    }
    encode(e) {
      let t = [];
      return (
        t.push(new U().encode(e.id)),
        t.push(new $().encode(e.val)),
        t.push(new $().encode(e.pc)),
        t.push(new $().encode(e.is)),
        se(t)
      );
    }
    decode(e, t) {
      let n,
        i = t;
      [n, i] = new U().decode(e, i);
      let a = n;
      [n, i] = new $().decode(e, i);
      let c = n;
      [n, i] = new $().decode(e, i);
      let v = n;
      return ([n, i] = new $().decode(e, i)), [{ type: 1, id: a, val: c, pc: v, is: n }, i];
    }
  },
  eo = class extends Ie {
    constructor() {
      super('ReceiptReturnData', 'struct ReceiptReturnData', 0);
    }
    encode(e) {
      let t = [];
      return (
        t.push(new U().encode(e.id)),
        t.push(new $().encode(e.ptr)),
        t.push(new $().encode(e.len)),
        t.push(new U().encode(e.digest)),
        t.push(new $().encode(e.pc)),
        t.push(new $().encode(e.is)),
        se(t)
      );
    }
    decode(e, t) {
      let n,
        i = t;
      [n, i] = new U().decode(e, i);
      let a = n;
      [n, i] = new $().decode(e, i);
      let c = n;
      [n, i] = new $().decode(e, i);
      let v = n;
      [n, i] = new U().decode(e, i);
      let w = n;
      [n, i] = new $().decode(e, i);
      let y = n;
      return (
        ([n, i] = new $().decode(e, i)),
        [{ type: 2, id: a, ptr: c, len: v, digest: w, pc: y, is: n }, i]
      );
    }
  },
  to = class extends Ie {
    constructor() {
      super('ReceiptPanic', 'struct ReceiptPanic', 0);
    }
    encode(e) {
      let t = [];
      return (
        t.push(new U().encode(e.id)),
        t.push(new $().encode(e.reason)),
        t.push(new $().encode(e.pc)),
        t.push(new $().encode(e.is)),
        t.push(new U().encode(e.contractId)),
        se(t)
      );
    }
    decode(e, t) {
      let n,
        i = t;
      [n, i] = new U().decode(e, i);
      let a = n;
      [n, i] = new $().decode(e, i);
      let c = n;
      [n, i] = new $().decode(e, i);
      let v = n;
      [n, i] = new $().decode(e, i);
      let w = n;
      return (
        ([n, i] = new U().decode(e, i)),
        [{ type: 3, id: a, reason: c, pc: v, is: w, contractId: n }, i]
      );
    }
  },
  ro = class extends Ie {
    constructor() {
      super('ReceiptRevert', 'struct ReceiptRevert', 0);
    }
    encode(e) {
      let t = [];
      return (
        t.push(new U().encode(e.id)),
        t.push(new $().encode(e.val)),
        t.push(new $().encode(e.pc)),
        t.push(new $().encode(e.is)),
        se(t)
      );
    }
    decode(e, t) {
      let n,
        i = t;
      [n, i] = new U().decode(e, i);
      let a = n;
      [n, i] = new $().decode(e, i);
      let c = n;
      [n, i] = new $().decode(e, i);
      let v = n;
      return ([n, i] = new $().decode(e, i)), [{ type: 4, id: a, val: c, pc: v, is: n }, i];
    }
  },
  no = class extends Ie {
    constructor() {
      super('ReceiptLog', 'struct ReceiptLog', 0);
    }
    encode(e) {
      let t = [];
      return (
        t.push(new U().encode(e.id)),
        t.push(new $().encode(e.val0)),
        t.push(new $().encode(e.val1)),
        t.push(new $().encode(e.val2)),
        t.push(new $().encode(e.val3)),
        t.push(new $().encode(e.pc)),
        t.push(new $().encode(e.is)),
        se(t)
      );
    }
    decode(e, t) {
      let n,
        i = t;
      [n, i] = new U().decode(e, i);
      let a = n;
      [n, i] = new $().decode(e, i);
      let c = n;
      [n, i] = new $().decode(e, i);
      let v = n;
      [n, i] = new $().decode(e, i);
      let w = n;
      [n, i] = new $().decode(e, i);
      let y = n;
      [n, i] = new $().decode(e, i);
      let M = n;
      return (
        ([n, i] = new $().decode(e, i)),
        [{ type: 5, id: a, val0: c, val1: v, val2: w, val3: y, pc: M, is: n }, i]
      );
    }
  },
  io = class extends Ie {
    constructor() {
      super('ReceiptLogData', 'struct ReceiptLogData', 0);
    }
    encode(e) {
      let t = [];
      return (
        t.push(new U().encode(e.id)),
        t.push(new $().encode(e.val0)),
        t.push(new $().encode(e.val1)),
        t.push(new $().encode(e.ptr)),
        t.push(new $().encode(e.len)),
        t.push(new U().encode(e.digest)),
        t.push(new $().encode(e.pc)),
        t.push(new $().encode(e.is)),
        se(t)
      );
    }
    decode(e, t) {
      let n,
        i = t;
      [n, i] = new U().decode(e, i);
      let a = n;
      [n, i] = new $().decode(e, i);
      let c = n;
      [n, i] = new $().decode(e, i);
      let v = n;
      [n, i] = new $().decode(e, i);
      let w = n;
      [n, i] = new $().decode(e, i);
      let y = n;
      [n, i] = new U().decode(e, i);
      let M = n;
      [n, i] = new $().decode(e, i);
      let S = n;
      return (
        ([n, i] = new $().decode(e, i)),
        [{ type: 6, id: a, val0: c, val1: v, ptr: w, len: y, digest: M, pc: S, is: n }, i]
      );
    }
  },
  ao = class extends Ie {
    constructor() {
      super('ReceiptTransfer', 'struct ReceiptTransfer', 0);
    }
    encode(r) {
      let e = [];
      return (
        e.push(new U().encode(r.from)),
        e.push(new U().encode(r.to)),
        e.push(new $().encode(r.amount)),
        e.push(new U().encode(r.assetId)),
        e.push(new $().encode(r.pc)),
        e.push(new $().encode(r.is)),
        se(e)
      );
    }
    decode(r, e) {
      let t,
        n = e;
      [t, n] = new U().decode(r, n);
      let i = t;
      [t, n] = new U().decode(r, n);
      let a = t;
      [t, n] = new $().decode(r, n);
      let c = t;
      [t, n] = new U().decode(r, n);
      let v = t;
      [t, n] = new $().decode(r, n);
      let w = t;
      return (
        ([t, n] = new $().decode(r, n)),
        [{ type: 7, from: i, to: a, amount: c, assetId: v, pc: w, is: t }, n]
      );
    }
  },
  so = class extends Ie {
    constructor() {
      super('ReceiptTransferOut', 'struct ReceiptTransferOut', 0);
    }
    encode(e) {
      let t = [];
      return (
        t.push(new U().encode(e.from)),
        t.push(new U().encode(e.to)),
        t.push(new $().encode(e.amount)),
        t.push(new U().encode(e.assetId)),
        t.push(new $().encode(e.pc)),
        t.push(new $().encode(e.is)),
        se(t)
      );
    }
    decode(e, t) {
      let n,
        i = t;
      [n, i] = new U().decode(e, i);
      let a = n;
      [n, i] = new U().decode(e, i);
      let c = n;
      [n, i] = new $().decode(e, i);
      let v = n;
      [n, i] = new U().decode(e, i);
      let w = n;
      [n, i] = new $().decode(e, i);
      let y = n;
      return (
        ([n, i] = new $().decode(e, i)),
        [{ type: 8, from: a, to: c, amount: v, assetId: w, pc: y, is: n }, i]
      );
    }
  },
  oo = class extends Ie {
    constructor() {
      super('ReceiptScriptResult', 'struct ReceiptScriptResult', 0);
    }
    encode(r) {
      let e = [];
      return e.push(new $().encode(r.result)), e.push(new $().encode(r.gasUsed)), se(e);
    }
    decode(r, e) {
      let t,
        n = e;
      [t, n] = new $().decode(r, n);
      let i = t;
      return ([t, n] = new $().decode(r, n)), [{ type: 9, result: i, gasUsed: t }, n];
    }
  },
  fo = class extends Ie {
    constructor() {
      super('ReceiptMessageOut', 'struct ReceiptMessageOut', 0);
    }
    encode(e) {
      let t = [];
      return (
        t.push(new U().encode(e.messageID)),
        t.push(new U().encode(e.sender)),
        t.push(new U().encode(e.recipient)),
        t.push(new $().encode(e.amount)),
        t.push(new U().encode(e.nonce)),
        t.push(new te('u16').encode(e.data.length)),
        t.push(new U().encode(e.digest)),
        t.push(new et(e.data.length).encode(e.data)),
        se(t)
      );
    }
    decode(e, t) {
      let n,
        i = t;
      [n, i] = new U().decode(e, i);
      let a = n;
      [n, i] = new U().decode(e, i);
      let c = n;
      [n, i] = new U().decode(e, i);
      let v = n;
      [n, i] = new $().decode(e, i);
      let w = n;
      [n, i] = new U().decode(e, i);
      let y = n;
      [n, i] = new te('u16').decode(e, i);
      let M = n;
      [n, i] = new U().decode(e, i);
      let S = n;
      [n, i] = new et(M).decode(e, i);
      let I = V(n);
      return [
        {
          type: 10,
          messageID: a,
          sender: c,
          recipient: v,
          amount: w,
          nonce: y,
          digest: S,
          data: I,
        },
        i,
      ];
    }
  },
  k0 = class extends Ie {
    constructor() {
      super('Receipt', 'struct Receipt', 0);
    }
    encode(r) {
      let e = [];
      switch ((e.push(new te('u8').encode(r.type)), r.type)) {
        case 0: {
          e.push(new Ys().encode(r));
          break;
        }
        case 1: {
          e.push(new Qs().encode(r));
          break;
        }
        case 2: {
          e.push(new eo().encode(r));
          break;
        }
        case 3: {
          e.push(new to().encode(r));
          break;
        }
        case 4: {
          e.push(new ro().encode(r));
          break;
        }
        case 5: {
          e.push(new no().encode(r));
          break;
        }
        case 6: {
          e.push(new io().encode(r));
          break;
        }
        case 7: {
          e.push(new ao().encode(r));
          break;
        }
        case 8: {
          e.push(new so().encode(r));
          break;
        }
        case 9: {
          e.push(new oo().encode(r));
          break;
        }
        case 10: {
          e.push(new fo().encode(r));
          break;
        }
        default:
          throw new Error('Invalid Receipt type');
      }
      return se(e);
    }
    decode(r, e) {
      let t,
        n = e;
      switch ((([t, n] = new te('u8').decode(r, n)), t)) {
        case 0:
          return ([t, n] = new Ys().decode(r, n)), [t, n];
        case 1:
          return ([t, n] = new Qs().decode(r, n)), [t, n];
        case 2:
          return ([t, n] = new eo().decode(r, n)), [t, n];
        case 3:
          return ([t, n] = new to().decode(r, n)), [t, n];
        case 4:
          return ([t, n] = new ro().decode(r, n)), [t, n];
        case 5:
          return ([t, n] = new no().decode(r, n)), [t, n];
        case 6:
          return ([t, n] = new io().decode(r, n)), [t, n];
        case 7:
          return ([t, n] = new ao().decode(r, n)), [t, n];
        case 8:
          return ([t, n] = new so().decode(r, n)), [t, n];
        case 9:
          return ([t, n] = new oo().decode(r, n)), [t, n];
        case 10:
          return ([t, n] = new fo().decode(r, n)), [t, n];
        default:
          throw new Error('Invalid Receipt type');
      }
    }
  },
  co = class extends zi {
    constructor() {
      super('StorageSlot', { key: new U(), value: new U() });
    }
  },
  Pi = class extends Ie {
    constructor() {
      super('Witness', 'unknown', 0);
    }
    encode(e) {
      let t = [];
      return (
        t.push(new te('u32').encode(e.dataLength)),
        t.push(new et(e.dataLength).encode(e.data)),
        se(t)
      );
    }
    decode(e, t) {
      let n,
        i = t;
      [n, i] = new te('u32').decode(e, i);
      let a = n;
      return ([n, i] = new et(a).decode(e, i)), [{ dataLength: a, data: n }, i];
    }
  },
  Cr = ((r) => (
    (r[(r.Script = 0)] = 'Script'), (r[(r.Create = 1)] = 'Create'), (r[(r.Mint = 2)] = 'Mint'), r
  ))(Cr || {}),
  uo = class extends Ie {
    constructor() {
      super('TransactionScript', 'struct TransactionScript', 0);
    }
    encode(e) {
      let t = [];
      return (
        t.push(new $().encode(e.gasPrice)),
        t.push(new $().encode(e.gasLimit)),
        t.push(new te('u32').encode(e.maturity)),
        t.push(new te('u16').encode(e.scriptLength)),
        t.push(new te('u16').encode(e.scriptDataLength)),
        t.push(new te('u8').encode(e.inputsCount)),
        t.push(new te('u8').encode(e.outputsCount)),
        t.push(new te('u8').encode(e.witnessesCount)),
        t.push(new U().encode(e.receiptsRoot)),
        t.push(new et(e.scriptLength).encode(e.script)),
        t.push(new et(e.scriptDataLength).encode(e.scriptData)),
        t.push(new Et(new Ci(), e.inputsCount).encode(e.inputs)),
        t.push(new Et(new yn(), e.outputsCount).encode(e.outputs)),
        t.push(new Et(new Pi(), e.witnessesCount).encode(e.witnesses)),
        se(t)
      );
    }
    decode(e, t) {
      let n,
        i = t;
      [n, i] = new $().decode(e, i);
      let a = n;
      [n, i] = new $().decode(e, i);
      let c = n;
      [n, i] = new te('u32').decode(e, i);
      let v = n;
      [n, i] = new te('u16').decode(e, i);
      let w = n;
      [n, i] = new te('u16').decode(e, i);
      let y = n;
      [n, i] = new te('u8').decode(e, i);
      let M = n;
      [n, i] = new te('u8').decode(e, i);
      let S = n;
      [n, i] = new te('u8').decode(e, i);
      let I = n;
      [n, i] = new U().decode(e, i);
      let E = n;
      [n, i] = new et(w).decode(e, i);
      let R = n;
      [n, i] = new et(y).decode(e, i);
      let T = n;
      [n, i] = new Et(new Ci(), M).decode(e, i);
      let z = n;
      [n, i] = new Et(new yn(), S).decode(e, i);
      let q = n;
      return (
        ([n, i] = new Et(new Pi(), I).decode(e, i)),
        [
          {
            type: 0,
            gasPrice: a,
            gasLimit: c,
            maturity: v,
            scriptLength: w,
            scriptDataLength: y,
            inputsCount: M,
            outputsCount: S,
            witnessesCount: I,
            receiptsRoot: E,
            script: R,
            scriptData: T,
            inputs: z,
            outputs: q,
            witnesses: n,
          },
          i,
        ]
      );
    }
  },
  ho = class extends Ie {
    constructor() {
      super('TransactionCreate', 'struct TransactionCreate', 0);
    }
    encode(e) {
      let t = [];
      return (
        t.push(new $().encode(e.gasPrice)),
        t.push(new $().encode(e.gasLimit)),
        t.push(new te('u32').encode(e.maturity)),
        t.push(new te('u16').encode(e.bytecodeLength)),
        t.push(new te('u8').encode(e.bytecodeWitnessIndex)),
        t.push(new te('u16').encode(e.storageSlotsCount)),
        t.push(new te('u8').encode(e.inputsCount)),
        t.push(new te('u8').encode(e.outputsCount)),
        t.push(new te('u8').encode(e.witnessesCount)),
        t.push(new U().encode(e.salt)),
        t.push(new Et(new co(), e.storageSlotsCount).encode(e.storageSlots)),
        t.push(new Et(new Ci(), e.inputsCount).encode(e.inputs)),
        t.push(new Et(new yn(), e.outputsCount).encode(e.outputs)),
        t.push(new Et(new Pi(), e.witnessesCount).encode(e.witnesses)),
        se(t)
      );
    }
    decode(e, t) {
      let n,
        i = t;
      [n, i] = new $().decode(e, i);
      let a = n;
      [n, i] = new $().decode(e, i);
      let c = n;
      [n, i] = new te('u32').decode(e, i);
      let v = n;
      [n, i] = new te('u16').decode(e, i);
      let w = n;
      [n, i] = new te('u8').decode(e, i);
      let y = n;
      [n, i] = new te('u16').decode(e, i);
      let M = n;
      [n, i] = new te('u8').decode(e, i);
      let S = n;
      [n, i] = new te('u8').decode(e, i);
      let I = n;
      [n, i] = new te('u8').decode(e, i);
      let E = n;
      [n, i] = new U().decode(e, i);
      let R = n;
      [n, i] = new Et(new co(), M).decode(e, i);
      let T = n;
      [n, i] = new Et(new Ci(), S).decode(e, i);
      let z = n;
      [n, i] = new Et(new yn(), I).decode(e, i);
      let q = n;
      return (
        ([n, i] = new Et(new Pi(), E).decode(e, i)),
        [
          {
            type: 1,
            gasPrice: a,
            gasLimit: c,
            maturity: v,
            bytecodeLength: w,
            bytecodeWitnessIndex: y,
            storageSlotsCount: M,
            inputsCount: S,
            outputsCount: I,
            witnessesCount: E,
            salt: R,
            storageSlots: T,
            inputs: z,
            outputs: q,
            witnesses: n,
          },
          i,
        ]
      );
    }
  },
  lo = class extends Ie {
    constructor() {
      super('TransactionMint', 'struct TransactionMint', 0);
    }
    encode(r) {
      let e = [];
      return (
        e.push(new gn().encode(r.txPointer)),
        e.push(new te('u8').encode(r.outputsCount)),
        e.push(new Et(new yn(), r.outputsCount).encode(r.outputs)),
        se(e)
      );
    }
    decode(r, e) {
      let t,
        n = e;
      [t, n] = new gn().decode(r, n);
      let i = t;
      [t, n] = new te('u8').decode(r, n);
      let a = t;
      return (
        ([t, n] = new Et(new yn(), a).decode(r, n)),
        [{ type: 2, outputsCount: a, outputs: t, txPointer: i }, n]
      );
    }
  },
  Wn = class extends Ie {
    constructor() {
      super('Transaction', 'struct Transaction', 0);
    }
    encode(r) {
      let e = [];
      switch ((e.push(new te('u8').encode(r.type)), r.type)) {
        case 0: {
          e.push(new uo().encode(r));
          break;
        }
        case 1: {
          e.push(new ho().encode(r));
          break;
        }
        case 2: {
          e.push(new lo().encode(r));
          break;
        }
        default:
          throw new Error('Invalid Transaction type');
      }
      return se(e);
    }
    decode(r, e) {
      let t,
        n = e;
      switch ((([t, n] = new te('u8').decode(r, n)), t)) {
        case 0:
          return ([t, n] = new uo().decode(r, n)), [t, n];
        case 1:
          return ([t, n] = new ho().decode(r, n)), [t, n];
        case 2:
          return ([t, n] = new lo().decode(r, n)), [t, n];
        default:
          throw new Error('Invalid Transaction type');
      }
    }
  },
  zn = B(1e8),
  $0 = B(1e6);
B(4);
var _h = '0xffffffffffff0001',
  wa = {},
  ki = { exports: {} };
(function (r, e) {
  var t = typeof self < 'u' ? self : ie,
    n = (function () {
      function a() {
        (this.fetch = !1), (this.DOMException = t.DOMException);
      }
      return (a.prototype = t), new a();
    })();
  (function (a) {
    (function (c) {
      var v = {
        searchParams: 'URLSearchParams' in a,
        iterable: 'Symbol' in a && 'iterator' in Symbol,
        blob:
          'FileReader' in a &&
          'Blob' in a &&
          (function () {
            try {
              return new Blob(), !0;
            } catch {
              return !1;
            }
          })(),
        formData: 'FormData' in a,
        arrayBuffer: 'ArrayBuffer' in a,
      };
      function w(s) {
        return s && DataView.prototype.isPrototypeOf(s);
      }
      if (v.arrayBuffer)
        var y = [
            '[object Int8Array]',
            '[object Uint8Array]',
            '[object Uint8ClampedArray]',
            '[object Int16Array]',
            '[object Uint16Array]',
            '[object Int32Array]',
            '[object Uint32Array]',
            '[object Float32Array]',
            '[object Float64Array]',
          ],
          M =
            ArrayBuffer.isView ||
            function (s) {
              return s && y.indexOf(Object.prototype.toString.call(s)) > -1;
            };
      function S(s) {
        if ((typeof s != 'string' && (s = String(s)), /[^a-z0-9\-#$%&'*+.^_`|~]/i.test(s)))
          throw new TypeError('Invalid character in header field name');
        return s.toLowerCase();
      }
      function I(s) {
        return typeof s != 'string' && (s = String(s)), s;
      }
      function E(s) {
        var o = {
          next: function () {
            var l = s.shift();
            return { done: l === void 0, value: l };
          },
        };
        return (
          v.iterable &&
            (o[Symbol.iterator] = function () {
              return o;
            }),
          o
        );
      }
      function R(s) {
        (this.map = {}),
          s instanceof R
            ? s.forEach(function (o, l) {
                this.append(l, o);
              }, this)
            : Array.isArray(s)
            ? s.forEach(function (o) {
                this.append(o[0], o[1]);
              }, this)
            : s &&
              Object.getOwnPropertyNames(s).forEach(function (o) {
                this.append(o, s[o]);
              }, this);
      }
      (R.prototype.append = function (s, o) {
        (s = S(s)), (o = I(o));
        var l = this.map[s];
        this.map[s] = l ? l + ', ' + o : o;
      }),
        (R.prototype.delete = function (s) {
          delete this.map[S(s)];
        }),
        (R.prototype.get = function (s) {
          return (s = S(s)), this.has(s) ? this.map[s] : null;
        }),
        (R.prototype.has = function (s) {
          return this.map.hasOwnProperty(S(s));
        }),
        (R.prototype.set = function (s, o) {
          this.map[S(s)] = I(o);
        }),
        (R.prototype.forEach = function (s, o) {
          for (var l in this.map) this.map.hasOwnProperty(l) && s.call(o, this.map[l], l, this);
        }),
        (R.prototype.keys = function () {
          var s = [];
          return (
            this.forEach(function (o, l) {
              s.push(l);
            }),
            E(s)
          );
        }),
        (R.prototype.values = function () {
          var s = [];
          return (
            this.forEach(function (o) {
              s.push(o);
            }),
            E(s)
          );
        }),
        (R.prototype.entries = function () {
          var s = [];
          return (
            this.forEach(function (o, l) {
              s.push([l, o]);
            }),
            E(s)
          );
        }),
        v.iterable && (R.prototype[Symbol.iterator] = R.prototype.entries);
      function T(s) {
        if (s.bodyUsed) return Promise.reject(new TypeError('Already read'));
        s.bodyUsed = !0;
      }
      function z(s) {
        return new Promise(function (o, l) {
          (s.onload = function () {
            o(s.result);
          }),
            (s.onerror = function () {
              l(s.error);
            });
        });
      }
      function q(s) {
        var o = new FileReader(),
          l = z(o);
        return o.readAsArrayBuffer(s), l;
      }
      function Y(s) {
        var o = new FileReader(),
          l = z(o);
        return o.readAsText(s), l;
      }
      function Se(s) {
        for (var o = new Uint8Array(s), l = new Array(o.length), g = 0; g < o.length; g++)
          l[g] = String.fromCharCode(o[g]);
        return l.join('');
      }
      function de(s) {
        if (s.slice) return s.slice(0);
        var o = new Uint8Array(s.byteLength);
        return o.set(new Uint8Array(s)), o.buffer;
      }
      function H() {
        return (
          (this.bodyUsed = !1),
          (this._initBody = function (s) {
            (this._bodyInit = s),
              s
                ? typeof s == 'string'
                  ? (this._bodyText = s)
                  : v.blob && Blob.prototype.isPrototypeOf(s)
                  ? (this._bodyBlob = s)
                  : v.formData && FormData.prototype.isPrototypeOf(s)
                  ? (this._bodyFormData = s)
                  : v.searchParams && URLSearchParams.prototype.isPrototypeOf(s)
                  ? (this._bodyText = s.toString())
                  : v.arrayBuffer && v.blob && w(s)
                  ? ((this._bodyArrayBuffer = de(s.buffer)),
                    (this._bodyInit = new Blob([this._bodyArrayBuffer])))
                  : v.arrayBuffer && (ArrayBuffer.prototype.isPrototypeOf(s) || M(s))
                  ? (this._bodyArrayBuffer = de(s))
                  : (this._bodyText = s = Object.prototype.toString.call(s))
                : (this._bodyText = ''),
              this.headers.get('content-type') ||
                (typeof s == 'string'
                  ? this.headers.set('content-type', 'text/plain;charset=UTF-8')
                  : this._bodyBlob && this._bodyBlob.type
                  ? this.headers.set('content-type', this._bodyBlob.type)
                  : v.searchParams &&
                    URLSearchParams.prototype.isPrototypeOf(s) &&
                    this.headers.set(
                      'content-type',
                      'application/x-www-form-urlencoded;charset=UTF-8'
                    ));
          }),
          v.blob &&
            ((this.blob = function () {
              var s = T(this);
              if (s) return s;
              if (this._bodyBlob) return Promise.resolve(this._bodyBlob);
              if (this._bodyArrayBuffer) return Promise.resolve(new Blob([this._bodyArrayBuffer]));
              if (this._bodyFormData) throw new Error('could not read FormData body as blob');
              return Promise.resolve(new Blob([this._bodyText]));
            }),
            (this.arrayBuffer = function () {
              return this._bodyArrayBuffer
                ? T(this) || Promise.resolve(this._bodyArrayBuffer)
                : this.blob().then(q);
            })),
          (this.text = function () {
            var s = T(this);
            if (s) return s;
            if (this._bodyBlob) return Y(this._bodyBlob);
            if (this._bodyArrayBuffer) return Promise.resolve(Se(this._bodyArrayBuffer));
            if (this._bodyFormData) throw new Error('could not read FormData body as text');
            return Promise.resolve(this._bodyText);
          }),
          v.formData &&
            (this.formData = function () {
              return this.text().then(W);
            }),
          (this.json = function () {
            return this.text().then(JSON.parse);
          }),
          this
        );
      }
      var F = ['DELETE', 'GET', 'HEAD', 'OPTIONS', 'POST', 'PUT'];
      function J(s) {
        var o = s.toUpperCase();
        return F.indexOf(o) > -1 ? o : s;
      }
      function X(s, o) {
        o = o || {};
        var l = o.body;
        if (s instanceof X) {
          if (s.bodyUsed) throw new TypeError('Already read');
          (this.url = s.url),
            (this.credentials = s.credentials),
            o.headers || (this.headers = new R(s.headers)),
            (this.method = s.method),
            (this.mode = s.mode),
            (this.signal = s.signal),
            !l && s._bodyInit != null && ((l = s._bodyInit), (s.bodyUsed = !0));
        } else this.url = String(s);
        if (
          ((this.credentials = o.credentials || this.credentials || 'same-origin'),
          (o.headers || !this.headers) && (this.headers = new R(o.headers)),
          (this.method = J(o.method || this.method || 'GET')),
          (this.mode = o.mode || this.mode || null),
          (this.signal = o.signal || this.signal),
          (this.referrer = null),
          (this.method === 'GET' || this.method === 'HEAD') && l)
        )
          throw new TypeError('Body not allowed for GET or HEAD requests');
        this._initBody(l);
      }
      X.prototype.clone = function () {
        return new X(this, { body: this._bodyInit });
      };
      function W(s) {
        var o = new FormData();
        return (
          s
            .trim()
            .split('&')
            .forEach(function (l) {
              if (l) {
                var g = l.split('='),
                  m = g.shift().replace(/\+/g, ' '),
                  b = g.join('=').replace(/\+/g, ' ');
                o.append(decodeURIComponent(m), decodeURIComponent(b));
              }
            }),
          o
        );
      }
      function G(s) {
        var o = new R(),
          l = s.replace(/\r?\n[\t ]+/g, ' ');
        return (
          l.split(/\r?\n/).forEach(function (g) {
            var m = g.split(':'),
              b = m.shift().trim();
            if (b) {
              var f = m.join(':').trim();
              o.append(b, f);
            }
          }),
          o
        );
      }
      H.call(X.prototype);
      function A(s, o) {
        o || (o = {}),
          (this.type = 'default'),
          (this.status = o.status === void 0 ? 200 : o.status),
          (this.ok = this.status >= 200 && this.status < 300),
          (this.statusText = 'statusText' in o ? o.statusText : 'OK'),
          (this.headers = new R(o.headers)),
          (this.url = o.url || ''),
          this._initBody(s);
      }
      H.call(A.prototype),
        (A.prototype.clone = function () {
          return new A(this._bodyInit, {
            status: this.status,
            statusText: this.statusText,
            headers: new R(this.headers),
            url: this.url,
          });
        }),
        (A.error = function () {
          var s = new A(null, { status: 0, statusText: '' });
          return (s.type = 'error'), s;
        });
      var d = [301, 302, 303, 307, 308];
      (A.redirect = function (s, o) {
        if (d.indexOf(o) === -1) throw new RangeError('Invalid status code');
        return new A(null, { status: o, headers: { location: s } });
      }),
        (c.DOMException = a.DOMException);
      try {
        new c.DOMException();
      } catch {
        (c.DOMException = function (o, l) {
          (this.message = o), (this.name = l);
          var g = Error(o);
          this.stack = g.stack;
        }),
          (c.DOMException.prototype = Object.create(Error.prototype)),
          (c.DOMException.prototype.constructor = c.DOMException);
      }
      function u(s, o) {
        return new Promise(function (l, g) {
          var m = new X(s, o);
          if (m.signal && m.signal.aborted) return g(new c.DOMException('Aborted', 'AbortError'));
          var b = new XMLHttpRequest();
          function f() {
            b.abort();
          }
          (b.onload = function () {
            var p = {
              status: b.status,
              statusText: b.statusText,
              headers: G(b.getAllResponseHeaders() || ''),
            };
            p.url = 'responseURL' in b ? b.responseURL : p.headers.get('X-Request-URL');
            var h = 'response' in b ? b.response : b.responseText;
            l(new A(h, p));
          }),
            (b.onerror = function () {
              g(new TypeError('Network request failed'));
            }),
            (b.ontimeout = function () {
              g(new TypeError('Network request failed'));
            }),
            (b.onabort = function () {
              g(new c.DOMException('Aborted', 'AbortError'));
            }),
            b.open(m.method, m.url, !0),
            m.credentials === 'include'
              ? (b.withCredentials = !0)
              : m.credentials === 'omit' && (b.withCredentials = !1),
            'responseType' in b && v.blob && (b.responseType = 'blob'),
            m.headers.forEach(function (p, h) {
              b.setRequestHeader(h, p);
            }),
            m.signal &&
              (m.signal.addEventListener('abort', f),
              (b.onreadystatechange = function () {
                b.readyState === 4 && m.signal.removeEventListener('abort', f);
              })),
            b.send(typeof m._bodyInit > 'u' ? null : m._bodyInit);
        });
      }
      return (
        (u.polyfill = !0),
        a.fetch || ((a.fetch = u), (a.Headers = R), (a.Request = X), (a.Response = A)),
        (c.Headers = R),
        (c.Request = X),
        (c.Response = A),
        (c.fetch = u),
        Object.defineProperty(c, '__esModule', { value: !0 }),
        c
      );
    })({});
  })(n),
    (n.fetch.ponyfill = !0),
    delete n.fetch.polyfill;
  var i = n;
  (e = i.fetch),
    (e.default = i.fetch),
    (e.fetch = i.fetch),
    (e.Headers = i.Headers),
    (e.Request = i.Request),
    (e.Response = i.Response),
    (r.exports = e);
})(ki, ki.exports);
const Yv = Kf(ki.exports),
  Sh = Wa(th),
  Ah = Wa(hh);
var rs = {},
  Hi = {},
  D0 = function (e) {
    var t = e.uri,
      n = e.name,
      i = e.type;
    (this.uri = t), (this.name = n), (this.type = i);
  },
  Eh = D0,
  L0 = function (e) {
    return (
      (typeof File < 'u' && e instanceof File) ||
      (typeof Blob < 'u' && e instanceof Blob) ||
      e instanceof Eh
    );
  },
  Ih = L0,
  Rh = function r(e, t, n) {
    t === void 0 && (t = ''), n === void 0 && (n = Ih);
    var i,
      a = new Map();
    function c(M, S) {
      var I = a.get(S);
      I ? I.push.apply(I, M) : a.set(S, M);
    }
    if (n(e)) (i = null), c([t], e);
    else {
      var v = t ? t + '.' : '';
      if (typeof FileList < 'u' && e instanceof FileList)
        i = Array.prototype.map.call(e, function (M, S) {
          return c(['' + v + S], M), null;
        });
      else if (Array.isArray(e))
        i = e.map(function (M, S) {
          var I = r(M, '' + v + S, n);
          return I.files.forEach(c), I.clone;
        });
      else if (e && e.constructor === Object) {
        i = {};
        for (var w in e) {
          var y = r(e[w], '' + v + w, n);
          y.files.forEach(c), (i[w] = y.clone);
        }
      } else i = e;
    }
    return { clone: i, files: a };
  };
Hi.ReactNativeFile = D0;
Hi.extractFiles = Rh;
Hi.isExtractableFile = L0;
var Nh = typeof self == 'object' ? self.FormData : window.FormData,
  Zn = {};
Object.defineProperty(Zn, '__esModule', { value: !0 });
Zn.defaultJsonSerializer = void 0;
Zn.defaultJsonSerializer = { parse: JSON.parse, stringify: JSON.stringify };
var Oh =
  (ie && ie.__importDefault) ||
  function (r) {
    return r && r.__esModule ? r : { default: r };
  };
Object.defineProperty(rs, '__esModule', { value: !0 });
var q0 = Hi,
  Th = Oh(Nh),
  Ch = Zn,
  Ph = function (r) {
    return (
      q0.isExtractableFile(r) || (r !== null && typeof r == 'object' && typeof r.pipe == 'function')
    );
  };
function kh(r, e, t, n) {
  n === void 0 && (n = Ch.defaultJsonSerializer);
  var i = q0.extractFiles({ query: r, variables: e, operationName: t }, '', Ph),
    a = i.clone,
    c = i.files;
  if (c.size === 0) {
    if (!Array.isArray(r)) return n.stringify(a);
    if (typeof e < 'u' && !Array.isArray(e))
      throw new Error('Cannot create request body with given variable type, array expected');
    var v = r.reduce(function (I, E, R) {
      return I.push({ query: E, variables: e ? e[R] : void 0 }), I;
    }, []);
    return n.stringify(v);
  }
  var w = typeof FormData > 'u' ? Th.default : FormData,
    y = new w();
  y.append('operations', n.stringify(a));
  var M = {},
    S = 0;
  return (
    c.forEach(function (I) {
      M[++S] = I;
    }),
    y.append('map', n.stringify(M)),
    (S = 0),
    c.forEach(function (I, E) {
      y.append('' + ++S, E);
    }),
    y
  );
}
rs.default = kh;
var Dt = {};
Object.defineProperty(Dt, '__esModule', { value: !0 });
Dt.parseBatchRequestsExtendedArgs =
  Dt.parseRawRequestExtendedArgs =
  Dt.parseRequestExtendedArgs =
  Dt.parseBatchRequestArgs =
  Dt.parseRawRequestArgs =
  Dt.parseRequestArgs =
    void 0;
function $h(r, e, t) {
  return r.document ? r : { document: r, variables: e, requestHeaders: t, signal: void 0 };
}
Dt.parseRequestArgs = $h;
function Dh(r, e, t) {
  return r.query ? r : { query: r, variables: e, requestHeaders: t, signal: void 0 };
}
Dt.parseRawRequestArgs = Dh;
function Lh(r, e) {
  return r.documents ? r : { documents: r, requestHeaders: e, signal: void 0 };
}
Dt.parseBatchRequestArgs = Lh;
function qh(r, e, t, n) {
  return r.document ? r : { url: r, document: e, variables: t, requestHeaders: n, signal: void 0 };
}
Dt.parseRequestExtendedArgs = qh;
function Bh(r, e, t, n) {
  return r.query ? r : { url: r, query: e, variables: t, requestHeaders: n, signal: void 0 };
}
Dt.parseRawRequestExtendedArgs = Bh;
function Fh(r, e, t) {
  return r.documents ? r : { url: r, documents: e, requestHeaders: t, signal: void 0 };
}
Dt.parseBatchRequestsExtendedArgs = Fh;
var Yn = {},
  Uh =
    (ie && ie.__extends) ||
    (function () {
      var r = function (e, t) {
        return (
          (r =
            Object.setPrototypeOf ||
            ({ __proto__: [] } instanceof Array &&
              function (n, i) {
                n.__proto__ = i;
              }) ||
            function (n, i) {
              for (var a in i) Object.prototype.hasOwnProperty.call(i, a) && (n[a] = i[a]);
            }),
          r(e, t)
        );
      };
      return function (e, t) {
        if (typeof t != 'function' && t !== null)
          throw new TypeError('Class extends value ' + String(t) + ' is not a constructor or null');
        r(e, t);
        function n() {
          this.constructor = e;
        }
        e.prototype = t === null ? Object.create(t) : ((n.prototype = t.prototype), new n());
      };
    })();
Object.defineProperty(Yn, '__esModule', { value: !0 });
Yn.ClientError = void 0;
var zh = (function (r) {
  Uh(e, r);
  function e(t, n) {
    var i = this,
      a = e.extractMessage(t) + ': ' + JSON.stringify({ response: t, request: n });
    return (
      (i = r.call(this, a) || this),
      Object.setPrototypeOf(i, e.prototype),
      (i.response = t),
      (i.request = n),
      typeof Error.captureStackTrace == 'function' && Error.captureStackTrace(i, e),
      i
    );
  }
  return (
    (e.extractMessage = function (t) {
      try {
        return t.errors[0].message;
      } catch {
        return 'GraphQL Error (Code: ' + t.status + ')';
      }
    }),
    e
  );
})(Error);
Yn.ClientError = zh;
var $n = {},
  po;
function jh() {
  if (po) return $n;
  po = 1;
  var r =
      (ie && ie.__assign) ||
      function () {
        return (
          (r =
            Object.assign ||
            function (H) {
              for (var F, J = 1, X = arguments.length; J < X; J++) {
                F = arguments[J];
                for (var W in F) Object.prototype.hasOwnProperty.call(F, W) && (H[W] = F[W]);
              }
              return H;
            }),
          r.apply(this, arguments)
        );
      },
    e =
      (ie && ie.__awaiter) ||
      function (H, F, J, X) {
        function W(G) {
          return G instanceof J
            ? G
            : new J(function (A) {
                A(G);
              });
        }
        return new (J || (J = Promise))(function (G, A) {
          function d(o) {
            try {
              s(X.next(o));
            } catch (l) {
              A(l);
            }
          }
          function u(o) {
            try {
              s(X.throw(o));
            } catch (l) {
              A(l);
            }
          }
          function s(o) {
            o.done ? G(o.value) : W(o.value).then(d, u);
          }
          s((X = X.apply(H, F || [])).next());
        });
      },
    t =
      (ie && ie.__generator) ||
      function (H, F) {
        var J = {
            label: 0,
            sent: function () {
              if (G[0] & 1) throw G[1];
              return G[1];
            },
            trys: [],
            ops: [],
          },
          X,
          W,
          G,
          A;
        return (
          (A = { next: d(0), throw: d(1), return: d(2) }),
          typeof Symbol == 'function' &&
            (A[Symbol.iterator] = function () {
              return this;
            }),
          A
        );
        function d(s) {
          return function (o) {
            return u([s, o]);
          };
        }
        function u(s) {
          if (X) throw new TypeError('Generator is already executing.');
          for (; J; )
            try {
              if (
                ((X = 1),
                W &&
                  (G =
                    s[0] & 2
                      ? W.return
                      : s[0]
                      ? W.throw || ((G = W.return) && G.call(W), 0)
                      : W.next) &&
                  !(G = G.call(W, s[1])).done)
              )
                return G;
              switch (((W = 0), G && (s = [s[0] & 2, G.value]), s[0])) {
                case 0:
                case 1:
                  G = s;
                  break;
                case 4:
                  return J.label++, { value: s[1], done: !1 };
                case 5:
                  J.label++, (W = s[1]), (s = [0]);
                  continue;
                case 7:
                  (s = J.ops.pop()), J.trys.pop();
                  continue;
                default:
                  if (
                    ((G = J.trys),
                    !(G = G.length > 0 && G[G.length - 1]) && (s[0] === 6 || s[0] === 2))
                  ) {
                    J = 0;
                    continue;
                  }
                  if (s[0] === 3 && (!G || (s[1] > G[0] && s[1] < G[3]))) {
                    J.label = s[1];
                    break;
                  }
                  if (s[0] === 6 && J.label < G[1]) {
                    (J.label = G[1]), (G = s);
                    break;
                  }
                  if (G && J.label < G[2]) {
                    (J.label = G[2]), J.ops.push(s);
                    break;
                  }
                  G[2] && J.ops.pop(), J.trys.pop();
                  continue;
              }
              s = F.call(H, J);
            } catch (o) {
              (s = [6, o]), (W = 0);
            } finally {
              X = G = 0;
            }
          if (s[0] & 5) throw s[1];
          return { value: s[0] ? s[1] : void 0, done: !0 };
        }
      };
  Object.defineProperty($n, '__esModule', { value: !0 }), ($n.GraphQLWebSocketClient = void 0);
  var n = Yn,
    i = B0(),
    a = 'connection_init',
    c = 'connection_ack',
    v = 'ping',
    w = 'pong',
    y = 'subscribe',
    M = 'next',
    S = 'error',
    I = 'complete',
    E = (function () {
      function H(F, J, X) {
        (this._type = F), (this._payload = J), (this._id = X);
      }
      return (
        Object.defineProperty(H.prototype, 'type', {
          get: function () {
            return this._type;
          },
          enumerable: !1,
          configurable: !0,
        }),
        Object.defineProperty(H.prototype, 'id', {
          get: function () {
            return this._id;
          },
          enumerable: !1,
          configurable: !0,
        }),
        Object.defineProperty(H.prototype, 'payload', {
          get: function () {
            return this._payload;
          },
          enumerable: !1,
          configurable: !0,
        }),
        Object.defineProperty(H.prototype, 'text', {
          get: function () {
            var F = { type: this.type };
            return (
              this.id != null && this.id != null && (F.id = this.id),
              this.payload != null && this.payload != null && (F.payload = this.payload),
              JSON.stringify(F)
            );
          },
          enumerable: !1,
          configurable: !0,
        }),
        (H.parse = function (F, J) {
          var X = JSON.parse(F),
            W = X.type,
            G = X.payload,
            A = X.id;
          return new H(W, J(G), A);
        }),
        H
      );
    })(),
    R = (function () {
      function H(F, J) {
        var X = this,
          W = J.onInit,
          G = J.onAcknowledged,
          A = J.onPing,
          d = J.onPong;
        (this.socketState = { acknowledged: !1, lastRequestId: 0, subscriptions: {} }),
          (this.socket = F),
          (F.onopen = function (u) {
            return e(X, void 0, void 0, function () {
              var s, o, l, g;
              return t(this, function (m) {
                switch (m.label) {
                  case 0:
                    return (
                      (this.socketState.acknowledged = !1),
                      (this.socketState.subscriptions = {}),
                      (o = (s = F).send),
                      (l = z),
                      W ? [4, W()] : [3, 2]
                    );
                  case 1:
                    return (g = m.sent()), [3, 3];
                  case 2:
                    (g = null), (m.label = 3);
                  case 3:
                    return o.apply(s, [l.apply(void 0, [g]).text]), [2];
                }
              });
            });
          }),
          (F.onclose = function (u) {
            (X.socketState.acknowledged = !1), (X.socketState.subscriptions = {});
          }),
          (F.onerror = function (u) {
            console.error(u);
          }),
          (F.onmessage = function (u) {
            try {
              var s = T(u.data);
              switch (s.type) {
                case c: {
                  X.socketState.acknowledged
                    ? console.warn('Duplicate CONNECTION_ACK message ignored')
                    : ((X.socketState.acknowledged = !0), G && G(s.payload));
                  return;
                }
                case v: {
                  A
                    ? A(s.payload).then(function (b) {
                        return F.send(Y(b).text);
                      })
                    : F.send(Y(null).text);
                  return;
                }
                case w: {
                  d && d(s.payload);
                  return;
                }
              }
              if (
                !X.socketState.acknowledged ||
                s.id === void 0 ||
                s.id === null ||
                !X.socketState.subscriptions[s.id]
              )
                return;
              var o = X.socketState.subscriptions[s.id],
                l = o.query,
                g = o.variables,
                m = o.subscriber;
              switch (s.type) {
                case M: {
                  !s.payload.errors && s.payload.data && m.next && m.next(s.payload.data),
                    s.payload.errors &&
                      m.error &&
                      m.error(
                        new n.ClientError(r(r({}, s.payload), { status: 200 }), {
                          query: l,
                          variables: g,
                        })
                      );
                  return;
                }
                case S: {
                  m.error &&
                    m.error(
                      new n.ClientError(
                        { errors: s.payload, status: 200 },
                        { query: l, variables: g }
                      )
                    );
                  return;
                }
                case I: {
                  m.complete && m.complete(), delete X.socketState.subscriptions[s.id];
                  return;
                }
              }
            } catch (b) {
              console.error(b), F.close(1006);
            }
            F.close(4400, 'Unknown graphql-ws message.');
          });
      }
      return (
        (H.prototype.makeSubscribe = function (F, J, X, W) {
          var G = this,
            A = (this.socketState.lastRequestId++).toString();
          return (
            (this.socketState.subscriptions[A] = { query: F, variables: X, subscriber: W }),
            this.socket.send(Se(A, { query: F, operationName: J, variables: X }).text),
            function () {
              G.socket.send(de(A).text), delete G.socketState.subscriptions[A];
            }
          );
        }),
        (H.prototype.rawRequest = function (F, J) {
          var X = this;
          return new Promise(function (W, G) {
            var A;
            X.rawSubscribe(
              F,
              {
                next: function (d, u) {
                  return (A = { data: d, extensions: u });
                },
                error: G,
                complete: function () {
                  return W(A);
                },
              },
              J
            );
          });
        }),
        (H.prototype.request = function (F, J) {
          var X = this;
          return new Promise(function (W, G) {
            var A;
            X.subscribe(
              F,
              {
                next: function (d) {
                  return (A = d);
                },
                error: G,
                complete: function () {
                  return W(A);
                },
              },
              J
            );
          });
        }),
        (H.prototype.subscribe = function (F, J, X) {
          var W = i.resolveRequestDocument(F),
            G = W.query,
            A = W.operationName;
          return this.makeSubscribe(G, A, X, J);
        }),
        (H.prototype.rawSubscribe = function (F, J, X) {
          return this.makeSubscribe(F, void 0, X, J);
        }),
        (H.prototype.ping = function (F) {
          this.socket.send(q(F).text);
        }),
        (H.prototype.close = function () {
          this.socket.close(1e3);
        }),
        (H.PROTOCOL = 'graphql-transport-ws'),
        H
      );
    })();
  $n.GraphQLWebSocketClient = R;
  function T(H, F) {
    F === void 0 &&
      (F = function (X) {
        return X;
      });
    var J = E.parse(H, F);
    return J;
  }
  function z(H) {
    return new E(a, H);
  }
  function q(H) {
    return new E(v, H, void 0);
  }
  function Y(H) {
    return new E(w, H, void 0);
  }
  function Se(H, F) {
    return new E(y, F, H);
  }
  function de(H) {
    return new E(I, void 0, H);
  }
  return $n;
}
var vo;
function B0() {
  return (
    vo ||
      ((vo = 1),
      (function (r) {
        var e =
            (ie && ie.__assign) ||
            function () {
              return (
                (e =
                  Object.assign ||
                  function (m) {
                    for (var b, f = 1, p = arguments.length; f < p; f++) {
                      b = arguments[f];
                      for (var h in b) Object.prototype.hasOwnProperty.call(b, h) && (m[h] = b[h]);
                    }
                    return m;
                  }),
                e.apply(this, arguments)
              );
            },
          t =
            (ie && ie.__createBinding) ||
            (Object.create
              ? function (m, b, f, p) {
                  p === void 0 && (p = f),
                    Object.defineProperty(m, p, {
                      enumerable: !0,
                      get: function () {
                        return b[f];
                      },
                    });
                }
              : function (m, b, f, p) {
                  p === void 0 && (p = f), (m[p] = b[f]);
                }),
          n =
            (ie && ie.__setModuleDefault) ||
            (Object.create
              ? function (m, b) {
                  Object.defineProperty(m, 'default', { enumerable: !0, value: b });
                }
              : function (m, b) {
                  m.default = b;
                }),
          i =
            (ie && ie.__importStar) ||
            function (m) {
              if (m && m.__esModule) return m;
              var b = {};
              if (m != null)
                for (var f in m)
                  f !== 'default' && Object.prototype.hasOwnProperty.call(m, f) && t(b, m, f);
              return n(b, m), b;
            },
          a =
            (ie && ie.__awaiter) ||
            function (m, b, f, p) {
              function h(x) {
                return x instanceof f
                  ? x
                  : new f(function (O) {
                      O(x);
                    });
              }
              return new (f || (f = Promise))(function (x, O) {
                function C(Z) {
                  try {
                    D(p.next(Z));
                  } catch (j) {
                    O(j);
                  }
                }
                function L(Z) {
                  try {
                    D(p.throw(Z));
                  } catch (j) {
                    O(j);
                  }
                }
                function D(Z) {
                  Z.done ? x(Z.value) : h(Z.value).then(C, L);
                }
                D((p = p.apply(m, b || [])).next());
              });
            },
          c =
            (ie && ie.__generator) ||
            function (m, b) {
              var f = {
                  label: 0,
                  sent: function () {
                    if (x[0] & 1) throw x[1];
                    return x[1];
                  },
                  trys: [],
                  ops: [],
                },
                p,
                h,
                x,
                O;
              return (
                (O = { next: C(0), throw: C(1), return: C(2) }),
                typeof Symbol == 'function' &&
                  (O[Symbol.iterator] = function () {
                    return this;
                  }),
                O
              );
              function C(D) {
                return function (Z) {
                  return L([D, Z]);
                };
              }
              function L(D) {
                if (p) throw new TypeError('Generator is already executing.');
                for (; f; )
                  try {
                    if (
                      ((p = 1),
                      h &&
                        (x =
                          D[0] & 2
                            ? h.return
                            : D[0]
                            ? h.throw || ((x = h.return) && x.call(h), 0)
                            : h.next) &&
                        !(x = x.call(h, D[1])).done)
                    )
                      return x;
                    switch (((h = 0), x && (D = [D[0] & 2, x.value]), D[0])) {
                      case 0:
                      case 1:
                        x = D;
                        break;
                      case 4:
                        return f.label++, { value: D[1], done: !1 };
                      case 5:
                        f.label++, (h = D[1]), (D = [0]);
                        continue;
                      case 7:
                        (D = f.ops.pop()), f.trys.pop();
                        continue;
                      default:
                        if (
                          ((x = f.trys),
                          !(x = x.length > 0 && x[x.length - 1]) && (D[0] === 6 || D[0] === 2))
                        ) {
                          f = 0;
                          continue;
                        }
                        if (D[0] === 3 && (!x || (D[1] > x[0] && D[1] < x[3]))) {
                          f.label = D[1];
                          break;
                        }
                        if (D[0] === 6 && f.label < x[1]) {
                          (f.label = x[1]), (x = D);
                          break;
                        }
                        if (x && f.label < x[2]) {
                          (f.label = x[2]), f.ops.push(D);
                          break;
                        }
                        x[2] && f.ops.pop(), f.trys.pop();
                        continue;
                    }
                    D = b.call(m, f);
                  } catch (Z) {
                    (D = [6, Z]), (h = 0);
                  } finally {
                    p = x = 0;
                  }
                if (D[0] & 5) throw D[1];
                return { value: D[0] ? D[1] : void 0, done: !0 };
              }
            },
          v =
            (ie && ie.__rest) ||
            function (m, b) {
              var f = {};
              for (var p in m)
                Object.prototype.hasOwnProperty.call(m, p) && b.indexOf(p) < 0 && (f[p] = m[p]);
              if (m != null && typeof Object.getOwnPropertySymbols == 'function')
                for (var h = 0, p = Object.getOwnPropertySymbols(m); h < p.length; h++)
                  b.indexOf(p[h]) < 0 &&
                    Object.prototype.propertyIsEnumerable.call(m, p[h]) &&
                    (f[p[h]] = m[p[h]]);
              return f;
            },
          w =
            (ie && ie.__importDefault) ||
            function (m) {
              return m && m.__esModule ? m : { default: m };
            };
        Object.defineProperty(r, '__esModule', { value: !0 }),
          (r.GraphQLWebSocketClient =
            r.gql =
            r.resolveRequestDocument =
            r.batchRequests =
            r.request =
            r.rawRequest =
            r.GraphQLClient =
            r.ClientError =
              void 0);
        var y = i(ki.exports),
          M = y,
          S = Sh,
          I = Ah,
          E = w(rs),
          R = Zn,
          T = Dt,
          z = Yn;
        Object.defineProperty(r, 'ClientError', {
          enumerable: !0,
          get: function () {
            return z.ClientError;
          },
        });
        var q = function (m) {
            var b = {};
            return (
              m &&
                ((typeof Headers < 'u' && m instanceof Headers) ||
                (M && M.Headers && m instanceof M.Headers)
                  ? (b = l(m))
                  : Array.isArray(m)
                  ? m.forEach(function (f) {
                      var p = f[0],
                        h = f[1];
                      b[p] = h;
                    })
                  : (b = m)),
              b
            );
          },
          Y = function (m) {
            return m.replace(/([\s,]|#[^\n\r]+)+/g, ' ').trim();
          },
          Se = function (m) {
            var b = m.query,
              f = m.variables,
              p = m.operationName,
              h = m.jsonSerializer;
            if (!Array.isArray(b)) {
              var x = ['query=' + encodeURIComponent(Y(b))];
              return (
                f && x.push('variables=' + encodeURIComponent(h.stringify(f))),
                p && x.push('operationName=' + encodeURIComponent(p)),
                x.join('&')
              );
            }
            if (typeof f < 'u' && !Array.isArray(f))
              throw new Error('Cannot create query with given variable type, array expected');
            var O = b.reduce(function (C, L, D) {
              return C.push({ query: Y(L), variables: f ? h.stringify(f[D]) : void 0 }), C;
            }, []);
            return 'query=' + encodeURIComponent(h.stringify(O));
          },
          de = function (m) {
            var b = m.url,
              f = m.query,
              p = m.variables,
              h = m.operationName,
              x = m.headers,
              O = m.fetch,
              C = m.fetchOptions,
              L = m.middleware;
            return a(void 0, void 0, void 0, function () {
              var D, Z;
              return c(this, function (j) {
                switch (j.label) {
                  case 0:
                    return (
                      (D = E.default(f, p, h, C.jsonSerializer)),
                      (Z = e(
                        {
                          method: 'POST',
                          headers: e(
                            e(
                              {},
                              typeof D == 'string' ? { 'Content-Type': 'application/json' } : {}
                            ),
                            x
                          ),
                          body: D,
                        },
                        C
                      )),
                      L ? [4, Promise.resolve(L(Z))] : [3, 2]
                    );
                  case 1:
                    (Z = j.sent()), (j.label = 2);
                  case 2:
                    return [4, O(b, Z)];
                  case 3:
                    return [2, j.sent()];
                }
              });
            });
          },
          H = function (m) {
            var b = m.url,
              f = m.query,
              p = m.variables,
              h = m.operationName,
              x = m.headers,
              O = m.fetch,
              C = m.fetchOptions,
              L = m.middleware;
            return a(void 0, void 0, void 0, function () {
              var D, Z;
              return c(this, function (j) {
                switch (j.label) {
                  case 0:
                    return (
                      (D = Se({
                        query: f,
                        variables: p,
                        operationName: h,
                        jsonSerializer: C.jsonSerializer,
                      })),
                      (Z = e({ method: 'GET', headers: x }, C)),
                      L ? [4, Promise.resolve(L(Z))] : [3, 2]
                    );
                  case 1:
                    (Z = j.sent()), (j.label = 2);
                  case 2:
                    return [4, O(b + '?' + D, Z)];
                  case 3:
                    return [2, j.sent()];
                }
              });
            });
          },
          F = (function () {
            function m(b, f) {
              f === void 0 && (f = {}), (this.url = b), (this.options = f);
            }
            return (
              (m.prototype.rawRequest = function (b, f, p) {
                return a(this, void 0, void 0, function () {
                  var h, x, O, C, L, D, Z, j, le, Ae, Q, Re;
                  return c(this, function (Ee) {
                    return (
                      (h = T.parseRawRequestArgs(b, f, p)),
                      (x = this.options),
                      (O = x.headers),
                      (C = x.fetch),
                      (L = C === void 0 ? y.default : C),
                      (D = x.method),
                      (Z = D === void 0 ? 'POST' : D),
                      (j = x.requestMiddleware),
                      (le = x.responseMiddleware),
                      (Ae = v(x, [
                        'headers',
                        'fetch',
                        'method',
                        'requestMiddleware',
                        'responseMiddleware',
                      ])),
                      (Q = this.url),
                      h.signal !== void 0 && (Ae.signal = h.signal),
                      (Re = u(h.query).operationName),
                      [
                        2,
                        J({
                          url: Q,
                          query: h.query,
                          variables: h.variables,
                          headers: e(e({}, q(s(O))), q(h.requestHeaders)),
                          operationName: Re,
                          fetch: L,
                          method: Z,
                          fetchOptions: Ae,
                          middleware: j,
                        })
                          .then(function (ne) {
                            return le && le(ne), ne;
                          })
                          .catch(function (ne) {
                            throw (le && le(ne), ne);
                          }),
                      ]
                    );
                  });
                });
              }),
              (m.prototype.request = function (b) {
                for (var f = [], p = 1; p < arguments.length; p++) f[p - 1] = arguments[p];
                var h = f[0],
                  x = f[1],
                  O = T.parseRequestArgs(b, h, x),
                  C = this.options,
                  L = C.headers,
                  D = C.fetch,
                  Z = D === void 0 ? y.default : D,
                  j = C.method,
                  le = j === void 0 ? 'POST' : j,
                  Ae = C.requestMiddleware,
                  Q = C.responseMiddleware,
                  Re = v(C, [
                    'headers',
                    'fetch',
                    'method',
                    'requestMiddleware',
                    'responseMiddleware',
                  ]),
                  Ee = this.url;
                O.signal !== void 0 && (Re.signal = O.signal);
                var ne = u(O.document),
                  ke = ne.query,
                  De = ne.operationName;
                return J({
                  url: Ee,
                  query: ke,
                  variables: O.variables,
                  headers: e(e({}, q(s(L))), q(O.requestHeaders)),
                  operationName: De,
                  fetch: Z,
                  method: le,
                  fetchOptions: Re,
                  middleware: Ae,
                })
                  .then(function (ae) {
                    return Q && Q(ae), ae.data;
                  })
                  .catch(function (ae) {
                    throw (Q && Q(ae), ae);
                  });
              }),
              (m.prototype.batchRequests = function (b, f) {
                var p = T.parseBatchRequestArgs(b, f),
                  h = this.options,
                  x = h.headers,
                  O = h.fetch,
                  C = O === void 0 ? y.default : O,
                  L = h.method,
                  D = L === void 0 ? 'POST' : L,
                  Z = h.requestMiddleware,
                  j = h.responseMiddleware,
                  le = v(h, [
                    'headers',
                    'fetch',
                    'method',
                    'requestMiddleware',
                    'responseMiddleware',
                  ]),
                  Ae = this.url;
                p.signal !== void 0 && (le.signal = p.signal);
                var Q = p.documents.map(function (Ee) {
                    var ne = Ee.document;
                    return u(ne).query;
                  }),
                  Re = p.documents.map(function (Ee) {
                    var ne = Ee.variables;
                    return ne;
                  });
                return J({
                  url: Ae,
                  query: Q,
                  variables: Re,
                  headers: e(e({}, q(s(x))), q(p.requestHeaders)),
                  operationName: void 0,
                  fetch: C,
                  method: D,
                  fetchOptions: le,
                  middleware: Z,
                })
                  .then(function (Ee) {
                    return j && j(Ee), Ee.data;
                  })
                  .catch(function (Ee) {
                    throw (j && j(Ee), Ee);
                  });
              }),
              (m.prototype.setHeaders = function (b) {
                return (this.options.headers = b), this;
              }),
              (m.prototype.setHeader = function (b, f) {
                var p,
                  h = this.options.headers;
                return h ? (h[b] = f) : (this.options.headers = ((p = {}), (p[b] = f), p)), this;
              }),
              (m.prototype.setEndpoint = function (b) {
                return (this.url = b), this;
              }),
              m
            );
          })();
        r.GraphQLClient = F;
        function J(m) {
          var b = m.url,
            f = m.query,
            p = m.variables,
            h = m.headers,
            x = m.operationName,
            O = m.fetch,
            C = m.method,
            L = C === void 0 ? 'POST' : C,
            D = m.fetchOptions,
            Z = m.middleware;
          return a(this, void 0, void 0, function () {
            var j, le, Ae, Q, Re, Ee, ne, ke, De, ae, Fe;
            return c(this, function ($e) {
              switch ($e.label) {
                case 0:
                  return (
                    (j = L.toUpperCase() === 'POST' ? de : H),
                    (le = Array.isArray(f)),
                    [
                      4,
                      j({
                        url: b,
                        query: f,
                        variables: p,
                        operationName: x,
                        headers: h,
                        fetch: O,
                        fetchOptions: D,
                        middleware: Z,
                      }),
                    ]
                  );
                case 1:
                  return (Ae = $e.sent()), [4, A(Ae, D.jsonSerializer)];
                case 2:
                  if (
                    ((Q = $e.sent()),
                    (Re =
                      le && Array.isArray(Q)
                        ? !Q.some(function (ce) {
                            var Ue = ce.data;
                            return !Ue;
                          })
                        : !!Q.data),
                    (Ee = !Q.errors || D.errorPolicy === 'all' || D.errorPolicy === 'ignore'),
                    Ae.ok && Ee && Re)
                  )
                    return (
                      (ne = Ae.headers),
                      (ke = Ae.status),
                      Q.errors,
                      (De = v(Q, ['errors'])),
                      (ae = D.errorPolicy === 'ignore' ? De : Q),
                      [2, e(e({}, le ? { data: ae } : ae), { headers: ne, status: ke })]
                    );
                  throw (
                    ((Fe = typeof Q == 'string' ? { error: Q } : Q),
                    new z.ClientError(e(e({}, Fe), { status: Ae.status, headers: Ae.headers }), {
                      query: f,
                      variables: p,
                    }))
                  );
              }
            });
          });
        }
        function X(m, b, f, p) {
          return a(this, void 0, void 0, function () {
            var h, x;
            return c(this, function (O) {
              return (
                (h = T.parseRawRequestExtendedArgs(m, b, f, p)),
                (x = new F(h.url)),
                [2, x.rawRequest(e({}, h))]
              );
            });
          });
        }
        r.rawRequest = X;
        function W(m, b) {
          for (var f = [], p = 2; p < arguments.length; p++) f[p - 2] = arguments[p];
          return a(this, void 0, void 0, function () {
            var h, x, O, C;
            return c(this, function (L) {
              return (
                (h = f[0]),
                (x = f[1]),
                (O = T.parseRequestExtendedArgs(m, b, h, x)),
                (C = new F(O.url)),
                [2, C.request(e({}, O))]
              );
            });
          });
        }
        r.request = W;
        function G(m, b, f) {
          return a(this, void 0, void 0, function () {
            var p, h;
            return c(this, function (x) {
              return (
                (p = T.parseBatchRequestsExtendedArgs(m, b, f)),
                (h = new F(p.url)),
                [2, h.batchRequests(e({}, p))]
              );
            });
          });
        }
        (r.batchRequests = G), (r.default = W);
        function A(m, b) {
          return (
            b === void 0 && (b = R.defaultJsonSerializer),
            a(this, void 0, void 0, function () {
              var f, p, h;
              return c(this, function (x) {
                switch (x.label) {
                  case 0:
                    return (
                      m.headers.forEach(function (O, C) {
                        C.toLowerCase() === 'content-type' && (f = O);
                      }),
                      f && f.toLowerCase().startsWith('application/json')
                        ? ((h = (p = b).parse), [4, m.text()])
                        : [3, 2]
                    );
                  case 1:
                    return [2, h.apply(p, [x.sent()])];
                  case 2:
                    return [2, m.text()];
                }
              });
            })
          );
        }
        function d(m) {
          var b,
            f = void 0,
            p = m.definitions.filter(function (h) {
              return h.kind === 'OperationDefinition';
            });
          return (
            p.length === 1 && (f = (b = p[0].name) === null || b === void 0 ? void 0 : b.value), f
          );
        }
        function u(m) {
          if (typeof m == 'string') {
            var b = void 0;
            try {
              var f = S.parse(m);
              b = d(f);
            } catch {}
            return { query: m, operationName: b };
          }
          var p = d(m);
          return { query: I.print(m), operationName: p };
        }
        r.resolveRequestDocument = u;
        function s(m) {
          return typeof m == 'function' ? m() : m;
        }
        function o(m) {
          for (var b = [], f = 1; f < arguments.length; f++) b[f - 1] = arguments[f];
          return m.reduce(function (p, h, x) {
            return '' + p + h + (x in b ? b[x] : '');
          }, '');
        }
        r.gql = o;
        function l(m) {
          var b = {};
          return (
            m.forEach(function (f, p) {
              b[p] = f;
            }),
            b
          );
        }
        var g = jh();
        Object.defineProperty(r, 'GraphQLWebSocketClient', {
          enumerable: !0,
          get: function () {
            return g.GraphQLWebSocketClient;
          },
        });
      })(wa)),
    wa
  );
}
var Vh = B0(),
  za = { exports: {} };
(function (r, e) {
  var t = 200,
    n = '__lodash_hash_undefined__',
    i = 9007199254740991,
    a = '[object Arguments]',
    c = '[object Array]',
    v = '[object Boolean]',
    w = '[object Date]',
    y = '[object Error]',
    M = '[object Function]',
    S = '[object GeneratorFunction]',
    I = '[object Map]',
    E = '[object Number]',
    R = '[object Object]',
    T = '[object Promise]',
    z = '[object RegExp]',
    q = '[object Set]',
    Y = '[object String]',
    Se = '[object Symbol]',
    de = '[object WeakMap]',
    H = '[object ArrayBuffer]',
    F = '[object DataView]',
    J = '[object Float32Array]',
    X = '[object Float64Array]',
    W = '[object Int8Array]',
    G = '[object Int16Array]',
    A = '[object Int32Array]',
    d = '[object Uint8Array]',
    u = '[object Uint8ClampedArray]',
    s = '[object Uint16Array]',
    o = '[object Uint32Array]',
    l = /[\\^$.*+?()[\]{}|]/g,
    g = /\w*$/,
    m = /^\[object .+?Constructor\]$/,
    b = /^(?:0|[1-9]\d*)$/,
    f = {};
  (f[a] =
    f[c] =
    f[H] =
    f[F] =
    f[v] =
    f[w] =
    f[J] =
    f[X] =
    f[W] =
    f[G] =
    f[A] =
    f[I] =
    f[E] =
    f[R] =
    f[z] =
    f[q] =
    f[Y] =
    f[Se] =
    f[d] =
    f[u] =
    f[s] =
    f[o] =
      !0),
    (f[y] = f[M] = f[de] = !1);
  var p = typeof ie == 'object' && ie && ie.Object === Object && ie,
    h = typeof self == 'object' && self && self.Object === Object && self,
    x = p || h || Function('return this')(),
    O = e && !e.nodeType && e,
    C = O && !0 && r && !r.nodeType && r,
    L = C && C.exports === O;
  function D(_, N) {
    return _.set(N[0], N[1]), _;
  }
  function Z(_, N) {
    return _.add(N), _;
  }
  function j(_, N) {
    for (var k = -1, re = _ ? _.length : 0; ++k < re && N(_[k], k, _) !== !1; );
    return _;
  }
  function le(_, N) {
    for (var k = -1, re = N.length, wt = _.length; ++k < re; ) _[wt + k] = N[k];
    return _;
  }
  function Ae(_, N, k, re) {
    var wt = -1,
      Tt = _ ? _.length : 0;
    for (re && Tt && (k = _[++wt]); ++wt < Tt; ) k = N(k, _[wt], wt, _);
    return k;
  }
  function Q(_, N) {
    for (var k = -1, re = Array(_); ++k < _; ) re[k] = N(k);
    return re;
  }
  function Re(_, N) {
    return _?.[N];
  }
  function Ee(_) {
    var N = !1;
    if (_ != null && typeof _.toString != 'function')
      try {
        N = !!(_ + '');
      } catch {}
    return N;
  }
  function ne(_) {
    var N = -1,
      k = Array(_.size);
    return (
      _.forEach(function (re, wt) {
        k[++N] = [wt, re];
      }),
      k
    );
  }
  function ke(_, N) {
    return function (k) {
      return _(N(k));
    };
  }
  function De(_) {
    var N = -1,
      k = Array(_.size);
    return (
      _.forEach(function (re) {
        k[++N] = re;
      }),
      k
    );
  }
  var ae = Array.prototype,
    Fe = Function.prototype,
    $e = Object.prototype,
    ce = x['__core-js_shared__'],
    Ue = (function () {
      var _ = /[^.]+$/.exec((ce && ce.keys && ce.keys.IE_PROTO) || '');
      return _ ? 'Symbol(src)_1.' + _ : '';
    })(),
    Ge = Fe.toString,
    oe = $e.hasOwnProperty,
    ze = $e.toString,
    rt = RegExp(
      '^' +
        Ge.call(oe)
          .replace(l, '\\$&')
          .replace(/hasOwnProperty|(function).*?(?=\\\()| for .+?(?=\\\])/g, '$1.*?') +
        '$'
    ),
    pe = L ? x.Buffer : void 0,
    Ke = x.Symbol,
    Xe = x.Uint8Array,
    be = ke(Object.getPrototypeOf, Object),
    nt = Object.create,
    it = $e.propertyIsEnumerable,
    me = ae.splice,
    Ze = Object.getOwnPropertySymbols,
    at = pe ? pe.isBuffer : void 0,
    ge = ke(Object.keys, Object),
    Je = Yr(x, 'DataView'),
    Be = Yr(x, 'Map'),
    he = Yr(x, 'Promise'),
    He = Yr(x, 'Set'),
    We = Yr(x, 'WeakMap'),
    ue = Yr(Object, 'create'),
    st = Fr(Je),
    ot = Fr(Be),
    ye = Fr(he),
    ft = Fr(He),
    ct = Fr(We),
    ve = Ke ? Ke.prototype : void 0,
    Ye = ve ? ve.valueOf : void 0;
  function Le(_) {
    var N = -1,
      k = _ ? _.length : 0;
    for (this.clear(); ++N < k; ) {
      var re = _[N];
      this.set(re[0], re[1]);
    }
  }
  function we() {
    this.__data__ = ue ? ue(null) : {};
  }
  function ut(_) {
    return this.has(_) && delete this.__data__[_];
  }
  function dt(_) {
    var N = this.__data__;
    if (ue) {
      var k = N[_];
      return k === n ? void 0 : k;
    }
    return oe.call(N, _) ? N[_] : void 0;
  }
  function xe(_) {
    var N = this.__data__;
    return ue ? N[_] !== void 0 : oe.call(N, _);
  }
  function ht(_, N) {
    var k = this.__data__;
    return (k[_] = ue && N === void 0 ? n : N), this;
  }
  (Le.prototype.clear = we),
    (Le.prototype.delete = ut),
    (Le.prototype.get = dt),
    (Le.prototype.has = xe),
    (Le.prototype.set = ht);
  function Ce(_) {
    var N = -1,
      k = _ ? _.length : 0;
    for (this.clear(); ++N < k; ) {
      var re = _[N];
      this.set(re[0], re[1]);
    }
  }
  function Me() {
    this.__data__ = [];
  }
  function lt(_) {
    var N = this.__data__,
      k = Vt(N, _);
    if (k < 0) return !1;
    var re = N.length - 1;
    return k == re ? N.pop() : me.call(N, k, 1), !0;
  }
  function pt(_) {
    var N = this.__data__,
      k = Vt(N, _);
    return k < 0 ? void 0 : N[k][1];
  }
  function _e(_) {
    return Vt(this.__data__, _) > -1;
  }
  function vt(_, N) {
    var k = this.__data__,
      re = Vt(k, _);
    return re < 0 ? k.push([_, N]) : (k[re][1] = N), this;
  }
  (Ce.prototype.clear = Me),
    (Ce.prototype.delete = lt),
    (Ce.prototype.get = pt),
    (Ce.prototype.has = _e),
    (Ce.prototype.set = vt);
  function qe(_) {
    var N = -1,
      k = _ ? _.length : 0;
    for (this.clear(); ++N < k; ) {
      var re = _[N];
      this.set(re[0], re[1]);
    }
  }
  function Qe() {
    this.__data__ = { hash: new Le(), map: new (Be || Ce)(), string: new Le() };
  }
  function pr(_) {
    return ii(this, _).delete(_);
  }
  function vr(_) {
    return ii(this, _).get(_);
  }
  function br(_) {
    return ii(this, _).has(_);
  }
  function mr(_, N) {
    return ii(this, _).set(_, N), this;
  }
  (qe.prototype.clear = Qe),
    (qe.prototype.delete = pr),
    (qe.prototype.get = vr),
    (qe.prototype.has = br),
    (qe.prototype.set = mr);
  function kt(_) {
    this.__data__ = new Ce(_);
  }
  function gr() {
    this.__data__ = new Ce();
  }
  function yr(_) {
    return this.__data__.delete(_);
  }
  function wr(_) {
    return this.__data__.get(_);
  }
  function xr(_) {
    return this.__data__.has(_);
  }
  function Mr(_, N) {
    var k = this.__data__;
    if (k instanceof Ce) {
      var re = k.__data__;
      if (!Be || re.length < t - 1) return re.push([_, N]), this;
      k = this.__data__ = new qe(re);
    }
    return k.set(_, N), this;
  }
  (kt.prototype.clear = gr),
    (kt.prototype.delete = yr),
    (kt.prototype.get = wr),
    (kt.prototype.has = xr),
    (kt.prototype.set = Mr);
  function _r(_, N) {
    var k = fa(_) || zf(_) ? Q(_.length, String) : [],
      re = k.length,
      wt = !!re;
    for (var Tt in _)
      (N || oe.call(_, Tt)) && !(wt && (Tt == 'length' || qf(Tt, re))) && k.push(Tt);
    return k;
  }
  function rr(_, N, k) {
    var re = _[N];
    (!(oe.call(_, N) && ms(re, k)) || (k === void 0 && !(N in _))) && (_[N] = k);
  }
  function Vt(_, N) {
    for (var k = _.length; k--; ) if (ms(_[k][0], N)) return k;
    return -1;
  }
  function Sr(_, N) {
    return _ && ps(N, ca(N), _);
  }
  function Xt(_, N, k, re, wt, Tt, Jt) {
    var $t;
    if ((re && ($t = Tt ? re(_, wt, Tt, Jt) : re(_)), $t !== void 0)) return $t;
    if (!ai(_)) return _;
    var ws = fa(_);
    if (ws) {
      if ((($t = $f(_)), !N)) return Cf(_, $t);
    } else {
      var Qr = Br(_),
        xs = Qr == M || Qr == S;
      if (Vf(_)) return Af(_, N);
      if (Qr == R || Qr == a || (xs && !Tt)) {
        if (Ee(_)) return Tt ? _ : {};
        if ((($t = Df(xs ? {} : _)), !N)) return Pf(_, Sr($t, _));
      } else {
        if (!f[Qr]) return Tt ? _ : {};
        $t = Lf(_, Qr, Xt, N);
      }
    }
    Jt || (Jt = new kt());
    var Ms = Jt.get(_);
    if (Ms) return Ms;
    if ((Jt.set(_, $t), !ws)) var _s = k ? kf(_) : ca(_);
    return (
      j(_s || _, function (ua, si) {
        _s && ((si = ua), (ua = _[si])), rr($t, si, Xt(ua, N, k, re, si, _, Jt));
      }),
      $t
    );
  }
  function Ar(_) {
    return ai(_) ? nt(_) : {};
  }
  function Er(_, N, k) {
    var re = N(_);
    return fa(_) ? re : le(re, k(_));
  }
  function On(_) {
    return ze.call(_);
  }
  function Tn(_) {
    if (!ai(_) || Ff(_)) return !1;
    var N = ys(_) || Ee(_) ? rt : m;
    return N.test(Fr(_));
  }
  function Sf(_) {
    if (!bs(_)) return ge(_);
    var N = [];
    for (var k in Object(_)) oe.call(_, k) && k != 'constructor' && N.push(k);
    return N;
  }
  function Af(_, N) {
    if (N) return _.slice();
    var k = new _.constructor(_.length);
    return _.copy(k), k;
  }
  function oa(_) {
    var N = new _.constructor(_.byteLength);
    return new Xe(N).set(new Xe(_)), N;
  }
  function Ef(_, N) {
    var k = N ? oa(_.buffer) : _.buffer;
    return new _.constructor(k, _.byteOffset, _.byteLength);
  }
  function If(_, N, k) {
    var re = N ? k(ne(_), !0) : ne(_);
    return Ae(re, D, new _.constructor());
  }
  function Rf(_) {
    var N = new _.constructor(_.source, g.exec(_));
    return (N.lastIndex = _.lastIndex), N;
  }
  function Nf(_, N, k) {
    var re = N ? k(De(_), !0) : De(_);
    return Ae(re, Z, new _.constructor());
  }
  function Of(_) {
    return Ye ? Object(Ye.call(_)) : {};
  }
  function Tf(_, N) {
    var k = N ? oa(_.buffer) : _.buffer;
    return new _.constructor(k, _.byteOffset, _.length);
  }
  function Cf(_, N) {
    var k = -1,
      re = _.length;
    for (N || (N = Array(re)); ++k < re; ) N[k] = _[k];
    return N;
  }
  function ps(_, N, k, re) {
    k || (k = {});
    for (var wt = -1, Tt = N.length; ++wt < Tt; ) {
      var Jt = N[wt],
        $t = re ? re(k[Jt], _[Jt], Jt, k, _) : void 0;
      rr(k, Jt, $t === void 0 ? _[Jt] : $t);
    }
    return k;
  }
  function Pf(_, N) {
    return ps(_, vs(_), N);
  }
  function kf(_) {
    return Er(_, ca, vs);
  }
  function ii(_, N) {
    var k = _.__data__;
    return Bf(N) ? k[typeof N == 'string' ? 'string' : 'hash'] : k.map;
  }
  function Yr(_, N) {
    var k = Re(_, N);
    return Tn(k) ? k : void 0;
  }
  var vs = Ze ? ke(Ze, Object) : Wf,
    Br = On;
  ((Je && Br(new Je(new ArrayBuffer(1))) != F) ||
    (Be && Br(new Be()) != I) ||
    (he && Br(he.resolve()) != T) ||
    (He && Br(new He()) != q) ||
    (We && Br(new We()) != de)) &&
    (Br = function (_) {
      var N = ze.call(_),
        k = N == R ? _.constructor : void 0,
        re = k ? Fr(k) : void 0;
      if (re)
        switch (re) {
          case st:
            return F;
          case ot:
            return I;
          case ye:
            return T;
          case ft:
            return q;
          case ct:
            return de;
        }
      return N;
    });
  function $f(_) {
    var N = _.length,
      k = _.constructor(N);
    return (
      N &&
        typeof _[0] == 'string' &&
        oe.call(_, 'index') &&
        ((k.index = _.index), (k.input = _.input)),
      k
    );
  }
  function Df(_) {
    return typeof _.constructor == 'function' && !bs(_) ? Ar(be(_)) : {};
  }
  function Lf(_, N, k, re) {
    var wt = _.constructor;
    switch (N) {
      case H:
        return oa(_);
      case v:
      case w:
        return new wt(+_);
      case F:
        return Ef(_, re);
      case J:
      case X:
      case W:
      case G:
      case A:
      case d:
      case u:
      case s:
      case o:
        return Tf(_, re);
      case I:
        return If(_, re, k);
      case E:
      case Y:
        return new wt(_);
      case z:
        return Rf(_);
      case q:
        return Nf(_, re, k);
      case Se:
        return Of(_);
    }
  }
  function qf(_, N) {
    return (
      (N = N ?? i), !!N && (typeof _ == 'number' || b.test(_)) && _ > -1 && _ % 1 == 0 && _ < N
    );
  }
  function Bf(_) {
    var N = typeof _;
    return N == 'string' || N == 'number' || N == 'symbol' || N == 'boolean'
      ? _ !== '__proto__'
      : _ === null;
  }
  function Ff(_) {
    return !!Ue && Ue in _;
  }
  function bs(_) {
    var N = _ && _.constructor,
      k = (typeof N == 'function' && N.prototype) || $e;
    return _ === k;
  }
  function Fr(_) {
    if (_ != null) {
      try {
        return Ge.call(_);
      } catch {}
      try {
        return _ + '';
      } catch {}
    }
    return '';
  }
  function Uf(_) {
    return Xt(_, !0, !0);
  }
  function ms(_, N) {
    return _ === N || (_ !== _ && N !== N);
  }
  function zf(_) {
    return jf(_) && oe.call(_, 'callee') && (!it.call(_, 'callee') || ze.call(_) == a);
  }
  var fa = Array.isArray;
  function gs(_) {
    return _ != null && Jf(_.length) && !ys(_);
  }
  function jf(_) {
    return Hf(_) && gs(_);
  }
  var Vf = at || Gf;
  function ys(_) {
    var N = ai(_) ? ze.call(_) : '';
    return N == M || N == S;
  }
  function Jf(_) {
    return typeof _ == 'number' && _ > -1 && _ % 1 == 0 && _ <= i;
  }
  function ai(_) {
    var N = typeof _;
    return !!_ && (N == 'object' || N == 'function');
  }
  function Hf(_) {
    return !!_ && typeof _ == 'object';
  }
  function ca(_) {
    return gs(_) ? _r(_) : Sf(_);
  }
  function Wf() {
    return [];
  }
  function Gf() {
    return !1;
  }
  r.exports = Uf;
})(za, za.exports);
const ja = za.exports;
var ns = (r) => {
    var e, t, n, i;
    let a, c, v;
    return (
      Array.isArray(r)
        ? ((c = r[0]), (a = (e = r[1]) != null ? e : Ht), (v = (t = r[2]) != null ? t : void 0))
        : ((c = r.amount),
          (a = (n = r.assetId) != null ? n : Ht),
          (v = (i = r.max) != null ? i : void 0)),
      { assetId: K(a), amount: B(c), max: v ? B(v) : void 0 }
    );
  },
  Jh = ((r) => ((r.Spent = 'SPENT'), (r.Unspent = 'UNSPENT'), r))(Jh || {}),
  Qn = Pe`
  fragment transactionFragment on Transaction {
    id
    rawPayload
    gasPrice
    status {
      type: __typename
      ... on SubmittedStatus {
        time
      }
      ... on SuccessStatus {
        block {
          id
        }
        time
        programState {
          returnType
          data
        }
      }
      ... on FailureStatus {
        block {
          id
        }
        time
        reason
      }
    }
  }
`,
  F0 = Pe`
  fragment receiptFragment on Receipt {
    data
    rawPayload
  }
`,
  is = Pe`
  fragment coinFragment on Coin {
    utxoId
    owner
    amount
    assetId
    maturity
    status
    blockCreated
  }
`,
  U0 = Pe`
  fragment messageFragment on Message {
    amount
    sender
    recipient
    data
    nonce
    daHeight
    fuelBlockSpend
  }
`,
  Hh = Pe`
  fragment messageProofFragment on MessageProof {
    proofSet
    proofIndex
    sender
    recipient
    nonce
    amount
    data
    signature
    header {
      id
      daHeight
      transactionsCount
      outputMessagesCount
      transactionsRoot
      outputMessagesRoot
      height
      prevRoot
      time
      applicationHash
    }
  }
`,
  z0 = Pe`
  fragment balanceFragment on Balance {
    owner
    amount
    assetId
  }
`,
  Wh = Pe`
  fragment consensusParametersFragment on ConsensusParameters {
    contractMaxSize
    maxInputs
    maxOutputs
    maxWitnesses
    maxGasPerTx
    maxScriptLength
    maxScriptDataLength
    maxStorageSlots
    maxPredicateLength
    maxPredicateDataLength
    gasPriceFactor
    gasPerByte
    maxMessageDataLength
  }
`,
  Wi = Pe`
  fragment blockFragment on Block {
    id
    header {
      height
      time
    }
    transactions {
      id
    }
  }
`,
  Gh = Pe`
  fragment chainInfoFragment on ChainInfo {
    name
    baseChainHeight
    peerCount
    consensusParameters {
      ...consensusParametersFragment
    }
    latestBlock {
      ...blockFragment
    }
  }
  ${Wh}
  ${Wi}
`,
  Kh = Pe`
  query getVersion {
    nodeInfo {
      nodeVersion
    }
  }
`,
  Xh = Pe`
  query getInfo {
    nodeInfo {
      nodeVersion
      minGasPrice
    }
  }
`,
  Zh = Pe`
  query getChain {
    chain {
      ...chainInfoFragment
    }
  }
  ${Gh}
`,
  Yh = Pe`
  query getTransaction($transactionId: TransactionId!) {
    transaction(id: $transactionId) {
      ...transactionFragment
    }
  }
  ${Qn}
`,
  Qh = Pe`
  query getTransactionWithReceipts($transactionId: TransactionId!) {
    transaction(id: $transactionId) {
      ...transactionFragment
      receipts {
        ...receiptFragment
      }
    }
  }
  ${Qn}
  ${F0}
`,
  el = Pe`
  query getTransactions($after: String, $before: String, $first: Int, $last: Int) {
    transactions(after: $after, before: $before, first: $first, last: $last) {
      edges {
        node {
          ...transactionFragment
        }
      }
    }
  }
  ${Qn}
`,
  tl = Pe`
  query getTransactionsByOwner(
    $owner: Address!
    $after: String
    $before: String
    $first: Int
    $last: Int
  ) {
    transactionsByOwner(owner: $owner, after: $after, before: $before, first: $first, last: $last) {
      edges {
        node {
          ...transactionFragment
        }
      }
    }
  }
  ${Qn}
`,
  rl = Pe`
  query getBlock($blockId: BlockId, $blockHeight: U64) {
    block(id: $blockId, height: $blockHeight) {
      ...blockFragment
    }
  }
  ${Wi}
`,
  nl = Pe`
  query getBlockWithTransactions($blockId: BlockId, $blockHeight: U64) {
    block(id: $blockId, height: $blockHeight) {
      ...blockFragment
      transactions {
        ...transactionFragment
      }
    }
  }
  ${Wi}
  ${Qn}
`,
  il = Pe`
  query getBlocks($after: String, $before: String, $first: Int, $last: Int) {
    blocks(after: $after, before: $before, first: $first, last: $last) {
      edges {
        node {
          ...blockFragment
        }
      }
    }
  }
  ${Wi}
`,
  al = Pe`
  query getCoin($coinId: UtxoId!) {
    coin(utxoId: $coinId) {
      ...coinFragment
    }
  }
  ${is}
`,
  sl = Pe`
  query getCoins(
    $filter: CoinFilterInput!
    $after: String
    $before: String
    $first: Int
    $last: Int
  ) {
    coins(filter: $filter, after: $after, before: $before, first: $first, last: $last) {
      edges {
        node {
          ...coinFragment
        }
      }
    }
  }
  ${is}
`,
  ol = Pe`
  query getResourcesToSpend(
    $owner: Address!
    $queryPerAsset: [SpendQueryElementInput!]!
    $excludedIds: ExcludeInput
  ) {
    resourcesToSpend(owner: $owner, queryPerAsset: $queryPerAsset, excludedIds: $excludedIds) {
      ...coinFragment
      ...messageFragment
    }
  }
  ${is}
  ${U0}
`,
  fl = Pe`
  query getContract($contractId: ContractId!) {
    contract(id: $contractId) {
      bytecode
      id
    }
  }
`,
  cl = Pe`
  query getBalance($owner: Address!, $assetId: AssetId!) {
    balance(owner: $owner, assetId: $assetId) {
      ...balanceFragment
    }
  }
  ${z0}
`,
  ul = Pe`
  query getBalances(
    $filter: BalanceFilterInput!
    $after: String
    $before: String
    $first: Int
    $last: Int
  ) {
    balances(filter: $filter, after: $after, before: $before, first: $first, last: $last) {
      edges {
        node {
          ...balanceFragment
        }
      }
    }
  }
  ${z0}
`,
  dl = Pe`
  query getMessages($owner: Address!, $after: String, $before: String, $first: Int, $last: Int) {
    messages(owner: $owner, after: $after, before: $before, first: $first, last: $last) {
      edges {
        node {
          ...messageFragment
        }
      }
    }
  }
  ${U0}
`,
  hl = Pe`
  query getMessageProof($transactionId: TransactionId!, $messageId: MessageId!) {
    messageProof(transactionId: $transactionId, messageId: $messageId) {
      ...messageProofFragment
    }
  }
  ${Hh}
`,
  ll = Pe`
  mutation dryRun($encodedTransaction: HexString!, $utxoValidation: Boolean) {
    dryRun(tx: $encodedTransaction, utxoValidation: $utxoValidation) {
      ...receiptFragment
    }
  }
  ${F0}
`,
  pl = Pe`
  mutation submit($encodedTransaction: HexString!) {
    submit(tx: $encodedTransaction) {
      id
    }
  }
`,
  vl = Pe`
  mutation startSession {
    startSession
  }
`,
  bl = Pe`
  mutation endSession($sessionId: ID!) {
    endSession(id: $sessionId)
  }
`,
  ml = Pe`
  mutation execute($sessionId: ID!, $op: String!) {
    execute(id: $sessionId, op: $op)
  }
`,
  gl = Pe`
  mutation reset($sessionId: ID!) {
    reset(id: $sessionId)
  }
`,
  yl = (r, e, t) => r();
function wl(r, e = yl) {
  return {
    getVersion(t, n) {
      return e((i) => r.request(Kh, t, { ...n, ...i }), 'getVersion', 'query');
    },
    getInfo(t, n) {
      return e((i) => r.request(Xh, t, { ...n, ...i }), 'getInfo', 'query');
    },
    getChain(t, n) {
      return e((i) => r.request(Zh, t, { ...n, ...i }), 'getChain', 'query');
    },
    getTransaction(t, n) {
      return e((i) => r.request(Yh, t, { ...n, ...i }), 'getTransaction', 'query');
    },
    getTransactionWithReceipts(t, n) {
      return e((i) => r.request(Qh, t, { ...n, ...i }), 'getTransactionWithReceipts', 'query');
    },
    getTransactions(t, n) {
      return e((i) => r.request(el, t, { ...n, ...i }), 'getTransactions', 'query');
    },
    getTransactionsByOwner(t, n) {
      return e((i) => r.request(tl, t, { ...n, ...i }), 'getTransactionsByOwner', 'query');
    },
    getBlock(t, n) {
      return e((i) => r.request(rl, t, { ...n, ...i }), 'getBlock', 'query');
    },
    getBlockWithTransactions(t, n) {
      return e((i) => r.request(nl, t, { ...n, ...i }), 'getBlockWithTransactions', 'query');
    },
    getBlocks(t, n) {
      return e((i) => r.request(il, t, { ...n, ...i }), 'getBlocks', 'query');
    },
    getCoin(t, n) {
      return e((i) => r.request(al, t, { ...n, ...i }), 'getCoin', 'query');
    },
    getCoins(t, n) {
      return e((i) => r.request(sl, t, { ...n, ...i }), 'getCoins', 'query');
    },
    getResourcesToSpend(t, n) {
      return e((i) => r.request(ol, t, { ...n, ...i }), 'getResourcesToSpend', 'query');
    },
    getContract(t, n) {
      return e((i) => r.request(fl, t, { ...n, ...i }), 'getContract', 'query');
    },
    getBalance(t, n) {
      return e((i) => r.request(cl, t, { ...n, ...i }), 'getBalance', 'query');
    },
    getBalances(t, n) {
      return e((i) => r.request(ul, t, { ...n, ...i }), 'getBalances', 'query');
    },
    getMessages(t, n) {
      return e((i) => r.request(dl, t, { ...n, ...i }), 'getMessages', 'query');
    },
    getMessageProof(t, n) {
      return e((i) => r.request(hl, t, { ...n, ...i }), 'getMessageProof', 'query');
    },
    dryRun(t, n) {
      return e((i) => r.request(ll, t, { ...n, ...i }), 'dryRun', 'mutation');
    },
    submit(t, n) {
      return e((i) => r.request(pl, t, { ...n, ...i }), 'submit', 'mutation');
    },
    startSession(t, n) {
      return e((i) => r.request(vl, t, { ...n, ...i }), 'startSession', 'mutation');
    },
    endSession(t, n) {
      return e((i) => r.request(bl, t, { ...n, ...i }), 'endSession', 'mutation');
    },
    execute(t, n) {
      return e((i) => r.request(ml, t, { ...n, ...i }), 'execute', 'mutation');
    },
    reset(t, n) {
      return e((i) => r.request(gl, t, { ...n, ...i }), 'reset', 'mutation');
    },
  };
}
var xl = (r) => 'utxoId' in r,
  li = (r) => 'id' in r,
  Ml = (r) => {
    var e, t, n, i, a;
    switch (r.type) {
      case Rt.Coin: {
        let c = V((e = r.predicate) != null ? e : '0x'),
          v = V((t = r.predicateData) != null ? t : '0x');
        return {
          type: Rt.Coin,
          utxoID: { transactionId: K(V(r.id).slice(0, 32)), outputIndex: V(r.id)[32] },
          owner: K(r.owner),
          amount: B(r.amount),
          assetId: K(r.assetId),
          txPointer: {
            blockHeight: nr(V(r.txPointer).slice(0, 8)),
            txIndex: nr(V(r.txPointer).slice(8, 16)),
          },
          witnessIndex: r.witnessIndex,
          maturity: (n = r.maturity) != null ? n : 0,
          predicateLength: c.length,
          predicateDataLength: v.length,
          predicate: K(c),
          predicateData: K(v),
        };
      }
      case Rt.Contract:
        return {
          type: Rt.Contract,
          utxoID: { transactionId: gt, outputIndex: 0 },
          balanceRoot: gt,
          stateRoot: gt,
          txPointer: {
            blockHeight: nr(V(r.txPointer).slice(0, 8)),
            txIndex: nr(V(r.txPointer).slice(8, 16)),
          },
          contractID: K(r.contractId),
        };
      case Rt.Message: {
        let c = V((i = r.predicate) != null ? i : '0x'),
          v = V((a = r.predicateData) != null ? a : '0x');
        return {
          type: Rt.Message,
          sender: K(r.sender),
          recipient: K(r.recipient),
          amount: B(r.amount),
          nonce: B(r.nonce),
          witnessIndex: r.witnessIndex,
          dataLength: r.data.length,
          predicateLength: c.length,
          predicateDataLength: v.length,
          data: K(r.data),
          predicate: K(c),
          predicateData: K(v),
        };
      }
      default:
        throw new Error('Invalid Input type');
    }
  },
  _l = (r) => {
    switch (r.type) {
      case je.Coin:
        return { type: je.Coin, to: K(r.to), amount: B(r.amount), assetId: K(r.assetId) };
      case je.Contract:
        return { type: je.Contract, inputIndex: r.inputIndex, balanceRoot: gt, stateRoot: gt };
      case je.Message:
        return { type: je.Message, recipient: K(r.recipient), amount: B(r.amount) };
      case je.Change:
        return { type: je.Change, to: K(r.to), amount: B(0), assetId: K(r.assetId) };
      case je.Variable:
        return { type: je.Variable, to: gt, amount: B(0), assetId: gt };
      case je.ContractCreated:
        return { type: je.ContractCreated, contractId: K(r.contractId), stateRoot: K(r.stateRoot) };
      default:
        throw new Error('Invalid Output type');
    }
  },
  bo = (r) => {
    if (r.length == null && typeof r == 'object') {
      let e = Object.keys(r).length;
      return V({ ...r, length: e });
    }
    return V(r);
  },
  j0 = (r, e, t) => B(Math.ceil(r.toNumber() / t.toNumber()) * e.toNumber()),
  Sl = (r) => {
    let e = r.find((t) => t.type === Nt.ScriptResult);
    return e && e.type === Nt.ScriptResult ? e.gasUsed : B(0);
  };
function Al(r = 1e3) {
  return new Promise((e) => {
    setTimeout(() => {
      e(!0);
    }, r);
  });
}
var El = (r) => r.type === Nt.Revert && r.val.toString('hex') === _h,
  Il = (r) =>
    r.type === Nt.Panic &&
    r.contractId !== '0x0000000000000000000000000000000000000000000000000000000000000000',
  Rl = (r) =>
    r.reduce(
      (e, t) => (
        El(t) && e.missingOutputVariables.push(t), Il(t) && e.missingOutputContractIds.push(t), e
      ),
      { missingOutputVariables: [], missingOutputContractIds: [] }
    ),
  Va = ({ receipts: r, gasPrice: e, margin: t }) => {
    let n = ic(Sl(r), t || 1),
      i = j0(n, e, $0);
    return { gasUsed: n, fee: i };
  },
  Nl = (r) => {
    let e = new Uint8Array(32);
    return e.set(V(r)), e;
  },
  Ol = (r) => {
    let e, t;
    return (
      Array.isArray(r) ? ((e = r[0]), (t = r[1])) : ((e = r.key), (t = r.value)),
      { key: K(e), value: K(Nl(t)) }
    );
  },
  Tl = (r) => {
    let e = V(r);
    return { data: K(e), dataLength: e.length };
  },
  mo = { bytes: V('0x24000000'), encodeScriptData: () => new Uint8Array(0) },
  Cl = {
    bytes: V('0x5040C0105D44C0064C40001124000000'),
    encodeScriptData: () => new Uint8Array(0),
  },
  Pl = class extends Error {
    constructor() {
      super(...arguments),
        (this.name = 'ChangeOutputCollisionError'),
        (this.message =
          'A ChangeOutput with the same "assetId" already exists for a different "to" address');
    }
  },
  kl = class extends Error {
    constructor(r) {
      super(),
        (this.index = r),
        (this.name = 'NoWitnessAtIndexError'),
        (this.message = `Witness at index "${r}" was not found`);
    }
  },
  $l = class extends Error {
    constructor(r) {
      super(),
        (this.owner = r),
        (this.name = 'NoWitnessByOwnerError'),
        (this.message = `A witness for the given owner "${r}" was not found`);
    }
  },
  V0 = class {
    constructor({
      gasPrice: e,
      gasLimit: t,
      maturity: n,
      inputs: i,
      outputs: a,
      witnesses: c,
    } = {}) {
      (this.inputs = []),
        (this.outputs = []),
        (this.witnesses = []),
        (this.gasPrice = B(e ?? 0)),
        (this.gasLimit = B(t ?? 0)),
        (this.maturity = n ?? 0),
        (this.inputs = [...(i ?? [])]),
        (this.outputs = [...(a ?? [])]),
        (this.witnesses = [...(c ?? [])]);
    }
    getBaseTransaction() {
      var e, t, n, i, a, c;
      let v = (t = (e = this.inputs) == null ? void 0 : e.map(Ml)) != null ? t : [],
        w = (i = (n = this.outputs) == null ? void 0 : n.map(_l)) != null ? i : [],
        y = (c = (a = this.witnesses) == null ? void 0 : a.map(Tl)) != null ? c : [];
      return {
        gasPrice: this.gasPrice,
        gasLimit: this.gasLimit,
        maturity: this.maturity,
        inputs: v,
        outputs: w,
        witnesses: y,
        inputsCount: v.length,
        outputsCount: w.length,
        witnessesCount: y.length,
      };
    }
    toTransactionBytes() {
      return new Wn().encode(this.toTransaction());
    }
    pushInput(e) {
      return this.inputs.push(e), this.inputs.length - 1;
    }
    pushOutput(e) {
      return this.outputs.push(e), this.outputs.length - 1;
    }
    createWitness() {
      return this.witnesses.push('0x'), this.witnesses.length - 1;
    }
    updateWitnessByOwner(e, t) {
      let n = this.getCoinInputWitnessIndexByOwner(e);
      typeof n == 'number' && this.updateWitness(n, t);
    }
    updateWitness(e, t) {
      if (!this.witnesses[e]) throw new kl(e);
      this.witnesses[e] = t;
    }
    getCoinInputs() {
      return this.inputs.filter((e) => e.type === Rt.Coin);
    }
    getCoinOutputs() {
      return this.outputs.filter((e) => e.type === je.Coin);
    }
    getChangeOutputs() {
      return this.outputs.filter((e) => e.type === je.Change);
    }
    getCoinInputWitnessIndexByOwner(e) {
      var t, n;
      let i = an(e);
      return (n =
        (t = this.inputs.find((a) => a.type === Rt.Coin && K(a.owner) === i.toB256())) == null
          ? void 0
          : t.witnessIndex) != null
        ? n
        : null;
    }
    updateWitnessByCoinInputOwner(e, t) {
      let n = this.getCoinInputWitnessIndexByOwner(e);
      if (!n) throw new $l(an(e));
      this.updateWitness(n, t);
    }
    addResource(e) {
      let t = li(e) ? e.owner : e.recipient,
        n = li(e) ? e.assetId : Ht,
        i = li(e) ? Rt.Coin : Rt.Message,
        a = this.getCoinInputWitnessIndexByOwner(t);
      typeof a != 'number' && (a = this.createWitness()),
        this.pushInput(
          li(e)
            ? {
                type: i,
                ...e,
                owner: e.owner.toB256(),
                witnessIndex: a,
                txPointer: '0x00000000000000000000000000000000',
              }
            : {
                type: i,
                ...e,
                sender: e.sender.toB256(),
                recipient: e.recipient.toB256(),
                witnessIndex: a,
                txPointer: '0x00000000000000000000000000000000',
              }
        );
      let c = this.getChangeOutputs().find((v) => K(v.assetId) === n);
      if (c && K(c.to) !== t.toB256()) throw new Pl();
      c || this.pushOutput({ type: je.Change, to: t.toB256(), assetId: n });
    }
    addResources(e) {
      e.forEach((t) => this.addResource(t));
    }
    addCoinOutput(e, t, n = Ht) {
      this.pushOutput({ type: je.Coin, to: an(e).toB256(), amount: t, assetId: n });
    }
    addCoinOutputs(e, t) {
      t.map(ns).forEach((n) => {
        this.pushOutput({
          type: je.Coin,
          to: an(e).toB256(),
          amount: n.amount,
          assetId: n.assetId,
        });
      });
    }
    byteSize() {
      return this.toTransactionBytes().length;
    }
    chargeableByteSize() {
      let e = this.witnesses.reduce((t, n) => t + V(n).length, 0);
      return B(this.toTransactionBytes().length - e);
    }
    calculateFee() {
      let e = j0(this.gasLimit, this.gasPrice, $0);
      return { assetId: Ht, amount: e.isZero() ? B(1) : e };
    }
    addMessage(e) {
      let t = this.getCoinInputWitnessIndexByOwner(e.recipient);
      typeof t != 'number' && (t = this.createWitness()),
        this.pushInput({
          type: Rt.Message,
          ...e,
          sender: e.sender.toBytes(),
          recipient: e.recipient.toBytes(),
          witnessIndex: t,
        });
    }
    addMessages(e) {
      e.forEach((t) => this.addMessage(t));
    }
  },
  Jr = class extends V0 {
    constructor({ script: e, scriptData: t, ...n } = {}) {
      super(n),
        (this.type = Cr.Script),
        (this.script = bo(e ?? mo.bytes)),
        (this.scriptData = bo(t ?? mo.encodeScriptData()));
    }
    static from(e) {
      return e instanceof this ? e : new this(e);
    }
    toTransaction() {
      var e, t;
      let n = V((e = this.script) != null ? e : '0x'),
        i = V((t = this.scriptData) != null ? t : '0x');
      return {
        type: Cr.Script,
        ...super.getBaseTransaction(),
        scriptLength: n.length,
        scriptDataLength: i.length,
        receiptsRoot: gt,
        script: K(n),
        scriptData: K(i),
      };
    }
    getContractInputs() {
      return this.inputs.filter((e) => e.type === Rt.Contract);
    }
    getContractOutputs() {
      return this.outputs.filter((e) => e.type === je.Contract);
    }
    getVariableOutputs() {
      return this.outputs.filter((e) => e.type === je.Variable);
    }
    setScript(e, t) {
      (this.script = e.bytes),
        (this.scriptData = e.encodeScriptData(t)),
        this.bytesOffset === void 0 && (this.bytesOffset = this.scriptData.byteLength);
    }
    addVariableOutputs(e = 1) {
      let t = e;
      for (; t; ) this.pushOutput({ type: je.Variable }), (t -= 1);
      return this.outputs.length - 1;
    }
    addMessageOutputs(e = 1) {
      let t = e;
      for (; t; )
        this.pushOutput({
          type: je.Message,
          recipient: '0x0000000000000000000000000000000000000000000000000000000000000000',
          amount: 0,
        }),
          (t -= 1);
      return this.outputs.length - 1;
    }
    addContract(e) {
      let t = an(e);
      if (this.getContractInputs().find((i) => i.contractId === t.toB256())) return;
      let n = super.pushInput({
        type: Rt.Contract,
        contractId: t.toB256(),
        txPointer: '0x00000000000000000000000000000000',
      });
      this.pushOutput({ type: je.Contract, inputIndex: n });
    }
  },
  go = class extends V0 {
    constructor({ bytecodeWitnessIndex: e, salt: t, storageSlots: n, ...i } = {}) {
      super(i),
        (this.type = Cr.Create),
        (this.bytecodeWitnessIndex = e ?? 0),
        (this.salt = K(t ?? gt)),
        (this.storageSlots = [...(n ?? [])]);
    }
    static from(e) {
      return e instanceof this ? e : new this(e);
    }
    toTransaction() {
      var e, t;
      let n = this.getBaseTransaction(),
        i = this.bytecodeWitnessIndex,
        a = (t = (e = this.storageSlots) == null ? void 0 : e.map(Ol)) != null ? t : [];
      return {
        type: Cr.Create,
        ...n,
        bytecodeLength: n.witnesses[i].dataLength / 4,
        bytecodeWitnessIndex: i,
        storageSlotsCount: a.length,
        salt: this.salt ? K(this.salt) : gt,
        storageSlots: a,
      };
    }
    getContractCreatedOutputs() {
      return this.outputs.filter((e) => e.type === je.ContractCreated);
    }
    addContractCreatedOutput(e, t) {
      this.pushOutput({ type: je.ContractCreated, contractId: e, stateRoot: t });
    }
  },
  er = (r) => {
    if (r instanceof Jr || r instanceof go) return r;
    switch (r.type) {
      case Cr.Script:
        return Jr.from(r);
      case Cr.Create:
        return go.from(r);
      default:
        throw new Error(`Unknown transaction type: ${r.type}`);
    }
  },
  Dl = 5e3,
  Ll = 500,
  yo = (r) => {
    let e = new k0().decode(V(r.rawPayload), 0)[0];
    switch (e.type) {
      case Nt.ReturnData:
        return { ...e, data: r.data };
      case Nt.LogData:
        return { ...e, data: r.data };
      default:
        return e;
    }
  },
  ql = class {
    constructor(e, t) {
      (this.gasUsed = B(0)), (this.attempts = 0), (this.id = e), (this.provider = t);
    }
    async fetch() {
      var e;
      let { transaction: t } = await this.provider.operations.getTransactionWithReceipts({
        transactionId: this.id,
      });
      if (!t) throw new Error('No Transaction was received from the client.');
      let n = (e = new Wn().decode(V(t.rawPayload), 0)) == null ? void 0 : e[0];
      return { transactionWithReceipts: t, transaction: n };
    }
    async waitForResult() {
      var e, t;
      let { transactionWithReceipts: n, transaction: i } = await this.fetch();
      switch ((e = n.status) == null ? void 0 : e.type) {
        case 'SubmittedStatus':
          return (
            (this.attempts += 1), await Al(Math.min(Ll * this.attempts, Dl)), this.waitForResult()
          );
        case 'FailureStatus': {
          let a = n.receipts.map(yo),
            { gasUsed: c, fee: v } = Va({ receipts: a, gasPrice: B(n?.gasPrice) });
          return (
            (this.gasUsed = c),
            {
              status: { type: 'failure', reason: n.status.reason },
              receipts: a,
              transactionId: this.id,
              blockId: n.status.block.id,
              time: n.status.time,
              gasUsed: c,
              fee: v,
              transaction: i,
            }
          );
        }
        case 'SuccessStatus': {
          let a = ((t = n.receipts) == null ? void 0 : t.map(yo)) || [],
            { gasUsed: c, fee: v } = Va({ receipts: a, gasPrice: B(n?.gasPrice) });
          return {
            status: { type: 'success', programState: n.status.programState },
            receipts: a,
            transactionId: this.id,
            blockId: n.status.block.id,
            time: n.status.time,
            gasUsed: c,
            fee: v,
            transaction: i,
          };
        }
        default:
          throw new Error('Invalid Transaction status');
      }
    }
    async wait() {
      let e = await this.waitForResult();
      if (e.status.type === 'failure') throw new Error(`Transaction failed: ${e.status.reason}`);
      return e;
    }
  },
  Bl = 10,
  xa = (r) => {
    let e = new k0().decode(V(r.rawPayload), 0)[0];
    switch (e.type) {
      case Nt.ReturnData:
        return { ...e, data: r.data };
      case Nt.LogData:
        return { ...e, data: r.data };
      default:
        return e;
    }
  },
  Fl = (r) => {
    let { name: e, baseChainHeight: t, peerCount: n, consensusParameters: i, latestBlock: a } = r;
    return {
      name: e,
      baseChainHeight: B(t),
      peerCount: n,
      consensusParameters: {
        contractMaxSize: B(i.contractMaxSize),
        maxInputs: B(i.maxInputs),
        maxOutputs: B(i.maxOutputs),
        maxWitnesses: B(i.maxWitnesses),
        maxGasPerTx: B(i.maxGasPerTx),
        maxScriptLength: B(i.maxScriptLength),
        maxScriptDataLength: B(i.maxScriptDataLength),
        maxStorageSlots: B(i.maxStorageSlots),
        maxPredicateLength: B(i.maxPredicateLength),
        maxPredicateDataLength: B(i.maxPredicateDataLength),
        gasPriceFactor: B(i.gasPriceFactor),
        gasPerByte: B(i.gasPerByte),
        maxMessageDataLength: B(i.maxMessageDataLength),
      },
      latestBlock: {
        id: a.id,
        height: B(a.header.height),
        time: a.header.time,
        transactions: a.transactions.map((c) => ({ id: c.id })),
      },
    };
  },
  Ul = (r) => ({ minGasPrice: B(r.minGasPrice), nodeVersion: r.nodeVersion }),
  zl = class {
    constructor(r) {
      (this.url = r),
        (this.addMissingVariables = async (t) => {
          let n = 0,
            i = 0,
            a = 0;
          if (t.type !== Cr.Create)
            do {
              let c = K(t.toTransactionBytes()),
                { dryRun: v } = await this.operations.dryRun({
                  encodedTransaction: c,
                  utxoValidation: !1,
                }),
                w = v.map(xa),
                { missingOutputVariables: y, missingOutputContractIds: M } = Rl(w);
              if (((n = y.length), (i = M.length), n === 0 && i === 0)) return;
              t.addVariableOutputs(n),
                M.forEach(({ contractId: S }) => t.addContract(yt.fromString(S))),
                (a += 1);
            } while (a < Bl);
        });
      let e = new Vh.GraphQLClient(r);
      this.operations = wl(e);
    }
    async getVersion() {
      let {
        nodeInfo: { nodeVersion: r },
      } = await this.operations.getVersion();
      return r;
    }
    async getNetwork() {
      return { name: 'fuelv2', chainId: 3735928559 };
    }
    async getBlockNumber() {
      let { chain: r } = await this.operations.getChain();
      return B(r.latestBlock.header.height, 10);
    }
    async getNodeInfo() {
      let { nodeInfo: r } = await this.operations.getInfo();
      return Ul(r);
    }
    async getChain() {
      let { chain: r } = await this.operations.getChain();
      return Fl(r);
    }
    async sendTransaction(r) {
      let e = er(r);
      await this.addMissingVariables(e);
      let t = K(e.toTransactionBytes()),
        { gasUsed: n, minGasPrice: i } = await this.getTransactionCost(e, 0);
      if (B(n).gt(B(e.gasLimit)))
        throw new Error(`gasLimit(${e.gasLimit}) is lower than the required (${n})`);
      if (B(i).gt(B(e.gasPrice)))
        throw new Error(`gasPrice(${e.gasPrice}) is lower than the required ${i}`);
      let {
        submit: { id: a },
      } = await this.operations.submit({ encodedTransaction: t });
      return new ql(a, this);
    }
    async call(r, { utxoValidation: e } = {}) {
      let t = er(r);
      await this.addMissingVariables(t);
      let n = K(t.toTransactionBytes()),
        { dryRun: i } = await this.operations.dryRun({
          encodedTransaction: n,
          utxoValidation: e || !1,
        });
      return { receipts: i.map(xa) };
    }
    async simulate(r) {
      let e = er(r);
      await this.addMissingVariables(e);
      let t = K(e.toTransactionBytes()),
        { dryRun: n } = await this.operations.dryRun({ encodedTransaction: t, utxoValidation: !0 });
      return { receipts: n.map(xa) };
    }
    async getTransactionCost(r, e = 0.2) {
      let t = er(ja(r)),
        { minGasPrice: n } = await this.getNodeInfo(),
        i = nc(t.gasPrice, n),
        a = 1 + e;
      (t.gasLimit = zn), (t.gasPrice = B(0));
      let { receipts: c } = await this.call(t),
        { gasUsed: v, fee: w } = Va({ gasPrice: i, receipts: c, margin: a });
      return { minGasPrice: n, gasPrice: i, gasUsed: v, fee: w };
    }
    async getCoins(r, e, t) {
      return (
        await this.operations.getCoins({
          first: 10,
          ...t,
          filter: { owner: r.toB256(), assetId: e && K(e) },
        })
      ).coins.edges
        .map((n) => n.node)
        .map((n) => ({
          id: n.utxoId,
          assetId: n.assetId,
          amount: B(n.amount),
          owner: yt.fromAddressOrString(n.owner),
          status: n.status,
          maturity: B(n.maturity).toNumber(),
          blockCreated: B(n.blockCreated),
        }));
    }
    async getResourcesToSpend(r, e, t) {
      var n, i;
      let a = {
        messages: ((n = t?.messages) == null ? void 0 : n.map((c) => K(c))) || [],
        utxos: ((i = t?.utxos) == null ? void 0 : i.map((c) => K(c))) || [],
      };
      return (
        await this.operations.getResourcesToSpend({
          owner: r.toB256(),
          queryPerAsset: e.map(ns).map(({ assetId: c, amount: v, max: w }) => ({
            assetId: K(c),
            amount: v.toString(10),
            max: w ? w.toString(10) : void 0,
          })),
          excludedIds: a,
        })
      ).resourcesToSpend
        .flat()
        .map((c) =>
          xl(c)
            ? {
                id: c.utxoId,
                amount: B(c.amount),
                status: c.status,
                assetId: c.assetId,
                owner: yt.fromAddressOrString(c.owner),
                maturity: B(c.maturity).toNumber(),
                blockCreated: B(c.blockCreated),
              }
            : {
                sender: yt.fromAddressOrString(c.sender),
                recipient: yt.fromAddressOrString(c.recipient),
                nonce: B(c.nonce),
                amount: B(c.amount),
                data: Hn.decodeData(c.data),
                daHeight: B(c.daHeight),
                fuelBlockSpend: B(c.fuelBlockSpend),
              }
        );
    }
    async getBlock(r) {
      let e;
      typeof r == 'number'
        ? (e = { blockHeight: B(r).toString(10) })
        : r === 'latest'
        ? (e = { blockHeight: (await this.getBlockNumber()).toString(10) })
        : (e = { blockId: B(r).toString(10) });
      let { block: t } = await this.operations.getBlock(e);
      return t
        ? {
            id: t.id,
            height: B(t.header.height),
            time: t.header.time,
            transactionIds: t.transactions.map((n) => n.id),
          }
        : null;
    }
    async getBlockWithTransactions(r) {
      let e;
      typeof r == 'number'
        ? (e = { blockHeight: B(r).toString(10) })
        : r === 'latest'
        ? (e = { blockHeight: (await this.getBlockNumber()).toString() })
        : (e = { blockId: r });
      let { block: t } = await this.operations.getBlockWithTransactions(e);
      return t
        ? {
            id: t.id,
            height: B(t.header.height, 10),
            time: t.header.time,
            transactionIds: t.transactions.map((n) => n.id),
            transactions: t.transactions.map((n) => {
              var i;
              return (i = new Wn().decode(V(n.rawPayload), 0)) == null ? void 0 : i[0];
            }),
          }
        : null;
    }
    async getTransaction(r) {
      var e;
      let { transaction: t } = await this.operations.getTransaction({ transactionId: r });
      return t ? ((e = new Wn().decode(V(t.rawPayload), 0)) == null ? void 0 : e[0]) : null;
    }
    async getContract(r) {
      let { contract: e } = await this.operations.getContract({ contractId: r });
      return e || null;
    }
    async getBalance(r, e) {
      let { balance: t } = await this.operations.getBalance({ owner: r.toB256(), assetId: K(e) });
      return B(t.amount, 10);
    }
    async getBalances(r, e) {
      return (
        await this.operations.getBalances({ first: 10, ...e, filter: { owner: r.toB256() } })
      ).balances.edges
        .map((t) => t.node)
        .map((t) => ({ assetId: t.assetId, amount: B(t.amount) }));
    }
    async getMessages(r, e) {
      return (
        await this.operations.getMessages({ first: 10, ...e, owner: r.toB256() })
      ).messages.edges
        .map((t) => t.node)
        .map((t) => ({
          sender: yt.fromAddressOrString(t.sender),
          recipient: yt.fromAddressOrString(t.recipient),
          nonce: B(t.nonce),
          amount: B(t.amount),
          data: Hn.decodeData(t.data),
          daHeight: B(t.daHeight),
          fuelBlockSpend: B(t.fuelBlockSpend),
        }));
    }
    async getMessageProof(r, e) {
      let t = await this.operations.getMessageProof({ transactionId: r, messageId: e });
      return t.messageProof
        ? {
            proofSet: t.messageProof.proofSet,
            proofIndex: B(t.messageProof.proofIndex),
            sender: yt.fromAddressOrString(t.messageProof.sender),
            recipient: yt.fromAddressOrString(t.messageProof.recipient),
            nonce: t.messageProof.nonce,
            amount: B(t.messageProof.amount),
            data: t.messageProof.data,
            signature: t.messageProof.signature,
            header: {
              id: t.messageProof.header.id,
              daHeight: B(t.messageProof.header.daHeight),
              transactionsCount: B(t.messageProof.header.transactionsCount),
              outputMessagesCount: B(t.messageProof.header.outputMessagesCount),
              transactionsRoot: t.messageProof.header.transactionsRoot,
              outputMessagesRoot: t.messageProof.header.outputMessagesRoot,
              height: B(t.messageProof.header.height),
              prevRoot: t.messageProof.header.prevRoot,
              time: t.messageProof.header.time,
              applicationHash: t.messageProof.header.applicationHash,
            },
          }
        : null;
    }
    async buildSpendPredicate(r, e, t, n, i = Ht, a, c) {
      let v = await this.getResourcesToSpend(r.address, [[e, i]]),
        w = { fundTransaction: !0, ...a },
        y = new Jr({ gasLimit: zn, ...w }),
        M;
      n && r.types && (M = new Ai().encode(r.types, n));
      let S = v.reduce(
        (E, R) => (
          y.addResource({ ...R, predicate: r.bytes, predicateData: M }),
          (y.outputs = []),
          E.add(R.amount)
        ),
        B(0)
      );
      y.addCoinOutput(t, S, i);
      let I = [];
      if ((w.fundTransaction && I.push(y.calculateFee()), I.length && c)) {
        let E = await this.getResourcesToSpend(c, I);
        y.addResources(E);
      }
      return y;
    }
    async submitSpendPredicate(r, e, t, n, i = Ht, a, c) {
      var v;
      let w = await this.buildSpendPredicate(r, e, t, n, i, a, c);
      try {
        return await (await this.sendTransaction(w)).waitForResult();
      } catch (y) {
        throw (((v = y?.response) == null ? void 0 : v.errors) || []).some(({ message: M }) =>
          M.includes('unexpected block execution error TransactionValidity(InvalidPredicate')
        )
          ? new Error('Invalid Predicate')
          : y;
      }
    }
  };
function jl(r) {
  return Pt(Rr(r, 'utf-8'));
}
function Vl(r) {
  let e = er(r).toTransaction();
  return (
    e.type === Cr.Script && (e.receiptsRoot = gt),
    (e.inputs = e.inputs.map((t) => {
      let n = ja(t);
      switch (n.type) {
        case Rt.Coin:
          return n;
        case Rt.Contract:
          return (
            (n.utxoID = { transactionId: gt, outputIndex: 0 }),
            (n.balanceRoot = gt),
            (n.stateRoot = gt),
            n
          );
        default:
          return n;
      }
    })),
    (e.outputs = e.outputs.map((t) => {
      let n = ja(t);
      switch (n.type) {
        case je.Contract:
          return (n.balanceRoot = gt), (n.stateRoot = gt), n;
        case je.Change:
          return (n.amount = B(0)), n;
        case je.Variable:
          return (n.to = gt), (n.amount = B(0)), (n.assetId = gt), n;
        default:
          return n;
      }
    })),
    (e.witnessesCount = 0),
    (e.witnesses = []),
    Pt(new Wn().encode(e))
  );
}
function Jl(r) {
  return Pt(r);
}
var J0 = {};
const Hl = 'elliptic',
  Wl = '6.5.4',
  Gl = 'EC cryptography',
  Kl = 'lib/elliptic.js',
  Xl = ['lib'],
  Zl = {
    lint: 'eslint lib test',
    'lint:fix': 'npm run lint -- --fix',
    unit: 'istanbul test _mocha --reporter=spec test/index.js',
    test: 'npm run lint && npm run unit',
    version: 'grunt dist && git add dist/',
  },
  Yl = { type: 'git', url: 'git@github.com:indutny/elliptic' },
  Ql = ['EC', 'Elliptic', 'curve', 'Cryptography'],
  ep = 'Fedor Indutny <fedor@indutny.com>',
  tp = 'MIT',
  rp = { url: 'https://github.com/indutny/elliptic/issues' },
  np = 'https://github.com/indutny/elliptic',
  ip = {
    brfs: '^2.0.2',
    coveralls: '^3.1.0',
    eslint: '^7.6.0',
    grunt: '^1.2.1',
    'grunt-browserify': '^5.3.0',
    'grunt-cli': '^1.3.2',
    'grunt-contrib-connect': '^3.0.0',
    'grunt-contrib-copy': '^1.0.0',
    'grunt-contrib-uglify': '^5.0.0',
    'grunt-mocha-istanbul': '^5.0.2',
    'grunt-saucelabs': '^9.0.1',
    istanbul: '^0.4.5',
    mocha: '^8.0.1',
  },
  ap = {
    'bn.js': '^4.11.9',
    brorand: '^1.1.0',
    'hash.js': '^1.0.0',
    'hmac-drbg': '^1.0.1',
    inherits: '^2.0.4',
    'minimalistic-assert': '^1.0.1',
    'minimalistic-crypto-utils': '^1.0.1',
  },
  sp = {
    name: Hl,
    version: Wl,
    description: Gl,
    main: Kl,
    files: Xl,
    scripts: Zl,
    repository: Yl,
    keywords: Ql,
    author: ep,
    license: tp,
    bugs: rp,
    homepage: np,
    devDependencies: ip,
    dependencies: ap,
  };
var jt = {},
  hr = { exports: {} };
(function (r) {
  (function (e, t) {
    function n(A, d) {
      if (!A) throw new Error(d || 'Assertion failed');
    }
    function i(A, d) {
      A.super_ = d;
      var u = function () {};
      (u.prototype = d.prototype), (A.prototype = new u()), (A.prototype.constructor = A);
    }
    function a(A, d, u) {
      if (a.isBN(A)) return A;
      (this.negative = 0),
        (this.words = null),
        (this.length = 0),
        (this.red = null),
        A !== null &&
          ((d === 'le' || d === 'be') && ((u = d), (d = 10)),
          this._init(A || 0, d || 10, u || 'be'));
    }
    typeof e == 'object' ? (e.exports = a) : (t.BN = a), (a.BN = a), (a.wordSize = 26);
    var c;
    try {
      typeof window < 'u' && typeof window.Buffer < 'u' ? (c = window.Buffer) : (c = Xa.Buffer);
    } catch {}
    (a.isBN = function (d) {
      return d instanceof a
        ? !0
        : d !== null &&
            typeof d == 'object' &&
            d.constructor.wordSize === a.wordSize &&
            Array.isArray(d.words);
    }),
      (a.max = function (d, u) {
        return d.cmp(u) > 0 ? d : u;
      }),
      (a.min = function (d, u) {
        return d.cmp(u) < 0 ? d : u;
      }),
      (a.prototype._init = function (d, u, s) {
        if (typeof d == 'number') return this._initNumber(d, u, s);
        if (typeof d == 'object') return this._initArray(d, u, s);
        u === 'hex' && (u = 16),
          n(u === (u | 0) && u >= 2 && u <= 36),
          (d = d.toString().replace(/\s+/g, ''));
        var o = 0;
        d[0] === '-' && (o++, (this.negative = 1)),
          o < d.length &&
            (u === 16
              ? this._parseHex(d, o, s)
              : (this._parseBase(d, u, o), s === 'le' && this._initArray(this.toArray(), u, s)));
      }),
      (a.prototype._initNumber = function (d, u, s) {
        d < 0 && ((this.negative = 1), (d = -d)),
          d < 67108864
            ? ((this.words = [d & 67108863]), (this.length = 1))
            : d < 4503599627370496
            ? ((this.words = [d & 67108863, (d / 67108864) & 67108863]), (this.length = 2))
            : (n(d < 9007199254740992),
              (this.words = [d & 67108863, (d / 67108864) & 67108863, 1]),
              (this.length = 3)),
          s === 'le' && this._initArray(this.toArray(), u, s);
      }),
      (a.prototype._initArray = function (d, u, s) {
        if ((n(typeof d.length == 'number'), d.length <= 0))
          return (this.words = [0]), (this.length = 1), this;
        (this.length = Math.ceil(d.length / 3)), (this.words = new Array(this.length));
        for (var o = 0; o < this.length; o++) this.words[o] = 0;
        var l,
          g,
          m = 0;
        if (s === 'be')
          for (o = d.length - 1, l = 0; o >= 0; o -= 3)
            (g = d[o] | (d[o - 1] << 8) | (d[o - 2] << 16)),
              (this.words[l] |= (g << m) & 67108863),
              (this.words[l + 1] = (g >>> (26 - m)) & 67108863),
              (m += 24),
              m >= 26 && ((m -= 26), l++);
        else if (s === 'le')
          for (o = 0, l = 0; o < d.length; o += 3)
            (g = d[o] | (d[o + 1] << 8) | (d[o + 2] << 16)),
              (this.words[l] |= (g << m) & 67108863),
              (this.words[l + 1] = (g >>> (26 - m)) & 67108863),
              (m += 24),
              m >= 26 && ((m -= 26), l++);
        return this.strip();
      });
    function v(A, d) {
      var u = A.charCodeAt(d);
      return u >= 65 && u <= 70 ? u - 55 : u >= 97 && u <= 102 ? u - 87 : (u - 48) & 15;
    }
    function w(A, d, u) {
      var s = v(A, u);
      return u - 1 >= d && (s |= v(A, u - 1) << 4), s;
    }
    a.prototype._parseHex = function (d, u, s) {
      (this.length = Math.ceil((d.length - u) / 6)), (this.words = new Array(this.length));
      for (var o = 0; o < this.length; o++) this.words[o] = 0;
      var l = 0,
        g = 0,
        m;
      if (s === 'be')
        for (o = d.length - 1; o >= u; o -= 2)
          (m = w(d, u, o) << l),
            (this.words[g] |= m & 67108863),
            l >= 18 ? ((l -= 18), (g += 1), (this.words[g] |= m >>> 26)) : (l += 8);
      else {
        var b = d.length - u;
        for (o = b % 2 === 0 ? u + 1 : u; o < d.length; o += 2)
          (m = w(d, u, o) << l),
            (this.words[g] |= m & 67108863),
            l >= 18 ? ((l -= 18), (g += 1), (this.words[g] |= m >>> 26)) : (l += 8);
      }
      this.strip();
    };
    function y(A, d, u, s) {
      for (var o = 0, l = Math.min(A.length, u), g = d; g < l; g++) {
        var m = A.charCodeAt(g) - 48;
        (o *= s), m >= 49 ? (o += m - 49 + 10) : m >= 17 ? (o += m - 17 + 10) : (o += m);
      }
      return o;
    }
    (a.prototype._parseBase = function (d, u, s) {
      (this.words = [0]), (this.length = 1);
      for (var o = 0, l = 1; l <= 67108863; l *= u) o++;
      o--, (l = (l / u) | 0);
      for (var g = d.length - s, m = g % o, b = Math.min(g, g - m) + s, f = 0, p = s; p < b; p += o)
        (f = y(d, p, p + o, u)),
          this.imuln(l),
          this.words[0] + f < 67108864 ? (this.words[0] += f) : this._iaddn(f);
      if (m !== 0) {
        var h = 1;
        for (f = y(d, p, d.length, u), p = 0; p < m; p++) h *= u;
        this.imuln(h), this.words[0] + f < 67108864 ? (this.words[0] += f) : this._iaddn(f);
      }
      this.strip();
    }),
      (a.prototype.copy = function (d) {
        d.words = new Array(this.length);
        for (var u = 0; u < this.length; u++) d.words[u] = this.words[u];
        (d.length = this.length), (d.negative = this.negative), (d.red = this.red);
      }),
      (a.prototype.clone = function () {
        var d = new a(null);
        return this.copy(d), d;
      }),
      (a.prototype._expand = function (d) {
        for (; this.length < d; ) this.words[this.length++] = 0;
        return this;
      }),
      (a.prototype.strip = function () {
        for (; this.length > 1 && this.words[this.length - 1] === 0; ) this.length--;
        return this._normSign();
      }),
      (a.prototype._normSign = function () {
        return this.length === 1 && this.words[0] === 0 && (this.negative = 0), this;
      }),
      (a.prototype.inspect = function () {
        return (this.red ? '<BN-R: ' : '<BN: ') + this.toString(16) + '>';
      });
    var M = [
        '',
        '0',
        '00',
        '000',
        '0000',
        '00000',
        '000000',
        '0000000',
        '00000000',
        '000000000',
        '0000000000',
        '00000000000',
        '000000000000',
        '0000000000000',
        '00000000000000',
        '000000000000000',
        '0000000000000000',
        '00000000000000000',
        '000000000000000000',
        '0000000000000000000',
        '00000000000000000000',
        '000000000000000000000',
        '0000000000000000000000',
        '00000000000000000000000',
        '000000000000000000000000',
        '0000000000000000000000000',
      ],
      S = [
        0, 0, 25, 16, 12, 11, 10, 9, 8, 8, 7, 7, 7, 7, 6, 6, 6, 6, 6, 6, 6, 5, 5, 5, 5, 5, 5, 5, 5,
        5, 5, 5, 5, 5, 5, 5, 5,
      ],
      I = [
        0, 0, 33554432, 43046721, 16777216, 48828125, 60466176, 40353607, 16777216, 43046721, 1e7,
        19487171, 35831808, 62748517, 7529536, 11390625, 16777216, 24137569, 34012224, 47045881,
        64e6, 4084101, 5153632, 6436343, 7962624, 9765625, 11881376, 14348907, 17210368, 20511149,
        243e5, 28629151, 33554432, 39135393, 45435424, 52521875, 60466176,
      ];
    (a.prototype.toString = function (d, u) {
      (d = d || 10), (u = u | 0 || 1);
      var s;
      if (d === 16 || d === 'hex') {
        s = '';
        for (var o = 0, l = 0, g = 0; g < this.length; g++) {
          var m = this.words[g],
            b = (((m << o) | l) & 16777215).toString(16);
          (l = (m >>> (24 - o)) & 16777215),
            l !== 0 || g !== this.length - 1 ? (s = M[6 - b.length] + b + s) : (s = b + s),
            (o += 2),
            o >= 26 && ((o -= 26), g--);
        }
        for (l !== 0 && (s = l.toString(16) + s); s.length % u !== 0; ) s = '0' + s;
        return this.negative !== 0 && (s = '-' + s), s;
      }
      if (d === (d | 0) && d >= 2 && d <= 36) {
        var f = S[d],
          p = I[d];
        s = '';
        var h = this.clone();
        for (h.negative = 0; !h.isZero(); ) {
          var x = h.modn(p).toString(d);
          (h = h.idivn(p)), h.isZero() ? (s = x + s) : (s = M[f - x.length] + x + s);
        }
        for (this.isZero() && (s = '0' + s); s.length % u !== 0; ) s = '0' + s;
        return this.negative !== 0 && (s = '-' + s), s;
      }
      n(!1, 'Base should be between 2 and 36');
    }),
      (a.prototype.toNumber = function () {
        var d = this.words[0];
        return (
          this.length === 2
            ? (d += this.words[1] * 67108864)
            : this.length === 3 && this.words[2] === 1
            ? (d += 4503599627370496 + this.words[1] * 67108864)
            : this.length > 2 && n(!1, 'Number can only safely store up to 53 bits'),
          this.negative !== 0 ? -d : d
        );
      }),
      (a.prototype.toJSON = function () {
        return this.toString(16);
      }),
      (a.prototype.toBuffer = function (d, u) {
        return n(typeof c < 'u'), this.toArrayLike(c, d, u);
      }),
      (a.prototype.toArray = function (d, u) {
        return this.toArrayLike(Array, d, u);
      }),
      (a.prototype.toArrayLike = function (d, u, s) {
        var o = this.byteLength(),
          l = s || Math.max(1, o);
        n(o <= l, 'byte array longer than desired length'),
          n(l > 0, 'Requested array length <= 0'),
          this.strip();
        var g = u === 'le',
          m = new d(l),
          b,
          f,
          p = this.clone();
        if (g) {
          for (f = 0; !p.isZero(); f++) (b = p.andln(255)), p.iushrn(8), (m[f] = b);
          for (; f < l; f++) m[f] = 0;
        } else {
          for (f = 0; f < l - o; f++) m[f] = 0;
          for (f = 0; !p.isZero(); f++) (b = p.andln(255)), p.iushrn(8), (m[l - f - 1] = b);
        }
        return m;
      }),
      Math.clz32
        ? (a.prototype._countBits = function (d) {
            return 32 - Math.clz32(d);
          })
        : (a.prototype._countBits = function (d) {
            var u = d,
              s = 0;
            return (
              u >= 4096 && ((s += 13), (u >>>= 13)),
              u >= 64 && ((s += 7), (u >>>= 7)),
              u >= 8 && ((s += 4), (u >>>= 4)),
              u >= 2 && ((s += 2), (u >>>= 2)),
              s + u
            );
          }),
      (a.prototype._zeroBits = function (d) {
        if (d === 0) return 26;
        var u = d,
          s = 0;
        return (
          u & 8191 || ((s += 13), (u >>>= 13)),
          u & 127 || ((s += 7), (u >>>= 7)),
          u & 15 || ((s += 4), (u >>>= 4)),
          u & 3 || ((s += 2), (u >>>= 2)),
          u & 1 || s++,
          s
        );
      }),
      (a.prototype.bitLength = function () {
        var d = this.words[this.length - 1],
          u = this._countBits(d);
        return (this.length - 1) * 26 + u;
      });
    function E(A) {
      for (var d = new Array(A.bitLength()), u = 0; u < d.length; u++) {
        var s = (u / 26) | 0,
          o = u % 26;
        d[u] = (A.words[s] & (1 << o)) >>> o;
      }
      return d;
    }
    (a.prototype.zeroBits = function () {
      if (this.isZero()) return 0;
      for (var d = 0, u = 0; u < this.length; u++) {
        var s = this._zeroBits(this.words[u]);
        if (((d += s), s !== 26)) break;
      }
      return d;
    }),
      (a.prototype.byteLength = function () {
        return Math.ceil(this.bitLength() / 8);
      }),
      (a.prototype.toTwos = function (d) {
        return this.negative !== 0 ? this.abs().inotn(d).iaddn(1) : this.clone();
      }),
      (a.prototype.fromTwos = function (d) {
        return this.testn(d - 1) ? this.notn(d).iaddn(1).ineg() : this.clone();
      }),
      (a.prototype.isNeg = function () {
        return this.negative !== 0;
      }),
      (a.prototype.neg = function () {
        return this.clone().ineg();
      }),
      (a.prototype.ineg = function () {
        return this.isZero() || (this.negative ^= 1), this;
      }),
      (a.prototype.iuor = function (d) {
        for (; this.length < d.length; ) this.words[this.length++] = 0;
        for (var u = 0; u < d.length; u++) this.words[u] = this.words[u] | d.words[u];
        return this.strip();
      }),
      (a.prototype.ior = function (d) {
        return n((this.negative | d.negative) === 0), this.iuor(d);
      }),
      (a.prototype.or = function (d) {
        return this.length > d.length ? this.clone().ior(d) : d.clone().ior(this);
      }),
      (a.prototype.uor = function (d) {
        return this.length > d.length ? this.clone().iuor(d) : d.clone().iuor(this);
      }),
      (a.prototype.iuand = function (d) {
        var u;
        this.length > d.length ? (u = d) : (u = this);
        for (var s = 0; s < u.length; s++) this.words[s] = this.words[s] & d.words[s];
        return (this.length = u.length), this.strip();
      }),
      (a.prototype.iand = function (d) {
        return n((this.negative | d.negative) === 0), this.iuand(d);
      }),
      (a.prototype.and = function (d) {
        return this.length > d.length ? this.clone().iand(d) : d.clone().iand(this);
      }),
      (a.prototype.uand = function (d) {
        return this.length > d.length ? this.clone().iuand(d) : d.clone().iuand(this);
      }),
      (a.prototype.iuxor = function (d) {
        var u, s;
        this.length > d.length ? ((u = this), (s = d)) : ((u = d), (s = this));
        for (var o = 0; o < s.length; o++) this.words[o] = u.words[o] ^ s.words[o];
        if (this !== u) for (; o < u.length; o++) this.words[o] = u.words[o];
        return (this.length = u.length), this.strip();
      }),
      (a.prototype.ixor = function (d) {
        return n((this.negative | d.negative) === 0), this.iuxor(d);
      }),
      (a.prototype.xor = function (d) {
        return this.length > d.length ? this.clone().ixor(d) : d.clone().ixor(this);
      }),
      (a.prototype.uxor = function (d) {
        return this.length > d.length ? this.clone().iuxor(d) : d.clone().iuxor(this);
      }),
      (a.prototype.inotn = function (d) {
        n(typeof d == 'number' && d >= 0);
        var u = Math.ceil(d / 26) | 0,
          s = d % 26;
        this._expand(u), s > 0 && u--;
        for (var o = 0; o < u; o++) this.words[o] = ~this.words[o] & 67108863;
        return s > 0 && (this.words[o] = ~this.words[o] & (67108863 >> (26 - s))), this.strip();
      }),
      (a.prototype.notn = function (d) {
        return this.clone().inotn(d);
      }),
      (a.prototype.setn = function (d, u) {
        n(typeof d == 'number' && d >= 0);
        var s = (d / 26) | 0,
          o = d % 26;
        return (
          this._expand(s + 1),
          u
            ? (this.words[s] = this.words[s] | (1 << o))
            : (this.words[s] = this.words[s] & ~(1 << o)),
          this.strip()
        );
      }),
      (a.prototype.iadd = function (d) {
        var u;
        if (this.negative !== 0 && d.negative === 0)
          return (this.negative = 0), (u = this.isub(d)), (this.negative ^= 1), this._normSign();
        if (this.negative === 0 && d.negative !== 0)
          return (d.negative = 0), (u = this.isub(d)), (d.negative = 1), u._normSign();
        var s, o;
        this.length > d.length ? ((s = this), (o = d)) : ((s = d), (o = this));
        for (var l = 0, g = 0; g < o.length; g++)
          (u = (s.words[g] | 0) + (o.words[g] | 0) + l),
            (this.words[g] = u & 67108863),
            (l = u >>> 26);
        for (; l !== 0 && g < s.length; g++)
          (u = (s.words[g] | 0) + l), (this.words[g] = u & 67108863), (l = u >>> 26);
        if (((this.length = s.length), l !== 0)) (this.words[this.length] = l), this.length++;
        else if (s !== this) for (; g < s.length; g++) this.words[g] = s.words[g];
        return this;
      }),
      (a.prototype.add = function (d) {
        var u;
        return d.negative !== 0 && this.negative === 0
          ? ((d.negative = 0), (u = this.sub(d)), (d.negative ^= 1), u)
          : d.negative === 0 && this.negative !== 0
          ? ((this.negative = 0), (u = d.sub(this)), (this.negative = 1), u)
          : this.length > d.length
          ? this.clone().iadd(d)
          : d.clone().iadd(this);
      }),
      (a.prototype.isub = function (d) {
        if (d.negative !== 0) {
          d.negative = 0;
          var u = this.iadd(d);
          return (d.negative = 1), u._normSign();
        } else if (this.negative !== 0)
          return (this.negative = 0), this.iadd(d), (this.negative = 1), this._normSign();
        var s = this.cmp(d);
        if (s === 0) return (this.negative = 0), (this.length = 1), (this.words[0] = 0), this;
        var o, l;
        s > 0 ? ((o = this), (l = d)) : ((o = d), (l = this));
        for (var g = 0, m = 0; m < l.length; m++)
          (u = (o.words[m] | 0) - (l.words[m] | 0) + g),
            (g = u >> 26),
            (this.words[m] = u & 67108863);
        for (; g !== 0 && m < o.length; m++)
          (u = (o.words[m] | 0) + g), (g = u >> 26), (this.words[m] = u & 67108863);
        if (g === 0 && m < o.length && o !== this)
          for (; m < o.length; m++) this.words[m] = o.words[m];
        return (
          (this.length = Math.max(this.length, m)), o !== this && (this.negative = 1), this.strip()
        );
      }),
      (a.prototype.sub = function (d) {
        return this.clone().isub(d);
      });
    function R(A, d, u) {
      u.negative = d.negative ^ A.negative;
      var s = (A.length + d.length) | 0;
      (u.length = s), (s = (s - 1) | 0);
      var o = A.words[0] | 0,
        l = d.words[0] | 0,
        g = o * l,
        m = g & 67108863,
        b = (g / 67108864) | 0;
      u.words[0] = m;
      for (var f = 1; f < s; f++) {
        for (
          var p = b >>> 26,
            h = b & 67108863,
            x = Math.min(f, d.length - 1),
            O = Math.max(0, f - A.length + 1);
          O <= x;
          O++
        ) {
          var C = (f - O) | 0;
          (o = A.words[C] | 0),
            (l = d.words[O] | 0),
            (g = o * l + h),
            (p += (g / 67108864) | 0),
            (h = g & 67108863);
        }
        (u.words[f] = h | 0), (b = p | 0);
      }
      return b !== 0 ? (u.words[f] = b | 0) : u.length--, u.strip();
    }
    var T = function (d, u, s) {
      var o = d.words,
        l = u.words,
        g = s.words,
        m = 0,
        b,
        f,
        p,
        h = o[0] | 0,
        x = h & 8191,
        O = h >>> 13,
        C = o[1] | 0,
        L = C & 8191,
        D = C >>> 13,
        Z = o[2] | 0,
        j = Z & 8191,
        le = Z >>> 13,
        Ae = o[3] | 0,
        Q = Ae & 8191,
        Re = Ae >>> 13,
        Ee = o[4] | 0,
        ne = Ee & 8191,
        ke = Ee >>> 13,
        De = o[5] | 0,
        ae = De & 8191,
        Fe = De >>> 13,
        $e = o[6] | 0,
        ce = $e & 8191,
        Ue = $e >>> 13,
        Ge = o[7] | 0,
        oe = Ge & 8191,
        ze = Ge >>> 13,
        rt = o[8] | 0,
        pe = rt & 8191,
        Ke = rt >>> 13,
        Xe = o[9] | 0,
        be = Xe & 8191,
        nt = Xe >>> 13,
        it = l[0] | 0,
        me = it & 8191,
        Ze = it >>> 13,
        at = l[1] | 0,
        ge = at & 8191,
        Je = at >>> 13,
        Be = l[2] | 0,
        he = Be & 8191,
        He = Be >>> 13,
        We = l[3] | 0,
        ue = We & 8191,
        st = We >>> 13,
        ot = l[4] | 0,
        ye = ot & 8191,
        ft = ot >>> 13,
        ct = l[5] | 0,
        ve = ct & 8191,
        Ye = ct >>> 13,
        Le = l[6] | 0,
        we = Le & 8191,
        ut = Le >>> 13,
        dt = l[7] | 0,
        xe = dt & 8191,
        ht = dt >>> 13,
        Ce = l[8] | 0,
        Me = Ce & 8191,
        lt = Ce >>> 13,
        pt = l[9] | 0,
        _e = pt & 8191,
        vt = pt >>> 13;
      (s.negative = d.negative ^ u.negative),
        (s.length = 19),
        (b = Math.imul(x, me)),
        (f = Math.imul(x, Ze)),
        (f = (f + Math.imul(O, me)) | 0),
        (p = Math.imul(O, Ze));
      var qe = (((m + b) | 0) + ((f & 8191) << 13)) | 0;
      (m = (((p + (f >>> 13)) | 0) + (qe >>> 26)) | 0),
        (qe &= 67108863),
        (b = Math.imul(L, me)),
        (f = Math.imul(L, Ze)),
        (f = (f + Math.imul(D, me)) | 0),
        (p = Math.imul(D, Ze)),
        (b = (b + Math.imul(x, ge)) | 0),
        (f = (f + Math.imul(x, Je)) | 0),
        (f = (f + Math.imul(O, ge)) | 0),
        (p = (p + Math.imul(O, Je)) | 0);
      var Qe = (((m + b) | 0) + ((f & 8191) << 13)) | 0;
      (m = (((p + (f >>> 13)) | 0) + (Qe >>> 26)) | 0),
        (Qe &= 67108863),
        (b = Math.imul(j, me)),
        (f = Math.imul(j, Ze)),
        (f = (f + Math.imul(le, me)) | 0),
        (p = Math.imul(le, Ze)),
        (b = (b + Math.imul(L, ge)) | 0),
        (f = (f + Math.imul(L, Je)) | 0),
        (f = (f + Math.imul(D, ge)) | 0),
        (p = (p + Math.imul(D, Je)) | 0),
        (b = (b + Math.imul(x, he)) | 0),
        (f = (f + Math.imul(x, He)) | 0),
        (f = (f + Math.imul(O, he)) | 0),
        (p = (p + Math.imul(O, He)) | 0);
      var pr = (((m + b) | 0) + ((f & 8191) << 13)) | 0;
      (m = (((p + (f >>> 13)) | 0) + (pr >>> 26)) | 0),
        (pr &= 67108863),
        (b = Math.imul(Q, me)),
        (f = Math.imul(Q, Ze)),
        (f = (f + Math.imul(Re, me)) | 0),
        (p = Math.imul(Re, Ze)),
        (b = (b + Math.imul(j, ge)) | 0),
        (f = (f + Math.imul(j, Je)) | 0),
        (f = (f + Math.imul(le, ge)) | 0),
        (p = (p + Math.imul(le, Je)) | 0),
        (b = (b + Math.imul(L, he)) | 0),
        (f = (f + Math.imul(L, He)) | 0),
        (f = (f + Math.imul(D, he)) | 0),
        (p = (p + Math.imul(D, He)) | 0),
        (b = (b + Math.imul(x, ue)) | 0),
        (f = (f + Math.imul(x, st)) | 0),
        (f = (f + Math.imul(O, ue)) | 0),
        (p = (p + Math.imul(O, st)) | 0);
      var vr = (((m + b) | 0) + ((f & 8191) << 13)) | 0;
      (m = (((p + (f >>> 13)) | 0) + (vr >>> 26)) | 0),
        (vr &= 67108863),
        (b = Math.imul(ne, me)),
        (f = Math.imul(ne, Ze)),
        (f = (f + Math.imul(ke, me)) | 0),
        (p = Math.imul(ke, Ze)),
        (b = (b + Math.imul(Q, ge)) | 0),
        (f = (f + Math.imul(Q, Je)) | 0),
        (f = (f + Math.imul(Re, ge)) | 0),
        (p = (p + Math.imul(Re, Je)) | 0),
        (b = (b + Math.imul(j, he)) | 0),
        (f = (f + Math.imul(j, He)) | 0),
        (f = (f + Math.imul(le, he)) | 0),
        (p = (p + Math.imul(le, He)) | 0),
        (b = (b + Math.imul(L, ue)) | 0),
        (f = (f + Math.imul(L, st)) | 0),
        (f = (f + Math.imul(D, ue)) | 0),
        (p = (p + Math.imul(D, st)) | 0),
        (b = (b + Math.imul(x, ye)) | 0),
        (f = (f + Math.imul(x, ft)) | 0),
        (f = (f + Math.imul(O, ye)) | 0),
        (p = (p + Math.imul(O, ft)) | 0);
      var br = (((m + b) | 0) + ((f & 8191) << 13)) | 0;
      (m = (((p + (f >>> 13)) | 0) + (br >>> 26)) | 0),
        (br &= 67108863),
        (b = Math.imul(ae, me)),
        (f = Math.imul(ae, Ze)),
        (f = (f + Math.imul(Fe, me)) | 0),
        (p = Math.imul(Fe, Ze)),
        (b = (b + Math.imul(ne, ge)) | 0),
        (f = (f + Math.imul(ne, Je)) | 0),
        (f = (f + Math.imul(ke, ge)) | 0),
        (p = (p + Math.imul(ke, Je)) | 0),
        (b = (b + Math.imul(Q, he)) | 0),
        (f = (f + Math.imul(Q, He)) | 0),
        (f = (f + Math.imul(Re, he)) | 0),
        (p = (p + Math.imul(Re, He)) | 0),
        (b = (b + Math.imul(j, ue)) | 0),
        (f = (f + Math.imul(j, st)) | 0),
        (f = (f + Math.imul(le, ue)) | 0),
        (p = (p + Math.imul(le, st)) | 0),
        (b = (b + Math.imul(L, ye)) | 0),
        (f = (f + Math.imul(L, ft)) | 0),
        (f = (f + Math.imul(D, ye)) | 0),
        (p = (p + Math.imul(D, ft)) | 0),
        (b = (b + Math.imul(x, ve)) | 0),
        (f = (f + Math.imul(x, Ye)) | 0),
        (f = (f + Math.imul(O, ve)) | 0),
        (p = (p + Math.imul(O, Ye)) | 0);
      var mr = (((m + b) | 0) + ((f & 8191) << 13)) | 0;
      (m = (((p + (f >>> 13)) | 0) + (mr >>> 26)) | 0),
        (mr &= 67108863),
        (b = Math.imul(ce, me)),
        (f = Math.imul(ce, Ze)),
        (f = (f + Math.imul(Ue, me)) | 0),
        (p = Math.imul(Ue, Ze)),
        (b = (b + Math.imul(ae, ge)) | 0),
        (f = (f + Math.imul(ae, Je)) | 0),
        (f = (f + Math.imul(Fe, ge)) | 0),
        (p = (p + Math.imul(Fe, Je)) | 0),
        (b = (b + Math.imul(ne, he)) | 0),
        (f = (f + Math.imul(ne, He)) | 0),
        (f = (f + Math.imul(ke, he)) | 0),
        (p = (p + Math.imul(ke, He)) | 0),
        (b = (b + Math.imul(Q, ue)) | 0),
        (f = (f + Math.imul(Q, st)) | 0),
        (f = (f + Math.imul(Re, ue)) | 0),
        (p = (p + Math.imul(Re, st)) | 0),
        (b = (b + Math.imul(j, ye)) | 0),
        (f = (f + Math.imul(j, ft)) | 0),
        (f = (f + Math.imul(le, ye)) | 0),
        (p = (p + Math.imul(le, ft)) | 0),
        (b = (b + Math.imul(L, ve)) | 0),
        (f = (f + Math.imul(L, Ye)) | 0),
        (f = (f + Math.imul(D, ve)) | 0),
        (p = (p + Math.imul(D, Ye)) | 0),
        (b = (b + Math.imul(x, we)) | 0),
        (f = (f + Math.imul(x, ut)) | 0),
        (f = (f + Math.imul(O, we)) | 0),
        (p = (p + Math.imul(O, ut)) | 0);
      var kt = (((m + b) | 0) + ((f & 8191) << 13)) | 0;
      (m = (((p + (f >>> 13)) | 0) + (kt >>> 26)) | 0),
        (kt &= 67108863),
        (b = Math.imul(oe, me)),
        (f = Math.imul(oe, Ze)),
        (f = (f + Math.imul(ze, me)) | 0),
        (p = Math.imul(ze, Ze)),
        (b = (b + Math.imul(ce, ge)) | 0),
        (f = (f + Math.imul(ce, Je)) | 0),
        (f = (f + Math.imul(Ue, ge)) | 0),
        (p = (p + Math.imul(Ue, Je)) | 0),
        (b = (b + Math.imul(ae, he)) | 0),
        (f = (f + Math.imul(ae, He)) | 0),
        (f = (f + Math.imul(Fe, he)) | 0),
        (p = (p + Math.imul(Fe, He)) | 0),
        (b = (b + Math.imul(ne, ue)) | 0),
        (f = (f + Math.imul(ne, st)) | 0),
        (f = (f + Math.imul(ke, ue)) | 0),
        (p = (p + Math.imul(ke, st)) | 0),
        (b = (b + Math.imul(Q, ye)) | 0),
        (f = (f + Math.imul(Q, ft)) | 0),
        (f = (f + Math.imul(Re, ye)) | 0),
        (p = (p + Math.imul(Re, ft)) | 0),
        (b = (b + Math.imul(j, ve)) | 0),
        (f = (f + Math.imul(j, Ye)) | 0),
        (f = (f + Math.imul(le, ve)) | 0),
        (p = (p + Math.imul(le, Ye)) | 0),
        (b = (b + Math.imul(L, we)) | 0),
        (f = (f + Math.imul(L, ut)) | 0),
        (f = (f + Math.imul(D, we)) | 0),
        (p = (p + Math.imul(D, ut)) | 0),
        (b = (b + Math.imul(x, xe)) | 0),
        (f = (f + Math.imul(x, ht)) | 0),
        (f = (f + Math.imul(O, xe)) | 0),
        (p = (p + Math.imul(O, ht)) | 0);
      var gr = (((m + b) | 0) + ((f & 8191) << 13)) | 0;
      (m = (((p + (f >>> 13)) | 0) + (gr >>> 26)) | 0),
        (gr &= 67108863),
        (b = Math.imul(pe, me)),
        (f = Math.imul(pe, Ze)),
        (f = (f + Math.imul(Ke, me)) | 0),
        (p = Math.imul(Ke, Ze)),
        (b = (b + Math.imul(oe, ge)) | 0),
        (f = (f + Math.imul(oe, Je)) | 0),
        (f = (f + Math.imul(ze, ge)) | 0),
        (p = (p + Math.imul(ze, Je)) | 0),
        (b = (b + Math.imul(ce, he)) | 0),
        (f = (f + Math.imul(ce, He)) | 0),
        (f = (f + Math.imul(Ue, he)) | 0),
        (p = (p + Math.imul(Ue, He)) | 0),
        (b = (b + Math.imul(ae, ue)) | 0),
        (f = (f + Math.imul(ae, st)) | 0),
        (f = (f + Math.imul(Fe, ue)) | 0),
        (p = (p + Math.imul(Fe, st)) | 0),
        (b = (b + Math.imul(ne, ye)) | 0),
        (f = (f + Math.imul(ne, ft)) | 0),
        (f = (f + Math.imul(ke, ye)) | 0),
        (p = (p + Math.imul(ke, ft)) | 0),
        (b = (b + Math.imul(Q, ve)) | 0),
        (f = (f + Math.imul(Q, Ye)) | 0),
        (f = (f + Math.imul(Re, ve)) | 0),
        (p = (p + Math.imul(Re, Ye)) | 0),
        (b = (b + Math.imul(j, we)) | 0),
        (f = (f + Math.imul(j, ut)) | 0),
        (f = (f + Math.imul(le, we)) | 0),
        (p = (p + Math.imul(le, ut)) | 0),
        (b = (b + Math.imul(L, xe)) | 0),
        (f = (f + Math.imul(L, ht)) | 0),
        (f = (f + Math.imul(D, xe)) | 0),
        (p = (p + Math.imul(D, ht)) | 0),
        (b = (b + Math.imul(x, Me)) | 0),
        (f = (f + Math.imul(x, lt)) | 0),
        (f = (f + Math.imul(O, Me)) | 0),
        (p = (p + Math.imul(O, lt)) | 0);
      var yr = (((m + b) | 0) + ((f & 8191) << 13)) | 0;
      (m = (((p + (f >>> 13)) | 0) + (yr >>> 26)) | 0),
        (yr &= 67108863),
        (b = Math.imul(be, me)),
        (f = Math.imul(be, Ze)),
        (f = (f + Math.imul(nt, me)) | 0),
        (p = Math.imul(nt, Ze)),
        (b = (b + Math.imul(pe, ge)) | 0),
        (f = (f + Math.imul(pe, Je)) | 0),
        (f = (f + Math.imul(Ke, ge)) | 0),
        (p = (p + Math.imul(Ke, Je)) | 0),
        (b = (b + Math.imul(oe, he)) | 0),
        (f = (f + Math.imul(oe, He)) | 0),
        (f = (f + Math.imul(ze, he)) | 0),
        (p = (p + Math.imul(ze, He)) | 0),
        (b = (b + Math.imul(ce, ue)) | 0),
        (f = (f + Math.imul(ce, st)) | 0),
        (f = (f + Math.imul(Ue, ue)) | 0),
        (p = (p + Math.imul(Ue, st)) | 0),
        (b = (b + Math.imul(ae, ye)) | 0),
        (f = (f + Math.imul(ae, ft)) | 0),
        (f = (f + Math.imul(Fe, ye)) | 0),
        (p = (p + Math.imul(Fe, ft)) | 0),
        (b = (b + Math.imul(ne, ve)) | 0),
        (f = (f + Math.imul(ne, Ye)) | 0),
        (f = (f + Math.imul(ke, ve)) | 0),
        (p = (p + Math.imul(ke, Ye)) | 0),
        (b = (b + Math.imul(Q, we)) | 0),
        (f = (f + Math.imul(Q, ut)) | 0),
        (f = (f + Math.imul(Re, we)) | 0),
        (p = (p + Math.imul(Re, ut)) | 0),
        (b = (b + Math.imul(j, xe)) | 0),
        (f = (f + Math.imul(j, ht)) | 0),
        (f = (f + Math.imul(le, xe)) | 0),
        (p = (p + Math.imul(le, ht)) | 0),
        (b = (b + Math.imul(L, Me)) | 0),
        (f = (f + Math.imul(L, lt)) | 0),
        (f = (f + Math.imul(D, Me)) | 0),
        (p = (p + Math.imul(D, lt)) | 0),
        (b = (b + Math.imul(x, _e)) | 0),
        (f = (f + Math.imul(x, vt)) | 0),
        (f = (f + Math.imul(O, _e)) | 0),
        (p = (p + Math.imul(O, vt)) | 0);
      var wr = (((m + b) | 0) + ((f & 8191) << 13)) | 0;
      (m = (((p + (f >>> 13)) | 0) + (wr >>> 26)) | 0),
        (wr &= 67108863),
        (b = Math.imul(be, ge)),
        (f = Math.imul(be, Je)),
        (f = (f + Math.imul(nt, ge)) | 0),
        (p = Math.imul(nt, Je)),
        (b = (b + Math.imul(pe, he)) | 0),
        (f = (f + Math.imul(pe, He)) | 0),
        (f = (f + Math.imul(Ke, he)) | 0),
        (p = (p + Math.imul(Ke, He)) | 0),
        (b = (b + Math.imul(oe, ue)) | 0),
        (f = (f + Math.imul(oe, st)) | 0),
        (f = (f + Math.imul(ze, ue)) | 0),
        (p = (p + Math.imul(ze, st)) | 0),
        (b = (b + Math.imul(ce, ye)) | 0),
        (f = (f + Math.imul(ce, ft)) | 0),
        (f = (f + Math.imul(Ue, ye)) | 0),
        (p = (p + Math.imul(Ue, ft)) | 0),
        (b = (b + Math.imul(ae, ve)) | 0),
        (f = (f + Math.imul(ae, Ye)) | 0),
        (f = (f + Math.imul(Fe, ve)) | 0),
        (p = (p + Math.imul(Fe, Ye)) | 0),
        (b = (b + Math.imul(ne, we)) | 0),
        (f = (f + Math.imul(ne, ut)) | 0),
        (f = (f + Math.imul(ke, we)) | 0),
        (p = (p + Math.imul(ke, ut)) | 0),
        (b = (b + Math.imul(Q, xe)) | 0),
        (f = (f + Math.imul(Q, ht)) | 0),
        (f = (f + Math.imul(Re, xe)) | 0),
        (p = (p + Math.imul(Re, ht)) | 0),
        (b = (b + Math.imul(j, Me)) | 0),
        (f = (f + Math.imul(j, lt)) | 0),
        (f = (f + Math.imul(le, Me)) | 0),
        (p = (p + Math.imul(le, lt)) | 0),
        (b = (b + Math.imul(L, _e)) | 0),
        (f = (f + Math.imul(L, vt)) | 0),
        (f = (f + Math.imul(D, _e)) | 0),
        (p = (p + Math.imul(D, vt)) | 0);
      var xr = (((m + b) | 0) + ((f & 8191) << 13)) | 0;
      (m = (((p + (f >>> 13)) | 0) + (xr >>> 26)) | 0),
        (xr &= 67108863),
        (b = Math.imul(be, he)),
        (f = Math.imul(be, He)),
        (f = (f + Math.imul(nt, he)) | 0),
        (p = Math.imul(nt, He)),
        (b = (b + Math.imul(pe, ue)) | 0),
        (f = (f + Math.imul(pe, st)) | 0),
        (f = (f + Math.imul(Ke, ue)) | 0),
        (p = (p + Math.imul(Ke, st)) | 0),
        (b = (b + Math.imul(oe, ye)) | 0),
        (f = (f + Math.imul(oe, ft)) | 0),
        (f = (f + Math.imul(ze, ye)) | 0),
        (p = (p + Math.imul(ze, ft)) | 0),
        (b = (b + Math.imul(ce, ve)) | 0),
        (f = (f + Math.imul(ce, Ye)) | 0),
        (f = (f + Math.imul(Ue, ve)) | 0),
        (p = (p + Math.imul(Ue, Ye)) | 0),
        (b = (b + Math.imul(ae, we)) | 0),
        (f = (f + Math.imul(ae, ut)) | 0),
        (f = (f + Math.imul(Fe, we)) | 0),
        (p = (p + Math.imul(Fe, ut)) | 0),
        (b = (b + Math.imul(ne, xe)) | 0),
        (f = (f + Math.imul(ne, ht)) | 0),
        (f = (f + Math.imul(ke, xe)) | 0),
        (p = (p + Math.imul(ke, ht)) | 0),
        (b = (b + Math.imul(Q, Me)) | 0),
        (f = (f + Math.imul(Q, lt)) | 0),
        (f = (f + Math.imul(Re, Me)) | 0),
        (p = (p + Math.imul(Re, lt)) | 0),
        (b = (b + Math.imul(j, _e)) | 0),
        (f = (f + Math.imul(j, vt)) | 0),
        (f = (f + Math.imul(le, _e)) | 0),
        (p = (p + Math.imul(le, vt)) | 0);
      var Mr = (((m + b) | 0) + ((f & 8191) << 13)) | 0;
      (m = (((p + (f >>> 13)) | 0) + (Mr >>> 26)) | 0),
        (Mr &= 67108863),
        (b = Math.imul(be, ue)),
        (f = Math.imul(be, st)),
        (f = (f + Math.imul(nt, ue)) | 0),
        (p = Math.imul(nt, st)),
        (b = (b + Math.imul(pe, ye)) | 0),
        (f = (f + Math.imul(pe, ft)) | 0),
        (f = (f + Math.imul(Ke, ye)) | 0),
        (p = (p + Math.imul(Ke, ft)) | 0),
        (b = (b + Math.imul(oe, ve)) | 0),
        (f = (f + Math.imul(oe, Ye)) | 0),
        (f = (f + Math.imul(ze, ve)) | 0),
        (p = (p + Math.imul(ze, Ye)) | 0),
        (b = (b + Math.imul(ce, we)) | 0),
        (f = (f + Math.imul(ce, ut)) | 0),
        (f = (f + Math.imul(Ue, we)) | 0),
        (p = (p + Math.imul(Ue, ut)) | 0),
        (b = (b + Math.imul(ae, xe)) | 0),
        (f = (f + Math.imul(ae, ht)) | 0),
        (f = (f + Math.imul(Fe, xe)) | 0),
        (p = (p + Math.imul(Fe, ht)) | 0),
        (b = (b + Math.imul(ne, Me)) | 0),
        (f = (f + Math.imul(ne, lt)) | 0),
        (f = (f + Math.imul(ke, Me)) | 0),
        (p = (p + Math.imul(ke, lt)) | 0),
        (b = (b + Math.imul(Q, _e)) | 0),
        (f = (f + Math.imul(Q, vt)) | 0),
        (f = (f + Math.imul(Re, _e)) | 0),
        (p = (p + Math.imul(Re, vt)) | 0);
      var _r = (((m + b) | 0) + ((f & 8191) << 13)) | 0;
      (m = (((p + (f >>> 13)) | 0) + (_r >>> 26)) | 0),
        (_r &= 67108863),
        (b = Math.imul(be, ye)),
        (f = Math.imul(be, ft)),
        (f = (f + Math.imul(nt, ye)) | 0),
        (p = Math.imul(nt, ft)),
        (b = (b + Math.imul(pe, ve)) | 0),
        (f = (f + Math.imul(pe, Ye)) | 0),
        (f = (f + Math.imul(Ke, ve)) | 0),
        (p = (p + Math.imul(Ke, Ye)) | 0),
        (b = (b + Math.imul(oe, we)) | 0),
        (f = (f + Math.imul(oe, ut)) | 0),
        (f = (f + Math.imul(ze, we)) | 0),
        (p = (p + Math.imul(ze, ut)) | 0),
        (b = (b + Math.imul(ce, xe)) | 0),
        (f = (f + Math.imul(ce, ht)) | 0),
        (f = (f + Math.imul(Ue, xe)) | 0),
        (p = (p + Math.imul(Ue, ht)) | 0),
        (b = (b + Math.imul(ae, Me)) | 0),
        (f = (f + Math.imul(ae, lt)) | 0),
        (f = (f + Math.imul(Fe, Me)) | 0),
        (p = (p + Math.imul(Fe, lt)) | 0),
        (b = (b + Math.imul(ne, _e)) | 0),
        (f = (f + Math.imul(ne, vt)) | 0),
        (f = (f + Math.imul(ke, _e)) | 0),
        (p = (p + Math.imul(ke, vt)) | 0);
      var rr = (((m + b) | 0) + ((f & 8191) << 13)) | 0;
      (m = (((p + (f >>> 13)) | 0) + (rr >>> 26)) | 0),
        (rr &= 67108863),
        (b = Math.imul(be, ve)),
        (f = Math.imul(be, Ye)),
        (f = (f + Math.imul(nt, ve)) | 0),
        (p = Math.imul(nt, Ye)),
        (b = (b + Math.imul(pe, we)) | 0),
        (f = (f + Math.imul(pe, ut)) | 0),
        (f = (f + Math.imul(Ke, we)) | 0),
        (p = (p + Math.imul(Ke, ut)) | 0),
        (b = (b + Math.imul(oe, xe)) | 0),
        (f = (f + Math.imul(oe, ht)) | 0),
        (f = (f + Math.imul(ze, xe)) | 0),
        (p = (p + Math.imul(ze, ht)) | 0),
        (b = (b + Math.imul(ce, Me)) | 0),
        (f = (f + Math.imul(ce, lt)) | 0),
        (f = (f + Math.imul(Ue, Me)) | 0),
        (p = (p + Math.imul(Ue, lt)) | 0),
        (b = (b + Math.imul(ae, _e)) | 0),
        (f = (f + Math.imul(ae, vt)) | 0),
        (f = (f + Math.imul(Fe, _e)) | 0),
        (p = (p + Math.imul(Fe, vt)) | 0);
      var Vt = (((m + b) | 0) + ((f & 8191) << 13)) | 0;
      (m = (((p + (f >>> 13)) | 0) + (Vt >>> 26)) | 0),
        (Vt &= 67108863),
        (b = Math.imul(be, we)),
        (f = Math.imul(be, ut)),
        (f = (f + Math.imul(nt, we)) | 0),
        (p = Math.imul(nt, ut)),
        (b = (b + Math.imul(pe, xe)) | 0),
        (f = (f + Math.imul(pe, ht)) | 0),
        (f = (f + Math.imul(Ke, xe)) | 0),
        (p = (p + Math.imul(Ke, ht)) | 0),
        (b = (b + Math.imul(oe, Me)) | 0),
        (f = (f + Math.imul(oe, lt)) | 0),
        (f = (f + Math.imul(ze, Me)) | 0),
        (p = (p + Math.imul(ze, lt)) | 0),
        (b = (b + Math.imul(ce, _e)) | 0),
        (f = (f + Math.imul(ce, vt)) | 0),
        (f = (f + Math.imul(Ue, _e)) | 0),
        (p = (p + Math.imul(Ue, vt)) | 0);
      var Sr = (((m + b) | 0) + ((f & 8191) << 13)) | 0;
      (m = (((p + (f >>> 13)) | 0) + (Sr >>> 26)) | 0),
        (Sr &= 67108863),
        (b = Math.imul(be, xe)),
        (f = Math.imul(be, ht)),
        (f = (f + Math.imul(nt, xe)) | 0),
        (p = Math.imul(nt, ht)),
        (b = (b + Math.imul(pe, Me)) | 0),
        (f = (f + Math.imul(pe, lt)) | 0),
        (f = (f + Math.imul(Ke, Me)) | 0),
        (p = (p + Math.imul(Ke, lt)) | 0),
        (b = (b + Math.imul(oe, _e)) | 0),
        (f = (f + Math.imul(oe, vt)) | 0),
        (f = (f + Math.imul(ze, _e)) | 0),
        (p = (p + Math.imul(ze, vt)) | 0);
      var Xt = (((m + b) | 0) + ((f & 8191) << 13)) | 0;
      (m = (((p + (f >>> 13)) | 0) + (Xt >>> 26)) | 0),
        (Xt &= 67108863),
        (b = Math.imul(be, Me)),
        (f = Math.imul(be, lt)),
        (f = (f + Math.imul(nt, Me)) | 0),
        (p = Math.imul(nt, lt)),
        (b = (b + Math.imul(pe, _e)) | 0),
        (f = (f + Math.imul(pe, vt)) | 0),
        (f = (f + Math.imul(Ke, _e)) | 0),
        (p = (p + Math.imul(Ke, vt)) | 0);
      var Ar = (((m + b) | 0) + ((f & 8191) << 13)) | 0;
      (m = (((p + (f >>> 13)) | 0) + (Ar >>> 26)) | 0),
        (Ar &= 67108863),
        (b = Math.imul(be, _e)),
        (f = Math.imul(be, vt)),
        (f = (f + Math.imul(nt, _e)) | 0),
        (p = Math.imul(nt, vt));
      var Er = (((m + b) | 0) + ((f & 8191) << 13)) | 0;
      return (
        (m = (((p + (f >>> 13)) | 0) + (Er >>> 26)) | 0),
        (Er &= 67108863),
        (g[0] = qe),
        (g[1] = Qe),
        (g[2] = pr),
        (g[3] = vr),
        (g[4] = br),
        (g[5] = mr),
        (g[6] = kt),
        (g[7] = gr),
        (g[8] = yr),
        (g[9] = wr),
        (g[10] = xr),
        (g[11] = Mr),
        (g[12] = _r),
        (g[13] = rr),
        (g[14] = Vt),
        (g[15] = Sr),
        (g[16] = Xt),
        (g[17] = Ar),
        (g[18] = Er),
        m !== 0 && ((g[19] = m), s.length++),
        s
      );
    };
    Math.imul || (T = R);
    function z(A, d, u) {
      (u.negative = d.negative ^ A.negative), (u.length = A.length + d.length);
      for (var s = 0, o = 0, l = 0; l < u.length - 1; l++) {
        var g = o;
        o = 0;
        for (
          var m = s & 67108863, b = Math.min(l, d.length - 1), f = Math.max(0, l - A.length + 1);
          f <= b;
          f++
        ) {
          var p = l - f,
            h = A.words[p] | 0,
            x = d.words[f] | 0,
            O = h * x,
            C = O & 67108863;
          (g = (g + ((O / 67108864) | 0)) | 0),
            (C = (C + m) | 0),
            (m = C & 67108863),
            (g = (g + (C >>> 26)) | 0),
            (o += g >>> 26),
            (g &= 67108863);
        }
        (u.words[l] = m), (s = g), (g = o);
      }
      return s !== 0 ? (u.words[l] = s) : u.length--, u.strip();
    }
    function q(A, d, u) {
      var s = new Y();
      return s.mulp(A, d, u);
    }
    a.prototype.mulTo = function (d, u) {
      var s,
        o = this.length + d.length;
      return (
        this.length === 10 && d.length === 10
          ? (s = T(this, d, u))
          : o < 63
          ? (s = R(this, d, u))
          : o < 1024
          ? (s = z(this, d, u))
          : (s = q(this, d, u)),
        s
      );
    };
    function Y(A, d) {
      (this.x = A), (this.y = d);
    }
    (Y.prototype.makeRBT = function (d) {
      for (var u = new Array(d), s = a.prototype._countBits(d) - 1, o = 0; o < d; o++)
        u[o] = this.revBin(o, s, d);
      return u;
    }),
      (Y.prototype.revBin = function (d, u, s) {
        if (d === 0 || d === s - 1) return d;
        for (var o = 0, l = 0; l < u; l++) (o |= (d & 1) << (u - l - 1)), (d >>= 1);
        return o;
      }),
      (Y.prototype.permute = function (d, u, s, o, l, g) {
        for (var m = 0; m < g; m++) (o[m] = u[d[m]]), (l[m] = s[d[m]]);
      }),
      (Y.prototype.transform = function (d, u, s, o, l, g) {
        this.permute(g, d, u, s, o, l);
        for (var m = 1; m < l; m <<= 1)
          for (
            var b = m << 1, f = Math.cos((2 * Math.PI) / b), p = Math.sin((2 * Math.PI) / b), h = 0;
            h < l;
            h += b
          )
            for (var x = f, O = p, C = 0; C < m; C++) {
              var L = s[h + C],
                D = o[h + C],
                Z = s[h + C + m],
                j = o[h + C + m],
                le = x * Z - O * j;
              (j = x * j + O * Z),
                (Z = le),
                (s[h + C] = L + Z),
                (o[h + C] = D + j),
                (s[h + C + m] = L - Z),
                (o[h + C + m] = D - j),
                C !== b && ((le = f * x - p * O), (O = f * O + p * x), (x = le));
            }
      }),
      (Y.prototype.guessLen13b = function (d, u) {
        var s = Math.max(u, d) | 1,
          o = s & 1,
          l = 0;
        for (s = (s / 2) | 0; s; s = s >>> 1) l++;
        return 1 << (l + 1 + o);
      }),
      (Y.prototype.conjugate = function (d, u, s) {
        if (!(s <= 1))
          for (var o = 0; o < s / 2; o++) {
            var l = d[o];
            (d[o] = d[s - o - 1]),
              (d[s - o - 1] = l),
              (l = u[o]),
              (u[o] = -u[s - o - 1]),
              (u[s - o - 1] = -l);
          }
      }),
      (Y.prototype.normalize13b = function (d, u) {
        for (var s = 0, o = 0; o < u / 2; o++) {
          var l = Math.round(d[2 * o + 1] / u) * 8192 + Math.round(d[2 * o] / u) + s;
          (d[o] = l & 67108863), l < 67108864 ? (s = 0) : (s = (l / 67108864) | 0);
        }
        return d;
      }),
      (Y.prototype.convert13b = function (d, u, s, o) {
        for (var l = 0, g = 0; g < u; g++)
          (l = l + (d[g] | 0)),
            (s[2 * g] = l & 8191),
            (l = l >>> 13),
            (s[2 * g + 1] = l & 8191),
            (l = l >>> 13);
        for (g = 2 * u; g < o; ++g) s[g] = 0;
        n(l === 0), n((l & -8192) === 0);
      }),
      (Y.prototype.stub = function (d) {
        for (var u = new Array(d), s = 0; s < d; s++) u[s] = 0;
        return u;
      }),
      (Y.prototype.mulp = function (d, u, s) {
        var o = 2 * this.guessLen13b(d.length, u.length),
          l = this.makeRBT(o),
          g = this.stub(o),
          m = new Array(o),
          b = new Array(o),
          f = new Array(o),
          p = new Array(o),
          h = new Array(o),
          x = new Array(o),
          O = s.words;
        (O.length = o),
          this.convert13b(d.words, d.length, m, o),
          this.convert13b(u.words, u.length, p, o),
          this.transform(m, g, b, f, o, l),
          this.transform(p, g, h, x, o, l);
        for (var C = 0; C < o; C++) {
          var L = b[C] * h[C] - f[C] * x[C];
          (f[C] = b[C] * x[C] + f[C] * h[C]), (b[C] = L);
        }
        return (
          this.conjugate(b, f, o),
          this.transform(b, f, O, g, o, l),
          this.conjugate(O, g, o),
          this.normalize13b(O, o),
          (s.negative = d.negative ^ u.negative),
          (s.length = d.length + u.length),
          s.strip()
        );
      }),
      (a.prototype.mul = function (d) {
        var u = new a(null);
        return (u.words = new Array(this.length + d.length)), this.mulTo(d, u);
      }),
      (a.prototype.mulf = function (d) {
        var u = new a(null);
        return (u.words = new Array(this.length + d.length)), q(this, d, u);
      }),
      (a.prototype.imul = function (d) {
        return this.clone().mulTo(d, this);
      }),
      (a.prototype.imuln = function (d) {
        n(typeof d == 'number'), n(d < 67108864);
        for (var u = 0, s = 0; s < this.length; s++) {
          var o = (this.words[s] | 0) * d,
            l = (o & 67108863) + (u & 67108863);
          (u >>= 26), (u += (o / 67108864) | 0), (u += l >>> 26), (this.words[s] = l & 67108863);
        }
        return u !== 0 && ((this.words[s] = u), this.length++), this;
      }),
      (a.prototype.muln = function (d) {
        return this.clone().imuln(d);
      }),
      (a.prototype.sqr = function () {
        return this.mul(this);
      }),
      (a.prototype.isqr = function () {
        return this.imul(this.clone());
      }),
      (a.prototype.pow = function (d) {
        var u = E(d);
        if (u.length === 0) return new a(1);
        for (var s = this, o = 0; o < u.length && u[o] === 0; o++, s = s.sqr());
        if (++o < u.length)
          for (var l = s.sqr(); o < u.length; o++, l = l.sqr()) u[o] !== 0 && (s = s.mul(l));
        return s;
      }),
      (a.prototype.iushln = function (d) {
        n(typeof d == 'number' && d >= 0);
        var u = d % 26,
          s = (d - u) / 26,
          o = (67108863 >>> (26 - u)) << (26 - u),
          l;
        if (u !== 0) {
          var g = 0;
          for (l = 0; l < this.length; l++) {
            var m = this.words[l] & o,
              b = ((this.words[l] | 0) - m) << u;
            (this.words[l] = b | g), (g = m >>> (26 - u));
          }
          g && ((this.words[l] = g), this.length++);
        }
        if (s !== 0) {
          for (l = this.length - 1; l >= 0; l--) this.words[l + s] = this.words[l];
          for (l = 0; l < s; l++) this.words[l] = 0;
          this.length += s;
        }
        return this.strip();
      }),
      (a.prototype.ishln = function (d) {
        return n(this.negative === 0), this.iushln(d);
      }),
      (a.prototype.iushrn = function (d, u, s) {
        n(typeof d == 'number' && d >= 0);
        var o;
        u ? (o = (u - (u % 26)) / 26) : (o = 0);
        var l = d % 26,
          g = Math.min((d - l) / 26, this.length),
          m = 67108863 ^ ((67108863 >>> l) << l),
          b = s;
        if (((o -= g), (o = Math.max(0, o)), b)) {
          for (var f = 0; f < g; f++) b.words[f] = this.words[f];
          b.length = g;
        }
        if (g !== 0)
          if (this.length > g)
            for (this.length -= g, f = 0; f < this.length; f++) this.words[f] = this.words[f + g];
          else (this.words[0] = 0), (this.length = 1);
        var p = 0;
        for (f = this.length - 1; f >= 0 && (p !== 0 || f >= o); f--) {
          var h = this.words[f] | 0;
          (this.words[f] = (p << (26 - l)) | (h >>> l)), (p = h & m);
        }
        return (
          b && p !== 0 && (b.words[b.length++] = p),
          this.length === 0 && ((this.words[0] = 0), (this.length = 1)),
          this.strip()
        );
      }),
      (a.prototype.ishrn = function (d, u, s) {
        return n(this.negative === 0), this.iushrn(d, u, s);
      }),
      (a.prototype.shln = function (d) {
        return this.clone().ishln(d);
      }),
      (a.prototype.ushln = function (d) {
        return this.clone().iushln(d);
      }),
      (a.prototype.shrn = function (d) {
        return this.clone().ishrn(d);
      }),
      (a.prototype.ushrn = function (d) {
        return this.clone().iushrn(d);
      }),
      (a.prototype.testn = function (d) {
        n(typeof d == 'number' && d >= 0);
        var u = d % 26,
          s = (d - u) / 26,
          o = 1 << u;
        if (this.length <= s) return !1;
        var l = this.words[s];
        return !!(l & o);
      }),
      (a.prototype.imaskn = function (d) {
        n(typeof d == 'number' && d >= 0);
        var u = d % 26,
          s = (d - u) / 26;
        if ((n(this.negative === 0, 'imaskn works only with positive numbers'), this.length <= s))
          return this;
        if ((u !== 0 && s++, (this.length = Math.min(s, this.length)), u !== 0)) {
          var o = 67108863 ^ ((67108863 >>> u) << u);
          this.words[this.length - 1] &= o;
        }
        return this.strip();
      }),
      (a.prototype.maskn = function (d) {
        return this.clone().imaskn(d);
      }),
      (a.prototype.iaddn = function (d) {
        return (
          n(typeof d == 'number'),
          n(d < 67108864),
          d < 0
            ? this.isubn(-d)
            : this.negative !== 0
            ? this.length === 1 && (this.words[0] | 0) < d
              ? ((this.words[0] = d - (this.words[0] | 0)), (this.negative = 0), this)
              : ((this.negative = 0), this.isubn(d), (this.negative = 1), this)
            : this._iaddn(d)
        );
      }),
      (a.prototype._iaddn = function (d) {
        this.words[0] += d;
        for (var u = 0; u < this.length && this.words[u] >= 67108864; u++)
          (this.words[u] -= 67108864),
            u === this.length - 1 ? (this.words[u + 1] = 1) : this.words[u + 1]++;
        return (this.length = Math.max(this.length, u + 1)), this;
      }),
      (a.prototype.isubn = function (d) {
        if ((n(typeof d == 'number'), n(d < 67108864), d < 0)) return this.iaddn(-d);
        if (this.negative !== 0)
          return (this.negative = 0), this.iaddn(d), (this.negative = 1), this;
        if (((this.words[0] -= d), this.length === 1 && this.words[0] < 0))
          (this.words[0] = -this.words[0]), (this.negative = 1);
        else
          for (var u = 0; u < this.length && this.words[u] < 0; u++)
            (this.words[u] += 67108864), (this.words[u + 1] -= 1);
        return this.strip();
      }),
      (a.prototype.addn = function (d) {
        return this.clone().iaddn(d);
      }),
      (a.prototype.subn = function (d) {
        return this.clone().isubn(d);
      }),
      (a.prototype.iabs = function () {
        return (this.negative = 0), this;
      }),
      (a.prototype.abs = function () {
        return this.clone().iabs();
      }),
      (a.prototype._ishlnsubmul = function (d, u, s) {
        var o = d.length + s,
          l;
        this._expand(o);
        var g,
          m = 0;
        for (l = 0; l < d.length; l++) {
          g = (this.words[l + s] | 0) + m;
          var b = (d.words[l] | 0) * u;
          (g -= b & 67108863),
            (m = (g >> 26) - ((b / 67108864) | 0)),
            (this.words[l + s] = g & 67108863);
        }
        for (; l < this.length - s; l++)
          (g = (this.words[l + s] | 0) + m), (m = g >> 26), (this.words[l + s] = g & 67108863);
        if (m === 0) return this.strip();
        for (n(m === -1), m = 0, l = 0; l < this.length; l++)
          (g = -(this.words[l] | 0) + m), (m = g >> 26), (this.words[l] = g & 67108863);
        return (this.negative = 1), this.strip();
      }),
      (a.prototype._wordDiv = function (d, u) {
        var s = this.length - d.length,
          o = this.clone(),
          l = d,
          g = l.words[l.length - 1] | 0,
          m = this._countBits(g);
        (s = 26 - m), s !== 0 && ((l = l.ushln(s)), o.iushln(s), (g = l.words[l.length - 1] | 0));
        var b = o.length - l.length,
          f;
        if (u !== 'mod') {
          (f = new a(null)), (f.length = b + 1), (f.words = new Array(f.length));
          for (var p = 0; p < f.length; p++) f.words[p] = 0;
        }
        var h = o.clone()._ishlnsubmul(l, 1, b);
        h.negative === 0 && ((o = h), f && (f.words[b] = 1));
        for (var x = b - 1; x >= 0; x--) {
          var O = (o.words[l.length + x] | 0) * 67108864 + (o.words[l.length + x - 1] | 0);
          for (O = Math.min((O / g) | 0, 67108863), o._ishlnsubmul(l, O, x); o.negative !== 0; )
            O--, (o.negative = 0), o._ishlnsubmul(l, 1, x), o.isZero() || (o.negative ^= 1);
          f && (f.words[x] = O);
        }
        return (
          f && f.strip(),
          o.strip(),
          u !== 'div' && s !== 0 && o.iushrn(s),
          { div: f || null, mod: o }
        );
      }),
      (a.prototype.divmod = function (d, u, s) {
        if ((n(!d.isZero()), this.isZero())) return { div: new a(0), mod: new a(0) };
        var o, l, g;
        return this.negative !== 0 && d.negative === 0
          ? ((g = this.neg().divmod(d, u)),
            u !== 'mod' && (o = g.div.neg()),
            u !== 'div' && ((l = g.mod.neg()), s && l.negative !== 0 && l.iadd(d)),
            { div: o, mod: l })
          : this.negative === 0 && d.negative !== 0
          ? ((g = this.divmod(d.neg(), u)),
            u !== 'mod' && (o = g.div.neg()),
            { div: o, mod: g.mod })
          : this.negative & d.negative
          ? ((g = this.neg().divmod(d.neg(), u)),
            u !== 'div' && ((l = g.mod.neg()), s && l.negative !== 0 && l.isub(d)),
            { div: g.div, mod: l })
          : d.length > this.length || this.cmp(d) < 0
          ? { div: new a(0), mod: this }
          : d.length === 1
          ? u === 'div'
            ? { div: this.divn(d.words[0]), mod: null }
            : u === 'mod'
            ? { div: null, mod: new a(this.modn(d.words[0])) }
            : { div: this.divn(d.words[0]), mod: new a(this.modn(d.words[0])) }
          : this._wordDiv(d, u);
      }),
      (a.prototype.div = function (d) {
        return this.divmod(d, 'div', !1).div;
      }),
      (a.prototype.mod = function (d) {
        return this.divmod(d, 'mod', !1).mod;
      }),
      (a.prototype.umod = function (d) {
        return this.divmod(d, 'mod', !0).mod;
      }),
      (a.prototype.divRound = function (d) {
        var u = this.divmod(d);
        if (u.mod.isZero()) return u.div;
        var s = u.div.negative !== 0 ? u.mod.isub(d) : u.mod,
          o = d.ushrn(1),
          l = d.andln(1),
          g = s.cmp(o);
        return g < 0 || (l === 1 && g === 0)
          ? u.div
          : u.div.negative !== 0
          ? u.div.isubn(1)
          : u.div.iaddn(1);
      }),
      (a.prototype.modn = function (d) {
        n(d <= 67108863);
        for (var u = (1 << 26) % d, s = 0, o = this.length - 1; o >= 0; o--)
          s = (u * s + (this.words[o] | 0)) % d;
        return s;
      }),
      (a.prototype.idivn = function (d) {
        n(d <= 67108863);
        for (var u = 0, s = this.length - 1; s >= 0; s--) {
          var o = (this.words[s] | 0) + u * 67108864;
          (this.words[s] = (o / d) | 0), (u = o % d);
        }
        return this.strip();
      }),
      (a.prototype.divn = function (d) {
        return this.clone().idivn(d);
      }),
      (a.prototype.egcd = function (d) {
        n(d.negative === 0), n(!d.isZero());
        var u = this,
          s = d.clone();
        u.negative !== 0 ? (u = u.umod(d)) : (u = u.clone());
        for (
          var o = new a(1), l = new a(0), g = new a(0), m = new a(1), b = 0;
          u.isEven() && s.isEven();

        )
          u.iushrn(1), s.iushrn(1), ++b;
        for (var f = s.clone(), p = u.clone(); !u.isZero(); ) {
          for (var h = 0, x = 1; !(u.words[0] & x) && h < 26; ++h, x <<= 1);
          if (h > 0)
            for (u.iushrn(h); h-- > 0; )
              (o.isOdd() || l.isOdd()) && (o.iadd(f), l.isub(p)), o.iushrn(1), l.iushrn(1);
          for (var O = 0, C = 1; !(s.words[0] & C) && O < 26; ++O, C <<= 1);
          if (O > 0)
            for (s.iushrn(O); O-- > 0; )
              (g.isOdd() || m.isOdd()) && (g.iadd(f), m.isub(p)), g.iushrn(1), m.iushrn(1);
          u.cmp(s) >= 0 ? (u.isub(s), o.isub(g), l.isub(m)) : (s.isub(u), g.isub(o), m.isub(l));
        }
        return { a: g, b: m, gcd: s.iushln(b) };
      }),
      (a.prototype._invmp = function (d) {
        n(d.negative === 0), n(!d.isZero());
        var u = this,
          s = d.clone();
        u.negative !== 0 ? (u = u.umod(d)) : (u = u.clone());
        for (var o = new a(1), l = new a(0), g = s.clone(); u.cmpn(1) > 0 && s.cmpn(1) > 0; ) {
          for (var m = 0, b = 1; !(u.words[0] & b) && m < 26; ++m, b <<= 1);
          if (m > 0) for (u.iushrn(m); m-- > 0; ) o.isOdd() && o.iadd(g), o.iushrn(1);
          for (var f = 0, p = 1; !(s.words[0] & p) && f < 26; ++f, p <<= 1);
          if (f > 0) for (s.iushrn(f); f-- > 0; ) l.isOdd() && l.iadd(g), l.iushrn(1);
          u.cmp(s) >= 0 ? (u.isub(s), o.isub(l)) : (s.isub(u), l.isub(o));
        }
        var h;
        return u.cmpn(1) === 0 ? (h = o) : (h = l), h.cmpn(0) < 0 && h.iadd(d), h;
      }),
      (a.prototype.gcd = function (d) {
        if (this.isZero()) return d.abs();
        if (d.isZero()) return this.abs();
        var u = this.clone(),
          s = d.clone();
        (u.negative = 0), (s.negative = 0);
        for (var o = 0; u.isEven() && s.isEven(); o++) u.iushrn(1), s.iushrn(1);
        do {
          for (; u.isEven(); ) u.iushrn(1);
          for (; s.isEven(); ) s.iushrn(1);
          var l = u.cmp(s);
          if (l < 0) {
            var g = u;
            (u = s), (s = g);
          } else if (l === 0 || s.cmpn(1) === 0) break;
          u.isub(s);
        } while (!0);
        return s.iushln(o);
      }),
      (a.prototype.invm = function (d) {
        return this.egcd(d).a.umod(d);
      }),
      (a.prototype.isEven = function () {
        return (this.words[0] & 1) === 0;
      }),
      (a.prototype.isOdd = function () {
        return (this.words[0] & 1) === 1;
      }),
      (a.prototype.andln = function (d) {
        return this.words[0] & d;
      }),
      (a.prototype.bincn = function (d) {
        n(typeof d == 'number');
        var u = d % 26,
          s = (d - u) / 26,
          o = 1 << u;
        if (this.length <= s) return this._expand(s + 1), (this.words[s] |= o), this;
        for (var l = o, g = s; l !== 0 && g < this.length; g++) {
          var m = this.words[g] | 0;
          (m += l), (l = m >>> 26), (m &= 67108863), (this.words[g] = m);
        }
        return l !== 0 && ((this.words[g] = l), this.length++), this;
      }),
      (a.prototype.isZero = function () {
        return this.length === 1 && this.words[0] === 0;
      }),
      (a.prototype.cmpn = function (d) {
        var u = d < 0;
        if (this.negative !== 0 && !u) return -1;
        if (this.negative === 0 && u) return 1;
        this.strip();
        var s;
        if (this.length > 1) s = 1;
        else {
          u && (d = -d), n(d <= 67108863, 'Number is too big');
          var o = this.words[0] | 0;
          s = o === d ? 0 : o < d ? -1 : 1;
        }
        return this.negative !== 0 ? -s | 0 : s;
      }),
      (a.prototype.cmp = function (d) {
        if (this.negative !== 0 && d.negative === 0) return -1;
        if (this.negative === 0 && d.negative !== 0) return 1;
        var u = this.ucmp(d);
        return this.negative !== 0 ? -u | 0 : u;
      }),
      (a.prototype.ucmp = function (d) {
        if (this.length > d.length) return 1;
        if (this.length < d.length) return -1;
        for (var u = 0, s = this.length - 1; s >= 0; s--) {
          var o = this.words[s] | 0,
            l = d.words[s] | 0;
          if (o !== l) {
            o < l ? (u = -1) : o > l && (u = 1);
            break;
          }
        }
        return u;
      }),
      (a.prototype.gtn = function (d) {
        return this.cmpn(d) === 1;
      }),
      (a.prototype.gt = function (d) {
        return this.cmp(d) === 1;
      }),
      (a.prototype.gten = function (d) {
        return this.cmpn(d) >= 0;
      }),
      (a.prototype.gte = function (d) {
        return this.cmp(d) >= 0;
      }),
      (a.prototype.ltn = function (d) {
        return this.cmpn(d) === -1;
      }),
      (a.prototype.lt = function (d) {
        return this.cmp(d) === -1;
      }),
      (a.prototype.lten = function (d) {
        return this.cmpn(d) <= 0;
      }),
      (a.prototype.lte = function (d) {
        return this.cmp(d) <= 0;
      }),
      (a.prototype.eqn = function (d) {
        return this.cmpn(d) === 0;
      }),
      (a.prototype.eq = function (d) {
        return this.cmp(d) === 0;
      }),
      (a.red = function (d) {
        return new W(d);
      }),
      (a.prototype.toRed = function (d) {
        return (
          n(!this.red, 'Already a number in reduction context'),
          n(this.negative === 0, 'red works only with positives'),
          d.convertTo(this)._forceRed(d)
        );
      }),
      (a.prototype.fromRed = function () {
        return (
          n(this.red, 'fromRed works only with numbers in reduction context'),
          this.red.convertFrom(this)
        );
      }),
      (a.prototype._forceRed = function (d) {
        return (this.red = d), this;
      }),
      (a.prototype.forceRed = function (d) {
        return n(!this.red, 'Already a number in reduction context'), this._forceRed(d);
      }),
      (a.prototype.redAdd = function (d) {
        return n(this.red, 'redAdd works only with red numbers'), this.red.add(this, d);
      }),
      (a.prototype.redIAdd = function (d) {
        return n(this.red, 'redIAdd works only with red numbers'), this.red.iadd(this, d);
      }),
      (a.prototype.redSub = function (d) {
        return n(this.red, 'redSub works only with red numbers'), this.red.sub(this, d);
      }),
      (a.prototype.redISub = function (d) {
        return n(this.red, 'redISub works only with red numbers'), this.red.isub(this, d);
      }),
      (a.prototype.redShl = function (d) {
        return n(this.red, 'redShl works only with red numbers'), this.red.shl(this, d);
      }),
      (a.prototype.redMul = function (d) {
        return (
          n(this.red, 'redMul works only with red numbers'),
          this.red._verify2(this, d),
          this.red.mul(this, d)
        );
      }),
      (a.prototype.redIMul = function (d) {
        return (
          n(this.red, 'redMul works only with red numbers'),
          this.red._verify2(this, d),
          this.red.imul(this, d)
        );
      }),
      (a.prototype.redSqr = function () {
        return (
          n(this.red, 'redSqr works only with red numbers'),
          this.red._verify1(this),
          this.red.sqr(this)
        );
      }),
      (a.prototype.redISqr = function () {
        return (
          n(this.red, 'redISqr works only with red numbers'),
          this.red._verify1(this),
          this.red.isqr(this)
        );
      }),
      (a.prototype.redSqrt = function () {
        return (
          n(this.red, 'redSqrt works only with red numbers'),
          this.red._verify1(this),
          this.red.sqrt(this)
        );
      }),
      (a.prototype.redInvm = function () {
        return (
          n(this.red, 'redInvm works only with red numbers'),
          this.red._verify1(this),
          this.red.invm(this)
        );
      }),
      (a.prototype.redNeg = function () {
        return (
          n(this.red, 'redNeg works only with red numbers'),
          this.red._verify1(this),
          this.red.neg(this)
        );
      }),
      (a.prototype.redPow = function (d) {
        return (
          n(this.red && !d.red, 'redPow(normalNum)'), this.red._verify1(this), this.red.pow(this, d)
        );
      });
    var Se = { k256: null, p224: null, p192: null, p25519: null };
    function de(A, d) {
      (this.name = A),
        (this.p = new a(d, 16)),
        (this.n = this.p.bitLength()),
        (this.k = new a(1).iushln(this.n).isub(this.p)),
        (this.tmp = this._tmp());
    }
    (de.prototype._tmp = function () {
      var d = new a(null);
      return (d.words = new Array(Math.ceil(this.n / 13))), d;
    }),
      (de.prototype.ireduce = function (d) {
        var u = d,
          s;
        do
          this.split(u, this.tmp), (u = this.imulK(u)), (u = u.iadd(this.tmp)), (s = u.bitLength());
        while (s > this.n);
        var o = s < this.n ? -1 : u.ucmp(this.p);
        return (
          o === 0
            ? ((u.words[0] = 0), (u.length = 1))
            : o > 0
            ? u.isub(this.p)
            : u.strip !== void 0
            ? u.strip()
            : u._strip(),
          u
        );
      }),
      (de.prototype.split = function (d, u) {
        d.iushrn(this.n, 0, u);
      }),
      (de.prototype.imulK = function (d) {
        return d.imul(this.k);
      });
    function H() {
      de.call(
        this,
        'k256',
        'ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff fffffffe fffffc2f'
      );
    }
    i(H, de),
      (H.prototype.split = function (d, u) {
        for (var s = 4194303, o = Math.min(d.length, 9), l = 0; l < o; l++) u.words[l] = d.words[l];
        if (((u.length = o), d.length <= 9)) {
          (d.words[0] = 0), (d.length = 1);
          return;
        }
        var g = d.words[9];
        for (u.words[u.length++] = g & s, l = 10; l < d.length; l++) {
          var m = d.words[l] | 0;
          (d.words[l - 10] = ((m & s) << 4) | (g >>> 22)), (g = m);
        }
        (g >>>= 22),
          (d.words[l - 10] = g),
          g === 0 && d.length > 10 ? (d.length -= 10) : (d.length -= 9);
      }),
      (H.prototype.imulK = function (d) {
        (d.words[d.length] = 0), (d.words[d.length + 1] = 0), (d.length += 2);
        for (var u = 0, s = 0; s < d.length; s++) {
          var o = d.words[s] | 0;
          (u += o * 977), (d.words[s] = u & 67108863), (u = o * 64 + ((u / 67108864) | 0));
        }
        return (
          d.words[d.length - 1] === 0 && (d.length--, d.words[d.length - 1] === 0 && d.length--), d
        );
      });
    function F() {
      de.call(this, 'p224', 'ffffffff ffffffff ffffffff ffffffff 00000000 00000000 00000001');
    }
    i(F, de);
    function J() {
      de.call(this, 'p192', 'ffffffff ffffffff ffffffff fffffffe ffffffff ffffffff');
    }
    i(J, de);
    function X() {
      de.call(this, '25519', '7fffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffed');
    }
    i(X, de),
      (X.prototype.imulK = function (d) {
        for (var u = 0, s = 0; s < d.length; s++) {
          var o = (d.words[s] | 0) * 19 + u,
            l = o & 67108863;
          (o >>>= 26), (d.words[s] = l), (u = o);
        }
        return u !== 0 && (d.words[d.length++] = u), d;
      }),
      (a._prime = function (d) {
        if (Se[d]) return Se[d];
        var u;
        if (d === 'k256') u = new H();
        else if (d === 'p224') u = new F();
        else if (d === 'p192') u = new J();
        else if (d === 'p25519') u = new X();
        else throw new Error('Unknown prime ' + d);
        return (Se[d] = u), u;
      });
    function W(A) {
      if (typeof A == 'string') {
        var d = a._prime(A);
        (this.m = d.p), (this.prime = d);
      } else n(A.gtn(1), 'modulus must be greater than 1'), (this.m = A), (this.prime = null);
    }
    (W.prototype._verify1 = function (d) {
      n(d.negative === 0, 'red works only with positives'),
        n(d.red, 'red works only with red numbers');
    }),
      (W.prototype._verify2 = function (d, u) {
        n((d.negative | u.negative) === 0, 'red works only with positives'),
          n(d.red && d.red === u.red, 'red works only with red numbers');
      }),
      (W.prototype.imod = function (d) {
        return this.prime ? this.prime.ireduce(d)._forceRed(this) : d.umod(this.m)._forceRed(this);
      }),
      (W.prototype.neg = function (d) {
        return d.isZero() ? d.clone() : this.m.sub(d)._forceRed(this);
      }),
      (W.prototype.add = function (d, u) {
        this._verify2(d, u);
        var s = d.add(u);
        return s.cmp(this.m) >= 0 && s.isub(this.m), s._forceRed(this);
      }),
      (W.prototype.iadd = function (d, u) {
        this._verify2(d, u);
        var s = d.iadd(u);
        return s.cmp(this.m) >= 0 && s.isub(this.m), s;
      }),
      (W.prototype.sub = function (d, u) {
        this._verify2(d, u);
        var s = d.sub(u);
        return s.cmpn(0) < 0 && s.iadd(this.m), s._forceRed(this);
      }),
      (W.prototype.isub = function (d, u) {
        this._verify2(d, u);
        var s = d.isub(u);
        return s.cmpn(0) < 0 && s.iadd(this.m), s;
      }),
      (W.prototype.shl = function (d, u) {
        return this._verify1(d), this.imod(d.ushln(u));
      }),
      (W.prototype.imul = function (d, u) {
        return this._verify2(d, u), this.imod(d.imul(u));
      }),
      (W.prototype.mul = function (d, u) {
        return this._verify2(d, u), this.imod(d.mul(u));
      }),
      (W.prototype.isqr = function (d) {
        return this.imul(d, d.clone());
      }),
      (W.prototype.sqr = function (d) {
        return this.mul(d, d);
      }),
      (W.prototype.sqrt = function (d) {
        if (d.isZero()) return d.clone();
        var u = this.m.andln(3);
        if ((n(u % 2 === 1), u === 3)) {
          var s = this.m.add(new a(1)).iushrn(2);
          return this.pow(d, s);
        }
        for (var o = this.m.subn(1), l = 0; !o.isZero() && o.andln(1) === 0; ) l++, o.iushrn(1);
        n(!o.isZero());
        var g = new a(1).toRed(this),
          m = g.redNeg(),
          b = this.m.subn(1).iushrn(1),
          f = this.m.bitLength();
        for (f = new a(2 * f * f).toRed(this); this.pow(f, b).cmp(m) !== 0; ) f.redIAdd(m);
        for (
          var p = this.pow(f, o), h = this.pow(d, o.addn(1).iushrn(1)), x = this.pow(d, o), O = l;
          x.cmp(g) !== 0;

        ) {
          for (var C = x, L = 0; C.cmp(g) !== 0; L++) C = C.redSqr();
          n(L < O);
          var D = this.pow(p, new a(1).iushln(O - L - 1));
          (h = h.redMul(D)), (p = D.redSqr()), (x = x.redMul(p)), (O = L);
        }
        return h;
      }),
      (W.prototype.invm = function (d) {
        var u = d._invmp(this.m);
        return u.negative !== 0 ? ((u.negative = 0), this.imod(u).redNeg()) : this.imod(u);
      }),
      (W.prototype.pow = function (d, u) {
        if (u.isZero()) return new a(1).toRed(this);
        if (u.cmpn(1) === 0) return d.clone();
        var s = 4,
          o = new Array(1 << s);
        (o[0] = new a(1).toRed(this)), (o[1] = d);
        for (var l = 2; l < o.length; l++) o[l] = this.mul(o[l - 1], d);
        var g = o[0],
          m = 0,
          b = 0,
          f = u.bitLength() % 26;
        for (f === 0 && (f = 26), l = u.length - 1; l >= 0; l--) {
          for (var p = u.words[l], h = f - 1; h >= 0; h--) {
            var x = (p >> h) & 1;
            if ((g !== o[0] && (g = this.sqr(g)), x === 0 && m === 0)) {
              b = 0;
              continue;
            }
            (m <<= 1),
              (m |= x),
              b++,
              !(b !== s && (l !== 0 || h !== 0)) && ((g = this.mul(g, o[m])), (b = 0), (m = 0));
          }
          f = 26;
        }
        return g;
      }),
      (W.prototype.convertTo = function (d) {
        var u = d.umod(this.m);
        return u === d ? u.clone() : u;
      }),
      (W.prototype.convertFrom = function (d) {
        var u = d.clone();
        return (u.red = null), u;
      }),
      (a.mont = function (d) {
        return new G(d);
      });
    function G(A) {
      W.call(this, A),
        (this.shift = this.m.bitLength()),
        this.shift % 26 !== 0 && (this.shift += 26 - (this.shift % 26)),
        (this.r = new a(1).iushln(this.shift)),
        (this.r2 = this.imod(this.r.sqr())),
        (this.rinv = this.r._invmp(this.m)),
        (this.minv = this.rinv.mul(this.r).isubn(1).div(this.m)),
        (this.minv = this.minv.umod(this.r)),
        (this.minv = this.r.sub(this.minv));
    }
    i(G, W),
      (G.prototype.convertTo = function (d) {
        return this.imod(d.ushln(this.shift));
      }),
      (G.prototype.convertFrom = function (d) {
        var u = this.imod(d.mul(this.rinv));
        return (u.red = null), u;
      }),
      (G.prototype.imul = function (d, u) {
        if (d.isZero() || u.isZero()) return (d.words[0] = 0), (d.length = 1), d;
        var s = d.imul(u),
          o = s.maskn(this.shift).mul(this.minv).imaskn(this.shift).mul(this.m),
          l = s.isub(o).iushrn(this.shift),
          g = l;
        return (
          l.cmp(this.m) >= 0 ? (g = l.isub(this.m)) : l.cmpn(0) < 0 && (g = l.iadd(this.m)),
          g._forceRed(this)
        );
      }),
      (G.prototype.mul = function (d, u) {
        if (d.isZero() || u.isZero()) return new a(0)._forceRed(this);
        var s = d.mul(u),
          o = s.maskn(this.shift).mul(this.minv).imaskn(this.shift).mul(this.m),
          l = s.isub(o).iushrn(this.shift),
          g = l;
        return (
          l.cmp(this.m) >= 0 ? (g = l.isub(this.m)) : l.cmpn(0) < 0 && (g = l.iadd(this.m)),
          g._forceRed(this)
        );
      }),
      (G.prototype.invm = function (d) {
        var u = this.imod(d._invmp(this.m).mul(this.r2));
        return u._forceRed(this);
      });
  })(r, ie);
})(hr);
var as = {};
(function (r) {
  var e = r;
  function t(a, c) {
    if (Array.isArray(a)) return a.slice();
    if (!a) return [];
    var v = [];
    if (typeof a != 'string') {
      for (var w = 0; w < a.length; w++) v[w] = a[w] | 0;
      return v;
    }
    if (c === 'hex') {
      (a = a.replace(/[^a-z0-9]+/gi, '')), a.length % 2 !== 0 && (a = '0' + a);
      for (var w = 0; w < a.length; w += 2) v.push(parseInt(a[w] + a[w + 1], 16));
    } else
      for (var w = 0; w < a.length; w++) {
        var y = a.charCodeAt(w),
          M = y >> 8,
          S = y & 255;
        M ? v.push(M, S) : v.push(S);
      }
    return v;
  }
  e.toArray = t;
  function n(a) {
    return a.length === 1 ? '0' + a : a;
  }
  e.zero2 = n;
  function i(a) {
    for (var c = '', v = 0; v < a.length; v++) c += n(a[v].toString(16));
    return c;
  }
  (e.toHex = i),
    (e.encode = function (c, v) {
      return v === 'hex' ? i(c) : c;
    });
})(as);
(function (r) {
  var e = r,
    t = hr.exports,
    n = Xr,
    i = as;
  (e.assert = n),
    (e.toArray = i.toArray),
    (e.zero2 = i.zero2),
    (e.toHex = i.toHex),
    (e.encode = i.encode);
  function a(M, S, I) {
    var E = new Array(Math.max(M.bitLength(), I) + 1);
    E.fill(0);
    for (var R = 1 << (S + 1), T = M.clone(), z = 0; z < E.length; z++) {
      var q,
        Y = T.andln(R - 1);
      T.isOdd() ? (Y > (R >> 1) - 1 ? (q = (R >> 1) - Y) : (q = Y), T.isubn(q)) : (q = 0),
        (E[z] = q),
        T.iushrn(1);
    }
    return E;
  }
  e.getNAF = a;
  function c(M, S) {
    var I = [[], []];
    (M = M.clone()), (S = S.clone());
    for (var E = 0, R = 0, T; M.cmpn(-E) > 0 || S.cmpn(-R) > 0; ) {
      var z = (M.andln(3) + E) & 3,
        q = (S.andln(3) + R) & 3;
      z === 3 && (z = -1), q === 3 && (q = -1);
      var Y;
      z & 1
        ? ((T = (M.andln(7) + E) & 7), (T === 3 || T === 5) && q === 2 ? (Y = -z) : (Y = z))
        : (Y = 0),
        I[0].push(Y);
      var Se;
      q & 1
        ? ((T = (S.andln(7) + R) & 7), (T === 3 || T === 5) && z === 2 ? (Se = -q) : (Se = q))
        : (Se = 0),
        I[1].push(Se),
        2 * E === Y + 1 && (E = 1 - E),
        2 * R === Se + 1 && (R = 1 - R),
        M.iushrn(1),
        S.iushrn(1);
    }
    return I;
  }
  e.getJSF = c;
  function v(M, S, I) {
    var E = '_' + S;
    M.prototype[S] = function () {
      return this[E] !== void 0 ? this[E] : (this[E] = I.call(this));
    };
  }
  e.cachedProperty = v;
  function w(M) {
    return typeof M == 'string' ? e.toArray(M, 'hex') : M;
  }
  e.parseBytes = w;
  function y(M) {
    return new t(M, 'hex', 'le');
  }
  e.intFromLE = y;
})(jt);
var Gi = { exports: {} },
  Ma;
Gi.exports = function (e) {
  return Ma || (Ma = new kr(null)), Ma.generate(e);
};
function kr(r) {
  this.rand = r;
}
Gi.exports.Rand = kr;
kr.prototype.generate = function (e) {
  return this._rand(e);
};
kr.prototype._rand = function (e) {
  if (this.rand.getBytes) return this.rand.getBytes(e);
  for (var t = new Uint8Array(e), n = 0; n < t.length; n++) t[n] = this.rand.getByte();
  return t;
};
if (typeof self == 'object')
  self.crypto && self.crypto.getRandomValues
    ? (kr.prototype._rand = function (e) {
        var t = new Uint8Array(e);
        return self.crypto.getRandomValues(t), t;
      })
    : self.msCrypto && self.msCrypto.getRandomValues
    ? (kr.prototype._rand = function (e) {
        var t = new Uint8Array(e);
        return self.msCrypto.getRandomValues(t), t;
      })
    : typeof window == 'object' &&
      (kr.prototype._rand = function () {
        throw new Error('Not implemented yet');
      });
else
  try {
    var wo = Xa;
    if (typeof wo.randomBytes != 'function') throw new Error('Not supported');
    kr.prototype._rand = function (e) {
      return wo.randomBytes(e);
    };
  } catch {}
var ss = {},
  Ur = hr.exports,
  ei = jt,
  $i = ei.getNAF,
  op = ei.getJSF,
  Di = ei.assert;
function qr(r, e) {
  (this.type = r),
    (this.p = new Ur(e.p, 16)),
    (this.red = e.prime ? Ur.red(e.prime) : Ur.mont(this.p)),
    (this.zero = new Ur(0).toRed(this.red)),
    (this.one = new Ur(1).toRed(this.red)),
    (this.two = new Ur(2).toRed(this.red)),
    (this.n = e.n && new Ur(e.n, 16)),
    (this.g = e.g && this.pointFromJSON(e.g, e.gRed)),
    (this._wnafT1 = new Array(4)),
    (this._wnafT2 = new Array(4)),
    (this._wnafT3 = new Array(4)),
    (this._wnafT4 = new Array(4)),
    (this._bitLength = this.n ? this.n.bitLength() : 0);
  var t = this.n && this.p.div(this.n);
  !t || t.cmpn(100) > 0
    ? (this.redN = null)
    : ((this._maxwellTrick = !0), (this.redN = this.n.toRed(this.red)));
}
var Ki = qr;
qr.prototype.point = function () {
  throw new Error('Not implemented');
};
qr.prototype.validate = function () {
  throw new Error('Not implemented');
};
qr.prototype._fixedNafMul = function (e, t) {
  Di(e.precomputed);
  var n = e._getDoubles(),
    i = $i(t, 1, this._bitLength),
    a = (1 << (n.step + 1)) - (n.step % 2 === 0 ? 2 : 1);
  a /= 3;
  var c = [],
    v,
    w;
  for (v = 0; v < i.length; v += n.step) {
    w = 0;
    for (var y = v + n.step - 1; y >= v; y--) w = (w << 1) + i[y];
    c.push(w);
  }
  for (
    var M = this.jpoint(null, null, null), S = this.jpoint(null, null, null), I = a;
    I > 0;
    I--
  ) {
    for (v = 0; v < c.length; v++)
      (w = c[v]),
        w === I ? (S = S.mixedAdd(n.points[v])) : w === -I && (S = S.mixedAdd(n.points[v].neg()));
    M = M.add(S);
  }
  return M.toP();
};
qr.prototype._wnafMul = function (e, t) {
  var n = 4,
    i = e._getNAFPoints(n);
  n = i.wnd;
  for (
    var a = i.points,
      c = $i(t, n, this._bitLength),
      v = this.jpoint(null, null, null),
      w = c.length - 1;
    w >= 0;
    w--
  ) {
    for (var y = 0; w >= 0 && c[w] === 0; w--) y++;
    if ((w >= 0 && y++, (v = v.dblp(y)), w < 0)) break;
    var M = c[w];
    Di(M !== 0),
      e.type === 'affine'
        ? M > 0
          ? (v = v.mixedAdd(a[(M - 1) >> 1]))
          : (v = v.mixedAdd(a[(-M - 1) >> 1].neg()))
        : M > 0
        ? (v = v.add(a[(M - 1) >> 1]))
        : (v = v.add(a[(-M - 1) >> 1].neg()));
  }
  return e.type === 'affine' ? v.toP() : v;
};
qr.prototype._wnafMulAdd = function (e, t, n, i, a) {
  var c = this._wnafT1,
    v = this._wnafT2,
    w = this._wnafT3,
    y = 0,
    M,
    S,
    I;
  for (M = 0; M < i; M++) {
    I = t[M];
    var E = I._getNAFPoints(e);
    (c[M] = E.wnd), (v[M] = E.points);
  }
  for (M = i - 1; M >= 1; M -= 2) {
    var R = M - 1,
      T = M;
    if (c[R] !== 1 || c[T] !== 1) {
      (w[R] = $i(n[R], c[R], this._bitLength)),
        (w[T] = $i(n[T], c[T], this._bitLength)),
        (y = Math.max(w[R].length, y)),
        (y = Math.max(w[T].length, y));
      continue;
    }
    var z = [t[R], null, null, t[T]];
    t[R].y.cmp(t[T].y) === 0
      ? ((z[1] = t[R].add(t[T])), (z[2] = t[R].toJ().mixedAdd(t[T].neg())))
      : t[R].y.cmp(t[T].y.redNeg()) === 0
      ? ((z[1] = t[R].toJ().mixedAdd(t[T])), (z[2] = t[R].add(t[T].neg())))
      : ((z[1] = t[R].toJ().mixedAdd(t[T])), (z[2] = t[R].toJ().mixedAdd(t[T].neg())));
    var q = [-3, -1, -5, -7, 0, 7, 5, 1, 3],
      Y = op(n[R], n[T]);
    for (
      y = Math.max(Y[0].length, y), w[R] = new Array(y), w[T] = new Array(y), S = 0;
      S < y;
      S++
    ) {
      var Se = Y[0][S] | 0,
        de = Y[1][S] | 0;
      (w[R][S] = q[(Se + 1) * 3 + (de + 1)]), (w[T][S] = 0), (v[R] = z);
    }
  }
  var H = this.jpoint(null, null, null),
    F = this._wnafT4;
  for (M = y; M >= 0; M--) {
    for (var J = 0; M >= 0; ) {
      var X = !0;
      for (S = 0; S < i; S++) (F[S] = w[S][M] | 0), F[S] !== 0 && (X = !1);
      if (!X) break;
      J++, M--;
    }
    if ((M >= 0 && J++, (H = H.dblp(J)), M < 0)) break;
    for (S = 0; S < i; S++) {
      var W = F[S];
      W !== 0 &&
        (W > 0 ? (I = v[S][(W - 1) >> 1]) : W < 0 && (I = v[S][(-W - 1) >> 1].neg()),
        I.type === 'affine' ? (H = H.mixedAdd(I)) : (H = H.add(I)));
    }
  }
  for (M = 0; M < i; M++) v[M] = null;
  return a ? H : H.toP();
};
function Gt(r, e) {
  (this.curve = r), (this.type = e), (this.precomputed = null);
}
qr.BasePoint = Gt;
Gt.prototype.eq = function () {
  throw new Error('Not implemented');
};
Gt.prototype.validate = function () {
  return this.curve.validate(this);
};
qr.prototype.decodePoint = function (e, t) {
  e = ei.toArray(e, t);
  var n = this.p.byteLength();
  if ((e[0] === 4 || e[0] === 6 || e[0] === 7) && e.length - 1 === 2 * n) {
    e[0] === 6 ? Di(e[e.length - 1] % 2 === 0) : e[0] === 7 && Di(e[e.length - 1] % 2 === 1);
    var i = this.point(e.slice(1, 1 + n), e.slice(1 + n, 1 + 2 * n));
    return i;
  } else if ((e[0] === 2 || e[0] === 3) && e.length - 1 === n)
    return this.pointFromX(e.slice(1, 1 + n), e[0] === 3);
  throw new Error('Unknown point format');
};
Gt.prototype.encodeCompressed = function (e) {
  return this.encode(e, !0);
};
Gt.prototype._encode = function (e) {
  var t = this.curve.p.byteLength(),
    n = this.getX().toArray('be', t);
  return e ? [this.getY().isEven() ? 2 : 3].concat(n) : [4].concat(n, this.getY().toArray('be', t));
};
Gt.prototype.encode = function (e, t) {
  return ei.encode(this._encode(t), e);
};
Gt.prototype.precompute = function (e) {
  if (this.precomputed) return this;
  var t = { doubles: null, naf: null, beta: null };
  return (
    (t.naf = this._getNAFPoints(8)),
    (t.doubles = this._getDoubles(4, e)),
    (t.beta = this._getBeta()),
    (this.precomputed = t),
    this
  );
};
Gt.prototype._hasDoubles = function (e) {
  if (!this.precomputed) return !1;
  var t = this.precomputed.doubles;
  return t ? t.points.length >= Math.ceil((e.bitLength() + 1) / t.step) : !1;
};
Gt.prototype._getDoubles = function (e, t) {
  if (this.precomputed && this.precomputed.doubles) return this.precomputed.doubles;
  for (var n = [this], i = this, a = 0; a < t; a += e) {
    for (var c = 0; c < e; c++) i = i.dbl();
    n.push(i);
  }
  return { step: e, points: n };
};
Gt.prototype._getNAFPoints = function (e) {
  if (this.precomputed && this.precomputed.naf) return this.precomputed.naf;
  for (var t = [this], n = (1 << e) - 1, i = n === 1 ? null : this.dbl(), a = 1; a < n; a++)
    t[a] = t[a - 1].add(i);
  return { wnd: e, points: t };
};
Gt.prototype._getBeta = function () {
  return null;
};
Gt.prototype.dblp = function (e) {
  for (var t = this, n = 0; n < e; n++) t = t.dbl();
  return t;
};
var fp = jt,
  bt = hr.exports,
  os = hn.exports,
  In = Ki,
  cp = fp.assert;
function Kt(r) {
  In.call(this, 'short', r),
    (this.a = new bt(r.a, 16).toRed(this.red)),
    (this.b = new bt(r.b, 16).toRed(this.red)),
    (this.tinv = this.two.redInvm()),
    (this.zeroA = this.a.fromRed().cmpn(0) === 0),
    (this.threeA = this.a.fromRed().sub(this.p).cmpn(-3) === 0),
    (this.endo = this._getEndomorphism(r)),
    (this._endoWnafT1 = new Array(4)),
    (this._endoWnafT2 = new Array(4));
}
os(Kt, In);
var up = Kt;
Kt.prototype._getEndomorphism = function (e) {
  if (!(!this.zeroA || !this.g || !this.n || this.p.modn(3) !== 1)) {
    var t, n;
    if (e.beta) t = new bt(e.beta, 16).toRed(this.red);
    else {
      var i = this._getEndoRoots(this.p);
      (t = i[0].cmp(i[1]) < 0 ? i[0] : i[1]), (t = t.toRed(this.red));
    }
    if (e.lambda) n = new bt(e.lambda, 16);
    else {
      var a = this._getEndoRoots(this.n);
      this.g.mul(a[0]).x.cmp(this.g.x.redMul(t)) === 0
        ? (n = a[0])
        : ((n = a[1]), cp(this.g.mul(n).x.cmp(this.g.x.redMul(t)) === 0));
    }
    var c;
    return (
      e.basis
        ? (c = e.basis.map(function (v) {
            return { a: new bt(v.a, 16), b: new bt(v.b, 16) };
          }))
        : (c = this._getEndoBasis(n)),
      { beta: t, lambda: n, basis: c }
    );
  }
};
Kt.prototype._getEndoRoots = function (e) {
  var t = e === this.p ? this.red : bt.mont(e),
    n = new bt(2).toRed(t).redInvm(),
    i = n.redNeg(),
    a = new bt(3).toRed(t).redNeg().redSqrt().redMul(n),
    c = i.redAdd(a).fromRed(),
    v = i.redSub(a).fromRed();
  return [c, v];
};
Kt.prototype._getEndoBasis = function (e) {
  for (
    var t = this.n.ushrn(Math.floor(this.n.bitLength() / 2)),
      n = e,
      i = this.n.clone(),
      a = new bt(1),
      c = new bt(0),
      v = new bt(0),
      w = new bt(1),
      y,
      M,
      S,
      I,
      E,
      R,
      T,
      z = 0,
      q,
      Y;
    n.cmpn(0) !== 0;

  ) {
    var Se = i.div(n);
    (q = i.sub(Se.mul(n))), (Y = v.sub(Se.mul(a)));
    var de = w.sub(Se.mul(c));
    if (!S && q.cmp(t) < 0) (y = T.neg()), (M = a), (S = q.neg()), (I = Y);
    else if (S && ++z === 2) break;
    (T = q), (i = n), (n = q), (v = a), (a = Y), (w = c), (c = de);
  }
  (E = q.neg()), (R = Y);
  var H = S.sqr().add(I.sqr()),
    F = E.sqr().add(R.sqr());
  return (
    F.cmp(H) >= 0 && ((E = y), (R = M)),
    S.negative && ((S = S.neg()), (I = I.neg())),
    E.negative && ((E = E.neg()), (R = R.neg())),
    [
      { a: S, b: I },
      { a: E, b: R },
    ]
  );
};
Kt.prototype._endoSplit = function (e) {
  var t = this.endo.basis,
    n = t[0],
    i = t[1],
    a = i.b.mul(e).divRound(this.n),
    c = n.b.neg().mul(e).divRound(this.n),
    v = a.mul(n.a),
    w = c.mul(i.a),
    y = a.mul(n.b),
    M = c.mul(i.b),
    S = e.sub(v).sub(w),
    I = y.add(M).neg();
  return { k1: S, k2: I };
};
Kt.prototype.pointFromX = function (e, t) {
  (e = new bt(e, 16)), e.red || (e = e.toRed(this.red));
  var n = e.redSqr().redMul(e).redIAdd(e.redMul(this.a)).redIAdd(this.b),
    i = n.redSqrt();
  if (i.redSqr().redSub(n).cmp(this.zero) !== 0) throw new Error('invalid point');
  var a = i.fromRed().isOdd();
  return ((t && !a) || (!t && a)) && (i = i.redNeg()), this.point(e, i);
};
Kt.prototype.validate = function (e) {
  if (e.inf) return !0;
  var t = e.x,
    n = e.y,
    i = this.a.redMul(t),
    a = t.redSqr().redMul(t).redIAdd(i).redIAdd(this.b);
  return n.redSqr().redISub(a).cmpn(0) === 0;
};
Kt.prototype._endoWnafMulAdd = function (e, t, n) {
  for (var i = this._endoWnafT1, a = this._endoWnafT2, c = 0; c < e.length; c++) {
    var v = this._endoSplit(t[c]),
      w = e[c],
      y = w._getBeta();
    v.k1.negative && (v.k1.ineg(), (w = w.neg(!0))),
      v.k2.negative && (v.k2.ineg(), (y = y.neg(!0))),
      (i[c * 2] = w),
      (i[c * 2 + 1] = y),
      (a[c * 2] = v.k1),
      (a[c * 2 + 1] = v.k2);
  }
  for (var M = this._wnafMulAdd(1, i, a, c * 2, n), S = 0; S < c * 2; S++)
    (i[S] = null), (a[S] = null);
  return M;
};
function At(r, e, t, n) {
  In.BasePoint.call(this, r, 'affine'),
    e === null && t === null
      ? ((this.x = null), (this.y = null), (this.inf = !0))
      : ((this.x = new bt(e, 16)),
        (this.y = new bt(t, 16)),
        n && (this.x.forceRed(this.curve.red), this.y.forceRed(this.curve.red)),
        this.x.red || (this.x = this.x.toRed(this.curve.red)),
        this.y.red || (this.y = this.y.toRed(this.curve.red)),
        (this.inf = !1));
}
os(At, In.BasePoint);
Kt.prototype.point = function (e, t, n) {
  return new At(this, e, t, n);
};
Kt.prototype.pointFromJSON = function (e, t) {
  return At.fromJSON(this, e, t);
};
At.prototype._getBeta = function () {
  if (this.curve.endo) {
    var e = this.precomputed;
    if (e && e.beta) return e.beta;
    var t = this.curve.point(this.x.redMul(this.curve.endo.beta), this.y);
    if (e) {
      var n = this.curve,
        i = function (a) {
          return n.point(a.x.redMul(n.endo.beta), a.y);
        };
      (e.beta = t),
        (t.precomputed = {
          beta: null,
          naf: e.naf && { wnd: e.naf.wnd, points: e.naf.points.map(i) },
          doubles: e.doubles && { step: e.doubles.step, points: e.doubles.points.map(i) },
        });
    }
    return t;
  }
};
At.prototype.toJSON = function () {
  return this.precomputed
    ? [
        this.x,
        this.y,
        this.precomputed && {
          doubles: this.precomputed.doubles && {
            step: this.precomputed.doubles.step,
            points: this.precomputed.doubles.points.slice(1),
          },
          naf: this.precomputed.naf && {
            wnd: this.precomputed.naf.wnd,
            points: this.precomputed.naf.points.slice(1),
          },
        },
      ]
    : [this.x, this.y];
};
At.fromJSON = function (e, t, n) {
  typeof t == 'string' && (t = JSON.parse(t));
  var i = e.point(t[0], t[1], n);
  if (!t[2]) return i;
  function a(v) {
    return e.point(v[0], v[1], n);
  }
  var c = t[2];
  return (
    (i.precomputed = {
      beta: null,
      doubles: c.doubles && { step: c.doubles.step, points: [i].concat(c.doubles.points.map(a)) },
      naf: c.naf && { wnd: c.naf.wnd, points: [i].concat(c.naf.points.map(a)) },
    }),
    i
  );
};
At.prototype.inspect = function () {
  return this.isInfinity()
    ? '<EC Point Infinity>'
    : '<EC Point x: ' +
        this.x.fromRed().toString(16, 2) +
        ' y: ' +
        this.y.fromRed().toString(16, 2) +
        '>';
};
At.prototype.isInfinity = function () {
  return this.inf;
};
At.prototype.add = function (e) {
  if (this.inf) return e;
  if (e.inf) return this;
  if (this.eq(e)) return this.dbl();
  if (this.neg().eq(e)) return this.curve.point(null, null);
  if (this.x.cmp(e.x) === 0) return this.curve.point(null, null);
  var t = this.y.redSub(e.y);
  t.cmpn(0) !== 0 && (t = t.redMul(this.x.redSub(e.x).redInvm()));
  var n = t.redSqr().redISub(this.x).redISub(e.x),
    i = t.redMul(this.x.redSub(n)).redISub(this.y);
  return this.curve.point(n, i);
};
At.prototype.dbl = function () {
  if (this.inf) return this;
  var e = this.y.redAdd(this.y);
  if (e.cmpn(0) === 0) return this.curve.point(null, null);
  var t = this.curve.a,
    n = this.x.redSqr(),
    i = e.redInvm(),
    a = n.redAdd(n).redIAdd(n).redIAdd(t).redMul(i),
    c = a.redSqr().redISub(this.x.redAdd(this.x)),
    v = a.redMul(this.x.redSub(c)).redISub(this.y);
  return this.curve.point(c, v);
};
At.prototype.getX = function () {
  return this.x.fromRed();
};
At.prototype.getY = function () {
  return this.y.fromRed();
};
At.prototype.mul = function (e) {
  return (
    (e = new bt(e, 16)),
    this.isInfinity()
      ? this
      : this._hasDoubles(e)
      ? this.curve._fixedNafMul(this, e)
      : this.curve.endo
      ? this.curve._endoWnafMulAdd([this], [e])
      : this.curve._wnafMul(this, e)
  );
};
At.prototype.mulAdd = function (e, t, n) {
  var i = [this, t],
    a = [e, n];
  return this.curve.endo ? this.curve._endoWnafMulAdd(i, a) : this.curve._wnafMulAdd(1, i, a, 2);
};
At.prototype.jmulAdd = function (e, t, n) {
  var i = [this, t],
    a = [e, n];
  return this.curve.endo
    ? this.curve._endoWnafMulAdd(i, a, !0)
    : this.curve._wnafMulAdd(1, i, a, 2, !0);
};
At.prototype.eq = function (e) {
  return (
    this === e ||
    (this.inf === e.inf && (this.inf || (this.x.cmp(e.x) === 0 && this.y.cmp(e.y) === 0)))
  );
};
At.prototype.neg = function (e) {
  if (this.inf) return this;
  var t = this.curve.point(this.x, this.y.redNeg());
  if (e && this.precomputed) {
    var n = this.precomputed,
      i = function (a) {
        return a.neg();
      };
    t.precomputed = {
      naf: n.naf && { wnd: n.naf.wnd, points: n.naf.points.map(i) },
      doubles: n.doubles && { step: n.doubles.step, points: n.doubles.points.map(i) },
    };
  }
  return t;
};
At.prototype.toJ = function () {
  if (this.inf) return this.curve.jpoint(null, null, null);
  var e = this.curve.jpoint(this.x, this.y, this.curve.one);
  return e;
};
function It(r, e, t, n) {
  In.BasePoint.call(this, r, 'jacobian'),
    e === null && t === null && n === null
      ? ((this.x = this.curve.one), (this.y = this.curve.one), (this.z = new bt(0)))
      : ((this.x = new bt(e, 16)), (this.y = new bt(t, 16)), (this.z = new bt(n, 16))),
    this.x.red || (this.x = this.x.toRed(this.curve.red)),
    this.y.red || (this.y = this.y.toRed(this.curve.red)),
    this.z.red || (this.z = this.z.toRed(this.curve.red)),
    (this.zOne = this.z === this.curve.one);
}
os(It, In.BasePoint);
Kt.prototype.jpoint = function (e, t, n) {
  return new It(this, e, t, n);
};
It.prototype.toP = function () {
  if (this.isInfinity()) return this.curve.point(null, null);
  var e = this.z.redInvm(),
    t = e.redSqr(),
    n = this.x.redMul(t),
    i = this.y.redMul(t).redMul(e);
  return this.curve.point(n, i);
};
It.prototype.neg = function () {
  return this.curve.jpoint(this.x, this.y.redNeg(), this.z);
};
It.prototype.add = function (e) {
  if (this.isInfinity()) return e;
  if (e.isInfinity()) return this;
  var t = e.z.redSqr(),
    n = this.z.redSqr(),
    i = this.x.redMul(t),
    a = e.x.redMul(n),
    c = this.y.redMul(t.redMul(e.z)),
    v = e.y.redMul(n.redMul(this.z)),
    w = i.redSub(a),
    y = c.redSub(v);
  if (w.cmpn(0) === 0) return y.cmpn(0) !== 0 ? this.curve.jpoint(null, null, null) : this.dbl();
  var M = w.redSqr(),
    S = M.redMul(w),
    I = i.redMul(M),
    E = y.redSqr().redIAdd(S).redISub(I).redISub(I),
    R = y.redMul(I.redISub(E)).redISub(c.redMul(S)),
    T = this.z.redMul(e.z).redMul(w);
  return this.curve.jpoint(E, R, T);
};
It.prototype.mixedAdd = function (e) {
  if (this.isInfinity()) return e.toJ();
  if (e.isInfinity()) return this;
  var t = this.z.redSqr(),
    n = this.x,
    i = e.x.redMul(t),
    a = this.y,
    c = e.y.redMul(t).redMul(this.z),
    v = n.redSub(i),
    w = a.redSub(c);
  if (v.cmpn(0) === 0) return w.cmpn(0) !== 0 ? this.curve.jpoint(null, null, null) : this.dbl();
  var y = v.redSqr(),
    M = y.redMul(v),
    S = n.redMul(y),
    I = w.redSqr().redIAdd(M).redISub(S).redISub(S),
    E = w.redMul(S.redISub(I)).redISub(a.redMul(M)),
    R = this.z.redMul(v);
  return this.curve.jpoint(I, E, R);
};
It.prototype.dblp = function (e) {
  if (e === 0) return this;
  if (this.isInfinity()) return this;
  if (!e) return this.dbl();
  var t;
  if (this.curve.zeroA || this.curve.threeA) {
    var n = this;
    for (t = 0; t < e; t++) n = n.dbl();
    return n;
  }
  var i = this.curve.a,
    a = this.curve.tinv,
    c = this.x,
    v = this.y,
    w = this.z,
    y = w.redSqr().redSqr(),
    M = v.redAdd(v);
  for (t = 0; t < e; t++) {
    var S = c.redSqr(),
      I = M.redSqr(),
      E = I.redSqr(),
      R = S.redAdd(S).redIAdd(S).redIAdd(i.redMul(y)),
      T = c.redMul(I),
      z = R.redSqr().redISub(T.redAdd(T)),
      q = T.redISub(z),
      Y = R.redMul(q);
    Y = Y.redIAdd(Y).redISub(E);
    var Se = M.redMul(w);
    t + 1 < e && (y = y.redMul(E)), (c = z), (w = Se), (M = Y);
  }
  return this.curve.jpoint(c, M.redMul(a), w);
};
It.prototype.dbl = function () {
  return this.isInfinity()
    ? this
    : this.curve.zeroA
    ? this._zeroDbl()
    : this.curve.threeA
    ? this._threeDbl()
    : this._dbl();
};
It.prototype._zeroDbl = function () {
  var e, t, n;
  if (this.zOne) {
    var i = this.x.redSqr(),
      a = this.y.redSqr(),
      c = a.redSqr(),
      v = this.x.redAdd(a).redSqr().redISub(i).redISub(c);
    v = v.redIAdd(v);
    var w = i.redAdd(i).redIAdd(i),
      y = w.redSqr().redISub(v).redISub(v),
      M = c.redIAdd(c);
    (M = M.redIAdd(M)),
      (M = M.redIAdd(M)),
      (e = y),
      (t = w.redMul(v.redISub(y)).redISub(M)),
      (n = this.y.redAdd(this.y));
  } else {
    var S = this.x.redSqr(),
      I = this.y.redSqr(),
      E = I.redSqr(),
      R = this.x.redAdd(I).redSqr().redISub(S).redISub(E);
    R = R.redIAdd(R);
    var T = S.redAdd(S).redIAdd(S),
      z = T.redSqr(),
      q = E.redIAdd(E);
    (q = q.redIAdd(q)),
      (q = q.redIAdd(q)),
      (e = z.redISub(R).redISub(R)),
      (t = T.redMul(R.redISub(e)).redISub(q)),
      (n = this.y.redMul(this.z)),
      (n = n.redIAdd(n));
  }
  return this.curve.jpoint(e, t, n);
};
It.prototype._threeDbl = function () {
  var e, t, n;
  if (this.zOne) {
    var i = this.x.redSqr(),
      a = this.y.redSqr(),
      c = a.redSqr(),
      v = this.x.redAdd(a).redSqr().redISub(i).redISub(c);
    v = v.redIAdd(v);
    var w = i.redAdd(i).redIAdd(i).redIAdd(this.curve.a),
      y = w.redSqr().redISub(v).redISub(v);
    e = y;
    var M = c.redIAdd(c);
    (M = M.redIAdd(M)),
      (M = M.redIAdd(M)),
      (t = w.redMul(v.redISub(y)).redISub(M)),
      (n = this.y.redAdd(this.y));
  } else {
    var S = this.z.redSqr(),
      I = this.y.redSqr(),
      E = this.x.redMul(I),
      R = this.x.redSub(S).redMul(this.x.redAdd(S));
    R = R.redAdd(R).redIAdd(R);
    var T = E.redIAdd(E);
    T = T.redIAdd(T);
    var z = T.redAdd(T);
    (e = R.redSqr().redISub(z)), (n = this.y.redAdd(this.z).redSqr().redISub(I).redISub(S));
    var q = I.redSqr();
    (q = q.redIAdd(q)),
      (q = q.redIAdd(q)),
      (q = q.redIAdd(q)),
      (t = R.redMul(T.redISub(e)).redISub(q));
  }
  return this.curve.jpoint(e, t, n);
};
It.prototype._dbl = function () {
  var e = this.curve.a,
    t = this.x,
    n = this.y,
    i = this.z,
    a = i.redSqr().redSqr(),
    c = t.redSqr(),
    v = n.redSqr(),
    w = c.redAdd(c).redIAdd(c).redIAdd(e.redMul(a)),
    y = t.redAdd(t);
  y = y.redIAdd(y);
  var M = y.redMul(v),
    S = w.redSqr().redISub(M.redAdd(M)),
    I = M.redISub(S),
    E = v.redSqr();
  (E = E.redIAdd(E)), (E = E.redIAdd(E)), (E = E.redIAdd(E));
  var R = w.redMul(I).redISub(E),
    T = n.redAdd(n).redMul(i);
  return this.curve.jpoint(S, R, T);
};
It.prototype.trpl = function () {
  if (!this.curve.zeroA) return this.dbl().add(this);
  var e = this.x.redSqr(),
    t = this.y.redSqr(),
    n = this.z.redSqr(),
    i = t.redSqr(),
    a = e.redAdd(e).redIAdd(e),
    c = a.redSqr(),
    v = this.x.redAdd(t).redSqr().redISub(e).redISub(i);
  (v = v.redIAdd(v)), (v = v.redAdd(v).redIAdd(v)), (v = v.redISub(c));
  var w = v.redSqr(),
    y = i.redIAdd(i);
  (y = y.redIAdd(y)), (y = y.redIAdd(y)), (y = y.redIAdd(y));
  var M = a.redIAdd(v).redSqr().redISub(c).redISub(w).redISub(y),
    S = t.redMul(M);
  (S = S.redIAdd(S)), (S = S.redIAdd(S));
  var I = this.x.redMul(w).redISub(S);
  (I = I.redIAdd(I)), (I = I.redIAdd(I));
  var E = this.y.redMul(M.redMul(y.redISub(M)).redISub(v.redMul(w)));
  (E = E.redIAdd(E)), (E = E.redIAdd(E)), (E = E.redIAdd(E));
  var R = this.z.redAdd(v).redSqr().redISub(n).redISub(w);
  return this.curve.jpoint(I, E, R);
};
It.prototype.mul = function (e, t) {
  return (e = new bt(e, t)), this.curve._wnafMul(this, e);
};
It.prototype.eq = function (e) {
  if (e.type === 'affine') return this.eq(e.toJ());
  if (this === e) return !0;
  var t = this.z.redSqr(),
    n = e.z.redSqr();
  if (this.x.redMul(n).redISub(e.x.redMul(t)).cmpn(0) !== 0) return !1;
  var i = t.redMul(this.z),
    a = n.redMul(e.z);
  return this.y.redMul(a).redISub(e.y.redMul(i)).cmpn(0) === 0;
};
It.prototype.eqXToP = function (e) {
  var t = this.z.redSqr(),
    n = e.toRed(this.curve.red).redMul(t);
  if (this.x.cmp(n) === 0) return !0;
  for (var i = e.clone(), a = this.curve.redN.redMul(t); ; ) {
    if ((i.iadd(this.curve.n), i.cmp(this.curve.p) >= 0)) return !1;
    if ((n.redIAdd(a), this.x.cmp(n) === 0)) return !0;
  }
};
It.prototype.inspect = function () {
  return this.isInfinity()
    ? '<EC JPoint Infinity>'
    : '<EC JPoint x: ' +
        this.x.toString(16, 2) +
        ' y: ' +
        this.y.toString(16, 2) +
        ' z: ' +
        this.z.toString(16, 2) +
        '>';
};
It.prototype.isInfinity = function () {
  return this.z.cmpn(0) === 0;
};
var on = hr.exports,
  H0 = hn.exports,
  Xi = Ki,
  dp = jt;
function Rn(r) {
  Xi.call(this, 'mont', r),
    (this.a = new on(r.a, 16).toRed(this.red)),
    (this.b = new on(r.b, 16).toRed(this.red)),
    (this.i4 = new on(4).toRed(this.red).redInvm()),
    (this.two = new on(2).toRed(this.red)),
    (this.a24 = this.i4.redMul(this.a.redAdd(this.two)));
}
H0(Rn, Xi);
var hp = Rn;
Rn.prototype.validate = function (e) {
  var t = e.normalize().x,
    n = t.redSqr(),
    i = n.redMul(t).redAdd(n.redMul(this.a)).redAdd(t),
    a = i.redSqrt();
  return a.redSqr().cmp(i) === 0;
};
function St(r, e, t) {
  Xi.BasePoint.call(this, r, 'projective'),
    e === null && t === null
      ? ((this.x = this.curve.one), (this.z = this.curve.zero))
      : ((this.x = new on(e, 16)),
        (this.z = new on(t, 16)),
        this.x.red || (this.x = this.x.toRed(this.curve.red)),
        this.z.red || (this.z = this.z.toRed(this.curve.red)));
}
H0(St, Xi.BasePoint);
Rn.prototype.decodePoint = function (e, t) {
  return this.point(dp.toArray(e, t), 1);
};
Rn.prototype.point = function (e, t) {
  return new St(this, e, t);
};
Rn.prototype.pointFromJSON = function (e) {
  return St.fromJSON(this, e);
};
St.prototype.precompute = function () {};
St.prototype._encode = function () {
  return this.getX().toArray('be', this.curve.p.byteLength());
};
St.fromJSON = function (e, t) {
  return new St(e, t[0], t[1] || e.one);
};
St.prototype.inspect = function () {
  return this.isInfinity()
    ? '<EC Point Infinity>'
    : '<EC Point x: ' +
        this.x.fromRed().toString(16, 2) +
        ' z: ' +
        this.z.fromRed().toString(16, 2) +
        '>';
};
St.prototype.isInfinity = function () {
  return this.z.cmpn(0) === 0;
};
St.prototype.dbl = function () {
  var e = this.x.redAdd(this.z),
    t = e.redSqr(),
    n = this.x.redSub(this.z),
    i = n.redSqr(),
    a = t.redSub(i),
    c = t.redMul(i),
    v = a.redMul(i.redAdd(this.curve.a24.redMul(a)));
  return this.curve.point(c, v);
};
St.prototype.add = function () {
  throw new Error('Not supported on Montgomery curve');
};
St.prototype.diffAdd = function (e, t) {
  var n = this.x.redAdd(this.z),
    i = this.x.redSub(this.z),
    a = e.x.redAdd(e.z),
    c = e.x.redSub(e.z),
    v = c.redMul(n),
    w = a.redMul(i),
    y = t.z.redMul(v.redAdd(w).redSqr()),
    M = t.x.redMul(v.redISub(w).redSqr());
  return this.curve.point(y, M);
};
St.prototype.mul = function (e) {
  for (
    var t = e.clone(), n = this, i = this.curve.point(null, null), a = this, c = [];
    t.cmpn(0) !== 0;
    t.iushrn(1)
  )
    c.push(t.andln(1));
  for (var v = c.length - 1; v >= 0; v--)
    c[v] === 0 ? ((n = n.diffAdd(i, a)), (i = i.dbl())) : ((i = n.diffAdd(i, a)), (n = n.dbl()));
  return i;
};
St.prototype.mulAdd = function () {
  throw new Error('Not supported on Montgomery curve');
};
St.prototype.jumlAdd = function () {
  throw new Error('Not supported on Montgomery curve');
};
St.prototype.eq = function (e) {
  return this.getX().cmp(e.getX()) === 0;
};
St.prototype.normalize = function () {
  return (this.x = this.x.redMul(this.z.redInvm())), (this.z = this.curve.one), this;
};
St.prototype.getX = function () {
  return this.normalize(), this.x.fromRed();
};
var lp = jt,
  Ir = hr.exports,
  W0 = hn.exports,
  Zi = Ki,
  pp = lp.assert;
function lr(r) {
  (this.twisted = (r.a | 0) !== 1),
    (this.mOneA = this.twisted && (r.a | 0) === -1),
    (this.extended = this.mOneA),
    Zi.call(this, 'edwards', r),
    (this.a = new Ir(r.a, 16).umod(this.red.m)),
    (this.a = this.a.toRed(this.red)),
    (this.c = new Ir(r.c, 16).toRed(this.red)),
    (this.c2 = this.c.redSqr()),
    (this.d = new Ir(r.d, 16).toRed(this.red)),
    (this.dd = this.d.redAdd(this.d)),
    pp(!this.twisted || this.c.fromRed().cmpn(1) === 0),
    (this.oneC = (r.c | 0) === 1);
}
W0(lr, Zi);
var vp = lr;
lr.prototype._mulA = function (e) {
  return this.mOneA ? e.redNeg() : this.a.redMul(e);
};
lr.prototype._mulC = function (e) {
  return this.oneC ? e : this.c.redMul(e);
};
lr.prototype.jpoint = function (e, t, n, i) {
  return this.point(e, t, n, i);
};
lr.prototype.pointFromX = function (e, t) {
  (e = new Ir(e, 16)), e.red || (e = e.toRed(this.red));
  var n = e.redSqr(),
    i = this.c2.redSub(this.a.redMul(n)),
    a = this.one.redSub(this.c2.redMul(this.d).redMul(n)),
    c = i.redMul(a.redInvm()),
    v = c.redSqrt();
  if (v.redSqr().redSub(c).cmp(this.zero) !== 0) throw new Error('invalid point');
  var w = v.fromRed().isOdd();
  return ((t && !w) || (!t && w)) && (v = v.redNeg()), this.point(e, v);
};
lr.prototype.pointFromY = function (e, t) {
  (e = new Ir(e, 16)), e.red || (e = e.toRed(this.red));
  var n = e.redSqr(),
    i = n.redSub(this.c2),
    a = n.redMul(this.d).redMul(this.c2).redSub(this.a),
    c = i.redMul(a.redInvm());
  if (c.cmp(this.zero) === 0) {
    if (t) throw new Error('invalid point');
    return this.point(this.zero, e);
  }
  var v = c.redSqrt();
  if (v.redSqr().redSub(c).cmp(this.zero) !== 0) throw new Error('invalid point');
  return v.fromRed().isOdd() !== t && (v = v.redNeg()), this.point(v, e);
};
lr.prototype.validate = function (e) {
  if (e.isInfinity()) return !0;
  e.normalize();
  var t = e.x.redSqr(),
    n = e.y.redSqr(),
    i = t.redMul(this.a).redAdd(n),
    a = this.c2.redMul(this.one.redAdd(this.d.redMul(t).redMul(n)));
  return i.cmp(a) === 0;
};
function tt(r, e, t, n, i) {
  Zi.BasePoint.call(this, r, 'projective'),
    e === null && t === null && n === null
      ? ((this.x = this.curve.zero),
        (this.y = this.curve.one),
        (this.z = this.curve.one),
        (this.t = this.curve.zero),
        (this.zOne = !0))
      : ((this.x = new Ir(e, 16)),
        (this.y = new Ir(t, 16)),
        (this.z = n ? new Ir(n, 16) : this.curve.one),
        (this.t = i && new Ir(i, 16)),
        this.x.red || (this.x = this.x.toRed(this.curve.red)),
        this.y.red || (this.y = this.y.toRed(this.curve.red)),
        this.z.red || (this.z = this.z.toRed(this.curve.red)),
        this.t && !this.t.red && (this.t = this.t.toRed(this.curve.red)),
        (this.zOne = this.z === this.curve.one),
        this.curve.extended &&
          !this.t &&
          ((this.t = this.x.redMul(this.y)),
          this.zOne || (this.t = this.t.redMul(this.z.redInvm()))));
}
W0(tt, Zi.BasePoint);
lr.prototype.pointFromJSON = function (e) {
  return tt.fromJSON(this, e);
};
lr.prototype.point = function (e, t, n, i) {
  return new tt(this, e, t, n, i);
};
tt.fromJSON = function (e, t) {
  return new tt(e, t[0], t[1], t[2]);
};
tt.prototype.inspect = function () {
  return this.isInfinity()
    ? '<EC Point Infinity>'
    : '<EC Point x: ' +
        this.x.fromRed().toString(16, 2) +
        ' y: ' +
        this.y.fromRed().toString(16, 2) +
        ' z: ' +
        this.z.fromRed().toString(16, 2) +
        '>';
};
tt.prototype.isInfinity = function () {
  return (
    this.x.cmpn(0) === 0 &&
    (this.y.cmp(this.z) === 0 || (this.zOne && this.y.cmp(this.curve.c) === 0))
  );
};
tt.prototype._extDbl = function () {
  var e = this.x.redSqr(),
    t = this.y.redSqr(),
    n = this.z.redSqr();
  n = n.redIAdd(n);
  var i = this.curve._mulA(e),
    a = this.x.redAdd(this.y).redSqr().redISub(e).redISub(t),
    c = i.redAdd(t),
    v = c.redSub(n),
    w = i.redSub(t),
    y = a.redMul(v),
    M = c.redMul(w),
    S = a.redMul(w),
    I = v.redMul(c);
  return this.curve.point(y, M, I, S);
};
tt.prototype._projDbl = function () {
  var e = this.x.redAdd(this.y).redSqr(),
    t = this.x.redSqr(),
    n = this.y.redSqr(),
    i,
    a,
    c,
    v,
    w,
    y;
  if (this.curve.twisted) {
    v = this.curve._mulA(t);
    var M = v.redAdd(n);
    this.zOne
      ? ((i = e.redSub(t).redSub(n).redMul(M.redSub(this.curve.two))),
        (a = M.redMul(v.redSub(n))),
        (c = M.redSqr().redSub(M).redSub(M)))
      : ((w = this.z.redSqr()),
        (y = M.redSub(w).redISub(w)),
        (i = e.redSub(t).redISub(n).redMul(y)),
        (a = M.redMul(v.redSub(n))),
        (c = M.redMul(y)));
  } else
    (v = t.redAdd(n)),
      (w = this.curve._mulC(this.z).redSqr()),
      (y = v.redSub(w).redSub(w)),
      (i = this.curve._mulC(e.redISub(v)).redMul(y)),
      (a = this.curve._mulC(v).redMul(t.redISub(n))),
      (c = v.redMul(y));
  return this.curve.point(i, a, c);
};
tt.prototype.dbl = function () {
  return this.isInfinity() ? this : this.curve.extended ? this._extDbl() : this._projDbl();
};
tt.prototype._extAdd = function (e) {
  var t = this.y.redSub(this.x).redMul(e.y.redSub(e.x)),
    n = this.y.redAdd(this.x).redMul(e.y.redAdd(e.x)),
    i = this.t.redMul(this.curve.dd).redMul(e.t),
    a = this.z.redMul(e.z.redAdd(e.z)),
    c = n.redSub(t),
    v = a.redSub(i),
    w = a.redAdd(i),
    y = n.redAdd(t),
    M = c.redMul(v),
    S = w.redMul(y),
    I = c.redMul(y),
    E = v.redMul(w);
  return this.curve.point(M, S, E, I);
};
tt.prototype._projAdd = function (e) {
  var t = this.z.redMul(e.z),
    n = t.redSqr(),
    i = this.x.redMul(e.x),
    a = this.y.redMul(e.y),
    c = this.curve.d.redMul(i).redMul(a),
    v = n.redSub(c),
    w = n.redAdd(c),
    y = this.x.redAdd(this.y).redMul(e.x.redAdd(e.y)).redISub(i).redISub(a),
    M = t.redMul(v).redMul(y),
    S,
    I;
  return (
    this.curve.twisted
      ? ((S = t.redMul(w).redMul(a.redSub(this.curve._mulA(i)))), (I = v.redMul(w)))
      : ((S = t.redMul(w).redMul(a.redSub(i))), (I = this.curve._mulC(v).redMul(w))),
    this.curve.point(M, S, I)
  );
};
tt.prototype.add = function (e) {
  return this.isInfinity()
    ? e
    : e.isInfinity()
    ? this
    : this.curve.extended
    ? this._extAdd(e)
    : this._projAdd(e);
};
tt.prototype.mul = function (e) {
  return this._hasDoubles(e) ? this.curve._fixedNafMul(this, e) : this.curve._wnafMul(this, e);
};
tt.prototype.mulAdd = function (e, t, n) {
  return this.curve._wnafMulAdd(1, [this, t], [e, n], 2, !1);
};
tt.prototype.jmulAdd = function (e, t, n) {
  return this.curve._wnafMulAdd(1, [this, t], [e, n], 2, !0);
};
tt.prototype.normalize = function () {
  if (this.zOne) return this;
  var e = this.z.redInvm();
  return (
    (this.x = this.x.redMul(e)),
    (this.y = this.y.redMul(e)),
    this.t && (this.t = this.t.redMul(e)),
    (this.z = this.curve.one),
    (this.zOne = !0),
    this
  );
};
tt.prototype.neg = function () {
  return this.curve.point(this.x.redNeg(), this.y, this.z, this.t && this.t.redNeg());
};
tt.prototype.getX = function () {
  return this.normalize(), this.x.fromRed();
};
tt.prototype.getY = function () {
  return this.normalize(), this.y.fromRed();
};
tt.prototype.eq = function (e) {
  return this === e || (this.getX().cmp(e.getX()) === 0 && this.getY().cmp(e.getY()) === 0);
};
tt.prototype.eqXToP = function (e) {
  var t = e.toRed(this.curve.red).redMul(this.z);
  if (this.x.cmp(t) === 0) return !0;
  for (var n = e.clone(), i = this.curve.redN.redMul(this.z); ; ) {
    if ((n.iadd(this.curve.n), n.cmp(this.curve.p) >= 0)) return !1;
    if ((t.redIAdd(i), this.x.cmp(t) === 0)) return !0;
  }
};
tt.prototype.toP = tt.prototype.normalize;
tt.prototype.mixedAdd = tt.prototype.add;
(function (r) {
  var e = r;
  (e.base = Ki), (e.short = up), (e.mont = hp), (e.edwards = vp);
})(ss);
var Yi = {},
  _a,
  xo;
function bp() {
  return (
    xo ||
      ((xo = 1),
      (_a = {
        doubles: {
          step: 4,
          points: [
            [
              'e60fce93b59e9ec53011aabc21c23e97b2a31369b87a5ae9c44ee89e2a6dec0a',
              'f7e3507399e595929db99f34f57937101296891e44d23f0be1f32cce69616821',
            ],
            [
              '8282263212c609d9ea2a6e3e172de238d8c39cabd5ac1ca10646e23fd5f51508',
              '11f8a8098557dfe45e8256e830b60ace62d613ac2f7b17bed31b6eaff6e26caf',
            ],
            [
              '175e159f728b865a72f99cc6c6fc846de0b93833fd2222ed73fce5b551e5b739',
              'd3506e0d9e3c79eba4ef97a51ff71f5eacb5955add24345c6efa6ffee9fed695',
            ],
            [
              '363d90d447b00c9c99ceac05b6262ee053441c7e55552ffe526bad8f83ff4640',
              '4e273adfc732221953b445397f3363145b9a89008199ecb62003c7f3bee9de9',
            ],
            [
              '8b4b5f165df3c2be8c6244b5b745638843e4a781a15bcd1b69f79a55dffdf80c',
              '4aad0a6f68d308b4b3fbd7813ab0da04f9e336546162ee56b3eff0c65fd4fd36',
            ],
            [
              '723cbaa6e5db996d6bf771c00bd548c7b700dbffa6c0e77bcb6115925232fcda',
              '96e867b5595cc498a921137488824d6e2660a0653779494801dc069d9eb39f5f',
            ],
            [
              'eebfa4d493bebf98ba5feec812c2d3b50947961237a919839a533eca0e7dd7fa',
              '5d9a8ca3970ef0f269ee7edaf178089d9ae4cdc3a711f712ddfd4fdae1de8999',
            ],
            [
              '100f44da696e71672791d0a09b7bde459f1215a29b3c03bfefd7835b39a48db0',
              'cdd9e13192a00b772ec8f3300c090666b7ff4a18ff5195ac0fbd5cd62bc65a09',
            ],
            [
              'e1031be262c7ed1b1dc9227a4a04c017a77f8d4464f3b3852c8acde6e534fd2d',
              '9d7061928940405e6bb6a4176597535af292dd419e1ced79a44f18f29456a00d',
            ],
            [
              'feea6cae46d55b530ac2839f143bd7ec5cf8b266a41d6af52d5e688d9094696d',
              'e57c6b6c97dce1bab06e4e12bf3ecd5c981c8957cc41442d3155debf18090088',
            ],
            [
              'da67a91d91049cdcb367be4be6ffca3cfeed657d808583de33fa978bc1ec6cb1',
              '9bacaa35481642bc41f463f7ec9780e5dec7adc508f740a17e9ea8e27a68be1d',
            ],
            [
              '53904faa0b334cdda6e000935ef22151ec08d0f7bb11069f57545ccc1a37b7c0',
              '5bc087d0bc80106d88c9eccac20d3c1c13999981e14434699dcb096b022771c8',
            ],
            [
              '8e7bcd0bd35983a7719cca7764ca906779b53a043a9b8bcaeff959f43ad86047',
              '10b7770b2a3da4b3940310420ca9514579e88e2e47fd68b3ea10047e8460372a',
            ],
            [
              '385eed34c1cdff21e6d0818689b81bde71a7f4f18397e6690a841e1599c43862',
              '283bebc3e8ea23f56701de19e9ebf4576b304eec2086dc8cc0458fe5542e5453',
            ],
            [
              '6f9d9b803ecf191637c73a4413dfa180fddf84a5947fbc9c606ed86c3fac3a7',
              '7c80c68e603059ba69b8e2a30e45c4d47ea4dd2f5c281002d86890603a842160',
            ],
            [
              '3322d401243c4e2582a2147c104d6ecbf774d163db0f5e5313b7e0e742d0e6bd',
              '56e70797e9664ef5bfb019bc4ddaf9b72805f63ea2873af624f3a2e96c28b2a0',
            ],
            [
              '85672c7d2de0b7da2bd1770d89665868741b3f9af7643397721d74d28134ab83',
              '7c481b9b5b43b2eb6374049bfa62c2e5e77f17fcc5298f44c8e3094f790313a6',
            ],
            [
              '948bf809b1988a46b06c9f1919413b10f9226c60f668832ffd959af60c82a0a',
              '53a562856dcb6646dc6b74c5d1c3418c6d4dff08c97cd2bed4cb7f88d8c8e589',
            ],
            [
              '6260ce7f461801c34f067ce0f02873a8f1b0e44dfc69752accecd819f38fd8e8',
              'bc2da82b6fa5b571a7f09049776a1ef7ecd292238051c198c1a84e95b2b4ae17',
            ],
            [
              'e5037de0afc1d8d43d8348414bbf4103043ec8f575bfdc432953cc8d2037fa2d',
              '4571534baa94d3b5f9f98d09fb990bddbd5f5b03ec481f10e0e5dc841d755bda',
            ],
            [
              'e06372b0f4a207adf5ea905e8f1771b4e7e8dbd1c6a6c5b725866a0ae4fce725',
              '7a908974bce18cfe12a27bb2ad5a488cd7484a7787104870b27034f94eee31dd',
            ],
            [
              '213c7a715cd5d45358d0bbf9dc0ce02204b10bdde2a3f58540ad6908d0559754',
              '4b6dad0b5ae462507013ad06245ba190bb4850f5f36a7eeddff2c27534b458f2',
            ],
            [
              '4e7c272a7af4b34e8dbb9352a5419a87e2838c70adc62cddf0cc3a3b08fbd53c',
              '17749c766c9d0b18e16fd09f6def681b530b9614bff7dd33e0b3941817dcaae6',
            ],
            [
              'fea74e3dbe778b1b10f238ad61686aa5c76e3db2be43057632427e2840fb27b6',
              '6e0568db9b0b13297cf674deccb6af93126b596b973f7b77701d3db7f23cb96f',
            ],
            [
              '76e64113f677cf0e10a2570d599968d31544e179b760432952c02a4417bdde39',
              'c90ddf8dee4e95cf577066d70681f0d35e2a33d2b56d2032b4b1752d1901ac01',
            ],
            [
              'c738c56b03b2abe1e8281baa743f8f9a8f7cc643df26cbee3ab150242bcbb891',
              '893fb578951ad2537f718f2eacbfbbbb82314eef7880cfe917e735d9699a84c3',
            ],
            [
              'd895626548b65b81e264c7637c972877d1d72e5f3a925014372e9f6588f6c14b',
              'febfaa38f2bc7eae728ec60818c340eb03428d632bb067e179363ed75d7d991f',
            ],
            [
              'b8da94032a957518eb0f6433571e8761ceffc73693e84edd49150a564f676e03',
              '2804dfa44805a1e4d7c99cc9762808b092cc584d95ff3b511488e4e74efdf6e7',
            ],
            [
              'e80fea14441fb33a7d8adab9475d7fab2019effb5156a792f1a11778e3c0df5d',
              'eed1de7f638e00771e89768ca3ca94472d155e80af322ea9fcb4291b6ac9ec78',
            ],
            [
              'a301697bdfcd704313ba48e51d567543f2a182031efd6915ddc07bbcc4e16070',
              '7370f91cfb67e4f5081809fa25d40f9b1735dbf7c0a11a130c0d1a041e177ea1',
            ],
            [
              '90ad85b389d6b936463f9d0512678de208cc330b11307fffab7ac63e3fb04ed4',
              'e507a3620a38261affdcbd9427222b839aefabe1582894d991d4d48cb6ef150',
            ],
            [
              '8f68b9d2f63b5f339239c1ad981f162ee88c5678723ea3351b7b444c9ec4c0da',
              '662a9f2dba063986de1d90c2b6be215dbbea2cfe95510bfdf23cbf79501fff82',
            ],
            [
              'e4f3fb0176af85d65ff99ff9198c36091f48e86503681e3e6686fd5053231e11',
              '1e63633ad0ef4f1c1661a6d0ea02b7286cc7e74ec951d1c9822c38576feb73bc',
            ],
            [
              '8c00fa9b18ebf331eb961537a45a4266c7034f2f0d4e1d0716fb6eae20eae29e',
              'efa47267fea521a1a9dc343a3736c974c2fadafa81e36c54e7d2a4c66702414b',
            ],
            [
              'e7a26ce69dd4829f3e10cec0a9e98ed3143d084f308b92c0997fddfc60cb3e41',
              '2a758e300fa7984b471b006a1aafbb18d0a6b2c0420e83e20e8a9421cf2cfd51',
            ],
            [
              'b6459e0ee3662ec8d23540c223bcbdc571cbcb967d79424f3cf29eb3de6b80ef',
              '67c876d06f3e06de1dadf16e5661db3c4b3ae6d48e35b2ff30bf0b61a71ba45',
            ],
            [
              'd68a80c8280bb840793234aa118f06231d6f1fc67e73c5a5deda0f5b496943e8',
              'db8ba9fff4b586d00c4b1f9177b0e28b5b0e7b8f7845295a294c84266b133120',
            ],
            [
              '324aed7df65c804252dc0270907a30b09612aeb973449cea4095980fc28d3d5d',
              '648a365774b61f2ff130c0c35aec1f4f19213b0c7e332843967224af96ab7c84',
            ],
            [
              '4df9c14919cde61f6d51dfdbe5fee5dceec4143ba8d1ca888e8bd373fd054c96',
              '35ec51092d8728050974c23a1d85d4b5d506cdc288490192ebac06cad10d5d',
            ],
            [
              '9c3919a84a474870faed8a9c1cc66021523489054d7f0308cbfc99c8ac1f98cd',
              'ddb84f0f4a4ddd57584f044bf260e641905326f76c64c8e6be7e5e03d4fc599d',
            ],
            [
              '6057170b1dd12fdf8de05f281d8e06bb91e1493a8b91d4cc5a21382120a959e5',
              '9a1af0b26a6a4807add9a2daf71df262465152bc3ee24c65e899be932385a2a8',
            ],
            [
              'a576df8e23a08411421439a4518da31880cef0fba7d4df12b1a6973eecb94266',
              '40a6bf20e76640b2c92b97afe58cd82c432e10a7f514d9f3ee8be11ae1b28ec8',
            ],
            [
              '7778a78c28dec3e30a05fe9629de8c38bb30d1f5cf9a3a208f763889be58ad71',
              '34626d9ab5a5b22ff7098e12f2ff580087b38411ff24ac563b513fc1fd9f43ac',
            ],
            [
              '928955ee637a84463729fd30e7afd2ed5f96274e5ad7e5cb09eda9c06d903ac',
              'c25621003d3f42a827b78a13093a95eeac3d26efa8a8d83fc5180e935bcd091f',
            ],
            [
              '85d0fef3ec6db109399064f3a0e3b2855645b4a907ad354527aae75163d82751',
              '1f03648413a38c0be29d496e582cf5663e8751e96877331582c237a24eb1f962',
            ],
            [
              'ff2b0dce97eece97c1c9b6041798b85dfdfb6d8882da20308f5404824526087e',
              '493d13fef524ba188af4c4dc54d07936c7b7ed6fb90e2ceb2c951e01f0c29907',
            ],
            [
              '827fbbe4b1e880ea9ed2b2e6301b212b57f1ee148cd6dd28780e5e2cf856e241',
              'c60f9c923c727b0b71bef2c67d1d12687ff7a63186903166d605b68baec293ec',
            ],
            [
              'eaa649f21f51bdbae7be4ae34ce6e5217a58fdce7f47f9aa7f3b58fa2120e2b3',
              'be3279ed5bbbb03ac69a80f89879aa5a01a6b965f13f7e59d47a5305ba5ad93d',
            ],
            [
              'e4a42d43c5cf169d9391df6decf42ee541b6d8f0c9a137401e23632dda34d24f',
              '4d9f92e716d1c73526fc99ccfb8ad34ce886eedfa8d8e4f13a7f7131deba9414',
            ],
            [
              '1ec80fef360cbdd954160fadab352b6b92b53576a88fea4947173b9d4300bf19',
              'aeefe93756b5340d2f3a4958a7abbf5e0146e77f6295a07b671cdc1cc107cefd',
            ],
            [
              '146a778c04670c2f91b00af4680dfa8bce3490717d58ba889ddb5928366642be',
              'b318e0ec3354028add669827f9d4b2870aaa971d2f7e5ed1d0b297483d83efd0',
            ],
            [
              'fa50c0f61d22e5f07e3acebb1aa07b128d0012209a28b9776d76a8793180eef9',
              '6b84c6922397eba9b72cd2872281a68a5e683293a57a213b38cd8d7d3f4f2811',
            ],
            [
              'da1d61d0ca721a11b1a5bf6b7d88e8421a288ab5d5bba5220e53d32b5f067ec2',
              '8157f55a7c99306c79c0766161c91e2966a73899d279b48a655fba0f1ad836f1',
            ],
            [
              'a8e282ff0c9706907215ff98e8fd416615311de0446f1e062a73b0610d064e13',
              '7f97355b8db81c09abfb7f3c5b2515888b679a3e50dd6bd6cef7c73111f4cc0c',
            ],
            [
              '174a53b9c9a285872d39e56e6913cab15d59b1fa512508c022f382de8319497c',
              'ccc9dc37abfc9c1657b4155f2c47f9e6646b3a1d8cb9854383da13ac079afa73',
            ],
            [
              '959396981943785c3d3e57edf5018cdbe039e730e4918b3d884fdff09475b7ba',
              '2e7e552888c331dd8ba0386a4b9cd6849c653f64c8709385e9b8abf87524f2fd',
            ],
            [
              'd2a63a50ae401e56d645a1153b109a8fcca0a43d561fba2dbb51340c9d82b151',
              'e82d86fb6443fcb7565aee58b2948220a70f750af484ca52d4142174dcf89405',
            ],
            [
              '64587e2335471eb890ee7896d7cfdc866bacbdbd3839317b3436f9b45617e073',
              'd99fcdd5bf6902e2ae96dd6447c299a185b90a39133aeab358299e5e9faf6589',
            ],
            [
              '8481bde0e4e4d885b3a546d3e549de042f0aa6cea250e7fd358d6c86dd45e458',
              '38ee7b8cba5404dd84a25bf39cecb2ca900a79c42b262e556d64b1b59779057e',
            ],
            [
              '13464a57a78102aa62b6979ae817f4637ffcfed3c4b1ce30bcd6303f6caf666b',
              '69be159004614580ef7e433453ccb0ca48f300a81d0942e13f495a907f6ecc27',
            ],
            [
              'bc4a9df5b713fe2e9aef430bcc1dc97a0cd9ccede2f28588cada3a0d2d83f366',
              'd3a81ca6e785c06383937adf4b798caa6e8a9fbfa547b16d758d666581f33c1',
            ],
            [
              '8c28a97bf8298bc0d23d8c749452a32e694b65e30a9472a3954ab30fe5324caa',
              '40a30463a3305193378fedf31f7cc0eb7ae784f0451cb9459e71dc73cbef9482',
            ],
            [
              '8ea9666139527a8c1dd94ce4f071fd23c8b350c5a4bb33748c4ba111faccae0',
              '620efabbc8ee2782e24e7c0cfb95c5d735b783be9cf0f8e955af34a30e62b945',
            ],
            [
              'dd3625faef5ba06074669716bbd3788d89bdde815959968092f76cc4eb9a9787',
              '7a188fa3520e30d461da2501045731ca941461982883395937f68d00c644a573',
            ],
            [
              'f710d79d9eb962297e4f6232b40e8f7feb2bc63814614d692c12de752408221e',
              'ea98e67232d3b3295d3b535532115ccac8612c721851617526ae47a9c77bfc82',
            ],
          ],
        },
        naf: {
          wnd: 7,
          points: [
            [
              'f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9',
              '388f7b0f632de8140fe337e62a37f3566500a99934c2231b6cb9fd7584b8e672',
            ],
            [
              '2f8bde4d1a07209355b4a7250a5c5128e88b84bddc619ab7cba8d569b240efe4',
              'd8ac222636e5e3d6d4dba9dda6c9c426f788271bab0d6840dca87d3aa6ac62d6',
            ],
            [
              '5cbdf0646e5db4eaa398f365f2ea7a0e3d419b7e0330e39ce92bddedcac4f9bc',
              '6aebca40ba255960a3178d6d861a54dba813d0b813fde7b5a5082628087264da',
            ],
            [
              'acd484e2f0c7f65309ad178a9f559abde09796974c57e714c35f110dfc27ccbe',
              'cc338921b0a7d9fd64380971763b61e9add888a4375f8e0f05cc262ac64f9c37',
            ],
            [
              '774ae7f858a9411e5ef4246b70c65aac5649980be5c17891bbec17895da008cb',
              'd984a032eb6b5e190243dd56d7b7b365372db1e2dff9d6a8301d74c9c953c61b',
            ],
            [
              'f28773c2d975288bc7d1d205c3748651b075fbc6610e58cddeeddf8f19405aa8',
              'ab0902e8d880a89758212eb65cdaf473a1a06da521fa91f29b5cb52db03ed81',
            ],
            [
              'd7924d4f7d43ea965a465ae3095ff41131e5946f3c85f79e44adbcf8e27e080e',
              '581e2872a86c72a683842ec228cc6defea40af2bd896d3a5c504dc9ff6a26b58',
            ],
            [
              'defdea4cdb677750a420fee807eacf21eb9898ae79b9768766e4faa04a2d4a34',
              '4211ab0694635168e997b0ead2a93daeced1f4a04a95c0f6cfb199f69e56eb77',
            ],
            [
              '2b4ea0a797a443d293ef5cff444f4979f06acfebd7e86d277475656138385b6c',
              '85e89bc037945d93b343083b5a1c86131a01f60c50269763b570c854e5c09b7a',
            ],
            [
              '352bbf4a4cdd12564f93fa332ce333301d9ad40271f8107181340aef25be59d5',
              '321eb4075348f534d59c18259dda3e1f4a1b3b2e71b1039c67bd3d8bcf81998c',
            ],
            [
              '2fa2104d6b38d11b0230010559879124e42ab8dfeff5ff29dc9cdadd4ecacc3f',
              '2de1068295dd865b64569335bd5dd80181d70ecfc882648423ba76b532b7d67',
            ],
            [
              '9248279b09b4d68dab21a9b066edda83263c3d84e09572e269ca0cd7f5453714',
              '73016f7bf234aade5d1aa71bdea2b1ff3fc0de2a887912ffe54a32ce97cb3402',
            ],
            [
              'daed4f2be3a8bf278e70132fb0beb7522f570e144bf615c07e996d443dee8729',
              'a69dce4a7d6c98e8d4a1aca87ef8d7003f83c230f3afa726ab40e52290be1c55',
            ],
            [
              'c44d12c7065d812e8acf28d7cbb19f9011ecd9e9fdf281b0e6a3b5e87d22e7db',
              '2119a460ce326cdc76c45926c982fdac0e106e861edf61c5a039063f0e0e6482',
            ],
            [
              '6a245bf6dc698504c89a20cfded60853152b695336c28063b61c65cbd269e6b4',
              'e022cf42c2bd4a708b3f5126f16a24ad8b33ba48d0423b6efd5e6348100d8a82',
            ],
            [
              '1697ffa6fd9de627c077e3d2fe541084ce13300b0bec1146f95ae57f0d0bd6a5',
              'b9c398f186806f5d27561506e4557433a2cf15009e498ae7adee9d63d01b2396',
            ],
            [
              '605bdb019981718b986d0f07e834cb0d9deb8360ffb7f61df982345ef27a7479',
              '2972d2de4f8d20681a78d93ec96fe23c26bfae84fb14db43b01e1e9056b8c49',
            ],
            [
              '62d14dab4150bf497402fdc45a215e10dcb01c354959b10cfe31c7e9d87ff33d',
              '80fc06bd8cc5b01098088a1950eed0db01aa132967ab472235f5642483b25eaf',
            ],
            [
              '80c60ad0040f27dade5b4b06c408e56b2c50e9f56b9b8b425e555c2f86308b6f',
              '1c38303f1cc5c30f26e66bad7fe72f70a65eed4cbe7024eb1aa01f56430bd57a',
            ],
            [
              '7a9375ad6167ad54aa74c6348cc54d344cc5dc9487d847049d5eabb0fa03c8fb',
              'd0e3fa9eca8726909559e0d79269046bdc59ea10c70ce2b02d499ec224dc7f7',
            ],
            [
              'd528ecd9b696b54c907a9ed045447a79bb408ec39b68df504bb51f459bc3ffc9',
              'eecf41253136e5f99966f21881fd656ebc4345405c520dbc063465b521409933',
            ],
            [
              '49370a4b5f43412ea25f514e8ecdad05266115e4a7ecb1387231808f8b45963',
              '758f3f41afd6ed428b3081b0512fd62a54c3f3afbb5b6764b653052a12949c9a',
            ],
            [
              '77f230936ee88cbbd73df930d64702ef881d811e0e1498e2f1c13eb1fc345d74',
              '958ef42a7886b6400a08266e9ba1b37896c95330d97077cbbe8eb3c7671c60d6',
            ],
            [
              'f2dac991cc4ce4b9ea44887e5c7c0bce58c80074ab9d4dbaeb28531b7739f530',
              'e0dedc9b3b2f8dad4da1f32dec2531df9eb5fbeb0598e4fd1a117dba703a3c37',
            ],
            [
              '463b3d9f662621fb1b4be8fbbe2520125a216cdfc9dae3debcba4850c690d45b',
              '5ed430d78c296c3543114306dd8622d7c622e27c970a1de31cb377b01af7307e',
            ],
            [
              'f16f804244e46e2a09232d4aff3b59976b98fac14328a2d1a32496b49998f247',
              'cedabd9b82203f7e13d206fcdf4e33d92a6c53c26e5cce26d6579962c4e31df6',
            ],
            [
              'caf754272dc84563b0352b7a14311af55d245315ace27c65369e15f7151d41d1',
              'cb474660ef35f5f2a41b643fa5e460575f4fa9b7962232a5c32f908318a04476',
            ],
            [
              '2600ca4b282cb986f85d0f1709979d8b44a09c07cb86d7c124497bc86f082120',
              '4119b88753c15bd6a693b03fcddbb45d5ac6be74ab5f0ef44b0be9475a7e4b40',
            ],
            [
              '7635ca72d7e8432c338ec53cd12220bc01c48685e24f7dc8c602a7746998e435',
              '91b649609489d613d1d5e590f78e6d74ecfc061d57048bad9e76f302c5b9c61',
            ],
            [
              '754e3239f325570cdbbf4a87deee8a66b7f2b33479d468fbc1a50743bf56cc18',
              '673fb86e5bda30fb3cd0ed304ea49a023ee33d0197a695d0c5d98093c536683',
            ],
            [
              'e3e6bd1071a1e96aff57859c82d570f0330800661d1c952f9fe2694691d9b9e8',
              '59c9e0bba394e76f40c0aa58379a3cb6a5a2283993e90c4167002af4920e37f5',
            ],
            [
              '186b483d056a033826ae73d88f732985c4ccb1f32ba35f4b4cc47fdcf04aa6eb',
              '3b952d32c67cf77e2e17446e204180ab21fb8090895138b4a4a797f86e80888b',
            ],
            [
              'df9d70a6b9876ce544c98561f4be4f725442e6d2b737d9c91a8321724ce0963f',
              '55eb2dafd84d6ccd5f862b785dc39d4ab157222720ef9da217b8c45cf2ba2417',
            ],
            [
              '5edd5cc23c51e87a497ca815d5dce0f8ab52554f849ed8995de64c5f34ce7143',
              'efae9c8dbc14130661e8cec030c89ad0c13c66c0d17a2905cdc706ab7399a868',
            ],
            [
              '290798c2b6476830da12fe02287e9e777aa3fba1c355b17a722d362f84614fba',
              'e38da76dcd440621988d00bcf79af25d5b29c094db2a23146d003afd41943e7a',
            ],
            [
              'af3c423a95d9f5b3054754efa150ac39cd29552fe360257362dfdecef4053b45',
              'f98a3fd831eb2b749a93b0e6f35cfb40c8cd5aa667a15581bc2feded498fd9c6',
            ],
            [
              '766dbb24d134e745cccaa28c99bf274906bb66b26dcf98df8d2fed50d884249a',
              '744b1152eacbe5e38dcc887980da38b897584a65fa06cedd2c924f97cbac5996',
            ],
            [
              '59dbf46f8c94759ba21277c33784f41645f7b44f6c596a58ce92e666191abe3e',
              'c534ad44175fbc300f4ea6ce648309a042ce739a7919798cd85e216c4a307f6e',
            ],
            [
              'f13ada95103c4537305e691e74e9a4a8dd647e711a95e73cb62dc6018cfd87b8',
              'e13817b44ee14de663bf4bc808341f326949e21a6a75c2570778419bdaf5733d',
            ],
            [
              '7754b4fa0e8aced06d4167a2c59cca4cda1869c06ebadfb6488550015a88522c',
              '30e93e864e669d82224b967c3020b8fa8d1e4e350b6cbcc537a48b57841163a2',
            ],
            [
              '948dcadf5990e048aa3874d46abef9d701858f95de8041d2a6828c99e2262519',
              'e491a42537f6e597d5d28a3224b1bc25df9154efbd2ef1d2cbba2cae5347d57e',
            ],
            [
              '7962414450c76c1689c7b48f8202ec37fb224cf5ac0bfa1570328a8a3d7c77ab',
              '100b610ec4ffb4760d5c1fc133ef6f6b12507a051f04ac5760afa5b29db83437',
            ],
            [
              '3514087834964b54b15b160644d915485a16977225b8847bb0dd085137ec47ca',
              'ef0afbb2056205448e1652c48e8127fc6039e77c15c2378b7e7d15a0de293311',
            ],
            [
              'd3cc30ad6b483e4bc79ce2c9dd8bc54993e947eb8df787b442943d3f7b527eaf',
              '8b378a22d827278d89c5e9be8f9508ae3c2ad46290358630afb34db04eede0a4',
            ],
            [
              '1624d84780732860ce1c78fcbfefe08b2b29823db913f6493975ba0ff4847610',
              '68651cf9b6da903e0914448c6cd9d4ca896878f5282be4c8cc06e2a404078575',
            ],
            [
              '733ce80da955a8a26902c95633e62a985192474b5af207da6df7b4fd5fc61cd4',
              'f5435a2bd2badf7d485a4d8b8db9fcce3e1ef8e0201e4578c54673bc1dc5ea1d',
            ],
            [
              '15d9441254945064cf1a1c33bbd3b49f8966c5092171e699ef258dfab81c045c',
              'd56eb30b69463e7234f5137b73b84177434800bacebfc685fc37bbe9efe4070d',
            ],
            [
              'a1d0fcf2ec9de675b612136e5ce70d271c21417c9d2b8aaaac138599d0717940',
              'edd77f50bcb5a3cab2e90737309667f2641462a54070f3d519212d39c197a629',
            ],
            [
              'e22fbe15c0af8ccc5780c0735f84dbe9a790badee8245c06c7ca37331cb36980',
              'a855babad5cd60c88b430a69f53a1a7a38289154964799be43d06d77d31da06',
            ],
            [
              '311091dd9860e8e20ee13473c1155f5f69635e394704eaa74009452246cfa9b3',
              '66db656f87d1f04fffd1f04788c06830871ec5a64feee685bd80f0b1286d8374',
            ],
            [
              '34c1fd04d301be89b31c0442d3e6ac24883928b45a9340781867d4232ec2dbdf',
              '9414685e97b1b5954bd46f730174136d57f1ceeb487443dc5321857ba73abee',
            ],
            [
              'f219ea5d6b54701c1c14de5b557eb42a8d13f3abbcd08affcc2a5e6b049b8d63',
              '4cb95957e83d40b0f73af4544cccf6b1f4b08d3c07b27fb8d8c2962a400766d1',
            ],
            [
              'd7b8740f74a8fbaab1f683db8f45de26543a5490bca627087236912469a0b448',
              'fa77968128d9c92ee1010f337ad4717eff15db5ed3c049b3411e0315eaa4593b',
            ],
            [
              '32d31c222f8f6f0ef86f7c98d3a3335ead5bcd32abdd94289fe4d3091aa824bf',
              '5f3032f5892156e39ccd3d7915b9e1da2e6dac9e6f26e961118d14b8462e1661',
            ],
            [
              '7461f371914ab32671045a155d9831ea8793d77cd59592c4340f86cbc18347b5',
              '8ec0ba238b96bec0cbdddcae0aa442542eee1ff50c986ea6b39847b3cc092ff6',
            ],
            [
              'ee079adb1df1860074356a25aa38206a6d716b2c3e67453d287698bad7b2b2d6',
              '8dc2412aafe3be5c4c5f37e0ecc5f9f6a446989af04c4e25ebaac479ec1c8c1e',
            ],
            [
              '16ec93e447ec83f0467b18302ee620f7e65de331874c9dc72bfd8616ba9da6b5',
              '5e4631150e62fb40d0e8c2a7ca5804a39d58186a50e497139626778e25b0674d',
            ],
            [
              'eaa5f980c245f6f038978290afa70b6bd8855897f98b6aa485b96065d537bd99',
              'f65f5d3e292c2e0819a528391c994624d784869d7e6ea67fb18041024edc07dc',
            ],
            [
              '78c9407544ac132692ee1910a02439958ae04877151342ea96c4b6b35a49f51',
              'f3e0319169eb9b85d5404795539a5e68fa1fbd583c064d2462b675f194a3ddb4',
            ],
            [
              '494f4be219a1a77016dcd838431aea0001cdc8ae7a6fc688726578d9702857a5',
              '42242a969283a5f339ba7f075e36ba2af925ce30d767ed6e55f4b031880d562c',
            ],
            [
              'a598a8030da6d86c6bc7f2f5144ea549d28211ea58faa70ebf4c1e665c1fe9b5',
              '204b5d6f84822c307e4b4a7140737aec23fc63b65b35f86a10026dbd2d864e6b',
            ],
            [
              'c41916365abb2b5d09192f5f2dbeafec208f020f12570a184dbadc3e58595997',
              '4f14351d0087efa49d245b328984989d5caf9450f34bfc0ed16e96b58fa9913',
            ],
            [
              '841d6063a586fa475a724604da03bc5b92a2e0d2e0a36acfe4c73a5514742881',
              '73867f59c0659e81904f9a1c7543698e62562d6744c169ce7a36de01a8d6154',
            ],
            [
              '5e95bb399a6971d376026947f89bde2f282b33810928be4ded112ac4d70e20d5',
              '39f23f366809085beebfc71181313775a99c9aed7d8ba38b161384c746012865',
            ],
            [
              '36e4641a53948fd476c39f8a99fd974e5ec07564b5315d8bf99471bca0ef2f66',
              'd2424b1b1abe4eb8164227b085c9aa9456ea13493fd563e06fd51cf5694c78fc',
            ],
            [
              '336581ea7bfbbb290c191a2f507a41cf5643842170e914faeab27c2c579f726',
              'ead12168595fe1be99252129b6e56b3391f7ab1410cd1e0ef3dcdcabd2fda224',
            ],
            [
              '8ab89816dadfd6b6a1f2634fcf00ec8403781025ed6890c4849742706bd43ede',
              '6fdcef09f2f6d0a044e654aef624136f503d459c3e89845858a47a9129cdd24e',
            ],
            [
              '1e33f1a746c9c5778133344d9299fcaa20b0938e8acff2544bb40284b8c5fb94',
              '60660257dd11b3aa9c8ed618d24edff2306d320f1d03010e33a7d2057f3b3b6',
            ],
            [
              '85b7c1dcb3cec1b7ee7f30ded79dd20a0ed1f4cc18cbcfcfa410361fd8f08f31',
              '3d98a9cdd026dd43f39048f25a8847f4fcafad1895d7a633c6fed3c35e999511',
            ],
            [
              '29df9fbd8d9e46509275f4b125d6d45d7fbe9a3b878a7af872a2800661ac5f51',
              'b4c4fe99c775a606e2d8862179139ffda61dc861c019e55cd2876eb2a27d84b',
            ],
            [
              'a0b1cae06b0a847a3fea6e671aaf8adfdfe58ca2f768105c8082b2e449fce252',
              'ae434102edde0958ec4b19d917a6a28e6b72da1834aff0e650f049503a296cf2',
            ],
            [
              '4e8ceafb9b3e9a136dc7ff67e840295b499dfb3b2133e4ba113f2e4c0e121e5',
              'cf2174118c8b6d7a4b48f6d534ce5c79422c086a63460502b827ce62a326683c',
            ],
            [
              'd24a44e047e19b6f5afb81c7ca2f69080a5076689a010919f42725c2b789a33b',
              '6fb8d5591b466f8fc63db50f1c0f1c69013f996887b8244d2cdec417afea8fa3',
            ],
            [
              'ea01606a7a6c9cdd249fdfcfacb99584001edd28abbab77b5104e98e8e3b35d4',
              '322af4908c7312b0cfbfe369f7a7b3cdb7d4494bc2823700cfd652188a3ea98d',
            ],
            [
              'af8addbf2b661c8a6c6328655eb96651252007d8c5ea31be4ad196de8ce2131f',
              '6749e67c029b85f52a034eafd096836b2520818680e26ac8f3dfbcdb71749700',
            ],
            [
              'e3ae1974566ca06cc516d47e0fb165a674a3dabcfca15e722f0e3450f45889',
              '2aeabe7e4531510116217f07bf4d07300de97e4874f81f533420a72eeb0bd6a4',
            ],
            [
              '591ee355313d99721cf6993ffed1e3e301993ff3ed258802075ea8ced397e246',
              'b0ea558a113c30bea60fc4775460c7901ff0b053d25ca2bdeee98f1a4be5d196',
            ],
            [
              '11396d55fda54c49f19aa97318d8da61fa8584e47b084945077cf03255b52984',
              '998c74a8cd45ac01289d5833a7beb4744ff536b01b257be4c5767bea93ea57a4',
            ],
            [
              '3c5d2a1ba39c5a1790000738c9e0c40b8dcdfd5468754b6405540157e017aa7a',
              'b2284279995a34e2f9d4de7396fc18b80f9b8b9fdd270f6661f79ca4c81bd257',
            ],
            [
              'cc8704b8a60a0defa3a99a7299f2e9c3fbc395afb04ac078425ef8a1793cc030',
              'bdd46039feed17881d1e0862db347f8cf395b74fc4bcdc4e940b74e3ac1f1b13',
            ],
            [
              'c533e4f7ea8555aacd9777ac5cad29b97dd4defccc53ee7ea204119b2889b197',
              '6f0a256bc5efdf429a2fb6242f1a43a2d9b925bb4a4b3a26bb8e0f45eb596096',
            ],
            [
              'c14f8f2ccb27d6f109f6d08d03cc96a69ba8c34eec07bbcf566d48e33da6593',
              'c359d6923bb398f7fd4473e16fe1c28475b740dd098075e6c0e8649113dc3a38',
            ],
            [
              'a6cbc3046bc6a450bac24789fa17115a4c9739ed75f8f21ce441f72e0b90e6ef',
              '21ae7f4680e889bb130619e2c0f95a360ceb573c70603139862afd617fa9b9f',
            ],
            [
              '347d6d9a02c48927ebfb86c1359b1caf130a3c0267d11ce6344b39f99d43cc38',
              '60ea7f61a353524d1c987f6ecec92f086d565ab687870cb12689ff1e31c74448',
            ],
            [
              'da6545d2181db8d983f7dcb375ef5866d47c67b1bf31c8cf855ef7437b72656a',
              '49b96715ab6878a79e78f07ce5680c5d6673051b4935bd897fea824b77dc208a',
            ],
            [
              'c40747cc9d012cb1a13b8148309c6de7ec25d6945d657146b9d5994b8feb1111',
              '5ca560753be2a12fc6de6caf2cb489565db936156b9514e1bb5e83037e0fa2d4',
            ],
            [
              '4e42c8ec82c99798ccf3a610be870e78338c7f713348bd34c8203ef4037f3502',
              '7571d74ee5e0fb92a7a8b33a07783341a5492144cc54bcc40a94473693606437',
            ],
            [
              '3775ab7089bc6af823aba2e1af70b236d251cadb0c86743287522a1b3b0dedea',
              'be52d107bcfa09d8bcb9736a828cfa7fac8db17bf7a76a2c42ad961409018cf7',
            ],
            [
              'cee31cbf7e34ec379d94fb814d3d775ad954595d1314ba8846959e3e82f74e26',
              '8fd64a14c06b589c26b947ae2bcf6bfa0149ef0be14ed4d80f448a01c43b1c6d',
            ],
            [
              'b4f9eaea09b6917619f6ea6a4eb5464efddb58fd45b1ebefcdc1a01d08b47986',
              '39e5c9925b5a54b07433a4f18c61726f8bb131c012ca542eb24a8ac07200682a',
            ],
            [
              'd4263dfc3d2df923a0179a48966d30ce84e2515afc3dccc1b77907792ebcc60e',
              '62dfaf07a0f78feb30e30d6295853ce189e127760ad6cf7fae164e122a208d54',
            ],
            [
              '48457524820fa65a4f8d35eb6930857c0032acc0a4a2de422233eeda897612c4',
              '25a748ab367979d98733c38a1fa1c2e7dc6cc07db2d60a9ae7a76aaa49bd0f77',
            ],
            [
              'dfeeef1881101f2cb11644f3a2afdfc2045e19919152923f367a1767c11cceda',
              'ecfb7056cf1de042f9420bab396793c0c390bde74b4bbdff16a83ae09a9a7517',
            ],
            [
              '6d7ef6b17543f8373c573f44e1f389835d89bcbc6062ced36c82df83b8fae859',
              'cd450ec335438986dfefa10c57fea9bcc521a0959b2d80bbf74b190dca712d10',
            ],
            [
              'e75605d59102a5a2684500d3b991f2e3f3c88b93225547035af25af66e04541f',
              'f5c54754a8f71ee540b9b48728473e314f729ac5308b06938360990e2bfad125',
            ],
            [
              'eb98660f4c4dfaa06a2be453d5020bc99a0c2e60abe388457dd43fefb1ed620c',
              '6cb9a8876d9cb8520609af3add26cd20a0a7cd8a9411131ce85f44100099223e',
            ],
            [
              '13e87b027d8514d35939f2e6892b19922154596941888336dc3563e3b8dba942',
              'fef5a3c68059a6dec5d624114bf1e91aac2b9da568d6abeb2570d55646b8adf1',
            ],
            [
              'ee163026e9fd6fe017c38f06a5be6fc125424b371ce2708e7bf4491691e5764a',
              '1acb250f255dd61c43d94ccc670d0f58f49ae3fa15b96623e5430da0ad6c62b2',
            ],
            [
              'b268f5ef9ad51e4d78de3a750c2dc89b1e626d43505867999932e5db33af3d80',
              '5f310d4b3c99b9ebb19f77d41c1dee018cf0d34fd4191614003e945a1216e423',
            ],
            [
              'ff07f3118a9df035e9fad85eb6c7bfe42b02f01ca99ceea3bf7ffdba93c4750d',
              '438136d603e858a3a5c440c38eccbaddc1d2942114e2eddd4740d098ced1f0d8',
            ],
            [
              '8d8b9855c7c052a34146fd20ffb658bea4b9f69e0d825ebec16e8c3ce2b526a1',
              'cdb559eedc2d79f926baf44fb84ea4d44bcf50fee51d7ceb30e2e7f463036758',
            ],
            [
              '52db0b5384dfbf05bfa9d472d7ae26dfe4b851ceca91b1eba54263180da32b63',
              'c3b997d050ee5d423ebaf66a6db9f57b3180c902875679de924b69d84a7b375',
            ],
            [
              'e62f9490d3d51da6395efd24e80919cc7d0f29c3f3fa48c6fff543becbd43352',
              '6d89ad7ba4876b0b22c2ca280c682862f342c8591f1daf5170e07bfd9ccafa7d',
            ],
            [
              '7f30ea2476b399b4957509c88f77d0191afa2ff5cb7b14fd6d8e7d65aaab1193',
              'ca5ef7d4b231c94c3b15389a5f6311e9daff7bb67b103e9880ef4bff637acaec',
            ],
            [
              '5098ff1e1d9f14fb46a210fada6c903fef0fb7b4a1dd1d9ac60a0361800b7a00',
              '9731141d81fc8f8084d37c6e7542006b3ee1b40d60dfe5362a5b132fd17ddc0',
            ],
            [
              '32b78c7de9ee512a72895be6b9cbefa6e2f3c4ccce445c96b9f2c81e2778ad58',
              'ee1849f513df71e32efc3896ee28260c73bb80547ae2275ba497237794c8753c',
            ],
            [
              'e2cb74fddc8e9fbcd076eef2a7c72b0ce37d50f08269dfc074b581550547a4f7',
              'd3aa2ed71c9dd2247a62df062736eb0baddea9e36122d2be8641abcb005cc4a4',
            ],
            [
              '8438447566d4d7bedadc299496ab357426009a35f235cb141be0d99cd10ae3a8',
              'c4e1020916980a4da5d01ac5e6ad330734ef0d7906631c4f2390426b2edd791f',
            ],
            [
              '4162d488b89402039b584c6fc6c308870587d9c46f660b878ab65c82c711d67e',
              '67163e903236289f776f22c25fb8a3afc1732f2b84b4e95dbda47ae5a0852649',
            ],
            [
              '3fad3fa84caf0f34f0f89bfd2dcf54fc175d767aec3e50684f3ba4a4bf5f683d',
              'cd1bc7cb6cc407bb2f0ca647c718a730cf71872e7d0d2a53fa20efcdfe61826',
            ],
            [
              '674f2600a3007a00568c1a7ce05d0816c1fb84bf1370798f1c69532faeb1a86b',
              '299d21f9413f33b3edf43b257004580b70db57da0b182259e09eecc69e0d38a5',
            ],
            [
              'd32f4da54ade74abb81b815ad1fb3b263d82d6c692714bcff87d29bd5ee9f08f',
              'f9429e738b8e53b968e99016c059707782e14f4535359d582fc416910b3eea87',
            ],
            [
              '30e4e670435385556e593657135845d36fbb6931f72b08cb1ed954f1e3ce3ff6',
              '462f9bce619898638499350113bbc9b10a878d35da70740dc695a559eb88db7b',
            ],
            [
              'be2062003c51cc3004682904330e4dee7f3dcd10b01e580bf1971b04d4cad297',
              '62188bc49d61e5428573d48a74e1c655b1c61090905682a0d5558ed72dccb9bc',
            ],
            [
              '93144423ace3451ed29e0fb9ac2af211cb6e84a601df5993c419859fff5df04a',
              '7c10dfb164c3425f5c71a3f9d7992038f1065224f72bb9d1d902a6d13037b47c',
            ],
            [
              'b015f8044f5fcbdcf21ca26d6c34fb8197829205c7b7d2a7cb66418c157b112c',
              'ab8c1e086d04e813744a655b2df8d5f83b3cdc6faa3088c1d3aea1454e3a1d5f',
            ],
            [
              'd5e9e1da649d97d89e4868117a465a3a4f8a18de57a140d36b3f2af341a21b52',
              '4cb04437f391ed73111a13cc1d4dd0db1693465c2240480d8955e8592f27447a',
            ],
            [
              'd3ae41047dd7ca065dbf8ed77b992439983005cd72e16d6f996a5316d36966bb',
              'bd1aeb21ad22ebb22a10f0303417c6d964f8cdd7df0aca614b10dc14d125ac46',
            ],
            [
              '463e2763d885f958fc66cdd22800f0a487197d0a82e377b49f80af87c897b065',
              'bfefacdb0e5d0fd7df3a311a94de062b26b80c61fbc97508b79992671ef7ca7f',
            ],
            [
              '7985fdfd127c0567c6f53ec1bb63ec3158e597c40bfe747c83cddfc910641917',
              '603c12daf3d9862ef2b25fe1de289aed24ed291e0ec6708703a5bd567f32ed03',
            ],
            [
              '74a1ad6b5f76e39db2dd249410eac7f99e74c59cb83d2d0ed5ff1543da7703e9',
              'cc6157ef18c9c63cd6193d83631bbea0093e0968942e8c33d5737fd790e0db08',
            ],
            [
              '30682a50703375f602d416664ba19b7fc9bab42c72747463a71d0896b22f6da3',
              '553e04f6b018b4fa6c8f39e7f311d3176290d0e0f19ca73f17714d9977a22ff8',
            ],
            [
              '9e2158f0d7c0d5f26c3791efefa79597654e7a2b2464f52b1ee6c1347769ef57',
              '712fcdd1b9053f09003a3481fa7762e9ffd7c8ef35a38509e2fbf2629008373',
            ],
            [
              '176e26989a43c9cfeba4029c202538c28172e566e3c4fce7322857f3be327d66',
              'ed8cc9d04b29eb877d270b4878dc43c19aefd31f4eee09ee7b47834c1fa4b1c3',
            ],
            [
              '75d46efea3771e6e68abb89a13ad747ecf1892393dfc4f1b7004788c50374da8',
              '9852390a99507679fd0b86fd2b39a868d7efc22151346e1a3ca4726586a6bed8',
            ],
            [
              '809a20c67d64900ffb698c4c825f6d5f2310fb0451c869345b7319f645605721',
              '9e994980d9917e22b76b061927fa04143d096ccc54963e6a5ebfa5f3f8e286c1',
            ],
            [
              '1b38903a43f7f114ed4500b4eac7083fdefece1cf29c63528d563446f972c180',
              '4036edc931a60ae889353f77fd53de4a2708b26b6f5da72ad3394119daf408f9',
            ],
          ],
        },
      })),
    _a
  );
}
(function (r) {
  var e = r,
    t = $r,
    n = ss,
    i = jt,
    a = i.assert;
  function c(y) {
    y.type === 'short'
      ? (this.curve = new n.short(y))
      : y.type === 'edwards'
      ? (this.curve = new n.edwards(y))
      : (this.curve = new n.mont(y)),
      (this.g = this.curve.g),
      (this.n = this.curve.n),
      (this.hash = y.hash),
      a(this.g.validate(), 'Invalid curve'),
      a(this.g.mul(this.n).isInfinity(), 'Invalid curve, G*N != O');
  }
  e.PresetCurve = c;
  function v(y, M) {
    Object.defineProperty(e, y, {
      configurable: !0,
      enumerable: !0,
      get: function () {
        var S = new c(M);
        return Object.defineProperty(e, y, { configurable: !0, enumerable: !0, value: S }), S;
      },
    });
  }
  v('p192', {
    type: 'short',
    prime: 'p192',
    p: 'ffffffff ffffffff ffffffff fffffffe ffffffff ffffffff',
    a: 'ffffffff ffffffff ffffffff fffffffe ffffffff fffffffc',
    b: '64210519 e59c80e7 0fa7e9ab 72243049 feb8deec c146b9b1',
    n: 'ffffffff ffffffff ffffffff 99def836 146bc9b1 b4d22831',
    hash: t.sha256,
    gRed: !1,
    g: [
      '188da80e b03090f6 7cbf20eb 43a18800 f4ff0afd 82ff1012',
      '07192b95 ffc8da78 631011ed 6b24cdd5 73f977a1 1e794811',
    ],
  }),
    v('p224', {
      type: 'short',
      prime: 'p224',
      p: 'ffffffff ffffffff ffffffff ffffffff 00000000 00000000 00000001',
      a: 'ffffffff ffffffff ffffffff fffffffe ffffffff ffffffff fffffffe',
      b: 'b4050a85 0c04b3ab f5413256 5044b0b7 d7bfd8ba 270b3943 2355ffb4',
      n: 'ffffffff ffffffff ffffffff ffff16a2 e0b8f03e 13dd2945 5c5c2a3d',
      hash: t.sha256,
      gRed: !1,
      g: [
        'b70e0cbd 6bb4bf7f 321390b9 4a03c1d3 56c21122 343280d6 115c1d21',
        'bd376388 b5f723fb 4c22dfe6 cd4375a0 5a074764 44d58199 85007e34',
      ],
    }),
    v('p256', {
      type: 'short',
      prime: null,
      p: 'ffffffff 00000001 00000000 00000000 00000000 ffffffff ffffffff ffffffff',
      a: 'ffffffff 00000001 00000000 00000000 00000000 ffffffff ffffffff fffffffc',
      b: '5ac635d8 aa3a93e7 b3ebbd55 769886bc 651d06b0 cc53b0f6 3bce3c3e 27d2604b',
      n: 'ffffffff 00000000 ffffffff ffffffff bce6faad a7179e84 f3b9cac2 fc632551',
      hash: t.sha256,
      gRed: !1,
      g: [
        '6b17d1f2 e12c4247 f8bce6e5 63a440f2 77037d81 2deb33a0 f4a13945 d898c296',
        '4fe342e2 fe1a7f9b 8ee7eb4a 7c0f9e16 2bce3357 6b315ece cbb64068 37bf51f5',
      ],
    }),
    v('p384', {
      type: 'short',
      prime: null,
      p: 'ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff fffffffe ffffffff 00000000 00000000 ffffffff',
      a: 'ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff fffffffe ffffffff 00000000 00000000 fffffffc',
      b: 'b3312fa7 e23ee7e4 988e056b e3f82d19 181d9c6e fe814112 0314088f 5013875a c656398d 8a2ed19d 2a85c8ed d3ec2aef',
      n: 'ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff c7634d81 f4372ddf 581a0db2 48b0a77a ecec196a ccc52973',
      hash: t.sha384,
      gRed: !1,
      g: [
        'aa87ca22 be8b0537 8eb1c71e f320ad74 6e1d3b62 8ba79b98 59f741e0 82542a38 5502f25d bf55296c 3a545e38 72760ab7',
        '3617de4a 96262c6f 5d9e98bf 9292dc29 f8f41dbd 289a147c e9da3113 b5f0b8c0 0a60b1ce 1d7e819d 7a431d7c 90ea0e5f',
      ],
    }),
    v('p521', {
      type: 'short',
      prime: null,
      p: '000001ff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff',
      a: '000001ff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff fffffffc',
      b: '00000051 953eb961 8e1c9a1f 929a21a0 b68540ee a2da725b 99b315f3 b8b48991 8ef109e1 56193951 ec7e937b 1652c0bd 3bb1bf07 3573df88 3d2c34f1 ef451fd4 6b503f00',
      n: '000001ff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff fffffffa 51868783 bf2f966b 7fcc0148 f709a5d0 3bb5c9b8 899c47ae bb6fb71e 91386409',
      hash: t.sha512,
      gRed: !1,
      g: [
        '000000c6 858e06b7 0404e9cd 9e3ecb66 2395b442 9c648139 053fb521 f828af60 6b4d3dba a14b5e77 efe75928 fe1dc127 a2ffa8de 3348b3c1 856a429b f97e7e31 c2e5bd66',
        '00000118 39296a78 9a3bc004 5c8a5fb4 2c7d1bd9 98f54449 579b4468 17afbd17 273e662c 97ee7299 5ef42640 c550b901 3fad0761 353c7086 a272c240 88be9476 9fd16650',
      ],
    }),
    v('curve25519', {
      type: 'mont',
      prime: 'p25519',
      p: '7fffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffed',
      a: '76d06',
      b: '1',
      n: '1000000000000000 0000000000000000 14def9dea2f79cd6 5812631a5cf5d3ed',
      hash: t.sha256,
      gRed: !1,
      g: ['9'],
    }),
    v('ed25519', {
      type: 'edwards',
      prime: 'p25519',
      p: '7fffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffed',
      a: '-1',
      c: '1',
      d: '52036cee2b6ffe73 8cc740797779e898 00700a4d4141d8ab 75eb4dca135978a3',
      n: '1000000000000000 0000000000000000 14def9dea2f79cd6 5812631a5cf5d3ed',
      hash: t.sha256,
      gRed: !1,
      g: [
        '216936d3cd6e53fec0a4e231fdd6dc5c692cc7609525a7b2c9562d608f25d51a',
        '6666666666666666666666666666666666666666666666666666666666666658',
      ],
    });
  var w;
  try {
    w = bp();
  } catch {
    w = void 0;
  }
  v('secp256k1', {
    type: 'short',
    prime: 'k256',
    p: 'ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff fffffffe fffffc2f',
    a: '0',
    b: '7',
    n: 'ffffffff ffffffff ffffffff fffffffe baaedce6 af48a03b bfd25e8c d0364141',
    h: '1',
    hash: t.sha256,
    beta: '7ae96a2b657c07106e64479eac3434e99cf0497512f58995c1396c28719501ee',
    lambda: '5363ad4cc05c30e0a5261c028812645a122e22ea20816678df02967c1b23bd72',
    basis: [
      { a: '3086d221a7d46bcde86c90e49284eb15', b: '-e4437ed6010e88286f547fa90abfe4c3' },
      { a: '114ca50f7a8e2f3f657c1108d9d44cfd8', b: '3086d221a7d46bcde86c90e49284eb15' },
    ],
    gRed: !1,
    g: [
      '79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798',
      '483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8',
      w,
    ],
  });
})(Yi);
var mp = $r,
  Hr = as,
  G0 = Xr;
function Lr(r) {
  if (!(this instanceof Lr)) return new Lr(r);
  (this.hash = r.hash),
    (this.predResist = !!r.predResist),
    (this.outLen = this.hash.outSize),
    (this.minEntropy = r.minEntropy || this.hash.hmacStrength),
    (this._reseed = null),
    (this.reseedInterval = null),
    (this.K = null),
    (this.V = null);
  var e = Hr.toArray(r.entropy, r.entropyEnc || 'hex'),
    t = Hr.toArray(r.nonce, r.nonceEnc || 'hex'),
    n = Hr.toArray(r.pers, r.persEnc || 'hex');
  G0(
    e.length >= this.minEntropy / 8,
    'Not enough entropy. Minimum is: ' + this.minEntropy + ' bits'
  ),
    this._init(e, t, n);
}
var gp = Lr;
Lr.prototype._init = function (e, t, n) {
  var i = e.concat(t).concat(n);
  (this.K = new Array(this.outLen / 8)), (this.V = new Array(this.outLen / 8));
  for (var a = 0; a < this.V.length; a++) (this.K[a] = 0), (this.V[a] = 1);
  this._update(i), (this._reseed = 1), (this.reseedInterval = 281474976710656);
};
Lr.prototype._hmac = function () {
  return new mp.hmac(this.hash, this.K);
};
Lr.prototype._update = function (e) {
  var t = this._hmac().update(this.V).update([0]);
  e && (t = t.update(e)),
    (this.K = t.digest()),
    (this.V = this._hmac().update(this.V).digest()),
    e &&
      ((this.K = this._hmac().update(this.V).update([1]).update(e).digest()),
      (this.V = this._hmac().update(this.V).digest()));
};
Lr.prototype.reseed = function (e, t, n, i) {
  typeof t != 'string' && ((i = n), (n = t), (t = null)),
    (e = Hr.toArray(e, t)),
    (n = Hr.toArray(n, i)),
    G0(
      e.length >= this.minEntropy / 8,
      'Not enough entropy. Minimum is: ' + this.minEntropy + ' bits'
    ),
    this._update(e.concat(n || [])),
    (this._reseed = 1);
};
Lr.prototype.generate = function (e, t, n, i) {
  if (this._reseed > this.reseedInterval) throw new Error('Reseed is required');
  typeof t != 'string' && ((i = n), (n = t), (t = null)),
    n && ((n = Hr.toArray(n, i || 'hex')), this._update(n));
  for (var a = []; a.length < e; )
    (this.V = this._hmac().update(this.V).digest()), (a = a.concat(this.V));
  var c = a.slice(0, e);
  return this._update(n), this._reseed++, Hr.encode(c, t);
};
var yp = hr.exports,
  wp = jt,
  Ja = wp.assert;
function Ot(r, e) {
  (this.ec = r),
    (this.priv = null),
    (this.pub = null),
    e.priv && this._importPrivate(e.priv, e.privEnc),
    e.pub && this._importPublic(e.pub, e.pubEnc);
}
var xp = Ot;
Ot.fromPublic = function (e, t, n) {
  return t instanceof Ot ? t : new Ot(e, { pub: t, pubEnc: n });
};
Ot.fromPrivate = function (e, t, n) {
  return t instanceof Ot ? t : new Ot(e, { priv: t, privEnc: n });
};
Ot.prototype.validate = function () {
  var e = this.getPublic();
  return e.isInfinity()
    ? { result: !1, reason: 'Invalid public key' }
    : e.validate()
    ? e.mul(this.ec.curve.n).isInfinity()
      ? { result: !0, reason: null }
      : { result: !1, reason: 'Public key * N != O' }
    : { result: !1, reason: 'Public key is not a point' };
};
Ot.prototype.getPublic = function (e, t) {
  return (
    typeof e == 'string' && ((t = e), (e = null)),
    this.pub || (this.pub = this.ec.g.mul(this.priv)),
    t ? this.pub.encode(t, e) : this.pub
  );
};
Ot.prototype.getPrivate = function (e) {
  return e === 'hex' ? this.priv.toString(16, 2) : this.priv;
};
Ot.prototype._importPrivate = function (e, t) {
  (this.priv = new yp(e, t || 16)), (this.priv = this.priv.umod(this.ec.curve.n));
};
Ot.prototype._importPublic = function (e, t) {
  if (e.x || e.y) {
    this.ec.curve.type === 'mont'
      ? Ja(e.x, 'Need x coordinate')
      : (this.ec.curve.type === 'short' || this.ec.curve.type === 'edwards') &&
        Ja(e.x && e.y, 'Need both x and y coordinate'),
      (this.pub = this.ec.curve.point(e.x, e.y));
    return;
  }
  this.pub = this.ec.curve.decodePoint(e, t);
};
Ot.prototype.derive = function (e) {
  return e.validate() || Ja(e.validate(), 'public point not validated'), e.mul(this.priv).getX();
};
Ot.prototype.sign = function (e, t, n) {
  return this.ec.sign(e, this, t, n);
};
Ot.prototype.verify = function (e, t) {
  return this.ec.verify(e, t, this);
};
Ot.prototype.inspect = function () {
  return (
    '<Key priv: ' +
    (this.priv && this.priv.toString(16, 2)) +
    ' pub: ' +
    (this.pub && this.pub.inspect()) +
    ' >'
  );
};
var Li = hr.exports,
  fs = jt,
  Mp = fs.assert;
function Qi(r, e) {
  if (r instanceof Qi) return r;
  this._importDER(r, e) ||
    (Mp(r.r && r.s, 'Signature without r or s'),
    (this.r = new Li(r.r, 16)),
    (this.s = new Li(r.s, 16)),
    r.recoveryParam === void 0
      ? (this.recoveryParam = null)
      : (this.recoveryParam = r.recoveryParam));
}
var _p = Qi;
function Sp() {
  this.place = 0;
}
function Sa(r, e) {
  var t = r[e.place++];
  if (!(t & 128)) return t;
  var n = t & 15;
  if (n === 0 || n > 4) return !1;
  for (var i = 0, a = 0, c = e.place; a < n; a++, c++) (i <<= 8), (i |= r[c]), (i >>>= 0);
  return i <= 127 ? !1 : ((e.place = c), i);
}
function Mo(r) {
  for (var e = 0, t = r.length - 1; !r[e] && !(r[e + 1] & 128) && e < t; ) e++;
  return e === 0 ? r : r.slice(e);
}
Qi.prototype._importDER = function (e, t) {
  e = fs.toArray(e, t);
  var n = new Sp();
  if (e[n.place++] !== 48) return !1;
  var i = Sa(e, n);
  if (i === !1 || i + n.place !== e.length || e[n.place++] !== 2) return !1;
  var a = Sa(e, n);
  if (a === !1) return !1;
  var c = e.slice(n.place, a + n.place);
  if (((n.place += a), e[n.place++] !== 2)) return !1;
  var v = Sa(e, n);
  if (v === !1 || e.length !== v + n.place) return !1;
  var w = e.slice(n.place, v + n.place);
  if (c[0] === 0)
    if (c[1] & 128) c = c.slice(1);
    else return !1;
  if (w[0] === 0)
    if (w[1] & 128) w = w.slice(1);
    else return !1;
  return (this.r = new Li(c)), (this.s = new Li(w)), (this.recoveryParam = null), !0;
};
function Aa(r, e) {
  if (e < 128) {
    r.push(e);
    return;
  }
  var t = 1 + ((Math.log(e) / Math.LN2) >>> 3);
  for (r.push(t | 128); --t; ) r.push((e >>> (t << 3)) & 255);
  r.push(e);
}
Qi.prototype.toDER = function (e) {
  var t = this.r.toArray(),
    n = this.s.toArray();
  for (
    t[0] & 128 && (t = [0].concat(t)), n[0] & 128 && (n = [0].concat(n)), t = Mo(t), n = Mo(n);
    !n[0] && !(n[1] & 128);

  )
    n = n.slice(1);
  var i = [2];
  Aa(i, t.length), (i = i.concat(t)), i.push(2), Aa(i, n.length);
  var a = i.concat(n),
    c = [48];
  return Aa(c, a.length), (c = c.concat(a)), fs.encode(c, e);
};
var Wr = hr.exports,
  K0 = gp,
  Ap = jt,
  Ea = Yi,
  Ep = Gi.exports,
  X0 = Ap.assert,
  cs = xp,
  ea = _p;
function Wt(r) {
  if (!(this instanceof Wt)) return new Wt(r);
  typeof r == 'string' &&
    (X0(Object.prototype.hasOwnProperty.call(Ea, r), 'Unknown curve ' + r), (r = Ea[r])),
    r instanceof Ea.PresetCurve && (r = { curve: r }),
    (this.curve = r.curve.curve),
    (this.n = this.curve.n),
    (this.nh = this.n.ushrn(1)),
    (this.g = this.curve.g),
    (this.g = r.curve.g),
    this.g.precompute(r.curve.n.bitLength() + 1),
    (this.hash = r.hash || r.curve.hash);
}
var Ip = Wt;
Wt.prototype.keyPair = function (e) {
  return new cs(this, e);
};
Wt.prototype.keyFromPrivate = function (e, t) {
  return cs.fromPrivate(this, e, t);
};
Wt.prototype.keyFromPublic = function (e, t) {
  return cs.fromPublic(this, e, t);
};
Wt.prototype.genKeyPair = function (e) {
  e || (e = {});
  for (
    var t = new K0({
        hash: this.hash,
        pers: e.pers,
        persEnc: e.persEnc || 'utf8',
        entropy: e.entropy || Ep(this.hash.hmacStrength),
        entropyEnc: (e.entropy && e.entropyEnc) || 'utf8',
        nonce: this.n.toArray(),
      }),
      n = this.n.byteLength(),
      i = this.n.sub(new Wr(2));
    ;

  ) {
    var a = new Wr(t.generate(n));
    if (!(a.cmp(i) > 0)) return a.iaddn(1), this.keyFromPrivate(a);
  }
};
Wt.prototype._truncateToN = function (e, t) {
  var n = e.byteLength() * 8 - this.n.bitLength();
  return n > 0 && (e = e.ushrn(n)), !t && e.cmp(this.n) >= 0 ? e.sub(this.n) : e;
};
Wt.prototype.sign = function (e, t, n, i) {
  typeof n == 'object' && ((i = n), (n = null)),
    i || (i = {}),
    (t = this.keyFromPrivate(t, n)),
    (e = this._truncateToN(new Wr(e, 16)));
  for (
    var a = this.n.byteLength(),
      c = t.getPrivate().toArray('be', a),
      v = e.toArray('be', a),
      w = new K0({
        hash: this.hash,
        entropy: c,
        nonce: v,
        pers: i.pers,
        persEnc: i.persEnc || 'utf8',
      }),
      y = this.n.sub(new Wr(1)),
      M = 0;
    ;
    M++
  ) {
    var S = i.k ? i.k(M) : new Wr(w.generate(this.n.byteLength()));
    if (((S = this._truncateToN(S, !0)), !(S.cmpn(1) <= 0 || S.cmp(y) >= 0))) {
      var I = this.g.mul(S);
      if (!I.isInfinity()) {
        var E = I.getX(),
          R = E.umod(this.n);
        if (R.cmpn(0) !== 0) {
          var T = S.invm(this.n).mul(R.mul(t.getPrivate()).iadd(e));
          if (((T = T.umod(this.n)), T.cmpn(0) !== 0)) {
            var z = (I.getY().isOdd() ? 1 : 0) | (E.cmp(R) !== 0 ? 2 : 0);
            return (
              i.canonical && T.cmp(this.nh) > 0 && ((T = this.n.sub(T)), (z ^= 1)),
              new ea({ r: R, s: T, recoveryParam: z })
            );
          }
        }
      }
    }
  }
};
Wt.prototype.verify = function (e, t, n, i) {
  (e = this._truncateToN(new Wr(e, 16))), (n = this.keyFromPublic(n, i)), (t = new ea(t, 'hex'));
  var a = t.r,
    c = t.s;
  if (a.cmpn(1) < 0 || a.cmp(this.n) >= 0 || c.cmpn(1) < 0 || c.cmp(this.n) >= 0) return !1;
  var v = c.invm(this.n),
    w = v.mul(e).umod(this.n),
    y = v.mul(a).umod(this.n),
    M;
  return this.curve._maxwellTrick
    ? ((M = this.g.jmulAdd(w, n.getPublic(), y)), M.isInfinity() ? !1 : M.eqXToP(a))
    : ((M = this.g.mulAdd(w, n.getPublic(), y)),
      M.isInfinity() ? !1 : M.getX().umod(this.n).cmp(a) === 0);
};
Wt.prototype.recoverPubKey = function (r, e, t, n) {
  X0((3 & t) === t, 'The recovery param is more than two bits'), (e = new ea(e, n));
  var i = this.n,
    a = new Wr(r),
    c = e.r,
    v = e.s,
    w = t & 1,
    y = t >> 1;
  if (c.cmp(this.curve.p.umod(this.curve.n)) >= 0 && y)
    throw new Error('Unable to find sencond key candinate');
  y ? (c = this.curve.pointFromX(c.add(this.curve.n), w)) : (c = this.curve.pointFromX(c, w));
  var M = e.r.invm(i),
    S = i.sub(a).mul(M).umod(i),
    I = v.mul(M).umod(i);
  return this.g.mulAdd(S, c, I);
};
Wt.prototype.getKeyRecoveryParam = function (r, e, t, n) {
  if (((e = new ea(e, n)), e.recoveryParam !== null)) return e.recoveryParam;
  for (var i = 0; i < 4; i++) {
    var a;
    try {
      a = this.recoverPubKey(r, e, i);
    } catch {
      continue;
    }
    if (a.eq(t)) return i;
  }
  throw new Error('Unable to find valid recovery factor');
};
var ti = jt,
  Z0 = ti.assert,
  _o = ti.parseBytes,
  Nn = ti.cachedProperty;
function _t(r, e) {
  (this.eddsa = r),
    (this._secret = _o(e.secret)),
    r.isPoint(e.pub) ? (this._pub = e.pub) : (this._pubBytes = _o(e.pub));
}
_t.fromPublic = function (e, t) {
  return t instanceof _t ? t : new _t(e, { pub: t });
};
_t.fromSecret = function (e, t) {
  return t instanceof _t ? t : new _t(e, { secret: t });
};
_t.prototype.secret = function () {
  return this._secret;
};
Nn(_t, 'pubBytes', function () {
  return this.eddsa.encodePoint(this.pub());
});
Nn(_t, 'pub', function () {
  return this._pubBytes ? this.eddsa.decodePoint(this._pubBytes) : this.eddsa.g.mul(this.priv());
});
Nn(_t, 'privBytes', function () {
  var e = this.eddsa,
    t = this.hash(),
    n = e.encodingLength - 1,
    i = t.slice(0, e.encodingLength);
  return (i[0] &= 248), (i[n] &= 127), (i[n] |= 64), i;
});
Nn(_t, 'priv', function () {
  return this.eddsa.decodeInt(this.privBytes());
});
Nn(_t, 'hash', function () {
  return this.eddsa.hash().update(this.secret()).digest();
});
Nn(_t, 'messagePrefix', function () {
  return this.hash().slice(this.eddsa.encodingLength);
});
_t.prototype.sign = function (e) {
  return Z0(this._secret, 'KeyPair can only verify'), this.eddsa.sign(e, this);
};
_t.prototype.verify = function (e, t) {
  return this.eddsa.verify(e, t, this);
};
_t.prototype.getSecret = function (e) {
  return Z0(this._secret, 'KeyPair is public only'), ti.encode(this.secret(), e);
};
_t.prototype.getPublic = function (e) {
  return ti.encode(this.pubBytes(), e);
};
var Rp = _t,
  Np = hr.exports,
  ta = jt,
  Op = ta.assert,
  ra = ta.cachedProperty,
  Tp = ta.parseBytes;
function Zr(r, e) {
  (this.eddsa = r),
    typeof e != 'object' && (e = Tp(e)),
    Array.isArray(e) && (e = { R: e.slice(0, r.encodingLength), S: e.slice(r.encodingLength) }),
    Op(e.R && e.S, 'Signature without R or S'),
    r.isPoint(e.R) && (this._R = e.R),
    e.S instanceof Np && (this._S = e.S),
    (this._Rencoded = Array.isArray(e.R) ? e.R : e.Rencoded),
    (this._Sencoded = Array.isArray(e.S) ? e.S : e.Sencoded);
}
ra(Zr, 'S', function () {
  return this.eddsa.decodeInt(this.Sencoded());
});
ra(Zr, 'R', function () {
  return this.eddsa.decodePoint(this.Rencoded());
});
ra(Zr, 'Rencoded', function () {
  return this.eddsa.encodePoint(this.R());
});
ra(Zr, 'Sencoded', function () {
  return this.eddsa.encodeInt(this.S());
});
Zr.prototype.toBytes = function () {
  return this.Rencoded().concat(this.Sencoded());
};
Zr.prototype.toHex = function () {
  return ta.encode(this.toBytes(), 'hex').toUpperCase();
};
var Cp = Zr,
  Pp = $r,
  kp = Yi,
  wn = jt,
  $p = wn.assert,
  Y0 = wn.parseBytes,
  Q0 = Rp,
  So = Cp;
function qt(r) {
  if (($p(r === 'ed25519', 'only tested with ed25519 so far'), !(this instanceof qt)))
    return new qt(r);
  (r = kp[r].curve),
    (this.curve = r),
    (this.g = r.g),
    this.g.precompute(r.n.bitLength() + 1),
    (this.pointClass = r.point().constructor),
    (this.encodingLength = Math.ceil(r.n.bitLength() / 8)),
    (this.hash = Pp.sha512);
}
var Dp = qt;
qt.prototype.sign = function (e, t) {
  e = Y0(e);
  var n = this.keyFromSecret(t),
    i = this.hashInt(n.messagePrefix(), e),
    a = this.g.mul(i),
    c = this.encodePoint(a),
    v = this.hashInt(c, n.pubBytes(), e).mul(n.priv()),
    w = i.add(v).umod(this.curve.n);
  return this.makeSignature({ R: a, S: w, Rencoded: c });
};
qt.prototype.verify = function (e, t, n) {
  (e = Y0(e)), (t = this.makeSignature(t));
  var i = this.keyFromPublic(n),
    a = this.hashInt(t.Rencoded(), i.pubBytes(), e),
    c = this.g.mul(t.S()),
    v = t.R().add(i.pub().mul(a));
  return v.eq(c);
};
qt.prototype.hashInt = function () {
  for (var e = this.hash(), t = 0; t < arguments.length; t++) e.update(arguments[t]);
  return wn.intFromLE(e.digest()).umod(this.curve.n);
};
qt.prototype.keyFromPublic = function (e) {
  return Q0.fromPublic(this, e);
};
qt.prototype.keyFromSecret = function (e) {
  return Q0.fromSecret(this, e);
};
qt.prototype.makeSignature = function (e) {
  return e instanceof So ? e : new So(this, e);
};
qt.prototype.encodePoint = function (e) {
  var t = e.getY().toArray('le', this.encodingLength);
  return (t[this.encodingLength - 1] |= e.getX().isOdd() ? 128 : 0), t;
};
qt.prototype.decodePoint = function (e) {
  e = wn.parseBytes(e);
  var t = e.length - 1,
    n = e.slice(0, t).concat(e[t] & -129),
    i = (e[t] & 128) !== 0,
    a = wn.intFromLE(n);
  return this.curve.pointFromY(a, i);
};
qt.prototype.encodeInt = function (e) {
  return e.toArray('le', this.encodingLength);
};
qt.prototype.decodeInt = function (e) {
  return wn.intFromLE(e);
};
qt.prototype.isPoint = function (e) {
  return e instanceof this.pointClass;
};
(function (r) {
  var e = r;
  (e.version = sp.version),
    (e.utils = jt),
    (e.rand = Gi.exports),
    (e.curve = ss),
    (e.curves = Yi),
    (e.ec = Ip),
    (e.eddsa = Dp);
})(J0);
function en() {
  return new J0.ec('secp256k1');
}
var ef = class {
    constructor(r) {
      typeof r == 'string' && r.match(/^[0-9a-f]*$/i) && r.length === 64 && (r = `0x${r}`);
      let e = V(r),
        t = en().keyFromPrivate(e, 'hex');
      (this.compressedPublicKey = K(t.getPublic(!0, 'array'))),
        (this.publicKey = K(t.getPublic(!1, 'array').slice(1))),
        (this.privateKey = K(e)),
        (this.address = yt.fromPublicKey(this.publicKey));
    }
    sign(r) {
      let e = en().keyFromPrivate(V(this.privateKey), 'hex').sign(V(r), { canonical: !0 }),
        t = or(e.r, 32),
        n = or(e.s, 32);
      return (n[0] |= (e.recoveryParam || 0) << 7), K(se([t, n]));
    }
    addPoint(r) {
      let e = en().keyFromPublic(V(this.compressedPublicKey)),
        t = en().keyFromPublic(V(r)),
        n = e.getPublic().add(t.getPublic());
      return K(n.encode('array', !0));
    }
    static recoverPublicKey(r, e) {
      let t = V(e),
        n = t.slice(0, 32),
        i = t.slice(32, 64),
        a = (i[0] & 128) >> 7;
      return (
        (i[0] &= 127), en().recoverPubKey(V(r), { r: n, s: i }, a).encode('array', !1).slice(1)
      );
    }
    static recoverAddress(r, e) {
      return yt.fromPublicKey(ef.recoverPublicKey(r, e));
    }
    static generatePrivateKey(r) {
      return r ? Jl(se([Tr(32), V(r)])) : Tr(32);
    }
    static extendPublicKey(r) {
      let e = en().keyFromPublic(V(r));
      return K(e.getPublic(!1, 'array').slice(1));
    }
  },
  Gn = ef;
class tf {
  constructor(e) {
    fi(this, 'alphabet', e),
      fi(this, 'base', e.length),
      fi(this, '_alphabetMap', {}),
      fi(this, '_leader', e.charAt(0));
    for (let t = 0; t < e.length; t++) this._alphabetMap[e.charAt(t)] = t;
  }
  encode(e) {
    let t = V(e);
    if (t.length === 0) return '';
    let n = [0];
    for (let a = 0; a < t.length; ++a) {
      let c = t[a];
      for (let v = 0; v < n.length; ++v)
        (c += n[v] << 8), (n[v] = c % this.base), (c = (c / this.base) | 0);
      for (; c > 0; ) n.push(c % this.base), (c = (c / this.base) | 0);
    }
    let i = '';
    for (let a = 0; t[a] === 0 && a < t.length - 1; ++a) i += this._leader;
    for (let a = n.length - 1; a >= 0; --a) i += this.alphabet[n[a]];
    return i;
  }
  decode(e) {
    if (typeof e != 'string') throw new TypeError('Expected String');
    let t = [];
    if (e.length === 0) return new Uint8Array(t);
    t.push(0);
    for (let n = 0; n < e.length; n++) {
      let i = this._alphabetMap[e[n]];
      if (i === void 0) throw new Error('Non-base' + this.base + ' character');
      let a = i;
      for (let c = 0; c < t.length; ++c) (a += t[c] * this.base), (t[c] = a & 255), (a >>= 8);
      for (; a > 0; ) t.push(a & 255), (a >>= 8);
    }
    for (let n = 0; e[n] === this._leader && n < e.length - 1; ++n) t.push(0);
    return V(new Uint8Array(t.reverse()));
  }
}
new tf('abcdefghijklmnopqrstuvwxyz234567');
const us = new tf('123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz');
var Ia = [
    'abandon',
    'ability',
    'able',
    'about',
    'above',
    'absent',
    'absorb',
    'abstract',
    'absurd',
    'abuse',
    'access',
    'accident',
    'account',
    'accuse',
    'achieve',
    'acid',
    'acoustic',
    'acquire',
    'across',
    'act',
    'action',
    'actor',
    'actress',
    'actual',
    'adapt',
    'add',
    'addict',
    'address',
    'adjust',
    'admit',
    'adult',
    'advance',
    'advice',
    'aerobic',
    'affair',
    'afford',
    'afraid',
    'again',
    'age',
    'agent',
    'agree',
    'ahead',
    'aim',
    'air',
    'airport',
    'aisle',
    'alarm',
    'album',
    'alcohol',
    'alert',
    'alien',
    'all',
    'alley',
    'allow',
    'almost',
    'alone',
    'alpha',
    'already',
    'also',
    'alter',
    'always',
    'amateur',
    'amazing',
    'among',
    'amount',
    'amused',
    'analyst',
    'anchor',
    'ancient',
    'anger',
    'angle',
    'angry',
    'animal',
    'ankle',
    'announce',
    'annual',
    'another',
    'answer',
    'antenna',
    'antique',
    'anxiety',
    'any',
    'apart',
    'apology',
    'appear',
    'apple',
    'approve',
    'april',
    'arch',
    'arctic',
    'area',
    'arena',
    'argue',
    'arm',
    'armed',
    'armor',
    'army',
    'around',
    'arrange',
    'arrest',
    'arrive',
    'arrow',
    'art',
    'artefact',
    'artist',
    'artwork',
    'ask',
    'aspect',
    'assault',
    'asset',
    'assist',
    'assume',
    'asthma',
    'athlete',
    'atom',
    'attack',
    'attend',
    'attitude',
    'attract',
    'auction',
    'audit',
    'august',
    'aunt',
    'author',
    'auto',
    'autumn',
    'average',
    'avocado',
    'avoid',
    'awake',
    'aware',
    'away',
    'awesome',
    'awful',
    'awkward',
    'axis',
    'baby',
    'bachelor',
    'bacon',
    'badge',
    'bag',
    'balance',
    'balcony',
    'ball',
    'bamboo',
    'banana',
    'banner',
    'bar',
    'barely',
    'bargain',
    'barrel',
    'base',
    'basic',
    'basket',
    'battle',
    'beach',
    'bean',
    'beauty',
    'because',
    'become',
    'beef',
    'before',
    'begin',
    'behave',
    'behind',
    'believe',
    'below',
    'belt',
    'bench',
    'benefit',
    'best',
    'betray',
    'better',
    'between',
    'beyond',
    'bicycle',
    'bid',
    'bike',
    'bind',
    'biology',
    'bird',
    'birth',
    'bitter',
    'black',
    'blade',
    'blame',
    'blanket',
    'blast',
    'bleak',
    'bless',
    'blind',
    'blood',
    'blossom',
    'blouse',
    'blue',
    'blur',
    'blush',
    'board',
    'boat',
    'body',
    'boil',
    'bomb',
    'bone',
    'bonus',
    'book',
    'boost',
    'border',
    'boring',
    'borrow',
    'boss',
    'bottom',
    'bounce',
    'box',
    'boy',
    'bracket',
    'brain',
    'brand',
    'brass',
    'brave',
    'bread',
    'breeze',
    'brick',
    'bridge',
    'brief',
    'bright',
    'bring',
    'brisk',
    'broccoli',
    'broken',
    'bronze',
    'broom',
    'brother',
    'brown',
    'brush',
    'bubble',
    'buddy',
    'budget',
    'buffalo',
    'build',
    'bulb',
    'bulk',
    'bullet',
    'bundle',
    'bunker',
    'burden',
    'burger',
    'burst',
    'bus',
    'business',
    'busy',
    'butter',
    'buyer',
    'buzz',
    'cabbage',
    'cabin',
    'cable',
    'cactus',
    'cage',
    'cake',
    'call',
    'calm',
    'camera',
    'camp',
    'can',
    'canal',
    'cancel',
    'candy',
    'cannon',
    'canoe',
    'canvas',
    'canyon',
    'capable',
    'capital',
    'captain',
    'car',
    'carbon',
    'card',
    'cargo',
    'carpet',
    'carry',
    'cart',
    'case',
    'cash',
    'casino',
    'castle',
    'casual',
    'cat',
    'catalog',
    'catch',
    'category',
    'cattle',
    'caught',
    'cause',
    'caution',
    'cave',
    'ceiling',
    'celery',
    'cement',
    'census',
    'century',
    'cereal',
    'certain',
    'chair',
    'chalk',
    'champion',
    'change',
    'chaos',
    'chapter',
    'charge',
    'chase',
    'chat',
    'cheap',
    'check',
    'cheese',
    'chef',
    'cherry',
    'chest',
    'chicken',
    'chief',
    'child',
    'chimney',
    'choice',
    'choose',
    'chronic',
    'chuckle',
    'chunk',
    'churn',
    'cigar',
    'cinnamon',
    'circle',
    'citizen',
    'city',
    'civil',
    'claim',
    'clap',
    'clarify',
    'claw',
    'clay',
    'clean',
    'clerk',
    'clever',
    'click',
    'client',
    'cliff',
    'climb',
    'clinic',
    'clip',
    'clock',
    'clog',
    'close',
    'cloth',
    'cloud',
    'clown',
    'club',
    'clump',
    'cluster',
    'clutch',
    'coach',
    'coast',
    'coconut',
    'code',
    'coffee',
    'coil',
    'coin',
    'collect',
    'color',
    'column',
    'combine',
    'come',
    'comfort',
    'comic',
    'common',
    'company',
    'concert',
    'conduct',
    'confirm',
    'congress',
    'connect',
    'consider',
    'control',
    'convince',
    'cook',
    'cool',
    'copper',
    'copy',
    'coral',
    'core',
    'corn',
    'correct',
    'cost',
    'cotton',
    'couch',
    'country',
    'couple',
    'course',
    'cousin',
    'cover',
    'coyote',
    'crack',
    'cradle',
    'craft',
    'cram',
    'crane',
    'crash',
    'crater',
    'crawl',
    'crazy',
    'cream',
    'credit',
    'creek',
    'crew',
    'cricket',
    'crime',
    'crisp',
    'critic',
    'crop',
    'cross',
    'crouch',
    'crowd',
    'crucial',
    'cruel',
    'cruise',
    'crumble',
    'crunch',
    'crush',
    'cry',
    'crystal',
    'cube',
    'culture',
    'cup',
    'cupboard',
    'curious',
    'current',
    'curtain',
    'curve',
    'cushion',
    'custom',
    'cute',
    'cycle',
    'dad',
    'damage',
    'damp',
    'dance',
    'danger',
    'daring',
    'dash',
    'daughter',
    'dawn',
    'day',
    'deal',
    'debate',
    'debris',
    'decade',
    'december',
    'decide',
    'decline',
    'decorate',
    'decrease',
    'deer',
    'defense',
    'define',
    'defy',
    'degree',
    'delay',
    'deliver',
    'demand',
    'demise',
    'denial',
    'dentist',
    'deny',
    'depart',
    'depend',
    'deposit',
    'depth',
    'deputy',
    'derive',
    'describe',
    'desert',
    'design',
    'desk',
    'despair',
    'destroy',
    'detail',
    'detect',
    'develop',
    'device',
    'devote',
    'diagram',
    'dial',
    'diamond',
    'diary',
    'dice',
    'diesel',
    'diet',
    'differ',
    'digital',
    'dignity',
    'dilemma',
    'dinner',
    'dinosaur',
    'direct',
    'dirt',
    'disagree',
    'discover',
    'disease',
    'dish',
    'dismiss',
    'disorder',
    'display',
    'distance',
    'divert',
    'divide',
    'divorce',
    'dizzy',
    'doctor',
    'document',
    'dog',
    'doll',
    'dolphin',
    'domain',
    'donate',
    'donkey',
    'donor',
    'door',
    'dose',
    'double',
    'dove',
    'draft',
    'dragon',
    'drama',
    'drastic',
    'draw',
    'dream',
    'dress',
    'drift',
    'drill',
    'drink',
    'drip',
    'drive',
    'drop',
    'drum',
    'dry',
    'duck',
    'dumb',
    'dune',
    'during',
    'dust',
    'dutch',
    'duty',
    'dwarf',
    'dynamic',
    'eager',
    'eagle',
    'early',
    'earn',
    'earth',
    'easily',
    'east',
    'easy',
    'echo',
    'ecology',
    'economy',
    'edge',
    'edit',
    'educate',
    'effort',
    'egg',
    'eight',
    'either',
    'elbow',
    'elder',
    'electric',
    'elegant',
    'element',
    'elephant',
    'elevator',
    'elite',
    'else',
    'embark',
    'embody',
    'embrace',
    'emerge',
    'emotion',
    'employ',
    'empower',
    'empty',
    'enable',
    'enact',
    'end',
    'endless',
    'endorse',
    'enemy',
    'energy',
    'enforce',
    'engage',
    'engine',
    'enhance',
    'enjoy',
    'enlist',
    'enough',
    'enrich',
    'enroll',
    'ensure',
    'enter',
    'entire',
    'entry',
    'envelope',
    'episode',
    'equal',
    'equip',
    'era',
    'erase',
    'erode',
    'erosion',
    'error',
    'erupt',
    'escape',
    'essay',
    'essence',
    'estate',
    'eternal',
    'ethics',
    'evidence',
    'evil',
    'evoke',
    'evolve',
    'exact',
    'example',
    'excess',
    'exchange',
    'excite',
    'exclude',
    'excuse',
    'execute',
    'exercise',
    'exhaust',
    'exhibit',
    'exile',
    'exist',
    'exit',
    'exotic',
    'expand',
    'expect',
    'expire',
    'explain',
    'expose',
    'express',
    'extend',
    'extra',
    'eye',
    'eyebrow',
    'fabric',
    'face',
    'faculty',
    'fade',
    'faint',
    'faith',
    'fall',
    'false',
    'fame',
    'family',
    'famous',
    'fan',
    'fancy',
    'fantasy',
    'farm',
    'fashion',
    'fat',
    'fatal',
    'father',
    'fatigue',
    'fault',
    'favorite',
    'feature',
    'february',
    'federal',
    'fee',
    'feed',
    'feel',
    'female',
    'fence',
    'festival',
    'fetch',
    'fever',
    'few',
    'fiber',
    'fiction',
    'field',
    'figure',
    'file',
    'film',
    'filter',
    'final',
    'find',
    'fine',
    'finger',
    'finish',
    'fire',
    'firm',
    'first',
    'fiscal',
    'fish',
    'fit',
    'fitness',
    'fix',
    'flag',
    'flame',
    'flash',
    'flat',
    'flavor',
    'flee',
    'flight',
    'flip',
    'float',
    'flock',
    'floor',
    'flower',
    'fluid',
    'flush',
    'fly',
    'foam',
    'focus',
    'fog',
    'foil',
    'fold',
    'follow',
    'food',
    'foot',
    'force',
    'forest',
    'forget',
    'fork',
    'fortune',
    'forum',
    'forward',
    'fossil',
    'foster',
    'found',
    'fox',
    'fragile',
    'frame',
    'frequent',
    'fresh',
    'friend',
    'fringe',
    'frog',
    'front',
    'frost',
    'frown',
    'frozen',
    'fruit',
    'fuel',
    'fun',
    'funny',
    'furnace',
    'fury',
    'future',
    'gadget',
    'gain',
    'galaxy',
    'gallery',
    'game',
    'gap',
    'garage',
    'garbage',
    'garden',
    'garlic',
    'garment',
    'gas',
    'gasp',
    'gate',
    'gather',
    'gauge',
    'gaze',
    'general',
    'genius',
    'genre',
    'gentle',
    'genuine',
    'gesture',
    'ghost',
    'giant',
    'gift',
    'giggle',
    'ginger',
    'giraffe',
    'girl',
    'give',
    'glad',
    'glance',
    'glare',
    'glass',
    'glide',
    'glimpse',
    'globe',
    'gloom',
    'glory',
    'glove',
    'glow',
    'glue',
    'goat',
    'goddess',
    'gold',
    'good',
    'goose',
    'gorilla',
    'gospel',
    'gossip',
    'govern',
    'gown',
    'grab',
    'grace',
    'grain',
    'grant',
    'grape',
    'grass',
    'gravity',
    'great',
    'green',
    'grid',
    'grief',
    'grit',
    'grocery',
    'group',
    'grow',
    'grunt',
    'guard',
    'guess',
    'guide',
    'guilt',
    'guitar',
    'gun',
    'gym',
    'habit',
    'hair',
    'half',
    'hammer',
    'hamster',
    'hand',
    'happy',
    'harbor',
    'hard',
    'harsh',
    'harvest',
    'hat',
    'have',
    'hawk',
    'hazard',
    'head',
    'health',
    'heart',
    'heavy',
    'hedgehog',
    'height',
    'hello',
    'helmet',
    'help',
    'hen',
    'hero',
    'hidden',
    'high',
    'hill',
    'hint',
    'hip',
    'hire',
    'history',
    'hobby',
    'hockey',
    'hold',
    'hole',
    'holiday',
    'hollow',
    'home',
    'honey',
    'hood',
    'hope',
    'horn',
    'horror',
    'horse',
    'hospital',
    'host',
    'hotel',
    'hour',
    'hover',
    'hub',
    'huge',
    'human',
    'humble',
    'humor',
    'hundred',
    'hungry',
    'hunt',
    'hurdle',
    'hurry',
    'hurt',
    'husband',
    'hybrid',
    'ice',
    'icon',
    'idea',
    'identify',
    'idle',
    'ignore',
    'ill',
    'illegal',
    'illness',
    'image',
    'imitate',
    'immense',
    'immune',
    'impact',
    'impose',
    'improve',
    'impulse',
    'inch',
    'include',
    'income',
    'increase',
    'index',
    'indicate',
    'indoor',
    'industry',
    'infant',
    'inflict',
    'inform',
    'inhale',
    'inherit',
    'initial',
    'inject',
    'injury',
    'inmate',
    'inner',
    'innocent',
    'input',
    'inquiry',
    'insane',
    'insect',
    'inside',
    'inspire',
    'install',
    'intact',
    'interest',
    'into',
    'invest',
    'invite',
    'involve',
    'iron',
    'island',
    'isolate',
    'issue',
    'item',
    'ivory',
    'jacket',
    'jaguar',
    'jar',
    'jazz',
    'jealous',
    'jeans',
    'jelly',
    'jewel',
    'job',
    'join',
    'joke',
    'journey',
    'joy',
    'judge',
    'juice',
    'jump',
    'jungle',
    'junior',
    'junk',
    'just',
    'kangaroo',
    'keen',
    'keep',
    'ketchup',
    'key',
    'kick',
    'kid',
    'kidney',
    'kind',
    'kingdom',
    'kiss',
    'kit',
    'kitchen',
    'kite',
    'kitten',
    'kiwi',
    'knee',
    'knife',
    'knock',
    'know',
    'lab',
    'label',
    'labor',
    'ladder',
    'lady',
    'lake',
    'lamp',
    'language',
    'laptop',
    'large',
    'later',
    'latin',
    'laugh',
    'laundry',
    'lava',
    'law',
    'lawn',
    'lawsuit',
    'layer',
    'lazy',
    'leader',
    'leaf',
    'learn',
    'leave',
    'lecture',
    'left',
    'leg',
    'legal',
    'legend',
    'leisure',
    'lemon',
    'lend',
    'length',
    'lens',
    'leopard',
    'lesson',
    'letter',
    'level',
    'liar',
    'liberty',
    'library',
    'license',
    'life',
    'lift',
    'light',
    'like',
    'limb',
    'limit',
    'link',
    'lion',
    'liquid',
    'list',
    'little',
    'live',
    'lizard',
    'load',
    'loan',
    'lobster',
    'local',
    'lock',
    'logic',
    'lonely',
    'long',
    'loop',
    'lottery',
    'loud',
    'lounge',
    'love',
    'loyal',
    'lucky',
    'luggage',
    'lumber',
    'lunar',
    'lunch',
    'luxury',
    'lyrics',
    'machine',
    'mad',
    'magic',
    'magnet',
    'maid',
    'mail',
    'main',
    'major',
    'make',
    'mammal',
    'man',
    'manage',
    'mandate',
    'mango',
    'mansion',
    'manual',
    'maple',
    'marble',
    'march',
    'margin',
    'marine',
    'market',
    'marriage',
    'mask',
    'mass',
    'master',
    'match',
    'material',
    'math',
    'matrix',
    'matter',
    'maximum',
    'maze',
    'meadow',
    'mean',
    'measure',
    'meat',
    'mechanic',
    'medal',
    'media',
    'melody',
    'melt',
    'member',
    'memory',
    'mention',
    'menu',
    'mercy',
    'merge',
    'merit',
    'merry',
    'mesh',
    'message',
    'metal',
    'method',
    'middle',
    'midnight',
    'milk',
    'million',
    'mimic',
    'mind',
    'minimum',
    'minor',
    'minute',
    'miracle',
    'mirror',
    'misery',
    'miss',
    'mistake',
    'mix',
    'mixed',
    'mixture',
    'mobile',
    'model',
    'modify',
    'mom',
    'moment',
    'monitor',
    'monkey',
    'monster',
    'month',
    'moon',
    'moral',
    'more',
    'morning',
    'mosquito',
    'mother',
    'motion',
    'motor',
    'mountain',
    'mouse',
    'move',
    'movie',
    'much',
    'muffin',
    'mule',
    'multiply',
    'muscle',
    'museum',
    'mushroom',
    'music',
    'must',
    'mutual',
    'myself',
    'mystery',
    'myth',
    'naive',
    'name',
    'napkin',
    'narrow',
    'nasty',
    'nation',
    'nature',
    'near',
    'neck',
    'need',
    'negative',
    'neglect',
    'neither',
    'nephew',
    'nerve',
    'nest',
    'net',
    'network',
    'neutral',
    'never',
    'news',
    'next',
    'nice',
    'night',
    'noble',
    'noise',
    'nominee',
    'noodle',
    'normal',
    'north',
    'nose',
    'notable',
    'note',
    'nothing',
    'notice',
    'novel',
    'now',
    'nuclear',
    'number',
    'nurse',
    'nut',
    'oak',
    'obey',
    'object',
    'oblige',
    'obscure',
    'observe',
    'obtain',
    'obvious',
    'occur',
    'ocean',
    'october',
    'odor',
    'off',
    'offer',
    'office',
    'often',
    'oil',
    'okay',
    'old',
    'olive',
    'olympic',
    'omit',
    'once',
    'one',
    'onion',
    'online',
    'only',
    'open',
    'opera',
    'opinion',
    'oppose',
    'option',
    'orange',
    'orbit',
    'orchard',
    'order',
    'ordinary',
    'organ',
    'orient',
    'original',
    'orphan',
    'ostrich',
    'other',
    'outdoor',
    'outer',
    'output',
    'outside',
    'oval',
    'oven',
    'over',
    'own',
    'owner',
    'oxygen',
    'oyster',
    'ozone',
    'pact',
    'paddle',
    'page',
    'pair',
    'palace',
    'palm',
    'panda',
    'panel',
    'panic',
    'panther',
    'paper',
    'parade',
    'parent',
    'park',
    'parrot',
    'party',
    'pass',
    'patch',
    'path',
    'patient',
    'patrol',
    'pattern',
    'pause',
    'pave',
    'payment',
    'peace',
    'peanut',
    'pear',
    'peasant',
    'pelican',
    'pen',
    'penalty',
    'pencil',
    'people',
    'pepper',
    'perfect',
    'permit',
    'person',
    'pet',
    'phone',
    'photo',
    'phrase',
    'physical',
    'piano',
    'picnic',
    'picture',
    'piece',
    'pig',
    'pigeon',
    'pill',
    'pilot',
    'pink',
    'pioneer',
    'pipe',
    'pistol',
    'pitch',
    'pizza',
    'place',
    'planet',
    'plastic',
    'plate',
    'play',
    'please',
    'pledge',
    'pluck',
    'plug',
    'plunge',
    'poem',
    'poet',
    'point',
    'polar',
    'pole',
    'police',
    'pond',
    'pony',
    'pool',
    'popular',
    'portion',
    'position',
    'possible',
    'post',
    'potato',
    'pottery',
    'poverty',
    'powder',
    'power',
    'practice',
    'praise',
    'predict',
    'prefer',
    'prepare',
    'present',
    'pretty',
    'prevent',
    'price',
    'pride',
    'primary',
    'print',
    'priority',
    'prison',
    'private',
    'prize',
    'problem',
    'process',
    'produce',
    'profit',
    'program',
    'project',
    'promote',
    'proof',
    'property',
    'prosper',
    'protect',
    'proud',
    'provide',
    'public',
    'pudding',
    'pull',
    'pulp',
    'pulse',
    'pumpkin',
    'punch',
    'pupil',
    'puppy',
    'purchase',
    'purity',
    'purpose',
    'purse',
    'push',
    'put',
    'puzzle',
    'pyramid',
    'quality',
    'quantum',
    'quarter',
    'question',
    'quick',
    'quit',
    'quiz',
    'quote',
    'rabbit',
    'raccoon',
    'race',
    'rack',
    'radar',
    'radio',
    'rail',
    'rain',
    'raise',
    'rally',
    'ramp',
    'ranch',
    'random',
    'range',
    'rapid',
    'rare',
    'rate',
    'rather',
    'raven',
    'raw',
    'razor',
    'ready',
    'real',
    'reason',
    'rebel',
    'rebuild',
    'recall',
    'receive',
    'recipe',
    'record',
    'recycle',
    'reduce',
    'reflect',
    'reform',
    'refuse',
    'region',
    'regret',
    'regular',
    'reject',
    'relax',
    'release',
    'relief',
    'rely',
    'remain',
    'remember',
    'remind',
    'remove',
    'render',
    'renew',
    'rent',
    'reopen',
    'repair',
    'repeat',
    'replace',
    'report',
    'require',
    'rescue',
    'resemble',
    'resist',
    'resource',
    'response',
    'result',
    'retire',
    'retreat',
    'return',
    'reunion',
    'reveal',
    'review',
    'reward',
    'rhythm',
    'rib',
    'ribbon',
    'rice',
    'rich',
    'ride',
    'ridge',
    'rifle',
    'right',
    'rigid',
    'ring',
    'riot',
    'ripple',
    'risk',
    'ritual',
    'rival',
    'river',
    'road',
    'roast',
    'robot',
    'robust',
    'rocket',
    'romance',
    'roof',
    'rookie',
    'room',
    'rose',
    'rotate',
    'rough',
    'round',
    'route',
    'royal',
    'rubber',
    'rude',
    'rug',
    'rule',
    'run',
    'runway',
    'rural',
    'sad',
    'saddle',
    'sadness',
    'safe',
    'sail',
    'salad',
    'salmon',
    'salon',
    'salt',
    'salute',
    'same',
    'sample',
    'sand',
    'satisfy',
    'satoshi',
    'sauce',
    'sausage',
    'save',
    'say',
    'scale',
    'scan',
    'scare',
    'scatter',
    'scene',
    'scheme',
    'school',
    'science',
    'scissors',
    'scorpion',
    'scout',
    'scrap',
    'screen',
    'script',
    'scrub',
    'sea',
    'search',
    'season',
    'seat',
    'second',
    'secret',
    'section',
    'security',
    'seed',
    'seek',
    'segment',
    'select',
    'sell',
    'seminar',
    'senior',
    'sense',
    'sentence',
    'series',
    'service',
    'session',
    'settle',
    'setup',
    'seven',
    'shadow',
    'shaft',
    'shallow',
    'share',
    'shed',
    'shell',
    'sheriff',
    'shield',
    'shift',
    'shine',
    'ship',
    'shiver',
    'shock',
    'shoe',
    'shoot',
    'shop',
    'short',
    'shoulder',
    'shove',
    'shrimp',
    'shrug',
    'shuffle',
    'shy',
    'sibling',
    'sick',
    'side',
    'siege',
    'sight',
    'sign',
    'silent',
    'silk',
    'silly',
    'silver',
    'similar',
    'simple',
    'since',
    'sing',
    'siren',
    'sister',
    'situate',
    'six',
    'size',
    'skate',
    'sketch',
    'ski',
    'skill',
    'skin',
    'skirt',
    'skull',
    'slab',
    'slam',
    'sleep',
    'slender',
    'slice',
    'slide',
    'slight',
    'slim',
    'slogan',
    'slot',
    'slow',
    'slush',
    'small',
    'smart',
    'smile',
    'smoke',
    'smooth',
    'snack',
    'snake',
    'snap',
    'sniff',
    'snow',
    'soap',
    'soccer',
    'social',
    'sock',
    'soda',
    'soft',
    'solar',
    'soldier',
    'solid',
    'solution',
    'solve',
    'someone',
    'song',
    'soon',
    'sorry',
    'sort',
    'soul',
    'sound',
    'soup',
    'source',
    'south',
    'space',
    'spare',
    'spatial',
    'spawn',
    'speak',
    'special',
    'speed',
    'spell',
    'spend',
    'sphere',
    'spice',
    'spider',
    'spike',
    'spin',
    'spirit',
    'split',
    'spoil',
    'sponsor',
    'spoon',
    'sport',
    'spot',
    'spray',
    'spread',
    'spring',
    'spy',
    'square',
    'squeeze',
    'squirrel',
    'stable',
    'stadium',
    'staff',
    'stage',
    'stairs',
    'stamp',
    'stand',
    'start',
    'state',
    'stay',
    'steak',
    'steel',
    'stem',
    'step',
    'stereo',
    'stick',
    'still',
    'sting',
    'stock',
    'stomach',
    'stone',
    'stool',
    'story',
    'stove',
    'strategy',
    'street',
    'strike',
    'strong',
    'struggle',
    'student',
    'stuff',
    'stumble',
    'style',
    'subject',
    'submit',
    'subway',
    'success',
    'such',
    'sudden',
    'suffer',
    'sugar',
    'suggest',
    'suit',
    'summer',
    'sun',
    'sunny',
    'sunset',
    'super',
    'supply',
    'supreme',
    'sure',
    'surface',
    'surge',
    'surprise',
    'surround',
    'survey',
    'suspect',
    'sustain',
    'swallow',
    'swamp',
    'swap',
    'swarm',
    'swear',
    'sweet',
    'swift',
    'swim',
    'swing',
    'switch',
    'sword',
    'symbol',
    'symptom',
    'syrup',
    'system',
    'table',
    'tackle',
    'tag',
    'tail',
    'talent',
    'talk',
    'tank',
    'tape',
    'target',
    'task',
    'taste',
    'tattoo',
    'taxi',
    'teach',
    'team',
    'tell',
    'ten',
    'tenant',
    'tennis',
    'tent',
    'term',
    'test',
    'text',
    'thank',
    'that',
    'theme',
    'then',
    'theory',
    'there',
    'they',
    'thing',
    'this',
    'thought',
    'three',
    'thrive',
    'throw',
    'thumb',
    'thunder',
    'ticket',
    'tide',
    'tiger',
    'tilt',
    'timber',
    'time',
    'tiny',
    'tip',
    'tired',
    'tissue',
    'title',
    'toast',
    'tobacco',
    'today',
    'toddler',
    'toe',
    'together',
    'toilet',
    'token',
    'tomato',
    'tomorrow',
    'tone',
    'tongue',
    'tonight',
    'tool',
    'tooth',
    'top',
    'topic',
    'topple',
    'torch',
    'tornado',
    'tortoise',
    'toss',
    'total',
    'tourist',
    'toward',
    'tower',
    'town',
    'toy',
    'track',
    'trade',
    'traffic',
    'tragic',
    'train',
    'transfer',
    'trap',
    'trash',
    'travel',
    'tray',
    'treat',
    'tree',
    'trend',
    'trial',
    'tribe',
    'trick',
    'trigger',
    'trim',
    'trip',
    'trophy',
    'trouble',
    'truck',
    'true',
    'truly',
    'trumpet',
    'trust',
    'truth',
    'try',
    'tube',
    'tuition',
    'tumble',
    'tuna',
    'tunnel',
    'turkey',
    'turn',
    'turtle',
    'twelve',
    'twenty',
    'twice',
    'twin',
    'twist',
    'two',
    'type',
    'typical',
    'ugly',
    'umbrella',
    'unable',
    'unaware',
    'uncle',
    'uncover',
    'under',
    'undo',
    'unfair',
    'unfold',
    'unhappy',
    'uniform',
    'unique',
    'unit',
    'universe',
    'unknown',
    'unlock',
    'until',
    'unusual',
    'unveil',
    'update',
    'upgrade',
    'uphold',
    'upon',
    'upper',
    'upset',
    'urban',
    'urge',
    'usage',
    'use',
    'used',
    'useful',
    'useless',
    'usual',
    'utility',
    'vacant',
    'vacuum',
    'vague',
    'valid',
    'valley',
    'valve',
    'van',
    'vanish',
    'vapor',
    'various',
    'vast',
    'vault',
    'vehicle',
    'velvet',
    'vendor',
    'venture',
    'venue',
    'verb',
    'verify',
    'version',
    'very',
    'vessel',
    'veteran',
    'viable',
    'vibrant',
    'vicious',
    'victory',
    'video',
    'view',
    'village',
    'vintage',
    'violin',
    'virtual',
    'virus',
    'visa',
    'visit',
    'visual',
    'vital',
    'vivid',
    'vocal',
    'voice',
    'void',
    'volcano',
    'volume',
    'vote',
    'voyage',
    'wage',
    'wagon',
    'wait',
    'walk',
    'wall',
    'walnut',
    'want',
    'warfare',
    'warm',
    'warrior',
    'wash',
    'wasp',
    'waste',
    'water',
    'wave',
    'way',
    'wealth',
    'weapon',
    'wear',
    'weasel',
    'weather',
    'web',
    'wedding',
    'weekend',
    'weird',
    'welcome',
    'west',
    'wet',
    'whale',
    'what',
    'wheat',
    'wheel',
    'when',
    'where',
    'whip',
    'whisper',
    'wide',
    'width',
    'wife',
    'wild',
    'will',
    'win',
    'window',
    'wine',
    'wing',
    'wink',
    'winner',
    'winter',
    'wire',
    'wisdom',
    'wise',
    'wish',
    'witness',
    'wolf',
    'woman',
    'wonder',
    'wood',
    'wool',
    'word',
    'work',
    'world',
    'worry',
    'worth',
    'wrap',
    'wreck',
    'wrestle',
    'wrist',
    'write',
    'wrong',
    'yard',
    'year',
    'yellow',
    'you',
    'young',
    'youth',
    'zebra',
    'zero',
    'zone',
    'zoo',
  ],
  Lp = ((r) => ((r.english = 'english'), r))(Lp || {});
function Ha(r) {
  let e = r.normalize('NFKD'),
    t = [];
  for (let n = 0; n < e.length; n += 1) {
    let i = e.charCodeAt(n);
    if (i < 128) t.push(i);
    else if (i < 2048) t.push((i >> 6) | 192), t.push((i & 63) | 128);
    else if ((i & 64512) === 55296) {
      n += 1;
      let a = e.charCodeAt(n);
      if (n >= e.length || (a & 64512) !== 56320) throw new Error('invalid utf-8 string');
      let c = 65536 + ((i & 1023) << 10) + (a & 1023);
      t.push((c >> 18) | 240),
        t.push(((c >> 12) & 63) | 128),
        t.push(((c >> 6) & 63) | 128),
        t.push((c & 63) | 128);
    } else t.push((i >> 12) | 224), t.push(((i >> 6) & 63) | 128), t.push((i & 63) | 128);
  }
  return V(t);
}
function qp(r) {
  return (1 << r) - 1;
}
function rf(r) {
  return ((1 << r) - 1) << (8 - r);
}
function Ao(r) {
  return Array.isArray(r) ? r : r.split(' ');
}
function Bp(r) {
  return Array.isArray(r) ? r.join(' ') : r;
}
function Fp(r) {
  let e = [0],
    t = 11;
  for (let a = 0; a < r.length; a += 1)
    t > 8
      ? ((e[e.length - 1] <<= 8), (e[e.length - 1] |= r[a]), (t -= 8))
      : ((e[e.length - 1] <<= t),
        (e[e.length - 1] |= r[a] >> (8 - t)),
        e.push(r[a] & qp(8 - t)),
        (t += 3));
  let n = r.length / 4,
    i = V(Pt(r))[0] & rf(n);
  return (e[e.length - 1] <<= n), (e[e.length - 1] |= i >> (8 - n)), e;
}
function Up(r, e) {
  let t = Math.ceil((11 * r.length) / 8),
    n = V(new Uint8Array(t)),
    i = 0;
  for (let w = 0; w < r.length; w += 1) {
    let y = e.indexOf(r[w].normalize('NFKD'));
    if (y === -1) throw new Error('invalid mnemonic');
    for (let M = 0; M < 11; M += 1)
      y & (1 << (10 - M)) && (n[i >> 3] |= 1 << (7 - (i % 8))), (i += 1);
  }
  let a = (32 * r.length) / 3,
    c = r.length / 3,
    v = rf(c);
  if ((V(Pt(n.slice(0, a / 8)))[0] & v) !== (n[n.length - 1] & v))
    throw new Error('invalid checksum');
  return n.slice(0, a / 8);
}
var zp = Ha('Bitcoin seed'),
  jp = 76066276,
  Vp = 70615956;
function Eo(r) {
  if (r.length !== 2048) throw new Error('Invalid word list length');
}
function Jp(r) {
  if (r.length % 4 !== 0 || r.length < 16 || r.length > 32) throw new Error('invalid entropy');
}
function Io(r) {
  if (![12, 15, 18, 21, 24].includes(r.length)) throw new Error('invalid mnemonic size');
}
var jr = class {
    constructor(e = Ia) {
      (this.wordlist = e), Eo(this.wordlist);
    }
    mnemonicToEntropy(e) {
      return jr.mnemonicToEntropy(e, this.wordlist);
    }
    entropyToMnemonic(e) {
      return jr.entropyToMnemonic(e, this.wordlist);
    }
    static mnemonicToEntropy(e, t = Ia) {
      let n = Ao(e);
      return Io(n), K(Up(n, t));
    }
    static entropyToMnemonic(e, t = Ia) {
      let n = V(e, { allowMissingPrefix: !0 });
      return (
        Eo(t),
        Jp(n),
        Fp(n)
          .map((i) => t[i])
          .join(' ')
      );
    }
    static mnemonicToSeed(e, t = '') {
      Io(Ao(e));
      let n = Ha(Bp(e)),
        i = Ha(`mnemonic${t}`);
      return l0(n, i, 2048, 64, 'sha512');
    }
    static mnemonicToMasterKeys(e, t = '') {
      let n = jr.mnemonicToSeed(e, t);
      return jr.masterKeysFromSeed(n);
    }
    static masterKeysFromSeed(e) {
      let t = V(e);
      if (t.length < 16 || t.length > 64) throw new Error('invalid seed');
      return V(Si(jn.sha512, zp, t));
    }
    static seedToExtendedKey(e, t = !1) {
      let n = jr.masterKeysFromSeed(e),
        i = V(t ? Vp : jp),
        a = '0x00',
        c = '0x00000000',
        v = '0x00000000',
        w = n.slice(32),
        y = n.slice(0, 32),
        M = se([i, a, c, v, w, se(['0x00', y])]),
        S = Ka(Pt(Pt(M)), 0, 4);
      return us.encode(se([M, S]));
    }
    static generate(e = 32, t = '') {
      let n = t ? Pt(se([Tr(e), V(t)])) : Tr(e);
      return jr.entropyToMnemonic(n);
    }
  },
  nf = jr,
  af = 2147483648,
  sf = K('0x0488ade4'),
  ds = K('0x0488b21e'),
  of = K('0x04358394'),
  hs = K('0x043587cf');
function Ro(r) {
  return us.encode(se([r, Ka(Pt(Pt(r)), 0, 4)]));
}
function Hp(r = !1, e = !1) {
  return r ? (e ? hs : ds) : e ? of : sf;
}
function Wp(r) {
  return [ds, hs].includes(K(r.slice(0, 4)));
}
function Gp(r) {
  return [sf, of, ds, hs].includes(K(r.slice(0, 4)));
}
function Kp(r, e = 0) {
  let t = r.split('/');
  if (t.length === 0 || (t[0] === 'm' && e !== 0)) throw new Error(`invalid path - ${r}`);
  return (
    t[0] === 'm' && t.shift(),
    t.map((n) => (~n.indexOf("'") ? parseInt(n, 10) + af : parseInt(n, 10)))
  );
}
var nn = class {
    constructor(e) {
      if (
        ((this.depth = 0),
        (this.index = 0),
        (this.fingerprint = K('0x00000000')),
        (this.parentFingerprint = K('0x00000000')),
        e.privateKey)
      ) {
        let t = new Gn(e.privateKey);
        (this.publicKey = K(t.compressedPublicKey)), (this.privateKey = K(e.privateKey));
      } else {
        if (!e.publicKey) throw new Error('Public and Private Key are missing!');
        this.publicKey = K(e.publicKey);
      }
      (this.parentFingerprint = e.parentFingerprint || this.parentFingerprint),
        (this.fingerprint = Ka(zu(Pt(this.publicKey)), 0, 4)),
        (this.depth = e.depth || this.depth),
        (this.index = e.index || this.index),
        (this.chainCode = e.chainCode);
    }
    get extendedKey() {
      return this.toExtendedKey();
    }
    deriveIndex(e) {
      let t = this.privateKey && V(this.privateKey),
        n = V(this.publicKey),
        i = V(this.chainCode),
        a = new Uint8Array(37);
      if (e & af) {
        if (!t) throw new Error('Derive hardened requires privateKey');
        a.set(t, 1);
      } else a.set(V(this.publicKey));
      a.set(or(e, 4), 33);
      let c = V(Si(jn.sha512, i, a)),
        v = c.slice(0, 32),
        w = c.slice(32);
      if (t) {
        let M = '0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141',
          S = B(v).add(t).mod(M).toBytes(32);
        return new nn({
          privateKey: S,
          chainCode: w,
          index: e,
          depth: this.depth + 1,
          parentFingerprint: this.fingerprint,
        });
      }
      let y = new Gn(K(v)).addPoint(n);
      return new nn({
        publicKey: y,
        chainCode: w,
        index: e,
        depth: this.depth + 1,
        parentFingerprint: this.fingerprint,
      });
    }
    derivePath(e) {
      return Kp(e, this.depth).reduce((t, n) => t.deriveIndex(n), this);
    }
    toExtendedKey(e = !1, t = !1) {
      if (this.depth >= 256) throw new Error('Depth too large!');
      let n = Hp(this.privateKey == null || e, t),
        i = K(this.depth),
        a = this.parentFingerprint,
        c = Za(this.index, 4),
        v = this.chainCode,
        w = this.privateKey != null && !e ? se(['0x00', this.privateKey]) : this.publicKey,
        y = se([n, i, a, c, v, w]);
      return Ro(y);
    }
    static fromSeed(e) {
      let t = nf.masterKeysFromSeed(e);
      return new nn({ chainCode: V(t.slice(32)), privateKey: V(t.slice(0, 32)) });
    }
    static fromExtendedKey(e) {
      let t = us.decode(e),
        n = Ro(t.slice(0, 78)) === e;
      if (t.length !== 82 || !Gp(t)) throw new Error('Invalid extended key');
      if (!n) throw new Error('Invalid checksum key');
      let i = t[4],
        a = K(t.slice(5, 9)),
        c = parseInt(K(t.slice(9, 13)).substring(2), 16),
        v = K(t.slice(13, 45)),
        w = t.slice(45, 78);
      if ((i === 0 && a !== '0x00000000') || (i === 0 && c !== 0)) throw new Error('Invalid depth');
      if (Wp(t)) {
        if (w[0] !== 3) throw new Error('Invalid public extended key');
        return new nn({ publicKey: w, chainCode: v, index: c, depth: i, parentFingerprint: a });
      }
      if (w[0] !== 0) throw new Error('Invalid private extended key');
      return new nn({
        privateKey: w.slice(1),
        chainCode: v,
        index: c,
        depth: i,
        parentFingerprint: a,
      });
    }
  },
  Ra = nn,
  Xp = Object.defineProperty,
  No = ((r) =>
    typeof require < 'u'
      ? require
      : typeof Proxy < 'u'
      ? new Proxy(r, { get: (e, t) => (typeof require < 'u' ? require : e)[t] })
      : r)(function (r) {
    if (typeof require < 'u') return require.apply(this, arguments);
    throw new Error('Dynamic require of "' + r + '" is not supported');
  }),
  Zp = (r, e) => () => (r && (e = r((r = 0))), e),
  Yp = (r, e) => {
    for (var t in e) Xp(r, t, { get: e[t], enumerable: !0 });
  },
  Bt = Zp(() => {});
Bt();
Bt();
Bt();
var qi = 'http://127.0.0.1:4000/graphql',
  ff = class extends h0 {
    constructor(e, t = qi) {
      super(),
        (this.provider = this.connect(t)),
        typeof e == 'string' ? (this._address = yt.fromString(e)) : (this._address = an(e));
    }
    get address() {
      return this._address;
    }
    connect(e) {
      if (e) typeof e == 'string' ? (this.provider = new zl(e)) : (this.provider = e);
      else throw new Error('Provider is required');
      return this.provider;
    }
    async getResourcesToSpend(e, t) {
      return this.provider.getResourcesToSpend(this.address, e, t);
    }
    async getCoins(e) {
      let t = [],
        n;
      for (;;) {
        let i = await this.provider.getCoins(this.address, e, { first: 9999, after: n });
        if ((t.push(...i), !(i.length >= 9999))) break;
        throw new Error(`Wallets with more than ${9999} coins are not yet supported`);
      }
      return t;
    }
    async getMessages() {
      let e = [],
        t;
      for (;;) {
        let n = await this.provider.getMessages(this.address, { first: 9999, after: t });
        if ((e.push(...n), !(n.length >= 9999))) break;
        throw new Error(`Wallets with more than ${9999} messages are not yet supported`);
      }
      return e;
    }
    async getBalance(e = Ht) {
      return await this.provider.getBalance(this.address, e);
    }
    async getBalances() {
      let e = [],
        t;
      for (;;) {
        let n = await this.provider.getBalances(this.address, { first: 9999, after: t });
        if ((e.push(...n), !(n.length >= 9999))) break;
        throw new Error(`Wallets with more than ${9999} balances are not yet supported`);
      }
      return e;
    }
    async fund(e) {
      let t = e.calculateFee(),
        n = await this.getResourcesToSpend([t]);
      e.addResources(n);
    }
    async transfer(e, t, n = Ht, i = {}) {
      let a = { gasLimit: zn, ...i },
        c = new Jr(a);
      c.addCoinOutput(e, t, n);
      let v = c.calculateFee(),
        w = [];
      v.assetId === K(n) ? (v.amount.add(t), (w = [v])) : (w = [[t, n], v]);
      let y = await this.getResourcesToSpend(w);
      return c.addResources(y), this.sendTransaction(c);
    }
    async withdrawToBaseLayer(e, t, n = {}) {
      let i = V('0x'.concat(e.toHexString().substring(2).padStart(64, '0'))),
        a = V('0x'.concat(B(t).toHex().substring(2).padStart(16, '0'))),
        c = { script: new Uint8Array([...V(Cl.bytes), ...i, ...a]), gasLimit: zn, ...n },
        v = new Jr(c);
      v.addMessageOutputs();
      let w = v.calculateFee(),
        y = [];
      w.amount.add(t), (y = [w]);
      let M = await this.getResourcesToSpend(y);
      return v.addResources(M), this.sendTransaction(v);
    }
    async sendTransaction(e) {
      let t = er(e);
      return await this.provider.addMissingVariables(t), this.provider.sendTransaction(t);
    }
    async simulateTransaction(e) {
      let t = er(e);
      return await this.provider.addMissingVariables(t), this.provider.simulate(t);
    }
    async buildPredicateTransaction(e, t, n = Ht, i) {
      let a = { fundTransaction: !0, ...i },
        c = new Jr({ gasLimit: zn, ...a });
      c.addCoinOutput(e, t, n);
      let v = [];
      if ((a.fundTransaction && v.push(c.calculateFee()), v.length)) {
        let w = await this.getResourcesToSpend(v);
        c.addResources(w);
      }
      return c;
    }
    async submitPredicate(e, t, n = Ht, i) {
      let a = await this.buildPredicateTransaction(e, t, n, i);
      return (await this.sendTransaction(a)).waitForResult();
    }
    async submitSpendPredicate(e, t, n, i = Ht, a) {
      return this.provider.submitSpendPredicate(e, t, this.address, n, i, a);
    }
  };
Bt();
var cf = class extends ff {
  constructor(e, t = qi) {
    let n = new Gn(e);
    super(n.address, t), (this.signer = () => n), (this.provider = this.connect(t));
  }
  get privateKey() {
    return this.signer().privateKey;
  }
  get publicKey() {
    return this.signer().publicKey;
  }
  async signMessage(e) {
    return this.signer().sign(jl(e));
  }
  async signTransaction(e) {
    let t = er(e),
      n = Vl(t);
    return this.signer().sign(n);
  }
  async populateTransactionWitnessesSignature(e) {
    let t = er(e),
      n = await this.signTransaction(t);
    return t.updateWitnessByOwner(this.address, n), t;
  }
  async sendTransaction(e) {
    let t = er(e);
    return (
      await this.provider.addMissingVariables(t),
      this.provider.sendTransaction(await this.populateTransactionWitnessesSignature(t))
    );
  }
  async simulateTransaction(e) {
    let t = er(e);
    return (
      await this.provider.addMissingVariables(t),
      this.provider.call(await this.populateTransactionWitnessesSignature(t), {
        utxoValidation: !0,
      })
    );
  }
};
cf.defaultPath = "m/44'/1179993420'/0'/0/0";
Bt();
Bt();
var uf = class extends ff {
    unlock(e) {
      return new Ut(e, this.provider);
    }
  },
  Ut = class extends cf {
    lock() {
      return (this.signer = () => new Gn('0x00')), new uf(this.address, this.provider);
    }
    static generate(e) {
      let t = Gn.generatePrivateKey(e?.entropy);
      return new Ut(t, e?.provider);
    }
    static fromSeed(e, t, n) {
      let i = Ra.fromSeed(e).derivePath(t || Ut.defaultPath);
      return new Ut(i.privateKey, n);
    }
    static fromMnemonic(e, t, n, i) {
      let a = nf.mnemonicToSeed(e, n),
        c = Ra.fromSeed(a).derivePath(t || Ut.defaultPath);
      return new Ut(c.privateKey, i);
    }
    static fromExtendedKey(e, t) {
      let n = Ra.fromExtendedKey(e);
      return new Ut(n.privateKey, t);
    }
  },
  Bn = class {
    static fromAddress(r, e = qi) {
      return new uf(r, e);
    }
    static fromPrivateKey(r, e = qi) {
      return new Ut(r, e);
    }
  };
(Bn.generate = Ut.generate),
  (Bn.fromSeed = Ut.fromSeed),
  (Bn.fromMnemonic = Ut.fromMnemonic),
  (Bn.fromExtendedKey = Ut.fromExtendedKey);
var Qp = {};
Yp(Qp, { generateTestWallet: () => r1, seedWallet: () => df });
Bt();
Bt();
Bt();
Bt();
Bt();
Bt();
var Bi,
  ls = 'Node';
typeof globalThis < 'u' && globalThis.crypto && ((Bi = globalThis.crypto), (ls = 'Web'));
if (!Bi && typeof No == 'function')
  try {
    (Bi = No('crypto')), (ls = 'Node');
  } catch (r) {
    console.error('keystore expects a standard Web browser or Node environment.', r);
  }
var Oo = Bi,
  e1 = ls;
Bt();
var t1 = (r) => (e1 === 'Node' ? Oo.randomBytes(r) : Oo.getRandomValues(new Uint8Array(r)));
Bt();
var df = async (r, e) => {
    let t = new Ut({}.GENESIS_SECRET || t1(32), r.provider),
      n = await t.getResourcesToSpend(e),
      i = new Jr({ gasLimit: 1e4, gasPrice: 1 });
    i.addResources(n),
      e.map(ns).forEach(({ amount: a, assetId: c }) => i.addCoinOutput(r.address, a, c)),
      await (await t.sendTransaction(i)).wait();
  },
  r1 = async (r, e) => {
    let t = Bn.generate({ provider: r });
    return e && (await df(t, e)), t;
  },
  n1 = '0xe3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855';
function xn(r) {
  return Pt(r);
}
var i1 = class {
    constructor(r, e, t, n, i, a = 0) {
      (this.left = r),
        (this.right = e),
        (this.parent = t),
        (this.hash = n),
        (this.data = i),
        (this.index = a);
    }
  },
  To = i1;
function a1(r) {
  return xn('0x00'.concat(r.slice(2)));
}
function s1(r, e) {
  return xn('0x01'.concat(r.slice(2)).concat(e.slice(2)));
}
function o1(r) {
  if (!r.length) return n1;
  let e = [];
  for (let a = 0; a < r.length; a += 1) {
    let c = a1(r[a]);
    e.push(new To(-1, -1, -1, c, r[a]));
  }
  let t = e,
    n = (e.length + 1) >> 1,
    i = e.length & 1;
  for (;;) {
    let a = 0;
    for (; a < n - i; a += 1) {
      let c = a << 1,
        v = s1(t[c].hash, t[c + 1].hash);
      e[a] = new To(t[c].index, t[c + 1].index, -1, v, '');
    }
    if ((i === 1 && (e[a] = t[a << 1]), n === 1)) break;
    (i = n & 1), (n = (n + 1) >> 1), (t = e);
  }
  return e[0].hash;
}
var f1 = '0x00',
  hf = '0x01';
function c1(r, e) {
  let t = '0x00'.concat(r.slice(2)).concat(xn(e).slice(2));
  return [xn(t), t];
}
function tn(r, e) {
  let t = '0x01'.concat(r.slice(2)).concat(e.slice(2));
  return [xn(t), t];
}
function Na(r) {
  let e = hf.length;
  return ['0x'.concat(r.slice(e, e + 64)), '0x'.concat(r.slice(e + 64))];
}
function u1(r) {
  let e = hf.length;
  return ['0x'.concat(r.slice(e, e + 64)), '0x'.concat(r.slice(e + 64))];
}
function Oa(r) {
  return r.slice(0, 4) === f1;
}
var d1 = class {
    constructor(r, e, t, n, i) {
      (this.SideNodes = r),
        (this.NonMembershipLeafData = e),
        (this.BitMask = t),
        (this.NumSideNodes = n),
        (this.SiblingData = i);
    }
  },
  h1 = d1,
  l1 = class {
    constructor(e, t, n) {
      (this.SideNodes = e), (this.NonMembershipLeafData = t), (this.SiblingData = n);
    }
  },
  p1 = l1,
  Ct = '0x0000000000000000000000000000000000000000000000000000000000000000';
function fn(r, e) {
  let t = r.slice(2),
    n = '0x'.concat(t.slice(Math.floor(e / 8) * 2, Math.floor(e / 8) * 2 + 2));
  return (Number(n) & (1 << (8 - 1 - (e % 8)))) > 0 ? 1 : 0;
}
function v1(r) {
  let e = 0,
    t = r.length - 1,
    n = r;
  for (; e < t; ) ([n[e], n[t]] = [n[t], n[e]]), (e += 1), (t -= 1);
  return n;
}
function b1(r, e) {
  let t = 0;
  for (let n = 0; n < 256 && fn(r, n) === fn(e, n); n += 1) t += 1;
  return t;
}
function m1(r) {
  let e = [],
    t = [],
    n;
  for (let i = 0; i < r.SideNodes.length; i += 1)
    (n = r.SideNodes[i]), n === Ct ? e.push(0) : (t.push(n), e.push(1));
  return new h1(t, r.NonMembershipLeafData, e, r.SideNodes.length, r.SiblingData);
}
var g1 = class {
    constructor() {
      let e = {};
      (this.ms = e), (this.root = Ct), (this.ms[this.root] = Ct);
    }
    get(e) {
      return this.ms[e];
    }
    set(e, t) {
      this.ms[e] = t;
    }
    setRoot(e) {
      this.root = e;
    }
    sideNodesForRoot(e, t) {
      let n = [];
      if (t === Ct) return [n, Ct, '', ''];
      let i = this.get(t);
      if (Oa(i)) return [n, t, i, ''];
      let a,
        c,
        v = '',
        w = '';
      for (let M = 0; M < 256; M += 1) {
        if (
          (([a, c] = u1(i)),
          fn(e, M) === 1 ? ((w = a), (v = c)) : ((w = c), (v = a)),
          n.push(w),
          v === Ct)
        ) {
          i = '';
          break;
        }
        if (((i = this.get(v)), Oa(i))) break;
      }
      let y = this.get(w);
      return [v1(n), v, i, y];
    }
    deleteWithSideNodes(e, t, n, i) {
      if (n === Ct) return this.root;
      let [a] = Na(i);
      if (a !== e) return this.root;
      let c = '',
        v = '',
        w = '',
        y = '',
        M = !1;
      for (let S = 0; S < t.length; S += 1)
        if (t[S] !== '') {
          if (((w = t[S]), v === ''))
            if (((y = this.get(w)), Oa(y))) {
              (c = w), (v = w);
              continue;
            } else (v = Ct), (M = !0);
          (!M && w === Ct) ||
            (M || (M = !0),
            fn(e, t.length - 1 - S) === 1 ? ([c, v] = tn(w, v)) : ([c, v] = tn(v, w)),
            this.set(c, v),
            (v = c));
        }
      return c === '' && (c = Ct), c;
    }
    updateWithSideNodes(e, t, n, i, a) {
      let c, v;
      this.set(xn(t), t), ([c, v] = c1(e, t)), this.set(c, v), (v = c);
      let w;
      if (i === Ct) w = 256;
      else {
        let [y] = Na(a);
        w = b1(e, y);
      }
      w !== 256 &&
        (fn(e, w) === 1 ? ([c, v] = tn(i, v)) : ([c, v] = tn(v, i)), this.set(c, v), (v = c));
      for (let y = 0; y < 256; y += 1) {
        let M,
          S = 256 - n.length;
        if (y - S < 0 || n[y - S] === '')
          if (w !== 256 && w > 256 - 1 - y) M = Ct;
          else continue;
        else M = n[y - S];
        fn(e, 256 - 1 - y) === 1 ? ([c, v] = tn(M, v)) : ([c, v] = tn(v, M)),
          this.set(c, v),
          (v = c);
      }
      return c;
    }
    update(e, t) {
      let [n, i, a] = this.sideNodesForRoot(e, this.root),
        c;
      t === Ct
        ? (c = this.deleteWithSideNodes(e, n, i, a))
        : (c = this.updateWithSideNodes(e, t, n, i, a)),
        this.setRoot(c);
    }
    delete(e) {
      this.update(e, Ct);
    }
    prove(e) {
      let [t, n, i, a] = this.sideNodesForRoot(e, this.root),
        c = [];
      for (let w = 0; w < t.length; w += 1) t[w] !== '' && c.push(t[w]);
      let v = '';
      if (n !== Ct) {
        let [w] = Na(i);
        w !== e && (v = i);
      }
      return new p1(c, v, a);
    }
    proveCompacted(e) {
      let t = this.prove(e);
      return m1(t);
    }
  },
  y1 = g1,
  w1 = [
    'Success',
    'Revert',
    'OutOfGas',
    'TransactionValidity',
    'MemoryOverflow',
    'ArithmeticOverflow',
    'ContractNotFound',
    'MemoryOwnership',
    'NotEnoughBalance',
    'ExpectedInternalContext',
    'AssetIdNotFound',
    'InputNotFound',
    'OutputNotFound',
    'WitnessNotFound',
    'TransactionMaturity',
    'InvalidMetadataIdentifier',
    'MalformedCallStructure',
    'ReservedRegisterNotWritable',
    'ErrorFlag',
    'InvalidImmediateValue',
    'ExpectedCoinInput',
    'MaxMemoryAccess',
    'MemoryWriteOverlap',
    'ContractNotInInputs',
    'InternalBalanceOverflow',
    'ContractMaxSize',
    'ExpectedUnallocatedStack',
    'MaxStaticContractsReached',
    'TransferAmountCannotBeZero',
    'ExpectedOutputVariable',
    'ExpectedParentInternalContext',
    'IllegalJump',
    'NonZeroMessageOutputRecipient',
    'ZeroedMessageOutputRecipient',
  ],
  Ta = 'https://docs.rs/fuel-asm/latest/fuel_asm/enum.PanicReason.html',
  x1 = (r) => (w1.includes(r) ? r : 'unknown'),
  M1 = (r) => {
    if (r?.type === 'failure') {
      let e = x1(r.reason);
      return { doc: e !== 'unknown' ? `${Ta}#variant.${e}` : Ta, reason: e };
    }
    return { doc: Ta, reason: 'unknown' };
  },
  Co = (r, e) => (typeof e == 'bigint' ? e.toString() : e),
  _1 = (r, e) => `${r === gt ? 'script' : r}: ${e}`,
  S1 = class extends Error {
    constructor(r, e, t) {
      let n = JSON.stringify(M1(r.status), null, 2),
        i = r.receipts.filter((w) => w.type === Nt.Revert),
        a = i.length
          ? `Reverts:
${i.map(({ id: w, ...y }) => _1(w, `${y.val} ${JSON.stringify(y, Co)}`)).join(`
`)}`
          : null,
        c = t.length
          ? `Logs:
${JSON.stringify(t, null, 2)}`
          : null,
        v = `Receipts:
${JSON.stringify(
  r.receipts.map(({ type: w, ...y }) => ({ type: Nt[w], ...y })),
  Co,
  2
)}`;
      super(`${e}

${n}

${
  a
    ? `${a}

`
    : ''
}${
        c
          ? `${c}

`
          : ''
      }${v}`),
        (this.logs = t);
    }
  };
function A1(r) {
  let e = [...r.receipts],
    t = e.pop();
  if (!t) throw new Error('Expected scriptResultReceipt');
  if (t.type !== Nt.ScriptResult) throw new Error(`Invalid scriptResultReceipt type: ${t.type}`);
  let n = e.pop();
  if (!n) throw new Error('Expected returnReceipt');
  if (n.type !== Nt.Return && n.type !== Nt.ReturnData && n.type !== Nt.Revert)
    throw new Error(`Invalid returnReceipt type: ${n.type}`);
  return {
    code: t.result,
    gasUsed: t.gasUsed,
    receipts: e,
    scriptResultReceipt: t,
    returnReceipt: n,
    callResult: r,
  };
}
var lf = class {
  constructor(e, t, n) {
    (this.bytes = V(e)), (this.scriptDataEncoder = t), (this.scriptResultDecoder = n);
  }
  getScriptDataOffset() {
    return rd + nd + new et(this.bytes.length).encodedLength;
  }
  getArgOffset() {
    return this.getScriptDataOffset() + ed + ga + td + ga + ga;
  }
  encodeScriptData(e) {
    return this.scriptDataEncoder(e);
  }
  decodeCallResult(e, t = []) {
    try {
      let n = A1(e);
      return this.scriptResultDecoder(n);
    } catch (n) {
      throw new S1(e, n.message, t);
    }
  }
};
new lf(
  '0x24000000',
  () => new Uint8Array(0),
  () => {}
);
var E1 = Object.defineProperty,
  I1 = (r, e) => {
    for (var t in e) E1(r, t, { get: e[t], enumerable: !0 });
  },
  R1 = {};
I1(R1, {
  assert: () => T1,
  getContractId: () => O1,
  getContractRoot: () => pf,
  getContractStorageRoot: () => N1,
  includeHexPrefix: () => C1,
});
var pf = (r) => {
    let e = [];
    for (let t = 0; t < r.length; t += 8) {
      let n = new Uint8Array(8);
      n.set(r.slice(t, t + 8)), e.push(n);
    }
    return o1(e.map((t) => K(t)));
  },
  N1 = (r) => {
    let e = new y1();
    return r.forEach(({ key: t, value: n }) => e.update(t, n)), e.root;
  },
  O1 = (r, e, t) => {
    let n = pf(V(r));
    return Pt(se(['0x4655454C', e, n, t]));
  };
function T1(r, e) {
  if (!r) throw new Error(e);
}
var C1 = (r, e) => K(r, { ...e, allowMissingPrefix: !0 }),
  Po = [
    {
      type: 'function',
      inputs: [
        {
          name: 'script_data',
          type: 'struct ScriptData',
          components: [
            {
              name: 'calls',
              type: '[enum Option; 5]',
              components: [
                {
                  name: '__array_element',
                  type: 'enum Option',
                  components: [
                    { name: 'None', type: '()', components: [], typeArguments: null },
                    {
                      name: 'Some',
                      type: 'struct MulticallCall',
                      components: [
                        {
                          name: 'contract_id',
                          type: 'struct ContractId',
                          components: [
                            { name: 'value', type: 'b256', components: null, typeArguments: null },
                          ],
                          typeArguments: null,
                        },
                        { name: 'fn_selector', type: 'u64', components: null, typeArguments: null },
                        {
                          name: 'fn_arg',
                          type: 'enum CallValue',
                          components: [
                            { name: 'Value', type: 'u64', components: null, typeArguments: null },
                            {
                              name: 'Data',
                              type: '(u64, u64)',
                              components: [
                                {
                                  name: '__tuple_element',
                                  type: 'u64',
                                  components: null,
                                  typeArguments: null,
                                },
                                {
                                  name: '__tuple_element',
                                  type: 'u64',
                                  components: null,
                                  typeArguments: null,
                                },
                              ],
                              typeArguments: null,
                            },
                          ],
                          typeArguments: null,
                        },
                        {
                          name: 'parameters',
                          type: 'struct CallParameters',
                          components: [
                            {
                              name: 'amount',
                              type: 'enum Option',
                              components: [
                                { name: 'None', type: '()', components: [], typeArguments: null },
                                {
                                  name: 'Some',
                                  type: 'u64',
                                  components: null,
                                  typeArguments: null,
                                },
                              ],
                              typeArguments: [
                                { name: 'T', type: 'u64', components: null, typeArguments: null },
                              ],
                            },
                            {
                              name: 'asset_id',
                              type: 'enum Option',
                              components: [
                                { name: 'None', type: '()', components: [], typeArguments: null },
                                {
                                  name: 'Some',
                                  type: 'struct ContractId',
                                  components: [
                                    {
                                      name: 'value',
                                      type: 'b256',
                                      components: null,
                                      typeArguments: null,
                                    },
                                  ],
                                  typeArguments: null,
                                },
                              ],
                              typeArguments: [
                                {
                                  name: 'T',
                                  type: 'struct ContractId',
                                  components: [
                                    {
                                      name: 'value',
                                      type: 'b256',
                                      components: null,
                                      typeArguments: null,
                                    },
                                  ],
                                  typeArguments: null,
                                },
                              ],
                            },
                            {
                              name: 'gas',
                              type: 'enum Option',
                              components: [
                                { name: 'None', type: '()', components: [], typeArguments: null },
                                {
                                  name: 'Some',
                                  type: 'u64',
                                  components: null,
                                  typeArguments: null,
                                },
                              ],
                              typeArguments: [
                                { name: 'T', type: 'u64', components: null, typeArguments: null },
                              ],
                            },
                          ],
                          typeArguments: null,
                        },
                      ],
                      typeArguments: null,
                    },
                  ],
                  typeArguments: [
                    {
                      name: 'T',
                      type: 'struct MulticallCall',
                      components: [
                        {
                          name: 'contract_id',
                          type: 'struct ContractId',
                          components: [
                            { name: 'value', type: 'b256', components: null, typeArguments: null },
                          ],
                          typeArguments: null,
                        },
                        { name: 'fn_selector', type: 'u64', components: null, typeArguments: null },
                        {
                          name: 'fn_arg',
                          type: 'enum CallValue',
                          components: [
                            { name: 'Value', type: 'u64', components: null, typeArguments: null },
                            {
                              name: 'Data',
                              type: '(u64, u64)',
                              components: [
                                {
                                  name: '__tuple_element',
                                  type: 'u64',
                                  components: null,
                                  typeArguments: null,
                                },
                                {
                                  name: '__tuple_element',
                                  type: 'u64',
                                  components: null,
                                  typeArguments: null,
                                },
                              ],
                              typeArguments: null,
                            },
                          ],
                          typeArguments: null,
                        },
                        {
                          name: 'parameters',
                          type: 'struct CallParameters',
                          components: [
                            {
                              name: 'amount',
                              type: 'enum Option',
                              components: [
                                { name: 'None', type: '()', components: [], typeArguments: null },
                                {
                                  name: 'Some',
                                  type: 'u64',
                                  components: null,
                                  typeArguments: null,
                                },
                              ],
                              typeArguments: [
                                { name: 'T', type: 'u64', components: null, typeArguments: null },
                              ],
                            },
                            {
                              name: 'asset_id',
                              type: 'enum Option',
                              components: [
                                { name: 'None', type: '()', components: [], typeArguments: null },
                                {
                                  name: 'Some',
                                  type: 'struct ContractId',
                                  components: [
                                    {
                                      name: 'value',
                                      type: 'b256',
                                      components: null,
                                      typeArguments: null,
                                    },
                                  ],
                                  typeArguments: null,
                                },
                              ],
                              typeArguments: [
                                {
                                  name: 'T',
                                  type: 'struct ContractId',
                                  components: [
                                    {
                                      name: 'value',
                                      type: 'b256',
                                      components: null,
                                      typeArguments: null,
                                    },
                                  ],
                                  typeArguments: null,
                                },
                              ],
                            },
                            {
                              name: 'gas',
                              type: 'enum Option',
                              components: [
                                { name: 'None', type: '()', components: [], typeArguments: null },
                                {
                                  name: 'Some',
                                  type: 'u64',
                                  components: null,
                                  typeArguments: null,
                                },
                              ],
                              typeArguments: [
                                { name: 'T', type: 'u64', components: null, typeArguments: null },
                              ],
                            },
                          ],
                          typeArguments: null,
                        },
                      ],
                      typeArguments: null,
                    },
                  ],
                },
              ],
              typeArguments: null,
            },
          ],
          typeArguments: null,
        },
      ],
      name: 'main',
      outputs: [
        {
          name: '',
          type: 'struct ScriptReturn',
          components: [
            {
              name: 'call_returns',
              type: '[enum Option; 5]',
              components: [
                {
                  name: '__array_element',
                  type: 'enum Option',
                  components: [
                    { name: 'None', type: '()', components: [], typeArguments: null },
                    {
                      name: 'Some',
                      type: 'enum CallValue',
                      components: [
                        { name: 'Value', type: 'u64', components: null, typeArguments: null },
                        {
                          name: 'Data',
                          type: '(u64, u64)',
                          components: [
                            {
                              name: '__tuple_element',
                              type: 'u64',
                              components: null,
                              typeArguments: null,
                            },
                            {
                              name: '__tuple_element',
                              type: 'u64',
                              components: null,
                              typeArguments: null,
                            },
                          ],
                          typeArguments: null,
                        },
                      ],
                      typeArguments: null,
                    },
                  ],
                  typeArguments: [
                    {
                      name: 'T',
                      type: 'enum CallValue',
                      components: [
                        { name: 'Value', type: 'u64', components: null, typeArguments: null },
                        {
                          name: 'Data',
                          type: '(u64, u64)',
                          components: [
                            {
                              name: '__tuple_element',
                              type: 'u64',
                              components: null,
                              typeArguments: null,
                            },
                            {
                              name: '__tuple_element',
                              type: 'u64',
                              components: null,
                              typeArguments: null,
                            },
                          ],
                          typeArguments: null,
                        },
                      ],
                      typeArguments: null,
                    },
                  ],
                },
              ],
              typeArguments: null,
            },
          ],
          typeArguments: null,
        },
      ],
    },
  ],
  P1 =
    '0x90000004470000000000000000000cd45dfcc00110fff3001a5c5000910005b861440006724002d0164114005b40100d360000006158000c61440001504175305f5d10a6504175305d4570a6504171385f5d1027504171385d417027134100007340001a9000001f1a445000910000085d43f0005f4500009000002b504171385d4170271341004073400024900000291a445000910000085d43f0015f4500009000002b360000001a44000050417528504175286041100850457528504170085041700860411008504170085d4100001341000073400037900000396144000c9000003b360000001a440000504174305f5d1086504174305d4570865d43f00210450440504174485f5d108961440001504175405f5d10a8504175405d4570a8504171405f5d1028504171405d417028134100007340004f900000541a445000910000085d43f0005f45000090000060504171405d41702813410040734000599000005e1a445000910000085d43f0015f45000090000060360000001a44000050417538504175386041100850457538504170005041700060411008504170005d410000134100007340006c9000006e6144000690000078504170005d410000134100407340007390000076360000001a44000090000078360000001a4400005d43f00220451400504173805f5d1070504174485d497089504173805d4170701a445000910000105f4520005f450001504175a8504175a8604110105d47f00326440000504470015041726050417260604110a026000000504070011a445000910000105f4500005f440001504174785041747860411010504173505f5c006a5d47f0025d43f00412451400504173005f5d1060504173505d45706a504173005d41706016411400734000a4900000b150496000504173505d41706a5545009010452440504170785041707860411090504170785d41000013410040734001249000031f504972601a445000910000a050411000604120a05041748850417488604110a026000000504070011a445000910000105f4500005f44000150417198504171986041101050517198505574885d454001504174085f5d10815d4540015d43f00310450440504173c85f5d10795d4140005d4d4001504573c85d457079154914c0734800d3900000e12644000050487001504573a85f5d207515453000734400da900000de504573a85d457075284504c0900000de504173a85d417075900000e15f510000504173c85d4170795f5100015d454000504174085d417081104504405d43f0032845540050557198505174785d41400113410000734000f1900000f35d4150019000011c5d455001504174105f5d10825d4550015d41400110450440504173d05f5d107a5d4150005d4d5001504573d05d45707a154914c073480102900001102644000050487001504573b05f5d207615453000734401099000010d504573b05d457076284504c09000010d504173b05d417076900001105f550000504173d05d41707a5f5500015d4940005d455000504174105d417082104504405d414001284524005d417082504171985d450000504171985d41000125450000504574885d43f003254500005041707850450008504171a8504171a860411088504171a850450028504171085041710860411018504171085d41000013410000734001339000013f504171085d450002504175485f5d10a9504175485d4970a91a445000910000185d43f0005f4500005f4520029000017b504171085d41000013410040734001449000016050417108504100085d450000504173e85f5d107d50417108504100085d450001504173785f5d106f504175a85d450000504173e85d41707d10450440504173785d41706f1a485000910000105f4910005f4900011a445000910000185d43f0015f45000050411008604120109000017b50417108504100085d450000504173f05f5d107e50417108504100085d450001504173905f5d1072504175a85d450000504173f05d41707e10450440504173905d4170721a485000910000105f4910005f4900011a445000910000185d43f0015f4500005041100860412010504173085041730860411018504171a850550000504171a85d51000450457308504171a8504d0040504170105041701060411018504170105d410000134100007340018d90000194504170105d450002504175505f5d10aa504175505d4570aa900001a8504170105d4100001341004073400199900001a150417010504100085d450000504174385f5d1087504174385d457087900001a850417010504100085d450000504174505f5d108a504174505d45708a504173205f5d1064504173205d4970641a4450009100003050411000604150205f4540045f45200550417230504172306041103050453000504170285041702860411010504170285d41000013410040734001be900001c5504170285d450001504171485f5d1029504171485d457029900001cd504170285d41000013410000734001ca900001cc1a440000900001cd1a440000504171505f5d102a50453010504170385041703860411028504170385d41000013410040734001d8900001df504170385045000850417358504173586041102050497358900001f1504170385d41000013410000734001e4900001eb1a485000910000205d47f00a104513005041200060411020900001f11a485000910000205d47f00a10451300504120006041102050417158504171586041202050453038504170605041706060411010504170605d41000013410040734001fd90000204504170605d450001504173405f5d1068504173405d4570689000020c504170605d41000013410000734002099000020b1a44a0009000020c1a44a000504173485f5d1069504d7230504171505d49702a50457158504173485d4170692d4d24501a44e000504170705f5d100e504170705d41700e134100007340021d900002281a44d000504175785f5d10af504175785d4570af1a485000910000185d43f0005f4900005f4910029000023d504170705d45700e504173885f5d10711a44d000504174585f5d108b504174585d49708b504173885d4170711a445000910000105f4520005f4500011a485000910000185d43f0015f490000504120086041101050417460504174606041201850457460504171205041712060411018504171205d410000134100007340024990000255504171205d450002504175a05f5d10b4504175a05d4970b41a445000910000185d43f0005f4500005f45200290000309504171205d410000134100407340025a900002b250417120504100085d450000504174285f5d108550417120504100085d450001504173985f5d1073504174285d497085504173985d4170731a445000910000105f4520005f45000150417178504171786041101050557478505171785d4140011341000073400275900002775d455001900002a15d455001504174185f5d10835d4550015d41400110450440504173d85f5d107b5d4150005d4d5001504573d85d45707b154914c073480286900002942644000050487001504573b85f5d2077154530007344028d90000291504573b85d457077284504c090000291504173b85d417077900002945f550000504173d85d41707b5f5500015d4940005d455000504174185d417083104504405d41400128452400504174185d457083504173f85f5d107f504173f85d45707f504173985d4170731a485000910000105f4910005f4900011a445000910000185d43f0015f45000050411008604120109000030950417120504100085d450000504174405f5d108850417120504100085d450001504173a05f5d1074504174405d497088504173a05d4170741a445000910000105f4520005f45000150417188504171886041101050557478505171885d41400113410000734002cd900002cf5d455001900002f95d455001504174205f5d10845d4550015d41400110450440504173e05f5d107c5d4150005d4d5001504573e05d45707c154914c0734802de900002ec2644000050487001504573c05f5d207815453000734402e5900002e9504573c05d457078284504c0900002e9504173c05d417078900002ec5f550000504173e05d41707c5f5500015d4940005d455000504174205d417084104504405d41400128452400504174205d457084504174005f5d1080504174005d457080504173a05d4170741a485000910000105f4910005f4900011a445000910000185d43f0015f4500005041100860412010504173285041732860411018504973281a445000910000205d43f0015f450000504110086041201850417558504175586041102050457260504173505d41706a5549002010491480504575585d43f009284914009000032e504175801a445000910000205d43f0005f450000504175806041102050457260504173505d41706a5549002010491480504575805d43f0092849140050417350504173505d41706a104014005f5d006a9000009d470000000000000000000000000000000000000100000000000002d000000000000000a00000000000000090000000000000000000000000000000000000000000000000000000000000000000000000000000200000000000000cfc';
new lf(
  P1,
  (r) => {
    let e = Po[0].inputs,
      t = new Ai().getCoder(e[0]),
      n = t.coders.calls.length;
    if (r.length > n) throw new Error(`At most ${n} calls are supported`);
    let i = new Uint8Array(),
      a = [];
    for (let w = 0; w < n; w += 1) {
      let y = r[w],
        M;
      if (y) {
        let S = V(y.data),
          I = S.slice(0, 8),
          E = S.slice(8, 16).some((z) => z === 1),
          R = S.slice(16),
          T;
        E
          ? ((T = { Data: [i.length, R.length] }), (i = se([i, R])))
          : (T = { Value: new $().decode(R, 0)[0] }),
          (M = {
            contract_id: { value: y.contractId },
            fn_selector: new $().decode(I, 0)[0],
            fn_arg: T,
            parameters: {
              amount: y.amount ? B(y.amount) : void 0,
              asset_id: y.assetId ? { value: y.assetId } : void 0,
              gas: y.gas ? B(y.gas) : void 0,
            },
          });
      } else M = void 0;
      a.push(M);
    }
    let c = { calls: a },
      v = t.encode(c);
    return se([v, i]);
  },
  (r) => {
    if (nr(r.code) !== 0) throw new Error(`Script returned non-zero result: ${r.code}`);
    if (r.returnReceipt.type !== Nt.ReturnData)
      throw new Error('Expected returnReceipt to be a ReturnDataReceipt');
    let e = V(r.returnReceipt.data),
      t = Po[0].outputs,
      n = new Ai().getCoder(t[0]),
      [i, a] = n.decode(e, 0),
      c = e.slice(a),
      v = [];
    return (
      i.call_returns.forEach((w, y) => {
        if (w)
          if (w.Data) {
            let [M, S] = w.Data;
            v[y] = c.slice(nr(M), nr(M) + nr(S));
          } else v[y] = new $().encode(w.Value);
      }),
      v
    );
  }
);
new Te(Mn.FUELS);
var vf = { exports: {} },
  dn = typeof Reflect == 'object' ? Reflect : null,
  ko =
    dn && typeof dn.apply == 'function'
      ? dn.apply
      : function (e, t, n) {
          return Function.prototype.apply.call(e, t, n);
        },
  Mi;
dn && typeof dn.ownKeys == 'function'
  ? (Mi = dn.ownKeys)
  : Object.getOwnPropertySymbols
  ? (Mi = function (e) {
      return Object.getOwnPropertyNames(e).concat(Object.getOwnPropertySymbols(e));
    })
  : (Mi = function (e) {
      return Object.getOwnPropertyNames(e);
    });
function k1(r) {
  console && console.warn && console.warn(r);
}
var bf =
  Number.isNaN ||
  function (e) {
    return e !== e;
  };
function Ve() {
  Ve.init.call(this);
}
vf.exports = Ve;
vf.exports.once = q1;
Ve.EventEmitter = Ve;
Ve.prototype._events = void 0;
Ve.prototype._eventsCount = 0;
Ve.prototype._maxListeners = void 0;
var $o = 10;
function na(r) {
  if (typeof r != 'function')
    throw new TypeError(
      'The "listener" argument must be of type Function. Received type ' + typeof r
    );
}
Object.defineProperty(Ve, 'defaultMaxListeners', {
  enumerable: !0,
  get: function () {
    return $o;
  },
  set: function (r) {
    if (typeof r != 'number' || r < 0 || bf(r))
      throw new RangeError(
        'The value of "defaultMaxListeners" is out of range. It must be a non-negative number. Received ' +
          r +
          '.'
      );
    $o = r;
  },
});
Ve.init = function () {
  (this._events === void 0 || this._events === Object.getPrototypeOf(this)._events) &&
    ((this._events = Object.create(null)), (this._eventsCount = 0)),
    (this._maxListeners = this._maxListeners || void 0);
};
Ve.prototype.setMaxListeners = function (e) {
  if (typeof e != 'number' || e < 0 || bf(e))
    throw new RangeError(
      'The value of "n" is out of range. It must be a non-negative number. Received ' + e + '.'
    );
  return (this._maxListeners = e), this;
};
function mf(r) {
  return r._maxListeners === void 0 ? Ve.defaultMaxListeners : r._maxListeners;
}
Ve.prototype.getMaxListeners = function () {
  return mf(this);
};
Ve.prototype.emit = function (e) {
  for (var t = [], n = 1; n < arguments.length; n++) t.push(arguments[n]);
  var i = e === 'error',
    a = this._events;
  if (a !== void 0) i = i && a.error === void 0;
  else if (!i) return !1;
  if (i) {
    var c;
    if ((t.length > 0 && (c = t[0]), c instanceof Error)) throw c;
    var v = new Error('Unhandled error.' + (c ? ' (' + c.message + ')' : ''));
    throw ((v.context = c), v);
  }
  var w = a[e];
  if (w === void 0) return !1;
  if (typeof w == 'function') ko(w, this, t);
  else for (var y = w.length, M = Mf(w, y), n = 0; n < y; ++n) ko(M[n], this, t);
  return !0;
};
function gf(r, e, t, n) {
  var i, a, c;
  if (
    (na(t),
    (a = r._events),
    a === void 0
      ? ((a = r._events = Object.create(null)), (r._eventsCount = 0))
      : (a.newListener !== void 0 &&
          (r.emit('newListener', e, t.listener ? t.listener : t), (a = r._events)),
        (c = a[e])),
    c === void 0)
  )
    (c = a[e] = t), ++r._eventsCount;
  else if (
    (typeof c == 'function' ? (c = a[e] = n ? [t, c] : [c, t]) : n ? c.unshift(t) : c.push(t),
    (i = mf(r)),
    i > 0 && c.length > i && !c.warned)
  ) {
    c.warned = !0;
    var v = new Error(
      'Possible EventEmitter memory leak detected. ' +
        c.length +
        ' ' +
        String(e) +
        ' listeners added. Use emitter.setMaxListeners() to increase limit'
    );
    (v.name = 'MaxListenersExceededWarning'),
      (v.emitter = r),
      (v.type = e),
      (v.count = c.length),
      k1(v);
  }
  return r;
}
Ve.prototype.addListener = function (e, t) {
  return gf(this, e, t, !1);
};
Ve.prototype.on = Ve.prototype.addListener;
Ve.prototype.prependListener = function (e, t) {
  return gf(this, e, t, !0);
};
function $1() {
  if (!this.fired)
    return (
      this.target.removeListener(this.type, this.wrapFn),
      (this.fired = !0),
      arguments.length === 0
        ? this.listener.call(this.target)
        : this.listener.apply(this.target, arguments)
    );
}
function yf(r, e, t) {
  var n = { fired: !1, wrapFn: void 0, target: r, type: e, listener: t },
    i = $1.bind(n);
  return (i.listener = t), (n.wrapFn = i), i;
}
Ve.prototype.once = function (e, t) {
  return na(t), this.on(e, yf(this, e, t)), this;
};
Ve.prototype.prependOnceListener = function (e, t) {
  return na(t), this.prependListener(e, yf(this, e, t)), this;
};
Ve.prototype.removeListener = function (e, t) {
  var n, i, a, c, v;
  if ((na(t), (i = this._events), i === void 0)) return this;
  if (((n = i[e]), n === void 0)) return this;
  if (n === t || n.listener === t)
    --this._eventsCount === 0
      ? (this._events = Object.create(null))
      : (delete i[e], i.removeListener && this.emit('removeListener', e, n.listener || t));
  else if (typeof n != 'function') {
    for (a = -1, c = n.length - 1; c >= 0; c--)
      if (n[c] === t || n[c].listener === t) {
        (v = n[c].listener), (a = c);
        break;
      }
    if (a < 0) return this;
    a === 0 ? n.shift() : D1(n, a),
      n.length === 1 && (i[e] = n[0]),
      i.removeListener !== void 0 && this.emit('removeListener', e, v || t);
  }
  return this;
};
Ve.prototype.off = Ve.prototype.removeListener;
Ve.prototype.removeAllListeners = function (e) {
  var t, n, i;
  if (((n = this._events), n === void 0)) return this;
  if (n.removeListener === void 0)
    return (
      arguments.length === 0
        ? ((this._events = Object.create(null)), (this._eventsCount = 0))
        : n[e] !== void 0 &&
          (--this._eventsCount === 0 ? (this._events = Object.create(null)) : delete n[e]),
      this
    );
  if (arguments.length === 0) {
    var a = Object.keys(n),
      c;
    for (i = 0; i < a.length; ++i) (c = a[i]), c !== 'removeListener' && this.removeAllListeners(c);
    return (
      this.removeAllListeners('removeListener'),
      (this._events = Object.create(null)),
      (this._eventsCount = 0),
      this
    );
  }
  if (((t = n[e]), typeof t == 'function')) this.removeListener(e, t);
  else if (t !== void 0) for (i = t.length - 1; i >= 0; i--) this.removeListener(e, t[i]);
  return this;
};
function wf(r, e, t) {
  var n = r._events;
  if (n === void 0) return [];
  var i = n[e];
  return i === void 0
    ? []
    : typeof i == 'function'
    ? t
      ? [i.listener || i]
      : [i]
    : t
    ? L1(i)
    : Mf(i, i.length);
}
Ve.prototype.listeners = function (e) {
  return wf(this, e, !0);
};
Ve.prototype.rawListeners = function (e) {
  return wf(this, e, !1);
};
Ve.listenerCount = function (r, e) {
  return typeof r.listenerCount == 'function' ? r.listenerCount(e) : xf.call(r, e);
};
Ve.prototype.listenerCount = xf;
function xf(r) {
  var e = this._events;
  if (e !== void 0) {
    var t = e[r];
    if (typeof t == 'function') return 1;
    if (t !== void 0) return t.length;
  }
  return 0;
}
Ve.prototype.eventNames = function () {
  return this._eventsCount > 0 ? Mi(this._events) : [];
};
function Mf(r, e) {
  for (var t = new Array(e), n = 0; n < e; ++n) t[n] = r[n];
  return t;
}
function D1(r, e) {
  for (; e + 1 < r.length; e++) r[e] = r[e + 1];
  r.pop();
}
function L1(r) {
  for (var e = new Array(r.length), t = 0; t < e.length; ++t) e[t] = r[t].listener || r[t];
  return e;
}
function q1(r, e) {
  return new Promise(function (t, n) {
    function i(c) {
      r.removeListener(e, a), n(c);
    }
    function a() {
      typeof r.removeListener == 'function' && r.removeListener('error', i),
        t([].slice.call(arguments));
    }
    _f(r, e, a, { once: !0 }), e !== 'error' && B1(r, i, { once: !0 });
  });
}
function B1(r, e, t) {
  typeof r.on == 'function' && _f(r, 'error', e, t);
}
function _f(r, e, t, n) {
  if (typeof r.on == 'function') n.once ? r.once(e, t) : r.on(e, t);
  else if (typeof r.addEventListener == 'function')
    r.addEventListener(e, function i(a) {
      n.once && r.removeEventListener(e, i), t(a);
    });
  else
    throw new TypeError(
      'The "emitter" argument must be of type EventEmitter. Received type ' + typeof r
    );
}
var F1 = {},
  ia = {},
  ri = {};
(function (r) {
  var e =
    (ie && ie.__extends) ||
    (function () {
      var E = function (R, T) {
        return (
          (E =
            Object.setPrototypeOf ||
            ({ __proto__: [] } instanceof Array &&
              function (z, q) {
                z.__proto__ = q;
              }) ||
            function (z, q) {
              for (var Y in q) Object.prototype.hasOwnProperty.call(q, Y) && (z[Y] = q[Y]);
            }),
          E(R, T)
        );
      };
      return function (R, T) {
        if (typeof T != 'function' && T !== null)
          throw new TypeError('Class extends value ' + String(T) + ' is not a constructor or null');
        E(R, T);
        function z() {
          this.constructor = R;
        }
        R.prototype = T === null ? Object.create(T) : ((z.prototype = T.prototype), new z());
      };
    })();
  Object.defineProperty(r, '__esModule', { value: !0 }),
    (r.createJSONRPCNotification =
      r.createJSONRPCRequest =
      r.createJSONRPCSuccessResponse =
      r.createJSONRPCErrorResponse =
      r.JSONRPCErrorCode =
      r.JSONRPCErrorException =
      r.isJSONRPCResponses =
      r.isJSONRPCResponse =
      r.isJSONRPCRequests =
      r.isJSONRPCRequest =
      r.isJSONRPCID =
      r.JSONRPC =
        void 0),
    (r.JSONRPC = '2.0');
  var t = function (E) {
    return typeof E == 'string' || typeof E == 'number' || E === null;
  };
  r.isJSONRPCID = t;
  var n = function (E) {
    return (
      E.jsonrpc === r.JSONRPC && E.method !== void 0 && E.result === void 0 && E.error === void 0
    );
  };
  r.isJSONRPCRequest = n;
  var i = function (E) {
    return Array.isArray(E) && E.every(r.isJSONRPCRequest);
  };
  r.isJSONRPCRequests = i;
  var a = function (E) {
    return (
      E.jsonrpc === r.JSONRPC && E.id !== void 0 && (E.result !== void 0 || E.error !== void 0)
    );
  };
  r.isJSONRPCResponse = a;
  var c = function (E) {
    return Array.isArray(E) && E.every(r.isJSONRPCResponse);
  };
  r.isJSONRPCResponses = c;
  var v = function (E, R, T) {
      var z = { code: E, message: R };
      return T != null && (z.data = T), z;
    },
    w = (function (E) {
      e(R, E);
      function R(T, z, q) {
        var Y = E.call(this, T) || this;
        return Object.setPrototypeOf(Y, R.prototype), (Y.code = z), (Y.data = q), Y;
      }
      return (
        (R.prototype.toObject = function () {
          return v(this.code, this.message, this.data);
        }),
        R
      );
    })(Error);
  (r.JSONRPCErrorException = w),
    (function (E) {
      (E[(E.ParseError = -32700)] = 'ParseError'),
        (E[(E.InvalidRequest = -32600)] = 'InvalidRequest'),
        (E[(E.MethodNotFound = -32601)] = 'MethodNotFound'),
        (E[(E.InvalidParams = -32602)] = 'InvalidParams'),
        (E[(E.InternalError = -32603)] = 'InternalError');
    })(r.JSONRPCErrorCode || (r.JSONRPCErrorCode = {}));
  var y = function (E, R, T, z) {
    return { jsonrpc: r.JSONRPC, id: E, error: v(R, T, z) };
  };
  r.createJSONRPCErrorResponse = y;
  var M = function (E, R) {
    return { jsonrpc: r.JSONRPC, id: E, result: R ?? null };
  };
  r.createJSONRPCSuccessResponse = M;
  var S = function (E, R, T) {
    return { jsonrpc: r.JSONRPC, id: E, method: R, params: T };
  };
  r.createJSONRPCRequest = S;
  var I = function (E, R) {
    return { jsonrpc: r.JSONRPC, method: E, params: R };
  };
  r.createJSONRPCNotification = I;
})(ri);
var ni = {};
Object.defineProperty(ni, '__esModule', { value: !0 });
ni.DefaultErrorCode = void 0;
ni.DefaultErrorCode = 0;
var U1 =
    (ie && ie.__awaiter) ||
    function (r, e, t, n) {
      function i(a) {
        return a instanceof t
          ? a
          : new t(function (c) {
              c(a);
            });
      }
      return new (t || (t = Promise))(function (a, c) {
        function v(M) {
          try {
            y(n.next(M));
          } catch (S) {
            c(S);
          }
        }
        function w(M) {
          try {
            y(n.throw(M));
          } catch (S) {
            c(S);
          }
        }
        function y(M) {
          M.done ? a(M.value) : i(M.value).then(v, w);
        }
        y((n = n.apply(r, e || [])).next());
      });
    },
  z1 =
    (ie && ie.__generator) ||
    function (r, e) {
      var t = {
          label: 0,
          sent: function () {
            if (a[0] & 1) throw a[1];
            return a[1];
          },
          trys: [],
          ops: [],
        },
        n,
        i,
        a,
        c;
      return (
        (c = { next: v(0), throw: v(1), return: v(2) }),
        typeof Symbol == 'function' &&
          (c[Symbol.iterator] = function () {
            return this;
          }),
        c
      );
      function v(y) {
        return function (M) {
          return w([y, M]);
        };
      }
      function w(y) {
        if (n) throw new TypeError('Generator is already executing.');
        for (; c && ((c = 0), y[0] && (t = 0)), t; )
          try {
            if (
              ((n = 1),
              i &&
                (a =
                  y[0] & 2
                    ? i.return
                    : y[0]
                    ? i.throw || ((a = i.return) && a.call(i), 0)
                    : i.next) &&
                !(a = a.call(i, y[1])).done)
            )
              return a;
            switch (((i = 0), a && (y = [y[0] & 2, a.value]), y[0])) {
              case 0:
              case 1:
                a = y;
                break;
              case 4:
                return t.label++, { value: y[1], done: !1 };
              case 5:
                t.label++, (i = y[1]), (y = [0]);
                continue;
              case 7:
                (y = t.ops.pop()), t.trys.pop();
                continue;
              default:
                if (
                  ((a = t.trys),
                  !(a = a.length > 0 && a[a.length - 1]) && (y[0] === 6 || y[0] === 2))
                ) {
                  t = 0;
                  continue;
                }
                if (y[0] === 3 && (!a || (y[1] > a[0] && y[1] < a[3]))) {
                  t.label = y[1];
                  break;
                }
                if (y[0] === 6 && t.label < a[1]) {
                  (t.label = a[1]), (a = y);
                  break;
                }
                if (a && t.label < a[2]) {
                  (t.label = a[2]), t.ops.push(y);
                  break;
                }
                a[2] && t.ops.pop(), t.trys.pop();
                continue;
            }
            y = e.call(r, t);
          } catch (M) {
            (y = [6, M]), (i = 0);
          } finally {
            n = a = 0;
          }
        if (y[0] & 5) throw y[1];
        return { value: y[0] ? y[1] : void 0, done: !0 };
      }
    };
Object.defineProperty(ia, '__esModule', { value: !0 });
ia.JSONRPCClient = void 0;
var rn = ri,
  Ca = ni,
  j1 = (function () {
    function r(e, t) {
      (this._send = e), (this.createID = t), (this.idToResolveMap = new Map()), (this.id = 0);
    }
    return (
      (r.prototype._createID = function () {
        return this.createID ? this.createID() : ++this.id;
      }),
      (r.prototype.timeout = function (e, t) {
        var n = this;
        t === void 0 &&
          (t = function (c) {
            return (0, rn.createJSONRPCErrorResponse)(c, Ca.DefaultErrorCode, 'Request timeout');
          });
        var i = function (c, v) {
            var w = setTimeout(function () {
              c.forEach(function (y) {
                var M = n.idToResolveMap.get(y);
                M && (n.idToResolveMap.delete(y), M(t(y)));
              });
            }, e);
            return v().then(
              function (y) {
                return clearTimeout(w), y;
              },
              function (y) {
                return clearTimeout(w), Promise.reject(y);
              }
            );
          },
          a = function (c, v) {
            var w = (Array.isArray(c) ? c : [c])
              .map(function (y) {
                return y.id;
              })
              .filter(Do);
            return i(w, function () {
              return n.requestAdvanced(c, v);
            });
          };
        return {
          request: function (c, v, w) {
            var y = n._createID();
            return i([y], function () {
              return n.requestWithID(c, v, w, y);
            });
          },
          requestAdvanced: function (c, v) {
            return a(c, v);
          },
        };
      }),
      (r.prototype.request = function (e, t, n) {
        return this.requestWithID(e, t, n, this._createID());
      }),
      (r.prototype.requestWithID = function (e, t, n, i) {
        return U1(this, void 0, void 0, function () {
          var a, c;
          return z1(this, function (v) {
            switch (v.label) {
              case 0:
                return (a = (0, rn.createJSONRPCRequest)(i, e, t)), [4, this.requestAdvanced(a, n)];
              case 1:
                return (
                  (c = v.sent()),
                  c.result !== void 0 && !c.error
                    ? [2, c.result]
                    : c.result === void 0 && c.error
                    ? [
                        2,
                        Promise.reject(
                          new rn.JSONRPCErrorException(c.error.message, c.error.code, c.error.data)
                        ),
                      ]
                    : [2, Promise.reject(new Error('An unexpected error occurred'))]
                );
            }
          });
        });
      }),
      (r.prototype.requestAdvanced = function (e, t) {
        var n = this,
          i = Array.isArray(e);
        Array.isArray(e) || (e = [e]);
        var a = e.filter(function (w) {
            return Do(w.id);
          }),
          c = a.map(function (w) {
            return new Promise(function (y) {
              return n.idToResolveMap.set(w.id, y);
            });
          }),
          v = Promise.all(c).then(function (w) {
            return i || !w.length ? w : w[0];
          });
        return this.send(i ? e : e[0], t).then(
          function () {
            return v;
          },
          function (w) {
            return (
              a.forEach(function (y) {
                n.receive(
                  (0, rn.createJSONRPCErrorResponse)(
                    y.id,
                    Ca.DefaultErrorCode,
                    (w && w.message) || 'Failed to send a request'
                  )
                );
              }),
              v
            );
          }
        );
      }),
      (r.prototype.notify = function (e, t, n) {
        var i = (0, rn.createJSONRPCNotification)(e, t);
        this.send(i, n).then(void 0, function () {});
      }),
      (r.prototype.send = function (e, t) {
        return this._send(e, t);
      }),
      (r.prototype.rejectAllPendingRequests = function (e) {
        this.idToResolveMap.forEach(function (t, n) {
          return t((0, rn.createJSONRPCErrorResponse)(n, Ca.DefaultErrorCode, e));
        }),
          this.idToResolveMap.clear();
      }),
      (r.prototype.receive = function (e) {
        var t = this;
        Array.isArray(e) || (e = [e]),
          e.forEach(function (n) {
            var i = t.idToResolveMap.get(n.id);
            i && (t.idToResolveMap.delete(n.id), i(n));
          });
      }),
      r
    );
  })();
ia.JSONRPCClient = j1;
var Do = function (r) {
    return r != null;
  },
  aa = {},
  Fi =
    (ie && ie.__assign) ||
    function () {
      return (
        (Fi =
          Object.assign ||
          function (r) {
            for (var e, t = 1, n = arguments.length; t < n; t++) {
              e = arguments[t];
              for (var i in e) Object.prototype.hasOwnProperty.call(e, i) && (r[i] = e[i]);
            }
            return r;
          }),
        Fi.apply(this, arguments)
      );
    },
  Lo =
    (ie && ie.__awaiter) ||
    function (r, e, t, n) {
      function i(a) {
        return a instanceof t
          ? a
          : new t(function (c) {
              c(a);
            });
      }
      return new (t || (t = Promise))(function (a, c) {
        function v(M) {
          try {
            y(n.next(M));
          } catch (S) {
            c(S);
          }
        }
        function w(M) {
          try {
            y(n.throw(M));
          } catch (S) {
            c(S);
          }
        }
        function y(M) {
          M.done ? a(M.value) : i(M.value).then(v, w);
        }
        y((n = n.apply(r, e || [])).next());
      });
    },
  qo =
    (ie && ie.__generator) ||
    function (r, e) {
      var t = {
          label: 0,
          sent: function () {
            if (a[0] & 1) throw a[1];
            return a[1];
          },
          trys: [],
          ops: [],
        },
        n,
        i,
        a,
        c;
      return (
        (c = { next: v(0), throw: v(1), return: v(2) }),
        typeof Symbol == 'function' &&
          (c[Symbol.iterator] = function () {
            return this;
          }),
        c
      );
      function v(y) {
        return function (M) {
          return w([y, M]);
        };
      }
      function w(y) {
        if (n) throw new TypeError('Generator is already executing.');
        for (; c && ((c = 0), y[0] && (t = 0)), t; )
          try {
            if (
              ((n = 1),
              i &&
                (a =
                  y[0] & 2
                    ? i.return
                    : y[0]
                    ? i.throw || ((a = i.return) && a.call(i), 0)
                    : i.next) &&
                !(a = a.call(i, y[1])).done)
            )
              return a;
            switch (((i = 0), a && (y = [y[0] & 2, a.value]), y[0])) {
              case 0:
              case 1:
                a = y;
                break;
              case 4:
                return t.label++, { value: y[1], done: !1 };
              case 5:
                t.label++, (i = y[1]), (y = [0]);
                continue;
              case 7:
                (y = t.ops.pop()), t.trys.pop();
                continue;
              default:
                if (
                  ((a = t.trys),
                  !(a = a.length > 0 && a[a.length - 1]) && (y[0] === 6 || y[0] === 2))
                ) {
                  t = 0;
                  continue;
                }
                if (y[0] === 3 && (!a || (y[1] > a[0] && y[1] < a[3]))) {
                  t.label = y[1];
                  break;
                }
                if (y[0] === 6 && t.label < a[1]) {
                  (t.label = a[1]), (a = y);
                  break;
                }
                if (a && t.label < a[2]) {
                  (t.label = a[2]), t.ops.push(y);
                  break;
                }
                a[2] && t.ops.pop(), t.trys.pop();
                continue;
            }
            y = e.call(r, t);
          } catch (M) {
            (y = [6, M]), (i = 0);
          } finally {
            n = a = 0;
          }
        if (y[0] & 5) throw y[1];
        return { value: y[0] ? y[1] : void 0, done: !0 };
      }
    },
  V1 =
    (ie && ie.__spreadArray) ||
    function (r, e, t) {
      if (t || arguments.length === 2)
        for (var n = 0, i = e.length, a; n < i; n++)
          (a || !(n in e)) && (a || (a = Array.prototype.slice.call(e, 0, n)), (a[n] = e[n]));
      return r.concat(a || Array.prototype.slice.call(e));
    };
Object.defineProperty(aa, '__esModule', { value: !0 });
aa.JSONRPCServer = void 0;
var zt = ri,
  J1 = ni,
  H1 = function () {
    return (0, zt.createJSONRPCErrorResponse)(null, zt.JSONRPCErrorCode.ParseError, 'Parse error');
  },
  W1 = function (r) {
    return (0, zt.createJSONRPCErrorResponse)(
      (0, zt.isJSONRPCID)(r.id) ? r.id : null,
      zt.JSONRPCErrorCode.InvalidRequest,
      'Invalid Request'
    );
  },
  G1 = function (r) {
    return (0, zt.createJSONRPCErrorResponse)(
      r,
      zt.JSONRPCErrorCode.MethodNotFound,
      'Method not found'
    );
  },
  K1 = (function () {
    function r(e) {
      e === void 0 && (e = {});
      var t;
      (this.mapErrorToJSONRPCErrorResponse = Q1),
        (this.nameToMethodDictionary = {}),
        (this.middleware = null),
        (this.errorListener = (t = e.errorListener) !== null && t !== void 0 ? t : console.warn);
    }
    return (
      (r.prototype.hasMethod = function (e) {
        return !!this.nameToMethodDictionary[e];
      }),
      (r.prototype.addMethod = function (e, t) {
        this.addMethodAdvanced(e, this.toJSONRPCMethod(t));
      }),
      (r.prototype.toJSONRPCMethod = function (e) {
        return function (t, n) {
          var i = e(t.params, n);
          return Promise.resolve(i).then(function (a) {
            return Y1(t.id, a);
          });
        };
      }),
      (r.prototype.addMethodAdvanced = function (e, t) {
        var n;
        this.nameToMethodDictionary = Fi(
          Fi({}, this.nameToMethodDictionary),
          ((n = {}), (n[e] = t), n)
        );
      }),
      (r.prototype.receiveJSON = function (e, t) {
        var n = this.tryParseRequestJSON(e);
        return n ? this.receive(n, t) : Promise.resolve(H1());
      }),
      (r.prototype.tryParseRequestJSON = function (e) {
        try {
          return JSON.parse(e);
        } catch {
          return null;
        }
      }),
      (r.prototype.receive = function (e, t) {
        return Array.isArray(e) ? this.receiveMultiple(e, t) : this.receiveSingle(e, t);
      }),
      (r.prototype.receiveMultiple = function (e, t) {
        return Lo(this, void 0, void 0, function () {
          var n,
            i = this;
          return qo(this, function (a) {
            switch (a.label) {
              case 0:
                return [
                  4,
                  Promise.all(
                    e.map(function (c) {
                      return i.receiveSingle(c, t);
                    })
                  ),
                ];
              case 1:
                return (
                  (n = a.sent().filter(X1)),
                  n.length === 1 ? [2, n[0]] : n.length ? [2, n] : [2, null]
                );
            }
          });
        });
      }),
      (r.prototype.receiveSingle = function (e, t) {
        return Lo(this, void 0, void 0, function () {
          var n, i;
          return qo(this, function (a) {
            switch (a.label) {
              case 0:
                return (
                  (n = this.nameToMethodDictionary[e.method]),
                  (0, zt.isJSONRPCRequest)(e) ? [3, 1] : [2, W1(e)]
                );
              case 1:
                return [4, this.callMethod(n, e, t)];
              case 2:
                return (i = a.sent()), [2, ev(e, i)];
            }
          });
        });
      }),
      (r.prototype.applyMiddleware = function () {
        for (var e = [], t = 0; t < arguments.length; t++) e[t] = arguments[t];
        this.middleware
          ? (this.middleware = this.combineMiddlewares(V1([this.middleware], e, !0)))
          : (this.middleware = this.combineMiddlewares(e));
      }),
      (r.prototype.combineMiddlewares = function (e) {
        return e.length ? e.reduce(this.middlewareReducer) : null;
      }),
      (r.prototype.middlewareReducer = function (e, t) {
        return function (n, i, a) {
          return e(
            function (c, v) {
              return t(n, c, v);
            },
            i,
            a
          );
        };
      }),
      (r.prototype.callMethod = function (e, t, n) {
        var i = this,
          a = function (v, w) {
            return e
              ? e(v, w)
              : v.id !== void 0
              ? Promise.resolve(G1(v.id))
              : Promise.resolve(null);
          },
          c = function (v) {
            return (
              i.errorListener(
                'An unexpected error occurred while executing "'.concat(
                  t.method,
                  '" JSON-RPC method:'
                ),
                v
              ),
              Promise.resolve(i.mapErrorToJSONRPCErrorResponseIfNecessary(t.id, v))
            );
          };
        try {
          return (this.middleware || Z1)(a, t, n).then(void 0, c);
        } catch (v) {
          return c(v);
        }
      }),
      (r.prototype.mapErrorToJSONRPCErrorResponseIfNecessary = function (e, t) {
        return e !== void 0 ? this.mapErrorToJSONRPCErrorResponse(e, t) : null;
      }),
      r
    );
  })();
aa.JSONRPCServer = K1;
var X1 = function (r) {
    return r !== null;
  },
  Z1 = function (r, e, t) {
    return r(e, t);
  },
  Y1 = function (r, e) {
    return r !== void 0 ? (0, zt.createJSONRPCSuccessResponse)(r, e) : null;
  },
  Q1 = function (r, e) {
    var t,
      n = (t = e?.message) !== null && t !== void 0 ? t : 'An unexpected error occurred',
      i = J1.DefaultErrorCode,
      a;
    return (
      e instanceof zt.JSONRPCErrorException && ((i = e.code), (a = e.data)),
      (0, zt.createJSONRPCErrorResponse)(r, i, n, a)
    );
  },
  ev = function (r, e) {
    return (
      e ||
      (r.id !== void 0
        ? (0, zt.createJSONRPCErrorResponse)(
            r.id,
            zt.JSONRPCErrorCode.InternalError,
            'Internal error'
          )
        : null)
    );
  },
  sa = {},
  tv =
    (ie && ie.__awaiter) ||
    function (r, e, t, n) {
      function i(a) {
        return a instanceof t
          ? a
          : new t(function (c) {
              c(a);
            });
      }
      return new (t || (t = Promise))(function (a, c) {
        function v(M) {
          try {
            y(n.next(M));
          } catch (S) {
            c(S);
          }
        }
        function w(M) {
          try {
            y(n.throw(M));
          } catch (S) {
            c(S);
          }
        }
        function y(M) {
          M.done ? a(M.value) : i(M.value).then(v, w);
        }
        y((n = n.apply(r, e || [])).next());
      });
    },
  rv =
    (ie && ie.__generator) ||
    function (r, e) {
      var t = {
          label: 0,
          sent: function () {
            if (a[0] & 1) throw a[1];
            return a[1];
          },
          trys: [],
          ops: [],
        },
        n,
        i,
        a,
        c;
      return (
        (c = { next: v(0), throw: v(1), return: v(2) }),
        typeof Symbol == 'function' &&
          (c[Symbol.iterator] = function () {
            return this;
          }),
        c
      );
      function v(y) {
        return function (M) {
          return w([y, M]);
        };
      }
      function w(y) {
        if (n) throw new TypeError('Generator is already executing.');
        for (; c && ((c = 0), y[0] && (t = 0)), t; )
          try {
            if (
              ((n = 1),
              i &&
                (a =
                  y[0] & 2
                    ? i.return
                    : y[0]
                    ? i.throw || ((a = i.return) && a.call(i), 0)
                    : i.next) &&
                !(a = a.call(i, y[1])).done)
            )
              return a;
            switch (((i = 0), a && (y = [y[0] & 2, a.value]), y[0])) {
              case 0:
              case 1:
                a = y;
                break;
              case 4:
                return t.label++, { value: y[1], done: !1 };
              case 5:
                t.label++, (i = y[1]), (y = [0]);
                continue;
              case 7:
                (y = t.ops.pop()), t.trys.pop();
                continue;
              default:
                if (
                  ((a = t.trys),
                  !(a = a.length > 0 && a[a.length - 1]) && (y[0] === 6 || y[0] === 2))
                ) {
                  t = 0;
                  continue;
                }
                if (y[0] === 3 && (!a || (y[1] > a[0] && y[1] < a[3]))) {
                  t.label = y[1];
                  break;
                }
                if (y[0] === 6 && t.label < a[1]) {
                  (t.label = a[1]), (a = y);
                  break;
                }
                if (a && t.label < a[2]) {
                  (t.label = a[2]), t.ops.push(y);
                  break;
                }
                a[2] && t.ops.pop(), t.trys.pop();
                continue;
            }
            y = e.call(r, t);
          } catch (M) {
            (y = [6, M]), (i = 0);
          } finally {
            n = a = 0;
          }
        if (y[0] & 5) throw y[1];
        return { value: y[0] ? y[1] : void 0, done: !0 };
      }
    };
Object.defineProperty(sa, '__esModule', { value: !0 });
sa.JSONRPCServerAndClient = void 0;
var pi = ri,
  nv = (function () {
    function r(e, t, n) {
      n === void 0 && (n = {});
      var i;
      (this.server = e),
        (this.client = t),
        (this.errorListener = (i = n.errorListener) !== null && i !== void 0 ? i : console.warn);
    }
    return (
      (r.prototype.applyServerMiddleware = function () {
        for (var e, t = [], n = 0; n < arguments.length; n++) t[n] = arguments[n];
        (e = this.server).applyMiddleware.apply(e, t);
      }),
      (r.prototype.hasMethod = function (e) {
        return this.server.hasMethod(e);
      }),
      (r.prototype.addMethod = function (e, t) {
        this.server.addMethod(e, t);
      }),
      (r.prototype.addMethodAdvanced = function (e, t) {
        this.server.addMethodAdvanced(e, t);
      }),
      (r.prototype.timeout = function (e) {
        return this.client.timeout(e);
      }),
      (r.prototype.request = function (e, t, n) {
        return this.client.request(e, t, n);
      }),
      (r.prototype.requestAdvanced = function (e, t) {
        return this.client.requestAdvanced(e, t);
      }),
      (r.prototype.notify = function (e, t, n) {
        this.client.notify(e, t, n);
      }),
      (r.prototype.rejectAllPendingRequests = function (e) {
        this.client.rejectAllPendingRequests(e);
      }),
      (r.prototype.receiveAndSend = function (e, t, n) {
        return tv(this, void 0, void 0, function () {
          var i, a;
          return rv(this, function (c) {
            switch (c.label) {
              case 0:
                return (0, pi.isJSONRPCResponse)(e) || (0, pi.isJSONRPCResponses)(e)
                  ? (this.client.receive(e), [3, 4])
                  : [3, 1];
              case 1:
                return (0, pi.isJSONRPCRequest)(e) || (0, pi.isJSONRPCRequests)(e)
                  ? [4, this.server.receive(e, t)]
                  : [3, 3];
              case 2:
                return (i = c.sent()), i ? [2, this.client.send(i, n)] : [3, 4];
              case 3:
                return (
                  (a = 'Received an invalid JSON-RPC message'),
                  this.errorListener(a, e),
                  [2, Promise.reject(new Error(a))]
                );
              case 4:
                return [2];
            }
          });
        });
      }),
      r
    );
  })();
sa.JSONRPCServerAndClient = nv;
(function (r) {
  var e =
      (ie && ie.__createBinding) ||
      (Object.create
        ? function (n, i, a, c) {
            c === void 0 && (c = a);
            var v = Object.getOwnPropertyDescriptor(i, a);
            (!v || ('get' in v ? !i.__esModule : v.writable || v.configurable)) &&
              (v = {
                enumerable: !0,
                get: function () {
                  return i[a];
                },
              }),
              Object.defineProperty(n, c, v);
          }
        : function (n, i, a, c) {
            c === void 0 && (c = a), (n[c] = i[a]);
          }),
    t =
      (ie && ie.__exportStar) ||
      function (n, i) {
        for (var a in n)
          a !== 'default' && !Object.prototype.hasOwnProperty.call(i, a) && e(i, n, a);
      };
  Object.defineProperty(r, '__esModule', { value: !0 }), t(ia, r), t(ri, r), t(aa, r), t(sa, r);
})(F1);
export {
  er as A,
  md as B,
  Nt as H,
  j0 as J,
  Wn as N,
  Mv as Q,
  nf as R,
  Rt as V,
  zl as W,
  je as X,
  Cr as Z,
  Ni as _,
  Sv as a,
  Av as b,
  ie as c,
  ql as d,
  vf as e,
  Yv as f,
  Kf as g,
  _v as h,
  B as i,
  yt as j,
  F1 as k,
  gi as s,
  uf as v,
  Bn as y,
};
