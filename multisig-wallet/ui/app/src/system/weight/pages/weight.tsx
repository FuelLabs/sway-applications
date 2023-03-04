import { BoxCentered, Button, Heading, Input, toast, Stack } from "@fuel-ui/react";
import { useState } from "react";
import { useContract } from "../../core/hooks";

export function WeightPage() {
    const [weight, setWeight] = useState("N/A")
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

        // TODO: error handling
        const { value } = await contract!.functions.approval_weight(user).get();
        setWeight(String(value));
    }

    return (
        <BoxCentered css={{ marginTop: "15%" }}>

            <Stack gap="$1">

                <Heading as="h3" css={{ marginBottom: "15%", color: "$accent1"}}>
                    Change the approval weight of a user
                </Heading>

                <Input size="lg">
                    <Input.Field name="weight-data" placeholder="Optional data"/>
                </Input>
                <Input size="lg">
                    <Input.Field name="weight-signatures" placeholder="Signature"/>
                </Input>
                <Input size="lg">
                    <Input.Field name="weight-address" placeholder="User address"/>
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

                <Heading as="h4" css={{ marginLeft: "auto", marginRight: "auto", marginTop: "15%", color: "$accent1"}}>
                    User Weight: {weight}
                </Heading>

                <Input size="lg">
                    <Input.Field name="user-weight" placeholder="User address"/>
                </Input>

                <Button
                    color="accent"
                    onPress={getWeight}
                    size="lg"
                    variant="solid"
                >
                    Check user weight
                </Button>

            </Stack>
            
        </BoxCentered>
    );
}
