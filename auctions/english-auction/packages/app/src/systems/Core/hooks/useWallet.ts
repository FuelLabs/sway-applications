import { toast } from '@fuel-ui/react';
import { useQuery } from 'react-query';

import { useFuel } from './useFuel';

export const useWallet = () => {
  const [fuel] = useFuel();

  if (!fuel) toast.error('Error fuelWeb3 instance is not defined');

  const {
    data: wallet,
    isLoading,
    isError,
  } = useQuery(
    ['wallet'],
    async () => {
      // TODO fix: don't hardcode
      await fuel.connect({ url: 'http://localhost:4000/graphql' });
      const selectedAccount = (await fuel.getSelectedAccount()) as string;
      const selectedWallet = await fuel.getWallet(selectedAccount);
      console.log(selectedWallet.address.toHexString());
      return selectedWallet;
    },
    {
      enabled: !!fuel,
    }
  );

  return { wallet, isLoading, isError };
};
