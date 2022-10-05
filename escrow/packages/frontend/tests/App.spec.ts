import { test, expect } from "@playwright/test";
import { ASSETS, DECIMAL_PRECISION, FUEL_PROVIDER_URL } from "../src/config";
import { Provider, TestUtils, Wallet } from "fuels";

let wallets: Wallet[] = [];
let numWallets = 4;

test.beforeAll(async () => {
    for (let i = 0; i < numWallets; ++i) {
        const wallet = new Wallet(process.env[`VITE_WALLET${i}`]!);
        wallets.push(wallet);
    }
});

test.beforeEach(async ({ page }, testInfo) => {
    await page.goto('localhost:3000/seller');

    const arbiter = wallets[1];
    const buyer = wallets[2];

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
    await escrowDeadlineInput.fill("1000");

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

    test.fixme("Arbiter resolves in favor of buyer", async ({ page }) => {
        // Buyer disputes
        const dispute = page.locator('[aria-label="Dispute"]');
        expect(dispute).toContainText("Dispute");
        await dispute.click();

        page.on('dialog', async dialog => {
            expect(dialog.message()).toContain("Dispute successful.");
        });

        await page.goto("localhost:3000/arbiter");

        // Arbiter resolves in favor of buyer
        const newArbiterFeeInput = page.locator('[aria-label="Resolve arbiter fee input"]');
        await newArbiterFeeInput.fill("0.1");
        const userToFavor = page.locator('[aria-label="Actions"]');
        // TODO figure out how to "fill" user to favor

        const resolveDispute = page.locator('[aria-label="Resolve dispute"]');
        expect(resolveDispute).toContainText("Resolve Dispute");
        await resolveDispute.click();

        page.on('dialog', async dialog => {
            expect(dialog.message()).toContain("Dispute resolved.")
        });
    });

    test.fixme("Arbiter resolves in favor of seller", async ({ page }) => {

    });

    // TODO marked as fixme until block manipulation/query
    test.fixme("Seller can withdraw collateral and take payment after deadline", async ({ page }) => {

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