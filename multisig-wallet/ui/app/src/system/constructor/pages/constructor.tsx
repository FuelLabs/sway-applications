import { BoxCentered, Button, Flex, Heading, Input, RadioGroup, toast, Stack } from "@fuel-ui/react";
import { useContract } from "../../core/hooks";
import { UserInput } from "../../../contracts/MultisigContractAbi";

export function ConstructorPage() {
    const { contract, isLoading, isError } = useContract()

    async function useConstructor() {
        const userAddress = document.querySelector<HTMLInputElement>(
            `[name="create-recipient"]`
        )!.value;

        const userWeight = document.querySelector<HTMLInputElement>(
            `[name="create-weight"]`
        )!.value;

        let user: UserInput = {
            address: userAddress,
            weight: userWeight
        }

        await contract!.functions.constructor([user]).call();
        toast.success("Wallet created!", { duration: 10000 });
    }

    return (
        <BoxCentered css={{ marginTop: "12%", width: "30%" }}>

            <Stack gap="$1" css={{ minWidth: "100%" }}>

                <Heading as="h3" css={{ marginLeft: "auto", marginRight: "auto", color: "$accent1"}}>
                    Create a new wallet
                </Heading>

                <Input size="lg">
                    <Input.Field name="create-recipient" placeholder="Recipient address" />
                </Input>
                <Input size="lg">
                    <Input.Number name="create-weight" placeholder="Recipient weight"/>
                </Input>
                <Button
                    color="accent"
                    onPress={useConstructor}
                    size="lg"
                    variant="solid"
                >
                    Create
                </Button>
            </Stack>
            
        </BoxCentered>
    );
}
