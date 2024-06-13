"use client";

import { FuelProvider } from "@fuels/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { Provider, type FuelConfig } from "fuels";
import React, { useCallback, useEffect, useState } from "react";
import { EVMWalletConnector } from "@fuels/connectors/evm";
import {
  FuelWalletConnector,
  FuelWalletDevelopmentConnector,
  FueletWalletConnector,
  BurnerWalletConnector,
} from "@fuels/connectors";
import { NODE_URL } from "@/lib";
import { StyledEngineProvider } from "@mui/material";

export const queryClient: QueryClient = new QueryClient();

export const AppProvider = ({ children }: { children: React.ReactNode }) => {
  const [fuelConfig, setFuelConfig] = useState<FuelConfig>({});
  const [currentProvider, setCurrentProvider] = useState<Provider>();

  const fetchProvider = useCallback(async () => {
    const provider = await Provider.create(NODE_URL);
    setCurrentProvider(provider);
  }, []);

  useEffect(() => {
    fetchProvider();
  }, [fetchProvider]);

  useEffect(() => {
    const newFuelConfig = {
      connectors: [
        new FuelWalletConnector(),
        new FueletWalletConnector(),
        new EVMWalletConnector({ fuelProvider: currentProvider }),
        new FuelWalletDevelopmentConnector(),
        new BurnerWalletConnector(),
      ],
    };
    setFuelConfig(newFuelConfig);
  }, [currentProvider]);

  return (
    <StyledEngineProvider injectFirst>
      <QueryClientProvider client={queryClient}>
        <FuelProvider fuelConfig={fuelConfig}>{children}</FuelProvider>
      </QueryClientProvider>
    </StyledEngineProvider>
  );
};
