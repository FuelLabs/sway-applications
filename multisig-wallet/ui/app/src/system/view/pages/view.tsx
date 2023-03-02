import { BoxCentered, Button, Card, Flex, FuelLogo, Heading, toast } from "@fuel-ui/react";
import { PleaseConnect } from "../../core/components";
import { useEffect, useState } from "react"

export function ViewPage() {
    // const [nonce, setNonce] = useState(0)
    // const [threshold, setThreshold] = useState(0)

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
        // setNonce(nonce + 1);
    }

    return (
        <>
            <Card>
                <Card.Body>
                    {/* Nonce: {nonce} */}
                    ViewPage
                </Card.Body>
            </Card>
            <Button onPress={updateNonce} variant="solid" css={{ color: 'black', fontWeight: 'bolder' }}>
                Nonce
            </Button>
        </>
        // <>ViewPage</>
    );
}
