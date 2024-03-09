import { useMutation } from "@tanstack/react-query";

import { useWallet } from "@fuels/react";
import { TictactoeContractAbi__factory } from "../contract-types";
import { queryClient } from "../components";
import { TicTacToeQueryKeys } from "../queryKeys";

export const useMakeMove = (position: number) => {
  const { wallet } = useWallet();

  const mutation = useMutation({
    mutationFn: async () => {
      if (!wallet) throw new Error(`Cannot make move if wallet is ${wallet}`);

      const contract = TictactoeContractAbi__factory.connect(
        import.meta.env.VITE_CONTRACT_ID,
        wallet
      );
      const result = await contract.functions.make_move(position).call();
      console.log(`result make move`, result);
      return result;
    },
    onSuccess: async () => {
      console.log("one");
      await queryClient.invalidateQueries({
        queryKey: [TicTacToeQueryKeys.gameBoard],
      });
      await queryClient.invalidateQueries({
        queryKey: [TicTacToeQueryKeys.gameState],
      });
      await queryClient.invalidateQueries({
        queryKey: [TicTacToeQueryKeys.currentPlayer],
      });
      console.log("two");
    },
    onError: (err) => {
      console.error(err);
    },
  });

  return mutation;
};
