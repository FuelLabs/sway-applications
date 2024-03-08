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
      return result.value;
    },
    onSuccess: async () => {
      await queryClient.invalidateQueries({
        queryKey: [TicTacToeQueryKeys.gameBoard, TicTacToeQueryKeys.gameState],
      });
    },
  });

  return mutation;
};
