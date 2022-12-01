import { useQuery } from 'react-query';
import { CONTRACT_ID } from '~/config';
import { AuctionContractAbi__factory } from '~/types/contracts';
import { useWallet } from './useWallet';

export const useContract = () => {
  const wallet = useWallet();
  const { data: contract } = useQuery(
    ['contract'],
    () => {
      // Connects our contract instance to the deployed contract
      // using the given wallet.
      return AuctionContractAbi__factory.connect(CONTRACT_ID, wallet!);
    },
    {
      enabled: !!wallet,
    }
  );

  return contract;
};
