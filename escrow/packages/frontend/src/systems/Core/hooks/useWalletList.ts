import { useContext } from "react";
import { AppContext } from "../context/AppContext";

export const useWalletList = () => {
    const { wallets } = useContext(AppContext)!;
    return wallets;
};