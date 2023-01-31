import { toast } from '@fuel-ui/react';
import { useEffect } from 'react';
import { useQuery } from 'react-query';

import { queryClient } from '../utils';

import { useFuel } from './useFuel';

export const useWallet = () => {
  const [fuel] = useFuel();

  if (!fuel) toast.error('Error fuelWeb3 instance is not defined');

  useEffect(() => {
    fuel.on('currentAccount', () => {
      queryClient.invalidateQueries({ queryKey: ['wallet'] });
    });

    return () => {
      fuel.off('currentAccount', () => {});
    };
  }, []);

  const {
    data: wallet,
    isLoading,
    isError,
  } = useQuery(
    ['wallet'],
    async () => {
      const isConnected = await fuel.isConnected();
      if (!isConnected) {
        await fuel.connect();
      }
      const selectedAccount = (await fuel.currentAccount()) as string;
      const selectedWallet = await fuel.getWallet(selectedAccount);
      return selectedWallet;
    },
    {
      enabled: !!fuel,
    }
  );

  return { wallet, isLoading, isError };
};
