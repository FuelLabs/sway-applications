import { useQuery } from "react-query";
import { useWallet } from "../context/AppContext";
import { useContract } from "./useContract";
import { useEscrows } from "./useEscrows";

export function useArbiterEscrows() {
    const contract = useContract();
    const wallet = useWallet();

    const { data: arbiterEscrowIds } = useQuery(
        ["ArbiterPage-arbiterEscrowIds"],
        async () => {
            return contract && (await contract.functions.arbiter_escrows({
                Address: {
                    value: wallet?.address.toHexString()!
                }
            }).call()).value
        }
    );

    const arbiterEscrows = useEscrows("ArbiterEscrows", arbiterEscrowIds);

    return arbiterEscrows;
}