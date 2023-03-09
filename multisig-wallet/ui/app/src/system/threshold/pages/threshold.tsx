import { BoxCentered, Button, Flex, Heading, toast, Stack } from "@fuel-ui/react";
import { useState } from "react";
import { useContract } from "../../core/hooks";
import { SignatureComponent } from "../../common/components/signature";
import { InputFieldComponent } from "../../common/components/input_field";
import { InputNumberComponent } from "../../common/components/input_number";
import { OptionalCheckBoxComponent } from "../../common/components/optional_data_checkbox";
import { validateOptionalData } from "../../common/utils/validate_optional_data";

export function ThresholdPage() {
    // Used for our component listeners
    const [data, setData] = useState("")
    const [threshold, setThreshold] = useState(0)

    const [optionalData, setOptionalData] = useState(false)
    const [signatures, setSignatures] = useState([<SignatureComponent id={1} name="transfer" />])
    const { contract, isLoading, isError } = useContract()

    async function useThreshold() {
        // const signatures = document.querySelector<HTMLInputElement>(
        //     `[name="threshold-signature"]`
        // )!.value;

        const { validatedData, isError } = validateOptionalData(data);
         
        if (isError) {
            return;
        }

        await contract!.functions.set_threshold(validatedData, [], threshold).call();
        toast.success("Updated threshold!")
    }

    async function addSignature() {
        setSignatures([...signatures, <SignatureComponent id={signatures.length+1} name="threshold" /> ]);
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

                <InputNumberComponent onChange={setThreshold} text="Threshold" placeholder="8" name="threshold" />

                {signatures.map((signatureComponent, index) => signatureComponent)}

                {optionalData && <InputFieldComponent onChange={setData} text="Optional data" placeholder="0x252afeeb6e..." name="trathresholdnsfer-data" />}

                <Button
                    color="accent"
                    onPress={useThreshold}
                    size="lg"
                    variant="solid"
                    css={{ marginTop: "$1", boxShadow: "0px 0px 1px 1px" }}
                >
                    Set threshold
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
            </Stack>
        </BoxCentered>
    );
}
