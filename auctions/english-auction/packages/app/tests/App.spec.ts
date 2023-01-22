import '../load.envs';
import type { BrowserContext, Page } from '@playwright/test';

import { test, expect } from './fixtures';

const WORDS = 'demand fashion unaware upgrade upon heart bright august panel kangaroo want gaze';
const WALLET_PASSWORD = '123123123';

async function walletSetup(context: BrowserContext, extensionId: string) {
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

  // Go back to app page and connect wallet
  await appPage.goto('/sell');

  // CONNECT TO WALLET
  const connectPage = await connectPagePromise;
  await connectPage.waitForLoadState();
  const connectButton = connectPage.locator('button').getByText('Connect');
  await connectButton.click();

  await appPage.goto('/sell');
  await appPage.reload();

  return { appPage, walletPage };
}

async function walletApprove(approvePagePromise: Promise<Page>) {
  // Handle transaction approval in web wallet
  const approvePage = await approvePagePromise;
  await approvePage.waitForLoadState();
  const approveButton = approvePage.locator('button').getByText('Confirm');
  await approveButton.click();

  const enterPasswordInput = approvePage.locator(`[aria-label="Your Password"]`);
  await enterPasswordInput.fill(WALLET_PASSWORD);
  const confirmButton = approvePage.locator('button').getByText('Confirm Transaction');
  await confirmButton.click();
}

async function addWallet(walletPage: Page, extensionId: string) {
  await walletPage.goto(`chrome-extension://${extensionId}/popup.html`);

  // First we have to add a second account
  const accountsButton = walletPage.locator('[aria-label="Accounts"]');
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
}

async function switchWallet(walletPage: Page, extensionId: string, accountName: string) {
  // Switch to account 1
  await walletPage.goto(`chrome-extension://${extensionId}/popup.html`);
  const accountsButton = walletPage.locator('[aria-label="Accounts"]');
  await accountsButton.click();
  const account1Button = walletPage.locator(`[aria-label="${accountName}"]`);
  await account1Button.click();
}

function getPages(context: BrowserContext) {
  const pages = context.pages();
  const [walletPage] = pages.filter((page) => page.url().includes('popup'));
  const [appPage] = pages.filter((page) => page.url().includes('localhost'));
  return { appPage, walletPage };
}

// TODO figure out how to test with extension
test.describe('e2e', () => {
  test.beforeAll(async ({ context, extensionId }) => {
    await walletSetup(context, extensionId);
  });

  // TODO this may require block manipulation etc
  test('Test auction expires', async () => {});

  test('Test auction (Sell: Token, Bid: Token) is canceled', async ({ context, extensionId }) => {
    // ACCOUNT 1 CREATES AUCTION

    const { appPage, walletPage } = getPages(context);

    const createAuctionButton = appPage.locator('button').getByText('Create Auction');
    expect(createAuctionButton).toBeDisabled();

    const fillSellerAddressButton = appPage.locator('[aria-label="Fill seller address"]');
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

    await walletApprove(approvePagePromise);

    // Expect transaction to be successful
    const transactionMessage = appPage.locator('text="Auction created successfully!"');
    await transactionMessage.waitFor();

    // ACCOUNT 2 BIDS ON AUCTION
    await appPage.goto('/buy');

    const errorText = appPage.locator('[aria-label="Seller cannot bid"]').first();
    await expect(errorText).toContainText(
      'Error sellers cannot bid on their own auctions. Change your wallet to bid on the auction.'
    );

    await addWallet(walletPage, extensionId);

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

    await walletApprove(approvePagePromise);

    // Expect transaction to be successful
    const bidTransactionMessage = appPage.locator('text="Auction bid placed successfully"');
    await bidTransactionMessage.waitFor();

    // ACCOUNT 1 CANCELS AUCTION

    // Switch to account 1
    await switchWallet(walletPage, extensionId, 'Account 1');

    // await appPage.goto('/buy');
    // await appPage.waitForLoadState();
    await appPage.reload();

    const cancelAuctionButton = appPage.locator('button').getByText('Cancel Auction').first();
    await expect(cancelAuctionButton).toBeEnabled();

    approvePagePromise = context.waitForEvent('page');

    await cancelAuctionButton.click();

    await walletApprove(approvePagePromise);

    // Expect transaction to be successful
    const cancelTransactionMessage = appPage.locator('text="Auction cancelled successfully!"');
    await cancelTransactionMessage.waitFor();

    // BOTH ACCOUNTS WITHDRAW
    // Account 1 withdraws
    const withdrawButton = appPage.locator('button').getByText('Withdraw from Auction').first();
    await expect(withdrawButton).toBeEnabled();

    approvePagePromise = context.waitForEvent('page');

    await withdrawButton.click();

    await walletApprove(approvePagePromise);

    // Expect transaction to be successful
    const withdrawTransactionMessage = appPage.locator('text="Withdraw from auction successful"');
    await withdrawTransactionMessage.waitFor();

    // Switch to account 2
    await switchWallet(walletPage, extensionId, 'Account 2');

    await appPage.reload();

    approvePagePromise = context.waitForEvent('page');

    await withdrawButton.click();

    await walletApprove(approvePagePromise);

    // Expect transaction to be successful
    await withdrawTransactionMessage.waitFor();
  });

  test('Test auction (Sell: Token, Bid: Token) with Reserve is canceled', async ({
    context,
    extensionId,
  }) => {
    const { appPage, walletPage } = getPages(context);

    await appPage.goto('/sell');

    await switchWallet(walletPage, extensionId, 'Account 1');

    await appPage.reload();

    const createAuctionButton = appPage.locator('button').getByText('Create Auction');
    expect(createAuctionButton).toBeDisabled();

    const fillSellerAddressButton = appPage.locator('[aria-label="Fill seller address"]');
    expect(fillSellerAddressButton).toBeDefined();
    await expect(fillSellerAddressButton).toBeEnabled();
    await fillSellerAddressButton.click();

    const sellAssetAmountInput = appPage.locator(`input[name="sellAssetAmount"]`);
    await sellAssetAmountInput.fill('0.001');

    const initialPriceInput = appPage.locator(`input[name="initialPrice"]`);
    await initialPriceInput.fill('0.001');

    const reservePriceButton = appPage.locator('[aria-label="Set reserve price"]');
    await reservePriceButton.click();

    const reservePriceInput = appPage.locator('[aria-label="Reserve price"]');
    await reservePriceInput.fill('0.002');

    const durationInput = appPage.locator(`input[name="duration"]`);
    await durationInput.fill('1000');

    await expect(createAuctionButton).toBeEnabled();
    let approvePagePromise = context.waitForEvent('page');
    await createAuctionButton.click();

    await walletApprove(approvePagePromise);

    // Expect transaction to be successful
    const transactionMessage = appPage.locator('text="Auction created successfully!"');
    await transactionMessage.waitFor();

    // ACCOUNT 2 BIDS ON AUCTION
    await appPage.goto('/buy');

    const errorText = appPage.locator('[aria-label="Seller cannot bid"]').first();
    await expect(errorText).toContainText(
      'Error sellers cannot bid on their own auctions. Change your wallet to bid on the auction.'
    );

    // We don't have to add an account in this test bc it was already added in the previous test
    await switchWallet(walletPage, extensionId, 'Account 2');

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

    await walletApprove(approvePagePromise);

    // Expect transaction to be successful
    const bidTransactionMessage = appPage.locator('text="Auction bid placed successfully"');
    await bidTransactionMessage.waitFor();

    // ACCOUNT 1 CANCELS AUCTION

    // Switch to account 1
    await switchWallet(walletPage, extensionId, 'Account 1');

    await appPage.reload();

    const cancelAuctionButton = appPage.locator('button').getByText('Cancel Auction').first();
    await expect(cancelAuctionButton).toBeEnabled();

    approvePagePromise = context.waitForEvent('page');

    await cancelAuctionButton.click();

    await walletApprove(approvePagePromise);

    // Expect transaction to be successful
    const cancelTransactionMessage = appPage.locator('text="Auction cancelled successfully!"');
    await cancelTransactionMessage.waitFor();

    // BOTH ACCOUNTS WITHDRAW
    // Account 1 withdraws
    const withdrawButton = appPage.locator('button').getByText('Withdraw from Auction').first();
    await expect(withdrawButton).toBeEnabled();

    approvePagePromise = context.waitForEvent('page');

    await withdrawButton.click();

    await walletApprove(approvePagePromise);

    // Expect transaction to be successful
    const withdrawTransactionMessage = appPage.locator('text="Withdraw from auction successful"');
    await withdrawTransactionMessage.waitFor();

    // Switch to account 2
    await switchWallet(walletPage, extensionId, 'Account 2');

    await appPage.reload();

    approvePagePromise = context.waitForEvent('page');

    await withdrawButton.click();

    await walletApprove(approvePagePromise);

    // Expect transaction to be successful
    await withdrawTransactionMessage.waitFor();
  });

  test('Test auction (Sell NFT, Bid NFT) is canceled', async ({ context, extensionId }) => {
    // ACCOUNT 1 CREATES AUCTION
    const { appPage, walletPage } = getPages(context);

    const createAuctionButton = appPage.locator('button').getByText('Create Auction');
    expect(createAuctionButton).toBeDisabled();

    const fillSellerAddressButton = appPage.locator('[aria-label="Fill seller address"]');
    expect(fillSellerAddressButton).toBeDefined();
    await expect(fillSellerAddressButton).toBeEnabled();
    await fillSellerAddressButton.click();

    // Switch to NFT for sell asset
    const sellAssetDropdown = appPage.locator('[aria-label="Sell Asset Dropdown"]');
    await sellAssetDropdown.click();

    const nftSelection = appPage.getByText('NFT');
    await nftSelection.click();

    const nftTokenIdInput = appPage.locator('input[name="sellNFTTokenId"]');
    // Thanks to our contract:init-test function
    // The seller owns an nft with token id 0
    await nftTokenIdInput.fill('0');
    const nftAssetIdInput = appPage.locator('input[name="sellNFTAssetId"]');
    await nftAssetIdInput.fill(process.env.VITE_NFT_ID!);

    // Switch to NFT for bid asset
    // Grab the second dropdown
    const bidAssetDropdown = appPage.locator('[aria-label="Bid Asset Dropdown"]');
    await bidAssetDropdown.click();

    const bidNFTSelection = appPage.locator('li').getByText('NFT');
    await bidNFTSelection.click();

    const bidNFTAssetIdInput = appPage.locator('input[name="bidNFTAssetId"]');
    await bidNFTAssetIdInput.fill(process.env.VITE_NFT_ID!);

    await expect(createAuctionButton).toBeEnabled();
    let approvePagePromise = context.waitForEvent('page');
    await createAuctionButton.click();

    await walletApprove(approvePagePromise);

    // Expect transaction to be successful
    const transactionMessage = appPage.locator('text="Auction created successfully!"');
    await transactionMessage.waitFor();

    // ACCOUNT 2 BIDS ON AUCTION
    await appPage.goto('/buy');

    const errorText = appPage.locator('[aria-label="Seller cannot bid"]').first();
    await expect(errorText).toContainText(
      'Error sellers cannot bid on their own auctions. Change your wallet to bid on the auction.'
    );

    // We don't have to add an account in this test bc it was already added in the previous test
    await switchWallet(walletPage, extensionId, 'Account 2');

    await appPage.reload();

    const cancelErrorText = appPage.locator('[aria-label="Buyer cannot cancel"]').first();
    await expect(cancelErrorText).toContainText(
      'Error only the seller of the auction can cancel it.'
    );

    // Now we can bid on the auction
    const tokenIdInput = appPage.getByPlaceholder('0').first();
    // The buyer has access to nft with token id 1
    await tokenIdInput.fill('1');
    const placeBidButton = appPage.locator('button').getByText('Bid on Auction').first();
    await expect(placeBidButton).toBeEnabled();

    approvePagePromise = context.waitForEvent('page');

    await placeBidButton.click();

    await walletApprove(approvePagePromise);

    // Expect transaction to be successful
    const bidTransactionMessage = appPage.locator('text="Auction bid placed successfully"');
    await bidTransactionMessage.waitFor();

    // ACCOUNT 1 CANCELS AUCTION

    // Switch to account 1
    await switchWallet(walletPage, extensionId, 'Account 1');

    await appPage.reload();

    const cancelAuctionButton = appPage.locator('button').getByText('Cancel Auction').first();
    await expect(cancelAuctionButton).toBeEnabled();

    approvePagePromise = context.waitForEvent('page');

    await cancelAuctionButton.click();

    await walletApprove(approvePagePromise);

    // Expect transaction to be successful
    const cancelTransactionMessage = appPage.locator('text="Auction cancelled successfully!"');
    await cancelTransactionMessage.waitFor();

    // BOTH ACCOUNTS WITHDRAW
    // Account 1 withdraws
    const withdrawButton = appPage.locator('button').getByText('Withdraw from Auction').first();
    await expect(withdrawButton).toBeEnabled();

    approvePagePromise = context.waitForEvent('page');

    await withdrawButton.click();

    await walletApprove(approvePagePromise);

    // Expect transaction to be successful
    const withdrawTransactionMessage = appPage.locator('text="Withdraw from auction successful"');
    await withdrawTransactionMessage.waitFor();

    // Switch to account 2
    await switchWallet(walletPage, extensionId, 'Account 2');

    await appPage.reload();

    approvePagePromise = context.waitForEvent('page');

    await withdrawButton.click();

    await walletApprove(approvePagePromise);

    // Expect transaction to be successful
    await withdrawTransactionMessage.waitFor();
  });

  test('Test auction (Sell NFT, Bid Token) is canceled', async () => {});

  test('Test auction (Sell Token, Bid NFT) is canceled', async () => {});
});
