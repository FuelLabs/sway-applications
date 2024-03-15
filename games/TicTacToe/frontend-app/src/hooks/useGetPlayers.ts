import { useAccounts } from '@fuels/react';
import { useGetCurrentPlayer, useGetMoveCounter } from '.';
import { useEffect, useState } from 'react';

export const useGetPlayers = () => {
  const { accounts } = useAccounts();
  const { currentPlayer } = useGetCurrentPlayer();
  const { moveCounter } = useGetMoveCounter();
  const [isPlayer1Turn, setIsPlayer1Turn] = useState<boolean | undefined>(
    undefined
  );

  let players: string[] = [];
  if (accounts.length === 1) {
    players = [accounts[0], accounts[0]];
  } else if (accounts.length > 1) {
    players = [accounts[0], accounts[1]];
  }

  useEffect(() => {
    setIsPlayer1Turn(
      accounts.length > 0 && !!moveCounter
        ? moveCounter.toNumber() % 2 === 0
        : undefined
    );
  }, [accounts, moveCounter]);

  return { players, isPlayer1Turn, currentPlayer };
};
