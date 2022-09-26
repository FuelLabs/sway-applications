import { BigNumberish } from "fuels";
import { useQuery } from "react-query";
import { useContract } from "./useContract";

export function useEscrows(queryString: string, escrowIds: BigNumberish[] | null | undefined) {
    const contract = useContract();

    // We have to convert the bigints to strings bc bigints are not serializable
    const { data: escrows } = useQuery(
        [queryString, escrowIds?.map(escrowId => { return escrowId.toString() })],
        async () => {
            const escrowPromises = escrowIds!.map(escrowId => {
                return contract!.functions.escrows(escrowId).get()
            });
            return await Promise.all(escrowPromises);
        },
        {
            enabled: !!escrowIds,
            onError: (err) => console.log(`use error: ${err}`),
        }
    );

    return escrows?.map(escrow => { return escrow.value });
}