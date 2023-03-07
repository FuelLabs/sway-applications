import { Button, Heading, Stack, toast } from "@fuel-ui/react";
import { useState } from "react";
import { useContract } from "../../core/hooks";
import { UserInput } from "../../../contracts/MultisigContractAbi";
import { InputFieldComponent } from "../../common/input_field";
import { InputNumberComponent } from "../../common/input_number";

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
        let user: UserInput = {
            address: address,
            weight: weight
        }

        const { value } = await contract!.functions.weight_hash(data, nonce, user).get();
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
                    css={{ marginTop: "$1" }}
                >
                    Create hash
                </Button>
            </Stack>
        </>
    );

}
