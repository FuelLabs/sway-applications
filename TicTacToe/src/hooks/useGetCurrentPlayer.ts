import { useAccount, useWallet } from '@fuels/react';
import { useQuery } from '@tanstack/react-query';

import { CONTRACT_ID } from '../config';
import { TictactoeContractAbi__factory } from '../contract-types';
import { TicTacToeQueryKeys } from '../queryKeys';

export const useGetCurrentPlayer = () => {
  const { account  } = useAccount();
  const { wallet, isError, isLoading } = useWallet(account);

  const query = useQuery({
    queryKey: [TicTacToeQueryKeys.currentPlayer, wallet?.provider.url],
    queryFn: async () => {
      if (!wallet)
        throw new Error(`Cannot get current player if wallet is ${wallet}`);

      const contract = TictactoeContractAbi__factory.connect(
        CONTRACT_ID,
        wallet
      );
      const result = await contract.functions.get_current_player().simulate();
      return result.value ?? null;
    },
    enabled: !!wallet && !isError && !isLoading,
  });

  return { ...query, currentPlayer: query.data };
};
