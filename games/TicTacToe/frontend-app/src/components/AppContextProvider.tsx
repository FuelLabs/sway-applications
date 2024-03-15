import { createContext, useContext, useState } from 'react';
import type { ReactNode } from 'react';
import { useGetGameBoard, useGetGameState } from '../hooks';

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
  const { gameBoard, isLoading: isGameBoardLoading } = useGetGameBoard();
  const { gameState, isLoading: isGameStateLoading } = useGetGameState();
  const [appContext, setAppContext] = useState<AppContextObject>({
    isGameBoardEnabled: true,
    showGameBoard: false,
  });

  if (isGameBoardLoading || isGameStateLoading) {
    return null;
  }

  return (
    <AppContext.Provider
      value={{
        appContextData: {
          ...appContext,
          isGameBoardEnabled: gameState === 'Playing',
          showGameBoard: !!gameBoard && gameBoard.length > 0,
        },
        setAppContext,
      }}
    >
      {children}
    </AppContext.Provider>
  );
};
