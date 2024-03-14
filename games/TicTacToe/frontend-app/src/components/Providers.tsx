import type { ReactNode } from "react";
import { FuelProvider } from "@fuels/react";
import { FuelWalletConnector } from "@fuel-wallet/sdk";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { AppContextProvider } from ".";

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
