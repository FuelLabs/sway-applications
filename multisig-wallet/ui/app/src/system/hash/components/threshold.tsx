import { Button, Heading, Stack, toast } from "@fuel-ui/react";
import { useState } from "react";
import { useContract } from "../../core/hooks";
import { InputFieldComponent } from "../../common/components/input_field";
import { InputNumberComponent } from "../../common/components/input_number";
import { validateOptionalData } from "../../common/utils/validate_optional_data";
import { useIsConnected } from "../../core/hooks/useIsConnected";

interface ComponentInput {
    optionalData: boolean,
}

export function ThresholdHashComponent( { optionalData }: ComponentInput ) {
    const [threshold, setThreshold] = useState(0)
    const [nonce, setNonce] = useState(0)
    const [data, setData] = useState("")
    
    const { contract, isLoading, isError } = useContract()
    const [isConnected] = useIsConnected();

    async function getHash() {
        let validatedData: string | undefined;
        
        if (optionalData) {
            validatedData = undefined;
        } else {
            const { validatedData: optData, isError } = validateOptionalData(data);
            if (isError) return;
            validatedData = optData;
        }
        
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

                {optionalData && <InputFieldComponent onChange={setData} text="Optional data" placeholder="0x252afeeb6e..." />}

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
