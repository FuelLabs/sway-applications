import { FuelWalletConnector } from '@fuel-wallet/sdk';
import { FuelProvider } from '@fuels/react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import type { ReactNode } from 'react';

import { AppContextProvider } from '.';

type ProvidersProps = {
  children?: ReactNode;
};

// TODO add toast for errors?
export const queryClient = new QueryClient();

export const Providers = ({ children }: ProvidersProps) => {
  return (
    <FuelProvider fuelConfig={{ connectors: [new FuelWalletConnector()] }}>
      <QueryClientProvider client={queryClient}>
        <AppContextProvider>{children}</AppContextProvider>
      </QueryClientProvider>
    </FuelProvider>
  );
};
