import { useWallet } from "@fuels/react";
import { useQuery } from "@tanstack/react-query";
import { TictactoeContractAbi__factory } from "../contract-types";
import { TicTacToeQueryKeys } from "../queryKeys";

export const useGetGameBoard = () => {
  const { wallet } = useWallet();

  const query = useQuery({
    queryKey: [TicTacToeQueryKeys.gameBoard],
    queryFn: async () => {
      if (!wallet)
        throw new Error(`Cannot get game board if wallet is ${wallet}`);

      const contract = TictactoeContractAbi__factory.connect(
        import.meta.env.VITE_CONTRACT_ID,
        wallet
      );
      const result = await contract.functions.get_board().simulate();
      console.log(`result`, result);
      return result.value ?? null;
    },
  });

  return { ...query, gameBoard: query.data };
};
