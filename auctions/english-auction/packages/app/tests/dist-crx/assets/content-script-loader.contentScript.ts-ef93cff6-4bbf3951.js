(function () {
  'use strict';

  (async () => {
    await import(
      /* @vite-ignore */
      chrome.runtime.getURL('assets/contentScript.ts-ef93cff6.js')
    );
  })().catch(console.error);
})();
