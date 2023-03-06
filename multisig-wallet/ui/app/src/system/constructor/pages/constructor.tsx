import { useEffect, useState } from "react";
import { bn } from "fuels";
import { useContract } from "../../core/hooks";
import { CreateWallet, CreatedWallet } from "../components";

export function ConstructorPage() {
    const [initialized, setInit] = useState(false)
    const { contract, isLoading, isError } = useContract()

    // useEffect(() => {
    //     async function main() {
    //         if (isLoading) {
    //             console.log("loading");
    //         }
    //         // const nonce = await (await contract!.functions.nonce().get()).value;
    //         const nonce = await contract!.functions.nonce().get();
    //         console.log("lalala")
    //         console.log(nonce);
    //         // console.log(nonce !== bn(0));
    //         console.log("aaaaaaa")
    //         // setInit(nonce !== bn(0));
    //     }
    //     main();
    // }, [initialized]);

    // if (isLoading) {
    //     console.log("loadingsdfsdf");
    // }

    return (
        <>
            {initialized ? <CreatedWallet /> : <CreateWallet />}
        </>
    );
}
