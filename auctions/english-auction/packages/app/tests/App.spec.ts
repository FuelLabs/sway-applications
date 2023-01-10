import { test } from './fixtures';

// TODO figure out how to test with extension
test.describe('e2e', () => {
  test.afterAll(({ context }) => {
    context.close();
  });

  // TODO this may require block manipulation etc
  test('Test auction expires', async () => {});

  test('Test auction is canceled', async ({ page, context, extensionId }) => {
    const walletPage = await context.newPage();
    await walletPage.goto(`chrome-extension://${extensionId}/popup.html`);
    await page.screenshot({ path: './temp1.png', fullPage: true });
    await walletPage.screenshot({ path: './wallet-temp1.png', fullPage: true });
  });
});
