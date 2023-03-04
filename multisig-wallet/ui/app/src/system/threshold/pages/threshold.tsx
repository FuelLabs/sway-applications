import { BoxCentered, Button, Heading, Input, toast, Stack } from "@fuel-ui/react";
import { useState } from "react";
import { useContract } from "../../core/hooks";

export function ThresholdPage() {
    const [threshold, setThreshold] = useState("N/A")
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
        setThreshold(String(value));
    }

    return (
        <BoxCentered css={{ marginTop: "15%", width: "30%" }}>

            <Stack gap="$1">

                <Heading as="h3" css={{ marginBottom: "15%", color: "$accent1"}}>
                    Change the threshold for execution
                </Heading>

                <Input size="lg">
                    <Input.Field name="threshold-data" placeholder="Optional data"/>
                </Input>
                <Input size="lg">
                    <Input.Field name="threshold-signature" placeholder="Signature"/>
                </Input>
                <Input size="lg">
                    <Input.Number name="threshold" placeholder="Threshold"/>
                </Input>
                <Button
                    color="accent"
                    onPress={useThreshold}
                    size="lg"
                    variant="solid"
                >
                    Set threshold
                </Button>

                <Heading as="h4" css={{ marginLeft: "auto", marginRight: "auto", marginTop: "15%", color: "$accent1"}}>
                    Current threshold: {threshold}
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
            
        </BoxCentered>
    );
}
