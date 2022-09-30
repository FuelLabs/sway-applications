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

describe("Propose arbiter", () => {
    afterEach(() => {
        jest.clearAllMocks();
    });

    xit("should disable propose arbiter button by default", async () => {
        const { user } = renderWithRouter(<App />, {
            route: "/seller",
        });

        const arbiter = wallets[1];
        const buyer = wallets[2];

        // First create an escrow
        const showCreateEscrowBtn = await screen.findByLabelText(/Show create escrow/);
        expect(showCreateEscrowBtn).toBeInTheDocument();
        await user.click(showCreateEscrowBtn);

        const arbiterAddressInput = await screen.findByLabelText(/Create arbiter address input/);
        fireEvent.change(arbiterAddressInput, {
            target: {
                value: arbiter.address.toHexString(),
            },
        });

        const arbiterAssetInput = await screen.findByLabelText(/Create arbiter asset input/);
        fireEvent.change(arbiterAssetInput, {
            target: {
                value: ASSETS[0]
            },
        });

        const arbiterFeeInput = await screen.findByLabelText(/Create arbiter fee input/);
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

        const proposeArbiterBtn = await screen.findByLabelText(/Propose arbiter/);
        expect(proposeArbiterBtn).toBeInTheDocument();
        expect(proposeArbiterBtn).toBeDisabled();
    });

    it("should be able to set input values", async () => {
        const { user } = renderWithRouter(<App />, {
            route: "/seller",
        });

        const arbiter = wallets[1];
        const buyer = wallets[2];
        const newArbiter = wallets[3];

        // First create an escrow
        const showCreateEscrowBtn = await screen.findByLabelText(/Show create escrow/);
        expect(showCreateEscrowBtn).toBeInTheDocument();
        await user.click(showCreateEscrowBtn);

        const arbiterAddressInput = await screen.findByLabelText(/Create arbiter address input/);
        fireEvent.change(arbiterAddressInput, {
            target: {
                value: arbiter.address.toHexString(),
            },
        });

        const arbiterAssetInput = await screen.findByLabelText(/Create arbiter asset input/);
        fireEvent.change(arbiterAssetInput, {
            target: {
                value: ASSETS[0]
            },
        });

        const arbiterFeeInput = await screen.findByLabelText(/Create arbiter fee input/);
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

        const newArbiterAddressInput = await screen.findByLabelText(/Propose arbiter address input/);
        fireEvent.change(newArbiterAddressInput, {
            target: {
                value: newArbiter.address.toHexString(),
            },
        });
        expect(newArbiterAddressInput).toHaveValue(newArbiter.address.toHexString());

        const newArbiterAssetInput = await screen.findByLabelText(/Propose arbiter asset input/);
        fireEvent.change(newArbiterAssetInput, {
            target: {
                value: ASSETS[1],
            },
        });
        expect(newArbiterAssetInput).toHaveValue(ASSETS[1]);

        const newArbiterFeeInput = await screen.findByLabelText(/Propose arbiter fee input/);
        const newArbiterFeeValue = "0.2";
        fireEvent.change(newArbiterFeeInput, {
            target: {
                value: newArbiterFeeValue
            }
        });
        expect(newArbiterFeeInput).toHaveValue(newArbiterFeeValue);
    });

    it("should be able to propose arbiter", async () => {
        const { user } = renderWithRouter(<App />, {
            route: "/seller",
        });

        const arbiter = wallets[1];
        const buyer = wallets[2];
        const newArbiter = wallets[3];

        // First create an escrow
        const showCreateEscrowBtn = await screen.findByLabelText(/Show create escrow/);
        expect(showCreateEscrowBtn).toBeInTheDocument();
        await user.click(showCreateEscrowBtn);

        const arbiterAddressInput = await screen.findByLabelText(/Create arbiter address input/);
        fireEvent.change(arbiterAddressInput, {
            target: {
                value: arbiter.address.toHexString(),
            },
        });

        const arbiterAssetInput = await screen.findByLabelText(/Create arbiter asset input/);
        fireEvent.change(arbiterAssetInput, {
            target: {
                value: ASSETS[0]
            },
        });

        const arbiterFeeInput = await screen.findByLabelText(/Create arbiter fee input/);
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

        const newArbiterAddressInput = await screen.findByLabelText(/Propose arbiter address input/);
        fireEvent.change(newArbiterAddressInput, {
            target: {
                value: newArbiter.address.toHexString(),
            },
        })

        const newArbiterAssetInput = await screen.findByLabelText(/Propose arbiter asset input/);
        fireEvent.change(newArbiterAssetInput, {
            target: {
                value: ASSETS[1],
            },
        });

        const newArbiterFeeInput = await screen.findByLabelText(/Propose arbiter fee input/);
        fireEvent.change(newArbiterFeeInput, {
            target: {
                value: "0.2"
            }
        });

        const proposeArbiterBtn = await screen.findByLabelText(/Propose arbiter button/);
        expect(proposeArbiterBtn).toBeInTheDocument();
        await user.click(proposeArbiterBtn);
    });
});