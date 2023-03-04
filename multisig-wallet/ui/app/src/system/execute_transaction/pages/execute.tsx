import { BoxCentered, Button, Flex, Heading, Input, RadioGroup, toast, Stack } from "@fuel-ui/react";
import { useState } from "react";
import { useContract } from "../../core/hooks";
import { IdentityInput } from "../../../contracts/MultisigContractAbi";


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

    async function getHash() {
        const data = document.querySelector<HTMLInputElement>(
            `[name="execute-hash-data"]`
        )!.value;

        const nonce = document.querySelector<HTMLInputElement>(
            `[name="execute-hash-nonce"]`
        )!.value;

        let recipient = document.querySelector<HTMLInputElement>(
            `[name="execute-hash-address"]`
        )!.value;

        const executeValue = document.querySelector<HTMLInputElement>(
            `[name="execute-hash-value"]`
        )!.value;

        let identity: IdentityInput;

        if (radio === "address") {
            identity = { Address: { value: recipient } };
        } else {
            identity = { ContractId: { value: recipient } };
        }

        const { value } = await contract!.functions.transaction_hash(data, nonce, identity, executeValue).get();
        toast.success(`Hash: ${value}`, { duration: 10000 });
    }

    async function getNonce() {
        const { value } = await contract!.functions.nonce().get();
        toast.success(`Current nonce: ${value}`, { duration: 10000 });
    }

    return (
        <BoxCentered css={{ marginTop: "8%" }}>

            <Stack gap="$1" css={{ minWidth: "100%" }}>

                <Flex gap="$24" css={{ marginBottom: "$14" }}>

                    <Stack>

                        <Heading as="h4" css={{ marginLeft: "auto", marginRight: "auto", color: "$accent1" }}>
                            Check current nonce
                        </Heading>

                        <Button
                            color="accent"
                            onPress={getNonce}
                            size="lg"
                            variant="solid"
                        >
                            Get nonce
                        </Button>

                    </Stack>

                    <Stack>

                        <Heading as="h4" css={{ marginLeft: "auto", marginRight: "auto", color: "$accent1" }}>
                            Create transaction hash to sign
                        </Heading>

                        <Input size="lg">
                            <Input.Field name="execute-hash-data" placeholder="Optional data" />
                        </Input>
                        <Input size="lg">
                            <Input.Number name="execute-hash-nonce" placeholder="Nonce" />
                        </Input>
                        <Input size="lg">
                            <Input.Field name="execute-hash-address" placeholder="Recipient address" />
                        </Input>
                        <Input size="lg">
                            <Input.Number name="execute-hash-value" placeholder="Value" />
                        </Input>
                        <Button
                            color="accent"
                            onPress={getHash}
                            size="lg"
                            variant="solid"
                        >
                            Create hash
                        </Button>

                    </Stack>

                </Flex>

                <Heading as="h3" css={{ marginLeft: "auto", marginRight: "auto", marginBottom: "$10", color: "$accent1" }}>
                    Execute a transaction
                </Heading>

                <Input size="lg">
                    <Input.Field name="transaction-data" placeholder="Optional data" />
                </Input>
                <Input size="lg">
                    <Input.Field name="transaction-signature" placeholder="Signature" />
                </Input>
                <Input size="lg">
                    <Input.Field name="transaction-recipient" placeholder="Recipient address" />
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
