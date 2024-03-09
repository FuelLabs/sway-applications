import { useQuery } from "@tanstack/react-query"
import { useWallet } from "@fuels/react";
import { TictactoeContractAbi__factory } from "../contract-types";
import { TicTacToeQueryKeys } from "../queryKeys";

export const useGetCurrentPlayer = () => {
    const { wallet, isError, isLoading } = useWallet();

    const query = useQuery({
        queryKey: [TicTacToeQueryKeys.currentPlayer],
        queryFn: async () => {
            if (!wallet) throw new Error(`Cannot get current player if wallet is ${wallet}`);

            const contract = TictactoeContractAbi__factory.connect(
                import.meta.env.VITE_CONTRACT_ID,
                wallet
            );
            const result = await contract.functions.get_current_player().simulate();
            return result.value ?? null;
        },
        enabled: !!wallet && !isError && !isLoading
    });

    return { ...query, currentPlayer: query.data };
}
