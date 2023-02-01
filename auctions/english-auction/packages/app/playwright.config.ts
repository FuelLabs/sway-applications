import './load.envs';
import type { PlaywrightTestConfig } from '@playwright/test';

const { E2E_PORT = 9000 } = process.env;

const config: PlaywrightTestConfig = {
  timeout: 120000,
  testDir: './tests',
  /* Retry on CI only */
  retries: process.env.CI ? 1 : 0,
  /* Opt out of parallel tests on CI. */
  workers: process.env.CI ? 1 : undefined,
  /* Reporter to use. See https://playwright.dev/docs/test-reporters */
  reporter: 'html',
  /* Shared settings for all the projects below. See https://playwright.dev/docs/api/class-testoptions. */
  use: {
    /* Maximum time each action such as `click()` can take. Defaults to 0 (no limit). */
    actionTimeout: 15000,

    /* Collect trace when retrying the failed test. See https://playwright.dev/docs/trace-viewer */
    trace: 'on-first-retry',
    permissions: ['clipboard-read', 'clipboard-write'],
    baseURL: `http://localhost:${E2E_PORT}/`,
  },

  /* Run your local dev server before starting the tests */
  webServer: {
    command: 'pnpm dev-test',
    port: Number(E2E_PORT),
    reuseExistingServer: false,
  },
};

export default config;
