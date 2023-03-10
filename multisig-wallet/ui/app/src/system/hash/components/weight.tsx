import { Button, Heading, Stack, toast } from "@fuel-ui/react";
import { useState } from "react";
import { useContract } from "../../core/hooks";
import { UserInput } from "../../../contracts/MultisigContractAbi";
import { InputFieldComponent } from "../../common/components/input_field";
import { InputNumberComponent } from "../../common/components/input_number";
import { validateOptionalData } from "../../common/utils/validate_optional_data";
import { validateAddress } from "../../common/utils/validate_address";
import { useIsConnected } from "../../core/hooks/useIsConnected";

interface ComponentInput {
    optionalData: boolean,
}

export function WeightHashComponent( { optionalData }: ComponentInput ) {
    const [address, setAddress] = useState("")
    const [weight, setWeight] = useState(0)
    const [nonce, setNonce] = useState(0)
    const [data, setData] = useState("")
    
    const { contract, isLoading, isError } = useContract()
    const [isConnected] = useIsConnected();

    async function getHash() {
        let { address: userAddress, isError: error } = validateAddress(address);
        if (error) return;

        let validatedData: string | undefined;
        
        if (optionalData) {
            validatedData = undefined;
        } else {
            const { validatedData: optData, isError } = validateOptionalData(data);
            if (isError) return;
            validatedData = optData;
        }

        let user: UserInput = {
            address: userAddress,
            weight: weight
        }

        try {
            const { value } = await contract!.functions.weight_hash(validatedData, nonce, user).get();
            toast.success(`Hash: ${value}`, { duration: 10000 });
        } catch (err) {
            toast.error("Ah! Math is hard rn, sorry", { duration: 10000 });
        }
    }

    return (
        <>
            <Stack>
                <Heading as="h4" css={{ marginLeft: "auto", marginRight: "auto", color: "$accent1" }}>
                    Hash for user weight
                </Heading>

                <InputFieldComponent onChange={setAddress} text="Recipient address" placeholder="0x80d5e8c2be..." />
                <InputNumberComponent onChange={setWeight} text="New weight" placeholder="2" />
                <InputNumberComponent onChange={setNonce} text="Nonce" placeholder="3" />

                {optionalData && <InputFieldComponent onChange={setData} text="Optional data" placeholder="0x252afeeb6e..." />}

                <Button
                    color="accent"
                    onPress={getHash}
                    size="lg"
                    variant="solid"
                    isDisabled={!isConnected}
                    css={{ marginTop: "$1", boxShadow: "0px 0px 1px 1px" }}
                >
                    Create hash
                </Button>
            </Stack>
        </>
    );

}
