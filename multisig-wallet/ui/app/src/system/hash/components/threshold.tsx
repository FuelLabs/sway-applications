import { Button, Heading, Stack, toast } from "@fuel-ui/react";
import { useState } from "react";
import { useContract } from "../../core/hooks";
import { InputFieldComponent } from "../../common/components/input_field";
import { InputNumberComponent } from "../../common/components/input_number";
import { validateOptionalData } from "../../common/utils/validate_optional_data";

interface ComponentInput {
    optionalData: boolean,
}

export function ThresholdHashComponent( { optionalData }: ComponentInput ) {
    // Used for our component listeners
    const [threshold, setThreshold] = useState(0)
    const [nonce, setNonce] = useState(0)
    const [data, setData] = useState("")
    
    const { contract, isLoading, isError } = useContract()

    async function getThresholdHash() {
        const { validatedData, isError } = validateOptionalData(data);
         
        if (isError) {
            return;
        }

        const { value } = await contract!.functions.threshold_hash(validatedData, nonce, threshold).get();
        toast.success(`Hash: ${value}`, { duration: 10000 });
    }

    return (
        <>
            <Stack>
                <Heading as="h4" css={{ marginLeft: "auto", marginRight: "auto", color: "$accent1" }}>
                    Hash for threshold 
                </Heading>

                <InputNumberComponent onChange={setThreshold} text="Threshold" placeholder="8" name="threshold-hash" />
                <InputNumberComponent onChange={setNonce} text="Nonce" placeholder="3" name="threshold-hash-nonce" />

                {optionalData && <InputFieldComponent onChange={setData} text="Optional data" placeholder="0x252afeeb6e..." name="threshold-hash-data" />}

                <Button
                    color="accent"
                    onPress={getThresholdHash}
                    size="lg"
                    variant="solid"
                    css={{ marginTop: "$1" }}
                >
                    Create hash
                </Button>
            </Stack>
        </>
    );

}
