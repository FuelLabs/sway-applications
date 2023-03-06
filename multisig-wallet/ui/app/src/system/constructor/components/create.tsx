import { BoxCentered, Button, Heading, Input, RadioGroup, Text, toast, Stack } from "@fuel-ui/react";
import { useContract } from "../../core/hooks";
import { UserInput } from "../../../contracts/MultisigContractAbi";
import { InputFieldComponent } from "../../common/input_field";
import { InputNumberComponent } from "../../common/input_number";

export function CreateWallet() {
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

            <Stack css={{ minWidth: "100%" }}>

                <Heading as="h3" css={{ marginLeft: "auto", marginRight: "auto", marginBottom: "$10", color: "$accent1"}}>
                    Create a new wallet
                </Heading>

                <InputFieldComponent text="User address" placeholder="0x80d5e8c2be..." name="create-recipient" />
                <InputNumberComponent text="Recipient weight" placeholder="2" name="create-weight" />

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
