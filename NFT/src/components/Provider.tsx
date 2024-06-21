"use client";

import { FuelProvider } from "@fuels/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { Provider, type FuelConfig } from "fuels";
import React, { useCallback, useEffect, useState } from "react";
import { coinbaseWallet, walletConnect } from "@wagmi/connectors";
import { http, createConfig, injected } from "@wagmi/core";
import { mainnet, sepolia } from "@wagmi/core/chains";
import {
  FuelWalletConnector,
  FuelWalletDevelopmentConnector,
  FueletWalletConnector,
  BurnerWalletConnector,
  WalletConnectConnector
} from "@fuels/connectors";
import { StyledEngineProvider } from "@mui/material";

import { NODE_URL, WC_PROJECT_ID } from "@/lib";

export const queryClient: QueryClient = new QueryClient();

export const AppProvider = ({ children }: { children: React.ReactNode }) => {
  const [fuelConfig, setFuelConfig] = useState<FuelConfig>({});
  const [currentProvider, setCurrentProvider] = useState<Provider>();
  const [currentUrl, setCurrentUrl] = useState("");

  useEffect(() => {
    if (window !== undefined) {
      setCurrentUrl(window.location.href);
    }
  }, []);

  // ============================================================
  // WalletConnect Connector configurations
  // https://docs.walletconnect.com/web3modal/javascript/about
  // ============================================================
  const METADATA = {
    name: "NFT App",
    description: "View and collect NFTs",
    url: currentUrl,
    icons: ["https://connectors.fuel.network/logo_white.png"],
  };
  const wagmiConfig = createConfig({
    chains: [mainnet, sepolia],
    transports: {
      [mainnet.id]: http(),
      [sepolia.id]: http(),
    },
    connectors: [
      injected({ shimDisconnect: false }),
      walletConnect({
        projectId: WC_PROJECT_ID,
        metadata: METADATA,
        showQrModal: false,
      }),
      coinbaseWallet({
        appName: METADATA.name,
        appLogoUrl: METADATA.icons[0],
        darkMode: true,
        reloadOnDisconnect: true,
      }),
    ],
  });

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
        new WalletConnectConnector({
          fuelProvider: currentProvider,
          wagmiConfig,
          projectId: WC_PROJECT_ID,
        }),
        new FuelWalletDevelopmentConnector(),
        new BurnerWalletConnector({ fuelProvider: currentProvider }),
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
