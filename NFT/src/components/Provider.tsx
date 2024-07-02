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

export const AppProvider = ({ children }: { children: React.ReactNode }) => {
  const [queryClient] = useState(() => {
    return new QueryClient({});
  });
  const [currentUrl, setCurrentUrl] = useState("");
  const [isMounted, setIsMounted] = useState(false);
  const [currentProvider] = useState(Provider.create(NODE_URL));

  useEffect(() => {
    setIsMounted(true);
    setCurrentUrl(window.location.href);
  }, []);

  // NOTE: If the component is not rendered in the client
  // the BurnerWalletConnector will throw an error
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
  // NOTE: we do not have ssr: true
  // Bc there is a bug in the connector
  // https://github.com/FuelLabs/fuel-connectors/issues/134
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
