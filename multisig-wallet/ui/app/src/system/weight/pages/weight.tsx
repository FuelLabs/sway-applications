import { BoxCentered, Button, Heading, Stack, toast } from "@fuel-ui/react";
import { useState } from "react";
import { useContract } from "../../core/hooks";
import { SignatureComponent } from "../../common/components/signature";
import { InputFieldComponent } from "../../common/components/input_field";
import { InputNumberComponent } from "../../common/components/input_number";
import { OptionalCheckBoxComponent } from "../../common/components/optional_data_checkbox";
import { UserInput } from "../../../contracts/MultisigContractAbi";
import { validateOptionalData } from "../../common/utils/validate_optional_data";
import { validateAddress } from "../../common/utils/validate_address";
import { SignatureButtonComponent } from "../../common/components/signature_buttons";
import { SignatureInfoInput } from "../../../contracts/MultisigContractAbi";
import { useIsConnected } from "../../core/hooks/useIsConnected";

export function WeightPage() {
    const [address, setAddress] = useState("")
    const [weight, setWeight] = useState(0)
    const [signatures, setSignatures] = useState<SignatureInfoInput[]>([{ 
        message_format: { None: [] }, 
        message_prefix: { None: [] }, 
        signature: { bytes: ["", ""] }, 
        wallet_type: { Fuel: [] }
    }])
    const [data, setData] = useState("")

    const [optionalData, setOptionalData] = useState(false)
    const { contract, isLoading, isError } = useContract()
    const [isConnected] = useIsConnected();

    async function executeWeight() {
        let { address: userAddress, isError } = validateAddress(address);
        if (isError) return;

        const { validatedData, isError: error } = validateOptionalData(data);
        if (error) return;

        let user: UserInput = {
            address: userAddress,
            weight: weight
        }

        try {
            await contract!.functions.set_weight(validatedData, signatures, user).call();
            toast.success("Updated user weight!", { duration: 10000 })
        } catch (err) {
            toast.error("I tried but today is just not your day...", { duration: 10000 });
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
                <Heading as="h3" css={{ marginLeft: "auto", marginRight: "auto", marginBottom: "$10", color: "$accent1" }}>
                    Change approval weight of user
                </Heading>

                <InputFieldComponent onChange={setAddress} text="Recipient address" placeholder="0x80d5e8c2be..." />
                <InputNumberComponent onChange={setWeight} text="New weight" placeholder="2" />

                {
                    signatures.map((signature, index) => {
                        return <SignatureComponent handler={updateSignature} index={index} />;
                    })
                }

                {optionalData && <InputFieldComponent onChange={setData} text="Optional data" placeholder="0x252afeeb6e..." />}

                <Button
                    color="accent"
                    onPress={executeWeight}
                    size="lg"
                    variant="solid"
                    isDisabled={!isConnected}
                    css={{ marginTop: "$1", boxShadow: "0px 0px 1px 1px" }}
                >
                    Set weight
                </Button>

                <SignatureButtonComponent addHandler={addSignature} removeHandler={removeSignature}/>
                <OptionalCheckBoxComponent handler={setOptionalData} optionalData={optionalData} />
            </Stack>
        </BoxCentered>
    );
}
