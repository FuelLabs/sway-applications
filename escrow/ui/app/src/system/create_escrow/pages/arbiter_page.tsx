import { Button, Stack } from "@fuel-ui/react";
import { InputFieldComponent, InputNumberComponent, RadioGroupComponent } from "../../common/components";
import { useIsConnected } from "../../core/hooks";

interface ArbiterInterface {
    setArbiter: (address: string) => void,
    setAsset: (asset: string) => void,
    setAssetAmount: (amount: number) => void,
    setRecipient: (type: string) => void,
    setPage: (page: number) => void,
    currentPage: number,
}

export function ArbiterPage( { setArbiter, setAsset, setAssetAmount, setRecipient, setPage, currentPage } : ArbiterInterface) {
    const isConnected = useIsConnected();

    return (
        <Stack css={{ marginLeft: "auto", marginRight: "auto", width: "40%" }}>
            <InputFieldComponent
                onChange={setArbiter}
                text="Arbiter address"
                placeholder="0x80d5e8c2be..."
            />

            <InputFieldComponent
                onChange={setAsset}
                text="Arbiter asset"
                placeholder="0x0000000000..."
            />

            <InputNumberComponent
                onChange={setAssetAmount}
                text="Arbiter payment amount"
                placeholder="1.0"
            />

            <RadioGroupComponent text="Arbiter" handler={setRecipient} />

            <Button
                color="accent"
                onPress={() => setPage(currentPage + 1)}
                size="lg"
                variant="solid"
                isDisabled={!isConnected}
                css={{
                    marginTop: "$10",
                    fontWeight: "$semibold",
                    background: "$pink6",
                    color: "pink",
                }}
                >
                Next
            </Button>

        </Stack>
    );
}
