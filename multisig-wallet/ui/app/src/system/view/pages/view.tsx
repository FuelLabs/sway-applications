import { BoxCentered, Button, Heading, Input, Stack, toast } from "@fuel-ui/react";
import { useContract } from "../../core/hooks";
import { ContractIdInput } from "../../../contracts/MultisigContractAbi";

export function ViewPage() {
    const { contract, isLoading, isError } = useContract()

    async function getBalance() {
        const asset = document.querySelector<HTMLInputElement>(
            `[name="view-asset"]`
        )!.value;

        let assetId: ContractIdInput = {
            value: asset
        } 

        const { value } = await contract!.functions.balance(assetId).get();
        toast.success(`Balance: ${value}`, { duration: 10000 });
    }

    return (
        <BoxCentered css={{ marginTop: "8%", width: "30%" }}>

            <Stack gap="$2" css={{ minWidth: "100%" }}>

                <Heading as="h3" css={{ marginLeft: "auto", marginRight: "auto", color: "$accent1" }}>
                    Check balance of asset
                </Heading>

                <Input size="lg">
                    <Input.Field name="view-asset" placeholder="Asset Id" />
                </Input>
                <Button
                    color="accent"
                    onPress={getBalance}
                    size="lg"
                    variant="solid"
                >
                    Get balance
                </Button>
            </Stack>
            
        </BoxCentered>
    );
}
