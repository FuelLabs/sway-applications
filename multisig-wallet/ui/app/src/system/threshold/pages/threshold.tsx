import { BoxCentered, Button, Checkbox, Flex, Form, Heading, Input, Text, toast, Stack } from "@fuel-ui/react";
import { useState } from "react";
import { useContract } from "../../core/hooks";
import { SignatureComponent } from "../../common/signature";
import { InputFieldComponent } from "../../common/input_field";
import { InputNumberComponent } from "../../common/input_number";

export function ThresholdPage() {
    const [optionalData, setOptionalData] = useState(false)
    const [signatures, setSignatures] = useState([<SignatureComponent id={1} name="transfer" />])
    const { contract, isLoading, isError } = useContract()

    async function useThreshold() {
        const data = document.querySelector<HTMLInputElement>(
            `[name="threshold-data"]`
        )!.value;

        const signatures = document.querySelector<HTMLInputElement>(
            `[name="threshold-signature"]`
        )!.value;

        const threshold = document.querySelector<HTMLInputElement>(
            `[name="threshold"]`
        )!.value;

        toast.error("Unimplemented")
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

                <InputNumberComponent text="Threshold" placeholder="8" name="threshold" />

                {signatures.map((signatureComponent, index) => signatureComponent)}

                {optionalData && <InputFieldComponent text="Optional data" placeholder="0x252afeeb6e..." name="trathresholdnsfer-data" />}

                <Button
                    color="accent"
                    onPress={useThreshold}
                    size="lg"
                    variant="solid"
                    css={{ marginTop: "$1" }}
                >
                    Set threshold
                </Button>

                <Flex gap="$1" css={{ marginTop: "$1" }}>
                    <Button
                        color="accent"
                        onPress={addSignature}
                        size="lg"
                        variant="solid"
                        css={{ width: "50%" }}
                    >
                        Add signature
                    </Button>

                    <Button
                        color="accent"
                        onPress={removeSignature}
                        size="lg"
                        variant="solid"
                        css={{ width: "50%" }}
                    >
                        Remove signature
                    </Button>
                </Flex>

                <BoxCentered css={{ marginTop: "$8" }}>
                    <Form.Control css={{ flexDirection: 'row' }}>
                        <Checkbox onClick={() => setOptionalData(!optionalData)} id="optional-data"/>
                        <Form.Label htmlFor="optional-data">
                            Optional data
                        </Form.Label>
                    </Form.Control>
                </BoxCentered>

            </Stack>
            
        </BoxCentered>
    );
}
