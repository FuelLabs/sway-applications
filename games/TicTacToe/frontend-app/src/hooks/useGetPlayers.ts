import { useAccounts } from "@fuels/react";
import { useGetCurrentPlayer, useGetMoveCounter } from ".";

export const useGetPlayers = () => {
  const { accounts } = useAccounts();
  const { currentPlayer } = useGetCurrentPlayer();
  const { moveCounter } = useGetMoveCounter();

  let players: string[] = [];
  if (accounts.length === 1) {
    players = [accounts[0], accounts[0]];
  } else if (accounts.length > 1) {
    players = [accounts[0], accounts[1]];
  }
  const isPlayer1Turn =
    accounts.length > 0 && !!currentPlayer && !!moveCounter
      ? moveCounter.toNumber() % 2 === 0
      : undefined;

  return { players, isPlayer1Turn, currentPlayer };
};
