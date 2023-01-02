import { useQuery } from 'react-query';

import { useWallet } from './useWallet';

export const useAssets = () => {
  const wallet = useWallet();
  const { data: balances } = useQuery(
    ['balances'],
    async () => {
      // eslint-disable-next-line @typescript-eslint/return-await
      return await wallet?.getBalances();
    },
    {
      enabled: !!wallet && !!wallet.address,
    }
  );
  return balances;
};
