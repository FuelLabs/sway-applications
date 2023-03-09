import { Button, Heading, Stack, toast } from "@fuel-ui/react";
import { useState } from "react";
import { useContract } from "../../core/hooks";
import { IdentityInput } from "../../../contracts/MultisigContractAbi";
import { InputFieldComponent } from "../../common/components/input_field";
import { InputNumberComponent } from "../../common/components/input_number";
import { validateData } from "../../common/utils/validate_data";
import { validateAddress } from "../../common/utils/validate_address";
import { validateContractId } from "../../common/utils/validate_contract_id";

interface ComponentInput {
    optionalData: boolean,
    recipient: string
}

export function ExecuteHashComponent( { optionalData, recipient }: ComponentInput ) {
    // Used for our component listeners
    const [address, setAddress] = useState("")
    const [assetAmount, setAssetAmount] = useState(0)
    const [nonce, setNonce] = useState(0)
    const [data, setData] = useState("")
    
    const { contract, isLoading, isError } = useContract()

    async function getHash() {
        let identity: IdentityInput;

        if (recipient === "address") {
            let { address: user, isError } = validateAddress(address);
            if (isError) return;

            identity = { Address: { value: user } };
        } else {
            let { address: user, isError } = validateContractId(address);
            if (isError) return;

            identity = { ContractId: { value: user } };
        }

        const { data: validatedData, isError } = validateData(data);
        if (isError) return;

        const { value } = await contract!.functions.transaction_hash(validatedData, nonce, identity, assetAmount).get();
        toast.success(`Hash: ${value}`, { duration: 10000 });
    }

    return (
        <>
            <Stack>
                <Heading as="h4" css={{ marginLeft: "auto", marginRight: "auto", color: "$accent1" }}>
                    Hash for execution
                </Heading>

                <InputFieldComponent onChange={setAddress} text="Recipient address" placeholder="0x80d5e8c2be..." name="execute-hash-address" />
                <InputNumberComponent onChange={setAssetAmount} text="Asset amount" placeholder="1.0" name="execute-hash-value" />
                <InputNumberComponent onChange={setNonce} text="Nonce" placeholder="3" name="execute-hash-nonce" />

                {optionalData && <InputFieldComponent onChange={setData} text="Optional data" placeholder="0x252afeeb6e..." name="execute-hash-data" />}

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
