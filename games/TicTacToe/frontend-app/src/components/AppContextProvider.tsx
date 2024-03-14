import { createContext, useContext, useEffect, useState } from "react";
import type { ReactNode } from "react";
import { useGetGameBoard } from "../hooks";

type AppContextObject = {
  isGameBoardEnabled: boolean;
  showGameBoard: boolean;
  lastGameOutcome?: boolean | string;
};

type AppContextType = {
  appContextData: AppContextObject;
  setAppContext: (appContext: AppContextObject) => void;
} | null;

const AppContext = createContext<AppContextType>(null);

export const useAppContext = () => {
  return useContext(AppContext);
};

type AppContextProviderProps = {
  children?: ReactNode;
};

export const AppContextProvider = ({ children }: AppContextProviderProps) => {
  const { gameBoard } = useGetGameBoard();
  console.log(`gameBoard`, gameBoard);
  const [appContext, setAppContext] = useState<AppContextObject>({
    isGameBoardEnabled: true,
    showGameBoard: !!gameBoard,
  });

  useEffect(() => {
    if (gameBoard) {
      setAppContext({
        ...appContext,
        showGameBoard: !!gameBoard,
    });
    }
  }, [gameBoard]);

  return (
    <AppContext.Provider value={{ appContextData: appContext, setAppContext }}>
      {children}
    </AppContext.Provider>
  );
};
