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

describe("Buyer deposit", () => {
    afterEach(() => {
        jest.clearAllMocks();
    });

    xit("should disable the deposit button by default", async () => {
        renderWithRouter(<App />, {
            route: "/buyer",
        });

        const depositBtn = await screen.findByLabelText(/Deposit/);
        expect(depositBtn).toBeInTheDocument();
        expect(depositBtn).toBeDisabled();
    });

    xit("should be able to set input values", async () => {
        renderWithRouter(<App />, {
            route: "/buyer"
        });

        const depositAssetInput = await screen.findByLabelText(/Deposit asset input/);
        fireEvent.change(depositAssetInput, {
            target: {
                value: ASSETS[1],
            },
        });
        expect(depositAssetInput).toHaveValue(ASSETS[1]);

        const depositAmountInput = await screen.findByLabelText(/Deposit amount input/);
        const depositAmountValue = "0.1";
        fireEvent.change(depositAmountInput, {
            target: {
                value: depositAmountValue,
            },
        });
        expect(depositAmountInput).toHaveValue(depositAmountValue);
    });

    xit("should be able to deposit", async () => {
        const { user } = renderWithRouter(<App />, {
            route: "/buyer"
        });

        const depositAssetInput = await screen.findByLabelText(/Deposit asset input/);
        fireEvent.change(depositAssetInput, {
            target: {
                value: ASSETS[1],
            },
        });

        const depositAmountInput = await screen.findByLabelText(/Deposit amount input/);
        const depositAmountValue = "0.1";
        fireEvent.change(depositAmountInput, {
            target: {
                value: depositAmountValue,
            },
        });

        const depositBtn = await screen.findByLabelText(/Deposit/);
        expect(depositBtn).toBeInTheDocument();
        await user.click(depositBtn);
    });

    xit("should not display the deposit button after deposit", async () => {
        const { user } = renderWithRouter(<App />, {
            route: "/buyer"
        });

        const depositAssetInput = await screen.findByLabelText(/Deposit asset input/);
        fireEvent.change(depositAssetInput, {
            target: {
                value: ASSETS[1],
            },
        });

        const depositAmountInput = await screen.findByLabelText(/Deposit amount input/);
        const depositAmountValue = "0.1";
        fireEvent.change(depositAmountInput, {
            target: {
                value: depositAmountValue,
            },
        });

        const depositBtn = await screen.findByLabelText(/Deposit/);
        expect(depositBtn).toBeInTheDocument();
        await user.click(depositBtn);

        // After a deposit the input and button should go away
        expect(depositBtn).toBeNull();
    });
});