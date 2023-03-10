import { BoxCentered, Button, Heading, toast, Stack } from "@fuel-ui/react";
import { useState } from "react";
import { useContract } from "../../core/hooks";
import { SignatureComponent } from "../../common/components/signature";
import { InputFieldComponent } from "../../common/components/input_field";
import { InputNumberComponent } from "../../common/components/input_number";
import { OptionalCheckBoxComponent } from "../../common/components/optional_data_checkbox";
import { validateOptionalData } from "../../common/utils/validate_optional_data";
import { SignatureButtonComponent } from "../../common/components/signature_buttons";
import { SignatureInfoInput } from "../../../contracts/MultisigContractAbi";

export function ThresholdPage() {
    const [threshold, setThreshold] = useState(0)
    const [signatures, setSignatures] = useState<SignatureInfoInput[]>([{ 
        message_format: { None: [] }, 
        message_prefix: { None: [] }, 
        signature: { bytes: ["", ""] }, 
        wallet_type: { Fuel: [] }
    }])
    const [data, setData] = useState("")

    const [optionalData, setOptionalData] = useState(false)
    const { contract, isLoading, isError } = useContract()

    async function executeThreshold() {
        const { validatedData, isError } = validateOptionalData(data);
        if (isError) return;

        try {
            await contract!.functions.set_threshold(validatedData, signatures, threshold).call();
            toast.success("Updated threshold!", { duration: 10000 })
        } catch (err) {
            toast.error("I don't know about that transfer chief", { duration: 10000 });
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
                    Change threshold for execution
                </Heading>

                <InputNumberComponent onChange={setThreshold} text="Threshold" placeholder="8" />

                {
                    signatures.map((signature, index) => {
                        return <SignatureComponent handler={updateSignature} index={index+1} />;
                    })
                }

                {optionalData && <InputFieldComponent onChange={setData} text="Optional data" placeholder="0x252afeeb6e..." />}

                <Button
                    color="accent"
                    onPress={executeThreshold}
                    size="lg"
                    variant="solid"
                    css={{ marginTop: "$1", boxShadow: "0px 0px 1px 1px" }}
                >
                    Set threshold
                </Button>

                <SignatureButtonComponent addHandler={addSignature} removeHandler={removeSignature}/>
                <OptionalCheckBoxComponent handler={setOptionalData} optionalData={optionalData} />
            </Stack>
        </BoxCentered>
    );
}
