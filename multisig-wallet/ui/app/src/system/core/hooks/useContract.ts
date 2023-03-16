import { useQuery } from '@tanstack/react-query';
import { useWallet } from './useWallet';
import { MultisigContractAbi__factory } from '../../../contracts';

export const useContract = () => {
  const { wallet, isLoading, isError } = useWallet();

  const { data: contract } = useQuery(
    ['contract', wallet],
    () => {
      return MultisigContractAbi__factory.connect(import.meta.env.VITE_CONTRACT_ID, wallet!);
    },
    {
      enabled: !isLoading && !isError && !!wallet,
    }
  );

  return contract;
};