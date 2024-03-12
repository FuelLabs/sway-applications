import { useAccounts } from "@fuels/react";
import { useGetCurrentPlayer } from ".";
import { Address } from "fuels";

export const useGetPlayers = () => {
  const { accounts } = useAccounts();
  const { currentPlayer } = useGetCurrentPlayer();

  let players: string[] = [];
  if (accounts.length === 1) {
    players = [accounts[0], accounts[0]];
  } else if (accounts.length > 1) {
    players = [accounts[0], accounts[1]];
  }
  const isPlayer1Turn =
    accounts.length > 0 && !!currentPlayer
      ? currentPlayer?.Address?.value ===
        Address.fromString(accounts[0]).toHexString()
      : undefined;
  return { players, isPlayer1Turn };
};
