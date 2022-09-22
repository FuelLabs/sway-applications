import { useQuery } from "react-query";

import { useWallet } from "../context/AppContext";
import { useContract } from "./useContract";
import { useEscrows } from "./useEscrows";

export function useBuyerEscrows() {
    const contract = useContract();
    const wallet = useWallet();

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

    const buyerEscrows = useEscrows("BuyerEscrows", buyerEscrowIds);

    return buyerEscrows;
}
