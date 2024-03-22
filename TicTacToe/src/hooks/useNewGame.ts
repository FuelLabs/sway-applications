import { useWallet } from '@fuels/react';
import { useMutation } from '@tanstack/react-query';

import { queryClient, useAppContext } from '../components';
import { CONTRACT_ID } from '../config';
import { TictactoeContractAbi__factory } from '../contract-types';

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
      await queryClient.invalidateQueries();
      appContext?.setAppContext({
        ...appContext.appContextData,
        showGameBoard: true,
        isGameBoardEnabled: true,
        lastGameOutcome: undefined,
      });
    },
    onError: async (err) => {
      // TODO: remove once we figure out why a successful call returns an error from the ts sdk
      // on beta-5
      await queryClient.invalidateQueries();
      // eslint-disable-next-line no-console
      console.error(err);
    },
  });

  return mutation;
};
