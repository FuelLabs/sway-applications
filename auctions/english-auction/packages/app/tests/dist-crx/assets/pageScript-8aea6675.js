import { W as o, d as a, v as c, e as u, k as s } from './index-ba02b8c1.js';
var d = {},
  l = 'Fuel',
  h = 'FuelContentScript',
  p = 'message',
  v = class extends u.exports {
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
        (this.client = new s.JSONRPCClient(this.sendRequest.bind(this))),
        (this.server = new s.JSONRPCServer());
    }
    externalMethods(e) {
      e.forEach((t) => {
        let n = t;
        t.name && (n = t.name), this.server.addMethod(n, this[n].bind(this));
      });
    }
    async sendRequest(e) {
      throw new Error('Send request not implemented');
    }
    sendResponse(e, t) {
      throw new Error('Send response not implemented');
    }
    onEvent(e) {
      e.events.forEach((t) => {
        this.emit(t.event, ...t.params);
      });
    }
    onResponse(e) {
      this.client.receive(e.response);
    }
    onRequest(e) {
      this.server.receive(e.request).then((t) => {
        this.sendResponse(t, e);
      });
    }
    onUIEvent(e) {}
  },
  w = class extends v {
    constructor() {
      super(),
        (this.onMessage = (e) => {
          const t = Object.freeze(e);
          if (!this.acceptMessage(t)) return;
          const { data: n } = t;
          this.onCommunicationMessage(n);
        }),
        window.addEventListener(p, this.onMessage.bind(this));
    }
    acceptMessage(e) {
      return !0;
    }
    postMessage(e, t) {
      window.postMessage(e, t || window.origin);
    }
  },
  { PUBLIC_PROVIDER_URL: g } = d,
  f = class extends w {
    constructor() {
      super(...arguments), (this.providerConfig = { url: g || 'http://localhost:4000/graphql' });
    }
    acceptMessage(e) {
      const { data: t } = e;
      return e.origin === window.origin && t.target === l;
    }
    async sendRequest(e) {
      e && this.postMessage({ type: 'request', target: h, request: e });
    }
    async selectNetwork(e) {
      this.providerConfig = e;
    }
    async network() {
      return this.client.request('network', {});
    }
    async isConnected() {
      return this.client.request('isConnected', {});
    }
    async connect(e) {
      return e && this.selectNetwork(e), this.client.request('connect', {});
    }
    async disconnect() {
      return this.client.request('disconnect', {});
    }
    async accounts() {
      return this.client.request('accounts', {});
    }
    async signMessage(e, t) {
      if (!t.trim()) throw new Error('Message is required');
      return this.client.request('signMessage', { address: e, message: t });
    }
    async sendTransaction(e) {
      if (!e) throw new Error('Transaction is required');
      return this.client.request('sendTransaction', {
        provider: this.providerConfig,
        transaction: JSON.stringify(e),
      });
    }
    async getSelectedAccount() {
      return this.client.request('getSelectedAccount', {});
    }
    on(e, t) {
      return super.on(e, t);
    }
  },
  r = class extends o {
    constructor(e) {
      super(e.providerConfig.url), (this.walletConnection = e);
    }
    async sendTransaction(e) {
      const t = await this.walletConnection.sendTransaction(e);
      return new a(t, this);
    }
  },
  C = class extends c {
    constructor(e, t) {
      super(e, new r(t));
    }
  },
  E = class extends f {
    getWallet(e) {
      return new C(e, this);
    }
    getProvider() {
      return new r(this);
    }
  },
  q = (e) =>
    new Proxy(e, {
      get(t, n) {
        return t[n];
      },
      set(t, n, i) {
        return Object.hasOwn(t, n) && ['_eventsCount', '_events'].includes(n)
          ? ((t[n] = i), !0)
          : !1;
      },
      defineProperty(t, n) {
        return Object.hasOwn(t, n) ? t[n] : !1;
      },
      deleteProperty() {
        return !1;
      },
    });
function M(e) {
  Object.defineProperty(e, 'fuel', {
    value: q(new E()),
    writable: !1,
    enumerable: !0,
    configurable: !0,
  });
}
M(window);
