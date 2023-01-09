import { useQuery } from 'react-query';

import { useWallet } from './useWallet';

import { CONTRACT_ID } from '~/config';
import { AuctionContractAbi__factory } from '~/types/contracts';

export const useContract = () => {
  const { wallet, isLoading, isError } = useWallet();

  const { data: contract } = useQuery(
    ['contract'],
    () => {
      // Connects our contract instance to the deployed contract
      // using the given wallet.
      return AuctionContractAbi__factory.connect(CONTRACT_ID, wallet!);
    },
    {
      enabled: !isLoading && !isError && !!wallet,
    }
  );

  return contract;
};
