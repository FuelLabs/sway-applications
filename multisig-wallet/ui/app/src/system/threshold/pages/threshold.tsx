import { BoxCentered, Button, Heading, toast, Stack } from "@fuel-ui/react";
import { useState } from "react";
import { useContract, useIsConnected } from "../../core/hooks";
import { InputFieldComponent, InputNumberComponent, SignatureButtonComponent, SignatureComponent } from "../../common/components";
import { validateData } from "../../common/utils/validate_data";
import { SignatureInfoInput } from "../../../contracts/MultisigContractAbi";

export function ThresholdPage() {
    const [threshold, setThreshold] = useState(0)
    const [data, setData] = useState("")
    const [signatures, setSignatures] = useState<SignatureInfoInput[]>([{ 
        message_format: { None: [] }, 
        message_prefix: { None: [] }, 
        signature: { bytes: ["", ""] }, 
        wallet_type: { Fuel: [] }
    }])

    const { contract, isLoading, isError } = useContract()
    const [isConnected] = useIsConnected();

    async function executeThreshold() {
        const { data: validatedData, isError } = validateData(data);
        if (isError) return;

        await contract!.functions.set_threshold(validatedData, signatures, threshold).call().then(
            (success) => {
                toast.success("Updated threshold!", { duration: 10000 });
            },
            (error) => {
                if (error.logs.length === 0) {
                    toast.error("Unknown error occurred during contract call.", { duration: 10000 });
                } else {
                    toast.error(`Error: ${Object.keys(error.logs[0])[0]}`, { duration: 10000 });
                }
            }
        );
    }     

    async function updateSignature(index: number, signature: string) {
        const localSignatures = [...signatures];
        // TODO: Figure out how to convert the signed message into a B512 in the SignatureInfo
        localSignatures[index].signature.bytes = [signature, signature];
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
                <InputFieldComponent onChange={setData} text="Data to sign" placeholder="0x252afeeb6e..." />
                
                {
                    signatures.map((signature, index) => {
                        return <SignatureComponent handler={updateSignature} index={index} />;
                    })
                }

                <Button
                    color="accent"
                    onPress={executeThreshold}
                    size="lg"
                    variant="solid"
                    isDisabled={!isConnected}
                    css={{ marginTop: "$2", boxShadow: "0px 0px 3px 1px", fontWeight: "$semibold" }}
                >
                    Set threshold
                </Button>

                <SignatureButtonComponent addHandler={addSignature} removeHandler={removeSignature}/>
            </Stack>
        </BoxCentered>
    );
}
