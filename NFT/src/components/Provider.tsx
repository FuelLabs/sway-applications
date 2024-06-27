"use client";

import { FuelProvider } from "@fuels/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { Provider } from "fuels";
import React, { useEffect, useState } from "react";
import { coinbaseWallet, walletConnect } from "@wagmi/connectors";
import { http, createConfig, injected } from "@wagmi/core";
import { mainnet, sepolia } from "@wagmi/core/chains";
import {
  FuelWalletConnector,
  FuelWalletDevelopmentConnector,
  FueletWalletConnector,
  BurnerWalletConnector,
  WalletConnectConnector,
} from "@fuels/connectors";
import { StyledEngineProvider } from "@mui/material";

import { NODE_URL, WC_PROJECT_ID } from "@/lib";

export const queryClient: QueryClient = new QueryClient();

export const AppProvider = ({ children }: { children: React.ReactNode }) => {
  const [currentUrl, setCurrentUrl] = useState("");
  const [isMounted, setIsMounted] = useState(false);


  useEffect(() => {
    setIsMounted(true);
    setCurrentUrl(window.location.href);
  }, []);

  if (!isMounted) return null;

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

  const currentProvider = Provider.create(NODE_URL);

  return (
    <StyledEngineProvider injectFirst>
      <QueryClientProvider client={queryClient}>
        <FuelProvider
          fuelConfig={{
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
          }}
        >
          {children}
        </FuelProvider>
      </QueryClientProvider>
    </StyledEngineProvider>
  );
};
