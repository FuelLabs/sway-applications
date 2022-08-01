import { randomBytes } from "ethers/lib/utils";
import { ScriptTransactionRequest, toBigInt, Wallet } from "fuels";
import { useAtom } from "jotai";
import React, { useContext, useState, useMemo, useEffect } from "react";
import type { PropsWithChildren } from "react";

import { ASSETS, DECIMAL_PRECISION, FUEL_PROVIDER_URL, ESCROW_ID } from "../../../config";
import { walletIndexAtom } from "../jotai";
import { EscrowAbi, EscrowAbi__factory } from "../types/contracts";

// Initial number of wallets to populate in app
const NUM_WALLETS = 10;

interface AppContextValue {
  wallets: Array<Wallet> | null;
  wallet: Wallet | null;
  contract: EscrowAbi | null;
  //contracts: Array<EscrowAbi> | null;
}

export const AppContext = React.createContext<AppContextValue | null>(null);

export const useAppContext = () => useContext(AppContext);

export const useWallet = () => {
  const { wallet } = useContext(AppContext)!;
  return wallet;
};

export const useWalletList = () => {
  const { wallets } = useContext(AppContext)!;
  return wallets;
};

export const seedWallet = async (
  wallet: Wallet,
  assetId: string,
  assetAmount: bigint
) => {
  const transactionRequest = new ScriptTransactionRequest({
    gasPrice: 0,
    gasLimit: 100_000_000,
    script: "0x24400000",
    scriptData: randomBytes(32),
  });
  // @ts-ignore
  transactionRequest.addCoin({
    id: "0x000000000000000000000000000000000000000000000000000000000000000000",
    assetId,
    amount: assetAmount,
    owner: "0xf1e92c42b90934aa6372e30bc568a326f6e66a1a0288595e6e3fbd392a4f3e6e",
  });
  transactionRequest.addCoinOutput(wallet.address, assetAmount, assetId);
  const submit = await wallet.sendTransaction(transactionRequest);
  return submit.wait();
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
    if (currentWalletIndex === null || !wallets) {
      return null;
    }
    console.log("app wallet", wallets[currentWalletIndex].address);
    return wallets[currentWalletIndex];
  }, [currentWalletIndex]);

  const contract = useMemo(() => {
    if (!wallet) return null;
    return EscrowAbi__factory.connect(ESCROW_ID, wallet);
  }, [wallet]);

  // TODO store wallets in local storage or somewhere more persistant
  useEffect(() => {
    if (wallets!.length > 0) {
      return;
    }
    const nextPrivateKeyList: Array<string> | null = Array(NUM_WALLETS);
    for (let i = 0; i < NUM_WALLETS; i += 1) {
      const nextWallet = Wallet.generate({
        provider: FUEL_PROVIDER_URL,
      });
      ASSETS.forEach((assetId) => {
        const randAssetAmount = Math.floor(Math.random() * 9) + 1;
        seedWallet(
          nextWallet,
          assetId,
          DECIMAL_PRECISION * toBigInt(randAssetAmount)
        );
      });
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
        contract,
      }}
    >
      {children}
    </AppContext.Provider>
  );
};
