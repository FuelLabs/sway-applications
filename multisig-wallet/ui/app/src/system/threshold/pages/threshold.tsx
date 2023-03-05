import { BoxCentered, Button, Heading, Input, Text, toast, Stack } from "@fuel-ui/react";
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

    return (
        <BoxCentered css={{ marginTop: "12%", width: "30%" }}>

            <Stack css={{ minWidth: "100%" }}>

                <Heading as="h3" css={{ marginLeft: "auto", marginRight: "auto", marginBottom: "$10", color: "$accent1"}}>
                    Change threshold for execution
                </Heading>

                <Text color="blackA12">Threshold</Text>
                <Input size="lg">
                    <Input.Number name="threshold" placeholder="8" />
                </Input>

                <Text color="blackA12">Signature</Text>
                <Input size="lg">
                    <Input.Field name="threshold-signature" placeholder="9c3f5ae085a4..." />
                </Input>

                <Text color="blackA12">Optional data</Text>
                <Input size="lg">
                    <Input.Field name="threshold-data" placeholder="0x252afeeb6e..." />
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
