import { BoxCentered, Button, Checkbox, Flex, Form, Heading, Input, RadioGroup, Text, toast, Stack } from "@fuel-ui/react";
import { useState } from "react";
import { useContract } from "../../core/hooks";
import { SignatureComponent } from "../../common/signature";
import { InputFieldComponent } from "../../common/input_field";
import { InputNumberComponent } from "../../common/input_number";

export function TransferPage() {
    const [radio, setRadio] = useState("address")
    const [optionalData, setOptionalData] = useState(false)
    const [signatures, setSignatures] = useState([<SignatureComponent id={1} name="transfer" />])
    const { contract, isLoading, isError } = useContract()

    async function useTransfer() {
        const recipient = document.querySelector<HTMLInputElement>(
            `[name="transfer-recipient"]`
        )!.value;

        const asset = document.querySelector<HTMLInputElement>(
            `[name="transfer-asset"]`
        )!.value;

        const value = document.querySelector<HTMLInputElement>(
            `[name="transfer-value"]`
        )!.value;

        // const data = document.querySelector<HTMLInputElement>(
        //     `[name="transfer-data"]`
        // )!.value;

        console.log(signatures);

        toast.error("Unimplemented")
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

                <InputFieldComponent text="Recipient address" placeholder="0x80d5e8c2be..." name="transfer-recipient" />
                <InputFieldComponent text="Asset id" placeholder="0x0000000000..." name="transfer-asset" />
                <InputNumberComponent text="Asset amount" placeholder="1.0" name="transfer-value" />

                {signatures.map((signatureComponent, index) => signatureComponent)}

                {optionalData && <InputFieldComponent text="Optional data" placeholder="0x252afeeb6e..." name="transfer-data" />}

                <Button
                    color="accent"
                    onPress={useTransfer}
                    size="lg"
                    variant="solid"
                    css={{ marginTop: "$1" }}
                >
                    Transfer
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

                <Heading as="h4" css={{ marginLeft: "auto", marginRight: "auto", marginTop: "$8", color: "$accent1"}}>
                    Recipient Type
                </Heading>

                <RadioGroup defaultValue="address" direction="row" css={{ margin: "auto" }}>
                    {/* 
                        TODO: 
                            change labels to be the color black
                            increase the size of the buttons and text 
                    */}
                    <RadioGroup.Item onClick={() => setRadio("address")} label="Address" value="address" />
                    <RadioGroup.Item onClick={() => setRadio("contract")} label="Contract" value="contract" />
                </RadioGroup>

            </Stack>
            
        </BoxCentered>
    );
}
