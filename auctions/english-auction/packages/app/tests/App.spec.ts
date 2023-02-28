import '../load.envs';
import type { BrowserContext, Page } from '@playwright/test';

import { test, expect } from './fixtures';

const MNEMONIC = 'demand fashion unaware upgrade upon heart bright august panel kangaroo want gaze';
const WALLET_PASSWORD = '$123Ran123Dom123!';
const ACCOUNT1 = 'Account 1';
const ACCOUNT2 = 'Account 2';

async function walletSetup(context: BrowserContext, extensionId: string) {
  const appPage = await context.newPage();

  await appPage.goto('/sell');

  const hasFuel = await appPage.evaluate(() => {
    return typeof window.fuel === 'object';
  });
  expect(hasFuel).toBeTruthy();

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
  await signupPage.evaluate(`navigator.clipboard.writeText('${MNEMONIC}')`);

  const pasteButton = signupPage.locator('button').getByText('Paste');
  await pasteButton.click();

  let nextButton = signupPage.locator('button').getByText('Next');
  await nextButton.click();

  // Enter password
  const enterPassword = signupPage.locator(`[aria-label="Your Password"]`);
  await enterPassword.type(WALLET_PASSWORD);
  // Confirm password
  const confirmPassword = signupPage.locator(`[aria-label="Confirm Password"]`);
  await confirmPassword.type(WALLET_PASSWORD);

  // This is needed to dismiss the password security popup
  await signupPage.click('body');

  // Agree to T&S
  await signupPage.getByRole('checkbox').click();
  await expect(signupPage.getByRole('checkbox')).toBeChecked();
  await signupPage.locator('button').getByText('Next').click();
  await expect(signupPage.getByText('Wallet created successfully')).toBeVisible({ timeout: 15000 });

  await addWallet(walletPage, extensionId, ACCOUNT2);

  // Navigate to add network and add test network
  await walletPage.locator('[aria-label="Selected Network"]').click();
  await walletPage.locator('button').getByText('Add new network').click();
  await walletPage.locator('[aria-label="Network name"]').fill('test');
  await walletPage.locator('[aria-label="Network URL"]').fill(process.env.VITE_FUEL_PROVIDER_URL!);
  await walletPage.locator('button', { hasText: 'Create' }).click();

  const connectPagePromise = context.waitForEvent('page');

  // Go back to app page and connect wallet
  await appPage.goto('/sell');

  // Connect to wallets
  const connectPage = await connectPagePromise;
  await connectPage.waitForLoadState();

  nextButton = connectPage.locator('button').getByText('Next');
  await nextButton.click();

  const changeButton = connectPage.locator('button').getByText('Change');
  await changeButton.click();

  const activateAccount1Card = connectPage.locator(`[aria-label="${ACCOUNT1}"]`);
  const switchButton = activateAccount1Card.getByRole('switch');
  await switchButton.click();

  nextButton = connectPage.locator('button').getByText('Next');
  await nextButton.click();

  const connectButton = connectPage.locator('button').getByText('Connect');
  await connectButton.click();

  return { appPage, walletPage };
}

async function walletApprove(context: BrowserContext) {
  let approvePage = context.pages().find((p) => p.url().includes('/request/transaction'));
  if (!approvePage) {
    approvePage = await context.waitForEvent('page', {
      predicate: (page) => page.url().includes('/request/transaction'),
    });
  }

  const approveButton = approvePage.locator('button').getByText('Confirm');
  await approveButton.click({ timeout: 15000 });
}

async function addWallet(walletPage: Page, extensionId: string, accountName: string) {
  await walletPage.goto(`chrome-extension://${extensionId}/popup.html`);

  await walletPage.waitForSelector('[aria-label="Accounts"]');

  // First we have to add a second account
  const accountsButton = walletPage.locator('[aria-label="Accounts"]');
  await accountsButton.click();

  const addAccountButton = walletPage.locator('[aria-label="Add account"]');
  await addAccountButton.click();

  const accountNameInput = walletPage.locator('[aria-label="Account Name"]');
  await accountNameInput.fill(accountName);

  const accountFormSubmitButton = walletPage.locator('button').getByText('Create');
  await accountFormSubmitButton.click();

  await walletPage.waitForSelector('img', { timeout: 10000 });
}

async function switchWallet(walletPage: Page, extensionId: string, accountName: string) {
  // Switch to ACCOUNT1
  await walletPage.goto(`chrome-extension://${extensionId}/popup.html`);
  await walletPage.waitForSelector('[aria-label="Accounts"]');
  const accountsButton = walletPage.locator('[aria-label="Accounts"]');
  await accountsButton.click();
  const accountButton = walletPage.locator(`[aria-label="${accountName}"]`);
  await accountButton.waitFor();
  await accountButton.click();
}

function getPages(context: BrowserContext) {
  const pages = context.pages();
  const [walletPage] = pages.filter((page) => page.url().includes('popup'));
  const [appPage] = pages.filter((page) => page.url().includes('localhost'));
  return { appPage, walletPage };
}

test.beforeAll(async ({ context, extensionId }) => {
  await walletSetup(context, extensionId);
});

test.describe('e2e', () => {
  // TODO this requires block manipulation etc
  test.fixme('Test auction expires', async () => {});
  test('Test auction (Sell: Token, Bid: Token) is canceled', async ({ context, extensionId }) => {
    // ACCOUNT1 CREATES AUCTION

    const { appPage, walletPage } = getPages(context);

    await appPage.goto('/sell');

    await switchWallet(walletPage, extensionId, ACCOUNT1);

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

    const durationInput = appPage.locator(`input[name="duration"]`);
    await durationInput.fill('1000');

    await expect(createAuctionButton).toBeEnabled({ timeout: 10000 });
    await createAuctionButton.click({ timeout: 10000 });

    await walletApprove(context);

    // Expect transaction to be successful
    const transactionMessage = appPage.locator('text=Auction created successfully!');
    await transactionMessage.waitFor();

    // ACCOUNT 2 BIDS ON AUCTION
    await appPage.goto('/buy');

    const errorText = appPage.locator('[aria-label="Seller cannot bid"]').last();
    await expect(errorText).toContainText(
      'Error sellers cannot bid on their own auctions. Change your wallet to bid on the auction.'
    );

    await switchWallet(walletPage, extensionId, ACCOUNT2);

    await appPage.reload();

    const cancelErrorText = appPage.locator('[aria-label="Buyer cannot cancel"]').last();
    await expect(cancelErrorText).toContainText(
      'Error only the seller of the auction can cancel it.'
    );

    // Now we can bid on the auction
    const bidAmountInput = appPage.getByPlaceholder('0.0').last();
    await bidAmountInput.fill('0.001');
    const placeBidButton = appPage.locator('button').getByText('Bid on Auction').last();
    await expect(placeBidButton).toBeEnabled();

    await placeBidButton.click();

    await walletApprove(context);

    // Expect transaction to be successful
    const bidTransactionMessage = appPage.locator('text=Auction bid placed successfully');
    await bidTransactionMessage.waitFor();

    // ACCOUNT1 CANCELS AUCTION

    // Switch to ACCOUNT1
    await switchWallet(walletPage, extensionId, ACCOUNT1);

    await appPage.reload();

    const cancelAuctionButton = appPage.locator('button').getByText('Cancel Auction').last();
    await expect(cancelAuctionButton).toBeEnabled();

    await cancelAuctionButton.click();

    await walletApprove(context);

    // Expect transaction to be successful
    const cancelTransactionMessage = appPage.locator('text=Auction cancelled successfully!');
    await cancelTransactionMessage.waitFor();

    // BOTH ACCOUNTS WITHDRAW
    // ACCOUNT1 withdraws
    const withdrawButton = appPage.locator('button').getByText('Withdraw from Auction').last();
    await withdrawButton.waitFor();
    await expect(withdrawButton).toBeEnabled();

    await withdrawButton.click();

    await walletApprove(context);

    // Expect transaction to be successful
    const withdrawTransactionMessage = appPage.locator('text=Withdraw from auction successful');
    await withdrawTransactionMessage.waitFor();

    // Switch to account 2
    await switchWallet(walletPage, extensionId, ACCOUNT2);

    await appPage.reload();

    await withdrawButton.click();

    await walletApprove(context);

    // Expect transaction to be successful
    await withdrawTransactionMessage.waitFor();
  });

  test('Test auction (Sell: Token, Bid: Token) with Reserve is canceled', async ({
    context,
    extensionId,
  }) => {
    const { appPage, walletPage } = getPages(context);

    await appPage.goto('/sell');

    await switchWallet(walletPage, extensionId, ACCOUNT1);

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
    await createAuctionButton.click();

    await walletApprove(context);

    // Expect transaction to be successful
    const transactionMessage = appPage.locator('text=Auction created successfully!');
    await transactionMessage.waitFor();

    // ACCOUNT 2 BIDS ON AUCTION
    await appPage.goto('/buy');

    const errorText = appPage.locator('[aria-label="Seller cannot bid"]').last();
    await expect(errorText).toContainText(
      'Error sellers cannot bid on their own auctions. Change your wallet to bid on the auction.'
    );

    await switchWallet(walletPage, extensionId, ACCOUNT2);

    await appPage.reload();

    const cancelErrorText = appPage.locator('[aria-label="Buyer cannot cancel"]').last();
    await expect(cancelErrorText).toContainText(
      'Error only the seller of the auction can cancel it.'
    );

    // Now we can bid on the auction
    const bidAmountInput = appPage.getByPlaceholder('0.0').last();
    await bidAmountInput.fill('0.001');
    const placeBidButton = appPage.locator('button').getByText('Bid on Auction').last();
    await expect(placeBidButton).toBeEnabled();

    await placeBidButton.click();

    await walletApprove(context);

    // Expect transaction to be successful
    const bidTransactionMessage = appPage.locator('text=Auction bid placed successfully');
    await bidTransactionMessage.waitFor();

    // ACCOUNT1 CANCELS AUCTION

    // Switch to ACCOUNT1
    await switchWallet(walletPage, extensionId, ACCOUNT1);

    await appPage.reload();

    const cancelAuctionButton = appPage.locator('button').getByText('Cancel Auction').last();
    await expect(cancelAuctionButton).toBeEnabled();

    await cancelAuctionButton.click();

    await walletApprove(context);

    // Expect transaction to be successful
    const cancelTransactionMessage = appPage.locator('text=Auction cancelled successfully!');
    await cancelTransactionMessage.waitFor();

    // BOTH ACCOUNTS WITHDRAW
    // ACCOUNT1 withdraws
    const withdrawButton = appPage.locator('button').getByText('Withdraw from Auction').last();
    await expect(withdrawButton).toBeEnabled();

    await withdrawButton.click();

    await walletApprove(context);

    // Expect transaction to be successful
    const withdrawTransactionMessage = appPage.locator('text=Withdraw from auction successful');
    await withdrawTransactionMessage.waitFor();

    // Switch to account 2
    await switchWallet(walletPage, extensionId, ACCOUNT2);

    await appPage.reload();

    await withdrawButton.click();

    await walletApprove(context);

    // Expect transaction to be successful
    await withdrawTransactionMessage.waitFor();
  });

  test('Test auction (Sell: NFT, Bid: NFT) is canceled', async ({ context, extensionId }) => {
    // ACCOUNT1 CREATES AUCTION
    const { appPage, walletPage } = getPages(context);

    await appPage.goto('/sell');

    await switchWallet(walletPage, extensionId, ACCOUNT1);

    await appPage.reload();

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

    const durationInput = appPage.locator(`input[name="duration"]`);
    await durationInput.fill('1000');

    await expect(createAuctionButton).toBeEnabled();
    await createAuctionButton.click();

    await walletApprove(context);

    // Expect transaction to be successful
    const transactionMessage = appPage.locator('text=Auction created successfully!');
    await transactionMessage.waitFor();

    // ACCOUNT 2 BIDS ON AUCTION
    await appPage.goto('/buy');

    const errorText = appPage.locator('[aria-label="Seller cannot bid"]').last();
    await expect(errorText).toContainText(
      'Error sellers cannot bid on their own auctions. Change your wallet to bid on the auction.'
    );

    // We don't have to add an account in this test bc it was already added in the previous test
    await switchWallet(walletPage, extensionId, ACCOUNT2);

    await appPage.reload();

    const cancelErrorText = appPage.locator('[aria-label="Buyer cannot cancel"]').last();
    await expect(cancelErrorText).toContainText(
      'Error only the seller of the auction can cancel it.'
    );

    // Now we can bid on the auction
    const tokenIdInput = appPage.getByPlaceholder('0').last();
    // The buyer has access to nft with token id 1 from contract:init
    await tokenIdInput.fill('10');
    const placeBidButton = appPage.locator('button').getByText('Bid on Auction').last();
    await expect(placeBidButton).toBeEnabled();

    await placeBidButton.click();

    await walletApprove(context);

    // Expect transaction to be successful
    const bidTransactionMessage = appPage.locator('text=Auction bid placed successfully');
    await bidTransactionMessage.waitFor();

    // ACCOUNT1 CANCELS AUCTION

    // Switch to ACCOUNT1
    await switchWallet(walletPage, extensionId, ACCOUNT1);

    await appPage.reload();

    // BOTH ACCOUNTS WITHDRAW
    // ACCOUNT1 withdraws
    const withdrawButton = appPage.locator('button').getByText('Withdraw from Auction').last();
    await expect(withdrawButton).toBeEnabled();

    await withdrawButton.click();

    await walletApprove(context);

    // Expect transaction to be successful
    const withdrawTransactionMessage = appPage.locator('text=Withdraw from auction successful');
    await withdrawTransactionMessage.waitFor();

    // Switch to account 2
    await switchWallet(walletPage, extensionId, ACCOUNT2);

    await appPage.reload();

    await withdrawButton.click();

    await walletApprove(context);

    // Expect transaction to be successful
    await withdrawTransactionMessage.waitFor();
  });

  test('Test auction (Sell Token, Bid NFT) is canceled', async ({ context, extensionId }) => {
    // ACCOUNT1 CREATES AUCTION
    const { appPage, walletPage } = getPages(context);

    await appPage.goto('/sell');

    await switchWallet(walletPage, extensionId, ACCOUNT1);

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

    // Switch to NFT for bid asset
    // Grab the second dropdown
    const bidAssetDropdown = appPage.locator('[aria-label="Bid Asset Dropdown"]');
    await bidAssetDropdown.click();

    const bidNFTSelection = appPage.locator('li').getByText('NFT');
    await bidNFTSelection.click();

    const bidNFTAssetIdInput = appPage.locator('input[name="bidNFTAssetId"]');
    await bidNFTAssetIdInput.fill(process.env.VITE_NFT_ID!);

    const durationInput = appPage.locator(`input[name="duration"]`);
    await durationInput.fill('1000');

    await expect(createAuctionButton).toBeEnabled();
    await createAuctionButton.click();

    await walletApprove(context);

    // Expect transaction to be successful
    const transactionMessage = appPage.locator('text=Auction created successfully!');
    await transactionMessage.waitFor();

    // ACCOUNT 2 BIDS ON AUCTION
    await appPage.goto('/buy');

    const errorText = appPage.locator('[aria-label="Seller cannot bid"]').last();
    await expect(errorText).toContainText(
      'Error sellers cannot bid on their own auctions. Change your wallet to bid on the auction.'
    );

    // We don't have to add an account in this test bc it was already added in the previous test
    await switchWallet(walletPage, extensionId, ACCOUNT2);

    await appPage.reload();

    const cancelErrorText = appPage.locator('[aria-label="Buyer cannot cancel"]').last();
    await expect(cancelErrorText).toContainText(
      'Error only the seller of the auction can cancel it.'
    );

    // Now we can bid on the auction
    const tokenIdInput = appPage.getByPlaceholder('0').last();
    // The buyer has access to nft with token id 12 from contract:init
    await tokenIdInput.fill('12');
    const placeBidButton = appPage.locator('button').getByText('Bid on Auction').last();
    await expect(placeBidButton).toBeEnabled();

    await placeBidButton.click();

    await walletApprove(context);

    // Expect transaction to be successful
    const bidTransactionMessage = appPage.locator('text=Auction bid placed successfully');
    await bidTransactionMessage.waitFor();

    // ACCOUNT1 CANCELS AUCTION

    // Switch to ACCOUNT1
    await switchWallet(walletPage, extensionId, ACCOUNT1);

    await appPage.reload();

    // BOTH ACCOUNTS WITHDRAW
    // ACCOUNT1 withdraws
    const withdrawButton = appPage.locator('button').getByText('Withdraw from Auction').last();
    await expect(withdrawButton).toBeEnabled();

    await withdrawButton.click();

    await walletApprove(context);

    // Expect transaction to be successful
    const withdrawTransactionMessage = appPage.locator('text=Withdraw from auction successful');
    await withdrawTransactionMessage.waitFor();

    // Switch to account 2
    await switchWallet(walletPage, extensionId, ACCOUNT2);

    await appPage.reload();

    await withdrawButton.click();

    await walletApprove(context);

    // Expect transaction to be successful
    await withdrawTransactionMessage.waitFor();
  });

  test('Test auction (Sell NFT, Bid Token) is canceled', async ({ context, extensionId }) => {
    // ACCOUNT1 CREATES AUCTION
    const { appPage, walletPage } = getPages(context);

    await appPage.goto('/sell');

    await switchWallet(walletPage, extensionId, ACCOUNT1);

    await appPage.reload();

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
    await nftTokenIdInput.fill('1');
    const nftAssetIdInput = appPage.locator('input[name="sellNFTAssetId"]');
    await nftAssetIdInput.fill(process.env.VITE_NFT_ID!);

    const initialPriceInput = appPage.locator(`input[name="initialPrice"]`);
    await initialPriceInput.fill('0.001');

    const durationInput = appPage.locator(`input[name="duration"]`);
    await durationInput.fill('1000');

    await expect(createAuctionButton).toBeEnabled();
    await createAuctionButton.click();

    await walletApprove(context);

    // Expect transaction to be successful
    const transactionMessage = appPage.locator('text=Auction created successfully!');
    await transactionMessage.waitFor();

    // ACCOUNT 2 BIDS ON AUCTION
    await appPage.goto('/buy');

    const errorText = appPage.locator('[aria-label="Seller cannot bid"]').last();
    await expect(errorText).toContainText(
      'Error sellers cannot bid on their own auctions. Change your wallet to bid on the auction.'
    );

    // We don't have to add an account in this test bc it was already added in the previous test
    await switchWallet(walletPage, extensionId, ACCOUNT2);

    await appPage.reload();

    const cancelErrorText = appPage.locator('[aria-label="Buyer cannot cancel"]').last();
    await expect(cancelErrorText).toContainText(
      'Error only the seller of the auction can cancel it.'
    );

    // Now we can bid on the auction
    const assetAmountInput = appPage.getByPlaceholder('0.0').last();
    // The buyer has access to nft with token id 11
    await assetAmountInput.fill('0.001');
    const placeBidButton = appPage.locator('button').getByText('Bid on Auction').last();
    await expect(placeBidButton).toBeEnabled();

    await placeBidButton.click();

    await walletApprove(context);

    // Expect transaction to be successful
    const bidTransactionMessage = appPage.locator('text=Auction bid placed successfully');
    await bidTransactionMessage.waitFor();

    // ACCOUNT1 CANCELS AUCTION

    // Switch to ACCOUNT1
    await switchWallet(walletPage, extensionId, ACCOUNT1);

    await appPage.reload();

    const cancelAuctionButton = appPage.locator('button').getByText('Cancel Auction').last();
    await expect(cancelAuctionButton).toBeEnabled();

    await cancelAuctionButton.click();

    await walletApprove(context);

    // Expect transaction to be successful
    const cancelTransactionMessage = appPage.locator('text=Auction cancelled successfully!');
    await cancelTransactionMessage.waitFor();

    // BOTH ACCOUNTS WITHDRAW
    // ACCOUNT1 withdraws
    const withdrawButton = appPage.locator('button').getByText('Withdraw from Auction').last();
    await expect(withdrawButton).toBeEnabled();

    await withdrawButton.click();

    await walletApprove(context);

    // Expect transaction to be successful
    const withdrawTransactionMessage = appPage.locator('text=Withdraw from auction successful');
    await withdrawTransactionMessage.waitFor();

    // Switch to account 2
    await switchWallet(walletPage, extensionId, ACCOUNT2);

    await appPage.reload();

    await withdrawButton.click();

    await walletApprove(context);

    // Expect transaction to be successful
    await withdrawTransactionMessage.waitFor();
  });
});
