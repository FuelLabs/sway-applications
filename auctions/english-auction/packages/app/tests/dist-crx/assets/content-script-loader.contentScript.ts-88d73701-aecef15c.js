(function () {
  'use strict';

  (async () => {
    await import(
      /* @vite-ignore */
      chrome.runtime.getURL('assets/contentScript.ts-88d73701.js')
    );
  })().catch(console.error);
})();
