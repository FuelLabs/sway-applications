import { ReactNode, createContext, useContext, useState } from "react";
import { FuelProvider } from "@fuels/react";
import { FuelWalletConnector } from "@fuel-wallet/sdk";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

type ProvidersProps = {
  children?: ReactNode;
};

// TODO add toast for errors?
export const queryClient = new QueryClient();

type AppContextType = {
  isGameBoardEnabled: boolean;
  showGameBoard: boolean;
  setAppContext: (appContext: AppContextType) => void;
} | null;

const AppContext = createContext<AppContextType>(null);

export const useAppContext = () => {
  return useContext(AppContext);
};

export const Providers = ({ children }: ProvidersProps) => {
  const [appContext, setAppContext] = useState<AppContextType>({
    isGameBoardEnabled: true,
    showGameBoard: false,
    setAppContext: () => {},
  });

  return (
    <FuelProvider fuelConfig={{ connectors: [new FuelWalletConnector()] }}>
      <QueryClientProvider client={queryClient}>
        <AppContext.Provider value={{ ...appContext, setAppContext } as AppContextType}>
          {children}
        </AppContext.Provider>
      </QueryClientProvider>
    </FuelProvider>
  );
};
