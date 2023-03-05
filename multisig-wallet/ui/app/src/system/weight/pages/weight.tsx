import { BoxCentered, Button, Heading, Input, Stack, toast } from "@fuel-ui/react";
import { useContract } from "../../core/hooks";

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

    return (
        <BoxCentered css={{ marginTop: "12%", width: "30%" }}>

            <Stack css={{ minWidth: "100%" }}>

                <Heading as="h3" css={{ marginLeft: "auto", marginRight: "auto", marginBottom: "$10", color: "$accent1" }}>
                    Change approval weight of user
                </Heading>

                <Input size="lg">
                    <Input.Field name="weight-data" placeholder="Optional data" />
                </Input>
                <Input size="lg">
                    <Input.Field name="weight-signatures" placeholder="Signature" />
                </Input>
                <Input size="lg">
                    <Input.Field name="weight-address" placeholder="Recipient address" />
                </Input>
                <Input size="lg">
                    <Input.Number name="weight" placeholder="New weight" />
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
