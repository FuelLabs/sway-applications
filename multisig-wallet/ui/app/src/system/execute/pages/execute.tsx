import { PleaseConnect } from "../../core/components";
import { useEffect, useState } from "react"
import { Button, toast } from "@fuel-ui/react";

export function ExecutePage() {
    // const [connected, setPage] = useState(<PleaseConnect />)

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

    return (
        <>
            <Button
                color="accent"
                onPress={function noRefCheck(){ toast.error("Unimplemented") }}
                size="md"
                variant="solid"
                css={{ margin: "auto" }}
            >
                Execute
            </Button>
        </>
    );
}
