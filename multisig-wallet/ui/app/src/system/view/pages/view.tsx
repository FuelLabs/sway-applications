import { BoxCentered, Button, Flex, Heading, Stack, toast } from "@fuel-ui/react";
import { useState } from "react";
import { useContract } from "../../core/hooks";
import { ContractIdInput } from "../../../contracts/MultisigContractAbi";
import { InputFieldComponent } from "../../common/components/input_field";
import { validateAddress } from "../../common/utils/validate_address";
import { validateContractId } from "../../common/utils/validate_contract_id";

export function ViewPage() {
    // Used for our component listeners 
    const [address, setAddress] = useState("")
    const [asset, setAsset] = useState("")

    const { contract, isLoading, isError } = useContract()
    
    async function getBalance() {
        let { address: userAsset, isError } = validateContractId(asset);
        if (isError) return;

        let assetId: ContractIdInput = {
            value: userAsset
        } 

        const { value } = await contract!.functions.balance(assetId).get();
        toast.success(`Balance: ${value}`, { duration: 10000 });
    }

    async function getNonce() {
        const { value } = await contract!.functions.nonce().get();
        toast.success(`Current nonce: ${value}`, { duration: 10000 });
    }

    async function getThreshold() {
        const { value } = await contract!.functions.threshold().get();
        toast.success(`Threshold: ${value}`, { duration: 10000 });
    }

    async function getWeight() {
        let { address: user, isError } = validateAddress(address);
        if (isError) return;

        const { value } = await contract!.functions.approval_weight(user).get();
        toast.success(`User weight: ${value}`, { duration: 10000 });
    }

    return (
        <BoxCentered css={{ marginTop: "12%", width: "30%" }}>
            <Stack css={{ minWidth: "100%" }}>
                <Stack>
                    <Heading as="h3" css={{ marginLeft: "auto", marginRight: "auto", color: "$accent1" }}>
                        Check user approval weight
                    </Heading>

                    <InputFieldComponent onChange={setAddress} text="User address" placeholder="0x80d5e8c2be..." name="user-weight" />

                    <Button
                        color="accent"
                        onPress={getWeight}
                        size="lg"
                        variant="solid"
                    >
                        Get weight
                    </Button>
                </Stack>

                <Stack css={{ minWidth: "100%", marginTop: "$10" }}>
                    <Heading as="h3" css={{ marginLeft: "auto", marginRight: "auto", color: "$accent1" }}>
                        Check balance of asset
                    </Heading>

                    <InputFieldComponent onChange={setAsset} text="Asset id" placeholder="0x0000000000..." name="view-asset" />

                    <Button
                        color="accent"
                        onPress={getBalance}
                        size="lg"
                        variant="solid"
                    >
                        Get balance
                    </Button>
                </Stack>

                <Flex gap="$1" css={{ minWidth: "100%", marginTop: "$10" }}>
                    <Stack css={{ minWidth: "50%" }}>
                        <Heading as="h3" css={{ marginLeft: "auto", marginRight: "auto", marginTop: "$14", color: "$accent1" }}>
                            Check nonce
                        </Heading>

                        <Button
                            color="accent"
                            onPress={getNonce}
                            size="lg"
                            variant="solid"
                        >
                            Get nonce
                        </Button>
                    </Stack>

                    <Stack css={{ minWidth: "50%" }}>
                        <Heading as="h3" css={{ marginLeft: "auto", marginRight: "auto", marginTop: "$14", color: "$accent1" }}>
                            Check threshold
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
                </Flex>
            </Stack>
        </BoxCentered>
    );
}
