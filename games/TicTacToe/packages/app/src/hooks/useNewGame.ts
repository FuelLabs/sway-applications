import { useWallet } from "@fuels/react";
import { useMutation } from "@tanstack/react-query";
import { TictactoeContractAbi__factory } from "../contract-types";
import { queryClient, useAppContext } from "../components";
import { TicTacToeQueryKeys } from "../queryKeys";
import { CONTRACT_ID } from "../config";

export const useNewGame = (player1Address: string, player2Address: string) => {
  const { wallet } = useWallet();
  const appContext = useAppContext();

  const mutation = useMutation({
    mutationFn: async () => {
      if (!wallet) throw new Error(`Cannot increment if wallet is ${wallet}`);

      const contract = TictactoeContractAbi__factory.connect(
        CONTRACT_ID,
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
      appContext?.setAppContext({ ...appContext, showGameBoard: true });
    },
    onError: (err) => console.error(err),
  });

  return mutation;
};
