import { useQuery } from 'react-query';

import { useWallet } from './useWallet';

export const useAssets = () => {
  const wallet = useWallet();
  const { data: balances } = useQuery(
    ['balances'],
    async () => {
      const temp = await wallet.getBalances();
      return temp;
    },
    {
      enabled: !!wallet?.address.toString(),
    }
  );
  return balances;
};
