import { BoxCentered, Button, Heading, Input, RadioGroup, Text, toast, Stack } from "@fuel-ui/react";
import { useState } from "react";
import { useContract } from "../../core/hooks";

export function TransferPage() {
    const [radio, setRadio] = useState("address")
    const { contract, isLoading, isError } = useContract()

    async function useTransfer() {
        const asset = document.querySelector<HTMLInputElement>(
            `[name="transfer-asset"]`
        )!.value;

        const data = document.querySelector<HTMLInputElement>(
            `[name="transfer-data"]`
        )!.value;

        const nonce = document.querySelector<HTMLInputElement>(
            `[name="transfer-hash-nonce"]`
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
        <BoxCentered css={{ marginTop: "12%", width: "30%" }}>

            <Stack css={{ minWidth: "100%" }}>

                <Heading as="h3" css={{ marginLeft: "auto", marginRight: "auto", marginBottom: "$10", color: "$accent1"}}>
                    Execute a transfer
                </Heading>

                <Text color="blackA12">Recipient address</Text>
                <Input size="lg">
                    <Input.Field name="transfer-recipient" placeholder="0x80d5e8c2be..." />
                </Input>

                <Text color="blackA12">Asset id</Text>
                <Input size="lg">
                    <Input.Field name="transfer-asset" placeholder="0x0000000000..." />
                </Input>

                <Text color="blackA12">Asset amount</Text>
                <Input size="lg">
                    <Input.Number name="transfer-value" placeholder="1.0" />
                </Input>

                <Text color="blackA12">Signature</Text>
                <Input size="lg">
                    <Input.Field name="transfer-signatures" placeholder="9c3f5ae085a4..." />
                </Input>

                <Text color="blackA12">Optional data</Text>
                <Input size="lg">
                    <Input.Field name="transfer-data" placeholder="0x252afeeb6e..." />
                </Input>

                <Button
                    color="accent"
                    onPress={useTransfer}
                    size="lg"
                    variant="solid"
                >
                    Transfer
                </Button>

                <Heading as="h4" css={{ marginLeft: "auto", marginRight: "auto", marginTop: "$14", color: "$accent1"}}>
                    Recipient Type
                </Heading>

                <RadioGroup defaultValue="address" direction="row" css={{ margin: "auto" }}>
                    {/* 
                        TODO: 
                            change labels to be the color black
                            increase the size of the buttons and text 
                    */}
                    <RadioGroup.Item onClick={() => setRadio("address")} label="Address" value="address" />
                    <RadioGroup.Item onClick={() => setRadio("contract")} label="Contract" value="contract" />
                </RadioGroup>

            </Stack>
            
        </BoxCentered>
    );
}
