import { useWallet } from "@fuels/react";
import { useMutation } from "@tanstack/react-query";
import { TictactoeContractAbi__factory } from "../contract-types";
import { queryClient } from "../components";
import { TicTacToeQueryKeys } from "../queryKeys";

export const useNewGame = (player1Address: string, player2Address: string) => {
  const { wallet } = useWallet();

  const mutation = useMutation({
    mutationFn: async () => {
      if (!wallet) throw new Error(`Cannot increment if wallet is ${wallet}`);

      const contract = TictactoeContractAbi__factory.connect(
        import.meta.env.VITE_CONTRACT_ID,
        wallet
      );
      await contract.functions
        .new_game(
          { Address: { value: player1Address } },
          { Address: { value: player2Address } }
        )
        .call();
    },
    onSuccess: async () => {
      await queryClient.invalidateQueries({
        queryKey: [TicTacToeQueryKeys.gameBoard],
      });
      await queryClient.invalidateQueries({
        queryKey: [TicTacToeQueryKeys.gameState],
      });
      await queryClient.invalidateQueries({
        queryKey: [TicTacToeQueryKeys.currentPlayer],
      });
    },
    onError: (err) => console.error(err),
  });

  return mutation;
};
