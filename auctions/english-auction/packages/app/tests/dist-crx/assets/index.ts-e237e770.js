import {
  W as l,
  b as M,
  T as S,
  w as T,
  t as U,
  c as r,
  N as P,
  A as b,
  C as a,
  P as h,
  B as q,
  i as f,
} from './BaseConnection-869e05f7.js';
import { P as L, C as w, B as C } from './constants-97362116.js';
import { y as I, f as E, t as k } from './index-0339478c.js';
async function N() {
  let n = 0,
    e = 0;
  try {
    const { top: t = 0, left: o = 0, width: s = 0 } = await chrome.windows.getLastFocused();
    (e = t), (n = o + (s - l));
  } catch {
    const { screenX: o, screenY: s, outerWidth: c } = window;
    (e = Math.max(s, 0)), (n = Math.max(o + (c - l), 0));
  }
  return { left: n, top: e };
}
async function R(n) {
  return (await chrome.tabs.query({ windowId: n }))?.[0].id || null;
}
async function j(n) {
  if (!n) return null;
  try {
    if (await chrome.windows.get(n)) return await chrome.windows.update(n, { focused: !0 });
  } catch {}
  return null;
}
async function x(n, e) {
  const { left: t, top: o } = await N();
  return await chrome.windows.create({
    type: 'popup',
    url: e,
    width: l,
    height: M + S,
    left: t,
    top: o,
  });
}
function _(n) {
  return n?.tab?.id;
}
let d;
const O = new Uint8Array(16);
function D() {
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
function W(n, e = 0) {
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
const B = typeof crypto < 'u' && crypto.randomUUID && crypto.randomUUID.bind(crypto),
  v = { randomUUID: B };
function H(n, e, t) {
  if (v.randomUUID && !e && !n) return v.randomUUID();
  n = n || {};
  const o = n.random || (n.rng || D)();
  if (((o[6] = (o[6] & 15) | 64), (o[8] = (o[8] & 63) | 128), e)) {
    t = t || 0;
    for (let s = 0; s < 16; ++s) e[t + s] = o[s];
    return e;
  }
  return W(o);
}
function $() {
  return H();
}
chrome.runtime.onInstalled.addListener((n) => {
  n.reason === chrome.runtime.OnInstalledReason.INSTALL && chrome.tabs.create({ url: T() });
});
function V() {
  const n = {};
  return (
    (n.promise = new Promise((e, t) => {
      (n.reject = t), (n.resolve = e);
    })),
    n
  );
}
const A = new Map(),
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
        (this.openingPromise = V()),
        (this.client = new I.JSONRPCClient(async (e) => {
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
      const e = A.get(n);
      return e && (await j(e.windowId)) ? e : null;
    }
    async requestConnection(n) {
      return this.client.request('requestConnection', { origin: n });
    }
    async signMessage(n) {
      return this.client.request('signMessage', n);
    }
    async sendTransaction(n) {
      if ((await P.getSelectedNetwork())?.url !== n.provider.url)
        throw new Error(
          [
            `${n.provider.url} is different from the user current network!`,
            'Request the user to add the new network. fuel.addNetwork([...]).',
          ].join(`
`)
        );
      return this.client.request('sendTransaction', n);
    }
  };
let u = p;
u.create = async (n, e, t) => {
  const o = new p(t);
  A.set(n, o);
  const s = await x(n, `${U.popup}#${e}`);
  return (o.tabId = await R(s.id)), (o.windowId = s.id), o;
};
u.open = async (n, e, t) => {
  let o = await p.getCurrent(n);
  return o || (o = await p.create(n, e, t)), o.openingPromise.promise;
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
        this.currentAccount,
      ]);
  }
  static start(e) {
    return new m(e);
  }
  setupListeners() {
    this.communicationProtocol.on(r.request, async (e) => {
      const t = e.sender.origin,
        o = await this.server.receive(e.request, { origin: t });
      o &&
        this.communicationProtocol.postMessage({
          id: e.id,
          type: r.response,
          target: w,
          response: o,
        });
    });
  }
  externalMethods(e) {
    e.forEach((t) => {
      let o = t;
      t.name && (o = t.name), this.server.addMethod(o, this[o].bind(this));
    });
  }
  async requireAccounts() {
    if ((await b.getAccounts()).length === 0)
      throw new Error('Unable to establish a connection. No accounts found');
  }
  async requireAccountConnecton(e, t) {
    if (!e) throw new Error('connection not found');
    if (!e.accounts.includes(E.fromString(t || '0x00').toString()))
      throw new Error('address is not authorized for this connection.');
  }
  async requireConnection(e) {
    if (!((e?.accounts || []).length > 0))
      throw new Error(
        'Connection not established. Please call connect() first to request a connection'
      );
  }
  async connectionMiddlware(e, t, o) {
    const s = await a.getConnection(o.origin);
    return (
      ['connect', 'isConnected'].includes(t.method)
        ? await this.requireAccounts()
        : await this.requireConnection(s),
      e(t, { connection: s, origin: o.origin })
    );
  }
  async sendEvent(e, t, o) {
    this.communicationProtocol.broadcast(e, {
      target: w,
      type: r.event,
      events: [{ event: t, params: o }],
    });
  }
  async isConnected(e, t) {
    return !!t.connection;
  }
  async connect(e, t) {
    const o = t.origin;
    let s = await a.getConnection(o);
    return (
      s ||
        (s = await (
          await u.open(o, h.requestConnection(), this.communicationProtocol)
        ).requestConnection(o)),
      s && this.sendEvent(o, 'connection', [!!s]),
      !!s
    );
  }
  async disconnect(e, t) {
    const o = t.origin;
    return o
      ? (await a.removeConnection({ origin: o }), this.sendEvent(o, 'connection', [!1]), !0)
      : !1;
  }
  async accounts(e, t) {
    const o = t.origin;
    return o ? (await a.getConnection(o))?.accounts || [] : [];
  }
  async signMessage(e, t) {
    const o = t.origin;
    return (
      await this.requireAccountConnecton(t.connection, e.address),
      await (
        await u.open(o, h.requestMessage(), this.communicationProtocol)
      ).signMessage({ ...e, origin: o })
    );
  }
  async sendTransaction(e, t) {
    const o = t.origin;
    return (
      await this.requireAccountConnecton(t.connection, e.address),
      await (
        await u.open(o, h.requestTransaction(), this.communicationProtocol)
      ).sendTransaction({ ...e, origin: o })
    );
  }
  async currentAccount(e, t) {
    const o = await b.getCurrentAccount();
    return await this.requireAccountConnecton(t.connection, o?.address), o?.address;
  }
  async network() {
    return { url: (await P.getSelectedNetwork())?.url };
  }
}
class z extends q {
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
        const o = Array.isArray(e) ? e : [e];
        this.ports.forEach((s) => {
          o.includes(s.sender?.origin || '') && s.postMessage(t);
        });
      }),
      (this.getPortId = (e) => {
        for (const [t, o] of this.ports.entries()) if (o === e) return t;
        return null;
      }),
      (this.onMessage = (e, t) => {
        if (
          t.sender?.id !== chrome.runtime.id ||
          e.target !== C ||
          !Object.keys(r).includes(e.type)
        )
          return;
        const s = this.getPortId(t);
        this.emit(e.type, Object.freeze({ ...e, id: s, sender: t.sender }));
      }),
      (this.ports = new Map());
  }
  addConnection(e) {
    const t = $();
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
class G extends k {
  constructor() {
    super(), this.setupListeners();
  }
  setupListeners() {
    f.on('changes', (e) => {
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
      f.open();
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
      if (!e.obj.isSelected) return;
      const o = (await a.getConnections()).map((s) => s.origin);
      this.communicationProtocol.broadcast(
        o,
        this.createEvents([{ event: 'network', params: [{ id: e.obj.id, url: e.obj.url }] }])
      );
    }),
      this.databaseObservable.on('accounts:update', async (e) => {
        if (!e.obj.isCurrent) return;
        const t = e.obj,
          s = (await a.getConnections())
            .filter((c) => c.accounts.includes(t?.address || ''))
            .map((c) => c.origin);
        this.communicationProtocol.broadcast(
          s,
          this.createEvents([{ event: 'currentAccount', params: [e.obj.address] }])
        );
      });
  }
}
const y = new z();
m.start(y);
g.start(y);
chrome.runtime.onConnect.addListener((n) => {
  n.name === C && y.addConnection(n);
});
