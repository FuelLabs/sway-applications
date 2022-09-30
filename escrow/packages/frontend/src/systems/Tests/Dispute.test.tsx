import { ASSETS, DECIMAL_PRECISION } from "../../config";
import { TestUtils, Wallet } from "fuels";
import { screen, renderWithRouter, fireEvent } from "@escrow/test-utils";
import { App } from "../../App";
import { createWallet, mockUseWallet, mockUseWalletList } from "../Core/hooks/__mocks__/useWallet";

let wallets: Wallet[] = [];
let numWallets = 4;
const coins = ASSETS.map(assetId => {
    return { assetId, amount: DECIMAL_PRECISION.mul(100) };
});

beforeAll(async () => {
    for (let i = 0; i < numWallets; ++i) {
        const wallet = createWallet();
        wallets.push(wallet);
    }
    mockUseWallet(wallets[0]);
    mockUseWalletList(wallets);
    await wallets.reduce(async (promise, wallet) => {
        await promise;
        await TestUtils.seedWallet(wallet, coins);
    }, Promise.resolve());
});

describe("Dispute", () => {
    afterEach(() => {
        jest.clearAllMocks();
    });

    xit("should be able to dispute", async () => {
        const { user } = renderWithRouter(<App />, {
            route: "/buyer"
        });

        const disputeBtn = await screen.findByLabelText(/Dispute/);
        expect(disputeBtn).toBeInTheDocument();
        await user.click(disputeBtn);
    });

    xit("should hide the disput button after dispute", async () => {
        const { user } = renderWithRouter(<App />, {
            route: "/buyer"
        });

        const disputeBtn = await screen.findByLabelText(/Dispute/);
        expect(disputeBtn).toBeInTheDocument();
        await user.click(disputeBtn);

        expect(disputeBtn).toBeNull();
    });
});