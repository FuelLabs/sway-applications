import { BoxCentered, Button, Heading, Input, RadioGroup, Text, toast, Stack } from "@fuel-ui/react";
import { useState } from "react";
import { useContract } from "../../core/hooks";


export function ExecuteTransactionPage() {
    const [radio, setRadio] = useState("address")
    const { contract, isLoading, isError } = useContract()

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
        <BoxCentered css={{ marginTop: "12%", width: "30%" }}>

            <Stack css={{ minWidth: "100%" }}>

                <Heading as="h3" css={{ marginLeft: "auto", marginRight: "auto", marginBottom: "$10", color: "$accent1" }}>
                    Execute a transaction
                </Heading>

                <Text color="blackA12">Recipient address</Text>
                <Input size="lg">
                    <Input.Field name="transaction-recipient" placeholder="0x80d5e8c2be..." />
                </Input>

                <Text color="blackA12">Asset amount</Text>
                <Input size="lg">
                    <Input.Number name="execute-value" placeholder="1.0" />
                </Input>

                <Text color="blackA12">Signature</Text>
                <Input size="lg">
                    <Input.Field name="transaction-signature" placeholder="9c3f5ae085a4..." />
                </Input>

                <Text color="blackA12">Optional data</Text>
                <Input size="lg">
                    <Input.Field name="transaction-data" placeholder="0x252afeeb6e..." />
                </Input>
                
                <Button
                    color="accent"
                    onPress={useExecuteTransaction}
                    size="lg"
                    variant="solid"
                >
                    Execute
                </Button>

                <Heading as="h4" css={{ marginLeft: "auto", marginRight: "auto", marginTop: "$14", color: "$accent1" }}>
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
