import { Wallet } from 'fuels';
import { useQuery } from 'react-query';

import { useFuel } from './useFuel';

export const useWallet = () => {
  const [fuel] = useFuel();

  if (!fuel) throw new Error('Error fuelWeb3 instance is not defined');
  // Auto connect application
  // TODO: check if connected to instance
  // https://github.com/FuelLabs/fuels-wallet/pull/413
  fuel.connect();

  const { data: wallet } = useQuery(
    ['wallet'],
    async () => {
      const selectedAccount = (await fuel.getSelectedAccount()) as string;
      return Wallet.fromAddress(selectedAccount, fuel.getProvider());
    },
    {
      enabled: !!fuel,
    }
  );

  return wallet;
};
