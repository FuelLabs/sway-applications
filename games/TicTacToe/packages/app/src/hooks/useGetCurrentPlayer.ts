import { useQuery } from "@tanstack/react-query"
import { useWallet } from "@fuels/react";
import { TictactoeContractAbi__factory } from "../contract-types";

export const useGetCurrentPlayer = () => {
    const { wallet } = useWallet();

    const query = useQuery({
        queryKey: ['currentPlayer'],
        queryFn: async () => {
            if (!wallet) throw new Error(`Cannot get current player if wallet is ${wallet}`);

            const contract = TictactoeContractAbi__factory.connect(
                import.meta.env.VITE_CONTRACT_ID,
                wallet
            );
        }
    });

    return query;
}
