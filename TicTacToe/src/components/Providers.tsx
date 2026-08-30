import {
  //BurnerWalletConnector,
  FuelWalletConnector,
  FuelWalletDevelopmentConnector,
  FueletWalletConnector,
  EVMWalletConnector
  //WalletConnectConnector
} from "@fuels/connectors";
//import { WalletConnectConnector } from "@fuels/connectors/walletconnect";
import { FuelProvider } from '@fuels/react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
// import { coinbaseWallet, walletConnect } from '@wagmi/connectors';
// import { http, createConfig, injected } from "@wagmi/core";
// import { mainnet, sepolia } from "@wagmi/core/chains";
import type { ReactNode } from 'react';


import { AppContextProvider } from '.';

type ProvidersProps = {
  children?: ReactNode;
};

// TODO add toast for errors?
export const queryClient = new QueryClient();

// ============================================================
// WalletConnect Connector configurations
// https://docs.walletconnect.com/web3modal/javascript/about
// ============================================================
// const WC_PROJECT_ID = import.meta.env.VITE_WC_PROJECT_ID;
// const METADATA = {
//   name: "TicTacToe",
//   description: "TicTacToe on Fuel network",
//   url: location.href,
//   icons: ['https://connectors.fuel.network/logo_white.png'], // TODO replace
// }
// const wagmiConfig = createConfig({
//   chains: [mainnet, sepolia],
//   transports: {
//     [mainnet.id]: http(),
//     [sepolia.id]: http(),
//   },
//   connectors: [
//     injected({ shimDisconnected: true }),
//     walletConnect({
//       projectId: WC_PROJECT_ID,
//       metadata: METADATA,
//       showQRModal: false,
//     }),
//     coinbaseWallet({
//       appName: METADATA.name,
//       appLogoUrl: METADATA.icons[0],
//       darkMode: false,
//       reloadOnDisconnect: true,
//     }),
//   ],
// });

export const Providers = ({ children }: ProvidersProps) => {
  return (
    <FuelProvider fuelConfig={{ connectors: [
        new FuelWalletConnector(),
        new FueletWalletConnector(),
        new EVMWalletConnector(),
        // new WalletConnectConnector({
        //   wagmiConfig,
        //   projectId: WC_PROJECT_ID,
        // }),
        new FuelWalletDevelopmentConnector(),
        //new BurnerWalletConnector(),
      ]
    }}>
      <QueryClientProvider client={queryClient}>
        <AppContextProvider>{children}</AppContextProvider>
      </QueryClientProvider>
    </FuelProvider>
  );
};
