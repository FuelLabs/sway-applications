import { useQuery } from "react-query";

import { useWallet } from "../context/AppContext";
import { useContract } from "./useContract";

export function useBuyerEscrows() {
    const contract = useContract();
    const wallet = useWallet();
    console.log("contract: ", contract);
    console.log("addr: ", wallet?.address.toHexString())
    const { data: buyerEscrowIds } = useQuery(
        ['BuyerPage-buyerEscrowIds'],
        async () => {
            return contract && (await contract!.functions.buyer_escrows({ Address: { value: wallet?.address!.toHexString()! } }).call()).value
        },
        {
            onSuccess: (data) => console.log("data 1: ", data),
            onError: (err) => console.log(JSON.stringify(err, null, 2)),
        }
    );
    console.log("buyer escrow ids: ", buyerEscrowIds);
    const { data: buyerEscrows } = useQuery(
        ["BuyerEscrows"],
        async () => {
            const escrowPromises = buyerEscrowIds!.map(async escrowId => {
                return (await contract!.functions.escrows(escrowId).call()).value
            });
            return await Promise.all(escrowPromises);
        },
        {
            enabled: !!buyerEscrowIds,
            onSuccess: (data) => console.log("data 2: ", data),
            onError: (err) => console.log(JSON.stringify(err, null, 2)),
        }
    );
    return buyerEscrows;
}
