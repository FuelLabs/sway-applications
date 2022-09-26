import { ASSETS } from "@/config";
import { screen, renderWithRouter, fireEvent } from "@escrow/test-utils";
import { App } from "../../App";

describe("Create Escrow", () => {
    it("Should be able to create an escrow", async () => {
        const { user } = renderWithRouter(<App />, {
            route: "/seller",
        });

        const arbiterAddressInput = await screen.findByLabelText(/Arbiter address input/);
        fireEvent.change(arbiterAddressInput, {
            target: {
                value: "",
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
                value: "",
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

        const submitBtn = await screen.findByText(/Create escrow/);
        expect(submitBtn).toBeInTheDocument();
        await user.click(submitBtn);
    });
});