import './load.envs';
import type { PlaywrightTestConfig } from '@playwright/test';
// import { devices } from '@playwright/test';

const { E2E_PORT = 9000 } = process.env;

const config: PlaywrightTestConfig = {
  testDir: './tests',
  /* Retry on CI only */
  retries: process.env.CI ? 2 : 0,
  /* Opt out of parallel tests on CI. */
  workers: process.env.CI ? 1 : undefined,
  /* Reporter to use. See https://playwright.dev/docs/test-reporters */
  reporter: [['list', { printSteps: true }]],
  /* Shared settings for all the projects below. See https://playwright.dev/docs/api/class-testoptions. */
  use: {
    /* Maximum time each action such as `click()` can take. Defaults to 0 (no limit). */
    actionTimeout: 0,

    /* Collect trace when retrying the failed test. See https://playwright.dev/docs/trace-viewer */
    trace: 'on-first-retry',
    permissions: ['clipboard-read', 'clipboard-write'],
    baseURL: `http://localhost:${E2E_PORT}/`,
  },

  /* Configure projects for major browsers */
  // projects: [
  //   {
  //     name: 'chromium',
  //     use: {
  //       ...devices['Desktop Chrome'],
  //     },
  //   },
  // ],

  /* Run your local dev server before starting the tests */
  webServer: {
    command: 'pnpm dev-test',
    port: Number(E2E_PORT),
    reuseExistingServer: false,
  },
};

export default config;
