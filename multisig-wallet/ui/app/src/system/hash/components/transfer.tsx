import { Button, Heading, Stack, toast } from "@fuel-ui/react";
import { useState } from "react";
import { useContract } from "../../core/hooks";
import { IdentityInput } from "../../../contracts/MultisigContractAbi";
import { InputFieldComponent } from "../../common/input_field";
import { InputNumberComponent } from "../../common/input_number";

interface ComponentInput {
    optionalData: boolean,
    recipient: string
}

export function TransferHashComponent( { optionalData, recipient }: ComponentInput ) {
    // Used for our component listeners
    const [address, setAddress] = useState("")
    const [asset, setAsset] = useState("")
    const [assetAmount, setAssetAmount] = useState(0)
    const [nonce, setNonce] = useState(0)
    const [data, setData] = useState("")
    
    const { contract, isLoading, isError } = useContract()

    async function getHash() {
        let identity: IdentityInput;

        if (recipient === "address") {
            identity = { Address: { value: address } };
        } else {
            identity = { ContractId: { value: address } };
        }

        // TODO: merge in new hashing function and use instead of this incorrect one
        const { value } = await contract!.functions.transaction_hash(data, nonce, identity, assetAmount).get();
        toast.success(`Hash: ${value}`, { duration: 10000 });
    }

    return (
        <>
            <Stack>
                <Heading as="h4" css={{ marginLeft: "auto", marginRight: "auto", color: "$accent1"}}>
                    Hash for transfer
                </Heading>

                <InputFieldComponent onChange={setAddress} text="Recipient address" placeholder="0x80d5e8c2be..." name="transfer-hash-address" />
                <InputFieldComponent onChange={setAsset} text="Asset id" placeholder="0x0000000000..." name="transfer-hash-asset" />
                <InputNumberComponent onChange={setAssetAmount} text="Asset amount" placeholder="1.0" name="transfer-hash-value" />
                <InputNumberComponent onChange={setNonce} text="Nonce" placeholder="3" name="transfer-hash-nonce" />

                {optionalData && <InputFieldComponent onChange={setData} text="Optional data" placeholder="0x252afeeb6e..." name="transfer-hash-data" />}

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
