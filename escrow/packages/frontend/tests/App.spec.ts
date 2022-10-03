import { test, expect } from "@playwright/test";
import { ASSETS, DECIMAL_PRECISION, FUEL_PROVIDER_URL } from "../src/config";
import { Provider, TestUtils, Wallet } from "fuels";

let wallets: Wallet[] = [];
let numWallets = 4;
const coins = ASSETS.map(assetId => {
    return { assetId, amount: DECIMAL_PRECISION.mul(100) };
});

test.describe("e2e", () => {
    test.beforeAll(async () => {
        const provider = new Provider(FUEL_PROVIDER_URL);
        for (let i = 0; i < numWallets; ++i) {
            const wallet = await TestUtils.generateTestWallet(
                provider,
                coins
            );
            wallets.push(wallet);
        }
    });

    test("Buyer transfers to seller", async ({ page }) => {
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
        assetInput0.fill(ASSETS[1]);
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

        page.on('dialog', async dialog => {
            expect(dialog.message()).toContain("New escrow created");
        });

        // TODO Switch page and wallet
        await page.goto("localhost:3000/buyer");

        // Buyer deposits
        const depositAssetInput = page.locator('[aria-label="Asset input"]');
        await depositAssetInput.fill(ASSETS[1]);
        const depositAmountInput = page.locator('[aria-label="Asset amount input"]');
        await depositAmountInput.fill("0.1");

        const deposit = page.locator('[aria-label="Deposit"]');
        expect(deposit).toContainText("Deposit Asset");
        await deposit.click();

        page.on('dialog', async dialog => {
            expect(dialog.message()).toContain("Deposit successful.");
        });

        // Buyer transfers to seller
        const transferToSeller = page.locator('[aria-label="Transfer to seller"]');
        expect(transferToSeller).toBeDefined();
        await transferToSeller.click();

        page.on('dialog', async dialog => {
            expect(dialog.message()).toContain("Transfer to seller successful.");
        });
    });

    test("Seller returns deposit to buyer", async ({ page }) => {
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
        assetInput0.fill(ASSETS[1]);
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

        page.on('dialog', async dialog => {
            expect(dialog.message()).toContain("New escrow created");
        });

        // TODO figure out if we need to explicitly switch wallet
        await page.goto("localhost:3000/buyer");

        // Buyer deposits
        const depositAssetInput = page.locator('[aria-label="Asset input"]');
        await depositAssetInput.fill(ASSETS[1]);
        const depositAmountInput = page.locator('[aria-label="Asset amount input"]');
        await depositAmountInput.fill("0.1");

        const deposit = page.locator('[aria-label="Deposit"]');
        expect(deposit).toContainText("Deposit Asset");
        await deposit.click();

        page.on('dialog', async dialog => {
            expect(dialog.message()).toContain("Deposit successful.");
        });

        // Switch back to seller page
        await page.goto("localhost:3000/seller");

        // Seller returns deposit to buyer
        const returnToBuyer = page.locator('[aria-label="Return deposit"]');
        expect(returnToBuyer).toContainText("Return Deposit");
        await returnToBuyer.click();

        page.on('dialog', async dialog => {
            expect(dialog.message()).toContain("Deposit returned to buyer.");
        });
    });

    test("Arbiter resolves in favor of buyer", async ({ page }) => {
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
        assetInput0.fill(ASSETS[1]);
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

        page.on('dialog', async dialog => {
            expect(dialog.message()).toContain("New escrow created");
        });

        // TODO figure out if we need to explicitly switch wallet
        await page.goto("localhost:3000/buyer");

        // Buyer deposits
        const depositAssetInput = page.locator('[aria-label="Asset input"]');
        await depositAssetInput.fill(ASSETS[1]);
        const depositAmountInput = page.locator('[aria-label="Asset amount input"]');
        await depositAmountInput.fill("0.1");

        const deposit = page.locator('[aria-label="Deposit"]');
        expect(deposit).toContainText("Deposit Asset");
        await deposit.click();

        page.on('dialog', async dialog => {
            expect(dialog.message()).toContain("Deposit successful.");
        });

        // Buyer disputes
        const dispute = page.locator('[aria-label="Dispute"]');
        expect(dispute).toContainText("Dispute");
        await dispute.click();

        page.on('dialog', async dialog => {
            expect(dialog.message()).toContain("Dispute successful.");
        });

        await page.goto("localhost:3000/arbiter");

        // Arbiter resolves in favor of buyer
        const newArbiterFeeInput = page.locator('[aria-label="Arbiter fee input"]');
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

    test("Arbiter resolves in favor of seller", async ({ page }) => {

    });

    test("Seller can withdraw collateral and take payment after deadline", async ({ page }) => {

    });

    test("Seller can propose an arbiter", async ({ page }) => {

    });
});