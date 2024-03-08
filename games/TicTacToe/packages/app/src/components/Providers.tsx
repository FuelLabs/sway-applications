import { ReactNode } from "react";
import { FuelProvider } from "@fuels/react";
import { FuelWalletConnector } from "@fuel-wallet/sdk";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

type ProvidersProps = {
  children?: ReactNode;
};

// TODO add toast for errors?
export const queryClient = new QueryClient();

export const Providers = ({ children }: ProvidersProps) => {
  return (
    <FuelProvider fuelConfig={{ connectors: [new FuelWalletConnector()] }}>
      <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
    </FuelProvider>
  );
};
