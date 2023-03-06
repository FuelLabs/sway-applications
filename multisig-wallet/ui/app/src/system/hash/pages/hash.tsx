import { BoxCentered, Button, Checkbox, Flex, Form, Heading, Input, RadioGroup, Stack, Text, toast } from "@fuel-ui/react";
import { useState } from "react";
import { useContract } from "../../core/hooks";
import { IdentityInput, UserInput } from "../../../contracts/MultisigContractAbi";

export function HashPage() {
    const [radio, setRadio] = useState("address")
    const [optionalData, setOptionalData] = useState(false)
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
        <BoxCentered css={{ marginTop: "3%", width: "30%" }}>

            <Stack>

                <Flex gap="130px" css={{ marginBottom: "$14" }}>

                    <Stack>
                        <Heading as="h4" css={{ marginLeft: "auto", marginRight: "auto", color: "$accent1" }}>
                            Hash for execution
                        </Heading>

                        <Text color="blackA12">Recipient address</Text>
                        <Input size="lg">
                            <Input.Field name="execute-hash-address" placeholder="0x80d5e8c2be..." />
                        </Input>

                        <Text color="blackA12">Asset amount</Text>
                        <Input size="lg">
                            <Input.Number name="execute-hash-value" placeholder="1.0" />
                        </Input>

                        <Text color="blackA12">Nonce</Text>
                        <Input size="lg">
                            <Input.Number name="execute-hash-nonce" placeholder="3" />
                        </Input>

                        {optionalData && 
                            <>
                                <Text color="blackA12">Optional data</Text>
                                <Input size="lg">
                                    <Input.Field name="execute-hash-data" placeholder="0x252afeeb6e..." />
                                </Input>
                            </>
                        }

                        <Button
                            color="accent"
                            onPress={getExecuteHash}
                            size="lg"
                            variant="solid"
                            css={{ marginTop: "$1" }}
                        >
                            Create hash
                        </Button>
                    </Stack>

                    <Stack>
                        <Heading as="h4" css={{ marginLeft: "auto", marginRight: "auto", color: "$accent1" }}>
                            Hash for user weight
                        </Heading>

                        <Text color="blackA12">Recipient address</Text>
                        <Input size="lg">
                            <Input.Field name="weight-hash-address" placeholder="0x80d5e8c2be..." />
                        </Input>

                        <Text color="blackA12">New weight</Text>
                        <Input size="lg">
                            <Input.Number name="weight-hash" placeholder="2" />
                        </Input>

                        <Text color="blackA12">Nonce</Text>
                        <Input size="lg">
                            <Input.Number name="weight-hash-nonce" placeholder="3" />
                        </Input>

                        {optionalData && 
                            <>
                                <Text color="blackA12">Optional data</Text>
                                <Input size="lg">
                                    <Input.Field name="weight-hash-data" placeholder="0x252afeeb6e..." />
                                </Input>
                            </>
                        }

                        <Button
                            color="accent"
                            onPress={getWeightHash}
                            size="lg"
                            variant="solid"
                            css={{ marginTop: "$1" }}
                        >
                            Create hash
                        </Button>
                    </Stack>

                </Flex>

                <Flex gap="130px" css={{ marginBottom: "$10" }}>

                    <Stack>
                        <Heading as="h4" css={{ marginLeft: "auto", marginRight: "auto", color: "$accent1"}}>
                            Hash for transfer
                        </Heading>

                        <Text color="blackA12">Recipient address</Text>
                        <Input size="lg">
                            <Input.Field name="transfer-hash-address" placeholder="0x80d5e8c2be..." />
                        </Input>

                        <Text color="blackA12">Asset id</Text>
                        <Input size="lg">
                            <Input.Field name="transfer-hash-asset" placeholder="0x0000000000..." />
                        </Input>

                        <Text color="blackA12">Asset amount</Text>
                        <Input size="lg">
                            <Input.Number name="transfer-hash-value" placeholder="1.0" />
                        </Input>

                        <Text color="blackA12">Nonce</Text>
                        <Input size="lg">
                            <Input.Number name="transfer-hash-nonce" placeholder="3" />
                        </Input>

                        {optionalData && 
                            <>
                                <Text color="blackA12">Optional data</Text>
                                <Input size="lg">
                                    <Input.Field name="transfer-hash-data" placeholder="0x252afeeb6e..." />
                                </Input>
                            </>
                        }

                        <Button
                            color="accent"
                            onPress={getTransferHash}
                            size="lg"
                            variant="solid"
                            css={{ marginTop: "$1" }}
                        >
                            Create hash
                        </Button>
                    </Stack>

                    <Stack>
                        <Heading as="h4" css={{ marginLeft: "auto", marginRight: "auto", color: "$accent1" }}>
                            Hash for threshold 
                        </Heading>

                        <Text color="blackA12">Threshold</Text>
                        <Input size="lg">
                            <Input.Number name="threshold-hash" placeholder="8" />
                        </Input>

                        <Text color="blackA12">Nonce</Text>
                        <Input size="lg">
                            <Input.Number name="threshold-hash-nonce" placeholder="3" />
                        </Input>

                        {optionalData && 
                            <>
                                <Text color="blackA12">Optional data</Text>
                                <Input size="lg">
                                    <Input.Field name="threshold-hash-data" placeholder="0x252afeeb6e..." />
                                </Input>
                            </>
                        }

                        <Button
                            color="accent"
                            onPress={getThresholdHash}
                            size="lg"
                            variant="solid"
                            css={{ marginTop: "$1" }}
                        >
                            Create hash
                        </Button>
                    </Stack>

                </Flex>

                <BoxCentered css={{ marginTop: "$8" }}>
                    <Form.Control css={{ flexDirection: 'row' }}>
                        <Checkbox onClick={() => setOptionalData(!optionalData)} id="optional-data"/>
                        <Form.Label htmlFor="optional-data">
                            Optional data
                        </Form.Label>
                    </Form.Control>
                </BoxCentered>

                <Heading as="h4" css={{ marginLeft: "auto", marginRight: "auto", marginTop: "$8", color: "$accent1" }}>
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
