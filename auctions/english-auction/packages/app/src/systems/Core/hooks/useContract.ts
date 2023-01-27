import { useQuery } from 'react-query';

import { useWallet } from './useWallet';

import { CONTRACT_ID } from '~/config';
import { AuctionContractAbi__factory } from '~/types/contracts';

export const useContract = () => {
  const { wallet, isLoading, isError } = useWallet();

  const {
    data: contract,
    isLoading: isContractLoading,
    isError: isContractError,
  } = useQuery(
    ['contract', wallet],
    () => {
      // Connects our contract instance to the deployed contract
      // using the given wallet.
      console.log('contract id', CONTRACT_ID);
      console.log('env id', process.env.VITE_CONTRACT_ID);
      return AuctionContractAbi__factory.connect(CONTRACT_ID, wallet!);
    },
    {
      enabled: !isLoading && !isError && !!wallet,
    }
  );

  return { contract, isLoading: isContractLoading, isError: isContractError };
};
