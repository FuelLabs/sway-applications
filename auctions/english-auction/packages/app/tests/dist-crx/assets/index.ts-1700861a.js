import {
  W as w,
  b as I,
  T as M,
  w as S,
  q as U,
  c as r,
  N as A,
  C as u,
  A as v,
  P as l,
  B as T,
  d as q,
} from './BaseConnection-6cf63181.js';
import { P as L, C as p, B as b } from './constants-97362116.js';
import { k as f, e as E } from './index-ba02b8c1.js';
async function R() {
  let n = 0,
    e = 0;
  try {
    const { top: t = 0, left: s = 0, width: o = 0 } = await chrome.windows.getLastFocused();
    (e = t), (n = s + (o - w));
  } catch {
    const { screenX: s, screenY: o, outerWidth: a } = window;
    (e = Math.max(o, 0)), (n = Math.max(s + (a - w), 0));
  }
  return { left: n, top: e };
}
async function N(n) {
  return (await chrome.tabs.query({ windowId: n }))?.[0].id || null;
}
async function k(n) {
  if (!n) return null;
  try {
    if (await chrome.windows.get(n)) return await chrome.windows.update(n, { focused: !0 });
  } catch {}
  return null;
}
async function x(n, e) {
  const { left: t, top: s } = await R();
  return await chrome.windows.create({
    type: 'popup',
    url: e,
    width: w,
    height: I + M,
    left: t,
    top: s,
  });
}
function _(n) {
  return n?.tab?.id;
}
let d;
const O = new Uint8Array(16);
function j() {
  if (
    !d &&
    ((d = typeof crypto < 'u' && crypto.getRandomValues && crypto.getRandomValues.bind(crypto)), !d)
  )
    throw new Error(
      'crypto.getRandomValues() not supported. See https://github.com/uuidjs/uuid#getrandomvalues-not-supported'
    );
  return d(O);
}
const i = [];
for (let n = 0; n < 256; ++n) i.push((n + 256).toString(16).slice(1));
function D(n, e = 0) {
  return (
    i[n[e + 0]] +
    i[n[e + 1]] +
    i[n[e + 2]] +
    i[n[e + 3]] +
    '-' +
    i[n[e + 4]] +
    i[n[e + 5]] +
    '-' +
    i[n[e + 6]] +
    i[n[e + 7]] +
    '-' +
    i[n[e + 8]] +
    i[n[e + 9]] +
    '-' +
    i[n[e + 10]] +
    i[n[e + 11]] +
    i[n[e + 12]] +
    i[n[e + 13]] +
    i[n[e + 14]] +
    i[n[e + 15]]
  ).toLowerCase();
}
const W = typeof crypto < 'u' && crypto.randomUUID && crypto.randomUUID.bind(crypto),
  P = { randomUUID: W };
function B(n, e, t) {
  if (P.randomUUID && !e && !n) return P.randomUUID();
  n = n || {};
  const s = n.random || (n.rng || j)();
  if (((s[6] = (s[6] & 15) | 64), (s[8] = (s[8] & 63) | 128), e)) {
    t = t || 0;
    for (let o = 0; o < 16; ++o) e[t + o] = s[o];
    return e;
  }
  return D(s);
}
function H() {
  return B();
}
chrome.runtime.onInstalled.addListener((n) => {
  n.reason === chrome.runtime.OnInstalledReason.INSTALL && chrome.tabs.create({ url: S() });
});
function $() {
  const n = {};
  return (
    (n.promise = new Promise((e, t) => {
      (n.reject = t), (n.resolve = e);
    })),
    n
  );
}
const C = new Map(),
  h = class {
    constructor(n) {
      (this.tabId = null),
        (this.windowId = null),
        (this.rejectAllRequests = (e) => {
          e === this.eventId &&
            this.client.rejectAllPendingRequests('Request cancelled without explicity response!');
        }),
        (this.onResponse = (e) => {
          e.id === this.eventId && e.response && this.client.receive(e.response);
        }),
        (this.onUIEvent = (e) => {
          _(e.sender) === this.tabId &&
            e.ready &&
            ((this.eventId = e.id), this.openingPromise.resolve(this));
        }),
        (this.communicationProtocol = n),
        (this.openingPromise = $()),
        (this.client = new f.JSONRPCClient(async (e) => {
          if (this.eventId)
            this.communicationProtocol.postMessage({
              type: r.request,
              target: L,
              id: this.eventId,
              request: e,
            });
          else throw new Error('UI not connected!');
        })),
        this.setupUIListeners(),
        this.setTimeout();
    }
    setTimeout(n = 5e3) {
      setTimeout(() => {
        this.openingPromise.reject(new Error('PopUp not opened!'));
      }, n);
    }
    setupUIListeners() {
      this.communicationProtocol.once(r.uiEvent, this.onUIEvent),
        this.communicationProtocol.on(r.response, this.onResponse),
        this.communicationProtocol.on(r.removeConnection, this.rejectAllRequests);
    }
    static async getCurrent(n) {
      const e = C.get(n);
      return e && (await k(e.windowId)) ? e : null;
    }
    async requestConnection(n) {
      return this.client.request('requestConnection', { origin: n });
    }
    async signMessage(n, e) {
      return this.client.request('signMessage', { origin: n, message: e });
    }
    async sendTransaction(n, e, t) {
      if ((await A.getSelectedNetwork())?.url !== e.url)
        throw new Error(
          [
            `${e.url} is different from the user current network!`,
            'Request the user to add the new network. fuel.addNetwork([...]).',
          ].join(`
`)
        );
      return this.client.request('sendTransaction', { origin: n, transaction: t, provider: e });
    }
  };
let c = h;
c.create = async (n, e, t) => {
  const s = new h(t);
  C.set(n, s);
  const o = await x(n, `${U.popup}#${e}`);
  return (s.tabId = await N(o.id)), (s.windowId = o.id), s;
};
c.open = async (n, e, t) => {
  let s = await h.getCurrent(n);
  return s || (s = await h.create(n, e, t)), s.openingPromise.promise;
};
class m {
  constructor(e) {
    (this.communicationProtocol = e),
      (this.server = new f.JSONRPCServer()),
      this.server.applyMiddleware(this.connectionMiddlware.bind(this)),
      this.setupListeners(),
      this.externalMethods([
        this.isConnected,
        this.accounts,
        this.connect,
        this.disconnect,
        this.signMessage,
        this.sendTransaction,
        this.getSelectedAccount,
      ]);
  }
  static start(e) {
    return new m(e);
  }
  setupListeners() {
    this.communicationProtocol.on(r.request, async (e) => {
      const t = e.sender.origin,
        s = await this.server.receive(e.request, { origin: t });
      s &&
        this.communicationProtocol.postMessage({
          id: e.id,
          type: r.response,
          target: p,
          response: s,
        });
    });
  }
  externalMethods(e) {
    e.forEach((t) => {
      let s = t;
      t.name && (s = t.name), this.server.addMethod(s, this[s].bind(this));
    });
  }
  async isConnected(e) {
    return !!(await u.getConnection(e));
  }
  async requireAccounts() {
    if ((await v.getAccounts()).length === 0)
      throw new Error('Unable to establish a connection. No accounts found');
  }
  async requireConnection(e) {
    if (!(await this.isConnected(e)))
      throw new Error(
        'Connection not established. Please call connect() first to request a connection'
      );
  }
  async connectionMiddlware(e, t, s) {
    return (
      ['connect', 'isConnected'].includes(t.method)
        ? await this.requireAccounts()
        : await this.requireConnection(s.origin),
      e(t, s)
    );
  }
  async sendEvent(e, t, s) {
    this.communicationProtocol.broadcast(e, {
      target: p,
      type: r.event,
      events: [{ event: t, params: s }],
    });
  }
  async connect(e, t) {
    const s = t.origin;
    let o = await u.getConnection(s);
    return (
      o ||
        (o = await (
          await c.open(s, l.requestConnection(), this.communicationProtocol)
        ).requestConnection(s)),
      o && this.sendEvent(s, 'connection', [!!o]),
      !!o
    );
  }
  async disconnect(e, t) {
    const s = t.origin;
    return s ? (await u.removeConnection(s), this.sendEvent(s, 'connection', [!1]), !0) : !1;
  }
  async accounts(e, t) {
    const s = t.origin;
    return s ? (await u.getConnection(s))?.accounts || [] : [];
  }
  async signMessage({ message: e }, t) {
    const s = t.origin;
    return await (
      await c.open(s, l.requestMessage(), this.communicationProtocol)
    ).signMessage(s, e);
  }
  async sendTransaction({ provider: e, transaction: t }, s) {
    const o = s.origin;
    return await (
      await c.open(o, l.requestTransaction(), this.communicationProtocol)
    ).sendTransaction(o, e, t);
  }
  async getSelectedAccount() {
    return (await v.getSelectedAccount())?.address;
  }
}
class V extends T {
  constructor() {
    super(),
      (this.removePort = (e) => {
        const t = this.ports.get(e);
        t &&
          (t.onMessage.removeListener(this.onMessage),
          this.ports.delete(e),
          this.emit(r.removeConnection, e));
      }),
      (this.postMessage = (e) => {
        const t = this.ports.get(e.id);
        t && t.postMessage(e);
      }),
      (this.broadcast = (e, t) => {
        this.ports.forEach((s) => {
          s.sender?.origin === e && s.postMessage(t);
        });
      }),
      (this.getPortId = (e) => {
        for (const [t, s] of this.ports.entries()) if (s === e) return t;
        return null;
      }),
      (this.onMessage = (e, t) => {
        if (
          t.sender?.id !== chrome.runtime.id ||
          e.target !== b ||
          !Object.keys(r).includes(e.type)
        )
          return;
        const o = this.getPortId(t);
        this.emit(e.type, Object.freeze({ ...e, id: o, sender: t.sender }));
      }),
      (this.ports = new Map());
  }
  addConnection(e) {
    const t = H();
    this.ports.set(t, e), this.setupListeners(t);
  }
  setupListeners(e) {
    const t = this.ports.get(e);
    t &&
      !t.onMessage.hasListener(this.onMessage) &&
      (t.onMessage.addListener(this.onMessage),
      t.onDisconnect.addListener(() => this.removePort(e)));
  }
  on(e, t) {
    return super.on(e, t);
  }
  destroy() {
    this.ports.forEach((e) => e.disconnect()), this.ports.clear();
  }
}
class G extends E.exports {
  constructor() {
    super(), this.setupListeners();
  }
  setupListeners() {
    q.on('changes', (e) => {
      e.forEach((t) => {
        switch (t.type) {
          case 1:
            super.emit(`${t.table}:create`, t);
            break;
          case 3:
            super.emit(`${t.table}:delete`, t);
            break;
        }
      });
    });
  }
  on(e, t) {
    return super.on(e, t);
  }
}
class g {
  constructor(e) {
    (this.communicationProtocol = e),
      (this.databaseObservable = new G()),
      this.setupApplicationWatcher();
  }
  static start(e) {
    return new g(e);
  }
  setupApplicationWatcher() {
    this.databaseObservable.on('applications:create', (e) => {
      this.communicationProtocol.broadcast(e.key, {
        target: p,
        type: r.event,
        events: [
          { event: 'accounts', params: [e.obj.accounts] },
          { event: 'connection', params: [!0] },
        ],
      });
    }),
      this.databaseObservable.on('applications:delete', (e) => {
        this.communicationProtocol.broadcast(e.key, {
          target: p,
          type: r.event,
          events: [{ event: 'connection', params: [!1] }],
        });
      });
  }
}
const y = new V();
m.start(y);
g.start(y);
chrome.runtime.onConnect.addListener((n) => {
  n.name === b && y.addConnection(n);
});
