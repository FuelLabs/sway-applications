import { BoxCentered, Button, Flex, Heading, Input, Stack, toast } from "@fuel-ui/react";
import { useContract } from "../../core/hooks";
import { UserInput } from "../../../contracts/MultisigContractAbi";

export function WeightPage() {
    const { contract, isLoading, isError } = useContract()

    async function useWeight() {
        const data = document.querySelector<HTMLInputElement>(
            `[name="weight-data"]`
        )!.value;

        const signatures = document.querySelector<HTMLInputElement>(
            `[name="weight-signatures"]`
        )!.value;

        const userAddress = document.querySelector<HTMLInputElement>(
            `[name="weight"]`
        )!.value;

        const userWeight = document.querySelector<HTMLInputElement>(
            `[name="weight"]`
        )!.value;

        toast.error("Unimplemented")
    }

    async function getWeight() {
        const user = document.querySelector<HTMLInputElement>(
            `[name="user-weight"]`
        )!.value;

        const { value } = await contract!.functions.approval_weight(user).get();
        toast.success(`User weight: ${value}`, { duration: 10000 });
    }

    async function getNonce() {
        const { value } = await contract!.functions.nonce().get();
        toast.success(`Current nonce: ${value}`, { duration: 10000 });
    }

    async function getHash() {
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

    return (
        <BoxCentered css={{ marginTop: "8%" }}>

            <Stack gap="$1">

                <Flex gap="$24" css={{ marginBottom: "$14" }}>

                    <Stack>

                        <Heading as="h4" css={{ marginLeft: "auto", marginRight: "auto", color: "$accent1"}}>
                            Check user approval weight
                        </Heading>

                        <Input size="lg">
                            <Input.Field name="user-weight" placeholder="User address"/>
                        </Input>

                        <Button
                            color="accent"
                            onPress={getWeight}
                            size="lg"
                            variant="solid"
                            css={{ marginBottom: "$2" }}
                        >
                            Get weight
                        </Button>

                        <Heading as="h4" css={{ marginLeft: "auto", marginRight: "auto", marginTop: "$14", color: "$accent1"}}>
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

                        <Heading as="h4" css={{ marginLeft: "auto", marginRight: "auto", color: "$accent1"}}>
                            Create transaction hash to sign
                        </Heading>

                        <Input size="lg">
                            <Input.Field name="weight-hash-data" placeholder="Optional data"/>
                        </Input>
                        <Input size="lg">
                            <Input.Number name="weight-hash-nonce" placeholder="Nonce"/>
                        </Input>
                        <Input size="lg">
                            <Input.Field name="weight-hash-address" placeholder="Recipient address"/>
                        </Input>
                        <Input size="lg">
                            <Input.Number name="weight-hash" placeholder="New weight"/>
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

                <Heading as="h3" css={{ marginLeft: "auto", marginRight: "auto", marginBottom: "$10", marginTop: "$10", color: "$accent1"}}>
                    Change the approval weight of a user
                </Heading>

                <Input size="lg">
                    <Input.Field name="weight-data" placeholder="Optional data"/>
                </Input>
                <Input size="lg">
                    <Input.Field name="weight-signatures" placeholder="Signature"/>
                </Input>
                <Input size="lg">
                    <Input.Field name="weight-address" placeholder="Recipient address"/>
                </Input>
                <Input size="lg">
                    <Input.Number name="weight" placeholder="New weight"/>
                </Input>
                <Button
                    color="accent"
                    onPress={useWeight}
                    size="lg"
                    variant="solid"
                >
                    Set weight
                </Button>

            </Stack>
            
        </BoxCentered>
    );
}
