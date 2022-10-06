import { test, expect } from "@playwright/test";
import { ASSETS, FUEL_PROVIDER_URL } from "../src/config";
import { Provider, Wallet } from "fuels";

let wallets: Wallet[] = [];
let numWallets = 4;
let arbiter: Wallet;
let buyer: Wallet;
let seller: Wallet;
let provider: Provider;

test.beforeAll(async () => {
    provider = new Provider(FUEL_PROVIDER_URL)
    for (let i = 0; i < numWallets; ++i) {
        const wallet = new Wallet(process.env[`VITE_WALLET${i}`]!);
        wallets.push(wallet);
    }
});

test.beforeEach(async ({ page }, testInfo) => {
    await page.goto('localhost:3000/seller');

    seller = wallets[0];
    arbiter = wallets[1];
    buyer = wallets[2];

    const deadline = testInfo.title === 'Seller can withdraw collateral and take payment after deadline' ? 3 : 1000;

    // Seller creates escrow
    const showCreateEscrow = page.locator('[aria-label="Show create escrow"]');
    expect(showCreateEscrow).toContainText("Create Escrow");
    await showCreateEscrow.click();

    const arbiterAddressInput = page.locator('[aria-label="Create arbiter address input"]');
    await arbiterAddressInput.fill(arbiter.address.toHexString());
    const arbiterAssetInput = page.locator('[aria-label="Create arbiter asset input"]');
    await arbiterAssetInput.fill(ASSETS[0]);
    const arbiterFeeInput = page.locator('[aria-label="Create arbiter fee input"]');
    await arbiterFeeInput.fill("0.1");

    const buyerAddressInput = page.locator('[aria-label="Buyer address input"]')
    await buyerAddressInput.fill(buyer.address.toHexString());

    const escrowDeadlineInput = page.locator('[aria-label="Escrow deadline input"]');
    const blockNumber = await provider.getBlockNumber();
    await escrowDeadlineInput.fill(blockNumber.add(deadline).toString());

    const assetInput0 = page.locator('[aria-label="Asset input 0"]');
    await assetInput0.fill(ASSETS[1]);
    const assetAmount0 = page.locator('[aria-label="Asset amount input 0"]');
    await assetAmount0.fill("0.1");

    const addAsset = page.locator('[aria-label="Add asset"]');
    expect(addAsset).toContainText("PlusAdd Asset");
    await addAsset.click();

    const assetInput1 = page.locator('[aria-label="Asset input 1"]');
    await assetInput1.fill(ASSETS[2]);
    const assetAmount1 = page.locator('[aria-label="Asset amount input 1"]')
    await assetAmount1.fill("0.2");

    const createEscrow = page.locator('[aria-label="Create escrow"]');
    expect(createEscrow).toContainText("PlusIconCreate Escrow");
    await createEscrow.click();

    let toast = page.locator('text="New escrow created."');
    await toast.waitFor();

    // Switch page and wallet
    await page.goto("localhost:3000/buyer");
    const showWallets = page.locator('[aria-label="Display wallets"]');
    await showWallets.selectOption({ index: 2 });

    // Buyer deposits
    const depositAssetInput = page.locator('[aria-label="Asset input"]');
    await depositAssetInput.fill(ASSETS[1]);
    const depositAmountInput = page.locator('[aria-label="Asset amount input"]');
    await depositAmountInput.fill("0.1");

    const deposit = page.locator('[aria-label="Deposit"]');
    expect(deposit).toContainText("Deposit Asset");
    await deposit.click();

    toast = page.locator('text="Deposit successful."');
    await toast.waitFor();
});

test.describe("e2e", () => {
    test("Buyer transfers to seller", async ({ page }) => {
        // Buyer transfers to seller
        const transferToSeller = page.locator('[aria-label="Transfer to seller"]');
        expect(transferToSeller).toContainText("Transfer To Seller");
        await transferToSeller.click();
        const toast = page.locator('text="Transfer to seller successful."');
        await toast.waitFor();
    });

    test("Seller returns deposit to buyer", async ({ page }) => {
        // Switch back to seller page
        await page.goto("localhost:3000/seller");
        expect(page.locator('[aria-label="Show create escrow"]')).toContainText("Create Escrow");

        // Seller returns deposit to buyer
        const returnToBuyer = page.locator('[aria-label="Return deposit"]');
        expect(returnToBuyer).toContainText("Return Deposit");
        await returnToBuyer.click();

        const toast = page.locator('text="Deposit returned to buyer."');
        await toast.waitFor();
    });

    test("Arbiter resolves in favor of buyer", async ({ page }) => {
        // Buyer disputes
        const dispute = page.locator('[aria-label="Dispute"]');
        expect(dispute).toContainText("Dispute");
        await dispute.click();

        let txFeedback = page.locator('text="Dispute successful."');
        await txFeedback.waitFor();

        await page.goto("localhost:3000/arbiter");
        const showWallets = page.locator('[aria-label="Display wallets"]');
        await showWallets.selectOption({ index: 1 });

        // Arbiter resolves in favor of buyer
        const newArbiterFeeInput = page.locator('[aria-label="Resolve arbiter fee input"]');
        await newArbiterFeeInput.fill("0.1");

        const userToFavor = page.locator('text=User to favor');
        await userToFavor.click();

        const buyerSelection = page.locator(`[data-key="${buyer.address.toHexString()}"]`);
        await buyerSelection.click();

        const resolveDispute = page.locator('[aria-label="Resolve dispute"]');
        expect(resolveDispute).toContainText("Resolve Dispute");
        await resolveDispute.click();

        txFeedback = page.locator('text="Dispute resolved."');
        await txFeedback.waitFor();
    });

    test("Arbiter resolves in favor of seller", async ({ page }) => {
        // Buyer disputes
        const dispute = page.locator('[aria-label="Dispute"]');
        expect(dispute).toContainText("Dispute");
        await dispute.click();

        let txFeedback = page.locator('text="Dispute successful."');
        await txFeedback.waitFor();

        await page.goto("localhost:3000/arbiter");
        const showWallets = page.locator('[aria-label="Display wallets"]');
        await showWallets.selectOption({ index: 1 });

        // Arbiter resolves in favor of buyer
        const newArbiterFeeInput = page.locator('[aria-label="Resolve arbiter fee input"]');
        await newArbiterFeeInput.fill("0.1");

        const userToFavor = page.locator('text=User to favor');
        await userToFavor.click();

        const sellerSelection = page.locator(`[data-key="${seller.address.toHexString()}"]`);
        await sellerSelection.click();

        const resolveDispute = page.locator('[aria-label="Resolve dispute"]');
        expect(resolveDispute).toContainText("Resolve Dispute");
        await resolveDispute.click();

        txFeedback = page.locator('text="Dispute resolved."');
        await txFeedback.waitFor();
    });

    test("Seller can withdraw collateral and take payment after deadline", async ({ page }) => {
        await page.goto('localhost:3000/seller');

        // Seller withdraws collateral
        const withdrawCollateral = page.locator('[aria-label="Withdraw collateral"]');
        await withdrawCollateral.click();

        // TODO fix withdraw collateral 
        // let toast = page.locator('text="Collateral withdrawn successfully."');
        // await toast.waitFor();

        // Seller takes payment
        const takePayment = page.locator('[aria-label="Take payment"]');
        await takePayment.click();
        let toast = page.locator('text="Took payment successfully."');
        await toast.waitFor();
    });

    test("can propose and accept arbiter", async ({ page }) => {
        // Switch back to seller page
        await page.goto("localhost:3000/seller");
        expect(page.locator('[aria-label="Show create escrow"]')).toContainText("Create Escrow");

        // Propose arbiter
        const proposeArbiterAddressInput = page.locator('[aria-label="Propose arbiter address input"]');
        await proposeArbiterAddressInput.fill(wallets[3].address.toHexString());
        const proposeArbiterAssetInput = page.locator('[aria-label="Propose arbiter asset input"]');
        await proposeArbiterAssetInput.fill(ASSETS[1]);
        const proposeArbiterFeeInput = page.locator('[aria-label="Propose arbiter fee input"]');
        await proposeArbiterFeeInput.fill("0.2");
        const proposeArbiter = page.locator('[aria-label="Propose arbiter button"]');
        expect(proposeArbiter).toContainText("Propose Arbiter");
        await proposeArbiter.click();


        let txFeedback = page.locator('text="New arbiter proposed."')
        await txFeedback.waitFor();

        await page.goto("localhost:3000/buyer");
        const showWallets = page.locator('[aria-label="Display wallets"]');
        await showWallets.selectOption({ index: 2 });

        // Accept arbiter
        const acceptArbiter = page.locator('[aria-label="Accept arbiter button"]');
        expect(acceptArbiter).toContainText("Accept Arbiter");
        await acceptArbiter.click();
        txFeedback = page.locator('text="Arbiter accepted successfully."');
        await txFeedback.waitFor();
    });
});