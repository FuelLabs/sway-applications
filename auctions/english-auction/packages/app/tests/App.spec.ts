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
    // WALLET SETUP
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

    // G0 back to app page and connect wallet
    await appPage.goto('/sell');

    // CONNECT TO WALLET
    const connectPage = await connectPagePromise;
    await connectPage.waitForLoadState();
    const connectButton = connectPage.locator('button').getByText('Connect');
    await connectButton.click();

    await appPage.goto('/sell');
    await appPage.reload();

    // ACCOUNT 1 CREATES AUCTION
    const createAuctionButton = appPage.locator('button').getByText('Create Auction');
    expect(createAuctionButton).toBeDisabled();

    const fillSellerAddressButton = appPage.locator('button').getByText('fuel...apex');
    expect(fillSellerAddressButton).toBeDefined();
    await expect(fillSellerAddressButton).toBeEnabled();
    await fillSellerAddressButton.click();

    const sellAssetAmountInput = appPage.locator(`input[name="sellAssetAmount"]`);
    await sellAssetAmountInput.fill('0.001');

    const initialPriceInput = appPage.locator(`input[name="initialPrice"]`);
    await initialPriceInput.fill('0.001');

    const durationInput = appPage.locator(`input[name="duration"]`);
    await durationInput.fill('1000');

    await expect(createAuctionButton).toBeEnabled();
    let approvePagePromise = context.waitForEvent('page');
    await createAuctionButton.click();

    // Handle transaction approval in web wallet
    let approvePage = await approvePagePromise;
    await approvePage.waitForLoadState();
    let approveButton = approvePage.locator('button').getByText('Confirm');
    await approveButton.click();

    let enterPasswordInput = approvePage.locator(`[aria-label="Your Password"]`);
    await enterPasswordInput.fill(WALLET_PASSWORD);
    let confirmButton = approvePage.locator('button').getByText('Confirm Transaction');
    await confirmButton.click();

    // Expect transaction to be successful
    const transactionMessage = appPage.locator('text="Auction created successfully!"');
    await transactionMessage.waitFor();

    // ACCOUNT 2 BIDS ON AUCTION
    await appPage.goto('/buy');

    const errorText = appPage.locator('[aria-label="Seller cannot bid"]').first();
    await expect(errorText).toContainText(
      'Error sellers cannot bid on their own auctions. Change your wallet to bid on the auction.'
    );

    // Switch to account 2
    await walletPage.goto(`chrome-extension://${extensionId}/popup.html`);

    // First we have to add a second account
    let accountsButton = walletPage.locator('[aria-label="Accounts"]');
    await accountsButton.click();

    const addAccountButton = walletPage.locator('[aria-label="Add account"]');
    await addAccountButton.click();

    const accountNameInput = walletPage.locator('[aria-label="Account Name"]');
    await accountNameInput.fill('Account 2');

    const accountFormSubmitButton = walletPage.locator('button').getByText('Create');
    await accountFormSubmitButton.click();

    const passwordInput = walletPage.locator('[aria-label="Your Password"]');
    await passwordInput.fill(WALLET_PASSWORD);
    const accountConfirmButton = walletPage.locator('button').getByText('Add Account');
    await accountConfirmButton.click();

    // await appPage.goto('/buy');
    // await appPage.waitForLoadState();
    await appPage.reload();

    const cancelErrorText = appPage.locator('[aria-label="Buyer cannot cancel"]').first();
    await expect(cancelErrorText).toContainText(
      'Error only the seller of the auction can cancel it.'
    );

    // Now we can bid on the auction
    const bidAmountInput = appPage.getByPlaceholder('0.0').first();
    await bidAmountInput.fill('0.001');
    const placeBidButton = appPage.locator('button').getByText('Bid on Auction').first();
    await expect(placeBidButton).toBeEnabled();

    approvePagePromise = context.waitForEvent('page');

    await placeBidButton.click();

    // Handle transaction approval in web wallet
    approvePage = await approvePagePromise;
    await approvePage.waitForLoadState();
    approveButton = approvePage.locator('button').getByText('Confirm');
    await approveButton.click();

    enterPasswordInput = approvePage.locator(`[aria-label="Your Password"]`);
    await enterPasswordInput.fill(WALLET_PASSWORD);
    confirmButton = approvePage.locator('button').getByText('Confirm Transaction');
    await confirmButton.click();

    // Expect transaction to be successful
    const bidTransactionMessage = appPage.locator('text="Auction bid placed successfully"');
    await bidTransactionMessage.waitFor();

    // ACCOUNT 1 CANCELS AUCTION

    // Switch to account 1
    await walletPage.goto(`chrome-extension://${extensionId}/popup.html`);
    accountsButton = walletPage.locator('[aria-label="Accounts"]');
    await accountsButton.click();
    const account1Button = walletPage.locator('[aria-label="Account 1"]');
    await account1Button.click();

    // await appPage.goto('/buy');
    // await appPage.waitForLoadState();
    await appPage.reload();

    const cancelAuctionButton = appPage.locator('button').getByText('Cancel Auction').first();
    await expect(cancelAuctionButton).toBeEnabled();

    approvePagePromise = context.waitForEvent('page');

    await cancelAuctionButton.click();

    // Handle transaction approval in web wallet
    approvePage = await approvePagePromise;
    await approvePage.waitForLoadState();
    approveButton = approvePage.locator('button').getByText('Confirm');
    await approveButton.click();

    enterPasswordInput = approvePage.locator(`[aria-label="Your Password"]`);
    await enterPasswordInput.fill(WALLET_PASSWORD);
    confirmButton = approvePage.locator('button').getByText('Confirm Transaction');
    await confirmButton.click();

    // Expect transaction to be successful
    const cancelTransactionMessage = appPage.locator('text="Auction cancelled successfully!"');
    await cancelTransactionMessage.waitFor();

    // BOTH ACCOUNTS WITHDRAW
    // Account 1 withdraws
    const withdrawButton = appPage.locator('button').getByText('Withdraw from Auction');
    await expect(withdrawButton).toBeEnabled();

    approvePagePromise = context.waitForEvent('page');

    await withdrawButton.click();

    // Handle transaction approval in web wallet
    approvePage = await approvePagePromise;
    await approvePage.waitForLoadState();
    approveButton = approvePage.locator('button').getByText('Confirm');
    await approveButton.click();

    enterPasswordInput = approvePage.locator(`[aria-label="Your Password"]`);
    await enterPasswordInput.fill(WALLET_PASSWORD);
    confirmButton = approvePage.locator('button').getByText('Confirm Transaction');
    await confirmButton.click();

    // Expect transaction to be successful
    const withdrawTransactionMessage = appPage.locator('text="Withdraw from auction successful"');
    await withdrawTransactionMessage.waitFor();

    // Switch to account 2
    await walletPage.goto(`chrome-extension://${extensionId}/popup.html`);
    accountsButton = walletPage.locator('[aria-label="Accounts"]');
    await accountsButton.click();
    const account2Button = walletPage.locator('[aria-label="Account 2"]');
    await account2Button.click();

    await appPage.reload();

    approvePagePromise = context.waitForEvent('page');

    await withdrawButton.click();

    // Handle transaction approval in web wallet
    approvePage = await approvePagePromise;
    await approvePage.waitForLoadState();
    approveButton = approvePage.locator('button').getByText('Confirm');
    await approveButton.click();

    enterPasswordInput = approvePage.locator(`[aria-label="Your Password"]`);
    await enterPasswordInput.fill(WALLET_PASSWORD);
    confirmButton = approvePage.locator('button').getByText('Confirm Transaction');
    await confirmButton.click();

    // Expect transaction to be successful
    await withdrawTransactionMessage.waitFor();
  });
});
