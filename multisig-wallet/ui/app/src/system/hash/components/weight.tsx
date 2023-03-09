import { Button, Heading, Stack, toast } from "@fuel-ui/react";
import { useState } from "react";
import { useContract } from "../../core/hooks";
import { UserInput } from "../../../contracts/MultisigContractAbi";
import { InputFieldComponent } from "../../common/components/input_field";
import { InputNumberComponent } from "../../common/components/input_number";
import { validateOptionalData } from "../../common/utils/validate_optional_data";
import { validateAddress } from "../../common/utils/validate_address";

interface ComponentInput {
    optionalData: boolean,
}

export function WeightHashComponent( { optionalData }: ComponentInput ) {
    // Used for our component listeners
    const [address, setAddress] = useState("")
    const [weight, setWeight] = useState(0)
    const [nonce, setNonce] = useState(0)
    const [data, setData] = useState("")
    
    const { contract, isLoading, isError } = useContract()

    async function getHash() {
        let { address: userAddress, isError: error } = validateAddress(address);
        if (error) return;

        const { validatedData, isError } = validateOptionalData(data);
        if (isError) return;

        let user: UserInput = {
            address: userAddress,
            weight: weight
        }

        const { value } = await contract!.functions.weight_hash(validatedData, nonce, user).get();
        toast.success(`Hash: ${value}`, { duration: 10000 });
    }

    return (
        <>
            <Stack>
                <Heading as="h4" css={{ marginLeft: "auto", marginRight: "auto", color: "$accent1" }}>
                    Hash for user weight
                </Heading>

                <InputFieldComponent onChange={setAddress} text="Recipient address" placeholder="0x80d5e8c2be..." name="weight-hash-address" />
                <InputNumberComponent onChange={setWeight} text="New weight" placeholder="2" name="weight-hash" />
                <InputNumberComponent onChange={setNonce} text="Nonce" placeholder="3" name="weight-hash-nonce" />

                {optionalData && <InputFieldComponent onChange={setData} text="Optional data" placeholder="0x252afeeb6e..." name="weight-hash-data" />}

                <Button
                    color="accent"
                    onPress={getHash}
                    size="lg"
                    variant="solid"
                    css={{ marginTop: "$1", boxShadow: "0px 0px 1px 1px" }}
                >
                    Create hash
                </Button>
            </Stack>
        </>
    );

}
