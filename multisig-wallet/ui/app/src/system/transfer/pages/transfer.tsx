import { BoxCentered, Button, Heading, Input, RadioGroup, toast, Stack } from "@fuel-ui/react";

export function TransferPage() {

    async function useTransfer() {
        const asset = document.querySelector<HTMLInputElement>(
            `[name="transfer-asset"]`
        )!.value;

        const data = document.querySelector<HTMLInputElement>(
            `[name="transfer-data"]`
        )!.value;

        const signatures = document.querySelector<HTMLInputElement>(
            `[name="transfer-signature"]`
        )!.value;

        const recipient = document.querySelector<HTMLInputElement>(
            `[name="transfer-recipient"]`
        )!.value;

        const value = document.querySelector<HTMLInputElement>(
            `[name="transfer-value"]`
        )!.value;

        toast.error("Unimplemented")
    }

    return (
        <BoxCentered css={{ marginTop: "15%", width: "30%"}}>

            <Stack gap="$1" css={{ minWidth: "100%"}}>

                <Heading as="h3" css={{ marginLeft: "auto", marginRight: "auto", marginBottom: "15%", color: "$accent1"}}>
                    Execute a transfer
                </Heading>

                <Input size="lg">
                    <Input.Field name="transfer-asset" placeholder="Asset id" />
                </Input>
                <Input size="lg">
                    <Input.Field name="transfer-data" placeholder="Optional data" />
                </Input>
                <Input size="lg">
                    <Input.Field name="transfer-signatures" placeholder="Signature" />
                </Input>
                <Input size="lg">
                    <Input.Field name="transfer-recipient" placeholder="Recipient" />
                </Input>
                <Input size="lg">
                    <Input.Number name="transfer-value" placeholder="Value" />
                </Input>
                <Button
                    color="accent"
                    onPress={useTransfer}
                    size="lg"
                    variant="solid"
                >
                    Transfer
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
