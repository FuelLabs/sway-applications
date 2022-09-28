import { ASSETS, DECIMAL_PRECISION } from "../../config";
import { TestUtils, Wallet } from "fuels";
import { screen, renderWithRouter, fireEvent } from "@escrow/test-utils";
import { App } from "../../App";
import { createWallet, mockUseWalletList } from "../Core/hooks/__mocks__/useWallet";

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
    mockUseWalletList(wallets);
    for (const wallet of wallets) {
        await TestUtils.seedWallet(wallet, coins);
    }
});

describe("Create Escrow", () => {
    afterEach(() => {
        jest.clearAllMocks();
    });

    it("Should be able to create an escrow", async () => {
        const { user } = renderWithRouter(<App />, {
            route: "/seller",
        });

        const seller = wallets[0];
        const arbiter = wallets[1];
        const buyer = wallets[2];

        const showCreateEscrowBtn = await screen.findByLabelText(/Show create escrow/);
        expect(showCreateEscrowBtn).toBeInTheDocument();
        await user.click(showCreateEscrowBtn);

        const arbiterAddressInput = await screen.findByLabelText(/Arbiter address input/);
        fireEvent.change(arbiterAddressInput, {
            target: {
                value: arbiter.address.toHexString(),
            },
        });

        const arbiterAssetInput = await screen.findByLabelText(/Arbiter asset input/);
        fireEvent.change(arbiterAssetInput, {
            target: {
                value: ASSETS[0]
            },
        });

        const arbiterFeeInput = await screen.findByLabelText(/Arbiter fee input/);
        fireEvent.change(arbiterFeeInput, {
            target: {
                value: "0.1",
            },
        });

        const buyerAddressInput = await screen.findByLabelText(/Buyer address input/);
        fireEvent.change(buyerAddressInput, {
            target: {
                value: buyer.address.toHexString(),
            },
        });

        // TODO this test should get the current blockheight and add some constant
        // Instead of hardcoding "1000".
        const escrowDeadlineInput = await screen.findByLabelText(/Escrow deadline input/);
        fireEvent.change(escrowDeadlineInput, {
            target: {
                value: "1000",
            },
        });

        const assetInput0 = await screen.findByLabelText(/Asset input 0/);
        fireEvent.change(assetInput0, {
            target: {
                value: ASSETS[0],
            },
        });

        const assetAmount0 = await screen.findByLabelText(/Asset amount input 0/);
        fireEvent.change(assetAmount0, {
            target: {
                value: "0.1",
            },
        });

        const addAssetBtn = await screen.findByLabelText(/Add asset/);
        expect(addAssetBtn).toBeInTheDocument();
        await user.click(addAssetBtn);

        const assetInput1 = await screen.findByLabelText(/Asset input 1/);
        fireEvent.change(assetInput1, {
            target: {
                value: ASSETS[1],
            },
        });

        const assetAmount1 = await screen.findByLabelText(/Asset amount input 1/);
        fireEvent.change(assetAmount1, {
            target: {
                value: "0.2",
            },
        });

        const submitBtn = await screen.findByLabelText(/Create escrow/);
        expect(submitBtn).toBeInTheDocument();
        await user.click(submitBtn);
    });
});