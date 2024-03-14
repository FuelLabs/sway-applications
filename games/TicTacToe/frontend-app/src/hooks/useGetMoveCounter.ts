import { useWallet } from "@fuels/react"
import { useQuery } from "@tanstack/react-query";

import { TicTacToeQueryKeys } from "../queryKeys";
import { TictactoeContractAbi__factory } from "../contract-types";
import { CONTRACT_ID } from "../config";

export const useGetMoveCounter = () => {
    const { wallet, isError, isLoading } = useWallet();

    const query = useQuery({
        queryKey: [TicTacToeQueryKeys.moveCounter, wallet?.provider.url],
        queryFn: async () => {
            if (!wallet) throw new Error(`Cannot get move counter if wallet is ${wallet}`);

            const contract = TictactoeContractAbi__factory.connect(
                CONTRACT_ID,
                wallet
            );

            const result = await contract.functions.get_move_counter().simulate();
            return result.value ?? null;
        },
        enabled: !!wallet && !isError && !isLoading
    });

    return { ...useQuery, moveCounter: query.data };
}
