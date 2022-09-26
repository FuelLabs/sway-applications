import { Wallet } from "fuels";
import { useAtom } from "jotai";
import React, { useContext, useState, useMemo, useEffect } from "react";
import type { PropsWithChildren } from "react";

import { NUM_WALLETS, FUEL_PROVIDER_URL } from "../../../config";
import { walletIndexAtom } from "../jotai";

import type { Maybe } from "@/types";

interface AppContextValue {
  wallets: Maybe<Array<Wallet>>;
  wallet: Maybe<Wallet>;
}

export const AppContext = React.createContext<Maybe<AppContextValue>>(null);

export const useAppContext = () => useContext(AppContext);

export const useWallet = () => {
  const { wallet } = useContext(AppContext)!;
  return wallet;
};

export const useWalletList = () => {
  const { wallets } = useContext(AppContext)!;
  return wallets;
};

export const AppContextProvider = ({
  children,
}: PropsWithChildren<unknown>) => {
  const [currentWalletIndex, setCurrentWalletIndex] = useAtom(walletIndexAtom);
  const [privateKeyList, setPrivateKeyList] = useState<Array<string> | null>(
    []
  );

  const wallets = useMemo(() => {
    if (!privateKeyList) {
      return null;
    }
    const walletList: Array<Wallet> | null = [];
    privateKeyList.forEach((privateKey) => {
      walletList?.push(new Wallet(privateKey, FUEL_PROVIDER_URL));
    });
    return walletList;
  }, [privateKeyList]);

  const wallet = useMemo(() => {
    if (currentWalletIndex === null || !wallets || wallets.length === 0) {
      return null;
    }
    return wallets[currentWalletIndex];
  }, [currentWalletIndex]);

  // TODO store wallets in local storage or somewhere more persistant
  useEffect(() => {
    if (wallets!.length > 0) {
      return;
    }
    const nextPrivateKeyList: Array<string> | null = Array(NUM_WALLETS);
    for (let i = 0; i < NUM_WALLETS; i += 1) {
      const nextPrivateKey = process.env[`VITE_WALLET${i}`]!;
      const nextWallet = new Wallet(nextPrivateKey, FUEL_PROVIDER_URL);
      nextPrivateKeyList[i] = nextWallet.privateKey;
    }
    setPrivateKeyList(nextPrivateKeyList);
    setCurrentWalletIndex(0);
  });

  return (
    <AppContext.Provider
      value={{
        wallets,
        wallet,
        // contract,
      }}
    >
      {children}
    </AppContext.Provider>
  );
};
