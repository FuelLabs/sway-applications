export function mockLocalStorage(initialStore: { [key: string]: string }) {
  let store = initialStore;

  const newLocalStorage = {
    writable: true,
    value: {
      getItem(key: string) {
        return store[key] || null;
      },
      setItem(key: string, value: string) {
        store[key] = value.toString();
      },
      removeItem(key: string) {
        delete store[key];
      },
      clear() {
        store = {};
      },
    },
  };

  Object.defineProperty(window, 'localStorage', newLocalStorage);
}
