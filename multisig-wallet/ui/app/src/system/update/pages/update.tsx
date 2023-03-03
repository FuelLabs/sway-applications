import { BoxCentered, Button, Input, toast, Stack } from "@fuel-ui/react";

export function UpdatePage() {

    async function useThreshold() {
        const data = document.querySelector<HTMLInputElement>(
            `[name="threshold-data"]`
        )!.value;

        const signatures = document.querySelector<HTMLInputElement>(
            `[name="threshold-signatures"]`
        )!.value;

        const threshold = document.querySelector<HTMLInputElement>(
            `[name="threshold"]`
        )!.value;

        toast.error("Unimplemented")
    }

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

    return (
        <BoxCentered css={{ margin: "auto" }}>

            <Stack gap="$14">

                <Stack gap="$1">

                    <Input size="md">
                        <Input.Field name="threshold-data" placeholder="Optional data"/>
                    </Input>
                    <Input size="md">
                        <Input.Field name="threshold-signatures" placeholder="Signatures"/>
                    </Input>
                    <Input size="md">
                        <Input.Number name="threshold" placeholder="Threshold"/>
                    </Input>
                    <Button
                        color="accent"
                        onPress={useThreshold}
                        size="md"
                        variant="solid"
                    >
                        Set threshold
                    </Button>

                </Stack>

                <Stack gap="$1">

                    <Input size="md">
                        <Input.Field name="weight-data" placeholder="Optional data"/>
                    </Input>
                    <Input size="md">
                        <Input.Field name="weight-signatures" placeholder="Signatures"/>
                    </Input>
                    <Input size="md">
                        <Input.Field name="weight-address" placeholder="User address"/>
                    </Input>
                    <Input size="md">
                        <Input.Number name="weight" placeholder="User weight"/>
                    </Input>
                    <Button
                        color="accent"
                        onPress={useWeight}
                        size="md"
                        variant="solid"
                    >
                        Set weight
                    </Button>

                </Stack>
            
            </Stack>

        </BoxCentered>
    );
}
