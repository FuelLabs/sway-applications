import { BoxCentered, Button, Flex, Heading, Stack, toast } from "@fuel-ui/react";
import { useState } from "react";
import { useContract } from "../../core/hooks";
import { SignatureComponent } from "../../common/components/signature";
import { InputFieldComponent } from "../../common/components/input_field";
import { InputNumberComponent } from "../../common/components/input_number";
import { ContractIdInput, IdentityInput } from "../../../contracts/MultisigContractAbi";
import { OptionalCheckBoxComponent } from "../../common/components/optional_data_checkbox";
import { RadioGroupComponent } from "../../common/components/radio_group";
import { validateData } from "../../common/utils/validate_data";
import { validateAddress } from "../../common/utils/validate_address";
import { validateContractId } from "../../common/utils/validate_contract_id";

export function TransferPage() {
    // Used for our component listeners
    const [address, setAddress] = useState("")
    const [asset, setAsset] = useState("")
    const [assetAmount, setAssetAmount] = useState(0)
    const [data, setData] = useState("")

    const [recipient, setRecipient] = useState("address")
    const [optionalData, setOptionalData] = useState(false)
    const [signatures, setSignatures] = useState([<SignatureComponent id={1} name="transfer" />])
    const { contract, isLoading, isError } = useContract()

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

        await contract!.functions.transfer(assetId, validatedData, [], identity, assetAmount).call();
        toast.success("Transfer complete!")
    }

    async function addSignature() {
        setSignatures([...signatures, <SignatureComponent id={signatures.length+1} name="transfer" /> ]);
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

                <InputFieldComponent onChange={setAddress} text="Recipient address" placeholder="0x80d5e8c2be..." name="transfer-recipient" />
                <InputFieldComponent onChange={setAsset} text="Asset id" placeholder="0x0000000000..." name="transfer-asset" />
                <InputNumberComponent onChange={setAssetAmount} text="Asset amount" placeholder="1.0" name="transfer-value" />

                {signatures.map((signatureComponent, index) => signatureComponent)}

                {optionalData && <InputFieldComponent onChange={setData} text="Optional data" placeholder="0x252afeeb6e..." name="transfer-data" />}

                <Button
                    color="accent"
                    onPress={transfer}
                    size="lg"
                    variant="solid"
                    css={{ marginTop: "$1", boxShadow: "0px 0px 1px 1px" }}
                >
                    Transfer
                </Button>

                <Flex gap="$2" css={{ marginTop: "$1" }}>
                    <Button
                        color="accent"
                        onPress={addSignature}
                        size="lg"
                        variant="solid"
                        css={{ width: "50%", boxShadow: "0px 0px 1px 1px" }}
                    >
                        Add signature
                    </Button>

                    <Button
                        color="accent"
                        onPress={removeSignature}
                        size="lg"
                        variant="solid"
                        css={{ width: "50%", boxShadow: "0px 0px 1px 1px" }}
                    >
                        Remove signature
                    </Button>
                </Flex>

                <OptionalCheckBoxComponent setOptionalData={setOptionalData} optionalData={optionalData} />
                <RadioGroupComponent setRecipient={setRecipient} />
            </Stack>
        </BoxCentered>
    );
}
