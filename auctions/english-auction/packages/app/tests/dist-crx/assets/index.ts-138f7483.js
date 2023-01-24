import {
  W as l,
  b as S,
  T as A,
  w as U,
  t as T,
  c as r,
  N as f,
  C as c,
  A as P,
  P as h,
  B as L,
  i as v,
} from './BaseConnection-b6f97547.js';
import { P as q, C as w, B as C } from './constants-97362116.js';
import { y as I, t as E } from './index-0339478c.js';
async function k() {
  let n = 0,
    e = 0;
  try {
    const { top: t = 0, left: s = 0, width: o = 0 } = await chrome.windows.getLastFocused();
    (e = t), (n = s + (o - l));
  } catch {
    const { screenX: s, screenY: o, outerWidth: u } = window;
    (e = Math.max(o, 0)), (n = Math.max(s + (u - l), 0));
  }
  return { left: n, top: e };
}
async function N(n) {
  return (await chrome.tabs.query({ windowId: n }))?.[0].id || null;
}
async function R(n) {
  if (!n) return null;
  try {
    if (await chrome.windows.get(n)) return await chrome.windows.update(n, { focused: !0 });
  } catch {}
  return null;
}
async function x(n, e) {
  const { left: t, top: s } = await k();
  return await chrome.windows.create({
    type: 'popup',
    url: e,
    width: l,
    height: S + A,
    left: t,
    top: s,
  });
}
function _(n) {
  return n?.tab?.id;
}
let d;
const j = new Uint8Array(16);
function O() {
  if (
    !d &&
    ((d = typeof crypto < 'u' && crypto.getRandomValues && crypto.getRandomValues.bind(crypto)), !d)
  )
    throw new Error(
      'crypto.getRandomValues() not supported. See https://github.com/uuidjs/uuid#getrandomvalues-not-supported'
    );
  return d(j);
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
  b = { randomUUID: W };
function B(n, e, t) {
  if (b.randomUUID && !e && !n) return b.randomUUID();
  n = n || {};
  const s = n.random || (n.rng || O)();
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
  n.reason === chrome.runtime.OnInstalledReason.INSTALL && chrome.tabs.create({ url: U() });
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
const M = new Map(),
  p = class {
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
        (this.client = new I.JSONRPCClient(async (e) => {
          if (this.eventId)
            this.communicationProtocol.postMessage({
              type: r.request,
              target: q,
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
      const e = M.get(n);
      return e && (await R(e.windowId)) ? e : null;
    }
    async requestConnection(n) {
      return this.client.request('requestConnection', { origin: n });
    }
    async signMessage(n, e) {
      return this.client.request('signMessage', { origin: n, message: e });
    }
    async sendTransaction(n, e, t) {
      if ((await f.getSelectedNetwork())?.url !== e.url)
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
let a = p;
a.create = async (n, e, t) => {
  const s = new p(t);
  M.set(n, s);
  const o = await x(n, `${T.popup}#${e}`);
  return (s.tabId = await N(o.id)), (s.windowId = o.id), s;
};
a.open = async (n, e, t) => {
  let s = await p.getCurrent(n);
  return s || (s = await p.create(n, e, t)), s.openingPromise.promise;
};
class m {
  constructor(e) {
    (this.communicationProtocol = e),
      (this.server = new I.JSONRPCServer()),
      this.server.applyMiddleware(this.connectionMiddlware.bind(this)),
      this.setupListeners(),
      this.externalMethods([
        this.isConnected,
        this.accounts,
        this.connect,
        this.network,
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
          target: w,
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
    return ((await c.getConnection(e))?.accounts || []).length > 0;
  }
  async requireAccounts() {
    if ((await P.getAccounts()).length === 0)
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
      target: w,
      type: r.event,
      events: [{ event: t, params: s }],
    });
  }
  async connect(e, t) {
    const s = t.origin;
    let o = await c.getConnection(s);
    return (
      o ||
        (o = await (
          await a.open(s, h.requestConnection(), this.communicationProtocol)
        ).requestConnection(s)),
      o && this.sendEvent(s, 'connection', [!!o]),
      !!o
    );
  }
  async disconnect(e, t) {
    const s = t.origin;
    return s
      ? (await c.removeConnection({ origin: s }), this.sendEvent(s, 'connection', [!1]), !0)
      : !1;
  }
  async accounts(e, t) {
    const s = t.origin;
    return s ? (await c.getConnection(s))?.accounts || [] : [];
  }
  async signMessage({ message: e }, t) {
    const s = t.origin;
    return await (
      await a.open(s, h.requestMessage(), this.communicationProtocol)
    ).signMessage(s, e);
  }
  async sendTransaction({ provider: e, transaction: t }, s) {
    const o = s.origin;
    return await (
      await a.open(o, h.requestTransaction(), this.communicationProtocol)
    ).sendTransaction(o, e, t);
  }
  async getSelectedAccount() {
    return (await P.getSelectedAccount())?.address;
  }
  async network() {
    return { url: (await f.getSelectedNetwork())?.url };
  }
}
class V extends L {
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
        const s = Array.isArray(e) ? e : [e];
        this.ports.forEach((o) => {
          s.includes(o.sender?.origin || '') && o.postMessage(t);
        });
      }),
      (this.getPortId = (e) => {
        for (const [t, s] of this.ports.entries()) if (s === e) return t;
        return null;
      }),
      (this.onMessage = (e, t) => {
        if (
          t.sender?.id !== chrome.runtime.id ||
          e.target !== C ||
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
class G extends E {
  constructor() {
    super(), this.setupListeners();
  }
  setupListeners() {
    v.on('changes', (e) => {
      e.forEach((t) => {
        switch (t.type) {
          case 1:
            super.emit(`${t.table}:create`, t);
            break;
          case 2:
            super.emit(`${t.table}:update`, t);
            break;
          case 3:
            super.emit(`${t.table}:delete`, t);
            break;
        }
      });
    }),
      v.open();
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
  createEvents(e) {
    return { target: w, type: r.event, events: e };
  }
  setupApplicationWatcher() {
    this.databaseObservable.on('networks:update', async (e) => {
      const s = (await c.getConnections()).map((o) => o.origin);
      e.obj.isSelected &&
        this.communicationProtocol.broadcast(
          s,
          this.createEvents([{ event: 'network', params: [{ id: e.obj.id, url: e.obj.url }] }])
        );
    });
  }
}
const y = new V();
m.start(y);
g.start(y);
chrome.runtime.onConnect.addListener((n) => {
  n.name === C && y.addConnection(n);
});
