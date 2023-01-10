import { test, expect } from './fixtures';

// TODO figure out how to test with extension
test.describe('e2e', () => {
  test.afterAll(({ context }) => {
    context.close();
  });

  // TODO this may require block manipulation etc
  test('Test auction expires', async () => {});

  test('Test auction is canceled', async ({ context, extensionId }) => {
    const walletPage = await context.newPage();
    await walletPage.goto(`chrome-extension://${extensionId}/popup.html`);
    const signupPage = await context.waitForEvent('page', {
      predicate: (page) => page.url().includes('sign-up'),
    });
    expect(signupPage.url()).toContain('sign-up');
    await signupPage.screenshot({ path: './screenshots/tempscreen.png', fullPage: true });
    const button = signupPage.locator('button').getByText('I already have a wallet');
    await button.click();
    await signupPage.screenshot({ path: './screenshots/postclick.png', fullPage: true });
  });
});
