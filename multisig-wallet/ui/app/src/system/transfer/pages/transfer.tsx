import { BoxCentered, Button, Heading, Stack, toast } from "@fuel-ui/react";
import { useState } from "react";
import { useContract } from "../../core/hooks";
import { SignatureComponent } from "../../common/components/signature";
import { InputFieldComponent } from "../../common/components/input_field";
import { InputNumberComponent } from "../../common/components/input_number";
import { ContractIdInput, IdentityInput } from "../../../contracts/MultisigContractAbi";
import { RadioGroupComponent } from "../../common/components/radio_group";
import { validateData } from "../../common/utils/validate_data";
import { validateAddress } from "../../common/utils/validate_address";
import { validateContractId } from "../../common/utils/validate_contract_id";
import { SignatureButtonComponent } from "../../common/components/signature_buttons";
import { SignatureInfoInput } from "../../../contracts/MultisigContractAbi";
import { useIsConnected } from "../../core/hooks/useIsConnected";

export function TransferPage() {
    const [address, setAddress] = useState("")
    const [asset, setAsset] = useState("")
    const [assetAmount, setAssetAmount] = useState(0)
    const [signatures, setSignatures] = useState<SignatureInfoInput[]>([{ 
        message_format: { None: [] }, 
        message_prefix: { None: [] }, 
        signature: { bytes: ["", ""] }, 
        wallet_type: { Fuel: [] }
    }])
    const [data, setData] = useState("")

    const [recipient, setRecipient] = useState("address")
    const { contract, isLoading, isError } = useContract()
    const [isConnected] = useIsConnected();

    async function transfer() {
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

        let { address: validatedAsset, isError: error} = validateContractId(asset);
        if (error) return;

        const { data: validatedData, isError } = validateData(data);
        if (isError) return;

        let assetId: ContractIdInput = { value: validatedAsset };

        try {
            await contract!.functions.transfer(assetId, validatedData, signatures, identity, assetAmount).call();
            toast.success("Transfer complete!", { duration: 10000 });
        } catch (err) {
            toast.error("Captain, that didn't go as planned", { duration: 10000 });
        }
    }

    async function updateSignature(index: number, signature: string) {
        const localSignatures = [...signatures];
        localSignatures[index].signature.bytes = [signature, ""];
        setSignatures(localSignatures);
    }

    async function addSignature() {
        let signature: SignatureInfoInput = { 
            message_format: { None: [] }, 
            message_prefix: { None: [] }, 
            signature: { bytes: ["", ""] }, 
            wallet_type: { Fuel: [] }
        };
        setSignatures([...signatures, signature ]);
    }

    async function removeSignature() {
        if (signatures.length === 1) {
            toast.error("Cannot remove the last signature")
            return;
        }

        setSignatures([...signatures.splice(0, signatures.length - 1)]);
    }

    return (
        <BoxCentered css={{ marginTop: "12%", width: "30%" }}>
            <Stack css={{ minWidth: "100%" }}>

                <Heading as="h3" css={{ marginLeft: "auto", marginRight: "auto", marginBottom: "$10", color: "$accent1"}}>
                    Execute a transfer
                </Heading>

                <InputFieldComponent onChange={setAddress} text="Recipient address" placeholder="0x80d5e8c2be..." />
                <InputFieldComponent onChange={setAsset} text="Asset id" placeholder="0x0000000000..." />
                <InputNumberComponent onChange={setAssetAmount} text="Asset amount" placeholder="1.0" />

                {
                    signatures.map((signature, index) => {
                        return <SignatureComponent handler={updateSignature} index={index} />;
                    })
                }

                <InputFieldComponent onChange={setData} text="Data" placeholder="0x252afeeb6e..." />

                <Button
                    color="accent"
                    onPress={transfer}
                    size="lg"
                    variant="solid"
                    isDisabled={!isConnected}
                    css={{ marginTop: "$1", boxShadow: "0px 0px 1px 1px" }}
                >
                    Transfer
                </Button>

                <SignatureButtonComponent addHandler={addSignature} removeHandler={removeSignature}/>
                <RadioGroupComponent handler={setRecipient} />
            </Stack>
        </BoxCentered>
    );
}
