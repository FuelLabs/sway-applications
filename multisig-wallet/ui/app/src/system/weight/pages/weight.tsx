import { BoxCentered, Button, Checkbox, Flex, Form, Heading, Input, RadioGroup, Text, Stack, toast } from "@fuel-ui/react";
import { useState } from "react";
import { useContract } from "../../core/hooks";
import { SignatureComponent } from "../../common/signature";
import { InputFieldComponent } from "../../common/input_field";
import { InputNumberComponent } from "../../common/input_number";

export function WeightPage() {
    const [radio, setRadio] = useState("address")
    const [optionalData, setOptionalData] = useState(false)
    const [signatures, setSignatures] = useState([<SignatureComponent id={1} name="transfer" />])
    const { contract, isLoading, isError } = useContract()

    async function useWeight() {
        const data = document.querySelector<HTMLInputElement>(
            `[name="weight-data"]`
        )!.value;

        const signatures = document.querySelector<HTMLInputElement>(
            `[name="weight-signatures"]`
        )!.value;

        const userAddress = document.querySelector<HTMLInputElement>(
            `[name="weight"]`
        )!.value;

        const userWeight = document.querySelector<HTMLInputElement>(
            `[name="weight"]`
        )!.value;

        toast.error("Unimplemented")
    }

    async function addSignature() {
        setSignatures([...signatures, <SignatureComponent id={signatures.length+1} name="weight" /> ]);
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

                <InputFieldComponent text="Recipient address" placeholder="0x80d5e8c2be..." name="weight-address" />
                <InputNumberComponent text="New weight" placeholder="2" name="transaction-value" />

                {signatures.map((signatureComponent, index) => signatureComponent)}

                {optionalData && <InputFieldComponent text="Optional data" placeholder="0x252afeeb6e..." name="weight-data" />}

                <Button
                    color="accent"
                    onPress={useWeight}
                    size="lg"
                    variant="solid"
                    css={{ marginTop: "$1" }}
                >
                    Set weight
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
