import { BoxCentered, Button, Heading, Input, toast, Stack } from "@fuel-ui/react";
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
