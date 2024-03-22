import { useWallet } from '@fuels/react';
import { useQuery } from '@tanstack/react-query';

import { CONTRACT_ID, PROVIDER_URL } from '../config';
import { TictactoeContractAbi__factory } from '../contract-types';
import { TicTacToeQueryKeys } from '../queryKeys';

export const useGetGameState = () => {
  const { wallet, isError, isLoading } = useWallet();

  const query = useQuery({
    queryKey: [TicTacToeQueryKeys.gameState, wallet?.provider.url],
    queryFn: async () => {
      if (!wallet)
        throw new Error(`Cannot get game state if the wallet is ${wallet}`);

      if (PROVIDER_URL !== wallet.provider.url) {
        return null;
      }

      const contract = TictactoeContractAbi__factory.connect(
        CONTRACT_ID,
        wallet
      );
      const result = await contract.functions.get_game_state().simulate();
      return result.value ?? null;
    },
    enabled: !!wallet && !isError && !isLoading,
  });

  return { ...query, gameState: query.data };
};
