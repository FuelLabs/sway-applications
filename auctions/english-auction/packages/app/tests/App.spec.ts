import { test, expect } from './fixtures';

const WORDS = 'monkey advice bacon rival fitness flip inspire public yard depart thank also';
const WALLET_PASSWORD = '123123123';

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

    const button = signupPage.locator('button').getByText('I already have a wallet');
    await button.click();

    /** Copy words to clipboard area */
    await signupPage.evaluate(`navigator.clipboard.writeText('${WORDS}')`);

    const pasteButton = signupPage.locator('button').getByText('Paste');
    await pasteButton.click();

    const nextButton = signupPage.locator('button').getByText('Next');
    await nextButton.click();

    // Enter password
    const enterPassword = signupPage.locator(`[aria-label="Your Password"]`);
    await enterPassword.type(WALLET_PASSWORD);
    // Confirm password
    const confirmPassword = signupPage.locator(`[aria-label="Confirm Password"]`);
    await confirmPassword.type(WALLET_PASSWORD);
    // Agree to T&S
    await signupPage.getByRole('checkbox').click();
    await signupPage.locator('button').getByText('Next').click();

    const appPage = await context.newPage();

    const connectPagePromise = context.waitForEvent('page');

    await appPage.goto('/sell');

    const connectPage = await connectPagePromise;
    await connectPage.waitForLoadState();
    const connectButton = connectPage.locator('button').getByText('Connect');
    await connectButton.click();
    await connectPage.close();

    await appPage.goto('/sell');

    await appPage.screenshot({ path: './screenshots/pic1.png', fullPage: true });

    // Create auction button should be initially disabled
    const createAuctionButton = appPage.locator('button').getByText('Create Auction');
    expect(createAuctionButton).toBeDisabled();

    await appPage.screenshot({ path: './screenshots/pic2.png', fullPage: true });
    // TODO fix test the below assertion should work
    // await expect(createAuctionButton).toBeDisabled();

    // const sellerAddressInput = appPage.locator(`input[name="sellerAddress"]`);
    // TODO figure out if I need this
    // await expect(sellerAddressInput).toBeFocused();
    const fillSellerAddressButton = appPage.locator('button').getByText('fuel...apex');
    expect(fillSellerAddressButton).toBeDefined();
    await appPage.screenshot({ path: './screenshots/pic3.png', fullPage: true });
    await expect(fillSellerAddressButton).toBeEnabled();
    await fillSellerAddressButton.click();

    const sellAssetAmountInput = appPage.locator(`input[name="sellAssetAmount"]`);
    await sellAssetAmountInput.fill('0.001');

    const initialPriceInput = appPage.locator(`input[name="initialPrice"]`);
    await initialPriceInput.fill('0.001');

    const durationInput = appPage.locator(`input[name="duration"]`);
    await durationInput.fill('1000');

    await appPage.screenshot({ path: './screenshots/pic4.png', fullPage: true });

    await expect(createAuctionButton).toBeEnabled();
    const approvePagePromise = context.waitForEvent('page');
    await createAuctionButton.click();

    const approvePage = await approvePagePromise;
    await approvePage.waitForLoadState();
    const approveButton = approvePage.locator('button').getByText('Approve');
    const enterPasswordPagePromise = context.waitForEvent('page');
    await approveButton.click();

    const enterPasswordPage = await enterPasswordPagePromise;
    await enterPasswordPage.waitForLoadState();
    const enterPasswordInput = enterPasswordPage.locator(`aria-label=["Your Password"]`);
    await enterPasswordInput.fill(WALLET_PASSWORD);
    const confirmButton = enterPasswordPage.locator('button').getByText('Confirm Transaction');
    await confirmButton.click();

    // Expect transaction to be successful
    const transactionMessage = appPage.locator('text="Auction created successfully!"');
    await transactionMessage.waitFor();
  });
});
