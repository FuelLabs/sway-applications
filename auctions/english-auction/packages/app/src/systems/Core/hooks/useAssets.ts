import { toast } from 'react-hot-toast';
import { useQuery } from 'react-query';

import { useWallet } from './useWallet';

export const useAssets = () => {
  const { wallet, isLoading, isError } = useWallet();

  if (isError) toast.error('Error: fetching wallet');

  const { data: balances } = useQuery(
    ['balances'],
    async () => {
      // eslint-disable-next-line @typescript-eslint/return-await
      return await wallet?.getBalances();
    },
    {
      enabled: !isLoading && !isError && !!wallet,
    }
  );
  return balances;
};
