import {
  f as a,
  x as c,
  n as u,
  z as d,
  V as i,
  w as l,
  t as h,
  y as o,
} from './index-0339478c.js';
var p = 'Fuel',
  v = 'FuelContentScript',
  w = 'message',
  g = class extends h {
    constructor() {
      super(),
        (this.onCommunicationMessage = (e) => {
          switch (e.type) {
            case 'response':
              this.onResponse(e);
              break;
            case 'request':
              this.onRequest(e);
              break;
            case 'event':
              this.onEvent(e);
              break;
            case 'uiEvent':
              this.onUIEvent(e);
              break;
          }
        }),
        (this.client = new o.JSONRPCClient(this.sendRequest.bind(this))),
        (this.server = new o.JSONRPCServer());
    }
    externalMethods(e) {
      e.forEach((n) => {
        let t = n;
        n.name && (t = n.name), this.server.addMethod(t, this[t].bind(this));
      });
    }
    async sendRequest(e) {
      throw new Error('Send request not implemented');
    }
    sendResponse(e, n) {
      throw new Error('Send response not implemented');
    }
    onEvent(e) {
      e.events.forEach((n) => {
        this.emit(n.event, ...n.params);
      });
    }
    onResponse(e) {
      this.client.receive(e.response);
    }
    onRequest(e) {
      this.server.receive(e.request).then((n) => {
        this.sendResponse(n, e);
      });
    }
    onUIEvent(e) {}
  },
  f = class extends g {
    constructor() {
      super(),
        (this.onMessage = (e) => {
          const n = Object.freeze(e);
          if (!this.acceptMessage(n)) return;
          const { data: t } = n;
          this.onCommunicationMessage(t);
        }),
        window.addEventListener(w, this.onMessage.bind(this));
    }
    acceptMessage(e) {
      return !0;
    }
    postMessage(e, n) {
      window.postMessage(e, n || window.origin);
    }
  };
function y(e) {
  const n = e.inputs?.map((t) => {
    switch (t.type) {
      case i.Message:
        return t.recipient;
      case i.Coin:
        return t.owner;
      default:
        return;
    }
  })[0];
  if (!n) throw new Error('No possible signer found!');
  return a.fromB256(l(n)).toString();
}
var E = class extends f {
    acceptMessage(e) {
      const { data: n } = e;
      return e.origin === window.origin && n.target === p;
    }
    async sendRequest(e) {
      e && this.postMessage({ type: 'request', target: v, request: e });
    }
    async network() {
      return this.client.request('network', {});
    }
    async isConnected() {
      return this.client.request('isConnected', {});
    }
    async connect() {
      return this.client.request('connect', {});
    }
    async disconnect() {
      return this.client.request('disconnect', {});
    }
    async accounts() {
      return this.client.request('accounts', {});
    }
    async currentAccount() {
      return this.client.request('currentAccount', {});
    }
    async signMessage(e, n) {
      if (!n.trim()) throw new Error('Message is required');
      return this.client.request('signMessage', { address: e, message: n });
    }
    async sendTransaction(e, n, t) {
      if (!e) throw new Error('Transaction is required');
      const r = t || e.signer || y(e);
      return this.client.request('sendTransaction', {
        address: r,
        provider: n,
        transaction: JSON.stringify(e),
      });
    }
    on(e, n) {
      return super.on(e, n);
    }
  },
  M = class extends c {
    constructor(e, n) {
      super(e, n), (this.provider = n);
    }
    async signMessage(e) {
      return this.provider.walletConnection.signMessage(this.address.toString(), e);
    }
    async sendTransaction(e) {
      return this.provider.sendTransaction({ ...e, signer: this.address.toString() });
    }
  },
  m = class extends u {
    constructor(e, n) {
      super(e), (this.walletConnection = n);
    }
    async sendTransaction(e) {
      const n = await this.walletConnection.sendTransaction(e, { url: this.url });
      return new d(n, this);
    }
  },
  s = {},
  C = class extends E {
    constructor() {
      super(...arguments),
        (this.utils = {
          createAddress: (e) => (
            console.warn('Do not use this method! It will be removed in the next release.'),
            a.fromString(e)
          ),
        });
    }
    async getProvider() {
      const e = await this.network();
      if (s.provider) return s.provider;
      const n = new m(e.url, this);
      return (
        (s.provider = n),
        this.on('network', async (t) => {
          s.provider?.connect(t.url);
        }),
        s.provider
      );
    }
    async getWallet(e) {
      const n = await this.getProvider();
      return new M(e, n);
    }
  },
  q = (e) =>
    new Proxy(e, {
      get(n, t) {
        return n[t];
      },
      set(n, t, r) {
        return Object.hasOwn(n, t) && ['_eventsCount', '_events'].includes(t)
          ? ((n[t] = r), !0)
          : !1;
      },
      defineProperty(n, t) {
        return Object.hasOwn(n, t) ? n[t] : !1;
      },
      deleteProperty() {
        return !1;
      },
    });
function b(e) {
  Object.defineProperty(e, 'fuel', {
    value: q(new C()),
    writable: !1,
    enumerable: !0,
    configurable: !0,
  });
}
b(window);
