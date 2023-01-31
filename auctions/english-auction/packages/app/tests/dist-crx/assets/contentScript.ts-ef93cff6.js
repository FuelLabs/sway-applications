import './index-0339478c.js';
import { C as n, B as i, E as a, a as r } from './constants-97362116.js';
class o {
  constructor() {
    (this.onMessageFromExtension = (e) => {
      e.target === n && this.postMessage(e);
    }),
      (this.onMessageFromWindow = (e) => {
        const { data: s, origin: c } = Object.freeze(e);
        c === window.location.origin &&
          s.target === n &&
          this.connection.postMessage({ ...s, target: i });
      }),
      (this.connection = chrome.runtime.connect(chrome.runtime.id, { name: i })),
      this.connection.onMessage.addListener(this.onMessageFromExtension),
      window.addEventListener(a, this.onMessageFromWindow);
  }
  static start() {
    return new o();
  }
  postMessage(e) {
    const s = { ...e, target: r };
    window.postMessage(s, window.location.origin);
  }
}
const d = '/assets/pageScript-70efaf62.js';
o.start();
async function m() {
  const t = document.createElement('script');
  (t.src = chrome.runtime.getURL(d)),
    (t.type = 'module'),
    (t.onload = () => {
      t.remove();
    }),
    (document.head || document.documentElement).appendChild(t);
}
m();
