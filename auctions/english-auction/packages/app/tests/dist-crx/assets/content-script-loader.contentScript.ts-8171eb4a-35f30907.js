(function () {
  'use strict';

  (async () => {
    await import(
      /* @vite-ignore */
      chrome.runtime.getURL('assets/contentScript.ts-8171eb4a.js')
    );
  })().catch(console.error);
})();
