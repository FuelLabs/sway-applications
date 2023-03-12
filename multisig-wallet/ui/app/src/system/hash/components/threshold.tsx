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

        const { value } = await contract!.functions.threshold_hash(validatedData, nonce, threshold).get().then(
            null,
            (error) => {
                if (error.logs === undefined || error.logs.length === 0) {
                    toast.error("Unknown error occurred during contract call.", { duration: 10000 });
                } else {
                    toast.error(`Error: ${Object.keys(error.logs[0])[0]}`, { duration: 10000 });
                }
                return;
            }
        );

        toast.success(`Hash: ${value}`, { duration: 10000 });
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
                    css={{ marginTop: "$2", boxShadow: "0px 0px 3px 1px", fontWeight: "$semibold" }}
                >
                    Create hash
                </Button>
            </Stack>
        </>
    );

}
