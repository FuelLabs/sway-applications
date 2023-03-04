import { BoxCentered, Button, Heading, Input, RadioGroup, toast, Stack } from "@fuel-ui/react";

export function ExecuteTransactionPage() {

    async function useExecuteTransaction() {
        const data = document.querySelector<HTMLInputElement>(
            `[name="transaction-data"]`
        )!.value;

        const signature = document.querySelector<HTMLInputElement>(
            `[name="transaction-signature"]`
        )!.value;

        const recipient = document.querySelector<HTMLInputElement>(
            `[name="transaction-recipient"]`
        )!.value;

        const value = document.querySelector<HTMLInputElement>(
            `[name="execute-value"]`
        )!.value;

        toast.error("Unimplemented")
    }

    return (
        <BoxCentered css={{ marginTop: "15%", width: "30%"}}>

            <Stack gap="$1" css={{ minWidth: "100%"}}>

                <Heading as="h3" css={{ marginLeft: "auto", marginRight: "auto", marginBottom: "15%", color: "$accent1"}}>
                    Execute a transaction
                </Heading>

                <Input size="lg">
                    <Input.Field name="transaction-data" placeholder="Optional data" />
                </Input>
                <Input size="lg">
                    <Input.Field name="transaction-signature" placeholder="Signature" />
                </Input>
                <Input size="lg">
                    <Input.Field name="transaction-recipient" placeholder="Recipient" />
                </Input>
                <Input size="lg">
                    <Input.Number name="execute-value" placeholder="Value" />
                </Input>
                <Button
                    color="accent"
                    onPress={useExecuteTransaction}
                    size="lg"
                    variant="solid"
                >
                    Execute
                </Button>

                <Heading as="h4" css={{ marginLeft: "auto", marginRight: "auto", marginTop: "15%", color: "$accent1"}}>
                    Recipient Type
                </Heading>

                <RadioGroup defaultValue="address" direction="row" css={{ margin: "auto" }}>
                    {/* 
                        TODO: 
                            change labels to be the color black
                            increase the size of the buttons and text 
                    */}
                    <RadioGroup.Item id="address" label="Address" value="address" />
                    <RadioGroup.Item id="contract" label="Contract" value="contract" />
                </RadioGroup>

            </Stack>
            
        </BoxCentered>
    );
}
