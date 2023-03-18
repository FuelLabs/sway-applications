import { Button, Flex, Stack } from "@fuel-ui/react";
import { InputFieldComponent, InputNumberComponent, RadioGroupComponent } from "../../common/components";
import { useIsConnected } from "../../core/hooks";

interface BuyerDeadlineInterface {
    setBuyer: (address: string) => void,
    setDeadline: (amount: number) => void,
    setRecipient: (address: string) => void,
    setPage: (page: number) => void,
    currentPage: number,
}

export function BuyerDeadlinePage( { setBuyer, setDeadline, setRecipient, setPage, currentPage } : BuyerDeadlineInterface) {
    const isConnected = useIsConnected();

    return (
        <Stack css={{ marginLeft: "auto", marginRight: "auto", width: "40%" }}>
            <InputFieldComponent
                onChange={setBuyer}
                text="Buyer address"
                placeholder="0x80d5e8c2be..."
            />

            <InputNumberComponent
                onChange={setDeadline}
                text="Deadline (block height)"
                placeholder="621"
            />

            <RadioGroupComponent handler={setRecipient} />

            <Flex gap="$1" css={{ marginTop: "$10" }}>
                <Button
                    color="accent"
                    onPress={() => setPage(currentPage - 1)}
                    size="lg"
                    variant="solid"
                    isDisabled={!isConnected}
                    css={{
                        fontWeight: "$semibold",
                        background: "$pink6",
                        color: "pink",
                        width: "100%",
                    }}
                    >
                    Back
                </Button>

                <Button
                    color="accent"
                    onPress={() => setPage(currentPage + 1)}
                    size="lg"
                    variant="solid"
                    isDisabled={!isConnected}
                    css={{
                        fontWeight: "$semibold",
                        background: "$pink6",
                        color: "pink",
                        width: "100%",
                    }}
                    >
                    Next
                </Button>
            </Flex>
        </Stack>
    );
}
