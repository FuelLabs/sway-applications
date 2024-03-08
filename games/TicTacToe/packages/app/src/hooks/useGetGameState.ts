import { useWallet } from "@fuels/react";
import { useQuery } from "@tanstack/react-query";

import { TictactoeContractAbi__factory } from "../contract-types";
import { TicTacToeQueryKeys } from "../queryKeys";

export const useGetGameState = () => {
    const { wallet } = useWallet();

    const query = useQuery({
        queryKey: [TicTacToeQueryKeys.gameState],
        queryFn: async () => {
            if (!wallet) throw new Error(`Cannot get game state if the walelt is ${wallet}`);

            const contract = TictactoeContractAbi__factory.connect(
                import.meta.env.VITE_CONTRACT_ID,
                wallet
            );
            const result = await contract.functions.get_game_state().simulate();
            return result.value ?? null;
        }
    });

    return { ...query, gameState: query.data };
}
