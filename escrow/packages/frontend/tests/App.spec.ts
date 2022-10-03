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

    test("Basic app flow", async ({ page }) => {
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
});