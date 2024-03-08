import { useAccounts } from "@fuels/react";

export const useGetPlayers = (): string[] => {
    const { accounts } = useAccounts();

    if (accounts.length === 0) {
        return [];
    }
    if (accounts.length === 1) {
        return [accounts[0], accounts[0]];
    }
    return [accounts[0], accounts[1]];
}
