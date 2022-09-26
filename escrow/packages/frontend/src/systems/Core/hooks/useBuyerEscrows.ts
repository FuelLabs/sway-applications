import { useQuery } from "react-query";

import { useWallet } from "../context/AppContext";
import { useContract } from "./useContract";
import { useEscrows } from "./useEscrows";

export function useBuyerEscrows() {
    const contract = useContract();
    const wallet = useWallet();

    const { data: buyerEscrowIds } = useQuery(
        ['BuyerPage-buyerEscrowIds', wallet?.address.toHexString()!],
        async () => {
            return contract && (await contract!.functions.buyer_escrows({ Address: { value: wallet?.address!.toHexString()! } }).get()).value
        },
        {
            onSuccess: (data) => console.log("data 1: ", data),
            onError: (err) => console.log(`Buyer error: ${err}`),
        }
    );

    const buyerEscrows = useEscrows("BuyerEscrows", buyerEscrowIds);

    return buyerEscrows;
}
