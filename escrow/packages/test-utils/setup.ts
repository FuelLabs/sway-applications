import failOnConsole from 'jest-fail-on-console';

const { getComputedStyle } = window;
window.getComputedStyle = (elt) => getComputedStyle(elt);

if (typeof window.matchMedia !== 'function') {
  Object.defineProperty(window, 'matchMedia', {
    enumerable: true,
    configurable: true,
    writable: true,
    value: jest.fn().mockImplementation((query) => ({
      matches: false,
      media: query,
      onchange: null,
      addListener: jest.fn(), // Deprecated
      removeListener: jest.fn(), // Deprecated
      addEventListener: jest.fn(),
      removeEventListener: jest.fn(),
      dispatchEvent: jest.fn(),
    })),
  });
}

failOnConsole({
  silenceMessage: (msg, method) => {
    if (msg.includes('toHexString')) return true;
    if (msg.includes('ReactDOM.render is no longer supported in React 18')) return true;
    if (method === 'warn') return true;
    return false;
  },
});
