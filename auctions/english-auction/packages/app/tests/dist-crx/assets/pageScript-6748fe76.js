import { f as o, x as a, n as c, z as u, t as d, y as r } from './index-0339478c.js';
var l = 'Fuel',
  h = 'FuelContentScript',
  v = 'message',
  p = class extends d {
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
        (this.client = new r.JSONRPCClient(this.sendRequest.bind(this))),
        (this.server = new r.JSONRPCServer());
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
  w = class extends p {
    constructor() {
      super(),
        (this.onMessage = (e) => {
          const t = Object.freeze(e);
          if (!this.acceptMessage(t)) return;
          const { data: n } = t;
          this.onCommunicationMessage(n);
        }),
        window.addEventListener(v, this.onMessage.bind(this));
    }
    acceptMessage(e) {
      return !0;
    }
    postMessage(e, t) {
      window.postMessage(e, t || window.origin);
    }
  },
  g = class extends w {
    acceptMessage(e) {
      const { data: t } = e;
      return e.origin === window.origin && t.target === l;
    }
    async sendRequest(e) {
      e && this.postMessage({ type: 'request', target: h, request: e });
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
    async signMessage(e, t) {
      if (!t.trim()) throw new Error('Message is required');
      return this.client.request('signMessage', { address: e, message: t });
    }
    async sendTransaction(e, t) {
      if (!e) throw new Error('Transaction is required');
      return this.client.request('sendTransaction', {
        provider: t,
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
  f = class extends c {
    constructor(e, t) {
      super(e), (this.walletConnection = t);
    }
    async sendTransaction(e) {
      const t = await this.walletConnection.sendTransaction(e, { url: this.url });
      return new u(t, this);
    }
  },
  s = {},
  E = class extends g {
    constructor() {
      super(...arguments),
        (this.utils = {
          createAddress: (e) => (
            console.warn('Do not use this method! It will be removed in the next release.'),
            o.fromString(e)
          ),
        });
    }
    async getProvider() {
      const e = await this.network();
      if (s.provider) return s.provider;
      const t = new f(e.url, this);
      return (
        (s.provider = t),
        this.on('network', async (n) => {
          s.provider?.connect(n.url);
        }),
        s.provider
      );
    }
    async getWallet(e) {
      const t = await this.getProvider();
      return new a(e, t);
    }
  },
  y = (e) =>
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
function q(e) {
  Object.defineProperty(e, 'fuel', {
    value: y(new E()),
    writable: !1,
    enumerable: !0,
    configurable: !0,
  });
}
q(window);
