import { toast } from '@fuel-ui/react';
import { useQuery } from 'react-query';

import { useFuel } from './useFuel';

export const useWallet = () => {
  const [fuel] = useFuel();

  if (!fuel) toast.error('Error fuelWeb3 instance is not defined');

  // useEffect(() => {
  //   // TODO move this somewhere where it is not called multiple times
  //   // We add this event listener a whole bunch
  //   fuel.on('currentAccount', () => {
  //     queryClient.invalidateQueries({ queryKey: ['wallet'] });
  //   });

  //   return () => {
  //     fuel.off('currentAccount', () => {});
  //   };
  // }, []);

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
