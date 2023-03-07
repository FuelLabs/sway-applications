import { BoxCentered, Button, Checkbox, Flex, Form, Heading, toast, Stack } from "@fuel-ui/react";
import { isB256 } from "fuels";
import { useState } from "react";
import { useContract } from "../../core/hooks";
import { SignatureComponent } from "../../common/signature";
import { InputFieldComponent } from "../../common/input_field";
import { InputNumberComponent } from "../../common/input_number";

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

        let userData: string | undefined = data;
        if (userData === "") {
            userData = undefined;
        } else if (!isB256(userData)) {
            toast.error("I don't know about that data format chief", { duration: 10000 });
            return;
        }

        await contract!.functions.set_threshold(userData, [], threshold).call();
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
