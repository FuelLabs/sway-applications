import { useQuery } from "react-query";
import { useContract } from "./useContract";

export function useEscrows(queryString: string, escrowIds: bigint[] | null | undefined) {
    const contract = useContract();

    const { data: escrows } = useQuery(
        [queryString],
        async () => {
            const escrowPromises = escrowIds!.map(async escrowId => {
                return (await contract!.functions.escrows(escrowId).call()).value
            });
            return await Promise.all(escrowPromises);
        },
        {
            enabled: !!escrowIds
        }
    );

    return escrows;
}