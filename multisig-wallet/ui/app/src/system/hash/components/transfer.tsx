import { Button, Heading, Stack, toast } from "@fuel-ui/react";
import { useState } from "react";
import { useContract, useIsConnected } from "../../core/hooks";
import { InputFieldComponent, InputNumberComponent } from "../../common/components";
import { validateAddress, validateContractId, validateData } from "../../common/utils";
import { IdentityInput } from "../../../contracts/MultisigContractAbi";

interface ComponentInput {
    recipient: string
}

export function TransferHashComponent( { recipient }: ComponentInput ) {
    const [address, setAddress] = useState("")
    const [asset, setAsset] = useState("")
    const [assetAmount, setAssetAmount] = useState(0)
    const [nonce, setNonce] = useState(0)
    const [data, setData] = useState("")
    
    const { contract, isLoading, isError } = useContract()
    const [isConnected] = useIsConnected();

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

        // TODO: merge in new hashing function and use instead of this incorrect one
        const { value } = await contract!.functions.transaction_hash(validatedData, nonce, identity, assetAmount).get().then(
            null,
            (error) => {
                if (error.logs === undefined || error.logs.length === 0) {
                    toast.error("Unknown error occurred during contract call.", { duration: 10000 });
                } else {
                    toast.error(`Error: ${Object.keys(error.logs[0])[0]}`, { duration: 10000 });
                }
                return;
            }
        );

        toast.success(`Hash: ${value}`, { duration: 10000 });      
    }

    return (
        <>
            <Stack>
                <Heading as="h4" css={{ marginLeft: "auto", marginRight: "auto", color: "$accent1"}}>
                    Hash for transfer
                </Heading>

                <InputFieldComponent onChange={setAddress} text="Recipient address" placeholder="0x80d5e8c2be..." />
                <InputFieldComponent onChange={setAsset} text="Asset id" placeholder="0x0000000000..." />
                <InputNumberComponent onChange={setAssetAmount} text="Asset amount" placeholder="1.0" />
                <InputNumberComponent onChange={setNonce} text="Nonce" placeholder="3" />
                <InputFieldComponent onChange={setData} text="Data to sign" placeholder="0x252afeeb6e..." />

                <Button
                    color="accent"
                    onPress={getHash}
                    size="lg"
                    variant="solid"
                    isDisabled={!isConnected}
                    css={{ marginTop: "$2", boxShadow: "0px 0px 3px 1px", fontWeight: "$semibold" }}
                >
                    Create hash
                </Button>
            </Stack>
        </>
    );

}
