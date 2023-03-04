import { BoxCentered, Button, Flex, Heading, Input, toast, Stack } from "@fuel-ui/react";
import { useContract } from "../../core/hooks";

export function ThresholdPage() {
    const { contract, isLoading, isError } = useContract()

    async function useThreshold() {
        const data = document.querySelector<HTMLInputElement>(
            `[name="threshold-data"]`
        )!.value;

        const signatures = document.querySelector<HTMLInputElement>(
            `[name="threshold-signature"]`
        )!.value;

        const threshold = document.querySelector<HTMLInputElement>(
            `[name="threshold"]`
        )!.value;

        toast.error("Unimplemented")
    }

    async function getThreshold() {
        const { value } = await contract!.functions.threshold().get();
        toast.success(`Threshold: ${value}`, { duration: 10000 });
    }

    async function getHash() {
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

    async function getNonce() {
        const { value } = await contract!.functions.nonce().get();
        toast.success(`Current nonce: ${value}`, { duration: 10000 });
    }

    return (
        <BoxCentered css={{ marginTop: "8%" }}>

            <Stack gap="$1">

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
                            css={{ marginBottom: "$2" }}
                        >
                            Get nonce
                        </Button>

                        <Heading as="h4" css={{ marginLeft: "auto", marginRight: "auto", marginTop: "$14", color: "$accent1" }}>
                            Check current threshold
                        </Heading>

                        <Button
                            color="accent"
                            onPress={getThreshold}
                            size="lg"
                            variant="solid"
                        >
                            Get threshold
                        </Button>

                    </Stack>

                    <Stack>

                        <Heading as="h4" css={{ marginLeft: "auto", marginRight: "auto", color: "$accent1" }}>
                            Create transaction hash to sign
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
                            onPress={getHash}
                            size="lg"
                            variant="solid"
                        >
                            Create hash
                        </Button>

                    </Stack>

                </Flex>

                <Heading as="h3" css={{ marginLeft: "auto", marginRight: "auto", marginBottom: "$10", color: "$accent1"}}>
                    Change the threshold for execution
                </Heading>

                <Input size="lg">
                    <Input.Field name="threshold-data" placeholder="Optional data" />
                </Input>
                <Input size="lg">
                    <Input.Field name="threshold-signature" placeholder="Signature" />
                </Input>
                <Input size="lg">
                    <Input.Number name="threshold" placeholder="Threshold" />
                </Input>
                <Button
                    color="accent"
                    onPress={useThreshold}
                    size="lg"
                    variant="solid"
                >
                    Set threshold
                </Button>

            </Stack>
            
        </BoxCentered>
    );
}
