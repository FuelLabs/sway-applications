import { useAccounts } from "@fuels/react";
import { useGetCurrentPlayer } from ".";

export const useGetPlayers = () => {
    const { accounts } = useAccounts();
    const { currentPlayer } = useGetCurrentPlayer();

    let players: string[] = [];
    if (accounts.length === 1) {
        players = [accounts[0], accounts[0]];
    } else if (accounts.length > 1) {
        players = [accounts[0], accounts[1]];
    }
    const isPlayer1Turn = accounts.length > 0 && !!currentPlayer ? currentPlayer?.Address?.value === accounts[0] : undefined;
    return { players, isPlayer1Turn };
}
