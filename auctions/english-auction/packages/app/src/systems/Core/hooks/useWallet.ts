import { useQuery } from 'react-query';

import { useFuel } from './useFuel';

export const useWallet = () => {
  const fuel = useFuel();

  // TODO figure out how to listen for the fuel.on('currentAccount', ...) event correctly
  // this small feature can be added in a later pr

  const {
    data: wallet,
    isLoading,
    isError,
  } = useQuery(
    ['wallet'],
    async () => {
      const isConnected = await fuel!.isConnected();
      if (!isConnected) {
        await fuel!.connect();
      }
      const selectedAccount = (await fuel!.currentAccount()) as string;
      const selectedWallet = await fuel!.getWallet(selectedAccount);
      return selectedWallet;
    },
    {
      enabled: !!fuel,
    }
  );

  return { wallet, isLoading, isError };
};
