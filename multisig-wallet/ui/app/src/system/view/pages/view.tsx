// import { BoxCentered, Button, Card, Flex, FuelLogo, Heading, toast } from "@fuel-ui/react";
import { PleaseConnect } from "../../core/components";
import { useEffect, useState } from "react"
import { Button, Text, toast } from "@fuel-ui/react";
import { useContract } from "../../core/hooks";

export function ViewPage() {
    const [nonce, setNonce] = useState(70)
    const [threshold, setThreshold] = useState(0)
    const { contract, isLoading, isError } = useContract()

    // useEffect(() => {
    //     async function main() {
    //         const isConnected = await window.fuel.isConnected();
    //         if (!isConnected) {
    //             setPage(<PleaseConnect />);
    //         } else {
    //             setPage(<>lalalala</>);
    //         }
    //     }
    //     main();
    // }, [connected]);

    async function updateNonce() {
        const { value } = await contract.functions.nonce().get();
        setNonce(Number(value));
    }

    return (
        <>
            <Text>Nonce: {nonce}</Text>
            <Button
                color="accent"
                onPress={updateNonce}
                size="md"
                variant="solid"
                css={{ margin: "auto" }}
            >
                Update Nonce
            </Button>
        </>
    );
}
