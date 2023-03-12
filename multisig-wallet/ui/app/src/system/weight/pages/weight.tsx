import { BoxCentered, Button, Heading, Stack, toast } from "@fuel-ui/react";
import { useState } from "react";
import { useContract, useIsConnected,  } from "../../core/hooks";
import { InputFieldComponent, InputNumberComponent, SignatureButtonComponent, SignatureComponent } from "../../common/components";
import { validateAddress, validateData } from "../../common/utils";
import { SignatureInfoInput, UserInput } from "../../../contracts/MultisigContractAbi";

export function WeightPage() {
    const [address, setAddress] = useState("")
    const [weight, setWeight] = useState(0)
    const [data, setData] = useState("")
    const [signatures, setSignatures] = useState<SignatureInfoInput[]>([{ 
        message_format: { None: [] }, 
        message_prefix: { None: [] }, 
        signature: { bytes: ["", ""] }, 
        wallet_type: { Fuel: [] }
    }])

    const { contract, isLoading, isError } = useContract()
    const [isConnected] = useIsConnected();

    async function executeWeight() {
        let { address: userAddress, isError } = validateAddress(address);
        if (isError) return;

        const { data: validatedData, isError: err } = validateData(data);
        if (err) return;

        let user: UserInput = {
            address: userAddress,
            weight: weight
        }

        // TODO: Figure out how to convert the signed message into a B512 in the SignatureInfo
        //       This is here to catch the error in the console rather than hide it via toast atm
        //       Once stable go back to toast
        await contract!.functions.set_weight(validatedData, signatures, user).call();
        toast.success("Updated user weight!", { duration: 10000 })

        // try {
        //     await contract!.functions.set_weight(validatedData, signatures, user).call();
        //     toast.success("Updated user weight!", { duration: 10000 })
        // } catch (err) {
        //     toast.error("I tried but today is just not your day...", { duration: 10000 });
        // }
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
                <InputFieldComponent onChange={setData} text="Data to sign" placeholder="0x252afeeb6e..." />

                {
                    signatures.map((signature, index) => {
                        return <SignatureComponent handler={updateSignature} index={index} />;
                    })
                }

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
            </Stack>
        </BoxCentered>
    );
}
