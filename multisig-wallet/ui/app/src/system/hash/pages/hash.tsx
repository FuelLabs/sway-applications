import { BoxCentered, Button, Flex, Heading, Input, RadioGroup, Stack, toast } from "@fuel-ui/react";
import { useState } from "react";
import { useContract } from "../../core/hooks";
import { IdentityInput, UserInput } from "../../../contracts/MultisigContractAbi";

export function HashPage() {
    const [radio, setRadio] = useState("address")
    const { contract, isLoading, isError } = useContract()

    async function getExecuteHash() {
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

    async function getTransferHash() {
        const asset = document.querySelector<HTMLInputElement>(
            `[name="transfer-hash-asset"]`
        )!.value;

        const data = document.querySelector<HTMLInputElement>(
            `[name="transfer-hash-data"]`
        )!.value;

        const nonce = document.querySelector<HTMLInputElement>(
            `[name="transfer-hash-nonce"]`
        )!.value;

        let recipient = document.querySelector<HTMLInputElement>(
            `[name="transfer-hash-address"]`
        )!.value;

        const transferValue = document.querySelector<HTMLInputElement>(
            `[name="transfer-hash-value"]`
        )!.value;

        let identity: IdentityInput;

        if (radio === "address") {
            identity = { Address: { value: recipient } };
        } else {
            identity = { ContractId: { value: recipient } };
        }

        // TODO: merge in new hashing function and use instead of this incorrect one
        const { value } = await contract!.functions.transaction_hash(data, nonce, identity, transferValue).get();
        toast.success(`Hash: ${value}`, { duration: 10000 });
    }

    async function getWeightHash() {
        const data = document.querySelector<HTMLInputElement>(
            `[name="weight-hash-data"]`
        )!.value;

        const nonce = document.querySelector<HTMLInputElement>(
            `[name="weight-hash-nonce"]`
        )!.value;

        const userAddress = document.querySelector<HTMLInputElement>(
            `[name="weight-hash-address"]`
        )!.value;

        const userWeight = document.querySelector<HTMLInputElement>(
            `[name="weight-hash"]`
        )!.value;

        let user: UserInput = {
            address: userAddress,
            weight: userWeight
        }

        const { value } = await contract!.functions.weight_hash(data, nonce, user).get();
        toast.success(`Hash: ${value}`, { duration: 10000 });
    }

    async function getThresholdHash() {
        const data = document.querySelector<HTMLInputElement>(
            `[name="threshold-hash-data"]`
        )!.value;

        const nonce = document.querySelector<HTMLInputElement>(
            `[name="threshold-hash-nonce"]`
        )!.value;

        let threshold = document.querySelector<HTMLInputElement>(
            `[name="threshold-hash"]`
        )!.value;

        // TODO: merge in new hashing function and use instead of this incorrect one
        const { value } = await contract!.functions.threshold_hash(data, nonce, threshold).get();
        toast.success(`Hash: ${value}`, { duration: 10000 });
    }

    return (
        <BoxCentered css={{ marginTop: "12%", width: "30%" }}>

            <Stack>

                <Flex gap="130px" css={{ marginBottom: "$14" }}>

                    <Stack>
                        <Heading as="h4" css={{ marginLeft: "auto", marginRight: "auto", color: "$accent1" }}>
                            Hash for execution
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
                            onPress={getExecuteHash}
                            size="lg"
                            variant="solid"
                        >
                            Create hash
                        </Button>
                    </Stack>

                    <Stack>
                        <Heading as="h4" css={{ marginLeft: "auto", marginRight: "auto", color: "$accent1" }}>
                            Hash for user weight
                        </Heading>

                        <Input size="lg">
                            <Input.Field name="weight-hash-data" placeholder="Optional data" />
                        </Input>
                        <Input size="lg">
                            <Input.Number name="weight-hash-nonce" placeholder="Nonce" />
                        </Input>
                        <Input size="lg">
                            <Input.Field name="weight-hash-address" placeholder="Recipient address" />
                        </Input>
                        <Input size="lg">
                            <Input.Number name="weight-hash" placeholder="New weight" />
                        </Input>
                        <Button
                            color="accent"
                            onPress={getWeightHash}
                            size="lg"
                            variant="solid"
                        >
                            Create hash
                        </Button>
                    </Stack>

                </Flex>

                <Flex gap="130px" css={{ marginBottom: "$14" }}>

                    <Stack>
                        <Heading as="h4" css={{ marginLeft: "auto", marginRight: "auto", color: "$accent1"}}>
                            Hash for transfer
                        </Heading>

                        <Input size="lg">
                            <Input.Field name="transfer-hash-asset" placeholder="Asset Id" />
                        </Input>
                        <Input size="lg">
                            <Input.Field name="transfer-hash-data" placeholder="Optional data" />
                        </Input>
                        <Input size="lg">
                            <Input.Number name="transfer-hash-nonce" placeholder="Nonce" />
                        </Input>
                        <Input size="lg">
                            <Input.Field name="transfer-hash-address" placeholder="Recipient address" />
                        </Input>
                        <Input size="lg">
                            <Input.Number name="transfer-hash-value" placeholder="Value" />
                        </Input>
                        <Button
                            color="accent"
                            onPress={getTransferHash}
                            size="lg"
                            variant="solid"
                        >
                            Create hash
                        </Button>
                    </Stack>

                    <Stack>
                        <Heading as="h4" css={{ marginLeft: "auto", marginRight: "auto", color: "$accent1" }}>
                            Hash for threshold 
                        </Heading>

                        <Input size="lg">
                            <Input.Field name="threshold-hash-data" placeholder="Optional data" />
                        </Input>
                        <Input size="lg">
                            <Input.Number name="threshold-hash-nonce" placeholder="Nonce" />
                        </Input>
                        <Input size="lg">
                            <Input.Number name="threshold-hash" placeholder="New threshold" />
                        </Input>
                        <Button
                            color="accent"
                            onPress={getThresholdHash}
                            size="lg"
                            variant="solid"
                        >
                            Create hash
                        </Button>
                    </Stack>

                </Flex>

                <Heading as="h4" css={{ marginLeft: "auto", marginRight: "auto", color: "$accent1" }}>
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
