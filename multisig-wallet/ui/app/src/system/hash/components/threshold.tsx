import { Button, Heading, Stack, toast } from "@fuel-ui/react";
import { useState } from "react";
import { useContract, useIsConnected } from "../../core/hooks";
import { InputFieldComponent, InputNumberComponent } from "../../common/components";
import { validateData } from "../../common/utils";

export function ThresholdHashComponent() {
    const [threshold, setThreshold] = useState(0)
    const [nonce, setNonce] = useState(0)
    const [data, setData] = useState("")
    
    const { contract, isLoading, isError } = useContract()
    const [isConnected] = useIsConnected();

    async function getHash() {
        const { data: validatedData, isError } = validateData(data);
        if (isError) return;
        
        try {
            const { value } = await contract!.functions.threshold_hash(validatedData, nonce, threshold).get();
            toast.success(`Hash: ${value}`, { duration: 10000 });
        } catch (err) {
            toast.error("Ah! Math is hard rn, sorry", { duration: 10000 });
        }
    }

    return (
        <>
            <Stack>
                <Heading as="h4" css={{ marginLeft: "auto", marginRight: "auto", color: "$accent1" }}>
                    Hash for threshold 
                </Heading>

                <InputNumberComponent onChange={setThreshold} text="Threshold" placeholder="8" />
                <InputNumberComponent onChange={setNonce} text="Nonce" placeholder="3" />
                <InputFieldComponent onChange={setData} text="Data to sign" placeholder="0x252afeeb6e..." />

                <Button
                    color="accent"
                    onPress={getHash}
                    size="lg"
                    variant="solid"
                    isDisabled={!isConnected}
                    css={{ marginTop: "$1", boxShadow: "0px 0px 1px 1px" }}
                >
                    Create hash
                </Button>
            </Stack>
        </>
    );

}
